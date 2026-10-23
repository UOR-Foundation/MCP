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
        Adt32::C4(v5) => {
            Ok(Some(f344(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f345(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f346(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f347(v8)?))
        }
        Adt32::C8(v9) => {
            f342(f348(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f349(v10)?))
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
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f350(f351(match v0.clone() {
        Adt123::C0(h1011, _, _) => {
            h1011
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f352({
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
    if f353(v0, match v1.clone() {
        Adt88::C0(h1015, _, _, _) => {
            h1015
        }
    }, v2.clone()) {
        f354(match v1.clone() {
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
    let m1020 = f355(f356(v0))?;
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
            f357()
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
            f358()
        }
        Adt126::C1(_, _) => {
            f359()
        }
        Adt126::C2(_) => {
            f360()
        }
        Adt126::C3(_) => {
            f361()
        }
        Adt126::C4(v6) => {
            f362(v6)
        }
    }
}

pub fn f273(v0: Adt126) -> Str {
    let m1024 = v0;
    match m1024 {
        Adt126::C0(v1) => {
            f363(v1)
        }
        Adt126::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt126::C2(v4) => {
            f364(v4)
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
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f365(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f79(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f366(v15)), List::<(Str, Adt6)>::nil()))))
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
            if f367(v2)? {
                Ok(false)
            } else {
                f138(f140(v1), f368())
            }
        }
    }
}

pub fn f276(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133) -> R<Result<Adt123, Adt126>> {
    f369(v0.clone(), v1, v2, v3.clone(), f370(match v0 {
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
            f371(v3, v0)
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
    f372(v0, v1, v2.clone(), f373(match v2 {
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
            f374(v1)
        }
    }
}

pub fn f282(v0: List<u64>) -> Option<List<u64>> {
    let m1059 = f375(f376(), v0);
    match m1059 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1060 = f375(f377(), f148(v1, List::<u64>::nil()));
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
                        if f378(v4.clone(), v2.clone()) {
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
                            }, f379(v0.clone(), match v4 {
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
                    if f380(v2) {
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
                            if f380({
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
                f381(v0, v1, v4)
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
                Ok(f246(v0, Adt115::C6(Adt116::C0, f382(v2, v4)?, v5)))
            }
        }
    } else {
        if f238(v2, 56320u64, 57343u64) {
            Ok(f246(v0, Adt115::C6(Adt116::C0, f382({
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
    let a1218 = f383();
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
    let m1250 = f384(f385(), v2.clone());
    match m1250 {
        None => {
            let m1251 = f386(f387(), v2.clone());
            match m1251 {
                None => {
                    f388(v0, v1, v2)
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
            f389(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f314(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt104>) -> List<Adt104> {
    f390(Str::lit("properties"), v0.clone(), v1, v2, f390(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f391(Str::lit("items"), v0.clone(), f391(Str::lit("additionalProperties"), v0.clone(), f392(Str::lit("allOf"), v0.clone(), f392(Str::lit("anyOf"), v0.clone(), f392(Str::lit("oneOf"), v0.clone(), f391(Str::lit("not"), v0.clone(), f391(Str::lit("if"), v0.clone(), f391(Str::lit("then"), v0.clone(), f391(Str::lit("else"), v0, v3)))))))))))
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
                            f395(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f393(v3.clone()) {
                        if f394(v3, v1)? {
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
                    f396(v3, v1)
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
            let m1266 = f397(v2, v1)?;
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
            let m1268 = f398(v4);
            match m1268 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v5) => {
                    let m1269 = f398(v3);
                    match m1269 {
                        None => {
                            Ok(Adt136::C0)
                        }
                        Some(v6) => {
                            if f399(v1, f400(v6, v5)?) {
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
            Ok(List::cons((Str::lit("minLength"), f401(Str::lit("minLength"), true, v0.clone(), f402(v4.clone())?)?), List::cons((Str::lit("maxLength"), f401(Str::lit("maxLength"), false, v0, f402(v4)?)?), List::<(Str, Adt136)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f401(Str::lit("minItems"), true, v0.clone(), f403(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f401(Str::lit("maxItems"), false, v0, f403(v5, 0u64)?)?), List::<(Str, Adt136)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f401(Str::lit("minProperties"), true, v0.clone(), f404(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f401(Str::lit("maxProperties"), false, v0, f404(v6, 0u64)?)?), List::<(Str, Adt136)>::nil())))
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
                            f405(v3, v4)
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
                    let m1280 = f406(v0, v5.clone());
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
                            Adt121::C3(f407(match v4 {
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
                            Adt121::C3(f410(match v4 {
                                Adt5::C0(h1290) => {
                                    h1290
                                }
                            }, f409(v0), v3, v2, List::<Adt101>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt121::C3(List::<Adt101>::nil())
                            } else {
                                if f408(match v4 {
                                    Adt5::C0(h1289) => {
                                        h1289
                                    }
                                }, f409(v0)) {
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
                    Ok(Adt121::C3(f411(v4, 0u64, v3, v2, List::<Adt101>::nil())?))
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
                            Adt121::C3(f412(v4, v1, v2, List::<Adt101>::nil()))
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
                                                    let m1304 = f413(v6, v8, v10, v12, f414(v13.clone()))?;
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
                        if f415(f400(match v2.clone() {
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
    let a1316 = f416(f332(), f417(List::<Bytes>::nil(), v1));
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
            let a1326 = f418();
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
                    let a1330 = f418();
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
        let a1353 = f419(match v0.clone() {
            Adt7::C0(h1351, _) => {
                h1351
            }
        })?;
        let a1354 = List::cons(101u64, f419(match v0 {
            Adt7::C0(_, h1352) => {
                h1352
            }
        })?);
        Ok(append_list(a1353, a1354))
    } else {
        f419(match v0 {
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
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f163(f420(v2)?, v1)))
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
    Ok(List::cons((Str::lit("supportedVersions"), f421(match v0.clone() {
        Adt33::C0(_, h1360, _, _) => {
            h1360
        }
    })), List::cons((Str::lit("capabilities"), f422(match v0.clone() {
        Adt33::C0(_, _, h1361, _) => {
            h1361
        }
    })), {
        let a1364 = f423(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1362) => {
                h1362
            }
        });
        let a1365 = f424(match v0 {
            Adt33::C0(h1363, _, _, _) => {
                h1363
            }
        })?;
        append_list(a1364, a1365)
    })))
}

pub fn f341(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f425(match v0.clone() {
        Adt34::C0(_, _, h1366) => {
            h1366
        }
    }))), {
        let a1369 = f423(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1367, _) => {
                h1367
            }
        });
        let a1370 = f424(match v0 {
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
            let m1372 = f355(Adt6::C5(v1))?;
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
    let m1376 = f426(match v0.clone() {
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
                let a1379 = f427(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1377, _) => {
                        h1377
                    }
                });
                let a1380 = f428(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1378) => {
                        h1378
                    }
                });
                append_list(a1379, a1380)
            })))
        }
    }
}

pub fn f344(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f429(match v0.clone() {
        Adt51::C0(_, _, h1381) => {
            h1381
        }
    })?)), {
        let a1384 = f423(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1382, _) => {
                h1382
            }
        });
        let a1385 = f424(match v0 {
            Adt51::C0(h1383, _, _) => {
                h1383
            }
        })?;
        append_list(a1384, a1385)
    }))
}

pub fn f345(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f430(match v0.clone() {
        Adt52::C0(_, h1386) => {
            h1386
        }
    }))), f424(match v0 {
        Adt52::C0(h1387, _) => {
            h1387
        }
    })?))
}

pub fn f346(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f431(match v0.clone() {
        Adt53::C0(_, _, h1388) => {
            h1388
        }
    }))), {
        let a1391 = f423(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1389, _) => {
                h1389
            }
        });
        let a1392 = f424(match v0 {
            Adt53::C0(h1390, _, _) => {
                h1390
            }
        })?;
        append_list(a1391, a1392)
    }))
}

pub fn f347(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f432(match v0.clone() {
        Adt55::C0(_, _, h1393) => {
            h1393
        }
    }))), {
        let a1396 = f423(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1394, _) => {
                h1394
            }
        });
        let a1397 = f424(match v0 {
            Adt55::C0(h1395, _, _) => {
                h1395
            }
        })?;
        append_list(a1396, a1397)
    }))
}

pub fn f348(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1399 = f433(match v0.clone() {
        Adt58::C0(_, h1398) => {
            h1398
        }
    })?;
    match m1399 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1401 = f423(Str::lit("description"), match v0 {
                    Adt58::C0(h1400, _) => {
                        h1400
                    }
                });
                let a1402 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1401, a1402)
            }))
        }
    }
}

pub fn f349(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f365({
        let m1405 = match v0.clone() {
            Adt60::C0(h1404) => {
                h1404
            }
        };
        match m1405 {
            Adt61::C0(h1403, _, _) => {
                h1403
            }
        }
    }))), {
        let a1412 = f434(Str::lit("total"), {
            let m1408 = match v0.clone() {
                Adt60::C0(h1407) => {
                    h1407
                }
            };
            match m1408 {
                Adt61::C0(_, h1406, _) => {
                    h1406
                }
            }
        })?;
        let a1413 = f428(Str::lit("hasMore"), {
            let m1411 = match v0 {
                Adt60::C0(h1410) => {
                    h1410
                }
            };
            match m1411 {
                Adt61::C0(_, _, h1409) => {
                    h1409
                }
            }
        });
        append_list(a1412, a1413)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f350(v0: Adt143) -> Str {
    let m1414 = v0;
    match m1414 {
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

pub fn f351(v0: Adt32) -> Adt143 {
    let m1415 = v0;
    match m1415 {
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

pub fn f352(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1416 = v0;
    match m1416 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f435(), f436(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f353(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f437(v2) {
            let a1417 = v1;
            let a1418 = 200u64;
            nat_eq(a1417, a1418)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f354(v0: u64, v1: List<List<u64>>) -> Adt88 {
    Adt88::C0(v0, f438(), List::<List<u64>>::nil(), f439(v1))
}

pub fn f355(v0: Adt6) -> R<Option<Adt6>> {
    f440(f441(), Adt144::C0(v0), List::<Adt145>::nil())
}

pub fn f356(v0: Adt125) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f125())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f442(match v0.clone() {
        Adt125::C0(h1419, _) => {
            h1419
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1422 = match v0.clone() {
            Adt125::C0(_, h1421) => {
                h1421
            }
        };
        match m1422 {
            Adt50::C0(h1420, _, _) => {
                h1420
            }
        }
    })), {
        let a1429 = f443({
            let m1425 = match v0.clone() {
                Adt125::C0(_, h1424) => {
                    h1424
                }
            };
            match m1425 {
                Adt50::C0(_, h1423, _) => {
                    h1423
                }
            }
        });
        let a1430 = f444({
            let m1428 = match v0 {
                Adt125::C0(_, h1427) => {
                    h1427
                }
            };
            match m1428 {
                Adt50::C0(_, _, h1426) => {
                    h1426
                }
            }
        });
        append_list(a1429, a1430)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f357() -> u64 {
    429u64
}

pub fn f358() -> i64 {
    -32602i64
}

pub fn f359() -> i64 {
    -32022i64
}

pub fn f360() -> i64 {
    -32020i64
}

pub fn f361() -> i64 {
    -32601i64
}

pub fn f362(v0: Adt84) -> i64 {
    let m1431 = v0;
    match m1431 {
        Adt84::C0(_) => {
            f358()
        }
        Adt84::C1(_) => {
            f358()
        }
        Adt84::C2(_) => {
            f358()
        }
        Adt84::C3(_) => {
            f358()
        }
        Adt84::C4(_) => {
            f358()
        }
        Adt84::C5(_) => {
            f358()
        }
        Adt84::C6(_) => {
            f445()
        }
        Adt84::C7(_) => {
            f446()
        }
        Adt84::C8(_) => {
            f447()
        }
        Adt84::C9(_) => {
            f445()
        }
    }
}

pub fn f363(v0: Adt127) -> Str {
    let m1432 = v0;
    match m1432 {
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

pub fn f364(v0: Adt108) -> Str {
    let m1433 = v0;
    match m1433 {
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
            let m1434 = v1;
            match m1434 {
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
            let m1435 = v2;
            match m1435 {
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

pub fn f365(v0: List<Str>) -> List<Adt6> {
    f294(f448(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f366(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1447 = f449(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1436, _, _, _, _) => {
                h1436
            }
        });
        let a1448 = {
            let a1445 = f449(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1437, _, _, _) => {
                    h1437
                }
            });
            let a1446 = {
                let a1443 = f449(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1438, _, _) => {
                        h1438
                    }
                });
                let a1444 = {
                    let a1441 = f449(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1439, _) => {
                            h1439
                        }
                    });
                    let a1442 = f449(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1440) => {
                            h1440
                        }
                    });
                    append_list(a1441, a1442)
                };
                append_list(a1443, a1444)
            };
            append_list(a1445, a1446)
        };
        append_list(a1447, a1448)
    })
}

pub fn f367(p1451: List<List<u64>>) -> R<bool> {
    let m1450 = p1451;
    {
        let mut m1450 = m1450;
        loop {
            let step = {
                let v0 = m1450;
                let m1449 = v0;
                match m1449.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f450(f140(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f369(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133, v4: bool) -> R<Result<Adt123, Adt126>> {
    if v4 {
        f451(v0, v1, v2, f452(match v3.clone() {
            Adt133::C0(_, h1452, _, _) => {
                h1452
            }
        }, match v3.clone() {
            Adt133::C0(_, _, h1453, _) => {
                h1453
            }
        }, match v3 {
            Adt133::C0(_, _, _, h1454) => {
                h1454
            }
        }))
    } else {
        Ok(Err::<Adt123, Adt126>(Adt126::C3(f453(match v3 {
            Adt133::C0(_, h1455, _, _) => {
                h1455
            }
        }))))
    }
}

pub fn f370(v0: Adt4, v1: Adt134) -> bool {
    let m1456 = v1;
    match m1456 {
        Adt134::C0 => {
            true
        }
        Adt134::C1 => {
            f454(match v0 {
                Adt4::C0(_, _, _, _, _, h1457, _) => {
                    h1457
                }
            })
        }
        Adt134::C2 => {
            f454(match v0 {
                Adt4::C0(_, _, _, _, _, h1458, _) => {
                    h1458
                }
            })
        }
        Adt134::C3 => {
            f455(match v0 {
                Adt4::C0(_, _, _, _, h1459, _, _) => {
                    h1459
                }
            })
        }
        Adt134::C4 => {
            f455(match v0 {
                Adt4::C0(_, _, _, _, h1460, _, _) => {
                    h1460
                }
            })
        }
        Adt134::C5 => {
            f455(match v0 {
                Adt4::C0(_, _, _, _, h1461, _, _) => {
                    h1461
                }
            })
        }
        Adt134::C6 => {
            f454(match v0 {
                Adt4::C0(_, _, _, h1462, _, _, _) => {
                    h1462
                }
            })
        }
        Adt134::C7 => {
            f454(match v0 {
                Adt4::C0(_, _, _, h1463, _, _, _) => {
                    h1463
                }
            })
        }
        Adt134::C8 => {
            f456(match v0 {
                Adt4::C0(_, _, h1464, _, _, _, _) => {
                    h1464
                }
            })
        }
        Adt134::C9 => {
            f457(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1465) => {
                    h1465
                }
            }, f458())
        }
        Adt134::C10 => {
            f457(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1466) => {
                    h1466
                }
            }, f458())
        }
        Adt134::C11 => {
            f457(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1467) => {
                    h1467
                }
            }, f458())
        }
    }
}

pub fn f371(v0: Adt139, v1: Adt137) -> Result<Adt133, Adt126> {
    let m1468 = v1;
    match m1468 {
        Adt137::C0 => {
            Err::<Adt133, Adt126>(Adt126::C3(match v0 {
                Adt139::C0(_, h1469, _, _) => {
                    h1469
                }
            }))
        }
        Adt137::C1(v2) => {
            Ok::<Adt133, Adt126>(Adt133::C0(match v0.clone() {
                Adt139::C0(h1470, _, _, _) => {
                    h1470
                }
            }, v2, match v0.clone() {
                Adt139::C0(_, _, h1471, _) => {
                    h1471
                }
            }, match v0 {
                Adt139::C0(_, _, _, h1472) => {
                    h1472
                }
            }))
        }
    }
}

pub fn f372(v0: Option<Adt107>, v1: List<Str>, v2: Adt103, v3: Result<Adt16, Adt127>) -> R<Adt138> {
    let m1473 = v3;
    match m1473 {
        Ok(v5) => {
            Ok(f459(v1, v2.clone(), v5.clone(), f460(v0, f281(match v2.clone() {
                Adt103::C0(_, h1474, _) => {
                    h1474
                }
            }), match v2.clone() {
                Adt103::C0(_, h1475, _) => {
                    h1475
                }
            }, match v5 {
                Adt16::C0(h1476, _, _, _, _, _) => {
                    h1476
                }
            }, f461(match v2 {
                Adt103::C0(_, _, h1477) => {
                    h1477
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt138::C1(Adt126::C0(v4)))
        }
    }
}

pub fn f373(v0: Option<Adt5>) -> Result<Adt16, Adt127> {
    let m1478 = v0;
    match m1478 {
        None => {
            Err::<Adt16, Adt127>(Adt127::C0)
        }
        Some(v1) => {
            let m1479 = f176(Str::lit("_meta"), v1);
            match m1479 {
                Adt130::C0 => {
                    Err::<Adt16, Adt127>(Adt127::C1)
                }
                Adt130::C1(v2) => {
                    let m1480 = f195(v2);
                    match m1480 {
                        None => {
                            Err::<Adt16, Adt127>(Adt127::C4)
                        }
                        Some(v3) => {
                            f462(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f374(v0: Adt134) -> Adt140 {
    let m1481 = v0;
    match m1481 {
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

pub fn f375(p1487: List<u64>, p1488: List<u64>) -> Option<List<u64>> {
    let m1486 = (p1487, p1488);
    {
        let mut m1486 = m1486;
        loop {
            let step = {
                let (v0, v1) = m1486;
                let m1482 = v0;
                match m1482.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1483 = v1;
                        match m1483.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1484 = v4;
                                let a1485 = v2;
                                if nat_eq(a1484, a1485) {
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
                Ok(next) => { m1486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f377() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f378(v0: Adt12, v1: Option<Str>) -> bool {
    let m1489 = v1;
    match m1489 {
        None => {
            false
        }
        Some(v2) => {
            let a1491 = match v0 {
                Adt12::C0(h1490, _, _, _) => {
                    h1490
                }
            };
            let a1492 = v2;
            equal(a1491, a1492)
        }
    }
}

pub fn f379(v0: List<Adt86>, v1: Str) -> R<Adt111> {
    let m1495 = f226(f70(v0, {
        let a1493 = f463();
        let a1494 = f464(v1)?;
        append_list(a1493, a1494)
    })?)?;
    match m1495 {
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
            let m1496 = f280(v2);
            match m1496 {
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

pub fn f380(v0: i64) -> bool {
    let a1503 = {
        let m1499 = {
            let a1497 = v0;
            let a1498 = 0i64;
            compare(a1497, a1498)
        };
        if match m1499 {
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
            let m1502 = {
                let a1500 = 0i64;
                let a1501 = v0;
                compare(a1500, a1501)
            };
            match m1502 {
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
    bool_not(a1503)
}

pub fn f381(v0: Adt114, v1: u64, v2: Str) -> Adt131 {
    let m1505 = match v0.clone() {
        Adt114::C0(_, h1504, _) => {
            h1504
        }
    };
    match m1505.uncons() {
        None => {
            f247(Adt93::C2(v1))
        }
        Some((v3, v4)) => {
            let m1506 = v3;
            match m1506 {
                Adt119::C0(_) => {
                    f247(Adt93::C2(v1))
                }
                Adt119::C1(v6, _) => {
                    Adt131::C1(Adt114::C0(Adt115::C5, List::cons(Adt119::C1(v6, Some(v2)), v4), match v0 {
                        Adt114::C0(_, _, h1507) => {
                            h1507
                        }
                    }))
                }
            }
        }
    }
}

pub fn f382(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1508 = v0;
    let a1509 = 128u64;
    if nat_lt(a1508, a1509) {
        Ok(List::cons(v0, v1))
    } else {
        let a1510 = v0;
        let a1511 = 2048u64;
        if nat_lt(a1510, a1511) {
            Ok(List::cons({
                let a1512 = 128u64;
                let a1513 = f465(v0, 64u64);
                nat_add(a1512, a1513)?
            }, List::cons({
                let a1514 = 192u64;
                let a1515 = f466(v0, 64u64);
                nat_add(a1514, a1515)?
            }, v1)))
        } else {
            let a1516 = v0;
            let a1517 = 65536u64;
            if nat_lt(a1516, a1517) {
                Ok(List::cons({
                    let a1518 = 128u64;
                    let a1519 = f465(v0, 64u64);
                    nat_add(a1518, a1519)?
                }, List::cons({
                    let a1520 = 128u64;
                    let a1521 = f465(f466(v0, 64u64), 64u64);
                    nat_add(a1520, a1521)?
                }, List::cons({
                    let a1522 = 224u64;
                    let a1523 = f466(v0, 4096u64);
                    nat_add(a1522, a1523)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1524 = 128u64;
                    let a1525 = f465(v0, 64u64);
                    nat_add(a1524, a1525)?
                }, List::cons({
                    let a1526 = 128u64;
                    let a1527 = f465(f466(v0, 64u64), 64u64);
                    nat_add(a1526, a1527)?
                }, List::cons({
                    let a1528 = 128u64;
                    let a1529 = f465(f466(v0, 4096u64), 64u64);
                    nat_add(a1528, a1529)?
                }, List::cons({
                    let a1530 = 240u64;
                    let a1531 = f466(v0, 262144u64);
                    nat_add(a1530, a1531)?
                }, v1)))))
            }
        }
    }
}

pub fn f383() -> u64 {
    18u64
}

pub fn f384(p1535: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1534 = p1535;
    {
        let mut m1534 = m1534;
        loop {
            let step = {
                let v0 = m1534;
                let m1532 = v0;
                match m1532.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1533 = f234(v2.clone(), v1.clone());
                        match m1533 {
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
                Ok(next) => { m1534 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f386(p1542: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1541 = p1542;
    {
        let mut m1541 = m1541;
        loop {
            let step = {
                let v0 = m1541;
                let m1536 = v0;
                match m1536.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1538 = f234({
                            let (h1537, _) = v2.clone();
                            h1537
                        }, v1.clone());
                        match m1538 {
                            Adt130::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt130::C1(v4) => {
                                if f467({
                                    let (_, h1539) = v2;
                                    h1539
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1540, _) = v2;
                                        h1540
                                    }))
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

pub fn f387() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f388(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1543 = f234(Str::lit("$ref"), v2);
    match m1543 {
        Adt130::C0 => {
            None::<Adt91>
        }
        Adt130::C1(v3) => {
            let m1544 = f231(v3);
            match m1544 {
                None => {
                    None::<Adt91>
                }
                Some(v4) => {
                    let m1545 = f406(v1, v4.clone());
                    match m1545 {
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

pub fn f389(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt120) -> R<Adt142> {
    if v3 {
        let m1546 = f231(v2);
        match m1546 {
            None => {
                Ok(Adt142::C0(Adt91::C7(v0)))
            }
            Some(v6) => {
                f468(v0, v1, v6.clone(), f113(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt142::C0(Adt91::C10(v0)))
    }
}

pub fn f390(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt104>) -> List<Adt104> {
    let m1547 = f234(v0.clone(), v1);
    match m1547 {
        Adt130::C0 => {
            v4
        }
        Adt130::C1(v5) => {
            let m1548 = f195(v5);
            match m1548 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1549 = v0;
                    let a1550 = Str::lit("properties");
                    if equal(a1549, a1550) {
                        f469(match v6 {
                            Adt5::C0(h1551) => {
                                h1551
                            }
                        }, v2, v3, v4)
                    } else {
                        f470(match v6 {
                            Adt5::C0(h1552) => {
                                h1552
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f391(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt104>) -> List<Adt104> {
    let m1553 = f234(v0, v1);
    match m1553 {
        Adt130::C0 => {
            v2
        }
        Adt130::C1(v3) => {
            List::cons(Adt104::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f392(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt104>) -> List<Adt104> {
    let m1554 = f234(v0, v1);
    match m1554 {
        Adt130::C0 => {
            v2
        }
        Adt130::C1(v3) => {
            let m1555 = f259(v3);
            match m1555 {
                None => {
                    v2
                }
                Some(v4) => {
                    f471(v4, v2)
                }
            }
        }
    }
}

pub fn f393(v0: Str) -> bool {
    let a1556 = v0.clone();
    let a1557 = Str::lit("null");
    if equal(a1556, a1557) {
        true
    } else {
        let a1558 = v0.clone();
        let a1559 = Str::lit("boolean");
        if equal(a1558, a1559) {
            true
        } else {
            let a1560 = v0.clone();
            let a1561 = Str::lit("object");
            if equal(a1560, a1561) {
                true
            } else {
                let a1562 = v0.clone();
                let a1563 = Str::lit("array");
                if equal(a1562, a1563) {
                    true
                } else {
                    let a1564 = v0.clone();
                    let a1565 = Str::lit("number");
                    if equal(a1564, a1565) {
                        true
                    } else {
                        let a1566 = v0.clone();
                        let a1567 = Str::lit("string");
                        if equal(a1566, a1567) {
                            true
                        } else {
                            let a1568 = v0;
                            let a1569 = Str::lit("integer");
                            equal(a1568, a1569)
                        }
                    }
                }
            }
        }
    }
}

pub fn f394(v0: Str, v1: Adt6) -> R<bool> {
    let a1570 = v0.clone();
    let a1571 = f472(v1.clone());
    if equal(a1570, a1571) {
        Ok(true)
    } else {
        let a1572 = v0;
        let a1573 = Str::lit("integer");
        if equal(a1572, a1573) {
            let m1574 = f398(v1);
            match m1574 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f473(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f395(p1577: List<Adt6>, v1: Adt6, p1578: bool) -> R<Adt136> {
    let m1576 = (p1577, p1578);
    {
        let mut m1576 = m1576;
        loop {
            let step = {
                let (v0, v2) = m1576;
                let m1575 = v0;
                match m1575.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f393(f474(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt136>>((v4, if v2 {
                                true
                            } else {
                                f394(f474(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C2))
                        }
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

pub fn f396(p1582: List<Adt6>, v1: Adt6) -> R<Adt136> {
    let m1581 = p1582;
    {
        let mut m1581 = m1581;
        loop {
            let step = {
                let v0 = m1581;
                let m1579 = v0;
                match m1579.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt136>>(Ok(Adt136::C1))
                    }
                    Some((v2, v3)) => {
                        let m1580 = f397(v2, v1.clone())?;
                        match m1580 {
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
                Ok(next) => { m1581 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f475(f476(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f398(v0: Adt6) -> Option<Adt7> {
    let m1583 = v0;
    match m1583 {
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

pub fn f399(v0: u64, v1: Adt147) -> bool {
    let a1584 = v0;
    let a1585 = 0u64;
    if nat_eq(a1584, a1585) {
        let a1586 = f477(v1);
        bool_not(a1586)
    } else {
        let a1587 = v0;
        let a1588 = 1u64;
        if nat_eq(a1587, a1588) {
            let a1589 = f415(v1);
            bool_not(a1589)
        } else {
            let a1590 = v0;
            let a1591 = 2u64;
            if nat_eq(a1590, a1591) {
                f415(v1)
            } else {
                f477(v1)
            }
        }
    }
}

pub fn f400(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f478(v0.clone(), v1.clone(), f479(match v0 {
        Adt7::C0(h1592, _) => {
            h1592
        }
    }), f479(match v1 {
        Adt7::C0(h1593, _) => {
            h1593
        }
    }))
}

pub fn f401(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt136> {
    let m1594 = f234(v0, v2);
    match m1594 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v4) => {
            let m1595 = f480(v4);
            match m1595 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v5) => {
                    let m1598 = {
                        let a1596 = v5;
                        let a1597 = 0i64;
                        compare(a1596, a1597)
                    };
                    if match m1598 {
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
                            if f477(f481(f79(v3, 0i64)?, v5)) {
                                Ok(Adt136::C1)
                            } else {
                                Ok(Adt136::C0)
                            }
                        } else {
                            if f415(f481(f79(v3, 0i64)?, v5)) {
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

pub fn f402(v0: Str) -> R<u64> {
    f482(f87({
        let a1599 = v0;
        utf8_encode(a1599)
    })?, 0u64)
}

pub fn f403(p1604: List<Adt6>, p1605: u64) -> R<u64> {
    let m1603 = (p1604, p1605);
    {
        let mut m1603 = m1603;
        loop {
            let step = {
                let (v0, v1) = m1603;
                let m1600 = v0;
                match m1600.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1601 = v1;
                            let a1602 = 1u64;
                            nat_add(a1601, a1602)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1603 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(p1610: List<(Str, Adt6)>, p1611: u64) -> R<u64> {
    let m1609 = (p1610, p1611);
    {
        let mut m1609 = m1609;
        loop {
            let step = {
                let (v0, v1) = m1609;
                let m1606 = v0;
                match m1606.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1607 = v1;
                            let a1608 = 1u64;
                            nat_add(a1607, a1608)?
                        }))
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

pub fn f405(p1615: List<Adt6>, v1: Adt5) -> Adt136 {
    let m1614 = p1615;
    {
        let mut m1614 = m1614;
        loop {
            let step = {
                let v0 = m1614;
                let m1612 = v0;
                match m1612.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt136>(Adt136::C0)
                    }
                    Some((v2, v3)) => {
                        let m1613 = f231(v2);
                        match m1613 {
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
                Ok(next) => { m1614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1616 = v1.clone();
    let a1617 = Str::lit("#");
    if equal(a1616, a1617) {
        Some(v0)
    } else {
        let m1618 = f195(v0);
        match m1618 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1619 = f176(Str::lit("$defs"), v2);
                match m1619 {
                    Adt130::C0 => {
                        None::<Adt6>
                    }
                    Adt130::C1(v3) => {
                        let m1620 = f195(v3);
                        match m1620 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f483(v1, match v4 {
                                    Adt5::C0(h1621) => {
                                        h1621
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

pub fn f407(p1628: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt99>, p1629: List<Adt101>) -> List<Adt101> {
    let m1627 = (p1628, p1629);
    {
        let mut m1627 = m1627;
        loop {
            let step = {
                let (v0, v3) = m1627;
                let m1622 = v0;
                match m1622.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1624 = f234({
                            let (h1623, _) = v4.clone();
                            h1623
                        }, v1.clone());
                        match m1624 {
                            Adt130::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v5, v3))
                            }
                            Adt130::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v5, List::cons(Adt101::C0({
                                    let (_, h1625) = v4;
                                    h1625
                                }, v6, List::cons(Adt99::C0({
                                    let (h1626, _) = v4;
                                    h1626
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1627 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(p1634: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1633 = p1634;
    {
        let mut m1633 = m1633;
        loop {
            let step = {
                let v0 = m1633;
                let m1630 = v0;
                match m1630.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1632 = f234({
                            let (h1631, _) = v2;
                            h1631
                        }, v1.clone());
                        match m1632 {
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
                Ok(next) => { m1633 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f409(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1635 = f234(Str::lit("properties"), v0);
    match m1635 {
        Adt130::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt130::C1(v1) => {
            let m1636 = f195(v1);
            match m1636 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1637) => {
                            h1637
                        }
                    }
                }
            }
        }
    }
}

pub fn f410(p1644: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>, p1645: List<Adt101>) -> List<Adt101> {
    let m1643 = (p1644, p1645);
    {
        let mut m1643 = m1643;
        loop {
            let step = {
                let (v0, v4) = m1643;
                let m1638 = v0;
                match m1638.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1640 = f234({
                            let (h1639, _) = v5.clone();
                            h1639
                        }, v1.clone());
                        match m1640 {
                            Adt130::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v6, List::cons(Adt101::C0(v2.clone(), {
                                    let (_, h1641) = v5;
                                    h1641
                                }, List::cons(Adt99::C0({
                                    let (h1642, _) = v5;
                                    h1642
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
                Ok(next) => { m1643 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(p1650: List<Adt6>, p1651: u64, v2: Adt6, v3: List<Adt99>, p1652: List<Adt101>) -> R<List<Adt101>> {
    let m1649 = (p1650, (p1651, p1652));
    {
        let mut m1649 = m1649;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1649;
                let m1646 = v0;
                match m1646.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt101>)), R<List<Adt101>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt101>)), R<List<Adt101>>>((v6, ({
                            let a1647 = v1;
                            let a1648 = 1u64;
                            nat_add(a1647, a1648)?
                        }, List::cons(Adt101::C0(v2.clone(), v5, List::cons(Adt99::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(p1655: List<Adt6>, v1: Adt6, v2: List<Adt99>, p1656: List<Adt101>) -> List<Adt101> {
    let m1654 = (p1655, p1656);
    {
        let mut m1654 = m1654;
        loop {
            let step = {
                let (v0, v3) = m1654;
                let m1653 = v0;
                match m1653.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt101>), List<Adt101>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt101>), List<Adt101>>((v5, List::cons(Adt101::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1654 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1657 = f484(v0)?;
    match m1657 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1658 = f484(v1)?;
            match m1658 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1659 = v2;
                    let a1660 = 61u64;
                    if nat_eq(a1659, a1660) {
                        f485(v5, v6, v4, v3)
                    } else {
                        let m1661 = f484(v2)?;
                        match m1661 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1662 = v3;
                                let a1663 = 61u64;
                                if nat_eq(a1662, a1663) {
                                    f486(v5, v6, v7, v4)
                                } else {
                                    let m1664 = f484(v3)?;
                                    match m1664 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f487(v5, v6, v7, v8)?))
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

pub fn f414(v0: List<u64>) -> bool {
    let m1665 = v0;
    match m1665.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f415(v0: Adt147) -> bool {
    let m1666 = v0;
    match m1666 {
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

pub fn f416(p1671: Bytes, p1672: List<Bytes>) -> Bytes {
    let m1670 = (p1671, p1672);
    {
        let mut m1670 = m1670;
        loop {
            let step = {
                let (v0, v1) = m1670;
                let m1667 = v1;
                match m1667.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1668 = v2;
                            let a1669 = v0;
                            append_bytes(a1668, a1669)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(p1675: List<Bytes>, p1676: List<u64>) -> List<Bytes> {
    let m1674 = (p1675, p1676);
    {
        let mut m1674 = m1674;
        loop {
            let step = {
                let (v0, v1) = m1674;
                let m1673 = v1;
                match m1673.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f488(f489(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418() -> i64 {
    999999999999999999i64
}

pub fn f419(v0: i64) -> R<List<u64>> {
    let m1679 = {
        let a1677 = v0;
        let a1678 = 0i64;
        compare(a1677, a1678)
    };
    if match m1679 {
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
        Ok(List::cons(45u64, f490(20u64, {
            let a1680 = v0;
            int_neg(a1680)?
        }, List::<u64>::nil())?))
    } else {
        f490(20u64, v0, List::<u64>::nil())
    }
}

pub fn f420(v0: u64) -> R<List<u64>> {
    let a1681 = v0;
    let a1682 = 34u64;
    if nat_eq(a1681, a1682) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1683 = v0;
        let a1684 = 92u64;
        if nat_eq(a1683, a1684) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1685 = v0;
            let a1686 = 8u64;
            if nat_eq(a1685, a1686) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1687 = v0;
                let a1688 = 9u64;
                if nat_eq(a1687, a1688) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1689 = v0;
                    let a1690 = 10u64;
                    if nat_eq(a1689, a1690) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1691 = v0;
                        let a1692 = 12u64;
                        if nat_eq(a1691, a1692) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1693 = v0;
                            let a1694 = 13u64;
                            if nat_eq(a1693, a1694) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1695 = v0;
                                let a1696 = 32u64;
                                if nat_lt(a1695, a1696) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f491(f466(v0, 16u64))?, List::cons(f491(f465(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f421(v0: List<Str>) -> Adt6 {
    Adt6::C4(f365(v0))
}

pub fn f422(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1714 = f449(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1697, _, _, _, _, _, _) => {
                h1697
            }
        });
        let a1715 = {
            let a1712 = f449(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1698, _, _, _, _, _) => {
                    h1698
                }
            });
            let a1713 = {
                let a1710 = f449(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1699, _, _, _, _) => {
                        h1699
                    }
                });
                let a1711 = {
                    let a1708 = f492(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1700, _, _, _) => {
                            h1700
                        }
                    });
                    let a1709 = {
                        let a1706 = f493(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1701, _, _) => {
                                h1701
                            }
                        });
                        let a1707 = {
                            let a1704 = f492(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1702, _) => {
                                    h1702
                                }
                            });
                            let a1705 = f449(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1703) => {
                                    h1703
                                }
                            });
                            append_list(a1704, a1705)
                        };
                        append_list(a1706, a1707)
                    };
                    append_list(a1708, a1709)
                };
                append_list(a1710, a1711)
            };
            append_list(a1712, a1713)
        };
        append_list(a1714, a1715)
    })
}

pub fn f423(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1716 = v1;
    match m1716 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f424(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f494(match v0.clone() {
        Adt10::C0(h1717, _) => {
            h1717
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f495(match v0 {
        Adt10::C0(_, h1718) => {
            h1718
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f425(v0: List<Adt35>) -> List<Adt6> {
    f294(f496(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f426(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1719 = f497(v0, List::<Adt6>::nil())?;
    match m1719 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f294(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f427(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1720 = v1;
    match m1720 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f428(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1721 = v1;
    match m1721 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f429(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f294(f498(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f430(v0: List<Adt46>) -> List<Adt6> {
    f294(f499(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f431(v0: List<Adt54>) -> List<Adt6> {
    f294(f500(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f432(v0: List<Adt56>) -> List<Adt6> {
    f294(f501(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f433(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m1722 = f502(v0, List::<Adt6>::nil())?;
    match m1722 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f294(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f434(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1723 = v1;
    match m1723 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f494(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f435() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f436(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1724, _, _, _, _, _) => {
            h1724
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1725, _, _, _) => {
            h1725
        }
    })), {
        let a1734 = f423(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1726, _, _, _, _) => {
                h1726
            }
        });
        let a1735 = {
            let a1732 = f423(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1727, _, _) => {
                    h1727
                }
            });
            let a1733 = {
                let a1730 = f423(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1728, _) => {
                        h1728
                    }
                });
                let a1731 = f503(match v0 {
                    Adt1::C0(_, _, _, _, _, h1729) => {
                        h1729
                    }
                });
                append_list(a1730, a1731)
            };
            append_list(a1732, a1733)
        };
        append_list(a1734, a1735)
    })))
}

pub fn f437(v0: List<List<u64>>) -> bool {
    let m1736 = v0;
    match m1736.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f438() -> List<Adt86> {
    List::cons(Adt86::C0(f504(), f368()), List::cons(Adt86::C0(f505(), f506()), List::cons(Adt86::C0(f507(), f508()), List::<Adt86>::nil())))
}

pub fn f439(v0: List<List<u64>>) -> List<u64> {
    f509(f38(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f440(p1751: u64, p1752: Adt144, p1753: List<Adt145>) -> R<Option<Adt6>> {
    let m1750 = (p1751, (p1752, p1753));
    {
        let mut m1750 = m1750;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1750;
                let m1737 = v0;
                if m1737 == 0 {
                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1737 - 1;
                    let m1738 = v1;
                    match m1738 {
                        Adt144::C0(v4) => {
                            let m1739 = v4.clone();
                            match m1739 {
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
                                    let v8: Str = f510(v7)?;
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1740 = v9;
                                    match m1740.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v10), List::cons(Adt145::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1741 = v12;
                                    match m1741.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f510({
                                                let (h1742, _) = v13;
                                                h1742
                                            })?;
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                let (_, h1743) = v13;
                                                h1743
                                            }), List::cons(Adt145::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt144::C1(v16) => {
                            let m1744 = v2;
                            match m1744.uncons() {
                                None => {
                                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1745 = v17;
                                    match m1745 {
                                        Adt145::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1746 = v20;
                                            match m1746.uncons() {
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
                                            let m1747 = v26;
                                            match m1747.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C5(f296(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f510({
                                                        let (h1748, _) = v28;
                                                        h1748
                                                    })?;
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                        let (_, h1749) = v28;
                                                        h1749
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
                Ok(next) => { m1750 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441() -> u64 {
    67108864u64
}

pub fn f442(v0: Adt19) -> Adt6 {
    let m1754 = v0;
    match m1754 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f443(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1755 = v0;
    match m1755 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f444(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1756 = v0;
    match m1756 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f445() -> i64 {
    -32603i64
}

pub fn f446() -> i64 {
    429i64
}

pub fn f447() -> i64 {
    -32021i64
}

pub fn f448(p1759: List<Str>, p1760: List<Adt6>) -> List<Adt6> {
    let m1758 = (p1759, p1760);
    {
        let mut m1758 = m1758;
        loop {
            let step = {
                let (v0, v1) = m1758;
                let m1757 = v0;
                match m1757.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f449(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1761 = v1;
    match m1761 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1762) => {
                    h1762
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f450(v0: List<u64>) -> R<bool> {
    let m1763 = v0;
    match m1763.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1764 = v2;
            match m1764.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1765 = f184(v1)?;
                    let a1766 = 113u64;
                    if if nat_eq(a1765, a1766) {
                        let a1767 = v3;
                        let a1768 = 61u64;
                        nat_eq(a1767, a1768)
                    } else {
                        false
                    } {
                        Ok(f511(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f451(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt127>) -> R<Result<Adt123, Adt126>> {
    let m1769 = v3;
    match m1769 {
        Ok(v5) => {
            f512(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt123, Adt126>(Adt126::C0(v4)))
        }
    }
}

pub fn f452(v0: Adt134, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt127> {
    let m1770 = v0;
    match m1770 {
        Adt134::C0 => {
            f513(v1, v2)
        }
        Adt134::C1 => {
            f514(v1, v2)
        }
        Adt134::C2 => {
            f515(v1, v2)
        }
        Adt134::C3 => {
            f516(v1, v2)
        }
        Adt134::C4 => {
            f517(v1, v2)
        }
        Adt134::C5 => {
            f518(v1, v2)
        }
        Adt134::C6 => {
            f519(v1, v2)
        }
        Adt134::C7 => {
            f520(v1, v2)
        }
        Adt134::C8 => {
            f521(v1, v2)
        }
        Adt134::C9 => {
            f522(v1, v2)
        }
        Adt134::C10 => {
            f523(v1, v2)
        }
        Adt134::C11 => {
            f524(v1, v2)
        }
    }
}

pub fn f453(v0: Adt134) -> Str {
    let m1771 = v0;
    match m1771 {
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

pub fn f454(v0: Option<Adt8>) -> bool {
    let m1772 = v0;
    match m1772 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f455(v0: Option<Adt9>) -> bool {
    let m1773 = v0;
    match m1773 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f456(v0: Option<Adt5>) -> bool {
    let m1774 = v0;
    match m1774 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f457(v0: Option<Adt5>, v1: Str) -> bool {
    let m1775 = v0;
    match m1775 {
        None => {
            false
        }
        Some(v2) => {
            f235(v1, v2)
        }
    }
}

pub fn f458() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f459(v0: List<Str>, v1: Adt103, v2: Adt16, v3: Option<Adt108>) -> Adt138 {
    let m1776 = v3;
    match m1776 {
        None => {
            f525(v1, v2.clone(), f526(match v2 {
                Adt16::C0(h1777, _, _, _, _, _) => {
                    h1777
                }
            }, v0))
        }
        Some(v4) => {
            Adt138::C1(Adt126::C2(v4))
        }
    }
}

pub fn f460(v0: Option<Adt107>, v1: Adt140, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt108>> {
    let m1778 = v0;
    match m1778 {
        None => {
            Ok(None::<Adt108>)
        }
        Some(v5) => {
            let m1780 = match v5.clone() {
                Adt107::C0(_, _, _, h1779, _) => {
                    h1779
                }
            };
            match m1780 {
                None => {
                    f527(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f461(v0: Option<Adt5>) -> Adt5 {
    let m1781 = v0;
    match m1781 {
        None => {
            f528()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f462(v0: Adt5) -> Result<Adt16, Adt127> {
    let m1782 = f529(v0.clone());
    match m1782 {
        Adt148::C0 => {
            f530(v0.clone(), f176(f531(), v0.clone()), f176(f532(), v0))
        }
        Adt148::C1 => {
            Err::<Adt16, Adt127>(Adt127::C2)
        }
        Adt148::C2 => {
            Err::<Adt16, Adt127>(Adt127::C3)
        }
    }
}

pub fn f463() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f464(v0: Str) -> R<List<u64>> {
    f87({
        let a1783 = v0;
        utf8_encode(a1783)
    })
}

pub fn f465(v0: u64, v1: u64) -> u64 {
    let a1784 = v0;
    let a1785 = v1;
    let a1786 = 0u64;
    nat_rem(a1784, a1785, a1786)
}

pub fn f466(v0: u64, v1: u64) -> u64 {
    let a1787 = v0;
    let a1788 = v1;
    let a1789 = 0u64;
    nat_quot(a1787, a1788, a1789)
}

pub fn f467(v0: u64, v1: Adt6) -> bool {
    let a1790 = v0;
    let a1791 = 0u64;
    if nat_eq(a1790, a1791) {
        f533(v1)
    } else {
        let a1792 = v0;
        let a1793 = 1u64;
        if nat_eq(a1792, a1793) {
            f534(v1)
        } else {
            let a1794 = v0;
            let a1795 = 2u64;
            if nat_eq(a1794, a1795) {
                f535(v1)
            } else {
                let a1796 = v0;
                let a1797 = 3u64;
                if nat_eq(a1796, a1797) {
                    f536(v1)
                } else {
                    let a1798 = v0;
                    let a1799 = 4u64;
                    if nat_eq(a1798, a1799) {
                        f537(v1)
                    } else {
                        let a1800 = v0;
                        let a1801 = 5u64;
                        if nat_eq(a1800, a1801) {
                            f538(v1)
                        } else {
                            let a1802 = v0;
                            let a1803 = 6u64;
                            if nat_eq(a1802, a1803) {
                                f539(v1)
                            } else {
                                let a1804 = v0;
                                let a1805 = 7u64;
                                if nat_eq(a1804, a1805) {
                                    f540(v1)
                                } else {
                                    f541(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f468(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt120) -> R<Adt142> {
    let m1806 = v3.clone();
    match m1806.uncons() {
        None => {
            Ok(Adt142::C0(Adt91::C8(v0)))
        }
        Some((_, _)) => {
            if f188(v3.clone(), true) {
                let m1807 = f542(v1);
                match m1807 {
                    None => {
                        Ok(Adt142::C0(Adt91::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f543(v3)?;
                        if f544(v9.clone(), match v5.clone() {
                            Adt120::C0(_, h1808) => {
                                h1808
                            }
                        }) {
                            Ok(Adt142::C0(Adt91::C12(v0, v2)))
                        } else {
                            Ok(Adt142::C1(Adt120::C0(List::cons(Adt12::C0(v0, v2, f545(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt120::C0(h1809, _) => {
                                    h1809
                                }
                            }), List::cons(v9, match v5 {
                                Adt120::C0(_, h1810) => {
                                    h1810
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

pub fn f469(p1815: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1816: List<Adt104>) -> List<Adt104> {
    let m1814 = (p1815, p1816);
    {
        let mut m1814 = m1814;
        loop {
            let step = {
                let (v0, v3) = m1814;
                let m1811 = v0;
                match m1811.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v5, List::cons(Adt104::C0({
                            let (_, h1812) = v4;
                            h1812
                        }, v1, List::cons({
                            let (h1813, _) = v4;
                            h1813
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f470(p1820: List<(Str, Adt6)>, p1821: List<Adt104>) -> List<Adt104> {
    let m1819 = (p1820, p1821);
    {
        let mut m1819 = m1819;
        loop {
            let step = {
                let (v0, v1) = m1819;
                let m1817 = v0;
                match m1817.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0({
                            let (_, h1818) = v2;
                            h1818
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1819 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(p1824: List<Adt6>, p1825: List<Adt104>) -> List<Adt104> {
    let m1823 = (p1824, p1825);
    {
        let mut m1823 = m1823;
        loop {
            let step = {
                let (v0, v1) = m1823;
                let m1822 = v0;
                match m1822.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Adt6) -> Str {
    let m1826 = v0;
    match m1826 {
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

pub fn f473(v0: Adt7) -> R<bool> {
    if f380(match v0.clone() {
        Adt7::C0(h1827, _) => {
            h1827
        }
    }) {
        Ok(true)
    } else {
        f546(40u64, match v0.clone() {
            Adt7::C0(h1828, _) => {
                h1828
            }
        }, match v0 {
            Adt7::C0(_, h1829) => {
                h1829
            }
        })
    }
}

pub fn f474(v0: Adt6) -> Str {
    let m1830 = f231(v0);
    match m1830 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f475(p1837: u64, p1838: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m1836 = (p1837, p1838);
    {
        let mut m1836 = m1836;
        loop {
            let step = {
                let (v0, v1) = m1836;
                let m1831 = v0;
                if m1831 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m1831 - 1;
                    let m1832 = v1;
                    match m1832.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m1835 = f547({
                                let (h1833, _) = v3;
                                h1833
                            }, {
                                let (_, h1834) = v3;
                                h1834
                            }, v4)?;
                            match m1835 {
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
                Ok(next) => { m1836 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476() -> u64 {
    67108864u64
}

pub fn f477(v0: Adt147) -> bool {
    let m1839 = v0;
    match m1839 {
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

pub fn f478(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m1840 = v2;
    match m1840 {
        Adt147::C0 => {
            let m1842 = v3;
            match m1842 {
                Adt147::C0 => {
                    Ok(f548(Adt147::C0, f549(f550(match v0.clone() {
                        Adt7::C0(h1843, _) => {
                            h1843
                        }
                    })?, match v0 {
                        Adt7::C0(_, h1844) => {
                            h1844
                        }
                    }, f550(match v1.clone() {
                        Adt7::C0(h1845, _) => {
                            h1845
                        }
                    })?, match v1 {
                        Adt7::C0(_, h1846) => {
                            h1846
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
            let m1841 = v3;
            match m1841 {
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
            let m1847 = v3;
            match m1847 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f549(match v0.clone() {
                        Adt7::C0(h1848, _) => {
                            h1848
                        }
                    }, match v0 {
                        Adt7::C0(_, h1849) => {
                            h1849
                        }
                    }, match v1.clone() {
                        Adt7::C0(h1850, _) => {
                            h1850
                        }
                    }, match v1 {
                        Adt7::C0(_, h1851) => {
                            h1851
                        }
                    })
                }
            }
        }
    }
}

pub fn f479(v0: i64) -> Adt147 {
    f481(v0, 0i64)
}

pub fn f480(v0: Adt6) -> Option<i64> {
    let m1852 = v0;
    match m1852 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f380(match v2.clone() {
                Adt7::C0(_, h1853) => {
                    h1853
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1854, _) => {
                        h1854
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

pub fn f481(v0: i64, v1: i64) -> Adt147 {
    let m1857 = {
        let a1855 = v0;
        let a1856 = v1;
        compare(a1855, a1856)
    };
    if match m1857 {
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
        let m1860 = {
            let a1858 = v1;
            let a1859 = v0;
            compare(a1858, a1859)
        };
        if match m1860 {
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

pub fn f482(p1869: List<u64>, p1870: u64) -> R<u64> {
    let m1868 = (p1869, p1870);
    {
        let mut m1868 = m1868;
        loop {
            let step = {
                let (v0, v1) = m1868;
                let m1861 = v0;
                match m1861.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a1862 = v2;
                        let a1863 = 128u64;
                        if if nat_lt(a1862, a1863) {
                            true
                        } else {
                            let a1864 = 191u64;
                            let a1865 = v2;
                            nat_lt(a1864, a1865)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a1866 = v1;
                                let a1867 = 1u64;
                                nat_add(a1866, a1867)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
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

pub fn f483(v0: Str, p1888: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1887 = p1888;
    {
        let mut m1887 = m1887;
        loop {
            let step = {
                let v1 = m1887;
                let m1871 = v1;
                match m1871.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a1875 = v0.clone();
                        let a1876 = {
                            let a1873 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h1872, _) = v2.clone();
                                h1872
                            }, List::<Str>::nil()));
                            let a1874 = Str::lit("");
                            join(a1873, a1874)
                        };
                        if equal(a1875, a1876) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h1877) = v2;
                                h1877
                            }))
                        } else {
                            let m1879 = f195({
                                let (_, h1878) = v2.clone();
                                h1878
                            });
                            match m1879 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m1880 = f176(Str::lit("$anchor"), v4);
                                    match m1880 {
                                        Adt130::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt130::C1(v5) => {
                                            let m1881 = f231(v5);
                                            match m1881 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a1884 = v0.clone();
                                                    let a1885 = {
                                                        let a1882 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a1883 = Str::lit("");
                                                        join(a1882, a1883)
                                                    };
                                                    if equal(a1884, a1885) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h1886) = v2;
                                                            h1886
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
                Ok(next) => { m1887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: u64) -> R<Option<u64>> {
    let a1889 = 64u64;
    let a1890 = v0;
    if if nat_lt(a1889, a1890) {
        let a1891 = v0;
        let a1892 = 91u64;
        nat_lt(a1891, a1892)
    } else {
        false
    } {
        Ok(Some({
            let a1893 = v0;
            let a1894 = 65u64;
            nat_sub(a1893, a1894)
        }))
    } else {
        let a1895 = 96u64;
        let a1896 = v0;
        if if nat_lt(a1895, a1896) {
            let a1897 = v0;
            let a1898 = 123u64;
            nat_lt(a1897, a1898)
        } else {
            false
        } {
            Ok(Some({
                let a1899 = v0;
                let a1900 = 71u64;
                nat_sub(a1899, a1900)
            }))
        } else {
            let a1901 = 47u64;
            let a1902 = v0;
            if if nat_lt(a1901, a1902) {
                let a1903 = v0;
                let a1904 = 58u64;
                nat_lt(a1903, a1904)
            } else {
                false
            } {
                Ok(Some({
                    let a1905 = v0;
                    let a1906 = 4u64;
                    nat_add(a1905, a1906)?
                }))
            } else {
                let a1907 = v0;
                let a1908 = 43u64;
                if nat_eq(a1907, a1908) {
                    Ok(Some(62u64))
                } else {
                    let a1909 = v0;
                    let a1910 = 47u64;
                    if nat_eq(a1909, a1910) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f485(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1911 = v3;
        let a1912 = 61u64;
        if nat_eq(a1911, a1912) {
            let a1913 = f551(v1, 16u64);
            let a1914 = 0u64;
            nat_eq(a1913, a1914)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f552(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f486(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1915 = f551(v2, 4u64);
        let a1916 = 0u64;
        nat_eq(a1915, a1916)
    } else {
        false
    } {
        Ok(Some(f553(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f487(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1919 = {
            let a1917 = v0;
            let a1918 = 4u64;
            nat_mul(a1917, a1918)?
        };
        let a1920 = f554(v1, 16u64);
        nat_add(a1919, a1920)?
    }, List::cons({
        let a1923 = {
            let a1921 = f551(v1, 16u64);
            let a1922 = 16u64;
            nat_mul(a1921, a1922)?
        };
        let a1924 = f554(v2, 4u64);
        nat_add(a1923, a1924)?
    }, List::cons({
        let a1927 = {
            let a1925 = f551(v2, 4u64);
            let a1926 = 64u64;
            nat_mul(a1925, a1926)?
        };
        let a1928 = v3;
        nat_add(a1927, a1928)?
    }, List::<u64>::nil()))))
}

pub fn f488(p1937: Bytes, p1938: List<Bytes>) -> List<Bytes> {
    let m1936 = (p1937, p1938);
    {
        let mut m1936 = m1936;
        loop {
            let step = {
                let (v0, v1) = m1936;
                let m1929 = v1;
                match m1929.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1932 = {
                            let a1930 = v2.clone();
                            length_bytes(a1930)
                        };
                        let a1933 = {
                            let a1931 = v0.clone();
                            length_bytes(a1931)
                        };
                        if nat_eq(a1932, a1933) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1934 = v2;
                                let a1935 = v0;
                                append_bytes(a1934, a1935)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
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

pub fn f489(v0: u64) -> Bytes {
    let m1942 = {
        let a1939 = f555();
        let a1940 = v0;
        let a1941 = 1u64;
        slice_bytes(a1939, a1940, a1941)
    };
    match m1942 {
        None => {
            f332()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f490(p1954: u64, p1955: i64, p1956: List<u64>) -> R<List<u64>> {
    let m1953 = (p1954, (p1955, p1956));
    {
        let mut m1953 = m1953;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1953;
                let m1943 = v0;
                if m1943 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1943 - 1;
                    let v4: i64 = {
                        let a1944 = v1;
                        let a1945 = 10i64;
                        let a1946 = 0i64;
                        int_quot(a1944, a1945, a1946)?
                    };
                    let v5: i64 = {
                        let a1947 = v1;
                        let a1948 = 10i64;
                        let a1949 = 0i64;
                        int_rem(a1947, a1948, a1949)
                    };
                    let m1952 = {
                        let a1950 = v4;
                        let a1951 = 1i64;
                        compare(a1950, a1951)
                    };
                    if match m1952 {
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
                Ok(next) => { m1953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f491(v0: u64) -> R<u64> {
    let a1957 = v0;
    let a1958 = 10u64;
    if nat_lt(a1957, a1958) {
        let a1959 = 48u64;
        let a1960 = v0;
        nat_add(a1959, a1960)
    } else {
        let a1961 = 87u64;
        let a1962 = v0;
        nat_add(a1961, a1962)
    }
}

pub fn f492(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1963 = v1;
    match m1963 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f557(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f493(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1964 = v0;
    match m1964 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1967 = f428(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1965, _) => {
                        h1965
                    }
                });
                let a1968 = f428(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1966) => {
                        h1966
                    }
                });
                append_list(a1967, a1968)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f494(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f558(v0, 0i64)?, 0i64)))
}

pub fn f495(v0: Adt11) -> Str {
    let m1969 = v0;
    match m1969 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f496(p1972: List<Adt35>, p1973: List<Adt6>) -> List<Adt6> {
    let m1971 = (p1972, p1973);
    {
        let mut m1971 = m1971;
        loop {
            let step = {
                let (v0, v1) = m1971;
                let m1970 = v0;
                match m1970.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f559(v2), v1)))
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

pub fn f497(p1977: List<Adt38>, p1978: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1976 = (p1977, p1978);
    {
        let mut m1976 = m1976;
        loop {
            let step = {
                let (v0, v1) = m1976;
                let m1974 = v0;
                match m1974.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1975 = f560(v2)?;
                        match m1975 {
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
                Ok(next) => { m1976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(p1981: List<Adt44>, p1982: List<Adt6>) -> R<List<Adt6>> {
    let m1980 = (p1981, p1982);
    {
        let mut m1980 = m1980;
        loop {
            let step = {
                let (v0, v1) = m1980;
                let m1979 = v0;
                match m1979.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f561(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1980 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(p1985: List<Adt46>, p1986: List<Adt6>) -> List<Adt6> {
    let m1984 = (p1985, p1986);
    {
        let mut m1984 = m1984;
        loop {
            let step = {
                let (v0, v1) = m1984;
                let m1983 = v0;
                match m1983.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f562(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(p1989: List<Adt54>, p1990: List<Adt6>) -> List<Adt6> {
    let m1988 = (p1989, p1990);
    {
        let mut m1988 = m1988;
        loop {
            let step = {
                let (v0, v1) = m1988;
                let m1987 = v0;
                match m1987.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f563(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(p1993: List<Adt56>, p1994: List<Adt6>) -> List<Adt6> {
    let m1992 = (p1993, p1994);
    {
        let mut m1992 = m1992;
        loop {
            let step = {
                let (v0, v1) = m1992;
                let m1991 = v0;
                match m1991.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f564(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(p1998: List<Adt59>, p1999: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1997 = (p1998, p1999);
    {
        let mut m1997 = m1997;
        loop {
            let step = {
                let (v0, v1) = m1997;
                let m1995 = v0;
                match m1995.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1996 = f565(v2)?;
                        match m1996 {
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
                Ok(next) => { m1997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2000 = v0;
    match m2000 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f566(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f504() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f505() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f506() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f507() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f508() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f509(p2005: List<List<u64>>, p2006: List<u64>) -> List<u64> {
    let m2004 = (p2005, p2006);
    {
        let mut m2004 = m2004;
        loop {
            let step = {
                let (v0, v1) = m2004;
                let m2001 = v0;
                match m2001.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2002 = f567(v2);
                            let a2003 = v1;
                            append_list(a2002, a2003)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2004 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f510(v0: Str) -> R<Str> {
    let m2009 = {
        let a2008 = f331(f332(), f163(f568(f87({
            let a2007 = v0;
            utf8_encode(a2007)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2008)
    };
    match m2009 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f511(v0: List<u64>) -> bool {
    let m2010 = v0;
    match m2010.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2011 = v1;
            let a2012 = 48u64;
            if nat_eq(a2011, a2012) {
                f569(v2)
            } else {
                false
            }
        }
    }
}

pub fn f512(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt123, Adt126>> {
    if f570(f571(v3.clone())) {
        Ok(Ok::<Adt123, Adt126>(f572(v0.clone(), Adt32::C0(f573(v0)))))
    } else {
        Ok(f574(v0.clone(), v3.clone(), f575(match v0 {
            Adt0::C0(_, _, _, _, _, h2013) => {
                h2013
            }
        }, v2, v1, v3)?))
    }
}

pub fn f513(v0: Adt16, _: Adt5) -> Result<Adt14, Adt127> {
    Ok::<Adt14, Adt127>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f514(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2014 = f576(Str::lit("cursor"), v1);
    match m2014 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f515(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2015 = f577(Str::lit("name"), v1.clone());
    match m2015 {
        Ok(v2) => {
            let m2016 = f578(Str::lit("arguments"), v1.clone());
            match m2016 {
                Ok(v3) => {
                    let m2017 = f578(Str::lit("inputResponses"), v1.clone());
                    match m2017 {
                        Ok(v4) => {
                            let m2018 = f576(Str::lit("requestState"), v1);
                            match m2018 {
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

pub fn f516(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2019 = f576(Str::lit("cursor"), v1);
    match m2019 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f517(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2020 = f577(Str::lit("uri"), v1.clone());
    match m2020 {
        Ok(v2) => {
            let m2021 = f578(Str::lit("inputResponses"), v1.clone());
            match m2021 {
                Ok(v3) => {
                    let m2022 = f576(Str::lit("requestState"), v1);
                    match m2022 {
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

pub fn f518(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2023 = f576(Str::lit("cursor"), v1);
    match m2023 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f519(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2024 = f576(Str::lit("cursor"), v1);
    match m2024 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f520(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2025 = f577(Str::lit("name"), v1.clone());
    match m2025 {
        Ok(v2) => {
            let m2026 = f579(Str::lit("arguments"), v1.clone());
            match m2026 {
                Ok(v3) => {
                    let m2027 = f578(Str::lit("inputResponses"), v1.clone());
                    match m2027 {
                        Ok(v4) => {
                            let m2028 = f576(Str::lit("requestState"), v1);
                            match m2028 {
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

pub fn f521(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2029 = f580(Str::lit("ref"), v1.clone());
    match m2029 {
        Ok(v2) => {
            let m2030 = f581(v2);
            match m2030 {
                Ok(v3) => {
                    let m2031 = f580(Str::lit("argument"), v1.clone());
                    match m2031 {
                        Ok(v4) => {
                            let m2032 = f582(v4);
                            match m2032 {
                                Ok(v5) => {
                                    let m2033 = f583(v1);
                                    match m2033 {
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

pub fn f522(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2034 = f577(Str::lit("taskId"), v1);
    match m2034 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f523(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2035 = f577(Str::lit("taskId"), v1.clone());
    match m2035 {
        Ok(v2) => {
            let m2036 = f580(Str::lit("inputResponses"), v1);
            match m2036 {
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

pub fn f524(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2037 = f577(Str::lit("taskId"), v1);
    match m2037 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f525(v0: Adt103, v1: Adt16, v2: Option<Adt126>) -> Adt138 {
    let m2038 = v2;
    match m2038 {
        None => {
            Adt138::C0(Adt139::C0(match v0.clone() {
                Adt103::C0(h2039, _, _) => {
                    h2039
                }
            }, match v0.clone() {
                Adt103::C0(_, h2040, _) => {
                    h2040
                }
            }, v1, f461(match v0 {
                Adt103::C0(_, _, h2041) => {
                    h2041
                }
            })))
        }
        Some(v3) => {
            Adt138::C1(v3)
        }
    }
}

pub fn f526(v0: Str, v1: List<Str>) -> Option<Adt126> {
    f584(f585(v0.clone(), v1.clone()), v0, v1)
}

pub fn f527(v0: Adt107, v1: Adt140, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt108>> {
    let m2043 = match v0.clone() {
        Adt107::C0(h2042, _, _, _, _) => {
            h2042
        }
    };
    match m2043 {
        None => {
            Ok(Some(Adt108::C0))
        }
        Some(v5) => {
            let a2044 = v5;
            let a2045 = v3;
            if equal(a2044, a2045) {
                let m2047 = match v0.clone() {
                    Adt107::C0(_, h2046, _, _, _) => {
                        h2046
                    }
                };
                match m2047 {
                    None => {
                        Ok(Some(Adt108::C1))
                    }
                    Some(v6) => {
                        let a2048 = v6;
                        let a2049 = v2;
                        if equal(a2048, a2049) {
                            f586(f587(match v0.clone() {
                                Adt107::C0(_, _, h2050, _, _) => {
                                    h2050
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt107::C0(_, _, _, _, h2051) => {
                                    h2051
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

pub fn f528() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f529(v0: Adt5) -> Adt148 {
    f588(f176(f531(), v0.clone()), f176(f532(), v0))
}

pub fn f530(v0: Adt5, v1: Adt130, v2: Adt130) -> Result<Adt16, Adt127> {
    let m2052 = v1;
    match m2052 {
        Adt130::C0 => {
            Err::<Adt16, Adt127>(Adt127::C2)
        }
        Adt130::C1(v3) => {
            let m2053 = v2;
            match m2053 {
                Adt130::C0 => {
                    Err::<Adt16, Adt127>(Adt127::C3)
                }
                Adt130::C1(v4) => {
                    f589(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f531() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f532() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f533(v0: Adt6) -> bool {
    let m2054 = f231(v0);
    match m2054 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f534(v0: Adt6) -> bool {
    let m2055 = f398(v0);
    match m2055 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f535(v0: Adt6) -> bool {
    let m2056 = f480(v0);
    match m2056 {
        None => {
            false
        }
        Some(v1) => {
            let a2060 = {
                let m2059 = {
                    let a2057 = v1;
                    let a2058 = 0i64;
                    compare(a2057, a2058)
                };
                match m2059 {
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
            bool_not(a2060)
        }
    }
}

pub fn f536(v0: Adt6) -> bool {
    let m2061 = f259(v0);
    match m2061 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f537(v0: Adt6) -> bool {
    let m2062 = f195(v0);
    match m2062 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f538(v0: Adt6) -> bool {
    let m2063 = f259(v0);
    match m2063 {
        None => {
            false
        }
        Some(v1) => {
            f590(v1)
        }
    }
}

pub fn f539(v0: Adt6) -> bool {
    let m2064 = f194(v0);
    match m2064 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f540(v0: Adt6) -> bool {
    let m2065 = f231(v0.clone());
    match m2065 {
        None => {
            let m2066 = f259(v0);
            match m2066 {
                None => {
                    false
                }
                Some(v2) => {
                    f591(v2)
                }
            }
        }
        Some(v1) => {
            f393(v1)
        }
    }
}

pub fn f541(v0: Adt6) -> bool {
    let m2067 = f259(v0);
    match m2067 {
        None => {
            false
        }
        Some(v1) => {
            let m2068 = v1;
            match m2068.uncons() {
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

pub fn f542(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2069 = f234(Str::lit("type"), v0);
    match m2069 {
        Adt130::C0 => {
            None::<Adt13>
        }
        Adt130::C1(v1) => {
            let m2070 = f231(v1);
            match m2070 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f592(v2)
                }
            }
        }
    }
}

pub fn f543(v0: List<u64>) -> R<List<u64>> {
    Ok(f163(f593(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f544(v0: List<u64>, p2073: List<List<u64>>) -> bool {
    let m2072 = p2073;
    {
        let mut m2072 = m2072;
        loop {
            let step = {
                let v1 = m2072;
                let m2071 = v1;
                match m2071.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f594(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f545(p2076: List<Str>, p2077: List<Str>) -> List<Str> {
    let m2075 = (p2076, p2077);
    {
        let mut m2075 = m2075;
        loop {
            let step = {
                let (v0, v1) = m2075;
                let m2074 = v0;
                match m2074.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(p2089: u64, p2090: i64, p2091: i64) -> R<bool> {
    let m2088 = (p2089, (p2090, p2091));
    {
        let mut m2088 = m2088;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2088;
                let m2078 = v0;
                if m2078 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2078 - 1;
                    let m2081 = {
                        let a2079 = v2;
                        let a2080 = 0i64;
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
                        if f380({
                            let a2082 = v1;
                            let a2083 = 10i64;
                            let a2084 = 0i64;
                            int_rem(a2082, a2083, a2084)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2085 = v1;
                                let a2086 = 10i64;
                                let a2087 = 0i64;
                                int_quot(a2085, a2086, a2087)?
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
                Ok(next) => { m2088 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2092 = f595(v0.clone());
    let a2093 = f595(v1.clone());
    if nat_eq(a2092, a2093) {
        let m2094 = v0;
        match m2094 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2095 = f194(v1);
                match m2095 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2096 = v3;
                        let a2097 = v4;
                        if equal(a2096, a2097) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2098 = f398(v1);
                match m2098 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f596(f400(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2099 = f231(v1);
                match m2099 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2100 = v7;
                        let a2101 = v8;
                        if equal(a2100, a2101) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2102 = f259(v1);
                match m2102 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f597(f598(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2103 = f195(v1);
                match m2103 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2105 = f404(v11.clone(), 0u64)?;
                        let a2106 = f404(match v12.clone() {
                            Adt5::C0(h2104) => {
                                h2104
                            }
                        }, 0u64)?;
                        if nat_eq(a2105, a2106) {
                            Ok(f597(f599(v11, match v12 {
                                Adt5::C0(h2107) => {
                                    h2107
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

pub fn f548(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2108 = v0;
    match m2108 {
        Adt147::C0 => {
            f600(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f549(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2109 = f481(f601(v0, v1)?, f601(v2, v3)?);
    match m2109 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f602(v0, f603(v0)?, v2, f603(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f550(v0: i64) -> R<i64> {
    let m2112 = {
        let a2110 = v0;
        let a2111 = 0i64;
        compare(a2110, a2111)
    };
    if match m2112 {
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
        let a2113 = v0;
        int_neg(a2113)
    } else {
        Ok(v0)
    }
}

pub fn f551(v0: u64, v1: u64) -> u64 {
    let a2114 = v0;
    let a2115 = v1;
    let a2116 = 0u64;
    nat_rem(a2114, a2115, a2116)
}

pub fn f552(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2119 = {
            let a2117 = v0;
            let a2118 = 4u64;
            nat_mul(a2117, a2118)?
        };
        let a2120 = f554(v1, 16u64);
        nat_add(a2119, a2120)?
    }, List::<u64>::nil()))
}

pub fn f553(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2123 = {
            let a2121 = v0;
            let a2122 = 4u64;
            nat_mul(a2121, a2122)?
        };
        let a2124 = f554(v1, 16u64);
        nat_add(a2123, a2124)?
    }, List::cons({
        let a2127 = {
            let a2125 = f551(v1, 16u64);
            let a2126 = 16u64;
            nat_mul(a2125, a2126)?
        };
        let a2128 = f554(v2, 4u64);
        nat_add(a2127, a2128)?
    }, List::<u64>::nil())))
}

pub fn f554(v0: u64, v1: u64) -> u64 {
    let a2129 = v0;
    let a2130 = v1;
    let a2131 = 0u64;
    nat_quot(a2129, a2130, a2131)
}

pub fn f555() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f556(v0: i64) -> u64 {
    let m2134 = {
        let a2132 = v0;
        let a2133 = 1i64;
        compare(a2132, a2133)
    };
    if match m2134 {
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
        let m2137 = {
            let a2135 = v0;
            let a2136 = 2i64;
            compare(a2135, a2136)
        };
        if match m2137 {
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
            let m2140 = {
                let a2138 = v0;
                let a2139 = 3i64;
                compare(a2138, a2139)
            };
            if match m2140 {
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
                let m2143 = {
                    let a2141 = v0;
                    let a2142 = 4i64;
                    compare(a2141, a2142)
                };
                if match m2143 {
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
                    let m2146 = {
                        let a2144 = v0;
                        let a2145 = 5i64;
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
                        52u64
                    } else {
                        let m2149 = {
                            let a2147 = v0;
                            let a2148 = 6i64;
                            compare(a2147, a2148)
                        };
                        if match m2149 {
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
                            let m2152 = {
                                let a2150 = v0;
                                let a2151 = 7i64;
                                compare(a2150, a2151)
                            };
                            if match m2152 {
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
                                let m2155 = {
                                    let a2153 = v0;
                                    let a2154 = 8i64;
                                    compare(a2153, a2154)
                                };
                                if match m2155 {
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
                                    let m2158 = {
                                        let a2156 = v0;
                                        let a2157 = 9i64;
                                        compare(a2156, a2157)
                                    };
                                    if match m2158 {
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
    Adt6::C5(f428(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2159) => {
            h2159
        }
    }))
}

pub fn f558(p2164: u64, p2165: i64) -> R<i64> {
    let m2163 = (p2164, p2165);
    {
        let mut m2163 = m2163;
        loop {
            let step = {
                let (v0, v1) = m2163;
                let m2160 = v0;
                if m2160 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2160 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2161 = v1;
                        let a2162 = -1i64;
                        int_sub(a2161, a2162)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2166, _, _, _, _, _, _, _) => {
            h2166
        }
    })), {
        let a2184 = f423(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2167, _, _, _, _, _, _) => {
                h2167
            }
        });
        let a2185 = {
            let a2182 = f503(match v0.clone() {
                Adt35::C0(_, _, h2168, _, _, _, _, _) => {
                    h2168
                }
            });
            let a2183 = {
                let a2180 = f423(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2169, _, _, _, _) => {
                        h2169
                    }
                });
                let a2181 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2172 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2171, _, _, _) => {
                            h2171
                        }
                    };
                    match m2172 {
                        Adt5::C0(h2170) => {
                            h2170
                        }
                    }
                })), {
                    let a2178 = f449(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2173, _, _) => {
                            h2173
                        }
                    });
                    let a2179 = {
                        let a2176 = f604(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2174, _) => {
                                h2174
                            }
                        });
                        let a2177 = f605(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2175) => {
                                h2175
                            }
                        });
                        append_list(a2176, a2177)
                    };
                    append_list(a2178, a2179)
                });
                append_list(a2180, a2181)
            };
            append_list(a2182, a2183)
        };
        append_list(a2184, a2185)
    }))
}

pub fn f560(v0: Adt38) -> R<Option<Adt6>> {
    let m2186 = v0;
    match m2186 {
        Adt38::C0(v1) => {
            Ok(Some(f606(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f607(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2187, _, _, _) => {
                    h2187
                }
            }, match v2.clone() {
                Adt42::C0(_, h2188, _, _) => {
                    h2188
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2189, _) => {
                    h2189
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2190) => {
                    h2190
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f607(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2191, _, _, _) => {
                    h2191
                }
            }, match v3.clone() {
                Adt43::C0(_, h2192, _, _) => {
                    h2192
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2193, _) => {
                    h2193
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2194) => {
                    h2194
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f608(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f609(v5)))
        }
    }
}

pub fn f561(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2218 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2195, _, _, _, _, _, _, _, _) => {
                h2195
            }
        })), List::<(Str, Adt6)>::nil());
        let a2219 = {
            let a2216 = f423(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2196, _, _, _, _, _, _, _) => {
                    h2196
                }
            });
            let a2217 = {
                let a2214 = f503(match v0.clone() {
                    Adt44::C0(_, _, h2197, _, _, _, _, _, _) => {
                        h2197
                    }
                });
                let a2215 = {
                    let a2212 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2198, _, _, _, _, _) => {
                            h2198
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2213 = {
                        let a2210 = f423(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2199, _, _, _, _) => {
                                h2199
                            }
                        });
                        let a2211 = {
                            let a2208 = f423(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2200, _, _, _) => {
                                    h2200
                                }
                            });
                            let a2209 = {
                                let a2206 = f610(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2201, _, _) => {
                                        h2201
                                    }
                                });
                                let a2207 = {
                                    let a2204 = f611(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2202, _) => {
                                            h2202
                                        }
                                    })?;
                                    let a2205 = f605(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2203) => {
                                            h2203
                                        }
                                    });
                                    append_list(a2204, a2205)
                                };
                                append_list(a2206, a2207)
                            };
                            append_list(a2208, a2209)
                        };
                        append_list(a2210, a2211)
                    };
                    append_list(a2212, a2213)
                };
                append_list(a2214, a2215)
            };
            append_list(a2216, a2217)
        };
        append_list(a2218, a2219)
    }))
}

pub fn f562(v0: Adt46) -> Adt6 {
    let m2220 = v0;
    match m2220 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2221, _, _, _) => {
                    h2221
                }
            })), {
                let a2225 = f423(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2222, _, _) => {
                        h2222
                    }
                });
                let a2226 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2223, _) => {
                        h2223
                    }
                })), f605(match v1 {
                    Adt47::C0(_, _, _, h2224) => {
                        h2224
                    }
                }));
                append_list(a2225, a2226)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2227, _, _, _) => {
                    h2227
                }
            })), {
                let a2231 = f423(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2228, _, _) => {
                        h2228
                    }
                });
                let a2232 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2229, _) => {
                        h2229
                    }
                })), f605(match v2 {
                    Adt48::C0(_, _, _, h2230) => {
                        h2230
                    }
                }));
                append_list(a2231, a2232)
            }))
        }
    }
}

pub fn f563(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a2253 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h2233, _, _, _, _, _, _, _) => {
                h2233
            }
        })), List::<(Str, Adt6)>::nil());
        let a2254 = {
            let a2251 = f423(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h2234, _, _, _, _, _, _) => {
                    h2234
                }
            });
            let a2252 = {
                let a2249 = f503(match v0.clone() {
                    Adt54::C0(_, _, h2235, _, _, _, _, _) => {
                        h2235
                    }
                });
                let a2250 = {
                    let a2247 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h2236, _, _, _, _) => {
                            h2236
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2248 = {
                        let a2245 = f423(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h2237, _, _, _) => {
                                h2237
                            }
                        });
                        let a2246 = {
                            let a2243 = f423(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h2238, _, _) => {
                                    h2238
                                }
                            });
                            let a2244 = {
                                let a2241 = f610(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h2239, _) => {
                                        h2239
                                    }
                                });
                                let a2242 = f605(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h2240) => {
                                        h2240
                                    }
                                });
                                append_list(a2241, a2242)
                            };
                            append_list(a2243, a2244)
                        };
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

pub fn f564(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2270 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2255, _, _, _, _, _) => {
                h2255
            }
        })), List::<(Str, Adt6)>::nil());
        let a2271 = {
            let a2268 = f423(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2256, _, _, _, _) => {
                    h2256
                }
            });
            let a2269 = {
                let a2266 = f503(match v0.clone() {
                    Adt56::C0(_, _, h2257, _, _, _) => {
                        h2257
                    }
                });
                let a2267 = {
                    let a2264 = f423(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h2258, _, _) => {
                            h2258
                        }
                    });
                    let a2265 = {
                        let a2262 = {
                            let m2260 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h2259, _) => {
                                    h2259
                                }
                            };
                            match m2260 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f612(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2263 = f605(match v0 {
                            Adt56::C0(_, _, _, _, _, h2261) => {
                                h2261
                            }
                        });
                        append_list(a2262, a2263)
                    };
                    append_list(a2264, a2265)
                };
                append_list(a2266, a2267)
            };
            append_list(a2268, a2269)
        };
        append_list(a2270, a2271)
    })
}

pub fn f565(v0: Adt59) -> R<Option<Adt6>> {
    let m2273 = f560(match v0.clone() {
        Adt59::C0(_, h2272) => {
            h2272
        }
    })?;
    match m2273 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f613(match v0 {
                Adt59::C0(h2274, _) => {
                    h2274
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f566(v0: List<Adt2>) -> List<Adt6> {
    f294(f614(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f567(v0: List<u64>) -> List<u64> {
    let a2277 = f615();
    let a2278 = {
        let a2275 = v0;
        let a2276 = f616();
        append_list(a2275, a2276)
    };
    append_list(a2277, a2278)
}

pub fn f568(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f617(v0, 0u64, v1)
}

pub fn f569(p2285: List<u64>) -> bool {
    let m2284 = p2285;
    {
        let mut m2284 = m2284;
        loop {
            let step = {
                let v0 = m2284;
                let m2279 = v0;
                match m2279.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2280 = v1;
                        let a2281 = 48u64;
                        if if nat_eq(a2280, a2281) {
                            true
                        } else {
                            let a2282 = v1;
                            let a2283 = 46u64;
                            nat_eq(a2282, a2283)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: Adt134) -> bool {
    let m2286 = v0;
    match m2286 {
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

pub fn f571(v0: Adt14) -> Adt134 {
    let m2287 = v0;
    match m2287 {
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

pub fn f572(v0: Adt0, v1: Adt32) -> Adt123 {
    Adt123::C0(v1, Adt124::C0(Some(match v0 {
        Adt0::C0(h2288, _, _, _, _, _) => {
            h2288
        }
    }), None::<Adt5>), List::<Adt125>::nil())
}

pub fn f573(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2289, _, _) => {
            h2289
        }
    }, f174(), match v0.clone() {
        Adt0::C0(_, _, h2290, _, _, _) => {
            h2290
        }
    }, match v0 {
        Adt0::C0(_, h2291, _, _, _, _) => {
            h2291
        }
    })
}

pub fn f574(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt123, Adt126> {
    let m2292 = v2;
    match m2292 {
        Ok(v4) => {
            if f618(f571(v1.clone()), v4.clone()) {
                f619(v0, v1, v4)
            } else {
                Err::<Adt123, Adt126>(Adt126::C4(f620()))
            }
        }
        Err(v3) => {
            Err::<Adt123, Adt126>(Adt126::C4(v3))
        }
    }
}

pub fn f575(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2293 = v3.clone();
    match m2293 {
        Adt14::C0(_) => {
            let c2295: Fn0 = v2;
            c2295.apply(v3)
        }
        Adt14::C1(_) => {
            let c2296: Fn0 = v2;
            c2296.apply(v3)
        }
        Adt14::C2(_) => {
            if f621(v0, v1) {
                let c2294: Fn0 = v2;
                c2294.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2297: Fn0 = v2;
            c2297.apply(v3)
        }
        Adt14::C4(_) => {
            let c2298: Fn0 = v2;
            c2298.apply(v3)
        }
        Adt14::C5(_) => {
            let c2299: Fn0 = v2;
            c2299.apply(v3)
        }
        Adt14::C6(_) => {
            let c2300: Fn0 = v2;
            c2300.apply(v3)
        }
        Adt14::C7(_) => {
            let c2301: Fn0 = v2;
            c2301.apply(v3)
        }
        Adt14::C8(_) => {
            let c2302: Fn0 = v2;
            c2302.apply(v3)
        }
        Adt14::C9(_) => {
            let c2303: Fn0 = v2;
            c2303.apply(v3)
        }
        Adt14::C10(_) => {
            let c2304: Fn0 = v2;
            c2304.apply(v3)
        }
        Adt14::C11(_) => {
            let c2305: Fn0 = v2;
            c2305.apply(v3)
        }
    }
}

pub fn f576(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt127> {
    let m2306 = f176(v0.clone(), v1);
    match m2306 {
        Adt130::C0 => {
            Ok::<Option<Str>, Adt127>(None::<Str>)
        }
        Adt130::C1(v2) => {
            let m2307 = f231(v2);
            match m2307 {
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

pub fn f577(v0: Str, v1: Adt5) -> Result<Str, Adt127> {
    let m2308 = f176(v0.clone(), v1);
    match m2308 {
        Adt130::C0 => {
            Err::<Str, Adt127>(Adt127::C7(v0))
        }
        Adt130::C1(v2) => {
            let m2309 = f231(v2);
            match m2309 {
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

pub fn f578(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt127> {
    let m2310 = f176(v0.clone(), v1);
    match m2310 {
        Adt130::C0 => {
            Ok::<Option<Adt5>, Adt127>(None::<Adt5>)
        }
        Adt130::C1(v2) => {
            let m2311 = f195(v2);
            match m2311 {
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

pub fn f579(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt127> {
    let m2312 = f176(v0.clone(), v1);
    match m2312 {
        Adt130::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt127>(None::<List<(Str, Str)>>)
        }
        Adt130::C1(v2) => {
            let m2313 = f195(v2);
            match m2313 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt127>(Adt127::C8(v0))
                }
                Some(v3) => {
                    let m2315 = f622(v0, match v3 {
                        Adt5::C0(h2314) => {
                            h2314
                        }
                    });
                    match m2315 {
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

pub fn f580(v0: Str, v1: Adt5) -> Result<Adt5, Adt127> {
    let m2316 = f176(v0.clone(), v1);
    match m2316 {
        Adt130::C0 => {
            Err::<Adt5, Adt127>(Adt127::C7(v0))
        }
        Adt130::C1(v2) => {
            let m2317 = f195(v2);
            match m2317 {
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

pub fn f581(v0: Adt5) -> Result<Adt25, Adt127> {
    let m2318 = f577(Str::lit("type"), v0.clone());
    match m2318 {
        Ok(v1) => {
            let a2319 = v1.clone();
            let a2320 = Str::lit("ref/prompt");
            if equal(a2319, a2320) {
                let m2321 = f577(Str::lit("name"), v0.clone());
                match m2321 {
                    Ok(v2) => {
                        let m2322 = f576(Str::lit("title"), v0);
                        match m2322 {
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
                let a2323 = v1;
                let a2324 = Str::lit("ref/resource");
                if equal(a2323, a2324) {
                    let m2325 = f577(Str::lit("uri"), v0);
                    match m2325 {
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

pub fn f582(v0: Adt5) -> Result<Adt27, Adt127> {
    let m2326 = f577(Str::lit("name"), v0.clone());
    match m2326 {
        Ok(v1) => {
            let m2327 = f577(Str::lit("value"), v0);
            match m2327 {
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

pub fn f583(v0: Adt5) -> Result<Option<Adt28>, Adt127> {
    let m2328 = f176(Str::lit("context"), v0);
    match m2328 {
        Adt130::C0 => {
            Ok::<Option<Adt28>, Adt127>(None::<Adt28>)
        }
        Adt130::C1(v1) => {
            let m2329 = f195(v1);
            match m2329 {
                None => {
                    Err::<Option<Adt28>, Adt127>(Adt127::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2330 = f579(Str::lit("arguments"), v2);
                    match m2330 {
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

pub fn f584(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt126> {
    if v0 {
        None::<Adt126>
    } else {
        Some(Adt126::C1(v1, v2))
    }
}

pub fn f585(v0: Str, p2335: List<Str>) -> bool {
    let m2334 = p2335;
    {
        let mut m2334 = m2334;
        loop {
            let step = {
                let v1 = m2334;
                let m2331 = v1;
                match m2331.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2332 = v2;
                        let a2333 = v0.clone();
                        if equal(a2332, a2333) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(v0: Option<Adt108>, v1: List<Adt110>, v2: Adt5) -> R<Option<Adt108>> {
    let m2336 = v0;
    match m2336 {
        None => {
            f623(v1, f624(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f587(v0: Option<Str>, v1: Adt140, v2: Adt5) -> Option<Adt108> {
    let m2337 = v1.clone();
    match m2337 {
        Adt140::C0 => {
            None::<Adt108>
        }
        Adt140::C1 => {
            f625(v0, f626(v1, v2))
        }
        Adt140::C2 => {
            f625(v0, f626(v1, v2))
        }
        Adt140::C3 => {
            f625(v0, f626(v1, v2))
        }
    }
}

pub fn f588(v0: Adt130, v1: Adt130) -> Adt148 {
    let m2338 = v0;
    match m2338 {
        Adt130::C0 => {
            Adt148::C1
        }
        Adt130::C1(_) => {
            let m2339 = v1;
            match m2339 {
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

pub fn f589(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt127> {
    let m2340 = f231(v1);
    match m2340 {
        None => {
            Err::<Adt16, Adt127>(Adt127::C5)
        }
        Some(v3) => {
            let m2341 = f627(v2);
            match m2341 {
                None => {
                    Err::<Adt16, Adt127>(Adt127::C6)
                }
                Some(v4) => {
                    let m2342 = f578(f628(), v0.clone());
                    match m2342 {
                        Ok(v6) => {
                            let m2343 = f629(v0.clone());
                            match m2343 {
                                Ok(v8) => {
                                    let m2344 = f630(v0.clone());
                                    match m2344 {
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

pub fn f590(p2348: List<Adt6>) -> bool {
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
                        let m2346 = f231(v1);
                        match m2346 {
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
                Ok(next) => { m2347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(p2352: List<Adt6>) -> bool {
    let m2351 = p2352;
    {
        let mut m2351 = m2351;
        loop {
            let step = {
                let v0 = m2351;
                let m2349 = v0;
                match m2349.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2350 = f231(v1);
                        match m2350 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f393(v3) {
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
                Ok(next) => { m2351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f592(v0: Str) -> Option<Adt13> {
    let a2353 = v0.clone();
    let a2354 = Str::lit("string");
    if equal(a2353, a2354) {
        Some(Adt13::C0)
    } else {
        let a2355 = v0.clone();
        let a2356 = Str::lit("integer");
        if equal(a2355, a2356) {
            Some(Adt13::C1)
        } else {
            let a2357 = v0;
            let a2358 = Str::lit("boolean");
            if equal(a2357, a2358) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f593(p2367: List<u64>, p2368: List<u64>) -> R<List<u64>> {
    let m2366 = (p2367, p2368);
    {
        let mut m2366 = m2366;
        loop {
            let step = {
                let (v0, v1) = m2366;
                let m2359 = v0;
                match m2359.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2360 = 65u64;
                        let a2361 = v2;
                        if if nat_le(a2360, a2361) {
                            let a2362 = v2;
                            let a2363 = 90u64;
                            nat_le(a2362, a2363)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2364 = v2;
                                let a2365 = 32u64;
                                nat_add(a2364, a2365)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2366 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(p2375: List<u64>, p2376: List<u64>) -> bool {
    let m2374 = (p2375, p2376);
    {
        let mut m2374 = m2374;
        loop {
            let step = {
                let (v0, v1) = m2374;
                let m2369 = v0;
                match m2369.uncons() {
                    None => {
                        let m2370 = v1;
                        match m2370.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2371 = v1;
                        match m2371.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2372 = v4;
                                let a2373 = v6;
                                if nat_eq(a2372, a2373) {
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
                Ok(next) => { m2374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: Adt6) -> u64 {
    let m2377 = v0;
    match m2377 {
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

pub fn f596(v0: Adt147) -> bool {
    let m2378 = v0;
    match m2378 {
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

pub fn f597(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2379 = v0;
    match m2379 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2380 = v2;
                let a2381 = v1;
                append_list(a2380, a2381)
            })
        }
    }
}

pub fn f598(p2386: List<Adt6>, p2387: List<Adt6>, p2388: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2385 = (p2386, (p2387, p2388));
    {
        let mut m2385 = m2385;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2385;
                let m2382 = v0;
                match m2382.uncons() {
                    None => {
                        let m2383 = v1;
                        match m2383.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2384 = v1;
                        match m2384.uncons() {
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
                Ok(next) => { m2385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f599(p2394: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2395: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2393 = (p2394, p2395);
    {
        let mut m2393 = m2393;
        loop {
            let step = {
                let (v0, v2) = m2393;
                let m2389 = v0;
                match m2389.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2391 = f234({
                            let (h2390, _) = v3;
                            h2390
                        }, v1.clone());
                        match m2391 {
                            Adt130::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt130::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2392) = v3;
                                    h2392
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(v0: Adt147) -> Adt147 {
    let m2396 = v0;
    match m2396 {
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

pub fn f601(v0: i64, v1: i64) -> R<i64> {
    f115(f79(f603(v0)?, 0i64)?, v1)
}

pub fn f602(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2397 = v1;
    let a2398 = v3;
    if nat_lt(a2397, a2398) {
        Ok(f481(f631({
            let a2399 = v3;
            let a2400 = v1;
            nat_sub(a2399, a2400)
        }, v0)?, v2))
    } else {
        let a2401 = v3;
        let a2402 = v1;
        if nat_lt(a2401, a2402) {
            Ok(f481(v0, f631({
                let a2403 = v1;
                let a2404 = v3;
                nat_sub(a2403, a2404)
            }, v2)?))
        } else {
            Ok(f481(v0, v2))
        }
    }
}

pub fn f603(v0: i64) -> R<u64> {
    f632(20u64, v0, 0u64)
}

pub fn f604(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2405 = v0;
    match m2405 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2417 = f423(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2406, _, _, _, _) => {
                        h2406
                    }
                });
                let a2418 = {
                    let a2415 = f428(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2407, _, _, _) => {
                            h2407
                        }
                    });
                    let a2416 = {
                        let a2413 = f428(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2408, _, _) => {
                                h2408
                            }
                        });
                        let a2414 = {
                            let a2411 = f428(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2409, _) => {
                                    h2409
                                }
                            });
                            let a2412 = f428(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2410) => {
                                    h2410
                                }
                            });
                            append_list(a2411, a2412)
                        };
                        append_list(a2413, a2414)
                    };
                    append_list(a2415, a2416)
                };
                append_list(a2417, a2418)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f605(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f449(Str::lit("_meta"), v0)
}

pub fn f606(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2419, _, _) => {
            h2419
        }
    })), {
        let a2422 = f610(match v0.clone() {
            Adt39::C0(_, h2420, _) => {
                h2420
            }
        });
        let a2423 = f605(match v0 {
            Adt39::C0(_, _, h2421) => {
                h2421
            }
        });
        append_list(a2422, a2423)
    })))
}

pub fn f607(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2424 = f610(v3);
        let a2425 = f605(v4);
        append_list(a2424, a2425)
    }))))
}

pub fn f608(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2449 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2426, _, _, _, _, _, _, _, _) => {
                h2426
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2450 = {
            let a2447 = f423(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2427, _, _, _, _, _, _, _) => {
                    h2427
                }
            });
            let a2448 = {
                let a2445 = f503(match v0.clone() {
                    Adt44::C0(_, _, h2428, _, _, _, _, _, _) => {
                        h2428
                    }
                });
                let a2446 = {
                    let a2443 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2429, _, _, _, _, _) => {
                            h2429
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2444 = {
                        let a2441 = f423(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2430, _, _, _, _) => {
                                h2430
                            }
                        });
                        let a2442 = {
                            let a2439 = f423(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2431, _, _, _) => {
                                    h2431
                                }
                            });
                            let a2440 = {
                                let a2437 = f610(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2432, _, _) => {
                                        h2432
                                    }
                                });
                                let a2438 = {
                                    let a2435 = f611(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2433, _) => {
                                            h2433
                                        }
                                    })?;
                                    let a2436 = f605(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2434) => {
                                            h2434
                                        }
                                    });
                                    append_list(a2435, a2436)
                                };
                                append_list(a2437, a2438)
                            };
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
    }))
}

pub fn f609(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f562(match v0.clone() {
        Adt45::C0(h2451, _, _) => {
            h2451
        }
    })), {
        let a2454 = f610(match v0.clone() {
            Adt45::C0(_, h2452, _) => {
                h2452
            }
        });
        let a2455 = f605(match v0 {
            Adt45::C0(_, _, h2453) => {
                h2453
            }
        });
        append_list(a2454, a2455)
    })))
}

pub fn f610(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2456 = v0;
    match m2456 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2462 = f633(match v1.clone() {
                    Adt40::C0(h2457, _, _) => {
                        h2457
                    }
                });
                let a2463 = {
                    let a2460 = f634(match v1.clone() {
                        Adt40::C0(_, h2458, _) => {
                            h2458
                        }
                    });
                    let a2461 = f423(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2459) => {
                            h2459
                        }
                    });
                    append_list(a2460, a2461)
                };
                append_list(a2462, a2463)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f611(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2464 = v0;
    match m2464 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f494(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f612(v0: List<Adt57>) -> List<Adt6> {
    f294(f635(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f613(v0: Adt41) -> Str {
    let m2465 = v0;
    match m2465 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f614(p2468: List<Adt2>, p2469: List<Adt6>) -> List<Adt6> {
    let m2467 = (p2468, p2469);
    {
        let mut m2467 = m2467;
        loop {
            let step = {
                let (v0, v1) = m2467;
                let m2466 = v0;
                match m2466.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f636(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f616() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f617(p2474: List<u64>, p2475: u64, p2476: List<u64>) -> List<u64> {
    let m2473 = (p2474, (p2475, p2476));
    {
        let mut m2473 = m2473;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2473;
                let m2470 = v0;
                match m2470.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f637(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt150 = f638(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt150::C0(h2471, _) => {
                                h2471
                            }
                        }, match v5 {
                            Adt150::C0(_, h2472) => {
                                h2472
                            }
                        })))
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

pub fn f618(v0: Adt134, v1: Adt32) -> bool {
    let m2477 = v1;
    match m2477 {
        Adt32::C0(_) => {
            f639(v0, Adt134::C0)
        }
        Adt32::C1(_) => {
            f639(v0, Adt134::C1)
        }
        Adt32::C2(_) => {
            f639(v0, Adt134::C2)
        }
        Adt32::C3(_) => {
            f639(v0, Adt134::C2)
        }
        Adt32::C4(_) => {
            f639(v0, Adt134::C3)
        }
        Adt32::C5(_) => {
            f639(v0, Adt134::C4)
        }
        Adt32::C6(_) => {
            f639(v0, Adt134::C5)
        }
        Adt32::C7(_) => {
            f639(v0, Adt134::C6)
        }
        Adt32::C8(_) => {
            f639(v0, Adt134::C7)
        }
        Adt32::C9(_) => {
            f639(v0, Adt134::C8)
        }
        Adt32::C10(_) => {
            f639(v0, Adt134::C9)
        }
        Adt32::C11 => {
            if f639(v0.clone(), Adt134::C10) {
                true
            } else {
                f639(v0, Adt134::C11)
            }
        }
        Adt32::C12(_) => {
            f640(v0)
        }
        Adt32::C13(_) => {
            f641(v0)
        }
    }
}

pub fn f619(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt123, Adt126> {
    let m2478 = v2.clone();
    match m2478 {
        Adt32::C0(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt123, Adt126>(f645(v0, f643(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt123, Adt126>(f572(v0, v2))
        }
        Adt32::C12(v3) => {
            f642(v0, f643(v1), v3)
        }
        Adt32::C13(v4) => {
            f644(v0, f643(v1), v4)
        }
    }
}

pub fn f620() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f621(v0: u64, v1: u64) -> bool {
    let a2479 = v1;
    let a2480 = v0;
    nat_lt(a2479, a2480)
}

pub fn f622(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt127> {
    let m2481 = f646(v0, v1, List::<(Str, Str)>::nil());
    match m2481 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt127>(f647(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt127>(v3)
        }
    }
}

pub fn f623(p2485: List<Adt110>, v1: Adt5) -> R<Option<Adt108>> {
    let m2484 = p2485;
    {
        let mut m2484 = m2484;
        loop {
            let step = {
                let v0 = m2484;
                let m2482 = v0;
                match m2482.uncons() {
                    None => {
                        Err::<List<Adt110>, R<Option<Adt108>>>(Ok(None::<Adt108>))
                    }
                    Some((v2, v3)) => {
                        let m2483 = f648(v2, v1.clone())?;
                        match m2483 {
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
                Ok(next) => { m2484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f624(v0: Adt5) -> Adt5 {
    let m2486 = f176(Str::lit("arguments"), v0);
    match m2486 {
        Adt130::C0 => {
            f528()
        }
        Adt130::C1(v1) => {
            let m2487 = f195(v1);
            match m2487 {
                None => {
                    f528()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f625(v0: Option<Str>, v1: Option<Str>) -> Option<Adt108> {
    let m2488 = v0;
    match m2488 {
        None => {
            Some(Adt108::C2)
        }
        Some(v2) => {
            let m2489 = v1;
            match m2489 {
                None => {
                    None::<Adt108>
                }
                Some(v3) => {
                    let a2490 = v2;
                    let a2491 = v3;
                    if equal(a2490, a2491) {
                        None::<Adt108>
                    } else {
                        Some(Adt108::C5)
                    }
                }
            }
        }
    }
}

pub fn f626(v0: Adt140, v1: Adt5) -> Option<Str> {
    let m2492 = v0;
    match m2492 {
        Adt140::C0 => {
            None::<Str>
        }
        Adt140::C1 => {
            let m2493 = f176(Str::lit("name"), v1);
            match m2493 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v2) => {
                    f231(v2)
                }
            }
        }
        Adt140::C2 => {
            let m2494 = f176(Str::lit("uri"), v1);
            match m2494 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v3) => {
                    f231(v3)
                }
            }
        }
        Adt140::C3 => {
            let m2495 = f176(Str::lit("taskId"), v1);
            match m2495 {
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

pub fn f627(v0: Adt6) -> Option<Adt17> {
    let m2496 = f195(v0);
    match m2496 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2497 = f649(Str::lit("experimental"), v1.clone());
            match m2497 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2498 = f649(Str::lit("roots"), v1.clone());
                    match m2498 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2499 = f649(Str::lit("sampling"), v1.clone());
                            match m2499 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2500 = f649(Str::lit("elicitation"), v1.clone());
                                    match m2500 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2501 = f649(Str::lit("extensions"), v1);
                                            match m2501 {
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

pub fn f628() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f629(v0: Adt5) -> Result<Option<Adt18>, Adt127> {
    let m2502 = f576(f650(), v0);
    match m2502 {
        Ok(v2) => {
            let m2503 = v2;
            match m2503 {
                None => {
                    Ok::<Option<Adt18>, Adt127>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2504 = f651(v3);
                    match m2504 {
                        Adt151::C0 => {
                            Err::<Option<Adt18>, Adt127>(Adt127::C8(f650()))
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

pub fn f630(v0: Adt5) -> Result<Option<Adt19>, Adt127> {
    let m2505 = f176(f652(), v0);
    match m2505 {
        Adt130::C0 => {
            Ok::<Option<Adt19>, Adt127>(None::<Adt19>)
        }
        Adt130::C1(v1) => {
            let m2506 = f231(v1.clone());
            match m2506 {
                None => {
                    let m2507 = f480(v1);
                    match m2507 {
                        None => {
                            Err::<Option<Adt19>, Adt127>(Adt127::C8(f652()))
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

pub fn f631(p2512: u64, p2513: i64) -> R<i64> {
    let m2511 = (p2512, p2513);
    {
        let mut m2511 = m2511;
        loop {
            let step = {
                let (v0, v1) = m2511;
                let m2508 = v0;
                if m2508 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2508 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2509 = v1;
                        let a2510 = 10i64;
                        int_mul(a2509, a2510)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f632(p2524: u64, p2525: i64, p2526: u64) -> R<u64> {
    let m2523 = (p2524, (p2525, p2526));
    {
        let mut m2523 = m2523;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2523;
                let m2514 = v0;
                if m2514 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2514 - 1;
                    let v4: u64 = {
                        let a2515 = v2;
                        let a2516 = 1u64;
                        nat_add(a2515, a2516)?
                    };
                    let m2519 = {
                        let a2517 = v1;
                        let a2518 = 10i64;
                        compare(a2517, a2518)
                    };
                    if match m2519 {
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
                            let a2520 = v1;
                            let a2521 = 10i64;
                            let a2522 = 0i64;
                            int_quot(a2520, a2521, a2522)?
                        }, v4)))
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

pub fn f633(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2527 = v0;
    match m2527 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f653(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f634(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2528 = v0;
    match m2528 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f635(p2531: List<Adt57>, p2532: List<Adt6>) -> List<Adt6> {
    let m2530 = (p2531, p2532);
    {
        let mut m2530 = m2530;
        loop {
            let step = {
                let (v0, v1) = m2530;
                let m2529 = v0;
                match m2529.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f654(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f636(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2533, _, _, _) => {
            h2533
        }
    })), {
        let a2539 = f423(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2534, _, _) => {
                h2534
            }
        });
        let a2540 = {
            let a2537 = f655(match v0.clone() {
                Adt2::C0(_, _, h2535, _) => {
                    h2535
                }
            });
            let a2538 = f656(match v0 {
                Adt2::C0(_, _, _, h2536) => {
                    h2536
                }
            });
            append_list(a2537, a2538)
        };
        append_list(a2539, a2540)
    }))
}

pub fn f637(v0: u64, v1: List<u64>) -> List<u64> {
    let a2541 = v0;
    let a2542 = 0u64;
    if nat_eq(a2541, a2542) {
        v1
    } else {
        let a2543 = v0;
        let a2544 = 1u64;
        if nat_eq(a2543, a2544) {
            List::cons(194u64, v1)
        } else {
            let a2545 = v0;
            let a2546 = 2u64;
            if nat_eq(a2545, a2546) {
                List::cons(226u64, v1)
            } else {
                let a2547 = v0;
                let a2548 = 3u64;
                if nat_eq(a2547, a2548) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f638(v0: u64, v1: u64, v2: List<u64>) -> Adt150 {
    let a2549 = v0;
    let a2550 = 0u64;
    if nat_eq(a2549, a2550) {
        f657(v1, v2)
    } else {
        let a2551 = v0;
        let a2552 = 1u64;
        if nat_eq(a2551, a2552) {
            let a2553 = 128u64;
            let a2554 = v1;
            if if nat_le(a2553, a2554) {
                let a2555 = v1;
                let a2556 = 159u64;
                nat_le(a2555, a2556)
            } else {
                false
            } {
                Adt150::C0(0u64, f658(v2))
            } else {
                f657(v1, List::cons(194u64, v2))
            }
        } else {
            let a2557 = v0;
            let a2558 = 2u64;
            if nat_eq(a2557, a2558) {
                let a2559 = v1;
                let a2560 = 128u64;
                if nat_eq(a2559, a2560) {
                    Adt150::C0(3u64, v2)
                } else {
                    let a2561 = v1;
                    let a2562 = 129u64;
                    if nat_eq(a2561, a2562) {
                        Adt150::C0(4u64, v2)
                    } else {
                        f657(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2563 = v0;
                let a2564 = 3u64;
                if nat_eq(a2563, a2564) {
                    let a2565 = 170u64;
                    let a2566 = v1;
                    if if nat_le(a2565, a2566) {
                        let a2567 = v1;
                        let a2568 = 174u64;
                        nat_le(a2567, a2568)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f658(v2))
                    } else {
                        f657(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2569 = 166u64;
                    let a2570 = v1;
                    if if nat_le(a2569, a2570) {
                        let a2571 = v1;
                        let a2572 = 169u64;
                        nat_le(a2571, a2572)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f658(v2))
                    } else {
                        f657(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f639(v0: Adt134, v1: Adt134) -> bool {
    let a2573 = f453(v0);
    let a2574 = f453(v1);
    equal(a2573, a2574)
}

pub fn f640(v0: Adt134) -> bool {
    let m2575 = v0;
    match m2575 {
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

pub fn f641(v0: Adt134) -> bool {
    let m2576 = v0;
    match m2576 {
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

pub fn f642(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt123, Adt126> {
    f659(v0, v2.clone(), f660(match v1 {
        Adt16::C0(_, h2577, _, _, _, _) => {
            h2577
        }
    }, f661(v2)))
}

pub fn f643(v0: Adt14) -> Adt16 {
    let m2578 = v0;
    match m2578 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2579) => {
                    h2579
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2580, _) => {
                    h2580
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2581, _, _, _, _) => {
                    h2581
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2582, _) => {
                    h2582
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2583, _, _, _) => {
                    h2583
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2584, _) => {
                    h2584
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2585, _) => {
                    h2585
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2586, _, _, _, _) => {
                    h2586
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2587, _, _, _) => {
                    h2587
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2588, _) => {
                    h2588
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2589, _, _) => {
                    h2589
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2590, _) => {
                    h2590
                }
            }
        }
    }
}

pub fn f644(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt123, Adt126> {
    f662(v0, v2, f663(match v1 {
        Adt16::C0(_, h2591, _, _, _, _) => {
            h2591
        }
    }))
}

pub fn f645(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt123 {
    Adt123::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2592, _) => {
            h2592
        }
    }), {
        let m2594 = f572(v0, Adt32::C11);
        match m2594 {
            Adt123::C0(_, h2593, _) => {
                h2593
            }
        }
    }, f664(v1, match v2 {
        Adt49::C0(_, h2595) => {
            h2595
        }
    }))
}

pub fn f646(v0: Str, p2601: List<(Str, Adt6)>, p2602: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt127> {
    let m2600 = (p2601, p2602);
    {
        let mut m2600 = m2600;
        loop {
            let step = {
                let (v1, v2) = m2600;
                let m2596 = v1;
                match m2596.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>(Ok::<List<(Str, Str)>, Adt127>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2598 = f231({
                            let (_, h2597) = v3;
                            h2597
                        });
                        match m2598 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>(Err::<List<(Str, Str)>, Adt127>(Adt127::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>((v4, List::cons(({
                                    let (h2599, _) = v3;
                                    h2599
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647(p2605: List<(Str, Str)>, p2606: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2604 = (p2605, p2606);
    {
        let mut m2604 = m2604;
        loop {
            let step = {
                let (v0, v1) = m2604;
                let m2603 = v0;
                match m2603.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2604 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(v0: Adt110, v1: Adt5) -> R<Option<Adt108>> {
    let m2608 = f665(match v0.clone() {
        Adt110::C0(_, h2607, _, _) => {
            h2607
        }
    }, v1);
    match m2608 {
        Adt130::C0 => {
            Ok(f666(v0))
        }
        Adt130::C1(v2) => {
            if f667(v2.clone()) {
                Ok(f666(v0))
            } else {
                f668(v0, v2)
            }
        }
    }
}

pub fn f649(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2609 = f176(v0, v1);
    match m2609 {
        Adt130::C0 => {
            Some(None::<Adt5>)
        }
        Adt130::C1(v2) => {
            let m2610 = f195(v2);
            match m2610 {
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

pub fn f650() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f651(v0: Str) -> Adt151 {
    let a2611 = v0.clone();
    let a2612 = Str::lit("debug");
    if equal(a2611, a2612) {
        Adt151::C1(Adt18::C0)
    } else {
        let a2613 = v0.clone();
        let a2614 = Str::lit("info");
        if equal(a2613, a2614) {
            Adt151::C1(Adt18::C1)
        } else {
            let a2615 = v0.clone();
            let a2616 = Str::lit("notice");
            if equal(a2615, a2616) {
                Adt151::C1(Adt18::C2)
            } else {
                let a2617 = v0.clone();
                let a2618 = Str::lit("warning");
                if equal(a2617, a2618) {
                    Adt151::C1(Adt18::C3)
                } else {
                    let a2619 = v0.clone();
                    let a2620 = Str::lit("error");
                    if equal(a2619, a2620) {
                        Adt151::C1(Adt18::C4)
                    } else {
                        let a2621 = v0.clone();
                        let a2622 = Str::lit("critical");
                        if equal(a2621, a2622) {
                            Adt151::C1(Adt18::C5)
                        } else {
                            let a2623 = v0.clone();
                            let a2624 = Str::lit("alert");
                            if equal(a2623, a2624) {
                                Adt151::C1(Adt18::C6)
                            } else {
                                let a2625 = v0;
                                let a2626 = Str::lit("emergency");
                                if equal(a2625, a2626) {
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

pub fn f652() -> Str {
    Str::lit("progressToken")
}

pub fn f653(v0: List<Adt41>) -> List<Adt6> {
    f294(f669(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f654(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2635 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2627, _, _, _) => {
                h2627
            }
        })), List::<(Str, Adt6)>::nil());
        let a2636 = {
            let a2633 = f423(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2628, _, _) => {
                    h2628
                }
            });
            let a2634 = {
                let a2631 = f423(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h2629, _) => {
                        h2629
                    }
                });
                let a2632 = f428(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h2630) => {
                        h2630
                    }
                });
                append_list(a2631, a2632)
            };
            append_list(a2633, a2634)
        };
        append_list(a2635, a2636)
    })
}

pub fn f655(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2637 = v0;
    match m2637 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f365(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f656(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2638 = v0;
    match m2638 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f670(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f657(v0: u64, v1: List<u64>) -> Adt150 {
    let a2639 = v0;
    let a2640 = 32u64;
    if nat_lt(a2639, a2640) {
        if f671(v0) {
            Adt150::C0(0u64, List::cons(v0, v1))
        } else {
            Adt150::C0(0u64, f658(v1))
        }
    } else {
        let a2641 = v0;
        let a2642 = 127u64;
        if nat_eq(a2641, a2642) {
            Adt150::C0(0u64, f658(v1))
        } else {
            let a2643 = v0;
            let a2644 = 194u64;
            if nat_eq(a2643, a2644) {
                Adt150::C0(1u64, v1)
            } else {
                let a2645 = v0;
                let a2646 = 226u64;
                if nat_eq(a2645, a2646) {
                    Adt150::C0(2u64, v1)
                } else {
                    Adt150::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f658(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f659(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt123, Adt126> {
    if v2 {
        Ok::<Adt123, Adt126>(f572(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt123, Adt126>(Adt126::C4(Adt84::C8(f672(f661(v1)))))
    }
}

pub fn f660(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2647 = f673(v1.clone());
    if if if bool_not(a2647) {
        true
    } else {
        f456(match v0.clone() {
            Adt17::C0(_, _, _, h2648, _) => {
                h2648
            }
        })
    } {
        let a2649 = f674(v1.clone());
        if bool_not(a2649) {
            true
        } else {
            f456(match v0.clone() {
                Adt17::C0(_, _, h2650, _, _) => {
                    h2650
                }
            })
        }
    } else {
        false
    } {
        let a2651 = f675(v1);
        if bool_not(a2651) {
            true
        } else {
            f456(match v0 {
                Adt17::C0(_, h2652, _, _, _) => {
                    h2652
                }
            })
        }
    } else {
        false
    }
}

pub fn f661(v0: Adt81) -> List<(Str, Adt64)> {
    let m2653 = v0;
    match m2653 {
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

pub fn f662(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt123, Adt126> {
    if v2 {
        Ok::<Adt123, Adt126>(f572(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt123, Adt126>(Adt126::C4(Adt84::C8(f676())))
    }
}

pub fn f663(v0: Adt17) -> bool {
    f457(match v0 {
        Adt17::C0(_, _, _, _, h2654) => {
            h2654
        }
    }, f458())
}

pub fn f664(v0: Adt16, v1: List<Adt50>) -> List<Adt125> {
    let m2656 = match v0 {
        Adt16::C0(_, _, _, _, h2655, _) => {
            h2655
        }
    };
    match m2656 {
        None => {
            List::<Adt125>::nil()
        }
        Some(v2) => {
            f677(f678(v2, v1, List::<Adt125>::nil()), List::<Adt125>::nil())
        }
    }
}

pub fn f665(p2662: List<Str>, p2663: Adt5) -> Adt130 {
    let m2661 = (p2662, p2663);
    {
        let mut m2661 = m2661;
        loop {
            let step = {
                let (v0, v1) = m2661;
                let m2657 = v0;
                match m2657.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                    }
                    Some((v2, v3)) => {
                        let m2658 = f176(v2, v1);
                        match m2658 {
                            Adt130::C0 => {
                                Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                            }
                            Adt130::C1(v4) => {
                                let m2659 = v3.clone();
                                match m2659.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt130>(Adt130::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2660 = f195(v4);
                                        match m2660 {
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
                Ok(next) => { m2661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f666(v0: Adt110) -> Option<Adt108> {
    let m2665 = match v0.clone() {
        Adt110::C0(_, _, _, h2664) => {
            h2664
        }
    };
    match m2665 {
        Adt111::C0 => {
            None::<Adt108>
        }
        Adt111::C1 => {
            Some(Adt108::C10(match v0 {
                Adt110::C0(h2666, _, _, _) => {
                    h2666
                }
            }))
        }
        Adt111::C2(_) => {
            None::<Adt108>
        }
    }
}

pub fn f667(v0: Adt6) -> bool {
    let m2667 = v0;
    match m2667 {
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

pub fn f668(v0: Adt110, v1: Adt6) -> R<Option<Adt108>> {
    let m2669 = match v0.clone() {
        Adt110::C0(_, _, _, h2668) => {
            h2668
        }
    };
    match m2669 {
        Adt111::C0 => {
            Ok(Some(Adt108::C8(match v0 {
                Adt110::C0(h2670, _, _, _) => {
                    h2670
                }
            })))
        }
        Adt111::C1 => {
            Ok(Some(Adt108::C10(match v0 {
                Adt110::C0(h2671, _, _, _) => {
                    h2671
                }
            })))
        }
        Adt111::C2(v2) => {
            if f679(match v0.clone() {
                Adt110::C0(_, _, h2672, _) => {
                    h2672
                }
            }, v2, v1)? {
                Ok(None::<Adt108>)
            } else {
                Ok(Some(Adt108::C9(match v0 {
                    Adt110::C0(h2673, _, _, _) => {
                        h2673
                    }
                })))
            }
        }
    }
}

pub fn f669(p2676: List<Adt41>, p2677: List<Adt6>) -> List<Adt6> {
    let m2675 = (p2676, p2677);
    {
        let mut m2675 = m2675;
        loop {
            let step = {
                let (v0, v1) = m2675;
                let m2674 = v0;
                match m2674.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f613(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2675 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f670(v0: Adt3) -> Str {
    let m2678 = v0;
    match m2678 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f671(v0: u64) -> bool {
    let a2679 = v0;
    let a2680 = 9u64;
    if nat_eq(a2679, a2680) {
        true
    } else {
        let a2681 = v0;
        let a2682 = 10u64;
        if nat_eq(a2681, a2682) {
            true
        } else {
            let a2683 = v0;
            let a2684 = 13u64;
            nat_eq(a2683, a2684)
        }
    }
}

pub fn f672(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f675(v0.clone()) {
        Some(f528())
    } else {
        None::<Adt5>
    }, if f674(v0.clone()) {
        Some(f528())
    } else {
        None::<Adt5>
    }, if f673(v0) {
        Some(f528())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f673(p2689: List<(Str, Adt64)>) -> bool {
    let m2688 = p2689;
    {
        let mut m2688 = m2688;
        loop {
            let step = {
                let v0 = m2688;
                let m2685 = v0;
                match m2685.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2687 = {
                            let (_, h2686) = v1;
                            h2686
                        };
                        match m2687 {
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
                Ok(next) => { m2688 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f674(p2694: List<(Str, Adt64)>) -> bool {
    let m2693 = p2694;
    {
        let mut m2693 = m2693;
        loop {
            let step = {
                let v0 = m2693;
                let m2690 = v0;
                match m2690.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2692 = {
                            let (_, h2691) = v1;
                            h2691
                        };
                        match m2692 {
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
                Ok(next) => { m2693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(p2699: List<(Str, Adt64)>) -> bool {
    let m2698 = p2699;
    {
        let mut m2698 = m2698;
        loop {
            let step = {
                let v0 = m2698;
                let m2695 = v0;
                match m2695.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2697 = {
                            let (_, h2696) = v1;
                            h2696
                        };
                        match m2697 {
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
                Ok(next) => { m2698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f676() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f458(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f677(p2702: List<Adt125>, p2703: List<Adt125>) -> List<Adt125> {
    let m2701 = (p2702, p2703);
    {
        let mut m2701 = m2701;
        loop {
            let step = {
                let (v0, v1) = m2701;
                let m2700 = v0;
                match m2700.uncons() {
                    None => {
                        Err::<(List<Adt125>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt125>, List<Adt125>), List<Adt125>>((v3, List::cons(v2, v1)))
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

pub fn f678(v0: Adt19, p2706: List<Adt50>, p2707: List<Adt125>) -> List<Adt125> {
    let m2705 = (p2706, p2707);
    {
        let mut m2705 = m2705;
        loop {
            let step = {
                let (v1, v2) = m2705;
                let m2704 = v1;
                match m2704.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt125>), List<Adt125>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt125>), List<Adt125>>((v4, List::cons(Adt125::C0(v0.clone(), v3), v2)))
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

pub fn f679(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2708 = v0;
    match m2708 {
        Adt13::C0 => {
            let m2709 = f231(v2);
            match m2709 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2710 = v1;
                    let a2711 = v3;
                    Ok(equal(a2710, a2711))
                }
            }
        }
        Adt13::C1 => {
            f680(v1, v2)
        }
        Adt13::C2 => {
            let m2712 = f194(v2);
            match m2712 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2713 = v1;
                        let a2714 = Str::lit("true");
                        Ok(equal(a2713, a2714))
                    } else {
                        let a2715 = v1;
                        let a2716 = Str::lit("false");
                        Ok(equal(a2715, a2716))
                    }
                }
            }
        }
    }
}

pub fn f680(v0: Str, v1: Adt6) -> R<bool> {
    let m2717 = f398(v1);
    match m2717 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2718 = f233(v2)?;
            match m2718 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2720 = f681(f87({
                        let a2719 = v0;
                        utf8_encode(a2719)
                    })?)?;
                    match m2720 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f380({
                                let a2721 = v3;
                                let a2722 = v4;
                                int_sub(a2721, a2722)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f681(v0: List<u64>) -> R<Option<i64>> {
    let m2723 = v0.clone();
    match m2723.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2724 = v1;
            let a2725 = 45u64;
            if nat_eq(a2724, a2725) {
                let m2726 = v2.clone();
                match m2726.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2727 = f682(v2, 0i64)?;
                        match m2727 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2728 = v5;
                                    int_neg(a2728)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f682(v0, 0i64)
            }
        }
    }
}

pub fn f682(p2734: List<u64>, p2735: i64) -> R<Option<i64>> {
    let m2733 = (p2734, p2735);
    {
        let mut m2733 = m2733;
        loop {
            let step = {
                let (v0, v1) = m2733;
                let m2729 = v0;
                match m2729.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2730 = f683(v2);
                        match m2730 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f115({
                                    let a2731 = v1;
                                    let a2732 = 10i64;
                                    int_mul(a2731, a2732)?
                                }, f293(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f683(v0: u64) -> Option<u64> {
    let a2736 = v0;
    let a2737 = 48u64;
    if if nat_lt(a2736, a2737) {
        true
    } else {
        let a2738 = 57u64;
        let a2739 = v0;
        nat_lt(a2738, a2739)
    } {
        None::<u64>
    } else {
        Some({
            let a2740 = v0;
            let a2741 = 48u64;
            nat_sub(a2740, a2741)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
