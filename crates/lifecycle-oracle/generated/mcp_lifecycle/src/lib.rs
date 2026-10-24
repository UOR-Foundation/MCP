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
    C0(Adt50),
    C1(Adt99, Adt50, Adt86),
    C2(Adt82),
}

#[derive(Clone)]
pub enum Adt99 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt100 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt101 {
    C0(Adt85, bool),
}

#[derive(Clone)]
pub enum Adt102 {
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
pub enum Adt103 {
    C0,
    C1(Option<Adt100>, Adt104),
}

#[derive(Clone)]
pub enum Adt104 {
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
pub enum Adt105 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt106 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt107 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt108 {
    C0(Adt100, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt109 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1(List<Adt111>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt111 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt112 {
    C0,
    C1(List<Adt111>, Str),
}

#[derive(Clone)]
pub enum Adt113 {
    C0(Adt6, Adt6, List<Adt111>),
    C1(List<Adt6>, Adt6, List<Adt111>),
    C2(List<Adt6>, Adt6, List<Adt111>, u64),
    C3(List<Adt111>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt111>),
    C5(Adt114, Adt6, Adt6, List<Adt111>),
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
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt116 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt117>, List<Adt119>),
}

#[derive(Clone)]
pub enum Adt117 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt118),
    C7(Adt118),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt118 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt119 {
    C0(Str, List<Str>, Adt13, Adt120),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt122 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt123 {
    C0(Adt124, List<Adt128>, u64),
}

#[derive(Clone)]
pub enum Adt124 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt125, List<u64>, bool),
    C7(u64, u64),
    C8(Adt126),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt125 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt126 {
    C0(bool, Adt127, i64, u64, i64, bool, i64),
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
    C7,
}

#[derive(Clone)]
pub enum Adt128 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt130 {
    C0(Adt112, List<Adt113>),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt132 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt133 {
    C0(Adt100, Result<Adt134, Adt137>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0(Adt33, Adt135, List<Adt136>),
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt20, Adt51),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Adt138),
    C1(Str, List<Str>),
    C2(Adt117),
    C3(Str),
    C4(Adt90),
}

#[derive(Clone)]
pub enum Adt138 {
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
pub enum Adt139 {
    C0(Option<Str>, Option<Adt117>),
}

#[derive(Clone)]
pub enum Adt140 {
    C0,
    C1(Adt100),
    C2,
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Adt102),
    C1(Adt123),
}

#[derive(Clone)]
pub enum Adt142 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt113>),
}

#[derive(Clone)]
pub enum Adt143 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt102),
    C1(List<Adt115>, List<u64>),
}

#[derive(Clone)]
pub enum Adt145 {
    C0(Adt100, Adt146, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt146 {
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
pub enum Adt147 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt148 {
    C0,
    C1(Adt146),
}

#[derive(Clone)]
pub enum Adt149 {
    C0(Adt150),
    C1(Adt137),
}

#[derive(Clone)]
pub enum Adt150 {
    C0(Adt100, Str, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt151 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt152 {
    C0(Adt97),
    C1(Adt129),
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
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt156 {
    C0,
    C1,
    C2,
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
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt160 {
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
    F34,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt22) -> R<Option<Adt98>> {
        match self {
            Fn1::F34 => f34(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F35,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt22) -> R<Result<Adt50, Adt90>> {
        match self {
            Fn2::F35 => f35(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F119(Fn0, Adt91),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt15) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn3::F119(k0, k1) => f119(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F167,
}

impl Fn4 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt47>>> {
        match self {
            Fn4::F167 => f167(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F173,
}

impl Fn5 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt59 {
        match self {
            Fn5::F173 => f173(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F175,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt25) -> R<Result<List<Str>, Adt90>> {
        match self {
            Fn6::F175 => f175(p0),
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

pub fn f4(v0: Bytes) -> R<Result<Adt91, Adt93>> {
    let m3 = f16(v0.clone(), 0u64)?;
    match m3 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(0u64)))
        }
        Some(v1) => {
            f17(v0.clone(), v1.clone(), f16(v0, match v1 {
                Adt95::C0(_, h4) => {
                    h4
                }
            })?)
        }
    }
}

pub fn f5(v0: Adt94) -> List<u64> {
    let a28 = f18(match v0.clone() {
        Adt94::C0(h5, _, _, _, _) => {
            h5
        }
    });
    let a29 = {
        let a26 = f18({
            let a7 = match v0.clone() {
                Adt94::C0(_, h6, _, _, _) => {
                    h6
                }
            };
            length_list(a7)
        });
        let a27 = {
            let a24 = f19(match v0.clone() {
                Adt94::C0(_, h8, _, _, _) => {
                    h8
                }
            });
            let a25 = {
                let a22 = f18({
                    let a10 = match v0.clone() {
                        Adt94::C0(_, _, h9, _, _) => {
                            h9
                        }
                    };
                    length_list(a10)
                });
                let a23 = {
                    let a20 = f20(match v0.clone() {
                        Adt94::C0(_, _, h11, _, _) => {
                            h11
                        }
                    });
                    let a21 = {
                        let a18 = f18({
                            let a13 = match v0.clone() {
                                Adt94::C0(_, _, _, h12, _) => {
                                    h12
                                }
                            };
                            length_list(a13)
                        });
                        let a19 = {
                            let a16 = match v0.clone() {
                                Adt94::C0(_, _, _, h14, _) => {
                                    h14
                                }
                            };
                            let a17 = f21(match v0 {
                                Adt94::C0(_, _, _, _, h15) => {
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

pub fn f6() -> Adt94 {
    Adt94::C0(f22(), f23(), List::<List<u64>>::nil(), f24(), None::<List<u64>>)
}

pub fn f7(v0: Adt0, v1: Fn0, v2: Adt91) -> R<Adt94> {
    f25(v0, v1, v2.clone(), f26(match v2.clone() {
        Adt91::C0(_, _, h30, _, _, _, _) => {
            h30
        }
    }, match v2 {
        Adt91::C0(_, _, _, _, _, h31, _) => {
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
    let a33 = f30()?;
    let a34 = f31();
    Ok(append_list(a33, a34))
}

pub fn f13() -> u64 {
    100u64
}

pub fn f14(v0: List<Adt36>) -> R<Option<Adt90>> {
    let m35 = f29(v0)?;
    match m35 {
        Ok(_) => {
            Ok(None::<Adt90>)
        }
        Err(v1) => {
            Ok(Some(Adt90::C9(f32(v1))))
        }
    }
}

pub fn f15(v0: Adt14, v1: Adt15) -> R<Result<Adt33, Adt90>> {
    let m36 = v1.clone();
    match m36 {
        Adt15::C0(_) => {
            f39(v1)
        }
        Adt15::C1(_) => {
            f39(v1)
        }
        Adt15::C2(v2) => {
            f33(v0, f12()?, Fn1::F34, Fn2::F35, v2)
        }
        Adt15::C3(_) => {
            f39(v1)
        }
        Adt15::C4(_) => {
            f39(v1)
        }
        Adt15::C5(_) => {
            f39(v1)
        }
        Adt15::C6(_) => {
            f39(v1)
        }
        Adt15::C7(_) => {
            f39(v1)
        }
        Adt15::C8(_) => {
            f39(v1)
        }
        Adt15::C9(v3) => {
            f36(v0, v3)
        }
        Adt15::C10(v4) => {
            f37(v0, v4)
        }
        Adt15::C11(v5) => {
            f38(v0, v5)
        }
    }
}

pub fn f16(v0: Bytes, v1: u64) -> R<Option<Adt95>> {
    let m37 = f40(v0.clone(), v1)?;
    match m37 {
        None => {
            Ok(None::<Adt95>)
        }
        Some(v2) => {
            let m43 = {
                let a40 = v0;
                let a41 = {
                    let a38 = v1;
                    let a39 = 4u64;
                    nat_add(a38, a39)?
                };
                let a42 = v2;
                slice_bytes(a40, a41, a42)
            };
            match m43 {
                None => {
                    Ok(None::<Adt95>)
                }
                Some(v3) => {
                    Ok(Some(Adt95::C0(v3, {
                        let a46 = {
                            let a44 = v1;
                            let a45 = 4u64;
                            nat_add(a44, a45)?
                        };
                        let a47 = v2;
                        nat_add(a46, a47)?
                    })))
                }
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: Adt95, v2: Option<Adt95>) -> R<Result<Adt91, Adt93>> {
    let m48 = v2;
    match m48 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(match v1 {
                Adt95::C0(_, h49) => {
                    h49
                }
            })))
        }
        Some(v3) => {
            f41(v0.clone(), v1, v3.clone(), f40(v0, match v3 {
                Adt95::C0(_, h50) => {
                    h50
                }
            })?)
        }
    }
}

pub fn f18(v0: u64) -> List<u64> {
    List::cons({
        let a54 = {
            let a51 = v0;
            let a52 = 16777216u64;
            let a53 = 0u64;
            nat_quot(a51, a52, a53)
        };
        let a55 = 256u64;
        let a56 = 0u64;
        nat_rem(a54, a55, a56)
    }, List::cons({
        let a60 = {
            let a57 = v0;
            let a58 = 65536u64;
            let a59 = 0u64;
            nat_quot(a57, a58, a59)
        };
        let a61 = 256u64;
        let a62 = 0u64;
        nat_rem(a60, a61, a62)
    }, List::cons({
        let a66 = {
            let a63 = v0;
            let a64 = 256u64;
            let a65 = 0u64;
            nat_quot(a63, a64, a65)
        };
        let a67 = 256u64;
        let a68 = 0u64;
        nat_rem(a66, a67, a68)
    }, List::cons({
        let a69 = v0;
        let a70 = 256u64;
        let a71 = 0u64;
        nat_rem(a69, a70, a71)
    }, List::<u64>::nil()))))
}

pub fn f19(v0: List<Adt92>) -> List<u64> {
    f42(f43(v0, List::<Adt92>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f44(f45(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21(v0: Option<List<u64>>) -> List<u64> {
    let m72 = v0;
    match m72 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a76 = f18(1u64);
            let a77 = {
                let a74 = f18({
                    let a73 = v1.clone();
                    length_list(a73)
                });
                let a75 = v1;
                append_list(a74, a75)
            };
            append_list(a76, a77)
        }
    }
}

pub fn f22() -> u64 {
    500u64
}

pub fn f23() -> List<Adt92> {
    List::cons(Adt92::C0(f46(), f47()), List::<Adt92>::nil())
}

pub fn f24() -> List<u64> {
    let a80 = f48();
    let a81 = {
        let a78 = f49();
        let a79 = f50();
        append_list(a78, a79)
    };
    append_list(a80, a81)
}

pub fn f25(v0: Adt0, v1: Fn0, v2: Adt91, v3: Option<Adt96>) -> R<Adt94> {
    let m82 = v3;
    match m82 {
        None => {
            f52(v0, v1, v2)
        }
        Some(v4) => {
            f51(v4)
        }
    }
}

pub fn f26(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m83 = f53(v0.clone(), v1.clone())?;
    match m83 {
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

pub fn f29(v0: List<Adt36>) -> R<Result<List<Adt12>, Adt97>> {
    let m84 = f55(v0.clone());
    match m84 {
        None => {
            f56(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f30() -> R<List<Adt36>> {
    Ok(List::cons(f57(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f58()), List::cons(f57(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f59()), List::cons(f57(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f58()), List::cons(f57(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f60()), List::cons(f57(Str::lit("test_image_content"), Str::lit("Tests image content response"), f60()), List::cons(f57(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f60()), List::cons(f57(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f60()), List::cons(f57(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f60()), List::cons(f57(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f60()), List::cons(f57(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f60()), List::cons(f57(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f61()), List::cons(f57(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f62()?), List::cons(f57(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f63()), List::cons(f64(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f60(), f65()), List::cons(f64(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f60(), f65()), List::cons(f57(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f60()), List::cons(f57(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f60()), List::cons(f57(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f60()), List::<Adt36>::nil())))))))))))))))))))
}

pub fn f31() -> List<Adt36> {
    List::cons(f66(Str::lit("greet"), Str::lit("Answers a greeting at once; never a task"), f67(Str::lit("name"))), List::cons(f66(Str::lit("slow_compute"), Str::lit("Computes for a number of seconds; a task for a client that declared the tasks extension"), f68()), List::cons(f66(Str::lit("failing_job"), Str::lit("Always fails; always a task, so the client must declare the tasks extension"), f69()), List::cons(f66(Str::lit("protocol_error_job"), Str::lit("Ends in a protocol error; a task for a client that declared the tasks extension"), f69()), List::cons(f66(Str::lit("confirm_delete"), Str::lit("Asks the client to confirm a deletion; a task that waits for input"), f67(Str::lit("filename"))), List::cons(f66(Str::lit("multi_input"), Str::lit("Asks the client two questions at once; a task that waits for both answers"), f69()), List::cons(f66(Str::lit("test_tool_with_task"), Str::lit("Asks for a name, then becomes a task; the client must declare the tasks extension"), f69()), List::<Adt36>::nil())))))))
}

pub fn f32(v0: Adt97) -> Str {
    let m85 = v0;
    match m85 {
        Adt97::C0(v1) => {
            let a86 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a87 = Str::lit(" ");
            join(a86, a87)
        }
        Adt97::C1(v2) => {
            let a88 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a89 = Str::lit(" ");
            join(a88, a89)
        }
        Adt97::C2(v3) => {
            let a90 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a91 = Str::lit(" ");
            join(a90, a91)
        }
        Adt97::C3(v4) => {
            let a92 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a93 = Str::lit(" ");
            join(a92, a93)
        }
        Adt97::C4(v5, v6) => {
            let a94 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a95 = Str::lit(" ");
            join(a94, a95)
        }
        Adt97::C5(v7, v8) => {
            let a96 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a97 = Str::lit(" ");
            join(a96, a97)
        }
        Adt97::C6(v9, v10) => {
            let a98 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a99 = Str::lit(" ");
            join(a98, a99)
        }
        Adt97::C7(v11) => {
            let a100 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a101 = Str::lit(" ");
            join(a100, a101)
        }
        Adt97::C8(v12) => {
            let a102 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a103 = Str::lit(" ");
            join(a102, a103)
        }
        Adt97::C9(v13) => {
            let a104 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a105 = Str::lit(" ");
            join(a104, a105)
        }
        Adt97::C10(v14) => {
            let a106 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a107 = Str::lit(" ");
            join(a106, a107)
        }
        Adt97::C11(v15) => {
            let a108 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a109 = Str::lit(" ");
            join(a108, a109)
        }
        Adt97::C12(v16, v17) => {
            let a110 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a111 = Str::lit(" ");
            join(a110, a111)
        }
    }
}

pub fn f33(v0: Adt14, v1: List<Adt36>, v2: Fn1, v3: Fn2, v4: Adt22) -> R<Result<Adt33, Adt90>> {
    let m113 = f70(match v4.clone() {
        Adt22::C0(_, h112, _, _, _) => {
            h112
        }
    }, v1.clone());
    match m113 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v4 {
                Adt22::C0(_, h114, _, _, _) => {
                    h114
                }
            })))
        }
        Some(v5) => {
            f71(v0, v5, v4.clone(), {
                let c115: Fn1 = v2;
                c115.apply(v4.clone())?
            }, f72(v1, v3, v4)?)
        }
    }
}

pub fn f34(v0: Adt22) -> R<Option<Adt98>> {
    if f73(v0.clone(), Str::lit("greet")) {
        Ok(Some(Adt98::C0(f74({
            let a116 = List::cons(Str::lit("Hello, "), List::cons(f75(v0, Str::lit("name"), Str::lit("world")), List::cons(Str::lit("!"), List::<Str>::nil())));
            let a117 = Str::lit("");
            join(a116, a117)
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
                                Ok(None::<Adt98>)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: Adt22) -> R<Result<Adt50, Adt90>> {
    if f82(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt50, Adt90>(f83(List::cons(f84(Str::lit("This is a simple text response for testing.")), List::<Adt39>::nil()))))
    } else {
        if f82(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt50, Adt90>(f83(List::cons(f85(), List::<Adt39>::nil()))))
        } else {
            if f82(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt50, Adt90>(f83(List::cons(f86(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt39>::nil()))))
            } else {
                if f82(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt50, Adt90>(f83(List::cons(f87(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt39>::nil()))))
                } else {
                    if f82(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt50, Adt90>(f83(List::cons(f84(Str::lit("Multiple content types test:")), List::cons(f85(), List::cons(f87(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt39>::nil()))))))
                    } else {
                        if f82(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt50, Adt90>(f88(List::cons(f84(Str::lit("This tool intentionally returns an error for testing")), List::<Adt39>::nil()), None::<Adt6>, true, List::<Adt51>::nil())))
                        } else {
                            if f82(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt50, Adt90>(f89(v0, f90()?)))
                            } else {
                                if f82(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt50, Adt90>(f89(v0, f91()?)))
                                } else {
                                    if f82(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt50, Adt90>(f88(List::cons(f84(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt39>::nil()), Some(f92()?), false, List::<Adt51>::nil())))
                                    } else {
                                        if f82(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt50, Adt90>(f88(List::cons(f84(Str::lit("{\"sum\":\"three\"}")), List::<Adt39>::nil()), Some(f93()), false, List::<Adt51>::nil())))
                                        } else {
                                            if f82(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt50, Adt90>(f83(List::cons(f94(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt39>::nil()))))
                                            } else {
                                                if f82(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt50, Adt90>(f83(List::cons(f84(f95()), List::<Adt39>::nil()))))
                                                } else {
                                                    Ok(f96(v0))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f36(v0: Adt14, v1: Adt30) -> R<Result<Adt33, Adt90>> {
    let m120 = f97(v0.clone(), match v1.clone() {
        Adt30::C0(h118, _) => {
            h118
        }
    }, match v1 {
        Adt30::C0(_, h119) => {
            h119
        }
    })?;
    match m120 {
        Ok(v3) => {
            f98(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f37(v0: Adt14, v1: Adt31) -> R<Result<Adt33, Adt90>> {
    let m123 = f97(v0.clone(), match v1.clone() {
        Adt31::C0(h121, _, _) => {
            h121
        }
    }, match v1.clone() {
        Adt31::C0(_, h122, _) => {
            h122
        }
    })?;
    match m123 {
        Ok(v3) => {
            f99(v0, v3, match v1 {
                Adt31::C0(_, _, h124) => {
                    h124
                }
            })
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f38(v0: Adt14, v1: Adt32) -> R<Result<Adt33, Adt90>> {
    let m127 = f97(v0.clone(), match v1.clone() {
        Adt32::C0(h125, _) => {
            h125
        }
    }, match v1 {
        Adt32::C0(_, h126) => {
            h126
        }
    })?;
    match m127 {
        Ok(v3) => {
            f100(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f39(v0: Adt15) -> R<Result<Adt33, Adt90>> {
    let m128 = v0;
    match m128 {
        Adt15::C0(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt15::C1(v1) => {
            Ok(f101(f10(), f12()?, v1))
        }
        Adt15::C2(v2) => {
            f72(f12()?, Fn2::F35, v2)
        }
        Adt15::C3(v4) => {
            Ok(f102(v4))
        }
        Adt15::C4(v5) => {
            f103(v5)
        }
        Adt15::C5(v6) => {
            Ok(f104(v6))
        }
        Adt15::C6(v7) => {
            Ok(f105(v7))
        }
        Adt15::C7(v8) => {
            Ok(f106(v8))
        }
        Adt15::C8(v9) => {
            f107(v9)
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

pub fn f40(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a132 = {
        let a129 = v1;
        let a130 = 4u64;
        nat_add(a129, a130)?
    };
    let a133 = {
        let a131 = v0.clone();
        length_bytes(a131)
    };
    if nat_le(a132, a133) {
        Ok(Some({
            let a150 = {
                let a134 = f108(v0.clone(), v1)?;
                let a135 = 16777216u64;
                nat_mul(a134, a135)?
            };
            let a151 = {
                let a148 = {
                    let a138 = f108(v0.clone(), {
                        let a136 = v1;
                        let a137 = 1u64;
                        nat_add(a136, a137)?
                    })?;
                    let a139 = 65536u64;
                    nat_mul(a138, a139)?
                };
                let a149 = {
                    let a146 = {
                        let a142 = f108(v0.clone(), {
                            let a140 = v1;
                            let a141 = 2u64;
                            nat_add(a140, a141)?
                        })?;
                        let a143 = 256u64;
                        nat_mul(a142, a143)?
                    };
                    let a147 = f108(v0, {
                        let a144 = v1;
                        let a145 = 3u64;
                        nat_add(a144, a145)?
                    })?;
                    nat_add(a146, a147)?
                };
                nat_add(a148, a149)?
            };
            nat_add(a150, a151)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f41(v0: Bytes, v1: Adt95, v2: Adt95, v3: Option<u64>) -> R<Result<Adt91, Adt93>> {
    let m152 = v3;
    match m152 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(match v2 {
                Adt95::C0(_, h153) => {
                    h153
                }
            })))
        }
        Some(v4) => {
            let m157 = f109(v0.clone(), {
                let a155 = match v2.clone() {
                    Adt95::C0(_, h154) => {
                        h154
                    }
                };
                let a156 = 4u64;
                nat_add(a155, a156)?
            }, v4, List::<Adt92>::nil())?;
            match m157 {
                Ok(v6) => {
                    f110(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt91, Adt93>(v5))
                }
            }
        }
    }
}

pub fn f42(p162: List<Adt92>, p163: List<u64>) -> List<u64> {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v0, v1) = m161;
                let m158 = v0;
                match m158.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<u64>), List<u64>>((v3, {
                            let a159 = f111(v2);
                            let a160 = v1;
                            append_list(a159, a160)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(p166: List<Adt92>, p167: List<Adt92>) -> List<Adt92> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v0, v1) = m165;
                let m164 = v0;
                match m164.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
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

pub fn f44(p172: List<List<u64>>, p173: List<u64>) -> List<u64> {
    let m171 = (p172, p173);
    {
        let mut m171 = m171;
        loop {
            let step = {
                let (v0, v1) = m171;
                let m168 = v0;
                match m168.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a169 = f112(v2);
                            let a170 = v1;
                            append_list(a169, a170)
                        }))
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

pub fn f45(p176: List<List<u64>>, p177: List<List<u64>>) -> List<List<u64>> {
    let m175 = (p176, p177);
    {
        let mut m175 = m175;
        loop {
            let step = {
                let (v0, v1) = m175;
                let m174 = v0;
                match m174.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
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

pub fn f51(v0: Adt96) -> R<Adt94> {
    f113(f114(v0.clone()), f115(None::<Adt100>, f116(v0)))
}

pub fn f52(v0: Adt0, v1: Fn0, v2: Adt91) -> R<Adt94> {
    if f117(match v2.clone() {
        Adt91::C0(h178, _, _, _, _, _, _) => {
            h178
        }
    }) {
        f118(match v2.clone() {
            Adt91::C0(_, _, _, _, _, _, h179) => {
                h179
            }
        }, v0, Fn3::F119(v1, v2.clone()), match v2.clone() {
            Adt91::C0(_, _, _, _, h180, _, _) => {
                h180
            }
        }, match v2.clone() {
            Adt91::C0(_, _, h181, _, _, _, _) => {
                h181
            }
        }, f120(match v2 {
            Adt91::C0(_, _, _, h182, _, _, _) => {
                h182
            }
        })?)
    } else {
        Ok(f121())
    }
}

pub fn f53(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m183 = f122(v0, f123())?;
    match m183 {
        Adt105::C0 => {
            Ok(None::<Adt96>)
        }
        Adt105::C1 => {
            Ok(Some(Adt96::C0))
        }
        Adt105::C2 => {
            Ok(Some(Adt96::C0))
        }
        Adt105::C3(v2) => {
            f124(f125(v2), v1, Adt96::C0)
        }
    }
}

pub fn f54(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m184 = f122(v0, f126())?;
    match m184 {
        Adt105::C0 => {
            Ok(None::<Adt96>)
        }
        Adt105::C1 => {
            Ok(Some(Adt96::C1))
        }
        Adt105::C2 => {
            Ok(Some(Adt96::C1))
        }
        Adt105::C3(v2) => {
            f124(f127(v2), v1, Adt96::C1)
        }
    }
}

pub fn f55(v0: List<Adt36>) -> Option<Adt97> {
    let m185 = f128(v0, List::<Str>::nil());
    match m185 {
        None => {
            None::<Adt97>
        }
        Some(v1) => {
            Some(Adt97::C1(v1))
        }
    }
}

pub fn f56(p189: List<Adt36>, p190: List<Adt12>) -> R<Result<List<Adt12>, Adt97>> {
    let m188 = (p189, p190);
    {
        let mut m188 = m188;
        loop {
            let step = {
                let (v0, v1) = m188;
                let m186 = v0;
                match m186.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>(Ok(Ok::<List<Adt12>, Adt97>(f129(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m187 = f130(v2)?;
                        match m187 {
                            Ok(v5) => {
                                Ok::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>((v3, f129(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>(Ok(Err::<List<Adt12>, Adt97>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m188 = next; }
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
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f131(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f132()), List::cons((Str::lit("limit"), f131(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f131(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f60() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f61() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f62() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f132()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f133(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f133(0u64)?), List::cons((Str::lit("maximum"), f133(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f133(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
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

pub fn f66(v0: Str, v1: Str, v2: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f67(v0: Str) -> Adt5 {
    f134(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v0.clone(), f135(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v0), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f68() -> Adt5 {
    f134(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("seconds"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), Adt6::C2(Adt7::C0(0i64, 0i64))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("label"), f135(Str::lit("string"))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("seconds")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f69() -> Adt5 {
    f134(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f70(v0: Str, p196: List<Adt36>) -> Option<Adt36> {
    let m195 = p196;
    {
        let mut m195 = m195;
        loop {
            let step = {
                let v1 = m195;
                let m191 = v1;
                match m191.uncons() {
                    None => {
                        Err::<List<Adt36>, Option<Adt36>>(None::<Adt36>)
                    }
                    Some((v2, v3)) => {
                        let a193 = match v2.clone() {
                            Adt36::C0(h192, _, _, _, _, _, _, _) => {
                                h192
                            }
                        };
                        let a194 = v0.clone();
                        if equal(a193, a194) {
                            Err::<List<Adt36>, Option<Adt36>>(Some(v2))
                        } else {
                            Ok::<List<Adt36>, Option<Adt36>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: Adt14, v1: Adt36, v2: Adt22, v3: Option<Adt98>, v4: Result<Adt33, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m197 = v3;
    match m197 {
        None => {
            Ok(v4)
        }
        Some(v5) => {
            f136(v0, v1.clone(), v2.clone(), v5, f137(v1, v2)?)
        }
    }
}

pub fn f72(v0: List<Adt36>, v1: Fn2, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m199 = f70(match v2.clone() {
        Adt22::C0(_, h198, _, _, _) => {
            h198
        }
    }, v0);
    match m199 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h200, _, _, _) => {
                    h200
                }
            })))
        }
        Some(v3) => {
            f138(v3, v1, v2)
        }
    }
}

pub fn f73(v0: Adt22, v1: Str) -> bool {
    let a202 = match v0 {
        Adt22::C0(_, h201, _, _, _) => {
            h201
        }
    };
    let a203 = v1;
    equal(a202, a203)
}

pub fn f74(v0: Str, v1: bool) -> Adt50 {
    Adt50::C0(Adt38::C0(List::cons(f139(v0), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt51>::nil())
}

pub fn f75(v0: Adt22, v1: Str, v2: Str) -> Str {
    let m204 = f140(v0, v1);
    match m204 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            let m205 = f141(v3);
            match m205 {
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

pub fn f76(v0: Adt22) -> R<Adt98> {
    let v1: i64 = f142(v0.clone());
    let v2: Str = {
        let a207 = List::cons(Str::lit("slow_compute"), List::cons(f75(v0, Str::lit("label"), Str::lit("unnamed")), List::cons(Str::lit("finished after"), List::cons({
            let a206 = v1;
            format_i64(a206)
        }, List::cons(Str::lit("seconds"), List::<Str>::nil())))));
        let a208 = Str::lit(" ");
        join(a207, a208)
    };
    Ok(Adt98::C1(Adt99::C0, f74(v2.clone(), false), f143(0i64, List::<(Str, Adt65)>::nil(), {
        let a209 = v1;
        let a210 = 1000i64;
        int_mul(a209, a210)?
    }, f144(v2, false)?)))
}

pub fn f77(v0: Adt22) -> R<Adt98> {
    let v1: Str = {
        let a211 = List::cons(Str::lit("Deleted"), List::cons(f75(v0.clone(), Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a212 = Str::lit(" ");
        join(a211, a212)
    };
    Ok(Adt98::C1(Adt99::C0, f74(v1.clone(), false), f143(0i64, List::cons((Str::lit("confirm"), f145({
        let a213 = List::cons(Str::lit("Delete"), List::cons(f75(v0, Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a214 = Str::lit(" ");
        join(a213, a214)
    }, Str::lit("confirm"), Str::lit("boolean"))), List::<(Str, Adt65)>::nil()), 0i64, f144(v1, false)?)))
}

pub fn f78() -> R<Adt98> {
    Ok(Adt98::C1(Adt99::C0, f74(Str::lit("Both inputs received"), false), f143(0i64, List::cons((Str::lit("first"), f145(Str::lit("First name"), Str::lit("name"), Str::lit("string"))), List::cons((Str::lit("second"), f145(Str::lit("Second name"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt65)>::nil())), 0i64, f144(Str::lit("Both inputs received"), false)?)))
}

pub fn f79() -> R<Adt98> {
    Ok(Adt98::C1(Adt99::C1, f74(Str::lit("failing_job always fails"), true), f143(0i64, List::<(Str, Adt65)>::nil(), 1000i64, f144(Str::lit("failing_job always fails"), true)?)))
}

pub fn f80() -> Adt98 {
    Adt98::C1(Adt99::C0, f74(Str::lit("protocol_error_job has no result"), true), f143(0i64, List::<(Str, Adt65)>::nil(), 500i64, Adt87::C1(Adt81::C0(-32603i64, Str::lit("protocol_error_job failed"), None::<Adt6>))))
}

pub fn f81(v0: Adt22) -> R<Adt98> {
    let m216 = match v0 {
        Adt22::C0(_, _, _, h215, _) => {
            h215
        }
    };
    match m216 {
        None => {
            Ok(f146())
        }
        Some(v1) => {
            let m217 = f147(v1);
            match m217 {
                None => {
                    Ok(f146())
                }
                Some(v2) => {
                    f148(v2)
                }
            }
        }
    }
}

pub fn f82(v0: Adt22, v1: Str) -> bool {
    let a219 = match v0 {
        Adt22::C0(_, h218, _, _, _) => {
            h218
        }
    };
    let a220 = v1;
    equal(a219, a220)
}

pub fn f83(v0: List<Adt39>) -> Adt50 {
    f88(v0, None::<Adt6>, false, List::<Adt51>::nil())
}

pub fn f84(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f85() -> Adt39 {
    f94(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f86(v0: Str, v1: Str) -> Adt39 {
    Adt39::C2(Adt44::C0(v0, v1, None::<Adt41>, None::<Adt5>))
}

pub fn f87(v0: Str, v1: Str, v2: Str) -> Adt39 {
    Adt39::C4(Adt46::C0(Adt47::C0(Adt48::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt41>, None::<Adt5>))
}

pub fn f88(v0: List<Adt39>, v1: Option<Adt6>, v2: bool, v3: List<Adt51>) -> Adt50 {
    Adt50::C0(Adt38::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f89(v0: Adt22, v1: List<Adt51>) -> Adt50 {
    f88(List::cons(f84(f149({
        let m223 = match v0 {
            Adt22::C0(h222, _, _, _, _) => {
                h222
            }
        };
        match m223 {
            Adt17::C0(_, _, _, _, h221, _) => {
                h221
            }
        }
    })), List::<Adt39>::nil()), None::<Adt6>, false, v1)
}

pub fn f90() -> R<List<Adt51>> {
    Ok(List::cons(f150(0u64, 100u64)?, List::cons(f150(50u64, 100u64)?, List::cons(f150(100u64, 100u64)?, List::<Adt51>::nil()))))
}

pub fn f91() -> R<List<Adt51>> {
    Ok(List::cons(f150(50u64, 100u64)?, List::cons(f150(10u64, 100u64)?, List::<Adt51>::nil())))
}

pub fn f92() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f133(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f93() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f94(v0: Str, v1: Str) -> Adt39 {
    Adt39::C1(Adt43::C0(v0, v1, None::<Adt41>, None::<Adt5>))
}

pub fn f95() -> Str {
    let m225 = {
        let a224 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a224)
    };
    match m225 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f96(v0: Adt22) -> Result<Adt50, Adt90> {
    if f82(v0.clone(), Str::lit("test_missing_capability")) {
        let m231 = {
            let m230 = {
                let m229 = match v0 {
                    Adt22::C0(h228, _, _, _, _) => {
                        h228
                    }
                };
                match m229 {
                    Adt17::C0(_, h227, _, _, _, _) => {
                        h227
                    }
                }
            };
            match m230 {
                Adt18::C0(_, _, h226, _, _) => {
                    h226
                }
            }
        };
        match m231 {
            None => {
                Err::<Adt50, Adt90>(Adt90::C8(f151()))
            }
            Some(_) => {
                Ok::<Adt50, Adt90>(f152())
            }
        }
    } else {
        if f82(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt50, Adt90>(f83(List::cons(f84(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt39>::nil())))
        } else {
            if f82(v0, Str::lit("test_arguments")) {
                Ok::<Adt50, Adt90>(f83(List::cons(f84(Str::lit("arguments accepted")), List::<Adt39>::nil())))
            } else {
                Ok::<Adt50, Adt90>(f152())
            }
        }
    }
}

pub fn f97(v0: Adt14, v1: Adt17, v2: Str) -> R<Result<Adt85, Adt90>> {
    if f153(match v1 {
        Adt17::C0(_, h232, _, _, _, _) => {
            h232
        }
    }) {
        let m234 = f154(match v0.clone() {
            Adt14::C0(_, _, _, h233) => {
                h233
            }
        })?;
        match m234 {
            None => {
                Ok(Err::<Adt85, Adt90>(f155()))
            }
            Some(v3) => {
                let m236 = f156(v2.clone(), v3, match v0 {
                    Adt14::C0(h235, _, _, _) => {
                        h235
                    }
                })?;
                match m236 {
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
        Ok(Err::<Adt85, Adt90>(f157()))
    }
}

pub fn f98(v0: Adt14, v1: Adt85) -> R<Result<Adt33, Adt90>> {
    Ok(Ok::<Adt33, Adt90>(f158(f159(v1, match v0.clone() {
        Adt14::C0(h237, _, _, _) => {
            h237
        }
    }, match v0 {
        Adt14::C0(_, h238, _, _) => {
            h238
        }
    })?, Adt89::C0, false)))
}

pub fn f99(v0: Adt14, v1: Adt85, v2: Adt5) -> R<Result<Adt33, Adt90>> {
    let v3: Adt101 = f159(v1, match v0.clone() {
        Adt14::C0(h239, _, _, _) => {
            h239
        }
    }, match v0.clone() {
        Adt14::C0(_, h240, _, _) => {
            h240
        }
    })?;
    Ok(Ok::<Adt33, Adt90>(f158(f160(match v3.clone() {
        Adt101::C0(h241, _) => {
            h241
        }
    }, v2, match v0.clone() {
        Adt14::C0(h242, _, _, _) => {
            h242
        }
    }, match v0 {
        Adt14::C0(_, h243, _, _) => {
            h243
        }
    })?, Adt89::C1, match v3 {
        Adt101::C0(_, h244) => {
            h244
        }
    })))
}

pub fn f100(v0: Adt14, v1: Adt85) -> R<Result<Adt33, Adt90>> {
    Ok(Ok::<Adt33, Adt90>(f158(f161(v1, match v0.clone() {
        Adt14::C0(h245, _, _, _) => {
            h245
        }
    }, match v0 {
        Adt14::C0(_, h246, _, _) => {
            h246
        }
    })?, Adt89::C1, false)))
}

pub fn f101(v0: Adt10, v1: List<Adt36>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m248 = match v2 {
        Adt21::C0(_, h247) => {
            h247
        }
    };
    match m248 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C1(Adt35::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f102(v0: Adt21) -> Result<Adt33, Adt90> {
    f162(f163(), f164(), v0)
}

pub fn f103(v0: Adt23) -> R<Result<Adt33, Adt90>> {
    f165(f166(), Fn4::F167, v0)
}

pub fn f104(v0: Adt21) -> Result<Adt33, Adt90> {
    f168(f163(), f169(), v0)
}

pub fn f105(v0: Adt21) -> Result<Adt33, Adt90> {
    f170(f163(), f171(), v0)
}

pub fn f106(v0: Adt24) -> Result<Adt33, Adt90> {
    f172(f171(), Fn5::F173, v0)
}

pub fn f107(v0: Adt25) -> R<Result<Adt33, Adt90>> {
    f174(Fn6::F175, v0)
}

pub fn f108(v0: Bytes, v1: u64) -> R<u64> {
    let m251 = {
        let a249 = v0;
        let a250 = v1;
        index_bytes(a249, a250)
    };
    match m251 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a296 = {
                let a272 = {
                    let a260 = {
                        let a254 = {
                            let a252 = v3;
                            let a253 = 128u8;
                            fixed_u8::bit_and(a252, a253)
                        };
                        let a255 = 128u8;
                        if equal(a254, a255) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a261 = {
                        let a258 = {
                            let a256 = v3;
                            let a257 = 64u8;
                            fixed_u8::bit_and(a256, a257)
                        };
                        let a259 = 64u8;
                        if equal(a258, a259) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a260, a261)?
                };
                let a273 = {
                    let a270 = {
                        let a264 = {
                            let a262 = v3;
                            let a263 = 32u8;
                            fixed_u8::bit_and(a262, a263)
                        };
                        let a265 = 32u8;
                        if equal(a264, a265) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a271 = {
                        let a268 = {
                            let a266 = v3;
                            let a267 = 16u8;
                            fixed_u8::bit_and(a266, a267)
                        };
                        let a269 = 16u8;
                        if equal(a268, a269) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a270, a271)?
                };
                nat_add(a272, a273)?
            };
            let a297 = {
                let a294 = {
                    let a282 = {
                        let a276 = {
                            let a274 = v3;
                            let a275 = 8u8;
                            fixed_u8::bit_and(a274, a275)
                        };
                        let a277 = 8u8;
                        if equal(a276, a277) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a283 = {
                        let a280 = {
                            let a278 = v3;
                            let a279 = 4u8;
                            fixed_u8::bit_and(a278, a279)
                        };
                        let a281 = 4u8;
                        if equal(a280, a281) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a282, a283)?
                };
                let a295 = {
                    let a292 = {
                        let a286 = {
                            let a284 = v3;
                            let a285 = 2u8;
                            fixed_u8::bit_and(a284, a285)
                        };
                        let a287 = 2u8;
                        if equal(a286, a287) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a293 = {
                        let a290 = {
                            let a288 = v3;
                            let a289 = 1u8;
                            fixed_u8::bit_and(a288, a289)
                        };
                        let a291 = 1u8;
                        if equal(a290, a291) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a292, a293)?
                };
                nat_add(a294, a295)?
            };
            nat_add(a296, a297)
        }
    }
}

pub fn f109(v0: Bytes, p307: u64, p308: u64, p309: List<Adt92>) -> R<Result<(List<Adt92>, u64), Adt93>> {
    let m306 = (p307, (p308, p309));
    {
        let mut m306 = m306;
        loop {
            let step = {
                let (v1, (v2, v3)) = m306;
                let m298 = v2;
                if m298 == 0 {
                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Ok::<(List<Adt92>, u64), Adt93>((f176(v3, List::<Adt92>::nil()), v1))))
                } else {
                    let v4 = m298 - 1;
                    let m299 = f16(v0.clone(), v1)?;
                    match m299 {
                        None => {
                            Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Err::<(List<Adt92>, u64), Adt93>(Adt93::C0(v1))))
                        }
                        Some(v5) => {
                            let m301 = f16(v0.clone(), match v5.clone() {
                                Adt95::C0(_, h300) => {
                                    h300
                                }
                            })?;
                            match m301 {
                                None => {
                                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Err::<(List<Adt92>, u64), Adt93>(Adt93::C0(match v5 {
                                        Adt95::C0(_, h302) => {
                                            h302
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>((match v6.clone() {
                                        Adt95::C0(_, h303) => {
                                            h303
                                        }
                                    }, (v4, List::cons(Adt92::C0(f177(match v5 {
                                        Adt95::C0(h304, _) => {
                                            h304
                                        }
                                    })?, f177(match v6 {
                                        Adt95::C0(h305, _) => {
                                            h305
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: Bytes, v1: Adt95, v2: Adt95, v3: (List<Adt92>, u64)) -> R<Result<Adt91, Adt93>> {
    let m311 = f16(v0.clone(), {
        let (_, h310) = v3.clone();
        h310
    })?;
    match m311 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0({
                let (_, h312) = v3;
                h312
            })))
        }
        Some(v4) => {
            let m314 = f40(v0.clone(), match v4.clone() {
                Adt95::C0(_, h313) => {
                    h313
                }
            })?;
            match m314 {
                None => {
                    Ok(Err::<Adt91, Adt93>(Adt93::C0(match v4 {
                        Adt95::C0(_, h315) => {
                            h315
                        }
                    })))
                }
                Some(v5) => {
                    f178(v0.clone(), Adt91::C0(f177(match v1 {
                        Adt95::C0(h316, _) => {
                            h316
                        }
                    })?, f177(match v2 {
                        Adt95::C0(h317, _) => {
                            h317
                        }
                    })?, {
                        let (h318, _) = v3;
                        h318
                    }, match v4.clone() {
                        Adt95::C0(h319, _) => {
                            h319
                        }
                    }, v5, List::<List<u64>>::nil(), Adt14::C0(0i64, Str::lit(""), Str::lit(""), f179())), f180(v0, {
                        let a321 = match v4 {
                            Adt95::C0(_, h320) => {
                                h320
                            }
                        };
                        let a322 = 4u64;
                        nat_add(a321, a322)?
                    })?)
                }
            }
        }
    }
}

pub fn f111(v0: Adt92) -> List<u64> {
    let a333 = f18({
        let a324 = match v0.clone() {
            Adt92::C0(h323, _) => {
                h323
            }
        };
        length_list(a324)
    });
    let a334 = {
        let a331 = match v0.clone() {
            Adt92::C0(h325, _) => {
                h325
            }
        };
        let a332 = {
            let a329 = f18({
                let a327 = match v0.clone() {
                    Adt92::C0(_, h326) => {
                        h326
                    }
                };
                length_list(a327)
            });
            let a330 = match v0 {
                Adt92::C0(_, h328) => {
                    h328
                }
            };
            append_list(a329, a330)
        };
        append_list(a331, a332)
    };
    append_list(a333, a334)
}

pub fn f112(v0: List<u64>) -> List<u64> {
    let a336 = f18({
        let a335 = v0.clone();
        length_list(a335)
    });
    let a337 = v0;
    append_list(a336, a337)
}

pub fn f113(v0: u64, v1: Adt6) -> R<Adt94> {
    let m338 = f181(v1)?;
    match m338 {
        Ok(v3) => {
            Ok(Adt94::C0(v0, f23(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f114(v0: Adt96) -> u64 {
    let m339 = v0;
    match m339 {
        Adt96::C0 => {
            f182()
        }
        Adt96::C1 => {
            f182()
        }
    }
}

pub fn f115(v0: Option<Adt100>, v1: Adt81) -> Adt6 {
    Adt6::C5(f183(v0, v1))
}

pub fn f116(v0: Adt96) -> Adt81 {
    Adt81::C0(f184(), f185(v0), None::<Adt6>)
}

pub fn f117(v0: List<u64>) -> bool {
    f186(v0, f187())
}

pub fn f118(v0: Adt14, v1: Adt0, v2: Fn3, v3: u64, v4: List<Adt92>, v5: Result<Adt6, Adt102>) -> R<Adt94> {
    let m340 = v5;
    match m340 {
        Ok(v7) => {
            f190(v0, v1, v2, v3, v4, f191(v7)?)
        }
        Err(_) => {
            f113(f188(), f115(None::<Adt100>, f189(Adt103::C0)))
        }
    }
}

pub fn f119(v0: Fn0, v1: Adt91, v2: Adt15) -> R<Result<Adt33, Adt90>> {
    let c341: Fn0 = v0;
    c341.apply(match v1 {
        Adt91::C0(_, _, _, _, _, _, h342) => {
            h342
        }
    }, v2)
}

pub fn f120(v0: Bytes) -> R<Result<Adt6, Adt102>> {
    f192(f177(v0)?)
}

pub fn f121() -> Adt94 {
    Adt94::C0(f193(), List::cons(Adt92::C0(f194(), f195()), List::<Adt92>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f122(v0: List<Adt92>, v1: List<u64>) -> R<Adt105> {
    let m343 = f196(v0, v1, Adt106::C0)?;
    match m343 {
        Adt106::C0 => {
            Ok(Adt105::C0)
        }
        Adt106::C1(v2) => {
            let m344 = f197(v2);
            match m344 {
                None => {
                    Ok(Adt105::C1)
                }
                Some(v3) => {
                    Ok(Adt105::C3(v3))
                }
            }
        }
        Adt106::C2 => {
            Ok(Adt105::C2)
        }
    }
}

pub fn f123() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f124(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt96) -> R<Option<Adt96>> {
    let m345 = v0;
    match m345 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f198(v3, v1)? {
                Ok(None::<Adt96>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f125(v0: List<u64>) -> Option<List<u64>> {
    let m346 = f199(v0);
    match m346 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f127(v1)
        }
    }
}

pub fn f126() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f127(v0: List<u64>) -> Option<List<u64>> {
    let m347 = v0.clone();
    match m347.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a348 = v1;
            let a349 = 91u64;
            if nat_eq(a348, a349) {
                f200(f201(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f200(f202(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f128(p355: List<Adt36>, p356: List<Str>) -> Option<Str> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v0, v1) = m354;
                let m350 = v0;
                match m350.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f203(match v2.clone() {
                            Adt36::C0(h351, _, _, _, _, _, _, _) => {
                                h351
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt36>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt36::C0(h352, _, _, _, _, _, _, _) => {
                                    h352
                                }
                            }))
                        } else {
                            Ok::<(List<Adt36>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt36::C0(h353, _, _, _, _, _, _, _) => {
                                    h353
                                }
                            }, v1)))
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

pub fn f129(p359: List<Adt12>, p360: List<Adt12>) -> List<Adt12> {
    let m358 = (p359, p360);
    {
        let mut m358 = m358;
        loop {
            let step = {
                let (v0, v1) = m358;
                let m357 = v0;
                match m357.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: Adt36) -> R<Result<List<Adt12>, Adt97>> {
    let m363 = f204(match v0.clone() {
        Adt36::C0(h361, _, _, _, _, _, _, _) => {
            h361
        }
    }, f205(match v0.clone() {
        Adt36::C0(h362, _, _, _, _, _, _, _) => {
            h362
        }
    })?);
    match m363 {
        None => {
            f206(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f131(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f132() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f133(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f207(v0, 0i64)?, 0i64)))
}

pub fn f134(v0: List<(Str, Adt6)>) -> Adt5 {
    Adt5::C0(v0)
}

pub fn f135(v0: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::<(Str, Adt6)>::nil()))
}

pub fn f136(v0: Adt14, v1: Adt36, v2: Adt22, v3: Adt98, v4: Result<Option<Adt33>, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m364 = v4;
    match m364 {
        Ok(v6) => {
            let m365 = v6;
            match m365 {
                None => {
                    f208(v0, v1, v2, v3)
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

pub fn f137(v0: Adt36, v1: Adt22) -> R<Result<Option<Adt33>, Adt90>> {
    let m372 = f209(Adt6::C5({
        let m368 = match v0.clone() {
            Adt36::C0(_, _, _, _, h367, _, _, _) => {
                h367
            }
        };
        match m368 {
            Adt5::C0(h366) => {
                h366
            }
        }
    }), Adt6::C5({
        let m371 = match v0 {
            Adt36::C0(_, _, _, _, h370, _, _, _) => {
                h370
            }
        };
        match m371 {
            Adt5::C0(h369) => {
                h369
            }
        }
    }), f210(v1))?;
    match m372 {
        Adt110::C0 => {
            Ok(Ok::<Option<Adt33>, Adt90>(None::<Adt33>))
        }
        Adt110::C1(v2, v3) => {
            Ok(Ok::<Option<Adt33>, Adt90>(Some(Adt33::C2(f211(f212(v2, v3)?, true)))))
        }
        Adt110::C2(_) => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt110::C3(_) => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt110::C4 => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f138(v0: Adt36, v1: Fn2, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m379 = f209(Adt6::C5({
        let m375 = match v0.clone() {
            Adt36::C0(_, _, _, _, h374, _, _, _) => {
                h374
            }
        };
        match m375 {
            Adt5::C0(h373) => {
                h373
            }
        }
    }), Adt6::C5({
        let m378 = match v0.clone() {
            Adt36::C0(_, _, _, _, h377, _, _, _) => {
                h377
            }
        };
        match m378 {
            Adt5::C0(h376) => {
                h376
            }
        }
    }), f210(v2.clone()))?;
    match m379 {
        Adt110::C0 => {
            f213(v0, v1, v2)
        }
        Adt110::C1(v3, v4) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(f211(f212(v3, v4)?, true))))
        }
        Adt110::C2(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt110::C3(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt110::C4 => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f139(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f140(v0: Adt22, v1: Str) -> Adt107 {
    let m381 = match v0 {
        Adt22::C0(_, _, h380, _, _) => {
            h380
        }
    };
    match m381 {
        None => {
            Adt107::C0
        }
        Some(v2) => {
            f214(v1, v2)
        }
    }
}

pub fn f141(v0: Adt6) -> Option<Str> {
    let m382 = v0;
    match m382 {
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

pub fn f142(v0: Adt22) -> i64 {
    let m383 = f140(v0, Str::lit("seconds"));
    match m383 {
        Adt107::C0 => {
            0i64
        }
        Adt107::C1(v1) => {
            let m384 = f215(v1);
            match m384 {
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

pub fn f143(v0: i64, v1: List<(Str, Adt65)>, v2: i64, v3: Adt87) -> Adt86 {
    Adt86::C0(v0, v1, v2, v3)
}

pub fn f144(v0: Str, v1: bool) -> R<Adt87> {
    let m387 = f216(f217({
        let m386 = f74(v0, v1);
        match m386 {
            Adt50::C0(h385, _) => {
                h385
            }
        }
    })?)?;
    match m387 {
        None => {
            Ok(f218())
        }
        Some(v2) => {
            Ok(Adt87::C0(Adt5::C0(v2)))
        }
    }
}

pub fn f145(v0: Str, v1: Str, v2: Str) -> Adt65 {
    Adt65::C0(Adt66::C0(Adt67::C0(Some(Adt68::C0), v0, f134(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v1.clone(), f135(v2)), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v1), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))))))
}

pub fn f146() -> Adt98 {
    Adt98::C2(Adt82::C0(List::cons((Str::lit("user_name"), f145(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt65)>::nil())))
}

pub fn f147(v0: Adt5) -> Option<Str> {
    let m388 = f214(Str::lit("user_name"), v0);
    match m388 {
        Adt107::C0 => {
            None::<Str>
        }
        Adt107::C1(v1) => {
            let m389 = f219(v1);
            match m389 {
                None => {
                    None::<Str>
                }
                Some(v2) => {
                    let m390 = f214(Str::lit("content"), v2);
                    match m390 {
                        Adt107::C0 => {
                            None::<Str>
                        }
                        Adt107::C1(v3) => {
                            let m391 = f219(v3);
                            match m391 {
                                None => {
                                    None::<Str>
                                }
                                Some(v4) => {
                                    let m392 = f214(Str::lit("name"), v4);
                                    match m392 {
                                        Adt107::C0 => {
                                            None::<Str>
                                        }
                                        Adt107::C1(v5) => {
                                            f141(v5)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f148(v0: Str) -> R<Adt98> {
    let v1: Str = {
        let a393 = List::cons(Str::lit("Hello, "), List::cons(v0, List::cons(Str::lit("! (from a task)"), List::<Str>::nil())));
        let a394 = Str::lit("");
        join(a393, a394)
    };
    Ok(Adt98::C1(Adt99::C1, f74(v1.clone(), false), f143(0i64, List::<(Str, Adt65)>::nil(), 100i64, f144(v1, false)?)))
}

pub fn f149(v0: Option<Adt20>) -> Str {
    let m395 = v0;
    match m395 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m396 = v1;
            match m396 {
                Adt20::C0(v2) => {
                    v2
                }
                Adt20::C1(v3) => {
                    let a397 = v3;
                    format_i64(a397)
                }
            }
        }
    }
}

pub fn f150(v0: u64, v1: u64) -> R<Adt51> {
    Ok(Adt51::C0(f220(v0)?, Some(f220(v1)?), Some({
        let a400 = List::cons(Str::lit("Completed step"), List::cons({
            let a398 = f221(v0, 0i64)?;
            format_i64(a398)
        }, List::cons(Str::lit("of"), List::cons({
            let a399 = f221(v1, 0i64)?;
            format_i64(a399)
        }, List::<Str>::nil()))));
        let a401 = Str::lit(" ");
        join(a400, a401)
    })))
}

pub fn f151() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, Some(f27()), None::<Adt5>, None::<Adt5>)
}

pub fn f152() -> Adt50 {
    f83(List::cons(f84(Str::lit("ok")), List::<Adt39>::nil()))
}

pub fn f153(v0: Adt18) -> bool {
    f222(match v0 {
        Adt18::C0(_, _, _, _, h402) => {
            h402
        }
    }, f28())
}

pub fn f154(v0: Bytes) -> R<Option<List<Adt85>>> {
    let a404 = {
        let a403 = v0.clone();
        length_bytes(a403)
    };
    let a405 = 0u64;
    if nat_eq(a404, a405) {
        Ok(Some(List::<Adt85>::nil()))
    } else {
        let m406 = f120(v0)?;
        match m406 {
            Ok(v2) => {
                let m407 = f223(v2);
                match m407 {
                    None => {
                        Ok(None::<List<Adt85>>)
                    }
                    Some(v3) => {
                        Ok(f224(v3, List::<Adt85>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt85>>)
            }
        }
    }
}

pub fn f155() -> Adt90 {
    Adt90::C9(Str::lit("the task table is not readable"))
}

pub fn f156(v0: Str, p412: List<Adt85>, v2: i64) -> R<Option<Adt85>> {
    let m411 = p412;
    {
        let mut m411 = m411;
        loop {
            let step = {
                let v1 = m411;
                let m408 = v1;
                match m408.uncons() {
                    None => {
                        Err::<List<Adt85>, R<Option<Adt85>>>(Ok(None::<Adt85>))
                    }
                    Some((v3, v4)) => {
                        let a409 = f225(v3.clone());
                        let a410 = v0.clone();
                        if equal(a409, a410) {
                            if f226(v3.clone(), v2)? {
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
                Ok(next) => { m411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157() -> Adt90 {
    Adt90::C8(f227())
}

pub fn f158(v0: Adt101, v1: Adt89, v2: bool) -> Adt33 {
    Adt33::C14(Adt88::C0(match v0.clone() {
        Adt101::C0(h413, _) => {
            h413
        }
    }, v1, if match v0 {
        Adt101::C0(_, h414) => {
            h414
        }
    } {
        true
    } else {
        v2
    }))
}

pub fn f159(v0: Adt85, v1: i64, v2: Str) -> R<Adt101> {
    let m416 = match v0.clone() {
        Adt85::C0(h415, _, _) => {
            h415
        }
    };
    match m416 {
        Adt63::C0(v3) => {
            f228(v0, v3, v1, v2)
        }
        Adt63::C1(_, _) => {
            Ok(f229(v0))
        }
        Adt63::C2(_, _) => {
            Ok(f229(v0))
        }
        Adt63::C3(_, _) => {
            Ok(f229(v0))
        }
        Adt63::C4(_) => {
            Ok(f229(v0))
        }
    }
}

pub fn f160(v0: Adt85, v1: Adt5, v2: i64, v3: Str) -> R<Adt101> {
    let m418 = match v0.clone() {
        Adt85::C0(h417, _, _) => {
            h417
        }
    };
    match m418 {
        Adt63::C0(_) => {
            Ok(f229(v0))
        }
        Adt63::C1(v5, v6) => {
            f230(v0, v5, v6, v1, v2, v3)
        }
        Adt63::C2(_, _) => {
            Ok(f229(v0))
        }
        Adt63::C3(_, _) => {
            Ok(f229(v0))
        }
        Adt63::C4(_) => {
            Ok(f229(v0))
        }
    }
}

pub fn f161(v0: Adt85, v1: i64, v2: Str) -> R<Adt101> {
    Ok(f231(f159(v0, v1, v2.clone())?, v2))
}

pub fn f162(v0: Adt10, v1: List<Adt45>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m420 = match v2 {
        Adt21::C0(_, h419) => {
            h419
        }
    };
    match m420 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C4(Adt52::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f163() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C0)
}

pub fn f164() -> List<Adt45> {
    f232(f233())
}

pub fn f165(v0: Adt10, v1: Fn4, v2: Adt23) -> R<Result<Adt33, Adt90>> {
    Ok(f234(v0, match v2.clone() {
        Adt23::C0(_, h421, _, _) => {
            h421
        }
    }, {
        let c422: Fn4 = v1;
        c422.apply(match v2 {
            Adt23::C0(_, h423, _, _) => {
                h423
            }
        })?
    }))
}

pub fn f166() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C1)
}

pub fn f167(v0: Str) -> R<Option<List<Adt47>>> {
    let m424 = f235(v0.clone(), f233());
    match m424 {
        None => {
            f236(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f168(v0: Adt10, v1: List<Adt55>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m426 = match v2 {
        Adt21::C0(_, h425) => {
            h425
        }
    };
    match m426 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C6(Adt54::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f169() -> List<Adt55> {
    List::cons(Adt55::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt2>>, f237(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt41>, None::<Adt5>), List::<Adt55>::nil())
}

pub fn f170(v0: Adt10, v1: List<Adt57>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m428 = match v2 {
        Adt21::C0(_, h427) => {
            h427
        }
    };
    match m428 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C7(Adt56::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f171() -> List<Adt57> {
    List::cons(f238(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt58>>), List::cons(f238(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f239(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f239(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt58>::nil())))), List::cons(f238(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f239(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt58>::nil()))), List::cons(f238(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt58>>), List::<Adt57>::nil()))))
}

pub fn f172(v0: List<Adt57>, v1: Fn5, v2: Adt24) -> Result<Adt33, Adt90> {
    let m430 = f240(match v2.clone() {
        Adt24::C0(_, h429, _, _, _) => {
            h429
        }
    }, v0);
    match m430 {
        None => {
            Err::<Adt33, Adt90>(Adt90::C1(match v2 {
                Adt24::C0(_, h431, _, _, _) => {
                    h431
                }
            }))
        }
        Some(v3) => {
            let m433 = f241(f242(v3), f243(match v2.clone() {
                Adt24::C0(_, _, h432, _, _) => {
                    h432
                }
            }));
            match m433 {
                None => {
                    Ok::<Adt33, Adt90>(Adt33::C8({
                        let c436: Fn5 = v1;
                        c436.apply(match v2.clone() {
                            Adt24::C0(_, h437, _, _, _) => {
                                h437
                            }
                        }, f243(match v2 {
                            Adt24::C0(_, _, h438, _, _) => {
                                h438
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt33, Adt90>(Adt90::C4({
                        let a434 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a435 = Str::lit(" ");
                        join(a434, a435)
                    }))
                }
            }
        }
    }
}

pub fn f173(v0: Str, v1: List<(Str, Str)>) -> Adt59 {
    let a439 = v0.clone();
    let a440 = Str::lit("test_simple_prompt");
    if equal(a439, a440) {
        f244(List::cons(f245(Str::lit("This is a simple prompt for testing.")), List::<Adt60>::nil()))
    } else {
        let a441 = v0.clone();
        let a442 = Str::lit("test_prompt_with_arguments");
        if equal(a441, a442) {
            f244(List::cons(f245({
                let a443 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f246(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f246(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a444 = Str::lit("");
                join(a443, a444)
            }), List::<Adt60>::nil()))
        } else {
            let a445 = v0.clone();
            let a446 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a445, a446) {
                f244(List::cons(Adt60::C0(Adt42::C0, Adt39::C4(Adt46::C0(f247(f246(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt41>, None::<Adt5>))), List::cons(f245(Str::lit("Please process the embedded resource above.")), List::<Adt60>::nil())))
            } else {
                let a447 = v0;
                let a448 = Str::lit("test_prompt_with_image");
                if equal(a447, a448) {
                    f244(List::cons(Adt60::C0(Adt42::C0, Adt39::C1(Adt43::C0(f248(), Str::lit("image/png"), None::<Adt41>, None::<Adt5>))), List::cons(f245(Str::lit("Please analyze the image above.")), List::<Adt60>::nil())))
                } else {
                    f244(List::<Adt60>::nil())
                }
            }
        }
    }
}

pub fn f174(v0: Fn6, v1: Adt25) -> R<Result<Adt33, Adt90>> {
    let m450 = {
        let c449: Fn6 = v0;
        c449.apply(v1)?
    };
    match m450 {
        Ok(v3) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C9(f249(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f175(v0: Adt25) -> R<Result<List<Str>, Adt90>> {
    let m452 = match v0.clone() {
        Adt25::C0(_, h451, _, _) => {
            h451
        }
    };
    match m452 {
        Adt26::C0(v1) => {
            let m454 = f240(match v1.clone() {
                Adt27::C0(h453, _) => {
                    h453
                }
            }, f171());
            match m454 {
                None => {
                    Ok(Err::<List<Str>, Adt90>(Adt90::C1(match v1 {
                        Adt27::C0(h455, _) => {
                            h455
                        }
                    })))
                }
                Some(v2) => {
                    if f250(f242(v2), {
                        let m458 = match v0.clone() {
                            Adt25::C0(_, _, h457, _) => {
                                h457
                            }
                        };
                        match m458 {
                            Adt28::C0(h456, _) => {
                                h456
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt90>(f251({
                            let m461 = match v0.clone() {
                                Adt25::C0(_, _, h460, _) => {
                                    h460
                                }
                            };
                            match m461 {
                                Adt28::C0(_, h459) => {
                                    h459
                                }
                            }
                        }, f252(match v1 {
                            Adt27::C0(h462, _) => {
                                h462
                            }
                        }, {
                            let m465 = match v0 {
                                Adt25::C0(_, _, h464, _) => {
                                    h464
                                }
                            };
                            match m465 {
                                Adt28::C0(h463, _) => {
                                    h463
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
            let a466 = v3.clone();
            let a467 = f237();
            if equal(a466, a467) {
                let a471 = {
                    let m470 = match v0.clone() {
                        Adt25::C0(_, _, h469, _) => {
                            h469
                        }
                    };
                    match m470 {
                        Adt28::C0(h468, _) => {
                            h468
                        }
                    }
                };
                let a472 = Str::lit("id");
                if equal(a471, a472) {
                    Ok(Ok::<List<Str>, Adt90>(f251({
                        let m475 = match v0 {
                            Adt25::C0(_, _, h474, _) => {
                                h474
                            }
                        };
                        match m475 {
                            Adt28::C0(_, h473) => {
                                h473
                            }
                        }
                    }, f253())?))
                } else {
                    Ok(Err::<List<Str>, Adt90>(Adt90::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt90>(Adt90::C2(v3)))
            }
        }
    }
}

pub fn f176(p478: List<Adt92>, p479: List<Adt92>) -> List<Adt92> {
    let m477 = (p478, p479);
    {
        let mut m477 = m477;
        loop {
            let step = {
                let (v0, v1) = m477;
                let m476 = v0;
                match m476.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m477 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes) -> R<List<u64>> {
    f254(v0.clone(), {
        let a480 = v0;
        length_bytes(a480)
    }, List::<u64>::nil())
}

pub fn f178(v0: Bytes, v1: Adt91, v2: Result<(Adt14, u64), Adt93>) -> R<Result<Adt91, Adt93>> {
    let m481 = v2;
    match m481 {
        Ok(v4) => {
            f255(v0, Adt91::C0(match v1.clone() {
                Adt91::C0(h482, _, _, _, _, _, _) => {
                    h482
                }
            }, match v1.clone() {
                Adt91::C0(_, h483, _, _, _, _, _) => {
                    h483
                }
            }, match v1.clone() {
                Adt91::C0(_, _, h484, _, _, _, _) => {
                    h484
                }
            }, match v1.clone() {
                Adt91::C0(_, _, _, h485, _, _, _) => {
                    h485
                }
            }, match v1.clone() {
                Adt91::C0(_, _, _, _, h486, _, _) => {
                    h486
                }
            }, match v1 {
                Adt91::C0(_, _, _, _, _, h487, _) => {
                    h487
                }
            }, {
                let (h488, _) = v4;
                h488
            }), {
                let (_, h489) = v4;
                h489
            })
        }
        Err(v3) => {
            Ok(Err::<Adt91, Adt93>(v3))
        }
    }
}

pub fn f179() -> Bytes {
    Bytes::lit(&[])
}

pub fn f180(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt93>> {
    let a491 = v1;
    let a492 = {
        let a490 = v0.clone();
        length_bytes(a490)
    };
    if nat_eq(a491, a492) {
        Ok(Ok::<(Adt14, u64), Adt93>((Adt14::C0(0i64, Str::lit(""), Str::lit(""), f179()), v1)))
    } else {
        f256(v0, v1)
    }
}

pub fn f181(v0: Adt6) -> R<Result<List<u64>, Adt102>> {
    f257(f258(), v0)
}

pub fn f182() -> u64 {
    403u64
}

pub fn f183(v0: Option<Adt100>, v1: Adt81) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f259())), {
        let a493 = f260(v0);
        let a494 = List::cons((Str::lit("error"), f261(v1)), List::<(Str, Adt6)>::nil());
        append_list(a493, a494)
    })
}

pub fn f184() -> i64 {
    -32000i64
}

pub fn f185(v0: Adt96) -> Str {
    let m495 = v0;
    match m495 {
        Adt96::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt96::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f186(p502: List<u64>, p503: List<u64>) -> bool {
    let m501 = (p502, p503);
    {
        let mut m501 = m501;
        loop {
            let step = {
                let (v0, v1) = m501;
                let m496 = v0;
                match m496.uncons() {
                    None => {
                        let m497 = v1;
                        match m497.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m498 = v1;
                        match m498.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a499 = v4;
                                let a500 = v6;
                                if nat_eq(a499, a500) {
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
                Ok(next) => { m501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f188() -> u64 {
    400u64
}

pub fn f189(v0: Adt103) -> Adt81 {
    Adt81::C0(f262(v0.clone()), f263(v0), None::<Adt6>)
}

pub fn f190(v0: Adt14, v1: Adt0, v2: Fn3, v3: u64, v4: List<Adt92>, v5: Result<Adt108, Adt103>) -> R<Adt94> {
    let m504 = v5;
    match m504 {
        Ok(v7) => {
            f266(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f113(f264(v6.clone()), f115(f265(v6.clone()), f189(v6)))
        }
    }
}

pub fn f191(v0: Adt6) -> R<Result<Adt108, Adt103>> {
    let m505 = v0;
    match m505 {
        Adt6::C0 => {
            Ok(f268(None::<Adt100>, Adt104::C0))
        }
        Adt6::C1(_) => {
            Ok(f268(None::<Adt100>, Adt104::C0))
        }
        Adt6::C2(_) => {
            Ok(f268(None::<Adt100>, Adt104::C0))
        }
        Adt6::C3(_) => {
            Ok(f268(None::<Adt100>, Adt104::C0))
        }
        Adt6::C4(_) => {
            Ok(f268(None::<Adt100>, Adt104::C1))
        }
        Adt6::C5(v1) => {
            f267(Adt5::C0(v1))
        }
    }
}

pub fn f192(v0: List<u64>) -> R<Result<Adt6, Adt102>> {
    let m506 = f269(v0.clone())?;
    match m506 {
        Adt121::C0 => {
            f270(f271(), 0u64, v0)
        }
        Adt121::C1(v1) => {
            Ok(Err::<Adt6, Adt102>(Adt102::C0(v1)))
        }
        Adt121::C2(v2) => {
            Ok(Err::<Adt6, Adt102>(Adt102::C1(v2)))
        }
    }
}

pub fn f193() -> u64 {
    405u64
}

pub fn f194() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f195() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f196(p511: List<Adt92>, v1: List<u64>, p512: Adt106) -> R<Adt106> {
    let m510 = (p511, p512);
    {
        let mut m510 = m510;
        loop {
            let step = {
                let (v0, v2) = m510;
                let m507 = v0;
                match m507.uncons() {
                    None => {
                        Err::<(List<Adt92>, Adt106), R<Adt106>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f272(match v3.clone() {
                            Adt92::C0(h508, _) => {
                                h508
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt92>, Adt106), R<Adt106>>((v4, f273(v2, match v3 {
                                Adt92::C0(_, h509) => {
                                    h509
                                }
                            })))
                        } else {
                            Ok::<(List<Adt92>, Adt106), R<Adt106>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m510 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f274(v0);
    if f275(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f198(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f272(v0.clone(), f276())? {
        Ok(true)
    } else {
        if f272(v0.clone(), f277())? {
            Ok(true)
        } else {
            if f272(v0.clone(), f278())? {
                Ok(true)
            } else {
                f279(v0, v1)
            }
        }
    }
}

pub fn f199(p517: List<u64>) -> Option<List<u64>> {
    let m516 = p517;
    {
        let mut m516 = m516;
        loop {
            let step = {
                let v0 = m516;
                let m513 = v0;
                match m513.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a514 = v1;
                        let a515 = 58u64;
                        if nat_eq(a514, a515) {
                            Err::<List<u64>, Option<List<u64>>>(f280(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
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

pub fn f200(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m518 = v0;
    match m518 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f281({
                let (_, h519) = v1;
                h519
            }) {
                Some({
                    let (h520, _) = v1;
                    h520
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f201(p525: List<u64>, p526: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m524 = (p525, p526);
    {
        let mut m524 = m524;
        loop {
            let step = {
                let (v0, v1) = m524;
                let m521 = v0;
                match m521.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a522 = v2;
                        let a523 = 93u64;
                        if nat_eq(a522, a523) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f282(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(p531: List<u64>, p532: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m530 = (p531, p532);
    {
        let mut m530 = m530;
        loop {
            let step = {
                let (v0, v1) = m530;
                let m527 = v0;
                match m527.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f282(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a528 = v2;
                        let a529 = 58u64;
                        if nat_eq(a528, a529) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f282(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f203(v0: Str, p537: List<Str>) -> bool {
    let m536 = p537;
    {
        let mut m536 = m536;
        loop {
            let step = {
                let v1 = m536;
                let m533 = v1;
                match m533.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a534 = v0.clone();
                        let a535 = v2;
                        if equal(a534, a535) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
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

pub fn f204(v0: Str, v1: List<u64>) -> Option<Adt97> {
    if f283(v1) {
        Some(Adt97::C0(v0))
    } else {
        None::<Adt97>
    }
}

pub fn f205(v0: Str) -> R<List<u64>> {
    f177({
        let a538 = v0;
        utf8_encode(a538)
    })
}

pub fn f206(v0: Adt36) -> R<Result<List<Adt12>, Adt97>> {
    let m539 = f284(v0.clone());
    match m539 {
        None => {
            f285(v0.clone(), f286(f287(), match v0.clone() {
                Adt36::C0(h540, _, _, _, _, _, _, _) => {
                    h540
                }
            }, Adt6::C5({
                let m543 = match v0.clone() {
                    Adt36::C0(_, _, _, _, h542, _, _, _) => {
                        h542
                    }
                };
                match m543 {
                    Adt5::C0(h541) => {
                        h541
                    }
                }
            }), List::cons(Adt109::C0(Adt6::C5({
                let m546 = match v0 {
                    Adt36::C0(_, _, _, _, h545, _, _, _) => {
                        h545
                    }
                };
                match m546 {
                    Adt5::C0(h544) => {
                        h544
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt109>::nil()), f288())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f207(p551: u64, p552: i64) -> R<i64> {
    let m550 = (p551, p552);
    {
        let mut m550 = m550;
        loop {
            let step = {
                let (v0, v1) = m550;
                let m547 = v0;
                if m547 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m547 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a548 = v1;
                        let a549 = -1i64;
                        int_sub(a548, a549)?
                    }))
                }
            };
            match step {
                Ok(next) => { m550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Adt14, v1: Adt36, v2: Adt22, v3: Adt98) -> R<Result<Adt33, Adt90>> {
    let m553 = v3;
    match m553 {
        Adt98::C0(v4) => {
            f289(v1, v4)
        }
        Adt98::C1(v6, v7, v8) => {
            if f290(v2) {
                f291(v0, v8)
            } else {
                f292(v6, v1, v7)
            }
        }
        Adt98::C2(v5) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C12(v5)))
        }
    }
}

pub fn f209(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt110> {
    f293(v0, f294(), Adt112::C0, List::cons(Adt113::C0(v1, v2, List::<Adt111>::nil()), List::<Adt113>::nil()))
}

pub fn f210(v0: Adt22) -> Adt6 {
    let m555 = match v0 {
        Adt22::C0(_, _, h554, _, _) => {
            h554
        }
    };
    match m555 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h556) => {
                    h556
                }
            })
        }
    }
}

pub fn f211(v0: Str, v1: bool) -> Adt38 {
    Adt38::C0(List::cons(Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f212(v0: List<Adt111>, v1: Str) -> R<Str> {
    let v2: Str = f295(v0)?;
    let a559 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a557 = v2.clone();
        let a558 = Str::lit("");
        if equal(a557, a558) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a560 = Str::lit(" ");
    Ok(join(a559, a560))
}

pub fn f213(v0: Adt36, v1: Fn2, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m562 = {
        let c561: Fn2 = v1;
        c561.apply(v2)?
    };
    match m562 {
        Ok(v4) => {
            f289(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f214(v0: Str, v1: Adt5) -> Adt107 {
    f296(v0, match v1 {
        Adt5::C0(h563) => {
            h563
        }
    })
}

pub fn f215(v0: Adt6) -> Option<i64> {
    let m564 = v0;
    match m564 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f297(match v2.clone() {
                Adt7::C0(_, h565) => {
                    h565
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h566, _) => {
                        h566
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

pub fn f216(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m567 = v0;
    match m567 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m568 = f298(Adt6::C5(v1))?;
            match m568 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m569 = f219(v2);
                    match m569 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h570) => {
                                    h570
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f217(v0: Adt38) -> R<Option<List<(Str, Adt6)>>> {
    let m572 = f299(match v0.clone() {
        Adt38::C0(h571, _, _) => {
            h571
        }
    })?;
    match m572 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a575 = f300(Str::lit("structuredContent"), match v0.clone() {
                    Adt38::C0(_, h573, _) => {
                        h573
                    }
                });
                let a576 = f301(Str::lit("isError"), match v0 {
                    Adt38::C0(_, _, h574) => {
                        h574
                    }
                });
                append_list(a575, a576)
            })))
        }
    }
}

pub fn f218() -> Adt87 {
    Adt87::C1(Adt81::C0(-32603i64, Str::lit("Internal error"), None::<Adt6>))
}

pub fn f219(v0: Adt6) -> Option<Adt5> {
    let m577 = v0;
    match m577 {
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

pub fn f220(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f221(v0, 0i64)?, 0i64))
}

pub fn f221(p580: u64, p581: i64) -> R<i64> {
    let m579 = (p580, p581);
    {
        let mut m579 = m579;
        loop {
            let step = {
                let (v0, v1) = m579;
                let m578 = v0;
                if m578 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m578 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f302(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: Option<Adt5>, v1: Str) -> bool {
    let m582 = v0;
    match m582 {
        None => {
            false
        }
        Some(v2) => {
            f303(v1, v2)
        }
    }
}

pub fn f223(v0: Adt6) -> Option<List<Adt6>> {
    let m583 = v0;
    match m583 {
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

pub fn f224(p587: List<Adt6>, p588: List<Adt85>) -> Option<List<Adt85>> {
    let m586 = (p587, p588);
    {
        let mut m586 = m586;
        loop {
            let step = {
                let (v0, v1) = m586;
                let m584 = v0;
                match m584.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(Some(f304(v1, List::<Adt85>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m585 = f305(v2);
                        match m585 {
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
                Ok(next) => { m586 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: Adt85) -> Str {
    let m591 = f306(match v0 {
        Adt85::C0(h590, _, _) => {
            h590
        }
    });
    match m591 {
        Adt64::C0(h589, _, _, _, _, _) => {
            h589
        }
    }
}

pub fn f226(v0: Adt85, v1: i64) -> R<bool> {
    let m595 = {
        let m594 = f306(match v0.clone() {
            Adt85::C0(h593, _, _) => {
                h593
            }
        });
        match m594 {
            Adt64::C0(_, _, _, _, h592, _) => {
                h592
            }
        }
    };
    match m595 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a602 = {
                let m601 = {
                    let a599 = {
                        let a597 = v1;
                        let a598 = match v0 {
                            Adt85::C0(_, h596, _) => {
                                h596
                            }
                        };
                        int_sub(a597, a598)?
                    };
                    let a600 = v2;
                    compare(a599, a600)
                };
                match m601 {
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
            Ok(bool_not(a602))
        }
    }
}

pub fn f227() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f28(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f228(v0: Adt85, v1: Adt64, v2: i64, v3: Str) -> R<Adt101> {
    let m606 = {
        let m605 = match v0.clone() {
            Adt85::C0(_, _, h604) => {
                h604
            }
        };
        match m605 {
            Adt86::C0(_, h603, _, _) => {
                h603
            }
        }
    };
    match m606.uncons() {
        None => {
            if f307({
                let m609 = match v0.clone() {
                    Adt85::C0(_, _, h608) => {
                        h608
                    }
                };
                match m609 {
                    Adt86::C0(_, _, h607, _) => {
                        h607
                    }
                }
            }, match v0.clone() {
                Adt85::C0(_, h610, _) => {
                    h610
                }
            }, v2)? {
                Ok(f308(v0.clone(), f309({
                    let m613 = match v0.clone() {
                        Adt85::C0(_, _, h612) => {
                            h612
                        }
                    };
                    match m613 {
                        Adt86::C0(_, _, _, h611) => {
                            h611
                        }
                    }
                }, f310(v1, v3)), match v0 {
                    Adt85::C0(_, _, h614) => {
                        h614
                    }
                }))
            } else {
                Ok(f229(v0))
            }
        }
        Some((_, _)) => {
            if f307({
                let m617 = match v0.clone() {
                    Adt85::C0(_, _, h616) => {
                        h616
                    }
                };
                match m617 {
                    Adt86::C0(h615, _, _, _) => {
                        h615
                    }
                }
            }, match v0.clone() {
                Adt85::C0(_, h618, _) => {
                    h618
                }
            }, v2)? {
                Ok(f308(v0.clone(), Adt63::C1(f310(v1, v3), {
                    let m621 = match v0.clone() {
                        Adt85::C0(_, _, h620) => {
                            h620
                        }
                    };
                    match m621 {
                        Adt86::C0(_, h619, _, _) => {
                            h619
                        }
                    }
                }), f311(match v0 {
                    Adt85::C0(_, _, h622) => {
                        h622
                    }
                })))
            } else {
                Ok(f229(v0))
            }
        }
    }
}

pub fn f229(v0: Adt85) -> Adt101 {
    Adt101::C0(v0, false)
}

pub fn f230(v0: Adt85, v1: Adt64, v2: List<(Str, Adt65)>, v3: Adt5, v4: i64, v5: Str) -> R<Adt101> {
    let m623 = f312(v2.clone(), v3);
    match m623.uncons() {
        None => {
            f313(v0, v1, v4, v5)
        }
        Some((v6, v7)) => {
            let a626 = {
                let a624 = List::cons(v6.clone(), v7.clone());
                length_list(a624)
            };
            let a627 = {
                let a625 = v2;
                length_list(a625)
            };
            if nat_eq(a626, a627) {
                Ok(f229(v0))
            } else {
                Ok(f308(v0.clone(), Adt63::C1(f310(v1, v5), List::cons(v6, v7)), match v0 {
                    Adt85::C0(_, _, h628) => {
                        h628
                    }
                }))
            }
        }
    }
}

pub fn f231(v0: Adt101, v1: Str) -> Adt101 {
    let m632 = {
        let m631 = match v0.clone() {
            Adt101::C0(h630, _) => {
                h630
            }
        };
        match m631 {
            Adt85::C0(h629, _, _) => {
                h629
            }
        }
    };
    match m632 {
        Adt63::C0(v2) => {
            f308(match v0.clone() {
                Adt101::C0(h633, _) => {
                    h633
                }
            }, Adt63::C4(f310(v2, v1)), {
                let m636 = match v0 {
                    Adt101::C0(h635, _) => {
                        h635
                    }
                };
                match m636 {
                    Adt85::C0(_, _, h634) => {
                        h634
                    }
                }
            })
        }
        Adt63::C1(v3, _) => {
            f308(match v0.clone() {
                Adt101::C0(h637, _) => {
                    h637
                }
            }, Adt63::C4(f310(v3, v1)), {
                let m640 = match v0 {
                    Adt101::C0(h639, _) => {
                        h639
                    }
                };
                match m640 {
                    Adt85::C0(_, _, h638) => {
                        h638
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

pub fn f232(v0: List<(Adt45, List<Adt47>)>) -> List<Adt45> {
    f314(f315(v0, List::<Adt45>::nil()), List::<Adt45>::nil())
}

pub fn f233() -> List<(Adt45, List<Adt47>)> {
    List::cons(f316(f317(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f247(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f316(f317(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f318(Str::lit("test://static-binary"), Str::lit("image/png"), f248())), List::cons(f316(f317(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f247(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f316(f317(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f247(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt45, List<Adt47>)>::nil()))))
}

pub fn f234(v0: Adt10, v1: Str, v2: Option<List<Adt47>>) -> Result<Adt33, Adt90> {
    let m641 = v2;
    match m641 {
        None => {
            Err::<Adt33, Adt90>(Adt90::C2(v1))
        }
        Some(v3) => {
            let m642 = v3.clone();
            match m642.uncons() {
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

pub fn f235(v0: Str, p651: List<(Adt45, List<Adt47>)>) -> Option<List<Adt47>> {
    let m650 = p651;
    {
        let mut m650 = m650;
        loop {
            let step = {
                let v1 = m650;
                let m643 = v1;
                match m643.uncons() {
                    None => {
                        Err::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(None::<List<Adt47>>)
                    }
                    Some((v2, v3)) => {
                        let a647 = {
                            let m646 = {
                                let (h645, _) = v2;
                                h645
                            };
                            match m646 {
                                Adt45::C0(_, _, _, h644, _, _, _, _, _) => {
                                    h644
                                }
                            }
                        };
                        let a648 = v0.clone();
                        if equal(a647, a648) {
                            Err::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(Some({
                                let (_, h649) = v2;
                                h649
                            }))
                        } else {
                            Ok::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(v3)
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

pub fn f236(v0: Str) -> R<Option<List<Adt47>>> {
    let m652 = f319(v0.clone())?;
    match m652 {
        None => {
            Ok(None::<List<Adt47>>)
        }
        Some(v1) => {
            let m653 = f320(v1)?;
            match m653 {
                None => {
                    Ok(None::<List<Adt47>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f247(v0, Str::lit("application/json"), v2), List::<Adt47>::nil())))
                }
            }
        }
    }
}

pub fn f237() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f238(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt58>>) -> Adt57 {
    Adt57::C0(v0, Some(v1), None::<List<Adt2>>, Some(v2), v3, None::<Adt5>)
}

pub fn f239(v0: Str, v1: Str) -> Adt58 {
    Adt58::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f240(v0: Str, p659: List<Adt57>) -> Option<Adt57> {
    let m658 = p659;
    {
        let mut m658 = m658;
        loop {
            let step = {
                let v1 = m658;
                let m654 = v1;
                match m654.uncons() {
                    None => {
                        Err::<List<Adt57>, Option<Adt57>>(None::<Adt57>)
                    }
                    Some((v2, v3)) => {
                        let a656 = match v2.clone() {
                            Adt57::C0(h655, _, _, _, _, _) => {
                                h655
                            }
                        };
                        let a657 = v0.clone();
                        if equal(a656, a657) {
                            Err::<List<Adt57>, Option<Adt57>>(Some(v2))
                        } else {
                            Ok::<List<Adt57>, Option<Adt57>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(p665: List<Adt58>, v1: List<(Str, Str)>) -> Option<Str> {
    let m664 = p665;
    {
        let mut m664 = m664;
        loop {
            let step = {
                let v0 = m664;
                let m660 = v0;
                match m660.uncons() {
                    None => {
                        Err::<List<Adt58>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f321(v2.clone()) {
                            let m662 = f322(match v2.clone() {
                                Adt58::C0(h661, _, _, _) => {
                                    h661
                                }
                            }, v1.clone());
                            match m662 {
                                None => {
                                    Err::<List<Adt58>, Option<Str>>(Some(match v2 {
                                        Adt58::C0(h663, _, _, _) => {
                                            h663
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
                Ok(next) => { m664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt57) -> List<Adt58> {
    let m667 = match v0 {
        Adt57::C0(_, _, _, _, h666, _) => {
            h666
        }
    };
    match m667 {
        None => {
            List::<Adt58>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f243(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m668 = v0;
    match m668 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f244(v0: List<Adt60>) -> Adt59 {
    Adt59::C0(None::<Str>, v0)
}

pub fn f245(v0: Str) -> Adt60 {
    Adt60::C0(Adt42::C0, Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)))
}

pub fn f246(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m669 = f322(v0, v1);
    match m669 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f247(v0: Str, v1: Str, v2: Str) -> Adt47 {
    Adt47::C0(Adt48::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f248() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f249(v0: List<Str>) -> R<Adt61> {
    let v1: u64 = f323(v0.clone(), 0u64)?;
    Ok(Adt61::C0(Adt62::C0(f324(v0), Some(v1), Some({
        let a670 = f325();
        let a671 = v1;
        nat_lt(a670, a671)
    }))))
}

pub fn f250(p677: List<Adt58>, v1: Str) -> bool {
    let m676 = p677;
    {
        let mut m676 = m676;
        loop {
            let step = {
                let v0 = m676;
                let m672 = v0;
                match m672.uncons() {
                    None => {
                        Err::<List<Adt58>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a674 = match v2 {
                            Adt58::C0(h673, _, _, _) => {
                                h673
                            }
                        };
                        let a675 = v1.clone();
                        if equal(a674, a675) {
                            Err::<List<Adt58>, bool>(true)
                        } else {
                            Ok::<List<Adt58>, bool>(v3)
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

pub fn f251(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f326(f327(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f252(v0: Str, v1: Str) -> List<Str> {
    let a678 = v0;
    let a679 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a678, a679) {
        let a680 = v1;
        let a681 = Str::lit("resourceUri");
        equal(a680, a681)
    } else {
        false
    } {
        f328()
    } else {
        List::<Str>::nil()
    }
}

pub fn f253() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f254(v0: Bytes, p684: u64, p685: List<u64>) -> R<List<u64>> {
    let m683 = (p684, p685);
    {
        let mut m683 = m683;
        loop {
            let step = {
                let (v1, v2) = m683;
                let m682 = v1;
                if m682 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m682 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f108(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: Bytes, v1: Adt91, v2: u64) -> R<Result<Adt91, Adt93>> {
    let a687 = v2;
    let a688 = {
        let a686 = v0.clone();
        length_bytes(a686)
    };
    if nat_eq(a687, a688) {
        Ok(Ok::<Adt91, Adt93>(v1))
    } else {
        let m689 = f40(v0.clone(), v2)?;
        match m689 {
            None => {
                Ok(Err::<Adt91, Adt93>(Adt93::C0(v2)))
            }
            Some(v3) => {
                let m692 = f329(v0.clone(), {
                    let a690 = v2;
                    let a691 = 4u64;
                    nat_add(a690, a691)?
                }, v3, List::<List<u64>>::nil())?;
                match m692 {
                    Ok(v5) => {
                        let a695 = {
                            let (_, h693) = v5.clone();
                            h693
                        };
                        let a696 = {
                            let a694 = v0;
                            length_bytes(a694)
                        };
                        if nat_eq(a695, a696) {
                            Ok(Ok::<Adt91, Adt93>(f330(v1, {
                                let (h697, _) = v5;
                                h697
                            })))
                        } else {
                            Ok(Err::<Adt91, Adt93>(Adt93::C1({
                                let (_, h698) = v5;
                                h698
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

pub fn f256(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt93>> {
    let m699 = f331(v0.clone(), v1)?;
    match m699 {
        None => {
            Ok(Err::<(Adt14, u64), Adt93>(Adt93::C0(v1)))
        }
        Some(v2) => {
            let m702 = f332(v0.clone(), {
                let a700 = v1;
                let a701 = 8u64;
                nat_add(a700, a701)?
            })?;
            match m702 {
                Ok(v4) => {
                    f333(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt14, u64), Adt93>(v3))
                }
            }
        }
    }
}

pub fn f257(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt102>> {
    let m703 = f334(v0, List::cons(Adt115::C1(0u64, v1), List::<Adt115>::nil()), List::<u64>::nil())?;
    match m703 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt102>(f335(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt102>(v3))
        }
    }
}

pub fn f258() -> u64 {
    67108864u64
}

pub fn f259() -> Str {
    Str::lit("2.0")
}

pub fn f260(v0: Option<Adt100>) -> List<(Str, Adt6)> {
    let m704 = v0;
    match m704 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f336(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f261(v0: Adt81) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt81::C0(h705, _, _) => {
            h705
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt81::C0(_, h706, _) => {
            h706
        }
    })), f337(match v0 {
        Adt81::C0(_, _, h707) => {
            h707
        }
    }))))
}

pub fn f262(v0: Adt103) -> i64 {
    let m708 = v0;
    match m708 {
        Adt103::C0 => {
            f338()
        }
        Adt103::C1(_, _) => {
            f339()
        }
    }
}

pub fn f263(v0: Adt103) -> Str {
    let m709 = v0;
    match m709 {
        Adt103::C0 => {
            Str::lit("Parse error")
        }
        Adt103::C1(_, v2) => {
            f340(v2)
        }
    }
}

pub fn f264(v0: Adt103) -> u64 {
    let m710 = v0;
    match m710 {
        Adt103::C0 => {
            f188()
        }
        Adt103::C1(_, _) => {
            f188()
        }
    }
}

pub fn f265(v0: Adt103) -> Option<Adt100> {
    let m711 = v0;
    match m711 {
        Adt103::C0 => {
            None::<Adt100>
        }
        Adt103::C1(v1, _) => {
            v1
        }
    }
}

pub fn f266(v0: Adt14, v1: Adt0, v2: Fn3, v3: u64, v4: List<Adt92>, v5: Adt108) -> R<Adt94> {
    f341(v0, f342(v4.clone())?, f343(v1.clone(), v2, v3, Some(f344(v4, match v1 {
        Adt0::C0(_, _, _, _, h712, _) => {
            h712
        }
    }, f345(v5.clone()), match v5.clone() {
        Adt108::C0(_, h713, _) => {
            h713
        }
    })?), f346(), v5)?)
}

pub fn f267(v0: Adt5) -> R<Result<Adt108, Adt103>> {
    let m714 = f347(v0.clone())?;
    match m714 {
        Adt140::C0 => {
            let m715 = f214(Str::lit("method"), v0.clone());
            match m715 {
                Adt107::C0 => {
                    Ok(f268(None::<Adt100>, f348(v0)))
                }
                Adt107::C1(_) => {
                    Ok(f268(None::<Adt100>, if f349(v0) {
                        Adt104::C8
                    } else {
                        Adt104::C7
                    }))
                }
            }
        }
        Adt140::C1(v2) => {
            Ok(f350(v2, v0))
        }
        Adt140::C2 => {
            Ok(f268(None::<Adt100>, Adt104::C5))
        }
    }
}

pub fn f268(v0: Option<Adt100>, v1: Adt104) -> Result<Adt108, Adt103> {
    Err::<Adt108, Adt103>(Adt103::C1(v0, v1))
}

pub fn f269(v0: List<u64>) -> R<Adt121> {
    Ok(f351({
        let a716 = v0.clone();
        length_list(a716)
    }, f352(Adt122::C0, 0u64, 0u64, v0)?))
}

pub fn f270(p723: Adt123, p724: u64, p725: List<u64>) -> R<Result<Adt6, Adt102>> {
    let m722 = (p723, (p724, p725));
    {
        let mut m722 = m722;
        loop {
            let step = {
                let (v0, (v1, v2)) = m722;
                let m717 = v2;
                match m717.uncons() {
                    None => {
                        let m718 = f353(v0, v1)?;
                        match m718 {
                            Ok(v3) => {
                                Err::<(Adt123, (u64, List<u64>)), R<Result<Adt6, Adt102>>>(Ok(Ok::<Adt6, Adt102>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt123, (u64, List<u64>)), R<Result<Adt6, Adt102>>>(Ok(Err::<Adt6, Adt102>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m719 = f354(v0, v1, v5)?;
                        match m719 {
                            Adt141::C0(v7) => {
                                Err::<(Adt123, (u64, List<u64>)), R<Result<Adt6, Adt102>>>(Ok(Err::<Adt6, Adt102>(v7)))
                            }
                            Adt141::C1(v8) => {
                                Ok::<(Adt123, (u64, List<u64>)), R<Result<Adt6, Adt102>>>((v8, ({
                                    let a720 = v1;
                                    let a721 = 1u64;
                                    nat_add(a720, a721)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m722 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f271() -> Adt123 {
    Adt123::C0(Adt124::C0, List::<Adt128>::nil(), 0u64)
}

pub fn f272(p732: List<u64>, p733: List<u64>) -> R<bool> {
    let m731 = (p732, p733);
    {
        let mut m731 = m731;
        loop {
            let step = {
                let (v0, v1) = m731;
                let m726 = v0;
                match m726.uncons() {
                    None => {
                        let m727 = v1;
                        match m727.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m728 = v1;
                        match m728.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a729 = f355(v4)?;
                                let a730 = f355(v6)?;
                                if nat_eq(a729, a730) {
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
                Ok(next) => { m731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Adt106, v1: List<u64>) -> Adt106 {
    let m734 = v0;
    match m734 {
        Adt106::C0 => {
            Adt106::C1(v1)
        }
        Adt106::C1(_) => {
            Adt106::C2
        }
        Adt106::C2 => {
            Adt106::C2
        }
    }
}

pub fn f274(v0: List<u64>) -> List<u64> {
    f282(f356(f282(f356(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f275(p737: List<u64>) -> bool {
    let m736 = p737;
    {
        let mut m736 = m736;
        loop {
            let step = {
                let v0 = m736;
                let m735 = v0;
                match m735.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f357(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m736 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f277() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f278() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f279(v0: List<u64>, p740: List<List<u64>>) -> R<bool> {
    let m739 = p740;
    {
        let mut m739 = m739;
        loop {
            let step = {
                let v1 = m739;
                let m738 = v1;
                match m738.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f272(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
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

pub fn f280(v0: List<u64>) -> Option<List<u64>> {
    let m741 = v0;
    match m741.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m742 = v2;
            match m742.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a743 = v1;
                    let a744 = 47u64;
                    if if nat_eq(a743, a744) {
                        let a745 = v3;
                        let a746 = 47u64;
                        nat_eq(a745, a746)
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

pub fn f281(v0: List<u64>) -> bool {
    let m747 = v0;
    match m747.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a748 = v1;
            let a749 = 58u64;
            if nat_eq(a748, a749) {
                let m750 = v2.clone();
                match m750.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f358(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f282(p753: List<u64>, p754: List<u64>) -> List<u64> {
    let m752 = (p753, p754);
    {
        let mut m752 = m752;
        loop {
            let step = {
                let (v0, v1) = m752;
                let m751 = v0;
                match m751.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f283(v0: List<u64>) -> bool {
    let m755 = v0.clone();
    match m755.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a759 = if f359(v0.clone(), false) {
                let a757 = {
                    let a756 = v0;
                    length_list(a756)
                };
                let a758 = f360();
                nat_le(a757, a758)
            } else {
                false
            };
            bool_not(a759)
        }
    }
}

pub fn f284(v0: Adt36) -> Option<Adt97> {
    let m764 = f361(match v0.clone() {
        Adt36::C0(h760, _, _, _, _, _, _, _) => {
            h760
        }
    }, {
        let m763 = match v0.clone() {
            Adt36::C0(_, _, _, _, h762, _, _, _) => {
                h762
            }
        };
        match m763 {
            Adt5::C0(h761) => {
                h761
            }
        }
    });
    match m764 {
        None => {
            f362(match v0.clone() {
                Adt36::C0(h765, _, _, _, _, _, _, _) => {
                    h765
                }
            }, {
                let m768 = match v0 {
                    Adt36::C0(_, _, _, _, h767, _, _, _) => {
                        h767
                    }
                };
                match m768 {
                    Adt5::C0(h766) => {
                        h766
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f285(v0: Adt36, v1: Result<Adt129, Adt97>) -> R<Result<List<Adt12>, Adt97>> {
    let m769 = v1;
    match m769 {
        Ok(v3) => {
            let m770 = f363(v0)?;
            match m770 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt97>(f129(match v3 {
                        Adt129::C0(h771, _) => {
                            h771
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

pub fn f286(p778: u64, v1: Str, v2: Adt6, p779: List<Adt109>, p780: Adt129) -> R<Result<Adt129, Adt97>> {
    let m777 = (p778, (p779, p780));
    {
        let mut m777 = m777;
        loop {
            let step = {
                let (v0, (v3, v4)) = m777;
                let m772 = v0;
                if m772 == 0 {
                    Err::<(u64, (List<Adt109>, Adt129)), R<Result<Adt129, Adt97>>>(Ok(Err::<Adt129, Adt97>(Adt97::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m772 - 1;
                    let m773 = v3;
                    match m773.uncons() {
                        None => {
                            Err::<(u64, (List<Adt109>, Adt129)), R<Result<Adt129, Adt97>>>(Ok(Ok::<Adt129, Adt97>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m774 = f364(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m774 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt109>, Adt129)), R<Result<Adt129, Adt97>>>((v5, ({
                                        let (h775, _) = v9;
                                        h775
                                    }, {
                                        let (_, h776) = v9;
                                        h776
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt109>, Adt129)), R<Result<Adt129, Adt97>>>(Ok(Err::<Adt129, Adt97>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m777 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287() -> u64 {
    67108864u64
}

pub fn f288() -> Adt129 {
    Adt129::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f289(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f365({
        let m783 = match v1.clone() {
            Adt50::C0(h782, _) => {
                h782
            }
        };
        match m783 {
            Adt38::C0(h781, _, _) => {
                h781
            }
        }
    })? {
        f366(v0, v1)
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f290(v0: Adt22) -> bool {
    f153({
        let m786 = match v0 {
            Adt22::C0(h785, _, _, _, _) => {
                h785
            }
        };
        match m786 {
            Adt17::C0(_, h784, _, _, _, _) => {
                h784
            }
        }
    })
}

pub fn f291(v0: Adt14, v1: Adt86) -> R<Result<Adt33, Adt90>> {
    let a788 = match v0.clone() {
        Adt14::C0(_, _, h787, _) => {
            h787
        }
    };
    let a789 = Str::lit("");
    if equal(a788, a789) {
        Ok(Err::<Adt33, Adt90>(f367()))
    } else {
        let m791 = f154(match v0.clone() {
            Adt14::C0(_, _, _, h790) => {
                h790
            }
        })?;
        match m791 {
            None => {
                Ok(Err::<Adt33, Adt90>(f155()))
            }
            Some(_) => {
                Ok(Ok::<Adt33, Adt90>(Adt33::C13(Adt83::C0(Adt84::C0, f306({
                    let m793 = f368(v0.clone(), v1.clone());
                    match m793 {
                        Adt85::C0(h792, _, _) => {
                            h792
                        }
                    }
                }), f368(v0, v1)))))
            }
        }
    }
}

pub fn f292(v0: Adt99, v1: Adt36, v2: Adt50) -> R<Result<Adt33, Adt90>> {
    let m794 = v0;
    match m794 {
        Adt99::C0 => {
            f289(v1, v2)
        }
        Adt99::C1 => {
            Ok(Err::<Adt33, Adt90>(f157()))
        }
    }
}

pub fn f293(v0: Adt6, p813: u64, p814: Adt112, p815: List<Adt113>) -> R<Adt110> {
    let m812 = (p813, (p814, p815));
    {
        let mut m812 = m812;
        loop {
            let step = {
                let (v1, (v2, v3)) = m812;
                let m795 = v1;
                if m795 == 0 {
                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C4))
                } else {
                    let v4 = m795 - 1;
                    let m796 = v3;
                    match m796.uncons() {
                        None => {
                            let m797 = v2;
                            match m797 {
                                Adt112::C0 => {
                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C0))
                                }
                                Adt112::C1(v5, v6) => {
                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m798 = v2.clone();
                            match m798 {
                                Adt112::C0 => {
                                    let m799 = v7.clone();
                                    match m799 {
                                        Adt113::C0(v9, v10, v11) => {
                                            let m800 = f369(v0.clone(), v9, v10, v11.clone())?;
                                            match m800 {
                                                Adt142::C0(v12) => {
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (Adt112::C1(v11, v12), v8)))
                                                }
                                                Adt142::C1(v13) => {
                                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C2(v13)))
                                                }
                                                Adt142::C2(v14) => {
                                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C3(v14)))
                                                }
                                                Adt142::C3(v15) => {
                                                    let v16: List<Adt113> = {
                                                        let a801 = v15;
                                                        let a802 = v8;
                                                        append_list(a801, a802)
                                                    };
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt113::C1(_, _, _) => {
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v8)))
                                        }
                                        Adt113::C2(_, _, _, _) => {
                                            let v24: Adt130 = f370(v7, v8)?;
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v24.clone() {
                                                Adt130::C0(h803, _) => {
                                                    h803
                                                }
                                            }, match v24 {
                                                Adt130::C0(_, h804) => {
                                                    h804
                                                }
                                            })))
                                        }
                                        Adt113::C3(v25) => {
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (Adt112::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt113::C4(_, _, _, _) => {
                                            let v39: Adt130 = f370(v7, v8)?;
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v39.clone() {
                                                Adt130::C0(h808, _) => {
                                                    h808
                                                }
                                            }, match v39 {
                                                Adt130::C0(_, h809) => {
                                                    h809
                                                }
                                            })))
                                        }
                                        Adt113::C5(v26, v27, v28, v29) => {
                                            let m805 = f371(v26, v27, v28, v29.clone());
                                            match m805 {
                                                Adt142::C0(v30) => {
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (Adt112::C1(v29, v30), v8)))
                                                }
                                                Adt142::C1(v31) => {
                                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C2(v31)))
                                                }
                                                Adt142::C2(v32) => {
                                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C3(v32)))
                                                }
                                                Adt142::C3(v33) => {
                                                    let v34: List<Adt113> = {
                                                        let a806 = v33;
                                                        let a807 = v8;
                                                        append_list(a806, a807)
                                                    };
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt112::C1(v40, v41) => {
                                    let v42: Adt130 = f372(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v42.clone() {
                                        Adt130::C0(h810, _) => {
                                            h810
                                        }
                                    }, match v42 {
                                        Adt130::C0(_, h811) => {
                                            h811
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294() -> u64 {
    67108864u64
}

pub fn f295(v0: List<Adt111>) -> R<Str> {
    let a816 = List::cons(Str::lit(""), f373(v0, List::<Str>::nil())?);
    let a817 = Str::lit("/");
    Ok(join(a816, a817))
}

pub fn f296(v0: Str, p823: List<(Str, Adt6)>) -> Adt107 {
    let m822 = p823;
    {
        let mut m822 = m822;
        loop {
            let step = {
                let v1 = m822;
                let m818 = v1;
                match m818.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt107>(Adt107::C0)
                    }
                    Some((v2, v3)) => {
                        let m820 = f374({
                            let (h819, _) = v2;
                            h819
                        }, v0.clone());
                        match m820 {
                            Adt143::C0 => {
                                Err::<List<(Str, Adt6)>, Adt107>(Adt107::C1({
                                    let (_, h821) = v2;
                                    h821
                                }))
                            }
                            Adt143::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt107>(v3)
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

pub fn f297(v0: i64) -> bool {
    let a830 = {
        let m826 = {
            let a824 = v0;
            let a825 = 0i64;
            compare(a824, a825)
        };
        if match m826 {
            Ordering::Less => {
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
            let m829 = {
                let a827 = 0i64;
                let a828 = v0;
                compare(a827, a828)
            };
            match m829 {
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
    bool_not(a830)
}

pub fn f298(v0: Adt6) -> R<Option<Adt6>> {
    f375(f376(), Adt131::C0(v0), List::<Adt132>::nil())
}

pub fn f299(v0: List<Adt39>) -> R<Option<List<Adt6>>> {
    let m831 = f377(v0, List::<Adt6>::nil())?;
    match m831 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f378(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f300(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m832 = v1;
    match m832 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f301(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m833 = v1;
    match m833 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f302(v0: i64, v1: i64) -> R<i64> {
    let a835 = v0;
    let a836 = {
        let a834 = v1;
        int_neg(a834)?
    };
    int_sub(a835, a836)
}

pub fn f303(v0: Str, v1: Adt5) -> bool {
    f379(f214(v0, v1))
}

pub fn f304(p839: List<Adt85>, p840: List<Adt85>) -> List<Adt85> {
    let m838 = (p839, p840);
    {
        let mut m838 = m838;
        loop {
            let step = {
                let (v0, v1) = m838;
                let m837 = v0;
                match m837.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), List<Adt85>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt85>, List<Adt85>), List<Adt85>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m838 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: Adt6) -> Option<Adt85> {
    let m841 = f219(v0);
    match m841 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m842 = f380(Str::lit("task"), v1.clone());
            match m842 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m843 = f381(v2);
                    match m843 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m844 = f382(Str::lit("createdMs"), v1.clone());
                            match m844 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m845 = f380(Str::lit("plan"), v1);
                                    match m845 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            let m846 = f383(v5);
                                            match m846 {
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

pub fn f306(v0: Adt63) -> Adt64 {
    let m847 = v0;
    match m847 {
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

pub fn f307(v0: i64, v1: i64, v2: i64) -> R<bool> {
    let a853 = {
        let m852 = {
            let a850 = {
                let a848 = v2;
                let a849 = v1;
                int_sub(a848, a849)?
            };
            let a851 = v0;
            compare(a850, a851)
        };
        match m852 {
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
    Ok(bool_not(a853))
}

pub fn f308(v0: Adt85, v1: Adt63, v2: Adt86) -> Adt101 {
    Adt101::C0(Adt85::C0(v1, match v0 {
        Adt85::C0(_, h854, _) => {
            h854
        }
    }, v2), true)
}

pub fn f309(v0: Adt87, v1: Adt64) -> Adt63 {
    let m855 = v0;
    match m855 {
        Adt87::C0(v2) => {
            Adt63::C2(v1, v2)
        }
        Adt87::C1(v3) => {
            Adt63::C3(v1, v3)
        }
    }
}

pub fn f310(v0: Adt64, v1: Str) -> Adt64 {
    Adt64::C0(match v0.clone() {
        Adt64::C0(h856, _, _, _, _, _) => {
            h856
        }
    }, match v0.clone() {
        Adt64::C0(_, h857, _, _, _, _) => {
            h857
        }
    }, match v0.clone() {
        Adt64::C0(_, _, h858, _, _, _) => {
            h858
        }
    }, v1, match v0.clone() {
        Adt64::C0(_, _, _, _, h859, _) => {
            h859
        }
    }, match v0 {
        Adt64::C0(_, _, _, _, _, h860) => {
            h860
        }
    })
}

pub fn f311(v0: Adt86) -> Adt86 {
    Adt86::C0(match v0.clone() {
        Adt86::C0(h861, _, _, _) => {
            h861
        }
    }, List::<(Str, Adt65)>::nil(), match v0.clone() {
        Adt86::C0(_, _, h862, _) => {
            h862
        }
    }, match v0 {
        Adt86::C0(_, _, _, h863) => {
            h863
        }
    })
}

pub fn f312(v0: List<(Str, Adt65)>, v1: Adt5) -> List<(Str, Adt65)> {
    f384(f385(v0, v1, List::<(Str, Adt65)>::nil()), List::<(Str, Adt65)>::nil())
}

pub fn f313(v0: Adt85, v1: Adt64, v2: i64, v3: Str) -> R<Adt101> {
    let v4: Adt85 = Adt85::C0(Adt63::C0(f310(v1, v3.clone())), match v0.clone() {
        Adt85::C0(_, h864, _) => {
            h864
        }
    }, match v0 {
        Adt85::C0(_, _, h865) => {
            h865
        }
    });
    Ok(Adt101::C0({
        let m867 = f159(v4, v2, v3)?;
        match m867 {
            Adt101::C0(h866, _) => {
                h866
            }
        }
    }, true))
}

pub fn f314(p870: List<Adt45>, p871: List<Adt45>) -> List<Adt45> {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v0, v1) = m869;
                let m868 = v0;
                match m868.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt45>), List<Adt45>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt45>), List<Adt45>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(p875: List<(Adt45, List<Adt47>)>, p876: List<Adt45>) -> List<Adt45> {
    let m874 = (p875, p876);
    {
        let mut m874 = m874;
        loop {
            let step = {
                let (v0, v1) = m874;
                let m872 = v0;
                match m872.uncons() {
                    None => {
                        Err::<(List<(Adt45, List<Adt47>)>, List<Adt45>), List<Adt45>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt45, List<Adt47>)>, List<Adt45>), List<Adt45>>((v3, List::cons({
                            let (h873, _) = v2;
                            h873
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m874 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: Adt45, v1: Adt47) -> (Adt45, List<Adt47>) {
    (v0, List::cons(v1, List::<Adt47>::nil()))
}

pub fn f317(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt45 {
    Adt45::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt41>, None::<u64>, None::<Adt5>)
}

pub fn f318(v0: Str, v1: Str, v2: Str) -> Adt47 {
    Adt47::C1(Adt49::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f319(v0: Str) -> R<Option<Str>> {
    let m877 = f386(v0)?;
    match m877 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a878 = f387(f179(), v1);
            Ok(utf8_decode(a878))
        }
    }
}

pub fn f320(v0: Str) -> R<Option<Str>> {
    let m881 = f388(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a879 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a880 = Str::lit(" ");
        join(a879, a880)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m881 {
        Ok(v1) => {
            let a882 = v1;
            Ok(utf8_decode(a882))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f321(v0: Adt58) -> bool {
    let m884 = match v0 {
        Adt58::C0(_, _, _, h883) => {
            h883
        }
    };
    match m884 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f322(v0: Str, p891: List<(Str, Str)>) -> Option<Str> {
    let m890 = p891;
    {
        let mut m890 = m890;
        loop {
            let step = {
                let v1 = m890;
                let m885 = v1;
                match m885.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a887 = {
                            let (h886, _) = v2;
                            h886
                        };
                        let a888 = v0.clone();
                        if equal(a887, a888) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h889) = v2;
                                h889
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f323(p896: List<Str>, p897: u64) -> R<u64> {
    let m895 = (p896, p897);
    {
        let mut m895 = m895;
        loop {
            let step = {
                let (v0, v1) = m895;
                let m892 = v0;
                match m892.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a893 = v1;
                            let a894 = 1u64;
                            nat_add(a893, a894)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m895 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: List<Str>) -> List<Str> {
    f326(f389(v0, f325(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f325() -> u64 {
    100u64
}

pub fn f326(p900: List<Str>, p901: List<Str>) -> List<Str> {
    let m899 = (p900, p901);
    {
        let mut m899 = m899;
        loop {
            let step = {
                let (v0, v1) = m899;
                let m898 = v0;
                match m898.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m899 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: Str, p904: List<Str>, p905: List<Str>) -> R<List<Str>> {
    let m903 = (p904, p905);
    {
        let mut m903 = m903;
        loop {
            let step = {
                let (v1, v2) = m903;
                let m902 = v1;
                match m902.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f390(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328() -> List<Str> {
    f326(f391(f164(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f329(v0: Bytes, p911: u64, p912: u64, p913: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt93>> {
    let m910 = (p911, (p912, p913));
    {
        let mut m910 = m910;
        loop {
            let step = {
                let (v1, (v2, v3)) = m910;
                let m906 = v2;
                if m906 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>(Ok(Ok::<(List<List<u64>>, u64), Adt93>((f45(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m906 - 1;
                    let m907 = f16(v0.clone(), v1)?;
                    match m907 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>(Ok(Err::<(List<List<u64>>, u64), Adt93>(Adt93::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>((match v5.clone() {
                                Adt95::C0(_, h908) => {
                                    h908
                                }
                            }, (v4, List::cons(f177(match v5 {
                                Adt95::C0(h909, _) => {
                                    h909
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m910 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(v0: Adt91, v1: List<List<u64>>) -> Adt91 {
    Adt91::C0(match v0.clone() {
        Adt91::C0(h914, _, _, _, _, _, _) => {
            h914
        }
    }, match v0.clone() {
        Adt91::C0(_, h915, _, _, _, _, _) => {
            h915
        }
    }, match v0.clone() {
        Adt91::C0(_, _, h916, _, _, _, _) => {
            h916
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, h917, _, _, _) => {
            h917
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, _, h918, _, _) => {
            h918
        }
    }, v1, match v0 {
        Adt91::C0(_, _, _, _, _, _, h919) => {
            h919
        }
    })
}

pub fn f331(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a923 = {
        let a920 = v1;
        let a921 = 8u64;
        nat_add(a920, a921)?
    };
    let a924 = {
        let a922 = v0.clone();
        length_bytes(a922)
    };
    if nat_le(a923, a924) {
        Ok(Some(f392(f392(f392(f392(f392(f392(f392(f392(0i64, f108(v0.clone(), v1)?)?, f108(v0.clone(), {
            let a925 = v1;
            let a926 = 1u64;
            nat_add(a925, a926)?
        })?)?, f108(v0.clone(), {
            let a927 = v1;
            let a928 = 2u64;
            nat_add(a927, a928)?
        })?)?, f108(v0.clone(), {
            let a929 = v1;
            let a930 = 3u64;
            nat_add(a929, a930)?
        })?)?, f108(v0.clone(), {
            let a931 = v1;
            let a932 = 4u64;
            nat_add(a931, a932)?
        })?)?, f108(v0.clone(), {
            let a933 = v1;
            let a934 = 5u64;
            nat_add(a933, a934)?
        })?)?, f108(v0.clone(), {
            let a935 = v1;
            let a936 = 6u64;
            nat_add(a935, a936)?
        })?)?, f108(v0, {
            let a937 = v1;
            let a938 = 7u64;
            nat_add(a937, a938)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f332(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt93>> {
    let m939 = f16(v0, v1)?;
    match m939 {
        None => {
            Ok(Err::<(Str, u64), Adt93>(Adt93::C0(v1)))
        }
        Some(v2) => {
            let m941 = f393(f177(match v2.clone() {
                Adt95::C0(h940, _) => {
                    h940
                }
            })?);
            match m941 {
                None => {
                    Ok(Err::<(Str, u64), Adt93>(Adt93::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt93>((v3, match v2 {
                        Adt95::C0(_, h942) => {
                            h942
                        }
                    })))
                }
            }
        }
    }
}

pub fn f333(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt14, u64), Adt93>> {
    let m944 = f332(v0.clone(), {
        let (_, h943) = v2;
        h943
    })?;
    match m944 {
        Ok(v4) => {
            f394(v0, v1, {
                let (h945, _) = v2;
                h945
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt14, u64), Adt93>(v3))
        }
    }
}

pub fn f334(p950: u64, p951: List<Adt115>, p952: List<u64>) -> R<Result<List<u64>, Adt102>> {
    let m949 = (p950, (p951, p952));
    {
        let mut m949 = m949;
        loop {
            let step = {
                let (v0, (v1, v2)) = m949;
                let m946 = v0;
                if m946 == 0 {
                    Err::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt102>>>(Ok(Err::<List<u64>, Adt102>(Adt102::C13)))
                } else {
                    let v3 = m946 - 1;
                    let m947 = v1;
                    match m947.uncons() {
                        None => {
                            Err::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt102>>>(Ok(Ok::<List<u64>, Adt102>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m948 = f395(v4, v5, v2)?;
                            match m948 {
                                Adt144::C0(v6) => {
                                    Err::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt102>>>(Ok(Err::<List<u64>, Adt102>(v6)))
                                }
                                Adt144::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt102>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m949 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(p955: List<u64>, p956: List<u64>) -> List<u64> {
    let m954 = (p955, p956);
    {
        let mut m954 = m954;
        loop {
            let step = {
                let (v0, v1) = m954;
                let m953 = v0;
                match m953.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f336(v0: Adt100) -> Adt6 {
    let m957 = v0;
    match m957 {
        Adt100::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt100::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f337(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m958 = v0;
    match m958 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f338() -> i64 {
    -32700i64
}

pub fn f339() -> i64 {
    -32600i64
}

pub fn f340(v0: Adt104) -> Str {
    let m959 = v0;
    match m959 {
        Adt104::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt104::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt104::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt104::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt104::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt104::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt104::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt104::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt104::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f341(v0: Adt14, v1: bool, v2: Adt133) -> R<Adt94> {
    let m961 = match v2.clone() {
        Adt133::C0(_, h960) => {
            h960
        }
    };
    match m961 {
        Ok(v3) => {
            let m962 = f396(v3.clone())?;
            match m962 {
                None => {
                    Ok(f6())
                }
                Some(v4) => {
                    f397(v0, f398(match v3.clone() {
                        Adt134::C0(h963, _, _) => {
                            h963
                        }
                    }), f399(v1, f113(f400(), f401(match v2 {
                        Adt133::C0(h964, _) => {
                            h964
                        }
                    }, v4))?, f402(f403(match v3.clone() {
                        Adt134::C0(_, _, h965) => {
                            h965
                        }
                    }, List::<List<u64>>::nil())?, f404(f398(match v3 {
                        Adt134::C0(h966, _, _) => {
                            h966
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f113(f405(v5.clone()), f115(Some(match v2 {
                Adt133::C0(h967, _) => {
                    h967
                }
            }), f406(v5)?))
        }
    }
}

pub fn f342(p972: List<Adt92>) -> R<bool> {
    let m971 = p972;
    {
        let mut m971 = m971;
        loop {
            let step = {
                let v0 = m971;
                let m968 = v0;
                match m968.uncons() {
                    None => {
                        Err::<List<Adt92>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f272(match v1.clone() {
                            Adt92::C0(h969, _) => {
                                h969
                            }
                        }, f407())? {
                            if f408(f409(44u64, match v1 {
                                Adt92::C0(_, h970) => {
                                    h970
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
                Ok(next) => { m971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: Adt0, v1: Fn3, v2: u64, v3: Option<Adt116>, v4: List<Str>, v5: Adt108) -> R<Adt133> {
    Ok(Adt133::C0(match v5.clone() {
        Adt108::C0(h973, _, _) => {
            h973
        }
    }, f410(v0, v1, v2, f411(v3, v4, v5)?)?))
}

pub fn f344(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt116> {
    let v4: Adt139 = f412(Adt118::C0, f122(v0.clone(), f413())?);
    let v5: Adt139 = f412(Adt118::C1, f122(v0.clone(), f414())?);
    let v6: Adt139 = if f415(v3) {
        f412(Adt118::C2, f416(f122(v0.clone(), f417())?)?)
    } else {
        f418()
    };
    Ok(Adt116::C0(match v4.clone() {
        Adt139::C0(h974, _) => {
            h974
        }
    }, match v5.clone() {
        Adt139::C0(h975, _) => {
            h975
        }
    }, match v6.clone() {
        Adt139::C0(h976, _) => {
            h976
        }
    }, f419(match v4 {
        Adt139::C0(_, h977) => {
            h977
        }
    }, f419(match v5 {
        Adt139::C0(_, h978) => {
            h978
        }
    }, match v6 {
        Adt139::C0(_, h979) => {
            h979
        }
    })), f420(v0, v1, v2)?))
}

pub fn f345(v0: Adt108) -> Option<Str> {
    let a981 = match v0.clone() {
        Adt108::C0(_, h980, _) => {
            h980
        }
    };
    let a982 = Str::lit("tools/call");
    if equal(a981, a982) {
        let m984 = match v0 {
            Adt108::C0(_, _, h983) => {
                h983
            }
        };
        match m984 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m985 = f214(Str::lit("name"), v1);
                match m985 {
                    Adt107::C0 => {
                        None::<Str>
                    }
                    Adt107::C1(v2) => {
                        f141(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f346() -> List<Str> {
    List::cons(f421(), List::<Str>::nil())
}

pub fn f347(v0: Adt5) -> R<Adt140> {
    let m986 = f214(Str::lit("id"), v0);
    match m986 {
        Adt107::C0 => {
            Ok(Adt140::C0)
        }
        Adt107::C1(v1) => {
            let m987 = v1;
            match m987 {
                Adt6::C0 => {
                    Ok(Adt140::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt140::C2)
                }
                Adt6::C2(v3) => {
                    let m988 = f422(v3)?;
                    match m988 {
                        None => {
                            Ok(Adt140::C2)
                        }
                        Some(v4) => {
                            Ok(Adt140::C1(Adt100::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt140::C1(Adt100::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt140::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt140::C2)
                }
            }
        }
    }
}

pub fn f348(v0: Adt5) -> Adt104 {
    if f349(v0) {
        Adt104::C8
    } else {
        Adt104::C3
    }
}

pub fn f349(v0: Adt5) -> bool {
    if f303(Str::lit("result"), v0.clone()) {
        true
    } else {
        f303(Str::lit("error"), v0)
    }
}

pub fn f350(v0: Adt100, v1: Adt5) -> Result<Adt108, Adt103> {
    let m989 = f214(Str::lit("jsonrpc"), v1.clone());
    match m989 {
        Adt107::C0 => {
            f268(Some(v0), Adt104::C2)
        }
        Adt107::C1(v2) => {
            let m990 = f141(v2);
            match m990 {
                None => {
                    f268(Some(v0), Adt104::C2)
                }
                Some(v3) => {
                    let a991 = v3;
                    let a992 = f259();
                    if equal(a991, a992) {
                        f423(v0, v1)
                    } else {
                        f268(Some(v0), Adt104::C2)
                    }
                }
            }
        }
    }
}

pub fn f351(v0: u64, v1: u64) -> Adt121 {
    let a993 = v1;
    let a994 = 0u64;
    if nat_eq(a993, a994) {
        Adt121::C0
    } else {
        let a995 = v1;
        let a996 = 2u64;
        if nat_eq(a995, a996) {
            Adt121::C2(v0)
        } else {
            Adt121::C1({
                let a1000 = v0;
                let a1001 = {
                    let a997 = v1;
                    let a998 = 3u64;
                    let a999 = 0u64;
                    nat_quot(a997, a998, a999)
                };
                nat_sub(a1000, a1001)
            })
        }
    }
}

pub fn f352(p1029: Adt122, p1030: u64, p1031: u64, p1032: List<u64>) -> R<u64> {
    let m1028 = (p1029, (p1030, (p1031, p1032)));
    {
        let mut m1028 = m1028;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1028;
                let m1002 = v3;
                match m1002.uncons() {
                    None => {
                        let m1003 = v0;
                        match m1003 {
                            Adt122::C0 => {
                                Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt122::C1 => {
                                Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt122::C2 => {
                                Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt122::C3 => {
                                Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1004 = v0;
                        match m1004 {
                            Adt122::C0 => {
                                let a1005 = v4;
                                let a1006 = 128u64;
                                if nat_lt(a1005, a1006) {
                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1007 = v4;
                                    let a1008 = 194u64;
                                    if nat_lt(a1007, a1008) {
                                        let v6: u64 = f424({
                                            let a1009 = List::cons(v4, v5);
                                            length_list(a1009)
                                        })?;
                                        Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1010 = v4;
                                        let a1011 = 224u64;
                                        if nat_lt(a1010, a1011) {
                                            Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1012 = v4;
                                            let a1013 = 224u64;
                                            if nat_eq(a1012, a1013) {
                                                Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1014 = v4;
                                                let a1015 = 237u64;
                                                if nat_eq(a1014, a1015) {
                                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1016 = v4;
                                                    let a1017 = 240u64;
                                                    if nat_lt(a1016, a1017) {
                                                        Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1018 = v4;
                                                        let a1019 = 240u64;
                                                        if nat_eq(a1018, a1019) {
                                                            Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1020 = v4;
                                                            let a1021 = 244u64;
                                                            if nat_eq(a1020, a1021) {
                                                                Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1022 = v4;
                                                                let a1023 = 244u64;
                                                                if nat_lt(a1022, a1023) {
                                                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f424({
                                                                        let a1024 = List::cons(v4, v5);
                                                                        length_list(a1024)
                                                                    })?;
                                                                    Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt122::C1 => {
                                if f425(v4, v1, v2) {
                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f424({
                                        let a1025 = List::cons(v4, v5);
                                        length_list(a1025)
                                    })?;
                                    Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt122::C2 => {
                                if f425(v4, v1, v2) {
                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f424({
                                        let a1026 = List::cons(v4, v5);
                                        length_list(a1026)
                                    })?;
                                    Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt122::C3 => {
                                if f425(v4, v1, v2) {
                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f424({
                                        let a1027 = List::cons(v4, v5);
                                        length_list(a1027)
                                    })?;
                                    Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: Adt123, v1: u64) -> R<Result<Adt6, Adt102>> {
    let m1034 = match v0.clone() {
        Adt123::C0(h1033, _, _) => {
            h1033
        }
    };
    match m1034 {
        Adt124::C0 => {
            Ok(Err::<Adt6, Adt102>(Adt102::C3))
        }
        Adt124::C1 => {
            Ok(Err::<Adt6, Adt102>(Adt102::C3))
        }
        Adt124::C2 => {
            Ok(Err::<Adt6, Adt102>(Adt102::C3))
        }
        Adt124::C3 => {
            Ok(Err::<Adt6, Adt102>(Adt102::C3))
        }
        Adt124::C4 => {
            Ok(Err::<Adt6, Adt102>(Adt102::C3))
        }
        Adt124::C5 => {
            Ok(Err::<Adt6, Adt102>(Adt102::C3))
        }
        Adt124::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt102>(Adt102::C3))
        }
        Adt124::C7(_, _) => {
            Ok(Err::<Adt6, Adt102>(Adt102::C3))
        }
        Adt124::C8(v3) => {
            let m1035 = f426(v3)?;
            match m1035 {
                Adt147::C0 => {
                    Ok(Err::<Adt6, Adt102>(Adt102::C7(v1)))
                }
                Adt147::C1 => {
                    Ok(Err::<Adt6, Adt102>(Adt102::C8(v1)))
                }
                Adt147::C2(v4) => {
                    let m1040 = {
                        let m1039 = f427(v4, match v0.clone() {
                            Adt123::C0(_, h1037, _) => {
                                h1037
                            }
                        }, match v0 {
                            Adt123::C0(_, _, h1038) => {
                                h1038
                            }
                        });
                        match m1039 {
                            Adt123::C0(h1036, _, _) => {
                                h1036
                            }
                        }
                    };
                    match m1040 {
                        Adt124::C0 => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C1 => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C2 => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C3 => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C4 => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C5 => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C7(_, _) => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C8(_) => {
                            Ok(Err::<Adt6, Adt102>(Adt102::C3))
                        }
                        Adt124::C9(v5) => {
                            Ok(Ok::<Adt6, Adt102>(v5))
                        }
                    }
                }
            }
        }
        Adt124::C9(v2) => {
            Ok(Ok::<Adt6, Adt102>(v2))
        }
    }
}

pub fn f354(v0: Adt123, v1: u64, v2: u64) -> R<Adt141> {
    let m1042 = match v0.clone() {
        Adt123::C0(h1041, _, _) => {
            h1041
        }
    };
    match m1042 {
        Adt124::C0 => {
            if f428(v2) {
                Ok(Adt141::C1(v0))
            } else {
                f429(v0, v1, v2)
            }
        }
        Adt124::C1 => {
            if f428(v2) {
                Ok(Adt141::C1(v0))
            } else {
                let a1043 = v2;
                let a1044 = 93u64;
                if nat_eq(a1043, a1044) {
                    Ok(f430(v0, v1))
                } else {
                    f429(v0, v1, v2)
                }
            }
        }
        Adt124::C2 => {
            Ok(f431(v0, v1, v2))
        }
        Adt124::C3 => {
            if f428(v2) {
                Ok(Adt141::C1(v0))
            } else {
                let a1045 = v2;
                let a1046 = 125u64;
                if nat_eq(a1045, a1046) {
                    Ok(f432(v0, v1))
                } else {
                    let a1047 = v2;
                    let a1048 = 34u64;
                    if nat_eq(a1047, a1048) {
                        Ok(f433(v0, Adt124::C6(Adt125::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f434(Adt102::C2(v1)))
                    }
                }
            }
        }
        Adt124::C4 => {
            if f428(v2) {
                Ok(Adt141::C1(v0))
            } else {
                let a1049 = v2;
                let a1050 = 34u64;
                if nat_eq(a1049, a1050) {
                    Ok(f433(v0, Adt124::C6(Adt125::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f434(Adt102::C2(v1)))
                }
            }
        }
        Adt124::C5 => {
            if f428(v2) {
                Ok(Adt141::C1(v0))
            } else {
                let a1051 = v2;
                let a1052 = 58u64;
                if nat_eq(a1051, a1052) {
                    Ok(f433(v0, Adt124::C0))
                } else {
                    Ok(f434(Adt102::C2(v1)))
                }
            }
        }
        Adt124::C6(v3, v4, v5) => {
            f435(v0, v1, v2, v3, v4, v5)
        }
        Adt124::C7(v6, v7) => {
            f436(v0, v1, v2, v6, v7)
        }
        Adt124::C8(v8) => {
            f437(v0, v1, v2, v8)
        }
        Adt124::C9(_) => {
            Ok(f431(v0, v1, v2))
        }
    }
}

pub fn f355(v0: u64) -> R<u64> {
    let a1053 = 64u64;
    let a1054 = v0;
    if if nat_lt(a1053, a1054) {
        let a1055 = v0;
        let a1056 = 91u64;
        nat_lt(a1055, a1056)
    } else {
        false
    } {
        let a1057 = v0;
        let a1058 = 32u64;
        nat_add(a1057, a1058)
    } else {
        Ok(v0)
    }
}

pub fn f356(p1061: List<u64>) -> List<u64> {
    let m1060 = p1061;
    {
        let mut m1060 = m1060;
        loop {
            let step = {
                let v0 = m1060;
                let m1059 = v0;
                match m1059.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f438(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
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

pub fn f357(v0: u64) -> bool {
    let a1062 = v0;
    let a1063 = 9u64;
    if nat_eq(a1062, a1063) {
        true
    } else {
        let a1064 = 31u64;
        let a1065 = v0;
        if nat_lt(a1064, a1065) {
            let a1066 = v0;
            let a1067 = 127u64;
            nat_lt(a1066, a1067)
        } else {
            false
        }
    }
}

pub fn f358(p1070: List<u64>) -> bool {
    let m1069 = p1070;
    {
        let mut m1069 = m1069;
        loop {
            let step = {
                let v0 = m1069;
                let m1068 = v0;
                match m1068.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f439(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(p1073: List<u64>, v1: bool) -> bool {
    let m1072 = p1073;
    {
        let mut m1072 = m1072;
        loop {
            let step = {
                let v0 = m1072;
                let m1071 = v0;
                match m1071.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f440(v2)
                        } else {
                            f441(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360() -> u64 {
    64u64
}

pub fn f361(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1074 = f296(Str::lit("type"), v1);
    match m1074 {
        Adt107::C0 => {
            Some(Adt97::C3(v0))
        }
        Adt107::C1(v2) => {
            let m1075 = f141(v2);
            match m1075 {
                None => {
                    Some(Adt97::C3(v0))
                }
                Some(v3) => {
                    let a1076 = v3;
                    let a1077 = Str::lit("object");
                    if equal(a1076, a1077) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f362(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1078 = f296(Str::lit("$schema"), v1);
    match m1078 {
        Adt107::C0 => {
            None::<Adt97>
        }
        Adt107::C1(v2) => {
            let m1079 = f141(v2);
            match m1079 {
                None => {
                    Some(Adt97::C2(v0))
                }
                Some(v3) => {
                    let a1080 = v3;
                    let a1081 = f442();
                    if equal(a1080, a1081) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f363(v0: Adt36) -> R<Option<Adt97>> {
    let m1083 = match v0.clone() {
        Adt36::C0(_, _, _, _, _, h1082, _, _) => {
            h1082
        }
    };
    match m1083 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v1) => {
            let m1086 = f362(match v0.clone() {
                Adt36::C0(h1084, _, _, _, _, _, _, _) => {
                    h1084
                }
            }, match v1.clone() {
                Adt5::C0(h1085) => {
                    h1085
                }
            });
            match m1086 {
                None => {
                    let m1090 = f286(f287(), match v0 {
                        Adt36::C0(h1087, _, _, _, _, _, _, _) => {
                            h1087
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1088) => {
                            h1088
                        }
                    }), List::cons(Adt109::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1089) => {
                            h1089
                        }
                    }), false, List::<Str>::nil()), List::<Adt109>::nil()), f288())?;
                    match m1090 {
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

pub fn f364(v0: Str, v1: Adt6, v2: Adt109, v3: Adt129, v4: List<Adt109>) -> R<Result<(List<Adt109>, Adt129), Adt97>> {
    let m1091 = v2;
    match m1091 {
        Adt109::C0(v5, v6, v7) => {
            let m1092 = f443(v5.clone());
            match m1092 {
                None => {
                    let m1093 = f219(v5);
                    match m1093 {
                        None => {
                            Ok(Err::<(List<Adt109>, Adt129), Adt97>(Adt97::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f444(v0, v1, match v9 {
                                Adt5::C0(h1094) => {
                                    h1094
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt109>, Adt129), Adt97>((v4, v3)))
                }
            }
        }
    }
}

pub fn f365(p1103: List<Adt39>) -> R<bool> {
    let m1102 = p1103;
    {
        let mut m1102 = m1102;
        loop {
            let step = {
                let v0 = m1102;
                let m1095 = v0;
                match m1095.uncons() {
                    None => {
                        Err::<List<Adt39>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m1096 = v1;
                        match m1096 {
                            Adt39::C0(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C1(v4) => {
                                if f445(match v4 {
                                    Adt43::C0(h1097, _, _, _) => {
                                        h1097
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C2(v5) => {
                                if f445(match v5 {
                                    Adt44::C0(h1098, _, _, _) => {
                                        h1098
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
                                let m1100 = match v7 {
                                    Adt46::C0(h1099, _, _) => {
                                        h1099
                                    }
                                };
                                match m1100 {
                                    Adt47::C0(_) => {
                                        Ok::<List<Adt39>, R<bool>>(v2)
                                    }
                                    Adt47::C1(v9) => {
                                        if f445(match v9 {
                                            Adt49::C0(_, _, h1101, _) => {
                                                h1101
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
                Ok(next) => { m1102 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f446(match v1.clone() {
        Adt50::C0(h1104, _) => {
            h1104
        }
    }) {
        f447(v1)
    } else {
        let m1106 = match v0 {
            Adt36::C0(_, _, _, _, _, h1105, _, _) => {
                h1105
            }
        };
        match m1106 {
            None => {
                f447(v1)
            }
            Some(v2) => {
                f448(v2, v1)
            }
        }
    }
}

pub fn f367() -> Adt90 {
    Adt90::C9(Str::lit("the host supplied no task id"))
}

pub fn f368(v0: Adt14, v1: Adt86) -> Adt85 {
    Adt85::C0(Adt63::C0(Adt64::C0(match v0.clone() {
        Adt14::C0(_, _, h1107, _) => {
            h1107
        }
    }, None::<Str>, match v0.clone() {
        Adt14::C0(_, h1108, _, _) => {
            h1108
        }
    }, match v0.clone() {
        Adt14::C0(_, h1109, _, _) => {
            h1109
        }
    }, Some(f449()), Some(f450()))), match v0 {
        Adt14::C0(h1110, _, _, _) => {
            h1110
        }
    }, v1)
}

pub fn f369(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt111>) -> R<Adt142> {
    let m1111 = f443(v1.clone());
    match m1111 {
        None => {
            let m1112 = f219(v1);
            match m1112 {
                None => {
                    Ok(Adt142::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f451(v0, match v5 {
                        Adt5::C0(h1113) => {
                            h1113
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt142::C3(List::<Adt113>::nil()))
            } else {
                Ok(Adt142::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f370(v0: Adt113, v1: List<Adt113>) -> R<Adt130> {
    let m1114 = v0;
    match m1114 {
        Adt113::C0(_, _, _) => {
            Ok(Adt130::C0(Adt112::C0, v1))
        }
        Adt113::C1(_, _, _) => {
            Ok(Adt130::C0(Adt112::C0, v1))
        }
        Adt113::C2(v8, v9, v10, v11) => {
            Ok(f452(v8, v9, v10, {
                let a1115 = v11;
                let a1116 = 1u64;
                nat_add(a1115, a1116)?
            }, v1))
        }
        Adt113::C3(v12) => {
            Ok(Adt130::C0(Adt112::C1(v12, Str::lit("not")), v1))
        }
        Adt113::C4(v13, _, v15, v16) => {
            Ok(f453(v13, v15, v16, v1))
        }
        Adt113::C5(_, _, _, _) => {
            Ok(Adt130::C0(Adt112::C0, v1))
        }
    }
}

pub fn f371(v0: Adt114, v1: Adt6, v2: Adt6, v3: List<Adt111>) -> Adt142 {
    let m1117 = f219(v1);
    match m1117 {
        None => {
            Adt142::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m1118 = v0;
            match m1118 {
                Adt114::C0 => {
                    f454(match v4 {
                        Adt5::C0(h1119) => {
                            h1119
                        }
                    }, v2, v3)
                }
                Adt114::C1 => {
                    f455(match v4 {
                        Adt5::C0(h1120) => {
                            h1120
                        }
                    }, v2, v3)
                }
                Adt114::C2 => {
                    f456(match v4 {
                        Adt5::C0(h1121) => {
                            h1121
                        }
                    }, v2, v3)
                }
                Adt114::C3 => {
                    f457(match v4 {
                        Adt5::C0(h1122) => {
                            h1122
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f372(v0: Adt113, v1: List<Adt113>, v2: List<Adt111>, v3: Str) -> Adt130 {
    let m1123 = v0;
    match m1123 {
        Adt113::C0(_, _, _) => {
            Adt130::C0(Adt112::C1(v2, v3), v1)
        }
        Adt113::C1(v7, v8, v9) => {
            f458(v7, v8, v9, v1)
        }
        Adt113::C2(v10, v11, v12, v13) => {
            f452(v10, v11, v12, v13, v1)
        }
        Adt113::C3(_) => {
            Adt130::C0(Adt112::C0, v1)
        }
        Adt113::C4(_, v16, v17, v18) => {
            f453(v16, v17, v18, v1)
        }
        Adt113::C5(_, _, _, _) => {
            Adt130::C0(Adt112::C1(v2, v3), v1)
        }
    }
}

pub fn f373(p1126: List<Adt111>, p1127: List<Str>) -> R<List<Str>> {
    let m1125 = (p1126, p1127);
    {
        let mut m1125 = m1125;
        loop {
            let step = {
                let (v0, v1) = m1125;
                let m1124 = v0;
                match m1124.uncons() {
                    None => {
                        Err::<(List<Adt111>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt111>, List<Str>), R<List<Str>>>((v3, List::cons(f459(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374(v0: Str, v1: Str) -> Adt143 {
    let a1128 = v0;
    let a1129 = v1;
    if equal(a1128, a1129) {
        Adt143::C0
    } else {
        Adt143::C1
    }
}

pub fn f375(p1144: u64, p1145: Adt131, p1146: List<Adt132>) -> R<Option<Adt6>> {
    let m1143 = (p1144, (p1145, p1146));
    {
        let mut m1143 = m1143;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1143;
                let m1130 = v0;
                if m1130 == 0 {
                    Err::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1130 - 1;
                    let m1131 = v1;
                    match m1131 {
                        Adt131::C0(v4) => {
                            let m1132 = v4.clone();
                            match m1132 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f460(v7)?;
                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1133 = v9;
                                    match m1133.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C0(v10), List::cons(Adt132::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1134 = v12;
                                    match m1134.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f460({
                                                let (h1135, _) = v13;
                                                h1135
                                            })?;
                                            Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C0({
                                                let (_, h1136) = v13;
                                                h1136
                                            }), List::cons(Adt132::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt131::C1(v16) => {
                            let m1137 = v2;
                            match m1137.uncons() {
                                None => {
                                    Err::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1138 = v17;
                                    match m1138 {
                                        Adt132::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1139 = v20;
                                            match m1139.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(Adt6::C4(f378(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C0(v22), List::cons(Adt132::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt132::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1140 = v26;
                                            match m1140.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(Adt6::C5(f461(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f460({
                                                        let (h1141, _) = v28;
                                                        h1141
                                                    })?;
                                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C0({
                                                        let (_, h1142) = v28;
                                                        h1142
                                                    }), List::cons(Adt132::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m1143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376() -> u64 {
    67108864u64
}

pub fn f377(p1150: List<Adt39>, p1151: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1149 = (p1150, p1151);
    {
        let mut m1149 = m1149;
        loop {
            let step = {
                let (v0, v1) = m1149;
                let m1147 = v0;
                match m1147.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1148 = f462(v2)?;
                        match m1148 {
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
                Ok(next) => { m1149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(p1154: List<Adt6>, p1155: List<Adt6>) -> List<Adt6> {
    let m1153 = (p1154, p1155);
    {
        let mut m1153 = m1153;
        loop {
            let step = {
                let (v0, v1) = m1153;
                let m1152 = v0;
                match m1152.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: Adt107) -> bool {
    let m1156 = v0;
    match m1156 {
        Adt107::C0 => {
            false
        }
        Adt107::C1(_) => {
            true
        }
    }
}

pub fn f380(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m1157 = f214(v0, v1);
    match m1157 {
        Adt107::C0 => {
            None::<Adt5>
        }
        Adt107::C1(v2) => {
            f219(v2)
        }
    }
}

pub fn f381(v0: Adt5) -> Option<Adt63> {
    let m1158 = f463(v0.clone());
    match m1158 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m1159 = f464(Str::lit("status"), v0.clone());
            match m1159 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let a1160 = v2.clone();
                    let a1161 = Str::lit("working");
                    if equal(a1160, a1161) {
                        Some(Adt63::C0(v1))
                    } else {
                        let a1162 = v2.clone();
                        let a1163 = Str::lit("cancelled");
                        if equal(a1162, a1163) {
                            Some(Adt63::C4(v1))
                        } else {
                            let a1164 = v2.clone();
                            let a1165 = Str::lit("input_required");
                            if equal(a1164, a1165) {
                                let m1166 = f465(Str::lit("inputRequests"), v0);
                                match m1166 {
                                    None => {
                                        None::<Adt63>
                                    }
                                    Some(v3) => {
                                        Some(Adt63::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a1167 = v2.clone();
                                let a1168 = Str::lit("completed");
                                if equal(a1167, a1168) {
                                    let m1169 = f380(Str::lit("result"), v0);
                                    match m1169 {
                                        None => {
                                            None::<Adt63>
                                        }
                                        Some(v4) => {
                                            Some(Adt63::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a1170 = v2;
                                    let a1171 = Str::lit("failed");
                                    if equal(a1170, a1171) {
                                        let m1172 = f380(Str::lit("error"), v0);
                                        match m1172 {
                                            None => {
                                                None::<Adt63>
                                            }
                                            Some(v5) => {
                                                let m1173 = f466(v5);
                                                match m1173 {
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

pub fn f382(v0: Str, v1: Adt5) -> Option<i64> {
    let m1174 = f214(v0, v1);
    match m1174 {
        Adt107::C0 => {
            None::<i64>
        }
        Adt107::C1(v2) => {
            f215(v2)
        }
    }
}

pub fn f383(v0: Adt5) -> Option<Adt86> {
    let m1175 = f382(Str::lit("parkAfterMs"), v0.clone());
    match m1175 {
        None => {
            None::<Adt86>
        }
        Some(v1) => {
            let m1176 = f465(Str::lit("parked"), v0.clone());
            match m1176 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m1177 = f382(Str::lit("finishAfterMs"), v0.clone());
                    match m1177 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            let m1178 = f380(Str::lit("outcome"), v0);
                            match m1178 {
                                None => {
                                    None::<Adt86>
                                }
                                Some(v4) => {
                                    let m1179 = f467(v4);
                                    match m1179 {
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

pub fn f384(p1182: List<(Str, Adt65)>, p1183: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m1181 = (p1182, p1183);
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let (v0, v1) = m1181;
                let m1180 = v0;
                match m1180.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v3, List::cons(v2, v1)))
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

pub fn f385(p1187: List<(Str, Adt65)>, v1: Adt5, p1188: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m1186 = (p1187, p1188);
    {
        let mut m1186 = m1186;
        loop {
            let step = {
                let (v0, v2) = m1186;
                let m1184 = v0;
                match m1184.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v2)
                    }
                    Some((v3, v4)) => {
                        if f303({
                            let (h1185, _) = v3.clone();
                            h1185
                        }, v1.clone()) {
                            Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v4, v2))
                        } else {
                            Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v4, List::cons(v3, v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: Str) -> R<Option<List<u64>>> {
    let m1189 = f468(f469(f470())?, f469(v0)?);
    match m1189 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1190 = f468(f471(List::cons(47u64, f469(f472())?)), f471(v1));
            match m1190 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f471(v2);
                    if if f473(v3.clone()) {
                        f474(v3.clone())
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

pub fn f387(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1191 = v0;
    let a1192 = f475(f179(), f476(List::<Bytes>::nil(), v1));
    append_bytes(a1191, a1192)
}

pub fn f388(v0: Adt6) -> R<Result<Bytes, Adt102>> {
    let m1193 = f181(v0)?;
    match m1193 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt102>(f387(f179(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt102>(v2))
        }
    }
}

pub fn f389(p1197: List<Str>, p1198: u64, p1199: List<Str>) -> List<Str> {
    let m1196 = (p1197, (p1198, p1199));
    {
        let mut m1196 = m1196;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1196;
                let m1194 = v0;
                match m1194.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m1195 = v1;
                        if m1195 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m1195 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: Str, v1: Str) -> R<bool> {
    Ok(f477(f469(v0)?, f469(v1)?))
}

pub fn f391(p1203: List<Adt45>, p1204: List<Str>) -> List<Str> {
    let m1202 = (p1203, p1204);
    {
        let mut m1202 = m1202;
        loop {
            let step = {
                let (v0, v1) = m1202;
                let m1200 = v0;
                match m1200.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt45::C0(_, _, _, h1201, _, _, _, _, _) => {
                                h1201
                            }
                        }, v1)))
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

pub fn f392(v0: i64, v1: u64) -> R<i64> {
    f302({
        let a1205 = v0;
        let a1206 = 256i64;
        int_mul(a1205, a1206)?
    }, f221(v1, 0i64)?)
}

pub fn f393(v0: List<u64>) -> Option<Str> {
    let a1207 = f387(f179(), v0);
    utf8_decode(a1207)
}

pub fn f394(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt14, u64), Adt93>> {
    let m1209 = f16(v0, {
        let (_, h1208) = v3.clone();
        h1208
    })?;
    match m1209 {
        None => {
            Ok(Err::<(Adt14, u64), Adt93>(Adt93::C0({
                let (_, h1210) = v3;
                h1210
            })))
        }
        Some(v4) => {
            Ok(f478(v1, v2, {
                let (h1211, _) = v3;
                h1211
            }, v4))
        }
    }
}

pub fn f395(v0: Adt115, v1: List<Adt115>, v2: List<u64>) -> R<Adt144> {
    let m1212 = v0;
    match m1212 {
        Adt115::C0(v3) => {
            Ok(Adt144::C1(v1, f335(v3, v2)))
        }
        Adt115::C1(v4, v5) => {
            f479(v4, v5, v1, v2)
        }
        Adt115::C2(v6, v7) => {
            let m1213 = v7;
            match m1213.uncons() {
                None => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt144::C1(List::cons(Adt115::C1(v6, v8), List::cons(Adt115::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt115::C3(v10, v11) => {
            let m1214 = v11.clone();
            match m1214.uncons() {
                None => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt115::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt115::C4(v14, v15) => {
            let m1215 = v15;
            match m1215.uncons() {
                None => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt144::C1(List::cons(Adt115::C0({
                        let a1217 = f480({
                            let (h1216, _) = v16;
                            h1216
                        })?;
                        let a1218 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1217, a1218)
                    }), List::cons(Adt115::C1(v14, {
                        let (_, h1219) = v16;
                        h1219
                    }), List::cons(Adt115::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt115::C5(v18, v19) => {
            let m1220 = v19.clone();
            match m1220.uncons() {
                None => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt115::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f396(v0: Adt134) -> R<Option<Adt5>> {
    let m1222 = f481(match v0.clone() {
        Adt134::C0(h1221, _, _) => {
            h1221
        }
    })?;
    match m1222 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1225 = v1;
                let a1226 = {
                    let m1224 = f482(v0);
                    match m1224 {
                        Adt5::C0(h1223) => {
                            h1223
                        }
                    }
                };
                append_list(a1225, a1226)
            })))
        }
    }
}

pub fn f397(v0: Adt14, v1: Option<Adt88>, v2: Adt94) -> R<Adt94> {
    let m1227 = v1;
    match m1227 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a1229 = match v2.clone() {
                Adt94::C0(h1228, _, _, _, _) => {
                    h1228
                }
            };
            let a1230 = f400();
            if nat_eq(a1229, a1230) {
                Ok(f483(f484(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f398(v0: Adt33) -> Option<Adt88> {
    let m1231 = v0;
    match m1231 {
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
                Adt83::C0(_, _, h1232) => {
                    h1232
                }
            }, Adt89::C0, false))
        }
        Adt33::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f399(v0: bool, v1: Adt94, v2: Option<List<List<u64>>>) -> Adt94 {
    let m1233 = v2;
    match m1233 {
        None => {
            f6()
        }
        Some(v3) => {
            f485(v0, v1, f45(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f400() -> u64 {
    200u64
}

pub fn f401(v0: Adt100, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f259())), List::cons((Str::lit("id"), f336(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1234) => {
            h1234
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f402(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m1235 = v0;
    match m1235 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m1236 = v1;
            match m1236 {
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

pub fn f403(p1240: List<Adt136>, p1241: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1239 = (p1240, p1241);
    {
        let mut m1239 = m1239;
        loop {
            let step = {
                let (v0, v1) = m1239;
                let m1237 = v0;
                match m1237.uncons() {
                    None => {
                        Err::<(List<Adt136>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1238 = f486(v2)?;
                        match m1238 {
                            None => {
                                Err::<(List<Adt136>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt136>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: Option<Adt88>) -> R<Option<List<List<u64>>>> {
    let m1242 = v0;
    match m1242 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f487(v1)
        }
    }
}

pub fn f405(v0: Adt137) -> u64 {
    let m1243 = v0;
    match m1243 {
        Adt137::C0(_) => {
            f188()
        }
        Adt137::C1(_, _) => {
            f188()
        }
        Adt137::C2(_) => {
            f188()
        }
        Adt137::C3(_) => {
            f488()
        }
        Adt137::C4(v6) => {
            f489(v6)
        }
    }
}

pub fn f406(v0: Adt137) -> R<Adt81> {
    Ok(Adt81::C0(f490(v0.clone()), f491(v0.clone()), f492(v0)?))
}

pub fn f407() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f408(p1246: List<List<u64>>) -> R<bool> {
    let m1245 = p1246;
    {
        let mut m1245 = m1245;
        loop {
            let step = {
                let v0 = m1245;
                let m1244 = v0;
                match m1244.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f493(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f409(v0: u64, p1251: List<u64>, p1252: List<u64>, p1253: List<List<u64>>) -> List<List<u64>> {
    let m1250 = (p1251, (p1252, p1253));
    {
        let mut m1250 = m1250;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1250;
                let m1247 = v1;
                match m1247.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f45(List::cons(f282(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1248 = v4;
                        let a1249 = v0;
                        if nat_eq(a1248, a1249) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f282(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: Adt0, v1: Fn3, v2: u64, v3: Result<Adt145, Adt137>) -> R<Result<Adt134, Adt137>> {
    let m1254 = v3;
    match m1254 {
        Ok(v5) => {
            f494(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt134, Adt137>(v4))
        }
    }
}

pub fn f411(v0: Option<Adt116>, v1: List<Str>, v2: Adt108) -> R<Result<Adt145, Adt137>> {
    Ok(f495(f496(match v2.clone() {
        Adt108::C0(_, h1255, _) => {
            h1255
        }
    }), f497(v0, v1, v2)?))
}

pub fn f412(v0: Adt118, v1: Adt105) -> Adt139 {
    let m1256 = v1;
    match m1256 {
        Adt105::C0 => {
            f418()
        }
        Adt105::C1 => {
            Adt139::C0(None::<Str>, Some(Adt117::C6(v0)))
        }
        Adt105::C2 => {
            Adt139::C0(None::<Str>, Some(Adt117::C7(v0)))
        }
        Adt105::C3(v2) => {
            let m1257 = f393(v2);
            match m1257 {
                None => {
                    Adt139::C0(None::<Str>, Some(Adt117::C6(v0)))
                }
                Some(v3) => {
                    Adt139::C0(Some(v3), None::<Adt117>)
                }
            }
        }
    }
}

pub fn f413() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f414() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f415(v0: Str) -> bool {
    let m1258 = f498(v0);
    match m1258 {
        Adt151::C0 => {
            false
        }
        Adt151::C1 => {
            true
        }
        Adt151::C2 => {
            true
        }
        Adt151::C3 => {
            true
        }
    }
}

pub fn f416(v0: Adt105) -> R<Adt105> {
    let m1259 = v0.clone();
    match m1259 {
        Adt105::C0 => {
            Ok(v0)
        }
        Adt105::C1 => {
            Ok(v0)
        }
        Adt105::C2 => {
            Ok(v0)
        }
        Adt105::C3(v1) => {
            let m1260 = f499(v1);
            match m1260 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1261 = f500(v2)?;
                    match m1261 {
                        None => {
                            Ok(Adt105::C1)
                        }
                        Some(v3) => {
                            Ok(Adt105::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f417() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f418() -> Adt139 {
    Adt139::C0(None::<Str>, None::<Adt117>)
}

pub fn f419(v0: Option<Adt117>, v1: Option<Adt117>) -> Option<Adt117> {
    let m1262 = v0.clone();
    match m1262 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f420(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt119>> {
    Ok(f501(f502(v0, v1, v2, List::<Adt119>::nil())?, List::<Adt119>::nil()))
}

pub fn f421() -> Str {
    Str::lit("2026-07-28")
}

pub fn f422(v0: Adt7) -> R<Option<i64>> {
    let m1265 = f503(40u64, match v0.clone() {
        Adt7::C0(h1263, _) => {
            h1263
        }
    }, match v0 {
        Adt7::C0(_, h1264) => {
            h1264
        }
    })?;
    match m1265 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f504(v1))
        }
    }
}

pub fn f423(v0: Adt100, v1: Adt5) -> Result<Adt108, Adt103> {
    let m1266 = f214(Str::lit("method"), v1.clone());
    match m1266 {
        Adt107::C0 => {
            f268(Some(v0), f348(v1))
        }
        Adt107::C1(v2) => {
            let m1267 = f141(v2);
            match m1267 {
                None => {
                    f268(Some(v0), Adt104::C4)
                }
                Some(v3) => {
                    f505(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f424(v0: u64) -> R<u64> {
    let a1270 = {
        let a1268 = v0;
        let a1269 = 3u64;
        nat_mul(a1268, a1269)?
    };
    let a1271 = 1u64;
    nat_add(a1270, a1271)
}

pub fn f425(v0: u64, v1: u64, v2: u64) -> bool {
    let a1272 = v1;
    let a1273 = v0;
    if nat_le(a1272, a1273) {
        let a1274 = v0;
        let a1275 = v2;
        nat_le(a1274, a1275)
    } else {
        false
    }
}

pub fn f426(v0: Adt126) -> R<Adt147> {
    if f506(match v0.clone() {
        Adt126::C0(_, h1276, _, _, _, _, _) => {
            h1276
        }
    }) {
        let v1: i64 = {
            let a1282 = if match v0.clone() {
                Adt126::C0(_, _, _, _, _, h1277, _) => {
                    h1277
                }
            } {
                let a1279 = match v0.clone() {
                    Adt126::C0(_, _, _, _, _, _, h1278) => {
                        h1278
                    }
                };
                int_neg(a1279)?
            } else {
                match v0.clone() {
                    Adt126::C0(_, _, _, _, _, _, h1280) => {
                        h1280
                    }
                }
            };
            let a1283 = match v0.clone() {
                Adt126::C0(_, _, _, _, h1281, _, _) => {
                    h1281
                }
            };
            int_sub(a1282, a1283)?
        };
        let m1286 = {
            let a1284 = v1;
            let a1285 = -9999i64;
            compare(a1284, a1285)
        };
        if if match m1286 {
            Ordering::Less => {
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
            let m1289 = {
                let a1287 = 9999i64;
                let a1288 = v1;
                compare(a1287, a1288)
            };
            match m1289 {
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
            Ok(Adt147::C1)
        } else {
            Ok(Adt147::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt126::C0(h1290, _, _, _, _, _, _) => {
                    h1290
                }
            } {
                let a1292 = match v0 {
                    Adt126::C0(_, _, h1291, _, _, _, _) => {
                        h1291
                    }
                };
                int_neg(a1292)?
            } else {
                match v0 {
                    Adt126::C0(_, _, h1293, _, _, _, _) => {
                        h1293
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt147::C0)
    }
}

pub fn f427(v0: Adt6, v1: List<Adt128>, v2: u64) -> Adt123 {
    let m1294 = v1.clone();
    match m1294.uncons() {
        None => {
            Adt123::C0(Adt124::C9(v0), List::<Adt128>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1295 = v3;
            match m1295 {
                Adt128::C0(v5) => {
                    Adt123::C0(Adt124::C2, List::cons(Adt128::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt128::C1(v6, v7) => {
                    let m1296 = v7;
                    match m1296 {
                        None => {
                            Adt123::C0(Adt124::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt123::C0(Adt124::C2, List::cons(Adt128::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f428(v0: u64) -> bool {
    let a1297 = v0;
    let a1298 = 32u64;
    if nat_eq(a1297, a1298) {
        true
    } else {
        let a1299 = v0;
        let a1300 = 10u64;
        if nat_eq(a1299, a1300) {
            true
        } else {
            let a1301 = v0;
            let a1302 = 13u64;
            if nat_eq(a1301, a1302) {
                true
            } else {
                let a1303 = v0;
                let a1304 = 9u64;
                nat_eq(a1303, a1304)
            }
        }
    }
}

pub fn f429(v0: Adt123, v1: u64, v2: u64) -> R<Adt141> {
    let a1305 = v2;
    let a1306 = 34u64;
    if nat_eq(a1305, a1306) {
        Ok(f433(v0, Adt124::C6(Adt125::C0, List::<u64>::nil(), false)))
    } else {
        let a1307 = v2;
        let a1308 = 123u64;
        if nat_eq(a1307, a1308) {
            f507(v0, Adt128::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt124::C3, v1)
        } else {
            let a1309 = v2;
            let a1310 = 91u64;
            if nat_eq(a1309, a1310) {
                f507(v0, Adt128::C0(List::<Adt6>::nil()), Adt124::C1, v1)
            } else {
                let a1311 = v2;
                let a1312 = 116u64;
                if nat_eq(a1311, a1312) {
                    Ok(f433(v0, Adt124::C7(0u64, 1u64)))
                } else {
                    let a1313 = v2;
                    let a1314 = 102u64;
                    if nat_eq(a1313, a1314) {
                        Ok(f433(v0, Adt124::C7(1u64, 1u64)))
                    } else {
                        let a1315 = v2;
                        let a1316 = 110u64;
                        if nat_eq(a1315, a1316) {
                            Ok(f433(v0, Adt124::C7(2u64, 1u64)))
                        } else {
                            let a1317 = v2;
                            let a1318 = 45u64;
                            if nat_eq(a1317, a1318) {
                                Ok(f433(v0, f508(true, Adt127::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1319 = v2;
                                let a1320 = 48u64;
                                if nat_eq(a1319, a1320) {
                                    Ok(f433(v0, f508(false, Adt127::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f425(v2, 49u64, 57u64) {
                                        Ok(f433(v0, f508(false, Adt127::C2, f509({
                                            let a1321 = v2;
                                            let a1322 = 48u64;
                                            nat_sub(a1321, a1322)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f434(Adt102::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f430(v0: Adt123, v1: u64) -> Adt141 {
    let m1324 = match v0.clone() {
        Adt123::C0(_, h1323, _) => {
            h1323
        }
    };
    match m1324.uncons() {
        None => {
            f434(Adt102::C2(v1))
        }
        Some((v2, v3)) => {
            let m1325 = v2;
            match m1325 {
                Adt128::C0(v4) => {
                    Adt141::C1(f427(Adt6::C4(f378(v4, List::<Adt6>::nil())), v3, {
                        let a1327 = match v0 {
                            Adt123::C0(_, _, h1326) => {
                                h1326
                            }
                        };
                        let a1328 = 1u64;
                        nat_sub(a1327, a1328)
                    }))
                }
                Adt128::C1(_, _) => {
                    f434(Adt102::C2(v1))
                }
            }
        }
    }
}

pub fn f431(v0: Adt123, v1: u64, v2: u64) -> Adt141 {
    let m1330 = match v0.clone() {
        Adt123::C0(h1329, _, _) => {
            h1329
        }
    };
    match m1330 {
        Adt124::C0 => {
            f434(Adt102::C2(v1))
        }
        Adt124::C1 => {
            f434(Adt102::C2(v1))
        }
        Adt124::C2 => {
            if f428(v2) {
                Adt141::C1(v0)
            } else {
                let a1331 = v2;
                let a1332 = 44u64;
                if nat_eq(a1331, a1332) {
                    f510(v0, v1)
                } else {
                    let a1333 = v2;
                    let a1334 = 93u64;
                    if nat_eq(a1333, a1334) {
                        f430(v0, v1)
                    } else {
                        let a1335 = v2;
                        let a1336 = 125u64;
                        if nat_eq(a1335, a1336) {
                            f432(v0, v1)
                        } else {
                            f434(Adt102::C2(v1))
                        }
                    }
                }
            }
        }
        Adt124::C3 => {
            f434(Adt102::C2(v1))
        }
        Adt124::C4 => {
            f434(Adt102::C2(v1))
        }
        Adt124::C5 => {
            f434(Adt102::C2(v1))
        }
        Adt124::C6(_, _, _) => {
            f434(Adt102::C2(v1))
        }
        Adt124::C7(_, _) => {
            f434(Adt102::C2(v1))
        }
        Adt124::C8(_) => {
            f434(Adt102::C2(v1))
        }
        Adt124::C9(_) => {
            if f428(v2) {
                Adt141::C1(v0)
            } else {
                f434(Adt102::C10(v1))
            }
        }
    }
}

pub fn f432(v0: Adt123, v1: u64) -> Adt141 {
    let m1338 = match v0.clone() {
        Adt123::C0(_, h1337, _) => {
            h1337
        }
    };
    match m1338.uncons() {
        None => {
            f434(Adt102::C2(v1))
        }
        Some((v2, v3)) => {
            let m1339 = v2;
            match m1339 {
                Adt128::C0(_) => {
                    f434(Adt102::C2(v1))
                }
                Adt128::C1(v5, _) => {
                    Adt141::C1(f427(Adt6::C5(f461(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1341 = match v0 {
                            Adt123::C0(_, _, h1340) => {
                                h1340
                            }
                        };
                        let a1342 = 1u64;
                        nat_sub(a1341, a1342)
                    }))
                }
            }
        }
    }
}

pub fn f433(v0: Adt123, v1: Adt124) -> Adt141 {
    Adt141::C1(f511(v0, v1))
}

pub fn f434(v0: Adt102) -> Adt141 {
    Adt141::C0(v0)
}

pub fn f435(v0: Adt123, v1: u64, v2: u64, v3: Adt125, v4: List<u64>, v5: bool) -> R<Adt141> {
    let m1343 = v3;
    match m1343 {
        Adt125::C0 => {
            let a1344 = v2;
            let a1345 = 34u64;
            if nat_eq(a1344, a1345) {
                Ok(f512(v0, v1, v4, v5))
            } else {
                let a1346 = v2;
                let a1347 = 92u64;
                if nat_eq(a1346, a1347) {
                    Ok(f433(v0, Adt124::C6(Adt125::C1, v4, v5)))
                } else {
                    let a1348 = v2;
                    let a1349 = 32u64;
                    if nat_lt(a1348, a1349) {
                        Ok(f434(Adt102::C4(v1)))
                    } else {
                        Ok(f433(v0, Adt124::C6(Adt125::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt125::C1 => {
            let a1350 = v2;
            let a1351 = 117u64;
            if nat_eq(a1350, a1351) {
                Ok(f433(v0, Adt124::C6(Adt125::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1352 = f513(v2);
                let a1353 = 0u64;
                if nat_eq(a1352, a1353) {
                    Ok(f434(Adt102::C5(v1)))
                } else {
                    Ok(f433(v0, Adt124::C6(Adt125::C0, List::cons(f513(v2), v4), v5)))
                }
            }
        }
        Adt125::C2(v6, v7, v8) => {
            let a1354 = 15u64;
            let a1355 = f514(v2);
            if nat_lt(a1354, a1355) {
                Ok(f434(Adt102::C5(v1)))
            } else {
                let a1358 = {
                    let a1356 = v6;
                    let a1357 = 1u64;
                    nat_add(a1356, a1357)?
                };
                let a1359 = 4u64;
                if nat_eq(a1358, a1359) {
                    f515(v0, v1, {
                        let a1362 = {
                            let a1360 = v7;
                            let a1361 = 16u64;
                            nat_mul(a1360, a1361)?
                        };
                        let a1363 = f514(v2);
                        nat_add(a1362, a1363)?
                    }, v8, v4, v5)
                } else {
                    Ok(f433(v0, Adt124::C6(Adt125::C2({
                        let a1364 = v6;
                        let a1365 = 1u64;
                        nat_add(a1364, a1365)?
                    }, {
                        let a1368 = {
                            let a1366 = v7;
                            let a1367 = 16u64;
                            nat_mul(a1366, a1367)?
                        };
                        let a1369 = f514(v2);
                        nat_add(a1368, a1369)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt125::C3(v9) => {
            let a1370 = v2;
            let a1371 = 92u64;
            if nat_eq(a1370, a1371) {
                Ok(f433(v0, Adt124::C6(Adt125::C4(v9), v4, v5)))
            } else {
                Ok(f434(Adt102::C6(v1)))
            }
        }
        Adt125::C4(v10) => {
            let a1372 = v2;
            let a1373 = 117u64;
            if nat_eq(a1372, a1373) {
                Ok(f433(v0, Adt124::C6(Adt125::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f434(Adt102::C6(v1)))
            }
        }
    }
}

pub fn f436(v0: Adt123, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt141> {
    let a1374 = v2;
    let a1375 = f516(v3, v4);
    if nat_eq(a1374, a1375) {
        let a1379 = {
            let a1376 = v4;
            let a1377 = 1u64;
            nat_add(a1376, a1377)?
        };
        let a1380 = {
            let a1378 = f517(v3);
            length_list(a1378)
        };
        if nat_eq(a1379, a1380) {
            Ok(f518(v0, f519(v3)))
        } else {
            Ok(f433(v0, Adt124::C7(v3, {
                let a1381 = v4;
                let a1382 = 1u64;
                nat_add(a1381, a1382)?
            })))
        }
    } else {
        Ok(f434(Adt102::C2(v1)))
    }
}

pub fn f437(v0: Adt123, v1: u64, v2: u64, v3: Adt126) -> R<Adt141> {
    let m1384 = match v3.clone() {
        Adt126::C0(_, h1383, _, _, _, _, _) => {
            h1383
        }
    };
    match m1384 {
        Adt127::C0 => {
            let a1385 = v2;
            let a1386 = 48u64;
            if nat_eq(a1385, a1386) {
                Ok(f520(v0, v3, Adt127::C1))
            } else {
                if f425(v2, 49u64, 57u64) {
                    f521(v0, v1, v2, v3, false)
                } else {
                    Ok(f434(Adt102::C7(v1)))
                }
            }
        }
        Adt127::C1 => {
            let a1387 = v2;
            let a1388 = 46u64;
            if nat_eq(a1387, a1388) {
                Ok(f520(v0, v3, Adt127::C3))
            } else {
                if f522(v2) {
                    Ok(f520(v0, v3, Adt127::C5))
                } else {
                    if f523(v2) {
                        Ok(f434(Adt102::C7(v1)))
                    } else {
                        f524(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt127::C2 => {
            if f523(v2) {
                f521(v0, v1, v2, v3, false)
            } else {
                let a1389 = v2;
                let a1390 = 46u64;
                if nat_eq(a1389, a1390) {
                    Ok(f520(v0, v3, Adt127::C3))
                } else {
                    if f522(v2) {
                        Ok(f520(v0, v3, Adt127::C5))
                    } else {
                        f524(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt127::C3 => {
            if f523(v2) {
                f521(v0, v1, v2, v3, true)
            } else {
                Ok(f434(Adt102::C7(v1)))
            }
        }
        Adt127::C4 => {
            if f523(v2) {
                f521(v0, v1, v2, v3, true)
            } else {
                if f522(v2) {
                    Ok(f520(v0, v3, Adt127::C5))
                } else {
                    f524(v0, v1, v2, v3)
                }
            }
        }
        Adt127::C5 => {
            let a1391 = v2;
            let a1392 = 43u64;
            if nat_eq(a1391, a1392) {
                Ok(f433(v0, f508(match v3.clone() {
                    Adt126::C0(h1393, _, _, _, _, _, _) => {
                        h1393
                    }
                }, Adt127::C6, match v3.clone() {
                    Adt126::C0(_, _, h1394, _, _, _, _) => {
                        h1394
                    }
                }, match v3.clone() {
                    Adt126::C0(_, _, _, h1395, _, _, _) => {
                        h1395
                    }
                }, match v3 {
                    Adt126::C0(_, _, _, _, h1396, _, _) => {
                        h1396
                    }
                }, false, 0i64)))
            } else {
                let a1397 = v2;
                let a1398 = 45u64;
                if nat_eq(a1397, a1398) {
                    Ok(f433(v0, f508(match v3.clone() {
                        Adt126::C0(h1399, _, _, _, _, _, _) => {
                            h1399
                        }
                    }, Adt127::C6, match v3.clone() {
                        Adt126::C0(_, _, h1400, _, _, _, _) => {
                            h1400
                        }
                    }, match v3.clone() {
                        Adt126::C0(_, _, _, h1401, _, _, _) => {
                            h1401
                        }
                    }, match v3 {
                        Adt126::C0(_, _, _, _, h1402, _, _) => {
                            h1402
                        }
                    }, true, 0i64)))
                } else {
                    if f523(v2) {
                        f525(v0, v1, v2, Adt126::C0(match v3.clone() {
                            Adt126::C0(h1403, _, _, _, _, _, _) => {
                                h1403
                            }
                        }, match v3.clone() {
                            Adt126::C0(_, h1404, _, _, _, _, _) => {
                                h1404
                            }
                        }, match v3.clone() {
                            Adt126::C0(_, _, h1405, _, _, _, _) => {
                                h1405
                            }
                        }, match v3.clone() {
                            Adt126::C0(_, _, _, h1406, _, _, _) => {
                                h1406
                            }
                        }, match v3 {
                            Adt126::C0(_, _, _, _, h1407, _, _) => {
                                h1407
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f434(Adt102::C7(v1)))
                    }
                }
            }
        }
        Adt127::C6 => {
            if f523(v2) {
                f525(v0, v1, v2, v3.clone(), match v3 {
                    Adt126::C0(_, _, _, _, _, h1408, _) => {
                        h1408
                    }
                })
            } else {
                Ok(f434(Adt102::C7(v1)))
            }
        }
        Adt127::C7 => {
            if f523(v2) {
                f525(v0, v1, v2, v3.clone(), match v3 {
                    Adt126::C0(_, _, _, _, _, h1409, _) => {
                        h1409
                    }
                })
            } else {
                f524(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f438(v0: u64) -> bool {
    let a1410 = v0;
    let a1411 = 9u64;
    if nat_eq(a1410, a1411) {
        true
    } else {
        let a1412 = v0;
        let a1413 = 32u64;
        nat_eq(a1412, a1413)
    }
}

pub fn f439(v0: u64) -> bool {
    let a1414 = 47u64;
    let a1415 = v0;
    if nat_lt(a1414, a1415) {
        let a1416 = v0;
        let a1417 = 58u64;
        nat_lt(a1416, a1417)
    } else {
        false
    }
}

pub fn f440(v0: u64) -> bool {
    let a1418 = 48u64;
    let a1419 = v0;
    if if nat_le(a1418, a1419) {
        let a1420 = v0;
        let a1421 = 57u64;
        nat_le(a1420, a1421)
    } else {
        false
    } {
        true
    } else {
        let a1422 = 65u64;
        let a1423 = v0;
        if if nat_le(a1422, a1423) {
            let a1424 = v0;
            let a1425 = 90u64;
            nat_le(a1424, a1425)
        } else {
            false
        } {
            true
        } else {
            let a1426 = 97u64;
            let a1427 = v0;
            if if nat_le(a1426, a1427) {
                let a1428 = v0;
                let a1429 = 122u64;
                nat_le(a1428, a1429)
            } else {
                false
            } {
                true
            } else {
                let a1430 = v0;
                let a1431 = 33u64;
                if nat_eq(a1430, a1431) {
                    true
                } else {
                    let a1432 = v0;
                    let a1433 = 35u64;
                    if nat_eq(a1432, a1433) {
                        true
                    } else {
                        let a1434 = v0;
                        let a1435 = 36u64;
                        if nat_eq(a1434, a1435) {
                            true
                        } else {
                            let a1436 = v0;
                            let a1437 = 37u64;
                            if nat_eq(a1436, a1437) {
                                true
                            } else {
                                let a1438 = v0;
                                let a1439 = 38u64;
                                if nat_eq(a1438, a1439) {
                                    true
                                } else {
                                    let a1440 = v0;
                                    let a1441 = 39u64;
                                    if nat_eq(a1440, a1441) {
                                        true
                                    } else {
                                        let a1442 = v0;
                                        let a1443 = 42u64;
                                        if nat_eq(a1442, a1443) {
                                            true
                                        } else {
                                            let a1444 = v0;
                                            let a1445 = 43u64;
                                            if nat_eq(a1444, a1445) {
                                                true
                                            } else {
                                                let a1446 = v0;
                                                let a1447 = 45u64;
                                                if nat_eq(a1446, a1447) {
                                                    true
                                                } else {
                                                    let a1448 = v0;
                                                    let a1449 = 46u64;
                                                    if nat_eq(a1448, a1449) {
                                                        true
                                                    } else {
                                                        let a1450 = v0;
                                                        let a1451 = 94u64;
                                                        if nat_eq(a1450, a1451) {
                                                            true
                                                        } else {
                                                            let a1452 = v0;
                                                            let a1453 = 95u64;
                                                            if nat_eq(a1452, a1453) {
                                                                true
                                                            } else {
                                                                let a1454 = v0;
                                                                let a1455 = 96u64;
                                                                if nat_eq(a1454, a1455) {
                                                                    true
                                                                } else {
                                                                    let a1456 = v0;
                                                                    let a1457 = 124u64;
                                                                    if nat_eq(a1456, a1457) {
                                                                        true
                                                                    } else {
                                                                        let a1458 = v0;
                                                                        let a1459 = 126u64;
                                                                        nat_eq(a1458, a1459)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f441(v0: u64) -> bool {
    let a1460 = 48u64;
    let a1461 = v0;
    if if nat_le(a1460, a1461) {
        let a1462 = v0;
        let a1463 = 57u64;
        nat_le(a1462, a1463)
    } else {
        false
    } {
        true
    } else {
        let a1464 = 65u64;
        let a1465 = v0;
        if if nat_le(a1464, a1465) {
            let a1466 = v0;
            let a1467 = 90u64;
            nat_le(a1466, a1467)
        } else {
            false
        } {
            true
        } else {
            let a1468 = 97u64;
            let a1469 = v0;
            if if nat_le(a1468, a1469) {
                let a1470 = v0;
                let a1471 = 122u64;
                nat_le(a1470, a1471)
            } else {
                false
            } {
                true
            } else {
                let a1472 = v0;
                let a1473 = 95u64;
                if nat_eq(a1472, a1473) {
                    true
                } else {
                    let a1474 = v0;
                    let a1475 = 45u64;
                    if nat_eq(a1474, a1475) {
                        true
                    } else {
                        let a1476 = v0;
                        let a1477 = 46u64;
                        nat_eq(a1476, a1477)
                    }
                }
            }
        }
    }
}

pub fn f442() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f443(v0: Adt6) -> Option<bool> {
    let m1478 = v0;
    match m1478 {
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

pub fn f444(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt129, v6: List<Adt109>) -> R<Result<(List<Adt109>, Adt129), Adt97>> {
    let m1479 = f526(v0.clone(), v1, v2.clone());
    match m1479 {
        None => {
            let m1480 = f527(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1480 {
                Adt152::C0(v8) => {
                    Ok(Err::<(List<Adt109>, Adt129), Adt97>(v8))
                }
                Adt152::C1(v9) => {
                    Ok(Ok::<(List<Adt109>, Adt129), Adt97>((f528(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt109>, Adt129), Adt97>(v7))
        }
    }
}

pub fn f445(v0: Str) -> R<bool> {
    let m1482 = f500(f177({
        let a1481 = v0;
        utf8_encode(a1481)
    })?)?;
    match m1482 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f446(v0: Adt38) -> bool {
    let m1484 = match v0 {
        Adt38::C0(_, _, h1483) => {
            h1483
        }
    };
    match m1484 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f447(v0: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1486 = match v0.clone() {
        Adt50::C0(_, h1485) => {
            h1485
        }
    };
    match m1486.uncons() {
        None => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(match v0 {
                Adt50::C0(h1487, _) => {
                    h1487
                }
            })))
        }
        Some((_, _)) => {
            if f529(match v0.clone() {
                Adt50::C0(_, h1488) => {
                    h1488
                }
            })? {
                Ok(Ok::<Adt33, Adt90>(Adt33::C3(v0)))
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f448(v0: Adt5, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1492 = {
        let m1491 = match v1.clone() {
            Adt50::C0(h1490, _) => {
                h1490
            }
        };
        match m1491 {
            Adt38::C0(_, h1489, _) => {
                h1489
            }
        }
    };
    match m1492 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1495 = f209(Adt6::C5(match v0.clone() {
                Adt5::C0(h1493) => {
                    h1493
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1494) => {
                    h1494
                }
            }), v2)?;
            match m1495 {
                Adt110::C0 => {
                    f447(v1)
                }
                Adt110::C1(_, _) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt110::C2(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt110::C3(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt110::C4 => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f449() -> i64 {
    60000i64
}

pub fn f450() -> i64 {
    1000i64
}

pub fn f451(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt111>) -> R<Adt142> {
    let m1496 = f530(v1.clone(), v2.clone())?;
    match m1496 {
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
            f531(v0, v1, v2, v3)
        }
    }
}

pub fn f452(v0: List<Adt6>, v1: Adt6, v2: List<Adt111>, v3: u64, v4: List<Adt113>) -> Adt130 {
    let m1497 = v0;
    match m1497.uncons() {
        None => {
            let a1498 = v3;
            let a1499 = 1u64;
            if nat_eq(a1498, a1499) {
                Adt130::C0(Adt112::C0, v4)
            } else {
                Adt130::C0(Adt112::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt130::C0(Adt112::C0, List::cons(Adt113::C0(v5, v1.clone(), v2.clone()), List::cons(Adt113::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f453(v0: Option<Adt6>, v1: Adt6, v2: List<Adt111>, v3: List<Adt113>) -> Adt130 {
    let m1500 = v0;
    match m1500 {
        None => {
            Adt130::C0(Adt112::C0, v3)
        }
        Some(v4) => {
            Adt130::C0(Adt112::C0, List::cons(Adt113::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f454(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m1501 = f296(Str::lit("anyOf"), v0);
    match m1501 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m1502 = f223(v3);
            match m1502 {
                None => {
                    Adt142::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1503 = v4;
                    match m1503.uncons() {
                        None => {
                            Adt142::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt142::C3(List::cons(Adt113::C0(v5, v1.clone(), v2.clone()), List::cons(Adt113::C1(v6, v1, v2), List::<Adt113>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f455(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m1504 = f296(Str::lit("oneOf"), v0);
    match m1504 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m1505 = f223(v3);
            match m1505 {
                None => {
                    Adt142::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1506 = v4;
                    match m1506.uncons() {
                        None => {
                            Adt142::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt142::C3(List::cons(Adt113::C0(v5, v1.clone(), v2.clone()), List::cons(Adt113::C2(v6, v1, v2, 0u64), List::<Adt113>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f456(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m1507 = f296(Str::lit("not"), v0);
    match m1507 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            Adt142::C3(List::cons(Adt113::C0(v3, v1, v2.clone()), List::cons(Adt113::C3(v2), List::<Adt113>::nil())))
        }
    }
}

pub fn f457(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m1508 = f296(Str::lit("if"), v0.clone());
    match m1508 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            Adt142::C3(List::cons(Adt113::C0(v3, v1.clone(), v2.clone()), List::cons(Adt113::C4(f532(Str::lit("then"), v0.clone()), f532(Str::lit("else"), v0), v1, v2), List::<Adt113>::nil())))
        }
    }
}

pub fn f458(v0: List<Adt6>, v1: Adt6, v2: List<Adt111>, v3: List<Adt113>) -> Adt130 {
    let m1509 = v0;
    match m1509.uncons() {
        None => {
            Adt130::C0(Adt112::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt130::C0(Adt112::C0, List::cons(Adt113::C0(v4, v1.clone(), v2.clone()), List::cons(Adt113::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f459(v0: Adt111) -> R<Str> {
    let m1510 = v0;
    match m1510 {
        Adt111::C0(v1) => {
            f533(v1)
        }
        Adt111::C1(v2) => {
            let a1511 = f221(v2, 0i64)?;
            Ok(format_i64(a1511))
        }
    }
}

pub fn f460(v0: Str) -> R<Str> {
    let m1514 = {
        let a1513 = f387(f179(), f335(f534(f177({
            let a1512 = v0;
            utf8_encode(a1512)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1513)
    };
    match m1514 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f461(p1517: List<(Str, Adt6)>, p1518: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1516 = (p1517, p1518);
    {
        let mut m1516 = m1516;
        loop {
            let step = {
                let (v0, v1) = m1516;
                let m1515 = v0;
                match m1515.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1516 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Adt39) -> R<Option<Adt6>> {
    let m1519 = v0;
    match m1519 {
        Adt39::C0(v1) => {
            Ok(Some(f535(v1)))
        }
        Adt39::C1(v2) => {
            Ok(Some(f536(Str::lit("image"), match v2.clone() {
                Adt43::C0(h1520, _, _, _) => {
                    h1520
                }
            }, match v2.clone() {
                Adt43::C0(_, h1521, _, _) => {
                    h1521
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h1522, _) => {
                    h1522
                }
            }, match v2 {
                Adt43::C0(_, _, _, h1523) => {
                    h1523
                }
            })))
        }
        Adt39::C2(v3) => {
            Ok(Some(f536(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h1524, _, _, _) => {
                    h1524
                }
            }, match v3.clone() {
                Adt44::C0(_, h1525, _, _) => {
                    h1525
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h1526, _) => {
                    h1526
                }
            }, match v3 {
                Adt44::C0(_, _, _, h1527) => {
                    h1527
                }
            })))
        }
        Adt39::C3(v4) => {
            Ok(Some(f537(v4)?))
        }
        Adt39::C4(v5) => {
            Ok(Some(f538(v5)))
        }
    }
}

pub fn f463(v0: Adt5) -> Option<Adt64> {
    let m1528 = f464(Str::lit("taskId"), v0.clone());
    match m1528 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m1529 = f464(Str::lit("createdAt"), v0.clone());
            match m1529 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let m1530 = f464(Str::lit("lastUpdatedAt"), v0.clone());
                    match m1530 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v3) => {
                            Some(Adt64::C0(v1, f464(Str::lit("statusMessage"), v0.clone()), v2, v3, f382(Str::lit("ttlMs"), v0.clone()), f382(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f464(v0: Str, v1: Adt5) -> Option<Str> {
    let m1531 = f214(v0, v1);
    match m1531 {
        Adt107::C0 => {
            None::<Str>
        }
        Adt107::C1(v2) => {
            f141(v2)
        }
    }
}

pub fn f465(v0: Str, v1: Adt5) -> Option<List<(Str, Adt65)>> {
    let m1532 = f380(v0, v1);
    match m1532 {
        None => {
            None::<List<(Str, Adt65)>>
        }
        Some(v2) => {
            f539(match v2 {
                Adt5::C0(h1533) => {
                    h1533
                }
            }, List::<(Str, Adt65)>::nil())
        }
    }
}

pub fn f466(v0: Adt5) -> Option<Adt81> {
    let m1534 = f382(Str::lit("code"), v0.clone());
    match m1534 {
        None => {
            None::<Adt81>
        }
        Some(v1) => {
            let m1535 = f464(Str::lit("message"), v0.clone());
            match m1535 {
                None => {
                    None::<Adt81>
                }
                Some(v2) => {
                    Some(Adt81::C0(v1, v2, {
                        let m1536 = f214(Str::lit("data"), v0);
                        match m1536 {
                            Adt107::C0 => {
                                None::<Adt6>
                            }
                            Adt107::C1(v3) => {
                                Some(v3)
                            }
                        }
                    }))
                }
            }
        }
    }
}

pub fn f467(v0: Adt5) -> Option<Adt87> {
    let m1537 = f380(Str::lit("completes"), v0.clone());
    match m1537 {
        None => {
            let m1538 = f380(Str::lit("fails"), v0);
            match m1538 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m1539 = f466(v2);
                    match m1539 {
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

pub fn f468(p1545: List<u64>, p1546: List<u64>) -> Option<List<u64>> {
    let m1544 = (p1545, p1546);
    {
        let mut m1544 = m1544;
        loop {
            let step = {
                let (v0, v1) = m1544;
                let m1540 = v0;
                match m1540.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1541 = v1;
                        match m1541.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1542 = v2;
                                let a1543 = v4;
                                if nat_eq(a1542, a1543) {
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
                Ok(next) => { m1544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: Str) -> R<List<u64>> {
    f177({
        let a1547 = v0;
        utf8_encode(a1547)
    })
}

pub fn f470() -> Str {
    Str::lit("test://template/")
}

pub fn f471(v0: List<u64>) -> List<u64> {
    f335(v0, List::<u64>::nil())
}

pub fn f472() -> Str {
    Str::lit("data")
}

pub fn f473(v0: List<u64>) -> bool {
    let m1548 = v0;
    match m1548.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f474(p1553: List<u64>) -> bool {
    let m1552 = p1553;
    {
        let mut m1552 = m1552;
        loop {
            let step = {
                let v0 = m1552;
                let m1549 = v0;
                match m1549.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1550 = v1;
                        let a1551 = 47u64;
                        if nat_eq(a1550, a1551) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
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

pub fn f475(p1558: Bytes, p1559: List<Bytes>) -> Bytes {
    let m1557 = (p1558, p1559);
    {
        let mut m1557 = m1557;
        loop {
            let step = {
                let (v0, v1) = m1557;
                let m1554 = v1;
                match m1554.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1555 = v2;
                            let a1556 = v0;
                            append_bytes(a1555, a1556)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(p1562: List<Bytes>, p1563: List<u64>) -> List<Bytes> {
    let m1561 = (p1562, p1563);
    {
        let mut m1561 = m1561;
        loop {
            let step = {
                let (v0, v1) = m1561;
                let m1560 = v1;
                match m1560.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f540(f541(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1561 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(p1569: List<u64>, p1570: List<u64>) -> bool {
    let m1568 = (p1569, p1570);
    {
        let mut m1568 = m1568;
        loop {
            let step = {
                let (v0, v1) = m1568;
                let m1564 = v0;
                match m1564.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1565 = v1;
                        match m1565.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1566 = v2;
                                let a1567 = v4;
                                if nat_eq(a1566, a1567) {
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
                Ok(next) => { m1568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: i64, v1: Str, v2: Str, v3: Adt95) -> Result<(Adt14, u64), Adt93> {
    Ok::<(Adt14, u64), Adt93>((Adt14::C0(v0, v1, v2, match v3.clone() {
        Adt95::C0(h1571, _) => {
            h1571
        }
    }), match v3 {
        Adt95::C0(_, h1572) => {
            h1572
        }
    }))
}

pub fn f479(v0: u64, v1: Adt6, v2: List<Adt115>, v3: List<u64>) -> R<Adt144> {
    let m1573 = v1;
    match m1573 {
        Adt6::C0 => {
            Ok(Adt144::C1(v2, f335(f542(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt144::C1(v2, f335(if v4 {
                f542(0u64)
            } else {
                f542(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f543(v5.clone())? {
                Ok(Adt144::C1(v2, f335(f544(v5)?, v3)))
            } else {
                Ok(Adt144::C0(Adt102::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt144::C1(v2, f335(f480(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1574 = v0;
            let a1575 = f545();
            if nat_lt(a1574, a1575) {
                Ok(Adt144::C1(f546(91u64, Adt115::C2({
                    let a1576 = v0;
                    let a1577 = 1u64;
                    nat_add(a1576, a1577)?
                }, v7), v2), v3))
            } else {
                Ok(Adt144::C0(Adt102::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1578 = v0;
            let a1579 = f545();
            if nat_lt(a1578, a1579) {
                Ok(Adt144::C1(f546(123u64, Adt115::C4({
                    let a1580 = v0;
                    let a1581 = 1u64;
                    nat_add(a1580, a1581)?
                }, v8), v2), v3))
            } else {
                Ok(Adt144::C0(Adt102::C12))
            }
        }
    }
}

pub fn f480(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1583 = f335(f547(f177({
            let a1582 = v0;
            utf8_encode(a1582)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1584 = List::cons(34u64, List::<u64>::nil());
        append_list(a1583, a1584)
    }))
}

pub fn f481(v0: Adt33) -> R<Option<List<(Str, Adt6)>>> {
    let m1585 = v0;
    match m1585 {
        Adt33::C0(v1) => {
            Ok(Some(f548(v1)?))
        }
        Adt33::C1(v2) => {
            Ok(Some(f549(v2)?))
        }
        Adt33::C2(v3) => {
            f216(f217(v3)?)
        }
        Adt33::C3(v4) => {
            f216(f217(match v4 {
                Adt50::C0(h1586, _) => {
                    h1586
                }
            })?)
        }
        Adt33::C4(v5) => {
            Ok(Some(f550(v5)?))
        }
        Adt33::C5(v6) => {
            Ok(Some(f551(v6)?))
        }
        Adt33::C6(v7) => {
            Ok(Some(f552(v7)?))
        }
        Adt33::C7(v8) => {
            Ok(Some(f553(v8)?))
        }
        Adt33::C8(v9) => {
            f216(f554(v9)?)
        }
        Adt33::C9(v10) => {
            Ok(Some(f555(v10)?))
        }
        Adt33::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt33::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt33::C12(v12) => {
            f216(f556(v12))
        }
        Adt33::C13(v13) => {
            f216(f557({
                let m1589 = match v13 {
                    Adt83::C0(_, _, h1588) => {
                        h1588
                    }
                };
                match m1589 {
                    Adt85::C0(h1587, _, _) => {
                        h1587
                    }
                }
            }))
        }
        Adt33::C14(v14) => {
            Ok(f558(v14))
        }
    }
}

pub fn f482(v0: Adt134) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f559(f560(match v0.clone() {
        Adt134::C0(h1590, _, _) => {
            h1590
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f561({
        let m1593 = match v0 {
            Adt134::C0(_, h1592, _) => {
                h1592
            }
        };
        match m1593 {
            Adt135::C0(h1591, _) => {
                h1591
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f483(v0: Option<Option<List<u64>>>, v1: Adt94) -> Adt94 {
    let m1594 = v0;
    match m1594 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt94::C0(match v1.clone() {
                Adt94::C0(h1595, _, _, _, _) => {
                    h1595
                }
            }, match v1.clone() {
                Adt94::C0(_, h1596, _, _, _) => {
                    h1596
                }
            }, match v1.clone() {
                Adt94::C0(_, _, h1597, _, _) => {
                    h1597
                }
            }, match v1 {
                Adt94::C0(_, _, _, h1598, _) => {
                    h1598
                }
            }, v2)
        }
    }
}

pub fn f484(v0: Adt14, v1: Adt88) -> R<Option<Option<List<u64>>>> {
    let m1600 = f154(match v0.clone() {
        Adt14::C0(_, _, _, h1599) => {
            h1599
        }
    })?;
    match m1600 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1603 = f562(f563(match v1 {
                Adt88::C0(h1601, _, _) => {
                    h1601
                }
            }, v2, match v0 {
                Adt14::C0(h1602, _, _, _) => {
                    h1602
                }
            })?)?;
            match m1603 {
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

pub fn f485(v0: bool, v1: Adt94, v2: List<List<u64>>) -> Adt94 {
    if f564(v0, match v1.clone() {
        Adt94::C0(h1604, _, _, _, _) => {
            h1604
        }
    }, v2.clone()) {
        f565(match v1.clone() {
            Adt94::C0(h1605, _, _, _, _) => {
                h1605
            }
        }, {
            let a1607 = v2;
            let a1608 = List::cons(match v1 {
                Adt94::C0(_, _, _, h1606, _) => {
                    h1606
                }
            }, List::<List<u64>>::nil());
            append_list(a1607, a1608)
        })
    } else {
        v1
    }
}

pub fn f486(v0: Adt136) -> R<Option<List<u64>>> {
    let m1609 = f298(f566(v0))?;
    match m1609 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1610 = f181(v1)?;
            match m1610 {
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

pub fn f487(v0: Adt88) -> R<Option<List<List<u64>>>> {
    if f567(v0.clone()) {
        let m1614 = f568({
            let m1613 = match v0 {
                Adt88::C0(h1612, _, _) => {
                    h1612
                }
            };
            match m1613 {
                Adt85::C0(h1611, _, _) => {
                    h1611
                }
            }
        });
        match m1614 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1615 = f298(v1)?;
                match m1615 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1616 = f181(v2)?;
                        match m1616 {
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

pub fn f488() -> u64 {
    404u64
}

pub fn f489(v0: Adt90) -> u64 {
    let m1617 = v0;
    match m1617 {
        Adt90::C0(_) => {
            f188()
        }
        Adt90::C1(_) => {
            f188()
        }
        Adt90::C2(_) => {
            f188()
        }
        Adt90::C3(_) => {
            f188()
        }
        Adt90::C4(_) => {
            f188()
        }
        Adt90::C5(_) => {
            f188()
        }
        Adt90::C6(_) => {
            f22()
        }
        Adt90::C7(_) => {
            f569()
        }
        Adt90::C8(_) => {
            f188()
        }
        Adt90::C9(_) => {
            f22()
        }
    }
}

pub fn f490(v0: Adt137) -> i64 {
    let m1618 = v0;
    match m1618 {
        Adt137::C0(_) => {
            f570()
        }
        Adt137::C1(_, _) => {
            f571()
        }
        Adt137::C2(_) => {
            f572()
        }
        Adt137::C3(_) => {
            f573()
        }
        Adt137::C4(v6) => {
            f574(v6)
        }
    }
}

pub fn f491(v0: Adt137) -> Str {
    let m1619 = v0;
    match m1619 {
        Adt137::C0(v1) => {
            f575(v1)
        }
        Adt137::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt137::C2(v4) => {
            f576(v4)
        }
        Adt137::C3(_) => {
            Str::lit("Method not found")
        }
        Adt137::C4(v6) => {
            let m1620 = v6;
            match m1620 {
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

pub fn f492(v0: Adt137) -> R<Option<Adt6>> {
    let m1621 = v0;
    match m1621 {
        Adt137::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt137::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f577(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt137::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt137::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt137::C4(v6) => {
            let m1622 = v6;
            match m1622 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f221(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f578(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f493(v0: List<u64>) -> R<bool> {
    let m1623 = f409(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1623.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f579(v2)? {
                Ok(false)
            } else {
                f272(f274(v1), f580())
            }
        }
    }
}

pub fn f494(v0: Adt0, v1: Fn3, v2: u64, v3: Adt145) -> R<Result<Adt134, Adt137>> {
    f581(v0.clone(), v1, v2, v3.clone(), f582(match v0 {
        Adt0::C0(_, _, h1624, _, _, _) => {
            h1624
        }
    }, match v3 {
        Adt145::C0(_, h1625, _, _) => {
            h1625
        }
    }))
}

pub fn f495(v0: Adt148, v1: Adt149) -> Result<Adt145, Adt137> {
    let m1626 = v1;
    match m1626 {
        Adt149::C0(v3) => {
            f583(v3, v0)
        }
        Adt149::C1(v2) => {
            Err::<Adt145, Adt137>(v2)
        }
    }
}

pub fn f496(v0: Str) -> Adt148 {
    let a1627 = v0.clone();
    let a1628 = Str::lit("server/discover");
    if equal(a1627, a1628) {
        Adt148::C1(Adt146::C0)
    } else {
        let a1629 = v0.clone();
        let a1630 = Str::lit("tools/list");
        if equal(a1629, a1630) {
            Adt148::C1(Adt146::C1)
        } else {
            let a1631 = v0.clone();
            let a1632 = Str::lit("tools/call");
            if equal(a1631, a1632) {
                Adt148::C1(Adt146::C2)
            } else {
                let a1633 = v0.clone();
                let a1634 = Str::lit("resources/list");
                if equal(a1633, a1634) {
                    Adt148::C1(Adt146::C3)
                } else {
                    let a1635 = v0.clone();
                    let a1636 = Str::lit("resources/read");
                    if equal(a1635, a1636) {
                        Adt148::C1(Adt146::C4)
                    } else {
                        let a1637 = v0.clone();
                        let a1638 = Str::lit("resources/templates/list");
                        if equal(a1637, a1638) {
                            Adt148::C1(Adt146::C5)
                        } else {
                            let a1639 = v0.clone();
                            let a1640 = Str::lit("prompts/list");
                            if equal(a1639, a1640) {
                                Adt148::C1(Adt146::C6)
                            } else {
                                let a1641 = v0.clone();
                                let a1642 = Str::lit("prompts/get");
                                if equal(a1641, a1642) {
                                    Adt148::C1(Adt146::C7)
                                } else {
                                    let a1643 = v0.clone();
                                    let a1644 = Str::lit("completion/complete");
                                    if equal(a1643, a1644) {
                                        Adt148::C1(Adt146::C8)
                                    } else {
                                        let a1645 = v0.clone();
                                        let a1646 = Str::lit("tasks/get");
                                        if equal(a1645, a1646) {
                                            Adt148::C1(Adt146::C9)
                                        } else {
                                            let a1647 = v0.clone();
                                            let a1648 = Str::lit("tasks/update");
                                            if equal(a1647, a1648) {
                                                Adt148::C1(Adt146::C10)
                                            } else {
                                                let a1649 = v0;
                                                let a1650 = Str::lit("tasks/cancel");
                                                if equal(a1649, a1650) {
                                                    Adt148::C1(Adt146::C11)
                                                } else {
                                                    Adt148::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f497(v0: Option<Adt116>, v1: List<Str>, v2: Adt108) -> R<Adt149> {
    f584(v0, v1, v2.clone(), f585(match v2 {
        Adt108::C0(_, _, h1651) => {
            h1651
        }
    }))
}

pub fn f498(v0: Str) -> Adt151 {
    let m1652 = f496(v0);
    match m1652 {
        Adt148::C0 => {
            Adt151::C0
        }
        Adt148::C1(v1) => {
            f586(v1)
        }
    }
}

pub fn f499(v0: List<u64>) -> Option<List<u64>> {
    let m1653 = f587(f588(), v0);
    match m1653 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1654 = f587(f589(), f282(v1, List::<u64>::nil()));
            match m1654 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f282(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f500(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1656 = f590({
        let a1655 = v0.clone();
        length_list(a1655)
    }, v0, List::<u64>::nil())?;
    match m1656 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f335(v1, List::<u64>::nil())))
        }
    }
}

pub fn f501(p1659: List<Adt119>, p1660: List<Adt119>) -> List<Adt119> {
    let m1658 = (p1659, p1660);
    {
        let mut m1658 = m1658;
        loop {
            let step = {
                let (v0, v1) = m1658;
                let m1657 = v0;
                match m1657.uncons() {
                    None => {
                        Err::<(List<Adt119>, List<Adt119>), List<Adt119>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt119>, List<Adt119>), List<Adt119>>((v3, List::cons(v2, v1)))
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

pub fn f502(v0: List<Adt92>, p1667: List<Adt12>, v2: Option<Str>, p1668: List<Adt119>) -> R<List<Adt119>> {
    let m1666 = (p1667, p1668);
    {
        let mut m1666 = m1666;
        loop {
            let step = {
                let (v1, v3) = m1666;
                let m1661 = v1;
                match m1661.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt119>), R<List<Adt119>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f591(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt119>), R<List<Adt119>>>((v5, List::cons(Adt119::C0(match v4.clone() {
                                Adt12::C0(_, h1662, _, _) => {
                                    h1662
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1663, _) => {
                                    h1663
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1664) => {
                                    h1664
                                }
                            }, f592(v0.clone(), match v4 {
                                Adt12::C0(_, h1665, _, _) => {
                                    h1665
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt119>), R<List<Adt119>>>((v5, v3))
                        }
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

pub fn f503(p1684: u64, p1685: i64, p1686: i64) -> R<Option<i64>> {
    let m1683 = (p1684, (p1685, p1686));
    {
        let mut m1683 = m1683;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1683;
                let m1669 = v0;
                if m1669 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1669 - 1;
                    if f297(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1672 = {
                            let a1670 = v2;
                            let a1671 = 0i64;
                            compare(a1670, a1671)
                        };
                        if match m1672 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f297({
                                let a1673 = v1;
                                let a1674 = 10i64;
                                let a1675 = 0i64;
                                int_rem(a1673, a1674, a1675)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1676 = v1;
                                    let a1677 = 10i64;
                                    let a1678 = 0i64;
                                    int_quot(a1676, a1677, a1678)?
                                }, f302(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1679 = v1;
                                let a1680 = 10i64;
                                int_mul(a1679, a1680)?
                            }, {
                                let a1681 = v2;
                                let a1682 = 1i64;
                                int_sub(a1681, a1682)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: i64) -> Option<i64> {
    let m1689 = {
        let a1687 = 9007199254740991i64;
        let a1688 = v0;
        compare(a1687, a1688)
    };
    if if match m1689 {
        Ordering::Less => {
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
        let m1692 = {
            let a1690 = v0;
            let a1691 = -9007199254740991i64;
            compare(a1690, a1691)
        };
        match m1692 {
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

pub fn f505(v0: Adt100, v1: Str, v2: Adt5) -> Result<Adt108, Adt103> {
    let m1693 = f214(Str::lit("params"), v2);
    match m1693 {
        Adt107::C0 => {
            Ok::<Adt108, Adt103>(Adt108::C0(v0, v1, None::<Adt5>))
        }
        Adt107::C1(v3) => {
            let m1694 = f219(v3);
            match m1694 {
                None => {
                    f268(Some(v0), Adt104::C6)
                }
                Some(v4) => {
                    Ok::<Adt108, Adt103>(Adt108::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f506(v0: Adt127) -> bool {
    let m1695 = v0;
    match m1695 {
        Adt127::C0 => {
            false
        }
        Adt127::C1 => {
            true
        }
        Adt127::C2 => {
            true
        }
        Adt127::C3 => {
            false
        }
        Adt127::C4 => {
            true
        }
        Adt127::C5 => {
            false
        }
        Adt127::C6 => {
            false
        }
        Adt127::C7 => {
            true
        }
    }
}

pub fn f507(v0: Adt123, v1: Adt128, v2: Adt124, v3: u64) -> R<Adt141> {
    let a1697 = match v0.clone() {
        Adt123::C0(_, _, h1696) => {
            h1696
        }
    };
    let a1698 = f545();
    if nat_lt(a1697, a1698) {
        Ok(Adt141::C1(Adt123::C0(v2, List::cons(v1, match v0.clone() {
            Adt123::C0(_, h1699, _) => {
                h1699
            }
        }), {
            let a1701 = match v0 {
                Adt123::C0(_, _, h1700) => {
                    h1700
                }
            };
            let a1702 = 1u64;
            nat_add(a1701, a1702)?
        })))
    } else {
        Ok(f434(Adt102::C9(v3)))
    }
}

pub fn f508(v0: bool, v1: Adt127, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt124 {
    Adt124::C8(Adt126::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f509(v0: u64) -> i64 {
    let a1703 = v0;
    let a1704 = 0u64;
    if nat_eq(a1703, a1704) {
        0i64
    } else {
        let a1705 = v0;
        let a1706 = 1u64;
        if nat_eq(a1705, a1706) {
            1i64
        } else {
            let a1707 = v0;
            let a1708 = 2u64;
            if nat_eq(a1707, a1708) {
                2i64
            } else {
                let a1709 = v0;
                let a1710 = 3u64;
                if nat_eq(a1709, a1710) {
                    3i64
                } else {
                    let a1711 = v0;
                    let a1712 = 4u64;
                    if nat_eq(a1711, a1712) {
                        4i64
                    } else {
                        let a1713 = v0;
                        let a1714 = 5u64;
                        if nat_eq(a1713, a1714) {
                            5i64
                        } else {
                            let a1715 = v0;
                            let a1716 = 6u64;
                            if nat_eq(a1715, a1716) {
                                6i64
                            } else {
                                let a1717 = v0;
                                let a1718 = 7u64;
                                if nat_eq(a1717, a1718) {
                                    7i64
                                } else {
                                    let a1719 = v0;
                                    let a1720 = 8u64;
                                    if nat_eq(a1719, a1720) {
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

pub fn f510(v0: Adt123, v1: u64) -> Adt141 {
    let m1722 = match v0.clone() {
        Adt123::C0(_, h1721, _) => {
            h1721
        }
    };
    match m1722.uncons() {
        None => {
            f434(Adt102::C2(v1))
        }
        Some((v2, _)) => {
            let m1723 = v2;
            match m1723 {
                Adt128::C0(_) => {
                    f433(v0, Adt124::C0)
                }
                Adt128::C1(_, _) => {
                    f433(v0, Adt124::C4)
                }
            }
        }
    }
}

pub fn f511(v0: Adt123, v1: Adt124) -> Adt123 {
    Adt123::C0(v1, match v0.clone() {
        Adt123::C0(_, h1724, _) => {
            h1724
        }
    }, match v0 {
        Adt123::C0(_, _, h1725) => {
            h1725
        }
    })
}

pub fn f512(v0: Adt123, v1: u64, v2: List<u64>, v3: bool) -> Adt141 {
    let m1727 = {
        let a1726 = f387(f179(), f335(v2, List::<u64>::nil()));
        utf8_decode(a1726)
    };
    match m1727 {
        None => {
            f434(Adt102::C14)
        }
        Some(v4) => {
            if v3 {
                f593(v0, v1, v4)
            } else {
                f518(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f513(v0: u64) -> u64 {
    let a1728 = v0;
    let a1729 = 34u64;
    if nat_eq(a1728, a1729) {
        34u64
    } else {
        let a1730 = v0;
        let a1731 = 92u64;
        if nat_eq(a1730, a1731) {
            92u64
        } else {
            let a1732 = v0;
            let a1733 = 47u64;
            if nat_eq(a1732, a1733) {
                47u64
            } else {
                let a1734 = v0;
                let a1735 = 98u64;
                if nat_eq(a1734, a1735) {
                    8u64
                } else {
                    let a1736 = v0;
                    let a1737 = 102u64;
                    if nat_eq(a1736, a1737) {
                        12u64
                    } else {
                        let a1738 = v0;
                        let a1739 = 110u64;
                        if nat_eq(a1738, a1739) {
                            10u64
                        } else {
                            let a1740 = v0;
                            let a1741 = 114u64;
                            if nat_eq(a1740, a1741) {
                                13u64
                            } else {
                                let a1742 = v0;
                                let a1743 = 116u64;
                                if nat_eq(a1742, a1743) {
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

pub fn f514(v0: u64) -> u64 {
    let a1744 = v0;
    let a1745 = 48u64;
    if nat_lt(a1744, a1745) {
        16u64
    } else {
        let a1746 = v0;
        let a1747 = 58u64;
        if nat_lt(a1746, a1747) {
            let a1748 = v0;
            let a1749 = 48u64;
            nat_sub(a1748, a1749)
        } else {
            let a1750 = v0;
            let a1751 = 65u64;
            if nat_lt(a1750, a1751) {
                16u64
            } else {
                let a1752 = v0;
                let a1753 = 71u64;
                if nat_lt(a1752, a1753) {
                    let a1754 = v0;
                    let a1755 = 55u64;
                    nat_sub(a1754, a1755)
                } else {
                    let a1756 = v0;
                    let a1757 = 97u64;
                    if nat_lt(a1756, a1757) {
                        16u64
                    } else {
                        let a1758 = v0;
                        let a1759 = 103u64;
                        if nat_lt(a1758, a1759) {
                            let a1760 = v0;
                            let a1761 = 87u64;
                            nat_sub(a1760, a1761)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f515(v0: Adt123, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt141> {
    let a1762 = v3;
    let a1763 = 0u64;
    if nat_eq(a1762, a1763) {
        if f425(v2, 55296u64, 56319u64) {
            Ok(f433(v0, Adt124::C6(Adt125::C3(v2), v4, v5)))
        } else {
            if f425(v2, 56320u64, 57343u64) {
                Ok(f434(Adt102::C6(v1)))
            } else {
                Ok(f433(v0, Adt124::C6(Adt125::C0, f594(v2, v4)?, v5)))
            }
        }
    } else {
        if f425(v2, 56320u64, 57343u64) {
            Ok(f433(v0, Adt124::C6(Adt125::C0, f594({
                let a1772 = 65536u64;
                let a1773 = {
                    let a1770 = {
                        let a1766 = {
                            let a1764 = v3;
                            let a1765 = 55296u64;
                            nat_sub(a1764, a1765)
                        };
                        let a1767 = 1024u64;
                        nat_mul(a1766, a1767)?
                    };
                    let a1771 = {
                        let a1768 = v2;
                        let a1769 = 56320u64;
                        nat_sub(a1768, a1769)
                    };
                    nat_add(a1770, a1771)?
                };
                nat_add(a1772, a1773)?
            }, v4)?, v5)))
        } else {
            Ok(f434(Adt102::C6(v1)))
        }
    }
}

pub fn f516(v0: u64, v1: u64) -> u64 {
    let m1776 = {
        let a1774 = f517(v0);
        let a1775 = v1;
        index_list(a1774, a1775)
    };
    match m1776 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f517(v0: u64) -> List<u64> {
    let a1777 = v0;
    let a1778 = 0u64;
    if nat_eq(a1777, a1778) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1779 = v0;
        let a1780 = 1u64;
        if nat_eq(a1779, a1780) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f518(v0: Adt123, v1: Adt6) -> Adt141 {
    Adt141::C1(f427(v1, match v0.clone() {
        Adt123::C0(_, h1781, _) => {
            h1781
        }
    }, match v0 {
        Adt123::C0(_, _, h1782) => {
            h1782
        }
    }))
}

pub fn f519(v0: u64) -> Adt6 {
    let a1783 = v0;
    let a1784 = 0u64;
    if nat_eq(a1783, a1784) {
        Adt6::C1(true)
    } else {
        let a1785 = v0;
        let a1786 = 1u64;
        if nat_eq(a1785, a1786) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f520(v0: Adt123, v1: Adt126, v2: Adt127) -> Adt141 {
    f433(v0, Adt124::C8(Adt126::C0(match v1.clone() {
        Adt126::C0(h1787, _, _, _, _, _, _) => {
            h1787
        }
    }, v2, match v1.clone() {
        Adt126::C0(_, _, h1788, _, _, _, _) => {
            h1788
        }
    }, match v1.clone() {
        Adt126::C0(_, _, _, h1789, _, _, _) => {
            h1789
        }
    }, match v1.clone() {
        Adt126::C0(_, _, _, _, h1790, _, _) => {
            h1790
        }
    }, match v1.clone() {
        Adt126::C0(_, _, _, _, _, h1791, _) => {
            h1791
        }
    }, match v1 {
        Adt126::C0(_, _, _, _, _, _, h1792) => {
            h1792
        }
    })))
}

pub fn f521(v0: Adt123, v1: u64, v2: u64, v3: Adt126, v4: bool) -> R<Adt141> {
    let v5: u64 = {
        let a1794 = match v3.clone() {
            Adt126::C0(_, _, _, h1793, _, _, _) => {
                h1793
            }
        };
        let a1795 = 0u64;
        if if nat_eq(a1794, a1795) {
            let a1796 = v2;
            let a1797 = 48u64;
            nat_eq(a1796, a1797)
        } else {
            false
        } {
            0u64
        } else {
            let a1799 = match v3.clone() {
                Adt126::C0(_, _, _, h1798, _, _, _) => {
                    h1798
                }
            };
            let a1800 = 1u64;
            nat_add(a1799, a1800)?
        }
    };
    let v6: i64 = if v4 {
        f302(match v3.clone() {
            Adt126::C0(_, _, _, _, h1801, _, _) => {
                h1801
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt126::C0(_, _, _, _, h1802, _, _) => {
                h1802
            }
        }
    };
    let a1803 = f595();
    let a1804 = v5;
    if nat_lt(a1803, a1804) {
        Ok(f434(Adt102::C8(v1)))
    } else {
        let m1807 = {
            let a1805 = 99999i64;
            let a1806 = v6;
            compare(a1805, a1806)
        };
        if match m1807 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f434(Adt102::C8(v1)))
        } else {
            Ok(f433(v0, f508(match v3.clone() {
                Adt126::C0(h1808, _, _, _, _, _, _) => {
                    h1808
                }
            }, if v4 {
                Adt127::C4
            } else {
                Adt127::C2
            }, f302({
                let a1810 = match v3.clone() {
                    Adt126::C0(_, _, h1809, _, _, _, _) => {
                        h1809
                    }
                };
                let a1811 = 10i64;
                int_mul(a1810, a1811)?
            }, f509({
                let a1812 = v2;
                let a1813 = 48u64;
                nat_sub(a1812, a1813)
            }))?, v5, v6, match v3.clone() {
                Adt126::C0(_, _, _, _, _, h1814, _) => {
                    h1814
                }
            }, match v3 {
                Adt126::C0(_, _, _, _, _, _, h1815) => {
                    h1815
                }
            })))
        }
    }
}

pub fn f522(v0: u64) -> bool {
    let a1816 = v0;
    let a1817 = 101u64;
    if nat_eq(a1816, a1817) {
        true
    } else {
        let a1818 = v0;
        let a1819 = 69u64;
        nat_eq(a1818, a1819)
    }
}

pub fn f523(v0: u64) -> bool {
    f425(v0, 48u64, 57u64)
}

pub fn f524(v0: Adt123, v1: u64, v2: u64, v3: Adt126) -> R<Adt141> {
    let m1820 = f426(v3)?;
    match m1820 {
        Adt147::C0 => {
            Ok(f434(Adt102::C7(v1)))
        }
        Adt147::C1 => {
            Ok(f434(Adt102::C8(v1)))
        }
        Adt147::C2(v4) => {
            Ok(f431(f427(v4, match v0.clone() {
                Adt123::C0(_, h1821, _) => {
                    h1821
                }
            }, match v0 {
                Adt123::C0(_, _, h1822) => {
                    h1822
                }
            }), v1, v2))
        }
    }
}

pub fn f525(v0: Adt123, v1: u64, v2: u64, v3: Adt126, v4: bool) -> R<Adt141> {
    let v5: i64 = f302({
        let a1824 = match v3.clone() {
            Adt126::C0(_, _, _, _, _, _, h1823) => {
                h1823
            }
        };
        let a1825 = 10i64;
        int_mul(a1824, a1825)?
    }, f509({
        let a1826 = v2;
        let a1827 = 48u64;
        nat_sub(a1826, a1827)
    }))?;
    let m1830 = {
        let a1828 = 99999i64;
        let a1829 = v5;
        compare(a1828, a1829)
    };
    if match m1830 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f434(Adt102::C8(v1)))
    } else {
        Ok(f433(v0, f508(match v3.clone() {
            Adt126::C0(h1831, _, _, _, _, _, _) => {
                h1831
            }
        }, Adt127::C7, match v3.clone() {
            Adt126::C0(_, _, h1832, _, _, _, _) => {
                h1832
            }
        }, match v3.clone() {
            Adt126::C0(_, _, _, h1833, _, _, _) => {
                h1833
            }
        }, match v3 {
            Adt126::C0(_, _, _, _, h1834, _, _) => {
                h1834
            }
        }, v4, v5)))
    }
}

pub fn f526(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1835 = f596(f597(), v2.clone());
    match m1835 {
        None => {
            let m1836 = f598(f599(), v2.clone());
            match m1836 {
                None => {
                    f600(v0, v1, v2)
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

pub fn f527(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt129) -> R<Adt152> {
    let m1837 = f296(Str::lit("x-mcp-header"), v1.clone());
    match m1837 {
        Adt107::C0 => {
            Ok(Adt152::C1(v4))
        }
        Adt107::C1(v5) => {
            f601(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f528(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt109>) -> List<Adt109> {
    f602(Str::lit("properties"), v0.clone(), v1, v2, f602(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f603(Str::lit("items"), v0.clone(), f603(Str::lit("additionalProperties"), v0.clone(), f604(Str::lit("allOf"), v0.clone(), f604(Str::lit("anyOf"), v0.clone(), f604(Str::lit("oneOf"), v0.clone(), f603(Str::lit("not"), v0.clone(), f603(Str::lit("if"), v0.clone(), f603(Str::lit("then"), v0.clone(), f603(Str::lit("else"), v0, v3)))))))))))
}

pub fn f529(v0: List<Adt51>) -> R<bool> {
    let m1838 = v0;
    match m1838.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f605(v2, match v1 {
                Adt51::C0(h1839, _, _) => {
                    h1839
                }
            })
        }
    }
}

pub fn f530(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    Ok(f606(List::cons((Str::lit("type"), f607(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f608(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f609(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f610(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f610(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f610(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f610(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1840 = f611(v0.clone(), v1.clone())?;
        let a1841 = List::cons((Str::lit("required"), f612(v0, v1)), List::<(Str, Adt153)>::nil());
        append_list(a1840, a1841)
    })))))))))
}

pub fn f531(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt111>) -> R<Adt142> {
    Ok(f613(f614(v0, v1.clone(), v2.clone(), v3.clone()), f613(f615(v1.clone(), v2.clone(), v3.clone()), f613(f616(v1.clone(), v2.clone(), v3.clone()), f613(f617(v1.clone(), v2.clone(), v3.clone())?, f613(f618(v1.clone(), v2.clone(), v3.clone()), f613(f619(Adt114::C0, f454(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f613(f619(Adt114::C1, f455(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f613(f619(Adt114::C2, f456(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f619(Adt114::C3, f457(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f532(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1842 = f296(v0, v1);
    match m1842 {
        Adt107::C0 => {
            None::<Adt6>
        }
        Adt107::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f533(v0: Str) -> R<Str> {
    let m1845 = {
        let a1844 = f387(f179(), f335(f620(f177({
            let a1843 = v0.clone();
            utf8_encode(a1843)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1844)
    };
    match m1845 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f534(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f621(v0, 0u64, v1)
}

pub fn f535(v0: Adt40) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt40::C0(h1846, _, _) => {
            h1846
        }
    })), {
        let a1849 = f622(match v0.clone() {
            Adt40::C0(_, h1847, _) => {
                h1847
            }
        });
        let a1850 = f623(match v0 {
            Adt40::C0(_, _, h1848) => {
                h1848
            }
        });
        append_list(a1849, a1850)
    })))
}

pub fn f536(v0: Str, v1: Str, v2: Str, v3: Option<Adt41>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a1851 = f622(v3);
        let a1852 = f623(v4);
        append_list(a1851, a1852)
    }))))
}

pub fn f537(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a1876 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h1853, _, _, _, _, _, _, _, _) => {
                h1853
            }
        })), List::<(Str, Adt6)>::nil()));
        let a1877 = {
            let a1874 = f624(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h1854, _, _, _, _, _, _, _) => {
                    h1854
                }
            });
            let a1875 = {
                let a1872 = f625(match v0.clone() {
                    Adt45::C0(_, _, h1855, _, _, _, _, _, _) => {
                        h1855
                    }
                });
                let a1873 = {
                    let a1870 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h1856, _, _, _, _, _) => {
                            h1856
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a1871 = {
                        let a1868 = f624(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h1857, _, _, _, _) => {
                                h1857
                            }
                        });
                        let a1869 = {
                            let a1866 = f624(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h1858, _, _, _) => {
                                    h1858
                                }
                            });
                            let a1867 = {
                                let a1864 = f622(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h1859, _, _) => {
                                        h1859
                                    }
                                });
                                let a1865 = {
                                    let a1862 = f626(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h1860, _) => {
                                            h1860
                                        }
                                    })?;
                                    let a1863 = f623(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h1861) => {
                                            h1861
                                        }
                                    });
                                    append_list(a1862, a1863)
                                };
                                append_list(a1864, a1865)
                            };
                            append_list(a1866, a1867)
                        };
                        append_list(a1868, a1869)
                    };
                    append_list(a1870, a1871)
                };
                append_list(a1872, a1873)
            };
            append_list(a1874, a1875)
        };
        append_list(a1876, a1877)
    }))
}

pub fn f538(v0: Adt46) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f627(match v0.clone() {
        Adt46::C0(h1878, _, _) => {
            h1878
        }
    })), {
        let a1881 = f622(match v0.clone() {
            Adt46::C0(_, h1879, _) => {
                h1879
            }
        });
        let a1882 = f623(match v0 {
            Adt46::C0(_, _, h1880) => {
                h1880
            }
        });
        append_list(a1881, a1882)
    })))
}

pub fn f539(p1888: List<(Str, Adt6)>, p1889: List<(Str, Adt65)>) -> Option<List<(Str, Adt65)>> {
    let m1887 = (p1888, p1889);
    {
        let mut m1887 = m1887;
        loop {
            let step = {
                let (v0, v1) = m1887;
                let m1883 = v0;
                match m1883.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(Some(f384(v1, List::<(Str, Adt65)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1885 = f628({
                            let (_, h1884) = v2;
                            h1884
                        });
                        match m1885 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(None::<List<(Str, Adt65)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>((v3, List::cons(({
                                    let (h1886, _) = v2;
                                    h1886
                                }, v4), v1)))
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

pub fn f540(p1898: Bytes, p1899: List<Bytes>) -> List<Bytes> {
    let m1897 = (p1898, p1899);
    {
        let mut m1897 = m1897;
        loop {
            let step = {
                let (v0, v1) = m1897;
                let m1890 = v1;
                match m1890.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1893 = {
                            let a1891 = v2.clone();
                            length_bytes(a1891)
                        };
                        let a1894 = {
                            let a1892 = v0.clone();
                            length_bytes(a1892)
                        };
                        if nat_eq(a1893, a1894) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1895 = v2;
                                let a1896 = v0;
                                append_bytes(a1895, a1896)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1897 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f541(v0: u64) -> Bytes {
    let m1903 = {
        let a1900 = f629();
        let a1901 = v0;
        let a1902 = 1u64;
        slice_bytes(a1900, a1901, a1902)
    };
    match m1903 {
        None => {
            f179()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f542(v0: u64) -> List<u64> {
    f517(v0)
}

pub fn f543(v0: Adt7) -> R<bool> {
    let a1921 = {
        let m1907 = {
            let a1905 = f630();
            let a1906 = match v0.clone() {
                Adt7::C0(h1904, _) => {
                    h1904
                }
            };
            compare(a1905, a1906)
        };
        if match m1907 {
            Ordering::Less => {
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
            let m1912 = {
                let a1910 = match v0.clone() {
                    Adt7::C0(h1908, _) => {
                        h1908
                    }
                };
                let a1911 = {
                    let a1909 = f630();
                    int_neg(a1909)?
                };
                compare(a1910, a1911)
            };
            if match m1912 {
                Ordering::Less => {
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
                let m1916 = {
                    let a1914 = 9999i64;
                    let a1915 = match v0.clone() {
                        Adt7::C0(_, h1913) => {
                            h1913
                        }
                    };
                    compare(a1914, a1915)
                };
                if match m1916 {
                    Ordering::Less => {
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
                    let m1920 = {
                        let a1918 = match v0 {
                            Adt7::C0(_, h1917) => {
                                h1917
                            }
                        };
                        let a1919 = -9999i64;
                        compare(a1918, a1919)
                    };
                    match m1920 {
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
    Ok(bool_not(a1921))
}

pub fn f544(v0: Adt7) -> R<List<u64>> {
    let m1925 = {
        let a1923 = match v0.clone() {
            Adt7::C0(_, h1922) => {
                h1922
            }
        };
        let a1924 = 0i64;
        compare(a1923, a1924)
    };
    if if match m1925 {
        Ordering::Less => {
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
        let m1929 = {
            let a1927 = 0i64;
            let a1928 = match v0.clone() {
                Adt7::C0(_, h1926) => {
                    h1926
                }
            };
            compare(a1927, a1928)
        };
        match m1929 {
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
        let a1932 = f631(match v0.clone() {
            Adt7::C0(h1930, _) => {
                h1930
            }
        })?;
        let a1933 = List::cons(101u64, f631(match v0 {
            Adt7::C0(_, h1931) => {
                h1931
            }
        })?);
        Ok(append_list(a1932, a1933))
    } else {
        f631(match v0 {
            Adt7::C0(h1934, _) => {
                h1934
            }
        })
    }
}

pub fn f545() -> u64 {
    512u64
}

pub fn f546(v0: u64, v1: Adt115, v2: List<Adt115>) -> List<Adt115> {
    List::cons(Adt115::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f547(p1937: List<u64>, p1938: List<u64>) -> R<List<u64>> {
    let m1936 = (p1937, p1938);
    {
        let mut m1936 = m1936;
        loop {
            let step = {
                let (v0, v1) = m1936;
                let m1935 = v0;
                match m1935.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f335(f632(v2)?, v1)))
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

pub fn f548(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f633(match v0.clone() {
        Adt34::C0(_, h1939, _, _) => {
            h1939
        }
    })), List::cons((Str::lit("capabilities"), f634(match v0.clone() {
        Adt34::C0(_, _, h1940, _) => {
            h1940
        }
    })), {
        let a1943 = f624(Str::lit("instructions"), match v0.clone() {
            Adt34::C0(_, _, _, h1941) => {
                h1941
            }
        });
        let a1944 = f635(match v0 {
            Adt34::C0(h1942, _, _, _) => {
                h1942
            }
        })?;
        append_list(a1943, a1944)
    })))
}

pub fn f549(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f636(match v0.clone() {
        Adt35::C0(_, _, h1945) => {
            h1945
        }
    }))), {
        let a1948 = f624(Str::lit("nextCursor"), match v0.clone() {
            Adt35::C0(_, h1946, _) => {
                h1946
            }
        });
        let a1949 = f635(match v0 {
            Adt35::C0(h1947, _, _) => {
                h1947
            }
        })?;
        append_list(a1948, a1949)
    }))
}

pub fn f550(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f637(match v0.clone() {
        Adt52::C0(_, _, h1950) => {
            h1950
        }
    })?)), {
        let a1953 = f624(Str::lit("nextCursor"), match v0.clone() {
            Adt52::C0(_, h1951, _) => {
                h1951
            }
        });
        let a1954 = f635(match v0 {
            Adt52::C0(h1952, _, _) => {
                h1952
            }
        })?;
        append_list(a1953, a1954)
    }))
}

pub fn f551(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f638(match v0.clone() {
        Adt53::C0(_, h1955) => {
            h1955
        }
    }))), f635(match v0 {
        Adt53::C0(h1956, _) => {
            h1956
        }
    })?))
}

pub fn f552(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f639(match v0.clone() {
        Adt54::C0(_, _, h1957) => {
            h1957
        }
    }))), {
        let a1960 = f624(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h1958, _) => {
                h1958
            }
        });
        let a1961 = f635(match v0 {
            Adt54::C0(h1959, _, _) => {
                h1959
            }
        })?;
        append_list(a1960, a1961)
    }))
}

pub fn f553(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f640(match v0.clone() {
        Adt56::C0(_, _, h1962) => {
            h1962
        }
    }))), {
        let a1965 = f624(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h1963, _) => {
                h1963
            }
        });
        let a1966 = f635(match v0 {
            Adt56::C0(h1964, _, _) => {
                h1964
            }
        })?;
        append_list(a1965, a1966)
    }))
}

pub fn f554(v0: Adt59) -> R<Option<List<(Str, Adt6)>>> {
    let m1968 = f641(match v0.clone() {
        Adt59::C0(_, h1967) => {
            h1967
        }
    })?;
    match m1968 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1970 = f624(Str::lit("description"), match v0 {
                    Adt59::C0(h1969, _) => {
                        h1969
                    }
                });
                let a1971 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1970, a1971)
            }))
        }
    }
}

pub fn f555(v0: Adt61) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f577({
        let m1974 = match v0.clone() {
            Adt61::C0(h1973) => {
                h1973
            }
        };
        match m1974 {
            Adt62::C0(h1972, _, _) => {
                h1972
            }
        }
    }))), {
        let a1981 = f642(Str::lit("total"), {
            let m1977 = match v0.clone() {
                Adt61::C0(h1976) => {
                    h1976
                }
            };
            match m1977 {
                Adt62::C0(_, h1975, _) => {
                    h1975
                }
            }
        })?;
        let a1982 = f301(Str::lit("hasMore"), {
            let m1980 = match v0 {
                Adt61::C0(h1979) => {
                    h1979
                }
            };
            match m1980 {
                Adt62::C0(_, _, h1978) => {
                    h1978
                }
            }
        });
        append_list(a1981, a1982)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f556(v0: Adt82) -> Option<List<(Str, Adt6)>> {
    let m1983 = f643(f644(v0.clone()));
    match m1983 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1985 = {
                    let m1984 = f644(v0.clone());
                    match m1984.uncons() {
                        None => {
                            List::<(Str, Adt6)>::nil()
                        }
                        Some((_, _)) => {
                            List::cons((Str::lit("inputRequests"), v1), List::<(Str, Adt6)>::nil())
                        }
                    }
                };
                let a1986 = f645(Str::lit("requestState"), f646(v0));
                append_list(a1985, a1986)
            })
        }
    }
}

pub fn f557(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1987 = f647(v0.clone());
    match m1987 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1988 = f648(f306(v0.clone()), f649(f650(v0)));
                let a1989 = v1;
                append_list(a1988, a1989)
            })
        }
    }
}

pub fn f558(v0: Adt88) -> Option<List<(Str, Adt6)>> {
    let m1991 = match v0.clone() {
        Adt88::C0(_, h1990, _) => {
            h1990
        }
    };
    match m1991 {
        Adt89::C0 => {
            f557({
                let m1994 = match v0 {
                    Adt88::C0(h1993, _, _) => {
                        h1993
                    }
                };
                match m1994 {
                    Adt85::C0(h1992, _, _) => {
                        h1992
                    }
                }
            })
        }
        Adt89::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f559(v0: Adt154) -> Str {
    let m1995 = v0;
    match m1995 {
        Adt154::C0 => {
            Str::lit("complete")
        }
        Adt154::C1 => {
            Str::lit("input_required")
        }
        Adt154::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f560(v0: Adt33) -> Adt154 {
    let m1996 = v0;
    match m1996 {
        Adt33::C0(_) => {
            Adt154::C0
        }
        Adt33::C1(_) => {
            Adt154::C0
        }
        Adt33::C2(_) => {
            Adt154::C0
        }
        Adt33::C3(_) => {
            Adt154::C0
        }
        Adt33::C4(_) => {
            Adt154::C0
        }
        Adt33::C5(_) => {
            Adt154::C0
        }
        Adt33::C6(_) => {
            Adt154::C0
        }
        Adt33::C7(_) => {
            Adt154::C0
        }
        Adt33::C8(_) => {
            Adt154::C0
        }
        Adt33::C9(_) => {
            Adt154::C0
        }
        Adt33::C10(_) => {
            Adt154::C0
        }
        Adt33::C11 => {
            Adt154::C0
        }
        Adt33::C12(_) => {
            Adt154::C1
        }
        Adt33::C13(_) => {
            Adt154::C2
        }
        Adt33::C14(_) => {
            Adt154::C0
        }
    }
}

pub fn f561(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1997 = v0;
    match m1997 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f651(), f652(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f562(v0: List<Adt85>) -> R<Option<List<u64>>> {
    let m1998 = f653(v0);
    match m1998 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1999 = f181(v1)?;
            match m1999 {
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

pub fn f563(v0: Adt85, v1: List<Adt85>, v2: i64) -> R<List<Adt85>> {
    Ok(List::cons(v0.clone(), f304(f654(f225(v0), v1, v2, List::<Adt85>::nil())?, List::<Adt85>::nil())))
}

pub fn f564(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f655(v2) {
            let a2000 = v1;
            let a2001 = 200u64;
            nat_eq(a2000, a2001)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f565(v0: u64, v1: List<List<u64>>) -> Adt94 {
    Adt94::C0(v0, f656(), List::<List<u64>>::nil(), f657(v1), None::<List<u64>>)
}

pub fn f566(v0: Adt136) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f259())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f658(match v0.clone() {
        Adt136::C0(h2002, _) => {
            h2002
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m2005 = match v0.clone() {
            Adt136::C0(_, h2004) => {
                h2004
            }
        };
        match m2005 {
            Adt51::C0(h2003, _, _) => {
                h2003
            }
        }
    })), {
        let a2012 = f659({
            let m2008 = match v0.clone() {
                Adt136::C0(_, h2007) => {
                    h2007
                }
            };
            match m2008 {
                Adt51::C0(_, h2006, _) => {
                    h2006
                }
            }
        });
        let a2013 = f660({
            let m2011 = match v0 {
                Adt136::C0(_, h2010) => {
                    h2010
                }
            };
            match m2011 {
                Adt51::C0(_, _, h2009) => {
                    h2009
                }
            }
        });
        append_list(a2012, a2013)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f567(v0: Adt88) -> bool {
    if match v0.clone() {
        Adt88::C0(_, _, h2014) => {
            h2014
        }
    } {
        let m2016 = match v0 {
            Adt88::C0(_, h2015, _) => {
                h2015
            }
        };
        match m2016 {
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

pub fn f568(v0: Adt63) -> Option<Adt6> {
    let m2017 = f557(v0);
    match m2017 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f569() -> u64 {
    429u64
}

pub fn f570() -> i64 {
    -32602i64
}

pub fn f571() -> i64 {
    -32022i64
}

pub fn f572() -> i64 {
    -32020i64
}

pub fn f573() -> i64 {
    -32601i64
}

pub fn f574(v0: Adt90) -> i64 {
    let m2018 = v0;
    match m2018 {
        Adt90::C0(_) => {
            f570()
        }
        Adt90::C1(_) => {
            f570()
        }
        Adt90::C2(_) => {
            f570()
        }
        Adt90::C3(_) => {
            f570()
        }
        Adt90::C4(_) => {
            f570()
        }
        Adt90::C5(_) => {
            f570()
        }
        Adt90::C6(_) => {
            f661()
        }
        Adt90::C7(_) => {
            f662()
        }
        Adt90::C8(_) => {
            f663()
        }
        Adt90::C9(_) => {
            f661()
        }
    }
}

pub fn f575(v0: Adt138) -> Str {
    let m2019 = v0;
    match m2019 {
        Adt138::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt138::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt138::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt138::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt138::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt138::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt138::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt138::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt138::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f576(v0: Adt117) -> Str {
    let m2020 = v0;
    match m2020 {
        Adt117::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt117::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt117::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt117::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt117::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt117::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt117::C6(v1) => {
            let m2021 = v1;
            match m2021 {
                Adt118::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt118::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt118::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt117::C7(v2) => {
            let m2022 = v2;
            match m2022 {
                Adt118::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt118::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt118::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt117::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt117::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt117::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f577(v0: List<Str>) -> List<Adt6> {
    f378(f664(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f578(v0: Adt18) -> Adt6 {
    Adt6::C5({
        let a2034 = f665(Str::lit("experimental"), match v0.clone() {
            Adt18::C0(h2023, _, _, _, _) => {
                h2023
            }
        });
        let a2035 = {
            let a2032 = f665(Str::lit("roots"), match v0.clone() {
                Adt18::C0(_, h2024, _, _, _) => {
                    h2024
                }
            });
            let a2033 = {
                let a2030 = f665(Str::lit("sampling"), match v0.clone() {
                    Adt18::C0(_, _, h2025, _, _) => {
                        h2025
                    }
                });
                let a2031 = {
                    let a2028 = f665(Str::lit("elicitation"), match v0.clone() {
                        Adt18::C0(_, _, _, h2026, _) => {
                            h2026
                        }
                    });
                    let a2029 = f665(Str::lit("extensions"), match v0 {
                        Adt18::C0(_, _, _, _, h2027) => {
                            h2027
                        }
                    });
                    append_list(a2028, a2029)
                };
                append_list(a2030, a2031)
            };
            append_list(a2032, a2033)
        };
        append_list(a2034, a2035)
    })
}

pub fn f579(p2038: List<List<u64>>) -> R<bool> {
    let m2037 = p2038;
    {
        let mut m2037 = m2037;
        loop {
            let step = {
                let v0 = m2037;
                let m2036 = v0;
                match m2036.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f666(f274(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f581(v0: Adt0, v1: Fn3, v2: u64, v3: Adt145, v4: bool) -> R<Result<Adt134, Adt137>> {
    if v4 {
        f667(v0, v1, v2, f668(match v3.clone() {
            Adt145::C0(_, h2039, _, _) => {
                h2039
            }
        }, match v3.clone() {
            Adt145::C0(_, _, h2040, _) => {
                h2040
            }
        }, match v3 {
            Adt145::C0(_, _, _, h2041) => {
                h2041
            }
        }))
    } else {
        Ok(Err::<Adt134, Adt137>(Adt137::C3(f669(match v3 {
            Adt145::C0(_, h2042, _, _) => {
                h2042
            }
        }))))
    }
}

pub fn f582(v0: Adt4, v1: Adt146) -> bool {
    let m2043 = v1;
    match m2043 {
        Adt146::C0 => {
            true
        }
        Adt146::C1 => {
            f670(match v0 {
                Adt4::C0(_, _, _, _, _, h2044, _) => {
                    h2044
                }
            })
        }
        Adt146::C2 => {
            f670(match v0 {
                Adt4::C0(_, _, _, _, _, h2045, _) => {
                    h2045
                }
            })
        }
        Adt146::C3 => {
            f671(match v0 {
                Adt4::C0(_, _, _, _, h2046, _, _) => {
                    h2046
                }
            })
        }
        Adt146::C4 => {
            f671(match v0 {
                Adt4::C0(_, _, _, _, h2047, _, _) => {
                    h2047
                }
            })
        }
        Adt146::C5 => {
            f671(match v0 {
                Adt4::C0(_, _, _, _, h2048, _, _) => {
                    h2048
                }
            })
        }
        Adt146::C6 => {
            f670(match v0 {
                Adt4::C0(_, _, _, h2049, _, _, _) => {
                    h2049
                }
            })
        }
        Adt146::C7 => {
            f670(match v0 {
                Adt4::C0(_, _, _, h2050, _, _, _) => {
                    h2050
                }
            })
        }
        Adt146::C8 => {
            f672(match v0 {
                Adt4::C0(_, _, h2051, _, _, _, _) => {
                    h2051
                }
            })
        }
        Adt146::C9 => {
            f222(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2052) => {
                    h2052
                }
            }, f28())
        }
        Adt146::C10 => {
            f222(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2053) => {
                    h2053
                }
            }, f28())
        }
        Adt146::C11 => {
            f222(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2054) => {
                    h2054
                }
            }, f28())
        }
    }
}

pub fn f583(v0: Adt150, v1: Adt148) -> Result<Adt145, Adt137> {
    let m2055 = v1;
    match m2055 {
        Adt148::C0 => {
            Err::<Adt145, Adt137>(Adt137::C3(match v0 {
                Adt150::C0(_, h2056, _, _) => {
                    h2056
                }
            }))
        }
        Adt148::C1(v2) => {
            Ok::<Adt145, Adt137>(Adt145::C0(match v0.clone() {
                Adt150::C0(h2057, _, _, _) => {
                    h2057
                }
            }, v2, match v0.clone() {
                Adt150::C0(_, _, h2058, _) => {
                    h2058
                }
            }, match v0 {
                Adt150::C0(_, _, _, h2059) => {
                    h2059
                }
            }))
        }
    }
}

pub fn f584(v0: Option<Adt116>, v1: List<Str>, v2: Adt108, v3: Result<Adt17, Adt138>) -> R<Adt149> {
    let m2060 = v3;
    match m2060 {
        Ok(v5) => {
            Ok(f673(v1, v2.clone(), v5.clone(), f674(v0, f498(match v2.clone() {
                Adt108::C0(_, h2061, _) => {
                    h2061
                }
            }), match v2.clone() {
                Adt108::C0(_, h2062, _) => {
                    h2062
                }
            }, match v5 {
                Adt17::C0(h2063, _, _, _, _, _) => {
                    h2063
                }
            }, f675(match v2 {
                Adt108::C0(_, _, h2064) => {
                    h2064
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt149::C1(Adt137::C0(v4)))
        }
    }
}

pub fn f585(v0: Option<Adt5>) -> Result<Adt17, Adt138> {
    let m2065 = v0;
    match m2065 {
        None => {
            Err::<Adt17, Adt138>(Adt138::C0)
        }
        Some(v1) => {
            let m2066 = f214(Str::lit("_meta"), v1);
            match m2066 {
                Adt107::C0 => {
                    Err::<Adt17, Adt138>(Adt138::C1)
                }
                Adt107::C1(v2) => {
                    let m2067 = f219(v2);
                    match m2067 {
                        None => {
                            Err::<Adt17, Adt138>(Adt138::C4)
                        }
                        Some(v3) => {
                            f676(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f586(v0: Adt146) -> Adt151 {
    let m2068 = v0;
    match m2068 {
        Adt146::C0 => {
            Adt151::C0
        }
        Adt146::C1 => {
            Adt151::C0
        }
        Adt146::C2 => {
            Adt151::C1
        }
        Adt146::C3 => {
            Adt151::C0
        }
        Adt146::C4 => {
            Adt151::C2
        }
        Adt146::C5 => {
            Adt151::C0
        }
        Adt146::C6 => {
            Adt151::C0
        }
        Adt146::C7 => {
            Adt151::C1
        }
        Adt146::C8 => {
            Adt151::C0
        }
        Adt146::C9 => {
            Adt151::C3
        }
        Adt146::C10 => {
            Adt151::C3
        }
        Adt146::C11 => {
            Adt151::C3
        }
    }
}

pub fn f587(p2074: List<u64>, p2075: List<u64>) -> Option<List<u64>> {
    let m2073 = (p2074, p2075);
    {
        let mut m2073 = m2073;
        loop {
            let step = {
                let (v0, v1) = m2073;
                let m2069 = v0;
                match m2069.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2070 = v1;
                        match m2070.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2071 = v4;
                                let a2072 = v2;
                                if nat_eq(a2071, a2072) {
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
                Ok(next) => { m2073 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f589() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f590(p2084: u64, p2085: List<u64>, p2086: List<u64>) -> R<Option<List<u64>>> {
    let m2083 = (p2084, (p2085, p2086));
    {
        let mut m2083 = m2083;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2083;
                let m2076 = v0;
                if m2076 == 0 {
                    let m2077 = v1;
                    match m2077.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m2076 - 1;
                    let m2078 = v1;
                    match m2078.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m2079 = v7;
                            match m2079.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m2080 = v9;
                                    match m2080.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m2081 = v11;
                                            match m2081.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m2082 = f677(v6, v8, v10, v12, f678(v13.clone()))?;
                                                    match m2082 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f335(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m2083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(v0: Adt12, v1: Option<Str>) -> bool {
    let m2087 = v1;
    match m2087 {
        None => {
            false
        }
        Some(v2) => {
            let a2089 = match v0 {
                Adt12::C0(h2088, _, _, _) => {
                    h2088
                }
            };
            let a2090 = v2;
            equal(a2089, a2090)
        }
    }
}

pub fn f592(v0: List<Adt92>, v1: Str) -> R<Adt120> {
    let m2093 = f416(f122(v0, {
        let a2091 = f679();
        let a2092 = f680(v1)?;
        append_list(a2091, a2092)
    })?)?;
    match m2093 {
        Adt105::C0 => {
            Ok(Adt120::C0)
        }
        Adt105::C1 => {
            Ok(Adt120::C1)
        }
        Adt105::C2 => {
            Ok(Adt120::C1)
        }
        Adt105::C3(v2) => {
            let m2094 = f393(v2);
            match m2094 {
                None => {
                    Ok(Adt120::C1)
                }
                Some(v3) => {
                    Ok(Adt120::C2(v3))
                }
            }
        }
    }
}

pub fn f593(v0: Adt123, v1: u64, v2: Str) -> Adt141 {
    let m2096 = match v0.clone() {
        Adt123::C0(_, h2095, _) => {
            h2095
        }
    };
    match m2096.uncons() {
        None => {
            f434(Adt102::C2(v1))
        }
        Some((v3, v4)) => {
            let m2097 = v3;
            match m2097 {
                Adt128::C0(_) => {
                    f434(Adt102::C2(v1))
                }
                Adt128::C1(v6, _) => {
                    Adt141::C1(Adt123::C0(Adt124::C5, List::cons(Adt128::C1(v6, Some(v2)), v4), match v0 {
                        Adt123::C0(_, _, h2098) => {
                            h2098
                        }
                    }))
                }
            }
        }
    }
}

pub fn f594(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2099 = v0;
    let a2100 = 128u64;
    if nat_lt(a2099, a2100) {
        Ok(List::cons(v0, v1))
    } else {
        let a2101 = v0;
        let a2102 = 2048u64;
        if nat_lt(a2101, a2102) {
            Ok(List::cons({
                let a2103 = 128u64;
                let a2104 = f681(v0, 64u64);
                nat_add(a2103, a2104)?
            }, List::cons({
                let a2105 = 192u64;
                let a2106 = f682(v0, 64u64);
                nat_add(a2105, a2106)?
            }, v1)))
        } else {
            let a2107 = v0;
            let a2108 = 65536u64;
            if nat_lt(a2107, a2108) {
                Ok(List::cons({
                    let a2109 = 128u64;
                    let a2110 = f681(v0, 64u64);
                    nat_add(a2109, a2110)?
                }, List::cons({
                    let a2111 = 128u64;
                    let a2112 = f681(f682(v0, 64u64), 64u64);
                    nat_add(a2111, a2112)?
                }, List::cons({
                    let a2113 = 224u64;
                    let a2114 = f682(v0, 4096u64);
                    nat_add(a2113, a2114)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2115 = 128u64;
                    let a2116 = f681(v0, 64u64);
                    nat_add(a2115, a2116)?
                }, List::cons({
                    let a2117 = 128u64;
                    let a2118 = f681(f682(v0, 64u64), 64u64);
                    nat_add(a2117, a2118)?
                }, List::cons({
                    let a2119 = 128u64;
                    let a2120 = f681(f682(v0, 4096u64), 64u64);
                    nat_add(a2119, a2120)?
                }, List::cons({
                    let a2121 = 240u64;
                    let a2122 = f682(v0, 262144u64);
                    nat_add(a2121, a2122)?
                }, v1)))))
            }
        }
    }
}

pub fn f595() -> u64 {
    18u64
}

pub fn f596(p2126: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2125 = p2126;
    {
        let mut m2125 = m2125;
        loop {
            let step = {
                let v0 = m2125;
                let m2123 = v0;
                match m2123.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2124 = f296(v2.clone(), v1.clone());
                        match m2124 {
                            Adt107::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt107::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f598(p2133: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2132 = p2133;
    {
        let mut m2132 = m2132;
        loop {
            let step = {
                let v0 = m2132;
                let m2127 = v0;
                match m2127.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2129 = f296({
                            let (h2128, _) = v2.clone();
                            h2128
                        }, v1.clone());
                        match m2129 {
                            Adt107::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt107::C1(v4) => {
                                if f683({
                                    let (_, h2130) = v2;
                                    h2130
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h2131, _) = v2;
                                        h2131
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f599() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f600(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt97> {
    let m2134 = f296(Str::lit("$ref"), v2);
    match m2134 {
        Adt107::C0 => {
            None::<Adt97>
        }
        Adt107::C1(v3) => {
            let m2135 = f141(v3);
            match m2135 {
                None => {
                    None::<Adt97>
                }
                Some(v4) => {
                    let m2136 = f684(v1, v4.clone());
                    match m2136 {
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

pub fn f601(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt129) -> R<Adt152> {
    if v3 {
        let m2137 = f141(v2);
        match m2137 {
            None => {
                Ok(Adt152::C0(Adt97::C7(v0)))
            }
            Some(v6) => {
                f685(v0, v1, v6.clone(), f205(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt152::C0(Adt97::C10(v0)))
    }
}

pub fn f602(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt109>) -> List<Adt109> {
    let m2138 = f296(v0.clone(), v1);
    match m2138 {
        Adt107::C0 => {
            v4
        }
        Adt107::C1(v5) => {
            let m2139 = f219(v5);
            match m2139 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a2140 = v0;
                    let a2141 = Str::lit("properties");
                    if equal(a2140, a2141) {
                        f686(match v6 {
                            Adt5::C0(h2142) => {
                                h2142
                            }
                        }, v2, v3, v4)
                    } else {
                        f687(match v6 {
                            Adt5::C0(h2143) => {
                                h2143
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f603(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt109>) -> List<Adt109> {
    let m2144 = f296(v0, v1);
    match m2144 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            List::cons(Adt109::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f604(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt109>) -> List<Adt109> {
    let m2145 = f296(v0, v1);
    match m2145 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            let m2146 = f223(v3);
            match m2146 {
                None => {
                    v2
                }
                Some(v4) => {
                    f688(v4, v2)
                }
            }
        }
    }
}

pub fn f605(p2151: List<Adt51>, p2152: Adt7) -> R<bool> {
    let m2150 = (p2151, p2152);
    {
        let mut m2150 = m2150;
        loop {
            let step = {
                let (v0, v1) = m2150;
                let m2147 = v0;
                match m2147.uncons() {
                    None => {
                        Err::<(List<Adt51>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f689(f690(match v2.clone() {
                            Adt51::C0(h2148, _, _) => {
                                h2148
                            }
                        }, v1)?) {
                            Ok::<(List<Adt51>, Adt7), R<bool>>((v3, match v2 {
                                Adt51::C0(h2149, _, _) => {
                                    h2149
                                }
                            }))
                        } else {
                            Err::<(List<Adt51>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606(p2159: List<(Str, Adt153)>) -> Adt142 {
    let m2158 = p2159;
    {
        let mut m2158 = m2158;
        loop {
            let step = {
                let v0 = m2158;
                let m2153 = v0;
                match m2153.uncons() {
                    None => {
                        Err::<List<(Str, Adt153)>, Adt142>(Adt142::C3(List::<Adt113>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m2155 = {
                            let (_, h2154) = v1;
                            h2154
                        };
                        match m2155 {
                            Adt153::C0 => {
                                Ok::<List<(Str, Adt153)>, Adt142>(v2)
                            }
                            Adt153::C1 => {
                                Err::<List<(Str, Adt153)>, Adt142>(Adt142::C0({
                                    let (h2156, _) = v1;
                                    h2156
                                }))
                            }
                            Adt153::C2 => {
                                Err::<List<(Str, Adt153)>, Adt142>(Adt142::C1({
                                    let (h2157, _) = v1;
                                    h2157
                                }))
                            }
                        }
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

pub fn f607(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt153> {
    let m2160 = f296(Str::lit("type"), v0);
    match m2160 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v2) => {
            let m2161 = f141(v2.clone());
            match m2161 {
                None => {
                    let m2162 = f223(v2);
                    match m2162 {
                        None => {
                            Ok(Adt153::C2)
                        }
                        Some(v4) => {
                            f693(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f691(v3.clone()) {
                        if f692(v3, v1)? {
                            Ok(Adt153::C0)
                        } else {
                            Ok(Adt153::C1)
                        }
                    } else {
                        Ok(Adt153::C2)
                    }
                }
            }
        }
    }
}

pub fn f608(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt153> {
    let m2163 = f296(Str::lit("enum"), v0);
    match m2163 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v2) => {
            let m2164 = f223(v2);
            match m2164 {
                None => {
                    Ok(Adt153::C2)
                }
                Some(v3) => {
                    f694(v3, v1)
                }
            }
        }
    }
}

pub fn f609(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt153> {
    let m2165 = f296(Str::lit("const"), v0);
    match m2165 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v2) => {
            let m2166 = f695(v2, v1)?;
            match m2166 {
                Adt157::C0 => {
                    Ok(Adt153::C0)
                }
                Adt157::C1 => {
                    Ok(Adt153::C1)
                }
                Adt157::C2 => {
                    Ok(Adt153::C2)
                }
            }
        }
    }
}

pub fn f610(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt153> {
    let m2167 = f296(v0, v2);
    match m2167 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v4) => {
            let m2168 = f696(v4);
            match m2168 {
                None => {
                    Ok(Adt153::C2)
                }
                Some(v5) => {
                    let m2169 = f696(v3);
                    match m2169 {
                        None => {
                            Ok(Adt153::C0)
                        }
                        Some(v6) => {
                            if f697(v1, f690(v6, v5)?) {
                                Ok(Adt153::C0)
                            } else {
                                Ok(Adt153::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f611(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt153)>> {
    let m2170 = v1;
    match m2170 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt153)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt153)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt153)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f698(Str::lit("minLength"), true, v0.clone(), f699(v4.clone())?)?), List::cons((Str::lit("maxLength"), f698(Str::lit("maxLength"), false, v0, f699(v4)?)?), List::<(Str, Adt153)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f698(Str::lit("minItems"), true, v0.clone(), f700(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f698(Str::lit("maxItems"), false, v0, f700(v5, 0u64)?)?), List::<(Str, Adt153)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f698(Str::lit("minProperties"), true, v0.clone(), f701(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f698(Str::lit("maxProperties"), false, v0, f701(v6, 0u64)?)?), List::<(Str, Adt153)>::nil())))
        }
    }
}

pub fn f612(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt153 {
    let m2171 = f296(Str::lit("required"), v0);
    match m2171 {
        Adt107::C0 => {
            Adt153::C0
        }
        Adt107::C1(v2) => {
            let m2172 = f223(v2);
            match m2172 {
                None => {
                    Adt153::C2
                }
                Some(v3) => {
                    let m2173 = f219(v1);
                    match m2173 {
                        None => {
                            Adt153::C0
                        }
                        Some(v4) => {
                            f702(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f613(v0: Adt142, v1: Adt142) -> Adt142 {
    let m2174 = v0.clone();
    match m2174 {
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
            let m2175 = v1.clone();
            match m2175 {
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
                        let a2176 = v5;
                        let a2177 = v9;
                        append_list(a2176, a2177)
                    })
                }
            }
        }
    }
}

pub fn f614(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt111>) -> Adt142 {
    let m2178 = f296(Str::lit("$ref"), v1);
    match m2178 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v4) => {
            let m2179 = f141(v4);
            match m2179 {
                None => {
                    Adt142::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m2180 = f684(v0, v5.clone());
                    match m2180 {
                        None => {
                            Adt142::C2(v5)
                        }
                        Some(v6) => {
                            Adt142::C3(List::cons(Adt113::C0(v6, v2, v3), List::<Adt113>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f615(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m2181 = f296(Str::lit("properties"), v0);
    match m2181 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m2182 = f219(v3);
            match m2182 {
                None => {
                    Adt142::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m2183 = f219(v1);
                    match m2183 {
                        None => {
                            Adt142::C3(List::<Adt113>::nil())
                        }
                        Some(v5) => {
                            Adt142::C3(f703(match v4 {
                                Adt5::C0(h2184) => {
                                    h2184
                                }
                            }, match v5 {
                                Adt5::C0(h2185) => {
                                    h2185
                                }
                            }, v2, List::<Adt113>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f616(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m2186 = f296(Str::lit("additionalProperties"), v0.clone());
    match m2186 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m2187 = f219(v1);
            match m2187 {
                None => {
                    Adt142::C3(List::<Adt113>::nil())
                }
                Some(v4) => {
                    let m2188 = f443(v3.clone());
                    match m2188 {
                        None => {
                            Adt142::C3(f706(match v4 {
                                Adt5::C0(h2190) => {
                                    h2190
                                }
                            }, f705(v0), v3, v2, List::<Adt113>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt142::C3(List::<Adt113>::nil())
                            } else {
                                if f704(match v4 {
                                    Adt5::C0(h2189) => {
                                        h2189
                                    }
                                }, f705(v0)) {
                                    Adt142::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt142::C3(List::<Adt113>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f617(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> R<Adt142> {
    let m2191 = f296(Str::lit("items"), v0);
    match m2191 {
        Adt107::C0 => {
            Ok(Adt142::C3(List::<Adt113>::nil()))
        }
        Adt107::C1(v3) => {
            let m2192 = f223(v1);
            match m2192 {
                None => {
                    Ok(Adt142::C3(List::<Adt113>::nil()))
                }
                Some(v4) => {
                    Ok(Adt142::C3(f707(v4, 0u64, v3, v2, List::<Adt113>::nil())?))
                }
            }
        }
    }
}

pub fn f618(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m2193 = f296(Str::lit("allOf"), v0);
    match m2193 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m2194 = f223(v3);
            match m2194 {
                None => {
                    Adt142::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m2195 = v4.clone();
                    match m2195.uncons() {
                        None => {
                            Adt142::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt142::C3(f708(v4, v1, v2, List::<Adt113>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f619(v0: Adt114, v1: Adt142, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt111>) -> Adt142 {
    let m2196 = v1.clone();
    match m2196 {
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
            let m2197 = v8;
            match m2197.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt142::C3(List::cons(Adt113::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt113>::nil()))
                }
            }
        }
    }
}

pub fn f620(p2204: List<u64>, p2205: List<u64>) -> List<u64> {
    let m2203 = (p2204, p2205);
    {
        let mut m2203 = m2203;
        loop {
            let step = {
                let (v0, v1) = m2203;
                let m2198 = v0;
                match m2198.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a2199 = v2;
                        let a2200 = 126u64;
                        if nat_eq(a2199, a2200) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a2201 = v2;
                            let a2202 = 47u64;
                            if nat_eq(a2201, a2202) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(p2210: List<u64>, p2211: u64, p2212: List<u64>) -> List<u64> {
    let m2209 = (p2210, (p2211, p2212));
    {
        let mut m2209 = m2209;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2209;
                let m2206 = v0;
                match m2206.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f709(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt155 = f710(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt155::C0(h2207, _) => {
                                h2207
                            }
                        }, match v5 {
                            Adt155::C0(_, h2208) => {
                                h2208
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(v0: Option<Adt41>) -> List<(Str, Adt6)> {
    let m2213 = v0;
    match m2213 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2219 = f711(match v1.clone() {
                    Adt41::C0(h2214, _, _) => {
                        h2214
                    }
                });
                let a2220 = {
                    let a2217 = f712(match v1.clone() {
                        Adt41::C0(_, h2215, _) => {
                            h2215
                        }
                    });
                    let a2218 = f624(Str::lit("lastModified"), match v1 {
                        Adt41::C0(_, _, h2216) => {
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

pub fn f623(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f665(Str::lit("_meta"), v0)
}

pub fn f624(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2221 = v1;
    match m2221 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f625(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2222 = v0;
    match m2222 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f713(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f626(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2223 = v0;
    match m2223 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f133(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f627(v0: Adt47) -> Adt6 {
    let m2224 = v0;
    match m2224 {
        Adt47::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt48::C0(h2225, _, _, _) => {
                    h2225
                }
            })), {
                let a2229 = f624(Str::lit("mimeType"), match v1.clone() {
                    Adt48::C0(_, h2226, _, _) => {
                        h2226
                    }
                });
                let a2230 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt48::C0(_, _, h2227, _) => {
                        h2227
                    }
                })), f623(match v1 {
                    Adt48::C0(_, _, _, h2228) => {
                        h2228
                    }
                }));
                append_list(a2229, a2230)
            }))
        }
        Adt47::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt49::C0(h2231, _, _, _) => {
                    h2231
                }
            })), {
                let a2235 = f624(Str::lit("mimeType"), match v2.clone() {
                    Adt49::C0(_, h2232, _, _) => {
                        h2232
                    }
                });
                let a2236 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt49::C0(_, _, h2233, _) => {
                        h2233
                    }
                })), f623(match v2 {
                    Adt49::C0(_, _, _, h2234) => {
                        h2234
                    }
                }));
                append_list(a2235, a2236)
            }))
        }
    }
}

pub fn f628(v0: Adt6) -> Option<Adt65> {
    let m2237 = f219(v0);
    match m2237 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2238 = f464(Str::lit("method"), v1.clone());
            match m2238 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a2239 = v2.clone();
                    let a2240 = Str::lit("elicitation/create");
                    if equal(a2239, a2240) {
                        let m2241 = f380(Str::lit("params"), v1);
                        match m2241 {
                            None => {
                                None::<Adt65>
                            }
                            Some(v3) => {
                                f714(v3)
                            }
                        }
                    } else {
                        let a2242 = v2;
                        let a2243 = Str::lit("roots/list");
                        if equal(a2242, a2243) {
                            f715(v1)
                        } else {
                            None::<Adt65>
                        }
                    }
                }
            }
        }
    }
}

pub fn f629() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f630() -> i64 {
    999999999999999999i64
}

pub fn f631(v0: i64) -> R<List<u64>> {
    let m2246 = {
        let a2244 = v0;
        let a2245 = 0i64;
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
        Ok(List::cons(45u64, f716(20u64, {
            let a2247 = v0;
            int_neg(a2247)?
        }, List::<u64>::nil())?))
    } else {
        f716(20u64, v0, List::<u64>::nil())
    }
}

pub fn f632(v0: u64) -> R<List<u64>> {
    let a2248 = v0;
    let a2249 = 34u64;
    if nat_eq(a2248, a2249) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2250 = v0;
        let a2251 = 92u64;
        if nat_eq(a2250, a2251) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2252 = v0;
            let a2253 = 8u64;
            if nat_eq(a2252, a2253) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2254 = v0;
                let a2255 = 9u64;
                if nat_eq(a2254, a2255) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2256 = v0;
                    let a2257 = 10u64;
                    if nat_eq(a2256, a2257) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2258 = v0;
                        let a2259 = 12u64;
                        if nat_eq(a2258, a2259) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2260 = v0;
                            let a2261 = 13u64;
                            if nat_eq(a2260, a2261) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2262 = v0;
                                let a2263 = 32u64;
                                if nat_lt(a2262, a2263) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f717(f682(v0, 16u64))?, List::cons(f717(f681(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f633(v0: List<Str>) -> Adt6 {
    Adt6::C4(f577(v0))
}

pub fn f634(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2281 = f665(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2264, _, _, _, _, _, _) => {
                h2264
            }
        });
        let a2282 = {
            let a2279 = f665(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2265, _, _, _, _, _) => {
                    h2265
                }
            });
            let a2280 = {
                let a2277 = f665(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2266, _, _, _, _) => {
                        h2266
                    }
                });
                let a2278 = {
                    let a2275 = f718(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2267, _, _, _) => {
                            h2267
                        }
                    });
                    let a2276 = {
                        let a2273 = f719(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2268, _, _) => {
                                h2268
                            }
                        });
                        let a2274 = {
                            let a2271 = f718(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2269, _) => {
                                    h2269
                                }
                            });
                            let a2272 = f665(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2270) => {
                                    h2270
                                }
                            });
                            append_list(a2271, a2272)
                        };
                        append_list(a2273, a2274)
                    };
                    append_list(a2275, a2276)
                };
                append_list(a2277, a2278)
            };
            append_list(a2279, a2280)
        };
        append_list(a2281, a2282)
    })
}

pub fn f635(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f133(match v0.clone() {
        Adt10::C0(h2283, _) => {
            h2283
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f720(match v0 {
        Adt10::C0(_, h2284) => {
            h2284
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f636(v0: List<Adt36>) -> List<Adt6> {
    f378(f721(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f637(v0: List<Adt45>) -> R<List<Adt6>> {
    Ok(f378(f722(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f638(v0: List<Adt47>) -> List<Adt6> {
    f378(f723(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f639(v0: List<Adt55>) -> List<Adt6> {
    f378(f724(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f640(v0: List<Adt57>) -> List<Adt6> {
    f378(f725(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f641(v0: List<Adt60>) -> R<Option<List<Adt6>>> {
    let m2285 = f726(v0, List::<Adt6>::nil())?;
    match m2285 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f378(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f642(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2286 = v1;
    match m2286 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f133(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f643(v0: List<(Str, Adt65)>) -> Option<Adt6> {
    let m2287 = f727(v0, List::<(Str, Adt6)>::nil());
    match m2287 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f461(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f644(v0: Adt82) -> List<(Str, Adt65)> {
    let m2288 = v0;
    match m2288 {
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

pub fn f645(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2289 = v1;
    match m2289 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f646(v0: Adt82) -> Option<Str> {
    let m2290 = v0;
    match m2290 {
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

pub fn f647(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m2291 = v0;
    match m2291 {
        Adt63::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt63::C1(_, v3) => {
            let m2292 = f643(v3);
            match m2292 {
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
                Adt5::C0(h2293) => {
                    h2293
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f261(v8)), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C4(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f648(v0: Adt64, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt64::C0(h2294, _, _, _, _, _) => {
            h2294
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a2300 = f645(Str::lit("statusMessage"), match v0.clone() {
            Adt64::C0(_, h2295, _, _, _, _) => {
                h2295
            }
        });
        let a2301 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, h2296, _, _, _) => {
                h2296
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, _, h2297, _, _) => {
                h2297
            }
        })), List::cons((Str::lit("ttlMs"), f728(match v0.clone() {
            Adt64::C0(_, _, _, _, h2298, _) => {
                h2298
            }
        })), f729(Str::lit("pollIntervalMs"), match v0 {
            Adt64::C0(_, _, _, _, _, h2299) => {
                h2299
            }
        }))));
        append_list(a2300, a2301)
    }))
}

pub fn f649(v0: Adt84) -> Str {
    let m2302 = v0;
    match m2302 {
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

pub fn f650(v0: Adt63) -> Adt84 {
    let m2303 = v0;
    match m2303 {
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

pub fn f651() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f652(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2304, _, _, _, _, _) => {
            h2304
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2305, _, _, _) => {
            h2305
        }
    })), {
        let a2314 = f624(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2306, _, _, _, _) => {
                h2306
            }
        });
        let a2315 = {
            let a2312 = f624(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2307, _, _) => {
                    h2307
                }
            });
            let a2313 = {
                let a2310 = f624(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2308, _) => {
                        h2308
                    }
                });
                let a2311 = f625(match v0 {
                    Adt1::C0(_, _, _, _, _, h2309) => {
                        h2309
                    }
                });
                append_list(a2310, a2311)
            };
            append_list(a2312, a2313)
        };
        append_list(a2314, a2315)
    })))
}

pub fn f653(v0: List<Adt85>) -> Option<Adt6> {
    let m2316 = f730(v0, List::<Adt6>::nil());
    match m2316 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f378(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f654(v0: Str, p2321: List<Adt85>, v2: i64, p2322: List<Adt85>) -> R<List<Adt85>> {
    let m2320 = (p2321, p2322);
    {
        let mut m2320 = m2320;
        loop {
            let step = {
                let (v1, v3) = m2320;
                let m2317 = v1;
                match m2317.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a2318 = f225(v4.clone());
                        let a2319 = v0.clone();
                        if if equal(a2318, a2319) {
                            true
                        } else {
                            f226(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f655(v0: List<List<u64>>) -> bool {
    let m2323 = v0;
    match m2323.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f656() -> List<Adt92> {
    List::cons(Adt92::C0(f731(), f580()), List::cons(Adt92::C0(f732(), f733()), List::cons(Adt92::C0(f734(), f735()), List::<Adt92>::nil())))
}

pub fn f657(v0: List<List<u64>>) -> List<u64> {
    f736(f45(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f658(v0: Adt20) -> Adt6 {
    let m2324 = v0;
    match m2324 {
        Adt20::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt20::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f659(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2325 = v0;
    match m2325 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f660(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2326 = v0;
    match m2326 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f661() -> i64 {
    -32603i64
}

pub fn f662() -> i64 {
    429i64
}

pub fn f663() -> i64 {
    -32021i64
}

pub fn f664(p2329: List<Str>, p2330: List<Adt6>) -> List<Adt6> {
    let m2328 = (p2329, p2330);
    {
        let mut m2328 = m2328;
        loop {
            let step = {
                let (v0, v1) = m2328;
                let m2327 = v0;
                match m2327.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f665(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2331 = v1;
    match m2331 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2332) => {
                    h2332
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f666(v0: List<u64>) -> R<bool> {
    let m2333 = v0;
    match m2333.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2334 = v2;
            match m2334.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2335 = f355(v1)?;
                    let a2336 = 113u64;
                    if if nat_eq(a2335, a2336) {
                        let a2337 = v3;
                        let a2338 = 61u64;
                        nat_eq(a2337, a2338)
                    } else {
                        false
                    } {
                        Ok(f737(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f667(v0: Adt0, v1: Fn3, v2: u64, v3: Result<Adt15, Adt138>) -> R<Result<Adt134, Adt137>> {
    let m2339 = v3;
    match m2339 {
        Ok(v5) => {
            f738(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt134, Adt137>(Adt137::C0(v4)))
        }
    }
}

pub fn f668(v0: Adt146, v1: Adt17, v2: Adt5) -> Result<Adt15, Adt138> {
    let m2340 = v0;
    match m2340 {
        Adt146::C0 => {
            f739(v1, v2)
        }
        Adt146::C1 => {
            f740(v1, v2)
        }
        Adt146::C2 => {
            f741(v1, v2)
        }
        Adt146::C3 => {
            f742(v1, v2)
        }
        Adt146::C4 => {
            f743(v1, v2)
        }
        Adt146::C5 => {
            f744(v1, v2)
        }
        Adt146::C6 => {
            f745(v1, v2)
        }
        Adt146::C7 => {
            f746(v1, v2)
        }
        Adt146::C8 => {
            f747(v1, v2)
        }
        Adt146::C9 => {
            f748(v1, v2)
        }
        Adt146::C10 => {
            f749(v1, v2)
        }
        Adt146::C11 => {
            f750(v1, v2)
        }
    }
}

pub fn f669(v0: Adt146) -> Str {
    let m2341 = v0;
    match m2341 {
        Adt146::C0 => {
            Str::lit("server/discover")
        }
        Adt146::C1 => {
            Str::lit("tools/list")
        }
        Adt146::C2 => {
            Str::lit("tools/call")
        }
        Adt146::C3 => {
            Str::lit("resources/list")
        }
        Adt146::C4 => {
            Str::lit("resources/read")
        }
        Adt146::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt146::C6 => {
            Str::lit("prompts/list")
        }
        Adt146::C7 => {
            Str::lit("prompts/get")
        }
        Adt146::C8 => {
            Str::lit("completion/complete")
        }
        Adt146::C9 => {
            Str::lit("tasks/get")
        }
        Adt146::C10 => {
            Str::lit("tasks/update")
        }
        Adt146::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f670(v0: Option<Adt8>) -> bool {
    let m2342 = v0;
    match m2342 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f671(v0: Option<Adt9>) -> bool {
    let m2343 = v0;
    match m2343 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f672(v0: Option<Adt5>) -> bool {
    let m2344 = v0;
    match m2344 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f673(v0: List<Str>, v1: Adt108, v2: Adt17, v3: Option<Adt117>) -> Adt149 {
    let m2345 = v3;
    match m2345 {
        None => {
            f751(v1, v2.clone(), f752(match v2 {
                Adt17::C0(h2346, _, _, _, _, _) => {
                    h2346
                }
            }, v0))
        }
        Some(v4) => {
            Adt149::C1(Adt137::C2(v4))
        }
    }
}

pub fn f674(v0: Option<Adt116>, v1: Adt151, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt117>> {
    let m2347 = v0;
    match m2347 {
        None => {
            Ok(None::<Adt117>)
        }
        Some(v5) => {
            let m2349 = match v5.clone() {
                Adt116::C0(_, _, _, h2348, _) => {
                    h2348
                }
            };
            match m2349 {
                None => {
                    f753(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f675(v0: Option<Adt5>) -> Adt5 {
    let m2350 = v0;
    match m2350 {
        None => {
            f27()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f676(v0: Adt5) -> Result<Adt17, Adt138> {
    let m2351 = f754(v0.clone());
    match m2351 {
        Adt158::C0 => {
            f755(v0.clone(), f214(f756(), v0.clone()), f214(f757(), v0))
        }
        Adt158::C1 => {
            Err::<Adt17, Adt138>(Adt138::C2)
        }
        Adt158::C2 => {
            Err::<Adt17, Adt138>(Adt138::C3)
        }
    }
}

pub fn f677(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2352 = f758(v0)?;
    match m2352 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2353 = f758(v1)?;
            match m2353 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2354 = v2;
                    let a2355 = 61u64;
                    if nat_eq(a2354, a2355) {
                        f759(v5, v6, v4, v3)
                    } else {
                        let m2356 = f758(v2)?;
                        match m2356 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2357 = v3;
                                let a2358 = 61u64;
                                if nat_eq(a2357, a2358) {
                                    f760(v5, v6, v7, v4)
                                } else {
                                    let m2359 = f758(v3)?;
                                    match m2359 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f761(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f678(v0: List<u64>) -> bool {
    let m2360 = v0;
    match m2360.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f679() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f680(v0: Str) -> R<List<u64>> {
    f177({
        let a2361 = v0;
        utf8_encode(a2361)
    })
}

pub fn f681(v0: u64, v1: u64) -> u64 {
    let a2362 = v0;
    let a2363 = v1;
    let a2364 = 0u64;
    nat_rem(a2362, a2363, a2364)
}

pub fn f682(v0: u64, v1: u64) -> u64 {
    let a2365 = v0;
    let a2366 = v1;
    let a2367 = 0u64;
    nat_quot(a2365, a2366, a2367)
}

pub fn f683(v0: u64, v1: Adt6) -> bool {
    let a2368 = v0;
    let a2369 = 0u64;
    if nat_eq(a2368, a2369) {
        f762(v1)
    } else {
        let a2370 = v0;
        let a2371 = 1u64;
        if nat_eq(a2370, a2371) {
            f763(v1)
        } else {
            let a2372 = v0;
            let a2373 = 2u64;
            if nat_eq(a2372, a2373) {
                f764(v1)
            } else {
                let a2374 = v0;
                let a2375 = 3u64;
                if nat_eq(a2374, a2375) {
                    f765(v1)
                } else {
                    let a2376 = v0;
                    let a2377 = 4u64;
                    if nat_eq(a2376, a2377) {
                        f766(v1)
                    } else {
                        let a2378 = v0;
                        let a2379 = 5u64;
                        if nat_eq(a2378, a2379) {
                            f767(v1)
                        } else {
                            let a2380 = v0;
                            let a2381 = 6u64;
                            if nat_eq(a2380, a2381) {
                                f768(v1)
                            } else {
                                let a2382 = v0;
                                let a2383 = 7u64;
                                if nat_eq(a2382, a2383) {
                                    f769(v1)
                                } else {
                                    f770(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f684(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a2384 = v1.clone();
    let a2385 = Str::lit("#");
    if equal(a2384, a2385) {
        Some(v0)
    } else {
        let m2386 = f219(v0);
        match m2386 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m2387 = f214(Str::lit("$defs"), v2);
                match m2387 {
                    Adt107::C0 => {
                        None::<Adt6>
                    }
                    Adt107::C1(v3) => {
                        let m2388 = f219(v3);
                        match m2388 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f771(v1, match v4 {
                                    Adt5::C0(h2389) => {
                                        h2389
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

pub fn f685(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt129) -> R<Adt152> {
    let m2390 = v3.clone();
    match m2390.uncons() {
        None => {
            Ok(Adt152::C0(Adt97::C8(v0)))
        }
        Some((_, _)) => {
            if f359(v3.clone(), true) {
                let m2391 = f772(v1);
                match m2391 {
                    None => {
                        Ok(Adt152::C0(Adt97::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f773(v3)?;
                        if f774(v9.clone(), match v5.clone() {
                            Adt129::C0(_, h2392) => {
                                h2392
                            }
                        }) {
                            Ok(Adt152::C0(Adt97::C12(v0, v2)))
                        } else {
                            Ok(Adt152::C1(Adt129::C0(List::cons(Adt12::C0(v0, v2, f326(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt129::C0(h2393, _) => {
                                    h2393
                                }
                            }), List::cons(v9, match v5 {
                                Adt129::C0(_, h2394) => {
                                    h2394
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt152::C0(Adt97::C9(v0)))
            }
        }
    }
}

pub fn f686(p2399: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2400: List<Adt109>) -> List<Adt109> {
    let m2398 = (p2399, p2400);
    {
        let mut m2398 = m2398;
        loop {
            let step = {
                let (v0, v3) = m2398;
                let m2395 = v0;
                match m2395.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0({
                            let (_, h2396) = v4;
                            h2396
                        }, v1, List::cons({
                            let (h2397, _) = v4;
                            h2397
                        }, v2.clone())), v3)))
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

pub fn f687(p2404: List<(Str, Adt6)>, p2405: List<Adt109>) -> List<Adt109> {
    let m2403 = (p2404, p2405);
    {
        let mut m2403 = m2403;
        loop {
            let step = {
                let (v0, v1) = m2403;
                let m2401 = v0;
                match m2401.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v3, List::cons(Adt109::C0({
                            let (_, h2402) = v2;
                            h2402
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f688(p2408: List<Adt6>, p2409: List<Adt109>) -> List<Adt109> {
    let m2407 = (p2408, p2409);
    {
        let mut m2407 = m2407;
        loop {
            let step = {
                let (v0, v1) = m2407;
                let m2406 = v0;
                match m2406.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt109>), List<Adt109>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt109>), List<Adt109>>((v3, List::cons(Adt109::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f689(v0: Adt156) -> bool {
    let m2410 = v0;
    match m2410 {
        Adt156::C0 => {
            false
        }
        Adt156::C1 => {
            false
        }
        Adt156::C2 => {
            true
        }
    }
}

pub fn f690(v0: Adt7, v1: Adt7) -> R<Adt156> {
    f775(v0.clone(), v1.clone(), f776(match v0 {
        Adt7::C0(h2411, _) => {
            h2411
        }
    }), f776(match v1 {
        Adt7::C0(h2412, _) => {
            h2412
        }
    }))
}

pub fn f691(v0: Str) -> bool {
    let a2413 = v0.clone();
    let a2414 = Str::lit("null");
    if equal(a2413, a2414) {
        true
    } else {
        let a2415 = v0.clone();
        let a2416 = Str::lit("boolean");
        if equal(a2415, a2416) {
            true
        } else {
            let a2417 = v0.clone();
            let a2418 = Str::lit("object");
            if equal(a2417, a2418) {
                true
            } else {
                let a2419 = v0.clone();
                let a2420 = Str::lit("array");
                if equal(a2419, a2420) {
                    true
                } else {
                    let a2421 = v0.clone();
                    let a2422 = Str::lit("number");
                    if equal(a2421, a2422) {
                        true
                    } else {
                        let a2423 = v0.clone();
                        let a2424 = Str::lit("string");
                        if equal(a2423, a2424) {
                            true
                        } else {
                            let a2425 = v0;
                            let a2426 = Str::lit("integer");
                            equal(a2425, a2426)
                        }
                    }
                }
            }
        }
    }
}

pub fn f692(v0: Str, v1: Adt6) -> R<bool> {
    let a2427 = v0.clone();
    let a2428 = f777(v1.clone());
    if equal(a2427, a2428) {
        Ok(true)
    } else {
        let a2429 = v0;
        let a2430 = Str::lit("integer");
        if equal(a2429, a2430) {
            let m2431 = f696(v1);
            match m2431 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f778(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f693(p2434: List<Adt6>, v1: Adt6, p2435: bool) -> R<Adt153> {
    let m2433 = (p2434, p2435);
    {
        let mut m2433 = m2433;
        loop {
            let step = {
                let (v0, v2) = m2433;
                let m2432 = v0;
                match m2432.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt153>>(Ok(Adt153::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt153>>(Ok(Adt153::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f691(f779(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt153>>((v4, if v2 {
                                true
                            } else {
                                f692(f779(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt153>>(Ok(Adt153::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f694(p2439: List<Adt6>, v1: Adt6) -> R<Adt153> {
    let m2438 = p2439;
    {
        let mut m2438 = m2438;
        loop {
            let step = {
                let v0 = m2438;
                let m2436 = v0;
                match m2436.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt153>>(Ok(Adt153::C1))
                    }
                    Some((v2, v3)) => {
                        let m2437 = f695(v2, v1.clone())?;
                        match m2437 {
                            Adt157::C0 => {
                                Err::<List<Adt6>, R<Adt153>>(Ok(Adt153::C0))
                            }
                            Adt157::C1 => {
                                Ok::<List<Adt6>, R<Adt153>>(v3)
                            }
                            Adt157::C2 => {
                                Err::<List<Adt6>, R<Adt153>>(Ok(Adt153::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f695(v0: Adt6, v1: Adt6) -> R<Adt157> {
    f780(f781(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f696(v0: Adt6) -> Option<Adt7> {
    let m2440 = v0;
    match m2440 {
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

pub fn f697(v0: u64, v1: Adt156) -> bool {
    let a2441 = v0;
    let a2442 = 0u64;
    if nat_eq(a2441, a2442) {
        let a2443 = f782(v1);
        bool_not(a2443)
    } else {
        let a2444 = v0;
        let a2445 = 1u64;
        if nat_eq(a2444, a2445) {
            let a2446 = f689(v1);
            bool_not(a2446)
        } else {
            let a2447 = v0;
            let a2448 = 2u64;
            if nat_eq(a2447, a2448) {
                f689(v1)
            } else {
                f782(v1)
            }
        }
    }
}

pub fn f698(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt153> {
    let m2449 = f296(v0, v2);
    match m2449 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v4) => {
            let m2450 = f215(v4);
            match m2450 {
                None => {
                    Ok(Adt153::C2)
                }
                Some(v5) => {
                    let m2453 = {
                        let a2451 = v5;
                        let a2452 = 0i64;
                        compare(a2451, a2452)
                    };
                    if match m2453 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt153::C2)
                    } else {
                        if v1 {
                            if f782(f783(f221(v3, 0i64)?, v5)) {
                                Ok(Adt153::C1)
                            } else {
                                Ok(Adt153::C0)
                            }
                        } else {
                            if f689(f783(f221(v3, 0i64)?, v5)) {
                                Ok(Adt153::C1)
                            } else {
                                Ok(Adt153::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f699(v0: Str) -> R<u64> {
    f784(f177({
        let a2454 = v0;
        utf8_encode(a2454)
    })?, 0u64)
}

pub fn f700(p2459: List<Adt6>, p2460: u64) -> R<u64> {
    let m2458 = (p2459, p2460);
    {
        let mut m2458 = m2458;
        loop {
            let step = {
                let (v0, v1) = m2458;
                let m2455 = v0;
                match m2455.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a2456 = v1;
                            let a2457 = 1u64;
                            nat_add(a2456, a2457)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f701(p2465: List<(Str, Adt6)>, p2466: u64) -> R<u64> {
    let m2464 = (p2465, p2466);
    {
        let mut m2464 = m2464;
        loop {
            let step = {
                let (v0, v1) = m2464;
                let m2461 = v0;
                match m2461.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a2462 = v1;
                            let a2463 = 1u64;
                            nat_add(a2462, a2463)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(p2470: List<Adt6>, v1: Adt5) -> Adt153 {
    let m2469 = p2470;
    {
        let mut m2469 = m2469;
        loop {
            let step = {
                let v0 = m2469;
                let m2467 = v0;
                match m2467.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt153>(Adt153::C0)
                    }
                    Some((v2, v3)) => {
                        let m2468 = f141(v2);
                        match m2468 {
                            None => {
                                Err::<List<Adt6>, Adt153>(Adt153::C2)
                            }
                            Some(v4) => {
                                if f303(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt153>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt153>(Adt153::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f703(p2477: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt111>, p2478: List<Adt113>) -> List<Adt113> {
    let m2476 = (p2477, p2478);
    {
        let mut m2476 = m2476;
        loop {
            let step = {
                let (v0, v3) = m2476;
                let m2471 = v0;
                match m2471.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2473 = f296({
                            let (h2472, _) = v4.clone();
                            h2472
                        }, v1.clone());
                        match m2473 {
                            Adt107::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v5, v3))
                            }
                            Adt107::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v5, List::cons(Adt113::C0({
                                    let (_, h2474) = v4;
                                    h2474
                                }, v6, List::cons(Adt111::C0({
                                    let (h2475, _) = v4;
                                    h2475
                                }), v2.clone())), v3)))
                            }
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

pub fn f704(p2483: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2482 = p2483;
    {
        let mut m2482 = m2482;
        loop {
            let step = {
                let v0 = m2482;
                let m2479 = v0;
                match m2479.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2481 = f296({
                            let (h2480, _) = v2;
                            h2480
                        }, v1.clone());
                        match m2481 {
                            Adt107::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt107::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f705(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2484 = f296(Str::lit("properties"), v0);
    match m2484 {
        Adt107::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt107::C1(v1) => {
            let m2485 = f219(v1);
            match m2485 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2486) => {
                            h2486
                        }
                    }
                }
            }
        }
    }
}

pub fn f706(p2493: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt111>, p2494: List<Adt113>) -> List<Adt113> {
    let m2492 = (p2493, p2494);
    {
        let mut m2492 = m2492;
        loop {
            let step = {
                let (v0, v4) = m2492;
                let m2487 = v0;
                match m2487.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2489 = f296({
                            let (h2488, _) = v5.clone();
                            h2488
                        }, v1.clone());
                        match m2489 {
                            Adt107::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v6, List::cons(Adt113::C0(v2.clone(), {
                                    let (_, h2490) = v5;
                                    h2490
                                }, List::cons(Adt111::C0({
                                    let (h2491, _) = v5;
                                    h2491
                                }), v3.clone())), v4)))
                            }
                            Adt107::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2492 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f707(p2499: List<Adt6>, p2500: u64, v2: Adt6, v3: List<Adt111>, p2501: List<Adt113>) -> R<List<Adt113>> {
    let m2498 = (p2499, (p2500, p2501));
    {
        let mut m2498 = m2498;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2498;
                let m2495 = v0;
                match m2495.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt113>)), R<List<Adt113>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt113>)), R<List<Adt113>>>((v6, ({
                            let a2496 = v1;
                            let a2497 = 1u64;
                            nat_add(a2496, a2497)?
                        }, List::cons(Adt113::C0(v2.clone(), v5, List::cons(Adt111::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f708(p2504: List<Adt6>, v1: Adt6, v2: List<Adt111>, p2505: List<Adt113>) -> List<Adt113> {
    let m2503 = (p2504, p2505);
    {
        let mut m2503 = m2503;
        loop {
            let step = {
                let (v0, v3) = m2503;
                let m2502 = v0;
                match m2502.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt113>), List<Adt113>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt113>), List<Adt113>>((v5, List::cons(Adt113::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f709(v0: u64, v1: List<u64>) -> List<u64> {
    let a2506 = v0;
    let a2507 = 0u64;
    if nat_eq(a2506, a2507) {
        v1
    } else {
        let a2508 = v0;
        let a2509 = 1u64;
        if nat_eq(a2508, a2509) {
            List::cons(194u64, v1)
        } else {
            let a2510 = v0;
            let a2511 = 2u64;
            if nat_eq(a2510, a2511) {
                List::cons(226u64, v1)
            } else {
                let a2512 = v0;
                let a2513 = 3u64;
                if nat_eq(a2512, a2513) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f710(v0: u64, v1: u64, v2: List<u64>) -> Adt155 {
    let a2514 = v0;
    let a2515 = 0u64;
    if nat_eq(a2514, a2515) {
        f785(v1, v2)
    } else {
        let a2516 = v0;
        let a2517 = 1u64;
        if nat_eq(a2516, a2517) {
            let a2518 = 128u64;
            let a2519 = v1;
            if if nat_le(a2518, a2519) {
                let a2520 = v1;
                let a2521 = 159u64;
                nat_le(a2520, a2521)
            } else {
                false
            } {
                Adt155::C0(0u64, f786(v2))
            } else {
                f785(v1, List::cons(194u64, v2))
            }
        } else {
            let a2522 = v0;
            let a2523 = 2u64;
            if nat_eq(a2522, a2523) {
                let a2524 = v1;
                let a2525 = 128u64;
                if nat_eq(a2524, a2525) {
                    Adt155::C0(3u64, v2)
                } else {
                    let a2526 = v1;
                    let a2527 = 129u64;
                    if nat_eq(a2526, a2527) {
                        Adt155::C0(4u64, v2)
                    } else {
                        f785(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2528 = v0;
                let a2529 = 3u64;
                if nat_eq(a2528, a2529) {
                    let a2530 = 170u64;
                    let a2531 = v1;
                    if if nat_le(a2530, a2531) {
                        let a2532 = v1;
                        let a2533 = 174u64;
                        nat_le(a2532, a2533)
                    } else {
                        false
                    } {
                        Adt155::C0(0u64, f786(v2))
                    } else {
                        f785(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2534 = 166u64;
                    let a2535 = v1;
                    if if nat_le(a2534, a2535) {
                        let a2536 = v1;
                        let a2537 = 169u64;
                        nat_le(a2536, a2537)
                    } else {
                        false
                    } {
                        Adt155::C0(0u64, f786(v2))
                    } else {
                        f785(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f711(v0: Option<List<Adt42>>) -> List<(Str, Adt6)> {
    let m2538 = v0;
    match m2538 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f787(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f712(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2539 = v0;
    match m2539 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f713(v0: List<Adt2>) -> List<Adt6> {
    f378(f788(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f714(v0: Adt5) -> Option<Adt65> {
    let m2540 = f464(Str::lit("message"), v0.clone());
    match m2540 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2541 = f214(Str::lit("mode"), v0.clone());
            match m2541 {
                Adt107::C0 => {
                    let m2542 = f380(Str::lit("requestedSchema"), v0);
                    match m2542 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v2) => {
                            Some(Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v1, v2))))
                        }
                    }
                }
                Adt107::C1(v3) => {
                    let m2543 = f141(v3);
                    match m2543 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v4) => {
                            let a2544 = v4.clone();
                            let a2545 = Str::lit("url");
                            if equal(a2544, a2545) {
                                let m2546 = f464(Str::lit("url"), v0);
                                match m2546 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v5) => {
                                        Some(Adt65::C0(Adt66::C1(Adt69::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a2547 = v4;
                                let a2548 = Str::lit("form");
                                if equal(a2547, a2548) {
                                    let m2549 = f380(Str::lit("requestedSchema"), v0);
                                    match m2549 {
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

pub fn f715(v0: Adt5) -> Option<Adt65> {
    let m2550 = f214(Str::lit("params"), v0);
    match m2550 {
        Adt107::C0 => {
            Some(Adt65::C2(None::<Adt5>))
        }
        Adt107::C1(v1) => {
            let m2551 = f219(v1);
            match m2551 {
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

pub fn f716(p2563: u64, p2564: i64, p2565: List<u64>) -> R<List<u64>> {
    let m2562 = (p2563, (p2564, p2565));
    {
        let mut m2562 = m2562;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2562;
                let m2552 = v0;
                if m2552 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2552 - 1;
                    let v4: i64 = {
                        let a2553 = v1;
                        let a2554 = 10i64;
                        let a2555 = 0i64;
                        int_quot(a2553, a2554, a2555)?
                    };
                    let v5: i64 = {
                        let a2556 = v1;
                        let a2557 = 10i64;
                        let a2558 = 0i64;
                        int_rem(a2556, a2557, a2558)
                    };
                    let m2561 = {
                        let a2559 = v4;
                        let a2560 = 1i64;
                        compare(a2559, a2560)
                    };
                    if match m2561 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f789(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f789(v5), v2))))
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

pub fn f717(v0: u64) -> R<u64> {
    let a2566 = v0;
    let a2567 = 10u64;
    if nat_lt(a2566, a2567) {
        let a2568 = 48u64;
        let a2569 = v0;
        nat_add(a2568, a2569)
    } else {
        let a2570 = 87u64;
        let a2571 = v0;
        nat_add(a2570, a2571)
    }
}

pub fn f718(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2572 = v1;
    match m2572 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f790(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f719(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2573 = v0;
    match m2573 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2576 = f301(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2574, _) => {
                        h2574
                    }
                });
                let a2577 = f301(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2575) => {
                        h2575
                    }
                });
                append_list(a2576, a2577)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f720(v0: Adt11) -> Str {
    let m2578 = v0;
    match m2578 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f721(p2581: List<Adt36>, p2582: List<Adt6>) -> List<Adt6> {
    let m2580 = (p2581, p2582);
    {
        let mut m2580 = m2580;
        loop {
            let step = {
                let (v0, v1) = m2580;
                let m2579 = v0;
                match m2579.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt36>, List<Adt6>), List<Adt6>>((v3, List::cons(f791(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f722(p2585: List<Adt45>, p2586: List<Adt6>) -> R<List<Adt6>> {
    let m2584 = (p2585, p2586);
    {
        let mut m2584 = m2584;
        loop {
            let step = {
                let (v0, v1) = m2584;
                let m2583 = v0;
                match m2583.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f792(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2584 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f723(p2589: List<Adt47>, p2590: List<Adt6>) -> List<Adt6> {
    let m2588 = (p2589, p2590);
    {
        let mut m2588 = m2588;
        loop {
            let step = {
                let (v0, v1) = m2588;
                let m2587 = v0;
                match m2587.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), List<Adt6>>((v3, List::cons(f627(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2588 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f724(p2593: List<Adt55>, p2594: List<Adt6>) -> List<Adt6> {
    let m2592 = (p2593, p2594);
    {
        let mut m2592 = m2592;
        loop {
            let step = {
                let (v0, v1) = m2592;
                let m2591 = v0;
                match m2591.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt55>, List<Adt6>), List<Adt6>>((v3, List::cons(f793(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f725(p2597: List<Adt57>, p2598: List<Adt6>) -> List<Adt6> {
    let m2596 = (p2597, p2598);
    {
        let mut m2596 = m2596;
        loop {
            let step = {
                let (v0, v1) = m2596;
                let m2595 = v0;
                match m2595.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f794(v2), v1)))
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

pub fn f726(p2602: List<Adt60>, p2603: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2601 = (p2602, p2603);
    {
        let mut m2601 = m2601;
        loop {
            let step = {
                let (v0, v1) = m2601;
                let m2599 = v0;
                match m2599.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2600 = f795(v2)?;
                        match m2600 {
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
                Ok(next) => { m2601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f727(p2609: List<(Str, Adt65)>, p2610: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m2608 = (p2609, p2610);
    {
        let mut m2608 = m2608;
        loop {
            let step = {
                let (v0, v1) = m2608;
                let m2604 = v0;
                match m2604.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2606 = f796({
                            let (_, h2605) = v2;
                            h2605
                        });
                        match m2606 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h2607, _) = v2;
                                    h2607
                                }, v4), v1)))
                            }
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

pub fn f728(v0: Option<i64>) -> Adt6 {
    let m2611 = v0;
    match m2611 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f797(v1)
        }
    }
}

pub fn f729(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m2612 = v1;
    match m2612 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f797(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f730(p2616: List<Adt85>, p2617: List<Adt6>) -> Option<List<Adt6>> {
    let m2615 = (p2616, p2617);
    {
        let mut m2615 = m2615;
        loop {
            let step = {
                let (v0, v1) = m2615;
                let m2613 = v0;
                match m2613.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2614 = f798(v2);
                        match m2614 {
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
                Ok(next) => { m2615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f731() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f732() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f733() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f734() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f735() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f736(p2622: List<List<u64>>, p2623: List<u64>) -> List<u64> {
    let m2621 = (p2622, p2623);
    {
        let mut m2621 = m2621;
        loop {
            let step = {
                let (v0, v1) = m2621;
                let m2618 = v0;
                match m2618.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2619 = f799(v2);
                            let a2620 = v1;
                            append_list(a2619, a2620)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f737(v0: List<u64>) -> bool {
    let m2624 = v0;
    match m2624.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2625 = v1;
            let a2626 = 48u64;
            if nat_eq(a2625, a2626) {
                f800(v2)
            } else {
                false
            }
        }
    }
}

pub fn f738(v0: Adt0, v1: Fn3, v2: u64, v3: Adt15) -> R<Result<Adt134, Adt137>> {
    if f801(f802(v3.clone())) {
        Ok(Ok::<Adt134, Adt137>(f803(v0.clone(), Adt33::C0(f804(v0)))))
    } else {
        Ok(f805(v0.clone(), v3.clone(), f806(match v0 {
            Adt0::C0(_, _, _, _, _, h2627) => {
                h2627
            }
        }, v2, v1, v3)?))
    }
}

pub fn f739(v0: Adt17, _: Adt5) -> Result<Adt15, Adt138> {
    Ok::<Adt15, Adt138>(Adt15::C0(Adt16::C0(v0)))
}

pub fn f740(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2628 = f807(Str::lit("cursor"), v1);
    match m2628 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C1(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f741(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2629 = f808(Str::lit("name"), v1.clone());
    match m2629 {
        Ok(v2) => {
            let m2630 = f809(Str::lit("arguments"), v1.clone());
            match m2630 {
                Ok(v3) => {
                    let m2631 = f809(Str::lit("inputResponses"), v1.clone());
                    match m2631 {
                        Ok(v4) => {
                            let m2632 = f807(Str::lit("requestState"), v1);
                            match m2632 {
                                Ok(v5) => {
                                    Ok::<Adt15, Adt138>(Adt15::C2(Adt22::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt15, Adt138>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt15, Adt138>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt15, Adt138>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt15, Adt138>(v9)
        }
    }
}

pub fn f742(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2633 = f807(Str::lit("cursor"), v1);
    match m2633 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C3(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f743(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2634 = f808(Str::lit("uri"), v1.clone());
    match m2634 {
        Ok(v2) => {
            let m2635 = f809(Str::lit("inputResponses"), v1.clone());
            match m2635 {
                Ok(v3) => {
                    let m2636 = f807(Str::lit("requestState"), v1);
                    match m2636 {
                        Ok(v4) => {
                            Ok::<Adt15, Adt138>(Adt15::C4(Adt23::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt15, Adt138>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt15, Adt138>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt15, Adt138>(v7)
        }
    }
}

pub fn f744(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2637 = f807(Str::lit("cursor"), v1);
    match m2637 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C5(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f745(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2638 = f807(Str::lit("cursor"), v1);
    match m2638 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C6(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f746(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2639 = f808(Str::lit("name"), v1.clone());
    match m2639 {
        Ok(v2) => {
            let m2640 = f810(Str::lit("arguments"), v1.clone());
            match m2640 {
                Ok(v3) => {
                    let m2641 = f809(Str::lit("inputResponses"), v1.clone());
                    match m2641 {
                        Ok(v4) => {
                            let m2642 = f807(Str::lit("requestState"), v1);
                            match m2642 {
                                Ok(v5) => {
                                    Ok::<Adt15, Adt138>(Adt15::C7(Adt24::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt15, Adt138>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt15, Adt138>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt15, Adt138>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt15, Adt138>(v9)
        }
    }
}

pub fn f747(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2643 = f811(Str::lit("ref"), v1.clone());
    match m2643 {
        Ok(v2) => {
            let m2644 = f812(v2);
            match m2644 {
                Ok(v3) => {
                    let m2645 = f811(Str::lit("argument"), v1.clone());
                    match m2645 {
                        Ok(v4) => {
                            let m2646 = f813(v4);
                            match m2646 {
                                Ok(v5) => {
                                    let m2647 = f814(v1);
                                    match m2647 {
                                        Ok(v6) => {
                                            Ok::<Adt15, Adt138>(Adt15::C8(Adt25::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt15, Adt138>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt15, Adt138>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt15, Adt138>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt15, Adt138>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt15, Adt138>(v11)
        }
    }
}

pub fn f748(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2648 = f808(Str::lit("taskId"), v1);
    match m2648 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C9(Adt30::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f749(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2649 = f808(Str::lit("taskId"), v1.clone());
    match m2649 {
        Ok(v2) => {
            let m2650 = f811(Str::lit("inputResponses"), v1);
            match m2650 {
                Ok(v3) => {
                    Ok::<Adt15, Adt138>(Adt15::C10(Adt31::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt15, Adt138>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt15, Adt138>(v5)
        }
    }
}

pub fn f750(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2651 = f808(Str::lit("taskId"), v1);
    match m2651 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C11(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f751(v0: Adt108, v1: Adt17, v2: Option<Adt137>) -> Adt149 {
    let m2652 = v2;
    match m2652 {
        None => {
            Adt149::C0(Adt150::C0(match v0.clone() {
                Adt108::C0(h2653, _, _) => {
                    h2653
                }
            }, match v0.clone() {
                Adt108::C0(_, h2654, _) => {
                    h2654
                }
            }, v1, f675(match v0 {
                Adt108::C0(_, _, h2655) => {
                    h2655
                }
            })))
        }
        Some(v3) => {
            Adt149::C1(v3)
        }
    }
}

pub fn f752(v0: Str, v1: List<Str>) -> Option<Adt137> {
    f815(f816(v0.clone(), v1.clone()), v0, v1)
}

pub fn f753(v0: Adt116, v1: Adt151, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt117>> {
    let m2657 = match v0.clone() {
        Adt116::C0(h2656, _, _, _, _) => {
            h2656
        }
    };
    match m2657 {
        None => {
            Ok(Some(Adt117::C0))
        }
        Some(v5) => {
            let a2658 = v5;
            let a2659 = v3;
            if equal(a2658, a2659) {
                let m2661 = match v0.clone() {
                    Adt116::C0(_, h2660, _, _, _) => {
                        h2660
                    }
                };
                match m2661 {
                    None => {
                        Ok(Some(Adt117::C1))
                    }
                    Some(v6) => {
                        let a2662 = v6;
                        let a2663 = v2;
                        if equal(a2662, a2663) {
                            f817(f818(match v0.clone() {
                                Adt116::C0(_, _, h2664, _, _) => {
                                    h2664
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt116::C0(_, _, _, _, h2665) => {
                                    h2665
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt117::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt117::C3))
            }
        }
    }
}

pub fn f754(v0: Adt5) -> Adt158 {
    f819(f214(f756(), v0.clone()), f214(f757(), v0))
}

pub fn f755(v0: Adt5, v1: Adt107, v2: Adt107) -> Result<Adt17, Adt138> {
    let m2666 = v1;
    match m2666 {
        Adt107::C0 => {
            Err::<Adt17, Adt138>(Adt138::C2)
        }
        Adt107::C1(v3) => {
            let m2667 = v2;
            match m2667 {
                Adt107::C0 => {
                    Err::<Adt17, Adt138>(Adt138::C3)
                }
                Adt107::C1(v4) => {
                    f820(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f756() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f757() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f758(v0: u64) -> R<Option<u64>> {
    let a2668 = 64u64;
    let a2669 = v0;
    if if nat_lt(a2668, a2669) {
        let a2670 = v0;
        let a2671 = 91u64;
        nat_lt(a2670, a2671)
    } else {
        false
    } {
        Ok(Some({
            let a2672 = v0;
            let a2673 = 65u64;
            nat_sub(a2672, a2673)
        }))
    } else {
        let a2674 = 96u64;
        let a2675 = v0;
        if if nat_lt(a2674, a2675) {
            let a2676 = v0;
            let a2677 = 123u64;
            nat_lt(a2676, a2677)
        } else {
            false
        } {
            Ok(Some({
                let a2678 = v0;
                let a2679 = 71u64;
                nat_sub(a2678, a2679)
            }))
        } else {
            let a2680 = 47u64;
            let a2681 = v0;
            if if nat_lt(a2680, a2681) {
                let a2682 = v0;
                let a2683 = 58u64;
                nat_lt(a2682, a2683)
            } else {
                false
            } {
                Ok(Some({
                    let a2684 = v0;
                    let a2685 = 4u64;
                    nat_add(a2684, a2685)?
                }))
            } else {
                let a2686 = v0;
                let a2687 = 43u64;
                if nat_eq(a2686, a2687) {
                    Ok(Some(62u64))
                } else {
                    let a2688 = v0;
                    let a2689 = 47u64;
                    if nat_eq(a2688, a2689) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f759(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2690 = v3;
        let a2691 = 61u64;
        if nat_eq(a2690, a2691) {
            let a2692 = f821(v1, 16u64);
            let a2693 = 0u64;
            nat_eq(a2692, a2693)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f822(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f760(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2694 = f821(v2, 4u64);
        let a2695 = 0u64;
        nat_eq(a2694, a2695)
    } else {
        false
    } {
        Ok(Some(f823(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f761(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2698 = {
            let a2696 = v0;
            let a2697 = 4u64;
            nat_mul(a2696, a2697)?
        };
        let a2699 = f824(v1, 16u64);
        nat_add(a2698, a2699)?
    }, List::cons({
        let a2702 = {
            let a2700 = f821(v1, 16u64);
            let a2701 = 16u64;
            nat_mul(a2700, a2701)?
        };
        let a2703 = f824(v2, 4u64);
        nat_add(a2702, a2703)?
    }, List::cons({
        let a2706 = {
            let a2704 = f821(v2, 4u64);
            let a2705 = 64u64;
            nat_mul(a2704, a2705)?
        };
        let a2707 = v3;
        nat_add(a2706, a2707)?
    }, List::<u64>::nil()))))
}

pub fn f762(v0: Adt6) -> bool {
    let m2708 = f141(v0);
    match m2708 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f763(v0: Adt6) -> bool {
    let m2709 = f696(v0);
    match m2709 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f764(v0: Adt6) -> bool {
    let m2710 = f215(v0);
    match m2710 {
        None => {
            false
        }
        Some(v1) => {
            let a2714 = {
                let m2713 = {
                    let a2711 = v1;
                    let a2712 = 0i64;
                    compare(a2711, a2712)
                };
                match m2713 {
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
            bool_not(a2714)
        }
    }
}

pub fn f765(v0: Adt6) -> bool {
    let m2715 = f223(v0);
    match m2715 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f766(v0: Adt6) -> bool {
    let m2716 = f219(v0);
    match m2716 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f767(v0: Adt6) -> bool {
    let m2717 = f223(v0);
    match m2717 {
        None => {
            false
        }
        Some(v1) => {
            f825(v1)
        }
    }
}

pub fn f768(v0: Adt6) -> bool {
    let m2718 = f443(v0);
    match m2718 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f769(v0: Adt6) -> bool {
    let m2719 = f141(v0.clone());
    match m2719 {
        None => {
            let m2720 = f223(v0);
            match m2720 {
                None => {
                    false
                }
                Some(v2) => {
                    f826(v2)
                }
            }
        }
        Some(v1) => {
            f691(v1)
        }
    }
}

pub fn f770(v0: Adt6) -> bool {
    let m2721 = f223(v0);
    match m2721 {
        None => {
            false
        }
        Some(v1) => {
            let m2722 = v1;
            match m2722.uncons() {
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

pub fn f771(v0: Str, p2740: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2739 = p2740;
    {
        let mut m2739 = m2739;
        loop {
            let step = {
                let v1 = m2739;
                let m2723 = v1;
                match m2723.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2727 = v0.clone();
                        let a2728 = {
                            let a2725 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2724, _) = v2.clone();
                                h2724
                            }, List::<Str>::nil()));
                            let a2726 = Str::lit("");
                            join(a2725, a2726)
                        };
                        if equal(a2727, a2728) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2729) = v2;
                                h2729
                            }))
                        } else {
                            let m2731 = f219({
                                let (_, h2730) = v2.clone();
                                h2730
                            });
                            match m2731 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2732 = f214(Str::lit("$anchor"), v4);
                                    match m2732 {
                                        Adt107::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt107::C1(v5) => {
                                            let m2733 = f141(v5);
                                            match m2733 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2736 = v0.clone();
                                                    let a2737 = {
                                                        let a2734 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2735 = Str::lit("");
                                                        join(a2734, a2735)
                                                    };
                                                    if equal(a2736, a2737) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2738) = v2;
                                                            h2738
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
                Ok(next) => { m2739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f772(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2741 = f296(Str::lit("type"), v0);
    match m2741 {
        Adt107::C0 => {
            None::<Adt13>
        }
        Adt107::C1(v1) => {
            let m2742 = f141(v1);
            match m2742 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f827(v2)
                }
            }
        }
    }
}

pub fn f773(v0: List<u64>) -> R<List<u64>> {
    Ok(f335(f828(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f774(v0: List<u64>, p2745: List<List<u64>>) -> bool {
    let m2744 = p2745;
    {
        let mut m2744 = m2744;
        loop {
            let step = {
                let v1 = m2744;
                let m2743 = v1;
                match m2743.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f829(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f775(v0: Adt7, v1: Adt7, v2: Adt156, v3: Adt156) -> R<Adt156> {
    let m2746 = v2;
    match m2746 {
        Adt156::C0 => {
            let m2748 = v3;
            match m2748 {
                Adt156::C0 => {
                    Ok(f830(Adt156::C0, f831(f832(match v0.clone() {
                        Adt7::C0(h2749, _) => {
                            h2749
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2750) => {
                            h2750
                        }
                    }, f832(match v1.clone() {
                        Adt7::C0(h2751, _) => {
                            h2751
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2752) => {
                            h2752
                        }
                    })?))
                }
                Adt156::C1 => {
                    Ok(Adt156::C0)
                }
                Adt156::C2 => {
                    Ok(Adt156::C0)
                }
            }
        }
        Adt156::C1 => {
            let m2747 = v3;
            match m2747 {
                Adt156::C0 => {
                    Ok(Adt156::C2)
                }
                Adt156::C1 => {
                    Ok(Adt156::C1)
                }
                Adt156::C2 => {
                    Ok(Adt156::C0)
                }
            }
        }
        Adt156::C2 => {
            let m2753 = v3;
            match m2753 {
                Adt156::C0 => {
                    Ok(Adt156::C2)
                }
                Adt156::C1 => {
                    Ok(Adt156::C2)
                }
                Adt156::C2 => {
                    f831(match v0.clone() {
                        Adt7::C0(h2754, _) => {
                            h2754
                        }
                    }, match v0 {
                        Adt7::C0(_, h2755) => {
                            h2755
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2756, _) => {
                            h2756
                        }
                    }, match v1 {
                        Adt7::C0(_, h2757) => {
                            h2757
                        }
                    })
                }
            }
        }
    }
}

pub fn f776(v0: i64) -> Adt156 {
    f783(v0, 0i64)
}

pub fn f777(v0: Adt6) -> Str {
    let m2758 = v0;
    match m2758 {
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

pub fn f778(v0: Adt7) -> R<bool> {
    if f297(match v0.clone() {
        Adt7::C0(h2759, _) => {
            h2759
        }
    }) {
        Ok(true)
    } else {
        f833(40u64, match v0.clone() {
            Adt7::C0(h2760, _) => {
                h2760
            }
        }, match v0 {
            Adt7::C0(_, h2761) => {
                h2761
            }
        })
    }
}

pub fn f779(v0: Adt6) -> Str {
    let m2762 = f141(v0);
    match m2762 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f780(p2769: u64, p2770: List<(Adt6, Adt6)>) -> R<Adt157> {
    let m2768 = (p2769, p2770);
    {
        let mut m2768 = m2768;
        loop {
            let step = {
                let (v0, v1) = m2768;
                let m2763 = v0;
                if m2763 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt157>>(Ok(Adt157::C2))
                } else {
                    let v2 = m2763 - 1;
                    let m2764 = v1;
                    match m2764.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt157>>(Ok(Adt157::C0))
                        }
                        Some((v3, v4)) => {
                            let m2767 = f834({
                                let (h2765, _) = v3;
                                h2765
                            }, {
                                let (_, h2766) = v3;
                                h2766
                            }, v4)?;
                            match m2767 {
                                Adt159::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt157>>(Ok(Adt157::C1))
                                }
                                Adt159::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt157>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f781() -> u64 {
    67108864u64
}

pub fn f782(v0: Adt156) -> bool {
    let m2771 = v0;
    match m2771 {
        Adt156::C0 => {
            true
        }
        Adt156::C1 => {
            false
        }
        Adt156::C2 => {
            false
        }
    }
}

pub fn f783(v0: i64, v1: i64) -> Adt156 {
    let m2774 = {
        let a2772 = v0;
        let a2773 = v1;
        compare(a2772, a2773)
    };
    if match m2774 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Adt156::C0
    } else {
        let m2777 = {
            let a2775 = v1;
            let a2776 = v0;
            compare(a2775, a2776)
        };
        if match m2777 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt156::C2
        } else {
            Adt156::C1
        }
    }
}

pub fn f784(p2786: List<u64>, p2787: u64) -> R<u64> {
    let m2785 = (p2786, p2787);
    {
        let mut m2785 = m2785;
        loop {
            let step = {
                let (v0, v1) = m2785;
                let m2778 = v0;
                match m2778.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2779 = v2;
                        let a2780 = 128u64;
                        if if nat_lt(a2779, a2780) {
                            true
                        } else {
                            let a2781 = 191u64;
                            let a2782 = v2;
                            nat_lt(a2781, a2782)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2783 = v1;
                                let a2784 = 1u64;
                                nat_add(a2783, a2784)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
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

pub fn f785(v0: u64, v1: List<u64>) -> Adt155 {
    let a2788 = v0;
    let a2789 = 32u64;
    if nat_lt(a2788, a2789) {
        if f835(v0) {
            Adt155::C0(0u64, List::cons(v0, v1))
        } else {
            Adt155::C0(0u64, f786(v1))
        }
    } else {
        let a2790 = v0;
        let a2791 = 127u64;
        if nat_eq(a2790, a2791) {
            Adt155::C0(0u64, f786(v1))
        } else {
            let a2792 = v0;
            let a2793 = 194u64;
            if nat_eq(a2792, a2793) {
                Adt155::C0(1u64, v1)
            } else {
                let a2794 = v0;
                let a2795 = 226u64;
                if nat_eq(a2794, a2795) {
                    Adt155::C0(2u64, v1)
                } else {
                    Adt155::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f786(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f787(v0: List<Adt42>) -> List<Adt6> {
    f378(f836(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f788(p2798: List<Adt2>, p2799: List<Adt6>) -> List<Adt6> {
    let m2797 = (p2798, p2799);
    {
        let mut m2797 = m2797;
        loop {
            let step = {
                let (v0, v1) = m2797;
                let m2796 = v0;
                match m2796.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f837(v2), v1)))
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

pub fn f789(v0: i64) -> u64 {
    let m2802 = {
        let a2800 = v0;
        let a2801 = 1i64;
        compare(a2800, a2801)
    };
    if match m2802 {
        Ordering::Less => {
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
        let m2805 = {
            let a2803 = v0;
            let a2804 = 2i64;
            compare(a2803, a2804)
        };
        if match m2805 {
            Ordering::Less => {
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
            let m2808 = {
                let a2806 = v0;
                let a2807 = 3i64;
                compare(a2806, a2807)
            };
            if match m2808 {
                Ordering::Less => {
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
                let m2811 = {
                    let a2809 = v0;
                    let a2810 = 4i64;
                    compare(a2809, a2810)
                };
                if match m2811 {
                    Ordering::Less => {
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
                    let m2814 = {
                        let a2812 = v0;
                        let a2813 = 5i64;
                        compare(a2812, a2813)
                    };
                    if match m2814 {
                        Ordering::Less => {
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
                        let m2817 = {
                            let a2815 = v0;
                            let a2816 = 6i64;
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
                            53u64
                        } else {
                            let m2820 = {
                                let a2818 = v0;
                                let a2819 = 7i64;
                                compare(a2818, a2819)
                            };
                            if match m2820 {
                                Ordering::Less => {
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
                                let m2823 = {
                                    let a2821 = v0;
                                    let a2822 = 8i64;
                                    compare(a2821, a2822)
                                };
                                if match m2823 {
                                    Ordering::Less => {
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
                                    let m2826 = {
                                        let a2824 = v0;
                                        let a2825 = 9i64;
                                        compare(a2824, a2825)
                                    };
                                    if match m2826 {
                                        Ordering::Less => {
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

pub fn f790(v0: Adt8) -> Adt6 {
    Adt6::C5(f301(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2827) => {
            h2827
        }
    }))
}

pub fn f791(v0: Adt36) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt36::C0(h2828, _, _, _, _, _, _, _) => {
            h2828
        }
    })), {
        let a2846 = f624(Str::lit("title"), match v0.clone() {
            Adt36::C0(_, h2829, _, _, _, _, _, _) => {
                h2829
            }
        });
        let a2847 = {
            let a2844 = f625(match v0.clone() {
                Adt36::C0(_, _, h2830, _, _, _, _, _) => {
                    h2830
                }
            });
            let a2845 = {
                let a2842 = f624(Str::lit("description"), match v0.clone() {
                    Adt36::C0(_, _, _, h2831, _, _, _, _) => {
                        h2831
                    }
                });
                let a2843 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2834 = match v0.clone() {
                        Adt36::C0(_, _, _, _, h2833, _, _, _) => {
                            h2833
                        }
                    };
                    match m2834 {
                        Adt5::C0(h2832) => {
                            h2832
                        }
                    }
                })), {
                    let a2840 = f665(Str::lit("outputSchema"), match v0.clone() {
                        Adt36::C0(_, _, _, _, _, h2835, _, _) => {
                            h2835
                        }
                    });
                    let a2841 = {
                        let a2838 = f838(match v0.clone() {
                            Adt36::C0(_, _, _, _, _, _, h2836, _) => {
                                h2836
                            }
                        });
                        let a2839 = f623(match v0 {
                            Adt36::C0(_, _, _, _, _, _, _, h2837) => {
                                h2837
                            }
                        });
                        append_list(a2838, a2839)
                    };
                    append_list(a2840, a2841)
                });
                append_list(a2842, a2843)
            };
            append_list(a2844, a2845)
        };
        append_list(a2846, a2847)
    }))
}

pub fn f792(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2871 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2848, _, _, _, _, _, _, _, _) => {
                h2848
            }
        })), List::<(Str, Adt6)>::nil());
        let a2872 = {
            let a2869 = f624(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2849, _, _, _, _, _, _, _) => {
                    h2849
                }
            });
            let a2870 = {
                let a2867 = f625(match v0.clone() {
                    Adt45::C0(_, _, h2850, _, _, _, _, _, _) => {
                        h2850
                    }
                });
                let a2868 = {
                    let a2865 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2851, _, _, _, _, _) => {
                            h2851
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2866 = {
                        let a2863 = f624(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2852, _, _, _, _) => {
                                h2852
                            }
                        });
                        let a2864 = {
                            let a2861 = f624(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2853, _, _, _) => {
                                    h2853
                                }
                            });
                            let a2862 = {
                                let a2859 = f622(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2854, _, _) => {
                                        h2854
                                    }
                                });
                                let a2860 = {
                                    let a2857 = f626(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2855, _) => {
                                            h2855
                                        }
                                    })?;
                                    let a2858 = f623(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2856) => {
                                            h2856
                                        }
                                    });
                                    append_list(a2857, a2858)
                                };
                                append_list(a2859, a2860)
                            };
                            append_list(a2861, a2862)
                        };
                        append_list(a2863, a2864)
                    };
                    append_list(a2865, a2866)
                };
                append_list(a2867, a2868)
            };
            append_list(a2869, a2870)
        };
        append_list(a2871, a2872)
    }))
}

pub fn f793(v0: Adt55) -> Adt6 {
    Adt6::C5({
        let a2893 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt55::C0(h2873, _, _, _, _, _, _, _) => {
                h2873
            }
        })), List::<(Str, Adt6)>::nil());
        let a2894 = {
            let a2891 = f624(Str::lit("title"), match v0.clone() {
                Adt55::C0(_, h2874, _, _, _, _, _, _) => {
                    h2874
                }
            });
            let a2892 = {
                let a2889 = f625(match v0.clone() {
                    Adt55::C0(_, _, h2875, _, _, _, _, _) => {
                        h2875
                    }
                });
                let a2890 = {
                    let a2887 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt55::C0(_, _, _, h2876, _, _, _, _) => {
                            h2876
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2888 = {
                        let a2885 = f624(Str::lit("description"), match v0.clone() {
                            Adt55::C0(_, _, _, _, h2877, _, _, _) => {
                                h2877
                            }
                        });
                        let a2886 = {
                            let a2883 = f624(Str::lit("mimeType"), match v0.clone() {
                                Adt55::C0(_, _, _, _, _, h2878, _, _) => {
                                    h2878
                                }
                            });
                            let a2884 = {
                                let a2881 = f622(match v0.clone() {
                                    Adt55::C0(_, _, _, _, _, _, h2879, _) => {
                                        h2879
                                    }
                                });
                                let a2882 = f623(match v0 {
                                    Adt55::C0(_, _, _, _, _, _, _, h2880) => {
                                        h2880
                                    }
                                });
                                append_list(a2881, a2882)
                            };
                            append_list(a2883, a2884)
                        };
                        append_list(a2885, a2886)
                    };
                    append_list(a2887, a2888)
                };
                append_list(a2889, a2890)
            };
            append_list(a2891, a2892)
        };
        append_list(a2893, a2894)
    })
}

pub fn f794(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2910 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2895, _, _, _, _, _) => {
                h2895
            }
        })), List::<(Str, Adt6)>::nil());
        let a2911 = {
            let a2908 = f624(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2896, _, _, _, _) => {
                    h2896
                }
            });
            let a2909 = {
                let a2906 = f625(match v0.clone() {
                    Adt57::C0(_, _, h2897, _, _, _) => {
                        h2897
                    }
                });
                let a2907 = {
                    let a2904 = f624(Str::lit("description"), match v0.clone() {
                        Adt57::C0(_, _, _, h2898, _, _) => {
                            h2898
                        }
                    });
                    let a2905 = {
                        let a2902 = {
                            let m2900 = match v0.clone() {
                                Adt57::C0(_, _, _, _, h2899, _) => {
                                    h2899
                                }
                            };
                            match m2900 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f839(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2903 = f623(match v0 {
                            Adt57::C0(_, _, _, _, _, h2901) => {
                                h2901
                            }
                        });
                        append_list(a2902, a2903)
                    };
                    append_list(a2904, a2905)
                };
                append_list(a2906, a2907)
            };
            append_list(a2908, a2909)
        };
        append_list(a2910, a2911)
    })
}

pub fn f795(v0: Adt60) -> R<Option<Adt6>> {
    let m2913 = f462(match v0.clone() {
        Adt60::C0(_, h2912) => {
            h2912
        }
    })?;
    match m2913 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f840(match v0 {
                Adt60::C0(h2914, _) => {
                    h2914
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f796(v0: Adt65) -> Option<Adt6> {
    let m2915 = v0;
    match m2915 {
        Adt65::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f841(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt65::C1(_) => {
            None::<Adt6>
        }
        Adt65::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m2916 = v3;
                match m2916 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h2917) => {
                                h2917
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f797(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f798(v0: Adt85) -> Option<Adt6> {
    let m2919 = f557(match v0.clone() {
        Adt85::C0(h2918, _, _) => {
            h2918
        }
    });
    match m2919 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m2921 = f842(match v0.clone() {
                Adt85::C0(_, _, h2920) => {
                    h2920
                }
            });
            match m2921 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f797(match v0 {
                        Adt85::C0(_, h2922, _) => {
                            h2922
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f799(v0: List<u64>) -> List<u64> {
    let a2925 = f843();
    let a2926 = {
        let a2923 = v0;
        let a2924 = f844();
        append_list(a2923, a2924)
    };
    append_list(a2925, a2926)
}

pub fn f800(p2933: List<u64>) -> bool {
    let m2932 = p2933;
    {
        let mut m2932 = m2932;
        loop {
            let step = {
                let v0 = m2932;
                let m2927 = v0;
                match m2927.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2928 = v1;
                        let a2929 = 48u64;
                        if if nat_eq(a2928, a2929) {
                            true
                        } else {
                            let a2930 = v1;
                            let a2931 = 46u64;
                            nat_eq(a2930, a2931)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f801(v0: Adt146) -> bool {
    let m2934 = v0;
    match m2934 {
        Adt146::C0 => {
            true
        }
        Adt146::C1 => {
            false
        }
        Adt146::C2 => {
            false
        }
        Adt146::C3 => {
            false
        }
        Adt146::C4 => {
            false
        }
        Adt146::C5 => {
            false
        }
        Adt146::C6 => {
            false
        }
        Adt146::C7 => {
            false
        }
        Adt146::C8 => {
            false
        }
        Adt146::C9 => {
            false
        }
        Adt146::C10 => {
            false
        }
        Adt146::C11 => {
            false
        }
    }
}

pub fn f802(v0: Adt15) -> Adt146 {
    let m2935 = v0;
    match m2935 {
        Adt15::C0(_) => {
            Adt146::C0
        }
        Adt15::C1(_) => {
            Adt146::C1
        }
        Adt15::C2(_) => {
            Adt146::C2
        }
        Adt15::C3(_) => {
            Adt146::C3
        }
        Adt15::C4(_) => {
            Adt146::C4
        }
        Adt15::C5(_) => {
            Adt146::C5
        }
        Adt15::C6(_) => {
            Adt146::C6
        }
        Adt15::C7(_) => {
            Adt146::C7
        }
        Adt15::C8(_) => {
            Adt146::C8
        }
        Adt15::C9(_) => {
            Adt146::C9
        }
        Adt15::C10(_) => {
            Adt146::C10
        }
        Adt15::C11(_) => {
            Adt146::C11
        }
    }
}

pub fn f803(v0: Adt0, v1: Adt33) -> Adt134 {
    Adt134::C0(v1, Adt135::C0(Some(match v0 {
        Adt0::C0(h2936, _, _, _, _, _) => {
            h2936
        }
    }), None::<Adt5>), List::<Adt136>::nil())
}

pub fn f804(v0: Adt0) -> Adt34 {
    Adt34::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2937, _, _) => {
            h2937
        }
    }, f346(), match v0.clone() {
        Adt0::C0(_, _, h2938, _, _, _) => {
            h2938
        }
    }, match v0 {
        Adt0::C0(_, h2939, _, _, _, _) => {
            h2939
        }
    })
}

pub fn f805(v0: Adt0, v1: Adt15, v2: Result<Adt33, Adt90>) -> Result<Adt134, Adt137> {
    let m2940 = v2;
    match m2940 {
        Ok(v4) => {
            if f845(f802(v1.clone()), v4.clone()) {
                f846(v0, v1, v4)
            } else {
                Err::<Adt134, Adt137>(Adt137::C4(f847()))
            }
        }
        Err(v3) => {
            Err::<Adt134, Adt137>(Adt137::C4(v3))
        }
    }
}

pub fn f806(v0: u64, v1: u64, v2: Fn3, v3: Adt15) -> R<Result<Adt33, Adt90>> {
    let m2941 = v3.clone();
    match m2941 {
        Adt15::C0(_) => {
            let c2943: Fn3 = v2;
            c2943.apply(v3)
        }
        Adt15::C1(_) => {
            let c2944: Fn3 = v2;
            c2944.apply(v3)
        }
        Adt15::C2(_) => {
            if f848(v0, v1) {
                let c2942: Fn3 = v2;
                c2942.apply(v3)
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C7(v0)))
            }
        }
        Adt15::C3(_) => {
            let c2945: Fn3 = v2;
            c2945.apply(v3)
        }
        Adt15::C4(_) => {
            let c2946: Fn3 = v2;
            c2946.apply(v3)
        }
        Adt15::C5(_) => {
            let c2947: Fn3 = v2;
            c2947.apply(v3)
        }
        Adt15::C6(_) => {
            let c2948: Fn3 = v2;
            c2948.apply(v3)
        }
        Adt15::C7(_) => {
            let c2949: Fn3 = v2;
            c2949.apply(v3)
        }
        Adt15::C8(_) => {
            let c2950: Fn3 = v2;
            c2950.apply(v3)
        }
        Adt15::C9(_) => {
            let c2951: Fn3 = v2;
            c2951.apply(v3)
        }
        Adt15::C10(_) => {
            let c2952: Fn3 = v2;
            c2952.apply(v3)
        }
        Adt15::C11(_) => {
            let c2953: Fn3 = v2;
            c2953.apply(v3)
        }
    }
}

pub fn f807(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt138> {
    let m2954 = f214(v0.clone(), v1);
    match m2954 {
        Adt107::C0 => {
            Ok::<Option<Str>, Adt138>(None::<Str>)
        }
        Adt107::C1(v2) => {
            let m2955 = f141(v2);
            match m2955 {
                None => {
                    Err::<Option<Str>, Adt138>(Adt138::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt138>(Some(v3))
                }
            }
        }
    }
}

pub fn f808(v0: Str, v1: Adt5) -> Result<Str, Adt138> {
    let m2956 = f214(v0.clone(), v1);
    match m2956 {
        Adt107::C0 => {
            Err::<Str, Adt138>(Adt138::C7(v0))
        }
        Adt107::C1(v2) => {
            let m2957 = f141(v2);
            match m2957 {
                None => {
                    Err::<Str, Adt138>(Adt138::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt138>(v3)
                }
            }
        }
    }
}

pub fn f809(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt138> {
    let m2958 = f214(v0.clone(), v1);
    match m2958 {
        Adt107::C0 => {
            Ok::<Option<Adt5>, Adt138>(None::<Adt5>)
        }
        Adt107::C1(v2) => {
            let m2959 = f219(v2);
            match m2959 {
                None => {
                    Err::<Option<Adt5>, Adt138>(Adt138::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt138>(Some(v3))
                }
            }
        }
    }
}

pub fn f810(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt138> {
    let m2960 = f214(v0.clone(), v1);
    match m2960 {
        Adt107::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt138>(None::<List<(Str, Str)>>)
        }
        Adt107::C1(v2) => {
            let m2961 = f219(v2);
            match m2961 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt138>(Adt138::C8(v0))
                }
                Some(v3) => {
                    let m2963 = f849(v0, match v3 {
                        Adt5::C0(h2962) => {
                            h2962
                        }
                    });
                    match m2963 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt138>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt138>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f811(v0: Str, v1: Adt5) -> Result<Adt5, Adt138> {
    let m2964 = f214(v0.clone(), v1);
    match m2964 {
        Adt107::C0 => {
            Err::<Adt5, Adt138>(Adt138::C7(v0))
        }
        Adt107::C1(v2) => {
            let m2965 = f219(v2);
            match m2965 {
                None => {
                    Err::<Adt5, Adt138>(Adt138::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt138>(v3)
                }
            }
        }
    }
}

pub fn f812(v0: Adt5) -> Result<Adt26, Adt138> {
    let m2966 = f808(Str::lit("type"), v0.clone());
    match m2966 {
        Ok(v1) => {
            let a2967 = v1.clone();
            let a2968 = Str::lit("ref/prompt");
            if equal(a2967, a2968) {
                let m2969 = f808(Str::lit("name"), v0.clone());
                match m2969 {
                    Ok(v2) => {
                        let m2970 = f807(Str::lit("title"), v0);
                        match m2970 {
                            Ok(v3) => {
                                Ok::<Adt26, Adt138>(Adt26::C0(Adt27::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt26, Adt138>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt26, Adt138>(v5)
                    }
                }
            } else {
                let a2971 = v1;
                let a2972 = Str::lit("ref/resource");
                if equal(a2971, a2972) {
                    let m2973 = f808(Str::lit("uri"), v0);
                    match m2973 {
                        Ok(v6) => {
                            Ok::<Adt26, Adt138>(Adt26::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt26, Adt138>(v7)
                        }
                    }
                } else {
                    Err::<Adt26, Adt138>(Adt138::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt26, Adt138>(v8)
        }
    }
}

pub fn f813(v0: Adt5) -> Result<Adt28, Adt138> {
    let m2974 = f808(Str::lit("name"), v0.clone());
    match m2974 {
        Ok(v1) => {
            let m2975 = f808(Str::lit("value"), v0);
            match m2975 {
                Ok(v2) => {
                    Ok::<Adt28, Adt138>(Adt28::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt28, Adt138>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt28, Adt138>(v4)
        }
    }
}

pub fn f814(v0: Adt5) -> Result<Option<Adt29>, Adt138> {
    let m2976 = f214(Str::lit("context"), v0);
    match m2976 {
        Adt107::C0 => {
            Ok::<Option<Adt29>, Adt138>(None::<Adt29>)
        }
        Adt107::C1(v1) => {
            let m2977 = f219(v1);
            match m2977 {
                None => {
                    Err::<Option<Adt29>, Adt138>(Adt138::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2978 = f810(Str::lit("arguments"), v2);
                    match m2978 {
                        Ok(v3) => {
                            Ok::<Option<Adt29>, Adt138>(Some(Adt29::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt29>, Adt138>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f815(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt137> {
    if v0 {
        None::<Adt137>
    } else {
        Some(Adt137::C1(v1, v2))
    }
}

pub fn f816(v0: Str, p2983: List<Str>) -> bool {
    let m2982 = p2983;
    {
        let mut m2982 = m2982;
        loop {
            let step = {
                let v1 = m2982;
                let m2979 = v1;
                match m2979.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2980 = v2;
                        let a2981 = v0.clone();
                        if equal(a2980, a2981) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
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

pub fn f817(v0: Option<Adt117>, v1: List<Adt119>, v2: Adt5) -> R<Option<Adt117>> {
    let m2984 = v0;
    match m2984 {
        None => {
            f850(v1, f851(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f818(v0: Option<Str>, v1: Adt151, v2: Adt5) -> Option<Adt117> {
    let m2985 = v1.clone();
    match m2985 {
        Adt151::C0 => {
            None::<Adt117>
        }
        Adt151::C1 => {
            f852(v0, f853(v1, v2))
        }
        Adt151::C2 => {
            f852(v0, f853(v1, v2))
        }
        Adt151::C3 => {
            f852(v0, f853(v1, v2))
        }
    }
}

pub fn f819(v0: Adt107, v1: Adt107) -> Adt158 {
    let m2986 = v0;
    match m2986 {
        Adt107::C0 => {
            Adt158::C1
        }
        Adt107::C1(_) => {
            let m2987 = v1;
            match m2987 {
                Adt107::C0 => {
                    Adt158::C2
                }
                Adt107::C1(_) => {
                    Adt158::C0
                }
            }
        }
    }
}

pub fn f820(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt17, Adt138> {
    let m2988 = f141(v1);
    match m2988 {
        None => {
            Err::<Adt17, Adt138>(Adt138::C5)
        }
        Some(v3) => {
            let m2989 = f854(v2);
            match m2989 {
                None => {
                    Err::<Adt17, Adt138>(Adt138::C6)
                }
                Some(v4) => {
                    let m2990 = f809(f855(), v0.clone());
                    match m2990 {
                        Ok(v6) => {
                            let m2991 = f856(v0.clone());
                            match m2991 {
                                Ok(v8) => {
                                    let m2992 = f857(v0.clone());
                                    match m2992 {
                                        Ok(v10) => {
                                            Ok::<Adt17, Adt138>(Adt17::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt17, Adt138>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt17, Adt138>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt17, Adt138>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f821(v0: u64, v1: u64) -> u64 {
    let a2993 = v0;
    let a2994 = v1;
    let a2995 = 0u64;
    nat_rem(a2993, a2994, a2995)
}

pub fn f822(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2998 = {
            let a2996 = v0;
            let a2997 = 4u64;
            nat_mul(a2996, a2997)?
        };
        let a2999 = f824(v1, 16u64);
        nat_add(a2998, a2999)?
    }, List::<u64>::nil()))
}

pub fn f823(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a3002 = {
            let a3000 = v0;
            let a3001 = 4u64;
            nat_mul(a3000, a3001)?
        };
        let a3003 = f824(v1, 16u64);
        nat_add(a3002, a3003)?
    }, List::cons({
        let a3006 = {
            let a3004 = f821(v1, 16u64);
            let a3005 = 16u64;
            nat_mul(a3004, a3005)?
        };
        let a3007 = f824(v2, 4u64);
        nat_add(a3006, a3007)?
    }, List::<u64>::nil())))
}

pub fn f824(v0: u64, v1: u64) -> u64 {
    let a3008 = v0;
    let a3009 = v1;
    let a3010 = 0u64;
    nat_quot(a3008, a3009, a3010)
}

pub fn f825(p3014: List<Adt6>) -> bool {
    let m3013 = p3014;
    {
        let mut m3013 = m3013;
        loop {
            let step = {
                let v0 = m3013;
                let m3011 = v0;
                match m3011.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3012 = f141(v1);
                        match m3012 {
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
                Ok(next) => { m3013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f826(p3018: List<Adt6>) -> bool {
    let m3017 = p3018;
    {
        let mut m3017 = m3017;
        loop {
            let step = {
                let v0 = m3017;
                let m3015 = v0;
                match m3015.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3016 = f141(v1);
                        match m3016 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f691(v3) {
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
                Ok(next) => { m3017 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f827(v0: Str) -> Option<Adt13> {
    let a3019 = v0.clone();
    let a3020 = Str::lit("string");
    if equal(a3019, a3020) {
        Some(Adt13::C0)
    } else {
        let a3021 = v0.clone();
        let a3022 = Str::lit("integer");
        if equal(a3021, a3022) {
            Some(Adt13::C1)
        } else {
            let a3023 = v0;
            let a3024 = Str::lit("boolean");
            if equal(a3023, a3024) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f828(p3033: List<u64>, p3034: List<u64>) -> R<List<u64>> {
    let m3032 = (p3033, p3034);
    {
        let mut m3032 = m3032;
        loop {
            let step = {
                let (v0, v1) = m3032;
                let m3025 = v0;
                match m3025.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a3026 = 65u64;
                        let a3027 = v2;
                        if if nat_le(a3026, a3027) {
                            let a3028 = v2;
                            let a3029 = 90u64;
                            nat_le(a3028, a3029)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a3030 = v2;
                                let a3031 = 32u64;
                                nat_add(a3030, a3031)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f829(p3041: List<u64>, p3042: List<u64>) -> bool {
    let m3040 = (p3041, p3042);
    {
        let mut m3040 = m3040;
        loop {
            let step = {
                let (v0, v1) = m3040;
                let m3035 = v0;
                match m3035.uncons() {
                    None => {
                        let m3036 = v1;
                        match m3036.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m3037 = v1;
                        match m3037.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a3038 = v4;
                                let a3039 = v6;
                                if nat_eq(a3038, a3039) {
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
                Ok(next) => { m3040 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f830(v0: Adt156, v1: Adt156) -> Adt156 {
    let m3043 = v0;
    match m3043 {
        Adt156::C0 => {
            f858(v1)
        }
        Adt156::C1 => {
            Adt156::C1
        }
        Adt156::C2 => {
            v1
        }
    }
}

pub fn f831(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt156> {
    let m3044 = f783(f859(v0, v1)?, f859(v2, v3)?);
    match m3044 {
        Adt156::C0 => {
            Ok(Adt156::C0)
        }
        Adt156::C1 => {
            f860(v0, f861(v0)?, v2, f861(v2)?)
        }
        Adt156::C2 => {
            Ok(Adt156::C2)
        }
    }
}

pub fn f832(v0: i64) -> R<i64> {
    let m3047 = {
        let a3045 = v0;
        let a3046 = 0i64;
        compare(a3045, a3046)
    };
    if match m3047 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a3048 = v0;
        int_neg(a3048)
    } else {
        Ok(v0)
    }
}

pub fn f833(p3060: u64, p3061: i64, p3062: i64) -> R<bool> {
    let m3059 = (p3060, (p3061, p3062));
    {
        let mut m3059 = m3059;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3059;
                let m3049 = v0;
                if m3049 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m3049 - 1;
                    let m3052 = {
                        let a3050 = v2;
                        let a3051 = 0i64;
                        compare(a3050, a3051)
                    };
                    if match m3052 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f297({
                            let a3053 = v1;
                            let a3054 = 10i64;
                            let a3055 = 0i64;
                            int_rem(a3053, a3054, a3055)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a3056 = v1;
                                let a3057 = 10i64;
                                let a3058 = 0i64;
                                int_quot(a3056, a3057, a3058)?
                            }, f302(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m3059 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f834(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt159> {
    let a3063 = f862(v0.clone());
    let a3064 = f862(v1.clone());
    if nat_eq(a3063, a3064) {
        let m3065 = v0;
        match m3065 {
            Adt6::C0 => {
                Ok(Adt159::C1(v2))
            }
            Adt6::C1(v3) => {
                let m3066 = f443(v1);
                match m3066 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v4) => {
                        let a3067 = v3;
                        let a3068 = v4;
                        if equal(a3067, a3068) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m3069 = f696(v1);
                match m3069 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v6) => {
                        if f863(f690(v5, v6)?) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m3070 = f141(v1);
                match m3070 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v8) => {
                        let a3071 = v7;
                        let a3072 = v8;
                        if equal(a3071, a3072) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m3073 = f223(v1);
                match m3073 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v10) => {
                        Ok(f864(f865(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m3074 = f219(v1);
                match m3074 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v12) => {
                        let a3076 = f701(v11.clone(), 0u64)?;
                        let a3077 = f701(match v12.clone() {
                            Adt5::C0(h3075) => {
                                h3075
                            }
                        }, 0u64)?;
                        if nat_eq(a3076, a3077) {
                            Ok(f864(f866(v11, match v12 {
                                Adt5::C0(h3078) => {
                                    h3078
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt159::C0)
    }
}

pub fn f835(v0: u64) -> bool {
    let a3079 = v0;
    let a3080 = 9u64;
    if nat_eq(a3079, a3080) {
        true
    } else {
        let a3081 = v0;
        let a3082 = 10u64;
        if nat_eq(a3081, a3082) {
            true
        } else {
            let a3083 = v0;
            let a3084 = 13u64;
            nat_eq(a3083, a3084)
        }
    }
}

pub fn f836(p3087: List<Adt42>, p3088: List<Adt6>) -> List<Adt6> {
    let m3086 = (p3087, p3088);
    {
        let mut m3086 = m3086;
        loop {
            let step = {
                let (v0, v1) = m3086;
                let m3085 = v0;
                match m3085.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt42>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f840(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3086 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f837(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h3089, _, _, _) => {
            h3089
        }
    })), {
        let a3095 = f624(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h3090, _, _) => {
                h3090
            }
        });
        let a3096 = {
            let a3093 = f867(match v0.clone() {
                Adt2::C0(_, _, h3091, _) => {
                    h3091
                }
            });
            let a3094 = f868(match v0 {
                Adt2::C0(_, _, _, h3092) => {
                    h3092
                }
            });
            append_list(a3093, a3094)
        };
        append_list(a3095, a3096)
    }))
}

pub fn f838(v0: Option<Adt37>) -> List<(Str, Adt6)> {
    let m3097 = v0;
    match m3097 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a3109 = f624(Str::lit("title"), match v1.clone() {
                    Adt37::C0(h3098, _, _, _, _) => {
                        h3098
                    }
                });
                let a3110 = {
                    let a3107 = f301(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt37::C0(_, h3099, _, _, _) => {
                            h3099
                        }
                    });
                    let a3108 = {
                        let a3105 = f301(Str::lit("destructiveHint"), match v1.clone() {
                            Adt37::C0(_, _, h3100, _, _) => {
                                h3100
                            }
                        });
                        let a3106 = {
                            let a3103 = f301(Str::lit("idempotentHint"), match v1.clone() {
                                Adt37::C0(_, _, _, h3101, _) => {
                                    h3101
                                }
                            });
                            let a3104 = f301(Str::lit("openWorldHint"), match v1 {
                                Adt37::C0(_, _, _, _, h3102) => {
                                    h3102
                                }
                            });
                            append_list(a3103, a3104)
                        };
                        append_list(a3105, a3106)
                    };
                    append_list(a3107, a3108)
                };
                append_list(a3109, a3110)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f839(v0: List<Adt58>) -> List<Adt6> {
    f378(f869(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f840(v0: Adt42) -> Str {
    let m3111 = v0;
    match m3111 {
        Adt42::C0 => {
            Str::lit("user")
        }
        Adt42::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f841(v0: Adt66) -> List<(Str, Adt6)> {
    let m3112 = v0;
    match m3112 {
        Adt66::C0(v1) => {
            let a3119 = {
                let m3114 = match v1.clone() {
                    Adt67::C0(h3113, _, _) => {
                        h3113
                    }
                };
                match m3114 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3120 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt67::C0(_, h3115, _) => {
                    h3115
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m3118 = match v1 {
                    Adt67::C0(_, _, h3117) => {
                        h3117
                    }
                };
                match m3118 {
                    Adt5::C0(h3116) => {
                        h3116
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a3119, a3120)
        }
        Adt66::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt69::C0(h3121, _) => {
                    h3121
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt69::C0(_, h3122) => {
                    h3122
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f842(v0: Adt86) -> Option<Adt6> {
    let m3124 = f643(match v0.clone() {
        Adt86::C0(_, h3123, _, _) => {
            h3123
        }
    });
    match m3124 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f797(match v0.clone() {
                Adt86::C0(h3125, _, _, _) => {
                    h3125
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f797(match v0.clone() {
                Adt86::C0(_, _, h3126, _) => {
                    h3126
                }
            })), List::cons((Str::lit("outcome"), f870(match v0 {
                Adt86::C0(_, _, _, h3127) => {
                    h3127
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f843() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f844() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f845(v0: Adt146, v1: Adt33) -> bool {
    let m3128 = v1;
    match m3128 {
        Adt33::C0(_) => {
            f871(v0, Adt146::C0)
        }
        Adt33::C1(_) => {
            f871(v0, Adt146::C1)
        }
        Adt33::C2(_) => {
            f871(v0, Adt146::C2)
        }
        Adt33::C3(_) => {
            f871(v0, Adt146::C2)
        }
        Adt33::C4(_) => {
            f871(v0, Adt146::C3)
        }
        Adt33::C5(_) => {
            f871(v0, Adt146::C4)
        }
        Adt33::C6(_) => {
            f871(v0, Adt146::C5)
        }
        Adt33::C7(_) => {
            f871(v0, Adt146::C6)
        }
        Adt33::C8(_) => {
            f871(v0, Adt146::C7)
        }
        Adt33::C9(_) => {
            f871(v0, Adt146::C8)
        }
        Adt33::C10(_) => {
            f871(v0, Adt146::C9)
        }
        Adt33::C11 => {
            if f871(v0.clone(), Adt146::C10) {
                true
            } else {
                f871(v0, Adt146::C11)
            }
        }
        Adt33::C12(_) => {
            f872(v0)
        }
        Adt33::C13(_) => {
            f873(v0)
        }
        Adt33::C14(v15) => {
            let m3130 = match v15 {
                Adt88::C0(_, h3129, _) => {
                    h3129
                }
            };
            match m3130 {
                Adt89::C0 => {
                    f871(v0, Adt146::C9)
                }
                Adt89::C1 => {
                    if f871(v0.clone(), Adt146::C10) {
                        true
                    } else {
                        f871(v0, Adt146::C11)
                    }
                }
            }
        }
    }
}

pub fn f846(v0: Adt0, v1: Adt15, v2: Adt33) -> Result<Adt134, Adt137> {
    let m3131 = v2.clone();
    match m3131 {
        Adt33::C0(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C1(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C2(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C3(v9) => {
            Ok::<Adt134, Adt137>(f878(v0, f875(v1), v9))
        }
        Adt33::C4(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C5(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C6(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C7(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C8(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C9(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C10(_) => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C11 => {
            Ok::<Adt134, Adt137>(f803(v0, v2))
        }
        Adt33::C12(v3) => {
            f874(v0, f875(v1), v3)
        }
        Adt33::C13(v4) => {
            f876(v0, f875(v1), v4)
        }
        Adt33::C14(v5) => {
            f877(v0, f875(v1), v5)
        }
    }
}

pub fn f847() -> Adt90 {
    Adt90::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f848(v0: u64, v1: u64) -> bool {
    let a3132 = v1;
    let a3133 = v0;
    nat_lt(a3132, a3133)
}

pub fn f849(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt138> {
    let m3134 = f879(v0, v1, List::<(Str, Str)>::nil());
    match m3134 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt138>(f880(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt138>(v3)
        }
    }
}

pub fn f850(p3138: List<Adt119>, v1: Adt5) -> R<Option<Adt117>> {
    let m3137 = p3138;
    {
        let mut m3137 = m3137;
        loop {
            let step = {
                let v0 = m3137;
                let m3135 = v0;
                match m3135.uncons() {
                    None => {
                        Err::<List<Adt119>, R<Option<Adt117>>>(Ok(None::<Adt117>))
                    }
                    Some((v2, v3)) => {
                        let m3136 = f881(v2, v1.clone())?;
                        match m3136 {
                            None => {
                                Ok::<List<Adt119>, R<Option<Adt117>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt119>, R<Option<Adt117>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f851(v0: Adt5) -> Adt5 {
    let m3139 = f214(Str::lit("arguments"), v0);
    match m3139 {
        Adt107::C0 => {
            f27()
        }
        Adt107::C1(v1) => {
            let m3140 = f219(v1);
            match m3140 {
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

pub fn f852(v0: Option<Str>, v1: Option<Str>) -> Option<Adt117> {
    let m3141 = v0;
    match m3141 {
        None => {
            Some(Adt117::C2)
        }
        Some(v2) => {
            let m3142 = v1;
            match m3142 {
                None => {
                    None::<Adt117>
                }
                Some(v3) => {
                    let a3143 = v2;
                    let a3144 = v3;
                    if equal(a3143, a3144) {
                        None::<Adt117>
                    } else {
                        Some(Adt117::C5)
                    }
                }
            }
        }
    }
}

pub fn f853(v0: Adt151, v1: Adt5) -> Option<Str> {
    let m3145 = v0;
    match m3145 {
        Adt151::C0 => {
            None::<Str>
        }
        Adt151::C1 => {
            let m3146 = f214(Str::lit("name"), v1);
            match m3146 {
                Adt107::C0 => {
                    None::<Str>
                }
                Adt107::C1(v2) => {
                    f141(v2)
                }
            }
        }
        Adt151::C2 => {
            let m3147 = f214(Str::lit("uri"), v1);
            match m3147 {
                Adt107::C0 => {
                    None::<Str>
                }
                Adt107::C1(v3) => {
                    f141(v3)
                }
            }
        }
        Adt151::C3 => {
            let m3148 = f214(Str::lit("taskId"), v1);
            match m3148 {
                Adt107::C0 => {
                    None::<Str>
                }
                Adt107::C1(v4) => {
                    f141(v4)
                }
            }
        }
    }
}

pub fn f854(v0: Adt6) -> Option<Adt18> {
    let m3149 = f219(v0);
    match m3149 {
        None => {
            None::<Adt18>
        }
        Some(v1) => {
            let m3150 = f882(Str::lit("experimental"), v1.clone());
            match m3150 {
                None => {
                    None::<Adt18>
                }
                Some(v2) => {
                    let m3151 = f882(Str::lit("roots"), v1.clone());
                    match m3151 {
                        None => {
                            None::<Adt18>
                        }
                        Some(v3) => {
                            let m3152 = f882(Str::lit("sampling"), v1.clone());
                            match m3152 {
                                None => {
                                    None::<Adt18>
                                }
                                Some(v4) => {
                                    let m3153 = f882(Str::lit("elicitation"), v1.clone());
                                    match m3153 {
                                        None => {
                                            None::<Adt18>
                                        }
                                        Some(v5) => {
                                            let m3154 = f882(Str::lit("extensions"), v1);
                                            match m3154 {
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

pub fn f855() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f856(v0: Adt5) -> Result<Option<Adt19>, Adt138> {
    let m3155 = f807(f883(), v0);
    match m3155 {
        Ok(v2) => {
            let m3156 = v2;
            match m3156 {
                None => {
                    Ok::<Option<Adt19>, Adt138>(None::<Adt19>)
                }
                Some(v3) => {
                    let m3157 = f884(v3);
                    match m3157 {
                        Adt160::C0 => {
                            Err::<Option<Adt19>, Adt138>(Adt138::C8(f883()))
                        }
                        Adt160::C1(v4) => {
                            Ok::<Option<Adt19>, Adt138>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt19>, Adt138>(v1)
        }
    }
}

pub fn f857(v0: Adt5) -> Result<Option<Adt20>, Adt138> {
    let m3158 = f214(f885(), v0);
    match m3158 {
        Adt107::C0 => {
            Ok::<Option<Adt20>, Adt138>(None::<Adt20>)
        }
        Adt107::C1(v1) => {
            let m3159 = f141(v1.clone());
            match m3159 {
                None => {
                    let m3160 = f215(v1);
                    match m3160 {
                        None => {
                            Err::<Option<Adt20>, Adt138>(Adt138::C8(f885()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt20>, Adt138>(Some(Adt20::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt20>, Adt138>(Some(Adt20::C0(v2)))
                }
            }
        }
    }
}

pub fn f858(v0: Adt156) -> Adt156 {
    let m3161 = v0;
    match m3161 {
        Adt156::C0 => {
            Adt156::C2
        }
        Adt156::C1 => {
            Adt156::C1
        }
        Adt156::C2 => {
            Adt156::C0
        }
    }
}

pub fn f859(v0: i64, v1: i64) -> R<i64> {
    f302(f221(f861(v0)?, 0i64)?, v1)
}

pub fn f860(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt156> {
    let a3162 = v1;
    let a3163 = v3;
    if nat_lt(a3162, a3163) {
        Ok(f783(f886({
            let a3164 = v3;
            let a3165 = v1;
            nat_sub(a3164, a3165)
        }, v0)?, v2))
    } else {
        let a3166 = v3;
        let a3167 = v1;
        if nat_lt(a3166, a3167) {
            Ok(f783(v0, f886({
                let a3168 = v1;
                let a3169 = v3;
                nat_sub(a3168, a3169)
            }, v2)?))
        } else {
            Ok(f783(v0, v2))
        }
    }
}

pub fn f861(v0: i64) -> R<u64> {
    f887(20u64, v0, 0u64)
}

pub fn f862(v0: Adt6) -> u64 {
    let m3170 = v0;
    match m3170 {
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

pub fn f863(v0: Adt156) -> bool {
    let m3171 = v0;
    match m3171 {
        Adt156::C0 => {
            false
        }
        Adt156::C1 => {
            true
        }
        Adt156::C2 => {
            false
        }
    }
}

pub fn f864(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt159 {
    let m3172 = v0;
    match m3172 {
        None => {
            Adt159::C0
        }
        Some(v2) => {
            Adt159::C1({
                let a3173 = v2;
                let a3174 = v1;
                append_list(a3173, a3174)
            })
        }
    }
}

pub fn f865(p3179: List<Adt6>, p3180: List<Adt6>, p3181: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3178 = (p3179, (p3180, p3181));
    {
        let mut m3178 = m3178;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3178;
                let m3175 = v0;
                match m3175.uncons() {
                    None => {
                        let m3176 = v1;
                        match m3176.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m3177 = v1;
                        match m3177.uncons() {
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
                Ok(next) => { m3178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f866(p3187: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p3188: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3186 = (p3187, p3188);
    {
        let mut m3186 = m3186;
        loop {
            let step = {
                let (v0, v2) = m3186;
                let m3182 = v0;
                match m3182.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m3184 = f296({
                            let (h3183, _) = v3;
                            h3183
                        }, v1.clone());
                        match m3184 {
                            Adt107::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt107::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h3185) = v3;
                                    h3185
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f867(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3189 = v0;
    match m3189 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f577(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f868(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3190 = v0;
    match m3190 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f888(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f869(p3193: List<Adt58>, p3194: List<Adt6>) -> List<Adt6> {
    let m3192 = (p3193, p3194);
    {
        let mut m3192 = m3192;
        loop {
            let step = {
                let (v0, v1) = m3192;
                let m3191 = v0;
                match m3191.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f889(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3192 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f870(v0: Adt87) -> Adt6 {
    let m3195 = v0;
    match m3195 {
        Adt87::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h3196) => {
                    h3196
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt87::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f261(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f871(v0: Adt146, v1: Adt146) -> bool {
    let a3197 = f669(v0);
    let a3198 = f669(v1);
    equal(a3197, a3198)
}

pub fn f872(v0: Adt146) -> bool {
    let m3199 = v0;
    match m3199 {
        Adt146::C0 => {
            false
        }
        Adt146::C1 => {
            false
        }
        Adt146::C2 => {
            true
        }
        Adt146::C3 => {
            false
        }
        Adt146::C4 => {
            true
        }
        Adt146::C5 => {
            false
        }
        Adt146::C6 => {
            false
        }
        Adt146::C7 => {
            true
        }
        Adt146::C8 => {
            false
        }
        Adt146::C9 => {
            false
        }
        Adt146::C10 => {
            false
        }
        Adt146::C11 => {
            false
        }
    }
}

pub fn f873(v0: Adt146) -> bool {
    let m3200 = v0;
    match m3200 {
        Adt146::C0 => {
            false
        }
        Adt146::C1 => {
            false
        }
        Adt146::C2 => {
            true
        }
        Adt146::C3 => {
            false
        }
        Adt146::C4 => {
            false
        }
        Adt146::C5 => {
            false
        }
        Adt146::C6 => {
            false
        }
        Adt146::C7 => {
            false
        }
        Adt146::C8 => {
            false
        }
        Adt146::C9 => {
            false
        }
        Adt146::C10 => {
            false
        }
        Adt146::C11 => {
            false
        }
    }
}

pub fn f874(v0: Adt0, v1: Adt17, v2: Adt82) -> Result<Adt134, Adt137> {
    f890(v0, v2.clone(), f891(match v1 {
        Adt17::C0(_, h3201, _, _, _, _) => {
            h3201
        }
    }, f644(v2)))
}

pub fn f875(v0: Adt15) -> Adt17 {
    let m3202 = v0;
    match m3202 {
        Adt15::C0(v1) => {
            match v1 {
                Adt16::C0(h3203) => {
                    h3203
                }
            }
        }
        Adt15::C1(v2) => {
            match v2 {
                Adt21::C0(h3204, _) => {
                    h3204
                }
            }
        }
        Adt15::C2(v3) => {
            match v3 {
                Adt22::C0(h3205, _, _, _, _) => {
                    h3205
                }
            }
        }
        Adt15::C3(v4) => {
            match v4 {
                Adt21::C0(h3206, _) => {
                    h3206
                }
            }
        }
        Adt15::C4(v5) => {
            match v5 {
                Adt23::C0(h3207, _, _, _) => {
                    h3207
                }
            }
        }
        Adt15::C5(v6) => {
            match v6 {
                Adt21::C0(h3208, _) => {
                    h3208
                }
            }
        }
        Adt15::C6(v7) => {
            match v7 {
                Adt21::C0(h3209, _) => {
                    h3209
                }
            }
        }
        Adt15::C7(v8) => {
            match v8 {
                Adt24::C0(h3210, _, _, _, _) => {
                    h3210
                }
            }
        }
        Adt15::C8(v9) => {
            match v9 {
                Adt25::C0(h3211, _, _, _) => {
                    h3211
                }
            }
        }
        Adt15::C9(v10) => {
            match v10 {
                Adt30::C0(h3212, _) => {
                    h3212
                }
            }
        }
        Adt15::C10(v11) => {
            match v11 {
                Adt31::C0(h3213, _, _) => {
                    h3213
                }
            }
        }
        Adt15::C11(v12) => {
            match v12 {
                Adt32::C0(h3214, _) => {
                    h3214
                }
            }
        }
    }
}

pub fn f876(v0: Adt0, v1: Adt17, v2: Adt83) -> Result<Adt134, Adt137> {
    f892(v0, v2, f153(match v1 {
        Adt17::C0(_, h3215, _, _, _, _) => {
            h3215
        }
    }))
}

pub fn f877(v0: Adt0, v1: Adt17, v2: Adt88) -> Result<Adt134, Adt137> {
    f893(v0, v2, f153(match v1 {
        Adt17::C0(_, h3216, _, _, _, _) => {
            h3216
        }
    }))
}

pub fn f878(v0: Adt0, v1: Adt17, v2: Adt50) -> Adt134 {
    Adt134::C0(Adt33::C2(match v2.clone() {
        Adt50::C0(h3217, _) => {
            h3217
        }
    }), {
        let m3219 = f803(v0, Adt33::C11);
        match m3219 {
            Adt134::C0(_, h3218, _) => {
                h3218
            }
        }
    }, f894(v1, match v2 {
        Adt50::C0(_, h3220) => {
            h3220
        }
    }))
}

pub fn f879(v0: Str, p3226: List<(Str, Adt6)>, p3227: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt138> {
    let m3225 = (p3226, p3227);
    {
        let mut m3225 = m3225;
        loop {
            let step = {
                let (v1, v2) = m3225;
                let m3221 = v1;
                match m3221.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt138>>(Ok::<List<(Str, Str)>, Adt138>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3223 = f141({
                            let (_, h3222) = v3;
                            h3222
                        });
                        match m3223 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt138>>(Err::<List<(Str, Str)>, Adt138>(Adt138::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt138>>((v4, List::cons(({
                                    let (h3224, _) = v3;
                                    h3224
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f880(p3230: List<(Str, Str)>, p3231: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3229 = (p3230, p3231);
    {
        let mut m3229 = m3229;
        loop {
            let step = {
                let (v0, v1) = m3229;
                let m3228 = v0;
                match m3228.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f881(v0: Adt119, v1: Adt5) -> R<Option<Adt117>> {
    let m3233 = f895(match v0.clone() {
        Adt119::C0(_, h3232, _, _) => {
            h3232
        }
    }, v1);
    match m3233 {
        Adt107::C0 => {
            Ok(f896(v0))
        }
        Adt107::C1(v2) => {
            if f897(v2.clone()) {
                Ok(f896(v0))
            } else {
                f898(v0, v2)
            }
        }
    }
}

pub fn f882(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m3234 = f214(v0, v1);
    match m3234 {
        Adt107::C0 => {
            Some(None::<Adt5>)
        }
        Adt107::C1(v2) => {
            let m3235 = f219(v2);
            match m3235 {
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

pub fn f883() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f884(v0: Str) -> Adt160 {
    let a3236 = v0.clone();
    let a3237 = Str::lit("debug");
    if equal(a3236, a3237) {
        Adt160::C1(Adt19::C0)
    } else {
        let a3238 = v0.clone();
        let a3239 = Str::lit("info");
        if equal(a3238, a3239) {
            Adt160::C1(Adt19::C1)
        } else {
            let a3240 = v0.clone();
            let a3241 = Str::lit("notice");
            if equal(a3240, a3241) {
                Adt160::C1(Adt19::C2)
            } else {
                let a3242 = v0.clone();
                let a3243 = Str::lit("warning");
                if equal(a3242, a3243) {
                    Adt160::C1(Adt19::C3)
                } else {
                    let a3244 = v0.clone();
                    let a3245 = Str::lit("error");
                    if equal(a3244, a3245) {
                        Adt160::C1(Adt19::C4)
                    } else {
                        let a3246 = v0.clone();
                        let a3247 = Str::lit("critical");
                        if equal(a3246, a3247) {
                            Adt160::C1(Adt19::C5)
                        } else {
                            let a3248 = v0.clone();
                            let a3249 = Str::lit("alert");
                            if equal(a3248, a3249) {
                                Adt160::C1(Adt19::C6)
                            } else {
                                let a3250 = v0;
                                let a3251 = Str::lit("emergency");
                                if equal(a3250, a3251) {
                                    Adt160::C1(Adt19::C7)
                                } else {
                                    Adt160::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f885() -> Str {
    Str::lit("progressToken")
}

pub fn f886(p3256: u64, p3257: i64) -> R<i64> {
    let m3255 = (p3256, p3257);
    {
        let mut m3255 = m3255;
        loop {
            let step = {
                let (v0, v1) = m3255;
                let m3252 = v0;
                if m3252 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3252 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3253 = v1;
                        let a3254 = 10i64;
                        int_mul(a3253, a3254)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f887(p3268: u64, p3269: i64, p3270: u64) -> R<u64> {
    let m3267 = (p3268, (p3269, p3270));
    {
        let mut m3267 = m3267;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3267;
                let m3258 = v0;
                if m3258 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3258 - 1;
                    let v4: u64 = {
                        let a3259 = v2;
                        let a3260 = 1u64;
                        nat_add(a3259, a3260)?
                    };
                    let m3263 = {
                        let a3261 = v1;
                        let a3262 = 10i64;
                        compare(a3261, a3262)
                    };
                    if match m3263 {
                        Ordering::Less => {
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
                            let a3264 = v1;
                            let a3265 = 10i64;
                            let a3266 = 0i64;
                            int_quot(a3264, a3265, a3266)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3267 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f888(v0: Adt3) -> Str {
    let m3271 = v0;
    match m3271 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f889(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a3280 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h3272, _, _, _) => {
                h3272
            }
        })), List::<(Str, Adt6)>::nil());
        let a3281 = {
            let a3278 = f624(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h3273, _, _) => {
                    h3273
                }
            });
            let a3279 = {
                let a3276 = f624(Str::lit("description"), match v0.clone() {
                    Adt58::C0(_, _, h3274, _) => {
                        h3274
                    }
                });
                let a3277 = f301(Str::lit("required"), match v0 {
                    Adt58::C0(_, _, _, h3275) => {
                        h3275
                    }
                });
                append_list(a3276, a3277)
            };
            append_list(a3278, a3279)
        };
        append_list(a3280, a3281)
    })
}

pub fn f890(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt134, Adt137> {
    if v2 {
        Ok::<Adt134, Adt137>(f803(v0, Adt33::C12(v1)))
    } else {
        Err::<Adt134, Adt137>(Adt137::C4(Adt90::C8(f899(f644(v1)))))
    }
}

pub fn f891(v0: Adt18, v1: List<(Str, Adt65)>) -> bool {
    let a3282 = f900(v1.clone());
    if if if bool_not(a3282) {
        true
    } else {
        f672(match v0.clone() {
            Adt18::C0(_, _, _, h3283, _) => {
                h3283
            }
        })
    } {
        let a3284 = f901(v1.clone());
        if bool_not(a3284) {
            true
        } else {
            f672(match v0.clone() {
                Adt18::C0(_, _, h3285, _, _) => {
                    h3285
                }
            })
        }
    } else {
        false
    } {
        let a3286 = f902(v1);
        if bool_not(a3286) {
            true
        } else {
            f672(match v0 {
                Adt18::C0(_, h3287, _, _, _) => {
                    h3287
                }
            })
        }
    } else {
        false
    }
}

pub fn f892(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt134, Adt137> {
    if v2 {
        Ok::<Adt134, Adt137>(f803(v0, Adt33::C13(v1)))
    } else {
        Err::<Adt134, Adt137>(Adt137::C4(Adt90::C8(f227())))
    }
}

pub fn f893(v0: Adt0, v1: Adt88, v2: bool) -> Result<Adt134, Adt137> {
    if v2 {
        Ok::<Adt134, Adt137>(f803(v0, Adt33::C14(v1)))
    } else {
        Err::<Adt134, Adt137>(Adt137::C4(Adt90::C8(f227())))
    }
}

pub fn f894(v0: Adt17, v1: List<Adt51>) -> List<Adt136> {
    let m3289 = match v0 {
        Adt17::C0(_, _, _, _, h3288, _) => {
            h3288
        }
    };
    match m3289 {
        None => {
            List::<Adt136>::nil()
        }
        Some(v2) => {
            f903(f904(v2, v1, List::<Adt136>::nil()), List::<Adt136>::nil())
        }
    }
}

pub fn f895(p3295: List<Str>, p3296: Adt5) -> Adt107 {
    let m3294 = (p3295, p3296);
    {
        let mut m3294 = m3294;
        loop {
            let step = {
                let (v0, v1) = m3294;
                let m3290 = v0;
                match m3290.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt107>(Adt107::C0)
                    }
                    Some((v2, v3)) => {
                        let m3291 = f214(v2, v1);
                        match m3291 {
                            Adt107::C0 => {
                                Err::<(List<Str>, Adt5), Adt107>(Adt107::C0)
                            }
                            Adt107::C1(v4) => {
                                let m3292 = v3.clone();
                                match m3292.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt107>(Adt107::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3293 = f219(v4);
                                        match m3293 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt107>(Adt107::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt107>((v3, v7))
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
                Ok(next) => { m3294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f896(v0: Adt119) -> Option<Adt117> {
    let m3298 = match v0.clone() {
        Adt119::C0(_, _, _, h3297) => {
            h3297
        }
    };
    match m3298 {
        Adt120::C0 => {
            None::<Adt117>
        }
        Adt120::C1 => {
            Some(Adt117::C10(match v0 {
                Adt119::C0(h3299, _, _, _) => {
                    h3299
                }
            }))
        }
        Adt120::C2(_) => {
            None::<Adt117>
        }
    }
}

pub fn f897(v0: Adt6) -> bool {
    let m3300 = v0;
    match m3300 {
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

pub fn f898(v0: Adt119, v1: Adt6) -> R<Option<Adt117>> {
    let m3302 = match v0.clone() {
        Adt119::C0(_, _, _, h3301) => {
            h3301
        }
    };
    match m3302 {
        Adt120::C0 => {
            Ok(Some(Adt117::C8(match v0 {
                Adt119::C0(h3303, _, _, _) => {
                    h3303
                }
            })))
        }
        Adt120::C1 => {
            Ok(Some(Adt117::C10(match v0 {
                Adt119::C0(h3304, _, _, _) => {
                    h3304
                }
            })))
        }
        Adt120::C2(v2) => {
            if f905(match v0.clone() {
                Adt119::C0(_, _, h3305, _) => {
                    h3305
                }
            }, v2, v1)? {
                Ok(None::<Adt117>)
            } else {
                Ok(Some(Adt117::C9(match v0 {
                    Adt119::C0(h3306, _, _, _) => {
                        h3306
                    }
                })))
            }
        }
    }
}

pub fn f899(v0: List<(Str, Adt65)>) -> Adt18 {
    Adt18::C0(None::<Adt5>, if f902(v0.clone()) {
        Some(f27())
    } else {
        None::<Adt5>
    }, if f901(v0.clone()) {
        Some(f27())
    } else {
        None::<Adt5>
    }, if f900(v0) {
        Some(f27())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f900(p3311: List<(Str, Adt65)>) -> bool {
    let m3310 = p3311;
    {
        let mut m3310 = m3310;
        loop {
            let step = {
                let v0 = m3310;
                let m3307 = v0;
                match m3307.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3309 = {
                            let (_, h3308) = v1;
                            h3308
                        };
                        match m3309 {
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
                Ok(next) => { m3310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f901(p3316: List<(Str, Adt65)>) -> bool {
    let m3315 = p3316;
    {
        let mut m3315 = m3315;
        loop {
            let step = {
                let v0 = m3315;
                let m3312 = v0;
                match m3312.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3314 = {
                            let (_, h3313) = v1;
                            h3313
                        };
                        match m3314 {
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
                Ok(next) => { m3315 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f902(p3321: List<(Str, Adt65)>) -> bool {
    let m3320 = p3321;
    {
        let mut m3320 = m3320;
        loop {
            let step = {
                let v0 = m3320;
                let m3317 = v0;
                match m3317.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3319 = {
                            let (_, h3318) = v1;
                            h3318
                        };
                        match m3319 {
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
                Ok(next) => { m3320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f903(p3324: List<Adt136>, p3325: List<Adt136>) -> List<Adt136> {
    let m3323 = (p3324, p3325);
    {
        let mut m3323 = m3323;
        loop {
            let step = {
                let (v0, v1) = m3323;
                let m3322 = v0;
                match m3322.uncons() {
                    None => {
                        Err::<(List<Adt136>, List<Adt136>), List<Adt136>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt136>, List<Adt136>), List<Adt136>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f904(v0: Adt20, p3328: List<Adt51>, p3329: List<Adt136>) -> List<Adt136> {
    let m3327 = (p3328, p3329);
    {
        let mut m3327 = m3327;
        loop {
            let step = {
                let (v1, v2) = m3327;
                let m3326 = v1;
                match m3326.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt136>), List<Adt136>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt51>, List<Adt136>), List<Adt136>>((v4, List::cons(Adt136::C0(v0.clone(), v3), v2)))
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

pub fn f905(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3330 = v0;
    match m3330 {
        Adt13::C0 => {
            let m3331 = f141(v2);
            match m3331 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3332 = v1;
                    let a3333 = v3;
                    Ok(equal(a3332, a3333))
                }
            }
        }
        Adt13::C1 => {
            f906(v1, v2)
        }
        Adt13::C2 => {
            let m3334 = f443(v2);
            match m3334 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3335 = v1;
                        let a3336 = Str::lit("true");
                        Ok(equal(a3335, a3336))
                    } else {
                        let a3337 = v1;
                        let a3338 = Str::lit("false");
                        Ok(equal(a3337, a3338))
                    }
                }
            }
        }
    }
}

pub fn f906(v0: Str, v1: Adt6) -> R<bool> {
    let m3339 = f696(v1);
    match m3339 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3340 = f422(v2)?;
            match m3340 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3342 = f907(f177({
                        let a3341 = v0;
                        utf8_encode(a3341)
                    })?)?;
                    match m3342 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f297({
                                let a3343 = v3;
                                let a3344 = v4;
                                int_sub(a3343, a3344)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f907(v0: List<u64>) -> R<Option<i64>> {
    let m3345 = v0.clone();
    match m3345.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3346 = v1;
            let a3347 = 45u64;
            if nat_eq(a3346, a3347) {
                let m3348 = v2.clone();
                match m3348.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3349 = f908(v2, 0i64)?;
                        match m3349 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3350 = v5;
                                    int_neg(a3350)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f908(v0, 0i64)
            }
        }
    }
}

pub fn f908(p3356: List<u64>, p3357: i64) -> R<Option<i64>> {
    let m3355 = (p3356, p3357);
    {
        let mut m3355 = m3355;
        loop {
            let step = {
                let (v0, v1) = m3355;
                let m3351 = v0;
                match m3351.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3352 = f909(v2);
                        match m3352 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f302({
                                    let a3353 = v1;
                                    let a3354 = 10i64;
                                    int_mul(a3353, a3354)?
                                }, f509(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f909(v0: u64) -> Option<u64> {
    let a3358 = v0;
    let a3359 = 48u64;
    if if nat_lt(a3358, a3359) {
        true
    } else {
        let a3360 = 57u64;
        let a3361 = v0;
        nat_lt(a3360, a3361)
    } {
        None::<u64>
    } else {
        Some({
            let a3362 = v0;
            let a3363 = 48u64;
            nat_sub(a3362, a3363)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
