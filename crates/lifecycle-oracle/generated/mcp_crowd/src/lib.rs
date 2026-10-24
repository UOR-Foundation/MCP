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
}

#[derive(Clone)]
pub enum Adt91 {
    C0(List<u64>, List<u64>, List<Adt92>, Bytes, u64, List<List<u64>>, Adt14),
}

#[derive(Clone)]
pub enum Adt92 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt93 {
    C0(u64),
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt94 {
    C0(u64, List<Adt92>, List<List<u64>>, List<u64>, Option<List<u64>>),
}

#[derive(Clone)]
pub enum Adt95 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt96 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt97 {
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
pub enum Adt98 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt99 {
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
pub enum Adt100 {
    C0,
    C1(Option<Adt98>, Adt101),
}

#[derive(Clone)]
pub enum Adt101 {
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
pub enum Adt102 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt103 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt104 {
    C0(Adt98, Str, Option<Adt5>),
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
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt112 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt113>, List<Adt115>),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt114),
    C7(Adt114),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt114 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt115 {
    C0(Str, List<Str>, Adt13, Adt116),
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt117 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt118 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt119 {
    C0(Adt120, List<Adt124>, u64),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt121, List<u64>, bool),
    C7(u64, u64),
    C8(Adt122),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt122 {
    C0(bool, Adt123, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt123 {
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
pub enum Adt124 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt126 {
    C0(Adt108, List<Adt109>),
}

#[derive(Clone)]
pub enum Adt127 {
    C0(Adt98, Result<Adt128, Adt131>),
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
    C2(Adt113),
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
    C0(Option<Str>, Option<Adt113>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0,
    C1(Adt98),
    C2,
}

#[derive(Clone)]
pub enum Adt135 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt99),
    C1(Adt119),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt109>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt99),
    C1(List<Adt111>, List<u64>),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt98, Adt140, Adt17, Adt5),
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
    C0,
    C1(Adt140),
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Adt144),
    C1(Adt131),
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt98, Str, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt145 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt146 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Adt97),
    C1(Adt125),
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
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt156 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt157 {
    C0,
    C1(Adt19),
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
    F67(Fn0, Adt91),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt15) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn2::F67(k0, k1) => f67(k0.clone(), k1.clone(), p0),
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

pub fn f3(_: Adt14, v1: Adt15) -> R<Result<Adt33, Adt90>> {
    f14(v1)
}

pub fn f4(v0: Bytes) -> R<Result<Adt91, Adt93>> {
    let m2 = f15(v0.clone(), 0u64)?;
    match m2 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(0u64)))
        }
        Some(v1) => {
            f16(v0.clone(), v1.clone(), f15(v0, match v1 {
                Adt95::C0(_, h3) => {
                    h3
                }
            })?)
        }
    }
}

