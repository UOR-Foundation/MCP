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
    Ok(Adt0::C0(f8(), None::<Str>, f9(), f10(), f11(f12()?)?, f13(), Bytes::lit(&[]), 0u64))
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
                    f87(v0.clone(), Adt92::C0(f86(match v1 {
                        Adt96::C0(h253, _) => {
                            h253
                        }
                    })?, f86(match v2 {
                        Adt96::C0(h254, _) => {
                            h254
                        }
                    })?, {
                        let (h255, _) = v3;
                        h255
                    }, match v4.clone() {
                        Adt96::C0(h256, _) => {
                            h256
                        }
                    }, v5, List::<List<u64>>::nil(), Bytes::lit(&[]), Adt14::C0(0i64, Str::lit(""), Str::lit(""), f88())), f89(v0, {
                        let a258 = match v4 {
                            Adt96::C0(_, h257) => {
                                h257
                            }
                        };
                        let a259 = 4u64;
                        nat_add(a258, a259)?
                    })?)
                }
            }
        }
    }
}

pub fn f59(v0: Adt93) -> List<u64> {
    let a270 = f17({
        let a261 = match v0.clone() {
            Adt93::C0(h260, _) => {
                h260
            }
        };
        length_list(a261)
    });
    let a271 = {
        let a268 = match v0.clone() {
            Adt93::C0(h262, _) => {
                h262
            }
        };
        let a269 = {
            let a266 = f17({
                let a264 = match v0.clone() {
                    Adt93::C0(_, h263) => {
                        h263
                    }
                };
                length_list(a264)
            });
            let a267 = match v0 {
                Adt93::C0(_, h265) => {
                    h265
                }
            };
            append_list(a266, a267)
        };
        append_list(a268, a269)
    };
    append_list(a270, a271)
}

pub fn f60(v0: List<u64>) -> List<u64> {
    let a273 = f17({
        let a272 = v0.clone();
        length_list(a272)
    });
    let a274 = v0;
    append_list(a273, a274)
}

pub fn f61(v0: u64, v1: Adt6) -> R<Adt95> {
    let m275 = f90(v1)?;
    match m275 {
        Ok(v3) => {
            Ok(Adt95::C0(v0, f22(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f62(v0: Adt97) -> u64 {
    let m276 = v0;
    match m276 {
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
    let m277 = v5;
    match m277 {
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
        Adt0::C0(h278, _, _, _, _, _, _, _) => {
            h278
        }
    }, match v0.clone() {
        Adt0::C0(_, h279, _, _, _, _, _, _) => {
            h279
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h280, _, _, _, _, _) => {
            h280
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h281, _, _, _, _) => {
            h281
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h282, _, _, _) => {
            h282
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h283, _, _) => {
            h283
        }
    }, v1, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h284) => {
            h284
        }
    })
}

pub fn f68(v0: Fn0, v1: Adt92, v2: Adt15) -> R<Result<Adt33, Adt90>> {
    let c285: Fn0 = v0;
    c285.apply(match v1 {
        Adt92::C0(_, _, _, _, _, _, _, h286) => {
            h286
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
    let m287 = f105(v0, v1, Adt104::C0)?;
    match m287 {
        Adt104::C0 => {
            Ok(Adt103::C0)
        }
        Adt104::C1(v2) => {
            let m288 = f106(v2);
            match m288 {
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
    let m289 = v0;
    match m289 {
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
    let m290 = f108(v0);
    match m290 {
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
    let m291 = v0.clone();
    match m291.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a292 = v1;
            let a293 = 91u64;
            if nat_eq(a292, a293) {
                f109(f110(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f109(f111(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f77(p299: List<Adt36>, p300: List<Str>) -> Option<Str> {
    let m298 = (p299, p300);
    {
        let mut m298 = m298;
        loop {
            let step = {
                let (v0, v1) = m298;
                let m294 = v0;
                match m294.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f112(match v2.clone() {
                            Adt36::C0(h295, _, _, _, _, _, _, _) => {
                                h295
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt36>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt36::C0(h296, _, _, _, _, _, _, _) => {
                                    h296
                                }
                            }))
                        } else {
                            Ok::<(List<Adt36>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt36::C0(h297, _, _, _, _, _, _, _) => {
                                    h297
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(p303: List<Adt12>, p304: List<Adt12>) -> List<Adt12> {
    let m302 = (p303, p304);
    {
        let mut m302 = m302;
        loop {
            let step = {
                let (v0, v1) = m302;
                let m301 = v0;
                match m301.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Adt36) -> R<Result<List<Adt12>, Adt98>> {
    let m307 = f113(match v0.clone() {
        Adt36::C0(h305, _, _, _, _, _, _, _) => {
            h305
        }
    }, f114(match v0.clone() {
        Adt36::C0(h306, _, _, _, _, _, _, _) => {
            h306
        }
    })?);
    match m307 {
        None => {
            f115(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f80(p310: u64, p311: i64) -> R<i64> {
    let m309 = (p310, p311);
    {
        let mut m309 = m309;
        loop {
            let step = {
                let (v0, v1) = m309;
                let m308 = v0;
                if m308 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m308 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f116(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Str, p317: List<Adt36>) -> Option<Adt36> {
    let m316 = p317;
    {
        let mut m316 = m316;
        loop {
            let step = {
                let v1 = m316;
                let m312 = v1;
                match m312.uncons() {
                    None => {
                        Err::<List<Adt36>, Option<Adt36>>(None::<Adt36>)
                    }
                    Some((v2, v3)) => {
                        let a314 = match v2.clone() {
                            Adt36::C0(h313, _, _, _, _, _, _, _) => {
                                h313
                            }
                        };
                        let a315 = v0.clone();
                        if equal(a314, a315) {
                            Err::<List<Adt36>, Option<Adt36>>(Some(v2))
                        } else {
                            Ok::<List<Adt36>, Option<Adt36>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m316 = next; }
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

pub fn f85(p320: List<Adt93>, p321: List<Adt93>) -> List<Adt93> {
    let m319 = (p320, p321);
    {
        let mut m319 = m319;
        loop {
            let step = {
                let (v0, v1) = m319;
                let m318 = v0;
                match m318.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
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

pub fn f86(v0: Bytes) -> R<List<u64>> {
    f120(v0.clone(), {
        let a322 = v0;
        length_bytes(a322)
    }, List::<u64>::nil())
}

pub fn f87(v0: Bytes, v1: Adt92, v2: Result<(Adt14, u64), Adt94>) -> R<Result<Adt92, Adt94>> {
    let m323 = v2;
    match m323 {
        Ok(v4) => {
            f121(v0, Adt92::C0(match v1.clone() {
                Adt92::C0(h324, _, _, _, _, _, _, _) => {
                    h324
                }
            }, match v1.clone() {
                Adt92::C0(_, h325, _, _, _, _, _, _) => {
                    h325
                }
            }, match v1.clone() {
                Adt92::C0(_, _, h326, _, _, _, _, _) => {
                    h326
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, h327, _, _, _, _) => {
                    h327
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, h328, _, _, _) => {
                    h328
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, _, h329, _, _) => {
                    h329
                }
            }, match v1 {
                Adt92::C0(_, _, _, _, _, _, h330, _) => {
                    h330
                }
            }, {
                let (h331, _) = v4;
                h331
            }), {
                let (_, h332) = v4;
                h332
            })
        }
        Err(v3) => {
            Ok(Err::<Adt92, Adt94>(v3))
        }
    }
}

pub fn f88() -> Bytes {
    Bytes::lit(&[])
}

pub fn f89(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt94>> {
    let m333 = f122(v0.clone(), v1)?;
    match m333 {
        None => {
            Ok(Err::<(Adt14, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m336 = f123(v0.clone(), {
                let a334 = v1;
                let a335 = 8u64;
                nat_add(a334, a335)?
            })?;
            match m336 {
                Ok(v4) => {
                    f124(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt14, u64), Adt94>(v3))
                }
            }
        }
    }
}

pub fn f90(v0: Adt6) -> R<Result<List<u64>, Adt100>> {
    f125(f126(), v0)
}

pub fn f91() -> u64 {
    403u64
}

pub fn f92(v0: Option<Adt99>, v1: Adt81) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), {
        let a337 = f128(v0);
        let a338 = List::cons((Str::lit("error"), f129(v1)), List::<(Str, Adt6)>::nil());
        append_list(a337, a338)
    })
}

pub fn f93() -> i64 {
    -32000i64
}

pub fn f94(v0: Adt97) -> Str {
    let m339 = v0;
    match m339 {
        Adt97::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt97::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f95(p346: List<u64>, p347: List<u64>) -> bool {
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
    let m348 = v5;
    match m348 {
        Ok(v7) => {
            f134(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f61(f132(v6.clone()), f63(f133(v6.clone()), f98(v6)))
        }
    }
}

pub fn f100(v0: Adt6) -> R<Result<Adt105, Adt101>> {
    let m349 = v0;
    match m349 {
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
    let m350 = f137(v0.clone())?;
    match m350 {
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

pub fn f105(p355: List<Adt93>, v1: List<u64>, p356: Adt104) -> R<Adt104> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v0, v2) = m354;
                let m351 = v0;
                match m351.uncons() {
                    None => {
                        Err::<(List<Adt93>, Adt104), R<Adt104>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f140(match v3.clone() {
                            Adt93::C0(h352, _) => {
                                h352
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt93>, Adt104), R<Adt104>>((v4, f141(v2, match v3 {
                                Adt93::C0(_, h353) => {
                                    h353
                                }
                            })))
                        } else {
                            Ok::<(List<Adt93>, Adt104), R<Adt104>>((v4, v2))
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

pub fn f108(p361: List<u64>) -> Option<List<u64>> {
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
                            Err::<List<u64>, Option<List<u64>>>(f148(v2))
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

pub fn f109(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m362 = v0;
    match m362 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f149({
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

pub fn f110(p369: List<u64>, p370: List<u64>) -> Option<(List<u64>, List<u64>)> {
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
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(List::cons(v2, v1), List::<u64>::nil()), v3)))
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

pub fn f111(p375: List<u64>, p376: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m374 = (p375, p376);
    {
        let mut m374 = m374;
        loop {
            let step = {
                let (v0, v1) = m374;
                let m371 = v0;
                match m371.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a372 = v2;
                        let a373 = 58u64;
                        if nat_eq(a372, a373) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(v1, List::<u64>::nil()), List::cons(v2, v3))))
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

pub fn f112(v0: Str, p381: List<Str>) -> bool {
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

pub fn f113(v0: Str, v1: List<u64>) -> Option<Adt98> {
    if f151(v1) {
        Some(Adt98::C0(v0))
    } else {
        None::<Adt98>
    }
}

pub fn f114(v0: Str) -> R<List<u64>> {
    f86({
        let a382 = v0;
        utf8_encode(a382)
    })
}

pub fn f115(v0: Adt36) -> R<Result<List<Adt12>, Adt98>> {
    let m383 = f152(v0.clone());
    match m383 {
        None => {
            f153(v0.clone(), f154(f155(), match v0.clone() {
                Adt36::C0(h384, _, _, _, _, _, _, _) => {
                    h384
                }
            }, Adt6::C5({
                let m387 = match v0.clone() {
                    Adt36::C0(_, _, _, _, h386, _, _, _) => {
                        h386
                    }
                };
                match m387 {
                    Adt5::C0(h385) => {
                        h385
                    }
                }
            }), List::cons(Adt106::C0(Adt6::C5({
                let m390 = match v0 {
                    Adt36::C0(_, _, _, _, h389, _, _, _) => {
                        h389
                    }
                };
                match m390 {
                    Adt5::C0(h388) => {
                        h388
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
    let a392 = v0;
    let a393 = {
        let a391 = v1;
        int_neg(a391)?
    };
    int_sub(a392, a393)
}

pub fn f117(v0: Adt36, v1: Adt22, v2: Fn3) -> R<Result<Adt33, Adt90>> {
    let m400 = f157(Adt6::C5({
        let m396 = match v0.clone() {
            Adt36::C0(_, _, _, _, h395, _, _, _) => {
                h395
            }
        };
        match m396 {
            Adt5::C0(h394) => {
                h394
            }
        }
    }), Adt6::C5({
        let m399 = match v0 {
            Adt36::C0(_, _, _, _, h398, _, _, _) => {
                h398
            }
        };
        match m399 {
            Adt5::C0(h397) => {
                h397
            }
        }
    }), f158(v1.clone()))?;
    match m400 {
        Adt122::C0 => {
            let c401: Fn3 = v2;
            c401.apply(v1)
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

pub fn f120(v0: Bytes, p404: u64, p405: List<u64>) -> R<List<u64>> {
    let m403 = (p404, p405);
    {
        let mut m403 = m403;
        loop {
            let step = {
                let (v1, v2) = m403;
                let m402 = v1;
                if m402 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m402 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f56(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Bytes, v1: Adt92, v2: u64) -> R<Result<Adt92, Adt94>> {
    let m406 = f162(v0.clone(), v2)?;
    match m406 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(v2)))
        }
        Some(v3) => {
            f163(v0, Adt92::C0(match v1.clone() {
                Adt92::C0(h407, _, _, _, _, _, _, _) => {
                    h407
                }
            }, match v1.clone() {
                Adt92::C0(_, h408, _, _, _, _, _, _) => {
                    h408
                }
            }, match v1.clone() {
                Adt92::C0(_, _, h409, _, _, _, _, _) => {
                    h409
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, h410, _, _, _, _) => {
                    h410
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, h411, _, _, _) => {
                    h411
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, _, h412, _, _) => {
                    h412
                }
            }, match v3.clone() {
                Adt96::C0(h413, _) => {
                    h413
                }
            }, match v1 {
                Adt92::C0(_, _, _, _, _, _, _, h414) => {
                    h414
                }
            }), match v3 {
                Adt96::C0(_, h415) => {
                    h415
                }
            })
        }
    }
}

pub fn f122(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a419 = {
        let a416 = v1;
        let a417 = 8u64;
        nat_add(a416, a417)?
    };
    let a420 = {
        let a418 = v0.clone();
        length_bytes(a418)
    };
    if nat_le(a419, a420) {
        Ok(Some(f164(f164(f164(f164(f164(f164(f164(f164(0i64, f56(v0.clone(), v1)?)?, f56(v0.clone(), {
            let a421 = v1;
            let a422 = 1u64;
            nat_add(a421, a422)?
        })?)?, f56(v0.clone(), {
            let a423 = v1;
            let a424 = 2u64;
            nat_add(a423, a424)?
        })?)?, f56(v0.clone(), {
            let a425 = v1;
            let a426 = 3u64;
            nat_add(a425, a426)?
        })?)?, f56(v0.clone(), {
            let a427 = v1;
            let a428 = 4u64;
            nat_add(a427, a428)?
        })?)?, f56(v0.clone(), {
            let a429 = v1;
            let a430 = 5u64;
            nat_add(a429, a430)?
        })?)?, f56(v0.clone(), {
            let a431 = v1;
            let a432 = 6u64;
            nat_add(a431, a432)?
        })?)?, f56(v0, {
            let a433 = v1;
            let a434 = 7u64;
            nat_add(a433, a434)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f123(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt94>> {
    let m435 = f15(v0, v1)?;
    match m435 {
        None => {
            Ok(Err::<(Str, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m437 = f165(f86(match v2.clone() {
                Adt96::C0(h436, _) => {
                    h436
                }
            })?);
            match m437 {
                None => {
                    Ok(Err::<(Str, u64), Adt94>(Adt94::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt94>((v3, match v2 {
                        Adt96::C0(_, h438) => {
                            h438
                        }
                    })))
                }
            }
        }
    }
}

pub fn f124(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt14, u64), Adt94>> {
    let m440 = f123(v0.clone(), {
        let (_, h439) = v2;
        h439
    })?;
    match m440 {
        Ok(v4) => {
            f166(v0, v1, {
                let (h441, _) = v2;
                h441
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt14, u64), Adt94>(v3))
        }
    }
}

pub fn f125(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt100>> {
    let m442 = f167(v0, List::cons(Adt107::C1(0u64, v1), List::<Adt107>::nil()), List::<u64>::nil())?;
    match m442 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt100>(f168(v2, List::<u64>::nil())))
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
    let m443 = v0;
    match m443 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f169(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f129(v0: Adt81) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt81::C0(h444, _, _) => {
            h444
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt81::C0(_, h445, _) => {
            h445
        }
    })), f170(match v0 {
        Adt81::C0(_, _, h446) => {
            h446
        }
    }))))
}

pub fn f130(v0: Adt101) -> i64 {
    let m447 = v0;
    match m447 {
        Adt101::C0 => {
            f171()
        }
        Adt101::C1(_, _) => {
            f172()
        }
    }
}

pub fn f131(v0: Adt101) -> Str {
    let m448 = v0;
    match m448 {
        Adt101::C0 => {
            Str::lit("Parse error")
        }
        Adt101::C1(_, v2) => {
            f173(v2)
        }
    }
}

pub fn f132(v0: Adt101) -> u64 {
    let m449 = v0;
    match m449 {
        Adt101::C0 => {
            f97()
        }
        Adt101::C1(_, _) => {
            f97()
        }
    }
}

pub fn f133(v0: Adt101) -> Option<Adt99> {
    let m450 = v0;
    match m450 {
        Adt101::C0 => {
            None::<Adt99>
        }
        Adt101::C1(v1, _) => {
            v1
        }
    }
}

pub fn f134(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt93>, v5: Adt105) -> R<Adt95> {
    f174(v0, f175(v4.clone())?, f176(v1.clone(), v2, v3, Some(f177(v4, match v1 {
        Adt0::C0(_, _, _, _, h451, _, _, _) => {
            h451
        }
    }, f178(v5.clone()), match v5.clone() {
        Adt105::C0(_, h452, _) => {
            h452
        }
    })?), f179(), v5)?)
}

pub fn f135(v0: Adt5) -> R<Result<Adt105, Adt101>> {
    let m453 = f180(v0.clone())?;
    match m453 {
        Adt134::C0 => {
            let m454 = f181(Str::lit("method"), v0.clone());
            match m454 {
                Adt135::C0 => {
                    Ok(f136(None::<Adt99>, f182(v0)))
                }
                Adt135::C1(_) => {
                    Ok(f136(None::<Adt99>, if f183(v0) {
                        Adt102::C8
                    } else {
                        Adt102::C7
                    }))
                }
            }
        }
        Adt134::C1(v2) => {
            Ok(f184(v2, v0))
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
    Ok(f185({
        let a455 = v0.clone();
        length_list(a455)
    }, f186(Adt114::C0, 0u64, 0u64, v0)?))
}

pub fn f138(p462: Adt115, p463: u64, p464: List<u64>) -> R<Result<Adt6, Adt100>> {
    let m461 = (p462, (p463, p464));
    {
        let mut m461 = m461;
        loop {
            let step = {
                let (v0, (v1, v2)) = m461;
                let m456 = v2;
                match m456.uncons() {
                    None => {
                        let m457 = f187(v0, v1)?;
                        match m457 {
                            Ok(v3) => {
                                Err::<(Adt115, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Ok::<Adt6, Adt100>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt115, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Err::<Adt6, Adt100>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m458 = f188(v0, v1, v5)?;
                        match m458 {
                            Adt136::C0(v7) => {
                                Err::<(Adt115, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Err::<Adt6, Adt100>(v7)))
                            }
                            Adt136::C1(v8) => {
                                Ok::<(Adt115, (u64, List<u64>)), R<Result<Adt6, Adt100>>>((v8, ({
                                    let a459 = v1;
                                    let a460 = 1u64;
                                    nat_add(a459, a460)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139() -> Adt115 {
    Adt115::C0(Adt116::C0, List::<Adt120>::nil(), 0u64)
}

pub fn f140(p471: List<u64>, p472: List<u64>) -> R<bool> {
    let m470 = (p471, p472);
    {
        let mut m470 = m470;
        loop {
            let step = {
                let (v0, v1) = m470;
                let m465 = v0;
                match m465.uncons() {
                    None => {
                        let m466 = v1;
                        match m466.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m467 = v1;
                        match m467.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a468 = f189(v4)?;
                                let a469 = f189(v6)?;
                                if nat_eq(a468, a469) {
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
                Ok(next) => { m470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt104, v1: List<u64>) -> Adt104 {
    let m473 = v0;
    match m473 {
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
    f150(f190(f150(f190(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f143(p476: List<u64>) -> bool {
    let m475 = p476;
    {
        let mut m475 = m475;
        loop {
            let step = {
                let v0 = m475;
                let m474 = v0;
                match m474.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f191(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m475 = next; }
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

pub fn f147(v0: List<u64>, p479: List<List<u64>>) -> R<bool> {
    let m478 = p479;
    {
        let mut m478 = m478;
        loop {
            let step = {
                let v1 = m478;
                let m477 = v1;
                match m477.uncons() {
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
                Ok(next) => { m478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: List<u64>) -> Option<List<u64>> {
    let m480 = v0;
    match m480.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m481 = v2;
            match m481.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a482 = v1;
                    let a483 = 47u64;
                    if if nat_eq(a482, a483) {
                        let a484 = v3;
                        let a485 = 47u64;
                        nat_eq(a484, a485)
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
    let m486 = v0;
    match m486.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a487 = v1;
            let a488 = 58u64;
            if nat_eq(a487, a488) {
                let m489 = v2.clone();
                match m489.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f192(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f150(p492: List<u64>, p493: List<u64>) -> List<u64> {
    let m491 = (p492, p493);
    {
        let mut m491 = m491;
        loop {
            let step = {
                let (v0, v1) = m491;
                let m490 = v0;
                match m490.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: List<u64>) -> bool {
    let m494 = v0.clone();
    match m494.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a498 = if f193(v0.clone(), false) {
                let a496 = {
                    let a495 = v0;
                    length_list(a495)
                };
                let a497 = f194();
                nat_le(a496, a497)
            } else {
                false
            };
            bool_not(a498)
        }
    }
}

pub fn f152(v0: Adt36) -> Option<Adt98> {
    let m503 = f195(match v0.clone() {
        Adt36::C0(h499, _, _, _, _, _, _, _) => {
            h499
        }
    }, {
        let m502 = match v0.clone() {
            Adt36::C0(_, _, _, _, h501, _, _, _) => {
                h501
            }
        };
        match m502 {
            Adt5::C0(h500) => {
                h500
            }
        }
    });
    match m503 {
        None => {
            f196(match v0.clone() {
                Adt36::C0(h504, _, _, _, _, _, _, _) => {
                    h504
                }
            }, {
                let m507 = match v0 {
                    Adt36::C0(_, _, _, _, h506, _, _, _) => {
                        h506
                    }
                };
                match m507 {
                    Adt5::C0(h505) => {
                        h505
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
    let m508 = v1;
    match m508 {
        Ok(v3) => {
            let m509 = f197(v0)?;
            match m509 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt98>(f78(match v3 {
                        Adt121::C0(h510, _) => {
                            h510
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

pub fn f154(p517: u64, v1: Str, v2: Adt6, p518: List<Adt106>, p519: Adt121) -> R<Result<Adt121, Adt98>> {
    let m516 = (p517, (p518, p519));
    {
        let mut m516 = m516;
        loop {
            let step = {
                let (v0, (v3, v4)) = m516;
                let m511 = v0;
                if m511 == 0 {
                    Err::<(u64, (List<Adt106>, Adt121)), R<Result<Adt121, Adt98>>>(Ok(Err::<Adt121, Adt98>(Adt98::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m511 - 1;
                    let m512 = v3;
                    match m512.uncons() {
                        None => {
                            Err::<(u64, (List<Adt106>, Adt121)), R<Result<Adt121, Adt98>>>(Ok(Ok::<Adt121, Adt98>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m513 = f198(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m513 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt106>, Adt121)), R<Result<Adt121, Adt98>>>((v5, ({
                                        let (h514, _) = v9;
                                        h514
                                    }, {
                                        let (_, h515) = v9;
                                        h515
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
                Ok(next) => { m516 = next; }
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
    f199(v0, f200(), Adt124::C0, List::cons(Adt125::C0(v1, v2, List::<Adt123>::nil()), List::<Adt125>::nil()))
}

pub fn f158(v0: Adt22) -> Adt6 {
    let m521 = match v0 {
        Adt22::C0(_, _, h520, _, _) => {
            h520
        }
    };
    match m521 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h522) => {
                    h522
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
    let v2: Str = f201(v0)?;
    let a525 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a523 = v2.clone();
        let a524 = Str::lit("");
        if equal(a523, a524) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a526 = Str::lit(" ");
    Ok(join(a525, a526))
}

pub fn f161(v0: Adt36, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m528 = {
        let c527: Fn1 = v1;
        c527.apply(v2)
    };
    match m528 {
        Ok(v4) => {
            f202(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f162(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let a530 = v1;
    let a531 = {
        let a529 = v0.clone();
        length_bytes(a529)
    };
    if nat_eq(a530, a531) {
        Ok(Some(Adt96::C0(Bytes::lit(&[]), v1)))
    } else {
        f15(v0, v1)
    }
}

pub fn f163(v0: Bytes, v1: Adt92, v2: u64) -> R<Result<Adt92, Adt94>> {
    let a533 = v2;
    let a534 = {
        let a532 = v0.clone();
        length_bytes(a532)
    };
    if nat_eq(a533, a534) {
        Ok(Ok::<Adt92, Adt94>(v1))
    } else {
        let m535 = f32(v0.clone(), v2)?;
        match m535 {
            None => {
                Ok(Err::<Adt92, Adt94>(Adt94::C0(v2)))
            }
            Some(v3) => {
                let m538 = f203(v0.clone(), {
                    let a536 = v2;
                    let a537 = 4u64;
                    nat_add(a536, a537)?
                }, v3, List::<List<u64>>::nil())?;
                match m538 {
                    Ok(v5) => {
                        let a541 = {
                            let (_, h539) = v5.clone();
                            h539
                        };
                        let a542 = {
                            let a540 = v0;
                            length_bytes(a540)
                        };
                        if nat_eq(a541, a542) {
                            Ok(Ok::<Adt92, Adt94>(f204(v1, {
                                let (h543, _) = v5;
                                h543
                            })))
                        } else {
                            Ok(Err::<Adt92, Adt94>(Adt94::C1({
                                let (_, h544) = v5;
                                h544
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

pub fn f164(v0: i64, v1: u64) -> R<i64> {
    f116({
        let a545 = v0;
        let a546 = 256i64;
        int_mul(a545, a546)?
    }, f80(v1, 0i64)?)
}

pub fn f165(v0: List<u64>) -> Option<Str> {
    let a547 = f205(f88(), v0);
    utf8_decode(a547)
}

pub fn f166(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt14, u64), Adt94>> {
    let m549 = f15(v0, {
        let (_, h548) = v3.clone();
        h548
    })?;
    match m549 {
        None => {
            Ok(Err::<(Adt14, u64), Adt94>(Adt94::C0({
                let (_, h550) = v3;
                h550
            })))
        }
        Some(v4) => {
            Ok(f206(v1, v2, {
                let (h551, _) = v3;
                h551
            }, v4))
        }
    }
}

pub fn f167(p556: u64, p557: List<Adt107>, p558: List<u64>) -> R<Result<List<u64>, Adt100>> {
    let m555 = (p556, (p557, p558));
    {
        let mut m555 = m555;
        loop {
            let step = {
                let (v0, (v1, v2)) = m555;
                let m552 = v0;
                if m552 == 0 {
                    Err::<(u64, (List<Adt107>, List<u64>)), R<Result<List<u64>, Adt100>>>(Ok(Err::<List<u64>, Adt100>(Adt100::C13)))
                } else {
                    let v3 = m552 - 1;
                    let m553 = v1;
                    match m553.uncons() {
                        None => {
                            Err::<(u64, (List<Adt107>, List<u64>)), R<Result<List<u64>, Adt100>>>(Ok(Ok::<List<u64>, Adt100>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m554 = f207(v4, v5, v2)?;
                            match m554 {
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
                Ok(next) => { m555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(p561: List<u64>, p562: List<u64>) -> List<u64> {
    let m560 = (p561, p562);
    {
        let mut m560 = m560;
        loop {
            let step = {
                let (v0, v1) = m560;
                let m559 = v0;
                match m559.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Adt99) -> Adt6 {
    let m563 = v0;
    match m563 {
        Adt99::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt99::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f170(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m564 = v0;
    match m564 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f171() -> i64 {
    -32700i64
}

pub fn f172() -> i64 {
    -32600i64
}

pub fn f173(v0: Adt102) -> Str {
    let m565 = v0;
    match m565 {
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

pub fn f174(v0: Adt14, v1: bool, v2: Adt127) -> R<Adt95> {
    let m567 = match v2.clone() {
        Adt127::C0(_, h566) => {
            h566
        }
    };
    match m567 {
        Ok(v3) => {
            let m568 = f208(v3.clone())?;
            match m568 {
                None => {
                    Ok(f6())
                }
                Some(v4) => {
                    f209(v0, f210(match v3.clone() {
                        Adt128::C0(h569, _, _) => {
                            h569
                        }
                    }), f211(v1, f61(f212(), f213(match v2 {
                        Adt127::C0(h570, _) => {
                            h570
                        }
                    }, v4))?, f214(f215(match v3.clone() {
                        Adt128::C0(_, _, h571) => {
                            h571
                        }
                    }, List::<List<u64>>::nil())?, f216(f210(match v3 {
                        Adt128::C0(h572, _, _) => {
                            h572
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f61(f217(v5.clone()), f63(Some(match v2 {
                Adt127::C0(h573, _) => {
                    h573
                }
            }), f218(v5)?))
        }
    }
}

pub fn f175(p578: List<Adt93>) -> R<bool> {
    let m577 = p578;
    {
        let mut m577 = m577;
        loop {
            let step = {
                let v0 = m577;
                let m574 = v0;
                match m574.uncons() {
                    None => {
                        Err::<List<Adt93>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f140(match v1.clone() {
                            Adt93::C0(h575, _) => {
                                h575
                            }
                        }, f219())? {
                            if f220(f221(44u64, match v1 {
                                Adt93::C0(_, h576) => {
                                    h576
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
                Ok(next) => { m577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Adt0, v1: Fn2, v2: u64, v3: Option<Adt108>, v4: List<Str>, v5: Adt105) -> R<Adt127> {
    Ok(Adt127::C0(match v5.clone() {
        Adt105::C0(h579, _, _) => {
            h579
        }
    }, f222(v0, v1, v2, f223(v3, v4, v5)?)?))
}

pub fn f177(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt108> {
    let v4: Adt133 = f224(Adt110::C0, f71(v0.clone(), f225())?);
    let v5: Adt133 = f224(Adt110::C1, f71(v0.clone(), f226())?);
    let v6: Adt133 = if f227(v3) {
        f224(Adt110::C2, f228(f71(v0.clone(), f229())?)?)
    } else {
        f230()
    };
    Ok(Adt108::C0(match v4.clone() {
        Adt133::C0(h580, _) => {
            h580
        }
    }, match v5.clone() {
        Adt133::C0(h581, _) => {
            h581
        }
    }, match v6.clone() {
        Adt133::C0(h582, _) => {
            h582
        }
    }, f231(match v4 {
        Adt133::C0(_, h583) => {
            h583
        }
    }, f231(match v5 {
        Adt133::C0(_, h584) => {
            h584
        }
    }, match v6 {
        Adt133::C0(_, h585) => {
            h585
        }
    })), f232(v0, v1, v2)?))
}

pub fn f178(v0: Adt105) -> Option<Str> {
    let a587 = match v0.clone() {
        Adt105::C0(_, h586, _) => {
            h586
        }
    };
    let a588 = Str::lit("tools/call");
    if equal(a587, a588) {
        let m590 = match v0 {
            Adt105::C0(_, _, h589) => {
                h589
            }
        };
        match m590 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m591 = f181(Str::lit("name"), v1);
                match m591 {
                    Adt135::C0 => {
                        None::<Str>
                    }
                    Adt135::C1(v2) => {
                        f233(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f179() -> List<Str> {
    List::cons(f234(), List::<Str>::nil())
}

pub fn f180(v0: Adt5) -> R<Adt134> {
    let m592 = f181(Str::lit("id"), v0);
    match m592 {
        Adt135::C0 => {
            Ok(Adt134::C0)
        }
        Adt135::C1(v1) => {
            let m593 = v1;
            match m593 {
                Adt6::C0 => {
                    Ok(Adt134::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt134::C2)
                }
                Adt6::C2(v3) => {
                    let m594 = f235(v3)?;
                    match m594 {
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

pub fn f181(v0: Str, v1: Adt5) -> Adt135 {
    f236(v0, match v1 {
        Adt5::C0(h595) => {
            h595
        }
    })
}

pub fn f182(v0: Adt5) -> Adt102 {
    if f183(v0) {
        Adt102::C8
    } else {
        Adt102::C3
    }
}

pub fn f183(v0: Adt5) -> bool {
    if f237(Str::lit("result"), v0.clone()) {
        true
    } else {
        f237(Str::lit("error"), v0)
    }
}

pub fn f184(v0: Adt99, v1: Adt5) -> Result<Adt105, Adt101> {
    let m596 = f181(Str::lit("jsonrpc"), v1.clone());
    match m596 {
        Adt135::C0 => {
            f136(Some(v0), Adt102::C2)
        }
        Adt135::C1(v2) => {
            let m597 = f233(v2);
            match m597 {
                None => {
                    f136(Some(v0), Adt102::C2)
                }
                Some(v3) => {
                    let a598 = v3;
                    let a599 = f127();
                    if equal(a598, a599) {
                        f238(v0, v1)
                    } else {
                        f136(Some(v0), Adt102::C2)
                    }
                }
            }
        }
    }
}

pub fn f185(v0: u64, v1: u64) -> Adt113 {
    let a600 = v1;
    let a601 = 0u64;
    if nat_eq(a600, a601) {
        Adt113::C0
    } else {
        let a602 = v1;
        let a603 = 2u64;
        if nat_eq(a602, a603) {
            Adt113::C2(v0)
        } else {
            Adt113::C1({
                let a607 = v0;
                let a608 = {
                    let a604 = v1;
                    let a605 = 3u64;
                    let a606 = 0u64;
                    nat_quot(a604, a605, a606)
                };
                nat_sub(a607, a608)
            })
        }
    }
}

pub fn f186(p636: Adt114, p637: u64, p638: u64, p639: List<u64>) -> R<u64> {
    let m635 = (p636, (p637, (p638, p639)));
    {
        let mut m635 = m635;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m635;
                let m609 = v3;
                match m609.uncons() {
                    None => {
                        let m610 = v0;
                        match m610 {
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
                        let m611 = v0;
                        match m611 {
                            Adt114::C0 => {
                                let a612 = v4;
                                let a613 = 128u64;
                                if nat_lt(a612, a613) {
                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a614 = v4;
                                    let a615 = 194u64;
                                    if nat_lt(a614, a615) {
                                        let v6: u64 = f239({
                                            let a616 = List::cons(v4, v5);
                                            length_list(a616)
                                        })?;
                                        Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a617 = v4;
                                        let a618 = 224u64;
                                        if nat_lt(a617, a618) {
                                            Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a619 = v4;
                                            let a620 = 224u64;
                                            if nat_eq(a619, a620) {
                                                Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a621 = v4;
                                                let a622 = 237u64;
                                                if nat_eq(a621, a622) {
                                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a623 = v4;
                                                    let a624 = 240u64;
                                                    if nat_lt(a623, a624) {
                                                        Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a625 = v4;
                                                        let a626 = 240u64;
                                                        if nat_eq(a625, a626) {
                                                            Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a627 = v4;
                                                            let a628 = 244u64;
                                                            if nat_eq(a627, a628) {
                                                                Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a629 = v4;
                                                                let a630 = 244u64;
                                                                if nat_lt(a629, a630) {
                                                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f239({
                                                                        let a631 = List::cons(v4, v5);
                                                                        length_list(a631)
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
                                if f240(v4, v1, v2) {
                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f239({
                                        let a632 = List::cons(v4, v5);
                                        length_list(a632)
                                    })?;
                                    Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt114::C2 => {
                                if f240(v4, v1, v2) {
                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f239({
                                        let a633 = List::cons(v4, v5);
                                        length_list(a633)
                                    })?;
                                    Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt114::C3 => {
                                if f240(v4, v1, v2) {
                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f239({
                                        let a634 = List::cons(v4, v5);
                                        length_list(a634)
                                    })?;
                                    Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
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

pub fn f187(v0: Adt115, v1: u64) -> R<Result<Adt6, Adt100>> {
    let m641 = match v0.clone() {
        Adt115::C0(h640, _, _) => {
            h640
        }
    };
    match m641 {
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
            let m642 = f241(v3)?;
            match m642 {
                Adt141::C0 => {
                    Ok(Err::<Adt6, Adt100>(Adt100::C7(v1)))
                }
                Adt141::C1 => {
                    Ok(Err::<Adt6, Adt100>(Adt100::C8(v1)))
                }
                Adt141::C2(v4) => {
                    let m647 = {
                        let m646 = f242(v4, match v0.clone() {
                            Adt115::C0(_, h644, _) => {
                                h644
                            }
                        }, match v0 {
                            Adt115::C0(_, _, h645) => {
                                h645
                            }
                        });
                        match m646 {
                            Adt115::C0(h643, _, _) => {
                                h643
                            }
                        }
                    };
                    match m647 {
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

pub fn f188(v0: Adt115, v1: u64, v2: u64) -> R<Adt136> {
    let m649 = match v0.clone() {
        Adt115::C0(h648, _, _) => {
            h648
        }
    };
    match m649 {
        Adt116::C0 => {
            if f243(v2) {
                Ok(Adt136::C1(v0))
            } else {
                f244(v0, v1, v2)
            }
        }
        Adt116::C1 => {
            if f243(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a650 = v2;
                let a651 = 93u64;
                if nat_eq(a650, a651) {
                    Ok(f245(v0, v1))
                } else {
                    f244(v0, v1, v2)
                }
            }
        }
        Adt116::C2 => {
            Ok(f246(v0, v1, v2))
        }
        Adt116::C3 => {
            if f243(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a652 = v2;
                let a653 = 125u64;
                if nat_eq(a652, a653) {
                    Ok(f247(v0, v1))
                } else {
                    let a654 = v2;
                    let a655 = 34u64;
                    if nat_eq(a654, a655) {
                        Ok(f248(v0, Adt116::C6(Adt117::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f249(Adt100::C2(v1)))
                    }
                }
            }
        }
        Adt116::C4 => {
            if f243(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a656 = v2;
                let a657 = 34u64;
                if nat_eq(a656, a657) {
                    Ok(f248(v0, Adt116::C6(Adt117::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f249(Adt100::C2(v1)))
                }
            }
        }
        Adt116::C5 => {
            if f243(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a658 = v2;
                let a659 = 58u64;
                if nat_eq(a658, a659) {
                    Ok(f248(v0, Adt116::C0))
                } else {
                    Ok(f249(Adt100::C2(v1)))
                }
            }
        }
        Adt116::C6(v3, v4, v5) => {
            f250(v0, v1, v2, v3, v4, v5)
        }
        Adt116::C7(v6, v7) => {
            f251(v0, v1, v2, v6, v7)
        }
        Adt116::C8(v8) => {
            f252(v0, v1, v2, v8)
        }
        Adt116::C9(_) => {
            Ok(f246(v0, v1, v2))
        }
    }
}

pub fn f189(v0: u64) -> R<u64> {
    let a660 = 64u64;
    let a661 = v0;
    if if nat_lt(a660, a661) {
        let a662 = v0;
        let a663 = 91u64;
        nat_lt(a662, a663)
    } else {
        false
    } {
        let a664 = v0;
        let a665 = 32u64;
        nat_add(a664, a665)
    } else {
        Ok(v0)
    }
}

pub fn f190(p668: List<u64>) -> List<u64> {
    let m667 = p668;
    {
        let mut m667 = m667;
        loop {
            let step = {
                let v0 = m667;
                let m666 = v0;
                match m666.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f253(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: u64) -> bool {
    let a669 = v0;
    let a670 = 9u64;
    if nat_eq(a669, a670) {
        true
    } else {
        let a671 = 31u64;
        let a672 = v0;
        if nat_lt(a671, a672) {
            let a673 = v0;
            let a674 = 127u64;
            nat_lt(a673, a674)
        } else {
            false
        }
    }
}

pub fn f192(p677: List<u64>) -> bool {
    let m676 = p677;
    {
        let mut m676 = m676;
        loop {
            let step = {
                let v0 = m676;
                let m675 = v0;
                match m675.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f254(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m676 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(p680: List<u64>, v1: bool) -> bool {
    let m679 = p680;
    {
        let mut m679 = m679;
        loop {
            let step = {
                let v0 = m679;
                let m678 = v0;
                match m678.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f255(v2)
                        } else {
                            f256(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194() -> u64 {
    64u64
}

pub fn f195(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt98> {
    let m681 = f236(Str::lit("type"), v1);
    match m681 {
        Adt135::C0 => {
            Some(Adt98::C3(v0))
        }
        Adt135::C1(v2) => {
            let m682 = f233(v2);
            match m682 {
                None => {
                    Some(Adt98::C3(v0))
                }
                Some(v3) => {
                    let a683 = v3;
                    let a684 = Str::lit("object");
                    if equal(a683, a684) {
                        None::<Adt98>
                    } else {
                        Some(Adt98::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f196(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt98> {
    let m685 = f236(Str::lit("$schema"), v1);
    match m685 {
        Adt135::C0 => {
            None::<Adt98>
        }
        Adt135::C1(v2) => {
            let m686 = f233(v2);
            match m686 {
                None => {
                    Some(Adt98::C2(v0))
                }
                Some(v3) => {
                    let a687 = v3;
                    let a688 = f257();
                    if equal(a687, a688) {
                        None::<Adt98>
                    } else {
                        Some(Adt98::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f197(v0: Adt36) -> R<Option<Adt98>> {
    let m690 = match v0.clone() {
        Adt36::C0(_, _, _, _, _, h689, _, _) => {
            h689
        }
    };
    match m690 {
        None => {
            Ok(None::<Adt98>)
        }
        Some(v1) => {
            let m693 = f196(match v0.clone() {
                Adt36::C0(h691, _, _, _, _, _, _, _) => {
                    h691
                }
            }, match v1.clone() {
                Adt5::C0(h692) => {
                    h692
                }
            });
            match m693 {
                None => {
                    let m697 = f154(f155(), match v0 {
                        Adt36::C0(h694, _, _, _, _, _, _, _) => {
                            h694
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h695) => {
                            h695
                        }
                    }), List::cons(Adt106::C0(Adt6::C5(match v1 {
                        Adt5::C0(h696) => {
                            h696
                        }
                    }), false, List::<Str>::nil()), List::<Adt106>::nil()), f156())?;
                    match m697 {
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

pub fn f198(v0: Str, v1: Adt6, v2: Adt106, v3: Adt121, v4: List<Adt106>) -> R<Result<(List<Adt106>, Adt121), Adt98>> {
    let m698 = v2;
    match m698 {
        Adt106::C0(v5, v6, v7) => {
            let m699 = f258(v5.clone());
            match m699 {
                None => {
                    let m700 = f259(v5);
                    match m700 {
                        None => {
                            Ok(Err::<(List<Adt106>, Adt121), Adt98>(Adt98::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f260(v0, v1, match v9 {
                                Adt5::C0(h701) => {
                                    h701
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

pub fn f199(v0: Adt6, p720: u64, p721: Adt124, p722: List<Adt125>) -> R<Adt122> {
    let m719 = (p720, (p721, p722));
    {
        let mut m719 = m719;
        loop {
            let step = {
                let (v1, (v2, v3)) = m719;
                let m702 = v1;
                if m702 == 0 {
                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C4))
                } else {
                    let v4 = m702 - 1;
                    let m703 = v3;
                    match m703.uncons() {
                        None => {
                            let m704 = v2;
                            match m704 {
                                Adt124::C0 => {
                                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C0))
                                }
                                Adt124::C1(v5, v6) => {
                                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m705 = v2.clone();
                            match m705 {
                                Adt124::C0 => {
                                    let m706 = v7.clone();
                                    match m706 {
                                        Adt125::C0(v9, v10, v11) => {
                                            let m707 = f261(v0.clone(), v9, v10, v11.clone())?;
                                            match m707 {
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
                                                        let a708 = v15;
                                                        let a709 = v8;
                                                        append_list(a708, a709)
                                                    };
                                                    Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt125::C1(_, _, _) => {
                                            Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (v2, v8)))
                                        }
                                        Adt125::C2(_, _, _, _) => {
                                            let v24: Adt137 = f262(v7, v8)?;
                                            Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (match v24.clone() {
                                                Adt137::C0(h710, _) => {
                                                    h710
                                                }
                                            }, match v24 {
                                                Adt137::C0(_, h711) => {
                                                    h711
                                                }
                                            })))
                                        }
                                        Adt125::C3(v25) => {
                                            Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (Adt124::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt125::C4(_, _, _, _) => {
                                            let v39: Adt137 = f262(v7, v8)?;
                                            Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (match v39.clone() {
                                                Adt137::C0(h715, _) => {
                                                    h715
                                                }
                                            }, match v39 {
                                                Adt137::C0(_, h716) => {
                                                    h716
                                                }
                                            })))
                                        }
                                        Adt125::C5(v26, v27, v28, v29) => {
                                            let m712 = f263(v26, v27, v28, v29.clone());
                                            match m712 {
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
                                                        let a713 = v33;
                                                        let a714 = v8;
                                                        append_list(a713, a714)
                                                    };
                                                    Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt124::C1(v40, v41) => {
                                    let v42: Adt137 = f264(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (match v42.clone() {
                                        Adt137::C0(h717, _) => {
                                            h717
                                        }
                                    }, match v42 {
                                        Adt137::C0(_, h718) => {
                                            h718
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m719 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200() -> u64 {
    67108864u64
}

pub fn f201(v0: List<Adt123>) -> R<Str> {
    let a723 = List::cons(Str::lit(""), f265(v0, List::<Str>::nil())?);
    let a724 = Str::lit("/");
    Ok(join(a723, a724))
}

pub fn f202(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f266({
        let m727 = match v1.clone() {
            Adt50::C0(h726, _) => {
                h726
            }
        };
        match m727 {
            Adt38::C0(h725, _, _) => {
                h725
            }
        }
    })? {
        f267(v0, v1)
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f203(v0: Bytes, p733: u64, p734: u64, p735: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt94>> {
    let m732 = (p733, (p734, p735));
    {
        let mut m732 = m732;
        loop {
            let step = {
                let (v1, (v2, v3)) = m732;
                let m728 = v2;
                if m728 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Ok::<(List<List<u64>>, u64), Adt94>((f37(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m728 - 1;
                    let m729 = f15(v0.clone(), v1)?;
                    match m729 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Err::<(List<List<u64>>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>((match v5.clone() {
                                Adt96::C0(_, h730) => {
                                    h730
                                }
                            }, (v4, List::cons(f86(match v5 {
                                Adt96::C0(h731, _) => {
                                    h731
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Adt92, v1: List<List<u64>>) -> Adt92 {
    Adt92::C0(match v0.clone() {
        Adt92::C0(h736, _, _, _, _, _, _, _) => {
            h736
        }
    }, match v0.clone() {
        Adt92::C0(_, h737, _, _, _, _, _, _) => {
            h737
        }
    }, match v0.clone() {
        Adt92::C0(_, _, h738, _, _, _, _, _) => {
            h738
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, h739, _, _, _, _) => {
            h739
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, _, h740, _, _, _) => {
            h740
        }
    }, v1, match v0.clone() {
        Adt92::C0(_, _, _, _, _, _, h741, _) => {
            h741
        }
    }, match v0 {
        Adt92::C0(_, _, _, _, _, _, _, h742) => {
            h742
        }
    })
}

pub fn f205(v0: Bytes, v1: List<u64>) -> Bytes {
    let a743 = v0;
    let a744 = f268(f88(), f269(List::<Bytes>::nil(), v1));
    append_bytes(a743, a744)
}

pub fn f206(v0: i64, v1: Str, v2: Str, v3: Adt96) -> Result<(Adt14, u64), Adt94> {
    Ok::<(Adt14, u64), Adt94>((Adt14::C0(v0, v1, v2, match v3.clone() {
        Adt96::C0(h745, _) => {
            h745
        }
    }), match v3 {
        Adt96::C0(_, h746) => {
            h746
        }
    }))
}

pub fn f207(v0: Adt107, v1: List<Adt107>, v2: List<u64>) -> R<Adt138> {
    let m747 = v0;
    match m747 {
        Adt107::C0(v3) => {
            Ok(Adt138::C1(v1, f168(v3, v2)))
        }
        Adt107::C1(v4, v5) => {
            f270(v4, v5, v1, v2)
        }
        Adt107::C2(v6, v7) => {
            let m748 = v7;
            match m748.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt138::C1(List::cons(Adt107::C1(v6, v8), List::cons(Adt107::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt107::C3(v10, v11) => {
            let m749 = v11.clone();
            match m749.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt107::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt107::C4(v14, v15) => {
            let m750 = v15;
            match m750.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt138::C1(List::cons(Adt107::C0({
                        let a752 = f271({
                            let (h751, _) = v16;
                            h751
                        })?;
                        let a753 = List::cons(58u64, List::<u64>::nil());
                        append_list(a752, a753)
                    }), List::cons(Adt107::C1(v14, {
                        let (_, h754) = v16;
                        h754
                    }), List::cons(Adt107::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt107::C5(v18, v19) => {
            let m755 = v19.clone();
            match m755.uncons() {
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

pub fn f208(v0: Adt128) -> R<Option<Adt5>> {
    let m757 = f272(match v0.clone() {
        Adt128::C0(h756, _, _) => {
            h756
        }
    })?;
    match m757 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a760 = v1;
                let a761 = {
                    let m759 = f273(v0);
                    match m759 {
                        Adt5::C0(h758) => {
                            h758
                        }
                    }
                };
                append_list(a760, a761)
            })))
        }
    }
}

pub fn f209(v0: Adt14, v1: Option<Adt88>, v2: Adt95) -> R<Adt95> {
    let m762 = v1;
    match m762 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a764 = match v2.clone() {
                Adt95::C0(h763, _, _, _, _) => {
                    h763
                }
            };
            let a765 = f212();
            if nat_eq(a764, a765) {
                Ok(f274(f275(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f210(v0: Adt33) -> Option<Adt88> {
    let m766 = v0;
    match m766 {
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
                Adt83::C0(_, _, h767) => {
                    h767
                }
            }, Adt89::C0, false))
        }
        Adt33::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f211(v0: bool, v1: Adt95, v2: Option<List<List<u64>>>) -> Adt95 {
    let m768 = v2;
    match m768 {
        None => {
            f6()
        }
        Some(v3) => {
            f276(v0, v1, f37(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f212() -> u64 {
    200u64
}

pub fn f213(v0: Adt99, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), List::cons((Str::lit("id"), f169(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h769) => {
            h769
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f214(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m770 = v0;
    match m770 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m771 = v1;
            match m771 {
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

pub fn f215(p775: List<Adt130>, p776: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m774 = (p775, p776);
    {
        let mut m774 = m774;
        loop {
            let step = {
                let (v0, v1) = m774;
                let m772 = v0;
                match m772.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m773 = f277(v2)?;
                        match m773 {
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
                Ok(next) => { m774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: Option<Adt88>) -> R<Option<List<List<u64>>>> {
    let m777 = v0;
    match m777 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f278(v1)
        }
    }
}

pub fn f217(v0: Adt131) -> u64 {
    let m778 = v0;
    match m778 {
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
            f279()
        }
        Adt131::C4(v6) => {
            f280(v6)
        }
    }
}

pub fn f218(v0: Adt131) -> R<Adt81> {
    Ok(Adt81::C0(f281(v0.clone()), f282(v0.clone()), f283(v0)?))
}

pub fn f219() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f220(p781: List<List<u64>>) -> R<bool> {
    let m780 = p781;
    {
        let mut m780 = m780;
        loop {
            let step = {
                let v0 = m780;
                let m779 = v0;
                match m779.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f284(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: u64, p786: List<u64>, p787: List<u64>, p788: List<List<u64>>) -> List<List<u64>> {
    let m785 = (p786, (p787, p788));
    {
        let mut m785 = m785;
        loop {
            let step = {
                let (v1, (v2, v3)) = m785;
                let m782 = v1;
                match m782.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f37(List::cons(f150(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a783 = v4;
                        let a784 = v0;
                        if nat_eq(a783, a784) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f150(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt139, Adt131>) -> R<Result<Adt128, Adt131>> {
    let m789 = v3;
    match m789 {
        Ok(v5) => {
            f285(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(v4))
        }
    }
}

pub fn f223(v0: Option<Adt108>, v1: List<Str>, v2: Adt105) -> R<Result<Adt139, Adt131>> {
    Ok(f286(f287(match v2.clone() {
        Adt105::C0(_, h790, _) => {
            h790
        }
    }), f288(v0, v1, v2)?))
}

pub fn f224(v0: Adt110, v1: Adt103) -> Adt133 {
    let m791 = v1;
    match m791 {
        Adt103::C0 => {
            f230()
        }
        Adt103::C1 => {
            Adt133::C0(None::<Str>, Some(Adt109::C6(v0)))
        }
        Adt103::C2 => {
            Adt133::C0(None::<Str>, Some(Adt109::C7(v0)))
        }
        Adt103::C3(v2) => {
            let m792 = f165(v2);
            match m792 {
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

pub fn f225() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f226() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f227(v0: Str) -> bool {
    let m793 = f289(v0);
    match m793 {
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

pub fn f228(v0: Adt103) -> R<Adt103> {
    let m794 = v0.clone();
    match m794 {
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
            let m795 = f290(v1);
            match m795 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m796 = f291(v2)?;
                    match m796 {
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

pub fn f229() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f230() -> Adt133 {
    Adt133::C0(None::<Str>, None::<Adt109>)
}

pub fn f231(v0: Option<Adt109>, v1: Option<Adt109>) -> Option<Adt109> {
    let m797 = v0.clone();
    match m797 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f232(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt111>> {
    Ok(f292(f293(v0, v1, v2, List::<Adt111>::nil())?, List::<Adt111>::nil()))
}

pub fn f233(v0: Adt6) -> Option<Str> {
    let m798 = v0;
    match m798 {
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

pub fn f234() -> Str {
    Str::lit("2026-07-28")
}

pub fn f235(v0: Adt7) -> R<Option<i64>> {
    let m801 = f294(40u64, match v0.clone() {
        Adt7::C0(h799, _) => {
            h799
        }
    }, match v0 {
        Adt7::C0(_, h800) => {
            h800
        }
    })?;
    match m801 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f295(v1))
        }
    }
}

pub fn f236(v0: Str, p807: List<(Str, Adt6)>) -> Adt135 {
    let m806 = p807;
    {
        let mut m806 = m806;
        loop {
            let step = {
                let v1 = m806;
                let m802 = v1;
                match m802.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt135>(Adt135::C0)
                    }
                    Some((v2, v3)) => {
                        let m804 = f296({
                            let (h803, _) = v2;
                            h803
                        }, v0.clone());
                        match m804 {
                            Adt147::C0 => {
                                Err::<List<(Str, Adt6)>, Adt135>(Adt135::C1({
                                    let (_, h805) = v2;
                                    h805
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
                Ok(next) => { m806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Str, v1: Adt5) -> bool {
    f297(f181(v0, v1))
}

pub fn f238(v0: Adt99, v1: Adt5) -> Result<Adt105, Adt101> {
    let m808 = f181(Str::lit("method"), v1.clone());
    match m808 {
        Adt135::C0 => {
            f136(Some(v0), f182(v1))
        }
        Adt135::C1(v2) => {
            let m809 = f233(v2);
            match m809 {
                None => {
                    f136(Some(v0), Adt102::C4)
                }
                Some(v3) => {
                    f298(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f239(v0: u64) -> R<u64> {
    let a812 = {
        let a810 = v0;
        let a811 = 3u64;
        nat_mul(a810, a811)?
    };
    let a813 = 1u64;
    nat_add(a812, a813)
}

pub fn f240(v0: u64, v1: u64, v2: u64) -> bool {
    let a814 = v1;
    let a815 = v0;
    if nat_le(a814, a815) {
        let a816 = v0;
        let a817 = v2;
        nat_le(a816, a817)
    } else {
        false
    }
}

pub fn f241(v0: Adt118) -> R<Adt141> {
    if f299(match v0.clone() {
        Adt118::C0(_, h818, _, _, _, _, _) => {
            h818
        }
    }) {
        let v1: i64 = {
            let a824 = if match v0.clone() {
                Adt118::C0(_, _, _, _, _, h819, _) => {
                    h819
                }
            } {
                let a821 = match v0.clone() {
                    Adt118::C0(_, _, _, _, _, _, h820) => {
                        h820
                    }
                };
                int_neg(a821)?
            } else {
                match v0.clone() {
                    Adt118::C0(_, _, _, _, _, _, h822) => {
                        h822
                    }
                }
            };
            let a825 = match v0.clone() {
                Adt118::C0(_, _, _, _, h823, _, _) => {
                    h823
                }
            };
            int_sub(a824, a825)?
        };
        let m828 = {
            let a826 = v1;
            let a827 = -9999i64;
            compare(a826, a827)
        };
        if if match m828 {
            Ordering::Less => {
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
            let m831 = {
                let a829 = 9999i64;
                let a830 = v1;
                compare(a829, a830)
            };
            match m831 {
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
                Adt118::C0(h832, _, _, _, _, _, _) => {
                    h832
                }
            } {
                let a834 = match v0 {
                    Adt118::C0(_, _, h833, _, _, _, _) => {
                        h833
                    }
                };
                int_neg(a834)?
            } else {
                match v0 {
                    Adt118::C0(_, _, h835, _, _, _, _) => {
                        h835
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt141::C0)
    }
}

pub fn f242(v0: Adt6, v1: List<Adt120>, v2: u64) -> Adt115 {
    let m836 = v1.clone();
    match m836.uncons() {
        None => {
            Adt115::C0(Adt116::C9(v0), List::<Adt120>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m837 = v3;
            match m837 {
                Adt120::C0(v5) => {
                    Adt115::C0(Adt116::C2, List::cons(Adt120::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt120::C1(v6, v7) => {
                    let m838 = v7;
                    match m838 {
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

pub fn f243(v0: u64) -> bool {
    let a839 = v0;
    let a840 = 32u64;
    if nat_eq(a839, a840) {
        true
    } else {
        let a841 = v0;
        let a842 = 10u64;
        if nat_eq(a841, a842) {
            true
        } else {
            let a843 = v0;
            let a844 = 13u64;
            if nat_eq(a843, a844) {
                true
            } else {
                let a845 = v0;
                let a846 = 9u64;
                nat_eq(a845, a846)
            }
        }
    }
}

pub fn f244(v0: Adt115, v1: u64, v2: u64) -> R<Adt136> {
    let a847 = v2;
    let a848 = 34u64;
    if nat_eq(a847, a848) {
        Ok(f248(v0, Adt116::C6(Adt117::C0, List::<u64>::nil(), false)))
    } else {
        let a849 = v2;
        let a850 = 123u64;
        if nat_eq(a849, a850) {
            f300(v0, Adt120::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt116::C3, v1)
        } else {
            let a851 = v2;
            let a852 = 91u64;
            if nat_eq(a851, a852) {
                f300(v0, Adt120::C0(List::<Adt6>::nil()), Adt116::C1, v1)
            } else {
                let a853 = v2;
                let a854 = 116u64;
                if nat_eq(a853, a854) {
                    Ok(f248(v0, Adt116::C7(0u64, 1u64)))
                } else {
                    let a855 = v2;
                    let a856 = 102u64;
                    if nat_eq(a855, a856) {
                        Ok(f248(v0, Adt116::C7(1u64, 1u64)))
                    } else {
                        let a857 = v2;
                        let a858 = 110u64;
                        if nat_eq(a857, a858) {
                            Ok(f248(v0, Adt116::C7(2u64, 1u64)))
                        } else {
                            let a859 = v2;
                            let a860 = 45u64;
                            if nat_eq(a859, a860) {
                                Ok(f248(v0, f301(true, Adt119::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a861 = v2;
                                let a862 = 48u64;
                                if nat_eq(a861, a862) {
                                    Ok(f248(v0, f301(false, Adt119::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f240(v2, 49u64, 57u64) {
                                        Ok(f248(v0, f301(false, Adt119::C2, f302({
                                            let a863 = v2;
                                            let a864 = 48u64;
                                            nat_sub(a863, a864)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f249(Adt100::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f245(v0: Adt115, v1: u64) -> Adt136 {
    let m866 = match v0.clone() {
        Adt115::C0(_, h865, _) => {
            h865
        }
    };
    match m866.uncons() {
        None => {
            f249(Adt100::C2(v1))
        }
        Some((v2, v3)) => {
            let m867 = v2;
            match m867 {
                Adt120::C0(v4) => {
                    Adt136::C1(f242(Adt6::C4(f303(v4, List::<Adt6>::nil())), v3, {
                        let a869 = match v0 {
                            Adt115::C0(_, _, h868) => {
                                h868
                            }
                        };
                        let a870 = 1u64;
                        nat_sub(a869, a870)
                    }))
                }
                Adt120::C1(_, _) => {
                    f249(Adt100::C2(v1))
                }
            }
        }
    }
}

pub fn f246(v0: Adt115, v1: u64, v2: u64) -> Adt136 {
    let m872 = match v0.clone() {
        Adt115::C0(h871, _, _) => {
            h871
        }
    };
    match m872 {
        Adt116::C0 => {
            f249(Adt100::C2(v1))
        }
        Adt116::C1 => {
            f249(Adt100::C2(v1))
        }
        Adt116::C2 => {
            if f243(v2) {
                Adt136::C1(v0)
            } else {
                let a873 = v2;
                let a874 = 44u64;
                if nat_eq(a873, a874) {
                    f304(v0, v1)
                } else {
                    let a875 = v2;
                    let a876 = 93u64;
                    if nat_eq(a875, a876) {
                        f245(v0, v1)
                    } else {
                        let a877 = v2;
                        let a878 = 125u64;
                        if nat_eq(a877, a878) {
                            f247(v0, v1)
                        } else {
                            f249(Adt100::C2(v1))
                        }
                    }
                }
            }
        }
        Adt116::C3 => {
            f249(Adt100::C2(v1))
        }
        Adt116::C4 => {
            f249(Adt100::C2(v1))
        }
        Adt116::C5 => {
            f249(Adt100::C2(v1))
        }
        Adt116::C6(_, _, _) => {
            f249(Adt100::C2(v1))
        }
        Adt116::C7(_, _) => {
            f249(Adt100::C2(v1))
        }
        Adt116::C8(_) => {
            f249(Adt100::C2(v1))
        }
        Adt116::C9(_) => {
            if f243(v2) {
                Adt136::C1(v0)
            } else {
                f249(Adt100::C10(v1))
            }
        }
    }
}

pub fn f247(v0: Adt115, v1: u64) -> Adt136 {
    let m880 = match v0.clone() {
        Adt115::C0(_, h879, _) => {
            h879
        }
    };
    match m880.uncons() {
        None => {
            f249(Adt100::C2(v1))
        }
        Some((v2, v3)) => {
            let m881 = v2;
            match m881 {
                Adt120::C0(_) => {
                    f249(Adt100::C2(v1))
                }
                Adt120::C1(v5, _) => {
                    Adt136::C1(f242(Adt6::C5(f305(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a883 = match v0 {
                            Adt115::C0(_, _, h882) => {
                                h882
                            }
                        };
                        let a884 = 1u64;
                        nat_sub(a883, a884)
                    }))
                }
            }
        }
    }
}

pub fn f248(v0: Adt115, v1: Adt116) -> Adt136 {
    Adt136::C1(f306(v0, v1))
}

pub fn f249(v0: Adt100) -> Adt136 {
    Adt136::C0(v0)
}

pub fn f250(v0: Adt115, v1: u64, v2: u64, v3: Adt117, v4: List<u64>, v5: bool) -> R<Adt136> {
    let m885 = v3;
    match m885 {
        Adt117::C0 => {
            let a886 = v2;
            let a887 = 34u64;
            if nat_eq(a886, a887) {
                Ok(f307(v0, v1, v4, v5))
            } else {
                let a888 = v2;
                let a889 = 92u64;
                if nat_eq(a888, a889) {
                    Ok(f248(v0, Adt116::C6(Adt117::C1, v4, v5)))
                } else {
                    let a890 = v2;
                    let a891 = 32u64;
                    if nat_lt(a890, a891) {
                        Ok(f249(Adt100::C4(v1)))
                    } else {
                        Ok(f248(v0, Adt116::C6(Adt117::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt117::C1 => {
            let a892 = v2;
            let a893 = 117u64;
            if nat_eq(a892, a893) {
                Ok(f248(v0, Adt116::C6(Adt117::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a894 = f308(v2);
                let a895 = 0u64;
                if nat_eq(a894, a895) {
                    Ok(f249(Adt100::C5(v1)))
                } else {
                    Ok(f248(v0, Adt116::C6(Adt117::C0, List::cons(f308(v2), v4), v5)))
                }
            }
        }
        Adt117::C2(v6, v7, v8) => {
            let a896 = 15u64;
            let a897 = f309(v2);
            if nat_lt(a896, a897) {
                Ok(f249(Adt100::C5(v1)))
            } else {
                let a900 = {
                    let a898 = v6;
                    let a899 = 1u64;
                    nat_add(a898, a899)?
                };
                let a901 = 4u64;
                if nat_eq(a900, a901) {
                    f310(v0, v1, {
                        let a904 = {
                            let a902 = v7;
                            let a903 = 16u64;
                            nat_mul(a902, a903)?
                        };
                        let a905 = f309(v2);
                        nat_add(a904, a905)?
                    }, v8, v4, v5)
                } else {
                    Ok(f248(v0, Adt116::C6(Adt117::C2({
                        let a906 = v6;
                        let a907 = 1u64;
                        nat_add(a906, a907)?
                    }, {
                        let a910 = {
                            let a908 = v7;
                            let a909 = 16u64;
                            nat_mul(a908, a909)?
                        };
                        let a911 = f309(v2);
                        nat_add(a910, a911)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt117::C3(v9) => {
            let a912 = v2;
            let a913 = 92u64;
            if nat_eq(a912, a913) {
                Ok(f248(v0, Adt116::C6(Adt117::C4(v9), v4, v5)))
            } else {
                Ok(f249(Adt100::C6(v1)))
            }
        }
        Adt117::C4(v10) => {
            let a914 = v2;
            let a915 = 117u64;
            if nat_eq(a914, a915) {
                Ok(f248(v0, Adt116::C6(Adt117::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f249(Adt100::C6(v1)))
            }
        }
    }
}

pub fn f251(v0: Adt115, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt136> {
    let a916 = v2;
    let a917 = f311(v3, v4);
    if nat_eq(a916, a917) {
        let a921 = {
            let a918 = v4;
            let a919 = 1u64;
            nat_add(a918, a919)?
        };
        let a922 = {
            let a920 = f312(v3);
            length_list(a920)
        };
        if nat_eq(a921, a922) {
            Ok(f313(v0, f314(v3)))
        } else {
            Ok(f248(v0, Adt116::C7(v3, {
                let a923 = v4;
                let a924 = 1u64;
                nat_add(a923, a924)?
            })))
        }
    } else {
        Ok(f249(Adt100::C2(v1)))
    }
}

pub fn f252(v0: Adt115, v1: u64, v2: u64, v3: Adt118) -> R<Adt136> {
    let m926 = match v3.clone() {
        Adt118::C0(_, h925, _, _, _, _, _) => {
            h925
        }
    };
    match m926 {
        Adt119::C0 => {
            let a927 = v2;
            let a928 = 48u64;
            if nat_eq(a927, a928) {
                Ok(f315(v0, v3, Adt119::C1))
            } else {
                if f240(v2, 49u64, 57u64) {
                    f316(v0, v1, v2, v3, false)
                } else {
                    Ok(f249(Adt100::C7(v1)))
                }
            }
        }
        Adt119::C1 => {
            let a929 = v2;
            let a930 = 46u64;
            if nat_eq(a929, a930) {
                Ok(f315(v0, v3, Adt119::C3))
            } else {
                if f317(v2) {
                    Ok(f315(v0, v3, Adt119::C5))
                } else {
                    if f318(v2) {
                        Ok(f249(Adt100::C7(v1)))
                    } else {
                        f319(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt119::C2 => {
            if f318(v2) {
                f316(v0, v1, v2, v3, false)
            } else {
                let a931 = v2;
                let a932 = 46u64;
                if nat_eq(a931, a932) {
                    Ok(f315(v0, v3, Adt119::C3))
                } else {
                    if f317(v2) {
                        Ok(f315(v0, v3, Adt119::C5))
                    } else {
                        f319(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt119::C3 => {
            if f318(v2) {
                f316(v0, v1, v2, v3, true)
            } else {
                Ok(f249(Adt100::C7(v1)))
            }
        }
        Adt119::C4 => {
            if f318(v2) {
                f316(v0, v1, v2, v3, true)
            } else {
                if f317(v2) {
                    Ok(f315(v0, v3, Adt119::C5))
                } else {
                    f319(v0, v1, v2, v3)
                }
            }
        }
        Adt119::C5 => {
            let a933 = v2;
            let a934 = 43u64;
            if nat_eq(a933, a934) {
                Ok(f248(v0, f301(match v3.clone() {
                    Adt118::C0(h935, _, _, _, _, _, _) => {
                        h935
                    }
                }, Adt119::C6, match v3.clone() {
                    Adt118::C0(_, _, h936, _, _, _, _) => {
                        h936
                    }
                }, match v3.clone() {
                    Adt118::C0(_, _, _, h937, _, _, _) => {
                        h937
                    }
                }, match v3 {
                    Adt118::C0(_, _, _, _, h938, _, _) => {
                        h938
                    }
                }, false, 0i64)))
            } else {
                let a939 = v2;
                let a940 = 45u64;
                if nat_eq(a939, a940) {
                    Ok(f248(v0, f301(match v3.clone() {
                        Adt118::C0(h941, _, _, _, _, _, _) => {
                            h941
                        }
                    }, Adt119::C6, match v3.clone() {
                        Adt118::C0(_, _, h942, _, _, _, _) => {
                            h942
                        }
                    }, match v3.clone() {
                        Adt118::C0(_, _, _, h943, _, _, _) => {
                            h943
                        }
                    }, match v3 {
                        Adt118::C0(_, _, _, _, h944, _, _) => {
                            h944
                        }
                    }, true, 0i64)))
                } else {
                    if f318(v2) {
                        f320(v0, v1, v2, Adt118::C0(match v3.clone() {
                            Adt118::C0(h945, _, _, _, _, _, _) => {
                                h945
                            }
                        }, match v3.clone() {
                            Adt118::C0(_, h946, _, _, _, _, _) => {
                                h946
                            }
                        }, match v3.clone() {
                            Adt118::C0(_, _, h947, _, _, _, _) => {
                                h947
                            }
                        }, match v3.clone() {
                            Adt118::C0(_, _, _, h948, _, _, _) => {
                                h948
                            }
                        }, match v3 {
                            Adt118::C0(_, _, _, _, h949, _, _) => {
                                h949
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f249(Adt100::C7(v1)))
                    }
                }
            }
        }
        Adt119::C6 => {
            if f318(v2) {
                f320(v0, v1, v2, v3.clone(), match v3 {
                    Adt118::C0(_, _, _, _, _, h950, _) => {
                        h950
                    }
                })
            } else {
                Ok(f249(Adt100::C7(v1)))
            }
        }
        Adt119::C7 => {
            if f318(v2) {
                f320(v0, v1, v2, v3.clone(), match v3 {
                    Adt118::C0(_, _, _, _, _, h951, _) => {
                        h951
                    }
                })
            } else {
                f319(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f253(v0: u64) -> bool {
    let a952 = v0;
    let a953 = 9u64;
    if nat_eq(a952, a953) {
        true
    } else {
        let a954 = v0;
        let a955 = 32u64;
        nat_eq(a954, a955)
    }
}

pub fn f254(v0: u64) -> bool {
    let a956 = 47u64;
    let a957 = v0;
    if nat_lt(a956, a957) {
        let a958 = v0;
        let a959 = 58u64;
        nat_lt(a958, a959)
    } else {
        false
    }
}

pub fn f255(v0: u64) -> bool {
    let a960 = 48u64;
    let a961 = v0;
    if if nat_le(a960, a961) {
        let a962 = v0;
        let a963 = 57u64;
        nat_le(a962, a963)
    } else {
        false
    } {
        true
    } else {
        let a964 = 65u64;
        let a965 = v0;
        if if nat_le(a964, a965) {
            let a966 = v0;
            let a967 = 90u64;
            nat_le(a966, a967)
        } else {
            false
        } {
            true
        } else {
            let a968 = 97u64;
            let a969 = v0;
            if if nat_le(a968, a969) {
                let a970 = v0;
                let a971 = 122u64;
                nat_le(a970, a971)
            } else {
                false
            } {
                true
            } else {
                let a972 = v0;
                let a973 = 33u64;
                if nat_eq(a972, a973) {
                    true
                } else {
                    let a974 = v0;
                    let a975 = 35u64;
                    if nat_eq(a974, a975) {
                        true
                    } else {
                        let a976 = v0;
                        let a977 = 36u64;
                        if nat_eq(a976, a977) {
                            true
                        } else {
                            let a978 = v0;
                            let a979 = 37u64;
                            if nat_eq(a978, a979) {
                                true
                            } else {
                                let a980 = v0;
                                let a981 = 38u64;
                                if nat_eq(a980, a981) {
                                    true
                                } else {
                                    let a982 = v0;
                                    let a983 = 39u64;
                                    if nat_eq(a982, a983) {
                                        true
                                    } else {
                                        let a984 = v0;
                                        let a985 = 42u64;
                                        if nat_eq(a984, a985) {
                                            true
                                        } else {
                                            let a986 = v0;
                                            let a987 = 43u64;
                                            if nat_eq(a986, a987) {
                                                true
                                            } else {
                                                let a988 = v0;
                                                let a989 = 45u64;
                                                if nat_eq(a988, a989) {
                                                    true
                                                } else {
                                                    let a990 = v0;
                                                    let a991 = 46u64;
                                                    if nat_eq(a990, a991) {
                                                        true
                                                    } else {
                                                        let a992 = v0;
                                                        let a993 = 94u64;
                                                        if nat_eq(a992, a993) {
                                                            true
                                                        } else {
                                                            let a994 = v0;
                                                            let a995 = 95u64;
                                                            if nat_eq(a994, a995) {
                                                                true
                                                            } else {
                                                                let a996 = v0;
                                                                let a997 = 96u64;
                                                                if nat_eq(a996, a997) {
                                                                    true
                                                                } else {
                                                                    let a998 = v0;
                                                                    let a999 = 124u64;
                                                                    if nat_eq(a998, a999) {
                                                                        true
                                                                    } else {
                                                                        let a1000 = v0;
                                                                        let a1001 = 126u64;
                                                                        nat_eq(a1000, a1001)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f256(v0: u64) -> bool {
    let a1002 = 48u64;
    let a1003 = v0;
    if if nat_le(a1002, a1003) {
        let a1004 = v0;
        let a1005 = 57u64;
        nat_le(a1004, a1005)
    } else {
        false
    } {
        true
    } else {
        let a1006 = 65u64;
        let a1007 = v0;
        if if nat_le(a1006, a1007) {
            let a1008 = v0;
            let a1009 = 90u64;
            nat_le(a1008, a1009)
        } else {
            false
        } {
            true
        } else {
            let a1010 = 97u64;
            let a1011 = v0;
            if if nat_le(a1010, a1011) {
                let a1012 = v0;
                let a1013 = 122u64;
                nat_le(a1012, a1013)
            } else {
                false
            } {
                true
            } else {
                let a1014 = v0;
                let a1015 = 95u64;
                if nat_eq(a1014, a1015) {
                    true
                } else {
                    let a1016 = v0;
                    let a1017 = 45u64;
                    if nat_eq(a1016, a1017) {
                        true
                    } else {
                        let a1018 = v0;
                        let a1019 = 46u64;
                        nat_eq(a1018, a1019)
                    }
                }
            }
        }
    }
}

pub fn f257() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f258(v0: Adt6) -> Option<bool> {
    let m1020 = v0;
    match m1020 {
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

pub fn f259(v0: Adt6) -> Option<Adt5> {
    let m1021 = v0;
    match m1021 {
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

pub fn f260(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt121, v6: List<Adt106>) -> R<Result<(List<Adt106>, Adt121), Adt98>> {
    let m1022 = f321(v0.clone(), v1, v2.clone());
    match m1022 {
        None => {
            let m1023 = f322(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1023 {
                Adt148::C0(v8) => {
                    Ok(Err::<(List<Adt106>, Adt121), Adt98>(v8))
                }
                Adt148::C1(v9) => {
                    Ok(Ok::<(List<Adt106>, Adt121), Adt98>((f323(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt106>, Adt121), Adt98>(v7))
        }
    }
}

pub fn f261(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt123>) -> R<Adt142> {
    let m1024 = f258(v1.clone());
    match m1024 {
        None => {
            let m1025 = f259(v1);
            match m1025 {
                None => {
                    Ok(Adt142::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f324(v0, match v5 {
                        Adt5::C0(h1026) => {
                            h1026
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

pub fn f262(v0: Adt125, v1: List<Adt125>) -> R<Adt137> {
    let m1027 = v0;
    match m1027 {
        Adt125::C0(_, _, _) => {
            Ok(Adt137::C0(Adt124::C0, v1))
        }
        Adt125::C1(_, _, _) => {
            Ok(Adt137::C0(Adt124::C0, v1))
        }
        Adt125::C2(v8, v9, v10, v11) => {
            Ok(f325(v8, v9, v10, {
                let a1028 = v11;
                let a1029 = 1u64;
                nat_add(a1028, a1029)?
            }, v1))
        }
        Adt125::C3(v12) => {
            Ok(Adt137::C0(Adt124::C1(v12, Str::lit("not")), v1))
        }
        Adt125::C4(v13, _, v15, v16) => {
            Ok(f326(v13, v15, v16, v1))
        }
        Adt125::C5(_, _, _, _) => {
            Ok(Adt137::C0(Adt124::C0, v1))
        }
    }
}

pub fn f263(v0: Adt126, v1: Adt6, v2: Adt6, v3: List<Adt123>) -> Adt142 {
    let m1030 = f259(v1);
    match m1030 {
        None => {
            Adt142::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m1031 = v0;
            match m1031 {
                Adt126::C0 => {
                    f327(match v4 {
                        Adt5::C0(h1032) => {
                            h1032
                        }
                    }, v2, v3)
                }
                Adt126::C1 => {
                    f328(match v4 {
                        Adt5::C0(h1033) => {
                            h1033
                        }
                    }, v2, v3)
                }
                Adt126::C2 => {
                    f329(match v4 {
                        Adt5::C0(h1034) => {
                            h1034
                        }
                    }, v2, v3)
                }
                Adt126::C3 => {
                    f330(match v4 {
                        Adt5::C0(h1035) => {
                            h1035
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f264(v0: Adt125, v1: List<Adt125>, v2: List<Adt123>, v3: Str) -> Adt137 {
    let m1036 = v0;
    match m1036 {
        Adt125::C0(_, _, _) => {
            Adt137::C0(Adt124::C1(v2, v3), v1)
        }
        Adt125::C1(v7, v8, v9) => {
            f331(v7, v8, v9, v1)
        }
        Adt125::C2(v10, v11, v12, v13) => {
            f325(v10, v11, v12, v13, v1)
        }
        Adt125::C3(_) => {
            Adt137::C0(Adt124::C0, v1)
        }
        Adt125::C4(_, v16, v17, v18) => {
            f326(v16, v17, v18, v1)
        }
        Adt125::C5(_, _, _, _) => {
            Adt137::C0(Adt124::C1(v2, v3), v1)
        }
    }
}

pub fn f265(p1039: List<Adt123>, p1040: List<Str>) -> R<List<Str>> {
    let m1038 = (p1039, p1040);
    {
        let mut m1038 = m1038;
        loop {
            let step = {
                let (v0, v1) = m1038;
                let m1037 = v0;
                match m1037.uncons() {
                    None => {
                        Err::<(List<Adt123>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt123>, List<Str>), R<List<Str>>>((v3, List::cons(f332(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1038 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(p1049: List<Adt39>) -> R<bool> {
    let m1048 = p1049;
    {
        let mut m1048 = m1048;
        loop {
            let step = {
                let v0 = m1048;
                let m1041 = v0;
                match m1041.uncons() {
                    None => {
                        Err::<List<Adt39>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m1042 = v1;
                        match m1042 {
                            Adt39::C0(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C1(v4) => {
                                if f333(match v4 {
                                    Adt43::C0(h1043, _, _, _) => {
                                        h1043
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C2(v5) => {
                                if f333(match v5 {
                                    Adt44::C0(h1044, _, _, _) => {
                                        h1044
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
                                let m1046 = match v7 {
                                    Adt46::C0(h1045, _, _) => {
                                        h1045
                                    }
                                };
                                match m1046 {
                                    Adt47::C0(_) => {
                                        Ok::<List<Adt39>, R<bool>>(v2)
                                    }
                                    Adt47::C1(v9) => {
                                        if f333(match v9 {
                                            Adt49::C0(_, _, h1047, _) => {
                                                h1047
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
                Ok(next) => { m1048 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f334(match v1.clone() {
        Adt50::C0(h1050, _) => {
            h1050
        }
    }) {
        f335(v1)
    } else {
        let m1052 = match v0 {
            Adt36::C0(_, _, _, _, _, h1051, _, _) => {
                h1051
            }
        };
        match m1052 {
            None => {
                f335(v1)
            }
            Some(v2) => {
                f336(v2, v1)
            }
        }
    }
}

pub fn f268(p1057: Bytes, p1058: List<Bytes>) -> Bytes {
    let m1056 = (p1057, p1058);
    {
        let mut m1056 = m1056;
        loop {
            let step = {
                let (v0, v1) = m1056;
                let m1053 = v1;
                match m1053.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1054 = v2;
                            let a1055 = v0;
                            append_bytes(a1054, a1055)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f269(p1061: List<Bytes>, p1062: List<u64>) -> List<Bytes> {
    let m1060 = (p1061, p1062);
    {
        let mut m1060 = m1060;
        loop {
            let step = {
                let (v0, v1) = m1060;
                let m1059 = v1;
                match m1059.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f337(f338(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: u64, v1: Adt6, v2: List<Adt107>, v3: List<u64>) -> R<Adt138> {
    let m1063 = v1;
    match m1063 {
        Adt6::C0 => {
            Ok(Adt138::C1(v2, f168(f339(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt138::C1(v2, f168(if v4 {
                f339(0u64)
            } else {
                f339(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f340(v5.clone())? {
                Ok(Adt138::C1(v2, f168(f341(v5)?, v3)))
            } else {
                Ok(Adt138::C0(Adt100::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt138::C1(v2, f168(f271(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1064 = v0;
            let a1065 = f342();
            if nat_lt(a1064, a1065) {
                Ok(Adt138::C1(f343(91u64, Adt107::C2({
                    let a1066 = v0;
                    let a1067 = 1u64;
                    nat_add(a1066, a1067)?
                }, v7), v2), v3))
            } else {
                Ok(Adt138::C0(Adt100::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1068 = v0;
            let a1069 = f342();
            if nat_lt(a1068, a1069) {
                Ok(Adt138::C1(f343(123u64, Adt107::C4({
                    let a1070 = v0;
                    let a1071 = 1u64;
                    nat_add(a1070, a1071)?
                }, v8), v2), v3))
            } else {
                Ok(Adt138::C0(Adt100::C12))
            }
        }
    }
}

pub fn f271(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1073 = f168(f344(f86({
            let a1072 = v0;
            utf8_encode(a1072)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1074 = List::cons(34u64, List::<u64>::nil());
        append_list(a1073, a1074)
    }))
}

pub fn f272(v0: Adt33) -> R<Option<List<(Str, Adt6)>>> {
    let m1075 = v0;
    match m1075 {
        Adt33::C0(v1) => {
            Ok(Some(f345(v1)?))
        }
        Adt33::C1(v2) => {
            Ok(Some(f346(v2)?))
        }
        Adt33::C2(v3) => {
            f347(f348(v3)?)
        }
        Adt33::C3(v4) => {
            f347(f348(match v4 {
                Adt50::C0(h1076, _) => {
                    h1076
                }
            })?)
        }
        Adt33::C4(v5) => {
            Ok(Some(f349(v5)?))
        }
        Adt33::C5(v6) => {
            Ok(Some(f350(v6)?))
        }
        Adt33::C6(v7) => {
            Ok(Some(f351(v7)?))
        }
        Adt33::C7(v8) => {
            Ok(Some(f352(v8)?))
        }
        Adt33::C8(v9) => {
            f347(f353(v9)?)
        }
        Adt33::C9(v10) => {
            Ok(Some(f354(v10)?))
        }
        Adt33::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt33::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt33::C12(v12) => {
            f347(f355(v12)?)
        }
        Adt33::C13(v13) => {
            f347(f356({
                let m1079 = match v13 {
                    Adt83::C0(_, _, h1078) => {
                        h1078
                    }
                };
                match m1079 {
                    Adt85::C0(h1077, _, _) => {
                        h1077
                    }
                }
            }))
        }
        Adt33::C14(v14) => {
            Ok(f357(v14))
        }
    }
}

pub fn f273(v0: Adt128) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f358(f359(match v0.clone() {
        Adt128::C0(h1080, _, _) => {
            h1080
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f360({
        let m1083 = match v0 {
            Adt128::C0(_, h1082, _) => {
                h1082
            }
        };
        match m1083 {
            Adt129::C0(h1081, _) => {
                h1081
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f274(v0: Option<Option<List<u64>>>, v1: Adt95) -> Adt95 {
    let m1084 = v0;
    match m1084 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt95::C0(match v1.clone() {
                Adt95::C0(h1085, _, _, _, _) => {
                    h1085
                }
            }, match v1.clone() {
                Adt95::C0(_, h1086, _, _, _) => {
                    h1086
                }
            }, match v1.clone() {
                Adt95::C0(_, _, h1087, _, _) => {
                    h1087
                }
            }, match v1 {
                Adt95::C0(_, _, _, h1088, _) => {
                    h1088
                }
            }, v2)
        }
    }
}

pub fn f275(v0: Adt14, v1: Adt88) -> R<Option<Option<List<u64>>>> {
    let m1090 = f361(match v0.clone() {
        Adt14::C0(_, _, _, h1089) => {
            h1089
        }
    })?;
    match m1090 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1093 = f362(f363(match v1 {
                Adt88::C0(h1091, _, _) => {
                    h1091
                }
            }, v2, match v0 {
                Adt14::C0(h1092, _, _, _) => {
                    h1092
                }
            })?)?;
            match m1093 {
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

pub fn f276(v0: bool, v1: Adt95, v2: List<List<u64>>) -> Adt95 {
    if f364(v0, match v1.clone() {
        Adt95::C0(h1094, _, _, _, _) => {
            h1094
        }
    }, v2.clone()) {
        f365(match v1.clone() {
            Adt95::C0(h1095, _, _, _, _) => {
                h1095
            }
        }, {
            let a1097 = v2;
            let a1098 = List::cons(match v1 {
                Adt95::C0(_, _, _, h1096, _) => {
                    h1096
                }
            }, List::<List<u64>>::nil());
            append_list(a1097, a1098)
        })
    } else {
        v1
    }
}

pub fn f277(v0: Adt130) -> R<Option<List<u64>>> {
    let m1099 = f366(f367(v0))?;
    match m1099 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1100 = f90(v1)?;
            match m1100 {
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

pub fn f278(v0: Adt88) -> R<Option<List<List<u64>>>> {
    if f368(v0.clone()) {
        let m1104 = f369({
            let m1103 = match v0 {
                Adt88::C0(h1102, _, _) => {
                    h1102
                }
            };
            match m1103 {
                Adt85::C0(h1101, _, _) => {
                    h1101
                }
            }
        });
        match m1104 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1105 = f366(v1)?;
                match m1105 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1106 = f90(v2)?;
                        match m1106 {
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

pub fn f279() -> u64 {
    404u64
}

pub fn f280(v0: Adt90) -> u64 {
    let m1107 = v0;
    match m1107 {
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
            f370()
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

pub fn f281(v0: Adt131) -> i64 {
    let m1108 = v0;
    match m1108 {
        Adt131::C0(_) => {
            f371()
        }
        Adt131::C1(_, _) => {
            f372()
        }
        Adt131::C2(_) => {
            f373()
        }
        Adt131::C3(_) => {
            f374()
        }
        Adt131::C4(v6) => {
            f375(v6)
        }
    }
}

pub fn f282(v0: Adt131) -> Str {
    let m1109 = v0;
    match m1109 {
        Adt131::C0(v1) => {
            f376(v1)
        }
        Adt131::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt131::C2(v4) => {
            f377(v4)
        }
        Adt131::C3(_) => {
            Str::lit("Method not found")
        }
        Adt131::C4(v6) => {
            let m1110 = v6;
            match m1110 {
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

pub fn f283(v0: Adt131) -> R<Option<Adt6>> {
    let m1111 = v0;
    match m1111 {
        Adt131::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f378(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt131::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C4(v6) => {
            let m1112 = v6;
            match m1112 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f379(v15)), List::<(Str, Adt6)>::nil()))))
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

pub fn f284(v0: List<u64>) -> R<bool> {
    let m1113 = f221(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1113.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f380(v2)? {
                Ok(false)
            } else {
                f140(f142(v1), f381())
            }
        }
    }
}

pub fn f285(v0: Adt0, v1: Fn2, v2: u64, v3: Adt139) -> R<Result<Adt128, Adt131>> {
    f382(v0.clone(), v1, v2, v3.clone(), f383(match v0 {
        Adt0::C0(_, _, h1114, _, _, _, _, _) => {
            h1114
        }
    }, match v3 {
        Adt139::C0(_, h1115, _, _) => {
            h1115
        }
    }))
}

pub fn f286(v0: Adt143, v1: Adt144) -> Result<Adt139, Adt131> {
    let m1116 = v1;
    match m1116 {
        Adt144::C0(v3) => {
            f384(v3, v0)
        }
        Adt144::C1(v2) => {
            Err::<Adt139, Adt131>(v2)
        }
    }
}

pub fn f287(v0: Str) -> Adt143 {
    let a1117 = v0.clone();
    let a1118 = Str::lit("server/discover");
    if equal(a1117, a1118) {
        Adt143::C1(Adt140::C0)
    } else {
        let a1119 = v0.clone();
        let a1120 = Str::lit("tools/list");
        if equal(a1119, a1120) {
            Adt143::C1(Adt140::C1)
        } else {
            let a1121 = v0.clone();
            let a1122 = Str::lit("tools/call");
            if equal(a1121, a1122) {
                Adt143::C1(Adt140::C2)
            } else {
                let a1123 = v0.clone();
                let a1124 = Str::lit("resources/list");
                if equal(a1123, a1124) {
                    Adt143::C1(Adt140::C3)
                } else {
                    let a1125 = v0.clone();
                    let a1126 = Str::lit("resources/read");
                    if equal(a1125, a1126) {
                        Adt143::C1(Adt140::C4)
                    } else {
                        let a1127 = v0.clone();
                        let a1128 = Str::lit("resources/templates/list");
                        if equal(a1127, a1128) {
                            Adt143::C1(Adt140::C5)
                        } else {
                            let a1129 = v0.clone();
                            let a1130 = Str::lit("prompts/list");
                            if equal(a1129, a1130) {
                                Adt143::C1(Adt140::C6)
                            } else {
                                let a1131 = v0.clone();
                                let a1132 = Str::lit("prompts/get");
                                if equal(a1131, a1132) {
                                    Adt143::C1(Adt140::C7)
                                } else {
                                    let a1133 = v0.clone();
                                    let a1134 = Str::lit("completion/complete");
                                    if equal(a1133, a1134) {
                                        Adt143::C1(Adt140::C8)
                                    } else {
                                        let a1135 = v0.clone();
                                        let a1136 = Str::lit("tasks/get");
                                        if equal(a1135, a1136) {
                                            Adt143::C1(Adt140::C9)
                                        } else {
                                            let a1137 = v0.clone();
                                            let a1138 = Str::lit("tasks/update");
                                            if equal(a1137, a1138) {
                                                Adt143::C1(Adt140::C10)
                                            } else {
                                                let a1139 = v0;
                                                let a1140 = Str::lit("tasks/cancel");
                                                if equal(a1139, a1140) {
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

pub fn f288(v0: Option<Adt108>, v1: List<Str>, v2: Adt105) -> R<Adt144> {
    f385(v0, v1, v2.clone(), f386(match v2 {
        Adt105::C0(_, _, h1141) => {
            h1141
        }
    }))
}

pub fn f289(v0: Str) -> Adt146 {
    let m1142 = f287(v0);
    match m1142 {
        Adt143::C0 => {
            Adt146::C0
        }
        Adt143::C1(v1) => {
            f387(v1)
        }
    }
}

pub fn f290(v0: List<u64>) -> Option<List<u64>> {
    let m1143 = f388(f389(), v0);
    match m1143 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1144 = f388(f390(), f150(v1, List::<u64>::nil()));
            match m1144 {
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

pub fn f291(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1146 = f391({
        let a1145 = v0.clone();
        length_list(a1145)
    }, v0, List::<u64>::nil())?;
    match m1146 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f168(v1, List::<u64>::nil())))
        }
    }
}

pub fn f292(p1149: List<Adt111>, p1150: List<Adt111>) -> List<Adt111> {
    let m1148 = (p1149, p1150);
    {
        let mut m1148 = m1148;
        loop {
            let step = {
                let (v0, v1) = m1148;
                let m1147 = v0;
                match m1147.uncons() {
                    None => {
                        Err::<(List<Adt111>, List<Adt111>), List<Adt111>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt111>, List<Adt111>), List<Adt111>>((v3, List::cons(v2, v1)))
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

pub fn f293(v0: List<Adt93>, p1157: List<Adt12>, v2: Option<Str>, p1158: List<Adt111>) -> R<List<Adt111>> {
    let m1156 = (p1157, p1158);
    {
        let mut m1156 = m1156;
        loop {
            let step = {
                let (v1, v3) = m1156;
                let m1151 = v1;
                match m1151.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt111>), R<List<Adt111>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f392(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt111>), R<List<Adt111>>>((v5, List::cons(Adt111::C0(match v4.clone() {
                                Adt12::C0(_, h1152, _, _) => {
                                    h1152
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1153, _) => {
                                    h1153
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1154) => {
                                    h1154
                                }
                            }, f393(v0.clone(), match v4 {
                                Adt12::C0(_, h1155, _, _) => {
                                    h1155
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt111>), R<List<Adt111>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(p1174: u64, p1175: i64, p1176: i64) -> R<Option<i64>> {
    let m1173 = (p1174, (p1175, p1176));
    {
        let mut m1173 = m1173;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1173;
                let m1159 = v0;
                if m1159 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1159 - 1;
                    if f394(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1162 = {
                            let a1160 = v2;
                            let a1161 = 0i64;
                            compare(a1160, a1161)
                        };
                        if match m1162 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f394({
                                let a1163 = v1;
                                let a1164 = 10i64;
                                let a1165 = 0i64;
                                int_rem(a1163, a1164, a1165)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1166 = v1;
                                    let a1167 = 10i64;
                                    let a1168 = 0i64;
                                    int_quot(a1166, a1167, a1168)?
                                }, f116(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1169 = v1;
                                let a1170 = 10i64;
                                int_mul(a1169, a1170)?
                            }, {
                                let a1171 = v2;
                                let a1172 = 1i64;
                                int_sub(a1171, a1172)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: i64) -> Option<i64> {
    let m1179 = {
        let a1177 = 9007199254740991i64;
        let a1178 = v0;
        compare(a1177, a1178)
    };
    if if match m1179 {
        Ordering::Less => {
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
        let m1182 = {
            let a1180 = v0;
            let a1181 = -9007199254740991i64;
            compare(a1180, a1181)
        };
        match m1182 {
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

pub fn f296(v0: Str, v1: Str) -> Adt147 {
    let a1183 = v0;
    let a1184 = v1;
    if equal(a1183, a1184) {
        Adt147::C0
    } else {
        Adt147::C1
    }
}

pub fn f297(v0: Adt135) -> bool {
    let m1185 = v0;
    match m1185 {
        Adt135::C0 => {
            false
        }
        Adt135::C1(_) => {
            true
        }
    }
}

pub fn f298(v0: Adt99, v1: Str, v2: Adt5) -> Result<Adt105, Adt101> {
    let m1186 = f181(Str::lit("params"), v2);
    match m1186 {
        Adt135::C0 => {
            Ok::<Adt105, Adt101>(Adt105::C0(v0, v1, None::<Adt5>))
        }
        Adt135::C1(v3) => {
            let m1187 = f259(v3);
            match m1187 {
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

pub fn f299(v0: Adt119) -> bool {
    let m1188 = v0;
    match m1188 {
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

pub fn f300(v0: Adt115, v1: Adt120, v2: Adt116, v3: u64) -> R<Adt136> {
    let a1190 = match v0.clone() {
        Adt115::C0(_, _, h1189) => {
            h1189
        }
    };
    let a1191 = f342();
    if nat_lt(a1190, a1191) {
        Ok(Adt136::C1(Adt115::C0(v2, List::cons(v1, match v0.clone() {
            Adt115::C0(_, h1192, _) => {
                h1192
            }
        }), {
            let a1194 = match v0 {
                Adt115::C0(_, _, h1193) => {
                    h1193
                }
            };
            let a1195 = 1u64;
            nat_add(a1194, a1195)?
        })))
    } else {
        Ok(f249(Adt100::C9(v3)))
    }
}

pub fn f301(v0: bool, v1: Adt119, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt116 {
    Adt116::C8(Adt118::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f302(v0: u64) -> i64 {
    let a1196 = v0;
    let a1197 = 0u64;
    if nat_eq(a1196, a1197) {
        0i64
    } else {
        let a1198 = v0;
        let a1199 = 1u64;
        if nat_eq(a1198, a1199) {
            1i64
        } else {
            let a1200 = v0;
            let a1201 = 2u64;
            if nat_eq(a1200, a1201) {
                2i64
            } else {
                let a1202 = v0;
                let a1203 = 3u64;
                if nat_eq(a1202, a1203) {
                    3i64
                } else {
                    let a1204 = v0;
                    let a1205 = 4u64;
                    if nat_eq(a1204, a1205) {
                        4i64
                    } else {
                        let a1206 = v0;
                        let a1207 = 5u64;
                        if nat_eq(a1206, a1207) {
                            5i64
                        } else {
                            let a1208 = v0;
                            let a1209 = 6u64;
                            if nat_eq(a1208, a1209) {
                                6i64
                            } else {
                                let a1210 = v0;
                                let a1211 = 7u64;
                                if nat_eq(a1210, a1211) {
                                    7i64
                                } else {
                                    let a1212 = v0;
                                    let a1213 = 8u64;
                                    if nat_eq(a1212, a1213) {
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

pub fn f303(p1216: List<Adt6>, p1217: List<Adt6>) -> List<Adt6> {
    let m1215 = (p1216, p1217);
    {
        let mut m1215 = m1215;
        loop {
            let step = {
                let (v0, v1) = m1215;
                let m1214 = v0;
                match m1214.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: Adt115, v1: u64) -> Adt136 {
    let m1219 = match v0.clone() {
        Adt115::C0(_, h1218, _) => {
            h1218
        }
    };
    match m1219.uncons() {
        None => {
            f249(Adt100::C2(v1))
        }
        Some((v2, _)) => {
            let m1220 = v2;
            match m1220 {
                Adt120::C0(_) => {
                    f248(v0, Adt116::C0)
                }
                Adt120::C1(_, _) => {
                    f248(v0, Adt116::C4)
                }
            }
        }
    }
}

pub fn f305(p1223: List<(Str, Adt6)>, p1224: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1222 = (p1223, p1224);
    {
        let mut m1222 = m1222;
        loop {
            let step = {
                let (v0, v1) = m1222;
                let m1221 = v0;
                match m1221.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1222 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Adt115, v1: Adt116) -> Adt115 {
    Adt115::C0(v1, match v0.clone() {
        Adt115::C0(_, h1225, _) => {
            h1225
        }
    }, match v0 {
        Adt115::C0(_, _, h1226) => {
            h1226
        }
    })
}

pub fn f307(v0: Adt115, v1: u64, v2: List<u64>, v3: bool) -> Adt136 {
    let m1228 = {
        let a1227 = f205(f88(), f168(v2, List::<u64>::nil()));
        utf8_decode(a1227)
    };
    match m1228 {
        None => {
            f249(Adt100::C14)
        }
        Some(v4) => {
            if v3 {
                f395(v0, v1, v4)
            } else {
                f313(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f308(v0: u64) -> u64 {
    let a1229 = v0;
    let a1230 = 34u64;
    if nat_eq(a1229, a1230) {
        34u64
    } else {
        let a1231 = v0;
        let a1232 = 92u64;
        if nat_eq(a1231, a1232) {
            92u64
        } else {
            let a1233 = v0;
            let a1234 = 47u64;
            if nat_eq(a1233, a1234) {
                47u64
            } else {
                let a1235 = v0;
                let a1236 = 98u64;
                if nat_eq(a1235, a1236) {
                    8u64
                } else {
                    let a1237 = v0;
                    let a1238 = 102u64;
                    if nat_eq(a1237, a1238) {
                        12u64
                    } else {
                        let a1239 = v0;
                        let a1240 = 110u64;
                        if nat_eq(a1239, a1240) {
                            10u64
                        } else {
                            let a1241 = v0;
                            let a1242 = 114u64;
                            if nat_eq(a1241, a1242) {
                                13u64
                            } else {
                                let a1243 = v0;
                                let a1244 = 116u64;
                                if nat_eq(a1243, a1244) {
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

pub fn f309(v0: u64) -> u64 {
    let a1245 = v0;
    let a1246 = 48u64;
    if nat_lt(a1245, a1246) {
        16u64
    } else {
        let a1247 = v0;
        let a1248 = 58u64;
        if nat_lt(a1247, a1248) {
            let a1249 = v0;
            let a1250 = 48u64;
            nat_sub(a1249, a1250)
        } else {
            let a1251 = v0;
            let a1252 = 65u64;
            if nat_lt(a1251, a1252) {
                16u64
            } else {
                let a1253 = v0;
                let a1254 = 71u64;
                if nat_lt(a1253, a1254) {
                    let a1255 = v0;
                    let a1256 = 55u64;
                    nat_sub(a1255, a1256)
                } else {
                    let a1257 = v0;
                    let a1258 = 97u64;
                    if nat_lt(a1257, a1258) {
                        16u64
                    } else {
                        let a1259 = v0;
                        let a1260 = 103u64;
                        if nat_lt(a1259, a1260) {
                            let a1261 = v0;
                            let a1262 = 87u64;
                            nat_sub(a1261, a1262)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f310(v0: Adt115, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt136> {
    let a1263 = v3;
    let a1264 = 0u64;
    if nat_eq(a1263, a1264) {
        if f240(v2, 55296u64, 56319u64) {
            Ok(f248(v0, Adt116::C6(Adt117::C3(v2), v4, v5)))
        } else {
            if f240(v2, 56320u64, 57343u64) {
                Ok(f249(Adt100::C6(v1)))
            } else {
                Ok(f248(v0, Adt116::C6(Adt117::C0, f396(v2, v4)?, v5)))
            }
        }
    } else {
        if f240(v2, 56320u64, 57343u64) {
            Ok(f248(v0, Adt116::C6(Adt117::C0, f396({
                let a1273 = 65536u64;
                let a1274 = {
                    let a1271 = {
                        let a1267 = {
                            let a1265 = v3;
                            let a1266 = 55296u64;
                            nat_sub(a1265, a1266)
                        };
                        let a1268 = 1024u64;
                        nat_mul(a1267, a1268)?
                    };
                    let a1272 = {
                        let a1269 = v2;
                        let a1270 = 56320u64;
                        nat_sub(a1269, a1270)
                    };
                    nat_add(a1271, a1272)?
                };
                nat_add(a1273, a1274)?
            }, v4)?, v5)))
        } else {
            Ok(f249(Adt100::C6(v1)))
        }
    }
}

pub fn f311(v0: u64, v1: u64) -> u64 {
    let m1277 = {
        let a1275 = f312(v0);
        let a1276 = v1;
        index_list(a1275, a1276)
    };
    match m1277 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f312(v0: u64) -> List<u64> {
    let a1278 = v0;
    let a1279 = 0u64;
    if nat_eq(a1278, a1279) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1280 = v0;
        let a1281 = 1u64;
        if nat_eq(a1280, a1281) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f313(v0: Adt115, v1: Adt6) -> Adt136 {
    Adt136::C1(f242(v1, match v0.clone() {
        Adt115::C0(_, h1282, _) => {
            h1282
        }
    }, match v0 {
        Adt115::C0(_, _, h1283) => {
            h1283
        }
    }))
}

pub fn f314(v0: u64) -> Adt6 {
    let a1284 = v0;
    let a1285 = 0u64;
    if nat_eq(a1284, a1285) {
        Adt6::C1(true)
    } else {
        let a1286 = v0;
        let a1287 = 1u64;
        if nat_eq(a1286, a1287) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f315(v0: Adt115, v1: Adt118, v2: Adt119) -> Adt136 {
    f248(v0, Adt116::C8(Adt118::C0(match v1.clone() {
        Adt118::C0(h1288, _, _, _, _, _, _) => {
            h1288
        }
    }, v2, match v1.clone() {
        Adt118::C0(_, _, h1289, _, _, _, _) => {
            h1289
        }
    }, match v1.clone() {
        Adt118::C0(_, _, _, h1290, _, _, _) => {
            h1290
        }
    }, match v1.clone() {
        Adt118::C0(_, _, _, _, h1291, _, _) => {
            h1291
        }
    }, match v1.clone() {
        Adt118::C0(_, _, _, _, _, h1292, _) => {
            h1292
        }
    }, match v1 {
        Adt118::C0(_, _, _, _, _, _, h1293) => {
            h1293
        }
    })))
}

pub fn f316(v0: Adt115, v1: u64, v2: u64, v3: Adt118, v4: bool) -> R<Adt136> {
    let v5: u64 = {
        let a1295 = match v3.clone() {
            Adt118::C0(_, _, _, h1294, _, _, _) => {
                h1294
            }
        };
        let a1296 = 0u64;
        if if nat_eq(a1295, a1296) {
            let a1297 = v2;
            let a1298 = 48u64;
            nat_eq(a1297, a1298)
        } else {
            false
        } {
            0u64
        } else {
            let a1300 = match v3.clone() {
                Adt118::C0(_, _, _, h1299, _, _, _) => {
                    h1299
                }
            };
            let a1301 = 1u64;
            nat_add(a1300, a1301)?
        }
    };
    let v6: i64 = if v4 {
        f116(match v3.clone() {
            Adt118::C0(_, _, _, _, h1302, _, _) => {
                h1302
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt118::C0(_, _, _, _, h1303, _, _) => {
                h1303
            }
        }
    };
    let a1304 = f397();
    let a1305 = v5;
    if nat_lt(a1304, a1305) {
        Ok(f249(Adt100::C8(v1)))
    } else {
        let m1308 = {
            let a1306 = 99999i64;
            let a1307 = v6;
            compare(a1306, a1307)
        };
        if match m1308 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f249(Adt100::C8(v1)))
        } else {
            Ok(f248(v0, f301(match v3.clone() {
                Adt118::C0(h1309, _, _, _, _, _, _) => {
                    h1309
                }
            }, if v4 {
                Adt119::C4
            } else {
                Adt119::C2
            }, f116({
                let a1311 = match v3.clone() {
                    Adt118::C0(_, _, h1310, _, _, _, _) => {
                        h1310
                    }
                };
                let a1312 = 10i64;
                int_mul(a1311, a1312)?
            }, f302({
                let a1313 = v2;
                let a1314 = 48u64;
                nat_sub(a1313, a1314)
            }))?, v5, v6, match v3.clone() {
                Adt118::C0(_, _, _, _, _, h1315, _) => {
                    h1315
                }
            }, match v3 {
                Adt118::C0(_, _, _, _, _, _, h1316) => {
                    h1316
                }
            })))
        }
    }
}

pub fn f317(v0: u64) -> bool {
    let a1317 = v0;
    let a1318 = 101u64;
    if nat_eq(a1317, a1318) {
        true
    } else {
        let a1319 = v0;
        let a1320 = 69u64;
        nat_eq(a1319, a1320)
    }
}

pub fn f318(v0: u64) -> bool {
    f240(v0, 48u64, 57u64)
}

pub fn f319(v0: Adt115, v1: u64, v2: u64, v3: Adt118) -> R<Adt136> {
    let m1321 = f241(v3)?;
    match m1321 {
        Adt141::C0 => {
            Ok(f249(Adt100::C7(v1)))
        }
        Adt141::C1 => {
            Ok(f249(Adt100::C8(v1)))
        }
        Adt141::C2(v4) => {
            Ok(f246(f242(v4, match v0.clone() {
                Adt115::C0(_, h1322, _) => {
                    h1322
                }
            }, match v0 {
                Adt115::C0(_, _, h1323) => {
                    h1323
                }
            }), v1, v2))
        }
    }
}

pub fn f320(v0: Adt115, v1: u64, v2: u64, v3: Adt118, v4: bool) -> R<Adt136> {
    let v5: i64 = f116({
        let a1325 = match v3.clone() {
            Adt118::C0(_, _, _, _, _, _, h1324) => {
                h1324
            }
        };
        let a1326 = 10i64;
        int_mul(a1325, a1326)?
    }, f302({
        let a1327 = v2;
        let a1328 = 48u64;
        nat_sub(a1327, a1328)
    }))?;
    let m1331 = {
        let a1329 = 99999i64;
        let a1330 = v5;
        compare(a1329, a1330)
    };
    if match m1331 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f249(Adt100::C8(v1)))
    } else {
        Ok(f248(v0, f301(match v3.clone() {
            Adt118::C0(h1332, _, _, _, _, _, _) => {
                h1332
            }
        }, Adt119::C7, match v3.clone() {
            Adt118::C0(_, _, h1333, _, _, _, _) => {
                h1333
            }
        }, match v3.clone() {
            Adt118::C0(_, _, _, h1334, _, _, _) => {
                h1334
            }
        }, match v3 {
            Adt118::C0(_, _, _, _, h1335, _, _) => {
                h1335
            }
        }, v4, v5)))
    }
}

pub fn f321(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1336 = f398(f399(), v2.clone());
    match m1336 {
        None => {
            let m1337 = f400(f401(), v2.clone());
            match m1337 {
                None => {
                    f402(v0, v1, v2)
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

pub fn f322(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt121) -> R<Adt148> {
    let m1338 = f236(Str::lit("x-mcp-header"), v1.clone());
    match m1338 {
        Adt135::C0 => {
            Ok(Adt148::C1(v4))
        }
        Adt135::C1(v5) => {
            f403(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f323(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt106>) -> List<Adt106> {
    f404(Str::lit("properties"), v0.clone(), v1, v2, f404(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f405(Str::lit("items"), v0.clone(), f405(Str::lit("additionalProperties"), v0.clone(), f406(Str::lit("allOf"), v0.clone(), f406(Str::lit("anyOf"), v0.clone(), f406(Str::lit("oneOf"), v0.clone(), f405(Str::lit("not"), v0.clone(), f405(Str::lit("if"), v0.clone(), f405(Str::lit("then"), v0.clone(), f405(Str::lit("else"), v0, v3)))))))))))
}

pub fn f324(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt123>) -> R<Adt142> {
    let m1339 = f407(v1.clone(), v2.clone())?;
    match m1339 {
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
            f408(v0, v1, v2, v3)
        }
    }
}

pub fn f325(v0: List<Adt6>, v1: Adt6, v2: List<Adt123>, v3: u64, v4: List<Adt125>) -> Adt137 {
    let m1340 = v0;
    match m1340.uncons() {
        None => {
            let a1341 = v3;
            let a1342 = 1u64;
            if nat_eq(a1341, a1342) {
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

pub fn f326(v0: Option<Adt6>, v1: Adt6, v2: List<Adt123>, v3: List<Adt125>) -> Adt137 {
    let m1343 = v0;
    match m1343 {
        None => {
            Adt137::C0(Adt124::C0, v3)
        }
        Some(v4) => {
            Adt137::C0(Adt124::C0, List::cons(Adt125::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f327(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1344 = f236(Str::lit("anyOf"), v0);
    match m1344 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1345 = f409(v3);
            match m1345 {
                None => {
                    Adt142::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1346 = v4;
                    match m1346.uncons() {
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

pub fn f328(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1347 = f236(Str::lit("oneOf"), v0);
    match m1347 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1348 = f409(v3);
            match m1348 {
                None => {
                    Adt142::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1349 = v4;
                    match m1349.uncons() {
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

pub fn f329(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1350 = f236(Str::lit("not"), v0);
    match m1350 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            Adt142::C3(List::cons(Adt125::C0(v3, v1, v2.clone()), List::cons(Adt125::C3(v2), List::<Adt125>::nil())))
        }
    }
}

pub fn f330(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1351 = f236(Str::lit("if"), v0.clone());
    match m1351 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            Adt142::C3(List::cons(Adt125::C0(v3, v1.clone(), v2.clone()), List::cons(Adt125::C4(f410(Str::lit("then"), v0.clone()), f410(Str::lit("else"), v0), v1, v2), List::<Adt125>::nil())))
        }
    }
}

pub fn f331(v0: List<Adt6>, v1: Adt6, v2: List<Adt123>, v3: List<Adt125>) -> Adt137 {
    let m1352 = v0;
    match m1352.uncons() {
        None => {
            Adt137::C0(Adt124::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt137::C0(Adt124::C0, List::cons(Adt125::C0(v4, v1.clone(), v2.clone()), List::cons(Adt125::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f332(v0: Adt123) -> R<Str> {
    let m1353 = v0;
    match m1353 {
        Adt123::C0(v1) => {
            f411(v1)
        }
        Adt123::C1(v2) => {
            let a1354 = f80(v2, 0i64)?;
            Ok(format_i64(a1354))
        }
    }
}

pub fn f333(v0: Str) -> R<bool> {
    let m1356 = f291(f86({
        let a1355 = v0;
        utf8_encode(a1355)
    })?)?;
    match m1356 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f334(v0: Adt38) -> bool {
    let m1358 = match v0 {
        Adt38::C0(_, _, h1357) => {
            h1357
        }
    };
    match m1358 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f335(v0: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1360 = match v0.clone() {
        Adt50::C0(_, h1359) => {
            h1359
        }
    };
    match m1360.uncons() {
        None => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(match v0 {
                Adt50::C0(h1361, _) => {
                    h1361
                }
            })))
        }
        Some((_, _)) => {
            if f412(match v0.clone() {
                Adt50::C0(_, h1362) => {
                    h1362
                }
            })? {
                Ok(Ok::<Adt33, Adt90>(Adt33::C3(v0)))
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f336(v0: Adt5, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1366 = {
        let m1365 = match v1.clone() {
            Adt50::C0(h1364, _) => {
                h1364
            }
        };
        match m1365 {
            Adt38::C0(_, h1363, _) => {
                h1363
            }
        }
    };
    match m1366 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1369 = f157(Adt6::C5(match v0.clone() {
                Adt5::C0(h1367) => {
                    h1367
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1368) => {
                    h1368
                }
            }), v2)?;
            match m1369 {
                Adt122::C0 => {
                    f335(v1)
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

pub fn f337(p1378: Bytes, p1379: List<Bytes>) -> List<Bytes> {
    let m1377 = (p1378, p1379);
    {
        let mut m1377 = m1377;
        loop {
            let step = {
                let (v0, v1) = m1377;
                let m1370 = v1;
                match m1370.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1373 = {
                            let a1371 = v2.clone();
                            length_bytes(a1371)
                        };
                        let a1374 = {
                            let a1372 = v0.clone();
                            length_bytes(a1372)
                        };
                        if nat_eq(a1373, a1374) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1375 = v2;
                                let a1376 = v0;
                                append_bytes(a1375, a1376)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: u64) -> Bytes {
    let m1383 = {
        let a1380 = f413();
        let a1381 = v0;
        let a1382 = 1u64;
        slice_bytes(a1380, a1381, a1382)
    };
    match m1383 {
        None => {
            f88()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f339(v0: u64) -> List<u64> {
    f312(v0)
}

pub fn f340(v0: Adt7) -> R<bool> {
    let a1401 = {
        let m1387 = {
            let a1385 = f414();
            let a1386 = match v0.clone() {
                Adt7::C0(h1384, _) => {
                    h1384
                }
            };
            compare(a1385, a1386)
        };
        if match m1387 {
            Ordering::Less => {
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
            let m1392 = {
                let a1390 = match v0.clone() {
                    Adt7::C0(h1388, _) => {
                        h1388
                    }
                };
                let a1391 = {
                    let a1389 = f414();
                    int_neg(a1389)?
                };
                compare(a1390, a1391)
            };
            if match m1392 {
                Ordering::Less => {
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
                let m1396 = {
                    let a1394 = 9999i64;
                    let a1395 = match v0.clone() {
                        Adt7::C0(_, h1393) => {
                            h1393
                        }
                    };
                    compare(a1394, a1395)
                };
                if match m1396 {
                    Ordering::Less => {
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
                    let m1400 = {
                        let a1398 = match v0 {
                            Adt7::C0(_, h1397) => {
                                h1397
                            }
                        };
                        let a1399 = -9999i64;
                        compare(a1398, a1399)
                    };
                    match m1400 {
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
    Ok(bool_not(a1401))
}

pub fn f341(v0: Adt7) -> R<List<u64>> {
    let m1405 = {
        let a1403 = match v0.clone() {
            Adt7::C0(_, h1402) => {
                h1402
            }
        };
        let a1404 = 0i64;
        compare(a1403, a1404)
    };
    if if match m1405 {
        Ordering::Less => {
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
        let m1409 = {
            let a1407 = 0i64;
            let a1408 = match v0.clone() {
                Adt7::C0(_, h1406) => {
                    h1406
                }
            };
            compare(a1407, a1408)
        };
        match m1409 {
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
        let a1412 = f415(match v0.clone() {
            Adt7::C0(h1410, _) => {
                h1410
            }
        })?;
        let a1413 = List::cons(101u64, f415(match v0 {
            Adt7::C0(_, h1411) => {
                h1411
            }
        })?);
        Ok(append_list(a1412, a1413))
    } else {
        f415(match v0 {
            Adt7::C0(h1414, _) => {
                h1414
            }
        })
    }
}

pub fn f342() -> u64 {
    512u64
}

pub fn f343(v0: u64, v1: Adt107, v2: List<Adt107>) -> List<Adt107> {
    List::cons(Adt107::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f344(p1417: List<u64>, p1418: List<u64>) -> R<List<u64>> {
    let m1416 = (p1417, p1418);
    {
        let mut m1416 = m1416;
        loop {
            let step = {
                let (v0, v1) = m1416;
                let m1415 = v0;
                match m1415.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f168(f416(v2)?, v1)))
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

pub fn f345(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f417(match v0.clone() {
        Adt34::C0(_, h1419, _, _) => {
            h1419
        }
    })), List::cons((Str::lit("capabilities"), f418(match v0.clone() {
        Adt34::C0(_, _, h1420, _) => {
            h1420
        }
    })), {
        let a1423 = f419(Str::lit("instructions"), match v0.clone() {
            Adt34::C0(_, _, _, h1421) => {
                h1421
            }
        });
        let a1424 = f420(match v0 {
            Adt34::C0(h1422, _, _, _) => {
                h1422
            }
        })?;
        append_list(a1423, a1424)
    })))
}

pub fn f346(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f421(match v0.clone() {
        Adt35::C0(_, _, h1425) => {
            h1425
        }
    }))), {
        let a1428 = f419(Str::lit("nextCursor"), match v0.clone() {
            Adt35::C0(_, h1426, _) => {
                h1426
            }
        });
        let a1429 = f420(match v0 {
            Adt35::C0(h1427, _, _) => {
                h1427
            }
        })?;
        append_list(a1428, a1429)
    }))
}

pub fn f347(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1430 = v0;
    match m1430 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1431 = f366(Adt6::C5(v1))?;
            match m1431 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1432 = f259(v2);
                    match m1432 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1433) => {
                                    h1433
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f348(v0: Adt38) -> R<Option<List<(Str, Adt6)>>> {
    let m1435 = f422(match v0.clone() {
        Adt38::C0(h1434, _, _) => {
            h1434
        }
    })?;
    match m1435 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1438 = f423(Str::lit("structuredContent"), match v0.clone() {
                    Adt38::C0(_, h1436, _) => {
                        h1436
                    }
                });
                let a1439 = f424(Str::lit("isError"), match v0 {
                    Adt38::C0(_, _, h1437) => {
                        h1437
                    }
                });
                append_list(a1438, a1439)
            })))
        }
    }
}

pub fn f349(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f425(match v0.clone() {
        Adt52::C0(_, _, h1440) => {
            h1440
        }
    })?)), {
        let a1443 = f419(Str::lit("nextCursor"), match v0.clone() {
            Adt52::C0(_, h1441, _) => {
                h1441
            }
        });
        let a1444 = f420(match v0 {
            Adt52::C0(h1442, _, _) => {
                h1442
            }
        })?;
        append_list(a1443, a1444)
    }))
}

pub fn f350(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f426(match v0.clone() {
        Adt53::C0(_, h1445) => {
            h1445
        }
    }))), f420(match v0 {
        Adt53::C0(h1446, _) => {
            h1446
        }
    })?))
}

pub fn f351(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f427(match v0.clone() {
        Adt54::C0(_, _, h1447) => {
            h1447
        }
    }))), {
        let a1450 = f419(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h1448, _) => {
                h1448
            }
        });
        let a1451 = f420(match v0 {
            Adt54::C0(h1449, _, _) => {
                h1449
            }
        })?;
        append_list(a1450, a1451)
    }))
}

pub fn f352(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f428(match v0.clone() {
        Adt56::C0(_, _, h1452) => {
            h1452
        }
    }))), {
        let a1455 = f419(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h1453, _) => {
                h1453
            }
        });
        let a1456 = f420(match v0 {
            Adt56::C0(h1454, _, _) => {
                h1454
            }
        })?;
        append_list(a1455, a1456)
    }))
}

pub fn f353(v0: Adt59) -> R<Option<List<(Str, Adt6)>>> {
    let m1458 = f429(match v0.clone() {
        Adt59::C0(_, h1457) => {
            h1457
        }
    })?;
    match m1458 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1460 = f419(Str::lit("description"), match v0 {
                    Adt59::C0(h1459, _) => {
                        h1459
                    }
                });
                let a1461 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1460, a1461)
            }))
        }
    }
}

pub fn f354(v0: Adt61) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f378({
        let m1464 = match v0.clone() {
            Adt61::C0(h1463) => {
                h1463
            }
        };
        match m1464 {
            Adt62::C0(h1462, _, _) => {
                h1462
            }
        }
    }))), {
        let a1471 = f430(Str::lit("total"), {
            let m1467 = match v0.clone() {
                Adt61::C0(h1466) => {
                    h1466
                }
            };
            match m1467 {
                Adt62::C0(_, h1465, _) => {
                    h1465
                }
            }
        })?;
        let a1472 = f424(Str::lit("hasMore"), {
            let m1470 = match v0 {
                Adt61::C0(h1469) => {
                    h1469
                }
            };
            match m1470 {
                Adt62::C0(_, _, h1468) => {
                    h1468
                }
            }
        });
        append_list(a1471, a1472)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f355(v0: Adt82) -> R<Option<List<(Str, Adt6)>>> {
    let m1473 = f431(f432(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m1473 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1474 = f433(v1);
                let a1475 = f434(f435(v0));
                append_list(a1474, a1475)
            }))
        }
    }
}

pub fn f356(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1476 = f436(v0.clone());
    match m1476 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1477 = f437(f438(v0.clone()), f439(f440(v0)));
                let a1478 = v1;
                append_list(a1477, a1478)
            })
        }
    }
}

pub fn f357(v0: Adt88) -> Option<List<(Str, Adt6)>> {
    let m1480 = match v0.clone() {
        Adt88::C0(_, h1479, _) => {
            h1479
        }
    };
    match m1480 {
        Adt89::C0 => {
            f356({
                let m1483 = match v0 {
                    Adt88::C0(h1482, _, _) => {
                        h1482
                    }
                };
                match m1483 {
                    Adt85::C0(h1481, _, _) => {
                        h1481
                    }
                }
            })
        }
        Adt89::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f358(v0: Adt149) -> Str {
    let m1484 = v0;
    match m1484 {
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

pub fn f359(v0: Adt33) -> Adt149 {
    let m1485 = v0;
    match m1485 {
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

pub fn f360(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1486 = v0;
    match m1486 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f441(), f442(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f361(v0: Bytes) -> R<Option<List<Adt85>>> {
    let a1488 = {
        let a1487 = v0.clone();
        length_bytes(a1487)
    };
    let a1489 = 0u64;
    if nat_eq(a1488, a1489) {
        Ok(Some(List::<Adt85>::nil()))
    } else {
        let m1490 = f69(v0)?;
        match m1490 {
            Ok(v2) => {
                let m1491 = f409(v2);
                match m1491 {
                    None => {
                        Ok(None::<List<Adt85>>)
                    }
                    Some(v3) => {
                        Ok(f443(v3, List::<Adt85>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt85>>)
            }
        }
    }
}

pub fn f362(v0: List<Adt85>) -> R<Option<List<u64>>> {
    let m1492 = f444(v0);
    match m1492 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1493 = f90(v1)?;
            match m1493 {
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

pub fn f363(v0: Adt85, v1: List<Adt85>, v2: i64) -> R<List<Adt85>> {
    Ok(List::cons(v0.clone(), f445(f446(f447(v0), v1, v2, List::<Adt85>::nil())?, List::<Adt85>::nil())))
}

pub fn f364(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f448(v2) {
            let a1494 = v1;
            let a1495 = 200u64;
            nat_eq(a1494, a1495)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f365(v0: u64, v1: List<List<u64>>) -> Adt95 {
    Adt95::C0(v0, f449(), List::<List<u64>>::nil(), f450(v1), None::<List<u64>>)
}

pub fn f366(v0: Adt6) -> R<Option<Adt6>> {
    f451(f452(), Adt150::C0(v0), List::<Adt151>::nil())
}

pub fn f367(v0: Adt130) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f453(match v0.clone() {
        Adt130::C0(h1496, _) => {
            h1496
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1499 = match v0.clone() {
            Adt130::C0(_, h1498) => {
                h1498
            }
        };
        match m1499 {
            Adt51::C0(h1497, _, _) => {
                h1497
            }
        }
    })), {
        let a1506 = f454({
            let m1502 = match v0.clone() {
                Adt130::C0(_, h1501) => {
                    h1501
                }
            };
            match m1502 {
                Adt51::C0(_, h1500, _) => {
                    h1500
                }
            }
        });
        let a1507 = f455({
            let m1505 = match v0 {
                Adt130::C0(_, h1504) => {
                    h1504
                }
            };
            match m1505 {
                Adt51::C0(_, _, h1503) => {
                    h1503
                }
            }
        });
        append_list(a1506, a1507)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f368(v0: Adt88) -> bool {
    if match v0.clone() {
        Adt88::C0(_, _, h1508) => {
            h1508
        }
    } {
        let m1510 = match v0 {
            Adt88::C0(_, h1509, _) => {
                h1509
            }
        };
        match m1510 {
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

pub fn f369(v0: Adt63) -> Option<Adt6> {
    let m1511 = f356(v0);
    match m1511 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f370() -> u64 {
    429u64
}

pub fn f371() -> i64 {
    -32602i64
}

pub fn f372() -> i64 {
    -32022i64
}

pub fn f373() -> i64 {
    -32020i64
}

pub fn f374() -> i64 {
    -32601i64
}

pub fn f375(v0: Adt90) -> i64 {
    let m1512 = v0;
    match m1512 {
        Adt90::C0(_) => {
            f371()
        }
        Adt90::C1(_) => {
            f371()
        }
        Adt90::C2(_) => {
            f371()
        }
        Adt90::C3(_) => {
            f371()
        }
        Adt90::C4(_) => {
            f371()
        }
        Adt90::C5(_) => {
            f371()
        }
        Adt90::C6(_) => {
            f456()
        }
        Adt90::C7(_) => {
            f457()
        }
        Adt90::C8(_) => {
            f458()
        }
        Adt90::C9(_) => {
            f456()
        }
        Adt90::C10 => {
            f371()
        }
        Adt90::C11 => {
            f371()
        }
        Adt90::C12(_) => {
            f456()
        }
        Adt90::C13 => {
            f456()
        }
        Adt90::C14 => {
            f456()
        }
        Adt90::C15(_) => {
            f456()
        }
    }
}

pub fn f376(v0: Adt132) -> Str {
    let m1513 = v0;
    match m1513 {
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

pub fn f377(v0: Adt109) -> Str {
    let m1514 = v0;
    match m1514 {
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
            let m1515 = v1;
            match m1515 {
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
            let m1516 = v2;
            match m1516 {
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

pub fn f378(v0: List<Str>) -> List<Adt6> {
    f303(f459(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f379(v0: Adt18) -> Adt6 {
    Adt6::C5({
        let a1528 = f460(Str::lit("experimental"), match v0.clone() {
            Adt18::C0(h1517, _, _, _, _) => {
                h1517
            }
        });
        let a1529 = {
            let a1526 = f460(Str::lit("roots"), match v0.clone() {
                Adt18::C0(_, h1518, _, _, _) => {
                    h1518
                }
            });
            let a1527 = {
                let a1524 = f460(Str::lit("sampling"), match v0.clone() {
                    Adt18::C0(_, _, h1519, _, _) => {
                        h1519
                    }
                });
                let a1525 = {
                    let a1522 = f460(Str::lit("elicitation"), match v0.clone() {
                        Adt18::C0(_, _, _, h1520, _) => {
                            h1520
                        }
                    });
                    let a1523 = f460(Str::lit("extensions"), match v0 {
                        Adt18::C0(_, _, _, _, h1521) => {
                            h1521
                        }
                    });
                    append_list(a1522, a1523)
                };
                append_list(a1524, a1525)
            };
            append_list(a1526, a1527)
        };
        append_list(a1528, a1529)
    })
}

pub fn f380(p1532: List<List<u64>>) -> R<bool> {
    let m1531 = p1532;
    {
        let mut m1531 = m1531;
        loop {
            let step = {
                let v0 = m1531;
                let m1530 = v0;
                match m1530.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f461(f142(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f382(v0: Adt0, v1: Fn2, v2: u64, v3: Adt139, v4: bool) -> R<Result<Adt128, Adt131>> {
    if v4 {
        f462(v0, v1, v2, f463(match v3.clone() {
            Adt139::C0(_, h1533, _, _) => {
                h1533
            }
        }, match v3.clone() {
            Adt139::C0(_, _, h1534, _) => {
                h1534
            }
        }, match v3 {
            Adt139::C0(_, _, _, h1535) => {
                h1535
            }
        }))
    } else {
        Ok(Err::<Adt128, Adt131>(Adt131::C3(f464(match v3 {
            Adt139::C0(_, h1536, _, _) => {
                h1536
            }
        }))))
    }
}

pub fn f383(v0: Adt4, v1: Adt140) -> bool {
    let m1537 = v1;
    match m1537 {
        Adt140::C0 => {
            true
        }
        Adt140::C1 => {
            f465(match v0 {
                Adt4::C0(_, _, _, _, _, h1538, _) => {
                    h1538
                }
            })
        }
        Adt140::C2 => {
            f465(match v0 {
                Adt4::C0(_, _, _, _, _, h1539, _) => {
                    h1539
                }
            })
        }
        Adt140::C3 => {
            f466(match v0 {
                Adt4::C0(_, _, _, _, h1540, _, _) => {
                    h1540
                }
            })
        }
        Adt140::C4 => {
            f466(match v0 {
                Adt4::C0(_, _, _, _, h1541, _, _) => {
                    h1541
                }
            })
        }
        Adt140::C5 => {
            f466(match v0 {
                Adt4::C0(_, _, _, _, h1542, _, _) => {
                    h1542
                }
            })
        }
        Adt140::C6 => {
            f465(match v0 {
                Adt4::C0(_, _, _, h1543, _, _, _) => {
                    h1543
                }
            })
        }
        Adt140::C7 => {
            f465(match v0 {
                Adt4::C0(_, _, _, h1544, _, _, _) => {
                    h1544
                }
            })
        }
        Adt140::C8 => {
            f467(match v0 {
                Adt4::C0(_, _, h1545, _, _, _, _) => {
                    h1545
                }
            })
        }
        Adt140::C9 => {
            f468(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1546) => {
                    h1546
                }
            }, f26())
        }
        Adt140::C10 => {
            f468(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1547) => {
                    h1547
                }
            }, f26())
        }
        Adt140::C11 => {
            f468(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1548) => {
                    h1548
                }
            }, f26())
        }
    }
}

pub fn f384(v0: Adt145, v1: Adt143) -> Result<Adt139, Adt131> {
    let m1549 = v1;
    match m1549 {
        Adt143::C0 => {
            Err::<Adt139, Adt131>(Adt131::C3(match v0 {
                Adt145::C0(_, h1550, _, _) => {
                    h1550
                }
            }))
        }
        Adt143::C1(v2) => {
            Ok::<Adt139, Adt131>(Adt139::C0(match v0.clone() {
                Adt145::C0(h1551, _, _, _) => {
                    h1551
                }
            }, v2, match v0.clone() {
                Adt145::C0(_, _, h1552, _) => {
                    h1552
                }
            }, match v0 {
                Adt145::C0(_, _, _, h1553) => {
                    h1553
                }
            }))
        }
    }
}

pub fn f385(v0: Option<Adt108>, v1: List<Str>, v2: Adt105, v3: Result<Adt17, Adt132>) -> R<Adt144> {
    let m1554 = v3;
    match m1554 {
        Ok(v5) => {
            Ok(f469(v1, v2.clone(), v5.clone(), f470(v0, f289(match v2.clone() {
                Adt105::C0(_, h1555, _) => {
                    h1555
                }
            }), match v2.clone() {
                Adt105::C0(_, h1556, _) => {
                    h1556
                }
            }, match v5 {
                Adt17::C0(h1557, _, _, _, _, _) => {
                    h1557
                }
            }, f471(match v2 {
                Adt105::C0(_, _, h1558) => {
                    h1558
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt144::C1(Adt131::C0(v4)))
        }
    }
}

pub fn f386(v0: Option<Adt5>) -> Result<Adt17, Adt132> {
    let m1559 = v0;
    match m1559 {
        None => {
            Err::<Adt17, Adt132>(Adt132::C0)
        }
        Some(v1) => {
            let m1560 = f181(Str::lit("_meta"), v1);
            match m1560 {
                Adt135::C0 => {
                    Err::<Adt17, Adt132>(Adt132::C1)
                }
                Adt135::C1(v2) => {
                    let m1561 = f259(v2);
                    match m1561 {
                        None => {
                            Err::<Adt17, Adt132>(Adt132::C4)
                        }
                        Some(v3) => {
                            f472(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f387(v0: Adt140) -> Adt146 {
    let m1562 = v0;
    match m1562 {
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

pub fn f388(p1568: List<u64>, p1569: List<u64>) -> Option<List<u64>> {
    let m1567 = (p1568, p1569);
    {
        let mut m1567 = m1567;
        loop {
            let step = {
                let (v0, v1) = m1567;
                let m1563 = v0;
                match m1563.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1564 = v1;
                        match m1564.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1565 = v4;
                                let a1566 = v2;
                                if nat_eq(a1565, a1566) {
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
                Ok(next) => { m1567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f390() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f391(p1578: u64, p1579: List<u64>, p1580: List<u64>) -> R<Option<List<u64>>> {
    let m1577 = (p1578, (p1579, p1580));
    {
        let mut m1577 = m1577;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1577;
                let m1570 = v0;
                if m1570 == 0 {
                    let m1571 = v1;
                    match m1571.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1570 - 1;
                    let m1572 = v1;
                    match m1572.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1573 = v7;
                            match m1573.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1574 = v9;
                                    match m1574.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1575 = v11;
                                            match m1575.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1576 = f473(v6, v8, v10, v12, f474(v13.clone()))?;
                                                    match m1576 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f168(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: Adt12, v1: Option<Str>) -> bool {
    let m1581 = v1;
    match m1581 {
        None => {
            false
        }
        Some(v2) => {
            let a1583 = match v0 {
                Adt12::C0(h1582, _, _, _) => {
                    h1582
                }
            };
            let a1584 = v2;
            equal(a1583, a1584)
        }
    }
}

pub fn f393(v0: List<Adt93>, v1: Str) -> R<Adt112> {
    let m1587 = f228(f71(v0, {
        let a1585 = f475();
        let a1586 = f476(v1)?;
        append_list(a1585, a1586)
    })?)?;
    match m1587 {
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
            let m1588 = f165(v2);
            match m1588 {
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

pub fn f394(v0: i64) -> bool {
    let a1595 = {
        let m1591 = {
            let a1589 = v0;
            let a1590 = 0i64;
            compare(a1589, a1590)
        };
        if match m1591 {
            Ordering::Less => {
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
            let m1594 = {
                let a1592 = 0i64;
                let a1593 = v0;
                compare(a1592, a1593)
            };
            match m1594 {
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
    bool_not(a1595)
}

pub fn f395(v0: Adt115, v1: u64, v2: Str) -> Adt136 {
    let m1597 = match v0.clone() {
        Adt115::C0(_, h1596, _) => {
            h1596
        }
    };
    match m1597.uncons() {
        None => {
            f249(Adt100::C2(v1))
        }
        Some((v3, v4)) => {
            let m1598 = v3;
            match m1598 {
                Adt120::C0(_) => {
                    f249(Adt100::C2(v1))
                }
                Adt120::C1(v6, _) => {
                    Adt136::C1(Adt115::C0(Adt116::C5, List::cons(Adt120::C1(v6, Some(v2)), v4), match v0 {
                        Adt115::C0(_, _, h1599) => {
                            h1599
                        }
                    }))
                }
            }
        }
    }
}

pub fn f396(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1600 = v0;
    let a1601 = 128u64;
    if nat_lt(a1600, a1601) {
        Ok(List::cons(v0, v1))
    } else {
        let a1602 = v0;
        let a1603 = 2048u64;
        if nat_lt(a1602, a1603) {
            Ok(List::cons({
                let a1604 = 128u64;
                let a1605 = f477(v0, 64u64);
                nat_add(a1604, a1605)?
            }, List::cons({
                let a1606 = 192u64;
                let a1607 = f478(v0, 64u64);
                nat_add(a1606, a1607)?
            }, v1)))
        } else {
            let a1608 = v0;
            let a1609 = 65536u64;
            if nat_lt(a1608, a1609) {
                Ok(List::cons({
                    let a1610 = 128u64;
                    let a1611 = f477(v0, 64u64);
                    nat_add(a1610, a1611)?
                }, List::cons({
                    let a1612 = 128u64;
                    let a1613 = f477(f478(v0, 64u64), 64u64);
                    nat_add(a1612, a1613)?
                }, List::cons({
                    let a1614 = 224u64;
                    let a1615 = f478(v0, 4096u64);
                    nat_add(a1614, a1615)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1616 = 128u64;
                    let a1617 = f477(v0, 64u64);
                    nat_add(a1616, a1617)?
                }, List::cons({
                    let a1618 = 128u64;
                    let a1619 = f477(f478(v0, 64u64), 64u64);
                    nat_add(a1618, a1619)?
                }, List::cons({
                    let a1620 = 128u64;
                    let a1621 = f477(f478(v0, 4096u64), 64u64);
                    nat_add(a1620, a1621)?
                }, List::cons({
                    let a1622 = 240u64;
                    let a1623 = f478(v0, 262144u64);
                    nat_add(a1622, a1623)?
                }, v1)))))
            }
        }
    }
}

pub fn f397() -> u64 {
    18u64
}

pub fn f398(p1627: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1626 = p1627;
    {
        let mut m1626 = m1626;
        loop {
            let step = {
                let v0 = m1626;
                let m1624 = v0;
                match m1624.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1625 = f236(v2.clone(), v1.clone());
                        match m1625 {
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
                Ok(next) => { m1626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f400(p1634: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1633 = p1634;
    {
        let mut m1633 = m1633;
        loop {
            let step = {
                let v0 = m1633;
                let m1628 = v0;
                match m1628.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1630 = f236({
                            let (h1629, _) = v2.clone();
                            h1629
                        }, v1.clone());
                        match m1630 {
                            Adt135::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt135::C1(v4) => {
                                if f479({
                                    let (_, h1631) = v2;
                                    h1631
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1632, _) = v2;
                                        h1632
                                    }))
                                }
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

pub fn f401() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f402(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1635 = f236(Str::lit("$ref"), v2);
    match m1635 {
        Adt135::C0 => {
            None::<Adt98>
        }
        Adt135::C1(v3) => {
            let m1636 = f233(v3);
            match m1636 {
                None => {
                    None::<Adt98>
                }
                Some(v4) => {
                    let m1637 = f480(v1, v4.clone());
                    match m1637 {
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

pub fn f403(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt121) -> R<Adt148> {
    if v3 {
        let m1638 = f233(v2);
        match m1638 {
            None => {
                Ok(Adt148::C0(Adt98::C7(v0)))
            }
            Some(v6) => {
                f481(v0, v1, v6.clone(), f114(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt148::C0(Adt98::C10(v0)))
    }
}

pub fn f404(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt106>) -> List<Adt106> {
    let m1639 = f236(v0.clone(), v1);
    match m1639 {
        Adt135::C0 => {
            v4
        }
        Adt135::C1(v5) => {
            let m1640 = f259(v5);
            match m1640 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1641 = v0;
                    let a1642 = Str::lit("properties");
                    if equal(a1641, a1642) {
                        f482(match v6 {
                            Adt5::C0(h1643) => {
                                h1643
                            }
                        }, v2, v3, v4)
                    } else {
                        f483(match v6 {
                            Adt5::C0(h1644) => {
                                h1644
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f405(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt106>) -> List<Adt106> {
    let m1645 = f236(v0, v1);
    match m1645 {
        Adt135::C0 => {
            v2
        }
        Adt135::C1(v3) => {
            List::cons(Adt106::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f406(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt106>) -> List<Adt106> {
    let m1646 = f236(v0, v1);
    match m1646 {
        Adt135::C0 => {
            v2
        }
        Adt135::C1(v3) => {
            let m1647 = f409(v3);
            match m1647 {
                None => {
                    v2
                }
                Some(v4) => {
                    f484(v4, v2)
                }
            }
        }
    }
}

pub fn f407(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    Ok(f485(List::cons((Str::lit("type"), f486(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f487(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f488(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f489(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f489(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f489(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f489(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1648 = f490(v0.clone(), v1.clone())?;
        let a1649 = List::cons((Str::lit("required"), f491(v0, v1)), List::<(Str, Adt152)>::nil());
        append_list(a1648, a1649)
    })))))))))
}

pub fn f408(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt123>) -> R<Adt142> {
    Ok(f492(f493(v0, v1.clone(), v2.clone(), v3.clone()), f492(f494(v1.clone(), v2.clone(), v3.clone()), f492(f495(v1.clone(), v2.clone(), v3.clone()), f492(f496(v1.clone(), v2.clone(), v3.clone())?, f492(f497(v1.clone(), v2.clone(), v3.clone()), f492(f498(Adt126::C0, f327(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f492(f498(Adt126::C1, f328(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f492(f498(Adt126::C2, f329(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f498(Adt126::C3, f330(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f409(v0: Adt6) -> Option<List<Adt6>> {
    let m1650 = v0;
    match m1650 {
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

pub fn f410(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1651 = f236(v0, v1);
    match m1651 {
        Adt135::C0 => {
            None::<Adt6>
        }
        Adt135::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f411(v0: Str) -> R<Str> {
    let m1654 = {
        let a1653 = f205(f88(), f168(f499(f86({
            let a1652 = v0.clone();
            utf8_encode(a1652)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1653)
    };
    match m1654 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f412(v0: List<Adt51>) -> R<bool> {
    let m1655 = v0;
    match m1655.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f500(v2, match v1 {
                Adt51::C0(h1656, _, _) => {
                    h1656
                }
            })
        }
    }
}

pub fn f413() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f414() -> i64 {
    999999999999999999i64
}

pub fn f415(v0: i64) -> R<List<u64>> {
    let m1659 = {
        let a1657 = v0;
        let a1658 = 0i64;
        compare(a1657, a1658)
    };
    if match m1659 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f501(20u64, {
            let a1660 = v0;
            int_neg(a1660)?
        }, List::<u64>::nil())?))
    } else {
        f501(20u64, v0, List::<u64>::nil())
    }
}

pub fn f416(v0: u64) -> R<List<u64>> {
    let a1661 = v0;
    let a1662 = 34u64;
    if nat_eq(a1661, a1662) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1663 = v0;
        let a1664 = 92u64;
        if nat_eq(a1663, a1664) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1665 = v0;
            let a1666 = 8u64;
            if nat_eq(a1665, a1666) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1667 = v0;
                let a1668 = 9u64;
                if nat_eq(a1667, a1668) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1669 = v0;
                    let a1670 = 10u64;
                    if nat_eq(a1669, a1670) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1671 = v0;
                        let a1672 = 12u64;
                        if nat_eq(a1671, a1672) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1673 = v0;
                            let a1674 = 13u64;
                            if nat_eq(a1673, a1674) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1675 = v0;
                                let a1676 = 32u64;
                                if nat_lt(a1675, a1676) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f502(f478(v0, 16u64))?, List::cons(f502(f477(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f417(v0: List<Str>) -> Adt6 {
    Adt6::C4(f378(v0))
}

pub fn f418(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1694 = f460(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1677, _, _, _, _, _, _) => {
                h1677
            }
        });
        let a1695 = {
            let a1692 = f460(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1678, _, _, _, _, _) => {
                    h1678
                }
            });
            let a1693 = {
                let a1690 = f460(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1679, _, _, _, _) => {
                        h1679
                    }
                });
                let a1691 = {
                    let a1688 = f503(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1680, _, _, _) => {
                            h1680
                        }
                    });
                    let a1689 = {
                        let a1686 = f504(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1681, _, _) => {
                                h1681
                            }
                        });
                        let a1687 = {
                            let a1684 = f503(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1682, _) => {
                                    h1682
                                }
                            });
                            let a1685 = f460(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1683) => {
                                    h1683
                                }
                            });
                            append_list(a1684, a1685)
                        };
                        append_list(a1686, a1687)
                    };
                    append_list(a1688, a1689)
                };
                append_list(a1690, a1691)
            };
            append_list(a1692, a1693)
        };
        append_list(a1694, a1695)
    })
}

pub fn f419(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1696 = v1;
    match m1696 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f420(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f505(match v0.clone() {
        Adt10::C0(h1697, _) => {
            h1697
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f506(match v0 {
        Adt10::C0(_, h1698) => {
            h1698
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f421(v0: List<Adt36>) -> List<Adt6> {
    f303(f507(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f422(v0: List<Adt39>) -> R<Option<List<Adt6>>> {
    let m1699 = f508(v0, List::<Adt6>::nil())?;
    match m1699 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f303(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f423(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1700 = v1;
    match m1700 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f424(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1701 = v1;
    match m1701 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f425(v0: List<Adt45>) -> R<List<Adt6>> {
    Ok(f303(f509(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f426(v0: List<Adt47>) -> List<Adt6> {
    f303(f510(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f427(v0: List<Adt55>) -> List<Adt6> {
    f303(f511(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f428(v0: List<Adt57>) -> List<Adt6> {
    f303(f512(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f429(v0: List<Adt60>) -> R<Option<List<Adt6>>> {
    let m1702 = f513(v0, List::<Adt6>::nil())?;
    match m1702 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f303(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f430(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1703 = v1;
    match m1703 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f505(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f431(p1709: List<(Str, Adt65)>, p1710: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m1708 = (p1709, p1710);
    {
        let mut m1708 = m1708;
        loop {
            let step = {
                let (v0, v1) = m1708;
                let m1704 = v0;
                match m1704.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1706 = f514({
                            let (_, h1705) = v2;
                            h1705
                        })?;
                        match m1706 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h1707, _) = v2;
                                    h1707
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: Adt82) -> List<(Str, Adt65)> {
    let m1711 = v0;
    match m1711 {
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

pub fn f433(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1712 = v0.clone();
    match m1712.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f305(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f434(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1713 = v0;
    match m1713 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f435(v0: Adt82) -> Option<Str> {
    let m1714 = v0;
    match m1714 {
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

pub fn f436(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1715 = v0;
    match m1715 {
        Adt63::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt63::C1(_, v3) => {
            let m1716 = f515(v3);
            match m1716 {
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
                Adt5::C0(h1717) => {
                    h1717
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

pub fn f437(v0: Adt64, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt64::C0(h1718, _, _, _, _, _) => {
            h1718
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a1724 = f516(Str::lit("statusMessage"), match v0.clone() {
            Adt64::C0(_, h1719, _, _, _, _) => {
                h1719
            }
        });
        let a1725 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, h1720, _, _, _) => {
                h1720
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, _, h1721, _, _) => {
                h1721
            }
        })), List::cons((Str::lit("ttlMs"), f517(match v0.clone() {
            Adt64::C0(_, _, _, _, h1722, _) => {
                h1722
            }
        })), f518(Str::lit("pollIntervalMs"), match v0 {
            Adt64::C0(_, _, _, _, _, h1723) => {
                h1723
            }
        }))));
        append_list(a1724, a1725)
    }))
}

pub fn f438(v0: Adt63) -> Adt64 {
    let m1726 = v0;
    match m1726 {
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

pub fn f439(v0: Adt84) -> Str {
    let m1727 = v0;
    match m1727 {
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

pub fn f440(v0: Adt63) -> Adt84 {
    let m1728 = v0;
    match m1728 {
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

pub fn f441() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f442(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1729, _, _, _, _, _) => {
            h1729
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1730, _, _, _) => {
            h1730
        }
    })), {
        let a1739 = f419(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1731, _, _, _, _) => {
                h1731
            }
        });
        let a1740 = {
            let a1737 = f419(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1732, _, _) => {
                    h1732
                }
            });
            let a1738 = {
                let a1735 = f419(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1733, _) => {
                        h1733
                    }
                });
                let a1736 = f519(match v0 {
                    Adt1::C0(_, _, _, _, _, h1734) => {
                        h1734
                    }
                });
                append_list(a1735, a1736)
            };
            append_list(a1737, a1738)
        };
        append_list(a1739, a1740)
    })))
}

pub fn f443(p1744: List<Adt6>, p1745: List<Adt85>) -> Option<List<Adt85>> {
    let m1743 = (p1744, p1745);
    {
        let mut m1743 = m1743;
        loop {
            let step = {
                let (v0, v1) = m1743;
                let m1741 = v0;
                match m1741.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(Some(f445(v1, List::<Adt85>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1742 = f520(v2);
                        match m1742 {
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
                Ok(next) => { m1743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f444(v0: List<Adt85>) -> Option<Adt6> {
    let m1746 = f521(v0, List::<Adt6>::nil());
    match m1746 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f303(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f445(p1749: List<Adt85>, p1750: List<Adt85>) -> List<Adt85> {
    let m1748 = (p1749, p1750);
    {
        let mut m1748 = m1748;
        loop {
            let step = {
                let (v0, v1) = m1748;
                let m1747 = v0;
                match m1747.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), List<Adt85>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt85>, List<Adt85>), List<Adt85>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(v0: Str, p1755: List<Adt85>, v2: i64, p1756: List<Adt85>) -> R<List<Adt85>> {
    let m1754 = (p1755, p1756);
    {
        let mut m1754 = m1754;
        loop {
            let step = {
                let (v1, v3) = m1754;
                let m1751 = v1;
                match m1751.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a1752 = f447(v4.clone());
                        let a1753 = v0.clone();
                        if if equal(a1752, a1753) {
                            true
                        } else {
                            f522(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1754 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Adt85) -> Str {
    let m1759 = f438(match v0 {
        Adt85::C0(h1758, _, _) => {
            h1758
        }
    });
    match m1759 {
        Adt64::C0(h1757, _, _, _, _, _) => {
            h1757
        }
    }
}

pub fn f448(v0: List<List<u64>>) -> bool {
    let m1760 = v0;
    match m1760.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f449() -> List<Adt93> {
    List::cons(Adt93::C0(f523(), f381()), List::cons(Adt93::C0(f524(), f525()), List::cons(Adt93::C0(f526(), f527()), List::<Adt93>::nil())))
}

pub fn f450(v0: List<List<u64>>) -> List<u64> {
    f528(f37(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f451(p1775: u64, p1776: Adt150, p1777: List<Adt151>) -> R<Option<Adt6>> {
    let m1774 = (p1775, (p1776, p1777));
    {
        let mut m1774 = m1774;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1774;
                let m1761 = v0;
                if m1761 == 0 {
                    Err::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1761 - 1;
                    let m1762 = v1;
                    match m1762 {
                        Adt150::C0(v4) => {
                            let m1763 = v4.clone();
                            match m1763 {
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
                                    let v8: Str = f529(v7)?;
                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1764 = v9;
                                    match m1764.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0(v10), List::cons(Adt151::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1765 = v12;
                                    match m1765.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f529({
                                                let (h1766, _) = v13;
                                                h1766
                                            })?;
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0({
                                                let (_, h1767) = v13;
                                                h1767
                                            }), List::cons(Adt151::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt150::C1(v16) => {
                            let m1768 = v2;
                            match m1768.uncons() {
                                None => {
                                    Err::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1769 = v17;
                                    match m1769 {
                                        Adt151::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1770 = v20;
                                            match m1770.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C4(f303(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0(v22), List::cons(Adt151::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt151::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1771 = v26;
                                            match m1771.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C5(f305(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f529({
                                                        let (h1772, _) = v28;
                                                        h1772
                                                    })?;
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0({
                                                        let (_, h1773) = v28;
                                                        h1773
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
                Ok(next) => { m1774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452() -> u64 {
    67108864u64
}

pub fn f453(v0: Adt20) -> Adt6 {
    let m1778 = v0;
    match m1778 {
        Adt20::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt20::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f454(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1779 = v0;
    match m1779 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f455(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1780 = v0;
    match m1780 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f456() -> i64 {
    -32603i64
}

pub fn f457() -> i64 {
    429i64
}

pub fn f458() -> i64 {
    -32021i64
}

pub fn f459(p1783: List<Str>, p1784: List<Adt6>) -> List<Adt6> {
    let m1782 = (p1783, p1784);
    {
        let mut m1782 = m1782;
        loop {
            let step = {
                let (v0, v1) = m1782;
                let m1781 = v0;
                match m1781.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f460(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1785 = v1;
    match m1785 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1786) => {
                    h1786
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f461(v0: List<u64>) -> R<bool> {
    let m1787 = v0;
    match m1787.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1788 = v2;
            match m1788.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1789 = f189(v1)?;
                    let a1790 = 113u64;
                    if if nat_eq(a1789, a1790) {
                        let a1791 = v3;
                        let a1792 = 61u64;
                        nat_eq(a1791, a1792)
                    } else {
                        false
                    } {
                        Ok(f530(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f462(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt15, Adt132>) -> R<Result<Adt128, Adt131>> {
    let m1793 = v3;
    match m1793 {
        Ok(v5) => {
            f531(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(Adt131::C0(v4)))
        }
    }
}

pub fn f463(v0: Adt140, v1: Adt17, v2: Adt5) -> Result<Adt15, Adt132> {
    let m1794 = v0;
    match m1794 {
        Adt140::C0 => {
            f532(v1, v2)
        }
        Adt140::C1 => {
            f533(v1, v2)
        }
        Adt140::C2 => {
            f534(v1, v2)
        }
        Adt140::C3 => {
            f535(v1, v2)
        }
        Adt140::C4 => {
            f536(v1, v2)
        }
        Adt140::C5 => {
            f537(v1, v2)
        }
        Adt140::C6 => {
            f538(v1, v2)
        }
        Adt140::C7 => {
            f539(v1, v2)
        }
        Adt140::C8 => {
            f540(v1, v2)
        }
        Adt140::C9 => {
            f541(v1, v2)
        }
        Adt140::C10 => {
            f542(v1, v2)
        }
        Adt140::C11 => {
            f543(v1, v2)
        }
    }
}

pub fn f464(v0: Adt140) -> Str {
    let m1795 = v0;
    match m1795 {
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

pub fn f465(v0: Option<Adt8>) -> bool {
    let m1796 = v0;
    match m1796 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f466(v0: Option<Adt9>) -> bool {
    let m1797 = v0;
    match m1797 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f467(v0: Option<Adt5>) -> bool {
    let m1798 = v0;
    match m1798 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f468(v0: Option<Adt5>, v1: Str) -> bool {
    let m1799 = v0;
    match m1799 {
        None => {
            false
        }
        Some(v2) => {
            f237(v1, v2)
        }
    }
}

pub fn f469(v0: List<Str>, v1: Adt105, v2: Adt17, v3: Option<Adt109>) -> Adt144 {
    let m1800 = v3;
    match m1800 {
        None => {
            f544(v1, v2.clone(), f545(match v2 {
                Adt17::C0(h1801, _, _, _, _, _) => {
                    h1801
                }
            }, v0))
        }
        Some(v4) => {
            Adt144::C1(Adt131::C2(v4))
        }
    }
}

pub fn f470(v0: Option<Adt108>, v1: Adt146, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt109>> {
    let m1802 = v0;
    match m1802 {
        None => {
            Ok(None::<Adt109>)
        }
        Some(v5) => {
            let m1804 = match v5.clone() {
                Adt108::C0(_, _, _, h1803, _) => {
                    h1803
                }
            };
            match m1804 {
                None => {
                    f546(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f471(v0: Option<Adt5>) -> Adt5 {
    let m1805 = v0;
    match m1805 {
        None => {
            f547()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f472(v0: Adt5) -> Result<Adt17, Adt132> {
    let m1806 = f548(v0.clone());
    match m1806 {
        Adt153::C0 => {
            f549(v0.clone(), f181(f550(), v0.clone()), f181(f551(), v0))
        }
        Adt153::C1 => {
            Err::<Adt17, Adt132>(Adt132::C2)
        }
        Adt153::C2 => {
            Err::<Adt17, Adt132>(Adt132::C3)
        }
    }
}

pub fn f473(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1807 = f552(v0)?;
    match m1807 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1808 = f552(v1)?;
            match m1808 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1809 = v2;
                    let a1810 = 61u64;
                    if nat_eq(a1809, a1810) {
                        f553(v5, v6, v4, v3)
                    } else {
                        let m1811 = f552(v2)?;
                        match m1811 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1812 = v3;
                                let a1813 = 61u64;
                                if nat_eq(a1812, a1813) {
                                    f554(v5, v6, v7, v4)
                                } else {
                                    let m1814 = f552(v3)?;
                                    match m1814 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f555(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f474(v0: List<u64>) -> bool {
    let m1815 = v0;
    match m1815.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f475() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f476(v0: Str) -> R<List<u64>> {
    f86({
        let a1816 = v0;
        utf8_encode(a1816)
    })
}

pub fn f477(v0: u64, v1: u64) -> u64 {
    let a1817 = v0;
    let a1818 = v1;
    let a1819 = 0u64;
    nat_rem(a1817, a1818, a1819)
}

pub fn f478(v0: u64, v1: u64) -> u64 {
    let a1820 = v0;
    let a1821 = v1;
    let a1822 = 0u64;
    nat_quot(a1820, a1821, a1822)
}

pub fn f479(v0: u64, v1: Adt6) -> bool {
    let a1823 = v0;
    let a1824 = 0u64;
    if nat_eq(a1823, a1824) {
        f556(v1)
    } else {
        let a1825 = v0;
        let a1826 = 1u64;
        if nat_eq(a1825, a1826) {
            f557(v1)
        } else {
            let a1827 = v0;
            let a1828 = 2u64;
            if nat_eq(a1827, a1828) {
                f558(v1)
            } else {
                let a1829 = v0;
                let a1830 = 3u64;
                if nat_eq(a1829, a1830) {
                    f559(v1)
                } else {
                    let a1831 = v0;
                    let a1832 = 4u64;
                    if nat_eq(a1831, a1832) {
                        f560(v1)
                    } else {
                        let a1833 = v0;
                        let a1834 = 5u64;
                        if nat_eq(a1833, a1834) {
                            f561(v1)
                        } else {
                            let a1835 = v0;
                            let a1836 = 6u64;
                            if nat_eq(a1835, a1836) {
                                f562(v1)
                            } else {
                                let a1837 = v0;
                                let a1838 = 7u64;
                                if nat_eq(a1837, a1838) {
                                    f563(v1)
                                } else {
                                    f564(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f480(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1839 = v1.clone();
    let a1840 = Str::lit("#");
    if equal(a1839, a1840) {
        Some(v0)
    } else {
        let m1841 = f259(v0);
        match m1841 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1842 = f181(Str::lit("$defs"), v2);
                match m1842 {
                    Adt135::C0 => {
                        None::<Adt6>
                    }
                    Adt135::C1(v3) => {
                        let m1843 = f259(v3);
                        match m1843 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f565(v1, match v4 {
                                    Adt5::C0(h1844) => {
                                        h1844
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

pub fn f481(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt121) -> R<Adt148> {
    let m1845 = v3.clone();
    match m1845.uncons() {
        None => {
            Ok(Adt148::C0(Adt98::C8(v0)))
        }
        Some((_, _)) => {
            if f193(v3.clone(), true) {
                let m1846 = f566(v1);
                match m1846 {
                    None => {
                        Ok(Adt148::C0(Adt98::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f567(v3)?;
                        if f568(v9.clone(), match v5.clone() {
                            Adt121::C0(_, h1847) => {
                                h1847
                            }
                        }) {
                            Ok(Adt148::C0(Adt98::C12(v0, v2)))
                        } else {
                            Ok(Adt148::C1(Adt121::C0(List::cons(Adt12::C0(v0, v2, f569(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt121::C0(h1848, _) => {
                                    h1848
                                }
                            }), List::cons(v9, match v5 {
                                Adt121::C0(_, h1849) => {
                                    h1849
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

pub fn f482(p1854: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1855: List<Adt106>) -> List<Adt106> {
    let m1853 = (p1854, p1855);
    {
        let mut m1853 = m1853;
        loop {
            let step = {
                let (v0, v3) = m1853;
                let m1850 = v0;
                match m1850.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>((v5, List::cons(Adt106::C0({
                            let (_, h1851) = v4;
                            h1851
                        }, v1, List::cons({
                            let (h1852, _) = v4;
                            h1852
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f483(p1859: List<(Str, Adt6)>, p1860: List<Adt106>) -> List<Adt106> {
    let m1858 = (p1859, p1860);
    {
        let mut m1858 = m1858;
        loop {
            let step = {
                let (v0, v1) = m1858;
                let m1856 = v0;
                match m1856.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>((v3, List::cons(Adt106::C0({
                            let (_, h1857) = v2;
                            h1857
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(p1863: List<Adt6>, p1864: List<Adt106>) -> List<Adt106> {
    let m1862 = (p1863, p1864);
    {
        let mut m1862 = m1862;
        loop {
            let step = {
                let (v0, v1) = m1862;
                let m1861 = v0;
                match m1861.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt106>), List<Adt106>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt106>), List<Adt106>>((v3, List::cons(Adt106::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(p1871: List<(Str, Adt152)>) -> Adt142 {
    let m1870 = p1871;
    {
        let mut m1870 = m1870;
        loop {
            let step = {
                let v0 = m1870;
                let m1865 = v0;
                match m1865.uncons() {
                    None => {
                        Err::<List<(Str, Adt152)>, Adt142>(Adt142::C3(List::<Adt125>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1867 = {
                            let (_, h1866) = v1;
                            h1866
                        };
                        match m1867 {
                            Adt152::C0 => {
                                Ok::<List<(Str, Adt152)>, Adt142>(v2)
                            }
                            Adt152::C1 => {
                                Err::<List<(Str, Adt152)>, Adt142>(Adt142::C0({
                                    let (h1868, _) = v1;
                                    h1868
                                }))
                            }
                            Adt152::C2 => {
                                Err::<List<(Str, Adt152)>, Adt142>(Adt142::C1({
                                    let (h1869, _) = v1;
                                    h1869
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1870 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f486(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m1872 = f236(Str::lit("type"), v0);
    match m1872 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v2) => {
            let m1873 = f233(v2.clone());
            match m1873 {
                None => {
                    let m1874 = f409(v2);
                    match m1874 {
                        None => {
                            Ok(Adt152::C2)
                        }
                        Some(v4) => {
                            f572(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f570(v3.clone()) {
                        if f571(v3, v1)? {
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

pub fn f487(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m1875 = f236(Str::lit("enum"), v0);
    match m1875 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v2) => {
            let m1876 = f409(v2);
            match m1876 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v3) => {
                    f573(v3, v1)
                }
            }
        }
    }
}

pub fn f488(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m1877 = f236(Str::lit("const"), v0);
    match m1877 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v2) => {
            let m1878 = f574(v2, v1)?;
            match m1878 {
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

pub fn f489(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt152> {
    let m1879 = f236(v0, v2);
    match m1879 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v4) => {
            let m1880 = f575(v4);
            match m1880 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v5) => {
                    let m1881 = f575(v3);
                    match m1881 {
                        None => {
                            Ok(Adt152::C0)
                        }
                        Some(v6) => {
                            if f576(v1, f577(v6, v5)?) {
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

pub fn f490(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt152)>> {
    let m1882 = v1;
    match m1882 {
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
            Ok(List::cons((Str::lit("minLength"), f578(Str::lit("minLength"), true, v0.clone(), f579(v4.clone())?)?), List::cons((Str::lit("maxLength"), f578(Str::lit("maxLength"), false, v0, f579(v4)?)?), List::<(Str, Adt152)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f578(Str::lit("minItems"), true, v0.clone(), f580(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f578(Str::lit("maxItems"), false, v0, f580(v5, 0u64)?)?), List::<(Str, Adt152)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f578(Str::lit("minProperties"), true, v0.clone(), f581(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f578(Str::lit("maxProperties"), false, v0, f581(v6, 0u64)?)?), List::<(Str, Adt152)>::nil())))
        }
    }
}

pub fn f491(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt152 {
    let m1883 = f236(Str::lit("required"), v0);
    match m1883 {
        Adt135::C0 => {
            Adt152::C0
        }
        Adt135::C1(v2) => {
            let m1884 = f409(v2);
            match m1884 {
                None => {
                    Adt152::C2
                }
                Some(v3) => {
                    let m1885 = f259(v1);
                    match m1885 {
                        None => {
                            Adt152::C0
                        }
                        Some(v4) => {
                            f582(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f492(v0: Adt142, v1: Adt142) -> Adt142 {
    let m1886 = v0.clone();
    match m1886 {
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
            let m1887 = v1.clone();
            match m1887 {
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
                        let a1888 = v5;
                        let a1889 = v9;
                        append_list(a1888, a1889)
                    })
                }
            }
        }
    }
}

pub fn f493(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt123>) -> Adt142 {
    let m1890 = f236(Str::lit("$ref"), v1);
    match m1890 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v4) => {
            let m1891 = f233(v4);
            match m1891 {
                None => {
                    Adt142::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1892 = f480(v0, v5.clone());
                    match m1892 {
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

pub fn f494(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1893 = f236(Str::lit("properties"), v0);
    match m1893 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1894 = f259(v3);
            match m1894 {
                None => {
                    Adt142::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1895 = f259(v1);
                    match m1895 {
                        None => {
                            Adt142::C3(List::<Adt125>::nil())
                        }
                        Some(v5) => {
                            Adt142::C3(f583(match v4 {
                                Adt5::C0(h1896) => {
                                    h1896
                                }
                            }, match v5 {
                                Adt5::C0(h1897) => {
                                    h1897
                                }
                            }, v2, List::<Adt125>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f495(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1898 = f236(Str::lit("additionalProperties"), v0.clone());
    match m1898 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1899 = f259(v1);
            match m1899 {
                None => {
                    Adt142::C3(List::<Adt125>::nil())
                }
                Some(v4) => {
                    let m1900 = f258(v3.clone());
                    match m1900 {
                        None => {
                            Adt142::C3(f586(match v4 {
                                Adt5::C0(h1902) => {
                                    h1902
                                }
                            }, f585(v0), v3, v2, List::<Adt125>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt142::C3(List::<Adt125>::nil())
                            } else {
                                if f584(match v4 {
                                    Adt5::C0(h1901) => {
                                        h1901
                                    }
                                }, f585(v0)) {
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

pub fn f496(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> R<Adt142> {
    let m1903 = f236(Str::lit("items"), v0);
    match m1903 {
        Adt135::C0 => {
            Ok(Adt142::C3(List::<Adt125>::nil()))
        }
        Adt135::C1(v3) => {
            let m1904 = f409(v1);
            match m1904 {
                None => {
                    Ok(Adt142::C3(List::<Adt125>::nil()))
                }
                Some(v4) => {
                    Ok(Adt142::C3(f587(v4, 0u64, v3, v2, List::<Adt125>::nil())?))
                }
            }
        }
    }
}

pub fn f497(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1905 = f236(Str::lit("allOf"), v0);
    match m1905 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1906 = f409(v3);
            match m1906 {
                None => {
                    Adt142::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1907 = v4.clone();
                    match m1907.uncons() {
                        None => {
                            Adt142::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt142::C3(f588(v4, v1, v2, List::<Adt125>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f498(v0: Adt126, v1: Adt142, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt123>) -> Adt142 {
    let m1908 = v1.clone();
    match m1908 {
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
            let m1909 = v8;
            match m1909.uncons() {
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

pub fn f499(p1916: List<u64>, p1917: List<u64>) -> List<u64> {
    let m1915 = (p1916, p1917);
    {
        let mut m1915 = m1915;
        loop {
            let step = {
                let (v0, v1) = m1915;
                let m1910 = v0;
                match m1910.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1911 = v2;
                        let a1912 = 126u64;
                        if nat_eq(a1911, a1912) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1913 = v2;
                            let a1914 = 47u64;
                            if nat_eq(a1913, a1914) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f500(p1922: List<Adt51>, p1923: Adt7) -> R<bool> {
    let m1921 = (p1922, p1923);
    {
        let mut m1921 = m1921;
        loop {
            let step = {
                let (v0, v1) = m1921;
                let m1918 = v0;
                match m1918.uncons() {
                    None => {
                        Err::<(List<Adt51>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f589(f577(match v2.clone() {
                            Adt51::C0(h1919, _, _) => {
                                h1919
                            }
                        }, v1)?) {
                            Ok::<(List<Adt51>, Adt7), R<bool>>((v3, match v2 {
                                Adt51::C0(h1920, _, _) => {
                                    h1920
                                }
                            }))
                        } else {
                            Err::<(List<Adt51>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(p1935: u64, p1936: i64, p1937: List<u64>) -> R<List<u64>> {
    let m1934 = (p1935, (p1936, p1937));
    {
        let mut m1934 = m1934;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1934;
                let m1924 = v0;
                if m1924 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1924 - 1;
                    let v4: i64 = {
                        let a1925 = v1;
                        let a1926 = 10i64;
                        let a1927 = 0i64;
                        int_quot(a1925, a1926, a1927)?
                    };
                    let v5: i64 = {
                        let a1928 = v1;
                        let a1929 = 10i64;
                        let a1930 = 0i64;
                        int_rem(a1928, a1929, a1930)
                    };
                    let m1933 = {
                        let a1931 = v4;
                        let a1932 = 1i64;
                        compare(a1931, a1932)
                    };
                    if match m1933 {
                        Ordering::Less => {
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
                Ok(next) => { m1934 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: u64) -> R<u64> {
    let a1938 = v0;
    let a1939 = 10u64;
    if nat_lt(a1938, a1939) {
        let a1940 = 48u64;
        let a1941 = v0;
        nat_add(a1940, a1941)
    } else {
        let a1942 = 87u64;
        let a1943 = v0;
        nat_add(a1942, a1943)
    }
}

pub fn f503(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1944 = v1;
    match m1944 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f591(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f504(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1945 = v0;
    match m1945 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1948 = f424(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1946, _) => {
                        h1946
                    }
                });
                let a1949 = f424(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1947) => {
                        h1947
                    }
                });
                append_list(a1948, a1949)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f505(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f592(v0, 0i64)?, 0i64)))
}

pub fn f506(v0: Adt11) -> Str {
    let m1950 = v0;
    match m1950 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f507(p1953: List<Adt36>, p1954: List<Adt6>) -> List<Adt6> {
    let m1952 = (p1953, p1954);
    {
        let mut m1952 = m1952;
        loop {
            let step = {
                let (v0, v1) = m1952;
                let m1951 = v0;
                match m1951.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt36>, List<Adt6>), List<Adt6>>((v3, List::cons(f593(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1952 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(p1958: List<Adt39>, p1959: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1957 = (p1958, p1959);
    {
        let mut m1957 = m1957;
        loop {
            let step = {
                let (v0, v1) = m1957;
                let m1955 = v0;
                match m1955.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1956 = f594(v2)?;
                        match m1956 {
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
                Ok(next) => { m1957 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(p1962: List<Adt45>, p1963: List<Adt6>) -> R<List<Adt6>> {
    let m1961 = (p1962, p1963);
    {
        let mut m1961 = m1961;
        loop {
            let step = {
                let (v0, v1) = m1961;
                let m1960 = v0;
                match m1960.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f595(v2)?, v1)))
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

pub fn f510(p1966: List<Adt47>, p1967: List<Adt6>) -> List<Adt6> {
    let m1965 = (p1966, p1967);
    {
        let mut m1965 = m1965;
        loop {
            let step = {
                let (v0, v1) = m1965;
                let m1964 = v0;
                match m1964.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), List<Adt6>>((v3, List::cons(f596(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1965 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(p1970: List<Adt55>, p1971: List<Adt6>) -> List<Adt6> {
    let m1969 = (p1970, p1971);
    {
        let mut m1969 = m1969;
        loop {
            let step = {
                let (v0, v1) = m1969;
                let m1968 = v0;
                match m1968.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt55>, List<Adt6>), List<Adt6>>((v3, List::cons(f597(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1969 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(p1974: List<Adt57>, p1975: List<Adt6>) -> List<Adt6> {
    let m1973 = (p1974, p1975);
    {
        let mut m1973 = m1973;
        loop {
            let step = {
                let (v0, v1) = m1973;
                let m1972 = v0;
                match m1972.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f598(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1973 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(p1979: List<Adt60>, p1980: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1978 = (p1979, p1980);
    {
        let mut m1978 = m1978;
        loop {
            let step = {
                let (v0, v1) = m1978;
                let m1976 = v0;
                match m1976.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1977 = f599(v2)?;
                        match m1977 {
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
                Ok(next) => { m1978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: Adt65) -> R<Option<Adt6>> {
    let m1981 = v0;
    match m1981 {
        Adt65::C0(v1) => {
            Ok(Some(f600(Str::lit("elicitation/create"), Some(f601(v1)))))
        }
        Adt65::C1(v2) => {
            let m1982 = f602(v2)?;
            match m1982 {
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
                let m1983 = v4;
                match m1983 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h1984) => {
                                h1984
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f515(v0: List<(Str, Adt65)>) -> Option<Adt6> {
    let m1985 = f603(v0, List::<(Str, Adt6)>::nil());
    match m1985 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f305(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f516(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1986 = v1;
    match m1986 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f517(v0: Option<i64>) -> Adt6 {
    let m1987 = v0;
    match m1987 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f604(v1)
        }
    }
}

pub fn f518(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m1988 = v1;
    match m1988 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f604(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f519(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1989 = v0;
    match m1989 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f605(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f520(v0: Adt6) -> Option<Adt85> {
    let m1990 = f259(v0);
    match m1990 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m1991 = f606(Str::lit("task"), v1.clone());
            match m1991 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m1992 = f607(v2);
                    match m1992 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m1993 = f608(Str::lit("createdMs"), v1.clone());
                            match m1993 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m1994 = f606(Str::lit("plan"), v1);
                                    match m1994 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            let m1995 = f609(v5);
                                            match m1995 {
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

pub fn f521(p1999: List<Adt85>, p2000: List<Adt6>) -> Option<List<Adt6>> {
    let m1998 = (p1999, p2000);
    {
        let mut m1998 = m1998;
        loop {
            let step = {
                let (v0, v1) = m1998;
                let m1996 = v0;
                match m1996.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1997 = f610(v2);
                        match m1997 {
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
                Ok(next) => { m1998 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(v0: Adt85, v1: i64) -> R<bool> {
    let m2004 = {
        let m2003 = f438(match v0.clone() {
            Adt85::C0(h2002, _, _) => {
                h2002
            }
        });
        match m2003 {
            Adt64::C0(_, _, _, _, h2001, _) => {
                h2001
            }
        }
    };
    match m2004 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a2011 = {
                let m2010 = {
                    let a2008 = {
                        let a2006 = v1;
                        let a2007 = match v0 {
                            Adt85::C0(_, h2005, _) => {
                                h2005
                            }
                        };
                        int_sub(a2006, a2007)?
                    };
                    let a2009 = v2;
                    compare(a2008, a2009)
                };
                match m2010 {
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
            Ok(bool_not(a2011))
        }
    }
}

pub fn f523() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f524() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f525() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f526() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f527() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f528(p2016: List<List<u64>>, p2017: List<u64>) -> List<u64> {
    let m2015 = (p2016, p2017);
    {
        let mut m2015 = m2015;
        loop {
            let step = {
                let (v0, v1) = m2015;
                let m2012 = v0;
                match m2012.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2013 = f611(v2);
                            let a2014 = v1;
                            append_list(a2013, a2014)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2015 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: Str) -> R<Str> {
    let m2020 = {
        let a2019 = f205(f88(), f168(f612(f86({
            let a2018 = v0;
            utf8_encode(a2018)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2019)
    };
    match m2020 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f530(v0: List<u64>) -> bool {
    let m2021 = v0;
    match m2021.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2022 = v1;
            let a2023 = 48u64;
            if nat_eq(a2022, a2023) {
                f613(v2)
            } else {
                false
            }
        }
    }
}

pub fn f531(v0: Adt0, v1: Fn2, v2: u64, v3: Adt15) -> R<Result<Adt128, Adt131>> {
    if f614(f615(v3.clone())) {
        Ok(Ok::<Adt128, Adt131>(f616(v0.clone(), Adt33::C0(f617(v0)))))
    } else {
        Ok(f618(v0.clone(), v3.clone(), f619(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2024, _) => {
                h2024
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2025, _, _) => {
                h2025
            }
        }, v2, v1, v3)?))
    }
}

pub fn f532(v0: Adt17, _: Adt5) -> Result<Adt15, Adt132> {
    Ok::<Adt15, Adt132>(Adt15::C0(Adt16::C0(v0)))
}

pub fn f533(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2026 = f620(Str::lit("cursor"), v1);
    match m2026 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C1(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f534(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2027 = f621(Str::lit("name"), v1.clone());
    match m2027 {
        Ok(v2) => {
            let m2028 = f622(Str::lit("arguments"), v1.clone());
            match m2028 {
                Ok(v3) => {
                    let m2029 = f622(Str::lit("inputResponses"), v1.clone());
                    match m2029 {
                        Ok(v4) => {
                            let m2030 = f620(Str::lit("requestState"), v1);
                            match m2030 {
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

pub fn f535(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2031 = f620(Str::lit("cursor"), v1);
    match m2031 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C3(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f536(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2032 = f621(Str::lit("uri"), v1.clone());
    match m2032 {
        Ok(v2) => {
            let m2033 = f622(Str::lit("inputResponses"), v1.clone());
            match m2033 {
                Ok(v3) => {
                    let m2034 = f620(Str::lit("requestState"), v1);
                    match m2034 {
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

pub fn f537(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2035 = f620(Str::lit("cursor"), v1);
    match m2035 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C5(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f538(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2036 = f620(Str::lit("cursor"), v1);
    match m2036 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C6(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f539(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2037 = f621(Str::lit("name"), v1.clone());
    match m2037 {
        Ok(v2) => {
            let m2038 = f623(Str::lit("arguments"), v1.clone());
            match m2038 {
                Ok(v3) => {
                    let m2039 = f622(Str::lit("inputResponses"), v1.clone());
                    match m2039 {
                        Ok(v4) => {
                            let m2040 = f620(Str::lit("requestState"), v1);
                            match m2040 {
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

pub fn f540(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2041 = f624(Str::lit("ref"), v1.clone());
    match m2041 {
        Ok(v2) => {
            let m2042 = f625(v2);
            match m2042 {
                Ok(v3) => {
                    let m2043 = f624(Str::lit("argument"), v1.clone());
                    match m2043 {
                        Ok(v4) => {
                            let m2044 = f626(v4);
                            match m2044 {
                                Ok(v5) => {
                                    let m2045 = f627(v1);
                                    match m2045 {
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

pub fn f541(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2046 = f621(Str::lit("taskId"), v1);
    match m2046 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C9(Adt30::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f542(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2047 = f621(Str::lit("taskId"), v1.clone());
    match m2047 {
        Ok(v2) => {
            let m2048 = f624(Str::lit("inputResponses"), v1);
            match m2048 {
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

pub fn f543(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2049 = f621(Str::lit("taskId"), v1);
    match m2049 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C11(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f544(v0: Adt105, v1: Adt17, v2: Option<Adt131>) -> Adt144 {
    let m2050 = v2;
    match m2050 {
        None => {
            Adt144::C0(Adt145::C0(match v0.clone() {
                Adt105::C0(h2051, _, _) => {
                    h2051
                }
            }, match v0.clone() {
                Adt105::C0(_, h2052, _) => {
                    h2052
                }
            }, v1, f471(match v0 {
                Adt105::C0(_, _, h2053) => {
                    h2053
                }
            })))
        }
        Some(v3) => {
            Adt144::C1(v3)
        }
    }
}

pub fn f545(v0: Str, v1: List<Str>) -> Option<Adt131> {
    f628(f629(v0.clone(), v1.clone()), v0, v1)
}

pub fn f546(v0: Adt108, v1: Adt146, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt109>> {
    let m2055 = match v0.clone() {
        Adt108::C0(h2054, _, _, _, _) => {
            h2054
        }
    };
    match m2055 {
        None => {
            Ok(Some(Adt109::C0))
        }
        Some(v5) => {
            let a2056 = v5;
            let a2057 = v3;
            if equal(a2056, a2057) {
                let m2059 = match v0.clone() {
                    Adt108::C0(_, h2058, _, _, _) => {
                        h2058
                    }
                };
                match m2059 {
                    None => {
                        Ok(Some(Adt109::C1))
                    }
                    Some(v6) => {
                        let a2060 = v6;
                        let a2061 = v2;
                        if equal(a2060, a2061) {
                            f630(f631(match v0.clone() {
                                Adt108::C0(_, _, h2062, _, _) => {
                                    h2062
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt108::C0(_, _, _, _, h2063) => {
                                    h2063
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

pub fn f547() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f548(v0: Adt5) -> Adt153 {
    f632(f181(f550(), v0.clone()), f181(f551(), v0))
}

pub fn f549(v0: Adt5, v1: Adt135, v2: Adt135) -> Result<Adt17, Adt132> {
    let m2064 = v1;
    match m2064 {
        Adt135::C0 => {
            Err::<Adt17, Adt132>(Adt132::C2)
        }
        Adt135::C1(v3) => {
            let m2065 = v2;
            match m2065 {
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

pub fn f550() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f551() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f552(v0: u64) -> R<Option<u64>> {
    let a2066 = 64u64;
    let a2067 = v0;
    if if nat_lt(a2066, a2067) {
        let a2068 = v0;
        let a2069 = 91u64;
        nat_lt(a2068, a2069)
    } else {
        false
    } {
        Ok(Some({
            let a2070 = v0;
            let a2071 = 65u64;
            nat_sub(a2070, a2071)
        }))
    } else {
        let a2072 = 96u64;
        let a2073 = v0;
        if if nat_lt(a2072, a2073) {
            let a2074 = v0;
            let a2075 = 123u64;
            nat_lt(a2074, a2075)
        } else {
            false
        } {
            Ok(Some({
                let a2076 = v0;
                let a2077 = 71u64;
                nat_sub(a2076, a2077)
            }))
        } else {
            let a2078 = 47u64;
            let a2079 = v0;
            if if nat_lt(a2078, a2079) {
                let a2080 = v0;
                let a2081 = 58u64;
                nat_lt(a2080, a2081)
            } else {
                false
            } {
                Ok(Some({
                    let a2082 = v0;
                    let a2083 = 4u64;
                    nat_add(a2082, a2083)?
                }))
            } else {
                let a2084 = v0;
                let a2085 = 43u64;
                if nat_eq(a2084, a2085) {
                    Ok(Some(62u64))
                } else {
                    let a2086 = v0;
                    let a2087 = 47u64;
                    if nat_eq(a2086, a2087) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f553(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2088 = v3;
        let a2089 = 61u64;
        if nat_eq(a2088, a2089) {
            let a2090 = f634(v1, 16u64);
            let a2091 = 0u64;
            nat_eq(a2090, a2091)
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

pub fn f554(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2092 = f634(v2, 4u64);
        let a2093 = 0u64;
        nat_eq(a2092, a2093)
    } else {
        false
    } {
        Ok(Some(f636(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f555(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2096 = {
            let a2094 = v0;
            let a2095 = 4u64;
            nat_mul(a2094, a2095)?
        };
        let a2097 = f637(v1, 16u64);
        nat_add(a2096, a2097)?
    }, List::cons({
        let a2100 = {
            let a2098 = f634(v1, 16u64);
            let a2099 = 16u64;
            nat_mul(a2098, a2099)?
        };
        let a2101 = f637(v2, 4u64);
        nat_add(a2100, a2101)?
    }, List::cons({
        let a2104 = {
            let a2102 = f634(v2, 4u64);
            let a2103 = 64u64;
            nat_mul(a2102, a2103)?
        };
        let a2105 = v3;
        nat_add(a2104, a2105)?
    }, List::<u64>::nil()))))
}

pub fn f556(v0: Adt6) -> bool {
    let m2106 = f233(v0);
    match m2106 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f557(v0: Adt6) -> bool {
    let m2107 = f575(v0);
    match m2107 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f558(v0: Adt6) -> bool {
    let m2108 = f638(v0);
    match m2108 {
        None => {
            false
        }
        Some(v1) => {
            let a2112 = {
                let m2111 = {
                    let a2109 = v1;
                    let a2110 = 0i64;
                    compare(a2109, a2110)
                };
                match m2111 {
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
            bool_not(a2112)
        }
    }
}

pub fn f559(v0: Adt6) -> bool {
    let m2113 = f409(v0);
    match m2113 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f560(v0: Adt6) -> bool {
    let m2114 = f259(v0);
    match m2114 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f561(v0: Adt6) -> bool {
    let m2115 = f409(v0);
    match m2115 {
        None => {
            false
        }
        Some(v1) => {
            f639(v1)
        }
    }
}

pub fn f562(v0: Adt6) -> bool {
    let m2116 = f258(v0);
    match m2116 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f563(v0: Adt6) -> bool {
    let m2117 = f233(v0.clone());
    match m2117 {
        None => {
            let m2118 = f409(v0);
            match m2118 {
                None => {
                    false
                }
                Some(v2) => {
                    f640(v2)
                }
            }
        }
        Some(v1) => {
            f570(v1)
        }
    }
}

pub fn f564(v0: Adt6) -> bool {
    let m2119 = f409(v0);
    match m2119 {
        None => {
            false
        }
        Some(v1) => {
            let m2120 = v1;
            match m2120.uncons() {
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

pub fn f565(v0: Str, p2138: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2137 = p2138;
    {
        let mut m2137 = m2137;
        loop {
            let step = {
                let v1 = m2137;
                let m2121 = v1;
                match m2121.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2125 = v0.clone();
                        let a2126 = {
                            let a2123 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2122, _) = v2.clone();
                                h2122
                            }, List::<Str>::nil()));
                            let a2124 = Str::lit("");
                            join(a2123, a2124)
                        };
                        if equal(a2125, a2126) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2127) = v2;
                                h2127
                            }))
                        } else {
                            let m2129 = f259({
                                let (_, h2128) = v2.clone();
                                h2128
                            });
                            match m2129 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2130 = f181(Str::lit("$anchor"), v4);
                                    match m2130 {
                                        Adt135::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt135::C1(v5) => {
                                            let m2131 = f233(v5);
                                            match m2131 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2134 = v0.clone();
                                                    let a2135 = {
                                                        let a2132 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2133 = Str::lit("");
                                                        join(a2132, a2133)
                                                    };
                                                    if equal(a2134, a2135) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2136) = v2;
                                                            h2136
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
                Ok(next) => { m2137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f566(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2139 = f236(Str::lit("type"), v0);
    match m2139 {
        Adt135::C0 => {
            None::<Adt13>
        }
        Adt135::C1(v1) => {
            let m2140 = f233(v1);
            match m2140 {
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

pub fn f567(v0: List<u64>) -> R<List<u64>> {
    Ok(f168(f642(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f568(v0: List<u64>, p2143: List<List<u64>>) -> bool {
    let m2142 = p2143;
    {
        let mut m2142 = m2142;
        loop {
            let step = {
                let v1 = m2142;
                let m2141 = v1;
                match m2141.uncons() {
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
                Ok(next) => { m2142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(p2146: List<Str>, p2147: List<Str>) -> List<Str> {
    let m2145 = (p2146, p2147);
    {
        let mut m2145 = m2145;
        loop {
            let step = {
                let (v0, v1) = m2145;
                let m2144 = v0;
                match m2144.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: Str) -> bool {
    let a2148 = v0.clone();
    let a2149 = Str::lit("null");
    if equal(a2148, a2149) {
        true
    } else {
        let a2150 = v0.clone();
        let a2151 = Str::lit("boolean");
        if equal(a2150, a2151) {
            true
        } else {
            let a2152 = v0.clone();
            let a2153 = Str::lit("object");
            if equal(a2152, a2153) {
                true
            } else {
                let a2154 = v0.clone();
                let a2155 = Str::lit("array");
                if equal(a2154, a2155) {
                    true
                } else {
                    let a2156 = v0.clone();
                    let a2157 = Str::lit("number");
                    if equal(a2156, a2157) {
                        true
                    } else {
                        let a2158 = v0.clone();
                        let a2159 = Str::lit("string");
                        if equal(a2158, a2159) {
                            true
                        } else {
                            let a2160 = v0;
                            let a2161 = Str::lit("integer");
                            equal(a2160, a2161)
                        }
                    }
                }
            }
        }
    }
}

pub fn f571(v0: Str, v1: Adt6) -> R<bool> {
    let a2162 = v0.clone();
    let a2163 = f644(v1.clone());
    if equal(a2162, a2163) {
        Ok(true)
    } else {
        let a2164 = v0;
        let a2165 = Str::lit("integer");
        if equal(a2164, a2165) {
            let m2166 = f575(v1);
            match m2166 {
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

pub fn f572(p2169: List<Adt6>, v1: Adt6, p2170: bool) -> R<Adt152> {
    let m2168 = (p2169, p2170);
    {
        let mut m2168 = m2168;
        loop {
            let step = {
                let (v0, v2) = m2168;
                let m2167 = v0;
                match m2167.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f570(f646(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt152>>((v4, if v2 {
                                true
                            } else {
                                f571(f646(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2168 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(p2174: List<Adt6>, v1: Adt6) -> R<Adt152> {
    let m2173 = p2174;
    {
        let mut m2173 = m2173;
        loop {
            let step = {
                let v0 = m2173;
                let m2171 = v0;
                match m2171.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt152>>(Ok(Adt152::C1))
                    }
                    Some((v2, v3)) => {
                        let m2172 = f574(v2, v1.clone())?;
                        match m2172 {
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
                Ok(next) => { m2173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f574(v0: Adt6, v1: Adt6) -> R<Adt154> {
    f647(f648(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f575(v0: Adt6) -> Option<Adt7> {
    let m2175 = v0;
    match m2175 {
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

pub fn f576(v0: u64, v1: Adt155) -> bool {
    let a2176 = v0;
    let a2177 = 0u64;
    if nat_eq(a2176, a2177) {
        let a2178 = f649(v1);
        bool_not(a2178)
    } else {
        let a2179 = v0;
        let a2180 = 1u64;
        if nat_eq(a2179, a2180) {
            let a2181 = f589(v1);
            bool_not(a2181)
        } else {
            let a2182 = v0;
            let a2183 = 2u64;
            if nat_eq(a2182, a2183) {
                f589(v1)
            } else {
                f649(v1)
            }
        }
    }
}

pub fn f577(v0: Adt7, v1: Adt7) -> R<Adt155> {
    f650(v0.clone(), v1.clone(), f651(match v0 {
        Adt7::C0(h2184, _) => {
            h2184
        }
    }), f651(match v1 {
        Adt7::C0(h2185, _) => {
            h2185
        }
    }))
}

pub fn f578(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt152> {
    let m2186 = f236(v0, v2);
    match m2186 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v4) => {
            let m2187 = f638(v4);
            match m2187 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v5) => {
                    let m2190 = {
                        let a2188 = v5;
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
                        Ok(Adt152::C2)
                    } else {
                        if v1 {
                            if f649(f652(f80(v3, 0i64)?, v5)) {
                                Ok(Adt152::C1)
                            } else {
                                Ok(Adt152::C0)
                            }
                        } else {
                            if f589(f652(f80(v3, 0i64)?, v5)) {
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

pub fn f579(v0: Str) -> R<u64> {
    f653(f86({
        let a2191 = v0;
        utf8_encode(a2191)
    })?, 0u64)
}

pub fn f580(p2196: List<Adt6>, p2197: u64) -> R<u64> {
    let m2195 = (p2196, p2197);
    {
        let mut m2195 = m2195;
        loop {
            let step = {
                let (v0, v1) = m2195;
                let m2192 = v0;
                match m2192.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a2193 = v1;
                            let a2194 = 1u64;
                            nat_add(a2193, a2194)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(p2202: List<(Str, Adt6)>, p2203: u64) -> R<u64> {
    let m2201 = (p2202, p2203);
    {
        let mut m2201 = m2201;
        loop {
            let step = {
                let (v0, v1) = m2201;
                let m2198 = v0;
                match m2198.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a2199 = v1;
                            let a2200 = 1u64;
                            nat_add(a2199, a2200)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(p2207: List<Adt6>, v1: Adt5) -> Adt152 {
    let m2206 = p2207;
    {
        let mut m2206 = m2206;
        loop {
            let step = {
                let v0 = m2206;
                let m2204 = v0;
                match m2204.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt152>(Adt152::C0)
                    }
                    Some((v2, v3)) => {
                        let m2205 = f233(v2);
                        match m2205 {
                            None => {
                                Err::<List<Adt6>, Adt152>(Adt152::C2)
                            }
                            Some(v4) => {
                                if f237(v4, v1.clone()) {
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
                Ok(next) => { m2206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(p2214: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt123>, p2215: List<Adt125>) -> List<Adt125> {
    let m2213 = (p2214, p2215);
    {
        let mut m2213 = m2213;
        loop {
            let step = {
                let (v0, v3) = m2213;
                let m2208 = v0;
                match m2208.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2210 = f236({
                            let (h2209, _) = v4.clone();
                            h2209
                        }, v1.clone());
                        match m2210 {
                            Adt135::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v5, v3))
                            }
                            Adt135::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v5, List::cons(Adt125::C0({
                                    let (_, h2211) = v4;
                                    h2211
                                }, v6, List::cons(Adt123::C0({
                                    let (h2212, _) = v4;
                                    h2212
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(p2220: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2219 = p2220;
    {
        let mut m2219 = m2219;
        loop {
            let step = {
                let v0 = m2219;
                let m2216 = v0;
                match m2216.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2218 = f236({
                            let (h2217, _) = v2;
                            h2217
                        }, v1.clone());
                        match m2218 {
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
                Ok(next) => { m2219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f585(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2221 = f236(Str::lit("properties"), v0);
    match m2221 {
        Adt135::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt135::C1(v1) => {
            let m2222 = f259(v1);
            match m2222 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2223) => {
                            h2223
                        }
                    }
                }
            }
        }
    }
}

pub fn f586(p2230: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt123>, p2231: List<Adt125>) -> List<Adt125> {
    let m2229 = (p2230, p2231);
    {
        let mut m2229 = m2229;
        loop {
            let step = {
                let (v0, v4) = m2229;
                let m2224 = v0;
                match m2224.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2226 = f236({
                            let (h2225, _) = v5.clone();
                            h2225
                        }, v1.clone());
                        match m2226 {
                            Adt135::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v6, List::cons(Adt125::C0(v2.clone(), {
                                    let (_, h2227) = v5;
                                    h2227
                                }, List::cons(Adt123::C0({
                                    let (h2228, _) = v5;
                                    h2228
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
                Ok(next) => { m2229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(p2236: List<Adt6>, p2237: u64, v2: Adt6, v3: List<Adt123>, p2238: List<Adt125>) -> R<List<Adt125>> {
    let m2235 = (p2236, (p2237, p2238));
    {
        let mut m2235 = m2235;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2235;
                let m2232 = v0;
                match m2232.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt125>)), R<List<Adt125>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt125>)), R<List<Adt125>>>((v6, ({
                            let a2233 = v1;
                            let a2234 = 1u64;
                            nat_add(a2233, a2234)?
                        }, List::cons(Adt125::C0(v2.clone(), v5, List::cons(Adt123::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(p2241: List<Adt6>, v1: Adt6, v2: List<Adt123>, p2242: List<Adt125>) -> List<Adt125> {
    let m2240 = (p2241, p2242);
    {
        let mut m2240 = m2240;
        loop {
            let step = {
                let (v0, v3) = m2240;
                let m2239 = v0;
                match m2239.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt125>), List<Adt125>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt125>), List<Adt125>>((v5, List::cons(Adt125::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: Adt155) -> bool {
    let m2243 = v0;
    match m2243 {
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

pub fn f590(v0: i64) -> u64 {
    let m2246 = {
        let a2244 = v0;
        let a2245 = 1i64;
        compare(a2244, a2245)
    };
    if match m2246 {
        Ordering::Less => {
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
        let m2249 = {
            let a2247 = v0;
            let a2248 = 2i64;
            compare(a2247, a2248)
        };
        if match m2249 {
            Ordering::Less => {
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
            let m2252 = {
                let a2250 = v0;
                let a2251 = 3i64;
                compare(a2250, a2251)
            };
            if match m2252 {
                Ordering::Less => {
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
                let m2255 = {
                    let a2253 = v0;
                    let a2254 = 4i64;
                    compare(a2253, a2254)
                };
                if match m2255 {
                    Ordering::Less => {
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
                    let m2258 = {
                        let a2256 = v0;
                        let a2257 = 5i64;
                        compare(a2256, a2257)
                    };
                    if match m2258 {
                        Ordering::Less => {
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
                        let m2261 = {
                            let a2259 = v0;
                            let a2260 = 6i64;
                            compare(a2259, a2260)
                        };
                        if match m2261 {
                            Ordering::Less => {
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
                            let m2264 = {
                                let a2262 = v0;
                                let a2263 = 7i64;
                                compare(a2262, a2263)
                            };
                            if match m2264 {
                                Ordering::Less => {
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
                                let m2267 = {
                                    let a2265 = v0;
                                    let a2266 = 8i64;
                                    compare(a2265, a2266)
                                };
                                if match m2267 {
                                    Ordering::Less => {
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
                                    let m2270 = {
                                        let a2268 = v0;
                                        let a2269 = 9i64;
                                        compare(a2268, a2269)
                                    };
                                    if match m2270 {
                                        Ordering::Less => {
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
    Adt6::C5(f424(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2271) => {
            h2271
        }
    }))
}

pub fn f592(p2276: u64, p2277: i64) -> R<i64> {
    let m2275 = (p2276, p2277);
    {
        let mut m2275 = m2275;
        loop {
            let step = {
                let (v0, v1) = m2275;
                let m2272 = v0;
                if m2272 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2272 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2273 = v1;
                        let a2274 = -1i64;
                        int_sub(a2273, a2274)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2275 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: Adt36) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt36::C0(h2278, _, _, _, _, _, _, _) => {
            h2278
        }
    })), {
        let a2296 = f419(Str::lit("title"), match v0.clone() {
            Adt36::C0(_, h2279, _, _, _, _, _, _) => {
                h2279
            }
        });
        let a2297 = {
            let a2294 = f519(match v0.clone() {
                Adt36::C0(_, _, h2280, _, _, _, _, _) => {
                    h2280
                }
            });
            let a2295 = {
                let a2292 = f419(Str::lit("description"), match v0.clone() {
                    Adt36::C0(_, _, _, h2281, _, _, _, _) => {
                        h2281
                    }
                });
                let a2293 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2284 = match v0.clone() {
                        Adt36::C0(_, _, _, _, h2283, _, _, _) => {
                            h2283
                        }
                    };
                    match m2284 {
                        Adt5::C0(h2282) => {
                            h2282
                        }
                    }
                })), {
                    let a2290 = f460(Str::lit("outputSchema"), match v0.clone() {
                        Adt36::C0(_, _, _, _, _, h2285, _, _) => {
                            h2285
                        }
                    });
                    let a2291 = {
                        let a2288 = f654(match v0.clone() {
                            Adt36::C0(_, _, _, _, _, _, h2286, _) => {
                                h2286
                            }
                        });
                        let a2289 = f655(match v0 {
                            Adt36::C0(_, _, _, _, _, _, _, h2287) => {
                                h2287
                            }
                        });
                        append_list(a2288, a2289)
                    };
                    append_list(a2290, a2291)
                });
                append_list(a2292, a2293)
            };
            append_list(a2294, a2295)
        };
        append_list(a2296, a2297)
    }))
}

pub fn f594(v0: Adt39) -> R<Option<Adt6>> {
    let m2298 = v0;
    match m2298 {
        Adt39::C0(v1) => {
            Ok(Some(f656(v1)))
        }
        Adt39::C1(v2) => {
            Ok(Some(f657(Str::lit("image"), match v2.clone() {
                Adt43::C0(h2299, _, _, _) => {
                    h2299
                }
            }, match v2.clone() {
                Adt43::C0(_, h2300, _, _) => {
                    h2300
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h2301, _) => {
                    h2301
                }
            }, match v2 {
                Adt43::C0(_, _, _, h2302) => {
                    h2302
                }
            })))
        }
        Adt39::C2(v3) => {
            Ok(Some(f657(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h2303, _, _, _) => {
                    h2303
                }
            }, match v3.clone() {
                Adt44::C0(_, h2304, _, _) => {
                    h2304
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h2305, _) => {
                    h2305
                }
            }, match v3 {
                Adt44::C0(_, _, _, h2306) => {
                    h2306
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
        let a2330 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2307, _, _, _, _, _, _, _, _) => {
                h2307
            }
        })), List::<(Str, Adt6)>::nil());
        let a2331 = {
            let a2328 = f419(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2308, _, _, _, _, _, _, _) => {
                    h2308
                }
            });
            let a2329 = {
                let a2326 = f519(match v0.clone() {
                    Adt45::C0(_, _, h2309, _, _, _, _, _, _) => {
                        h2309
                    }
                });
                let a2327 = {
                    let a2324 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2310, _, _, _, _, _) => {
                            h2310
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2325 = {
                        let a2322 = f419(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2311, _, _, _, _) => {
                                h2311
                            }
                        });
                        let a2323 = {
                            let a2320 = f419(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2312, _, _, _) => {
                                    h2312
                                }
                            });
                            let a2321 = {
                                let a2318 = f660(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2313, _, _) => {
                                        h2313
                                    }
                                });
                                let a2319 = {
                                    let a2316 = f661(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2314, _) => {
                                            h2314
                                        }
                                    })?;
                                    let a2317 = f655(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2315) => {
                                            h2315
                                        }
                                    });
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
        };
        append_list(a2330, a2331)
    }))
}

pub fn f596(v0: Adt47) -> Adt6 {
    let m2332 = v0;
    match m2332 {
        Adt47::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt48::C0(h2333, _, _, _) => {
                    h2333
                }
            })), {
                let a2337 = f419(Str::lit("mimeType"), match v1.clone() {
                    Adt48::C0(_, h2334, _, _) => {
                        h2334
                    }
                });
                let a2338 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt48::C0(_, _, h2335, _) => {
                        h2335
                    }
                })), f655(match v1 {
                    Adt48::C0(_, _, _, h2336) => {
                        h2336
                    }
                }));
                append_list(a2337, a2338)
            }))
        }
        Adt47::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt49::C0(h2339, _, _, _) => {
                    h2339
                }
            })), {
                let a2343 = f419(Str::lit("mimeType"), match v2.clone() {
                    Adt49::C0(_, h2340, _, _) => {
                        h2340
                    }
                });
                let a2344 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt49::C0(_, _, h2341, _) => {
                        h2341
                    }
                })), f655(match v2 {
                    Adt49::C0(_, _, _, h2342) => {
                        h2342
                    }
                }));
                append_list(a2343, a2344)
            }))
        }
    }
}

pub fn f597(v0: Adt55) -> Adt6 {
    Adt6::C5({
        let a2365 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt55::C0(h2345, _, _, _, _, _, _, _) => {
                h2345
            }
        })), List::<(Str, Adt6)>::nil());
        let a2366 = {
            let a2363 = f419(Str::lit("title"), match v0.clone() {
                Adt55::C0(_, h2346, _, _, _, _, _, _) => {
                    h2346
                }
            });
            let a2364 = {
                let a2361 = f519(match v0.clone() {
                    Adt55::C0(_, _, h2347, _, _, _, _, _) => {
                        h2347
                    }
                });
                let a2362 = {
                    let a2359 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt55::C0(_, _, _, h2348, _, _, _, _) => {
                            h2348
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2360 = {
                        let a2357 = f419(Str::lit("description"), match v0.clone() {
                            Adt55::C0(_, _, _, _, h2349, _, _, _) => {
                                h2349
                            }
                        });
                        let a2358 = {
                            let a2355 = f419(Str::lit("mimeType"), match v0.clone() {
                                Adt55::C0(_, _, _, _, _, h2350, _, _) => {
                                    h2350
                                }
                            });
                            let a2356 = {
                                let a2353 = f660(match v0.clone() {
                                    Adt55::C0(_, _, _, _, _, _, h2351, _) => {
                                        h2351
                                    }
                                });
                                let a2354 = f655(match v0 {
                                    Adt55::C0(_, _, _, _, _, _, _, h2352) => {
                                        h2352
                                    }
                                });
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
        };
        append_list(a2365, a2366)
    })
}

pub fn f598(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2382 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2367, _, _, _, _, _) => {
                h2367
            }
        })), List::<(Str, Adt6)>::nil());
        let a2383 = {
            let a2380 = f419(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2368, _, _, _, _) => {
                    h2368
                }
            });
            let a2381 = {
                let a2378 = f519(match v0.clone() {
                    Adt57::C0(_, _, h2369, _, _, _) => {
                        h2369
                    }
                });
                let a2379 = {
                    let a2376 = f419(Str::lit("description"), match v0.clone() {
                        Adt57::C0(_, _, _, h2370, _, _) => {
                            h2370
                        }
                    });
                    let a2377 = {
                        let a2374 = {
                            let m2372 = match v0.clone() {
                                Adt57::C0(_, _, _, _, h2371, _) => {
                                    h2371
                                }
                            };
                            match m2372 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f662(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2375 = f655(match v0 {
                            Adt57::C0(_, _, _, _, _, h2373) => {
                                h2373
                            }
                        });
                        append_list(a2374, a2375)
                    };
                    append_list(a2376, a2377)
                };
                append_list(a2378, a2379)
            };
            append_list(a2380, a2381)
        };
        append_list(a2382, a2383)
    })
}

pub fn f599(v0: Adt60) -> R<Option<Adt6>> {
    let m2385 = f594(match v0.clone() {
        Adt60::C0(_, h2384) => {
            h2384
        }
    })?;
    match m2385 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f663(match v0 {
                Adt60::C0(h2386, _) => {
                    h2386
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f600(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f423(Str::lit("params"), v1)))
}

pub fn f601(v0: Adt66) -> Adt6 {
    let m2387 = v0;
    match m2387 {
        Adt66::C0(v1) => {
            f664(v1)
        }
        Adt66::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt69::C0(h2388, _) => {
                    h2388
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt69::C0(_, h2389) => {
                    h2389
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f602(v0: Adt70) -> R<Option<Adt6>> {
    let m2391 = f665(match v0.clone() {
        Adt70::C0(h2390, _, _, _, _, _, _, _, _, _) => {
            h2390
        }
    })?;
    match m2391 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2393 = f666(v0.clone());
                let a2394 = List::cons((Str::lit("maxTokens"), f505(match v0.clone() {
                    Adt70::C0(_, _, _, _, _, h2392, _, _, _, _) => {
                        h2392
                    }
                })?), f667(v0));
                append_list(a2393, a2394)
            }))))
        }
    }
}

pub fn f603(p2400: List<(Str, Adt65)>, p2401: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m2399 = (p2400, p2401);
    {
        let mut m2399 = m2399;
        loop {
            let step = {
                let (v0, v1) = m2399;
                let m2395 = v0;
                match m2395.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2397 = f668({
                            let (_, h2396) = v2;
                            h2396
                        });
                        match m2397 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h2398, _) = v2;
                                    h2398
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f605(v0: List<Adt2>) -> List<Adt6> {
    f303(f669(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f606(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m2402 = f181(v0, v1);
    match m2402 {
        Adt135::C0 => {
            None::<Adt5>
        }
        Adt135::C1(v2) => {
            f259(v2)
        }
    }
}

pub fn f607(v0: Adt5) -> Option<Adt63> {
    let m2403 = f670(v0.clone());
    match m2403 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m2404 = f671(Str::lit("status"), v0.clone());
            match m2404 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let a2405 = v2.clone();
                    let a2406 = Str::lit("working");
                    if equal(a2405, a2406) {
                        Some(Adt63::C0(v1))
                    } else {
                        let a2407 = v2.clone();
                        let a2408 = Str::lit("cancelled");
                        if equal(a2407, a2408) {
                            Some(Adt63::C4(v1))
                        } else {
                            let a2409 = v2.clone();
                            let a2410 = Str::lit("input_required");
                            if equal(a2409, a2410) {
                                let m2411 = f672(Str::lit("inputRequests"), v0);
                                match m2411 {
                                    None => {
                                        None::<Adt63>
                                    }
                                    Some(v3) => {
                                        Some(Adt63::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a2412 = v2.clone();
                                let a2413 = Str::lit("completed");
                                if equal(a2412, a2413) {
                                    let m2414 = f606(Str::lit("result"), v0);
                                    match m2414 {
                                        None => {
                                            None::<Adt63>
                                        }
                                        Some(v4) => {
                                            Some(Adt63::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a2415 = v2;
                                    let a2416 = Str::lit("failed");
                                    if equal(a2415, a2416) {
                                        let m2417 = f606(Str::lit("error"), v0);
                                        match m2417 {
                                            None => {
                                                None::<Adt63>
                                            }
                                            Some(v5) => {
                                                let m2418 = f673(v5);
                                                match m2418 {
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
    let m2419 = f181(v0, v1);
    match m2419 {
        Adt135::C0 => {
            None::<i64>
        }
        Adt135::C1(v2) => {
            f638(v2)
        }
    }
}

pub fn f609(v0: Adt5) -> Option<Adt86> {
    let m2420 = f608(Str::lit("parkAfterMs"), v0.clone());
    match m2420 {
        None => {
            None::<Adt86>
        }
        Some(v1) => {
            let m2421 = f672(Str::lit("parked"), v0.clone());
            match m2421 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m2422 = f608(Str::lit("finishAfterMs"), v0.clone());
                    match m2422 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            let m2423 = f606(Str::lit("outcome"), v0);
                            match m2423 {
                                None => {
                                    None::<Adt86>
                                }
                                Some(v4) => {
                                    let m2424 = f674(v4);
                                    match m2424 {
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
    let m2426 = f356(match v0.clone() {
        Adt85::C0(h2425, _, _) => {
            h2425
        }
    });
    match m2426 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m2428 = f675(match v0.clone() {
                Adt85::C0(_, _, h2427) => {
                    h2427
                }
            });
            match m2428 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f604(match v0 {
                        Adt85::C0(_, h2429, _) => {
                            h2429
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f611(v0: List<u64>) -> List<u64> {
    let a2432 = f676();
    let a2433 = {
        let a2430 = v0;
        let a2431 = f677();
        append_list(a2430, a2431)
    };
    append_list(a2432, a2433)
}

pub fn f612(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f678(v0, 0u64, v1)
}

pub fn f613(p2440: List<u64>) -> bool {
    let m2439 = p2440;
    {
        let mut m2439 = m2439;
        loop {
            let step = {
                let v0 = m2439;
                let m2434 = v0;
                match m2434.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2435 = v1;
                        let a2436 = 48u64;
                        if if nat_eq(a2435, a2436) {
                            true
                        } else {
                            let a2437 = v1;
                            let a2438 = 46u64;
                            nat_eq(a2437, a2438)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: Adt140) -> bool {
    let m2441 = v0;
    match m2441 {
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
    let m2442 = v0;
    match m2442 {
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
        Adt0::C0(h2443, _, _, _, _, _, _, _) => {
            h2443
        }
    }), None::<Adt5>), List::<Adt130>::nil())
}

pub fn f617(v0: Adt0) -> Adt34 {
    Adt34::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2444, _, _, _, _) => {
            h2444
        }
    }, f179(), match v0.clone() {
        Adt0::C0(_, _, h2445, _, _, _, _, _) => {
            h2445
        }
    }, match v0 {
        Adt0::C0(_, h2446, _, _, _, _, _, _) => {
            h2446
        }
    })
}

pub fn f618(v0: Adt0, v1: Adt15, v2: Result<Adt33, Adt90>) -> Result<Adt128, Adt131> {
    let m2447 = v2;
    match m2447 {
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
    let m2448 = f181(v0.clone(), v1);
    match m2448 {
        Adt135::C0 => {
            Ok::<Option<Str>, Adt132>(None::<Str>)
        }
        Adt135::C1(v2) => {
            let m2449 = f233(v2);
            match m2449 {
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
    let m2450 = f181(v0.clone(), v1);
    match m2450 {
        Adt135::C0 => {
            Err::<Str, Adt132>(Adt132::C7(v0))
        }
        Adt135::C1(v2) => {
            let m2451 = f233(v2);
            match m2451 {
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
    let m2452 = f181(v0.clone(), v1);
    match m2452 {
        Adt135::C0 => {
            Ok::<Option<Adt5>, Adt132>(None::<Adt5>)
        }
        Adt135::C1(v2) => {
            let m2453 = f259(v2);
            match m2453 {
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
    let m2454 = f181(v0.clone(), v1);
    match m2454 {
        Adt135::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt132>(None::<List<(Str, Str)>>)
        }
        Adt135::C1(v2) => {
            let m2455 = f259(v2);
            match m2455 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    let m2457 = f684(v0, match v3 {
                        Adt5::C0(h2456) => {
                            h2456
                        }
                    });
                    match m2457 {
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
    let m2458 = f181(v0.clone(), v1);
    match m2458 {
        Adt135::C0 => {
            Err::<Adt5, Adt132>(Adt132::C7(v0))
        }
        Adt135::C1(v2) => {
            let m2459 = f259(v2);
            match m2459 {
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
    let m2460 = f621(Str::lit("type"), v0.clone());
    match m2460 {
        Ok(v1) => {
            let a2461 = v1.clone();
            let a2462 = Str::lit("ref/prompt");
            if equal(a2461, a2462) {
                let m2463 = f621(Str::lit("name"), v0.clone());
                match m2463 {
                    Ok(v2) => {
                        let m2464 = f620(Str::lit("title"), v0);
                        match m2464 {
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
                let a2465 = v1;
                let a2466 = Str::lit("ref/resource");
                if equal(a2465, a2466) {
                    let m2467 = f621(Str::lit("uri"), v0);
                    match m2467 {
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
    let m2468 = f621(Str::lit("name"), v0.clone());
    match m2468 {
        Ok(v1) => {
            let m2469 = f621(Str::lit("value"), v0);
            match m2469 {
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
    let m2470 = f181(Str::lit("context"), v0);
    match m2470 {
        Adt135::C0 => {
            Ok::<Option<Adt29>, Adt132>(None::<Adt29>)
        }
        Adt135::C1(v1) => {
            let m2471 = f259(v1);
            match m2471 {
                None => {
                    Err::<Option<Adt29>, Adt132>(Adt132::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2472 = f623(Str::lit("arguments"), v2);
                    match m2472 {
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

pub fn f629(v0: Str, p2477: List<Str>) -> bool {
    let m2476 = p2477;
    {
        let mut m2476 = m2476;
        loop {
            let step = {
                let v1 = m2476;
                let m2473 = v1;
                match m2473.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2474 = v2;
                        let a2475 = v0.clone();
                        if equal(a2474, a2475) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2476 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f630(v0: Option<Adt109>, v1: List<Adt111>, v2: Adt5) -> R<Option<Adt109>> {
    let m2478 = v0;
    match m2478 {
        None => {
            f685(v1, f686(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f631(v0: Option<Str>, v1: Adt146, v2: Adt5) -> Option<Adt109> {
    let m2479 = v1.clone();
    match m2479 {
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
    let m2480 = v0;
    match m2480 {
        Adt135::C0 => {
            Adt153::C1
        }
        Adt135::C1(_) => {
            let m2481 = v1;
            match m2481 {
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
    let m2482 = f233(v1);
    match m2482 {
        None => {
            Err::<Adt17, Adt132>(Adt132::C5)
        }
        Some(v3) => {
            let m2483 = f689(v2);
            match m2483 {
                None => {
                    Err::<Adt17, Adt132>(Adt132::C6)
                }
                Some(v4) => {
                    let m2484 = f622(f690(), v0.clone());
                    match m2484 {
                        Ok(v6) => {
                            let m2485 = f691(v0.clone());
                            match m2485 {
                                Ok(v8) => {
                                    let m2486 = f692(v0.clone());
                                    match m2486 {
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
    let a2487 = v0;
    let a2488 = v1;
    let a2489 = 0u64;
    nat_rem(a2487, a2488, a2489)
}

pub fn f635(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2492 = {
            let a2490 = v0;
            let a2491 = 4u64;
            nat_mul(a2490, a2491)?
        };
        let a2493 = f637(v1, 16u64);
        nat_add(a2492, a2493)?
    }, List::<u64>::nil()))
}

pub fn f636(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2496 = {
            let a2494 = v0;
            let a2495 = 4u64;
            nat_mul(a2494, a2495)?
        };
        let a2497 = f637(v1, 16u64);
        nat_add(a2496, a2497)?
    }, List::cons({
        let a2500 = {
            let a2498 = f634(v1, 16u64);
            let a2499 = 16u64;
            nat_mul(a2498, a2499)?
        };
        let a2501 = f637(v2, 4u64);
        nat_add(a2500, a2501)?
    }, List::<u64>::nil())))
}

pub fn f637(v0: u64, v1: u64) -> u64 {
    let a2502 = v0;
    let a2503 = v1;
    let a2504 = 0u64;
    nat_quot(a2502, a2503, a2504)
}

pub fn f638(v0: Adt6) -> Option<i64> {
    let m2505 = v0;
    match m2505 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f394(match v2.clone() {
                Adt7::C0(_, h2506) => {
                    h2506
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2507, _) => {
                        h2507
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

pub fn f639(p2511: List<Adt6>) -> bool {
    let m2510 = p2511;
    {
        let mut m2510 = m2510;
        loop {
            let step = {
                let v0 = m2510;
                let m2508 = v0;
                match m2508.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2509 = f233(v1);
                        match m2509 {
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
                Ok(next) => { m2510 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(p2515: List<Adt6>) -> bool {
    let m2514 = p2515;
    {
        let mut m2514 = m2514;
        loop {
            let step = {
                let v0 = m2514;
                let m2512 = v0;
                match m2512.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2513 = f233(v1);
                        match m2513 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f570(v3) {
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
                Ok(next) => { m2514 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f641(v0: Str) -> Option<Adt13> {
    let a2516 = v0.clone();
    let a2517 = Str::lit("string");
    if equal(a2516, a2517) {
        Some(Adt13::C0)
    } else {
        let a2518 = v0.clone();
        let a2519 = Str::lit("integer");
        if equal(a2518, a2519) {
            Some(Adt13::C1)
        } else {
            let a2520 = v0;
            let a2521 = Str::lit("boolean");
            if equal(a2520, a2521) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f642(p2530: List<u64>, p2531: List<u64>) -> R<List<u64>> {
    let m2529 = (p2530, p2531);
    {
        let mut m2529 = m2529;
        loop {
            let step = {
                let (v0, v1) = m2529;
                let m2522 = v0;
                match m2522.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2523 = 65u64;
                        let a2524 = v2;
                        if if nat_le(a2523, a2524) {
                            let a2525 = v2;
                            let a2526 = 90u64;
                            nat_le(a2525, a2526)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2527 = v2;
                                let a2528 = 32u64;
                                nat_add(a2527, a2528)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2529 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(p2538: List<u64>, p2539: List<u64>) -> bool {
    let m2537 = (p2538, p2539);
    {
        let mut m2537 = m2537;
        loop {
            let step = {
                let (v0, v1) = m2537;
                let m2532 = v0;
                match m2532.uncons() {
                    None => {
                        let m2533 = v1;
                        match m2533.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2534 = v1;
                        match m2534.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2535 = v4;
                                let a2536 = v6;
                                if nat_eq(a2535, a2536) {
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
                Ok(next) => { m2537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: Adt6) -> Str {
    let m2540 = v0;
    match m2540 {
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
    if f394(match v0.clone() {
        Adt7::C0(h2541, _) => {
            h2541
        }
    }) {
        Ok(true)
    } else {
        f693(40u64, match v0.clone() {
            Adt7::C0(h2542, _) => {
                h2542
            }
        }, match v0 {
            Adt7::C0(_, h2543) => {
                h2543
            }
        })
    }
}

pub fn f646(v0: Adt6) -> Str {
    let m2544 = f233(v0);
    match m2544 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f647(p2551: u64, p2552: List<(Adt6, Adt6)>) -> R<Adt154> {
    let m2550 = (p2551, p2552);
    {
        let mut m2550 = m2550;
        loop {
            let step = {
                let (v0, v1) = m2550;
                let m2545 = v0;
                if m2545 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt154>>(Ok(Adt154::C2))
                } else {
                    let v2 = m2545 - 1;
                    let m2546 = v1;
                    match m2546.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt154>>(Ok(Adt154::C0))
                        }
                        Some((v3, v4)) => {
                            let m2549 = f694({
                                let (h2547, _) = v3;
                                h2547
                            }, {
                                let (_, h2548) = v3;
                                h2548
                            }, v4)?;
                            match m2549 {
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
                Ok(next) => { m2550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648() -> u64 {
    67108864u64
}

pub fn f649(v0: Adt155) -> bool {
    let m2553 = v0;
    match m2553 {
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
    let m2554 = v2;
    match m2554 {
        Adt155::C0 => {
            let m2556 = v3;
            match m2556 {
                Adt155::C0 => {
                    Ok(f695(Adt155::C0, f696(f697(match v0.clone() {
                        Adt7::C0(h2557, _) => {
                            h2557
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2558) => {
                            h2558
                        }
                    }, f697(match v1.clone() {
                        Adt7::C0(h2559, _) => {
                            h2559
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2560) => {
                            h2560
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
            let m2555 = v3;
            match m2555 {
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
            let m2561 = v3;
            match m2561 {
                Adt155::C0 => {
                    Ok(Adt155::C2)
                }
                Adt155::C1 => {
                    Ok(Adt155::C2)
                }
                Adt155::C2 => {
                    f696(match v0.clone() {
                        Adt7::C0(h2562, _) => {
                            h2562
                        }
                    }, match v0 {
                        Adt7::C0(_, h2563) => {
                            h2563
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2564, _) => {
                            h2564
                        }
                    }, match v1 {
                        Adt7::C0(_, h2565) => {
                            h2565
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
    let m2568 = {
        let a2566 = v0;
        let a2567 = v1;
        compare(a2566, a2567)
    };
    if match m2568 {
        Ordering::Less => {
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
        let m2571 = {
            let a2569 = v1;
            let a2570 = v0;
            compare(a2569, a2570)
        };
        if match m2571 {
            Ordering::Less => {
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

pub fn f653(p2580: List<u64>, p2581: u64) -> R<u64> {
    let m2579 = (p2580, p2581);
    {
        let mut m2579 = m2579;
        loop {
            let step = {
                let (v0, v1) = m2579;
                let m2572 = v0;
                match m2572.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2573 = v2;
                        let a2574 = 128u64;
                        if if nat_lt(a2573, a2574) {
                            true
                        } else {
                            let a2575 = 191u64;
                            let a2576 = v2;
                            nat_lt(a2575, a2576)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2577 = v1;
                                let a2578 = 1u64;
                                nat_add(a2577, a2578)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
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

pub fn f654(v0: Option<Adt37>) -> List<(Str, Adt6)> {
    let m2582 = v0;
    match m2582 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2594 = f419(Str::lit("title"), match v1.clone() {
                    Adt37::C0(h2583, _, _, _, _) => {
                        h2583
                    }
                });
                let a2595 = {
                    let a2592 = f424(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt37::C0(_, h2584, _, _, _) => {
                            h2584
                        }
                    });
                    let a2593 = {
                        let a2590 = f424(Str::lit("destructiveHint"), match v1.clone() {
                            Adt37::C0(_, _, h2585, _, _) => {
                                h2585
                            }
                        });
                        let a2591 = {
                            let a2588 = f424(Str::lit("idempotentHint"), match v1.clone() {
                                Adt37::C0(_, _, _, h2586, _) => {
                                    h2586
                                }
                            });
                            let a2589 = f424(Str::lit("openWorldHint"), match v1 {
                                Adt37::C0(_, _, _, _, h2587) => {
                                    h2587
                                }
                            });
                            append_list(a2588, a2589)
                        };
                        append_list(a2590, a2591)
                    };
                    append_list(a2592, a2593)
                };
                append_list(a2594, a2595)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f655(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f460(Str::lit("_meta"), v0)
}

pub fn f656(v0: Adt40) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt40::C0(h2596, _, _) => {
            h2596
        }
    })), {
        let a2599 = f660(match v0.clone() {
            Adt40::C0(_, h2597, _) => {
                h2597
            }
        });
        let a2600 = f655(match v0 {
            Adt40::C0(_, _, h2598) => {
                h2598
            }
        });
        append_list(a2599, a2600)
    })))
}

pub fn f657(v0: Str, v1: Str, v2: Str, v3: Option<Adt41>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2601 = f660(v3);
        let a2602 = f655(v4);
        append_list(a2601, a2602)
    }))))
}

pub fn f658(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2626 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2603, _, _, _, _, _, _, _, _) => {
                h2603
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2627 = {
            let a2624 = f419(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2604, _, _, _, _, _, _, _) => {
                    h2604
                }
            });
            let a2625 = {
                let a2622 = f519(match v0.clone() {
                    Adt45::C0(_, _, h2605, _, _, _, _, _, _) => {
                        h2605
                    }
                });
                let a2623 = {
                    let a2620 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2606, _, _, _, _, _) => {
                            h2606
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2621 = {
                        let a2618 = f419(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2607, _, _, _, _) => {
                                h2607
                            }
                        });
                        let a2619 = {
                            let a2616 = f419(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2608, _, _, _) => {
                                    h2608
                                }
                            });
                            let a2617 = {
                                let a2614 = f660(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2609, _, _) => {
                                        h2609
                                    }
                                });
                                let a2615 = {
                                    let a2612 = f661(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2610, _) => {
                                            h2610
                                        }
                                    })?;
                                    let a2613 = f655(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2611) => {
                                            h2611
                                        }
                                    });
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
        };
        append_list(a2626, a2627)
    }))
}

pub fn f659(v0: Adt46) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f596(match v0.clone() {
        Adt46::C0(h2628, _, _) => {
            h2628
        }
    })), {
        let a2631 = f660(match v0.clone() {
            Adt46::C0(_, h2629, _) => {
                h2629
            }
        });
        let a2632 = f655(match v0 {
            Adt46::C0(_, _, h2630) => {
                h2630
            }
        });
        append_list(a2631, a2632)
    })))
}

pub fn f660(v0: Option<Adt41>) -> List<(Str, Adt6)> {
    let m2633 = v0;
    match m2633 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2639 = f698(match v1.clone() {
                    Adt41::C0(h2634, _, _) => {
                        h2634
                    }
                });
                let a2640 = {
                    let a2637 = f699(match v1.clone() {
                        Adt41::C0(_, h2635, _) => {
                            h2635
                        }
                    });
                    let a2638 = f419(Str::lit("lastModified"), match v1 {
                        Adt41::C0(_, _, h2636) => {
                            h2636
                        }
                    });
                    append_list(a2637, a2638)
                };
                append_list(a2639, a2640)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f661(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2641 = v0;
    match m2641 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f505(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f662(v0: List<Adt58>) -> List<Adt6> {
    f303(f700(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f663(v0: Adt42) -> Str {
    let m2642 = v0;
    match m2642 {
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
        let a2649 = {
            let m2644 = match v0.clone() {
                Adt67::C0(h2643, _, _) => {
                    h2643
                }
            };
            match m2644 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2650 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt67::C0(_, h2645, _) => {
                h2645
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2648 = match v0 {
                Adt67::C0(_, _, h2647) => {
                    h2647
                }
            };
            match m2648 {
                Adt5::C0(h2646) => {
                    h2646
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2649, a2650)
    })
}

pub fn f665(v0: List<Adt71>) -> R<Option<List<Adt6>>> {
    let m2651 = f701(v0, List::<Adt6>::nil())?;
    match m2651 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f303(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f666(v0: Adt70) -> List<(Str, Adt6)> {
    let a2662 = {
        let m2653 = match v0.clone() {
            Adt70::C0(_, h2652, _, _, _, _, _, _, _, _) => {
                h2652
            }
        };
        match m2653 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f702(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2663 = {
        let a2660 = f419(Str::lit("systemPrompt"), match v0.clone() {
            Adt70::C0(_, _, h2654, _, _, _, _, _, _, _) => {
                h2654
            }
        });
        let a2661 = {
            let a2658 = {
                let m2656 = match v0.clone() {
                    Adt70::C0(_, _, _, h2655, _, _, _, _, _, _) => {
                        h2655
                    }
                };
                match m2656 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f703(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2659 = f704(Str::lit("temperature"), match v0 {
                Adt70::C0(_, _, _, _, h2657, _, _, _, _, _) => {
                    h2657
                }
            });
            append_list(a2658, a2659)
        };
        append_list(a2660, a2661)
    };
    append_list(a2662, a2663)
}

pub fn f667(v0: Adt70) -> List<(Str, Adt6)> {
    let a2675 = {
        let m2665 = match v0.clone() {
            Adt70::C0(_, _, _, _, _, _, h2664, _, _, _) => {
                h2664
            }
        };
        match m2665 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f417(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2676 = {
        let a2673 = f460(Str::lit("metadata"), match v0.clone() {
            Adt70::C0(_, _, _, _, _, _, _, h2666, _, _) => {
                h2666
            }
        });
        let a2674 = {
            let a2671 = {
                let m2668 = match v0.clone() {
                    Adt70::C0(_, _, _, _, _, _, _, _, h2667, _) => {
                        h2667
                    }
                };
                match m2668 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f421(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2672 = {
                let m2670 = match v0 {
                    Adt70::C0(_, _, _, _, _, _, _, _, _, h2669) => {
                        h2669
                    }
                };
                match m2670 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f705(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2671, a2672)
        };
        append_list(a2673, a2674)
    };
    append_list(a2675, a2676)
}

pub fn f668(v0: Adt65) -> Option<Adt6> {
    let m2677 = v0;
    match m2677 {
        Adt65::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f706(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt65::C1(_) => {
            None::<Adt6>
        }
        Adt65::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m2678 = v3;
                match m2678 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h2679) => {
                                h2679
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f669(p2682: List<Adt2>, p2683: List<Adt6>) -> List<Adt6> {
    let m2681 = (p2682, p2683);
    {
        let mut m2681 = m2681;
        loop {
            let step = {
                let (v0, v1) = m2681;
                let m2680 = v0;
                match m2680.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f707(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2681 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f670(v0: Adt5) -> Option<Adt64> {
    let m2684 = f671(Str::lit("taskId"), v0.clone());
    match m2684 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m2685 = f671(Str::lit("createdAt"), v0.clone());
            match m2685 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let m2686 = f671(Str::lit("lastUpdatedAt"), v0.clone());
                    match m2686 {
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
    let m2687 = f181(v0, v1);
    match m2687 {
        Adt135::C0 => {
            None::<Str>
        }
        Adt135::C1(v2) => {
            f233(v2)
        }
    }
}

pub fn f672(v0: Str, v1: Adt5) -> Option<List<(Str, Adt65)>> {
    let m2688 = f606(v0, v1);
    match m2688 {
        None => {
            None::<List<(Str, Adt65)>>
        }
        Some(v2) => {
            f708(match v2 {
                Adt5::C0(h2689) => {
                    h2689
                }
            }, List::<(Str, Adt65)>::nil())
        }
    }
}

pub fn f673(v0: Adt5) -> Option<Adt81> {
    let m2690 = f608(Str::lit("code"), v0.clone());
    match m2690 {
        None => {
            None::<Adt81>
        }
        Some(v1) => {
            let m2691 = f671(Str::lit("message"), v0.clone());
            match m2691 {
                None => {
                    None::<Adt81>
                }
                Some(v2) => {
                    Some(Adt81::C0(v1, v2, {
                        let m2692 = f181(Str::lit("data"), v0);
                        match m2692 {
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
    let m2693 = f606(Str::lit("completes"), v0.clone());
    match m2693 {
        None => {
            let m2694 = f606(Str::lit("fails"), v0);
            match m2694 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m2695 = f673(v2);
                    match m2695 {
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
    let m2697 = f515(match v0.clone() {
        Adt86::C0(_, h2696, _, _) => {
            h2696
        }
    });
    match m2697 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f604(match v0.clone() {
                Adt86::C0(h2698, _, _, _) => {
                    h2698
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f604(match v0.clone() {
                Adt86::C0(_, _, h2699, _) => {
                    h2699
                }
            })), List::cons((Str::lit("outcome"), f709(match v0 {
                Adt86::C0(_, _, _, h2700) => {
                    h2700
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

pub fn f678(p2705: List<u64>, p2706: u64, p2707: List<u64>) -> List<u64> {
    let m2704 = (p2705, (p2706, p2707));
    {
        let mut m2704 = m2704;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2704;
                let m2701 = v0;
                match m2701.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f710(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt156 = f711(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt156::C0(h2702, _) => {
                                h2702
                            }
                        }, match v5 {
                            Adt156::C0(_, h2703) => {
                                h2703
                            }
                        })))
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

pub fn f679(v0: Adt140, v1: Adt33) -> bool {
    let m2708 = v1;
    match m2708 {
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
            let m2710 = match v15 {
                Adt88::C0(_, h2709, _) => {
                    h2709
                }
            };
            match m2710 {
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
    let m2711 = v2.clone();
    match m2711 {
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
    let m2712 = v5;
    match m2712 {
        Ok(v7) => {
            f720(v0, v4, f721(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt33, Adt90>(v6))
        }
    }
}

pub fn f683(v0: Bytes, v1: Adt15) -> R<Result<Adt15, Adt90>> {
    let m2713 = v1.clone();
    match m2713 {
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
    let m2714 = f725(v0, v1, List::<(Str, Str)>::nil());
    match m2714 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt132>(f726(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt132>(v3)
        }
    }
}

pub fn f685(p2718: List<Adt111>, v1: Adt5) -> R<Option<Adt109>> {
    let m2717 = p2718;
    {
        let mut m2717 = m2717;
        loop {
            let step = {
                let v0 = m2717;
                let m2715 = v0;
                match m2715.uncons() {
                    None => {
                        Err::<List<Adt111>, R<Option<Adt109>>>(Ok(None::<Adt109>))
                    }
                    Some((v2, v3)) => {
                        let m2716 = f727(v2, v1.clone())?;
                        match m2716 {
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
                Ok(next) => { m2717 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Adt5) -> Adt5 {
    let m2719 = f181(Str::lit("arguments"), v0);
    match m2719 {
        Adt135::C0 => {
            f547()
        }
        Adt135::C1(v1) => {
            let m2720 = f259(v1);
            match m2720 {
                None => {
                    f547()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f687(v0: Option<Str>, v1: Option<Str>) -> Option<Adt109> {
    let m2721 = v0;
    match m2721 {
        None => {
            Some(Adt109::C2)
        }
        Some(v2) => {
            let m2722 = v1;
            match m2722 {
                None => {
                    None::<Adt109>
                }
                Some(v3) => {
                    let a2723 = v2;
                    let a2724 = v3;
                    if equal(a2723, a2724) {
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
    let m2725 = v0;
    match m2725 {
        Adt146::C0 => {
            None::<Str>
        }
        Adt146::C1 => {
            let m2726 = f181(Str::lit("name"), v1);
            match m2726 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v2) => {
                    f233(v2)
                }
            }
        }
        Adt146::C2 => {
            let m2727 = f181(Str::lit("uri"), v1);
            match m2727 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v3) => {
                    f233(v3)
                }
            }
        }
        Adt146::C3 => {
            let m2728 = f181(Str::lit("taskId"), v1);
            match m2728 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v4) => {
                    f233(v4)
                }
            }
        }
    }
}

pub fn f689(v0: Adt6) -> Option<Adt18> {
    let m2729 = f259(v0);
    match m2729 {
        None => {
            None::<Adt18>
        }
        Some(v1) => {
            let m2730 = f728(Str::lit("experimental"), v1.clone());
            match m2730 {
                None => {
                    None::<Adt18>
                }
                Some(v2) => {
                    let m2731 = f728(Str::lit("roots"), v1.clone());
                    match m2731 {
                        None => {
                            None::<Adt18>
                        }
                        Some(v3) => {
                            let m2732 = f728(Str::lit("sampling"), v1.clone());
                            match m2732 {
                                None => {
                                    None::<Adt18>
                                }
                                Some(v4) => {
                                    let m2733 = f728(Str::lit("elicitation"), v1.clone());
                                    match m2733 {
                                        None => {
                                            None::<Adt18>
                                        }
                                        Some(v5) => {
                                            let m2734 = f728(Str::lit("extensions"), v1);
                                            match m2734 {
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
    let m2735 = f620(f729(), v0);
    match m2735 {
        Ok(v2) => {
            let m2736 = v2;
            match m2736 {
                None => {
                    Ok::<Option<Adt19>, Adt132>(None::<Adt19>)
                }
                Some(v3) => {
                    let m2737 = f730(v3);
                    match m2737 {
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
    let m2738 = f181(f731(), v0);
    match m2738 {
        Adt135::C0 => {
            Ok::<Option<Adt20>, Adt132>(None::<Adt20>)
        }
        Adt135::C1(v1) => {
            let m2739 = f233(v1.clone());
            match m2739 {
                None => {
                    let m2740 = f638(v1);
                    match m2740 {
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

pub fn f693(p2752: u64, p2753: i64, p2754: i64) -> R<bool> {
    let m2751 = (p2752, (p2753, p2754));
    {
        let mut m2751 = m2751;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2751;
                let m2741 = v0;
                if m2741 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2741 - 1;
                    let m2744 = {
                        let a2742 = v2;
                        let a2743 = 0i64;
                        compare(a2742, a2743)
                    };
                    if match m2744 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f394({
                            let a2745 = v1;
                            let a2746 = 10i64;
                            let a2747 = 0i64;
                            int_rem(a2745, a2746, a2747)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2748 = v1;
                                let a2749 = 10i64;
                                let a2750 = 0i64;
                                int_quot(a2748, a2749, a2750)?
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
                Ok(next) => { m2751 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f694(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt157> {
    let a2755 = f732(v0.clone());
    let a2756 = f732(v1.clone());
    if nat_eq(a2755, a2756) {
        let m2757 = v0;
        match m2757 {
            Adt6::C0 => {
                Ok(Adt157::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2758 = f258(v1);
                match m2758 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v4) => {
                        let a2759 = v3;
                        let a2760 = v4;
                        if equal(a2759, a2760) {
                            Ok(Adt157::C1(v2))
                        } else {
                            Ok(Adt157::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2761 = f575(v1);
                match m2761 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v6) => {
                        if f733(f577(v5, v6)?) {
                            Ok(Adt157::C1(v2))
                        } else {
                            Ok(Adt157::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2762 = f233(v1);
                match m2762 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v8) => {
                        let a2763 = v7;
                        let a2764 = v8;
                        if equal(a2763, a2764) {
                            Ok(Adt157::C1(v2))
                        } else {
                            Ok(Adt157::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2765 = f409(v1);
                match m2765 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v10) => {
                        Ok(f734(f735(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2766 = f259(v1);
                match m2766 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v12) => {
                        let a2768 = f581(v11.clone(), 0u64)?;
                        let a2769 = f581(match v12.clone() {
                            Adt5::C0(h2767) => {
                                h2767
                            }
                        }, 0u64)?;
                        if nat_eq(a2768, a2769) {
                            Ok(f734(f736(v11, match v12 {
                                Adt5::C0(h2770) => {
                                    h2770
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
    let m2771 = v0;
    match m2771 {
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
    let m2772 = f652(f738(v0, v1)?, f738(v2, v3)?);
    match m2772 {
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
    let m2775 = {
        let a2773 = v0;
        let a2774 = 0i64;
        compare(a2773, a2774)
    };
    if match m2775 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2776 = v0;
        int_neg(a2776)
    } else {
        Ok(v0)
    }
}

pub fn f698(v0: Option<List<Adt42>>) -> List<(Str, Adt6)> {
    let m2777 = v0;
    match m2777 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f741(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f699(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2778 = v0;
    match m2778 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f700(p2781: List<Adt58>, p2782: List<Adt6>) -> List<Adt6> {
    let m2780 = (p2781, p2782);
    {
        let mut m2780 = m2780;
        loop {
            let step = {
                let (v0, v1) = m2780;
                let m2779 = v0;
                match m2779.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f742(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f701(p2786: List<Adt71>, p2787: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2785 = (p2786, p2787);
    {
        let mut m2785 = m2785;
        loop {
            let step = {
                let (v0, v1) = m2785;
                let m2783 = v0;
                match m2783.uncons() {
                    None => {
                        Err::<(List<Adt71>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2784 = f743(v2)?;
                        match m2784 {
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
                Ok(next) => { m2785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(v0: Adt76) -> Adt6 {
    Adt6::C5({
        let a2797 = {
            let m2789 = match v0.clone() {
                Adt76::C0(h2788, _, _, _) => {
                    h2788
                }
            };
            match m2789 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f303(f744(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2798 = {
            let a2795 = f704(Str::lit("costPriority"), match v0.clone() {
                Adt76::C0(_, h2790, _, _) => {
                    h2790
                }
            });
            let a2796 = {
                let a2793 = f704(Str::lit("speedPriority"), match v0.clone() {
                    Adt76::C0(_, _, h2791, _) => {
                        h2791
                    }
                });
                let a2794 = f704(Str::lit("intelligencePriority"), match v0 {
                    Adt76::C0(_, _, _, h2792) => {
                        h2792
                    }
                });
                append_list(a2793, a2794)
            };
            append_list(a2795, a2796)
        };
        append_list(a2797, a2798)
    })
}

pub fn f703(v0: Adt78) -> Str {
    let m2799 = v0;
    match m2799 {
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
    let m2800 = v1;
    match m2800 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f705(v0: Adt79) -> List<(Str, Adt6)> {
    let m2802 = match v0 {
        Adt79::C0(h2801) => {
            h2801
        }
    };
    match m2802 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f745(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f706(v0: Adt66) -> List<(Str, Adt6)> {
    let m2803 = v0;
    match m2803 {
        Adt66::C0(v1) => {
            let a2810 = {
                let m2805 = match v1.clone() {
                    Adt67::C0(h2804, _, _) => {
                        h2804
                    }
                };
                match m2805 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2811 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt67::C0(_, h2806, _) => {
                    h2806
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m2809 = match v1 {
                    Adt67::C0(_, _, h2808) => {
                        h2808
                    }
                };
                match m2809 {
                    Adt5::C0(h2807) => {
                        h2807
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a2810, a2811)
        }
        Adt66::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt69::C0(h2812, _) => {
                    h2812
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt69::C0(_, h2813) => {
                    h2813
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f707(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2814, _, _, _) => {
            h2814
        }
    })), {
        let a2820 = f419(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2815, _, _) => {
                h2815
            }
        });
        let a2821 = {
            let a2818 = f746(match v0.clone() {
                Adt2::C0(_, _, h2816, _) => {
                    h2816
                }
            });
            let a2819 = f747(match v0 {
                Adt2::C0(_, _, _, h2817) => {
                    h2817
                }
            });
            append_list(a2818, a2819)
        };
        append_list(a2820, a2821)
    }))
}

pub fn f708(p2827: List<(Str, Adt6)>, p2828: List<(Str, Adt65)>) -> Option<List<(Str, Adt65)>> {
    let m2826 = (p2827, p2828);
    {
        let mut m2826 = m2826;
        loop {
            let step = {
                let (v0, v1) = m2826;
                let m2822 = v0;
                match m2822.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(Some(f748(v1, List::<(Str, Adt65)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2824 = f749({
                            let (_, h2823) = v2;
                            h2823
                        });
                        match m2824 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(None::<List<(Str, Adt65)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>((v3, List::cons(({
                                    let (h2825, _) = v2;
                                    h2825
                                }, v4), v1)))
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

pub fn f709(v0: Adt87) -> Adt6 {
    let m2829 = v0;
    match m2829 {
        Adt87::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h2830) => {
                    h2830
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt87::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f129(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f710(v0: u64, v1: List<u64>) -> List<u64> {
    let a2831 = v0;
    let a2832 = 0u64;
    if nat_eq(a2831, a2832) {
        v1
    } else {
        let a2833 = v0;
        let a2834 = 1u64;
        if nat_eq(a2833, a2834) {
            List::cons(194u64, v1)
        } else {
            let a2835 = v0;
            let a2836 = 2u64;
            if nat_eq(a2835, a2836) {
                List::cons(226u64, v1)
            } else {
                let a2837 = v0;
                let a2838 = 3u64;
                if nat_eq(a2837, a2838) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f711(v0: u64, v1: u64, v2: List<u64>) -> Adt156 {
    let a2839 = v0;
    let a2840 = 0u64;
    if nat_eq(a2839, a2840) {
        f750(v1, v2)
    } else {
        let a2841 = v0;
        let a2842 = 1u64;
        if nat_eq(a2841, a2842) {
            let a2843 = 128u64;
            let a2844 = v1;
            if if nat_le(a2843, a2844) {
                let a2845 = v1;
                let a2846 = 159u64;
                nat_le(a2845, a2846)
            } else {
                false
            } {
                Adt156::C0(0u64, f751(v2))
            } else {
                f750(v1, List::cons(194u64, v2))
            }
        } else {
            let a2847 = v0;
            let a2848 = 2u64;
            if nat_eq(a2847, a2848) {
                let a2849 = v1;
                let a2850 = 128u64;
                if nat_eq(a2849, a2850) {
                    Adt156::C0(3u64, v2)
                } else {
                    let a2851 = v1;
                    let a2852 = 129u64;
                    if nat_eq(a2851, a2852) {
                        Adt156::C0(4u64, v2)
                    } else {
                        f750(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2853 = v0;
                let a2854 = 3u64;
                if nat_eq(a2853, a2854) {
                    let a2855 = 170u64;
                    let a2856 = v1;
                    if if nat_le(a2855, a2856) {
                        let a2857 = v1;
                        let a2858 = 174u64;
                        nat_le(a2857, a2858)
                    } else {
                        false
                    } {
                        Adt156::C0(0u64, f751(v2))
                    } else {
                        f750(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2859 = 166u64;
                    let a2860 = v1;
                    if if nat_le(a2859, a2860) {
                        let a2861 = v1;
                        let a2862 = 169u64;
                        nat_le(a2861, a2862)
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
    let a2863 = f464(v0);
    let a2864 = f464(v1);
    equal(a2863, a2864)
}

pub fn f713(v0: Adt140) -> bool {
    let m2865 = v0;
    match m2865 {
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
    let m2866 = v0;
    match m2866 {
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
        Adt17::C0(_, h2867, _, _, _, _) => {
            h2867
        }
    }, f432(v2)))
}

pub fn f716(v0: Adt15) -> Adt17 {
    let m2868 = v0;
    match m2868 {
        Adt15::C0(v1) => {
            match v1 {
                Adt16::C0(h2869) => {
                    h2869
                }
            }
        }
        Adt15::C1(v2) => {
            match v2 {
                Adt21::C0(h2870, _) => {
                    h2870
                }
            }
        }
        Adt15::C2(v3) => {
            match v3 {
                Adt22::C0(h2871, _, _, _, _) => {
                    h2871
                }
            }
        }
        Adt15::C3(v4) => {
            match v4 {
                Adt21::C0(h2872, _) => {
                    h2872
                }
            }
        }
        Adt15::C4(v5) => {
            match v5 {
                Adt23::C0(h2873, _, _, _) => {
                    h2873
                }
            }
        }
        Adt15::C5(v6) => {
            match v6 {
                Adt21::C0(h2874, _) => {
                    h2874
                }
            }
        }
        Adt15::C6(v7) => {
            match v7 {
                Adt21::C0(h2875, _) => {
                    h2875
                }
            }
        }
        Adt15::C7(v8) => {
            match v8 {
                Adt24::C0(h2876, _, _, _, _) => {
                    h2876
                }
            }
        }
        Adt15::C8(v9) => {
            match v9 {
                Adt25::C0(h2877, _, _, _) => {
                    h2877
                }
            }
        }
        Adt15::C9(v10) => {
            match v10 {
                Adt30::C0(h2878, _) => {
                    h2878
                }
            }
        }
        Adt15::C10(v11) => {
            match v11 {
                Adt31::C0(h2879, _, _) => {
                    h2879
                }
            }
        }
        Adt15::C11(v12) => {
            match v12 {
                Adt32::C0(h2880, _) => {
                    h2880
                }
            }
        }
    }
}

pub fn f717(v0: Adt0, v1: Adt17, v2: Adt83) -> Result<Adt128, Adt131> {
    f754(v0, v2, f755(match v1 {
        Adt17::C0(_, h2881, _, _, _, _) => {
            h2881
        }
    }))
}

pub fn f718(v0: Adt0, v1: Adt17, v2: Adt88) -> Result<Adt128, Adt131> {
    f756(v0, v2, f755(match v1 {
        Adt17::C0(_, h2882, _, _, _, _) => {
            h2882
        }
    }))
}

pub fn f719(v0: Adt0, v1: Adt17, v2: Adt50) -> Adt128 {
    Adt128::C0(Adt33::C2(match v2.clone() {
        Adt50::C0(h2883, _) => {
            h2883
        }
    }), {
        let m2885 = f616(v0, Adt33::C11);
        match m2885 {
            Adt128::C0(_, h2884, _) => {
                h2884
            }
        }
    }, f757(v1, match v2 {
        Adt50::C0(_, h2886) => {
            h2886
        }
    }))
}

pub fn f720(v0: Bytes, v1: Adt15, v2: Result<Adt33, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m2887 = v2;
    match m2887 {
        Ok(v4) => {
            f758(v0, f615(v1.clone()), f759(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f721(v0: u64, v1: u64, v2: Fn2, v3: Adt15) -> R<Result<Adt33, Adt90>> {
    let m2888 = v3.clone();
    match m2888 {
        Adt15::C0(_) => {
            let c2890: Fn2 = v2;
            c2890.apply(v3)
        }
        Adt15::C1(_) => {
            let c2891: Fn2 = v2;
            c2891.apply(v3)
        }
        Adt15::C2(_) => {
            if f760(v0, v1) {
                let c2889: Fn2 = v2;
                c2889.apply(v3)
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C7(v0)))
            }
        }
        Adt15::C3(_) => {
            let c2892: Fn2 = v2;
            c2892.apply(v3)
        }
        Adt15::C4(_) => {
            let c2893: Fn2 = v2;
            c2893.apply(v3)
        }
        Adt15::C5(_) => {
            let c2894: Fn2 = v2;
            c2894.apply(v3)
        }
        Adt15::C6(_) => {
            let c2895: Fn2 = v2;
            c2895.apply(v3)
        }
        Adt15::C7(_) => {
            let c2896: Fn2 = v2;
            c2896.apply(v3)
        }
        Adt15::C8(_) => {
            let c2897: Fn2 = v2;
            c2897.apply(v3)
        }
        Adt15::C9(_) => {
            let c2898: Fn2 = v2;
            c2898.apply(v3)
        }
        Adt15::C10(_) => {
            let c2899: Fn2 = v2;
            c2899.apply(v3)
        }
        Adt15::C11(_) => {
            let c2900: Fn2 = v2;
            c2900.apply(v3)
        }
    }
}

pub fn f722(v0: Bytes, v1: Adt22) -> R<Result<Adt15, Adt90>> {
    let m2904 = f761(v0, Str::lit("tools/call"), match v1.clone() {
        Adt22::C0(_, h2901, _, _, _) => {
            h2901
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, _, h2902) => {
            h2902
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h2903, _) => {
            h2903
        }
    })?;
    match m2904 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C2(Adt22::C0(match v1.clone() {
                Adt22::C0(h2905, _, _, _, _) => {
                    h2905
                }
            }, match v1.clone() {
                Adt22::C0(_, h2906, _, _, _) => {
                    h2906
                }
            }, match v1.clone() {
                Adt22::C0(_, _, h2907, _, _) => {
                    h2907
                }
            }, match v1 {
                Adt22::C0(_, _, _, h2908, _) => {
                    h2908
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f723(v0: Bytes, v1: Adt23) -> R<Result<Adt15, Adt90>> {
    let m2912 = f761(v0, Str::lit("resources/read"), match v1.clone() {
        Adt23::C0(_, h2909, _, _) => {
            h2909
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h2910) => {
            h2910
        }
    }, match v1.clone() {
        Adt23::C0(_, _, h2911, _) => {
            h2911
        }
    })?;
    match m2912 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C4(Adt23::C0(match v1.clone() {
                Adt23::C0(h2913, _, _, _) => {
                    h2913
                }
            }, match v1.clone() {
                Adt23::C0(_, h2914, _, _) => {
                    h2914
                }
            }, match v1 {
                Adt23::C0(_, _, h2915, _) => {
                    h2915
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f724(v0: Bytes, v1: Adt24) -> R<Result<Adt15, Adt90>> {
    let m2919 = f761(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt24::C0(_, h2916, _, _, _) => {
            h2916
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, _, h2917) => {
            h2917
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, h2918, _) => {
            h2918
        }
    })?;
    match m2919 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C7(Adt24::C0(match v1.clone() {
                Adt24::C0(h2920, _, _, _, _) => {
                    h2920
                }
            }, match v1.clone() {
                Adt24::C0(_, h2921, _, _, _) => {
                    h2921
                }
            }, match v1.clone() {
                Adt24::C0(_, _, h2922, _, _) => {
                    h2922
                }
            }, match v1 {
                Adt24::C0(_, _, _, h2923, _) => {
                    h2923
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f725(v0: Str, p2929: List<(Str, Adt6)>, p2930: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt132> {
    let m2928 = (p2929, p2930);
    {
        let mut m2928 = m2928;
        loop {
            let step = {
                let (v1, v2) = m2928;
                let m2924 = v1;
                match m2924.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Ok::<List<(Str, Str)>, Adt132>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2926 = f233({
                            let (_, h2925) = v3;
                            h2925
                        });
                        match m2926 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Err::<List<(Str, Str)>, Adt132>(Adt132::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>((v4, List::cons(({
                                    let (h2927, _) = v3;
                                    h2927
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2928 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f726(p2933: List<(Str, Str)>, p2934: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2932 = (p2933, p2934);
    {
        let mut m2932 = m2932;
        loop {
            let step = {
                let (v0, v1) = m2932;
                let m2931 = v0;
                match m2931.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2932 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f727(v0: Adt111, v1: Adt5) -> R<Option<Adt109>> {
    let m2936 = f762(match v0.clone() {
        Adt111::C0(_, h2935, _, _) => {
            h2935
        }
    }, v1);
    match m2936 {
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
    let m2937 = f181(v0, v1);
    match m2937 {
        Adt135::C0 => {
            Some(None::<Adt5>)
        }
        Adt135::C1(v2) => {
            let m2938 = f259(v2);
            match m2938 {
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
    let a2939 = v0.clone();
    let a2940 = Str::lit("debug");
    if equal(a2939, a2940) {
        Adt158::C1(Adt19::C0)
    } else {
        let a2941 = v0.clone();
        let a2942 = Str::lit("info");
        if equal(a2941, a2942) {
            Adt158::C1(Adt19::C1)
        } else {
            let a2943 = v0.clone();
            let a2944 = Str::lit("notice");
            if equal(a2943, a2944) {
                Adt158::C1(Adt19::C2)
            } else {
                let a2945 = v0.clone();
                let a2946 = Str::lit("warning");
                if equal(a2945, a2946) {
                    Adt158::C1(Adt19::C3)
                } else {
                    let a2947 = v0.clone();
                    let a2948 = Str::lit("error");
                    if equal(a2947, a2948) {
                        Adt158::C1(Adt19::C4)
                    } else {
                        let a2949 = v0.clone();
                        let a2950 = Str::lit("critical");
                        if equal(a2949, a2950) {
                            Adt158::C1(Adt19::C5)
                        } else {
                            let a2951 = v0.clone();
                            let a2952 = Str::lit("alert");
                            if equal(a2951, a2952) {
                                Adt158::C1(Adt19::C6)
                            } else {
                                let a2953 = v0;
                                let a2954 = Str::lit("emergency");
                                if equal(a2953, a2954) {
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
    let m2955 = v0;
    match m2955 {
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
    let m2956 = v0;
    match m2956 {
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
    let m2957 = v0;
    match m2957 {
        None => {
            Adt157::C0
        }
        Some(v2) => {
            Adt157::C1({
                let a2958 = v2;
                let a2959 = v1;
                append_list(a2958, a2959)
            })
        }
    }
}

pub fn f735(p2964: List<Adt6>, p2965: List<Adt6>, p2966: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2963 = (p2964, (p2965, p2966));
    {
        let mut m2963 = m2963;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2963;
                let m2960 = v0;
                match m2960.uncons() {
                    None => {
                        let m2961 = v1;
                        match m2961.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2962 = v1;
                        match m2962.uncons() {
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
                Ok(next) => { m2963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f736(p2972: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2973: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2971 = (p2972, p2973);
    {
        let mut m2971 = m2971;
        loop {
            let step = {
                let (v0, v2) = m2971;
                let m2967 = v0;
                match m2967.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2969 = f236({
                            let (h2968, _) = v3;
                            h2968
                        }, v1.clone());
                        match m2969 {
                            Adt135::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt135::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2970) = v3;
                                    h2970
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f737(v0: Adt155) -> Adt155 {
    let m2974 = v0;
    match m2974 {
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
    let a2975 = v1;
    let a2976 = v3;
    if nat_lt(a2975, a2976) {
        Ok(f652(f766({
            let a2977 = v3;
            let a2978 = v1;
            nat_sub(a2977, a2978)
        }, v0)?, v2))
    } else {
        let a2979 = v3;
        let a2980 = v1;
        if nat_lt(a2979, a2980) {
            Ok(f652(v0, f766({
                let a2981 = v1;
                let a2982 = v3;
                nat_sub(a2981, a2982)
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
    f303(f768(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f742(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a2991 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h2983, _, _, _) => {
                h2983
            }
        })), List::<(Str, Adt6)>::nil());
        let a2992 = {
            let a2989 = f419(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2984, _, _) => {
                    h2984
                }
            });
            let a2990 = {
                let a2987 = f419(Str::lit("description"), match v0.clone() {
                    Adt58::C0(_, _, h2985, _) => {
                        h2985
                    }
                });
                let a2988 = f424(Str::lit("required"), match v0 {
                    Adt58::C0(_, _, _, h2986) => {
                        h2986
                    }
                });
                append_list(a2987, a2988)
            };
            append_list(a2989, a2990)
        };
        append_list(a2991, a2992)
    })
}

pub fn f743(v0: Adt71) -> R<Option<Adt6>> {
    let m2994 = f769(match v0.clone() {
        Adt71::C0(_, h2993, _) => {
            h2993
        }
    })?;
    match m2994 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f663(match v0.clone() {
                Adt71::C0(h2995, _, _) => {
                    h2995
                }
            }))), List::cons((Str::lit("content"), v1), f655(match v0 {
                Adt71::C0(_, _, h2996) => {
                    h2996
                }
            }))))))
        }
    }
}

pub fn f744(p3000: List<Adt77>, p3001: List<Adt6>) -> List<Adt6> {
    let m2999 = (p3000, p3001);
    {
        let mut m2999 = m2999;
        loop {
            let step = {
                let (v0, v1) = m2999;
                let m2997 = v0;
                match m2997.uncons() {
                    None => {
                        Err::<(List<Adt77>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt77>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f419(Str::lit("name"), match v2 {
                            Adt77::C0(h2998) => {
                                h2998
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2999 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f745(v0: Adt80) -> Str {
    let m3002 = v0;
    match m3002 {
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
    let m3003 = v0;
    match m3003 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f378(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f747(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3004 = v0;
    match m3004 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f770(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f748(p3007: List<(Str, Adt65)>, p3008: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m3006 = (p3007, p3008);
    {
        let mut m3006 = m3006;
        loop {
            let step = {
                let (v0, v1) = m3006;
                let m3005 = v0;
                match m3005.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3006 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f749(v0: Adt6) -> Option<Adt65> {
    let m3009 = f259(v0);
    match m3009 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m3010 = f671(Str::lit("method"), v1.clone());
            match m3010 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a3011 = v2.clone();
                    let a3012 = Str::lit("elicitation/create");
                    if equal(a3011, a3012) {
                        let m3013 = f606(Str::lit("params"), v1);
                        match m3013 {
                            None => {
                                None::<Adt65>
                            }
                            Some(v3) => {
                                f771(v3)
                            }
                        }
                    } else {
                        let a3014 = v2;
                        let a3015 = Str::lit("roots/list");
                        if equal(a3014, a3015) {
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
    let a3016 = v0;
    let a3017 = 32u64;
    if nat_lt(a3016, a3017) {
        if f773(v0) {
            Adt156::C0(0u64, List::cons(v0, v1))
        } else {
            Adt156::C0(0u64, f751(v1))
        }
    } else {
        let a3018 = v0;
        let a3019 = 127u64;
        if nat_eq(a3018, a3019) {
            Adt156::C0(0u64, f751(v1))
        } else {
            let a3020 = v0;
            let a3021 = 194u64;
            if nat_eq(a3020, a3021) {
                Adt156::C0(1u64, v1)
            } else {
                let a3022 = v0;
                let a3023 = 226u64;
                if nat_eq(a3022, a3023) {
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
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f774(f432(v1)))))
    }
}

pub fn f753(v0: Adt18, v1: List<(Str, Adt65)>) -> bool {
    let a3024 = f775(v1.clone());
    if if if bool_not(a3024) {
        true
    } else {
        f467(match v0.clone() {
            Adt18::C0(_, _, _, h3025, _) => {
                h3025
            }
        })
    } {
        let a3026 = f776(v1.clone());
        if bool_not(a3026) {
            true
        } else {
            f467(match v0.clone() {
                Adt18::C0(_, _, h3027, _, _) => {
                    h3027
                }
            })
        }
    } else {
        false
    } {
        let a3028 = f777(v1);
        if bool_not(a3028) {
            true
        } else {
            f467(match v0 {
                Adt18::C0(_, h3029, _, _, _) => {
                    h3029
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
    f468(match v0 {
        Adt18::C0(_, _, _, _, h3030) => {
            h3030
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
    let m3032 = match v0 {
        Adt17::C0(_, _, _, _, h3031, _) => {
            h3031
        }
    };
    match m3032 {
        None => {
            List::<Adt130>::nil()
        }
        Some(v2) => {
            f779(f780(v2, v1, List::<Adt130>::nil()), List::<Adt130>::nil())
        }
    }
}

pub fn f758(v0: Bytes, v1: Adt140, v2: Str, v3: Adt33) -> R<Result<Adt33, Adt90>> {
    let m3033 = v3.clone();
    match m3033 {
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
    let m3034 = v0;
    match m3034 {
        Adt15::C0(_) => {
            Str::lit("")
        }
        Adt15::C1(_) => {
            Str::lit("")
        }
        Adt15::C2(v1) => {
            match v1 {
                Adt22::C0(_, h3035, _, _, _) => {
                    h3035
                }
            }
        }
        Adt15::C3(_) => {
            Str::lit("")
        }
        Adt15::C4(v2) => {
            match v2 {
                Adt23::C0(_, h3036, _, _) => {
                    h3036
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
                Adt24::C0(_, h3037, _, _, _) => {
                    h3037
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
    let a3038 = v1;
    let a3039 = v0;
    nat_lt(a3038, a3039)
}

pub fn f761(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt90>> {
    Ok(f782(f783(v0, v1, v2, v3)?, f784(v4)))
}

pub fn f762(p3045: List<Str>, p3046: Adt5) -> Adt135 {
    let m3044 = (p3045, p3046);
    {
        let mut m3044 = m3044;
        loop {
            let step = {
                let (v0, v1) = m3044;
                let m3040 = v0;
                match m3040.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                    }
                    Some((v2, v3)) => {
                        let m3041 = f181(v2, v1);
                        match m3041 {
                            Adt135::C0 => {
                                Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                            }
                            Adt135::C1(v4) => {
                                let m3042 = v3.clone();
                                match m3042.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt135>(Adt135::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3043 = f259(v4);
                                        match m3043 {
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
                Ok(next) => { m3044 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f763(v0: Adt111) -> Option<Adt109> {
    let m3048 = match v0.clone() {
        Adt111::C0(_, _, _, h3047) => {
            h3047
        }
    };
    match m3048 {
        Adt112::C0 => {
            None::<Adt109>
        }
        Adt112::C1 => {
            Some(Adt109::C10(match v0 {
                Adt111::C0(h3049, _, _, _) => {
                    h3049
                }
            }))
        }
        Adt112::C2(_) => {
            None::<Adt109>
        }
    }
}

pub fn f764(v0: Adt6) -> bool {
    let m3050 = v0;
    match m3050 {
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
    let m3052 = match v0.clone() {
        Adt111::C0(_, _, _, h3051) => {
            h3051
        }
    };
    match m3052 {
        Adt112::C0 => {
            Ok(Some(Adt109::C8(match v0 {
                Adt111::C0(h3053, _, _, _) => {
                    h3053
                }
            })))
        }
        Adt112::C1 => {
            Ok(Some(Adt109::C10(match v0 {
                Adt111::C0(h3054, _, _, _) => {
                    h3054
                }
            })))
        }
        Adt112::C2(v2) => {
            if f785(match v0.clone() {
                Adt111::C0(_, _, h3055, _) => {
                    h3055
                }
            }, v2, v1)? {
                Ok(None::<Adt109>)
            } else {
                Ok(Some(Adt109::C9(match v0 {
                    Adt111::C0(h3056, _, _, _) => {
                        h3056
                    }
                })))
            }
        }
    }
}

pub fn f766(p3061: u64, p3062: i64) -> R<i64> {
    let m3060 = (p3061, p3062);
    {
        let mut m3060 = m3060;
        loop {
            let step = {
                let (v0, v1) = m3060;
                let m3057 = v0;
                if m3057 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3057 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3058 = v1;
                        let a3059 = 10i64;
                        int_mul(a3058, a3059)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f767(p3073: u64, p3074: i64, p3075: u64) -> R<u64> {
    let m3072 = (p3073, (p3074, p3075));
    {
        let mut m3072 = m3072;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3072;
                let m3063 = v0;
                if m3063 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3063 - 1;
                    let v4: u64 = {
                        let a3064 = v2;
                        let a3065 = 1u64;
                        nat_add(a3064, a3065)?
                    };
                    let m3068 = {
                        let a3066 = v1;
                        let a3067 = 10i64;
                        compare(a3066, a3067)
                    };
                    if match m3068 {
                        Ordering::Less => {
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
                            let a3069 = v1;
                            let a3070 = 10i64;
                            let a3071 = 0i64;
                            int_quot(a3069, a3070, a3071)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f768(p3078: List<Adt42>, p3079: List<Adt6>) -> List<Adt6> {
    let m3077 = (p3078, p3079);
    {
        let mut m3077 = m3077;
        loop {
            let step = {
                let (v0, v1) = m3077;
                let m3076 = v0;
                match m3076.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt42>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f663(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3077 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f769(v0: Adt72) -> R<Option<Adt6>> {
    let m3080 = v0;
    match m3080 {
        Adt72::C0(v1) => {
            f786(v1)
        }
        Adt72::C1(v2) => {
            let m3081 = f787(v2, List::<Adt6>::nil())?;
            match m3081 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f303(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f770(v0: Adt3) -> Str {
    let m3082 = v0;
    match m3082 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f771(v0: Adt5) -> Option<Adt65> {
    let m3083 = f671(Str::lit("message"), v0.clone());
    match m3083 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m3084 = f181(Str::lit("mode"), v0.clone());
            match m3084 {
                Adt135::C0 => {
                    let m3085 = f606(Str::lit("requestedSchema"), v0);
                    match m3085 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v2) => {
                            Some(Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v1, v2))))
                        }
                    }
                }
                Adt135::C1(v3) => {
                    let m3086 = f233(v3);
                    match m3086 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v4) => {
                            let a3087 = v4.clone();
                            let a3088 = Str::lit("url");
                            if equal(a3087, a3088) {
                                let m3089 = f671(Str::lit("url"), v0);
                                match m3089 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v5) => {
                                        Some(Adt65::C0(Adt66::C1(Adt69::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a3090 = v4;
                                let a3091 = Str::lit("form");
                                if equal(a3090, a3091) {
                                    let m3092 = f606(Str::lit("requestedSchema"), v0);
                                    match m3092 {
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
    let m3093 = f181(Str::lit("params"), v0);
    match m3093 {
        Adt135::C0 => {
            Some(Adt65::C2(None::<Adt5>))
        }
        Adt135::C1(v1) => {
            let m3094 = f259(v1);
            match m3094 {
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
    let a3095 = v0;
    let a3096 = 9u64;
    if nat_eq(a3095, a3096) {
        true
    } else {
        let a3097 = v0;
        let a3098 = 10u64;
        if nat_eq(a3097, a3098) {
            true
        } else {
            let a3099 = v0;
            let a3100 = 13u64;
            nat_eq(a3099, a3100)
        }
    }
}

pub fn f774(v0: List<(Str, Adt65)>) -> Adt18 {
    Adt18::C0(None::<Adt5>, if f777(v0.clone()) {
        Some(f547())
    } else {
        None::<Adt5>
    }, if f776(v0.clone()) {
        Some(f547())
    } else {
        None::<Adt5>
    }, if f775(v0) {
        Some(f547())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f775(p3105: List<(Str, Adt65)>) -> bool {
    let m3104 = p3105;
    {
        let mut m3104 = m3104;
        loop {
            let step = {
                let v0 = m3104;
                let m3101 = v0;
                match m3101.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3103 = {
                            let (_, h3102) = v1;
                            h3102
                        };
                        match m3103 {
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
                Ok(next) => { m3104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f776(p3110: List<(Str, Adt65)>) -> bool {
    let m3109 = p3110;
    {
        let mut m3109 = m3109;
        loop {
            let step = {
                let v0 = m3109;
                let m3106 = v0;
                match m3106.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3108 = {
                            let (_, h3107) = v1;
                            h3107
                        };
                        match m3108 {
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
                Ok(next) => { m3109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f777(p3115: List<(Str, Adt65)>) -> bool {
    let m3114 = p3115;
    {
        let mut m3114 = m3114;
        loop {
            let step = {
                let v0 = m3114;
                let m3111 = v0;
                match m3111.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3113 = {
                            let (_, h3112) = v1;
                            h3112
                        };
                        match m3113 {
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
                Ok(next) => { m3114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f778() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f26(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f779(p3118: List<Adt130>, p3119: List<Adt130>) -> List<Adt130> {
    let m3117 = (p3118, p3119);
    {
        let mut m3117 = m3117;
        loop {
            let step = {
                let (v0, v1) = m3117;
                let m3116 = v0;
                match m3116.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<Adt130>), List<Adt130>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt130>, List<Adt130>), List<Adt130>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f780(v0: Adt20, p3122: List<Adt51>, p3123: List<Adt130>) -> List<Adt130> {
    let m3121 = (p3122, p3123);
    {
        let mut m3121 = m3121;
        loop {
            let step = {
                let (v1, v2) = m3121;
                let m3120 = v1;
                match m3120.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt130>), List<Adt130>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt51>, List<Adt130>), List<Adt130>>((v4, List::cons(Adt130::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f781(v0: Bytes, v1: Adt140, v2: Str, v3: Adt82) -> R<Result<Adt33, Adt90>> {
    if f713(v1.clone()) {
        if f788(v3.clone()) {
            let m3124 = f789(v0, f464(v1), v2, v3)?;
            match m3124 {
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
        Ok(Err::<Adt33, Adt90>(Adt90::C12(f464(v1))))
    }
}

pub fn f782(v0: Result<Option<Str>, Adt90>, v1: bool) -> Result<Option<Str>, Adt90> {
    let m3125 = v0;
    match m3125 {
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
    let m3126 = v3;
    match m3126 {
        None => {
            Ok(Ok::<Option<Str>, Adt90>(None::<Str>))
        }
        Some(v4) => {
            Ok(f790(f791(v0, v1, v2, v4)?))
        }
    }
}

pub fn f784(v0: Option<Adt5>) -> bool {
    let m3127 = v0;
    match m3127 {
        None => {
            true
        }
        Some(v1) => {
            f792(match v1 {
                Adt5::C0(h3128) => {
                    h3128
                }
            })
        }
    }
}

pub fn f785(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3129 = v0;
    match m3129 {
        Adt13::C0 => {
            let m3130 = f233(v2);
            match m3130 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3131 = v1;
                    let a3132 = v3;
                    Ok(equal(a3131, a3132))
                }
            }
        }
        Adt13::C1 => {
            f793(v1, v2)
        }
        Adt13::C2 => {
            let m3133 = f258(v2);
            match m3133 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3134 = v1;
                        let a3135 = Str::lit("true");
                        Ok(equal(a3134, a3135))
                    } else {
                        let a3136 = v1;
                        let a3137 = Str::lit("false");
                        Ok(equal(a3136, a3137))
                    }
                }
            }
        }
    }
}

pub fn f786(v0: Adt73) -> R<Option<Adt6>> {
    let m3138 = v0;
    match m3138 {
        Adt73::C0(v1) => {
            Ok(Some(f656(v1)))
        }
        Adt73::C1(v2) => {
            Ok(Some(f657(Str::lit("image"), match v2.clone() {
                Adt43::C0(h3139, _, _, _) => {
                    h3139
                }
            }, match v2.clone() {
                Adt43::C0(_, h3140, _, _) => {
                    h3140
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h3141, _) => {
                    h3141
                }
            }, match v2 {
                Adt43::C0(_, _, _, h3142) => {
                    h3142
                }
            })))
        }
        Adt73::C2(v3) => {
            Ok(Some(f657(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h3143, _, _, _) => {
                    h3143
                }
            }, match v3.clone() {
                Adt44::C0(_, h3144, _, _) => {
                    h3144
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h3145, _) => {
                    h3145
                }
            }, match v3 {
                Adt44::C0(_, _, _, h3146) => {
                    h3146
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

pub fn f787(p3150: List<Adt73>, p3151: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3149 = (p3150, p3151);
    {
        let mut m3149 = m3149;
        loop {
            let step = {
                let (v0, v1) = m3149;
                let m3147 = v0;
                match m3147.uncons() {
                    None => {
                        Err::<(List<Adt73>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3148 = f786(v2)?;
                        match m3148 {
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
                Ok(next) => { m3149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f788(v0: Adt82) -> bool {
    let m3152 = v0;
    match m3152 {
        Adt82::C0(v1) => {
            let m3153 = v1;
            match m3153.uncons() {
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
    let m3154 = v3.clone();
    match m3154 {
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
    let m3155 = v0;
    match m3155 {
        None => {
            Err::<Option<Str>, Adt90>(Adt90::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt90>(Some(v1))
        }
    }
}

pub fn f791(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3159 = f798(v0, {
        let a3156 = v1;
        utf8_encode(a3156)
    }, {
        let a3157 = v2;
        utf8_encode(a3157)
    }, {
        let a3158 = v3;
        utf8_encode(a3158)
    })?;
    match m3159 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3160 = v4;
            Ok(utf8_decode(a3160))
        }
    }
}

pub fn f792(p3165: List<(Str, Adt6)>) -> bool {
    let m3164 = p3165;
    {
        let mut m3164 = m3164;
        loop {
            let step = {
                let v0 = m3164;
                let m3161 = v0;
                match m3161.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3163 = f259({
                            let (_, h3162) = v1;
                            h3162
                        });
                        match m3163 {
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
                Ok(next) => { m3164 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f793(v0: Str, v1: Adt6) -> R<bool> {
    let m3166 = f575(v1);
    match m3166 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3167 = f235(v2)?;
            match m3167 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3169 = f799(f86({
                        let a3168 = v0;
                        utf8_encode(a3168)
                    })?)?;
                    match m3169 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f394({
                                let a3170 = v3;
                                let a3171 = v4;
                                int_sub(a3170, a3171)?
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
        Adt74::C0(h3172, _, _, _) => {
            h3172
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt74::C0(_, h3173, _, _) => {
            h3173
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3176 = match v0.clone() {
            Adt74::C0(_, _, h3175, _) => {
                h3175
            }
        };
        match m3176 {
            Adt5::C0(h3174) => {
                h3174
            }
        }
    })), f655(match v0 {
        Adt74::C0(_, _, _, h3177) => {
            h3177
        }
    }))))))
}

pub fn f795(v0: Adt75) -> R<Option<Adt6>> {
    let m3179 = f422(match v0.clone() {
        Adt75::C0(_, h3178, _, _, _) => {
            h3178
        }
    })?;
    match m3179 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt75::C0(h3180, _, _, _, _) => {
                    h3180
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3186 = f423(Str::lit("structuredContent"), match v0.clone() {
                    Adt75::C0(_, _, h3181, _, _) => {
                        h3181
                    }
                });
                let a3187 = {
                    let a3184 = f424(Str::lit("isError"), match v0.clone() {
                        Adt75::C0(_, _, _, h3182, _) => {
                            h3182
                        }
                    });
                    let a3185 = f655(match v0 {
                        Adt75::C0(_, _, _, _, h3183) => {
                            h3183
                        }
                    });
                    append_list(a3184, a3185)
                };
                append_list(a3186, a3187)
            }))))))
        }
    }
}

pub fn f796(v0: Option<Str>, v1: List<(Str, Adt65)>) -> Result<Adt82, Adt90> {
    let m3188 = v0;
    match m3188 {
        None => {
            Err::<Adt82, Adt90>(Adt90::C14)
        }
        Some(v2) => {
            let m3189 = v1.clone();
            match m3189.uncons() {
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
    let m3193 = f800(v0, {
        let a3190 = v1;
        utf8_encode(a3190)
    }, {
        let a3191 = v2;
        utf8_encode(a3191)
    }, {
        let a3192 = v3;
        utf8_encode(a3192)
    })?;
    match m3193 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3194 = v4;
            Ok(utf8_decode(a3194))
        }
    }
}

pub fn f798(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3196 = f801();
    let a3197 = {
        let a3195 = v0.clone();
        length_bytes(a3195)
    };
    if nat_le(a3196, a3197) {
        let a3199 = f802(v3.clone())?;
        let a3200 = {
            let a3198 = v3.clone();
            length_bytes(a3198)
        };
        if nat_eq(a3199, a3200) {
            Ok(None::<Bytes>)
        } else {
            f803(v0, v1, v2, v3.clone(), f802(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f799(v0: List<u64>) -> R<Option<i64>> {
    let m3201 = v0.clone();
    match m3201.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3202 = v1;
            let a3203 = 45u64;
            if nat_eq(a3202, a3203) {
                let m3204 = v2.clone();
                match m3204.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3205 = f804(v2, 0i64)?;
                        match m3205 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3206 = v5;
                                    int_neg(a3206)?
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
    let a3208 = f801();
    let a3209 = {
        let a3207 = v0.clone();
        length_bytes(a3207)
    };
    if nat_le(a3208, a3209) {
        Ok(Some({
            let a3212 = f805(v3.clone())?;
            let a3213 = {
                let a3210 = Bytes::lit(&[0x2e]);
                let a3211 = f805(f806(v0, v1, v2, v3)?)?;
                append_bytes(a3210, a3211)
            };
            append_bytes(a3212, a3213)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f801() -> u64 {
    32u64
}

pub fn f802(v0: Bytes) -> R<u64> {
    let (_, h3214) = f807(Fn4::F808(v0.clone()), {
        let a3215 = v0.clone();
        length_bytes(a3215)
    }, (0u64, {
        let a3216 = v0;
        length_bytes(a3216)
    }))?;
    Ok(h3214)
}

pub fn f803(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3220 = {
        let a3217 = v3.clone();
        let a3218 = 0u64;
        let a3219 = v4;
        slice_bytes(a3217, a3218, a3219)
    };
    match m3220 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3231 = {
                let a3228 = v3.clone();
                let a3229 = {
                    let a3221 = v4;
                    let a3222 = 1u64;
                    nat_add(a3221, a3222)?
                };
                let a3230 = {
                    let a3226 = {
                        let a3223 = v3;
                        length_bytes(a3223)
                    };
                    let a3227 = {
                        let a3224 = v4;
                        let a3225 = 1u64;
                        nat_add(a3224, a3225)?
                    };
                    nat_sub(a3226, a3227)
                };
                slice_bytes(a3228, a3229, a3230)
            };
            match m3231 {
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

pub fn f804(p3237: List<u64>, p3238: i64) -> R<Option<i64>> {
    let m3236 = (p3237, p3238);
    {
        let mut m3236 = m3236;
        loop {
            let step = {
                let (v0, v1) = m3236;
                let m3232 = v0;
                match m3232.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3233 = f811(v2);
                        match m3233 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f116({
                                    let a3234 = v1;
                                    let a3235 = 10i64;
                                    int_mul(a3234, a3235)?
                                }, f302(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f805(v0: Bytes) -> R<Bytes> {
    let (_, h3239) = f812(Fn5::F813(v0.clone()), {
        let a3240 = v0;
        length_bytes(a3240)
    }, (0u64, f88()))?;
    Ok(h3239)
}

pub fn f806(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f814(v0, {
        let a3241 = f815(v1, v2);
        let a3242 = v3;
        append_bytes(a3241, a3242)
    })
}

pub fn f807(v0: Fn4, p3246: u64, p3247: (u64, u64)) -> R<(u64, u64)> {
    let m3245 = (p3246, p3247);
    {
        let mut m3245 = m3245;
        loop {
            let step = {
                let (v1, v2) = m3245;
                let m3243 = v1;
                if m3243 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3243 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3244: Fn4 = v0.clone();
                        c3244.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f808(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3249 = {
            let (h3248, _) = v1;
            h3248
        };
        let a3250 = 1u64;
        nat_add(a3249, a3250)?
    }, {
        let a3253 = {
            let (_, h3251) = v1;
            h3251
        };
        let a3254 = {
            let a3252 = v0.clone();
            length_bytes(a3252)
        };
        if nat_eq(a3253, a3254) {
            let a3256 = f56(v0, {
                let (h3255, _) = v1;
                h3255
            })?;
            let a3257 = 46u64;
            if nat_eq(a3256, a3257) {
                let (h3258, _) = v1;
                h3258
            } else {
                let (_, h3259) = v1;
                h3259
            }
        } else {
            let (_, h3260) = v1;
            h3260
        }
    }))
}

pub fn f809(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3261 = v3;
    match m3261 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3262 = v4;
            match m3262 {
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
    let a3267 = {
        let a3264 = {
            let a3263 = v0.clone();
            length_bytes(a3263)
        };
        let a3265 = 2u64;
        let a3266 = 0u64;
        nat_rem(a3264, a3265, a3266)
    };
    let a3268 = 0u64;
    if nat_eq(a3267, a3268) {
        let v1: (u64, (Bytes, u64)) = f817(Fn6::F818(v0.clone()), {
            let a3270 = {
                let a3269 = v0;
                length_bytes(a3269)
            };
            let a3271 = 2u64;
            let a3272 = 0u64;
            nat_quot(a3270, a3271, a3272)
        }, (0u64, (f88(), 0u64)))?;
        let a3275 = {
            let (_, h3273) = {
                let (_, h3274) = v1.clone();
                h3274
            };
            h3273
        };
        let a3276 = 0u64;
        if nat_eq(a3275, a3276) {
            Ok(Some({
                let (h3277, _) = {
                    let (_, h3278) = v1;
                    h3278
                };
                h3277
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f811(v0: u64) -> Option<u64> {
    let a3279 = v0;
    let a3280 = 48u64;
    if if nat_lt(a3279, a3280) {
        true
    } else {
        let a3281 = 57u64;
        let a3282 = v0;
        nat_lt(a3281, a3282)
    } {
        None::<u64>
    } else {
        Some({
            let a3283 = v0;
            let a3284 = 48u64;
            nat_sub(a3283, a3284)
        })
    }
}

pub fn f812(v0: Fn5, p3288: u64, p3289: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3287 = (p3288, p3289);
    {
        let mut m3287 = m3287;
        loop {
            let step = {
                let (v1, v2) = m3287;
                let m3285 = v1;
                if m3285 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3285 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3286: Fn5 = v0.clone();
                        c3286.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f813(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3291 = {
            let (h3290, _) = v1.clone();
            h3290
        };
        let a3292 = 1u64;
        nat_add(a3291, a3292)?
    }, {
        let a3295 = {
            let (_, h3293) = v1;
            h3293
        };
        let a3296 = f819(f56(v0, {
            let (h3294, _) = v1;
            h3294
        })?)?;
        append_bytes(a3295, a3296)
    }))
}

pub fn f814(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f820(f821(v0, v1)?)
}

pub fn f815(v0: Bytes, v1: Bytes) -> Bytes {
    let a3303 = f822({
        let a3297 = v0.clone();
        length_bytes(a3297)
    });
    let a3304 = {
        let a3301 = v0;
        let a3302 = {
            let a3299 = f822({
                let a3298 = v1.clone();
                length_bytes(a3298)
            });
            let a3300 = v1;
            append_bytes(a3299, a3300)
        };
        append_bytes(a3301, a3302)
    };
    append_bytes(a3303, a3304)
}

pub fn f816(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3307 = {
        let a3305 = v0.clone();
        length_bytes(a3305)
    };
    let a3308 = {
        let a3306 = v1.clone();
        length_bytes(a3306)
    };
    if nat_eq(a3307, a3308) {
        let a3309 = f823(v0, v1)?;
        let a3310 = 0u64;
        Ok(nat_eq(a3309, a3310))
    } else {
        Ok(false)
    }
}

pub fn f817(v0: Fn6, p3314: u64, p3315: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3313 = (p3314, p3315);
    {
        let mut m3313 = m3313;
        loop {
            let step = {
                let (v1, v2) = m3313;
                let m3311 = v1;
                if m3311 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3311 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3312: Fn6 = v0.clone();
                        c3312.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f818(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f824(v0, v1)
}

pub fn f819(v0: u64) -> R<Bytes> {
    let a3322 = f825({
        let a3316 = v0;
        let a3317 = 16u64;
        let a3318 = 0u64;
        nat_quot(a3316, a3317, a3318)
    })?;
    let a3323 = f825({
        let a3319 = v0;
        let a3320 = 16u64;
        let a3321 = 0u64;
        nat_rem(a3319, a3320, a3321)
    })?;
    Ok(append_bytes(a3322, a3323))
}

pub fn f820(v0: Adt159) -> R<Bytes> {
    let a3344 = f826(match v0.clone() {
        Adt159::C0(h3324, _, _, _, _, _, _, _) => {
            h3324
        }
    })?;
    let a3345 = {
        let a3342 = f826(match v0.clone() {
            Adt159::C0(_, h3325, _, _, _, _, _, _) => {
                h3325
            }
        })?;
        let a3343 = {
            let a3340 = f826(match v0.clone() {
                Adt159::C0(_, _, h3326, _, _, _, _, _) => {
                    h3326
                }
            })?;
            let a3341 = {
                let a3338 = f826(match v0.clone() {
                    Adt159::C0(_, _, _, h3327, _, _, _, _) => {
                        h3327
                    }
                })?;
                let a3339 = {
                    let a3336 = f826(match v0.clone() {
                        Adt159::C0(_, _, _, _, h3328, _, _, _) => {
                            h3328
                        }
                    })?;
                    let a3337 = {
                        let a3334 = f826(match v0.clone() {
                            Adt159::C0(_, _, _, _, _, h3329, _, _) => {
                                h3329
                            }
                        })?;
                        let a3335 = {
                            let a3332 = f826(match v0.clone() {
                                Adt159::C0(_, _, _, _, _, _, h3330, _) => {
                                    h3330
                                }
                            })?;
                            let a3333 = f826(match v0 {
                                Adt159::C0(_, _, _, _, _, _, _, h3331) => {
                                    h3331
                                }
                            })?;
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
        append_bytes(a3342, a3343)
    };
    Ok(append_bytes(a3344, a3345))
}

pub fn f821(v0: Bytes, v1: Bytes) -> R<Adt159> {
    let v2: Adt160 = f827(v0)?;
    let v3: Adt159 = f828(f829(f830(), f831(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f829(f829(f830(), f831(v2, 1549556828u32))?, f832(v3))
}

pub fn f822(v0: u64) -> Bytes {
    let a3371 = f338({
        let a3349 = {
            let a3346 = v0;
            let a3347 = 16777216u64;
            let a3348 = 0u64;
            nat_quot(a3346, a3347, a3348)
        };
        let a3350 = 256u64;
        let a3351 = 0u64;
        nat_rem(a3349, a3350, a3351)
    });
    let a3372 = {
        let a3369 = f338({
            let a3355 = {
                let a3352 = v0;
                let a3353 = 65536u64;
                let a3354 = 0u64;
                nat_quot(a3352, a3353, a3354)
            };
            let a3356 = 256u64;
            let a3357 = 0u64;
            nat_rem(a3355, a3356, a3357)
        });
        let a3370 = {
            let a3367 = f338({
                let a3361 = {
                    let a3358 = v0;
                    let a3359 = 256u64;
                    let a3360 = 0u64;
                    nat_quot(a3358, a3359, a3360)
                };
                let a3362 = 256u64;
                let a3363 = 0u64;
                nat_rem(a3361, a3362, a3363)
            });
            let a3368 = f338({
                let a3364 = v0;
                let a3365 = 256u64;
                let a3366 = 0u64;
                nat_rem(a3364, a3365, a3366)
            });
            append_bytes(a3367, a3368)
        };
        append_bytes(a3369, a3370)
    };
    append_bytes(a3371, a3372)
}

pub fn f823(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3373) = f807(Fn4::F833(v0.clone(), v1), {
        let a3374 = v0;
        length_bytes(a3374)
    }, (0u64, 0u64))?;
    Ok(h3373)
}

pub fn f824(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f834(f56(v0.clone(), {
        let (h3375, _) = v1.clone();
        h3375
    })?);
    let v3: u64 = f834(f56(v0, {
        let a3377 = {
            let (h3376, _) = v1.clone();
            h3376
        };
        let a3378 = 1u64;
        nat_add(a3377, a3378)?
    })?);
    let a3379 = v2;
    let a3380 = 16u64;
    if if nat_lt(a3379, a3380) {
        let a3381 = v3;
        let a3382 = 16u64;
        nat_lt(a3381, a3382)
    } else {
        false
    } {
        Ok(({
            let a3384 = {
                let (h3383, _) = v1.clone();
                h3383
            };
            let a3385 = 2u64;
            nat_add(a3384, a3385)?
        }, ({
            let a3392 = {
                let (h3386, _) = {
                    let (_, h3387) = v1.clone();
                    h3387
                };
                h3386
            };
            let a3393 = f338({
                let a3390 = {
                    let a3388 = v2;
                    let a3389 = 16u64;
                    nat_mul(a3388, a3389)?
                };
                let a3391 = v3;
                nat_add(a3390, a3391)?
            });
            append_bytes(a3392, a3393)
        }, {
            let (_, h3394) = {
                let (_, h3395) = v1;
                h3395
            };
            h3394
        })))
    } else {
        Ok(({
            let a3397 = {
                let (h3396, _) = v1.clone();
                h3396
            };
            let a3398 = 2u64;
            nat_add(a3397, a3398)?
        }, ({
            let (h3399, _) = {
                let (_, h3400) = v1.clone();
                h3400
            };
            h3399
        }, {
            let a3403 = {
                let (_, h3401) = {
                    let (_, h3402) = v1;
                    h3402
                };
                h3401
            };
            let a3404 = 1u64;
            nat_add(a3403, a3404)?
        })))
    }
}

pub fn f825(v0: u64) -> R<Bytes> {
    Ok(f338({
        let a3405 = v0;
        let a3406 = 10u64;
        if nat_lt(a3405, a3406) {
            let a3407 = 48u64;
            let a3408 = v0;
            nat_add(a3407, a3408)?
        } else {
            let a3409 = 87u64;
            let a3410 = v0;
            nat_add(a3409, a3410)?
        }
    }))
}

pub fn f826(v0: u32) -> R<Bytes> {
    let a3415 = f835(v0, 24u32)?;
    let a3416 = {
        let a3413 = f835(v0, 16u32)?;
        let a3414 = {
            let a3411 = f835(v0, 8u32)?;
            let a3412 = f835(v0, 0u32)?;
            append_bytes(a3411, a3412)
        };
        append_bytes(a3413, a3414)
    };
    Ok(append_bytes(a3415, a3416))
}

pub fn f827(v0: Bytes) -> R<Adt160> {
    let a3418 = f836();
    let a3419 = {
        let a3417 = v0.clone();
        length_bytes(a3417)
    };
    if nat_lt(a3418, a3419) {
        Ok(f837(f838(v0)?))
    } else {
        f839(v0)
    }
}

pub fn f828(v0: Adt159, v1: u64, v2: Bytes) -> R<Adt159> {
    let v3: u64 = {
        let a3420 = v2.clone();
        length_bytes(a3420)
    };
    let v4: u64 = {
        let a3426 = {
            let a3423 = {
                let a3421 = v3;
                let a3422 = 72u64;
                nat_add(a3421, a3422)?
            };
            let a3424 = 64u64;
            let a3425 = 0u64;
            nat_quot(a3423, a3424, a3425)
        };
        let a3427 = 64u64;
        nat_mul(a3426, a3427)?
    };
    let (_, h3428) = f840(Fn7::F841(v2, v1, v3, v4), {
        let a3429 = v4;
        let a3430 = 64u64;
        let a3431 = 0u64;
        nat_quot(a3429, a3430, a3431)
    }, (0u64, v0))?;
    Ok(h3428)
}

pub fn f829(v0: Adt159, v1: Adt160) -> R<Adt159> {
    let v2: Adt161 = f842(Fn8::F843, 64u64, Adt161::C0(match v0.clone() {
        Adt159::C0(h3432, _, _, _, _, _, _, _) => {
            h3432
        }
    }, match v0.clone() {
        Adt159::C0(_, h3433, _, _, _, _, _, _) => {
            h3433
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3434, _, _, _, _, _) => {
            h3434
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3435, _, _, _, _) => {
            h3435
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3436, _, _, _) => {
            h3436
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3437, _, _) => {
            h3437
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3438, _) => {
            h3438
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, _, h3439) => {
            h3439
        }
    }, v1, 0u64))?;
    Ok(Adt159::C0(f844(match v0.clone() {
        Adt159::C0(h3440, _, _, _, _, _, _, _) => {
            h3440
        }
    }, match v2.clone() {
        Adt161::C0(h3441, _, _, _, _, _, _, _, _, _) => {
            h3441
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, h3442, _, _, _, _, _, _) => {
            h3442
        }
    }, match v2.clone() {
        Adt161::C0(_, h3443, _, _, _, _, _, _, _, _) => {
            h3443
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, h3444, _, _, _, _, _) => {
            h3444
        }
    }, match v2.clone() {
        Adt161::C0(_, _, h3445, _, _, _, _, _, _, _) => {
            h3445
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, _, h3446, _, _, _, _) => {
            h3446
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, h3447, _, _, _, _, _, _) => {
            h3447
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, _, _, h3448, _, _, _) => {
            h3448
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, h3449, _, _, _, _, _) => {
            h3449
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3450, _, _) => {
            h3450
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, h3451, _, _, _, _) => {
            h3451
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3452, _) => {
            h3452
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, _, h3453, _, _, _) => {
            h3453
        }
    }), f844(match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3454) => {
            h3454
        }
    }, match v2 {
        Adt161::C0(_, _, _, _, _, _, _, h3455, _, _) => {
            h3455
        }
    })))
}

pub fn f830() -> Adt159 {
    Adt159::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f831(v0: Adt160, v1: u32) -> Adt160 {
    Adt160::C0({
        let a3457 = match v0.clone() {
            Adt160::C0(h3456, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3456
            }
        };
        let a3458 = v1;
        fixed_u32::bit_xor(a3457, a3458)
    }, {
        let a3460 = match v0.clone() {
            Adt160::C0(_, h3459, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3459
            }
        };
        let a3461 = v1;
        fixed_u32::bit_xor(a3460, a3461)
    }, {
        let a3463 = match v0.clone() {
            Adt160::C0(_, _, h3462, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3462
            }
        };
        let a3464 = v1;
        fixed_u32::bit_xor(a3463, a3464)
    }, {
        let a3466 = match v0.clone() {
            Adt160::C0(_, _, _, h3465, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3465
            }
        };
        let a3467 = v1;
        fixed_u32::bit_xor(a3466, a3467)
    }, {
        let a3469 = match v0.clone() {
            Adt160::C0(_, _, _, _, h3468, _, _, _, _, _, _, _, _, _, _, _) => {
                h3468
            }
        };
        let a3470 = v1;
        fixed_u32::bit_xor(a3469, a3470)
    }, {
        let a3472 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, h3471, _, _, _, _, _, _, _, _, _, _) => {
                h3471
            }
        };
        let a3473 = v1;
        fixed_u32::bit_xor(a3472, a3473)
    }, {
        let a3475 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, h3474, _, _, _, _, _, _, _, _, _) => {
                h3474
            }
        };
        let a3476 = v1;
        fixed_u32::bit_xor(a3475, a3476)
    }, {
        let a3478 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, h3477, _, _, _, _, _, _, _, _) => {
                h3477
            }
        };
        let a3479 = v1;
        fixed_u32::bit_xor(a3478, a3479)
    }, {
        let a3481 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, h3480, _, _, _, _, _, _, _) => {
                h3480
            }
        };
        let a3482 = v1;
        fixed_u32::bit_xor(a3481, a3482)
    }, {
        let a3484 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, h3483, _, _, _, _, _, _) => {
                h3483
            }
        };
        let a3485 = v1;
        fixed_u32::bit_xor(a3484, a3485)
    }, {
        let a3487 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, h3486, _, _, _, _, _) => {
                h3486
            }
        };
        let a3488 = v1;
        fixed_u32::bit_xor(a3487, a3488)
    }, {
        let a3490 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h3489, _, _, _, _) => {
                h3489
            }
        };
        let a3491 = v1;
        fixed_u32::bit_xor(a3490, a3491)
    }, {
        let a3493 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3492, _, _, _) => {
                h3492
            }
        };
        let a3494 = v1;
        fixed_u32::bit_xor(a3493, a3494)
    }, {
        let a3496 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3495, _, _) => {
                h3495
            }
        };
        let a3497 = v1;
        fixed_u32::bit_xor(a3496, a3497)
    }, {
        let a3499 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3498, _) => {
                h3498
            }
        };
        let a3500 = v1;
        fixed_u32::bit_xor(a3499, a3500)
    }, {
        let a3502 = match v0 {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3501) => {
                h3501
            }
        };
        let a3503 = v1;
        fixed_u32::bit_xor(a3502, a3503)
    })
}

pub fn f832(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3504, _, _, _, _, _, _, _) => {
            h3504
        }
    }, match v0.clone() {
        Adt159::C0(_, h3505, _, _, _, _, _, _) => {
            h3505
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3506, _, _, _, _, _) => {
            h3506
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3507, _, _, _, _) => {
            h3507
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3508, _, _, _) => {
            h3508
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3509, _, _) => {
            h3509
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3510, _) => {
            h3510
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3511) => {
            h3511
        }
    }, 2147483648u32, f845(), f845(), f845(), f845(), f845(), f845(), 768u32)
}

pub fn f833(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3513 = {
            let (h3512, _) = v2;
            h3512
        };
        let a3514 = 1u64;
        nat_add(a3513, a3514)?
    }, {
        let a3566 = {
            let (_, h3515) = v2;
            h3515
        };
        let a3567 = {
            let v3: u8 = {
                let a3518 = f846(v0, {
                    let (h3516, _) = v2;
                    h3516
                });
                let a3519 = f846(v1, {
                    let (h3517, _) = v2;
                    h3517
                });
                fixed_u8::bit_xor(a3518, a3519)
            };
            let a3564 = {
                let a3540 = {
                    let a3528 = {
                        let a3522 = {
                            let a3520 = v3;
                            let a3521 = 128u8;
                            fixed_u8::bit_and(a3520, a3521)
                        };
                        let a3523 = 128u8;
                        if equal(a3522, a3523) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3529 = {
                        let a3526 = {
                            let a3524 = v3;
                            let a3525 = 64u8;
                            fixed_u8::bit_and(a3524, a3525)
                        };
                        let a3527 = 64u8;
                        if equal(a3526, a3527) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3528, a3529)?
                };
                let a3541 = {
                    let a3538 = {
                        let a3532 = {
                            let a3530 = v3;
                            let a3531 = 32u8;
                            fixed_u8::bit_and(a3530, a3531)
                        };
                        let a3533 = 32u8;
                        if equal(a3532, a3533) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3539 = {
                        let a3536 = {
                            let a3534 = v3;
                            let a3535 = 16u8;
                            fixed_u8::bit_and(a3534, a3535)
                        };
                        let a3537 = 16u8;
                        if equal(a3536, a3537) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3538, a3539)?
                };
                nat_add(a3540, a3541)?
            };
            let a3565 = {
                let a3562 = {
                    let a3550 = {
                        let a3544 = {
                            let a3542 = v3;
                            let a3543 = 8u8;
                            fixed_u8::bit_and(a3542, a3543)
                        };
                        let a3545 = 8u8;
                        if equal(a3544, a3545) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3551 = {
                        let a3548 = {
                            let a3546 = v3;
                            let a3547 = 4u8;
                            fixed_u8::bit_and(a3546, a3547)
                        };
                        let a3549 = 4u8;
                        if equal(a3548, a3549) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3550, a3551)?
                };
                let a3563 = {
                    let a3560 = {
                        let a3554 = {
                            let a3552 = v3;
                            let a3553 = 2u8;
                            fixed_u8::bit_and(a3552, a3553)
                        };
                        let a3555 = 2u8;
                        if equal(a3554, a3555) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3561 = {
                        let a3558 = {
                            let a3556 = v3;
                            let a3557 = 1u8;
                            fixed_u8::bit_and(a3556, a3557)
                        };
                        let a3559 = 1u8;
                        if equal(a3558, a3559) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3560, a3561)?
                };
                nat_add(a3562, a3563)?
            };
            nat_add(a3564, a3565)?
        };
        nat_add(a3566, a3567)?
    }))
}

pub fn f834(v0: u64) -> u64 {
    let a3568 = v0;
    let a3569 = 48u64;
    if nat_lt(a3568, a3569) {
        16u64
    } else {
        let a3570 = v0;
        let a3571 = 58u64;
        if nat_lt(a3570, a3571) {
            let a3572 = v0;
            let a3573 = 48u64;
            nat_sub(a3572, a3573)
        } else {
            let a3574 = v0;
            let a3575 = 97u64;
            if nat_lt(a3574, a3575) {
                16u64
            } else {
                let a3576 = v0;
                let a3577 = 103u64;
                if nat_lt(a3576, a3577) {
                    let a3578 = v0;
                    let a3579 = 87u64;
                    nat_sub(a3578, a3579)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f835(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f338({
        let v2: u32 = {
            let a3580 = f847(v0, v1);
            let a3581 = 255u32;
            fixed_u32::bit_and(a3580, a3581)
        };
        let a3770 = {
            let a3674 = {
                let a3626 = {
                    let a3602 = {
                        let a3590 = {
                            let a3584 = {
                                let a3582 = v2;
                                let a3583 = 2147483648u32;
                                fixed_u32::bit_and(a3582, a3583)
                            };
                            let a3585 = 2147483648u32;
                            if equal(a3584, a3585) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3591 = {
                            let a3588 = {
                                let a3586 = v2;
                                let a3587 = 1073741824u32;
                                fixed_u32::bit_and(a3586, a3587)
                            };
                            let a3589 = 1073741824u32;
                            if equal(a3588, a3589) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3590, a3591)?
                    };
                    let a3603 = {
                        let a3600 = {
                            let a3594 = {
                                let a3592 = v2;
                                let a3593 = 536870912u32;
                                fixed_u32::bit_and(a3592, a3593)
                            };
                            let a3595 = 536870912u32;
                            if equal(a3594, a3595) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3601 = {
                            let a3598 = {
                                let a3596 = v2;
                                let a3597 = 268435456u32;
                                fixed_u32::bit_and(a3596, a3597)
                            };
                            let a3599 = 268435456u32;
                            if equal(a3598, a3599) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3600, a3601)?
                    };
                    nat_add(a3602, a3603)?
                };
                let a3627 = {
                    let a3624 = {
                        let a3612 = {
                            let a3606 = {
                                let a3604 = v2;
                                let a3605 = 134217728u32;
                                fixed_u32::bit_and(a3604, a3605)
                            };
                            let a3607 = 134217728u32;
                            if equal(a3606, a3607) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3613 = {
                            let a3610 = {
                                let a3608 = v2;
                                let a3609 = 67108864u32;
                                fixed_u32::bit_and(a3608, a3609)
                            };
                            let a3611 = 67108864u32;
                            if equal(a3610, a3611) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3612, a3613)?
                    };
                    let a3625 = {
                        let a3622 = {
                            let a3616 = {
                                let a3614 = v2;
                                let a3615 = 33554432u32;
                                fixed_u32::bit_and(a3614, a3615)
                            };
                            let a3617 = 33554432u32;
                            if equal(a3616, a3617) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3623 = {
                            let a3620 = {
                                let a3618 = v2;
                                let a3619 = 16777216u32;
                                fixed_u32::bit_and(a3618, a3619)
                            };
                            let a3621 = 16777216u32;
                            if equal(a3620, a3621) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3622, a3623)?
                    };
                    nat_add(a3624, a3625)?
                };
                nat_add(a3626, a3627)?
            };
            let a3675 = {
                let a3672 = {
                    let a3648 = {
                        let a3636 = {
                            let a3630 = {
                                let a3628 = v2;
                                let a3629 = 8388608u32;
                                fixed_u32::bit_and(a3628, a3629)
                            };
                            let a3631 = 8388608u32;
                            if equal(a3630, a3631) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3637 = {
                            let a3634 = {
                                let a3632 = v2;
                                let a3633 = 4194304u32;
                                fixed_u32::bit_and(a3632, a3633)
                            };
                            let a3635 = 4194304u32;
                            if equal(a3634, a3635) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3636, a3637)?
                    };
                    let a3649 = {
                        let a3646 = {
                            let a3640 = {
                                let a3638 = v2;
                                let a3639 = 2097152u32;
                                fixed_u32::bit_and(a3638, a3639)
                            };
                            let a3641 = 2097152u32;
                            if equal(a3640, a3641) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3647 = {
                            let a3644 = {
                                let a3642 = v2;
                                let a3643 = 1048576u32;
                                fixed_u32::bit_and(a3642, a3643)
                            };
                            let a3645 = 1048576u32;
                            if equal(a3644, a3645) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3646, a3647)?
                    };
                    nat_add(a3648, a3649)?
                };
                let a3673 = {
                    let a3670 = {
                        let a3658 = {
                            let a3652 = {
                                let a3650 = v2;
                                let a3651 = 524288u32;
                                fixed_u32::bit_and(a3650, a3651)
                            };
                            let a3653 = 524288u32;
                            if equal(a3652, a3653) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3659 = {
                            let a3656 = {
                                let a3654 = v2;
                                let a3655 = 262144u32;
                                fixed_u32::bit_and(a3654, a3655)
                            };
                            let a3657 = 262144u32;
                            if equal(a3656, a3657) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3658, a3659)?
                    };
                    let a3671 = {
                        let a3668 = {
                            let a3662 = {
                                let a3660 = v2;
                                let a3661 = 131072u32;
                                fixed_u32::bit_and(a3660, a3661)
                            };
                            let a3663 = 131072u32;
                            if equal(a3662, a3663) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3669 = {
                            let a3666 = {
                                let a3664 = v2;
                                let a3665 = 65536u32;
                                fixed_u32::bit_and(a3664, a3665)
                            };
                            let a3667 = 65536u32;
                            if equal(a3666, a3667) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3668, a3669)?
                    };
                    nat_add(a3670, a3671)?
                };
                nat_add(a3672, a3673)?
            };
            nat_add(a3674, a3675)?
        };
        let a3771 = {
            let a3768 = {
                let a3720 = {
                    let a3696 = {
                        let a3684 = {
                            let a3678 = {
                                let a3676 = v2;
                                let a3677 = 32768u32;
                                fixed_u32::bit_and(a3676, a3677)
                            };
                            let a3679 = 32768u32;
                            if equal(a3678, a3679) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3685 = {
                            let a3682 = {
                                let a3680 = v2;
                                let a3681 = 16384u32;
                                fixed_u32::bit_and(a3680, a3681)
                            };
                            let a3683 = 16384u32;
                            if equal(a3682, a3683) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3684, a3685)?
                    };
                    let a3697 = {
                        let a3694 = {
                            let a3688 = {
                                let a3686 = v2;
                                let a3687 = 8192u32;
                                fixed_u32::bit_and(a3686, a3687)
                            };
                            let a3689 = 8192u32;
                            if equal(a3688, a3689) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3695 = {
                            let a3692 = {
                                let a3690 = v2;
                                let a3691 = 4096u32;
                                fixed_u32::bit_and(a3690, a3691)
                            };
                            let a3693 = 4096u32;
                            if equal(a3692, a3693) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3694, a3695)?
                    };
                    nat_add(a3696, a3697)?
                };
                let a3721 = {
                    let a3718 = {
                        let a3706 = {
                            let a3700 = {
                                let a3698 = v2;
                                let a3699 = 2048u32;
                                fixed_u32::bit_and(a3698, a3699)
                            };
                            let a3701 = 2048u32;
                            if equal(a3700, a3701) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3707 = {
                            let a3704 = {
                                let a3702 = v2;
                                let a3703 = 1024u32;
                                fixed_u32::bit_and(a3702, a3703)
                            };
                            let a3705 = 1024u32;
                            if equal(a3704, a3705) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3706, a3707)?
                    };
                    let a3719 = {
                        let a3716 = {
                            let a3710 = {
                                let a3708 = v2;
                                let a3709 = 512u32;
                                fixed_u32::bit_and(a3708, a3709)
                            };
                            let a3711 = 512u32;
                            if equal(a3710, a3711) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3717 = {
                            let a3714 = {
                                let a3712 = v2;
                                let a3713 = 256u32;
                                fixed_u32::bit_and(a3712, a3713)
                            };
                            let a3715 = 256u32;
                            if equal(a3714, a3715) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3716, a3717)?
                    };
                    nat_add(a3718, a3719)?
                };
                nat_add(a3720, a3721)?
            };
            let a3769 = {
                let a3766 = {
                    let a3742 = {
                        let a3730 = {
                            let a3724 = {
                                let a3722 = v2;
                                let a3723 = 128u32;
                                fixed_u32::bit_and(a3722, a3723)
                            };
                            let a3725 = 128u32;
                            if equal(a3724, a3725) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3731 = {
                            let a3728 = {
                                let a3726 = v2;
                                let a3727 = 64u32;
                                fixed_u32::bit_and(a3726, a3727)
                            };
                            let a3729 = 64u32;
                            if equal(a3728, a3729) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3730, a3731)?
                    };
                    let a3743 = {
                        let a3740 = {
                            let a3734 = {
                                let a3732 = v2;
                                let a3733 = 32u32;
                                fixed_u32::bit_and(a3732, a3733)
                            };
                            let a3735 = 32u32;
                            if equal(a3734, a3735) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3741 = {
                            let a3738 = {
                                let a3736 = v2;
                                let a3737 = 16u32;
                                fixed_u32::bit_and(a3736, a3737)
                            };
                            let a3739 = 16u32;
                            if equal(a3738, a3739) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3740, a3741)?
                    };
                    nat_add(a3742, a3743)?
                };
                let a3767 = {
                    let a3764 = {
                        let a3752 = {
                            let a3746 = {
                                let a3744 = v2;
                                let a3745 = 8u32;
                                fixed_u32::bit_and(a3744, a3745)
                            };
                            let a3747 = 8u32;
                            if equal(a3746, a3747) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3753 = {
                            let a3750 = {
                                let a3748 = v2;
                                let a3749 = 4u32;
                                fixed_u32::bit_and(a3748, a3749)
                            };
                            let a3751 = 4u32;
                            if equal(a3750, a3751) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3752, a3753)?
                    };
                    let a3765 = {
                        let a3762 = {
                            let a3756 = {
                                let a3754 = v2;
                                let a3755 = 2u32;
                                fixed_u32::bit_and(a3754, a3755)
                            };
                            let a3757 = 2u32;
                            if equal(a3756, a3757) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3763 = {
                            let a3760 = {
                                let a3758 = v2;
                                let a3759 = 1u32;
                                fixed_u32::bit_and(a3758, a3759)
                            };
                            let a3761 = 1u32;
                            if equal(a3760, a3761) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3762, a3763)?
                    };
                    nat_add(a3764, a3765)?
                };
                nat_add(a3766, a3767)?
            };
            nat_add(a3768, a3769)?
        };
        nat_add(a3770, a3771)?
    }))
}

pub fn f836() -> u64 {
    64u64
}

pub fn f837(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3772, _, _, _, _, _, _, _) => {
            h3772
        }
    }, match v0.clone() {
        Adt159::C0(_, h3773, _, _, _, _, _, _) => {
            h3773
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3774, _, _, _, _, _) => {
            h3774
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3775, _, _, _, _) => {
            h3775
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3776, _, _, _) => {
            h3776
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3777, _, _) => {
            h3777
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3778, _) => {
            h3778
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3779) => {
            h3779
        }
    }, f845(), f845(), f845(), f845(), f845(), f845(), f845(), f845())
}

pub fn f838(v0: Bytes) -> R<Adt159> {
    let v1: u64 = {
        let a3780 = v0.clone();
        length_bytes(a3780)
    };
    let v2: u64 = {
        let a3786 = {
            let a3783 = {
                let a3781 = v1;
                let a3782 = 72u64;
                nat_add(a3781, a3782)?
            };
            let a3784 = 64u64;
            let a3785 = 0u64;
            nat_quot(a3783, a3784, a3785)
        };
        let a3787 = 64u64;
        nat_mul(a3786, a3787)?
    };
    let (_, h3788) = f840(Fn7::F848(v0, v1, v2), {
        let a3789 = v2;
        let a3790 = 64u64;
        let a3791 = 0u64;
        nat_quot(a3789, a3790, a3791)
    }, (0u64, f830()))?;
    Ok(h3788)
}

pub fn f839(v0: Bytes) -> R<Adt160> {
    Ok(Adt160::C0(f849(v0.clone(), 0u64)?, f849(v0.clone(), 4u64)?, f849(v0.clone(), 8u64)?, f849(v0.clone(), 12u64)?, f849(v0.clone(), 16u64)?, f849(v0.clone(), 20u64)?, f849(v0.clone(), 24u64)?, f849(v0.clone(), 28u64)?, f849(v0.clone(), 32u64)?, f849(v0.clone(), 36u64)?, f849(v0.clone(), 40u64)?, f849(v0.clone(), 44u64)?, f849(v0.clone(), 48u64)?, f849(v0.clone(), 52u64)?, f849(v0.clone(), 56u64)?, f849(v0, 60u64)?))
}

pub fn f840(v0: Fn7, p3795: u64, p3796: (u64, Adt159)) -> R<(u64, Adt159)> {
    let m3794 = (p3795, p3796);
    {
        let mut m3794 = m3794;
        loop {
            let step = {
                let (v1, v2) = m3794;
                let m3792 = v1;
                if m3792 == 0 {
                    Err::<(u64, (u64, Adt159)), R<(u64, Adt159)>>(Ok(v2))
                } else {
                    let v3 = m3792 - 1;
                    Ok::<(u64, (u64, Adt159)), R<(u64, Adt159)>>((v3, {
                        let c3793: Fn7 = v0.clone();
                        c3793.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3794 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f841(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3798 = {
            let (h3797, _) = v4.clone();
            h3797
        };
        let a3799 = 64u64;
        nat_add(a3798, a3799)?
    }, f829({
        let (_, h3800) = v4;
        h3800
    }, f850(v0, v2, v3, {
        let a3803 = {
            let a3801 = v1;
            let a3802 = v2;
            nat_add(a3801, a3802)?
        };
        let a3804 = 8u64;
        nat_mul(a3803, a3804)?
    }, {
        let (h3805, _) = v4;
        h3805
    })?)?))
}

pub fn f842(v0: Fn8, p3809: u64, p3810: Adt161) -> R<Adt161> {
    let m3808 = (p3809, p3810);
    {
        let mut m3808 = m3808;
        loop {
            let step = {
                let (v1, v2) = m3808;
                let m3806 = v1;
                if m3806 == 0 {
                    Err::<(u64, Adt161), R<Adt161>>(Ok(v2))
                } else {
                    let v3 = m3806 - 1;
                    Ok::<(u64, Adt161), R<Adt161>>((v3, {
                        let c3807: Fn8 = v0.clone();
                        c3807.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3808 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f843(v0: Adt161) -> R<Adt161> {
    f851(v0)
}

pub fn f844(v0: u32, v1: u32) -> u32 {
    let m3813 = {
        let a3811 = f852(v0);
        let a3812 = f852(v1);
        fixed_u64::checked_add(a3811, a3812)
    };
    match m3813 {
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
    let m3816 = {
        let a3814 = v0;
        let a3815 = v1;
        index_bytes(a3814, a3815)
    };
    match m3816 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f847(v0: u32, v1: u32) -> u32 {
    let m3819 = {
        let a3817 = v0;
        let a3818 = v1;
        fixed_u32::shift_right(a3817, a3818)
    };
    match m3819 {
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
        let a3821 = {
            let (h3820, _) = v3.clone();
            h3820
        };
        let a3822 = 64u64;
        nat_add(a3821, a3822)?
    }, f829({
        let (_, h3823) = v3;
        h3823
    }, f854(v0, v1, v2, {
        let (h3824, _) = v3;
        h3824
    })?)?))
}

pub fn f849(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f855(f856(v0.clone(), v1), f856(v0.clone(), {
        let a3825 = v1;
        let a3826 = 1u64;
        nat_add(a3825, a3826)?
    }), f856(v0.clone(), {
        let a3827 = v1;
        let a3828 = 2u64;
        nat_add(a3827, a3828)?
    }), f856(v0, {
        let a3829 = v1;
        let a3830 = 3u64;
        nat_add(a3829, a3830)?
    })))
}

pub fn f850(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt160> {
    Ok(Adt160::C0(f857(v0.clone(), v1, v2, v3, v4)?, f857(v0.clone(), v1, v2, v3, {
        let a3831 = v4;
        let a3832 = 4u64;
        nat_add(a3831, a3832)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3833 = v4;
        let a3834 = 8u64;
        nat_add(a3833, a3834)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3835 = v4;
        let a3836 = 12u64;
        nat_add(a3835, a3836)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3837 = v4;
        let a3838 = 16u64;
        nat_add(a3837, a3838)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3839 = v4;
        let a3840 = 20u64;
        nat_add(a3839, a3840)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3841 = v4;
        let a3842 = 24u64;
        nat_add(a3841, a3842)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3843 = v4;
        let a3844 = 28u64;
        nat_add(a3843, a3844)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3845 = v4;
        let a3846 = 32u64;
        nat_add(a3845, a3846)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3847 = v4;
        let a3848 = 36u64;
        nat_add(a3847, a3848)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3849 = v4;
        let a3850 = 40u64;
        nat_add(a3849, a3850)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3851 = v4;
        let a3852 = 44u64;
        nat_add(a3851, a3852)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3853 = v4;
        let a3854 = 48u64;
        nat_add(a3853, a3854)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3855 = v4;
        let a3856 = 52u64;
        nat_add(a3855, a3856)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3857 = v4;
        let a3858 = 56u64;
        nat_add(a3857, a3858)?
    })?, f857(v0, v1, v2, v3, {
        let a3859 = v4;
        let a3860 = 60u64;
        nat_add(a3859, a3860)?
    })?))
}

pub fn f851(v0: Adt161) -> R<Adt161> {
    let v1: u32 = f858(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, _, h3861) => {
            h3861
        }
    });
    let v2: u32 = f844(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, h3862, _, _) => {
            h3862
        }
    }, f844(f859(match v0.clone() {
        Adt161::C0(_, _, _, _, h3863, _, _, _, _, _) => {
            h3863
        }
    }), f844(f860(match v0.clone() {
        Adt161::C0(_, _, _, _, h3864, _, _, _, _, _) => {
            h3864
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3865, _, _, _, _) => {
            h3865
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3866, _, _, _) => {
            h3866
        }
    }), f844(v1, {
        let m3869 = match v0.clone() {
            Adt161::C0(_, _, _, _, _, _, _, _, h3868, _) => {
                h3868
            }
        };
        match m3869 {
            Adt160::C0(h3867, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3867
            }
        }
    }))));
    let v3: u32 = f844(f861(match v0.clone() {
        Adt161::C0(h3870, _, _, _, _, _, _, _, _, _) => {
            h3870
        }
    }), f862(match v0.clone() {
        Adt161::C0(h3871, _, _, _, _, _, _, _, _, _) => {
            h3871
        }
    }, match v0.clone() {
        Adt161::C0(_, h3872, _, _, _, _, _, _, _, _) => {
            h3872
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3873, _, _, _, _, _, _, _) => {
            h3873
        }
    }));
    Ok(Adt161::C0(f844(v2, v3), match v0.clone() {
        Adt161::C0(h3874, _, _, _, _, _, _, _, _, _) => {
            h3874
        }
    }, match v0.clone() {
        Adt161::C0(_, h3875, _, _, _, _, _, _, _, _) => {
            h3875
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3876, _, _, _, _, _, _, _) => {
            h3876
        }
    }, f844(match v0.clone() {
        Adt161::C0(_, _, _, h3877, _, _, _, _, _, _) => {
            h3877
        }
    }, v2), match v0.clone() {
        Adt161::C0(_, _, _, _, h3878, _, _, _, _, _) => {
            h3878
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3879, _, _, _, _) => {
            h3879
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3880, _, _, _) => {
            h3880
        }
    }, f863(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, h3881, _) => {
            h3881
        }
    }), {
        let a3883 = match v0 {
            Adt161::C0(_, _, _, _, _, _, _, _, _, h3882) => {
                h3882
            }
        };
        let a3884 = 1u64;
        nat_add(a3883, a3884)?
    }))
}

pub fn f852(v0: u32) -> u64 {
    let m3886 = {
        let a3885 = v0;
        fixed_u64::convert(i128::from(a3885))
    };
    match m3886 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f853(v0: u64) -> u32 {
    let m3890 = {
        let a3889 = {
            let a3887 = v0;
            let a3888 = 4294967295u64;
            fixed_u64::bit_and(a3887, a3888)
        };
        fixed_u32::convert(i128::from(a3889))
    };
    match m3890 {
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
        let a3891 = v3;
        let a3892 = 4u64;
        nat_add(a3891, a3892)?
    })?, f864(v0.clone(), v1, v2, {
        let a3893 = v3;
        let a3894 = 8u64;
        nat_add(a3893, a3894)?
    })?, f864(v0.clone(), v1, v2, {
        let a3895 = v3;
        let a3896 = 12u64;
        nat_add(a3895, a3896)?
    })?, f864(v0.clone(), v1, v2, {
        let a3897 = v3;
        let a3898 = 16u64;
        nat_add(a3897, a3898)?
    })?, f864(v0.clone(), v1, v2, {
        let a3899 = v3;
        let a3900 = 20u64;
        nat_add(a3899, a3900)?
    })?, f864(v0.clone(), v1, v2, {
        let a3901 = v3;
        let a3902 = 24u64;
        nat_add(a3901, a3902)?
    })?, f864(v0.clone(), v1, v2, {
        let a3903 = v3;
        let a3904 = 28u64;
        nat_add(a3903, a3904)?
    })?, f864(v0.clone(), v1, v2, {
        let a3905 = v3;
        let a3906 = 32u64;
        nat_add(a3905, a3906)?
    })?, f864(v0.clone(), v1, v2, {
        let a3907 = v3;
        let a3908 = 36u64;
        nat_add(a3907, a3908)?
    })?, f864(v0.clone(), v1, v2, {
        let a3909 = v3;
        let a3910 = 40u64;
        nat_add(a3909, a3910)?
    })?, f864(v0.clone(), v1, v2, {
        let a3911 = v3;
        let a3912 = 44u64;
        nat_add(a3911, a3912)?
    })?, f864(v0.clone(), v1, v2, {
        let a3913 = v3;
        let a3914 = 48u64;
        nat_add(a3913, a3914)?
    })?, f864(v0.clone(), v1, v2, {
        let a3915 = v3;
        let a3916 = 52u64;
        nat_add(a3915, a3916)?
    })?, f864(v0.clone(), v1, v2, {
        let a3917 = v3;
        let a3918 = 56u64;
        nat_add(a3917, a3918)?
    })?, f864(v0, v1, v2, {
        let a3919 = v3;
        let a3920 = 60u64;
        nat_add(a3919, a3920)?
    })?))
}

pub fn f855(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3925 = f865(v0, 24u32);
    let a3926 = {
        let a3923 = f865(v1, 16u32);
        let a3924 = {
            let a3921 = f865(v2, 8u32);
            let a3922 = v3;
            fixed_u32::bit_or(a3921, a3922)
        };
        fixed_u32::bit_or(a3923, a3924)
    };
    fixed_u32::bit_or(a3925, a3926)
}

pub fn f856(v0: Bytes, v1: u64) -> u32 {
    let m3929 = {
        let a3927 = v0;
        let a3928 = v1;
        index_bytes(a3927, a3928)
    };
    match m3929 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3931 = {
                let a3930 = v2;
                fixed_u32::convert(i128::from(a3930))
            };
            match m3931 {
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
    let a3934 = {
        let a3932 = v4;
        let a3933 = 8u64;
        nat_add(a3932, a3933)?
    };
    let a3935 = v2;
    if nat_eq(a3934, a3935) {
        Ok(f866(v3, true))
    } else {
        let a3938 = {
            let a3936 = v4;
            let a3937 = 4u64;
            nat_add(a3936, a3937)?
        };
        let a3939 = v2;
        if nat_eq(a3938, a3939) {
            Ok(f866(v3, false))
        } else {
            Ok(f855(f867(v0.clone(), v1, v4), f867(v0.clone(), v1, {
                let a3940 = v4;
                let a3941 = 1u64;
                nat_add(a3940, a3941)?
            }), f867(v0.clone(), v1, {
                let a3942 = v4;
                let a3943 = 2u64;
                nat_add(a3942, a3943)?
            }), f867(v0, v1, {
                let a3944 = v4;
                let a3945 = 3u64;
                nat_add(a3944, a3945)?
            })))
        }
    }
}

pub fn f858(v0: u64) -> u32 {
    let a3946 = v0;
    let a3947 = 32u64;
    if nat_lt(a3946, a3947) {
        let a3948 = v0;
        let a3949 = 16u64;
        if nat_lt(a3948, a3949) {
            let a3950 = v0;
            let a3951 = 8u64;
            if nat_lt(a3950, a3951) {
                let a3952 = v0;
                let a3953 = 4u64;
                if nat_lt(a3952, a3953) {
                    let a3954 = v0;
                    let a3955 = 2u64;
                    if nat_lt(a3954, a3955) {
                        let a3956 = v0;
                        let a3957 = 1u64;
                        if nat_lt(a3956, a3957) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3958 = v0;
                        let a3959 = 3u64;
                        if nat_lt(a3958, a3959) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3960 = v0;
                    let a3961 = 6u64;
                    if nat_lt(a3960, a3961) {
                        let a3962 = v0;
                        let a3963 = 5u64;
                        if nat_lt(a3962, a3963) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3964 = v0;
                        let a3965 = 7u64;
                        if nat_lt(a3964, a3965) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3966 = v0;
                let a3967 = 12u64;
                if nat_lt(a3966, a3967) {
                    let a3968 = v0;
                    let a3969 = 10u64;
                    if nat_lt(a3968, a3969) {
                        let a3970 = v0;
                        let a3971 = 9u64;
                        if nat_lt(a3970, a3971) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3972 = v0;
                        let a3973 = 11u64;
                        if nat_lt(a3972, a3973) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3974 = v0;
                    let a3975 = 14u64;
                    if nat_lt(a3974, a3975) {
                        let a3976 = v0;
                        let a3977 = 13u64;
                        if nat_lt(a3976, a3977) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3978 = v0;
                        let a3979 = 15u64;
                        if nat_lt(a3978, a3979) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3980 = v0;
            let a3981 = 24u64;
            if nat_lt(a3980, a3981) {
                let a3982 = v0;
                let a3983 = 20u64;
                if nat_lt(a3982, a3983) {
                    let a3984 = v0;
                    let a3985 = 18u64;
                    if nat_lt(a3984, a3985) {
                        let a3986 = v0;
                        let a3987 = 17u64;
                        if nat_lt(a3986, a3987) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3988 = v0;
                        let a3989 = 19u64;
                        if nat_lt(a3988, a3989) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3990 = v0;
                    let a3991 = 22u64;
                    if nat_lt(a3990, a3991) {
                        let a3992 = v0;
                        let a3993 = 21u64;
                        if nat_lt(a3992, a3993) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3994 = v0;
                        let a3995 = 23u64;
                        if nat_lt(a3994, a3995) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3996 = v0;
                let a3997 = 28u64;
                if nat_lt(a3996, a3997) {
                    let a3998 = v0;
                    let a3999 = 26u64;
                    if nat_lt(a3998, a3999) {
                        let a4000 = v0;
                        let a4001 = 25u64;
                        if nat_lt(a4000, a4001) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4002 = v0;
                        let a4003 = 27u64;
                        if nat_lt(a4002, a4003) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4004 = v0;
                    let a4005 = 30u64;
                    if nat_lt(a4004, a4005) {
                        let a4006 = v0;
                        let a4007 = 29u64;
                        if nat_lt(a4006, a4007) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4008 = v0;
                        let a4009 = 31u64;
                        if nat_lt(a4008, a4009) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4010 = v0;
        let a4011 = 48u64;
        if nat_lt(a4010, a4011) {
            let a4012 = v0;
            let a4013 = 40u64;
            if nat_lt(a4012, a4013) {
                let a4014 = v0;
                let a4015 = 36u64;
                if nat_lt(a4014, a4015) {
                    let a4016 = v0;
                    let a4017 = 34u64;
                    if nat_lt(a4016, a4017) {
                        let a4018 = v0;
                        let a4019 = 33u64;
                        if nat_lt(a4018, a4019) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4020 = v0;
                        let a4021 = 35u64;
                        if nat_lt(a4020, a4021) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4022 = v0;
                    let a4023 = 38u64;
                    if nat_lt(a4022, a4023) {
                        let a4024 = v0;
                        let a4025 = 37u64;
                        if nat_lt(a4024, a4025) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4026 = v0;
                        let a4027 = 39u64;
                        if nat_lt(a4026, a4027) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4028 = v0;
                let a4029 = 44u64;
                if nat_lt(a4028, a4029) {
                    let a4030 = v0;
                    let a4031 = 42u64;
                    if nat_lt(a4030, a4031) {
                        let a4032 = v0;
                        let a4033 = 41u64;
                        if nat_lt(a4032, a4033) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4034 = v0;
                        let a4035 = 43u64;
                        if nat_lt(a4034, a4035) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4036 = v0;
                    let a4037 = 46u64;
                    if nat_lt(a4036, a4037) {
                        let a4038 = v0;
                        let a4039 = 45u64;
                        if nat_lt(a4038, a4039) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4040 = v0;
                        let a4041 = 47u64;
                        if nat_lt(a4040, a4041) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4042 = v0;
            let a4043 = 56u64;
            if nat_lt(a4042, a4043) {
                let a4044 = v0;
                let a4045 = 52u64;
                if nat_lt(a4044, a4045) {
                    let a4046 = v0;
                    let a4047 = 50u64;
                    if nat_lt(a4046, a4047) {
                        let a4048 = v0;
                        let a4049 = 49u64;
                        if nat_lt(a4048, a4049) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4050 = v0;
                        let a4051 = 51u64;
                        if nat_lt(a4050, a4051) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4052 = v0;
                    let a4053 = 54u64;
                    if nat_lt(a4052, a4053) {
                        let a4054 = v0;
                        let a4055 = 53u64;
                        if nat_lt(a4054, a4055) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4056 = v0;
                        let a4057 = 55u64;
                        if nat_lt(a4056, a4057) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4058 = v0;
                let a4059 = 60u64;
                if nat_lt(a4058, a4059) {
                    let a4060 = v0;
                    let a4061 = 58u64;
                    if nat_lt(a4060, a4061) {
                        let a4062 = v0;
                        let a4063 = 57u64;
                        if nat_lt(a4062, a4063) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4064 = v0;
                        let a4065 = 59u64;
                        if nat_lt(a4064, a4065) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4066 = v0;
                    let a4067 = 62u64;
                    if nat_lt(a4066, a4067) {
                        let a4068 = v0;
                        let a4069 = 61u64;
                        if nat_lt(a4068, a4069) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4070 = v0;
                        let a4071 = 63u64;
                        if nat_lt(a4070, a4071) {
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
    let a4074 = {
        let a4072 = f868(v0, 6u32, 26u32);
        let a4073 = f868(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4072, a4073)
    };
    let a4075 = f868(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4074, a4075)
}

pub fn f860(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4081 = {
        let a4076 = v0;
        let a4077 = v1;
        fixed_u32::bit_and(a4076, a4077)
    };
    let a4082 = {
        let a4079 = {
            let a4078 = v0;
            fixed_u32::bit_not(a4078)
        };
        let a4080 = v2;
        fixed_u32::bit_and(a4079, a4080)
    };
    fixed_u32::bit_xor(a4081, a4082)
}

pub fn f861(v0: u32) -> u32 {
    let a4085 = {
        let a4083 = f868(v0, 2u32, 30u32);
        let a4084 = f868(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4083, a4084)
    };
    let a4086 = f868(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4085, a4086)
}

pub fn f862(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4095 = {
        let a4091 = {
            let a4087 = v0;
            let a4088 = v1;
            fixed_u32::bit_and(a4087, a4088)
        };
        let a4092 = {
            let a4089 = v0;
            let a4090 = v2;
            fixed_u32::bit_and(a4089, a4090)
        };
        fixed_u32::bit_xor(a4091, a4092)
    };
    let a4096 = {
        let a4093 = v1;
        let a4094 = v2;
        fixed_u32::bit_and(a4093, a4094)
    };
    fixed_u32::bit_xor(a4095, a4096)
}

pub fn f863(v0: Adt160) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt160::C0(_, h4097, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4097
        }
    }, match v0.clone() {
        Adt160::C0(_, _, h4098, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4098
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, h4099, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4099
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, h4100, _, _, _, _, _, _, _, _, _, _, _) => {
            h4100
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, h4101, _, _, _, _, _, _, _, _, _, _) => {
            h4101
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, h4102, _, _, _, _, _, _, _, _, _) => {
            h4102
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, h4103, _, _, _, _, _, _, _, _) => {
            h4103
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, h4104, _, _, _, _, _, _, _) => {
            h4104
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4105, _, _, _, _, _, _) => {
            h4105
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, h4106, _, _, _, _, _) => {
            h4106
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h4107, _, _, _, _) => {
            h4107
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4108, _, _, _) => {
            h4108
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4109, _, _) => {
            h4109
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4110, _) => {
            h4110
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4111) => {
            h4111
        }
    }, f844(f869(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4112, _) => {
            h4112
        }
    }), f844(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4113, _, _, _, _, _, _) => {
            h4113
        }
    }, f844(f870(match v0.clone() {
        Adt160::C0(_, h4114, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4114
        }
    }), match v0 {
        Adt160::C0(h4115, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4115
        }
    }))))
}

pub fn f864(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4118 = {
        let a4116 = v3;
        let a4117 = 8u64;
        nat_add(a4116, a4117)?
    };
    let a4119 = v2;
    if nat_eq(a4118, a4119) {
        Ok(f871({
            let a4122 = {
                let a4120 = v1;
                let a4121 = 8u64;
                nat_mul(a4120, a4121)?
            };
            let a4123 = 4294967296u64;
            let a4124 = 0u64;
            nat_quot(a4122, a4123, a4124)
        }))
    } else {
        let a4127 = {
            let a4125 = v3;
            let a4126 = 4u64;
            nat_add(a4125, a4126)?
        };
        let a4128 = v2;
        if nat_eq(a4127, a4128) {
            Ok(f871({
                let a4131 = {
                    let a4129 = v1;
                    let a4130 = 8u64;
                    nat_mul(a4129, a4130)?
                };
                let a4132 = 4294967296u64;
                let a4133 = 0u64;
                nat_rem(a4131, a4132, a4133)
            }))
        } else {
            Ok(f855(f867(v0.clone(), v1, v3), f867(v0.clone(), v1, {
                let a4134 = v3;
                let a4135 = 1u64;
                nat_add(a4134, a4135)?
            }), f867(v0.clone(), v1, {
                let a4136 = v3;
                let a4137 = 2u64;
                nat_add(a4136, a4137)?
            }), f867(v0, v1, {
                let a4138 = v3;
                let a4139 = 3u64;
                nat_add(a4138, a4139)?
            })))
        }
    }
}

pub fn f865(v0: u32, v1: u32) -> u32 {
    let m4142 = {
        let a4140 = v0;
        let a4141 = v1;
        fixed_u32::shift_left(a4140, a4141)
    };
    match m4142 {
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
            let a4143 = v0;
            let a4144 = 4294967296u64;
            let a4145 = 0u64;
            nat_quot(a4143, a4144, a4145)
        })
    } else {
        f871({
            let a4146 = v0;
            let a4147 = 4294967296u64;
            let a4148 = 0u64;
            nat_rem(a4146, a4147, a4148)
        })
    }
}

pub fn f867(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4149 = v2;
    let a4150 = v1;
    if nat_lt(a4149, a4150) {
        let m4153 = {
            let a4151 = v0;
            let a4152 = v2;
            index_bytes(a4151, a4152)
        };
        match m4153 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4155 = {
                    let a4154 = v3;
                    fixed_u32::convert(i128::from(a4154))
                };
                match m4155 {
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
        let a4156 = v2;
        let a4157 = v1;
        if nat_eq(a4156, a4157) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f868(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4158 = f847(v0, v1);
    let a4159 = f865(v0, v2);
    fixed_u32::bit_or(a4158, a4159)
}

pub fn f869(v0: u32) -> u32 {
    let a4162 = {
        let a4160 = f868(v0, 17u32, 15u32);
        let a4161 = f868(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4160, a4161)
    };
    let a4163 = f847(v0, 10u32);
    fixed_u32::bit_xor(a4162, a4163)
}

pub fn f870(v0: u32) -> u32 {
    let a4166 = {
        let a4164 = f868(v0, 7u32, 25u32);
        let a4165 = f868(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4164, a4165)
    };
    let a4167 = f847(v0, 3u32);
    fixed_u32::bit_xor(a4166, a4167)
}

pub fn f871(v0: u64) -> u32 {
    let (_, h4168) = {
        let (_, h4169) = f872(Fn9::F873, 32u64, (v0, (1u32, 0u32)));
        h4169
    };
    h4168
}

pub fn f872(v0: Fn9, p4173: u64, p4174: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4172 = (p4173, p4174);
    {
        let mut m4172 = m4172;
        loop {
            let step = {
                let (v1, v2) = m4172;
                let m4170 = v1;
                if m4170 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4170 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4171: Fn9 = v0.clone();
                        c4171.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f873(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4175, _) = v0;
        h4175
    };
    let v2: u32 = {
        let (h4176, _) = {
            let (_, h4177) = v0;
            h4177
        };
        h4176
    };
    let v3: u32 = {
        let (_, h4178) = {
            let (_, h4179) = v0;
            h4179
        };
        h4178
    };
    ({
        let a4180 = v1;
        let a4181 = 2u64;
        let a4182 = 0u64;
        nat_quot(a4180, a4181, a4182)
    }, (f865(v2, 1u32), {
        let a4186 = {
            let a4183 = v1;
            let a4184 = 2u64;
            let a4185 = 0u64;
            nat_rem(a4183, a4184, a4185)
        };
        let a4187 = 1u64;
        if nat_eq(a4186, a4187) {
            let a4188 = v3;
            let a4189 = v2;
            fixed_u32::bit_or(a4188, a4189)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
