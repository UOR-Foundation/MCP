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
    C0(Adt1, Option<Str>, Adt4, Adt10, List<Adt12>, u64, Bytes, u64),
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
    C14(Adt87),
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
    C0(Str, Option<Str>, Str, Str, Option<i64>, Option<i64>),
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
    C0(Adt83, Adt63, Adt84),
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
    C0(Adt62, i64, Adt85),
}

#[derive(Clone)]
pub enum Adt85 {
    C0(i64, List<(Str, Adt64)>, i64, Adt86),
}

#[derive(Clone)]
pub enum Adt86 {
    C0(Adt5),
    C1(Adt80),
}

#[derive(Clone)]
pub enum Adt87 {
    C0(Adt84, Adt88, bool),
}

#[derive(Clone)]
pub enum Adt88 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt89 {
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
    C15(Adt90),
}

#[derive(Clone)]
pub enum Adt90 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt91 {
    C0(i64, Str, Str, Bytes),
}

#[derive(Clone)]
pub enum Adt92 {
    C0(List<u64>, List<u64>, List<Adt93>, Bytes, u64, List<List<u64>>, Bytes, Adt91),
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
pub enum Adt97 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt98 {
    C0,
    C1,
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
    C0(Adt99, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt112 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt113 {
    C0(Adt108, List<Adt109>),
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
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt116>, List<Adt118>),
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt117),
    C7(Adt117),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt117 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt118 {
    C0(Str, List<Str>, Adt13, Adt119),
}

#[derive(Clone)]
pub enum Adt119 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt122 {
    C0(Adt123, List<Adt127>, u64),
}

#[derive(Clone)]
pub enum Adt123 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt124, List<u64>, bool),
    C7(u64, u64),
    C8(Adt125),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt124 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(bool, Adt126, i64, u64, i64, bool, i64),
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
    C7,
}

#[derive(Clone)]
pub enum Adt127 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt128 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt109>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0(Adt99, Result<Adt130, Adt133>),
}

#[derive(Clone)]
pub enum Adt130 {
    C0(Adt32, Adt131, List<Adt132>),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt132 {
    C0(Adt19, Adt50),
}

#[derive(Clone)]
pub enum Adt133 {
    C0(Adt134),
    C1(Str, List<Str>),
    C2(Adt116),
    C3(Str),
    C4(Adt89),
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
    C7(Str),
    C8(Str),
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Option<Str>, Option<Adt116>),
}

#[derive(Clone)]
pub enum Adt136 {
    C0,
    C1(Adt99),
    C2,
}

#[derive(Clone)]
pub enum Adt137 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt100),
    C1(Adt122),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt100),
    C1(List<Adt114>, List<u64>),
}

#[derive(Clone)]
pub enum Adt140 {
    C0(Adt99, Adt141, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt141 {
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
pub enum Adt142 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt143 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt96),
    C1(Adt112),
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
    C1(Adt141),
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Adt148),
    C1(Adt133),
}

#[derive(Clone)]
pub enum Adt148 {
    C0(Adt99, Str, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt149 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt150 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt151 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt152 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
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
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt157 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt158 {
    C0,
    C1(Adt18),
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
    F80(std::rc::Rc<Fn1>, Adt92),
}

impl Fn0 {
    pub fn apply(&self, p0: Adt14) -> R<Result<Adt32, Adt89>> {
        match self {
            Fn0::F3 => f3(p0),
            Fn0::F80(k0, k1) => f80((**k0).clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F5(std::rc::Rc<Fn0>),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt91, p1: Adt14) -> R<Result<Adt32, Adt89>> {
        match self {
            Fn1::F5(k0) => f5((**k0).clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F25,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt21) -> Result<Adt49, Adt89> {
        match self {
            Fn2::F25 => f25(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F66(Fn2, Adt35),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt32, Adt89>> {
        match self {
            Fn3::F66(k0, k1) => f66(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F809(Bytes),
    F834(Bytes, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn4::F809(k0) => f809(k0.clone(), p0),
            Fn4::F834(k0, k1) => f834(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F814(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn5::F814(k0) => f814(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F819(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn6::F819(k0) => f819(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F842(Bytes, u64, u64, u64),
    F849(Bytes, u64, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Adt159)) -> R<(u64, Adt159)> {
        match self {
            Fn7::F842(k0, k1, k2, k3) => f842(k0.clone(), *k1, *k2, *k3, p0),
            Fn7::F849(k0, k1, k2) => f849(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F844,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt161) -> R<Adt161> {
        match self {
            Fn8::F844 => f844(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F874,
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn9::F874 => f874(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<List<u64>> {
    f1(f2()?, Fn0::F3, v0)
}

pub fn f1(v0: Adt0, v1: Fn0, v2: Bytes) -> R<List<u64>> {
    f4(v0, Fn1::F5(std::rc::Rc::new(v1)), v2)
}

pub fn f2() -> R<Adt0> {
    Ok(Adt0::C0(f6(), None::<Str>, f7(), f8(), f9(f10()?)?, f11(), Bytes::lit(&[]), 0u64))
}

pub fn f3(v0: Adt14) -> R<Result<Adt32, Adt89>> {
    let m1 = f12(f10()?)?;
    match m1 {
        None => {
            f13(v0)
        }
        Some(v1) => {
            Ok(Err::<Adt32, Adt89>(v1))
        }
    }
}

pub fn f4(v0: Adt0, v1: Fn1, v2: Bytes) -> R<List<u64>> {
    let m2 = f14(v2)?;
    match m2 {
        Ok(v4) => {
            Ok(f15(f17(v0, v1, v4)?))
        }
        Err(_) => {
            Ok(f15(f16()))
        }
    }
}

pub fn f5(v0: Fn0, _: Adt91, v2: Adt14) -> R<Result<Adt32, Adt89>> {
    let c3: Fn0 = v0;
    c3.apply(v2)
}

pub fn f6() -> Adt1 {
    Adt1::C0(Str::lit("mcp-core-fixture"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt2>>)
}

pub fn f7() -> Adt4 {
    Adt4::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt8>, None::<Adt9>, Some(Adt8::C0(None::<bool>)), Some(Adt5::C0(List::cons((f18(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f8() -> Adt10 {
    Adt10::C0(0u64, Adt11::C1)
}

pub fn f9(v0: List<Adt35>) -> R<List<Adt12>> {
    let m4 = f19(v0)?;
    match m4 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt12>::nil())
        }
    }
}

pub fn f10() -> R<List<Adt35>> {
    f20(f21(), List::<Adt35>::nil())
}

pub fn f11() -> u64 {
    100u64
}

pub fn f12(v0: List<Adt35>) -> R<Option<Adt89>> {
    let m5 = f19(v0)?;
    match m5 {
        Ok(_) => {
            Ok(None::<Adt89>)
        }
        Err(v1) => {
            Ok(Some(Adt89::C9(f22(v1))))
        }
    }
}

pub fn f13(v0: Adt14) -> R<Result<Adt32, Adt89>> {
    let m6 = v0;
    match m6 {
        Adt14::C0(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C1(v1) => {
            Ok(f23(f8(), f10()?, v1))
        }
        Adt14::C2(v2) => {
            f24(f10()?, Fn2::F25, v2)
        }
        Adt14::C3(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C4(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C5(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C6(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C7(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C8(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C9(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C10(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C11(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the crowd offers tools only"))))
        }
    }
}

pub fn f14(v0: Bytes) -> R<Result<Adt92, Adt94>> {
    let m7 = f26(v0.clone(), 0u64)?;
    match m7 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(0u64)))
        }
        Some(v1) => {
            f27(v0.clone(), v1.clone(), f26(v0, match v1 {
                Adt97::C0(_, h8) => {
                    h8
                }
            })?)
        }
    }
}

pub fn f15(v0: Adt95) -> List<u64> {
    let a32 = f28(match v0.clone() {
        Adt95::C0(h9, _, _, _, _) => {
            h9
        }
    });
    let a33 = {
        let a30 = f28({
            let a11 = match v0.clone() {
                Adt95::C0(_, h10, _, _, _) => {
                    h10
                }
            };
            length_list(a11)
        });
        let a31 = {
            let a28 = f29(match v0.clone() {
                Adt95::C0(_, h12, _, _, _) => {
                    h12
                }
            });
            let a29 = {
                let a26 = f28({
                    let a14 = match v0.clone() {
                        Adt95::C0(_, _, h13, _, _) => {
                            h13
                        }
                    };
                    length_list(a14)
                });
                let a27 = {
                    let a24 = f30(match v0.clone() {
                        Adt95::C0(_, _, h15, _, _) => {
                            h15
                        }
                    });
                    let a25 = {
                        let a22 = f28({
                            let a17 = match v0.clone() {
                                Adt95::C0(_, _, _, h16, _) => {
                                    h16
                                }
                            };
                            length_list(a17)
                        });
                        let a23 = {
                            let a20 = match v0.clone() {
                                Adt95::C0(_, _, _, h18, _) => {
                                    h18
                                }
                            };
                            let a21 = f31(match v0 {
                                Adt95::C0(_, _, _, _, h19) => {
                                    h19
                                }
                            });
                            append_list(a20, a21)
                        };
                        append_list(a22, a23)
                    };
                    append_list(a24, a25)
                };
                append_list(a26, a27)
            };
            append_list(a28, a29)
        };
        append_list(a30, a31)
    };
    append_list(a32, a33)
}

pub fn f16() -> Adt95 {
    Adt95::C0(f32(), f33(), List::<List<u64>>::nil(), f34(), None::<List<u64>>)
}

pub fn f17(v0: Adt0, v1: Fn1, v2: Adt92) -> R<Adt95> {
    f35(v0, v1, v2.clone(), f36(match v2.clone() {
        Adt92::C0(_, _, h34, _, _, _, _, _) => {
            h34
        }
    }, match v2 {
        Adt92::C0(_, _, _, _, _, h35, _, _) => {
            h35
        }
    })?)
}

pub fn f18() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f19(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt96>> {
    let m36 = f37(v0.clone());
    match m36 {
        None => {
            f38(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt96>(v1))
        }
    }
}

pub fn f20(p39: u64, p40: List<Adt35>) -> R<List<Adt35>> {
    let m38 = (p39, p40);
    {
        let mut m38 = m38;
        loop {
            let step = {
                let (v0, v1) = m38;
                let m37 = v0;
                if m37 == 0 {
                    Err::<(u64, List<Adt35>), R<List<Adt35>>>(Ok(v1))
                } else {
                    let v2 = m37 - 1;
                    let v3: Adt35 = f39(f40(v2)?, Str::lit("Answers with its own name"), f41());
                    Ok::<(u64, List<Adt35>), R<List<Adt35>>>((v2, List::cons(v3, v1)))
                }
            };
            match step {
                Ok(next) => { m38 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21() -> u64 {
    60u64
}

pub fn f22(v0: Adt96) -> Str {
    let m41 = v0;
    match m41 {
        Adt96::C0(v1) => {
            let a42 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a43 = Str::lit(" ");
            join(a42, a43)
        }
        Adt96::C1(v2) => {
            let a44 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a45 = Str::lit(" ");
            join(a44, a45)
        }
        Adt96::C2(v3) => {
            let a46 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a47 = Str::lit(" ");
            join(a46, a47)
        }
        Adt96::C3(v4) => {
            let a48 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a49 = Str::lit(" ");
            join(a48, a49)
        }
        Adt96::C4(v5, v6) => {
            let a50 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a51 = Str::lit(" ");
            join(a50, a51)
        }
        Adt96::C5(v7, v8) => {
            let a52 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a53 = Str::lit(" ");
            join(a52, a53)
        }
        Adt96::C6(v9, v10) => {
            let a54 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a55 = Str::lit(" ");
            join(a54, a55)
        }
        Adt96::C7(v11) => {
            let a56 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a57 = Str::lit(" ");
            join(a56, a57)
        }
        Adt96::C8(v12) => {
            let a58 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a59 = Str::lit(" ");
            join(a58, a59)
        }
        Adt96::C9(v13) => {
            let a60 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a61 = Str::lit(" ");
            join(a60, a61)
        }
        Adt96::C10(v14) => {
            let a62 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a63 = Str::lit(" ");
            join(a62, a63)
        }
        Adt96::C11(v15) => {
            let a64 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a65 = Str::lit(" ");
            join(a64, a65)
        }
        Adt96::C12(v16, v17) => {
            let a66 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a67 = Str::lit(" ");
            join(a66, a67)
        }
    }
}

pub fn f23(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt89> {
    let m69 = match v2 {
        Adt20::C0(_, h68) => {
            h68
        }
    };
    match m69 {
        None => {
            Ok::<Adt32, Adt89>(Adt32::C1(Adt34::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt89>(Adt89::C5(v3))
        }
    }
}

pub fn f24(v0: List<Adt35>, v1: Fn2, v2: Adt21) -> R<Result<Adt32, Adt89>> {
    let m71 = f42(match v2.clone() {
        Adt21::C0(_, h70, _, _, _) => {
            h70
        }
    }, v0);
    match m71 {
        None => {
            Ok(Err::<Adt32, Adt89>(Adt89::C0(match v2 {
                Adt21::C0(_, h72, _, _, _) => {
                    h72
                }
            })))
        }
        Some(v3) => {
            f43(v3, v1, v2)
        }
    }
}

pub fn f25(v0: Adt21) -> Result<Adt49, Adt89> {
    Ok::<Adt49, Adt89>(f44(List::cons(f45(match v0 {
        Adt21::C0(_, h73, _, _, _) => {
            h73
        }
    }), List::<Adt38>::nil())))
}

pub fn f26(v0: Bytes, v1: u64) -> R<Option<Adt97>> {
    let m74 = f46(v0.clone(), v1)?;
    match m74 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v2) => {
            let m80 = {
                let a77 = v0;
                let a78 = {
                    let a75 = v1;
                    let a76 = 4u64;
                    nat_add(a75, a76)?
                };
                let a79 = v2;
                slice_bytes(a77, a78, a79)
            };
            match m80 {
                None => {
                    Ok(None::<Adt97>)
                }
                Some(v3) => {
                    Ok(Some(Adt97::C0(v3, {
                        let a83 = {
                            let a81 = v1;
                            let a82 = 4u64;
                            nat_add(a81, a82)?
                        };
                        let a84 = v2;
                        nat_add(a83, a84)?
                    })))
                }
            }
        }
    }
}

pub fn f27(v0: Bytes, v1: Adt97, v2: Option<Adt97>) -> R<Result<Adt92, Adt94>> {
    let m85 = v2;
    match m85 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(match v1 {
                Adt97::C0(_, h86) => {
                    h86
                }
            })))
        }
        Some(v3) => {
            f47(v0.clone(), v1, v3.clone(), f46(v0, match v3 {
                Adt97::C0(_, h87) => {
                    h87
                }
            })?)
        }
    }
}

pub fn f28(v0: u64) -> List<u64> {
    List::cons({
        let a91 = {
            let a88 = v0;
            let a89 = 16777216u64;
            let a90 = 0u64;
            nat_quot(a88, a89, a90)
        };
        let a92 = 256u64;
        let a93 = 0u64;
        nat_rem(a91, a92, a93)
    }, List::cons({
        let a97 = {
            let a94 = v0;
            let a95 = 65536u64;
            let a96 = 0u64;
            nat_quot(a94, a95, a96)
        };
        let a98 = 256u64;
        let a99 = 0u64;
        nat_rem(a97, a98, a99)
    }, List::cons({
        let a103 = {
            let a100 = v0;
            let a101 = 256u64;
            let a102 = 0u64;
            nat_quot(a100, a101, a102)
        };
        let a104 = 256u64;
        let a105 = 0u64;
        nat_rem(a103, a104, a105)
    }, List::cons({
        let a106 = v0;
        let a107 = 256u64;
        let a108 = 0u64;
        nat_rem(a106, a107, a108)
    }, List::<u64>::nil()))))
}

pub fn f29(v0: List<Adt93>) -> List<u64> {
    f48(f49(v0, List::<Adt93>::nil()), List::<u64>::nil())
}

pub fn f30(v0: List<List<u64>>) -> List<u64> {
    f50(f51(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f31(v0: Option<List<u64>>) -> List<u64> {
    let m109 = v0;
    match m109 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a113 = f28(1u64);
            let a114 = {
                let a111 = f28({
                    let a110 = v1.clone();
                    length_list(a110)
                });
                let a112 = v1;
                append_list(a111, a112)
            };
            append_list(a113, a114)
        }
    }
}

pub fn f32() -> u64 {
    500u64
}

pub fn f33() -> List<Adt93> {
    List::cons(Adt93::C0(f52(), f53()), List::<Adt93>::nil())
}

pub fn f34() -> List<u64> {
    let a117 = f54();
    let a118 = {
        let a115 = f55();
        let a116 = f56();
        append_list(a115, a116)
    };
    append_list(a117, a118)
}

pub fn f35(v0: Adt0, v1: Fn1, v2: Adt92, v3: Option<Adt98>) -> R<Adt95> {
    let m119 = v3;
    match m119 {
        None => {
            f58(v0, v1, v2)
        }
        Some(v4) => {
            f57(v4)
        }
    }
}

pub fn f36(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m120 = f59(v0.clone(), v1.clone())?;
    match m120 {
        None => {
            f60(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f37(v0: List<Adt35>) -> Option<Adt96> {
    let m121 = f61(v0, List::<Str>::nil());
    match m121 {
        None => {
            None::<Adt96>
        }
        Some(v1) => {
            Some(Adt96::C1(v1))
        }
    }
}

pub fn f38(p125: List<Adt35>, p126: List<Adt12>) -> R<Result<List<Adt12>, Adt96>> {
    let m124 = (p125, p126);
    {
        let mut m124 = m124;
        loop {
            let step = {
                let (v0, v1) = m124;
                let m122 = v0;
                match m122.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt96>>>(Ok(Ok::<List<Adt12>, Adt96>(f62(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m123 = f63(v2)?;
                        match m123 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt96>>>((v3, f62(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt96>>>(Ok(Err::<List<Adt12>, Adt96>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m124 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f40(v0: u64) -> R<Str> {
    let a128 = List::cons(Str::lit("crowd_"), List::cons({
        let a127 = f64(v0, 0i64)?;
        format_i64(a127)
    }, List::<Str>::nil()));
    let a129 = Str::lit("");
    Ok(join(a128, a129))
}

pub fn f41() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f42(v0: Str, p135: List<Adt35>) -> Option<Adt35> {
    let m134 = p135;
    {
        let mut m134 = m134;
        loop {
            let step = {
                let v1 = m134;
                let m130 = v1;
                match m130.uncons() {
                    None => {
                        Err::<List<Adt35>, Option<Adt35>>(None::<Adt35>)
                    }
                    Some((v2, v3)) => {
                        let a132 = match v2.clone() {
                            Adt35::C0(h131, _, _, _, _, _, _, _) => {
                                h131
                            }
                        };
                        let a133 = v0.clone();
                        if equal(a132, a133) {
                            Err::<List<Adt35>, Option<Adt35>>(Some(v2))
                        } else {
                            Ok::<List<Adt35>, Option<Adt35>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m134 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Adt35, v1: Fn2, v2: Adt21) -> R<Result<Adt32, Adt89>> {
    f65(v0.clone(), v2, Fn3::F66(v1, v0))
}

pub fn f44(v0: List<Adt38>) -> Adt49 {
    f67(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f45(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f46(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a139 = {
        let a136 = v1;
        let a137 = 4u64;
        nat_add(a136, a137)?
    };
    let a140 = {
        let a138 = v0.clone();
        length_bytes(a138)
    };
    if nat_le(a139, a140) {
        Ok(Some({
            let a157 = {
                let a141 = f68(v0.clone(), v1)?;
                let a142 = 16777216u64;
                nat_mul(a141, a142)?
            };
            let a158 = {
                let a155 = {
                    let a145 = f68(v0.clone(), {
                        let a143 = v1;
                        let a144 = 1u64;
                        nat_add(a143, a144)?
                    })?;
                    let a146 = 65536u64;
                    nat_mul(a145, a146)?
                };
                let a156 = {
                    let a153 = {
                        let a149 = f68(v0.clone(), {
                            let a147 = v1;
                            let a148 = 2u64;
                            nat_add(a147, a148)?
                        })?;
                        let a150 = 256u64;
                        nat_mul(a149, a150)?
                    };
                    let a154 = f68(v0, {
                        let a151 = v1;
                        let a152 = 3u64;
                        nat_add(a151, a152)?
                    })?;
                    nat_add(a153, a154)?
                };
                nat_add(a155, a156)?
            };
            nat_add(a157, a158)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f47(v0: Bytes, v1: Adt97, v2: Adt97, v3: Option<u64>) -> R<Result<Adt92, Adt94>> {
    let m159 = v3;
    match m159 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(match v2 {
                Adt97::C0(_, h160) => {
                    h160
                }
            })))
        }
        Some(v4) => {
            let m164 = f69(v0.clone(), {
                let a162 = match v2.clone() {
                    Adt97::C0(_, h161) => {
                        h161
                    }
                };
                let a163 = 4u64;
                nat_add(a162, a163)?
            }, v4, List::<Adt93>::nil())?;
            match m164 {
                Ok(v6) => {
                    f70(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt92, Adt94>(v5))
                }
            }
        }
    }
}

pub fn f48(p169: List<Adt93>, p170: List<u64>) -> List<u64> {
    let m168 = (p169, p170);
    {
        let mut m168 = m168;
        loop {
            let step = {
                let (v0, v1) = m168;
                let m165 = v0;
                match m165.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<u64>), List<u64>>((v3, {
                            let a166 = f71(v2);
                            let a167 = v1;
                            append_list(a166, a167)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m168 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(p173: List<Adt93>, p174: List<Adt93>) -> List<Adt93> {
    let m172 = (p173, p174);
    {
        let mut m172 = m172;
        loop {
            let step = {
                let (v0, v1) = m172;
                let m171 = v0;
                match m171.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
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

pub fn f50(p179: List<List<u64>>, p180: List<u64>) -> List<u64> {
    let m178 = (p179, p180);
    {
        let mut m178 = m178;
        loop {
            let step = {
                let (v0, v1) = m178;
                let m175 = v0;
                match m175.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a176 = f72(v2);
                            let a177 = v1;
                            append_list(a176, a177)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(p183: List<List<u64>>, p184: List<List<u64>>) -> List<List<u64>> {
    let m182 = (p183, p184);
    {
        let mut m182 = m182;
        loop {
            let step = {
                let (v0, v1) = m182;
                let m181 = v0;
                match m181.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f53() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f54() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f55() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f56() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f57(v0: Adt98) -> R<Adt95> {
    f73(f74(v0.clone()), f75(None::<Adt99>, f76(v0)))
}

pub fn f58(v0: Adt0, v1: Fn1, v2: Adt92) -> R<Adt95> {
    if f77(match v2.clone() {
        Adt92::C0(h185, _, _, _, _, _, _, _) => {
            h185
        }
    }) {
        f78(match v2.clone() {
            Adt92::C0(_, _, _, _, _, _, _, h186) => {
                h186
            }
        }, f79(v0, match v2.clone() {
            Adt92::C0(_, _, _, _, _, _, h187, _) => {
                h187
            }
        }), Fn0::F80(std::rc::Rc::new(v1), v2.clone()), match v2.clone() {
            Adt92::C0(_, _, _, _, h188, _, _, _) => {
                h188
            }
        }, match v2.clone() {
            Adt92::C0(_, _, h189, _, _, _, _, _) => {
                h189
            }
        }, f81(match v2 {
            Adt92::C0(_, _, _, h190, _, _, _, _) => {
                h190
            }
        })?)
    } else {
        Ok(f82())
    }
}

pub fn f59(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m191 = f83(v0, f84())?;
    match m191 {
        Adt103::C0 => {
            Ok(None::<Adt98>)
        }
        Adt103::C1 => {
            Ok(Some(Adt98::C0))
        }
        Adt103::C2 => {
            Ok(Some(Adt98::C0))
        }
        Adt103::C3(v2) => {
            f85(f86(v2), v1, Adt98::C0)
        }
    }
}

pub fn f60(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt98>> {
    let m192 = f83(v0, f87())?;
    match m192 {
        Adt103::C0 => {
            Ok(None::<Adt98>)
        }
        Adt103::C1 => {
            Ok(Some(Adt98::C1))
        }
        Adt103::C2 => {
            Ok(Some(Adt98::C1))
        }
        Adt103::C3(v2) => {
            f85(f88(v2), v1, Adt98::C1)
        }
    }
}

pub fn f61(p198: List<Adt35>, p199: List<Str>) -> Option<Str> {
    let m197 = (p198, p199);
    {
        let mut m197 = m197;
        loop {
            let step = {
                let (v0, v1) = m197;
                let m193 = v0;
                match m193.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f89(match v2.clone() {
                            Adt35::C0(h194, _, _, _, _, _, _, _) => {
                                h194
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h195, _, _, _, _, _, _, _) => {
                                    h195
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h196, _, _, _, _, _, _, _) => {
                                    h196
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(p202: List<Adt12>, p203: List<Adt12>) -> List<Adt12> {
    let m201 = (p202, p203);
    {
        let mut m201 = m201;
        loop {
            let step = {
                let (v0, v1) = m201;
                let m200 = v0;
                match m200.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Adt35) -> R<Result<List<Adt12>, Adt96>> {
    let m206 = f90(match v0.clone() {
        Adt35::C0(h204, _, _, _, _, _, _, _) => {
            h204
        }
    }, f91(match v0.clone() {
        Adt35::C0(h205, _, _, _, _, _, _, _) => {
            h205
        }
    })?);
    match m206 {
        None => {
            f92(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt96>(v1))
        }
    }
}

pub fn f64(p209: u64, p210: i64) -> R<i64> {
    let m208 = (p209, p210);
    {
        let mut m208 = m208;
        loop {
            let step = {
                let (v0, v1) = m208;
                let m207 = v0;
                if m207 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m207 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f93(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt35, v1: Adt21, v2: Fn3) -> R<Result<Adt32, Adt89>> {
    let m217 = f94(Adt6::C5({
        let m213 = match v0.clone() {
            Adt35::C0(_, _, _, _, h212, _, _, _) => {
                h212
            }
        };
        match m213 {
            Adt5::C0(h211) => {
                h211
            }
        }
    }), Adt6::C5({
        let m216 = match v0 {
            Adt35::C0(_, _, _, _, h215, _, _, _) => {
                h215
            }
        };
        match m216 {
            Adt5::C0(h214) => {
                h214
            }
        }
    }), f95(v1.clone()))?;
    match m217 {
        Adt106::C0 => {
            let c218: Fn3 = v2;
            c218.apply(v1)
        }
        Adt106::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt89>(Adt32::C2(f96(f97(v3, v4)?, true))))
        }
        Adt106::C2(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt106::C3(_) => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt106::C4 => {
            Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f66(v0: Fn2, v1: Adt35, v2: Adt21) -> R<Result<Adt32, Adt89>> {
    f98(v1, v0, v2)
}

pub fn f67(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f68(v0: Bytes, v1: u64) -> R<u64> {
    let m221 = {
        let a219 = v0;
        let a220 = v1;
        index_bytes(a219, a220)
    };
    match m221 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a266 = {
                let a242 = {
                    let a230 = {
                        let a224 = {
                            let a222 = v3;
                            let a223 = 128u8;
                            fixed_u8::bit_and(a222, a223)
                        };
                        let a225 = 128u8;
                        if equal(a224, a225) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a231 = {
                        let a228 = {
                            let a226 = v3;
                            let a227 = 64u8;
                            fixed_u8::bit_and(a226, a227)
                        };
                        let a229 = 64u8;
                        if equal(a228, a229) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a230, a231)?
                };
                let a243 = {
                    let a240 = {
                        let a234 = {
                            let a232 = v3;
                            let a233 = 32u8;
                            fixed_u8::bit_and(a232, a233)
                        };
                        let a235 = 32u8;
                        if equal(a234, a235) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a241 = {
                        let a238 = {
                            let a236 = v3;
                            let a237 = 16u8;
                            fixed_u8::bit_and(a236, a237)
                        };
                        let a239 = 16u8;
                        if equal(a238, a239) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a240, a241)?
                };
                nat_add(a242, a243)?
            };
            let a267 = {
                let a264 = {
                    let a252 = {
                        let a246 = {
                            let a244 = v3;
                            let a245 = 8u8;
                            fixed_u8::bit_and(a244, a245)
                        };
                        let a247 = 8u8;
                        if equal(a246, a247) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a253 = {
                        let a250 = {
                            let a248 = v3;
                            let a249 = 4u8;
                            fixed_u8::bit_and(a248, a249)
                        };
                        let a251 = 4u8;
                        if equal(a250, a251) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a252, a253)?
                };
                let a265 = {
                    let a262 = {
                        let a256 = {
                            let a254 = v3;
                            let a255 = 2u8;
                            fixed_u8::bit_and(a254, a255)
                        };
                        let a257 = 2u8;
                        if equal(a256, a257) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a263 = {
                        let a260 = {
                            let a258 = v3;
                            let a259 = 1u8;
                            fixed_u8::bit_and(a258, a259)
                        };
                        let a261 = 1u8;
                        if equal(a260, a261) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a262, a263)?
                };
                nat_add(a264, a265)?
            };
            nat_add(a266, a267)
        }
    }
}

pub fn f69(v0: Bytes, p277: u64, p278: u64, p279: List<Adt93>) -> R<Result<(List<Adt93>, u64), Adt94>> {
    let m276 = (p277, (p278, p279));
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, (v2, v3)) = m276;
                let m268 = v2;
                if m268 == 0 {
                    Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Ok::<(List<Adt93>, u64), Adt94>((f99(v3, List::<Adt93>::nil()), v1))))
                } else {
                    let v4 = m268 - 1;
                    let m269 = f26(v0.clone(), v1)?;
                    match m269 {
                        None => {
                            Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Err::<(List<Adt93>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            let m271 = f26(v0.clone(), match v5.clone() {
                                Adt97::C0(_, h270) => {
                                    h270
                                }
                            })?;
                            match m271 {
                                None => {
                                    Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Err::<(List<Adt93>, u64), Adt94>(Adt94::C0(match v5 {
                                        Adt97::C0(_, h272) => {
                                            h272
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>((match v6.clone() {
                                        Adt97::C0(_, h273) => {
                                            h273
                                        }
                                    }, (v4, List::cons(Adt93::C0(f100(match v5 {
                                        Adt97::C0(h274, _) => {
                                            h274
                                        }
                                    })?, f100(match v6 {
                                        Adt97::C0(h275, _) => {
                                            h275
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Bytes, v1: Adt97, v2: Adt97, v3: (List<Adt93>, u64)) -> R<Result<Adt92, Adt94>> {
    let m281 = f26(v0.clone(), {
        let (_, h280) = v3.clone();
        h280
    })?;
    match m281 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0({
                let (_, h282) = v3;
                h282
            })))
        }
        Some(v4) => {
            let m284 = f46(v0.clone(), match v4.clone() {
                Adt97::C0(_, h283) => {
                    h283
                }
            })?;
            match m284 {
                None => {
                    Ok(Err::<Adt92, Adt94>(Adt94::C0(match v4 {
                        Adt97::C0(_, h285) => {
                            h285
                        }
                    })))
                }
                Some(v5) => {
                    f101(v0.clone(), Adt92::C0(f100(match v1 {
                        Adt97::C0(h286, _) => {
                            h286
                        }
                    })?, f100(match v2 {
                        Adt97::C0(h287, _) => {
                            h287
                        }
                    })?, {
                        let (h288, _) = v3;
                        h288
                    }, match v4.clone() {
                        Adt97::C0(h289, _) => {
                            h289
                        }
                    }, v5, List::<List<u64>>::nil(), Bytes::lit(&[]), Adt91::C0(0i64, Str::lit(""), Str::lit(""), f102())), f103(v0, {
                        let a291 = match v4 {
                            Adt97::C0(_, h290) => {
                                h290
                            }
                        };
                        let a292 = 4u64;
                        nat_add(a291, a292)?
                    })?)
                }
            }
        }
    }
}

pub fn f71(v0: Adt93) -> List<u64> {
    let a303 = f28({
        let a294 = match v0.clone() {
            Adt93::C0(h293, _) => {
                h293
            }
        };
        length_list(a294)
    });
    let a304 = {
        let a301 = match v0.clone() {
            Adt93::C0(h295, _) => {
                h295
            }
        };
        let a302 = {
            let a299 = f28({
                let a297 = match v0.clone() {
                    Adt93::C0(_, h296) => {
                        h296
                    }
                };
                length_list(a297)
            });
            let a300 = match v0 {
                Adt93::C0(_, h298) => {
                    h298
                }
            };
            append_list(a299, a300)
        };
        append_list(a301, a302)
    };
    append_list(a303, a304)
}

pub fn f72(v0: List<u64>) -> List<u64> {
    let a306 = f28({
        let a305 = v0.clone();
        length_list(a305)
    });
    let a307 = v0;
    append_list(a306, a307)
}

pub fn f73(v0: u64, v1: Adt6) -> R<Adt95> {
    let m308 = f104(v1)?;
    match m308 {
        Ok(v3) => {
            Ok(Adt95::C0(v0, f33(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f16())
        }
    }
}

pub fn f74(v0: Adt98) -> u64 {
    let m309 = v0;
    match m309 {
        Adt98::C0 => {
            f105()
        }
        Adt98::C1 => {
            f105()
        }
    }
}

pub fn f75(v0: Option<Adt99>, v1: Adt80) -> Adt6 {
    Adt6::C5(f106(v0, v1))
}

pub fn f76(v0: Adt98) -> Adt80 {
    Adt80::C0(f107(), f108(v0), None::<Adt6>)
}

pub fn f77(v0: List<u64>) -> bool {
    f109(v0, f110())
}

pub fn f78(v0: Adt91, v1: Adt0, v2: Fn0, v3: u64, v4: List<Adt93>, v5: Result<Adt6, Adt100>) -> R<Adt95> {
    let m310 = v5;
    match m310 {
        Ok(v7) => {
            f113(v0, v1, v2, v3, v4, f114(v7)?)
        }
        Err(_) => {
            f73(f111(), f75(None::<Adt99>, f112(Adt101::C0)))
        }
    }
}

pub fn f79(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h311, _, _, _, _, _, _, _) => {
            h311
        }
    }, match v0.clone() {
        Adt0::C0(_, h312, _, _, _, _, _, _) => {
            h312
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h313, _, _, _, _, _) => {
            h313
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h314, _, _, _, _) => {
            h314
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h315, _, _, _) => {
            h315
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h316, _, _) => {
            h316
        }
    }, v1, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h317) => {
            h317
        }
    })
}

pub fn f80(v0: Fn1, v1: Adt92, v2: Adt14) -> R<Result<Adt32, Adt89>> {
    let c318: Fn1 = v0;
    c318.apply(match v1 {
        Adt92::C0(_, _, _, _, _, _, _, h319) => {
            h319
        }
    }, v2)
}

pub fn f81(v0: Bytes) -> R<Result<Adt6, Adt100>> {
    f115(f100(v0)?)
}

pub fn f82() -> Adt95 {
    Adt95::C0(f116(), List::cons(Adt93::C0(f117(), f118()), List::<Adt93>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f83(v0: List<Adt93>, v1: List<u64>) -> R<Adt103> {
    let m320 = f119(v0, v1, Adt104::C0)?;
    match m320 {
        Adt104::C0 => {
            Ok(Adt103::C0)
        }
        Adt104::C1(v2) => {
            let m321 = f120(v2);
            match m321 {
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

pub fn f84() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f85(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt98) -> R<Option<Adt98>> {
    let m322 = v0;
    match m322 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f121(v3, v1)? {
                Ok(None::<Adt98>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f86(v0: List<u64>) -> Option<List<u64>> {
    let m323 = f122(v0);
    match m323 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f88(v1)
        }
    }
}

pub fn f87() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f88(v0: List<u64>) -> Option<List<u64>> {
    let m324 = v0.clone();
    match m324.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a325 = v1;
            let a326 = 91u64;
            if nat_eq(a325, a326) {
                f123(f124(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f123(f125(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f89(v0: Str, p331: List<Str>) -> bool {
    let m330 = p331;
    {
        let mut m330 = m330;
        loop {
            let step = {
                let v1 = m330;
                let m327 = v1;
                match m327.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a328 = v0.clone();
                        let a329 = v2;
                        if equal(a328, a329) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Str, v1: List<u64>) -> Option<Adt96> {
    if f126(v1) {
        Some(Adt96::C0(v0))
    } else {
        None::<Adt96>
    }
}

pub fn f91(v0: Str) -> R<List<u64>> {
    f100({
        let a332 = v0;
        utf8_encode(a332)
    })
}

pub fn f92(v0: Adt35) -> R<Result<List<Adt12>, Adt96>> {
    let m333 = f127(v0.clone());
    match m333 {
        None => {
            f128(v0.clone(), f129(f130(), match v0.clone() {
                Adt35::C0(h334, _, _, _, _, _, _, _) => {
                    h334
                }
            }, Adt6::C5({
                let m337 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h336, _, _, _) => {
                        h336
                    }
                };
                match m337 {
                    Adt5::C0(h335) => {
                        h335
                    }
                }
            }), List::cons(Adt105::C0(Adt6::C5({
                let m340 = match v0 {
                    Adt35::C0(_, _, _, _, h339, _, _, _) => {
                        h339
                    }
                };
                match m340 {
                    Adt5::C0(h338) => {
                        h338
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt105>::nil()), f131())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt96>(v1))
        }
    }
}

pub fn f93(v0: i64, v1: i64) -> R<i64> {
    let a342 = v0;
    let a343 = {
        let a341 = v1;
        int_neg(a341)?
    };
    int_sub(a342, a343)
}

pub fn f94(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt106> {
    f132(v0, f133(), Adt108::C0, List::cons(Adt109::C0(v1, v2, List::<Adt107>::nil()), List::<Adt109>::nil()))
}

pub fn f95(v0: Adt21) -> Adt6 {
    let m345 = match v0 {
        Adt21::C0(_, _, h344, _, _) => {
            h344
        }
    };
    match m345 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h346) => {
                    h346
                }
            })
        }
    }
}

pub fn f96(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f97(v0: List<Adt107>, v1: Str) -> R<Str> {
    let v2: Str = f134(v0)?;
    let a349 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a347 = v2.clone();
        let a348 = Str::lit("");
        if equal(a347, a348) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a350 = Str::lit(" ");
    Ok(join(a349, a350))
}

pub fn f98(v0: Adt35, v1: Fn2, v2: Adt21) -> R<Result<Adt32, Adt89>> {
    let m352 = {
        let c351: Fn2 = v1;
        c351.apply(v2)
    };
    match m352 {
        Ok(v4) => {
            f135(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt89>(v3))
        }
    }
}

pub fn f99(p355: List<Adt93>, p356: List<Adt93>) -> List<Adt93> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v0, v1) = m354;
                let m353 = v0;
                match m353.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
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

pub fn f100(v0: Bytes) -> R<List<u64>> {
    f136(v0.clone(), {
        let a357 = v0;
        length_bytes(a357)
    }, List::<u64>::nil())
}

pub fn f101(v0: Bytes, v1: Adt92, v2: Result<(Adt91, u64), Adt94>) -> R<Result<Adt92, Adt94>> {
    let m358 = v2;
    match m358 {
        Ok(v4) => {
            f137(v0, Adt92::C0(match v1.clone() {
                Adt92::C0(h359, _, _, _, _, _, _, _) => {
                    h359
                }
            }, match v1.clone() {
                Adt92::C0(_, h360, _, _, _, _, _, _) => {
                    h360
                }
            }, match v1.clone() {
                Adt92::C0(_, _, h361, _, _, _, _, _) => {
                    h361
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, h362, _, _, _, _) => {
                    h362
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, h363, _, _, _) => {
                    h363
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, _, h364, _, _) => {
                    h364
                }
            }, match v1 {
                Adt92::C0(_, _, _, _, _, _, h365, _) => {
                    h365
                }
            }, {
                let (h366, _) = v4;
                h366
            }), {
                let (_, h367) = v4;
                h367
            })
        }
        Err(v3) => {
            Ok(Err::<Adt92, Adt94>(v3))
        }
    }
}

pub fn f102() -> Bytes {
    Bytes::lit(&[])
}

pub fn f103(v0: Bytes, v1: u64) -> R<Result<(Adt91, u64), Adt94>> {
    let m368 = f138(v0.clone(), v1)?;
    match m368 {
        None => {
            Ok(Err::<(Adt91, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m371 = f139(v0.clone(), {
                let a369 = v1;
                let a370 = 8u64;
                nat_add(a369, a370)?
            })?;
            match m371 {
                Ok(v4) => {
                    f140(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt91, u64), Adt94>(v3))
                }
            }
        }
    }
}

pub fn f104(v0: Adt6) -> R<Result<List<u64>, Adt100>> {
    f141(f142(), v0)
}

pub fn f105() -> u64 {
    403u64
}

pub fn f106(v0: Option<Adt99>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f143())), {
        let a372 = f144(v0);
        let a373 = List::cons((Str::lit("error"), f145(v1)), List::<(Str, Adt6)>::nil());
        append_list(a372, a373)
    })
}

pub fn f107() -> i64 {
    -32000i64
}

pub fn f108(v0: Adt98) -> Str {
    let m374 = v0;
    match m374 {
        Adt98::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt98::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f109(p381: List<u64>, p382: List<u64>) -> bool {
    let m380 = (p381, p382);
    {
        let mut m380 = m380;
        loop {
            let step = {
                let (v0, v1) = m380;
                let m375 = v0;
                match m375.uncons() {
                    None => {
                        let m376 = v1;
                        match m376.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m377 = v1;
                        match m377.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a378 = v4;
                                let a379 = v6;
                                if nat_eq(a378, a379) {
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
                Ok(next) => { m380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f111() -> u64 {
    400u64
}

pub fn f112(v0: Adt101) -> Adt80 {
    Adt80::C0(f146(v0.clone()), f147(v0), None::<Adt6>)
}

pub fn f113(v0: Adt91, v1: Adt0, v2: Fn0, v3: u64, v4: List<Adt93>, v5: Result<Adt111, Adt101>) -> R<Adt95> {
    let m383 = v5;
    match m383 {
        Ok(v7) => {
            f150(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f73(f148(v6.clone()), f75(f149(v6.clone()), f112(v6)))
        }
    }
}

pub fn f114(v0: Adt6) -> R<Result<Adt111, Adt101>> {
    let m384 = v0;
    match m384 {
        Adt6::C0 => {
            Ok(f152(None::<Adt99>, Adt102::C0))
        }
        Adt6::C1(_) => {
            Ok(f152(None::<Adt99>, Adt102::C0))
        }
        Adt6::C2(_) => {
            Ok(f152(None::<Adt99>, Adt102::C0))
        }
        Adt6::C3(_) => {
            Ok(f152(None::<Adt99>, Adt102::C0))
        }
        Adt6::C4(_) => {
            Ok(f152(None::<Adt99>, Adt102::C1))
        }
        Adt6::C5(v1) => {
            f151(Adt5::C0(v1))
        }
    }
}

pub fn f115(v0: List<u64>) -> R<Result<Adt6, Adt100>> {
    let m385 = f153(v0.clone())?;
    match m385 {
        Adt120::C0 => {
            f154(f155(), 0u64, v0)
        }
        Adt120::C1(v1) => {
            Ok(Err::<Adt6, Adt100>(Adt100::C0(v1)))
        }
        Adt120::C2(v2) => {
            Ok(Err::<Adt6, Adt100>(Adt100::C1(v2)))
        }
    }
}

pub fn f116() -> u64 {
    405u64
}

pub fn f117() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f118() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f119(p390: List<Adt93>, v1: List<u64>, p391: Adt104) -> R<Adt104> {
    let m389 = (p390, p391);
    {
        let mut m389 = m389;
        loop {
            let step = {
                let (v0, v2) = m389;
                let m386 = v0;
                match m386.uncons() {
                    None => {
                        Err::<(List<Adt93>, Adt104), R<Adt104>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f156(match v3.clone() {
                            Adt93::C0(h387, _) => {
                                h387
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt93>, Adt104), R<Adt104>>((v4, f157(v2, match v3 {
                                Adt93::C0(_, h388) => {
                                    h388
                                }
                            })))
                        } else {
                            Ok::<(List<Adt93>, Adt104), R<Adt104>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f158(v0);
    if f159(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f121(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f156(v0.clone(), f160())? {
        Ok(true)
    } else {
        if f156(v0.clone(), f161())? {
            Ok(true)
        } else {
            if f156(v0.clone(), f162())? {
                Ok(true)
            } else {
                f163(v0, v1)
            }
        }
    }
}

pub fn f122(p396: List<u64>) -> Option<List<u64>> {
    let m395 = p396;
    {
        let mut m395 = m395;
        loop {
            let step = {
                let v0 = m395;
                let m392 = v0;
                match m392.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a393 = v1;
                        let a394 = 58u64;
                        if nat_eq(a393, a394) {
                            Err::<List<u64>, Option<List<u64>>>(f164(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m397 = v0;
    match m397 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f165({
                let (_, h398) = v1;
                h398
            }) {
                Some({
                    let (h399, _) = v1;
                    h399
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f124(p404: List<u64>, p405: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m403 = (p404, p405);
    {
        let mut m403 = m403;
        loop {
            let step = {
                let (v0, v1) = m403;
                let m400 = v0;
                match m400.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a401 = v2;
                        let a402 = 93u64;
                        if nat_eq(a401, a402) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f166(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(p410: List<u64>, p411: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m409 = (p410, p411);
    {
        let mut m409 = m409;
        loop {
            let step = {
                let (v0, v1) = m409;
                let m406 = v0;
                match m406.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f166(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a407 = v2;
                        let a408 = 58u64;
                        if nat_eq(a407, a408) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f166(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: List<u64>) -> bool {
    let m412 = v0.clone();
    match m412.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a416 = if f167(v0.clone(), false) {
                let a414 = {
                    let a413 = v0;
                    length_list(a413)
                };
                let a415 = f168();
                nat_le(a414, a415)
            } else {
                false
            };
            bool_not(a416)
        }
    }
}

pub fn f127(v0: Adt35) -> Option<Adt96> {
    let m421 = f169(match v0.clone() {
        Adt35::C0(h417, _, _, _, _, _, _, _) => {
            h417
        }
    }, {
        let m420 = match v0.clone() {
            Adt35::C0(_, _, _, _, h419, _, _, _) => {
                h419
            }
        };
        match m420 {
            Adt5::C0(h418) => {
                h418
            }
        }
    });
    match m421 {
        None => {
            f170(match v0.clone() {
                Adt35::C0(h422, _, _, _, _, _, _, _) => {
                    h422
                }
            }, {
                let m425 = match v0 {
                    Adt35::C0(_, _, _, _, h424, _, _, _) => {
                        h424
                    }
                };
                match m425 {
                    Adt5::C0(h423) => {
                        h423
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f128(v0: Adt35, v1: Result<Adt112, Adt96>) -> R<Result<List<Adt12>, Adt96>> {
    let m426 = v1;
    match m426 {
        Ok(v3) => {
            let m427 = f171(v0)?;
            match m427 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt96>(f62(match v3 {
                        Adt112::C0(h428, _) => {
                            h428
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt96>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt96>(v2))
        }
    }
}

pub fn f129(p435: u64, v1: Str, v2: Adt6, p436: List<Adt105>, p437: Adt112) -> R<Result<Adt112, Adt96>> {
    let m434 = (p435, (p436, p437));
    {
        let mut m434 = m434;
        loop {
            let step = {
                let (v0, (v3, v4)) = m434;
                let m429 = v0;
                if m429 == 0 {
                    Err::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt96>>>(Ok(Err::<Adt112, Adt96>(Adt96::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m429 - 1;
                    let m430 = v3;
                    match m430.uncons() {
                        None => {
                            Err::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt96>>>(Ok(Ok::<Adt112, Adt96>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m431 = f172(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m431 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt96>>>((v5, ({
                                        let (h432, _) = v9;
                                        h432
                                    }, {
                                        let (_, h433) = v9;
                                        h433
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt105>, Adt112)), R<Result<Adt112, Adt96>>>(Ok(Err::<Adt112, Adt96>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130() -> u64 {
    67108864u64
}

pub fn f131() -> Adt112 {
    Adt112::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f132(v0: Adt6, p456: u64, p457: Adt108, p458: List<Adt109>) -> R<Adt106> {
    let m455 = (p456, (p457, p458));
    {
        let mut m455 = m455;
        loop {
            let step = {
                let (v1, (v2, v3)) = m455;
                let m438 = v1;
                if m438 == 0 {
                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C4))
                } else {
                    let v4 = m438 - 1;
                    let m439 = v3;
                    match m439.uncons() {
                        None => {
                            let m440 = v2;
                            match m440 {
                                Adt108::C0 => {
                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C0))
                                }
                                Adt108::C1(v5, v6) => {
                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m441 = v2.clone();
                            match m441 {
                                Adt108::C0 => {
                                    let m442 = v7.clone();
                                    match m442 {
                                        Adt109::C0(v9, v10, v11) => {
                                            let m443 = f173(v0.clone(), v9, v10, v11.clone())?;
                                            match m443 {
                                                Adt128::C0(v12) => {
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v11, v12), v8)))
                                                }
                                                Adt128::C1(v13) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C2(v13)))
                                                }
                                                Adt128::C2(v14) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C3(v14)))
                                                }
                                                Adt128::C3(v15) => {
                                                    let v16: List<Adt109> = {
                                                        let a444 = v15;
                                                        let a445 = v8;
                                                        append_list(a444, a445)
                                                    };
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt109::C1(_, _, _) => {
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v8)))
                                        }
                                        Adt109::C2(_, _, _, _) => {
                                            let v24: Adt113 = f174(v7, v8)?;
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v24.clone() {
                                                Adt113::C0(h446, _) => {
                                                    h446
                                                }
                                            }, match v24 {
                                                Adt113::C0(_, h447) => {
                                                    h447
                                                }
                                            })))
                                        }
                                        Adt109::C3(v25) => {
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt109::C4(_, _, _, _) => {
                                            let v39: Adt113 = f174(v7, v8)?;
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v39.clone() {
                                                Adt113::C0(h451, _) => {
                                                    h451
                                                }
                                            }, match v39 {
                                                Adt113::C0(_, h452) => {
                                                    h452
                                                }
                                            })))
                                        }
                                        Adt109::C5(v26, v27, v28, v29) => {
                                            let m448 = f175(v26, v27, v28, v29.clone());
                                            match m448 {
                                                Adt128::C0(v30) => {
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v29, v30), v8)))
                                                }
                                                Adt128::C1(v31) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C2(v31)))
                                                }
                                                Adt128::C2(v32) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C3(v32)))
                                                }
                                                Adt128::C3(v33) => {
                                                    let v34: List<Adt109> = {
                                                        let a449 = v33;
                                                        let a450 = v8;
                                                        append_list(a449, a450)
                                                    };
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt108::C1(v40, v41) => {
                                    let v42: Adt113 = f176(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v42.clone() {
                                        Adt113::C0(h453, _) => {
                                            h453
                                        }
                                    }, match v42 {
                                        Adt113::C0(_, h454) => {
                                            h454
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133() -> u64 {
    67108864u64
}

pub fn f134(v0: List<Adt107>) -> R<Str> {
    let a459 = List::cons(Str::lit(""), f177(v0, List::<Str>::nil())?);
    let a460 = Str::lit("/");
    Ok(join(a459, a460))
}

pub fn f135(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt89>> {
    if f178({
        let m463 = match v1.clone() {
            Adt49::C0(h462, _) => {
                h462
            }
        };
        match m463 {
            Adt37::C0(h461, _, _) => {
                h461
            }
        }
    })? {
        f179(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt89>(Adt89::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f136(v0: Bytes, p466: u64, p467: List<u64>) -> R<List<u64>> {
    let m465 = (p466, p467);
    {
        let mut m465 = m465;
        loop {
            let step = {
                let (v1, v2) = m465;
                let m464 = v1;
                if m464 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m464 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f68(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Bytes, v1: Adt92, v2: u64) -> R<Result<Adt92, Adt94>> {
    let m468 = f180(v0.clone(), v2)?;
    match m468 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(v2)))
        }
        Some(v3) => {
            f181(v0, Adt92::C0(match v1.clone() {
                Adt92::C0(h469, _, _, _, _, _, _, _) => {
                    h469
                }
            }, match v1.clone() {
                Adt92::C0(_, h470, _, _, _, _, _, _) => {
                    h470
                }
            }, match v1.clone() {
                Adt92::C0(_, _, h471, _, _, _, _, _) => {
                    h471
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, h472, _, _, _, _) => {
                    h472
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, h473, _, _, _) => {
                    h473
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, _, h474, _, _) => {
                    h474
                }
            }, match v3.clone() {
                Adt97::C0(h475, _) => {
                    h475
                }
            }, match v1 {
                Adt92::C0(_, _, _, _, _, _, _, h476) => {
                    h476
                }
            }), match v3 {
                Adt97::C0(_, h477) => {
                    h477
                }
            })
        }
    }
}

pub fn f138(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a481 = {
        let a478 = v1;
        let a479 = 8u64;
        nat_add(a478, a479)?
    };
    let a482 = {
        let a480 = v0.clone();
        length_bytes(a480)
    };
    if nat_le(a481, a482) {
        Ok(Some(f182(f182(f182(f182(f182(f182(f182(f182(0i64, f68(v0.clone(), v1)?)?, f68(v0.clone(), {
            let a483 = v1;
            let a484 = 1u64;
            nat_add(a483, a484)?
        })?)?, f68(v0.clone(), {
            let a485 = v1;
            let a486 = 2u64;
            nat_add(a485, a486)?
        })?)?, f68(v0.clone(), {
            let a487 = v1;
            let a488 = 3u64;
            nat_add(a487, a488)?
        })?)?, f68(v0.clone(), {
            let a489 = v1;
            let a490 = 4u64;
            nat_add(a489, a490)?
        })?)?, f68(v0.clone(), {
            let a491 = v1;
            let a492 = 5u64;
            nat_add(a491, a492)?
        })?)?, f68(v0.clone(), {
            let a493 = v1;
            let a494 = 6u64;
            nat_add(a493, a494)?
        })?)?, f68(v0, {
            let a495 = v1;
            let a496 = 7u64;
            nat_add(a495, a496)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f139(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt94>> {
    let m497 = f26(v0, v1)?;
    match m497 {
        None => {
            Ok(Err::<(Str, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m499 = f183(f100(match v2.clone() {
                Adt97::C0(h498, _) => {
                    h498
                }
            })?);
            match m499 {
                None => {
                    Ok(Err::<(Str, u64), Adt94>(Adt94::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt94>((v3, match v2 {
                        Adt97::C0(_, h500) => {
                            h500
                        }
                    })))
                }
            }
        }
    }
}

pub fn f140(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt91, u64), Adt94>> {
    let m502 = f139(v0.clone(), {
        let (_, h501) = v2;
        h501
    })?;
    match m502 {
        Ok(v4) => {
            f184(v0, v1, {
                let (h503, _) = v2;
                h503
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt91, u64), Adt94>(v3))
        }
    }
}

pub fn f141(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt100>> {
    let m504 = f185(v0, List::cons(Adt114::C1(0u64, v1), List::<Adt114>::nil()), List::<u64>::nil())?;
    match m504 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt100>(f186(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt100>(v3))
        }
    }
}

pub fn f142() -> u64 {
    67108864u64
}

pub fn f143() -> Str {
    Str::lit("2.0")
}

pub fn f144(v0: Option<Adt99>) -> List<(Str, Adt6)> {
    let m505 = v0;
    match m505 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f187(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f145(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h506, _, _) => {
            h506
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h507, _) => {
            h507
        }
    })), f188(match v0 {
        Adt80::C0(_, _, h508) => {
            h508
        }
    }))))
}

pub fn f146(v0: Adt101) -> i64 {
    let m509 = v0;
    match m509 {
        Adt101::C0 => {
            f189()
        }
        Adt101::C1(_, _) => {
            f190()
        }
    }
}

pub fn f147(v0: Adt101) -> Str {
    let m510 = v0;
    match m510 {
        Adt101::C0 => {
            Str::lit("Parse error")
        }
        Adt101::C1(_, v2) => {
            f191(v2)
        }
    }
}

pub fn f148(v0: Adt101) -> u64 {
    let m511 = v0;
    match m511 {
        Adt101::C0 => {
            f111()
        }
        Adt101::C1(_, _) => {
            f111()
        }
    }
}

pub fn f149(v0: Adt101) -> Option<Adt99> {
    let m512 = v0;
    match m512 {
        Adt101::C0 => {
            None::<Adt99>
        }
        Adt101::C1(v1, _) => {
            v1
        }
    }
}

pub fn f150(v0: Adt91, v1: Adt0, v2: Fn0, v3: u64, v4: List<Adt93>, v5: Adt111) -> R<Adt95> {
    f192(v0, f193(v4.clone())?, f194(v1.clone(), v2, v3, Some(f195(v4, match v1 {
        Adt0::C0(_, _, _, _, h513, _, _, _) => {
            h513
        }
    }, f196(v5.clone()), match v5.clone() {
        Adt111::C0(_, h514, _) => {
            h514
        }
    })?), f197(), v5)?)
}

pub fn f151(v0: Adt5) -> R<Result<Adt111, Adt101>> {
    let m515 = f198(v0.clone())?;
    match m515 {
        Adt136::C0 => {
            let m516 = f199(Str::lit("method"), v0.clone());
            match m516 {
                Adt137::C0 => {
                    Ok(f152(None::<Adt99>, f200(v0)))
                }
                Adt137::C1(_) => {
                    Ok(f152(None::<Adt99>, if f201(v0) {
                        Adt102::C8
                    } else {
                        Adt102::C7
                    }))
                }
            }
        }
        Adt136::C1(v2) => {
            Ok(f202(v2, v0))
        }
        Adt136::C2 => {
            Ok(f152(None::<Adt99>, Adt102::C5))
        }
    }
}

pub fn f152(v0: Option<Adt99>, v1: Adt102) -> Result<Adt111, Adt101> {
    Err::<Adt111, Adt101>(Adt101::C1(v0, v1))
}

pub fn f153(v0: List<u64>) -> R<Adt120> {
    Ok(f203({
        let a517 = v0.clone();
        length_list(a517)
    }, f204(Adt121::C0, 0u64, 0u64, v0)?))
}

pub fn f154(p524: Adt122, p525: u64, p526: List<u64>) -> R<Result<Adt6, Adt100>> {
    let m523 = (p524, (p525, p526));
    {
        let mut m523 = m523;
        loop {
            let step = {
                let (v0, (v1, v2)) = m523;
                let m518 = v2;
                match m518.uncons() {
                    None => {
                        let m519 = f205(v0, v1)?;
                        match m519 {
                            Ok(v3) => {
                                Err::<(Adt122, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Ok::<Adt6, Adt100>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt122, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Err::<Adt6, Adt100>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m520 = f206(v0, v1, v5)?;
                        match m520 {
                            Adt138::C0(v7) => {
                                Err::<(Adt122, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Err::<Adt6, Adt100>(v7)))
                            }
                            Adt138::C1(v8) => {
                                Ok::<(Adt122, (u64, List<u64>)), R<Result<Adt6, Adt100>>>((v8, ({
                                    let a521 = v1;
                                    let a522 = 1u64;
                                    nat_add(a521, a522)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155() -> Adt122 {
    Adt122::C0(Adt123::C0, List::<Adt127>::nil(), 0u64)
}

pub fn f156(p533: List<u64>, p534: List<u64>) -> R<bool> {
    let m532 = (p533, p534);
    {
        let mut m532 = m532;
        loop {
            let step = {
                let (v0, v1) = m532;
                let m527 = v0;
                match m527.uncons() {
                    None => {
                        let m528 = v1;
                        match m528.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m529 = v1;
                        match m529.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a530 = f207(v4)?;
                                let a531 = f207(v6)?;
                                if nat_eq(a530, a531) {
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
                Ok(next) => { m532 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: Adt104, v1: List<u64>) -> Adt104 {
    let m535 = v0;
    match m535 {
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

pub fn f158(v0: List<u64>) -> List<u64> {
    f166(f208(f166(f208(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f159(p538: List<u64>) -> bool {
    let m537 = p538;
    {
        let mut m537 = m537;
        loop {
            let step = {
                let v0 = m537;
                let m536 = v0;
                match m536.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f209(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f161() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f162() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f163(v0: List<u64>, p541: List<List<u64>>) -> R<bool> {
    let m540 = p541;
    {
        let mut m540 = m540;
        loop {
            let step = {
                let v1 = m540;
                let m539 = v1;
                match m539.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f156(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
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

pub fn f164(v0: List<u64>) -> Option<List<u64>> {
    let m542 = v0;
    match m542.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m543 = v2;
            match m543.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a544 = v1;
                    let a545 = 47u64;
                    if if nat_eq(a544, a545) {
                        let a546 = v3;
                        let a547 = 47u64;
                        nat_eq(a546, a547)
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

pub fn f165(v0: List<u64>) -> bool {
    let m548 = v0;
    match m548.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a549 = v1;
            let a550 = 58u64;
            if nat_eq(a549, a550) {
                let m551 = v2.clone();
                match m551.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f210(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f166(p554: List<u64>, p555: List<u64>) -> List<u64> {
    let m553 = (p554, p555);
    {
        let mut m553 = m553;
        loop {
            let step = {
                let (v0, v1) = m553;
                let m552 = v0;
                match m552.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(p558: List<u64>, v1: bool) -> bool {
    let m557 = p558;
    {
        let mut m557 = m557;
        loop {
            let step = {
                let v0 = m557;
                let m556 = v0;
                match m556.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f211(v2)
                        } else {
                            f212(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f168() -> u64 {
    64u64
}

pub fn f169(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt96> {
    let m559 = f213(Str::lit("type"), v1);
    match m559 {
        Adt137::C0 => {
            Some(Adt96::C3(v0))
        }
        Adt137::C1(v2) => {
            let m560 = f214(v2);
            match m560 {
                None => {
                    Some(Adt96::C3(v0))
                }
                Some(v3) => {
                    let a561 = v3;
                    let a562 = Str::lit("object");
                    if equal(a561, a562) {
                        None::<Adt96>
                    } else {
                        Some(Adt96::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f170(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt96> {
    let m563 = f213(Str::lit("$schema"), v1);
    match m563 {
        Adt137::C0 => {
            None::<Adt96>
        }
        Adt137::C1(v2) => {
            let m564 = f214(v2);
            match m564 {
                None => {
                    Some(Adt96::C2(v0))
                }
                Some(v3) => {
                    let a565 = v3;
                    let a566 = f215();
                    if equal(a565, a566) {
                        None::<Adt96>
                    } else {
                        Some(Adt96::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f171(v0: Adt35) -> R<Option<Adt96>> {
    let m568 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h567, _, _) => {
            h567
        }
    };
    match m568 {
        None => {
            Ok(None::<Adt96>)
        }
        Some(v1) => {
            let m571 = f170(match v0.clone() {
                Adt35::C0(h569, _, _, _, _, _, _, _) => {
                    h569
                }
            }, match v1.clone() {
                Adt5::C0(h570) => {
                    h570
                }
            });
            match m571 {
                None => {
                    let m575 = f129(f130(), match v0 {
                        Adt35::C0(h572, _, _, _, _, _, _, _) => {
                            h572
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h573) => {
                            h573
                        }
                    }), List::cons(Adt105::C0(Adt6::C5(match v1 {
                        Adt5::C0(h574) => {
                            h574
                        }
                    }), false, List::<Str>::nil()), List::<Adt105>::nil()), f131())?;
                    match m575 {
                        Ok(_) => {
                            Ok(None::<Adt96>)
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

pub fn f172(v0: Str, v1: Adt6, v2: Adt105, v3: Adt112, v4: List<Adt105>) -> R<Result<(List<Adt105>, Adt112), Adt96>> {
    let m576 = v2;
    match m576 {
        Adt105::C0(v5, v6, v7) => {
            let m577 = f216(v5.clone());
            match m577 {
                None => {
                    let m578 = f217(v5);
                    match m578 {
                        None => {
                            Ok(Err::<(List<Adt105>, Adt112), Adt96>(Adt96::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f218(v0, v1, match v9 {
                                Adt5::C0(h579) => {
                                    h579
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt105>, Adt112), Adt96>((v4, v3)))
                }
            }
        }
    }
}

pub fn f173(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt107>) -> R<Adt128> {
    let m580 = f216(v1.clone());
    match m580 {
        None => {
            let m581 = f217(v1);
            match m581 {
                None => {
                    Ok(Adt128::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f219(v0, match v5 {
                        Adt5::C0(h582) => {
                            h582
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt128::C3(List::<Adt109>::nil()))
            } else {
                Ok(Adt128::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f174(v0: Adt109, v1: List<Adt109>) -> R<Adt113> {
    let m583 = v0;
    match m583 {
        Adt109::C0(_, _, _) => {
            Ok(Adt113::C0(Adt108::C0, v1))
        }
        Adt109::C1(_, _, _) => {
            Ok(Adt113::C0(Adt108::C0, v1))
        }
        Adt109::C2(v8, v9, v10, v11) => {
            Ok(f220(v8, v9, v10, {
                let a584 = v11;
                let a585 = 1u64;
                nat_add(a584, a585)?
            }, v1))
        }
        Adt109::C3(v12) => {
            Ok(Adt113::C0(Adt108::C1(v12, Str::lit("not")), v1))
        }
        Adt109::C4(v13, _, v15, v16) => {
            Ok(f221(v13, v15, v16, v1))
        }
        Adt109::C5(_, _, _, _) => {
            Ok(Adt113::C0(Adt108::C0, v1))
        }
    }
}

pub fn f175(v0: Adt110, v1: Adt6, v2: Adt6, v3: List<Adt107>) -> Adt128 {
    let m586 = f217(v1);
    match m586 {
        None => {
            Adt128::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m587 = v0;
            match m587 {
                Adt110::C0 => {
                    f222(match v4 {
                        Adt5::C0(h588) => {
                            h588
                        }
                    }, v2, v3)
                }
                Adt110::C1 => {
                    f223(match v4 {
                        Adt5::C0(h589) => {
                            h589
                        }
                    }, v2, v3)
                }
                Adt110::C2 => {
                    f224(match v4 {
                        Adt5::C0(h590) => {
                            h590
                        }
                    }, v2, v3)
                }
                Adt110::C3 => {
                    f225(match v4 {
                        Adt5::C0(h591) => {
                            h591
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f176(v0: Adt109, v1: List<Adt109>, v2: List<Adt107>, v3: Str) -> Adt113 {
    let m592 = v0;
    match m592 {
        Adt109::C0(_, _, _) => {
            Adt113::C0(Adt108::C1(v2, v3), v1)
        }
        Adt109::C1(v7, v8, v9) => {
            f226(v7, v8, v9, v1)
        }
        Adt109::C2(v10, v11, v12, v13) => {
            f220(v10, v11, v12, v13, v1)
        }
        Adt109::C3(_) => {
            Adt113::C0(Adt108::C0, v1)
        }
        Adt109::C4(_, v16, v17, v18) => {
            f221(v16, v17, v18, v1)
        }
        Adt109::C5(_, _, _, _) => {
            Adt113::C0(Adt108::C1(v2, v3), v1)
        }
    }
}

pub fn f177(p595: List<Adt107>, p596: List<Str>) -> R<List<Str>> {
    let m594 = (p595, p596);
    {
        let mut m594 = m594;
        loop {
            let step = {
                let (v0, v1) = m594;
                let m593 = v0;
                match m593.uncons() {
                    None => {
                        Err::<(List<Adt107>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt107>, List<Str>), R<List<Str>>>((v3, List::cons(f227(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(p605: List<Adt38>) -> R<bool> {
    let m604 = p605;
    {
        let mut m604 = m604;
        loop {
            let step = {
                let v0 = m604;
                let m597 = v0;
                match m597.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m598 = v1;
                        match m598 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f228(match v4 {
                                    Adt42::C0(h599, _, _, _) => {
                                        h599
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f228(match v5 {
                                    Adt43::C0(h600, _, _, _) => {
                                        h600
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
                                let m602 = match v7 {
                                    Adt45::C0(h601, _, _) => {
                                        h601
                                    }
                                };
                                match m602 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f228(match v9 {
                                            Adt48::C0(_, _, h603, _) => {
                                                h603
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
                Ok(next) => { m604 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt89>> {
    if f229(match v1.clone() {
        Adt49::C0(h606, _) => {
            h606
        }
    }) {
        f230(v1)
    } else {
        let m608 = match v0 {
            Adt35::C0(_, _, _, _, _, h607, _, _) => {
                h607
            }
        };
        match m608 {
            None => {
                f230(v1)
            }
            Some(v2) => {
                f231(v2, v1)
            }
        }
    }
}

pub fn f180(v0: Bytes, v1: u64) -> R<Option<Adt97>> {
    let a610 = v1;
    let a611 = {
        let a609 = v0.clone();
        length_bytes(a609)
    };
    if nat_eq(a610, a611) {
        Ok(Some(Adt97::C0(Bytes::lit(&[]), v1)))
    } else {
        f26(v0, v1)
    }
}

pub fn f181(v0: Bytes, v1: Adt92, v2: u64) -> R<Result<Adt92, Adt94>> {
    let a613 = v2;
    let a614 = {
        let a612 = v0.clone();
        length_bytes(a612)
    };
    if nat_eq(a613, a614) {
        Ok(Ok::<Adt92, Adt94>(v1))
    } else {
        let m615 = f46(v0.clone(), v2)?;
        match m615 {
            None => {
                Ok(Err::<Adt92, Adt94>(Adt94::C0(v2)))
            }
            Some(v3) => {
                let m618 = f232(v0.clone(), {
                    let a616 = v2;
                    let a617 = 4u64;
                    nat_add(a616, a617)?
                }, v3, List::<List<u64>>::nil())?;
                match m618 {
                    Ok(v5) => {
                        let a621 = {
                            let (_, h619) = v5.clone();
                            h619
                        };
                        let a622 = {
                            let a620 = v0;
                            length_bytes(a620)
                        };
                        if nat_eq(a621, a622) {
                            Ok(Ok::<Adt92, Adt94>(f233(v1, {
                                let (h623, _) = v5;
                                h623
                            })))
                        } else {
                            Ok(Err::<Adt92, Adt94>(Adt94::C1({
                                let (_, h624) = v5;
                                h624
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt92, Adt94>(v4))
                    }
                }
            }
        }
    }
}

pub fn f182(v0: i64, v1: u64) -> R<i64> {
    f93({
        let a625 = v0;
        let a626 = 256i64;
        int_mul(a625, a626)?
    }, f64(v1, 0i64)?)
}

pub fn f183(v0: List<u64>) -> Option<Str> {
    let a627 = f234(f102(), v0);
    utf8_decode(a627)
}

pub fn f184(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt91, u64), Adt94>> {
    let m629 = f26(v0, {
        let (_, h628) = v3.clone();
        h628
    })?;
    match m629 {
        None => {
            Ok(Err::<(Adt91, u64), Adt94>(Adt94::C0({
                let (_, h630) = v3;
                h630
            })))
        }
        Some(v4) => {
            Ok(f235(v1, v2, {
                let (h631, _) = v3;
                h631
            }, v4))
        }
    }
}

pub fn f185(p636: u64, p637: List<Adt114>, p638: List<u64>) -> R<Result<List<u64>, Adt100>> {
    let m635 = (p636, (p637, p638));
    {
        let mut m635 = m635;
        loop {
            let step = {
                let (v0, (v1, v2)) = m635;
                let m632 = v0;
                if m632 == 0 {
                    Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt100>>>(Ok(Err::<List<u64>, Adt100>(Adt100::C13)))
                } else {
                    let v3 = m632 - 1;
                    let m633 = v1;
                    match m633.uncons() {
                        None => {
                            Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt100>>>(Ok(Ok::<List<u64>, Adt100>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m634 = f236(v4, v5, v2)?;
                            match m634 {
                                Adt139::C0(v6) => {
                                    Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt100>>>(Ok(Err::<List<u64>, Adt100>(v6)))
                                }
                                Adt139::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt100>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(p641: List<u64>, p642: List<u64>) -> List<u64> {
    let m640 = (p641, p642);
    {
        let mut m640 = m640;
        loop {
            let step = {
                let (v0, v1) = m640;
                let m639 = v0;
                match m639.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m640 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Adt99) -> Adt6 {
    let m643 = v0;
    match m643 {
        Adt99::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt99::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f188(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m644 = v0;
    match m644 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f189() -> i64 {
    -32700i64
}

pub fn f190() -> i64 {
    -32600i64
}

pub fn f191(v0: Adt102) -> Str {
    let m645 = v0;
    match m645 {
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

pub fn f192(v0: Adt91, v1: bool, v2: Adt129) -> R<Adt95> {
    let m647 = match v2.clone() {
        Adt129::C0(_, h646) => {
            h646
        }
    };
    match m647 {
        Ok(v3) => {
            let m648 = f237(v3.clone())?;
            match m648 {
                None => {
                    Ok(f16())
                }
                Some(v4) => {
                    f238(v0, f239(match v3.clone() {
                        Adt130::C0(h649, _, _) => {
                            h649
                        }
                    }), f240(v1, f73(f241(), f242(match v2 {
                        Adt129::C0(h650, _) => {
                            h650
                        }
                    }, v4))?, f243(f244(match v3.clone() {
                        Adt130::C0(_, _, h651) => {
                            h651
                        }
                    }, List::<List<u64>>::nil())?, f245(f239(match v3 {
                        Adt130::C0(h652, _, _) => {
                            h652
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f73(f246(v5.clone()), f75(Some(match v2 {
                Adt129::C0(h653, _) => {
                    h653
                }
            }), f247(v5)?))
        }
    }
}

pub fn f193(p658: List<Adt93>) -> R<bool> {
    let m657 = p658;
    {
        let mut m657 = m657;
        loop {
            let step = {
                let v0 = m657;
                let m654 = v0;
                match m654.uncons() {
                    None => {
                        Err::<List<Adt93>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f156(match v1.clone() {
                            Adt93::C0(h655, _) => {
                                h655
                            }
                        }, f248())? {
                            if f249(f250(44u64, match v1 {
                                Adt93::C0(_, h656) => {
                                    h656
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
                Ok(next) => { m657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt115>, v4: List<Str>, v5: Adt111) -> R<Adt129> {
    Ok(Adt129::C0(match v5.clone() {
        Adt111::C0(h659, _, _) => {
            h659
        }
    }, f251(v0, v1, v2, f252(v3, v4, v5)?)?))
}

pub fn f195(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt115> {
    let v4: Adt135 = f253(Adt117::C0, f83(v0.clone(), f254())?);
    let v5: Adt135 = f253(Adt117::C1, f83(v0.clone(), f255())?);
    let v6: Adt135 = if f256(v3) {
        f253(Adt117::C2, f257(f83(v0.clone(), f258())?)?)
    } else {
        f259()
    };
    Ok(Adt115::C0(match v4.clone() {
        Adt135::C0(h660, _) => {
            h660
        }
    }, match v5.clone() {
        Adt135::C0(h661, _) => {
            h661
        }
    }, match v6.clone() {
        Adt135::C0(h662, _) => {
            h662
        }
    }, f260(match v4 {
        Adt135::C0(_, h663) => {
            h663
        }
    }, f260(match v5 {
        Adt135::C0(_, h664) => {
            h664
        }
    }, match v6 {
        Adt135::C0(_, h665) => {
            h665
        }
    })), f261(v0, v1, v2)?))
}

pub fn f196(v0: Adt111) -> Option<Str> {
    let a667 = match v0.clone() {
        Adt111::C0(_, h666, _) => {
            h666
        }
    };
    let a668 = Str::lit("tools/call");
    if equal(a667, a668) {
        let m670 = match v0 {
            Adt111::C0(_, _, h669) => {
                h669
            }
        };
        match m670 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m671 = f199(Str::lit("name"), v1);
                match m671 {
                    Adt137::C0 => {
                        None::<Str>
                    }
                    Adt137::C1(v2) => {
                        f214(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f197() -> List<Str> {
    List::cons(f262(), List::<Str>::nil())
}

pub fn f198(v0: Adt5) -> R<Adt136> {
    let m672 = f199(Str::lit("id"), v0);
    match m672 {
        Adt137::C0 => {
            Ok(Adt136::C0)
        }
        Adt137::C1(v1) => {
            let m673 = v1;
            match m673 {
                Adt6::C0 => {
                    Ok(Adt136::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt136::C2)
                }
                Adt6::C2(v3) => {
                    let m674 = f263(v3)?;
                    match m674 {
                        None => {
                            Ok(Adt136::C2)
                        }
                        Some(v4) => {
                            Ok(Adt136::C1(Adt99::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt136::C1(Adt99::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt136::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt136::C2)
                }
            }
        }
    }
}

pub fn f199(v0: Str, v1: Adt5) -> Adt137 {
    f213(v0, match v1 {
        Adt5::C0(h675) => {
            h675
        }
    })
}

pub fn f200(v0: Adt5) -> Adt102 {
    if f201(v0) {
        Adt102::C8
    } else {
        Adt102::C3
    }
}

pub fn f201(v0: Adt5) -> bool {
    if f264(Str::lit("result"), v0.clone()) {
        true
    } else {
        f264(Str::lit("error"), v0)
    }
}

pub fn f202(v0: Adt99, v1: Adt5) -> Result<Adt111, Adt101> {
    let m676 = f199(Str::lit("jsonrpc"), v1.clone());
    match m676 {
        Adt137::C0 => {
            f152(Some(v0), Adt102::C2)
        }
        Adt137::C1(v2) => {
            let m677 = f214(v2);
            match m677 {
                None => {
                    f152(Some(v0), Adt102::C2)
                }
                Some(v3) => {
                    let a678 = v3;
                    let a679 = f143();
                    if equal(a678, a679) {
                        f265(v0, v1)
                    } else {
                        f152(Some(v0), Adt102::C2)
                    }
                }
            }
        }
    }
}

pub fn f203(v0: u64, v1: u64) -> Adt120 {
    let a680 = v1;
    let a681 = 0u64;
    if nat_eq(a680, a681) {
        Adt120::C0
    } else {
        let a682 = v1;
        let a683 = 2u64;
        if nat_eq(a682, a683) {
            Adt120::C2(v0)
        } else {
            Adt120::C1({
                let a687 = v0;
                let a688 = {
                    let a684 = v1;
                    let a685 = 3u64;
                    let a686 = 0u64;
                    nat_quot(a684, a685, a686)
                };
                nat_sub(a687, a688)
            })
        }
    }
}

pub fn f204(p716: Adt121, p717: u64, p718: u64, p719: List<u64>) -> R<u64> {
    let m715 = (p716, (p717, (p718, p719)));
    {
        let mut m715 = m715;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m715;
                let m689 = v3;
                match m689.uncons() {
                    None => {
                        let m690 = v0;
                        match m690 {
                            Adt121::C0 => {
                                Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt121::C1 => {
                                Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt121::C2 => {
                                Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt121::C3 => {
                                Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m691 = v0;
                        match m691 {
                            Adt121::C0 => {
                                let a692 = v4;
                                let a693 = 128u64;
                                if nat_lt(a692, a693) {
                                    Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a694 = v4;
                                    let a695 = 194u64;
                                    if nat_lt(a694, a695) {
                                        let v6: u64 = f266({
                                            let a696 = List::cons(v4, v5);
                                            length_list(a696)
                                        })?;
                                        Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a697 = v4;
                                        let a698 = 224u64;
                                        if nat_lt(a697, a698) {
                                            Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a699 = v4;
                                            let a700 = 224u64;
                                            if nat_eq(a699, a700) {
                                                Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a701 = v4;
                                                let a702 = 237u64;
                                                if nat_eq(a701, a702) {
                                                    Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a703 = v4;
                                                    let a704 = 240u64;
                                                    if nat_lt(a703, a704) {
                                                        Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a705 = v4;
                                                        let a706 = 240u64;
                                                        if nat_eq(a705, a706) {
                                                            Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a707 = v4;
                                                            let a708 = 244u64;
                                                            if nat_eq(a707, a708) {
                                                                Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a709 = v4;
                                                                let a710 = 244u64;
                                                                if nat_lt(a709, a710) {
                                                                    Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f266({
                                                                        let a711 = List::cons(v4, v5);
                                                                        length_list(a711)
                                                                    })?;
                                                                    Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
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
                            Adt121::C1 => {
                                if f267(v4, v1, v2) {
                                    Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f266({
                                        let a712 = List::cons(v4, v5);
                                        length_list(a712)
                                    })?;
                                    Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt121::C2 => {
                                if f267(v4, v1, v2) {
                                    Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f266({
                                        let a713 = List::cons(v4, v5);
                                        length_list(a713)
                                    })?;
                                    Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt121::C3 => {
                                if f267(v4, v1, v2) {
                                    Ok::<(Adt121, (u64, (u64, List<u64>))), R<u64>>((Adt121::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f266({
                                        let a714 = List::cons(v4, v5);
                                        length_list(a714)
                                    })?;
                                    Err::<(Adt121, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
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

pub fn f205(v0: Adt122, v1: u64) -> R<Result<Adt6, Adt100>> {
    let m721 = match v0.clone() {
        Adt122::C0(h720, _, _) => {
            h720
        }
    };
    match m721 {
        Adt123::C0 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt123::C1 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt123::C2 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt123::C3 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt123::C4 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt123::C5 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt123::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt123::C7(_, _) => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt123::C8(v3) => {
            let m722 = f268(v3)?;
            match m722 {
                Adt142::C0 => {
                    Ok(Err::<Adt6, Adt100>(Adt100::C7(v1)))
                }
                Adt142::C1 => {
                    Ok(Err::<Adt6, Adt100>(Adt100::C8(v1)))
                }
                Adt142::C2(v4) => {
                    let m727 = {
                        let m726 = f269(v4, match v0.clone() {
                            Adt122::C0(_, h724, _) => {
                                h724
                            }
                        }, match v0 {
                            Adt122::C0(_, _, h725) => {
                                h725
                            }
                        });
                        match m726 {
                            Adt122::C0(h723, _, _) => {
                                h723
                            }
                        }
                    };
                    match m727 {
                        Adt123::C0 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C1 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C2 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C3 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C4 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C5 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C7(_, _) => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C8(_) => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt123::C9(v5) => {
                            Ok(Ok::<Adt6, Adt100>(v5))
                        }
                    }
                }
            }
        }
        Adt123::C9(v2) => {
            Ok(Ok::<Adt6, Adt100>(v2))
        }
    }
}

pub fn f206(v0: Adt122, v1: u64, v2: u64) -> R<Adt138> {
    let m729 = match v0.clone() {
        Adt122::C0(h728, _, _) => {
            h728
        }
    };
    match m729 {
        Adt123::C0 => {
            if f270(v2) {
                Ok(Adt138::C1(v0))
            } else {
                f271(v0, v1, v2)
            }
        }
        Adt123::C1 => {
            if f270(v2) {
                Ok(Adt138::C1(v0))
            } else {
                let a730 = v2;
                let a731 = 93u64;
                if nat_eq(a730, a731) {
                    Ok(f272(v0, v1))
                } else {
                    f271(v0, v1, v2)
                }
            }
        }
        Adt123::C2 => {
            Ok(f273(v0, v1, v2))
        }
        Adt123::C3 => {
            if f270(v2) {
                Ok(Adt138::C1(v0))
            } else {
                let a732 = v2;
                let a733 = 125u64;
                if nat_eq(a732, a733) {
                    Ok(f274(v0, v1))
                } else {
                    let a734 = v2;
                    let a735 = 34u64;
                    if nat_eq(a734, a735) {
                        Ok(f275(v0, Adt123::C6(Adt124::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f276(Adt100::C2(v1)))
                    }
                }
            }
        }
        Adt123::C4 => {
            if f270(v2) {
                Ok(Adt138::C1(v0))
            } else {
                let a736 = v2;
                let a737 = 34u64;
                if nat_eq(a736, a737) {
                    Ok(f275(v0, Adt123::C6(Adt124::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f276(Adt100::C2(v1)))
                }
            }
        }
        Adt123::C5 => {
            if f270(v2) {
                Ok(Adt138::C1(v0))
            } else {
                let a738 = v2;
                let a739 = 58u64;
                if nat_eq(a738, a739) {
                    Ok(f275(v0, Adt123::C0))
                } else {
                    Ok(f276(Adt100::C2(v1)))
                }
            }
        }
        Adt123::C6(v3, v4, v5) => {
            f277(v0, v1, v2, v3, v4, v5)
        }
        Adt123::C7(v6, v7) => {
            f278(v0, v1, v2, v6, v7)
        }
        Adt123::C8(v8) => {
            f279(v0, v1, v2, v8)
        }
        Adt123::C9(_) => {
            Ok(f273(v0, v1, v2))
        }
    }
}

pub fn f207(v0: u64) -> R<u64> {
    let a740 = 64u64;
    let a741 = v0;
    if if nat_lt(a740, a741) {
        let a742 = v0;
        let a743 = 91u64;
        nat_lt(a742, a743)
    } else {
        false
    } {
        let a744 = v0;
        let a745 = 32u64;
        nat_add(a744, a745)
    } else {
        Ok(v0)
    }
}

pub fn f208(p748: List<u64>) -> List<u64> {
    let m747 = p748;
    {
        let mut m747 = m747;
        loop {
            let step = {
                let v0 = m747;
                let m746 = v0;
                match m746.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f280(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
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

pub fn f209(v0: u64) -> bool {
    let a749 = v0;
    let a750 = 9u64;
    if nat_eq(a749, a750) {
        true
    } else {
        let a751 = 31u64;
        let a752 = v0;
        if nat_lt(a751, a752) {
            let a753 = v0;
            let a754 = 127u64;
            nat_lt(a753, a754)
        } else {
            false
        }
    }
}

pub fn f210(p757: List<u64>) -> bool {
    let m756 = p757;
    {
        let mut m756 = m756;
        loop {
            let step = {
                let v0 = m756;
                let m755 = v0;
                match m755.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f281(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: u64) -> bool {
    let a758 = 48u64;
    let a759 = v0;
    if if nat_le(a758, a759) {
        let a760 = v0;
        let a761 = 57u64;
        nat_le(a760, a761)
    } else {
        false
    } {
        true
    } else {
        let a762 = 65u64;
        let a763 = v0;
        if if nat_le(a762, a763) {
            let a764 = v0;
            let a765 = 90u64;
            nat_le(a764, a765)
        } else {
            false
        } {
            true
        } else {
            let a766 = 97u64;
            let a767 = v0;
            if if nat_le(a766, a767) {
                let a768 = v0;
                let a769 = 122u64;
                nat_le(a768, a769)
            } else {
                false
            } {
                true
            } else {
                let a770 = v0;
                let a771 = 33u64;
                if nat_eq(a770, a771) {
                    true
                } else {
                    let a772 = v0;
                    let a773 = 35u64;
                    if nat_eq(a772, a773) {
                        true
                    } else {
                        let a774 = v0;
                        let a775 = 36u64;
                        if nat_eq(a774, a775) {
                            true
                        } else {
                            let a776 = v0;
                            let a777 = 37u64;
                            if nat_eq(a776, a777) {
                                true
                            } else {
                                let a778 = v0;
                                let a779 = 38u64;
                                if nat_eq(a778, a779) {
                                    true
                                } else {
                                    let a780 = v0;
                                    let a781 = 39u64;
                                    if nat_eq(a780, a781) {
                                        true
                                    } else {
                                        let a782 = v0;
                                        let a783 = 42u64;
                                        if nat_eq(a782, a783) {
                                            true
                                        } else {
                                            let a784 = v0;
                                            let a785 = 43u64;
                                            if nat_eq(a784, a785) {
                                                true
                                            } else {
                                                let a786 = v0;
                                                let a787 = 45u64;
                                                if nat_eq(a786, a787) {
                                                    true
                                                } else {
                                                    let a788 = v0;
                                                    let a789 = 46u64;
                                                    if nat_eq(a788, a789) {
                                                        true
                                                    } else {
                                                        let a790 = v0;
                                                        let a791 = 94u64;
                                                        if nat_eq(a790, a791) {
                                                            true
                                                        } else {
                                                            let a792 = v0;
                                                            let a793 = 95u64;
                                                            if nat_eq(a792, a793) {
                                                                true
                                                            } else {
                                                                let a794 = v0;
                                                                let a795 = 96u64;
                                                                if nat_eq(a794, a795) {
                                                                    true
                                                                } else {
                                                                    let a796 = v0;
                                                                    let a797 = 124u64;
                                                                    if nat_eq(a796, a797) {
                                                                        true
                                                                    } else {
                                                                        let a798 = v0;
                                                                        let a799 = 126u64;
                                                                        nat_eq(a798, a799)
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

pub fn f212(v0: u64) -> bool {
    let a800 = 48u64;
    let a801 = v0;
    if if nat_le(a800, a801) {
        let a802 = v0;
        let a803 = 57u64;
        nat_le(a802, a803)
    } else {
        false
    } {
        true
    } else {
        let a804 = 65u64;
        let a805 = v0;
        if if nat_le(a804, a805) {
            let a806 = v0;
            let a807 = 90u64;
            nat_le(a806, a807)
        } else {
            false
        } {
            true
        } else {
            let a808 = 97u64;
            let a809 = v0;
            if if nat_le(a808, a809) {
                let a810 = v0;
                let a811 = 122u64;
                nat_le(a810, a811)
            } else {
                false
            } {
                true
            } else {
                let a812 = v0;
                let a813 = 95u64;
                if nat_eq(a812, a813) {
                    true
                } else {
                    let a814 = v0;
                    let a815 = 45u64;
                    if nat_eq(a814, a815) {
                        true
                    } else {
                        let a816 = v0;
                        let a817 = 46u64;
                        nat_eq(a816, a817)
                    }
                }
            }
        }
    }
}

pub fn f213(v0: Str, p823: List<(Str, Adt6)>) -> Adt137 {
    let m822 = p823;
    {
        let mut m822 = m822;
        loop {
            let step = {
                let v1 = m822;
                let m818 = v1;
                match m818.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt137>(Adt137::C0)
                    }
                    Some((v2, v3)) => {
                        let m820 = f282({
                            let (h819, _) = v2;
                            h819
                        }, v0.clone());
                        match m820 {
                            Adt143::C0 => {
                                Err::<List<(Str, Adt6)>, Adt137>(Adt137::C1({
                                    let (_, h821) = v2;
                                    h821
                                }))
                            }
                            Adt143::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt137>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: Adt6) -> Option<Str> {
    let m824 = v0;
    match m824 {
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

pub fn f215() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f216(v0: Adt6) -> Option<bool> {
    let m825 = v0;
    match m825 {
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

pub fn f217(v0: Adt6) -> Option<Adt5> {
    let m826 = v0;
    match m826 {
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

pub fn f218(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt112, v6: List<Adt105>) -> R<Result<(List<Adt105>, Adt112), Adt96>> {
    let m827 = f283(v0.clone(), v1, v2.clone());
    match m827 {
        None => {
            let m828 = f284(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m828 {
                Adt144::C0(v8) => {
                    Ok(Err::<(List<Adt105>, Adt112), Adt96>(v8))
                }
                Adt144::C1(v9) => {
                    Ok(Ok::<(List<Adt105>, Adt112), Adt96>((f285(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt105>, Adt112), Adt96>(v7))
        }
    }
}

pub fn f219(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> R<Adt128> {
    let m829 = f286(v1.clone(), v2.clone())?;
    match m829 {
        Adt128::C0(v4) => {
            Ok(Adt128::C0(v4))
        }
        Adt128::C1(v5) => {
            Ok(Adt128::C1(v5))
        }
        Adt128::C2(v6) => {
            Ok(Adt128::C2(v6))
        }
        Adt128::C3(_) => {
            f287(v0, v1, v2, v3)
        }
    }
}

pub fn f220(v0: List<Adt6>, v1: Adt6, v2: List<Adt107>, v3: u64, v4: List<Adt109>) -> Adt113 {
    let m830 = v0;
    match m830.uncons() {
        None => {
            let a831 = v3;
            let a832 = 1u64;
            if nat_eq(a831, a832) {
                Adt113::C0(Adt108::C0, v4)
            } else {
                Adt113::C0(Adt108::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt113::C0(Adt108::C0, List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f221(v0: Option<Adt6>, v1: Adt6, v2: List<Adt107>, v3: List<Adt109>) -> Adt113 {
    let m833 = v0;
    match m833 {
        None => {
            Adt113::C0(Adt108::C0, v3)
        }
        Some(v4) => {
            Adt113::C0(Adt108::C0, List::cons(Adt109::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f222(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt128 {
    let m834 = f213(Str::lit("anyOf"), v0);
    match m834 {
        Adt137::C0 => {
            Adt128::C3(List::<Adt109>::nil())
        }
        Adt137::C1(v3) => {
            let m835 = f288(v3);
            match m835 {
                None => {
                    Adt128::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m836 = v4;
                    match m836.uncons() {
                        None => {
                            Adt128::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt128::C3(List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C1(v6, v1, v2), List::<Adt109>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f223(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt128 {
    let m837 = f213(Str::lit("oneOf"), v0);
    match m837 {
        Adt137::C0 => {
            Adt128::C3(List::<Adt109>::nil())
        }
        Adt137::C1(v3) => {
            let m838 = f288(v3);
            match m838 {
                None => {
                    Adt128::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m839 = v4;
                    match m839.uncons() {
                        None => {
                            Adt128::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt128::C3(List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C2(v6, v1, v2, 0u64), List::<Adt109>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f224(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt128 {
    let m840 = f213(Str::lit("not"), v0);
    match m840 {
        Adt137::C0 => {
            Adt128::C3(List::<Adt109>::nil())
        }
        Adt137::C1(v3) => {
            Adt128::C3(List::cons(Adt109::C0(v3, v1, v2.clone()), List::cons(Adt109::C3(v2), List::<Adt109>::nil())))
        }
    }
}

pub fn f225(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt128 {
    let m841 = f213(Str::lit("if"), v0.clone());
    match m841 {
        Adt137::C0 => {
            Adt128::C3(List::<Adt109>::nil())
        }
        Adt137::C1(v3) => {
            Adt128::C3(List::cons(Adt109::C0(v3, v1.clone(), v2.clone()), List::cons(Adt109::C4(f289(Str::lit("then"), v0.clone()), f289(Str::lit("else"), v0), v1, v2), List::<Adt109>::nil())))
        }
    }
}

pub fn f226(v0: List<Adt6>, v1: Adt6, v2: List<Adt107>, v3: List<Adt109>) -> Adt113 {
    let m842 = v0;
    match m842.uncons() {
        None => {
            Adt113::C0(Adt108::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt113::C0(Adt108::C0, List::cons(Adt109::C0(v4, v1.clone(), v2.clone()), List::cons(Adt109::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f227(v0: Adt107) -> R<Str> {
    let m843 = v0;
    match m843 {
        Adt107::C0(v1) => {
            f290(v1)
        }
        Adt107::C1(v2) => {
            let a844 = f64(v2, 0i64)?;
            Ok(format_i64(a844))
        }
    }
}

pub fn f228(v0: Str) -> R<bool> {
    let m846 = f291(f100({
        let a845 = v0;
        utf8_encode(a845)
    })?)?;
    match m846 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f229(v0: Adt37) -> bool {
    let m848 = match v0 {
        Adt37::C0(_, _, h847) => {
            h847
        }
    };
    match m848 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f230(v0: Adt49) -> R<Result<Adt32, Adt89>> {
    let m850 = match v0.clone() {
        Adt49::C0(_, h849) => {
            h849
        }
    };
    match m850.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt89>(Adt32::C2(match v0 {
                Adt49::C0(h851, _) => {
                    h851
                }
            })))
        }
        Some((_, _)) => {
            if f292(match v0.clone() {
                Adt49::C0(_, h852) => {
                    h852
                }
            })? {
                Ok(Ok::<Adt32, Adt89>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt89>(Adt89::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f231(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt89>> {
    let m856 = {
        let m855 = match v1.clone() {
            Adt49::C0(h854, _) => {
                h854
            }
        };
        match m855 {
            Adt37::C0(_, h853, _) => {
                h853
            }
        }
    };
    match m856 {
        None => {
            Ok(Err::<Adt32, Adt89>(Adt89::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m859 = f94(Adt6::C5(match v0.clone() {
                Adt5::C0(h857) => {
                    h857
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h858) => {
                    h858
                }
            }), v2)?;
            match m859 {
                Adt106::C0 => {
                    f230(v1)
                }
                Adt106::C1(_, _) => {
                    Ok(Err::<Adt32, Adt89>(Adt89::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt106::C2(_) => {
                    Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt106::C3(_) => {
                    Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt106::C4 => {
                    Ok(Err::<Adt32, Adt89>(Adt89::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f232(v0: Bytes, p865: u64, p866: u64, p867: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt94>> {
    let m864 = (p865, (p866, p867));
    {
        let mut m864 = m864;
        loop {
            let step = {
                let (v1, (v2, v3)) = m864;
                let m860 = v2;
                if m860 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Ok::<(List<List<u64>>, u64), Adt94>((f51(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m860 - 1;
                    let m861 = f26(v0.clone(), v1)?;
                    match m861 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Err::<(List<List<u64>>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>((match v5.clone() {
                                Adt97::C0(_, h862) => {
                                    h862
                                }
                            }, (v4, List::cons(f100(match v5 {
                                Adt97::C0(h863, _) => {
                                    h863
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233(v0: Adt92, v1: List<List<u64>>) -> Adt92 {
    Adt92::C0(match v0.clone() {
        Adt92::C0(h868, _, _, _, _, _, _, _) => {
            h868
        }
    }, match v0.clone() {
        Adt92::C0(_, h869, _, _, _, _, _, _) => {
            h869
        }
    }, match v0.clone() {
        Adt92::C0(_, _, h870, _, _, _, _, _) => {
            h870
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, h871, _, _, _, _) => {
            h871
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, _, h872, _, _, _) => {
            h872
        }
    }, v1, match v0.clone() {
        Adt92::C0(_, _, _, _, _, _, h873, _) => {
            h873
        }
    }, match v0 {
        Adt92::C0(_, _, _, _, _, _, _, h874) => {
            h874
        }
    })
}

pub fn f234(v0: Bytes, v1: List<u64>) -> Bytes {
    let a875 = v0;
    let a876 = f293(f102(), f294(List::<Bytes>::nil(), v1));
    append_bytes(a875, a876)
}

pub fn f235(v0: i64, v1: Str, v2: Str, v3: Adt97) -> Result<(Adt91, u64), Adt94> {
    Ok::<(Adt91, u64), Adt94>((Adt91::C0(v0, v1, v2, match v3.clone() {
        Adt97::C0(h877, _) => {
            h877
        }
    }), match v3 {
        Adt97::C0(_, h878) => {
            h878
        }
    }))
}

pub fn f236(v0: Adt114, v1: List<Adt114>, v2: List<u64>) -> R<Adt139> {
    let m879 = v0;
    match m879 {
        Adt114::C0(v3) => {
            Ok(Adt139::C1(v1, f186(v3, v2)))
        }
        Adt114::C1(v4, v5) => {
            f295(v4, v5, v1, v2)
        }
        Adt114::C2(v6, v7) => {
            let m880 = v7;
            match m880.uncons() {
                None => {
                    Ok(Adt139::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt139::C1(List::cons(Adt114::C1(v6, v8), List::cons(Adt114::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt114::C3(v10, v11) => {
            let m881 = v11.clone();
            match m881.uncons() {
                None => {
                    Ok(Adt139::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt139::C1(List::cons(Adt114::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt114::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt114::C4(v14, v15) => {
            let m882 = v15;
            match m882.uncons() {
                None => {
                    Ok(Adt139::C1(List::cons(Adt114::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt139::C1(List::cons(Adt114::C0({
                        let a884 = f296({
                            let (h883, _) = v16;
                            h883
                        })?;
                        let a885 = List::cons(58u64, List::<u64>::nil());
                        append_list(a884, a885)
                    }), List::cons(Adt114::C1(v14, {
                        let (_, h886) = v16;
                        h886
                    }), List::cons(Adt114::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt114::C5(v18, v19) => {
            let m887 = v19.clone();
            match m887.uncons() {
                None => {
                    Ok(Adt139::C1(List::cons(Adt114::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt139::C1(List::cons(Adt114::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt114::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f237(v0: Adt130) -> R<Option<Adt5>> {
    let m889 = f297(match v0.clone() {
        Adt130::C0(h888, _, _) => {
            h888
        }
    })?;
    match m889 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a892 = v1;
                let a893 = {
                    let m891 = f298(v0);
                    match m891 {
                        Adt5::C0(h890) => {
                            h890
                        }
                    }
                };
                append_list(a892, a893)
            })))
        }
    }
}

pub fn f238(v0: Adt91, v1: Option<Adt87>, v2: Adt95) -> R<Adt95> {
    let m894 = v1;
    match m894 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a896 = match v2.clone() {
                Adt95::C0(h895, _, _, _, _) => {
                    h895
                }
            };
            let a897 = f241();
            if nat_eq(a896, a897) {
                Ok(f299(f300(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f239(v0: Adt32) -> Option<Adt87> {
    let m898 = v0;
    match m898 {
        Adt32::C0(_) => {
            None::<Adt87>
        }
        Adt32::C1(_) => {
            None::<Adt87>
        }
        Adt32::C2(_) => {
            None::<Adt87>
        }
        Adt32::C3(_) => {
            None::<Adt87>
        }
        Adt32::C4(_) => {
            None::<Adt87>
        }
        Adt32::C5(_) => {
            None::<Adt87>
        }
        Adt32::C6(_) => {
            None::<Adt87>
        }
        Adt32::C7(_) => {
            None::<Adt87>
        }
        Adt32::C8(_) => {
            None::<Adt87>
        }
        Adt32::C9(_) => {
            None::<Adt87>
        }
        Adt32::C10(_) => {
            None::<Adt87>
        }
        Adt32::C11 => {
            None::<Adt87>
        }
        Adt32::C12(_) => {
            None::<Adt87>
        }
        Adt32::C13(v13) => {
            Some(Adt87::C0(match v13 {
                Adt82::C0(_, _, h899) => {
                    h899
                }
            }, Adt88::C0, false))
        }
        Adt32::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f240(v0: bool, v1: Adt95, v2: Option<List<List<u64>>>) -> Adt95 {
    let m900 = v2;
    match m900 {
        None => {
            f16()
        }
        Some(v3) => {
            f301(v0, v1, f51(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f241() -> u64 {
    200u64
}

pub fn f242(v0: Adt99, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f143())), List::cons((Str::lit("id"), f187(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h901) => {
            h901
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f243(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m902 = v0;
    match m902 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m903 = v1;
            match m903 {
                None => {
                    None::<List<List<u64>>>
                }
                Some(v3) => {
                    Some(f51(v3, v2))
                }
            }
        }
    }
}

pub fn f244(p907: List<Adt132>, p908: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m906 = (p907, p908);
    {
        let mut m906 = m906;
        loop {
            let step = {
                let (v0, v1) = m906;
                let m904 = v0;
                match m904.uncons() {
                    None => {
                        Err::<(List<Adt132>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m905 = f302(v2)?;
                        match m905 {
                            None => {
                                Err::<(List<Adt132>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt132>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: Option<Adt87>) -> R<Option<List<List<u64>>>> {
    let m909 = v0;
    match m909 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f303(v1)
        }
    }
}

pub fn f246(v0: Adt133) -> u64 {
    let m910 = v0;
    match m910 {
        Adt133::C0(_) => {
            f111()
        }
        Adt133::C1(_, _) => {
            f111()
        }
        Adt133::C2(_) => {
            f111()
        }
        Adt133::C3(_) => {
            f304()
        }
        Adt133::C4(v6) => {
            f305(v6)
        }
    }
}

pub fn f247(v0: Adt133) -> R<Adt80> {
    Ok(Adt80::C0(f306(v0.clone()), f307(v0.clone()), f308(v0)?))
}

pub fn f248() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f249(p913: List<List<u64>>) -> R<bool> {
    let m912 = p913;
    {
        let mut m912 = m912;
        loop {
            let step = {
                let v0 = m912;
                let m911 = v0;
                match m911.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f309(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: u64, p918: List<u64>, p919: List<u64>, p920: List<List<u64>>) -> List<List<u64>> {
    let m917 = (p918, (p919, p920));
    {
        let mut m917 = m917;
        loop {
            let step = {
                let (v1, (v2, v3)) = m917;
                let m914 = v1;
                match m914.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f51(List::cons(f166(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a915 = v4;
                        let a916 = v0;
                        if nat_eq(a915, a916) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f166(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt140, Adt133>) -> R<Result<Adt130, Adt133>> {
    let m921 = v3;
    match m921 {
        Ok(v5) => {
            f310(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt130, Adt133>(v4))
        }
    }
}

pub fn f252(v0: Option<Adt115>, v1: List<Str>, v2: Adt111) -> R<Result<Adt140, Adt133>> {
    Ok(f311(f312(match v2.clone() {
        Adt111::C0(_, h922, _) => {
            h922
        }
    }), f313(v0, v1, v2)?))
}

pub fn f253(v0: Adt117, v1: Adt103) -> Adt135 {
    let m923 = v1;
    match m923 {
        Adt103::C0 => {
            f259()
        }
        Adt103::C1 => {
            Adt135::C0(None::<Str>, Some(Adt116::C6(v0)))
        }
        Adt103::C2 => {
            Adt135::C0(None::<Str>, Some(Adt116::C7(v0)))
        }
        Adt103::C3(v2) => {
            let m924 = f183(v2);
            match m924 {
                None => {
                    Adt135::C0(None::<Str>, Some(Adt116::C6(v0)))
                }
                Some(v3) => {
                    Adt135::C0(Some(v3), None::<Adt116>)
                }
            }
        }
    }
}

pub fn f254() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f255() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f256(v0: Str) -> bool {
    let m925 = f314(v0);
    match m925 {
        Adt149::C0 => {
            false
        }
        Adt149::C1 => {
            true
        }
        Adt149::C2 => {
            true
        }
        Adt149::C3 => {
            true
        }
    }
}

pub fn f257(v0: Adt103) -> R<Adt103> {
    let m926 = v0.clone();
    match m926 {
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
            let m927 = f315(v1);
            match m927 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m928 = f291(v2)?;
                    match m928 {
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

pub fn f258() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f259() -> Adt135 {
    Adt135::C0(None::<Str>, None::<Adt116>)
}

pub fn f260(v0: Option<Adt116>, v1: Option<Adt116>) -> Option<Adt116> {
    let m929 = v0.clone();
    match m929 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f261(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt118>> {
    Ok(f316(f317(v0, v1, v2, List::<Adt118>::nil())?, List::<Adt118>::nil()))
}

pub fn f262() -> Str {
    Str::lit("2026-07-28")
}

pub fn f263(v0: Adt7) -> R<Option<i64>> {
    let m932 = f318(40u64, match v0.clone() {
        Adt7::C0(h930, _) => {
            h930
        }
    }, match v0 {
        Adt7::C0(_, h931) => {
            h931
        }
    })?;
    match m932 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f319(v1))
        }
    }
}

pub fn f264(v0: Str, v1: Adt5) -> bool {
    f320(f199(v0, v1))
}

pub fn f265(v0: Adt99, v1: Adt5) -> Result<Adt111, Adt101> {
    let m933 = f199(Str::lit("method"), v1.clone());
    match m933 {
        Adt137::C0 => {
            f152(Some(v0), f200(v1))
        }
        Adt137::C1(v2) => {
            let m934 = f214(v2);
            match m934 {
                None => {
                    f152(Some(v0), Adt102::C4)
                }
                Some(v3) => {
                    f321(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f266(v0: u64) -> R<u64> {
    let a937 = {
        let a935 = v0;
        let a936 = 3u64;
        nat_mul(a935, a936)?
    };
    let a938 = 1u64;
    nat_add(a937, a938)
}

pub fn f267(v0: u64, v1: u64, v2: u64) -> bool {
    let a939 = v1;
    let a940 = v0;
    if nat_le(a939, a940) {
        let a941 = v0;
        let a942 = v2;
        nat_le(a941, a942)
    } else {
        false
    }
}

pub fn f268(v0: Adt125) -> R<Adt142> {
    if f322(match v0.clone() {
        Adt125::C0(_, h943, _, _, _, _, _) => {
            h943
        }
    }) {
        let v1: i64 = {
            let a949 = if match v0.clone() {
                Adt125::C0(_, _, _, _, _, h944, _) => {
                    h944
                }
            } {
                let a946 = match v0.clone() {
                    Adt125::C0(_, _, _, _, _, _, h945) => {
                        h945
                    }
                };
                int_neg(a946)?
            } else {
                match v0.clone() {
                    Adt125::C0(_, _, _, _, _, _, h947) => {
                        h947
                    }
                }
            };
            let a950 = match v0.clone() {
                Adt125::C0(_, _, _, _, h948, _, _) => {
                    h948
                }
            };
            int_sub(a949, a950)?
        };
        let m953 = {
            let a951 = v1;
            let a952 = -9999i64;
            compare(a951, a952)
        };
        if if match m953 {
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
            let m956 = {
                let a954 = 9999i64;
                let a955 = v1;
                compare(a954, a955)
            };
            match m956 {
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
            Ok(Adt142::C1)
        } else {
            Ok(Adt142::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt125::C0(h957, _, _, _, _, _, _) => {
                    h957
                }
            } {
                let a959 = match v0 {
                    Adt125::C0(_, _, h958, _, _, _, _) => {
                        h958
                    }
                };
                int_neg(a959)?
            } else {
                match v0 {
                    Adt125::C0(_, _, h960, _, _, _, _) => {
                        h960
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt142::C0)
    }
}

pub fn f269(v0: Adt6, v1: List<Adt127>, v2: u64) -> Adt122 {
    let m961 = v1.clone();
    match m961.uncons() {
        None => {
            Adt122::C0(Adt123::C9(v0), List::<Adt127>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m962 = v3;
            match m962 {
                Adt127::C0(v5) => {
                    Adt122::C0(Adt123::C2, List::cons(Adt127::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt127::C1(v6, v7) => {
                    let m963 = v7;
                    match m963 {
                        None => {
                            Adt122::C0(Adt123::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt122::C0(Adt123::C2, List::cons(Adt127::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f270(v0: u64) -> bool {
    let a964 = v0;
    let a965 = 32u64;
    if nat_eq(a964, a965) {
        true
    } else {
        let a966 = v0;
        let a967 = 10u64;
        if nat_eq(a966, a967) {
            true
        } else {
            let a968 = v0;
            let a969 = 13u64;
            if nat_eq(a968, a969) {
                true
            } else {
                let a970 = v0;
                let a971 = 9u64;
                nat_eq(a970, a971)
            }
        }
    }
}

pub fn f271(v0: Adt122, v1: u64, v2: u64) -> R<Adt138> {
    let a972 = v2;
    let a973 = 34u64;
    if nat_eq(a972, a973) {
        Ok(f275(v0, Adt123::C6(Adt124::C0, List::<u64>::nil(), false)))
    } else {
        let a974 = v2;
        let a975 = 123u64;
        if nat_eq(a974, a975) {
            f323(v0, Adt127::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt123::C3, v1)
        } else {
            let a976 = v2;
            let a977 = 91u64;
            if nat_eq(a976, a977) {
                f323(v0, Adt127::C0(List::<Adt6>::nil()), Adt123::C1, v1)
            } else {
                let a978 = v2;
                let a979 = 116u64;
                if nat_eq(a978, a979) {
                    Ok(f275(v0, Adt123::C7(0u64, 1u64)))
                } else {
                    let a980 = v2;
                    let a981 = 102u64;
                    if nat_eq(a980, a981) {
                        Ok(f275(v0, Adt123::C7(1u64, 1u64)))
                    } else {
                        let a982 = v2;
                        let a983 = 110u64;
                        if nat_eq(a982, a983) {
                            Ok(f275(v0, Adt123::C7(2u64, 1u64)))
                        } else {
                            let a984 = v2;
                            let a985 = 45u64;
                            if nat_eq(a984, a985) {
                                Ok(f275(v0, f324(true, Adt126::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a986 = v2;
                                let a987 = 48u64;
                                if nat_eq(a986, a987) {
                                    Ok(f275(v0, f324(false, Adt126::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f267(v2, 49u64, 57u64) {
                                        Ok(f275(v0, f324(false, Adt126::C2, f325({
                                            let a988 = v2;
                                            let a989 = 48u64;
                                            nat_sub(a988, a989)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f276(Adt100::C2(v1)))
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

pub fn f272(v0: Adt122, v1: u64) -> Adt138 {
    let m991 = match v0.clone() {
        Adt122::C0(_, h990, _) => {
            h990
        }
    };
    match m991.uncons() {
        None => {
            f276(Adt100::C2(v1))
        }
        Some((v2, v3)) => {
            let m992 = v2;
            match m992 {
                Adt127::C0(v4) => {
                    Adt138::C1(f269(Adt6::C4(f326(v4, List::<Adt6>::nil())), v3, {
                        let a994 = match v0 {
                            Adt122::C0(_, _, h993) => {
                                h993
                            }
                        };
                        let a995 = 1u64;
                        nat_sub(a994, a995)
                    }))
                }
                Adt127::C1(_, _) => {
                    f276(Adt100::C2(v1))
                }
            }
        }
    }
}

pub fn f273(v0: Adt122, v1: u64, v2: u64) -> Adt138 {
    let m997 = match v0.clone() {
        Adt122::C0(h996, _, _) => {
            h996
        }
    };
    match m997 {
        Adt123::C0 => {
            f276(Adt100::C2(v1))
        }
        Adt123::C1 => {
            f276(Adt100::C2(v1))
        }
        Adt123::C2 => {
            if f270(v2) {
                Adt138::C1(v0)
            } else {
                let a998 = v2;
                let a999 = 44u64;
                if nat_eq(a998, a999) {
                    f327(v0, v1)
                } else {
                    let a1000 = v2;
                    let a1001 = 93u64;
                    if nat_eq(a1000, a1001) {
                        f272(v0, v1)
                    } else {
                        let a1002 = v2;
                        let a1003 = 125u64;
                        if nat_eq(a1002, a1003) {
                            f274(v0, v1)
                        } else {
                            f276(Adt100::C2(v1))
                        }
                    }
                }
            }
        }
        Adt123::C3 => {
            f276(Adt100::C2(v1))
        }
        Adt123::C4 => {
            f276(Adt100::C2(v1))
        }
        Adt123::C5 => {
            f276(Adt100::C2(v1))
        }
        Adt123::C6(_, _, _) => {
            f276(Adt100::C2(v1))
        }
        Adt123::C7(_, _) => {
            f276(Adt100::C2(v1))
        }
        Adt123::C8(_) => {
            f276(Adt100::C2(v1))
        }
        Adt123::C9(_) => {
            if f270(v2) {
                Adt138::C1(v0)
            } else {
                f276(Adt100::C10(v1))
            }
        }
    }
}

pub fn f274(v0: Adt122, v1: u64) -> Adt138 {
    let m1005 = match v0.clone() {
        Adt122::C0(_, h1004, _) => {
            h1004
        }
    };
    match m1005.uncons() {
        None => {
            f276(Adt100::C2(v1))
        }
        Some((v2, v3)) => {
            let m1006 = v2;
            match m1006 {
                Adt127::C0(_) => {
                    f276(Adt100::C2(v1))
                }
                Adt127::C1(v5, _) => {
                    Adt138::C1(f269(Adt6::C5(f328(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1008 = match v0 {
                            Adt122::C0(_, _, h1007) => {
                                h1007
                            }
                        };
                        let a1009 = 1u64;
                        nat_sub(a1008, a1009)
                    }))
                }
            }
        }
    }
}

pub fn f275(v0: Adt122, v1: Adt123) -> Adt138 {
    Adt138::C1(f329(v0, v1))
}

pub fn f276(v0: Adt100) -> Adt138 {
    Adt138::C0(v0)
}

pub fn f277(v0: Adt122, v1: u64, v2: u64, v3: Adt124, v4: List<u64>, v5: bool) -> R<Adt138> {
    let m1010 = v3;
    match m1010 {
        Adt124::C0 => {
            let a1011 = v2;
            let a1012 = 34u64;
            if nat_eq(a1011, a1012) {
                Ok(f330(v0, v1, v4, v5))
            } else {
                let a1013 = v2;
                let a1014 = 92u64;
                if nat_eq(a1013, a1014) {
                    Ok(f275(v0, Adt123::C6(Adt124::C1, v4, v5)))
                } else {
                    let a1015 = v2;
                    let a1016 = 32u64;
                    if nat_lt(a1015, a1016) {
                        Ok(f276(Adt100::C4(v1)))
                    } else {
                        Ok(f275(v0, Adt123::C6(Adt124::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt124::C1 => {
            let a1017 = v2;
            let a1018 = 117u64;
            if nat_eq(a1017, a1018) {
                Ok(f275(v0, Adt123::C6(Adt124::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1019 = f331(v2);
                let a1020 = 0u64;
                if nat_eq(a1019, a1020) {
                    Ok(f276(Adt100::C5(v1)))
                } else {
                    Ok(f275(v0, Adt123::C6(Adt124::C0, List::cons(f331(v2), v4), v5)))
                }
            }
        }
        Adt124::C2(v6, v7, v8) => {
            let a1021 = 15u64;
            let a1022 = f332(v2);
            if nat_lt(a1021, a1022) {
                Ok(f276(Adt100::C5(v1)))
            } else {
                let a1025 = {
                    let a1023 = v6;
                    let a1024 = 1u64;
                    nat_add(a1023, a1024)?
                };
                let a1026 = 4u64;
                if nat_eq(a1025, a1026) {
                    f333(v0, v1, {
                        let a1029 = {
                            let a1027 = v7;
                            let a1028 = 16u64;
                            nat_mul(a1027, a1028)?
                        };
                        let a1030 = f332(v2);
                        nat_add(a1029, a1030)?
                    }, v8, v4, v5)
                } else {
                    Ok(f275(v0, Adt123::C6(Adt124::C2({
                        let a1031 = v6;
                        let a1032 = 1u64;
                        nat_add(a1031, a1032)?
                    }, {
                        let a1035 = {
                            let a1033 = v7;
                            let a1034 = 16u64;
                            nat_mul(a1033, a1034)?
                        };
                        let a1036 = f332(v2);
                        nat_add(a1035, a1036)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt124::C3(v9) => {
            let a1037 = v2;
            let a1038 = 92u64;
            if nat_eq(a1037, a1038) {
                Ok(f275(v0, Adt123::C6(Adt124::C4(v9), v4, v5)))
            } else {
                Ok(f276(Adt100::C6(v1)))
            }
        }
        Adt124::C4(v10) => {
            let a1039 = v2;
            let a1040 = 117u64;
            if nat_eq(a1039, a1040) {
                Ok(f275(v0, Adt123::C6(Adt124::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f276(Adt100::C6(v1)))
            }
        }
    }
}

pub fn f278(v0: Adt122, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt138> {
    let a1041 = v2;
    let a1042 = f334(v3, v4);
    if nat_eq(a1041, a1042) {
        let a1046 = {
            let a1043 = v4;
            let a1044 = 1u64;
            nat_add(a1043, a1044)?
        };
        let a1047 = {
            let a1045 = f335(v3);
            length_list(a1045)
        };
        if nat_eq(a1046, a1047) {
            Ok(f336(v0, f337(v3)))
        } else {
            Ok(f275(v0, Adt123::C7(v3, {
                let a1048 = v4;
                let a1049 = 1u64;
                nat_add(a1048, a1049)?
            })))
        }
    } else {
        Ok(f276(Adt100::C2(v1)))
    }
}

pub fn f279(v0: Adt122, v1: u64, v2: u64, v3: Adt125) -> R<Adt138> {
    let m1051 = match v3.clone() {
        Adt125::C0(_, h1050, _, _, _, _, _) => {
            h1050
        }
    };
    match m1051 {
        Adt126::C0 => {
            let a1052 = v2;
            let a1053 = 48u64;
            if nat_eq(a1052, a1053) {
                Ok(f338(v0, v3, Adt126::C1))
            } else {
                if f267(v2, 49u64, 57u64) {
                    f339(v0, v1, v2, v3, false)
                } else {
                    Ok(f276(Adt100::C7(v1)))
                }
            }
        }
        Adt126::C1 => {
            let a1054 = v2;
            let a1055 = 46u64;
            if nat_eq(a1054, a1055) {
                Ok(f338(v0, v3, Adt126::C3))
            } else {
                if f340(v2) {
                    Ok(f338(v0, v3, Adt126::C5))
                } else {
                    if f341(v2) {
                        Ok(f276(Adt100::C7(v1)))
                    } else {
                        f342(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt126::C2 => {
            if f341(v2) {
                f339(v0, v1, v2, v3, false)
            } else {
                let a1056 = v2;
                let a1057 = 46u64;
                if nat_eq(a1056, a1057) {
                    Ok(f338(v0, v3, Adt126::C3))
                } else {
                    if f340(v2) {
                        Ok(f338(v0, v3, Adt126::C5))
                    } else {
                        f342(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt126::C3 => {
            if f341(v2) {
                f339(v0, v1, v2, v3, true)
            } else {
                Ok(f276(Adt100::C7(v1)))
            }
        }
        Adt126::C4 => {
            if f341(v2) {
                f339(v0, v1, v2, v3, true)
            } else {
                if f340(v2) {
                    Ok(f338(v0, v3, Adt126::C5))
                } else {
                    f342(v0, v1, v2, v3)
                }
            }
        }
        Adt126::C5 => {
            let a1058 = v2;
            let a1059 = 43u64;
            if nat_eq(a1058, a1059) {
                Ok(f275(v0, f324(match v3.clone() {
                    Adt125::C0(h1060, _, _, _, _, _, _) => {
                        h1060
                    }
                }, Adt126::C6, match v3.clone() {
                    Adt125::C0(_, _, h1061, _, _, _, _) => {
                        h1061
                    }
                }, match v3.clone() {
                    Adt125::C0(_, _, _, h1062, _, _, _) => {
                        h1062
                    }
                }, match v3 {
                    Adt125::C0(_, _, _, _, h1063, _, _) => {
                        h1063
                    }
                }, false, 0i64)))
            } else {
                let a1064 = v2;
                let a1065 = 45u64;
                if nat_eq(a1064, a1065) {
                    Ok(f275(v0, f324(match v3.clone() {
                        Adt125::C0(h1066, _, _, _, _, _, _) => {
                            h1066
                        }
                    }, Adt126::C6, match v3.clone() {
                        Adt125::C0(_, _, h1067, _, _, _, _) => {
                            h1067
                        }
                    }, match v3.clone() {
                        Adt125::C0(_, _, _, h1068, _, _, _) => {
                            h1068
                        }
                    }, match v3 {
                        Adt125::C0(_, _, _, _, h1069, _, _) => {
                            h1069
                        }
                    }, true, 0i64)))
                } else {
                    if f341(v2) {
                        f343(v0, v1, v2, Adt125::C0(match v3.clone() {
                            Adt125::C0(h1070, _, _, _, _, _, _) => {
                                h1070
                            }
                        }, match v3.clone() {
                            Adt125::C0(_, h1071, _, _, _, _, _) => {
                                h1071
                            }
                        }, match v3.clone() {
                            Adt125::C0(_, _, h1072, _, _, _, _) => {
                                h1072
                            }
                        }, match v3.clone() {
                            Adt125::C0(_, _, _, h1073, _, _, _) => {
                                h1073
                            }
                        }, match v3 {
                            Adt125::C0(_, _, _, _, h1074, _, _) => {
                                h1074
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f276(Adt100::C7(v1)))
                    }
                }
            }
        }
        Adt126::C6 => {
            if f341(v2) {
                f343(v0, v1, v2, v3.clone(), match v3 {
                    Adt125::C0(_, _, _, _, _, h1075, _) => {
                        h1075
                    }
                })
            } else {
                Ok(f276(Adt100::C7(v1)))
            }
        }
        Adt126::C7 => {
            if f341(v2) {
                f343(v0, v1, v2, v3.clone(), match v3 {
                    Adt125::C0(_, _, _, _, _, h1076, _) => {
                        h1076
                    }
                })
            } else {
                f342(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f280(v0: u64) -> bool {
    let a1077 = v0;
    let a1078 = 9u64;
    if nat_eq(a1077, a1078) {
        true
    } else {
        let a1079 = v0;
        let a1080 = 32u64;
        nat_eq(a1079, a1080)
    }
}

pub fn f281(v0: u64) -> bool {
    let a1081 = 47u64;
    let a1082 = v0;
    if nat_lt(a1081, a1082) {
        let a1083 = v0;
        let a1084 = 58u64;
        nat_lt(a1083, a1084)
    } else {
        false
    }
}

pub fn f282(v0: Str, v1: Str) -> Adt143 {
    let a1085 = v0;
    let a1086 = v1;
    if equal(a1085, a1086) {
        Adt143::C0
    } else {
        Adt143::C1
    }
}

pub fn f283(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt96> {
    let m1087 = f344(f345(), v2.clone());
    match m1087 {
        None => {
            let m1088 = f346(f347(), v2.clone());
            match m1088 {
                None => {
                    f348(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt96::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt96::C4(v0, v3))
        }
    }
}

pub fn f284(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt112) -> R<Adt144> {
    let m1089 = f213(Str::lit("x-mcp-header"), v1.clone());
    match m1089 {
        Adt137::C0 => {
            Ok(Adt144::C1(v4))
        }
        Adt137::C1(v5) => {
            f349(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f285(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt105>) -> List<Adt105> {
    f350(Str::lit("properties"), v0.clone(), v1, v2, f350(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f351(Str::lit("items"), v0.clone(), f351(Str::lit("additionalProperties"), v0.clone(), f352(Str::lit("allOf"), v0.clone(), f352(Str::lit("anyOf"), v0.clone(), f352(Str::lit("oneOf"), v0.clone(), f351(Str::lit("not"), v0.clone(), f351(Str::lit("if"), v0.clone(), f351(Str::lit("then"), v0.clone(), f351(Str::lit("else"), v0, v3)))))))))))
}

pub fn f286(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt128> {
    Ok(f353(List::cons((Str::lit("type"), f354(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f355(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f356(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f357(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f357(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f357(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f357(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1090 = f358(v0.clone(), v1.clone())?;
        let a1091 = List::cons((Str::lit("required"), f359(v0, v1)), List::<(Str, Adt145)>::nil());
        append_list(a1090, a1091)
    })))))))))
}

pub fn f287(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> R<Adt128> {
    Ok(f360(f361(v0, v1.clone(), v2.clone(), v3.clone()), f360(f362(v1.clone(), v2.clone(), v3.clone()), f360(f363(v1.clone(), v2.clone(), v3.clone()), f360(f364(v1.clone(), v2.clone(), v3.clone())?, f360(f365(v1.clone(), v2.clone(), v3.clone()), f360(f366(Adt110::C0, f222(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f360(f366(Adt110::C1, f223(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f360(f366(Adt110::C2, f224(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f366(Adt110::C3, f225(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f288(v0: Adt6) -> Option<List<Adt6>> {
    let m1092 = v0;
    match m1092 {
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

pub fn f289(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1093 = f213(v0, v1);
    match m1093 {
        Adt137::C0 => {
            None::<Adt6>
        }
        Adt137::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f290(v0: Str) -> R<Str> {
    let m1096 = {
        let a1095 = f234(f102(), f186(f367(f100({
            let a1094 = v0.clone();
            utf8_encode(a1094)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1095)
    };
    match m1096 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f291(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1098 = f368({
        let a1097 = v0.clone();
        length_list(a1097)
    }, v0, List::<u64>::nil())?;
    match m1098 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f186(v1, List::<u64>::nil())))
        }
    }
}

pub fn f292(v0: List<Adt50>) -> R<bool> {
    let m1099 = v0;
    match m1099.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f369(v2, match v1 {
                Adt50::C0(h1100, _, _) => {
                    h1100
                }
            })
        }
    }
}

pub fn f293(p1105: Bytes, p1106: List<Bytes>) -> Bytes {
    let m1104 = (p1105, p1106);
    {
        let mut m1104 = m1104;
        loop {
            let step = {
                let (v0, v1) = m1104;
                let m1101 = v1;
                match m1101.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1102 = v2;
                            let a1103 = v0;
                            append_bytes(a1102, a1103)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(p1109: List<Bytes>, p1110: List<u64>) -> List<Bytes> {
    let m1108 = (p1109, p1110);
    {
        let mut m1108 = m1108;
        loop {
            let step = {
                let (v0, v1) = m1108;
                let m1107 = v1;
                match m1107.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f370(f371(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: u64, v1: Adt6, v2: List<Adt114>, v3: List<u64>) -> R<Adt139> {
    let m1111 = v1;
    match m1111 {
        Adt6::C0 => {
            Ok(Adt139::C1(v2, f186(f372(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt139::C1(v2, f186(if v4 {
                f372(0u64)
            } else {
                f372(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f373(v5.clone())? {
                Ok(Adt139::C1(v2, f186(f374(v5)?, v3)))
            } else {
                Ok(Adt139::C0(Adt100::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt139::C1(v2, f186(f296(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1112 = v0;
            let a1113 = f375();
            if nat_lt(a1112, a1113) {
                Ok(Adt139::C1(f376(91u64, Adt114::C2({
                    let a1114 = v0;
                    let a1115 = 1u64;
                    nat_add(a1114, a1115)?
                }, v7), v2), v3))
            } else {
                Ok(Adt139::C0(Adt100::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1116 = v0;
            let a1117 = f375();
            if nat_lt(a1116, a1117) {
                Ok(Adt139::C1(f376(123u64, Adt114::C4({
                    let a1118 = v0;
                    let a1119 = 1u64;
                    nat_add(a1118, a1119)?
                }, v8), v2), v3))
            } else {
                Ok(Adt139::C0(Adt100::C12))
            }
        }
    }
}

pub fn f296(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1121 = f186(f377(f100({
            let a1120 = v0;
            utf8_encode(a1120)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1122 = List::cons(34u64, List::<u64>::nil());
        append_list(a1121, a1122)
    }))
}

pub fn f297(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m1123 = v0;
    match m1123 {
        Adt32::C0(v1) => {
            Ok(Some(f378(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f379(v2)?))
        }
        Adt32::C2(v3) => {
            f380(f381(v3)?)
        }
        Adt32::C3(v4) => {
            f380(f381(match v4 {
                Adt49::C0(h1124, _) => {
                    h1124
                }
            })?)
        }
        Adt32::C4(v5) => {
            Ok(Some(f382(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f383(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f384(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f385(v8)?))
        }
        Adt32::C8(v9) => {
            f380(f386(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f387(v10)?))
        }
        Adt32::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt32::C12(v12) => {
            f380(f388(v12)?)
        }
        Adt32::C13(v13) => {
            f380(f389({
                let m1127 = match v13 {
                    Adt82::C0(_, _, h1126) => {
                        h1126
                    }
                };
                match m1127 {
                    Adt84::C0(h1125, _, _) => {
                        h1125
                    }
                }
            }))
        }
        Adt32::C14(v14) => {
            Ok(f390(v14))
        }
    }
}

pub fn f298(v0: Adt130) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f391(f392(match v0.clone() {
        Adt130::C0(h1128, _, _) => {
            h1128
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f393({
        let m1131 = match v0 {
            Adt130::C0(_, h1130, _) => {
                h1130
            }
        };
        match m1131 {
            Adt131::C0(h1129, _) => {
                h1129
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f299(v0: Option<Option<List<u64>>>, v1: Adt95) -> Adt95 {
    let m1132 = v0;
    match m1132 {
        None => {
            f16()
        }
        Some(v2) => {
            Adt95::C0(match v1.clone() {
                Adt95::C0(h1133, _, _, _, _) => {
                    h1133
                }
            }, match v1.clone() {
                Adt95::C0(_, h1134, _, _, _) => {
                    h1134
                }
            }, match v1.clone() {
                Adt95::C0(_, _, h1135, _, _) => {
                    h1135
                }
            }, match v1 {
                Adt95::C0(_, _, _, h1136, _) => {
                    h1136
                }
            }, v2)
        }
    }
}

pub fn f300(v0: Adt91, v1: Adt87) -> R<Option<Option<List<u64>>>> {
    let m1138 = f394(match v0.clone() {
        Adt91::C0(_, _, _, h1137) => {
            h1137
        }
    })?;
    match m1138 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1141 = f395(f396(match v1 {
                Adt87::C0(h1139, _, _) => {
                    h1139
                }
            }, v2, match v0 {
                Adt91::C0(h1140, _, _, _) => {
                    h1140
                }
            })?)?;
            match m1141 {
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

pub fn f301(v0: bool, v1: Adt95, v2: List<List<u64>>) -> Adt95 {
    if f397(v0, match v1.clone() {
        Adt95::C0(h1142, _, _, _, _) => {
            h1142
        }
    }, v2.clone()) {
        f398(match v1.clone() {
            Adt95::C0(h1143, _, _, _, _) => {
                h1143
            }
        }, {
            let a1145 = v2;
            let a1146 = List::cons(match v1 {
                Adt95::C0(_, _, _, h1144, _) => {
                    h1144
                }
            }, List::<List<u64>>::nil());
            append_list(a1145, a1146)
        })
    } else {
        v1
    }
}

pub fn f302(v0: Adt132) -> R<Option<List<u64>>> {
    let m1147 = f399(f400(v0))?;
    match m1147 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1148 = f104(v1)?;
            match m1148 {
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

pub fn f303(v0: Adt87) -> R<Option<List<List<u64>>>> {
    if f401(v0.clone()) {
        let m1152 = f402({
            let m1151 = match v0 {
                Adt87::C0(h1150, _, _) => {
                    h1150
                }
            };
            match m1151 {
                Adt84::C0(h1149, _, _) => {
                    h1149
                }
            }
        });
        match m1152 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1153 = f399(v1)?;
                match m1153 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1154 = f104(v2)?;
                        match m1154 {
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

pub fn f304() -> u64 {
    404u64
}

pub fn f305(v0: Adt89) -> u64 {
    let m1155 = v0;
    match m1155 {
        Adt89::C0(_) => {
            f111()
        }
        Adt89::C1(_) => {
            f111()
        }
        Adt89::C2(_) => {
            f111()
        }
        Adt89::C3(_) => {
            f111()
        }
        Adt89::C4(_) => {
            f111()
        }
        Adt89::C5(_) => {
            f111()
        }
        Adt89::C6(_) => {
            f32()
        }
        Adt89::C7(_) => {
            f403()
        }
        Adt89::C8(_) => {
            f111()
        }
        Adt89::C9(_) => {
            f32()
        }
        Adt89::C10 => {
            f111()
        }
        Adt89::C11 => {
            f111()
        }
        Adt89::C12(_) => {
            f32()
        }
        Adt89::C13 => {
            f32()
        }
        Adt89::C14 => {
            f32()
        }
        Adt89::C15(_) => {
            f32()
        }
    }
}

pub fn f306(v0: Adt133) -> i64 {
    let m1156 = v0;
    match m1156 {
        Adt133::C0(_) => {
            f404()
        }
        Adt133::C1(_, _) => {
            f405()
        }
        Adt133::C2(_) => {
            f406()
        }
        Adt133::C3(_) => {
            f407()
        }
        Adt133::C4(v6) => {
            f408(v6)
        }
    }
}

pub fn f307(v0: Adt133) -> Str {
    let m1157 = v0;
    match m1157 {
        Adt133::C0(v1) => {
            f409(v1)
        }
        Adt133::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt133::C2(v4) => {
            f410(v4)
        }
        Adt133::C3(_) => {
            Str::lit("Method not found")
        }
        Adt133::C4(v6) => {
            let m1158 = v6;
            match m1158 {
                Adt89::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt89::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt89::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt89::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt89::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt89::C5(_) => {
                    Str::lit("Invalid params: invalid cursor")
                }
                Adt89::C6(_) => {
                    Str::lit("Internal error")
                }
                Adt89::C7(_) => {
                    Str::lit("Rate limit exceeded: too many tool calls")
                }
                Adt89::C8(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt89::C9(_) => {
                    Str::lit("Internal error")
                }
                Adt89::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt89::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt89::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt89::C13 => {
                    Str::lit("Internal error")
                }
                Adt89::C14 => {
                    Str::lit("Internal error")
                }
                Adt89::C15(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f308(v0: Adt133) -> R<Option<Adt6>> {
    let m1159 = v0;
    match m1159 {
        Adt133::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt133::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f411(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt133::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt133::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt133::C4(v6) => {
            let m1160 = v6;
            match m1160 {
                Adt89::C0(_) => {
                    Ok(None::<Adt6>)
                }
                Adt89::C1(_) => {
                    Ok(None::<Adt6>)
                }
                Adt89::C2(v9) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(v9)), List::<(Str, Adt6)>::nil()))))
                }
                Adt89::C3(_) => {
                    Ok(None::<Adt6>)
                }
                Adt89::C4(v11) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("reason"), Adt6::C3(v11)), List::<(Str, Adt6)>::nil()))))
                }
                Adt89::C5(_) => {
                    Ok(None::<Adt6>)
                }
                Adt89::C6(_) => {
                    Ok(None::<Adt6>)
                }
                Adt89::C7(v14) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f64(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt89::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f412(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt89::C9(_) => {
                    Ok(None::<Adt6>)
                }
                Adt89::C10 => {
                    Ok(None::<Adt6>)
                }
                Adt89::C11 => {
                    Ok(None::<Adt6>)
                }
                Adt89::C12(_) => {
                    Ok(None::<Adt6>)
                }
                Adt89::C13 => {
                    Ok(None::<Adt6>)
                }
                Adt89::C14 => {
                    Ok(None::<Adt6>)
                }
                Adt89::C15(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f309(v0: List<u64>) -> R<bool> {
    let m1161 = f250(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1161.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f413(v2)? {
                Ok(false)
            } else {
                f156(f158(v1), f414())
            }
        }
    }
}

pub fn f310(v0: Adt0, v1: Fn0, v2: u64, v3: Adt140) -> R<Result<Adt130, Adt133>> {
    f415(v0.clone(), v1, v2, v3.clone(), f416(match v0 {
        Adt0::C0(_, _, h1162, _, _, _, _, _) => {
            h1162
        }
    }, match v3 {
        Adt140::C0(_, h1163, _, _) => {
            h1163
        }
    }))
}

pub fn f311(v0: Adt146, v1: Adt147) -> Result<Adt140, Adt133> {
    let m1164 = v1;
    match m1164 {
        Adt147::C0(v3) => {
            f417(v3, v0)
        }
        Adt147::C1(v2) => {
            Err::<Adt140, Adt133>(v2)
        }
    }
}

pub fn f312(v0: Str) -> Adt146 {
    let a1165 = v0.clone();
    let a1166 = Str::lit("server/discover");
    if equal(a1165, a1166) {
        Adt146::C1(Adt141::C0)
    } else {
        let a1167 = v0.clone();
        let a1168 = Str::lit("tools/list");
        if equal(a1167, a1168) {
            Adt146::C1(Adt141::C1)
        } else {
            let a1169 = v0.clone();
            let a1170 = Str::lit("tools/call");
            if equal(a1169, a1170) {
                Adt146::C1(Adt141::C2)
            } else {
                let a1171 = v0.clone();
                let a1172 = Str::lit("resources/list");
                if equal(a1171, a1172) {
                    Adt146::C1(Adt141::C3)
                } else {
                    let a1173 = v0.clone();
                    let a1174 = Str::lit("resources/read");
                    if equal(a1173, a1174) {
                        Adt146::C1(Adt141::C4)
                    } else {
                        let a1175 = v0.clone();
                        let a1176 = Str::lit("resources/templates/list");
                        if equal(a1175, a1176) {
                            Adt146::C1(Adt141::C5)
                        } else {
                            let a1177 = v0.clone();
                            let a1178 = Str::lit("prompts/list");
                            if equal(a1177, a1178) {
                                Adt146::C1(Adt141::C6)
                            } else {
                                let a1179 = v0.clone();
                                let a1180 = Str::lit("prompts/get");
                                if equal(a1179, a1180) {
                                    Adt146::C1(Adt141::C7)
                                } else {
                                    let a1181 = v0.clone();
                                    let a1182 = Str::lit("completion/complete");
                                    if equal(a1181, a1182) {
                                        Adt146::C1(Adt141::C8)
                                    } else {
                                        let a1183 = v0.clone();
                                        let a1184 = Str::lit("tasks/get");
                                        if equal(a1183, a1184) {
                                            Adt146::C1(Adt141::C9)
                                        } else {
                                            let a1185 = v0.clone();
                                            let a1186 = Str::lit("tasks/update");
                                            if equal(a1185, a1186) {
                                                Adt146::C1(Adt141::C10)
                                            } else {
                                                let a1187 = v0;
                                                let a1188 = Str::lit("tasks/cancel");
                                                if equal(a1187, a1188) {
                                                    Adt146::C1(Adt141::C11)
                                                } else {
                                                    Adt146::C0
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

pub fn f313(v0: Option<Adt115>, v1: List<Str>, v2: Adt111) -> R<Adt147> {
    f418(v0, v1, v2.clone(), f419(match v2 {
        Adt111::C0(_, _, h1189) => {
            h1189
        }
    }))
}

pub fn f314(v0: Str) -> Adt149 {
    let m1190 = f312(v0);
    match m1190 {
        Adt146::C0 => {
            Adt149::C0
        }
        Adt146::C1(v1) => {
            f420(v1)
        }
    }
}

pub fn f315(v0: List<u64>) -> Option<List<u64>> {
    let m1191 = f421(f422(), v0);
    match m1191 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1192 = f421(f423(), f166(v1, List::<u64>::nil()));
            match m1192 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f166(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f316(p1195: List<Adt118>, p1196: List<Adt118>) -> List<Adt118> {
    let m1194 = (p1195, p1196);
    {
        let mut m1194 = m1194;
        loop {
            let step = {
                let (v0, v1) = m1194;
                let m1193 = v0;
                match m1193.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<Adt118>), List<Adt118>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt118>, List<Adt118>), List<Adt118>>((v3, List::cons(v2, v1)))
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

pub fn f317(v0: List<Adt93>, p1203: List<Adt12>, v2: Option<Str>, p1204: List<Adt118>) -> R<List<Adt118>> {
    let m1202 = (p1203, p1204);
    {
        let mut m1202 = m1202;
        loop {
            let step = {
                let (v1, v3) = m1202;
                let m1197 = v1;
                match m1197.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt118>), R<List<Adt118>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f424(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt118>), R<List<Adt118>>>((v5, List::cons(Adt118::C0(match v4.clone() {
                                Adt12::C0(_, h1198, _, _) => {
                                    h1198
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1199, _) => {
                                    h1199
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1200) => {
                                    h1200
                                }
                            }, f425(v0.clone(), match v4 {
                                Adt12::C0(_, h1201, _, _) => {
                                    h1201
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt118>), R<List<Adt118>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(p1220: u64, p1221: i64, p1222: i64) -> R<Option<i64>> {
    let m1219 = (p1220, (p1221, p1222));
    {
        let mut m1219 = m1219;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1219;
                let m1205 = v0;
                if m1205 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1205 - 1;
                    if f426(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1208 = {
                            let a1206 = v2;
                            let a1207 = 0i64;
                            compare(a1206, a1207)
                        };
                        if match m1208 {
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
                            if f426({
                                let a1209 = v1;
                                let a1210 = 10i64;
                                let a1211 = 0i64;
                                int_rem(a1209, a1210, a1211)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1212 = v1;
                                    let a1213 = 10i64;
                                    let a1214 = 0i64;
                                    int_quot(a1212, a1213, a1214)?
                                }, f93(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1215 = v1;
                                let a1216 = 10i64;
                                int_mul(a1215, a1216)?
                            }, {
                                let a1217 = v2;
                                let a1218 = 1i64;
                                int_sub(a1217, a1218)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: i64) -> Option<i64> {
    let m1225 = {
        let a1223 = 9007199254740991i64;
        let a1224 = v0;
        compare(a1223, a1224)
    };
    if if match m1225 {
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
        let m1228 = {
            let a1226 = v0;
            let a1227 = -9007199254740991i64;
            compare(a1226, a1227)
        };
        match m1228 {
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

pub fn f320(v0: Adt137) -> bool {
    let m1229 = v0;
    match m1229 {
        Adt137::C0 => {
            false
        }
        Adt137::C1(_) => {
            true
        }
    }
}

pub fn f321(v0: Adt99, v1: Str, v2: Adt5) -> Result<Adt111, Adt101> {
    let m1230 = f199(Str::lit("params"), v2);
    match m1230 {
        Adt137::C0 => {
            Ok::<Adt111, Adt101>(Adt111::C0(v0, v1, None::<Adt5>))
        }
        Adt137::C1(v3) => {
            let m1231 = f217(v3);
            match m1231 {
                None => {
                    f152(Some(v0), Adt102::C6)
                }
                Some(v4) => {
                    Ok::<Adt111, Adt101>(Adt111::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f322(v0: Adt126) -> bool {
    let m1232 = v0;
    match m1232 {
        Adt126::C0 => {
            false
        }
        Adt126::C1 => {
            true
        }
        Adt126::C2 => {
            true
        }
        Adt126::C3 => {
            false
        }
        Adt126::C4 => {
            true
        }
        Adt126::C5 => {
            false
        }
        Adt126::C6 => {
            false
        }
        Adt126::C7 => {
            true
        }
    }
}

pub fn f323(v0: Adt122, v1: Adt127, v2: Adt123, v3: u64) -> R<Adt138> {
    let a1234 = match v0.clone() {
        Adt122::C0(_, _, h1233) => {
            h1233
        }
    };
    let a1235 = f375();
    if nat_lt(a1234, a1235) {
        Ok(Adt138::C1(Adt122::C0(v2, List::cons(v1, match v0.clone() {
            Adt122::C0(_, h1236, _) => {
                h1236
            }
        }), {
            let a1238 = match v0 {
                Adt122::C0(_, _, h1237) => {
                    h1237
                }
            };
            let a1239 = 1u64;
            nat_add(a1238, a1239)?
        })))
    } else {
        Ok(f276(Adt100::C9(v3)))
    }
}

pub fn f324(v0: bool, v1: Adt126, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt123 {
    Adt123::C8(Adt125::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f325(v0: u64) -> i64 {
    let a1240 = v0;
    let a1241 = 0u64;
    if nat_eq(a1240, a1241) {
        0i64
    } else {
        let a1242 = v0;
        let a1243 = 1u64;
        if nat_eq(a1242, a1243) {
            1i64
        } else {
            let a1244 = v0;
            let a1245 = 2u64;
            if nat_eq(a1244, a1245) {
                2i64
            } else {
                let a1246 = v0;
                let a1247 = 3u64;
                if nat_eq(a1246, a1247) {
                    3i64
                } else {
                    let a1248 = v0;
                    let a1249 = 4u64;
                    if nat_eq(a1248, a1249) {
                        4i64
                    } else {
                        let a1250 = v0;
                        let a1251 = 5u64;
                        if nat_eq(a1250, a1251) {
                            5i64
                        } else {
                            let a1252 = v0;
                            let a1253 = 6u64;
                            if nat_eq(a1252, a1253) {
                                6i64
                            } else {
                                let a1254 = v0;
                                let a1255 = 7u64;
                                if nat_eq(a1254, a1255) {
                                    7i64
                                } else {
                                    let a1256 = v0;
                                    let a1257 = 8u64;
                                    if nat_eq(a1256, a1257) {
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

pub fn f326(p1260: List<Adt6>, p1261: List<Adt6>) -> List<Adt6> {
    let m1259 = (p1260, p1261);
    {
        let mut m1259 = m1259;
        loop {
            let step = {
                let (v0, v1) = m1259;
                let m1258 = v0;
                match m1258.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1259 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: Adt122, v1: u64) -> Adt138 {
    let m1263 = match v0.clone() {
        Adt122::C0(_, h1262, _) => {
            h1262
        }
    };
    match m1263.uncons() {
        None => {
            f276(Adt100::C2(v1))
        }
        Some((v2, _)) => {
            let m1264 = v2;
            match m1264 {
                Adt127::C0(_) => {
                    f275(v0, Adt123::C0)
                }
                Adt127::C1(_, _) => {
                    f275(v0, Adt123::C4)
                }
            }
        }
    }
}

pub fn f328(p1267: List<(Str, Adt6)>, p1268: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1266 = (p1267, p1268);
    {
        let mut m1266 = m1266;
        loop {
            let step = {
                let (v0, v1) = m1266;
                let m1265 = v0;
                match m1265.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
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

pub fn f329(v0: Adt122, v1: Adt123) -> Adt122 {
    Adt122::C0(v1, match v0.clone() {
        Adt122::C0(_, h1269, _) => {
            h1269
        }
    }, match v0 {
        Adt122::C0(_, _, h1270) => {
            h1270
        }
    })
}

pub fn f330(v0: Adt122, v1: u64, v2: List<u64>, v3: bool) -> Adt138 {
    let m1272 = {
        let a1271 = f234(f102(), f186(v2, List::<u64>::nil()));
        utf8_decode(a1271)
    };
    match m1272 {
        None => {
            f276(Adt100::C14)
        }
        Some(v4) => {
            if v3 {
                f427(v0, v1, v4)
            } else {
                f336(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f331(v0: u64) -> u64 {
    let a1273 = v0;
    let a1274 = 34u64;
    if nat_eq(a1273, a1274) {
        34u64
    } else {
        let a1275 = v0;
        let a1276 = 92u64;
        if nat_eq(a1275, a1276) {
            92u64
        } else {
            let a1277 = v0;
            let a1278 = 47u64;
            if nat_eq(a1277, a1278) {
                47u64
            } else {
                let a1279 = v0;
                let a1280 = 98u64;
                if nat_eq(a1279, a1280) {
                    8u64
                } else {
                    let a1281 = v0;
                    let a1282 = 102u64;
                    if nat_eq(a1281, a1282) {
                        12u64
                    } else {
                        let a1283 = v0;
                        let a1284 = 110u64;
                        if nat_eq(a1283, a1284) {
                            10u64
                        } else {
                            let a1285 = v0;
                            let a1286 = 114u64;
                            if nat_eq(a1285, a1286) {
                                13u64
                            } else {
                                let a1287 = v0;
                                let a1288 = 116u64;
                                if nat_eq(a1287, a1288) {
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

pub fn f332(v0: u64) -> u64 {
    let a1289 = v0;
    let a1290 = 48u64;
    if nat_lt(a1289, a1290) {
        16u64
    } else {
        let a1291 = v0;
        let a1292 = 58u64;
        if nat_lt(a1291, a1292) {
            let a1293 = v0;
            let a1294 = 48u64;
            nat_sub(a1293, a1294)
        } else {
            let a1295 = v0;
            let a1296 = 65u64;
            if nat_lt(a1295, a1296) {
                16u64
            } else {
                let a1297 = v0;
                let a1298 = 71u64;
                if nat_lt(a1297, a1298) {
                    let a1299 = v0;
                    let a1300 = 55u64;
                    nat_sub(a1299, a1300)
                } else {
                    let a1301 = v0;
                    let a1302 = 97u64;
                    if nat_lt(a1301, a1302) {
                        16u64
                    } else {
                        let a1303 = v0;
                        let a1304 = 103u64;
                        if nat_lt(a1303, a1304) {
                            let a1305 = v0;
                            let a1306 = 87u64;
                            nat_sub(a1305, a1306)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f333(v0: Adt122, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt138> {
    let a1307 = v3;
    let a1308 = 0u64;
    if nat_eq(a1307, a1308) {
        if f267(v2, 55296u64, 56319u64) {
            Ok(f275(v0, Adt123::C6(Adt124::C3(v2), v4, v5)))
        } else {
            if f267(v2, 56320u64, 57343u64) {
                Ok(f276(Adt100::C6(v1)))
            } else {
                Ok(f275(v0, Adt123::C6(Adt124::C0, f428(v2, v4)?, v5)))
            }
        }
    } else {
        if f267(v2, 56320u64, 57343u64) {
            Ok(f275(v0, Adt123::C6(Adt124::C0, f428({
                let a1317 = 65536u64;
                let a1318 = {
                    let a1315 = {
                        let a1311 = {
                            let a1309 = v3;
                            let a1310 = 55296u64;
                            nat_sub(a1309, a1310)
                        };
                        let a1312 = 1024u64;
                        nat_mul(a1311, a1312)?
                    };
                    let a1316 = {
                        let a1313 = v2;
                        let a1314 = 56320u64;
                        nat_sub(a1313, a1314)
                    };
                    nat_add(a1315, a1316)?
                };
                nat_add(a1317, a1318)?
            }, v4)?, v5)))
        } else {
            Ok(f276(Adt100::C6(v1)))
        }
    }
}

pub fn f334(v0: u64, v1: u64) -> u64 {
    let m1321 = {
        let a1319 = f335(v0);
        let a1320 = v1;
        index_list(a1319, a1320)
    };
    match m1321 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f335(v0: u64) -> List<u64> {
    let a1322 = v0;
    let a1323 = 0u64;
    if nat_eq(a1322, a1323) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1324 = v0;
        let a1325 = 1u64;
        if nat_eq(a1324, a1325) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f336(v0: Adt122, v1: Adt6) -> Adt138 {
    Adt138::C1(f269(v1, match v0.clone() {
        Adt122::C0(_, h1326, _) => {
            h1326
        }
    }, match v0 {
        Adt122::C0(_, _, h1327) => {
            h1327
        }
    }))
}

pub fn f337(v0: u64) -> Adt6 {
    let a1328 = v0;
    let a1329 = 0u64;
    if nat_eq(a1328, a1329) {
        Adt6::C1(true)
    } else {
        let a1330 = v0;
        let a1331 = 1u64;
        if nat_eq(a1330, a1331) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f338(v0: Adt122, v1: Adt125, v2: Adt126) -> Adt138 {
    f275(v0, Adt123::C8(Adt125::C0(match v1.clone() {
        Adt125::C0(h1332, _, _, _, _, _, _) => {
            h1332
        }
    }, v2, match v1.clone() {
        Adt125::C0(_, _, h1333, _, _, _, _) => {
            h1333
        }
    }, match v1.clone() {
        Adt125::C0(_, _, _, h1334, _, _, _) => {
            h1334
        }
    }, match v1.clone() {
        Adt125::C0(_, _, _, _, h1335, _, _) => {
            h1335
        }
    }, match v1.clone() {
        Adt125::C0(_, _, _, _, _, h1336, _) => {
            h1336
        }
    }, match v1 {
        Adt125::C0(_, _, _, _, _, _, h1337) => {
            h1337
        }
    })))
}

pub fn f339(v0: Adt122, v1: u64, v2: u64, v3: Adt125, v4: bool) -> R<Adt138> {
    let v5: u64 = {
        let a1339 = match v3.clone() {
            Adt125::C0(_, _, _, h1338, _, _, _) => {
                h1338
            }
        };
        let a1340 = 0u64;
        if if nat_eq(a1339, a1340) {
            let a1341 = v2;
            let a1342 = 48u64;
            nat_eq(a1341, a1342)
        } else {
            false
        } {
            0u64
        } else {
            let a1344 = match v3.clone() {
                Adt125::C0(_, _, _, h1343, _, _, _) => {
                    h1343
                }
            };
            let a1345 = 1u64;
            nat_add(a1344, a1345)?
        }
    };
    let v6: i64 = if v4 {
        f93(match v3.clone() {
            Adt125::C0(_, _, _, _, h1346, _, _) => {
                h1346
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt125::C0(_, _, _, _, h1347, _, _) => {
                h1347
            }
        }
    };
    let a1348 = f429();
    let a1349 = v5;
    if nat_lt(a1348, a1349) {
        Ok(f276(Adt100::C8(v1)))
    } else {
        let m1352 = {
            let a1350 = 99999i64;
            let a1351 = v6;
            compare(a1350, a1351)
        };
        if match m1352 {
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
            Ok(f276(Adt100::C8(v1)))
        } else {
            Ok(f275(v0, f324(match v3.clone() {
                Adt125::C0(h1353, _, _, _, _, _, _) => {
                    h1353
                }
            }, if v4 {
                Adt126::C4
            } else {
                Adt126::C2
            }, f93({
                let a1355 = match v3.clone() {
                    Adt125::C0(_, _, h1354, _, _, _, _) => {
                        h1354
                    }
                };
                let a1356 = 10i64;
                int_mul(a1355, a1356)?
            }, f325({
                let a1357 = v2;
                let a1358 = 48u64;
                nat_sub(a1357, a1358)
            }))?, v5, v6, match v3.clone() {
                Adt125::C0(_, _, _, _, _, h1359, _) => {
                    h1359
                }
            }, match v3 {
                Adt125::C0(_, _, _, _, _, _, h1360) => {
                    h1360
                }
            })))
        }
    }
}

pub fn f340(v0: u64) -> bool {
    let a1361 = v0;
    let a1362 = 101u64;
    if nat_eq(a1361, a1362) {
        true
    } else {
        let a1363 = v0;
        let a1364 = 69u64;
        nat_eq(a1363, a1364)
    }
}

pub fn f341(v0: u64) -> bool {
    f267(v0, 48u64, 57u64)
}

pub fn f342(v0: Adt122, v1: u64, v2: u64, v3: Adt125) -> R<Adt138> {
    let m1365 = f268(v3)?;
    match m1365 {
        Adt142::C0 => {
            Ok(f276(Adt100::C7(v1)))
        }
        Adt142::C1 => {
            Ok(f276(Adt100::C8(v1)))
        }
        Adt142::C2(v4) => {
            Ok(f273(f269(v4, match v0.clone() {
                Adt122::C0(_, h1366, _) => {
                    h1366
                }
            }, match v0 {
                Adt122::C0(_, _, h1367) => {
                    h1367
                }
            }), v1, v2))
        }
    }
}

pub fn f343(v0: Adt122, v1: u64, v2: u64, v3: Adt125, v4: bool) -> R<Adt138> {
    let v5: i64 = f93({
        let a1369 = match v3.clone() {
            Adt125::C0(_, _, _, _, _, _, h1368) => {
                h1368
            }
        };
        let a1370 = 10i64;
        int_mul(a1369, a1370)?
    }, f325({
        let a1371 = v2;
        let a1372 = 48u64;
        nat_sub(a1371, a1372)
    }))?;
    let m1375 = {
        let a1373 = 99999i64;
        let a1374 = v5;
        compare(a1373, a1374)
    };
    if match m1375 {
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
        Ok(f276(Adt100::C8(v1)))
    } else {
        Ok(f275(v0, f324(match v3.clone() {
            Adt125::C0(h1376, _, _, _, _, _, _) => {
                h1376
            }
        }, Adt126::C7, match v3.clone() {
            Adt125::C0(_, _, h1377, _, _, _, _) => {
                h1377
            }
        }, match v3.clone() {
            Adt125::C0(_, _, _, h1378, _, _, _) => {
                h1378
            }
        }, match v3 {
            Adt125::C0(_, _, _, _, h1379, _, _) => {
                h1379
            }
        }, v4, v5)))
    }
}

pub fn f344(p1383: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1382 = p1383;
    {
        let mut m1382 = m1382;
        loop {
            let step = {
                let v0 = m1382;
                let m1380 = v0;
                match m1380.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1381 = f213(v2.clone(), v1.clone());
                        match m1381 {
                            Adt137::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt137::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f346(p1390: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1389 = p1390;
    {
        let mut m1389 = m1389;
        loop {
            let step = {
                let v0 = m1389;
                let m1384 = v0;
                match m1384.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1386 = f213({
                            let (h1385, _) = v2.clone();
                            h1385
                        }, v1.clone());
                        match m1386 {
                            Adt137::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt137::C1(v4) => {
                                if f430({
                                    let (_, h1387) = v2;
                                    h1387
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1388, _) = v2;
                                        h1388
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f348(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt96> {
    let m1391 = f213(Str::lit("$ref"), v2);
    match m1391 {
        Adt137::C0 => {
            None::<Adt96>
        }
        Adt137::C1(v3) => {
            let m1392 = f214(v3);
            match m1392 {
                None => {
                    None::<Adt96>
                }
                Some(v4) => {
                    let m1393 = f431(v1, v4.clone());
                    match m1393 {
                        None => {
                            Some(Adt96::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt96>
                        }
                    }
                }
            }
        }
    }
}

pub fn f349(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt112) -> R<Adt144> {
    if v3 {
        let m1394 = f214(v2);
        match m1394 {
            None => {
                Ok(Adt144::C0(Adt96::C7(v0)))
            }
            Some(v6) => {
                f432(v0, v1, v6.clone(), f91(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt144::C0(Adt96::C10(v0)))
    }
}

pub fn f350(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt105>) -> List<Adt105> {
    let m1395 = f213(v0.clone(), v1);
    match m1395 {
        Adt137::C0 => {
            v4
        }
        Adt137::C1(v5) => {
            let m1396 = f217(v5);
            match m1396 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1397 = v0;
                    let a1398 = Str::lit("properties");
                    if equal(a1397, a1398) {
                        f433(match v6 {
                            Adt5::C0(h1399) => {
                                h1399
                            }
                        }, v2, v3, v4)
                    } else {
                        f434(match v6 {
                            Adt5::C0(h1400) => {
                                h1400
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f351(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1401 = f213(v0, v1);
    match m1401 {
        Adt137::C0 => {
            v2
        }
        Adt137::C1(v3) => {
            List::cons(Adt105::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f352(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1402 = f213(v0, v1);
    match m1402 {
        Adt137::C0 => {
            v2
        }
        Adt137::C1(v3) => {
            let m1403 = f288(v3);
            match m1403 {
                None => {
                    v2
                }
                Some(v4) => {
                    f435(v4, v2)
                }
            }
        }
    }
}

pub fn f353(p1410: List<(Str, Adt145)>) -> Adt128 {
    let m1409 = p1410;
    {
        let mut m1409 = m1409;
        loop {
            let step = {
                let v0 = m1409;
                let m1404 = v0;
                match m1404.uncons() {
                    None => {
                        Err::<List<(Str, Adt145)>, Adt128>(Adt128::C3(List::<Adt109>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1406 = {
                            let (_, h1405) = v1;
                            h1405
                        };
                        match m1406 {
                            Adt145::C0 => {
                                Ok::<List<(Str, Adt145)>, Adt128>(v2)
                            }
                            Adt145::C1 => {
                                Err::<List<(Str, Adt145)>, Adt128>(Adt128::C0({
                                    let (h1407, _) = v1;
                                    h1407
                                }))
                            }
                            Adt145::C2 => {
                                Err::<List<(Str, Adt145)>, Adt128>(Adt128::C1({
                                    let (h1408, _) = v1;
                                    h1408
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt145> {
    let m1411 = f213(Str::lit("type"), v0);
    match m1411 {
        Adt137::C0 => {
            Ok(Adt145::C0)
        }
        Adt137::C1(v2) => {
            let m1412 = f214(v2.clone());
            match m1412 {
                None => {
                    let m1413 = f288(v2);
                    match m1413 {
                        None => {
                            Ok(Adt145::C2)
                        }
                        Some(v4) => {
                            f438(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f436(v3.clone()) {
                        if f437(v3, v1)? {
                            Ok(Adt145::C0)
                        } else {
                            Ok(Adt145::C1)
                        }
                    } else {
                        Ok(Adt145::C2)
                    }
                }
            }
        }
    }
}

pub fn f355(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt145> {
    let m1414 = f213(Str::lit("enum"), v0);
    match m1414 {
        Adt137::C0 => {
            Ok(Adt145::C0)
        }
        Adt137::C1(v2) => {
            let m1415 = f288(v2);
            match m1415 {
                None => {
                    Ok(Adt145::C2)
                }
                Some(v3) => {
                    f439(v3, v1)
                }
            }
        }
    }
}

pub fn f356(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt145> {
    let m1416 = f213(Str::lit("const"), v0);
    match m1416 {
        Adt137::C0 => {
            Ok(Adt145::C0)
        }
        Adt137::C1(v2) => {
            let m1417 = f440(v2, v1)?;
            match m1417 {
                Adt153::C0 => {
                    Ok(Adt145::C0)
                }
                Adt153::C1 => {
                    Ok(Adt145::C1)
                }
                Adt153::C2 => {
                    Ok(Adt145::C2)
                }
            }
        }
    }
}

pub fn f357(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt145> {
    let m1418 = f213(v0, v2);
    match m1418 {
        Adt137::C0 => {
            Ok(Adt145::C0)
        }
        Adt137::C1(v4) => {
            let m1419 = f441(v4);
            match m1419 {
                None => {
                    Ok(Adt145::C2)
                }
                Some(v5) => {
                    let m1420 = f441(v3);
                    match m1420 {
                        None => {
                            Ok(Adt145::C0)
                        }
                        Some(v6) => {
                            if f442(v1, f443(v6, v5)?) {
                                Ok(Adt145::C0)
                            } else {
                                Ok(Adt145::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f358(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt145)>> {
    let m1421 = v1;
    match m1421 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt145)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt145)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt145)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f444(Str::lit("minLength"), true, v0.clone(), f445(v4.clone())?)?), List::cons((Str::lit("maxLength"), f444(Str::lit("maxLength"), false, v0, f445(v4)?)?), List::<(Str, Adt145)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f444(Str::lit("minItems"), true, v0.clone(), f446(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f444(Str::lit("maxItems"), false, v0, f446(v5, 0u64)?)?), List::<(Str, Adt145)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f444(Str::lit("minProperties"), true, v0.clone(), f447(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f444(Str::lit("maxProperties"), false, v0, f447(v6, 0u64)?)?), List::<(Str, Adt145)>::nil())))
        }
    }
}

pub fn f359(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt145 {
    let m1422 = f213(Str::lit("required"), v0);
    match m1422 {
        Adt137::C0 => {
            Adt145::C0
        }
        Adt137::C1(v2) => {
            let m1423 = f288(v2);
            match m1423 {
                None => {
                    Adt145::C2
                }
                Some(v3) => {
                    let m1424 = f217(v1);
                    match m1424 {
                        None => {
                            Adt145::C0
                        }
                        Some(v4) => {
                            f448(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f360(v0: Adt128, v1: Adt128) -> Adt128 {
    let m1425 = v0.clone();
    match m1425 {
        Adt128::C0(_) => {
            v0
        }
        Adt128::C1(_) => {
            v0
        }
        Adt128::C2(_) => {
            v0
        }
        Adt128::C3(v5) => {
            let m1426 = v1.clone();
            match m1426 {
                Adt128::C0(_) => {
                    v1
                }
                Adt128::C1(_) => {
                    v1
                }
                Adt128::C2(_) => {
                    v1
                }
                Adt128::C3(v9) => {
                    Adt128::C3({
                        let a1427 = v5;
                        let a1428 = v9;
                        append_list(a1427, a1428)
                    })
                }
            }
        }
    }
}

pub fn f361(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> Adt128 {
    let m1429 = f213(Str::lit("$ref"), v1);
    match m1429 {
        Adt137::C0 => {
            Adt128::C3(List::<Adt109>::nil())
        }
        Adt137::C1(v4) => {
            let m1430 = f214(v4);
            match m1430 {
                None => {
                    Adt128::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1431 = f431(v0, v5.clone());
                    match m1431 {
                        None => {
                            Adt128::C2(v5)
                        }
                        Some(v6) => {
                            Adt128::C3(List::cons(Adt109::C0(v6, v2, v3), List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f362(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt128 {
    let m1432 = f213(Str::lit("properties"), v0);
    match m1432 {
        Adt137::C0 => {
            Adt128::C3(List::<Adt109>::nil())
        }
        Adt137::C1(v3) => {
            let m1433 = f217(v3);
            match m1433 {
                None => {
                    Adt128::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1434 = f217(v1);
                    match m1434 {
                        None => {
                            Adt128::C3(List::<Adt109>::nil())
                        }
                        Some(v5) => {
                            Adt128::C3(f449(match v4 {
                                Adt5::C0(h1435) => {
                                    h1435
                                }
                            }, match v5 {
                                Adt5::C0(h1436) => {
                                    h1436
                                }
                            }, v2, List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f363(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt128 {
    let m1437 = f213(Str::lit("additionalProperties"), v0.clone());
    match m1437 {
        Adt137::C0 => {
            Adt128::C3(List::<Adt109>::nil())
        }
        Adt137::C1(v3) => {
            let m1438 = f217(v1);
            match m1438 {
                None => {
                    Adt128::C3(List::<Adt109>::nil())
                }
                Some(v4) => {
                    let m1439 = f216(v3.clone());
                    match m1439 {
                        None => {
                            Adt128::C3(f452(match v4 {
                                Adt5::C0(h1441) => {
                                    h1441
                                }
                            }, f451(v0), v3, v2, List::<Adt109>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt128::C3(List::<Adt109>::nil())
                            } else {
                                if f450(match v4 {
                                    Adt5::C0(h1440) => {
                                        h1440
                                    }
                                }, f451(v0)) {
                                    Adt128::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt128::C3(List::<Adt109>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f364(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> R<Adt128> {
    let m1442 = f213(Str::lit("items"), v0);
    match m1442 {
        Adt137::C0 => {
            Ok(Adt128::C3(List::<Adt109>::nil()))
        }
        Adt137::C1(v3) => {
            let m1443 = f288(v1);
            match m1443 {
                None => {
                    Ok(Adt128::C3(List::<Adt109>::nil()))
                }
                Some(v4) => {
                    Ok(Adt128::C3(f453(v4, 0u64, v3, v2, List::<Adt109>::nil())?))
                }
            }
        }
    }
}

pub fn f365(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt128 {
    let m1444 = f213(Str::lit("allOf"), v0);
    match m1444 {
        Adt137::C0 => {
            Adt128::C3(List::<Adt109>::nil())
        }
        Adt137::C1(v3) => {
            let m1445 = f288(v3);
            match m1445 {
                None => {
                    Adt128::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1446 = v4.clone();
                    match m1446.uncons() {
                        None => {
                            Adt128::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt128::C3(f454(v4, v1, v2, List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f366(v0: Adt110, v1: Adt128, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt107>) -> Adt128 {
    let m1447 = v1.clone();
    match m1447 {
        Adt128::C0(_) => {
            v1
        }
        Adt128::C1(_) => {
            v1
        }
        Adt128::C2(_) => {
            v1
        }
        Adt128::C3(v8) => {
            let m1448 = v8;
            match m1448.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt128::C3(List::cons(Adt109::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt109>::nil()))
                }
            }
        }
    }
}

pub fn f367(p1455: List<u64>, p1456: List<u64>) -> List<u64> {
    let m1454 = (p1455, p1456);
    {
        let mut m1454 = m1454;
        loop {
            let step = {
                let (v0, v1) = m1454;
                let m1449 = v0;
                match m1449.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1450 = v2;
                        let a1451 = 126u64;
                        if nat_eq(a1450, a1451) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1452 = v2;
                            let a1453 = 47u64;
                            if nat_eq(a1452, a1453) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(p1465: u64, p1466: List<u64>, p1467: List<u64>) -> R<Option<List<u64>>> {
    let m1464 = (p1465, (p1466, p1467));
    {
        let mut m1464 = m1464;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1464;
                let m1457 = v0;
                if m1457 == 0 {
                    let m1458 = v1;
                    match m1458.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1457 - 1;
                    let m1459 = v1;
                    match m1459.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1460 = v7;
                            match m1460.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1461 = v9;
                                    match m1461.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1462 = v11;
                                            match m1462.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1463 = f455(v6, v8, v10, v12, f456(v13.clone()))?;
                                                    match m1463 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f186(v14, v2))))
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
                Ok(next) => { m1464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(p1472: List<Adt50>, p1473: Adt7) -> R<bool> {
    let m1471 = (p1472, p1473);
    {
        let mut m1471 = m1471;
        loop {
            let step = {
                let (v0, v1) = m1471;
                let m1468 = v0;
                match m1468.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f457(f443(match v2.clone() {
                            Adt50::C0(h1469, _, _) => {
                                h1469
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1470, _, _) => {
                                    h1470
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f370(p1482: Bytes, p1483: List<Bytes>) -> List<Bytes> {
    let m1481 = (p1482, p1483);
    {
        let mut m1481 = m1481;
        loop {
            let step = {
                let (v0, v1) = m1481;
                let m1474 = v1;
                match m1474.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1477 = {
                            let a1475 = v2.clone();
                            length_bytes(a1475)
                        };
                        let a1478 = {
                            let a1476 = v0.clone();
                            length_bytes(a1476)
                        };
                        if nat_eq(a1477, a1478) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1479 = v2;
                                let a1480 = v0;
                                append_bytes(a1479, a1480)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1481 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: u64) -> Bytes {
    let m1487 = {
        let a1484 = f458();
        let a1485 = v0;
        let a1486 = 1u64;
        slice_bytes(a1484, a1485, a1486)
    };
    match m1487 {
        None => {
            f102()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f372(v0: u64) -> List<u64> {
    f335(v0)
}

pub fn f373(v0: Adt7) -> R<bool> {
    let a1505 = {
        let m1491 = {
            let a1489 = f459();
            let a1490 = match v0.clone() {
                Adt7::C0(h1488, _) => {
                    h1488
                }
            };
            compare(a1489, a1490)
        };
        if match m1491 {
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
            let m1496 = {
                let a1494 = match v0.clone() {
                    Adt7::C0(h1492, _) => {
                        h1492
                    }
                };
                let a1495 = {
                    let a1493 = f459();
                    int_neg(a1493)?
                };
                compare(a1494, a1495)
            };
            if match m1496 {
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
                let m1500 = {
                    let a1498 = 9999i64;
                    let a1499 = match v0.clone() {
                        Adt7::C0(_, h1497) => {
                            h1497
                        }
                    };
                    compare(a1498, a1499)
                };
                if match m1500 {
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
                    let m1504 = {
                        let a1502 = match v0 {
                            Adt7::C0(_, h1501) => {
                                h1501
                            }
                        };
                        let a1503 = -9999i64;
                        compare(a1502, a1503)
                    };
                    match m1504 {
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
    Ok(bool_not(a1505))
}

pub fn f374(v0: Adt7) -> R<List<u64>> {
    let m1509 = {
        let a1507 = match v0.clone() {
            Adt7::C0(_, h1506) => {
                h1506
            }
        };
        let a1508 = 0i64;
        compare(a1507, a1508)
    };
    if if match m1509 {
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
        let m1513 = {
            let a1511 = 0i64;
            let a1512 = match v0.clone() {
                Adt7::C0(_, h1510) => {
                    h1510
                }
            };
            compare(a1511, a1512)
        };
        match m1513 {
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
        let a1516 = f460(match v0.clone() {
            Adt7::C0(h1514, _) => {
                h1514
            }
        })?;
        let a1517 = List::cons(101u64, f460(match v0 {
            Adt7::C0(_, h1515) => {
                h1515
            }
        })?);
        Ok(append_list(a1516, a1517))
    } else {
        f460(match v0 {
            Adt7::C0(h1518, _) => {
                h1518
            }
        })
    }
}

pub fn f375() -> u64 {
    512u64
}

pub fn f376(v0: u64, v1: Adt114, v2: List<Adt114>) -> List<Adt114> {
    List::cons(Adt114::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f377(p1521: List<u64>, p1522: List<u64>) -> R<List<u64>> {
    let m1520 = (p1521, p1522);
    {
        let mut m1520 = m1520;
        loop {
            let step = {
                let (v0, v1) = m1520;
                let m1519 = v0;
                match m1519.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f186(f461(v2)?, v1)))
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

pub fn f378(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f462(match v0.clone() {
        Adt33::C0(_, h1523, _, _) => {
            h1523
        }
    })), List::cons((Str::lit("capabilities"), f463(match v0.clone() {
        Adt33::C0(_, _, h1524, _) => {
            h1524
        }
    })), {
        let a1527 = f464(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1525) => {
                h1525
            }
        });
        let a1528 = f465(match v0 {
            Adt33::C0(h1526, _, _, _) => {
                h1526
            }
        })?;
        append_list(a1527, a1528)
    })))
}

pub fn f379(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f466(match v0.clone() {
        Adt34::C0(_, _, h1529) => {
            h1529
        }
    }))), {
        let a1532 = f464(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1530, _) => {
                h1530
            }
        });
        let a1533 = f465(match v0 {
            Adt34::C0(h1531, _, _) => {
                h1531
            }
        })?;
        append_list(a1532, a1533)
    }))
}

pub fn f380(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1534 = v0;
    match m1534 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1535 = f399(Adt6::C5(v1))?;
            match m1535 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1536 = f217(v2);
                    match m1536 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1537) => {
                                    h1537
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f381(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1539 = f467(match v0.clone() {
        Adt37::C0(h1538, _, _) => {
            h1538
        }
    })?;
    match m1539 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1542 = f468(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1540, _) => {
                        h1540
                    }
                });
                let a1543 = f469(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1541) => {
                        h1541
                    }
                });
                append_list(a1542, a1543)
            })))
        }
    }
}

pub fn f382(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f470(match v0.clone() {
        Adt51::C0(_, _, h1544) => {
            h1544
        }
    })?)), {
        let a1547 = f464(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1545, _) => {
                h1545
            }
        });
        let a1548 = f465(match v0 {
            Adt51::C0(h1546, _, _) => {
                h1546
            }
        })?;
        append_list(a1547, a1548)
    }))
}

pub fn f383(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f471(match v0.clone() {
        Adt52::C0(_, h1549) => {
            h1549
        }
    }))), f465(match v0 {
        Adt52::C0(h1550, _) => {
            h1550
        }
    })?))
}

pub fn f384(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f472(match v0.clone() {
        Adt53::C0(_, _, h1551) => {
            h1551
        }
    }))), {
        let a1554 = f464(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1552, _) => {
                h1552
            }
        });
        let a1555 = f465(match v0 {
            Adt53::C0(h1553, _, _) => {
                h1553
            }
        })?;
        append_list(a1554, a1555)
    }))
}

pub fn f385(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f473(match v0.clone() {
        Adt55::C0(_, _, h1556) => {
            h1556
        }
    }))), {
        let a1559 = f464(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1557, _) => {
                h1557
            }
        });
        let a1560 = f465(match v0 {
            Adt55::C0(h1558, _, _) => {
                h1558
            }
        })?;
        append_list(a1559, a1560)
    }))
}

pub fn f386(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1562 = f474(match v0.clone() {
        Adt58::C0(_, h1561) => {
            h1561
        }
    })?;
    match m1562 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1564 = f464(Str::lit("description"), match v0 {
                    Adt58::C0(h1563, _) => {
                        h1563
                    }
                });
                let a1565 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1564, a1565)
            }))
        }
    }
}

pub fn f387(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f411({
        let m1568 = match v0.clone() {
            Adt60::C0(h1567) => {
                h1567
            }
        };
        match m1568 {
            Adt61::C0(h1566, _, _) => {
                h1566
            }
        }
    }))), {
        let a1575 = f475(Str::lit("total"), {
            let m1571 = match v0.clone() {
                Adt60::C0(h1570) => {
                    h1570
                }
            };
            match m1571 {
                Adt61::C0(_, h1569, _) => {
                    h1569
                }
            }
        })?;
        let a1576 = f469(Str::lit("hasMore"), {
            let m1574 = match v0 {
                Adt60::C0(h1573) => {
                    h1573
                }
            };
            match m1574 {
                Adt61::C0(_, _, h1572) => {
                    h1572
                }
            }
        });
        append_list(a1575, a1576)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f388(v0: Adt81) -> R<Option<List<(Str, Adt6)>>> {
    let m1577 = f476(f477(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m1577 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1578 = f478(v1);
                let a1579 = f479(f480(v0));
                append_list(a1578, a1579)
            }))
        }
    }
}

pub fn f389(v0: Adt62) -> Option<List<(Str, Adt6)>> {
    let m1580 = f481(v0.clone());
    match m1580 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1581 = f482(f483(v0.clone()), f484(f485(v0)));
                let a1582 = v1;
                append_list(a1581, a1582)
            })
        }
    }
}

pub fn f390(v0: Adt87) -> Option<List<(Str, Adt6)>> {
    let m1584 = match v0.clone() {
        Adt87::C0(_, h1583, _) => {
            h1583
        }
    };
    match m1584 {
        Adt88::C0 => {
            f389({
                let m1587 = match v0 {
                    Adt87::C0(h1586, _, _) => {
                        h1586
                    }
                };
                match m1587 {
                    Adt84::C0(h1585, _, _) => {
                        h1585
                    }
                }
            })
        }
        Adt88::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f391(v0: Adt150) -> Str {
    let m1588 = v0;
    match m1588 {
        Adt150::C0 => {
            Str::lit("complete")
        }
        Adt150::C1 => {
            Str::lit("input_required")
        }
        Adt150::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f392(v0: Adt32) -> Adt150 {
    let m1589 = v0;
    match m1589 {
        Adt32::C0(_) => {
            Adt150::C0
        }
        Adt32::C1(_) => {
            Adt150::C0
        }
        Adt32::C2(_) => {
            Adt150::C0
        }
        Adt32::C3(_) => {
            Adt150::C0
        }
        Adt32::C4(_) => {
            Adt150::C0
        }
        Adt32::C5(_) => {
            Adt150::C0
        }
        Adt32::C6(_) => {
            Adt150::C0
        }
        Adt32::C7(_) => {
            Adt150::C0
        }
        Adt32::C8(_) => {
            Adt150::C0
        }
        Adt32::C9(_) => {
            Adt150::C0
        }
        Adt32::C10(_) => {
            Adt150::C0
        }
        Adt32::C11 => {
            Adt150::C0
        }
        Adt32::C12(_) => {
            Adt150::C1
        }
        Adt32::C13(_) => {
            Adt150::C2
        }
        Adt32::C14(_) => {
            Adt150::C0
        }
    }
}

pub fn f393(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1590 = v0;
    match m1590 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f486(), f487(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f394(v0: Bytes) -> R<Option<List<Adt84>>> {
    let a1592 = {
        let a1591 = v0.clone();
        length_bytes(a1591)
    };
    let a1593 = 0u64;
    if nat_eq(a1592, a1593) {
        Ok(Some(List::<Adt84>::nil()))
    } else {
        let m1594 = f81(v0)?;
        match m1594 {
            Ok(v2) => {
                let m1595 = f288(v2);
                match m1595 {
                    None => {
                        Ok(None::<List<Adt84>>)
                    }
                    Some(v3) => {
                        Ok(f488(v3, List::<Adt84>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt84>>)
            }
        }
    }
}

pub fn f395(v0: List<Adt84>) -> R<Option<List<u64>>> {
    let m1596 = f489(v0);
    match m1596 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1597 = f104(v1)?;
            match m1597 {
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

pub fn f396(v0: Adt84, v1: List<Adt84>, v2: i64) -> R<List<Adt84>> {
    Ok(List::cons(v0.clone(), f490(f491(f492(v0), v1, v2, List::<Adt84>::nil())?, List::<Adt84>::nil())))
}

pub fn f397(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f493(v2) {
            let a1598 = v1;
            let a1599 = 200u64;
            nat_eq(a1598, a1599)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f398(v0: u64, v1: List<List<u64>>) -> Adt95 {
    Adt95::C0(v0, f494(), List::<List<u64>>::nil(), f495(v1), None::<List<u64>>)
}

pub fn f399(v0: Adt6) -> R<Option<Adt6>> {
    f496(f497(), Adt151::C0(v0), List::<Adt152>::nil())
}

pub fn f400(v0: Adt132) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f143())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f498(match v0.clone() {
        Adt132::C0(h1600, _) => {
            h1600
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1603 = match v0.clone() {
            Adt132::C0(_, h1602) => {
                h1602
            }
        };
        match m1603 {
            Adt50::C0(h1601, _, _) => {
                h1601
            }
        }
    })), {
        let a1610 = f499({
            let m1606 = match v0.clone() {
                Adt132::C0(_, h1605) => {
                    h1605
                }
            };
            match m1606 {
                Adt50::C0(_, h1604, _) => {
                    h1604
                }
            }
        });
        let a1611 = f500({
            let m1609 = match v0 {
                Adt132::C0(_, h1608) => {
                    h1608
                }
            };
            match m1609 {
                Adt50::C0(_, _, h1607) => {
                    h1607
                }
            }
        });
        append_list(a1610, a1611)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f401(v0: Adt87) -> bool {
    if match v0.clone() {
        Adt87::C0(_, _, h1612) => {
            h1612
        }
    } {
        let m1614 = match v0 {
            Adt87::C0(_, h1613, _) => {
                h1613
            }
        };
        match m1614 {
            Adt88::C0 => {
                true
            }
            Adt88::C1 => {
                true
            }
        }
    } else {
        false
    }
}

pub fn f402(v0: Adt62) -> Option<Adt6> {
    let m1615 = f389(v0);
    match m1615 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f403() -> u64 {
    429u64
}

pub fn f404() -> i64 {
    -32602i64
}

pub fn f405() -> i64 {
    -32022i64
}

pub fn f406() -> i64 {
    -32020i64
}

pub fn f407() -> i64 {
    -32601i64
}

pub fn f408(v0: Adt89) -> i64 {
    let m1616 = v0;
    match m1616 {
        Adt89::C0(_) => {
            f404()
        }
        Adt89::C1(_) => {
            f404()
        }
        Adt89::C2(_) => {
            f404()
        }
        Adt89::C3(_) => {
            f404()
        }
        Adt89::C4(_) => {
            f404()
        }
        Adt89::C5(_) => {
            f404()
        }
        Adt89::C6(_) => {
            f501()
        }
        Adt89::C7(_) => {
            f502()
        }
        Adt89::C8(_) => {
            f503()
        }
        Adt89::C9(_) => {
            f501()
        }
        Adt89::C10 => {
            f404()
        }
        Adt89::C11 => {
            f404()
        }
        Adt89::C12(_) => {
            f501()
        }
        Adt89::C13 => {
            f501()
        }
        Adt89::C14 => {
            f501()
        }
        Adt89::C15(_) => {
            f501()
        }
    }
}

pub fn f409(v0: Adt134) -> Str {
    let m1617 = v0;
    match m1617 {
        Adt134::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt134::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt134::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt134::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt134::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt134::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt134::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt134::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt134::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f410(v0: Adt116) -> Str {
    let m1618 = v0;
    match m1618 {
        Adt116::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt116::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt116::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt116::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt116::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt116::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt116::C6(v1) => {
            let m1619 = v1;
            match m1619 {
                Adt117::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt117::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt117::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt116::C7(v2) => {
            let m1620 = v2;
            match m1620 {
                Adt117::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt117::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt117::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt116::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt116::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt116::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f411(v0: List<Str>) -> List<Adt6> {
    f326(f504(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f412(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1632 = f505(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1621, _, _, _, _) => {
                h1621
            }
        });
        let a1633 = {
            let a1630 = f505(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1622, _, _, _) => {
                    h1622
                }
            });
            let a1631 = {
                let a1628 = f505(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1623, _, _) => {
                        h1623
                    }
                });
                let a1629 = {
                    let a1626 = f505(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1624, _) => {
                            h1624
                        }
                    });
                    let a1627 = f505(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1625) => {
                            h1625
                        }
                    });
                    append_list(a1626, a1627)
                };
                append_list(a1628, a1629)
            };
            append_list(a1630, a1631)
        };
        append_list(a1632, a1633)
    })
}

pub fn f413(p1636: List<List<u64>>) -> R<bool> {
    let m1635 = p1636;
    {
        let mut m1635 = m1635;
        loop {
            let step = {
                let v0 = m1635;
                let m1634 = v0;
                match m1634.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f506(f158(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f415(v0: Adt0, v1: Fn0, v2: u64, v3: Adt140, v4: bool) -> R<Result<Adt130, Adt133>> {
    if v4 {
        f507(v0, v1, v2, f508(match v3.clone() {
            Adt140::C0(_, h1637, _, _) => {
                h1637
            }
        }, match v3.clone() {
            Adt140::C0(_, _, h1638, _) => {
                h1638
            }
        }, match v3 {
            Adt140::C0(_, _, _, h1639) => {
                h1639
            }
        }))
    } else {
        Ok(Err::<Adt130, Adt133>(Adt133::C3(f509(match v3 {
            Adt140::C0(_, h1640, _, _) => {
                h1640
            }
        }))))
    }
}

pub fn f416(v0: Adt4, v1: Adt141) -> bool {
    let m1641 = v1;
    match m1641 {
        Adt141::C0 => {
            true
        }
        Adt141::C1 => {
            f510(match v0 {
                Adt4::C0(_, _, _, _, _, h1642, _) => {
                    h1642
                }
            })
        }
        Adt141::C2 => {
            f510(match v0 {
                Adt4::C0(_, _, _, _, _, h1643, _) => {
                    h1643
                }
            })
        }
        Adt141::C3 => {
            f511(match v0 {
                Adt4::C0(_, _, _, _, h1644, _, _) => {
                    h1644
                }
            })
        }
        Adt141::C4 => {
            f511(match v0 {
                Adt4::C0(_, _, _, _, h1645, _, _) => {
                    h1645
                }
            })
        }
        Adt141::C5 => {
            f511(match v0 {
                Adt4::C0(_, _, _, _, h1646, _, _) => {
                    h1646
                }
            })
        }
        Adt141::C6 => {
            f510(match v0 {
                Adt4::C0(_, _, _, h1647, _, _, _) => {
                    h1647
                }
            })
        }
        Adt141::C7 => {
            f510(match v0 {
                Adt4::C0(_, _, _, h1648, _, _, _) => {
                    h1648
                }
            })
        }
        Adt141::C8 => {
            f512(match v0 {
                Adt4::C0(_, _, h1649, _, _, _, _) => {
                    h1649
                }
            })
        }
        Adt141::C9 => {
            f513(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1650) => {
                    h1650
                }
            }, f18())
        }
        Adt141::C10 => {
            f513(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1651) => {
                    h1651
                }
            }, f18())
        }
        Adt141::C11 => {
            f513(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1652) => {
                    h1652
                }
            }, f18())
        }
    }
}

pub fn f417(v0: Adt148, v1: Adt146) -> Result<Adt140, Adt133> {
    let m1653 = v1;
    match m1653 {
        Adt146::C0 => {
            Err::<Adt140, Adt133>(Adt133::C3(match v0 {
                Adt148::C0(_, h1654, _, _) => {
                    h1654
                }
            }))
        }
        Adt146::C1(v2) => {
            Ok::<Adt140, Adt133>(Adt140::C0(match v0.clone() {
                Adt148::C0(h1655, _, _, _) => {
                    h1655
                }
            }, v2, match v0.clone() {
                Adt148::C0(_, _, h1656, _) => {
                    h1656
                }
            }, match v0 {
                Adt148::C0(_, _, _, h1657) => {
                    h1657
                }
            }))
        }
    }
}

pub fn f418(v0: Option<Adt115>, v1: List<Str>, v2: Adt111, v3: Result<Adt16, Adt134>) -> R<Adt147> {
    let m1658 = v3;
    match m1658 {
        Ok(v5) => {
            Ok(f514(v1, v2.clone(), v5.clone(), f515(v0, f314(match v2.clone() {
                Adt111::C0(_, h1659, _) => {
                    h1659
                }
            }), match v2.clone() {
                Adt111::C0(_, h1660, _) => {
                    h1660
                }
            }, match v5 {
                Adt16::C0(h1661, _, _, _, _, _) => {
                    h1661
                }
            }, f516(match v2 {
                Adt111::C0(_, _, h1662) => {
                    h1662
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt147::C1(Adt133::C0(v4)))
        }
    }
}

pub fn f419(v0: Option<Adt5>) -> Result<Adt16, Adt134> {
    let m1663 = v0;
    match m1663 {
        None => {
            Err::<Adt16, Adt134>(Adt134::C0)
        }
        Some(v1) => {
            let m1664 = f199(Str::lit("_meta"), v1);
            match m1664 {
                Adt137::C0 => {
                    Err::<Adt16, Adt134>(Adt134::C1)
                }
                Adt137::C1(v2) => {
                    let m1665 = f217(v2);
                    match m1665 {
                        None => {
                            Err::<Adt16, Adt134>(Adt134::C4)
                        }
                        Some(v3) => {
                            f517(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f420(v0: Adt141) -> Adt149 {
    let m1666 = v0;
    match m1666 {
        Adt141::C0 => {
            Adt149::C0
        }
        Adt141::C1 => {
            Adt149::C0
        }
        Adt141::C2 => {
            Adt149::C1
        }
        Adt141::C3 => {
            Adt149::C0
        }
        Adt141::C4 => {
            Adt149::C2
        }
        Adt141::C5 => {
            Adt149::C0
        }
        Adt141::C6 => {
            Adt149::C0
        }
        Adt141::C7 => {
            Adt149::C1
        }
        Adt141::C8 => {
            Adt149::C0
        }
        Adt141::C9 => {
            Adt149::C3
        }
        Adt141::C10 => {
            Adt149::C3
        }
        Adt141::C11 => {
            Adt149::C3
        }
    }
}

pub fn f421(p1672: List<u64>, p1673: List<u64>) -> Option<List<u64>> {
    let m1671 = (p1672, p1673);
    {
        let mut m1671 = m1671;
        loop {
            let step = {
                let (v0, v1) = m1671;
                let m1667 = v0;
                match m1667.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1668 = v1;
                        match m1668.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1669 = v4;
                                let a1670 = v2;
                                if nat_eq(a1669, a1670) {
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
                Ok(next) => { m1671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f422() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f423() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f424(v0: Adt12, v1: Option<Str>) -> bool {
    let m1674 = v1;
    match m1674 {
        None => {
            false
        }
        Some(v2) => {
            let a1676 = match v0 {
                Adt12::C0(h1675, _, _, _) => {
                    h1675
                }
            };
            let a1677 = v2;
            equal(a1676, a1677)
        }
    }
}

pub fn f425(v0: List<Adt93>, v1: Str) -> R<Adt119> {
    let m1680 = f257(f83(v0, {
        let a1678 = f518();
        let a1679 = f519(v1)?;
        append_list(a1678, a1679)
    })?)?;
    match m1680 {
        Adt103::C0 => {
            Ok(Adt119::C0)
        }
        Adt103::C1 => {
            Ok(Adt119::C1)
        }
        Adt103::C2 => {
            Ok(Adt119::C1)
        }
        Adt103::C3(v2) => {
            let m1681 = f183(v2);
            match m1681 {
                None => {
                    Ok(Adt119::C1)
                }
                Some(v3) => {
                    Ok(Adt119::C2(v3))
                }
            }
        }
    }
}

pub fn f426(v0: i64) -> bool {
    let a1688 = {
        let m1684 = {
            let a1682 = v0;
            let a1683 = 0i64;
            compare(a1682, a1683)
        };
        if match m1684 {
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
            let m1687 = {
                let a1685 = 0i64;
                let a1686 = v0;
                compare(a1685, a1686)
            };
            match m1687 {
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
    bool_not(a1688)
}

pub fn f427(v0: Adt122, v1: u64, v2: Str) -> Adt138 {
    let m1690 = match v0.clone() {
        Adt122::C0(_, h1689, _) => {
            h1689
        }
    };
    match m1690.uncons() {
        None => {
            f276(Adt100::C2(v1))
        }
        Some((v3, v4)) => {
            let m1691 = v3;
            match m1691 {
                Adt127::C0(_) => {
                    f276(Adt100::C2(v1))
                }
                Adt127::C1(v6, _) => {
                    Adt138::C1(Adt122::C0(Adt123::C5, List::cons(Adt127::C1(v6, Some(v2)), v4), match v0 {
                        Adt122::C0(_, _, h1692) => {
                            h1692
                        }
                    }))
                }
            }
        }
    }
}

pub fn f428(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1693 = v0;
    let a1694 = 128u64;
    if nat_lt(a1693, a1694) {
        Ok(List::cons(v0, v1))
    } else {
        let a1695 = v0;
        let a1696 = 2048u64;
        if nat_lt(a1695, a1696) {
            Ok(List::cons({
                let a1697 = 128u64;
                let a1698 = f520(v0, 64u64);
                nat_add(a1697, a1698)?
            }, List::cons({
                let a1699 = 192u64;
                let a1700 = f521(v0, 64u64);
                nat_add(a1699, a1700)?
            }, v1)))
        } else {
            let a1701 = v0;
            let a1702 = 65536u64;
            if nat_lt(a1701, a1702) {
                Ok(List::cons({
                    let a1703 = 128u64;
                    let a1704 = f520(v0, 64u64);
                    nat_add(a1703, a1704)?
                }, List::cons({
                    let a1705 = 128u64;
                    let a1706 = f520(f521(v0, 64u64), 64u64);
                    nat_add(a1705, a1706)?
                }, List::cons({
                    let a1707 = 224u64;
                    let a1708 = f521(v0, 4096u64);
                    nat_add(a1707, a1708)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1709 = 128u64;
                    let a1710 = f520(v0, 64u64);
                    nat_add(a1709, a1710)?
                }, List::cons({
                    let a1711 = 128u64;
                    let a1712 = f520(f521(v0, 64u64), 64u64);
                    nat_add(a1711, a1712)?
                }, List::cons({
                    let a1713 = 128u64;
                    let a1714 = f520(f521(v0, 4096u64), 64u64);
                    nat_add(a1713, a1714)?
                }, List::cons({
                    let a1715 = 240u64;
                    let a1716 = f521(v0, 262144u64);
                    nat_add(a1715, a1716)?
                }, v1)))))
            }
        }
    }
}

pub fn f429() -> u64 {
    18u64
}

pub fn f430(v0: u64, v1: Adt6) -> bool {
    let a1717 = v0;
    let a1718 = 0u64;
    if nat_eq(a1717, a1718) {
        f522(v1)
    } else {
        let a1719 = v0;
        let a1720 = 1u64;
        if nat_eq(a1719, a1720) {
            f523(v1)
        } else {
            let a1721 = v0;
            let a1722 = 2u64;
            if nat_eq(a1721, a1722) {
                f524(v1)
            } else {
                let a1723 = v0;
                let a1724 = 3u64;
                if nat_eq(a1723, a1724) {
                    f525(v1)
                } else {
                    let a1725 = v0;
                    let a1726 = 4u64;
                    if nat_eq(a1725, a1726) {
                        f526(v1)
                    } else {
                        let a1727 = v0;
                        let a1728 = 5u64;
                        if nat_eq(a1727, a1728) {
                            f527(v1)
                        } else {
                            let a1729 = v0;
                            let a1730 = 6u64;
                            if nat_eq(a1729, a1730) {
                                f528(v1)
                            } else {
                                let a1731 = v0;
                                let a1732 = 7u64;
                                if nat_eq(a1731, a1732) {
                                    f529(v1)
                                } else {
                                    f530(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f431(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1733 = v1.clone();
    let a1734 = Str::lit("#");
    if equal(a1733, a1734) {
        Some(v0)
    } else {
        let m1735 = f217(v0);
        match m1735 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1736 = f199(Str::lit("$defs"), v2);
                match m1736 {
                    Adt137::C0 => {
                        None::<Adt6>
                    }
                    Adt137::C1(v3) => {
                        let m1737 = f217(v3);
                        match m1737 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f531(v1, match v4 {
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

pub fn f432(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt112) -> R<Adt144> {
    let m1739 = v3.clone();
    match m1739.uncons() {
        None => {
            Ok(Adt144::C0(Adt96::C8(v0)))
        }
        Some((_, _)) => {
            if f167(v3.clone(), true) {
                let m1740 = f532(v1);
                match m1740 {
                    None => {
                        Ok(Adt144::C0(Adt96::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f533(v3)?;
                        if f534(v9.clone(), match v5.clone() {
                            Adt112::C0(_, h1741) => {
                                h1741
                            }
                        }) {
                            Ok(Adt144::C0(Adt96::C12(v0, v2)))
                        } else {
                            Ok(Adt144::C1(Adt112::C0(List::cons(Adt12::C0(v0, v2, f535(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt112::C0(h1742, _) => {
                                    h1742
                                }
                            }), List::cons(v9, match v5 {
                                Adt112::C0(_, h1743) => {
                                    h1743
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt144::C0(Adt96::C9(v0)))
            }
        }
    }
}

pub fn f433(p1748: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1749: List<Adt105>) -> List<Adt105> {
    let m1747 = (p1748, p1749);
    {
        let mut m1747 = m1747;
        loop {
            let step = {
                let (v0, v3) = m1747;
                let m1744 = v0;
                match m1744.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v5, List::cons(Adt105::C0({
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

pub fn f434(p1753: List<(Str, Adt6)>, p1754: List<Adt105>) -> List<Adt105> {
    let m1752 = (p1753, p1754);
    {
        let mut m1752 = m1752;
        loop {
            let step = {
                let (v0, v1) = m1752;
                let m1750 = v0;
                match m1750.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0({
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

pub fn f435(p1757: List<Adt6>, p1758: List<Adt105>) -> List<Adt105> {
    let m1756 = (p1757, p1758);
    {
        let mut m1756 = m1756;
        loop {
            let step = {
                let (v0, v1) = m1756;
                let m1755 = v0;
                match m1755.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0(v2, false, List::<Str>::nil()), v1)))
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

pub fn f436(v0: Str) -> bool {
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

pub fn f437(v0: Str, v1: Adt6) -> R<bool> {
    let a1773 = v0.clone();
    let a1774 = f536(v1.clone());
    if equal(a1773, a1774) {
        Ok(true)
    } else {
        let a1775 = v0;
        let a1776 = Str::lit("integer");
        if equal(a1775, a1776) {
            let m1777 = f441(v1);
            match m1777 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f537(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f438(p1780: List<Adt6>, v1: Adt6, p1781: bool) -> R<Adt145> {
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
                            Err::<(List<Adt6>, bool), R<Adt145>>(Ok(Adt145::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt145>>(Ok(Adt145::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f436(f538(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt145>>((v4, if v2 {
                                true
                            } else {
                                f437(f538(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt145>>(Ok(Adt145::C2))
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

pub fn f439(p1785: List<Adt6>, v1: Adt6) -> R<Adt145> {
    let m1784 = p1785;
    {
        let mut m1784 = m1784;
        loop {
            let step = {
                let v0 = m1784;
                let m1782 = v0;
                match m1782.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt145>>(Ok(Adt145::C1))
                    }
                    Some((v2, v3)) => {
                        let m1783 = f440(v2, v1.clone())?;
                        match m1783 {
                            Adt153::C0 => {
                                Err::<List<Adt6>, R<Adt145>>(Ok(Adt145::C0))
                            }
                            Adt153::C1 => {
                                Ok::<List<Adt6>, R<Adt145>>(v3)
                            }
                            Adt153::C2 => {
                                Err::<List<Adt6>, R<Adt145>>(Ok(Adt145::C2))
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

pub fn f440(v0: Adt6, v1: Adt6) -> R<Adt153> {
    f539(f540(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f441(v0: Adt6) -> Option<Adt7> {
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

pub fn f442(v0: u64, v1: Adt154) -> bool {
    let a1787 = v0;
    let a1788 = 0u64;
    if nat_eq(a1787, a1788) {
        let a1789 = f541(v1);
        bool_not(a1789)
    } else {
        let a1790 = v0;
        let a1791 = 1u64;
        if nat_eq(a1790, a1791) {
            let a1792 = f457(v1);
            bool_not(a1792)
        } else {
            let a1793 = v0;
            let a1794 = 2u64;
            if nat_eq(a1793, a1794) {
                f457(v1)
            } else {
                f541(v1)
            }
        }
    }
}

pub fn f443(v0: Adt7, v1: Adt7) -> R<Adt154> {
    f542(v0.clone(), v1.clone(), f543(match v0 {
        Adt7::C0(h1795, _) => {
            h1795
        }
    }), f543(match v1 {
        Adt7::C0(h1796, _) => {
            h1796
        }
    }))
}

pub fn f444(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt145> {
    let m1797 = f213(v0, v2);
    match m1797 {
        Adt137::C0 => {
            Ok(Adt145::C0)
        }
        Adt137::C1(v4) => {
            let m1798 = f544(v4);
            match m1798 {
                None => {
                    Ok(Adt145::C2)
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
                        Ok(Adt145::C2)
                    } else {
                        if v1 {
                            if f541(f545(f64(v3, 0i64)?, v5)) {
                                Ok(Adt145::C1)
                            } else {
                                Ok(Adt145::C0)
                            }
                        } else {
                            if f457(f545(f64(v3, 0i64)?, v5)) {
                                Ok(Adt145::C1)
                            } else {
                                Ok(Adt145::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f445(v0: Str) -> R<u64> {
    f546(f100({
        let a1802 = v0;
        utf8_encode(a1802)
    })?, 0u64)
}

pub fn f446(p1807: List<Adt6>, p1808: u64) -> R<u64> {
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

pub fn f447(p1813: List<(Str, Adt6)>, p1814: u64) -> R<u64> {
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

pub fn f448(p1818: List<Adt6>, v1: Adt5) -> Adt145 {
    let m1817 = p1818;
    {
        let mut m1817 = m1817;
        loop {
            let step = {
                let v0 = m1817;
                let m1815 = v0;
                match m1815.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt145>(Adt145::C0)
                    }
                    Some((v2, v3)) => {
                        let m1816 = f214(v2);
                        match m1816 {
                            None => {
                                Err::<List<Adt6>, Adt145>(Adt145::C2)
                            }
                            Some(v4) => {
                                if f264(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt145>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt145>(Adt145::C1)
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

pub fn f449(p1825: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt107>, p1826: List<Adt109>) -> List<Adt109> {
    let m1824 = (p1825, p1826);
    {
        let mut m1824 = m1824;
        loop {
            let step = {
                let (v0, v3) = m1824;
                let m1819 = v0;
                match m1819.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1821 = f213({
                            let (h1820, _) = v4.clone();
                            h1820
                        }, v1.clone());
                        match m1821 {
                            Adt137::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, v3))
                            }
                            Adt137::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0({
                                    let (_, h1822) = v4;
                                    h1822
                                }, v6, List::cons(Adt107::C0({
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

pub fn f450(p1831: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
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
                        let m1829 = f213({
                            let (h1828, _) = v2;
                            h1828
                        }, v1.clone());
                        match m1829 {
                            Adt137::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt137::C1(_) => {
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

pub fn f451(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1832 = f213(Str::lit("properties"), v0);
    match m1832 {
        Adt137::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt137::C1(v1) => {
            let m1833 = f217(v1);
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

pub fn f452(p1841: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>, p1842: List<Adt109>) -> List<Adt109> {
    let m1840 = (p1841, p1842);
    {
        let mut m1840 = m1840;
        loop {
            let step = {
                let (v0, v4) = m1840;
                let m1835 = v0;
                match m1835.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1837 = f213({
                            let (h1836, _) = v5.clone();
                            h1836
                        }, v1.clone());
                        match m1837 {
                            Adt137::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v6, List::cons(Adt109::C0(v2.clone(), {
                                    let (_, h1838) = v5;
                                    h1838
                                }, List::cons(Adt107::C0({
                                    let (h1839, _) = v5;
                                    h1839
                                }), v3.clone())), v4)))
                            }
                            Adt137::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v6, v4))
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

pub fn f453(p1847: List<Adt6>, p1848: u64, v2: Adt6, v3: List<Adt107>, p1849: List<Adt109>) -> R<List<Adt109>> {
    let m1846 = (p1847, (p1848, p1849));
    {
        let mut m1846 = m1846;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1846;
                let m1843 = v0;
                match m1843.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt109>)), R<List<Adt109>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt109>)), R<List<Adt109>>>((v6, ({
                            let a1844 = v1;
                            let a1845 = 1u64;
                            nat_add(a1844, a1845)?
                        }, List::cons(Adt109::C0(v2.clone(), v5, List::cons(Adt107::C1(v1), v3.clone())), v4))))
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

pub fn f454(p1852: List<Adt6>, v1: Adt6, v2: List<Adt107>, p1853: List<Adt109>) -> List<Adt109> {
    let m1851 = (p1852, p1853);
    {
        let mut m1851 = m1851;
        loop {
            let step = {
                let (v0, v3) = m1851;
                let m1850 = v0;
                match m1850.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0(v4, v1.clone(), v2.clone()), v3)))
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

pub fn f455(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1854 = f547(v0)?;
    match m1854 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1855 = f547(v1)?;
            match m1855 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1856 = v2;
                    let a1857 = 61u64;
                    if nat_eq(a1856, a1857) {
                        f548(v5, v6, v4, v3)
                    } else {
                        let m1858 = f547(v2)?;
                        match m1858 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1859 = v3;
                                let a1860 = 61u64;
                                if nat_eq(a1859, a1860) {
                                    f549(v5, v6, v7, v4)
                                } else {
                                    let m1861 = f547(v3)?;
                                    match m1861 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f550(v5, v6, v7, v8)?))
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

pub fn f456(v0: List<u64>) -> bool {
    let m1862 = v0;
    match m1862.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f457(v0: Adt154) -> bool {
    let m1863 = v0;
    match m1863 {
        Adt154::C0 => {
            false
        }
        Adt154::C1 => {
            false
        }
        Adt154::C2 => {
            true
        }
    }
}

pub fn f458() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f459() -> i64 {
    999999999999999999i64
}

pub fn f460(v0: i64) -> R<List<u64>> {
    let m1866 = {
        let a1864 = v0;
        let a1865 = 0i64;
        compare(a1864, a1865)
    };
    if match m1866 {
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
        Ok(List::cons(45u64, f551(20u64, {
            let a1867 = v0;
            int_neg(a1867)?
        }, List::<u64>::nil())?))
    } else {
        f551(20u64, v0, List::<u64>::nil())
    }
}

pub fn f461(v0: u64) -> R<List<u64>> {
    let a1868 = v0;
    let a1869 = 34u64;
    if nat_eq(a1868, a1869) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1870 = v0;
        let a1871 = 92u64;
        if nat_eq(a1870, a1871) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1872 = v0;
            let a1873 = 8u64;
            if nat_eq(a1872, a1873) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1874 = v0;
                let a1875 = 9u64;
                if nat_eq(a1874, a1875) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1876 = v0;
                    let a1877 = 10u64;
                    if nat_eq(a1876, a1877) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1878 = v0;
                        let a1879 = 12u64;
                        if nat_eq(a1878, a1879) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1880 = v0;
                            let a1881 = 13u64;
                            if nat_eq(a1880, a1881) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1882 = v0;
                                let a1883 = 32u64;
                                if nat_lt(a1882, a1883) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f552(f521(v0, 16u64))?, List::cons(f552(f520(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f462(v0: List<Str>) -> Adt6 {
    Adt6::C4(f411(v0))
}

pub fn f463(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1901 = f505(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1884, _, _, _, _, _, _) => {
                h1884
            }
        });
        let a1902 = {
            let a1899 = f505(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1885, _, _, _, _, _) => {
                    h1885
                }
            });
            let a1900 = {
                let a1897 = f505(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1886, _, _, _, _) => {
                        h1886
                    }
                });
                let a1898 = {
                    let a1895 = f553(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1887, _, _, _) => {
                            h1887
                        }
                    });
                    let a1896 = {
                        let a1893 = f554(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1888, _, _) => {
                                h1888
                            }
                        });
                        let a1894 = {
                            let a1891 = f553(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1889, _) => {
                                    h1889
                                }
                            });
                            let a1892 = f505(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1890) => {
                                    h1890
                                }
                            });
                            append_list(a1891, a1892)
                        };
                        append_list(a1893, a1894)
                    };
                    append_list(a1895, a1896)
                };
                append_list(a1897, a1898)
            };
            append_list(a1899, a1900)
        };
        append_list(a1901, a1902)
    })
}

pub fn f464(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1903 = v1;
    match m1903 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f465(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f555(match v0.clone() {
        Adt10::C0(h1904, _) => {
            h1904
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f556(match v0 {
        Adt10::C0(_, h1905) => {
            h1905
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f466(v0: List<Adt35>) -> List<Adt6> {
    f326(f557(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f467(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1906 = f558(v0, List::<Adt6>::nil())?;
    match m1906 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f326(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f468(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1907 = v1;
    match m1907 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f469(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1908 = v1;
    match m1908 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f470(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f326(f559(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f471(v0: List<Adt46>) -> List<Adt6> {
    f326(f560(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f472(v0: List<Adt54>) -> List<Adt6> {
    f326(f561(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f473(v0: List<Adt56>) -> List<Adt6> {
    f326(f562(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f474(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m1909 = f563(v0, List::<Adt6>::nil())?;
    match m1909 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f326(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f475(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1910 = v1;
    match m1910 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f555(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f476(p1916: List<(Str, Adt64)>, p1917: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m1915 = (p1916, p1917);
    {
        let mut m1915 = m1915;
        loop {
            let step = {
                let (v0, v1) = m1915;
                let m1911 = v0;
                match m1911.uncons() {
                    None => {
                        Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1913 = f564({
                            let (_, h1912) = v2;
                            h1912
                        })?;
                        match m1913 {
                            None => {
                                Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h1914, _) = v2;
                                    h1914
                                }, v4), v1)))
                            }
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

pub fn f477(v0: Adt81) -> List<(Str, Adt64)> {
    let m1918 = v0;
    match m1918 {
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

pub fn f478(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1919 = v0.clone();
    match m1919.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f328(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f479(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1920 = v0;
    match m1920 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f480(v0: Adt81) -> Option<Str> {
    let m1921 = v0;
    match m1921 {
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

pub fn f481(v0: Adt62) -> Option<List<(Str, Adt6)>> {
    let m1922 = v0;
    match m1922 {
        Adt62::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt62::C1(_, v3) => {
            let m1923 = f565(v3);
            match m1923 {
                None => {
                    None::<List<(Str, Adt6)>>
                }
                Some(v4) => {
                    Some(List::cons((Str::lit("inputRequests"), v4), List::<(Str, Adt6)>::nil()))
                }
            }
        }
        Adt62::C2(_, v6) => {
            Some(List::cons((Str::lit("result"), Adt6::C5(match v6 {
                Adt5::C0(h1924) => {
                    h1924
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt62::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f145(v8)), List::<(Str, Adt6)>::nil()))
        }
        Adt62::C4(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f482(v0: Adt63, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt63::C0(h1925, _, _, _, _, _) => {
            h1925
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a1931 = f566(Str::lit("statusMessage"), match v0.clone() {
            Adt63::C0(_, h1926, _, _, _, _) => {
                h1926
            }
        });
        let a1932 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt63::C0(_, _, h1927, _, _, _) => {
                h1927
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt63::C0(_, _, _, h1928, _, _) => {
                h1928
            }
        })), List::cons((Str::lit("ttlMs"), f567(match v0.clone() {
            Adt63::C0(_, _, _, _, h1929, _) => {
                h1929
            }
        })), f568(Str::lit("pollIntervalMs"), match v0 {
            Adt63::C0(_, _, _, _, _, h1930) => {
                h1930
            }
        }))));
        append_list(a1931, a1932)
    }))
}

pub fn f483(v0: Adt62) -> Adt63 {
    let m1933 = v0;
    match m1933 {
        Adt62::C0(v1) => {
            v1
        }
        Adt62::C1(v2, _) => {
            v2
        }
        Adt62::C2(v4, _) => {
            v4
        }
        Adt62::C3(v6, _) => {
            v6
        }
        Adt62::C4(v8) => {
            v8
        }
    }
}

pub fn f484(v0: Adt83) -> Str {
    let m1934 = v0;
    match m1934 {
        Adt83::C0 => {
            Str::lit("working")
        }
        Adt83::C1 => {
            Str::lit("input_required")
        }
        Adt83::C2 => {
            Str::lit("completed")
        }
        Adt83::C3 => {
            Str::lit("failed")
        }
        Adt83::C4 => {
            Str::lit("cancelled")
        }
    }
}

pub fn f485(v0: Adt62) -> Adt83 {
    let m1935 = v0;
    match m1935 {
        Adt62::C0(_) => {
            Adt83::C0
        }
        Adt62::C1(_, _) => {
            Adt83::C1
        }
        Adt62::C2(_, _) => {
            Adt83::C2
        }
        Adt62::C3(_, _) => {
            Adt83::C3
        }
        Adt62::C4(_) => {
            Adt83::C4
        }
    }
}

pub fn f486() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f487(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1936, _, _, _, _, _) => {
            h1936
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1937, _, _, _) => {
            h1937
        }
    })), {
        let a1946 = f464(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1938, _, _, _, _) => {
                h1938
            }
        });
        let a1947 = {
            let a1944 = f464(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1939, _, _) => {
                    h1939
                }
            });
            let a1945 = {
                let a1942 = f464(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1940, _) => {
                        h1940
                    }
                });
                let a1943 = f569(match v0 {
                    Adt1::C0(_, _, _, _, _, h1941) => {
                        h1941
                    }
                });
                append_list(a1942, a1943)
            };
            append_list(a1944, a1945)
        };
        append_list(a1946, a1947)
    })))
}

pub fn f488(p1951: List<Adt6>, p1952: List<Adt84>) -> Option<List<Adt84>> {
    let m1950 = (p1951, p1952);
    {
        let mut m1950 = m1950;
        loop {
            let step = {
                let (v0, v1) = m1950;
                let m1948 = v0;
                match m1948.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt84>), Option<List<Adt84>>>(Some(f490(v1, List::<Adt84>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1949 = f570(v2);
                        match m1949 {
                            None => {
                                Err::<(List<Adt6>, List<Adt84>), Option<List<Adt84>>>(None::<List<Adt84>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt6>, List<Adt84>), Option<List<Adt84>>>((v3, List::cons(v4, v1)))
                            }
                        }
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

pub fn f489(v0: List<Adt84>) -> Option<Adt6> {
    let m1953 = f571(v0, List::<Adt6>::nil());
    match m1953 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f326(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f490(p1956: List<Adt84>, p1957: List<Adt84>) -> List<Adt84> {
    let m1955 = (p1956, p1957);
    {
        let mut m1955 = m1955;
        loop {
            let step = {
                let (v0, v1) = m1955;
                let m1954 = v0;
                match m1954.uncons() {
                    None => {
                        Err::<(List<Adt84>, List<Adt84>), List<Adt84>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt84>, List<Adt84>), List<Adt84>>((v3, List::cons(v2, v1)))
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

pub fn f491(v0: Str, p1962: List<Adt84>, v2: i64, p1963: List<Adt84>) -> R<List<Adt84>> {
    let m1961 = (p1962, p1963);
    {
        let mut m1961 = m1961;
        loop {
            let step = {
                let (v1, v3) = m1961;
                let m1958 = v1;
                match m1958.uncons() {
                    None => {
                        Err::<(List<Adt84>, List<Adt84>), R<List<Adt84>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a1959 = f492(v4.clone());
                        let a1960 = v0.clone();
                        if if equal(a1959, a1960) {
                            true
                        } else {
                            f572(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt84>, List<Adt84>), R<List<Adt84>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt84>, List<Adt84>), R<List<Adt84>>>((v5, List::cons(v4, v3)))
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

pub fn f492(v0: Adt84) -> Str {
    let m1966 = f483(match v0 {
        Adt84::C0(h1965, _, _) => {
            h1965
        }
    });
    match m1966 {
        Adt63::C0(h1964, _, _, _, _, _) => {
            h1964
        }
    }
}

pub fn f493(v0: List<List<u64>>) -> bool {
    let m1967 = v0;
    match m1967.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f494() -> List<Adt93> {
    List::cons(Adt93::C0(f573(), f414()), List::cons(Adt93::C0(f574(), f575()), List::cons(Adt93::C0(f576(), f577()), List::<Adt93>::nil())))
}

pub fn f495(v0: List<List<u64>>) -> List<u64> {
    f578(f51(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f496(p1982: u64, p1983: Adt151, p1984: List<Adt152>) -> R<Option<Adt6>> {
    let m1981 = (p1982, (p1983, p1984));
    {
        let mut m1981 = m1981;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1981;
                let m1968 = v0;
                if m1968 == 0 {
                    Err::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1968 - 1;
                    let m1969 = v1;
                    match m1969 {
                        Adt151::C0(v4) => {
                            let m1970 = v4.clone();
                            match m1970 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f579(v7)?;
                                    Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1971 = v9;
                                    match m1971.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C0(v10), List::cons(Adt152::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1972 = v12;
                                    match m1972.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f579({
                                                let (h1973, _) = v13;
                                                h1973
                                            })?;
                                            Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C0({
                                                let (_, h1974) = v13;
                                                h1974
                                            }), List::cons(Adt152::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt151::C1(v16) => {
                            let m1975 = v2;
                            match m1975.uncons() {
                                None => {
                                    Err::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1976 = v17;
                                    match m1976 {
                                        Adt152::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1977 = v20;
                                            match m1977.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C1(Adt6::C4(f326(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C0(v22), List::cons(Adt152::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt152::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1978 = v26;
                                            match m1978.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C1(Adt6::C5(f328(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f579({
                                                        let (h1979, _) = v28;
                                                        h1979
                                                    })?;
                                                    Ok::<(u64, (Adt151, List<Adt152>)), R<Option<Adt6>>>((v3, (Adt151::C0({
                                                        let (_, h1980) = v28;
                                                        h1980
                                                    }), List::cons(Adt152::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m1981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497() -> u64 {
    67108864u64
}

pub fn f498(v0: Adt19) -> Adt6 {
    let m1985 = v0;
    match m1985 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f499(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1986 = v0;
    match m1986 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f500(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1987 = v0;
    match m1987 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f501() -> i64 {
    -32603i64
}

pub fn f502() -> i64 {
    429i64
}

pub fn f503() -> i64 {
    -32021i64
}

pub fn f504(p1990: List<Str>, p1991: List<Adt6>) -> List<Adt6> {
    let m1989 = (p1990, p1991);
    {
        let mut m1989 = m1989;
        loop {
            let step = {
                let (v0, v1) = m1989;
                let m1988 = v0;
                match m1988.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1989 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1992 = v1;
    match m1992 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1993) => {
                    h1993
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f506(v0: List<u64>) -> R<bool> {
    let m1994 = v0;
    match m1994.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1995 = v2;
            match m1995.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1996 = f207(v1)?;
                    let a1997 = 113u64;
                    if if nat_eq(a1996, a1997) {
                        let a1998 = v3;
                        let a1999 = 61u64;
                        nat_eq(a1998, a1999)
                    } else {
                        false
                    } {
                        Ok(f580(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f507(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt134>) -> R<Result<Adt130, Adt133>> {
    let m2000 = v3;
    match m2000 {
        Ok(v5) => {
            f581(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt130, Adt133>(Adt133::C0(v4)))
        }
    }
}

pub fn f508(v0: Adt141, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt134> {
    let m2001 = v0;
    match m2001 {
        Adt141::C0 => {
            f582(v1, v2)
        }
        Adt141::C1 => {
            f583(v1, v2)
        }
        Adt141::C2 => {
            f584(v1, v2)
        }
        Adt141::C3 => {
            f585(v1, v2)
        }
        Adt141::C4 => {
            f586(v1, v2)
        }
        Adt141::C5 => {
            f587(v1, v2)
        }
        Adt141::C6 => {
            f588(v1, v2)
        }
        Adt141::C7 => {
            f589(v1, v2)
        }
        Adt141::C8 => {
            f590(v1, v2)
        }
        Adt141::C9 => {
            f591(v1, v2)
        }
        Adt141::C10 => {
            f592(v1, v2)
        }
        Adt141::C11 => {
            f593(v1, v2)
        }
    }
}

pub fn f509(v0: Adt141) -> Str {
    let m2002 = v0;
    match m2002 {
        Adt141::C0 => {
            Str::lit("server/discover")
        }
        Adt141::C1 => {
            Str::lit("tools/list")
        }
        Adt141::C2 => {
            Str::lit("tools/call")
        }
        Adt141::C3 => {
            Str::lit("resources/list")
        }
        Adt141::C4 => {
            Str::lit("resources/read")
        }
        Adt141::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt141::C6 => {
            Str::lit("prompts/list")
        }
        Adt141::C7 => {
            Str::lit("prompts/get")
        }
        Adt141::C8 => {
            Str::lit("completion/complete")
        }
        Adt141::C9 => {
            Str::lit("tasks/get")
        }
        Adt141::C10 => {
            Str::lit("tasks/update")
        }
        Adt141::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f510(v0: Option<Adt8>) -> bool {
    let m2003 = v0;
    match m2003 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f511(v0: Option<Adt9>) -> bool {
    let m2004 = v0;
    match m2004 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f512(v0: Option<Adt5>) -> bool {
    let m2005 = v0;
    match m2005 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f513(v0: Option<Adt5>, v1: Str) -> bool {
    let m2006 = v0;
    match m2006 {
        None => {
            false
        }
        Some(v2) => {
            f264(v1, v2)
        }
    }
}

pub fn f514(v0: List<Str>, v1: Adt111, v2: Adt16, v3: Option<Adt116>) -> Adt147 {
    let m2007 = v3;
    match m2007 {
        None => {
            f594(v1, v2.clone(), f595(match v2 {
                Adt16::C0(h2008, _, _, _, _, _) => {
                    h2008
                }
            }, v0))
        }
        Some(v4) => {
            Adt147::C1(Adt133::C2(v4))
        }
    }
}

pub fn f515(v0: Option<Adt115>, v1: Adt149, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt116>> {
    let m2009 = v0;
    match m2009 {
        None => {
            Ok(None::<Adt116>)
        }
        Some(v5) => {
            let m2011 = match v5.clone() {
                Adt115::C0(_, _, _, h2010, _) => {
                    h2010
                }
            };
            match m2011 {
                None => {
                    f596(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f516(v0: Option<Adt5>) -> Adt5 {
    let m2012 = v0;
    match m2012 {
        None => {
            f597()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f517(v0: Adt5) -> Result<Adt16, Adt134> {
    let m2013 = f598(v0.clone());
    match m2013 {
        Adt155::C0 => {
            f599(v0.clone(), f199(f600(), v0.clone()), f199(f601(), v0))
        }
        Adt155::C1 => {
            Err::<Adt16, Adt134>(Adt134::C2)
        }
        Adt155::C2 => {
            Err::<Adt16, Adt134>(Adt134::C3)
        }
    }
}

pub fn f518() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f519(v0: Str) -> R<List<u64>> {
    f100({
        let a2014 = v0;
        utf8_encode(a2014)
    })
}

pub fn f520(v0: u64, v1: u64) -> u64 {
    let a2015 = v0;
    let a2016 = v1;
    let a2017 = 0u64;
    nat_rem(a2015, a2016, a2017)
}

pub fn f521(v0: u64, v1: u64) -> u64 {
    let a2018 = v0;
    let a2019 = v1;
    let a2020 = 0u64;
    nat_quot(a2018, a2019, a2020)
}

pub fn f522(v0: Adt6) -> bool {
    let m2021 = f214(v0);
    match m2021 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f523(v0: Adt6) -> bool {
    let m2022 = f441(v0);
    match m2022 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f524(v0: Adt6) -> bool {
    let m2023 = f544(v0);
    match m2023 {
        None => {
            false
        }
        Some(v1) => {
            let a2027 = {
                let m2026 = {
                    let a2024 = v1;
                    let a2025 = 0i64;
                    compare(a2024, a2025)
                };
                match m2026 {
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
            bool_not(a2027)
        }
    }
}

pub fn f525(v0: Adt6) -> bool {
    let m2028 = f288(v0);
    match m2028 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f526(v0: Adt6) -> bool {
    let m2029 = f217(v0);
    match m2029 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f527(v0: Adt6) -> bool {
    let m2030 = f288(v0);
    match m2030 {
        None => {
            false
        }
        Some(v1) => {
            f602(v1)
        }
    }
}

pub fn f528(v0: Adt6) -> bool {
    let m2031 = f216(v0);
    match m2031 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f529(v0: Adt6) -> bool {
    let m2032 = f214(v0.clone());
    match m2032 {
        None => {
            let m2033 = f288(v0);
            match m2033 {
                None => {
                    false
                }
                Some(v2) => {
                    f603(v2)
                }
            }
        }
        Some(v1) => {
            f436(v1)
        }
    }
}

pub fn f530(v0: Adt6) -> bool {
    let m2034 = f288(v0);
    match m2034 {
        None => {
            false
        }
        Some(v1) => {
            let m2035 = v1;
            match m2035.uncons() {
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

pub fn f531(v0: Str, p2053: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2052 = p2053;
    {
        let mut m2052 = m2052;
        loop {
            let step = {
                let v1 = m2052;
                let m2036 = v1;
                match m2036.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2040 = v0.clone();
                        let a2041 = {
                            let a2038 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2037, _) = v2.clone();
                                h2037
                            }, List::<Str>::nil()));
                            let a2039 = Str::lit("");
                            join(a2038, a2039)
                        };
                        if equal(a2040, a2041) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2042) = v2;
                                h2042
                            }))
                        } else {
                            let m2044 = f217({
                                let (_, h2043) = v2.clone();
                                h2043
                            });
                            match m2044 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2045 = f199(Str::lit("$anchor"), v4);
                                    match m2045 {
                                        Adt137::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt137::C1(v5) => {
                                            let m2046 = f214(v5);
                                            match m2046 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2049 = v0.clone();
                                                    let a2050 = {
                                                        let a2047 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2048 = Str::lit("");
                                                        join(a2047, a2048)
                                                    };
                                                    if equal(a2049, a2050) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2051) = v2;
                                                            h2051
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
                Ok(next) => { m2052 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2054 = f213(Str::lit("type"), v0);
    match m2054 {
        Adt137::C0 => {
            None::<Adt13>
        }
        Adt137::C1(v1) => {
            let m2055 = f214(v1);
            match m2055 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f604(v2)
                }
            }
        }
    }
}

pub fn f533(v0: List<u64>) -> R<List<u64>> {
    Ok(f186(f605(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f534(v0: List<u64>, p2058: List<List<u64>>) -> bool {
    let m2057 = p2058;
    {
        let mut m2057 = m2057;
        loop {
            let step = {
                let v1 = m2057;
                let m2056 = v1;
                match m2056.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f606(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2057 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f535(p2061: List<Str>, p2062: List<Str>) -> List<Str> {
    let m2060 = (p2061, p2062);
    {
        let mut m2060 = m2060;
        loop {
            let step = {
                let (v0, v1) = m2060;
                let m2059 = v0;
                match m2059.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f536(v0: Adt6) -> Str {
    let m2063 = v0;
    match m2063 {
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

pub fn f537(v0: Adt7) -> R<bool> {
    if f426(match v0.clone() {
        Adt7::C0(h2064, _) => {
            h2064
        }
    }) {
        Ok(true)
    } else {
        f607(40u64, match v0.clone() {
            Adt7::C0(h2065, _) => {
                h2065
            }
        }, match v0 {
            Adt7::C0(_, h2066) => {
                h2066
            }
        })
    }
}

pub fn f538(v0: Adt6) -> Str {
    let m2067 = f214(v0);
    match m2067 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f539(p2074: u64, p2075: List<(Adt6, Adt6)>) -> R<Adt153> {
    let m2073 = (p2074, p2075);
    {
        let mut m2073 = m2073;
        loop {
            let step = {
                let (v0, v1) = m2073;
                let m2068 = v0;
                if m2068 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt153>>(Ok(Adt153::C2))
                } else {
                    let v2 = m2068 - 1;
                    let m2069 = v1;
                    match m2069.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt153>>(Ok(Adt153::C0))
                        }
                        Some((v3, v4)) => {
                            let m2072 = f608({
                                let (h2070, _) = v3;
                                h2070
                            }, {
                                let (_, h2071) = v3;
                                h2071
                            }, v4)?;
                            match m2072 {
                                Adt156::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt153>>(Ok(Adt153::C1))
                                }
                                Adt156::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt153>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2073 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540() -> u64 {
    67108864u64
}

pub fn f541(v0: Adt154) -> bool {
    let m2076 = v0;
    match m2076 {
        Adt154::C0 => {
            true
        }
        Adt154::C1 => {
            false
        }
        Adt154::C2 => {
            false
        }
    }
}

pub fn f542(v0: Adt7, v1: Adt7, v2: Adt154, v3: Adt154) -> R<Adt154> {
    let m2077 = v2;
    match m2077 {
        Adt154::C0 => {
            let m2079 = v3;
            match m2079 {
                Adt154::C0 => {
                    Ok(f609(Adt154::C0, f610(f611(match v0.clone() {
                        Adt7::C0(h2080, _) => {
                            h2080
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2081) => {
                            h2081
                        }
                    }, f611(match v1.clone() {
                        Adt7::C0(h2082, _) => {
                            h2082
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2083) => {
                            h2083
                        }
                    })?))
                }
                Adt154::C1 => {
                    Ok(Adt154::C0)
                }
                Adt154::C2 => {
                    Ok(Adt154::C0)
                }
            }
        }
        Adt154::C1 => {
            let m2078 = v3;
            match m2078 {
                Adt154::C0 => {
                    Ok(Adt154::C2)
                }
                Adt154::C1 => {
                    Ok(Adt154::C1)
                }
                Adt154::C2 => {
                    Ok(Adt154::C0)
                }
            }
        }
        Adt154::C2 => {
            let m2084 = v3;
            match m2084 {
                Adt154::C0 => {
                    Ok(Adt154::C2)
                }
                Adt154::C1 => {
                    Ok(Adt154::C2)
                }
                Adt154::C2 => {
                    f610(match v0.clone() {
                        Adt7::C0(h2085, _) => {
                            h2085
                        }
                    }, match v0 {
                        Adt7::C0(_, h2086) => {
                            h2086
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2087, _) => {
                            h2087
                        }
                    }, match v1 {
                        Adt7::C0(_, h2088) => {
                            h2088
                        }
                    })
                }
            }
        }
    }
}

pub fn f543(v0: i64) -> Adt154 {
    f545(v0, 0i64)
}

pub fn f544(v0: Adt6) -> Option<i64> {
    let m2089 = v0;
    match m2089 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f426(match v2.clone() {
                Adt7::C0(_, h2090) => {
                    h2090
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2091, _) => {
                        h2091
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

pub fn f545(v0: i64, v1: i64) -> Adt154 {
    let m2094 = {
        let a2092 = v0;
        let a2093 = v1;
        compare(a2092, a2093)
    };
    if match m2094 {
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
        Adt154::C0
    } else {
        let m2097 = {
            let a2095 = v1;
            let a2096 = v0;
            compare(a2095, a2096)
        };
        if match m2097 {
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
            Adt154::C2
        } else {
            Adt154::C1
        }
    }
}

pub fn f546(p2106: List<u64>, p2107: u64) -> R<u64> {
    let m2105 = (p2106, p2107);
    {
        let mut m2105 = m2105;
        loop {
            let step = {
                let (v0, v1) = m2105;
                let m2098 = v0;
                match m2098.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2099 = v2;
                        let a2100 = 128u64;
                        if if nat_lt(a2099, a2100) {
                            true
                        } else {
                            let a2101 = 191u64;
                            let a2102 = v2;
                            nat_lt(a2101, a2102)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2103 = v1;
                                let a2104 = 1u64;
                                nat_add(a2103, a2104)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(v0: u64) -> R<Option<u64>> {
    let a2108 = 64u64;
    let a2109 = v0;
    if if nat_lt(a2108, a2109) {
        let a2110 = v0;
        let a2111 = 91u64;
        nat_lt(a2110, a2111)
    } else {
        false
    } {
        Ok(Some({
            let a2112 = v0;
            let a2113 = 65u64;
            nat_sub(a2112, a2113)
        }))
    } else {
        let a2114 = 96u64;
        let a2115 = v0;
        if if nat_lt(a2114, a2115) {
            let a2116 = v0;
            let a2117 = 123u64;
            nat_lt(a2116, a2117)
        } else {
            false
        } {
            Ok(Some({
                let a2118 = v0;
                let a2119 = 71u64;
                nat_sub(a2118, a2119)
            }))
        } else {
            let a2120 = 47u64;
            let a2121 = v0;
            if if nat_lt(a2120, a2121) {
                let a2122 = v0;
                let a2123 = 58u64;
                nat_lt(a2122, a2123)
            } else {
                false
            } {
                Ok(Some({
                    let a2124 = v0;
                    let a2125 = 4u64;
                    nat_add(a2124, a2125)?
                }))
            } else {
                let a2126 = v0;
                let a2127 = 43u64;
                if nat_eq(a2126, a2127) {
                    Ok(Some(62u64))
                } else {
                    let a2128 = v0;
                    let a2129 = 47u64;
                    if nat_eq(a2128, a2129) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f548(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2130 = v3;
        let a2131 = 61u64;
        if nat_eq(a2130, a2131) {
            let a2132 = f612(v1, 16u64);
            let a2133 = 0u64;
            nat_eq(a2132, a2133)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f613(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f549(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2134 = f612(v2, 4u64);
        let a2135 = 0u64;
        nat_eq(a2134, a2135)
    } else {
        false
    } {
        Ok(Some(f614(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f550(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2138 = {
            let a2136 = v0;
            let a2137 = 4u64;
            nat_mul(a2136, a2137)?
        };
        let a2139 = f615(v1, 16u64);
        nat_add(a2138, a2139)?
    }, List::cons({
        let a2142 = {
            let a2140 = f612(v1, 16u64);
            let a2141 = 16u64;
            nat_mul(a2140, a2141)?
        };
        let a2143 = f615(v2, 4u64);
        nat_add(a2142, a2143)?
    }, List::cons({
        let a2146 = {
            let a2144 = f612(v2, 4u64);
            let a2145 = 64u64;
            nat_mul(a2144, a2145)?
        };
        let a2147 = v3;
        nat_add(a2146, a2147)?
    }, List::<u64>::nil()))))
}

pub fn f551(p2159: u64, p2160: i64, p2161: List<u64>) -> R<List<u64>> {
    let m2158 = (p2159, (p2160, p2161));
    {
        let mut m2158 = m2158;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2158;
                let m2148 = v0;
                if m2148 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2148 - 1;
                    let v4: i64 = {
                        let a2149 = v1;
                        let a2150 = 10i64;
                        let a2151 = 0i64;
                        int_quot(a2149, a2150, a2151)?
                    };
                    let v5: i64 = {
                        let a2152 = v1;
                        let a2153 = 10i64;
                        let a2154 = 0i64;
                        int_rem(a2152, a2153, a2154)
                    };
                    let m2157 = {
                        let a2155 = v4;
                        let a2156 = 1i64;
                        compare(a2155, a2156)
                    };
                    if match m2157 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f616(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f616(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552(v0: u64) -> R<u64> {
    let a2162 = v0;
    let a2163 = 10u64;
    if nat_lt(a2162, a2163) {
        let a2164 = 48u64;
        let a2165 = v0;
        nat_add(a2164, a2165)
    } else {
        let a2166 = 87u64;
        let a2167 = v0;
        nat_add(a2166, a2167)
    }
}

pub fn f553(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2168 = v1;
    match m2168 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f617(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f554(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2169 = v0;
    match m2169 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2172 = f469(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2170, _) => {
                        h2170
                    }
                });
                let a2173 = f469(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2171) => {
                        h2171
                    }
                });
                append_list(a2172, a2173)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f555(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f618(v0, 0i64)?, 0i64)))
}

pub fn f556(v0: Adt11) -> Str {
    let m2174 = v0;
    match m2174 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f557(p2177: List<Adt35>, p2178: List<Adt6>) -> List<Adt6> {
    let m2176 = (p2177, p2178);
    {
        let mut m2176 = m2176;
        loop {
            let step = {
                let (v0, v1) = m2176;
                let m2175 = v0;
                match m2175.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f619(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f558(p2182: List<Adt38>, p2183: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2181 = (p2182, p2183);
    {
        let mut m2181 = m2181;
        loop {
            let step = {
                let (v0, v1) = m2181;
                let m2179 = v0;
                match m2179.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2180 = f620(v2)?;
                        match m2180 {
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
                Ok(next) => { m2181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(p2186: List<Adt44>, p2187: List<Adt6>) -> R<List<Adt6>> {
    let m2185 = (p2186, p2187);
    {
        let mut m2185 = m2185;
        loop {
            let step = {
                let (v0, v1) = m2185;
                let m2184 = v0;
                match m2184.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f621(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f560(p2190: List<Adt46>, p2191: List<Adt6>) -> List<Adt6> {
    let m2189 = (p2190, p2191);
    {
        let mut m2189 = m2189;
        loop {
            let step = {
                let (v0, v1) = m2189;
                let m2188 = v0;
                match m2188.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f622(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2189 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(p2194: List<Adt54>, p2195: List<Adt6>) -> List<Adt6> {
    let m2193 = (p2194, p2195);
    {
        let mut m2193 = m2193;
        loop {
            let step = {
                let (v0, v1) = m2193;
                let m2192 = v0;
                match m2192.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f623(v2), v1)))
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

pub fn f562(p2198: List<Adt56>, p2199: List<Adt6>) -> List<Adt6> {
    let m2197 = (p2198, p2199);
    {
        let mut m2197 = m2197;
        loop {
            let step = {
                let (v0, v1) = m2197;
                let m2196 = v0;
                match m2196.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f624(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563(p2203: List<Adt59>, p2204: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2202 = (p2203, p2204);
    {
        let mut m2202 = m2202;
        loop {
            let step = {
                let (v0, v1) = m2202;
                let m2200 = v0;
                match m2200.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2201 = f625(v2)?;
                        match m2201 {
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
                Ok(next) => { m2202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f564(v0: Adt64) -> R<Option<Adt6>> {
    let m2205 = v0;
    match m2205 {
        Adt64::C0(v1) => {
            Ok(Some(f626(Str::lit("elicitation/create"), Some(f627(v1)))))
        }
        Adt64::C1(v2) => {
            let m2206 = f628(v2)?;
            match m2206 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f626(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt64::C2(v4) => {
            Ok(Some(f626(Str::lit("roots/list"), {
                let m2207 = v4;
                match m2207 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h2208) => {
                                h2208
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f565(v0: List<(Str, Adt64)>) -> Option<Adt6> {
    let m2209 = f629(v0, List::<(Str, Adt6)>::nil());
    match m2209 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f328(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f566(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2210 = v1;
    match m2210 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f567(v0: Option<i64>) -> Adt6 {
    let m2211 = v0;
    match m2211 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f630(v1)
        }
    }
}

pub fn f568(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m2212 = v1;
    match m2212 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f630(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f569(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2213 = v0;
    match m2213 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f631(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f570(v0: Adt6) -> Option<Adt84> {
    let m2214 = f217(v0);
    match m2214 {
        None => {
            None::<Adt84>
        }
        Some(v1) => {
            let m2215 = f632(Str::lit("task"), v1.clone());
            match m2215 {
                None => {
                    None::<Adt84>
                }
                Some(v2) => {
                    let m2216 = f633(v2);
                    match m2216 {
                        None => {
                            None::<Adt84>
                        }
                        Some(v3) => {
                            let m2217 = f634(Str::lit("createdMs"), v1.clone());
                            match m2217 {
                                None => {
                                    None::<Adt84>
                                }
                                Some(v4) => {
                                    let m2218 = f632(Str::lit("plan"), v1);
                                    match m2218 {
                                        None => {
                                            None::<Adt84>
                                        }
                                        Some(v5) => {
                                            let m2219 = f635(v5);
                                            match m2219 {
                                                None => {
                                                    None::<Adt84>
                                                }
                                                Some(v6) => {
                                                    Some(Adt84::C0(v3, v4, v6))
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

pub fn f571(p2223: List<Adt84>, p2224: List<Adt6>) -> Option<List<Adt6>> {
    let m2222 = (p2223, p2224);
    {
        let mut m2222 = m2222;
        loop {
            let step = {
                let (v0, v1) = m2222;
                let m2220 = v0;
                match m2220.uncons() {
                    None => {
                        Err::<(List<Adt84>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2221 = f636(v2);
                        match m2221 {
                            None => {
                                Err::<(List<Adt84>, List<Adt6>), Option<List<Adt6>>>(None::<List<Adt6>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt84>, List<Adt6>), Option<List<Adt6>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: Adt84, v1: i64) -> R<bool> {
    let m2228 = {
        let m2227 = f483(match v0.clone() {
            Adt84::C0(h2226, _, _) => {
                h2226
            }
        });
        match m2227 {
            Adt63::C0(_, _, _, _, h2225, _) => {
                h2225
            }
        }
    };
    match m2228 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a2235 = {
                let m2234 = {
                    let a2232 = {
                        let a2230 = v1;
                        let a2231 = match v0 {
                            Adt84::C0(_, h2229, _) => {
                                h2229
                            }
                        };
                        int_sub(a2230, a2231)?
                    };
                    let a2233 = v2;
                    compare(a2232, a2233)
                };
                match m2234 {
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
            Ok(bool_not(a2235))
        }
    }
}

pub fn f573() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f574() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f575() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f576() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f577() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f578(p2240: List<List<u64>>, p2241: List<u64>) -> List<u64> {
    let m2239 = (p2240, p2241);
    {
        let mut m2239 = m2239;
        loop {
            let step = {
                let (v0, v1) = m2239;
                let m2236 = v0;
                match m2236.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2237 = f637(v2);
                            let a2238 = v1;
                            append_list(a2237, a2238)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: Str) -> R<Str> {
    let m2244 = {
        let a2243 = f234(f102(), f186(f638(f100({
            let a2242 = v0;
            utf8_encode(a2242)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2243)
    };
    match m2244 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f580(v0: List<u64>) -> bool {
    let m2245 = v0;
    match m2245.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2246 = v1;
            let a2247 = 48u64;
            if nat_eq(a2246, a2247) {
                f639(v2)
            } else {
                false
            }
        }
    }
}

pub fn f581(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt130, Adt133>> {
    if f640(f641(v3.clone())) {
        Ok(Ok::<Adt130, Adt133>(f642(v0.clone(), Adt32::C0(f643(v0)))))
    } else {
        Ok(f644(v0.clone(), v3.clone(), f645(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2248, _) => {
                h2248
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2249, _, _) => {
                h2249
            }
        }, v2, v1, v3)?))
    }
}

pub fn f582(v0: Adt16, _: Adt5) -> Result<Adt14, Adt134> {
    Ok::<Adt14, Adt134>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f583(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2250 = f646(Str::lit("cursor"), v1);
    match m2250 {
        Ok(v2) => {
            Ok::<Adt14, Adt134>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt134>(v3)
        }
    }
}

pub fn f584(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2251 = f647(Str::lit("name"), v1.clone());
    match m2251 {
        Ok(v2) => {
            let m2252 = f648(Str::lit("arguments"), v1.clone());
            match m2252 {
                Ok(v3) => {
                    let m2253 = f648(Str::lit("inputResponses"), v1.clone());
                    match m2253 {
                        Ok(v4) => {
                            let m2254 = f646(Str::lit("requestState"), v1);
                            match m2254 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt134>(Adt14::C2(Adt21::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt134>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt134>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt134>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt134>(v9)
        }
    }
}

pub fn f585(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2255 = f646(Str::lit("cursor"), v1);
    match m2255 {
        Ok(v2) => {
            Ok::<Adt14, Adt134>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt134>(v3)
        }
    }
}

pub fn f586(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2256 = f647(Str::lit("uri"), v1.clone());
    match m2256 {
        Ok(v2) => {
            let m2257 = f648(Str::lit("inputResponses"), v1.clone());
            match m2257 {
                Ok(v3) => {
                    let m2258 = f646(Str::lit("requestState"), v1);
                    match m2258 {
                        Ok(v4) => {
                            Ok::<Adt14, Adt134>(Adt14::C4(Adt22::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt14, Adt134>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt14, Adt134>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt14, Adt134>(v7)
        }
    }
}

pub fn f587(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2259 = f646(Str::lit("cursor"), v1);
    match m2259 {
        Ok(v2) => {
            Ok::<Adt14, Adt134>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt134>(v3)
        }
    }
}

pub fn f588(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2260 = f646(Str::lit("cursor"), v1);
    match m2260 {
        Ok(v2) => {
            Ok::<Adt14, Adt134>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt134>(v3)
        }
    }
}

pub fn f589(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2261 = f647(Str::lit("name"), v1.clone());
    match m2261 {
        Ok(v2) => {
            let m2262 = f649(Str::lit("arguments"), v1.clone());
            match m2262 {
                Ok(v3) => {
                    let m2263 = f648(Str::lit("inputResponses"), v1.clone());
                    match m2263 {
                        Ok(v4) => {
                            let m2264 = f646(Str::lit("requestState"), v1);
                            match m2264 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt134>(Adt14::C7(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt134>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt134>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt134>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt134>(v9)
        }
    }
}

pub fn f590(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2265 = f650(Str::lit("ref"), v1.clone());
    match m2265 {
        Ok(v2) => {
            let m2266 = f651(v2);
            match m2266 {
                Ok(v3) => {
                    let m2267 = f650(Str::lit("argument"), v1.clone());
                    match m2267 {
                        Ok(v4) => {
                            let m2268 = f652(v4);
                            match m2268 {
                                Ok(v5) => {
                                    let m2269 = f653(v1);
                                    match m2269 {
                                        Ok(v6) => {
                                            Ok::<Adt14, Adt134>(Adt14::C8(Adt24::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt14, Adt134>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt14, Adt134>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt14, Adt134>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt14, Adt134>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt14, Adt134>(v11)
        }
    }
}

pub fn f591(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2270 = f647(Str::lit("taskId"), v1);
    match m2270 {
        Ok(v2) => {
            Ok::<Adt14, Adt134>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt134>(v3)
        }
    }
}

pub fn f592(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2271 = f647(Str::lit("taskId"), v1.clone());
    match m2271 {
        Ok(v2) => {
            let m2272 = f650(Str::lit("inputResponses"), v1);
            match m2272 {
                Ok(v3) => {
                    Ok::<Adt14, Adt134>(Adt14::C10(Adt30::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt14, Adt134>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt14, Adt134>(v5)
        }
    }
}

pub fn f593(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt134> {
    let m2273 = f647(Str::lit("taskId"), v1);
    match m2273 {
        Ok(v2) => {
            Ok::<Adt14, Adt134>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt134>(v3)
        }
    }
}

pub fn f594(v0: Adt111, v1: Adt16, v2: Option<Adt133>) -> Adt147 {
    let m2274 = v2;
    match m2274 {
        None => {
            Adt147::C0(Adt148::C0(match v0.clone() {
                Adt111::C0(h2275, _, _) => {
                    h2275
                }
            }, match v0.clone() {
                Adt111::C0(_, h2276, _) => {
                    h2276
                }
            }, v1, f516(match v0 {
                Adt111::C0(_, _, h2277) => {
                    h2277
                }
            })))
        }
        Some(v3) => {
            Adt147::C1(v3)
        }
    }
}

pub fn f595(v0: Str, v1: List<Str>) -> Option<Adt133> {
    f654(f655(v0.clone(), v1.clone()), v0, v1)
}

pub fn f596(v0: Adt115, v1: Adt149, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt116>> {
    let m2279 = match v0.clone() {
        Adt115::C0(h2278, _, _, _, _) => {
            h2278
        }
    };
    match m2279 {
        None => {
            Ok(Some(Adt116::C0))
        }
        Some(v5) => {
            let a2280 = v5;
            let a2281 = v3;
            if equal(a2280, a2281) {
                let m2283 = match v0.clone() {
                    Adt115::C0(_, h2282, _, _, _) => {
                        h2282
                    }
                };
                match m2283 {
                    None => {
                        Ok(Some(Adt116::C1))
                    }
                    Some(v6) => {
                        let a2284 = v6;
                        let a2285 = v2;
                        if equal(a2284, a2285) {
                            f656(f657(match v0.clone() {
                                Adt115::C0(_, _, h2286, _, _) => {
                                    h2286
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt115::C0(_, _, _, _, h2287) => {
                                    h2287
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt116::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt116::C3))
            }
        }
    }
}

pub fn f597() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f598(v0: Adt5) -> Adt155 {
    f658(f199(f600(), v0.clone()), f199(f601(), v0))
}

pub fn f599(v0: Adt5, v1: Adt137, v2: Adt137) -> Result<Adt16, Adt134> {
    let m2288 = v1;
    match m2288 {
        Adt137::C0 => {
            Err::<Adt16, Adt134>(Adt134::C2)
        }
        Adt137::C1(v3) => {
            let m2289 = v2;
            match m2289 {
                Adt137::C0 => {
                    Err::<Adt16, Adt134>(Adt134::C3)
                }
                Adt137::C1(v4) => {
                    f659(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f600() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f601() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f602(p2293: List<Adt6>) -> bool {
    let m2292 = p2293;
    {
        let mut m2292 = m2292;
        loop {
            let step = {
                let v0 = m2292;
                let m2290 = v0;
                match m2290.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2291 = f214(v1);
                        match m2291 {
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
                Ok(next) => { m2292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f603(p2297: List<Adt6>) -> bool {
    let m2296 = p2297;
    {
        let mut m2296 = m2296;
        loop {
            let step = {
                let v0 = m2296;
                let m2294 = v0;
                match m2294.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2295 = f214(v1);
                        match m2295 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f436(v3) {
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
                Ok(next) => { m2296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: Str) -> Option<Adt13> {
    let a2298 = v0.clone();
    let a2299 = Str::lit("string");
    if equal(a2298, a2299) {
        Some(Adt13::C0)
    } else {
        let a2300 = v0.clone();
        let a2301 = Str::lit("integer");
        if equal(a2300, a2301) {
            Some(Adt13::C1)
        } else {
            let a2302 = v0;
            let a2303 = Str::lit("boolean");
            if equal(a2302, a2303) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f605(p2312: List<u64>, p2313: List<u64>) -> R<List<u64>> {
    let m2311 = (p2312, p2313);
    {
        let mut m2311 = m2311;
        loop {
            let step = {
                let (v0, v1) = m2311;
                let m2304 = v0;
                match m2304.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2305 = 65u64;
                        let a2306 = v2;
                        if if nat_le(a2305, a2306) {
                            let a2307 = v2;
                            let a2308 = 90u64;
                            nat_le(a2307, a2308)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2309 = v2;
                                let a2310 = 32u64;
                                nat_add(a2309, a2310)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606(p2320: List<u64>, p2321: List<u64>) -> bool {
    let m2319 = (p2320, p2321);
    {
        let mut m2319 = m2319;
        loop {
            let step = {
                let (v0, v1) = m2319;
                let m2314 = v0;
                match m2314.uncons() {
                    None => {
                        let m2315 = v1;
                        match m2315.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2316 = v1;
                        match m2316.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2317 = v4;
                                let a2318 = v6;
                                if nat_eq(a2317, a2318) {
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
                Ok(next) => { m2319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f607(p2333: u64, p2334: i64, p2335: i64) -> R<bool> {
    let m2332 = (p2333, (p2334, p2335));
    {
        let mut m2332 = m2332;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2332;
                let m2322 = v0;
                if m2322 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2322 - 1;
                    let m2325 = {
                        let a2323 = v2;
                        let a2324 = 0i64;
                        compare(a2323, a2324)
                    };
                    if match m2325 {
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
                        if f426({
                            let a2326 = v1;
                            let a2327 = 10i64;
                            let a2328 = 0i64;
                            int_rem(a2326, a2327, a2328)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2329 = v1;
                                let a2330 = 10i64;
                                let a2331 = 0i64;
                                int_quot(a2329, a2330, a2331)?
                            }, f93(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f608(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt156> {
    let a2336 = f660(v0.clone());
    let a2337 = f660(v1.clone());
    if nat_eq(a2336, a2337) {
        let m2338 = v0;
        match m2338 {
            Adt6::C0 => {
                Ok(Adt156::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2339 = f216(v1);
                match m2339 {
                    None => {
                        Ok(Adt156::C0)
                    }
                    Some(v4) => {
                        let a2340 = v3;
                        let a2341 = v4;
                        if equal(a2340, a2341) {
                            Ok(Adt156::C1(v2))
                        } else {
                            Ok(Adt156::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2342 = f441(v1);
                match m2342 {
                    None => {
                        Ok(Adt156::C0)
                    }
                    Some(v6) => {
                        if f661(f443(v5, v6)?) {
                            Ok(Adt156::C1(v2))
                        } else {
                            Ok(Adt156::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2343 = f214(v1);
                match m2343 {
                    None => {
                        Ok(Adt156::C0)
                    }
                    Some(v8) => {
                        let a2344 = v7;
                        let a2345 = v8;
                        if equal(a2344, a2345) {
                            Ok(Adt156::C1(v2))
                        } else {
                            Ok(Adt156::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2346 = f288(v1);
                match m2346 {
                    None => {
                        Ok(Adt156::C0)
                    }
                    Some(v10) => {
                        Ok(f662(f663(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2347 = f217(v1);
                match m2347 {
                    None => {
                        Ok(Adt156::C0)
                    }
                    Some(v12) => {
                        let a2349 = f447(v11.clone(), 0u64)?;
                        let a2350 = f447(match v12.clone() {
                            Adt5::C0(h2348) => {
                                h2348
                            }
                        }, 0u64)?;
                        if nat_eq(a2349, a2350) {
                            Ok(f662(f664(v11, match v12 {
                                Adt5::C0(h2351) => {
                                    h2351
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt156::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt156::C0)
    }
}

pub fn f609(v0: Adt154, v1: Adt154) -> Adt154 {
    let m2352 = v0;
    match m2352 {
        Adt154::C0 => {
            f665(v1)
        }
        Adt154::C1 => {
            Adt154::C1
        }
        Adt154::C2 => {
            v1
        }
    }
}

pub fn f610(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt154> {
    let m2353 = f545(f666(v0, v1)?, f666(v2, v3)?);
    match m2353 {
        Adt154::C0 => {
            Ok(Adt154::C0)
        }
        Adt154::C1 => {
            f667(v0, f668(v0)?, v2, f668(v2)?)
        }
        Adt154::C2 => {
            Ok(Adt154::C2)
        }
    }
}

pub fn f611(v0: i64) -> R<i64> {
    let m2356 = {
        let a2354 = v0;
        let a2355 = 0i64;
        compare(a2354, a2355)
    };
    if match m2356 {
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
        let a2357 = v0;
        int_neg(a2357)
    } else {
        Ok(v0)
    }
}

pub fn f612(v0: u64, v1: u64) -> u64 {
    let a2358 = v0;
    let a2359 = v1;
    let a2360 = 0u64;
    nat_rem(a2358, a2359, a2360)
}

pub fn f613(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2363 = {
            let a2361 = v0;
            let a2362 = 4u64;
            nat_mul(a2361, a2362)?
        };
        let a2364 = f615(v1, 16u64);
        nat_add(a2363, a2364)?
    }, List::<u64>::nil()))
}

pub fn f614(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2367 = {
            let a2365 = v0;
            let a2366 = 4u64;
            nat_mul(a2365, a2366)?
        };
        let a2368 = f615(v1, 16u64);
        nat_add(a2367, a2368)?
    }, List::cons({
        let a2371 = {
            let a2369 = f612(v1, 16u64);
            let a2370 = 16u64;
            nat_mul(a2369, a2370)?
        };
        let a2372 = f615(v2, 4u64);
        nat_add(a2371, a2372)?
    }, List::<u64>::nil())))
}

pub fn f615(v0: u64, v1: u64) -> u64 {
    let a2373 = v0;
    let a2374 = v1;
    let a2375 = 0u64;
    nat_quot(a2373, a2374, a2375)
}

pub fn f616(v0: i64) -> u64 {
    let m2378 = {
        let a2376 = v0;
        let a2377 = 1i64;
        compare(a2376, a2377)
    };
    if match m2378 {
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
        let m2381 = {
            let a2379 = v0;
            let a2380 = 2i64;
            compare(a2379, a2380)
        };
        if match m2381 {
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
            let m2384 = {
                let a2382 = v0;
                let a2383 = 3i64;
                compare(a2382, a2383)
            };
            if match m2384 {
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
                let m2387 = {
                    let a2385 = v0;
                    let a2386 = 4i64;
                    compare(a2385, a2386)
                };
                if match m2387 {
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
                    let m2390 = {
                        let a2388 = v0;
                        let a2389 = 5i64;
                        compare(a2388, a2389)
                    };
                    if match m2390 {
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
                        let m2393 = {
                            let a2391 = v0;
                            let a2392 = 6i64;
                            compare(a2391, a2392)
                        };
                        if match m2393 {
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
                            let m2396 = {
                                let a2394 = v0;
                                let a2395 = 7i64;
                                compare(a2394, a2395)
                            };
                            if match m2396 {
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
                                let m2399 = {
                                    let a2397 = v0;
                                    let a2398 = 8i64;
                                    compare(a2397, a2398)
                                };
                                if match m2399 {
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
                                    let m2402 = {
                                        let a2400 = v0;
                                        let a2401 = 9i64;
                                        compare(a2400, a2401)
                                    };
                                    if match m2402 {
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

pub fn f617(v0: Adt8) -> Adt6 {
    Adt6::C5(f469(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2403) => {
            h2403
        }
    }))
}

pub fn f618(p2408: u64, p2409: i64) -> R<i64> {
    let m2407 = (p2408, p2409);
    {
        let mut m2407 = m2407;
        loop {
            let step = {
                let (v0, v1) = m2407;
                let m2404 = v0;
                if m2404 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2404 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2405 = v1;
                        let a2406 = -1i64;
                        int_sub(a2405, a2406)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f619(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2410, _, _, _, _, _, _, _) => {
            h2410
        }
    })), {
        let a2428 = f464(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2411, _, _, _, _, _, _) => {
                h2411
            }
        });
        let a2429 = {
            let a2426 = f569(match v0.clone() {
                Adt35::C0(_, _, h2412, _, _, _, _, _) => {
                    h2412
                }
            });
            let a2427 = {
                let a2424 = f464(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2413, _, _, _, _) => {
                        h2413
                    }
                });
                let a2425 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2416 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2415, _, _, _) => {
                            h2415
                        }
                    };
                    match m2416 {
                        Adt5::C0(h2414) => {
                            h2414
                        }
                    }
                })), {
                    let a2422 = f505(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2417, _, _) => {
                            h2417
                        }
                    });
                    let a2423 = {
                        let a2420 = f669(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2418, _) => {
                                h2418
                            }
                        });
                        let a2421 = f670(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2419) => {
                                h2419
                            }
                        });
                        append_list(a2420, a2421)
                    };
                    append_list(a2422, a2423)
                });
                append_list(a2424, a2425)
            };
            append_list(a2426, a2427)
        };
        append_list(a2428, a2429)
    }))
}

pub fn f620(v0: Adt38) -> R<Option<Adt6>> {
    let m2430 = v0;
    match m2430 {
        Adt38::C0(v1) => {
            Ok(Some(f671(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f672(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2431, _, _, _) => {
                    h2431
                }
            }, match v2.clone() {
                Adt42::C0(_, h2432, _, _) => {
                    h2432
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2433, _) => {
                    h2433
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2434) => {
                    h2434
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f672(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2435, _, _, _) => {
                    h2435
                }
            }, match v3.clone() {
                Adt43::C0(_, h2436, _, _) => {
                    h2436
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2437, _) => {
                    h2437
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2438) => {
                    h2438
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f673(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f674(v5)))
        }
    }
}

pub fn f621(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2462 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2439, _, _, _, _, _, _, _, _) => {
                h2439
            }
        })), List::<(Str, Adt6)>::nil());
        let a2463 = {
            let a2460 = f464(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2440, _, _, _, _, _, _, _) => {
                    h2440
                }
            });
            let a2461 = {
                let a2458 = f569(match v0.clone() {
                    Adt44::C0(_, _, h2441, _, _, _, _, _, _) => {
                        h2441
                    }
                });
                let a2459 = {
                    let a2456 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2442, _, _, _, _, _) => {
                            h2442
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2457 = {
                        let a2454 = f464(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2443, _, _, _, _) => {
                                h2443
                            }
                        });
                        let a2455 = {
                            let a2452 = f464(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2444, _, _, _) => {
                                    h2444
                                }
                            });
                            let a2453 = {
                                let a2450 = f675(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2445, _, _) => {
                                        h2445
                                    }
                                });
                                let a2451 = {
                                    let a2448 = f676(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2446, _) => {
                                            h2446
                                        }
                                    })?;
                                    let a2449 = f670(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2447) => {
                                            h2447
                                        }
                                    });
                                    append_list(a2448, a2449)
                                };
                                append_list(a2450, a2451)
                            };
                            append_list(a2452, a2453)
                        };
                        append_list(a2454, a2455)
                    };
                    append_list(a2456, a2457)
                };
                append_list(a2458, a2459)
            };
            append_list(a2460, a2461)
        };
        append_list(a2462, a2463)
    }))
}

pub fn f622(v0: Adt46) -> Adt6 {
    let m2464 = v0;
    match m2464 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2465, _, _, _) => {
                    h2465
                }
            })), {
                let a2469 = f464(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2466, _, _) => {
                        h2466
                    }
                });
                let a2470 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2467, _) => {
                        h2467
                    }
                })), f670(match v1 {
                    Adt47::C0(_, _, _, h2468) => {
                        h2468
                    }
                }));
                append_list(a2469, a2470)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2471, _, _, _) => {
                    h2471
                }
            })), {
                let a2475 = f464(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2472, _, _) => {
                        h2472
                    }
                });
                let a2476 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2473, _) => {
                        h2473
                    }
                })), f670(match v2 {
                    Adt48::C0(_, _, _, h2474) => {
                        h2474
                    }
                }));
                append_list(a2475, a2476)
            }))
        }
    }
}

pub fn f623(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a2497 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h2477, _, _, _, _, _, _, _) => {
                h2477
            }
        })), List::<(Str, Adt6)>::nil());
        let a2498 = {
            let a2495 = f464(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h2478, _, _, _, _, _, _) => {
                    h2478
                }
            });
            let a2496 = {
                let a2493 = f569(match v0.clone() {
                    Adt54::C0(_, _, h2479, _, _, _, _, _) => {
                        h2479
                    }
                });
                let a2494 = {
                    let a2491 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h2480, _, _, _, _) => {
                            h2480
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2492 = {
                        let a2489 = f464(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h2481, _, _, _) => {
                                h2481
                            }
                        });
                        let a2490 = {
                            let a2487 = f464(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h2482, _, _) => {
                                    h2482
                                }
                            });
                            let a2488 = {
                                let a2485 = f675(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h2483, _) => {
                                        h2483
                                    }
                                });
                                let a2486 = f670(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h2484) => {
                                        h2484
                                    }
                                });
                                append_list(a2485, a2486)
                            };
                            append_list(a2487, a2488)
                        };
                        append_list(a2489, a2490)
                    };
                    append_list(a2491, a2492)
                };
                append_list(a2493, a2494)
            };
            append_list(a2495, a2496)
        };
        append_list(a2497, a2498)
    })
}

pub fn f624(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2514 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2499, _, _, _, _, _) => {
                h2499
            }
        })), List::<(Str, Adt6)>::nil());
        let a2515 = {
            let a2512 = f464(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2500, _, _, _, _) => {
                    h2500
                }
            });
            let a2513 = {
                let a2510 = f569(match v0.clone() {
                    Adt56::C0(_, _, h2501, _, _, _) => {
                        h2501
                    }
                });
                let a2511 = {
                    let a2508 = f464(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h2502, _, _) => {
                            h2502
                        }
                    });
                    let a2509 = {
                        let a2506 = {
                            let m2504 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h2503, _) => {
                                    h2503
                                }
                            };
                            match m2504 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f677(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2507 = f670(match v0 {
                            Adt56::C0(_, _, _, _, _, h2505) => {
                                h2505
                            }
                        });
                        append_list(a2506, a2507)
                    };
                    append_list(a2508, a2509)
                };
                append_list(a2510, a2511)
            };
            append_list(a2512, a2513)
        };
        append_list(a2514, a2515)
    })
}

pub fn f625(v0: Adt59) -> R<Option<Adt6>> {
    let m2517 = f620(match v0.clone() {
        Adt59::C0(_, h2516) => {
            h2516
        }
    })?;
    match m2517 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f678(match v0 {
                Adt59::C0(h2518, _) => {
                    h2518
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f626(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f468(Str::lit("params"), v1)))
}

pub fn f627(v0: Adt65) -> Adt6 {
    let m2519 = v0;
    match m2519 {
        Adt65::C0(v1) => {
            f679(v1)
        }
        Adt65::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt68::C0(h2520, _) => {
                    h2520
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt68::C0(_, h2521) => {
                    h2521
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f628(v0: Adt69) -> R<Option<Adt6>> {
    let m2523 = f680(match v0.clone() {
        Adt69::C0(h2522, _, _, _, _, _, _, _, _, _) => {
            h2522
        }
    })?;
    match m2523 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2525 = f681(v0.clone());
                let a2526 = List::cons((Str::lit("maxTokens"), f555(match v0.clone() {
                    Adt69::C0(_, _, _, _, _, h2524, _, _, _, _) => {
                        h2524
                    }
                })?), f682(v0));
                append_list(a2525, a2526)
            }))))
        }
    }
}

pub fn f629(p2532: List<(Str, Adt64)>, p2533: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m2531 = (p2532, p2533);
    {
        let mut m2531 = m2531;
        loop {
            let step = {
                let (v0, v1) = m2531;
                let m2527 = v0;
                match m2527.uncons() {
                    None => {
                        Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2529 = f683({
                            let (_, h2528) = v2;
                            h2528
                        });
                        match m2529 {
                            None => {
                                Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt64)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h2530, _) = v2;
                                    h2530
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f630(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f631(v0: List<Adt2>) -> List<Adt6> {
    f326(f684(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f632(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m2534 = f199(v0, v1);
    match m2534 {
        Adt137::C0 => {
            None::<Adt5>
        }
        Adt137::C1(v2) => {
            f217(v2)
        }
    }
}

pub fn f633(v0: Adt5) -> Option<Adt62> {
    let m2535 = f685(v0.clone());
    match m2535 {
        None => {
            None::<Adt62>
        }
        Some(v1) => {
            let m2536 = f686(Str::lit("status"), v0.clone());
            match m2536 {
                None => {
                    None::<Adt62>
                }
                Some(v2) => {
                    let a2537 = v2.clone();
                    let a2538 = Str::lit("working");
                    if equal(a2537, a2538) {
                        Some(Adt62::C0(v1))
                    } else {
                        let a2539 = v2.clone();
                        let a2540 = Str::lit("cancelled");
                        if equal(a2539, a2540) {
                            Some(Adt62::C4(v1))
                        } else {
                            let a2541 = v2.clone();
                            let a2542 = Str::lit("input_required");
                            if equal(a2541, a2542) {
                                let m2543 = f687(Str::lit("inputRequests"), v0);
                                match m2543 {
                                    None => {
                                        None::<Adt62>
                                    }
                                    Some(v3) => {
                                        Some(Adt62::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a2544 = v2.clone();
                                let a2545 = Str::lit("completed");
                                if equal(a2544, a2545) {
                                    let m2546 = f632(Str::lit("result"), v0);
                                    match m2546 {
                                        None => {
                                            None::<Adt62>
                                        }
                                        Some(v4) => {
                                            Some(Adt62::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a2547 = v2;
                                    let a2548 = Str::lit("failed");
                                    if equal(a2547, a2548) {
                                        let m2549 = f632(Str::lit("error"), v0);
                                        match m2549 {
                                            None => {
                                                None::<Adt62>
                                            }
                                            Some(v5) => {
                                                let m2550 = f688(v5);
                                                match m2550 {
                                                    None => {
                                                        None::<Adt62>
                                                    }
                                                    Some(v6) => {
                                                        Some(Adt62::C3(v1, v6))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        None::<Adt62>
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

pub fn f634(v0: Str, v1: Adt5) -> Option<i64> {
    let m2551 = f199(v0, v1);
    match m2551 {
        Adt137::C0 => {
            None::<i64>
        }
        Adt137::C1(v2) => {
            f544(v2)
        }
    }
}

pub fn f635(v0: Adt5) -> Option<Adt85> {
    let m2552 = f634(Str::lit("parkAfterMs"), v0.clone());
    match m2552 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m2553 = f687(Str::lit("parked"), v0.clone());
            match m2553 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m2554 = f634(Str::lit("finishAfterMs"), v0.clone());
                    match m2554 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m2555 = f632(Str::lit("outcome"), v0);
                            match m2555 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m2556 = f689(v4);
                                    match m2556 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            Some(Adt85::C0(v1, v2, v3, v5))
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

pub fn f636(v0: Adt84) -> Option<Adt6> {
    let m2558 = f389(match v0.clone() {
        Adt84::C0(h2557, _, _) => {
            h2557
        }
    });
    match m2558 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m2560 = f690(match v0.clone() {
                Adt84::C0(_, _, h2559) => {
                    h2559
                }
            });
            match m2560 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f630(match v0 {
                        Adt84::C0(_, h2561, _) => {
                            h2561
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f637(v0: List<u64>) -> List<u64> {
    let a2564 = f691();
    let a2565 = {
        let a2562 = v0;
        let a2563 = f692();
        append_list(a2562, a2563)
    };
    append_list(a2564, a2565)
}

pub fn f638(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f693(v0, 0u64, v1)
}

pub fn f639(p2572: List<u64>) -> bool {
    let m2571 = p2572;
    {
        let mut m2571 = m2571;
        loop {
            let step = {
                let v0 = m2571;
                let m2566 = v0;
                match m2566.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2567 = v1;
                        let a2568 = 48u64;
                        if if nat_eq(a2567, a2568) {
                            true
                        } else {
                            let a2569 = v1;
                            let a2570 = 46u64;
                            nat_eq(a2569, a2570)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(v0: Adt141) -> bool {
    let m2573 = v0;
    match m2573 {
        Adt141::C0 => {
            true
        }
        Adt141::C1 => {
            false
        }
        Adt141::C2 => {
            false
        }
        Adt141::C3 => {
            false
        }
        Adt141::C4 => {
            false
        }
        Adt141::C5 => {
            false
        }
        Adt141::C6 => {
            false
        }
        Adt141::C7 => {
            false
        }
        Adt141::C8 => {
            false
        }
        Adt141::C9 => {
            false
        }
        Adt141::C10 => {
            false
        }
        Adt141::C11 => {
            false
        }
    }
}

pub fn f641(v0: Adt14) -> Adt141 {
    let m2574 = v0;
    match m2574 {
        Adt14::C0(_) => {
            Adt141::C0
        }
        Adt14::C1(_) => {
            Adt141::C1
        }
        Adt14::C2(_) => {
            Adt141::C2
        }
        Adt14::C3(_) => {
            Adt141::C3
        }
        Adt14::C4(_) => {
            Adt141::C4
        }
        Adt14::C5(_) => {
            Adt141::C5
        }
        Adt14::C6(_) => {
            Adt141::C6
        }
        Adt14::C7(_) => {
            Adt141::C7
        }
        Adt14::C8(_) => {
            Adt141::C8
        }
        Adt14::C9(_) => {
            Adt141::C9
        }
        Adt14::C10(_) => {
            Adt141::C10
        }
        Adt14::C11(_) => {
            Adt141::C11
        }
    }
}

pub fn f642(v0: Adt0, v1: Adt32) -> Adt130 {
    Adt130::C0(v1, Adt131::C0(Some(match v0 {
        Adt0::C0(h2575, _, _, _, _, _, _, _) => {
            h2575
        }
    }), None::<Adt5>), List::<Adt132>::nil())
}

pub fn f643(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2576, _, _, _, _) => {
            h2576
        }
    }, f197(), match v0.clone() {
        Adt0::C0(_, _, h2577, _, _, _, _, _) => {
            h2577
        }
    }, match v0 {
        Adt0::C0(_, h2578, _, _, _, _, _, _) => {
            h2578
        }
    })
}

pub fn f644(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt89>) -> Result<Adt130, Adt133> {
    let m2579 = v2;
    match m2579 {
        Ok(v4) => {
            if f694(f641(v1.clone()), v4.clone()) {
                f695(v0, v1, v4)
            } else {
                Err::<Adt130, Adt133>(Adt133::C4(f696()))
            }
        }
        Err(v3) => {
            Err::<Adt130, Adt133>(Adt133::C4(v3))
        }
    }
}

pub fn f645(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14) -> R<Result<Adt32, Adt89>> {
    f697(v0.clone(), v1, v2, v3, v4.clone(), f698(v0, v4)?)
}

pub fn f646(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt134> {
    let m2580 = f199(v0.clone(), v1);
    match m2580 {
        Adt137::C0 => {
            Ok::<Option<Str>, Adt134>(None::<Str>)
        }
        Adt137::C1(v2) => {
            let m2581 = f214(v2);
            match m2581 {
                None => {
                    Err::<Option<Str>, Adt134>(Adt134::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt134>(Some(v3))
                }
            }
        }
    }
}

pub fn f647(v0: Str, v1: Adt5) -> Result<Str, Adt134> {
    let m2582 = f199(v0.clone(), v1);
    match m2582 {
        Adt137::C0 => {
            Err::<Str, Adt134>(Adt134::C7(v0))
        }
        Adt137::C1(v2) => {
            let m2583 = f214(v2);
            match m2583 {
                None => {
                    Err::<Str, Adt134>(Adt134::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt134>(v3)
                }
            }
        }
    }
}

pub fn f648(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt134> {
    let m2584 = f199(v0.clone(), v1);
    match m2584 {
        Adt137::C0 => {
            Ok::<Option<Adt5>, Adt134>(None::<Adt5>)
        }
        Adt137::C1(v2) => {
            let m2585 = f217(v2);
            match m2585 {
                None => {
                    Err::<Option<Adt5>, Adt134>(Adt134::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt134>(Some(v3))
                }
            }
        }
    }
}

pub fn f649(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt134> {
    let m2586 = f199(v0.clone(), v1);
    match m2586 {
        Adt137::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt134>(None::<List<(Str, Str)>>)
        }
        Adt137::C1(v2) => {
            let m2587 = f217(v2);
            match m2587 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt134>(Adt134::C8(v0))
                }
                Some(v3) => {
                    let m2589 = f699(v0, match v3 {
                        Adt5::C0(h2588) => {
                            h2588
                        }
                    });
                    match m2589 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt134>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt134>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f650(v0: Str, v1: Adt5) -> Result<Adt5, Adt134> {
    let m2590 = f199(v0.clone(), v1);
    match m2590 {
        Adt137::C0 => {
            Err::<Adt5, Adt134>(Adt134::C7(v0))
        }
        Adt137::C1(v2) => {
            let m2591 = f217(v2);
            match m2591 {
                None => {
                    Err::<Adt5, Adt134>(Adt134::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt134>(v3)
                }
            }
        }
    }
}

pub fn f651(v0: Adt5) -> Result<Adt25, Adt134> {
    let m2592 = f647(Str::lit("type"), v0.clone());
    match m2592 {
        Ok(v1) => {
            let a2593 = v1.clone();
            let a2594 = Str::lit("ref/prompt");
            if equal(a2593, a2594) {
                let m2595 = f647(Str::lit("name"), v0.clone());
                match m2595 {
                    Ok(v2) => {
                        let m2596 = f646(Str::lit("title"), v0);
                        match m2596 {
                            Ok(v3) => {
                                Ok::<Adt25, Adt134>(Adt25::C0(Adt26::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt25, Adt134>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt25, Adt134>(v5)
                    }
                }
            } else {
                let a2597 = v1;
                let a2598 = Str::lit("ref/resource");
                if equal(a2597, a2598) {
                    let m2599 = f647(Str::lit("uri"), v0);
                    match m2599 {
                        Ok(v6) => {
                            Ok::<Adt25, Adt134>(Adt25::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt25, Adt134>(v7)
                        }
                    }
                } else {
                    Err::<Adt25, Adt134>(Adt134::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt25, Adt134>(v8)
        }
    }
}

pub fn f652(v0: Adt5) -> Result<Adt27, Adt134> {
    let m2600 = f647(Str::lit("name"), v0.clone());
    match m2600 {
        Ok(v1) => {
            let m2601 = f647(Str::lit("value"), v0);
            match m2601 {
                Ok(v2) => {
                    Ok::<Adt27, Adt134>(Adt27::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt27, Adt134>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt27, Adt134>(v4)
        }
    }
}

pub fn f653(v0: Adt5) -> Result<Option<Adt28>, Adt134> {
    let m2602 = f199(Str::lit("context"), v0);
    match m2602 {
        Adt137::C0 => {
            Ok::<Option<Adt28>, Adt134>(None::<Adt28>)
        }
        Adt137::C1(v1) => {
            let m2603 = f217(v1);
            match m2603 {
                None => {
                    Err::<Option<Adt28>, Adt134>(Adt134::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2604 = f649(Str::lit("arguments"), v2);
                    match m2604 {
                        Ok(v3) => {
                            Ok::<Option<Adt28>, Adt134>(Some(Adt28::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt28>, Adt134>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f654(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt133> {
    if v0 {
        None::<Adt133>
    } else {
        Some(Adt133::C1(v1, v2))
    }
}

pub fn f655(v0: Str, p2609: List<Str>) -> bool {
    let m2608 = p2609;
    {
        let mut m2608 = m2608;
        loop {
            let step = {
                let v1 = m2608;
                let m2605 = v1;
                match m2605.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2606 = v2;
                        let a2607 = v0.clone();
                        if equal(a2606, a2607) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f656(v0: Option<Adt116>, v1: List<Adt118>, v2: Adt5) -> R<Option<Adt116>> {
    let m2610 = v0;
    match m2610 {
        None => {
            f700(v1, f701(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f657(v0: Option<Str>, v1: Adt149, v2: Adt5) -> Option<Adt116> {
    let m2611 = v1.clone();
    match m2611 {
        Adt149::C0 => {
            None::<Adt116>
        }
        Adt149::C1 => {
            f702(v0, f703(v1, v2))
        }
        Adt149::C2 => {
            f702(v0, f703(v1, v2))
        }
        Adt149::C3 => {
            f702(v0, f703(v1, v2))
        }
    }
}

pub fn f658(v0: Adt137, v1: Adt137) -> Adt155 {
    let m2612 = v0;
    match m2612 {
        Adt137::C0 => {
            Adt155::C1
        }
        Adt137::C1(_) => {
            let m2613 = v1;
            match m2613 {
                Adt137::C0 => {
                    Adt155::C2
                }
                Adt137::C1(_) => {
                    Adt155::C0
                }
            }
        }
    }
}

pub fn f659(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt134> {
    let m2614 = f214(v1);
    match m2614 {
        None => {
            Err::<Adt16, Adt134>(Adt134::C5)
        }
        Some(v3) => {
            let m2615 = f704(v2);
            match m2615 {
                None => {
                    Err::<Adt16, Adt134>(Adt134::C6)
                }
                Some(v4) => {
                    let m2616 = f648(f705(), v0.clone());
                    match m2616 {
                        Ok(v6) => {
                            let m2617 = f706(v0.clone());
                            match m2617 {
                                Ok(v8) => {
                                    let m2618 = f707(v0.clone());
                                    match m2618 {
                                        Ok(v10) => {
                                            Ok::<Adt16, Adt134>(Adt16::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt16, Adt134>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt16, Adt134>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt134>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f660(v0: Adt6) -> u64 {
    let m2619 = v0;
    match m2619 {
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

pub fn f661(v0: Adt154) -> bool {
    let m2620 = v0;
    match m2620 {
        Adt154::C0 => {
            false
        }
        Adt154::C1 => {
            true
        }
        Adt154::C2 => {
            false
        }
    }
}

pub fn f662(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt156 {
    let m2621 = v0;
    match m2621 {
        None => {
            Adt156::C0
        }
        Some(v2) => {
            Adt156::C1({
                let a2622 = v2;
                let a2623 = v1;
                append_list(a2622, a2623)
            })
        }
    }
}

pub fn f663(p2628: List<Adt6>, p2629: List<Adt6>, p2630: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2627 = (p2628, (p2629, p2630));
    {
        let mut m2627 = m2627;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2627;
                let m2624 = v0;
                match m2624.uncons() {
                    None => {
                        let m2625 = v1;
                        match m2625.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2626 = v1;
                        match m2626.uncons() {
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
                Ok(next) => { m2627 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f664(p2636: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2637: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2635 = (p2636, p2637);
    {
        let mut m2635 = m2635;
        loop {
            let step = {
                let (v0, v2) = m2635;
                let m2631 = v0;
                match m2631.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2633 = f213({
                            let (h2632, _) = v3;
                            h2632
                        }, v1.clone());
                        match m2633 {
                            Adt137::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt137::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2634) = v3;
                                    h2634
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f665(v0: Adt154) -> Adt154 {
    let m2638 = v0;
    match m2638 {
        Adt154::C0 => {
            Adt154::C2
        }
        Adt154::C1 => {
            Adt154::C1
        }
        Adt154::C2 => {
            Adt154::C0
        }
    }
}

pub fn f666(v0: i64, v1: i64) -> R<i64> {
    f93(f64(f668(v0)?, 0i64)?, v1)
}

pub fn f667(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt154> {
    let a2639 = v1;
    let a2640 = v3;
    if nat_lt(a2639, a2640) {
        Ok(f545(f708({
            let a2641 = v3;
            let a2642 = v1;
            nat_sub(a2641, a2642)
        }, v0)?, v2))
    } else {
        let a2643 = v3;
        let a2644 = v1;
        if nat_lt(a2643, a2644) {
            Ok(f545(v0, f708({
                let a2645 = v1;
                let a2646 = v3;
                nat_sub(a2645, a2646)
            }, v2)?))
        } else {
            Ok(f545(v0, v2))
        }
    }
}

pub fn f668(v0: i64) -> R<u64> {
    f709(20u64, v0, 0u64)
}

pub fn f669(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2647 = v0;
    match m2647 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2659 = f464(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2648, _, _, _, _) => {
                        h2648
                    }
                });
                let a2660 = {
                    let a2657 = f469(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2649, _, _, _) => {
                            h2649
                        }
                    });
                    let a2658 = {
                        let a2655 = f469(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2650, _, _) => {
                                h2650
                            }
                        });
                        let a2656 = {
                            let a2653 = f469(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2651, _) => {
                                    h2651
                                }
                            });
                            let a2654 = f469(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2652) => {
                                    h2652
                                }
                            });
                            append_list(a2653, a2654)
                        };
                        append_list(a2655, a2656)
                    };
                    append_list(a2657, a2658)
                };
                append_list(a2659, a2660)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f670(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f505(Str::lit("_meta"), v0)
}

pub fn f671(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2661, _, _) => {
            h2661
        }
    })), {
        let a2664 = f675(match v0.clone() {
            Adt39::C0(_, h2662, _) => {
                h2662
            }
        });
        let a2665 = f670(match v0 {
            Adt39::C0(_, _, h2663) => {
                h2663
            }
        });
        append_list(a2664, a2665)
    })))
}

pub fn f672(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2666 = f675(v3);
        let a2667 = f670(v4);
        append_list(a2666, a2667)
    }))))
}

pub fn f673(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2691 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2668, _, _, _, _, _, _, _, _) => {
                h2668
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2692 = {
            let a2689 = f464(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2669, _, _, _, _, _, _, _) => {
                    h2669
                }
            });
            let a2690 = {
                let a2687 = f569(match v0.clone() {
                    Adt44::C0(_, _, h2670, _, _, _, _, _, _) => {
                        h2670
                    }
                });
                let a2688 = {
                    let a2685 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2671, _, _, _, _, _) => {
                            h2671
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2686 = {
                        let a2683 = f464(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2672, _, _, _, _) => {
                                h2672
                            }
                        });
                        let a2684 = {
                            let a2681 = f464(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2673, _, _, _) => {
                                    h2673
                                }
                            });
                            let a2682 = {
                                let a2679 = f675(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2674, _, _) => {
                                        h2674
                                    }
                                });
                                let a2680 = {
                                    let a2677 = f676(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2675, _) => {
                                            h2675
                                        }
                                    })?;
                                    let a2678 = f670(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2676) => {
                                            h2676
                                        }
                                    });
                                    append_list(a2677, a2678)
                                };
                                append_list(a2679, a2680)
                            };
                            append_list(a2681, a2682)
                        };
                        append_list(a2683, a2684)
                    };
                    append_list(a2685, a2686)
                };
                append_list(a2687, a2688)
            };
            append_list(a2689, a2690)
        };
        append_list(a2691, a2692)
    }))
}

pub fn f674(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f622(match v0.clone() {
        Adt45::C0(h2693, _, _) => {
            h2693
        }
    })), {
        let a2696 = f675(match v0.clone() {
            Adt45::C0(_, h2694, _) => {
                h2694
            }
        });
        let a2697 = f670(match v0 {
            Adt45::C0(_, _, h2695) => {
                h2695
            }
        });
        append_list(a2696, a2697)
    })))
}

pub fn f675(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2698 = v0;
    match m2698 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2704 = f710(match v1.clone() {
                    Adt40::C0(h2699, _, _) => {
                        h2699
                    }
                });
                let a2705 = {
                    let a2702 = f711(match v1.clone() {
                        Adt40::C0(_, h2700, _) => {
                            h2700
                        }
                    });
                    let a2703 = f464(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2701) => {
                            h2701
                        }
                    });
                    append_list(a2702, a2703)
                };
                append_list(a2704, a2705)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f676(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2706 = v0;
    match m2706 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f555(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f677(v0: List<Adt57>) -> List<Adt6> {
    f326(f712(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f678(v0: Adt41) -> Str {
    let m2707 = v0;
    match m2707 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f679(v0: Adt66) -> Adt6 {
    Adt6::C5({
        let a2714 = {
            let m2709 = match v0.clone() {
                Adt66::C0(h2708, _, _) => {
                    h2708
                }
            };
            match m2709 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2715 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt66::C0(_, h2710, _) => {
                h2710
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2713 = match v0 {
                Adt66::C0(_, _, h2712) => {
                    h2712
                }
            };
            match m2713 {
                Adt5::C0(h2711) => {
                    h2711
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2714, a2715)
    })
}

pub fn f680(v0: List<Adt70>) -> R<Option<List<Adt6>>> {
    let m2716 = f713(v0, List::<Adt6>::nil())?;
    match m2716 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f326(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f681(v0: Adt69) -> List<(Str, Adt6)> {
    let a2727 = {
        let m2718 = match v0.clone() {
            Adt69::C0(_, h2717, _, _, _, _, _, _, _, _) => {
                h2717
            }
        };
        match m2718 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f714(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2728 = {
        let a2725 = f464(Str::lit("systemPrompt"), match v0.clone() {
            Adt69::C0(_, _, h2719, _, _, _, _, _, _, _) => {
                h2719
            }
        });
        let a2726 = {
            let a2723 = {
                let m2721 = match v0.clone() {
                    Adt69::C0(_, _, _, h2720, _, _, _, _, _, _) => {
                        h2720
                    }
                };
                match m2721 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f715(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2724 = f716(Str::lit("temperature"), match v0 {
                Adt69::C0(_, _, _, _, h2722, _, _, _, _, _) => {
                    h2722
                }
            });
            append_list(a2723, a2724)
        };
        append_list(a2725, a2726)
    };
    append_list(a2727, a2728)
}

pub fn f682(v0: Adt69) -> List<(Str, Adt6)> {
    let a2740 = {
        let m2730 = match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, h2729, _, _, _) => {
                h2729
            }
        };
        match m2730 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f462(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2741 = {
        let a2738 = f505(Str::lit("metadata"), match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, _, h2731, _, _) => {
                h2731
            }
        });
        let a2739 = {
            let a2736 = {
                let m2733 = match v0.clone() {
                    Adt69::C0(_, _, _, _, _, _, _, _, h2732, _) => {
                        h2732
                    }
                };
                match m2733 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f466(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2737 = {
                let m2735 = match v0 {
                    Adt69::C0(_, _, _, _, _, _, _, _, _, h2734) => {
                        h2734
                    }
                };
                match m2735 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f717(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2736, a2737)
        };
        append_list(a2738, a2739)
    };
    append_list(a2740, a2741)
}

pub fn f683(v0: Adt64) -> Option<Adt6> {
    let m2742 = v0;
    match m2742 {
        Adt64::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f718(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt64::C1(_) => {
            None::<Adt6>
        }
        Adt64::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m2743 = v3;
                match m2743 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h2744) => {
                                h2744
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f684(p2747: List<Adt2>, p2748: List<Adt6>) -> List<Adt6> {
    let m2746 = (p2747, p2748);
    {
        let mut m2746 = m2746;
        loop {
            let step = {
                let (v0, v1) = m2746;
                let m2745 = v0;
                match m2745.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f719(v2), v1)))
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

pub fn f685(v0: Adt5) -> Option<Adt63> {
    let m2749 = f686(Str::lit("taskId"), v0.clone());
    match m2749 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m2750 = f686(Str::lit("createdAt"), v0.clone());
            match m2750 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let m2751 = f686(Str::lit("lastUpdatedAt"), v0.clone());
                    match m2751 {
                        None => {
                            None::<Adt63>
                        }
                        Some(v3) => {
                            Some(Adt63::C0(v1, f686(Str::lit("statusMessage"), v0.clone()), v2, v3, f634(Str::lit("ttlMs"), v0.clone()), f634(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f686(v0: Str, v1: Adt5) -> Option<Str> {
    let m2752 = f199(v0, v1);
    match m2752 {
        Adt137::C0 => {
            None::<Str>
        }
        Adt137::C1(v2) => {
            f214(v2)
        }
    }
}

pub fn f687(v0: Str, v1: Adt5) -> Option<List<(Str, Adt64)>> {
    let m2753 = f632(v0, v1);
    match m2753 {
        None => {
            None::<List<(Str, Adt64)>>
        }
        Some(v2) => {
            f720(match v2 {
                Adt5::C0(h2754) => {
                    h2754
                }
            }, List::<(Str, Adt64)>::nil())
        }
    }
}

pub fn f688(v0: Adt5) -> Option<Adt80> {
    let m2755 = f634(Str::lit("code"), v0.clone());
    match m2755 {
        None => {
            None::<Adt80>
        }
        Some(v1) => {
            let m2756 = f686(Str::lit("message"), v0.clone());
            match m2756 {
                None => {
                    None::<Adt80>
                }
                Some(v2) => {
                    Some(Adt80::C0(v1, v2, {
                        let m2757 = f199(Str::lit("data"), v0);
                        match m2757 {
                            Adt137::C0 => {
                                None::<Adt6>
                            }
                            Adt137::C1(v3) => {
                                Some(v3)
                            }
                        }
                    }))
                }
            }
        }
    }
}

pub fn f689(v0: Adt5) -> Option<Adt86> {
    let m2758 = f632(Str::lit("completes"), v0.clone());
    match m2758 {
        None => {
            let m2759 = f632(Str::lit("fails"), v0);
            match m2759 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m2760 = f688(v2);
                    match m2760 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            Some(Adt86::C1(v3))
                        }
                    }
                }
            }
        }
        Some(v1) => {
            Some(Adt86::C0(v1))
        }
    }
}

pub fn f690(v0: Adt85) -> Option<Adt6> {
    let m2762 = f565(match v0.clone() {
        Adt85::C0(_, h2761, _, _) => {
            h2761
        }
    });
    match m2762 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f630(match v0.clone() {
                Adt85::C0(h2763, _, _, _) => {
                    h2763
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f630(match v0.clone() {
                Adt85::C0(_, _, h2764, _) => {
                    h2764
                }
            })), List::cons((Str::lit("outcome"), f721(match v0 {
                Adt85::C0(_, _, _, h2765) => {
                    h2765
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f691() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f692() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f693(p2770: List<u64>, p2771: u64, p2772: List<u64>) -> List<u64> {
    let m2769 = (p2770, (p2771, p2772));
    {
        let mut m2769 = m2769;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2769;
                let m2766 = v0;
                match m2766.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f722(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt157 = f723(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt157::C0(h2767, _) => {
                                h2767
                            }
                        }, match v5 {
                            Adt157::C0(_, h2768) => {
                                h2768
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f694(v0: Adt141, v1: Adt32) -> bool {
    let m2773 = v1;
    match m2773 {
        Adt32::C0(_) => {
            f724(v0, Adt141::C0)
        }
        Adt32::C1(_) => {
            f724(v0, Adt141::C1)
        }
        Adt32::C2(_) => {
            f724(v0, Adt141::C2)
        }
        Adt32::C3(_) => {
            f724(v0, Adt141::C2)
        }
        Adt32::C4(_) => {
            f724(v0, Adt141::C3)
        }
        Adt32::C5(_) => {
            f724(v0, Adt141::C4)
        }
        Adt32::C6(_) => {
            f724(v0, Adt141::C5)
        }
        Adt32::C7(_) => {
            f724(v0, Adt141::C6)
        }
        Adt32::C8(_) => {
            f724(v0, Adt141::C7)
        }
        Adt32::C9(_) => {
            f724(v0, Adt141::C8)
        }
        Adt32::C10(_) => {
            f724(v0, Adt141::C9)
        }
        Adt32::C11 => {
            if f724(v0.clone(), Adt141::C10) {
                true
            } else {
                f724(v0, Adt141::C11)
            }
        }
        Adt32::C12(_) => {
            f725(v0)
        }
        Adt32::C13(_) => {
            f726(v0)
        }
        Adt32::C14(v15) => {
            let m2775 = match v15 {
                Adt87::C0(_, h2774, _) => {
                    h2774
                }
            };
            match m2775 {
                Adt88::C0 => {
                    f724(v0, Adt141::C9)
                }
                Adt88::C1 => {
                    if f724(v0.clone(), Adt141::C10) {
                        true
                    } else {
                        f724(v0, Adt141::C11)
                    }
                }
            }
        }
    }
}

pub fn f695(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt130, Adt133> {
    let m2776 = v2.clone();
    match m2776 {
        Adt32::C0(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C3(v9) => {
            Ok::<Adt130, Adt133>(f731(v0, f728(v1), v9))
        }
        Adt32::C4(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt130, Adt133>(f642(v0, v2))
        }
        Adt32::C12(v3) => {
            f727(v0, f728(v1), v3)
        }
        Adt32::C13(v4) => {
            f729(v0, f728(v1), v4)
        }
        Adt32::C14(v5) => {
            f730(v0, f728(v1), v5)
        }
    }
}

pub fn f696() -> Adt89 {
    Adt89::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f697(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14, v5: Result<Adt14, Adt89>) -> R<Result<Adt32, Adt89>> {
    let m2777 = v5;
    match m2777 {
        Ok(v7) => {
            f732(v0, v4, f733(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt32, Adt89>(v6))
        }
    }
}

pub fn f698(v0: Bytes, v1: Adt14) -> R<Result<Adt14, Adt89>> {
    let m2778 = v1.clone();
    match m2778 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
        Adt14::C1(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
        Adt14::C2(v2) => {
            f734(v0, v2)
        }
        Adt14::C3(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
        Adt14::C4(v3) => {
            f735(v0, v3)
        }
        Adt14::C5(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
        Adt14::C6(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
        Adt14::C7(v4) => {
            f736(v0, v4)
        }
        Adt14::C8(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
        Adt14::C9(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
        Adt14::C10(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
        Adt14::C11(_) => {
            Ok(Ok::<Adt14, Adt89>(v1))
        }
    }
}

pub fn f699(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt134> {
    let m2779 = f737(v0, v1, List::<(Str, Str)>::nil());
    match m2779 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt134>(f738(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt134>(v3)
        }
    }
}

pub fn f700(p2783: List<Adt118>, v1: Adt5) -> R<Option<Adt116>> {
    let m2782 = p2783;
    {
        let mut m2782 = m2782;
        loop {
            let step = {
                let v0 = m2782;
                let m2780 = v0;
                match m2780.uncons() {
                    None => {
                        Err::<List<Adt118>, R<Option<Adt116>>>(Ok(None::<Adt116>))
                    }
                    Some((v2, v3)) => {
                        let m2781 = f739(v2, v1.clone())?;
                        match m2781 {
                            None => {
                                Ok::<List<Adt118>, R<Option<Adt116>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt118>, R<Option<Adt116>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f701(v0: Adt5) -> Adt5 {
    let m2784 = f199(Str::lit("arguments"), v0);
    match m2784 {
        Adt137::C0 => {
            f597()
        }
        Adt137::C1(v1) => {
            let m2785 = f217(v1);
            match m2785 {
                None => {
                    f597()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f702(v0: Option<Str>, v1: Option<Str>) -> Option<Adt116> {
    let m2786 = v0;
    match m2786 {
        None => {
            Some(Adt116::C2)
        }
        Some(v2) => {
            let m2787 = v1;
            match m2787 {
                None => {
                    None::<Adt116>
                }
                Some(v3) => {
                    let a2788 = v2;
                    let a2789 = v3;
                    if equal(a2788, a2789) {
                        None::<Adt116>
                    } else {
                        Some(Adt116::C5)
                    }
                }
            }
        }
    }
}

pub fn f703(v0: Adt149, v1: Adt5) -> Option<Str> {
    let m2790 = v0;
    match m2790 {
        Adt149::C0 => {
            None::<Str>
        }
        Adt149::C1 => {
            let m2791 = f199(Str::lit("name"), v1);
            match m2791 {
                Adt137::C0 => {
                    None::<Str>
                }
                Adt137::C1(v2) => {
                    f214(v2)
                }
            }
        }
        Adt149::C2 => {
            let m2792 = f199(Str::lit("uri"), v1);
            match m2792 {
                Adt137::C0 => {
                    None::<Str>
                }
                Adt137::C1(v3) => {
                    f214(v3)
                }
            }
        }
        Adt149::C3 => {
            let m2793 = f199(Str::lit("taskId"), v1);
            match m2793 {
                Adt137::C0 => {
                    None::<Str>
                }
                Adt137::C1(v4) => {
                    f214(v4)
                }
            }
        }
    }
}

pub fn f704(v0: Adt6) -> Option<Adt17> {
    let m2794 = f217(v0);
    match m2794 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2795 = f740(Str::lit("experimental"), v1.clone());
            match m2795 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2796 = f740(Str::lit("roots"), v1.clone());
                    match m2796 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2797 = f740(Str::lit("sampling"), v1.clone());
                            match m2797 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2798 = f740(Str::lit("elicitation"), v1.clone());
                                    match m2798 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2799 = f740(Str::lit("extensions"), v1);
                                            match m2799 {
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

pub fn f705() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f706(v0: Adt5) -> Result<Option<Adt18>, Adt134> {
    let m2800 = f646(f741(), v0);
    match m2800 {
        Ok(v2) => {
            let m2801 = v2;
            match m2801 {
                None => {
                    Ok::<Option<Adt18>, Adt134>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2802 = f742(v3);
                    match m2802 {
                        Adt158::C0 => {
                            Err::<Option<Adt18>, Adt134>(Adt134::C8(f741()))
                        }
                        Adt158::C1(v4) => {
                            Ok::<Option<Adt18>, Adt134>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt18>, Adt134>(v1)
        }
    }
}

pub fn f707(v0: Adt5) -> Result<Option<Adt19>, Adt134> {
    let m2803 = f199(f743(), v0);
    match m2803 {
        Adt137::C0 => {
            Ok::<Option<Adt19>, Adt134>(None::<Adt19>)
        }
        Adt137::C1(v1) => {
            let m2804 = f214(v1.clone());
            match m2804 {
                None => {
                    let m2805 = f544(v1);
                    match m2805 {
                        None => {
                            Err::<Option<Adt19>, Adt134>(Adt134::C8(f743()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt19>, Adt134>(Some(Adt19::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt19>, Adt134>(Some(Adt19::C0(v2)))
                }
            }
        }
    }
}

pub fn f708(p2810: u64, p2811: i64) -> R<i64> {
    let m2809 = (p2810, p2811);
    {
        let mut m2809 = m2809;
        loop {
            let step = {
                let (v0, v1) = m2809;
                let m2806 = v0;
                if m2806 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2806 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2807 = v1;
                        let a2808 = 10i64;
                        int_mul(a2807, a2808)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f709(p2822: u64, p2823: i64, p2824: u64) -> R<u64> {
    let m2821 = (p2822, (p2823, p2824));
    {
        let mut m2821 = m2821;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2821;
                let m2812 = v0;
                if m2812 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2812 - 1;
                    let v4: u64 = {
                        let a2813 = v2;
                        let a2814 = 1u64;
                        nat_add(a2813, a2814)?
                    };
                    let m2817 = {
                        let a2815 = v1;
                        let a2816 = 10i64;
                        compare(a2815, a2816)
                    };
                    if match m2817 {
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
                            let a2818 = v1;
                            let a2819 = 10i64;
                            let a2820 = 0i64;
                            int_quot(a2818, a2819, a2820)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f710(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2825 = v0;
    match m2825 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f744(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f711(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2826 = v0;
    match m2826 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f712(p2829: List<Adt57>, p2830: List<Adt6>) -> List<Adt6> {
    let m2828 = (p2829, p2830);
    {
        let mut m2828 = m2828;
        loop {
            let step = {
                let (v0, v1) = m2828;
                let m2827 = v0;
                match m2827.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f745(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f713(p2834: List<Adt70>, p2835: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2833 = (p2834, p2835);
    {
        let mut m2833 = m2833;
        loop {
            let step = {
                let (v0, v1) = m2833;
                let m2831 = v0;
                match m2831.uncons() {
                    None => {
                        Err::<(List<Adt70>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2832 = f746(v2)?;
                        match m2832 {
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
                Ok(next) => { m2833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f714(v0: Adt75) -> Adt6 {
    Adt6::C5({
        let a2845 = {
            let m2837 = match v0.clone() {
                Adt75::C0(h2836, _, _, _) => {
                    h2836
                }
            };
            match m2837 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f326(f747(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2846 = {
            let a2843 = f716(Str::lit("costPriority"), match v0.clone() {
                Adt75::C0(_, h2838, _, _) => {
                    h2838
                }
            });
            let a2844 = {
                let a2841 = f716(Str::lit("speedPriority"), match v0.clone() {
                    Adt75::C0(_, _, h2839, _) => {
                        h2839
                    }
                });
                let a2842 = f716(Str::lit("intelligencePriority"), match v0 {
                    Adt75::C0(_, _, _, h2840) => {
                        h2840
                    }
                });
                append_list(a2841, a2842)
            };
            append_list(a2843, a2844)
        };
        append_list(a2845, a2846)
    })
}

pub fn f715(v0: Adt77) -> Str {
    let m2847 = v0;
    match m2847 {
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

pub fn f716(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2848 = v1;
    match m2848 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f717(v0: Adt78) -> List<(Str, Adt6)> {
    let m2850 = match v0 {
        Adt78::C0(h2849) => {
            h2849
        }
    };
    match m2850 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f748(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f718(v0: Adt65) -> List<(Str, Adt6)> {
    let m2851 = v0;
    match m2851 {
        Adt65::C0(v1) => {
            let a2858 = {
                let m2853 = match v1.clone() {
                    Adt66::C0(h2852, _, _) => {
                        h2852
                    }
                };
                match m2853 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2859 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt66::C0(_, h2854, _) => {
                    h2854
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m2857 = match v1 {
                    Adt66::C0(_, _, h2856) => {
                        h2856
                    }
                };
                match m2857 {
                    Adt5::C0(h2855) => {
                        h2855
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a2858, a2859)
        }
        Adt65::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt68::C0(h2860, _) => {
                    h2860
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt68::C0(_, h2861) => {
                    h2861
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f719(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2862, _, _, _) => {
            h2862
        }
    })), {
        let a2868 = f464(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2863, _, _) => {
                h2863
            }
        });
        let a2869 = {
            let a2866 = f749(match v0.clone() {
                Adt2::C0(_, _, h2864, _) => {
                    h2864
                }
            });
            let a2867 = f750(match v0 {
                Adt2::C0(_, _, _, h2865) => {
                    h2865
                }
            });
            append_list(a2866, a2867)
        };
        append_list(a2868, a2869)
    }))
}

pub fn f720(p2875: List<(Str, Adt6)>, p2876: List<(Str, Adt64)>) -> Option<List<(Str, Adt64)>> {
    let m2874 = (p2875, p2876);
    {
        let mut m2874 = m2874;
        loop {
            let step = {
                let (v0, v1) = m2874;
                let m2870 = v0;
                match m2870.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt64)>), Option<List<(Str, Adt64)>>>(Some(f751(v1, List::<(Str, Adt64)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2872 = f752({
                            let (_, h2871) = v2;
                            h2871
                        });
                        match m2872 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt64)>), Option<List<(Str, Adt64)>>>(None::<List<(Str, Adt64)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt64)>), Option<List<(Str, Adt64)>>>((v3, List::cons(({
                                    let (h2873, _) = v2;
                                    h2873
                                }, v4), v1)))
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

pub fn f721(v0: Adt86) -> Adt6 {
    let m2877 = v0;
    match m2877 {
        Adt86::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h2878) => {
                    h2878
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt86::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f145(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f722(v0: u64, v1: List<u64>) -> List<u64> {
    let a2879 = v0;
    let a2880 = 0u64;
    if nat_eq(a2879, a2880) {
        v1
    } else {
        let a2881 = v0;
        let a2882 = 1u64;
        if nat_eq(a2881, a2882) {
            List::cons(194u64, v1)
        } else {
            let a2883 = v0;
            let a2884 = 2u64;
            if nat_eq(a2883, a2884) {
                List::cons(226u64, v1)
            } else {
                let a2885 = v0;
                let a2886 = 3u64;
                if nat_eq(a2885, a2886) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f723(v0: u64, v1: u64, v2: List<u64>) -> Adt157 {
    let a2887 = v0;
    let a2888 = 0u64;
    if nat_eq(a2887, a2888) {
        f753(v1, v2)
    } else {
        let a2889 = v0;
        let a2890 = 1u64;
        if nat_eq(a2889, a2890) {
            let a2891 = 128u64;
            let a2892 = v1;
            if if nat_le(a2891, a2892) {
                let a2893 = v1;
                let a2894 = 159u64;
                nat_le(a2893, a2894)
            } else {
                false
            } {
                Adt157::C0(0u64, f754(v2))
            } else {
                f753(v1, List::cons(194u64, v2))
            }
        } else {
            let a2895 = v0;
            let a2896 = 2u64;
            if nat_eq(a2895, a2896) {
                let a2897 = v1;
                let a2898 = 128u64;
                if nat_eq(a2897, a2898) {
                    Adt157::C0(3u64, v2)
                } else {
                    let a2899 = v1;
                    let a2900 = 129u64;
                    if nat_eq(a2899, a2900) {
                        Adt157::C0(4u64, v2)
                    } else {
                        f753(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2901 = v0;
                let a2902 = 3u64;
                if nat_eq(a2901, a2902) {
                    let a2903 = 170u64;
                    let a2904 = v1;
                    if if nat_le(a2903, a2904) {
                        let a2905 = v1;
                        let a2906 = 174u64;
                        nat_le(a2905, a2906)
                    } else {
                        false
                    } {
                        Adt157::C0(0u64, f754(v2))
                    } else {
                        f753(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2907 = 166u64;
                    let a2908 = v1;
                    if if nat_le(a2907, a2908) {
                        let a2909 = v1;
                        let a2910 = 169u64;
                        nat_le(a2909, a2910)
                    } else {
                        false
                    } {
                        Adt157::C0(0u64, f754(v2))
                    } else {
                        f753(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f724(v0: Adt141, v1: Adt141) -> bool {
    let a2911 = f509(v0);
    let a2912 = f509(v1);
    equal(a2911, a2912)
}

pub fn f725(v0: Adt141) -> bool {
    let m2913 = v0;
    match m2913 {
        Adt141::C0 => {
            false
        }
        Adt141::C1 => {
            false
        }
        Adt141::C2 => {
            true
        }
        Adt141::C3 => {
            false
        }
        Adt141::C4 => {
            true
        }
        Adt141::C5 => {
            false
        }
        Adt141::C6 => {
            false
        }
        Adt141::C7 => {
            true
        }
        Adt141::C8 => {
            false
        }
        Adt141::C9 => {
            false
        }
        Adt141::C10 => {
            false
        }
        Adt141::C11 => {
            false
        }
    }
}

pub fn f726(v0: Adt141) -> bool {
    let m2914 = v0;
    match m2914 {
        Adt141::C0 => {
            false
        }
        Adt141::C1 => {
            false
        }
        Adt141::C2 => {
            true
        }
        Adt141::C3 => {
            false
        }
        Adt141::C4 => {
            false
        }
        Adt141::C5 => {
            false
        }
        Adt141::C6 => {
            false
        }
        Adt141::C7 => {
            false
        }
        Adt141::C8 => {
            false
        }
        Adt141::C9 => {
            false
        }
        Adt141::C10 => {
            false
        }
        Adt141::C11 => {
            false
        }
    }
}

pub fn f727(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt130, Adt133> {
    f755(v0, v2.clone(), f756(match v1 {
        Adt16::C0(_, h2915, _, _, _, _) => {
            h2915
        }
    }, f477(v2)))
}

pub fn f728(v0: Adt14) -> Adt16 {
    let m2916 = v0;
    match m2916 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2917) => {
                    h2917
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2918, _) => {
                    h2918
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2919, _, _, _, _) => {
                    h2919
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2920, _) => {
                    h2920
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2921, _, _, _) => {
                    h2921
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2922, _) => {
                    h2922
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2923, _) => {
                    h2923
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2924, _, _, _, _) => {
                    h2924
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2925, _, _, _) => {
                    h2925
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2926, _) => {
                    h2926
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2927, _, _) => {
                    h2927
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2928, _) => {
                    h2928
                }
            }
        }
    }
}

pub fn f729(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt130, Adt133> {
    f757(v0, v2, f758(match v1 {
        Adt16::C0(_, h2929, _, _, _, _) => {
            h2929
        }
    }))
}

pub fn f730(v0: Adt0, v1: Adt16, v2: Adt87) -> Result<Adt130, Adt133> {
    f759(v0, v2, f758(match v1 {
        Adt16::C0(_, h2930, _, _, _, _) => {
            h2930
        }
    }))
}

pub fn f731(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt130 {
    Adt130::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2931, _) => {
            h2931
        }
    }), {
        let m2933 = f642(v0, Adt32::C11);
        match m2933 {
            Adt130::C0(_, h2932, _) => {
                h2932
            }
        }
    }, f760(v1, match v2 {
        Adt49::C0(_, h2934) => {
            h2934
        }
    }))
}

pub fn f732(v0: Bytes, v1: Adt14, v2: Result<Adt32, Adt89>) -> R<Result<Adt32, Adt89>> {
    let m2935 = v2;
    match m2935 {
        Ok(v4) => {
            f761(v0, f641(v1.clone()), f762(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt89>(v3))
        }
    }
}

pub fn f733(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt89>> {
    let m2936 = v3.clone();
    match m2936 {
        Adt14::C0(_) => {
            let c2938: Fn0 = v2;
            c2938.apply(v3)
        }
        Adt14::C1(_) => {
            let c2939: Fn0 = v2;
            c2939.apply(v3)
        }
        Adt14::C2(_) => {
            if f763(v0, v1) {
                let c2937: Fn0 = v2;
                c2937.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt89>(Adt89::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2940: Fn0 = v2;
            c2940.apply(v3)
        }
        Adt14::C4(_) => {
            let c2941: Fn0 = v2;
            c2941.apply(v3)
        }
        Adt14::C5(_) => {
            let c2942: Fn0 = v2;
            c2942.apply(v3)
        }
        Adt14::C6(_) => {
            let c2943: Fn0 = v2;
            c2943.apply(v3)
        }
        Adt14::C7(_) => {
            let c2944: Fn0 = v2;
            c2944.apply(v3)
        }
        Adt14::C8(_) => {
            let c2945: Fn0 = v2;
            c2945.apply(v3)
        }
        Adt14::C9(_) => {
            let c2946: Fn0 = v2;
            c2946.apply(v3)
        }
        Adt14::C10(_) => {
            let c2947: Fn0 = v2;
            c2947.apply(v3)
        }
        Adt14::C11(_) => {
            let c2948: Fn0 = v2;
            c2948.apply(v3)
        }
    }
}

pub fn f734(v0: Bytes, v1: Adt21) -> R<Result<Adt14, Adt89>> {
    let m2952 = f764(v0, Str::lit("tools/call"), match v1.clone() {
        Adt21::C0(_, h2949, _, _, _) => {
            h2949
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, _, h2950) => {
            h2950
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, h2951, _) => {
            h2951
        }
    })?;
    match m2952 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt89>(Adt14::C2(Adt21::C0(match v1.clone() {
                Adt21::C0(h2953, _, _, _, _) => {
                    h2953
                }
            }, match v1.clone() {
                Adt21::C0(_, h2954, _, _, _) => {
                    h2954
                }
            }, match v1.clone() {
                Adt21::C0(_, _, h2955, _, _) => {
                    h2955
                }
            }, match v1 {
                Adt21::C0(_, _, _, h2956, _) => {
                    h2956
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt89>(v2))
        }
    }
}

pub fn f735(v0: Bytes, v1: Adt22) -> R<Result<Adt14, Adt89>> {
    let m2960 = f764(v0, Str::lit("resources/read"), match v1.clone() {
        Adt22::C0(_, h2957, _, _) => {
            h2957
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h2958) => {
            h2958
        }
    }, match v1.clone() {
        Adt22::C0(_, _, h2959, _) => {
            h2959
        }
    })?;
    match m2960 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt89>(Adt14::C4(Adt22::C0(match v1.clone() {
                Adt22::C0(h2961, _, _, _) => {
                    h2961
                }
            }, match v1.clone() {
                Adt22::C0(_, h2962, _, _) => {
                    h2962
                }
            }, match v1 {
                Adt22::C0(_, _, h2963, _) => {
                    h2963
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt89>(v2))
        }
    }
}

pub fn f736(v0: Bytes, v1: Adt23) -> R<Result<Adt14, Adt89>> {
    let m2967 = f764(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt23::C0(_, h2964, _, _, _) => {
            h2964
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, _, h2965) => {
            h2965
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h2966, _) => {
            h2966
        }
    })?;
    match m2967 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt89>(Adt14::C7(Adt23::C0(match v1.clone() {
                Adt23::C0(h2968, _, _, _, _) => {
                    h2968
                }
            }, match v1.clone() {
                Adt23::C0(_, h2969, _, _, _) => {
                    h2969
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h2970, _, _) => {
                    h2970
                }
            }, match v1 {
                Adt23::C0(_, _, _, h2971, _) => {
                    h2971
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt89>(v2))
        }
    }
}

pub fn f737(v0: Str, p2977: List<(Str, Adt6)>, p2978: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt134> {
    let m2976 = (p2977, p2978);
    {
        let mut m2976 = m2976;
        loop {
            let step = {
                let (v1, v2) = m2976;
                let m2972 = v1;
                match m2972.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt134>>(Ok::<List<(Str, Str)>, Adt134>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2974 = f214({
                            let (_, h2973) = v3;
                            h2973
                        });
                        match m2974 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt134>>(Err::<List<(Str, Str)>, Adt134>(Adt134::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt134>>((v4, List::cons(({
                                    let (h2975, _) = v3;
                                    h2975
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f738(p2981: List<(Str, Str)>, p2982: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2980 = (p2981, p2982);
    {
        let mut m2980 = m2980;
        loop {
            let step = {
                let (v0, v1) = m2980;
                let m2979 = v0;
                match m2979.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2980 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f739(v0: Adt118, v1: Adt5) -> R<Option<Adt116>> {
    let m2984 = f765(match v0.clone() {
        Adt118::C0(_, h2983, _, _) => {
            h2983
        }
    }, v1);
    match m2984 {
        Adt137::C0 => {
            Ok(f766(v0))
        }
        Adt137::C1(v2) => {
            if f767(v2.clone()) {
                Ok(f766(v0))
            } else {
                f768(v0, v2)
            }
        }
    }
}

pub fn f740(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2985 = f199(v0, v1);
    match m2985 {
        Adt137::C0 => {
            Some(None::<Adt5>)
        }
        Adt137::C1(v2) => {
            let m2986 = f217(v2);
            match m2986 {
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

pub fn f741() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f742(v0: Str) -> Adt158 {
    let a2987 = v0.clone();
    let a2988 = Str::lit("debug");
    if equal(a2987, a2988) {
        Adt158::C1(Adt18::C0)
    } else {
        let a2989 = v0.clone();
        let a2990 = Str::lit("info");
        if equal(a2989, a2990) {
            Adt158::C1(Adt18::C1)
        } else {
            let a2991 = v0.clone();
            let a2992 = Str::lit("notice");
            if equal(a2991, a2992) {
                Adt158::C1(Adt18::C2)
            } else {
                let a2993 = v0.clone();
                let a2994 = Str::lit("warning");
                if equal(a2993, a2994) {
                    Adt158::C1(Adt18::C3)
                } else {
                    let a2995 = v0.clone();
                    let a2996 = Str::lit("error");
                    if equal(a2995, a2996) {
                        Adt158::C1(Adt18::C4)
                    } else {
                        let a2997 = v0.clone();
                        let a2998 = Str::lit("critical");
                        if equal(a2997, a2998) {
                            Adt158::C1(Adt18::C5)
                        } else {
                            let a2999 = v0.clone();
                            let a3000 = Str::lit("alert");
                            if equal(a2999, a3000) {
                                Adt158::C1(Adt18::C6)
                            } else {
                                let a3001 = v0;
                                let a3002 = Str::lit("emergency");
                                if equal(a3001, a3002) {
                                    Adt158::C1(Adt18::C7)
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

pub fn f743() -> Str {
    Str::lit("progressToken")
}

pub fn f744(v0: List<Adt41>) -> List<Adt6> {
    f326(f769(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f745(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a3011 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h3003, _, _, _) => {
                h3003
            }
        })), List::<(Str, Adt6)>::nil());
        let a3012 = {
            let a3009 = f464(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h3004, _, _) => {
                    h3004
                }
            });
            let a3010 = {
                let a3007 = f464(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h3005, _) => {
                        h3005
                    }
                });
                let a3008 = f469(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h3006) => {
                        h3006
                    }
                });
                append_list(a3007, a3008)
            };
            append_list(a3009, a3010)
        };
        append_list(a3011, a3012)
    })
}

pub fn f746(v0: Adt70) -> R<Option<Adt6>> {
    let m3014 = f770(match v0.clone() {
        Adt70::C0(_, h3013, _) => {
            h3013
        }
    })?;
    match m3014 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f678(match v0.clone() {
                Adt70::C0(h3015, _, _) => {
                    h3015
                }
            }))), List::cons((Str::lit("content"), v1), f670(match v0 {
                Adt70::C0(_, _, h3016) => {
                    h3016
                }
            }))))))
        }
    }
}

pub fn f747(p3020: List<Adt76>, p3021: List<Adt6>) -> List<Adt6> {
    let m3019 = (p3020, p3021);
    {
        let mut m3019 = m3019;
        loop {
            let step = {
                let (v0, v1) = m3019;
                let m3017 = v0;
                match m3017.uncons() {
                    None => {
                        Err::<(List<Adt76>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt76>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f464(Str::lit("name"), match v2 {
                            Adt76::C0(h3018) => {
                                h3018
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3019 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f748(v0: Adt79) -> Str {
    let m3022 = v0;
    match m3022 {
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

pub fn f749(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3023 = v0;
    match m3023 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f411(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f750(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3024 = v0;
    match m3024 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f771(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f751(p3027: List<(Str, Adt64)>, p3028: List<(Str, Adt64)>) -> List<(Str, Adt64)> {
    let m3026 = (p3027, p3028);
    {
        let mut m3026 = m3026;
        loop {
            let step = {
                let (v0, v1) = m3026;
                let m3025 = v0;
                match m3025.uncons() {
                    None => {
                        Err::<(List<(Str, Adt64)>, List<(Str, Adt64)>), List<(Str, Adt64)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt64)>, List<(Str, Adt64)>), List<(Str, Adt64)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f752(v0: Adt6) -> Option<Adt64> {
    let m3029 = f217(v0);
    match m3029 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m3030 = f686(Str::lit("method"), v1.clone());
            match m3030 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let a3031 = v2.clone();
                    let a3032 = Str::lit("elicitation/create");
                    if equal(a3031, a3032) {
                        let m3033 = f632(Str::lit("params"), v1);
                        match m3033 {
                            None => {
                                None::<Adt64>
                            }
                            Some(v3) => {
                                f772(v3)
                            }
                        }
                    } else {
                        let a3034 = v2;
                        let a3035 = Str::lit("roots/list");
                        if equal(a3034, a3035) {
                            f773(v1)
                        } else {
                            None::<Adt64>
                        }
                    }
                }
            }
        }
    }
}

pub fn f753(v0: u64, v1: List<u64>) -> Adt157 {
    let a3036 = v0;
    let a3037 = 32u64;
    if nat_lt(a3036, a3037) {
        if f774(v0) {
            Adt157::C0(0u64, List::cons(v0, v1))
        } else {
            Adt157::C0(0u64, f754(v1))
        }
    } else {
        let a3038 = v0;
        let a3039 = 127u64;
        if nat_eq(a3038, a3039) {
            Adt157::C0(0u64, f754(v1))
        } else {
            let a3040 = v0;
            let a3041 = 194u64;
            if nat_eq(a3040, a3041) {
                Adt157::C0(1u64, v1)
            } else {
                let a3042 = v0;
                let a3043 = 226u64;
                if nat_eq(a3042, a3043) {
                    Adt157::C0(2u64, v1)
                } else {
                    Adt157::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f754(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f755(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt130, Adt133> {
    if v2 {
        Ok::<Adt130, Adt133>(f642(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt130, Adt133>(Adt133::C4(Adt89::C8(f775(f477(v1)))))
    }
}

pub fn f756(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a3044 = f776(v1.clone());
    if if if bool_not(a3044) {
        true
    } else {
        f512(match v0.clone() {
            Adt17::C0(_, _, _, h3045, _) => {
                h3045
            }
        })
    } {
        let a3046 = f777(v1.clone());
        if bool_not(a3046) {
            true
        } else {
            f512(match v0.clone() {
                Adt17::C0(_, _, h3047, _, _) => {
                    h3047
                }
            })
        }
    } else {
        false
    } {
        let a3048 = f778(v1);
        if bool_not(a3048) {
            true
        } else {
            f512(match v0 {
                Adt17::C0(_, h3049, _, _, _) => {
                    h3049
                }
            })
        }
    } else {
        false
    }
}

pub fn f757(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt130, Adt133> {
    if v2 {
        Ok::<Adt130, Adt133>(f642(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt130, Adt133>(Adt133::C4(Adt89::C8(f779())))
    }
}

pub fn f758(v0: Adt17) -> bool {
    f513(match v0 {
        Adt17::C0(_, _, _, _, h3050) => {
            h3050
        }
    }, f18())
}

pub fn f759(v0: Adt0, v1: Adt87, v2: bool) -> Result<Adt130, Adt133> {
    if v2 {
        Ok::<Adt130, Adt133>(f642(v0, Adt32::C14(v1)))
    } else {
        Err::<Adt130, Adt133>(Adt133::C4(Adt89::C8(f779())))
    }
}

pub fn f760(v0: Adt16, v1: List<Adt50>) -> List<Adt132> {
    let m3052 = match v0 {
        Adt16::C0(_, _, _, _, h3051, _) => {
            h3051
        }
    };
    match m3052 {
        None => {
            List::<Adt132>::nil()
        }
        Some(v2) => {
            f780(f781(v2, v1, List::<Adt132>::nil()), List::<Adt132>::nil())
        }
    }
}

pub fn f761(v0: Bytes, v1: Adt141, v2: Str, v3: Adt32) -> R<Result<Adt32, Adt89>> {
    let m3053 = v3.clone();
    match m3053 {
        Adt32::C0(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C1(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C2(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C3(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C4(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C5(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C6(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C7(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C8(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C9(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C10(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C11 => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C12(v4) => {
            f782(v0, v1, v2, v4)
        }
        Adt32::C13(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
        Adt32::C14(_) => {
            Ok(Ok::<Adt32, Adt89>(v3))
        }
    }
}

pub fn f762(v0: Adt14) -> Str {
    let m3054 = v0;
    match m3054 {
        Adt14::C0(_) => {
            Str::lit("")
        }
        Adt14::C1(_) => {
            Str::lit("")
        }
        Adt14::C2(v1) => {
            match v1 {
                Adt21::C0(_, h3055, _, _, _) => {
                    h3055
                }
            }
        }
        Adt14::C3(_) => {
            Str::lit("")
        }
        Adt14::C4(v2) => {
            match v2 {
                Adt22::C0(_, h3056, _, _) => {
                    h3056
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
                Adt23::C0(_, h3057, _, _, _) => {
                    h3057
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

pub fn f763(v0: u64, v1: u64) -> bool {
    let a3058 = v1;
    let a3059 = v0;
    nat_lt(a3058, a3059)
}

pub fn f764(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt89>> {
    Ok(f783(f784(v0, v1, v2, v3)?, f785(v4)))
}

pub fn f765(p3065: List<Str>, p3066: Adt5) -> Adt137 {
    let m3064 = (p3065, p3066);
    {
        let mut m3064 = m3064;
        loop {
            let step = {
                let (v0, v1) = m3064;
                let m3060 = v0;
                match m3060.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt137>(Adt137::C0)
                    }
                    Some((v2, v3)) => {
                        let m3061 = f199(v2, v1);
                        match m3061 {
                            Adt137::C0 => {
                                Err::<(List<Str>, Adt5), Adt137>(Adt137::C0)
                            }
                            Adt137::C1(v4) => {
                                let m3062 = v3.clone();
                                match m3062.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt137>(Adt137::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3063 = f217(v4);
                                        match m3063 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt137>(Adt137::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt137>((v3, v7))
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
                Ok(next) => { m3064 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f766(v0: Adt118) -> Option<Adt116> {
    let m3068 = match v0.clone() {
        Adt118::C0(_, _, _, h3067) => {
            h3067
        }
    };
    match m3068 {
        Adt119::C0 => {
            None::<Adt116>
        }
        Adt119::C1 => {
            Some(Adt116::C10(match v0 {
                Adt118::C0(h3069, _, _, _) => {
                    h3069
                }
            }))
        }
        Adt119::C2(_) => {
            None::<Adt116>
        }
    }
}

pub fn f767(v0: Adt6) -> bool {
    let m3070 = v0;
    match m3070 {
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

pub fn f768(v0: Adt118, v1: Adt6) -> R<Option<Adt116>> {
    let m3072 = match v0.clone() {
        Adt118::C0(_, _, _, h3071) => {
            h3071
        }
    };
    match m3072 {
        Adt119::C0 => {
            Ok(Some(Adt116::C8(match v0 {
                Adt118::C0(h3073, _, _, _) => {
                    h3073
                }
            })))
        }
        Adt119::C1 => {
            Ok(Some(Adt116::C10(match v0 {
                Adt118::C0(h3074, _, _, _) => {
                    h3074
                }
            })))
        }
        Adt119::C2(v2) => {
            if f786(match v0.clone() {
                Adt118::C0(_, _, h3075, _) => {
                    h3075
                }
            }, v2, v1)? {
                Ok(None::<Adt116>)
            } else {
                Ok(Some(Adt116::C9(match v0 {
                    Adt118::C0(h3076, _, _, _) => {
                        h3076
                    }
                })))
            }
        }
    }
}

pub fn f769(p3079: List<Adt41>, p3080: List<Adt6>) -> List<Adt6> {
    let m3078 = (p3079, p3080);
    {
        let mut m3078 = m3078;
        loop {
            let step = {
                let (v0, v1) = m3078;
                let m3077 = v0;
                match m3077.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f678(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f770(v0: Adt71) -> R<Option<Adt6>> {
    let m3081 = v0;
    match m3081 {
        Adt71::C0(v1) => {
            f787(v1)
        }
        Adt71::C1(v2) => {
            let m3082 = f788(v2, List::<Adt6>::nil())?;
            match m3082 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f326(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f771(v0: Adt3) -> Str {
    let m3083 = v0;
    match m3083 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f772(v0: Adt5) -> Option<Adt64> {
    let m3084 = f686(Str::lit("message"), v0.clone());
    match m3084 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m3085 = f199(Str::lit("mode"), v0.clone());
            match m3085 {
                Adt137::C0 => {
                    let m3086 = f632(Str::lit("requestedSchema"), v0);
                    match m3086 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v2) => {
                            Some(Adt64::C0(Adt65::C0(Adt66::C0(None::<Adt67>, v1, v2))))
                        }
                    }
                }
                Adt137::C1(v3) => {
                    let m3087 = f214(v3);
                    match m3087 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v4) => {
                            let a3088 = v4.clone();
                            let a3089 = Str::lit("url");
                            if equal(a3088, a3089) {
                                let m3090 = f686(Str::lit("url"), v0);
                                match m3090 {
                                    None => {
                                        None::<Adt64>
                                    }
                                    Some(v5) => {
                                        Some(Adt64::C0(Adt65::C1(Adt68::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a3091 = v4;
                                let a3092 = Str::lit("form");
                                if equal(a3091, a3092) {
                                    let m3093 = f632(Str::lit("requestedSchema"), v0);
                                    match m3093 {
                                        None => {
                                            None::<Adt64>
                                        }
                                        Some(v6) => {
                                            Some(Adt64::C0(Adt65::C0(Adt66::C0(Some(Adt67::C0), v1, v6))))
                                        }
                                    }
                                } else {
                                    None::<Adt64>
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f773(v0: Adt5) -> Option<Adt64> {
    let m3094 = f199(Str::lit("params"), v0);
    match m3094 {
        Adt137::C0 => {
            Some(Adt64::C2(None::<Adt5>))
        }
        Adt137::C1(v1) => {
            let m3095 = f217(v1);
            match m3095 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    Some(Adt64::C2(Some(v2)))
                }
            }
        }
    }
}

pub fn f774(v0: u64) -> bool {
    let a3096 = v0;
    let a3097 = 9u64;
    if nat_eq(a3096, a3097) {
        true
    } else {
        let a3098 = v0;
        let a3099 = 10u64;
        if nat_eq(a3098, a3099) {
            true
        } else {
            let a3100 = v0;
            let a3101 = 13u64;
            nat_eq(a3100, a3101)
        }
    }
}

pub fn f775(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f778(v0.clone()) {
        Some(f597())
    } else {
        None::<Adt5>
    }, if f777(v0.clone()) {
        Some(f597())
    } else {
        None::<Adt5>
    }, if f776(v0) {
        Some(f597())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f776(p3106: List<(Str, Adt64)>) -> bool {
    let m3105 = p3106;
    {
        let mut m3105 = m3105;
        loop {
            let step = {
                let v0 = m3105;
                let m3102 = v0;
                match m3102.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3104 = {
                            let (_, h3103) = v1;
                            h3103
                        };
                        match m3104 {
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
                Ok(next) => { m3105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f777(p3111: List<(Str, Adt64)>) -> bool {
    let m3110 = p3111;
    {
        let mut m3110 = m3110;
        loop {
            let step = {
                let v0 = m3110;
                let m3107 = v0;
                match m3107.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3109 = {
                            let (_, h3108) = v1;
                            h3108
                        };
                        match m3109 {
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
                Ok(next) => { m3110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f778(p3116: List<(Str, Adt64)>) -> bool {
    let m3115 = p3116;
    {
        let mut m3115 = m3115;
        loop {
            let step = {
                let v0 = m3115;
                let m3112 = v0;
                match m3112.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3114 = {
                            let (_, h3113) = v1;
                            h3113
                        };
                        match m3114 {
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
                Ok(next) => { m3115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f779() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f18(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f780(p3119: List<Adt132>, p3120: List<Adt132>) -> List<Adt132> {
    let m3118 = (p3119, p3120);
    {
        let mut m3118 = m3118;
        loop {
            let step = {
                let (v0, v1) = m3118;
                let m3117 = v0;
                match m3117.uncons() {
                    None => {
                        Err::<(List<Adt132>, List<Adt132>), List<Adt132>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt132>, List<Adt132>), List<Adt132>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f781(v0: Adt19, p3123: List<Adt50>, p3124: List<Adt132>) -> List<Adt132> {
    let m3122 = (p3123, p3124);
    {
        let mut m3122 = m3122;
        loop {
            let step = {
                let (v1, v2) = m3122;
                let m3121 = v1;
                match m3121.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt132>), List<Adt132>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt132>), List<Adt132>>((v4, List::cons(Adt132::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f782(v0: Bytes, v1: Adt141, v2: Str, v3: Adt81) -> R<Result<Adt32, Adt89>> {
    if f725(v1.clone()) {
        if f789(v3.clone()) {
            let m3125 = f790(v0, f509(v1), v2, v3)?;
            match m3125 {
                Ok(v5) => {
                    Ok(Ok::<Adt32, Adt89>(Adt32::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt32, Adt89>(v4))
                }
            }
        } else {
            Ok(Err::<Adt32, Adt89>(Adt89::C13))
        }
    } else {
        Ok(Err::<Adt32, Adt89>(Adt89::C12(f509(v1))))
    }
}

pub fn f783(v0: Result<Option<Str>, Adt89>, v1: bool) -> Result<Option<Str>, Adt89> {
    let m3126 = v0;
    match m3126 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt89>(v3)
            } else {
                Err::<Option<Str>, Adt89>(Adt89::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt89>(v2)
        }
    }
}

pub fn f784(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt89>> {
    let m3127 = v3;
    match m3127 {
        None => {
            Ok(Ok::<Option<Str>, Adt89>(None::<Str>))
        }
        Some(v4) => {
            Ok(f791(f792(v0, v1, v2, v4)?))
        }
    }
}

pub fn f785(v0: Option<Adt5>) -> bool {
    let m3128 = v0;
    match m3128 {
        None => {
            true
        }
        Some(v1) => {
            f793(match v1 {
                Adt5::C0(h3129) => {
                    h3129
                }
            })
        }
    }
}

pub fn f786(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3130 = v0;
    match m3130 {
        Adt13::C0 => {
            let m3131 = f214(v2);
            match m3131 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3132 = v1;
                    let a3133 = v3;
                    Ok(equal(a3132, a3133))
                }
            }
        }
        Adt13::C1 => {
            f794(v1, v2)
        }
        Adt13::C2 => {
            let m3134 = f216(v2);
            match m3134 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3135 = v1;
                        let a3136 = Str::lit("true");
                        Ok(equal(a3135, a3136))
                    } else {
                        let a3137 = v1;
                        let a3138 = Str::lit("false");
                        Ok(equal(a3137, a3138))
                    }
                }
            }
        }
    }
}

pub fn f787(v0: Adt72) -> R<Option<Adt6>> {
    let m3139 = v0;
    match m3139 {
        Adt72::C0(v1) => {
            Ok(Some(f671(v1)))
        }
        Adt72::C1(v2) => {
            Ok(Some(f672(Str::lit("image"), match v2.clone() {
                Adt42::C0(h3140, _, _, _) => {
                    h3140
                }
            }, match v2.clone() {
                Adt42::C0(_, h3141, _, _) => {
                    h3141
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h3142, _) => {
                    h3142
                }
            }, match v2 {
                Adt42::C0(_, _, _, h3143) => {
                    h3143
                }
            })))
        }
        Adt72::C2(v3) => {
            Ok(Some(f672(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h3144, _, _, _) => {
                    h3144
                }
            }, match v3.clone() {
                Adt43::C0(_, h3145, _, _) => {
                    h3145
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h3146, _) => {
                    h3146
                }
            }, match v3 {
                Adt43::C0(_, _, _, h3147) => {
                    h3147
                }
            })))
        }
        Adt72::C3(v4) => {
            Ok(Some(f795(v4)))
        }
        Adt72::C4(v5) => {
            f796(v5)
        }
    }
}

pub fn f788(p3151: List<Adt72>, p3152: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3150 = (p3151, p3152);
    {
        let mut m3150 = m3150;
        loop {
            let step = {
                let (v0, v1) = m3150;
                let m3148 = v0;
                match m3148.uncons() {
                    None => {
                        Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3149 = f787(v2)?;
                        match m3149 {
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
                Ok(next) => { m3150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f789(v0: Adt81) -> bool {
    let m3153 = v0;
    match m3153 {
        Adt81::C0(v1) => {
            let m3154 = v1;
            match m3154.uncons() {
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

pub fn f790(v0: Bytes, v1: Str, v2: Str, v3: Adt81) -> R<Result<Adt81, Adt89>> {
    let m3155 = v3.clone();
    match m3155 {
        Adt81::C0(_) => {
            Ok(Ok::<Adt81, Adt89>(v3))
        }
        Adt81::C1(v5) => {
            Ok(f797(f798(v0, v1, v2, v5)?, List::<(Str, Adt64)>::nil()))
        }
        Adt81::C2(v6, v7) => {
            Ok(f797(f798(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f791(v0: Option<Str>) -> Result<Option<Str>, Adt89> {
    let m3156 = v0;
    match m3156 {
        None => {
            Err::<Option<Str>, Adt89>(Adt89::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt89>(Some(v1))
        }
    }
}

pub fn f792(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3160 = f799(v0, {
        let a3157 = v1;
        utf8_encode(a3157)
    }, {
        let a3158 = v2;
        utf8_encode(a3158)
    }, {
        let a3159 = v3;
        utf8_encode(a3159)
    })?;
    match m3160 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3161 = v4;
            Ok(utf8_decode(a3161))
        }
    }
}

pub fn f793(p3166: List<(Str, Adt6)>) -> bool {
    let m3165 = p3166;
    {
        let mut m3165 = m3165;
        loop {
            let step = {
                let v0 = m3165;
                let m3162 = v0;
                match m3162.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3164 = f217({
                            let (_, h3163) = v1;
                            h3163
                        });
                        match m3164 {
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
                Ok(next) => { m3165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f794(v0: Str, v1: Adt6) -> R<bool> {
    let m3167 = f441(v1);
    match m3167 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3168 = f263(v2)?;
            match m3168 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3170 = f800(f100({
                        let a3169 = v0;
                        utf8_encode(a3169)
                    })?)?;
                    match m3170 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f426({
                                let a3171 = v3;
                                let a3172 = v4;
                                int_sub(a3171, a3172)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f795(v0: Adt73) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt73::C0(h3173, _, _, _) => {
            h3173
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt73::C0(_, h3174, _, _) => {
            h3174
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3177 = match v0.clone() {
            Adt73::C0(_, _, h3176, _) => {
                h3176
            }
        };
        match m3177 {
            Adt5::C0(h3175) => {
                h3175
            }
        }
    })), f670(match v0 {
        Adt73::C0(_, _, _, h3178) => {
            h3178
        }
    }))))))
}

pub fn f796(v0: Adt74) -> R<Option<Adt6>> {
    let m3180 = f467(match v0.clone() {
        Adt74::C0(_, h3179, _, _, _) => {
            h3179
        }
    })?;
    match m3180 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt74::C0(h3181, _, _, _, _) => {
                    h3181
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3187 = f468(Str::lit("structuredContent"), match v0.clone() {
                    Adt74::C0(_, _, h3182, _, _) => {
                        h3182
                    }
                });
                let a3188 = {
                    let a3185 = f469(Str::lit("isError"), match v0.clone() {
                        Adt74::C0(_, _, _, h3183, _) => {
                            h3183
                        }
                    });
                    let a3186 = f670(match v0 {
                        Adt74::C0(_, _, _, _, h3184) => {
                            h3184
                        }
                    });
                    append_list(a3185, a3186)
                };
                append_list(a3187, a3188)
            }))))))
        }
    }
}

pub fn f797(v0: Option<Str>, v1: List<(Str, Adt64)>) -> Result<Adt81, Adt89> {
    let m3189 = v0;
    match m3189 {
        None => {
            Err::<Adt81, Adt89>(Adt89::C14)
        }
        Some(v2) => {
            let m3190 = v1.clone();
            match m3190.uncons() {
                None => {
                    Ok::<Adt81, Adt89>(Adt81::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt81, Adt89>(Adt81::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f798(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3194 = f801(v0, {
        let a3191 = v1;
        utf8_encode(a3191)
    }, {
        let a3192 = v2;
        utf8_encode(a3192)
    }, {
        let a3193 = v3;
        utf8_encode(a3193)
    })?;
    match m3194 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3195 = v4;
            Ok(utf8_decode(a3195))
        }
    }
}

pub fn f799(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3197 = f802();
    let a3198 = {
        let a3196 = v0.clone();
        length_bytes(a3196)
    };
    if nat_le(a3197, a3198) {
        let a3200 = f803(v3.clone())?;
        let a3201 = {
            let a3199 = v3.clone();
            length_bytes(a3199)
        };
        if nat_eq(a3200, a3201) {
            Ok(None::<Bytes>)
        } else {
            f804(v0, v1, v2, v3.clone(), f803(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f800(v0: List<u64>) -> R<Option<i64>> {
    let m3202 = v0.clone();
    match m3202.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3203 = v1;
            let a3204 = 45u64;
            if nat_eq(a3203, a3204) {
                let m3205 = v2.clone();
                match m3205.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3206 = f805(v2, 0i64)?;
                        match m3206 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3207 = v5;
                                    int_neg(a3207)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f805(v0, 0i64)
            }
        }
    }
}

pub fn f801(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3209 = f802();
    let a3210 = {
        let a3208 = v0.clone();
        length_bytes(a3208)
    };
    if nat_le(a3209, a3210) {
        Ok(Some({
            let a3213 = f806(v3.clone())?;
            let a3214 = {
                let a3211 = Bytes::lit(&[0x2e]);
                let a3212 = f806(f807(v0, v1, v2, v3)?)?;
                append_bytes(a3211, a3212)
            };
            append_bytes(a3213, a3214)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f802() -> u64 {
    32u64
}

pub fn f803(v0: Bytes) -> R<u64> {
    let (_, h3215) = f808(Fn4::F809(v0.clone()), {
        let a3216 = v0.clone();
        length_bytes(a3216)
    }, (0u64, {
        let a3217 = v0;
        length_bytes(a3217)
    }))?;
    Ok(h3215)
}

pub fn f804(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3221 = {
        let a3218 = v3.clone();
        let a3219 = 0u64;
        let a3220 = v4;
        slice_bytes(a3218, a3219, a3220)
    };
    match m3221 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3232 = {
                let a3229 = v3.clone();
                let a3230 = {
                    let a3222 = v4;
                    let a3223 = 1u64;
                    nat_add(a3222, a3223)?
                };
                let a3231 = {
                    let a3227 = {
                        let a3224 = v3;
                        length_bytes(a3224)
                    };
                    let a3228 = {
                        let a3225 = v4;
                        let a3226 = 1u64;
                        nat_add(a3225, a3226)?
                    };
                    nat_sub(a3227, a3228)
                };
                slice_bytes(a3229, a3230, a3231)
            };
            match m3232 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f810(v0, v1, v2, f811(v5)?, f811(v6)?)
                }
            }
        }
    }
}

pub fn f805(p3238: List<u64>, p3239: i64) -> R<Option<i64>> {
    let m3237 = (p3238, p3239);
    {
        let mut m3237 = m3237;
        loop {
            let step = {
                let (v0, v1) = m3237;
                let m3233 = v0;
                match m3233.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3234 = f812(v2);
                        match m3234 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f93({
                                    let a3235 = v1;
                                    let a3236 = 10i64;
                                    int_mul(a3235, a3236)?
                                }, f325(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f806(v0: Bytes) -> R<Bytes> {
    let (_, h3240) = f813(Fn5::F814(v0.clone()), {
        let a3241 = v0;
        length_bytes(a3241)
    }, (0u64, f102()))?;
    Ok(h3240)
}

pub fn f807(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f815(v0, {
        let a3242 = f816(v1, v2);
        let a3243 = v3;
        append_bytes(a3242, a3243)
    })
}

pub fn f808(v0: Fn4, p3247: u64, p3248: (u64, u64)) -> R<(u64, u64)> {
    let m3246 = (p3247, p3248);
    {
        let mut m3246 = m3246;
        loop {
            let step = {
                let (v1, v2) = m3246;
                let m3244 = v1;
                if m3244 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3244 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3245: Fn4 = v0.clone();
                        c3245.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f809(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3250 = {
            let (h3249, _) = v1;
            h3249
        };
        let a3251 = 1u64;
        nat_add(a3250, a3251)?
    }, {
        let a3254 = {
            let (_, h3252) = v1;
            h3252
        };
        let a3255 = {
            let a3253 = v0.clone();
            length_bytes(a3253)
        };
        if nat_eq(a3254, a3255) {
            let a3257 = f68(v0, {
                let (h3256, _) = v1;
                h3256
            })?;
            let a3258 = 46u64;
            if nat_eq(a3257, a3258) {
                let (h3259, _) = v1;
                h3259
            } else {
                let (_, h3260) = v1;
                h3260
            }
        } else {
            let (_, h3261) = v1;
            h3261
        }
    }))
}

pub fn f810(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3262 = v3;
    match m3262 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3263 = v4;
            match m3263 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f817(f807(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f811(v0: Bytes) -> R<Option<Bytes>> {
    let a3268 = {
        let a3265 = {
            let a3264 = v0.clone();
            length_bytes(a3264)
        };
        let a3266 = 2u64;
        let a3267 = 0u64;
        nat_rem(a3265, a3266, a3267)
    };
    let a3269 = 0u64;
    if nat_eq(a3268, a3269) {
        let v1: (u64, (Bytes, u64)) = f818(Fn6::F819(v0.clone()), {
            let a3271 = {
                let a3270 = v0;
                length_bytes(a3270)
            };
            let a3272 = 2u64;
            let a3273 = 0u64;
            nat_quot(a3271, a3272, a3273)
        }, (0u64, (f102(), 0u64)))?;
        let a3276 = {
            let (_, h3274) = {
                let (_, h3275) = v1.clone();
                h3275
            };
            h3274
        };
        let a3277 = 0u64;
        if nat_eq(a3276, a3277) {
            Ok(Some({
                let (h3278, _) = {
                    let (_, h3279) = v1;
                    h3279
                };
                h3278
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f812(v0: u64) -> Option<u64> {
    let a3280 = v0;
    let a3281 = 48u64;
    if if nat_lt(a3280, a3281) {
        true
    } else {
        let a3282 = 57u64;
        let a3283 = v0;
        nat_lt(a3282, a3283)
    } {
        None::<u64>
    } else {
        Some({
            let a3284 = v0;
            let a3285 = 48u64;
            nat_sub(a3284, a3285)
        })
    }
}

pub fn f813(v0: Fn5, p3289: u64, p3290: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3288 = (p3289, p3290);
    {
        let mut m3288 = m3288;
        loop {
            let step = {
                let (v1, v2) = m3288;
                let m3286 = v1;
                if m3286 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3286 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3287: Fn5 = v0.clone();
                        c3287.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3288 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f814(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3292 = {
            let (h3291, _) = v1.clone();
            h3291
        };
        let a3293 = 1u64;
        nat_add(a3292, a3293)?
    }, {
        let a3296 = {
            let (_, h3294) = v1;
            h3294
        };
        let a3297 = f820(f68(v0, {
            let (h3295, _) = v1;
            h3295
        })?)?;
        append_bytes(a3296, a3297)
    }))
}

pub fn f815(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f821(f822(v0, v1)?)
}

pub fn f816(v0: Bytes, v1: Bytes) -> Bytes {
    let a3304 = f823({
        let a3298 = v0.clone();
        length_bytes(a3298)
    });
    let a3305 = {
        let a3302 = v0;
        let a3303 = {
            let a3300 = f823({
                let a3299 = v1.clone();
                length_bytes(a3299)
            });
            let a3301 = v1;
            append_bytes(a3300, a3301)
        };
        append_bytes(a3302, a3303)
    };
    append_bytes(a3304, a3305)
}

pub fn f817(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3308 = {
        let a3306 = v0.clone();
        length_bytes(a3306)
    };
    let a3309 = {
        let a3307 = v1.clone();
        length_bytes(a3307)
    };
    if nat_eq(a3308, a3309) {
        let a3310 = f824(v0, v1)?;
        let a3311 = 0u64;
        Ok(nat_eq(a3310, a3311))
    } else {
        Ok(false)
    }
}

pub fn f818(v0: Fn6, p3315: u64, p3316: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3314 = (p3315, p3316);
    {
        let mut m3314 = m3314;
        loop {
            let step = {
                let (v1, v2) = m3314;
                let m3312 = v1;
                if m3312 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3312 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3313: Fn6 = v0.clone();
                        c3313.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f819(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f825(v0, v1)
}

pub fn f820(v0: u64) -> R<Bytes> {
    let a3323 = f826({
        let a3317 = v0;
        let a3318 = 16u64;
        let a3319 = 0u64;
        nat_quot(a3317, a3318, a3319)
    })?;
    let a3324 = f826({
        let a3320 = v0;
        let a3321 = 16u64;
        let a3322 = 0u64;
        nat_rem(a3320, a3321, a3322)
    })?;
    Ok(append_bytes(a3323, a3324))
}

pub fn f821(v0: Adt159) -> R<Bytes> {
    let a3345 = f827(match v0.clone() {
        Adt159::C0(h3325, _, _, _, _, _, _, _) => {
            h3325
        }
    })?;
    let a3346 = {
        let a3343 = f827(match v0.clone() {
            Adt159::C0(_, h3326, _, _, _, _, _, _) => {
                h3326
            }
        })?;
        let a3344 = {
            let a3341 = f827(match v0.clone() {
                Adt159::C0(_, _, h3327, _, _, _, _, _) => {
                    h3327
                }
            })?;
            let a3342 = {
                let a3339 = f827(match v0.clone() {
                    Adt159::C0(_, _, _, h3328, _, _, _, _) => {
                        h3328
                    }
                })?;
                let a3340 = {
                    let a3337 = f827(match v0.clone() {
                        Adt159::C0(_, _, _, _, h3329, _, _, _) => {
                            h3329
                        }
                    })?;
                    let a3338 = {
                        let a3335 = f827(match v0.clone() {
                            Adt159::C0(_, _, _, _, _, h3330, _, _) => {
                                h3330
                            }
                        })?;
                        let a3336 = {
                            let a3333 = f827(match v0.clone() {
                                Adt159::C0(_, _, _, _, _, _, h3331, _) => {
                                    h3331
                                }
                            })?;
                            let a3334 = f827(match v0 {
                                Adt159::C0(_, _, _, _, _, _, _, h3332) => {
                                    h3332
                                }
                            })?;
                            append_bytes(a3333, a3334)
                        };
                        append_bytes(a3335, a3336)
                    };
                    append_bytes(a3337, a3338)
                };
                append_bytes(a3339, a3340)
            };
            append_bytes(a3341, a3342)
        };
        append_bytes(a3343, a3344)
    };
    Ok(append_bytes(a3345, a3346))
}

pub fn f822(v0: Bytes, v1: Bytes) -> R<Adt159> {
    let v2: Adt160 = f828(v0)?;
    let v3: Adt159 = f829(f830(f831(), f832(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f830(f830(f831(), f832(v2, 1549556828u32))?, f833(v3))
}

pub fn f823(v0: u64) -> Bytes {
    let a3372 = f371({
        let a3350 = {
            let a3347 = v0;
            let a3348 = 16777216u64;
            let a3349 = 0u64;
            nat_quot(a3347, a3348, a3349)
        };
        let a3351 = 256u64;
        let a3352 = 0u64;
        nat_rem(a3350, a3351, a3352)
    });
    let a3373 = {
        let a3370 = f371({
            let a3356 = {
                let a3353 = v0;
                let a3354 = 65536u64;
                let a3355 = 0u64;
                nat_quot(a3353, a3354, a3355)
            };
            let a3357 = 256u64;
            let a3358 = 0u64;
            nat_rem(a3356, a3357, a3358)
        });
        let a3371 = {
            let a3368 = f371({
                let a3362 = {
                    let a3359 = v0;
                    let a3360 = 256u64;
                    let a3361 = 0u64;
                    nat_quot(a3359, a3360, a3361)
                };
                let a3363 = 256u64;
                let a3364 = 0u64;
                nat_rem(a3362, a3363, a3364)
            });
            let a3369 = f371({
                let a3365 = v0;
                let a3366 = 256u64;
                let a3367 = 0u64;
                nat_rem(a3365, a3366, a3367)
            });
            append_bytes(a3368, a3369)
        };
        append_bytes(a3370, a3371)
    };
    append_bytes(a3372, a3373)
}

pub fn f824(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3374) = f808(Fn4::F834(v0.clone(), v1), {
        let a3375 = v0;
        length_bytes(a3375)
    }, (0u64, 0u64))?;
    Ok(h3374)
}

pub fn f825(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f835(f68(v0.clone(), {
        let (h3376, _) = v1.clone();
        h3376
    })?);
    let v3: u64 = f835(f68(v0, {
        let a3378 = {
            let (h3377, _) = v1.clone();
            h3377
        };
        let a3379 = 1u64;
        nat_add(a3378, a3379)?
    })?);
    let a3380 = v2;
    let a3381 = 16u64;
    if if nat_lt(a3380, a3381) {
        let a3382 = v3;
        let a3383 = 16u64;
        nat_lt(a3382, a3383)
    } else {
        false
    } {
        Ok(({
            let a3385 = {
                let (h3384, _) = v1.clone();
                h3384
            };
            let a3386 = 2u64;
            nat_add(a3385, a3386)?
        }, ({
            let a3393 = {
                let (h3387, _) = {
                    let (_, h3388) = v1.clone();
                    h3388
                };
                h3387
            };
            let a3394 = f371({
                let a3391 = {
                    let a3389 = v2;
                    let a3390 = 16u64;
                    nat_mul(a3389, a3390)?
                };
                let a3392 = v3;
                nat_add(a3391, a3392)?
            });
            append_bytes(a3393, a3394)
        }, {
            let (_, h3395) = {
                let (_, h3396) = v1;
                h3396
            };
            h3395
        })))
    } else {
        Ok(({
            let a3398 = {
                let (h3397, _) = v1.clone();
                h3397
            };
            let a3399 = 2u64;
            nat_add(a3398, a3399)?
        }, ({
            let (h3400, _) = {
                let (_, h3401) = v1.clone();
                h3401
            };
            h3400
        }, {
            let a3404 = {
                let (_, h3402) = {
                    let (_, h3403) = v1;
                    h3403
                };
                h3402
            };
            let a3405 = 1u64;
            nat_add(a3404, a3405)?
        })))
    }
}

pub fn f826(v0: u64) -> R<Bytes> {
    Ok(f371({
        let a3406 = v0;
        let a3407 = 10u64;
        if nat_lt(a3406, a3407) {
            let a3408 = 48u64;
            let a3409 = v0;
            nat_add(a3408, a3409)?
        } else {
            let a3410 = 87u64;
            let a3411 = v0;
            nat_add(a3410, a3411)?
        }
    }))
}

pub fn f827(v0: u32) -> R<Bytes> {
    let a3416 = f836(v0, 24u32)?;
    let a3417 = {
        let a3414 = f836(v0, 16u32)?;
        let a3415 = {
            let a3412 = f836(v0, 8u32)?;
            let a3413 = f836(v0, 0u32)?;
            append_bytes(a3412, a3413)
        };
        append_bytes(a3414, a3415)
    };
    Ok(append_bytes(a3416, a3417))
}

pub fn f828(v0: Bytes) -> R<Adt160> {
    let a3419 = f837();
    let a3420 = {
        let a3418 = v0.clone();
        length_bytes(a3418)
    };
    if nat_lt(a3419, a3420) {
        Ok(f838(f839(v0)?))
    } else {
        f840(v0)
    }
}

pub fn f829(v0: Adt159, v1: u64, v2: Bytes) -> R<Adt159> {
    let v3: u64 = {
        let a3421 = v2.clone();
        length_bytes(a3421)
    };
    let v4: u64 = {
        let a3427 = {
            let a3424 = {
                let a3422 = v3;
                let a3423 = 72u64;
                nat_add(a3422, a3423)?
            };
            let a3425 = 64u64;
            let a3426 = 0u64;
            nat_quot(a3424, a3425, a3426)
        };
        let a3428 = 64u64;
        nat_mul(a3427, a3428)?
    };
    let (_, h3429) = f841(Fn7::F842(v2, v1, v3, v4), {
        let a3430 = v4;
        let a3431 = 64u64;
        let a3432 = 0u64;
        nat_quot(a3430, a3431, a3432)
    }, (0u64, v0))?;
    Ok(h3429)
}

pub fn f830(v0: Adt159, v1: Adt160) -> R<Adt159> {
    let v2: Adt161 = f843(Fn8::F844, 64u64, Adt161::C0(match v0.clone() {
        Adt159::C0(h3433, _, _, _, _, _, _, _) => {
            h3433
        }
    }, match v0.clone() {
        Adt159::C0(_, h3434, _, _, _, _, _, _) => {
            h3434
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3435, _, _, _, _, _) => {
            h3435
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3436, _, _, _, _) => {
            h3436
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3437, _, _, _) => {
            h3437
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3438, _, _) => {
            h3438
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3439, _) => {
            h3439
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, _, h3440) => {
            h3440
        }
    }, v1, 0u64))?;
    Ok(Adt159::C0(f845(match v0.clone() {
        Adt159::C0(h3441, _, _, _, _, _, _, _) => {
            h3441
        }
    }, match v2.clone() {
        Adt161::C0(h3442, _, _, _, _, _, _, _, _, _) => {
            h3442
        }
    }), f845(match v0.clone() {
        Adt159::C0(_, h3443, _, _, _, _, _, _) => {
            h3443
        }
    }, match v2.clone() {
        Adt161::C0(_, h3444, _, _, _, _, _, _, _, _) => {
            h3444
        }
    }), f845(match v0.clone() {
        Adt159::C0(_, _, h3445, _, _, _, _, _) => {
            h3445
        }
    }, match v2.clone() {
        Adt161::C0(_, _, h3446, _, _, _, _, _, _, _) => {
            h3446
        }
    }), f845(match v0.clone() {
        Adt159::C0(_, _, _, h3447, _, _, _, _) => {
            h3447
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, h3448, _, _, _, _, _, _) => {
            h3448
        }
    }), f845(match v0.clone() {
        Adt159::C0(_, _, _, _, h3449, _, _, _) => {
            h3449
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, h3450, _, _, _, _, _) => {
            h3450
        }
    }), f845(match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3451, _, _) => {
            h3451
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, h3452, _, _, _, _) => {
            h3452
        }
    }), f845(match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3453, _) => {
            h3453
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, _, h3454, _, _, _) => {
            h3454
        }
    }), f845(match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3455) => {
            h3455
        }
    }, match v2 {
        Adt161::C0(_, _, _, _, _, _, _, h3456, _, _) => {
            h3456
        }
    })))
}

pub fn f831() -> Adt159 {
    Adt159::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f832(v0: Adt160, v1: u32) -> Adt160 {
    Adt160::C0({
        let a3458 = match v0.clone() {
            Adt160::C0(h3457, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3457
            }
        };
        let a3459 = v1;
        fixed_u32::bit_xor(a3458, a3459)
    }, {
        let a3461 = match v0.clone() {
            Adt160::C0(_, h3460, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3460
            }
        };
        let a3462 = v1;
        fixed_u32::bit_xor(a3461, a3462)
    }, {
        let a3464 = match v0.clone() {
            Adt160::C0(_, _, h3463, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3463
            }
        };
        let a3465 = v1;
        fixed_u32::bit_xor(a3464, a3465)
    }, {
        let a3467 = match v0.clone() {
            Adt160::C0(_, _, _, h3466, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3466
            }
        };
        let a3468 = v1;
        fixed_u32::bit_xor(a3467, a3468)
    }, {
        let a3470 = match v0.clone() {
            Adt160::C0(_, _, _, _, h3469, _, _, _, _, _, _, _, _, _, _, _) => {
                h3469
            }
        };
        let a3471 = v1;
        fixed_u32::bit_xor(a3470, a3471)
    }, {
        let a3473 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, h3472, _, _, _, _, _, _, _, _, _, _) => {
                h3472
            }
        };
        let a3474 = v1;
        fixed_u32::bit_xor(a3473, a3474)
    }, {
        let a3476 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, h3475, _, _, _, _, _, _, _, _, _) => {
                h3475
            }
        };
        let a3477 = v1;
        fixed_u32::bit_xor(a3476, a3477)
    }, {
        let a3479 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, h3478, _, _, _, _, _, _, _, _) => {
                h3478
            }
        };
        let a3480 = v1;
        fixed_u32::bit_xor(a3479, a3480)
    }, {
        let a3482 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, h3481, _, _, _, _, _, _, _) => {
                h3481
            }
        };
        let a3483 = v1;
        fixed_u32::bit_xor(a3482, a3483)
    }, {
        let a3485 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, h3484, _, _, _, _, _, _) => {
                h3484
            }
        };
        let a3486 = v1;
        fixed_u32::bit_xor(a3485, a3486)
    }, {
        let a3488 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, h3487, _, _, _, _, _) => {
                h3487
            }
        };
        let a3489 = v1;
        fixed_u32::bit_xor(a3488, a3489)
    }, {
        let a3491 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h3490, _, _, _, _) => {
                h3490
            }
        };
        let a3492 = v1;
        fixed_u32::bit_xor(a3491, a3492)
    }, {
        let a3494 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3493, _, _, _) => {
                h3493
            }
        };
        let a3495 = v1;
        fixed_u32::bit_xor(a3494, a3495)
    }, {
        let a3497 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3496, _, _) => {
                h3496
            }
        };
        let a3498 = v1;
        fixed_u32::bit_xor(a3497, a3498)
    }, {
        let a3500 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3499, _) => {
                h3499
            }
        };
        let a3501 = v1;
        fixed_u32::bit_xor(a3500, a3501)
    }, {
        let a3503 = match v0 {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3502) => {
                h3502
            }
        };
        let a3504 = v1;
        fixed_u32::bit_xor(a3503, a3504)
    })
}

pub fn f833(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3505, _, _, _, _, _, _, _) => {
            h3505
        }
    }, match v0.clone() {
        Adt159::C0(_, h3506, _, _, _, _, _, _) => {
            h3506
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3507, _, _, _, _, _) => {
            h3507
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3508, _, _, _, _) => {
            h3508
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3509, _, _, _) => {
            h3509
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3510, _, _) => {
            h3510
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3511, _) => {
            h3511
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3512) => {
            h3512
        }
    }, 2147483648u32, f846(), f846(), f846(), f846(), f846(), f846(), 768u32)
}

pub fn f834(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3514 = {
            let (h3513, _) = v2;
            h3513
        };
        let a3515 = 1u64;
        nat_add(a3514, a3515)?
    }, {
        let a3567 = {
            let (_, h3516) = v2;
            h3516
        };
        let a3568 = {
            let v3: u8 = {
                let a3519 = f847(v0, {
                    let (h3517, _) = v2;
                    h3517
                });
                let a3520 = f847(v1, {
                    let (h3518, _) = v2;
                    h3518
                });
                fixed_u8::bit_xor(a3519, a3520)
            };
            let a3565 = {
                let a3541 = {
                    let a3529 = {
                        let a3523 = {
                            let a3521 = v3;
                            let a3522 = 128u8;
                            fixed_u8::bit_and(a3521, a3522)
                        };
                        let a3524 = 128u8;
                        if equal(a3523, a3524) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3530 = {
                        let a3527 = {
                            let a3525 = v3;
                            let a3526 = 64u8;
                            fixed_u8::bit_and(a3525, a3526)
                        };
                        let a3528 = 64u8;
                        if equal(a3527, a3528) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3529, a3530)?
                };
                let a3542 = {
                    let a3539 = {
                        let a3533 = {
                            let a3531 = v3;
                            let a3532 = 32u8;
                            fixed_u8::bit_and(a3531, a3532)
                        };
                        let a3534 = 32u8;
                        if equal(a3533, a3534) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3540 = {
                        let a3537 = {
                            let a3535 = v3;
                            let a3536 = 16u8;
                            fixed_u8::bit_and(a3535, a3536)
                        };
                        let a3538 = 16u8;
                        if equal(a3537, a3538) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3539, a3540)?
                };
                nat_add(a3541, a3542)?
            };
            let a3566 = {
                let a3563 = {
                    let a3551 = {
                        let a3545 = {
                            let a3543 = v3;
                            let a3544 = 8u8;
                            fixed_u8::bit_and(a3543, a3544)
                        };
                        let a3546 = 8u8;
                        if equal(a3545, a3546) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3552 = {
                        let a3549 = {
                            let a3547 = v3;
                            let a3548 = 4u8;
                            fixed_u8::bit_and(a3547, a3548)
                        };
                        let a3550 = 4u8;
                        if equal(a3549, a3550) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3551, a3552)?
                };
                let a3564 = {
                    let a3561 = {
                        let a3555 = {
                            let a3553 = v3;
                            let a3554 = 2u8;
                            fixed_u8::bit_and(a3553, a3554)
                        };
                        let a3556 = 2u8;
                        if equal(a3555, a3556) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3562 = {
                        let a3559 = {
                            let a3557 = v3;
                            let a3558 = 1u8;
                            fixed_u8::bit_and(a3557, a3558)
                        };
                        let a3560 = 1u8;
                        if equal(a3559, a3560) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3561, a3562)?
                };
                nat_add(a3563, a3564)?
            };
            nat_add(a3565, a3566)?
        };
        nat_add(a3567, a3568)?
    }))
}

pub fn f835(v0: u64) -> u64 {
    let a3569 = v0;
    let a3570 = 48u64;
    if nat_lt(a3569, a3570) {
        16u64
    } else {
        let a3571 = v0;
        let a3572 = 58u64;
        if nat_lt(a3571, a3572) {
            let a3573 = v0;
            let a3574 = 48u64;
            nat_sub(a3573, a3574)
        } else {
            let a3575 = v0;
            let a3576 = 97u64;
            if nat_lt(a3575, a3576) {
                16u64
            } else {
                let a3577 = v0;
                let a3578 = 103u64;
                if nat_lt(a3577, a3578) {
                    let a3579 = v0;
                    let a3580 = 87u64;
                    nat_sub(a3579, a3580)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f836(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f371({
        let v2: u32 = {
            let a3581 = f848(v0, v1);
            let a3582 = 255u32;
            fixed_u32::bit_and(a3581, a3582)
        };
        let a3771 = {
            let a3675 = {
                let a3627 = {
                    let a3603 = {
                        let a3591 = {
                            let a3585 = {
                                let a3583 = v2;
                                let a3584 = 2147483648u32;
                                fixed_u32::bit_and(a3583, a3584)
                            };
                            let a3586 = 2147483648u32;
                            if equal(a3585, a3586) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3592 = {
                            let a3589 = {
                                let a3587 = v2;
                                let a3588 = 1073741824u32;
                                fixed_u32::bit_and(a3587, a3588)
                            };
                            let a3590 = 1073741824u32;
                            if equal(a3589, a3590) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3591, a3592)?
                    };
                    let a3604 = {
                        let a3601 = {
                            let a3595 = {
                                let a3593 = v2;
                                let a3594 = 536870912u32;
                                fixed_u32::bit_and(a3593, a3594)
                            };
                            let a3596 = 536870912u32;
                            if equal(a3595, a3596) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3602 = {
                            let a3599 = {
                                let a3597 = v2;
                                let a3598 = 268435456u32;
                                fixed_u32::bit_and(a3597, a3598)
                            };
                            let a3600 = 268435456u32;
                            if equal(a3599, a3600) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3601, a3602)?
                    };
                    nat_add(a3603, a3604)?
                };
                let a3628 = {
                    let a3625 = {
                        let a3613 = {
                            let a3607 = {
                                let a3605 = v2;
                                let a3606 = 134217728u32;
                                fixed_u32::bit_and(a3605, a3606)
                            };
                            let a3608 = 134217728u32;
                            if equal(a3607, a3608) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3614 = {
                            let a3611 = {
                                let a3609 = v2;
                                let a3610 = 67108864u32;
                                fixed_u32::bit_and(a3609, a3610)
                            };
                            let a3612 = 67108864u32;
                            if equal(a3611, a3612) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3613, a3614)?
                    };
                    let a3626 = {
                        let a3623 = {
                            let a3617 = {
                                let a3615 = v2;
                                let a3616 = 33554432u32;
                                fixed_u32::bit_and(a3615, a3616)
                            };
                            let a3618 = 33554432u32;
                            if equal(a3617, a3618) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3624 = {
                            let a3621 = {
                                let a3619 = v2;
                                let a3620 = 16777216u32;
                                fixed_u32::bit_and(a3619, a3620)
                            };
                            let a3622 = 16777216u32;
                            if equal(a3621, a3622) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3623, a3624)?
                    };
                    nat_add(a3625, a3626)?
                };
                nat_add(a3627, a3628)?
            };
            let a3676 = {
                let a3673 = {
                    let a3649 = {
                        let a3637 = {
                            let a3631 = {
                                let a3629 = v2;
                                let a3630 = 8388608u32;
                                fixed_u32::bit_and(a3629, a3630)
                            };
                            let a3632 = 8388608u32;
                            if equal(a3631, a3632) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3638 = {
                            let a3635 = {
                                let a3633 = v2;
                                let a3634 = 4194304u32;
                                fixed_u32::bit_and(a3633, a3634)
                            };
                            let a3636 = 4194304u32;
                            if equal(a3635, a3636) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3637, a3638)?
                    };
                    let a3650 = {
                        let a3647 = {
                            let a3641 = {
                                let a3639 = v2;
                                let a3640 = 2097152u32;
                                fixed_u32::bit_and(a3639, a3640)
                            };
                            let a3642 = 2097152u32;
                            if equal(a3641, a3642) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3648 = {
                            let a3645 = {
                                let a3643 = v2;
                                let a3644 = 1048576u32;
                                fixed_u32::bit_and(a3643, a3644)
                            };
                            let a3646 = 1048576u32;
                            if equal(a3645, a3646) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3647, a3648)?
                    };
                    nat_add(a3649, a3650)?
                };
                let a3674 = {
                    let a3671 = {
                        let a3659 = {
                            let a3653 = {
                                let a3651 = v2;
                                let a3652 = 524288u32;
                                fixed_u32::bit_and(a3651, a3652)
                            };
                            let a3654 = 524288u32;
                            if equal(a3653, a3654) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3660 = {
                            let a3657 = {
                                let a3655 = v2;
                                let a3656 = 262144u32;
                                fixed_u32::bit_and(a3655, a3656)
                            };
                            let a3658 = 262144u32;
                            if equal(a3657, a3658) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3659, a3660)?
                    };
                    let a3672 = {
                        let a3669 = {
                            let a3663 = {
                                let a3661 = v2;
                                let a3662 = 131072u32;
                                fixed_u32::bit_and(a3661, a3662)
                            };
                            let a3664 = 131072u32;
                            if equal(a3663, a3664) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3670 = {
                            let a3667 = {
                                let a3665 = v2;
                                let a3666 = 65536u32;
                                fixed_u32::bit_and(a3665, a3666)
                            };
                            let a3668 = 65536u32;
                            if equal(a3667, a3668) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3669, a3670)?
                    };
                    nat_add(a3671, a3672)?
                };
                nat_add(a3673, a3674)?
            };
            nat_add(a3675, a3676)?
        };
        let a3772 = {
            let a3769 = {
                let a3721 = {
                    let a3697 = {
                        let a3685 = {
                            let a3679 = {
                                let a3677 = v2;
                                let a3678 = 32768u32;
                                fixed_u32::bit_and(a3677, a3678)
                            };
                            let a3680 = 32768u32;
                            if equal(a3679, a3680) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3686 = {
                            let a3683 = {
                                let a3681 = v2;
                                let a3682 = 16384u32;
                                fixed_u32::bit_and(a3681, a3682)
                            };
                            let a3684 = 16384u32;
                            if equal(a3683, a3684) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3685, a3686)?
                    };
                    let a3698 = {
                        let a3695 = {
                            let a3689 = {
                                let a3687 = v2;
                                let a3688 = 8192u32;
                                fixed_u32::bit_and(a3687, a3688)
                            };
                            let a3690 = 8192u32;
                            if equal(a3689, a3690) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3696 = {
                            let a3693 = {
                                let a3691 = v2;
                                let a3692 = 4096u32;
                                fixed_u32::bit_and(a3691, a3692)
                            };
                            let a3694 = 4096u32;
                            if equal(a3693, a3694) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3695, a3696)?
                    };
                    nat_add(a3697, a3698)?
                };
                let a3722 = {
                    let a3719 = {
                        let a3707 = {
                            let a3701 = {
                                let a3699 = v2;
                                let a3700 = 2048u32;
                                fixed_u32::bit_and(a3699, a3700)
                            };
                            let a3702 = 2048u32;
                            if equal(a3701, a3702) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3708 = {
                            let a3705 = {
                                let a3703 = v2;
                                let a3704 = 1024u32;
                                fixed_u32::bit_and(a3703, a3704)
                            };
                            let a3706 = 1024u32;
                            if equal(a3705, a3706) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3707, a3708)?
                    };
                    let a3720 = {
                        let a3717 = {
                            let a3711 = {
                                let a3709 = v2;
                                let a3710 = 512u32;
                                fixed_u32::bit_and(a3709, a3710)
                            };
                            let a3712 = 512u32;
                            if equal(a3711, a3712) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3718 = {
                            let a3715 = {
                                let a3713 = v2;
                                let a3714 = 256u32;
                                fixed_u32::bit_and(a3713, a3714)
                            };
                            let a3716 = 256u32;
                            if equal(a3715, a3716) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3717, a3718)?
                    };
                    nat_add(a3719, a3720)?
                };
                nat_add(a3721, a3722)?
            };
            let a3770 = {
                let a3767 = {
                    let a3743 = {
                        let a3731 = {
                            let a3725 = {
                                let a3723 = v2;
                                let a3724 = 128u32;
                                fixed_u32::bit_and(a3723, a3724)
                            };
                            let a3726 = 128u32;
                            if equal(a3725, a3726) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3732 = {
                            let a3729 = {
                                let a3727 = v2;
                                let a3728 = 64u32;
                                fixed_u32::bit_and(a3727, a3728)
                            };
                            let a3730 = 64u32;
                            if equal(a3729, a3730) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3731, a3732)?
                    };
                    let a3744 = {
                        let a3741 = {
                            let a3735 = {
                                let a3733 = v2;
                                let a3734 = 32u32;
                                fixed_u32::bit_and(a3733, a3734)
                            };
                            let a3736 = 32u32;
                            if equal(a3735, a3736) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3742 = {
                            let a3739 = {
                                let a3737 = v2;
                                let a3738 = 16u32;
                                fixed_u32::bit_and(a3737, a3738)
                            };
                            let a3740 = 16u32;
                            if equal(a3739, a3740) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3741, a3742)?
                    };
                    nat_add(a3743, a3744)?
                };
                let a3768 = {
                    let a3765 = {
                        let a3753 = {
                            let a3747 = {
                                let a3745 = v2;
                                let a3746 = 8u32;
                                fixed_u32::bit_and(a3745, a3746)
                            };
                            let a3748 = 8u32;
                            if equal(a3747, a3748) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3754 = {
                            let a3751 = {
                                let a3749 = v2;
                                let a3750 = 4u32;
                                fixed_u32::bit_and(a3749, a3750)
                            };
                            let a3752 = 4u32;
                            if equal(a3751, a3752) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3753, a3754)?
                    };
                    let a3766 = {
                        let a3763 = {
                            let a3757 = {
                                let a3755 = v2;
                                let a3756 = 2u32;
                                fixed_u32::bit_and(a3755, a3756)
                            };
                            let a3758 = 2u32;
                            if equal(a3757, a3758) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3764 = {
                            let a3761 = {
                                let a3759 = v2;
                                let a3760 = 1u32;
                                fixed_u32::bit_and(a3759, a3760)
                            };
                            let a3762 = 1u32;
                            if equal(a3761, a3762) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3763, a3764)?
                    };
                    nat_add(a3765, a3766)?
                };
                nat_add(a3767, a3768)?
            };
            nat_add(a3769, a3770)?
        };
        nat_add(a3771, a3772)?
    }))
}

pub fn f837() -> u64 {
    64u64
}

pub fn f838(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3773, _, _, _, _, _, _, _) => {
            h3773
        }
    }, match v0.clone() {
        Adt159::C0(_, h3774, _, _, _, _, _, _) => {
            h3774
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3775, _, _, _, _, _) => {
            h3775
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3776, _, _, _, _) => {
            h3776
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3777, _, _, _) => {
            h3777
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3778, _, _) => {
            h3778
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3779, _) => {
            h3779
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3780) => {
            h3780
        }
    }, f846(), f846(), f846(), f846(), f846(), f846(), f846(), f846())
}

pub fn f839(v0: Bytes) -> R<Adt159> {
    let v1: u64 = {
        let a3781 = v0.clone();
        length_bytes(a3781)
    };
    let v2: u64 = {
        let a3787 = {
            let a3784 = {
                let a3782 = v1;
                let a3783 = 72u64;
                nat_add(a3782, a3783)?
            };
            let a3785 = 64u64;
            let a3786 = 0u64;
            nat_quot(a3784, a3785, a3786)
        };
        let a3788 = 64u64;
        nat_mul(a3787, a3788)?
    };
    let (_, h3789) = f841(Fn7::F849(v0, v1, v2), {
        let a3790 = v2;
        let a3791 = 64u64;
        let a3792 = 0u64;
        nat_quot(a3790, a3791, a3792)
    }, (0u64, f831()))?;
    Ok(h3789)
}

pub fn f840(v0: Bytes) -> R<Adt160> {
    Ok(Adt160::C0(f850(v0.clone(), 0u64)?, f850(v0.clone(), 4u64)?, f850(v0.clone(), 8u64)?, f850(v0.clone(), 12u64)?, f850(v0.clone(), 16u64)?, f850(v0.clone(), 20u64)?, f850(v0.clone(), 24u64)?, f850(v0.clone(), 28u64)?, f850(v0.clone(), 32u64)?, f850(v0.clone(), 36u64)?, f850(v0.clone(), 40u64)?, f850(v0.clone(), 44u64)?, f850(v0.clone(), 48u64)?, f850(v0.clone(), 52u64)?, f850(v0.clone(), 56u64)?, f850(v0, 60u64)?))
}

pub fn f841(v0: Fn7, p3796: u64, p3797: (u64, Adt159)) -> R<(u64, Adt159)> {
    let m3795 = (p3796, p3797);
    {
        let mut m3795 = m3795;
        loop {
            let step = {
                let (v1, v2) = m3795;
                let m3793 = v1;
                if m3793 == 0 {
                    Err::<(u64, (u64, Adt159)), R<(u64, Adt159)>>(Ok(v2))
                } else {
                    let v3 = m3793 - 1;
                    Ok::<(u64, (u64, Adt159)), R<(u64, Adt159)>>((v3, {
                        let c3794: Fn7 = v0.clone();
                        c3794.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f842(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3799 = {
            let (h3798, _) = v4.clone();
            h3798
        };
        let a3800 = 64u64;
        nat_add(a3799, a3800)?
    }, f830({
        let (_, h3801) = v4;
        h3801
    }, f851(v0, v2, v3, {
        let a3804 = {
            let a3802 = v1;
            let a3803 = v2;
            nat_add(a3802, a3803)?
        };
        let a3805 = 8u64;
        nat_mul(a3804, a3805)?
    }, {
        let (h3806, _) = v4;
        h3806
    })?)?))
}

pub fn f843(v0: Fn8, p3810: u64, p3811: Adt161) -> R<Adt161> {
    let m3809 = (p3810, p3811);
    {
        let mut m3809 = m3809;
        loop {
            let step = {
                let (v1, v2) = m3809;
                let m3807 = v1;
                if m3807 == 0 {
                    Err::<(u64, Adt161), R<Adt161>>(Ok(v2))
                } else {
                    let v3 = m3807 - 1;
                    Ok::<(u64, Adt161), R<Adt161>>((v3, {
                        let c3808: Fn8 = v0.clone();
                        c3808.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f844(v0: Adt161) -> R<Adt161> {
    f852(v0)
}

pub fn f845(v0: u32, v1: u32) -> u32 {
    let m3814 = {
        let a3812 = f853(v0);
        let a3813 = f853(v1);
        fixed_u64::checked_add(a3812, a3813)
    };
    match m3814 {
        None => {
            0u32
        }
        Some(v2) => {
            f854(v2)
        }
    }
}

pub fn f846() -> u32 {
    0u32
}

pub fn f847(v0: Bytes, v1: u64) -> u8 {
    let m3817 = {
        let a3815 = v0;
        let a3816 = v1;
        index_bytes(a3815, a3816)
    };
    match m3817 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f848(v0: u32, v1: u32) -> u32 {
    let m3820 = {
        let a3818 = v0;
        let a3819 = v1;
        fixed_u32::shift_right(a3818, a3819)
    };
    match m3820 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f849(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3822 = {
            let (h3821, _) = v3.clone();
            h3821
        };
        let a3823 = 64u64;
        nat_add(a3822, a3823)?
    }, f830({
        let (_, h3824) = v3;
        h3824
    }, f855(v0, v1, v2, {
        let (h3825, _) = v3;
        h3825
    })?)?))
}

pub fn f850(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f856(f857(v0.clone(), v1), f857(v0.clone(), {
        let a3826 = v1;
        let a3827 = 1u64;
        nat_add(a3826, a3827)?
    }), f857(v0.clone(), {
        let a3828 = v1;
        let a3829 = 2u64;
        nat_add(a3828, a3829)?
    }), f857(v0, {
        let a3830 = v1;
        let a3831 = 3u64;
        nat_add(a3830, a3831)?
    })))
}

pub fn f851(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt160> {
    Ok(Adt160::C0(f858(v0.clone(), v1, v2, v3, v4)?, f858(v0.clone(), v1, v2, v3, {
        let a3832 = v4;
        let a3833 = 4u64;
        nat_add(a3832, a3833)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3834 = v4;
        let a3835 = 8u64;
        nat_add(a3834, a3835)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3836 = v4;
        let a3837 = 12u64;
        nat_add(a3836, a3837)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3838 = v4;
        let a3839 = 16u64;
        nat_add(a3838, a3839)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3840 = v4;
        let a3841 = 20u64;
        nat_add(a3840, a3841)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3842 = v4;
        let a3843 = 24u64;
        nat_add(a3842, a3843)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3844 = v4;
        let a3845 = 28u64;
        nat_add(a3844, a3845)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3846 = v4;
        let a3847 = 32u64;
        nat_add(a3846, a3847)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3848 = v4;
        let a3849 = 36u64;
        nat_add(a3848, a3849)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3850 = v4;
        let a3851 = 40u64;
        nat_add(a3850, a3851)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3852 = v4;
        let a3853 = 44u64;
        nat_add(a3852, a3853)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3854 = v4;
        let a3855 = 48u64;
        nat_add(a3854, a3855)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3856 = v4;
        let a3857 = 52u64;
        nat_add(a3856, a3857)?
    })?, f858(v0.clone(), v1, v2, v3, {
        let a3858 = v4;
        let a3859 = 56u64;
        nat_add(a3858, a3859)?
    })?, f858(v0, v1, v2, v3, {
        let a3860 = v4;
        let a3861 = 60u64;
        nat_add(a3860, a3861)?
    })?))
}

pub fn f852(v0: Adt161) -> R<Adt161> {
    let v1: u32 = f859(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, _, h3862) => {
            h3862
        }
    });
    let v2: u32 = f845(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, h3863, _, _) => {
            h3863
        }
    }, f845(f860(match v0.clone() {
        Adt161::C0(_, _, _, _, h3864, _, _, _, _, _) => {
            h3864
        }
    }), f845(f861(match v0.clone() {
        Adt161::C0(_, _, _, _, h3865, _, _, _, _, _) => {
            h3865
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3866, _, _, _, _) => {
            h3866
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3867, _, _, _) => {
            h3867
        }
    }), f845(v1, {
        let m3870 = match v0.clone() {
            Adt161::C0(_, _, _, _, _, _, _, _, h3869, _) => {
                h3869
            }
        };
        match m3870 {
            Adt160::C0(h3868, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3868
            }
        }
    }))));
    let v3: u32 = f845(f862(match v0.clone() {
        Adt161::C0(h3871, _, _, _, _, _, _, _, _, _) => {
            h3871
        }
    }), f863(match v0.clone() {
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
    }));
    Ok(Adt161::C0(f845(v2, v3), match v0.clone() {
        Adt161::C0(h3875, _, _, _, _, _, _, _, _, _) => {
            h3875
        }
    }, match v0.clone() {
        Adt161::C0(_, h3876, _, _, _, _, _, _, _, _) => {
            h3876
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3877, _, _, _, _, _, _, _) => {
            h3877
        }
    }, f845(match v0.clone() {
        Adt161::C0(_, _, _, h3878, _, _, _, _, _, _) => {
            h3878
        }
    }, v2), match v0.clone() {
        Adt161::C0(_, _, _, _, h3879, _, _, _, _, _) => {
            h3879
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3880, _, _, _, _) => {
            h3880
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3881, _, _, _) => {
            h3881
        }
    }, f864(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, h3882, _) => {
            h3882
        }
    }), {
        let a3884 = match v0 {
            Adt161::C0(_, _, _, _, _, _, _, _, _, h3883) => {
                h3883
            }
        };
        let a3885 = 1u64;
        nat_add(a3884, a3885)?
    }))
}

pub fn f853(v0: u32) -> u64 {
    let m3887 = {
        let a3886 = v0;
        fixed_u64::convert(i128::from(a3886))
    };
    match m3887 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f854(v0: u64) -> u32 {
    let m3891 = {
        let a3890 = {
            let a3888 = v0;
            let a3889 = 4294967295u64;
            fixed_u64::bit_and(a3888, a3889)
        };
        fixed_u32::convert(i128::from(a3890))
    };
    match m3891 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f855(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt160> {
    Ok(Adt160::C0(f865(v0.clone(), v1, v2, v3)?, f865(v0.clone(), v1, v2, {
        let a3892 = v3;
        let a3893 = 4u64;
        nat_add(a3892, a3893)?
    })?, f865(v0.clone(), v1, v2, {
        let a3894 = v3;
        let a3895 = 8u64;
        nat_add(a3894, a3895)?
    })?, f865(v0.clone(), v1, v2, {
        let a3896 = v3;
        let a3897 = 12u64;
        nat_add(a3896, a3897)?
    })?, f865(v0.clone(), v1, v2, {
        let a3898 = v3;
        let a3899 = 16u64;
        nat_add(a3898, a3899)?
    })?, f865(v0.clone(), v1, v2, {
        let a3900 = v3;
        let a3901 = 20u64;
        nat_add(a3900, a3901)?
    })?, f865(v0.clone(), v1, v2, {
        let a3902 = v3;
        let a3903 = 24u64;
        nat_add(a3902, a3903)?
    })?, f865(v0.clone(), v1, v2, {
        let a3904 = v3;
        let a3905 = 28u64;
        nat_add(a3904, a3905)?
    })?, f865(v0.clone(), v1, v2, {
        let a3906 = v3;
        let a3907 = 32u64;
        nat_add(a3906, a3907)?
    })?, f865(v0.clone(), v1, v2, {
        let a3908 = v3;
        let a3909 = 36u64;
        nat_add(a3908, a3909)?
    })?, f865(v0.clone(), v1, v2, {
        let a3910 = v3;
        let a3911 = 40u64;
        nat_add(a3910, a3911)?
    })?, f865(v0.clone(), v1, v2, {
        let a3912 = v3;
        let a3913 = 44u64;
        nat_add(a3912, a3913)?
    })?, f865(v0.clone(), v1, v2, {
        let a3914 = v3;
        let a3915 = 48u64;
        nat_add(a3914, a3915)?
    })?, f865(v0.clone(), v1, v2, {
        let a3916 = v3;
        let a3917 = 52u64;
        nat_add(a3916, a3917)?
    })?, f865(v0.clone(), v1, v2, {
        let a3918 = v3;
        let a3919 = 56u64;
        nat_add(a3918, a3919)?
    })?, f865(v0, v1, v2, {
        let a3920 = v3;
        let a3921 = 60u64;
        nat_add(a3920, a3921)?
    })?))
}

pub fn f856(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3926 = f866(v0, 24u32);
    let a3927 = {
        let a3924 = f866(v1, 16u32);
        let a3925 = {
            let a3922 = f866(v2, 8u32);
            let a3923 = v3;
            fixed_u32::bit_or(a3922, a3923)
        };
        fixed_u32::bit_or(a3924, a3925)
    };
    fixed_u32::bit_or(a3926, a3927)
}

pub fn f857(v0: Bytes, v1: u64) -> u32 {
    let m3930 = {
        let a3928 = v0;
        let a3929 = v1;
        index_bytes(a3928, a3929)
    };
    match m3930 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3932 = {
                let a3931 = v2;
                fixed_u32::convert(i128::from(a3931))
            };
            match m3932 {
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

pub fn f858(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3935 = {
        let a3933 = v4;
        let a3934 = 8u64;
        nat_add(a3933, a3934)?
    };
    let a3936 = v2;
    if nat_eq(a3935, a3936) {
        Ok(f867(v3, true))
    } else {
        let a3939 = {
            let a3937 = v4;
            let a3938 = 4u64;
            nat_add(a3937, a3938)?
        };
        let a3940 = v2;
        if nat_eq(a3939, a3940) {
            Ok(f867(v3, false))
        } else {
            Ok(f856(f868(v0.clone(), v1, v4), f868(v0.clone(), v1, {
                let a3941 = v4;
                let a3942 = 1u64;
                nat_add(a3941, a3942)?
            }), f868(v0.clone(), v1, {
                let a3943 = v4;
                let a3944 = 2u64;
                nat_add(a3943, a3944)?
            }), f868(v0, v1, {
                let a3945 = v4;
                let a3946 = 3u64;
                nat_add(a3945, a3946)?
            })))
        }
    }
}

pub fn f859(v0: u64) -> u32 {
    let a3947 = v0;
    let a3948 = 32u64;
    if nat_lt(a3947, a3948) {
        let a3949 = v0;
        let a3950 = 16u64;
        if nat_lt(a3949, a3950) {
            let a3951 = v0;
            let a3952 = 8u64;
            if nat_lt(a3951, a3952) {
                let a3953 = v0;
                let a3954 = 4u64;
                if nat_lt(a3953, a3954) {
                    let a3955 = v0;
                    let a3956 = 2u64;
                    if nat_lt(a3955, a3956) {
                        let a3957 = v0;
                        let a3958 = 1u64;
                        if nat_lt(a3957, a3958) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3959 = v0;
                        let a3960 = 3u64;
                        if nat_lt(a3959, a3960) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3961 = v0;
                    let a3962 = 6u64;
                    if nat_lt(a3961, a3962) {
                        let a3963 = v0;
                        let a3964 = 5u64;
                        if nat_lt(a3963, a3964) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3965 = v0;
                        let a3966 = 7u64;
                        if nat_lt(a3965, a3966) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3967 = v0;
                let a3968 = 12u64;
                if nat_lt(a3967, a3968) {
                    let a3969 = v0;
                    let a3970 = 10u64;
                    if nat_lt(a3969, a3970) {
                        let a3971 = v0;
                        let a3972 = 9u64;
                        if nat_lt(a3971, a3972) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3973 = v0;
                        let a3974 = 11u64;
                        if nat_lt(a3973, a3974) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3975 = v0;
                    let a3976 = 14u64;
                    if nat_lt(a3975, a3976) {
                        let a3977 = v0;
                        let a3978 = 13u64;
                        if nat_lt(a3977, a3978) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3979 = v0;
                        let a3980 = 15u64;
                        if nat_lt(a3979, a3980) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3981 = v0;
            let a3982 = 24u64;
            if nat_lt(a3981, a3982) {
                let a3983 = v0;
                let a3984 = 20u64;
                if nat_lt(a3983, a3984) {
                    let a3985 = v0;
                    let a3986 = 18u64;
                    if nat_lt(a3985, a3986) {
                        let a3987 = v0;
                        let a3988 = 17u64;
                        if nat_lt(a3987, a3988) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3989 = v0;
                        let a3990 = 19u64;
                        if nat_lt(a3989, a3990) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3991 = v0;
                    let a3992 = 22u64;
                    if nat_lt(a3991, a3992) {
                        let a3993 = v0;
                        let a3994 = 21u64;
                        if nat_lt(a3993, a3994) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3995 = v0;
                        let a3996 = 23u64;
                        if nat_lt(a3995, a3996) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3997 = v0;
                let a3998 = 28u64;
                if nat_lt(a3997, a3998) {
                    let a3999 = v0;
                    let a4000 = 26u64;
                    if nat_lt(a3999, a4000) {
                        let a4001 = v0;
                        let a4002 = 25u64;
                        if nat_lt(a4001, a4002) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4003 = v0;
                        let a4004 = 27u64;
                        if nat_lt(a4003, a4004) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4005 = v0;
                    let a4006 = 30u64;
                    if nat_lt(a4005, a4006) {
                        let a4007 = v0;
                        let a4008 = 29u64;
                        if nat_lt(a4007, a4008) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4009 = v0;
                        let a4010 = 31u64;
                        if nat_lt(a4009, a4010) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4011 = v0;
        let a4012 = 48u64;
        if nat_lt(a4011, a4012) {
            let a4013 = v0;
            let a4014 = 40u64;
            if nat_lt(a4013, a4014) {
                let a4015 = v0;
                let a4016 = 36u64;
                if nat_lt(a4015, a4016) {
                    let a4017 = v0;
                    let a4018 = 34u64;
                    if nat_lt(a4017, a4018) {
                        let a4019 = v0;
                        let a4020 = 33u64;
                        if nat_lt(a4019, a4020) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4021 = v0;
                        let a4022 = 35u64;
                        if nat_lt(a4021, a4022) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4023 = v0;
                    let a4024 = 38u64;
                    if nat_lt(a4023, a4024) {
                        let a4025 = v0;
                        let a4026 = 37u64;
                        if nat_lt(a4025, a4026) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4027 = v0;
                        let a4028 = 39u64;
                        if nat_lt(a4027, a4028) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4029 = v0;
                let a4030 = 44u64;
                if nat_lt(a4029, a4030) {
                    let a4031 = v0;
                    let a4032 = 42u64;
                    if nat_lt(a4031, a4032) {
                        let a4033 = v0;
                        let a4034 = 41u64;
                        if nat_lt(a4033, a4034) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4035 = v0;
                        let a4036 = 43u64;
                        if nat_lt(a4035, a4036) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4037 = v0;
                    let a4038 = 46u64;
                    if nat_lt(a4037, a4038) {
                        let a4039 = v0;
                        let a4040 = 45u64;
                        if nat_lt(a4039, a4040) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4041 = v0;
                        let a4042 = 47u64;
                        if nat_lt(a4041, a4042) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4043 = v0;
            let a4044 = 56u64;
            if nat_lt(a4043, a4044) {
                let a4045 = v0;
                let a4046 = 52u64;
                if nat_lt(a4045, a4046) {
                    let a4047 = v0;
                    let a4048 = 50u64;
                    if nat_lt(a4047, a4048) {
                        let a4049 = v0;
                        let a4050 = 49u64;
                        if nat_lt(a4049, a4050) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4051 = v0;
                        let a4052 = 51u64;
                        if nat_lt(a4051, a4052) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4053 = v0;
                    let a4054 = 54u64;
                    if nat_lt(a4053, a4054) {
                        let a4055 = v0;
                        let a4056 = 53u64;
                        if nat_lt(a4055, a4056) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4057 = v0;
                        let a4058 = 55u64;
                        if nat_lt(a4057, a4058) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4059 = v0;
                let a4060 = 60u64;
                if nat_lt(a4059, a4060) {
                    let a4061 = v0;
                    let a4062 = 58u64;
                    if nat_lt(a4061, a4062) {
                        let a4063 = v0;
                        let a4064 = 57u64;
                        if nat_lt(a4063, a4064) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4065 = v0;
                        let a4066 = 59u64;
                        if nat_lt(a4065, a4066) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4067 = v0;
                    let a4068 = 62u64;
                    if nat_lt(a4067, a4068) {
                        let a4069 = v0;
                        let a4070 = 61u64;
                        if nat_lt(a4069, a4070) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4071 = v0;
                        let a4072 = 63u64;
                        if nat_lt(a4071, a4072) {
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

pub fn f860(v0: u32) -> u32 {
    let a4075 = {
        let a4073 = f869(v0, 6u32, 26u32);
        let a4074 = f869(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4073, a4074)
    };
    let a4076 = f869(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4075, a4076)
}

pub fn f861(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4082 = {
        let a4077 = v0;
        let a4078 = v1;
        fixed_u32::bit_and(a4077, a4078)
    };
    let a4083 = {
        let a4080 = {
            let a4079 = v0;
            fixed_u32::bit_not(a4079)
        };
        let a4081 = v2;
        fixed_u32::bit_and(a4080, a4081)
    };
    fixed_u32::bit_xor(a4082, a4083)
}

pub fn f862(v0: u32) -> u32 {
    let a4086 = {
        let a4084 = f869(v0, 2u32, 30u32);
        let a4085 = f869(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4084, a4085)
    };
    let a4087 = f869(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4086, a4087)
}

pub fn f863(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4096 = {
        let a4092 = {
            let a4088 = v0;
            let a4089 = v1;
            fixed_u32::bit_and(a4088, a4089)
        };
        let a4093 = {
            let a4090 = v0;
            let a4091 = v2;
            fixed_u32::bit_and(a4090, a4091)
        };
        fixed_u32::bit_xor(a4092, a4093)
    };
    let a4097 = {
        let a4094 = v1;
        let a4095 = v2;
        fixed_u32::bit_and(a4094, a4095)
    };
    fixed_u32::bit_xor(a4096, a4097)
}

pub fn f864(v0: Adt160) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt160::C0(_, h4098, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4098
        }
    }, match v0.clone() {
        Adt160::C0(_, _, h4099, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4099
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, h4100, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4100
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, h4101, _, _, _, _, _, _, _, _, _, _, _) => {
            h4101
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, h4102, _, _, _, _, _, _, _, _, _, _) => {
            h4102
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, h4103, _, _, _, _, _, _, _, _, _) => {
            h4103
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, h4104, _, _, _, _, _, _, _, _) => {
            h4104
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, h4105, _, _, _, _, _, _, _) => {
            h4105
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4106, _, _, _, _, _, _) => {
            h4106
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, h4107, _, _, _, _, _) => {
            h4107
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h4108, _, _, _, _) => {
            h4108
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4109, _, _, _) => {
            h4109
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4110, _, _) => {
            h4110
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4111, _) => {
            h4111
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4112) => {
            h4112
        }
    }, f845(f870(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4113, _) => {
            h4113
        }
    }), f845(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4114, _, _, _, _, _, _) => {
            h4114
        }
    }, f845(f871(match v0.clone() {
        Adt160::C0(_, h4115, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4115
        }
    }), match v0 {
        Adt160::C0(h4116, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4116
        }
    }))))
}

pub fn f865(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4119 = {
        let a4117 = v3;
        let a4118 = 8u64;
        nat_add(a4117, a4118)?
    };
    let a4120 = v2;
    if nat_eq(a4119, a4120) {
        Ok(f872({
            let a4123 = {
                let a4121 = v1;
                let a4122 = 8u64;
                nat_mul(a4121, a4122)?
            };
            let a4124 = 4294967296u64;
            let a4125 = 0u64;
            nat_quot(a4123, a4124, a4125)
        }))
    } else {
        let a4128 = {
            let a4126 = v3;
            let a4127 = 4u64;
            nat_add(a4126, a4127)?
        };
        let a4129 = v2;
        if nat_eq(a4128, a4129) {
            Ok(f872({
                let a4132 = {
                    let a4130 = v1;
                    let a4131 = 8u64;
                    nat_mul(a4130, a4131)?
                };
                let a4133 = 4294967296u64;
                let a4134 = 0u64;
                nat_rem(a4132, a4133, a4134)
            }))
        } else {
            Ok(f856(f868(v0.clone(), v1, v3), f868(v0.clone(), v1, {
                let a4135 = v3;
                let a4136 = 1u64;
                nat_add(a4135, a4136)?
            }), f868(v0.clone(), v1, {
                let a4137 = v3;
                let a4138 = 2u64;
                nat_add(a4137, a4138)?
            }), f868(v0, v1, {
                let a4139 = v3;
                let a4140 = 3u64;
                nat_add(a4139, a4140)?
            })))
        }
    }
}

pub fn f866(v0: u32, v1: u32) -> u32 {
    let m4143 = {
        let a4141 = v0;
        let a4142 = v1;
        fixed_u32::shift_left(a4141, a4142)
    };
    match m4143 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f867(v0: u64, v1: bool) -> u32 {
    if v1 {
        f872({
            let a4144 = v0;
            let a4145 = 4294967296u64;
            let a4146 = 0u64;
            nat_quot(a4144, a4145, a4146)
        })
    } else {
        f872({
            let a4147 = v0;
            let a4148 = 4294967296u64;
            let a4149 = 0u64;
            nat_rem(a4147, a4148, a4149)
        })
    }
}

pub fn f868(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4150 = v2;
    let a4151 = v1;
    if nat_lt(a4150, a4151) {
        let m4154 = {
            let a4152 = v0;
            let a4153 = v2;
            index_bytes(a4152, a4153)
        };
        match m4154 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4156 = {
                    let a4155 = v3;
                    fixed_u32::convert(i128::from(a4155))
                };
                match m4156 {
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
        let a4157 = v2;
        let a4158 = v1;
        if nat_eq(a4157, a4158) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f869(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4159 = f848(v0, v1);
    let a4160 = f866(v0, v2);
    fixed_u32::bit_or(a4159, a4160)
}

pub fn f870(v0: u32) -> u32 {
    let a4163 = {
        let a4161 = f869(v0, 17u32, 15u32);
        let a4162 = f869(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4161, a4162)
    };
    let a4164 = f848(v0, 10u32);
    fixed_u32::bit_xor(a4163, a4164)
}

pub fn f871(v0: u32) -> u32 {
    let a4167 = {
        let a4165 = f869(v0, 7u32, 25u32);
        let a4166 = f869(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4165, a4166)
    };
    let a4168 = f848(v0, 3u32);
    fixed_u32::bit_xor(a4167, a4168)
}

pub fn f872(v0: u64) -> u32 {
    let (_, h4169) = {
        let (_, h4170) = f873(Fn9::F874, 32u64, (v0, (1u32, 0u32)));
        h4170
    };
    h4169
}

pub fn f873(v0: Fn9, p4174: u64, p4175: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4173 = (p4174, p4175);
    {
        let mut m4173 = m4173;
        loop {
            let step = {
                let (v1, v2) = m4173;
                let m4171 = v1;
                if m4171 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4171 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4172: Fn9 = v0.clone();
                        c4172.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f874(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4176, _) = v0;
        h4176
    };
    let v2: u32 = {
        let (h4177, _) = {
            let (_, h4178) = v0;
            h4178
        };
        h4177
    };
    let v3: u32 = {
        let (_, h4179) = {
            let (_, h4180) = v0;
            h4180
        };
        h4179
    };
    ({
        let a4181 = v1;
        let a4182 = 2u64;
        let a4183 = 0u64;
        nat_quot(a4181, a4182, a4183)
    }, (f866(v2, 1u32), {
        let a4187 = {
            let a4184 = v1;
            let a4185 = 2u64;
            let a4186 = 0u64;
            nat_rem(a4184, a4185, a4186)
        };
        let a4188 = 1u64;
        if nat_eq(a4187, a4188) {
            let a4189 = v3;
            let a4190 = v2;
            fixed_u32::bit_or(a4189, a4190)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
