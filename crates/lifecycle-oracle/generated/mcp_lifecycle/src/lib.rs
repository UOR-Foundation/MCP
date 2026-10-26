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
    C0(Adt50),
    C1(Adt100, Adt50, Adt86),
    C2(Adt82),
}

#[derive(Clone)]
pub enum Adt100 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt101 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt102 {
    C0(Adt50),
    C1(Adt82),
}

#[derive(Clone)]
pub enum Adt103 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt104 {
    C0(Adt85, bool),
}

#[derive(Clone)]
pub enum Adt105 {
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
pub enum Adt106 {
    C0,
    C1(Option<Adt101>, Adt107),
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
    C8,
}

#[derive(Clone)]
pub enum Adt108 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt111 {
    C0,
    C1,
    C2(Adt5),
}

#[derive(Clone)]
pub enum Adt112 {
    C0(Adt101, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt113 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt114 {
    C0,
    C1(List<Adt115>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt115 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1(List<Adt115>, Str),
}

#[derive(Clone)]
pub enum Adt117 {
    C0(Adt6, Adt6, List<Adt115>),
    C1(List<Adt6>, Adt6, List<Adt115>),
    C2(List<Adt6>, Adt6, List<Adt115>, u64),
    C3(List<Adt115>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt115>),
    C5(Adt118, Adt6, Adt6, List<Adt115>),
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
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt120 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt121>, List<Adt123>),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt122),
    C7(Adt122),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt122 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt123 {
    C0(Str, List<Str>, Adt13, Adt124),
}

#[derive(Clone)]
pub enum Adt124 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt125 {
    C0,
    C1(u64),
    C2(u64),
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
    C0(Adt128, List<Adt132>, u64),
}

#[derive(Clone)]
pub enum Adt128 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt129, List<u64>, bool),
    C7(u64, u64),
    C8(Adt130),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt129 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt130 {
    C0(bool, Adt131, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt131 {
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
pub enum Adt132 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt133 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0(Adt116, List<Adt117>),
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Adt101, Result<Adt138, Adt141>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt33, Adt139, List<Adt140>),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt140 {
    C0(Adt20, Adt51),
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Adt142),
    C1(Str, List<Str>),
    C2(Adt121),
    C3(Str),
    C4(Adt90),
}

#[derive(Clone)]
pub enum Adt142 {
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
pub enum Adt143 {
    C0(Option<Str>, Option<Adt121>),
}

#[derive(Clone)]
pub enum Adt144 {
    C0,
    C1(Adt101),
    C2,
}

#[derive(Clone)]
pub enum Adt145 {
    C0(Adt105),
    C1(Adt127),
}

#[derive(Clone)]
pub enum Adt146 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt117>),
}

#[derive(Clone)]
pub enum Adt147 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt148 {
    C0(Adt105),
    C1(List<Adt119>, List<u64>),
}

#[derive(Clone)]
pub enum Adt149 {
    C0(Adt101, Adt150, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt150 {
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
pub enum Adt151 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt152 {
    C0,
    C1(Adt150),
}

#[derive(Clone)]
pub enum Adt153 {
    C0(Adt154),
    C1(Adt141),
}

#[derive(Clone)]
pub enum Adt154 {
    C0(Adt101, Str, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt155 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt156 {
    C0(Adt98),
    C1(Adt133),
}

#[derive(Clone)]
pub enum Adt157 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt158 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt159 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt160 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt161 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt162 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt163 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt164 {
    C0,
    C1(Adt19),
}

#[derive(Clone)]
pub enum Adt165 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt166 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt167 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt166, u64),
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
    F35,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt22) -> R<Option<Adt99>> {
        match self {
            Fn1::F35 => f35(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F85,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt22) -> R<Result<Adt102, Adt90>> {
        match self {
            Fn2::F85 => f85(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F87,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt22) -> R<Result<Adt50, Adt90>> {
        match self {
            Fn3::F87 => f87(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F112(Fn0, Adt92),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt15) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn4::F112(k0, k1) => f112(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F142(Fn2, Adt36),
    F263(Fn3, Adt36),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt22) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn5::F142(k0, k1) => f142(k0.clone(), k1.clone(), p0),
            Fn5::F263(k0, k1) => f263(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F173,
}

impl Fn6 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt47>>> {
        match self {
            Fn6::F173 => f173(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F185,
}

impl Fn7 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt59 {
        match self {
            Fn7::F185 => f185(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F187,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt25) -> R<Result<List<Str>, Adt90>> {
        match self {
            Fn8::F187 => f187(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F1035(Bytes),
    F1060(Bytes, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn9::F1035(k0) => f1035(k0.clone(), p0),
            Fn9::F1060(k0, k1) => f1060(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F1040(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn10::F1040(k0) => f1040(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F1045(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn11::F1045(k0) => f1045(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F1068(Bytes, u64, u64, u64),
    F1075(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt165)) -> R<(u64, Adt165)> {
        match self {
            Fn12::F1068(k0, k1, k2, k3) => f1068(k0.clone(), *k1, *k2, *k3, p0),
            Fn12::F1075(k0, k1, k2) => f1075(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F1070,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt167) -> R<Adt167> {
        match self {
            Fn13::F1070 => f1070(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F1100,
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn14::F1100 => f1100(p0),
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

pub fn f3(v0: Adt14, v1: Adt15) -> R<Result<Adt33, Adt90>> {
    let m2 = f14(f12()?)?;
    match m2 {
        None => {
            f15(v0, v1)
        }
        Some(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f4(v0: Bytes) -> R<Result<Adt92, Adt94>> {
    let m3 = f16(v0.clone(), 0u64)?;
    match m3 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(0u64)))
        }
        Some(v1) => {
            f17(v0.clone(), v1.clone(), f16(v0, match v1 {
                Adt96::C0(_, h4) => {
                    h4
                }
            })?)
        }
    }
}

pub fn f5(v0: Adt95) -> List<u64> {
    let a28 = f18(match v0.clone() {
        Adt95::C0(h5, _, _, _, _) => {
            h5
        }
    });
    let a29 = {
        let a26 = f18({
            let a7 = match v0.clone() {
                Adt95::C0(_, h6, _, _, _) => {
                    h6
                }
            };
            length_list(a7)
        });
        let a27 = {
            let a24 = f19(match v0.clone() {
                Adt95::C0(_, h8, _, _, _) => {
                    h8
                }
            });
            let a25 = {
                let a22 = f18({
                    let a10 = match v0.clone() {
                        Adt95::C0(_, _, h9, _, _) => {
                            h9
                        }
                    };
                    length_list(a10)
                });
                let a23 = {
                    let a20 = f20(match v0.clone() {
                        Adt95::C0(_, _, h11, _, _) => {
                            h11
                        }
                    });
                    let a21 = {
                        let a18 = f18({
                            let a13 = match v0.clone() {
                                Adt95::C0(_, _, _, h12, _) => {
                                    h12
                                }
                            };
                            length_list(a13)
                        });
                        let a19 = {
                            let a16 = match v0.clone() {
                                Adt95::C0(_, _, _, h14, _) => {
                                    h14
                                }
                            };
                            let a17 = f21(match v0 {
                                Adt95::C0(_, _, _, _, h15) => {
                                    h15
                                }
                            });
                            append_list(a16, a17)
                        };
                        append_list(a18, a19)
                    };
                    append_list(a20, a21)
                };
                append_list(a22, a23)
            };
            append_list(a24, a25)
        };
        append_list(a26, a27)
    };
    append_list(a28, a29)
}

pub fn f6() -> Adt95 {
    Adt95::C0(f22(), f23(), List::<List<u64>>::nil(), f24(), None::<List<u64>>)
}

pub fn f7(v0: Adt0, v1: Fn0, v2: Adt92) -> R<Adt95> {
    f25(v0, v1, v2.clone(), f26(match v2.clone() {
        Adt92::C0(_, _, h30, _, _, _, _, _) => {
            h30
        }
    }, match v2 {
        Adt92::C0(_, _, _, _, _, h31, _, _) => {
            h31
        }
    })?)
}

pub fn f8() -> Adt1 {
    Adt1::C0(Str::lit("mcp-core-fixture"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt2>>)
}

pub fn f9() -> Adt4 {
    Adt4::C0(None::<Adt5>, None::<Adt5>, Some(f27()), Some(Adt8::C0(None::<bool>)), Some(Adt9::C0(None::<bool>, None::<bool>)), Some(Adt8::C0(None::<bool>)), Some(Adt5::C0(List::cons((f28(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f10() -> Adt10 {
    Adt10::C0(0u64, Adt11::C1)
}

pub fn f11(v0: List<Adt36>) -> R<List<Adt12>> {
    let m32 = f29(v0)?;
    match m32 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt12>::nil())
        }
    }
}

pub fn f12() -> R<List<Adt36>> {
    let a35 = {
        let a33 = f30()?;
        let a34 = f31();
        append_list(a33, a34)
    };
    let a36 = f32();
    Ok(append_list(a35, a36))
}

pub fn f13() -> u64 {
    100u64
}

pub fn f14(v0: List<Adt36>) -> R<Option<Adt90>> {
    let m37 = f29(v0)?;
    match m37 {
        Ok(_) => {
            Ok(None::<Adt90>)
        }
        Err(v1) => {
            Ok(Some(Adt90::C9(f33(v1))))
        }
    }
}

pub fn f15(v0: Adt14, v1: Adt15) -> R<Result<Adt33, Adt90>> {
    let m38 = v1.clone();
    match m38 {
        Adt15::C0(_) => {
            f36(v1)
        }
        Adt15::C1(_) => {
            f36(v1)
        }
        Adt15::C2(v2) => {
            f34(v0, f12()?, Fn1::F35, f36(v1)?, v2)
        }
        Adt15::C3(_) => {
            f36(v1)
        }
        Adt15::C4(_) => {
            f36(v1)
        }
        Adt15::C5(_) => {
            f36(v1)
        }
        Adt15::C6(_) => {
            f36(v1)
        }
        Adt15::C7(_) => {
            f36(v1)
        }
        Adt15::C8(_) => {
            f36(v1)
        }
        Adt15::C9(v3) => {
            f37(v0, v3)
        }
        Adt15::C10(v4) => {
            f38(v0, v4)
        }
        Adt15::C11(v5) => {
            f39(v0, v5)
        }
    }
}

pub fn f16(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let m39 = f40(v0.clone(), v1)?;
    match m39 {
        None => {
            Ok(None::<Adt96>)
        }
        Some(v2) => {
            let m45 = {
                let a42 = v0;
                let a43 = {
                    let a40 = v1;
                    let a41 = 4u64;
                    nat_add(a40, a41)?
                };
                let a44 = v2;
                slice_bytes(a42, a43, a44)
            };
            match m45 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v3) => {
                    Ok(Some(Adt96::C0(v3, {
                        let a48 = {
                            let a46 = v1;
                            let a47 = 4u64;
                            nat_add(a46, a47)?
                        };
                        let a49 = v2;
                        nat_add(a48, a49)?
                    })))
                }
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: Adt96, v2: Option<Adt96>) -> R<Result<Adt92, Adt94>> {
    let m50 = v2;
    match m50 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(match v1 {
                Adt96::C0(_, h51) => {
                    h51
                }
            })))
        }
        Some(v3) => {
            f41(v0.clone(), v1, v3.clone(), f40(v0, match v3 {
                Adt96::C0(_, h52) => {
                    h52
                }
            })?)
        }
    }
}

pub fn f18(v0: u64) -> List<u64> {
    List::cons({
        let a56 = {
            let a53 = v0;
            let a54 = 16777216u64;
            let a55 = 0u64;
            nat_quot(a53, a54, a55)
        };
        let a57 = 256u64;
        let a58 = 0u64;
        nat_rem(a56, a57, a58)
    }, List::cons({
        let a62 = {
            let a59 = v0;
            let a60 = 65536u64;
            let a61 = 0u64;
            nat_quot(a59, a60, a61)
        };
        let a63 = 256u64;
        let a64 = 0u64;
        nat_rem(a62, a63, a64)
    }, List::cons({
        let a68 = {
            let a65 = v0;
            let a66 = 256u64;
            let a67 = 0u64;
            nat_quot(a65, a66, a67)
        };
        let a69 = 256u64;
        let a70 = 0u64;
        nat_rem(a68, a69, a70)
    }, List::cons({
        let a71 = v0;
        let a72 = 256u64;
        let a73 = 0u64;
        nat_rem(a71, a72, a73)
    }, List::<u64>::nil()))))
}

pub fn f19(v0: List<Adt93>) -> List<u64> {
    f42(f43(v0, List::<Adt93>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f44(f45(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21(v0: Option<List<u64>>) -> List<u64> {
    let m74 = v0;
    match m74 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a78 = f18(1u64);
            let a79 = {
                let a76 = f18({
                    let a75 = v1.clone();
                    length_list(a75)
                });
                let a77 = v1;
                append_list(a76, a77)
            };
            append_list(a78, a79)
        }
    }
}

pub fn f22() -> u64 {
    500u64
}

pub fn f23() -> List<Adt93> {
    List::cons(Adt93::C0(f46(), f47()), List::<Adt93>::nil())
}

pub fn f24() -> List<u64> {
    let a82 = f48();
    let a83 = {
        let a80 = f49();
        let a81 = f50();
        append_list(a80, a81)
    };
    append_list(a82, a83)
}

pub fn f25(v0: Adt0, v1: Fn0, v2: Adt92, v3: Option<Adt97>) -> R<Adt95> {
    let m84 = v3;
    match m84 {
        None => {
            f52(v0, v1, v2)
        }
        Some(v4) => {
            f51(v4)
        }
    }
}

pub fn f26(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m85 = f53(v0.clone(), v1.clone())?;
    match m85 {
        None => {
            f54(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f27() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f28() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f29(v0: List<Adt36>) -> R<Result<List<Adt12>, Adt98>> {
    let m86 = f55(v0.clone());
    match m86 {
        None => {
            f56(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f30() -> R<List<Adt36>> {
    Ok(List::cons(f57(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f58()), List::cons(f57(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f59()), List::cons(f57(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f58()), List::cons(f57(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f60()), List::cons(f57(Str::lit("test_image_content"), Str::lit("Tests image content response"), f60()), List::cons(f57(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f60()), List::cons(f57(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f60()), List::cons(f57(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f60()), List::cons(f57(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f60()), List::cons(f57(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f60()), List::cons(f57(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f61()), List::cons(f57(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f62()?), List::cons(f57(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f63()), List::cons(f64(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f60(), f65()), List::cons(f64(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f60(), f65()), List::cons(f57(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f60()), List::cons(f57(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f60()), List::cons(f57(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f60()), List::cons(f57(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f60()), List::cons(f57(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f60()), List::<Adt36>::nil())))))))))))))))))))))
}

pub fn f31() -> List<Adt36> {
    List::cons(f66(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f66(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f66(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f66(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f66(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f66(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f66(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f66(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt36>::nil()))))))))
}

pub fn f32() -> List<Adt36> {
    List::cons(f67(Str::lit("greet"), Str::lit("Answers a greeting at once; never a task"), f68(Str::lit("name"))), List::cons(f67(Str::lit("slow_compute"), Str::lit("Computes for a number of seconds; a task for a client that declared the tasks extension"), f69()), List::cons(f67(Str::lit("failing_job"), Str::lit("Always fails; always a task, so the client must declare the tasks extension"), f70()), List::cons(f67(Str::lit("protocol_error_job"), Str::lit("Ends in a protocol error; a task for a client that declared the tasks extension"), f70()), List::cons(f67(Str::lit("confirm_delete"), Str::lit("Asks the client to confirm a deletion; a task that waits for input"), f68(Str::lit("filename"))), List::cons(f67(Str::lit("multi_input"), Str::lit("Asks the client two questions at once; a task that waits for both answers"), f70()), List::cons(f67(Str::lit("test_tool_with_task"), Str::lit("Asks for a name, then becomes a task; the client must declare the tasks extension"), f70()), List::<Adt36>::nil())))))))
}

pub fn f33(v0: Adt98) -> Str {
    let m87 = v0;
    match m87 {
        Adt98::C0(v1) => {
            let a88 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a89 = Str::lit(" ");
            join(a88, a89)
        }
        Adt98::C1(v2) => {
            let a90 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a91 = Str::lit(" ");
            join(a90, a91)
        }
        Adt98::C2(v3) => {
            let a92 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a93 = Str::lit(" ");
            join(a92, a93)
        }
        Adt98::C3(v4) => {
            let a94 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a95 = Str::lit(" ");
            join(a94, a95)
        }
        Adt98::C4(v5, v6) => {
            let a96 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a97 = Str::lit(" ");
            join(a96, a97)
        }
        Adt98::C5(v7, v8) => {
            let a98 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a99 = Str::lit(" ");
            join(a98, a99)
        }
        Adt98::C6(v9, v10) => {
            let a100 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a101 = Str::lit(" ");
            join(a100, a101)
        }
        Adt98::C7(v11) => {
            let a102 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a103 = Str::lit(" ");
            join(a102, a103)
        }
        Adt98::C8(v12) => {
            let a104 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a105 = Str::lit(" ");
            join(a104, a105)
        }
        Adt98::C9(v13) => {
            let a106 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a107 = Str::lit(" ");
            join(a106, a107)
        }
        Adt98::C10(v14) => {
            let a108 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a109 = Str::lit(" ");
            join(a108, a109)
        }
        Adt98::C11(v15) => {
            let a110 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a111 = Str::lit(" ");
            join(a110, a111)
        }
        Adt98::C12(v16, v17) => {
            let a112 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a113 = Str::lit(" ");
            join(a112, a113)
        }
    }
}

pub fn f34(v0: Adt14, v1: List<Adt36>, v2: Fn1, v3: Result<Adt33, Adt90>, v4: Adt22) -> R<Result<Adt33, Adt90>> {
    let m115 = f71(match v4.clone() {
        Adt22::C0(_, h114, _, _, _) => {
            h114
        }
    }, v1);
    match m115 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v4 {
                Adt22::C0(_, h116, _, _, _) => {
                    h116
                }
            })))
        }
        Some(v5) => {
            f72(v0, v5, v4.clone(), {
                let c117: Fn1 = v2;
                c117.apply(v4)?
            }, v3)
        }
    }
}

pub fn f35(v0: Adt22) -> R<Option<Adt99>> {
    if f73(v0.clone(), Str::lit("greet")) {
        Ok(Some(Adt99::C0(f74({
            let a118 = List::cons(Str::lit("Hello, "), List::cons(f75(v0, Str::lit("name"), Str::lit("world")), List::cons(Str::lit("!"), List::<Str>::nil())));
            let a119 = Str::lit("");
            join(a118, a119)
        }, false))))
    } else {
        if f73(v0.clone(), Str::lit("slow_compute")) {
            Ok(Some(f76(v0)?))
        } else {
            if f73(v0.clone(), Str::lit("confirm_delete")) {
                Ok(Some(f77(v0)?))
            } else {
                if f73(v0.clone(), Str::lit("multi_input")) {
                    Ok(Some(f78()?))
                } else {
                    if f73(v0.clone(), Str::lit("failing_job")) {
                        Ok(Some(f79()?))
                    } else {
                        if f73(v0.clone(), Str::lit("protocol_error_job")) {
                            Ok(Some(f80()))
                        } else {
                            if f73(v0.clone(), Str::lit("test_tool_with_task")) {
                                Ok(Some(f81(v0)?))
                            } else {
                                Ok(None::<Adt99>)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f36(v0: Adt15) -> R<Result<Adt33, Adt90>> {
    let m120 = v0;
    match m120 {
        Adt15::C0(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt15::C1(v1) => {
            Ok(f82(f10(), f12()?, v1))
        }
        Adt15::C2(v2) => {
            if f83(match v2.clone() {
                Adt22::C0(_, h121, _, _, _) => {
                    h121
                }
            }) {
                f84(f12()?, Fn2::F85, v2)
            } else {
                f86(f12()?, Fn3::F87, v2)
            }
        }
        Adt15::C3(v4) => {
            Ok(f88(v4))
        }
        Adt15::C4(v5) => {
            f89(v5)
        }
        Adt15::C5(v6) => {
            Ok(f90(v6))
        }
        Adt15::C6(v7) => {
            Ok(f91(v7))
        }
        Adt15::C7(v8) => {
            if f92(match v8.clone() {
                Adt24::C0(_, h122, _, _, _) => {
                    h122
                }
            }) {
                Ok(f93(v8))
            } else {
                Ok(f94(v8))
            }
        }
        Adt15::C8(v9) => {
            f95(v9)
        }
        Adt15::C9(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt15::C10(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt15::C11(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the fixture offers tools only"))))
        }
    }
}

pub fn f37(v0: Adt14, v1: Adt30) -> R<Result<Adt33, Adt90>> {
    let m125 = f96(v0.clone(), match v1.clone() {
        Adt30::C0(h123, _) => {
            h123
        }
    }, match v1 {
        Adt30::C0(_, h124) => {
            h124
        }
    })?;
    match m125 {
        Ok(v3) => {
            f97(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f38(v0: Adt14, v1: Adt31) -> R<Result<Adt33, Adt90>> {
    let m128 = f96(v0.clone(), match v1.clone() {
        Adt31::C0(h126, _, _) => {
            h126
        }
    }, match v1.clone() {
        Adt31::C0(_, h127, _) => {
            h127
        }
    })?;
    match m128 {
        Ok(v3) => {
            f98(v0, v3, match v1 {
                Adt31::C0(_, _, h129) => {
                    h129
                }
            })
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f39(v0: Adt14, v1: Adt32) -> R<Result<Adt33, Adt90>> {
    let m132 = f96(v0.clone(), match v1.clone() {
        Adt32::C0(h130, _) => {
            h130
        }
    }, match v1 {
        Adt32::C0(_, h131) => {
            h131
        }
    })?;
    match m132 {
        Ok(v3) => {
            f99(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f40(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a136 = {
        let a133 = v1;
        let a134 = 4u64;
        nat_add(a133, a134)?
    };
    let a137 = {
        let a135 = v0.clone();
        length_bytes(a135)
    };
    if nat_le(a136, a137) {
        Ok(Some({
            let a154 = {
                let a138 = f100(v0.clone(), v1)?;
                let a139 = 16777216u64;
                nat_mul(a138, a139)?
            };
            let a155 = {
                let a152 = {
                    let a142 = f100(v0.clone(), {
                        let a140 = v1;
                        let a141 = 1u64;
                        nat_add(a140, a141)?
                    })?;
                    let a143 = 65536u64;
                    nat_mul(a142, a143)?
                };
                let a153 = {
                    let a150 = {
                        let a146 = f100(v0.clone(), {
                            let a144 = v1;
                            let a145 = 2u64;
                            nat_add(a144, a145)?
                        })?;
                        let a147 = 256u64;
                        nat_mul(a146, a147)?
                    };
                    let a151 = f100(v0, {
                        let a148 = v1;
                        let a149 = 3u64;
                        nat_add(a148, a149)?
                    })?;
                    nat_add(a150, a151)?
                };
                nat_add(a152, a153)?
            };
            nat_add(a154, a155)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f41(v0: Bytes, v1: Adt96, v2: Adt96, v3: Option<u64>) -> R<Result<Adt92, Adt94>> {
    let m156 = v3;
    match m156 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(match v2 {
                Adt96::C0(_, h157) => {
                    h157
                }
            })))
        }
        Some(v4) => {
            let m161 = f101(v0.clone(), {
                let a159 = match v2.clone() {
                    Adt96::C0(_, h158) => {
                        h158
                    }
                };
                let a160 = 4u64;
                nat_add(a159, a160)?
            }, v4, List::<Adt93>::nil())?;
            match m161 {
                Ok(v6) => {
                    f102(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt92, Adt94>(v5))
                }
            }
        }
    }
}

pub fn f42(p166: List<Adt93>, p167: List<u64>) -> List<u64> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v0, v1) = m165;
                let m162 = v0;
                match m162.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<u64>), List<u64>>((v3, {
                            let a163 = f103(v2);
                            let a164 = v1;
                            append_list(a163, a164)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(p170: List<Adt93>, p171: List<Adt93>) -> List<Adt93> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v0, v1) = m169;
                let m168 = v0;
                match m168.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
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

pub fn f44(p176: List<List<u64>>, p177: List<u64>) -> List<u64> {
    let m175 = (p176, p177);
    {
        let mut m175 = m175;
        loop {
            let step = {
                let (v0, v1) = m175;
                let m172 = v0;
                match m172.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a173 = f104(v2);
                            let a174 = v1;
                            append_list(a173, a174)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(p180: List<List<u64>>, p181: List<List<u64>>) -> List<List<u64>> {
    let m179 = (p180, p181);
    {
        let mut m179 = m179;
        loop {
            let step = {
                let (v0, v1) = m179;
                let m178 = v0;
                match m178.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
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

pub fn f46() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f47() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f48() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f49() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f50() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f51(v0: Adt97) -> R<Adt95> {
    f105(f106(v0.clone()), f107(None::<Adt101>, f108(v0)))
}

pub fn f52(v0: Adt0, v1: Fn0, v2: Adt92) -> R<Adt95> {
    if f109(match v2.clone() {
        Adt92::C0(h182, _, _, _, _, _, _, _) => {
            h182
        }
    }) {
        f110(match v2.clone() {
            Adt92::C0(_, _, _, _, _, _, _, h183) => {
                h183
            }
        }, f111(v0, match v2.clone() {
            Adt92::C0(_, _, _, _, _, _, h184, _) => {
                h184
            }
        }), Fn4::F112(v1, v2.clone()), match v2.clone() {
            Adt92::C0(_, _, _, _, h185, _, _, _) => {
                h185
            }
        }, match v2.clone() {
            Adt92::C0(_, _, h186, _, _, _, _, _) => {
                h186
            }
        }, f113(match v2 {
            Adt92::C0(_, _, _, h187, _, _, _, _) => {
                h187
            }
        })?)
    } else {
        Ok(f114())
    }
}

pub fn f53(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m188 = f115(v0, f116())?;
    match m188 {
        Adt108::C0 => {
            Ok(None::<Adt97>)
        }
        Adt108::C1 => {
            Ok(Some(Adt97::C0))
        }
        Adt108::C2 => {
            Ok(Some(Adt97::C0))
        }
        Adt108::C3(v2) => {
            f117(f118(v2), v1, Adt97::C0)
        }
    }
}

pub fn f54(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m189 = f115(v0, f119())?;
    match m189 {
        Adt108::C0 => {
            Ok(None::<Adt97>)
        }
        Adt108::C1 => {
            Ok(Some(Adt97::C1))
        }
        Adt108::C2 => {
            Ok(Some(Adt97::C1))
        }
        Adt108::C3(v2) => {
            f117(f120(v2), v1, Adt97::C1)
        }
    }
}

pub fn f55(v0: List<Adt36>) -> Option<Adt98> {
    let m190 = f121(v0, List::<Str>::nil());
    match m190 {
        None => {
            None::<Adt98>
        }
        Some(v1) => {
            Some(Adt98::C1(v1))
        }
    }
}

pub fn f56(p194: List<Adt36>, p195: List<Adt12>) -> R<Result<List<Adt12>, Adt98>> {
    let m193 = (p194, p195);
    {
        let mut m193 = m193;
        loop {
            let step = {
                let (v0, v1) = m193;
                let m191 = v0;
                match m191.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>(Ok(Ok::<List<Adt12>, Adt98>(f122(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m192 = f123(v2)?;
                        match m192 {
                            Ok(v5) => {
                                Ok::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>((v3, f122(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>(Ok(Err::<List<Adt12>, Adt98>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Str, v1: Str, v2: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f58() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f59() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f124(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f125()), List::cons((Str::lit("limit"), f124(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f124(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f60() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f61() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f62() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f125()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f126(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f126(0u64)?), List::cons((Str::lit("maximum"), f126(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f126(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
}

pub fn f63() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("target"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("zone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt6::C3(Str::lit("Zone"))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f64(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt37>, None::<Adt5>)
}

pub fn f65() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("sum"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("sum")), List::cons(Adt6::C3(Str::lit("label")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f66(v0: Str, v1: Str) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))), None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f67(v0: Str, v1: Str, v2: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f68(v0: Str) -> Adt5 {
    f127(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v0.clone(), f128(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v0), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f69() -> Adt5 {
    f127(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("seconds"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), Adt6::C2(Adt7::C0(0i64, 0i64))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("label"), f128(Str::lit("string"))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("seconds")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f70() -> Adt5 {
    f127(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f71(v0: Str, p201: List<Adt36>) -> Option<Adt36> {
    let m200 = p201;
    {
        let mut m200 = m200;
        loop {
            let step = {
                let v1 = m200;
                let m196 = v1;
                match m196.uncons() {
                    None => {
                        Err::<List<Adt36>, Option<Adt36>>(None::<Adt36>)
                    }
                    Some((v2, v3)) => {
                        let a198 = match v2.clone() {
                            Adt36::C0(h197, _, _, _, _, _, _, _) => {
                                h197
                            }
                        };
                        let a199 = v0.clone();
                        if equal(a198, a199) {
                            Err::<List<Adt36>, Option<Adt36>>(Some(v2))
                        } else {
                            Ok::<List<Adt36>, Option<Adt36>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt14, v1: Adt36, v2: Adt22, v3: Option<Adt99>, v4: Result<Adt33, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m202 = v3;
    match m202 {
        None => {
            Ok(v4)
        }
        Some(v5) => {
            f129(v0, v1.clone(), v2.clone(), v5, f130(v1, v2)?)
        }
    }
}

pub fn f73(v0: Adt22, v1: Str) -> bool {
    let a204 = match v0 {
        Adt22::C0(_, h203, _, _, _) => {
            h203
        }
    };
    let a205 = v1;
    equal(a204, a205)
}

pub fn f74(v0: Str, v1: bool) -> Adt50 {
    Adt50::C0(Adt38::C0(List::cons(f131(v0), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt51>::nil())
}

pub fn f75(v0: Adt22, v1: Str, v2: Str) -> Str {
    let m206 = f132(v0, v1);
    match m206 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            let m207 = f133(v3);
            match m207 {
                None => {
                    v2
                }
                Some(v4) => {
                    v4
                }
            }
        }
    }
}

pub fn f76(v0: Adt22) -> R<Adt99> {
    let v1: i64 = f134(v0.clone());
    let v2: Str = {
        let a209 = List::cons(Str::lit("slow_compute"), List::cons(f75(v0, Str::lit("label"), Str::lit("unnamed")), List::cons(Str::lit("finished after"), List::cons({
            let a208 = v1;
            format_i64(a208)
        }, List::cons(Str::lit("seconds"), List::<Str>::nil())))));
        let a210 = Str::lit(" ");
        join(a209, a210)
    };
    Ok(Adt99::C1(Adt100::C0, f74(v2.clone(), false), f135(0i64, List::<(Str, Adt65)>::nil(), {
        let a211 = v1;
        let a212 = 1000i64;
        int_mul(a211, a212)?
    }, f136(v2, false)?)))
}

pub fn f77(v0: Adt22) -> R<Adt99> {
    let v1: Str = {
        let a213 = List::cons(Str::lit("Deleted"), List::cons(f75(v0.clone(), Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a214 = Str::lit(" ");
        join(a213, a214)
    };
    Ok(Adt99::C1(Adt100::C0, f74(v1.clone(), false), f135(0i64, List::cons((Str::lit("confirm"), f137({
        let a215 = List::cons(Str::lit("Delete"), List::cons(f75(v0, Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a216 = Str::lit(" ");
        join(a215, a216)
    }, Str::lit("confirm"), Str::lit("boolean"))), List::<(Str, Adt65)>::nil()), 0i64, f136(v1, false)?)))
}

pub fn f78() -> R<Adt99> {
    Ok(Adt99::C1(Adt100::C0, f74(Str::lit("Both inputs received"), false), f135(0i64, List::cons((Str::lit("first"), f137(Str::lit("First name"), Str::lit("name"), Str::lit("string"))), List::cons((Str::lit("second"), f137(Str::lit("Second name"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt65)>::nil())), 0i64, f136(Str::lit("Both inputs received"), false)?)))
}

pub fn f79() -> R<Adt99> {
    Ok(Adt99::C1(Adt100::C1, f74(Str::lit("failing_job always fails"), true), f135(0i64, List::<(Str, Adt65)>::nil(), 1000i64, f136(Str::lit("failing_job always fails"), true)?)))
}

pub fn f80() -> Adt99 {
    Adt99::C1(Adt100::C0, f74(Str::lit("protocol_error_job has no result"), true), f135(0i64, List::<(Str, Adt65)>::nil(), 500i64, Adt87::C1(Adt81::C0(-32603i64, Str::lit("protocol_error_job failed"), None::<Adt6>))))
}

pub fn f81(v0: Adt22) -> R<Adt99> {
    let m218 = match v0 {
        Adt22::C0(_, _, _, h217, _) => {
            h217
        }
    };
    match m218 {
        None => {
            Ok(f138())
        }
        Some(v1) => {
            let m219 = f139(v1);
            match m219 {
                None => {
                    Ok(f138())
                }
                Some(v2) => {
                    f140(v2)
                }
            }
        }
    }
}

pub fn f82(v0: Adt10, v1: List<Adt36>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m221 = match v2 {
        Adt21::C0(_, h220) => {
            h220
        }
    };
    match m221 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C1(Adt35::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f83(v0: Str) -> bool {
    let a222 = v0.clone();
    let a223 = Str::lit("test_input_required_result_elicitation");
    if equal(a222, a223) {
        true
    } else {
        let a224 = v0.clone();
        let a225 = Str::lit("test_input_required_result_sampling");
        if equal(a224, a225) {
            true
        } else {
            let a226 = v0.clone();
            let a227 = Str::lit("test_input_required_result_list_roots");
            if equal(a226, a227) {
                true
            } else {
                let a228 = v0.clone();
                let a229 = Str::lit("test_input_required_result_request_state");
                if equal(a228, a229) {
                    true
                } else {
                    let a230 = v0.clone();
                    let a231 = Str::lit("test_input_required_result_multiple_inputs");
                    if equal(a230, a231) {
                        true
                    } else {
                        let a232 = v0.clone();
                        let a233 = Str::lit("test_input_required_result_multi_round");
                        if equal(a232, a233) {
                            true
                        } else {
                            let a234 = v0.clone();
                            let a235 = Str::lit("test_input_required_result_tampered_state");
                            if equal(a234, a235) {
                                true
                            } else {
                                let a236 = v0;
                                let a237 = Str::lit("test_input_required_result_capabilities");
                                equal(a236, a237)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f84(v0: List<Adt36>, v1: Fn2, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m239 = f71(match v2.clone() {
        Adt22::C0(_, h238, _, _, _) => {
            h238
        }
    }, v0);
    match m239 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h240, _, _, _) => {
                    h240
                }
            })))
        }
        Some(v3) => {
            f141(v3.clone(), v2, Fn5::F142(v1, v3))
        }
    }
}

pub fn f85(v0: Adt22) -> R<Result<Adt102, Adt90>> {
    if f143(v0.clone(), Str::lit("test_input_required_result_elicitation")) {
        Ok(f144(v0))
    } else {
        if f143(v0.clone(), Str::lit("test_input_required_result_sampling")) {
            Ok(f145(v0))
        } else {
            if f143(v0.clone(), Str::lit("test_input_required_result_list_roots")) {
                f146(v0)
            } else {
                if f143(v0.clone(), Str::lit("test_input_required_result_request_state")) {
                    Ok(f147(v0))
                } else {
                    if f143(v0.clone(), Str::lit("test_input_required_result_multiple_inputs")) {
                        f148(v0)
                    } else {
                        if f143(v0.clone(), Str::lit("test_input_required_result_multi_round")) {
                            f149(v0)
                        } else {
                            if f143(v0.clone(), Str::lit("test_input_required_result_tampered_state")) {
                                Ok(f150(v0))
                            } else {
                                if f143(v0.clone(), Str::lit("test_input_required_result_capabilities")) {
                                    Ok(f151(v0))
                                } else {
                                    Ok(Err::<Adt102, Adt90>(Adt90::C9(Str::lit("not an input-required tool"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f86(v0: List<Adt36>, v1: Fn3, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m242 = f71(match v2.clone() {
        Adt22::C0(_, h241, _, _, _) => {
            h241
        }
    }, v0);
    match m242 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h243, _, _, _) => {
                    h243
                }
            })))
        }
        Some(v3) => {
            f152(v3, v1, v2)
        }
    }
}

pub fn f87(v0: Adt22) -> R<Result<Adt50, Adt90>> {
    if f153(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt50, Adt90>(f154(List::cons(f155(Str::lit("This is a simple text response for testing.")), List::<Adt39>::nil()))))
    } else {
        if f153(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt50, Adt90>(f154(List::cons(f156(), List::<Adt39>::nil()))))
        } else {
            if f153(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt50, Adt90>(f154(List::cons(f157(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt39>::nil()))))
            } else {
                if f153(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt50, Adt90>(f154(List::cons(f158(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt39>::nil()))))
                } else {
                    if f153(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt50, Adt90>(f154(List::cons(f155(Str::lit("Multiple content types test:")), List::cons(f156(), List::cons(f158(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt39>::nil()))))))
                    } else {
                        if f153(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt50, Adt90>(f159(List::cons(f155(Str::lit("This tool intentionally returns an error for testing")), List::<Adt39>::nil()), None::<Adt6>, true, List::<Adt51>::nil())))
                        } else {
                            if f153(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt50, Adt90>(f160(v0, f161()?)))
                            } else {
                                if f153(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt50, Adt90>(f160(v0, f162()?)))
                                } else {
                                    if f153(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt50, Adt90>(f159(List::cons(f155(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt39>::nil()), Some(f163()?), false, List::<Adt51>::nil())))
                                    } else {
                                        if f153(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt50, Adt90>(f159(List::cons(f155(Str::lit("{\"sum\":\"three\"}")), List::<Adt39>::nil()), Some(f164()), false, List::<Adt51>::nil())))
                                        } else {
                                            if f153(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt50, Adt90>(f154(List::cons(f165(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt39>::nil()))))
                                            } else {
                                                if f153(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt50, Adt90>(f154(List::cons(f155(f166()), List::<Adt39>::nil()))))
                                                } else {
                                                    if f153(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt50, Adt90>(f154(List::cons(f155(Str::lit("Streaming complete")), List::<Adt39>::nil()))))
                                                    } else {
                                                        if f153(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt50, Adt90>(f154(List::cons(f155(Str::lit("Logging evaluated")), List::<Adt39>::nil()))))
                                                        } else {
                                                            Ok(f167(v0))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f88(v0: Adt21) -> Result<Adt33, Adt90> {
    f168(f169(), f170(), v0)
}

pub fn f89(v0: Adt23) -> R<Result<Adt33, Adt90>> {
    f171(f172(), Fn6::F173, v0)
}

pub fn f90(v0: Adt21) -> Result<Adt33, Adt90> {
    f174(f169(), f175(), v0)
}

pub fn f91(v0: Adt21) -> Result<Adt33, Adt90> {
    f176(f169(), f177(), v0)
}

pub fn f92(v0: Str) -> bool {
    let a244 = v0;
    let a245 = f178();
    equal(a244, a245)
}

pub fn f93(v0: Adt24) -> Result<Adt33, Adt90> {
    let m247 = f179(Str::lit("user_context"), Adt103::C0, match v0 {
        Adt24::C0(_, _, _, h246, _) => {
            h246
        }
    });
    match m247 {
        Adt111::C0 => {
            Ok::<Adt33, Adt90>(Adt33::C12(Adt82::C0(f182(Str::lit("user_context"), f183(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt111::C1 => {
            Err::<Adt33, Adt90>(Adt90::C11)
        }
        Adt111::C2(v1) => {
            Ok::<Adt33, Adt90>(Adt33::C8(f180(f181(v1, Str::lit("context"), Str::lit("unknown")))))
        }
    }
}

pub fn f94(v0: Adt24) -> Result<Adt33, Adt90> {
    f184(f177(), Fn7::F185, v0)
}

pub fn f95(v0: Adt25) -> R<Result<Adt33, Adt90>> {
    f186(Fn8::F187, v0)
}

pub fn f96(v0: Adt14, v1: Adt17, v2: Str) -> R<Result<Adt85, Adt90>> {
    if f188(match v1 {
        Adt17::C0(_, h248, _, _, _, _) => {
            h248
        }
    }) {
        let m250 = f189(match v0.clone() {
            Adt14::C0(_, _, _, h249) => {
                h249
            }
        })?;
        match m250 {
            None => {
                Ok(Err::<Adt85, Adt90>(f190()))
            }
            Some(v3) => {
                let m252 = f191(v2.clone(), v3, match v0 {
                    Adt14::C0(h251, _, _, _) => {
                        h251
                    }
                })?;
                match m252 {
                    None => {
                        Ok(Err::<Adt85, Adt90>(Adt90::C3(v2)))
                    }
                    Some(v4) => {
                        Ok(Ok::<Adt85, Adt90>(v4))
                    }
                }
            }
        }
    } else {
        Ok(Err::<Adt85, Adt90>(f192()))
    }
}

pub fn f97(v0: Adt14, v1: Adt85) -> R<Result<Adt33, Adt90>> {
    Ok(Ok::<Adt33, Adt90>(f193(f194(v1, match v0.clone() {
        Adt14::C0(h253, _, _, _) => {
            h253
        }
    }, match v0 {
        Adt14::C0(_, h254, _, _) => {
            h254
        }
    })?, Adt89::C0, false)))
}

pub fn f98(v0: Adt14, v1: Adt85, v2: Adt5) -> R<Result<Adt33, Adt90>> {
    let v3: Adt104 = f194(v1, match v0.clone() {
        Adt14::C0(h255, _, _, _) => {
            h255
        }
    }, match v0.clone() {
        Adt14::C0(_, h256, _, _) => {
            h256
        }
    })?;
    Ok(Ok::<Adt33, Adt90>(f193(f195(match v3.clone() {
        Adt104::C0(h257, _) => {
            h257
        }
    }, v2, match v0.clone() {
        Adt14::C0(h258, _, _, _) => {
            h258
        }
    }, match v0 {
        Adt14::C0(_, h259, _, _) => {
            h259
        }
    })?, Adt89::C1, match v3 {
        Adt104::C0(_, h260) => {
            h260
        }
    })))
}

pub fn f99(v0: Adt14, v1: Adt85) -> R<Result<Adt33, Adt90>> {
    Ok(Ok::<Adt33, Adt90>(f193(f196(v1, match v0.clone() {
        Adt14::C0(h261, _, _, _) => {
            h261
        }
    }, match v0 {
        Adt14::C0(_, h262, _, _) => {
            h262
        }
    })?, Adt89::C1, false)))
}

pub fn f100(v0: Bytes, v1: u64) -> R<u64> {
    let m265 = {
        let a263 = v0;
        let a264 = v1;
        index_bytes(a263, a264)
    };
    match m265 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a310 = {
                let a286 = {
                    let a274 = {
                        let a268 = {
                            let a266 = v3;
                            let a267 = 128u8;
                            fixed_u8::bit_and(a266, a267)
                        };
                        let a269 = 128u8;
                        if equal(a268, a269) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a275 = {
                        let a272 = {
                            let a270 = v3;
                            let a271 = 64u8;
                            fixed_u8::bit_and(a270, a271)
                        };
                        let a273 = 64u8;
                        if equal(a272, a273) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a274, a275)?
                };
                let a287 = {
                    let a284 = {
                        let a278 = {
                            let a276 = v3;
                            let a277 = 32u8;
                            fixed_u8::bit_and(a276, a277)
                        };
                        let a279 = 32u8;
                        if equal(a278, a279) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a285 = {
                        let a282 = {
                            let a280 = v3;
                            let a281 = 16u8;
                            fixed_u8::bit_and(a280, a281)
                        };
                        let a283 = 16u8;
                        if equal(a282, a283) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a284, a285)?
                };
                nat_add(a286, a287)?
            };
            let a311 = {
                let a308 = {
                    let a296 = {
                        let a290 = {
                            let a288 = v3;
                            let a289 = 8u8;
                            fixed_u8::bit_and(a288, a289)
                        };
                        let a291 = 8u8;
                        if equal(a290, a291) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a297 = {
                        let a294 = {
                            let a292 = v3;
                            let a293 = 4u8;
                            fixed_u8::bit_and(a292, a293)
                        };
                        let a295 = 4u8;
                        if equal(a294, a295) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a296, a297)?
                };
                let a309 = {
                    let a306 = {
                        let a300 = {
                            let a298 = v3;
                            let a299 = 2u8;
                            fixed_u8::bit_and(a298, a299)
                        };
                        let a301 = 2u8;
                        if equal(a300, a301) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a307 = {
                        let a304 = {
                            let a302 = v3;
                            let a303 = 1u8;
                            fixed_u8::bit_and(a302, a303)
                        };
                        let a305 = 1u8;
                        if equal(a304, a305) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a306, a307)?
                };
                nat_add(a308, a309)?
            };
            nat_add(a310, a311)
        }
    }
}

pub fn f101(v0: Bytes, p321: u64, p322: u64, p323: List<Adt93>) -> R<Result<(List<Adt93>, u64), Adt94>> {
    let m320 = (p321, (p322, p323));
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v1, (v2, v3)) = m320;
                let m312 = v2;
                if m312 == 0 {
                    Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Ok::<(List<Adt93>, u64), Adt94>((f197(v3, List::<Adt93>::nil()), v1))))
                } else {
                    let v4 = m312 - 1;
                    let m313 = f16(v0.clone(), v1)?;
                    match m313 {
                        None => {
                            Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Err::<(List<Adt93>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            let m315 = f16(v0.clone(), match v5.clone() {
                                Adt96::C0(_, h314) => {
                                    h314
                                }
                            })?;
                            match m315 {
                                None => {
                                    Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Err::<(List<Adt93>, u64), Adt94>(Adt94::C0(match v5 {
                                        Adt96::C0(_, h316) => {
                                            h316
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>((match v6.clone() {
                                        Adt96::C0(_, h317) => {
                                            h317
                                        }
                                    }, (v4, List::cons(Adt93::C0(f198(match v5 {
                                        Adt96::C0(h318, _) => {
                                            h318
                                        }
                                    })?, f198(match v6 {
                                        Adt96::C0(h319, _) => {
                                            h319
                                        }
                                    })?), v3))))
                                }
                            }
                        }
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

pub fn f102(v0: Bytes, v1: Adt96, v2: Adt96, v3: (List<Adt93>, u64)) -> R<Result<Adt92, Adt94>> {
    let m325 = f16(v0.clone(), {
        let (_, h324) = v3.clone();
        h324
    })?;
    match m325 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0({
                let (_, h326) = v3;
                h326
            })))
        }
        Some(v4) => {
            let m328 = f40(v0.clone(), match v4.clone() {
                Adt96::C0(_, h327) => {
                    h327
                }
            })?;
            match m328 {
                None => {
                    Ok(Err::<Adt92, Adt94>(Adt94::C0(match v4 {
                        Adt96::C0(_, h329) => {
                            h329
                        }
                    })))
                }
                Some(v5) => {
                    f199(v0.clone(), Adt92::C0(f198(match v1 {
                        Adt96::C0(h330, _) => {
                            h330
                        }
                    })?, f198(match v2 {
                        Adt96::C0(h331, _) => {
                            h331
                        }
                    })?, {
                        let (h332, _) = v3;
                        h332
                    }, match v4.clone() {
                        Adt96::C0(h333, _) => {
                            h333
                        }
                    }, v5, List::<List<u64>>::nil(), Bytes::lit(&[]), Adt14::C0(0i64, Str::lit(""), Str::lit(""), f200())), f201(v0, {
                        let a335 = match v4 {
                            Adt96::C0(_, h334) => {
                                h334
                            }
                        };
                        let a336 = 4u64;
                        nat_add(a335, a336)?
                    })?)
                }
            }
        }
    }
}

pub fn f103(v0: Adt93) -> List<u64> {
    let a347 = f18({
        let a338 = match v0.clone() {
            Adt93::C0(h337, _) => {
                h337
            }
        };
        length_list(a338)
    });
    let a348 = {
        let a345 = match v0.clone() {
            Adt93::C0(h339, _) => {
                h339
            }
        };
        let a346 = {
            let a343 = f18({
                let a341 = match v0.clone() {
                    Adt93::C0(_, h340) => {
                        h340
                    }
                };
                length_list(a341)
            });
            let a344 = match v0 {
                Adt93::C0(_, h342) => {
                    h342
                }
            };
            append_list(a343, a344)
        };
        append_list(a345, a346)
    };
    append_list(a347, a348)
}

pub fn f104(v0: List<u64>) -> List<u64> {
    let a350 = f18({
        let a349 = v0.clone();
        length_list(a349)
    });
    let a351 = v0;
    append_list(a350, a351)
}

pub fn f105(v0: u64, v1: Adt6) -> R<Adt95> {
    let m352 = f202(v1)?;
    match m352 {
        Ok(v3) => {
            Ok(Adt95::C0(v0, f23(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f106(v0: Adt97) -> u64 {
    let m353 = v0;
    match m353 {
        Adt97::C0 => {
            f203()
        }
        Adt97::C1 => {
            f203()
        }
    }
}

pub fn f107(v0: Option<Adt101>, v1: Adt81) -> Adt6 {
    Adt6::C5(f204(v0, v1))
}

pub fn f108(v0: Adt97) -> Adt81 {
    Adt81::C0(f205(), f206(v0), None::<Adt6>)
}

pub fn f109(v0: List<u64>) -> bool {
    f207(v0, f208())
}

pub fn f110(v0: Adt14, v1: Adt0, v2: Fn4, v3: u64, v4: List<Adt93>, v5: Result<Adt6, Adt105>) -> R<Adt95> {
    let m354 = v5;
    match m354 {
        Ok(v7) => {
            f211(v0, v1, v2, v3, v4, f212(v7)?)
        }
        Err(_) => {
            f105(f209(), f107(None::<Adt101>, f210(Adt106::C0)))
        }
    }
}

pub fn f111(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h355, _, _, _, _, _, _, _) => {
            h355
        }
    }, match v0.clone() {
        Adt0::C0(_, h356, _, _, _, _, _, _) => {
            h356
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h357, _, _, _, _, _) => {
            h357
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h358, _, _, _, _) => {
            h358
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h359, _, _, _) => {
            h359
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h360, _, _) => {
            h360
        }
    }, v1, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h361) => {
            h361
        }
    })
}

pub fn f112(v0: Fn0, v1: Adt92, v2: Adt15) -> R<Result<Adt33, Adt90>> {
    let c362: Fn0 = v0;
    c362.apply(match v1 {
        Adt92::C0(_, _, _, _, _, _, _, h363) => {
            h363
        }
    }, v2)
}

pub fn f113(v0: Bytes) -> R<Result<Adt6, Adt105>> {
    f213(f198(v0)?)
}

pub fn f114() -> Adt95 {
    Adt95::C0(f214(), List::cons(Adt93::C0(f215(), f216()), List::<Adt93>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f115(v0: List<Adt93>, v1: List<u64>) -> R<Adt108> {
    let m364 = f217(v0, v1, Adt109::C0)?;
    match m364 {
        Adt109::C0 => {
            Ok(Adt108::C0)
        }
        Adt109::C1(v2) => {
            let m365 = f218(v2);
            match m365 {
                None => {
                    Ok(Adt108::C1)
                }
                Some(v3) => {
                    Ok(Adt108::C3(v3))
                }
            }
        }
        Adt109::C2 => {
            Ok(Adt108::C2)
        }
    }
}

pub fn f116() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f117(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt97) -> R<Option<Adt97>> {
    let m366 = v0;
    match m366 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f219(v3, v1)? {
                Ok(None::<Adt97>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f118(v0: List<u64>) -> Option<List<u64>> {
    let m367 = f220(v0);
    match m367 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f120(v1)
        }
    }
}

pub fn f119() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f120(v0: List<u64>) -> Option<List<u64>> {
    let m368 = v0.clone();
    match m368.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a369 = v1;
            let a370 = 91u64;
            if nat_eq(a369, a370) {
                f221(f222(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f221(f223(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f121(p376: List<Adt36>, p377: List<Str>) -> Option<Str> {
    let m375 = (p376, p377);
    {
        let mut m375 = m375;
        loop {
            let step = {
                let (v0, v1) = m375;
                let m371 = v0;
                match m371.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f224(match v2.clone() {
                            Adt36::C0(h372, _, _, _, _, _, _, _) => {
                                h372
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt36>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt36::C0(h373, _, _, _, _, _, _, _) => {
                                    h373
                                }
                            }))
                        } else {
                            Ok::<(List<Adt36>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt36::C0(h374, _, _, _, _, _, _, _) => {
                                    h374
                                }
                            }, v1)))
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

pub fn f122(p380: List<Adt12>, p381: List<Adt12>) -> List<Adt12> {
    let m379 = (p380, p381);
    {
        let mut m379 = m379;
        loop {
            let step = {
                let (v0, v1) = m379;
                let m378 = v0;
                match m378.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Adt36) -> R<Result<List<Adt12>, Adt98>> {
    let m384 = f225(match v0.clone() {
        Adt36::C0(h382, _, _, _, _, _, _, _) => {
            h382
        }
    }, f226(match v0.clone() {
        Adt36::C0(h383, _, _, _, _, _, _, _) => {
            h383
        }
    })?);
    match m384 {
        None => {
            f227(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f124(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f125() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f126(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f228(v0, 0i64)?, 0i64)))
}

pub fn f127(v0: List<(Str, Adt6)>) -> Adt5 {
    Adt5::C0(v0)
}

pub fn f128(v0: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::<(Str, Adt6)>::nil()))
}

pub fn f129(v0: Adt14, v1: Adt36, v2: Adt22, v3: Adt99, v4: Result<Option<Adt33>, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m385 = v4;
    match m385 {
        Ok(v6) => {
            let m386 = v6;
            match m386 {
                None => {
                    f229(v0, v1, v2, v3)
                }
                Some(v7) => {
                    Ok(Ok::<Adt33, Adt90>(v7))
                }
            }
        }
        Err(v5) => {
            Ok(Err::<Adt33, Adt90>(v5))
        }
    }
}

pub fn f130(v0: Adt36, v1: Adt22) -> R<Result<Option<Adt33>, Adt90>> {
    let m393 = f230(Adt6::C5({
        let m389 = match v0.clone() {
            Adt36::C0(_, _, _, _, h388, _, _, _) => {
                h388
            }
        };
        match m389 {
            Adt5::C0(h387) => {
                h387
            }
        }
    }), Adt6::C5({
        let m392 = match v0 {
            Adt36::C0(_, _, _, _, h391, _, _, _) => {
                h391
            }
        };
        match m392 {
            Adt5::C0(h390) => {
                h390
            }
        }
    }), f231(v1))?;
    match m393 {
        Adt114::C0 => {
            Ok(Ok::<Option<Adt33>, Adt90>(None::<Adt33>))
        }
        Adt114::C1(v2, v3) => {
            Ok(Ok::<Option<Adt33>, Adt90>(Some(Adt33::C2(f232(f233(v2, v3)?, true)))))
        }
        Adt114::C2(_) => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt114::C3(_) => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt114::C4 => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f131(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f132(v0: Adt22, v1: Str) -> Adt110 {
    let m395 = match v0 {
        Adt22::C0(_, _, h394, _, _) => {
            h394
        }
    };
    match m395 {
        None => {
            Adt110::C0
        }
        Some(v2) => {
            f234(v1, v2)
        }
    }
}

pub fn f133(v0: Adt6) -> Option<Str> {
    let m396 = v0;
    match m396 {
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

pub fn f134(v0: Adt22) -> i64 {
    let m397 = f132(v0, Str::lit("seconds"));
    match m397 {
        Adt110::C0 => {
            0i64
        }
        Adt110::C1(v1) => {
            let m398 = f235(v1);
            match m398 {
                None => {
                    0i64
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f135(v0: i64, v1: List<(Str, Adt65)>, v2: i64, v3: Adt87) -> Adt86 {
    Adt86::C0(v0, v1, v2, v3)
}

pub fn f136(v0: Str, v1: bool) -> R<Adt87> {
    let m401 = f236(f237({
        let m400 = f74(v0, v1);
        match m400 {
            Adt50::C0(h399, _) => {
                h399
            }
        }
    })?)?;
    match m401 {
        None => {
            Ok(f238())
        }
        Some(v2) => {
            Ok(Adt87::C0(Adt5::C0(v2)))
        }
    }
}

pub fn f137(v0: Str, v1: Str, v2: Str) -> Adt65 {
    Adt65::C0(Adt66::C0(Adt67::C0(Some(Adt68::C0), v0, f127(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v1.clone(), f128(v2)), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v1), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))))))
}

pub fn f138() -> Adt99 {
    Adt99::C2(Adt82::C0(List::cons((Str::lit("user_name"), f137(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt65)>::nil())))
}

pub fn f139(v0: Adt5) -> Option<Str> {
    let m402 = f234(Str::lit("user_name"), v0);
    match m402 {
        Adt110::C0 => {
            None::<Str>
        }
        Adt110::C1(v1) => {
            let m403 = f239(v1);
            match m403 {
                None => {
                    None::<Str>
                }
                Some(v2) => {
                    let m404 = f234(Str::lit("content"), v2);
                    match m404 {
                        Adt110::C0 => {
                            None::<Str>
                        }
                        Adt110::C1(v3) => {
                            let m405 = f239(v3);
                            match m405 {
                                None => {
                                    None::<Str>
                                }
                                Some(v4) => {
                                    let m406 = f234(Str::lit("name"), v4);
                                    match m406 {
                                        Adt110::C0 => {
                                            None::<Str>
                                        }
                                        Adt110::C1(v5) => {
                                            f133(v5)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f140(v0: Str) -> R<Adt99> {
    let v1: Str = {
        let a407 = List::cons(Str::lit("Hello, "), List::cons(v0, List::cons(Str::lit("! (from a task)"), List::<Str>::nil())));
        let a408 = Str::lit("");
        join(a407, a408)
    };
    Ok(Adt99::C1(Adt100::C1, f74(v1.clone(), false), f135(0i64, List::<(Str, Adt65)>::nil(), 100i64, f136(v1, false)?)))
}

pub fn f141(v0: Adt36, v1: Adt22, v2: Fn5) -> R<Result<Adt33, Adt90>> {
    let m415 = f230(Adt6::C5({
        let m411 = match v0.clone() {
            Adt36::C0(_, _, _, _, h410, _, _, _) => {
                h410
            }
        };
        match m411 {
            Adt5::C0(h409) => {
                h409
            }
        }
    }), Adt6::C5({
        let m414 = match v0 {
            Adt36::C0(_, _, _, _, h413, _, _, _) => {
                h413
            }
        };
        match m414 {
            Adt5::C0(h412) => {
                h412
            }
        }
    }), f231(v1.clone()))?;
    match m415 {
        Adt114::C0 => {
            let c416: Fn5 = v2;
            c416.apply(v1)
        }
        Adt114::C1(v3, v4) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(f232(f233(v3, v4)?, true))))
        }
        Adt114::C2(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt114::C3(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt114::C4 => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f142(v0: Fn2, v1: Adt36, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    f240(v1, {
        let c417: Fn2 = v0;
        c417.apply(v2)?
    })
}

pub fn f143(v0: Adt22, v1: Str) -> bool {
    let a419 = match v0 {
        Adt22::C0(_, h418, _, _, _) => {
            h418
        }
    };
    let a420 = v1;
    equal(a419, a420)
}

pub fn f144(v0: Adt22) -> Result<Adt102, Adt90> {
    let m422 = f179(Str::lit("user_name"), Adt103::C0, match v0 {
        Adt22::C0(_, _, _, h421, _) => {
            h421
        }
    });
    match m422 {
        Adt111::C0 => {
            f243(f182(Str::lit("user_name"), f244()))
        }
        Adt111::C1 => {
            f245()
        }
        Adt111::C2(v1) => {
            f241(f242(List::cons(Str::lit("Hello, "), List::cons(f181(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f145(v0: Adt22) -> Result<Adt102, Adt90> {
    let m424 = f179(Str::lit("capital_question"), Adt103::C1, match v0 {
        Adt22::C0(_, _, _, h423, _) => {
            h423
        }
    });
    match m424 {
        Adt111::C0 => {
            f243(f182(Str::lit("capital_question"), f246(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt111::C1 => {
            f245()
        }
        Adt111::C2(v1) => {
            f241(f242(List::cons(Str::lit("Sampling result: "), List::cons(f181(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f146(v0: Adt22) -> R<Result<Adt102, Adt90>> {
    let m426 = f179(Str::lit("client_roots"), Adt103::C2, match v0 {
        Adt22::C0(_, _, _, h425, _) => {
            h425
        }
    });
    match m426 {
        Adt111::C0 => {
            Ok(f243(f182(Str::lit("client_roots"), f249())))
        }
        Adt111::C1 => {
            Ok(f245())
        }
        Adt111::C2(v1) => {
            Ok(f241(f242(List::cons(Str::lit("Found "), List::cons(f247(f248(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f147(v0: Adt22) -> Result<Adt102, Adt90> {
    f250(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f148(v0: Adt22) -> R<Result<Adt102, Adt90>> {
    if f251(v0.clone(), Str::lit("multiple-inputs")) {
        f252(f179(Str::lit("user_name"), Adt103::C0, match v0.clone() {
            Adt22::C0(_, _, _, h427, _) => {
                h427
            }
        }), f179(Str::lit("greeting"), Adt103::C1, match v0.clone() {
            Adt22::C0(_, _, _, h428, _) => {
                h428
            }
        }), f179(Str::lit("client_roots"), Adt103::C2, match v0 {
            Adt22::C0(_, _, _, h429, _) => {
                h429
            }
        }))
    } else {
        Ok(f253(f254(Adt111::C0, Adt111::C0, Adt111::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f149(v0: Adt22) -> R<Result<Adt102, Adt90>> {
    let v1: u64 = f255(f256(v0.clone()))?;
    let a430 = v1;
    let a431 = 49u64;
    if nat_eq(a430, a431) {
        Ok(f257(v0))
    } else {
        let a432 = v1;
        let a433 = 50u64;
        if nat_eq(a432, a433) {
            Ok(f258(v0))
        } else {
            Ok(f253(f182(Str::lit("step1"), f259()), Str::lit("1")))
        }
    }
}

pub fn f150(v0: Adt22) -> Result<Adt102, Adt90> {
    f250(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f151(v0: Adt22) -> Result<Adt102, Adt90> {
    let m435 = match v0.clone() {
        Adt22::C0(_, _, _, h434, _) => {
            h434
        }
    };
    match m435 {
        None => {
            f260(f261({
                let m438 = match v0 {
                    Adt22::C0(h437, _, _, _, _) => {
                        h437
                    }
                };
                match m438 {
                    Adt17::C0(_, h436, _, _, _, _) => {
                        h436
                    }
                }
            }))
        }
        Some(v1) => {
            let m440 = match v1.clone() {
                Adt5::C0(h439) => {
                    h439
                }
            };
            match m440.uncons() {
                None => {
                    f260(f261({
                        let m443 = match v0 {
                            Adt22::C0(h442, _, _, _, _) => {
                                h442
                            }
                        };
                        match m443 {
                            Adt17::C0(_, h441, _, _, _, _) => {
                                h441
                            }
                        }
                    }))
                }
                Some((_, _)) => {
                    f241(f242(List::cons(Str::lit("capabilities-ok: received "), List::cons({
                        let a445 = f262(match v1 {
                            Adt5::C0(h444) => {
                                h444
                            }
                        });
                        let a446 = Str::lit(",");
                        join(a445, a446)
                    }, List::<Str>::nil()))))
                }
            }
        }
    }
}

pub fn f152(v0: Adt36, v1: Fn3, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    f141(v0.clone(), v2, Fn5::F263(v1, v0))
}

pub fn f153(v0: Adt22, v1: Str) -> bool {
    let a448 = match v0 {
        Adt22::C0(_, h447, _, _, _) => {
            h447
        }
    };
    let a449 = v1;
    equal(a448, a449)
}

pub fn f154(v0: List<Adt39>) -> Adt50 {
    f159(v0, None::<Adt6>, false, List::<Adt51>::nil())
}

pub fn f155(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f156() -> Adt39 {
    f165(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f157(v0: Str, v1: Str) -> Adt39 {
    Adt39::C2(Adt44::C0(v0, v1, None::<Adt41>, None::<Adt5>))
}

pub fn f158(v0: Str, v1: Str, v2: Str) -> Adt39 {
    Adt39::C4(Adt46::C0(Adt47::C0(Adt48::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt41>, None::<Adt5>))
}

pub fn f159(v0: List<Adt39>, v1: Option<Adt6>, v2: bool, v3: List<Adt51>) -> Adt50 {
    Adt50::C0(Adt38::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f160(v0: Adt22, v1: List<Adt51>) -> Adt50 {
    f159(List::cons(f155(f264({
        let m452 = match v0 {
            Adt22::C0(h451, _, _, _, _) => {
                h451
            }
        };
        match m452 {
            Adt17::C0(_, _, _, _, h450, _) => {
                h450
            }
        }
    })), List::<Adt39>::nil()), None::<Adt6>, false, v1)
}

pub fn f161() -> R<List<Adt51>> {
    Ok(List::cons(f265(0u64, 100u64)?, List::cons(f265(50u64, 100u64)?, List::cons(f265(100u64, 100u64)?, List::<Adt51>::nil()))))
}

pub fn f162() -> R<List<Adt51>> {
    Ok(List::cons(f265(50u64, 100u64)?, List::cons(f265(10u64, 100u64)?, List::<Adt51>::nil())))
}

pub fn f163() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f126(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f164() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f165(v0: Str, v1: Str) -> Adt39 {
    Adt39::C1(Adt43::C0(v0, v1, None::<Adt41>, None::<Adt5>))
}

pub fn f166() -> Str {
    let m454 = {
        let a453 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a453)
    };
    match m454 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f167(v0: Adt22) -> Result<Adt50, Adt90> {
    if f153(v0.clone(), Str::lit("test_missing_capability")) {
        let m460 = {
            let m459 = {
                let m458 = match v0 {
                    Adt22::C0(h457, _, _, _, _) => {
                        h457
                    }
                };
                match m458 {
                    Adt17::C0(_, h456, _, _, _, _) => {
                        h456
                    }
                }
            };
            match m459 {
                Adt18::C0(_, _, h455, _, _) => {
                    h455
                }
            }
        };
        match m460 {
            None => {
                Err::<Adt50, Adt90>(Adt90::C8(f266()))
            }
            Some(_) => {
                Ok::<Adt50, Adt90>(f267())
            }
        }
    } else {
        if f153(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt50, Adt90>(f154(List::cons(f155(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt39>::nil())))
        } else {
            if f153(v0, Str::lit("test_arguments")) {
                Ok::<Adt50, Adt90>(f154(List::cons(f155(Str::lit("arguments accepted")), List::<Adt39>::nil())))
            } else {
                Ok::<Adt50, Adt90>(f267())
            }
        }
    }
}

pub fn f168(v0: Adt10, v1: List<Adt45>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m462 = match v2 {
        Adt21::C0(_, h461) => {
            h461
        }
    };
    match m462 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C4(Adt52::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f169() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C0)
}

pub fn f170() -> List<Adt45> {
    f268(f269())
}

pub fn f171(v0: Adt10, v1: Fn6, v2: Adt23) -> R<Result<Adt33, Adt90>> {
    Ok(f270(v0, match v2.clone() {
        Adt23::C0(_, h463, _, _) => {
            h463
        }
    }, {
        let c464: Fn6 = v1;
        c464.apply(match v2 {
            Adt23::C0(_, h465, _, _) => {
                h465
            }
        })?
    }))
}

pub fn f172() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C1)
}

pub fn f173(v0: Str) -> R<Option<List<Adt47>>> {
    let m466 = f271(v0.clone(), f269());
    match m466 {
        None => {
            f272(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f174(v0: Adt10, v1: List<Adt55>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m468 = match v2 {
        Adt21::C0(_, h467) => {
            h467
        }
    };
    match m468 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C6(Adt54::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f175() -> List<Adt55> {
    List::cons(Adt55::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt2>>, f273(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt41>, None::<Adt5>), List::<Adt55>::nil())
}

pub fn f176(v0: Adt10, v1: List<Adt57>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m470 = match v2 {
        Adt21::C0(_, h469) => {
            h469
        }
    };
    match m470 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C7(Adt56::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f177() -> List<Adt57> {
    List::cons(f274(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt58>>), List::cons(f274(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f275(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f275(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt58>::nil())))), List::cons(f274(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f275(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt58>::nil()))), List::cons(f274(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt58>>), List::<Adt57>::nil()))))
}

pub fn f178() -> Str {
    Str::lit("test_input_required_result_prompt")
}

pub fn f179(v0: Str, v1: Adt103, v2: Option<Adt5>) -> Adt111 {
    let m471 = v2;
    match m471 {
        None => {
            Adt111::C0
        }
        Some(v3) => {
            let m472 = f234(v0, v3);
            match m472 {
                Adt110::C0 => {
                    Adt111::C0
                }
                Adt110::C1(v4) => {
                    let m473 = f239(v4);
                    match m473 {
                        None => {
                            Adt111::C1
                        }
                        Some(v5) => {
                            if f276(v1, v5.clone()) {
                                Adt111::C2(v5)
                            } else {
                                Adt111::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f180(v0: Str) -> Adt59 {
    Adt59::C0(None::<Str>, List::cons(Adt60::C0(Adt42::C0, Adt39::C0(Adt40::C0(f242(List::cons(Str::lit("Prompt with context: "), List::cons(v0, List::<Str>::nil()))), None::<Adt41>, None::<Adt5>))), List::<Adt60>::nil()))
}

pub fn f181(v0: Adt5, v1: Str, v2: Str) -> Str {
    let m474 = f234(Str::lit("content"), v0);
    match m474 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            let m475 = f239(v3);
            match m475 {
                None => {
                    v2
                }
                Some(v4) => {
                    f277(v1, v4, v2)
                }
            }
        }
    }
}

pub fn f182(v0: Str, v1: Adt65) -> List<(Str, Adt65)> {
    List::cons((v0, v1), List::<(Str, Adt65)>::nil())
}

pub fn f183(v0: Str, v1: Str, v2: Str) -> Adt65 {
    Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v0, f278(v1, v2))))
}

pub fn f184(v0: List<Adt57>, v1: Fn7, v2: Adt24) -> Result<Adt33, Adt90> {
    let m477 = f279(match v2.clone() {
        Adt24::C0(_, h476, _, _, _) => {
            h476
        }
    }, v0);
    match m477 {
        None => {
            Err::<Adt33, Adt90>(Adt90::C1(match v2 {
                Adt24::C0(_, h478, _, _, _) => {
                    h478
                }
            }))
        }
        Some(v3) => {
            let m480 = f280(f281(v3), f282(match v2.clone() {
                Adt24::C0(_, _, h479, _, _) => {
                    h479
                }
            }));
            match m480 {
                None => {
                    Ok::<Adt33, Adt90>(Adt33::C8({
                        let c483: Fn7 = v1;
                        c483.apply(match v2.clone() {
                            Adt24::C0(_, h484, _, _, _) => {
                                h484
                            }
                        }, f282(match v2 {
                            Adt24::C0(_, _, h485, _, _) => {
                                h485
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt33, Adt90>(Adt90::C4({
                        let a481 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a482 = Str::lit(" ");
                        join(a481, a482)
                    }))
                }
            }
        }
    }
}

pub fn f185(v0: Str, v1: List<(Str, Str)>) -> Adt59 {
    let a486 = v0.clone();
    let a487 = Str::lit("test_simple_prompt");
    if equal(a486, a487) {
        f283(List::cons(f284(Str::lit("This is a simple prompt for testing.")), List::<Adt60>::nil()))
    } else {
        let a488 = v0.clone();
        let a489 = Str::lit("test_prompt_with_arguments");
        if equal(a488, a489) {
            f283(List::cons(f284({
                let a490 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f285(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f285(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a491 = Str::lit("");
                join(a490, a491)
            }), List::<Adt60>::nil()))
        } else {
            let a492 = v0.clone();
            let a493 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a492, a493) {
                f283(List::cons(Adt60::C0(Adt42::C0, Adt39::C4(Adt46::C0(f286(f285(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt41>, None::<Adt5>))), List::cons(f284(Str::lit("Please process the embedded resource above.")), List::<Adt60>::nil())))
            } else {
                let a494 = v0;
                let a495 = Str::lit("test_prompt_with_image");
                if equal(a494, a495) {
                    f283(List::cons(Adt60::C0(Adt42::C0, Adt39::C1(Adt43::C0(f287(), Str::lit("image/png"), None::<Adt41>, None::<Adt5>))), List::cons(f284(Str::lit("Please analyze the image above.")), List::<Adt60>::nil())))
                } else {
                    f283(List::<Adt60>::nil())
                }
            }
        }
    }
}

pub fn f186(v0: Fn8, v1: Adt25) -> R<Result<Adt33, Adt90>> {
    let m497 = {
        let c496: Fn8 = v0;
        c496.apply(v1)?
    };
    match m497 {
        Ok(v3) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C9(f288(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f187(v0: Adt25) -> R<Result<List<Str>, Adt90>> {
    let m499 = match v0.clone() {
        Adt25::C0(_, h498, _, _) => {
            h498
        }
    };
    match m499 {
        Adt26::C0(v1) => {
            let m501 = f279(match v1.clone() {
                Adt27::C0(h500, _) => {
                    h500
                }
            }, f177());
            match m501 {
                None => {
                    Ok(Err::<List<Str>, Adt90>(Adt90::C1(match v1 {
                        Adt27::C0(h502, _) => {
                            h502
                        }
                    })))
                }
                Some(v2) => {
                    if f289(f281(v2), {
                        let m505 = match v0.clone() {
                            Adt25::C0(_, _, h504, _) => {
                                h504
                            }
                        };
                        match m505 {
                            Adt28::C0(h503, _) => {
                                h503
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt90>(f290({
                            let m508 = match v0.clone() {
                                Adt25::C0(_, _, h507, _) => {
                                    h507
                                }
                            };
                            match m508 {
                                Adt28::C0(_, h506) => {
                                    h506
                                }
                            }
                        }, f291(match v1 {
                            Adt27::C0(h509, _) => {
                                h509
                            }
                        }, {
                            let m512 = match v0 {
                                Adt25::C0(_, _, h511, _) => {
                                    h511
                                }
                            };
                            match m512 {
                                Adt28::C0(h510, _) => {
                                    h510
                                }
                            }
                        }))?))
                    } else {
                        Ok(Err::<List<Str>, Adt90>(Adt90::C4(Str::lit("the prompt declares no such argument"))))
                    }
                }
            }
        }
        Adt26::C1(v3) => {
            let a513 = v3.clone();
            let a514 = f273();
            if equal(a513, a514) {
                let a518 = {
                    let m517 = match v0.clone() {
                        Adt25::C0(_, _, h516, _) => {
                            h516
                        }
                    };
                    match m517 {
                        Adt28::C0(h515, _) => {
                            h515
                        }
                    }
                };
                let a519 = Str::lit("id");
                if equal(a518, a519) {
                    Ok(Ok::<List<Str>, Adt90>(f290({
                        let m522 = match v0 {
                            Adt25::C0(_, _, h521, _) => {
                                h521
                            }
                        };
                        match m522 {
                            Adt28::C0(_, h520) => {
                                h520
                            }
                        }
                    }, f292())?))
                } else {
                    Ok(Err::<List<Str>, Adt90>(Adt90::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt90>(Adt90::C2(v3)))
            }
        }
    }
}

pub fn f188(v0: Adt18) -> bool {
    f293(match v0 {
        Adt18::C0(_, _, _, _, h523) => {
            h523
        }
    }, f28())
}

pub fn f189(v0: Bytes) -> R<Option<List<Adt85>>> {
    let a525 = {
        let a524 = v0.clone();
        length_bytes(a524)
    };
    let a526 = 0u64;
    if nat_eq(a525, a526) {
        Ok(Some(List::<Adt85>::nil()))
    } else {
        let m527 = f113(v0)?;
        match m527 {
            Ok(v2) => {
                let m528 = f294(v2);
                match m528 {
                    None => {
                        Ok(None::<List<Adt85>>)
                    }
                    Some(v3) => {
                        Ok(f295(v3, List::<Adt85>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt85>>)
            }
        }
    }
}

pub fn f190() -> Adt90 {
    Adt90::C9(Str::lit("the task table is not readable"))
}

pub fn f191(v0: Str, p533: List<Adt85>, v2: i64) -> R<Option<Adt85>> {
    let m532 = p533;
    {
        let mut m532 = m532;
        loop {
            let step = {
                let v1 = m532;
                let m529 = v1;
                match m529.uncons() {
                    None => {
                        Err::<List<Adt85>, R<Option<Adt85>>>(Ok(None::<Adt85>))
                    }
                    Some((v3, v4)) => {
                        let a530 = f296(v3.clone());
                        let a531 = v0.clone();
                        if equal(a530, a531) {
                            if f297(v3.clone(), v2)? {
                                Err::<List<Adt85>, R<Option<Adt85>>>(Ok(None::<Adt85>))
                            } else {
                                Err::<List<Adt85>, R<Option<Adt85>>>(Ok(Some(v3)))
                            }
                        } else {
                            Ok::<List<Adt85>, R<Option<Adt85>>>(v4)
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

pub fn f192() -> Adt90 {
    Adt90::C8(f298())
}

pub fn f193(v0: Adt104, v1: Adt89, v2: bool) -> Adt33 {
    Adt33::C14(Adt88::C0(match v0.clone() {
        Adt104::C0(h534, _) => {
            h534
        }
    }, v1, if match v0 {
        Adt104::C0(_, h535) => {
            h535
        }
    } {
        true
    } else {
        v2
    }))
}

pub fn f194(v0: Adt85, v1: i64, v2: Str) -> R<Adt104> {
    let m537 = match v0.clone() {
        Adt85::C0(h536, _, _) => {
            h536
        }
    };
    match m537 {
        Adt63::C0(v3) => {
            f299(v0, v3, v1, v2)
        }
        Adt63::C1(_, _) => {
            Ok(f300(v0))
        }
        Adt63::C2(_, _) => {
            Ok(f300(v0))
        }
        Adt63::C3(_, _) => {
            Ok(f300(v0))
        }
        Adt63::C4(_) => {
            Ok(f300(v0))
        }
    }
}

pub fn f195(v0: Adt85, v1: Adt5, v2: i64, v3: Str) -> R<Adt104> {
    let m539 = match v0.clone() {
        Adt85::C0(h538, _, _) => {
            h538
        }
    };
    match m539 {
        Adt63::C0(_) => {
            Ok(f300(v0))
        }
        Adt63::C1(v5, v6) => {
            f301(v0, v5, v6, v1, v2, v3)
        }
        Adt63::C2(_, _) => {
            Ok(f300(v0))
        }
        Adt63::C3(_, _) => {
            Ok(f300(v0))
        }
        Adt63::C4(_) => {
            Ok(f300(v0))
        }
    }
}

pub fn f196(v0: Adt85, v1: i64, v2: Str) -> R<Adt104> {
    Ok(f302(f194(v0, v1, v2.clone())?, v2))
}

pub fn f197(p542: List<Adt93>, p543: List<Adt93>) -> List<Adt93> {
    let m541 = (p542, p543);
    {
        let mut m541 = m541;
        loop {
            let step = {
                let (v0, v1) = m541;
                let m540 = v0;
                match m540.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m541 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Bytes) -> R<List<u64>> {
    f303(v0.clone(), {
        let a544 = v0;
        length_bytes(a544)
    }, List::<u64>::nil())
}

pub fn f199(v0: Bytes, v1: Adt92, v2: Result<(Adt14, u64), Adt94>) -> R<Result<Adt92, Adt94>> {
    let m545 = v2;
    match m545 {
        Ok(v4) => {
            f304(v0, Adt92::C0(match v1.clone() {
                Adt92::C0(h546, _, _, _, _, _, _, _) => {
                    h546
                }
            }, match v1.clone() {
                Adt92::C0(_, h547, _, _, _, _, _, _) => {
                    h547
                }
            }, match v1.clone() {
                Adt92::C0(_, _, h548, _, _, _, _, _) => {
                    h548
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, h549, _, _, _, _) => {
                    h549
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, h550, _, _, _) => {
                    h550
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, _, h551, _, _) => {
                    h551
                }
            }, match v1 {
                Adt92::C0(_, _, _, _, _, _, h552, _) => {
                    h552
                }
            }, {
                let (h553, _) = v4;
                h553
            }), {
                let (_, h554) = v4;
                h554
            })
        }
        Err(v3) => {
            Ok(Err::<Adt92, Adt94>(v3))
        }
    }
}

pub fn f200() -> Bytes {
    Bytes::lit(&[])
}

pub fn f201(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt94>> {
    let m555 = f305(v0.clone(), v1)?;
    match m555 {
        None => {
            Ok(Err::<(Adt14, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m558 = f306(v0.clone(), {
                let a556 = v1;
                let a557 = 8u64;
                nat_add(a556, a557)?
            })?;
            match m558 {
                Ok(v4) => {
                    f307(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt14, u64), Adt94>(v3))
                }
            }
        }
    }
}

pub fn f202(v0: Adt6) -> R<Result<List<u64>, Adt105>> {
    f308(f309(), v0)
}

pub fn f203() -> u64 {
    403u64
}

pub fn f204(v0: Option<Adt101>, v1: Adt81) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f310())), {
        let a559 = f311(v0);
        let a560 = List::cons((Str::lit("error"), f312(v1)), List::<(Str, Adt6)>::nil());
        append_list(a559, a560)
    })
}

pub fn f205() -> i64 {
    -32000i64
}

pub fn f206(v0: Adt97) -> Str {
    let m561 = v0;
    match m561 {
        Adt97::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt97::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f207(p568: List<u64>, p569: List<u64>) -> bool {
    let m567 = (p568, p569);
    {
        let mut m567 = m567;
        loop {
            let step = {
                let (v0, v1) = m567;
                let m562 = v0;
                match m562.uncons() {
                    None => {
                        let m563 = v1;
                        match m563.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m564 = v1;
                        match m564.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a565 = v4;
                                let a566 = v6;
                                if nat_eq(a565, a566) {
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
                Ok(next) => { m567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f209() -> u64 {
    400u64
}

pub fn f210(v0: Adt106) -> Adt81 {
    Adt81::C0(f313(v0.clone()), f314(v0), None::<Adt6>)
}

pub fn f211(v0: Adt14, v1: Adt0, v2: Fn4, v3: u64, v4: List<Adt93>, v5: Result<Adt112, Adt106>) -> R<Adt95> {
    let m570 = v5;
    match m570 {
        Ok(v7) => {
            f317(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f105(f315(v6.clone()), f107(f316(v6.clone()), f210(v6)))
        }
    }
}

pub fn f212(v0: Adt6) -> R<Result<Adt112, Adt106>> {
    let m571 = v0;
    match m571 {
        Adt6::C0 => {
            Ok(f319(None::<Adt101>, Adt107::C0))
        }
        Adt6::C1(_) => {
            Ok(f319(None::<Adt101>, Adt107::C0))
        }
        Adt6::C2(_) => {
            Ok(f319(None::<Adt101>, Adt107::C0))
        }
        Adt6::C3(_) => {
            Ok(f319(None::<Adt101>, Adt107::C0))
        }
        Adt6::C4(_) => {
            Ok(f319(None::<Adt101>, Adt107::C1))
        }
        Adt6::C5(v1) => {
            f318(Adt5::C0(v1))
        }
    }
}

pub fn f213(v0: List<u64>) -> R<Result<Adt6, Adt105>> {
    let m572 = f320(v0.clone())?;
    match m572 {
        Adt125::C0 => {
            f321(f322(), 0u64, v0)
        }
        Adt125::C1(v1) => {
            Ok(Err::<Adt6, Adt105>(Adt105::C0(v1)))
        }
        Adt125::C2(v2) => {
            Ok(Err::<Adt6, Adt105>(Adt105::C1(v2)))
        }
    }
}

pub fn f214() -> u64 {
    405u64
}

pub fn f215() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f216() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f217(p577: List<Adt93>, v1: List<u64>, p578: Adt109) -> R<Adt109> {
    let m576 = (p577, p578);
    {
        let mut m576 = m576;
        loop {
            let step = {
                let (v0, v2) = m576;
                let m573 = v0;
                match m573.uncons() {
                    None => {
                        Err::<(List<Adt93>, Adt109), R<Adt109>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f323(match v3.clone() {
                            Adt93::C0(h574, _) => {
                                h574
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt93>, Adt109), R<Adt109>>((v4, f324(v2, match v3 {
                                Adt93::C0(_, h575) => {
                                    h575
                                }
                            })))
                        } else {
                            Ok::<(List<Adt93>, Adt109), R<Adt109>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f325(v0);
    if f326(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f219(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f323(v0.clone(), f327())? {
        Ok(true)
    } else {
        if f323(v0.clone(), f328())? {
            Ok(true)
        } else {
            if f323(v0.clone(), f329())? {
                Ok(true)
            } else {
                f330(v0, v1)
            }
        }
    }
}

pub fn f220(p583: List<u64>) -> Option<List<u64>> {
    let m582 = p583;
    {
        let mut m582 = m582;
        loop {
            let step = {
                let v0 = m582;
                let m579 = v0;
                match m579.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a580 = v1;
                        let a581 = 58u64;
                        if nat_eq(a580, a581) {
                            Err::<List<u64>, Option<List<u64>>>(f331(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m584 = v0;
    match m584 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f332({
                let (_, h585) = v1;
                h585
            }) {
                Some({
                    let (h586, _) = v1;
                    h586
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f222(p591: List<u64>, p592: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m590 = (p591, p592);
    {
        let mut m590 = m590;
        loop {
            let step = {
                let (v0, v1) = m590;
                let m587 = v0;
                match m587.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a588 = v2;
                        let a589 = 93u64;
                        if nat_eq(a588, a589) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f333(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f223(p597: List<u64>, p598: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m596 = (p597, p598);
    {
        let mut m596 = m596;
        loop {
            let step = {
                let (v0, v1) = m596;
                let m593 = v0;
                match m593.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f333(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a594 = v2;
                        let a595 = 58u64;
                        if nat_eq(a594, a595) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f333(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Str, p603: List<Str>) -> bool {
    let m602 = p603;
    {
        let mut m602 = m602;
        loop {
            let step = {
                let v1 = m602;
                let m599 = v1;
                match m599.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a600 = v0.clone();
                        let a601 = v2;
                        if equal(a600, a601) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: Str, v1: List<u64>) -> Option<Adt98> {
    if f334(v1) {
        Some(Adt98::C0(v0))
    } else {
        None::<Adt98>
    }
}

pub fn f226(v0: Str) -> R<List<u64>> {
    f198({
        let a604 = v0;
        utf8_encode(a604)
    })
}

pub fn f227(v0: Adt36) -> R<Result<List<Adt12>, Adt98>> {
    let m605 = f335(v0.clone());
    match m605 {
        None => {
            f336(v0.clone(), f337(f338(), match v0.clone() {
                Adt36::C0(h606, _, _, _, _, _, _, _) => {
                    h606
                }
            }, Adt6::C5({
                let m609 = match v0.clone() {
                    Adt36::C0(_, _, _, _, h608, _, _, _) => {
                        h608
                    }
                };
                match m609 {
                    Adt5::C0(h607) => {
                        h607
                    }
                }
            }), List::cons(Adt113::C0(Adt6::C5({
                let m612 = match v0 {
                    Adt36::C0(_, _, _, _, h611, _, _, _) => {
                        h611
                    }
                };
                match m612 {
                    Adt5::C0(h610) => {
                        h610
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt113>::nil()), f339())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f228(p617: u64, p618: i64) -> R<i64> {
    let m616 = (p617, p618);
    {
        let mut m616 = m616;
        loop {
            let step = {
                let (v0, v1) = m616;
                let m613 = v0;
                if m613 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m613 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a614 = v1;
                        let a615 = -1i64;
                        int_sub(a614, a615)?
                    }))
                }
            };
            match step {
                Ok(next) => { m616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Adt14, v1: Adt36, v2: Adt22, v3: Adt99) -> R<Result<Adt33, Adt90>> {
    let m619 = v3;
    match m619 {
        Adt99::C0(v4) => {
            f340(v1, v4)
        }
        Adt99::C1(v6, v7, v8) => {
            if f341(v2) {
                f342(v0, v8)
            } else {
                f343(v6, v1, v7)
            }
        }
        Adt99::C2(v5) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C12(v5)))
        }
    }
}

pub fn f230(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt114> {
    f344(v0, f345(), Adt116::C0, List::cons(Adt117::C0(v1, v2, List::<Adt115>::nil()), List::<Adt117>::nil()))
}

pub fn f231(v0: Adt22) -> Adt6 {
    let m621 = match v0 {
        Adt22::C0(_, _, h620, _, _) => {
            h620
        }
    };
    match m621 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h622) => {
                    h622
                }
            })
        }
    }
}

pub fn f232(v0: Str, v1: bool) -> Adt38 {
    Adt38::C0(List::cons(Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f233(v0: List<Adt115>, v1: Str) -> R<Str> {
    let v2: Str = f346(v0)?;
    let a625 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a623 = v2.clone();
        let a624 = Str::lit("");
        if equal(a623, a624) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a626 = Str::lit(" ");
    Ok(join(a625, a626))
}

pub fn f234(v0: Str, v1: Adt5) -> Adt110 {
    f347(v0, match v1 {
        Adt5::C0(h627) => {
            h627
        }
    })
}

pub fn f235(v0: Adt6) -> Option<i64> {
    let m628 = v0;
    match m628 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f348(match v2.clone() {
                Adt7::C0(_, h629) => {
                    h629
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h630, _) => {
                        h630
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

pub fn f236(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m631 = v0;
    match m631 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m632 = f349(Adt6::C5(v1))?;
            match m632 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m633 = f239(v2);
                    match m633 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h634) => {
                                    h634
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f237(v0: Adt38) -> R<Option<List<(Str, Adt6)>>> {
    let m636 = f350(match v0.clone() {
        Adt38::C0(h635, _, _) => {
            h635
        }
    })?;
    match m636 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a639 = f351(Str::lit("structuredContent"), match v0.clone() {
                    Adt38::C0(_, h637, _) => {
                        h637
                    }
                });
                let a640 = f352(Str::lit("isError"), match v0 {
                    Adt38::C0(_, _, h638) => {
                        h638
                    }
                });
                append_list(a639, a640)
            })))
        }
    }
}

pub fn f238() -> Adt87 {
    Adt87::C1(Adt81::C0(-32603i64, Str::lit("Internal error"), None::<Adt6>))
}

pub fn f239(v0: Adt6) -> Option<Adt5> {
    let m641 = v0;
    match m641 {
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

pub fn f240(v0: Adt36, v1: Result<Adt102, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m642 = v1;
    match m642 {
        Ok(v3) => {
            let m643 = v3;
            match m643 {
                Adt102::C0(v4) => {
                    f340(v0, v4)
                }
                Adt102::C1(v5) => {
                    Ok(Ok::<Adt33, Adt90>(Adt33::C12(v5)))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f241(v0: Str) -> Result<Adt102, Adt90> {
    Ok::<Adt102, Adt90>(Adt102::C0(Adt50::C0(Adt38::C0(List::cons(Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)), List::<Adt39>::nil()), None::<Adt6>, None::<bool>), List::<Adt51>::nil())))
}

pub fn f242(v0: List<Str>) -> Str {
    let a644 = v0;
    let a645 = Str::lit("");
    join(a644, a645)
}

pub fn f243(v0: List<(Str, Adt65)>) -> Result<Adt102, Adt90> {
    Ok::<Adt102, Adt90>(Adt102::C1(Adt82::C0(v0)))
}

pub fn f244() -> Adt65 {
    f183(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f245() -> Result<Adt102, Adt90> {
    Err::<Adt102, Adt90>(Adt90::C11)
}

pub fn f246(v0: Str, v1: u64) -> Adt65 {
    Adt65::C1(Adt70::C0(List::cons(f353(v0), List::<Adt71>::nil()), None::<Adt76>, None::<Str>, None::<Adt78>, None::<Adt7>, v1, None::<List<Str>>, None::<Adt5>, None::<List<Adt36>>, None::<Adt79>))
}

pub fn f247(v0: u64) -> R<Str> {
    let a646 = f354(v0, 0i64)?;
    Ok(format_i64(a646))
}

pub fn f248(v0: Adt5) -> u64 {
    let m647 = f234(Str::lit("roots"), v0);
    match m647 {
        Adt110::C0 => {
            0u64
        }
        Adt110::C1(v1) => {
            let m648 = f294(v1);
            match m648 {
                None => {
                    0u64
                }
                Some(v2) => {
                    let a649 = v2;
                    length_list(a649)
                }
            }
        }
    }
}

pub fn f249() -> Adt65 {
    Adt65::C2(Some(f27()))
}

pub fn f250(v0: Adt22, v1: Str, v2: Str) -> Result<Adt102, Adt90> {
    if f251(v0.clone(), v1.clone()) {
        let m651 = f179(Str::lit("confirm"), Adt103::C0, match v0 {
            Adt22::C0(_, _, _, h650, _) => {
                h650
            }
        });
        match m651 {
            Adt111::C0 => {
                f253(f182(Str::lit("confirm"), f356()), v1)
            }
            Adt111::C1 => {
                f245()
            }
            Adt111::C2(v3) => {
                if f355(v3) {
                    f241(v2)
                } else {
                    f253(f182(Str::lit("confirm"), f356()), v1)
                }
            }
        }
    } else {
        f253(f182(Str::lit("confirm"), f356()), v1)
    }
}

pub fn f251(v0: Adt22, v1: Str) -> bool {
    let m653 = match v0 {
        Adt22::C0(_, _, _, _, h652) => {
            h652
        }
    };
    match m653 {
        None => {
            false
        }
        Some(v2) => {
            let a654 = v2;
            let a655 = v1;
            equal(a654, a655)
        }
    }
}

pub fn f252(v0: Adt111, v1: Adt111, v2: Adt111) -> R<Result<Adt102, Adt90>> {
    if if f357(v0.clone()) {
        true
    } else {
        if f357(v1.clone()) {
            true
        } else {
            f357(v2.clone())
        }
    } {
        Ok(f245())
    } else {
        let m656 = f254(v0.clone(), v1.clone(), v2.clone());
        match m656.uncons() {
            None => {
                Ok(f241(f242(List::cons(Str::lit("Name: "), List::cons(f181(f358(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f181(f358(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f247(f248(f358(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f253(f254(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f253(v0: List<(Str, Adt65)>, v1: Str) -> Result<Adt102, Adt90> {
    Ok::<Adt102, Adt90>(Adt102::C1(Adt82::C2(v0, v1)))
}

pub fn f254(v0: Adt111, v1: Adt111, v2: Adt111) -> List<(Str, Adt65)> {
    let a659 = if f359(v0) {
        f182(Str::lit("user_name"), f244())
    } else {
        List::<(Str, Adt65)>::nil()
    };
    let a660 = {
        let a657 = if f359(v1) {
            f182(Str::lit("greeting"), f246(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt65)>::nil()
        };
        let a658 = if f359(v2) {
            f182(Str::lit("client_roots"), f249())
        } else {
            List::<(Str, Adt65)>::nil()
        };
        append_list(a657, a658)
    };
    append_list(a659, a660)
}

pub fn f255(v0: Str) -> R<u64> {
    f100({
        let a661 = v0;
        utf8_encode(a661)
    }, 0u64)
}

pub fn f256(v0: Adt22) -> Str {
    let m663 = match v0 {
        Adt22::C0(_, _, _, _, h662) => {
            h662
        }
    };
    match m663 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f257(v0: Adt22) -> Result<Adt102, Adt90> {
    let m665 = f179(Str::lit("step1"), Adt103::C0, match v0 {
        Adt22::C0(_, _, _, h664, _) => {
            h664
        }
    });
    match m665 {
        Adt111::C0 => {
            f253(f182(Str::lit("step1"), f259()), Str::lit("1"))
        }
        Adt111::C1 => {
            f245()
        }
        Adt111::C2(v1) => {
            f253(f182(Str::lit("step2"), f360()), f242(List::cons(Str::lit("2"), List::cons(f181(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f258(v0: Adt22) -> Result<Adt102, Adt90> {
    let m667 = f179(Str::lit("step2"), Adt103::C0, match v0.clone() {
        Adt22::C0(_, _, _, h666, _) => {
            h666
        }
    });
    match m667 {
        Adt111::C0 => {
            f253(f182(Str::lit("step2"), f360()), f256(v0))
        }
        Adt111::C1 => {
            f245()
        }
        Adt111::C2(v1) => {
            f241(f242(List::cons(Str::lit("Multi-round complete for "), List::cons(f361(f256(v0)), List::cons(Str::lit(" who likes "), List::cons(f181(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f259() -> Adt65 {
    f183(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f260(v0: List<(Str, Adt65)>) -> Result<Adt102, Adt90> {
    let m668 = v0.clone();
    match m668.uncons() {
        None => {
            f241(Str::lit("No supported capabilities declared"))
        }
        Some((_, _)) => {
            f253(v0, Str::lit("capabilities-test"))
        }
    }
}

pub fn f261(v0: Adt18) -> List<(Str, Adt65)> {
    let a671 = if f362(match v0.clone() {
        Adt18::C0(_, _, _, h669, _) => {
            h669
        }
    }) {
        f182(Str::lit("elicit_input"), f183(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt65)>::nil()
    };
    let a672 = if f362(match v0 {
        Adt18::C0(_, _, h670, _, _) => {
            h670
        }
    }) {
        f182(Str::lit("sample_input"), f246(Str::lit("Sample request"), 50u64))
    } else {
        List::<(Str, Adt65)>::nil()
    };
    append_list(a671, a672)
}

pub fn f262(v0: List<(Str, Adt6)>) -> List<Str> {
    f363(f364(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f263(v0: Fn3, v1: Adt36, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    f365(v1, v0, v2)
}

pub fn f264(v0: Option<Adt20>) -> Str {
    let m673 = v0;
    match m673 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m674 = v1;
            match m674 {
                Adt20::C0(v2) => {
                    v2
                }
                Adt20::C1(v3) => {
                    let a675 = v3;
                    format_i64(a675)
                }
            }
        }
    }
}

pub fn f265(v0: u64, v1: u64) -> R<Adt51> {
    Ok(Adt51::C0(f366(v0)?, Some(f366(v1)?), Some({
        let a678 = List::cons(Str::lit("Completed step"), List::cons({
            let a676 = f354(v0, 0i64)?;
            format_i64(a676)
        }, List::cons(Str::lit("of"), List::cons({
            let a677 = f354(v1, 0i64)?;
            format_i64(a677)
        }, List::<Str>::nil()))));
        let a679 = Str::lit(" ");
        join(a678, a679)
    })))
}

pub fn f266() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, Some(f27()), None::<Adt5>, None::<Adt5>)
}

pub fn f267() -> Adt50 {
    f154(List::cons(f155(Str::lit("ok")), List::<Adt39>::nil()))
}

pub fn f268(v0: List<(Adt45, List<Adt47>)>) -> List<Adt45> {
    f367(f368(v0, List::<Adt45>::nil()), List::<Adt45>::nil())
}

pub fn f269() -> List<(Adt45, List<Adt47>)> {
    List::cons(f369(f370(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f286(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f369(f370(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f371(Str::lit("test://static-binary"), Str::lit("image/png"), f287())), List::cons(f369(f370(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f286(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f369(f370(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f286(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt45, List<Adt47>)>::nil()))))
}

pub fn f270(v0: Adt10, v1: Str, v2: Option<List<Adt47>>) -> Result<Adt33, Adt90> {
    let m680 = v2;
    match m680 {
        None => {
            Err::<Adt33, Adt90>(Adt90::C2(v1))
        }
        Some(v3) => {
            let m681 = v3.clone();
            match m681.uncons() {
                None => {
                    Err::<Adt33, Adt90>(Adt90::C9(Str::lit("a listed resource has no contents")))
                }
                Some((_, _)) => {
                    Ok::<Adt33, Adt90>(Adt33::C5(Adt53::C0(v0, v3)))
                }
            }
        }
    }
}

pub fn f271(v0: Str, p690: List<(Adt45, List<Adt47>)>) -> Option<List<Adt47>> {
    let m689 = p690;
    {
        let mut m689 = m689;
        loop {
            let step = {
                let v1 = m689;
                let m682 = v1;
                match m682.uncons() {
                    None => {
                        Err::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(None::<List<Adt47>>)
                    }
                    Some((v2, v3)) => {
                        let a686 = {
                            let m685 = {
                                let (h684, _) = v2;
                                h684
                            };
                            match m685 {
                                Adt45::C0(_, _, _, h683, _, _, _, _, _) => {
                                    h683
                                }
                            }
                        };
                        let a687 = v0.clone();
                        if equal(a686, a687) {
                            Err::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(Some({
                                let (_, h688) = v2;
                                h688
                            }))
                        } else {
                            Ok::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(v3)
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

pub fn f272(v0: Str) -> R<Option<List<Adt47>>> {
    let m691 = f372(v0.clone())?;
    match m691 {
        None => {
            Ok(None::<List<Adt47>>)
        }
        Some(v1) => {
            let m692 = f373(v1)?;
            match m692 {
                None => {
                    Ok(None::<List<Adt47>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f286(v0, Str::lit("application/json"), v2), List::<Adt47>::nil())))
                }
            }
        }
    }
}

pub fn f273() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f274(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt58>>) -> Adt57 {
    Adt57::C0(v0, Some(v1), None::<List<Adt2>>, Some(v2), v3, None::<Adt5>)
}

pub fn f275(v0: Str, v1: Str) -> Adt58 {
    Adt58::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f276(v0: Adt103, v1: Adt5) -> bool {
    let m693 = v0;
    match m693 {
        Adt103::C0 => {
            f374(v1)
        }
        Adt103::C1 => {
            f375(v1)
        }
        Adt103::C2 => {
            f376(v1)
        }
    }
}

pub fn f277(v0: Str, v1: Adt5, v2: Str) -> Str {
    let m694 = f234(v0, v1);
    match m694 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            let m695 = f133(v3);
            match m695 {
                None => {
                    v2
                }
                Some(v4) => {
                    v4
                }
            }
        }
    }
}

pub fn f278(v0: Str, v1: Str) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v0.clone(), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v0), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f279(v0: Str, p701: List<Adt57>) -> Option<Adt57> {
    let m700 = p701;
    {
        let mut m700 = m700;
        loop {
            let step = {
                let v1 = m700;
                let m696 = v1;
                match m696.uncons() {
                    None => {
                        Err::<List<Adt57>, Option<Adt57>>(None::<Adt57>)
                    }
                    Some((v2, v3)) => {
                        let a698 = match v2.clone() {
                            Adt57::C0(h697, _, _, _, _, _) => {
                                h697
                            }
                        };
                        let a699 = v0.clone();
                        if equal(a698, a699) {
                            Err::<List<Adt57>, Option<Adt57>>(Some(v2))
                        } else {
                            Ok::<List<Adt57>, Option<Adt57>>(v3)
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

pub fn f280(p707: List<Adt58>, v1: List<(Str, Str)>) -> Option<Str> {
    let m706 = p707;
    {
        let mut m706 = m706;
        loop {
            let step = {
                let v0 = m706;
                let m702 = v0;
                match m702.uncons() {
                    None => {
                        Err::<List<Adt58>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f377(v2.clone()) {
                            let m704 = f378(match v2.clone() {
                                Adt58::C0(h703, _, _, _) => {
                                    h703
                                }
                            }, v1.clone());
                            match m704 {
                                None => {
                                    Err::<List<Adt58>, Option<Str>>(Some(match v2 {
                                        Adt58::C0(h705, _, _, _) => {
                                            h705
                                        }
                                    }))
                                }
                                Some(_) => {
                                    Ok::<List<Adt58>, Option<Str>>(v3)
                                }
                            }
                        } else {
                            Ok::<List<Adt58>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: Adt57) -> List<Adt58> {
    let m709 = match v0 {
        Adt57::C0(_, _, _, _, h708, _) => {
            h708
        }
    };
    match m709 {
        None => {
            List::<Adt58>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f282(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m710 = v0;
    match m710 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f283(v0: List<Adt60>) -> Adt59 {
    Adt59::C0(None::<Str>, v0)
}

pub fn f284(v0: Str) -> Adt60 {
    Adt60::C0(Adt42::C0, Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)))
}

pub fn f285(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m711 = f378(v0, v1);
    match m711 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f286(v0: Str, v1: Str, v2: Str) -> Adt47 {
    Adt47::C0(Adt48::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f287() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f288(v0: List<Str>) -> R<Adt61> {
    let v1: u64 = f379(v0.clone(), 0u64)?;
    Ok(Adt61::C0(Adt62::C0(f380(v0), Some(v1), Some({
        let a712 = f381();
        let a713 = v1;
        nat_lt(a712, a713)
    }))))
}

pub fn f289(p719: List<Adt58>, v1: Str) -> bool {
    let m718 = p719;
    {
        let mut m718 = m718;
        loop {
            let step = {
                let v0 = m718;
                let m714 = v0;
                match m714.uncons() {
                    None => {
                        Err::<List<Adt58>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a716 = match v2 {
                            Adt58::C0(h715, _, _, _) => {
                                h715
                            }
                        };
                        let a717 = v1.clone();
                        if equal(a716, a717) {
                            Err::<List<Adt58>, bool>(true)
                        } else {
                            Ok::<List<Adt58>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f363(f382(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f291(v0: Str, v1: Str) -> List<Str> {
    let a720 = v0;
    let a721 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a720, a721) {
        let a722 = v1;
        let a723 = Str::lit("resourceUri");
        equal(a722, a723)
    } else {
        false
    } {
        f383()
    } else {
        List::<Str>::nil()
    }
}

pub fn f292() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f293(v0: Option<Adt5>, v1: Str) -> bool {
    let m724 = v0;
    match m724 {
        None => {
            false
        }
        Some(v2) => {
            f384(v1, v2)
        }
    }
}

pub fn f294(v0: Adt6) -> Option<List<Adt6>> {
    let m725 = v0;
    match m725 {
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

pub fn f295(p729: List<Adt6>, p730: List<Adt85>) -> Option<List<Adt85>> {
    let m728 = (p729, p730);
    {
        let mut m728 = m728;
        loop {
            let step = {
                let (v0, v1) = m728;
                let m726 = v0;
                match m726.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(Some(f385(v1, List::<Adt85>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m727 = f386(v2);
                        match m727 {
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
                Ok(next) => { m728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: Adt85) -> Str {
    let m733 = f387(match v0 {
        Adt85::C0(h732, _, _) => {
            h732
        }
    });
    match m733 {
        Adt64::C0(h731, _, _, _, _, _) => {
            h731
        }
    }
}

pub fn f297(v0: Adt85, v1: i64) -> R<bool> {
    let m737 = {
        let m736 = f387(match v0.clone() {
            Adt85::C0(h735, _, _) => {
                h735
            }
        });
        match m736 {
            Adt64::C0(_, _, _, _, h734, _) => {
                h734
            }
        }
    };
    match m737 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a744 = {
                let m743 = {
                    let a741 = {
                        let a739 = v1;
                        let a740 = match v0 {
                            Adt85::C0(_, h738, _) => {
                                h738
                            }
                        };
                        int_sub(a739, a740)?
                    };
                    let a742 = v2;
                    compare(a741, a742)
                };
                match m743 {
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
            Ok(bool_not(a744))
        }
    }
}

pub fn f298() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f28(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f299(v0: Adt85, v1: Adt64, v2: i64, v3: Str) -> R<Adt104> {
    let m748 = {
        let m747 = match v0.clone() {
            Adt85::C0(_, _, h746) => {
                h746
            }
        };
        match m747 {
            Adt86::C0(_, h745, _, _) => {
                h745
            }
        }
    };
    match m748.uncons() {
        None => {
            if f388({
                let m751 = match v0.clone() {
                    Adt85::C0(_, _, h750) => {
                        h750
                    }
                };
                match m751 {
                    Adt86::C0(_, _, h749, _) => {
                        h749
                    }
                }
            }, match v0.clone() {
                Adt85::C0(_, h752, _) => {
                    h752
                }
            }, v2)? {
                Ok(f389(v0.clone(), f390({
                    let m755 = match v0.clone() {
                        Adt85::C0(_, _, h754) => {
                            h754
                        }
                    };
                    match m755 {
                        Adt86::C0(_, _, _, h753) => {
                            h753
                        }
                    }
                }, f391(v1, v3)), match v0 {
                    Adt85::C0(_, _, h756) => {
                        h756
                    }
                }))
            } else {
                Ok(f300(v0))
            }
        }
        Some((_, _)) => {
            if f388({
                let m759 = match v0.clone() {
                    Adt85::C0(_, _, h758) => {
                        h758
                    }
                };
                match m759 {
                    Adt86::C0(h757, _, _, _) => {
                        h757
                    }
                }
            }, match v0.clone() {
                Adt85::C0(_, h760, _) => {
                    h760
                }
            }, v2)? {
                Ok(f389(v0.clone(), Adt63::C1(f391(v1, v3), {
                    let m763 = match v0.clone() {
                        Adt85::C0(_, _, h762) => {
                            h762
                        }
                    };
                    match m763 {
                        Adt86::C0(_, h761, _, _) => {
                            h761
                        }
                    }
                }), f392(match v0 {
                    Adt85::C0(_, _, h764) => {
                        h764
                    }
                })))
            } else {
                Ok(f300(v0))
            }
        }
    }
}

pub fn f300(v0: Adt85) -> Adt104 {
    Adt104::C0(v0, false)
}

pub fn f301(v0: Adt85, v1: Adt64, v2: List<(Str, Adt65)>, v3: Adt5, v4: i64, v5: Str) -> R<Adt104> {
    let m765 = f393(v2.clone(), v3);
    match m765.uncons() {
        None => {
            f394(v0, v1, v4, v5)
        }
        Some((v6, v7)) => {
            let a768 = {
                let a766 = List::cons(v6.clone(), v7.clone());
                length_list(a766)
            };
            let a769 = {
                let a767 = v2;
                length_list(a767)
            };
            if nat_eq(a768, a769) {
                Ok(f300(v0))
            } else {
                Ok(f389(v0.clone(), Adt63::C1(f391(v1, v5), List::cons(v6, v7)), match v0 {
                    Adt85::C0(_, _, h770) => {
                        h770
                    }
                }))
            }
        }
    }
}

pub fn f302(v0: Adt104, v1: Str) -> Adt104 {
    let m774 = {
        let m773 = match v0.clone() {
            Adt104::C0(h772, _) => {
                h772
            }
        };
        match m773 {
            Adt85::C0(h771, _, _) => {
                h771
            }
        }
    };
    match m774 {
        Adt63::C0(v2) => {
            f389(match v0.clone() {
                Adt104::C0(h775, _) => {
                    h775
                }
            }, Adt63::C4(f391(v2, v1)), {
                let m778 = match v0 {
                    Adt104::C0(h777, _) => {
                        h777
                    }
                };
                match m778 {
                    Adt85::C0(_, _, h776) => {
                        h776
                    }
                }
            })
        }
        Adt63::C1(v3, _) => {
            f389(match v0.clone() {
                Adt104::C0(h779, _) => {
                    h779
                }
            }, Adt63::C4(f391(v3, v1)), {
                let m782 = match v0 {
                    Adt104::C0(h781, _) => {
                        h781
                    }
                };
                match m782 {
                    Adt85::C0(_, _, h780) => {
                        h780
                    }
                }
            })
        }
        Adt63::C2(_, _) => {
            v0
        }
        Adt63::C3(_, _) => {
            v0
        }
        Adt63::C4(_) => {
            v0
        }
    }
}

pub fn f303(v0: Bytes, p785: u64, p786: List<u64>) -> R<List<u64>> {
    let m784 = (p785, p786);
    {
        let mut m784 = m784;
        loop {
            let step = {
                let (v1, v2) = m784;
                let m783 = v1;
                if m783 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m783 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f100(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: Bytes, v1: Adt92, v2: u64) -> R<Result<Adt92, Adt94>> {
    let m787 = f395(v0.clone(), v2)?;
    match m787 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(v2)))
        }
        Some(v3) => {
            f396(v0, Adt92::C0(match v1.clone() {
                Adt92::C0(h788, _, _, _, _, _, _, _) => {
                    h788
                }
            }, match v1.clone() {
                Adt92::C0(_, h789, _, _, _, _, _, _) => {
                    h789
                }
            }, match v1.clone() {
                Adt92::C0(_, _, h790, _, _, _, _, _) => {
                    h790
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, h791, _, _, _, _) => {
                    h791
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, h792, _, _, _) => {
                    h792
                }
            }, match v1.clone() {
                Adt92::C0(_, _, _, _, _, h793, _, _) => {
                    h793
                }
            }, match v3.clone() {
                Adt96::C0(h794, _) => {
                    h794
                }
            }, match v1 {
                Adt92::C0(_, _, _, _, _, _, _, h795) => {
                    h795
                }
            }), match v3 {
                Adt96::C0(_, h796) => {
                    h796
                }
            })
        }
    }
}

pub fn f305(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a800 = {
        let a797 = v1;
        let a798 = 8u64;
        nat_add(a797, a798)?
    };
    let a801 = {
        let a799 = v0.clone();
        length_bytes(a799)
    };
    if nat_le(a800, a801) {
        Ok(Some(f397(f397(f397(f397(f397(f397(f397(f397(0i64, f100(v0.clone(), v1)?)?, f100(v0.clone(), {
            let a802 = v1;
            let a803 = 1u64;
            nat_add(a802, a803)?
        })?)?, f100(v0.clone(), {
            let a804 = v1;
            let a805 = 2u64;
            nat_add(a804, a805)?
        })?)?, f100(v0.clone(), {
            let a806 = v1;
            let a807 = 3u64;
            nat_add(a806, a807)?
        })?)?, f100(v0.clone(), {
            let a808 = v1;
            let a809 = 4u64;
            nat_add(a808, a809)?
        })?)?, f100(v0.clone(), {
            let a810 = v1;
            let a811 = 5u64;
            nat_add(a810, a811)?
        })?)?, f100(v0.clone(), {
            let a812 = v1;
            let a813 = 6u64;
            nat_add(a812, a813)?
        })?)?, f100(v0, {
            let a814 = v1;
            let a815 = 7u64;
            nat_add(a814, a815)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f306(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt94>> {
    let m816 = f16(v0, v1)?;
    match m816 {
        None => {
            Ok(Err::<(Str, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m818 = f398(f198(match v2.clone() {
                Adt96::C0(h817, _) => {
                    h817
                }
            })?);
            match m818 {
                None => {
                    Ok(Err::<(Str, u64), Adt94>(Adt94::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt94>((v3, match v2 {
                        Adt96::C0(_, h819) => {
                            h819
                        }
                    })))
                }
            }
        }
    }
}

pub fn f307(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt14, u64), Adt94>> {
    let m821 = f306(v0.clone(), {
        let (_, h820) = v2;
        h820
    })?;
    match m821 {
        Ok(v4) => {
            f399(v0, v1, {
                let (h822, _) = v2;
                h822
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt14, u64), Adt94>(v3))
        }
    }
}

pub fn f308(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt105>> {
    let m823 = f400(v0, List::cons(Adt119::C1(0u64, v1), List::<Adt119>::nil()), List::<u64>::nil())?;
    match m823 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt105>(f401(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt105>(v3))
        }
    }
}

pub fn f309() -> u64 {
    67108864u64
}

pub fn f310() -> Str {
    Str::lit("2.0")
}

pub fn f311(v0: Option<Adt101>) -> List<(Str, Adt6)> {
    let m824 = v0;
    match m824 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f402(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f312(v0: Adt81) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt81::C0(h825, _, _) => {
            h825
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt81::C0(_, h826, _) => {
            h826
        }
    })), f403(match v0 {
        Adt81::C0(_, _, h827) => {
            h827
        }
    }))))
}

pub fn f313(v0: Adt106) -> i64 {
    let m828 = v0;
    match m828 {
        Adt106::C0 => {
            f404()
        }
        Adt106::C1(_, _) => {
            f405()
        }
    }
}

pub fn f314(v0: Adt106) -> Str {
    let m829 = v0;
    match m829 {
        Adt106::C0 => {
            Str::lit("Parse error")
        }
        Adt106::C1(_, v2) => {
            f406(v2)
        }
    }
}

pub fn f315(v0: Adt106) -> u64 {
    let m830 = v0;
    match m830 {
        Adt106::C0 => {
            f209()
        }
        Adt106::C1(_, _) => {
            f209()
        }
    }
}

pub fn f316(v0: Adt106) -> Option<Adt101> {
    let m831 = v0;
    match m831 {
        Adt106::C0 => {
            None::<Adt101>
        }
        Adt106::C1(v1, _) => {
            v1
        }
    }
}

pub fn f317(v0: Adt14, v1: Adt0, v2: Fn4, v3: u64, v4: List<Adt93>, v5: Adt112) -> R<Adt95> {
    f407(v0, f408(v4.clone())?, f409(v1.clone(), v2, v3, Some(f410(v4, match v1 {
        Adt0::C0(_, _, _, _, h832, _, _, _) => {
            h832
        }
    }, f411(v5.clone()), match v5.clone() {
        Adt112::C0(_, h833, _) => {
            h833
        }
    })?), f412(), v5)?)
}

pub fn f318(v0: Adt5) -> R<Result<Adt112, Adt106>> {
    let m834 = f413(v0.clone())?;
    match m834 {
        Adt144::C0 => {
            let m835 = f234(Str::lit("method"), v0.clone());
            match m835 {
                Adt110::C0 => {
                    Ok(f319(None::<Adt101>, f414(v0)))
                }
                Adt110::C1(_) => {
                    Ok(f319(None::<Adt101>, if f415(v0) {
                        Adt107::C8
                    } else {
                        Adt107::C7
                    }))
                }
            }
        }
        Adt144::C1(v2) => {
            Ok(f416(v2, v0))
        }
        Adt144::C2 => {
            Ok(f319(None::<Adt101>, Adt107::C5))
        }
    }
}

pub fn f319(v0: Option<Adt101>, v1: Adt107) -> Result<Adt112, Adt106> {
    Err::<Adt112, Adt106>(Adt106::C1(v0, v1))
}

pub fn f320(v0: List<u64>) -> R<Adt125> {
    Ok(f417({
        let a836 = v0.clone();
        length_list(a836)
    }, f418(Adt126::C0, 0u64, 0u64, v0)?))
}

pub fn f321(p843: Adt127, p844: u64, p845: List<u64>) -> R<Result<Adt6, Adt105>> {
    let m842 = (p843, (p844, p845));
    {
        let mut m842 = m842;
        loop {
            let step = {
                let (v0, (v1, v2)) = m842;
                let m837 = v2;
                match m837.uncons() {
                    None => {
                        let m838 = f419(v0, v1)?;
                        match m838 {
                            Ok(v3) => {
                                Err::<(Adt127, (u64, List<u64>)), R<Result<Adt6, Adt105>>>(Ok(Ok::<Adt6, Adt105>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt127, (u64, List<u64>)), R<Result<Adt6, Adt105>>>(Ok(Err::<Adt6, Adt105>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m839 = f420(v0, v1, v5)?;
                        match m839 {
                            Adt145::C0(v7) => {
                                Err::<(Adt127, (u64, List<u64>)), R<Result<Adt6, Adt105>>>(Ok(Err::<Adt6, Adt105>(v7)))
                            }
                            Adt145::C1(v8) => {
                                Ok::<(Adt127, (u64, List<u64>)), R<Result<Adt6, Adt105>>>((v8, ({
                                    let a840 = v1;
                                    let a841 = 1u64;
                                    nat_add(a840, a841)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322() -> Adt127 {
    Adt127::C0(Adt128::C0, List::<Adt132>::nil(), 0u64)
}

pub fn f323(p852: List<u64>, p853: List<u64>) -> R<bool> {
    let m851 = (p852, p853);
    {
        let mut m851 = m851;
        loop {
            let step = {
                let (v0, v1) = m851;
                let m846 = v0;
                match m846.uncons() {
                    None => {
                        let m847 = v1;
                        match m847.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m848 = v1;
                        match m848.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a849 = f421(v4)?;
                                let a850 = f421(v6)?;
                                if nat_eq(a849, a850) {
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
                Ok(next) => { m851 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: Adt109, v1: List<u64>) -> Adt109 {
    let m854 = v0;
    match m854 {
        Adt109::C0 => {
            Adt109::C1(v1)
        }
        Adt109::C1(_) => {
            Adt109::C2
        }
        Adt109::C2 => {
            Adt109::C2
        }
    }
}

pub fn f325(v0: List<u64>) -> List<u64> {
    f333(f422(f333(f422(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f326(p857: List<u64>) -> bool {
    let m856 = p857;
    {
        let mut m856 = m856;
        loop {
            let step = {
                let v0 = m856;
                let m855 = v0;
                match m855.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f423(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m856 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f328() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f329() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f330(v0: List<u64>, p860: List<List<u64>>) -> R<bool> {
    let m859 = p860;
    {
        let mut m859 = m859;
        loop {
            let step = {
                let v1 = m859;
                let m858 = v1;
                match m858.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f323(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: List<u64>) -> Option<List<u64>> {
    let m861 = v0;
    match m861.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m862 = v2;
            match m862.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a863 = v1;
                    let a864 = 47u64;
                    if if nat_eq(a863, a864) {
                        let a865 = v3;
                        let a866 = 47u64;
                        nat_eq(a865, a866)
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

pub fn f332(v0: List<u64>) -> bool {
    let m867 = v0;
    match m867.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a868 = v1;
            let a869 = 58u64;
            if nat_eq(a868, a869) {
                let m870 = v2.clone();
                match m870.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f424(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f333(p873: List<u64>, p874: List<u64>) -> List<u64> {
    let m872 = (p873, p874);
    {
        let mut m872 = m872;
        loop {
            let step = {
                let (v0, v1) = m872;
                let m871 = v0;
                match m871.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m872 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f334(v0: List<u64>) -> bool {
    let m875 = v0.clone();
    match m875.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a879 = if f425(v0.clone(), false) {
                let a877 = {
                    let a876 = v0;
                    length_list(a876)
                };
                let a878 = f426();
                nat_le(a877, a878)
            } else {
                false
            };
            bool_not(a879)
        }
    }
}

pub fn f335(v0: Adt36) -> Option<Adt98> {
    let m884 = f427(match v0.clone() {
        Adt36::C0(h880, _, _, _, _, _, _, _) => {
            h880
        }
    }, {
        let m883 = match v0.clone() {
            Adt36::C0(_, _, _, _, h882, _, _, _) => {
                h882
            }
        };
        match m883 {
            Adt5::C0(h881) => {
                h881
            }
        }
    });
    match m884 {
        None => {
            f428(match v0.clone() {
                Adt36::C0(h885, _, _, _, _, _, _, _) => {
                    h885
                }
            }, {
                let m888 = match v0 {
                    Adt36::C0(_, _, _, _, h887, _, _, _) => {
                        h887
                    }
                };
                match m888 {
                    Adt5::C0(h886) => {
                        h886
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f336(v0: Adt36, v1: Result<Adt133, Adt98>) -> R<Result<List<Adt12>, Adt98>> {
    let m889 = v1;
    match m889 {
        Ok(v3) => {
            let m890 = f429(v0)?;
            match m890 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt98>(f122(match v3 {
                        Adt133::C0(h891, _) => {
                            h891
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

pub fn f337(p898: u64, v1: Str, v2: Adt6, p899: List<Adt113>, p900: Adt133) -> R<Result<Adt133, Adt98>> {
    let m897 = (p898, (p899, p900));
    {
        let mut m897 = m897;
        loop {
            let step = {
                let (v0, (v3, v4)) = m897;
                let m892 = v0;
                if m892 == 0 {
                    Err::<(u64, (List<Adt113>, Adt133)), R<Result<Adt133, Adt98>>>(Ok(Err::<Adt133, Adt98>(Adt98::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m892 - 1;
                    let m893 = v3;
                    match m893.uncons() {
                        None => {
                            Err::<(u64, (List<Adt113>, Adt133)), R<Result<Adt133, Adt98>>>(Ok(Ok::<Adt133, Adt98>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m894 = f430(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m894 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt113>, Adt133)), R<Result<Adt133, Adt98>>>((v5, ({
                                        let (h895, _) = v9;
                                        h895
                                    }, {
                                        let (_, h896) = v9;
                                        h896
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt113>, Adt133)), R<Result<Adt133, Adt98>>>(Ok(Err::<Adt133, Adt98>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m897 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338() -> u64 {
    67108864u64
}

pub fn f339() -> Adt133 {
    Adt133::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f340(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f431({
        let m903 = match v1.clone() {
            Adt50::C0(h902, _) => {
                h902
            }
        };
        match m903 {
            Adt38::C0(h901, _, _) => {
                h901
            }
        }
    })? {
        f432(v0, v1)
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f341(v0: Adt22) -> bool {
    f188({
        let m906 = match v0 {
            Adt22::C0(h905, _, _, _, _) => {
                h905
            }
        };
        match m906 {
            Adt17::C0(_, h904, _, _, _, _) => {
                h904
            }
        }
    })
}

pub fn f342(v0: Adt14, v1: Adt86) -> R<Result<Adt33, Adt90>> {
    let a908 = match v0.clone() {
        Adt14::C0(_, _, h907, _) => {
            h907
        }
    };
    let a909 = Str::lit("");
    if equal(a908, a909) {
        Ok(Err::<Adt33, Adt90>(f433()))
    } else {
        let m911 = f189(match v0.clone() {
            Adt14::C0(_, _, _, h910) => {
                h910
            }
        })?;
        match m911 {
            None => {
                Ok(Err::<Adt33, Adt90>(f190()))
            }
            Some(v2) => {
                let m914 = f191(match v0.clone() {
                    Adt14::C0(_, _, h912, _) => {
                        h912
                    }
                }, v2, match v0.clone() {
                    Adt14::C0(h913, _, _, _) => {
                        h913
                    }
                })?;
                match m914 {
                    None => {
                        Ok(Ok::<Adt33, Adt90>(Adt33::C13(Adt83::C0(Adt84::C0, f387({
                            let m916 = f435(v0.clone(), v1.clone());
                            match m916 {
                                Adt85::C0(h915, _, _) => {
                                    h915
                                }
                            }
                        }), f435(v0, v1)))))
                    }
                    Some(_) => {
                        Ok(Err::<Adt33, Adt90>(f434()))
                    }
                }
            }
        }
    }
}

pub fn f343(v0: Adt100, v1: Adt36, v2: Adt50) -> R<Result<Adt33, Adt90>> {
    let m917 = v0;
    match m917 {
        Adt100::C0 => {
            f340(v1, v2)
        }
        Adt100::C1 => {
            Ok(Err::<Adt33, Adt90>(f192()))
        }
    }
}

pub fn f344(v0: Adt6, p936: u64, p937: Adt116, p938: List<Adt117>) -> R<Adt114> {
    let m935 = (p936, (p937, p938));
    {
        let mut m935 = m935;
        loop {
            let step = {
                let (v1, (v2, v3)) = m935;
                let m918 = v1;
                if m918 == 0 {
                    Err::<(u64, (Adt116, List<Adt117>)), R<Adt114>>(Ok(Adt114::C4))
                } else {
                    let v4 = m918 - 1;
                    let m919 = v3;
                    match m919.uncons() {
                        None => {
                            let m920 = v2;
                            match m920 {
                                Adt116::C0 => {
                                    Err::<(u64, (Adt116, List<Adt117>)), R<Adt114>>(Ok(Adt114::C0))
                                }
                                Adt116::C1(v5, v6) => {
                                    Err::<(u64, (Adt116, List<Adt117>)), R<Adt114>>(Ok(Adt114::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m921 = v2.clone();
                            match m921 {
                                Adt116::C0 => {
                                    let m922 = v7.clone();
                                    match m922 {
                                        Adt117::C0(v9, v10, v11) => {
                                            let m923 = f436(v0.clone(), v9, v10, v11.clone())?;
                                            match m923 {
                                                Adt146::C0(v12) => {
                                                    Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (Adt116::C1(v11, v12), v8)))
                                                }
                                                Adt146::C1(v13) => {
                                                    Err::<(u64, (Adt116, List<Adt117>)), R<Adt114>>(Ok(Adt114::C2(v13)))
                                                }
                                                Adt146::C2(v14) => {
                                                    Err::<(u64, (Adt116, List<Adt117>)), R<Adt114>>(Ok(Adt114::C3(v14)))
                                                }
                                                Adt146::C3(v15) => {
                                                    let v16: List<Adt117> = {
                                                        let a924 = v15;
                                                        let a925 = v8;
                                                        append_list(a924, a925)
                                                    };
                                                    Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt117::C1(_, _, _) => {
                                            Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (v2, v8)))
                                        }
                                        Adt117::C2(_, _, _, _) => {
                                            let v24: Adt134 = f437(v7, v8)?;
                                            Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (match v24.clone() {
                                                Adt134::C0(h926, _) => {
                                                    h926
                                                }
                                            }, match v24 {
                                                Adt134::C0(_, h927) => {
                                                    h927
                                                }
                                            })))
                                        }
                                        Adt117::C3(v25) => {
                                            Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (Adt116::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt117::C4(_, _, _, _) => {
                                            let v39: Adt134 = f437(v7, v8)?;
                                            Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (match v39.clone() {
                                                Adt134::C0(h931, _) => {
                                                    h931
                                                }
                                            }, match v39 {
                                                Adt134::C0(_, h932) => {
                                                    h932
                                                }
                                            })))
                                        }
                                        Adt117::C5(v26, v27, v28, v29) => {
                                            let m928 = f438(v26, v27, v28, v29.clone());
                                            match m928 {
                                                Adt146::C0(v30) => {
                                                    Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (Adt116::C1(v29, v30), v8)))
                                                }
                                                Adt146::C1(v31) => {
                                                    Err::<(u64, (Adt116, List<Adt117>)), R<Adt114>>(Ok(Adt114::C2(v31)))
                                                }
                                                Adt146::C2(v32) => {
                                                    Err::<(u64, (Adt116, List<Adt117>)), R<Adt114>>(Ok(Adt114::C3(v32)))
                                                }
                                                Adt146::C3(v33) => {
                                                    let v34: List<Adt117> = {
                                                        let a929 = v33;
                                                        let a930 = v8;
                                                        append_list(a929, a930)
                                                    };
                                                    Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt116::C1(v40, v41) => {
                                    let v42: Adt134 = f439(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt116, List<Adt117>)), R<Adt114>>((v4, (match v42.clone() {
                                        Adt134::C0(h933, _) => {
                                            h933
                                        }
                                    }, match v42 {
                                        Adt134::C0(_, h934) => {
                                            h934
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m935 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345() -> u64 {
    67108864u64
}

pub fn f346(v0: List<Adt115>) -> R<Str> {
    let a939 = List::cons(Str::lit(""), f440(v0, List::<Str>::nil())?);
    let a940 = Str::lit("/");
    Ok(join(a939, a940))
}

pub fn f347(v0: Str, p946: List<(Str, Adt6)>) -> Adt110 {
    let m945 = p946;
    {
        let mut m945 = m945;
        loop {
            let step = {
                let v1 = m945;
                let m941 = v1;
                match m941.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt110>(Adt110::C0)
                    }
                    Some((v2, v3)) => {
                        let m943 = f441({
                            let (h942, _) = v2;
                            h942
                        }, v0.clone());
                        match m943 {
                            Adt147::C0 => {
                                Err::<List<(Str, Adt6)>, Adt110>(Adt110::C1({
                                    let (_, h944) = v2;
                                    h944
                                }))
                            }
                            Adt147::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt110>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m945 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f348(v0: i64) -> bool {
    let a953 = {
        let m949 = {
            let a947 = v0;
            let a948 = 0i64;
            compare(a947, a948)
        };
        if match m949 {
            Ordering::Less => {
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
            let m952 = {
                let a950 = 0i64;
                let a951 = v0;
                compare(a950, a951)
            };
            match m952 {
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
    bool_not(a953)
}

pub fn f349(v0: Adt6) -> R<Option<Adt6>> {
    f442(f443(), Adt135::C0(v0), List::<Adt136>::nil())
}

pub fn f350(v0: List<Adt39>) -> R<Option<List<Adt6>>> {
    let m954 = f444(v0, List::<Adt6>::nil())?;
    match m954 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f445(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f351(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m955 = v1;
    match m955 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f352(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m956 = v1;
    match m956 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f353(v0: Str) -> Adt71 {
    Adt71::C0(Adt42::C0, Adt72::C0(Adt73::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))), None::<Adt5>)
}

pub fn f354(p959: u64, p960: i64) -> R<i64> {
    let m958 = (p959, p960);
    {
        let mut m958 = m958;
        loop {
            let step = {
                let (v0, v1) = m958;
                let m957 = v0;
                if m957 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m957 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f446(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m958 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: Adt5) -> bool {
    let m961 = f234(Str::lit("content"), v0);
    match m961 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v1) => {
            let m962 = f239(v1);
            match m962 {
                None => {
                    false
                }
                Some(v2) => {
                    let m963 = f234(Str::lit("ok"), v2);
                    match m963 {
                        Adt110::C0 => {
                            false
                        }
                        Adt110::C1(v3) => {
                            let m964 = f447(v3);
                            match m964 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    v4
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f356() -> Adt65 {
    f183(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f357(v0: Adt111) -> bool {
    let m965 = v0;
    match m965 {
        Adt111::C0 => {
            false
        }
        Adt111::C1 => {
            true
        }
        Adt111::C2(_) => {
            false
        }
    }
}

pub fn f358(v0: Adt111) -> Adt5 {
    let m966 = v0;
    match m966 {
        Adt111::C0 => {
            f27()
        }
        Adt111::C1 => {
            f27()
        }
        Adt111::C2(v1) => {
            v1
        }
    }
}

pub fn f359(v0: Adt111) -> bool {
    let m967 = v0;
    match m967 {
        Adt111::C0 => {
            true
        }
        Adt111::C1 => {
            false
        }
        Adt111::C2(_) => {
            false
        }
    }
}

pub fn f360() -> Adt65 {
    f183(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f361(v0: Str) -> Str {
    let v1: Bytes = {
        let a968 = v0;
        utf8_encode(a968)
    };
    let m975 = {
        let a972 = v1.clone();
        let a973 = 1u64;
        let a974 = {
            let a970 = {
                let a969 = v1;
                length_bytes(a969)
            };
            let a971 = 1u64;
            nat_sub(a970, a971)
        };
        slice_bytes(a972, a973, a974)
    };
    match m975 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m977 = {
                let a976 = v2;
                utf8_decode(a976)
            };
            match m977 {
                None => {
                    Str::lit("")
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f362(v0: Option<Adt5>) -> bool {
    let m978 = v0;
    match m978 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f363(p981: List<Str>, p982: List<Str>) -> List<Str> {
    let m980 = (p981, p982);
    {
        let mut m980 = m980;
        loop {
            let step = {
                let (v0, v1) = m980;
                let m979 = v0;
                match m979.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m980 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(p986: List<(Str, Adt6)>, p987: List<Str>) -> List<Str> {
    let m985 = (p986, p987);
    {
        let mut m985 = m985;
        loop {
            let step = {
                let (v0, v1) = m985;
                let m983 = v0;
                match m983.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h984, _) = v2;
                            h984
                        }, v1)))
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

pub fn f365(v0: Adt36, v1: Fn3, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m989 = {
        let c988: Fn3 = v1;
        c988.apply(v2)?
    };
    match m989 {
        Ok(v4) => {
            f340(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f366(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f354(v0, 0i64)?, 0i64))
}

pub fn f367(p992: List<Adt45>, p993: List<Adt45>) -> List<Adt45> {
    let m991 = (p992, p993);
    {
        let mut m991 = m991;
        loop {
            let step = {
                let (v0, v1) = m991;
                let m990 = v0;
                match m990.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt45>), List<Adt45>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt45>), List<Adt45>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m991 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(p997: List<(Adt45, List<Adt47>)>, p998: List<Adt45>) -> List<Adt45> {
    let m996 = (p997, p998);
    {
        let mut m996 = m996;
        loop {
            let step = {
                let (v0, v1) = m996;
                let m994 = v0;
                match m994.uncons() {
                    None => {
                        Err::<(List<(Adt45, List<Adt47>)>, List<Adt45>), List<Adt45>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt45, List<Adt47>)>, List<Adt45>), List<Adt45>>((v3, List::cons({
                            let (h995, _) = v2;
                            h995
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m996 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: Adt45, v1: Adt47) -> (Adt45, List<Adt47>) {
    (v0, List::cons(v1, List::<Adt47>::nil()))
}

pub fn f370(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt45 {
    Adt45::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt41>, None::<u64>, None::<Adt5>)
}

pub fn f371(v0: Str, v1: Str, v2: Str) -> Adt47 {
    Adt47::C1(Adt49::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f372(v0: Str) -> R<Option<Str>> {
    let m999 = f448(v0)?;
    match m999 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a1000 = f449(f200(), v1);
            Ok(utf8_decode(a1000))
        }
    }
}

pub fn f373(v0: Str) -> R<Option<Str>> {
    let m1003 = f450(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a1001 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a1002 = Str::lit(" ");
        join(a1001, a1002)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m1003 {
        Ok(v1) => {
            let a1004 = v1;
            Ok(utf8_decode(a1004))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f374(v0: Adt5) -> bool {
    if f451(v0.clone()) {
        f452(v0)
    } else {
        false
    }
}

pub fn f375(v0: Adt5) -> bool {
    if f453(v0.clone()) {
        if f454(f234(Str::lit("model"), v0.clone())) {
            f455(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f376(v0: Adt5) -> bool {
    let m1005 = f234(Str::lit("roots"), v0);
    match m1005 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v1) => {
            let m1006 = f294(v1);
            match m1006 {
                None => {
                    false
                }
                Some(v2) => {
                    f456(v2)
                }
            }
        }
    }
}

pub fn f377(v0: Adt58) -> bool {
    let m1008 = match v0 {
        Adt58::C0(_, _, _, h1007) => {
            h1007
        }
    };
    match m1008 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f378(v0: Str, p1015: List<(Str, Str)>) -> Option<Str> {
    let m1014 = p1015;
    {
        let mut m1014 = m1014;
        loop {
            let step = {
                let v1 = m1014;
                let m1009 = v1;
                match m1009.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a1011 = {
                            let (h1010, _) = v2;
                            h1010
                        };
                        let a1012 = v0.clone();
                        if equal(a1011, a1012) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h1013) = v2;
                                h1013
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1014 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(p1020: List<Str>, p1021: u64) -> R<u64> {
    let m1019 = (p1020, p1021);
    {
        let mut m1019 = m1019;
        loop {
            let step = {
                let (v0, v1) = m1019;
                let m1016 = v0;
                match m1016.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a1017 = v1;
                            let a1018 = 1u64;
                            nat_add(a1017, a1018)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1019 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: List<Str>) -> List<Str> {
    f363(f457(v0, f381(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f381() -> u64 {
    100u64
}

pub fn f382(v0: Str, p1024: List<Str>, p1025: List<Str>) -> R<List<Str>> {
    let m1023 = (p1024, p1025);
    {
        let mut m1023 = m1023;
        loop {
            let step = {
                let (v1, v2) = m1023;
                let m1022 = v1;
                match m1022.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f458(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1023 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383() -> List<Str> {
    f363(f459(f170(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f384(v0: Str, v1: Adt5) -> bool {
    f460(f234(v0, v1))
}

pub fn f385(p1028: List<Adt85>, p1029: List<Adt85>) -> List<Adt85> {
    let m1027 = (p1028, p1029);
    {
        let mut m1027 = m1027;
        loop {
            let step = {
                let (v0, v1) = m1027;
                let m1026 = v0;
                match m1026.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), List<Adt85>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt85>, List<Adt85>), List<Adt85>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: Adt6) -> Option<Adt85> {
    let m1030 = f239(v0);
    match m1030 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m1031 = f461(Str::lit("task"), v1.clone());
            match m1031 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m1032 = f462(v2);
                    match m1032 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m1033 = f463(Str::lit("createdMs"), v1.clone());
                            match m1033 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m1034 = f461(Str::lit("plan"), v1);
                                    match m1034 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            let m1035 = f464(v5);
                                            match m1035 {
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

pub fn f387(v0: Adt63) -> Adt64 {
    let m1036 = v0;
    match m1036 {
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

pub fn f388(v0: i64, v1: i64, v2: i64) -> R<bool> {
    let a1042 = {
        let m1041 = {
            let a1039 = {
                let a1037 = v2;
                let a1038 = v1;
                int_sub(a1037, a1038)?
            };
            let a1040 = v0;
            compare(a1039, a1040)
        };
        match m1041 {
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
    Ok(bool_not(a1042))
}

pub fn f389(v0: Adt85, v1: Adt63, v2: Adt86) -> Adt104 {
    Adt104::C0(Adt85::C0(v1, match v0 {
        Adt85::C0(_, h1043, _) => {
            h1043
        }
    }, v2), true)
}

pub fn f390(v0: Adt87, v1: Adt64) -> Adt63 {
    let m1044 = v0;
    match m1044 {
        Adt87::C0(v2) => {
            Adt63::C2(v1, v2)
        }
        Adt87::C1(v3) => {
            Adt63::C3(v1, v3)
        }
    }
}

pub fn f391(v0: Adt64, v1: Str) -> Adt64 {
    Adt64::C0(match v0.clone() {
        Adt64::C0(h1045, _, _, _, _, _) => {
            h1045
        }
    }, match v0.clone() {
        Adt64::C0(_, h1046, _, _, _, _) => {
            h1046
        }
    }, match v0.clone() {
        Adt64::C0(_, _, h1047, _, _, _) => {
            h1047
        }
    }, v1, match v0.clone() {
        Adt64::C0(_, _, _, _, h1048, _) => {
            h1048
        }
    }, match v0 {
        Adt64::C0(_, _, _, _, _, h1049) => {
            h1049
        }
    })
}

pub fn f392(v0: Adt86) -> Adt86 {
    Adt86::C0(match v0.clone() {
        Adt86::C0(h1050, _, _, _) => {
            h1050
        }
    }, List::<(Str, Adt65)>::nil(), match v0.clone() {
        Adt86::C0(_, _, h1051, _) => {
            h1051
        }
    }, match v0 {
        Adt86::C0(_, _, _, h1052) => {
            h1052
        }
    })
}

pub fn f393(v0: List<(Str, Adt65)>, v1: Adt5) -> List<(Str, Adt65)> {
    f465(f466(v0, v1, List::<(Str, Adt65)>::nil()), List::<(Str, Adt65)>::nil())
}

pub fn f394(v0: Adt85, v1: Adt64, v2: i64, v3: Str) -> R<Adt104> {
    let v4: Adt85 = Adt85::C0(Adt63::C0(f391(v1, v3.clone())), match v0.clone() {
        Adt85::C0(_, h1053, _) => {
            h1053
        }
    }, match v0 {
        Adt85::C0(_, _, h1054) => {
            h1054
        }
    });
    Ok(Adt104::C0({
        let m1056 = f194(v4, v2, v3)?;
        match m1056 {
            Adt104::C0(h1055, _) => {
                h1055
            }
        }
    }, true))
}

pub fn f395(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let a1058 = v1;
    let a1059 = {
        let a1057 = v0.clone();
        length_bytes(a1057)
    };
    if nat_eq(a1058, a1059) {
        Ok(Some(Adt96::C0(Bytes::lit(&[]), v1)))
    } else {
        f16(v0, v1)
    }
}

pub fn f396(v0: Bytes, v1: Adt92, v2: u64) -> R<Result<Adt92, Adt94>> {
    let a1061 = v2;
    let a1062 = {
        let a1060 = v0.clone();
        length_bytes(a1060)
    };
    if nat_eq(a1061, a1062) {
        Ok(Ok::<Adt92, Adt94>(v1))
    } else {
        let m1063 = f40(v0.clone(), v2)?;
        match m1063 {
            None => {
                Ok(Err::<Adt92, Adt94>(Adt94::C0(v2)))
            }
            Some(v3) => {
                let m1066 = f467(v0.clone(), {
                    let a1064 = v2;
                    let a1065 = 4u64;
                    nat_add(a1064, a1065)?
                }, v3, List::<List<u64>>::nil())?;
                match m1066 {
                    Ok(v5) => {
                        let a1069 = {
                            let (_, h1067) = v5.clone();
                            h1067
                        };
                        let a1070 = {
                            let a1068 = v0;
                            length_bytes(a1068)
                        };
                        if nat_eq(a1069, a1070) {
                            Ok(Ok::<Adt92, Adt94>(f468(v1, {
                                let (h1071, _) = v5;
                                h1071
                            })))
                        } else {
                            Ok(Err::<Adt92, Adt94>(Adt94::C1({
                                let (_, h1072) = v5;
                                h1072
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

pub fn f397(v0: i64, v1: u64) -> R<i64> {
    f446({
        let a1073 = v0;
        let a1074 = 256i64;
        int_mul(a1073, a1074)?
    }, f354(v1, 0i64)?)
}

pub fn f398(v0: List<u64>) -> Option<Str> {
    let a1075 = f449(f200(), v0);
    utf8_decode(a1075)
}

pub fn f399(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt14, u64), Adt94>> {
    let m1077 = f16(v0, {
        let (_, h1076) = v3.clone();
        h1076
    })?;
    match m1077 {
        None => {
            Ok(Err::<(Adt14, u64), Adt94>(Adt94::C0({
                let (_, h1078) = v3;
                h1078
            })))
        }
        Some(v4) => {
            Ok(f469(v1, v2, {
                let (h1079, _) = v3;
                h1079
            }, v4))
        }
    }
}

pub fn f400(p1084: u64, p1085: List<Adt119>, p1086: List<u64>) -> R<Result<List<u64>, Adt105>> {
    let m1083 = (p1084, (p1085, p1086));
    {
        let mut m1083 = m1083;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1083;
                let m1080 = v0;
                if m1080 == 0 {
                    Err::<(u64, (List<Adt119>, List<u64>)), R<Result<List<u64>, Adt105>>>(Ok(Err::<List<u64>, Adt105>(Adt105::C13)))
                } else {
                    let v3 = m1080 - 1;
                    let m1081 = v1;
                    match m1081.uncons() {
                        None => {
                            Err::<(u64, (List<Adt119>, List<u64>)), R<Result<List<u64>, Adt105>>>(Ok(Ok::<List<u64>, Adt105>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m1082 = f470(v4, v5, v2)?;
                            match m1082 {
                                Adt148::C0(v6) => {
                                    Err::<(u64, (List<Adt119>, List<u64>)), R<Result<List<u64>, Adt105>>>(Ok(Err::<List<u64>, Adt105>(v6)))
                                }
                                Adt148::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt119>, List<u64>)), R<Result<List<u64>, Adt105>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401(p1089: List<u64>, p1090: List<u64>) -> List<u64> {
    let m1088 = (p1089, p1090);
    {
        let mut m1088 = m1088;
        loop {
            let step = {
                let (v0, v1) = m1088;
                let m1087 = v0;
                match m1087.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1088 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: Adt101) -> Adt6 {
    let m1091 = v0;
    match m1091 {
        Adt101::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt101::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f403(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1092 = v0;
    match m1092 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f404() -> i64 {
    -32700i64
}

pub fn f405() -> i64 {
    -32600i64
}

pub fn f406(v0: Adt107) -> Str {
    let m1093 = v0;
    match m1093 {
        Adt107::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt107::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt107::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt107::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt107::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt107::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt107::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt107::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt107::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f407(v0: Adt14, v1: bool, v2: Adt137) -> R<Adt95> {
    let m1095 = match v2.clone() {
        Adt137::C0(_, h1094) => {
            h1094
        }
    };
    match m1095 {
        Ok(v3) => {
            let m1096 = f471(v3.clone())?;
            match m1096 {
                None => {
                    Ok(f6())
                }
                Some(v4) => {
                    f472(v0, f473(match v3.clone() {
                        Adt138::C0(h1097, _, _) => {
                            h1097
                        }
                    }), f474(v1, f105(f475(), f476(match v2 {
                        Adt137::C0(h1098, _) => {
                            h1098
                        }
                    }, v4))?, f477(f478(match v3.clone() {
                        Adt138::C0(_, _, h1099) => {
                            h1099
                        }
                    }, List::<List<u64>>::nil())?, f479(f473(match v3 {
                        Adt138::C0(h1100, _, _) => {
                            h1100
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f105(f480(v5.clone()), f107(Some(match v2 {
                Adt137::C0(h1101, _) => {
                    h1101
                }
            }), f481(v5)?))
        }
    }
}

pub fn f408(p1106: List<Adt93>) -> R<bool> {
    let m1105 = p1106;
    {
        let mut m1105 = m1105;
        loop {
            let step = {
                let v0 = m1105;
                let m1102 = v0;
                match m1102.uncons() {
                    None => {
                        Err::<List<Adt93>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f323(match v1.clone() {
                            Adt93::C0(h1103, _) => {
                                h1103
                            }
                        }, f482())? {
                            if f483(f484(44u64, match v1 {
                                Adt93::C0(_, h1104) => {
                                    h1104
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
                Ok(next) => { m1105 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f409(v0: Adt0, v1: Fn4, v2: u64, v3: Option<Adt120>, v4: List<Str>, v5: Adt112) -> R<Adt137> {
    Ok(Adt137::C0(match v5.clone() {
        Adt112::C0(h1107, _, _) => {
            h1107
        }
    }, f485(v0, v1, v2, f486(v3, v4, v5)?)?))
}

pub fn f410(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt120> {
    let v4: Adt143 = f487(Adt122::C0, f115(v0.clone(), f488())?);
    let v5: Adt143 = f487(Adt122::C1, f115(v0.clone(), f489())?);
    let v6: Adt143 = if f490(v3) {
        f487(Adt122::C2, f491(f115(v0.clone(), f492())?)?)
    } else {
        f493()
    };
    Ok(Adt120::C0(match v4.clone() {
        Adt143::C0(h1108, _) => {
            h1108
        }
    }, match v5.clone() {
        Adt143::C0(h1109, _) => {
            h1109
        }
    }, match v6.clone() {
        Adt143::C0(h1110, _) => {
            h1110
        }
    }, f494(match v4 {
        Adt143::C0(_, h1111) => {
            h1111
        }
    }, f494(match v5 {
        Adt143::C0(_, h1112) => {
            h1112
        }
    }, match v6 {
        Adt143::C0(_, h1113) => {
            h1113
        }
    })), f495(v0, v1, v2)?))
}

pub fn f411(v0: Adt112) -> Option<Str> {
    let a1115 = match v0.clone() {
        Adt112::C0(_, h1114, _) => {
            h1114
        }
    };
    let a1116 = Str::lit("tools/call");
    if equal(a1115, a1116) {
        let m1118 = match v0 {
            Adt112::C0(_, _, h1117) => {
                h1117
            }
        };
        match m1118 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1119 = f234(Str::lit("name"), v1);
                match m1119 {
                    Adt110::C0 => {
                        None::<Str>
                    }
                    Adt110::C1(v2) => {
                        f133(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f412() -> List<Str> {
    List::cons(f496(), List::<Str>::nil())
}

pub fn f413(v0: Adt5) -> R<Adt144> {
    let m1120 = f234(Str::lit("id"), v0);
    match m1120 {
        Adt110::C0 => {
            Ok(Adt144::C0)
        }
        Adt110::C1(v1) => {
            let m1121 = v1;
            match m1121 {
                Adt6::C0 => {
                    Ok(Adt144::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt144::C2)
                }
                Adt6::C2(v3) => {
                    let m1122 = f497(v3)?;
                    match m1122 {
                        None => {
                            Ok(Adt144::C2)
                        }
                        Some(v4) => {
                            Ok(Adt144::C1(Adt101::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt144::C1(Adt101::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt144::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt144::C2)
                }
            }
        }
    }
}

pub fn f414(v0: Adt5) -> Adt107 {
    if f415(v0) {
        Adt107::C8
    } else {
        Adt107::C3
    }
}

pub fn f415(v0: Adt5) -> bool {
    if f384(Str::lit("result"), v0.clone()) {
        true
    } else {
        f384(Str::lit("error"), v0)
    }
}

pub fn f416(v0: Adt101, v1: Adt5) -> Result<Adt112, Adt106> {
    let m1123 = f234(Str::lit("jsonrpc"), v1.clone());
    match m1123 {
        Adt110::C0 => {
            f319(Some(v0), Adt107::C2)
        }
        Adt110::C1(v2) => {
            let m1124 = f133(v2);
            match m1124 {
                None => {
                    f319(Some(v0), Adt107::C2)
                }
                Some(v3) => {
                    let a1125 = v3;
                    let a1126 = f310();
                    if equal(a1125, a1126) {
                        f498(v0, v1)
                    } else {
                        f319(Some(v0), Adt107::C2)
                    }
                }
            }
        }
    }
}

pub fn f417(v0: u64, v1: u64) -> Adt125 {
    let a1127 = v1;
    let a1128 = 0u64;
    if nat_eq(a1127, a1128) {
        Adt125::C0
    } else {
        let a1129 = v1;
        let a1130 = 2u64;
        if nat_eq(a1129, a1130) {
            Adt125::C2(v0)
        } else {
            Adt125::C1({
                let a1134 = v0;
                let a1135 = {
                    let a1131 = v1;
                    let a1132 = 3u64;
                    let a1133 = 0u64;
                    nat_quot(a1131, a1132, a1133)
                };
                nat_sub(a1134, a1135)
            })
        }
    }
}

pub fn f418(p1163: Adt126, p1164: u64, p1165: u64, p1166: List<u64>) -> R<u64> {
    let m1162 = (p1163, (p1164, (p1165, p1166)));
    {
        let mut m1162 = m1162;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1162;
                let m1136 = v3;
                match m1136.uncons() {
                    None => {
                        let m1137 = v0;
                        match m1137 {
                            Adt126::C0 => {
                                Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt126::C1 => {
                                Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt126::C2 => {
                                Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt126::C3 => {
                                Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1138 = v0;
                        match m1138 {
                            Adt126::C0 => {
                                let a1139 = v4;
                                let a1140 = 128u64;
                                if nat_lt(a1139, a1140) {
                                    Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1141 = v4;
                                    let a1142 = 194u64;
                                    if nat_lt(a1141, a1142) {
                                        let v6: u64 = f499({
                                            let a1143 = List::cons(v4, v5);
                                            length_list(a1143)
                                        })?;
                                        Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1144 = v4;
                                        let a1145 = 224u64;
                                        if nat_lt(a1144, a1145) {
                                            Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1146 = v4;
                                            let a1147 = 224u64;
                                            if nat_eq(a1146, a1147) {
                                                Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1148 = v4;
                                                let a1149 = 237u64;
                                                if nat_eq(a1148, a1149) {
                                                    Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1150 = v4;
                                                    let a1151 = 240u64;
                                                    if nat_lt(a1150, a1151) {
                                                        Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1152 = v4;
                                                        let a1153 = 240u64;
                                                        if nat_eq(a1152, a1153) {
                                                            Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1154 = v4;
                                                            let a1155 = 244u64;
                                                            if nat_eq(a1154, a1155) {
                                                                Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1156 = v4;
                                                                let a1157 = 244u64;
                                                                if nat_lt(a1156, a1157) {
                                                                    Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f499({
                                                                        let a1158 = List::cons(v4, v5);
                                                                        length_list(a1158)
                                                                    })?;
                                                                    Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt126::C1 => {
                                if f500(v4, v1, v2) {
                                    Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f499({
                                        let a1159 = List::cons(v4, v5);
                                        length_list(a1159)
                                    })?;
                                    Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt126::C2 => {
                                if f500(v4, v1, v2) {
                                    Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f499({
                                        let a1160 = List::cons(v4, v5);
                                        length_list(a1160)
                                    })?;
                                    Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt126::C3 => {
                                if f500(v4, v1, v2) {
                                    Ok::<(Adt126, (u64, (u64, List<u64>))), R<u64>>((Adt126::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f499({
                                        let a1161 = List::cons(v4, v5);
                                        length_list(a1161)
                                    })?;
                                    Err::<(Adt126, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
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

pub fn f419(v0: Adt127, v1: u64) -> R<Result<Adt6, Adt105>> {
    let m1168 = match v0.clone() {
        Adt127::C0(h1167, _, _) => {
            h1167
        }
    };
    match m1168 {
        Adt128::C0 => {
            Ok(Err::<Adt6, Adt105>(Adt105::C3))
        }
        Adt128::C1 => {
            Ok(Err::<Adt6, Adt105>(Adt105::C3))
        }
        Adt128::C2 => {
            Ok(Err::<Adt6, Adt105>(Adt105::C3))
        }
        Adt128::C3 => {
            Ok(Err::<Adt6, Adt105>(Adt105::C3))
        }
        Adt128::C4 => {
            Ok(Err::<Adt6, Adt105>(Adt105::C3))
        }
        Adt128::C5 => {
            Ok(Err::<Adt6, Adt105>(Adt105::C3))
        }
        Adt128::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt105>(Adt105::C3))
        }
        Adt128::C7(_, _) => {
            Ok(Err::<Adt6, Adt105>(Adt105::C3))
        }
        Adt128::C8(v3) => {
            let m1169 = f501(v3)?;
            match m1169 {
                Adt151::C0 => {
                    Ok(Err::<Adt6, Adt105>(Adt105::C7(v1)))
                }
                Adt151::C1 => {
                    Ok(Err::<Adt6, Adt105>(Adt105::C8(v1)))
                }
                Adt151::C2(v4) => {
                    let m1174 = {
                        let m1173 = f502(v4, match v0.clone() {
                            Adt127::C0(_, h1171, _) => {
                                h1171
                            }
                        }, match v0 {
                            Adt127::C0(_, _, h1172) => {
                                h1172
                            }
                        });
                        match m1173 {
                            Adt127::C0(h1170, _, _) => {
                                h1170
                            }
                        }
                    };
                    match m1174 {
                        Adt128::C0 => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C1 => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C2 => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C3 => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C4 => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C5 => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C7(_, _) => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C8(_) => {
                            Ok(Err::<Adt6, Adt105>(Adt105::C3))
                        }
                        Adt128::C9(v5) => {
                            Ok(Ok::<Adt6, Adt105>(v5))
                        }
                    }
                }
            }
        }
        Adt128::C9(v2) => {
            Ok(Ok::<Adt6, Adt105>(v2))
        }
    }
}

pub fn f420(v0: Adt127, v1: u64, v2: u64) -> R<Adt145> {
    let m1176 = match v0.clone() {
        Adt127::C0(h1175, _, _) => {
            h1175
        }
    };
    match m1176 {
        Adt128::C0 => {
            if f503(v2) {
                Ok(Adt145::C1(v0))
            } else {
                f504(v0, v1, v2)
            }
        }
        Adt128::C1 => {
            if f503(v2) {
                Ok(Adt145::C1(v0))
            } else {
                let a1177 = v2;
                let a1178 = 93u64;
                if nat_eq(a1177, a1178) {
                    Ok(f505(v0, v1))
                } else {
                    f504(v0, v1, v2)
                }
            }
        }
        Adt128::C2 => {
            Ok(f506(v0, v1, v2))
        }
        Adt128::C3 => {
            if f503(v2) {
                Ok(Adt145::C1(v0))
            } else {
                let a1179 = v2;
                let a1180 = 125u64;
                if nat_eq(a1179, a1180) {
                    Ok(f507(v0, v1))
                } else {
                    let a1181 = v2;
                    let a1182 = 34u64;
                    if nat_eq(a1181, a1182) {
                        Ok(f508(v0, Adt128::C6(Adt129::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f509(Adt105::C2(v1)))
                    }
                }
            }
        }
        Adt128::C4 => {
            if f503(v2) {
                Ok(Adt145::C1(v0))
            } else {
                let a1183 = v2;
                let a1184 = 34u64;
                if nat_eq(a1183, a1184) {
                    Ok(f508(v0, Adt128::C6(Adt129::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f509(Adt105::C2(v1)))
                }
            }
        }
        Adt128::C5 => {
            if f503(v2) {
                Ok(Adt145::C1(v0))
            } else {
                let a1185 = v2;
                let a1186 = 58u64;
                if nat_eq(a1185, a1186) {
                    Ok(f508(v0, Adt128::C0))
                } else {
                    Ok(f509(Adt105::C2(v1)))
                }
            }
        }
        Adt128::C6(v3, v4, v5) => {
            f510(v0, v1, v2, v3, v4, v5)
        }
        Adt128::C7(v6, v7) => {
            f511(v0, v1, v2, v6, v7)
        }
        Adt128::C8(v8) => {
            f512(v0, v1, v2, v8)
        }
        Adt128::C9(_) => {
            Ok(f506(v0, v1, v2))
        }
    }
}

pub fn f421(v0: u64) -> R<u64> {
    let a1187 = 64u64;
    let a1188 = v0;
    if if nat_lt(a1187, a1188) {
        let a1189 = v0;
        let a1190 = 91u64;
        nat_lt(a1189, a1190)
    } else {
        false
    } {
        let a1191 = v0;
        let a1192 = 32u64;
        nat_add(a1191, a1192)
    } else {
        Ok(v0)
    }
}

pub fn f422(p1195: List<u64>) -> List<u64> {
    let m1194 = p1195;
    {
        let mut m1194 = m1194;
        loop {
            let step = {
                let v0 = m1194;
                let m1193 = v0;
                match m1193.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f513(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
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

pub fn f423(v0: u64) -> bool {
    let a1196 = v0;
    let a1197 = 9u64;
    if nat_eq(a1196, a1197) {
        true
    } else {
        let a1198 = 31u64;
        let a1199 = v0;
        if nat_lt(a1198, a1199) {
            let a1200 = v0;
            let a1201 = 127u64;
            nat_lt(a1200, a1201)
        } else {
            false
        }
    }
}

pub fn f424(p1204: List<u64>) -> bool {
    let m1203 = p1204;
    {
        let mut m1203 = m1203;
        loop {
            let step = {
                let v0 = m1203;
                let m1202 = v0;
                match m1202.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f514(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(p1207: List<u64>, v1: bool) -> bool {
    let m1206 = p1207;
    {
        let mut m1206 = m1206;
        loop {
            let step = {
                let v0 = m1206;
                let m1205 = v0;
                match m1205.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f515(v2)
                        } else {
                            f516(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f426() -> u64 {
    64u64
}

pub fn f427(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1208 = f347(Str::lit("type"), v1);
    match m1208 {
        Adt110::C0 => {
            Some(Adt98::C3(v0))
        }
        Adt110::C1(v2) => {
            let m1209 = f133(v2);
            match m1209 {
                None => {
                    Some(Adt98::C3(v0))
                }
                Some(v3) => {
                    let a1210 = v3;
                    let a1211 = Str::lit("object");
                    if equal(a1210, a1211) {
                        None::<Adt98>
                    } else {
                        Some(Adt98::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f428(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1212 = f347(Str::lit("$schema"), v1);
    match m1212 {
        Adt110::C0 => {
            None::<Adt98>
        }
        Adt110::C1(v2) => {
            let m1213 = f133(v2);
            match m1213 {
                None => {
                    Some(Adt98::C2(v0))
                }
                Some(v3) => {
                    let a1214 = v3;
                    let a1215 = f517();
                    if equal(a1214, a1215) {
                        None::<Adt98>
                    } else {
                        Some(Adt98::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f429(v0: Adt36) -> R<Option<Adt98>> {
    let m1217 = match v0.clone() {
        Adt36::C0(_, _, _, _, _, h1216, _, _) => {
            h1216
        }
    };
    match m1217 {
        None => {
            Ok(None::<Adt98>)
        }
        Some(v1) => {
            let m1220 = f428(match v0.clone() {
                Adt36::C0(h1218, _, _, _, _, _, _, _) => {
                    h1218
                }
            }, match v1.clone() {
                Adt5::C0(h1219) => {
                    h1219
                }
            });
            match m1220 {
                None => {
                    let m1224 = f337(f338(), match v0 {
                        Adt36::C0(h1221, _, _, _, _, _, _, _) => {
                            h1221
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1222) => {
                            h1222
                        }
                    }), List::cons(Adt113::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1223) => {
                            h1223
                        }
                    }), false, List::<Str>::nil()), List::<Adt113>::nil()), f339())?;
                    match m1224 {
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

pub fn f430(v0: Str, v1: Adt6, v2: Adt113, v3: Adt133, v4: List<Adt113>) -> R<Result<(List<Adt113>, Adt133), Adt98>> {
    let m1225 = v2;
    match m1225 {
        Adt113::C0(v5, v6, v7) => {
            let m1226 = f447(v5.clone());
            match m1226 {
                None => {
                    let m1227 = f239(v5);
                    match m1227 {
                        None => {
                            Ok(Err::<(List<Adt113>, Adt133), Adt98>(Adt98::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f518(v0, v1, match v9 {
                                Adt5::C0(h1228) => {
                                    h1228
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt113>, Adt133), Adt98>((v4, v3)))
                }
            }
        }
    }
}

pub fn f431(p1237: List<Adt39>) -> R<bool> {
    let m1236 = p1237;
    {
        let mut m1236 = m1236;
        loop {
            let step = {
                let v0 = m1236;
                let m1229 = v0;
                match m1229.uncons() {
                    None => {
                        Err::<List<Adt39>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m1230 = v1;
                        match m1230 {
                            Adt39::C0(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C1(v4) => {
                                if f519(match v4 {
                                    Adt43::C0(h1231, _, _, _) => {
                                        h1231
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C2(v5) => {
                                if f519(match v5 {
                                    Adt44::C0(h1232, _, _, _) => {
                                        h1232
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
                                let m1234 = match v7 {
                                    Adt46::C0(h1233, _, _) => {
                                        h1233
                                    }
                                };
                                match m1234 {
                                    Adt47::C0(_) => {
                                        Ok::<List<Adt39>, R<bool>>(v2)
                                    }
                                    Adt47::C1(v9) => {
                                        if f519(match v9 {
                                            Adt49::C0(_, _, h1235, _) => {
                                                h1235
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
                Ok(next) => { m1236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f520(match v1.clone() {
        Adt50::C0(h1238, _) => {
            h1238
        }
    }) {
        f521(v1)
    } else {
        let m1240 = match v0 {
            Adt36::C0(_, _, _, _, _, h1239, _, _) => {
                h1239
            }
        };
        match m1240 {
            None => {
                f521(v1)
            }
            Some(v2) => {
                f522(v2, v1)
            }
        }
    }
}

pub fn f433() -> Adt90 {
    Adt90::C9(Str::lit("the host supplied no task id"))
}

pub fn f434() -> Adt90 {
    Adt90::C9(Str::lit("the host supplied a task id that a task already has"))
}

pub fn f435(v0: Adt14, v1: Adt86) -> Adt85 {
    Adt85::C0(Adt63::C0(Adt64::C0(match v0.clone() {
        Adt14::C0(_, _, h1241, _) => {
            h1241
        }
    }, None::<Str>, match v0.clone() {
        Adt14::C0(_, h1242, _, _) => {
            h1242
        }
    }, match v0.clone() {
        Adt14::C0(_, h1243, _, _) => {
            h1243
        }
    }, Some(f523()), Some(f524()))), match v0 {
        Adt14::C0(h1244, _, _, _) => {
            h1244
        }
    }, v1)
}

pub fn f436(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt115>) -> R<Adt146> {
    let m1245 = f447(v1.clone());
    match m1245 {
        None => {
            let m1246 = f239(v1);
            match m1246 {
                None => {
                    Ok(Adt146::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f525(v0, match v5 {
                        Adt5::C0(h1247) => {
                            h1247
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt146::C3(List::<Adt117>::nil()))
            } else {
                Ok(Adt146::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f437(v0: Adt117, v1: List<Adt117>) -> R<Adt134> {
    let m1248 = v0;
    match m1248 {
        Adt117::C0(_, _, _) => {
            Ok(Adt134::C0(Adt116::C0, v1))
        }
        Adt117::C1(_, _, _) => {
            Ok(Adt134::C0(Adt116::C0, v1))
        }
        Adt117::C2(v8, v9, v10, v11) => {
            Ok(f526(v8, v9, v10, {
                let a1249 = v11;
                let a1250 = 1u64;
                nat_add(a1249, a1250)?
            }, v1))
        }
        Adt117::C3(v12) => {
            Ok(Adt134::C0(Adt116::C1(v12, Str::lit("not")), v1))
        }
        Adt117::C4(v13, _, v15, v16) => {
            Ok(f527(v13, v15, v16, v1))
        }
        Adt117::C5(_, _, _, _) => {
            Ok(Adt134::C0(Adt116::C0, v1))
        }
    }
}

pub fn f438(v0: Adt118, v1: Adt6, v2: Adt6, v3: List<Adt115>) -> Adt146 {
    let m1251 = f239(v1);
    match m1251 {
        None => {
            Adt146::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m1252 = v0;
            match m1252 {
                Adt118::C0 => {
                    f528(match v4 {
                        Adt5::C0(h1253) => {
                            h1253
                        }
                    }, v2, v3)
                }
                Adt118::C1 => {
                    f529(match v4 {
                        Adt5::C0(h1254) => {
                            h1254
                        }
                    }, v2, v3)
                }
                Adt118::C2 => {
                    f530(match v4 {
                        Adt5::C0(h1255) => {
                            h1255
                        }
                    }, v2, v3)
                }
                Adt118::C3 => {
                    f531(match v4 {
                        Adt5::C0(h1256) => {
                            h1256
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f439(v0: Adt117, v1: List<Adt117>, v2: List<Adt115>, v3: Str) -> Adt134 {
    let m1257 = v0;
    match m1257 {
        Adt117::C0(_, _, _) => {
            Adt134::C0(Adt116::C1(v2, v3), v1)
        }
        Adt117::C1(v7, v8, v9) => {
            f532(v7, v8, v9, v1)
        }
        Adt117::C2(v10, v11, v12, v13) => {
            f526(v10, v11, v12, v13, v1)
        }
        Adt117::C3(_) => {
            Adt134::C0(Adt116::C0, v1)
        }
        Adt117::C4(_, v16, v17, v18) => {
            f527(v16, v17, v18, v1)
        }
        Adt117::C5(_, _, _, _) => {
            Adt134::C0(Adt116::C1(v2, v3), v1)
        }
    }
}

pub fn f440(p1260: List<Adt115>, p1261: List<Str>) -> R<List<Str>> {
    let m1259 = (p1260, p1261);
    {
        let mut m1259 = m1259;
        loop {
            let step = {
                let (v0, v1) = m1259;
                let m1258 = v0;
                match m1258.uncons() {
                    None => {
                        Err::<(List<Adt115>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt115>, List<Str>), R<List<Str>>>((v3, List::cons(f533(v2)?, v1)))
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

pub fn f441(v0: Str, v1: Str) -> Adt147 {
    let a1262 = v0;
    let a1263 = v1;
    if equal(a1262, a1263) {
        Adt147::C0
    } else {
        Adt147::C1
    }
}

pub fn f442(p1278: u64, p1279: Adt135, p1280: List<Adt136>) -> R<Option<Adt6>> {
    let m1277 = (p1278, (p1279, p1280));
    {
        let mut m1277 = m1277;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1277;
                let m1264 = v0;
                if m1264 == 0 {
                    Err::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1264 - 1;
                    let m1265 = v1;
                    match m1265 {
                        Adt135::C0(v4) => {
                            let m1266 = v4.clone();
                            match m1266 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f534(v7)?;
                                    Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1267 = v9;
                                    match m1267.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C0(v10), List::cons(Adt136::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1268 = v12;
                                    match m1268.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f534({
                                                let (h1269, _) = v13;
                                                h1269
                                            })?;
                                            Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C0({
                                                let (_, h1270) = v13;
                                                h1270
                                            }), List::cons(Adt136::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt135::C1(v16) => {
                            let m1271 = v2;
                            match m1271.uncons() {
                                None => {
                                    Err::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1272 = v17;
                                    match m1272 {
                                        Adt136::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1273 = v20;
                                            match m1273.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C1(Adt6::C4(f445(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C0(v22), List::cons(Adt136::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt136::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1274 = v26;
                                            match m1274.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C1(Adt6::C5(f535(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f534({
                                                        let (h1275, _) = v28;
                                                        h1275
                                                    })?;
                                                    Ok::<(u64, (Adt135, List<Adt136>)), R<Option<Adt6>>>((v3, (Adt135::C0({
                                                        let (_, h1276) = v28;
                                                        h1276
                                                    }), List::cons(Adt136::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m1277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443() -> u64 {
    67108864u64
}

pub fn f444(p1284: List<Adt39>, p1285: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1283 = (p1284, p1285);
    {
        let mut m1283 = m1283;
        loop {
            let step = {
                let (v0, v1) = m1283;
                let m1281 = v0;
                match m1281.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1282 = f536(v2)?;
                        match m1282 {
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
                Ok(next) => { m1283 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(p1288: List<Adt6>, p1289: List<Adt6>) -> List<Adt6> {
    let m1287 = (p1288, p1289);
    {
        let mut m1287 = m1287;
        loop {
            let step = {
                let (v0, v1) = m1287;
                let m1286 = v0;
                match m1286.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(v0: i64, v1: i64) -> R<i64> {
    let a1291 = v0;
    let a1292 = {
        let a1290 = v1;
        int_neg(a1290)?
    };
    int_sub(a1291, a1292)
}

pub fn f447(v0: Adt6) -> Option<bool> {
    let m1293 = v0;
    match m1293 {
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

pub fn f448(v0: Str) -> R<Option<List<u64>>> {
    let m1294 = f537(f538(f539())?, f538(v0)?);
    match m1294 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1295 = f537(f540(List::cons(47u64, f538(f541())?)), f540(v1));
            match m1295 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f540(v2);
                    if if f542(v3.clone()) {
                        f543(v3.clone())
                    } else {
                        false
                    } {
                        Ok(Some(v3))
                    } else {
                        Ok(None::<List<u64>>)
                    }
                }
            }
        }
    }
}

pub fn f449(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1296 = v0;
    let a1297 = f544(f200(), f545(List::<Bytes>::nil(), v1));
    append_bytes(a1296, a1297)
}

pub fn f450(v0: Adt6) -> R<Result<Bytes, Adt105>> {
    let m1298 = f202(v0)?;
    match m1298 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt105>(f449(f200(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt105>(v2))
        }
    }
}

pub fn f451(v0: Adt5) -> bool {
    if f546(f234(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f546(f234(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f546(f234(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f452(v0: Adt5) -> bool {
    let m1299 = f234(Str::lit("content"), v0);
    match m1299 {
        Adt110::C0 => {
            true
        }
        Adt110::C1(v1) => {
            let m1300 = f239(v1);
            match m1300 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f453(v0: Adt5) -> bool {
    if f546(f234(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f546(f234(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f454(v0: Adt110) -> bool {
    let m1301 = v0;
    match m1301 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v1) => {
            let m1302 = f133(v1);
            match m1302 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f455(v0: Adt5) -> bool {
    let m1303 = f234(Str::lit("content"), v0);
    match m1303 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v1) => {
            let m1304 = f239(v1.clone());
            match m1304 {
                None => {
                    let m1305 = f294(v1);
                    match m1305 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f456(p1309: List<Adt6>) -> bool {
    let m1308 = p1309;
    {
        let mut m1308 = m1308;
        loop {
            let step = {
                let v0 = m1308;
                let m1306 = v0;
                match m1306.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m1307 = f239(v1);
                        match m1307 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f454(f234(Str::lit("uri"), v3)) {
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
                Ok(next) => { m1308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(p1313: List<Str>, p1314: u64, p1315: List<Str>) -> List<Str> {
    let m1312 = (p1313, (p1314, p1315));
    {
        let mut m1312 = m1312;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1312;
                let m1310 = v0;
                match m1310.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m1311 = v1;
                        if m1311 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m1311 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
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

pub fn f458(v0: Str, v1: Str) -> R<bool> {
    Ok(f547(f538(v0)?, f538(v1)?))
}

pub fn f459(p1319: List<Adt45>, p1320: List<Str>) -> List<Str> {
    let m1318 = (p1319, p1320);
    {
        let mut m1318 = m1318;
        loop {
            let step = {
                let (v0, v1) = m1318;
                let m1316 = v0;
                match m1316.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt45::C0(_, _, _, h1317, _, _, _, _, _) => {
                                h1317
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: Adt110) -> bool {
    let m1321 = v0;
    match m1321 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(_) => {
            true
        }
    }
}

pub fn f461(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m1322 = f234(v0, v1);
    match m1322 {
        Adt110::C0 => {
            None::<Adt5>
        }
        Adt110::C1(v2) => {
            f239(v2)
        }
    }
}

pub fn f462(v0: Adt5) -> Option<Adt63> {
    let m1323 = f548(v0.clone());
    match m1323 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m1324 = f549(Str::lit("status"), v0.clone());
            match m1324 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let a1325 = v2.clone();
                    let a1326 = Str::lit("working");
                    if equal(a1325, a1326) {
                        Some(Adt63::C0(v1))
                    } else {
                        let a1327 = v2.clone();
                        let a1328 = Str::lit("cancelled");
                        if equal(a1327, a1328) {
                            Some(Adt63::C4(v1))
                        } else {
                            let a1329 = v2.clone();
                            let a1330 = Str::lit("input_required");
                            if equal(a1329, a1330) {
                                let m1331 = f550(Str::lit("inputRequests"), v0);
                                match m1331 {
                                    None => {
                                        None::<Adt63>
                                    }
                                    Some(v3) => {
                                        Some(Adt63::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a1332 = v2.clone();
                                let a1333 = Str::lit("completed");
                                if equal(a1332, a1333) {
                                    let m1334 = f461(Str::lit("result"), v0);
                                    match m1334 {
                                        None => {
                                            None::<Adt63>
                                        }
                                        Some(v4) => {
                                            Some(Adt63::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a1335 = v2;
                                    let a1336 = Str::lit("failed");
                                    if equal(a1335, a1336) {
                                        let m1337 = f461(Str::lit("error"), v0);
                                        match m1337 {
                                            None => {
                                                None::<Adt63>
                                            }
                                            Some(v5) => {
                                                let m1338 = f551(v5);
                                                match m1338 {
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

pub fn f463(v0: Str, v1: Adt5) -> Option<i64> {
    let m1339 = f234(v0, v1);
    match m1339 {
        Adt110::C0 => {
            None::<i64>
        }
        Adt110::C1(v2) => {
            f235(v2)
        }
    }
}

pub fn f464(v0: Adt5) -> Option<Adt86> {
    let m1340 = f463(Str::lit("parkAfterMs"), v0.clone());
    match m1340 {
        None => {
            None::<Adt86>
        }
        Some(v1) => {
            let m1341 = f550(Str::lit("parked"), v0.clone());
            match m1341 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m1342 = f463(Str::lit("finishAfterMs"), v0.clone());
                    match m1342 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            let m1343 = f461(Str::lit("outcome"), v0);
                            match m1343 {
                                None => {
                                    None::<Adt86>
                                }
                                Some(v4) => {
                                    let m1344 = f552(v4);
                                    match m1344 {
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

pub fn f465(p1347: List<(Str, Adt65)>, p1348: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m1346 = (p1347, p1348);
    {
        let mut m1346 = m1346;
        loop {
            let step = {
                let (v0, v1) = m1346;
                let m1345 = v0;
                match m1345.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(p1352: List<(Str, Adt65)>, v1: Adt5, p1353: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m1351 = (p1352, p1353);
    {
        let mut m1351 = m1351;
        loop {
            let step = {
                let (v0, v2) = m1351;
                let m1349 = v0;
                match m1349.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v2)
                    }
                    Some((v3, v4)) => {
                        if f384({
                            let (h1350, _) = v3.clone();
                            h1350
                        }, v1.clone()) {
                            Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v4, v2))
                        } else {
                            Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v4, List::cons(v3, v2)))
                        }
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

pub fn f467(v0: Bytes, p1359: u64, p1360: u64, p1361: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt94>> {
    let m1358 = (p1359, (p1360, p1361));
    {
        let mut m1358 = m1358;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1358;
                let m1354 = v2;
                if m1354 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Ok::<(List<List<u64>>, u64), Adt94>((f45(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m1354 - 1;
                    let m1355 = f16(v0.clone(), v1)?;
                    match m1355 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Err::<(List<List<u64>>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>((match v5.clone() {
                                Adt96::C0(_, h1356) => {
                                    h1356
                                }
                            }, (v4, List::cons(f198(match v5 {
                                Adt96::C0(h1357, _) => {
                                    h1357
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f468(v0: Adt92, v1: List<List<u64>>) -> Adt92 {
    Adt92::C0(match v0.clone() {
        Adt92::C0(h1362, _, _, _, _, _, _, _) => {
            h1362
        }
    }, match v0.clone() {
        Adt92::C0(_, h1363, _, _, _, _, _, _) => {
            h1363
        }
    }, match v0.clone() {
        Adt92::C0(_, _, h1364, _, _, _, _, _) => {
            h1364
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, h1365, _, _, _, _) => {
            h1365
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, _, h1366, _, _, _) => {
            h1366
        }
    }, v1, match v0.clone() {
        Adt92::C0(_, _, _, _, _, _, h1367, _) => {
            h1367
        }
    }, match v0 {
        Adt92::C0(_, _, _, _, _, _, _, h1368) => {
            h1368
        }
    })
}

pub fn f469(v0: i64, v1: Str, v2: Str, v3: Adt96) -> Result<(Adt14, u64), Adt94> {
    Ok::<(Adt14, u64), Adt94>((Adt14::C0(v0, v1, v2, match v3.clone() {
        Adt96::C0(h1369, _) => {
            h1369
        }
    }), match v3 {
        Adt96::C0(_, h1370) => {
            h1370
        }
    }))
}

pub fn f470(v0: Adt119, v1: List<Adt119>, v2: List<u64>) -> R<Adt148> {
    let m1371 = v0;
    match m1371 {
        Adt119::C0(v3) => {
            Ok(Adt148::C1(v1, f401(v3, v2)))
        }
        Adt119::C1(v4, v5) => {
            f553(v4, v5, v1, v2)
        }
        Adt119::C2(v6, v7) => {
            let m1372 = v7;
            match m1372.uncons() {
                None => {
                    Ok(Adt148::C1(List::cons(Adt119::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt148::C1(List::cons(Adt119::C1(v6, v8), List::cons(Adt119::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt119::C3(v10, v11) => {
            let m1373 = v11.clone();
            match m1373.uncons() {
                None => {
                    Ok(Adt148::C1(List::cons(Adt119::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt148::C1(List::cons(Adt119::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt119::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt119::C4(v14, v15) => {
            let m1374 = v15;
            match m1374.uncons() {
                None => {
                    Ok(Adt148::C1(List::cons(Adt119::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt148::C1(List::cons(Adt119::C0({
                        let a1376 = f554({
                            let (h1375, _) = v16;
                            h1375
                        })?;
                        let a1377 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1376, a1377)
                    }), List::cons(Adt119::C1(v14, {
                        let (_, h1378) = v16;
                        h1378
                    }), List::cons(Adt119::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt119::C5(v18, v19) => {
            let m1379 = v19.clone();
            match m1379.uncons() {
                None => {
                    Ok(Adt148::C1(List::cons(Adt119::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt148::C1(List::cons(Adt119::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt119::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f471(v0: Adt138) -> R<Option<Adt5>> {
    let m1381 = f555(match v0.clone() {
        Adt138::C0(h1380, _, _) => {
            h1380
        }
    })?;
    match m1381 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1384 = v1;
                let a1385 = {
                    let m1383 = f556(v0);
                    match m1383 {
                        Adt5::C0(h1382) => {
                            h1382
                        }
                    }
                };
                append_list(a1384, a1385)
            })))
        }
    }
}

pub fn f472(v0: Adt14, v1: Option<Adt88>, v2: Adt95) -> R<Adt95> {
    let m1386 = v1;
    match m1386 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a1388 = match v2.clone() {
                Adt95::C0(h1387, _, _, _, _) => {
                    h1387
                }
            };
            let a1389 = f475();
            if nat_eq(a1388, a1389) {
                Ok(f557(f558(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f473(v0: Adt33) -> Option<Adt88> {
    let m1390 = v0;
    match m1390 {
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
                Adt83::C0(_, _, h1391) => {
                    h1391
                }
            }, Adt89::C0, false))
        }
        Adt33::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f474(v0: bool, v1: Adt95, v2: Option<List<List<u64>>>) -> Adt95 {
    let m1392 = v2;
    match m1392 {
        None => {
            f6()
        }
        Some(v3) => {
            f559(v0, v1, f45(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f475() -> u64 {
    200u64
}

pub fn f476(v0: Adt101, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f310())), List::cons((Str::lit("id"), f402(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1393) => {
            h1393
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f477(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m1394 = v0;
    match m1394 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m1395 = v1;
            match m1395 {
                None => {
                    None::<List<List<u64>>>
                }
                Some(v3) => {
                    Some(f45(v3, v2))
                }
            }
        }
    }
}

pub fn f478(p1399: List<Adt140>, p1400: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1398 = (p1399, p1400);
    {
        let mut m1398 = m1398;
        loop {
            let step = {
                let (v0, v1) = m1398;
                let m1396 = v0;
                match m1396.uncons() {
                    None => {
                        Err::<(List<Adt140>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1397 = f560(v2)?;
                        match m1397 {
                            None => {
                                Err::<(List<Adt140>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt140>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: Option<Adt88>) -> R<Option<List<List<u64>>>> {
    let m1401 = v0;
    match m1401 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f561(v1)
        }
    }
}

pub fn f480(v0: Adt141) -> u64 {
    let m1402 = v0;
    match m1402 {
        Adt141::C0(_) => {
            f209()
        }
        Adt141::C1(_, _) => {
            f209()
        }
        Adt141::C2(_) => {
            f209()
        }
        Adt141::C3(_) => {
            f562()
        }
        Adt141::C4(v6) => {
            f563(v6)
        }
    }
}

pub fn f481(v0: Adt141) -> R<Adt81> {
    Ok(Adt81::C0(f564(v0.clone()), f565(v0.clone()), f566(v0)?))
}

pub fn f482() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f483(p1405: List<List<u64>>) -> R<bool> {
    let m1404 = p1405;
    {
        let mut m1404 = m1404;
        loop {
            let step = {
                let v0 = m1404;
                let m1403 = v0;
                match m1403.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f567(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: u64, p1410: List<u64>, p1411: List<u64>, p1412: List<List<u64>>) -> List<List<u64>> {
    let m1409 = (p1410, (p1411, p1412));
    {
        let mut m1409 = m1409;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1409;
                let m1406 = v1;
                match m1406.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f45(List::cons(f333(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1407 = v4;
                        let a1408 = v0;
                        if nat_eq(a1407, a1408) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f333(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
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

pub fn f485(v0: Adt0, v1: Fn4, v2: u64, v3: Result<Adt149, Adt141>) -> R<Result<Adt138, Adt141>> {
    let m1413 = v3;
    match m1413 {
        Ok(v5) => {
            f568(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt138, Adt141>(v4))
        }
    }
}

pub fn f486(v0: Option<Adt120>, v1: List<Str>, v2: Adt112) -> R<Result<Adt149, Adt141>> {
    Ok(f569(f570(match v2.clone() {
        Adt112::C0(_, h1414, _) => {
            h1414
        }
    }), f571(v0, v1, v2)?))
}

pub fn f487(v0: Adt122, v1: Adt108) -> Adt143 {
    let m1415 = v1;
    match m1415 {
        Adt108::C0 => {
            f493()
        }
        Adt108::C1 => {
            Adt143::C0(None::<Str>, Some(Adt121::C6(v0)))
        }
        Adt108::C2 => {
            Adt143::C0(None::<Str>, Some(Adt121::C7(v0)))
        }
        Adt108::C3(v2) => {
            let m1416 = f398(v2);
            match m1416 {
                None => {
                    Adt143::C0(None::<Str>, Some(Adt121::C6(v0)))
                }
                Some(v3) => {
                    Adt143::C0(Some(v3), None::<Adt121>)
                }
            }
        }
    }
}

pub fn f488() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f489() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f490(v0: Str) -> bool {
    let m1417 = f572(v0);
    match m1417 {
        Adt155::C0 => {
            false
        }
        Adt155::C1 => {
            true
        }
        Adt155::C2 => {
            true
        }
        Adt155::C3 => {
            true
        }
    }
}

pub fn f491(v0: Adt108) -> R<Adt108> {
    let m1418 = v0.clone();
    match m1418 {
        Adt108::C0 => {
            Ok(v0)
        }
        Adt108::C1 => {
            Ok(v0)
        }
        Adt108::C2 => {
            Ok(v0)
        }
        Adt108::C3(v1) => {
            let m1419 = f573(v1);
            match m1419 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1420 = f574(v2)?;
                    match m1420 {
                        None => {
                            Ok(Adt108::C1)
                        }
                        Some(v3) => {
                            Ok(Adt108::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f492() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f493() -> Adt143 {
    Adt143::C0(None::<Str>, None::<Adt121>)
}

pub fn f494(v0: Option<Adt121>, v1: Option<Adt121>) -> Option<Adt121> {
    let m1421 = v0.clone();
    match m1421 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f495(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt123>> {
    Ok(f575(f576(v0, v1, v2, List::<Adt123>::nil())?, List::<Adt123>::nil()))
}

pub fn f496() -> Str {
    Str::lit("2026-07-28")
}

pub fn f497(v0: Adt7) -> R<Option<i64>> {
    let m1424 = f577(40u64, match v0.clone() {
        Adt7::C0(h1422, _) => {
            h1422
        }
    }, match v0 {
        Adt7::C0(_, h1423) => {
            h1423
        }
    })?;
    match m1424 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f578(v1))
        }
    }
}

pub fn f498(v0: Adt101, v1: Adt5) -> Result<Adt112, Adt106> {
    let m1425 = f234(Str::lit("method"), v1.clone());
    match m1425 {
        Adt110::C0 => {
            f319(Some(v0), f414(v1))
        }
        Adt110::C1(v2) => {
            let m1426 = f133(v2);
            match m1426 {
                None => {
                    f319(Some(v0), Adt107::C4)
                }
                Some(v3) => {
                    f579(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f499(v0: u64) -> R<u64> {
    let a1429 = {
        let a1427 = v0;
        let a1428 = 3u64;
        nat_mul(a1427, a1428)?
    };
    let a1430 = 1u64;
    nat_add(a1429, a1430)
}

pub fn f500(v0: u64, v1: u64, v2: u64) -> bool {
    let a1431 = v1;
    let a1432 = v0;
    if nat_le(a1431, a1432) {
        let a1433 = v0;
        let a1434 = v2;
        nat_le(a1433, a1434)
    } else {
        false
    }
}

pub fn f501(v0: Adt130) -> R<Adt151> {
    if f580(match v0.clone() {
        Adt130::C0(_, h1435, _, _, _, _, _) => {
            h1435
        }
    }) {
        let v1: i64 = {
            let a1441 = if match v0.clone() {
                Adt130::C0(_, _, _, _, _, h1436, _) => {
                    h1436
                }
            } {
                let a1438 = match v0.clone() {
                    Adt130::C0(_, _, _, _, _, _, h1437) => {
                        h1437
                    }
                };
                int_neg(a1438)?
            } else {
                match v0.clone() {
                    Adt130::C0(_, _, _, _, _, _, h1439) => {
                        h1439
                    }
                }
            };
            let a1442 = match v0.clone() {
                Adt130::C0(_, _, _, _, h1440, _, _) => {
                    h1440
                }
            };
            int_sub(a1441, a1442)?
        };
        let m1445 = {
            let a1443 = v1;
            let a1444 = -9999i64;
            compare(a1443, a1444)
        };
        if if match m1445 {
            Ordering::Less => {
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
            let m1448 = {
                let a1446 = 9999i64;
                let a1447 = v1;
                compare(a1446, a1447)
            };
            match m1448 {
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
            Ok(Adt151::C1)
        } else {
            Ok(Adt151::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt130::C0(h1449, _, _, _, _, _, _) => {
                    h1449
                }
            } {
                let a1451 = match v0 {
                    Adt130::C0(_, _, h1450, _, _, _, _) => {
                        h1450
                    }
                };
                int_neg(a1451)?
            } else {
                match v0 {
                    Adt130::C0(_, _, h1452, _, _, _, _) => {
                        h1452
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt151::C0)
    }
}

pub fn f502(v0: Adt6, v1: List<Adt132>, v2: u64) -> Adt127 {
    let m1453 = v1.clone();
    match m1453.uncons() {
        None => {
            Adt127::C0(Adt128::C9(v0), List::<Adt132>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1454 = v3;
            match m1454 {
                Adt132::C0(v5) => {
                    Adt127::C0(Adt128::C2, List::cons(Adt132::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt132::C1(v6, v7) => {
                    let m1455 = v7;
                    match m1455 {
                        None => {
                            Adt127::C0(Adt128::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt127::C0(Adt128::C2, List::cons(Adt132::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f503(v0: u64) -> bool {
    let a1456 = v0;
    let a1457 = 32u64;
    if nat_eq(a1456, a1457) {
        true
    } else {
        let a1458 = v0;
        let a1459 = 10u64;
        if nat_eq(a1458, a1459) {
            true
        } else {
            let a1460 = v0;
            let a1461 = 13u64;
            if nat_eq(a1460, a1461) {
                true
            } else {
                let a1462 = v0;
                let a1463 = 9u64;
                nat_eq(a1462, a1463)
            }
        }
    }
}

pub fn f504(v0: Adt127, v1: u64, v2: u64) -> R<Adt145> {
    let a1464 = v2;
    let a1465 = 34u64;
    if nat_eq(a1464, a1465) {
        Ok(f508(v0, Adt128::C6(Adt129::C0, List::<u64>::nil(), false)))
    } else {
        let a1466 = v2;
        let a1467 = 123u64;
        if nat_eq(a1466, a1467) {
            f581(v0, Adt132::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt128::C3, v1)
        } else {
            let a1468 = v2;
            let a1469 = 91u64;
            if nat_eq(a1468, a1469) {
                f581(v0, Adt132::C0(List::<Adt6>::nil()), Adt128::C1, v1)
            } else {
                let a1470 = v2;
                let a1471 = 116u64;
                if nat_eq(a1470, a1471) {
                    Ok(f508(v0, Adt128::C7(0u64, 1u64)))
                } else {
                    let a1472 = v2;
                    let a1473 = 102u64;
                    if nat_eq(a1472, a1473) {
                        Ok(f508(v0, Adt128::C7(1u64, 1u64)))
                    } else {
                        let a1474 = v2;
                        let a1475 = 110u64;
                        if nat_eq(a1474, a1475) {
                            Ok(f508(v0, Adt128::C7(2u64, 1u64)))
                        } else {
                            let a1476 = v2;
                            let a1477 = 45u64;
                            if nat_eq(a1476, a1477) {
                                Ok(f508(v0, f582(true, Adt131::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1478 = v2;
                                let a1479 = 48u64;
                                if nat_eq(a1478, a1479) {
                                    Ok(f508(v0, f582(false, Adt131::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f500(v2, 49u64, 57u64) {
                                        Ok(f508(v0, f582(false, Adt131::C2, f583({
                                            let a1480 = v2;
                                            let a1481 = 48u64;
                                            nat_sub(a1480, a1481)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f509(Adt105::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f505(v0: Adt127, v1: u64) -> Adt145 {
    let m1483 = match v0.clone() {
        Adt127::C0(_, h1482, _) => {
            h1482
        }
    };
    match m1483.uncons() {
        None => {
            f509(Adt105::C2(v1))
        }
        Some((v2, v3)) => {
            let m1484 = v2;
            match m1484 {
                Adt132::C0(v4) => {
                    Adt145::C1(f502(Adt6::C4(f445(v4, List::<Adt6>::nil())), v3, {
                        let a1486 = match v0 {
                            Adt127::C0(_, _, h1485) => {
                                h1485
                            }
                        };
                        let a1487 = 1u64;
                        nat_sub(a1486, a1487)
                    }))
                }
                Adt132::C1(_, _) => {
                    f509(Adt105::C2(v1))
                }
            }
        }
    }
}

pub fn f506(v0: Adt127, v1: u64, v2: u64) -> Adt145 {
    let m1489 = match v0.clone() {
        Adt127::C0(h1488, _, _) => {
            h1488
        }
    };
    match m1489 {
        Adt128::C0 => {
            f509(Adt105::C2(v1))
        }
        Adt128::C1 => {
            f509(Adt105::C2(v1))
        }
        Adt128::C2 => {
            if f503(v2) {
                Adt145::C1(v0)
            } else {
                let a1490 = v2;
                let a1491 = 44u64;
                if nat_eq(a1490, a1491) {
                    f584(v0, v1)
                } else {
                    let a1492 = v2;
                    let a1493 = 93u64;
                    if nat_eq(a1492, a1493) {
                        f505(v0, v1)
                    } else {
                        let a1494 = v2;
                        let a1495 = 125u64;
                        if nat_eq(a1494, a1495) {
                            f507(v0, v1)
                        } else {
                            f509(Adt105::C2(v1))
                        }
                    }
                }
            }
        }
        Adt128::C3 => {
            f509(Adt105::C2(v1))
        }
        Adt128::C4 => {
            f509(Adt105::C2(v1))
        }
        Adt128::C5 => {
            f509(Adt105::C2(v1))
        }
        Adt128::C6(_, _, _) => {
            f509(Adt105::C2(v1))
        }
        Adt128::C7(_, _) => {
            f509(Adt105::C2(v1))
        }
        Adt128::C8(_) => {
            f509(Adt105::C2(v1))
        }
        Adt128::C9(_) => {
            if f503(v2) {
                Adt145::C1(v0)
            } else {
                f509(Adt105::C10(v1))
            }
        }
    }
}

pub fn f507(v0: Adt127, v1: u64) -> Adt145 {
    let m1497 = match v0.clone() {
        Adt127::C0(_, h1496, _) => {
            h1496
        }
    };
    match m1497.uncons() {
        None => {
            f509(Adt105::C2(v1))
        }
        Some((v2, v3)) => {
            let m1498 = v2;
            match m1498 {
                Adt132::C0(_) => {
                    f509(Adt105::C2(v1))
                }
                Adt132::C1(v5, _) => {
                    Adt145::C1(f502(Adt6::C5(f535(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1500 = match v0 {
                            Adt127::C0(_, _, h1499) => {
                                h1499
                            }
                        };
                        let a1501 = 1u64;
                        nat_sub(a1500, a1501)
                    }))
                }
            }
        }
    }
}

pub fn f508(v0: Adt127, v1: Adt128) -> Adt145 {
    Adt145::C1(f585(v0, v1))
}

pub fn f509(v0: Adt105) -> Adt145 {
    Adt145::C0(v0)
}

pub fn f510(v0: Adt127, v1: u64, v2: u64, v3: Adt129, v4: List<u64>, v5: bool) -> R<Adt145> {
    let m1502 = v3;
    match m1502 {
        Adt129::C0 => {
            let a1503 = v2;
            let a1504 = 34u64;
            if nat_eq(a1503, a1504) {
                Ok(f586(v0, v1, v4, v5))
            } else {
                let a1505 = v2;
                let a1506 = 92u64;
                if nat_eq(a1505, a1506) {
                    Ok(f508(v0, Adt128::C6(Adt129::C1, v4, v5)))
                } else {
                    let a1507 = v2;
                    let a1508 = 32u64;
                    if nat_lt(a1507, a1508) {
                        Ok(f509(Adt105::C4(v1)))
                    } else {
                        Ok(f508(v0, Adt128::C6(Adt129::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt129::C1 => {
            let a1509 = v2;
            let a1510 = 117u64;
            if nat_eq(a1509, a1510) {
                Ok(f508(v0, Adt128::C6(Adt129::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1511 = f587(v2);
                let a1512 = 0u64;
                if nat_eq(a1511, a1512) {
                    Ok(f509(Adt105::C5(v1)))
                } else {
                    Ok(f508(v0, Adt128::C6(Adt129::C0, List::cons(f587(v2), v4), v5)))
                }
            }
        }
        Adt129::C2(v6, v7, v8) => {
            let a1513 = 15u64;
            let a1514 = f588(v2);
            if nat_lt(a1513, a1514) {
                Ok(f509(Adt105::C5(v1)))
            } else {
                let a1517 = {
                    let a1515 = v6;
                    let a1516 = 1u64;
                    nat_add(a1515, a1516)?
                };
                let a1518 = 4u64;
                if nat_eq(a1517, a1518) {
                    f589(v0, v1, {
                        let a1521 = {
                            let a1519 = v7;
                            let a1520 = 16u64;
                            nat_mul(a1519, a1520)?
                        };
                        let a1522 = f588(v2);
                        nat_add(a1521, a1522)?
                    }, v8, v4, v5)
                } else {
                    Ok(f508(v0, Adt128::C6(Adt129::C2({
                        let a1523 = v6;
                        let a1524 = 1u64;
                        nat_add(a1523, a1524)?
                    }, {
                        let a1527 = {
                            let a1525 = v7;
                            let a1526 = 16u64;
                            nat_mul(a1525, a1526)?
                        };
                        let a1528 = f588(v2);
                        nat_add(a1527, a1528)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt129::C3(v9) => {
            let a1529 = v2;
            let a1530 = 92u64;
            if nat_eq(a1529, a1530) {
                Ok(f508(v0, Adt128::C6(Adt129::C4(v9), v4, v5)))
            } else {
                Ok(f509(Adt105::C6(v1)))
            }
        }
        Adt129::C4(v10) => {
            let a1531 = v2;
            let a1532 = 117u64;
            if nat_eq(a1531, a1532) {
                Ok(f508(v0, Adt128::C6(Adt129::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f509(Adt105::C6(v1)))
            }
        }
    }
}

pub fn f511(v0: Adt127, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt145> {
    let a1533 = v2;
    let a1534 = f590(v3, v4);
    if nat_eq(a1533, a1534) {
        let a1538 = {
            let a1535 = v4;
            let a1536 = 1u64;
            nat_add(a1535, a1536)?
        };
        let a1539 = {
            let a1537 = f591(v3);
            length_list(a1537)
        };
        if nat_eq(a1538, a1539) {
            Ok(f592(v0, f593(v3)))
        } else {
            Ok(f508(v0, Adt128::C7(v3, {
                let a1540 = v4;
                let a1541 = 1u64;
                nat_add(a1540, a1541)?
            })))
        }
    } else {
        Ok(f509(Adt105::C2(v1)))
    }
}

pub fn f512(v0: Adt127, v1: u64, v2: u64, v3: Adt130) -> R<Adt145> {
    let m1543 = match v3.clone() {
        Adt130::C0(_, h1542, _, _, _, _, _) => {
            h1542
        }
    };
    match m1543 {
        Adt131::C0 => {
            let a1544 = v2;
            let a1545 = 48u64;
            if nat_eq(a1544, a1545) {
                Ok(f594(v0, v3, Adt131::C1))
            } else {
                if f500(v2, 49u64, 57u64) {
                    f595(v0, v1, v2, v3, false)
                } else {
                    Ok(f509(Adt105::C7(v1)))
                }
            }
        }
        Adt131::C1 => {
            let a1546 = v2;
            let a1547 = 46u64;
            if nat_eq(a1546, a1547) {
                Ok(f594(v0, v3, Adt131::C3))
            } else {
                if f596(v2) {
                    Ok(f594(v0, v3, Adt131::C5))
                } else {
                    if f597(v2) {
                        Ok(f509(Adt105::C7(v1)))
                    } else {
                        f598(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt131::C2 => {
            if f597(v2) {
                f595(v0, v1, v2, v3, false)
            } else {
                let a1548 = v2;
                let a1549 = 46u64;
                if nat_eq(a1548, a1549) {
                    Ok(f594(v0, v3, Adt131::C3))
                } else {
                    if f596(v2) {
                        Ok(f594(v0, v3, Adt131::C5))
                    } else {
                        f598(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt131::C3 => {
            if f597(v2) {
                f595(v0, v1, v2, v3, true)
            } else {
                Ok(f509(Adt105::C7(v1)))
            }
        }
        Adt131::C4 => {
            if f597(v2) {
                f595(v0, v1, v2, v3, true)
            } else {
                if f596(v2) {
                    Ok(f594(v0, v3, Adt131::C5))
                } else {
                    f598(v0, v1, v2, v3)
                }
            }
        }
        Adt131::C5 => {
            let a1550 = v2;
            let a1551 = 43u64;
            if nat_eq(a1550, a1551) {
                Ok(f508(v0, f582(match v3.clone() {
                    Adt130::C0(h1552, _, _, _, _, _, _) => {
                        h1552
                    }
                }, Adt131::C6, match v3.clone() {
                    Adt130::C0(_, _, h1553, _, _, _, _) => {
                        h1553
                    }
                }, match v3.clone() {
                    Adt130::C0(_, _, _, h1554, _, _, _) => {
                        h1554
                    }
                }, match v3 {
                    Adt130::C0(_, _, _, _, h1555, _, _) => {
                        h1555
                    }
                }, false, 0i64)))
            } else {
                let a1556 = v2;
                let a1557 = 45u64;
                if nat_eq(a1556, a1557) {
                    Ok(f508(v0, f582(match v3.clone() {
                        Adt130::C0(h1558, _, _, _, _, _, _) => {
                            h1558
                        }
                    }, Adt131::C6, match v3.clone() {
                        Adt130::C0(_, _, h1559, _, _, _, _) => {
                            h1559
                        }
                    }, match v3.clone() {
                        Adt130::C0(_, _, _, h1560, _, _, _) => {
                            h1560
                        }
                    }, match v3 {
                        Adt130::C0(_, _, _, _, h1561, _, _) => {
                            h1561
                        }
                    }, true, 0i64)))
                } else {
                    if f597(v2) {
                        f599(v0, v1, v2, Adt130::C0(match v3.clone() {
                            Adt130::C0(h1562, _, _, _, _, _, _) => {
                                h1562
                            }
                        }, match v3.clone() {
                            Adt130::C0(_, h1563, _, _, _, _, _) => {
                                h1563
                            }
                        }, match v3.clone() {
                            Adt130::C0(_, _, h1564, _, _, _, _) => {
                                h1564
                            }
                        }, match v3.clone() {
                            Adt130::C0(_, _, _, h1565, _, _, _) => {
                                h1565
                            }
                        }, match v3 {
                            Adt130::C0(_, _, _, _, h1566, _, _) => {
                                h1566
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f509(Adt105::C7(v1)))
                    }
                }
            }
        }
        Adt131::C6 => {
            if f597(v2) {
                f599(v0, v1, v2, v3.clone(), match v3 {
                    Adt130::C0(_, _, _, _, _, h1567, _) => {
                        h1567
                    }
                })
            } else {
                Ok(f509(Adt105::C7(v1)))
            }
        }
        Adt131::C7 => {
            if f597(v2) {
                f599(v0, v1, v2, v3.clone(), match v3 {
                    Adt130::C0(_, _, _, _, _, h1568, _) => {
                        h1568
                    }
                })
            } else {
                f598(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f513(v0: u64) -> bool {
    let a1569 = v0;
    let a1570 = 9u64;
    if nat_eq(a1569, a1570) {
        true
    } else {
        let a1571 = v0;
        let a1572 = 32u64;
        nat_eq(a1571, a1572)
    }
}

pub fn f514(v0: u64) -> bool {
    let a1573 = 47u64;
    let a1574 = v0;
    if nat_lt(a1573, a1574) {
        let a1575 = v0;
        let a1576 = 58u64;
        nat_lt(a1575, a1576)
    } else {
        false
    }
}

pub fn f515(v0: u64) -> bool {
    let a1577 = 48u64;
    let a1578 = v0;
    if if nat_le(a1577, a1578) {
        let a1579 = v0;
        let a1580 = 57u64;
        nat_le(a1579, a1580)
    } else {
        false
    } {
        true
    } else {
        let a1581 = 65u64;
        let a1582 = v0;
        if if nat_le(a1581, a1582) {
            let a1583 = v0;
            let a1584 = 90u64;
            nat_le(a1583, a1584)
        } else {
            false
        } {
            true
        } else {
            let a1585 = 97u64;
            let a1586 = v0;
            if if nat_le(a1585, a1586) {
                let a1587 = v0;
                let a1588 = 122u64;
                nat_le(a1587, a1588)
            } else {
                false
            } {
                true
            } else {
                let a1589 = v0;
                let a1590 = 33u64;
                if nat_eq(a1589, a1590) {
                    true
                } else {
                    let a1591 = v0;
                    let a1592 = 35u64;
                    if nat_eq(a1591, a1592) {
                        true
                    } else {
                        let a1593 = v0;
                        let a1594 = 36u64;
                        if nat_eq(a1593, a1594) {
                            true
                        } else {
                            let a1595 = v0;
                            let a1596 = 37u64;
                            if nat_eq(a1595, a1596) {
                                true
                            } else {
                                let a1597 = v0;
                                let a1598 = 38u64;
                                if nat_eq(a1597, a1598) {
                                    true
                                } else {
                                    let a1599 = v0;
                                    let a1600 = 39u64;
                                    if nat_eq(a1599, a1600) {
                                        true
                                    } else {
                                        let a1601 = v0;
                                        let a1602 = 42u64;
                                        if nat_eq(a1601, a1602) {
                                            true
                                        } else {
                                            let a1603 = v0;
                                            let a1604 = 43u64;
                                            if nat_eq(a1603, a1604) {
                                                true
                                            } else {
                                                let a1605 = v0;
                                                let a1606 = 45u64;
                                                if nat_eq(a1605, a1606) {
                                                    true
                                                } else {
                                                    let a1607 = v0;
                                                    let a1608 = 46u64;
                                                    if nat_eq(a1607, a1608) {
                                                        true
                                                    } else {
                                                        let a1609 = v0;
                                                        let a1610 = 94u64;
                                                        if nat_eq(a1609, a1610) {
                                                            true
                                                        } else {
                                                            let a1611 = v0;
                                                            let a1612 = 95u64;
                                                            if nat_eq(a1611, a1612) {
                                                                true
                                                            } else {
                                                                let a1613 = v0;
                                                                let a1614 = 96u64;
                                                                if nat_eq(a1613, a1614) {
                                                                    true
                                                                } else {
                                                                    let a1615 = v0;
                                                                    let a1616 = 124u64;
                                                                    if nat_eq(a1615, a1616) {
                                                                        true
                                                                    } else {
                                                                        let a1617 = v0;
                                                                        let a1618 = 126u64;
                                                                        nat_eq(a1617, a1618)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f516(v0: u64) -> bool {
    let a1619 = 48u64;
    let a1620 = v0;
    if if nat_le(a1619, a1620) {
        let a1621 = v0;
        let a1622 = 57u64;
        nat_le(a1621, a1622)
    } else {
        false
    } {
        true
    } else {
        let a1623 = 65u64;
        let a1624 = v0;
        if if nat_le(a1623, a1624) {
            let a1625 = v0;
            let a1626 = 90u64;
            nat_le(a1625, a1626)
        } else {
            false
        } {
            true
        } else {
            let a1627 = 97u64;
            let a1628 = v0;
            if if nat_le(a1627, a1628) {
                let a1629 = v0;
                let a1630 = 122u64;
                nat_le(a1629, a1630)
            } else {
                false
            } {
                true
            } else {
                let a1631 = v0;
                let a1632 = 95u64;
                if nat_eq(a1631, a1632) {
                    true
                } else {
                    let a1633 = v0;
                    let a1634 = 45u64;
                    if nat_eq(a1633, a1634) {
                        true
                    } else {
                        let a1635 = v0;
                        let a1636 = 46u64;
                        nat_eq(a1635, a1636)
                    }
                }
            }
        }
    }
}

pub fn f517() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f518(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt133, v6: List<Adt113>) -> R<Result<(List<Adt113>, Adt133), Adt98>> {
    let m1637 = f600(v0.clone(), v1, v2.clone());
    match m1637 {
        None => {
            let m1638 = f601(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1638 {
                Adt156::C0(v8) => {
                    Ok(Err::<(List<Adt113>, Adt133), Adt98>(v8))
                }
                Adt156::C1(v9) => {
                    Ok(Ok::<(List<Adt113>, Adt133), Adt98>((f602(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt113>, Adt133), Adt98>(v7))
        }
    }
}

pub fn f519(v0: Str) -> R<bool> {
    let m1640 = f574(f198({
        let a1639 = v0;
        utf8_encode(a1639)
    })?)?;
    match m1640 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f520(v0: Adt38) -> bool {
    let m1642 = match v0 {
        Adt38::C0(_, _, h1641) => {
            h1641
        }
    };
    match m1642 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f521(v0: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1644 = match v0.clone() {
        Adt50::C0(_, h1643) => {
            h1643
        }
    };
    match m1644.uncons() {
        None => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(match v0 {
                Adt50::C0(h1645, _) => {
                    h1645
                }
            })))
        }
        Some((_, _)) => {
            if f603(match v0.clone() {
                Adt50::C0(_, h1646) => {
                    h1646
                }
            })? {
                Ok(Ok::<Adt33, Adt90>(Adt33::C3(v0)))
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f522(v0: Adt5, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1650 = {
        let m1649 = match v1.clone() {
            Adt50::C0(h1648, _) => {
                h1648
            }
        };
        match m1649 {
            Adt38::C0(_, h1647, _) => {
                h1647
            }
        }
    };
    match m1650 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1653 = f230(Adt6::C5(match v0.clone() {
                Adt5::C0(h1651) => {
                    h1651
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1652) => {
                    h1652
                }
            }), v2)?;
            match m1653 {
                Adt114::C0 => {
                    f521(v1)
                }
                Adt114::C1(_, _) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt114::C2(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt114::C3(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt114::C4 => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f523() -> i64 {
    60000i64
}

pub fn f524() -> i64 {
    1000i64
}

pub fn f525(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt115>) -> R<Adt146> {
    let m1654 = f604(v1.clone(), v2.clone())?;
    match m1654 {
        Adt146::C0(v4) => {
            Ok(Adt146::C0(v4))
        }
        Adt146::C1(v5) => {
            Ok(Adt146::C1(v5))
        }
        Adt146::C2(v6) => {
            Ok(Adt146::C2(v6))
        }
        Adt146::C3(_) => {
            f605(v0, v1, v2, v3)
        }
    }
}

pub fn f526(v0: List<Adt6>, v1: Adt6, v2: List<Adt115>, v3: u64, v4: List<Adt117>) -> Adt134 {
    let m1655 = v0;
    match m1655.uncons() {
        None => {
            let a1656 = v3;
            let a1657 = 1u64;
            if nat_eq(a1656, a1657) {
                Adt134::C0(Adt116::C0, v4)
            } else {
                Adt134::C0(Adt116::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt134::C0(Adt116::C0, List::cons(Adt117::C0(v5, v1.clone(), v2.clone()), List::cons(Adt117::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f527(v0: Option<Adt6>, v1: Adt6, v2: List<Adt115>, v3: List<Adt117>) -> Adt134 {
    let m1658 = v0;
    match m1658 {
        None => {
            Adt134::C0(Adt116::C0, v3)
        }
        Some(v4) => {
            Adt134::C0(Adt116::C0, List::cons(Adt117::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f528(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt115>) -> Adt146 {
    let m1659 = f347(Str::lit("anyOf"), v0);
    match m1659 {
        Adt110::C0 => {
            Adt146::C3(List::<Adt117>::nil())
        }
        Adt110::C1(v3) => {
            let m1660 = f294(v3);
            match m1660 {
                None => {
                    Adt146::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1661 = v4;
                    match m1661.uncons() {
                        None => {
                            Adt146::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt146::C3(List::cons(Adt117::C0(v5, v1.clone(), v2.clone()), List::cons(Adt117::C1(v6, v1, v2), List::<Adt117>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f529(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt115>) -> Adt146 {
    let m1662 = f347(Str::lit("oneOf"), v0);
    match m1662 {
        Adt110::C0 => {
            Adt146::C3(List::<Adt117>::nil())
        }
        Adt110::C1(v3) => {
            let m1663 = f294(v3);
            match m1663 {
                None => {
                    Adt146::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1664 = v4;
                    match m1664.uncons() {
                        None => {
                            Adt146::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt146::C3(List::cons(Adt117::C0(v5, v1.clone(), v2.clone()), List::cons(Adt117::C2(v6, v1, v2, 0u64), List::<Adt117>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f530(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt115>) -> Adt146 {
    let m1665 = f347(Str::lit("not"), v0);
    match m1665 {
        Adt110::C0 => {
            Adt146::C3(List::<Adt117>::nil())
        }
        Adt110::C1(v3) => {
            Adt146::C3(List::cons(Adt117::C0(v3, v1, v2.clone()), List::cons(Adt117::C3(v2), List::<Adt117>::nil())))
        }
    }
}

pub fn f531(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt115>) -> Adt146 {
    let m1666 = f347(Str::lit("if"), v0.clone());
    match m1666 {
        Adt110::C0 => {
            Adt146::C3(List::<Adt117>::nil())
        }
        Adt110::C1(v3) => {
            Adt146::C3(List::cons(Adt117::C0(v3, v1.clone(), v2.clone()), List::cons(Adt117::C4(f606(Str::lit("then"), v0.clone()), f606(Str::lit("else"), v0), v1, v2), List::<Adt117>::nil())))
        }
    }
}

pub fn f532(v0: List<Adt6>, v1: Adt6, v2: List<Adt115>, v3: List<Adt117>) -> Adt134 {
    let m1667 = v0;
    match m1667.uncons() {
        None => {
            Adt134::C0(Adt116::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt134::C0(Adt116::C0, List::cons(Adt117::C0(v4, v1.clone(), v2.clone()), List::cons(Adt117::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f533(v0: Adt115) -> R<Str> {
    let m1668 = v0;
    match m1668 {
        Adt115::C0(v1) => {
            f607(v1)
        }
        Adt115::C1(v2) => {
            let a1669 = f354(v2, 0i64)?;
            Ok(format_i64(a1669))
        }
    }
}

pub fn f534(v0: Str) -> R<Str> {
    let m1672 = {
        let a1671 = f449(f200(), f401(f608(f198({
            let a1670 = v0;
            utf8_encode(a1670)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1671)
    };
    match m1672 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f535(p1675: List<(Str, Adt6)>, p1676: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1674 = (p1675, p1676);
    {
        let mut m1674 = m1674;
        loop {
            let step = {
                let (v0, v1) = m1674;
                let m1673 = v0;
                match m1673.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
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

pub fn f536(v0: Adt39) -> R<Option<Adt6>> {
    let m1677 = v0;
    match m1677 {
        Adt39::C0(v1) => {
            Ok(Some(f609(v1)))
        }
        Adt39::C1(v2) => {
            Ok(Some(f610(Str::lit("image"), match v2.clone() {
                Adt43::C0(h1678, _, _, _) => {
                    h1678
                }
            }, match v2.clone() {
                Adt43::C0(_, h1679, _, _) => {
                    h1679
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h1680, _) => {
                    h1680
                }
            }, match v2 {
                Adt43::C0(_, _, _, h1681) => {
                    h1681
                }
            })))
        }
        Adt39::C2(v3) => {
            Ok(Some(f610(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h1682, _, _, _) => {
                    h1682
                }
            }, match v3.clone() {
                Adt44::C0(_, h1683, _, _) => {
                    h1683
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h1684, _) => {
                    h1684
                }
            }, match v3 {
                Adt44::C0(_, _, _, h1685) => {
                    h1685
                }
            })))
        }
        Adt39::C3(v4) => {
            Ok(Some(f611(v4)?))
        }
        Adt39::C4(v5) => {
            Ok(Some(f612(v5)))
        }
    }
}

pub fn f537(p1691: List<u64>, p1692: List<u64>) -> Option<List<u64>> {
    let m1690 = (p1691, p1692);
    {
        let mut m1690 = m1690;
        loop {
            let step = {
                let (v0, v1) = m1690;
                let m1686 = v0;
                match m1686.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1687 = v1;
                        match m1687.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1688 = v2;
                                let a1689 = v4;
                                if nat_eq(a1688, a1689) {
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
                Ok(next) => { m1690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: Str) -> R<List<u64>> {
    f198({
        let a1693 = v0;
        utf8_encode(a1693)
    })
}

pub fn f539() -> Str {
    Str::lit("test://template/")
}

pub fn f540(v0: List<u64>) -> List<u64> {
    f401(v0, List::<u64>::nil())
}

pub fn f541() -> Str {
    Str::lit("data")
}

pub fn f542(v0: List<u64>) -> bool {
    let m1694 = v0;
    match m1694.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f543(p1699: List<u64>) -> bool {
    let m1698 = p1699;
    {
        let mut m1698 = m1698;
        loop {
            let step = {
                let v0 = m1698;
                let m1695 = v0;
                match m1695.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1696 = v1;
                        let a1697 = 47u64;
                        if nat_eq(a1696, a1697) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
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

pub fn f544(p1704: Bytes, p1705: List<Bytes>) -> Bytes {
    let m1703 = (p1704, p1705);
    {
        let mut m1703 = m1703;
        loop {
            let step = {
                let (v0, v1) = m1703;
                let m1700 = v1;
                match m1700.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1701 = v2;
                            let a1702 = v0;
                            append_bytes(a1701, a1702)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f545(p1708: List<Bytes>, p1709: List<u64>) -> List<Bytes> {
    let m1707 = (p1708, p1709);
    {
        let mut m1707 = m1707;
        loop {
            let step = {
                let (v0, v1) = m1707;
                let m1706 = v1;
                match m1706.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f613(f614(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(v0: Adt110, v1: Str) -> bool {
    let m1710 = v0;
    match m1710 {
        Adt110::C0 => {
            false
        }
        Adt110::C1(v2) => {
            let m1711 = f133(v2);
            match m1711 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1712 = v3;
                    let a1713 = v1;
                    equal(a1712, a1713)
                }
            }
        }
    }
}

pub fn f547(p1719: List<u64>, p1720: List<u64>) -> bool {
    let m1718 = (p1719, p1720);
    {
        let mut m1718 = m1718;
        loop {
            let step = {
                let (v0, v1) = m1718;
                let m1714 = v0;
                match m1714.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1715 = v1;
                        match m1715.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1716 = v2;
                                let a1717 = v4;
                                if nat_eq(a1716, a1717) {
                                    Ok::<(List<u64>, List<u64>), bool>((v3, v5))
                                } else {
                                    Err::<(List<u64>, List<u64>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: Adt5) -> Option<Adt64> {
    let m1721 = f549(Str::lit("taskId"), v0.clone());
    match m1721 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m1722 = f549(Str::lit("createdAt"), v0.clone());
            match m1722 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let m1723 = f549(Str::lit("lastUpdatedAt"), v0.clone());
                    match m1723 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v3) => {
                            Some(Adt64::C0(v1, f549(Str::lit("statusMessage"), v0.clone()), v2, v3, f463(Str::lit("ttlMs"), v0.clone()), f463(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f549(v0: Str, v1: Adt5) -> Option<Str> {
    let m1724 = f234(v0, v1);
    match m1724 {
        Adt110::C0 => {
            None::<Str>
        }
        Adt110::C1(v2) => {
            f133(v2)
        }
    }
}

pub fn f550(v0: Str, v1: Adt5) -> Option<List<(Str, Adt65)>> {
    let m1725 = f461(v0, v1);
    match m1725 {
        None => {
            None::<List<(Str, Adt65)>>
        }
        Some(v2) => {
            f615(match v2 {
                Adt5::C0(h1726) => {
                    h1726
                }
            }, List::<(Str, Adt65)>::nil())
        }
    }
}

pub fn f551(v0: Adt5) -> Option<Adt81> {
    let m1727 = f463(Str::lit("code"), v0.clone());
    match m1727 {
        None => {
            None::<Adt81>
        }
        Some(v1) => {
            let m1728 = f549(Str::lit("message"), v0.clone());
            match m1728 {
                None => {
                    None::<Adt81>
                }
                Some(v2) => {
                    Some(Adt81::C0(v1, v2, {
                        let m1729 = f234(Str::lit("data"), v0);
                        match m1729 {
                            Adt110::C0 => {
                                None::<Adt6>
                            }
                            Adt110::C1(v3) => {
                                Some(v3)
                            }
                        }
                    }))
                }
            }
        }
    }
}

pub fn f552(v0: Adt5) -> Option<Adt87> {
    let m1730 = f461(Str::lit("completes"), v0.clone());
    match m1730 {
        None => {
            let m1731 = f461(Str::lit("fails"), v0);
            match m1731 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m1732 = f551(v2);
                    match m1732 {
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

pub fn f553(v0: u64, v1: Adt6, v2: List<Adt119>, v3: List<u64>) -> R<Adt148> {
    let m1733 = v1;
    match m1733 {
        Adt6::C0 => {
            Ok(Adt148::C1(v2, f401(f616(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt148::C1(v2, f401(if v4 {
                f616(0u64)
            } else {
                f616(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f617(v5.clone())? {
                Ok(Adt148::C1(v2, f401(f618(v5)?, v3)))
            } else {
                Ok(Adt148::C0(Adt105::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt148::C1(v2, f401(f554(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1734 = v0;
            let a1735 = f619();
            if nat_lt(a1734, a1735) {
                Ok(Adt148::C1(f620(91u64, Adt119::C2({
                    let a1736 = v0;
                    let a1737 = 1u64;
                    nat_add(a1736, a1737)?
                }, v7), v2), v3))
            } else {
                Ok(Adt148::C0(Adt105::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1738 = v0;
            let a1739 = f619();
            if nat_lt(a1738, a1739) {
                Ok(Adt148::C1(f620(123u64, Adt119::C4({
                    let a1740 = v0;
                    let a1741 = 1u64;
                    nat_add(a1740, a1741)?
                }, v8), v2), v3))
            } else {
                Ok(Adt148::C0(Adt105::C12))
            }
        }
    }
}

pub fn f554(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1743 = f401(f621(f198({
            let a1742 = v0;
            utf8_encode(a1742)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1744 = List::cons(34u64, List::<u64>::nil());
        append_list(a1743, a1744)
    }))
}

pub fn f555(v0: Adt33) -> R<Option<List<(Str, Adt6)>>> {
    let m1745 = v0;
    match m1745 {
        Adt33::C0(v1) => {
            Ok(Some(f622(v1)?))
        }
        Adt33::C1(v2) => {
            Ok(Some(f623(v2)?))
        }
        Adt33::C2(v3) => {
            f236(f237(v3)?)
        }
        Adt33::C3(v4) => {
            f236(f237(match v4 {
                Adt50::C0(h1746, _) => {
                    h1746
                }
            })?)
        }
        Adt33::C4(v5) => {
            Ok(Some(f624(v5)?))
        }
        Adt33::C5(v6) => {
            Ok(Some(f625(v6)?))
        }
        Adt33::C6(v7) => {
            Ok(Some(f626(v7)?))
        }
        Adt33::C7(v8) => {
            Ok(Some(f627(v8)?))
        }
        Adt33::C8(v9) => {
            f236(f628(v9)?)
        }
        Adt33::C9(v10) => {
            Ok(Some(f629(v10)?))
        }
        Adt33::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt33::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt33::C12(v12) => {
            f236(f630(v12)?)
        }
        Adt33::C13(v13) => {
            f236(f631({
                let m1749 = match v13 {
                    Adt83::C0(_, _, h1748) => {
                        h1748
                    }
                };
                match m1749 {
                    Adt85::C0(h1747, _, _) => {
                        h1747
                    }
                }
            }))
        }
        Adt33::C14(v14) => {
            Ok(f632(v14))
        }
    }
}

pub fn f556(v0: Adt138) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f633(f634(match v0.clone() {
        Adt138::C0(h1750, _, _) => {
            h1750
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f635({
        let m1753 = match v0 {
            Adt138::C0(_, h1752, _) => {
                h1752
            }
        };
        match m1753 {
            Adt139::C0(h1751, _) => {
                h1751
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f557(v0: Option<Option<List<u64>>>, v1: Adt95) -> Adt95 {
    let m1754 = v0;
    match m1754 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt95::C0(match v1.clone() {
                Adt95::C0(h1755, _, _, _, _) => {
                    h1755
                }
            }, match v1.clone() {
                Adt95::C0(_, h1756, _, _, _) => {
                    h1756
                }
            }, match v1.clone() {
                Adt95::C0(_, _, h1757, _, _) => {
                    h1757
                }
            }, match v1 {
                Adt95::C0(_, _, _, h1758, _) => {
                    h1758
                }
            }, v2)
        }
    }
}

pub fn f558(v0: Adt14, v1: Adt88) -> R<Option<Option<List<u64>>>> {
    let m1760 = f189(match v0.clone() {
        Adt14::C0(_, _, _, h1759) => {
            h1759
        }
    })?;
    match m1760 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1763 = f636(f637(match v1 {
                Adt88::C0(h1761, _, _) => {
                    h1761
                }
            }, v2, match v0 {
                Adt14::C0(h1762, _, _, _) => {
                    h1762
                }
            })?)?;
            match m1763 {
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

pub fn f559(v0: bool, v1: Adt95, v2: List<List<u64>>) -> Adt95 {
    if f638(v0, match v1.clone() {
        Adt95::C0(h1764, _, _, _, _) => {
            h1764
        }
    }, v2.clone()) {
        f639(match v1.clone() {
            Adt95::C0(h1765, _, _, _, _) => {
                h1765
            }
        }, {
            let a1767 = v2;
            let a1768 = List::cons(match v1 {
                Adt95::C0(_, _, _, h1766, _) => {
                    h1766
                }
            }, List::<List<u64>>::nil());
            append_list(a1767, a1768)
        })
    } else {
        v1
    }
}

pub fn f560(v0: Adt140) -> R<Option<List<u64>>> {
    let m1769 = f349(f640(v0))?;
    match m1769 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1770 = f202(v1)?;
            match m1770 {
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

pub fn f561(v0: Adt88) -> R<Option<List<List<u64>>>> {
    if f641(v0.clone()) {
        let m1774 = f642({
            let m1773 = match v0 {
                Adt88::C0(h1772, _, _) => {
                    h1772
                }
            };
            match m1773 {
                Adt85::C0(h1771, _, _) => {
                    h1771
                }
            }
        });
        match m1774 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1775 = f349(v1)?;
                match m1775 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1776 = f202(v2)?;
                        match m1776 {
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

pub fn f562() -> u64 {
    404u64
}

pub fn f563(v0: Adt90) -> u64 {
    let m1777 = v0;
    match m1777 {
        Adt90::C0(_) => {
            f209()
        }
        Adt90::C1(_) => {
            f209()
        }
        Adt90::C2(_) => {
            f209()
        }
        Adt90::C3(_) => {
            f209()
        }
        Adt90::C4(_) => {
            f209()
        }
        Adt90::C5(_) => {
            f209()
        }
        Adt90::C6(_) => {
            f22()
        }
        Adt90::C7(_) => {
            f643()
        }
        Adt90::C8(_) => {
            f209()
        }
        Adt90::C9(_) => {
            f22()
        }
        Adt90::C10 => {
            f209()
        }
        Adt90::C11 => {
            f209()
        }
        Adt90::C12(_) => {
            f22()
        }
        Adt90::C13 => {
            f22()
        }
        Adt90::C14 => {
            f22()
        }
        Adt90::C15(_) => {
            f22()
        }
    }
}

pub fn f564(v0: Adt141) -> i64 {
    let m1778 = v0;
    match m1778 {
        Adt141::C0(_) => {
            f644()
        }
        Adt141::C1(_, _) => {
            f645()
        }
        Adt141::C2(_) => {
            f646()
        }
        Adt141::C3(_) => {
            f647()
        }
        Adt141::C4(v6) => {
            f648(v6)
        }
    }
}

pub fn f565(v0: Adt141) -> Str {
    let m1779 = v0;
    match m1779 {
        Adt141::C0(v1) => {
            f649(v1)
        }
        Adt141::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt141::C2(v4) => {
            f650(v4)
        }
        Adt141::C3(_) => {
            Str::lit("Method not found")
        }
        Adt141::C4(v6) => {
            let m1780 = v6;
            match m1780 {
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

pub fn f566(v0: Adt141) -> R<Option<Adt6>> {
    let m1781 = v0;
    match m1781 {
        Adt141::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt141::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f651(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt141::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt141::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt141::C4(v6) => {
            let m1782 = v6;
            match m1782 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f354(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f652(v15)), List::<(Str, Adt6)>::nil()))))
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

pub fn f567(v0: List<u64>) -> R<bool> {
    let m1783 = f484(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1783.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f653(v2)? {
                Ok(false)
            } else {
                f323(f325(v1), f654())
            }
        }
    }
}

pub fn f568(v0: Adt0, v1: Fn4, v2: u64, v3: Adt149) -> R<Result<Adt138, Adt141>> {
    f655(v0.clone(), v1, v2, v3.clone(), f656(match v0 {
        Adt0::C0(_, _, h1784, _, _, _, _, _) => {
            h1784
        }
    }, match v3 {
        Adt149::C0(_, h1785, _, _) => {
            h1785
        }
    }))
}

pub fn f569(v0: Adt152, v1: Adt153) -> Result<Adt149, Adt141> {
    let m1786 = v1;
    match m1786 {
        Adt153::C0(v3) => {
            f657(v3, v0)
        }
        Adt153::C1(v2) => {
            Err::<Adt149, Adt141>(v2)
        }
    }
}

pub fn f570(v0: Str) -> Adt152 {
    let a1787 = v0.clone();
    let a1788 = Str::lit("server/discover");
    if equal(a1787, a1788) {
        Adt152::C1(Adt150::C0)
    } else {
        let a1789 = v0.clone();
        let a1790 = Str::lit("tools/list");
        if equal(a1789, a1790) {
            Adt152::C1(Adt150::C1)
        } else {
            let a1791 = v0.clone();
            let a1792 = Str::lit("tools/call");
            if equal(a1791, a1792) {
                Adt152::C1(Adt150::C2)
            } else {
                let a1793 = v0.clone();
                let a1794 = Str::lit("resources/list");
                if equal(a1793, a1794) {
                    Adt152::C1(Adt150::C3)
                } else {
                    let a1795 = v0.clone();
                    let a1796 = Str::lit("resources/read");
                    if equal(a1795, a1796) {
                        Adt152::C1(Adt150::C4)
                    } else {
                        let a1797 = v0.clone();
                        let a1798 = Str::lit("resources/templates/list");
                        if equal(a1797, a1798) {
                            Adt152::C1(Adt150::C5)
                        } else {
                            let a1799 = v0.clone();
                            let a1800 = Str::lit("prompts/list");
                            if equal(a1799, a1800) {
                                Adt152::C1(Adt150::C6)
                            } else {
                                let a1801 = v0.clone();
                                let a1802 = Str::lit("prompts/get");
                                if equal(a1801, a1802) {
                                    Adt152::C1(Adt150::C7)
                                } else {
                                    let a1803 = v0.clone();
                                    let a1804 = Str::lit("completion/complete");
                                    if equal(a1803, a1804) {
                                        Adt152::C1(Adt150::C8)
                                    } else {
                                        let a1805 = v0.clone();
                                        let a1806 = Str::lit("tasks/get");
                                        if equal(a1805, a1806) {
                                            Adt152::C1(Adt150::C9)
                                        } else {
                                            let a1807 = v0.clone();
                                            let a1808 = Str::lit("tasks/update");
                                            if equal(a1807, a1808) {
                                                Adt152::C1(Adt150::C10)
                                            } else {
                                                let a1809 = v0;
                                                let a1810 = Str::lit("tasks/cancel");
                                                if equal(a1809, a1810) {
                                                    Adt152::C1(Adt150::C11)
                                                } else {
                                                    Adt152::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f571(v0: Option<Adt120>, v1: List<Str>, v2: Adt112) -> R<Adt153> {
    f658(v0, v1, v2.clone(), f659(match v2 {
        Adt112::C0(_, _, h1811) => {
            h1811
        }
    }))
}

pub fn f572(v0: Str) -> Adt155 {
    let m1812 = f570(v0);
    match m1812 {
        Adt152::C0 => {
            Adt155::C0
        }
        Adt152::C1(v1) => {
            f660(v1)
        }
    }
}

pub fn f573(v0: List<u64>) -> Option<List<u64>> {
    let m1813 = f661(f662(), v0);
    match m1813 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1814 = f661(f663(), f333(v1, List::<u64>::nil()));
            match m1814 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f333(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f574(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1816 = f664({
        let a1815 = v0.clone();
        length_list(a1815)
    }, v0, List::<u64>::nil())?;
    match m1816 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f401(v1, List::<u64>::nil())))
        }
    }
}

pub fn f575(p1819: List<Adt123>, p1820: List<Adt123>) -> List<Adt123> {
    let m1818 = (p1819, p1820);
    {
        let mut m1818 = m1818;
        loop {
            let step = {
                let (v0, v1) = m1818;
                let m1817 = v0;
                match m1817.uncons() {
                    None => {
                        Err::<(List<Adt123>, List<Adt123>), List<Adt123>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt123>, List<Adt123>), List<Adt123>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1818 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(v0: List<Adt93>, p1827: List<Adt12>, v2: Option<Str>, p1828: List<Adt123>) -> R<List<Adt123>> {
    let m1826 = (p1827, p1828);
    {
        let mut m1826 = m1826;
        loop {
            let step = {
                let (v1, v3) = m1826;
                let m1821 = v1;
                match m1821.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt123>), R<List<Adt123>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f665(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt123>), R<List<Adt123>>>((v5, List::cons(Adt123::C0(match v4.clone() {
                                Adt12::C0(_, h1822, _, _) => {
                                    h1822
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1823, _) => {
                                    h1823
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1824) => {
                                    h1824
                                }
                            }, f666(v0.clone(), match v4 {
                                Adt12::C0(_, h1825, _, _) => {
                                    h1825
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt123>), R<List<Adt123>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1826 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f577(p1844: u64, p1845: i64, p1846: i64) -> R<Option<i64>> {
    let m1843 = (p1844, (p1845, p1846));
    {
        let mut m1843 = m1843;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1843;
                let m1829 = v0;
                if m1829 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1829 - 1;
                    if f348(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1832 = {
                            let a1830 = v2;
                            let a1831 = 0i64;
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
                            if f348({
                                let a1833 = v1;
                                let a1834 = 10i64;
                                let a1835 = 0i64;
                                int_rem(a1833, a1834, a1835)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1836 = v1;
                                    let a1837 = 10i64;
                                    let a1838 = 0i64;
                                    int_quot(a1836, a1837, a1838)?
                                }, f446(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1839 = v1;
                                let a1840 = 10i64;
                                int_mul(a1839, a1840)?
                            }, {
                                let a1841 = v2;
                                let a1842 = 1i64;
                                int_sub(a1841, a1842)?
                            })))
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

pub fn f578(v0: i64) -> Option<i64> {
    let m1849 = {
        let a1847 = 9007199254740991i64;
        let a1848 = v0;
        compare(a1847, a1848)
    };
    if if match m1849 {
        Ordering::Less => {
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
        let m1852 = {
            let a1850 = v0;
            let a1851 = -9007199254740991i64;
            compare(a1850, a1851)
        };
        match m1852 {
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

pub fn f579(v0: Adt101, v1: Str, v2: Adt5) -> Result<Adt112, Adt106> {
    let m1853 = f234(Str::lit("params"), v2);
    match m1853 {
        Adt110::C0 => {
            Ok::<Adt112, Adt106>(Adt112::C0(v0, v1, None::<Adt5>))
        }
        Adt110::C1(v3) => {
            let m1854 = f239(v3);
            match m1854 {
                None => {
                    f319(Some(v0), Adt107::C6)
                }
                Some(v4) => {
                    Ok::<Adt112, Adt106>(Adt112::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f580(v0: Adt131) -> bool {
    let m1855 = v0;
    match m1855 {
        Adt131::C0 => {
            false
        }
        Adt131::C1 => {
            true
        }
        Adt131::C2 => {
            true
        }
        Adt131::C3 => {
            false
        }
        Adt131::C4 => {
            true
        }
        Adt131::C5 => {
            false
        }
        Adt131::C6 => {
            false
        }
        Adt131::C7 => {
            true
        }
    }
}

pub fn f581(v0: Adt127, v1: Adt132, v2: Adt128, v3: u64) -> R<Adt145> {
    let a1857 = match v0.clone() {
        Adt127::C0(_, _, h1856) => {
            h1856
        }
    };
    let a1858 = f619();
    if nat_lt(a1857, a1858) {
        Ok(Adt145::C1(Adt127::C0(v2, List::cons(v1, match v0.clone() {
            Adt127::C0(_, h1859, _) => {
                h1859
            }
        }), {
            let a1861 = match v0 {
                Adt127::C0(_, _, h1860) => {
                    h1860
                }
            };
            let a1862 = 1u64;
            nat_add(a1861, a1862)?
        })))
    } else {
        Ok(f509(Adt105::C9(v3)))
    }
}

pub fn f582(v0: bool, v1: Adt131, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt128 {
    Adt128::C8(Adt130::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f583(v0: u64) -> i64 {
    let a1863 = v0;
    let a1864 = 0u64;
    if nat_eq(a1863, a1864) {
        0i64
    } else {
        let a1865 = v0;
        let a1866 = 1u64;
        if nat_eq(a1865, a1866) {
            1i64
        } else {
            let a1867 = v0;
            let a1868 = 2u64;
            if nat_eq(a1867, a1868) {
                2i64
            } else {
                let a1869 = v0;
                let a1870 = 3u64;
                if nat_eq(a1869, a1870) {
                    3i64
                } else {
                    let a1871 = v0;
                    let a1872 = 4u64;
                    if nat_eq(a1871, a1872) {
                        4i64
                    } else {
                        let a1873 = v0;
                        let a1874 = 5u64;
                        if nat_eq(a1873, a1874) {
                            5i64
                        } else {
                            let a1875 = v0;
                            let a1876 = 6u64;
                            if nat_eq(a1875, a1876) {
                                6i64
                            } else {
                                let a1877 = v0;
                                let a1878 = 7u64;
                                if nat_eq(a1877, a1878) {
                                    7i64
                                } else {
                                    let a1879 = v0;
                                    let a1880 = 8u64;
                                    if nat_eq(a1879, a1880) {
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

pub fn f584(v0: Adt127, v1: u64) -> Adt145 {
    let m1882 = match v0.clone() {
        Adt127::C0(_, h1881, _) => {
            h1881
        }
    };
    match m1882.uncons() {
        None => {
            f509(Adt105::C2(v1))
        }
        Some((v2, _)) => {
            let m1883 = v2;
            match m1883 {
                Adt132::C0(_) => {
                    f508(v0, Adt128::C0)
                }
                Adt132::C1(_, _) => {
                    f508(v0, Adt128::C4)
                }
            }
        }
    }
}

pub fn f585(v0: Adt127, v1: Adt128) -> Adt127 {
    Adt127::C0(v1, match v0.clone() {
        Adt127::C0(_, h1884, _) => {
            h1884
        }
    }, match v0 {
        Adt127::C0(_, _, h1885) => {
            h1885
        }
    })
}

pub fn f586(v0: Adt127, v1: u64, v2: List<u64>, v3: bool) -> Adt145 {
    let m1887 = {
        let a1886 = f449(f200(), f401(v2, List::<u64>::nil()));
        utf8_decode(a1886)
    };
    match m1887 {
        None => {
            f509(Adt105::C14)
        }
        Some(v4) => {
            if v3 {
                f667(v0, v1, v4)
            } else {
                f592(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f587(v0: u64) -> u64 {
    let a1888 = v0;
    let a1889 = 34u64;
    if nat_eq(a1888, a1889) {
        34u64
    } else {
        let a1890 = v0;
        let a1891 = 92u64;
        if nat_eq(a1890, a1891) {
            92u64
        } else {
            let a1892 = v0;
            let a1893 = 47u64;
            if nat_eq(a1892, a1893) {
                47u64
            } else {
                let a1894 = v0;
                let a1895 = 98u64;
                if nat_eq(a1894, a1895) {
                    8u64
                } else {
                    let a1896 = v0;
                    let a1897 = 102u64;
                    if nat_eq(a1896, a1897) {
                        12u64
                    } else {
                        let a1898 = v0;
                        let a1899 = 110u64;
                        if nat_eq(a1898, a1899) {
                            10u64
                        } else {
                            let a1900 = v0;
                            let a1901 = 114u64;
                            if nat_eq(a1900, a1901) {
                                13u64
                            } else {
                                let a1902 = v0;
                                let a1903 = 116u64;
                                if nat_eq(a1902, a1903) {
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

pub fn f588(v0: u64) -> u64 {
    let a1904 = v0;
    let a1905 = 48u64;
    if nat_lt(a1904, a1905) {
        16u64
    } else {
        let a1906 = v0;
        let a1907 = 58u64;
        if nat_lt(a1906, a1907) {
            let a1908 = v0;
            let a1909 = 48u64;
            nat_sub(a1908, a1909)
        } else {
            let a1910 = v0;
            let a1911 = 65u64;
            if nat_lt(a1910, a1911) {
                16u64
            } else {
                let a1912 = v0;
                let a1913 = 71u64;
                if nat_lt(a1912, a1913) {
                    let a1914 = v0;
                    let a1915 = 55u64;
                    nat_sub(a1914, a1915)
                } else {
                    let a1916 = v0;
                    let a1917 = 97u64;
                    if nat_lt(a1916, a1917) {
                        16u64
                    } else {
                        let a1918 = v0;
                        let a1919 = 103u64;
                        if nat_lt(a1918, a1919) {
                            let a1920 = v0;
                            let a1921 = 87u64;
                            nat_sub(a1920, a1921)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f589(v0: Adt127, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt145> {
    let a1922 = v3;
    let a1923 = 0u64;
    if nat_eq(a1922, a1923) {
        if f500(v2, 55296u64, 56319u64) {
            Ok(f508(v0, Adt128::C6(Adt129::C3(v2), v4, v5)))
        } else {
            if f500(v2, 56320u64, 57343u64) {
                Ok(f509(Adt105::C6(v1)))
            } else {
                Ok(f508(v0, Adt128::C6(Adt129::C0, f668(v2, v4)?, v5)))
            }
        }
    } else {
        if f500(v2, 56320u64, 57343u64) {
            Ok(f508(v0, Adt128::C6(Adt129::C0, f668({
                let a1932 = 65536u64;
                let a1933 = {
                    let a1930 = {
                        let a1926 = {
                            let a1924 = v3;
                            let a1925 = 55296u64;
                            nat_sub(a1924, a1925)
                        };
                        let a1927 = 1024u64;
                        nat_mul(a1926, a1927)?
                    };
                    let a1931 = {
                        let a1928 = v2;
                        let a1929 = 56320u64;
                        nat_sub(a1928, a1929)
                    };
                    nat_add(a1930, a1931)?
                };
                nat_add(a1932, a1933)?
            }, v4)?, v5)))
        } else {
            Ok(f509(Adt105::C6(v1)))
        }
    }
}

pub fn f590(v0: u64, v1: u64) -> u64 {
    let m1936 = {
        let a1934 = f591(v0);
        let a1935 = v1;
        index_list(a1934, a1935)
    };
    match m1936 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f591(v0: u64) -> List<u64> {
    let a1937 = v0;
    let a1938 = 0u64;
    if nat_eq(a1937, a1938) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1939 = v0;
        let a1940 = 1u64;
        if nat_eq(a1939, a1940) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f592(v0: Adt127, v1: Adt6) -> Adt145 {
    Adt145::C1(f502(v1, match v0.clone() {
        Adt127::C0(_, h1941, _) => {
            h1941
        }
    }, match v0 {
        Adt127::C0(_, _, h1942) => {
            h1942
        }
    }))
}

pub fn f593(v0: u64) -> Adt6 {
    let a1943 = v0;
    let a1944 = 0u64;
    if nat_eq(a1943, a1944) {
        Adt6::C1(true)
    } else {
        let a1945 = v0;
        let a1946 = 1u64;
        if nat_eq(a1945, a1946) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f594(v0: Adt127, v1: Adt130, v2: Adt131) -> Adt145 {
    f508(v0, Adt128::C8(Adt130::C0(match v1.clone() {
        Adt130::C0(h1947, _, _, _, _, _, _) => {
            h1947
        }
    }, v2, match v1.clone() {
        Adt130::C0(_, _, h1948, _, _, _, _) => {
            h1948
        }
    }, match v1.clone() {
        Adt130::C0(_, _, _, h1949, _, _, _) => {
            h1949
        }
    }, match v1.clone() {
        Adt130::C0(_, _, _, _, h1950, _, _) => {
            h1950
        }
    }, match v1.clone() {
        Adt130::C0(_, _, _, _, _, h1951, _) => {
            h1951
        }
    }, match v1 {
        Adt130::C0(_, _, _, _, _, _, h1952) => {
            h1952
        }
    })))
}

pub fn f595(v0: Adt127, v1: u64, v2: u64, v3: Adt130, v4: bool) -> R<Adt145> {
    let v5: u64 = {
        let a1954 = match v3.clone() {
            Adt130::C0(_, _, _, h1953, _, _, _) => {
                h1953
            }
        };
        let a1955 = 0u64;
        if if nat_eq(a1954, a1955) {
            let a1956 = v2;
            let a1957 = 48u64;
            nat_eq(a1956, a1957)
        } else {
            false
        } {
            0u64
        } else {
            let a1959 = match v3.clone() {
                Adt130::C0(_, _, _, h1958, _, _, _) => {
                    h1958
                }
            };
            let a1960 = 1u64;
            nat_add(a1959, a1960)?
        }
    };
    let v6: i64 = if v4 {
        f446(match v3.clone() {
            Adt130::C0(_, _, _, _, h1961, _, _) => {
                h1961
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt130::C0(_, _, _, _, h1962, _, _) => {
                h1962
            }
        }
    };
    let a1963 = f669();
    let a1964 = v5;
    if nat_lt(a1963, a1964) {
        Ok(f509(Adt105::C8(v1)))
    } else {
        let m1967 = {
            let a1965 = 99999i64;
            let a1966 = v6;
            compare(a1965, a1966)
        };
        if match m1967 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f509(Adt105::C8(v1)))
        } else {
            Ok(f508(v0, f582(match v3.clone() {
                Adt130::C0(h1968, _, _, _, _, _, _) => {
                    h1968
                }
            }, if v4 {
                Adt131::C4
            } else {
                Adt131::C2
            }, f446({
                let a1970 = match v3.clone() {
                    Adt130::C0(_, _, h1969, _, _, _, _) => {
                        h1969
                    }
                };
                let a1971 = 10i64;
                int_mul(a1970, a1971)?
            }, f583({
                let a1972 = v2;
                let a1973 = 48u64;
                nat_sub(a1972, a1973)
            }))?, v5, v6, match v3.clone() {
                Adt130::C0(_, _, _, _, _, h1974, _) => {
                    h1974
                }
            }, match v3 {
                Adt130::C0(_, _, _, _, _, _, h1975) => {
                    h1975
                }
            })))
        }
    }
}

pub fn f596(v0: u64) -> bool {
    let a1976 = v0;
    let a1977 = 101u64;
    if nat_eq(a1976, a1977) {
        true
    } else {
        let a1978 = v0;
        let a1979 = 69u64;
        nat_eq(a1978, a1979)
    }
}

pub fn f597(v0: u64) -> bool {
    f500(v0, 48u64, 57u64)
}

pub fn f598(v0: Adt127, v1: u64, v2: u64, v3: Adt130) -> R<Adt145> {
    let m1980 = f501(v3)?;
    match m1980 {
        Adt151::C0 => {
            Ok(f509(Adt105::C7(v1)))
        }
        Adt151::C1 => {
            Ok(f509(Adt105::C8(v1)))
        }
        Adt151::C2(v4) => {
            Ok(f506(f502(v4, match v0.clone() {
                Adt127::C0(_, h1981, _) => {
                    h1981
                }
            }, match v0 {
                Adt127::C0(_, _, h1982) => {
                    h1982
                }
            }), v1, v2))
        }
    }
}

pub fn f599(v0: Adt127, v1: u64, v2: u64, v3: Adt130, v4: bool) -> R<Adt145> {
    let v5: i64 = f446({
        let a1984 = match v3.clone() {
            Adt130::C0(_, _, _, _, _, _, h1983) => {
                h1983
            }
        };
        let a1985 = 10i64;
        int_mul(a1984, a1985)?
    }, f583({
        let a1986 = v2;
        let a1987 = 48u64;
        nat_sub(a1986, a1987)
    }))?;
    let m1990 = {
        let a1988 = 99999i64;
        let a1989 = v5;
        compare(a1988, a1989)
    };
    if match m1990 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f509(Adt105::C8(v1)))
    } else {
        Ok(f508(v0, f582(match v3.clone() {
            Adt130::C0(h1991, _, _, _, _, _, _) => {
                h1991
            }
        }, Adt131::C7, match v3.clone() {
            Adt130::C0(_, _, h1992, _, _, _, _) => {
                h1992
            }
        }, match v3.clone() {
            Adt130::C0(_, _, _, h1993, _, _, _) => {
                h1993
            }
        }, match v3 {
            Adt130::C0(_, _, _, _, h1994, _, _) => {
                h1994
            }
        }, v4, v5)))
    }
}

pub fn f600(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1995 = f670(f671(), v2.clone());
    match m1995 {
        None => {
            let m1996 = f672(f673(), v2.clone());
            match m1996 {
                None => {
                    f674(v0, v1, v2)
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

pub fn f601(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt133) -> R<Adt156> {
    let m1997 = f347(Str::lit("x-mcp-header"), v1.clone());
    match m1997 {
        Adt110::C0 => {
            Ok(Adt156::C1(v4))
        }
        Adt110::C1(v5) => {
            f675(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f602(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt113>) -> List<Adt113> {
    f676(Str::lit("properties"), v0.clone(), v1, v2, f676(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f677(Str::lit("items"), v0.clone(), f677(Str::lit("additionalProperties"), v0.clone(), f678(Str::lit("allOf"), v0.clone(), f678(Str::lit("anyOf"), v0.clone(), f678(Str::lit("oneOf"), v0.clone(), f677(Str::lit("not"), v0.clone(), f677(Str::lit("if"), v0.clone(), f677(Str::lit("then"), v0.clone(), f677(Str::lit("else"), v0, v3)))))))))))
}

pub fn f603(v0: List<Adt51>) -> R<bool> {
    let m1998 = v0;
    match m1998.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f679(v2, match v1 {
                Adt51::C0(h1999, _, _) => {
                    h1999
                }
            })
        }
    }
}

pub fn f604(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt146> {
    Ok(f680(List::cons((Str::lit("type"), f681(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f682(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f683(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f684(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f684(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f684(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f684(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a2000 = f685(v0.clone(), v1.clone())?;
        let a2001 = List::cons((Str::lit("required"), f686(v0, v1)), List::<(Str, Adt157)>::nil());
        append_list(a2000, a2001)
    })))))))))
}

pub fn f605(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt115>) -> R<Adt146> {
    Ok(f687(f688(v0, v1.clone(), v2.clone(), v3.clone()), f687(f689(v1.clone(), v2.clone(), v3.clone()), f687(f690(v1.clone(), v2.clone(), v3.clone()), f687(f691(v1.clone(), v2.clone(), v3.clone())?, f687(f692(v1.clone(), v2.clone(), v3.clone()), f687(f693(Adt118::C0, f528(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f687(f693(Adt118::C1, f529(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f687(f693(Adt118::C2, f530(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f693(Adt118::C3, f531(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f606(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2002 = f347(v0, v1);
    match m2002 {
        Adt110::C0 => {
            None::<Adt6>
        }
        Adt110::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f607(v0: Str) -> R<Str> {
    let m2005 = {
        let a2004 = f449(f200(), f401(f694(f198({
            let a2003 = v0.clone();
            utf8_encode(a2003)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2004)
    };
    match m2005 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f608(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f695(v0, 0u64, v1)
}

pub fn f609(v0: Adt40) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt40::C0(h2006, _, _) => {
            h2006
        }
    })), {
        let a2009 = f696(match v0.clone() {
            Adt40::C0(_, h2007, _) => {
                h2007
            }
        });
        let a2010 = f697(match v0 {
            Adt40::C0(_, _, h2008) => {
                h2008
            }
        });
        append_list(a2009, a2010)
    })))
}

pub fn f610(v0: Str, v1: Str, v2: Str, v3: Option<Adt41>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2011 = f696(v3);
        let a2012 = f697(v4);
        append_list(a2011, a2012)
    }))))
}

pub fn f611(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2036 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2013, _, _, _, _, _, _, _, _) => {
                h2013
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2037 = {
            let a2034 = f698(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2014, _, _, _, _, _, _, _) => {
                    h2014
                }
            });
            let a2035 = {
                let a2032 = f699(match v0.clone() {
                    Adt45::C0(_, _, h2015, _, _, _, _, _, _) => {
                        h2015
                    }
                });
                let a2033 = {
                    let a2030 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2016, _, _, _, _, _) => {
                            h2016
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2031 = {
                        let a2028 = f698(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2017, _, _, _, _) => {
                                h2017
                            }
                        });
                        let a2029 = {
                            let a2026 = f698(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2018, _, _, _) => {
                                    h2018
                                }
                            });
                            let a2027 = {
                                let a2024 = f696(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2019, _, _) => {
                                        h2019
                                    }
                                });
                                let a2025 = {
                                    let a2022 = f700(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2020, _) => {
                                            h2020
                                        }
                                    })?;
                                    let a2023 = f697(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2021) => {
                                            h2021
                                        }
                                    });
                                    append_list(a2022, a2023)
                                };
                                append_list(a2024, a2025)
                            };
                            append_list(a2026, a2027)
                        };
                        append_list(a2028, a2029)
                    };
                    append_list(a2030, a2031)
                };
                append_list(a2032, a2033)
            };
            append_list(a2034, a2035)
        };
        append_list(a2036, a2037)
    }))
}

pub fn f612(v0: Adt46) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f701(match v0.clone() {
        Adt46::C0(h2038, _, _) => {
            h2038
        }
    })), {
        let a2041 = f696(match v0.clone() {
            Adt46::C0(_, h2039, _) => {
                h2039
            }
        });
        let a2042 = f697(match v0 {
            Adt46::C0(_, _, h2040) => {
                h2040
            }
        });
        append_list(a2041, a2042)
    })))
}

pub fn f613(p2051: Bytes, p2052: List<Bytes>) -> List<Bytes> {
    let m2050 = (p2051, p2052);
    {
        let mut m2050 = m2050;
        loop {
            let step = {
                let (v0, v1) = m2050;
                let m2043 = v1;
                match m2043.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a2046 = {
                            let a2044 = v2.clone();
                            length_bytes(a2044)
                        };
                        let a2047 = {
                            let a2045 = v0.clone();
                            length_bytes(a2045)
                        };
                        if nat_eq(a2046, a2047) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a2048 = v2;
                                let a2049 = v0;
                                append_bytes(a2048, a2049)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: u64) -> Bytes {
    let m2056 = {
        let a2053 = f702();
        let a2054 = v0;
        let a2055 = 1u64;
        slice_bytes(a2053, a2054, a2055)
    };
    match m2056 {
        None => {
            f200()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f615(p2062: List<(Str, Adt6)>, p2063: List<(Str, Adt65)>) -> Option<List<(Str, Adt65)>> {
    let m2061 = (p2062, p2063);
    {
        let mut m2061 = m2061;
        loop {
            let step = {
                let (v0, v1) = m2061;
                let m2057 = v0;
                match m2057.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(Some(f465(v1, List::<(Str, Adt65)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2059 = f703({
                            let (_, h2058) = v2;
                            h2058
                        });
                        match m2059 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(None::<List<(Str, Adt65)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>((v3, List::cons(({
                                    let (h2060, _) = v2;
                                    h2060
                                }, v4), v1)))
                            }
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

pub fn f616(v0: u64) -> List<u64> {
    f591(v0)
}

pub fn f617(v0: Adt7) -> R<bool> {
    let a2081 = {
        let m2067 = {
            let a2065 = f704();
            let a2066 = match v0.clone() {
                Adt7::C0(h2064, _) => {
                    h2064
                }
            };
            compare(a2065, a2066)
        };
        if match m2067 {
            Ordering::Less => {
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
            let m2072 = {
                let a2070 = match v0.clone() {
                    Adt7::C0(h2068, _) => {
                        h2068
                    }
                };
                let a2071 = {
                    let a2069 = f704();
                    int_neg(a2069)?
                };
                compare(a2070, a2071)
            };
            if match m2072 {
                Ordering::Less => {
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
                let m2076 = {
                    let a2074 = 9999i64;
                    let a2075 = match v0.clone() {
                        Adt7::C0(_, h2073) => {
                            h2073
                        }
                    };
                    compare(a2074, a2075)
                };
                if match m2076 {
                    Ordering::Less => {
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
                    let m2080 = {
                        let a2078 = match v0 {
                            Adt7::C0(_, h2077) => {
                                h2077
                            }
                        };
                        let a2079 = -9999i64;
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
                }
            }
        }
    };
    Ok(bool_not(a2081))
}

pub fn f618(v0: Adt7) -> R<List<u64>> {
    let m2085 = {
        let a2083 = match v0.clone() {
            Adt7::C0(_, h2082) => {
                h2082
            }
        };
        let a2084 = 0i64;
        compare(a2083, a2084)
    };
    if if match m2085 {
        Ordering::Less => {
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
        let m2089 = {
            let a2087 = 0i64;
            let a2088 = match v0.clone() {
                Adt7::C0(_, h2086) => {
                    h2086
                }
            };
            compare(a2087, a2088)
        };
        match m2089 {
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
        let a2092 = f705(match v0.clone() {
            Adt7::C0(h2090, _) => {
                h2090
            }
        })?;
        let a2093 = List::cons(101u64, f705(match v0 {
            Adt7::C0(_, h2091) => {
                h2091
            }
        })?);
        Ok(append_list(a2092, a2093))
    } else {
        f705(match v0 {
            Adt7::C0(h2094, _) => {
                h2094
            }
        })
    }
}

pub fn f619() -> u64 {
    512u64
}

pub fn f620(v0: u64, v1: Adt119, v2: List<Adt119>) -> List<Adt119> {
    List::cons(Adt119::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f621(p2097: List<u64>, p2098: List<u64>) -> R<List<u64>> {
    let m2096 = (p2097, p2098);
    {
        let mut m2096 = m2096;
        loop {
            let step = {
                let (v0, v1) = m2096;
                let m2095 = v0;
                match m2095.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f401(f706(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2096 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f707(match v0.clone() {
        Adt34::C0(_, h2099, _, _) => {
            h2099
        }
    })), List::cons((Str::lit("capabilities"), f708(match v0.clone() {
        Adt34::C0(_, _, h2100, _) => {
            h2100
        }
    })), {
        let a2103 = f698(Str::lit("instructions"), match v0.clone() {
            Adt34::C0(_, _, _, h2101) => {
                h2101
            }
        });
        let a2104 = f709(match v0 {
            Adt34::C0(h2102, _, _, _) => {
                h2102
            }
        })?;
        append_list(a2103, a2104)
    })))
}

pub fn f623(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f710(match v0.clone() {
        Adt35::C0(_, _, h2105) => {
            h2105
        }
    }))), {
        let a2108 = f698(Str::lit("nextCursor"), match v0.clone() {
            Adt35::C0(_, h2106, _) => {
                h2106
            }
        });
        let a2109 = f709(match v0 {
            Adt35::C0(h2107, _, _) => {
                h2107
            }
        })?;
        append_list(a2108, a2109)
    }))
}

pub fn f624(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f711(match v0.clone() {
        Adt52::C0(_, _, h2110) => {
            h2110
        }
    })?)), {
        let a2113 = f698(Str::lit("nextCursor"), match v0.clone() {
            Adt52::C0(_, h2111, _) => {
                h2111
            }
        });
        let a2114 = f709(match v0 {
            Adt52::C0(h2112, _, _) => {
                h2112
            }
        })?;
        append_list(a2113, a2114)
    }))
}

pub fn f625(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f712(match v0.clone() {
        Adt53::C0(_, h2115) => {
            h2115
        }
    }))), f709(match v0 {
        Adt53::C0(h2116, _) => {
            h2116
        }
    })?))
}

pub fn f626(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f713(match v0.clone() {
        Adt54::C0(_, _, h2117) => {
            h2117
        }
    }))), {
        let a2120 = f698(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h2118, _) => {
                h2118
            }
        });
        let a2121 = f709(match v0 {
            Adt54::C0(h2119, _, _) => {
                h2119
            }
        })?;
        append_list(a2120, a2121)
    }))
}

pub fn f627(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f714(match v0.clone() {
        Adt56::C0(_, _, h2122) => {
            h2122
        }
    }))), {
        let a2125 = f698(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h2123, _) => {
                h2123
            }
        });
        let a2126 = f709(match v0 {
            Adt56::C0(h2124, _, _) => {
                h2124
            }
        })?;
        append_list(a2125, a2126)
    }))
}

pub fn f628(v0: Adt59) -> R<Option<List<(Str, Adt6)>>> {
    let m2128 = f715(match v0.clone() {
        Adt59::C0(_, h2127) => {
            h2127
        }
    })?;
    match m2128 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2130 = f698(Str::lit("description"), match v0 {
                    Adt59::C0(h2129, _) => {
                        h2129
                    }
                });
                let a2131 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a2130, a2131)
            }))
        }
    }
}

pub fn f629(v0: Adt61) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f651({
        let m2134 = match v0.clone() {
            Adt61::C0(h2133) => {
                h2133
            }
        };
        match m2134 {
            Adt62::C0(h2132, _, _) => {
                h2132
            }
        }
    }))), {
        let a2141 = f716(Str::lit("total"), {
            let m2137 = match v0.clone() {
                Adt61::C0(h2136) => {
                    h2136
                }
            };
            match m2137 {
                Adt62::C0(_, h2135, _) => {
                    h2135
                }
            }
        })?;
        let a2142 = f352(Str::lit("hasMore"), {
            let m2140 = match v0 {
                Adt61::C0(h2139) => {
                    h2139
                }
            };
            match m2140 {
                Adt62::C0(_, _, h2138) => {
                    h2138
                }
            }
        });
        append_list(a2141, a2142)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f630(v0: Adt82) -> R<Option<List<(Str, Adt6)>>> {
    let m2143 = f717(f718(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m2143 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2144 = f719(v1);
                let a2145 = f720(f721(v0));
                append_list(a2144, a2145)
            }))
        }
    }
}

pub fn f631(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m2146 = f722(v0.clone());
    match m2146 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a2147 = f723(f387(v0.clone()), f724(f725(v0)));
                let a2148 = v1;
                append_list(a2147, a2148)
            })
        }
    }
}

pub fn f632(v0: Adt88) -> Option<List<(Str, Adt6)>> {
    let m2150 = match v0.clone() {
        Adt88::C0(_, h2149, _) => {
            h2149
        }
    };
    match m2150 {
        Adt89::C0 => {
            f631({
                let m2153 = match v0 {
                    Adt88::C0(h2152, _, _) => {
                        h2152
                    }
                };
                match m2153 {
                    Adt85::C0(h2151, _, _) => {
                        h2151
                    }
                }
            })
        }
        Adt89::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f633(v0: Adt158) -> Str {
    let m2154 = v0;
    match m2154 {
        Adt158::C0 => {
            Str::lit("complete")
        }
        Adt158::C1 => {
            Str::lit("input_required")
        }
        Adt158::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f634(v0: Adt33) -> Adt158 {
    let m2155 = v0;
    match m2155 {
        Adt33::C0(_) => {
            Adt158::C0
        }
        Adt33::C1(_) => {
            Adt158::C0
        }
        Adt33::C2(_) => {
            Adt158::C0
        }
        Adt33::C3(_) => {
            Adt158::C0
        }
        Adt33::C4(_) => {
            Adt158::C0
        }
        Adt33::C5(_) => {
            Adt158::C0
        }
        Adt33::C6(_) => {
            Adt158::C0
        }
        Adt33::C7(_) => {
            Adt158::C0
        }
        Adt33::C8(_) => {
            Adt158::C0
        }
        Adt33::C9(_) => {
            Adt158::C0
        }
        Adt33::C10(_) => {
            Adt158::C0
        }
        Adt33::C11 => {
            Adt158::C0
        }
        Adt33::C12(_) => {
            Adt158::C1
        }
        Adt33::C13(_) => {
            Adt158::C2
        }
        Adt33::C14(_) => {
            Adt158::C0
        }
    }
}

pub fn f635(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m2156 = v0;
    match m2156 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f726(), f727(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f636(v0: List<Adt85>) -> R<Option<List<u64>>> {
    let m2157 = f728(v0);
    match m2157 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m2158 = f202(v1)?;
            match m2158 {
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

pub fn f637(v0: Adt85, v1: List<Adt85>, v2: i64) -> R<List<Adt85>> {
    Ok(List::cons(v0.clone(), f385(f729(f296(v0), v1, v2, List::<Adt85>::nil())?, List::<Adt85>::nil())))
}

pub fn f638(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f730(v2) {
            let a2159 = v1;
            let a2160 = 200u64;
            nat_eq(a2159, a2160)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f639(v0: u64, v1: List<List<u64>>) -> Adt95 {
    Adt95::C0(v0, f731(), List::<List<u64>>::nil(), f732(v1), None::<List<u64>>)
}

pub fn f640(v0: Adt140) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f310())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f733(match v0.clone() {
        Adt140::C0(h2161, _) => {
            h2161
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m2164 = match v0.clone() {
            Adt140::C0(_, h2163) => {
                h2163
            }
        };
        match m2164 {
            Adt51::C0(h2162, _, _) => {
                h2162
            }
        }
    })), {
        let a2171 = f734({
            let m2167 = match v0.clone() {
                Adt140::C0(_, h2166) => {
                    h2166
                }
            };
            match m2167 {
                Adt51::C0(_, h2165, _) => {
                    h2165
                }
            }
        });
        let a2172 = f735({
            let m2170 = match v0 {
                Adt140::C0(_, h2169) => {
                    h2169
                }
            };
            match m2170 {
                Adt51::C0(_, _, h2168) => {
                    h2168
                }
            }
        });
        append_list(a2171, a2172)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f641(v0: Adt88) -> bool {
    if match v0.clone() {
        Adt88::C0(_, _, h2173) => {
            h2173
        }
    } {
        let m2175 = match v0 {
            Adt88::C0(_, h2174, _) => {
                h2174
            }
        };
        match m2175 {
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

pub fn f642(v0: Adt63) -> Option<Adt6> {
    let m2176 = f631(v0);
    match m2176 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f643() -> u64 {
    429u64
}

pub fn f644() -> i64 {
    -32602i64
}

pub fn f645() -> i64 {
    -32022i64
}

pub fn f646() -> i64 {
    -32020i64
}

pub fn f647() -> i64 {
    -32601i64
}

pub fn f648(v0: Adt90) -> i64 {
    let m2177 = v0;
    match m2177 {
        Adt90::C0(_) => {
            f644()
        }
        Adt90::C1(_) => {
            f644()
        }
        Adt90::C2(_) => {
            f644()
        }
        Adt90::C3(_) => {
            f644()
        }
        Adt90::C4(_) => {
            f644()
        }
        Adt90::C5(_) => {
            f644()
        }
        Adt90::C6(_) => {
            f736()
        }
        Adt90::C7(_) => {
            f737()
        }
        Adt90::C8(_) => {
            f738()
        }
        Adt90::C9(_) => {
            f736()
        }
        Adt90::C10 => {
            f644()
        }
        Adt90::C11 => {
            f644()
        }
        Adt90::C12(_) => {
            f736()
        }
        Adt90::C13 => {
            f736()
        }
        Adt90::C14 => {
            f736()
        }
        Adt90::C15(_) => {
            f736()
        }
    }
}

pub fn f649(v0: Adt142) -> Str {
    let m2178 = v0;
    match m2178 {
        Adt142::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt142::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt142::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt142::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt142::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt142::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt142::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt142::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt142::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f650(v0: Adt121) -> Str {
    let m2179 = v0;
    match m2179 {
        Adt121::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt121::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt121::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt121::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt121::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt121::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt121::C6(v1) => {
            let m2180 = v1;
            match m2180 {
                Adt122::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt122::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt122::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt121::C7(v2) => {
            let m2181 = v2;
            match m2181 {
                Adt122::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt122::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt122::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt121::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt121::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt121::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f651(v0: List<Str>) -> List<Adt6> {
    f445(f739(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f652(v0: Adt18) -> Adt6 {
    Adt6::C5({
        let a2193 = f740(Str::lit("experimental"), match v0.clone() {
            Adt18::C0(h2182, _, _, _, _) => {
                h2182
            }
        });
        let a2194 = {
            let a2191 = f740(Str::lit("roots"), match v0.clone() {
                Adt18::C0(_, h2183, _, _, _) => {
                    h2183
                }
            });
            let a2192 = {
                let a2189 = f740(Str::lit("sampling"), match v0.clone() {
                    Adt18::C0(_, _, h2184, _, _) => {
                        h2184
                    }
                });
                let a2190 = {
                    let a2187 = f740(Str::lit("elicitation"), match v0.clone() {
                        Adt18::C0(_, _, _, h2185, _) => {
                            h2185
                        }
                    });
                    let a2188 = f740(Str::lit("extensions"), match v0 {
                        Adt18::C0(_, _, _, _, h2186) => {
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
    })
}

pub fn f653(p2197: List<List<u64>>) -> R<bool> {
    let m2196 = p2197;
    {
        let mut m2196 = m2196;
        loop {
            let step = {
                let v0 = m2196;
                let m2195 = v0;
                match m2195.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f741(f325(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f654() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f655(v0: Adt0, v1: Fn4, v2: u64, v3: Adt149, v4: bool) -> R<Result<Adt138, Adt141>> {
    if v4 {
        f742(v0, v1, v2, f743(match v3.clone() {
            Adt149::C0(_, h2198, _, _) => {
                h2198
            }
        }, match v3.clone() {
            Adt149::C0(_, _, h2199, _) => {
                h2199
            }
        }, match v3 {
            Adt149::C0(_, _, _, h2200) => {
                h2200
            }
        }))
    } else {
        Ok(Err::<Adt138, Adt141>(Adt141::C3(f744(match v3 {
            Adt149::C0(_, h2201, _, _) => {
                h2201
            }
        }))))
    }
}

pub fn f656(v0: Adt4, v1: Adt150) -> bool {
    let m2202 = v1;
    match m2202 {
        Adt150::C0 => {
            true
        }
        Adt150::C1 => {
            f745(match v0 {
                Adt4::C0(_, _, _, _, _, h2203, _) => {
                    h2203
                }
            })
        }
        Adt150::C2 => {
            f745(match v0 {
                Adt4::C0(_, _, _, _, _, h2204, _) => {
                    h2204
                }
            })
        }
        Adt150::C3 => {
            f746(match v0 {
                Adt4::C0(_, _, _, _, h2205, _, _) => {
                    h2205
                }
            })
        }
        Adt150::C4 => {
            f746(match v0 {
                Adt4::C0(_, _, _, _, h2206, _, _) => {
                    h2206
                }
            })
        }
        Adt150::C5 => {
            f746(match v0 {
                Adt4::C0(_, _, _, _, h2207, _, _) => {
                    h2207
                }
            })
        }
        Adt150::C6 => {
            f745(match v0 {
                Adt4::C0(_, _, _, h2208, _, _, _) => {
                    h2208
                }
            })
        }
        Adt150::C7 => {
            f745(match v0 {
                Adt4::C0(_, _, _, h2209, _, _, _) => {
                    h2209
                }
            })
        }
        Adt150::C8 => {
            f362(match v0 {
                Adt4::C0(_, _, h2210, _, _, _, _) => {
                    h2210
                }
            })
        }
        Adt150::C9 => {
            f293(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2211) => {
                    h2211
                }
            }, f28())
        }
        Adt150::C10 => {
            f293(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2212) => {
                    h2212
                }
            }, f28())
        }
        Adt150::C11 => {
            f293(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2213) => {
                    h2213
                }
            }, f28())
        }
    }
}

pub fn f657(v0: Adt154, v1: Adt152) -> Result<Adt149, Adt141> {
    let m2214 = v1;
    match m2214 {
        Adt152::C0 => {
            Err::<Adt149, Adt141>(Adt141::C3(match v0 {
                Adt154::C0(_, h2215, _, _) => {
                    h2215
                }
            }))
        }
        Adt152::C1(v2) => {
            Ok::<Adt149, Adt141>(Adt149::C0(match v0.clone() {
                Adt154::C0(h2216, _, _, _) => {
                    h2216
                }
            }, v2, match v0.clone() {
                Adt154::C0(_, _, h2217, _) => {
                    h2217
                }
            }, match v0 {
                Adt154::C0(_, _, _, h2218) => {
                    h2218
                }
            }))
        }
    }
}

pub fn f658(v0: Option<Adt120>, v1: List<Str>, v2: Adt112, v3: Result<Adt17, Adt142>) -> R<Adt153> {
    let m2219 = v3;
    match m2219 {
        Ok(v5) => {
            Ok(f747(v1, v2.clone(), v5.clone(), f748(v0, f572(match v2.clone() {
                Adt112::C0(_, h2220, _) => {
                    h2220
                }
            }), match v2.clone() {
                Adt112::C0(_, h2221, _) => {
                    h2221
                }
            }, match v5 {
                Adt17::C0(h2222, _, _, _, _, _) => {
                    h2222
                }
            }, f749(match v2 {
                Adt112::C0(_, _, h2223) => {
                    h2223
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt153::C1(Adt141::C0(v4)))
        }
    }
}

pub fn f659(v0: Option<Adt5>) -> Result<Adt17, Adt142> {
    let m2224 = v0;
    match m2224 {
        None => {
            Err::<Adt17, Adt142>(Adt142::C0)
        }
        Some(v1) => {
            let m2225 = f234(Str::lit("_meta"), v1);
            match m2225 {
                Adt110::C0 => {
                    Err::<Adt17, Adt142>(Adt142::C1)
                }
                Adt110::C1(v2) => {
                    let m2226 = f239(v2);
                    match m2226 {
                        None => {
                            Err::<Adt17, Adt142>(Adt142::C4)
                        }
                        Some(v3) => {
                            f750(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f660(v0: Adt150) -> Adt155 {
    let m2227 = v0;
    match m2227 {
        Adt150::C0 => {
            Adt155::C0
        }
        Adt150::C1 => {
            Adt155::C0
        }
        Adt150::C2 => {
            Adt155::C1
        }
        Adt150::C3 => {
            Adt155::C0
        }
        Adt150::C4 => {
            Adt155::C2
        }
        Adt150::C5 => {
            Adt155::C0
        }
        Adt150::C6 => {
            Adt155::C0
        }
        Adt150::C7 => {
            Adt155::C1
        }
        Adt150::C8 => {
            Adt155::C0
        }
        Adt150::C9 => {
            Adt155::C3
        }
        Adt150::C10 => {
            Adt155::C3
        }
        Adt150::C11 => {
            Adt155::C3
        }
    }
}

pub fn f661(p2233: List<u64>, p2234: List<u64>) -> Option<List<u64>> {
    let m2232 = (p2233, p2234);
    {
        let mut m2232 = m2232;
        loop {
            let step = {
                let (v0, v1) = m2232;
                let m2228 = v0;
                match m2228.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2229 = v1;
                        match m2229.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2230 = v4;
                                let a2231 = v2;
                                if nat_eq(a2230, a2231) {
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
                Ok(next) => { m2232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f662() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f663() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f664(p2243: u64, p2244: List<u64>, p2245: List<u64>) -> R<Option<List<u64>>> {
    let m2242 = (p2243, (p2244, p2245));
    {
        let mut m2242 = m2242;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2242;
                let m2235 = v0;
                if m2235 == 0 {
                    let m2236 = v1;
                    match m2236.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m2235 - 1;
                    let m2237 = v1;
                    match m2237.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m2238 = v7;
                            match m2238.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m2239 = v9;
                                    match m2239.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m2240 = v11;
                                            match m2240.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m2241 = f751(v6, v8, v10, v12, f752(v13.clone()))?;
                                                    match m2241 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f401(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m2242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f665(v0: Adt12, v1: Option<Str>) -> bool {
    let m2246 = v1;
    match m2246 {
        None => {
            false
        }
        Some(v2) => {
            let a2248 = match v0 {
                Adt12::C0(h2247, _, _, _) => {
                    h2247
                }
            };
            let a2249 = v2;
            equal(a2248, a2249)
        }
    }
}

pub fn f666(v0: List<Adt93>, v1: Str) -> R<Adt124> {
    let m2252 = f491(f115(v0, {
        let a2250 = f753();
        let a2251 = f754(v1)?;
        append_list(a2250, a2251)
    })?)?;
    match m2252 {
        Adt108::C0 => {
            Ok(Adt124::C0)
        }
        Adt108::C1 => {
            Ok(Adt124::C1)
        }
        Adt108::C2 => {
            Ok(Adt124::C1)
        }
        Adt108::C3(v2) => {
            let m2253 = f398(v2);
            match m2253 {
                None => {
                    Ok(Adt124::C1)
                }
                Some(v3) => {
                    Ok(Adt124::C2(v3))
                }
            }
        }
    }
}

pub fn f667(v0: Adt127, v1: u64, v2: Str) -> Adt145 {
    let m2255 = match v0.clone() {
        Adt127::C0(_, h2254, _) => {
            h2254
        }
    };
    match m2255.uncons() {
        None => {
            f509(Adt105::C2(v1))
        }
        Some((v3, v4)) => {
            let m2256 = v3;
            match m2256 {
                Adt132::C0(_) => {
                    f509(Adt105::C2(v1))
                }
                Adt132::C1(v6, _) => {
                    Adt145::C1(Adt127::C0(Adt128::C5, List::cons(Adt132::C1(v6, Some(v2)), v4), match v0 {
                        Adt127::C0(_, _, h2257) => {
                            h2257
                        }
                    }))
                }
            }
        }
    }
}

pub fn f668(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2258 = v0;
    let a2259 = 128u64;
    if nat_lt(a2258, a2259) {
        Ok(List::cons(v0, v1))
    } else {
        let a2260 = v0;
        let a2261 = 2048u64;
        if nat_lt(a2260, a2261) {
            Ok(List::cons({
                let a2262 = 128u64;
                let a2263 = f755(v0, 64u64);
                nat_add(a2262, a2263)?
            }, List::cons({
                let a2264 = 192u64;
                let a2265 = f756(v0, 64u64);
                nat_add(a2264, a2265)?
            }, v1)))
        } else {
            let a2266 = v0;
            let a2267 = 65536u64;
            if nat_lt(a2266, a2267) {
                Ok(List::cons({
                    let a2268 = 128u64;
                    let a2269 = f755(v0, 64u64);
                    nat_add(a2268, a2269)?
                }, List::cons({
                    let a2270 = 128u64;
                    let a2271 = f755(f756(v0, 64u64), 64u64);
                    nat_add(a2270, a2271)?
                }, List::cons({
                    let a2272 = 224u64;
                    let a2273 = f756(v0, 4096u64);
                    nat_add(a2272, a2273)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2274 = 128u64;
                    let a2275 = f755(v0, 64u64);
                    nat_add(a2274, a2275)?
                }, List::cons({
                    let a2276 = 128u64;
                    let a2277 = f755(f756(v0, 64u64), 64u64);
                    nat_add(a2276, a2277)?
                }, List::cons({
                    let a2278 = 128u64;
                    let a2279 = f755(f756(v0, 4096u64), 64u64);
                    nat_add(a2278, a2279)?
                }, List::cons({
                    let a2280 = 240u64;
                    let a2281 = f756(v0, 262144u64);
                    nat_add(a2280, a2281)?
                }, v1)))))
            }
        }
    }
}

pub fn f669() -> u64 {
    18u64
}

pub fn f670(p2285: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2284 = p2285;
    {
        let mut m2284 = m2284;
        loop {
            let step = {
                let v0 = m2284;
                let m2282 = v0;
                match m2282.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2283 = f347(v2.clone(), v1.clone());
                        match m2283 {
                            Adt110::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt110::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
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

pub fn f671() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f672(p2292: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2291 = p2292;
    {
        let mut m2291 = m2291;
        loop {
            let step = {
                let v0 = m2291;
                let m2286 = v0;
                match m2286.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2288 = f347({
                            let (h2287, _) = v2.clone();
                            h2287
                        }, v1.clone());
                        match m2288 {
                            Adt110::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt110::C1(v4) => {
                                if f757({
                                    let (_, h2289) = v2;
                                    h2289
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h2290, _) = v2;
                                        h2290
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f673() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f674(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt98> {
    let m2293 = f347(Str::lit("$ref"), v2);
    match m2293 {
        Adt110::C0 => {
            None::<Adt98>
        }
        Adt110::C1(v3) => {
            let m2294 = f133(v3);
            match m2294 {
                None => {
                    None::<Adt98>
                }
                Some(v4) => {
                    let m2295 = f758(v1, v4.clone());
                    match m2295 {
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

pub fn f675(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt133) -> R<Adt156> {
    if v3 {
        let m2296 = f133(v2);
        match m2296 {
            None => {
                Ok(Adt156::C0(Adt98::C7(v0)))
            }
            Some(v6) => {
                f759(v0, v1, v6.clone(), f226(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt156::C0(Adt98::C10(v0)))
    }
}

pub fn f676(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt113>) -> List<Adt113> {
    let m2297 = f347(v0.clone(), v1);
    match m2297 {
        Adt110::C0 => {
            v4
        }
        Adt110::C1(v5) => {
            let m2298 = f239(v5);
            match m2298 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a2299 = v0;
                    let a2300 = Str::lit("properties");
                    if equal(a2299, a2300) {
                        f760(match v6 {
                            Adt5::C0(h2301) => {
                                h2301
                            }
                        }, v2, v3, v4)
                    } else {
                        f761(match v6 {
                            Adt5::C0(h2302) => {
                                h2302
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f677(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt113>) -> List<Adt113> {
    let m2303 = f347(v0, v1);
    match m2303 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            List::cons(Adt113::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f678(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt113>) -> List<Adt113> {
    let m2304 = f347(v0, v1);
    match m2304 {
        Adt110::C0 => {
            v2
        }
        Adt110::C1(v3) => {
            let m2305 = f294(v3);
            match m2305 {
                None => {
                    v2
                }
                Some(v4) => {
                    f762(v4, v2)
                }
            }
        }
    }
}

pub fn f679(p2310: List<Adt51>, p2311: Adt7) -> R<bool> {
    let m2309 = (p2310, p2311);
    {
        let mut m2309 = m2309;
        loop {
            let step = {
                let (v0, v1) = m2309;
                let m2306 = v0;
                match m2306.uncons() {
                    None => {
                        Err::<(List<Adt51>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f763(f764(match v2.clone() {
                            Adt51::C0(h2307, _, _) => {
                                h2307
                            }
                        }, v1)?) {
                            Ok::<(List<Adt51>, Adt7), R<bool>>((v3, match v2 {
                                Adt51::C0(h2308, _, _) => {
                                    h2308
                                }
                            }))
                        } else {
                            Err::<(List<Adt51>, Adt7), R<bool>>(Ok(false))
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

pub fn f680(p2318: List<(Str, Adt157)>) -> Adt146 {
    let m2317 = p2318;
    {
        let mut m2317 = m2317;
        loop {
            let step = {
                let v0 = m2317;
                let m2312 = v0;
                match m2312.uncons() {
                    None => {
                        Err::<List<(Str, Adt157)>, Adt146>(Adt146::C3(List::<Adt117>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m2314 = {
                            let (_, h2313) = v1;
                            h2313
                        };
                        match m2314 {
                            Adt157::C0 => {
                                Ok::<List<(Str, Adt157)>, Adt146>(v2)
                            }
                            Adt157::C1 => {
                                Err::<List<(Str, Adt157)>, Adt146>(Adt146::C0({
                                    let (h2315, _) = v1;
                                    h2315
                                }))
                            }
                            Adt157::C2 => {
                                Err::<List<(Str, Adt157)>, Adt146>(Adt146::C1({
                                    let (h2316, _) = v1;
                                    h2316
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f681(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt157> {
    let m2319 = f347(Str::lit("type"), v0);
    match m2319 {
        Adt110::C0 => {
            Ok(Adt157::C0)
        }
        Adt110::C1(v2) => {
            let m2320 = f133(v2.clone());
            match m2320 {
                None => {
                    let m2321 = f294(v2);
                    match m2321 {
                        None => {
                            Ok(Adt157::C2)
                        }
                        Some(v4) => {
                            f767(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f765(v3.clone()) {
                        if f766(v3, v1)? {
                            Ok(Adt157::C0)
                        } else {
                            Ok(Adt157::C1)
                        }
                    } else {
                        Ok(Adt157::C2)
                    }
                }
            }
        }
    }
}

pub fn f682(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt157> {
    let m2322 = f347(Str::lit("enum"), v0);
    match m2322 {
        Adt110::C0 => {
            Ok(Adt157::C0)
        }
        Adt110::C1(v2) => {
            let m2323 = f294(v2);
            match m2323 {
                None => {
                    Ok(Adt157::C2)
                }
                Some(v3) => {
                    f768(v3, v1)
                }
            }
        }
    }
}

pub fn f683(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt157> {
    let m2324 = f347(Str::lit("const"), v0);
    match m2324 {
        Adt110::C0 => {
            Ok(Adt157::C0)
        }
        Adt110::C1(v2) => {
            let m2325 = f769(v2, v1)?;
            match m2325 {
                Adt161::C0 => {
                    Ok(Adt157::C0)
                }
                Adt161::C1 => {
                    Ok(Adt157::C1)
                }
                Adt161::C2 => {
                    Ok(Adt157::C2)
                }
            }
        }
    }
}

pub fn f684(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt157> {
    let m2326 = f347(v0, v2);
    match m2326 {
        Adt110::C0 => {
            Ok(Adt157::C0)
        }
        Adt110::C1(v4) => {
            let m2327 = f770(v4);
            match m2327 {
                None => {
                    Ok(Adt157::C2)
                }
                Some(v5) => {
                    let m2328 = f770(v3);
                    match m2328 {
                        None => {
                            Ok(Adt157::C0)
                        }
                        Some(v6) => {
                            if f771(v1, f764(v6, v5)?) {
                                Ok(Adt157::C0)
                            } else {
                                Ok(Adt157::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f685(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt157)>> {
    let m2329 = v1;
    match m2329 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt157)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt157)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt157)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f772(Str::lit("minLength"), true, v0.clone(), f773(v4.clone())?)?), List::cons((Str::lit("maxLength"), f772(Str::lit("maxLength"), false, v0, f773(v4)?)?), List::<(Str, Adt157)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f772(Str::lit("minItems"), true, v0.clone(), f774(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f772(Str::lit("maxItems"), false, v0, f774(v5, 0u64)?)?), List::<(Str, Adt157)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f772(Str::lit("minProperties"), true, v0.clone(), f775(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f772(Str::lit("maxProperties"), false, v0, f775(v6, 0u64)?)?), List::<(Str, Adt157)>::nil())))
        }
    }
}

pub fn f686(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt157 {
    let m2330 = f347(Str::lit("required"), v0);
    match m2330 {
        Adt110::C0 => {
            Adt157::C0
        }
        Adt110::C1(v2) => {
            let m2331 = f294(v2);
            match m2331 {
                None => {
                    Adt157::C2
                }
                Some(v3) => {
                    let m2332 = f239(v1);
                    match m2332 {
                        None => {
                            Adt157::C0
                        }
                        Some(v4) => {
                            f776(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f687(v0: Adt146, v1: Adt146) -> Adt146 {
    let m2333 = v0.clone();
    match m2333 {
        Adt146::C0(_) => {
            v0
        }
        Adt146::C1(_) => {
            v0
        }
        Adt146::C2(_) => {
            v0
        }
        Adt146::C3(v5) => {
            let m2334 = v1.clone();
            match m2334 {
                Adt146::C0(_) => {
                    v1
                }
                Adt146::C1(_) => {
                    v1
                }
                Adt146::C2(_) => {
                    v1
                }
                Adt146::C3(v9) => {
                    Adt146::C3({
                        let a2335 = v5;
                        let a2336 = v9;
                        append_list(a2335, a2336)
                    })
                }
            }
        }
    }
}

pub fn f688(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt115>) -> Adt146 {
    let m2337 = f347(Str::lit("$ref"), v1);
    match m2337 {
        Adt110::C0 => {
            Adt146::C3(List::<Adt117>::nil())
        }
        Adt110::C1(v4) => {
            let m2338 = f133(v4);
            match m2338 {
                None => {
                    Adt146::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m2339 = f758(v0, v5.clone());
                    match m2339 {
                        None => {
                            Adt146::C2(v5)
                        }
                        Some(v6) => {
                            Adt146::C3(List::cons(Adt117::C0(v6, v2, v3), List::<Adt117>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f689(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt115>) -> Adt146 {
    let m2340 = f347(Str::lit("properties"), v0);
    match m2340 {
        Adt110::C0 => {
            Adt146::C3(List::<Adt117>::nil())
        }
        Adt110::C1(v3) => {
            let m2341 = f239(v3);
            match m2341 {
                None => {
                    Adt146::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m2342 = f239(v1);
                    match m2342 {
                        None => {
                            Adt146::C3(List::<Adt117>::nil())
                        }
                        Some(v5) => {
                            Adt146::C3(f777(match v4 {
                                Adt5::C0(h2343) => {
                                    h2343
                                }
                            }, match v5 {
                                Adt5::C0(h2344) => {
                                    h2344
                                }
                            }, v2, List::<Adt117>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f690(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt115>) -> Adt146 {
    let m2345 = f347(Str::lit("additionalProperties"), v0.clone());
    match m2345 {
        Adt110::C0 => {
            Adt146::C3(List::<Adt117>::nil())
        }
        Adt110::C1(v3) => {
            let m2346 = f239(v1);
            match m2346 {
                None => {
                    Adt146::C3(List::<Adt117>::nil())
                }
                Some(v4) => {
                    let m2347 = f447(v3.clone());
                    match m2347 {
                        None => {
                            Adt146::C3(f780(match v4 {
                                Adt5::C0(h2349) => {
                                    h2349
                                }
                            }, f779(v0), v3, v2, List::<Adt117>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt146::C3(List::<Adt117>::nil())
                            } else {
                                if f778(match v4 {
                                    Adt5::C0(h2348) => {
                                        h2348
                                    }
                                }, f779(v0)) {
                                    Adt146::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt146::C3(List::<Adt117>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f691(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt115>) -> R<Adt146> {
    let m2350 = f347(Str::lit("items"), v0);
    match m2350 {
        Adt110::C0 => {
            Ok(Adt146::C3(List::<Adt117>::nil()))
        }
        Adt110::C1(v3) => {
            let m2351 = f294(v1);
            match m2351 {
                None => {
                    Ok(Adt146::C3(List::<Adt117>::nil()))
                }
                Some(v4) => {
                    Ok(Adt146::C3(f781(v4, 0u64, v3, v2, List::<Adt117>::nil())?))
                }
            }
        }
    }
}

pub fn f692(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt115>) -> Adt146 {
    let m2352 = f347(Str::lit("allOf"), v0);
    match m2352 {
        Adt110::C0 => {
            Adt146::C3(List::<Adt117>::nil())
        }
        Adt110::C1(v3) => {
            let m2353 = f294(v3);
            match m2353 {
                None => {
                    Adt146::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m2354 = v4.clone();
                    match m2354.uncons() {
                        None => {
                            Adt146::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt146::C3(f782(v4, v1, v2, List::<Adt117>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f693(v0: Adt118, v1: Adt146, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt115>) -> Adt146 {
    let m2355 = v1.clone();
    match m2355 {
        Adt146::C0(_) => {
            v1
        }
        Adt146::C1(_) => {
            v1
        }
        Adt146::C2(_) => {
            v1
        }
        Adt146::C3(v8) => {
            let m2356 = v8;
            match m2356.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt146::C3(List::cons(Adt117::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt117>::nil()))
                }
            }
        }
    }
}

pub fn f694(p2363: List<u64>, p2364: List<u64>) -> List<u64> {
    let m2362 = (p2363, p2364);
    {
        let mut m2362 = m2362;
        loop {
            let step = {
                let (v0, v1) = m2362;
                let m2357 = v0;
                match m2357.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a2358 = v2;
                        let a2359 = 126u64;
                        if nat_eq(a2358, a2359) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a2360 = v2;
                            let a2361 = 47u64;
                            if nat_eq(a2360, a2361) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
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

pub fn f695(p2369: List<u64>, p2370: u64, p2371: List<u64>) -> List<u64> {
    let m2368 = (p2369, (p2370, p2371));
    {
        let mut m2368 = m2368;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2368;
                let m2365 = v0;
                match m2365.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f783(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt159 = f784(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt159::C0(h2366, _) => {
                                h2366
                            }
                        }, match v5 {
                            Adt159::C0(_, h2367) => {
                                h2367
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f696(v0: Option<Adt41>) -> List<(Str, Adt6)> {
    let m2372 = v0;
    match m2372 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2378 = f785(match v1.clone() {
                    Adt41::C0(h2373, _, _) => {
                        h2373
                    }
                });
                let a2379 = {
                    let a2376 = f786(match v1.clone() {
                        Adt41::C0(_, h2374, _) => {
                            h2374
                        }
                    });
                    let a2377 = f698(Str::lit("lastModified"), match v1 {
                        Adt41::C0(_, _, h2375) => {
                            h2375
                        }
                    });
                    append_list(a2376, a2377)
                };
                append_list(a2378, a2379)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f697(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f740(Str::lit("_meta"), v0)
}

pub fn f698(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2380 = v1;
    match m2380 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f699(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2381 = v0;
    match m2381 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f787(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f700(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2382 = v0;
    match m2382 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f126(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f701(v0: Adt47) -> Adt6 {
    let m2383 = v0;
    match m2383 {
        Adt47::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt48::C0(h2384, _, _, _) => {
                    h2384
                }
            })), {
                let a2388 = f698(Str::lit("mimeType"), match v1.clone() {
                    Adt48::C0(_, h2385, _, _) => {
                        h2385
                    }
                });
                let a2389 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt48::C0(_, _, h2386, _) => {
                        h2386
                    }
                })), f697(match v1 {
                    Adt48::C0(_, _, _, h2387) => {
                        h2387
                    }
                }));
                append_list(a2388, a2389)
            }))
        }
        Adt47::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt49::C0(h2390, _, _, _) => {
                    h2390
                }
            })), {
                let a2394 = f698(Str::lit("mimeType"), match v2.clone() {
                    Adt49::C0(_, h2391, _, _) => {
                        h2391
                    }
                });
                let a2395 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt49::C0(_, _, h2392, _) => {
                        h2392
                    }
                })), f697(match v2 {
                    Adt49::C0(_, _, _, h2393) => {
                        h2393
                    }
                }));
                append_list(a2394, a2395)
            }))
        }
    }
}

pub fn f702() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f703(v0: Adt6) -> Option<Adt65> {
    let m2396 = f239(v0);
    match m2396 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2397 = f549(Str::lit("method"), v1.clone());
            match m2397 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a2398 = v2.clone();
                    let a2399 = Str::lit("elicitation/create");
                    if equal(a2398, a2399) {
                        let m2400 = f461(Str::lit("params"), v1);
                        match m2400 {
                            None => {
                                None::<Adt65>
                            }
                            Some(v3) => {
                                f788(v3)
                            }
                        }
                    } else {
                        let a2401 = v2;
                        let a2402 = Str::lit("roots/list");
                        if equal(a2401, a2402) {
                            f789(v1)
                        } else {
                            None::<Adt65>
                        }
                    }
                }
            }
        }
    }
}

pub fn f704() -> i64 {
    999999999999999999i64
}

pub fn f705(v0: i64) -> R<List<u64>> {
    let m2405 = {
        let a2403 = v0;
        let a2404 = 0i64;
        compare(a2403, a2404)
    };
    if match m2405 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f790(20u64, {
            let a2406 = v0;
            int_neg(a2406)?
        }, List::<u64>::nil())?))
    } else {
        f790(20u64, v0, List::<u64>::nil())
    }
}

pub fn f706(v0: u64) -> R<List<u64>> {
    let a2407 = v0;
    let a2408 = 34u64;
    if nat_eq(a2407, a2408) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2409 = v0;
        let a2410 = 92u64;
        if nat_eq(a2409, a2410) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2411 = v0;
            let a2412 = 8u64;
            if nat_eq(a2411, a2412) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2413 = v0;
                let a2414 = 9u64;
                if nat_eq(a2413, a2414) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2415 = v0;
                    let a2416 = 10u64;
                    if nat_eq(a2415, a2416) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2417 = v0;
                        let a2418 = 12u64;
                        if nat_eq(a2417, a2418) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2419 = v0;
                            let a2420 = 13u64;
                            if nat_eq(a2419, a2420) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2421 = v0;
                                let a2422 = 32u64;
                                if nat_lt(a2421, a2422) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f791(f756(v0, 16u64))?, List::cons(f791(f755(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f707(v0: List<Str>) -> Adt6 {
    Adt6::C4(f651(v0))
}

pub fn f708(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2440 = f740(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2423, _, _, _, _, _, _) => {
                h2423
            }
        });
        let a2441 = {
            let a2438 = f740(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2424, _, _, _, _, _) => {
                    h2424
                }
            });
            let a2439 = {
                let a2436 = f740(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2425, _, _, _, _) => {
                        h2425
                    }
                });
                let a2437 = {
                    let a2434 = f792(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2426, _, _, _) => {
                            h2426
                        }
                    });
                    let a2435 = {
                        let a2432 = f793(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2427, _, _) => {
                                h2427
                            }
                        });
                        let a2433 = {
                            let a2430 = f792(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2428, _) => {
                                    h2428
                                }
                            });
                            let a2431 = f740(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2429) => {
                                    h2429
                                }
                            });
                            append_list(a2430, a2431)
                        };
                        append_list(a2432, a2433)
                    };
                    append_list(a2434, a2435)
                };
                append_list(a2436, a2437)
            };
            append_list(a2438, a2439)
        };
        append_list(a2440, a2441)
    })
}

pub fn f709(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f126(match v0.clone() {
        Adt10::C0(h2442, _) => {
            h2442
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f794(match v0 {
        Adt10::C0(_, h2443) => {
            h2443
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f710(v0: List<Adt36>) -> List<Adt6> {
    f445(f795(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f711(v0: List<Adt45>) -> R<List<Adt6>> {
    Ok(f445(f796(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f712(v0: List<Adt47>) -> List<Adt6> {
    f445(f797(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f713(v0: List<Adt55>) -> List<Adt6> {
    f445(f798(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f714(v0: List<Adt57>) -> List<Adt6> {
    f445(f799(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f715(v0: List<Adt60>) -> R<Option<List<Adt6>>> {
    let m2444 = f800(v0, List::<Adt6>::nil())?;
    match m2444 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f445(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f716(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2445 = v1;
    match m2445 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f126(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f717(p2451: List<(Str, Adt65)>, p2452: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m2450 = (p2451, p2452);
    {
        let mut m2450 = m2450;
        loop {
            let step = {
                let (v0, v1) = m2450;
                let m2446 = v0;
                match m2446.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2448 = f801({
                            let (_, h2447) = v2;
                            h2447
                        })?;
                        match m2448 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h2449, _) = v2;
                                    h2449
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: Adt82) -> List<(Str, Adt65)> {
    let m2453 = v0;
    match m2453 {
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

pub fn f719(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2454 = v0.clone();
    match m2454.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f535(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f720(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2455 = v0;
    match m2455 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f721(v0: Adt82) -> Option<Str> {
    let m2456 = v0;
    match m2456 {
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

pub fn f722(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m2457 = v0;
    match m2457 {
        Adt63::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt63::C1(_, v3) => {
            let m2458 = f802(v3);
            match m2458 {
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
                Adt5::C0(h2459) => {
                    h2459
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f312(v8)), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C4(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f723(v0: Adt64, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt64::C0(h2460, _, _, _, _, _) => {
            h2460
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a2466 = f803(Str::lit("statusMessage"), match v0.clone() {
            Adt64::C0(_, h2461, _, _, _, _) => {
                h2461
            }
        });
        let a2467 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, h2462, _, _, _) => {
                h2462
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, _, h2463, _, _) => {
                h2463
            }
        })), List::cons((Str::lit("ttlMs"), f804(match v0.clone() {
            Adt64::C0(_, _, _, _, h2464, _) => {
                h2464
            }
        })), f805(Str::lit("pollIntervalMs"), match v0 {
            Adt64::C0(_, _, _, _, _, h2465) => {
                h2465
            }
        }))));
        append_list(a2466, a2467)
    }))
}

pub fn f724(v0: Adt84) -> Str {
    let m2468 = v0;
    match m2468 {
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

pub fn f725(v0: Adt63) -> Adt84 {
    let m2469 = v0;
    match m2469 {
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

pub fn f726() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f727(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2470, _, _, _, _, _) => {
            h2470
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2471, _, _, _) => {
            h2471
        }
    })), {
        let a2480 = f698(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2472, _, _, _, _) => {
                h2472
            }
        });
        let a2481 = {
            let a2478 = f698(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2473, _, _) => {
                    h2473
                }
            });
            let a2479 = {
                let a2476 = f698(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2474, _) => {
                        h2474
                    }
                });
                let a2477 = f699(match v0 {
                    Adt1::C0(_, _, _, _, _, h2475) => {
                        h2475
                    }
                });
                append_list(a2476, a2477)
            };
            append_list(a2478, a2479)
        };
        append_list(a2480, a2481)
    })))
}

pub fn f728(v0: List<Adt85>) -> Option<Adt6> {
    let m2482 = f806(v0, List::<Adt6>::nil());
    match m2482 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f445(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f729(v0: Str, p2487: List<Adt85>, v2: i64, p2488: List<Adt85>) -> R<List<Adt85>> {
    let m2486 = (p2487, p2488);
    {
        let mut m2486 = m2486;
        loop {
            let step = {
                let (v1, v3) = m2486;
                let m2483 = v1;
                match m2483.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a2484 = f296(v4.clone());
                        let a2485 = v0.clone();
                        if if equal(a2484, a2485) {
                            true
                        } else {
                            f297(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, List::cons(v4, v3)))
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

pub fn f730(v0: List<List<u64>>) -> bool {
    let m2489 = v0;
    match m2489.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f731() -> List<Adt93> {
    List::cons(Adt93::C0(f807(), f654()), List::cons(Adt93::C0(f808(), f809()), List::cons(Adt93::C0(f810(), f811()), List::<Adt93>::nil())))
}

pub fn f732(v0: List<List<u64>>) -> List<u64> {
    f812(f45(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f733(v0: Adt20) -> Adt6 {
    let m2490 = v0;
    match m2490 {
        Adt20::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt20::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f734(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2491 = v0;
    match m2491 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f735(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2492 = v0;
    match m2492 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f736() -> i64 {
    -32603i64
}

pub fn f737() -> i64 {
    429i64
}

pub fn f738() -> i64 {
    -32021i64
}

pub fn f739(p2495: List<Str>, p2496: List<Adt6>) -> List<Adt6> {
    let m2494 = (p2495, p2496);
    {
        let mut m2494 = m2494;
        loop {
            let step = {
                let (v0, v1) = m2494;
                let m2493 = v0;
                match m2493.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f740(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2497 = v1;
    match m2497 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2498) => {
                    h2498
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f741(v0: List<u64>) -> R<bool> {
    let m2499 = v0;
    match m2499.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2500 = v2;
            match m2500.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2501 = f421(v1)?;
                    let a2502 = 113u64;
                    if if nat_eq(a2501, a2502) {
                        let a2503 = v3;
                        let a2504 = 61u64;
                        nat_eq(a2503, a2504)
                    } else {
                        false
                    } {
                        Ok(f813(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f742(v0: Adt0, v1: Fn4, v2: u64, v3: Result<Adt15, Adt142>) -> R<Result<Adt138, Adt141>> {
    let m2505 = v3;
    match m2505 {
        Ok(v5) => {
            f814(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt138, Adt141>(Adt141::C0(v4)))
        }
    }
}

pub fn f743(v0: Adt150, v1: Adt17, v2: Adt5) -> Result<Adt15, Adt142> {
    let m2506 = v0;
    match m2506 {
        Adt150::C0 => {
            f815(v1, v2)
        }
        Adt150::C1 => {
            f816(v1, v2)
        }
        Adt150::C2 => {
            f817(v1, v2)
        }
        Adt150::C3 => {
            f818(v1, v2)
        }
        Adt150::C4 => {
            f819(v1, v2)
        }
        Adt150::C5 => {
            f820(v1, v2)
        }
        Adt150::C6 => {
            f821(v1, v2)
        }
        Adt150::C7 => {
            f822(v1, v2)
        }
        Adt150::C8 => {
            f823(v1, v2)
        }
        Adt150::C9 => {
            f824(v1, v2)
        }
        Adt150::C10 => {
            f825(v1, v2)
        }
        Adt150::C11 => {
            f826(v1, v2)
        }
    }
}

pub fn f744(v0: Adt150) -> Str {
    let m2507 = v0;
    match m2507 {
        Adt150::C0 => {
            Str::lit("server/discover")
        }
        Adt150::C1 => {
            Str::lit("tools/list")
        }
        Adt150::C2 => {
            Str::lit("tools/call")
        }
        Adt150::C3 => {
            Str::lit("resources/list")
        }
        Adt150::C4 => {
            Str::lit("resources/read")
        }
        Adt150::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt150::C6 => {
            Str::lit("prompts/list")
        }
        Adt150::C7 => {
            Str::lit("prompts/get")
        }
        Adt150::C8 => {
            Str::lit("completion/complete")
        }
        Adt150::C9 => {
            Str::lit("tasks/get")
        }
        Adt150::C10 => {
            Str::lit("tasks/update")
        }
        Adt150::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f745(v0: Option<Adt8>) -> bool {
    let m2508 = v0;
    match m2508 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f746(v0: Option<Adt9>) -> bool {
    let m2509 = v0;
    match m2509 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f747(v0: List<Str>, v1: Adt112, v2: Adt17, v3: Option<Adt121>) -> Adt153 {
    let m2510 = v3;
    match m2510 {
        None => {
            f827(v1, v2.clone(), f828(match v2 {
                Adt17::C0(h2511, _, _, _, _, _) => {
                    h2511
                }
            }, v0))
        }
        Some(v4) => {
            Adt153::C1(Adt141::C2(v4))
        }
    }
}

pub fn f748(v0: Option<Adt120>, v1: Adt155, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt121>> {
    let m2512 = v0;
    match m2512 {
        None => {
            Ok(None::<Adt121>)
        }
        Some(v5) => {
            let m2514 = match v5.clone() {
                Adt120::C0(_, _, _, h2513, _) => {
                    h2513
                }
            };
            match m2514 {
                None => {
                    f829(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f749(v0: Option<Adt5>) -> Adt5 {
    let m2515 = v0;
    match m2515 {
        None => {
            f27()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f750(v0: Adt5) -> Result<Adt17, Adt142> {
    let m2516 = f830(v0.clone());
    match m2516 {
        Adt162::C0 => {
            f831(v0.clone(), f234(f832(), v0.clone()), f234(f833(), v0))
        }
        Adt162::C1 => {
            Err::<Adt17, Adt142>(Adt142::C2)
        }
        Adt162::C2 => {
            Err::<Adt17, Adt142>(Adt142::C3)
        }
    }
}

pub fn f751(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2517 = f834(v0)?;
    match m2517 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2518 = f834(v1)?;
            match m2518 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2519 = v2;
                    let a2520 = 61u64;
                    if nat_eq(a2519, a2520) {
                        f835(v5, v6, v4, v3)
                    } else {
                        let m2521 = f834(v2)?;
                        match m2521 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2522 = v3;
                                let a2523 = 61u64;
                                if nat_eq(a2522, a2523) {
                                    f836(v5, v6, v7, v4)
                                } else {
                                    let m2524 = f834(v3)?;
                                    match m2524 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f837(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f752(v0: List<u64>) -> bool {
    let m2525 = v0;
    match m2525.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f753() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f754(v0: Str) -> R<List<u64>> {
    f198({
        let a2526 = v0;
        utf8_encode(a2526)
    })
}

pub fn f755(v0: u64, v1: u64) -> u64 {
    let a2527 = v0;
    let a2528 = v1;
    let a2529 = 0u64;
    nat_rem(a2527, a2528, a2529)
}

pub fn f756(v0: u64, v1: u64) -> u64 {
    let a2530 = v0;
    let a2531 = v1;
    let a2532 = 0u64;
    nat_quot(a2530, a2531, a2532)
}

pub fn f757(v0: u64, v1: Adt6) -> bool {
    let a2533 = v0;
    let a2534 = 0u64;
    if nat_eq(a2533, a2534) {
        f838(v1)
    } else {
        let a2535 = v0;
        let a2536 = 1u64;
        if nat_eq(a2535, a2536) {
            f839(v1)
        } else {
            let a2537 = v0;
            let a2538 = 2u64;
            if nat_eq(a2537, a2538) {
                f840(v1)
            } else {
                let a2539 = v0;
                let a2540 = 3u64;
                if nat_eq(a2539, a2540) {
                    f841(v1)
                } else {
                    let a2541 = v0;
                    let a2542 = 4u64;
                    if nat_eq(a2541, a2542) {
                        f842(v1)
                    } else {
                        let a2543 = v0;
                        let a2544 = 5u64;
                        if nat_eq(a2543, a2544) {
                            f843(v1)
                        } else {
                            let a2545 = v0;
                            let a2546 = 6u64;
                            if nat_eq(a2545, a2546) {
                                f844(v1)
                            } else {
                                let a2547 = v0;
                                let a2548 = 7u64;
                                if nat_eq(a2547, a2548) {
                                    f845(v1)
                                } else {
                                    f846(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f758(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a2549 = v1.clone();
    let a2550 = Str::lit("#");
    if equal(a2549, a2550) {
        Some(v0)
    } else {
        let m2551 = f239(v0);
        match m2551 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m2552 = f234(Str::lit("$defs"), v2);
                match m2552 {
                    Adt110::C0 => {
                        None::<Adt6>
                    }
                    Adt110::C1(v3) => {
                        let m2553 = f239(v3);
                        match m2553 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f847(v1, match v4 {
                                    Adt5::C0(h2554) => {
                                        h2554
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

pub fn f759(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt133) -> R<Adt156> {
    let m2555 = v3.clone();
    match m2555.uncons() {
        None => {
            Ok(Adt156::C0(Adt98::C8(v0)))
        }
        Some((_, _)) => {
            if f425(v3.clone(), true) {
                let m2556 = f848(v1);
                match m2556 {
                    None => {
                        Ok(Adt156::C0(Adt98::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f849(v3)?;
                        if f850(v9.clone(), match v5.clone() {
                            Adt133::C0(_, h2557) => {
                                h2557
                            }
                        }) {
                            Ok(Adt156::C0(Adt98::C12(v0, v2)))
                        } else {
                            Ok(Adt156::C1(Adt133::C0(List::cons(Adt12::C0(v0, v2, f363(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt133::C0(h2558, _) => {
                                    h2558
                                }
                            }), List::cons(v9, match v5 {
                                Adt133::C0(_, h2559) => {
                                    h2559
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt156::C0(Adt98::C9(v0)))
            }
        }
    }
}

pub fn f760(p2564: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2565: List<Adt113>) -> List<Adt113> {
    let m2563 = (p2564, p2565);
    {
        let mut m2563 = m2563;
        loop {
            let step = {
                let (v0, v3) = m2563;
                let m2560 = v0;
                match m2560.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v5, List::cons(Adt113::C0({
                            let (_, h2561) = v4;
                            h2561
                        }, v1, List::cons({
                            let (h2562, _) = v4;
                            h2562
                        }, v2.clone())), v3)))
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

pub fn f761(p2569: List<(Str, Adt6)>, p2570: List<Adt113>) -> List<Adt113> {
    let m2568 = (p2569, p2570);
    {
        let mut m2568 = m2568;
        loop {
            let step = {
                let (v0, v1) = m2568;
                let m2566 = v0;
                match m2566.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v3, List::cons(Adt113::C0({
                            let (_, h2567) = v2;
                            h2567
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f762(p2573: List<Adt6>, p2574: List<Adt113>) -> List<Adt113> {
    let m2572 = (p2573, p2574);
    {
        let mut m2572 = m2572;
        loop {
            let step = {
                let (v0, v1) = m2572;
                let m2571 = v0;
                match m2571.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt113>), List<Adt113>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt113>), List<Adt113>>((v3, List::cons(Adt113::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f763(v0: Adt160) -> bool {
    let m2575 = v0;
    match m2575 {
        Adt160::C0 => {
            false
        }
        Adt160::C1 => {
            false
        }
        Adt160::C2 => {
            true
        }
    }
}

pub fn f764(v0: Adt7, v1: Adt7) -> R<Adt160> {
    f851(v0.clone(), v1.clone(), f852(match v0 {
        Adt7::C0(h2576, _) => {
            h2576
        }
    }), f852(match v1 {
        Adt7::C0(h2577, _) => {
            h2577
        }
    }))
}

pub fn f765(v0: Str) -> bool {
    let a2578 = v0.clone();
    let a2579 = Str::lit("null");
    if equal(a2578, a2579) {
        true
    } else {
        let a2580 = v0.clone();
        let a2581 = Str::lit("boolean");
        if equal(a2580, a2581) {
            true
        } else {
            let a2582 = v0.clone();
            let a2583 = Str::lit("object");
            if equal(a2582, a2583) {
                true
            } else {
                let a2584 = v0.clone();
                let a2585 = Str::lit("array");
                if equal(a2584, a2585) {
                    true
                } else {
                    let a2586 = v0.clone();
                    let a2587 = Str::lit("number");
                    if equal(a2586, a2587) {
                        true
                    } else {
                        let a2588 = v0.clone();
                        let a2589 = Str::lit("string");
                        if equal(a2588, a2589) {
                            true
                        } else {
                            let a2590 = v0;
                            let a2591 = Str::lit("integer");
                            equal(a2590, a2591)
                        }
                    }
                }
            }
        }
    }
}

pub fn f766(v0: Str, v1: Adt6) -> R<bool> {
    let a2592 = v0.clone();
    let a2593 = f853(v1.clone());
    if equal(a2592, a2593) {
        Ok(true)
    } else {
        let a2594 = v0;
        let a2595 = Str::lit("integer");
        if equal(a2594, a2595) {
            let m2596 = f770(v1);
            match m2596 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f854(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f767(p2599: List<Adt6>, v1: Adt6, p2600: bool) -> R<Adt157> {
    let m2598 = (p2599, p2600);
    {
        let mut m2598 = m2598;
        loop {
            let step = {
                let (v0, v2) = m2598;
                let m2597 = v0;
                match m2597.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt157>>(Ok(Adt157::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt157>>(Ok(Adt157::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f765(f855(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt157>>((v4, if v2 {
                                true
                            } else {
                                f766(f855(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt157>>(Ok(Adt157::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2598 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f768(p2604: List<Adt6>, v1: Adt6) -> R<Adt157> {
    let m2603 = p2604;
    {
        let mut m2603 = m2603;
        loop {
            let step = {
                let v0 = m2603;
                let m2601 = v0;
                match m2601.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt157>>(Ok(Adt157::C1))
                    }
                    Some((v2, v3)) => {
                        let m2602 = f769(v2, v1.clone())?;
                        match m2602 {
                            Adt161::C0 => {
                                Err::<List<Adt6>, R<Adt157>>(Ok(Adt157::C0))
                            }
                            Adt161::C1 => {
                                Ok::<List<Adt6>, R<Adt157>>(v3)
                            }
                            Adt161::C2 => {
                                Err::<List<Adt6>, R<Adt157>>(Ok(Adt157::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2603 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f769(v0: Adt6, v1: Adt6) -> R<Adt161> {
    f856(f857(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f770(v0: Adt6) -> Option<Adt7> {
    let m2605 = v0;
    match m2605 {
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

pub fn f771(v0: u64, v1: Adt160) -> bool {
    let a2606 = v0;
    let a2607 = 0u64;
    if nat_eq(a2606, a2607) {
        let a2608 = f858(v1);
        bool_not(a2608)
    } else {
        let a2609 = v0;
        let a2610 = 1u64;
        if nat_eq(a2609, a2610) {
            let a2611 = f763(v1);
            bool_not(a2611)
        } else {
            let a2612 = v0;
            let a2613 = 2u64;
            if nat_eq(a2612, a2613) {
                f763(v1)
            } else {
                f858(v1)
            }
        }
    }
}

pub fn f772(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt157> {
    let m2614 = f347(v0, v2);
    match m2614 {
        Adt110::C0 => {
            Ok(Adt157::C0)
        }
        Adt110::C1(v4) => {
            let m2615 = f235(v4);
            match m2615 {
                None => {
                    Ok(Adt157::C2)
                }
                Some(v5) => {
                    let m2618 = {
                        let a2616 = v5;
                        let a2617 = 0i64;
                        compare(a2616, a2617)
                    };
                    if match m2618 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt157::C2)
                    } else {
                        if v1 {
                            if f858(f859(f354(v3, 0i64)?, v5)) {
                                Ok(Adt157::C1)
                            } else {
                                Ok(Adt157::C0)
                            }
                        } else {
                            if f763(f859(f354(v3, 0i64)?, v5)) {
                                Ok(Adt157::C1)
                            } else {
                                Ok(Adt157::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f773(v0: Str) -> R<u64> {
    f860(f198({
        let a2619 = v0;
        utf8_encode(a2619)
    })?, 0u64)
}

pub fn f774(p2624: List<Adt6>, p2625: u64) -> R<u64> {
    let m2623 = (p2624, p2625);
    {
        let mut m2623 = m2623;
        loop {
            let step = {
                let (v0, v1) = m2623;
                let m2620 = v0;
                match m2620.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a2621 = v1;
                            let a2622 = 1u64;
                            nat_add(a2621, a2622)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f775(p2630: List<(Str, Adt6)>, p2631: u64) -> R<u64> {
    let m2629 = (p2630, p2631);
    {
        let mut m2629 = m2629;
        loop {
            let step = {
                let (v0, v1) = m2629;
                let m2626 = v0;
                match m2626.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a2627 = v1;
                            let a2628 = 1u64;
                            nat_add(a2627, a2628)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2629 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f776(p2635: List<Adt6>, v1: Adt5) -> Adt157 {
    let m2634 = p2635;
    {
        let mut m2634 = m2634;
        loop {
            let step = {
                let v0 = m2634;
                let m2632 = v0;
                match m2632.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt157>(Adt157::C0)
                    }
                    Some((v2, v3)) => {
                        let m2633 = f133(v2);
                        match m2633 {
                            None => {
                                Err::<List<Adt6>, Adt157>(Adt157::C2)
                            }
                            Some(v4) => {
                                if f384(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt157>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt157>(Adt157::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2634 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f777(p2642: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt115>, p2643: List<Adt117>) -> List<Adt117> {
    let m2641 = (p2642, p2643);
    {
        let mut m2641 = m2641;
        loop {
            let step = {
                let (v0, v3) = m2641;
                let m2636 = v0;
                match m2636.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt117>), List<Adt117>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2638 = f347({
                            let (h2637, _) = v4.clone();
                            h2637
                        }, v1.clone());
                        match m2638 {
                            Adt110::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt117>), List<Adt117>>((v5, v3))
                            }
                            Adt110::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt117>), List<Adt117>>((v5, List::cons(Adt117::C0({
                                    let (_, h2639) = v4;
                                    h2639
                                }, v6, List::cons(Adt115::C0({
                                    let (h2640, _) = v4;
                                    h2640
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f778(p2648: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2647 = p2648;
    {
        let mut m2647 = m2647;
        loop {
            let step = {
                let v0 = m2647;
                let m2644 = v0;
                match m2644.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2646 = f347({
                            let (h2645, _) = v2;
                            h2645
                        }, v1.clone());
                        match m2646 {
                            Adt110::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt110::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f779(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2649 = f347(Str::lit("properties"), v0);
    match m2649 {
        Adt110::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt110::C1(v1) => {
            let m2650 = f239(v1);
            match m2650 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2651) => {
                            h2651
                        }
                    }
                }
            }
        }
    }
}

pub fn f780(p2658: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt115>, p2659: List<Adt117>) -> List<Adt117> {
    let m2657 = (p2658, p2659);
    {
        let mut m2657 = m2657;
        loop {
            let step = {
                let (v0, v4) = m2657;
                let m2652 = v0;
                match m2652.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt117>), List<Adt117>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2654 = f347({
                            let (h2653, _) = v5.clone();
                            h2653
                        }, v1.clone());
                        match m2654 {
                            Adt110::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt117>), List<Adt117>>((v6, List::cons(Adt117::C0(v2.clone(), {
                                    let (_, h2655) = v5;
                                    h2655
                                }, List::cons(Adt115::C0({
                                    let (h2656, _) = v5;
                                    h2656
                                }), v3.clone())), v4)))
                            }
                            Adt110::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt117>), List<Adt117>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f781(p2664: List<Adt6>, p2665: u64, v2: Adt6, v3: List<Adt115>, p2666: List<Adt117>) -> R<List<Adt117>> {
    let m2663 = (p2664, (p2665, p2666));
    {
        let mut m2663 = m2663;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2663;
                let m2660 = v0;
                match m2660.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt117>)), R<List<Adt117>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt117>)), R<List<Adt117>>>((v6, ({
                            let a2661 = v1;
                            let a2662 = 1u64;
                            nat_add(a2661, a2662)?
                        }, List::cons(Adt117::C0(v2.clone(), v5, List::cons(Adt115::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2663 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f782(p2669: List<Adt6>, v1: Adt6, v2: List<Adt115>, p2670: List<Adt117>) -> List<Adt117> {
    let m2668 = (p2669, p2670);
    {
        let mut m2668 = m2668;
        loop {
            let step = {
                let (v0, v3) = m2668;
                let m2667 = v0;
                match m2667.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt117>), List<Adt117>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt117>), List<Adt117>>((v5, List::cons(Adt117::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f783(v0: u64, v1: List<u64>) -> List<u64> {
    let a2671 = v0;
    let a2672 = 0u64;
    if nat_eq(a2671, a2672) {
        v1
    } else {
        let a2673 = v0;
        let a2674 = 1u64;
        if nat_eq(a2673, a2674) {
            List::cons(194u64, v1)
        } else {
            let a2675 = v0;
            let a2676 = 2u64;
            if nat_eq(a2675, a2676) {
                List::cons(226u64, v1)
            } else {
                let a2677 = v0;
                let a2678 = 3u64;
                if nat_eq(a2677, a2678) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f784(v0: u64, v1: u64, v2: List<u64>) -> Adt159 {
    let a2679 = v0;
    let a2680 = 0u64;
    if nat_eq(a2679, a2680) {
        f861(v1, v2)
    } else {
        let a2681 = v0;
        let a2682 = 1u64;
        if nat_eq(a2681, a2682) {
            let a2683 = 128u64;
            let a2684 = v1;
            if if nat_le(a2683, a2684) {
                let a2685 = v1;
                let a2686 = 159u64;
                nat_le(a2685, a2686)
            } else {
                false
            } {
                Adt159::C0(0u64, f862(v2))
            } else {
                f861(v1, List::cons(194u64, v2))
            }
        } else {
            let a2687 = v0;
            let a2688 = 2u64;
            if nat_eq(a2687, a2688) {
                let a2689 = v1;
                let a2690 = 128u64;
                if nat_eq(a2689, a2690) {
                    Adt159::C0(3u64, v2)
                } else {
                    let a2691 = v1;
                    let a2692 = 129u64;
                    if nat_eq(a2691, a2692) {
                        Adt159::C0(4u64, v2)
                    } else {
                        f861(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2693 = v0;
                let a2694 = 3u64;
                if nat_eq(a2693, a2694) {
                    let a2695 = 170u64;
                    let a2696 = v1;
                    if if nat_le(a2695, a2696) {
                        let a2697 = v1;
                        let a2698 = 174u64;
                        nat_le(a2697, a2698)
                    } else {
                        false
                    } {
                        Adt159::C0(0u64, f862(v2))
                    } else {
                        f861(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2699 = 166u64;
                    let a2700 = v1;
                    if if nat_le(a2699, a2700) {
                        let a2701 = v1;
                        let a2702 = 169u64;
                        nat_le(a2701, a2702)
                    } else {
                        false
                    } {
                        Adt159::C0(0u64, f862(v2))
                    } else {
                        f861(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f785(v0: Option<List<Adt42>>) -> List<(Str, Adt6)> {
    let m2703 = v0;
    match m2703 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f863(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f786(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2704 = v0;
    match m2704 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f787(v0: List<Adt2>) -> List<Adt6> {
    f445(f864(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f788(v0: Adt5) -> Option<Adt65> {
    let m2705 = f549(Str::lit("message"), v0.clone());
    match m2705 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2706 = f234(Str::lit("mode"), v0.clone());
            match m2706 {
                Adt110::C0 => {
                    let m2707 = f461(Str::lit("requestedSchema"), v0);
                    match m2707 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v2) => {
                            Some(Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v1, v2))))
                        }
                    }
                }
                Adt110::C1(v3) => {
                    let m2708 = f133(v3);
                    match m2708 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v4) => {
                            let a2709 = v4.clone();
                            let a2710 = Str::lit("url");
                            if equal(a2709, a2710) {
                                let m2711 = f549(Str::lit("url"), v0);
                                match m2711 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v5) => {
                                        Some(Adt65::C0(Adt66::C1(Adt69::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a2712 = v4;
                                let a2713 = Str::lit("form");
                                if equal(a2712, a2713) {
                                    let m2714 = f461(Str::lit("requestedSchema"), v0);
                                    match m2714 {
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

pub fn f789(v0: Adt5) -> Option<Adt65> {
    let m2715 = f234(Str::lit("params"), v0);
    match m2715 {
        Adt110::C0 => {
            Some(Adt65::C2(None::<Adt5>))
        }
        Adt110::C1(v1) => {
            let m2716 = f239(v1);
            match m2716 {
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

pub fn f790(p2728: u64, p2729: i64, p2730: List<u64>) -> R<List<u64>> {
    let m2727 = (p2728, (p2729, p2730));
    {
        let mut m2727 = m2727;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2727;
                let m2717 = v0;
                if m2717 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2717 - 1;
                    let v4: i64 = {
                        let a2718 = v1;
                        let a2719 = 10i64;
                        let a2720 = 0i64;
                        int_quot(a2718, a2719, a2720)?
                    };
                    let v5: i64 = {
                        let a2721 = v1;
                        let a2722 = 10i64;
                        let a2723 = 0i64;
                        int_rem(a2721, a2722, a2723)
                    };
                    let m2726 = {
                        let a2724 = v4;
                        let a2725 = 1i64;
                        compare(a2724, a2725)
                    };
                    if match m2726 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f865(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f865(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2727 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f791(v0: u64) -> R<u64> {
    let a2731 = v0;
    let a2732 = 10u64;
    if nat_lt(a2731, a2732) {
        let a2733 = 48u64;
        let a2734 = v0;
        nat_add(a2733, a2734)
    } else {
        let a2735 = 87u64;
        let a2736 = v0;
        nat_add(a2735, a2736)
    }
}

pub fn f792(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2737 = v1;
    match m2737 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f866(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f793(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2738 = v0;
    match m2738 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2741 = f352(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2739, _) => {
                        h2739
                    }
                });
                let a2742 = f352(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2740) => {
                        h2740
                    }
                });
                append_list(a2741, a2742)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f794(v0: Adt11) -> Str {
    let m2743 = v0;
    match m2743 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f795(p2746: List<Adt36>, p2747: List<Adt6>) -> List<Adt6> {
    let m2745 = (p2746, p2747);
    {
        let mut m2745 = m2745;
        loop {
            let step = {
                let (v0, v1) = m2745;
                let m2744 = v0;
                match m2744.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt36>, List<Adt6>), List<Adt6>>((v3, List::cons(f867(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2745 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f796(p2750: List<Adt45>, p2751: List<Adt6>) -> R<List<Adt6>> {
    let m2749 = (p2750, p2751);
    {
        let mut m2749 = m2749;
        loop {
            let step = {
                let (v0, v1) = m2749;
                let m2748 = v0;
                match m2748.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f868(v2)?, v1)))
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

pub fn f797(p2754: List<Adt47>, p2755: List<Adt6>) -> List<Adt6> {
    let m2753 = (p2754, p2755);
    {
        let mut m2753 = m2753;
        loop {
            let step = {
                let (v0, v1) = m2753;
                let m2752 = v0;
                match m2752.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), List<Adt6>>((v3, List::cons(f701(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2753 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f798(p2758: List<Adt55>, p2759: List<Adt6>) -> List<Adt6> {
    let m2757 = (p2758, p2759);
    {
        let mut m2757 = m2757;
        loop {
            let step = {
                let (v0, v1) = m2757;
                let m2756 = v0;
                match m2756.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt55>, List<Adt6>), List<Adt6>>((v3, List::cons(f869(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2757 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f799(p2762: List<Adt57>, p2763: List<Adt6>) -> List<Adt6> {
    let m2761 = (p2762, p2763);
    {
        let mut m2761 = m2761;
        loop {
            let step = {
                let (v0, v1) = m2761;
                let m2760 = v0;
                match m2760.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f870(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f800(p2767: List<Adt60>, p2768: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2766 = (p2767, p2768);
    {
        let mut m2766 = m2766;
        loop {
            let step = {
                let (v0, v1) = m2766;
                let m2764 = v0;
                match m2764.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2765 = f871(v2)?;
                        match m2765 {
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
                Ok(next) => { m2766 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f801(v0: Adt65) -> R<Option<Adt6>> {
    let m2769 = v0;
    match m2769 {
        Adt65::C0(v1) => {
            Ok(Some(f872(Str::lit("elicitation/create"), Some(f873(v1)))))
        }
        Adt65::C1(v2) => {
            let m2770 = f874(v2)?;
            match m2770 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f872(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt65::C2(v4) => {
            Ok(Some(f872(Str::lit("roots/list"), {
                let m2771 = v4;
                match m2771 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h2772) => {
                                h2772
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f802(v0: List<(Str, Adt65)>) -> Option<Adt6> {
    let m2773 = f875(v0, List::<(Str, Adt6)>::nil());
    match m2773 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f535(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f803(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2774 = v1;
    match m2774 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f804(v0: Option<i64>) -> Adt6 {
    let m2775 = v0;
    match m2775 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f876(v1)
        }
    }
}

pub fn f805(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m2776 = v1;
    match m2776 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f876(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f806(p2780: List<Adt85>, p2781: List<Adt6>) -> Option<List<Adt6>> {
    let m2779 = (p2780, p2781);
    {
        let mut m2779 = m2779;
        loop {
            let step = {
                let (v0, v1) = m2779;
                let m2777 = v0;
                match m2777.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2778 = f877(v2);
                        match m2778 {
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
                Ok(next) => { m2779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f807() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f808() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f809() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f810() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f811() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f812(p2786: List<List<u64>>, p2787: List<u64>) -> List<u64> {
    let m2785 = (p2786, p2787);
    {
        let mut m2785 = m2785;
        loop {
            let step = {
                let (v0, v1) = m2785;
                let m2782 = v0;
                match m2782.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2783 = f878(v2);
                            let a2784 = v1;
                            append_list(a2783, a2784)
                        }))
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

pub fn f813(v0: List<u64>) -> bool {
    let m2788 = v0;
    match m2788.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2789 = v1;
            let a2790 = 48u64;
            if nat_eq(a2789, a2790) {
                f879(v2)
            } else {
                false
            }
        }
    }
}

pub fn f814(v0: Adt0, v1: Fn4, v2: u64, v3: Adt15) -> R<Result<Adt138, Adt141>> {
    if f880(f881(v3.clone())) {
        Ok(Ok::<Adt138, Adt141>(f882(v0.clone(), Adt33::C0(f883(v0)))))
    } else {
        Ok(f884(v0.clone(), v3.clone(), f885(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2791, _) => {
                h2791
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2792, _, _) => {
                h2792
            }
        }, v2, v1, v3)?))
    }
}

pub fn f815(v0: Adt17, _: Adt5) -> Result<Adt15, Adt142> {
    Ok::<Adt15, Adt142>(Adt15::C0(Adt16::C0(v0)))
}

pub fn f816(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2793 = f886(Str::lit("cursor"), v1);
    match m2793 {
        Ok(v2) => {
            Ok::<Adt15, Adt142>(Adt15::C1(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt142>(v3)
        }
    }
}

pub fn f817(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2794 = f887(Str::lit("name"), v1.clone());
    match m2794 {
        Ok(v2) => {
            let m2795 = f888(Str::lit("arguments"), v1.clone());
            match m2795 {
                Ok(v3) => {
                    let m2796 = f888(Str::lit("inputResponses"), v1.clone());
                    match m2796 {
                        Ok(v4) => {
                            let m2797 = f886(Str::lit("requestState"), v1);
                            match m2797 {
                                Ok(v5) => {
                                    Ok::<Adt15, Adt142>(Adt15::C2(Adt22::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt15, Adt142>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt15, Adt142>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt15, Adt142>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt15, Adt142>(v9)
        }
    }
}

pub fn f818(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2798 = f886(Str::lit("cursor"), v1);
    match m2798 {
        Ok(v2) => {
            Ok::<Adt15, Adt142>(Adt15::C3(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt142>(v3)
        }
    }
}

pub fn f819(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2799 = f887(Str::lit("uri"), v1.clone());
    match m2799 {
        Ok(v2) => {
            let m2800 = f888(Str::lit("inputResponses"), v1.clone());
            match m2800 {
                Ok(v3) => {
                    let m2801 = f886(Str::lit("requestState"), v1);
                    match m2801 {
                        Ok(v4) => {
                            Ok::<Adt15, Adt142>(Adt15::C4(Adt23::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt15, Adt142>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt15, Adt142>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt15, Adt142>(v7)
        }
    }
}

pub fn f820(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2802 = f886(Str::lit("cursor"), v1);
    match m2802 {
        Ok(v2) => {
            Ok::<Adt15, Adt142>(Adt15::C5(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt142>(v3)
        }
    }
}

pub fn f821(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2803 = f886(Str::lit("cursor"), v1);
    match m2803 {
        Ok(v2) => {
            Ok::<Adt15, Adt142>(Adt15::C6(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt142>(v3)
        }
    }
}

pub fn f822(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2804 = f887(Str::lit("name"), v1.clone());
    match m2804 {
        Ok(v2) => {
            let m2805 = f889(Str::lit("arguments"), v1.clone());
            match m2805 {
                Ok(v3) => {
                    let m2806 = f888(Str::lit("inputResponses"), v1.clone());
                    match m2806 {
                        Ok(v4) => {
                            let m2807 = f886(Str::lit("requestState"), v1);
                            match m2807 {
                                Ok(v5) => {
                                    Ok::<Adt15, Adt142>(Adt15::C7(Adt24::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt15, Adt142>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt15, Adt142>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt15, Adt142>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt15, Adt142>(v9)
        }
    }
}

pub fn f823(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2808 = f890(Str::lit("ref"), v1.clone());
    match m2808 {
        Ok(v2) => {
            let m2809 = f891(v2);
            match m2809 {
                Ok(v3) => {
                    let m2810 = f890(Str::lit("argument"), v1.clone());
                    match m2810 {
                        Ok(v4) => {
                            let m2811 = f892(v4);
                            match m2811 {
                                Ok(v5) => {
                                    let m2812 = f893(v1);
                                    match m2812 {
                                        Ok(v6) => {
                                            Ok::<Adt15, Adt142>(Adt15::C8(Adt25::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt15, Adt142>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt15, Adt142>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt15, Adt142>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt15, Adt142>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt15, Adt142>(v11)
        }
    }
}

pub fn f824(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2813 = f887(Str::lit("taskId"), v1);
    match m2813 {
        Ok(v2) => {
            Ok::<Adt15, Adt142>(Adt15::C9(Adt30::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt142>(v3)
        }
    }
}

pub fn f825(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2814 = f887(Str::lit("taskId"), v1.clone());
    match m2814 {
        Ok(v2) => {
            let m2815 = f890(Str::lit("inputResponses"), v1);
            match m2815 {
                Ok(v3) => {
                    Ok::<Adt15, Adt142>(Adt15::C10(Adt31::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt15, Adt142>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt15, Adt142>(v5)
        }
    }
}

pub fn f826(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt142> {
    let m2816 = f887(Str::lit("taskId"), v1);
    match m2816 {
        Ok(v2) => {
            Ok::<Adt15, Adt142>(Adt15::C11(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt142>(v3)
        }
    }
}

pub fn f827(v0: Adt112, v1: Adt17, v2: Option<Adt141>) -> Adt153 {
    let m2817 = v2;
    match m2817 {
        None => {
            Adt153::C0(Adt154::C0(match v0.clone() {
                Adt112::C0(h2818, _, _) => {
                    h2818
                }
            }, match v0.clone() {
                Adt112::C0(_, h2819, _) => {
                    h2819
                }
            }, v1, f749(match v0 {
                Adt112::C0(_, _, h2820) => {
                    h2820
                }
            })))
        }
        Some(v3) => {
            Adt153::C1(v3)
        }
    }
}

pub fn f828(v0: Str, v1: List<Str>) -> Option<Adt141> {
    f894(f895(v0.clone(), v1.clone()), v0, v1)
}

pub fn f829(v0: Adt120, v1: Adt155, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt121>> {
    let m2822 = match v0.clone() {
        Adt120::C0(h2821, _, _, _, _) => {
            h2821
        }
    };
    match m2822 {
        None => {
            Ok(Some(Adt121::C0))
        }
        Some(v5) => {
            let a2823 = v5;
            let a2824 = v3;
            if equal(a2823, a2824) {
                let m2826 = match v0.clone() {
                    Adt120::C0(_, h2825, _, _, _) => {
                        h2825
                    }
                };
                match m2826 {
                    None => {
                        Ok(Some(Adt121::C1))
                    }
                    Some(v6) => {
                        let a2827 = v6;
                        let a2828 = v2;
                        if equal(a2827, a2828) {
                            f896(f897(match v0.clone() {
                                Adt120::C0(_, _, h2829, _, _) => {
                                    h2829
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt120::C0(_, _, _, _, h2830) => {
                                    h2830
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt121::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt121::C3))
            }
        }
    }
}

pub fn f830(v0: Adt5) -> Adt162 {
    f898(f234(f832(), v0.clone()), f234(f833(), v0))
}

pub fn f831(v0: Adt5, v1: Adt110, v2: Adt110) -> Result<Adt17, Adt142> {
    let m2831 = v1;
    match m2831 {
        Adt110::C0 => {
            Err::<Adt17, Adt142>(Adt142::C2)
        }
        Adt110::C1(v3) => {
            let m2832 = v2;
            match m2832 {
                Adt110::C0 => {
                    Err::<Adt17, Adt142>(Adt142::C3)
                }
                Adt110::C1(v4) => {
                    f899(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f832() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f833() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f834(v0: u64) -> R<Option<u64>> {
    let a2833 = 64u64;
    let a2834 = v0;
    if if nat_lt(a2833, a2834) {
        let a2835 = v0;
        let a2836 = 91u64;
        nat_lt(a2835, a2836)
    } else {
        false
    } {
        Ok(Some({
            let a2837 = v0;
            let a2838 = 65u64;
            nat_sub(a2837, a2838)
        }))
    } else {
        let a2839 = 96u64;
        let a2840 = v0;
        if if nat_lt(a2839, a2840) {
            let a2841 = v0;
            let a2842 = 123u64;
            nat_lt(a2841, a2842)
        } else {
            false
        } {
            Ok(Some({
                let a2843 = v0;
                let a2844 = 71u64;
                nat_sub(a2843, a2844)
            }))
        } else {
            let a2845 = 47u64;
            let a2846 = v0;
            if if nat_lt(a2845, a2846) {
                let a2847 = v0;
                let a2848 = 58u64;
                nat_lt(a2847, a2848)
            } else {
                false
            } {
                Ok(Some({
                    let a2849 = v0;
                    let a2850 = 4u64;
                    nat_add(a2849, a2850)?
                }))
            } else {
                let a2851 = v0;
                let a2852 = 43u64;
                if nat_eq(a2851, a2852) {
                    Ok(Some(62u64))
                } else {
                    let a2853 = v0;
                    let a2854 = 47u64;
                    if nat_eq(a2853, a2854) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f835(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2855 = v3;
        let a2856 = 61u64;
        if nat_eq(a2855, a2856) {
            let a2857 = f900(v1, 16u64);
            let a2858 = 0u64;
            nat_eq(a2857, a2858)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f901(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f836(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2859 = f900(v2, 4u64);
        let a2860 = 0u64;
        nat_eq(a2859, a2860)
    } else {
        false
    } {
        Ok(Some(f902(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f837(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2863 = {
            let a2861 = v0;
            let a2862 = 4u64;
            nat_mul(a2861, a2862)?
        };
        let a2864 = f903(v1, 16u64);
        nat_add(a2863, a2864)?
    }, List::cons({
        let a2867 = {
            let a2865 = f900(v1, 16u64);
            let a2866 = 16u64;
            nat_mul(a2865, a2866)?
        };
        let a2868 = f903(v2, 4u64);
        nat_add(a2867, a2868)?
    }, List::cons({
        let a2871 = {
            let a2869 = f900(v2, 4u64);
            let a2870 = 64u64;
            nat_mul(a2869, a2870)?
        };
        let a2872 = v3;
        nat_add(a2871, a2872)?
    }, List::<u64>::nil()))))
}

pub fn f838(v0: Adt6) -> bool {
    let m2873 = f133(v0);
    match m2873 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f839(v0: Adt6) -> bool {
    let m2874 = f770(v0);
    match m2874 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f840(v0: Adt6) -> bool {
    let m2875 = f235(v0);
    match m2875 {
        None => {
            false
        }
        Some(v1) => {
            let a2879 = {
                let m2878 = {
                    let a2876 = v1;
                    let a2877 = 0i64;
                    compare(a2876, a2877)
                };
                match m2878 {
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
            bool_not(a2879)
        }
    }
}

pub fn f841(v0: Adt6) -> bool {
    let m2880 = f294(v0);
    match m2880 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f842(v0: Adt6) -> bool {
    let m2881 = f239(v0);
    match m2881 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f843(v0: Adt6) -> bool {
    let m2882 = f294(v0);
    match m2882 {
        None => {
            false
        }
        Some(v1) => {
            f904(v1)
        }
    }
}

pub fn f844(v0: Adt6) -> bool {
    let m2883 = f447(v0);
    match m2883 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f845(v0: Adt6) -> bool {
    let m2884 = f133(v0.clone());
    match m2884 {
        None => {
            let m2885 = f294(v0);
            match m2885 {
                None => {
                    false
                }
                Some(v2) => {
                    f905(v2)
                }
            }
        }
        Some(v1) => {
            f765(v1)
        }
    }
}

pub fn f846(v0: Adt6) -> bool {
    let m2886 = f294(v0);
    match m2886 {
        None => {
            false
        }
        Some(v1) => {
            let m2887 = v1;
            match m2887.uncons() {
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

pub fn f847(v0: Str, p2905: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2904 = p2905;
    {
        let mut m2904 = m2904;
        loop {
            let step = {
                let v1 = m2904;
                let m2888 = v1;
                match m2888.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2892 = v0.clone();
                        let a2893 = {
                            let a2890 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2889, _) = v2.clone();
                                h2889
                            }, List::<Str>::nil()));
                            let a2891 = Str::lit("");
                            join(a2890, a2891)
                        };
                        if equal(a2892, a2893) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2894) = v2;
                                h2894
                            }))
                        } else {
                            let m2896 = f239({
                                let (_, h2895) = v2.clone();
                                h2895
                            });
                            match m2896 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2897 = f234(Str::lit("$anchor"), v4);
                                    match m2897 {
                                        Adt110::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt110::C1(v5) => {
                                            let m2898 = f133(v5);
                                            match m2898 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2901 = v0.clone();
                                                    let a2902 = {
                                                        let a2899 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2900 = Str::lit("");
                                                        join(a2899, a2900)
                                                    };
                                                    if equal(a2901, a2902) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2903) = v2;
                                                            h2903
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
                Ok(next) => { m2904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f848(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2906 = f347(Str::lit("type"), v0);
    match m2906 {
        Adt110::C0 => {
            None::<Adt13>
        }
        Adt110::C1(v1) => {
            let m2907 = f133(v1);
            match m2907 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f906(v2)
                }
            }
        }
    }
}

pub fn f849(v0: List<u64>) -> R<List<u64>> {
    Ok(f401(f907(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f850(v0: List<u64>, p2910: List<List<u64>>) -> bool {
    let m2909 = p2910;
    {
        let mut m2909 = m2909;
        loop {
            let step = {
                let v1 = m2909;
                let m2908 = v1;
                match m2908.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f908(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f851(v0: Adt7, v1: Adt7, v2: Adt160, v3: Adt160) -> R<Adt160> {
    let m2911 = v2;
    match m2911 {
        Adt160::C0 => {
            let m2913 = v3;
            match m2913 {
                Adt160::C0 => {
                    Ok(f909(Adt160::C0, f910(f911(match v0.clone() {
                        Adt7::C0(h2914, _) => {
                            h2914
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2915) => {
                            h2915
                        }
                    }, f911(match v1.clone() {
                        Adt7::C0(h2916, _) => {
                            h2916
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2917) => {
                            h2917
                        }
                    })?))
                }
                Adt160::C1 => {
                    Ok(Adt160::C0)
                }
                Adt160::C2 => {
                    Ok(Adt160::C0)
                }
            }
        }
        Adt160::C1 => {
            let m2912 = v3;
            match m2912 {
                Adt160::C0 => {
                    Ok(Adt160::C2)
                }
                Adt160::C1 => {
                    Ok(Adt160::C1)
                }
                Adt160::C2 => {
                    Ok(Adt160::C0)
                }
            }
        }
        Adt160::C2 => {
            let m2918 = v3;
            match m2918 {
                Adt160::C0 => {
                    Ok(Adt160::C2)
                }
                Adt160::C1 => {
                    Ok(Adt160::C2)
                }
                Adt160::C2 => {
                    f910(match v0.clone() {
                        Adt7::C0(h2919, _) => {
                            h2919
                        }
                    }, match v0 {
                        Adt7::C0(_, h2920) => {
                            h2920
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2921, _) => {
                            h2921
                        }
                    }, match v1 {
                        Adt7::C0(_, h2922) => {
                            h2922
                        }
                    })
                }
            }
        }
    }
}

pub fn f852(v0: i64) -> Adt160 {
    f859(v0, 0i64)
}

pub fn f853(v0: Adt6) -> Str {
    let m2923 = v0;
    match m2923 {
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

pub fn f854(v0: Adt7) -> R<bool> {
    if f348(match v0.clone() {
        Adt7::C0(h2924, _) => {
            h2924
        }
    }) {
        Ok(true)
    } else {
        f912(40u64, match v0.clone() {
            Adt7::C0(h2925, _) => {
                h2925
            }
        }, match v0 {
            Adt7::C0(_, h2926) => {
                h2926
            }
        })
    }
}

pub fn f855(v0: Adt6) -> Str {
    let m2927 = f133(v0);
    match m2927 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f856(p2934: u64, p2935: List<(Adt6, Adt6)>) -> R<Adt161> {
    let m2933 = (p2934, p2935);
    {
        let mut m2933 = m2933;
        loop {
            let step = {
                let (v0, v1) = m2933;
                let m2928 = v0;
                if m2928 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt161>>(Ok(Adt161::C2))
                } else {
                    let v2 = m2928 - 1;
                    let m2929 = v1;
                    match m2929.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt161>>(Ok(Adt161::C0))
                        }
                        Some((v3, v4)) => {
                            let m2932 = f913({
                                let (h2930, _) = v3;
                                h2930
                            }, {
                                let (_, h2931) = v3;
                                h2931
                            }, v4)?;
                            match m2932 {
                                Adt163::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt161>>(Ok(Adt161::C1))
                                }
                                Adt163::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt161>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2933 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f857() -> u64 {
    67108864u64
}

pub fn f858(v0: Adt160) -> bool {
    let m2936 = v0;
    match m2936 {
        Adt160::C0 => {
            true
        }
        Adt160::C1 => {
            false
        }
        Adt160::C2 => {
            false
        }
    }
}

pub fn f859(v0: i64, v1: i64) -> Adt160 {
    let m2939 = {
        let a2937 = v0;
        let a2938 = v1;
        compare(a2937, a2938)
    };
    if match m2939 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Adt160::C0
    } else {
        let m2942 = {
            let a2940 = v1;
            let a2941 = v0;
            compare(a2940, a2941)
        };
        if match m2942 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt160::C2
        } else {
            Adt160::C1
        }
    }
}

pub fn f860(p2951: List<u64>, p2952: u64) -> R<u64> {
    let m2950 = (p2951, p2952);
    {
        let mut m2950 = m2950;
        loop {
            let step = {
                let (v0, v1) = m2950;
                let m2943 = v0;
                match m2943.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2944 = v2;
                        let a2945 = 128u64;
                        if if nat_lt(a2944, a2945) {
                            true
                        } else {
                            let a2946 = 191u64;
                            let a2947 = v2;
                            nat_lt(a2946, a2947)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2948 = v1;
                                let a2949 = 1u64;
                                nat_add(a2948, a2949)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2950 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f861(v0: u64, v1: List<u64>) -> Adt159 {
    let a2953 = v0;
    let a2954 = 32u64;
    if nat_lt(a2953, a2954) {
        if f914(v0) {
            Adt159::C0(0u64, List::cons(v0, v1))
        } else {
            Adt159::C0(0u64, f862(v1))
        }
    } else {
        let a2955 = v0;
        let a2956 = 127u64;
        if nat_eq(a2955, a2956) {
            Adt159::C0(0u64, f862(v1))
        } else {
            let a2957 = v0;
            let a2958 = 194u64;
            if nat_eq(a2957, a2958) {
                Adt159::C0(1u64, v1)
            } else {
                let a2959 = v0;
                let a2960 = 226u64;
                if nat_eq(a2959, a2960) {
                    Adt159::C0(2u64, v1)
                } else {
                    Adt159::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f862(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f863(v0: List<Adt42>) -> List<Adt6> {
    f445(f915(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f864(p2963: List<Adt2>, p2964: List<Adt6>) -> List<Adt6> {
    let m2962 = (p2963, p2964);
    {
        let mut m2962 = m2962;
        loop {
            let step = {
                let (v0, v1) = m2962;
                let m2961 = v0;
                match m2961.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f916(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2962 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f865(v0: i64) -> u64 {
    let m2967 = {
        let a2965 = v0;
        let a2966 = 1i64;
        compare(a2965, a2966)
    };
    if match m2967 {
        Ordering::Less => {
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
        let m2970 = {
            let a2968 = v0;
            let a2969 = 2i64;
            compare(a2968, a2969)
        };
        if match m2970 {
            Ordering::Less => {
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
            let m2973 = {
                let a2971 = v0;
                let a2972 = 3i64;
                compare(a2971, a2972)
            };
            if match m2973 {
                Ordering::Less => {
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
                let m2976 = {
                    let a2974 = v0;
                    let a2975 = 4i64;
                    compare(a2974, a2975)
                };
                if match m2976 {
                    Ordering::Less => {
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
                    let m2979 = {
                        let a2977 = v0;
                        let a2978 = 5i64;
                        compare(a2977, a2978)
                    };
                    if match m2979 {
                        Ordering::Less => {
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
                        let m2982 = {
                            let a2980 = v0;
                            let a2981 = 6i64;
                            compare(a2980, a2981)
                        };
                        if match m2982 {
                            Ordering::Less => {
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
                            let m2985 = {
                                let a2983 = v0;
                                let a2984 = 7i64;
                                compare(a2983, a2984)
                            };
                            if match m2985 {
                                Ordering::Less => {
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
                                let m2988 = {
                                    let a2986 = v0;
                                    let a2987 = 8i64;
                                    compare(a2986, a2987)
                                };
                                if match m2988 {
                                    Ordering::Less => {
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
                                    let m2991 = {
                                        let a2989 = v0;
                                        let a2990 = 9i64;
                                        compare(a2989, a2990)
                                    };
                                    if match m2991 {
                                        Ordering::Less => {
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

pub fn f866(v0: Adt8) -> Adt6 {
    Adt6::C5(f352(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2992) => {
            h2992
        }
    }))
}

pub fn f867(v0: Adt36) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt36::C0(h2993, _, _, _, _, _, _, _) => {
            h2993
        }
    })), {
        let a3011 = f698(Str::lit("title"), match v0.clone() {
            Adt36::C0(_, h2994, _, _, _, _, _, _) => {
                h2994
            }
        });
        let a3012 = {
            let a3009 = f699(match v0.clone() {
                Adt36::C0(_, _, h2995, _, _, _, _, _) => {
                    h2995
                }
            });
            let a3010 = {
                let a3007 = f698(Str::lit("description"), match v0.clone() {
                    Adt36::C0(_, _, _, h2996, _, _, _, _) => {
                        h2996
                    }
                });
                let a3008 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2999 = match v0.clone() {
                        Adt36::C0(_, _, _, _, h2998, _, _, _) => {
                            h2998
                        }
                    };
                    match m2999 {
                        Adt5::C0(h2997) => {
                            h2997
                        }
                    }
                })), {
                    let a3005 = f740(Str::lit("outputSchema"), match v0.clone() {
                        Adt36::C0(_, _, _, _, _, h3000, _, _) => {
                            h3000
                        }
                    });
                    let a3006 = {
                        let a3003 = f917(match v0.clone() {
                            Adt36::C0(_, _, _, _, _, _, h3001, _) => {
                                h3001
                            }
                        });
                        let a3004 = f697(match v0 {
                            Adt36::C0(_, _, _, _, _, _, _, h3002) => {
                                h3002
                            }
                        });
                        append_list(a3003, a3004)
                    };
                    append_list(a3005, a3006)
                });
                append_list(a3007, a3008)
            };
            append_list(a3009, a3010)
        };
        append_list(a3011, a3012)
    }))
}

pub fn f868(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a3036 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h3013, _, _, _, _, _, _, _, _) => {
                h3013
            }
        })), List::<(Str, Adt6)>::nil());
        let a3037 = {
            let a3034 = f698(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h3014, _, _, _, _, _, _, _) => {
                    h3014
                }
            });
            let a3035 = {
                let a3032 = f699(match v0.clone() {
                    Adt45::C0(_, _, h3015, _, _, _, _, _, _) => {
                        h3015
                    }
                });
                let a3033 = {
                    let a3030 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h3016, _, _, _, _, _) => {
                            h3016
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a3031 = {
                        let a3028 = f698(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h3017, _, _, _, _) => {
                                h3017
                            }
                        });
                        let a3029 = {
                            let a3026 = f698(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h3018, _, _, _) => {
                                    h3018
                                }
                            });
                            let a3027 = {
                                let a3024 = f696(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h3019, _, _) => {
                                        h3019
                                    }
                                });
                                let a3025 = {
                                    let a3022 = f700(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h3020, _) => {
                                            h3020
                                        }
                                    })?;
                                    let a3023 = f697(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h3021) => {
                                            h3021
                                        }
                                    });
                                    append_list(a3022, a3023)
                                };
                                append_list(a3024, a3025)
                            };
                            append_list(a3026, a3027)
                        };
                        append_list(a3028, a3029)
                    };
                    append_list(a3030, a3031)
                };
                append_list(a3032, a3033)
            };
            append_list(a3034, a3035)
        };
        append_list(a3036, a3037)
    }))
}

pub fn f869(v0: Adt55) -> Adt6 {
    Adt6::C5({
        let a3058 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt55::C0(h3038, _, _, _, _, _, _, _) => {
                h3038
            }
        })), List::<(Str, Adt6)>::nil());
        let a3059 = {
            let a3056 = f698(Str::lit("title"), match v0.clone() {
                Adt55::C0(_, h3039, _, _, _, _, _, _) => {
                    h3039
                }
            });
            let a3057 = {
                let a3054 = f699(match v0.clone() {
                    Adt55::C0(_, _, h3040, _, _, _, _, _) => {
                        h3040
                    }
                });
                let a3055 = {
                    let a3052 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt55::C0(_, _, _, h3041, _, _, _, _) => {
                            h3041
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a3053 = {
                        let a3050 = f698(Str::lit("description"), match v0.clone() {
                            Adt55::C0(_, _, _, _, h3042, _, _, _) => {
                                h3042
                            }
                        });
                        let a3051 = {
                            let a3048 = f698(Str::lit("mimeType"), match v0.clone() {
                                Adt55::C0(_, _, _, _, _, h3043, _, _) => {
                                    h3043
                                }
                            });
                            let a3049 = {
                                let a3046 = f696(match v0.clone() {
                                    Adt55::C0(_, _, _, _, _, _, h3044, _) => {
                                        h3044
                                    }
                                });
                                let a3047 = f697(match v0 {
                                    Adt55::C0(_, _, _, _, _, _, _, h3045) => {
                                        h3045
                                    }
                                });
                                append_list(a3046, a3047)
                            };
                            append_list(a3048, a3049)
                        };
                        append_list(a3050, a3051)
                    };
                    append_list(a3052, a3053)
                };
                append_list(a3054, a3055)
            };
            append_list(a3056, a3057)
        };
        append_list(a3058, a3059)
    })
}

pub fn f870(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a3075 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h3060, _, _, _, _, _) => {
                h3060
            }
        })), List::<(Str, Adt6)>::nil());
        let a3076 = {
            let a3073 = f698(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h3061, _, _, _, _) => {
                    h3061
                }
            });
            let a3074 = {
                let a3071 = f699(match v0.clone() {
                    Adt57::C0(_, _, h3062, _, _, _) => {
                        h3062
                    }
                });
                let a3072 = {
                    let a3069 = f698(Str::lit("description"), match v0.clone() {
                        Adt57::C0(_, _, _, h3063, _, _) => {
                            h3063
                        }
                    });
                    let a3070 = {
                        let a3067 = {
                            let m3065 = match v0.clone() {
                                Adt57::C0(_, _, _, _, h3064, _) => {
                                    h3064
                                }
                            };
                            match m3065 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f918(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a3068 = f697(match v0 {
                            Adt57::C0(_, _, _, _, _, h3066) => {
                                h3066
                            }
                        });
                        append_list(a3067, a3068)
                    };
                    append_list(a3069, a3070)
                };
                append_list(a3071, a3072)
            };
            append_list(a3073, a3074)
        };
        append_list(a3075, a3076)
    })
}

pub fn f871(v0: Adt60) -> R<Option<Adt6>> {
    let m3078 = f536(match v0.clone() {
        Adt60::C0(_, h3077) => {
            h3077
        }
    })?;
    match m3078 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f919(match v0 {
                Adt60::C0(h3079, _) => {
                    h3079
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f872(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f351(Str::lit("params"), v1)))
}

pub fn f873(v0: Adt66) -> Adt6 {
    let m3080 = v0;
    match m3080 {
        Adt66::C0(v1) => {
            f920(v1)
        }
        Adt66::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt69::C0(h3081, _) => {
                    h3081
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt69::C0(_, h3082) => {
                    h3082
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f874(v0: Adt70) -> R<Option<Adt6>> {
    let m3084 = f921(match v0.clone() {
        Adt70::C0(h3083, _, _, _, _, _, _, _, _, _) => {
            h3083
        }
    })?;
    match m3084 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a3086 = f922(v0.clone());
                let a3087 = List::cons((Str::lit("maxTokens"), f126(match v0.clone() {
                    Adt70::C0(_, _, _, _, _, h3085, _, _, _, _) => {
                        h3085
                    }
                })?), f923(v0));
                append_list(a3086, a3087)
            }))))
        }
    }
}

pub fn f875(p3093: List<(Str, Adt65)>, p3094: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m3092 = (p3093, p3094);
    {
        let mut m3092 = m3092;
        loop {
            let step = {
                let (v0, v1) = m3092;
                let m3088 = v0;
                match m3088.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m3090 = f924({
                            let (_, h3089) = v2;
                            h3089
                        });
                        match m3090 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h3091, _) = v2;
                                    h3091
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3092 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f876(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f877(v0: Adt85) -> Option<Adt6> {
    let m3096 = f631(match v0.clone() {
        Adt85::C0(h3095, _, _) => {
            h3095
        }
    });
    match m3096 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m3098 = f925(match v0.clone() {
                Adt85::C0(_, _, h3097) => {
                    h3097
                }
            });
            match m3098 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f876(match v0 {
                        Adt85::C0(_, h3099, _) => {
                            h3099
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f878(v0: List<u64>) -> List<u64> {
    let a3102 = f926();
    let a3103 = {
        let a3100 = v0;
        let a3101 = f927();
        append_list(a3100, a3101)
    };
    append_list(a3102, a3103)
}

pub fn f879(p3110: List<u64>) -> bool {
    let m3109 = p3110;
    {
        let mut m3109 = m3109;
        loop {
            let step = {
                let v0 = m3109;
                let m3104 = v0;
                match m3104.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a3105 = v1;
                        let a3106 = 48u64;
                        if if nat_eq(a3105, a3106) {
                            true
                        } else {
                            let a3107 = v1;
                            let a3108 = 46u64;
                            nat_eq(a3107, a3108)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f880(v0: Adt150) -> bool {
    let m3111 = v0;
    match m3111 {
        Adt150::C0 => {
            true
        }
        Adt150::C1 => {
            false
        }
        Adt150::C2 => {
            false
        }
        Adt150::C3 => {
            false
        }
        Adt150::C4 => {
            false
        }
        Adt150::C5 => {
            false
        }
        Adt150::C6 => {
            false
        }
        Adt150::C7 => {
            false
        }
        Adt150::C8 => {
            false
        }
        Adt150::C9 => {
            false
        }
        Adt150::C10 => {
            false
        }
        Adt150::C11 => {
            false
        }
    }
}

pub fn f881(v0: Adt15) -> Adt150 {
    let m3112 = v0;
    match m3112 {
        Adt15::C0(_) => {
            Adt150::C0
        }
        Adt15::C1(_) => {
            Adt150::C1
        }
        Adt15::C2(_) => {
            Adt150::C2
        }
        Adt15::C3(_) => {
            Adt150::C3
        }
        Adt15::C4(_) => {
            Adt150::C4
        }
        Adt15::C5(_) => {
            Adt150::C5
        }
        Adt15::C6(_) => {
            Adt150::C6
        }
        Adt15::C7(_) => {
            Adt150::C7
        }
        Adt15::C8(_) => {
            Adt150::C8
        }
        Adt15::C9(_) => {
            Adt150::C9
        }
        Adt15::C10(_) => {
            Adt150::C10
        }
        Adt15::C11(_) => {
            Adt150::C11
        }
    }
}

pub fn f882(v0: Adt0, v1: Adt33) -> Adt138 {
    Adt138::C0(v1, Adt139::C0(Some(match v0 {
        Adt0::C0(h3113, _, _, _, _, _, _, _) => {
            h3113
        }
    }), None::<Adt5>), List::<Adt140>::nil())
}

pub fn f883(v0: Adt0) -> Adt34 {
    Adt34::C0(match v0.clone() {
        Adt0::C0(_, _, _, h3114, _, _, _, _) => {
            h3114
        }
    }, f412(), match v0.clone() {
        Adt0::C0(_, _, h3115, _, _, _, _, _) => {
            h3115
        }
    }, match v0 {
        Adt0::C0(_, h3116, _, _, _, _, _, _) => {
            h3116
        }
    })
}

pub fn f884(v0: Adt0, v1: Adt15, v2: Result<Adt33, Adt90>) -> Result<Adt138, Adt141> {
    let m3117 = v2;
    match m3117 {
        Ok(v4) => {
            if f928(f881(v1.clone()), v4.clone()) {
                f929(v0, v1, v4)
            } else {
                Err::<Adt138, Adt141>(Adt141::C4(f930()))
            }
        }
        Err(v3) => {
            Err::<Adt138, Adt141>(Adt141::C4(v3))
        }
    }
}

pub fn f885(v0: Bytes, v1: u64, v2: u64, v3: Fn4, v4: Adt15) -> R<Result<Adt33, Adt90>> {
    f931(v0.clone(), v1, v2, v3, v4.clone(), f932(v0, v4)?)
}

pub fn f886(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt142> {
    let m3118 = f234(v0.clone(), v1);
    match m3118 {
        Adt110::C0 => {
            Ok::<Option<Str>, Adt142>(None::<Str>)
        }
        Adt110::C1(v2) => {
            let m3119 = f133(v2);
            match m3119 {
                None => {
                    Err::<Option<Str>, Adt142>(Adt142::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt142>(Some(v3))
                }
            }
        }
    }
}

pub fn f887(v0: Str, v1: Adt5) -> Result<Str, Adt142> {
    let m3120 = f234(v0.clone(), v1);
    match m3120 {
        Adt110::C0 => {
            Err::<Str, Adt142>(Adt142::C7(v0))
        }
        Adt110::C1(v2) => {
            let m3121 = f133(v2);
            match m3121 {
                None => {
                    Err::<Str, Adt142>(Adt142::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt142>(v3)
                }
            }
        }
    }
}

pub fn f888(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt142> {
    let m3122 = f234(v0.clone(), v1);
    match m3122 {
        Adt110::C0 => {
            Ok::<Option<Adt5>, Adt142>(None::<Adt5>)
        }
        Adt110::C1(v2) => {
            let m3123 = f239(v2);
            match m3123 {
                None => {
                    Err::<Option<Adt5>, Adt142>(Adt142::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt142>(Some(v3))
                }
            }
        }
    }
}

pub fn f889(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt142> {
    let m3124 = f234(v0.clone(), v1);
    match m3124 {
        Adt110::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt142>(None::<List<(Str, Str)>>)
        }
        Adt110::C1(v2) => {
            let m3125 = f239(v2);
            match m3125 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt142>(Adt142::C8(v0))
                }
                Some(v3) => {
                    let m3127 = f933(v0, match v3 {
                        Adt5::C0(h3126) => {
                            h3126
                        }
                    });
                    match m3127 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt142>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt142>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f890(v0: Str, v1: Adt5) -> Result<Adt5, Adt142> {
    let m3128 = f234(v0.clone(), v1);
    match m3128 {
        Adt110::C0 => {
            Err::<Adt5, Adt142>(Adt142::C7(v0))
        }
        Adt110::C1(v2) => {
            let m3129 = f239(v2);
            match m3129 {
                None => {
                    Err::<Adt5, Adt142>(Adt142::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt142>(v3)
                }
            }
        }
    }
}

pub fn f891(v0: Adt5) -> Result<Adt26, Adt142> {
    let m3130 = f887(Str::lit("type"), v0.clone());
    match m3130 {
        Ok(v1) => {
            let a3131 = v1.clone();
            let a3132 = Str::lit("ref/prompt");
            if equal(a3131, a3132) {
                let m3133 = f887(Str::lit("name"), v0.clone());
                match m3133 {
                    Ok(v2) => {
                        let m3134 = f886(Str::lit("title"), v0);
                        match m3134 {
                            Ok(v3) => {
                                Ok::<Adt26, Adt142>(Adt26::C0(Adt27::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt26, Adt142>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt26, Adt142>(v5)
                    }
                }
            } else {
                let a3135 = v1;
                let a3136 = Str::lit("ref/resource");
                if equal(a3135, a3136) {
                    let m3137 = f887(Str::lit("uri"), v0);
                    match m3137 {
                        Ok(v6) => {
                            Ok::<Adt26, Adt142>(Adt26::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt26, Adt142>(v7)
                        }
                    }
                } else {
                    Err::<Adt26, Adt142>(Adt142::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt26, Adt142>(v8)
        }
    }
}

pub fn f892(v0: Adt5) -> Result<Adt28, Adt142> {
    let m3138 = f887(Str::lit("name"), v0.clone());
    match m3138 {
        Ok(v1) => {
            let m3139 = f887(Str::lit("value"), v0);
            match m3139 {
                Ok(v2) => {
                    Ok::<Adt28, Adt142>(Adt28::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt28, Adt142>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt28, Adt142>(v4)
        }
    }
}

pub fn f893(v0: Adt5) -> Result<Option<Adt29>, Adt142> {
    let m3140 = f234(Str::lit("context"), v0);
    match m3140 {
        Adt110::C0 => {
            Ok::<Option<Adt29>, Adt142>(None::<Adt29>)
        }
        Adt110::C1(v1) => {
            let m3141 = f239(v1);
            match m3141 {
                None => {
                    Err::<Option<Adt29>, Adt142>(Adt142::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m3142 = f889(Str::lit("arguments"), v2);
                    match m3142 {
                        Ok(v3) => {
                            Ok::<Option<Adt29>, Adt142>(Some(Adt29::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt29>, Adt142>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f894(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt141> {
    if v0 {
        None::<Adt141>
    } else {
        Some(Adt141::C1(v1, v2))
    }
}

pub fn f895(v0: Str, p3147: List<Str>) -> bool {
    let m3146 = p3147;
    {
        let mut m3146 = m3146;
        loop {
            let step = {
                let v1 = m3146;
                let m3143 = v1;
                match m3143.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3144 = v2;
                        let a3145 = v0.clone();
                        if equal(a3144, a3145) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f896(v0: Option<Adt121>, v1: List<Adt123>, v2: Adt5) -> R<Option<Adt121>> {
    let m3148 = v0;
    match m3148 {
        None => {
            f934(v1, f935(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f897(v0: Option<Str>, v1: Adt155, v2: Adt5) -> Option<Adt121> {
    let m3149 = v1.clone();
    match m3149 {
        Adt155::C0 => {
            None::<Adt121>
        }
        Adt155::C1 => {
            f936(v0, f937(v1, v2))
        }
        Adt155::C2 => {
            f936(v0, f937(v1, v2))
        }
        Adt155::C3 => {
            f936(v0, f937(v1, v2))
        }
    }
}

pub fn f898(v0: Adt110, v1: Adt110) -> Adt162 {
    let m3150 = v0;
    match m3150 {
        Adt110::C0 => {
            Adt162::C1
        }
        Adt110::C1(_) => {
            let m3151 = v1;
            match m3151 {
                Adt110::C0 => {
                    Adt162::C2
                }
                Adt110::C1(_) => {
                    Adt162::C0
                }
            }
        }
    }
}

pub fn f899(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt17, Adt142> {
    let m3152 = f133(v1);
    match m3152 {
        None => {
            Err::<Adt17, Adt142>(Adt142::C5)
        }
        Some(v3) => {
            let m3153 = f938(v2);
            match m3153 {
                None => {
                    Err::<Adt17, Adt142>(Adt142::C6)
                }
                Some(v4) => {
                    let m3154 = f888(f939(), v0.clone());
                    match m3154 {
                        Ok(v6) => {
                            let m3155 = f940(v0.clone());
                            match m3155 {
                                Ok(v8) => {
                                    let m3156 = f941(v0.clone());
                                    match m3156 {
                                        Ok(v10) => {
                                            Ok::<Adt17, Adt142>(Adt17::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt17, Adt142>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt17, Adt142>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt17, Adt142>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f900(v0: u64, v1: u64) -> u64 {
    let a3157 = v0;
    let a3158 = v1;
    let a3159 = 0u64;
    nat_rem(a3157, a3158, a3159)
}

pub fn f901(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a3162 = {
            let a3160 = v0;
            let a3161 = 4u64;
            nat_mul(a3160, a3161)?
        };
        let a3163 = f903(v1, 16u64);
        nat_add(a3162, a3163)?
    }, List::<u64>::nil()))
}

pub fn f902(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a3166 = {
            let a3164 = v0;
            let a3165 = 4u64;
            nat_mul(a3164, a3165)?
        };
        let a3167 = f903(v1, 16u64);
        nat_add(a3166, a3167)?
    }, List::cons({
        let a3170 = {
            let a3168 = f900(v1, 16u64);
            let a3169 = 16u64;
            nat_mul(a3168, a3169)?
        };
        let a3171 = f903(v2, 4u64);
        nat_add(a3170, a3171)?
    }, List::<u64>::nil())))
}

pub fn f903(v0: u64, v1: u64) -> u64 {
    let a3172 = v0;
    let a3173 = v1;
    let a3174 = 0u64;
    nat_quot(a3172, a3173, a3174)
}

pub fn f904(p3178: List<Adt6>) -> bool {
    let m3177 = p3178;
    {
        let mut m3177 = m3177;
        loop {
            let step = {
                let v0 = m3177;
                let m3175 = v0;
                match m3175.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3176 = f133(v1);
                        match m3176 {
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
                Ok(next) => { m3177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f905(p3182: List<Adt6>) -> bool {
    let m3181 = p3182;
    {
        let mut m3181 = m3181;
        loop {
            let step = {
                let v0 = m3181;
                let m3179 = v0;
                match m3179.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3180 = f133(v1);
                        match m3180 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f765(v3) {
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
                Ok(next) => { m3181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f906(v0: Str) -> Option<Adt13> {
    let a3183 = v0.clone();
    let a3184 = Str::lit("string");
    if equal(a3183, a3184) {
        Some(Adt13::C0)
    } else {
        let a3185 = v0.clone();
        let a3186 = Str::lit("integer");
        if equal(a3185, a3186) {
            Some(Adt13::C1)
        } else {
            let a3187 = v0;
            let a3188 = Str::lit("boolean");
            if equal(a3187, a3188) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f907(p3197: List<u64>, p3198: List<u64>) -> R<List<u64>> {
    let m3196 = (p3197, p3198);
    {
        let mut m3196 = m3196;
        loop {
            let step = {
                let (v0, v1) = m3196;
                let m3189 = v0;
                match m3189.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a3190 = 65u64;
                        let a3191 = v2;
                        if if nat_le(a3190, a3191) {
                            let a3192 = v2;
                            let a3193 = 90u64;
                            nat_le(a3192, a3193)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a3194 = v2;
                                let a3195 = 32u64;
                                nat_add(a3194, a3195)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f908(p3205: List<u64>, p3206: List<u64>) -> bool {
    let m3204 = (p3205, p3206);
    {
        let mut m3204 = m3204;
        loop {
            let step = {
                let (v0, v1) = m3204;
                let m3199 = v0;
                match m3199.uncons() {
                    None => {
                        let m3200 = v1;
                        match m3200.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m3201 = v1;
                        match m3201.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a3202 = v4;
                                let a3203 = v6;
                                if nat_eq(a3202, a3203) {
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
                Ok(next) => { m3204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f909(v0: Adt160, v1: Adt160) -> Adt160 {
    let m3207 = v0;
    match m3207 {
        Adt160::C0 => {
            f942(v1)
        }
        Adt160::C1 => {
            Adt160::C1
        }
        Adt160::C2 => {
            v1
        }
    }
}

pub fn f910(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt160> {
    let m3208 = f859(f943(v0, v1)?, f943(v2, v3)?);
    match m3208 {
        Adt160::C0 => {
            Ok(Adt160::C0)
        }
        Adt160::C1 => {
            f944(v0, f945(v0)?, v2, f945(v2)?)
        }
        Adt160::C2 => {
            Ok(Adt160::C2)
        }
    }
}

pub fn f911(v0: i64) -> R<i64> {
    let m3211 = {
        let a3209 = v0;
        let a3210 = 0i64;
        compare(a3209, a3210)
    };
    if match m3211 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a3212 = v0;
        int_neg(a3212)
    } else {
        Ok(v0)
    }
}

pub fn f912(p3224: u64, p3225: i64, p3226: i64) -> R<bool> {
    let m3223 = (p3224, (p3225, p3226));
    {
        let mut m3223 = m3223;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3223;
                let m3213 = v0;
                if m3213 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m3213 - 1;
                    let m3216 = {
                        let a3214 = v2;
                        let a3215 = 0i64;
                        compare(a3214, a3215)
                    };
                    if match m3216 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f348({
                            let a3217 = v1;
                            let a3218 = 10i64;
                            let a3219 = 0i64;
                            int_rem(a3217, a3218, a3219)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a3220 = v1;
                                let a3221 = 10i64;
                                let a3222 = 0i64;
                                int_quot(a3220, a3221, a3222)?
                            }, f446(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m3223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f913(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt163> {
    let a3227 = f946(v0.clone());
    let a3228 = f946(v1.clone());
    if nat_eq(a3227, a3228) {
        let m3229 = v0;
        match m3229 {
            Adt6::C0 => {
                Ok(Adt163::C1(v2))
            }
            Adt6::C1(v3) => {
                let m3230 = f447(v1);
                match m3230 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v4) => {
                        let a3231 = v3;
                        let a3232 = v4;
                        if equal(a3231, a3232) {
                            Ok(Adt163::C1(v2))
                        } else {
                            Ok(Adt163::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m3233 = f770(v1);
                match m3233 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v6) => {
                        if f947(f764(v5, v6)?) {
                            Ok(Adt163::C1(v2))
                        } else {
                            Ok(Adt163::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m3234 = f133(v1);
                match m3234 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v8) => {
                        let a3235 = v7;
                        let a3236 = v8;
                        if equal(a3235, a3236) {
                            Ok(Adt163::C1(v2))
                        } else {
                            Ok(Adt163::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m3237 = f294(v1);
                match m3237 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v10) => {
                        Ok(f948(f949(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m3238 = f239(v1);
                match m3238 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v12) => {
                        let a3240 = f775(v11.clone(), 0u64)?;
                        let a3241 = f775(match v12.clone() {
                            Adt5::C0(h3239) => {
                                h3239
                            }
                        }, 0u64)?;
                        if nat_eq(a3240, a3241) {
                            Ok(f948(f950(v11, match v12 {
                                Adt5::C0(h3242) => {
                                    h3242
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt163::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt163::C0)
    }
}

pub fn f914(v0: u64) -> bool {
    let a3243 = v0;
    let a3244 = 9u64;
    if nat_eq(a3243, a3244) {
        true
    } else {
        let a3245 = v0;
        let a3246 = 10u64;
        if nat_eq(a3245, a3246) {
            true
        } else {
            let a3247 = v0;
            let a3248 = 13u64;
            nat_eq(a3247, a3248)
        }
    }
}

pub fn f915(p3251: List<Adt42>, p3252: List<Adt6>) -> List<Adt6> {
    let m3250 = (p3251, p3252);
    {
        let mut m3250 = m3250;
        loop {
            let step = {
                let (v0, v1) = m3250;
                let m3249 = v0;
                match m3249.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt42>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f919(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f916(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h3253, _, _, _) => {
            h3253
        }
    })), {
        let a3259 = f698(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h3254, _, _) => {
                h3254
            }
        });
        let a3260 = {
            let a3257 = f951(match v0.clone() {
                Adt2::C0(_, _, h3255, _) => {
                    h3255
                }
            });
            let a3258 = f952(match v0 {
                Adt2::C0(_, _, _, h3256) => {
                    h3256
                }
            });
            append_list(a3257, a3258)
        };
        append_list(a3259, a3260)
    }))
}

pub fn f917(v0: Option<Adt37>) -> List<(Str, Adt6)> {
    let m3261 = v0;
    match m3261 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a3273 = f698(Str::lit("title"), match v1.clone() {
                    Adt37::C0(h3262, _, _, _, _) => {
                        h3262
                    }
                });
                let a3274 = {
                    let a3271 = f352(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt37::C0(_, h3263, _, _, _) => {
                            h3263
                        }
                    });
                    let a3272 = {
                        let a3269 = f352(Str::lit("destructiveHint"), match v1.clone() {
                            Adt37::C0(_, _, h3264, _, _) => {
                                h3264
                            }
                        });
                        let a3270 = {
                            let a3267 = f352(Str::lit("idempotentHint"), match v1.clone() {
                                Adt37::C0(_, _, _, h3265, _) => {
                                    h3265
                                }
                            });
                            let a3268 = f352(Str::lit("openWorldHint"), match v1 {
                                Adt37::C0(_, _, _, _, h3266) => {
                                    h3266
                                }
                            });
                            append_list(a3267, a3268)
                        };
                        append_list(a3269, a3270)
                    };
                    append_list(a3271, a3272)
                };
                append_list(a3273, a3274)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f918(v0: List<Adt58>) -> List<Adt6> {
    f445(f953(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f919(v0: Adt42) -> Str {
    let m3275 = v0;
    match m3275 {
        Adt42::C0 => {
            Str::lit("user")
        }
        Adt42::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f920(v0: Adt67) -> Adt6 {
    Adt6::C5({
        let a3282 = {
            let m3277 = match v0.clone() {
                Adt67::C0(h3276, _, _) => {
                    h3276
                }
            };
            match m3277 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a3283 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt67::C0(_, h3278, _) => {
                h3278
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m3281 = match v0 {
                Adt67::C0(_, _, h3280) => {
                    h3280
                }
            };
            match m3281 {
                Adt5::C0(h3279) => {
                    h3279
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a3282, a3283)
    })
}

pub fn f921(v0: List<Adt71>) -> R<Option<List<Adt6>>> {
    let m3284 = f954(v0, List::<Adt6>::nil())?;
    match m3284 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f445(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f922(v0: Adt70) -> List<(Str, Adt6)> {
    let a3295 = {
        let m3286 = match v0.clone() {
            Adt70::C0(_, h3285, _, _, _, _, _, _, _, _) => {
                h3285
            }
        };
        match m3286 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f955(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a3296 = {
        let a3293 = f698(Str::lit("systemPrompt"), match v0.clone() {
            Adt70::C0(_, _, h3287, _, _, _, _, _, _, _) => {
                h3287
            }
        });
        let a3294 = {
            let a3291 = {
                let m3289 = match v0.clone() {
                    Adt70::C0(_, _, _, h3288, _, _, _, _, _, _) => {
                        h3288
                    }
                };
                match m3289 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f956(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3292 = f957(Str::lit("temperature"), match v0 {
                Adt70::C0(_, _, _, _, h3290, _, _, _, _, _) => {
                    h3290
                }
            });
            append_list(a3291, a3292)
        };
        append_list(a3293, a3294)
    };
    append_list(a3295, a3296)
}

pub fn f923(v0: Adt70) -> List<(Str, Adt6)> {
    let a3308 = {
        let m3298 = match v0.clone() {
            Adt70::C0(_, _, _, _, _, _, h3297, _, _, _) => {
                h3297
            }
        };
        match m3298 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f707(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a3309 = {
        let a3306 = f740(Str::lit("metadata"), match v0.clone() {
            Adt70::C0(_, _, _, _, _, _, _, h3299, _, _) => {
                h3299
            }
        });
        let a3307 = {
            let a3304 = {
                let m3301 = match v0.clone() {
                    Adt70::C0(_, _, _, _, _, _, _, _, h3300, _) => {
                        h3300
                    }
                };
                match m3301 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f710(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3305 = {
                let m3303 = match v0 {
                    Adt70::C0(_, _, _, _, _, _, _, _, _, h3302) => {
                        h3302
                    }
                };
                match m3303 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f958(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a3304, a3305)
        };
        append_list(a3306, a3307)
    };
    append_list(a3308, a3309)
}

pub fn f924(v0: Adt65) -> Option<Adt6> {
    let m3310 = v0;
    match m3310 {
        Adt65::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f959(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt65::C1(_) => {
            None::<Adt6>
        }
        Adt65::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m3311 = v3;
                match m3311 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h3312) => {
                                h3312
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f925(v0: Adt86) -> Option<Adt6> {
    let m3314 = f802(match v0.clone() {
        Adt86::C0(_, h3313, _, _) => {
            h3313
        }
    });
    match m3314 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f876(match v0.clone() {
                Adt86::C0(h3315, _, _, _) => {
                    h3315
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f876(match v0.clone() {
                Adt86::C0(_, _, h3316, _) => {
                    h3316
                }
            })), List::cons((Str::lit("outcome"), f960(match v0 {
                Adt86::C0(_, _, _, h3317) => {
                    h3317
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f926() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f927() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f928(v0: Adt150, v1: Adt33) -> bool {
    let m3318 = v1;
    match m3318 {
        Adt33::C0(_) => {
            f961(v0, Adt150::C0)
        }
        Adt33::C1(_) => {
            f961(v0, Adt150::C1)
        }
        Adt33::C2(_) => {
            f961(v0, Adt150::C2)
        }
        Adt33::C3(_) => {
            f961(v0, Adt150::C2)
        }
        Adt33::C4(_) => {
            f961(v0, Adt150::C3)
        }
        Adt33::C5(_) => {
            f961(v0, Adt150::C4)
        }
        Adt33::C6(_) => {
            f961(v0, Adt150::C5)
        }
        Adt33::C7(_) => {
            f961(v0, Adt150::C6)
        }
        Adt33::C8(_) => {
            f961(v0, Adt150::C7)
        }
        Adt33::C9(_) => {
            f961(v0, Adt150::C8)
        }
        Adt33::C10(_) => {
            f961(v0, Adt150::C9)
        }
        Adt33::C11 => {
            if f961(v0.clone(), Adt150::C10) {
                true
            } else {
                f961(v0, Adt150::C11)
            }
        }
        Adt33::C12(_) => {
            f962(v0)
        }
        Adt33::C13(_) => {
            f963(v0)
        }
        Adt33::C14(v15) => {
            let m3320 = match v15 {
                Adt88::C0(_, h3319, _) => {
                    h3319
                }
            };
            match m3320 {
                Adt89::C0 => {
                    f961(v0, Adt150::C9)
                }
                Adt89::C1 => {
                    if f961(v0.clone(), Adt150::C10) {
                        true
                    } else {
                        f961(v0, Adt150::C11)
                    }
                }
            }
        }
    }
}

pub fn f929(v0: Adt0, v1: Adt15, v2: Adt33) -> Result<Adt138, Adt141> {
    let m3321 = v2.clone();
    match m3321 {
        Adt33::C0(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C1(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C2(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C3(v9) => {
            Ok::<Adt138, Adt141>(f968(v0, f965(v1), v9))
        }
        Adt33::C4(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C5(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C6(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C7(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C8(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C9(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C10(_) => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C11 => {
            Ok::<Adt138, Adt141>(f882(v0, v2))
        }
        Adt33::C12(v3) => {
            f964(v0, f965(v1), v3)
        }
        Adt33::C13(v4) => {
            f966(v0, f965(v1), v4)
        }
        Adt33::C14(v5) => {
            f967(v0, f965(v1), v5)
        }
    }
}

pub fn f930() -> Adt90 {
    Adt90::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f931(v0: Bytes, v1: u64, v2: u64, v3: Fn4, v4: Adt15, v5: Result<Adt15, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m3322 = v5;
    match m3322 {
        Ok(v7) => {
            f969(v0, v4, f970(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt33, Adt90>(v6))
        }
    }
}

pub fn f932(v0: Bytes, v1: Adt15) -> R<Result<Adt15, Adt90>> {
    let m3323 = v1.clone();
    match m3323 {
        Adt15::C0(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C1(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C2(v2) => {
            f971(v0, v2)
        }
        Adt15::C3(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C4(v3) => {
            f972(v0, v3)
        }
        Adt15::C5(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C6(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C7(v4) => {
            f973(v0, v4)
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

pub fn f933(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt142> {
    let m3324 = f974(v0, v1, List::<(Str, Str)>::nil());
    match m3324 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt142>(f975(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt142>(v3)
        }
    }
}

pub fn f934(p3328: List<Adt123>, v1: Adt5) -> R<Option<Adt121>> {
    let m3327 = p3328;
    {
        let mut m3327 = m3327;
        loop {
            let step = {
                let v0 = m3327;
                let m3325 = v0;
                match m3325.uncons() {
                    None => {
                        Err::<List<Adt123>, R<Option<Adt121>>>(Ok(None::<Adt121>))
                    }
                    Some((v2, v3)) => {
                        let m3326 = f976(v2, v1.clone())?;
                        match m3326 {
                            None => {
                                Ok::<List<Adt123>, R<Option<Adt121>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt123>, R<Option<Adt121>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f935(v0: Adt5) -> Adt5 {
    let m3329 = f234(Str::lit("arguments"), v0);
    match m3329 {
        Adt110::C0 => {
            f27()
        }
        Adt110::C1(v1) => {
            let m3330 = f239(v1);
            match m3330 {
                None => {
                    f27()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f936(v0: Option<Str>, v1: Option<Str>) -> Option<Adt121> {
    let m3331 = v0;
    match m3331 {
        None => {
            Some(Adt121::C2)
        }
        Some(v2) => {
            let m3332 = v1;
            match m3332 {
                None => {
                    None::<Adt121>
                }
                Some(v3) => {
                    let a3333 = v2;
                    let a3334 = v3;
                    if equal(a3333, a3334) {
                        None::<Adt121>
                    } else {
                        Some(Adt121::C5)
                    }
                }
            }
        }
    }
}

pub fn f937(v0: Adt155, v1: Adt5) -> Option<Str> {
    let m3335 = v0;
    match m3335 {
        Adt155::C0 => {
            None::<Str>
        }
        Adt155::C1 => {
            let m3336 = f234(Str::lit("name"), v1);
            match m3336 {
                Adt110::C0 => {
                    None::<Str>
                }
                Adt110::C1(v2) => {
                    f133(v2)
                }
            }
        }
        Adt155::C2 => {
            let m3337 = f234(Str::lit("uri"), v1);
            match m3337 {
                Adt110::C0 => {
                    None::<Str>
                }
                Adt110::C1(v3) => {
                    f133(v3)
                }
            }
        }
        Adt155::C3 => {
            let m3338 = f234(Str::lit("taskId"), v1);
            match m3338 {
                Adt110::C0 => {
                    None::<Str>
                }
                Adt110::C1(v4) => {
                    f133(v4)
                }
            }
        }
    }
}

pub fn f938(v0: Adt6) -> Option<Adt18> {
    let m3339 = f239(v0);
    match m3339 {
        None => {
            None::<Adt18>
        }
        Some(v1) => {
            let m3340 = f977(Str::lit("experimental"), v1.clone());
            match m3340 {
                None => {
                    None::<Adt18>
                }
                Some(v2) => {
                    let m3341 = f977(Str::lit("roots"), v1.clone());
                    match m3341 {
                        None => {
                            None::<Adt18>
                        }
                        Some(v3) => {
                            let m3342 = f977(Str::lit("sampling"), v1.clone());
                            match m3342 {
                                None => {
                                    None::<Adt18>
                                }
                                Some(v4) => {
                                    let m3343 = f977(Str::lit("elicitation"), v1.clone());
                                    match m3343 {
                                        None => {
                                            None::<Adt18>
                                        }
                                        Some(v5) => {
                                            let m3344 = f977(Str::lit("extensions"), v1);
                                            match m3344 {
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

pub fn f939() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f940(v0: Adt5) -> Result<Option<Adt19>, Adt142> {
    let m3345 = f886(f978(), v0);
    match m3345 {
        Ok(v2) => {
            let m3346 = v2;
            match m3346 {
                None => {
                    Ok::<Option<Adt19>, Adt142>(None::<Adt19>)
                }
                Some(v3) => {
                    let m3347 = f979(v3);
                    match m3347 {
                        Adt164::C0 => {
                            Err::<Option<Adt19>, Adt142>(Adt142::C8(f978()))
                        }
                        Adt164::C1(v4) => {
                            Ok::<Option<Adt19>, Adt142>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt19>, Adt142>(v1)
        }
    }
}

pub fn f941(v0: Adt5) -> Result<Option<Adt20>, Adt142> {
    let m3348 = f234(f980(), v0);
    match m3348 {
        Adt110::C0 => {
            Ok::<Option<Adt20>, Adt142>(None::<Adt20>)
        }
        Adt110::C1(v1) => {
            let m3349 = f133(v1.clone());
            match m3349 {
                None => {
                    let m3350 = f235(v1);
                    match m3350 {
                        None => {
                            Err::<Option<Adt20>, Adt142>(Adt142::C8(f980()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt20>, Adt142>(Some(Adt20::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt20>, Adt142>(Some(Adt20::C0(v2)))
                }
            }
        }
    }
}

pub fn f942(v0: Adt160) -> Adt160 {
    let m3351 = v0;
    match m3351 {
        Adt160::C0 => {
            Adt160::C2
        }
        Adt160::C1 => {
            Adt160::C1
        }
        Adt160::C2 => {
            Adt160::C0
        }
    }
}

pub fn f943(v0: i64, v1: i64) -> R<i64> {
    f446(f354(f945(v0)?, 0i64)?, v1)
}

pub fn f944(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt160> {
    let a3352 = v1;
    let a3353 = v3;
    if nat_lt(a3352, a3353) {
        Ok(f859(f981({
            let a3354 = v3;
            let a3355 = v1;
            nat_sub(a3354, a3355)
        }, v0)?, v2))
    } else {
        let a3356 = v3;
        let a3357 = v1;
        if nat_lt(a3356, a3357) {
            Ok(f859(v0, f981({
                let a3358 = v1;
                let a3359 = v3;
                nat_sub(a3358, a3359)
            }, v2)?))
        } else {
            Ok(f859(v0, v2))
        }
    }
}

pub fn f945(v0: i64) -> R<u64> {
    f982(20u64, v0, 0u64)
}

pub fn f946(v0: Adt6) -> u64 {
    let m3360 = v0;
    match m3360 {
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

pub fn f947(v0: Adt160) -> bool {
    let m3361 = v0;
    match m3361 {
        Adt160::C0 => {
            false
        }
        Adt160::C1 => {
            true
        }
        Adt160::C2 => {
            false
        }
    }
}

pub fn f948(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt163 {
    let m3362 = v0;
    match m3362 {
        None => {
            Adt163::C0
        }
        Some(v2) => {
            Adt163::C1({
                let a3363 = v2;
                let a3364 = v1;
                append_list(a3363, a3364)
            })
        }
    }
}

pub fn f949(p3369: List<Adt6>, p3370: List<Adt6>, p3371: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3368 = (p3369, (p3370, p3371));
    {
        let mut m3368 = m3368;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3368;
                let m3365 = v0;
                match m3365.uncons() {
                    None => {
                        let m3366 = v1;
                        match m3366.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m3367 = v1;
                        match m3367.uncons() {
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
                Ok(next) => { m3368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f950(p3377: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p3378: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3376 = (p3377, p3378);
    {
        let mut m3376 = m3376;
        loop {
            let step = {
                let (v0, v2) = m3376;
                let m3372 = v0;
                match m3372.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m3374 = f347({
                            let (h3373, _) = v3;
                            h3373
                        }, v1.clone());
                        match m3374 {
                            Adt110::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt110::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h3375) = v3;
                                    h3375
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f951(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3379 = v0;
    match m3379 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f651(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f952(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3380 = v0;
    match m3380 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f983(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f953(p3383: List<Adt58>, p3384: List<Adt6>) -> List<Adt6> {
    let m3382 = (p3383, p3384);
    {
        let mut m3382 = m3382;
        loop {
            let step = {
                let (v0, v1) = m3382;
                let m3381 = v0;
                match m3381.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f984(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f954(p3388: List<Adt71>, p3389: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3387 = (p3388, p3389);
    {
        let mut m3387 = m3387;
        loop {
            let step = {
                let (v0, v1) = m3387;
                let m3385 = v0;
                match m3385.uncons() {
                    None => {
                        Err::<(List<Adt71>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3386 = f985(v2)?;
                        match m3386 {
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
                Ok(next) => { m3387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f955(v0: Adt76) -> Adt6 {
    Adt6::C5({
        let a3399 = {
            let m3391 = match v0.clone() {
                Adt76::C0(h3390, _, _, _) => {
                    h3390
                }
            };
            match m3391 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f445(f986(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a3400 = {
            let a3397 = f957(Str::lit("costPriority"), match v0.clone() {
                Adt76::C0(_, h3392, _, _) => {
                    h3392
                }
            });
            let a3398 = {
                let a3395 = f957(Str::lit("speedPriority"), match v0.clone() {
                    Adt76::C0(_, _, h3393, _) => {
                        h3393
                    }
                });
                let a3396 = f957(Str::lit("intelligencePriority"), match v0 {
                    Adt76::C0(_, _, _, h3394) => {
                        h3394
                    }
                });
                append_list(a3395, a3396)
            };
            append_list(a3397, a3398)
        };
        append_list(a3399, a3400)
    })
}

pub fn f956(v0: Adt78) -> Str {
    let m3401 = v0;
    match m3401 {
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

pub fn f957(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m3402 = v1;
    match m3402 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f958(v0: Adt79) -> List<(Str, Adt6)> {
    let m3404 = match v0 {
        Adt79::C0(h3403) => {
            h3403
        }
    };
    match m3404 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f987(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f959(v0: Adt66) -> List<(Str, Adt6)> {
    let m3405 = v0;
    match m3405 {
        Adt66::C0(v1) => {
            let a3412 = {
                let m3407 = match v1.clone() {
                    Adt67::C0(h3406, _, _) => {
                        h3406
                    }
                };
                match m3407 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3413 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt67::C0(_, h3408, _) => {
                    h3408
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m3411 = match v1 {
                    Adt67::C0(_, _, h3410) => {
                        h3410
                    }
                };
                match m3411 {
                    Adt5::C0(h3409) => {
                        h3409
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a3412, a3413)
        }
        Adt66::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt69::C0(h3414, _) => {
                    h3414
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt69::C0(_, h3415) => {
                    h3415
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f960(v0: Adt87) -> Adt6 {
    let m3416 = v0;
    match m3416 {
        Adt87::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h3417) => {
                    h3417
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt87::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f312(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f961(v0: Adt150, v1: Adt150) -> bool {
    let a3418 = f744(v0);
    let a3419 = f744(v1);
    equal(a3418, a3419)
}

pub fn f962(v0: Adt150) -> bool {
    let m3420 = v0;
    match m3420 {
        Adt150::C0 => {
            false
        }
        Adt150::C1 => {
            false
        }
        Adt150::C2 => {
            true
        }
        Adt150::C3 => {
            false
        }
        Adt150::C4 => {
            true
        }
        Adt150::C5 => {
            false
        }
        Adt150::C6 => {
            false
        }
        Adt150::C7 => {
            true
        }
        Adt150::C8 => {
            false
        }
        Adt150::C9 => {
            false
        }
        Adt150::C10 => {
            false
        }
        Adt150::C11 => {
            false
        }
    }
}

pub fn f963(v0: Adt150) -> bool {
    let m3421 = v0;
    match m3421 {
        Adt150::C0 => {
            false
        }
        Adt150::C1 => {
            false
        }
        Adt150::C2 => {
            true
        }
        Adt150::C3 => {
            false
        }
        Adt150::C4 => {
            false
        }
        Adt150::C5 => {
            false
        }
        Adt150::C6 => {
            false
        }
        Adt150::C7 => {
            false
        }
        Adt150::C8 => {
            false
        }
        Adt150::C9 => {
            false
        }
        Adt150::C10 => {
            false
        }
        Adt150::C11 => {
            false
        }
    }
}

pub fn f964(v0: Adt0, v1: Adt17, v2: Adt82) -> Result<Adt138, Adt141> {
    f988(v0, v2.clone(), f989(match v1 {
        Adt17::C0(_, h3422, _, _, _, _) => {
            h3422
        }
    }, f718(v2)))
}

pub fn f965(v0: Adt15) -> Adt17 {
    let m3423 = v0;
    match m3423 {
        Adt15::C0(v1) => {
            match v1 {
                Adt16::C0(h3424) => {
                    h3424
                }
            }
        }
        Adt15::C1(v2) => {
            match v2 {
                Adt21::C0(h3425, _) => {
                    h3425
                }
            }
        }
        Adt15::C2(v3) => {
            match v3 {
                Adt22::C0(h3426, _, _, _, _) => {
                    h3426
                }
            }
        }
        Adt15::C3(v4) => {
            match v4 {
                Adt21::C0(h3427, _) => {
                    h3427
                }
            }
        }
        Adt15::C4(v5) => {
            match v5 {
                Adt23::C0(h3428, _, _, _) => {
                    h3428
                }
            }
        }
        Adt15::C5(v6) => {
            match v6 {
                Adt21::C0(h3429, _) => {
                    h3429
                }
            }
        }
        Adt15::C6(v7) => {
            match v7 {
                Adt21::C0(h3430, _) => {
                    h3430
                }
            }
        }
        Adt15::C7(v8) => {
            match v8 {
                Adt24::C0(h3431, _, _, _, _) => {
                    h3431
                }
            }
        }
        Adt15::C8(v9) => {
            match v9 {
                Adt25::C0(h3432, _, _, _) => {
                    h3432
                }
            }
        }
        Adt15::C9(v10) => {
            match v10 {
                Adt30::C0(h3433, _) => {
                    h3433
                }
            }
        }
        Adt15::C10(v11) => {
            match v11 {
                Adt31::C0(h3434, _, _) => {
                    h3434
                }
            }
        }
        Adt15::C11(v12) => {
            match v12 {
                Adt32::C0(h3435, _) => {
                    h3435
                }
            }
        }
    }
}

pub fn f966(v0: Adt0, v1: Adt17, v2: Adt83) -> Result<Adt138, Adt141> {
    f990(v0, v2, f188(match v1 {
        Adt17::C0(_, h3436, _, _, _, _) => {
            h3436
        }
    }))
}

pub fn f967(v0: Adt0, v1: Adt17, v2: Adt88) -> Result<Adt138, Adt141> {
    f991(v0, v2, f188(match v1 {
        Adt17::C0(_, h3437, _, _, _, _) => {
            h3437
        }
    }))
}

pub fn f968(v0: Adt0, v1: Adt17, v2: Adt50) -> Adt138 {
    Adt138::C0(Adt33::C2(match v2.clone() {
        Adt50::C0(h3438, _) => {
            h3438
        }
    }), {
        let m3440 = f882(v0, Adt33::C11);
        match m3440 {
            Adt138::C0(_, h3439, _) => {
                h3439
            }
        }
    }, f992(v1, match v2 {
        Adt50::C0(_, h3441) => {
            h3441
        }
    }))
}

pub fn f969(v0: Bytes, v1: Adt15, v2: Result<Adt33, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m3442 = v2;
    match m3442 {
        Ok(v4) => {
            f993(v0, f881(v1.clone()), f994(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f970(v0: u64, v1: u64, v2: Fn4, v3: Adt15) -> R<Result<Adt33, Adt90>> {
    let m3443 = v3.clone();
    match m3443 {
        Adt15::C0(_) => {
            let c3445: Fn4 = v2;
            c3445.apply(v3)
        }
        Adt15::C1(_) => {
            let c3446: Fn4 = v2;
            c3446.apply(v3)
        }
        Adt15::C2(_) => {
            if f995(v0, v1) {
                let c3444: Fn4 = v2;
                c3444.apply(v3)
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C7(v0)))
            }
        }
        Adt15::C3(_) => {
            let c3447: Fn4 = v2;
            c3447.apply(v3)
        }
        Adt15::C4(_) => {
            let c3448: Fn4 = v2;
            c3448.apply(v3)
        }
        Adt15::C5(_) => {
            let c3449: Fn4 = v2;
            c3449.apply(v3)
        }
        Adt15::C6(_) => {
            let c3450: Fn4 = v2;
            c3450.apply(v3)
        }
        Adt15::C7(_) => {
            let c3451: Fn4 = v2;
            c3451.apply(v3)
        }
        Adt15::C8(_) => {
            let c3452: Fn4 = v2;
            c3452.apply(v3)
        }
        Adt15::C9(_) => {
            let c3453: Fn4 = v2;
            c3453.apply(v3)
        }
        Adt15::C10(_) => {
            let c3454: Fn4 = v2;
            c3454.apply(v3)
        }
        Adt15::C11(_) => {
            let c3455: Fn4 = v2;
            c3455.apply(v3)
        }
    }
}

pub fn f971(v0: Bytes, v1: Adt22) -> R<Result<Adt15, Adt90>> {
    let m3459 = f996(v0, Str::lit("tools/call"), match v1.clone() {
        Adt22::C0(_, h3456, _, _, _) => {
            h3456
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, _, h3457) => {
            h3457
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h3458, _) => {
            h3458
        }
    })?;
    match m3459 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C2(Adt22::C0(match v1.clone() {
                Adt22::C0(h3460, _, _, _, _) => {
                    h3460
                }
            }, match v1.clone() {
                Adt22::C0(_, h3461, _, _, _) => {
                    h3461
                }
            }, match v1.clone() {
                Adt22::C0(_, _, h3462, _, _) => {
                    h3462
                }
            }, match v1 {
                Adt22::C0(_, _, _, h3463, _) => {
                    h3463
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f972(v0: Bytes, v1: Adt23) -> R<Result<Adt15, Adt90>> {
    let m3467 = f996(v0, Str::lit("resources/read"), match v1.clone() {
        Adt23::C0(_, h3464, _, _) => {
            h3464
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h3465) => {
            h3465
        }
    }, match v1.clone() {
        Adt23::C0(_, _, h3466, _) => {
            h3466
        }
    })?;
    match m3467 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C4(Adt23::C0(match v1.clone() {
                Adt23::C0(h3468, _, _, _) => {
                    h3468
                }
            }, match v1.clone() {
                Adt23::C0(_, h3469, _, _) => {
                    h3469
                }
            }, match v1 {
                Adt23::C0(_, _, h3470, _) => {
                    h3470
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f973(v0: Bytes, v1: Adt24) -> R<Result<Adt15, Adt90>> {
    let m3474 = f996(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt24::C0(_, h3471, _, _, _) => {
            h3471
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, _, h3472) => {
            h3472
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, h3473, _) => {
            h3473
        }
    })?;
    match m3474 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C7(Adt24::C0(match v1.clone() {
                Adt24::C0(h3475, _, _, _, _) => {
                    h3475
                }
            }, match v1.clone() {
                Adt24::C0(_, h3476, _, _, _) => {
                    h3476
                }
            }, match v1.clone() {
                Adt24::C0(_, _, h3477, _, _) => {
                    h3477
                }
            }, match v1 {
                Adt24::C0(_, _, _, h3478, _) => {
                    h3478
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f974(v0: Str, p3484: List<(Str, Adt6)>, p3485: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt142> {
    let m3483 = (p3484, p3485);
    {
        let mut m3483 = m3483;
        loop {
            let step = {
                let (v1, v2) = m3483;
                let m3479 = v1;
                match m3479.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt142>>(Ok::<List<(Str, Str)>, Adt142>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3481 = f133({
                            let (_, h3480) = v3;
                            h3480
                        });
                        match m3481 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt142>>(Err::<List<(Str, Str)>, Adt142>(Adt142::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt142>>((v4, List::cons(({
                                    let (h3482, _) = v3;
                                    h3482
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3483 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f975(p3488: List<(Str, Str)>, p3489: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3487 = (p3488, p3489);
    {
        let mut m3487 = m3487;
        loop {
            let step = {
                let (v0, v1) = m3487;
                let m3486 = v0;
                match m3486.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f976(v0: Adt123, v1: Adt5) -> R<Option<Adt121>> {
    let m3491 = f997(match v0.clone() {
        Adt123::C0(_, h3490, _, _) => {
            h3490
        }
    }, v1);
    match m3491 {
        Adt110::C0 => {
            Ok(f998(v0))
        }
        Adt110::C1(v2) => {
            if f999(v2.clone()) {
                Ok(f998(v0))
            } else {
                f1000(v0, v2)
            }
        }
    }
}

pub fn f977(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m3492 = f234(v0, v1);
    match m3492 {
        Adt110::C0 => {
            Some(None::<Adt5>)
        }
        Adt110::C1(v2) => {
            let m3493 = f239(v2);
            match m3493 {
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

pub fn f978() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f979(v0: Str) -> Adt164 {
    let a3494 = v0.clone();
    let a3495 = Str::lit("debug");
    if equal(a3494, a3495) {
        Adt164::C1(Adt19::C0)
    } else {
        let a3496 = v0.clone();
        let a3497 = Str::lit("info");
        if equal(a3496, a3497) {
            Adt164::C1(Adt19::C1)
        } else {
            let a3498 = v0.clone();
            let a3499 = Str::lit("notice");
            if equal(a3498, a3499) {
                Adt164::C1(Adt19::C2)
            } else {
                let a3500 = v0.clone();
                let a3501 = Str::lit("warning");
                if equal(a3500, a3501) {
                    Adt164::C1(Adt19::C3)
                } else {
                    let a3502 = v0.clone();
                    let a3503 = Str::lit("error");
                    if equal(a3502, a3503) {
                        Adt164::C1(Adt19::C4)
                    } else {
                        let a3504 = v0.clone();
                        let a3505 = Str::lit("critical");
                        if equal(a3504, a3505) {
                            Adt164::C1(Adt19::C5)
                        } else {
                            let a3506 = v0.clone();
                            let a3507 = Str::lit("alert");
                            if equal(a3506, a3507) {
                                Adt164::C1(Adt19::C6)
                            } else {
                                let a3508 = v0;
                                let a3509 = Str::lit("emergency");
                                if equal(a3508, a3509) {
                                    Adt164::C1(Adt19::C7)
                                } else {
                                    Adt164::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f980() -> Str {
    Str::lit("progressToken")
}

pub fn f981(p3514: u64, p3515: i64) -> R<i64> {
    let m3513 = (p3514, p3515);
    {
        let mut m3513 = m3513;
        loop {
            let step = {
                let (v0, v1) = m3513;
                let m3510 = v0;
                if m3510 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3510 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3511 = v1;
                        let a3512 = 10i64;
                        int_mul(a3511, a3512)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f982(p3526: u64, p3527: i64, p3528: u64) -> R<u64> {
    let m3525 = (p3526, (p3527, p3528));
    {
        let mut m3525 = m3525;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3525;
                let m3516 = v0;
                if m3516 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3516 - 1;
                    let v4: u64 = {
                        let a3517 = v2;
                        let a3518 = 1u64;
                        nat_add(a3517, a3518)?
                    };
                    let m3521 = {
                        let a3519 = v1;
                        let a3520 = 10i64;
                        compare(a3519, a3520)
                    };
                    if match m3521 {
                        Ordering::Less => {
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
                            let a3522 = v1;
                            let a3523 = 10i64;
                            let a3524 = 0i64;
                            int_quot(a3522, a3523, a3524)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3525 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f983(v0: Adt3) -> Str {
    let m3529 = v0;
    match m3529 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f984(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a3538 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h3530, _, _, _) => {
                h3530
            }
        })), List::<(Str, Adt6)>::nil());
        let a3539 = {
            let a3536 = f698(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h3531, _, _) => {
                    h3531
                }
            });
            let a3537 = {
                let a3534 = f698(Str::lit("description"), match v0.clone() {
                    Adt58::C0(_, _, h3532, _) => {
                        h3532
                    }
                });
                let a3535 = f352(Str::lit("required"), match v0 {
                    Adt58::C0(_, _, _, h3533) => {
                        h3533
                    }
                });
                append_list(a3534, a3535)
            };
            append_list(a3536, a3537)
        };
        append_list(a3538, a3539)
    })
}

pub fn f985(v0: Adt71) -> R<Option<Adt6>> {
    let m3541 = f1001(match v0.clone() {
        Adt71::C0(_, h3540, _) => {
            h3540
        }
    })?;
    match m3541 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f919(match v0.clone() {
                Adt71::C0(h3542, _, _) => {
                    h3542
                }
            }))), List::cons((Str::lit("content"), v1), f697(match v0 {
                Adt71::C0(_, _, h3543) => {
                    h3543
                }
            }))))))
        }
    }
}

pub fn f986(p3547: List<Adt77>, p3548: List<Adt6>) -> List<Adt6> {
    let m3546 = (p3547, p3548);
    {
        let mut m3546 = m3546;
        loop {
            let step = {
                let (v0, v1) = m3546;
                let m3544 = v0;
                match m3544.uncons() {
                    None => {
                        Err::<(List<Adt77>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt77>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f698(Str::lit("name"), match v2 {
                            Adt77::C0(h3545) => {
                                h3545
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f987(v0: Adt80) -> Str {
    let m3549 = v0;
    match m3549 {
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

pub fn f988(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt138, Adt141> {
    if v2 {
        Ok::<Adt138, Adt141>(f882(v0, Adt33::C12(v1)))
    } else {
        Err::<Adt138, Adt141>(Adt141::C4(Adt90::C8(f1002(f718(v1)))))
    }
}

pub fn f989(v0: Adt18, v1: List<(Str, Adt65)>) -> bool {
    let a3550 = f1003(v1.clone());
    if if if bool_not(a3550) {
        true
    } else {
        f362(match v0.clone() {
            Adt18::C0(_, _, _, h3551, _) => {
                h3551
            }
        })
    } {
        let a3552 = f1004(v1.clone());
        if bool_not(a3552) {
            true
        } else {
            f362(match v0.clone() {
                Adt18::C0(_, _, h3553, _, _) => {
                    h3553
                }
            })
        }
    } else {
        false
    } {
        let a3554 = f1005(v1);
        if bool_not(a3554) {
            true
        } else {
            f362(match v0 {
                Adt18::C0(_, h3555, _, _, _) => {
                    h3555
                }
            })
        }
    } else {
        false
    }
}

pub fn f990(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt138, Adt141> {
    if v2 {
        Ok::<Adt138, Adt141>(f882(v0, Adt33::C13(v1)))
    } else {
        Err::<Adt138, Adt141>(Adt141::C4(Adt90::C8(f298())))
    }
}

pub fn f991(v0: Adt0, v1: Adt88, v2: bool) -> Result<Adt138, Adt141> {
    if v2 {
        Ok::<Adt138, Adt141>(f882(v0, Adt33::C14(v1)))
    } else {
        Err::<Adt138, Adt141>(Adt141::C4(Adt90::C8(f298())))
    }
}

pub fn f992(v0: Adt17, v1: List<Adt51>) -> List<Adt140> {
    let m3557 = match v0 {
        Adt17::C0(_, _, _, _, h3556, _) => {
            h3556
        }
    };
    match m3557 {
        None => {
            List::<Adt140>::nil()
        }
        Some(v2) => {
            f1006(f1007(v2, v1, List::<Adt140>::nil()), List::<Adt140>::nil())
        }
    }
}

pub fn f993(v0: Bytes, v1: Adt150, v2: Str, v3: Adt33) -> R<Result<Adt33, Adt90>> {
    let m3558 = v3.clone();
    match m3558 {
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
            f1008(v0, v1, v2, v4)
        }
        Adt33::C13(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C14(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f994(v0: Adt15) -> Str {
    let m3559 = v0;
    match m3559 {
        Adt15::C0(_) => {
            Str::lit("")
        }
        Adt15::C1(_) => {
            Str::lit("")
        }
        Adt15::C2(v1) => {
            match v1 {
                Adt22::C0(_, h3560, _, _, _) => {
                    h3560
                }
            }
        }
        Adt15::C3(_) => {
            Str::lit("")
        }
        Adt15::C4(v2) => {
            match v2 {
                Adt23::C0(_, h3561, _, _) => {
                    h3561
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
                Adt24::C0(_, h3562, _, _, _) => {
                    h3562
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

pub fn f995(v0: u64, v1: u64) -> bool {
    let a3563 = v1;
    let a3564 = v0;
    nat_lt(a3563, a3564)
}

pub fn f996(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt90>> {
    Ok(f1009(f1010(v0, v1, v2, v3)?, f1011(v4)))
}

pub fn f997(p3570: List<Str>, p3571: Adt5) -> Adt110 {
    let m3569 = (p3570, p3571);
    {
        let mut m3569 = m3569;
        loop {
            let step = {
                let (v0, v1) = m3569;
                let m3565 = v0;
                match m3565.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt110>(Adt110::C0)
                    }
                    Some((v2, v3)) => {
                        let m3566 = f234(v2, v1);
                        match m3566 {
                            Adt110::C0 => {
                                Err::<(List<Str>, Adt5), Adt110>(Adt110::C0)
                            }
                            Adt110::C1(v4) => {
                                let m3567 = v3.clone();
                                match m3567.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt110>(Adt110::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3568 = f239(v4);
                                        match m3568 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt110>(Adt110::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt110>((v3, v7))
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
                Ok(next) => { m3569 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f998(v0: Adt123) -> Option<Adt121> {
    let m3573 = match v0.clone() {
        Adt123::C0(_, _, _, h3572) => {
            h3572
        }
    };
    match m3573 {
        Adt124::C0 => {
            None::<Adt121>
        }
        Adt124::C1 => {
            Some(Adt121::C10(match v0 {
                Adt123::C0(h3574, _, _, _) => {
                    h3574
                }
            }))
        }
        Adt124::C2(_) => {
            None::<Adt121>
        }
    }
}

pub fn f999(v0: Adt6) -> bool {
    let m3575 = v0;
    match m3575 {
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

pub fn f1000(v0: Adt123, v1: Adt6) -> R<Option<Adt121>> {
    let m3577 = match v0.clone() {
        Adt123::C0(_, _, _, h3576) => {
            h3576
        }
    };
    match m3577 {
        Adt124::C0 => {
            Ok(Some(Adt121::C8(match v0 {
                Adt123::C0(h3578, _, _, _) => {
                    h3578
                }
            })))
        }
        Adt124::C1 => {
            Ok(Some(Adt121::C10(match v0 {
                Adt123::C0(h3579, _, _, _) => {
                    h3579
                }
            })))
        }
        Adt124::C2(v2) => {
            if f1012(match v0.clone() {
                Adt123::C0(_, _, h3580, _) => {
                    h3580
                }
            }, v2, v1)? {
                Ok(None::<Adt121>)
            } else {
                Ok(Some(Adt121::C9(match v0 {
                    Adt123::C0(h3581, _, _, _) => {
                        h3581
                    }
                })))
            }
        }
    }
}

pub fn f1001(v0: Adt72) -> R<Option<Adt6>> {
    let m3582 = v0;
    match m3582 {
        Adt72::C0(v1) => {
            f1013(v1)
        }
        Adt72::C1(v2) => {
            let m3583 = f1014(v2, List::<Adt6>::nil())?;
            match m3583 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f445(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f1002(v0: List<(Str, Adt65)>) -> Adt18 {
    Adt18::C0(None::<Adt5>, if f1005(v0.clone()) {
        Some(f27())
    } else {
        None::<Adt5>
    }, if f1004(v0.clone()) {
        Some(f27())
    } else {
        None::<Adt5>
    }, if f1003(v0) {
        Some(f27())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f1003(p3588: List<(Str, Adt65)>) -> bool {
    let m3587 = p3588;
    {
        let mut m3587 = m3587;
        loop {
            let step = {
                let v0 = m3587;
                let m3584 = v0;
                match m3584.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3586 = {
                            let (_, h3585) = v1;
                            h3585
                        };
                        match m3586 {
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
                Ok(next) => { m3587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1004(p3593: List<(Str, Adt65)>) -> bool {
    let m3592 = p3593;
    {
        let mut m3592 = m3592;
        loop {
            let step = {
                let v0 = m3592;
                let m3589 = v0;
                match m3589.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3591 = {
                            let (_, h3590) = v1;
                            h3590
                        };
                        match m3591 {
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
                Ok(next) => { m3592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1005(p3598: List<(Str, Adt65)>) -> bool {
    let m3597 = p3598;
    {
        let mut m3597 = m3597;
        loop {
            let step = {
                let v0 = m3597;
                let m3594 = v0;
                match m3594.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3596 = {
                            let (_, h3595) = v1;
                            h3595
                        };
                        match m3596 {
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
                Ok(next) => { m3597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1006(p3601: List<Adt140>, p3602: List<Adt140>) -> List<Adt140> {
    let m3600 = (p3601, p3602);
    {
        let mut m3600 = m3600;
        loop {
            let step = {
                let (v0, v1) = m3600;
                let m3599 = v0;
                match m3599.uncons() {
                    None => {
                        Err::<(List<Adt140>, List<Adt140>), List<Adt140>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt140>, List<Adt140>), List<Adt140>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1007(v0: Adt20, p3605: List<Adt51>, p3606: List<Adt140>) -> List<Adt140> {
    let m3604 = (p3605, p3606);
    {
        let mut m3604 = m3604;
        loop {
            let step = {
                let (v1, v2) = m3604;
                let m3603 = v1;
                match m3603.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt140>), List<Adt140>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt51>, List<Adt140>), List<Adt140>>((v4, List::cons(Adt140::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3604 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1008(v0: Bytes, v1: Adt150, v2: Str, v3: Adt82) -> R<Result<Adt33, Adt90>> {
    if f962(v1.clone()) {
        if f1015(v3.clone()) {
            let m3607 = f1016(v0, f744(v1), v2, v3)?;
            match m3607 {
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
        Ok(Err::<Adt33, Adt90>(Adt90::C12(f744(v1))))
    }
}

pub fn f1009(v0: Result<Option<Str>, Adt90>, v1: bool) -> Result<Option<Str>, Adt90> {
    let m3608 = v0;
    match m3608 {
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

pub fn f1010(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt90>> {
    let m3609 = v3;
    match m3609 {
        None => {
            Ok(Ok::<Option<Str>, Adt90>(None::<Str>))
        }
        Some(v4) => {
            Ok(f1017(f1018(v0, v1, v2, v4)?))
        }
    }
}

pub fn f1011(v0: Option<Adt5>) -> bool {
    let m3610 = v0;
    match m3610 {
        None => {
            true
        }
        Some(v1) => {
            f1019(match v1 {
                Adt5::C0(h3611) => {
                    h3611
                }
            })
        }
    }
}

pub fn f1012(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3612 = v0;
    match m3612 {
        Adt13::C0 => {
            let m3613 = f133(v2);
            match m3613 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3614 = v1;
                    let a3615 = v3;
                    Ok(equal(a3614, a3615))
                }
            }
        }
        Adt13::C1 => {
            f1020(v1, v2)
        }
        Adt13::C2 => {
            let m3616 = f447(v2);
            match m3616 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3617 = v1;
                        let a3618 = Str::lit("true");
                        Ok(equal(a3617, a3618))
                    } else {
                        let a3619 = v1;
                        let a3620 = Str::lit("false");
                        Ok(equal(a3619, a3620))
                    }
                }
            }
        }
    }
}

pub fn f1013(v0: Adt73) -> R<Option<Adt6>> {
    let m3621 = v0;
    match m3621 {
        Adt73::C0(v1) => {
            Ok(Some(f609(v1)))
        }
        Adt73::C1(v2) => {
            Ok(Some(f610(Str::lit("image"), match v2.clone() {
                Adt43::C0(h3622, _, _, _) => {
                    h3622
                }
            }, match v2.clone() {
                Adt43::C0(_, h3623, _, _) => {
                    h3623
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h3624, _) => {
                    h3624
                }
            }, match v2 {
                Adt43::C0(_, _, _, h3625) => {
                    h3625
                }
            })))
        }
        Adt73::C2(v3) => {
            Ok(Some(f610(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h3626, _, _, _) => {
                    h3626
                }
            }, match v3.clone() {
                Adt44::C0(_, h3627, _, _) => {
                    h3627
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h3628, _) => {
                    h3628
                }
            }, match v3 {
                Adt44::C0(_, _, _, h3629) => {
                    h3629
                }
            })))
        }
        Adt73::C3(v4) => {
            Ok(Some(f1021(v4)))
        }
        Adt73::C4(v5) => {
            f1022(v5)
        }
    }
}

pub fn f1014(p3633: List<Adt73>, p3634: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3632 = (p3633, p3634);
    {
        let mut m3632 = m3632;
        loop {
            let step = {
                let (v0, v1) = m3632;
                let m3630 = v0;
                match m3630.uncons() {
                    None => {
                        Err::<(List<Adt73>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3631 = f1013(v2)?;
                        match m3631 {
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
                Ok(next) => { m3632 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1015(v0: Adt82) -> bool {
    let m3635 = v0;
    match m3635 {
        Adt82::C0(v1) => {
            let m3636 = v1;
            match m3636.uncons() {
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

pub fn f1016(v0: Bytes, v1: Str, v2: Str, v3: Adt82) -> R<Result<Adt82, Adt90>> {
    let m3637 = v3.clone();
    match m3637 {
        Adt82::C0(_) => {
            Ok(Ok::<Adt82, Adt90>(v3))
        }
        Adt82::C1(v5) => {
            Ok(f1023(f1024(v0, v1, v2, v5)?, List::<(Str, Adt65)>::nil()))
        }
        Adt82::C2(v6, v7) => {
            Ok(f1023(f1024(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f1017(v0: Option<Str>) -> Result<Option<Str>, Adt90> {
    let m3638 = v0;
    match m3638 {
        None => {
            Err::<Option<Str>, Adt90>(Adt90::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt90>(Some(v1))
        }
    }
}

pub fn f1018(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3642 = f1025(v0, {
        let a3639 = v1;
        utf8_encode(a3639)
    }, {
        let a3640 = v2;
        utf8_encode(a3640)
    }, {
        let a3641 = v3;
        utf8_encode(a3641)
    })?;
    match m3642 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3643 = v4;
            Ok(utf8_decode(a3643))
        }
    }
}

pub fn f1019(p3648: List<(Str, Adt6)>) -> bool {
    let m3647 = p3648;
    {
        let mut m3647 = m3647;
        loop {
            let step = {
                let v0 = m3647;
                let m3644 = v0;
                match m3644.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3646 = f239({
                            let (_, h3645) = v1;
                            h3645
                        });
                        match m3646 {
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
                Ok(next) => { m3647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1020(v0: Str, v1: Adt6) -> R<bool> {
    let m3649 = f770(v1);
    match m3649 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3650 = f497(v2)?;
            match m3650 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3652 = f1026(f198({
                        let a3651 = v0;
                        utf8_encode(a3651)
                    })?)?;
                    match m3652 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f348({
                                let a3653 = v3;
                                let a3654 = v4;
                                int_sub(a3653, a3654)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f1021(v0: Adt74) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt74::C0(h3655, _, _, _) => {
            h3655
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt74::C0(_, h3656, _, _) => {
            h3656
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3659 = match v0.clone() {
            Adt74::C0(_, _, h3658, _) => {
                h3658
            }
        };
        match m3659 {
            Adt5::C0(h3657) => {
                h3657
            }
        }
    })), f697(match v0 {
        Adt74::C0(_, _, _, h3660) => {
            h3660
        }
    }))))))
}

pub fn f1022(v0: Adt75) -> R<Option<Adt6>> {
    let m3662 = f350(match v0.clone() {
        Adt75::C0(_, h3661, _, _, _) => {
            h3661
        }
    })?;
    match m3662 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt75::C0(h3663, _, _, _, _) => {
                    h3663
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3669 = f351(Str::lit("structuredContent"), match v0.clone() {
                    Adt75::C0(_, _, h3664, _, _) => {
                        h3664
                    }
                });
                let a3670 = {
                    let a3667 = f352(Str::lit("isError"), match v0.clone() {
                        Adt75::C0(_, _, _, h3665, _) => {
                            h3665
                        }
                    });
                    let a3668 = f697(match v0 {
                        Adt75::C0(_, _, _, _, h3666) => {
                            h3666
                        }
                    });
                    append_list(a3667, a3668)
                };
                append_list(a3669, a3670)
            }))))))
        }
    }
}

pub fn f1023(v0: Option<Str>, v1: List<(Str, Adt65)>) -> Result<Adt82, Adt90> {
    let m3671 = v0;
    match m3671 {
        None => {
            Err::<Adt82, Adt90>(Adt90::C14)
        }
        Some(v2) => {
            let m3672 = v1.clone();
            match m3672.uncons() {
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

pub fn f1024(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3676 = f1027(v0, {
        let a3673 = v1;
        utf8_encode(a3673)
    }, {
        let a3674 = v2;
        utf8_encode(a3674)
    }, {
        let a3675 = v3;
        utf8_encode(a3675)
    })?;
    match m3676 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3677 = v4;
            Ok(utf8_decode(a3677))
        }
    }
}

pub fn f1025(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3679 = f1028();
    let a3680 = {
        let a3678 = v0.clone();
        length_bytes(a3678)
    };
    if nat_le(a3679, a3680) {
        let a3682 = f1029(v3.clone())?;
        let a3683 = {
            let a3681 = v3.clone();
            length_bytes(a3681)
        };
        if nat_eq(a3682, a3683) {
            Ok(None::<Bytes>)
        } else {
            f1030(v0, v1, v2, v3.clone(), f1029(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1026(v0: List<u64>) -> R<Option<i64>> {
    let m3684 = v0.clone();
    match m3684.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3685 = v1;
            let a3686 = 45u64;
            if nat_eq(a3685, a3686) {
                let m3687 = v2.clone();
                match m3687.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3688 = f1031(v2, 0i64)?;
                        match m3688 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3689 = v5;
                                    int_neg(a3689)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f1031(v0, 0i64)
            }
        }
    }
}

pub fn f1027(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3691 = f1028();
    let a3692 = {
        let a3690 = v0.clone();
        length_bytes(a3690)
    };
    if nat_le(a3691, a3692) {
        Ok(Some({
            let a3695 = f1032(v3.clone())?;
            let a3696 = {
                let a3693 = Bytes::lit(&[0x2e]);
                let a3694 = f1032(f1033(v0, v1, v2, v3)?)?;
                append_bytes(a3693, a3694)
            };
            append_bytes(a3695, a3696)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1028() -> u64 {
    32u64
}

pub fn f1029(v0: Bytes) -> R<u64> {
    let (_, h3697) = f1034(Fn9::F1035(v0.clone()), {
        let a3698 = v0.clone();
        length_bytes(a3698)
    }, (0u64, {
        let a3699 = v0;
        length_bytes(a3699)
    }))?;
    Ok(h3697)
}

pub fn f1030(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3703 = {
        let a3700 = v3.clone();
        let a3701 = 0u64;
        let a3702 = v4;
        slice_bytes(a3700, a3701, a3702)
    };
    match m3703 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3714 = {
                let a3711 = v3.clone();
                let a3712 = {
                    let a3704 = v4;
                    let a3705 = 1u64;
                    nat_add(a3704, a3705)?
                };
                let a3713 = {
                    let a3709 = {
                        let a3706 = v3;
                        length_bytes(a3706)
                    };
                    let a3710 = {
                        let a3707 = v4;
                        let a3708 = 1u64;
                        nat_add(a3707, a3708)?
                    };
                    nat_sub(a3709, a3710)
                };
                slice_bytes(a3711, a3712, a3713)
            };
            match m3714 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f1036(v0, v1, v2, f1037(v5)?, f1037(v6)?)
                }
            }
        }
    }
}

pub fn f1031(p3720: List<u64>, p3721: i64) -> R<Option<i64>> {
    let m3719 = (p3720, p3721);
    {
        let mut m3719 = m3719;
        loop {
            let step = {
                let (v0, v1) = m3719;
                let m3715 = v0;
                match m3715.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3716 = f1038(v2);
                        match m3716 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f446({
                                    let a3717 = v1;
                                    let a3718 = 10i64;
                                    int_mul(a3717, a3718)?
                                }, f583(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3719 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1032(v0: Bytes) -> R<Bytes> {
    let (_, h3722) = f1039(Fn10::F1040(v0.clone()), {
        let a3723 = v0;
        length_bytes(a3723)
    }, (0u64, f200()))?;
    Ok(h3722)
}

pub fn f1033(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f1041(v0, {
        let a3724 = f1042(v1, v2);
        let a3725 = v3;
        append_bytes(a3724, a3725)
    })
}

pub fn f1034(v0: Fn9, p3729: u64, p3730: (u64, u64)) -> R<(u64, u64)> {
    let m3728 = (p3729, p3730);
    {
        let mut m3728 = m3728;
        loop {
            let step = {
                let (v1, v2) = m3728;
                let m3726 = v1;
                if m3726 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3726 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3727: Fn9 = v0.clone();
                        c3727.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1035(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3732 = {
            let (h3731, _) = v1;
            h3731
        };
        let a3733 = 1u64;
        nat_add(a3732, a3733)?
    }, {
        let a3736 = {
            let (_, h3734) = v1;
            h3734
        };
        let a3737 = {
            let a3735 = v0.clone();
            length_bytes(a3735)
        };
        if nat_eq(a3736, a3737) {
            let a3739 = f100(v0, {
                let (h3738, _) = v1;
                h3738
            })?;
            let a3740 = 46u64;
            if nat_eq(a3739, a3740) {
                let (h3741, _) = v1;
                h3741
            } else {
                let (_, h3742) = v1;
                h3742
            }
        } else {
            let (_, h3743) = v1;
            h3743
        }
    }))
}

pub fn f1036(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3744 = v3;
    match m3744 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3745 = v4;
            match m3745 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f1043(f1033(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f1037(v0: Bytes) -> R<Option<Bytes>> {
    let a3750 = {
        let a3747 = {
            let a3746 = v0.clone();
            length_bytes(a3746)
        };
        let a3748 = 2u64;
        let a3749 = 0u64;
        nat_rem(a3747, a3748, a3749)
    };
    let a3751 = 0u64;
    if nat_eq(a3750, a3751) {
        let v1: (u64, (Bytes, u64)) = f1044(Fn11::F1045(v0.clone()), {
            let a3753 = {
                let a3752 = v0;
                length_bytes(a3752)
            };
            let a3754 = 2u64;
            let a3755 = 0u64;
            nat_quot(a3753, a3754, a3755)
        }, (0u64, (f200(), 0u64)))?;
        let a3758 = {
            let (_, h3756) = {
                let (_, h3757) = v1.clone();
                h3757
            };
            h3756
        };
        let a3759 = 0u64;
        if nat_eq(a3758, a3759) {
            Ok(Some({
                let (h3760, _) = {
                    let (_, h3761) = v1;
                    h3761
                };
                h3760
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1038(v0: u64) -> Option<u64> {
    let a3762 = v0;
    let a3763 = 48u64;
    if if nat_lt(a3762, a3763) {
        true
    } else {
        let a3764 = 57u64;
        let a3765 = v0;
        nat_lt(a3764, a3765)
    } {
        None::<u64>
    } else {
        Some({
            let a3766 = v0;
            let a3767 = 48u64;
            nat_sub(a3766, a3767)
        })
    }
}

pub fn f1039(v0: Fn10, p3771: u64, p3772: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3770 = (p3771, p3772);
    {
        let mut m3770 = m3770;
        loop {
            let step = {
                let (v1, v2) = m3770;
                let m3768 = v1;
                if m3768 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3768 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3769: Fn10 = v0.clone();
                        c3769.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1040(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3774 = {
            let (h3773, _) = v1.clone();
            h3773
        };
        let a3775 = 1u64;
        nat_add(a3774, a3775)?
    }, {
        let a3778 = {
            let (_, h3776) = v1;
            h3776
        };
        let a3779 = f1046(f100(v0, {
            let (h3777, _) = v1;
            h3777
        })?)?;
        append_bytes(a3778, a3779)
    }))
}

pub fn f1041(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f1047(f1048(v0, v1)?)
}

pub fn f1042(v0: Bytes, v1: Bytes) -> Bytes {
    let a3786 = f1049({
        let a3780 = v0.clone();
        length_bytes(a3780)
    });
    let a3787 = {
        let a3784 = v0;
        let a3785 = {
            let a3782 = f1049({
                let a3781 = v1.clone();
                length_bytes(a3781)
            });
            let a3783 = v1;
            append_bytes(a3782, a3783)
        };
        append_bytes(a3784, a3785)
    };
    append_bytes(a3786, a3787)
}

pub fn f1043(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3790 = {
        let a3788 = v0.clone();
        length_bytes(a3788)
    };
    let a3791 = {
        let a3789 = v1.clone();
        length_bytes(a3789)
    };
    if nat_eq(a3790, a3791) {
        let a3792 = f1050(v0, v1)?;
        let a3793 = 0u64;
        Ok(nat_eq(a3792, a3793))
    } else {
        Ok(false)
    }
}

pub fn f1044(v0: Fn11, p3797: u64, p3798: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3796 = (p3797, p3798);
    {
        let mut m3796 = m3796;
        loop {
            let step = {
                let (v1, v2) = m3796;
                let m3794 = v1;
                if m3794 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3794 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3795: Fn11 = v0.clone();
                        c3795.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1045(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f1051(v0, v1)
}

pub fn f1046(v0: u64) -> R<Bytes> {
    let a3805 = f1052({
        let a3799 = v0;
        let a3800 = 16u64;
        let a3801 = 0u64;
        nat_quot(a3799, a3800, a3801)
    })?;
    let a3806 = f1052({
        let a3802 = v0;
        let a3803 = 16u64;
        let a3804 = 0u64;
        nat_rem(a3802, a3803, a3804)
    })?;
    Ok(append_bytes(a3805, a3806))
}

pub fn f1047(v0: Adt165) -> R<Bytes> {
    let a3827 = f1053(match v0.clone() {
        Adt165::C0(h3807, _, _, _, _, _, _, _) => {
            h3807
        }
    })?;
    let a3828 = {
        let a3825 = f1053(match v0.clone() {
            Adt165::C0(_, h3808, _, _, _, _, _, _) => {
                h3808
            }
        })?;
        let a3826 = {
            let a3823 = f1053(match v0.clone() {
                Adt165::C0(_, _, h3809, _, _, _, _, _) => {
                    h3809
                }
            })?;
            let a3824 = {
                let a3821 = f1053(match v0.clone() {
                    Adt165::C0(_, _, _, h3810, _, _, _, _) => {
                        h3810
                    }
                })?;
                let a3822 = {
                    let a3819 = f1053(match v0.clone() {
                        Adt165::C0(_, _, _, _, h3811, _, _, _) => {
                            h3811
                        }
                    })?;
                    let a3820 = {
                        let a3817 = f1053(match v0.clone() {
                            Adt165::C0(_, _, _, _, _, h3812, _, _) => {
                                h3812
                            }
                        })?;
                        let a3818 = {
                            let a3815 = f1053(match v0.clone() {
                                Adt165::C0(_, _, _, _, _, _, h3813, _) => {
                                    h3813
                                }
                            })?;
                            let a3816 = f1053(match v0 {
                                Adt165::C0(_, _, _, _, _, _, _, h3814) => {
                                    h3814
                                }
                            })?;
                            append_bytes(a3815, a3816)
                        };
                        append_bytes(a3817, a3818)
                    };
                    append_bytes(a3819, a3820)
                };
                append_bytes(a3821, a3822)
            };
            append_bytes(a3823, a3824)
        };
        append_bytes(a3825, a3826)
    };
    Ok(append_bytes(a3827, a3828))
}

pub fn f1048(v0: Bytes, v1: Bytes) -> R<Adt165> {
    let v2: Adt166 = f1054(v0)?;
    let v3: Adt165 = f1055(f1056(f1057(), f1058(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f1056(f1056(f1057(), f1058(v2, 1549556828u32))?, f1059(v3))
}

pub fn f1049(v0: u64) -> Bytes {
    let a3854 = f614({
        let a3832 = {
            let a3829 = v0;
            let a3830 = 16777216u64;
            let a3831 = 0u64;
            nat_quot(a3829, a3830, a3831)
        };
        let a3833 = 256u64;
        let a3834 = 0u64;
        nat_rem(a3832, a3833, a3834)
    });
    let a3855 = {
        let a3852 = f614({
            let a3838 = {
                let a3835 = v0;
                let a3836 = 65536u64;
                let a3837 = 0u64;
                nat_quot(a3835, a3836, a3837)
            };
            let a3839 = 256u64;
            let a3840 = 0u64;
            nat_rem(a3838, a3839, a3840)
        });
        let a3853 = {
            let a3850 = f614({
                let a3844 = {
                    let a3841 = v0;
                    let a3842 = 256u64;
                    let a3843 = 0u64;
                    nat_quot(a3841, a3842, a3843)
                };
                let a3845 = 256u64;
                let a3846 = 0u64;
                nat_rem(a3844, a3845, a3846)
            });
            let a3851 = f614({
                let a3847 = v0;
                let a3848 = 256u64;
                let a3849 = 0u64;
                nat_rem(a3847, a3848, a3849)
            });
            append_bytes(a3850, a3851)
        };
        append_bytes(a3852, a3853)
    };
    append_bytes(a3854, a3855)
}

pub fn f1050(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3856) = f1034(Fn9::F1060(v0.clone(), v1), {
        let a3857 = v0;
        length_bytes(a3857)
    }, (0u64, 0u64))?;
    Ok(h3856)
}

pub fn f1051(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f1061(f100(v0.clone(), {
        let (h3858, _) = v1.clone();
        h3858
    })?);
    let v3: u64 = f1061(f100(v0, {
        let a3860 = {
            let (h3859, _) = v1.clone();
            h3859
        };
        let a3861 = 1u64;
        nat_add(a3860, a3861)?
    })?);
    let a3862 = v2;
    let a3863 = 16u64;
    if if nat_lt(a3862, a3863) {
        let a3864 = v3;
        let a3865 = 16u64;
        nat_lt(a3864, a3865)
    } else {
        false
    } {
        Ok(({
            let a3867 = {
                let (h3866, _) = v1.clone();
                h3866
            };
            let a3868 = 2u64;
            nat_add(a3867, a3868)?
        }, ({
            let a3875 = {
                let (h3869, _) = {
                    let (_, h3870) = v1.clone();
                    h3870
                };
                h3869
            };
            let a3876 = f614({
                let a3873 = {
                    let a3871 = v2;
                    let a3872 = 16u64;
                    nat_mul(a3871, a3872)?
                };
                let a3874 = v3;
                nat_add(a3873, a3874)?
            });
            append_bytes(a3875, a3876)
        }, {
            let (_, h3877) = {
                let (_, h3878) = v1;
                h3878
            };
            h3877
        })))
    } else {
        Ok(({
            let a3880 = {
                let (h3879, _) = v1.clone();
                h3879
            };
            let a3881 = 2u64;
            nat_add(a3880, a3881)?
        }, ({
            let (h3882, _) = {
                let (_, h3883) = v1.clone();
                h3883
            };
            h3882
        }, {
            let a3886 = {
                let (_, h3884) = {
                    let (_, h3885) = v1;
                    h3885
                };
                h3884
            };
            let a3887 = 1u64;
            nat_add(a3886, a3887)?
        })))
    }
}

pub fn f1052(v0: u64) -> R<Bytes> {
    Ok(f614({
        let a3888 = v0;
        let a3889 = 10u64;
        if nat_lt(a3888, a3889) {
            let a3890 = 48u64;
            let a3891 = v0;
            nat_add(a3890, a3891)?
        } else {
            let a3892 = 87u64;
            let a3893 = v0;
            nat_add(a3892, a3893)?
        }
    }))
}

pub fn f1053(v0: u32) -> R<Bytes> {
    let a3898 = f1062(v0, 24u32)?;
    let a3899 = {
        let a3896 = f1062(v0, 16u32)?;
        let a3897 = {
            let a3894 = f1062(v0, 8u32)?;
            let a3895 = f1062(v0, 0u32)?;
            append_bytes(a3894, a3895)
        };
        append_bytes(a3896, a3897)
    };
    Ok(append_bytes(a3898, a3899))
}

pub fn f1054(v0: Bytes) -> R<Adt166> {
    let a3901 = f1063();
    let a3902 = {
        let a3900 = v0.clone();
        length_bytes(a3900)
    };
    if nat_lt(a3901, a3902) {
        Ok(f1064(f1065(v0)?))
    } else {
        f1066(v0)
    }
}

pub fn f1055(v0: Adt165, v1: u64, v2: Bytes) -> R<Adt165> {
    let v3: u64 = {
        let a3903 = v2.clone();
        length_bytes(a3903)
    };
    let v4: u64 = {
        let a3909 = {
            let a3906 = {
                let a3904 = v3;
                let a3905 = 72u64;
                nat_add(a3904, a3905)?
            };
            let a3907 = 64u64;
            let a3908 = 0u64;
            nat_quot(a3906, a3907, a3908)
        };
        let a3910 = 64u64;
        nat_mul(a3909, a3910)?
    };
    let (_, h3911) = f1067(Fn12::F1068(v2, v1, v3, v4), {
        let a3912 = v4;
        let a3913 = 64u64;
        let a3914 = 0u64;
        nat_quot(a3912, a3913, a3914)
    }, (0u64, v0))?;
    Ok(h3911)
}

pub fn f1056(v0: Adt165, v1: Adt166) -> R<Adt165> {
    let v2: Adt167 = f1069(Fn13::F1070, 64u64, Adt167::C0(match v0.clone() {
        Adt165::C0(h3915, _, _, _, _, _, _, _) => {
            h3915
        }
    }, match v0.clone() {
        Adt165::C0(_, h3916, _, _, _, _, _, _) => {
            h3916
        }
    }, match v0.clone() {
        Adt165::C0(_, _, h3917, _, _, _, _, _) => {
            h3917
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, h3918, _, _, _, _) => {
            h3918
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, h3919, _, _, _) => {
            h3919
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, h3920, _, _) => {
            h3920
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h3921, _) => {
            h3921
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, _, h3922) => {
            h3922
        }
    }, v1, 0u64))?;
    Ok(Adt165::C0(f1071(match v0.clone() {
        Adt165::C0(h3923, _, _, _, _, _, _, _) => {
            h3923
        }
    }, match v2.clone() {
        Adt167::C0(h3924, _, _, _, _, _, _, _, _, _) => {
            h3924
        }
    }), f1071(match v0.clone() {
        Adt165::C0(_, h3925, _, _, _, _, _, _) => {
            h3925
        }
    }, match v2.clone() {
        Adt167::C0(_, h3926, _, _, _, _, _, _, _, _) => {
            h3926
        }
    }), f1071(match v0.clone() {
        Adt165::C0(_, _, h3927, _, _, _, _, _) => {
            h3927
        }
    }, match v2.clone() {
        Adt167::C0(_, _, h3928, _, _, _, _, _, _, _) => {
            h3928
        }
    }), f1071(match v0.clone() {
        Adt165::C0(_, _, _, h3929, _, _, _, _) => {
            h3929
        }
    }, match v2.clone() {
        Adt167::C0(_, _, _, h3930, _, _, _, _, _, _) => {
            h3930
        }
    }), f1071(match v0.clone() {
        Adt165::C0(_, _, _, _, h3931, _, _, _) => {
            h3931
        }
    }, match v2.clone() {
        Adt167::C0(_, _, _, _, h3932, _, _, _, _, _) => {
            h3932
        }
    }), f1071(match v0.clone() {
        Adt165::C0(_, _, _, _, _, h3933, _, _) => {
            h3933
        }
    }, match v2.clone() {
        Adt167::C0(_, _, _, _, _, h3934, _, _, _, _) => {
            h3934
        }
    }), f1071(match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h3935, _) => {
            h3935
        }
    }, match v2.clone() {
        Adt167::C0(_, _, _, _, _, _, h3936, _, _, _) => {
            h3936
        }
    }), f1071(match v0 {
        Adt165::C0(_, _, _, _, _, _, _, h3937) => {
            h3937
        }
    }, match v2 {
        Adt167::C0(_, _, _, _, _, _, _, h3938, _, _) => {
            h3938
        }
    })))
}

pub fn f1057() -> Adt165 {
    Adt165::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f1058(v0: Adt166, v1: u32) -> Adt166 {
    Adt166::C0({
        let a3940 = match v0.clone() {
            Adt166::C0(h3939, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3939
            }
        };
        let a3941 = v1;
        fixed_u32::bit_xor(a3940, a3941)
    }, {
        let a3943 = match v0.clone() {
            Adt166::C0(_, h3942, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3942
            }
        };
        let a3944 = v1;
        fixed_u32::bit_xor(a3943, a3944)
    }, {
        let a3946 = match v0.clone() {
            Adt166::C0(_, _, h3945, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3945
            }
        };
        let a3947 = v1;
        fixed_u32::bit_xor(a3946, a3947)
    }, {
        let a3949 = match v0.clone() {
            Adt166::C0(_, _, _, h3948, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3948
            }
        };
        let a3950 = v1;
        fixed_u32::bit_xor(a3949, a3950)
    }, {
        let a3952 = match v0.clone() {
            Adt166::C0(_, _, _, _, h3951, _, _, _, _, _, _, _, _, _, _, _) => {
                h3951
            }
        };
        let a3953 = v1;
        fixed_u32::bit_xor(a3952, a3953)
    }, {
        let a3955 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, h3954, _, _, _, _, _, _, _, _, _, _) => {
                h3954
            }
        };
        let a3956 = v1;
        fixed_u32::bit_xor(a3955, a3956)
    }, {
        let a3958 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, h3957, _, _, _, _, _, _, _, _, _) => {
                h3957
            }
        };
        let a3959 = v1;
        fixed_u32::bit_xor(a3958, a3959)
    }, {
        let a3961 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, h3960, _, _, _, _, _, _, _, _) => {
                h3960
            }
        };
        let a3962 = v1;
        fixed_u32::bit_xor(a3961, a3962)
    }, {
        let a3964 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, h3963, _, _, _, _, _, _, _) => {
                h3963
            }
        };
        let a3965 = v1;
        fixed_u32::bit_xor(a3964, a3965)
    }, {
        let a3967 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, h3966, _, _, _, _, _, _) => {
                h3966
            }
        };
        let a3968 = v1;
        fixed_u32::bit_xor(a3967, a3968)
    }, {
        let a3970 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, h3969, _, _, _, _, _) => {
                h3969
            }
        };
        let a3971 = v1;
        fixed_u32::bit_xor(a3970, a3971)
    }, {
        let a3973 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, h3972, _, _, _, _) => {
                h3972
            }
        };
        let a3974 = v1;
        fixed_u32::bit_xor(a3973, a3974)
    }, {
        let a3976 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3975, _, _, _) => {
                h3975
            }
        };
        let a3977 = v1;
        fixed_u32::bit_xor(a3976, a3977)
    }, {
        let a3979 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3978, _, _) => {
                h3978
            }
        };
        let a3980 = v1;
        fixed_u32::bit_xor(a3979, a3980)
    }, {
        let a3982 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3981, _) => {
                h3981
            }
        };
        let a3983 = v1;
        fixed_u32::bit_xor(a3982, a3983)
    }, {
        let a3985 = match v0 {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3984) => {
                h3984
            }
        };
        let a3986 = v1;
        fixed_u32::bit_xor(a3985, a3986)
    })
}

pub fn f1059(v0: Adt165) -> Adt166 {
    Adt166::C0(match v0.clone() {
        Adt165::C0(h3987, _, _, _, _, _, _, _) => {
            h3987
        }
    }, match v0.clone() {
        Adt165::C0(_, h3988, _, _, _, _, _, _) => {
            h3988
        }
    }, match v0.clone() {
        Adt165::C0(_, _, h3989, _, _, _, _, _) => {
            h3989
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, h3990, _, _, _, _) => {
            h3990
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, h3991, _, _, _) => {
            h3991
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, h3992, _, _) => {
            h3992
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h3993, _) => {
            h3993
        }
    }, match v0 {
        Adt165::C0(_, _, _, _, _, _, _, h3994) => {
            h3994
        }
    }, 2147483648u32, f1072(), f1072(), f1072(), f1072(), f1072(), f1072(), 768u32)
}

pub fn f1060(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3996 = {
            let (h3995, _) = v2;
            h3995
        };
        let a3997 = 1u64;
        nat_add(a3996, a3997)?
    }, {
        let a4049 = {
            let (_, h3998) = v2;
            h3998
        };
        let a4050 = {
            let v3: u8 = {
                let a4001 = f1073(v0, {
                    let (h3999, _) = v2;
                    h3999
                });
                let a4002 = f1073(v1, {
                    let (h4000, _) = v2;
                    h4000
                });
                fixed_u8::bit_xor(a4001, a4002)
            };
            let a4047 = {
                let a4023 = {
                    let a4011 = {
                        let a4005 = {
                            let a4003 = v3;
                            let a4004 = 128u8;
                            fixed_u8::bit_and(a4003, a4004)
                        };
                        let a4006 = 128u8;
                        if equal(a4005, a4006) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a4012 = {
                        let a4009 = {
                            let a4007 = v3;
                            let a4008 = 64u8;
                            fixed_u8::bit_and(a4007, a4008)
                        };
                        let a4010 = 64u8;
                        if equal(a4009, a4010) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4011, a4012)?
                };
                let a4024 = {
                    let a4021 = {
                        let a4015 = {
                            let a4013 = v3;
                            let a4014 = 32u8;
                            fixed_u8::bit_and(a4013, a4014)
                        };
                        let a4016 = 32u8;
                        if equal(a4015, a4016) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a4022 = {
                        let a4019 = {
                            let a4017 = v3;
                            let a4018 = 16u8;
                            fixed_u8::bit_and(a4017, a4018)
                        };
                        let a4020 = 16u8;
                        if equal(a4019, a4020) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4021, a4022)?
                };
                nat_add(a4023, a4024)?
            };
            let a4048 = {
                let a4045 = {
                    let a4033 = {
                        let a4027 = {
                            let a4025 = v3;
                            let a4026 = 8u8;
                            fixed_u8::bit_and(a4025, a4026)
                        };
                        let a4028 = 8u8;
                        if equal(a4027, a4028) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a4034 = {
                        let a4031 = {
                            let a4029 = v3;
                            let a4030 = 4u8;
                            fixed_u8::bit_and(a4029, a4030)
                        };
                        let a4032 = 4u8;
                        if equal(a4031, a4032) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4033, a4034)?
                };
                let a4046 = {
                    let a4043 = {
                        let a4037 = {
                            let a4035 = v3;
                            let a4036 = 2u8;
                            fixed_u8::bit_and(a4035, a4036)
                        };
                        let a4038 = 2u8;
                        if equal(a4037, a4038) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a4044 = {
                        let a4041 = {
                            let a4039 = v3;
                            let a4040 = 1u8;
                            fixed_u8::bit_and(a4039, a4040)
                        };
                        let a4042 = 1u8;
                        if equal(a4041, a4042) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4043, a4044)?
                };
                nat_add(a4045, a4046)?
            };
            nat_add(a4047, a4048)?
        };
        nat_add(a4049, a4050)?
    }))
}

pub fn f1061(v0: u64) -> u64 {
    let a4051 = v0;
    let a4052 = 48u64;
    if nat_lt(a4051, a4052) {
        16u64
    } else {
        let a4053 = v0;
        let a4054 = 58u64;
        if nat_lt(a4053, a4054) {
            let a4055 = v0;
            let a4056 = 48u64;
            nat_sub(a4055, a4056)
        } else {
            let a4057 = v0;
            let a4058 = 97u64;
            if nat_lt(a4057, a4058) {
                16u64
            } else {
                let a4059 = v0;
                let a4060 = 103u64;
                if nat_lt(a4059, a4060) {
                    let a4061 = v0;
                    let a4062 = 87u64;
                    nat_sub(a4061, a4062)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f1062(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f614({
        let v2: u32 = {
            let a4063 = f1074(v0, v1);
            let a4064 = 255u32;
            fixed_u32::bit_and(a4063, a4064)
        };
        let a4253 = {
            let a4157 = {
                let a4109 = {
                    let a4085 = {
                        let a4073 = {
                            let a4067 = {
                                let a4065 = v2;
                                let a4066 = 2147483648u32;
                                fixed_u32::bit_and(a4065, a4066)
                            };
                            let a4068 = 2147483648u32;
                            if equal(a4067, a4068) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a4074 = {
                            let a4071 = {
                                let a4069 = v2;
                                let a4070 = 1073741824u32;
                                fixed_u32::bit_and(a4069, a4070)
                            };
                            let a4072 = 1073741824u32;
                            if equal(a4071, a4072) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4073, a4074)?
                    };
                    let a4086 = {
                        let a4083 = {
                            let a4077 = {
                                let a4075 = v2;
                                let a4076 = 536870912u32;
                                fixed_u32::bit_and(a4075, a4076)
                            };
                            let a4078 = 536870912u32;
                            if equal(a4077, a4078) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a4084 = {
                            let a4081 = {
                                let a4079 = v2;
                                let a4080 = 268435456u32;
                                fixed_u32::bit_and(a4079, a4080)
                            };
                            let a4082 = 268435456u32;
                            if equal(a4081, a4082) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4083, a4084)?
                    };
                    nat_add(a4085, a4086)?
                };
                let a4110 = {
                    let a4107 = {
                        let a4095 = {
                            let a4089 = {
                                let a4087 = v2;
                                let a4088 = 134217728u32;
                                fixed_u32::bit_and(a4087, a4088)
                            };
                            let a4090 = 134217728u32;
                            if equal(a4089, a4090) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a4096 = {
                            let a4093 = {
                                let a4091 = v2;
                                let a4092 = 67108864u32;
                                fixed_u32::bit_and(a4091, a4092)
                            };
                            let a4094 = 67108864u32;
                            if equal(a4093, a4094) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4095, a4096)?
                    };
                    let a4108 = {
                        let a4105 = {
                            let a4099 = {
                                let a4097 = v2;
                                let a4098 = 33554432u32;
                                fixed_u32::bit_and(a4097, a4098)
                            };
                            let a4100 = 33554432u32;
                            if equal(a4099, a4100) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a4106 = {
                            let a4103 = {
                                let a4101 = v2;
                                let a4102 = 16777216u32;
                                fixed_u32::bit_and(a4101, a4102)
                            };
                            let a4104 = 16777216u32;
                            if equal(a4103, a4104) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4105, a4106)?
                    };
                    nat_add(a4107, a4108)?
                };
                nat_add(a4109, a4110)?
            };
            let a4158 = {
                let a4155 = {
                    let a4131 = {
                        let a4119 = {
                            let a4113 = {
                                let a4111 = v2;
                                let a4112 = 8388608u32;
                                fixed_u32::bit_and(a4111, a4112)
                            };
                            let a4114 = 8388608u32;
                            if equal(a4113, a4114) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a4120 = {
                            let a4117 = {
                                let a4115 = v2;
                                let a4116 = 4194304u32;
                                fixed_u32::bit_and(a4115, a4116)
                            };
                            let a4118 = 4194304u32;
                            if equal(a4117, a4118) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4119, a4120)?
                    };
                    let a4132 = {
                        let a4129 = {
                            let a4123 = {
                                let a4121 = v2;
                                let a4122 = 2097152u32;
                                fixed_u32::bit_and(a4121, a4122)
                            };
                            let a4124 = 2097152u32;
                            if equal(a4123, a4124) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a4130 = {
                            let a4127 = {
                                let a4125 = v2;
                                let a4126 = 1048576u32;
                                fixed_u32::bit_and(a4125, a4126)
                            };
                            let a4128 = 1048576u32;
                            if equal(a4127, a4128) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4129, a4130)?
                    };
                    nat_add(a4131, a4132)?
                };
                let a4156 = {
                    let a4153 = {
                        let a4141 = {
                            let a4135 = {
                                let a4133 = v2;
                                let a4134 = 524288u32;
                                fixed_u32::bit_and(a4133, a4134)
                            };
                            let a4136 = 524288u32;
                            if equal(a4135, a4136) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a4142 = {
                            let a4139 = {
                                let a4137 = v2;
                                let a4138 = 262144u32;
                                fixed_u32::bit_and(a4137, a4138)
                            };
                            let a4140 = 262144u32;
                            if equal(a4139, a4140) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4141, a4142)?
                    };
                    let a4154 = {
                        let a4151 = {
                            let a4145 = {
                                let a4143 = v2;
                                let a4144 = 131072u32;
                                fixed_u32::bit_and(a4143, a4144)
                            };
                            let a4146 = 131072u32;
                            if equal(a4145, a4146) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a4152 = {
                            let a4149 = {
                                let a4147 = v2;
                                let a4148 = 65536u32;
                                fixed_u32::bit_and(a4147, a4148)
                            };
                            let a4150 = 65536u32;
                            if equal(a4149, a4150) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4151, a4152)?
                    };
                    nat_add(a4153, a4154)?
                };
                nat_add(a4155, a4156)?
            };
            nat_add(a4157, a4158)?
        };
        let a4254 = {
            let a4251 = {
                let a4203 = {
                    let a4179 = {
                        let a4167 = {
                            let a4161 = {
                                let a4159 = v2;
                                let a4160 = 32768u32;
                                fixed_u32::bit_and(a4159, a4160)
                            };
                            let a4162 = 32768u32;
                            if equal(a4161, a4162) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a4168 = {
                            let a4165 = {
                                let a4163 = v2;
                                let a4164 = 16384u32;
                                fixed_u32::bit_and(a4163, a4164)
                            };
                            let a4166 = 16384u32;
                            if equal(a4165, a4166) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4167, a4168)?
                    };
                    let a4180 = {
                        let a4177 = {
                            let a4171 = {
                                let a4169 = v2;
                                let a4170 = 8192u32;
                                fixed_u32::bit_and(a4169, a4170)
                            };
                            let a4172 = 8192u32;
                            if equal(a4171, a4172) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a4178 = {
                            let a4175 = {
                                let a4173 = v2;
                                let a4174 = 4096u32;
                                fixed_u32::bit_and(a4173, a4174)
                            };
                            let a4176 = 4096u32;
                            if equal(a4175, a4176) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4177, a4178)?
                    };
                    nat_add(a4179, a4180)?
                };
                let a4204 = {
                    let a4201 = {
                        let a4189 = {
                            let a4183 = {
                                let a4181 = v2;
                                let a4182 = 2048u32;
                                fixed_u32::bit_and(a4181, a4182)
                            };
                            let a4184 = 2048u32;
                            if equal(a4183, a4184) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a4190 = {
                            let a4187 = {
                                let a4185 = v2;
                                let a4186 = 1024u32;
                                fixed_u32::bit_and(a4185, a4186)
                            };
                            let a4188 = 1024u32;
                            if equal(a4187, a4188) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4189, a4190)?
                    };
                    let a4202 = {
                        let a4199 = {
                            let a4193 = {
                                let a4191 = v2;
                                let a4192 = 512u32;
                                fixed_u32::bit_and(a4191, a4192)
                            };
                            let a4194 = 512u32;
                            if equal(a4193, a4194) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a4200 = {
                            let a4197 = {
                                let a4195 = v2;
                                let a4196 = 256u32;
                                fixed_u32::bit_and(a4195, a4196)
                            };
                            let a4198 = 256u32;
                            if equal(a4197, a4198) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4199, a4200)?
                    };
                    nat_add(a4201, a4202)?
                };
                nat_add(a4203, a4204)?
            };
            let a4252 = {
                let a4249 = {
                    let a4225 = {
                        let a4213 = {
                            let a4207 = {
                                let a4205 = v2;
                                let a4206 = 128u32;
                                fixed_u32::bit_and(a4205, a4206)
                            };
                            let a4208 = 128u32;
                            if equal(a4207, a4208) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4214 = {
                            let a4211 = {
                                let a4209 = v2;
                                let a4210 = 64u32;
                                fixed_u32::bit_and(a4209, a4210)
                            };
                            let a4212 = 64u32;
                            if equal(a4211, a4212) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4213, a4214)?
                    };
                    let a4226 = {
                        let a4223 = {
                            let a4217 = {
                                let a4215 = v2;
                                let a4216 = 32u32;
                                fixed_u32::bit_and(a4215, a4216)
                            };
                            let a4218 = 32u32;
                            if equal(a4217, a4218) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4224 = {
                            let a4221 = {
                                let a4219 = v2;
                                let a4220 = 16u32;
                                fixed_u32::bit_and(a4219, a4220)
                            };
                            let a4222 = 16u32;
                            if equal(a4221, a4222) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4223, a4224)?
                    };
                    nat_add(a4225, a4226)?
                };
                let a4250 = {
                    let a4247 = {
                        let a4235 = {
                            let a4229 = {
                                let a4227 = v2;
                                let a4228 = 8u32;
                                fixed_u32::bit_and(a4227, a4228)
                            };
                            let a4230 = 8u32;
                            if equal(a4229, a4230) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4236 = {
                            let a4233 = {
                                let a4231 = v2;
                                let a4232 = 4u32;
                                fixed_u32::bit_and(a4231, a4232)
                            };
                            let a4234 = 4u32;
                            if equal(a4233, a4234) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4235, a4236)?
                    };
                    let a4248 = {
                        let a4245 = {
                            let a4239 = {
                                let a4237 = v2;
                                let a4238 = 2u32;
                                fixed_u32::bit_and(a4237, a4238)
                            };
                            let a4240 = 2u32;
                            if equal(a4239, a4240) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4246 = {
                            let a4243 = {
                                let a4241 = v2;
                                let a4242 = 1u32;
                                fixed_u32::bit_and(a4241, a4242)
                            };
                            let a4244 = 1u32;
                            if equal(a4243, a4244) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4245, a4246)?
                    };
                    nat_add(a4247, a4248)?
                };
                nat_add(a4249, a4250)?
            };
            nat_add(a4251, a4252)?
        };
        nat_add(a4253, a4254)?
    }))
}

pub fn f1063() -> u64 {
    64u64
}

pub fn f1064(v0: Adt165) -> Adt166 {
    Adt166::C0(match v0.clone() {
        Adt165::C0(h4255, _, _, _, _, _, _, _) => {
            h4255
        }
    }, match v0.clone() {
        Adt165::C0(_, h4256, _, _, _, _, _, _) => {
            h4256
        }
    }, match v0.clone() {
        Adt165::C0(_, _, h4257, _, _, _, _, _) => {
            h4257
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, h4258, _, _, _, _) => {
            h4258
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, h4259, _, _, _) => {
            h4259
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, h4260, _, _) => {
            h4260
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h4261, _) => {
            h4261
        }
    }, match v0 {
        Adt165::C0(_, _, _, _, _, _, _, h4262) => {
            h4262
        }
    }, f1072(), f1072(), f1072(), f1072(), f1072(), f1072(), f1072(), f1072())
}

pub fn f1065(v0: Bytes) -> R<Adt165> {
    let v1: u64 = {
        let a4263 = v0.clone();
        length_bytes(a4263)
    };
    let v2: u64 = {
        let a4269 = {
            let a4266 = {
                let a4264 = v1;
                let a4265 = 72u64;
                nat_add(a4264, a4265)?
            };
            let a4267 = 64u64;
            let a4268 = 0u64;
            nat_quot(a4266, a4267, a4268)
        };
        let a4270 = 64u64;
        nat_mul(a4269, a4270)?
    };
    let (_, h4271) = f1067(Fn12::F1075(v0, v1, v2), {
        let a4272 = v2;
        let a4273 = 64u64;
        let a4274 = 0u64;
        nat_quot(a4272, a4273, a4274)
    }, (0u64, f1057()))?;
    Ok(h4271)
}

pub fn f1066(v0: Bytes) -> R<Adt166> {
    Ok(Adt166::C0(f1076(v0.clone(), 0u64)?, f1076(v0.clone(), 4u64)?, f1076(v0.clone(), 8u64)?, f1076(v0.clone(), 12u64)?, f1076(v0.clone(), 16u64)?, f1076(v0.clone(), 20u64)?, f1076(v0.clone(), 24u64)?, f1076(v0.clone(), 28u64)?, f1076(v0.clone(), 32u64)?, f1076(v0.clone(), 36u64)?, f1076(v0.clone(), 40u64)?, f1076(v0.clone(), 44u64)?, f1076(v0.clone(), 48u64)?, f1076(v0.clone(), 52u64)?, f1076(v0.clone(), 56u64)?, f1076(v0, 60u64)?))
}

pub fn f1067(v0: Fn12, p4278: u64, p4279: (u64, Adt165)) -> R<(u64, Adt165)> {
    let m4277 = (p4278, p4279);
    {
        let mut m4277 = m4277;
        loop {
            let step = {
                let (v1, v2) = m4277;
                let m4275 = v1;
                if m4275 == 0 {
                    Err::<(u64, (u64, Adt165)), R<(u64, Adt165)>>(Ok(v2))
                } else {
                    let v3 = m4275 - 1;
                    Ok::<(u64, (u64, Adt165)), R<(u64, Adt165)>>((v3, {
                        let c4276: Fn12 = v0.clone();
                        c4276.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1068(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt165)) -> R<(u64, Adt165)> {
    Ok(({
        let a4281 = {
            let (h4280, _) = v4.clone();
            h4280
        };
        let a4282 = 64u64;
        nat_add(a4281, a4282)?
    }, f1056({
        let (_, h4283) = v4;
        h4283
    }, f1077(v0, v2, v3, {
        let a4286 = {
            let a4284 = v1;
            let a4285 = v2;
            nat_add(a4284, a4285)?
        };
        let a4287 = 8u64;
        nat_mul(a4286, a4287)?
    }, {
        let (h4288, _) = v4;
        h4288
    })?)?))
}

pub fn f1069(v0: Fn13, p4292: u64, p4293: Adt167) -> R<Adt167> {
    let m4291 = (p4292, p4293);
    {
        let mut m4291 = m4291;
        loop {
            let step = {
                let (v1, v2) = m4291;
                let m4289 = v1;
                if m4289 == 0 {
                    Err::<(u64, Adt167), R<Adt167>>(Ok(v2))
                } else {
                    let v3 = m4289 - 1;
                    Ok::<(u64, Adt167), R<Adt167>>((v3, {
                        let c4290: Fn13 = v0.clone();
                        c4290.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1070(v0: Adt167) -> R<Adt167> {
    f1078(v0)
}

pub fn f1071(v0: u32, v1: u32) -> u32 {
    let m4296 = {
        let a4294 = f1079(v0);
        let a4295 = f1079(v1);
        fixed_u64::checked_add(a4294, a4295)
    };
    match m4296 {
        None => {
            0u32
        }
        Some(v2) => {
            f1080(v2)
        }
    }
}

pub fn f1072() -> u32 {
    0u32
}

pub fn f1073(v0: Bytes, v1: u64) -> u8 {
    let m4299 = {
        let a4297 = v0;
        let a4298 = v1;
        index_bytes(a4297, a4298)
    };
    match m4299 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1074(v0: u32, v1: u32) -> u32 {
    let m4302 = {
        let a4300 = v0;
        let a4301 = v1;
        fixed_u32::shift_right(a4300, a4301)
    };
    match m4302 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1075(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt165)) -> R<(u64, Adt165)> {
    Ok(({
        let a4304 = {
            let (h4303, _) = v3.clone();
            h4303
        };
        let a4305 = 64u64;
        nat_add(a4304, a4305)?
    }, f1056({
        let (_, h4306) = v3;
        h4306
    }, f1081(v0, v1, v2, {
        let (h4307, _) = v3;
        h4307
    })?)?))
}

pub fn f1076(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f1082(f1083(v0.clone(), v1), f1083(v0.clone(), {
        let a4308 = v1;
        let a4309 = 1u64;
        nat_add(a4308, a4309)?
    }), f1083(v0.clone(), {
        let a4310 = v1;
        let a4311 = 2u64;
        nat_add(a4310, a4311)?
    }), f1083(v0, {
        let a4312 = v1;
        let a4313 = 3u64;
        nat_add(a4312, a4313)?
    })))
}

pub fn f1077(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt166> {
    Ok(Adt166::C0(f1084(v0.clone(), v1, v2, v3, v4)?, f1084(v0.clone(), v1, v2, v3, {
        let a4314 = v4;
        let a4315 = 4u64;
        nat_add(a4314, a4315)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4316 = v4;
        let a4317 = 8u64;
        nat_add(a4316, a4317)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4318 = v4;
        let a4319 = 12u64;
        nat_add(a4318, a4319)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4320 = v4;
        let a4321 = 16u64;
        nat_add(a4320, a4321)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4322 = v4;
        let a4323 = 20u64;
        nat_add(a4322, a4323)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4324 = v4;
        let a4325 = 24u64;
        nat_add(a4324, a4325)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4326 = v4;
        let a4327 = 28u64;
        nat_add(a4326, a4327)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4328 = v4;
        let a4329 = 32u64;
        nat_add(a4328, a4329)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4330 = v4;
        let a4331 = 36u64;
        nat_add(a4330, a4331)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4332 = v4;
        let a4333 = 40u64;
        nat_add(a4332, a4333)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4334 = v4;
        let a4335 = 44u64;
        nat_add(a4334, a4335)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4336 = v4;
        let a4337 = 48u64;
        nat_add(a4336, a4337)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4338 = v4;
        let a4339 = 52u64;
        nat_add(a4338, a4339)?
    })?, f1084(v0.clone(), v1, v2, v3, {
        let a4340 = v4;
        let a4341 = 56u64;
        nat_add(a4340, a4341)?
    })?, f1084(v0, v1, v2, v3, {
        let a4342 = v4;
        let a4343 = 60u64;
        nat_add(a4342, a4343)?
    })?))
}

pub fn f1078(v0: Adt167) -> R<Adt167> {
    let v1: u32 = f1085(match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, _, _, _, h4344) => {
            h4344
        }
    });
    let v2: u32 = f1071(match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, _, h4345, _, _) => {
            h4345
        }
    }, f1071(f1086(match v0.clone() {
        Adt167::C0(_, _, _, _, h4346, _, _, _, _, _) => {
            h4346
        }
    }), f1071(f1087(match v0.clone() {
        Adt167::C0(_, _, _, _, h4347, _, _, _, _, _) => {
            h4347
        }
    }, match v0.clone() {
        Adt167::C0(_, _, _, _, _, h4348, _, _, _, _) => {
            h4348
        }
    }, match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, h4349, _, _, _) => {
            h4349
        }
    }), f1071(v1, {
        let m4352 = match v0.clone() {
            Adt167::C0(_, _, _, _, _, _, _, _, h4351, _) => {
                h4351
            }
        };
        match m4352 {
            Adt166::C0(h4350, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4350
            }
        }
    }))));
    let v3: u32 = f1071(f1088(match v0.clone() {
        Adt167::C0(h4353, _, _, _, _, _, _, _, _, _) => {
            h4353
        }
    }), f1089(match v0.clone() {
        Adt167::C0(h4354, _, _, _, _, _, _, _, _, _) => {
            h4354
        }
    }, match v0.clone() {
        Adt167::C0(_, h4355, _, _, _, _, _, _, _, _) => {
            h4355
        }
    }, match v0.clone() {
        Adt167::C0(_, _, h4356, _, _, _, _, _, _, _) => {
            h4356
        }
    }));
    Ok(Adt167::C0(f1071(v2, v3), match v0.clone() {
        Adt167::C0(h4357, _, _, _, _, _, _, _, _, _) => {
            h4357
        }
    }, match v0.clone() {
        Adt167::C0(_, h4358, _, _, _, _, _, _, _, _) => {
            h4358
        }
    }, match v0.clone() {
        Adt167::C0(_, _, h4359, _, _, _, _, _, _, _) => {
            h4359
        }
    }, f1071(match v0.clone() {
        Adt167::C0(_, _, _, h4360, _, _, _, _, _, _) => {
            h4360
        }
    }, v2), match v0.clone() {
        Adt167::C0(_, _, _, _, h4361, _, _, _, _, _) => {
            h4361
        }
    }, match v0.clone() {
        Adt167::C0(_, _, _, _, _, h4362, _, _, _, _) => {
            h4362
        }
    }, match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, h4363, _, _, _) => {
            h4363
        }
    }, f1090(match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, _, _, h4364, _) => {
            h4364
        }
    }), {
        let a4366 = match v0 {
            Adt167::C0(_, _, _, _, _, _, _, _, _, h4365) => {
                h4365
            }
        };
        let a4367 = 1u64;
        nat_add(a4366, a4367)?
    }))
}

pub fn f1079(v0: u32) -> u64 {
    let m4369 = {
        let a4368 = v0;
        fixed_u64::convert(i128::from(a4368))
    };
    match m4369 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1080(v0: u64) -> u32 {
    let m4373 = {
        let a4372 = {
            let a4370 = v0;
            let a4371 = 4294967295u64;
            fixed_u64::bit_and(a4370, a4371)
        };
        fixed_u32::convert(i128::from(a4372))
    };
    match m4373 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1081(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt166> {
    Ok(Adt166::C0(f1091(v0.clone(), v1, v2, v3)?, f1091(v0.clone(), v1, v2, {
        let a4374 = v3;
        let a4375 = 4u64;
        nat_add(a4374, a4375)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4376 = v3;
        let a4377 = 8u64;
        nat_add(a4376, a4377)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4378 = v3;
        let a4379 = 12u64;
        nat_add(a4378, a4379)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4380 = v3;
        let a4381 = 16u64;
        nat_add(a4380, a4381)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4382 = v3;
        let a4383 = 20u64;
        nat_add(a4382, a4383)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4384 = v3;
        let a4385 = 24u64;
        nat_add(a4384, a4385)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4386 = v3;
        let a4387 = 28u64;
        nat_add(a4386, a4387)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4388 = v3;
        let a4389 = 32u64;
        nat_add(a4388, a4389)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4390 = v3;
        let a4391 = 36u64;
        nat_add(a4390, a4391)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4392 = v3;
        let a4393 = 40u64;
        nat_add(a4392, a4393)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4394 = v3;
        let a4395 = 44u64;
        nat_add(a4394, a4395)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4396 = v3;
        let a4397 = 48u64;
        nat_add(a4396, a4397)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4398 = v3;
        let a4399 = 52u64;
        nat_add(a4398, a4399)?
    })?, f1091(v0.clone(), v1, v2, {
        let a4400 = v3;
        let a4401 = 56u64;
        nat_add(a4400, a4401)?
    })?, f1091(v0, v1, v2, {
        let a4402 = v3;
        let a4403 = 60u64;
        nat_add(a4402, a4403)?
    })?))
}

pub fn f1082(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a4408 = f1092(v0, 24u32);
    let a4409 = {
        let a4406 = f1092(v1, 16u32);
        let a4407 = {
            let a4404 = f1092(v2, 8u32);
            let a4405 = v3;
            fixed_u32::bit_or(a4404, a4405)
        };
        fixed_u32::bit_or(a4406, a4407)
    };
    fixed_u32::bit_or(a4408, a4409)
}

pub fn f1083(v0: Bytes, v1: u64) -> u32 {
    let m4412 = {
        let a4410 = v0;
        let a4411 = v1;
        index_bytes(a4410, a4411)
    };
    match m4412 {
        None => {
            0u32
        }
        Some(v2) => {
            let m4414 = {
                let a4413 = v2;
                fixed_u32::convert(i128::from(a4413))
            };
            match m4414 {
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

pub fn f1084(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a4417 = {
        let a4415 = v4;
        let a4416 = 8u64;
        nat_add(a4415, a4416)?
    };
    let a4418 = v2;
    if nat_eq(a4417, a4418) {
        Ok(f1093(v3, true))
    } else {
        let a4421 = {
            let a4419 = v4;
            let a4420 = 4u64;
            nat_add(a4419, a4420)?
        };
        let a4422 = v2;
        if nat_eq(a4421, a4422) {
            Ok(f1093(v3, false))
        } else {
            Ok(f1082(f1094(v0.clone(), v1, v4), f1094(v0.clone(), v1, {
                let a4423 = v4;
                let a4424 = 1u64;
                nat_add(a4423, a4424)?
            }), f1094(v0.clone(), v1, {
                let a4425 = v4;
                let a4426 = 2u64;
                nat_add(a4425, a4426)?
            }), f1094(v0, v1, {
                let a4427 = v4;
                let a4428 = 3u64;
                nat_add(a4427, a4428)?
            })))
        }
    }
}

pub fn f1085(v0: u64) -> u32 {
    let a4429 = v0;
    let a4430 = 32u64;
    if nat_lt(a4429, a4430) {
        let a4431 = v0;
        let a4432 = 16u64;
        if nat_lt(a4431, a4432) {
            let a4433 = v0;
            let a4434 = 8u64;
            if nat_lt(a4433, a4434) {
                let a4435 = v0;
                let a4436 = 4u64;
                if nat_lt(a4435, a4436) {
                    let a4437 = v0;
                    let a4438 = 2u64;
                    if nat_lt(a4437, a4438) {
                        let a4439 = v0;
                        let a4440 = 1u64;
                        if nat_lt(a4439, a4440) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4441 = v0;
                        let a4442 = 3u64;
                        if nat_lt(a4441, a4442) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4443 = v0;
                    let a4444 = 6u64;
                    if nat_lt(a4443, a4444) {
                        let a4445 = v0;
                        let a4446 = 5u64;
                        if nat_lt(a4445, a4446) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4447 = v0;
                        let a4448 = 7u64;
                        if nat_lt(a4447, a4448) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4449 = v0;
                let a4450 = 12u64;
                if nat_lt(a4449, a4450) {
                    let a4451 = v0;
                    let a4452 = 10u64;
                    if nat_lt(a4451, a4452) {
                        let a4453 = v0;
                        let a4454 = 9u64;
                        if nat_lt(a4453, a4454) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4455 = v0;
                        let a4456 = 11u64;
                        if nat_lt(a4455, a4456) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4457 = v0;
                    let a4458 = 14u64;
                    if nat_lt(a4457, a4458) {
                        let a4459 = v0;
                        let a4460 = 13u64;
                        if nat_lt(a4459, a4460) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4461 = v0;
                        let a4462 = 15u64;
                        if nat_lt(a4461, a4462) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4463 = v0;
            let a4464 = 24u64;
            if nat_lt(a4463, a4464) {
                let a4465 = v0;
                let a4466 = 20u64;
                if nat_lt(a4465, a4466) {
                    let a4467 = v0;
                    let a4468 = 18u64;
                    if nat_lt(a4467, a4468) {
                        let a4469 = v0;
                        let a4470 = 17u64;
                        if nat_lt(a4469, a4470) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4471 = v0;
                        let a4472 = 19u64;
                        if nat_lt(a4471, a4472) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4473 = v0;
                    let a4474 = 22u64;
                    if nat_lt(a4473, a4474) {
                        let a4475 = v0;
                        let a4476 = 21u64;
                        if nat_lt(a4475, a4476) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4477 = v0;
                        let a4478 = 23u64;
                        if nat_lt(a4477, a4478) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4479 = v0;
                let a4480 = 28u64;
                if nat_lt(a4479, a4480) {
                    let a4481 = v0;
                    let a4482 = 26u64;
                    if nat_lt(a4481, a4482) {
                        let a4483 = v0;
                        let a4484 = 25u64;
                        if nat_lt(a4483, a4484) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4485 = v0;
                        let a4486 = 27u64;
                        if nat_lt(a4485, a4486) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4487 = v0;
                    let a4488 = 30u64;
                    if nat_lt(a4487, a4488) {
                        let a4489 = v0;
                        let a4490 = 29u64;
                        if nat_lt(a4489, a4490) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4491 = v0;
                        let a4492 = 31u64;
                        if nat_lt(a4491, a4492) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4493 = v0;
        let a4494 = 48u64;
        if nat_lt(a4493, a4494) {
            let a4495 = v0;
            let a4496 = 40u64;
            if nat_lt(a4495, a4496) {
                let a4497 = v0;
                let a4498 = 36u64;
                if nat_lt(a4497, a4498) {
                    let a4499 = v0;
                    let a4500 = 34u64;
                    if nat_lt(a4499, a4500) {
                        let a4501 = v0;
                        let a4502 = 33u64;
                        if nat_lt(a4501, a4502) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4503 = v0;
                        let a4504 = 35u64;
                        if nat_lt(a4503, a4504) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4505 = v0;
                    let a4506 = 38u64;
                    if nat_lt(a4505, a4506) {
                        let a4507 = v0;
                        let a4508 = 37u64;
                        if nat_lt(a4507, a4508) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4509 = v0;
                        let a4510 = 39u64;
                        if nat_lt(a4509, a4510) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4511 = v0;
                let a4512 = 44u64;
                if nat_lt(a4511, a4512) {
                    let a4513 = v0;
                    let a4514 = 42u64;
                    if nat_lt(a4513, a4514) {
                        let a4515 = v0;
                        let a4516 = 41u64;
                        if nat_lt(a4515, a4516) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4517 = v0;
                        let a4518 = 43u64;
                        if nat_lt(a4517, a4518) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4519 = v0;
                    let a4520 = 46u64;
                    if nat_lt(a4519, a4520) {
                        let a4521 = v0;
                        let a4522 = 45u64;
                        if nat_lt(a4521, a4522) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4523 = v0;
                        let a4524 = 47u64;
                        if nat_lt(a4523, a4524) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4525 = v0;
            let a4526 = 56u64;
            if nat_lt(a4525, a4526) {
                let a4527 = v0;
                let a4528 = 52u64;
                if nat_lt(a4527, a4528) {
                    let a4529 = v0;
                    let a4530 = 50u64;
                    if nat_lt(a4529, a4530) {
                        let a4531 = v0;
                        let a4532 = 49u64;
                        if nat_lt(a4531, a4532) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4533 = v0;
                        let a4534 = 51u64;
                        if nat_lt(a4533, a4534) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4535 = v0;
                    let a4536 = 54u64;
                    if nat_lt(a4535, a4536) {
                        let a4537 = v0;
                        let a4538 = 53u64;
                        if nat_lt(a4537, a4538) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4539 = v0;
                        let a4540 = 55u64;
                        if nat_lt(a4539, a4540) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4541 = v0;
                let a4542 = 60u64;
                if nat_lt(a4541, a4542) {
                    let a4543 = v0;
                    let a4544 = 58u64;
                    if nat_lt(a4543, a4544) {
                        let a4545 = v0;
                        let a4546 = 57u64;
                        if nat_lt(a4545, a4546) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4547 = v0;
                        let a4548 = 59u64;
                        if nat_lt(a4547, a4548) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4549 = v0;
                    let a4550 = 62u64;
                    if nat_lt(a4549, a4550) {
                        let a4551 = v0;
                        let a4552 = 61u64;
                        if nat_lt(a4551, a4552) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4553 = v0;
                        let a4554 = 63u64;
                        if nat_lt(a4553, a4554) {
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

pub fn f1086(v0: u32) -> u32 {
    let a4557 = {
        let a4555 = f1095(v0, 6u32, 26u32);
        let a4556 = f1095(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4555, a4556)
    };
    let a4558 = f1095(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4557, a4558)
}

pub fn f1087(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4564 = {
        let a4559 = v0;
        let a4560 = v1;
        fixed_u32::bit_and(a4559, a4560)
    };
    let a4565 = {
        let a4562 = {
            let a4561 = v0;
            fixed_u32::bit_not(a4561)
        };
        let a4563 = v2;
        fixed_u32::bit_and(a4562, a4563)
    };
    fixed_u32::bit_xor(a4564, a4565)
}

pub fn f1088(v0: u32) -> u32 {
    let a4568 = {
        let a4566 = f1095(v0, 2u32, 30u32);
        let a4567 = f1095(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4566, a4567)
    };
    let a4569 = f1095(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4568, a4569)
}

pub fn f1089(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4578 = {
        let a4574 = {
            let a4570 = v0;
            let a4571 = v1;
            fixed_u32::bit_and(a4570, a4571)
        };
        let a4575 = {
            let a4572 = v0;
            let a4573 = v2;
            fixed_u32::bit_and(a4572, a4573)
        };
        fixed_u32::bit_xor(a4574, a4575)
    };
    let a4579 = {
        let a4576 = v1;
        let a4577 = v2;
        fixed_u32::bit_and(a4576, a4577)
    };
    fixed_u32::bit_xor(a4578, a4579)
}

pub fn f1090(v0: Adt166) -> Adt166 {
    Adt166::C0(match v0.clone() {
        Adt166::C0(_, h4580, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4580
        }
    }, match v0.clone() {
        Adt166::C0(_, _, h4581, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4581
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, h4582, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4582
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, h4583, _, _, _, _, _, _, _, _, _, _, _) => {
            h4583
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, h4584, _, _, _, _, _, _, _, _, _, _) => {
            h4584
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, h4585, _, _, _, _, _, _, _, _, _) => {
            h4585
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, h4586, _, _, _, _, _, _, _, _) => {
            h4586
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, h4587, _, _, _, _, _, _, _) => {
            h4587
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, h4588, _, _, _, _, _, _) => {
            h4588
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, h4589, _, _, _, _, _) => {
            h4589
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, h4590, _, _, _, _) => {
            h4590
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4591, _, _, _) => {
            h4591
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4592, _, _) => {
            h4592
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4593, _) => {
            h4593
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4594) => {
            h4594
        }
    }, f1071(f1096(match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4595, _) => {
            h4595
        }
    }), f1071(match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, h4596, _, _, _, _, _, _) => {
            h4596
        }
    }, f1071(f1097(match v0.clone() {
        Adt166::C0(_, h4597, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4597
        }
    }), match v0 {
        Adt166::C0(h4598, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4598
        }
    }))))
}

pub fn f1091(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4601 = {
        let a4599 = v3;
        let a4600 = 8u64;
        nat_add(a4599, a4600)?
    };
    let a4602 = v2;
    if nat_eq(a4601, a4602) {
        Ok(f1098({
            let a4605 = {
                let a4603 = v1;
                let a4604 = 8u64;
                nat_mul(a4603, a4604)?
            };
            let a4606 = 4294967296u64;
            let a4607 = 0u64;
            nat_quot(a4605, a4606, a4607)
        }))
    } else {
        let a4610 = {
            let a4608 = v3;
            let a4609 = 4u64;
            nat_add(a4608, a4609)?
        };
        let a4611 = v2;
        if nat_eq(a4610, a4611) {
            Ok(f1098({
                let a4614 = {
                    let a4612 = v1;
                    let a4613 = 8u64;
                    nat_mul(a4612, a4613)?
                };
                let a4615 = 4294967296u64;
                let a4616 = 0u64;
                nat_rem(a4614, a4615, a4616)
            }))
        } else {
            Ok(f1082(f1094(v0.clone(), v1, v3), f1094(v0.clone(), v1, {
                let a4617 = v3;
                let a4618 = 1u64;
                nat_add(a4617, a4618)?
            }), f1094(v0.clone(), v1, {
                let a4619 = v3;
                let a4620 = 2u64;
                nat_add(a4619, a4620)?
            }), f1094(v0, v1, {
                let a4621 = v3;
                let a4622 = 3u64;
                nat_add(a4621, a4622)?
            })))
        }
    }
}

pub fn f1092(v0: u32, v1: u32) -> u32 {
    let m4625 = {
        let a4623 = v0;
        let a4624 = v1;
        fixed_u32::shift_left(a4623, a4624)
    };
    match m4625 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1093(v0: u64, v1: bool) -> u32 {
    if v1 {
        f1098({
            let a4626 = v0;
            let a4627 = 4294967296u64;
            let a4628 = 0u64;
            nat_quot(a4626, a4627, a4628)
        })
    } else {
        f1098({
            let a4629 = v0;
            let a4630 = 4294967296u64;
            let a4631 = 0u64;
            nat_rem(a4629, a4630, a4631)
        })
    }
}

pub fn f1094(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4632 = v2;
    let a4633 = v1;
    if nat_lt(a4632, a4633) {
        let m4636 = {
            let a4634 = v0;
            let a4635 = v2;
            index_bytes(a4634, a4635)
        };
        match m4636 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4638 = {
                    let a4637 = v3;
                    fixed_u32::convert(i128::from(a4637))
                };
                match m4638 {
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
        let a4639 = v2;
        let a4640 = v1;
        if nat_eq(a4639, a4640) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f1095(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4641 = f1074(v0, v1);
    let a4642 = f1092(v0, v2);
    fixed_u32::bit_or(a4641, a4642)
}

pub fn f1096(v0: u32) -> u32 {
    let a4645 = {
        let a4643 = f1095(v0, 17u32, 15u32);
        let a4644 = f1095(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4643, a4644)
    };
    let a4646 = f1074(v0, 10u32);
    fixed_u32::bit_xor(a4645, a4646)
}

pub fn f1097(v0: u32) -> u32 {
    let a4649 = {
        let a4647 = f1095(v0, 7u32, 25u32);
        let a4648 = f1095(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4647, a4648)
    };
    let a4650 = f1074(v0, 3u32);
    fixed_u32::bit_xor(a4649, a4650)
}

pub fn f1098(v0: u64) -> u32 {
    let (_, h4651) = {
        let (_, h4652) = f1099(Fn14::F1100, 32u64, (v0, (1u32, 0u32)));
        h4652
    };
    h4651
}

pub fn f1099(v0: Fn14, p4656: u64, p4657: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4655 = (p4656, p4657);
    {
        let mut m4655 = m4655;
        loop {
            let step = {
                let (v1, v2) = m4655;
                let m4653 = v1;
                if m4653 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4653 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4654: Fn14 = v0.clone();
                        c4654.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1100(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4658, _) = v0;
        h4658
    };
    let v2: u32 = {
        let (h4659, _) = {
            let (_, h4660) = v0;
            h4660
        };
        h4659
    };
    let v3: u32 = {
        let (_, h4661) = {
            let (_, h4662) = v0;
            h4662
        };
        h4661
    };
    ({
        let a4663 = v1;
        let a4664 = 2u64;
        let a4665 = 0u64;
        nat_quot(a4663, a4664, a4665)
    }, (f1092(v2, 1u32), {
        let a4669 = {
            let a4666 = v1;
            let a4667 = 2u64;
            let a4668 = 0u64;
            nat_rem(a4666, a4667, a4668)
        };
        let a4670 = 1u64;
        if nat_eq(a4669, a4670) {
            let a4671 = v3;
            let a4672 = v2;
            fixed_u32::bit_or(a4671, a4672)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
