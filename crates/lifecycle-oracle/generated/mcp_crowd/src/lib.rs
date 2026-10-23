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
    F34,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt21) -> Result<Adt49, Adt84> {
        match self {
            Fn1::F34 => f34(p0),
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
    f29(f30(), List::<Adt35>::nil())
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
            Ok(Some(Adt84::C9(f31(v1))))
        }
    }
}

pub fn f15(v0: Adt14) -> R<Result<Adt32, Adt84>> {
    let m33 = v0;
    match m33 {
        Adt14::C0(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C1(v1) => {
            Ok(f32(f10(), f12()?, v1))
        }
        Adt14::C2(v2) => {
            f33(f12()?, Fn1::F34, v2)
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
    let m34 = f35(v0.clone(), v1)?;
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
            f36(v0.clone(), v1, v3.clone(), f35(v0, match v3 {
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
    f37(f38(v0, List::<Adt86>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f39(f40(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt86> {
    List::cons(Adt86::C0(f41(), f42()), List::<Adt86>::nil())
}

pub fn f23() -> List<u64> {
    let a71 = f43();
    let a72 = {
        let a69 = f44();
        let a70 = f45();
        append_list(a69, a70)
    };
    append_list(a71, a72)
}

pub fn f24(v0: List<u64>) -> bool {
    f46(v0, f47())
}

pub fn f25(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt90>) -> R<Adt88> {
    let m73 = v4;
    match m73 {
        Ok(v6) => {
            f52(v0, v1, v2, v3, f53(v6)?)
        }
        Err(_) => {
            f48(f49(), f50(None::<Adt91>, f51(Adt92::C0)))
        }
    }
}

pub fn f26(v0: Bytes) -> R<Result<Adt6, Adt90>> {
    f54(f55(v0)?)
}

pub fn f27() -> Adt88 {
    Adt88::C0(f56(), List::cons(Adt86::C0(f57(), f58()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f28(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt94>> {
    let m74 = f59(v0.clone());
    match m74 {
        None => {
            f60(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f29(p77: u64, p78: List<Adt35>) -> R<List<Adt35>> {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v0, v1) = m76;
                let m75 = v0;
                if m75 == 0 {
                    Err::<(u64, List<Adt35>), R<List<Adt35>>>(Ok(v1))
                } else {
                    let v2 = m75 - 1;
                    let v3: Adt35 = f61(f62(v2)?, Str::lit("Answers with its own name"), f63());
                    Ok::<(u64, List<Adt35>), R<List<Adt35>>>((v2, List::cons(v3, v1)))
                }
            };
            match step {
                Ok(next) => { m76 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30() -> u64 {
    60u64
}

pub fn f31(v0: Adt94) -> Str {
    let m79 = v0;
    match m79 {
        Adt94::C0(v1) => {
            let a80 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a81 = Str::lit(" ");
            join(a80, a81)
        }
        Adt94::C1(v2) => {
            let a82 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a83 = Str::lit(" ");
            join(a82, a83)
        }
        Adt94::C2(v3) => {
            let a84 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a85 = Str::lit(" ");
            join(a84, a85)
        }
        Adt94::C3(v4) => {
            let a86 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a87 = Str::lit(" ");
            join(a86, a87)
        }
        Adt94::C4(v5, v6) => {
            let a88 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a89 = Str::lit(" ");
            join(a88, a89)
        }
        Adt94::C5(v7, v8) => {
            let a90 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a91 = Str::lit(" ");
            join(a90, a91)
        }
        Adt94::C6(v9, v10) => {
            let a92 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a93 = Str::lit(" ");
            join(a92, a93)
        }
        Adt94::C7(v11) => {
            let a94 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a95 = Str::lit(" ");
            join(a94, a95)
        }
        Adt94::C8(v12) => {
            let a96 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a97 = Str::lit(" ");
            join(a96, a97)
        }
        Adt94::C9(v13) => {
            let a98 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a99 = Str::lit(" ");
            join(a98, a99)
        }
        Adt94::C10(v14) => {
            let a100 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a101 = Str::lit(" ");
            join(a100, a101)
        }
        Adt94::C11(v15) => {
            let a102 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a103 = Str::lit(" ");
            join(a102, a103)
        }
        Adt94::C12(v16, v17) => {
            let a104 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a105 = Str::lit(" ");
            join(a104, a105)
        }
    }
}

pub fn f32(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m107 = match v2 {
        Adt20::C0(_, h106) => {
            h106
        }
    };
    match m107 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C1(Adt34::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f33(v0: List<Adt35>, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m109 = f64(match v2.clone() {
        Adt21::C0(_, h108, _, _, _) => {
            h108
        }
    }, v0);
    match m109 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C0(match v2 {
                Adt21::C0(_, h110, _, _, _) => {
                    h110
                }
            })))
        }
        Some(v3) => {
            f65(v3, v1, v2)
        }
    }
}

pub fn f34(v0: Adt21) -> Result<Adt49, Adt84> {
    Ok::<Adt49, Adt84>(f66(List::cons(f67(match v0 {
        Adt21::C0(_, h111, _, _, _) => {
            h111
        }
    }), List::<Adt38>::nil())))
}

pub fn f35(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a115 = {
        let a112 = v1;
        let a113 = 4u64;
        nat_add(a112, a113)?
    };
    let a116 = {
        let a114 = v0.clone();
        length_bytes(a114)
    };
    if nat_le(a115, a116) {
        Ok(Some({
            let a133 = {
                let a117 = f68(v0.clone(), v1)?;
                let a118 = 16777216u64;
                nat_mul(a117, a118)?
            };
            let a134 = {
                let a131 = {
                    let a121 = f68(v0.clone(), {
                        let a119 = v1;
                        let a120 = 1u64;
                        nat_add(a119, a120)?
                    })?;
                    let a122 = 65536u64;
                    nat_mul(a121, a122)?
                };
                let a132 = {
                    let a129 = {
                        let a125 = f68(v0.clone(), {
                            let a123 = v1;
                            let a124 = 2u64;
                            nat_add(a123, a124)?
                        })?;
                        let a126 = 256u64;
                        nat_mul(a125, a126)?
                    };
                    let a130 = f68(v0, {
                        let a127 = v1;
                        let a128 = 3u64;
                        nat_add(a127, a128)?
                    })?;
                    nat_add(a129, a130)?
                };
                nat_add(a131, a132)?
            };
            nat_add(a133, a134)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f36(v0: Bytes, v1: Adt89, v2: Adt89, v3: Option<u64>) -> R<Result<Adt85, Adt87>> {
    let m135 = v3;
    match m135 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v2 {
                Adt89::C0(_, h136) => {
                    h136
                }
            })))
        }
        Some(v4) => {
            let m140 = f69(v0.clone(), {
                let a138 = match v2.clone() {
                    Adt89::C0(_, h137) => {
                        h137
                    }
                };
                let a139 = 4u64;
                nat_add(a138, a139)?
            }, v4, List::<Adt86>::nil())?;
            match m140 {
                Ok(v6) => {
                    f70(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt85, Adt87>(v5))
                }
            }
        }
    }
}

pub fn f37(p145: List<Adt86>, p146: List<u64>) -> List<u64> {
    let m144 = (p145, p146);
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v0, v1) = m144;
                let m141 = v0;
                match m141.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<u64>), List<u64>>((v3, {
                            let a142 = f71(v2);
                            let a143 = v1;
                            append_list(a142, a143)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(p149: List<Adt86>, p150: List<Adt86>) -> List<Adt86> {
    let m148 = (p149, p150);
    {
        let mut m148 = m148;
        loop {
            let step = {
                let (v0, v1) = m148;
                let m147 = v0;
                match m147.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39(p155: List<List<u64>>, p156: List<u64>) -> List<u64> {
    let m154 = (p155, p156);
    {
        let mut m154 = m154;
        loop {
            let step = {
                let (v0, v1) = m154;
                let m151 = v0;
                match m151.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a152 = f72(v2);
                            let a153 = v1;
                            append_list(a152, a153)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(p159: List<List<u64>>, p160: List<List<u64>>) -> List<List<u64>> {
    let m158 = (p159, p160);
    {
        let mut m158 = m158;
        loop {
            let step = {
                let (v0, v1) = m158;
                let m157 = v0;
                match m157.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f42() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f43() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f44() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f45() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f46(p167: List<u64>, p168: List<u64>) -> bool {
    let m166 = (p167, p168);
    {
        let mut m166 = m166;
        loop {
            let step = {
                let (v0, v1) = m166;
                let m161 = v0;
                match m161.uncons() {
                    None => {
                        let m162 = v1;
                        match m162.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m163 = v1;
                        match m163.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a164 = v4;
                                let a165 = v6;
                                if nat_eq(a164, a165) {
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
                Ok(next) => { m166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f48(v0: u64, v1: Adt6) -> R<Adt88> {
    let m169 = f73(v1)?;
    match m169 {
        Ok(v3) => {
            Ok(Adt88::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f49() -> u64 {
    400u64
}

pub fn f50(v0: Option<Adt91>, v1: Adt80) -> Adt6 {
    Adt6::C5(f74(v0, v1))
}

pub fn f51(v0: Adt92) -> Adt80 {
    Adt80::C0(f75(v0.clone()), f76(v0), None::<Adt6>)
}

pub fn f52(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt95, Adt92>) -> R<Adt88> {
    let m170 = v4;
    match m170 {
        Ok(v6) => {
            f79(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f48(f77(v5.clone()), f50(f78(v5.clone()), f51(v5)))
        }
    }
}

pub fn f53(v0: Adt6) -> R<Result<Adt95, Adt92>> {
    let m171 = v0;
    match m171 {
        Adt6::C0 => {
            Ok(f81(None::<Adt91>, Adt93::C0))
        }
        Adt6::C1(_) => {
            Ok(f81(None::<Adt91>, Adt93::C0))
        }
        Adt6::C2(_) => {
            Ok(f81(None::<Adt91>, Adt93::C0))
        }
        Adt6::C3(_) => {
            Ok(f81(None::<Adt91>, Adt93::C0))
        }
        Adt6::C4(_) => {
            Ok(f81(None::<Adt91>, Adt93::C1))
        }
        Adt6::C5(v1) => {
            f80(Adt5::C0(v1))
        }
    }
}

pub fn f54(v0: List<u64>) -> R<Result<Adt6, Adt90>> {
    let m172 = f82(v0.clone())?;
    match m172 {
        Adt101::C0 => {
            f83(f84(), 0u64, v0)
        }
        Adt101::C1(v1) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C0(v1)))
        }
        Adt101::C2(v2) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C1(v2)))
        }
    }
}

pub fn f55(v0: Bytes) -> R<List<u64>> {
    f85(v0.clone(), {
        let a173 = v0;
        length_bytes(a173)
    }, List::<u64>::nil())
}

pub fn f56() -> u64 {
    405u64
}

pub fn f57() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f58() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f59(v0: List<Adt35>) -> Option<Adt94> {
    let m174 = f86(v0, List::<Str>::nil());
    match m174 {
        None => {
            None::<Adt94>
        }
        Some(v1) => {
            Some(Adt94::C1(v1))
        }
    }
}

pub fn f60(p178: List<Adt35>, p179: List<Adt12>) -> R<Result<List<Adt12>, Adt94>> {
    let m177 = (p178, p179);
    {
        let mut m177 = m177;
        loop {
            let step = {
                let (v0, v1) = m177;
                let m175 = v0;
                match m175.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>(Ok(Ok::<List<Adt12>, Adt94>(f87(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m176 = f88(v2)?;
                        match m176 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>((v3, f87(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>(Ok(Err::<List<Adt12>, Adt94>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f62(v0: u64) -> R<Str> {
    let a181 = List::cons(Str::lit("crowd_"), List::cons({
        let a180 = f89(v0, 0i64)?;
        format_i64(a180)
    }, List::<Str>::nil()));
    let a182 = Str::lit("");
    Ok(join(a181, a182))
}

pub fn f63() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f64(v0: Str, p188: List<Adt35>) -> Option<Adt35> {
    let m187 = p188;
    {
        let mut m187 = m187;
        loop {
            let step = {
                let v1 = m187;
                let m183 = v1;
                match m183.uncons() {
                    None => {
                        Err::<List<Adt35>, Option<Adt35>>(None::<Adt35>)
                    }
                    Some((v2, v3)) => {
                        let a185 = match v2.clone() {
                            Adt35::C0(h184, _, _, _, _, _, _, _) => {
                                h184
                            }
                        };
                        let a186 = v0.clone();
                        if equal(a185, a186) {
                            Err::<List<Adt35>, Option<Adt35>>(Some(v2))
                        } else {
                            Ok::<List<Adt35>, Option<Adt35>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m195 = f90(Adt6::C5({
        let m191 = match v0.clone() {
            Adt35::C0(_, _, _, _, h190, _, _, _) => {
                h190
            }
        };
        match m191 {
            Adt5::C0(h189) => {
                h189
            }
        }
    }), Adt6::C5({
        let m194 = match v0.clone() {
            Adt35::C0(_, _, _, _, h193, _, _, _) => {
                h193
            }
        };
        match m194 {
            Adt5::C0(h192) => {
                h192
            }
        }
    }), f91(v2.clone()))?;
    match m195 {
        Adt109::C0 => {
            f92(v0, v1, v2)
        }
        Adt109::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f93(f94(v3, v4)?, true))))
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

pub fn f66(v0: List<Adt38>) -> Adt49 {
    f95(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f67(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f68(v0: Bytes, v1: u64) -> R<u64> {
    let m198 = {
        let a196 = v0;
        let a197 = v1;
        index_bytes(a196, a197)
    };
    match m198 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a243 = {
                let a219 = {
                    let a207 = {
                        let a201 = {
                            let a199 = v3;
                            let a200 = 128u8;
                            fixed_u8::bit_and(a199, a200)
                        };
                        let a202 = 128u8;
                        if equal(a201, a202) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a208 = {
                        let a205 = {
                            let a203 = v3;
                            let a204 = 64u8;
                            fixed_u8::bit_and(a203, a204)
                        };
                        let a206 = 64u8;
                        if equal(a205, a206) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a207, a208)?
                };
                let a220 = {
                    let a217 = {
                        let a211 = {
                            let a209 = v3;
                            let a210 = 32u8;
                            fixed_u8::bit_and(a209, a210)
                        };
                        let a212 = 32u8;
                        if equal(a211, a212) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a218 = {
                        let a215 = {
                            let a213 = v3;
                            let a214 = 16u8;
                            fixed_u8::bit_and(a213, a214)
                        };
                        let a216 = 16u8;
                        if equal(a215, a216) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a217, a218)?
                };
                nat_add(a219, a220)?
            };
            let a244 = {
                let a241 = {
                    let a229 = {
                        let a223 = {
                            let a221 = v3;
                            let a222 = 8u8;
                            fixed_u8::bit_and(a221, a222)
                        };
                        let a224 = 8u8;
                        if equal(a223, a224) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a230 = {
                        let a227 = {
                            let a225 = v3;
                            let a226 = 4u8;
                            fixed_u8::bit_and(a225, a226)
                        };
                        let a228 = 4u8;
                        if equal(a227, a228) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a229, a230)?
                };
                let a242 = {
                    let a239 = {
                        let a233 = {
                            let a231 = v3;
                            let a232 = 2u8;
                            fixed_u8::bit_and(a231, a232)
                        };
                        let a234 = 2u8;
                        if equal(a233, a234) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a240 = {
                        let a237 = {
                            let a235 = v3;
                            let a236 = 1u8;
                            fixed_u8::bit_and(a235, a236)
                        };
                        let a238 = 1u8;
                        if equal(a237, a238) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a239, a240)?
                };
                nat_add(a241, a242)?
            };
            nat_add(a243, a244)
        }
    }
}

pub fn f69(v0: Bytes, p254: u64, p255: u64, p256: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m253 = (p254, (p255, p256));
    {
        let mut m253 = m253;
        loop {
            let step = {
                let (v1, (v2, v3)) = m253;
                let m245 = v2;
                if m245 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f96(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m245 - 1;
                    let m246 = f16(v0.clone(), v1)?;
                    match m246 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m248 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h247) => {
                                    h247
                                }
                            })?;
                            match m248 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h249) => {
                                            h249
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h250) => {
                                            h250
                                        }
                                    }, (v4, List::cons(Adt86::C0(f55(match v5 {
                                        Adt89::C0(h251, _) => {
                                            h251
                                        }
                                    })?, f55(match v6 {
                                        Adt89::C0(h252, _) => {
                                            h252
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m258 = f16(v0.clone(), {
        let (_, h257) = v3.clone();
        h257
    })?;
    match m258 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h259) = v3;
                h259
            })))
        }
        Some(v4) => {
            let m261 = f35(v0.clone(), match v4.clone() {
                Adt89::C0(_, h260) => {
                    h260
                }
            })?;
            match m261 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h262) => {
                            h262
                        }
                    })))
                }
                Some(v5) => {
                    let a267 = {
                        let a264 = match v4.clone() {
                            Adt89::C0(_, h263) => {
                                h263
                            }
                        };
                        let a265 = 4u64;
                        nat_add(a264, a265)?
                    };
                    let a268 = {
                        let a266 = v0;
                        length_bytes(a266)
                    };
                    if nat_eq(a267, a268) {
                        Ok(Ok::<Adt85, Adt87>(Adt85::C0(f55(match v1 {
                            Adt89::C0(h269, _) => {
                                h269
                            }
                        })?, f55(match v2 {
                            Adt89::C0(h270, _) => {
                                h270
                            }
                        })?, {
                            let (h271, _) = v3;
                            h271
                        }, match v4 {
                            Adt89::C0(h272, _) => {
                                h272
                            }
                        }, v5)))
                    } else {
                        Ok(Err::<Adt85, Adt87>(Adt87::C1({
                            let a274 = match v4 {
                                Adt89::C0(_, h273) => {
                                    h273
                                }
                            };
                            let a275 = 4u64;
                            nat_add(a274, a275)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f71(v0: Adt86) -> List<u64> {
    let a286 = f18({
        let a277 = match v0.clone() {
            Adt86::C0(h276, _) => {
                h276
            }
        };
        length_list(a277)
    });
    let a287 = {
        let a284 = match v0.clone() {
            Adt86::C0(h278, _) => {
                h278
            }
        };
        let a285 = {
            let a282 = f18({
                let a280 = match v0.clone() {
                    Adt86::C0(_, h279) => {
                        h279
                    }
                };
                length_list(a280)
            });
            let a283 = match v0 {
                Adt86::C0(_, h281) => {
                    h281
                }
            };
            append_list(a282, a283)
        };
        append_list(a284, a285)
    };
    append_list(a286, a287)
}

pub fn f72(v0: List<u64>) -> List<u64> {
    let a289 = f18({
        let a288 = v0.clone();
        length_list(a288)
    });
    let a290 = v0;
    append_list(a289, a290)
}

pub fn f73(v0: Adt6) -> R<Result<List<u64>, Adt90>> {
    f97(f98(), v0)
}

pub fn f74(v0: Option<Adt91>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f99())), {
        let a291 = f100(v0);
        let a292 = List::cons((Str::lit("error"), f101(v1)), List::<(Str, Adt6)>::nil());
        append_list(a291, a292)
    })
}

pub fn f75(v0: Adt92) -> i64 {
    let m293 = v0;
    match m293 {
        Adt92::C0 => {
            f102()
        }
        Adt92::C1(_, _) => {
            f103()
        }
    }
}

pub fn f76(v0: Adt92) -> Str {
    let m294 = v0;
    match m294 {
        Adt92::C0 => {
            Str::lit("Parse error")
        }
        Adt92::C1(_, v2) => {
            f104(v2)
        }
    }
}

pub fn f77(v0: Adt92) -> u64 {
    let m295 = v0;
    match m295 {
        Adt92::C0 => {
            f49()
        }
        Adt92::C1(_, _) => {
            f49()
        }
    }
}

pub fn f78(v0: Adt92) -> Option<Adt91> {
    let m296 = v0;
    match m296 {
        Adt92::C0 => {
            None::<Adt91>
        }
        Adt92::C1(v1, _) => {
            v1
        }
    }
}

pub fn f79(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt95) -> R<Adt88> {
    f105(f106(v0.clone(), v1, v2, Some(f107(v3, match v0 {
        Adt0::C0(_, _, _, _, h297, _) => {
            h297
        }
    }, f108(v4.clone()), match v4.clone() {
        Adt95::C0(_, h298, _) => {
            h298
        }
    })?), f109(), v4)?)
}

pub fn f80(v0: Adt5) -> R<Result<Adt95, Adt92>> {
    let m299 = f110(v0.clone())?;
    match m299 {
        Adt122::C0 => {
            let m300 = f111(Str::lit("method"), v0.clone());
            match m300 {
                Adt123::C0 => {
                    Ok(f81(None::<Adt91>, f112(v0)))
                }
                Adt123::C1(_) => {
                    Ok(f81(None::<Adt91>, if f113(v0) {
                        Adt93::C8
                    } else {
                        Adt93::C7
                    }))
                }
            }
        }
        Adt122::C1(v2) => {
            Ok(f114(v2, v0))
        }
        Adt122::C2 => {
            Ok(f81(None::<Adt91>, Adt93::C5))
        }
    }
}

pub fn f81(v0: Option<Adt91>, v1: Adt93) -> Result<Adt95, Adt92> {
    Err::<Adt95, Adt92>(Adt92::C1(v0, v1))
}

pub fn f82(v0: List<u64>) -> R<Adt101> {
    Ok(f115({
        let a301 = v0.clone();
        length_list(a301)
    }, f116(Adt102::C0, 0u64, 0u64, v0)?))
}

pub fn f83(p308: Adt103, p309: u64, p310: List<u64>) -> R<Result<Adt6, Adt90>> {
    let m307 = (p308, (p309, p310));
    {
        let mut m307 = m307;
        loop {
            let step = {
                let (v0, (v1, v2)) = m307;
                let m302 = v2;
                match m302.uncons() {
                    None => {
                        let m303 = f117(v0, v1)?;
                        match m303 {
                            Ok(v3) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Ok::<Adt6, Adt90>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Err::<Adt6, Adt90>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m304 = f118(v0, v1, v5)?;
                        match m304 {
                            Adt124::C0(v7) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Err::<Adt6, Adt90>(v7)))
                            }
                            Adt124::C1(v8) => {
                                Ok::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>((v8, ({
                                    let a305 = v1;
                                    let a306 = 1u64;
                                    nat_add(a305, a306)?
                                }, v6)))
                            }
                        }
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

pub fn f84() -> Adt103 {
    Adt103::C0(Adt104::C0, List::<Adt108>::nil(), 0u64)
}

pub fn f85(v0: Bytes, p313: u64, p314: List<u64>) -> R<List<u64>> {
    let m312 = (p313, p314);
    {
        let mut m312 = m312;
        loop {
            let step = {
                let (v1, v2) = m312;
                let m311 = v1;
                if m311 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m311 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f68(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(p320: List<Adt35>, p321: List<Str>) -> Option<Str> {
    let m319 = (p320, p321);
    {
        let mut m319 = m319;
        loop {
            let step = {
                let (v0, v1) = m319;
                let m315 = v0;
                match m315.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f119(match v2.clone() {
                            Adt35::C0(h316, _, _, _, _, _, _, _) => {
                                h316
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h317, _, _, _, _, _, _, _) => {
                                    h317
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h318, _, _, _, _, _, _, _) => {
                                    h318
                                }
                            }, v1)))
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

pub fn f87(p324: List<Adt12>, p325: List<Adt12>) -> List<Adt12> {
    let m323 = (p324, p325);
    {
        let mut m323 = m323;
        loop {
            let step = {
                let (v0, v1) = m323;
                let m322 = v0;
                match m322.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt35) -> R<Result<List<Adt12>, Adt94>> {
    let m328 = f120(match v0.clone() {
        Adt35::C0(h326, _, _, _, _, _, _, _) => {
            h326
        }
    }, f121(match v0.clone() {
        Adt35::C0(h327, _, _, _, _, _, _, _) => {
            h327
        }
    })?);
    match m328 {
        None => {
            f122(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f89(p331: u64, p332: i64) -> R<i64> {
    let m330 = (p331, p332);
    {
        let mut m330 = m330;
        loop {
            let step = {
                let (v0, v1) = m330;
                let m329 = v0;
                if m329 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m329 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f123(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt109> {
    f124(v0, f125(), Adt111::C0, List::cons(Adt112::C0(v1, v2, List::<Adt110>::nil()), List::<Adt112>::nil()))
}

pub fn f91(v0: Adt21) -> Adt6 {
    let m334 = match v0 {
        Adt21::C0(_, _, h333, _, _) => {
            h333
        }
    };
    match m334 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h335) => {
                    h335
                }
            })
        }
    }
}

pub fn f92(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m337 = {
        let c336: Fn1 = v1;
        c336.apply(v2)
    };
    match m337 {
        Ok(v4) => {
            f126(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f93(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f94(v0: List<Adt110>, v1: Str) -> R<Str> {
    let v2: Str = f127(v0)?;
    let a340 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a338 = v2.clone();
        let a339 = Str::lit("");
        if equal(a338, a339) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a341 = Str::lit(" ");
    Ok(join(a340, a341))
}

pub fn f95(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f96(p344: List<Adt86>, p345: List<Adt86>) -> List<Adt86> {
    let m343 = (p344, p345);
    {
        let mut m343 = m343;
        loop {
            let step = {
                let (v0, v1) = m343;
                let m342 = v0;
                match m342.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt90>> {
    let m346 = f128(v0, List::cons(Adt114::C1(0u64, v1), List::<Adt114>::nil()), List::<u64>::nil())?;
    match m346 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt90>(f129(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt90>(v3))
        }
    }
}

pub fn f98() -> u64 {
    67108864u64
}

pub fn f99() -> Str {
    Str::lit("2.0")
}

pub fn f100(v0: Option<Adt91>) -> List<(Str, Adt6)> {
    let m347 = v0;
    match m347 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f130(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f101(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h348, _, _) => {
            h348
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h349, _) => {
            h349
        }
    })), f131(match v0 {
        Adt80::C0(_, _, h350) => {
            h350
        }
    }))))
}

pub fn f102() -> i64 {
    -32700i64
}

pub fn f103() -> i64 {
    -32600i64
}

pub fn f104(v0: Adt93) -> Str {
    let m351 = v0;
    match m351 {
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

pub fn f105(v0: Adt115) -> R<Adt88> {
    let m353 = match v0.clone() {
        Adt115::C0(_, h352) => {
            h352
        }
    };
    match m353 {
        Ok(v1) => {
            let m354 = f132(v1.clone())?;
            match m354 {
                None => {
                    Ok(f6())
                }
                Some(v2) => {
                    Ok(f133(f48(f134(), f135(match v0 {
                        Adt115::C0(h355, _) => {
                            h355
                        }
                    }, v2))?, f136(match v1 {
                        Adt116::C0(_, _, h356) => {
                            h356
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v3) => {
            f48(f137(v3.clone()), f50(Some(match v0 {
                Adt115::C0(h357, _) => {
                    h357
                }
            }), f138(v3)?))
        }
    }
}

pub fn f106(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt96>, v4: List<Str>, v5: Adt95) -> R<Adt115> {
    Ok(Adt115::C0(match v5.clone() {
        Adt95::C0(h358, _, _) => {
            h358
        }
    }, f139(v0, v1, v2, f140(v3, v4, v5)?)?))
}

pub fn f107(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt96> {
    let v4: Adt121 = f141(Adt98::C0, f142(v0.clone(), f143())?);
    let v5: Adt121 = f141(Adt98::C1, f142(v0.clone(), f144())?);
    let v6: Adt121 = if f145(v3) {
        f141(Adt98::C2, f146(f142(v0.clone(), f147())?)?)
    } else {
        f148()
    };
    Ok(Adt96::C0(match v4.clone() {
        Adt121::C0(h359, _) => {
            h359
        }
    }, match v5.clone() {
        Adt121::C0(h360, _) => {
            h360
        }
    }, match v6.clone() {
        Adt121::C0(h361, _) => {
            h361
        }
    }, f149(match v4 {
        Adt121::C0(_, h362) => {
            h362
        }
    }, f149(match v5 {
        Adt121::C0(_, h363) => {
            h363
        }
    }, match v6 {
        Adt121::C0(_, h364) => {
            h364
        }
    })), f150(v0, v1, v2)?))
}

pub fn f108(v0: Adt95) -> Option<Str> {
    let a366 = match v0.clone() {
        Adt95::C0(_, h365, _) => {
            h365
        }
    };
    let a367 = Str::lit("tools/call");
    if equal(a366, a367) {
        let m369 = match v0 {
            Adt95::C0(_, _, h368) => {
                h368
            }
        };
        match m369 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m370 = f111(Str::lit("name"), v1);
                match m370 {
                    Adt123::C0 => {
                        None::<Str>
                    }
                    Adt123::C1(v2) => {
                        f151(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f109() -> List<Str> {
    List::cons(f152(), List::<Str>::nil())
}

pub fn f110(v0: Adt5) -> R<Adt122> {
    let m371 = f111(Str::lit("id"), v0);
    match m371 {
        Adt123::C0 => {
            Ok(Adt122::C0)
        }
        Adt123::C1(v1) => {
            let m372 = v1;
            match m372 {
                Adt6::C0 => {
                    Ok(Adt122::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt122::C2)
                }
                Adt6::C2(v3) => {
                    let m373 = f153(v3)?;
                    match m373 {
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

pub fn f111(v0: Str, v1: Adt5) -> Adt123 {
    f154(v0, match v1 {
        Adt5::C0(h374) => {
            h374
        }
    })
}

pub fn f112(v0: Adt5) -> Adt93 {
    if f113(v0) {
        Adt93::C8
    } else {
        Adt93::C3
    }
}

pub fn f113(v0: Adt5) -> bool {
    if f155(Str::lit("result"), v0.clone()) {
        true
    } else {
        f155(Str::lit("error"), v0)
    }
}

pub fn f114(v0: Adt91, v1: Adt5) -> Result<Adt95, Adt92> {
    let m375 = f111(Str::lit("jsonrpc"), v1.clone());
    match m375 {
        Adt123::C0 => {
            f81(Some(v0), Adt93::C2)
        }
        Adt123::C1(v2) => {
            let m376 = f151(v2);
            match m376 {
                None => {
                    f81(Some(v0), Adt93::C2)
                }
                Some(v3) => {
                    let a377 = v3;
                    let a378 = f99();
                    if equal(a377, a378) {
                        f156(v0, v1)
                    } else {
                        f81(Some(v0), Adt93::C2)
                    }
                }
            }
        }
    }
}

pub fn f115(v0: u64, v1: u64) -> Adt101 {
    let a379 = v1;
    let a380 = 0u64;
    if nat_eq(a379, a380) {
        Adt101::C0
    } else {
        let a381 = v1;
        let a382 = 2u64;
        if nat_eq(a381, a382) {
            Adt101::C2(v0)
        } else {
            Adt101::C1({
                let a386 = v0;
                let a387 = {
                    let a383 = v1;
                    let a384 = 3u64;
                    let a385 = 0u64;
                    nat_quot(a383, a384, a385)
                };
                nat_sub(a386, a387)
            })
        }
    }
}

pub fn f116(p415: Adt102, p416: u64, p417: u64, p418: List<u64>) -> R<u64> {
    let m414 = (p415, (p416, (p417, p418)));
    {
        let mut m414 = m414;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m414;
                let m388 = v3;
                match m388.uncons() {
                    None => {
                        let m389 = v0;
                        match m389 {
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
                        let m390 = v0;
                        match m390 {
                            Adt102::C0 => {
                                let a391 = v4;
                                let a392 = 128u64;
                                if nat_lt(a391, a392) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a393 = v4;
                                    let a394 = 194u64;
                                    if nat_lt(a393, a394) {
                                        let v6: u64 = f157({
                                            let a395 = List::cons(v4, v5);
                                            length_list(a395)
                                        })?;
                                        Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a396 = v4;
                                        let a397 = 224u64;
                                        if nat_lt(a396, a397) {
                                            Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a398 = v4;
                                            let a399 = 224u64;
                                            if nat_eq(a398, a399) {
                                                Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a400 = v4;
                                                let a401 = 237u64;
                                                if nat_eq(a400, a401) {
                                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a402 = v4;
                                                    let a403 = 240u64;
                                                    if nat_lt(a402, a403) {
                                                        Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a404 = v4;
                                                        let a405 = 240u64;
                                                        if nat_eq(a404, a405) {
                                                            Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a406 = v4;
                                                            let a407 = 244u64;
                                                            if nat_eq(a406, a407) {
                                                                Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a408 = v4;
                                                                let a409 = 244u64;
                                                                if nat_lt(a408, a409) {
                                                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f157({
                                                                        let a410 = List::cons(v4, v5);
                                                                        length_list(a410)
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
                                if f158(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f157({
                                        let a411 = List::cons(v4, v5);
                                        length_list(a411)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt102::C2 => {
                                if f158(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f157({
                                        let a412 = List::cons(v4, v5);
                                        length_list(a412)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt102::C3 => {
                                if f158(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f157({
                                        let a413 = List::cons(v4, v5);
                                        length_list(a413)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
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

pub fn f117(v0: Adt103, v1: u64) -> R<Result<Adt6, Adt90>> {
    let m420 = match v0.clone() {
        Adt103::C0(h419, _, _) => {
            h419
        }
    };
    match m420 {
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
            let m421 = f159(v3)?;
            match m421 {
                Adt131::C0 => {
                    Ok(Err::<Adt6, Adt90>(Adt90::C7(v1)))
                }
                Adt131::C1 => {
                    Ok(Err::<Adt6, Adt90>(Adt90::C8(v1)))
                }
                Adt131::C2(v4) => {
                    let m426 = {
                        let m425 = f160(v4, match v0.clone() {
                            Adt103::C0(_, h423, _) => {
                                h423
                            }
                        }, match v0 {
                            Adt103::C0(_, _, h424) => {
                                h424
                            }
                        });
                        match m425 {
                            Adt103::C0(h422, _, _) => {
                                h422
                            }
                        }
                    };
                    match m426 {
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

pub fn f118(v0: Adt103, v1: u64, v2: u64) -> R<Adt124> {
    let m428 = match v0.clone() {
        Adt103::C0(h427, _, _) => {
            h427
        }
    };
    match m428 {
        Adt104::C0 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                f162(v0, v1, v2)
            }
        }
        Adt104::C1 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a429 = v2;
                let a430 = 93u64;
                if nat_eq(a429, a430) {
                    Ok(f163(v0, v1))
                } else {
                    f162(v0, v1, v2)
                }
            }
        }
        Adt104::C2 => {
            Ok(f164(v0, v1, v2))
        }
        Adt104::C3 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a431 = v2;
                let a432 = 125u64;
                if nat_eq(a431, a432) {
                    Ok(f165(v0, v1))
                } else {
                    let a433 = v2;
                    let a434 = 34u64;
                    if nat_eq(a433, a434) {
                        Ok(f166(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f167(Adt90::C2(v1)))
                    }
                }
            }
        }
        Adt104::C4 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a435 = v2;
                let a436 = 34u64;
                if nat_eq(a435, a436) {
                    Ok(f166(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f167(Adt90::C2(v1)))
                }
            }
        }
        Adt104::C5 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a437 = v2;
                let a438 = 58u64;
                if nat_eq(a437, a438) {
                    Ok(f166(v0, Adt104::C0))
                } else {
                    Ok(f167(Adt90::C2(v1)))
                }
            }
        }
        Adt104::C6(v3, v4, v5) => {
            f168(v0, v1, v2, v3, v4, v5)
        }
        Adt104::C7(v6, v7) => {
            f169(v0, v1, v2, v6, v7)
        }
        Adt104::C8(v8) => {
            f170(v0, v1, v2, v8)
        }
        Adt104::C9(_) => {
            Ok(f164(v0, v1, v2))
        }
    }
}

pub fn f119(v0: Str, p443: List<Str>) -> bool {
    let m442 = p443;
    {
        let mut m442 = m442;
        loop {
            let step = {
                let v1 = m442;
                let m439 = v1;
                match m439.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a440 = v0.clone();
                        let a441 = v2;
                        if equal(a440, a441) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: Str, v1: List<u64>) -> Option<Adt94> {
    if f171(v1) {
        Some(Adt94::C0(v0))
    } else {
        None::<Adt94>
    }
}

pub fn f121(v0: Str) -> R<List<u64>> {
    f55({
        let a444 = v0;
        utf8_encode(a444)
    })
}

pub fn f122(v0: Adt35) -> R<Result<List<Adt12>, Adt94>> {
    let m445 = f172(v0.clone());
    match m445 {
        None => {
            f173(v0.clone(), f174(f175(), match v0.clone() {
                Adt35::C0(h446, _, _, _, _, _, _, _) => {
                    h446
                }
            }, Adt6::C5({
                let m449 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h448, _, _, _) => {
                        h448
                    }
                };
                match m449 {
                    Adt5::C0(h447) => {
                        h447
                    }
                }
            }), List::cons(Adt125::C0(Adt6::C5({
                let m452 = match v0 {
                    Adt35::C0(_, _, _, _, h451, _, _, _) => {
                        h451
                    }
                };
                match m452 {
                    Adt5::C0(h450) => {
                        h450
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt125>::nil()), f176())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f123(v0: i64, v1: i64) -> R<i64> {
    let a454 = v0;
    let a455 = {
        let a453 = v1;
        int_neg(a453)?
    };
    int_sub(a454, a455)
}

pub fn f124(v0: Adt6, p474: u64, p475: Adt111, p476: List<Adt112>) -> R<Adt109> {
    let m473 = (p474, (p475, p476));
    {
        let mut m473 = m473;
        loop {
            let step = {
                let (v1, (v2, v3)) = m473;
                let m456 = v1;
                if m456 == 0 {
                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C4))
                } else {
                    let v4 = m456 - 1;
                    let m457 = v3;
                    match m457.uncons() {
                        None => {
                            let m458 = v2;
                            match m458 {
                                Adt111::C0 => {
                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C0))
                                }
                                Adt111::C1(v5, v6) => {
                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m459 = v2.clone();
                            match m459 {
                                Adt111::C0 => {
                                    let m460 = v7.clone();
                                    match m460 {
                                        Adt112::C0(v9, v10, v11) => {
                                            let m461 = f177(v0.clone(), v9, v10, v11.clone())?;
                                            match m461 {
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
                                                        let a462 = v15;
                                                        let a463 = v8;
                                                        append_list(a462, a463)
                                                    };
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt112::C1(_, _, _) => {
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v8)))
                                        }
                                        Adt112::C2(_, _, _, _) => {
                                            let v24: Adt126 = f178(v7, v8)?;
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v24.clone() {
                                                Adt126::C0(h464, _) => {
                                                    h464
                                                }
                                            }, match v24 {
                                                Adt126::C0(_, h465) => {
                                                    h465
                                                }
                                            })))
                                        }
                                        Adt112::C3(v25) => {
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (Adt111::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt112::C4(_, _, _, _) => {
                                            let v39: Adt126 = f178(v7, v8)?;
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v39.clone() {
                                                Adt126::C0(h469, _) => {
                                                    h469
                                                }
                                            }, match v39 {
                                                Adt126::C0(_, h470) => {
                                                    h470
                                                }
                                            })))
                                        }
                                        Adt112::C5(v26, v27, v28, v29) => {
                                            let m466 = f179(v26, v27, v28, v29.clone());
                                            match m466 {
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
                                                        let a467 = v33;
                                                        let a468 = v8;
                                                        append_list(a467, a468)
                                                    };
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt111::C1(v40, v41) => {
                                    let v42: Adt126 = f180(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v42.clone() {
                                        Adt126::C0(h471, _) => {
                                            h471
                                        }
                                    }, match v42 {
                                        Adt126::C0(_, h472) => {
                                            h472
                                        }
                                    })))
                                }
                            }
                        }
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

pub fn f125() -> u64 {
    67108864u64
}

pub fn f126(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f181({
        let m479 = match v1.clone() {
            Adt49::C0(h478, _) => {
                h478
            }
        };
        match m479 {
            Adt37::C0(h477, _, _) => {
                h477
            }
        }
    })? {
        f182(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f127(v0: List<Adt110>) -> R<Str> {
    let a480 = List::cons(Str::lit(""), f183(v0, List::<Str>::nil())?);
    let a481 = Str::lit("/");
    Ok(join(a480, a481))
}

pub fn f128(p486: u64, p487: List<Adt114>, p488: List<u64>) -> R<Result<List<u64>, Adt90>> {
    let m485 = (p486, (p487, p488));
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v0, (v1, v2)) = m485;
                let m482 = v0;
                if m482 == 0 {
                    Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Err::<List<u64>, Adt90>(Adt90::C13)))
                } else {
                    let v3 = m482 - 1;
                    let m483 = v1;
                    match m483.uncons() {
                        None => {
                            Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Ok::<List<u64>, Adt90>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m484 = f184(v4, v5, v2)?;
                            match m484 {
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
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(p491: List<u64>, p492: List<u64>) -> List<u64> {
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

pub fn f130(v0: Adt91) -> Adt6 {
    let m493 = v0;
    match m493 {
        Adt91::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt91::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f131(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m494 = v0;
    match m494 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f132(v0: Adt116) -> R<Option<Adt5>> {
    let m496 = f185(match v0.clone() {
        Adt116::C0(h495, _, _) => {
            h495
        }
    })?;
    match m496 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a499 = v1;
                let a500 = {
                    let m498 = f186(v0);
                    match m498 {
                        Adt5::C0(h497) => {
                            h497
                        }
                    }
                };
                append_list(a499, a500)
            })))
        }
    }
}

pub fn f133(v0: Adt88, v1: Option<List<List<u64>>>) -> Adt88 {
    let m501 = v1;
    match m501 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt88::C0(match v0.clone() {
                Adt88::C0(h502, _, _, _) => {
                    h502
                }
            }, match v0.clone() {
                Adt88::C0(_, h503, _, _) => {
                    h503
                }
            }, f40(v2, List::<List<u64>>::nil()), match v0 {
                Adt88::C0(_, _, _, h504) => {
                    h504
                }
            })
        }
    }
}

pub fn f134() -> u64 {
    200u64
}

pub fn f135(v0: Adt91, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f99())), List::cons((Str::lit("id"), f130(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h505) => {
            h505
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f136(p509: List<Adt118>, p510: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m508 = (p509, p510);
    {
        let mut m508 = m508;
        loop {
            let step = {
                let (v0, v1) = m508;
                let m506 = v0;
                match m506.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m507 = f187(v2)?;
                        match m507 {
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
                Ok(next) => { m508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Adt119) -> u64 {
    let m511 = v0;
    match m511 {
        Adt119::C0(_) => {
            f49()
        }
        Adt119::C1(_, _) => {
            f49()
        }
        Adt119::C2(_) => {
            f49()
        }
        Adt119::C3(_) => {
            f188()
        }
        Adt119::C4(v6) => {
            f189(v6)
        }
    }
}

pub fn f138(v0: Adt119) -> R<Adt80> {
    Ok(Adt80::C0(f190(v0.clone()), f191(v0.clone()), f192(v0)?))
}

pub fn f139(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt127, Adt119>) -> R<Result<Adt116, Adt119>> {
    let m512 = v3;
    match m512 {
        Ok(v5) => {
            f193(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt116, Adt119>(v4))
        }
    }
}

pub fn f140(v0: Option<Adt96>, v1: List<Str>, v2: Adt95) -> R<Result<Adt127, Adt119>> {
    Ok(f194(f195(match v2.clone() {
        Adt95::C0(_, h513, _) => {
            h513
        }
    }), f196(v0, v1, v2)?))
}

pub fn f141(v0: Adt98, v1: Adt129) -> Adt121 {
    let m514 = v1;
    match m514 {
        Adt129::C0 => {
            f148()
        }
        Adt129::C1 => {
            Adt121::C0(None::<Str>, Some(Adt97::C6(v0)))
        }
        Adt129::C2 => {
            Adt121::C0(None::<Str>, Some(Adt97::C7(v0)))
        }
        Adt129::C3(v2) => {
            let m515 = f197(v2);
            match m515 {
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

pub fn f142(v0: List<Adt86>, v1: List<u64>) -> R<Adt129> {
    let m516 = f198(v0, v1, Adt130::C0)?;
    match m516 {
        Adt130::C0 => {
            Ok(Adt129::C0)
        }
        Adt130::C1(v2) => {
            let m517 = f199(v2);
            match m517 {
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

pub fn f143() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f144() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f145(v0: Str) -> bool {
    let m518 = f200(v0);
    match m518 {
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

pub fn f146(v0: Adt129) -> R<Adt129> {
    let m519 = v0.clone();
    match m519 {
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
            let m520 = f201(v1);
            match m520 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m521 = f202(v2)?;
                    match m521 {
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

pub fn f147() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f148() -> Adt121 {
    Adt121::C0(None::<Str>, None::<Adt97>)
}

pub fn f149(v0: Option<Adt97>, v1: Option<Adt97>) -> Option<Adt97> {
    let m522 = v0.clone();
    match m522 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f150(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt99>> {
    Ok(f203(f204(v0, v1, v2, List::<Adt99>::nil())?, List::<Adt99>::nil()))
}

pub fn f151(v0: Adt6) -> Option<Str> {
    let m523 = v0;
    match m523 {
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

pub fn f152() -> Str {
    Str::lit("2026-07-28")
}

pub fn f153(v0: Adt7) -> R<Option<i64>> {
    let m526 = f205(40u64, match v0.clone() {
        Adt7::C0(h524, _) => {
            h524
        }
    }, match v0 {
        Adt7::C0(_, h525) => {
            h525
        }
    })?;
    match m526 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f206(v1))
        }
    }
}

pub fn f154(v0: Str, p532: List<(Str, Adt6)>) -> Adt123 {
    let m531 = p532;
    {
        let mut m531 = m531;
        loop {
            let step = {
                let v1 = m531;
                let m527 = v1;
                match m527.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt123>(Adt123::C0)
                    }
                    Some((v2, v3)) => {
                        let m529 = f207({
                            let (h528, _) = v2;
                            h528
                        }, v0.clone());
                        match m529 {
                            Adt139::C0 => {
                                Err::<List<(Str, Adt6)>, Adt123>(Adt123::C1({
                                    let (_, h530) = v2;
                                    h530
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
                Ok(next) => { m531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Str, v1: Adt5) -> bool {
    f208(f111(v0, v1))
}

pub fn f156(v0: Adt91, v1: Adt5) -> Result<Adt95, Adt92> {
    let m533 = f111(Str::lit("method"), v1.clone());
    match m533 {
        Adt123::C0 => {
            f81(Some(v0), f112(v1))
        }
        Adt123::C1(v2) => {
            let m534 = f151(v2);
            match m534 {
                None => {
                    f81(Some(v0), Adt93::C4)
                }
                Some(v3) => {
                    f209(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f157(v0: u64) -> R<u64> {
    let a537 = {
        let a535 = v0;
        let a536 = 3u64;
        nat_mul(a535, a536)?
    };
    let a538 = 1u64;
    nat_add(a537, a538)
}

pub fn f158(v0: u64, v1: u64, v2: u64) -> bool {
    let a539 = v1;
    let a540 = v0;
    if nat_le(a539, a540) {
        let a541 = v0;
        let a542 = v2;
        nat_le(a541, a542)
    } else {
        false
    }
}

pub fn f159(v0: Adt106) -> R<Adt131> {
    if f210(match v0.clone() {
        Adt106::C0(_, h543, _, _, _, _, _) => {
            h543
        }
    }) {
        let v1: i64 = {
            let a549 = if match v0.clone() {
                Adt106::C0(_, _, _, _, _, h544, _) => {
                    h544
                }
            } {
                let a546 = match v0.clone() {
                    Adt106::C0(_, _, _, _, _, _, h545) => {
                        h545
                    }
                };
                int_neg(a546)?
            } else {
                match v0.clone() {
                    Adt106::C0(_, _, _, _, _, _, h547) => {
                        h547
                    }
                }
            };
            let a550 = match v0.clone() {
                Adt106::C0(_, _, _, _, h548, _, _) => {
                    h548
                }
            };
            int_sub(a549, a550)?
        };
        let m553 = {
            let a551 = v1;
            let a552 = -9999i64;
            compare(a551, a552)
        };
        if if match m553 {
            Ordering::Less => {
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
            let m556 = {
                let a554 = 9999i64;
                let a555 = v1;
                compare(a554, a555)
            };
            match m556 {
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
                Adt106::C0(h557, _, _, _, _, _, _) => {
                    h557
                }
            } {
                let a559 = match v0 {
                    Adt106::C0(_, _, h558, _, _, _, _) => {
                        h558
                    }
                };
                int_neg(a559)?
            } else {
                match v0 {
                    Adt106::C0(_, _, h560, _, _, _, _) => {
                        h560
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt131::C0)
    }
}

pub fn f160(v0: Adt6, v1: List<Adt108>, v2: u64) -> Adt103 {
    let m561 = v1.clone();
    match m561.uncons() {
        None => {
            Adt103::C0(Adt104::C9(v0), List::<Adt108>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m562 = v3;
            match m562 {
                Adt108::C0(v5) => {
                    Adt103::C0(Adt104::C2, List::cons(Adt108::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt108::C1(v6, v7) => {
                    let m563 = v7;
                    match m563 {
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

pub fn f161(v0: u64) -> bool {
    let a564 = v0;
    let a565 = 32u64;
    if nat_eq(a564, a565) {
        true
    } else {
        let a566 = v0;
        let a567 = 10u64;
        if nat_eq(a566, a567) {
            true
        } else {
            let a568 = v0;
            let a569 = 13u64;
            if nat_eq(a568, a569) {
                true
            } else {
                let a570 = v0;
                let a571 = 9u64;
                nat_eq(a570, a571)
            }
        }
    }
}

pub fn f162(v0: Adt103, v1: u64, v2: u64) -> R<Adt124> {
    let a572 = v2;
    let a573 = 34u64;
    if nat_eq(a572, a573) {
        Ok(f166(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), false)))
    } else {
        let a574 = v2;
        let a575 = 123u64;
        if nat_eq(a574, a575) {
            f211(v0, Adt108::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt104::C3, v1)
        } else {
            let a576 = v2;
            let a577 = 91u64;
            if nat_eq(a576, a577) {
                f211(v0, Adt108::C0(List::<Adt6>::nil()), Adt104::C1, v1)
            } else {
                let a578 = v2;
                let a579 = 116u64;
                if nat_eq(a578, a579) {
                    Ok(f166(v0, Adt104::C7(0u64, 1u64)))
                } else {
                    let a580 = v2;
                    let a581 = 102u64;
                    if nat_eq(a580, a581) {
                        Ok(f166(v0, Adt104::C7(1u64, 1u64)))
                    } else {
                        let a582 = v2;
                        let a583 = 110u64;
                        if nat_eq(a582, a583) {
                            Ok(f166(v0, Adt104::C7(2u64, 1u64)))
                        } else {
                            let a584 = v2;
                            let a585 = 45u64;
                            if nat_eq(a584, a585) {
                                Ok(f166(v0, f212(true, Adt107::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a586 = v2;
                                let a587 = 48u64;
                                if nat_eq(a586, a587) {
                                    Ok(f166(v0, f212(false, Adt107::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f158(v2, 49u64, 57u64) {
                                        Ok(f166(v0, f212(false, Adt107::C2, f213({
                                            let a588 = v2;
                                            let a589 = 48u64;
                                            nat_sub(a588, a589)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f167(Adt90::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f163(v0: Adt103, v1: u64) -> Adt124 {
    let m591 = match v0.clone() {
        Adt103::C0(_, h590, _) => {
            h590
        }
    };
    match m591.uncons() {
        None => {
            f167(Adt90::C2(v1))
        }
        Some((v2, v3)) => {
            let m592 = v2;
            match m592 {
                Adt108::C0(v4) => {
                    Adt124::C1(f160(Adt6::C4(f214(v4, List::<Adt6>::nil())), v3, {
                        let a594 = match v0 {
                            Adt103::C0(_, _, h593) => {
                                h593
                            }
                        };
                        let a595 = 1u64;
                        nat_sub(a594, a595)
                    }))
                }
                Adt108::C1(_, _) => {
                    f167(Adt90::C2(v1))
                }
            }
        }
    }
}

pub fn f164(v0: Adt103, v1: u64, v2: u64) -> Adt124 {
    let m597 = match v0.clone() {
        Adt103::C0(h596, _, _) => {
            h596
        }
    };
    match m597 {
        Adt104::C0 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C1 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C2 => {
            if f161(v2) {
                Adt124::C1(v0)
            } else {
                let a598 = v2;
                let a599 = 44u64;
                if nat_eq(a598, a599) {
                    f215(v0, v1)
                } else {
                    let a600 = v2;
                    let a601 = 93u64;
                    if nat_eq(a600, a601) {
                        f163(v0, v1)
                    } else {
                        let a602 = v2;
                        let a603 = 125u64;
                        if nat_eq(a602, a603) {
                            f165(v0, v1)
                        } else {
                            f167(Adt90::C2(v1))
                        }
                    }
                }
            }
        }
        Adt104::C3 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C4 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C5 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C6(_, _, _) => {
            f167(Adt90::C2(v1))
        }
        Adt104::C7(_, _) => {
            f167(Adt90::C2(v1))
        }
        Adt104::C8(_) => {
            f167(Adt90::C2(v1))
        }
        Adt104::C9(_) => {
            if f161(v2) {
                Adt124::C1(v0)
            } else {
                f167(Adt90::C10(v1))
            }
        }
    }
}

pub fn f165(v0: Adt103, v1: u64) -> Adt124 {
    let m605 = match v0.clone() {
        Adt103::C0(_, h604, _) => {
            h604
        }
    };
    match m605.uncons() {
        None => {
            f167(Adt90::C2(v1))
        }
        Some((v2, v3)) => {
            let m606 = v2;
            match m606 {
                Adt108::C0(_) => {
                    f167(Adt90::C2(v1))
                }
                Adt108::C1(v5, _) => {
                    Adt124::C1(f160(Adt6::C5(f216(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a608 = match v0 {
                            Adt103::C0(_, _, h607) => {
                                h607
                            }
                        };
                        let a609 = 1u64;
                        nat_sub(a608, a609)
                    }))
                }
            }
        }
    }
}

pub fn f166(v0: Adt103, v1: Adt104) -> Adt124 {
    Adt124::C1(f217(v0, v1))
}

pub fn f167(v0: Adt90) -> Adt124 {
    Adt124::C0(v0)
}

pub fn f168(v0: Adt103, v1: u64, v2: u64, v3: Adt105, v4: List<u64>, v5: bool) -> R<Adt124> {
    let m610 = v3;
    match m610 {
        Adt105::C0 => {
            let a611 = v2;
            let a612 = 34u64;
            if nat_eq(a611, a612) {
                Ok(f218(v0, v1, v4, v5))
            } else {
                let a613 = v2;
                let a614 = 92u64;
                if nat_eq(a613, a614) {
                    Ok(f166(v0, Adt104::C6(Adt105::C1, v4, v5)))
                } else {
                    let a615 = v2;
                    let a616 = 32u64;
                    if nat_lt(a615, a616) {
                        Ok(f167(Adt90::C4(v1)))
                    } else {
                        Ok(f166(v0, Adt104::C6(Adt105::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt105::C1 => {
            let a617 = v2;
            let a618 = 117u64;
            if nat_eq(a617, a618) {
                Ok(f166(v0, Adt104::C6(Adt105::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a619 = f219(v2);
                let a620 = 0u64;
                if nat_eq(a619, a620) {
                    Ok(f167(Adt90::C5(v1)))
                } else {
                    Ok(f166(v0, Adt104::C6(Adt105::C0, List::cons(f219(v2), v4), v5)))
                }
            }
        }
        Adt105::C2(v6, v7, v8) => {
            let a621 = 15u64;
            let a622 = f220(v2);
            if nat_lt(a621, a622) {
                Ok(f167(Adt90::C5(v1)))
            } else {
                let a625 = {
                    let a623 = v6;
                    let a624 = 1u64;
                    nat_add(a623, a624)?
                };
                let a626 = 4u64;
                if nat_eq(a625, a626) {
                    f221(v0, v1, {
                        let a629 = {
                            let a627 = v7;
                            let a628 = 16u64;
                            nat_mul(a627, a628)?
                        };
                        let a630 = f220(v2);
                        nat_add(a629, a630)?
                    }, v8, v4, v5)
                } else {
                    Ok(f166(v0, Adt104::C6(Adt105::C2({
                        let a631 = v6;
                        let a632 = 1u64;
                        nat_add(a631, a632)?
                    }, {
                        let a635 = {
                            let a633 = v7;
                            let a634 = 16u64;
                            nat_mul(a633, a634)?
                        };
                        let a636 = f220(v2);
                        nat_add(a635, a636)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt105::C3(v9) => {
            let a637 = v2;
            let a638 = 92u64;
            if nat_eq(a637, a638) {
                Ok(f166(v0, Adt104::C6(Adt105::C4(v9), v4, v5)))
            } else {
                Ok(f167(Adt90::C6(v1)))
            }
        }
        Adt105::C4(v10) => {
            let a639 = v2;
            let a640 = 117u64;
            if nat_eq(a639, a640) {
                Ok(f166(v0, Adt104::C6(Adt105::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f167(Adt90::C6(v1)))
            }
        }
    }
}

pub fn f169(v0: Adt103, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt124> {
    let a641 = v2;
    let a642 = f222(v3, v4);
    if nat_eq(a641, a642) {
        let a646 = {
            let a643 = v4;
            let a644 = 1u64;
            nat_add(a643, a644)?
        };
        let a647 = {
            let a645 = f223(v3);
            length_list(a645)
        };
        if nat_eq(a646, a647) {
            Ok(f224(v0, f225(v3)))
        } else {
            Ok(f166(v0, Adt104::C7(v3, {
                let a648 = v4;
                let a649 = 1u64;
                nat_add(a648, a649)?
            })))
        }
    } else {
        Ok(f167(Adt90::C2(v1)))
    }
}

pub fn f170(v0: Adt103, v1: u64, v2: u64, v3: Adt106) -> R<Adt124> {
    let m651 = match v3.clone() {
        Adt106::C0(_, h650, _, _, _, _, _) => {
            h650
        }
    };
    match m651 {
        Adt107::C0 => {
            let a652 = v2;
            let a653 = 48u64;
            if nat_eq(a652, a653) {
                Ok(f226(v0, v3, Adt107::C1))
            } else {
                if f158(v2, 49u64, 57u64) {
                    f227(v0, v1, v2, v3, false)
                } else {
                    Ok(f167(Adt90::C7(v1)))
                }
            }
        }
        Adt107::C1 => {
            let a654 = v2;
            let a655 = 46u64;
            if nat_eq(a654, a655) {
                Ok(f226(v0, v3, Adt107::C3))
            } else {
                if f228(v2) {
                    Ok(f226(v0, v3, Adt107::C5))
                } else {
                    if f229(v2) {
                        Ok(f167(Adt90::C7(v1)))
                    } else {
                        f230(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt107::C2 => {
            if f229(v2) {
                f227(v0, v1, v2, v3, false)
            } else {
                let a656 = v2;
                let a657 = 46u64;
                if nat_eq(a656, a657) {
                    Ok(f226(v0, v3, Adt107::C3))
                } else {
                    if f228(v2) {
                        Ok(f226(v0, v3, Adt107::C5))
                    } else {
                        f230(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt107::C3 => {
            if f229(v2) {
                f227(v0, v1, v2, v3, true)
            } else {
                Ok(f167(Adt90::C7(v1)))
            }
        }
        Adt107::C4 => {
            if f229(v2) {
                f227(v0, v1, v2, v3, true)
            } else {
                if f228(v2) {
                    Ok(f226(v0, v3, Adt107::C5))
                } else {
                    f230(v0, v1, v2, v3)
                }
            }
        }
        Adt107::C5 => {
            let a658 = v2;
            let a659 = 43u64;
            if nat_eq(a658, a659) {
                Ok(f166(v0, f212(match v3.clone() {
                    Adt106::C0(h660, _, _, _, _, _, _) => {
                        h660
                    }
                }, Adt107::C6, match v3.clone() {
                    Adt106::C0(_, _, h661, _, _, _, _) => {
                        h661
                    }
                }, match v3.clone() {
                    Adt106::C0(_, _, _, h662, _, _, _) => {
                        h662
                    }
                }, match v3 {
                    Adt106::C0(_, _, _, _, h663, _, _) => {
                        h663
                    }
                }, false, 0i64)))
            } else {
                let a664 = v2;
                let a665 = 45u64;
                if nat_eq(a664, a665) {
                    Ok(f166(v0, f212(match v3.clone() {
                        Adt106::C0(h666, _, _, _, _, _, _) => {
                            h666
                        }
                    }, Adt107::C6, match v3.clone() {
                        Adt106::C0(_, _, h667, _, _, _, _) => {
                            h667
                        }
                    }, match v3.clone() {
                        Adt106::C0(_, _, _, h668, _, _, _) => {
                            h668
                        }
                    }, match v3 {
                        Adt106::C0(_, _, _, _, h669, _, _) => {
                            h669
                        }
                    }, true, 0i64)))
                } else {
                    if f229(v2) {
                        f231(v0, v1, v2, Adt106::C0(match v3.clone() {
                            Adt106::C0(h670, _, _, _, _, _, _) => {
                                h670
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, h671, _, _, _, _, _) => {
                                h671
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, _, h672, _, _, _, _) => {
                                h672
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, _, _, h673, _, _, _) => {
                                h673
                            }
                        }, match v3 {
                            Adt106::C0(_, _, _, _, h674, _, _) => {
                                h674
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f167(Adt90::C7(v1)))
                    }
                }
            }
        }
        Adt107::C6 => {
            if f229(v2) {
                f231(v0, v1, v2, v3.clone(), match v3 {
                    Adt106::C0(_, _, _, _, _, h675, _) => {
                        h675
                    }
                })
            } else {
                Ok(f167(Adt90::C7(v1)))
            }
        }
        Adt107::C7 => {
            if f229(v2) {
                f231(v0, v1, v2, v3.clone(), match v3 {
                    Adt106::C0(_, _, _, _, _, h676, _) => {
                        h676
                    }
                })
            } else {
                f230(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f171(v0: List<u64>) -> bool {
    let m677 = v0.clone();
    match m677.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a681 = if f232(v0.clone(), false) {
                let a679 = {
                    let a678 = v0;
                    length_list(a678)
                };
                let a680 = f233();
                nat_le(a679, a680)
            } else {
                false
            };
            bool_not(a681)
        }
    }
}

pub fn f172(v0: Adt35) -> Option<Adt94> {
    let m686 = f234(match v0.clone() {
        Adt35::C0(h682, _, _, _, _, _, _, _) => {
            h682
        }
    }, {
        let m685 = match v0.clone() {
            Adt35::C0(_, _, _, _, h684, _, _, _) => {
                h684
            }
        };
        match m685 {
            Adt5::C0(h683) => {
                h683
            }
        }
    });
    match m686 {
        None => {
            f235(match v0.clone() {
                Adt35::C0(h687, _, _, _, _, _, _, _) => {
                    h687
                }
            }, {
                let m690 = match v0 {
                    Adt35::C0(_, _, _, _, h689, _, _, _) => {
                        h689
                    }
                };
                match m690 {
                    Adt5::C0(h688) => {
                        h688
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f173(v0: Adt35, v1: Result<Adt132, Adt94>) -> R<Result<List<Adt12>, Adt94>> {
    let m691 = v1;
    match m691 {
        Ok(v3) => {
            let m692 = f236(v0)?;
            match m692 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt94>(f87(match v3 {
                        Adt132::C0(h693, _) => {
                            h693
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

pub fn f174(p700: u64, v1: Str, v2: Adt6, p701: List<Adt125>, p702: Adt132) -> R<Result<Adt132, Adt94>> {
    let m699 = (p700, (p701, p702));
    {
        let mut m699 = m699;
        loop {
            let step = {
                let (v0, (v3, v4)) = m699;
                let m694 = v0;
                if m694 == 0 {
                    Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Err::<Adt132, Adt94>(Adt94::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m694 - 1;
                    let m695 = v3;
                    match m695.uncons() {
                        None => {
                            Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Ok::<Adt132, Adt94>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m696 = f237(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m696 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>((v5, ({
                                        let (h697, _) = v9;
                                        h697
                                    }, {
                                        let (_, h698) = v9;
                                        h698
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
                Ok(next) => { m699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175() -> u64 {
    67108864u64
}

pub fn f176() -> Adt132 {
    Adt132::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f177(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    let m703 = f238(v1.clone());
    match m703 {
        None => {
            let m704 = f239(v1);
            match m704 {
                None => {
                    Ok(Adt133::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f240(v0, match v5 {
                        Adt5::C0(h705) => {
                            h705
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

pub fn f178(v0: Adt112, v1: List<Adt112>) -> R<Adt126> {
    let m706 = v0;
    match m706 {
        Adt112::C0(_, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
        Adt112::C1(_, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
        Adt112::C2(v8, v9, v10, v11) => {
            Ok(f241(v8, v9, v10, {
                let a707 = v11;
                let a708 = 1u64;
                nat_add(a707, a708)?
            }, v1))
        }
        Adt112::C3(v12) => {
            Ok(Adt126::C0(Adt111::C1(v12, Str::lit("not")), v1))
        }
        Adt112::C4(v13, _, v15, v16) => {
            Ok(f242(v13, v15, v16, v1))
        }
        Adt112::C5(_, _, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
    }
}

pub fn f179(v0: Adt113, v1: Adt6, v2: Adt6, v3: List<Adt110>) -> Adt133 {
    let m709 = f239(v1);
    match m709 {
        None => {
            Adt133::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m710 = v0;
            match m710 {
                Adt113::C0 => {
                    f243(match v4 {
                        Adt5::C0(h711) => {
                            h711
                        }
                    }, v2, v3)
                }
                Adt113::C1 => {
                    f244(match v4 {
                        Adt5::C0(h712) => {
                            h712
                        }
                    }, v2, v3)
                }
                Adt113::C2 => {
                    f245(match v4 {
                        Adt5::C0(h713) => {
                            h713
                        }
                    }, v2, v3)
                }
                Adt113::C3 => {
                    f246(match v4 {
                        Adt5::C0(h714) => {
                            h714
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f180(v0: Adt112, v1: List<Adt112>, v2: List<Adt110>, v3: Str) -> Adt126 {
    let m715 = v0;
    match m715 {
        Adt112::C0(_, _, _) => {
            Adt126::C0(Adt111::C1(v2, v3), v1)
        }
        Adt112::C1(v7, v8, v9) => {
            f247(v7, v8, v9, v1)
        }
        Adt112::C2(v10, v11, v12, v13) => {
            f241(v10, v11, v12, v13, v1)
        }
        Adt112::C3(_) => {
            Adt126::C0(Adt111::C0, v1)
        }
        Adt112::C4(_, v16, v17, v18) => {
            f242(v16, v17, v18, v1)
        }
        Adt112::C5(_, _, _, _) => {
            Adt126::C0(Adt111::C1(v2, v3), v1)
        }
    }
}

pub fn f181(p724: List<Adt38>) -> R<bool> {
    let m723 = p724;
    {
        let mut m723 = m723;
        loop {
            let step = {
                let v0 = m723;
                let m716 = v0;
                match m716.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m717 = v1;
                        match m717 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f248(match v4 {
                                    Adt42::C0(h718, _, _, _) => {
                                        h718
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f248(match v5 {
                                    Adt43::C0(h719, _, _, _) => {
                                        h719
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
                                let m721 = match v7 {
                                    Adt45::C0(h720, _, _) => {
                                        h720
                                    }
                                };
                                match m721 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f248(match v9 {
                                            Adt48::C0(_, _, h722, _) => {
                                                h722
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
                Ok(next) => { m723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f249(match v1.clone() {
        Adt49::C0(h725, _) => {
            h725
        }
    }) {
        f250(v1)
    } else {
        let m727 = match v0 {
            Adt35::C0(_, _, _, _, _, h726, _, _) => {
                h726
            }
        };
        match m727 {
            None => {
                f250(v1)
            }
            Some(v2) => {
                f251(v2, v1)
            }
        }
    }
}

pub fn f183(p730: List<Adt110>, p731: List<Str>) -> R<List<Str>> {
    let m729 = (p730, p731);
    {
        let mut m729 = m729;
        loop {
            let step = {
                let (v0, v1) = m729;
                let m728 = v0;
                match m728.uncons() {
                    None => {
                        Err::<(List<Adt110>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt110>, List<Str>), R<List<Str>>>((v3, List::cons(f252(v2)?, v1)))
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

pub fn f184(v0: Adt114, v1: List<Adt114>, v2: List<u64>) -> R<Adt134> {
    let m732 = v0;
    match m732 {
        Adt114::C0(v3) => {
            Ok(Adt134::C1(v1, f129(v3, v2)))
        }
        Adt114::C1(v4, v5) => {
            f253(v4, v5, v1, v2)
        }
        Adt114::C2(v6, v7) => {
            let m733 = v7;
            match m733.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C1(v6, v8), List::cons(Adt114::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt114::C3(v10, v11) => {
            let m734 = v11.clone();
            match m734.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt114::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt114::C4(v14, v15) => {
            let m735 = v15;
            match m735.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0({
                        let a737 = f254({
                            let (h736, _) = v16;
                            h736
                        })?;
                        let a738 = List::cons(58u64, List::<u64>::nil());
                        append_list(a737, a738)
                    }), List::cons(Adt114::C1(v14, {
                        let (_, h739) = v16;
                        h739
                    }), List::cons(Adt114::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt114::C5(v18, v19) => {
            let m740 = v19.clone();
            match m740.uncons() {
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

pub fn f185(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m741 = v0;
    match m741 {
        Adt32::C0(v1) => {
            Ok(Some(f255(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f256(v2)?))
        }
        Adt32::C2(v3) => {
            f257(f258(v3)?)
        }
        Adt32::C3(v4) => {
            f257(f258(match v4 {
                Adt49::C0(h742, _) => {
                    h742
                }
            })?)
        }
        Adt32::C4(v5) => {
            Ok(Some(f259(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f260(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f261(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f262(v8)?))
        }
        Adt32::C8(v9) => {
            f257(f263(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f264(v10)?))
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

pub fn f186(v0: Adt116) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f265(f266(match v0.clone() {
        Adt116::C0(h743, _, _) => {
            h743
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f267({
        let m746 = match v0 {
            Adt116::C0(_, h745, _) => {
                h745
            }
        };
        match m746 {
            Adt117::C0(h744, _) => {
                h744
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f187(v0: Adt118) -> R<Option<List<u64>>> {
    let m747 = f268(f269(v0))?;
    match m747 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m748 = f73(v1)?;
            match m748 {
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

pub fn f188() -> u64 {
    404u64
}

pub fn f189(v0: Adt84) -> u64 {
    let m749 = v0;
    match m749 {
        Adt84::C0(_) => {
            f49()
        }
        Adt84::C1(_) => {
            f49()
        }
        Adt84::C2(_) => {
            f49()
        }
        Adt84::C3(_) => {
            f49()
        }
        Adt84::C4(_) => {
            f49()
        }
        Adt84::C5(_) => {
            f49()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f270()
        }
        Adt84::C8(_) => {
            f49()
        }
        Adt84::C9(_) => {
            f21()
        }
    }
}

pub fn f190(v0: Adt119) -> i64 {
    let m750 = v0;
    match m750 {
        Adt119::C0(_) => {
            f271()
        }
        Adt119::C1(_, _) => {
            f272()
        }
        Adt119::C2(_) => {
            f273()
        }
        Adt119::C3(_) => {
            f274()
        }
        Adt119::C4(v6) => {
            f275(v6)
        }
    }
}

pub fn f191(v0: Adt119) -> Str {
    let m751 = v0;
    match m751 {
        Adt119::C0(v1) => {
            f276(v1)
        }
        Adt119::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt119::C2(v4) => {
            f277(v4)
        }
        Adt119::C3(_) => {
            Str::lit("Method not found")
        }
        Adt119::C4(v6) => {
            let m752 = v6;
            match m752 {
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

pub fn f192(v0: Adt119) -> R<Option<Adt6>> {
    let m753 = v0;
    match m753 {
        Adt119::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f278(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt119::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C4(v6) => {
            let m754 = v6;
            match m754 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f89(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f279(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f193(v0: Adt0, v1: Fn0, v2: u64, v3: Adt127) -> R<Result<Adt116, Adt119>> {
    f280(v0.clone(), v1, v2, v3.clone(), f281(match v0 {
        Adt0::C0(_, _, h755, _, _, _) => {
            h755
        }
    }, match v3 {
        Adt127::C0(_, h756, _, _) => {
            h756
        }
    }))
}

pub fn f194(v0: Adt135, v1: Adt136) -> Result<Adt127, Adt119> {
    let m757 = v1;
    match m757 {
        Adt136::C0(v3) => {
            f282(v3, v0)
        }
        Adt136::C1(v2) => {
            Err::<Adt127, Adt119>(v2)
        }
    }
}

pub fn f195(v0: Str) -> Adt135 {
    let a758 = v0.clone();
    let a759 = Str::lit("server/discover");
    if equal(a758, a759) {
        Adt135::C1(Adt128::C0)
    } else {
        let a760 = v0.clone();
        let a761 = Str::lit("tools/list");
        if equal(a760, a761) {
            Adt135::C1(Adt128::C1)
        } else {
            let a762 = v0.clone();
            let a763 = Str::lit("tools/call");
            if equal(a762, a763) {
                Adt135::C1(Adt128::C2)
            } else {
                let a764 = v0.clone();
                let a765 = Str::lit("resources/list");
                if equal(a764, a765) {
                    Adt135::C1(Adt128::C3)
                } else {
                    let a766 = v0.clone();
                    let a767 = Str::lit("resources/read");
                    if equal(a766, a767) {
                        Adt135::C1(Adt128::C4)
                    } else {
                        let a768 = v0.clone();
                        let a769 = Str::lit("resources/templates/list");
                        if equal(a768, a769) {
                            Adt135::C1(Adt128::C5)
                        } else {
                            let a770 = v0.clone();
                            let a771 = Str::lit("prompts/list");
                            if equal(a770, a771) {
                                Adt135::C1(Adt128::C6)
                            } else {
                                let a772 = v0.clone();
                                let a773 = Str::lit("prompts/get");
                                if equal(a772, a773) {
                                    Adt135::C1(Adt128::C7)
                                } else {
                                    let a774 = v0.clone();
                                    let a775 = Str::lit("completion/complete");
                                    if equal(a774, a775) {
                                        Adt135::C1(Adt128::C8)
                                    } else {
                                        let a776 = v0.clone();
                                        let a777 = Str::lit("tasks/get");
                                        if equal(a776, a777) {
                                            Adt135::C1(Adt128::C9)
                                        } else {
                                            let a778 = v0.clone();
                                            let a779 = Str::lit("tasks/update");
                                            if equal(a778, a779) {
                                                Adt135::C1(Adt128::C10)
                                            } else {
                                                let a780 = v0;
                                                let a781 = Str::lit("tasks/cancel");
                                                if equal(a780, a781) {
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

pub fn f196(v0: Option<Adt96>, v1: List<Str>, v2: Adt95) -> R<Adt136> {
    f283(v0, v1, v2.clone(), f284(match v2 {
        Adt95::C0(_, _, h782) => {
            h782
        }
    }))
}

pub fn f197(v0: List<u64>) -> Option<Str> {
    let a783 = f285(f286(), v0);
    utf8_decode(a783)
}

pub fn f198(p788: List<Adt86>, v1: List<u64>, p789: Adt130) -> R<Adt130> {
    let m787 = (p788, p789);
    {
        let mut m787 = m787;
        loop {
            let step = {
                let (v0, v2) = m787;
                let m784 = v0;
                match m784.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt130), R<Adt130>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f287(match v3.clone() {
                            Adt86::C0(h785, _) => {
                                h785
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt130), R<Adt130>>((v4, f288(v2, match v3 {
                                Adt86::C0(_, h786) => {
                                    h786
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt130), R<Adt130>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m787 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f289(v0);
    if f290(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f200(v0: Str) -> Adt138 {
    let m790 = f195(v0);
    match m790 {
        Adt135::C0 => {
            Adt138::C0
        }
        Adt135::C1(v1) => {
            f291(v1)
        }
    }
}

pub fn f201(v0: List<u64>) -> Option<List<u64>> {
    let m791 = f292(f293(), v0);
    match m791 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m792 = f292(f294(), f295(v1, List::<u64>::nil()));
            match m792 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f295(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f202(v0: List<u64>) -> R<Option<List<u64>>> {
    let m794 = f296({
        let a793 = v0.clone();
        length_list(a793)
    }, v0, List::<u64>::nil())?;
    match m794 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f129(v1, List::<u64>::nil())))
        }
    }
}

pub fn f203(p797: List<Adt99>, p798: List<Adt99>) -> List<Adt99> {
    let m796 = (p797, p798);
    {
        let mut m796 = m796;
        loop {
            let step = {
                let (v0, v1) = m796;
                let m795 = v0;
                match m795.uncons() {
                    None => {
                        Err::<(List<Adt99>, List<Adt99>), List<Adt99>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt99>, List<Adt99>), List<Adt99>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: List<Adt86>, p805: List<Adt12>, v2: Option<Str>, p806: List<Adt99>) -> R<List<Adt99>> {
    let m804 = (p805, p806);
    {
        let mut m804 = m804;
        loop {
            let step = {
                let (v1, v3) = m804;
                let m799 = v1;
                match m799.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f297(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>((v5, List::cons(Adt99::C0(match v4.clone() {
                                Adt12::C0(_, h800, _, _) => {
                                    h800
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h801, _) => {
                                    h801
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h802) => {
                                    h802
                                }
                            }, f298(v0.clone(), match v4 {
                                Adt12::C0(_, h803, _, _) => {
                                    h803
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m804 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(p822: u64, p823: i64, p824: i64) -> R<Option<i64>> {
    let m821 = (p822, (p823, p824));
    {
        let mut m821 = m821;
        loop {
            let step = {
                let (v0, (v1, v2)) = m821;
                let m807 = v0;
                if m807 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m807 - 1;
                    if f299(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m810 = {
                            let a808 = v2;
                            let a809 = 0i64;
                            compare(a808, a809)
                        };
                        if match m810 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f299({
                                let a811 = v1;
                                let a812 = 10i64;
                                let a813 = 0i64;
                                int_rem(a811, a812, a813)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a814 = v1;
                                    let a815 = 10i64;
                                    let a816 = 0i64;
                                    int_quot(a814, a815, a816)?
                                }, f123(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a817 = v1;
                                let a818 = 10i64;
                                int_mul(a817, a818)?
                            }, {
                                let a819 = v2;
                                let a820 = 1i64;
                                int_sub(a819, a820)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: i64) -> Option<i64> {
    let m827 = {
        let a825 = 9007199254740991i64;
        let a826 = v0;
        compare(a825, a826)
    };
    if if match m827 {
        Ordering::Less => {
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
        let m830 = {
            let a828 = v0;
            let a829 = -9007199254740991i64;
            compare(a828, a829)
        };
        match m830 {
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

pub fn f207(v0: Str, v1: Str) -> Adt139 {
    let a831 = v0;
    let a832 = v1;
    if equal(a831, a832) {
        Adt139::C0
    } else {
        Adt139::C1
    }
}

pub fn f208(v0: Adt123) -> bool {
    let m833 = v0;
    match m833 {
        Adt123::C0 => {
            false
        }
        Adt123::C1(_) => {
            true
        }
    }
}

pub fn f209(v0: Adt91, v1: Str, v2: Adt5) -> Result<Adt95, Adt92> {
    let m834 = f111(Str::lit("params"), v2);
    match m834 {
        Adt123::C0 => {
            Ok::<Adt95, Adt92>(Adt95::C0(v0, v1, None::<Adt5>))
        }
        Adt123::C1(v3) => {
            let m835 = f239(v3);
            match m835 {
                None => {
                    f81(Some(v0), Adt93::C6)
                }
                Some(v4) => {
                    Ok::<Adt95, Adt92>(Adt95::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f210(v0: Adt107) -> bool {
    let m836 = v0;
    match m836 {
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

pub fn f211(v0: Adt103, v1: Adt108, v2: Adt104, v3: u64) -> R<Adt124> {
    let a838 = match v0.clone() {
        Adt103::C0(_, _, h837) => {
            h837
        }
    };
    let a839 = f300();
    if nat_lt(a838, a839) {
        Ok(Adt124::C1(Adt103::C0(v2, List::cons(v1, match v0.clone() {
            Adt103::C0(_, h840, _) => {
                h840
            }
        }), {
            let a842 = match v0 {
                Adt103::C0(_, _, h841) => {
                    h841
                }
            };
            let a843 = 1u64;
            nat_add(a842, a843)?
        })))
    } else {
        Ok(f167(Adt90::C9(v3)))
    }
}

pub fn f212(v0: bool, v1: Adt107, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt104 {
    Adt104::C8(Adt106::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f213(v0: u64) -> i64 {
    let a844 = v0;
    let a845 = 0u64;
    if nat_eq(a844, a845) {
        0i64
    } else {
        let a846 = v0;
        let a847 = 1u64;
        if nat_eq(a846, a847) {
            1i64
        } else {
            let a848 = v0;
            let a849 = 2u64;
            if nat_eq(a848, a849) {
                2i64
            } else {
                let a850 = v0;
                let a851 = 3u64;
                if nat_eq(a850, a851) {
                    3i64
                } else {
                    let a852 = v0;
                    let a853 = 4u64;
                    if nat_eq(a852, a853) {
                        4i64
                    } else {
                        let a854 = v0;
                        let a855 = 5u64;
                        if nat_eq(a854, a855) {
                            5i64
                        } else {
                            let a856 = v0;
                            let a857 = 6u64;
                            if nat_eq(a856, a857) {
                                6i64
                            } else {
                                let a858 = v0;
                                let a859 = 7u64;
                                if nat_eq(a858, a859) {
                                    7i64
                                } else {
                                    let a860 = v0;
                                    let a861 = 8u64;
                                    if nat_eq(a860, a861) {
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

pub fn f214(p864: List<Adt6>, p865: List<Adt6>) -> List<Adt6> {
    let m863 = (p864, p865);
    {
        let mut m863 = m863;
        loop {
            let step = {
                let (v0, v1) = m863;
                let m862 = v0;
                match m862.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: Adt103, v1: u64) -> Adt124 {
    let m867 = match v0.clone() {
        Adt103::C0(_, h866, _) => {
            h866
        }
    };
    match m867.uncons() {
        None => {
            f167(Adt90::C2(v1))
        }
        Some((v2, _)) => {
            let m868 = v2;
            match m868 {
                Adt108::C0(_) => {
                    f166(v0, Adt104::C0)
                }
                Adt108::C1(_, _) => {
                    f166(v0, Adt104::C4)
                }
            }
        }
    }
}

pub fn f216(p871: List<(Str, Adt6)>, p872: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m870 = (p871, p872);
    {
        let mut m870 = m870;
        loop {
            let step = {
                let (v0, v1) = m870;
                let m869 = v0;
                match m869.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
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

pub fn f217(v0: Adt103, v1: Adt104) -> Adt103 {
    Adt103::C0(v1, match v0.clone() {
        Adt103::C0(_, h873, _) => {
            h873
        }
    }, match v0 {
        Adt103::C0(_, _, h874) => {
            h874
        }
    })
}

pub fn f218(v0: Adt103, v1: u64, v2: List<u64>, v3: bool) -> Adt124 {
    let m876 = {
        let a875 = f285(f286(), f129(v2, List::<u64>::nil()));
        utf8_decode(a875)
    };
    match m876 {
        None => {
            f167(Adt90::C14)
        }
        Some(v4) => {
            if v3 {
                f301(v0, v1, v4)
            } else {
                f224(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f219(v0: u64) -> u64 {
    let a877 = v0;
    let a878 = 34u64;
    if nat_eq(a877, a878) {
        34u64
    } else {
        let a879 = v0;
        let a880 = 92u64;
        if nat_eq(a879, a880) {
            92u64
        } else {
            let a881 = v0;
            let a882 = 47u64;
            if nat_eq(a881, a882) {
                47u64
            } else {
                let a883 = v0;
                let a884 = 98u64;
                if nat_eq(a883, a884) {
                    8u64
                } else {
                    let a885 = v0;
                    let a886 = 102u64;
                    if nat_eq(a885, a886) {
                        12u64
                    } else {
                        let a887 = v0;
                        let a888 = 110u64;
                        if nat_eq(a887, a888) {
                            10u64
                        } else {
                            let a889 = v0;
                            let a890 = 114u64;
                            if nat_eq(a889, a890) {
                                13u64
                            } else {
                                let a891 = v0;
                                let a892 = 116u64;
                                if nat_eq(a891, a892) {
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

pub fn f220(v0: u64) -> u64 {
    let a893 = v0;
    let a894 = 48u64;
    if nat_lt(a893, a894) {
        16u64
    } else {
        let a895 = v0;
        let a896 = 58u64;
        if nat_lt(a895, a896) {
            let a897 = v0;
            let a898 = 48u64;
            nat_sub(a897, a898)
        } else {
            let a899 = v0;
            let a900 = 65u64;
            if nat_lt(a899, a900) {
                16u64
            } else {
                let a901 = v0;
                let a902 = 71u64;
                if nat_lt(a901, a902) {
                    let a903 = v0;
                    let a904 = 55u64;
                    nat_sub(a903, a904)
                } else {
                    let a905 = v0;
                    let a906 = 97u64;
                    if nat_lt(a905, a906) {
                        16u64
                    } else {
                        let a907 = v0;
                        let a908 = 103u64;
                        if nat_lt(a907, a908) {
                            let a909 = v0;
                            let a910 = 87u64;
                            nat_sub(a909, a910)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f221(v0: Adt103, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt124> {
    let a911 = v3;
    let a912 = 0u64;
    if nat_eq(a911, a912) {
        if f158(v2, 55296u64, 56319u64) {
            Ok(f166(v0, Adt104::C6(Adt105::C3(v2), v4, v5)))
        } else {
            if f158(v2, 56320u64, 57343u64) {
                Ok(f167(Adt90::C6(v1)))
            } else {
                Ok(f166(v0, Adt104::C6(Adt105::C0, f302(v2, v4)?, v5)))
            }
        }
    } else {
        if f158(v2, 56320u64, 57343u64) {
            Ok(f166(v0, Adt104::C6(Adt105::C0, f302({
                let a921 = 65536u64;
                let a922 = {
                    let a919 = {
                        let a915 = {
                            let a913 = v3;
                            let a914 = 55296u64;
                            nat_sub(a913, a914)
                        };
                        let a916 = 1024u64;
                        nat_mul(a915, a916)?
                    };
                    let a920 = {
                        let a917 = v2;
                        let a918 = 56320u64;
                        nat_sub(a917, a918)
                    };
                    nat_add(a919, a920)?
                };
                nat_add(a921, a922)?
            }, v4)?, v5)))
        } else {
            Ok(f167(Adt90::C6(v1)))
        }
    }
}

pub fn f222(v0: u64, v1: u64) -> u64 {
    let m925 = {
        let a923 = f223(v0);
        let a924 = v1;
        index_list(a923, a924)
    };
    match m925 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f223(v0: u64) -> List<u64> {
    let a926 = v0;
    let a927 = 0u64;
    if nat_eq(a926, a927) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a928 = v0;
        let a929 = 1u64;
        if nat_eq(a928, a929) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f224(v0: Adt103, v1: Adt6) -> Adt124 {
    Adt124::C1(f160(v1, match v0.clone() {
        Adt103::C0(_, h930, _) => {
            h930
        }
    }, match v0 {
        Adt103::C0(_, _, h931) => {
            h931
        }
    }))
}

pub fn f225(v0: u64) -> Adt6 {
    let a932 = v0;
    let a933 = 0u64;
    if nat_eq(a932, a933) {
        Adt6::C1(true)
    } else {
        let a934 = v0;
        let a935 = 1u64;
        if nat_eq(a934, a935) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f226(v0: Adt103, v1: Adt106, v2: Adt107) -> Adt124 {
    f166(v0, Adt104::C8(Adt106::C0(match v1.clone() {
        Adt106::C0(h936, _, _, _, _, _, _) => {
            h936
        }
    }, v2, match v1.clone() {
        Adt106::C0(_, _, h937, _, _, _, _) => {
            h937
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, h938, _, _, _) => {
            h938
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, _, h939, _, _) => {
            h939
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, _, _, h940, _) => {
            h940
        }
    }, match v1 {
        Adt106::C0(_, _, _, _, _, _, h941) => {
            h941
        }
    })))
}

pub fn f227(v0: Adt103, v1: u64, v2: u64, v3: Adt106, v4: bool) -> R<Adt124> {
    let v5: u64 = {
        let a943 = match v3.clone() {
            Adt106::C0(_, _, _, h942, _, _, _) => {
                h942
            }
        };
        let a944 = 0u64;
        if if nat_eq(a943, a944) {
            let a945 = v2;
            let a946 = 48u64;
            nat_eq(a945, a946)
        } else {
            false
        } {
            0u64
        } else {
            let a948 = match v3.clone() {
                Adt106::C0(_, _, _, h947, _, _, _) => {
                    h947
                }
            };
            let a949 = 1u64;
            nat_add(a948, a949)?
        }
    };
    let v6: i64 = if v4 {
        f123(match v3.clone() {
            Adt106::C0(_, _, _, _, h950, _, _) => {
                h950
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt106::C0(_, _, _, _, h951, _, _) => {
                h951
            }
        }
    };
    let a952 = f303();
    let a953 = v5;
    if nat_lt(a952, a953) {
        Ok(f167(Adt90::C8(v1)))
    } else {
        let m956 = {
            let a954 = 99999i64;
            let a955 = v6;
            compare(a954, a955)
        };
        if match m956 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f167(Adt90::C8(v1)))
        } else {
            Ok(f166(v0, f212(match v3.clone() {
                Adt106::C0(h957, _, _, _, _, _, _) => {
                    h957
                }
            }, if v4 {
                Adt107::C4
            } else {
                Adt107::C2
            }, f123({
                let a959 = match v3.clone() {
                    Adt106::C0(_, _, h958, _, _, _, _) => {
                        h958
                    }
                };
                let a960 = 10i64;
                int_mul(a959, a960)?
            }, f213({
                let a961 = v2;
                let a962 = 48u64;
                nat_sub(a961, a962)
            }))?, v5, v6, match v3.clone() {
                Adt106::C0(_, _, _, _, _, h963, _) => {
                    h963
                }
            }, match v3 {
                Adt106::C0(_, _, _, _, _, _, h964) => {
                    h964
                }
            })))
        }
    }
}

pub fn f228(v0: u64) -> bool {
    let a965 = v0;
    let a966 = 101u64;
    if nat_eq(a965, a966) {
        true
    } else {
        let a967 = v0;
        let a968 = 69u64;
        nat_eq(a967, a968)
    }
}

pub fn f229(v0: u64) -> bool {
    f158(v0, 48u64, 57u64)
}

pub fn f230(v0: Adt103, v1: u64, v2: u64, v3: Adt106) -> R<Adt124> {
    let m969 = f159(v3)?;
    match m969 {
        Adt131::C0 => {
            Ok(f167(Adt90::C7(v1)))
        }
        Adt131::C1 => {
            Ok(f167(Adt90::C8(v1)))
        }
        Adt131::C2(v4) => {
            Ok(f164(f160(v4, match v0.clone() {
                Adt103::C0(_, h970, _) => {
                    h970
                }
            }, match v0 {
                Adt103::C0(_, _, h971) => {
                    h971
                }
            }), v1, v2))
        }
    }
}

pub fn f231(v0: Adt103, v1: u64, v2: u64, v3: Adt106, v4: bool) -> R<Adt124> {
    let v5: i64 = f123({
        let a973 = match v3.clone() {
            Adt106::C0(_, _, _, _, _, _, h972) => {
                h972
            }
        };
        let a974 = 10i64;
        int_mul(a973, a974)?
    }, f213({
        let a975 = v2;
        let a976 = 48u64;
        nat_sub(a975, a976)
    }))?;
    let m979 = {
        let a977 = 99999i64;
        let a978 = v5;
        compare(a977, a978)
    };
    if match m979 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f167(Adt90::C8(v1)))
    } else {
        Ok(f166(v0, f212(match v3.clone() {
            Adt106::C0(h980, _, _, _, _, _, _) => {
                h980
            }
        }, Adt107::C7, match v3.clone() {
            Adt106::C0(_, _, h981, _, _, _, _) => {
                h981
            }
        }, match v3.clone() {
            Adt106::C0(_, _, _, h982, _, _, _) => {
                h982
            }
        }, match v3 {
            Adt106::C0(_, _, _, _, h983, _, _) => {
                h983
            }
        }, v4, v5)))
    }
}

pub fn f232(p986: List<u64>, v1: bool) -> bool {
    let m985 = p986;
    {
        let mut m985 = m985;
        loop {
            let step = {
                let v0 = m985;
                let m984 = v0;
                match m984.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f304(v2)
                        } else {
                            f305(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m985 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233() -> u64 {
    64u64
}

pub fn f234(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt94> {
    let m987 = f154(Str::lit("type"), v1);
    match m987 {
        Adt123::C0 => {
            Some(Adt94::C3(v0))
        }
        Adt123::C1(v2) => {
            let m988 = f151(v2);
            match m988 {
                None => {
                    Some(Adt94::C3(v0))
                }
                Some(v3) => {
                    let a989 = v3;
                    let a990 = Str::lit("object");
                    if equal(a989, a990) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f235(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt94> {
    let m991 = f154(Str::lit("$schema"), v1);
    match m991 {
        Adt123::C0 => {
            None::<Adt94>
        }
        Adt123::C1(v2) => {
            let m992 = f151(v2);
            match m992 {
                None => {
                    Some(Adt94::C2(v0))
                }
                Some(v3) => {
                    let a993 = v3;
                    let a994 = f306();
                    if equal(a993, a994) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f236(v0: Adt35) -> R<Option<Adt94>> {
    let m996 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h995, _, _) => {
            h995
        }
    };
    match m996 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v1) => {
            let m999 = f235(match v0.clone() {
                Adt35::C0(h997, _, _, _, _, _, _, _) => {
                    h997
                }
            }, match v1.clone() {
                Adt5::C0(h998) => {
                    h998
                }
            });
            match m999 {
                None => {
                    let m1003 = f174(f175(), match v0 {
                        Adt35::C0(h1000, _, _, _, _, _, _, _) => {
                            h1000
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1001) => {
                            h1001
                        }
                    }), List::cons(Adt125::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1002) => {
                            h1002
                        }
                    }), false, List::<Str>::nil()), List::<Adt125>::nil()), f176())?;
                    match m1003 {
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

pub fn f237(v0: Str, v1: Adt6, v2: Adt125, v3: Adt132, v4: List<Adt125>) -> R<Result<(List<Adt125>, Adt132), Adt94>> {
    let m1004 = v2;
    match m1004 {
        Adt125::C0(v5, v6, v7) => {
            let m1005 = f238(v5.clone());
            match m1005 {
                None => {
                    let m1006 = f239(v5);
                    match m1006 {
                        None => {
                            Ok(Err::<(List<Adt125>, Adt132), Adt94>(Adt94::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f307(v0, v1, match v9 {
                                Adt5::C0(h1007) => {
                                    h1007
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

pub fn f238(v0: Adt6) -> Option<bool> {
    let m1008 = v0;
    match m1008 {
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

pub fn f239(v0: Adt6) -> Option<Adt5> {
    let m1009 = v0;
    match m1009 {
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

pub fn f240(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    let m1010 = f308(v1.clone(), v2.clone())?;
    match m1010 {
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
            f309(v0, v1, v2, v3)
        }
    }
}

pub fn f241(v0: List<Adt6>, v1: Adt6, v2: List<Adt110>, v3: u64, v4: List<Adt112>) -> Adt126 {
    let m1011 = v0;
    match m1011.uncons() {
        None => {
            let a1012 = v3;
            let a1013 = 1u64;
            if nat_eq(a1012, a1013) {
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

pub fn f242(v0: Option<Adt6>, v1: Adt6, v2: List<Adt110>, v3: List<Adt112>) -> Adt126 {
    let m1014 = v0;
    match m1014 {
        None => {
            Adt126::C0(Adt111::C0, v3)
        }
        Some(v4) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f243(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1015 = f154(Str::lit("anyOf"), v0);
    match m1015 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1016 = f310(v3);
            match m1016 {
                None => {
                    Adt133::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1017 = v4;
                    match m1017.uncons() {
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

pub fn f244(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1018 = f154(Str::lit("oneOf"), v0);
    match m1018 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1019 = f310(v3);
            match m1019 {
                None => {
                    Adt133::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1020 = v4;
                    match m1020.uncons() {
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

pub fn f245(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1021 = f154(Str::lit("not"), v0);
    match m1021 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            Adt133::C3(List::cons(Adt112::C0(v3, v1, v2.clone()), List::cons(Adt112::C3(v2), List::<Adt112>::nil())))
        }
    }
}

pub fn f246(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1022 = f154(Str::lit("if"), v0.clone());
    match m1022 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            Adt133::C3(List::cons(Adt112::C0(v3, v1.clone(), v2.clone()), List::cons(Adt112::C4(f311(Str::lit("then"), v0.clone()), f311(Str::lit("else"), v0), v1, v2), List::<Adt112>::nil())))
        }
    }
}

pub fn f247(v0: List<Adt6>, v1: Adt6, v2: List<Adt110>, v3: List<Adt112>) -> Adt126 {
    let m1023 = v0;
    match m1023.uncons() {
        None => {
            Adt126::C0(Adt111::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v4, v1.clone(), v2.clone()), List::cons(Adt112::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f248(v0: Str) -> R<bool> {
    let m1025 = f202(f55({
        let a1024 = v0;
        utf8_encode(a1024)
    })?)?;
    match m1025 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f249(v0: Adt37) -> bool {
    let m1027 = match v0 {
        Adt37::C0(_, _, h1026) => {
            h1026
        }
    };
    match m1027 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f250(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1029 = match v0.clone() {
        Adt49::C0(_, h1028) => {
            h1028
        }
    };
    match m1029.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h1030, _) => {
                    h1030
                }
            })))
        }
        Some((_, _)) => {
            if f312(match v0.clone() {
                Adt49::C0(_, h1031) => {
                    h1031
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f251(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1035 = {
        let m1034 = match v1.clone() {
            Adt49::C0(h1033, _) => {
                h1033
            }
        };
        match m1034 {
            Adt37::C0(_, h1032, _) => {
                h1032
            }
        }
    };
    match m1035 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1038 = f90(Adt6::C5(match v0.clone() {
                Adt5::C0(h1036) => {
                    h1036
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1037) => {
                    h1037
                }
            }), v2)?;
            match m1038 {
                Adt109::C0 => {
                    f250(v1)
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

pub fn f252(v0: Adt110) -> R<Str> {
    let m1039 = v0;
    match m1039 {
        Adt110::C0(v1) => {
            f313(v1)
        }
        Adt110::C1(v2) => {
            let a1040 = f89(v2, 0i64)?;
            Ok(format_i64(a1040))
        }
    }
}

pub fn f253(v0: u64, v1: Adt6, v2: List<Adt114>, v3: List<u64>) -> R<Adt134> {
    let m1041 = v1;
    match m1041 {
        Adt6::C0 => {
            Ok(Adt134::C1(v2, f129(f314(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt134::C1(v2, f129(if v4 {
                f314(0u64)
            } else {
                f314(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f315(v5.clone())? {
                Ok(Adt134::C1(v2, f129(f316(v5)?, v3)))
            } else {
                Ok(Adt134::C0(Adt90::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt134::C1(v2, f129(f254(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1042 = v0;
            let a1043 = f300();
            if nat_lt(a1042, a1043) {
                Ok(Adt134::C1(f317(91u64, Adt114::C2({
                    let a1044 = v0;
                    let a1045 = 1u64;
                    nat_add(a1044, a1045)?
                }, v7), v2), v3))
            } else {
                Ok(Adt134::C0(Adt90::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1046 = v0;
            let a1047 = f300();
            if nat_lt(a1046, a1047) {
                Ok(Adt134::C1(f317(123u64, Adt114::C4({
                    let a1048 = v0;
                    let a1049 = 1u64;
                    nat_add(a1048, a1049)?
                }, v8), v2), v3))
            } else {
                Ok(Adt134::C0(Adt90::C12))
            }
        }
    }
}

pub fn f254(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1051 = f129(f318(f55({
            let a1050 = v0;
            utf8_encode(a1050)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1052 = List::cons(34u64, List::<u64>::nil());
        append_list(a1051, a1052)
    }))
}

pub fn f255(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f319(match v0.clone() {
        Adt33::C0(_, h1053, _, _) => {
            h1053
        }
    })), List::cons((Str::lit("capabilities"), f320(match v0.clone() {
        Adt33::C0(_, _, h1054, _) => {
            h1054
        }
    })), {
        let a1057 = f321(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1055) => {
                h1055
            }
        });
        let a1058 = f322(match v0 {
            Adt33::C0(h1056, _, _, _) => {
                h1056
            }
        })?;
        append_list(a1057, a1058)
    })))
}

pub fn f256(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f323(match v0.clone() {
        Adt34::C0(_, _, h1059) => {
            h1059
        }
    }))), {
        let a1062 = f321(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1060, _) => {
                h1060
            }
        });
        let a1063 = f322(match v0 {
            Adt34::C0(h1061, _, _) => {
                h1061
            }
        })?;
        append_list(a1062, a1063)
    }))
}

pub fn f257(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1064 = v0;
    match m1064 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1065 = f268(Adt6::C5(v1))?;
            match m1065 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1066 = f239(v2);
                    match m1066 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1067) => {
                                    h1067
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f258(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1069 = f324(match v0.clone() {
        Adt37::C0(h1068, _, _) => {
            h1068
        }
    })?;
    match m1069 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1072 = f325(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1070, _) => {
                        h1070
                    }
                });
                let a1073 = f326(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1071) => {
                        h1071
                    }
                });
                append_list(a1072, a1073)
            })))
        }
    }
}

pub fn f259(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f327(match v0.clone() {
        Adt51::C0(_, _, h1074) => {
            h1074
        }
    })?)), {
        let a1077 = f321(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1075, _) => {
                h1075
            }
        });
        let a1078 = f322(match v0 {
            Adt51::C0(h1076, _, _) => {
                h1076
            }
        })?;
        append_list(a1077, a1078)
    }))
}

pub fn f260(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f328(match v0.clone() {
        Adt52::C0(_, h1079) => {
            h1079
        }
    }))), f322(match v0 {
        Adt52::C0(h1080, _) => {
            h1080
        }
    })?))
}

pub fn f261(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f329(match v0.clone() {
        Adt53::C0(_, _, h1081) => {
            h1081
        }
    }))), {
        let a1084 = f321(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1082, _) => {
                h1082
            }
        });
        let a1085 = f322(match v0 {
            Adt53::C0(h1083, _, _) => {
                h1083
            }
        })?;
        append_list(a1084, a1085)
    }))
}

pub fn f262(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f330(match v0.clone() {
        Adt55::C0(_, _, h1086) => {
            h1086
        }
    }))), {
        let a1089 = f321(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1087, _) => {
                h1087
            }
        });
        let a1090 = f322(match v0 {
            Adt55::C0(h1088, _, _) => {
                h1088
            }
        })?;
        append_list(a1089, a1090)
    }))
}

pub fn f263(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1092 = f331(match v0.clone() {
        Adt58::C0(_, h1091) => {
            h1091
        }
    })?;
    match m1092 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1094 = f321(Str::lit("description"), match v0 {
                    Adt58::C0(h1093, _) => {
                        h1093
                    }
                });
                let a1095 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1094, a1095)
            }))
        }
    }
}

pub fn f264(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f278({
        let m1098 = match v0.clone() {
            Adt60::C0(h1097) => {
                h1097
            }
        };
        match m1098 {
            Adt61::C0(h1096, _, _) => {
                h1096
            }
        }
    }))), {
        let a1105 = f332(Str::lit("total"), {
            let m1101 = match v0.clone() {
                Adt60::C0(h1100) => {
                    h1100
                }
            };
            match m1101 {
                Adt61::C0(_, h1099, _) => {
                    h1099
                }
            }
        })?;
        let a1106 = f326(Str::lit("hasMore"), {
            let m1104 = match v0 {
                Adt60::C0(h1103) => {
                    h1103
                }
            };
            match m1104 {
                Adt61::C0(_, _, h1102) => {
                    h1102
                }
            }
        });
        append_list(a1105, a1106)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f265(v0: Adt140) -> Str {
    let m1107 = v0;
    match m1107 {
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

pub fn f266(v0: Adt32) -> Adt140 {
    let m1108 = v0;
    match m1108 {
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

pub fn f267(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1109 = v0;
    match m1109 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f333(), f334(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f268(v0: Adt6) -> R<Option<Adt6>> {
    f335(f336(), Adt141::C0(v0), List::<Adt142>::nil())
}

pub fn f269(v0: Adt118) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f99())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f337(match v0.clone() {
        Adt118::C0(h1110, _) => {
            h1110
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1113 = match v0.clone() {
            Adt118::C0(_, h1112) => {
                h1112
            }
        };
        match m1113 {
            Adt50::C0(h1111, _, _) => {
                h1111
            }
        }
    })), {
        let a1120 = f338({
            let m1116 = match v0.clone() {
                Adt118::C0(_, h1115) => {
                    h1115
                }
            };
            match m1116 {
                Adt50::C0(_, h1114, _) => {
                    h1114
                }
            }
        });
        let a1121 = f339({
            let m1119 = match v0 {
                Adt118::C0(_, h1118) => {
                    h1118
                }
            };
            match m1119 {
                Adt50::C0(_, _, h1117) => {
                    h1117
                }
            }
        });
        append_list(a1120, a1121)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f270() -> u64 {
    429u64
}

pub fn f271() -> i64 {
    -32602i64
}

pub fn f272() -> i64 {
    -32022i64
}

pub fn f273() -> i64 {
    -32020i64
}

pub fn f274() -> i64 {
    -32601i64
}

pub fn f275(v0: Adt84) -> i64 {
    let m1122 = v0;
    match m1122 {
        Adt84::C0(_) => {
            f271()
        }
        Adt84::C1(_) => {
            f271()
        }
        Adt84::C2(_) => {
            f271()
        }
        Adt84::C3(_) => {
            f271()
        }
        Adt84::C4(_) => {
            f271()
        }
        Adt84::C5(_) => {
            f271()
        }
        Adt84::C6(_) => {
            f340()
        }
        Adt84::C7(_) => {
            f341()
        }
        Adt84::C8(_) => {
            f342()
        }
        Adt84::C9(_) => {
            f340()
        }
    }
}

pub fn f276(v0: Adt120) -> Str {
    let m1123 = v0;
    match m1123 {
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

pub fn f277(v0: Adt97) -> Str {
    let m1124 = v0;
    match m1124 {
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
            let m1125 = v1;
            match m1125 {
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
            let m1126 = v2;
            match m1126 {
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

pub fn f278(v0: List<Str>) -> List<Adt6> {
    f214(f343(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f279(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1138 = f344(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1127, _, _, _, _) => {
                h1127
            }
        });
        let a1139 = {
            let a1136 = f344(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1128, _, _, _) => {
                    h1128
                }
            });
            let a1137 = {
                let a1134 = f344(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1129, _, _) => {
                        h1129
                    }
                });
                let a1135 = {
                    let a1132 = f344(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1130, _) => {
                            h1130
                        }
                    });
                    let a1133 = f344(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1131) => {
                            h1131
                        }
                    });
                    append_list(a1132, a1133)
                };
                append_list(a1134, a1135)
            };
            append_list(a1136, a1137)
        };
        append_list(a1138, a1139)
    })
}

pub fn f280(v0: Adt0, v1: Fn0, v2: u64, v3: Adt127, v4: bool) -> R<Result<Adt116, Adt119>> {
    if v4 {
        f345(v0, v1, v2, f346(match v3.clone() {
            Adt127::C0(_, h1140, _, _) => {
                h1140
            }
        }, match v3.clone() {
            Adt127::C0(_, _, h1141, _) => {
                h1141
            }
        }, match v3 {
            Adt127::C0(_, _, _, h1142) => {
                h1142
            }
        }))
    } else {
        Ok(Err::<Adt116, Adt119>(Adt119::C3(f347(match v3 {
            Adt127::C0(_, h1143, _, _) => {
                h1143
            }
        }))))
    }
}

pub fn f281(v0: Adt4, v1: Adt128) -> bool {
    let m1144 = v1;
    match m1144 {
        Adt128::C0 => {
            true
        }
        Adt128::C1 => {
            f348(match v0 {
                Adt4::C0(_, _, _, _, _, h1145, _) => {
                    h1145
                }
            })
        }
        Adt128::C2 => {
            f348(match v0 {
                Adt4::C0(_, _, _, _, _, h1146, _) => {
                    h1146
                }
            })
        }
        Adt128::C3 => {
            f349(match v0 {
                Adt4::C0(_, _, _, _, h1147, _, _) => {
                    h1147
                }
            })
        }
        Adt128::C4 => {
            f349(match v0 {
                Adt4::C0(_, _, _, _, h1148, _, _) => {
                    h1148
                }
            })
        }
        Adt128::C5 => {
            f349(match v0 {
                Adt4::C0(_, _, _, _, h1149, _, _) => {
                    h1149
                }
            })
        }
        Adt128::C6 => {
            f348(match v0 {
                Adt4::C0(_, _, _, h1150, _, _, _) => {
                    h1150
                }
            })
        }
        Adt128::C7 => {
            f348(match v0 {
                Adt4::C0(_, _, _, h1151, _, _, _) => {
                    h1151
                }
            })
        }
        Adt128::C8 => {
            f350(match v0 {
                Adt4::C0(_, _, h1152, _, _, _, _) => {
                    h1152
                }
            })
        }
        Adt128::C9 => {
            f351(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1153) => {
                    h1153
                }
            }, f352())
        }
        Adt128::C10 => {
            f351(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1154) => {
                    h1154
                }
            }, f352())
        }
        Adt128::C11 => {
            f351(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1155) => {
                    h1155
                }
            }, f352())
        }
    }
}

pub fn f282(v0: Adt137, v1: Adt135) -> Result<Adt127, Adt119> {
    let m1156 = v1;
    match m1156 {
        Adt135::C0 => {
            Err::<Adt127, Adt119>(Adt119::C3(match v0 {
                Adt137::C0(_, h1157, _, _) => {
                    h1157
                }
            }))
        }
        Adt135::C1(v2) => {
            Ok::<Adt127, Adt119>(Adt127::C0(match v0.clone() {
                Adt137::C0(h1158, _, _, _) => {
                    h1158
                }
            }, v2, match v0.clone() {
                Adt137::C0(_, _, h1159, _) => {
                    h1159
                }
            }, match v0 {
                Adt137::C0(_, _, _, h1160) => {
                    h1160
                }
            }))
        }
    }
}

pub fn f283(v0: Option<Adt96>, v1: List<Str>, v2: Adt95, v3: Result<Adt16, Adt120>) -> R<Adt136> {
    let m1161 = v3;
    match m1161 {
        Ok(v5) => {
            Ok(f353(v1, v2.clone(), v5.clone(), f354(v0, f200(match v2.clone() {
                Adt95::C0(_, h1162, _) => {
                    h1162
                }
            }), match v2.clone() {
                Adt95::C0(_, h1163, _) => {
                    h1163
                }
            }, match v5 {
                Adt16::C0(h1164, _, _, _, _, _) => {
                    h1164
                }
            }, f355(match v2 {
                Adt95::C0(_, _, h1165) => {
                    h1165
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt136::C1(Adt119::C0(v4)))
        }
    }
}

pub fn f284(v0: Option<Adt5>) -> Result<Adt16, Adt120> {
    let m1166 = v0;
    match m1166 {
        None => {
            Err::<Adt16, Adt120>(Adt120::C0)
        }
        Some(v1) => {
            let m1167 = f111(Str::lit("_meta"), v1);
            match m1167 {
                Adt123::C0 => {
                    Err::<Adt16, Adt120>(Adt120::C1)
                }
                Adt123::C1(v2) => {
                    let m1168 = f239(v2);
                    match m1168 {
                        None => {
                            Err::<Adt16, Adt120>(Adt120::C4)
                        }
                        Some(v3) => {
                            f356(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f285(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1169 = v0;
    let a1170 = f357(f286(), f358(List::<Bytes>::nil(), v1));
    append_bytes(a1169, a1170)
}

pub fn f286() -> Bytes {
    Bytes::lit(&[])
}

pub fn f287(p1177: List<u64>, p1178: List<u64>) -> R<bool> {
    let m1176 = (p1177, p1178);
    {
        let mut m1176 = m1176;
        loop {
            let step = {
                let (v0, v1) = m1176;
                let m1171 = v0;
                match m1171.uncons() {
                    None => {
                        let m1172 = v1;
                        match m1172.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1173 = v1;
                        match m1173.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a1174 = f359(v4)?;
                                let a1175 = f359(v6)?;
                                if nat_eq(a1174, a1175) {
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
                Ok(next) => { m1176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Adt130, v1: List<u64>) -> Adt130 {
    let m1179 = v0;
    match m1179 {
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

pub fn f289(v0: List<u64>) -> List<u64> {
    f295(f360(f295(f360(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f290(p1182: List<u64>) -> bool {
    let m1181 = p1182;
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let v0 = m1181;
                let m1180 = v0;
                match m1180.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f361(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: Adt128) -> Adt138 {
    let m1183 = v0;
    match m1183 {
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

pub fn f292(p1189: List<u64>, p1190: List<u64>) -> Option<List<u64>> {
    let m1188 = (p1189, p1190);
    {
        let mut m1188 = m1188;
        loop {
            let step = {
                let (v0, v1) = m1188;
                let m1184 = v0;
                match m1184.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1185 = v1;
                        match m1185.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1186 = v4;
                                let a1187 = v2;
                                if nat_eq(a1186, a1187) {
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
                Ok(next) => { m1188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f294() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f295(p1193: List<u64>, p1194: List<u64>) -> List<u64> {
    let m1192 = (p1193, p1194);
    {
        let mut m1192 = m1192;
        loop {
            let step = {
                let (v0, v1) = m1192;
                let m1191 = v0;
                match m1191.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1192 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(p1203: u64, p1204: List<u64>, p1205: List<u64>) -> R<Option<List<u64>>> {
    let m1202 = (p1203, (p1204, p1205));
    {
        let mut m1202 = m1202;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1202;
                let m1195 = v0;
                if m1195 == 0 {
                    let m1196 = v1;
                    match m1196.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1195 - 1;
                    let m1197 = v1;
                    match m1197.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1198 = v7;
                            match m1198.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1199 = v9;
                                    match m1199.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1200 = v11;
                                            match m1200.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1201 = f362(v6, v8, v10, v12, f363(v13.clone()))?;
                                                    match m1201 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f129(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: Adt12, v1: Option<Str>) -> bool {
    let m1206 = v1;
    match m1206 {
        None => {
            false
        }
        Some(v2) => {
            let a1208 = match v0 {
                Adt12::C0(h1207, _, _, _) => {
                    h1207
                }
            };
            let a1209 = v2;
            equal(a1208, a1209)
        }
    }
}

pub fn f298(v0: List<Adt86>, v1: Str) -> R<Adt100> {
    let m1212 = f146(f142(v0, {
        let a1210 = f364();
        let a1211 = f365(v1)?;
        append_list(a1210, a1211)
    })?)?;
    match m1212 {
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
            let m1213 = f197(v2);
            match m1213 {
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

pub fn f299(v0: i64) -> bool {
    let a1220 = {
        let m1216 = {
            let a1214 = v0;
            let a1215 = 0i64;
            compare(a1214, a1215)
        };
        if match m1216 {
            Ordering::Less => {
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
            let m1219 = {
                let a1217 = 0i64;
                let a1218 = v0;
                compare(a1217, a1218)
            };
            match m1219 {
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
    bool_not(a1220)
}

pub fn f300() -> u64 {
    512u64
}

pub fn f301(v0: Adt103, v1: u64, v2: Str) -> Adt124 {
    let m1222 = match v0.clone() {
        Adt103::C0(_, h1221, _) => {
            h1221
        }
    };
    match m1222.uncons() {
        None => {
            f167(Adt90::C2(v1))
        }
        Some((v3, v4)) => {
            let m1223 = v3;
            match m1223 {
                Adt108::C0(_) => {
                    f167(Adt90::C2(v1))
                }
                Adt108::C1(v6, _) => {
                    Adt124::C1(Adt103::C0(Adt104::C5, List::cons(Adt108::C1(v6, Some(v2)), v4), match v0 {
                        Adt103::C0(_, _, h1224) => {
                            h1224
                        }
                    }))
                }
            }
        }
    }
}

pub fn f302(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1225 = v0;
    let a1226 = 128u64;
    if nat_lt(a1225, a1226) {
        Ok(List::cons(v0, v1))
    } else {
        let a1227 = v0;
        let a1228 = 2048u64;
        if nat_lt(a1227, a1228) {
            Ok(List::cons({
                let a1229 = 128u64;
                let a1230 = f366(v0, 64u64);
                nat_add(a1229, a1230)?
            }, List::cons({
                let a1231 = 192u64;
                let a1232 = f367(v0, 64u64);
                nat_add(a1231, a1232)?
            }, v1)))
        } else {
            let a1233 = v0;
            let a1234 = 65536u64;
            if nat_lt(a1233, a1234) {
                Ok(List::cons({
                    let a1235 = 128u64;
                    let a1236 = f366(v0, 64u64);
                    nat_add(a1235, a1236)?
                }, List::cons({
                    let a1237 = 128u64;
                    let a1238 = f366(f367(v0, 64u64), 64u64);
                    nat_add(a1237, a1238)?
                }, List::cons({
                    let a1239 = 224u64;
                    let a1240 = f367(v0, 4096u64);
                    nat_add(a1239, a1240)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1241 = 128u64;
                    let a1242 = f366(v0, 64u64);
                    nat_add(a1241, a1242)?
                }, List::cons({
                    let a1243 = 128u64;
                    let a1244 = f366(f367(v0, 64u64), 64u64);
                    nat_add(a1243, a1244)?
                }, List::cons({
                    let a1245 = 128u64;
                    let a1246 = f366(f367(v0, 4096u64), 64u64);
                    nat_add(a1245, a1246)?
                }, List::cons({
                    let a1247 = 240u64;
                    let a1248 = f367(v0, 262144u64);
                    nat_add(a1247, a1248)?
                }, v1)))))
            }
        }
    }
}

pub fn f303() -> u64 {
    18u64
}

pub fn f304(v0: u64) -> bool {
    let a1249 = 48u64;
    let a1250 = v0;
    if if nat_le(a1249, a1250) {
        let a1251 = v0;
        let a1252 = 57u64;
        nat_le(a1251, a1252)
    } else {
        false
    } {
        true
    } else {
        let a1253 = 65u64;
        let a1254 = v0;
        if if nat_le(a1253, a1254) {
            let a1255 = v0;
            let a1256 = 90u64;
            nat_le(a1255, a1256)
        } else {
            false
        } {
            true
        } else {
            let a1257 = 97u64;
            let a1258 = v0;
            if if nat_le(a1257, a1258) {
                let a1259 = v0;
                let a1260 = 122u64;
                nat_le(a1259, a1260)
            } else {
                false
            } {
                true
            } else {
                let a1261 = v0;
                let a1262 = 33u64;
                if nat_eq(a1261, a1262) {
                    true
                } else {
                    let a1263 = v0;
                    let a1264 = 35u64;
                    if nat_eq(a1263, a1264) {
                        true
                    } else {
                        let a1265 = v0;
                        let a1266 = 36u64;
                        if nat_eq(a1265, a1266) {
                            true
                        } else {
                            let a1267 = v0;
                            let a1268 = 37u64;
                            if nat_eq(a1267, a1268) {
                                true
                            } else {
                                let a1269 = v0;
                                let a1270 = 38u64;
                                if nat_eq(a1269, a1270) {
                                    true
                                } else {
                                    let a1271 = v0;
                                    let a1272 = 39u64;
                                    if nat_eq(a1271, a1272) {
                                        true
                                    } else {
                                        let a1273 = v0;
                                        let a1274 = 42u64;
                                        if nat_eq(a1273, a1274) {
                                            true
                                        } else {
                                            let a1275 = v0;
                                            let a1276 = 43u64;
                                            if nat_eq(a1275, a1276) {
                                                true
                                            } else {
                                                let a1277 = v0;
                                                let a1278 = 45u64;
                                                if nat_eq(a1277, a1278) {
                                                    true
                                                } else {
                                                    let a1279 = v0;
                                                    let a1280 = 46u64;
                                                    if nat_eq(a1279, a1280) {
                                                        true
                                                    } else {
                                                        let a1281 = v0;
                                                        let a1282 = 94u64;
                                                        if nat_eq(a1281, a1282) {
                                                            true
                                                        } else {
                                                            let a1283 = v0;
                                                            let a1284 = 95u64;
                                                            if nat_eq(a1283, a1284) {
                                                                true
                                                            } else {
                                                                let a1285 = v0;
                                                                let a1286 = 96u64;
                                                                if nat_eq(a1285, a1286) {
                                                                    true
                                                                } else {
                                                                    let a1287 = v0;
                                                                    let a1288 = 124u64;
                                                                    if nat_eq(a1287, a1288) {
                                                                        true
                                                                    } else {
                                                                        let a1289 = v0;
                                                                        let a1290 = 126u64;
                                                                        nat_eq(a1289, a1290)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f305(v0: u64) -> bool {
    let a1291 = 48u64;
    let a1292 = v0;
    if if nat_le(a1291, a1292) {
        let a1293 = v0;
        let a1294 = 57u64;
        nat_le(a1293, a1294)
    } else {
        false
    } {
        true
    } else {
        let a1295 = 65u64;
        let a1296 = v0;
        if if nat_le(a1295, a1296) {
            let a1297 = v0;
            let a1298 = 90u64;
            nat_le(a1297, a1298)
        } else {
            false
        } {
            true
        } else {
            let a1299 = 97u64;
            let a1300 = v0;
            if if nat_le(a1299, a1300) {
                let a1301 = v0;
                let a1302 = 122u64;
                nat_le(a1301, a1302)
            } else {
                false
            } {
                true
            } else {
                let a1303 = v0;
                let a1304 = 95u64;
                if nat_eq(a1303, a1304) {
                    true
                } else {
                    let a1305 = v0;
                    let a1306 = 45u64;
                    if nat_eq(a1305, a1306) {
                        true
                    } else {
                        let a1307 = v0;
                        let a1308 = 46u64;
                        nat_eq(a1307, a1308)
                    }
                }
            }
        }
    }
}

pub fn f306() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f307(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt132, v6: List<Adt125>) -> R<Result<(List<Adt125>, Adt132), Adt94>> {
    let m1309 = f368(v0.clone(), v1, v2.clone());
    match m1309 {
        None => {
            let m1310 = f369(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1310 {
                Adt144::C0(v8) => {
                    Ok(Err::<(List<Adt125>, Adt132), Adt94>(v8))
                }
                Adt144::C1(v9) => {
                    Ok(Ok::<(List<Adt125>, Adt132), Adt94>((f370(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt125>, Adt132), Adt94>(v7))
        }
    }
}

pub fn f308(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt133> {
    Ok(f371(List::cons((Str::lit("type"), f372(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f373(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f374(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f375(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f375(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f375(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f375(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1311 = f376(v0.clone(), v1.clone())?;
        let a1312 = List::cons((Str::lit("required"), f377(v0, v1)), List::<(Str, Adt143)>::nil());
        append_list(a1311, a1312)
    })))))))))
}

pub fn f309(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    Ok(f378(f379(v0, v1.clone(), v2.clone(), v3.clone()), f378(f380(v1.clone(), v2.clone(), v3.clone()), f378(f381(v1.clone(), v2.clone(), v3.clone()), f378(f382(v1.clone(), v2.clone(), v3.clone())?, f378(f383(v1.clone(), v2.clone(), v3.clone()), f378(f384(Adt113::C0, f243(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f378(f384(Adt113::C1, f244(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f378(f384(Adt113::C2, f245(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f384(Adt113::C3, f246(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f310(v0: Adt6) -> Option<List<Adt6>> {
    let m1313 = v0;
    match m1313 {
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

pub fn f311(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1314 = f154(v0, v1);
    match m1314 {
        Adt123::C0 => {
            None::<Adt6>
        }
        Adt123::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f312(v0: List<Adt50>) -> R<bool> {
    let m1315 = v0;
    match m1315.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f385(v2, match v1 {
                Adt50::C0(h1316, _, _) => {
                    h1316
                }
            })
        }
    }
}

pub fn f313(v0: Str) -> R<Str> {
    let m1319 = {
        let a1318 = f285(f286(), f129(f386(f55({
            let a1317 = v0.clone();
            utf8_encode(a1317)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1318)
    };
    match m1319 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f314(v0: u64) -> List<u64> {
    f223(v0)
}

pub fn f315(v0: Adt7) -> R<bool> {
    let a1337 = {
        let m1323 = {
            let a1321 = f387();
            let a1322 = match v0.clone() {
                Adt7::C0(h1320, _) => {
                    h1320
                }
            };
            compare(a1321, a1322)
        };
        if match m1323 {
            Ordering::Less => {
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
            let m1328 = {
                let a1326 = match v0.clone() {
                    Adt7::C0(h1324, _) => {
                        h1324
                    }
                };
                let a1327 = {
                    let a1325 = f387();
                    int_neg(a1325)?
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
                let m1332 = {
                    let a1330 = 9999i64;
                    let a1331 = match v0.clone() {
                        Adt7::C0(_, h1329) => {
                            h1329
                        }
                    };
                    compare(a1330, a1331)
                };
                if match m1332 {
                    Ordering::Less => {
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
                    let m1336 = {
                        let a1334 = match v0 {
                            Adt7::C0(_, h1333) => {
                                h1333
                            }
                        };
                        let a1335 = -9999i64;
                        compare(a1334, a1335)
                    };
                    match m1336 {
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
    Ok(bool_not(a1337))
}

pub fn f316(v0: Adt7) -> R<List<u64>> {
    let m1341 = {
        let a1339 = match v0.clone() {
            Adt7::C0(_, h1338) => {
                h1338
            }
        };
        let a1340 = 0i64;
        compare(a1339, a1340)
    };
    if if match m1341 {
        Ordering::Less => {
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
        let m1345 = {
            let a1343 = 0i64;
            let a1344 = match v0.clone() {
                Adt7::C0(_, h1342) => {
                    h1342
                }
            };
            compare(a1343, a1344)
        };
        match m1345 {
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
        let a1348 = f388(match v0.clone() {
            Adt7::C0(h1346, _) => {
                h1346
            }
        })?;
        let a1349 = List::cons(101u64, f388(match v0 {
            Adt7::C0(_, h1347) => {
                h1347
            }
        })?);
        Ok(append_list(a1348, a1349))
    } else {
        f388(match v0 {
            Adt7::C0(h1350, _) => {
                h1350
            }
        })
    }
}

pub fn f317(v0: u64, v1: Adt114, v2: List<Adt114>) -> List<Adt114> {
    List::cons(Adt114::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f318(p1353: List<u64>, p1354: List<u64>) -> R<List<u64>> {
    let m1352 = (p1353, p1354);
    {
        let mut m1352 = m1352;
        loop {
            let step = {
                let (v0, v1) = m1352;
                let m1351 = v0;
                match m1351.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f129(f389(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1352 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: List<Str>) -> Adt6 {
    Adt6::C4(f278(v0))
}

pub fn f320(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1372 = f344(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1355, _, _, _, _, _, _) => {
                h1355
            }
        });
        let a1373 = {
            let a1370 = f344(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1356, _, _, _, _, _) => {
                    h1356
                }
            });
            let a1371 = {
                let a1368 = f344(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1357, _, _, _, _) => {
                        h1357
                    }
                });
                let a1369 = {
                    let a1366 = f390(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1358, _, _, _) => {
                            h1358
                        }
                    });
                    let a1367 = {
                        let a1364 = f391(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1359, _, _) => {
                                h1359
                            }
                        });
                        let a1365 = {
                            let a1362 = f390(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1360, _) => {
                                    h1360
                                }
                            });
                            let a1363 = f344(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1361) => {
                                    h1361
                                }
                            });
                            append_list(a1362, a1363)
                        };
                        append_list(a1364, a1365)
                    };
                    append_list(a1366, a1367)
                };
                append_list(a1368, a1369)
            };
            append_list(a1370, a1371)
        };
        append_list(a1372, a1373)
    })
}

pub fn f321(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1374 = v1;
    match m1374 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f322(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f392(match v0.clone() {
        Adt10::C0(h1375, _) => {
            h1375
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f393(match v0 {
        Adt10::C0(_, h1376) => {
            h1376
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f323(v0: List<Adt35>) -> List<Adt6> {
    f214(f394(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f324(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1377 = f395(v0, List::<Adt6>::nil())?;
    match m1377 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f214(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f325(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1378 = v1;
    match m1378 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f326(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1379 = v1;
    match m1379 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f327(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f214(f396(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f328(v0: List<Adt46>) -> List<Adt6> {
    f214(f397(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f329(v0: List<Adt54>) -> List<Adt6> {
    f214(f398(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f330(v0: List<Adt56>) -> List<Adt6> {
    f214(f399(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f331(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m1380 = f400(v0, List::<Adt6>::nil())?;
    match m1380 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f214(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f332(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1381 = v1;
    match m1381 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f392(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f333() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f334(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1382, _, _, _, _, _) => {
            h1382
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1383, _, _, _) => {
            h1383
        }
    })), {
        let a1392 = f321(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1384, _, _, _, _) => {
                h1384
            }
        });
        let a1393 = {
            let a1390 = f321(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1385, _, _) => {
                    h1385
                }
            });
            let a1391 = {
                let a1388 = f321(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1386, _) => {
                        h1386
                    }
                });
                let a1389 = f401(match v0 {
                    Adt1::C0(_, _, _, _, _, h1387) => {
                        h1387
                    }
                });
                append_list(a1388, a1389)
            };
            append_list(a1390, a1391)
        };
        append_list(a1392, a1393)
    })))
}

pub fn f335(p1408: u64, p1409: Adt141, p1410: List<Adt142>) -> R<Option<Adt6>> {
    let m1407 = (p1408, (p1409, p1410));
    {
        let mut m1407 = m1407;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1407;
                let m1394 = v0;
                if m1394 == 0 {
                    Err::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1394 - 1;
                    let m1395 = v1;
                    match m1395 {
                        Adt141::C0(v4) => {
                            let m1396 = v4.clone();
                            match m1396 {
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
                                    let v8: Str = f402(v7)?;
                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1397 = v9;
                                    match m1397.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0(v10), List::cons(Adt142::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1398 = v12;
                                    match m1398.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f402({
                                                let (h1399, _) = v13;
                                                h1399
                                            })?;
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0({
                                                let (_, h1400) = v13;
                                                h1400
                                            }), List::cons(Adt142::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt141::C1(v16) => {
                            let m1401 = v2;
                            match m1401.uncons() {
                                None => {
                                    Err::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1402 = v17;
                                    match m1402 {
                                        Adt142::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1403 = v20;
                                            match m1403.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C4(f214(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0(v22), List::cons(Adt142::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt142::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1404 = v26;
                                            match m1404.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C5(f216(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f402({
                                                        let (h1405, _) = v28;
                                                        h1405
                                                    })?;
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0({
                                                        let (_, h1406) = v28;
                                                        h1406
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
                Ok(next) => { m1407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f336() -> u64 {
    67108864u64
}

pub fn f337(v0: Adt19) -> Adt6 {
    let m1411 = v0;
    match m1411 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f338(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1412 = v0;
    match m1412 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f339(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1413 = v0;
    match m1413 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f340() -> i64 {
    -32603i64
}

pub fn f341() -> i64 {
    429i64
}

pub fn f342() -> i64 {
    -32021i64
}

pub fn f343(p1416: List<Str>, p1417: List<Adt6>) -> List<Adt6> {
    let m1415 = (p1416, p1417);
    {
        let mut m1415 = m1415;
        loop {
            let step = {
                let (v0, v1) = m1415;
                let m1414 = v0;
                match m1414.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1418 = v1;
    match m1418 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1419) => {
                    h1419
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f345(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt120>) -> R<Result<Adt116, Adt119>> {
    let m1420 = v3;
    match m1420 {
        Ok(v5) => {
            f403(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt116, Adt119>(Adt119::C0(v4)))
        }
    }
}

pub fn f346(v0: Adt128, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt120> {
    let m1421 = v0;
    match m1421 {
        Adt128::C0 => {
            f404(v1, v2)
        }
        Adt128::C1 => {
            f405(v1, v2)
        }
        Adt128::C2 => {
            f406(v1, v2)
        }
        Adt128::C3 => {
            f407(v1, v2)
        }
        Adt128::C4 => {
            f408(v1, v2)
        }
        Adt128::C5 => {
            f409(v1, v2)
        }
        Adt128::C6 => {
            f410(v1, v2)
        }
        Adt128::C7 => {
            f411(v1, v2)
        }
        Adt128::C8 => {
            f412(v1, v2)
        }
        Adt128::C9 => {
            f413(v1, v2)
        }
        Adt128::C10 => {
            f414(v1, v2)
        }
        Adt128::C11 => {
            f415(v1, v2)
        }
    }
}

pub fn f347(v0: Adt128) -> Str {
    let m1422 = v0;
    match m1422 {
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

pub fn f348(v0: Option<Adt8>) -> bool {
    let m1423 = v0;
    match m1423 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f349(v0: Option<Adt9>) -> bool {
    let m1424 = v0;
    match m1424 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f350(v0: Option<Adt5>) -> bool {
    let m1425 = v0;
    match m1425 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f351(v0: Option<Adt5>, v1: Str) -> bool {
    let m1426 = v0;
    match m1426 {
        None => {
            false
        }
        Some(v2) => {
            f155(v1, v2)
        }
    }
}

pub fn f352() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f353(v0: List<Str>, v1: Adt95, v2: Adt16, v3: Option<Adt97>) -> Adt136 {
    let m1427 = v3;
    match m1427 {
        None => {
            f416(v1, v2.clone(), f417(match v2 {
                Adt16::C0(h1428, _, _, _, _, _) => {
                    h1428
                }
            }, v0))
        }
        Some(v4) => {
            Adt136::C1(Adt119::C2(v4))
        }
    }
}

pub fn f354(v0: Option<Adt96>, v1: Adt138, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt97>> {
    let m1429 = v0;
    match m1429 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v5) => {
            let m1431 = match v5.clone() {
                Adt96::C0(_, _, _, h1430, _) => {
                    h1430
                }
            };
            match m1431 {
                None => {
                    f418(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f355(v0: Option<Adt5>) -> Adt5 {
    let m1432 = v0;
    match m1432 {
        None => {
            f419()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f356(v0: Adt5) -> Result<Adt16, Adt120> {
    let m1433 = f420(v0.clone());
    match m1433 {
        Adt145::C0 => {
            f421(v0.clone(), f111(f422(), v0.clone()), f111(f423(), v0))
        }
        Adt145::C1 => {
            Err::<Adt16, Adt120>(Adt120::C2)
        }
        Adt145::C2 => {
            Err::<Adt16, Adt120>(Adt120::C3)
        }
    }
}

pub fn f357(p1438: Bytes, p1439: List<Bytes>) -> Bytes {
    let m1437 = (p1438, p1439);
    {
        let mut m1437 = m1437;
        loop {
            let step = {
                let (v0, v1) = m1437;
                let m1434 = v1;
                match m1434.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1435 = v2;
                            let a1436 = v0;
                            append_bytes(a1435, a1436)
                        }, v3))
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

pub fn f358(p1442: List<Bytes>, p1443: List<u64>) -> List<Bytes> {
    let m1441 = (p1442, p1443);
    {
        let mut m1441 = m1441;
        loop {
            let step = {
                let (v0, v1) = m1441;
                let m1440 = v1;
                match m1440.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f424(f425(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: u64) -> R<u64> {
    let a1444 = 64u64;
    let a1445 = v0;
    if if nat_lt(a1444, a1445) {
        let a1446 = v0;
        let a1447 = 91u64;
        nat_lt(a1446, a1447)
    } else {
        false
    } {
        let a1448 = v0;
        let a1449 = 32u64;
        nat_add(a1448, a1449)
    } else {
        Ok(v0)
    }
}

pub fn f360(p1452: List<u64>) -> List<u64> {
    let m1451 = p1452;
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let v0 = m1451;
                let m1450 = v0;
                match m1450.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f426(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: u64) -> bool {
    let a1453 = v0;
    let a1454 = 9u64;
    if nat_eq(a1453, a1454) {
        true
    } else {
        let a1455 = 31u64;
        let a1456 = v0;
        if nat_lt(a1455, a1456) {
            let a1457 = v0;
            let a1458 = 127u64;
            nat_lt(a1457, a1458)
        } else {
            false
        }
    }
}

pub fn f362(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1459 = f427(v0)?;
    match m1459 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1460 = f427(v1)?;
            match m1460 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1461 = v2;
                    let a1462 = 61u64;
                    if nat_eq(a1461, a1462) {
                        f428(v5, v6, v4, v3)
                    } else {
                        let m1463 = f427(v2)?;
                        match m1463 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1464 = v3;
                                let a1465 = 61u64;
                                if nat_eq(a1464, a1465) {
                                    f429(v5, v6, v7, v4)
                                } else {
                                    let m1466 = f427(v3)?;
                                    match m1466 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f430(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f363(v0: List<u64>) -> bool {
    let m1467 = v0;
    match m1467.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f364() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f365(v0: Str) -> R<List<u64>> {
    f55({
        let a1468 = v0;
        utf8_encode(a1468)
    })
}

pub fn f366(v0: u64, v1: u64) -> u64 {
    let a1469 = v0;
    let a1470 = v1;
    let a1471 = 0u64;
    nat_rem(a1469, a1470, a1471)
}

pub fn f367(v0: u64, v1: u64) -> u64 {
    let a1472 = v0;
    let a1473 = v1;
    let a1474 = 0u64;
    nat_quot(a1472, a1473, a1474)
}

pub fn f368(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1475 = f431(f432(), v2.clone());
    match m1475 {
        None => {
            let m1476 = f433(f434(), v2.clone());
            match m1476 {
                None => {
                    f435(v0, v1, v2)
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

pub fn f369(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt132) -> R<Adt144> {
    let m1477 = f154(Str::lit("x-mcp-header"), v1.clone());
    match m1477 {
        Adt123::C0 => {
            Ok(Adt144::C1(v4))
        }
        Adt123::C1(v5) => {
            f436(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f370(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt125>) -> List<Adt125> {
    f437(Str::lit("properties"), v0.clone(), v1, v2, f437(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f438(Str::lit("items"), v0.clone(), f438(Str::lit("additionalProperties"), v0.clone(), f439(Str::lit("allOf"), v0.clone(), f439(Str::lit("anyOf"), v0.clone(), f439(Str::lit("oneOf"), v0.clone(), f438(Str::lit("not"), v0.clone(), f438(Str::lit("if"), v0.clone(), f438(Str::lit("then"), v0.clone(), f438(Str::lit("else"), v0, v3)))))))))))
}

pub fn f371(p1484: List<(Str, Adt143)>) -> Adt133 {
    let m1483 = p1484;
    {
        let mut m1483 = m1483;
        loop {
            let step = {
                let v0 = m1483;
                let m1478 = v0;
                match m1478.uncons() {
                    None => {
                        Err::<List<(Str, Adt143)>, Adt133>(Adt133::C3(List::<Adt112>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1480 = {
                            let (_, h1479) = v1;
                            h1479
                        };
                        match m1480 {
                            Adt143::C0 => {
                                Ok::<List<(Str, Adt143)>, Adt133>(v2)
                            }
                            Adt143::C1 => {
                                Err::<List<(Str, Adt143)>, Adt133>(Adt133::C0({
                                    let (h1481, _) = v1;
                                    h1481
                                }))
                            }
                            Adt143::C2 => {
                                Err::<List<(Str, Adt143)>, Adt133>(Adt133::C1({
                                    let (h1482, _) = v1;
                                    h1482
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1483 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1485 = f154(Str::lit("type"), v0);
    match m1485 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1486 = f151(v2.clone());
            match m1486 {
                None => {
                    let m1487 = f310(v2);
                    match m1487 {
                        None => {
                            Ok(Adt143::C2)
                        }
                        Some(v4) => {
                            f442(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f440(v3.clone()) {
                        if f441(v3, v1)? {
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

pub fn f373(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1488 = f154(Str::lit("enum"), v0);
    match m1488 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1489 = f310(v2);
            match m1489 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v3) => {
                    f443(v3, v1)
                }
            }
        }
    }
}

pub fn f374(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1490 = f154(Str::lit("const"), v0);
    match m1490 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1491 = f444(v2, v1)?;
            match m1491 {
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

pub fn f375(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt143> {
    let m1492 = f154(v0, v2);
    match m1492 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v4) => {
            let m1493 = f445(v4);
            match m1493 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v5) => {
                    let m1494 = f445(v3);
                    match m1494 {
                        None => {
                            Ok(Adt143::C0)
                        }
                        Some(v6) => {
                            if f446(v1, f447(v6, v5)?) {
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

pub fn f376(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt143)>> {
    let m1495 = v1;
    match m1495 {
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
            Ok(List::cons((Str::lit("minLength"), f448(Str::lit("minLength"), true, v0.clone(), f449(v4.clone())?)?), List::cons((Str::lit("maxLength"), f448(Str::lit("maxLength"), false, v0, f449(v4)?)?), List::<(Str, Adt143)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f448(Str::lit("minItems"), true, v0.clone(), f450(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f448(Str::lit("maxItems"), false, v0, f450(v5, 0u64)?)?), List::<(Str, Adt143)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f448(Str::lit("minProperties"), true, v0.clone(), f451(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f448(Str::lit("maxProperties"), false, v0, f451(v6, 0u64)?)?), List::<(Str, Adt143)>::nil())))
        }
    }
}

pub fn f377(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt143 {
    let m1496 = f154(Str::lit("required"), v0);
    match m1496 {
        Adt123::C0 => {
            Adt143::C0
        }
        Adt123::C1(v2) => {
            let m1497 = f310(v2);
            match m1497 {
                None => {
                    Adt143::C2
                }
                Some(v3) => {
                    let m1498 = f239(v1);
                    match m1498 {
                        None => {
                            Adt143::C0
                        }
                        Some(v4) => {
                            f452(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f378(v0: Adt133, v1: Adt133) -> Adt133 {
    let m1499 = v0.clone();
    match m1499 {
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
            let m1500 = v1.clone();
            match m1500 {
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
                        let a1501 = v5;
                        let a1502 = v9;
                        append_list(a1501, a1502)
                    })
                }
            }
        }
    }
}

pub fn f379(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> Adt133 {
    let m1503 = f154(Str::lit("$ref"), v1);
    match m1503 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v4) => {
            let m1504 = f151(v4);
            match m1504 {
                None => {
                    Adt133::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1505 = f453(v0, v5.clone());
                    match m1505 {
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

pub fn f380(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1506 = f154(Str::lit("properties"), v0);
    match m1506 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1507 = f239(v3);
            match m1507 {
                None => {
                    Adt133::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1508 = f239(v1);
                    match m1508 {
                        None => {
                            Adt133::C3(List::<Adt112>::nil())
                        }
                        Some(v5) => {
                            Adt133::C3(f454(match v4 {
                                Adt5::C0(h1509) => {
                                    h1509
                                }
                            }, match v5 {
                                Adt5::C0(h1510) => {
                                    h1510
                                }
                            }, v2, List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f381(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1511 = f154(Str::lit("additionalProperties"), v0.clone());
    match m1511 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1512 = f239(v1);
            match m1512 {
                None => {
                    Adt133::C3(List::<Adt112>::nil())
                }
                Some(v4) => {
                    let m1513 = f238(v3.clone());
                    match m1513 {
                        None => {
                            Adt133::C3(f457(match v4 {
                                Adt5::C0(h1515) => {
                                    h1515
                                }
                            }, f456(v0), v3, v2, List::<Adt112>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt133::C3(List::<Adt112>::nil())
                            } else {
                                if f455(match v4 {
                                    Adt5::C0(h1514) => {
                                        h1514
                                    }
                                }, f456(v0)) {
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

pub fn f382(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> R<Adt133> {
    let m1516 = f154(Str::lit("items"), v0);
    match m1516 {
        Adt123::C0 => {
            Ok(Adt133::C3(List::<Adt112>::nil()))
        }
        Adt123::C1(v3) => {
            let m1517 = f310(v1);
            match m1517 {
                None => {
                    Ok(Adt133::C3(List::<Adt112>::nil()))
                }
                Some(v4) => {
                    Ok(Adt133::C3(f458(v4, 0u64, v3, v2, List::<Adt112>::nil())?))
                }
            }
        }
    }
}

pub fn f383(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1518 = f154(Str::lit("allOf"), v0);
    match m1518 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1519 = f310(v3);
            match m1519 {
                None => {
                    Adt133::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1520 = v4.clone();
                    match m1520.uncons() {
                        None => {
                            Adt133::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt133::C3(f459(v4, v1, v2, List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f384(v0: Adt113, v1: Adt133, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt110>) -> Adt133 {
    let m1521 = v1.clone();
    match m1521 {
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
            let m1522 = v8;
            match m1522.uncons() {
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

pub fn f385(p1527: List<Adt50>, p1528: Adt7) -> R<bool> {
    let m1526 = (p1527, p1528);
    {
        let mut m1526 = m1526;
        loop {
            let step = {
                let (v0, v1) = m1526;
                let m1523 = v0;
                match m1523.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f460(f447(match v2.clone() {
                            Adt50::C0(h1524, _, _) => {
                                h1524
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1525, _, _) => {
                                    h1525
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(p1535: List<u64>, p1536: List<u64>) -> List<u64> {
    let m1534 = (p1535, p1536);
    {
        let mut m1534 = m1534;
        loop {
            let step = {
                let (v0, v1) = m1534;
                let m1529 = v0;
                match m1529.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1530 = v2;
                        let a1531 = 126u64;
                        if nat_eq(a1530, a1531) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1532 = v2;
                            let a1533 = 47u64;
                            if nat_eq(a1532, a1533) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f387() -> i64 {
    999999999999999999i64
}

pub fn f388(v0: i64) -> R<List<u64>> {
    let m1539 = {
        let a1537 = v0;
        let a1538 = 0i64;
        compare(a1537, a1538)
    };
    if match m1539 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f461(20u64, {
            let a1540 = v0;
            int_neg(a1540)?
        }, List::<u64>::nil())?))
    } else {
        f461(20u64, v0, List::<u64>::nil())
    }
}

pub fn f389(v0: u64) -> R<List<u64>> {
    let a1541 = v0;
    let a1542 = 34u64;
    if nat_eq(a1541, a1542) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1543 = v0;
        let a1544 = 92u64;
        if nat_eq(a1543, a1544) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1545 = v0;
            let a1546 = 8u64;
            if nat_eq(a1545, a1546) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1547 = v0;
                let a1548 = 9u64;
                if nat_eq(a1547, a1548) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1549 = v0;
                    let a1550 = 10u64;
                    if nat_eq(a1549, a1550) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1551 = v0;
                        let a1552 = 12u64;
                        if nat_eq(a1551, a1552) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1553 = v0;
                            let a1554 = 13u64;
                            if nat_eq(a1553, a1554) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1555 = v0;
                                let a1556 = 32u64;
                                if nat_lt(a1555, a1556) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f462(f367(v0, 16u64))?, List::cons(f462(f366(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f390(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1557 = v1;
    match m1557 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f463(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f391(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1558 = v0;
    match m1558 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1561 = f326(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1559, _) => {
                        h1559
                    }
                });
                let a1562 = f326(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1560) => {
                        h1560
                    }
                });
                append_list(a1561, a1562)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f392(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f464(v0, 0i64)?, 0i64)))
}

pub fn f393(v0: Adt11) -> Str {
    let m1563 = v0;
    match m1563 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f394(p1566: List<Adt35>, p1567: List<Adt6>) -> List<Adt6> {
    let m1565 = (p1566, p1567);
    {
        let mut m1565 = m1565;
        loop {
            let step = {
                let (v0, v1) = m1565;
                let m1564 = v0;
                match m1564.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f465(v2), v1)))
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

pub fn f395(p1571: List<Adt38>, p1572: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1570 = (p1571, p1572);
    {
        let mut m1570 = m1570;
        loop {
            let step = {
                let (v0, v1) = m1570;
                let m1568 = v0;
                match m1568.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1569 = f466(v2)?;
                        match m1569 {
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
                Ok(next) => { m1570 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(p1575: List<Adt44>, p1576: List<Adt6>) -> R<List<Adt6>> {
    let m1574 = (p1575, p1576);
    {
        let mut m1574 = m1574;
        loop {
            let step = {
                let (v0, v1) = m1574;
                let m1573 = v0;
                match m1573.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f467(v2)?, v1)))
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

pub fn f397(p1579: List<Adt46>, p1580: List<Adt6>) -> List<Adt6> {
    let m1578 = (p1579, p1580);
    {
        let mut m1578 = m1578;
        loop {
            let step = {
                let (v0, v1) = m1578;
                let m1577 = v0;
                match m1577.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f468(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1578 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(p1583: List<Adt54>, p1584: List<Adt6>) -> List<Adt6> {
    let m1582 = (p1583, p1584);
    {
        let mut m1582 = m1582;
        loop {
            let step = {
                let (v0, v1) = m1582;
                let m1581 = v0;
                match m1581.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f469(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(p1587: List<Adt56>, p1588: List<Adt6>) -> List<Adt6> {
    let m1586 = (p1587, p1588);
    {
        let mut m1586 = m1586;
        loop {
            let step = {
                let (v0, v1) = m1586;
                let m1585 = v0;
                match m1585.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f470(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1586 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(p1592: List<Adt59>, p1593: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1591 = (p1592, p1593);
    {
        let mut m1591 = m1591;
        loop {
            let step = {
                let (v0, v1) = m1591;
                let m1589 = v0;
                match m1589.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1590 = f471(v2)?;
                        match m1590 {
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
                Ok(next) => { m1591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1594 = v0;
    match m1594 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f472(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f402(v0: Str) -> R<Str> {
    let m1597 = {
        let a1596 = f285(f286(), f129(f473(f55({
            let a1595 = v0;
            utf8_encode(a1595)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1596)
    };
    match m1597 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f403(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt116, Adt119>> {
    if f474(f475(v3.clone())) {
        Ok(Ok::<Adt116, Adt119>(f476(v0.clone(), Adt32::C0(f477(v0)))))
    } else {
        Ok(f478(v0.clone(), v3.clone(), f479(match v0 {
            Adt0::C0(_, _, _, _, _, h1598) => {
                h1598
            }
        }, v2, v1, v3)?))
    }
}

pub fn f404(v0: Adt16, _: Adt5) -> Result<Adt14, Adt120> {
    Ok::<Adt14, Adt120>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f405(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1599 = f480(Str::lit("cursor"), v1);
    match m1599 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f406(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1600 = f481(Str::lit("name"), v1.clone());
    match m1600 {
        Ok(v2) => {
            let m1601 = f482(Str::lit("arguments"), v1.clone());
            match m1601 {
                Ok(v3) => {
                    let m1602 = f482(Str::lit("inputResponses"), v1.clone());
                    match m1602 {
                        Ok(v4) => {
                            let m1603 = f480(Str::lit("requestState"), v1);
                            match m1603 {
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

pub fn f407(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1604 = f480(Str::lit("cursor"), v1);
    match m1604 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f408(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1605 = f481(Str::lit("uri"), v1.clone());
    match m1605 {
        Ok(v2) => {
            let m1606 = f482(Str::lit("inputResponses"), v1.clone());
            match m1606 {
                Ok(v3) => {
                    let m1607 = f480(Str::lit("requestState"), v1);
                    match m1607 {
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

pub fn f409(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1608 = f480(Str::lit("cursor"), v1);
    match m1608 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f410(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1609 = f480(Str::lit("cursor"), v1);
    match m1609 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f411(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1610 = f481(Str::lit("name"), v1.clone());
    match m1610 {
        Ok(v2) => {
            let m1611 = f483(Str::lit("arguments"), v1.clone());
            match m1611 {
                Ok(v3) => {
                    let m1612 = f482(Str::lit("inputResponses"), v1.clone());
                    match m1612 {
                        Ok(v4) => {
                            let m1613 = f480(Str::lit("requestState"), v1);
                            match m1613 {
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

pub fn f412(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1614 = f484(Str::lit("ref"), v1.clone());
    match m1614 {
        Ok(v2) => {
            let m1615 = f485(v2);
            match m1615 {
                Ok(v3) => {
                    let m1616 = f484(Str::lit("argument"), v1.clone());
                    match m1616 {
                        Ok(v4) => {
                            let m1617 = f486(v4);
                            match m1617 {
                                Ok(v5) => {
                                    let m1618 = f487(v1);
                                    match m1618 {
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

pub fn f413(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1619 = f481(Str::lit("taskId"), v1);
    match m1619 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f414(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1620 = f481(Str::lit("taskId"), v1.clone());
    match m1620 {
        Ok(v2) => {
            let m1621 = f484(Str::lit("inputResponses"), v1);
            match m1621 {
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

pub fn f415(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1622 = f481(Str::lit("taskId"), v1);
    match m1622 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f416(v0: Adt95, v1: Adt16, v2: Option<Adt119>) -> Adt136 {
    let m1623 = v2;
    match m1623 {
        None => {
            Adt136::C0(Adt137::C0(match v0.clone() {
                Adt95::C0(h1624, _, _) => {
                    h1624
                }
            }, match v0.clone() {
                Adt95::C0(_, h1625, _) => {
                    h1625
                }
            }, v1, f355(match v0 {
                Adt95::C0(_, _, h1626) => {
                    h1626
                }
            })))
        }
        Some(v3) => {
            Adt136::C1(v3)
        }
    }
}

pub fn f417(v0: Str, v1: List<Str>) -> Option<Adt119> {
    f488(f489(v0.clone(), v1.clone()), v0, v1)
}

pub fn f418(v0: Adt96, v1: Adt138, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt97>> {
    let m1628 = match v0.clone() {
        Adt96::C0(h1627, _, _, _, _) => {
            h1627
        }
    };
    match m1628 {
        None => {
            Ok(Some(Adt97::C0))
        }
        Some(v5) => {
            let a1629 = v5;
            let a1630 = v3;
            if equal(a1629, a1630) {
                let m1632 = match v0.clone() {
                    Adt96::C0(_, h1631, _, _, _) => {
                        h1631
                    }
                };
                match m1632 {
                    None => {
                        Ok(Some(Adt97::C1))
                    }
                    Some(v6) => {
                        let a1633 = v6;
                        let a1634 = v2;
                        if equal(a1633, a1634) {
                            f490(f491(match v0.clone() {
                                Adt96::C0(_, _, h1635, _, _) => {
                                    h1635
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt96::C0(_, _, _, _, h1636) => {
                                    h1636
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

pub fn f419() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f420(v0: Adt5) -> Adt145 {
    f492(f111(f422(), v0.clone()), f111(f423(), v0))
}

pub fn f421(v0: Adt5, v1: Adt123, v2: Adt123) -> Result<Adt16, Adt120> {
    let m1637 = v1;
    match m1637 {
        Adt123::C0 => {
            Err::<Adt16, Adt120>(Adt120::C2)
        }
        Adt123::C1(v3) => {
            let m1638 = v2;
            match m1638 {
                Adt123::C0 => {
                    Err::<Adt16, Adt120>(Adt120::C3)
                }
                Adt123::C1(v4) => {
                    f493(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f422() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f423() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f424(p1647: Bytes, p1648: List<Bytes>) -> List<Bytes> {
    let m1646 = (p1647, p1648);
    {
        let mut m1646 = m1646;
        loop {
            let step = {
                let (v0, v1) = m1646;
                let m1639 = v1;
                match m1639.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1642 = {
                            let a1640 = v2.clone();
                            length_bytes(a1640)
                        };
                        let a1643 = {
                            let a1641 = v0.clone();
                            length_bytes(a1641)
                        };
                        if nat_eq(a1642, a1643) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1644 = v2;
                                let a1645 = v0;
                                append_bytes(a1644, a1645)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1646 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(v0: u64) -> Bytes {
    let m1652 = {
        let a1649 = f494();
        let a1650 = v0;
        let a1651 = 1u64;
        slice_bytes(a1649, a1650, a1651)
    };
    match m1652 {
        None => {
            f286()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f426(v0: u64) -> bool {
    let a1653 = v0;
    let a1654 = 9u64;
    if nat_eq(a1653, a1654) {
        true
    } else {
        let a1655 = v0;
        let a1656 = 32u64;
        nat_eq(a1655, a1656)
    }
}

pub fn f427(v0: u64) -> R<Option<u64>> {
    let a1657 = 64u64;
    let a1658 = v0;
    if if nat_lt(a1657, a1658) {
        let a1659 = v0;
        let a1660 = 91u64;
        nat_lt(a1659, a1660)
    } else {
        false
    } {
        Ok(Some({
            let a1661 = v0;
            let a1662 = 65u64;
            nat_sub(a1661, a1662)
        }))
    } else {
        let a1663 = 96u64;
        let a1664 = v0;
        if if nat_lt(a1663, a1664) {
            let a1665 = v0;
            let a1666 = 123u64;
            nat_lt(a1665, a1666)
        } else {
            false
        } {
            Ok(Some({
                let a1667 = v0;
                let a1668 = 71u64;
                nat_sub(a1667, a1668)
            }))
        } else {
            let a1669 = 47u64;
            let a1670 = v0;
            if if nat_lt(a1669, a1670) {
                let a1671 = v0;
                let a1672 = 58u64;
                nat_lt(a1671, a1672)
            } else {
                false
            } {
                Ok(Some({
                    let a1673 = v0;
                    let a1674 = 4u64;
                    nat_add(a1673, a1674)?
                }))
            } else {
                let a1675 = v0;
                let a1676 = 43u64;
                if nat_eq(a1675, a1676) {
                    Ok(Some(62u64))
                } else {
                    let a1677 = v0;
                    let a1678 = 47u64;
                    if nat_eq(a1677, a1678) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f428(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1679 = v3;
        let a1680 = 61u64;
        if nat_eq(a1679, a1680) {
            let a1681 = f495(v1, 16u64);
            let a1682 = 0u64;
            nat_eq(a1681, a1682)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f496(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f429(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1683 = f495(v2, 4u64);
        let a1684 = 0u64;
        nat_eq(a1683, a1684)
    } else {
        false
    } {
        Ok(Some(f497(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f430(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1687 = {
            let a1685 = v0;
            let a1686 = 4u64;
            nat_mul(a1685, a1686)?
        };
        let a1688 = f498(v1, 16u64);
        nat_add(a1687, a1688)?
    }, List::cons({
        let a1691 = {
            let a1689 = f495(v1, 16u64);
            let a1690 = 16u64;
            nat_mul(a1689, a1690)?
        };
        let a1692 = f498(v2, 4u64);
        nat_add(a1691, a1692)?
    }, List::cons({
        let a1695 = {
            let a1693 = f495(v2, 4u64);
            let a1694 = 64u64;
            nat_mul(a1693, a1694)?
        };
        let a1696 = v3;
        nat_add(a1695, a1696)?
    }, List::<u64>::nil()))))
}

pub fn f431(p1700: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1699 = p1700;
    {
        let mut m1699 = m1699;
        loop {
            let step = {
                let v0 = m1699;
                let m1697 = v0;
                match m1697.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1698 = f154(v2.clone(), v1.clone());
                        match m1698 {
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
                Ok(next) => { m1699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f433(p1707: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1706 = p1707;
    {
        let mut m1706 = m1706;
        loop {
            let step = {
                let v0 = m1706;
                let m1701 = v0;
                match m1701.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1703 = f154({
                            let (h1702, _) = v2.clone();
                            h1702
                        }, v1.clone());
                        match m1703 {
                            Adt123::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt123::C1(v4) => {
                                if f499({
                                    let (_, h1704) = v2;
                                    h1704
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1705, _) = v2;
                                        h1705
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f435(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1708 = f154(Str::lit("$ref"), v2);
    match m1708 {
        Adt123::C0 => {
            None::<Adt94>
        }
        Adt123::C1(v3) => {
            let m1709 = f151(v3);
            match m1709 {
                None => {
                    None::<Adt94>
                }
                Some(v4) => {
                    let m1710 = f453(v1, v4.clone());
                    match m1710 {
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

pub fn f436(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt132) -> R<Adt144> {
    if v3 {
        let m1711 = f151(v2);
        match m1711 {
            None => {
                Ok(Adt144::C0(Adt94::C7(v0)))
            }
            Some(v6) => {
                f500(v0, v1, v6.clone(), f121(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt144::C0(Adt94::C10(v0)))
    }
}

pub fn f437(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt125>) -> List<Adt125> {
    let m1712 = f154(v0.clone(), v1);
    match m1712 {
        Adt123::C0 => {
            v4
        }
        Adt123::C1(v5) => {
            let m1713 = f239(v5);
            match m1713 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1714 = v0;
                    let a1715 = Str::lit("properties");
                    if equal(a1714, a1715) {
                        f501(match v6 {
                            Adt5::C0(h1716) => {
                                h1716
                            }
                        }, v2, v3, v4)
                    } else {
                        f502(match v6 {
                            Adt5::C0(h1717) => {
                                h1717
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f438(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt125>) -> List<Adt125> {
    let m1718 = f154(v0, v1);
    match m1718 {
        Adt123::C0 => {
            v2
        }
        Adt123::C1(v3) => {
            List::cons(Adt125::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f439(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt125>) -> List<Adt125> {
    let m1719 = f154(v0, v1);
    match m1719 {
        Adt123::C0 => {
            v2
        }
        Adt123::C1(v3) => {
            let m1720 = f310(v3);
            match m1720 {
                None => {
                    v2
                }
                Some(v4) => {
                    f503(v4, v2)
                }
            }
        }
    }
}

pub fn f440(v0: Str) -> bool {
    let a1721 = v0.clone();
    let a1722 = Str::lit("null");
    if equal(a1721, a1722) {
        true
    } else {
        let a1723 = v0.clone();
        let a1724 = Str::lit("boolean");
        if equal(a1723, a1724) {
            true
        } else {
            let a1725 = v0.clone();
            let a1726 = Str::lit("object");
            if equal(a1725, a1726) {
                true
            } else {
                let a1727 = v0.clone();
                let a1728 = Str::lit("array");
                if equal(a1727, a1728) {
                    true
                } else {
                    let a1729 = v0.clone();
                    let a1730 = Str::lit("number");
                    if equal(a1729, a1730) {
                        true
                    } else {
                        let a1731 = v0.clone();
                        let a1732 = Str::lit("string");
                        if equal(a1731, a1732) {
                            true
                        } else {
                            let a1733 = v0;
                            let a1734 = Str::lit("integer");
                            equal(a1733, a1734)
                        }
                    }
                }
            }
        }
    }
}

pub fn f441(v0: Str, v1: Adt6) -> R<bool> {
    let a1735 = v0.clone();
    let a1736 = f504(v1.clone());
    if equal(a1735, a1736) {
        Ok(true)
    } else {
        let a1737 = v0;
        let a1738 = Str::lit("integer");
        if equal(a1737, a1738) {
            let m1739 = f445(v1);
            match m1739 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f505(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f442(p1742: List<Adt6>, v1: Adt6, p1743: bool) -> R<Adt143> {
    let m1741 = (p1742, p1743);
    {
        let mut m1741 = m1741;
        loop {
            let step = {
                let (v0, v2) = m1741;
                let m1740 = v0;
                match m1740.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f440(f506(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt143>>((v4, if v2 {
                                true
                            } else {
                                f441(f506(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(p1747: List<Adt6>, v1: Adt6) -> R<Adt143> {
    let m1746 = p1747;
    {
        let mut m1746 = m1746;
        loop {
            let step = {
                let v0 = m1746;
                let m1744 = v0;
                match m1744.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt143>>(Ok(Adt143::C1))
                    }
                    Some((v2, v3)) => {
                        let m1745 = f444(v2, v1.clone())?;
                        match m1745 {
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
                Ok(next) => { m1746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f444(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f507(f508(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f445(v0: Adt6) -> Option<Adt7> {
    let m1748 = v0;
    match m1748 {
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

pub fn f446(v0: u64, v1: Adt147) -> bool {
    let a1749 = v0;
    let a1750 = 0u64;
    if nat_eq(a1749, a1750) {
        let a1751 = f509(v1);
        bool_not(a1751)
    } else {
        let a1752 = v0;
        let a1753 = 1u64;
        if nat_eq(a1752, a1753) {
            let a1754 = f460(v1);
            bool_not(a1754)
        } else {
            let a1755 = v0;
            let a1756 = 2u64;
            if nat_eq(a1755, a1756) {
                f460(v1)
            } else {
                f509(v1)
            }
        }
    }
}

pub fn f447(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f510(v0.clone(), v1.clone(), f511(match v0 {
        Adt7::C0(h1757, _) => {
            h1757
        }
    }), f511(match v1 {
        Adt7::C0(h1758, _) => {
            h1758
        }
    }))
}

pub fn f448(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt143> {
    let m1759 = f154(v0, v2);
    match m1759 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v4) => {
            let m1760 = f512(v4);
            match m1760 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v5) => {
                    let m1763 = {
                        let a1761 = v5;
                        let a1762 = 0i64;
                        compare(a1761, a1762)
                    };
                    if match m1763 {
                        Ordering::Less => {
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
                            if f509(f513(f89(v3, 0i64)?, v5)) {
                                Ok(Adt143::C1)
                            } else {
                                Ok(Adt143::C0)
                            }
                        } else {
                            if f460(f513(f89(v3, 0i64)?, v5)) {
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

pub fn f449(v0: Str) -> R<u64> {
    f514(f55({
        let a1764 = v0;
        utf8_encode(a1764)
    })?, 0u64)
}

pub fn f450(p1769: List<Adt6>, p1770: u64) -> R<u64> {
    let m1768 = (p1769, p1770);
    {
        let mut m1768 = m1768;
        loop {
            let step = {
                let (v0, v1) = m1768;
                let m1765 = v0;
                match m1765.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1766 = v1;
                            let a1767 = 1u64;
                            nat_add(a1766, a1767)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f451(p1775: List<(Str, Adt6)>, p1776: u64) -> R<u64> {
    let m1774 = (p1775, p1776);
    {
        let mut m1774 = m1774;
        loop {
            let step = {
                let (v0, v1) = m1774;
                let m1771 = v0;
                match m1771.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1772 = v1;
                            let a1773 = 1u64;
                            nat_add(a1772, a1773)?
                        }))
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

pub fn f452(p1780: List<Adt6>, v1: Adt5) -> Adt143 {
    let m1779 = p1780;
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let v0 = m1779;
                let m1777 = v0;
                match m1777.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt143>(Adt143::C0)
                    }
                    Some((v2, v3)) => {
                        let m1778 = f151(v2);
                        match m1778 {
                            None => {
                                Err::<List<Adt6>, Adt143>(Adt143::C2)
                            }
                            Some(v4) => {
                                if f155(v4, v1.clone()) {
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
                Ok(next) => { m1779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1781 = v1.clone();
    let a1782 = Str::lit("#");
    if equal(a1781, a1782) {
        Some(v0)
    } else {
        let m1783 = f239(v0);
        match m1783 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1784 = f111(Str::lit("$defs"), v2);
                match m1784 {
                    Adt123::C0 => {
                        None::<Adt6>
                    }
                    Adt123::C1(v3) => {
                        let m1785 = f239(v3);
                        match m1785 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f515(v1, match v4 {
                                    Adt5::C0(h1786) => {
                                        h1786
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

pub fn f454(p1793: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt110>, p1794: List<Adt112>) -> List<Adt112> {
    let m1792 = (p1793, p1794);
    {
        let mut m1792 = m1792;
        loop {
            let step = {
                let (v0, v3) = m1792;
                let m1787 = v0;
                match m1787.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1789 = f154({
                            let (h1788, _) = v4.clone();
                            h1788
                        }, v1.clone());
                        match m1789 {
                            Adt123::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v5, v3))
                            }
                            Adt123::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0({
                                    let (_, h1790) = v4;
                                    h1790
                                }, v6, List::cons(Adt110::C0({
                                    let (h1791, _) = v4;
                                    h1791
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(p1799: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1798 = p1799;
    {
        let mut m1798 = m1798;
        loop {
            let step = {
                let v0 = m1798;
                let m1795 = v0;
                match m1795.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1797 = f154({
                            let (h1796, _) = v2;
                            h1796
                        }, v1.clone());
                        match m1797 {
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
                Ok(next) => { m1798 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1800 = f154(Str::lit("properties"), v0);
    match m1800 {
        Adt123::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt123::C1(v1) => {
            let m1801 = f239(v1);
            match m1801 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1802) => {
                            h1802
                        }
                    }
                }
            }
        }
    }
}

pub fn f457(p1809: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>, p1810: List<Adt112>) -> List<Adt112> {
    let m1808 = (p1809, p1810);
    {
        let mut m1808 = m1808;
        loop {
            let step = {
                let (v0, v4) = m1808;
                let m1803 = v0;
                match m1803.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1805 = f154({
                            let (h1804, _) = v5.clone();
                            h1804
                        }, v1.clone());
                        match m1805 {
                            Adt123::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v6, List::cons(Adt112::C0(v2.clone(), {
                                    let (_, h1806) = v5;
                                    h1806
                                }, List::cons(Adt110::C0({
                                    let (h1807, _) = v5;
                                    h1807
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
                Ok(next) => { m1808 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(p1815: List<Adt6>, p1816: u64, v2: Adt6, v3: List<Adt110>, p1817: List<Adt112>) -> R<List<Adt112>> {
    let m1814 = (p1815, (p1816, p1817));
    {
        let mut m1814 = m1814;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1814;
                let m1811 = v0;
                match m1811.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt112>)), R<List<Adt112>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt112>)), R<List<Adt112>>>((v6, ({
                            let a1812 = v1;
                            let a1813 = 1u64;
                            nat_add(a1812, a1813)?
                        }, List::cons(Adt112::C0(v2.clone(), v5, List::cons(Adt110::C1(v1), v3.clone())), v4))))
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

pub fn f459(p1820: List<Adt6>, v1: Adt6, v2: List<Adt110>, p1821: List<Adt112>) -> List<Adt112> {
    let m1819 = (p1820, p1821);
    {
        let mut m1819 = m1819;
        loop {
            let step = {
                let (v0, v3) = m1819;
                let m1818 = v0;
                match m1818.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0(v4, v1.clone(), v2.clone()), v3)))
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

pub fn f460(v0: Adt147) -> bool {
    let m1822 = v0;
    match m1822 {
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

pub fn f461(p1834: u64, p1835: i64, p1836: List<u64>) -> R<List<u64>> {
    let m1833 = (p1834, (p1835, p1836));
    {
        let mut m1833 = m1833;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1833;
                let m1823 = v0;
                if m1823 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1823 - 1;
                    let v4: i64 = {
                        let a1824 = v1;
                        let a1825 = 10i64;
                        let a1826 = 0i64;
                        int_quot(a1824, a1825, a1826)?
                    };
                    let v5: i64 = {
                        let a1827 = v1;
                        let a1828 = 10i64;
                        let a1829 = 0i64;
                        int_rem(a1827, a1828, a1829)
                    };
                    let m1832 = {
                        let a1830 = v4;
                        let a1831 = 1i64;
                        compare(a1830, a1831)
                    };
                    if match m1832 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f516(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f516(v5), v2))))
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

pub fn f462(v0: u64) -> R<u64> {
    let a1837 = v0;
    let a1838 = 10u64;
    if nat_lt(a1837, a1838) {
        let a1839 = 48u64;
        let a1840 = v0;
        nat_add(a1839, a1840)
    } else {
        let a1841 = 87u64;
        let a1842 = v0;
        nat_add(a1841, a1842)
    }
}

pub fn f463(v0: Adt8) -> Adt6 {
    Adt6::C5(f326(Str::lit("listChanged"), match v0 {
        Adt8::C0(h1843) => {
            h1843
        }
    }))
}

pub fn f464(p1848: u64, p1849: i64) -> R<i64> {
    let m1847 = (p1848, p1849);
    {
        let mut m1847 = m1847;
        loop {
            let step = {
                let (v0, v1) = m1847;
                let m1844 = v0;
                if m1844 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m1844 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a1845 = v1;
                        let a1846 = -1i64;
                        int_sub(a1845, a1846)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h1850, _, _, _, _, _, _, _) => {
            h1850
        }
    })), {
        let a1868 = f321(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h1851, _, _, _, _, _, _) => {
                h1851
            }
        });
        let a1869 = {
            let a1866 = f401(match v0.clone() {
                Adt35::C0(_, _, h1852, _, _, _, _, _) => {
                    h1852
                }
            });
            let a1867 = {
                let a1864 = f321(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h1853, _, _, _, _) => {
                        h1853
                    }
                });
                let a1865 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m1856 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h1855, _, _, _) => {
                            h1855
                        }
                    };
                    match m1856 {
                        Adt5::C0(h1854) => {
                            h1854
                        }
                    }
                })), {
                    let a1862 = f344(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h1857, _, _) => {
                            h1857
                        }
                    });
                    let a1863 = {
                        let a1860 = f517(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h1858, _) => {
                                h1858
                            }
                        });
                        let a1861 = f518(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h1859) => {
                                h1859
                            }
                        });
                        append_list(a1860, a1861)
                    };
                    append_list(a1862, a1863)
                });
                append_list(a1864, a1865)
            };
            append_list(a1866, a1867)
        };
        append_list(a1868, a1869)
    }))
}

pub fn f466(v0: Adt38) -> R<Option<Adt6>> {
    let m1870 = v0;
    match m1870 {
        Adt38::C0(v1) => {
            Ok(Some(f519(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f520(Str::lit("image"), match v2.clone() {
                Adt42::C0(h1871, _, _, _) => {
                    h1871
                }
            }, match v2.clone() {
                Adt42::C0(_, h1872, _, _) => {
                    h1872
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h1873, _) => {
                    h1873
                }
            }, match v2 {
                Adt42::C0(_, _, _, h1874) => {
                    h1874
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f520(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h1875, _, _, _) => {
                    h1875
                }
            }, match v3.clone() {
                Adt43::C0(_, h1876, _, _) => {
                    h1876
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h1877, _) => {
                    h1877
                }
            }, match v3 {
                Adt43::C0(_, _, _, h1878) => {
                    h1878
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f521(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f522(v5)))
        }
    }
}

pub fn f467(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a1902 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h1879, _, _, _, _, _, _, _, _) => {
                h1879
            }
        })), List::<(Str, Adt6)>::nil());
        let a1903 = {
            let a1900 = f321(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h1880, _, _, _, _, _, _, _) => {
                    h1880
                }
            });
            let a1901 = {
                let a1898 = f401(match v0.clone() {
                    Adt44::C0(_, _, h1881, _, _, _, _, _, _) => {
                        h1881
                    }
                });
                let a1899 = {
                    let a1896 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h1882, _, _, _, _, _) => {
                            h1882
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a1897 = {
                        let a1894 = f321(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h1883, _, _, _, _) => {
                                h1883
                            }
                        });
                        let a1895 = {
                            let a1892 = f321(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h1884, _, _, _) => {
                                    h1884
                                }
                            });
                            let a1893 = {
                                let a1890 = f523(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h1885, _, _) => {
                                        h1885
                                    }
                                });
                                let a1891 = {
                                    let a1888 = f524(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h1886, _) => {
                                            h1886
                                        }
                                    })?;
                                    let a1889 = f518(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h1887) => {
                                            h1887
                                        }
                                    });
                                    append_list(a1888, a1889)
                                };
                                append_list(a1890, a1891)
                            };
                            append_list(a1892, a1893)
                        };
                        append_list(a1894, a1895)
                    };
                    append_list(a1896, a1897)
                };
                append_list(a1898, a1899)
            };
            append_list(a1900, a1901)
        };
        append_list(a1902, a1903)
    }))
}

pub fn f468(v0: Adt46) -> Adt6 {
    let m1904 = v0;
    match m1904 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h1905, _, _, _) => {
                    h1905
                }
            })), {
                let a1909 = f321(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h1906, _, _) => {
                        h1906
                    }
                });
                let a1910 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h1907, _) => {
                        h1907
                    }
                })), f518(match v1 {
                    Adt47::C0(_, _, _, h1908) => {
                        h1908
                    }
                }));
                append_list(a1909, a1910)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h1911, _, _, _) => {
                    h1911
                }
            })), {
                let a1915 = f321(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h1912, _, _) => {
                        h1912
                    }
                });
                let a1916 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h1913, _) => {
                        h1913
                    }
                })), f518(match v2 {
                    Adt48::C0(_, _, _, h1914) => {
                        h1914
                    }
                }));
                append_list(a1915, a1916)
            }))
        }
    }
}

pub fn f469(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a1937 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h1917, _, _, _, _, _, _, _) => {
                h1917
            }
        })), List::<(Str, Adt6)>::nil());
        let a1938 = {
            let a1935 = f321(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h1918, _, _, _, _, _, _) => {
                    h1918
                }
            });
            let a1936 = {
                let a1933 = f401(match v0.clone() {
                    Adt54::C0(_, _, h1919, _, _, _, _, _) => {
                        h1919
                    }
                });
                let a1934 = {
                    let a1931 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h1920, _, _, _, _) => {
                            h1920
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a1932 = {
                        let a1929 = f321(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h1921, _, _, _) => {
                                h1921
                            }
                        });
                        let a1930 = {
                            let a1927 = f321(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h1922, _, _) => {
                                    h1922
                                }
                            });
                            let a1928 = {
                                let a1925 = f523(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h1923, _) => {
                                        h1923
                                    }
                                });
                                let a1926 = f518(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h1924) => {
                                        h1924
                                    }
                                });
                                append_list(a1925, a1926)
                            };
                            append_list(a1927, a1928)
                        };
                        append_list(a1929, a1930)
                    };
                    append_list(a1931, a1932)
                };
                append_list(a1933, a1934)
            };
            append_list(a1935, a1936)
        };
        append_list(a1937, a1938)
    })
}

pub fn f470(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a1954 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h1939, _, _, _, _, _) => {
                h1939
            }
        })), List::<(Str, Adt6)>::nil());
        let a1955 = {
            let a1952 = f321(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h1940, _, _, _, _) => {
                    h1940
                }
            });
            let a1953 = {
                let a1950 = f401(match v0.clone() {
                    Adt56::C0(_, _, h1941, _, _, _) => {
                        h1941
                    }
                });
                let a1951 = {
                    let a1948 = f321(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h1942, _, _) => {
                            h1942
                        }
                    });
                    let a1949 = {
                        let a1946 = {
                            let m1944 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h1943, _) => {
                                    h1943
                                }
                            };
                            match m1944 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f525(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a1947 = f518(match v0 {
                            Adt56::C0(_, _, _, _, _, h1945) => {
                                h1945
                            }
                        });
                        append_list(a1946, a1947)
                    };
                    append_list(a1948, a1949)
                };
                append_list(a1950, a1951)
            };
            append_list(a1952, a1953)
        };
        append_list(a1954, a1955)
    })
}

pub fn f471(v0: Adt59) -> R<Option<Adt6>> {
    let m1957 = f466(match v0.clone() {
        Adt59::C0(_, h1956) => {
            h1956
        }
    })?;
    match m1957 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f526(match v0 {
                Adt59::C0(h1958, _) => {
                    h1958
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f472(v0: List<Adt2>) -> List<Adt6> {
    f214(f527(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f473(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f528(v0, 0u64, v1)
}

pub fn f474(v0: Adt128) -> bool {
    let m1959 = v0;
    match m1959 {
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

pub fn f475(v0: Adt14) -> Adt128 {
    let m1960 = v0;
    match m1960 {
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

pub fn f476(v0: Adt0, v1: Adt32) -> Adt116 {
    Adt116::C0(v1, Adt117::C0(Some(match v0 {
        Adt0::C0(h1961, _, _, _, _, _) => {
            h1961
        }
    }), None::<Adt5>), List::<Adt118>::nil())
}

pub fn f477(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h1962, _, _) => {
            h1962
        }
    }, f109(), match v0.clone() {
        Adt0::C0(_, _, h1963, _, _, _) => {
            h1963
        }
    }, match v0 {
        Adt0::C0(_, h1964, _, _, _, _) => {
            h1964
        }
    })
}

pub fn f478(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt116, Adt119> {
    let m1965 = v2;
    match m1965 {
        Ok(v4) => {
            if f529(f475(v1.clone()), v4.clone()) {
                f530(v0, v1, v4)
            } else {
                Err::<Adt116, Adt119>(Adt119::C4(f531()))
            }
        }
        Err(v3) => {
            Err::<Adt116, Adt119>(Adt119::C4(v3))
        }
    }
}

pub fn f479(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m1966 = v3.clone();
    match m1966 {
        Adt14::C0(_) => {
            let c1968: Fn0 = v2;
            c1968.apply(v3)
        }
        Adt14::C1(_) => {
            let c1969: Fn0 = v2;
            c1969.apply(v3)
        }
        Adt14::C2(_) => {
            if f532(v0, v1) {
                let c1967: Fn0 = v2;
                c1967.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c1970: Fn0 = v2;
            c1970.apply(v3)
        }
        Adt14::C4(_) => {
            let c1971: Fn0 = v2;
            c1971.apply(v3)
        }
        Adt14::C5(_) => {
            let c1972: Fn0 = v2;
            c1972.apply(v3)
        }
        Adt14::C6(_) => {
            let c1973: Fn0 = v2;
            c1973.apply(v3)
        }
        Adt14::C7(_) => {
            let c1974: Fn0 = v2;
            c1974.apply(v3)
        }
        Adt14::C8(_) => {
            let c1975: Fn0 = v2;
            c1975.apply(v3)
        }
        Adt14::C9(_) => {
            let c1976: Fn0 = v2;
            c1976.apply(v3)
        }
        Adt14::C10(_) => {
            let c1977: Fn0 = v2;
            c1977.apply(v3)
        }
        Adt14::C11(_) => {
            let c1978: Fn0 = v2;
            c1978.apply(v3)
        }
    }
}

pub fn f480(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt120> {
    let m1979 = f111(v0.clone(), v1);
    match m1979 {
        Adt123::C0 => {
            Ok::<Option<Str>, Adt120>(None::<Str>)
        }
        Adt123::C1(v2) => {
            let m1980 = f151(v2);
            match m1980 {
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

pub fn f481(v0: Str, v1: Adt5) -> Result<Str, Adt120> {
    let m1981 = f111(v0.clone(), v1);
    match m1981 {
        Adt123::C0 => {
            Err::<Str, Adt120>(Adt120::C7(v0))
        }
        Adt123::C1(v2) => {
            let m1982 = f151(v2);
            match m1982 {
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

pub fn f482(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt120> {
    let m1983 = f111(v0.clone(), v1);
    match m1983 {
        Adt123::C0 => {
            Ok::<Option<Adt5>, Adt120>(None::<Adt5>)
        }
        Adt123::C1(v2) => {
            let m1984 = f239(v2);
            match m1984 {
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

pub fn f483(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt120> {
    let m1985 = f111(v0.clone(), v1);
    match m1985 {
        Adt123::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt120>(None::<List<(Str, Str)>>)
        }
        Adt123::C1(v2) => {
            let m1986 = f239(v2);
            match m1986 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt120>(Adt120::C8(v0))
                }
                Some(v3) => {
                    let m1988 = f533(v0, match v3 {
                        Adt5::C0(h1987) => {
                            h1987
                        }
                    });
                    match m1988 {
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

pub fn f484(v0: Str, v1: Adt5) -> Result<Adt5, Adt120> {
    let m1989 = f111(v0.clone(), v1);
    match m1989 {
        Adt123::C0 => {
            Err::<Adt5, Adt120>(Adt120::C7(v0))
        }
        Adt123::C1(v2) => {
            let m1990 = f239(v2);
            match m1990 {
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

pub fn f485(v0: Adt5) -> Result<Adt25, Adt120> {
    let m1991 = f481(Str::lit("type"), v0.clone());
    match m1991 {
        Ok(v1) => {
            let a1992 = v1.clone();
            let a1993 = Str::lit("ref/prompt");
            if equal(a1992, a1993) {
                let m1994 = f481(Str::lit("name"), v0.clone());
                match m1994 {
                    Ok(v2) => {
                        let m1995 = f480(Str::lit("title"), v0);
                        match m1995 {
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
                let a1996 = v1;
                let a1997 = Str::lit("ref/resource");
                if equal(a1996, a1997) {
                    let m1998 = f481(Str::lit("uri"), v0);
                    match m1998 {
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

pub fn f486(v0: Adt5) -> Result<Adt27, Adt120> {
    let m1999 = f481(Str::lit("name"), v0.clone());
    match m1999 {
        Ok(v1) => {
            let m2000 = f481(Str::lit("value"), v0);
            match m2000 {
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

pub fn f487(v0: Adt5) -> Result<Option<Adt28>, Adt120> {
    let m2001 = f111(Str::lit("context"), v0);
    match m2001 {
        Adt123::C0 => {
            Ok::<Option<Adt28>, Adt120>(None::<Adt28>)
        }
        Adt123::C1(v1) => {
            let m2002 = f239(v1);
            match m2002 {
                None => {
                    Err::<Option<Adt28>, Adt120>(Adt120::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2003 = f483(Str::lit("arguments"), v2);
                    match m2003 {
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

pub fn f488(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt119> {
    if v0 {
        None::<Adt119>
    } else {
        Some(Adt119::C1(v1, v2))
    }
}

pub fn f489(v0: Str, p2008: List<Str>) -> bool {
    let m2007 = p2008;
    {
        let mut m2007 = m2007;
        loop {
            let step = {
                let v1 = m2007;
                let m2004 = v1;
                match m2004.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2005 = v2;
                        let a2006 = v0.clone();
                        if equal(a2005, a2006) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2007 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: Option<Adt97>, v1: List<Adt99>, v2: Adt5) -> R<Option<Adt97>> {
    let m2009 = v0;
    match m2009 {
        None => {
            f534(v1, f535(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f491(v0: Option<Str>, v1: Adt138, v2: Adt5) -> Option<Adt97> {
    let m2010 = v1.clone();
    match m2010 {
        Adt138::C0 => {
            None::<Adt97>
        }
        Adt138::C1 => {
            f536(v0, f537(v1, v2))
        }
        Adt138::C2 => {
            f536(v0, f537(v1, v2))
        }
        Adt138::C3 => {
            f536(v0, f537(v1, v2))
        }
    }
}

pub fn f492(v0: Adt123, v1: Adt123) -> Adt145 {
    let m2011 = v0;
    match m2011 {
        Adt123::C0 => {
            Adt145::C1
        }
        Adt123::C1(_) => {
            let m2012 = v1;
            match m2012 {
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

pub fn f493(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt120> {
    let m2013 = f151(v1);
    match m2013 {
        None => {
            Err::<Adt16, Adt120>(Adt120::C5)
        }
        Some(v3) => {
            let m2014 = f538(v2);
            match m2014 {
                None => {
                    Err::<Adt16, Adt120>(Adt120::C6)
                }
                Some(v4) => {
                    let m2015 = f482(f539(), v0.clone());
                    match m2015 {
                        Ok(v6) => {
                            let m2016 = f540(v0.clone());
                            match m2016 {
                                Ok(v8) => {
                                    let m2017 = f541(v0.clone());
                                    match m2017 {
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

pub fn f494() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f495(v0: u64, v1: u64) -> u64 {
    let a2018 = v0;
    let a2019 = v1;
    let a2020 = 0u64;
    nat_rem(a2018, a2019, a2020)
}

pub fn f496(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2023 = {
            let a2021 = v0;
            let a2022 = 4u64;
            nat_mul(a2021, a2022)?
        };
        let a2024 = f498(v1, 16u64);
        nat_add(a2023, a2024)?
    }, List::<u64>::nil()))
}

pub fn f497(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2027 = {
            let a2025 = v0;
            let a2026 = 4u64;
            nat_mul(a2025, a2026)?
        };
        let a2028 = f498(v1, 16u64);
        nat_add(a2027, a2028)?
    }, List::cons({
        let a2031 = {
            let a2029 = f495(v1, 16u64);
            let a2030 = 16u64;
            nat_mul(a2029, a2030)?
        };
        let a2032 = f498(v2, 4u64);
        nat_add(a2031, a2032)?
    }, List::<u64>::nil())))
}

pub fn f498(v0: u64, v1: u64) -> u64 {
    let a2033 = v0;
    let a2034 = v1;
    let a2035 = 0u64;
    nat_quot(a2033, a2034, a2035)
}

pub fn f499(v0: u64, v1: Adt6) -> bool {
    let a2036 = v0;
    let a2037 = 0u64;
    if nat_eq(a2036, a2037) {
        f542(v1)
    } else {
        let a2038 = v0;
        let a2039 = 1u64;
        if nat_eq(a2038, a2039) {
            f543(v1)
        } else {
            let a2040 = v0;
            let a2041 = 2u64;
            if nat_eq(a2040, a2041) {
                f544(v1)
            } else {
                let a2042 = v0;
                let a2043 = 3u64;
                if nat_eq(a2042, a2043) {
                    f545(v1)
                } else {
                    let a2044 = v0;
                    let a2045 = 4u64;
                    if nat_eq(a2044, a2045) {
                        f546(v1)
                    } else {
                        let a2046 = v0;
                        let a2047 = 5u64;
                        if nat_eq(a2046, a2047) {
                            f547(v1)
                        } else {
                            let a2048 = v0;
                            let a2049 = 6u64;
                            if nat_eq(a2048, a2049) {
                                f548(v1)
                            } else {
                                let a2050 = v0;
                                let a2051 = 7u64;
                                if nat_eq(a2050, a2051) {
                                    f549(v1)
                                } else {
                                    f550(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f500(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt132) -> R<Adt144> {
    let m2052 = v3.clone();
    match m2052.uncons() {
        None => {
            Ok(Adt144::C0(Adt94::C8(v0)))
        }
        Some((_, _)) => {
            if f232(v3.clone(), true) {
                let m2053 = f551(v1);
                match m2053 {
                    None => {
                        Ok(Adt144::C0(Adt94::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f552(v3)?;
                        if f553(v9.clone(), match v5.clone() {
                            Adt132::C0(_, h2054) => {
                                h2054
                            }
                        }) {
                            Ok(Adt144::C0(Adt94::C12(v0, v2)))
                        } else {
                            Ok(Adt144::C1(Adt132::C0(List::cons(Adt12::C0(v0, v2, f554(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt132::C0(h2055, _) => {
                                    h2055
                                }
                            }), List::cons(v9, match v5 {
                                Adt132::C0(_, h2056) => {
                                    h2056
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

pub fn f501(p2061: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2062: List<Adt125>) -> List<Adt125> {
    let m2060 = (p2061, p2062);
    {
        let mut m2060 = m2060;
        loop {
            let step = {
                let (v0, v3) = m2060;
                let m2057 = v0;
                match m2057.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v5, List::cons(Adt125::C0({
                            let (_, h2058) = v4;
                            h2058
                        }, v1, List::cons({
                            let (h2059, _) = v4;
                            h2059
                        }, v2.clone())), v3)))
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

pub fn f502(p2066: List<(Str, Adt6)>, p2067: List<Adt125>) -> List<Adt125> {
    let m2065 = (p2066, p2067);
    {
        let mut m2065 = m2065;
        loop {
            let step = {
                let (v0, v1) = m2065;
                let m2063 = v0;
                match m2063.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v3, List::cons(Adt125::C0({
                            let (_, h2064) = v2;
                            h2064
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(p2070: List<Adt6>, p2071: List<Adt125>) -> List<Adt125> {
    let m2069 = (p2070, p2071);
    {
        let mut m2069 = m2069;
        loop {
            let step = {
                let (v0, v1) = m2069;
                let m2068 = v0;
                match m2068.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt125>), List<Adt125>>((v3, List::cons(Adt125::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: Adt6) -> Str {
    let m2072 = v0;
    match m2072 {
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

pub fn f505(v0: Adt7) -> R<bool> {
    if f299(match v0.clone() {
        Adt7::C0(h2073, _) => {
            h2073
        }
    }) {
        Ok(true)
    } else {
        f555(40u64, match v0.clone() {
            Adt7::C0(h2074, _) => {
                h2074
            }
        }, match v0 {
            Adt7::C0(_, h2075) => {
                h2075
            }
        })
    }
}

pub fn f506(v0: Adt6) -> Str {
    let m2076 = f151(v0);
    match m2076 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f507(p2083: u64, p2084: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m2082 = (p2083, p2084);
    {
        let mut m2082 = m2082;
        loop {
            let step = {
                let (v0, v1) = m2082;
                let m2077 = v0;
                if m2077 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m2077 - 1;
                    let m2078 = v1;
                    match m2078.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m2081 = f556({
                                let (h2079, _) = v3;
                                h2079
                            }, {
                                let (_, h2080) = v3;
                                h2080
                            }, v4)?;
                            match m2081 {
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
                Ok(next) => { m2082 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508() -> u64 {
    67108864u64
}

pub fn f509(v0: Adt147) -> bool {
    let m2085 = v0;
    match m2085 {
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

pub fn f510(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m2086 = v2;
    match m2086 {
        Adt147::C0 => {
            let m2088 = v3;
            match m2088 {
                Adt147::C0 => {
                    Ok(f557(Adt147::C0, f558(f559(match v0.clone() {
                        Adt7::C0(h2089, _) => {
                            h2089
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2090) => {
                            h2090
                        }
                    }, f559(match v1.clone() {
                        Adt7::C0(h2091, _) => {
                            h2091
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2092) => {
                            h2092
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
            let m2087 = v3;
            match m2087 {
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
            let m2093 = v3;
            match m2093 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f558(match v0.clone() {
                        Adt7::C0(h2094, _) => {
                            h2094
                        }
                    }, match v0 {
                        Adt7::C0(_, h2095) => {
                            h2095
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2096, _) => {
                            h2096
                        }
                    }, match v1 {
                        Adt7::C0(_, h2097) => {
                            h2097
                        }
                    })
                }
            }
        }
    }
}

pub fn f511(v0: i64) -> Adt147 {
    f513(v0, 0i64)
}

pub fn f512(v0: Adt6) -> Option<i64> {
    let m2098 = v0;
    match m2098 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f299(match v2.clone() {
                Adt7::C0(_, h2099) => {
                    h2099
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2100, _) => {
                        h2100
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

pub fn f513(v0: i64, v1: i64) -> Adt147 {
    let m2103 = {
        let a2101 = v0;
        let a2102 = v1;
        compare(a2101, a2102)
    };
    if match m2103 {
        Ordering::Less => {
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
        let m2106 = {
            let a2104 = v1;
            let a2105 = v0;
            compare(a2104, a2105)
        };
        if match m2106 {
            Ordering::Less => {
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

pub fn f514(p2115: List<u64>, p2116: u64) -> R<u64> {
    let m2114 = (p2115, p2116);
    {
        let mut m2114 = m2114;
        loop {
            let step = {
                let (v0, v1) = m2114;
                let m2107 = v0;
                match m2107.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2108 = v2;
                        let a2109 = 128u64;
                        if if nat_lt(a2108, a2109) {
                            true
                        } else {
                            let a2110 = 191u64;
                            let a2111 = v2;
                            nat_lt(a2110, a2111)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2112 = v1;
                                let a2113 = 1u64;
                                nat_add(a2112, a2113)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: Str, p2134: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2133 = p2134;
    {
        let mut m2133 = m2133;
        loop {
            let step = {
                let v1 = m2133;
                let m2117 = v1;
                match m2117.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2121 = v0.clone();
                        let a2122 = {
                            let a2119 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2118, _) = v2.clone();
                                h2118
                            }, List::<Str>::nil()));
                            let a2120 = Str::lit("");
                            join(a2119, a2120)
                        };
                        if equal(a2121, a2122) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2123) = v2;
                                h2123
                            }))
                        } else {
                            let m2125 = f239({
                                let (_, h2124) = v2.clone();
                                h2124
                            });
                            match m2125 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2126 = f111(Str::lit("$anchor"), v4);
                                    match m2126 {
                                        Adt123::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt123::C1(v5) => {
                                            let m2127 = f151(v5);
                                            match m2127 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2130 = v0.clone();
                                                    let a2131 = {
                                                        let a2128 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2129 = Str::lit("");
                                                        join(a2128, a2129)
                                                    };
                                                    if equal(a2130, a2131) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2132) = v2;
                                                            h2132
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
                Ok(next) => { m2133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: i64) -> u64 {
    let m2137 = {
        let a2135 = v0;
        let a2136 = 1i64;
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
        48u64
    } else {
        let m2140 = {
            let a2138 = v0;
            let a2139 = 2i64;
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
            49u64
        } else {
            let m2143 = {
                let a2141 = v0;
                let a2142 = 3i64;
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
                50u64
            } else {
                let m2146 = {
                    let a2144 = v0;
                    let a2145 = 4i64;
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
                    51u64
                } else {
                    let m2149 = {
                        let a2147 = v0;
                        let a2148 = 5i64;
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
                        52u64
                    } else {
                        let m2152 = {
                            let a2150 = v0;
                            let a2151 = 6i64;
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
                            53u64
                        } else {
                            let m2155 = {
                                let a2153 = v0;
                                let a2154 = 7i64;
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
                                54u64
                            } else {
                                let m2158 = {
                                    let a2156 = v0;
                                    let a2157 = 8i64;
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
                                    55u64
                                } else {
                                    let m2161 = {
                                        let a2159 = v0;
                                        let a2160 = 9i64;
                                        compare(a2159, a2160)
                                    };
                                    if match m2161 {
                                        Ordering::Less => {
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

pub fn f517(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2162 = v0;
    match m2162 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2174 = f321(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2163, _, _, _, _) => {
                        h2163
                    }
                });
                let a2175 = {
                    let a2172 = f326(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2164, _, _, _) => {
                            h2164
                        }
                    });
                    let a2173 = {
                        let a2170 = f326(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2165, _, _) => {
                                h2165
                            }
                        });
                        let a2171 = {
                            let a2168 = f326(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2166, _) => {
                                    h2166
                                }
                            });
                            let a2169 = f326(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2167) => {
                                    h2167
                                }
                            });
                            append_list(a2168, a2169)
                        };
                        append_list(a2170, a2171)
                    };
                    append_list(a2172, a2173)
                };
                append_list(a2174, a2175)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f518(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f344(Str::lit("_meta"), v0)
}

pub fn f519(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2176, _, _) => {
            h2176
        }
    })), {
        let a2179 = f523(match v0.clone() {
            Adt39::C0(_, h2177, _) => {
                h2177
            }
        });
        let a2180 = f518(match v0 {
            Adt39::C0(_, _, h2178) => {
                h2178
            }
        });
        append_list(a2179, a2180)
    })))
}

pub fn f520(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2181 = f523(v3);
        let a2182 = f518(v4);
        append_list(a2181, a2182)
    }))))
}

pub fn f521(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2206 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2183, _, _, _, _, _, _, _, _) => {
                h2183
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2207 = {
            let a2204 = f321(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2184, _, _, _, _, _, _, _) => {
                    h2184
                }
            });
            let a2205 = {
                let a2202 = f401(match v0.clone() {
                    Adt44::C0(_, _, h2185, _, _, _, _, _, _) => {
                        h2185
                    }
                });
                let a2203 = {
                    let a2200 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2186, _, _, _, _, _) => {
                            h2186
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2201 = {
                        let a2198 = f321(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2187, _, _, _, _) => {
                                h2187
                            }
                        });
                        let a2199 = {
                            let a2196 = f321(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2188, _, _, _) => {
                                    h2188
                                }
                            });
                            let a2197 = {
                                let a2194 = f523(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2189, _, _) => {
                                        h2189
                                    }
                                });
                                let a2195 = {
                                    let a2192 = f524(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2190, _) => {
                                            h2190
                                        }
                                    })?;
                                    let a2193 = f518(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2191) => {
                                            h2191
                                        }
                                    });
                                    append_list(a2192, a2193)
                                };
                                append_list(a2194, a2195)
                            };
                            append_list(a2196, a2197)
                        };
                        append_list(a2198, a2199)
                    };
                    append_list(a2200, a2201)
                };
                append_list(a2202, a2203)
            };
            append_list(a2204, a2205)
        };
        append_list(a2206, a2207)
    }))
}

pub fn f522(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f468(match v0.clone() {
        Adt45::C0(h2208, _, _) => {
            h2208
        }
    })), {
        let a2211 = f523(match v0.clone() {
            Adt45::C0(_, h2209, _) => {
                h2209
            }
        });
        let a2212 = f518(match v0 {
            Adt45::C0(_, _, h2210) => {
                h2210
            }
        });
        append_list(a2211, a2212)
    })))
}

pub fn f523(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2213 = v0;
    match m2213 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2219 = f560(match v1.clone() {
                    Adt40::C0(h2214, _, _) => {
                        h2214
                    }
                });
                let a2220 = {
                    let a2217 = f561(match v1.clone() {
                        Adt40::C0(_, h2215, _) => {
                            h2215
                        }
                    });
                    let a2218 = f321(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2216) => {
                            h2216
                        }
                    });
                    append_list(a2217, a2218)
                };
                append_list(a2219, a2220)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f524(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2221 = v0;
    match m2221 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f392(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f525(v0: List<Adt57>) -> List<Adt6> {
    f214(f562(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f526(v0: Adt41) -> Str {
    let m2222 = v0;
    match m2222 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f527(p2225: List<Adt2>, p2226: List<Adt6>) -> List<Adt6> {
    let m2224 = (p2225, p2226);
    {
        let mut m2224 = m2224;
        loop {
            let step = {
                let (v0, v1) = m2224;
                let m2223 = v0;
                match m2223.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f563(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(p2231: List<u64>, p2232: u64, p2233: List<u64>) -> List<u64> {
    let m2230 = (p2231, (p2232, p2233));
    {
        let mut m2230 = m2230;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2230;
                let m2227 = v0;
                match m2227.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f564(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt148 = f565(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt148::C0(h2228, _) => {
                                h2228
                            }
                        }, match v5 {
                            Adt148::C0(_, h2229) => {
                                h2229
                            }
                        })))
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

pub fn f529(v0: Adt128, v1: Adt32) -> bool {
    let m2234 = v1;
    match m2234 {
        Adt32::C0(_) => {
            f566(v0, Adt128::C0)
        }
        Adt32::C1(_) => {
            f566(v0, Adt128::C1)
        }
        Adt32::C2(_) => {
            f566(v0, Adt128::C2)
        }
        Adt32::C3(_) => {
            f566(v0, Adt128::C2)
        }
        Adt32::C4(_) => {
            f566(v0, Adt128::C3)
        }
        Adt32::C5(_) => {
            f566(v0, Adt128::C4)
        }
        Adt32::C6(_) => {
            f566(v0, Adt128::C5)
        }
        Adt32::C7(_) => {
            f566(v0, Adt128::C6)
        }
        Adt32::C8(_) => {
            f566(v0, Adt128::C7)
        }
        Adt32::C9(_) => {
            f566(v0, Adt128::C8)
        }
        Adt32::C10(_) => {
            f566(v0, Adt128::C9)
        }
        Adt32::C11 => {
            if f566(v0.clone(), Adt128::C10) {
                true
            } else {
                f566(v0, Adt128::C11)
            }
        }
        Adt32::C12(_) => {
            f567(v0)
        }
        Adt32::C13(_) => {
            f568(v0)
        }
    }
}

pub fn f530(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt116, Adt119> {
    let m2235 = v2.clone();
    match m2235 {
        Adt32::C0(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt116, Adt119>(f572(v0, f570(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt116, Adt119>(f476(v0, v2))
        }
        Adt32::C12(v3) => {
            f569(v0, f570(v1), v3)
        }
        Adt32::C13(v4) => {
            f571(v0, f570(v1), v4)
        }
    }
}

pub fn f531() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f532(v0: u64, v1: u64) -> bool {
    let a2236 = v1;
    let a2237 = v0;
    nat_lt(a2236, a2237)
}

pub fn f533(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt120> {
    let m2238 = f573(v0, v1, List::<(Str, Str)>::nil());
    match m2238 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt120>(f574(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt120>(v3)
        }
    }
}

pub fn f534(p2242: List<Adt99>, v1: Adt5) -> R<Option<Adt97>> {
    let m2241 = p2242;
    {
        let mut m2241 = m2241;
        loop {
            let step = {
                let v0 = m2241;
                let m2239 = v0;
                match m2239.uncons() {
                    None => {
                        Err::<List<Adt99>, R<Option<Adt97>>>(Ok(None::<Adt97>))
                    }
                    Some((v2, v3)) => {
                        let m2240 = f575(v2, v1.clone())?;
                        match m2240 {
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
                Ok(next) => { m2241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f535(v0: Adt5) -> Adt5 {
    let m2243 = f111(Str::lit("arguments"), v0);
    match m2243 {
        Adt123::C0 => {
            f419()
        }
        Adt123::C1(v1) => {
            let m2244 = f239(v1);
            match m2244 {
                None => {
                    f419()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f536(v0: Option<Str>, v1: Option<Str>) -> Option<Adt97> {
    let m2245 = v0;
    match m2245 {
        None => {
            Some(Adt97::C2)
        }
        Some(v2) => {
            let m2246 = v1;
            match m2246 {
                None => {
                    None::<Adt97>
                }
                Some(v3) => {
                    let a2247 = v2;
                    let a2248 = v3;
                    if equal(a2247, a2248) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C5)
                    }
                }
            }
        }
    }
}

pub fn f537(v0: Adt138, v1: Adt5) -> Option<Str> {
    let m2249 = v0;
    match m2249 {
        Adt138::C0 => {
            None::<Str>
        }
        Adt138::C1 => {
            let m2250 = f111(Str::lit("name"), v1);
            match m2250 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v2) => {
                    f151(v2)
                }
            }
        }
        Adt138::C2 => {
            let m2251 = f111(Str::lit("uri"), v1);
            match m2251 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v3) => {
                    f151(v3)
                }
            }
        }
        Adt138::C3 => {
            let m2252 = f111(Str::lit("taskId"), v1);
            match m2252 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v4) => {
                    f151(v4)
                }
            }
        }
    }
}

pub fn f538(v0: Adt6) -> Option<Adt17> {
    let m2253 = f239(v0);
    match m2253 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2254 = f576(Str::lit("experimental"), v1.clone());
            match m2254 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2255 = f576(Str::lit("roots"), v1.clone());
                    match m2255 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2256 = f576(Str::lit("sampling"), v1.clone());
                            match m2256 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2257 = f576(Str::lit("elicitation"), v1.clone());
                                    match m2257 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2258 = f576(Str::lit("extensions"), v1);
                                            match m2258 {
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

pub fn f539() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f540(v0: Adt5) -> Result<Option<Adt18>, Adt120> {
    let m2259 = f480(f577(), v0);
    match m2259 {
        Ok(v2) => {
            let m2260 = v2;
            match m2260 {
                None => {
                    Ok::<Option<Adt18>, Adt120>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2261 = f578(v3);
                    match m2261 {
                        Adt150::C0 => {
                            Err::<Option<Adt18>, Adt120>(Adt120::C8(f577()))
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

pub fn f541(v0: Adt5) -> Result<Option<Adt19>, Adt120> {
    let m2262 = f111(f579(), v0);
    match m2262 {
        Adt123::C0 => {
            Ok::<Option<Adt19>, Adt120>(None::<Adt19>)
        }
        Adt123::C1(v1) => {
            let m2263 = f151(v1.clone());
            match m2263 {
                None => {
                    let m2264 = f512(v1);
                    match m2264 {
                        None => {
                            Err::<Option<Adt19>, Adt120>(Adt120::C8(f579()))
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

pub fn f542(v0: Adt6) -> bool {
    let m2265 = f151(v0);
    match m2265 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f543(v0: Adt6) -> bool {
    let m2266 = f445(v0);
    match m2266 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f544(v0: Adt6) -> bool {
    let m2267 = f512(v0);
    match m2267 {
        None => {
            false
        }
        Some(v1) => {
            let a2271 = {
                let m2270 = {
                    let a2268 = v1;
                    let a2269 = 0i64;
                    compare(a2268, a2269)
                };
                match m2270 {
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
            bool_not(a2271)
        }
    }
}

pub fn f545(v0: Adt6) -> bool {
    let m2272 = f310(v0);
    match m2272 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f546(v0: Adt6) -> bool {
    let m2273 = f239(v0);
    match m2273 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f547(v0: Adt6) -> bool {
    let m2274 = f310(v0);
    match m2274 {
        None => {
            false
        }
        Some(v1) => {
            f580(v1)
        }
    }
}

pub fn f548(v0: Adt6) -> bool {
    let m2275 = f238(v0);
    match m2275 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f549(v0: Adt6) -> bool {
    let m2276 = f151(v0.clone());
    match m2276 {
        None => {
            let m2277 = f310(v0);
            match m2277 {
                None => {
                    false
                }
                Some(v2) => {
                    f581(v2)
                }
            }
        }
        Some(v1) => {
            f440(v1)
        }
    }
}

pub fn f550(v0: Adt6) -> bool {
    let m2278 = f310(v0);
    match m2278 {
        None => {
            false
        }
        Some(v1) => {
            let m2279 = v1;
            match m2279.uncons() {
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

pub fn f551(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2280 = f154(Str::lit("type"), v0);
    match m2280 {
        Adt123::C0 => {
            None::<Adt13>
        }
        Adt123::C1(v1) => {
            let m2281 = f151(v1);
            match m2281 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f582(v2)
                }
            }
        }
    }
}

pub fn f552(v0: List<u64>) -> R<List<u64>> {
    Ok(f129(f583(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f553(v0: List<u64>, p2284: List<List<u64>>) -> bool {
    let m2283 = p2284;
    {
        let mut m2283 = m2283;
        loop {
            let step = {
                let v1 = m2283;
                let m2282 = v1;
                match m2282.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f584(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2283 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(p2287: List<Str>, p2288: List<Str>) -> List<Str> {
    let m2286 = (p2287, p2288);
    {
        let mut m2286 = m2286;
        loop {
            let step = {
                let (v0, v1) = m2286;
                let m2285 = v0;
                match m2285.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(p2300: u64, p2301: i64, p2302: i64) -> R<bool> {
    let m2299 = (p2300, (p2301, p2302));
    {
        let mut m2299 = m2299;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2299;
                let m2289 = v0;
                if m2289 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2289 - 1;
                    let m2292 = {
                        let a2290 = v2;
                        let a2291 = 0i64;
                        compare(a2290, a2291)
                    };
                    if match m2292 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f299({
                            let a2293 = v1;
                            let a2294 = 10i64;
                            let a2295 = 0i64;
                            int_rem(a2293, a2294, a2295)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2296 = v1;
                                let a2297 = 10i64;
                                let a2298 = 0i64;
                                int_quot(a2296, a2297, a2298)?
                            }, f123(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2303 = f585(v0.clone());
    let a2304 = f585(v1.clone());
    if nat_eq(a2303, a2304) {
        let m2305 = v0;
        match m2305 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2306 = f238(v1);
                match m2306 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2307 = v3;
                        let a2308 = v4;
                        if equal(a2307, a2308) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2309 = f445(v1);
                match m2309 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f586(f447(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2310 = f151(v1);
                match m2310 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2311 = v7;
                        let a2312 = v8;
                        if equal(a2311, a2312) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2313 = f310(v1);
                match m2313 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f587(f588(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2314 = f239(v1);
                match m2314 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2316 = f451(v11.clone(), 0u64)?;
                        let a2317 = f451(match v12.clone() {
                            Adt5::C0(h2315) => {
                                h2315
                            }
                        }, 0u64)?;
                        if nat_eq(a2316, a2317) {
                            Ok(f587(f589(v11, match v12 {
                                Adt5::C0(h2318) => {
                                    h2318
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

pub fn f557(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2319 = v0;
    match m2319 {
        Adt147::C0 => {
            f590(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f558(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2320 = f513(f591(v0, v1)?, f591(v2, v3)?);
    match m2320 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f592(v0, f593(v0)?, v2, f593(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f559(v0: i64) -> R<i64> {
    let m2323 = {
        let a2321 = v0;
        let a2322 = 0i64;
        compare(a2321, a2322)
    };
    if match m2323 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2324 = v0;
        int_neg(a2324)
    } else {
        Ok(v0)
    }
}

pub fn f560(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2325 = v0;
    match m2325 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f594(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f561(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2326 = v0;
    match m2326 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f562(p2329: List<Adt57>, p2330: List<Adt6>) -> List<Adt6> {
    let m2328 = (p2329, p2330);
    {
        let mut m2328 = m2328;
        loop {
            let step = {
                let (v0, v1) = m2328;
                let m2327 = v0;
                match m2327.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f595(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2331, _, _, _) => {
            h2331
        }
    })), {
        let a2337 = f321(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2332, _, _) => {
                h2332
            }
        });
        let a2338 = {
            let a2335 = f596(match v0.clone() {
                Adt2::C0(_, _, h2333, _) => {
                    h2333
                }
            });
            let a2336 = f597(match v0 {
                Adt2::C0(_, _, _, h2334) => {
                    h2334
                }
            });
            append_list(a2335, a2336)
        };
        append_list(a2337, a2338)
    }))
}

pub fn f564(v0: u64, v1: List<u64>) -> List<u64> {
    let a2339 = v0;
    let a2340 = 0u64;
    if nat_eq(a2339, a2340) {
        v1
    } else {
        let a2341 = v0;
        let a2342 = 1u64;
        if nat_eq(a2341, a2342) {
            List::cons(194u64, v1)
        } else {
            let a2343 = v0;
            let a2344 = 2u64;
            if nat_eq(a2343, a2344) {
                List::cons(226u64, v1)
            } else {
                let a2345 = v0;
                let a2346 = 3u64;
                if nat_eq(a2345, a2346) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f565(v0: u64, v1: u64, v2: List<u64>) -> Adt148 {
    let a2347 = v0;
    let a2348 = 0u64;
    if nat_eq(a2347, a2348) {
        f598(v1, v2)
    } else {
        let a2349 = v0;
        let a2350 = 1u64;
        if nat_eq(a2349, a2350) {
            let a2351 = 128u64;
            let a2352 = v1;
            if if nat_le(a2351, a2352) {
                let a2353 = v1;
                let a2354 = 159u64;
                nat_le(a2353, a2354)
            } else {
                false
            } {
                Adt148::C0(0u64, f599(v2))
            } else {
                f598(v1, List::cons(194u64, v2))
            }
        } else {
            let a2355 = v0;
            let a2356 = 2u64;
            if nat_eq(a2355, a2356) {
                let a2357 = v1;
                let a2358 = 128u64;
                if nat_eq(a2357, a2358) {
                    Adt148::C0(3u64, v2)
                } else {
                    let a2359 = v1;
                    let a2360 = 129u64;
                    if nat_eq(a2359, a2360) {
                        Adt148::C0(4u64, v2)
                    } else {
                        f598(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2361 = v0;
                let a2362 = 3u64;
                if nat_eq(a2361, a2362) {
                    let a2363 = 170u64;
                    let a2364 = v1;
                    if if nat_le(a2363, a2364) {
                        let a2365 = v1;
                        let a2366 = 174u64;
                        nat_le(a2365, a2366)
                    } else {
                        false
                    } {
                        Adt148::C0(0u64, f599(v2))
                    } else {
                        f598(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2367 = 166u64;
                    let a2368 = v1;
                    if if nat_le(a2367, a2368) {
                        let a2369 = v1;
                        let a2370 = 169u64;
                        nat_le(a2369, a2370)
                    } else {
                        false
                    } {
                        Adt148::C0(0u64, f599(v2))
                    } else {
                        f598(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f566(v0: Adt128, v1: Adt128) -> bool {
    let a2371 = f347(v0);
    let a2372 = f347(v1);
    equal(a2371, a2372)
}

pub fn f567(v0: Adt128) -> bool {
    let m2373 = v0;
    match m2373 {
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

pub fn f568(v0: Adt128) -> bool {
    let m2374 = v0;
    match m2374 {
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

pub fn f569(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt116, Adt119> {
    f600(v0, v2.clone(), f601(match v1 {
        Adt16::C0(_, h2375, _, _, _, _) => {
            h2375
        }
    }, f602(v2)))
}

pub fn f570(v0: Adt14) -> Adt16 {
    let m2376 = v0;
    match m2376 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2377) => {
                    h2377
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2378, _) => {
                    h2378
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2379, _, _, _, _) => {
                    h2379
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2380, _) => {
                    h2380
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2381, _, _, _) => {
                    h2381
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2382, _) => {
                    h2382
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2383, _) => {
                    h2383
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2384, _, _, _, _) => {
                    h2384
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2385, _, _, _) => {
                    h2385
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2386, _) => {
                    h2386
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2387, _, _) => {
                    h2387
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2388, _) => {
                    h2388
                }
            }
        }
    }
}

pub fn f571(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt116, Adt119> {
    f603(v0, v2, f604(match v1 {
        Adt16::C0(_, h2389, _, _, _, _) => {
            h2389
        }
    }))
}

pub fn f572(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt116 {
    Adt116::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2390, _) => {
            h2390
        }
    }), {
        let m2392 = f476(v0, Adt32::C11);
        match m2392 {
            Adt116::C0(_, h2391, _) => {
                h2391
            }
        }
    }, f605(v1, match v2 {
        Adt49::C0(_, h2393) => {
            h2393
        }
    }))
}

pub fn f573(v0: Str, p2399: List<(Str, Adt6)>, p2400: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt120> {
    let m2398 = (p2399, p2400);
    {
        let mut m2398 = m2398;
        loop {
            let step = {
                let (v1, v2) = m2398;
                let m2394 = v1;
                match m2394.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>(Ok::<List<(Str, Str)>, Adt120>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2396 = f151({
                            let (_, h2395) = v3;
                            h2395
                        });
                        match m2396 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>(Err::<List<(Str, Str)>, Adt120>(Adt120::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>((v4, List::cons(({
                                    let (h2397, _) = v3;
                                    h2397
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f574(p2403: List<(Str, Str)>, p2404: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2402 = (p2403, p2404);
    {
        let mut m2402 = m2402;
        loop {
            let step = {
                let (v0, v1) = m2402;
                let m2401 = v0;
                match m2401.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f575(v0: Adt99, v1: Adt5) -> R<Option<Adt97>> {
    let m2406 = f606(match v0.clone() {
        Adt99::C0(_, h2405, _, _) => {
            h2405
        }
    }, v1);
    match m2406 {
        Adt123::C0 => {
            Ok(f607(v0))
        }
        Adt123::C1(v2) => {
            if f608(v2.clone()) {
                Ok(f607(v0))
            } else {
                f609(v0, v2)
            }
        }
    }
}

pub fn f576(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2407 = f111(v0, v1);
    match m2407 {
        Adt123::C0 => {
            Some(None::<Adt5>)
        }
        Adt123::C1(v2) => {
            let m2408 = f239(v2);
            match m2408 {
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

pub fn f577() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f578(v0: Str) -> Adt150 {
    let a2409 = v0.clone();
    let a2410 = Str::lit("debug");
    if equal(a2409, a2410) {
        Adt150::C1(Adt18::C0)
    } else {
        let a2411 = v0.clone();
        let a2412 = Str::lit("info");
        if equal(a2411, a2412) {
            Adt150::C1(Adt18::C1)
        } else {
            let a2413 = v0.clone();
            let a2414 = Str::lit("notice");
            if equal(a2413, a2414) {
                Adt150::C1(Adt18::C2)
            } else {
                let a2415 = v0.clone();
                let a2416 = Str::lit("warning");
                if equal(a2415, a2416) {
                    Adt150::C1(Adt18::C3)
                } else {
                    let a2417 = v0.clone();
                    let a2418 = Str::lit("error");
                    if equal(a2417, a2418) {
                        Adt150::C1(Adt18::C4)
                    } else {
                        let a2419 = v0.clone();
                        let a2420 = Str::lit("critical");
                        if equal(a2419, a2420) {
                            Adt150::C1(Adt18::C5)
                        } else {
                            let a2421 = v0.clone();
                            let a2422 = Str::lit("alert");
                            if equal(a2421, a2422) {
                                Adt150::C1(Adt18::C6)
                            } else {
                                let a2423 = v0;
                                let a2424 = Str::lit("emergency");
                                if equal(a2423, a2424) {
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

pub fn f579() -> Str {
    Str::lit("progressToken")
}

pub fn f580(p2428: List<Adt6>) -> bool {
    let m2427 = p2428;
    {
        let mut m2427 = m2427;
        loop {
            let step = {
                let v0 = m2427;
                let m2425 = v0;
                match m2425.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2426 = f151(v1);
                        match m2426 {
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
                Ok(next) => { m2427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(p2432: List<Adt6>) -> bool {
    let m2431 = p2432;
    {
        let mut m2431 = m2431;
        loop {
            let step = {
                let v0 = m2431;
                let m2429 = v0;
                match m2429.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2430 = f151(v1);
                        match m2430 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f440(v3) {
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
                Ok(next) => { m2431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(v0: Str) -> Option<Adt13> {
    let a2433 = v0.clone();
    let a2434 = Str::lit("string");
    if equal(a2433, a2434) {
        Some(Adt13::C0)
    } else {
        let a2435 = v0.clone();
        let a2436 = Str::lit("integer");
        if equal(a2435, a2436) {
            Some(Adt13::C1)
        } else {
            let a2437 = v0;
            let a2438 = Str::lit("boolean");
            if equal(a2437, a2438) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f583(p2447: List<u64>, p2448: List<u64>) -> R<List<u64>> {
    let m2446 = (p2447, p2448);
    {
        let mut m2446 = m2446;
        loop {
            let step = {
                let (v0, v1) = m2446;
                let m2439 = v0;
                match m2439.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2440 = 65u64;
                        let a2441 = v2;
                        if if nat_le(a2440, a2441) {
                            let a2442 = v2;
                            let a2443 = 90u64;
                            nat_le(a2442, a2443)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2444 = v2;
                                let a2445 = 32u64;
                                nat_add(a2444, a2445)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
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

pub fn f584(p2455: List<u64>, p2456: List<u64>) -> bool {
    let m2454 = (p2455, p2456);
    {
        let mut m2454 = m2454;
        loop {
            let step = {
                let (v0, v1) = m2454;
                let m2449 = v0;
                match m2449.uncons() {
                    None => {
                        let m2450 = v1;
                        match m2450.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2451 = v1;
                        match m2451.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2452 = v4;
                                let a2453 = v6;
                                if nat_eq(a2452, a2453) {
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
                Ok(next) => { m2454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f585(v0: Adt6) -> u64 {
    let m2457 = v0;
    match m2457 {
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

pub fn f586(v0: Adt147) -> bool {
    let m2458 = v0;
    match m2458 {
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

pub fn f587(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2459 = v0;
    match m2459 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2460 = v2;
                let a2461 = v1;
                append_list(a2460, a2461)
            })
        }
    }
}

pub fn f588(p2466: List<Adt6>, p2467: List<Adt6>, p2468: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2465 = (p2466, (p2467, p2468));
    {
        let mut m2465 = m2465;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2465;
                let m2462 = v0;
                match m2462.uncons() {
                    None => {
                        let m2463 = v1;
                        match m2463.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2464 = v1;
                        match m2464.uncons() {
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
                Ok(next) => { m2465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(p2474: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2475: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2473 = (p2474, p2475);
    {
        let mut m2473 = m2473;
        loop {
            let step = {
                let (v0, v2) = m2473;
                let m2469 = v0;
                match m2469.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2471 = f154({
                            let (h2470, _) = v3;
                            h2470
                        }, v1.clone());
                        match m2471 {
                            Adt123::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt123::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2472) = v3;
                                    h2472
                                }, v5), v2)))
                            }
                        }
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

pub fn f590(v0: Adt147) -> Adt147 {
    let m2476 = v0;
    match m2476 {
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

pub fn f591(v0: i64, v1: i64) -> R<i64> {
    f123(f89(f593(v0)?, 0i64)?, v1)
}

pub fn f592(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2477 = v1;
    let a2478 = v3;
    if nat_lt(a2477, a2478) {
        Ok(f513(f610({
            let a2479 = v3;
            let a2480 = v1;
            nat_sub(a2479, a2480)
        }, v0)?, v2))
    } else {
        let a2481 = v3;
        let a2482 = v1;
        if nat_lt(a2481, a2482) {
            Ok(f513(v0, f610({
                let a2483 = v1;
                let a2484 = v3;
                nat_sub(a2483, a2484)
            }, v2)?))
        } else {
            Ok(f513(v0, v2))
        }
    }
}

pub fn f593(v0: i64) -> R<u64> {
    f611(20u64, v0, 0u64)
}

pub fn f594(v0: List<Adt41>) -> List<Adt6> {
    f214(f612(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f595(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2493 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2485, _, _, _) => {
                h2485
            }
        })), List::<(Str, Adt6)>::nil());
        let a2494 = {
            let a2491 = f321(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2486, _, _) => {
                    h2486
                }
            });
            let a2492 = {
                let a2489 = f321(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h2487, _) => {
                        h2487
                    }
                });
                let a2490 = f326(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h2488) => {
                        h2488
                    }
                });
                append_list(a2489, a2490)
            };
            append_list(a2491, a2492)
        };
        append_list(a2493, a2494)
    })
}

pub fn f596(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2495 = v0;
    match m2495 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f278(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f597(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2496 = v0;
    match m2496 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f613(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f598(v0: u64, v1: List<u64>) -> Adt148 {
    let a2497 = v0;
    let a2498 = 32u64;
    if nat_lt(a2497, a2498) {
        if f614(v0) {
            Adt148::C0(0u64, List::cons(v0, v1))
        } else {
            Adt148::C0(0u64, f599(v1))
        }
    } else {
        let a2499 = v0;
        let a2500 = 127u64;
        if nat_eq(a2499, a2500) {
            Adt148::C0(0u64, f599(v1))
        } else {
            let a2501 = v0;
            let a2502 = 194u64;
            if nat_eq(a2501, a2502) {
                Adt148::C0(1u64, v1)
            } else {
                let a2503 = v0;
                let a2504 = 226u64;
                if nat_eq(a2503, a2504) {
                    Adt148::C0(2u64, v1)
                } else {
                    Adt148::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f599(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f600(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt116, Adt119> {
    if v2 {
        Ok::<Adt116, Adt119>(f476(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt116, Adt119>(Adt119::C4(Adt84::C8(f615(f602(v1)))))
    }
}

pub fn f601(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2505 = f616(v1.clone());
    if if if bool_not(a2505) {
        true
    } else {
        f350(match v0.clone() {
            Adt17::C0(_, _, _, h2506, _) => {
                h2506
            }
        })
    } {
        let a2507 = f617(v1.clone());
        if bool_not(a2507) {
            true
        } else {
            f350(match v0.clone() {
                Adt17::C0(_, _, h2508, _, _) => {
                    h2508
                }
            })
        }
    } else {
        false
    } {
        let a2509 = f618(v1);
        if bool_not(a2509) {
            true
        } else {
            f350(match v0 {
                Adt17::C0(_, h2510, _, _, _) => {
                    h2510
                }
            })
        }
    } else {
        false
    }
}

pub fn f602(v0: Adt81) -> List<(Str, Adt64)> {
    let m2511 = v0;
    match m2511 {
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

pub fn f603(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt116, Adt119> {
    if v2 {
        Ok::<Adt116, Adt119>(f476(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt116, Adt119>(Adt119::C4(Adt84::C8(f619())))
    }
}

pub fn f604(v0: Adt17) -> bool {
    f351(match v0 {
        Adt17::C0(_, _, _, _, h2512) => {
            h2512
        }
    }, f352())
}

pub fn f605(v0: Adt16, v1: List<Adt50>) -> List<Adt118> {
    let m2514 = match v0 {
        Adt16::C0(_, _, _, _, h2513, _) => {
            h2513
        }
    };
    match m2514 {
        None => {
            List::<Adt118>::nil()
        }
        Some(v2) => {
            f620(f621(v2, v1, List::<Adt118>::nil()), List::<Adt118>::nil())
        }
    }
}

pub fn f606(p2520: List<Str>, p2521: Adt5) -> Adt123 {
    let m2519 = (p2520, p2521);
    {
        let mut m2519 = m2519;
        loop {
            let step = {
                let (v0, v1) = m2519;
                let m2515 = v0;
                match m2515.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                    }
                    Some((v2, v3)) => {
                        let m2516 = f111(v2, v1);
                        match m2516 {
                            Adt123::C0 => {
                                Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                            }
                            Adt123::C1(v4) => {
                                let m2517 = v3.clone();
                                match m2517.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt123>(Adt123::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2518 = f239(v4);
                                        match m2518 {
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
                Ok(next) => { m2519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f607(v0: Adt99) -> Option<Adt97> {
    let m2523 = match v0.clone() {
        Adt99::C0(_, _, _, h2522) => {
            h2522
        }
    };
    match m2523 {
        Adt100::C0 => {
            None::<Adt97>
        }
        Adt100::C1 => {
            Some(Adt97::C10(match v0 {
                Adt99::C0(h2524, _, _, _) => {
                    h2524
                }
            }))
        }
        Adt100::C2(_) => {
            None::<Adt97>
        }
    }
}

pub fn f608(v0: Adt6) -> bool {
    let m2525 = v0;
    match m2525 {
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

pub fn f609(v0: Adt99, v1: Adt6) -> R<Option<Adt97>> {
    let m2527 = match v0.clone() {
        Adt99::C0(_, _, _, h2526) => {
            h2526
        }
    };
    match m2527 {
        Adt100::C0 => {
            Ok(Some(Adt97::C8(match v0 {
                Adt99::C0(h2528, _, _, _) => {
                    h2528
                }
            })))
        }
        Adt100::C1 => {
            Ok(Some(Adt97::C10(match v0 {
                Adt99::C0(h2529, _, _, _) => {
                    h2529
                }
            })))
        }
        Adt100::C2(v2) => {
            if f622(match v0.clone() {
                Adt99::C0(_, _, h2530, _) => {
                    h2530
                }
            }, v2, v1)? {
                Ok(None::<Adt97>)
            } else {
                Ok(Some(Adt97::C9(match v0 {
                    Adt99::C0(h2531, _, _, _) => {
                        h2531
                    }
                })))
            }
        }
    }
}

pub fn f610(p2536: u64, p2537: i64) -> R<i64> {
    let m2535 = (p2536, p2537);
    {
        let mut m2535 = m2535;
        loop {
            let step = {
                let (v0, v1) = m2535;
                let m2532 = v0;
                if m2532 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2532 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2533 = v1;
                        let a2534 = 10i64;
                        int_mul(a2533, a2534)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f611(p2548: u64, p2549: i64, p2550: u64) -> R<u64> {
    let m2547 = (p2548, (p2549, p2550));
    {
        let mut m2547 = m2547;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2547;
                let m2538 = v0;
                if m2538 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2538 - 1;
                    let v4: u64 = {
                        let a2539 = v2;
                        let a2540 = 1u64;
                        nat_add(a2539, a2540)?
                    };
                    let m2543 = {
                        let a2541 = v1;
                        let a2542 = 10i64;
                        compare(a2541, a2542)
                    };
                    if match m2543 {
                        Ordering::Less => {
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
                            let a2544 = v1;
                            let a2545 = 10i64;
                            let a2546 = 0i64;
                            int_quot(a2544, a2545, a2546)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2547 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f612(p2553: List<Adt41>, p2554: List<Adt6>) -> List<Adt6> {
    let m2552 = (p2553, p2554);
    {
        let mut m2552 = m2552;
        loop {
            let step = {
                let (v0, v1) = m2552;
                let m2551 = v0;
                match m2551.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f526(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f613(v0: Adt3) -> Str {
    let m2555 = v0;
    match m2555 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f614(v0: u64) -> bool {
    let a2556 = v0;
    let a2557 = 9u64;
    if nat_eq(a2556, a2557) {
        true
    } else {
        let a2558 = v0;
        let a2559 = 10u64;
        if nat_eq(a2558, a2559) {
            true
        } else {
            let a2560 = v0;
            let a2561 = 13u64;
            nat_eq(a2560, a2561)
        }
    }
}

pub fn f615(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f618(v0.clone()) {
        Some(f419())
    } else {
        None::<Adt5>
    }, if f617(v0.clone()) {
        Some(f419())
    } else {
        None::<Adt5>
    }, if f616(v0) {
        Some(f419())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f616(p2566: List<(Str, Adt64)>) -> bool {
    let m2565 = p2566;
    {
        let mut m2565 = m2565;
        loop {
            let step = {
                let v0 = m2565;
                let m2562 = v0;
                match m2562.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2564 = {
                            let (_, h2563) = v1;
                            h2563
                        };
                        match m2564 {
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
                Ok(next) => { m2565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f617(p2571: List<(Str, Adt64)>) -> bool {
    let m2570 = p2571;
    {
        let mut m2570 = m2570;
        loop {
            let step = {
                let v0 = m2570;
                let m2567 = v0;
                match m2567.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2569 = {
                            let (_, h2568) = v1;
                            h2568
                        };
                        match m2569 {
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
                Ok(next) => { m2570 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(p2576: List<(Str, Adt64)>) -> bool {
    let m2575 = p2576;
    {
        let mut m2575 = m2575;
        loop {
            let step = {
                let v0 = m2575;
                let m2572 = v0;
                match m2572.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2574 = {
                            let (_, h2573) = v1;
                            h2573
                        };
                        match m2574 {
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
                Ok(next) => { m2575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f619() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f352(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f620(p2579: List<Adt118>, p2580: List<Adt118>) -> List<Adt118> {
    let m2578 = (p2579, p2580);
    {
        let mut m2578 = m2578;
        loop {
            let step = {
                let (v0, v1) = m2578;
                let m2577 = v0;
                match m2577.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<Adt118>), List<Adt118>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt118>, List<Adt118>), List<Adt118>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2578 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: Adt19, p2583: List<Adt50>, p2584: List<Adt118>) -> List<Adt118> {
    let m2582 = (p2583, p2584);
    {
        let mut m2582 = m2582;
        loop {
            let step = {
                let (v1, v2) = m2582;
                let m2581 = v1;
                match m2581.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt118>), List<Adt118>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt118>), List<Adt118>>((v4, List::cons(Adt118::C0(v0.clone(), v3), v2)))
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

pub fn f622(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2585 = v0;
    match m2585 {
        Adt13::C0 => {
            let m2586 = f151(v2);
            match m2586 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2587 = v1;
                    let a2588 = v3;
                    Ok(equal(a2587, a2588))
                }
            }
        }
        Adt13::C1 => {
            f623(v1, v2)
        }
        Adt13::C2 => {
            let m2589 = f238(v2);
            match m2589 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2590 = v1;
                        let a2591 = Str::lit("true");
                        Ok(equal(a2590, a2591))
                    } else {
                        let a2592 = v1;
                        let a2593 = Str::lit("false");
                        Ok(equal(a2592, a2593))
                    }
                }
            }
        }
    }
}

pub fn f623(v0: Str, v1: Adt6) -> R<bool> {
    let m2594 = f445(v1);
    match m2594 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2595 = f153(v2)?;
            match m2595 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2597 = f624(f55({
                        let a2596 = v0;
                        utf8_encode(a2596)
                    })?)?;
                    match m2597 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f299({
                                let a2598 = v3;
                                let a2599 = v4;
                                int_sub(a2598, a2599)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f624(v0: List<u64>) -> R<Option<i64>> {
    let m2600 = v0.clone();
    match m2600.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2601 = v1;
            let a2602 = 45u64;
            if nat_eq(a2601, a2602) {
                let m2603 = v2.clone();
                match m2603.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2604 = f625(v2, 0i64)?;
                        match m2604 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2605 = v5;
                                    int_neg(a2605)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f625(v0, 0i64)
            }
        }
    }
}

pub fn f625(p2611: List<u64>, p2612: i64) -> R<Option<i64>> {
    let m2610 = (p2611, p2612);
    {
        let mut m2610 = m2610;
        loop {
            let step = {
                let (v0, v1) = m2610;
                let m2606 = v0;
                match m2606.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2607 = f626(v2);
                        match m2607 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f123({
                                    let a2608 = v1;
                                    let a2609 = 10i64;
                                    int_mul(a2608, a2609)?
                                }, f213(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(v0: u64) -> Option<u64> {
    let a2613 = v0;
    let a2614 = 48u64;
    if if nat_lt(a2613, a2614) {
        true
    } else {
        let a2615 = 57u64;
        let a2616 = v0;
        nat_lt(a2615, a2616)
    } {
        None::<u64>
    } else {
        Some({
            let a2617 = v0;
            let a2618 = 48u64;
            nat_sub(a2617, a2618)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
