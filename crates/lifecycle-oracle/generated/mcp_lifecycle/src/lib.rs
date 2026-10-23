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
    F39,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt49, Adt84>> {
        match self {
            Fn1::F39 => f39(p0),
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
    Ok(List::cons(f27(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f28()), List::cons(f27(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f29()), List::cons(f27(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f28()), List::cons(f27(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f30()), List::cons(f27(Str::lit("test_image_content"), Str::lit("Tests image content response"), f30()), List::cons(f27(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f30()), List::cons(f27(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f30()), List::cons(f27(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f30()), List::cons(f27(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f30()), List::cons(f27(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f30()), List::cons(f27(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f31()), List::cons(f27(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f32()?), List::cons(f27(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f33()), List::cons(f34(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f30(), f35()), List::cons(f34(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f30(), f35()), List::cons(f27(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f30()), List::cons(f27(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f30()), List::cons(f27(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f30()), List::<Adt35>::nil())))))))))))))))))))
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
            Ok(Some(Adt84::C9(f36(v1))))
        }
    }
}

pub fn f15(v0: Adt14) -> R<Result<Adt32, Adt84>> {
    let m31 = v0;
    match m31 {
        Adt14::C0(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C1(v1) => {
            Ok(f37(f10(), f12()?, v1))
        }
        Adt14::C2(v2) => {
            f38(f12()?, Fn1::F39, v2)
        }
        Adt14::C3(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C4(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C5(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C6(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C7(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C8(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C9(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C10(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C11(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
    }
}

pub fn f16(v0: Bytes, v1: u64) -> R<Option<Adt89>> {
    let m32 = f40(v0.clone(), v1)?;
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
            f41(v0.clone(), v1, v3.clone(), f40(v0, match v3 {
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
    f42(f43(v0, List::<Adt86>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f44(f45(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt86> {
    List::cons(Adt86::C0(f46(), f47()), List::<Adt86>::nil())
}

pub fn f23() -> List<u64> {
    let a69 = f48();
    let a70 = {
        let a67 = f49();
        let a68 = f50();
        append_list(a67, a68)
    };
    append_list(a69, a70)
}

pub fn f24(v0: Adt0, v1: Fn0, v2: Adt85, v3: Option<Adt90>) -> R<Adt88> {
    let m71 = v3;
    match m71 {
        None => {
            f52(v0, v1, v2)
        }
        Some(v4) => {
            f51(v4)
        }
    }
}

pub fn f25(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m72 = f53(v0.clone(), v1.clone())?;
    match m72 {
        None => {
            f54(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f26(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt91>> {
    let m73 = f55(v0.clone());
    match m73 {
        None => {
            f56(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f27(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f28() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f29() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f57(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f58()), List::cons((Str::lit("limit"), f57(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f57(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f30() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f31() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f32() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f58()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f59(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f59(0u64)?), List::cons((Str::lit("maximum"), f59(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f59(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
}

pub fn f33() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("target"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("zone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt6::C3(Str::lit("Zone"))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f34(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt36>, None::<Adt5>)
}

pub fn f35() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("sum"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("sum")), List::cons(Adt6::C3(Str::lit("label")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f36(v0: Adt91) -> Str {
    let m74 = v0;
    match m74 {
        Adt91::C0(v1) => {
            let a75 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a76 = Str::lit(" ");
            join(a75, a76)
        }
        Adt91::C1(v2) => {
            let a77 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a78 = Str::lit(" ");
            join(a77, a78)
        }
        Adt91::C2(v3) => {
            let a79 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a80 = Str::lit(" ");
            join(a79, a80)
        }
        Adt91::C3(v4) => {
            let a81 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a82 = Str::lit(" ");
            join(a81, a82)
        }
        Adt91::C4(v5, v6) => {
            let a83 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a84 = Str::lit(" ");
            join(a83, a84)
        }
        Adt91::C5(v7, v8) => {
            let a85 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a86 = Str::lit(" ");
            join(a85, a86)
        }
        Adt91::C6(v9, v10) => {
            let a87 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a88 = Str::lit(" ");
            join(a87, a88)
        }
        Adt91::C7(v11) => {
            let a89 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a90 = Str::lit(" ");
            join(a89, a90)
        }
        Adt91::C8(v12) => {
            let a91 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a92 = Str::lit(" ");
            join(a91, a92)
        }
        Adt91::C9(v13) => {
            let a93 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a94 = Str::lit(" ");
            join(a93, a94)
        }
        Adt91::C10(v14) => {
            let a95 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a96 = Str::lit(" ");
            join(a95, a96)
        }
        Adt91::C11(v15) => {
            let a97 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a98 = Str::lit(" ");
            join(a97, a98)
        }
        Adt91::C12(v16, v17) => {
            let a99 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a100 = Str::lit(" ");
            join(a99, a100)
        }
    }
}

pub fn f37(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m102 = match v2 {
        Adt20::C0(_, h101) => {
            h101
        }
    };
    match m102 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C1(Adt34::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f38(v0: List<Adt35>, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m104 = f60(match v2.clone() {
        Adt21::C0(_, h103, _, _, _) => {
            h103
        }
    }, v0);
    match m104 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C0(match v2 {
                Adt21::C0(_, h105, _, _, _) => {
                    h105
                }
            })))
        }
        Some(v3) => {
            f61(v3, v1, v2)
        }
    }
}

pub fn f39(v0: Adt21) -> R<Result<Adt49, Adt84>> {
    if f62(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt49, Adt84>(f63(List::cons(f64(Str::lit("This is a simple text response for testing.")), List::<Adt38>::nil()))))
    } else {
        if f62(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt49, Adt84>(f63(List::cons(f65(), List::<Adt38>::nil()))))
        } else {
            if f62(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt49, Adt84>(f63(List::cons(f66(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt38>::nil()))))
            } else {
                if f62(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt49, Adt84>(f63(List::cons(f67(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt38>::nil()))))
                } else {
                    if f62(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt49, Adt84>(f63(List::cons(f64(Str::lit("Multiple content types test:")), List::cons(f65(), List::cons(f67(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt38>::nil()))))))
                    } else {
                        if f62(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt49, Adt84>(f68(List::cons(f64(Str::lit("This tool intentionally returns an error for testing")), List::<Adt38>::nil()), None::<Adt6>, true, List::<Adt50>::nil())))
                        } else {
                            if f62(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt49, Adt84>(f69(v0, f70()?)))
                            } else {
                                if f62(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt49, Adt84>(f69(v0, f71()?)))
                                } else {
                                    if f62(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt49, Adt84>(f68(List::cons(f64(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt38>::nil()), Some(f72()?), false, List::<Adt50>::nil())))
                                    } else {
                                        if f62(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt49, Adt84>(f68(List::cons(f64(Str::lit("{\"sum\":\"three\"}")), List::<Adt38>::nil()), Some(f73()), false, List::<Adt50>::nil())))
                                        } else {
                                            if f62(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt49, Adt84>(f63(List::cons(f74(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt38>::nil()))))
                                            } else {
                                                if f62(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt49, Adt84>(f63(List::cons(f64(f75()), List::<Adt38>::nil()))))
                                                } else {
                                                    Ok(f76(v0))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f40(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a109 = {
        let a106 = v1;
        let a107 = 4u64;
        nat_add(a106, a107)?
    };
    let a110 = {
        let a108 = v0.clone();
        length_bytes(a108)
    };
    if nat_le(a109, a110) {
        Ok(Some({
            let a127 = {
                let a111 = f77(v0.clone(), v1)?;
                let a112 = 16777216u64;
                nat_mul(a111, a112)?
            };
            let a128 = {
                let a125 = {
                    let a115 = f77(v0.clone(), {
                        let a113 = v1;
                        let a114 = 1u64;
                        nat_add(a113, a114)?
                    })?;
                    let a116 = 65536u64;
                    nat_mul(a115, a116)?
                };
                let a126 = {
                    let a123 = {
                        let a119 = f77(v0.clone(), {
                            let a117 = v1;
                            let a118 = 2u64;
                            nat_add(a117, a118)?
                        })?;
                        let a120 = 256u64;
                        nat_mul(a119, a120)?
                    };
                    let a124 = f77(v0, {
                        let a121 = v1;
                        let a122 = 3u64;
                        nat_add(a121, a122)?
                    })?;
                    nat_add(a123, a124)?
                };
                nat_add(a125, a126)?
            };
            nat_add(a127, a128)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f41(v0: Bytes, v1: Adt89, v2: Adt89, v3: Option<u64>) -> R<Result<Adt85, Adt87>> {
    let m129 = v3;
    match m129 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v2 {
                Adt89::C0(_, h130) => {
                    h130
                }
            })))
        }
        Some(v4) => {
            let m134 = f78(v0.clone(), {
                let a132 = match v2.clone() {
                    Adt89::C0(_, h131) => {
                        h131
                    }
                };
                let a133 = 4u64;
                nat_add(a132, a133)?
            }, v4, List::<Adt86>::nil())?;
            match m134 {
                Ok(v6) => {
                    f79(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt85, Adt87>(v5))
                }
            }
        }
    }
}

pub fn f42(p139: List<Adt86>, p140: List<u64>) -> List<u64> {
    let m138 = (p139, p140);
    {
        let mut m138 = m138;
        loop {
            let step = {
                let (v0, v1) = m138;
                let m135 = v0;
                match m135.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<u64>), List<u64>>((v3, {
                            let a136 = f80(v2);
                            let a137 = v1;
                            append_list(a136, a137)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(p143: List<Adt86>, p144: List<Adt86>) -> List<Adt86> {
    let m142 = (p143, p144);
    {
        let mut m142 = m142;
        loop {
            let step = {
                let (v0, v1) = m142;
                let m141 = v0;
                match m141.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(p149: List<List<u64>>, p150: List<u64>) -> List<u64> {
    let m148 = (p149, p150);
    {
        let mut m148 = m148;
        loop {
            let step = {
                let (v0, v1) = m148;
                let m145 = v0;
                match m145.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a146 = f81(v2);
                            let a147 = v1;
                            append_list(a146, a147)
                        }))
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

pub fn f45(p153: List<List<u64>>, p154: List<List<u64>>) -> List<List<u64>> {
    let m152 = (p153, p154);
    {
        let mut m152 = m152;
        loop {
            let step = {
                let (v0, v1) = m152;
                let m151 = v0;
                match m151.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m152 = next; }
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

pub fn f51(v0: Adt90) -> R<Adt88> {
    f82(f83(v0.clone()), f84(None::<Adt92>, f85(v0)))
}

pub fn f52(v0: Adt0, v1: Fn0, v2: Adt85) -> R<Adt88> {
    if f86(match v2.clone() {
        Adt85::C0(h155, _, _, _, _, _) => {
            h155
        }
    }) {
        f87(v0, v1, match v2.clone() {
            Adt85::C0(_, _, _, _, h156, _) => {
                h156
            }
        }, match v2.clone() {
            Adt85::C0(_, _, h157, _, _, _) => {
                h157
            }
        }, f88(match v2 {
            Adt85::C0(_, _, _, h158, _, _) => {
                h158
            }
        })?)
    } else {
        Ok(f89())
    }
}

pub fn f53(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m159 = f90(v0, f91())?;
    match m159 {
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
            f92(f93(v2), v1, Adt90::C0)
        }
    }
}

pub fn f54(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m160 = f90(v0, f94())?;
    match m160 {
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
            f92(f95(v2), v1, Adt90::C1)
        }
    }
}

pub fn f55(v0: List<Adt35>) -> Option<Adt91> {
    let m161 = f96(v0, List::<Str>::nil());
    match m161 {
        None => {
            None::<Adt91>
        }
        Some(v1) => {
            Some(Adt91::C1(v1))
        }
    }
}

pub fn f56(p165: List<Adt35>, p166: List<Adt12>) -> R<Result<List<Adt12>, Adt91>> {
    let m164 = (p165, p166);
    {
        let mut m164 = m164;
        loop {
            let step = {
                let (v0, v1) = m164;
                let m162 = v0;
                match m162.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Ok::<List<Adt12>, Adt91>(f97(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m163 = f98(v2)?;
                        match m163 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>((v3, f97(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Err::<List<Adt12>, Adt91>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m164 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f58() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f59(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f99(v0, 0i64)?, 0i64)))
}

pub fn f60(v0: Str, p172: List<Adt35>) -> Option<Adt35> {
    let m171 = p172;
    {
        let mut m171 = m171;
        loop {
            let step = {
                let v1 = m171;
                let m167 = v1;
                match m167.uncons() {
                    None => {
                        Err::<List<Adt35>, Option<Adt35>>(None::<Adt35>)
                    }
                    Some((v2, v3)) => {
                        let a169 = match v2.clone() {
                            Adt35::C0(h168, _, _, _, _, _, _, _) => {
                                h168
                            }
                        };
                        let a170 = v0.clone();
                        if equal(a169, a170) {
                            Err::<List<Adt35>, Option<Adt35>>(Some(v2))
                        } else {
                            Ok::<List<Adt35>, Option<Adt35>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m179 = f100(Adt6::C5({
        let m175 = match v0.clone() {
            Adt35::C0(_, _, _, _, h174, _, _, _) => {
                h174
            }
        };
        match m175 {
            Adt5::C0(h173) => {
                h173
            }
        }
    }), Adt6::C5({
        let m178 = match v0.clone() {
            Adt35::C0(_, _, _, _, h177, _, _, _) => {
                h177
            }
        };
        match m178 {
            Adt5::C0(h176) => {
                h176
            }
        }
    }), f101(v2.clone()))?;
    match m179 {
        Adt98::C0 => {
            f102(v0, v1, v2)
        }
        Adt98::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f103(f104(v3, v4)?, true))))
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

pub fn f62(v0: Adt21, v1: Str) -> bool {
    let a181 = match v0 {
        Adt21::C0(_, h180, _, _, _) => {
            h180
        }
    };
    let a182 = v1;
    equal(a181, a182)
}

pub fn f63(v0: List<Adt38>) -> Adt49 {
    f68(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f64(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f65() -> Adt38 {
    f74(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f66(v0: Str, v1: Str) -> Adt38 {
    Adt38::C2(Adt43::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f67(v0: Str, v1: Str, v2: Str) -> Adt38 {
    Adt38::C4(Adt45::C0(Adt46::C0(Adt47::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt40>, None::<Adt5>))
}

pub fn f68(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f69(v0: Adt21, v1: List<Adt50>) -> Adt49 {
    f68(List::cons(f64(f105({
        let m185 = match v0 {
            Adt21::C0(h184, _, _, _, _) => {
                h184
            }
        };
        match m185 {
            Adt16::C0(_, _, _, _, h183, _) => {
                h183
            }
        }
    })), List::<Adt38>::nil()), None::<Adt6>, false, v1)
}

pub fn f70() -> R<List<Adt50>> {
    Ok(List::cons(f106(0u64, 100u64)?, List::cons(f106(50u64, 100u64)?, List::cons(f106(100u64, 100u64)?, List::<Adt50>::nil()))))
}

pub fn f71() -> R<List<Adt50>> {
    Ok(List::cons(f106(50u64, 100u64)?, List::cons(f106(10u64, 100u64)?, List::<Adt50>::nil())))
}

pub fn f72() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f59(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f73() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f74(v0: Str, v1: Str) -> Adt38 {
    Adt38::C1(Adt42::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f75() -> Str {
    let m187 = {
        let a186 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a186)
    };
    match m187 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f76(v0: Adt21) -> Result<Adt49, Adt84> {
    if f62(v0.clone(), Str::lit("test_missing_capability")) {
        let m193 = {
            let m192 = {
                let m191 = match v0 {
                    Adt21::C0(h190, _, _, _, _) => {
                        h190
                    }
                };
                match m191 {
                    Adt16::C0(_, h189, _, _, _, _) => {
                        h189
                    }
                }
            };
            match m192 {
                Adt17::C0(_, _, h188, _, _) => {
                    h188
                }
            }
        };
        match m193 {
            None => {
                Err::<Adt49, Adt84>(Adt84::C8(f107()))
            }
            Some(_) => {
                Ok::<Adt49, Adt84>(f108())
            }
        }
    } else {
        if f62(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt49, Adt84>(f63(List::cons(f64(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt38>::nil())))
        } else {
            if f62(v0, Str::lit("test_arguments")) {
                Ok::<Adt49, Adt84>(f63(List::cons(f64(Str::lit("arguments accepted")), List::<Adt38>::nil())))
            } else {
                Ok::<Adt49, Adt84>(f108())
            }
        }
    }
}

pub fn f77(v0: Bytes, v1: u64) -> R<u64> {
    let m196 = {
        let a194 = v0;
        let a195 = v1;
        index_bytes(a194, a195)
    };
    match m196 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a241 = {
                let a217 = {
                    let a205 = {
                        let a199 = {
                            let a197 = v3;
                            let a198 = 128u8;
                            fixed_u8::bit_and(a197, a198)
                        };
                        let a200 = 128u8;
                        if equal(a199, a200) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a206 = {
                        let a203 = {
                            let a201 = v3;
                            let a202 = 64u8;
                            fixed_u8::bit_and(a201, a202)
                        };
                        let a204 = 64u8;
                        if equal(a203, a204) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a205, a206)?
                };
                let a218 = {
                    let a215 = {
                        let a209 = {
                            let a207 = v3;
                            let a208 = 32u8;
                            fixed_u8::bit_and(a207, a208)
                        };
                        let a210 = 32u8;
                        if equal(a209, a210) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a216 = {
                        let a213 = {
                            let a211 = v3;
                            let a212 = 16u8;
                            fixed_u8::bit_and(a211, a212)
                        };
                        let a214 = 16u8;
                        if equal(a213, a214) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a215, a216)?
                };
                nat_add(a217, a218)?
            };
            let a242 = {
                let a239 = {
                    let a227 = {
                        let a221 = {
                            let a219 = v3;
                            let a220 = 8u8;
                            fixed_u8::bit_and(a219, a220)
                        };
                        let a222 = 8u8;
                        if equal(a221, a222) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a228 = {
                        let a225 = {
                            let a223 = v3;
                            let a224 = 4u8;
                            fixed_u8::bit_and(a223, a224)
                        };
                        let a226 = 4u8;
                        if equal(a225, a226) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a227, a228)?
                };
                let a240 = {
                    let a237 = {
                        let a231 = {
                            let a229 = v3;
                            let a230 = 2u8;
                            fixed_u8::bit_and(a229, a230)
                        };
                        let a232 = 2u8;
                        if equal(a231, a232) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a238 = {
                        let a235 = {
                            let a233 = v3;
                            let a234 = 1u8;
                            fixed_u8::bit_and(a233, a234)
                        };
                        let a236 = 1u8;
                        if equal(a235, a236) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a237, a238)?
                };
                nat_add(a239, a240)?
            };
            nat_add(a241, a242)
        }
    }
}

pub fn f78(v0: Bytes, p252: u64, p253: u64, p254: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m251 = (p252, (p253, p254));
    {
        let mut m251 = m251;
        loop {
            let step = {
                let (v1, (v2, v3)) = m251;
                let m243 = v2;
                if m243 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f109(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m243 - 1;
                    let m244 = f16(v0.clone(), v1)?;
                    match m244 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m246 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h245) => {
                                    h245
                                }
                            })?;
                            match m246 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h247) => {
                                            h247
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h248) => {
                                            h248
                                        }
                                    }, (v4, List::cons(Adt86::C0(f110(match v5 {
                                        Adt89::C0(h249, _) => {
                                            h249
                                        }
                                    })?, f110(match v6 {
                                        Adt89::C0(h250, _) => {
                                            h250
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m256 = f16(v0.clone(), {
        let (_, h255) = v3.clone();
        h255
    })?;
    match m256 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h257) = v3;
                h257
            })))
        }
        Some(v4) => {
            let m259 = f40(v0.clone(), match v4.clone() {
                Adt89::C0(_, h258) => {
                    h258
                }
            })?;
            match m259 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h260) => {
                            h260
                        }
                    })))
                }
                Some(v5) => {
                    f111(v0, Adt85::C0(f110(match v1 {
                        Adt89::C0(h261, _) => {
                            h261
                        }
                    })?, f110(match v2 {
                        Adt89::C0(h262, _) => {
                            h262
                        }
                    })?, {
                        let (h263, _) = v3;
                        h263
                    }, match v4.clone() {
                        Adt89::C0(h264, _) => {
                            h264
                        }
                    }, v5, List::<List<u64>>::nil()), {
                        let a266 = match v4 {
                            Adt89::C0(_, h265) => {
                                h265
                            }
                        };
                        let a267 = 4u64;
                        nat_add(a266, a267)?
                    })
                }
            }
        }
    }
}

pub fn f80(v0: Adt86) -> List<u64> {
    let a278 = f18({
        let a269 = match v0.clone() {
            Adt86::C0(h268, _) => {
                h268
            }
        };
        length_list(a269)
    });
    let a279 = {
        let a276 = match v0.clone() {
            Adt86::C0(h270, _) => {
                h270
            }
        };
        let a277 = {
            let a274 = f18({
                let a272 = match v0.clone() {
                    Adt86::C0(_, h271) => {
                        h271
                    }
                };
                length_list(a272)
            });
            let a275 = match v0 {
                Adt86::C0(_, h273) => {
                    h273
                }
            };
            append_list(a274, a275)
        };
        append_list(a276, a277)
    };
    append_list(a278, a279)
}

pub fn f81(v0: List<u64>) -> List<u64> {
    let a281 = f18({
        let a280 = v0.clone();
        length_list(a280)
    });
    let a282 = v0;
    append_list(a281, a282)
}

pub fn f82(v0: u64, v1: Adt6) -> R<Adt88> {
    let m283 = f112(v1)?;
    match m283 {
        Ok(v3) => {
            Ok(Adt88::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f83(v0: Adt90) -> u64 {
    let m284 = v0;
    match m284 {
        Adt90::C0 => {
            f113()
        }
        Adt90::C1 => {
            f113()
        }
    }
}

pub fn f84(v0: Option<Adt92>, v1: Adt80) -> Adt6 {
    Adt6::C5(f114(v0, v1))
}

pub fn f85(v0: Adt90) -> Adt80 {
    Adt80::C0(f115(), f116(v0), None::<Adt6>)
}

pub fn f86(v0: List<u64>) -> bool {
    f117(v0, f118())
}

pub fn f87(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt93>) -> R<Adt88> {
    let m285 = v4;
    match m285 {
        Ok(v6) => {
            f121(v0, v1, v2, v3, f122(v6)?)
        }
        Err(_) => {
            f82(f119(), f84(None::<Adt92>, f120(Adt94::C0)))
        }
    }
}

pub fn f88(v0: Bytes) -> R<Result<Adt6, Adt93>> {
    f123(f110(v0)?)
}

pub fn f89() -> Adt88 {
    Adt88::C0(f124(), List::cons(Adt86::C0(f125(), f126()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f90(v0: List<Adt86>, v1: List<u64>) -> R<Adt96> {
    let m286 = f127(v0, v1, Adt97::C0)?;
    match m286 {
        Adt97::C0 => {
            Ok(Adt96::C0)
        }
        Adt97::C1(v2) => {
            let m287 = f128(v2);
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

pub fn f91() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f92(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt90) -> R<Option<Adt90>> {
    let m288 = v0;
    match m288 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f129(v3, v1)? {
                Ok(None::<Adt90>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f93(v0: List<u64>) -> Option<List<u64>> {
    let m289 = f130(v0);
    match m289 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f95(v1)
        }
    }
}

pub fn f94() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f95(v0: List<u64>) -> Option<List<u64>> {
    let m290 = v0.clone();
    match m290.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a291 = v1;
            let a292 = 91u64;
            if nat_eq(a291, a292) {
                f131(f132(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f131(f133(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f96(p298: List<Adt35>, p299: List<Str>) -> Option<Str> {
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
                        if f134(match v2.clone() {
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

pub fn f97(p302: List<Adt12>, p303: List<Adt12>) -> List<Adt12> {
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

pub fn f98(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m306 = f135(match v0.clone() {
        Adt35::C0(h304, _, _, _, _, _, _, _) => {
            h304
        }
    }, f136(match v0.clone() {
        Adt35::C0(h305, _, _, _, _, _, _, _) => {
            h305
        }
    })?);
    match m306 {
        None => {
            f137(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f99(p311: u64, p312: i64) -> R<i64> {
    let m310 = (p311, p312);
    {
        let mut m310 = m310;
        loop {
            let step = {
                let (v0, v1) = m310;
                let m307 = v0;
                if m307 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m307 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a308 = v1;
                        let a309 = -1i64;
                        int_sub(a308, a309)?
                    }))
                }
            };
            match step {
                Ok(next) => { m310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt98> {
    f138(v0, f139(), Adt100::C0, List::cons(Adt101::C0(v1, v2, List::<Adt99>::nil()), List::<Adt101>::nil()))
}

pub fn f101(v0: Adt21) -> Adt6 {
    let m314 = match v0 {
        Adt21::C0(_, _, h313, _, _) => {
            h313
        }
    };
    match m314 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h315) => {
                    h315
                }
            })
        }
    }
}

pub fn f102(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m317 = {
        let c316: Fn1 = v1;
        c316.apply(v2)?
    };
    match m317 {
        Ok(v4) => {
            f140(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f103(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f104(v0: List<Adt99>, v1: Str) -> R<Str> {
    let v2: Str = f141(v0)?;
    let a320 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a318 = v2.clone();
        let a319 = Str::lit("");
        if equal(a318, a319) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a321 = Str::lit(" ");
    Ok(join(a320, a321))
}

pub fn f105(v0: Option<Adt19>) -> Str {
    let m322 = v0;
    match m322 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m323 = v1;
            match m323 {
                Adt19::C0(v2) => {
                    v2
                }
                Adt19::C1(v3) => {
                    let a324 = v3;
                    format_i64(a324)
                }
            }
        }
    }
}

pub fn f106(v0: u64, v1: u64) -> R<Adt50> {
    Ok(Adt50::C0(f142(v0)?, Some(f142(v1)?), Some({
        let a327 = List::cons(Str::lit("Completed step"), List::cons({
            let a325 = f143(v0, 0i64)?;
            format_i64(a325)
        }, List::cons(Str::lit("of"), List::cons({
            let a326 = f143(v1, 0i64)?;
            format_i64(a326)
        }, List::<Str>::nil()))));
        let a328 = Str::lit(" ");
        join(a327, a328)
    })))
}

pub fn f107() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, Some(f144()), None::<Adt5>, None::<Adt5>)
}

pub fn f108() -> Adt49 {
    f63(List::cons(f64(Str::lit("ok")), List::<Adt38>::nil()))
}

pub fn f109(p331: List<Adt86>, p332: List<Adt86>) -> List<Adt86> {
    let m330 = (p331, p332);
    {
        let mut m330 = m330;
        loop {
            let step = {
                let (v0, v1) = m330;
                let m329 = v0;
                match m329.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
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

pub fn f110(v0: Bytes) -> R<List<u64>> {
    f145(v0.clone(), {
        let a333 = v0;
        length_bytes(a333)
    }, List::<u64>::nil())
}

pub fn f111(v0: Bytes, v1: Adt85, v2: u64) -> R<Result<Adt85, Adt87>> {
    let a335 = v2;
    let a336 = {
        let a334 = v0.clone();
        length_bytes(a334)
    };
    if nat_eq(a335, a336) {
        Ok(Ok::<Adt85, Adt87>(v1))
    } else {
        let m337 = f40(v0.clone(), v2)?;
        match m337 {
            None => {
                Ok(Err::<Adt85, Adt87>(Adt87::C0(v2)))
            }
            Some(v3) => {
                let m340 = f146(v0.clone(), {
                    let a338 = v2;
                    let a339 = 4u64;
                    nat_add(a338, a339)?
                }, v3, List::<List<u64>>::nil())?;
                match m340 {
                    Ok(v5) => {
                        let a343 = {
                            let (_, h341) = v5.clone();
                            h341
                        };
                        let a344 = {
                            let a342 = v0;
                            length_bytes(a342)
                        };
                        if nat_eq(a343, a344) {
                            Ok(Ok::<Adt85, Adt87>(f147(v1, {
                                let (h345, _) = v5;
                                h345
                            })))
                        } else {
                            Ok(Err::<Adt85, Adt87>(Adt87::C1({
                                let (_, h346) = v5;
                                h346
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

pub fn f112(v0: Adt6) -> R<Result<List<u64>, Adt93>> {
    f148(f149(), v0)
}

pub fn f113() -> u64 {
    403u64
}

pub fn f114(v0: Option<Adt92>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f150())), {
        let a347 = f151(v0);
        let a348 = List::cons((Str::lit("error"), f152(v1)), List::<(Str, Adt6)>::nil());
        append_list(a347, a348)
    })
}

pub fn f115() -> i64 {
    -32000i64
}

pub fn f116(v0: Adt90) -> Str {
    let m349 = v0;
    match m349 {
        Adt90::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt90::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f117(p356: List<u64>, p357: List<u64>) -> bool {
    let m355 = (p356, p357);
    {
        let mut m355 = m355;
        loop {
            let step = {
                let (v0, v1) = m355;
                let m350 = v0;
                match m350.uncons() {
                    None => {
                        let m351 = v1;
                        match m351.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m352 = v1;
                        match m352.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a353 = v4;
                                let a354 = v6;
                                if nat_eq(a353, a354) {
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
                Ok(next) => { m355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f119() -> u64 {
    400u64
}

pub fn f120(v0: Adt94) -> Adt80 {
    Adt80::C0(f153(v0.clone()), f154(v0), None::<Adt6>)
}

pub fn f121(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt103, Adt94>) -> R<Adt88> {
    let m358 = v4;
    match m358 {
        Ok(v6) => {
            f157(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f82(f155(v5.clone()), f84(f156(v5.clone()), f120(v5)))
        }
    }
}

pub fn f122(v0: Adt6) -> R<Result<Adt103, Adt94>> {
    let m359 = v0;
    match m359 {
        Adt6::C0 => {
            Ok(f159(None::<Adt92>, Adt95::C0))
        }
        Adt6::C1(_) => {
            Ok(f159(None::<Adt92>, Adt95::C0))
        }
        Adt6::C2(_) => {
            Ok(f159(None::<Adt92>, Adt95::C0))
        }
        Adt6::C3(_) => {
            Ok(f159(None::<Adt92>, Adt95::C0))
        }
        Adt6::C4(_) => {
            Ok(f159(None::<Adt92>, Adt95::C1))
        }
        Adt6::C5(v1) => {
            f158(Adt5::C0(v1))
        }
    }
}

pub fn f123(v0: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m360 = f160(v0.clone())?;
    match m360 {
        Adt112::C0 => {
            f161(f162(), 0u64, v0)
        }
        Adt112::C1(v1) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C0(v1)))
        }
        Adt112::C2(v2) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C1(v2)))
        }
    }
}

pub fn f124() -> u64 {
    405u64
}

pub fn f125() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f126() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f127(p365: List<Adt86>, v1: List<u64>, p366: Adt97) -> R<Adt97> {
    let m364 = (p365, p366);
    {
        let mut m364 = m364;
        loop {
            let step = {
                let (v0, v2) = m364;
                let m361 = v0;
                match m361.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt97), R<Adt97>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f163(match v3.clone() {
                            Adt86::C0(h362, _) => {
                                h362
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, f164(v2, match v3 {
                                Adt86::C0(_, h363) => {
                                    h363
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f165(v0);
    if f166(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f129(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f163(v0.clone(), f167())? {
        Ok(true)
    } else {
        if f163(v0.clone(), f168())? {
            Ok(true)
        } else {
            if f163(v0.clone(), f169())? {
                Ok(true)
            } else {
                f170(v0, v1)
            }
        }
    }
}

pub fn f130(p371: List<u64>) -> Option<List<u64>> {
    let m370 = p371;
    {
        let mut m370 = m370;
        loop {
            let step = {
                let v0 = m370;
                let m367 = v0;
                match m367.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a368 = v1;
                        let a369 = 58u64;
                        if nat_eq(a368, a369) {
                            Err::<List<u64>, Option<List<u64>>>(f171(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m372 = v0;
    match m372 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f172({
                let (_, h373) = v1;
                h373
            }) {
                Some({
                    let (h374, _) = v1;
                    h374
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f132(p379: List<u64>, p380: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m378 = (p379, p380);
    {
        let mut m378 = m378;
        loop {
            let step = {
                let (v0, v1) = m378;
                let m375 = v0;
                match m375.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a376 = v2;
                        let a377 = 93u64;
                        if nat_eq(a376, a377) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f173(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(p385: List<u64>, p386: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m384 = (p385, p386);
    {
        let mut m384 = m384;
        loop {
            let step = {
                let (v0, v1) = m384;
                let m381 = v0;
                match m381.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f173(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a382 = v2;
                        let a383 = 58u64;
                        if nat_eq(a382, a383) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f173(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m384 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: Str, p391: List<Str>) -> bool {
    let m390 = p391;
    {
        let mut m390 = m390;
        loop {
            let step = {
                let v1 = m390;
                let m387 = v1;
                match m387.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a388 = v0.clone();
                        let a389 = v2;
                        if equal(a388, a389) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: Str, v1: List<u64>) -> Option<Adt91> {
    if f174(v1) {
        Some(Adt91::C0(v0))
    } else {
        None::<Adt91>
    }
}

pub fn f136(v0: Str) -> R<List<u64>> {
    f110({
        let a392 = v0;
        utf8_encode(a392)
    })
}

pub fn f137(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m393 = f175(v0.clone());
    match m393 {
        None => {
            f176(v0.clone(), f177(f178(), match v0.clone() {
                Adt35::C0(h394, _, _, _, _, _, _, _) => {
                    h394
                }
            }, Adt6::C5({
                let m397 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h396, _, _, _) => {
                        h396
                    }
                };
                match m397 {
                    Adt5::C0(h395) => {
                        h395
                    }
                }
            }), List::cons(Adt104::C0(Adt6::C5({
                let m400 = match v0 {
                    Adt35::C0(_, _, _, _, h399, _, _, _) => {
                        h399
                    }
                };
                match m400 {
                    Adt5::C0(h398) => {
                        h398
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt104>::nil()), f179())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f138(v0: Adt6, p419: u64, p420: Adt100, p421: List<Adt101>) -> R<Adt98> {
    let m418 = (p419, (p420, p421));
    {
        let mut m418 = m418;
        loop {
            let step = {
                let (v1, (v2, v3)) = m418;
                let m401 = v1;
                if m401 == 0 {
                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C4))
                } else {
                    let v4 = m401 - 1;
                    let m402 = v3;
                    match m402.uncons() {
                        None => {
                            let m403 = v2;
                            match m403 {
                                Adt100::C0 => {
                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C0))
                                }
                                Adt100::C1(v5, v6) => {
                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m404 = v2.clone();
                            match m404 {
                                Adt100::C0 => {
                                    let m405 = v7.clone();
                                    match m405 {
                                        Adt101::C0(v9, v10, v11) => {
                                            let m406 = f180(v0.clone(), v9, v10, v11.clone())?;
                                            match m406 {
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
                                                        let a407 = v15;
                                                        let a408 = v8;
                                                        append_list(a407, a408)
                                                    };
                                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt101::C1(_, _, _) => {
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v8)))
                                        }
                                        Adt101::C2(_, _, _, _) => {
                                            let v24: Adt105 = f181(v7, v8)?;
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v24.clone() {
                                                Adt105::C0(h409, _) => {
                                                    h409
                                                }
                                            }, match v24 {
                                                Adt105::C0(_, h410) => {
                                                    h410
                                                }
                                            })))
                                        }
                                        Adt101::C3(v25) => {
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (Adt100::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt101::C4(_, _, _, _) => {
                                            let v39: Adt105 = f181(v7, v8)?;
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v39.clone() {
                                                Adt105::C0(h414, _) => {
                                                    h414
                                                }
                                            }, match v39 {
                                                Adt105::C0(_, h415) => {
                                                    h415
                                                }
                                            })))
                                        }
                                        Adt101::C5(v26, v27, v28, v29) => {
                                            let m411 = f182(v26, v27, v28, v29.clone());
                                            match m411 {
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
                                                        let a412 = v33;
                                                        let a413 = v8;
                                                        append_list(a412, a413)
                                                    };
                                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt100::C1(v40, v41) => {
                                    let v42: Adt105 = f183(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v42.clone() {
                                        Adt105::C0(h416, _) => {
                                            h416
                                        }
                                    }, match v42 {
                                        Adt105::C0(_, h417) => {
                                            h417
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139() -> u64 {
    67108864u64
}

pub fn f140(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f184({
        let m424 = match v1.clone() {
            Adt49::C0(h423, _) => {
                h423
            }
        };
        match m424 {
            Adt37::C0(h422, _, _) => {
                h422
            }
        }
    })? {
        f185(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f141(v0: List<Adt99>) -> R<Str> {
    let a425 = List::cons(Str::lit(""), f186(v0, List::<Str>::nil())?);
    let a426 = Str::lit("/");
    Ok(join(a425, a426))
}

pub fn f142(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f143(v0, 0i64)?, 0i64))
}

pub fn f143(p429: u64, p430: i64) -> R<i64> {
    let m428 = (p429, p430);
    {
        let mut m428 = m428;
        loop {
            let step = {
                let (v0, v1) = m428;
                let m427 = v0;
                if m427 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m427 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f187(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f145(v0: Bytes, p433: u64, p434: List<u64>) -> R<List<u64>> {
    let m432 = (p433, p434);
    {
        let mut m432 = m432;
        loop {
            let step = {
                let (v1, v2) = m432;
                let m431 = v1;
                if m431 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m431 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f77(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: Bytes, p440: u64, p441: u64, p442: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt87>> {
    let m439 = (p440, (p441, p442));
    {
        let mut m439 = m439;
        loop {
            let step = {
                let (v1, (v2, v3)) = m439;
                let m435 = v2;
                if m435 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Ok::<(List<List<u64>>, u64), Adt87>((f45(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m435 - 1;
                    let m436 = f16(v0.clone(), v1)?;
                    match m436 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Err::<(List<List<u64>>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>((match v5.clone() {
                                Adt89::C0(_, h437) => {
                                    h437
                                }
                            }, (v4, List::cons(f110(match v5 {
                                Adt89::C0(h438, _) => {
                                    h438
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Adt85, v1: List<List<u64>>) -> Adt85 {
    Adt85::C0(match v0.clone() {
        Adt85::C0(h443, _, _, _, _, _) => {
            h443
        }
    }, match v0.clone() {
        Adt85::C0(_, h444, _, _, _, _) => {
            h444
        }
    }, match v0.clone() {
        Adt85::C0(_, _, h445, _, _, _) => {
            h445
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, h446, _, _) => {
            h446
        }
    }, match v0 {
        Adt85::C0(_, _, _, _, h447, _) => {
            h447
        }
    }, v1)
}

pub fn f148(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt93>> {
    let m448 = f188(v0, List::cons(Adt106::C1(0u64, v1), List::<Adt106>::nil()), List::<u64>::nil())?;
    match m448 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt93>(f189(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt93>(v3))
        }
    }
}

pub fn f149() -> u64 {
    67108864u64
}

pub fn f150() -> Str {
    Str::lit("2.0")
}

pub fn f151(v0: Option<Adt92>) -> List<(Str, Adt6)> {
    let m449 = v0;
    match m449 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f190(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f152(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h450, _, _) => {
            h450
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h451, _) => {
            h451
        }
    })), f191(match v0 {
        Adt80::C0(_, _, h452) => {
            h452
        }
    }))))
}

pub fn f153(v0: Adt94) -> i64 {
    let m453 = v0;
    match m453 {
        Adt94::C0 => {
            f192()
        }
        Adt94::C1(_, _) => {
            f193()
        }
    }
}

pub fn f154(v0: Adt94) -> Str {
    let m454 = v0;
    match m454 {
        Adt94::C0 => {
            Str::lit("Parse error")
        }
        Adt94::C1(_, v2) => {
            f194(v2)
        }
    }
}

pub fn f155(v0: Adt94) -> u64 {
    let m455 = v0;
    match m455 {
        Adt94::C0 => {
            f119()
        }
        Adt94::C1(_, _) => {
            f119()
        }
    }
}

pub fn f156(v0: Adt94) -> Option<Adt92> {
    let m456 = v0;
    match m456 {
        Adt94::C0 => {
            None::<Adt92>
        }
        Adt94::C1(v1, _) => {
            v1
        }
    }
}

pub fn f157(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt103) -> R<Adt88> {
    f195(f196(v3.clone())?, f197(v0.clone(), v1, v2, Some(f198(v3, match v0 {
        Adt0::C0(_, _, _, _, h457, _) => {
            h457
        }
    }, f199(v4.clone()), match v4.clone() {
        Adt103::C0(_, h458, _) => {
            h458
        }
    })?), f200(), v4)?)
}

pub fn f158(v0: Adt5) -> R<Result<Adt103, Adt94>> {
    let m459 = f201(v0.clone())?;
    match m459 {
        Adt129::C0 => {
            let m460 = f202(Str::lit("method"), v0.clone());
            match m460 {
                Adt130::C0 => {
                    Ok(f159(None::<Adt92>, f203(v0)))
                }
                Adt130::C1(_) => {
                    Ok(f159(None::<Adt92>, if f204(v0) {
                        Adt95::C8
                    } else {
                        Adt95::C7
                    }))
                }
            }
        }
        Adt129::C1(v2) => {
            Ok(f205(v2, v0))
        }
        Adt129::C2 => {
            Ok(f159(None::<Adt92>, Adt95::C5))
        }
    }
}

pub fn f159(v0: Option<Adt92>, v1: Adt95) -> Result<Adt103, Adt94> {
    Err::<Adt103, Adt94>(Adt94::C1(v0, v1))
}

pub fn f160(v0: List<u64>) -> R<Adt112> {
    Ok(f206({
        let a461 = v0.clone();
        length_list(a461)
    }, f207(Adt113::C0, 0u64, 0u64, v0)?))
}

pub fn f161(p468: Adt114, p469: u64, p470: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m467 = (p468, (p469, p470));
    {
        let mut m467 = m467;
        loop {
            let step = {
                let (v0, (v1, v2)) = m467;
                let m462 = v2;
                match m462.uncons() {
                    None => {
                        let m463 = f208(v0, v1)?;
                        match m463 {
                            Ok(v3) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Ok::<Adt6, Adt93>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m464 = f209(v0, v1, v5)?;
                        match m464 {
                            Adt131::C0(v7) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v7)))
                            }
                            Adt131::C1(v8) => {
                                Ok::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>((v8, ({
                                    let a465 = v1;
                                    let a466 = 1u64;
                                    nat_add(a465, a466)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162() -> Adt114 {
    Adt114::C0(Adt115::C0, List::<Adt119>::nil(), 0u64)
}

pub fn f163(p477: List<u64>, p478: List<u64>) -> R<bool> {
    let m476 = (p477, p478);
    {
        let mut m476 = m476;
        loop {
            let step = {
                let (v0, v1) = m476;
                let m471 = v0;
                match m471.uncons() {
                    None => {
                        let m472 = v1;
                        match m472.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m473 = v1;
                        match m473.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a474 = f210(v4)?;
                                let a475 = f210(v6)?;
                                if nat_eq(a474, a475) {
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
                Ok(next) => { m476 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: Adt97, v1: List<u64>) -> Adt97 {
    let m479 = v0;
    match m479 {
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

pub fn f165(v0: List<u64>) -> List<u64> {
    f173(f211(f173(f211(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f166(p482: List<u64>) -> bool {
    let m481 = p482;
    {
        let mut m481 = m481;
        loop {
            let step = {
                let v0 = m481;
                let m480 = v0;
                match m480.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f212(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f167() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f168() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f169() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f170(v0: List<u64>, p485: List<List<u64>>) -> R<bool> {
    let m484 = p485;
    {
        let mut m484 = m484;
        loop {
            let step = {
                let v1 = m484;
                let m483 = v1;
                match m483.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f163(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: List<u64>) -> Option<List<u64>> {
    let m486 = v0;
    match m486.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m487 = v2;
            match m487.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a488 = v1;
                    let a489 = 47u64;
                    if if nat_eq(a488, a489) {
                        let a490 = v3;
                        let a491 = 47u64;
                        nat_eq(a490, a491)
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

pub fn f172(v0: List<u64>) -> bool {
    let m492 = v0;
    match m492.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a493 = v1;
            let a494 = 58u64;
            if nat_eq(a493, a494) {
                let m495 = v2.clone();
                match m495.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f213(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f173(p498: List<u64>, p499: List<u64>) -> List<u64> {
    let m497 = (p498, p499);
    {
        let mut m497 = m497;
        loop {
            let step = {
                let (v0, v1) = m497;
                let m496 = v0;
                match m496.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: List<u64>) -> bool {
    let m500 = v0.clone();
    match m500.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a504 = if f214(v0.clone(), false) {
                let a502 = {
                    let a501 = v0;
                    length_list(a501)
                };
                let a503 = f215();
                nat_le(a502, a503)
            } else {
                false
            };
            bool_not(a504)
        }
    }
}

pub fn f175(v0: Adt35) -> Option<Adt91> {
    let m509 = f216(match v0.clone() {
        Adt35::C0(h505, _, _, _, _, _, _, _) => {
            h505
        }
    }, {
        let m508 = match v0.clone() {
            Adt35::C0(_, _, _, _, h507, _, _, _) => {
                h507
            }
        };
        match m508 {
            Adt5::C0(h506) => {
                h506
            }
        }
    });
    match m509 {
        None => {
            f217(match v0.clone() {
                Adt35::C0(h510, _, _, _, _, _, _, _) => {
                    h510
                }
            }, {
                let m513 = match v0 {
                    Adt35::C0(_, _, _, _, h512, _, _, _) => {
                        h512
                    }
                };
                match m513 {
                    Adt5::C0(h511) => {
                        h511
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f176(v0: Adt35, v1: Result<Adt120, Adt91>) -> R<Result<List<Adt12>, Adt91>> {
    let m514 = v1;
    match m514 {
        Ok(v3) => {
            let m515 = f218(v0)?;
            match m515 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt91>(f97(match v3 {
                        Adt120::C0(h516, _) => {
                            h516
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

pub fn f177(p523: u64, v1: Str, v2: Adt6, p524: List<Adt104>, p525: Adt120) -> R<Result<Adt120, Adt91>> {
    let m522 = (p523, (p524, p525));
    {
        let mut m522 = m522;
        loop {
            let step = {
                let (v0, (v3, v4)) = m522;
                let m517 = v0;
                if m517 == 0 {
                    Err::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>(Ok(Err::<Adt120, Adt91>(Adt91::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m517 - 1;
                    let m518 = v3;
                    match m518.uncons() {
                        None => {
                            Err::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>(Ok(Ok::<Adt120, Adt91>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m519 = f219(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m519 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>((v5, ({
                                        let (h520, _) = v9;
                                        h520
                                    }, {
                                        let (_, h521) = v9;
                                        h521
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
                Ok(next) => { m522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178() -> u64 {
    67108864u64
}

pub fn f179() -> Adt120 {
    Adt120::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f180(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    let m526 = f220(v1.clone());
    match m526 {
        None => {
            let m527 = f221(v1);
            match m527 {
                None => {
                    Ok(Adt121::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f222(v0, match v5 {
                        Adt5::C0(h528) => {
                            h528
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

pub fn f181(v0: Adt101, v1: List<Adt101>) -> R<Adt105> {
    let m529 = v0;
    match m529 {
        Adt101::C0(_, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
        Adt101::C1(_, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
        Adt101::C2(v8, v9, v10, v11) => {
            Ok(f223(v8, v9, v10, {
                let a530 = v11;
                let a531 = 1u64;
                nat_add(a530, a531)?
            }, v1))
        }
        Adt101::C3(v12) => {
            Ok(Adt105::C0(Adt100::C1(v12, Str::lit("not")), v1))
        }
        Adt101::C4(v13, _, v15, v16) => {
            Ok(f224(v13, v15, v16, v1))
        }
        Adt101::C5(_, _, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
    }
}

pub fn f182(v0: Adt102, v1: Adt6, v2: Adt6, v3: List<Adt99>) -> Adt121 {
    let m532 = f221(v1);
    match m532 {
        None => {
            Adt121::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m533 = v0;
            match m533 {
                Adt102::C0 => {
                    f225(match v4 {
                        Adt5::C0(h534) => {
                            h534
                        }
                    }, v2, v3)
                }
                Adt102::C1 => {
                    f226(match v4 {
                        Adt5::C0(h535) => {
                            h535
                        }
                    }, v2, v3)
                }
                Adt102::C2 => {
                    f227(match v4 {
                        Adt5::C0(h536) => {
                            h536
                        }
                    }, v2, v3)
                }
                Adt102::C3 => {
                    f228(match v4 {
                        Adt5::C0(h537) => {
                            h537
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f183(v0: Adt101, v1: List<Adt101>, v2: List<Adt99>, v3: Str) -> Adt105 {
    let m538 = v0;
    match m538 {
        Adt101::C0(_, _, _) => {
            Adt105::C0(Adt100::C1(v2, v3), v1)
        }
        Adt101::C1(v7, v8, v9) => {
            f229(v7, v8, v9, v1)
        }
        Adt101::C2(v10, v11, v12, v13) => {
            f223(v10, v11, v12, v13, v1)
        }
        Adt101::C3(_) => {
            Adt105::C0(Adt100::C0, v1)
        }
        Adt101::C4(_, v16, v17, v18) => {
            f224(v16, v17, v18, v1)
        }
        Adt101::C5(_, _, _, _) => {
            Adt105::C0(Adt100::C1(v2, v3), v1)
        }
    }
}

pub fn f184(p547: List<Adt38>) -> R<bool> {
    let m546 = p547;
    {
        let mut m546 = m546;
        loop {
            let step = {
                let v0 = m546;
                let m539 = v0;
                match m539.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m540 = v1;
                        match m540 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f230(match v4 {
                                    Adt42::C0(h541, _, _, _) => {
                                        h541
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f230(match v5 {
                                    Adt43::C0(h542, _, _, _) => {
                                        h542
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
                                let m544 = match v7 {
                                    Adt45::C0(h543, _, _) => {
                                        h543
                                    }
                                };
                                match m544 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f230(match v9 {
                                            Adt48::C0(_, _, h545, _) => {
                                                h545
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
                Ok(next) => { m546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f231(match v1.clone() {
        Adt49::C0(h548, _) => {
            h548
        }
    }) {
        f232(v1)
    } else {
        let m550 = match v0 {
            Adt35::C0(_, _, _, _, _, h549, _, _) => {
                h549
            }
        };
        match m550 {
            None => {
                f232(v1)
            }
            Some(v2) => {
                f233(v2, v1)
            }
        }
    }
}

pub fn f186(p553: List<Adt99>, p554: List<Str>) -> R<List<Str>> {
    let m552 = (p553, p554);
    {
        let mut m552 = m552;
        loop {
            let step = {
                let (v0, v1) = m552;
                let m551 = v0;
                match m551.uncons() {
                    None => {
                        Err::<(List<Adt99>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt99>, List<Str>), R<List<Str>>>((v3, List::cons(f234(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: i64, v1: i64) -> R<i64> {
    let a556 = v0;
    let a557 = {
        let a555 = v1;
        int_neg(a555)?
    };
    int_sub(a556, a557)
}

pub fn f188(p562: u64, p563: List<Adt106>, p564: List<u64>) -> R<Result<List<u64>, Adt93>> {
    let m561 = (p562, (p563, p564));
    {
        let mut m561 = m561;
        loop {
            let step = {
                let (v0, (v1, v2)) = m561;
                let m558 = v0;
                if m558 == 0 {
                    Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Err::<List<u64>, Adt93>(Adt93::C13)))
                } else {
                    let v3 = m558 - 1;
                    let m559 = v1;
                    match m559.uncons() {
                        None => {
                            Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Ok::<List<u64>, Adt93>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m560 = f235(v4, v5, v2)?;
                            match m560 {
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
                Ok(next) => { m561 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(p567: List<u64>, p568: List<u64>) -> List<u64> {
    let m566 = (p567, p568);
    {
        let mut m566 = m566;
        loop {
            let step = {
                let (v0, v1) = m566;
                let m565 = v0;
                match m565.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m566 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Adt92) -> Adt6 {
    let m569 = v0;
    match m569 {
        Adt92::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt92::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f191(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m570 = v0;
    match m570 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f192() -> i64 {
    -32700i64
}

pub fn f193() -> i64 {
    -32600i64
}

pub fn f194(v0: Adt95) -> Str {
    let m571 = v0;
    match m571 {
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

pub fn f195(v0: bool, v1: Adt122) -> R<Adt88> {
    let m573 = match v1.clone() {
        Adt122::C0(_, h572) => {
            h572
        }
    };
    match m573 {
        Ok(v2) => {
            let m574 = f236(v2.clone())?;
            match m574 {
                None => {
                    Ok(f6())
                }
                Some(v3) => {
                    Ok(f237(v0, f82(f238(), f239(match v1 {
                        Adt122::C0(h575, _) => {
                            h575
                        }
                    }, v3))?, f240(match v2 {
                        Adt123::C0(_, _, h576) => {
                            h576
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f82(f241(v4.clone()), f84(Some(match v1 {
                Adt122::C0(h577, _) => {
                    h577
                }
            }), f242(v4)?))
        }
    }
}

pub fn f196(p582: List<Adt86>) -> R<bool> {
    let m581 = p582;
    {
        let mut m581 = m581;
        loop {
            let step = {
                let v0 = m581;
                let m578 = v0;
                match m578.uncons() {
                    None => {
                        Err::<List<Adt86>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f163(match v1.clone() {
                            Adt86::C0(h579, _) => {
                                h579
                            }
                        }, f243())? {
                            if f244(f245(44u64, match v1 {
                                Adt86::C0(_, h580) => {
                                    h580
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
                Ok(next) => { m581 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt107>, v4: List<Str>, v5: Adt103) -> R<Adt122> {
    Ok(Adt122::C0(match v5.clone() {
        Adt103::C0(h583, _, _) => {
            h583
        }
    }, f246(v0, v1, v2, f247(v3, v4, v5)?)?))
}

pub fn f198(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt107> {
    let v4: Adt128 = f248(Adt109::C0, f90(v0.clone(), f249())?);
    let v5: Adt128 = f248(Adt109::C1, f90(v0.clone(), f250())?);
    let v6: Adt128 = if f251(v3) {
        f248(Adt109::C2, f252(f90(v0.clone(), f253())?)?)
    } else {
        f254()
    };
    Ok(Adt107::C0(match v4.clone() {
        Adt128::C0(h584, _) => {
            h584
        }
    }, match v5.clone() {
        Adt128::C0(h585, _) => {
            h585
        }
    }, match v6.clone() {
        Adt128::C0(h586, _) => {
            h586
        }
    }, f255(match v4 {
        Adt128::C0(_, h587) => {
            h587
        }
    }, f255(match v5 {
        Adt128::C0(_, h588) => {
            h588
        }
    }, match v6 {
        Adt128::C0(_, h589) => {
            h589
        }
    })), f256(v0, v1, v2)?))
}

pub fn f199(v0: Adt103) -> Option<Str> {
    let a591 = match v0.clone() {
        Adt103::C0(_, h590, _) => {
            h590
        }
    };
    let a592 = Str::lit("tools/call");
    if equal(a591, a592) {
        let m594 = match v0 {
            Adt103::C0(_, _, h593) => {
                h593
            }
        };
        match m594 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m595 = f202(Str::lit("name"), v1);
                match m595 {
                    Adt130::C0 => {
                        None::<Str>
                    }
                    Adt130::C1(v2) => {
                        f257(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f200() -> List<Str> {
    List::cons(f258(), List::<Str>::nil())
}

pub fn f201(v0: Adt5) -> R<Adt129> {
    let m596 = f202(Str::lit("id"), v0);
    match m596 {
        Adt130::C0 => {
            Ok(Adt129::C0)
        }
        Adt130::C1(v1) => {
            let m597 = v1;
            match m597 {
                Adt6::C0 => {
                    Ok(Adt129::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt129::C2)
                }
                Adt6::C2(v3) => {
                    let m598 = f259(v3)?;
                    match m598 {
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

pub fn f202(v0: Str, v1: Adt5) -> Adt130 {
    f260(v0, match v1 {
        Adt5::C0(h599) => {
            h599
        }
    })
}

pub fn f203(v0: Adt5) -> Adt95 {
    if f204(v0) {
        Adt95::C8
    } else {
        Adt95::C3
    }
}

pub fn f204(v0: Adt5) -> bool {
    if f261(Str::lit("result"), v0.clone()) {
        true
    } else {
        f261(Str::lit("error"), v0)
    }
}

pub fn f205(v0: Adt92, v1: Adt5) -> Result<Adt103, Adt94> {
    let m600 = f202(Str::lit("jsonrpc"), v1.clone());
    match m600 {
        Adt130::C0 => {
            f159(Some(v0), Adt95::C2)
        }
        Adt130::C1(v2) => {
            let m601 = f257(v2);
            match m601 {
                None => {
                    f159(Some(v0), Adt95::C2)
                }
                Some(v3) => {
                    let a602 = v3;
                    let a603 = f150();
                    if equal(a602, a603) {
                        f262(v0, v1)
                    } else {
                        f159(Some(v0), Adt95::C2)
                    }
                }
            }
        }
    }
}

pub fn f206(v0: u64, v1: u64) -> Adt112 {
    let a604 = v1;
    let a605 = 0u64;
    if nat_eq(a604, a605) {
        Adt112::C0
    } else {
        let a606 = v1;
        let a607 = 2u64;
        if nat_eq(a606, a607) {
            Adt112::C2(v0)
        } else {
            Adt112::C1({
                let a611 = v0;
                let a612 = {
                    let a608 = v1;
                    let a609 = 3u64;
                    let a610 = 0u64;
                    nat_quot(a608, a609, a610)
                };
                nat_sub(a611, a612)
            })
        }
    }
}

pub fn f207(p640: Adt113, p641: u64, p642: u64, p643: List<u64>) -> R<u64> {
    let m639 = (p640, (p641, (p642, p643)));
    {
        let mut m639 = m639;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m639;
                let m613 = v3;
                match m613.uncons() {
                    None => {
                        let m614 = v0;
                        match m614 {
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
                        let m615 = v0;
                        match m615 {
                            Adt113::C0 => {
                                let a616 = v4;
                                let a617 = 128u64;
                                if nat_lt(a616, a617) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a618 = v4;
                                    let a619 = 194u64;
                                    if nat_lt(a618, a619) {
                                        let v6: u64 = f263({
                                            let a620 = List::cons(v4, v5);
                                            length_list(a620)
                                        })?;
                                        Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a621 = v4;
                                        let a622 = 224u64;
                                        if nat_lt(a621, a622) {
                                            Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a623 = v4;
                                            let a624 = 224u64;
                                            if nat_eq(a623, a624) {
                                                Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a625 = v4;
                                                let a626 = 237u64;
                                                if nat_eq(a625, a626) {
                                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a627 = v4;
                                                    let a628 = 240u64;
                                                    if nat_lt(a627, a628) {
                                                        Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a629 = v4;
                                                        let a630 = 240u64;
                                                        if nat_eq(a629, a630) {
                                                            Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a631 = v4;
                                                            let a632 = 244u64;
                                                            if nat_eq(a631, a632) {
                                                                Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a633 = v4;
                                                                let a634 = 244u64;
                                                                if nat_lt(a633, a634) {
                                                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f263({
                                                                        let a635 = List::cons(v4, v5);
                                                                        length_list(a635)
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
                                if f264(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f263({
                                        let a636 = List::cons(v4, v5);
                                        length_list(a636)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt113::C2 => {
                                if f264(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f263({
                                        let a637 = List::cons(v4, v5);
                                        length_list(a637)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt113::C3 => {
                                if f264(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f263({
                                        let a638 = List::cons(v4, v5);
                                        length_list(a638)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Adt114, v1: u64) -> R<Result<Adt6, Adt93>> {
    let m645 = match v0.clone() {
        Adt114::C0(h644, _, _) => {
            h644
        }
    };
    match m645 {
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
            let m646 = f265(v3)?;
            match m646 {
                Adt135::C0 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C7(v1)))
                }
                Adt135::C1 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C8(v1)))
                }
                Adt135::C2(v4) => {
                    let m651 = {
                        let m650 = f266(v4, match v0.clone() {
                            Adt114::C0(_, h648, _) => {
                                h648
                            }
                        }, match v0 {
                            Adt114::C0(_, _, h649) => {
                                h649
                            }
                        });
                        match m650 {
                            Adt114::C0(h647, _, _) => {
                                h647
                            }
                        }
                    };
                    match m651 {
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

pub fn f209(v0: Adt114, v1: u64, v2: u64) -> R<Adt131> {
    let m653 = match v0.clone() {
        Adt114::C0(h652, _, _) => {
            h652
        }
    };
    match m653 {
        Adt115::C0 => {
            if f267(v2) {
                Ok(Adt131::C1(v0))
            } else {
                f268(v0, v1, v2)
            }
        }
        Adt115::C1 => {
            if f267(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a654 = v2;
                let a655 = 93u64;
                if nat_eq(a654, a655) {
                    Ok(f269(v0, v1))
                } else {
                    f268(v0, v1, v2)
                }
            }
        }
        Adt115::C2 => {
            Ok(f270(v0, v1, v2))
        }
        Adt115::C3 => {
            if f267(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a656 = v2;
                let a657 = 125u64;
                if nat_eq(a656, a657) {
                    Ok(f271(v0, v1))
                } else {
                    let a658 = v2;
                    let a659 = 34u64;
                    if nat_eq(a658, a659) {
                        Ok(f272(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f273(Adt93::C2(v1)))
                    }
                }
            }
        }
        Adt115::C4 => {
            if f267(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a660 = v2;
                let a661 = 34u64;
                if nat_eq(a660, a661) {
                    Ok(f272(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f273(Adt93::C2(v1)))
                }
            }
        }
        Adt115::C5 => {
            if f267(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a662 = v2;
                let a663 = 58u64;
                if nat_eq(a662, a663) {
                    Ok(f272(v0, Adt115::C0))
                } else {
                    Ok(f273(Adt93::C2(v1)))
                }
            }
        }
        Adt115::C6(v3, v4, v5) => {
            f274(v0, v1, v2, v3, v4, v5)
        }
        Adt115::C7(v6, v7) => {
            f275(v0, v1, v2, v6, v7)
        }
        Adt115::C8(v8) => {
            f276(v0, v1, v2, v8)
        }
        Adt115::C9(_) => {
            Ok(f270(v0, v1, v2))
        }
    }
}

pub fn f210(v0: u64) -> R<u64> {
    let a664 = 64u64;
    let a665 = v0;
    if if nat_lt(a664, a665) {
        let a666 = v0;
        let a667 = 91u64;
        nat_lt(a666, a667)
    } else {
        false
    } {
        let a668 = v0;
        let a669 = 32u64;
        nat_add(a668, a669)
    } else {
        Ok(v0)
    }
}

pub fn f211(p672: List<u64>) -> List<u64> {
    let m671 = p672;
    {
        let mut m671 = m671;
        loop {
            let step = {
                let v0 = m671;
                let m670 = v0;
                match m670.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f277(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: u64) -> bool {
    let a673 = v0;
    let a674 = 9u64;
    if nat_eq(a673, a674) {
        true
    } else {
        let a675 = 31u64;
        let a676 = v0;
        if nat_lt(a675, a676) {
            let a677 = v0;
            let a678 = 127u64;
            nat_lt(a677, a678)
        } else {
            false
        }
    }
}

pub fn f213(p681: List<u64>) -> bool {
    let m680 = p681;
    {
        let mut m680 = m680;
        loop {
            let step = {
                let v0 = m680;
                let m679 = v0;
                match m679.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f278(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f214(p684: List<u64>, v1: bool) -> bool {
    let m683 = p684;
    {
        let mut m683 = m683;
        loop {
            let step = {
                let v0 = m683;
                let m682 = v0;
                match m682.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f279(v2)
                        } else {
                            f280(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215() -> u64 {
    64u64
}

pub fn f216(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m685 = f260(Str::lit("type"), v1);
    match m685 {
        Adt130::C0 => {
            Some(Adt91::C3(v0))
        }
        Adt130::C1(v2) => {
            let m686 = f257(v2);
            match m686 {
                None => {
                    Some(Adt91::C3(v0))
                }
                Some(v3) => {
                    let a687 = v3;
                    let a688 = Str::lit("object");
                    if equal(a687, a688) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f217(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m689 = f260(Str::lit("$schema"), v1);
    match m689 {
        Adt130::C0 => {
            None::<Adt91>
        }
        Adt130::C1(v2) => {
            let m690 = f257(v2);
            match m690 {
                None => {
                    Some(Adt91::C2(v0))
                }
                Some(v3) => {
                    let a691 = v3;
                    let a692 = f281();
                    if equal(a691, a692) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f218(v0: Adt35) -> R<Option<Adt91>> {
    let m694 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h693, _, _) => {
            h693
        }
    };
    match m694 {
        None => {
            Ok(None::<Adt91>)
        }
        Some(v1) => {
            let m697 = f217(match v0.clone() {
                Adt35::C0(h695, _, _, _, _, _, _, _) => {
                    h695
                }
            }, match v1.clone() {
                Adt5::C0(h696) => {
                    h696
                }
            });
            match m697 {
                None => {
                    let m701 = f177(f178(), match v0 {
                        Adt35::C0(h698, _, _, _, _, _, _, _) => {
                            h698
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h699) => {
                            h699
                        }
                    }), List::cons(Adt104::C0(Adt6::C5(match v1 {
                        Adt5::C0(h700) => {
                            h700
                        }
                    }), false, List::<Str>::nil()), List::<Adt104>::nil()), f179())?;
                    match m701 {
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

pub fn f219(v0: Str, v1: Adt6, v2: Adt104, v3: Adt120, v4: List<Adt104>) -> R<Result<(List<Adt104>, Adt120), Adt91>> {
    let m702 = v2;
    match m702 {
        Adt104::C0(v5, v6, v7) => {
            let m703 = f220(v5.clone());
            match m703 {
                None => {
                    let m704 = f221(v5);
                    match m704 {
                        None => {
                            Ok(Err::<(List<Adt104>, Adt120), Adt91>(Adt91::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f282(v0, v1, match v9 {
                                Adt5::C0(h705) => {
                                    h705
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

pub fn f220(v0: Adt6) -> Option<bool> {
    let m706 = v0;
    match m706 {
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

pub fn f221(v0: Adt6) -> Option<Adt5> {
    let m707 = v0;
    match m707 {
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

pub fn f222(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    let m708 = f283(v1.clone(), v2.clone())?;
    match m708 {
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
            f284(v0, v1, v2, v3)
        }
    }
}

pub fn f223(v0: List<Adt6>, v1: Adt6, v2: List<Adt99>, v3: u64, v4: List<Adt101>) -> Adt105 {
    let m709 = v0;
    match m709.uncons() {
        None => {
            let a710 = v3;
            let a711 = 1u64;
            if nat_eq(a710, a711) {
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

pub fn f224(v0: Option<Adt6>, v1: Adt6, v2: List<Adt99>, v3: List<Adt101>) -> Adt105 {
    let m712 = v0;
    match m712 {
        None => {
            Adt105::C0(Adt100::C0, v3)
        }
        Some(v4) => {
            Adt105::C0(Adt100::C0, List::cons(Adt101::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f225(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m713 = f260(Str::lit("anyOf"), v0);
    match m713 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m714 = f285(v3);
            match m714 {
                None => {
                    Adt121::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m715 = v4;
                    match m715.uncons() {
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

pub fn f226(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m716 = f260(Str::lit("oneOf"), v0);
    match m716 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m717 = f285(v3);
            match m717 {
                None => {
                    Adt121::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m718 = v4;
                    match m718.uncons() {
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

pub fn f227(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m719 = f260(Str::lit("not"), v0);
    match m719 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            Adt121::C3(List::cons(Adt101::C0(v3, v1, v2.clone()), List::cons(Adt101::C3(v2), List::<Adt101>::nil())))
        }
    }
}

pub fn f228(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m720 = f260(Str::lit("if"), v0.clone());
    match m720 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            Adt121::C3(List::cons(Adt101::C0(v3, v1.clone(), v2.clone()), List::cons(Adt101::C4(f286(Str::lit("then"), v0.clone()), f286(Str::lit("else"), v0), v1, v2), List::<Adt101>::nil())))
        }
    }
}

pub fn f229(v0: List<Adt6>, v1: Adt6, v2: List<Adt99>, v3: List<Adt101>) -> Adt105 {
    let m721 = v0;
    match m721.uncons() {
        None => {
            Adt105::C0(Adt100::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt105::C0(Adt100::C0, List::cons(Adt101::C0(v4, v1.clone(), v2.clone()), List::cons(Adt101::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f230(v0: Str) -> R<bool> {
    let m723 = f287(f110({
        let a722 = v0;
        utf8_encode(a722)
    })?)?;
    match m723 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f231(v0: Adt37) -> bool {
    let m725 = match v0 {
        Adt37::C0(_, _, h724) => {
            h724
        }
    };
    match m725 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f232(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m727 = match v0.clone() {
        Adt49::C0(_, h726) => {
            h726
        }
    };
    match m727.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h728, _) => {
                    h728
                }
            })))
        }
        Some((_, _)) => {
            if f288(match v0.clone() {
                Adt49::C0(_, h729) => {
                    h729
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f233(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m733 = {
        let m732 = match v1.clone() {
            Adt49::C0(h731, _) => {
                h731
            }
        };
        match m732 {
            Adt37::C0(_, h730, _) => {
                h730
            }
        }
    };
    match m733 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m736 = f100(Adt6::C5(match v0.clone() {
                Adt5::C0(h734) => {
                    h734
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h735) => {
                    h735
                }
            }), v2)?;
            match m736 {
                Adt98::C0 => {
                    f232(v1)
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

pub fn f234(v0: Adt99) -> R<Str> {
    let m737 = v0;
    match m737 {
        Adt99::C0(v1) => {
            f289(v1)
        }
        Adt99::C1(v2) => {
            let a738 = f143(v2, 0i64)?;
            Ok(format_i64(a738))
        }
    }
}

pub fn f235(v0: Adt106, v1: List<Adt106>, v2: List<u64>) -> R<Adt132> {
    let m739 = v0;
    match m739 {
        Adt106::C0(v3) => {
            Ok(Adt132::C1(v1, f189(v3, v2)))
        }
        Adt106::C1(v4, v5) => {
            f290(v4, v5, v1, v2)
        }
        Adt106::C2(v6, v7) => {
            let m740 = v7;
            match m740.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C1(v6, v8), List::cons(Adt106::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt106::C3(v10, v11) => {
            let m741 = v11.clone();
            match m741.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt106::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt106::C4(v14, v15) => {
            let m742 = v15;
            match m742.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C0({
                        let a744 = f291({
                            let (h743, _) = v16;
                            h743
                        })?;
                        let a745 = List::cons(58u64, List::<u64>::nil());
                        append_list(a744, a745)
                    }), List::cons(Adt106::C1(v14, {
                        let (_, h746) = v16;
                        h746
                    }), List::cons(Adt106::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt106::C5(v18, v19) => {
            let m747 = v19.clone();
            match m747.uncons() {
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

pub fn f236(v0: Adt123) -> R<Option<Adt5>> {
    let m749 = f292(match v0.clone() {
        Adt123::C0(h748, _, _) => {
            h748
        }
    })?;
    match m749 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a752 = v1;
                let a753 = {
                    let m751 = f293(v0);
                    match m751 {
                        Adt5::C0(h750) => {
                            h750
                        }
                    }
                };
                append_list(a752, a753)
            })))
        }
    }
}

pub fn f237(v0: bool, v1: Adt88, v2: Option<List<List<u64>>>) -> Adt88 {
    let m754 = v2;
    match m754 {
        None => {
            f6()
        }
        Some(v3) => {
            f294(v0, v1, f45(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f238() -> u64 {
    200u64
}

pub fn f239(v0: Adt92, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f150())), List::cons((Str::lit("id"), f190(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h755) => {
            h755
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f240(p759: List<Adt125>, p760: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m758 = (p759, p760);
    {
        let mut m758 = m758;
        loop {
            let step = {
                let (v0, v1) = m758;
                let m756 = v0;
                match m756.uncons() {
                    None => {
                        Err::<(List<Adt125>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m757 = f295(v2)?;
                        match m757 {
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
                Ok(next) => { m758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Adt126) -> u64 {
    let m761 = v0;
    match m761 {
        Adt126::C0(_) => {
            f119()
        }
        Adt126::C1(_, _) => {
            f119()
        }
        Adt126::C2(_) => {
            f119()
        }
        Adt126::C3(_) => {
            f296()
        }
        Adt126::C4(v6) => {
            f297(v6)
        }
    }
}

pub fn f242(v0: Adt126) -> R<Adt80> {
    Ok(Adt80::C0(f298(v0.clone()), f299(v0.clone()), f300(v0)?))
}

pub fn f243() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f244(p764: List<List<u64>>) -> R<bool> {
    let m763 = p764;
    {
        let mut m763 = m763;
        loop {
            let step = {
                let v0 = m763;
                let m762 = v0;
                match m762.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f301(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m763 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: u64, p769: List<u64>, p770: List<u64>, p771: List<List<u64>>) -> List<List<u64>> {
    let m768 = (p769, (p770, p771));
    {
        let mut m768 = m768;
        loop {
            let step = {
                let (v1, (v2, v3)) = m768;
                let m765 = v1;
                match m765.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f45(List::cons(f173(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a766 = v4;
                        let a767 = v0;
                        if nat_eq(a766, a767) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f173(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt133, Adt126>) -> R<Result<Adt123, Adt126>> {
    let m772 = v3;
    match m772 {
        Ok(v5) => {
            f302(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt123, Adt126>(v4))
        }
    }
}

pub fn f247(v0: Option<Adt107>, v1: List<Str>, v2: Adt103) -> R<Result<Adt133, Adt126>> {
    Ok(f303(f304(match v2.clone() {
        Adt103::C0(_, h773, _) => {
            h773
        }
    }), f305(v0, v1, v2)?))
}

pub fn f248(v0: Adt109, v1: Adt96) -> Adt128 {
    let m774 = v1;
    match m774 {
        Adt96::C0 => {
            f254()
        }
        Adt96::C1 => {
            Adt128::C0(None::<Str>, Some(Adt108::C6(v0)))
        }
        Adt96::C2 => {
            Adt128::C0(None::<Str>, Some(Adt108::C7(v0)))
        }
        Adt96::C3(v2) => {
            let m775 = f306(v2);
            match m775 {
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

pub fn f249() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f250() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f251(v0: Str) -> bool {
    let m776 = f307(v0);
    match m776 {
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

pub fn f252(v0: Adt96) -> R<Adt96> {
    let m777 = v0.clone();
    match m777 {
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
            let m778 = f308(v1);
            match m778 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m779 = f287(v2)?;
                    match m779 {
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

pub fn f253() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f254() -> Adt128 {
    Adt128::C0(None::<Str>, None::<Adt108>)
}

pub fn f255(v0: Option<Adt108>, v1: Option<Adt108>) -> Option<Adt108> {
    let m780 = v0.clone();
    match m780 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f256(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt110>> {
    Ok(f309(f310(v0, v1, v2, List::<Adt110>::nil())?, List::<Adt110>::nil()))
}

pub fn f257(v0: Adt6) -> Option<Str> {
    let m781 = v0;
    match m781 {
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

pub fn f258() -> Str {
    Str::lit("2026-07-28")
}

pub fn f259(v0: Adt7) -> R<Option<i64>> {
    let m784 = f311(40u64, match v0.clone() {
        Adt7::C0(h782, _) => {
            h782
        }
    }, match v0 {
        Adt7::C0(_, h783) => {
            h783
        }
    })?;
    match m784 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f312(v1))
        }
    }
}

pub fn f260(v0: Str, p790: List<(Str, Adt6)>) -> Adt130 {
    let m789 = p790;
    {
        let mut m789 = m789;
        loop {
            let step = {
                let v1 = m789;
                let m785 = v1;
                match m785.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt130>(Adt130::C0)
                    }
                    Some((v2, v3)) => {
                        let m787 = f313({
                            let (h786, _) = v2;
                            h786
                        }, v0.clone());
                        match m787 {
                            Adt141::C0 => {
                                Err::<List<(Str, Adt6)>, Adt130>(Adt130::C1({
                                    let (_, h788) = v2;
                                    h788
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
                Ok(next) => { m789 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: Str, v1: Adt5) -> bool {
    f314(f202(v0, v1))
}

pub fn f262(v0: Adt92, v1: Adt5) -> Result<Adt103, Adt94> {
    let m791 = f202(Str::lit("method"), v1.clone());
    match m791 {
        Adt130::C0 => {
            f159(Some(v0), f203(v1))
        }
        Adt130::C1(v2) => {
            let m792 = f257(v2);
            match m792 {
                None => {
                    f159(Some(v0), Adt95::C4)
                }
                Some(v3) => {
                    f315(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f263(v0: u64) -> R<u64> {
    let a795 = {
        let a793 = v0;
        let a794 = 3u64;
        nat_mul(a793, a794)?
    };
    let a796 = 1u64;
    nat_add(a795, a796)
}

pub fn f264(v0: u64, v1: u64, v2: u64) -> bool {
    let a797 = v1;
    let a798 = v0;
    if nat_le(a797, a798) {
        let a799 = v0;
        let a800 = v2;
        nat_le(a799, a800)
    } else {
        false
    }
}

pub fn f265(v0: Adt117) -> R<Adt135> {
    if f316(match v0.clone() {
        Adt117::C0(_, h801, _, _, _, _, _) => {
            h801
        }
    }) {
        let v1: i64 = {
            let a807 = if match v0.clone() {
                Adt117::C0(_, _, _, _, _, h802, _) => {
                    h802
                }
            } {
                let a804 = match v0.clone() {
                    Adt117::C0(_, _, _, _, _, _, h803) => {
                        h803
                    }
                };
                int_neg(a804)?
            } else {
                match v0.clone() {
                    Adt117::C0(_, _, _, _, _, _, h805) => {
                        h805
                    }
                }
            };
            let a808 = match v0.clone() {
                Adt117::C0(_, _, _, _, h806, _, _) => {
                    h806
                }
            };
            int_sub(a807, a808)?
        };
        let m811 = {
            let a809 = v1;
            let a810 = -9999i64;
            compare(a809, a810)
        };
        if if match m811 {
            Ordering::Less => {
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
            let m814 = {
                let a812 = 9999i64;
                let a813 = v1;
                compare(a812, a813)
            };
            match m814 {
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
                Adt117::C0(h815, _, _, _, _, _, _) => {
                    h815
                }
            } {
                let a817 = match v0 {
                    Adt117::C0(_, _, h816, _, _, _, _) => {
                        h816
                    }
                };
                int_neg(a817)?
            } else {
                match v0 {
                    Adt117::C0(_, _, h818, _, _, _, _) => {
                        h818
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt135::C0)
    }
}

pub fn f266(v0: Adt6, v1: List<Adt119>, v2: u64) -> Adt114 {
    let m819 = v1.clone();
    match m819.uncons() {
        None => {
            Adt114::C0(Adt115::C9(v0), List::<Adt119>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m820 = v3;
            match m820 {
                Adt119::C0(v5) => {
                    Adt114::C0(Adt115::C2, List::cons(Adt119::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt119::C1(v6, v7) => {
                    let m821 = v7;
                    match m821 {
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

pub fn f267(v0: u64) -> bool {
    let a822 = v0;
    let a823 = 32u64;
    if nat_eq(a822, a823) {
        true
    } else {
        let a824 = v0;
        let a825 = 10u64;
        if nat_eq(a824, a825) {
            true
        } else {
            let a826 = v0;
            let a827 = 13u64;
            if nat_eq(a826, a827) {
                true
            } else {
                let a828 = v0;
                let a829 = 9u64;
                nat_eq(a828, a829)
            }
        }
    }
}

pub fn f268(v0: Adt114, v1: u64, v2: u64) -> R<Adt131> {
    let a830 = v2;
    let a831 = 34u64;
    if nat_eq(a830, a831) {
        Ok(f272(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), false)))
    } else {
        let a832 = v2;
        let a833 = 123u64;
        if nat_eq(a832, a833) {
            f317(v0, Adt119::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt115::C3, v1)
        } else {
            let a834 = v2;
            let a835 = 91u64;
            if nat_eq(a834, a835) {
                f317(v0, Adt119::C0(List::<Adt6>::nil()), Adt115::C1, v1)
            } else {
                let a836 = v2;
                let a837 = 116u64;
                if nat_eq(a836, a837) {
                    Ok(f272(v0, Adt115::C7(0u64, 1u64)))
                } else {
                    let a838 = v2;
                    let a839 = 102u64;
                    if nat_eq(a838, a839) {
                        Ok(f272(v0, Adt115::C7(1u64, 1u64)))
                    } else {
                        let a840 = v2;
                        let a841 = 110u64;
                        if nat_eq(a840, a841) {
                            Ok(f272(v0, Adt115::C7(2u64, 1u64)))
                        } else {
                            let a842 = v2;
                            let a843 = 45u64;
                            if nat_eq(a842, a843) {
                                Ok(f272(v0, f318(true, Adt118::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a844 = v2;
                                let a845 = 48u64;
                                if nat_eq(a844, a845) {
                                    Ok(f272(v0, f318(false, Adt118::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f264(v2, 49u64, 57u64) {
                                        Ok(f272(v0, f318(false, Adt118::C2, f319({
                                            let a846 = v2;
                                            let a847 = 48u64;
                                            nat_sub(a846, a847)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f273(Adt93::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f269(v0: Adt114, v1: u64) -> Adt131 {
    let m849 = match v0.clone() {
        Adt114::C0(_, h848, _) => {
            h848
        }
    };
    match m849.uncons() {
        None => {
            f273(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m850 = v2;
            match m850 {
                Adt119::C0(v4) => {
                    Adt131::C1(f266(Adt6::C4(f320(v4, List::<Adt6>::nil())), v3, {
                        let a852 = match v0 {
                            Adt114::C0(_, _, h851) => {
                                h851
                            }
                        };
                        let a853 = 1u64;
                        nat_sub(a852, a853)
                    }))
                }
                Adt119::C1(_, _) => {
                    f273(Adt93::C2(v1))
                }
            }
        }
    }
}

pub fn f270(v0: Adt114, v1: u64, v2: u64) -> Adt131 {
    let m855 = match v0.clone() {
        Adt114::C0(h854, _, _) => {
            h854
        }
    };
    match m855 {
        Adt115::C0 => {
            f273(Adt93::C2(v1))
        }
        Adt115::C1 => {
            f273(Adt93::C2(v1))
        }
        Adt115::C2 => {
            if f267(v2) {
                Adt131::C1(v0)
            } else {
                let a856 = v2;
                let a857 = 44u64;
                if nat_eq(a856, a857) {
                    f321(v0, v1)
                } else {
                    let a858 = v2;
                    let a859 = 93u64;
                    if nat_eq(a858, a859) {
                        f269(v0, v1)
                    } else {
                        let a860 = v2;
                        let a861 = 125u64;
                        if nat_eq(a860, a861) {
                            f271(v0, v1)
                        } else {
                            f273(Adt93::C2(v1))
                        }
                    }
                }
            }
        }
        Adt115::C3 => {
            f273(Adt93::C2(v1))
        }
        Adt115::C4 => {
            f273(Adt93::C2(v1))
        }
        Adt115::C5 => {
            f273(Adt93::C2(v1))
        }
        Adt115::C6(_, _, _) => {
            f273(Adt93::C2(v1))
        }
        Adt115::C7(_, _) => {
            f273(Adt93::C2(v1))
        }
        Adt115::C8(_) => {
            f273(Adt93::C2(v1))
        }
        Adt115::C9(_) => {
            if f267(v2) {
                Adt131::C1(v0)
            } else {
                f273(Adt93::C10(v1))
            }
        }
    }
}

pub fn f271(v0: Adt114, v1: u64) -> Adt131 {
    let m863 = match v0.clone() {
        Adt114::C0(_, h862, _) => {
            h862
        }
    };
    match m863.uncons() {
        None => {
            f273(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m864 = v2;
            match m864 {
                Adt119::C0(_) => {
                    f273(Adt93::C2(v1))
                }
                Adt119::C1(v5, _) => {
                    Adt131::C1(f266(Adt6::C5(f322(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a866 = match v0 {
                            Adt114::C0(_, _, h865) => {
                                h865
                            }
                        };
                        let a867 = 1u64;
                        nat_sub(a866, a867)
                    }))
                }
            }
        }
    }
}

pub fn f272(v0: Adt114, v1: Adt115) -> Adt131 {
    Adt131::C1(f323(v0, v1))
}

pub fn f273(v0: Adt93) -> Adt131 {
    Adt131::C0(v0)
}

pub fn f274(v0: Adt114, v1: u64, v2: u64, v3: Adt116, v4: List<u64>, v5: bool) -> R<Adt131> {
    let m868 = v3;
    match m868 {
        Adt116::C0 => {
            let a869 = v2;
            let a870 = 34u64;
            if nat_eq(a869, a870) {
                Ok(f324(v0, v1, v4, v5))
            } else {
                let a871 = v2;
                let a872 = 92u64;
                if nat_eq(a871, a872) {
                    Ok(f272(v0, Adt115::C6(Adt116::C1, v4, v5)))
                } else {
                    let a873 = v2;
                    let a874 = 32u64;
                    if nat_lt(a873, a874) {
                        Ok(f273(Adt93::C4(v1)))
                    } else {
                        Ok(f272(v0, Adt115::C6(Adt116::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt116::C1 => {
            let a875 = v2;
            let a876 = 117u64;
            if nat_eq(a875, a876) {
                Ok(f272(v0, Adt115::C6(Adt116::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a877 = f325(v2);
                let a878 = 0u64;
                if nat_eq(a877, a878) {
                    Ok(f273(Adt93::C5(v1)))
                } else {
                    Ok(f272(v0, Adt115::C6(Adt116::C0, List::cons(f325(v2), v4), v5)))
                }
            }
        }
        Adt116::C2(v6, v7, v8) => {
            let a879 = 15u64;
            let a880 = f326(v2);
            if nat_lt(a879, a880) {
                Ok(f273(Adt93::C5(v1)))
            } else {
                let a883 = {
                    let a881 = v6;
                    let a882 = 1u64;
                    nat_add(a881, a882)?
                };
                let a884 = 4u64;
                if nat_eq(a883, a884) {
                    f327(v0, v1, {
                        let a887 = {
                            let a885 = v7;
                            let a886 = 16u64;
                            nat_mul(a885, a886)?
                        };
                        let a888 = f326(v2);
                        nat_add(a887, a888)?
                    }, v8, v4, v5)
                } else {
                    Ok(f272(v0, Adt115::C6(Adt116::C2({
                        let a889 = v6;
                        let a890 = 1u64;
                        nat_add(a889, a890)?
                    }, {
                        let a893 = {
                            let a891 = v7;
                            let a892 = 16u64;
                            nat_mul(a891, a892)?
                        };
                        let a894 = f326(v2);
                        nat_add(a893, a894)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt116::C3(v9) => {
            let a895 = v2;
            let a896 = 92u64;
            if nat_eq(a895, a896) {
                Ok(f272(v0, Adt115::C6(Adt116::C4(v9), v4, v5)))
            } else {
                Ok(f273(Adt93::C6(v1)))
            }
        }
        Adt116::C4(v10) => {
            let a897 = v2;
            let a898 = 117u64;
            if nat_eq(a897, a898) {
                Ok(f272(v0, Adt115::C6(Adt116::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f273(Adt93::C6(v1)))
            }
        }
    }
}

pub fn f275(v0: Adt114, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt131> {
    let a899 = v2;
    let a900 = f328(v3, v4);
    if nat_eq(a899, a900) {
        let a904 = {
            let a901 = v4;
            let a902 = 1u64;
            nat_add(a901, a902)?
        };
        let a905 = {
            let a903 = f329(v3);
            length_list(a903)
        };
        if nat_eq(a904, a905) {
            Ok(f330(v0, f331(v3)))
        } else {
            Ok(f272(v0, Adt115::C7(v3, {
                let a906 = v4;
                let a907 = 1u64;
                nat_add(a906, a907)?
            })))
        }
    } else {
        Ok(f273(Adt93::C2(v1)))
    }
}

pub fn f276(v0: Adt114, v1: u64, v2: u64, v3: Adt117) -> R<Adt131> {
    let m909 = match v3.clone() {
        Adt117::C0(_, h908, _, _, _, _, _) => {
            h908
        }
    };
    match m909 {
        Adt118::C0 => {
            let a910 = v2;
            let a911 = 48u64;
            if nat_eq(a910, a911) {
                Ok(f332(v0, v3, Adt118::C1))
            } else {
                if f264(v2, 49u64, 57u64) {
                    f333(v0, v1, v2, v3, false)
                } else {
                    Ok(f273(Adt93::C7(v1)))
                }
            }
        }
        Adt118::C1 => {
            let a912 = v2;
            let a913 = 46u64;
            if nat_eq(a912, a913) {
                Ok(f332(v0, v3, Adt118::C3))
            } else {
                if f334(v2) {
                    Ok(f332(v0, v3, Adt118::C5))
                } else {
                    if f335(v2) {
                        Ok(f273(Adt93::C7(v1)))
                    } else {
                        f336(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt118::C2 => {
            if f335(v2) {
                f333(v0, v1, v2, v3, false)
            } else {
                let a914 = v2;
                let a915 = 46u64;
                if nat_eq(a914, a915) {
                    Ok(f332(v0, v3, Adt118::C3))
                } else {
                    if f334(v2) {
                        Ok(f332(v0, v3, Adt118::C5))
                    } else {
                        f336(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt118::C3 => {
            if f335(v2) {
                f333(v0, v1, v2, v3, true)
            } else {
                Ok(f273(Adt93::C7(v1)))
            }
        }
        Adt118::C4 => {
            if f335(v2) {
                f333(v0, v1, v2, v3, true)
            } else {
                if f334(v2) {
                    Ok(f332(v0, v3, Adt118::C5))
                } else {
                    f336(v0, v1, v2, v3)
                }
            }
        }
        Adt118::C5 => {
            let a916 = v2;
            let a917 = 43u64;
            if nat_eq(a916, a917) {
                Ok(f272(v0, f318(match v3.clone() {
                    Adt117::C0(h918, _, _, _, _, _, _) => {
                        h918
                    }
                }, Adt118::C6, match v3.clone() {
                    Adt117::C0(_, _, h919, _, _, _, _) => {
                        h919
                    }
                }, match v3.clone() {
                    Adt117::C0(_, _, _, h920, _, _, _) => {
                        h920
                    }
                }, match v3 {
                    Adt117::C0(_, _, _, _, h921, _, _) => {
                        h921
                    }
                }, false, 0i64)))
            } else {
                let a922 = v2;
                let a923 = 45u64;
                if nat_eq(a922, a923) {
                    Ok(f272(v0, f318(match v3.clone() {
                        Adt117::C0(h924, _, _, _, _, _, _) => {
                            h924
                        }
                    }, Adt118::C6, match v3.clone() {
                        Adt117::C0(_, _, h925, _, _, _, _) => {
                            h925
                        }
                    }, match v3.clone() {
                        Adt117::C0(_, _, _, h926, _, _, _) => {
                            h926
                        }
                    }, match v3 {
                        Adt117::C0(_, _, _, _, h927, _, _) => {
                            h927
                        }
                    }, true, 0i64)))
                } else {
                    if f335(v2) {
                        f337(v0, v1, v2, Adt117::C0(match v3.clone() {
                            Adt117::C0(h928, _, _, _, _, _, _) => {
                                h928
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, h929, _, _, _, _, _) => {
                                h929
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, _, h930, _, _, _, _) => {
                                h930
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, _, _, h931, _, _, _) => {
                                h931
                            }
                        }, match v3 {
                            Adt117::C0(_, _, _, _, h932, _, _) => {
                                h932
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f273(Adt93::C7(v1)))
                    }
                }
            }
        }
        Adt118::C6 => {
            if f335(v2) {
                f337(v0, v1, v2, v3.clone(), match v3 {
                    Adt117::C0(_, _, _, _, _, h933, _) => {
                        h933
                    }
                })
            } else {
                Ok(f273(Adt93::C7(v1)))
            }
        }
        Adt118::C7 => {
            if f335(v2) {
                f337(v0, v1, v2, v3.clone(), match v3 {
                    Adt117::C0(_, _, _, _, _, h934, _) => {
                        h934
                    }
                })
            } else {
                f336(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f277(v0: u64) -> bool {
    let a935 = v0;
    let a936 = 9u64;
    if nat_eq(a935, a936) {
        true
    } else {
        let a937 = v0;
        let a938 = 32u64;
        nat_eq(a937, a938)
    }
}

pub fn f278(v0: u64) -> bool {
    let a939 = 47u64;
    let a940 = v0;
    if nat_lt(a939, a940) {
        let a941 = v0;
        let a942 = 58u64;
        nat_lt(a941, a942)
    } else {
        false
    }
}

pub fn f279(v0: u64) -> bool {
    let a943 = 48u64;
    let a944 = v0;
    if if nat_le(a943, a944) {
        let a945 = v0;
        let a946 = 57u64;
        nat_le(a945, a946)
    } else {
        false
    } {
        true
    } else {
        let a947 = 65u64;
        let a948 = v0;
        if if nat_le(a947, a948) {
            let a949 = v0;
            let a950 = 90u64;
            nat_le(a949, a950)
        } else {
            false
        } {
            true
        } else {
            let a951 = 97u64;
            let a952 = v0;
            if if nat_le(a951, a952) {
                let a953 = v0;
                let a954 = 122u64;
                nat_le(a953, a954)
            } else {
                false
            } {
                true
            } else {
                let a955 = v0;
                let a956 = 33u64;
                if nat_eq(a955, a956) {
                    true
                } else {
                    let a957 = v0;
                    let a958 = 35u64;
                    if nat_eq(a957, a958) {
                        true
                    } else {
                        let a959 = v0;
                        let a960 = 36u64;
                        if nat_eq(a959, a960) {
                            true
                        } else {
                            let a961 = v0;
                            let a962 = 37u64;
                            if nat_eq(a961, a962) {
                                true
                            } else {
                                let a963 = v0;
                                let a964 = 38u64;
                                if nat_eq(a963, a964) {
                                    true
                                } else {
                                    let a965 = v0;
                                    let a966 = 39u64;
                                    if nat_eq(a965, a966) {
                                        true
                                    } else {
                                        let a967 = v0;
                                        let a968 = 42u64;
                                        if nat_eq(a967, a968) {
                                            true
                                        } else {
                                            let a969 = v0;
                                            let a970 = 43u64;
                                            if nat_eq(a969, a970) {
                                                true
                                            } else {
                                                let a971 = v0;
                                                let a972 = 45u64;
                                                if nat_eq(a971, a972) {
                                                    true
                                                } else {
                                                    let a973 = v0;
                                                    let a974 = 46u64;
                                                    if nat_eq(a973, a974) {
                                                        true
                                                    } else {
                                                        let a975 = v0;
                                                        let a976 = 94u64;
                                                        if nat_eq(a975, a976) {
                                                            true
                                                        } else {
                                                            let a977 = v0;
                                                            let a978 = 95u64;
                                                            if nat_eq(a977, a978) {
                                                                true
                                                            } else {
                                                                let a979 = v0;
                                                                let a980 = 96u64;
                                                                if nat_eq(a979, a980) {
                                                                    true
                                                                } else {
                                                                    let a981 = v0;
                                                                    let a982 = 124u64;
                                                                    if nat_eq(a981, a982) {
                                                                        true
                                                                    } else {
                                                                        let a983 = v0;
                                                                        let a984 = 126u64;
                                                                        nat_eq(a983, a984)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f280(v0: u64) -> bool {
    let a985 = 48u64;
    let a986 = v0;
    if if nat_le(a985, a986) {
        let a987 = v0;
        let a988 = 57u64;
        nat_le(a987, a988)
    } else {
        false
    } {
        true
    } else {
        let a989 = 65u64;
        let a990 = v0;
        if if nat_le(a989, a990) {
            let a991 = v0;
            let a992 = 90u64;
            nat_le(a991, a992)
        } else {
            false
        } {
            true
        } else {
            let a993 = 97u64;
            let a994 = v0;
            if if nat_le(a993, a994) {
                let a995 = v0;
                let a996 = 122u64;
                nat_le(a995, a996)
            } else {
                false
            } {
                true
            } else {
                let a997 = v0;
                let a998 = 95u64;
                if nat_eq(a997, a998) {
                    true
                } else {
                    let a999 = v0;
                    let a1000 = 45u64;
                    if nat_eq(a999, a1000) {
                        true
                    } else {
                        let a1001 = v0;
                        let a1002 = 46u64;
                        nat_eq(a1001, a1002)
                    }
                }
            }
        }
    }
}

pub fn f281() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f282(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt120, v6: List<Adt104>) -> R<Result<(List<Adt104>, Adt120), Adt91>> {
    let m1003 = f338(v0.clone(), v1, v2.clone());
    match m1003 {
        None => {
            let m1004 = f339(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1004 {
                Adt142::C0(v8) => {
                    Ok(Err::<(List<Adt104>, Adt120), Adt91>(v8))
                }
                Adt142::C1(v9) => {
                    Ok(Ok::<(List<Adt104>, Adt120), Adt91>((f340(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt104>, Adt120), Adt91>(v7))
        }
    }
}

pub fn f283(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt121> {
    Ok(f341(List::cons((Str::lit("type"), f342(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f343(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f344(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f345(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f345(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f345(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f345(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1005 = f346(v0.clone(), v1.clone())?;
        let a1006 = List::cons((Str::lit("required"), f347(v0, v1)), List::<(Str, Adt136)>::nil());
        append_list(a1005, a1006)
    })))))))))
}

pub fn f284(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    Ok(f348(f349(v0, v1.clone(), v2.clone(), v3.clone()), f348(f350(v1.clone(), v2.clone(), v3.clone()), f348(f351(v1.clone(), v2.clone(), v3.clone()), f348(f352(v1.clone(), v2.clone(), v3.clone())?, f348(f353(v1.clone(), v2.clone(), v3.clone()), f348(f354(Adt102::C0, f225(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f348(f354(Adt102::C1, f226(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f348(f354(Adt102::C2, f227(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f354(Adt102::C3, f228(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f285(v0: Adt6) -> Option<List<Adt6>> {
    let m1007 = v0;
    match m1007 {
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

pub fn f286(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1008 = f260(v0, v1);
    match m1008 {
        Adt130::C0 => {
            None::<Adt6>
        }
        Adt130::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f287(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1010 = f355({
        let a1009 = v0.clone();
        length_list(a1009)
    }, v0, List::<u64>::nil())?;
    match m1010 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f189(v1, List::<u64>::nil())))
        }
    }
}

pub fn f288(v0: List<Adt50>) -> R<bool> {
    let m1011 = v0;
    match m1011.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f356(v2, match v1 {
                Adt50::C0(h1012, _, _) => {
                    h1012
                }
            })
        }
    }
}

pub fn f289(v0: Str) -> R<Str> {
    let m1015 = {
        let a1014 = f357(f358(), f189(f359(f110({
            let a1013 = v0.clone();
            utf8_encode(a1013)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1014)
    };
    match m1015 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f290(v0: u64, v1: Adt6, v2: List<Adt106>, v3: List<u64>) -> R<Adt132> {
    let m1016 = v1;
    match m1016 {
        Adt6::C0 => {
            Ok(Adt132::C1(v2, f189(f360(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt132::C1(v2, f189(if v4 {
                f360(0u64)
            } else {
                f360(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f361(v5.clone())? {
                Ok(Adt132::C1(v2, f189(f362(v5)?, v3)))
            } else {
                Ok(Adt132::C0(Adt93::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt132::C1(v2, f189(f291(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1017 = v0;
            let a1018 = f363();
            if nat_lt(a1017, a1018) {
                Ok(Adt132::C1(f364(91u64, Adt106::C2({
                    let a1019 = v0;
                    let a1020 = 1u64;
                    nat_add(a1019, a1020)?
                }, v7), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1021 = v0;
            let a1022 = f363();
            if nat_lt(a1021, a1022) {
                Ok(Adt132::C1(f364(123u64, Adt106::C4({
                    let a1023 = v0;
                    let a1024 = 1u64;
                    nat_add(a1023, a1024)?
                }, v8), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
    }
}

pub fn f291(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1026 = f189(f365(f110({
            let a1025 = v0;
            utf8_encode(a1025)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1027 = List::cons(34u64, List::<u64>::nil());
        append_list(a1026, a1027)
    }))
}

pub fn f292(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m1028 = v0;
    match m1028 {
        Adt32::C0(v1) => {
            Ok(Some(f366(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f367(v2)?))
        }
        Adt32::C2(v3) => {
            f368(f369(v3)?)
        }
        Adt32::C3(v4) => {
            f368(f369(match v4 {
                Adt49::C0(h1029, _) => {
                    h1029
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

pub fn f293(v0: Adt123) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f370(f371(match v0.clone() {
        Adt123::C0(h1030, _, _) => {
            h1030
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f372({
        let m1033 = match v0 {
            Adt123::C0(_, h1032, _) => {
                h1032
            }
        };
        match m1033 {
            Adt124::C0(h1031, _) => {
                h1031
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f294(v0: bool, v1: Adt88, v2: List<List<u64>>) -> Adt88 {
    if f373(v0, match v1.clone() {
        Adt88::C0(h1034, _, _, _) => {
            h1034
        }
    }, v2.clone()) {
        f374(match v1.clone() {
            Adt88::C0(h1035, _, _, _) => {
                h1035
            }
        }, {
            let a1037 = v2;
            let a1038 = List::cons(match v1 {
                Adt88::C0(_, _, _, h1036) => {
                    h1036
                }
            }, List::<List<u64>>::nil());
            append_list(a1037, a1038)
        })
    } else {
        v1
    }
}

pub fn f295(v0: Adt125) -> R<Option<List<u64>>> {
    let m1039 = f375(f376(v0))?;
    match m1039 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1040 = f112(v1)?;
            match m1040 {
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

pub fn f296() -> u64 {
    404u64
}

pub fn f297(v0: Adt84) -> u64 {
    let m1041 = v0;
    match m1041 {
        Adt84::C0(_) => {
            f119()
        }
        Adt84::C1(_) => {
            f119()
        }
        Adt84::C2(_) => {
            f119()
        }
        Adt84::C3(_) => {
            f119()
        }
        Adt84::C4(_) => {
            f119()
        }
        Adt84::C5(_) => {
            f119()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f377()
        }
        Adt84::C8(_) => {
            f119()
        }
        Adt84::C9(_) => {
            f21()
        }
    }
}

pub fn f298(v0: Adt126) -> i64 {
    let m1042 = v0;
    match m1042 {
        Adt126::C0(_) => {
            f378()
        }
        Adt126::C1(_, _) => {
            f379()
        }
        Adt126::C2(_) => {
            f380()
        }
        Adt126::C3(_) => {
            f381()
        }
        Adt126::C4(v6) => {
            f382(v6)
        }
    }
}

pub fn f299(v0: Adt126) -> Str {
    let m1043 = v0;
    match m1043 {
        Adt126::C0(v1) => {
            f383(v1)
        }
        Adt126::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt126::C2(v4) => {
            f384(v4)
        }
        Adt126::C3(_) => {
            Str::lit("Method not found")
        }
        Adt126::C4(v6) => {
            let m1044 = v6;
            match m1044 {
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

pub fn f300(v0: Adt126) -> R<Option<Adt6>> {
    let m1045 = v0;
    match m1045 {
        Adt126::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f385(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt126::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C4(v6) => {
            let m1046 = v6;
            match m1046 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f143(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f386(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f301(v0: List<u64>) -> R<bool> {
    let m1047 = f245(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1047.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f387(v2)? {
                Ok(false)
            } else {
                f163(f165(v1), f388())
            }
        }
    }
}

pub fn f302(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133) -> R<Result<Adt123, Adt126>> {
    f389(v0.clone(), v1, v2, v3.clone(), f390(match v0 {
        Adt0::C0(_, _, h1048, _, _, _) => {
            h1048
        }
    }, match v3 {
        Adt133::C0(_, h1049, _, _) => {
            h1049
        }
    }))
}

pub fn f303(v0: Adt137, v1: Adt138) -> Result<Adt133, Adt126> {
    let m1050 = v1;
    match m1050 {
        Adt138::C0(v3) => {
            f391(v3, v0)
        }
        Adt138::C1(v2) => {
            Err::<Adt133, Adt126>(v2)
        }
    }
}

pub fn f304(v0: Str) -> Adt137 {
    let a1051 = v0.clone();
    let a1052 = Str::lit("server/discover");
    if equal(a1051, a1052) {
        Adt137::C1(Adt134::C0)
    } else {
        let a1053 = v0.clone();
        let a1054 = Str::lit("tools/list");
        if equal(a1053, a1054) {
            Adt137::C1(Adt134::C1)
        } else {
            let a1055 = v0.clone();
            let a1056 = Str::lit("tools/call");
            if equal(a1055, a1056) {
                Adt137::C1(Adt134::C2)
            } else {
                let a1057 = v0.clone();
                let a1058 = Str::lit("resources/list");
                if equal(a1057, a1058) {
                    Adt137::C1(Adt134::C3)
                } else {
                    let a1059 = v0.clone();
                    let a1060 = Str::lit("resources/read");
                    if equal(a1059, a1060) {
                        Adt137::C1(Adt134::C4)
                    } else {
                        let a1061 = v0.clone();
                        let a1062 = Str::lit("resources/templates/list");
                        if equal(a1061, a1062) {
                            Adt137::C1(Adt134::C5)
                        } else {
                            let a1063 = v0.clone();
                            let a1064 = Str::lit("prompts/list");
                            if equal(a1063, a1064) {
                                Adt137::C1(Adt134::C6)
                            } else {
                                let a1065 = v0.clone();
                                let a1066 = Str::lit("prompts/get");
                                if equal(a1065, a1066) {
                                    Adt137::C1(Adt134::C7)
                                } else {
                                    let a1067 = v0.clone();
                                    let a1068 = Str::lit("completion/complete");
                                    if equal(a1067, a1068) {
                                        Adt137::C1(Adt134::C8)
                                    } else {
                                        let a1069 = v0.clone();
                                        let a1070 = Str::lit("tasks/get");
                                        if equal(a1069, a1070) {
                                            Adt137::C1(Adt134::C9)
                                        } else {
                                            let a1071 = v0.clone();
                                            let a1072 = Str::lit("tasks/update");
                                            if equal(a1071, a1072) {
                                                Adt137::C1(Adt134::C10)
                                            } else {
                                                let a1073 = v0;
                                                let a1074 = Str::lit("tasks/cancel");
                                                if equal(a1073, a1074) {
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

pub fn f305(v0: Option<Adt107>, v1: List<Str>, v2: Adt103) -> R<Adt138> {
    f392(v0, v1, v2.clone(), f393(match v2 {
        Adt103::C0(_, _, h1075) => {
            h1075
        }
    }))
}

pub fn f306(v0: List<u64>) -> Option<Str> {
    let a1076 = f357(f358(), v0);
    utf8_decode(a1076)
}

pub fn f307(v0: Str) -> Adt140 {
    let m1077 = f304(v0);
    match m1077 {
        Adt137::C0 => {
            Adt140::C0
        }
        Adt137::C1(v1) => {
            f394(v1)
        }
    }
}

pub fn f308(v0: List<u64>) -> Option<List<u64>> {
    let m1078 = f395(f396(), v0);
    match m1078 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1079 = f395(f397(), f173(v1, List::<u64>::nil()));
            match m1079 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f173(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f309(p1082: List<Adt110>, p1083: List<Adt110>) -> List<Adt110> {
    let m1081 = (p1082, p1083);
    {
        let mut m1081 = m1081;
        loop {
            let step = {
                let (v0, v1) = m1081;
                let m1080 = v0;
                match m1080.uncons() {
                    None => {
                        Err::<(List<Adt110>, List<Adt110>), List<Adt110>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt110>, List<Adt110>), List<Adt110>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1081 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: List<Adt86>, p1090: List<Adt12>, v2: Option<Str>, p1091: List<Adt110>) -> R<List<Adt110>> {
    let m1089 = (p1090, p1091);
    {
        let mut m1089 = m1089;
        loop {
            let step = {
                let (v1, v3) = m1089;
                let m1084 = v1;
                match m1084.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f398(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>((v5, List::cons(Adt110::C0(match v4.clone() {
                                Adt12::C0(_, h1085, _, _) => {
                                    h1085
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1086, _) => {
                                    h1086
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1087) => {
                                    h1087
                                }
                            }, f399(v0.clone(), match v4 {
                                Adt12::C0(_, h1088, _, _) => {
                                    h1088
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1089 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(p1107: u64, p1108: i64, p1109: i64) -> R<Option<i64>> {
    let m1106 = (p1107, (p1108, p1109));
    {
        let mut m1106 = m1106;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1106;
                let m1092 = v0;
                if m1092 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1092 - 1;
                    if f400(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1095 = {
                            let a1093 = v2;
                            let a1094 = 0i64;
                            compare(a1093, a1094)
                        };
                        if match m1095 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f400({
                                let a1096 = v1;
                                let a1097 = 10i64;
                                let a1098 = 0i64;
                                int_rem(a1096, a1097, a1098)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1099 = v1;
                                    let a1100 = 10i64;
                                    let a1101 = 0i64;
                                    int_quot(a1099, a1100, a1101)?
                                }, f187(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1102 = v1;
                                let a1103 = 10i64;
                                int_mul(a1102, a1103)?
                            }, {
                                let a1104 = v2;
                                let a1105 = 1i64;
                                int_sub(a1104, a1105)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: i64) -> Option<i64> {
    let m1112 = {
        let a1110 = 9007199254740991i64;
        let a1111 = v0;
        compare(a1110, a1111)
    };
    if if match m1112 {
        Ordering::Less => {
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
        let m1115 = {
            let a1113 = v0;
            let a1114 = -9007199254740991i64;
            compare(a1113, a1114)
        };
        match m1115 {
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

pub fn f313(v0: Str, v1: Str) -> Adt141 {
    let a1116 = v0;
    let a1117 = v1;
    if equal(a1116, a1117) {
        Adt141::C0
    } else {
        Adt141::C1
    }
}

pub fn f314(v0: Adt130) -> bool {
    let m1118 = v0;
    match m1118 {
        Adt130::C0 => {
            false
        }
        Adt130::C1(_) => {
            true
        }
    }
}

pub fn f315(v0: Adt92, v1: Str, v2: Adt5) -> Result<Adt103, Adt94> {
    let m1119 = f202(Str::lit("params"), v2);
    match m1119 {
        Adt130::C0 => {
            Ok::<Adt103, Adt94>(Adt103::C0(v0, v1, None::<Adt5>))
        }
        Adt130::C1(v3) => {
            let m1120 = f221(v3);
            match m1120 {
                None => {
                    f159(Some(v0), Adt95::C6)
                }
                Some(v4) => {
                    Ok::<Adt103, Adt94>(Adt103::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f316(v0: Adt118) -> bool {
    let m1121 = v0;
    match m1121 {
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

pub fn f317(v0: Adt114, v1: Adt119, v2: Adt115, v3: u64) -> R<Adt131> {
    let a1123 = match v0.clone() {
        Adt114::C0(_, _, h1122) => {
            h1122
        }
    };
    let a1124 = f363();
    if nat_lt(a1123, a1124) {
        Ok(Adt131::C1(Adt114::C0(v2, List::cons(v1, match v0.clone() {
            Adt114::C0(_, h1125, _) => {
                h1125
            }
        }), {
            let a1127 = match v0 {
                Adt114::C0(_, _, h1126) => {
                    h1126
                }
            };
            let a1128 = 1u64;
            nat_add(a1127, a1128)?
        })))
    } else {
        Ok(f273(Adt93::C9(v3)))
    }
}

pub fn f318(v0: bool, v1: Adt118, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt115 {
    Adt115::C8(Adt117::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f319(v0: u64) -> i64 {
    let a1129 = v0;
    let a1130 = 0u64;
    if nat_eq(a1129, a1130) {
        0i64
    } else {
        let a1131 = v0;
        let a1132 = 1u64;
        if nat_eq(a1131, a1132) {
            1i64
        } else {
            let a1133 = v0;
            let a1134 = 2u64;
            if nat_eq(a1133, a1134) {
                2i64
            } else {
                let a1135 = v0;
                let a1136 = 3u64;
                if nat_eq(a1135, a1136) {
                    3i64
                } else {
                    let a1137 = v0;
                    let a1138 = 4u64;
                    if nat_eq(a1137, a1138) {
                        4i64
                    } else {
                        let a1139 = v0;
                        let a1140 = 5u64;
                        if nat_eq(a1139, a1140) {
                            5i64
                        } else {
                            let a1141 = v0;
                            let a1142 = 6u64;
                            if nat_eq(a1141, a1142) {
                                6i64
                            } else {
                                let a1143 = v0;
                                let a1144 = 7u64;
                                if nat_eq(a1143, a1144) {
                                    7i64
                                } else {
                                    let a1145 = v0;
                                    let a1146 = 8u64;
                                    if nat_eq(a1145, a1146) {
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

pub fn f320(p1149: List<Adt6>, p1150: List<Adt6>) -> List<Adt6> {
    let m1148 = (p1149, p1150);
    {
        let mut m1148 = m1148;
        loop {
            let step = {
                let (v0, v1) = m1148;
                let m1147 = v0;
                match m1147.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
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

pub fn f321(v0: Adt114, v1: u64) -> Adt131 {
    let m1152 = match v0.clone() {
        Adt114::C0(_, h1151, _) => {
            h1151
        }
    };
    match m1152.uncons() {
        None => {
            f273(Adt93::C2(v1))
        }
        Some((v2, _)) => {
            let m1153 = v2;
            match m1153 {
                Adt119::C0(_) => {
                    f272(v0, Adt115::C0)
                }
                Adt119::C1(_, _) => {
                    f272(v0, Adt115::C4)
                }
            }
        }
    }
}

pub fn f322(p1156: List<(Str, Adt6)>, p1157: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1155 = (p1156, p1157);
    {
        let mut m1155 = m1155;
        loop {
            let step = {
                let (v0, v1) = m1155;
                let m1154 = v0;
                match m1154.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f323(v0: Adt114, v1: Adt115) -> Adt114 {
    Adt114::C0(v1, match v0.clone() {
        Adt114::C0(_, h1158, _) => {
            h1158
        }
    }, match v0 {
        Adt114::C0(_, _, h1159) => {
            h1159
        }
    })
}

pub fn f324(v0: Adt114, v1: u64, v2: List<u64>, v3: bool) -> Adt131 {
    let m1161 = {
        let a1160 = f357(f358(), f189(v2, List::<u64>::nil()));
        utf8_decode(a1160)
    };
    match m1161 {
        None => {
            f273(Adt93::C14)
        }
        Some(v4) => {
            if v3 {
                f401(v0, v1, v4)
            } else {
                f330(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f325(v0: u64) -> u64 {
    let a1162 = v0;
    let a1163 = 34u64;
    if nat_eq(a1162, a1163) {
        34u64
    } else {
        let a1164 = v0;
        let a1165 = 92u64;
        if nat_eq(a1164, a1165) {
            92u64
        } else {
            let a1166 = v0;
            let a1167 = 47u64;
            if nat_eq(a1166, a1167) {
                47u64
            } else {
                let a1168 = v0;
                let a1169 = 98u64;
                if nat_eq(a1168, a1169) {
                    8u64
                } else {
                    let a1170 = v0;
                    let a1171 = 102u64;
                    if nat_eq(a1170, a1171) {
                        12u64
                    } else {
                        let a1172 = v0;
                        let a1173 = 110u64;
                        if nat_eq(a1172, a1173) {
                            10u64
                        } else {
                            let a1174 = v0;
                            let a1175 = 114u64;
                            if nat_eq(a1174, a1175) {
                                13u64
                            } else {
                                let a1176 = v0;
                                let a1177 = 116u64;
                                if nat_eq(a1176, a1177) {
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

pub fn f326(v0: u64) -> u64 {
    let a1178 = v0;
    let a1179 = 48u64;
    if nat_lt(a1178, a1179) {
        16u64
    } else {
        let a1180 = v0;
        let a1181 = 58u64;
        if nat_lt(a1180, a1181) {
            let a1182 = v0;
            let a1183 = 48u64;
            nat_sub(a1182, a1183)
        } else {
            let a1184 = v0;
            let a1185 = 65u64;
            if nat_lt(a1184, a1185) {
                16u64
            } else {
                let a1186 = v0;
                let a1187 = 71u64;
                if nat_lt(a1186, a1187) {
                    let a1188 = v0;
                    let a1189 = 55u64;
                    nat_sub(a1188, a1189)
                } else {
                    let a1190 = v0;
                    let a1191 = 97u64;
                    if nat_lt(a1190, a1191) {
                        16u64
                    } else {
                        let a1192 = v0;
                        let a1193 = 103u64;
                        if nat_lt(a1192, a1193) {
                            let a1194 = v0;
                            let a1195 = 87u64;
                            nat_sub(a1194, a1195)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f327(v0: Adt114, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt131> {
    let a1196 = v3;
    let a1197 = 0u64;
    if nat_eq(a1196, a1197) {
        if f264(v2, 55296u64, 56319u64) {
            Ok(f272(v0, Adt115::C6(Adt116::C3(v2), v4, v5)))
        } else {
            if f264(v2, 56320u64, 57343u64) {
                Ok(f273(Adt93::C6(v1)))
            } else {
                Ok(f272(v0, Adt115::C6(Adt116::C0, f402(v2, v4)?, v5)))
            }
        }
    } else {
        if f264(v2, 56320u64, 57343u64) {
            Ok(f272(v0, Adt115::C6(Adt116::C0, f402({
                let a1206 = 65536u64;
                let a1207 = {
                    let a1204 = {
                        let a1200 = {
                            let a1198 = v3;
                            let a1199 = 55296u64;
                            nat_sub(a1198, a1199)
                        };
                        let a1201 = 1024u64;
                        nat_mul(a1200, a1201)?
                    };
                    let a1205 = {
                        let a1202 = v2;
                        let a1203 = 56320u64;
                        nat_sub(a1202, a1203)
                    };
                    nat_add(a1204, a1205)?
                };
                nat_add(a1206, a1207)?
            }, v4)?, v5)))
        } else {
            Ok(f273(Adt93::C6(v1)))
        }
    }
}

pub fn f328(v0: u64, v1: u64) -> u64 {
    let m1210 = {
        let a1208 = f329(v0);
        let a1209 = v1;
        index_list(a1208, a1209)
    };
    match m1210 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f329(v0: u64) -> List<u64> {
    let a1211 = v0;
    let a1212 = 0u64;
    if nat_eq(a1211, a1212) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1213 = v0;
        let a1214 = 1u64;
        if nat_eq(a1213, a1214) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f330(v0: Adt114, v1: Adt6) -> Adt131 {
    Adt131::C1(f266(v1, match v0.clone() {
        Adt114::C0(_, h1215, _) => {
            h1215
        }
    }, match v0 {
        Adt114::C0(_, _, h1216) => {
            h1216
        }
    }))
}

pub fn f331(v0: u64) -> Adt6 {
    let a1217 = v0;
    let a1218 = 0u64;
    if nat_eq(a1217, a1218) {
        Adt6::C1(true)
    } else {
        let a1219 = v0;
        let a1220 = 1u64;
        if nat_eq(a1219, a1220) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f332(v0: Adt114, v1: Adt117, v2: Adt118) -> Adt131 {
    f272(v0, Adt115::C8(Adt117::C0(match v1.clone() {
        Adt117::C0(h1221, _, _, _, _, _, _) => {
            h1221
        }
    }, v2, match v1.clone() {
        Adt117::C0(_, _, h1222, _, _, _, _) => {
            h1222
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, h1223, _, _, _) => {
            h1223
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, _, h1224, _, _) => {
            h1224
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, _, _, h1225, _) => {
            h1225
        }
    }, match v1 {
        Adt117::C0(_, _, _, _, _, _, h1226) => {
            h1226
        }
    })))
}

pub fn f333(v0: Adt114, v1: u64, v2: u64, v3: Adt117, v4: bool) -> R<Adt131> {
    let v5: u64 = {
        let a1228 = match v3.clone() {
            Adt117::C0(_, _, _, h1227, _, _, _) => {
                h1227
            }
        };
        let a1229 = 0u64;
        if if nat_eq(a1228, a1229) {
            let a1230 = v2;
            let a1231 = 48u64;
            nat_eq(a1230, a1231)
        } else {
            false
        } {
            0u64
        } else {
            let a1233 = match v3.clone() {
                Adt117::C0(_, _, _, h1232, _, _, _) => {
                    h1232
                }
            };
            let a1234 = 1u64;
            nat_add(a1233, a1234)?
        }
    };
    let v6: i64 = if v4 {
        f187(match v3.clone() {
            Adt117::C0(_, _, _, _, h1235, _, _) => {
                h1235
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt117::C0(_, _, _, _, h1236, _, _) => {
                h1236
            }
        }
    };
    let a1237 = f403();
    let a1238 = v5;
    if nat_lt(a1237, a1238) {
        Ok(f273(Adt93::C8(v1)))
    } else {
        let m1241 = {
            let a1239 = 99999i64;
            let a1240 = v6;
            compare(a1239, a1240)
        };
        if match m1241 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f273(Adt93::C8(v1)))
        } else {
            Ok(f272(v0, f318(match v3.clone() {
                Adt117::C0(h1242, _, _, _, _, _, _) => {
                    h1242
                }
            }, if v4 {
                Adt118::C4
            } else {
                Adt118::C2
            }, f187({
                let a1244 = match v3.clone() {
                    Adt117::C0(_, _, h1243, _, _, _, _) => {
                        h1243
                    }
                };
                let a1245 = 10i64;
                int_mul(a1244, a1245)?
            }, f319({
                let a1246 = v2;
                let a1247 = 48u64;
                nat_sub(a1246, a1247)
            }))?, v5, v6, match v3.clone() {
                Adt117::C0(_, _, _, _, _, h1248, _) => {
                    h1248
                }
            }, match v3 {
                Adt117::C0(_, _, _, _, _, _, h1249) => {
                    h1249
                }
            })))
        }
    }
}

pub fn f334(v0: u64) -> bool {
    let a1250 = v0;
    let a1251 = 101u64;
    if nat_eq(a1250, a1251) {
        true
    } else {
        let a1252 = v0;
        let a1253 = 69u64;
        nat_eq(a1252, a1253)
    }
}

pub fn f335(v0: u64) -> bool {
    f264(v0, 48u64, 57u64)
}

pub fn f336(v0: Adt114, v1: u64, v2: u64, v3: Adt117) -> R<Adt131> {
    let m1254 = f265(v3)?;
    match m1254 {
        Adt135::C0 => {
            Ok(f273(Adt93::C7(v1)))
        }
        Adt135::C1 => {
            Ok(f273(Adt93::C8(v1)))
        }
        Adt135::C2(v4) => {
            Ok(f270(f266(v4, match v0.clone() {
                Adt114::C0(_, h1255, _) => {
                    h1255
                }
            }, match v0 {
                Adt114::C0(_, _, h1256) => {
                    h1256
                }
            }), v1, v2))
        }
    }
}

pub fn f337(v0: Adt114, v1: u64, v2: u64, v3: Adt117, v4: bool) -> R<Adt131> {
    let v5: i64 = f187({
        let a1258 = match v3.clone() {
            Adt117::C0(_, _, _, _, _, _, h1257) => {
                h1257
            }
        };
        let a1259 = 10i64;
        int_mul(a1258, a1259)?
    }, f319({
        let a1260 = v2;
        let a1261 = 48u64;
        nat_sub(a1260, a1261)
    }))?;
    let m1264 = {
        let a1262 = 99999i64;
        let a1263 = v5;
        compare(a1262, a1263)
    };
    if match m1264 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f273(Adt93::C8(v1)))
    } else {
        Ok(f272(v0, f318(match v3.clone() {
            Adt117::C0(h1265, _, _, _, _, _, _) => {
                h1265
            }
        }, Adt118::C7, match v3.clone() {
            Adt117::C0(_, _, h1266, _, _, _, _) => {
                h1266
            }
        }, match v3.clone() {
            Adt117::C0(_, _, _, h1267, _, _, _) => {
                h1267
            }
        }, match v3 {
            Adt117::C0(_, _, _, _, h1268, _, _) => {
                h1268
            }
        }, v4, v5)))
    }
}

pub fn f338(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1269 = f404(f405(), v2.clone());
    match m1269 {
        None => {
            let m1270 = f406(f407(), v2.clone());
            match m1270 {
                None => {
                    f408(v0, v1, v2)
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

pub fn f339(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt120) -> R<Adt142> {
    let m1271 = f260(Str::lit("x-mcp-header"), v1.clone());
    match m1271 {
        Adt130::C0 => {
            Ok(Adt142::C1(v4))
        }
        Adt130::C1(v5) => {
            f409(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f340(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt104>) -> List<Adt104> {
    f410(Str::lit("properties"), v0.clone(), v1, v2, f410(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f411(Str::lit("items"), v0.clone(), f411(Str::lit("additionalProperties"), v0.clone(), f412(Str::lit("allOf"), v0.clone(), f412(Str::lit("anyOf"), v0.clone(), f412(Str::lit("oneOf"), v0.clone(), f411(Str::lit("not"), v0.clone(), f411(Str::lit("if"), v0.clone(), f411(Str::lit("then"), v0.clone(), f411(Str::lit("else"), v0, v3)))))))))))
}

pub fn f341(p1278: List<(Str, Adt136)>) -> Adt121 {
    let m1277 = p1278;
    {
        let mut m1277 = m1277;
        loop {
            let step = {
                let v0 = m1277;
                let m1272 = v0;
                match m1272.uncons() {
                    None => {
                        Err::<List<(Str, Adt136)>, Adt121>(Adt121::C3(List::<Adt101>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1274 = {
                            let (_, h1273) = v1;
                            h1273
                        };
                        match m1274 {
                            Adt136::C0 => {
                                Ok::<List<(Str, Adt136)>, Adt121>(v2)
                            }
                            Adt136::C1 => {
                                Err::<List<(Str, Adt136)>, Adt121>(Adt121::C0({
                                    let (h1275, _) = v1;
                                    h1275
                                }))
                            }
                            Adt136::C2 => {
                                Err::<List<(Str, Adt136)>, Adt121>(Adt121::C1({
                                    let (h1276, _) = v1;
                                    h1276
                                }))
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

pub fn f342(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1279 = f260(Str::lit("type"), v0);
    match m1279 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1280 = f257(v2.clone());
            match m1280 {
                None => {
                    let m1281 = f285(v2);
                    match m1281 {
                        None => {
                            Ok(Adt136::C2)
                        }
                        Some(v4) => {
                            f415(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f413(v3.clone()) {
                        if f414(v3, v1)? {
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

pub fn f343(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1282 = f260(Str::lit("enum"), v0);
    match m1282 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1283 = f285(v2);
            match m1283 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v3) => {
                    f416(v3, v1)
                }
            }
        }
    }
}

pub fn f344(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1284 = f260(Str::lit("const"), v0);
    match m1284 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1285 = f417(v2, v1)?;
            match m1285 {
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

pub fn f345(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt136> {
    let m1286 = f260(v0, v2);
    match m1286 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v4) => {
            let m1287 = f418(v4);
            match m1287 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v5) => {
                    let m1288 = f418(v3);
                    match m1288 {
                        None => {
                            Ok(Adt136::C0)
                        }
                        Some(v6) => {
                            if f419(v1, f420(v6, v5)?) {
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

pub fn f346(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt136)>> {
    let m1289 = v1;
    match m1289 {
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
            Ok(List::cons((Str::lit("minLength"), f421(Str::lit("minLength"), true, v0.clone(), f422(v4.clone())?)?), List::cons((Str::lit("maxLength"), f421(Str::lit("maxLength"), false, v0, f422(v4)?)?), List::<(Str, Adt136)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f421(Str::lit("minItems"), true, v0.clone(), f423(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f421(Str::lit("maxItems"), false, v0, f423(v5, 0u64)?)?), List::<(Str, Adt136)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f421(Str::lit("minProperties"), true, v0.clone(), f424(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f421(Str::lit("maxProperties"), false, v0, f424(v6, 0u64)?)?), List::<(Str, Adt136)>::nil())))
        }
    }
}

pub fn f347(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt136 {
    let m1290 = f260(Str::lit("required"), v0);
    match m1290 {
        Adt130::C0 => {
            Adt136::C0
        }
        Adt130::C1(v2) => {
            let m1291 = f285(v2);
            match m1291 {
                None => {
                    Adt136::C2
                }
                Some(v3) => {
                    let m1292 = f221(v1);
                    match m1292 {
                        None => {
                            Adt136::C0
                        }
                        Some(v4) => {
                            f425(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f348(v0: Adt121, v1: Adt121) -> Adt121 {
    let m1293 = v0.clone();
    match m1293 {
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
            let m1294 = v1.clone();
            match m1294 {
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
                        let a1295 = v5;
                        let a1296 = v9;
                        append_list(a1295, a1296)
                    })
                }
            }
        }
    }
}

pub fn f349(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> Adt121 {
    let m1297 = f260(Str::lit("$ref"), v1);
    match m1297 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v4) => {
            let m1298 = f257(v4);
            match m1298 {
                None => {
                    Adt121::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1299 = f426(v0, v5.clone());
                    match m1299 {
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

pub fn f350(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1300 = f260(Str::lit("properties"), v0);
    match m1300 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1301 = f221(v3);
            match m1301 {
                None => {
                    Adt121::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1302 = f221(v1);
                    match m1302 {
                        None => {
                            Adt121::C3(List::<Adt101>::nil())
                        }
                        Some(v5) => {
                            Adt121::C3(f427(match v4 {
                                Adt5::C0(h1303) => {
                                    h1303
                                }
                            }, match v5 {
                                Adt5::C0(h1304) => {
                                    h1304
                                }
                            }, v2, List::<Adt101>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f351(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1305 = f260(Str::lit("additionalProperties"), v0.clone());
    match m1305 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1306 = f221(v1);
            match m1306 {
                None => {
                    Adt121::C3(List::<Adt101>::nil())
                }
                Some(v4) => {
                    let m1307 = f220(v3.clone());
                    match m1307 {
                        None => {
                            Adt121::C3(f430(match v4 {
                                Adt5::C0(h1309) => {
                                    h1309
                                }
                            }, f429(v0), v3, v2, List::<Adt101>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt121::C3(List::<Adt101>::nil())
                            } else {
                                if f428(match v4 {
                                    Adt5::C0(h1308) => {
                                        h1308
                                    }
                                }, f429(v0)) {
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

pub fn f352(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> R<Adt121> {
    let m1310 = f260(Str::lit("items"), v0);
    match m1310 {
        Adt130::C0 => {
            Ok(Adt121::C3(List::<Adt101>::nil()))
        }
        Adt130::C1(v3) => {
            let m1311 = f285(v1);
            match m1311 {
                None => {
                    Ok(Adt121::C3(List::<Adt101>::nil()))
                }
                Some(v4) => {
                    Ok(Adt121::C3(f431(v4, 0u64, v3, v2, List::<Adt101>::nil())?))
                }
            }
        }
    }
}

pub fn f353(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1312 = f260(Str::lit("allOf"), v0);
    match m1312 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1313 = f285(v3);
            match m1313 {
                None => {
                    Adt121::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1314 = v4.clone();
                    match m1314.uncons() {
                        None => {
                            Adt121::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt121::C3(f432(v4, v1, v2, List::<Adt101>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f354(v0: Adt102, v1: Adt121, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt99>) -> Adt121 {
    let m1315 = v1.clone();
    match m1315 {
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
            let m1316 = v8;
            match m1316.uncons() {
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

pub fn f355(p1325: u64, p1326: List<u64>, p1327: List<u64>) -> R<Option<List<u64>>> {
    let m1324 = (p1325, (p1326, p1327));
    {
        let mut m1324 = m1324;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1324;
                let m1317 = v0;
                if m1317 == 0 {
                    let m1318 = v1;
                    match m1318.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1317 - 1;
                    let m1319 = v1;
                    match m1319.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1320 = v7;
                            match m1320.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1321 = v9;
                                    match m1321.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1322 = v11;
                                            match m1322.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1323 = f433(v6, v8, v10, v12, f434(v13.clone()))?;
                                                    match m1323 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f189(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(p1332: List<Adt50>, p1333: Adt7) -> R<bool> {
    let m1331 = (p1332, p1333);
    {
        let mut m1331 = m1331;
        loop {
            let step = {
                let (v0, v1) = m1331;
                let m1328 = v0;
                match m1328.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f435(f420(match v2.clone() {
                            Adt50::C0(h1329, _, _) => {
                                h1329
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1330, _, _) => {
                                    h1330
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1331 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1334 = v0;
    let a1335 = f436(f358(), f437(List::<Bytes>::nil(), v1));
    append_bytes(a1334, a1335)
}

pub fn f358() -> Bytes {
    Bytes::lit(&[])
}

pub fn f359(p1342: List<u64>, p1343: List<u64>) -> List<u64> {
    let m1341 = (p1342, p1343);
    {
        let mut m1341 = m1341;
        loop {
            let step = {
                let (v0, v1) = m1341;
                let m1336 = v0;
                match m1336.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1337 = v2;
                        let a1338 = 126u64;
                        if nat_eq(a1337, a1338) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1339 = v2;
                            let a1340 = 47u64;
                            if nat_eq(a1339, a1340) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1341 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(v0: u64) -> List<u64> {
    f329(v0)
}

pub fn f361(v0: Adt7) -> R<bool> {
    let a1361 = {
        let m1347 = {
            let a1345 = f438();
            let a1346 = match v0.clone() {
                Adt7::C0(h1344, _) => {
                    h1344
                }
            };
            compare(a1345, a1346)
        };
        if match m1347 {
            Ordering::Less => {
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
            let m1352 = {
                let a1350 = match v0.clone() {
                    Adt7::C0(h1348, _) => {
                        h1348
                    }
                };
                let a1351 = {
                    let a1349 = f438();
                    int_neg(a1349)?
                };
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
                true
            } else {
                let m1356 = {
                    let a1354 = 9999i64;
                    let a1355 = match v0.clone() {
                        Adt7::C0(_, h1353) => {
                            h1353
                        }
                    };
                    compare(a1354, a1355)
                };
                if match m1356 {
                    Ordering::Less => {
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
                    let m1360 = {
                        let a1358 = match v0 {
                            Adt7::C0(_, h1357) => {
                                h1357
                            }
                        };
                        let a1359 = -9999i64;
                        compare(a1358, a1359)
                    };
                    match m1360 {
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
    Ok(bool_not(a1361))
}

pub fn f362(v0: Adt7) -> R<List<u64>> {
    let m1365 = {
        let a1363 = match v0.clone() {
            Adt7::C0(_, h1362) => {
                h1362
            }
        };
        let a1364 = 0i64;
        compare(a1363, a1364)
    };
    if if match m1365 {
        Ordering::Less => {
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
        let m1369 = {
            let a1367 = 0i64;
            let a1368 = match v0.clone() {
                Adt7::C0(_, h1366) => {
                    h1366
                }
            };
            compare(a1367, a1368)
        };
        match m1369 {
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
        let a1372 = f439(match v0.clone() {
            Adt7::C0(h1370, _) => {
                h1370
            }
        })?;
        let a1373 = List::cons(101u64, f439(match v0 {
            Adt7::C0(_, h1371) => {
                h1371
            }
        })?);
        Ok(append_list(a1372, a1373))
    } else {
        f439(match v0 {
            Adt7::C0(h1374, _) => {
                h1374
            }
        })
    }
}

pub fn f363() -> u64 {
    512u64
}

pub fn f364(v0: u64, v1: Adt106, v2: List<Adt106>) -> List<Adt106> {
    List::cons(Adt106::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f365(p1377: List<u64>, p1378: List<u64>) -> R<List<u64>> {
    let m1376 = (p1377, p1378);
    {
        let mut m1376 = m1376;
        loop {
            let step = {
                let (v0, v1) = m1376;
                let m1375 = v0;
                match m1375.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f189(f440(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f441(match v0.clone() {
        Adt33::C0(_, h1379, _, _) => {
            h1379
        }
    })), List::cons((Str::lit("capabilities"), f442(match v0.clone() {
        Adt33::C0(_, _, h1380, _) => {
            h1380
        }
    })), {
        let a1383 = f443(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1381) => {
                h1381
            }
        });
        let a1384 = f444(match v0 {
            Adt33::C0(h1382, _, _, _) => {
                h1382
            }
        })?;
        append_list(a1383, a1384)
    })))
}

pub fn f367(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f445(match v0.clone() {
        Adt34::C0(_, _, h1385) => {
            h1385
        }
    }))), {
        let a1388 = f443(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1386, _) => {
                h1386
            }
        });
        let a1389 = f444(match v0 {
            Adt34::C0(h1387, _, _) => {
                h1387
            }
        })?;
        append_list(a1388, a1389)
    }))
}

pub fn f368(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1390 = v0;
    match m1390 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1391 = f375(Adt6::C5(v1))?;
            match m1391 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1392 = f221(v2);
                    match m1392 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1393) => {
                                    h1393
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f369(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1395 = f446(match v0.clone() {
        Adt37::C0(h1394, _, _) => {
            h1394
        }
    })?;
    match m1395 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1398 = f447(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1396, _) => {
                        h1396
                    }
                });
                let a1399 = f448(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1397) => {
                        h1397
                    }
                });
                append_list(a1398, a1399)
            })))
        }
    }
}

pub fn f370(v0: Adt143) -> Str {
    let m1400 = v0;
    match m1400 {
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

pub fn f371(v0: Adt32) -> Adt143 {
    let m1401 = v0;
    match m1401 {
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

pub fn f372(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1402 = v0;
    match m1402 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f449(), f450(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f373(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f451(v2) {
            let a1403 = v1;
            let a1404 = 200u64;
            nat_eq(a1403, a1404)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f374(v0: u64, v1: List<List<u64>>) -> Adt88 {
    Adt88::C0(v0, f452(), List::<List<u64>>::nil(), f453(v1))
}

pub fn f375(v0: Adt6) -> R<Option<Adt6>> {
    f454(f455(), Adt144::C0(v0), List::<Adt145>::nil())
}

pub fn f376(v0: Adt125) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f150())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f456(match v0.clone() {
        Adt125::C0(h1405, _) => {
            h1405
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1408 = match v0.clone() {
            Adt125::C0(_, h1407) => {
                h1407
            }
        };
        match m1408 {
            Adt50::C0(h1406, _, _) => {
                h1406
            }
        }
    })), {
        let a1415 = f457({
            let m1411 = match v0.clone() {
                Adt125::C0(_, h1410) => {
                    h1410
                }
            };
            match m1411 {
                Adt50::C0(_, h1409, _) => {
                    h1409
                }
            }
        });
        let a1416 = f458({
            let m1414 = match v0 {
                Adt125::C0(_, h1413) => {
                    h1413
                }
            };
            match m1414 {
                Adt50::C0(_, _, h1412) => {
                    h1412
                }
            }
        });
        append_list(a1415, a1416)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f377() -> u64 {
    429u64
}

pub fn f378() -> i64 {
    -32602i64
}

pub fn f379() -> i64 {
    -32022i64
}

pub fn f380() -> i64 {
    -32020i64
}

pub fn f381() -> i64 {
    -32601i64
}

pub fn f382(v0: Adt84) -> i64 {
    let m1417 = v0;
    match m1417 {
        Adt84::C0(_) => {
            f378()
        }
        Adt84::C1(_) => {
            f378()
        }
        Adt84::C2(_) => {
            f378()
        }
        Adt84::C3(_) => {
            f378()
        }
        Adt84::C4(_) => {
            f378()
        }
        Adt84::C5(_) => {
            f378()
        }
        Adt84::C6(_) => {
            f459()
        }
        Adt84::C7(_) => {
            f460()
        }
        Adt84::C8(_) => {
            f461()
        }
        Adt84::C9(_) => {
            f459()
        }
    }
}

pub fn f383(v0: Adt127) -> Str {
    let m1418 = v0;
    match m1418 {
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

pub fn f384(v0: Adt108) -> Str {
    let m1419 = v0;
    match m1419 {
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
            let m1420 = v1;
            match m1420 {
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
            let m1421 = v2;
            match m1421 {
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

pub fn f385(v0: List<Str>) -> List<Adt6> {
    f320(f462(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f386(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1433 = f463(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1422, _, _, _, _) => {
                h1422
            }
        });
        let a1434 = {
            let a1431 = f463(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1423, _, _, _) => {
                    h1423
                }
            });
            let a1432 = {
                let a1429 = f463(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1424, _, _) => {
                        h1424
                    }
                });
                let a1430 = {
                    let a1427 = f463(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1425, _) => {
                            h1425
                        }
                    });
                    let a1428 = f463(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1426) => {
                            h1426
                        }
                    });
                    append_list(a1427, a1428)
                };
                append_list(a1429, a1430)
            };
            append_list(a1431, a1432)
        };
        append_list(a1433, a1434)
    })
}

pub fn f387(p1437: List<List<u64>>) -> R<bool> {
    let m1436 = p1437;
    {
        let mut m1436 = m1436;
        loop {
            let step = {
                let v0 = m1436;
                let m1435 = v0;
                match m1435.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f464(f165(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f389(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133, v4: bool) -> R<Result<Adt123, Adt126>> {
    if v4 {
        f465(v0, v1, v2, f466(match v3.clone() {
            Adt133::C0(_, h1438, _, _) => {
                h1438
            }
        }, match v3.clone() {
            Adt133::C0(_, _, h1439, _) => {
                h1439
            }
        }, match v3 {
            Adt133::C0(_, _, _, h1440) => {
                h1440
            }
        }))
    } else {
        Ok(Err::<Adt123, Adt126>(Adt126::C3(f467(match v3 {
            Adt133::C0(_, h1441, _, _) => {
                h1441
            }
        }))))
    }
}

pub fn f390(v0: Adt4, v1: Adt134) -> bool {
    let m1442 = v1;
    match m1442 {
        Adt134::C0 => {
            true
        }
        Adt134::C1 => {
            f468(match v0 {
                Adt4::C0(_, _, _, _, _, h1443, _) => {
                    h1443
                }
            })
        }
        Adt134::C2 => {
            f468(match v0 {
                Adt4::C0(_, _, _, _, _, h1444, _) => {
                    h1444
                }
            })
        }
        Adt134::C3 => {
            f469(match v0 {
                Adt4::C0(_, _, _, _, h1445, _, _) => {
                    h1445
                }
            })
        }
        Adt134::C4 => {
            f469(match v0 {
                Adt4::C0(_, _, _, _, h1446, _, _) => {
                    h1446
                }
            })
        }
        Adt134::C5 => {
            f469(match v0 {
                Adt4::C0(_, _, _, _, h1447, _, _) => {
                    h1447
                }
            })
        }
        Adt134::C6 => {
            f468(match v0 {
                Adt4::C0(_, _, _, h1448, _, _, _) => {
                    h1448
                }
            })
        }
        Adt134::C7 => {
            f468(match v0 {
                Adt4::C0(_, _, _, h1449, _, _, _) => {
                    h1449
                }
            })
        }
        Adt134::C8 => {
            f470(match v0 {
                Adt4::C0(_, _, h1450, _, _, _, _) => {
                    h1450
                }
            })
        }
        Adt134::C9 => {
            f471(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1451) => {
                    h1451
                }
            }, f472())
        }
        Adt134::C10 => {
            f471(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1452) => {
                    h1452
                }
            }, f472())
        }
        Adt134::C11 => {
            f471(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1453) => {
                    h1453
                }
            }, f472())
        }
    }
}

pub fn f391(v0: Adt139, v1: Adt137) -> Result<Adt133, Adt126> {
    let m1454 = v1;
    match m1454 {
        Adt137::C0 => {
            Err::<Adt133, Adt126>(Adt126::C3(match v0 {
                Adt139::C0(_, h1455, _, _) => {
                    h1455
                }
            }))
        }
        Adt137::C1(v2) => {
            Ok::<Adt133, Adt126>(Adt133::C0(match v0.clone() {
                Adt139::C0(h1456, _, _, _) => {
                    h1456
                }
            }, v2, match v0.clone() {
                Adt139::C0(_, _, h1457, _) => {
                    h1457
                }
            }, match v0 {
                Adt139::C0(_, _, _, h1458) => {
                    h1458
                }
            }))
        }
    }
}

pub fn f392(v0: Option<Adt107>, v1: List<Str>, v2: Adt103, v3: Result<Adt16, Adt127>) -> R<Adt138> {
    let m1459 = v3;
    match m1459 {
        Ok(v5) => {
            Ok(f473(v1, v2.clone(), v5.clone(), f474(v0, f307(match v2.clone() {
                Adt103::C0(_, h1460, _) => {
                    h1460
                }
            }), match v2.clone() {
                Adt103::C0(_, h1461, _) => {
                    h1461
                }
            }, match v5 {
                Adt16::C0(h1462, _, _, _, _, _) => {
                    h1462
                }
            }, f475(match v2 {
                Adt103::C0(_, _, h1463) => {
                    h1463
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt138::C1(Adt126::C0(v4)))
        }
    }
}

pub fn f393(v0: Option<Adt5>) -> Result<Adt16, Adt127> {
    let m1464 = v0;
    match m1464 {
        None => {
            Err::<Adt16, Adt127>(Adt127::C0)
        }
        Some(v1) => {
            let m1465 = f202(Str::lit("_meta"), v1);
            match m1465 {
                Adt130::C0 => {
                    Err::<Adt16, Adt127>(Adt127::C1)
                }
                Adt130::C1(v2) => {
                    let m1466 = f221(v2);
                    match m1466 {
                        None => {
                            Err::<Adt16, Adt127>(Adt127::C4)
                        }
                        Some(v3) => {
                            f476(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f394(v0: Adt134) -> Adt140 {
    let m1467 = v0;
    match m1467 {
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

pub fn f395(p1473: List<u64>, p1474: List<u64>) -> Option<List<u64>> {
    let m1472 = (p1473, p1474);
    {
        let mut m1472 = m1472;
        loop {
            let step = {
                let (v0, v1) = m1472;
                let m1468 = v0;
                match m1468.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1469 = v1;
                        match m1469.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1470 = v4;
                                let a1471 = v2;
                                if nat_eq(a1470, a1471) {
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
                Ok(next) => { m1472 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f397() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f398(v0: Adt12, v1: Option<Str>) -> bool {
    let m1475 = v1;
    match m1475 {
        None => {
            false
        }
        Some(v2) => {
            let a1477 = match v0 {
                Adt12::C0(h1476, _, _, _) => {
                    h1476
                }
            };
            let a1478 = v2;
            equal(a1477, a1478)
        }
    }
}

pub fn f399(v0: List<Adt86>, v1: Str) -> R<Adt111> {
    let m1481 = f252(f90(v0, {
        let a1479 = f477();
        let a1480 = f478(v1)?;
        append_list(a1479, a1480)
    })?)?;
    match m1481 {
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
            let m1482 = f306(v2);
            match m1482 {
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

pub fn f400(v0: i64) -> bool {
    let a1489 = {
        let m1485 = {
            let a1483 = v0;
            let a1484 = 0i64;
            compare(a1483, a1484)
        };
        if match m1485 {
            Ordering::Less => {
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
            let m1488 = {
                let a1486 = 0i64;
                let a1487 = v0;
                compare(a1486, a1487)
            };
            match m1488 {
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
    bool_not(a1489)
}

pub fn f401(v0: Adt114, v1: u64, v2: Str) -> Adt131 {
    let m1491 = match v0.clone() {
        Adt114::C0(_, h1490, _) => {
            h1490
        }
    };
    match m1491.uncons() {
        None => {
            f273(Adt93::C2(v1))
        }
        Some((v3, v4)) => {
            let m1492 = v3;
            match m1492 {
                Adt119::C0(_) => {
                    f273(Adt93::C2(v1))
                }
                Adt119::C1(v6, _) => {
                    Adt131::C1(Adt114::C0(Adt115::C5, List::cons(Adt119::C1(v6, Some(v2)), v4), match v0 {
                        Adt114::C0(_, _, h1493) => {
                            h1493
                        }
                    }))
                }
            }
        }
    }
}

pub fn f402(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1494 = v0;
    let a1495 = 128u64;
    if nat_lt(a1494, a1495) {
        Ok(List::cons(v0, v1))
    } else {
        let a1496 = v0;
        let a1497 = 2048u64;
        if nat_lt(a1496, a1497) {
            Ok(List::cons({
                let a1498 = 128u64;
                let a1499 = f479(v0, 64u64);
                nat_add(a1498, a1499)?
            }, List::cons({
                let a1500 = 192u64;
                let a1501 = f480(v0, 64u64);
                nat_add(a1500, a1501)?
            }, v1)))
        } else {
            let a1502 = v0;
            let a1503 = 65536u64;
            if nat_lt(a1502, a1503) {
                Ok(List::cons({
                    let a1504 = 128u64;
                    let a1505 = f479(v0, 64u64);
                    nat_add(a1504, a1505)?
                }, List::cons({
                    let a1506 = 128u64;
                    let a1507 = f479(f480(v0, 64u64), 64u64);
                    nat_add(a1506, a1507)?
                }, List::cons({
                    let a1508 = 224u64;
                    let a1509 = f480(v0, 4096u64);
                    nat_add(a1508, a1509)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1510 = 128u64;
                    let a1511 = f479(v0, 64u64);
                    nat_add(a1510, a1511)?
                }, List::cons({
                    let a1512 = 128u64;
                    let a1513 = f479(f480(v0, 64u64), 64u64);
                    nat_add(a1512, a1513)?
                }, List::cons({
                    let a1514 = 128u64;
                    let a1515 = f479(f480(v0, 4096u64), 64u64);
                    nat_add(a1514, a1515)?
                }, List::cons({
                    let a1516 = 240u64;
                    let a1517 = f480(v0, 262144u64);
                    nat_add(a1516, a1517)?
                }, v1)))))
            }
        }
    }
}

pub fn f403() -> u64 {
    18u64
}

pub fn f404(p1521: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1520 = p1521;
    {
        let mut m1520 = m1520;
        loop {
            let step = {
                let v0 = m1520;
                let m1518 = v0;
                match m1518.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1519 = f260(v2.clone(), v1.clone());
                        match m1519 {
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
                Ok(next) => { m1520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f406(p1528: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1527 = p1528;
    {
        let mut m1527 = m1527;
        loop {
            let step = {
                let v0 = m1527;
                let m1522 = v0;
                match m1522.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1524 = f260({
                            let (h1523, _) = v2.clone();
                            h1523
                        }, v1.clone());
                        match m1524 {
                            Adt130::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt130::C1(v4) => {
                                if f481({
                                    let (_, h1525) = v2;
                                    h1525
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1526, _) = v2;
                                        h1526
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f408(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1529 = f260(Str::lit("$ref"), v2);
    match m1529 {
        Adt130::C0 => {
            None::<Adt91>
        }
        Adt130::C1(v3) => {
            let m1530 = f257(v3);
            match m1530 {
                None => {
                    None::<Adt91>
                }
                Some(v4) => {
                    let m1531 = f426(v1, v4.clone());
                    match m1531 {
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

pub fn f409(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt120) -> R<Adt142> {
    if v3 {
        let m1532 = f257(v2);
        match m1532 {
            None => {
                Ok(Adt142::C0(Adt91::C7(v0)))
            }
            Some(v6) => {
                f482(v0, v1, v6.clone(), f136(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt142::C0(Adt91::C10(v0)))
    }
}

pub fn f410(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt104>) -> List<Adt104> {
    let m1533 = f260(v0.clone(), v1);
    match m1533 {
        Adt130::C0 => {
            v4
        }
        Adt130::C1(v5) => {
            let m1534 = f221(v5);
            match m1534 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1535 = v0;
                    let a1536 = Str::lit("properties");
                    if equal(a1535, a1536) {
                        f483(match v6 {
                            Adt5::C0(h1537) => {
                                h1537
                            }
                        }, v2, v3, v4)
                    } else {
                        f484(match v6 {
                            Adt5::C0(h1538) => {
                                h1538
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f411(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt104>) -> List<Adt104> {
    let m1539 = f260(v0, v1);
    match m1539 {
        Adt130::C0 => {
            v2
        }
        Adt130::C1(v3) => {
            List::cons(Adt104::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f412(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt104>) -> List<Adt104> {
    let m1540 = f260(v0, v1);
    match m1540 {
        Adt130::C0 => {
            v2
        }
        Adt130::C1(v3) => {
            let m1541 = f285(v3);
            match m1541 {
                None => {
                    v2
                }
                Some(v4) => {
                    f485(v4, v2)
                }
            }
        }
    }
}

pub fn f413(v0: Str) -> bool {
    let a1542 = v0.clone();
    let a1543 = Str::lit("null");
    if equal(a1542, a1543) {
        true
    } else {
        let a1544 = v0.clone();
        let a1545 = Str::lit("boolean");
        if equal(a1544, a1545) {
            true
        } else {
            let a1546 = v0.clone();
            let a1547 = Str::lit("object");
            if equal(a1546, a1547) {
                true
            } else {
                let a1548 = v0.clone();
                let a1549 = Str::lit("array");
                if equal(a1548, a1549) {
                    true
                } else {
                    let a1550 = v0.clone();
                    let a1551 = Str::lit("number");
                    if equal(a1550, a1551) {
                        true
                    } else {
                        let a1552 = v0.clone();
                        let a1553 = Str::lit("string");
                        if equal(a1552, a1553) {
                            true
                        } else {
                            let a1554 = v0;
                            let a1555 = Str::lit("integer");
                            equal(a1554, a1555)
                        }
                    }
                }
            }
        }
    }
}

pub fn f414(v0: Str, v1: Adt6) -> R<bool> {
    let a1556 = v0.clone();
    let a1557 = f486(v1.clone());
    if equal(a1556, a1557) {
        Ok(true)
    } else {
        let a1558 = v0;
        let a1559 = Str::lit("integer");
        if equal(a1558, a1559) {
            let m1560 = f418(v1);
            match m1560 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f487(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f415(p1563: List<Adt6>, v1: Adt6, p1564: bool) -> R<Adt136> {
    let m1562 = (p1563, p1564);
    {
        let mut m1562 = m1562;
        loop {
            let step = {
                let (v0, v2) = m1562;
                let m1561 = v0;
                match m1561.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f413(f488(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt136>>((v4, if v2 {
                                true
                            } else {
                                f414(f488(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(p1568: List<Adt6>, v1: Adt6) -> R<Adt136> {
    let m1567 = p1568;
    {
        let mut m1567 = m1567;
        loop {
            let step = {
                let v0 = m1567;
                let m1565 = v0;
                match m1565.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt136>>(Ok(Adt136::C1))
                    }
                    Some((v2, v3)) => {
                        let m1566 = f417(v2, v1.clone())?;
                        match m1566 {
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
                Ok(next) => { m1567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f489(f490(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f418(v0: Adt6) -> Option<Adt7> {
    let m1569 = v0;
    match m1569 {
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

pub fn f419(v0: u64, v1: Adt147) -> bool {
    let a1570 = v0;
    let a1571 = 0u64;
    if nat_eq(a1570, a1571) {
        let a1572 = f491(v1);
        bool_not(a1572)
    } else {
        let a1573 = v0;
        let a1574 = 1u64;
        if nat_eq(a1573, a1574) {
            let a1575 = f435(v1);
            bool_not(a1575)
        } else {
            let a1576 = v0;
            let a1577 = 2u64;
            if nat_eq(a1576, a1577) {
                f435(v1)
            } else {
                f491(v1)
            }
        }
    }
}

pub fn f420(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f492(v0.clone(), v1.clone(), f493(match v0 {
        Adt7::C0(h1578, _) => {
            h1578
        }
    }), f493(match v1 {
        Adt7::C0(h1579, _) => {
            h1579
        }
    }))
}

pub fn f421(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt136> {
    let m1580 = f260(v0, v2);
    match m1580 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v4) => {
            let m1581 = f494(v4);
            match m1581 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v5) => {
                    let m1584 = {
                        let a1582 = v5;
                        let a1583 = 0i64;
                        compare(a1582, a1583)
                    };
                    if match m1584 {
                        Ordering::Less => {
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
                            if f491(f495(f143(v3, 0i64)?, v5)) {
                                Ok(Adt136::C1)
                            } else {
                                Ok(Adt136::C0)
                            }
                        } else {
                            if f435(f495(f143(v3, 0i64)?, v5)) {
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

pub fn f422(v0: Str) -> R<u64> {
    f496(f110({
        let a1585 = v0;
        utf8_encode(a1585)
    })?, 0u64)
}

pub fn f423(p1590: List<Adt6>, p1591: u64) -> R<u64> {
    let m1589 = (p1590, p1591);
    {
        let mut m1589 = m1589;
        loop {
            let step = {
                let (v0, v1) = m1589;
                let m1586 = v0;
                match m1586.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1587 = v1;
                            let a1588 = 1u64;
                            nat_add(a1587, a1588)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1589 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424(p1596: List<(Str, Adt6)>, p1597: u64) -> R<u64> {
    let m1595 = (p1596, p1597);
    {
        let mut m1595 = m1595;
        loop {
            let step = {
                let (v0, v1) = m1595;
                let m1592 = v0;
                match m1592.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1593 = v1;
                            let a1594 = 1u64;
                            nat_add(a1593, a1594)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(p1601: List<Adt6>, v1: Adt5) -> Adt136 {
    let m1600 = p1601;
    {
        let mut m1600 = m1600;
        loop {
            let step = {
                let v0 = m1600;
                let m1598 = v0;
                match m1598.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt136>(Adt136::C0)
                    }
                    Some((v2, v3)) => {
                        let m1599 = f257(v2);
                        match m1599 {
                            None => {
                                Err::<List<Adt6>, Adt136>(Adt136::C2)
                            }
                            Some(v4) => {
                                if f261(v4, v1.clone()) {
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
                Ok(next) => { m1600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1602 = v1.clone();
    let a1603 = Str::lit("#");
    if equal(a1602, a1603) {
        Some(v0)
    } else {
        let m1604 = f221(v0);
        match m1604 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1605 = f202(Str::lit("$defs"), v2);
                match m1605 {
                    Adt130::C0 => {
                        None::<Adt6>
                    }
                    Adt130::C1(v3) => {
                        let m1606 = f221(v3);
                        match m1606 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f497(v1, match v4 {
                                    Adt5::C0(h1607) => {
                                        h1607
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

pub fn f427(p1614: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt99>, p1615: List<Adt101>) -> List<Adt101> {
    let m1613 = (p1614, p1615);
    {
        let mut m1613 = m1613;
        loop {
            let step = {
                let (v0, v3) = m1613;
                let m1608 = v0;
                match m1608.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1610 = f260({
                            let (h1609, _) = v4.clone();
                            h1609
                        }, v1.clone());
                        match m1610 {
                            Adt130::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v5, v3))
                            }
                            Adt130::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v5, List::cons(Adt101::C0({
                                    let (_, h1611) = v4;
                                    h1611
                                }, v6, List::cons(Adt99::C0({
                                    let (h1612, _) = v4;
                                    h1612
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1613 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f428(p1620: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1619 = p1620;
    {
        let mut m1619 = m1619;
        loop {
            let step = {
                let v0 = m1619;
                let m1616 = v0;
                match m1616.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1618 = f260({
                            let (h1617, _) = v2;
                            h1617
                        }, v1.clone());
                        match m1618 {
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
                Ok(next) => { m1619 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1621 = f260(Str::lit("properties"), v0);
    match m1621 {
        Adt130::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt130::C1(v1) => {
            let m1622 = f221(v1);
            match m1622 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1623) => {
                            h1623
                        }
                    }
                }
            }
        }
    }
}

pub fn f430(p1630: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>, p1631: List<Adt101>) -> List<Adt101> {
    let m1629 = (p1630, p1631);
    {
        let mut m1629 = m1629;
        loop {
            let step = {
                let (v0, v4) = m1629;
                let m1624 = v0;
                match m1624.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1626 = f260({
                            let (h1625, _) = v5.clone();
                            h1625
                        }, v1.clone());
                        match m1626 {
                            Adt130::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v6, List::cons(Adt101::C0(v2.clone(), {
                                    let (_, h1627) = v5;
                                    h1627
                                }, List::cons(Adt99::C0({
                                    let (h1628, _) = v5;
                                    h1628
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
                Ok(next) => { m1629 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f431(p1636: List<Adt6>, p1637: u64, v2: Adt6, v3: List<Adt99>, p1638: List<Adt101>) -> R<List<Adt101>> {
    let m1635 = (p1636, (p1637, p1638));
    {
        let mut m1635 = m1635;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1635;
                let m1632 = v0;
                match m1632.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt101>)), R<List<Adt101>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt101>)), R<List<Adt101>>>((v6, ({
                            let a1633 = v1;
                            let a1634 = 1u64;
                            nat_add(a1633, a1634)?
                        }, List::cons(Adt101::C0(v2.clone(), v5, List::cons(Adt99::C1(v1), v3.clone())), v4))))
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

pub fn f432(p1641: List<Adt6>, v1: Adt6, v2: List<Adt99>, p1642: List<Adt101>) -> List<Adt101> {
    let m1640 = (p1641, p1642);
    {
        let mut m1640 = m1640;
        loop {
            let step = {
                let (v0, v3) = m1640;
                let m1639 = v0;
                match m1639.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt101>), List<Adt101>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt101>), List<Adt101>>((v5, List::cons(Adt101::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1640 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1643 = f498(v0)?;
    match m1643 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1644 = f498(v1)?;
            match m1644 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1645 = v2;
                    let a1646 = 61u64;
                    if nat_eq(a1645, a1646) {
                        f499(v5, v6, v4, v3)
                    } else {
                        let m1647 = f498(v2)?;
                        match m1647 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1648 = v3;
                                let a1649 = 61u64;
                                if nat_eq(a1648, a1649) {
                                    f500(v5, v6, v7, v4)
                                } else {
                                    let m1650 = f498(v3)?;
                                    match m1650 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f501(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f434(v0: List<u64>) -> bool {
    let m1651 = v0;
    match m1651.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f435(v0: Adt147) -> bool {
    let m1652 = v0;
    match m1652 {
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

pub fn f436(p1657: Bytes, p1658: List<Bytes>) -> Bytes {
    let m1656 = (p1657, p1658);
    {
        let mut m1656 = m1656;
        loop {
            let step = {
                let (v0, v1) = m1656;
                let m1653 = v1;
                match m1653.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1654 = v2;
                            let a1655 = v0;
                            append_bytes(a1654, a1655)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(p1661: List<Bytes>, p1662: List<u64>) -> List<Bytes> {
    let m1660 = (p1661, p1662);
    {
        let mut m1660 = m1660;
        loop {
            let step = {
                let (v0, v1) = m1660;
                let m1659 = v1;
                match m1659.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f502(f503(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438() -> i64 {
    999999999999999999i64
}

pub fn f439(v0: i64) -> R<List<u64>> {
    let m1665 = {
        let a1663 = v0;
        let a1664 = 0i64;
        compare(a1663, a1664)
    };
    if match m1665 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f504(20u64, {
            let a1666 = v0;
            int_neg(a1666)?
        }, List::<u64>::nil())?))
    } else {
        f504(20u64, v0, List::<u64>::nil())
    }
}

pub fn f440(v0: u64) -> R<List<u64>> {
    let a1667 = v0;
    let a1668 = 34u64;
    if nat_eq(a1667, a1668) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1669 = v0;
        let a1670 = 92u64;
        if nat_eq(a1669, a1670) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1671 = v0;
            let a1672 = 8u64;
            if nat_eq(a1671, a1672) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1673 = v0;
                let a1674 = 9u64;
                if nat_eq(a1673, a1674) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1675 = v0;
                    let a1676 = 10u64;
                    if nat_eq(a1675, a1676) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1677 = v0;
                        let a1678 = 12u64;
                        if nat_eq(a1677, a1678) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1679 = v0;
                            let a1680 = 13u64;
                            if nat_eq(a1679, a1680) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1681 = v0;
                                let a1682 = 32u64;
                                if nat_lt(a1681, a1682) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f505(f480(v0, 16u64))?, List::cons(f505(f479(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f441(v0: List<Str>) -> Adt6 {
    Adt6::C4(f385(v0))
}

pub fn f442(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1700 = f463(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1683, _, _, _, _, _, _) => {
                h1683
            }
        });
        let a1701 = {
            let a1698 = f463(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1684, _, _, _, _, _) => {
                    h1684
                }
            });
            let a1699 = {
                let a1696 = f463(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1685, _, _, _, _) => {
                        h1685
                    }
                });
                let a1697 = {
                    let a1694 = f506(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1686, _, _, _) => {
                            h1686
                        }
                    });
                    let a1695 = {
                        let a1692 = f507(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1687, _, _) => {
                                h1687
                            }
                        });
                        let a1693 = {
                            let a1690 = f506(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1688, _) => {
                                    h1688
                                }
                            });
                            let a1691 = f463(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1689) => {
                                    h1689
                                }
                            });
                            append_list(a1690, a1691)
                        };
                        append_list(a1692, a1693)
                    };
                    append_list(a1694, a1695)
                };
                append_list(a1696, a1697)
            };
            append_list(a1698, a1699)
        };
        append_list(a1700, a1701)
    })
}

pub fn f443(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1702 = v1;
    match m1702 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f444(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f59(match v0.clone() {
        Adt10::C0(h1703, _) => {
            h1703
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f508(match v0 {
        Adt10::C0(_, h1704) => {
            h1704
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f445(v0: List<Adt35>) -> List<Adt6> {
    f320(f509(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f446(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1705 = f510(v0, List::<Adt6>::nil())?;
    match m1705 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f320(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f447(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1706 = v1;
    match m1706 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f448(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1707 = v1;
    match m1707 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f449() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f450(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1708, _, _, _, _, _) => {
            h1708
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1709, _, _, _) => {
            h1709
        }
    })), {
        let a1718 = f443(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1710, _, _, _, _) => {
                h1710
            }
        });
        let a1719 = {
            let a1716 = f443(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1711, _, _) => {
                    h1711
                }
            });
            let a1717 = {
                let a1714 = f443(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1712, _) => {
                        h1712
                    }
                });
                let a1715 = f511(match v0 {
                    Adt1::C0(_, _, _, _, _, h1713) => {
                        h1713
                    }
                });
                append_list(a1714, a1715)
            };
            append_list(a1716, a1717)
        };
        append_list(a1718, a1719)
    })))
}

pub fn f451(v0: List<List<u64>>) -> bool {
    let m1720 = v0;
    match m1720.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f452() -> List<Adt86> {
    List::cons(Adt86::C0(f512(), f388()), List::cons(Adt86::C0(f513(), f514()), List::cons(Adt86::C0(f515(), f516()), List::<Adt86>::nil())))
}

pub fn f453(v0: List<List<u64>>) -> List<u64> {
    f517(f45(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f454(p1735: u64, p1736: Adt144, p1737: List<Adt145>) -> R<Option<Adt6>> {
    let m1734 = (p1735, (p1736, p1737));
    {
        let mut m1734 = m1734;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1734;
                let m1721 = v0;
                if m1721 == 0 {
                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1721 - 1;
                    let m1722 = v1;
                    match m1722 {
                        Adt144::C0(v4) => {
                            let m1723 = v4.clone();
                            match m1723 {
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
                                    let v8: Str = f518(v7)?;
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1724 = v9;
                                    match m1724.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v10), List::cons(Adt145::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1725 = v12;
                                    match m1725.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f518({
                                                let (h1726, _) = v13;
                                                h1726
                                            })?;
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                let (_, h1727) = v13;
                                                h1727
                                            }), List::cons(Adt145::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt144::C1(v16) => {
                            let m1728 = v2;
                            match m1728.uncons() {
                                None => {
                                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1729 = v17;
                                    match m1729 {
                                        Adt145::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1730 = v20;
                                            match m1730.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C4(f320(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v22), List::cons(Adt145::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt145::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1731 = v26;
                                            match m1731.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C5(f322(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f518({
                                                        let (h1732, _) = v28;
                                                        h1732
                                                    })?;
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                        let (_, h1733) = v28;
                                                        h1733
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
                Ok(next) => { m1734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455() -> u64 {
    67108864u64
}

pub fn f456(v0: Adt19) -> Adt6 {
    let m1738 = v0;
    match m1738 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f457(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1739 = v0;
    match m1739 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f458(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1740 = v0;
    match m1740 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f459() -> i64 {
    -32603i64
}

pub fn f460() -> i64 {
    429i64
}

pub fn f461() -> i64 {
    -32021i64
}

pub fn f462(p1743: List<Str>, p1744: List<Adt6>) -> List<Adt6> {
    let m1742 = (p1743, p1744);
    {
        let mut m1742 = m1742;
        loop {
            let step = {
                let (v0, v1) = m1742;
                let m1741 = v0;
                match m1741.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1742 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1745 = v1;
    match m1745 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1746) => {
                    h1746
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f464(v0: List<u64>) -> R<bool> {
    let m1747 = v0;
    match m1747.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1748 = v2;
            match m1748.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1749 = f210(v1)?;
                    let a1750 = 113u64;
                    if if nat_eq(a1749, a1750) {
                        let a1751 = v3;
                        let a1752 = 61u64;
                        nat_eq(a1751, a1752)
                    } else {
                        false
                    } {
                        Ok(f519(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f465(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt127>) -> R<Result<Adt123, Adt126>> {
    let m1753 = v3;
    match m1753 {
        Ok(v5) => {
            f520(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt123, Adt126>(Adt126::C0(v4)))
        }
    }
}

pub fn f466(v0: Adt134, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt127> {
    let m1754 = v0;
    match m1754 {
        Adt134::C0 => {
            f521(v1, v2)
        }
        Adt134::C1 => {
            f522(v1, v2)
        }
        Adt134::C2 => {
            f523(v1, v2)
        }
        Adt134::C3 => {
            f524(v1, v2)
        }
        Adt134::C4 => {
            f525(v1, v2)
        }
        Adt134::C5 => {
            f526(v1, v2)
        }
        Adt134::C6 => {
            f527(v1, v2)
        }
        Adt134::C7 => {
            f528(v1, v2)
        }
        Adt134::C8 => {
            f529(v1, v2)
        }
        Adt134::C9 => {
            f530(v1, v2)
        }
        Adt134::C10 => {
            f531(v1, v2)
        }
        Adt134::C11 => {
            f532(v1, v2)
        }
    }
}

pub fn f467(v0: Adt134) -> Str {
    let m1755 = v0;
    match m1755 {
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

pub fn f468(v0: Option<Adt8>) -> bool {
    let m1756 = v0;
    match m1756 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f469(v0: Option<Adt9>) -> bool {
    let m1757 = v0;
    match m1757 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f470(v0: Option<Adt5>) -> bool {
    let m1758 = v0;
    match m1758 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f471(v0: Option<Adt5>, v1: Str) -> bool {
    let m1759 = v0;
    match m1759 {
        None => {
            false
        }
        Some(v2) => {
            f261(v1, v2)
        }
    }
}

pub fn f472() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f473(v0: List<Str>, v1: Adt103, v2: Adt16, v3: Option<Adt108>) -> Adt138 {
    let m1760 = v3;
    match m1760 {
        None => {
            f533(v1, v2.clone(), f534(match v2 {
                Adt16::C0(h1761, _, _, _, _, _) => {
                    h1761
                }
            }, v0))
        }
        Some(v4) => {
            Adt138::C1(Adt126::C2(v4))
        }
    }
}

pub fn f474(v0: Option<Adt107>, v1: Adt140, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt108>> {
    let m1762 = v0;
    match m1762 {
        None => {
            Ok(None::<Adt108>)
        }
        Some(v5) => {
            let m1764 = match v5.clone() {
                Adt107::C0(_, _, _, h1763, _) => {
                    h1763
                }
            };
            match m1764 {
                None => {
                    f535(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f475(v0: Option<Adt5>) -> Adt5 {
    let m1765 = v0;
    match m1765 {
        None => {
            f144()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f476(v0: Adt5) -> Result<Adt16, Adt127> {
    let m1766 = f536(v0.clone());
    match m1766 {
        Adt148::C0 => {
            f537(v0.clone(), f202(f538(), v0.clone()), f202(f539(), v0))
        }
        Adt148::C1 => {
            Err::<Adt16, Adt127>(Adt127::C2)
        }
        Adt148::C2 => {
            Err::<Adt16, Adt127>(Adt127::C3)
        }
    }
}

pub fn f477() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f478(v0: Str) -> R<List<u64>> {
    f110({
        let a1767 = v0;
        utf8_encode(a1767)
    })
}

pub fn f479(v0: u64, v1: u64) -> u64 {
    let a1768 = v0;
    let a1769 = v1;
    let a1770 = 0u64;
    nat_rem(a1768, a1769, a1770)
}

pub fn f480(v0: u64, v1: u64) -> u64 {
    let a1771 = v0;
    let a1772 = v1;
    let a1773 = 0u64;
    nat_quot(a1771, a1772, a1773)
}

pub fn f481(v0: u64, v1: Adt6) -> bool {
    let a1774 = v0;
    let a1775 = 0u64;
    if nat_eq(a1774, a1775) {
        f540(v1)
    } else {
        let a1776 = v0;
        let a1777 = 1u64;
        if nat_eq(a1776, a1777) {
            f541(v1)
        } else {
            let a1778 = v0;
            let a1779 = 2u64;
            if nat_eq(a1778, a1779) {
                f542(v1)
            } else {
                let a1780 = v0;
                let a1781 = 3u64;
                if nat_eq(a1780, a1781) {
                    f543(v1)
                } else {
                    let a1782 = v0;
                    let a1783 = 4u64;
                    if nat_eq(a1782, a1783) {
                        f544(v1)
                    } else {
                        let a1784 = v0;
                        let a1785 = 5u64;
                        if nat_eq(a1784, a1785) {
                            f545(v1)
                        } else {
                            let a1786 = v0;
                            let a1787 = 6u64;
                            if nat_eq(a1786, a1787) {
                                f546(v1)
                            } else {
                                let a1788 = v0;
                                let a1789 = 7u64;
                                if nat_eq(a1788, a1789) {
                                    f547(v1)
                                } else {
                                    f548(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f482(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt120) -> R<Adt142> {
    let m1790 = v3.clone();
    match m1790.uncons() {
        None => {
            Ok(Adt142::C0(Adt91::C8(v0)))
        }
        Some((_, _)) => {
            if f214(v3.clone(), true) {
                let m1791 = f549(v1);
                match m1791 {
                    None => {
                        Ok(Adt142::C0(Adt91::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f550(v3)?;
                        if f551(v9.clone(), match v5.clone() {
                            Adt120::C0(_, h1792) => {
                                h1792
                            }
                        }) {
                            Ok(Adt142::C0(Adt91::C12(v0, v2)))
                        } else {
                            Ok(Adt142::C1(Adt120::C0(List::cons(Adt12::C0(v0, v2, f552(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt120::C0(h1793, _) => {
                                    h1793
                                }
                            }), List::cons(v9, match v5 {
                                Adt120::C0(_, h1794) => {
                                    h1794
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

pub fn f483(p1799: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1800: List<Adt104>) -> List<Adt104> {
    let m1798 = (p1799, p1800);
    {
        let mut m1798 = m1798;
        loop {
            let step = {
                let (v0, v3) = m1798;
                let m1795 = v0;
                match m1795.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v5, List::cons(Adt104::C0({
                            let (_, h1796) = v4;
                            h1796
                        }, v1, List::cons({
                            let (h1797, _) = v4;
                            h1797
                        }, v2.clone())), v3)))
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

pub fn f484(p1804: List<(Str, Adt6)>, p1805: List<Adt104>) -> List<Adt104> {
    let m1803 = (p1804, p1805);
    {
        let mut m1803 = m1803;
        loop {
            let step = {
                let (v0, v1) = m1803;
                let m1801 = v0;
                match m1801.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0({
                            let (_, h1802) = v2;
                            h1802
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1803 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(p1808: List<Adt6>, p1809: List<Adt104>) -> List<Adt104> {
    let m1807 = (p1808, p1809);
    {
        let mut m1807 = m1807;
        loop {
            let step = {
                let (v0, v1) = m1807;
                let m1806 = v0;
                match m1806.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f486(v0: Adt6) -> Str {
    let m1810 = v0;
    match m1810 {
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

pub fn f487(v0: Adt7) -> R<bool> {
    if f400(match v0.clone() {
        Adt7::C0(h1811, _) => {
            h1811
        }
    }) {
        Ok(true)
    } else {
        f553(40u64, match v0.clone() {
            Adt7::C0(h1812, _) => {
                h1812
            }
        }, match v0 {
            Adt7::C0(_, h1813) => {
                h1813
            }
        })
    }
}

pub fn f488(v0: Adt6) -> Str {
    let m1814 = f257(v0);
    match m1814 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f489(p1821: u64, p1822: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m1820 = (p1821, p1822);
    {
        let mut m1820 = m1820;
        loop {
            let step = {
                let (v0, v1) = m1820;
                let m1815 = v0;
                if m1815 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m1815 - 1;
                    let m1816 = v1;
                    match m1816.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m1819 = f554({
                                let (h1817, _) = v3;
                                h1817
                            }, {
                                let (_, h1818) = v3;
                                h1818
                            }, v4)?;
                            match m1819 {
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
                Ok(next) => { m1820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490() -> u64 {
    67108864u64
}

pub fn f491(v0: Adt147) -> bool {
    let m1823 = v0;
    match m1823 {
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

pub fn f492(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m1824 = v2;
    match m1824 {
        Adt147::C0 => {
            let m1826 = v3;
            match m1826 {
                Adt147::C0 => {
                    Ok(f555(Adt147::C0, f556(f557(match v0.clone() {
                        Adt7::C0(h1827, _) => {
                            h1827
                        }
                    })?, match v0 {
                        Adt7::C0(_, h1828) => {
                            h1828
                        }
                    }, f557(match v1.clone() {
                        Adt7::C0(h1829, _) => {
                            h1829
                        }
                    })?, match v1 {
                        Adt7::C0(_, h1830) => {
                            h1830
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
            let m1825 = v3;
            match m1825 {
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
            let m1831 = v3;
            match m1831 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f556(match v0.clone() {
                        Adt7::C0(h1832, _) => {
                            h1832
                        }
                    }, match v0 {
                        Adt7::C0(_, h1833) => {
                            h1833
                        }
                    }, match v1.clone() {
                        Adt7::C0(h1834, _) => {
                            h1834
                        }
                    }, match v1 {
                        Adt7::C0(_, h1835) => {
                            h1835
                        }
                    })
                }
            }
        }
    }
}

pub fn f493(v0: i64) -> Adt147 {
    f495(v0, 0i64)
}

pub fn f494(v0: Adt6) -> Option<i64> {
    let m1836 = v0;
    match m1836 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f400(match v2.clone() {
                Adt7::C0(_, h1837) => {
                    h1837
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1838, _) => {
                        h1838
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

pub fn f495(v0: i64, v1: i64) -> Adt147 {
    let m1841 = {
        let a1839 = v0;
        let a1840 = v1;
        compare(a1839, a1840)
    };
    if match m1841 {
        Ordering::Less => {
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
        let m1844 = {
            let a1842 = v1;
            let a1843 = v0;
            compare(a1842, a1843)
        };
        if match m1844 {
            Ordering::Less => {
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

pub fn f496(p1853: List<u64>, p1854: u64) -> R<u64> {
    let m1852 = (p1853, p1854);
    {
        let mut m1852 = m1852;
        loop {
            let step = {
                let (v0, v1) = m1852;
                let m1845 = v0;
                match m1845.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a1846 = v2;
                        let a1847 = 128u64;
                        if if nat_lt(a1846, a1847) {
                            true
                        } else {
                            let a1848 = 191u64;
                            let a1849 = v2;
                            nat_lt(a1848, a1849)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a1850 = v1;
                                let a1851 = 1u64;
                                nat_add(a1850, a1851)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
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

pub fn f497(v0: Str, p1872: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1871 = p1872;
    {
        let mut m1871 = m1871;
        loop {
            let step = {
                let v1 = m1871;
                let m1855 = v1;
                match m1855.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a1859 = v0.clone();
                        let a1860 = {
                            let a1857 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h1856, _) = v2.clone();
                                h1856
                            }, List::<Str>::nil()));
                            let a1858 = Str::lit("");
                            join(a1857, a1858)
                        };
                        if equal(a1859, a1860) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h1861) = v2;
                                h1861
                            }))
                        } else {
                            let m1863 = f221({
                                let (_, h1862) = v2.clone();
                                h1862
                            });
                            match m1863 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m1864 = f202(Str::lit("$anchor"), v4);
                                    match m1864 {
                                        Adt130::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt130::C1(v5) => {
                                            let m1865 = f257(v5);
                                            match m1865 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a1868 = v0.clone();
                                                    let a1869 = {
                                                        let a1866 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a1867 = Str::lit("");
                                                        join(a1866, a1867)
                                                    };
                                                    if equal(a1868, a1869) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h1870) = v2;
                                                            h1870
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
                Ok(next) => { m1871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: u64) -> R<Option<u64>> {
    let a1873 = 64u64;
    let a1874 = v0;
    if if nat_lt(a1873, a1874) {
        let a1875 = v0;
        let a1876 = 91u64;
        nat_lt(a1875, a1876)
    } else {
        false
    } {
        Ok(Some({
            let a1877 = v0;
            let a1878 = 65u64;
            nat_sub(a1877, a1878)
        }))
    } else {
        let a1879 = 96u64;
        let a1880 = v0;
        if if nat_lt(a1879, a1880) {
            let a1881 = v0;
            let a1882 = 123u64;
            nat_lt(a1881, a1882)
        } else {
            false
        } {
            Ok(Some({
                let a1883 = v0;
                let a1884 = 71u64;
                nat_sub(a1883, a1884)
            }))
        } else {
            let a1885 = 47u64;
            let a1886 = v0;
            if if nat_lt(a1885, a1886) {
                let a1887 = v0;
                let a1888 = 58u64;
                nat_lt(a1887, a1888)
            } else {
                false
            } {
                Ok(Some({
                    let a1889 = v0;
                    let a1890 = 4u64;
                    nat_add(a1889, a1890)?
                }))
            } else {
                let a1891 = v0;
                let a1892 = 43u64;
                if nat_eq(a1891, a1892) {
                    Ok(Some(62u64))
                } else {
                    let a1893 = v0;
                    let a1894 = 47u64;
                    if nat_eq(a1893, a1894) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f499(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1895 = v3;
        let a1896 = 61u64;
        if nat_eq(a1895, a1896) {
            let a1897 = f558(v1, 16u64);
            let a1898 = 0u64;
            nat_eq(a1897, a1898)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f559(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f500(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1899 = f558(v2, 4u64);
        let a1900 = 0u64;
        nat_eq(a1899, a1900)
    } else {
        false
    } {
        Ok(Some(f560(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f501(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1903 = {
            let a1901 = v0;
            let a1902 = 4u64;
            nat_mul(a1901, a1902)?
        };
        let a1904 = f561(v1, 16u64);
        nat_add(a1903, a1904)?
    }, List::cons({
        let a1907 = {
            let a1905 = f558(v1, 16u64);
            let a1906 = 16u64;
            nat_mul(a1905, a1906)?
        };
        let a1908 = f561(v2, 4u64);
        nat_add(a1907, a1908)?
    }, List::cons({
        let a1911 = {
            let a1909 = f558(v2, 4u64);
            let a1910 = 64u64;
            nat_mul(a1909, a1910)?
        };
        let a1912 = v3;
        nat_add(a1911, a1912)?
    }, List::<u64>::nil()))))
}

pub fn f502(p1921: Bytes, p1922: List<Bytes>) -> List<Bytes> {
    let m1920 = (p1921, p1922);
    {
        let mut m1920 = m1920;
        loop {
            let step = {
                let (v0, v1) = m1920;
                let m1913 = v1;
                match m1913.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1916 = {
                            let a1914 = v2.clone();
                            length_bytes(a1914)
                        };
                        let a1917 = {
                            let a1915 = v0.clone();
                            length_bytes(a1915)
                        };
                        if nat_eq(a1916, a1917) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1918 = v2;
                                let a1919 = v0;
                                append_bytes(a1918, a1919)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
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

pub fn f503(v0: u64) -> Bytes {
    let m1926 = {
        let a1923 = f562();
        let a1924 = v0;
        let a1925 = 1u64;
        slice_bytes(a1923, a1924, a1925)
    };
    match m1926 {
        None => {
            f358()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f504(p1938: u64, p1939: i64, p1940: List<u64>) -> R<List<u64>> {
    let m1937 = (p1938, (p1939, p1940));
    {
        let mut m1937 = m1937;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1937;
                let m1927 = v0;
                if m1927 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1927 - 1;
                    let v4: i64 = {
                        let a1928 = v1;
                        let a1929 = 10i64;
                        let a1930 = 0i64;
                        int_quot(a1928, a1929, a1930)?
                    };
                    let v5: i64 = {
                        let a1931 = v1;
                        let a1932 = 10i64;
                        let a1933 = 0i64;
                        int_rem(a1931, a1932, a1933)
                    };
                    let m1936 = {
                        let a1934 = v4;
                        let a1935 = 1i64;
                        compare(a1934, a1935)
                    };
                    if match m1936 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f563(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f563(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1937 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(v0: u64) -> R<u64> {
    let a1941 = v0;
    let a1942 = 10u64;
    if nat_lt(a1941, a1942) {
        let a1943 = 48u64;
        let a1944 = v0;
        nat_add(a1943, a1944)
    } else {
        let a1945 = 87u64;
        let a1946 = v0;
        nat_add(a1945, a1946)
    }
}

pub fn f506(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1947 = v1;
    match m1947 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f564(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f507(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1948 = v0;
    match m1948 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1951 = f448(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1949, _) => {
                        h1949
                    }
                });
                let a1952 = f448(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1950) => {
                        h1950
                    }
                });
                append_list(a1951, a1952)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f508(v0: Adt11) -> Str {
    let m1953 = v0;
    match m1953 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f509(p1956: List<Adt35>, p1957: List<Adt6>) -> List<Adt6> {
    let m1955 = (p1956, p1957);
    {
        let mut m1955 = m1955;
        loop {
            let step = {
                let (v0, v1) = m1955;
                let m1954 = v0;
                match m1954.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f565(v2), v1)))
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

pub fn f510(p1961: List<Adt38>, p1962: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1960 = (p1961, p1962);
    {
        let mut m1960 = m1960;
        loop {
            let step = {
                let (v0, v1) = m1960;
                let m1958 = v0;
                match m1958.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1959 = f566(v2)?;
                        match m1959 {
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
                Ok(next) => { m1960 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1963 = v0;
    match m1963 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f567(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f512() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f513() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f514() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f515() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f516() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f517(p1968: List<List<u64>>, p1969: List<u64>) -> List<u64> {
    let m1967 = (p1968, p1969);
    {
        let mut m1967 = m1967;
        loop {
            let step = {
                let (v0, v1) = m1967;
                let m1964 = v0;
                match m1964.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a1965 = f568(v2);
                            let a1966 = v1;
                            append_list(a1965, a1966)
                        }))
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

pub fn f518(v0: Str) -> R<Str> {
    let m1972 = {
        let a1971 = f357(f358(), f189(f569(f110({
            let a1970 = v0;
            utf8_encode(a1970)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1971)
    };
    match m1972 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f519(v0: List<u64>) -> bool {
    let m1973 = v0;
    match m1973.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a1974 = v1;
            let a1975 = 48u64;
            if nat_eq(a1974, a1975) {
                f570(v2)
            } else {
                false
            }
        }
    }
}

pub fn f520(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt123, Adt126>> {
    if f571(f572(v3.clone())) {
        Ok(Ok::<Adt123, Adt126>(f573(v0.clone(), Adt32::C0(f574(v0)))))
    } else {
        Ok(f575(v0.clone(), v3.clone(), f576(match v0 {
            Adt0::C0(_, _, _, _, _, h1976) => {
                h1976
            }
        }, v2, v1, v3)?))
    }
}

pub fn f521(v0: Adt16, _: Adt5) -> Result<Adt14, Adt127> {
    Ok::<Adt14, Adt127>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f522(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1977 = f577(Str::lit("cursor"), v1);
    match m1977 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f523(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1978 = f578(Str::lit("name"), v1.clone());
    match m1978 {
        Ok(v2) => {
            let m1979 = f579(Str::lit("arguments"), v1.clone());
            match m1979 {
                Ok(v3) => {
                    let m1980 = f579(Str::lit("inputResponses"), v1.clone());
                    match m1980 {
                        Ok(v4) => {
                            let m1981 = f577(Str::lit("requestState"), v1);
                            match m1981 {
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

pub fn f524(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1982 = f577(Str::lit("cursor"), v1);
    match m1982 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f525(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1983 = f578(Str::lit("uri"), v1.clone());
    match m1983 {
        Ok(v2) => {
            let m1984 = f579(Str::lit("inputResponses"), v1.clone());
            match m1984 {
                Ok(v3) => {
                    let m1985 = f577(Str::lit("requestState"), v1);
                    match m1985 {
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

pub fn f526(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1986 = f577(Str::lit("cursor"), v1);
    match m1986 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f527(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1987 = f577(Str::lit("cursor"), v1);
    match m1987 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f528(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1988 = f578(Str::lit("name"), v1.clone());
    match m1988 {
        Ok(v2) => {
            let m1989 = f580(Str::lit("arguments"), v1.clone());
            match m1989 {
                Ok(v3) => {
                    let m1990 = f579(Str::lit("inputResponses"), v1.clone());
                    match m1990 {
                        Ok(v4) => {
                            let m1991 = f577(Str::lit("requestState"), v1);
                            match m1991 {
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

pub fn f529(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1992 = f581(Str::lit("ref"), v1.clone());
    match m1992 {
        Ok(v2) => {
            let m1993 = f582(v2);
            match m1993 {
                Ok(v3) => {
                    let m1994 = f581(Str::lit("argument"), v1.clone());
                    match m1994 {
                        Ok(v4) => {
                            let m1995 = f583(v4);
                            match m1995 {
                                Ok(v5) => {
                                    let m1996 = f584(v1);
                                    match m1996 {
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

pub fn f530(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1997 = f578(Str::lit("taskId"), v1);
    match m1997 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f531(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1998 = f578(Str::lit("taskId"), v1.clone());
    match m1998 {
        Ok(v2) => {
            let m1999 = f581(Str::lit("inputResponses"), v1);
            match m1999 {
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

pub fn f532(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2000 = f578(Str::lit("taskId"), v1);
    match m2000 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f533(v0: Adt103, v1: Adt16, v2: Option<Adt126>) -> Adt138 {
    let m2001 = v2;
    match m2001 {
        None => {
            Adt138::C0(Adt139::C0(match v0.clone() {
                Adt103::C0(h2002, _, _) => {
                    h2002
                }
            }, match v0.clone() {
                Adt103::C0(_, h2003, _) => {
                    h2003
                }
            }, v1, f475(match v0 {
                Adt103::C0(_, _, h2004) => {
                    h2004
                }
            })))
        }
        Some(v3) => {
            Adt138::C1(v3)
        }
    }
}

pub fn f534(v0: Str, v1: List<Str>) -> Option<Adt126> {
    f585(f586(v0.clone(), v1.clone()), v0, v1)
}

pub fn f535(v0: Adt107, v1: Adt140, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt108>> {
    let m2006 = match v0.clone() {
        Adt107::C0(h2005, _, _, _, _) => {
            h2005
        }
    };
    match m2006 {
        None => {
            Ok(Some(Adt108::C0))
        }
        Some(v5) => {
            let a2007 = v5;
            let a2008 = v3;
            if equal(a2007, a2008) {
                let m2010 = match v0.clone() {
                    Adt107::C0(_, h2009, _, _, _) => {
                        h2009
                    }
                };
                match m2010 {
                    None => {
                        Ok(Some(Adt108::C1))
                    }
                    Some(v6) => {
                        let a2011 = v6;
                        let a2012 = v2;
                        if equal(a2011, a2012) {
                            f587(f588(match v0.clone() {
                                Adt107::C0(_, _, h2013, _, _) => {
                                    h2013
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt107::C0(_, _, _, _, h2014) => {
                                    h2014
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

pub fn f536(v0: Adt5) -> Adt148 {
    f589(f202(f538(), v0.clone()), f202(f539(), v0))
}

pub fn f537(v0: Adt5, v1: Adt130, v2: Adt130) -> Result<Adt16, Adt127> {
    let m2015 = v1;
    match m2015 {
        Adt130::C0 => {
            Err::<Adt16, Adt127>(Adt127::C2)
        }
        Adt130::C1(v3) => {
            let m2016 = v2;
            match m2016 {
                Adt130::C0 => {
                    Err::<Adt16, Adt127>(Adt127::C3)
                }
                Adt130::C1(v4) => {
                    f590(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f538() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f539() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f540(v0: Adt6) -> bool {
    let m2017 = f257(v0);
    match m2017 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f541(v0: Adt6) -> bool {
    let m2018 = f418(v0);
    match m2018 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f542(v0: Adt6) -> bool {
    let m2019 = f494(v0);
    match m2019 {
        None => {
            false
        }
        Some(v1) => {
            let a2023 = {
                let m2022 = {
                    let a2020 = v1;
                    let a2021 = 0i64;
                    compare(a2020, a2021)
                };
                match m2022 {
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
            bool_not(a2023)
        }
    }
}

pub fn f543(v0: Adt6) -> bool {
    let m2024 = f285(v0);
    match m2024 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f544(v0: Adt6) -> bool {
    let m2025 = f221(v0);
    match m2025 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f545(v0: Adt6) -> bool {
    let m2026 = f285(v0);
    match m2026 {
        None => {
            false
        }
        Some(v1) => {
            f591(v1)
        }
    }
}

pub fn f546(v0: Adt6) -> bool {
    let m2027 = f220(v0);
    match m2027 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f547(v0: Adt6) -> bool {
    let m2028 = f257(v0.clone());
    match m2028 {
        None => {
            let m2029 = f285(v0);
            match m2029 {
                None => {
                    false
                }
                Some(v2) => {
                    f592(v2)
                }
            }
        }
        Some(v1) => {
            f413(v1)
        }
    }
}

pub fn f548(v0: Adt6) -> bool {
    let m2030 = f285(v0);
    match m2030 {
        None => {
            false
        }
        Some(v1) => {
            let m2031 = v1;
            match m2031.uncons() {
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

pub fn f549(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2032 = f260(Str::lit("type"), v0);
    match m2032 {
        Adt130::C0 => {
            None::<Adt13>
        }
        Adt130::C1(v1) => {
            let m2033 = f257(v1);
            match m2033 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f593(v2)
                }
            }
        }
    }
}

pub fn f550(v0: List<u64>) -> R<List<u64>> {
    Ok(f189(f594(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f551(v0: List<u64>, p2036: List<List<u64>>) -> bool {
    let m2035 = p2036;
    {
        let mut m2035 = m2035;
        loop {
            let step = {
                let v1 = m2035;
                let m2034 = v1;
                match m2034.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f595(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552(p2039: List<Str>, p2040: List<Str>) -> List<Str> {
    let m2038 = (p2039, p2040);
    {
        let mut m2038 = m2038;
        loop {
            let step = {
                let (v0, v1) = m2038;
                let m2037 = v0;
                match m2037.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
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

pub fn f553(p2052: u64, p2053: i64, p2054: i64) -> R<bool> {
    let m2051 = (p2052, (p2053, p2054));
    {
        let mut m2051 = m2051;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2051;
                let m2041 = v0;
                if m2041 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2041 - 1;
                    let m2044 = {
                        let a2042 = v2;
                        let a2043 = 0i64;
                        compare(a2042, a2043)
                    };
                    if match m2044 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f400({
                            let a2045 = v1;
                            let a2046 = 10i64;
                            let a2047 = 0i64;
                            int_rem(a2045, a2046, a2047)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2048 = v1;
                                let a2049 = 10i64;
                                let a2050 = 0i64;
                                int_quot(a2048, a2049, a2050)?
                            }, f187(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2051 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2055 = f596(v0.clone());
    let a2056 = f596(v1.clone());
    if nat_eq(a2055, a2056) {
        let m2057 = v0;
        match m2057 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2058 = f220(v1);
                match m2058 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2059 = v3;
                        let a2060 = v4;
                        if equal(a2059, a2060) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2061 = f418(v1);
                match m2061 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f597(f420(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2062 = f257(v1);
                match m2062 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2063 = v7;
                        let a2064 = v8;
                        if equal(a2063, a2064) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2065 = f285(v1);
                match m2065 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f598(f599(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2066 = f221(v1);
                match m2066 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2068 = f424(v11.clone(), 0u64)?;
                        let a2069 = f424(match v12.clone() {
                            Adt5::C0(h2067) => {
                                h2067
                            }
                        }, 0u64)?;
                        if nat_eq(a2068, a2069) {
                            Ok(f598(f600(v11, match v12 {
                                Adt5::C0(h2070) => {
                                    h2070
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

pub fn f555(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2071 = v0;
    match m2071 {
        Adt147::C0 => {
            f601(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f556(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2072 = f495(f602(v0, v1)?, f602(v2, v3)?);
    match m2072 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f603(v0, f604(v0)?, v2, f604(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f557(v0: i64) -> R<i64> {
    let m2075 = {
        let a2073 = v0;
        let a2074 = 0i64;
        compare(a2073, a2074)
    };
    if match m2075 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2076 = v0;
        int_neg(a2076)
    } else {
        Ok(v0)
    }
}

pub fn f558(v0: u64, v1: u64) -> u64 {
    let a2077 = v0;
    let a2078 = v1;
    let a2079 = 0u64;
    nat_rem(a2077, a2078, a2079)
}

pub fn f559(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2082 = {
            let a2080 = v0;
            let a2081 = 4u64;
            nat_mul(a2080, a2081)?
        };
        let a2083 = f561(v1, 16u64);
        nat_add(a2082, a2083)?
    }, List::<u64>::nil()))
}

pub fn f560(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2086 = {
            let a2084 = v0;
            let a2085 = 4u64;
            nat_mul(a2084, a2085)?
        };
        let a2087 = f561(v1, 16u64);
        nat_add(a2086, a2087)?
    }, List::cons({
        let a2090 = {
            let a2088 = f558(v1, 16u64);
            let a2089 = 16u64;
            nat_mul(a2088, a2089)?
        };
        let a2091 = f561(v2, 4u64);
        nat_add(a2090, a2091)?
    }, List::<u64>::nil())))
}

pub fn f561(v0: u64, v1: u64) -> u64 {
    let a2092 = v0;
    let a2093 = v1;
    let a2094 = 0u64;
    nat_quot(a2092, a2093, a2094)
}

pub fn f562() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f563(v0: i64) -> u64 {
    let m2097 = {
        let a2095 = v0;
        let a2096 = 1i64;
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
        48u64
    } else {
        let m2100 = {
            let a2098 = v0;
            let a2099 = 2i64;
            compare(a2098, a2099)
        };
        if match m2100 {
            Ordering::Less => {
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
            let m2103 = {
                let a2101 = v0;
                let a2102 = 3i64;
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
                50u64
            } else {
                let m2106 = {
                    let a2104 = v0;
                    let a2105 = 4i64;
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
                    51u64
                } else {
                    let m2109 = {
                        let a2107 = v0;
                        let a2108 = 5i64;
                        compare(a2107, a2108)
                    };
                    if match m2109 {
                        Ordering::Less => {
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
                        let m2112 = {
                            let a2110 = v0;
                            let a2111 = 6i64;
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
                            53u64
                        } else {
                            let m2115 = {
                                let a2113 = v0;
                                let a2114 = 7i64;
                                compare(a2113, a2114)
                            };
                            if match m2115 {
                                Ordering::Less => {
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
                                let m2118 = {
                                    let a2116 = v0;
                                    let a2117 = 8i64;
                                    compare(a2116, a2117)
                                };
                                if match m2118 {
                                    Ordering::Less => {
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
                                    let m2121 = {
                                        let a2119 = v0;
                                        let a2120 = 9i64;
                                        compare(a2119, a2120)
                                    };
                                    if match m2121 {
                                        Ordering::Less => {
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

pub fn f564(v0: Adt8) -> Adt6 {
    Adt6::C5(f448(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2122) => {
            h2122
        }
    }))
}

pub fn f565(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2123, _, _, _, _, _, _, _) => {
            h2123
        }
    })), {
        let a2141 = f443(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2124, _, _, _, _, _, _) => {
                h2124
            }
        });
        let a2142 = {
            let a2139 = f511(match v0.clone() {
                Adt35::C0(_, _, h2125, _, _, _, _, _) => {
                    h2125
                }
            });
            let a2140 = {
                let a2137 = f443(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2126, _, _, _, _) => {
                        h2126
                    }
                });
                let a2138 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2129 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2128, _, _, _) => {
                            h2128
                        }
                    };
                    match m2129 {
                        Adt5::C0(h2127) => {
                            h2127
                        }
                    }
                })), {
                    let a2135 = f463(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2130, _, _) => {
                            h2130
                        }
                    });
                    let a2136 = {
                        let a2133 = f605(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2131, _) => {
                                h2131
                            }
                        });
                        let a2134 = f606(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2132) => {
                                h2132
                            }
                        });
                        append_list(a2133, a2134)
                    };
                    append_list(a2135, a2136)
                });
                append_list(a2137, a2138)
            };
            append_list(a2139, a2140)
        };
        append_list(a2141, a2142)
    }))
}

pub fn f566(v0: Adt38) -> R<Option<Adt6>> {
    let m2143 = v0;
    match m2143 {
        Adt38::C0(v1) => {
            Ok(Some(f607(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f608(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2144, _, _, _) => {
                    h2144
                }
            }, match v2.clone() {
                Adt42::C0(_, h2145, _, _) => {
                    h2145
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2146, _) => {
                    h2146
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2147) => {
                    h2147
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f608(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2148, _, _, _) => {
                    h2148
                }
            }, match v3.clone() {
                Adt43::C0(_, h2149, _, _) => {
                    h2149
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2150, _) => {
                    h2150
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2151) => {
                    h2151
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f609(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f610(v5)))
        }
    }
}

pub fn f567(v0: List<Adt2>) -> List<Adt6> {
    f320(f611(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f568(v0: List<u64>) -> List<u64> {
    let a2154 = f612();
    let a2155 = {
        let a2152 = v0;
        let a2153 = f613();
        append_list(a2152, a2153)
    };
    append_list(a2154, a2155)
}

pub fn f569(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f614(v0, 0u64, v1)
}

pub fn f570(p2162: List<u64>) -> bool {
    let m2161 = p2162;
    {
        let mut m2161 = m2161;
        loop {
            let step = {
                let v0 = m2161;
                let m2156 = v0;
                match m2156.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2157 = v1;
                        let a2158 = 48u64;
                        if if nat_eq(a2157, a2158) {
                            true
                        } else {
                            let a2159 = v1;
                            let a2160 = 46u64;
                            nat_eq(a2159, a2160)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f571(v0: Adt134) -> bool {
    let m2163 = v0;
    match m2163 {
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

pub fn f572(v0: Adt14) -> Adt134 {
    let m2164 = v0;
    match m2164 {
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

pub fn f573(v0: Adt0, v1: Adt32) -> Adt123 {
    Adt123::C0(v1, Adt124::C0(Some(match v0 {
        Adt0::C0(h2165, _, _, _, _, _) => {
            h2165
        }
    }), None::<Adt5>), List::<Adt125>::nil())
}

pub fn f574(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2166, _, _) => {
            h2166
        }
    }, f200(), match v0.clone() {
        Adt0::C0(_, _, h2167, _, _, _) => {
            h2167
        }
    }, match v0 {
        Adt0::C0(_, h2168, _, _, _, _) => {
            h2168
        }
    })
}

pub fn f575(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt123, Adt126> {
    let m2169 = v2;
    match m2169 {
        Ok(v4) => {
            if f615(f572(v1.clone()), v4.clone()) {
                f616(v0, v1, v4)
            } else {
                Err::<Adt123, Adt126>(Adt126::C4(f617()))
            }
        }
        Err(v3) => {
            Err::<Adt123, Adt126>(Adt126::C4(v3))
        }
    }
}

pub fn f576(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2170 = v3.clone();
    match m2170 {
        Adt14::C0(_) => {
            let c2172: Fn0 = v2;
            c2172.apply(v3)
        }
        Adt14::C1(_) => {
            let c2173: Fn0 = v2;
            c2173.apply(v3)
        }
        Adt14::C2(_) => {
            if f618(v0, v1) {
                let c2171: Fn0 = v2;
                c2171.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2174: Fn0 = v2;
            c2174.apply(v3)
        }
        Adt14::C4(_) => {
            let c2175: Fn0 = v2;
            c2175.apply(v3)
        }
        Adt14::C5(_) => {
            let c2176: Fn0 = v2;
            c2176.apply(v3)
        }
        Adt14::C6(_) => {
            let c2177: Fn0 = v2;
            c2177.apply(v3)
        }
        Adt14::C7(_) => {
            let c2178: Fn0 = v2;
            c2178.apply(v3)
        }
        Adt14::C8(_) => {
            let c2179: Fn0 = v2;
            c2179.apply(v3)
        }
        Adt14::C9(_) => {
            let c2180: Fn0 = v2;
            c2180.apply(v3)
        }
        Adt14::C10(_) => {
            let c2181: Fn0 = v2;
            c2181.apply(v3)
        }
        Adt14::C11(_) => {
            let c2182: Fn0 = v2;
            c2182.apply(v3)
        }
    }
}

pub fn f577(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt127> {
    let m2183 = f202(v0.clone(), v1);
    match m2183 {
        Adt130::C0 => {
            Ok::<Option<Str>, Adt127>(None::<Str>)
        }
        Adt130::C1(v2) => {
            let m2184 = f257(v2);
            match m2184 {
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

pub fn f578(v0: Str, v1: Adt5) -> Result<Str, Adt127> {
    let m2185 = f202(v0.clone(), v1);
    match m2185 {
        Adt130::C0 => {
            Err::<Str, Adt127>(Adt127::C7(v0))
        }
        Adt130::C1(v2) => {
            let m2186 = f257(v2);
            match m2186 {
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

pub fn f579(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt127> {
    let m2187 = f202(v0.clone(), v1);
    match m2187 {
        Adt130::C0 => {
            Ok::<Option<Adt5>, Adt127>(None::<Adt5>)
        }
        Adt130::C1(v2) => {
            let m2188 = f221(v2);
            match m2188 {
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

pub fn f580(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt127> {
    let m2189 = f202(v0.clone(), v1);
    match m2189 {
        Adt130::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt127>(None::<List<(Str, Str)>>)
        }
        Adt130::C1(v2) => {
            let m2190 = f221(v2);
            match m2190 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt127>(Adt127::C8(v0))
                }
                Some(v3) => {
                    let m2192 = f619(v0, match v3 {
                        Adt5::C0(h2191) => {
                            h2191
                        }
                    });
                    match m2192 {
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

pub fn f581(v0: Str, v1: Adt5) -> Result<Adt5, Adt127> {
    let m2193 = f202(v0.clone(), v1);
    match m2193 {
        Adt130::C0 => {
            Err::<Adt5, Adt127>(Adt127::C7(v0))
        }
        Adt130::C1(v2) => {
            let m2194 = f221(v2);
            match m2194 {
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

pub fn f582(v0: Adt5) -> Result<Adt25, Adt127> {
    let m2195 = f578(Str::lit("type"), v0.clone());
    match m2195 {
        Ok(v1) => {
            let a2196 = v1.clone();
            let a2197 = Str::lit("ref/prompt");
            if equal(a2196, a2197) {
                let m2198 = f578(Str::lit("name"), v0.clone());
                match m2198 {
                    Ok(v2) => {
                        let m2199 = f577(Str::lit("title"), v0);
                        match m2199 {
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
                let a2200 = v1;
                let a2201 = Str::lit("ref/resource");
                if equal(a2200, a2201) {
                    let m2202 = f578(Str::lit("uri"), v0);
                    match m2202 {
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

pub fn f583(v0: Adt5) -> Result<Adt27, Adt127> {
    let m2203 = f578(Str::lit("name"), v0.clone());
    match m2203 {
        Ok(v1) => {
            let m2204 = f578(Str::lit("value"), v0);
            match m2204 {
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

pub fn f584(v0: Adt5) -> Result<Option<Adt28>, Adt127> {
    let m2205 = f202(Str::lit("context"), v0);
    match m2205 {
        Adt130::C0 => {
            Ok::<Option<Adt28>, Adt127>(None::<Adt28>)
        }
        Adt130::C1(v1) => {
            let m2206 = f221(v1);
            match m2206 {
                None => {
                    Err::<Option<Adt28>, Adt127>(Adt127::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2207 = f580(Str::lit("arguments"), v2);
                    match m2207 {
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

pub fn f585(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt126> {
    if v0 {
        None::<Adt126>
    } else {
        Some(Adt126::C1(v1, v2))
    }
}

pub fn f586(v0: Str, p2212: List<Str>) -> bool {
    let m2211 = p2212;
    {
        let mut m2211 = m2211;
        loop {
            let step = {
                let v1 = m2211;
                let m2208 = v1;
                match m2208.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2209 = v2;
                        let a2210 = v0.clone();
                        if equal(a2209, a2210) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
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

pub fn f587(v0: Option<Adt108>, v1: List<Adt110>, v2: Adt5) -> R<Option<Adt108>> {
    let m2213 = v0;
    match m2213 {
        None => {
            f620(v1, f621(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f588(v0: Option<Str>, v1: Adt140, v2: Adt5) -> Option<Adt108> {
    let m2214 = v1.clone();
    match m2214 {
        Adt140::C0 => {
            None::<Adt108>
        }
        Adt140::C1 => {
            f622(v0, f623(v1, v2))
        }
        Adt140::C2 => {
            f622(v0, f623(v1, v2))
        }
        Adt140::C3 => {
            f622(v0, f623(v1, v2))
        }
    }
}

pub fn f589(v0: Adt130, v1: Adt130) -> Adt148 {
    let m2215 = v0;
    match m2215 {
        Adt130::C0 => {
            Adt148::C1
        }
        Adt130::C1(_) => {
            let m2216 = v1;
            match m2216 {
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

pub fn f590(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt127> {
    let m2217 = f257(v1);
    match m2217 {
        None => {
            Err::<Adt16, Adt127>(Adt127::C5)
        }
        Some(v3) => {
            let m2218 = f624(v2);
            match m2218 {
                None => {
                    Err::<Adt16, Adt127>(Adt127::C6)
                }
                Some(v4) => {
                    let m2219 = f579(f625(), v0.clone());
                    match m2219 {
                        Ok(v6) => {
                            let m2220 = f626(v0.clone());
                            match m2220 {
                                Ok(v8) => {
                                    let m2221 = f627(v0.clone());
                                    match m2221 {
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

pub fn f591(p2225: List<Adt6>) -> bool {
    let m2224 = p2225;
    {
        let mut m2224 = m2224;
        loop {
            let step = {
                let v0 = m2224;
                let m2222 = v0;
                match m2222.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2223 = f257(v1);
                        match m2223 {
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
                Ok(next) => { m2224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f592(p2229: List<Adt6>) -> bool {
    let m2228 = p2229;
    {
        let mut m2228 = m2228;
        loop {
            let step = {
                let v0 = m2228;
                let m2226 = v0;
                match m2226.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2227 = f257(v1);
                        match m2227 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f413(v3) {
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
                Ok(next) => { m2228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: Str) -> Option<Adt13> {
    let a2230 = v0.clone();
    let a2231 = Str::lit("string");
    if equal(a2230, a2231) {
        Some(Adt13::C0)
    } else {
        let a2232 = v0.clone();
        let a2233 = Str::lit("integer");
        if equal(a2232, a2233) {
            Some(Adt13::C1)
        } else {
            let a2234 = v0;
            let a2235 = Str::lit("boolean");
            if equal(a2234, a2235) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f594(p2244: List<u64>, p2245: List<u64>) -> R<List<u64>> {
    let m2243 = (p2244, p2245);
    {
        let mut m2243 = m2243;
        loop {
            let step = {
                let (v0, v1) = m2243;
                let m2236 = v0;
                match m2236.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2237 = 65u64;
                        let a2238 = v2;
                        if if nat_le(a2237, a2238) {
                            let a2239 = v2;
                            let a2240 = 90u64;
                            nat_le(a2239, a2240)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2241 = v2;
                                let a2242 = 32u64;
                                nat_add(a2241, a2242)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(p2252: List<u64>, p2253: List<u64>) -> bool {
    let m2251 = (p2252, p2253);
    {
        let mut m2251 = m2251;
        loop {
            let step = {
                let (v0, v1) = m2251;
                let m2246 = v0;
                match m2246.uncons() {
                    None => {
                        let m2247 = v1;
                        match m2247.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2248 = v1;
                        match m2248.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2249 = v4;
                                let a2250 = v6;
                                if nat_eq(a2249, a2250) {
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
                Ok(next) => { m2251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: Adt6) -> u64 {
    let m2254 = v0;
    match m2254 {
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

pub fn f597(v0: Adt147) -> bool {
    let m2255 = v0;
    match m2255 {
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

pub fn f598(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2256 = v0;
    match m2256 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2257 = v2;
                let a2258 = v1;
                append_list(a2257, a2258)
            })
        }
    }
}

pub fn f599(p2263: List<Adt6>, p2264: List<Adt6>, p2265: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2262 = (p2263, (p2264, p2265));
    {
        let mut m2262 = m2262;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2262;
                let m2259 = v0;
                match m2259.uncons() {
                    None => {
                        let m2260 = v1;
                        match m2260.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2261 = v1;
                        match m2261.uncons() {
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
                Ok(next) => { m2262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(p2271: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2272: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2270 = (p2271, p2272);
    {
        let mut m2270 = m2270;
        loop {
            let step = {
                let (v0, v2) = m2270;
                let m2266 = v0;
                match m2266.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2268 = f260({
                            let (h2267, _) = v3;
                            h2267
                        }, v1.clone());
                        match m2268 {
                            Adt130::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt130::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2269) = v3;
                                    h2269
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2270 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f601(v0: Adt147) -> Adt147 {
    let m2273 = v0;
    match m2273 {
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

pub fn f602(v0: i64, v1: i64) -> R<i64> {
    f187(f143(f604(v0)?, 0i64)?, v1)
}

pub fn f603(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2274 = v1;
    let a2275 = v3;
    if nat_lt(a2274, a2275) {
        Ok(f495(f628({
            let a2276 = v3;
            let a2277 = v1;
            nat_sub(a2276, a2277)
        }, v0)?, v2))
    } else {
        let a2278 = v3;
        let a2279 = v1;
        if nat_lt(a2278, a2279) {
            Ok(f495(v0, f628({
                let a2280 = v1;
                let a2281 = v3;
                nat_sub(a2280, a2281)
            }, v2)?))
        } else {
            Ok(f495(v0, v2))
        }
    }
}

pub fn f604(v0: i64) -> R<u64> {
    f629(20u64, v0, 0u64)
}

pub fn f605(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2282 = v0;
    match m2282 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2294 = f443(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2283, _, _, _, _) => {
                        h2283
                    }
                });
                let a2295 = {
                    let a2292 = f448(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2284, _, _, _) => {
                            h2284
                        }
                    });
                    let a2293 = {
                        let a2290 = f448(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2285, _, _) => {
                                h2285
                            }
                        });
                        let a2291 = {
                            let a2288 = f448(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2286, _) => {
                                    h2286
                                }
                            });
                            let a2289 = f448(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2287) => {
                                    h2287
                                }
                            });
                            append_list(a2288, a2289)
                        };
                        append_list(a2290, a2291)
                    };
                    append_list(a2292, a2293)
                };
                append_list(a2294, a2295)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f606(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f463(Str::lit("_meta"), v0)
}

pub fn f607(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2296, _, _) => {
            h2296
        }
    })), {
        let a2299 = f630(match v0.clone() {
            Adt39::C0(_, h2297, _) => {
                h2297
            }
        });
        let a2300 = f606(match v0 {
            Adt39::C0(_, _, h2298) => {
                h2298
            }
        });
        append_list(a2299, a2300)
    })))
}

pub fn f608(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2301 = f630(v3);
        let a2302 = f606(v4);
        append_list(a2301, a2302)
    }))))
}

pub fn f609(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2326 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2303, _, _, _, _, _, _, _, _) => {
                h2303
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2327 = {
            let a2324 = f443(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2304, _, _, _, _, _, _, _) => {
                    h2304
                }
            });
            let a2325 = {
                let a2322 = f511(match v0.clone() {
                    Adt44::C0(_, _, h2305, _, _, _, _, _, _) => {
                        h2305
                    }
                });
                let a2323 = {
                    let a2320 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2306, _, _, _, _, _) => {
                            h2306
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2321 = {
                        let a2318 = f443(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2307, _, _, _, _) => {
                                h2307
                            }
                        });
                        let a2319 = {
                            let a2316 = f443(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2308, _, _, _) => {
                                    h2308
                                }
                            });
                            let a2317 = {
                                let a2314 = f630(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2309, _, _) => {
                                        h2309
                                    }
                                });
                                let a2315 = {
                                    let a2312 = f631(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2310, _) => {
                                            h2310
                                        }
                                    })?;
                                    let a2313 = f606(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2311) => {
                                            h2311
                                        }
                                    });
                                    append_list(a2312, a2313)
                                };
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
    }))
}

pub fn f610(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f632(match v0.clone() {
        Adt45::C0(h2328, _, _) => {
            h2328
        }
    })), {
        let a2331 = f630(match v0.clone() {
            Adt45::C0(_, h2329, _) => {
                h2329
            }
        });
        let a2332 = f606(match v0 {
            Adt45::C0(_, _, h2330) => {
                h2330
            }
        });
        append_list(a2331, a2332)
    })))
}

pub fn f611(p2335: List<Adt2>, p2336: List<Adt6>) -> List<Adt6> {
    let m2334 = (p2335, p2336);
    {
        let mut m2334 = m2334;
        loop {
            let step = {
                let (v0, v1) = m2334;
                let m2333 = v0;
                match m2333.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f633(v2), v1)))
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

pub fn f612() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f613() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f614(p2341: List<u64>, p2342: u64, p2343: List<u64>) -> List<u64> {
    let m2340 = (p2341, (p2342, p2343));
    {
        let mut m2340 = m2340;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2340;
                let m2337 = v0;
                match m2337.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f634(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt150 = f635(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt150::C0(h2338, _) => {
                                h2338
                            }
                        }, match v5 {
                            Adt150::C0(_, h2339) => {
                                h2339
                            }
                        })))
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

pub fn f615(v0: Adt134, v1: Adt32) -> bool {
    let m2344 = v1;
    match m2344 {
        Adt32::C0(_) => {
            f636(v0, Adt134::C0)
        }
        Adt32::C1(_) => {
            f636(v0, Adt134::C1)
        }
        Adt32::C2(_) => {
            f636(v0, Adt134::C2)
        }
        Adt32::C3(_) => {
            f636(v0, Adt134::C2)
        }
        Adt32::C4(_) => {
            f636(v0, Adt134::C3)
        }
        Adt32::C5(_) => {
            f636(v0, Adt134::C4)
        }
        Adt32::C6(_) => {
            f636(v0, Adt134::C5)
        }
        Adt32::C7(_) => {
            f636(v0, Adt134::C6)
        }
        Adt32::C8(_) => {
            f636(v0, Adt134::C7)
        }
        Adt32::C9(_) => {
            f636(v0, Adt134::C8)
        }
        Adt32::C10(_) => {
            f636(v0, Adt134::C9)
        }
        Adt32::C11 => {
            if f636(v0.clone(), Adt134::C10) {
                true
            } else {
                f636(v0, Adt134::C11)
            }
        }
        Adt32::C12(_) => {
            f637(v0)
        }
        Adt32::C13(_) => {
            f638(v0)
        }
    }
}

pub fn f616(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt123, Adt126> {
    let m2345 = v2.clone();
    match m2345 {
        Adt32::C0(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt123, Adt126>(f642(v0, f640(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt123, Adt126>(f573(v0, v2))
        }
        Adt32::C12(v3) => {
            f639(v0, f640(v1), v3)
        }
        Adt32::C13(v4) => {
            f641(v0, f640(v1), v4)
        }
    }
}

pub fn f617() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f618(v0: u64, v1: u64) -> bool {
    let a2346 = v1;
    let a2347 = v0;
    nat_lt(a2346, a2347)
}

pub fn f619(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt127> {
    let m2348 = f643(v0, v1, List::<(Str, Str)>::nil());
    match m2348 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt127>(f644(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt127>(v3)
        }
    }
}

pub fn f620(p2352: List<Adt110>, v1: Adt5) -> R<Option<Adt108>> {
    let m2351 = p2352;
    {
        let mut m2351 = m2351;
        loop {
            let step = {
                let v0 = m2351;
                let m2349 = v0;
                match m2349.uncons() {
                    None => {
                        Err::<List<Adt110>, R<Option<Adt108>>>(Ok(None::<Adt108>))
                    }
                    Some((v2, v3)) => {
                        let m2350 = f645(v2, v1.clone())?;
                        match m2350 {
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
                Ok(next) => { m2351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: Adt5) -> Adt5 {
    let m2353 = f202(Str::lit("arguments"), v0);
    match m2353 {
        Adt130::C0 => {
            f144()
        }
        Adt130::C1(v1) => {
            let m2354 = f221(v1);
            match m2354 {
                None => {
                    f144()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f622(v0: Option<Str>, v1: Option<Str>) -> Option<Adt108> {
    let m2355 = v0;
    match m2355 {
        None => {
            Some(Adt108::C2)
        }
        Some(v2) => {
            let m2356 = v1;
            match m2356 {
                None => {
                    None::<Adt108>
                }
                Some(v3) => {
                    let a2357 = v2;
                    let a2358 = v3;
                    if equal(a2357, a2358) {
                        None::<Adt108>
                    } else {
                        Some(Adt108::C5)
                    }
                }
            }
        }
    }
}

pub fn f623(v0: Adt140, v1: Adt5) -> Option<Str> {
    let m2359 = v0;
    match m2359 {
        Adt140::C0 => {
            None::<Str>
        }
        Adt140::C1 => {
            let m2360 = f202(Str::lit("name"), v1);
            match m2360 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v2) => {
                    f257(v2)
                }
            }
        }
        Adt140::C2 => {
            let m2361 = f202(Str::lit("uri"), v1);
            match m2361 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v3) => {
                    f257(v3)
                }
            }
        }
        Adt140::C3 => {
            let m2362 = f202(Str::lit("taskId"), v1);
            match m2362 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v4) => {
                    f257(v4)
                }
            }
        }
    }
}

pub fn f624(v0: Adt6) -> Option<Adt17> {
    let m2363 = f221(v0);
    match m2363 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2364 = f646(Str::lit("experimental"), v1.clone());
            match m2364 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2365 = f646(Str::lit("roots"), v1.clone());
                    match m2365 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2366 = f646(Str::lit("sampling"), v1.clone());
                            match m2366 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2367 = f646(Str::lit("elicitation"), v1.clone());
                                    match m2367 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2368 = f646(Str::lit("extensions"), v1);
                                            match m2368 {
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

pub fn f625() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f626(v0: Adt5) -> Result<Option<Adt18>, Adt127> {
    let m2369 = f577(f647(), v0);
    match m2369 {
        Ok(v2) => {
            let m2370 = v2;
            match m2370 {
                None => {
                    Ok::<Option<Adt18>, Adt127>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2371 = f648(v3);
                    match m2371 {
                        Adt151::C0 => {
                            Err::<Option<Adt18>, Adt127>(Adt127::C8(f647()))
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

pub fn f627(v0: Adt5) -> Result<Option<Adt19>, Adt127> {
    let m2372 = f202(f649(), v0);
    match m2372 {
        Adt130::C0 => {
            Ok::<Option<Adt19>, Adt127>(None::<Adt19>)
        }
        Adt130::C1(v1) => {
            let m2373 = f257(v1.clone());
            match m2373 {
                None => {
                    let m2374 = f494(v1);
                    match m2374 {
                        None => {
                            Err::<Option<Adt19>, Adt127>(Adt127::C8(f649()))
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

pub fn f628(p2379: u64, p2380: i64) -> R<i64> {
    let m2378 = (p2379, p2380);
    {
        let mut m2378 = m2378;
        loop {
            let step = {
                let (v0, v1) = m2378;
                let m2375 = v0;
                if m2375 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2375 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2376 = v1;
                        let a2377 = 10i64;
                        int_mul(a2376, a2377)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f629(p2391: u64, p2392: i64, p2393: u64) -> R<u64> {
    let m2390 = (p2391, (p2392, p2393));
    {
        let mut m2390 = m2390;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2390;
                let m2381 = v0;
                if m2381 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2381 - 1;
                    let v4: u64 = {
                        let a2382 = v2;
                        let a2383 = 1u64;
                        nat_add(a2382, a2383)?
                    };
                    let m2386 = {
                        let a2384 = v1;
                        let a2385 = 10i64;
                        compare(a2384, a2385)
                    };
                    if match m2386 {
                        Ordering::Less => {
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
                            let a2387 = v1;
                            let a2388 = 10i64;
                            let a2389 = 0i64;
                            int_quot(a2387, a2388, a2389)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f630(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2394 = v0;
    match m2394 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2400 = f650(match v1.clone() {
                    Adt40::C0(h2395, _, _) => {
                        h2395
                    }
                });
                let a2401 = {
                    let a2398 = f651(match v1.clone() {
                        Adt40::C0(_, h2396, _) => {
                            h2396
                        }
                    });
                    let a2399 = f443(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2397) => {
                            h2397
                        }
                    });
                    append_list(a2398, a2399)
                };
                append_list(a2400, a2401)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f631(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2402 = v0;
    match m2402 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f59(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f632(v0: Adt46) -> Adt6 {
    let m2403 = v0;
    match m2403 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2404, _, _, _) => {
                    h2404
                }
            })), {
                let a2408 = f443(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2405, _, _) => {
                        h2405
                    }
                });
                let a2409 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2406, _) => {
                        h2406
                    }
                })), f606(match v1 {
                    Adt47::C0(_, _, _, h2407) => {
                        h2407
                    }
                }));
                append_list(a2408, a2409)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2410, _, _, _) => {
                    h2410
                }
            })), {
                let a2414 = f443(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2411, _, _) => {
                        h2411
                    }
                });
                let a2415 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2412, _) => {
                        h2412
                    }
                })), f606(match v2 {
                    Adt48::C0(_, _, _, h2413) => {
                        h2413
                    }
                }));
                append_list(a2414, a2415)
            }))
        }
    }
}

pub fn f633(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2416, _, _, _) => {
            h2416
        }
    })), {
        let a2422 = f443(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2417, _, _) => {
                h2417
            }
        });
        let a2423 = {
            let a2420 = f652(match v0.clone() {
                Adt2::C0(_, _, h2418, _) => {
                    h2418
                }
            });
            let a2421 = f653(match v0 {
                Adt2::C0(_, _, _, h2419) => {
                    h2419
                }
            });
            append_list(a2420, a2421)
        };
        append_list(a2422, a2423)
    }))
}

pub fn f634(v0: u64, v1: List<u64>) -> List<u64> {
    let a2424 = v0;
    let a2425 = 0u64;
    if nat_eq(a2424, a2425) {
        v1
    } else {
        let a2426 = v0;
        let a2427 = 1u64;
        if nat_eq(a2426, a2427) {
            List::cons(194u64, v1)
        } else {
            let a2428 = v0;
            let a2429 = 2u64;
            if nat_eq(a2428, a2429) {
                List::cons(226u64, v1)
            } else {
                let a2430 = v0;
                let a2431 = 3u64;
                if nat_eq(a2430, a2431) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f635(v0: u64, v1: u64, v2: List<u64>) -> Adt150 {
    let a2432 = v0;
    let a2433 = 0u64;
    if nat_eq(a2432, a2433) {
        f654(v1, v2)
    } else {
        let a2434 = v0;
        let a2435 = 1u64;
        if nat_eq(a2434, a2435) {
            let a2436 = 128u64;
            let a2437 = v1;
            if if nat_le(a2436, a2437) {
                let a2438 = v1;
                let a2439 = 159u64;
                nat_le(a2438, a2439)
            } else {
                false
            } {
                Adt150::C0(0u64, f655(v2))
            } else {
                f654(v1, List::cons(194u64, v2))
            }
        } else {
            let a2440 = v0;
            let a2441 = 2u64;
            if nat_eq(a2440, a2441) {
                let a2442 = v1;
                let a2443 = 128u64;
                if nat_eq(a2442, a2443) {
                    Adt150::C0(3u64, v2)
                } else {
                    let a2444 = v1;
                    let a2445 = 129u64;
                    if nat_eq(a2444, a2445) {
                        Adt150::C0(4u64, v2)
                    } else {
                        f654(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2446 = v0;
                let a2447 = 3u64;
                if nat_eq(a2446, a2447) {
                    let a2448 = 170u64;
                    let a2449 = v1;
                    if if nat_le(a2448, a2449) {
                        let a2450 = v1;
                        let a2451 = 174u64;
                        nat_le(a2450, a2451)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f655(v2))
                    } else {
                        f654(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2452 = 166u64;
                    let a2453 = v1;
                    if if nat_le(a2452, a2453) {
                        let a2454 = v1;
                        let a2455 = 169u64;
                        nat_le(a2454, a2455)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f655(v2))
                    } else {
                        f654(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f636(v0: Adt134, v1: Adt134) -> bool {
    let a2456 = f467(v0);
    let a2457 = f467(v1);
    equal(a2456, a2457)
}

pub fn f637(v0: Adt134) -> bool {
    let m2458 = v0;
    match m2458 {
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

pub fn f638(v0: Adt134) -> bool {
    let m2459 = v0;
    match m2459 {
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

pub fn f639(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt123, Adt126> {
    f656(v0, v2.clone(), f657(match v1 {
        Adt16::C0(_, h2460, _, _, _, _) => {
            h2460
        }
    }, f658(v2)))
}

pub fn f640(v0: Adt14) -> Adt16 {
    let m2461 = v0;
    match m2461 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2462) => {
                    h2462
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2463, _) => {
                    h2463
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2464, _, _, _, _) => {
                    h2464
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2465, _) => {
                    h2465
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2466, _, _, _) => {
                    h2466
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2467, _) => {
                    h2467
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2468, _) => {
                    h2468
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2469, _, _, _, _) => {
                    h2469
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2470, _, _, _) => {
                    h2470
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2471, _) => {
                    h2471
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2472, _, _) => {
                    h2472
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2473, _) => {
                    h2473
                }
            }
        }
    }
}

pub fn f641(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt123, Adt126> {
    f659(v0, v2, f660(match v1 {
        Adt16::C0(_, h2474, _, _, _, _) => {
            h2474
        }
    }))
}

pub fn f642(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt123 {
    Adt123::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2475, _) => {
            h2475
        }
    }), {
        let m2477 = f573(v0, Adt32::C11);
        match m2477 {
            Adt123::C0(_, h2476, _) => {
                h2476
            }
        }
    }, f661(v1, match v2 {
        Adt49::C0(_, h2478) => {
            h2478
        }
    }))
}

pub fn f643(v0: Str, p2484: List<(Str, Adt6)>, p2485: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt127> {
    let m2483 = (p2484, p2485);
    {
        let mut m2483 = m2483;
        loop {
            let step = {
                let (v1, v2) = m2483;
                let m2479 = v1;
                match m2479.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>(Ok::<List<(Str, Str)>, Adt127>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2481 = f257({
                            let (_, h2480) = v3;
                            h2480
                        });
                        match m2481 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>(Err::<List<(Str, Str)>, Adt127>(Adt127::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>((v4, List::cons(({
                                    let (h2482, _) = v3;
                                    h2482
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2483 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(p2488: List<(Str, Str)>, p2489: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2487 = (p2488, p2489);
    {
        let mut m2487 = m2487;
        loop {
            let step = {
                let (v0, v1) = m2487;
                let m2486 = v0;
                match m2486.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(v0: Adt110, v1: Adt5) -> R<Option<Adt108>> {
    let m2491 = f662(match v0.clone() {
        Adt110::C0(_, h2490, _, _) => {
            h2490
        }
    }, v1);
    match m2491 {
        Adt130::C0 => {
            Ok(f663(v0))
        }
        Adt130::C1(v2) => {
            if f664(v2.clone()) {
                Ok(f663(v0))
            } else {
                f665(v0, v2)
            }
        }
    }
}

pub fn f646(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2492 = f202(v0, v1);
    match m2492 {
        Adt130::C0 => {
            Some(None::<Adt5>)
        }
        Adt130::C1(v2) => {
            let m2493 = f221(v2);
            match m2493 {
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

pub fn f647() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f648(v0: Str) -> Adt151 {
    let a2494 = v0.clone();
    let a2495 = Str::lit("debug");
    if equal(a2494, a2495) {
        Adt151::C1(Adt18::C0)
    } else {
        let a2496 = v0.clone();
        let a2497 = Str::lit("info");
        if equal(a2496, a2497) {
            Adt151::C1(Adt18::C1)
        } else {
            let a2498 = v0.clone();
            let a2499 = Str::lit("notice");
            if equal(a2498, a2499) {
                Adt151::C1(Adt18::C2)
            } else {
                let a2500 = v0.clone();
                let a2501 = Str::lit("warning");
                if equal(a2500, a2501) {
                    Adt151::C1(Adt18::C3)
                } else {
                    let a2502 = v0.clone();
                    let a2503 = Str::lit("error");
                    if equal(a2502, a2503) {
                        Adt151::C1(Adt18::C4)
                    } else {
                        let a2504 = v0.clone();
                        let a2505 = Str::lit("critical");
                        if equal(a2504, a2505) {
                            Adt151::C1(Adt18::C5)
                        } else {
                            let a2506 = v0.clone();
                            let a2507 = Str::lit("alert");
                            if equal(a2506, a2507) {
                                Adt151::C1(Adt18::C6)
                            } else {
                                let a2508 = v0;
                                let a2509 = Str::lit("emergency");
                                if equal(a2508, a2509) {
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

pub fn f649() -> Str {
    Str::lit("progressToken")
}

pub fn f650(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2510 = v0;
    match m2510 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f666(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f651(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2511 = v0;
    match m2511 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f652(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2512 = v0;
    match m2512 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f385(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f653(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2513 = v0;
    match m2513 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f667(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f654(v0: u64, v1: List<u64>) -> Adt150 {
    let a2514 = v0;
    let a2515 = 32u64;
    if nat_lt(a2514, a2515) {
        if f668(v0) {
            Adt150::C0(0u64, List::cons(v0, v1))
        } else {
            Adt150::C0(0u64, f655(v1))
        }
    } else {
        let a2516 = v0;
        let a2517 = 127u64;
        if nat_eq(a2516, a2517) {
            Adt150::C0(0u64, f655(v1))
        } else {
            let a2518 = v0;
            let a2519 = 194u64;
            if nat_eq(a2518, a2519) {
                Adt150::C0(1u64, v1)
            } else {
                let a2520 = v0;
                let a2521 = 226u64;
                if nat_eq(a2520, a2521) {
                    Adt150::C0(2u64, v1)
                } else {
                    Adt150::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f655(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f656(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt123, Adt126> {
    if v2 {
        Ok::<Adt123, Adt126>(f573(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt123, Adt126>(Adt126::C4(Adt84::C8(f669(f658(v1)))))
    }
}

pub fn f657(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2522 = f670(v1.clone());
    if if if bool_not(a2522) {
        true
    } else {
        f470(match v0.clone() {
            Adt17::C0(_, _, _, h2523, _) => {
                h2523
            }
        })
    } {
        let a2524 = f671(v1.clone());
        if bool_not(a2524) {
            true
        } else {
            f470(match v0.clone() {
                Adt17::C0(_, _, h2525, _, _) => {
                    h2525
                }
            })
        }
    } else {
        false
    } {
        let a2526 = f672(v1);
        if bool_not(a2526) {
            true
        } else {
            f470(match v0 {
                Adt17::C0(_, h2527, _, _, _) => {
                    h2527
                }
            })
        }
    } else {
        false
    }
}

pub fn f658(v0: Adt81) -> List<(Str, Adt64)> {
    let m2528 = v0;
    match m2528 {
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

pub fn f659(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt123, Adt126> {
    if v2 {
        Ok::<Adt123, Adt126>(f573(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt123, Adt126>(Adt126::C4(Adt84::C8(f673())))
    }
}

pub fn f660(v0: Adt17) -> bool {
    f471(match v0 {
        Adt17::C0(_, _, _, _, h2529) => {
            h2529
        }
    }, f472())
}

pub fn f661(v0: Adt16, v1: List<Adt50>) -> List<Adt125> {
    let m2531 = match v0 {
        Adt16::C0(_, _, _, _, h2530, _) => {
            h2530
        }
    };
    match m2531 {
        None => {
            List::<Adt125>::nil()
        }
        Some(v2) => {
            f674(f675(v2, v1, List::<Adt125>::nil()), List::<Adt125>::nil())
        }
    }
}

pub fn f662(p2537: List<Str>, p2538: Adt5) -> Adt130 {
    let m2536 = (p2537, p2538);
    {
        let mut m2536 = m2536;
        loop {
            let step = {
                let (v0, v1) = m2536;
                let m2532 = v0;
                match m2532.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                    }
                    Some((v2, v3)) => {
                        let m2533 = f202(v2, v1);
                        match m2533 {
                            Adt130::C0 => {
                                Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                            }
                            Adt130::C1(v4) => {
                                let m2534 = v3.clone();
                                match m2534.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt130>(Adt130::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2535 = f221(v4);
                                        match m2535 {
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
                Ok(next) => { m2536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f663(v0: Adt110) -> Option<Adt108> {
    let m2540 = match v0.clone() {
        Adt110::C0(_, _, _, h2539) => {
            h2539
        }
    };
    match m2540 {
        Adt111::C0 => {
            None::<Adt108>
        }
        Adt111::C1 => {
            Some(Adt108::C10(match v0 {
                Adt110::C0(h2541, _, _, _) => {
                    h2541
                }
            }))
        }
        Adt111::C2(_) => {
            None::<Adt108>
        }
    }
}

pub fn f664(v0: Adt6) -> bool {
    let m2542 = v0;
    match m2542 {
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

pub fn f665(v0: Adt110, v1: Adt6) -> R<Option<Adt108>> {
    let m2544 = match v0.clone() {
        Adt110::C0(_, _, _, h2543) => {
            h2543
        }
    };
    match m2544 {
        Adt111::C0 => {
            Ok(Some(Adt108::C8(match v0 {
                Adt110::C0(h2545, _, _, _) => {
                    h2545
                }
            })))
        }
        Adt111::C1 => {
            Ok(Some(Adt108::C10(match v0 {
                Adt110::C0(h2546, _, _, _) => {
                    h2546
                }
            })))
        }
        Adt111::C2(v2) => {
            if f676(match v0.clone() {
                Adt110::C0(_, _, h2547, _) => {
                    h2547
                }
            }, v2, v1)? {
                Ok(None::<Adt108>)
            } else {
                Ok(Some(Adt108::C9(match v0 {
                    Adt110::C0(h2548, _, _, _) => {
                        h2548
                    }
                })))
            }
        }
    }
}

pub fn f666(v0: List<Adt41>) -> List<Adt6> {
    f320(f677(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f667(v0: Adt3) -> Str {
    let m2549 = v0;
    match m2549 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f668(v0: u64) -> bool {
    let a2550 = v0;
    let a2551 = 9u64;
    if nat_eq(a2550, a2551) {
        true
    } else {
        let a2552 = v0;
        let a2553 = 10u64;
        if nat_eq(a2552, a2553) {
            true
        } else {
            let a2554 = v0;
            let a2555 = 13u64;
            nat_eq(a2554, a2555)
        }
    }
}

pub fn f669(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f672(v0.clone()) {
        Some(f144())
    } else {
        None::<Adt5>
    }, if f671(v0.clone()) {
        Some(f144())
    } else {
        None::<Adt5>
    }, if f670(v0) {
        Some(f144())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f670(p2560: List<(Str, Adt64)>) -> bool {
    let m2559 = p2560;
    {
        let mut m2559 = m2559;
        loop {
            let step = {
                let v0 = m2559;
                let m2556 = v0;
                match m2556.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2558 = {
                            let (_, h2557) = v1;
                            h2557
                        };
                        match m2558 {
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
                Ok(next) => { m2559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f671(p2565: List<(Str, Adt64)>) -> bool {
    let m2564 = p2565;
    {
        let mut m2564 = m2564;
        loop {
            let step = {
                let v0 = m2564;
                let m2561 = v0;
                match m2561.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2563 = {
                            let (_, h2562) = v1;
                            h2562
                        };
                        match m2563 {
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
                Ok(next) => { m2564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f672(p2570: List<(Str, Adt64)>) -> bool {
    let m2569 = p2570;
    {
        let mut m2569 = m2569;
        loop {
            let step = {
                let v0 = m2569;
                let m2566 = v0;
                match m2566.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2568 = {
                            let (_, h2567) = v1;
                            h2567
                        };
                        match m2568 {
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
                Ok(next) => { m2569 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f673() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f472(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f674(p2573: List<Adt125>, p2574: List<Adt125>) -> List<Adt125> {
    let m2572 = (p2573, p2574);
    {
        let mut m2572 = m2572;
        loop {
            let step = {
                let (v0, v1) = m2572;
                let m2571 = v0;
                match m2571.uncons() {
                    None => {
                        Err::<(List<Adt125>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt125>, List<Adt125>), List<Adt125>>((v3, List::cons(v2, v1)))
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

pub fn f675(v0: Adt19, p2577: List<Adt50>, p2578: List<Adt125>) -> List<Adt125> {
    let m2576 = (p2577, p2578);
    {
        let mut m2576 = m2576;
        loop {
            let step = {
                let (v1, v2) = m2576;
                let m2575 = v1;
                match m2575.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt125>), List<Adt125>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt125>), List<Adt125>>((v4, List::cons(Adt125::C0(v0.clone(), v3), v2)))
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

pub fn f676(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2579 = v0;
    match m2579 {
        Adt13::C0 => {
            let m2580 = f257(v2);
            match m2580 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2581 = v1;
                    let a2582 = v3;
                    Ok(equal(a2581, a2582))
                }
            }
        }
        Adt13::C1 => {
            f678(v1, v2)
        }
        Adt13::C2 => {
            let m2583 = f220(v2);
            match m2583 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2584 = v1;
                        let a2585 = Str::lit("true");
                        Ok(equal(a2584, a2585))
                    } else {
                        let a2586 = v1;
                        let a2587 = Str::lit("false");
                        Ok(equal(a2586, a2587))
                    }
                }
            }
        }
    }
}

pub fn f677(p2590: List<Adt41>, p2591: List<Adt6>) -> List<Adt6> {
    let m2589 = (p2590, p2591);
    {
        let mut m2589 = m2589;
        loop {
            let step = {
                let (v0, v1) = m2589;
                let m2588 = v0;
                match m2588.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f679(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2589 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f678(v0: Str, v1: Adt6) -> R<bool> {
    let m2592 = f418(v1);
    match m2592 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2593 = f259(v2)?;
            match m2593 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2595 = f680(f110({
                        let a2594 = v0;
                        utf8_encode(a2594)
                    })?)?;
                    match m2595 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f400({
                                let a2596 = v3;
                                let a2597 = v4;
                                int_sub(a2596, a2597)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f679(v0: Adt41) -> Str {
    let m2598 = v0;
    match m2598 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f680(v0: List<u64>) -> R<Option<i64>> {
    let m2599 = v0.clone();
    match m2599.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2600 = v1;
            let a2601 = 45u64;
            if nat_eq(a2600, a2601) {
                let m2602 = v2.clone();
                match m2602.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2603 = f681(v2, 0i64)?;
                        match m2603 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2604 = v5;
                                    int_neg(a2604)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f681(v0, 0i64)
            }
        }
    }
}

pub fn f681(p2610: List<u64>, p2611: i64) -> R<Option<i64>> {
    let m2609 = (p2610, p2611);
    {
        let mut m2609 = m2609;
        loop {
            let step = {
                let (v0, v1) = m2609;
                let m2605 = v0;
                match m2605.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2606 = f682(v2);
                        match m2606 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f187({
                                    let a2607 = v1;
                                    let a2608 = 10i64;
                                    int_mul(a2607, a2608)?
                                }, f319(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f682(v0: u64) -> Option<u64> {
    let a2612 = v0;
    let a2613 = 48u64;
    if if nat_lt(a2612, a2613) {
        true
    } else {
        let a2614 = 57u64;
        let a2615 = v0;
        nat_lt(a2614, a2615)
    } {
        None::<u64>
    } else {
        Some({
            let a2616 = v0;
            let a2617 = 48u64;
            nat_sub(a2616, a2617)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