pub fn f5(v0: Adt94) -> List<u64> {
    let a27 = f17(match v0.clone() {
        Adt94::C0(h4, _, _, _, _) => {
            h4
        }
    });
    let a28 = {
        let a25 = f17({
            let a6 = match v0.clone() {
                Adt94::C0(_, h5, _, _, _) => {
                    h5
                }
            };
            length_list(a6)
        });
        let a26 = {
            let a23 = f18(match v0.clone() {
                Adt94::C0(_, h7, _, _, _) => {
                    h7
                }
            });
            let a24 = {
                let a21 = f17({
                    let a9 = match v0.clone() {
                        Adt94::C0(_, _, h8, _, _) => {
                            h8
                        }
                    };
                    length_list(a9)
                });
                let a22 = {
                    let a19 = f19(match v0.clone() {
                        Adt94::C0(_, _, h10, _, _) => {
                            h10
                        }
                    });
                    let a20 = {
                        let a17 = f17({
                            let a12 = match v0.clone() {
                                Adt94::C0(_, _, _, h11, _) => {
                                    h11
                                }
                            };
                            length_list(a12)
                        });
                        let a18 = {
                            let a15 = match v0.clone() {
                                Adt94::C0(_, _, _, h13, _) => {
                                    h13
                                }
                            };
                            let a16 = f20(match v0 {
                                Adt94::C0(_, _, _, _, h14) => {
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

pub fn f6() -> Adt94 {
    Adt94::C0(f21(), f22(), List::<List<u64>>::nil(), f23(), None::<List<u64>>)
}

pub fn f7(v0: Adt0, v1: Fn0, v2: Adt91) -> R<Adt94> {
    f24(v0, v1, v2.clone(), f25(match v2.clone() {
        Adt91::C0(_, _, h29, _, _, _, _) => {
            h29
        }
    }, match v2 {
        Adt91::C0(_, _, _, _, _, h30, _) => {
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

pub fn f15(v0: Bytes, v1: u64) -> R<Option<Adt95>> {
    let m33 = f32(v0.clone(), v1)?;
    match m33 {
        None => {
            Ok(None::<Adt95>)
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
                    Ok(None::<Adt95>)
                }
                Some(v3) => {
                    Ok(Some(Adt95::C0(v3, {
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

pub fn f16(v0: Bytes, v1: Adt95, v2: Option<Adt95>) -> R<Result<Adt91, Adt93>> {
    let m44 = v2;
    match m44 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(match v1 {
                Adt95::C0(_, h45) => {
                    h45
                }
            })))
        }
        Some(v3) => {
            f33(v0.clone(), v1, v3.clone(), f32(v0, match v3 {
                Adt95::C0(_, h46) => {
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

pub fn f18(v0: List<Adt92>) -> List<u64> {
    f34(f35(v0, List::<Adt92>::nil()), List::<u64>::nil())
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

pub fn f22() -> List<Adt92> {
    List::cons(Adt92::C0(f38(), f39()), List::<Adt92>::nil())
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

pub fn f24(v0: Adt0, v1: Fn0, v2: Adt91, v3: Option<Adt96>) -> R<Adt94> {
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

pub fn f25(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
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

pub fn f27(v0: List<Adt36>) -> R<Result<List<Adt12>, Adt97>> {
    let m80 = f47(v0.clone());
    match m80 {
        None => {
            f48(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
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

pub fn f33(v0: Bytes, v1: Adt95, v2: Adt95, v3: Option<u64>) -> R<Result<Adt91, Adt93>> {
    let m110 = v3;
    match m110 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(match v2 {
                Adt95::C0(_, h111) => {
                    h111
                }
            })))
        }
        Some(v4) => {
            let m115 = f57(v0.clone(), {
                let a113 = match v2.clone() {
                    Adt95::C0(_, h112) => {
                        h112
                    }
                };
                let a114 = 4u64;
                nat_add(a113, a114)?
            }, v4, List::<Adt92>::nil())?;
            match m115 {
                Ok(v6) => {
                    f58(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt91, Adt93>(v5))
                }
            }
        }
    }
}

pub fn f34(p120: List<Adt92>, p121: List<u64>) -> List<u64> {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v0, v1) = m119;
                let m116 = v0;
                match m116.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<u64>), List<u64>>((v3, {
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

pub fn f35(p124: List<Adt92>, p125: List<Adt92>) -> List<Adt92> {
    let m123 = (p124, p125);
    {
        let mut m123 = m123;
        loop {
            let step = {
                let (v0, v1) = m123;
                let m122 = v0;
                match m122.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
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

pub fn f43(v0: Adt96) -> R<Adt94> {
    f61(f62(v0.clone()), f63(None::<Adt98>, f64(v0)))
}

pub fn f44(v0: Adt0, v1: Fn0, v2: Adt91) -> R<Adt94> {
    if f65(match v2.clone() {
        Adt91::C0(h136, _, _, _, _, _, _) => {
            h136
        }
    }) {
        f66(match v2.clone() {
            Adt91::C0(_, _, _, _, _, _, h137) => {
                h137
            }
        }, v0, Fn2::F67(v1, v2.clone()), match v2.clone() {
            Adt91::C0(_, _, _, _, h138, _, _) => {
                h138
            }
        }, match v2.clone() {
            Adt91::C0(_, _, h139, _, _, _, _) => {
                h139
            }
        }, f68(match v2 {
            Adt91::C0(_, _, _, h140, _, _, _) => {
                h140
            }
        })?)
    } else {
        Ok(f69())
    }
}

pub fn f45(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m141 = f70(v0, f71())?;
    match m141 {
        Adt102::C0 => {
            Ok(None::<Adt96>)
        }
        Adt102::C1 => {
            Ok(Some(Adt96::C0))
        }
        Adt102::C2 => {
            Ok(Some(Adt96::C0))
        }
        Adt102::C3(v2) => {
            f72(f73(v2), v1, Adt96::C0)
        }
    }
}

pub fn f46(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m142 = f70(v0, f74())?;
    match m142 {
        Adt102::C0 => {
            Ok(None::<Adt96>)
        }
        Adt102::C1 => {
            Ok(Some(Adt96::C1))
        }
        Adt102::C2 => {
            Ok(Some(Adt96::C1))
        }
        Adt102::C3(v2) => {
            f72(f75(v2), v1, Adt96::C1)
        }
    }
}

pub fn f47(v0: List<Adt36>) -> Option<Adt97> {
    let m143 = f76(v0, List::<Str>::nil());
    match m143 {
        None => {
            None::<Adt97>
        }
        Some(v1) => {
            Some(Adt97::C1(v1))
        }
    }
}

pub fn f48(p147: List<Adt36>, p148: List<Adt12>) -> R<Result<List<Adt12>, Adt97>> {
    let m146 = (p147, p148);
    {
        let mut m146 = m146;
        loop {
            let step = {
                let (v0, v1) = m146;
                let m144 = v0;
                match m144.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>(Ok(Ok::<List<Adt12>, Adt97>(f77(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m145 = f78(v2)?;
                        match m145 {
                            Ok(v5) => {
                                Ok::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>((v3, f77(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>(Ok(Err::<List<Adt12>, Adt97>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Str, v1: Str, v2: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f50(v0: u64) -> R<Str> {
    let a150 = List::cons(Str::lit("crowd_"), List::cons({
        let a149 = f79(v0, 0i64)?;
        format_i64(a149)
    }, List::<Str>::nil()));
    let a151 = Str::lit("");
    Ok(join(a150, a151))
}

pub fn f51() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f52(v0: Adt97) -> Str {
    let m152 = v0;
    match m152 {
        Adt97::C0(v1) => {
            let a153 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a154 = Str::lit(" ");
            join(a153, a154)
        }
        Adt97::C1(v2) => {
            let a155 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a156 = Str::lit(" ");
            join(a155, a156)
        }
        Adt97::C2(v3) => {
            let a157 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a158 = Str::lit(" ");
            join(a157, a158)
        }
        Adt97::C3(v4) => {
            let a159 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a160 = Str::lit(" ");
            join(a159, a160)
        }
        Adt97::C4(v5, v6) => {
            let a161 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a162 = Str::lit(" ");
            join(a161, a162)
        }
        Adt97::C5(v7, v8) => {
            let a163 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a164 = Str::lit(" ");
            join(a163, a164)
        }
        Adt97::C6(v9, v10) => {
            let a165 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a166 = Str::lit(" ");
            join(a165, a166)
        }
        Adt97::C7(v11) => {
            let a167 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a168 = Str::lit(" ");
            join(a167, a168)
        }
        Adt97::C8(v12) => {
            let a169 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a170 = Str::lit(" ");
            join(a169, a170)
        }
        Adt97::C9(v13) => {
            let a171 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a172 = Str::lit(" ");
            join(a171, a172)
        }
        Adt97::C10(v14) => {
            let a173 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a174 = Str::lit(" ");
            join(a173, a174)
        }
        Adt97::C11(v15) => {
            let a175 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a176 = Str::lit(" ");
            join(a175, a176)
        }
        Adt97::C12(v16, v17) => {
            let a177 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a178 = Str::lit(" ");
            join(a177, a178)
        }
    }
}

pub fn f53(v0: Adt10, v1: List<Adt36>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m180 = match v2 {
        Adt21::C0(_, h179) => {
            h179
        }
    };
    match m180 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C1(Adt35::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f54(v0: List<Adt36>, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m182 = f80(match v2.clone() {
        Adt22::C0(_, h181, _, _, _) => {
            h181
        }
    }, v0);
    match m182 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h183, _, _, _) => {
                    h183
                }
            })))
        }
        Some(v3) => {
            f81(v3, v1, v2)
        }
    }
}

pub fn f55(v0: Adt22) -> Result<Adt50, Adt90> {
    Ok::<Adt50, Adt90>(f82(List::cons(f83(match v0 {
        Adt22::C0(_, h184, _, _, _) => {
            h184
        }
    }), List::<Adt39>::nil())))
}

pub fn f56(v0: Bytes, v1: u64) -> R<u64> {
    let m187 = {
        let a185 = v0;
        let a186 = v1;
        index_bytes(a185, a186)
    };
    match m187 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a232 = {
                let a208 = {
                    let a196 = {
                        let a190 = {
                            let a188 = v3;
                            let a189 = 128u8;
                            fixed_u8::bit_and(a188, a189)
                        };
                        let a191 = 128u8;
                        if equal(a190, a191) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a197 = {
                        let a194 = {
                            let a192 = v3;
                            let a193 = 64u8;
                            fixed_u8::bit_and(a192, a193)
                        };
                        let a195 = 64u8;
                        if equal(a194, a195) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a196, a197)?
                };
                let a209 = {
                    let a206 = {
                        let a200 = {
                            let a198 = v3;
                            let a199 = 32u8;
                            fixed_u8::bit_and(a198, a199)
                        };
                        let a201 = 32u8;
                        if equal(a200, a201) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a207 = {
                        let a204 = {
                            let a202 = v3;
                            let a203 = 16u8;
                            fixed_u8::bit_and(a202, a203)
                        };
                        let a205 = 16u8;
                        if equal(a204, a205) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a206, a207)?
                };
                nat_add(a208, a209)?
            };
            let a233 = {
                let a230 = {
                    let a218 = {
                        let a212 = {
                            let a210 = v3;
                            let a211 = 8u8;
                            fixed_u8::bit_and(a210, a211)
                        };
                        let a213 = 8u8;
                        if equal(a212, a213) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a219 = {
                        let a216 = {
                            let a214 = v3;
                            let a215 = 4u8;
                            fixed_u8::bit_and(a214, a215)
                        };
                        let a217 = 4u8;
                        if equal(a216, a217) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a218, a219)?
                };
                let a231 = {
                    let a228 = {
                        let a222 = {
                            let a220 = v3;
                            let a221 = 2u8;
                            fixed_u8::bit_and(a220, a221)
                        };
                        let a223 = 2u8;
                        if equal(a222, a223) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a229 = {
                        let a226 = {
                            let a224 = v3;
                            let a225 = 1u8;
                            fixed_u8::bit_and(a224, a225)
                        };
                        let a227 = 1u8;
                        if equal(a226, a227) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a228, a229)?
                };
                nat_add(a230, a231)?
            };
            nat_add(a232, a233)
        }
    }
}

pub fn f57(v0: Bytes, p243: u64, p244: u64, p245: List<Adt92>) -> R<Result<(List<Adt92>, u64), Adt93>> {
    let m242 = (p243, (p244, p245));
    {
        let mut m242 = m242;
        loop {
            let step = {
                let (v1, (v2, v3)) = m242;
                let m234 = v2;
                if m234 == 0 {
                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Ok::<(List<Adt92>, u64), Adt93>((f84(v3, List::<Adt92>::nil()), v1))))
                } else {
                    let v4 = m234 - 1;
                    let m235 = f15(v0.clone(), v1)?;
                    match m235 {
                        None => {
                            Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Err::<(List<Adt92>, u64), Adt93>(Adt93::C0(v1))))
                        }
                        Some(v5) => {
                            let m237 = f15(v0.clone(), match v5.clone() {
                                Adt95::C0(_, h236) => {
                                    h236
                                }
                            })?;
                            match m237 {
                                None => {
                                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Err::<(List<Adt92>, u64), Adt93>(Adt93::C0(match v5 {
                                        Adt95::C0(_, h238) => {
                                            h238
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>((match v6.clone() {
                                        Adt95::C0(_, h239) => {
                                            h239
                                        }
                                    }, (v4, List::cons(Adt92::C0(f85(match v5 {
                                        Adt95::C0(h240, _) => {
                                            h240
                                        }
                                    })?, f85(match v6 {
                                        Adt95::C0(h241, _) => {
                                            h241
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Bytes, v1: Adt95, v2: Adt95, v3: (List<Adt92>, u64)) -> R<Result<Adt91, Adt93>> {
    let m247 = f15(v0.clone(), {
        let (_, h246) = v3.clone();
        h246
    })?;
    match m247 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0({
                let (_, h248) = v3;
                h248
            })))
        }
        Some(v4) => {
            let m250 = f32(v0.clone(), match v4.clone() {
                Adt95::C0(_, h249) => {
                    h249
                }
            })?;
            match m250 {
                None => {
                    Ok(Err::<Adt91, Adt93>(Adt93::C0(match v4 {
                        Adt95::C0(_, h251) => {
                            h251
                        }
                    })))
                }
                Some(v5) => {
                    f86(v0.clone(), Adt91::C0(f85(match v1 {
                        Adt95::C0(h252, _) => {
                            h252
                        }
                    })?, f85(match v2 {
                        Adt95::C0(h253, _) => {
                            h253
                        }
                    })?, {
                        let (h254, _) = v3;
                        h254
                    }, match v4.clone() {
                        Adt95::C0(h255, _) => {
                            h255
                        }
                    }, v5, List::<List<u64>>::nil(), Adt14::C0(0i64, Str::lit(""), Str::lit(""), f87())), f88(v0, {
                        let a257 = match v4 {
                            Adt95::C0(_, h256) => {
                                h256
                            }
                        };
                        let a258 = 4u64;
                        nat_add(a257, a258)?
                    })?)
                }
            }
        }
    }
}

pub fn f59(v0: Adt92) -> List<u64> {
    let a269 = f17({
        let a260 = match v0.clone() {
            Adt92::C0(h259, _) => {
                h259
            }
        };
        length_list(a260)
    });
    let a270 = {
        let a267 = match v0.clone() {
            Adt92::C0(h261, _) => {
                h261
            }
        };
        let a268 = {
            let a265 = f17({
                let a263 = match v0.clone() {
                    Adt92::C0(_, h262) => {
                        h262
                    }
                };
                length_list(a263)
            });
            let a266 = match v0 {
                Adt92::C0(_, h264) => {
                    h264
                }
            };
            append_list(a265, a266)
        };
        append_list(a267, a268)
    };
    append_list(a269, a270)
}

pub fn f60(v0: List<u64>) -> List<u64> {
    let a272 = f17({
        let a271 = v0.clone();
        length_list(a271)
    });
    let a273 = v0;
    append_list(a272, a273)
}

pub fn f61(v0: u64, v1: Adt6) -> R<Adt94> {
    let m274 = f89(v1)?;
    match m274 {
        Ok(v3) => {
            Ok(Adt94::C0(v0, f22(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f62(v0: Adt96) -> u64 {
    let m275 = v0;
    match m275 {
        Adt96::C0 => {
            f90()
        }
        Adt96::C1 => {
            f90()
        }
    }
}

pub fn f63(v0: Option<Adt98>, v1: Adt81) -> Adt6 {
    Adt6::C5(f91(v0, v1))
}

pub fn f64(v0: Adt96) -> Adt81 {
    Adt81::C0(f92(), f93(v0), None::<Adt6>)
}

pub fn f65(v0: List<u64>) -> bool {
    f94(v0, f95())
}

pub fn f66(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt92>, v5: Result<Adt6, Adt99>) -> R<Adt94> {
    let m276 = v5;
    match m276 {
        Ok(v7) => {
            f98(v0, v1, v2, v3, v4, f99(v7)?)
        }
        Err(_) => {
            f61(f96(), f63(None::<Adt98>, f97(Adt100::C0)))
        }
    }
}

pub fn f67(v0: Fn0, v1: Adt91, v2: Adt15) -> R<Result<Adt33, Adt90>> {
    let c277: Fn0 = v0;
    c277.apply(match v1 {
        Adt91::C0(_, _, _, _, _, _, h278) => {
            h278
        }
    }, v2)
}

pub fn f68(v0: Bytes) -> R<Result<Adt6, Adt99>> {
    f100(f85(v0)?)
}

pub fn f69() -> Adt94 {
    Adt94::C0(f101(), List::cons(Adt92::C0(f102(), f103()), List::<Adt92>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f70(v0: List<Adt92>, v1: List<u64>) -> R<Adt102> {
    let m279 = f104(v0, v1, Adt103::C0)?;
    match m279 {
        Adt103::C0 => {
            Ok(Adt102::C0)
        }
        Adt103::C1(v2) => {
            let m280 = f105(v2);
            match m280 {
                None => {
                    Ok(Adt102::C1)
                }
                Some(v3) => {
                    Ok(Adt102::C3(v3))
                }
            }
        }
        Adt103::C2 => {
            Ok(Adt102::C2)
        }
    }
}

pub fn f71() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f72(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt96) -> R<Option<Adt96>> {
    let m281 = v0;
    match m281 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f106(v3, v1)? {
                Ok(None::<Adt96>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f73(v0: List<u64>) -> Option<List<u64>> {
    let m282 = f107(v0);
    match m282 {
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
    let m283 = v0.clone();
    match m283.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a284 = v1;
            let a285 = 91u64;
            if nat_eq(a284, a285) {
                f108(f109(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f108(f110(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f76(p291: List<Adt36>, p292: List<Str>) -> Option<Str> {
    let m290 = (p291, p292);
    {
        let mut m290 = m290;
        loop {
            let step = {
                let (v0, v1) = m290;
                let m286 = v0;
                match m286.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f111(match v2.clone() {
                            Adt36::C0(h287, _, _, _, _, _, _, _) => {
                                h287
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt36>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt36::C0(h288, _, _, _, _, _, _, _) => {
                                    h288
                                }
                            }))
                        } else {
                            Ok::<(List<Adt36>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt36::C0(h289, _, _, _, _, _, _, _) => {
                                    h289
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m290 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(p295: List<Adt12>, p296: List<Adt12>) -> List<Adt12> {
    let m294 = (p295, p296);
    {
        let mut m294 = m294;
        loop {
            let step = {
                let (v0, v1) = m294;
                let m293 = v0;
                match m293.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Adt36) -> R<Result<List<Adt12>, Adt97>> {
    let m299 = f112(match v0.clone() {
        Adt36::C0(h297, _, _, _, _, _, _, _) => {
            h297
        }
    }, f113(match v0.clone() {
        Adt36::C0(h298, _, _, _, _, _, _, _) => {
            h298
        }
    })?);
    match m299 {
        None => {
            f114(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f79(p302: u64, p303: i64) -> R<i64> {
    let m301 = (p302, p303);
    {
        let mut m301 = m301;
        loop {
            let step = {
                let (v0, v1) = m301;
                let m300 = v0;
                if m300 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m300 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f115(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m301 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Str, p309: List<Adt36>) -> Option<Adt36> {
    let m308 = p309;
    {
        let mut m308 = m308;
        loop {
            let step = {
                let v1 = m308;
                let m304 = v1;
                match m304.uncons() {
                    None => {
                        Err::<List<Adt36>, Option<Adt36>>(None::<Adt36>)
                    }
                    Some((v2, v3)) => {
                        let a306 = match v2.clone() {
                            Adt36::C0(h305, _, _, _, _, _, _, _) => {
                                h305
                            }
                        };
                        let a307 = v0.clone();
                        if equal(a306, a307) {
                            Err::<List<Adt36>, Option<Adt36>>(Some(v2))
                        } else {
                            Ok::<List<Adt36>, Option<Adt36>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Adt36, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m316 = f116(Adt6::C5({
        let m312 = match v0.clone() {
            Adt36::C0(_, _, _, _, h311, _, _, _) => {
                h311
            }
        };
        match m312 {
            Adt5::C0(h310) => {
                h310
            }
        }
    }), Adt6::C5({
        let m315 = match v0.clone() {
            Adt36::C0(_, _, _, _, h314, _, _, _) => {
                h314
            }
        };
        match m315 {
            Adt5::C0(h313) => {
                h313
            }
        }
    }), f117(v2.clone()))?;
    match m316 {
        Adt106::C0 => {
            f118(v0, v1, v2)
        }
        Adt106::C1(v3, v4) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(f119(f120(v3, v4)?, true))))
        }
        Adt106::C2(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt106::C3(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt106::C4 => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f82(v0: List<Adt39>) -> Adt50 {
    f121(v0, None::<Adt6>, false, List::<Adt51>::nil())
}

pub fn f83(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f84(p319: List<Adt92>, p320: List<Adt92>) -> List<Adt92> {
    let m318 = (p319, p320);
    {
        let mut m318 = m318;
        loop {
            let step = {
                let (v0, v1) = m318;
                let m317 = v0;
                match m317.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Bytes) -> R<List<u64>> {
    f122(v0.clone(), {
        let a321 = v0;
        length_bytes(a321)
    }, List::<u64>::nil())
}

pub fn f86(v0: Bytes, v1: Adt91, v2: Result<(Adt14, u64), Adt93>) -> R<Result<Adt91, Adt93>> {
    let m322 = v2;
    match m322 {
        Ok(v4) => {
            f123(v0, Adt91::C0(match v1.clone() {
                Adt91::C0(h323, _, _, _, _, _, _) => {
                    h323
                }
            }, match v1.clone() {
                Adt91::C0(_, h324, _, _, _, _, _) => {
                    h324
                }
            }, match v1.clone() {
                Adt91::C0(_, _, h325, _, _, _, _) => {
                    h325
                }
            }, match v1.clone() {
                Adt91::C0(_, _, _, h326, _, _, _) => {
                    h326
                }
            }, match v1.clone() {
                Adt91::C0(_, _, _, _, h327, _, _) => {
                    h327
                }
            }, match v1 {
                Adt91::C0(_, _, _, _, _, h328, _) => {
                    h328
                }
            }, {
                let (h329, _) = v4;
                h329
            }), {
                let (_, h330) = v4;
                h330
            })
        }
        Err(v3) => {
            Ok(Err::<Adt91, Adt93>(v3))
        }
    }
}

pub fn f87() -> Bytes {
    Bytes::lit(&[])
}

pub fn f88(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt93>> {
    let a332 = v1;
    let a333 = {
        let a331 = v0.clone();
        length_bytes(a331)
    };
    if nat_eq(a332, a333) {
        Ok(Ok::<(Adt14, u64), Adt93>((Adt14::C0(0i64, Str::lit(""), Str::lit(""), f87()), v1)))
    } else {
        f124(v0, v1)
    }
}

pub fn f89(v0: Adt6) -> R<Result<List<u64>, Adt99>> {
    f125(f126(), v0)
}

pub fn f90() -> u64 {
    403u64
}

pub fn f91(v0: Option<Adt98>, v1: Adt81) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), {
        let a334 = f128(v0);
        let a335 = List::cons((Str::lit("error"), f129(v1)), List::<(Str, Adt6)>::nil());
        append_list(a334, a335)
    })
}

pub fn f92() -> i64 {
    -32000i64
}

pub fn f93(v0: Adt96) -> Str {
    let m336 = v0;
    match m336 {
        Adt96::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt96::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f94(p343: List<u64>, p344: List<u64>) -> bool {
    let m342 = (p343, p344);
    {
        let mut m342 = m342;
        loop {
            let step = {
                let (v0, v1) = m342;
                let m337 = v0;
                match m337.uncons() {
                    None => {
                        let m338 = v1;
                        match m338.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m339 = v1;
                        match m339.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a340 = v4;
                                let a341 = v6;
                                if nat_eq(a340, a341) {
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
                Ok(next) => { m342 = next; }
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

pub fn f97(v0: Adt100) -> Adt81 {
    Adt81::C0(f130(v0.clone()), f131(v0), None::<Adt6>)
}

pub fn f98(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt92>, v5: Result<Adt104, Adt100>) -> R<Adt94> {
    let m345 = v5;
    match m345 {
        Ok(v7) => {
            f134(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f61(f132(v6.clone()), f63(f133(v6.clone()), f97(v6)))
        }
    }
}

pub fn f99(v0: Adt6) -> R<Result<Adt104, Adt100>> {
    let m346 = v0;
    match m346 {
        Adt6::C0 => {
            Ok(f136(None::<Adt98>, Adt101::C0))
        }
        Adt6::C1(_) => {
            Ok(f136(None::<Adt98>, Adt101::C0))
        }
        Adt6::C2(_) => {
            Ok(f136(None::<Adt98>, Adt101::C0))
        }
        Adt6::C3(_) => {
            Ok(f136(None::<Adt98>, Adt101::C0))
        }
        Adt6::C4(_) => {
            Ok(f136(None::<Adt98>, Adt101::C1))
        }
        Adt6::C5(v1) => {
            f135(Adt5::C0(v1))
        }
    }
}

pub fn f100(v0: List<u64>) -> R<Result<Adt6, Adt99>> {
    let m347 = f137(v0.clone())?;
    match m347 {
        Adt117::C0 => {
            f138(f139(), 0u64, v0)
        }
        Adt117::C1(v1) => {
            Ok(Err::<Adt6, Adt99>(Adt99::C0(v1)))
        }
        Adt117::C2(v2) => {
            Ok(Err::<Adt6, Adt99>(Adt99::C1(v2)))
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

pub fn f104(p352: List<Adt92>, v1: List<u64>, p353: Adt103) -> R<Adt103> {
    let m351 = (p352, p353);
    {
        let mut m351 = m351;
        loop {
            let step = {
                let (v0, v2) = m351;
                let m348 = v0;
                match m348.uncons() {
                    None => {
                        Err::<(List<Adt92>, Adt103), R<Adt103>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f140(match v3.clone() {
                            Adt92::C0(h349, _) => {
                                h349
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt92>, Adt103), R<Adt103>>((v4, f141(v2, match v3 {
                                Adt92::C0(_, h350) => {
                                    h350
                                }
                            })))
                        } else {
                            Ok::<(List<Adt92>, Adt103), R<Adt103>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f142(v0);
    if f143(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f106(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
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

pub fn f107(p358: List<u64>) -> Option<List<u64>> {
    let m357 = p358;
    {
        let mut m357 = m357;
        loop {
            let step = {
                let v0 = m357;
                let m354 = v0;
                match m354.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a355 = v1;
                        let a356 = 58u64;
                        if nat_eq(a355, a356) {
                            Err::<List<u64>, Option<List<u64>>>(f148(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
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

pub fn f108(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m359 = v0;
    match m359 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f149({
                let (_, h360) = v1;
                h360
            }) {
                Some({
                    let (h361, _) = v1;
                    h361
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f109(p366: List<u64>, p367: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m365 = (p366, p367);
    {
        let mut m365 = m365;
        loop {
            let step = {
                let (v0, v1) = m365;
                let m362 = v0;
                match m362.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a363 = v2;
                        let a364 = 93u64;
                        if nat_eq(a363, a364) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m365 = next; }
                Err(done) => break done,
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
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a369 = v2;
                        let a370 = 58u64;
                        if nat_eq(a369, a370) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(v1, List::<u64>::nil()), List::cons(v2, v3))))
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

pub fn f111(v0: Str, p378: List<Str>) -> bool {
    let m377 = p378;
    {
        let mut m377 = m377;
        loop {
            let step = {
                let v1 = m377;
                let m374 = v1;
                match m374.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a375 = v0.clone();
                        let a376 = v2;
                        if equal(a375, a376) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
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

pub fn f112(v0: Str, v1: List<u64>) -> Option<Adt97> {
    if f151(v1) {
        Some(Adt97::C0(v0))
    } else {
        None::<Adt97>
    }
}

pub fn f113(v0: Str) -> R<List<u64>> {
    f85({
        let a379 = v0;
        utf8_encode(a379)
    })
}

pub fn f114(v0: Adt36) -> R<Result<List<Adt12>, Adt97>> {
    let m380 = f152(v0.clone());
    match m380 {
        None => {
            f153(v0.clone(), f154(f155(), match v0.clone() {
                Adt36::C0(h381, _, _, _, _, _, _, _) => {
                    h381
                }
            }, Adt6::C5({
                let m384 = match v0.clone() {
                    Adt36::C0(_, _, _, _, h383, _, _, _) => {
                        h383
                    }
                };
                match m384 {
                    Adt5::C0(h382) => {
                        h382
                    }
                }
            }), List::cons(Adt105::C0(Adt6::C5({
                let m387 = match v0 {
                    Adt36::C0(_, _, _, _, h386, _, _, _) => {
                        h386
                    }
                };
                match m387 {
                    Adt5::C0(h385) => {
                        h385
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt105>::nil()), f156())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f115(v0: i64, v1: i64) -> R<i64> {
    let a389 = v0;
    let a390 = {
        let a388 = v1;
        int_neg(a388)?
    };
    int_sub(a389, a390)
}

pub fn f116(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt106> {
    f157(v0, f158(), Adt108::C0, List::cons(Adt109::C0(v1, v2, List::<Adt107>::nil()), List::<Adt109>::nil()))
}

pub fn f117(v0: Adt22) -> Adt6 {
    let m392 = match v0 {
        Adt22::C0(_, _, h391, _, _) => {
            h391
        }
    };
    match m392 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h393) => {
                    h393
                }
            })
        }
    }
}

pub fn f118(v0: Adt36, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m395 = {
        let c394: Fn1 = v1;
        c394.apply(v2)
    };
    match m395 {
        Ok(v4) => {
            f159(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f119(v0: Str, v1: bool) -> Adt38 {
    Adt38::C0(List::cons(Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f120(v0: List<Adt107>, v1: Str) -> R<Str> {
    let v2: Str = f160(v0)?;
    let a398 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a396 = v2.clone();
        let a397 = Str::lit("");
        if equal(a396, a397) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a399 = Str::lit(" ");
    Ok(join(a398, a399))
}

pub fn f121(v0: List<Adt39>, v1: Option<Adt6>, v2: bool, v3: List<Adt51>) -> Adt50 {
    Adt50::C0(Adt38::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f122(v0: Bytes, p402: u64, p403: List<u64>) -> R<List<u64>> {
    let m401 = (p402, p403);
    {
        let mut m401 = m401;
        loop {
            let step = {
                let (v1, v2) = m401;
                let m400 = v1;
                if m400 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m400 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f56(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Bytes, v1: Adt91, v2: u64) -> R<Result<Adt91, Adt93>> {
    let a405 = v2;
    let a406 = {
        let a404 = v0.clone();
        length_bytes(a404)
    };
    if nat_eq(a405, a406) {
        Ok(Ok::<Adt91, Adt93>(v1))
    } else {
        let m407 = f32(v0.clone(), v2)?;
        match m407 {
            None => {
                Ok(Err::<Adt91, Adt93>(Adt93::C0(v2)))
            }
            Some(v3) => {
                let m410 = f161(v0.clone(), {
                    let a408 = v2;
                    let a409 = 4u64;
                    nat_add(a408, a409)?
                }, v3, List::<List<u64>>::nil())?;
                match m410 {
                    Ok(v5) => {
                        let a413 = {
                            let (_, h411) = v5.clone();
                            h411
                        };
                        let a414 = {
                            let a412 = v0;
                            length_bytes(a412)
                        };
                        if nat_eq(a413, a414) {
                            Ok(Ok::<Adt91, Adt93>(f162(v1, {
                                let (h415, _) = v5;
                                h415
                            })))
                        } else {
                            Ok(Err::<Adt91, Adt93>(Adt93::C1({
                                let (_, h416) = v5;
                                h416
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt91, Adt93>(v4))
                    }
                }
            }
        }
    }
}

pub fn f124(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt93>> {
    let m417 = f163(v0.clone(), v1)?;
    match m417 {
        None => {
            Ok(Err::<(Adt14, u64), Adt93>(Adt93::C0(v1)))
        }
        Some(v2) => {
            let m420 = f164(v0.clone(), {
                let a418 = v1;
                let a419 = 8u64;
                nat_add(a418, a419)?
            })?;
            match m420 {
                Ok(v4) => {
                    f165(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt14, u64), Adt93>(v3))
                }
            }
        }
    }
}

pub fn f125(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt99>> {
    let m421 = f166(v0, List::cons(Adt111::C1(0u64, v1), List::<Adt111>::nil()), List::<u64>::nil())?;
    match m421 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt99>(f167(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt99>(v3))
        }
    }
}

pub fn f126() -> u64 {
    67108864u64
}

pub fn f127() -> Str {
    Str::lit("2.0")
}

pub fn f128(v0: Option<Adt98>) -> List<(Str, Adt6)> {
    let m422 = v0;
    match m422 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f168(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f129(v0: Adt81) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt81::C0(h423, _, _) => {
            h423
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt81::C0(_, h424, _) => {
            h424
        }
    })), f169(match v0 {
        Adt81::C0(_, _, h425) => {
            h425
        }
    }))))
}

pub fn f130(v0: Adt100) -> i64 {
    let m426 = v0;
    match m426 {
        Adt100::C0 => {
            f170()
        }
        Adt100::C1(_, _) => {
            f171()
        }
    }
}

pub fn f131(v0: Adt100) -> Str {
    let m427 = v0;
    match m427 {
        Adt100::C0 => {
            Str::lit("Parse error")
        }
        Adt100::C1(_, v2) => {
            f172(v2)
        }
    }
}

pub fn f132(v0: Adt100) -> u64 {
    let m428 = v0;
    match m428 {
        Adt100::C0 => {
            f96()
        }
        Adt100::C1(_, _) => {
            f96()
        }
    }
}

pub fn f133(v0: Adt100) -> Option<Adt98> {
    let m429 = v0;
    match m429 {
        Adt100::C0 => {
            None::<Adt98>
        }
        Adt100::C1(v1, _) => {
            v1
        }
    }
}

pub fn f134(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt92>, v5: Adt104) -> R<Adt94> {
    f173(v0, f174(v4.clone())?, f175(v1.clone(), v2, v3, Some(f176(v4, match v1 {
        Adt0::C0(_, _, _, _, h430, _) => {
            h430
        }
    }, f177(v5.clone()), match v5.clone() {
        Adt104::C0(_, h431, _) => {
            h431
        }
    })?), f178(), v5)?)
}

pub fn f135(v0: Adt5) -> R<Result<Adt104, Adt100>> {
    let m432 = f179(v0.clone())?;
    match m432 {
        Adt134::C0 => {
            let m433 = f180(Str::lit("method"), v0.clone());
            match m433 {
                Adt135::C0 => {
                    Ok(f136(None::<Adt98>, f181(v0)))
                }
                Adt135::C1(_) => {
                    Ok(f136(None::<Adt98>, if f182(v0) {
                        Adt101::C8
                    } else {
                        Adt101::C7
                    }))
                }
            }
        }
        Adt134::C1(v2) => {
            Ok(f183(v2, v0))
        }
        Adt134::C2 => {
            Ok(f136(None::<Adt98>, Adt101::C5))
        }
    }
}

pub fn f136(v0: Option<Adt98>, v1: Adt101) -> Result<Adt104, Adt100> {
    Err::<Adt104, Adt100>(Adt100::C1(v0, v1))
}

pub fn f137(v0: List<u64>) -> R<Adt117> {
    Ok(f184({
        let a434 = v0.clone();
        length_list(a434)
    }, f185(Adt118::C0, 0u64, 0u64, v0)?))
}

pub fn f138(p441: Adt119, p442: u64, p443: List<u64>) -> R<Result<Adt6, Adt99>> {
    let m440 = (p441, (p442, p443));
    {
        let mut m440 = m440;
        loop {
            let step = {
                let (v0, (v1, v2)) = m440;
                let m435 = v2;
                match m435.uncons() {
                    None => {
                        let m436 = f186(v0, v1)?;
                        match m436 {
                            Ok(v3) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt99>>>(Ok(Ok::<Adt6, Adt99>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt99>>>(Ok(Err::<Adt6, Adt99>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m437 = f187(v0, v1, v5)?;
                        match m437 {
                            Adt136::C0(v7) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt99>>>(Ok(Err::<Adt6, Adt99>(v7)))
                            }
                            Adt136::C1(v8) => {
                                Ok::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt99>>>((v8, ({
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

pub fn f139() -> Adt119 {
    Adt119::C0(Adt120::C0, List::<Adt124>::nil(), 0u64)
}

pub fn f140(p450: List<u64>, p451: List<u64>) -> R<bool> {
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
                                let a447 = f188(v4)?;
                                let a448 = f188(v6)?;
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

pub fn f141(v0: Adt103, v1: List<u64>) -> Adt103 {
    let m452 = v0;
    match m452 {
        Adt103::C0 => {
            Adt103::C1(v1)
        }
        Adt103::C1(_) => {
            Adt103::C2
        }
        Adt103::C2 => {
            Adt103::C2
        }
    }
}

pub fn f142(v0: List<u64>) -> List<u64> {
    f150(f189(f150(f189(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f143(p455: List<u64>) -> bool {
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
                        if f190(v1) {
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

pub fn f144() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f145() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f146() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f147(v0: List<u64>, p458: List<List<u64>>) -> R<bool> {
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
                        if f140(v0.clone(), v2)? {
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

pub fn f148(v0: List<u64>) -> Option<List<u64>> {
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

pub fn f149(v0: List<u64>) -> bool {
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
                        f191(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f150(p471: List<u64>, p472: List<u64>) -> List<u64> {
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

pub fn f151(v0: List<u64>) -> bool {
    let m473 = v0.clone();
    match m473.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a477 = if f192(v0.clone(), false) {
                let a475 = {
                    let a474 = v0;
                    length_list(a474)
                };
                let a476 = f193();
                nat_le(a475, a476)
            } else {
                false
            };
            bool_not(a477)
        }
    }
}

pub fn f152(v0: Adt36) -> Option<Adt97> {
    let m482 = f194(match v0.clone() {
        Adt36::C0(h478, _, _, _, _, _, _, _) => {
            h478
        }
    }, {
        let m481 = match v0.clone() {
            Adt36::C0(_, _, _, _, h480, _, _, _) => {
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
            f195(match v0.clone() {
                Adt36::C0(h483, _, _, _, _, _, _, _) => {
                    h483
                }
            }, {
                let m486 = match v0 {
                    Adt36::C0(_, _, _, _, h485, _, _, _) => {
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

pub fn f153(v0: Adt36, v1: Result<Adt125, Adt97>) -> R<Result<List<Adt12>, Adt97>> {
    let m487 = v1;
    match m487 {
        Ok(v3) => {
            let m488 = f196(v0)?;
            match m488 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt97>(f77(match v3 {
                        Adt125::C0(h489, _) => {
                            h489
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt97>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt97>(v2))
        }
    }
}

pub fn f154(p496: u64, v1: Str, v2: Adt6, p497: List<Adt105>, p498: Adt125) -> R<Result<Adt125, Adt97>> {
    let m495 = (p496, (p497, p498));
    {
        let mut m495 = m495;
        loop {
            let step = {
                let (v0, (v3, v4)) = m495;
                let m490 = v0;
                if m490 == 0 {
                    Err::<(u64, (List<Adt105>, Adt125)), R<Result<Adt125, Adt97>>>(Ok(Err::<Adt125, Adt97>(Adt97::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m490 - 1;
                    let m491 = v3;
                    match m491.uncons() {
                        None => {
                            Err::<(u64, (List<Adt105>, Adt125)), R<Result<Adt125, Adt97>>>(Ok(Ok::<Adt125, Adt97>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m492 = f197(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m492 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt105>, Adt125)), R<Result<Adt125, Adt97>>>((v5, ({
                                        let (h493, _) = v9;
                                        h493
                                    }, {
                                        let (_, h494) = v9;
                                        h494
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt105>, Adt125)), R<Result<Adt125, Adt97>>>(Ok(Err::<Adt125, Adt97>(v8)))
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

pub fn f155() -> u64 {
    67108864u64
}

pub fn f156() -> Adt125 {
    Adt125::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f157(v0: Adt6, p517: u64, p518: Adt108, p519: List<Adt109>) -> R<Adt106> {
    let m516 = (p517, (p518, p519));
    {
        let mut m516 = m516;
        loop {
            let step = {
                let (v1, (v2, v3)) = m516;
                let m499 = v1;
                if m499 == 0 {
                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C4))
                } else {
                    let v4 = m499 - 1;
                    let m500 = v3;
                    match m500.uncons() {
                        None => {
                            let m501 = v2;
                            match m501 {
                                Adt108::C0 => {
                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C0))
                                }
                                Adt108::C1(v5, v6) => {
                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m502 = v2.clone();
                            match m502 {
                                Adt108::C0 => {
                                    let m503 = v7.clone();
                                    match m503 {
                                        Adt109::C0(v9, v10, v11) => {
                                            let m504 = f198(v0.clone(), v9, v10, v11.clone())?;
                                            match m504 {
                                                Adt137::C0(v12) => {
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v11, v12), v8)))
                                                }
                                                Adt137::C1(v13) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C2(v13)))
                                                }
                                                Adt137::C2(v14) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C3(v14)))
                                                }
                                                Adt137::C3(v15) => {
                                                    let v16: List<Adt109> = {
                                                        let a505 = v15;
                                                        let a506 = v8;
                                                        append_list(a505, a506)
                                                    };
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt109::C1(_, _, _) => {
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v8)))
                                        }
                                        Adt109::C2(_, _, _, _) => {
                                            let v24: Adt126 = f199(v7, v8)?;
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v24.clone() {
                                                Adt126::C0(h507, _) => {
                                                    h507
                                                }
                                            }, match v24 {
                                                Adt126::C0(_, h508) => {
                                                    h508
                                                }
                                            })))
                                        }
                                        Adt109::C3(v25) => {
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt109::C4(_, _, _, _) => {
                                            let v39: Adt126 = f199(v7, v8)?;
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v39.clone() {
                                                Adt126::C0(h512, _) => {
                                                    h512
                                                }
                                            }, match v39 {
                                                Adt126::C0(_, h513) => {
                                                    h513
                                                }
                                            })))
                                        }
                                        Adt109::C5(v26, v27, v28, v29) => {
                                            let m509 = f200(v26, v27, v28, v29.clone());
                                            match m509 {
                                                Adt137::C0(v30) => {
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v29, v30), v8)))
                                                }
                                                Adt137::C1(v31) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C2(v31)))
                                                }
                                                Adt137::C2(v32) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C3(v32)))
                                                }
                                                Adt137::C3(v33) => {
                                                    let v34: List<Adt109> = {
                                                        let a510 = v33;
                                                        let a511 = v8;
                                                        append_list(a510, a511)
                                                    };
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt108::C1(v40, v41) => {
                                    let v42: Adt126 = f201(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v42.clone() {
                                        Adt126::C0(h514, _) => {
                                            h514
                                        }
                                    }, match v42 {
                                        Adt126::C0(_, h515) => {
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

pub fn f158() -> u64 {
    67108864u64
}

pub fn f159(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f202({
        let m522 = match v1.clone() {
            Adt50::C0(h521, _) => {
                h521
            }
        };
        match m522 {
            Adt38::C0(h520, _, _) => {
                h520
            }
        }
    })? {
        f203(v0, v1)
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f160(v0: List<Adt107>) -> R<Str> {
    let a523 = List::cons(Str::lit(""), f204(v0, List::<Str>::nil())?);
    let a524 = Str::lit("/");
    Ok(join(a523, a524))
}

pub fn f161(v0: Bytes, p530: u64, p531: u64, p532: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt93>> {
    let m529 = (p530, (p531, p532));
    {
        let mut m529 = m529;
        loop {
            let step = {
                let (v1, (v2, v3)) = m529;
                let m525 = v2;
                if m525 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>(Ok(Ok::<(List<List<u64>>, u64), Adt93>((f37(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m525 - 1;
                    let m526 = f15(v0.clone(), v1)?;
                    match m526 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>(Ok(Err::<(List<List<u64>>, u64), Adt93>(Adt93::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>((match v5.clone() {
                                Adt95::C0(_, h527) => {
                                    h527
                                }
                            }, (v4, List::cons(f85(match v5 {
                                Adt95::C0(h528, _) => {
                                    h528
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m529 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Adt91, v1: List<List<u64>>) -> Adt91 {
    Adt91::C0(match v0.clone() {
        Adt91::C0(h533, _, _, _, _, _, _) => {
            h533
        }
    }, match v0.clone() {
        Adt91::C0(_, h534, _, _, _, _, _) => {
            h534
        }
    }, match v0.clone() {
        Adt91::C0(_, _, h535, _, _, _, _) => {
            h535
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, h536, _, _, _) => {
            h536
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, _, h537, _, _) => {
            h537
        }
    }, v1, match v0 {
        Adt91::C0(_, _, _, _, _, _, h538) => {
            h538
        }
    })
}

pub fn f163(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a542 = {
        let a539 = v1;
        let a540 = 8u64;
        nat_add(a539, a540)?
    };
    let a543 = {
        let a541 = v0.clone();
        length_bytes(a541)
    };
    if nat_le(a542, a543) {
        Ok(Some(f205(f205(f205(f205(f205(f205(f205(f205(0i64, f56(v0.clone(), v1)?)?, f56(v0.clone(), {
            let a544 = v1;
            let a545 = 1u64;
            nat_add(a544, a545)?
        })?)?, f56(v0.clone(), {
            let a546 = v1;
            let a547 = 2u64;
            nat_add(a546, a547)?
        })?)?, f56(v0.clone(), {
            let a548 = v1;
            let a549 = 3u64;
            nat_add(a548, a549)?
        })?)?, f56(v0.clone(), {
            let a550 = v1;
            let a551 = 4u64;
            nat_add(a550, a551)?
        })?)?, f56(v0.clone(), {
            let a552 = v1;
            let a553 = 5u64;
            nat_add(a552, a553)?
        })?)?, f56(v0.clone(), {
            let a554 = v1;
            let a555 = 6u64;
            nat_add(a554, a555)?
        })?)?, f56(v0, {
            let a556 = v1;
            let a557 = 7u64;
            nat_add(a556, a557)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f164(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt93>> {
    let m558 = f15(v0, v1)?;
    match m558 {
        None => {
            Ok(Err::<(Str, u64), Adt93>(Adt93::C0(v1)))
        }
        Some(v2) => {
            let m560 = f206(f85(match v2.clone() {
                Adt95::C0(h559, _) => {
                    h559
                }
            })?);
            match m560 {
                None => {
                    Ok(Err::<(Str, u64), Adt93>(Adt93::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt93>((v3, match v2 {
                        Adt95::C0(_, h561) => {
                            h561
                        }
                    })))
                }
            }
        }
    }
}

pub fn f165(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt14, u64), Adt93>> {
    let m563 = f164(v0.clone(), {
        let (_, h562) = v2;
        h562
    })?;
    match m563 {
        Ok(v4) => {
            f207(v0, v1, {
                let (h564, _) = v2;
                h564
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt14, u64), Adt93>(v3))
        }
    }
}

pub fn f166(p569: u64, p570: List<Adt111>, p571: List<u64>) -> R<Result<List<u64>, Adt99>> {
    let m568 = (p569, (p570, p571));
    {
        let mut m568 = m568;
        loop {
            let step = {
                let (v0, (v1, v2)) = m568;
                let m565 = v0;
                if m565 == 0 {
                    Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt99>>>(Ok(Err::<List<u64>, Adt99>(Adt99::C13)))
                } else {
                    let v3 = m565 - 1;
                    let m566 = v1;
                    match m566.uncons() {
                        None => {
                            Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt99>>>(Ok(Ok::<List<u64>, Adt99>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m567 = f208(v4, v5, v2)?;
                            match m567 {
                                Adt138::C0(v6) => {
                                    Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt99>>>(Ok(Err::<List<u64>, Adt99>(v6)))
                                }
                                Adt138::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt99>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(p574: List<u64>, p575: List<u64>) -> List<u64> {
    let m573 = (p574, p575);
    {
        let mut m573 = m573;
        loop {
            let step = {
                let (v0, v1) = m573;
                let m572 = v0;
                match m572.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Adt98) -> Adt6 {
    let m576 = v0;
    match m576 {
        Adt98::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt98::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f169(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m577 = v0;
    match m577 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f170() -> i64 {
    -32700i64
}

pub fn f171() -> i64 {
    -32600i64
}

pub fn f172(v0: Adt101) -> Str {
    let m578 = v0;
    match m578 {
        Adt101::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt101::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt101::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt101::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt101::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt101::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt101::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt101::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt101::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f173(v0: Adt14, v1: bool, v2: Adt127) -> R<Adt94> {
    let m580 = match v2.clone() {
        Adt127::C0(_, h579) => {
            h579
        }
    };
    match m580 {
        Ok(v3) => {
            let m581 = f209(v3.clone())?;
            match m581 {
                None => {
                    Ok(f6())
                }
                Some(v4) => {
                    f210(v0, f211(match v3.clone() {
                        Adt128::C0(h582, _, _) => {
                            h582
                        }
                    }), f212(v1, f61(f213(), f214(match v2 {
                        Adt127::C0(h583, _) => {
                            h583
                        }
                    }, v4))?, f215(f216(match v3.clone() {
                        Adt128::C0(_, _, h584) => {
                            h584
                        }
                    }, List::<List<u64>>::nil())?, f217(f211(match v3 {
                        Adt128::C0(h585, _, _) => {
                            h585
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f61(f218(v5.clone()), f63(Some(match v2 {
                Adt127::C0(h586, _) => {
                    h586
                }
            }), f219(v5)?))
        }
    }
}

pub fn f174(p591: List<Adt92>) -> R<bool> {
    let m590 = p591;
    {
        let mut m590 = m590;
        loop {
            let step = {
                let v0 = m590;
                let m587 = v0;
                match m587.uncons() {
                    None => {
                        Err::<List<Adt92>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f140(match v1.clone() {
                            Adt92::C0(h588, _) => {
                                h588
                            }
                        }, f220())? {
                            if f221(f222(44u64, match v1 {
                                Adt92::C0(_, h589) => {
                                    h589
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt92>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt92>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt92>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Adt0, v1: Fn2, v2: u64, v3: Option<Adt112>, v4: List<Str>, v5: Adt104) -> R<Adt127> {
    Ok(Adt127::C0(match v5.clone() {
        Adt104::C0(h592, _, _) => {
            h592
        }
    }, f223(v0, v1, v2, f224(v3, v4, v5)?)?))
}

pub fn f176(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt112> {
    let v4: Adt133 = f225(Adt114::C0, f70(v0.clone(), f226())?);
    let v5: Adt133 = f225(Adt114::C1, f70(v0.clone(), f227())?);
    let v6: Adt133 = if f228(v3) {
        f225(Adt114::C2, f229(f70(v0.clone(), f230())?)?)
    } else {
        f231()
    };
    Ok(Adt112::C0(match v4.clone() {
        Adt133::C0(h593, _) => {
            h593
        }
    }, match v5.clone() {
        Adt133::C0(h594, _) => {
            h594
        }
    }, match v6.clone() {
        Adt133::C0(h595, _) => {
            h595
        }
    }, f232(match v4 {
        Adt133::C0(_, h596) => {
            h596
        }
    }, f232(match v5 {
        Adt133::C0(_, h597) => {
            h597
        }
    }, match v6 {
        Adt133::C0(_, h598) => {
            h598
        }
    })), f233(v0, v1, v2)?))
}

pub fn f177(v0: Adt104) -> Option<Str> {
    let a600 = match v0.clone() {
        Adt104::C0(_, h599, _) => {
            h599
        }
    };
    let a601 = Str::lit("tools/call");
    if equal(a600, a601) {
        let m603 = match v0 {
            Adt104::C0(_, _, h602) => {
                h602
            }
        };
        match m603 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m604 = f180(Str::lit("name"), v1);
                match m604 {
                    Adt135::C0 => {
                        None::<Str>
                    }
                    Adt135::C1(v2) => {
                        f234(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f178() -> List<Str> {
    List::cons(f235(), List::<Str>::nil())
}

pub fn f179(v0: Adt5) -> R<Adt134> {
    let m605 = f180(Str::lit("id"), v0);
    match m605 {
        Adt135::C0 => {
            Ok(Adt134::C0)
        }
        Adt135::C1(v1) => {
            let m606 = v1;
            match m606 {
                Adt6::C0 => {
                    Ok(Adt134::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt134::C2)
                }
                Adt6::C2(v3) => {
                    let m607 = f236(v3)?;
                    match m607 {
                        None => {
                            Ok(Adt134::C2)
                        }
                        Some(v4) => {
                            Ok(Adt134::C1(Adt98::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt134::C1(Adt98::C0(v2)))
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

pub fn f180(v0: Str, v1: Adt5) -> Adt135 {
    f237(v0, match v1 {
        Adt5::C0(h608) => {
            h608
        }
    })
}

pub fn f181(v0: Adt5) -> Adt101 {
    if f182(v0) {
        Adt101::C8
    } else {
        Adt101::C3
    }
}

pub fn f182(v0: Adt5) -> bool {
    if f238(Str::lit("result"), v0.clone()) {
        true
    } else {
        f238(Str::lit("error"), v0)
    }
}

pub fn f183(v0: Adt98, v1: Adt5) -> Result<Adt104, Adt100> {
    let m609 = f180(Str::lit("jsonrpc"), v1.clone());
    match m609 {
        Adt135::C0 => {
            f136(Some(v0), Adt101::C2)
        }
        Adt135::C1(v2) => {
            let m610 = f234(v2);
            match m610 {
                None => {
                    f136(Some(v0), Adt101::C2)
                }
                Some(v3) => {
                    let a611 = v3;
                    let a612 = f127();
                    if equal(a611, a612) {
                        f239(v0, v1)
                    } else {
                        f136(Some(v0), Adt101::C2)
                    }
                }
            }
        }
    }
}

pub fn f184(v0: u64, v1: u64) -> Adt117 {
    let a613 = v1;
    let a614 = 0u64;
    if nat_eq(a613, a614) {
        Adt117::C0
    } else {
        let a615 = v1;
        let a616 = 2u64;
        if nat_eq(a615, a616) {
            Adt117::C2(v0)
        } else {
            Adt117::C1({
                let a620 = v0;
                let a621 = {
                    let a617 = v1;
                    let a618 = 3u64;
                    let a619 = 0u64;
                    nat_quot(a617, a618, a619)
                };
                nat_sub(a620, a621)
            })
        }
    }
}

pub fn f185(p649: Adt118, p650: u64, p651: u64, p652: List<u64>) -> R<u64> {
    let m648 = (p649, (p650, (p651, p652)));
    {
        let mut m648 = m648;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m648;
                let m622 = v3;
                match m622.uncons() {
                    None => {
                        let m623 = v0;
                        match m623 {
                            Adt118::C0 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt118::C1 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt118::C2 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt118::C3 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m624 = v0;
                        match m624 {
                            Adt118::C0 => {
                                let a625 = v4;
                                let a626 = 128u64;
                                if nat_lt(a625, a626) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a627 = v4;
                                    let a628 = 194u64;
                                    if nat_lt(a627, a628) {
                                        let v6: u64 = f240({
                                            let a629 = List::cons(v4, v5);
                                            length_list(a629)
                                        })?;
                                        Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a630 = v4;
                                        let a631 = 224u64;
                                        if nat_lt(a630, a631) {
                                            Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a632 = v4;
                                            let a633 = 224u64;
                                            if nat_eq(a632, a633) {
                                                Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a634 = v4;
                                                let a635 = 237u64;
                                                if nat_eq(a634, a635) {
                                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a636 = v4;
                                                    let a637 = 240u64;
                                                    if nat_lt(a636, a637) {
                                                        Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a638 = v4;
                                                        let a639 = 240u64;
                                                        if nat_eq(a638, a639) {
                                                            Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a640 = v4;
                                                            let a641 = 244u64;
                                                            if nat_eq(a640, a641) {
                                                                Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a642 = v4;
                                                                let a643 = 244u64;
                                                                if nat_lt(a642, a643) {
                                                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f240({
                                                                        let a644 = List::cons(v4, v5);
                                                                        length_list(a644)
                                                                    })?;
                                                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt118::C1 => {
                                if f241(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f240({
                                        let a645 = List::cons(v4, v5);
                                        length_list(a645)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt118::C2 => {
                                if f241(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f240({
                                        let a646 = List::cons(v4, v5);
                                        length_list(a646)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt118::C3 => {
                                if f241(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f240({
                                        let a647 = List::cons(v4, v5);
                                        length_list(a647)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m648 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: Adt119, v1: u64) -> R<Result<Adt6, Adt99>> {
    let m654 = match v0.clone() {
        Adt119::C0(h653, _, _) => {
            h653
        }
    };
    match m654 {
        Adt120::C0 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C1 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C2 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C3 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C4 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C5 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C7(_, _) => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C8(v3) => {
            let m655 = f242(v3)?;
            match m655 {
                Adt141::C0 => {
                    Ok(Err::<Adt6, Adt99>(Adt99::C7(v1)))
                }
                Adt141::C1 => {
                    Ok(Err::<Adt6, Adt99>(Adt99::C8(v1)))
                }
                Adt141::C2(v4) => {
                    let m660 = {
                        let m659 = f243(v4, match v0.clone() {
                            Adt119::C0(_, h657, _) => {
                                h657
                            }
                        }, match v0 {
                            Adt119::C0(_, _, h658) => {
                                h658
                            }
                        });
                        match m659 {
                            Adt119::C0(h656, _, _) => {
                                h656
                            }
                        }
                    };
                    match m660 {
                        Adt120::C0 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C1 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C2 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C3 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C4 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C5 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C7(_, _) => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C8(_) => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C9(v5) => {
                            Ok(Ok::<Adt6, Adt99>(v5))
                        }
                    }
                }
            }
        }
        Adt120::C9(v2) => {
            Ok(Ok::<Adt6, Adt99>(v2))
        }
    }
}

pub fn f187(v0: Adt119, v1: u64, v2: u64) -> R<Adt136> {
    let m662 = match v0.clone() {
        Adt119::C0(h661, _, _) => {
            h661
        }
    };
    match m662 {
        Adt120::C0 => {
            if f244(v2) {
                Ok(Adt136::C1(v0))
            } else {
                f245(v0, v1, v2)
            }
        }
        Adt120::C1 => {
            if f244(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a663 = v2;
                let a664 = 93u64;
                if nat_eq(a663, a664) {
                    Ok(f246(v0, v1))
                } else {
                    f245(v0, v1, v2)
                }
            }
        }
        Adt120::C2 => {
            Ok(f247(v0, v1, v2))
        }
        Adt120::C3 => {
            if f244(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a665 = v2;
                let a666 = 125u64;
                if nat_eq(a665, a666) {
                    Ok(f248(v0, v1))
                } else {
                    let a667 = v2;
                    let a668 = 34u64;
                    if nat_eq(a667, a668) {
                        Ok(f249(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f250(Adt99::C2(v1)))
                    }
                }
            }
        }
        Adt120::C4 => {
            if f244(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a669 = v2;
                let a670 = 34u64;
                if nat_eq(a669, a670) {
                    Ok(f249(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f250(Adt99::C2(v1)))
                }
            }
        }
        Adt120::C5 => {
            if f244(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a671 = v2;
                let a672 = 58u64;
                if nat_eq(a671, a672) {
                    Ok(f249(v0, Adt120::C0))
                } else {
                    Ok(f250(Adt99::C2(v1)))
                }
            }
        }
        Adt120::C6(v3, v4, v5) => {
            f251(v0, v1, v2, v3, v4, v5)
        }
        Adt120::C7(v6, v7) => {
            f252(v0, v1, v2, v6, v7)
        }
        Adt120::C8(v8) => {
            f253(v0, v1, v2, v8)
        }
        Adt120::C9(_) => {
            Ok(f247(v0, v1, v2))
        }
    }
}

pub fn f188(v0: u64) -> R<u64> {
    let a673 = 64u64;
    let a674 = v0;
    if if nat_lt(a673, a674) {
        let a675 = v0;
        let a676 = 91u64;
        nat_lt(a675, a676)
    } else {
        false
    } {
        let a677 = v0;
        let a678 = 32u64;
        nat_add(a677, a678)
    } else {
        Ok(v0)
    }
}

pub fn f189(p681: List<u64>) -> List<u64> {
    let m680 = p681;
    {
        let mut m680 = m680;
        loop {
            let step = {
                let v0 = m680;
                let m679 = v0;
                match m679.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f254(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: u64) -> bool {
    let a682 = v0;
    let a683 = 9u64;
    if nat_eq(a682, a683) {
        true
    } else {
        let a684 = 31u64;
        let a685 = v0;
        if nat_lt(a684, a685) {
            let a686 = v0;
            let a687 = 127u64;
            nat_lt(a686, a687)
        } else {
            false
        }
    }
}

pub fn f191(p690: List<u64>) -> bool {
    let m689 = p690;
    {
        let mut m689 = m689;
        loop {
            let step = {
                let v0 = m689;
                let m688 = v0;
                match m688.uncons() {
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
                Ok(next) => { m689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(p693: List<u64>, v1: bool) -> bool {
    let m692 = p693;
    {
        let mut m692 = m692;
        loop {
            let step = {
                let v0 = m692;
                let m691 = v0;
                match m691.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f256(v2)
                        } else {
                            f257(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m692 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193() -> u64 {
    64u64
}

pub fn f194(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt97> {
    let m694 = f237(Str::lit("type"), v1);
    match m694 {
        Adt135::C0 => {
            Some(Adt97::C3(v0))
        }
        Adt135::C1(v2) => {
            let m695 = f234(v2);
            match m695 {
                None => {
                    Some(Adt97::C3(v0))
                }
                Some(v3) => {
                    let a696 = v3;
                    let a697 = Str::lit("object");
                    if equal(a696, a697) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f195(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt97> {
    let m698 = f237(Str::lit("$schema"), v1);
    match m698 {
        Adt135::C0 => {
            None::<Adt97>
        }
        Adt135::C1(v2) => {
            let m699 = f234(v2);
            match m699 {
                None => {
                    Some(Adt97::C2(v0))
                }
                Some(v3) => {
                    let a700 = v3;
                    let a701 = f258();
                    if equal(a700, a701) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f196(v0: Adt36) -> R<Option<Adt97>> {
    let m703 = match v0.clone() {
        Adt36::C0(_, _, _, _, _, h702, _, _) => {
            h702
        }
    };
    match m703 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v1) => {
            let m706 = f195(match v0.clone() {
                Adt36::C0(h704, _, _, _, _, _, _, _) => {
                    h704
                }
            }, match v1.clone() {
                Adt5::C0(h705) => {
                    h705
                }
            });
            match m706 {
                None => {
                    let m710 = f154(f155(), match v0 {
                        Adt36::C0(h707, _, _, _, _, _, _, _) => {
                            h707
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h708) => {
                            h708
                        }
                    }), List::cons(Adt105::C0(Adt6::C5(match v1 {
                        Adt5::C0(h709) => {
                            h709
                        }
                    }), false, List::<Str>::nil()), List::<Adt105>::nil()), f156())?;
                    match m710 {
                        Ok(_) => {
                            Ok(None::<Adt97>)
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

pub fn f197(v0: Str, v1: Adt6, v2: Adt105, v3: Adt125, v4: List<Adt105>) -> R<Result<(List<Adt105>, Adt125), Adt97>> {
    let m711 = v2;
    match m711 {
        Adt105::C0(v5, v6, v7) => {
            let m712 = f259(v5.clone());
            match m712 {
                None => {
                    let m713 = f260(v5);
                    match m713 {
                        None => {
                            Ok(Err::<(List<Adt105>, Adt125), Adt97>(Adt97::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f261(v0, v1, match v9 {
                                Adt5::C0(h714) => {
                                    h714
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt105>, Adt125), Adt97>((v4, v3)))
                }
            }
        }
    }
}

pub fn f198(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt107>) -> R<Adt137> {
    let m715 = f259(v1.clone());
    match m715 {
        None => {
            let m716 = f260(v1);
            match m716 {
                None => {
                    Ok(Adt137::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f262(v0, match v5 {
                        Adt5::C0(h717) => {
                            h717
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt137::C3(List::<Adt109>::nil()))
            } else {
                Ok(Adt137::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f199(v0: Adt109, v1: List<Adt109>) -> R<Adt126> {
    let m718 = v0;
    match m718 {
        Adt109::C0(_, _, _) => {
            Ok(Adt126::C0(Adt108::C0, v1))
        }
        Adt109::C1(_, _, _) => {
            Ok(Adt126::C0(Adt108::C0, v1))
        }
        Adt109::C2(v8, v9, v10, v11) => {
            Ok(f263(v8, v9, v10, {
                let a719 = v11;
                let a720 = 1u64;
                nat_add(a719, a720)?
            }, v1))
        }
        Adt109::C3(v12) => {
            Ok(Adt126::C0(Adt108::C1(v12, Str::lit("not")), v1))
        }
        Adt109::C4(v13, _, v15, v16) => {
            Ok(f264(v13, v15, v16, v1))
        }
        Adt109::C5(_, _, _, _) => {
            Ok(Adt126::C0(Adt108::C0, v1))
        }
    }
}

pub fn f200(v0: Adt110, v1: Adt6, v2: Adt6, v3: List<Adt107>) -> Adt137 {
    let m721 = f260(v1);
    match m721 {
        None => {
            Adt137::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m722 = v0;
            match m722 {
                Adt110::C0 => {
                    f265(match v4 {
                        Adt5::C0(h723) => {
                            h723
                        }
                    }, v2, v3)
                }
                Adt110::C1 => {
                    f266(match v4 {
                        Adt5::C0(h724) => {
                            h724
                        }
                    }, v2, v3)
                }
                Adt110::C2 => {
                    f267(match v4 {
                        Adt5::C0(h725) => {
                            h725
                        }
                    }, v2, v3)
                }
                Adt110::C3 => {
                    f268(match v4 {
                        Adt5::C0(h726) => {
                            h726
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f201(v0: Adt109, v1: List<Adt109>, v2: List<Adt107>, v3: Str) -> Adt126 {
    let m727 = v0;
    match m727 {
        Adt109::C0(_, _, _) => {
            Adt126::C0(Adt108::C1(v2, v3), v1)
        }
        Adt109::C1(v7, v8, v9) => {
            f269(v7, v8, v9, v1)
        }
        Adt109::C2(v10, v11, v12, v13) => {
            f263(v10, v11, v12, v13, v1)
        }
        Adt109::C3(_) => {
            Adt126::C0(Adt108::C0, v1)
        }
        Adt109::C4(_, v16, v17, v18) => {
            f264(v16, v17, v18, v1)
        }
        Adt109::C5(_, _, _, _) => {
            Adt126::C0(Adt108::C1(v2, v3), v1)
        }
    }
}

pub fn f202(p736: List<Adt39>) -> R<bool> {
    let m735 = p736;
    {
        let mut m735 = m735;
        loop {
            let step = {
                let v0 = m735;
                let m728 = v0;
                match m728.uncons() {
                    None => {
                        Err::<List<Adt39>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m729 = v1;
                        match m729 {
                            Adt39::C0(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C1(v4) => {
                                if f270(match v4 {
                                    Adt43::C0(h730, _, _, _) => {
                                        h730
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C2(v5) => {
                                if f270(match v5 {
                                    Adt44::C0(h731, _, _, _) => {
                                        h731
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
                                let m733 = match v7 {
                                    Adt46::C0(h732, _, _) => {
                                        h732
                                    }
                                };
                                match m733 {
                                    Adt47::C0(_) => {
                                        Ok::<List<Adt39>, R<bool>>(v2)
                                    }
                                    Adt47::C1(v9) => {
                                        if f270(match v9 {
                                            Adt49::C0(_, _, h734, _) => {
                                                h734
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
                Ok(next) => { m735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f271(match v1.clone() {
        Adt50::C0(h737, _) => {
            h737
        }
    }) {
        f272(v1)
    } else {
        let m739 = match v0 {
            Adt36::C0(_, _, _, _, _, h738, _, _) => {
                h738
            }
        };
        match m739 {
            None => {
                f272(v1)
            }
            Some(v2) => {
                f273(v2, v1)
            }
        }
    }
}

pub fn f204(p742: List<Adt107>, p743: List<Str>) -> R<List<Str>> {
    let m741 = (p742, p743);
    {
        let mut m741 = m741;
        loop {
            let step = {
                let (v0, v1) = m741;
                let m740 = v0;
                match m740.uncons() {
                    None => {
                        Err::<(List<Adt107>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt107>, List<Str>), R<List<Str>>>((v3, List::cons(f274(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: i64, v1: u64) -> R<i64> {
    f115({
        let a744 = v0;
        let a745 = 256i64;
        int_mul(a744, a745)?
    }, f79(v1, 0i64)?)
}

pub fn f206(v0: List<u64>) -> Option<Str> {
    let a746 = f275(f87(), v0);
    utf8_decode(a746)
}

pub fn f207(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt14, u64), Adt93>> {
    let m748 = f15(v0, {
        let (_, h747) = v3.clone();
        h747
    })?;
    match m748 {
        None => {
            Ok(Err::<(Adt14, u64), Adt93>(Adt93::C0({
                let (_, h749) = v3;
                h749
            })))
        }
        Some(v4) => {
            Ok(f276(v1, v2, {
                let (h750, _) = v3;
                h750
            }, v4))
        }
    }
}

pub fn f208(v0: Adt111, v1: List<Adt111>, v2: List<u64>) -> R<Adt138> {
    let m751 = v0;
    match m751 {
        Adt111::C0(v3) => {
            Ok(Adt138::C1(v1, f167(v3, v2)))
        }
        Adt111::C1(v4, v5) => {
            f277(v4, v5, v1, v2)
        }
        Adt111::C2(v6, v7) => {
            let m752 = v7;
            match m752.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt138::C1(List::cons(Adt111::C1(v6, v8), List::cons(Adt111::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt111::C3(v10, v11) => {
            let m753 = v11.clone();
            match m753.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt111::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt111::C4(v14, v15) => {
            let m754 = v15;
            match m754.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt138::C1(List::cons(Adt111::C0({
                        let a756 = f278({
                            let (h755, _) = v16;
                            h755
                        })?;
                        let a757 = List::cons(58u64, List::<u64>::nil());
                        append_list(a756, a757)
                    }), List::cons(Adt111::C1(v14, {
                        let (_, h758) = v16;
                        h758
                    }), List::cons(Adt111::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt111::C5(v18, v19) => {
            let m759 = v19.clone();
            match m759.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt111::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f209(v0: Adt128) -> R<Option<Adt5>> {
    let m761 = f279(match v0.clone() {
        Adt128::C0(h760, _, _) => {
            h760
        }
    })?;
    match m761 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a764 = v1;
                let a765 = {
                    let m763 = f280(v0);
                    match m763 {
                        Adt5::C0(h762) => {
                            h762
                        }
                    }
                };
                append_list(a764, a765)
            })))
        }
    }
}

pub fn f210(v0: Adt14, v1: Option<Adt88>, v2: Adt94) -> R<Adt94> {
    let m766 = v1;
    match m766 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a768 = match v2.clone() {
                Adt94::C0(h767, _, _, _, _) => {
                    h767
                }
            };
            let a769 = f213();
            if nat_eq(a768, a769) {
                Ok(f281(f282(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f211(v0: Adt33) -> Option<Adt88> {
    let m770 = v0;
    match m770 {
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
                Adt83::C0(_, _, h771) => {
                    h771
                }
            }, Adt89::C0, false))
        }
        Adt33::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f212(v0: bool, v1: Adt94, v2: Option<List<List<u64>>>) -> Adt94 {
    let m772 = v2;
    match m772 {
        None => {
            f6()
        }
        Some(v3) => {
            f283(v0, v1, f37(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f213() -> u64 {
    200u64
}

pub fn f214(v0: Adt98, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), List::cons((Str::lit("id"), f168(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h773) => {
            h773
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f215(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m774 = v0;
    match m774 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m775 = v1;
            match m775 {
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

pub fn f216(p779: List<Adt130>, p780: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m778 = (p779, p780);
    {
        let mut m778 = m778;
        loop {
            let step = {
                let (v0, v1) = m778;
                let m776 = v0;
                match m776.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m777 = f284(v2)?;
                        match m777 {
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
                Ok(next) => { m778 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Option<Adt88>) -> R<Option<List<List<u64>>>> {
    let m781 = v0;
    match m781 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f285(v1)
        }
    }
}

pub fn f218(v0: Adt131) -> u64 {
    let m782 = v0;
    match m782 {
        Adt131::C0(_) => {
            f96()
        }
        Adt131::C1(_, _) => {
            f96()
        }
        Adt131::C2(_) => {
            f96()
        }
        Adt131::C3(_) => {
            f286()
        }
        Adt131::C4(v6) => {
            f287(v6)
        }
    }
}

pub fn f219(v0: Adt131) -> R<Adt81> {
    Ok(Adt81::C0(f288(v0.clone()), f289(v0.clone()), f290(v0)?))
}

pub fn f220() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f221(p785: List<List<u64>>) -> R<bool> {
    let m784 = p785;
    {
        let mut m784 = m784;
        loop {
            let step = {
                let v0 = m784;
                let m783 = v0;
                match m783.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f291(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: u64, p790: List<u64>, p791: List<u64>, p792: List<List<u64>>) -> List<List<u64>> {
    let m789 = (p790, (p791, p792));
    {
        let mut m789 = m789;
        loop {
            let step = {
                let (v1, (v2, v3)) = m789;
                let m786 = v1;
                match m786.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f37(List::cons(f150(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a787 = v4;
                        let a788 = v0;
                        if nat_eq(a787, a788) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f150(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m789 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt139, Adt131>) -> R<Result<Adt128, Adt131>> {
    let m793 = v3;
    match m793 {
        Ok(v5) => {
            f292(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(v4))
        }
    }
}

pub fn f224(v0: Option<Adt112>, v1: List<Str>, v2: Adt104) -> R<Result<Adt139, Adt131>> {
    Ok(f293(f294(match v2.clone() {
        Adt104::C0(_, h794, _) => {
            h794
        }
    }), f295(v0, v1, v2)?))
}

pub fn f225(v0: Adt114, v1: Adt102) -> Adt133 {
    let m795 = v1;
    match m795 {
        Adt102::C0 => {
            f231()
        }
        Adt102::C1 => {
            Adt133::C0(None::<Str>, Some(Adt113::C6(v0)))
        }
        Adt102::C2 => {
            Adt133::C0(None::<Str>, Some(Adt113::C7(v0)))
        }
        Adt102::C3(v2) => {
            let m796 = f206(v2);
            match m796 {
                None => {
                    Adt133::C0(None::<Str>, Some(Adt113::C6(v0)))
                }
                Some(v3) => {
                    Adt133::C0(Some(v3), None::<Adt113>)
                }
            }
        }
    }
}

pub fn f226() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f227() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f228(v0: Str) -> bool {
    let m797 = f296(v0);
    match m797 {
        Adt145::C0 => {
            false
        }
        Adt145::C1 => {
            true
        }
        Adt145::C2 => {
            true
        }
        Adt145::C3 => {
            true
        }
    }
}

pub fn f229(v0: Adt102) -> R<Adt102> {
    let m798 = v0.clone();
    match m798 {
        Adt102::C0 => {
            Ok(v0)
        }
        Adt102::C1 => {
            Ok(v0)
        }
        Adt102::C2 => {
            Ok(v0)
        }
        Adt102::C3(v1) => {
            let m799 = f297(v1);
            match m799 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m800 = f298(v2)?;
                    match m800 {
                        None => {
                            Ok(Adt102::C1)
                        }
                        Some(v3) => {
                            Ok(Adt102::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f230() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f231() -> Adt133 {
    Adt133::C0(None::<Str>, None::<Adt113>)
}

pub fn f232(v0: Option<Adt113>, v1: Option<Adt113>) -> Option<Adt113> {
    let m801 = v0.clone();
    match m801 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f233(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt115>> {
    Ok(f299(f300(v0, v1, v2, List::<Adt115>::nil())?, List::<Adt115>::nil()))
}

pub fn f234(v0: Adt6) -> Option<Str> {
    let m802 = v0;
    match m802 {
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

pub fn f235() -> Str {
    Str::lit("2026-07-28")
}

pub fn f236(v0: Adt7) -> R<Option<i64>> {
    let m805 = f301(40u64, match v0.clone() {
        Adt7::C0(h803, _) => {
            h803
        }
    }, match v0 {
        Adt7::C0(_, h804) => {
            h804
        }
    })?;
    match m805 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f302(v1))
        }
    }
}

pub fn f237(v0: Str, p811: List<(Str, Adt6)>) -> Adt135 {
    let m810 = p811;
    {
        let mut m810 = m810;
        loop {
            let step = {
                let v1 = m810;
                let m806 = v1;
                match m806.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt135>(Adt135::C0)
                    }
                    Some((v2, v3)) => {
                        let m808 = f303({
                            let (h807, _) = v2;
                            h807
                        }, v0.clone());
                        match m808 {
                            Adt146::C0 => {
                                Err::<List<(Str, Adt6)>, Adt135>(Adt135::C1({
                                    let (_, h809) = v2;
                                    h809
                                }))
                            }
                            Adt146::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt135>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: Str, v1: Adt5) -> bool {
    f304(f180(v0, v1))
}

pub fn f239(v0: Adt98, v1: Adt5) -> Result<Adt104, Adt100> {
    let m812 = f180(Str::lit("method"), v1.clone());
    match m812 {
        Adt135::C0 => {
            f136(Some(v0), f181(v1))
        }
        Adt135::C1(v2) => {
            let m813 = f234(v2);
            match m813 {
                None => {
                    f136(Some(v0), Adt101::C4)
                }
                Some(v3) => {
                    f305(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f240(v0: u64) -> R<u64> {
    let a816 = {
        let a814 = v0;
        let a815 = 3u64;
        nat_mul(a814, a815)?
    };
    let a817 = 1u64;
    nat_add(a816, a817)
}

pub fn f241(v0: u64, v1: u64, v2: u64) -> bool {
    let a818 = v1;
    let a819 = v0;
    if nat_le(a818, a819) {
        let a820 = v0;
        let a821 = v2;
        nat_le(a820, a821)
    } else {
        false
    }
}

pub fn f242(v0: Adt122) -> R<Adt141> {
    if f306(match v0.clone() {
        Adt122::C0(_, h822, _, _, _, _, _) => {
            h822
        }
    }) {
        let v1: i64 = {
            let a828 = if match v0.clone() {
                Adt122::C0(_, _, _, _, _, h823, _) => {
                    h823
                }
            } {
                let a825 = match v0.clone() {
                    Adt122::C0(_, _, _, _, _, _, h824) => {
                        h824
                    }
                };
                int_neg(a825)?
            } else {
                match v0.clone() {
                    Adt122::C0(_, _, _, _, _, _, h826) => {
                        h826
                    }
                }
            };
            let a829 = match v0.clone() {
                Adt122::C0(_, _, _, _, h827, _, _) => {
                    h827
                }
            };
            int_sub(a828, a829)?
        };
        let m832 = {
            let a830 = v1;
            let a831 = -9999i64;
            compare(a830, a831)
        };
        if if match m832 {
            Ordering::Less => {
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
            let m835 = {
                let a833 = 9999i64;
                let a834 = v1;
                compare(a833, a834)
            };
            match m835 {
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
                Adt122::C0(h836, _, _, _, _, _, _) => {
                    h836
                }
            } {
                let a838 = match v0 {
                    Adt122::C0(_, _, h837, _, _, _, _) => {
                        h837
                    }
                };
                int_neg(a838)?
            } else {
                match v0 {
                    Adt122::C0(_, _, h839, _, _, _, _) => {
                        h839
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt141::C0)
    }
}

pub fn f243(v0: Adt6, v1: List<Adt124>, v2: u64) -> Adt119 {
    let m840 = v1.clone();
    match m840.uncons() {
        None => {
            Adt119::C0(Adt120::C9(v0), List::<Adt124>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m841 = v3;
            match m841 {
                Adt124::C0(v5) => {
                    Adt119::C0(Adt120::C2, List::cons(Adt124::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt124::C1(v6, v7) => {
                    let m842 = v7;
                    match m842 {
                        None => {
                            Adt119::C0(Adt120::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt119::C0(Adt120::C2, List::cons(Adt124::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f244(v0: u64) -> bool {
    let a843 = v0;
    let a844 = 32u64;
    if nat_eq(a843, a844) {
        true
    } else {
        let a845 = v0;
        let a846 = 10u64;
        if nat_eq(a845, a846) {
            true
        } else {
            let a847 = v0;
            let a848 = 13u64;
            if nat_eq(a847, a848) {
                true
            } else {
                let a849 = v0;
                let a850 = 9u64;
                nat_eq(a849, a850)
            }
        }
    }
}

pub fn f245(v0: Adt119, v1: u64, v2: u64) -> R<Adt136> {
    let a851 = v2;
    let a852 = 34u64;
    if nat_eq(a851, a852) {
        Ok(f249(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), false)))
    } else {
        let a853 = v2;
        let a854 = 123u64;
        if nat_eq(a853, a854) {
            f307(v0, Adt124::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt120::C3, v1)
        } else {
            let a855 = v2;
            let a856 = 91u64;
            if nat_eq(a855, a856) {
                f307(v0, Adt124::C0(List::<Adt6>::nil()), Adt120::C1, v1)
            } else {
                let a857 = v2;
                let a858 = 116u64;
                if nat_eq(a857, a858) {
                    Ok(f249(v0, Adt120::C7(0u64, 1u64)))
                } else {
                    let a859 = v2;
                    let a860 = 102u64;
                    if nat_eq(a859, a860) {
                        Ok(f249(v0, Adt120::C7(1u64, 1u64)))
                    } else {
                        let a861 = v2;
                        let a862 = 110u64;
                        if nat_eq(a861, a862) {
                            Ok(f249(v0, Adt120::C7(2u64, 1u64)))
                        } else {
                            let a863 = v2;
                            let a864 = 45u64;
                            if nat_eq(a863, a864) {
                                Ok(f249(v0, f308(true, Adt123::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a865 = v2;
                                let a866 = 48u64;
                                if nat_eq(a865, a866) {
                                    Ok(f249(v0, f308(false, Adt123::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f241(v2, 49u64, 57u64) {
                                        Ok(f249(v0, f308(false, Adt123::C2, f309({
                                            let a867 = v2;
                                            let a868 = 48u64;
                                            nat_sub(a867, a868)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f250(Adt99::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f246(v0: Adt119, v1: u64) -> Adt136 {
    let m870 = match v0.clone() {
        Adt119::C0(_, h869, _) => {
            h869
        }
    };
    match m870.uncons() {
        None => {
            f250(Adt99::C2(v1))
        }
        Some((v2, v3)) => {
            let m871 = v2;
            match m871 {
                Adt124::C0(v4) => {
                    Adt136::C1(f243(Adt6::C4(f310(v4, List::<Adt6>::nil())), v3, {
                        let a873 = match v0 {
                            Adt119::C0(_, _, h872) => {
                                h872
                            }
                        };
                        let a874 = 1u64;
                        nat_sub(a873, a874)
                    }))
                }
                Adt124::C1(_, _) => {
                    f250(Adt99::C2(v1))
                }
            }
        }
    }
}

pub fn f247(v0: Adt119, v1: u64, v2: u64) -> Adt136 {
    let m876 = match v0.clone() {
        Adt119::C0(h875, _, _) => {
            h875
        }
    };
    match m876 {
        Adt120::C0 => {
            f250(Adt99::C2(v1))
        }
        Adt120::C1 => {
            f250(Adt99::C2(v1))
        }
        Adt120::C2 => {
            if f244(v2) {
                Adt136::C1(v0)
            } else {
                let a877 = v2;
                let a878 = 44u64;
                if nat_eq(a877, a878) {
                    f311(v0, v1)
                } else {
                    let a879 = v2;
                    let a880 = 93u64;
                    if nat_eq(a879, a880) {
                        f246(v0, v1)
                    } else {
                        let a881 = v2;
                        let a882 = 125u64;
                        if nat_eq(a881, a882) {
                            f248(v0, v1)
                        } else {
                            f250(Adt99::C2(v1))
                        }
                    }
                }
            }
        }
        Adt120::C3 => {
            f250(Adt99::C2(v1))
        }
        Adt120::C4 => {
            f250(Adt99::C2(v1))
        }
        Adt120::C5 => {
            f250(Adt99::C2(v1))
        }
        Adt120::C6(_, _, _) => {
            f250(Adt99::C2(v1))
        }
        Adt120::C7(_, _) => {
            f250(Adt99::C2(v1))
        }
        Adt120::C8(_) => {
            f250(Adt99::C2(v1))
        }
        Adt120::C9(_) => {
            if f244(v2) {
                Adt136::C1(v0)
            } else {
                f250(Adt99::C10(v1))
            }
        }
    }
}

pub fn f248(v0: Adt119, v1: u64) -> Adt136 {
    let m884 = match v0.clone() {
        Adt119::C0(_, h883, _) => {
            h883
        }
    };
    match m884.uncons() {
        None => {
            f250(Adt99::C2(v1))
        }
        Some((v2, v3)) => {
            let m885 = v2;
            match m885 {
                Adt124::C0(_) => {
                    f250(Adt99::C2(v1))
                }
                Adt124::C1(v5, _) => {
                    Adt136::C1(f243(Adt6::C5(f312(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a887 = match v0 {
                            Adt119::C0(_, _, h886) => {
                                h886
                            }
                        };
                        let a888 = 1u64;
                        nat_sub(a887, a888)
                    }))
                }
            }
        }
    }
}

pub fn f249(v0: Adt119, v1: Adt120) -> Adt136 {
    Adt136::C1(f313(v0, v1))
}

pub fn f250(v0: Adt99) -> Adt136 {
    Adt136::C0(v0)
}

pub fn f251(v0: Adt119, v1: u64, v2: u64, v3: Adt121, v4: List<u64>, v5: bool) -> R<Adt136> {
    let m889 = v3;
    match m889 {
        Adt121::C0 => {
            let a890 = v2;
            let a891 = 34u64;
            if nat_eq(a890, a891) {
                Ok(f314(v0, v1, v4, v5))
            } else {
                let a892 = v2;
                let a893 = 92u64;
                if nat_eq(a892, a893) {
                    Ok(f249(v0, Adt120::C6(Adt121::C1, v4, v5)))
                } else {
                    let a894 = v2;
                    let a895 = 32u64;
                    if nat_lt(a894, a895) {
                        Ok(f250(Adt99::C4(v1)))
                    } else {
                        Ok(f249(v0, Adt120::C6(Adt121::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt121::C1 => {
            let a896 = v2;
            let a897 = 117u64;
            if nat_eq(a896, a897) {
                Ok(f249(v0, Adt120::C6(Adt121::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a898 = f315(v2);
                let a899 = 0u64;
                if nat_eq(a898, a899) {
                    Ok(f250(Adt99::C5(v1)))
                } else {
                    Ok(f249(v0, Adt120::C6(Adt121::C0, List::cons(f315(v2), v4), v5)))
                }
            }
        }
        Adt121::C2(v6, v7, v8) => {
            let a900 = 15u64;
            let a901 = f316(v2);
            if nat_lt(a900, a901) {
                Ok(f250(Adt99::C5(v1)))
            } else {
                let a904 = {
                    let a902 = v6;
                    let a903 = 1u64;
                    nat_add(a902, a903)?
                };
                let a905 = 4u64;
                if nat_eq(a904, a905) {
                    f317(v0, v1, {
                        let a908 = {
                            let a906 = v7;
                            let a907 = 16u64;
                            nat_mul(a906, a907)?
                        };
                        let a909 = f316(v2);
                        nat_add(a908, a909)?
                    }, v8, v4, v5)
                } else {
                    Ok(f249(v0, Adt120::C6(Adt121::C2({
                        let a910 = v6;
                        let a911 = 1u64;
                        nat_add(a910, a911)?
                    }, {
                        let a914 = {
                            let a912 = v7;
                            let a913 = 16u64;
                            nat_mul(a912, a913)?
                        };
                        let a915 = f316(v2);
                        nat_add(a914, a915)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt121::C3(v9) => {
            let a916 = v2;
            let a917 = 92u64;
            if nat_eq(a916, a917) {
                Ok(f249(v0, Adt120::C6(Adt121::C4(v9), v4, v5)))
            } else {
                Ok(f250(Adt99::C6(v1)))
            }
        }
        Adt121::C4(v10) => {
            let a918 = v2;
            let a919 = 117u64;
            if nat_eq(a918, a919) {
                Ok(f249(v0, Adt120::C6(Adt121::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f250(Adt99::C6(v1)))
            }
        }
    }
}

pub fn f252(v0: Adt119, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt136> {
    let a920 = v2;
    let a921 = f318(v3, v4);
    if nat_eq(a920, a921) {
        let a925 = {
            let a922 = v4;
            let a923 = 1u64;
            nat_add(a922, a923)?
        };
        let a926 = {
            let a924 = f319(v3);
            length_list(a924)
        };
        if nat_eq(a925, a926) {
            Ok(f320(v0, f321(v3)))
        } else {
            Ok(f249(v0, Adt120::C7(v3, {
                let a927 = v4;
                let a928 = 1u64;
                nat_add(a927, a928)?
            })))
        }
    } else {
        Ok(f250(Adt99::C2(v1)))
    }
}

pub fn f253(v0: Adt119, v1: u64, v2: u64, v3: Adt122) -> R<Adt136> {
    let m930 = match v3.clone() {
        Adt122::C0(_, h929, _, _, _, _, _) => {
            h929
        }
    };
    match m930 {
        Adt123::C0 => {
            let a931 = v2;
            let a932 = 48u64;
            if nat_eq(a931, a932) {
                Ok(f322(v0, v3, Adt123::C1))
            } else {
                if f241(v2, 49u64, 57u64) {
                    f323(v0, v1, v2, v3, false)
                } else {
                    Ok(f250(Adt99::C7(v1)))
                }
            }
        }
        Adt123::C1 => {
            let a933 = v2;
            let a934 = 46u64;
            if nat_eq(a933, a934) {
                Ok(f322(v0, v3, Adt123::C3))
            } else {
                if f324(v2) {
                    Ok(f322(v0, v3, Adt123::C5))
                } else {
                    if f325(v2) {
                        Ok(f250(Adt99::C7(v1)))
                    } else {
                        f326(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt123::C2 => {
            if f325(v2) {
                f323(v0, v1, v2, v3, false)
            } else {
                let a935 = v2;
                let a936 = 46u64;
                if nat_eq(a935, a936) {
                    Ok(f322(v0, v3, Adt123::C3))
                } else {
                    if f324(v2) {
                        Ok(f322(v0, v3, Adt123::C5))
                    } else {
                        f326(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt123::C3 => {
            if f325(v2) {
                f323(v0, v1, v2, v3, true)
            } else {
                Ok(f250(Adt99::C7(v1)))
            }
        }
        Adt123::C4 => {
            if f325(v2) {
                f323(v0, v1, v2, v3, true)
            } else {
                if f324(v2) {
                    Ok(f322(v0, v3, Adt123::C5))
                } else {
                    f326(v0, v1, v2, v3)
                }
            }
        }
        Adt123::C5 => {
            let a937 = v2;
            let a938 = 43u64;
            if nat_eq(a937, a938) {
                Ok(f249(v0, f308(match v3.clone() {
                    Adt122::C0(h939, _, _, _, _, _, _) => {
                        h939
                    }
                }, Adt123::C6, match v3.clone() {
                    Adt122::C0(_, _, h940, _, _, _, _) => {
                        h940
                    }
                }, match v3.clone() {
                    Adt122::C0(_, _, _, h941, _, _, _) => {
                        h941
                    }
                }, match v3 {
                    Adt122::C0(_, _, _, _, h942, _, _) => {
                        h942
                    }
                }, false, 0i64)))
            } else {
                let a943 = v2;
                let a944 = 45u64;
                if nat_eq(a943, a944) {
                    Ok(f249(v0, f308(match v3.clone() {
                        Adt122::C0(h945, _, _, _, _, _, _) => {
                            h945
                        }
                    }, Adt123::C6, match v3.clone() {
                        Adt122::C0(_, _, h946, _, _, _, _) => {
                            h946
                        }
                    }, match v3.clone() {
                        Adt122::C0(_, _, _, h947, _, _, _) => {
                            h947
                        }
                    }, match v3 {
                        Adt122::C0(_, _, _, _, h948, _, _) => {
                            h948
                        }
                    }, true, 0i64)))
                } else {
                    if f325(v2) {
                        f327(v0, v1, v2, Adt122::C0(match v3.clone() {
                            Adt122::C0(h949, _, _, _, _, _, _) => {
                                h949
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, h950, _, _, _, _, _) => {
                                h950
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, _, h951, _, _, _, _) => {
                                h951
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, _, _, h952, _, _, _) => {
                                h952
                            }
                        }, match v3 {
                            Adt122::C0(_, _, _, _, h953, _, _) => {
                                h953
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f250(Adt99::C7(v1)))
                    }
                }
            }
        }
        Adt123::C6 => {
            if f325(v2) {
                f327(v0, v1, v2, v3.clone(), match v3 {
                    Adt122::C0(_, _, _, _, _, h954, _) => {
                        h954
                    }
                })
            } else {
                Ok(f250(Adt99::C7(v1)))
            }
        }
        Adt123::C7 => {
            if f325(v2) {
                f327(v0, v1, v2, v3.clone(), match v3 {
                    Adt122::C0(_, _, _, _, _, h955, _) => {
                        h955
                    }
                })
            } else {
                f326(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f254(v0: u64) -> bool {
    let a956 = v0;
    let a957 = 9u64;
    if nat_eq(a956, a957) {
        true
    } else {
        let a958 = v0;
        let a959 = 32u64;
        nat_eq(a958, a959)
    }
}

pub fn f255(v0: u64) -> bool {
    let a960 = 47u64;
    let a961 = v0;
    if nat_lt(a960, a961) {
        let a962 = v0;
        let a963 = 58u64;
        nat_lt(a962, a963)
    } else {
        false
    }
}

pub fn f256(v0: u64) -> bool {
    let a964 = 48u64;
    let a965 = v0;
    if if nat_le(a964, a965) {
        let a966 = v0;
        let a967 = 57u64;
        nat_le(a966, a967)
    } else {
        false
    } {
        true
    } else {
        let a968 = 65u64;
        let a969 = v0;
        if if nat_le(a968, a969) {
            let a970 = v0;
            let a971 = 90u64;
            nat_le(a970, a971)
        } else {
            false
        } {
            true
        } else {
            let a972 = 97u64;
            let a973 = v0;
            if if nat_le(a972, a973) {
                let a974 = v0;
                let a975 = 122u64;
                nat_le(a974, a975)
            } else {
                false
            } {
                true
            } else {
                let a976 = v0;
                let a977 = 33u64;
                if nat_eq(a976, a977) {
                    true
                } else {
                    let a978 = v0;
                    let a979 = 35u64;
                    if nat_eq(a978, a979) {
                        true
                    } else {
                        let a980 = v0;
                        let a981 = 36u64;
                        if nat_eq(a980, a981) {
                            true
                        } else {
                            let a982 = v0;
                            let a983 = 37u64;
                            if nat_eq(a982, a983) {
                                true
                            } else {
                                let a984 = v0;
                                let a985 = 38u64;
                                if nat_eq(a984, a985) {
                                    true
                                } else {
                                    let a986 = v0;
                                    let a987 = 39u64;
                                    if nat_eq(a986, a987) {
                                        true
                                    } else {
                                        let a988 = v0;
                                        let a989 = 42u64;
                                        if nat_eq(a988, a989) {
                                            true
                                        } else {
                                            let a990 = v0;
                                            let a991 = 43u64;
                                            if nat_eq(a990, a991) {
                                                true
                                            } else {
                                                let a992 = v0;
                                                let a993 = 45u64;
                                                if nat_eq(a992, a993) {
                                                    true
                                                } else {
                                                    let a994 = v0;
                                                    let a995 = 46u64;
                                                    if nat_eq(a994, a995) {
                                                        true
                                                    } else {
                                                        let a996 = v0;
                                                        let a997 = 94u64;
                                                        if nat_eq(a996, a997) {
                                                            true
                                                        } else {
                                                            let a998 = v0;
                                                            let a999 = 95u64;
                                                            if nat_eq(a998, a999) {
                                                                true
                                                            } else {
                                                                let a1000 = v0;
                                                                let a1001 = 96u64;
                                                                if nat_eq(a1000, a1001) {
                                                                    true
                                                                } else {
                                                                    let a1002 = v0;
                                                                    let a1003 = 124u64;
                                                                    if nat_eq(a1002, a1003) {
                                                                        true
                                                                    } else {
                                                                        let a1004 = v0;
                                                                        let a1005 = 126u64;
                                                                        nat_eq(a1004, a1005)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f257(v0: u64) -> bool {
    let a1006 = 48u64;
    let a1007 = v0;
    if if nat_le(a1006, a1007) {
        let a1008 = v0;
        let a1009 = 57u64;
        nat_le(a1008, a1009)
    } else {
        false
    } {
        true
    } else {
        let a1010 = 65u64;
        let a1011 = v0;
        if if nat_le(a1010, a1011) {
            let a1012 = v0;
            let a1013 = 90u64;
            nat_le(a1012, a1013)
        } else {
            false
        } {
            true
        } else {
            let a1014 = 97u64;
            let a1015 = v0;
            if if nat_le(a1014, a1015) {
                let a1016 = v0;
                let a1017 = 122u64;
                nat_le(a1016, a1017)
            } else {
                false
            } {
                true
            } else {
                let a1018 = v0;
                let a1019 = 95u64;
                if nat_eq(a1018, a1019) {
                    true
                } else {
                    let a1020 = v0;
                    let a1021 = 45u64;
                    if nat_eq(a1020, a1021) {
                        true
                    } else {
                        let a1022 = v0;
                        let a1023 = 46u64;
                        nat_eq(a1022, a1023)
                    }
                }
            }
        }
    }
}

pub fn f258() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f259(v0: Adt6) -> Option<bool> {
    let m1024 = v0;
    match m1024 {
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

pub fn f260(v0: Adt6) -> Option<Adt5> {
    let m1025 = v0;
    match m1025 {
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

pub fn f261(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt125, v6: List<Adt105>) -> R<Result<(List<Adt105>, Adt125), Adt97>> {
    let m1026 = f328(v0.clone(), v1, v2.clone());
    match m1026 {
        None => {
            let m1027 = f329(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1027 {
                Adt147::C0(v8) => {
                    Ok(Err::<(List<Adt105>, Adt125), Adt97>(v8))
                }
                Adt147::C1(v9) => {
                    Ok(Ok::<(List<Adt105>, Adt125), Adt97>((f330(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt105>, Adt125), Adt97>(v7))
        }
    }
}

pub fn f262(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> R<Adt137> {
    let m1028 = f331(v1.clone(), v2.clone())?;
    match m1028 {
        Adt137::C0(v4) => {
            Ok(Adt137::C0(v4))
        }
        Adt137::C1(v5) => {
            Ok(Adt137::C1(v5))
        }
        Adt137::C2(v6) => {
            Ok(Adt137::C2(v6))
        }
        Adt137::C3(_) => {
            f332(v0, v1, v2, v3)
        }
    }
}

pub fn f263(v0: List<Adt6>, v1: Adt6, v2: List<Adt107>, v3: u64, v4: List<Adt109>) -> Adt126 {
    let m1029 = v0;
    match m1029.uncons() {
        None => {
            let a1030 = v3;
            let a1031 = 1u64;
            if nat_eq(a1030, a1031) {
                Adt126::C0(Adt108::C0, v4)
            } else {
                Adt126::C0(Adt108::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt126::C0(Adt108::C0, List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f264(v0: Option<Adt6>, v1: Adt6, v2: List<Adt107>, v3: List<Adt109>) -> Adt126 {
    let m1032 = v0;
    match m1032 {
        None => {
            Adt126::C0(Adt108::C0, v3)
        }
        Some(v4) => {
            Adt126::C0(Adt108::C0, List::cons(Adt109::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f265(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1033 = f237(Str::lit("anyOf"), v0);
    match m1033 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1034 = f333(v3);
            match m1034 {
                None => {
                    Adt137::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1035 = v4;
                    match m1035.uncons() {
                        None => {
                            Adt137::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt137::C3(List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C1(v6, v1, v2), List::<Adt109>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f266(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1036 = f237(Str::lit("oneOf"), v0);
    match m1036 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1037 = f333(v3);
            match m1037 {
                None => {
                    Adt137::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1038 = v4;
                    match m1038.uncons() {
                        None => {
                            Adt137::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt137::C3(List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C2(v6, v1, v2, 0u64), List::<Adt109>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f267(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1039 = f237(Str::lit("not"), v0);
    match m1039 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            Adt137::C3(List::cons(Adt109::C0(v3, v1, v2.clone()), List::cons(Adt109::C3(v2), List::<Adt109>::nil())))
        }
    }
}

pub fn f268(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1040 = f237(Str::lit("if"), v0.clone());
    match m1040 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            Adt137::C3(List::cons(Adt109::C0(v3, v1.clone(), v2.clone()), List::cons(Adt109::C4(f334(Str::lit("then"), v0.clone()), f334(Str::lit("else"), v0), v1, v2), List::<Adt109>::nil())))
        }
    }
}

pub fn f269(v0: List<Adt6>, v1: Adt6, v2: List<Adt107>, v3: List<Adt109>) -> Adt126 {
    let m1041 = v0;
    match m1041.uncons() {
        None => {
            Adt126::C0(Adt108::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt126::C0(Adt108::C0, List::cons(Adt109::C0(v4, v1.clone(), v2.clone()), List::cons(Adt109::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f270(v0: Str) -> R<bool> {
    let m1043 = f298(f85({
        let a1042 = v0;
        utf8_encode(a1042)
    })?)?;
    match m1043 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f271(v0: Adt38) -> bool {
    let m1045 = match v0 {
        Adt38::C0(_, _, h1044) => {
            h1044
        }
    };
    match m1045 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f272(v0: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1047 = match v0.clone() {
        Adt50::C0(_, h1046) => {
            h1046
        }
    };
    match m1047.uncons() {
        None => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(match v0 {
                Adt50::C0(h1048, _) => {
                    h1048
                }
            })))
        }
        Some((_, _)) => {
            if f335(match v0.clone() {
                Adt50::C0(_, h1049) => {
                    h1049
                }
            })? {
                Ok(Ok::<Adt33, Adt90>(Adt33::C3(v0)))
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f273(v0: Adt5, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1053 = {
        let m1052 = match v1.clone() {
            Adt50::C0(h1051, _) => {
                h1051
            }
        };
        match m1052 {
            Adt38::C0(_, h1050, _) => {
                h1050
            }
        }
    };
    match m1053 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1056 = f116(Adt6::C5(match v0.clone() {
                Adt5::C0(h1054) => {
                    h1054
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1055) => {
                    h1055
                }
            }), v2)?;
            match m1056 {
                Adt106::C0 => {
                    f272(v1)
                }
                Adt106::C1(_, _) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt106::C2(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt106::C3(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt106::C4 => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f274(v0: Adt107) -> R<Str> {
    let m1057 = v0;
    match m1057 {
        Adt107::C0(v1) => {
            f336(v1)
        }
        Adt107::C1(v2) => {
            let a1058 = f79(v2, 0i64)?;
            Ok(format_i64(a1058))
        }
    }
}

pub fn f275(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1059 = v0;
    let a1060 = f337(f87(), f338(List::<Bytes>::nil(), v1));
    append_bytes(a1059, a1060)
}

pub fn f276(v0: i64, v1: Str, v2: Str, v3: Adt95) -> Result<(Adt14, u64), Adt93> {
    Ok::<(Adt14, u64), Adt93>((Adt14::C0(v0, v1, v2, match v3.clone() {
        Adt95::C0(h1061, _) => {
            h1061
        }
    }), match v3 {
        Adt95::C0(_, h1062) => {
            h1062
        }
    }))
}

pub fn f277(v0: u64, v1: Adt6, v2: List<Adt111>, v3: List<u64>) -> R<Adt138> {
    let m1063 = v1;
    match m1063 {
        Adt6::C0 => {
            Ok(Adt138::C1(v2, f167(f339(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt138::C1(v2, f167(if v4 {
                f339(0u64)
            } else {
                f339(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f340(v5.clone())? {
                Ok(Adt138::C1(v2, f167(f341(v5)?, v3)))
            } else {
                Ok(Adt138::C0(Adt99::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt138::C1(v2, f167(f278(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1064 = v0;
            let a1065 = f342();
            if nat_lt(a1064, a1065) {
                Ok(Adt138::C1(f343(91u64, Adt111::C2({
                    let a1066 = v0;
                    let a1067 = 1u64;
                    nat_add(a1066, a1067)?
                }, v7), v2), v3))
            } else {
                Ok(Adt138::C0(Adt99::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1068 = v0;
            let a1069 = f342();
            if nat_lt(a1068, a1069) {
                Ok(Adt138::C1(f343(123u64, Adt111::C4({
                    let a1070 = v0;
                    let a1071 = 1u64;
                    nat_add(a1070, a1071)?
                }, v8), v2), v3))
            } else {
                Ok(Adt138::C0(Adt99::C12))
            }
        }
    }
}

pub fn f278(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1073 = f167(f344(f85({
            let a1072 = v0;
            utf8_encode(a1072)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1074 = List::cons(34u64, List::<u64>::nil());
        append_list(a1073, a1074)
    }))
}

pub fn f279(v0: Adt33) -> R<Option<List<(Str, Adt6)>>> {
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
            f347(f355(v12))
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

pub fn f280(v0: Adt128) -> Adt5 {
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

pub fn f281(v0: Option<Option<List<u64>>>, v1: Adt94) -> Adt94 {
    let m1084 = v0;
    match m1084 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt94::C0(match v1.clone() {
                Adt94::C0(h1085, _, _, _, _) => {
                    h1085
                }
            }, match v1.clone() {
                Adt94::C0(_, h1086, _, _, _) => {
                    h1086
                }
            }, match v1.clone() {
                Adt94::C0(_, _, h1087, _, _) => {
                    h1087
                }
            }, match v1 {
                Adt94::C0(_, _, _, h1088, _) => {
                    h1088
                }
            }, v2)
        }
    }
}

pub fn f282(v0: Adt14, v1: Adt88) -> R<Option<Option<List<u64>>>> {
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

pub fn f283(v0: bool, v1: Adt94, v2: List<List<u64>>) -> Adt94 {
    if f364(v0, match v1.clone() {
        Adt94::C0(h1094, _, _, _, _) => {
            h1094
        }
    }, v2.clone()) {
        f365(match v1.clone() {
            Adt94::C0(h1095, _, _, _, _) => {
                h1095
            }
        }, {
            let a1097 = v2;
            let a1098 = List::cons(match v1 {
                Adt94::C0(_, _, _, h1096, _) => {
                    h1096
                }
            }, List::<List<u64>>::nil());
            append_list(a1097, a1098)
        })
    } else {
        v1
    }
}

pub fn f284(v0: Adt130) -> R<Option<List<u64>>> {
    let m1099 = f366(f367(v0))?;
    match m1099 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1100 = f89(v1)?;
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

pub fn f285(v0: Adt88) -> R<Option<List<List<u64>>>> {
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
                        let m1106 = f89(v2)?;
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

pub fn f286() -> u64 {
    404u64
}

pub fn f287(v0: Adt90) -> u64 {
    let m1107 = v0;
    match m1107 {
        Adt90::C0(_) => {
            f96()
        }
        Adt90::C1(_) => {
            f96()
        }
        Adt90::C2(_) => {
            f96()
        }
        Adt90::C3(_) => {
            f96()
        }
        Adt90::C4(_) => {
            f96()
        }
        Adt90::C5(_) => {
            f96()
        }
        Adt90::C6(_) => {
            f21()
        }
        Adt90::C7(_) => {
            f370()
        }
        Adt90::C8(_) => {
            f96()
        }
        Adt90::C9(_) => {
            f21()
        }
    }
}

pub fn f288(v0: Adt131) -> i64 {
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

pub fn f289(v0: Adt131) -> Str {
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
            }
        }
    }
}

pub fn f290(v0: Adt131) -> R<Option<Adt6>> {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f79(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f379(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f291(v0: List<u64>) -> R<bool> {
    let m1113 = f222(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
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

pub fn f292(v0: Adt0, v1: Fn2, v2: u64, v3: Adt139) -> R<Result<Adt128, Adt131>> {
    f382(v0.clone(), v1, v2, v3.clone(), f383(match v0 {
        Adt0::C0(_, _, h1114, _, _, _) => {
            h1114
        }
    }, match v3 {
        Adt139::C0(_, h1115, _, _) => {
            h1115
        }
    }))
}

pub fn f293(v0: Adt142, v1: Adt143) -> Result<Adt139, Adt131> {
    let m1116 = v1;
    match m1116 {
        Adt143::C0(v3) => {
            f384(v3, v0)
        }
        Adt143::C1(v2) => {
            Err::<Adt139, Adt131>(v2)
        }
    }
}

pub fn f294(v0: Str) -> Adt142 {
    let a1117 = v0.clone();
    let a1118 = Str::lit("server/discover");
    if equal(a1117, a1118) {
        Adt142::C1(Adt140::C0)
    } else {
        let a1119 = v0.clone();
        let a1120 = Str::lit("tools/list");
        if equal(a1119, a1120) {
            Adt142::C1(Adt140::C1)
        } else {
            let a1121 = v0.clone();
            let a1122 = Str::lit("tools/call");
            if equal(a1121, a1122) {
                Adt142::C1(Adt140::C2)
            } else {
                let a1123 = v0.clone();
                let a1124 = Str::lit("resources/list");
                if equal(a1123, a1124) {
                    Adt142::C1(Adt140::C3)
                } else {
                    let a1125 = v0.clone();
                    let a1126 = Str::lit("resources/read");
                    if equal(a1125, a1126) {
                        Adt142::C1(Adt140::C4)
                    } else {
                        let a1127 = v0.clone();
                        let a1128 = Str::lit("resources/templates/list");
                        if equal(a1127, a1128) {
                            Adt142::C1(Adt140::C5)
                        } else {
                            let a1129 = v0.clone();
                            let a1130 = Str::lit("prompts/list");
                            if equal(a1129, a1130) {
                                Adt142::C1(Adt140::C6)
                            } else {
                                let a1131 = v0.clone();
                                let a1132 = Str::lit("prompts/get");
                                if equal(a1131, a1132) {
                                    Adt142::C1(Adt140::C7)
                                } else {
                                    let a1133 = v0.clone();
                                    let a1134 = Str::lit("completion/complete");
                                    if equal(a1133, a1134) {
                                        Adt142::C1(Adt140::C8)
                                    } else {
                                        let a1135 = v0.clone();
                                        let a1136 = Str::lit("tasks/get");
                                        if equal(a1135, a1136) {
                                            Adt142::C1(Adt140::C9)
                                        } else {
                                            let a1137 = v0.clone();
                                            let a1138 = Str::lit("tasks/update");
                                            if equal(a1137, a1138) {
                                                Adt142::C1(Adt140::C10)
                                            } else {
                                                let a1139 = v0;
                                                let a1140 = Str::lit("tasks/cancel");
                                                if equal(a1139, a1140) {
                                                    Adt142::C1(Adt140::C11)
                                                } else {
                                                    Adt142::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f295(v0: Option<Adt112>, v1: List<Str>, v2: Adt104) -> R<Adt143> {
    f385(v0, v1, v2.clone(), f386(match v2 {
        Adt104::C0(_, _, h1141) => {
            h1141
        }
    }))
}

pub fn f296(v0: Str) -> Adt145 {
    let m1142 = f294(v0);
    match m1142 {
        Adt142::C0 => {
            Adt145::C0
        }
        Adt142::C1(v1) => {
            f387(v1)
        }
    }
}

pub fn f297(v0: List<u64>) -> Option<List<u64>> {
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

pub fn f298(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1146 = f391({
        let a1145 = v0.clone();
        length_list(a1145)
    }, v0, List::<u64>::nil())?;
    match m1146 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f167(v1, List::<u64>::nil())))
        }
    }
}

pub fn f299(p1149: List<Adt115>, p1150: List<Adt115>) -> List<Adt115> {
    let m1148 = (p1149, p1150);
    {
        let mut m1148 = m1148;
        loop {
            let step = {
                let (v0, v1) = m1148;
                let m1147 = v0;
                match m1147.uncons() {
                    None => {
                        Err::<(List<Adt115>, List<Adt115>), List<Adt115>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt115>, List<Adt115>), List<Adt115>>((v3, List::cons(v2, v1)))
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

pub fn f300(v0: List<Adt92>, p1157: List<Adt12>, v2: Option<Str>, p1158: List<Adt115>) -> R<List<Adt115>> {
    let m1156 = (p1157, p1158);
    {
        let mut m1156 = m1156;
        loop {
            let step = {
                let (v1, v3) = m1156;
                let m1151 = v1;
                match m1151.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f392(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>((v5, List::cons(Adt115::C0(match v4.clone() {
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
                            Ok::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>((v5, v3))
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

pub fn f301(p1174: u64, p1175: i64, p1176: i64) -> R<Option<i64>> {
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
                                }, f115(v2, 1i64)?)))
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

pub fn f302(v0: i64) -> Option<i64> {
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

pub fn f303(v0: Str, v1: Str) -> Adt146 {
    let a1183 = v0;
    let a1184 = v1;
    if equal(a1183, a1184) {
        Adt146::C0
    } else {
        Adt146::C1
    }
}

pub fn f304(v0: Adt135) -> bool {
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

pub fn f305(v0: Adt98, v1: Str, v2: Adt5) -> Result<Adt104, Adt100> {
    let m1186 = f180(Str::lit("params"), v2);
    match m1186 {
        Adt135::C0 => {
            Ok::<Adt104, Adt100>(Adt104::C0(v0, v1, None::<Adt5>))
        }
        Adt135::C1(v3) => {
            let m1187 = f260(v3);
            match m1187 {
                None => {
                    f136(Some(v0), Adt101::C6)
                }
                Some(v4) => {
                    Ok::<Adt104, Adt100>(Adt104::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f306(v0: Adt123) -> bool {
    let m1188 = v0;
    match m1188 {
        Adt123::C0 => {
            false
        }
        Adt123::C1 => {
            true
        }
        Adt123::C2 => {
            true
        }
        Adt123::C3 => {
            false
        }
        Adt123::C4 => {
            true
        }
        Adt123::C5 => {
            false
        }
        Adt123::C6 => {
            false
        }
        Adt123::C7 => {
            true
        }
    }
}

pub fn f307(v0: Adt119, v1: Adt124, v2: Adt120, v3: u64) -> R<Adt136> {
    let a1190 = match v0.clone() {
        Adt119::C0(_, _, h1189) => {
            h1189
        }
    };
    let a1191 = f342();
    if nat_lt(a1190, a1191) {
        Ok(Adt136::C1(Adt119::C0(v2, List::cons(v1, match v0.clone() {
            Adt119::C0(_, h1192, _) => {
                h1192
            }
        }), {
            let a1194 = match v0 {
                Adt119::C0(_, _, h1193) => {
                    h1193
                }
            };
            let a1195 = 1u64;
            nat_add(a1194, a1195)?
        })))
    } else {
        Ok(f250(Adt99::C9(v3)))
    }
}

pub fn f308(v0: bool, v1: Adt123, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt120 {
    Adt120::C8(Adt122::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f309(v0: u64) -> i64 {
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

pub fn f310(p1216: List<Adt6>, p1217: List<Adt6>) -> List<Adt6> {
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

pub fn f311(v0: Adt119, v1: u64) -> Adt136 {
    let m1219 = match v0.clone() {
        Adt119::C0(_, h1218, _) => {
            h1218
        }
    };
    match m1219.uncons() {
        None => {
            f250(Adt99::C2(v1))
        }
        Some((v2, _)) => {
            let m1220 = v2;
            match m1220 {
                Adt124::C0(_) => {
                    f249(v0, Adt120::C0)
                }
                Adt124::C1(_, _) => {
                    f249(v0, Adt120::C4)
                }
            }
        }
    }
}

pub fn f312(p1223: List<(Str, Adt6)>, p1224: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
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

pub fn f313(v0: Adt119, v1: Adt120) -> Adt119 {
    Adt119::C0(v1, match v0.clone() {
        Adt119::C0(_, h1225, _) => {
            h1225
        }
    }, match v0 {
        Adt119::C0(_, _, h1226) => {
            h1226
        }
    })
}

pub fn f314(v0: Adt119, v1: u64, v2: List<u64>, v3: bool) -> Adt136 {
    let m1228 = {
        let a1227 = f275(f87(), f167(v2, List::<u64>::nil()));
        utf8_decode(a1227)
    };
    match m1228 {
        None => {
            f250(Adt99::C14)
        }
        Some(v4) => {
            if v3 {
                f395(v0, v1, v4)
            } else {
                f320(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f315(v0: u64) -> u64 {
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

pub fn f316(v0: u64) -> u64 {
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

pub fn f317(v0: Adt119, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt136> {
    let a1263 = v3;
    let a1264 = 0u64;
    if nat_eq(a1263, a1264) {
        if f241(v2, 55296u64, 56319u64) {
            Ok(f249(v0, Adt120::C6(Adt121::C3(v2), v4, v5)))
        } else {
            if f241(v2, 56320u64, 57343u64) {
                Ok(f250(Adt99::C6(v1)))
            } else {
                Ok(f249(v0, Adt120::C6(Adt121::C0, f396(v2, v4)?, v5)))
            }
        }
    } else {
        if f241(v2, 56320u64, 57343u64) {
            Ok(f249(v0, Adt120::C6(Adt121::C0, f396({
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
            Ok(f250(Adt99::C6(v1)))
        }
    }
}

pub fn f318(v0: u64, v1: u64) -> u64 {
    let m1277 = {
        let a1275 = f319(v0);
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

pub fn f319(v0: u64) -> List<u64> {
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

pub fn f320(v0: Adt119, v1: Adt6) -> Adt136 {
    Adt136::C1(f243(v1, match v0.clone() {
        Adt119::C0(_, h1282, _) => {
            h1282
        }
    }, match v0 {
        Adt119::C0(_, _, h1283) => {
            h1283
        }
    }))
}

pub fn f321(v0: u64) -> Adt6 {
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

pub fn f322(v0: Adt119, v1: Adt122, v2: Adt123) -> Adt136 {
    f249(v0, Adt120::C8(Adt122::C0(match v1.clone() {
        Adt122::C0(h1288, _, _, _, _, _, _) => {
            h1288
        }
    }, v2, match v1.clone() {
        Adt122::C0(_, _, h1289, _, _, _, _) => {
            h1289
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, h1290, _, _, _) => {
            h1290
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, _, h1291, _, _) => {
            h1291
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, _, _, h1292, _) => {
            h1292
        }
    }, match v1 {
        Adt122::C0(_, _, _, _, _, _, h1293) => {
            h1293
        }
    })))
}

pub fn f323(v0: Adt119, v1: u64, v2: u64, v3: Adt122, v4: bool) -> R<Adt136> {
    let v5: u64 = {
        let a1295 = match v3.clone() {
            Adt122::C0(_, _, _, h1294, _, _, _) => {
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
                Adt122::C0(_, _, _, h1299, _, _, _) => {
                    h1299
                }
            };
            let a1301 = 1u64;
            nat_add(a1300, a1301)?
        }
    };
    let v6: i64 = if v4 {
        f115(match v3.clone() {
            Adt122::C0(_, _, _, _, h1302, _, _) => {
                h1302
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt122::C0(_, _, _, _, h1303, _, _) => {
                h1303
            }
        }
    };
    let a1304 = f397();
    let a1305 = v5;
    if nat_lt(a1304, a1305) {
        Ok(f250(Adt99::C8(v1)))
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
            Ok(f250(Adt99::C8(v1)))
        } else {
            Ok(f249(v0, f308(match v3.clone() {
                Adt122::C0(h1309, _, _, _, _, _, _) => {
                    h1309
                }
            }, if v4 {
                Adt123::C4
            } else {
                Adt123::C2
            }, f115({
                let a1311 = match v3.clone() {
                    Adt122::C0(_, _, h1310, _, _, _, _) => {
                        h1310
                    }
                };
                let a1312 = 10i64;
                int_mul(a1311, a1312)?
            }, f309({
                let a1313 = v2;
                let a1314 = 48u64;
                nat_sub(a1313, a1314)
            }))?, v5, v6, match v3.clone() {
                Adt122::C0(_, _, _, _, _, h1315, _) => {
                    h1315
                }
            }, match v3 {
                Adt122::C0(_, _, _, _, _, _, h1316) => {
                    h1316
                }
            })))
        }
    }
}

pub fn f324(v0: u64) -> bool {
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

pub fn f325(v0: u64) -> bool {
    f241(v0, 48u64, 57u64)
}

pub fn f326(v0: Adt119, v1: u64, v2: u64, v3: Adt122) -> R<Adt136> {
    let m1321 = f242(v3)?;
    match m1321 {
        Adt141::C0 => {
            Ok(f250(Adt99::C7(v1)))
        }
        Adt141::C1 => {
            Ok(f250(Adt99::C8(v1)))
        }
        Adt141::C2(v4) => {
            Ok(f247(f243(v4, match v0.clone() {
                Adt119::C0(_, h1322, _) => {
                    h1322
                }
            }, match v0 {
                Adt119::C0(_, _, h1323) => {
                    h1323
                }
            }), v1, v2))
        }
    }
}

pub fn f327(v0: Adt119, v1: u64, v2: u64, v3: Adt122, v4: bool) -> R<Adt136> {
    let v5: i64 = f115({
        let a1325 = match v3.clone() {
            Adt122::C0(_, _, _, _, _, _, h1324) => {
                h1324
            }
        };
        let a1326 = 10i64;
        int_mul(a1325, a1326)?
    }, f309({
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
        Ok(f250(Adt99::C8(v1)))
    } else {
        Ok(f249(v0, f308(match v3.clone() {
            Adt122::C0(h1332, _, _, _, _, _, _) => {
                h1332
            }
        }, Adt123::C7, match v3.clone() {
            Adt122::C0(_, _, h1333, _, _, _, _) => {
                h1333
            }
        }, match v3.clone() {
            Adt122::C0(_, _, _, h1334, _, _, _) => {
                h1334
            }
        }, match v3 {
            Adt122::C0(_, _, _, _, h1335, _, _) => {
                h1335
            }
        }, v4, v5)))
    }
}

pub fn f328(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1336 = f398(f399(), v2.clone());
    match m1336 {
        None => {
            let m1337 = f400(f401(), v2.clone());
            match m1337 {
                None => {
                    f402(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt97::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt97::C4(v0, v3))
        }
    }
}

pub fn f329(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt125) -> R<Adt147> {
    let m1338 = f237(Str::lit("x-mcp-header"), v1.clone());
    match m1338 {
        Adt135::C0 => {
            Ok(Adt147::C1(v4))
        }
        Adt135::C1(v5) => {
            f403(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f330(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt105>) -> List<Adt105> {
    f404(Str::lit("properties"), v0.clone(), v1, v2, f404(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f405(Str::lit("items"), v0.clone(), f405(Str::lit("additionalProperties"), v0.clone(), f406(Str::lit("allOf"), v0.clone(), f406(Str::lit("anyOf"), v0.clone(), f406(Str::lit("oneOf"), v0.clone(), f405(Str::lit("not"), v0.clone(), f405(Str::lit("if"), v0.clone(), f405(Str::lit("then"), v0.clone(), f405(Str::lit("else"), v0, v3)))))))))))
}

pub fn f331(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt137> {
    Ok(f407(List::cons((Str::lit("type"), f408(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f409(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f410(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f411(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f411(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f411(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f411(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1339 = f412(v0.clone(), v1.clone())?;
        let a1340 = List::cons((Str::lit("required"), f413(v0, v1)), List::<(Str, Adt148)>::nil());
        append_list(a1339, a1340)
    })))))))))
}

pub fn f332(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> R<Adt137> {
    Ok(f414(f415(v0, v1.clone(), v2.clone(), v3.clone()), f414(f416(v1.clone(), v2.clone(), v3.clone()), f414(f417(v1.clone(), v2.clone(), v3.clone()), f414(f418(v1.clone(), v2.clone(), v3.clone())?, f414(f419(v1.clone(), v2.clone(), v3.clone()), f414(f420(Adt110::C0, f265(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f414(f420(Adt110::C1, f266(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f414(f420(Adt110::C2, f267(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f420(Adt110::C3, f268(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f333(v0: Adt6) -> Option<List<Adt6>> {
    let m1341 = v0;
    match m1341 {
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

pub fn f334(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1342 = f237(v0, v1);
    match m1342 {
        Adt135::C0 => {
            None::<Adt6>
        }
        Adt135::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f335(v0: List<Adt51>) -> R<bool> {
    let m1343 = v0;
    match m1343.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f421(v2, match v1 {
                Adt51::C0(h1344, _, _) => {
                    h1344
                }
            })
        }
    }
}

pub fn f336(v0: Str) -> R<Str> {
    let m1347 = {
        let a1346 = f275(f87(), f167(f422(f85({
            let a1345 = v0.clone();
            utf8_encode(a1345)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1346)
    };
    match m1347 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f337(p1352: Bytes, p1353: List<Bytes>) -> Bytes {
    let m1351 = (p1352, p1353);
    {
        let mut m1351 = m1351;
        loop {
            let step = {
                let (v0, v1) = m1351;
                let m1348 = v1;
                match m1348.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1349 = v2;
                            let a1350 = v0;
                            append_bytes(a1349, a1350)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(p1356: List<Bytes>, p1357: List<u64>) -> List<Bytes> {
    let m1355 = (p1356, p1357);
    {
        let mut m1355 = m1355;
        loop {
            let step = {
                let (v0, v1) = m1355;
                let m1354 = v1;
                match m1354.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f423(f424(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: u64) -> List<u64> {
    f319(v0)
}

pub fn f340(v0: Adt7) -> R<bool> {
    let a1375 = {
        let m1361 = {
            let a1359 = f425();
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
                    let a1363 = f425();
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

pub fn f341(v0: Adt7) -> R<List<u64>> {
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
        let a1386 = f426(match v0.clone() {
            Adt7::C0(h1384, _) => {
                h1384
            }
        })?;
        let a1387 = List::cons(101u64, f426(match v0 {
            Adt7::C0(_, h1385) => {
                h1385
            }
        })?);
        Ok(append_list(a1386, a1387))
    } else {
        f426(match v0 {
            Adt7::C0(h1388, _) => {
                h1388
            }
        })
    }
}

pub fn f342() -> u64 {
    512u64
}

pub fn f343(v0: u64, v1: Adt111, v2: List<Adt111>) -> List<Adt111> {
    List::cons(Adt111::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f344(p1391: List<u64>, p1392: List<u64>) -> R<List<u64>> {
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
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f167(f427(v2)?, v1)))
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

pub fn f345(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f428(match v0.clone() {
        Adt34::C0(_, h1393, _, _) => {
            h1393
        }
    })), List::cons((Str::lit("capabilities"), f429(match v0.clone() {
        Adt34::C0(_, _, h1394, _) => {
            h1394
        }
    })), {
        let a1397 = f430(Str::lit("instructions"), match v0.clone() {
            Adt34::C0(_, _, _, h1395) => {
                h1395
            }
        });
        let a1398 = f431(match v0 {
            Adt34::C0(h1396, _, _, _) => {
                h1396
            }
        })?;
        append_list(a1397, a1398)
    })))
}

pub fn f346(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f432(match v0.clone() {
        Adt35::C0(_, _, h1399) => {
            h1399
        }
    }))), {
        let a1402 = f430(Str::lit("nextCursor"), match v0.clone() {
            Adt35::C0(_, h1400, _) => {
                h1400
            }
        });
        let a1403 = f431(match v0 {
            Adt35::C0(h1401, _, _) => {
                h1401
            }
        })?;
        append_list(a1402, a1403)
    }))
}

pub fn f347(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1404 = v0;
    match m1404 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1405 = f366(Adt6::C5(v1))?;
            match m1405 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1406 = f260(v2);
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

pub fn f348(v0: Adt38) -> R<Option<List<(Str, Adt6)>>> {
    let m1409 = f433(match v0.clone() {
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
                let a1412 = f434(Str::lit("structuredContent"), match v0.clone() {
                    Adt38::C0(_, h1410, _) => {
                        h1410
                    }
                });
                let a1413 = f435(Str::lit("isError"), match v0 {
                    Adt38::C0(_, _, h1411) => {
                        h1411
                    }
                });
                append_list(a1412, a1413)
            })))
        }
    }
}

pub fn f349(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f436(match v0.clone() {
        Adt52::C0(_, _, h1414) => {
            h1414
        }
    })?)), {
        let a1417 = f430(Str::lit("nextCursor"), match v0.clone() {
            Adt52::C0(_, h1415, _) => {
                h1415
            }
        });
        let a1418 = f431(match v0 {
            Adt52::C0(h1416, _, _) => {
                h1416
            }
        })?;
        append_list(a1417, a1418)
    }))
}

pub fn f350(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f437(match v0.clone() {
        Adt53::C0(_, h1419) => {
            h1419
        }
    }))), f431(match v0 {
        Adt53::C0(h1420, _) => {
            h1420
        }
    })?))
}

pub fn f351(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f438(match v0.clone() {
        Adt54::C0(_, _, h1421) => {
            h1421
        }
    }))), {
        let a1424 = f430(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h1422, _) => {
                h1422
            }
        });
        let a1425 = f431(match v0 {
            Adt54::C0(h1423, _, _) => {
                h1423
            }
        })?;
        append_list(a1424, a1425)
    }))
}

pub fn f352(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f439(match v0.clone() {
        Adt56::C0(_, _, h1426) => {
            h1426
        }
    }))), {
        let a1429 = f430(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h1427, _) => {
                h1427
            }
        });
        let a1430 = f431(match v0 {
            Adt56::C0(h1428, _, _) => {
                h1428
            }
        })?;
        append_list(a1429, a1430)
    }))
}

pub fn f353(v0: Adt59) -> R<Option<List<(Str, Adt6)>>> {
    let m1432 = f440(match v0.clone() {
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
                let a1434 = f430(Str::lit("description"), match v0 {
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

pub fn f354(v0: Adt61) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f378({
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
        let a1445 = f441(Str::lit("total"), {
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
        let a1446 = f435(Str::lit("hasMore"), {
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

pub fn f355(v0: Adt82) -> Option<List<(Str, Adt6)>> {
    let m1447 = f442(f443(v0.clone()));
    match m1447 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1449 = {
                    let m1448 = f443(v0.clone());
                    match m1448.uncons() {
                        None => {
                            List::<(Str, Adt6)>::nil()
                        }
                        Some((_, _)) => {
                            List::cons((Str::lit("inputRequests"), v1), List::<(Str, Adt6)>::nil())
                        }
                    }
                };
                let a1450 = f444(Str::lit("requestState"), f445(v0));
                append_list(a1449, a1450)
            })
        }
    }
}

pub fn f356(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1451 = f446(v0.clone());
    match m1451 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1452 = f447(f448(v0.clone()), f449(f450(v0)));
                let a1453 = v1;
                append_list(a1452, a1453)
            })
        }
    }
}

pub fn f357(v0: Adt88) -> Option<List<(Str, Adt6)>> {
    let m1455 = match v0.clone() {
        Adt88::C0(_, h1454, _) => {
            h1454
        }
    };
    match m1455 {
        Adt89::C0 => {
            f356({
                let m1458 = match v0 {
                    Adt88::C0(h1457, _, _) => {
                        h1457
                    }
                };
                match m1458 {
                    Adt85::C0(h1456, _, _) => {
                        h1456
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
    let m1459 = v0;
    match m1459 {
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
    let m1460 = v0;
    match m1460 {
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
    let m1461 = v0;
    match m1461 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f451(), f452(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f361(v0: Bytes) -> R<Option<List<Adt85>>> {
    let a1463 = {
        let a1462 = v0.clone();
        length_bytes(a1462)
    };
    let a1464 = 0u64;
    if nat_eq(a1463, a1464) {
        Ok(Some(List::<Adt85>::nil()))
    } else {
        let m1465 = f68(v0)?;
        match m1465 {
            Ok(v2) => {
                let m1466 = f333(v2);
                match m1466 {
                    None => {
                        Ok(None::<List<Adt85>>)
                    }
                    Some(v3) => {
                        Ok(f453(v3, List::<Adt85>::nil()))
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
    let m1467 = f454(v0);
    match m1467 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1468 = f89(v1)?;
            match m1468 {
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
    Ok(List::cons(v0.clone(), f455(f456(f457(v0), v1, v2, List::<Adt85>::nil())?, List::<Adt85>::nil())))
}

pub fn f364(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f458(v2) {
            let a1469 = v1;
            let a1470 = 200u64;
            nat_eq(a1469, a1470)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f365(v0: u64, v1: List<List<u64>>) -> Adt94 {
    Adt94::C0(v0, f459(), List::<List<u64>>::nil(), f460(v1), None::<List<u64>>)
}

pub fn f366(v0: Adt6) -> R<Option<Adt6>> {
    f461(f462(), Adt150::C0(v0), List::<Adt151>::nil())
}

pub fn f367(v0: Adt130) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f463(match v0.clone() {
        Adt130::C0(h1471, _) => {
            h1471
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1474 = match v0.clone() {
            Adt130::C0(_, h1473) => {
                h1473
            }
        };
        match m1474 {
            Adt51::C0(h1472, _, _) => {
                h1472
            }
        }
    })), {
        let a1481 = f464({
            let m1477 = match v0.clone() {
                Adt130::C0(_, h1476) => {
                    h1476
                }
            };
            match m1477 {
                Adt51::C0(_, h1475, _) => {
                    h1475
                }
            }
        });
        let a1482 = f465({
            let m1480 = match v0 {
                Adt130::C0(_, h1479) => {
                    h1479
                }
            };
            match m1480 {
                Adt51::C0(_, _, h1478) => {
                    h1478
                }
            }
        });
        append_list(a1481, a1482)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f368(v0: Adt88) -> bool {
    if match v0.clone() {
        Adt88::C0(_, _, h1483) => {
            h1483
        }
    } {
        let m1485 = match v0 {
            Adt88::C0(_, h1484, _) => {
                h1484
            }
        };
        match m1485 {
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
    let m1486 = f356(v0);
    match m1486 {
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
    let m1487 = v0;
    match m1487 {
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
            f466()
        }
        Adt90::C7(_) => {
            f467()
        }
        Adt90::C8(_) => {
            f468()
        }
        Adt90::C9(_) => {
            f466()
        }
    }
}

pub fn f376(v0: Adt132) -> Str {
    let m1488 = v0;
    match m1488 {
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

pub fn f377(v0: Adt113) -> Str {
    let m1489 = v0;
    match m1489 {
        Adt113::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt113::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt113::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt113::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt113::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt113::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt113::C6(v1) => {
            let m1490 = v1;
            match m1490 {
                Adt114::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt114::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt114::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt113::C7(v2) => {
            let m1491 = v2;
            match m1491 {
                Adt114::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt114::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt114::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt113::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt113::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt113::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f378(v0: List<Str>) -> List<Adt6> {
    f310(f469(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f379(v0: Adt18) -> Adt6 {
    Adt6::C5({
        let a1503 = f470(Str::lit("experimental"), match v0.clone() {
            Adt18::C0(h1492, _, _, _, _) => {
                h1492
            }
        });
        let a1504 = {
            let a1501 = f470(Str::lit("roots"), match v0.clone() {
                Adt18::C0(_, h1493, _, _, _) => {
                    h1493
                }
            });
            let a1502 = {
                let a1499 = f470(Str::lit("sampling"), match v0.clone() {
                    Adt18::C0(_, _, h1494, _, _) => {
                        h1494
                    }
                });
                let a1500 = {
                    let a1497 = f470(Str::lit("elicitation"), match v0.clone() {
                        Adt18::C0(_, _, _, h1495, _) => {
                            h1495
                        }
                    });
                    let a1498 = f470(Str::lit("extensions"), match v0 {
                        Adt18::C0(_, _, _, _, h1496) => {
                            h1496
                        }
                    });
                    append_list(a1497, a1498)
                };
                append_list(a1499, a1500)
            };
            append_list(a1501, a1502)
        };
        append_list(a1503, a1504)
    })
}

pub fn f380(p1507: List<List<u64>>) -> R<bool> {
    let m1506 = p1507;
    {
        let mut m1506 = m1506;
        loop {
            let step = {
                let v0 = m1506;
                let m1505 = v0;
                match m1505.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f471(f142(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1506 = next; }
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
        f472(v0, v1, v2, f473(match v3.clone() {
            Adt139::C0(_, h1508, _, _) => {
                h1508
            }
        }, match v3.clone() {
            Adt139::C0(_, _, h1509, _) => {
                h1509
            }
        }, match v3 {
            Adt139::C0(_, _, _, h1510) => {
                h1510
            }
        }))
    } else {
        Ok(Err::<Adt128, Adt131>(Adt131::C3(f474(match v3 {
            Adt139::C0(_, h1511, _, _) => {
                h1511
            }
        }))))
    }
}

pub fn f383(v0: Adt4, v1: Adt140) -> bool {
    let m1512 = v1;
    match m1512 {
        Adt140::C0 => {
            true
        }
        Adt140::C1 => {
            f475(match v0 {
                Adt4::C0(_, _, _, _, _, h1513, _) => {
                    h1513
                }
            })
        }
        Adt140::C2 => {
            f475(match v0 {
                Adt4::C0(_, _, _, _, _, h1514, _) => {
                    h1514
                }
            })
        }
        Adt140::C3 => {
            f476(match v0 {
                Adt4::C0(_, _, _, _, h1515, _, _) => {
                    h1515
                }
            })
        }
        Adt140::C4 => {
            f476(match v0 {
                Adt4::C0(_, _, _, _, h1516, _, _) => {
                    h1516
                }
            })
        }
        Adt140::C5 => {
            f476(match v0 {
                Adt4::C0(_, _, _, _, h1517, _, _) => {
                    h1517
                }
            })
        }
        Adt140::C6 => {
            f475(match v0 {
                Adt4::C0(_, _, _, h1518, _, _, _) => {
                    h1518
                }
            })
        }
        Adt140::C7 => {
            f475(match v0 {
                Adt4::C0(_, _, _, h1519, _, _, _) => {
                    h1519
                }
            })
        }
        Adt140::C8 => {
            f477(match v0 {
                Adt4::C0(_, _, h1520, _, _, _, _) => {
                    h1520
                }
            })
        }
        Adt140::C9 => {
            f478(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1521) => {
                    h1521
                }
            }, f26())
        }
        Adt140::C10 => {
            f478(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1522) => {
                    h1522
                }
            }, f26())
        }
        Adt140::C11 => {
            f478(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1523) => {
                    h1523
                }
            }, f26())
        }
    }
}

pub fn f384(v0: Adt144, v1: Adt142) -> Result<Adt139, Adt131> {
    let m1524 = v1;
    match m1524 {
        Adt142::C0 => {
            Err::<Adt139, Adt131>(Adt131::C3(match v0 {
                Adt144::C0(_, h1525, _, _) => {
                    h1525
                }
            }))
        }
        Adt142::C1(v2) => {
            Ok::<Adt139, Adt131>(Adt139::C0(match v0.clone() {
                Adt144::C0(h1526, _, _, _) => {
                    h1526
                }
            }, v2, match v0.clone() {
                Adt144::C0(_, _, h1527, _) => {
                    h1527
                }
            }, match v0 {
                Adt144::C0(_, _, _, h1528) => {
                    h1528
                }
            }))
        }
    }
}

pub fn f385(v0: Option<Adt112>, v1: List<Str>, v2: Adt104, v3: Result<Adt17, Adt132>) -> R<Adt143> {
    let m1529 = v3;
    match m1529 {
        Ok(v5) => {
            Ok(f479(v1, v2.clone(), v5.clone(), f480(v0, f296(match v2.clone() {
                Adt104::C0(_, h1530, _) => {
                    h1530
                }
            }), match v2.clone() {
                Adt104::C0(_, h1531, _) => {
                    h1531
                }
            }, match v5 {
                Adt17::C0(h1532, _, _, _, _, _) => {
                    h1532
                }
            }, f481(match v2 {
                Adt104::C0(_, _, h1533) => {
                    h1533
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt143::C1(Adt131::C0(v4)))
        }
    }
}

pub fn f386(v0: Option<Adt5>) -> Result<Adt17, Adt132> {
    let m1534 = v0;
    match m1534 {
        None => {
            Err::<Adt17, Adt132>(Adt132::C0)
        }
        Some(v1) => {
            let m1535 = f180(Str::lit("_meta"), v1);
            match m1535 {
                Adt135::C0 => {
                    Err::<Adt17, Adt132>(Adt132::C1)
                }
                Adt135::C1(v2) => {
                    let m1536 = f260(v2);
                    match m1536 {
                        None => {
                            Err::<Adt17, Adt132>(Adt132::C4)
                        }
                        Some(v3) => {
                            f482(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f387(v0: Adt140) -> Adt145 {
    let m1537 = v0;
    match m1537 {
        Adt140::C0 => {
            Adt145::C0
        }
        Adt140::C1 => {
            Adt145::C0
        }
        Adt140::C2 => {
            Adt145::C1
        }
        Adt140::C3 => {
            Adt145::C0
        }
        Adt140::C4 => {
            Adt145::C2
        }
        Adt140::C5 => {
            Adt145::C0
        }
        Adt140::C6 => {
            Adt145::C0
        }
        Adt140::C7 => {
            Adt145::C1
        }
        Adt140::C8 => {
            Adt145::C0
        }
        Adt140::C9 => {
            Adt145::C3
        }
        Adt140::C10 => {
            Adt145::C3
        }
        Adt140::C11 => {
            Adt145::C3
        }
    }
}

pub fn f388(p1543: List<u64>, p1544: List<u64>) -> Option<List<u64>> {
    let m1542 = (p1543, p1544);
    {
        let mut m1542 = m1542;
        loop {
            let step = {
                let (v0, v1) = m1542;
                let m1538 = v0;
                match m1538.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1539 = v1;
                        match m1539.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1540 = v4;
                                let a1541 = v2;
                                if nat_eq(a1540, a1541) {
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
                Ok(next) => { m1542 = next; }
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

pub fn f391(p1553: u64, p1554: List<u64>, p1555: List<u64>) -> R<Option<List<u64>>> {
    let m1552 = (p1553, (p1554, p1555));
    {
        let mut m1552 = m1552;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1552;
                let m1545 = v0;
                if m1545 == 0 {
                    let m1546 = v1;
                    match m1546.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1545 - 1;
                    let m1547 = v1;
                    match m1547.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1548 = v7;
                            match m1548.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1549 = v9;
                                    match m1549.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1550 = v11;
                                            match m1550.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1551 = f483(v6, v8, v10, v12, f484(v13.clone()))?;
                                                    match m1551 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f167(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: Adt12, v1: Option<Str>) -> bool {
    let m1556 = v1;
    match m1556 {
        None => {
            false
        }
        Some(v2) => {
            let a1558 = match v0 {
                Adt12::C0(h1557, _, _, _) => {
                    h1557
                }
            };
            let a1559 = v2;
            equal(a1558, a1559)
        }
    }
}

pub fn f393(v0: List<Adt92>, v1: Str) -> R<Adt116> {
    let m1562 = f229(f70(v0, {
        let a1560 = f485();
        let a1561 = f486(v1)?;
        append_list(a1560, a1561)
    })?)?;
    match m1562 {
        Adt102::C0 => {
            Ok(Adt116::C0)
        }
        Adt102::C1 => {
            Ok(Adt116::C1)
        }
        Adt102::C2 => {
            Ok(Adt116::C1)
        }
        Adt102::C3(v2) => {
            let m1563 = f206(v2);
            match m1563 {
                None => {
                    Ok(Adt116::C1)
                }
                Some(v3) => {
                    Ok(Adt116::C2(v3))
                }
            }
        }
    }
}

pub fn f394(v0: i64) -> bool {
    let a1570 = {
        let m1566 = {
            let a1564 = v0;
            let a1565 = 0i64;
            compare(a1564, a1565)
        };
        if match m1566 {
            Ordering::Less => {
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
            let m1569 = {
                let a1567 = 0i64;
                let a1568 = v0;
                compare(a1567, a1568)
            };
            match m1569 {
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
    bool_not(a1570)
}

pub fn f395(v0: Adt119, v1: u64, v2: Str) -> Adt136 {
    let m1572 = match v0.clone() {
        Adt119::C0(_, h1571, _) => {
            h1571
        }
    };
    match m1572.uncons() {
        None => {
            f250(Adt99::C2(v1))
        }
        Some((v3, v4)) => {
            let m1573 = v3;
            match m1573 {
                Adt124::C0(_) => {
                    f250(Adt99::C2(v1))
                }
                Adt124::C1(v6, _) => {
                    Adt136::C1(Adt119::C0(Adt120::C5, List::cons(Adt124::C1(v6, Some(v2)), v4), match v0 {
                        Adt119::C0(_, _, h1574) => {
                            h1574
                        }
                    }))
                }
            }
        }
    }
}

pub fn f396(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1575 = v0;
    let a1576 = 128u64;
    if nat_lt(a1575, a1576) {
        Ok(List::cons(v0, v1))
    } else {
        let a1577 = v0;
        let a1578 = 2048u64;
        if nat_lt(a1577, a1578) {
            Ok(List::cons({
                let a1579 = 128u64;
                let a1580 = f487(v0, 64u64);
                nat_add(a1579, a1580)?
            }, List::cons({
                let a1581 = 192u64;
                let a1582 = f488(v0, 64u64);
                nat_add(a1581, a1582)?
            }, v1)))
        } else {
            let a1583 = v0;
            let a1584 = 65536u64;
            if nat_lt(a1583, a1584) {
                Ok(List::cons({
                    let a1585 = 128u64;
                    let a1586 = f487(v0, 64u64);
                    nat_add(a1585, a1586)?
                }, List::cons({
                    let a1587 = 128u64;
                    let a1588 = f487(f488(v0, 64u64), 64u64);
                    nat_add(a1587, a1588)?
                }, List::cons({
                    let a1589 = 224u64;
                    let a1590 = f488(v0, 4096u64);
                    nat_add(a1589, a1590)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1591 = 128u64;
                    let a1592 = f487(v0, 64u64);
                    nat_add(a1591, a1592)?
                }, List::cons({
                    let a1593 = 128u64;
                    let a1594 = f487(f488(v0, 64u64), 64u64);
                    nat_add(a1593, a1594)?
                }, List::cons({
                    let a1595 = 128u64;
                    let a1596 = f487(f488(v0, 4096u64), 64u64);
                    nat_add(a1595, a1596)?
                }, List::cons({
                    let a1597 = 240u64;
                    let a1598 = f488(v0, 262144u64);
                    nat_add(a1597, a1598)?
                }, v1)))))
            }
        }
    }
}

pub fn f397() -> u64 {
    18u64
}

pub fn f398(p1602: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1601 = p1602;
    {
        let mut m1601 = m1601;
        loop {
            let step = {
                let v0 = m1601;
                let m1599 = v0;
                match m1599.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1600 = f237(v2.clone(), v1.clone());
                        match m1600 {
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
                Ok(next) => { m1601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f400(p1609: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1608 = p1609;
    {
        let mut m1608 = m1608;
        loop {
            let step = {
                let v0 = m1608;
                let m1603 = v0;
                match m1603.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1605 = f237({
                            let (h1604, _) = v2.clone();
                            h1604
                        }, v1.clone());
                        match m1605 {
                            Adt135::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt135::C1(v4) => {
                                if f489({
                                    let (_, h1606) = v2;
                                    h1606
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1607, _) = v2;
                                        h1607
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f402(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1610 = f237(Str::lit("$ref"), v2);
    match m1610 {
        Adt135::C0 => {
            None::<Adt97>
        }
        Adt135::C1(v3) => {
            let m1611 = f234(v3);
            match m1611 {
                None => {
                    None::<Adt97>
                }
                Some(v4) => {
                    let m1612 = f490(v1, v4.clone());
                    match m1612 {
                        None => {
                            Some(Adt97::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt97>
                        }
                    }
                }
            }
        }
    }
}

pub fn f403(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt125) -> R<Adt147> {
    if v3 {
        let m1613 = f234(v2);
        match m1613 {
            None => {
                Ok(Adt147::C0(Adt97::C7(v0)))
            }
            Some(v6) => {
                f491(v0, v1, v6.clone(), f113(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt147::C0(Adt97::C10(v0)))
    }
}

pub fn f404(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt105>) -> List<Adt105> {
    let m1614 = f237(v0.clone(), v1);
    match m1614 {
        Adt135::C0 => {
            v4
        }
        Adt135::C1(v5) => {
            let m1615 = f260(v5);
            match m1615 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1616 = v0;
                    let a1617 = Str::lit("properties");
                    if equal(a1616, a1617) {
                        f492(match v6 {
                            Adt5::C0(h1618) => {
                                h1618
                            }
                        }, v2, v3, v4)
                    } else {
                        f493(match v6 {
                            Adt5::C0(h1619) => {
                                h1619
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f405(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1620 = f237(v0, v1);
    match m1620 {
        Adt135::C0 => {
            v2
        }
        Adt135::C1(v3) => {
            List::cons(Adt105::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f406(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1621 = f237(v0, v1);
    match m1621 {
        Adt135::C0 => {
            v2
        }
        Adt135::C1(v3) => {
            let m1622 = f333(v3);
            match m1622 {
                None => {
                    v2
                }
                Some(v4) => {
                    f494(v4, v2)
                }
            }
        }
    }
}

pub fn f407(p1629: List<(Str, Adt148)>) -> Adt137 {
    let m1628 = p1629;
    {
        let mut m1628 = m1628;
        loop {
            let step = {
                let v0 = m1628;
                let m1623 = v0;
                match m1623.uncons() {
                    None => {
                        Err::<List<(Str, Adt148)>, Adt137>(Adt137::C3(List::<Adt109>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1625 = {
                            let (_, h1624) = v1;
                            h1624
                        };
                        match m1625 {
                            Adt148::C0 => {
                                Ok::<List<(Str, Adt148)>, Adt137>(v2)
                            }
                            Adt148::C1 => {
                                Err::<List<(Str, Adt148)>, Adt137>(Adt137::C0({
                                    let (h1626, _) = v1;
                                    h1626
                                }))
                            }
                            Adt148::C2 => {
                                Err::<List<(Str, Adt148)>, Adt137>(Adt137::C1({
                                    let (h1627, _) = v1;
                                    h1627
                                }))
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

pub fn f408(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt148> {
    let m1630 = f237(Str::lit("type"), v0);
    match m1630 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v2) => {
            let m1631 = f234(v2.clone());
            match m1631 {
                None => {
                    let m1632 = f333(v2);
                    match m1632 {
                        None => {
                            Ok(Adt148::C2)
                        }
                        Some(v4) => {
                            f497(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f495(v3.clone()) {
                        if f496(v3, v1)? {
                            Ok(Adt148::C0)
                        } else {
                            Ok(Adt148::C1)
                        }
                    } else {
                        Ok(Adt148::C2)
                    }
                }
            }
        }
    }
}

pub fn f409(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt148> {
    let m1633 = f237(Str::lit("enum"), v0);
    match m1633 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v2) => {
            let m1634 = f333(v2);
            match m1634 {
                None => {
                    Ok(Adt148::C2)
                }
                Some(v3) => {
                    f498(v3, v1)
                }
            }
        }
    }
}

pub fn f410(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt148> {
    let m1635 = f237(Str::lit("const"), v0);
    match m1635 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v2) => {
            let m1636 = f499(v2, v1)?;
            match m1636 {
                Adt152::C0 => {
                    Ok(Adt148::C0)
                }
                Adt152::C1 => {
                    Ok(Adt148::C1)
                }
                Adt152::C2 => {
                    Ok(Adt148::C2)
                }
            }
        }
    }
}

pub fn f411(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt148> {
    let m1637 = f237(v0, v2);
    match m1637 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v4) => {
            let m1638 = f500(v4);
            match m1638 {
                None => {
                    Ok(Adt148::C2)
                }
                Some(v5) => {
                    let m1639 = f500(v3);
                    match m1639 {
                        None => {
                            Ok(Adt148::C0)
                        }
                        Some(v6) => {
                            if f501(v1, f502(v6, v5)?) {
                                Ok(Adt148::C0)
                            } else {
                                Ok(Adt148::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f412(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt148)>> {
    let m1640 = v1;
    match m1640 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt148)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt148)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt148)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f503(Str::lit("minLength"), true, v0.clone(), f504(v4.clone())?)?), List::cons((Str::lit("maxLength"), f503(Str::lit("maxLength"), false, v0, f504(v4)?)?), List::<(Str, Adt148)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f503(Str::lit("minItems"), true, v0.clone(), f505(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f503(Str::lit("maxItems"), false, v0, f505(v5, 0u64)?)?), List::<(Str, Adt148)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f503(Str::lit("minProperties"), true, v0.clone(), f506(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f503(Str::lit("maxProperties"), false, v0, f506(v6, 0u64)?)?), List::<(Str, Adt148)>::nil())))
        }
    }
}

pub fn f413(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt148 {
    let m1641 = f237(Str::lit("required"), v0);
    match m1641 {
        Adt135::C0 => {
            Adt148::C0
        }
        Adt135::C1(v2) => {
            let m1642 = f333(v2);
            match m1642 {
                None => {
                    Adt148::C2
                }
                Some(v3) => {
                    let m1643 = f260(v1);
                    match m1643 {
                        None => {
                            Adt148::C0
                        }
                        Some(v4) => {
                            f507(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f414(v0: Adt137, v1: Adt137) -> Adt137 {
    let m1644 = v0.clone();
    match m1644 {
        Adt137::C0(_) => {
            v0
        }
        Adt137::C1(_) => {
            v0
        }
        Adt137::C2(_) => {
            v0
        }
        Adt137::C3(v5) => {
            let m1645 = v1.clone();
            match m1645 {
                Adt137::C0(_) => {
                    v1
                }
                Adt137::C1(_) => {
                    v1
                }
                Adt137::C2(_) => {
                    v1
                }
                Adt137::C3(v9) => {
                    Adt137::C3({
                        let a1646 = v5;
                        let a1647 = v9;
                        append_list(a1646, a1647)
                    })
                }
            }
        }
    }
}

pub fn f415(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> Adt137 {
    let m1648 = f237(Str::lit("$ref"), v1);
    match m1648 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v4) => {
            let m1649 = f234(v4);
            match m1649 {
                None => {
                    Adt137::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1650 = f490(v0, v5.clone());
                    match m1650 {
                        None => {
                            Adt137::C2(v5)
                        }
                        Some(v6) => {
                            Adt137::C3(List::cons(Adt109::C0(v6, v2, v3), List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f416(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1651 = f237(Str::lit("properties"), v0);
    match m1651 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1652 = f260(v3);
            match m1652 {
                None => {
                    Adt137::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1653 = f260(v1);
                    match m1653 {
                        None => {
                            Adt137::C3(List::<Adt109>::nil())
                        }
                        Some(v5) => {
                            Adt137::C3(f508(match v4 {
                                Adt5::C0(h1654) => {
                                    h1654
                                }
                            }, match v5 {
                                Adt5::C0(h1655) => {
                                    h1655
                                }
                            }, v2, List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f417(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1656 = f237(Str::lit("additionalProperties"), v0.clone());
    match m1656 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1657 = f260(v1);
            match m1657 {
                None => {
                    Adt137::C3(List::<Adt109>::nil())
                }
                Some(v4) => {
                    let m1658 = f259(v3.clone());
                    match m1658 {
                        None => {
                            Adt137::C3(f511(match v4 {
                                Adt5::C0(h1660) => {
                                    h1660
                                }
                            }, f510(v0), v3, v2, List::<Adt109>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt137::C3(List::<Adt109>::nil())
                            } else {
                                if f509(match v4 {
                                    Adt5::C0(h1659) => {
                                        h1659
                                    }
                                }, f510(v0)) {
                                    Adt137::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt137::C3(List::<Adt109>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f418(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> R<Adt137> {
    let m1661 = f237(Str::lit("items"), v0);
    match m1661 {
        Adt135::C0 => {
            Ok(Adt137::C3(List::<Adt109>::nil()))
        }
        Adt135::C1(v3) => {
            let m1662 = f333(v1);
            match m1662 {
                None => {
                    Ok(Adt137::C3(List::<Adt109>::nil()))
                }
                Some(v4) => {
                    Ok(Adt137::C3(f512(v4, 0u64, v3, v2, List::<Adt109>::nil())?))
                }
            }
        }
    }
}

pub fn f419(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1663 = f237(Str::lit("allOf"), v0);
    match m1663 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1664 = f333(v3);
            match m1664 {
                None => {
                    Adt137::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1665 = v4.clone();
                    match m1665.uncons() {
                        None => {
                            Adt137::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt137::C3(f513(v4, v1, v2, List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f420(v0: Adt110, v1: Adt137, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt107>) -> Adt137 {
    let m1666 = v1.clone();
    match m1666 {
        Adt137::C0(_) => {
            v1
        }
        Adt137::C1(_) => {
            v1
        }
        Adt137::C2(_) => {
            v1
        }
        Adt137::C3(v8) => {
            let m1667 = v8;
            match m1667.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt137::C3(List::cons(Adt109::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt109>::nil()))
                }
            }
        }
    }
}

pub fn f421(p1672: List<Adt51>, p1673: Adt7) -> R<bool> {
    let m1671 = (p1672, p1673);
    {
        let mut m1671 = m1671;
        loop {
            let step = {
                let (v0, v1) = m1671;
                let m1668 = v0;
                match m1668.uncons() {
                    None => {
                        Err::<(List<Adt51>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f514(f502(match v2.clone() {
                            Adt51::C0(h1669, _, _) => {
                                h1669
                            }
                        }, v1)?) {
                            Ok::<(List<Adt51>, Adt7), R<bool>>((v3, match v2 {
                                Adt51::C0(h1670, _, _) => {
                                    h1670
                                }
                            }))
                        } else {
                            Err::<(List<Adt51>, Adt7), R<bool>>(Ok(false))
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

pub fn f422(p1680: List<u64>, p1681: List<u64>) -> List<u64> {
    let m1679 = (p1680, p1681);
    {
        let mut m1679 = m1679;
        loop {
            let step = {
                let (v0, v1) = m1679;
                let m1674 = v0;
                match m1674.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1675 = v2;
                        let a1676 = 126u64;
                        if nat_eq(a1675, a1676) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1677 = v2;
                            let a1678 = 47u64;
                            if nat_eq(a1677, a1678) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(p1690: Bytes, p1691: List<Bytes>) -> List<Bytes> {
    let m1689 = (p1690, p1691);
    {
        let mut m1689 = m1689;
        loop {
            let step = {
                let (v0, v1) = m1689;
                let m1682 = v1;
                match m1682.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1685 = {
                            let a1683 = v2.clone();
                            length_bytes(a1683)
                        };
                        let a1686 = {
                            let a1684 = v0.clone();
                            length_bytes(a1684)
                        };
                        if nat_eq(a1685, a1686) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1687 = v2;
                                let a1688 = v0;
                                append_bytes(a1687, a1688)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424(v0: u64) -> Bytes {
    let m1695 = {
        let a1692 = f515();
        let a1693 = v0;
        let a1694 = 1u64;
        slice_bytes(a1692, a1693, a1694)
    };
    match m1695 {
        None => {
            f87()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f425() -> i64 {
    999999999999999999i64
}

pub fn f426(v0: i64) -> R<List<u64>> {
    let m1698 = {
        let a1696 = v0;
        let a1697 = 0i64;
        compare(a1696, a1697)
    };
    if match m1698 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f516(20u64, {
            let a1699 = v0;
            int_neg(a1699)?
        }, List::<u64>::nil())?))
    } else {
        f516(20u64, v0, List::<u64>::nil())
    }
}

pub fn f427(v0: u64) -> R<List<u64>> {
    let a1700 = v0;
    let a1701 = 34u64;
    if nat_eq(a1700, a1701) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1702 = v0;
        let a1703 = 92u64;
        if nat_eq(a1702, a1703) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1704 = v0;
            let a1705 = 8u64;
            if nat_eq(a1704, a1705) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1706 = v0;
                let a1707 = 9u64;
                if nat_eq(a1706, a1707) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1708 = v0;
                    let a1709 = 10u64;
                    if nat_eq(a1708, a1709) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1710 = v0;
                        let a1711 = 12u64;
                        if nat_eq(a1710, a1711) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1712 = v0;
                            let a1713 = 13u64;
                            if nat_eq(a1712, a1713) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1714 = v0;
                                let a1715 = 32u64;
                                if nat_lt(a1714, a1715) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f517(f488(v0, 16u64))?, List::cons(f517(f487(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f428(v0: List<Str>) -> Adt6 {
    Adt6::C4(f378(v0))
}

pub fn f429(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1733 = f470(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1716, _, _, _, _, _, _) => {
                h1716
            }
        });
        let a1734 = {
            let a1731 = f470(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1717, _, _, _, _, _) => {
                    h1717
                }
            });
            let a1732 = {
                let a1729 = f470(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1718, _, _, _, _) => {
                        h1718
                    }
                });
                let a1730 = {
                    let a1727 = f518(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1719, _, _, _) => {
                            h1719
                        }
                    });
                    let a1728 = {
                        let a1725 = f519(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1720, _, _) => {
                                h1720
                            }
                        });
                        let a1726 = {
                            let a1723 = f518(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1721, _) => {
                                    h1721
                                }
                            });
                            let a1724 = f470(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1722) => {
                                    h1722
                                }
                            });
                            append_list(a1723, a1724)
                        };
                        append_list(a1725, a1726)
                    };
                    append_list(a1727, a1728)
                };
                append_list(a1729, a1730)
            };
            append_list(a1731, a1732)
        };
        append_list(a1733, a1734)
    })
}

pub fn f430(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1735 = v1;
    match m1735 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f431(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f520(match v0.clone() {
        Adt10::C0(h1736, _) => {
            h1736
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f521(match v0 {
        Adt10::C0(_, h1737) => {
            h1737
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f432(v0: List<Adt36>) -> List<Adt6> {
    f310(f522(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f433(v0: List<Adt39>) -> R<Option<List<Adt6>>> {
    let m1738 = f523(v0, List::<Adt6>::nil())?;
    match m1738 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f310(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f434(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1739 = v1;
    match m1739 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f435(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1740 = v1;
    match m1740 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f436(v0: List<Adt45>) -> R<List<Adt6>> {
    Ok(f310(f524(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f437(v0: List<Adt47>) -> List<Adt6> {
    f310(f525(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f438(v0: List<Adt55>) -> List<Adt6> {
    f310(f526(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f439(v0: List<Adt57>) -> List<Adt6> {
    f310(f527(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f440(v0: List<Adt60>) -> R<Option<List<Adt6>>> {
    let m1741 = f528(v0, List::<Adt6>::nil())?;
    match m1741 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f310(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f441(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1742 = v1;
    match m1742 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f520(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f442(v0: List<(Str, Adt65)>) -> Option<Adt6> {
    let m1743 = f529(v0, List::<(Str, Adt6)>::nil());
    match m1743 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f312(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f443(v0: Adt82) -> List<(Str, Adt65)> {
    let m1744 = v0;
    match m1744 {
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

pub fn f444(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1745 = v1;
    match m1745 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f445(v0: Adt82) -> Option<Str> {
    let m1746 = v0;
    match m1746 {
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

pub fn f446(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1747 = v0;
    match m1747 {
        Adt63::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt63::C1(_, v3) => {
            let m1748 = f442(v3);
            match m1748 {
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
                Adt5::C0(h1749) => {
                    h1749
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

pub fn f447(v0: Adt64, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt64::C0(h1750, _, _, _, _, _) => {
            h1750
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a1756 = f444(Str::lit("statusMessage"), match v0.clone() {
            Adt64::C0(_, h1751, _, _, _, _) => {
                h1751
            }
        });
        let a1757 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, h1752, _, _, _) => {
                h1752
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, _, h1753, _, _) => {
                h1753
            }
        })), List::cons((Str::lit("ttlMs"), f530(match v0.clone() {
            Adt64::C0(_, _, _, _, h1754, _) => {
                h1754
            }
        })), f531(Str::lit("pollIntervalMs"), match v0 {
            Adt64::C0(_, _, _, _, _, h1755) => {
                h1755
            }
        }))));
        append_list(a1756, a1757)
    }))
}

pub fn f448(v0: Adt63) -> Adt64 {
    let m1758 = v0;
    match m1758 {
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

pub fn f449(v0: Adt84) -> Str {
    let m1759 = v0;
    match m1759 {
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

pub fn f450(v0: Adt63) -> Adt84 {
    let m1760 = v0;
    match m1760 {
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

pub fn f451() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f452(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1761, _, _, _, _, _) => {
            h1761
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1762, _, _, _) => {
            h1762
        }
    })), {
        let a1771 = f430(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1763, _, _, _, _) => {
                h1763
            }
        });
        let a1772 = {
            let a1769 = f430(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1764, _, _) => {
                    h1764
                }
            });
            let a1770 = {
                let a1767 = f430(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1765, _) => {
                        h1765
                    }
                });
                let a1768 = f532(match v0 {
                    Adt1::C0(_, _, _, _, _, h1766) => {
                        h1766
                    }
                });
                append_list(a1767, a1768)
            };
            append_list(a1769, a1770)
        };
        append_list(a1771, a1772)
    })))
}

pub fn f453(p1776: List<Adt6>, p1777: List<Adt85>) -> Option<List<Adt85>> {
    let m1775 = (p1776, p1777);
    {
        let mut m1775 = m1775;
        loop {
            let step = {
                let (v0, v1) = m1775;
                let m1773 = v0;
                match m1773.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(Some(f455(v1, List::<Adt85>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1774 = f533(v2);
                        match m1774 {
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
                Ok(next) => { m1775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: List<Adt85>) -> Option<Adt6> {
    let m1778 = f534(v0, List::<Adt6>::nil());
    match m1778 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f310(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f455(p1781: List<Adt85>, p1782: List<Adt85>) -> List<Adt85> {
    let m1780 = (p1781, p1782);
    {
        let mut m1780 = m1780;
        loop {
            let step = {
                let (v0, v1) = m1780;
                let m1779 = v0;
                match m1779.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), List<Adt85>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt85>, List<Adt85>), List<Adt85>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: Str, p1787: List<Adt85>, v2: i64, p1788: List<Adt85>) -> R<List<Adt85>> {
    let m1786 = (p1787, p1788);
    {
        let mut m1786 = m1786;
        loop {
            let step = {
                let (v1, v3) = m1786;
                let m1783 = v1;
                match m1783.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a1784 = f457(v4.clone());
                        let a1785 = v0.clone();
                        if if equal(a1784, a1785) {
                            true
                        } else {
                            f535(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1786 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: Adt85) -> Str {
    let m1791 = f448(match v0 {
        Adt85::C0(h1790, _, _) => {
            h1790
        }
    });
    match m1791 {
        Adt64::C0(h1789, _, _, _, _, _) => {
            h1789
        }
    }
}

pub fn f458(v0: List<List<u64>>) -> bool {
    let m1792 = v0;
    match m1792.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f459() -> List<Adt92> {
    List::cons(Adt92::C0(f536(), f381()), List::cons(Adt92::C0(f537(), f538()), List::cons(Adt92::C0(f539(), f540()), List::<Adt92>::nil())))
}

pub fn f460(v0: List<List<u64>>) -> List<u64> {
    f541(f37(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f461(p1807: u64, p1808: Adt150, p1809: List<Adt151>) -> R<Option<Adt6>> {
    let m1806 = (p1807, (p1808, p1809));
    {
        let mut m1806 = m1806;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1806;
                let m1793 = v0;
                if m1793 == 0 {
                    Err::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1793 - 1;
                    let m1794 = v1;
                    match m1794 {
                        Adt150::C0(v4) => {
                            let m1795 = v4.clone();
                            match m1795 {
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
                                    let v8: Str = f542(v7)?;
                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1796 = v9;
                                    match m1796.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0(v10), List::cons(Adt151::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1797 = v12;
                                    match m1797.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f542({
                                                let (h1798, _) = v13;
                                                h1798
                                            })?;
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0({
                                                let (_, h1799) = v13;
                                                h1799
                                            }), List::cons(Adt151::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt150::C1(v16) => {
                            let m1800 = v2;
                            match m1800.uncons() {
                                None => {
                                    Err::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1801 = v17;
                                    match m1801 {
                                        Adt151::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1802 = v20;
                                            match m1802.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C4(f310(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0(v22), List::cons(Adt151::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt151::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1803 = v26;
                                            match m1803.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C5(f312(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f542({
                                                        let (h1804, _) = v28;
                                                        h1804
                                                    })?;
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0({
                                                        let (_, h1805) = v28;
                                                        h1805
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
                Ok(next) => { m1806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462() -> u64 {
    67108864u64
}

pub fn f463(v0: Adt20) -> Adt6 {
    let m1810 = v0;
    match m1810 {
        Adt20::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt20::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f464(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1811 = v0;
    match m1811 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f465(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1812 = v0;
    match m1812 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f466() -> i64 {
    -32603i64
}

pub fn f467() -> i64 {
    429i64
}

pub fn f468() -> i64 {
    -32021i64
}

pub fn f469(p1815: List<Str>, p1816: List<Adt6>) -> List<Adt6> {
    let m1814 = (p1815, p1816);
    {
        let mut m1814 = m1814;
        loop {
            let step = {
                let (v0, v1) = m1814;
                let m1813 = v0;
                match m1813.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f470(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1817 = v1;
    match m1817 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1818) => {
                    h1818
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f471(v0: List<u64>) -> R<bool> {
    let m1819 = v0;
    match m1819.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1820 = v2;
            match m1820.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1821 = f188(v1)?;
                    let a1822 = 113u64;
                    if if nat_eq(a1821, a1822) {
                        let a1823 = v3;
                        let a1824 = 61u64;
                        nat_eq(a1823, a1824)
                    } else {
                        false
                    } {
                        Ok(f543(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f472(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt15, Adt132>) -> R<Result<Adt128, Adt131>> {
    let m1825 = v3;
    match m1825 {
        Ok(v5) => {
            f544(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(Adt131::C0(v4)))
        }
    }
}

pub fn f473(v0: Adt140, v1: Adt17, v2: Adt5) -> Result<Adt15, Adt132> {
    let m1826 = v0;
    match m1826 {
        Adt140::C0 => {
            f545(v1, v2)
        }
        Adt140::C1 => {
            f546(v1, v2)
        }
        Adt140::C2 => {
            f547(v1, v2)
        }
        Adt140::C3 => {
            f548(v1, v2)
        }
        Adt140::C4 => {
            f549(v1, v2)
        }
        Adt140::C5 => {
            f550(v1, v2)
        }
        Adt140::C6 => {
            f551(v1, v2)
        }
        Adt140::C7 => {
            f552(v1, v2)
        }
        Adt140::C8 => {
            f553(v1, v2)
        }
        Adt140::C9 => {
            f554(v1, v2)
        }
        Adt140::C10 => {
            f555(v1, v2)
        }
        Adt140::C11 => {
            f556(v1, v2)
        }
    }
}

pub fn f474(v0: Adt140) -> Str {
    let m1827 = v0;
    match m1827 {
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

pub fn f475(v0: Option<Adt8>) -> bool {
    let m1828 = v0;
    match m1828 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f476(v0: Option<Adt9>) -> bool {
    let m1829 = v0;
    match m1829 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f477(v0: Option<Adt5>) -> bool {
    let m1830 = v0;
    match m1830 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f478(v0: Option<Adt5>, v1: Str) -> bool {
    let m1831 = v0;
    match m1831 {
        None => {
            false
        }
        Some(v2) => {
            f238(v1, v2)
        }
    }
}

pub fn f479(v0: List<Str>, v1: Adt104, v2: Adt17, v3: Option<Adt113>) -> Adt143 {
    let m1832 = v3;
    match m1832 {
        None => {
            f557(v1, v2.clone(), f558(match v2 {
                Adt17::C0(h1833, _, _, _, _, _) => {
                    h1833
                }
            }, v0))
        }
        Some(v4) => {
            Adt143::C1(Adt131::C2(v4))
        }
    }
}

pub fn f480(v0: Option<Adt112>, v1: Adt145, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt113>> {
    let m1834 = v0;
    match m1834 {
        None => {
            Ok(None::<Adt113>)
        }
        Some(v5) => {
            let m1836 = match v5.clone() {
                Adt112::C0(_, _, _, h1835, _) => {
                    h1835
                }
            };
            match m1836 {
                None => {
                    f559(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f481(v0: Option<Adt5>) -> Adt5 {
    let m1837 = v0;
    match m1837 {
        None => {
            f560()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f482(v0: Adt5) -> Result<Adt17, Adt132> {
    let m1838 = f561(v0.clone());
    match m1838 {
        Adt154::C0 => {
            f562(v0.clone(), f180(f563(), v0.clone()), f180(f564(), v0))
        }
        Adt154::C1 => {
            Err::<Adt17, Adt132>(Adt132::C2)
        }
        Adt154::C2 => {
            Err::<Adt17, Adt132>(Adt132::C3)
        }
    }
}

pub fn f483(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1839 = f565(v0)?;
    match m1839 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1840 = f565(v1)?;
            match m1840 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1841 = v2;
                    let a1842 = 61u64;
                    if nat_eq(a1841, a1842) {
                        f566(v5, v6, v4, v3)
                    } else {
                        let m1843 = f565(v2)?;
                        match m1843 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1844 = v3;
                                let a1845 = 61u64;
                                if nat_eq(a1844, a1845) {
                                    f567(v5, v6, v7, v4)
                                } else {
                                    let m1846 = f565(v3)?;
                                    match m1846 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f568(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f484(v0: List<u64>) -> bool {
    let m1847 = v0;
    match m1847.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f485() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f486(v0: Str) -> R<List<u64>> {
    f85({
        let a1848 = v0;
        utf8_encode(a1848)
    })
}

pub fn f487(v0: u64, v1: u64) -> u64 {
    let a1849 = v0;
    let a1850 = v1;
    let a1851 = 0u64;
    nat_rem(a1849, a1850, a1851)
}

pub fn f488(v0: u64, v1: u64) -> u64 {
    let a1852 = v0;
    let a1853 = v1;
    let a1854 = 0u64;
    nat_quot(a1852, a1853, a1854)
}

pub fn f489(v0: u64, v1: Adt6) -> bool {
    let a1855 = v0;
    let a1856 = 0u64;
    if nat_eq(a1855, a1856) {
        f569(v1)
    } else {
        let a1857 = v0;
        let a1858 = 1u64;
        if nat_eq(a1857, a1858) {
            f570(v1)
        } else {
            let a1859 = v0;
            let a1860 = 2u64;
            if nat_eq(a1859, a1860) {
                f571(v1)
            } else {
                let a1861 = v0;
                let a1862 = 3u64;
                if nat_eq(a1861, a1862) {
                    f572(v1)
                } else {
                    let a1863 = v0;
                    let a1864 = 4u64;
                    if nat_eq(a1863, a1864) {
                        f573(v1)
                    } else {
                        let a1865 = v0;
                        let a1866 = 5u64;
                        if nat_eq(a1865, a1866) {
                            f574(v1)
                        } else {
                            let a1867 = v0;
                            let a1868 = 6u64;
                            if nat_eq(a1867, a1868) {
                                f575(v1)
                            } else {
                                let a1869 = v0;
                                let a1870 = 7u64;
                                if nat_eq(a1869, a1870) {
                                    f576(v1)
                                } else {
                                    f577(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f490(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1871 = v1.clone();
    let a1872 = Str::lit("#");
    if equal(a1871, a1872) {
        Some(v0)
    } else {
        let m1873 = f260(v0);
        match m1873 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1874 = f180(Str::lit("$defs"), v2);
                match m1874 {
                    Adt135::C0 => {
                        None::<Adt6>
                    }
                    Adt135::C1(v3) => {
                        let m1875 = f260(v3);
                        match m1875 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f578(v1, match v4 {
                                    Adt5::C0(h1876) => {
                                        h1876
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

pub fn f491(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt125) -> R<Adt147> {
    let m1877 = v3.clone();
    match m1877.uncons() {
        None => {
            Ok(Adt147::C0(Adt97::C8(v0)))
        }
        Some((_, _)) => {
            if f192(v3.clone(), true) {
                let m1878 = f579(v1);
                match m1878 {
                    None => {
                        Ok(Adt147::C0(Adt97::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f580(v3)?;
                        if f581(v9.clone(), match v5.clone() {
                            Adt125::C0(_, h1879) => {
                                h1879
                            }
                        }) {
                            Ok(Adt147::C0(Adt97::C12(v0, v2)))
                        } else {
                            Ok(Adt147::C1(Adt125::C0(List::cons(Adt12::C0(v0, v2, f582(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt125::C0(h1880, _) => {
                                    h1880
                                }
                            }), List::cons(v9, match v5 {
                                Adt125::C0(_, h1881) => {
                                    h1881
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt147::C0(Adt97::C9(v0)))
            }
        }
    }
}

pub fn f492(p1886: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1887: List<Adt105>) -> List<Adt105> {
    let m1885 = (p1886, p1887);
    {
        let mut m1885 = m1885;
        loop {
            let step = {
                let (v0, v3) = m1885;
                let m1882 = v0;
                match m1882.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v5, List::cons(Adt105::C0({
                            let (_, h1883) = v4;
                            h1883
                        }, v1, List::cons({
                            let (h1884, _) = v4;
                            h1884
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(p1891: List<(Str, Adt6)>, p1892: List<Adt105>) -> List<Adt105> {
    let m1890 = (p1891, p1892);
    {
        let mut m1890 = m1890;
        loop {
            let step = {
                let (v0, v1) = m1890;
                let m1888 = v0;
                match m1888.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0({
                            let (_, h1889) = v2;
                            h1889
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(p1895: List<Adt6>, p1896: List<Adt105>) -> List<Adt105> {
    let m1894 = (p1895, p1896);
    {
        let mut m1894 = m1894;
        loop {
            let step = {
                let (v0, v1) = m1894;
                let m1893 = v0;
                match m1893.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: Str) -> bool {
    let a1897 = v0.clone();
    let a1898 = Str::lit("null");
    if equal(a1897, a1898) {
        true
    } else {
        let a1899 = v0.clone();
        let a1900 = Str::lit("boolean");
        if equal(a1899, a1900) {
            true
        } else {
            let a1901 = v0.clone();
            let a1902 = Str::lit("object");
            if equal(a1901, a1902) {
                true
            } else {
                let a1903 = v0.clone();
                let a1904 = Str::lit("array");
                if equal(a1903, a1904) {
                    true
                } else {
                    let a1905 = v0.clone();
                    let a1906 = Str::lit("number");
                    if equal(a1905, a1906) {
                        true
                    } else {
                        let a1907 = v0.clone();
                        let a1908 = Str::lit("string");
                        if equal(a1907, a1908) {
                            true
                        } else {
                            let a1909 = v0;
                            let a1910 = Str::lit("integer");
                            equal(a1909, a1910)
                        }
                    }
                }
            }
        }
    }
}

pub fn f496(v0: Str, v1: Adt6) -> R<bool> {
    let a1911 = v0.clone();
    let a1912 = f583(v1.clone());
    if equal(a1911, a1912) {
        Ok(true)
    } else {
        let a1913 = v0;
        let a1914 = Str::lit("integer");
        if equal(a1913, a1914) {
            let m1915 = f500(v1);
            match m1915 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f584(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f497(p1918: List<Adt6>, v1: Adt6, p1919: bool) -> R<Adt148> {
    let m1917 = (p1918, p1919);
    {
        let mut m1917 = m1917;
        loop {
            let step = {
                let (v0, v2) = m1917;
                let m1916 = v0;
                match m1916.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt148>>(Ok(Adt148::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt148>>(Ok(Adt148::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f495(f585(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt148>>((v4, if v2 {
                                true
                            } else {
                                f496(f585(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt148>>(Ok(Adt148::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(p1923: List<Adt6>, v1: Adt6) -> R<Adt148> {
    let m1922 = p1923;
    {
        let mut m1922 = m1922;
        loop {
            let step = {
                let v0 = m1922;
                let m1920 = v0;
                match m1920.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt148>>(Ok(Adt148::C1))
                    }
                    Some((v2, v3)) => {
                        let m1921 = f499(v2, v1.clone())?;
                        match m1921 {
                            Adt152::C0 => {
                                Err::<List<Adt6>, R<Adt148>>(Ok(Adt148::C0))
                            }
                            Adt152::C1 => {
                                Ok::<List<Adt6>, R<Adt148>>(v3)
                            }
                            Adt152::C2 => {
                                Err::<List<Adt6>, R<Adt148>>(Ok(Adt148::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1922 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(v0: Adt6, v1: Adt6) -> R<Adt152> {
    f586(f587(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f500(v0: Adt6) -> Option<Adt7> {
    let m1924 = v0;
    match m1924 {
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

pub fn f501(v0: u64, v1: Adt153) -> bool {
    let a1925 = v0;
    let a1926 = 0u64;
    if nat_eq(a1925, a1926) {
        let a1927 = f588(v1);
        bool_not(a1927)
    } else {
        let a1928 = v0;
        let a1929 = 1u64;
        if nat_eq(a1928, a1929) {
            let a1930 = f514(v1);
            bool_not(a1930)
        } else {
            let a1931 = v0;
            let a1932 = 2u64;
            if nat_eq(a1931, a1932) {
                f514(v1)
            } else {
                f588(v1)
            }
        }
    }
}

pub fn f502(v0: Adt7, v1: Adt7) -> R<Adt153> {
    f589(v0.clone(), v1.clone(), f590(match v0 {
        Adt7::C0(h1933, _) => {
            h1933
        }
    }), f590(match v1 {
        Adt7::C0(h1934, _) => {
            h1934
        }
    }))
}

pub fn f503(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt148> {
    let m1935 = f237(v0, v2);
    match m1935 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v4) => {
            let m1936 = f591(v4);
            match m1936 {
                None => {
                    Ok(Adt148::C2)
                }
                Some(v5) => {
                    let m1939 = {
                        let a1937 = v5;
                        let a1938 = 0i64;
                        compare(a1937, a1938)
                    };
                    if match m1939 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt148::C2)
                    } else {
                        if v1 {
                            if f588(f592(f79(v3, 0i64)?, v5)) {
                                Ok(Adt148::C1)
                            } else {
                                Ok(Adt148::C0)
                            }
                        } else {
                            if f514(f592(f79(v3, 0i64)?, v5)) {
                                Ok(Adt148::C1)
                            } else {
                                Ok(Adt148::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f504(v0: Str) -> R<u64> {
    f593(f85({
        let a1940 = v0;
        utf8_encode(a1940)
    })?, 0u64)
}

pub fn f505(p1945: List<Adt6>, p1946: u64) -> R<u64> {
    let m1944 = (p1945, p1946);
    {
        let mut m1944 = m1944;
        loop {
            let step = {
                let (v0, v1) = m1944;
                let m1941 = v0;
                match m1941.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1942 = v1;
                            let a1943 = 1u64;
                            nat_add(a1942, a1943)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1944 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(p1951: List<(Str, Adt6)>, p1952: u64) -> R<u64> {
    let m1950 = (p1951, p1952);
    {
        let mut m1950 = m1950;
        loop {
            let step = {
                let (v0, v1) = m1950;
                let m1947 = v0;
                match m1947.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1948 = v1;
                            let a1949 = 1u64;
                            nat_add(a1948, a1949)?
                        }))
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

pub fn f507(p1956: List<Adt6>, v1: Adt5) -> Adt148 {
    let m1955 = p1956;
    {
        let mut m1955 = m1955;
        loop {
            let step = {
                let v0 = m1955;
                let m1953 = v0;
                match m1953.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt148>(Adt148::C0)
                    }
                    Some((v2, v3)) => {
                        let m1954 = f234(v2);
                        match m1954 {
                            None => {
                                Err::<List<Adt6>, Adt148>(Adt148::C2)
                            }
                            Some(v4) => {
                                if f238(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt148>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt148>(Adt148::C1)
                                }
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

pub fn f508(p1963: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt107>, p1964: List<Adt109>) -> List<Adt109> {
    let m1962 = (p1963, p1964);
    {
        let mut m1962 = m1962;
        loop {
            let step = {
                let (v0, v3) = m1962;
                let m1957 = v0;
                match m1957.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1959 = f237({
                            let (h1958, _) = v4.clone();
                            h1958
                        }, v1.clone());
                        match m1959 {
                            Adt135::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, v3))
                            }
                            Adt135::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0({
                                    let (_, h1960) = v4;
                                    h1960
                                }, v6, List::cons(Adt107::C0({
                                    let (h1961, _) = v4;
                                    h1961
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1962 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(p1969: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1968 = p1969;
    {
        let mut m1968 = m1968;
        loop {
            let step = {
                let v0 = m1968;
                let m1965 = v0;
                match m1965.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1967 = f237({
                            let (h1966, _) = v2;
                            h1966
                        }, v1.clone());
                        match m1967 {
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
                Ok(next) => { m1968 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f510(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1970 = f237(Str::lit("properties"), v0);
    match m1970 {
        Adt135::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt135::C1(v1) => {
            let m1971 = f260(v1);
            match m1971 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1972) => {
                            h1972
                        }
                    }
                }
            }
        }
    }
}

pub fn f511(p1979: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>, p1980: List<Adt109>) -> List<Adt109> {
    let m1978 = (p1979, p1980);
    {
        let mut m1978 = m1978;
        loop {
            let step = {
                let (v0, v4) = m1978;
                let m1973 = v0;
                match m1973.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1975 = f237({
                            let (h1974, _) = v5.clone();
                            h1974
                        }, v1.clone());
                        match m1975 {
                            Adt135::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v6, List::cons(Adt109::C0(v2.clone(), {
                                    let (_, h1976) = v5;
                                    h1976
                                }, List::cons(Adt107::C0({
                                    let (h1977, _) = v5;
                                    h1977
                                }), v3.clone())), v4)))
                            }
                            Adt135::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v6, v4))
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

pub fn f512(p1985: List<Adt6>, p1986: u64, v2: Adt6, v3: List<Adt107>, p1987: List<Adt109>) -> R<List<Adt109>> {
    let m1984 = (p1985, (p1986, p1987));
    {
        let mut m1984 = m1984;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1984;
                let m1981 = v0;
                match m1981.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt109>)), R<List<Adt109>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt109>)), R<List<Adt109>>>((v6, ({
                            let a1982 = v1;
                            let a1983 = 1u64;
                            nat_add(a1982, a1983)?
                        }, List::cons(Adt109::C0(v2.clone(), v5, List::cons(Adt107::C1(v1), v3.clone())), v4))))
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

pub fn f513(p1990: List<Adt6>, v1: Adt6, v2: List<Adt107>, p1991: List<Adt109>) -> List<Adt109> {
    let m1989 = (p1990, p1991);
    {
        let mut m1989 = m1989;
        loop {
            let step = {
                let (v0, v3) = m1989;
                let m1988 = v0;
                match m1988.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0(v4, v1.clone(), v2.clone()), v3)))
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

pub fn f514(v0: Adt153) -> bool {
    let m1992 = v0;
    match m1992 {
        Adt153::C0 => {
            false
        }
        Adt153::C1 => {
            false
        }
        Adt153::C2 => {
            true
        }
    }
}

pub fn f515() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f516(p2004: u64, p2005: i64, p2006: List<u64>) -> R<List<u64>> {
    let m2003 = (p2004, (p2005, p2006));
    {
        let mut m2003 = m2003;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2003;
                let m1993 = v0;
                if m1993 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1993 - 1;
                    let v4: i64 = {
                        let a1994 = v1;
                        let a1995 = 10i64;
                        let a1996 = 0i64;
                        int_quot(a1994, a1995, a1996)?
                    };
                    let v5: i64 = {
                        let a1997 = v1;
                        let a1998 = 10i64;
                        let a1999 = 0i64;
                        int_rem(a1997, a1998, a1999)
                    };
                    let m2002 = {
                        let a2000 = v4;
                        let a2001 = 1i64;
                        compare(a2000, a2001)
                    };
                    if match m2002 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f594(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f594(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2003 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: u64) -> R<u64> {
    let a2007 = v0;
    let a2008 = 10u64;
    if nat_lt(a2007, a2008) {
        let a2009 = 48u64;
        let a2010 = v0;
        nat_add(a2009, a2010)
    } else {
        let a2011 = 87u64;
        let a2012 = v0;
        nat_add(a2011, a2012)
    }
}

pub fn f518(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2013 = v1;
    match m2013 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f595(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f519(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2014 = v0;
    match m2014 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2017 = f435(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2015, _) => {
                        h2015
                    }
                });
                let a2018 = f435(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2016) => {
                        h2016
                    }
                });
                append_list(a2017, a2018)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f520(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f596(v0, 0i64)?, 0i64)))
}

pub fn f521(v0: Adt11) -> Str {
    let m2019 = v0;
    match m2019 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f522(p2022: List<Adt36>, p2023: List<Adt6>) -> List<Adt6> {
    let m2021 = (p2022, p2023);
    {
        let mut m2021 = m2021;
        loop {
            let step = {
                let (v0, v1) = m2021;
                let m2020 = v0;
                match m2020.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt36>, List<Adt6>), List<Adt6>>((v3, List::cons(f597(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2021 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(p2027: List<Adt39>, p2028: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2026 = (p2027, p2028);
    {
        let mut m2026 = m2026;
        loop {
            let step = {
                let (v0, v1) = m2026;
                let m2024 = v0;
                match m2024.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2025 = f598(v2)?;
                        match m2025 {
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
                Ok(next) => { m2026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(p2031: List<Adt45>, p2032: List<Adt6>) -> R<List<Adt6>> {
    let m2030 = (p2031, p2032);
    {
        let mut m2030 = m2030;
        loop {
            let step = {
                let (v0, v1) = m2030;
                let m2029 = v0;
                match m2029.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f599(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2030 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(p2035: List<Adt47>, p2036: List<Adt6>) -> List<Adt6> {
    let m2034 = (p2035, p2036);
    {
        let mut m2034 = m2034;
        loop {
            let step = {
                let (v0, v1) = m2034;
                let m2033 = v0;
                match m2033.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), List<Adt6>>((v3, List::cons(f600(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(p2039: List<Adt55>, p2040: List<Adt6>) -> List<Adt6> {
    let m2038 = (p2039, p2040);
    {
        let mut m2038 = m2038;
        loop {
            let step = {
                let (v0, v1) = m2038;
                let m2037 = v0;
                match m2037.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt55>, List<Adt6>), List<Adt6>>((v3, List::cons(f601(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2038 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(p2043: List<Adt57>, p2044: List<Adt6>) -> List<Adt6> {
    let m2042 = (p2043, p2044);
    {
        let mut m2042 = m2042;
        loop {
            let step = {
                let (v0, v1) = m2042;
                let m2041 = v0;
                match m2041.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f602(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2042 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(p2048: List<Adt60>, p2049: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2047 = (p2048, p2049);
    {
        let mut m2047 = m2047;
        loop {
            let step = {
                let (v0, v1) = m2047;
                let m2045 = v0;
                match m2045.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2046 = f603(v2)?;
                        match m2046 {
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
                Ok(next) => { m2047 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(p2055: List<(Str, Adt65)>, p2056: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m2054 = (p2055, p2056);
    {
        let mut m2054 = m2054;
        loop {
            let step = {
                let (v0, v1) = m2054;
                let m2050 = v0;
                match m2050.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2052 = f604({
                            let (_, h2051) = v2;
                            h2051
                        });
                        match m2052 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h2053, _) = v2;
                                    h2053
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2054 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f530(v0: Option<i64>) -> Adt6 {
    let m2057 = v0;
    match m2057 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f605(v1)
        }
    }
}

pub fn f531(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m2058 = v1;
    match m2058 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f605(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f532(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2059 = v0;
    match m2059 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f606(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f533(v0: Adt6) -> Option<Adt85> {
    let m2060 = f260(v0);
    match m2060 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m2061 = f607(Str::lit("task"), v1.clone());
            match m2061 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m2062 = f608(v2);
                    match m2062 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m2063 = f609(Str::lit("createdMs"), v1.clone());
                            match m2063 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m2064 = f607(Str::lit("plan"), v1);
                                    match m2064 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            let m2065 = f610(v5);
                                            match m2065 {
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

pub fn f534(p2069: List<Adt85>, p2070: List<Adt6>) -> Option<List<Adt6>> {
    let m2068 = (p2069, p2070);
    {
        let mut m2068 = m2068;
        loop {
            let step = {
                let (v0, v1) = m2068;
                let m2066 = v0;
                match m2066.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2067 = f611(v2);
                        match m2067 {
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
                Ok(next) => { m2068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f535(v0: Adt85, v1: i64) -> R<bool> {
    let m2074 = {
        let m2073 = f448(match v0.clone() {
            Adt85::C0(h2072, _, _) => {
                h2072
            }
        });
        match m2073 {
            Adt64::C0(_, _, _, _, h2071, _) => {
                h2071
            }
        }
    };
    match m2074 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a2081 = {
                let m2080 = {
                    let a2078 = {
                        let a2076 = v1;
                        let a2077 = match v0 {
                            Adt85::C0(_, h2075, _) => {
                                h2075
                            }
                        };
                        int_sub(a2076, a2077)?
                    };
                    let a2079 = v2;
                    compare(a2078, a2079)
                };
                match m2080 {
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
            Ok(bool_not(a2081))
        }
    }
}

pub fn f536() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f537() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f538() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f539() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f540() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f541(p2086: List<List<u64>>, p2087: List<u64>) -> List<u64> {
    let m2085 = (p2086, p2087);
    {
        let mut m2085 = m2085;
        loop {
            let step = {
                let (v0, v1) = m2085;
                let m2082 = v0;
                match m2082.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2083 = f612(v2);
                            let a2084 = v1;
                            append_list(a2083, a2084)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2085 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542(v0: Str) -> R<Str> {
    let m2090 = {
        let a2089 = f275(f87(), f167(f613(f85({
            let a2088 = v0;
            utf8_encode(a2088)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2089)
    };
    match m2090 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f543(v0: List<u64>) -> bool {
    let m2091 = v0;
    match m2091.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2092 = v1;
            let a2093 = 48u64;
            if nat_eq(a2092, a2093) {
                f614(v2)
            } else {
                false
            }
        }
    }
}

pub fn f544(v0: Adt0, v1: Fn2, v2: u64, v3: Adt15) -> R<Result<Adt128, Adt131>> {
    if f615(f616(v3.clone())) {
        Ok(Ok::<Adt128, Adt131>(f617(v0.clone(), Adt33::C0(f618(v0)))))
    } else {
        Ok(f619(v0.clone(), v3.clone(), f620(match v0 {
            Adt0::C0(_, _, _, _, _, h2094) => {
                h2094
            }
        }, v2, v1, v3)?))
    }
}

pub fn f545(v0: Adt17, _: Adt5) -> Result<Adt15, Adt132> {
    Ok::<Adt15, Adt132>(Adt15::C0(Adt16::C0(v0)))
}

pub fn f546(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2095 = f621(Str::lit("cursor"), v1);
    match m2095 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C1(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f547(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2096 = f622(Str::lit("name"), v1.clone());
    match m2096 {
        Ok(v2) => {
            let m2097 = f623(Str::lit("arguments"), v1.clone());
            match m2097 {
                Ok(v3) => {
                    let m2098 = f623(Str::lit("inputResponses"), v1.clone());
                    match m2098 {
                        Ok(v4) => {
                            let m2099 = f621(Str::lit("requestState"), v1);
                            match m2099 {
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

pub fn f548(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2100 = f621(Str::lit("cursor"), v1);
    match m2100 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C3(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f549(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2101 = f622(Str::lit("uri"), v1.clone());
    match m2101 {
        Ok(v2) => {
            let m2102 = f623(Str::lit("inputResponses"), v1.clone());
            match m2102 {
                Ok(v3) => {
                    let m2103 = f621(Str::lit("requestState"), v1);
                    match m2103 {
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

pub fn f550(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2104 = f621(Str::lit("cursor"), v1);
    match m2104 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C5(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f551(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2105 = f621(Str::lit("cursor"), v1);
    match m2105 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C6(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f552(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2106 = f622(Str::lit("name"), v1.clone());
    match m2106 {
        Ok(v2) => {
            let m2107 = f624(Str::lit("arguments"), v1.clone());
            match m2107 {
                Ok(v3) => {
                    let m2108 = f623(Str::lit("inputResponses"), v1.clone());
                    match m2108 {
                        Ok(v4) => {
                            let m2109 = f621(Str::lit("requestState"), v1);
                            match m2109 {
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

pub fn f553(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2110 = f625(Str::lit("ref"), v1.clone());
    match m2110 {
        Ok(v2) => {
            let m2111 = f626(v2);
            match m2111 {
                Ok(v3) => {
                    let m2112 = f625(Str::lit("argument"), v1.clone());
                    match m2112 {
                        Ok(v4) => {
                            let m2113 = f627(v4);
                            match m2113 {
                                Ok(v5) => {
                                    let m2114 = f628(v1);
                                    match m2114 {
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

pub fn f554(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2115 = f622(Str::lit("taskId"), v1);
    match m2115 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C9(Adt30::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f555(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2116 = f622(Str::lit("taskId"), v1.clone());
    match m2116 {
        Ok(v2) => {
            let m2117 = f625(Str::lit("inputResponses"), v1);
            match m2117 {
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

pub fn f556(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2118 = f622(Str::lit("taskId"), v1);
    match m2118 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C11(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f557(v0: Adt104, v1: Adt17, v2: Option<Adt131>) -> Adt143 {
    let m2119 = v2;
    match m2119 {
        None => {
            Adt143::C0(Adt144::C0(match v0.clone() {
                Adt104::C0(h2120, _, _) => {
                    h2120
                }
            }, match v0.clone() {
                Adt104::C0(_, h2121, _) => {
                    h2121
                }
            }, v1, f481(match v0 {
                Adt104::C0(_, _, h2122) => {
                    h2122
                }
            })))
        }
        Some(v3) => {
            Adt143::C1(v3)
        }
    }
}

pub fn f558(v0: Str, v1: List<Str>) -> Option<Adt131> {
    f629(f630(v0.clone(), v1.clone()), v0, v1)
}

pub fn f559(v0: Adt112, v1: Adt145, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt113>> {
    let m2124 = match v0.clone() {
        Adt112::C0(h2123, _, _, _, _) => {
            h2123
        }
    };
    match m2124 {
        None => {
            Ok(Some(Adt113::C0))
        }
        Some(v5) => {
            let a2125 = v5;
            let a2126 = v3;
            if equal(a2125, a2126) {
                let m2128 = match v0.clone() {
                    Adt112::C0(_, h2127, _, _, _) => {
                        h2127
                    }
                };
                match m2128 {
                    None => {
                        Ok(Some(Adt113::C1))
                    }
                    Some(v6) => {
                        let a2129 = v6;
                        let a2130 = v2;
                        if equal(a2129, a2130) {
                            f631(f632(match v0.clone() {
                                Adt112::C0(_, _, h2131, _, _) => {
                                    h2131
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt112::C0(_, _, _, _, h2132) => {
                                    h2132
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt113::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt113::C3))
            }
        }
    }
}

pub fn f560() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f561(v0: Adt5) -> Adt154 {
    f633(f180(f563(), v0.clone()), f180(f564(), v0))
}

pub fn f562(v0: Adt5, v1: Adt135, v2: Adt135) -> Result<Adt17, Adt132> {
    let m2133 = v1;
    match m2133 {
        Adt135::C0 => {
            Err::<Adt17, Adt132>(Adt132::C2)
        }
        Adt135::C1(v3) => {
            let m2134 = v2;
            match m2134 {
                Adt135::C0 => {
                    Err::<Adt17, Adt132>(Adt132::C3)
                }
                Adt135::C1(v4) => {
                    f634(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f563() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f564() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f565(v0: u64) -> R<Option<u64>> {
    let a2135 = 64u64;
    let a2136 = v0;
    if if nat_lt(a2135, a2136) {
        let a2137 = v0;
        let a2138 = 91u64;
        nat_lt(a2137, a2138)
    } else {
        false
    } {
        Ok(Some({
            let a2139 = v0;
            let a2140 = 65u64;
            nat_sub(a2139, a2140)
        }))
    } else {
        let a2141 = 96u64;
        let a2142 = v0;
        if if nat_lt(a2141, a2142) {
            let a2143 = v0;
            let a2144 = 123u64;
            nat_lt(a2143, a2144)
        } else {
            false
        } {
            Ok(Some({
                let a2145 = v0;
                let a2146 = 71u64;
                nat_sub(a2145, a2146)
            }))
        } else {
            let a2147 = 47u64;
            let a2148 = v0;
            if if nat_lt(a2147, a2148) {
                let a2149 = v0;
                let a2150 = 58u64;
                nat_lt(a2149, a2150)
            } else {
                false
            } {
                Ok(Some({
                    let a2151 = v0;
                    let a2152 = 4u64;
                    nat_add(a2151, a2152)?
                }))
            } else {
                let a2153 = v0;
                let a2154 = 43u64;
                if nat_eq(a2153, a2154) {
                    Ok(Some(62u64))
                } else {
                    let a2155 = v0;
                    let a2156 = 47u64;
                    if nat_eq(a2155, a2156) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f566(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2157 = v3;
        let a2158 = 61u64;
        if nat_eq(a2157, a2158) {
            let a2159 = f635(v1, 16u64);
            let a2160 = 0u64;
            nat_eq(a2159, a2160)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f636(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f567(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2161 = f635(v2, 4u64);
        let a2162 = 0u64;
        nat_eq(a2161, a2162)
    } else {
        false
    } {
        Ok(Some(f637(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f568(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2165 = {
            let a2163 = v0;
            let a2164 = 4u64;
            nat_mul(a2163, a2164)?
        };
        let a2166 = f638(v1, 16u64);
        nat_add(a2165, a2166)?
    }, List::cons({
        let a2169 = {
            let a2167 = f635(v1, 16u64);
            let a2168 = 16u64;
            nat_mul(a2167, a2168)?
        };
        let a2170 = f638(v2, 4u64);
        nat_add(a2169, a2170)?
    }, List::cons({
        let a2173 = {
            let a2171 = f635(v2, 4u64);
            let a2172 = 64u64;
            nat_mul(a2171, a2172)?
        };
        let a2174 = v3;
        nat_add(a2173, a2174)?
    }, List::<u64>::nil()))))
}

pub fn f569(v0: Adt6) -> bool {
    let m2175 = f234(v0);
    match m2175 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f570(v0: Adt6) -> bool {
    let m2176 = f500(v0);
    match m2176 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f571(v0: Adt6) -> bool {
    let m2177 = f591(v0);
    match m2177 {
        None => {
            false
        }
        Some(v1) => {
            let a2181 = {
                let m2180 = {
                    let a2178 = v1;
                    let a2179 = 0i64;
                    compare(a2178, a2179)
                };
                match m2180 {
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
            bool_not(a2181)
        }
    }
}

pub fn f572(v0: Adt6) -> bool {
    let m2182 = f333(v0);
    match m2182 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f573(v0: Adt6) -> bool {
    let m2183 = f260(v0);
    match m2183 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f574(v0: Adt6) -> bool {
    let m2184 = f333(v0);
    match m2184 {
        None => {
            false
        }
        Some(v1) => {
            f639(v1)
        }
    }
}

pub fn f575(v0: Adt6) -> bool {
    let m2185 = f259(v0);
    match m2185 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f576(v0: Adt6) -> bool {
    let m2186 = f234(v0.clone());
    match m2186 {
        None => {
            let m2187 = f333(v0);
            match m2187 {
                None => {
                    false
                }
                Some(v2) => {
                    f640(v2)
                }
            }
        }
        Some(v1) => {
            f495(v1)
        }
    }
}

pub fn f577(v0: Adt6) -> bool {
    let m2188 = f333(v0);
    match m2188 {
        None => {
            false
        }
        Some(v1) => {
            let m2189 = v1;
            match m2189.uncons() {
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

pub fn f578(v0: Str, p2207: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2206 = p2207;
    {
        let mut m2206 = m2206;
        loop {
            let step = {
                let v1 = m2206;
                let m2190 = v1;
                match m2190.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2194 = v0.clone();
                        let a2195 = {
                            let a2192 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2191, _) = v2.clone();
                                h2191
                            }, List::<Str>::nil()));
                            let a2193 = Str::lit("");
                            join(a2192, a2193)
                        };
                        if equal(a2194, a2195) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2196) = v2;
                                h2196
                            }))
                        } else {
                            let m2198 = f260({
                                let (_, h2197) = v2.clone();
                                h2197
                            });
                            match m2198 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2199 = f180(Str::lit("$anchor"), v4);
                                    match m2199 {
                                        Adt135::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt135::C1(v5) => {
                                            let m2200 = f234(v5);
                                            match m2200 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2203 = v0.clone();
                                                    let a2204 = {
                                                        let a2201 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2202 = Str::lit("");
                                                        join(a2201, a2202)
                                                    };
                                                    if equal(a2203, a2204) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2205) = v2;
                                                            h2205
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
                Ok(next) => { m2206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2208 = f237(Str::lit("type"), v0);
    match m2208 {
        Adt135::C0 => {
            None::<Adt13>
        }
        Adt135::C1(v1) => {
            let m2209 = f234(v1);
            match m2209 {
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

pub fn f580(v0: List<u64>) -> R<List<u64>> {
    Ok(f167(f642(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f581(v0: List<u64>, p2212: List<List<u64>>) -> bool {
    let m2211 = p2212;
    {
        let mut m2211 = m2211;
        loop {
            let step = {
                let v1 = m2211;
                let m2210 = v1;
                match m2210.uncons() {
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
                Ok(next) => { m2211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(p2215: List<Str>, p2216: List<Str>) -> List<Str> {
    let m2214 = (p2215, p2216);
    {
        let mut m2214 = m2214;
        loop {
            let step = {
                let (v0, v1) = m2214;
                let m2213 = v0;
                match m2213.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: Adt6) -> Str {
    let m2217 = v0;
    match m2217 {
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

pub fn f584(v0: Adt7) -> R<bool> {
    if f394(match v0.clone() {
        Adt7::C0(h2218, _) => {
            h2218
        }
    }) {
        Ok(true)
    } else {
        f644(40u64, match v0.clone() {
            Adt7::C0(h2219, _) => {
                h2219
            }
        }, match v0 {
            Adt7::C0(_, h2220) => {
                h2220
            }
        })
    }
}

pub fn f585(v0: Adt6) -> Str {
    let m2221 = f234(v0);
    match m2221 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f586(p2228: u64, p2229: List<(Adt6, Adt6)>) -> R<Adt152> {
    let m2227 = (p2228, p2229);
    {
        let mut m2227 = m2227;
        loop {
            let step = {
                let (v0, v1) = m2227;
                let m2222 = v0;
                if m2222 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt152>>(Ok(Adt152::C2))
                } else {
                    let v2 = m2222 - 1;
                    let m2223 = v1;
                    match m2223.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt152>>(Ok(Adt152::C0))
                        }
                        Some((v3, v4)) => {
                            let m2226 = f645({
                                let (h2224, _) = v3;
                                h2224
                            }, {
                                let (_, h2225) = v3;
                                h2225
                            }, v4)?;
                            match m2226 {
                                Adt155::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt152>>(Ok(Adt152::C1))
                                }
                                Adt155::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt152>>((v2, v5))
                                }
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

pub fn f587() -> u64 {
    67108864u64
}

pub fn f588(v0: Adt153) -> bool {
    let m2230 = v0;
    match m2230 {
        Adt153::C0 => {
            true
        }
        Adt153::C1 => {
            false
        }
        Adt153::C2 => {
            false
        }
    }
}

pub fn f589(v0: Adt7, v1: Adt7, v2: Adt153, v3: Adt153) -> R<Adt153> {
    let m2231 = v2;
    match m2231 {
        Adt153::C0 => {
            let m2233 = v3;
            match m2233 {
                Adt153::C0 => {
                    Ok(f646(Adt153::C0, f647(f648(match v0.clone() {
                        Adt7::C0(h2234, _) => {
                            h2234
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2235) => {
                            h2235
                        }
                    }, f648(match v1.clone() {
                        Adt7::C0(h2236, _) => {
                            h2236
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2237) => {
                            h2237
                        }
                    })?))
                }
                Adt153::C1 => {
                    Ok(Adt153::C0)
                }
                Adt153::C2 => {
                    Ok(Adt153::C0)
                }
            }
        }
        Adt153::C1 => {
            let m2232 = v3;
            match m2232 {
                Adt153::C0 => {
                    Ok(Adt153::C2)
                }
                Adt153::C1 => {
                    Ok(Adt153::C1)
                }
                Adt153::C2 => {
                    Ok(Adt153::C0)
                }
            }
        }
        Adt153::C2 => {
            let m2238 = v3;
            match m2238 {
                Adt153::C0 => {
                    Ok(Adt153::C2)
                }
                Adt153::C1 => {
                    Ok(Adt153::C2)
                }
                Adt153::C2 => {
                    f647(match v0.clone() {
                        Adt7::C0(h2239, _) => {
                            h2239
                        }
                    }, match v0 {
                        Adt7::C0(_, h2240) => {
                            h2240
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2241, _) => {
                            h2241
                        }
                    }, match v1 {
                        Adt7::C0(_, h2242) => {
                            h2242
                        }
                    })
                }
            }
        }
    }
}

pub fn f590(v0: i64) -> Adt153 {
    f592(v0, 0i64)
}

pub fn f591(v0: Adt6) -> Option<i64> {
    let m2243 = v0;
    match m2243 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f394(match v2.clone() {
                Adt7::C0(_, h2244) => {
                    h2244
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2245, _) => {
                        h2245
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

pub fn f592(v0: i64, v1: i64) -> Adt153 {
    let m2248 = {
        let a2246 = v0;
        let a2247 = v1;
        compare(a2246, a2247)
    };
    if match m2248 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Adt153::C0
    } else {
        let m2251 = {
            let a2249 = v1;
            let a2250 = v0;
            compare(a2249, a2250)
        };
        if match m2251 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt153::C2
        } else {
            Adt153::C1
        }
    }
}

pub fn f593(p2260: List<u64>, p2261: u64) -> R<u64> {
    let m2259 = (p2260, p2261);
    {
        let mut m2259 = m2259;
        loop {
            let step = {
                let (v0, v1) = m2259;
                let m2252 = v0;
                match m2252.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2253 = v2;
                        let a2254 = 128u64;
                        if if nat_lt(a2253, a2254) {
                            true
                        } else {
                            let a2255 = 191u64;
                            let a2256 = v2;
                            nat_lt(a2255, a2256)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2257 = v1;
                                let a2258 = 1u64;
                                nat_add(a2257, a2258)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2259 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(v0: i64) -> u64 {
    let m2264 = {
        let a2262 = v0;
        let a2263 = 1i64;
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
        48u64
    } else {
        let m2267 = {
            let a2265 = v0;
            let a2266 = 2i64;
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
            49u64
        } else {
            let m2270 = {
                let a2268 = v0;
                let a2269 = 3i64;
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
                50u64
            } else {
                let m2273 = {
                    let a2271 = v0;
                    let a2272 = 4i64;
                    compare(a2271, a2272)
                };
                if match m2273 {
                    Ordering::Less => {
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
                    let m2276 = {
                        let a2274 = v0;
                        let a2275 = 5i64;
                        compare(a2274, a2275)
                    };
                    if match m2276 {
                        Ordering::Less => {
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
                        let m2279 = {
                            let a2277 = v0;
                            let a2278 = 6i64;
                            compare(a2277, a2278)
                        };
                        if match m2279 {
                            Ordering::Less => {
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
                            let m2282 = {
                                let a2280 = v0;
                                let a2281 = 7i64;
                                compare(a2280, a2281)
                            };
                            if match m2282 {
                                Ordering::Less => {
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
                                let m2285 = {
                                    let a2283 = v0;
                                    let a2284 = 8i64;
                                    compare(a2283, a2284)
                                };
                                if match m2285 {
                                    Ordering::Less => {
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
                                    let m2288 = {
                                        let a2286 = v0;
                                        let a2287 = 9i64;
                                        compare(a2286, a2287)
                                    };
                                    if match m2288 {
                                        Ordering::Less => {
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

pub fn f595(v0: Adt8) -> Adt6 {
    Adt6::C5(f435(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2289) => {
            h2289
        }
    }))
}

pub fn f596(p2294: u64, p2295: i64) -> R<i64> {
    let m2293 = (p2294, p2295);
    {
        let mut m2293 = m2293;
        loop {
            let step = {
                let (v0, v1) = m2293;
                let m2290 = v0;
                if m2290 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2290 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2291 = v1;
                        let a2292 = -1i64;
                        int_sub(a2291, a2292)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2293 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(v0: Adt36) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt36::C0(h2296, _, _, _, _, _, _, _) => {
            h2296
        }
    })), {
        let a2314 = f430(Str::lit("title"), match v0.clone() {
            Adt36::C0(_, h2297, _, _, _, _, _, _) => {
                h2297
            }
        });
        let a2315 = {
            let a2312 = f532(match v0.clone() {
                Adt36::C0(_, _, h2298, _, _, _, _, _) => {
                    h2298
                }
            });
            let a2313 = {
                let a2310 = f430(Str::lit("description"), match v0.clone() {
                    Adt36::C0(_, _, _, h2299, _, _, _, _) => {
                        h2299
                    }
                });
                let a2311 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2302 = match v0.clone() {
                        Adt36::C0(_, _, _, _, h2301, _, _, _) => {
                            h2301
                        }
                    };
                    match m2302 {
                        Adt5::C0(h2300) => {
                            h2300
                        }
                    }
                })), {
                    let a2308 = f470(Str::lit("outputSchema"), match v0.clone() {
                        Adt36::C0(_, _, _, _, _, h2303, _, _) => {
                            h2303
                        }
                    });
                    let a2309 = {
                        let a2306 = f649(match v0.clone() {
                            Adt36::C0(_, _, _, _, _, _, h2304, _) => {
                                h2304
                            }
                        });
                        let a2307 = f650(match v0 {
                            Adt36::C0(_, _, _, _, _, _, _, h2305) => {
                                h2305
                            }
                        });
                        append_list(a2306, a2307)
                    };
                    append_list(a2308, a2309)
                });
                append_list(a2310, a2311)
            };
            append_list(a2312, a2313)
        };
        append_list(a2314, a2315)
    }))
}

pub fn f598(v0: Adt39) -> R<Option<Adt6>> {
    let m2316 = v0;
    match m2316 {
        Adt39::C0(v1) => {
            Ok(Some(f651(v1)))
        }
        Adt39::C1(v2) => {
            Ok(Some(f652(Str::lit("image"), match v2.clone() {
                Adt43::C0(h2317, _, _, _) => {
                    h2317
                }
            }, match v2.clone() {
                Adt43::C0(_, h2318, _, _) => {
                    h2318
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h2319, _) => {
                    h2319
                }
            }, match v2 {
                Adt43::C0(_, _, _, h2320) => {
                    h2320
                }
            })))
        }
        Adt39::C2(v3) => {
            Ok(Some(f652(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h2321, _, _, _) => {
                    h2321
                }
            }, match v3.clone() {
                Adt44::C0(_, h2322, _, _) => {
                    h2322
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h2323, _) => {
                    h2323
                }
            }, match v3 {
                Adt44::C0(_, _, _, h2324) => {
                    h2324
                }
            })))
        }
        Adt39::C3(v4) => {
            Ok(Some(f653(v4)?))
        }
        Adt39::C4(v5) => {
            Ok(Some(f654(v5)))
        }
    }
}

pub fn f599(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2348 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2325, _, _, _, _, _, _, _, _) => {
                h2325
            }
        })), List::<(Str, Adt6)>::nil());
        let a2349 = {
            let a2346 = f430(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2326, _, _, _, _, _, _, _) => {
                    h2326
                }
            });
            let a2347 = {
                let a2344 = f532(match v0.clone() {
                    Adt45::C0(_, _, h2327, _, _, _, _, _, _) => {
                        h2327
                    }
                });
                let a2345 = {
                    let a2342 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2328, _, _, _, _, _) => {
                            h2328
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2343 = {
                        let a2340 = f430(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2329, _, _, _, _) => {
                                h2329
                            }
                        });
                        let a2341 = {
                            let a2338 = f430(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2330, _, _, _) => {
                                    h2330
                                }
                            });
                            let a2339 = {
                                let a2336 = f655(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2331, _, _) => {
                                        h2331
                                    }
                                });
                                let a2337 = {
                                    let a2334 = f656(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2332, _) => {
                                            h2332
                                        }
                                    })?;
                                    let a2335 = f650(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2333) => {
                                            h2333
                                        }
                                    });
                                    append_list(a2334, a2335)
                                };
                                append_list(a2336, a2337)
                            };
                            append_list(a2338, a2339)
                        };
                        append_list(a2340, a2341)
                    };
                    append_list(a2342, a2343)
                };
                append_list(a2344, a2345)
            };
            append_list(a2346, a2347)
        };
        append_list(a2348, a2349)
    }))
}

pub fn f600(v0: Adt47) -> Adt6 {
    let m2350 = v0;
    match m2350 {
        Adt47::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt48::C0(h2351, _, _, _) => {
                    h2351
                }
            })), {
                let a2355 = f430(Str::lit("mimeType"), match v1.clone() {
                    Adt48::C0(_, h2352, _, _) => {
                        h2352
                    }
                });
                let a2356 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt48::C0(_, _, h2353, _) => {
                        h2353
                    }
                })), f650(match v1 {
                    Adt48::C0(_, _, _, h2354) => {
                        h2354
                    }
                }));
                append_list(a2355, a2356)
            }))
        }
        Adt47::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt49::C0(h2357, _, _, _) => {
                    h2357
                }
            })), {
                let a2361 = f430(Str::lit("mimeType"), match v2.clone() {
                    Adt49::C0(_, h2358, _, _) => {
                        h2358
                    }
                });
                let a2362 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt49::C0(_, _, h2359, _) => {
                        h2359
                    }
                })), f650(match v2 {
                    Adt49::C0(_, _, _, h2360) => {
                        h2360
                    }
                }));
                append_list(a2361, a2362)
            }))
        }
    }
}

pub fn f601(v0: Adt55) -> Adt6 {
    Adt6::C5({
        let a2383 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt55::C0(h2363, _, _, _, _, _, _, _) => {
                h2363
            }
        })), List::<(Str, Adt6)>::nil());
        let a2384 = {
            let a2381 = f430(Str::lit("title"), match v0.clone() {
                Adt55::C0(_, h2364, _, _, _, _, _, _) => {
                    h2364
                }
            });
            let a2382 = {
                let a2379 = f532(match v0.clone() {
                    Adt55::C0(_, _, h2365, _, _, _, _, _) => {
                        h2365
                    }
                });
                let a2380 = {
                    let a2377 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt55::C0(_, _, _, h2366, _, _, _, _) => {
                            h2366
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2378 = {
                        let a2375 = f430(Str::lit("description"), match v0.clone() {
                            Adt55::C0(_, _, _, _, h2367, _, _, _) => {
                                h2367
                            }
                        });
                        let a2376 = {
                            let a2373 = f430(Str::lit("mimeType"), match v0.clone() {
                                Adt55::C0(_, _, _, _, _, h2368, _, _) => {
                                    h2368
                                }
                            });
                            let a2374 = {
                                let a2371 = f655(match v0.clone() {
                                    Adt55::C0(_, _, _, _, _, _, h2369, _) => {
                                        h2369
                                    }
                                });
                                let a2372 = f650(match v0 {
                                    Adt55::C0(_, _, _, _, _, _, _, h2370) => {
                                        h2370
                                    }
                                });
                                append_list(a2371, a2372)
                            };
                            append_list(a2373, a2374)
                        };
                        append_list(a2375, a2376)
                    };
                    append_list(a2377, a2378)
                };
                append_list(a2379, a2380)
            };
            append_list(a2381, a2382)
        };
        append_list(a2383, a2384)
    })
}

pub fn f602(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2400 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2385, _, _, _, _, _) => {
                h2385
            }
        })), List::<(Str, Adt6)>::nil());
        let a2401 = {
            let a2398 = f430(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2386, _, _, _, _) => {
                    h2386
                }
            });
            let a2399 = {
                let a2396 = f532(match v0.clone() {
                    Adt57::C0(_, _, h2387, _, _, _) => {
                        h2387
                    }
                });
                let a2397 = {
                    let a2394 = f430(Str::lit("description"), match v0.clone() {
                        Adt57::C0(_, _, _, h2388, _, _) => {
                            h2388
                        }
                    });
                    let a2395 = {
                        let a2392 = {
                            let m2390 = match v0.clone() {
                                Adt57::C0(_, _, _, _, h2389, _) => {
                                    h2389
                                }
                            };
                            match m2390 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f657(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2393 = f650(match v0 {
                            Adt57::C0(_, _, _, _, _, h2391) => {
                                h2391
                            }
                        });
                        append_list(a2392, a2393)
                    };
                    append_list(a2394, a2395)
                };
                append_list(a2396, a2397)
            };
            append_list(a2398, a2399)
        };
        append_list(a2400, a2401)
    })
}

pub fn f603(v0: Adt60) -> R<Option<Adt6>> {
    let m2403 = f598(match v0.clone() {
        Adt60::C0(_, h2402) => {
            h2402
        }
    })?;
    match m2403 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f658(match v0 {
                Adt60::C0(h2404, _) => {
                    h2404
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f604(v0: Adt65) -> Option<Adt6> {
    let m2405 = v0;
    match m2405 {
        Adt65::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f659(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt65::C1(_) => {
            None::<Adt6>
        }
        Adt65::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m2406 = v3;
                match m2406 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h2407) => {
                                h2407
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f605(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f606(v0: List<Adt2>) -> List<Adt6> {
    f310(f660(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f607(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m2408 = f180(v0, v1);
    match m2408 {
        Adt135::C0 => {
            None::<Adt5>
        }
        Adt135::C1(v2) => {
            f260(v2)
        }
    }
}

pub fn f608(v0: Adt5) -> Option<Adt63> {
    let m2409 = f661(v0.clone());
    match m2409 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m2410 = f662(Str::lit("status"), v0.clone());
            match m2410 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let a2411 = v2.clone();
                    let a2412 = Str::lit("working");
                    if equal(a2411, a2412) {
                        Some(Adt63::C0(v1))
                    } else {
                        let a2413 = v2.clone();
                        let a2414 = Str::lit("cancelled");
                        if equal(a2413, a2414) {
                            Some(Adt63::C4(v1))
                        } else {
                            let a2415 = v2.clone();
                            let a2416 = Str::lit("input_required");
                            if equal(a2415, a2416) {
                                let m2417 = f663(Str::lit("inputRequests"), v0);
                                match m2417 {
                                    None => {
                                        None::<Adt63>
                                    }
                                    Some(v3) => {
                                        Some(Adt63::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a2418 = v2.clone();
                                let a2419 = Str::lit("completed");
                                if equal(a2418, a2419) {
                                    let m2420 = f607(Str::lit("result"), v0);
                                    match m2420 {
                                        None => {
                                            None::<Adt63>
                                        }
                                        Some(v4) => {
                                            Some(Adt63::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a2421 = v2;
                                    let a2422 = Str::lit("failed");
                                    if equal(a2421, a2422) {
                                        let m2423 = f607(Str::lit("error"), v0);
                                        match m2423 {
                                            None => {
                                                None::<Adt63>
                                            }
                                            Some(v5) => {
                                                let m2424 = f664(v5);
                                                match m2424 {
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

pub fn f609(v0: Str, v1: Adt5) -> Option<i64> {
    let m2425 = f180(v0, v1);
    match m2425 {
        Adt135::C0 => {
            None::<i64>
        }
        Adt135::C1(v2) => {
            f591(v2)
        }
    }
}

pub fn f610(v0: Adt5) -> Option<Adt86> {
    let m2426 = f609(Str::lit("parkAfterMs"), v0.clone());
    match m2426 {
        None => {
            None::<Adt86>
        }
        Some(v1) => {
            let m2427 = f663(Str::lit("parked"), v0.clone());
            match m2427 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m2428 = f609(Str::lit("finishAfterMs"), v0.clone());
                    match m2428 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            let m2429 = f607(Str::lit("outcome"), v0);
                            match m2429 {
                                None => {
                                    None::<Adt86>
                                }
                                Some(v4) => {
                                    let m2430 = f665(v4);
                                    match m2430 {
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

pub fn f611(v0: Adt85) -> Option<Adt6> {
    let m2432 = f356(match v0.clone() {
        Adt85::C0(h2431, _, _) => {
            h2431
        }
    });
    match m2432 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m2434 = f666(match v0.clone() {
                Adt85::C0(_, _, h2433) => {
                    h2433
                }
            });
            match m2434 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f605(match v0 {
                        Adt85::C0(_, h2435, _) => {
                            h2435
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f612(v0: List<u64>) -> List<u64> {
    let a2438 = f667();
    let a2439 = {
        let a2436 = v0;
        let a2437 = f668();
        append_list(a2436, a2437)
    };
    append_list(a2438, a2439)
}

pub fn f613(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f669(v0, 0u64, v1)
}

pub fn f614(p2446: List<u64>) -> bool {
    let m2445 = p2446;
    {
        let mut m2445 = m2445;
        loop {
            let step = {
                let v0 = m2445;
                let m2440 = v0;
                match m2440.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2441 = v1;
                        let a2442 = 48u64;
                        if if nat_eq(a2441, a2442) {
                            true
                        } else {
                            let a2443 = v1;
                            let a2444 = 46u64;
                            nat_eq(a2443, a2444)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615(v0: Adt140) -> bool {
    let m2447 = v0;
    match m2447 {
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

pub fn f616(v0: Adt15) -> Adt140 {
    let m2448 = v0;
    match m2448 {
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

pub fn f617(v0: Adt0, v1: Adt33) -> Adt128 {
    Adt128::C0(v1, Adt129::C0(Some(match v0 {
        Adt0::C0(h2449, _, _, _, _, _) => {
            h2449
        }
    }), None::<Adt5>), List::<Adt130>::nil())
}

pub fn f618(v0: Adt0) -> Adt34 {
    Adt34::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2450, _, _) => {
            h2450
        }
    }, f178(), match v0.clone() {
        Adt0::C0(_, _, h2451, _, _, _) => {
            h2451
        }
    }, match v0 {
        Adt0::C0(_, h2452, _, _, _, _) => {
            h2452
        }
    })
}

pub fn f619(v0: Adt0, v1: Adt15, v2: Result<Adt33, Adt90>) -> Result<Adt128, Adt131> {
    let m2453 = v2;
    match m2453 {
        Ok(v4) => {
            if f670(f616(v1.clone()), v4.clone()) {
                f671(v0, v1, v4)
            } else {
                Err::<Adt128, Adt131>(Adt131::C4(f672()))
            }
        }
        Err(v3) => {
            Err::<Adt128, Adt131>(Adt131::C4(v3))
        }
    }
}

pub fn f620(v0: u64, v1: u64, v2: Fn2, v3: Adt15) -> R<Result<Adt33, Adt90>> {
    let m2454 = v3.clone();
    match m2454 {
        Adt15::C0(_) => {
            let c2456: Fn2 = v2;
            c2456.apply(v3)
        }
        Adt15::C1(_) => {
            let c2457: Fn2 = v2;
            c2457.apply(v3)
        }
        Adt15::C2(_) => {
            if f673(v0, v1) {
                let c2455: Fn2 = v2;
                c2455.apply(v3)
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C7(v0)))
            }
        }
        Adt15::C3(_) => {
            let c2458: Fn2 = v2;
            c2458.apply(v3)
        }
        Adt15::C4(_) => {
            let c2459: Fn2 = v2;
            c2459.apply(v3)
        }
        Adt15::C5(_) => {
            let c2460: Fn2 = v2;
            c2460.apply(v3)
        }
        Adt15::C6(_) => {
            let c2461: Fn2 = v2;
            c2461.apply(v3)
        }
        Adt15::C7(_) => {
            let c2462: Fn2 = v2;
            c2462.apply(v3)
        }
        Adt15::C8(_) => {
            let c2463: Fn2 = v2;
            c2463.apply(v3)
        }
        Adt15::C9(_) => {
            let c2464: Fn2 = v2;
            c2464.apply(v3)
        }
        Adt15::C10(_) => {
            let c2465: Fn2 = v2;
            c2465.apply(v3)
        }
        Adt15::C11(_) => {
            let c2466: Fn2 = v2;
            c2466.apply(v3)
        }
    }
}

pub fn f621(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt132> {
    let m2467 = f180(v0.clone(), v1);
    match m2467 {
        Adt135::C0 => {
            Ok::<Option<Str>, Adt132>(None::<Str>)
        }
        Adt135::C1(v2) => {
            let m2468 = f234(v2);
            match m2468 {
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

pub fn f622(v0: Str, v1: Adt5) -> Result<Str, Adt132> {
    let m2469 = f180(v0.clone(), v1);
    match m2469 {
        Adt135::C0 => {
            Err::<Str, Adt132>(Adt132::C7(v0))
        }
        Adt135::C1(v2) => {
            let m2470 = f234(v2);
            match m2470 {
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

pub fn f623(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt132> {
    let m2471 = f180(v0.clone(), v1);
    match m2471 {
        Adt135::C0 => {
            Ok::<Option<Adt5>, Adt132>(None::<Adt5>)
        }
        Adt135::C1(v2) => {
            let m2472 = f260(v2);
            match m2472 {
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

pub fn f624(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt132> {
    let m2473 = f180(v0.clone(), v1);
    match m2473 {
        Adt135::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt132>(None::<List<(Str, Str)>>)
        }
        Adt135::C1(v2) => {
            let m2474 = f260(v2);
            match m2474 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    let m2476 = f674(v0, match v3 {
                        Adt5::C0(h2475) => {
                            h2475
                        }
                    });
                    match m2476 {
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

pub fn f625(v0: Str, v1: Adt5) -> Result<Adt5, Adt132> {
    let m2477 = f180(v0.clone(), v1);
    match m2477 {
        Adt135::C0 => {
            Err::<Adt5, Adt132>(Adt132::C7(v0))
        }
        Adt135::C1(v2) => {
            let m2478 = f260(v2);
            match m2478 {
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

pub fn f626(v0: Adt5) -> Result<Adt26, Adt132> {
    let m2479 = f622(Str::lit("type"), v0.clone());
    match m2479 {
        Ok(v1) => {
            let a2480 = v1.clone();
            let a2481 = Str::lit("ref/prompt");
            if equal(a2480, a2481) {
                let m2482 = f622(Str::lit("name"), v0.clone());
                match m2482 {
                    Ok(v2) => {
                        let m2483 = f621(Str::lit("title"), v0);
                        match m2483 {
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
                let a2484 = v1;
                let a2485 = Str::lit("ref/resource");
                if equal(a2484, a2485) {
                    let m2486 = f622(Str::lit("uri"), v0);
                    match m2486 {
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

pub fn f627(v0: Adt5) -> Result<Adt28, Adt132> {
    let m2487 = f622(Str::lit("name"), v0.clone());
    match m2487 {
        Ok(v1) => {
            let m2488 = f622(Str::lit("value"), v0);
            match m2488 {
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

pub fn f628(v0: Adt5) -> Result<Option<Adt29>, Adt132> {
    let m2489 = f180(Str::lit("context"), v0);
    match m2489 {
        Adt135::C0 => {
            Ok::<Option<Adt29>, Adt132>(None::<Adt29>)
        }
        Adt135::C1(v1) => {
            let m2490 = f260(v1);
            match m2490 {
                None => {
                    Err::<Option<Adt29>, Adt132>(Adt132::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2491 = f624(Str::lit("arguments"), v2);
                    match m2491 {
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

pub fn f629(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt131> {
    if v0 {
        None::<Adt131>
    } else {
        Some(Adt131::C1(v1, v2))
    }
}

pub fn f630(v0: Str, p2496: List<Str>) -> bool {
    let m2495 = p2496;
    {
        let mut m2495 = m2495;
        loop {
            let step = {
                let v1 = m2495;
                let m2492 = v1;
                match m2492.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2493 = v2;
                        let a2494 = v0.clone();
                        if equal(a2493, a2494) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(v0: Option<Adt113>, v1: List<Adt115>, v2: Adt5) -> R<Option<Adt113>> {
    let m2497 = v0;
    match m2497 {
        None => {
            f675(v1, f676(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f632(v0: Option<Str>, v1: Adt145, v2: Adt5) -> Option<Adt113> {
    let m2498 = v1.clone();
    match m2498 {
        Adt145::C0 => {
            None::<Adt113>
        }
        Adt145::C1 => {
            f677(v0, f678(v1, v2))
        }
        Adt145::C2 => {
            f677(v0, f678(v1, v2))
        }
        Adt145::C3 => {
            f677(v0, f678(v1, v2))
        }
    }
}

pub fn f633(v0: Adt135, v1: Adt135) -> Adt154 {
    let m2499 = v0;
    match m2499 {
        Adt135::C0 => {
            Adt154::C1
        }
        Adt135::C1(_) => {
            let m2500 = v1;
            match m2500 {
                Adt135::C0 => {
                    Adt154::C2
                }
                Adt135::C1(_) => {
                    Adt154::C0
                }
            }
        }
    }
}

pub fn f634(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt17, Adt132> {
    let m2501 = f234(v1);
    match m2501 {
        None => {
            Err::<Adt17, Adt132>(Adt132::C5)
        }
        Some(v3) => {
            let m2502 = f679(v2);
            match m2502 {
                None => {
                    Err::<Adt17, Adt132>(Adt132::C6)
                }
                Some(v4) => {
                    let m2503 = f623(f680(), v0.clone());
                    match m2503 {
                        Ok(v6) => {
                            let m2504 = f681(v0.clone());
                            match m2504 {
                                Ok(v8) => {
                                    let m2505 = f682(v0.clone());
                                    match m2505 {
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

pub fn f635(v0: u64, v1: u64) -> u64 {
    let a2506 = v0;
    let a2507 = v1;
    let a2508 = 0u64;
    nat_rem(a2506, a2507, a2508)
}

pub fn f636(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2511 = {
            let a2509 = v0;
            let a2510 = 4u64;
            nat_mul(a2509, a2510)?
        };
        let a2512 = f638(v1, 16u64);
        nat_add(a2511, a2512)?
    }, List::<u64>::nil()))
}

pub fn f637(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2515 = {
            let a2513 = v0;
            let a2514 = 4u64;
            nat_mul(a2513, a2514)?
        };
        let a2516 = f638(v1, 16u64);
        nat_add(a2515, a2516)?
    }, List::cons({
        let a2519 = {
            let a2517 = f635(v1, 16u64);
            let a2518 = 16u64;
            nat_mul(a2517, a2518)?
        };
        let a2520 = f638(v2, 4u64);
        nat_add(a2519, a2520)?
    }, List::<u64>::nil())))
}

pub fn f638(v0: u64, v1: u64) -> u64 {
    let a2521 = v0;
    let a2522 = v1;
    let a2523 = 0u64;
    nat_quot(a2521, a2522, a2523)
}

pub fn f639(p2527: List<Adt6>) -> bool {
    let m2526 = p2527;
    {
        let mut m2526 = m2526;
        loop {
            let step = {
                let v0 = m2526;
                let m2524 = v0;
                match m2524.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2525 = f234(v1);
                        match m2525 {
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
                Ok(next) => { m2526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(p2531: List<Adt6>) -> bool {
    let m2530 = p2531;
    {
        let mut m2530 = m2530;
        loop {
            let step = {
                let v0 = m2530;
                let m2528 = v0;
                match m2528.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2529 = f234(v1);
                        match m2529 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f495(v3) {
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
                Ok(next) => { m2530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f641(v0: Str) -> Option<Adt13> {
    let a2532 = v0.clone();
    let a2533 = Str::lit("string");
    if equal(a2532, a2533) {
        Some(Adt13::C0)
    } else {
        let a2534 = v0.clone();
        let a2535 = Str::lit("integer");
        if equal(a2534, a2535) {
            Some(Adt13::C1)
        } else {
            let a2536 = v0;
            let a2537 = Str::lit("boolean");
            if equal(a2536, a2537) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f642(p2546: List<u64>, p2547: List<u64>) -> R<List<u64>> {
    let m2545 = (p2546, p2547);
    {
        let mut m2545 = m2545;
        loop {
            let step = {
                let (v0, v1) = m2545;
                let m2538 = v0;
                match m2538.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2539 = 65u64;
                        let a2540 = v2;
                        if if nat_le(a2539, a2540) {
                            let a2541 = v2;
                            let a2542 = 90u64;
                            nat_le(a2541, a2542)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2543 = v2;
                                let a2544 = 32u64;
                                nat_add(a2543, a2544)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(p2554: List<u64>, p2555: List<u64>) -> bool {
    let m2553 = (p2554, p2555);
    {
        let mut m2553 = m2553;
        loop {
            let step = {
                let (v0, v1) = m2553;
                let m2548 = v0;
                match m2548.uncons() {
                    None => {
                        let m2549 = v1;
                        match m2549.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2550 = v1;
                        match m2550.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2551 = v4;
                                let a2552 = v6;
                                if nat_eq(a2551, a2552) {
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
                Ok(next) => { m2553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(p2567: u64, p2568: i64, p2569: i64) -> R<bool> {
    let m2566 = (p2567, (p2568, p2569));
    {
        let mut m2566 = m2566;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2566;
                let m2556 = v0;
                if m2556 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2556 - 1;
                    let m2559 = {
                        let a2557 = v2;
                        let a2558 = 0i64;
                        compare(a2557, a2558)
                    };
                    if match m2559 {
                        Ordering::Less => {
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
                            let a2560 = v1;
                            let a2561 = 10i64;
                            let a2562 = 0i64;
                            int_rem(a2560, a2561, a2562)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2563 = v1;
                                let a2564 = 10i64;
                                let a2565 = 0i64;
                                int_quot(a2563, a2564, a2565)?
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
                Ok(next) => { m2566 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt155> {
    let a2570 = f683(v0.clone());
    let a2571 = f683(v1.clone());
    if nat_eq(a2570, a2571) {
        let m2572 = v0;
        match m2572 {
            Adt6::C0 => {
                Ok(Adt155::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2573 = f259(v1);
                match m2573 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v4) => {
                        let a2574 = v3;
                        let a2575 = v4;
                        if equal(a2574, a2575) {
                            Ok(Adt155::C1(v2))
                        } else {
                            Ok(Adt155::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2576 = f500(v1);
                match m2576 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v6) => {
                        if f684(f502(v5, v6)?) {
                            Ok(Adt155::C1(v2))
                        } else {
                            Ok(Adt155::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2577 = f234(v1);
                match m2577 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v8) => {
                        let a2578 = v7;
                        let a2579 = v8;
                        if equal(a2578, a2579) {
                            Ok(Adt155::C1(v2))
                        } else {
                            Ok(Adt155::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2580 = f333(v1);
                match m2580 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v10) => {
                        Ok(f685(f686(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2581 = f260(v1);
                match m2581 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v12) => {
                        let a2583 = f506(v11.clone(), 0u64)?;
                        let a2584 = f506(match v12.clone() {
                            Adt5::C0(h2582) => {
                                h2582
                            }
                        }, 0u64)?;
                        if nat_eq(a2583, a2584) {
                            Ok(f685(f687(v11, match v12 {
                                Adt5::C0(h2585) => {
                                    h2585
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt155::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt155::C0)
    }
}

pub fn f646(v0: Adt153, v1: Adt153) -> Adt153 {
    let m2586 = v0;
    match m2586 {
        Adt153::C0 => {
            f688(v1)
        }
        Adt153::C1 => {
            Adt153::C1
        }
        Adt153::C2 => {
            v1
        }
    }
}

pub fn f647(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt153> {
    let m2587 = f592(f689(v0, v1)?, f689(v2, v3)?);
    match m2587 {
        Adt153::C0 => {
            Ok(Adt153::C0)
        }
        Adt153::C1 => {
            f690(v0, f691(v0)?, v2, f691(v2)?)
        }
        Adt153::C2 => {
            Ok(Adt153::C2)
        }
    }
}

pub fn f648(v0: i64) -> R<i64> {
    let m2590 = {
        let a2588 = v0;
        let a2589 = 0i64;
        compare(a2588, a2589)
    };
    if match m2590 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2591 = v0;
        int_neg(a2591)
    } else {
        Ok(v0)
    }
}

pub fn f649(v0: Option<Adt37>) -> List<(Str, Adt6)> {
    let m2592 = v0;
    match m2592 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2604 = f430(Str::lit("title"), match v1.clone() {
                    Adt37::C0(h2593, _, _, _, _) => {
                        h2593
                    }
                });
                let a2605 = {
                    let a2602 = f435(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt37::C0(_, h2594, _, _, _) => {
                            h2594
                        }
                    });
                    let a2603 = {
                        let a2600 = f435(Str::lit("destructiveHint"), match v1.clone() {
                            Adt37::C0(_, _, h2595, _, _) => {
                                h2595
                            }
                        });
                        let a2601 = {
                            let a2598 = f435(Str::lit("idempotentHint"), match v1.clone() {
                                Adt37::C0(_, _, _, h2596, _) => {
                                    h2596
                                }
                            });
                            let a2599 = f435(Str::lit("openWorldHint"), match v1 {
                                Adt37::C0(_, _, _, _, h2597) => {
                                    h2597
                                }
                            });
                            append_list(a2598, a2599)
                        };
                        append_list(a2600, a2601)
                    };
                    append_list(a2602, a2603)
                };
                append_list(a2604, a2605)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f650(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f470(Str::lit("_meta"), v0)
}

pub fn f651(v0: Adt40) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt40::C0(h2606, _, _) => {
            h2606
        }
    })), {
        let a2609 = f655(match v0.clone() {
            Adt40::C0(_, h2607, _) => {
                h2607
            }
        });
        let a2610 = f650(match v0 {
            Adt40::C0(_, _, h2608) => {
                h2608
            }
        });
        append_list(a2609, a2610)
    })))
}

pub fn f652(v0: Str, v1: Str, v2: Str, v3: Option<Adt41>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2611 = f655(v3);
        let a2612 = f650(v4);
        append_list(a2611, a2612)
    }))))
}

pub fn f653(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2636 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2613, _, _, _, _, _, _, _, _) => {
                h2613
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2637 = {
            let a2634 = f430(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2614, _, _, _, _, _, _, _) => {
                    h2614
                }
            });
            let a2635 = {
                let a2632 = f532(match v0.clone() {
                    Adt45::C0(_, _, h2615, _, _, _, _, _, _) => {
                        h2615
                    }
                });
                let a2633 = {
                    let a2630 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2616, _, _, _, _, _) => {
                            h2616
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2631 = {
                        let a2628 = f430(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2617, _, _, _, _) => {
                                h2617
                            }
                        });
                        let a2629 = {
                            let a2626 = f430(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2618, _, _, _) => {
                                    h2618
                                }
                            });
                            let a2627 = {
                                let a2624 = f655(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2619, _, _) => {
                                        h2619
                                    }
                                });
                                let a2625 = {
                                    let a2622 = f656(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2620, _) => {
                                            h2620
                                        }
                                    })?;
                                    let a2623 = f650(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2621) => {
                                            h2621
                                        }
                                    });
                                    append_list(a2622, a2623)
                                };
                                append_list(a2624, a2625)
                            };
                            append_list(a2626, a2627)
                        };
                        append_list(a2628, a2629)
                    };
                    append_list(a2630, a2631)
                };
                append_list(a2632, a2633)
            };
            append_list(a2634, a2635)
        };
        append_list(a2636, a2637)
    }))
}

pub fn f654(v0: Adt46) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f600(match v0.clone() {
        Adt46::C0(h2638, _, _) => {
            h2638
        }
    })), {
        let a2641 = f655(match v0.clone() {
            Adt46::C0(_, h2639, _) => {
                h2639
            }
        });
        let a2642 = f650(match v0 {
            Adt46::C0(_, _, h2640) => {
                h2640
            }
        });
        append_list(a2641, a2642)
    })))
}

pub fn f655(v0: Option<Adt41>) -> List<(Str, Adt6)> {
    let m2643 = v0;
    match m2643 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2649 = f692(match v1.clone() {
                    Adt41::C0(h2644, _, _) => {
                        h2644
                    }
                });
                let a2650 = {
                    let a2647 = f693(match v1.clone() {
                        Adt41::C0(_, h2645, _) => {
                            h2645
                        }
                    });
                    let a2648 = f430(Str::lit("lastModified"), match v1 {
                        Adt41::C0(_, _, h2646) => {
                            h2646
                        }
                    });
                    append_list(a2647, a2648)
                };
                append_list(a2649, a2650)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f656(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2651 = v0;
    match m2651 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f520(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f657(v0: List<Adt58>) -> List<Adt6> {
    f310(f694(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f658(v0: Adt42) -> Str {
    let m2652 = v0;
    match m2652 {
        Adt42::C0 => {
            Str::lit("user")
        }
        Adt42::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f659(v0: Adt66) -> List<(Str, Adt6)> {
    let m2653 = v0;
    match m2653 {
        Adt66::C0(v1) => {
            let a2660 = {
                let m2655 = match v1.clone() {
                    Adt67::C0(h2654, _, _) => {
                        h2654
                    }
                };
                match m2655 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2661 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt67::C0(_, h2656, _) => {
                    h2656
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m2659 = match v1 {
                    Adt67::C0(_, _, h2658) => {
                        h2658
                    }
                };
                match m2659 {
                    Adt5::C0(h2657) => {
                        h2657
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a2660, a2661)
        }
        Adt66::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt69::C0(h2662, _) => {
                    h2662
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt69::C0(_, h2663) => {
                    h2663
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f660(p2666: List<Adt2>, p2667: List<Adt6>) -> List<Adt6> {
    let m2665 = (p2666, p2667);
    {
        let mut m2665 = m2665;
        loop {
            let step = {
                let (v0, v1) = m2665;
                let m2664 = v0;
                match m2664.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f695(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f661(v0: Adt5) -> Option<Adt64> {
    let m2668 = f662(Str::lit("taskId"), v0.clone());
    match m2668 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m2669 = f662(Str::lit("createdAt"), v0.clone());
            match m2669 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let m2670 = f662(Str::lit("lastUpdatedAt"), v0.clone());
                    match m2670 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v3) => {
                            Some(Adt64::C0(v1, f662(Str::lit("statusMessage"), v0.clone()), v2, v3, f609(Str::lit("ttlMs"), v0.clone()), f609(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f662(v0: Str, v1: Adt5) -> Option<Str> {
    let m2671 = f180(v0, v1);
    match m2671 {
        Adt135::C0 => {
            None::<Str>
        }
        Adt135::C1(v2) => {
            f234(v2)
        }
    }
}

pub fn f663(v0: Str, v1: Adt5) -> Option<List<(Str, Adt65)>> {
    let m2672 = f607(v0, v1);
    match m2672 {
        None => {
            None::<List<(Str, Adt65)>>
        }
        Some(v2) => {
            f696(match v2 {
                Adt5::C0(h2673) => {
                    h2673
                }
            }, List::<(Str, Adt65)>::nil())
        }
    }
}

pub fn f664(v0: Adt5) -> Option<Adt81> {
    let m2674 = f609(Str::lit("code"), v0.clone());
    match m2674 {
        None => {
            None::<Adt81>
        }
        Some(v1) => {
            let m2675 = f662(Str::lit("message"), v0.clone());
            match m2675 {
                None => {
                    None::<Adt81>
                }
                Some(v2) => {
                    Some(Adt81::C0(v1, v2, {
                        let m2676 = f180(Str::lit("data"), v0);
                        match m2676 {
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

pub fn f665(v0: Adt5) -> Option<Adt87> {
    let m2677 = f607(Str::lit("completes"), v0.clone());
    match m2677 {
        None => {
            let m2678 = f607(Str::lit("fails"), v0);
            match m2678 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m2679 = f664(v2);
                    match m2679 {
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

pub fn f666(v0: Adt86) -> Option<Adt6> {
    let m2681 = f442(match v0.clone() {
        Adt86::C0(_, h2680, _, _) => {
            h2680
        }
    });
    match m2681 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f605(match v0.clone() {
                Adt86::C0(h2682, _, _, _) => {
                    h2682
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f605(match v0.clone() {
                Adt86::C0(_, _, h2683, _) => {
                    h2683
                }
            })), List::cons((Str::lit("outcome"), f697(match v0 {
                Adt86::C0(_, _, _, h2684) => {
                    h2684
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f667() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f668() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f669(p2689: List<u64>, p2690: u64, p2691: List<u64>) -> List<u64> {
    let m2688 = (p2689, (p2690, p2691));
    {
        let mut m2688 = m2688;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2688;
                let m2685 = v0;
                match m2685.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f698(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt156 = f699(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt156::C0(h2686, _) => {
                                h2686
                            }
                        }, match v5 {
                            Adt156::C0(_, h2687) => {
                                h2687
                            }
                        })))
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

pub fn f670(v0: Adt140, v1: Adt33) -> bool {
    let m2692 = v1;
    match m2692 {
        Adt33::C0(_) => {
            f700(v0, Adt140::C0)
        }
        Adt33::C1(_) => {
            f700(v0, Adt140::C1)
        }
        Adt33::C2(_) => {
            f700(v0, Adt140::C2)
        }
        Adt33::C3(_) => {
            f700(v0, Adt140::C2)
        }
        Adt33::C4(_) => {
            f700(v0, Adt140::C3)
        }
        Adt33::C5(_) => {
            f700(v0, Adt140::C4)
        }
        Adt33::C6(_) => {
            f700(v0, Adt140::C5)
        }
        Adt33::C7(_) => {
            f700(v0, Adt140::C6)
        }
        Adt33::C8(_) => {
            f700(v0, Adt140::C7)
        }
        Adt33::C9(_) => {
            f700(v0, Adt140::C8)
        }
        Adt33::C10(_) => {
            f700(v0, Adt140::C9)
        }
        Adt33::C11 => {
            if f700(v0.clone(), Adt140::C10) {
                true
            } else {
                f700(v0, Adt140::C11)
            }
        }
        Adt33::C12(_) => {
            f701(v0)
        }
        Adt33::C13(_) => {
            f702(v0)
        }
        Adt33::C14(v15) => {
            let m2694 = match v15 {
                Adt88::C0(_, h2693, _) => {
                    h2693
                }
            };
            match m2694 {
                Adt89::C0 => {
                    f700(v0, Adt140::C9)
                }
                Adt89::C1 => {
                    if f700(v0.clone(), Adt140::C10) {
                        true
                    } else {
                        f700(v0, Adt140::C11)
                    }
                }
            }
        }
    }
}

pub fn f671(v0: Adt0, v1: Adt15, v2: Adt33) -> Result<Adt128, Adt131> {
    let m2695 = v2.clone();
    match m2695 {
        Adt33::C0(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C1(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C2(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C3(v9) => {
            Ok::<Adt128, Adt131>(f707(v0, f704(v1), v9))
        }
        Adt33::C4(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C5(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C6(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C7(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C8(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C9(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C10(_) => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C11 => {
            Ok::<Adt128, Adt131>(f617(v0, v2))
        }
        Adt33::C12(v3) => {
            f703(v0, f704(v1), v3)
        }
        Adt33::C13(v4) => {
            f705(v0, f704(v1), v4)
        }
        Adt33::C14(v5) => {
            f706(v0, f704(v1), v5)
        }
    }
}

pub fn f672() -> Adt90 {
    Adt90::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f673(v0: u64, v1: u64) -> bool {
    let a2696 = v1;
    let a2697 = v0;
    nat_lt(a2696, a2697)
}

pub fn f674(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt132> {
    let m2698 = f708(v0, v1, List::<(Str, Str)>::nil());
    match m2698 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt132>(f709(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt132>(v3)
        }
    }
}

pub fn f675(p2702: List<Adt115>, v1: Adt5) -> R<Option<Adt113>> {
    let m2701 = p2702;
    {
        let mut m2701 = m2701;
        loop {
            let step = {
                let v0 = m2701;
                let m2699 = v0;
                match m2699.uncons() {
                    None => {
                        Err::<List<Adt115>, R<Option<Adt113>>>(Ok(None::<Adt113>))
                    }
                    Some((v2, v3)) => {
                        let m2700 = f710(v2, v1.clone())?;
                        match m2700 {
                            None => {
                                Ok::<List<Adt115>, R<Option<Adt113>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt115>, R<Option<Adt113>>>(Ok(Some(v4)))
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

pub fn f676(v0: Adt5) -> Adt5 {
    let m2703 = f180(Str::lit("arguments"), v0);
    match m2703 {
        Adt135::C0 => {
            f560()
        }
        Adt135::C1(v1) => {
            let m2704 = f260(v1);
            match m2704 {
                None => {
                    f560()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f677(v0: Option<Str>, v1: Option<Str>) -> Option<Adt113> {
    let m2705 = v0;
    match m2705 {
        None => {
            Some(Adt113::C2)
        }
        Some(v2) => {
            let m2706 = v1;
            match m2706 {
                None => {
                    None::<Adt113>
                }
                Some(v3) => {
                    let a2707 = v2;
                    let a2708 = v3;
                    if equal(a2707, a2708) {
                        None::<Adt113>
                    } else {
                        Some(Adt113::C5)
                    }
                }
            }
        }
    }
}

pub fn f678(v0: Adt145, v1: Adt5) -> Option<Str> {
    let m2709 = v0;
    match m2709 {
        Adt145::C0 => {
            None::<Str>
        }
        Adt145::C1 => {
            let m2710 = f180(Str::lit("name"), v1);
            match m2710 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v2) => {
                    f234(v2)
                }
            }
        }
        Adt145::C2 => {
            let m2711 = f180(Str::lit("uri"), v1);
            match m2711 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v3) => {
                    f234(v3)
                }
            }
        }
        Adt145::C3 => {
            let m2712 = f180(Str::lit("taskId"), v1);
            match m2712 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v4) => {
                    f234(v4)
                }
            }
        }
    }
}

pub fn f679(v0: Adt6) -> Option<Adt18> {
    let m2713 = f260(v0);
    match m2713 {
        None => {
            None::<Adt18>
        }
        Some(v1) => {
            let m2714 = f711(Str::lit("experimental"), v1.clone());
            match m2714 {
                None => {
                    None::<Adt18>
                }
                Some(v2) => {
                    let m2715 = f711(Str::lit("roots"), v1.clone());
                    match m2715 {
                        None => {
                            None::<Adt18>
                        }
                        Some(v3) => {
                            let m2716 = f711(Str::lit("sampling"), v1.clone());
                            match m2716 {
                                None => {
                                    None::<Adt18>
                                }
                                Some(v4) => {
                                    let m2717 = f711(Str::lit("elicitation"), v1.clone());
                                    match m2717 {
                                        None => {
                                            None::<Adt18>
                                        }
                                        Some(v5) => {
                                            let m2718 = f711(Str::lit("extensions"), v1);
                                            match m2718 {
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

pub fn f680() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f681(v0: Adt5) -> Result<Option<Adt19>, Adt132> {
    let m2719 = f621(f712(), v0);
    match m2719 {
        Ok(v2) => {
            let m2720 = v2;
            match m2720 {
                None => {
                    Ok::<Option<Adt19>, Adt132>(None::<Adt19>)
                }
                Some(v3) => {
                    let m2721 = f713(v3);
                    match m2721 {
                        Adt157::C0 => {
                            Err::<Option<Adt19>, Adt132>(Adt132::C8(f712()))
                        }
                        Adt157::C1(v4) => {
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

pub fn f682(v0: Adt5) -> Result<Option<Adt20>, Adt132> {
    let m2722 = f180(f714(), v0);
    match m2722 {
        Adt135::C0 => {
            Ok::<Option<Adt20>, Adt132>(None::<Adt20>)
        }
        Adt135::C1(v1) => {
            let m2723 = f234(v1.clone());
            match m2723 {
                None => {
                    let m2724 = f591(v1);
                    match m2724 {
                        None => {
                            Err::<Option<Adt20>, Adt132>(Adt132::C8(f714()))
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

pub fn f683(v0: Adt6) -> u64 {
    let m2725 = v0;
    match m2725 {
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

pub fn f684(v0: Adt153) -> bool {
    let m2726 = v0;
    match m2726 {
        Adt153::C0 => {
            false
        }
        Adt153::C1 => {
            true
        }
        Adt153::C2 => {
            false
        }
    }
}

pub fn f685(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt155 {
    let m2727 = v0;
    match m2727 {
        None => {
            Adt155::C0
        }
        Some(v2) => {
            Adt155::C1({
                let a2728 = v2;
                let a2729 = v1;
                append_list(a2728, a2729)
            })
        }
    }
}

pub fn f686(p2734: List<Adt6>, p2735: List<Adt6>, p2736: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2733 = (p2734, (p2735, p2736));
    {
        let mut m2733 = m2733;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2733;
                let m2730 = v0;
                match m2730.uncons() {
                    None => {
                        let m2731 = v1;
                        match m2731.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2732 = v1;
                        match m2732.uncons() {
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
                Ok(next) => { m2733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f687(p2742: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2743: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2741 = (p2742, p2743);
    {
        let mut m2741 = m2741;
        loop {
            let step = {
                let (v0, v2) = m2741;
                let m2737 = v0;
                match m2737.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2739 = f237({
                            let (h2738, _) = v3;
                            h2738
                        }, v1.clone());
                        match m2739 {
                            Adt135::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt135::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2740) = v3;
                                    h2740
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f688(v0: Adt153) -> Adt153 {
    let m2744 = v0;
    match m2744 {
        Adt153::C0 => {
            Adt153::C2
        }
        Adt153::C1 => {
            Adt153::C1
        }
        Adt153::C2 => {
            Adt153::C0
        }
    }
}

pub fn f689(v0: i64, v1: i64) -> R<i64> {
    f115(f79(f691(v0)?, 0i64)?, v1)
}

pub fn f690(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt153> {
    let a2745 = v1;
    let a2746 = v3;
    if nat_lt(a2745, a2746) {
        Ok(f592(f715({
            let a2747 = v3;
            let a2748 = v1;
            nat_sub(a2747, a2748)
        }, v0)?, v2))
    } else {
        let a2749 = v3;
        let a2750 = v1;
        if nat_lt(a2749, a2750) {
            Ok(f592(v0, f715({
                let a2751 = v1;
                let a2752 = v3;
                nat_sub(a2751, a2752)
            }, v2)?))
        } else {
            Ok(f592(v0, v2))
        }
    }
}

pub fn f691(v0: i64) -> R<u64> {
    f716(20u64, v0, 0u64)
}

pub fn f692(v0: Option<List<Adt42>>) -> List<(Str, Adt6)> {
    let m2753 = v0;
    match m2753 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f717(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f693(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2754 = v0;
    match m2754 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f694(p2757: List<Adt58>, p2758: List<Adt6>) -> List<Adt6> {
    let m2756 = (p2757, p2758);
    {
        let mut m2756 = m2756;
        loop {
            let step = {
                let (v0, v1) = m2756;
                let m2755 = v0;
                match m2755.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f718(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f695(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2759, _, _, _) => {
            h2759
        }
    })), {
        let a2765 = f430(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2760, _, _) => {
                h2760
            }
        });
        let a2766 = {
            let a2763 = f719(match v0.clone() {
                Adt2::C0(_, _, h2761, _) => {
                    h2761
                }
            });
            let a2764 = f720(match v0 {
                Adt2::C0(_, _, _, h2762) => {
                    h2762
                }
            });
            append_list(a2763, a2764)
        };
        append_list(a2765, a2766)
    }))
}

pub fn f696(p2772: List<(Str, Adt6)>, p2773: List<(Str, Adt65)>) -> Option<List<(Str, Adt65)>> {
    let m2771 = (p2772, p2773);
    {
        let mut m2771 = m2771;
        loop {
            let step = {
                let (v0, v1) = m2771;
                let m2767 = v0;
                match m2767.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(Some(f721(v1, List::<(Str, Adt65)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2769 = f722({
                            let (_, h2768) = v2;
                            h2768
                        });
                        match m2769 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(None::<List<(Str, Adt65)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>((v3, List::cons(({
                                    let (h2770, _) = v2;
                                    h2770
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f697(v0: Adt87) -> Adt6 {
    let m2774 = v0;
    match m2774 {
        Adt87::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h2775) => {
                    h2775
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt87::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f129(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f698(v0: u64, v1: List<u64>) -> List<u64> {
    let a2776 = v0;
    let a2777 = 0u64;
    if nat_eq(a2776, a2777) {
        v1
    } else {
        let a2778 = v0;
        let a2779 = 1u64;
        if nat_eq(a2778, a2779) {
            List::cons(194u64, v1)
        } else {
            let a2780 = v0;
            let a2781 = 2u64;
            if nat_eq(a2780, a2781) {
                List::cons(226u64, v1)
            } else {
                let a2782 = v0;
                let a2783 = 3u64;
                if nat_eq(a2782, a2783) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f699(v0: u64, v1: u64, v2: List<u64>) -> Adt156 {
    let a2784 = v0;
    let a2785 = 0u64;
    if nat_eq(a2784, a2785) {
        f723(v1, v2)
    } else {
        let a2786 = v0;
        let a2787 = 1u64;
        if nat_eq(a2786, a2787) {
            let a2788 = 128u64;
            let a2789 = v1;
            if if nat_le(a2788, a2789) {
                let a2790 = v1;
                let a2791 = 159u64;
                nat_le(a2790, a2791)
            } else {
                false
            } {
                Adt156::C0(0u64, f724(v2))
            } else {
                f723(v1, List::cons(194u64, v2))
            }
        } else {
            let a2792 = v0;
            let a2793 = 2u64;
            if nat_eq(a2792, a2793) {
                let a2794 = v1;
                let a2795 = 128u64;
                if nat_eq(a2794, a2795) {
                    Adt156::C0(3u64, v2)
                } else {
                    let a2796 = v1;
                    let a2797 = 129u64;
                    if nat_eq(a2796, a2797) {
                        Adt156::C0(4u64, v2)
                    } else {
                        f723(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2798 = v0;
                let a2799 = 3u64;
                if nat_eq(a2798, a2799) {
                    let a2800 = 170u64;
                    let a2801 = v1;
                    if if nat_le(a2800, a2801) {
                        let a2802 = v1;
                        let a2803 = 174u64;
                        nat_le(a2802, a2803)
                    } else {
                        false
                    } {
                        Adt156::C0(0u64, f724(v2))
                    } else {
                        f723(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2804 = 166u64;
                    let a2805 = v1;
                    if if nat_le(a2804, a2805) {
                        let a2806 = v1;
                        let a2807 = 169u64;
                        nat_le(a2806, a2807)
                    } else {
                        false
                    } {
                        Adt156::C0(0u64, f724(v2))
                    } else {
                        f723(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f700(v0: Adt140, v1: Adt140) -> bool {
    let a2808 = f474(v0);
    let a2809 = f474(v1);
    equal(a2808, a2809)
}

pub fn f701(v0: Adt140) -> bool {
    let m2810 = v0;
    match m2810 {
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

pub fn f702(v0: Adt140) -> bool {
    let m2811 = v0;
    match m2811 {
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

pub fn f703(v0: Adt0, v1: Adt17, v2: Adt82) -> Result<Adt128, Adt131> {
    f725(v0, v2.clone(), f726(match v1 {
        Adt17::C0(_, h2812, _, _, _, _) => {
            h2812
        }
    }, f443(v2)))
}

pub fn f704(v0: Adt15) -> Adt17 {
    let m2813 = v0;
    match m2813 {
        Adt15::C0(v1) => {
            match v1 {
                Adt16::C0(h2814) => {
                    h2814
                }
            }
        }
        Adt15::C1(v2) => {
            match v2 {
                Adt21::C0(h2815, _) => {
                    h2815
                }
            }
        }
        Adt15::C2(v3) => {
            match v3 {
                Adt22::C0(h2816, _, _, _, _) => {
                    h2816
                }
            }
        }
        Adt15::C3(v4) => {
            match v4 {
                Adt21::C0(h2817, _) => {
                    h2817
                }
            }
        }
        Adt15::C4(v5) => {
            match v5 {
                Adt23::C0(h2818, _, _, _) => {
                    h2818
                }
            }
        }
        Adt15::C5(v6) => {
            match v6 {
                Adt21::C0(h2819, _) => {
                    h2819
                }
            }
        }
        Adt15::C6(v7) => {
            match v7 {
                Adt21::C0(h2820, _) => {
                    h2820
                }
            }
        }
        Adt15::C7(v8) => {
            match v8 {
                Adt24::C0(h2821, _, _, _, _) => {
                    h2821
                }
            }
        }
        Adt15::C8(v9) => {
            match v9 {
                Adt25::C0(h2822, _, _, _) => {
                    h2822
                }
            }
        }
        Adt15::C9(v10) => {
            match v10 {
                Adt30::C0(h2823, _) => {
                    h2823
                }
            }
        }
        Adt15::C10(v11) => {
            match v11 {
                Adt31::C0(h2824, _, _) => {
                    h2824
                }
            }
        }
        Adt15::C11(v12) => {
            match v12 {
                Adt32::C0(h2825, _) => {
                    h2825
                }
            }
        }
    }
}

pub fn f705(v0: Adt0, v1: Adt17, v2: Adt83) -> Result<Adt128, Adt131> {
    f727(v0, v2, f728(match v1 {
        Adt17::C0(_, h2826, _, _, _, _) => {
            h2826
        }
    }))
}

pub fn f706(v0: Adt0, v1: Adt17, v2: Adt88) -> Result<Adt128, Adt131> {
    f729(v0, v2, f728(match v1 {
        Adt17::C0(_, h2827, _, _, _, _) => {
            h2827
        }
    }))
}

pub fn f707(v0: Adt0, v1: Adt17, v2: Adt50) -> Adt128 {
    Adt128::C0(Adt33::C2(match v2.clone() {
        Adt50::C0(h2828, _) => {
            h2828
        }
    }), {
        let m2830 = f617(v0, Adt33::C11);
        match m2830 {
            Adt128::C0(_, h2829, _) => {
                h2829
            }
        }
    }, f730(v1, match v2 {
        Adt50::C0(_, h2831) => {
            h2831
        }
    }))
}

pub fn f708(v0: Str, p2837: List<(Str, Adt6)>, p2838: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt132> {
    let m2836 = (p2837, p2838);
    {
        let mut m2836 = m2836;
        loop {
            let step = {
                let (v1, v2) = m2836;
                let m2832 = v1;
                match m2832.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Ok::<List<(Str, Str)>, Adt132>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2834 = f234({
                            let (_, h2833) = v3;
                            h2833
                        });
                        match m2834 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Err::<List<(Str, Str)>, Adt132>(Adt132::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>((v4, List::cons(({
                                    let (h2835, _) = v3;
                                    h2835
                                }, v5), v2)))
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

pub fn f709(p2841: List<(Str, Str)>, p2842: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2840 = (p2841, p2842);
    {
        let mut m2840 = m2840;
        loop {
            let step = {
                let (v0, v1) = m2840;
                let m2839 = v0;
                match m2839.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f710(v0: Adt115, v1: Adt5) -> R<Option<Adt113>> {
    let m2844 = f731(match v0.clone() {
        Adt115::C0(_, h2843, _, _) => {
            h2843
        }
    }, v1);
    match m2844 {
        Adt135::C0 => {
            Ok(f732(v0))
        }
        Adt135::C1(v2) => {
            if f733(v2.clone()) {
                Ok(f732(v0))
            } else {
                f734(v0, v2)
            }
        }
    }
}

pub fn f711(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2845 = f180(v0, v1);
    match m2845 {
        Adt135::C0 => {
            Some(None::<Adt5>)
        }
        Adt135::C1(v2) => {
            let m2846 = f260(v2);
            match m2846 {
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

pub fn f712() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f713(v0: Str) -> Adt157 {
    let a2847 = v0.clone();
    let a2848 = Str::lit("debug");
    if equal(a2847, a2848) {
        Adt157::C1(Adt19::C0)
    } else {
        let a2849 = v0.clone();
        let a2850 = Str::lit("info");
        if equal(a2849, a2850) {
            Adt157::C1(Adt19::C1)
        } else {
            let a2851 = v0.clone();
            let a2852 = Str::lit("notice");
            if equal(a2851, a2852) {
                Adt157::C1(Adt19::C2)
            } else {
                let a2853 = v0.clone();
                let a2854 = Str::lit("warning");
                if equal(a2853, a2854) {
                    Adt157::C1(Adt19::C3)
                } else {
                    let a2855 = v0.clone();
                    let a2856 = Str::lit("error");
                    if equal(a2855, a2856) {
                        Adt157::C1(Adt19::C4)
                    } else {
                        let a2857 = v0.clone();
                        let a2858 = Str::lit("critical");
                        if equal(a2857, a2858) {
                            Adt157::C1(Adt19::C5)
                        } else {
                            let a2859 = v0.clone();
                            let a2860 = Str::lit("alert");
                            if equal(a2859, a2860) {
                                Adt157::C1(Adt19::C6)
                            } else {
                                let a2861 = v0;
                                let a2862 = Str::lit("emergency");
                                if equal(a2861, a2862) {
                                    Adt157::C1(Adt19::C7)
                                } else {
                                    Adt157::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f714() -> Str {
    Str::lit("progressToken")
}

pub fn f715(p2867: u64, p2868: i64) -> R<i64> {
    let m2866 = (p2867, p2868);
    {
        let mut m2866 = m2866;
        loop {
            let step = {
                let (v0, v1) = m2866;
                let m2863 = v0;
                if m2863 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2863 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2864 = v1;
                        let a2865 = 10i64;
                        int_mul(a2864, a2865)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2866 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716(p2879: u64, p2880: i64, p2881: u64) -> R<u64> {
    let m2878 = (p2879, (p2880, p2881));
    {
        let mut m2878 = m2878;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2878;
                let m2869 = v0;
                if m2869 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2869 - 1;
                    let v4: u64 = {
                        let a2870 = v2;
                        let a2871 = 1u64;
                        nat_add(a2870, a2871)?
                    };
                    let m2874 = {
                        let a2872 = v1;
                        let a2873 = 10i64;
                        compare(a2872, a2873)
                    };
                    if match m2874 {
                        Ordering::Less => {
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
                            let a2875 = v1;
                            let a2876 = 10i64;
                            let a2877 = 0i64;
                            int_quot(a2875, a2876, a2877)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2878 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f717(v0: List<Adt42>) -> List<Adt6> {
    f310(f735(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f718(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a2890 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h2882, _, _, _) => {
                h2882
            }
        })), List::<(Str, Adt6)>::nil());
        let a2891 = {
            let a2888 = f430(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2883, _, _) => {
                    h2883
                }
            });
            let a2889 = {
                let a2886 = f430(Str::lit("description"), match v0.clone() {
                    Adt58::C0(_, _, h2884, _) => {
                        h2884
                    }
                });
                let a2887 = f435(Str::lit("required"), match v0 {
                    Adt58::C0(_, _, _, h2885) => {
                        h2885
                    }
                });
                append_list(a2886, a2887)
            };
            append_list(a2888, a2889)
        };
        append_list(a2890, a2891)
    })
}

pub fn f719(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2892 = v0;
    match m2892 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f378(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f720(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2893 = v0;
    match m2893 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f736(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f721(p2896: List<(Str, Adt65)>, p2897: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m2895 = (p2896, p2897);
    {
        let mut m2895 = m2895;
        loop {
            let step = {
                let (v0, v1) = m2895;
                let m2894 = v0;
                match m2894.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2895 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f722(v0: Adt6) -> Option<Adt65> {
    let m2898 = f260(v0);
    match m2898 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2899 = f662(Str::lit("method"), v1.clone());
            match m2899 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a2900 = v2.clone();
                    let a2901 = Str::lit("elicitation/create");
                    if equal(a2900, a2901) {
                        let m2902 = f607(Str::lit("params"), v1);
                        match m2902 {
                            None => {
                                None::<Adt65>
                            }
                            Some(v3) => {
                                f737(v3)
                            }
                        }
                    } else {
                        let a2903 = v2;
                        let a2904 = Str::lit("roots/list");
                        if equal(a2903, a2904) {
                            f738(v1)
                        } else {
                            None::<Adt65>
                        }
                    }
                }
            }
        }
    }
}

pub fn f723(v0: u64, v1: List<u64>) -> Adt156 {
    let a2905 = v0;
    let a2906 = 32u64;
    if nat_lt(a2905, a2906) {
        if f739(v0) {
            Adt156::C0(0u64, List::cons(v0, v1))
        } else {
            Adt156::C0(0u64, f724(v1))
        }
    } else {
        let a2907 = v0;
        let a2908 = 127u64;
        if nat_eq(a2907, a2908) {
            Adt156::C0(0u64, f724(v1))
        } else {
            let a2909 = v0;
            let a2910 = 194u64;
            if nat_eq(a2909, a2910) {
                Adt156::C0(1u64, v1)
            } else {
                let a2911 = v0;
                let a2912 = 226u64;
                if nat_eq(a2911, a2912) {
                    Adt156::C0(2u64, v1)
                } else {
                    Adt156::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f724(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f725(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f617(v0, Adt33::C12(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f740(f443(v1)))))
    }
}

pub fn f726(v0: Adt18, v1: List<(Str, Adt65)>) -> bool {
    let a2913 = f741(v1.clone());
    if if if bool_not(a2913) {
        true
    } else {
        f477(match v0.clone() {
            Adt18::C0(_, _, _, h2914, _) => {
                h2914
            }
        })
    } {
        let a2915 = f742(v1.clone());
        if bool_not(a2915) {
            true
        } else {
            f477(match v0.clone() {
                Adt18::C0(_, _, h2916, _, _) => {
                    h2916
                }
            })
        }
    } else {
        false
    } {
        let a2917 = f743(v1);
        if bool_not(a2917) {
            true
        } else {
            f477(match v0 {
                Adt18::C0(_, h2918, _, _, _) => {
                    h2918
                }
            })
        }
    } else {
        false
    }
}

pub fn f727(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f617(v0, Adt33::C13(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f744())))
    }
}

pub fn f728(v0: Adt18) -> bool {
    f478(match v0 {
        Adt18::C0(_, _, _, _, h2919) => {
            h2919
        }
    }, f26())
}

pub fn f729(v0: Adt0, v1: Adt88, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f617(v0, Adt33::C14(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f744())))
    }
}

pub fn f730(v0: Adt17, v1: List<Adt51>) -> List<Adt130> {
    let m2921 = match v0 {
        Adt17::C0(_, _, _, _, h2920, _) => {
            h2920
        }
    };
    match m2921 {
        None => {
            List::<Adt130>::nil()
        }
        Some(v2) => {
            f745(f746(v2, v1, List::<Adt130>::nil()), List::<Adt130>::nil())
        }
    }
}

pub fn f731(p2927: List<Str>, p2928: Adt5) -> Adt135 {
    let m2926 = (p2927, p2928);
    {
        let mut m2926 = m2926;
        loop {
            let step = {
                let (v0, v1) = m2926;
                let m2922 = v0;
                match m2922.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                    }
                    Some((v2, v3)) => {
                        let m2923 = f180(v2, v1);
                        match m2923 {
                            Adt135::C0 => {
                                Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                            }
                            Adt135::C1(v4) => {
                                let m2924 = v3.clone();
                                match m2924.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt135>(Adt135::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2925 = f260(v4);
                                        match m2925 {
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
                Ok(next) => { m2926 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f732(v0: Adt115) -> Option<Adt113> {
    let m2930 = match v0.clone() {
        Adt115::C0(_, _, _, h2929) => {
            h2929
        }
    };
    match m2930 {
        Adt116::C0 => {
            None::<Adt113>
        }
        Adt116::C1 => {
            Some(Adt113::C10(match v0 {
                Adt115::C0(h2931, _, _, _) => {
                    h2931
                }
            }))
        }
        Adt116::C2(_) => {
            None::<Adt113>
        }
    }
}

pub fn f733(v0: Adt6) -> bool {
    let m2932 = v0;
    match m2932 {
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

pub fn f734(v0: Adt115, v1: Adt6) -> R<Option<Adt113>> {
    let m2934 = match v0.clone() {
        Adt115::C0(_, _, _, h2933) => {
            h2933
        }
    };
    match m2934 {
        Adt116::C0 => {
            Ok(Some(Adt113::C8(match v0 {
                Adt115::C0(h2935, _, _, _) => {
                    h2935
                }
            })))
        }
        Adt116::C1 => {
            Ok(Some(Adt113::C10(match v0 {
                Adt115::C0(h2936, _, _, _) => {
                    h2936
                }
            })))
        }
        Adt116::C2(v2) => {
            if f747(match v0.clone() {
                Adt115::C0(_, _, h2937, _) => {
                    h2937
                }
            }, v2, v1)? {
                Ok(None::<Adt113>)
            } else {
                Ok(Some(Adt113::C9(match v0 {
                    Adt115::C0(h2938, _, _, _) => {
                        h2938
                    }
                })))
            }
        }
    }
}

pub fn f735(p2941: List<Adt42>, p2942: List<Adt6>) -> List<Adt6> {
    let m2940 = (p2941, p2942);
    {
        let mut m2940 = m2940;
        loop {
            let step = {
                let (v0, v1) = m2940;
                let m2939 = v0;
                match m2939.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt42>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f658(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f736(v0: Adt3) -> Str {
    let m2943 = v0;
    match m2943 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f737(v0: Adt5) -> Option<Adt65> {
    let m2944 = f662(Str::lit("message"), v0.clone());
    match m2944 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2945 = f180(Str::lit("mode"), v0.clone());
            match m2945 {
                Adt135::C0 => {
                    let m2946 = f607(Str::lit("requestedSchema"), v0);
                    match m2946 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v2) => {
                            Some(Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v1, v2))))
                        }
                    }
                }
                Adt135::C1(v3) => {
                    let m2947 = f234(v3);
                    match m2947 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v4) => {
                            let a2948 = v4.clone();
                            let a2949 = Str::lit("url");
                            if equal(a2948, a2949) {
                                let m2950 = f662(Str::lit("url"), v0);
                                match m2950 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v5) => {
                                        Some(Adt65::C0(Adt66::C1(Adt69::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a2951 = v4;
                                let a2952 = Str::lit("form");
                                if equal(a2951, a2952) {
                                    let m2953 = f607(Str::lit("requestedSchema"), v0);
                                    match m2953 {
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

pub fn f738(v0: Adt5) -> Option<Adt65> {
    let m2954 = f180(Str::lit("params"), v0);
    match m2954 {
        Adt135::C0 => {
            Some(Adt65::C2(None::<Adt5>))
        }
        Adt135::C1(v1) => {
            let m2955 = f260(v1);
            match m2955 {
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

pub fn f739(v0: u64) -> bool {
    let a2956 = v0;
    let a2957 = 9u64;
    if nat_eq(a2956, a2957) {
        true
    } else {
        let a2958 = v0;
        let a2959 = 10u64;
        if nat_eq(a2958, a2959) {
            true
        } else {
            let a2960 = v0;
            let a2961 = 13u64;
            nat_eq(a2960, a2961)
        }
    }
}

pub fn f740(v0: List<(Str, Adt65)>) -> Adt18 {
    Adt18::C0(None::<Adt5>, if f743(v0.clone()) {
        Some(f560())
    } else {
        None::<Adt5>
    }, if f742(v0.clone()) {
        Some(f560())
    } else {
        None::<Adt5>
    }, if f741(v0) {
        Some(f560())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f741(p2966: List<(Str, Adt65)>) -> bool {
    let m2965 = p2966;
    {
        let mut m2965 = m2965;
        loop {
            let step = {
                let v0 = m2965;
                let m2962 = v0;
                match m2962.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2964 = {
                            let (_, h2963) = v1;
                            h2963
                        };
                        match m2964 {
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
                Ok(next) => { m2965 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f742(p2971: List<(Str, Adt65)>) -> bool {
    let m2970 = p2971;
    {
        let mut m2970 = m2970;
        loop {
            let step = {
                let v0 = m2970;
                let m2967 = v0;
                match m2967.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2969 = {
                            let (_, h2968) = v1;
                            h2968
                        };
                        match m2969 {
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
                Ok(next) => { m2970 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f743(p2976: List<(Str, Adt65)>) -> bool {
    let m2975 = p2976;
    {
        let mut m2975 = m2975;
        loop {
            let step = {
                let v0 = m2975;
                let m2972 = v0;
                match m2972.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2974 = {
                            let (_, h2973) = v1;
                            h2973
                        };
                        match m2974 {
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
                Ok(next) => { m2975 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f744() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f26(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f745(p2979: List<Adt130>, p2980: List<Adt130>) -> List<Adt130> {
    let m2978 = (p2979, p2980);
    {
        let mut m2978 = m2978;
        loop {
            let step = {
                let (v0, v1) = m2978;
                let m2977 = v0;
                match m2977.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<Adt130>), List<Adt130>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt130>, List<Adt130>), List<Adt130>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f746(v0: Adt20, p2983: List<Adt51>, p2984: List<Adt130>) -> List<Adt130> {
    let m2982 = (p2983, p2984);
    {
        let mut m2982 = m2982;
        loop {
            let step = {
                let (v1, v2) = m2982;
                let m2981 = v1;
                match m2981.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt130>), List<Adt130>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt51>, List<Adt130>), List<Adt130>>((v4, List::cons(Adt130::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m2982 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f747(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2985 = v0;
    match m2985 {
        Adt13::C0 => {
            let m2986 = f234(v2);
            match m2986 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2987 = v1;
                    let a2988 = v3;
                    Ok(equal(a2987, a2988))
                }
            }
        }
        Adt13::C1 => {
            f748(v1, v2)
        }
        Adt13::C2 => {
            let m2989 = f259(v2);
            match m2989 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2990 = v1;
                        let a2991 = Str::lit("true");
                        Ok(equal(a2990, a2991))
                    } else {
                        let a2992 = v1;
                        let a2993 = Str::lit("false");
                        Ok(equal(a2992, a2993))
                    }
                }
            }
        }
    }
}

pub fn f748(v0: Str, v1: Adt6) -> R<bool> {
    let m2994 = f500(v1);
    match m2994 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2995 = f236(v2)?;
            match m2995 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2997 = f749(f85({
                        let a2996 = v0;
                        utf8_encode(a2996)
                    })?)?;
                    match m2997 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f394({
                                let a2998 = v3;
                                let a2999 = v4;
                                int_sub(a2998, a2999)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f749(v0: List<u64>) -> R<Option<i64>> {
    let m3000 = v0.clone();
    match m3000.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3001 = v1;
            let a3002 = 45u64;
            if nat_eq(a3001, a3002) {
                let m3003 = v2.clone();
                match m3003.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3004 = f750(v2, 0i64)?;
                        match m3004 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3005 = v5;
                                    int_neg(a3005)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f750(v0, 0i64)
            }
        }
    }
}

pub fn f750(p3011: List<u64>, p3012: i64) -> R<Option<i64>> {
    let m3010 = (p3011, p3012);
    {
        let mut m3010 = m3010;
        loop {
            let step = {
                let (v0, v1) = m3010;
                let m3006 = v0;
                match m3006.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3007 = f751(v2);
                        match m3007 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f115({
                                    let a3008 = v1;
                                    let a3009 = 10i64;
                                    int_mul(a3008, a3009)?
                                }, f309(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f751(v0: u64) -> Option<u64> {
    let a3013 = v0;
    let a3014 = 48u64;
    if if nat_lt(a3013, a3014) {
        true
    } else {
        let a3015 = 57u64;
        let a3016 = v0;
        nat_lt(a3015, a3016)
    } {
        None::<u64>
    } else {
        Some({
            let a3017 = v0;
            let a3018 = 48u64;
            nat_sub(a3017, a3018)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
