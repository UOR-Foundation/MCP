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
    C1(Adt82),
}

#[derive(Clone)]
pub enum Adt100 {
    C0(Adt50),
    C1(Adt101, Adt50, Adt86),
    C2(Adt82),
}

#[derive(Clone)]
pub enum Adt101 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt102 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt103 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt104 {
    C0,
    C1,
    C2(Adt5),
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt85, bool),
}

#[derive(Clone)]
pub enum Adt106 {
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
pub enum Adt107 {
    C0,
    C1(Option<Adt102>, Adt108),
}

#[derive(Clone)]
pub enum Adt108 {
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
pub enum Adt109 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt111 {
    C0,
    C1(List<Adt112>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt112 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1(List<Adt112>, Str),
}

#[derive(Clone)]
pub enum Adt114 {
    C0(Adt6, Adt6, List<Adt112>),
    C1(List<Adt6>, Adt6, List<Adt112>),
    C2(List<Adt6>, Adt6, List<Adt112>, u64),
    C3(List<Adt112>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt112>),
    C5(Adt115, Adt6, Adt6, List<Adt112>),
}

#[derive(Clone)]
pub enum Adt115 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt117 {
    C0(Adt102, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt118 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt119 {
    C0(Adt113, List<Adt114>),
}

#[derive(Clone)]
pub enum Adt120 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt121 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt122>, List<Adt124>),
}

#[derive(Clone)]
pub enum Adt122 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt123),
    C7(Adt123),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt123 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt124 {
    C0(Str, List<Str>, Adt13, Adt125),
}

#[derive(Clone)]
pub enum Adt125 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt126 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt127 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt128 {
    C0(Adt129, List<Adt133>, u64),
}

#[derive(Clone)]
pub enum Adt129 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt130, List<u64>, bool),
    C7(u64, u64),
    C8(Adt131),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt130 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(bool, Adt132, i64, u64, i64, bool, i64),
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
    C7,
}

#[derive(Clone)]
pub enum Adt133 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt114>),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt102, Result<Adt139, Adt142>),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt33, Adt140, List<Adt141>),
}

#[derive(Clone)]
pub enum Adt140 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Adt20, Adt51),
}

#[derive(Clone)]
pub enum Adt142 {
    C0(Adt143),
    C1(Str, List<Str>),
    C2(Adt122),
    C3(Str),
    C4(Adt90),
}

#[derive(Clone)]
pub enum Adt143 {
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
pub enum Adt144 {
    C0(Option<Str>, Option<Adt122>),
}

#[derive(Clone)]
pub enum Adt145 {
    C0,
    C1(Adt102),
    C2,
}

#[derive(Clone)]
pub enum Adt146 {
    C0(Adt106),
    C1(Adt128),
}

#[derive(Clone)]
pub enum Adt147 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt148 {
    C0(Adt106),
    C1(List<Adt120>, List<u64>),
}

#[derive(Clone)]
pub enum Adt149 {
    C0(Adt102, Adt150, Adt17, Adt5),
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
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt153 {
    C0,
    C1(Adt150),
}

#[derive(Clone)]
pub enum Adt154 {
    C0(Adt155),
    C1(Adt142),
}

#[derive(Clone)]
pub enum Adt155 {
    C0(Adt102, Str, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt156 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt157 {
    C0(Adt98),
    C1(Adt134),
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
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt160 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt161 {
    C0(u64, List<u64>),
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
    F36,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt22) -> R<Result<Adt99, Adt90>> {
        match self {
            Fn1::F36 => f36(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F38,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt22) -> R<Option<Adt100>> {
        match self {
            Fn2::F38 => f38(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F39,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt22) -> R<Result<Adt50, Adt90>> {
        match self {
            Fn3::F39 => f39(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F77(Fn1, Adt36),
    F282(Fn3, Adt36),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt22) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn4::F77(k0, k1) => f77(k0.clone(), k1.clone(), p0),
            Fn4::F282(k0, k1) => f282(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F138(Fn0, Adt92),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt15) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn5::F138(k0, k1) => f138(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F216,
}

impl Fn6 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt47>>> {
        match self {
            Fn6::F216 => f216(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F225,
}

impl Fn7 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt59 {
        match self {
            Fn7::F225 => f225(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F227,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt25) -> R<Result<List<Str>, Adt90>> {
        match self {
            Fn8::F227 => f227(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F1034(Bytes),
    F1059(Bytes, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn9::F1034(k0) => f1034(k0.clone(), p0),
            Fn9::F1059(k0, k1) => f1059(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F1039(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn10::F1039(k0) => f1039(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F1044(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn11::F1044(k0) => f1044(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F1067(Bytes, u64, u64, u64),
    F1074(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt165)) -> R<(u64, Adt165)> {
        match self {
            Fn12::F1067(k0, k1, k2, k3) => f1067(k0.clone(), *k1, *k2, *k3, p0),
            Fn12::F1074(k0, k1, k2) => f1074(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F1069,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt167) -> R<Adt167> {
        match self {
            Fn13::F1069 => f1069(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F1099,
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn14::F1099 => f1099(p0),
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
            f43(v1)
        }
        Adt15::C1(_) => {
            f43(v1)
        }
        Adt15::C2(v2) => {
            if f34(match v2.clone() {
                Adt22::C0(_, h39, _, _, _) => {
                    h39
                }
            }) {
                f35(f12()?, Fn1::F36, v2)
            } else {
                f37(v0, f12()?, Fn2::F38, Fn3::F39, v2)
            }
        }
        Adt15::C3(_) => {
            f43(v1)
        }
        Adt15::C4(_) => {
            f43(v1)
        }
        Adt15::C5(_) => {
            f43(v1)
        }
        Adt15::C6(_) => {
            f43(v1)
        }
        Adt15::C7(_) => {
            f43(v1)
        }
        Adt15::C8(_) => {
            f43(v1)
        }
        Adt15::C9(v3) => {
            f40(v0, v3)
        }
        Adt15::C10(v4) => {
            f41(v0, v4)
        }
        Adt15::C11(v5) => {
            f42(v0, v5)
        }
    }
}

pub fn f16(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let m40 = f44(v0.clone(), v1)?;
    match m40 {
        None => {
            Ok(None::<Adt96>)
        }
        Some(v2) => {
            let m46 = {
                let a43 = v0;
                let a44 = {
                    let a41 = v1;
                    let a42 = 4u64;
                    nat_add(a41, a42)?
                };
                let a45 = v2;
                slice_bytes(a43, a44, a45)
            };
            match m46 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v3) => {
                    Ok(Some(Adt96::C0(v3, {
                        let a49 = {
                            let a47 = v1;
                            let a48 = 4u64;
                            nat_add(a47, a48)?
                        };
                        let a50 = v2;
                        nat_add(a49, a50)?
                    })))
                }
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: Adt96, v2: Option<Adt96>) -> R<Result<Adt92, Adt94>> {
    let m51 = v2;
    match m51 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(match v1 {
                Adt96::C0(_, h52) => {
                    h52
                }
            })))
        }
        Some(v3) => {
            f45(v0.clone(), v1, v3.clone(), f44(v0, match v3 {
                Adt96::C0(_, h53) => {
                    h53
                }
            })?)
        }
    }
}

pub fn f18(v0: u64) -> List<u64> {
    List::cons({
        let a57 = {
            let a54 = v0;
            let a55 = 16777216u64;
            let a56 = 0u64;
            nat_quot(a54, a55, a56)
        };
        let a58 = 256u64;
        let a59 = 0u64;
        nat_rem(a57, a58, a59)
    }, List::cons({
        let a63 = {
            let a60 = v0;
            let a61 = 65536u64;
            let a62 = 0u64;
            nat_quot(a60, a61, a62)
        };
        let a64 = 256u64;
        let a65 = 0u64;
        nat_rem(a63, a64, a65)
    }, List::cons({
        let a69 = {
            let a66 = v0;
            let a67 = 256u64;
            let a68 = 0u64;
            nat_quot(a66, a67, a68)
        };
        let a70 = 256u64;
        let a71 = 0u64;
        nat_rem(a69, a70, a71)
    }, List::cons({
        let a72 = v0;
        let a73 = 256u64;
        let a74 = 0u64;
        nat_rem(a72, a73, a74)
    }, List::<u64>::nil()))))
}

pub fn f19(v0: List<Adt93>) -> List<u64> {
    f46(f47(v0, List::<Adt93>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f48(f49(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21(v0: Option<List<u64>>) -> List<u64> {
    let m75 = v0;
    match m75 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a79 = f18(1u64);
            let a80 = {
                let a77 = f18({
                    let a76 = v1.clone();
                    length_list(a76)
                });
                let a78 = v1;
                append_list(a77, a78)
            };
            append_list(a79, a80)
        }
    }
}

pub fn f22() -> u64 {
    500u64
}

pub fn f23() -> List<Adt93> {
    List::cons(Adt93::C0(f50(), f51()), List::<Adt93>::nil())
}

pub fn f24() -> List<u64> {
    let a83 = f52();
    let a84 = {
        let a81 = f53();
        let a82 = f54();
        append_list(a81, a82)
    };
    append_list(a83, a84)
}

pub fn f25(v0: Adt0, v1: Fn0, v2: Adt92, v3: Option<Adt97>) -> R<Adt95> {
    let m85 = v3;
    match m85 {
        None => {
            f56(v0, v1, v2)
        }
        Some(v4) => {
            f55(v4)
        }
    }
}

pub fn f26(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m86 = f57(v0.clone(), v1.clone())?;
    match m86 {
        None => {
            f58(v0, v1)
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
    let m87 = f59(v0.clone());
    match m87 {
        None => {
            f60(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f30() -> R<List<Adt36>> {
    Ok(List::cons(f61(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f62()), List::cons(f61(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f63()), List::cons(f61(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f62()), List::cons(f61(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f64()), List::cons(f61(Str::lit("test_image_content"), Str::lit("Tests image content response"), f64()), List::cons(f61(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f64()), List::cons(f61(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f64()), List::cons(f61(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f64()), List::cons(f61(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f64()), List::cons(f61(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f64()), List::cons(f61(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f65()), List::cons(f61(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f66()?), List::cons(f61(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f67()), List::cons(f68(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f64(), f69()), List::cons(f68(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f64(), f69()), List::cons(f61(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f64()), List::cons(f61(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f64()), List::cons(f61(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f64()), List::cons(f61(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f64()), List::cons(f61(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f64()), List::<Adt36>::nil())))))))))))))))))))))
}

pub fn f31() -> List<Adt36> {
    List::cons(f70(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f70(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f70(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f70(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f70(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f70(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f70(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f70(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt36>::nil()))))))))
}

pub fn f32() -> List<Adt36> {
    List::cons(f71(Str::lit("greet"), Str::lit("Answers a greeting at once; never a task"), f72(Str::lit("name"))), List::cons(f71(Str::lit("slow_compute"), Str::lit("Computes for a number of seconds; a task for a client that declared the tasks extension"), f73()), List::cons(f71(Str::lit("failing_job"), Str::lit("Always fails; always a task, so the client must declare the tasks extension"), f74()), List::cons(f71(Str::lit("protocol_error_job"), Str::lit("Ends in a protocol error; a task for a client that declared the tasks extension"), f74()), List::cons(f71(Str::lit("confirm_delete"), Str::lit("Asks the client to confirm a deletion; a task that waits for input"), f72(Str::lit("filename"))), List::cons(f71(Str::lit("multi_input"), Str::lit("Asks the client two questions at once; a task that waits for both answers"), f74()), List::cons(f71(Str::lit("test_tool_with_task"), Str::lit("Asks for a name, then becomes a task; the client must declare the tasks extension"), f74()), List::<Adt36>::nil())))))))
}

pub fn f33(v0: Adt98) -> Str {
    let m88 = v0;
    match m88 {
        Adt98::C0(v1) => {
            let a89 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a90 = Str::lit(" ");
            join(a89, a90)
        }
        Adt98::C1(v2) => {
            let a91 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a92 = Str::lit(" ");
            join(a91, a92)
        }
        Adt98::C2(v3) => {
            let a93 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a94 = Str::lit(" ");
            join(a93, a94)
        }
        Adt98::C3(v4) => {
            let a95 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a96 = Str::lit(" ");
            join(a95, a96)
        }
        Adt98::C4(v5, v6) => {
            let a97 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a98 = Str::lit(" ");
            join(a97, a98)
        }
        Adt98::C5(v7, v8) => {
            let a99 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a100 = Str::lit(" ");
            join(a99, a100)
        }
        Adt98::C6(v9, v10) => {
            let a101 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a102 = Str::lit(" ");
            join(a101, a102)
        }
        Adt98::C7(v11) => {
            let a103 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a104 = Str::lit(" ");
            join(a103, a104)
        }
        Adt98::C8(v12) => {
            let a105 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a106 = Str::lit(" ");
            join(a105, a106)
        }
        Adt98::C9(v13) => {
            let a107 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a108 = Str::lit(" ");
            join(a107, a108)
        }
        Adt98::C10(v14) => {
            let a109 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a110 = Str::lit(" ");
            join(a109, a110)
        }
        Adt98::C11(v15) => {
            let a111 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a112 = Str::lit(" ");
            join(a111, a112)
        }
        Adt98::C12(v16, v17) => {
            let a113 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a114 = Str::lit(" ");
            join(a113, a114)
        }
    }
}

pub fn f34(v0: Str) -> bool {
    let a115 = v0.clone();
    let a116 = Str::lit("test_input_required_result_elicitation");
    if equal(a115, a116) {
        true
    } else {
        let a117 = v0.clone();
        let a118 = Str::lit("test_input_required_result_sampling");
        if equal(a117, a118) {
            true
        } else {
            let a119 = v0.clone();
            let a120 = Str::lit("test_input_required_result_list_roots");
            if equal(a119, a120) {
                true
            } else {
                let a121 = v0.clone();
                let a122 = Str::lit("test_input_required_result_request_state");
                if equal(a121, a122) {
                    true
                } else {
                    let a123 = v0.clone();
                    let a124 = Str::lit("test_input_required_result_multiple_inputs");
                    if equal(a123, a124) {
                        true
                    } else {
                        let a125 = v0.clone();
                        let a126 = Str::lit("test_input_required_result_multi_round");
                        if equal(a125, a126) {
                            true
                        } else {
                            let a127 = v0.clone();
                            let a128 = Str::lit("test_input_required_result_tampered_state");
                            if equal(a127, a128) {
                                true
                            } else {
                                let a129 = v0;
                                let a130 = Str::lit("test_input_required_result_capabilities");
                                equal(a129, a130)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: List<Adt36>, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m132 = f75(match v2.clone() {
        Adt22::C0(_, h131, _, _, _) => {
            h131
        }
    }, v0);
    match m132 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h133, _, _, _) => {
                    h133
                }
            })))
        }
        Some(v3) => {
            f76(v3.clone(), v2, Fn4::F77(v1, v3))
        }
    }
}

pub fn f36(v0: Adt22) -> R<Result<Adt99, Adt90>> {
    if f78(v0.clone(), Str::lit("test_input_required_result_elicitation")) {
        Ok(f79(v0))
    } else {
        if f78(v0.clone(), Str::lit("test_input_required_result_sampling")) {
            Ok(f80(v0))
        } else {
            if f78(v0.clone(), Str::lit("test_input_required_result_list_roots")) {
                f81(v0)
            } else {
                if f78(v0.clone(), Str::lit("test_input_required_result_request_state")) {
                    Ok(f82(v0))
                } else {
                    if f78(v0.clone(), Str::lit("test_input_required_result_multiple_inputs")) {
                        f83(v0)
                    } else {
                        if f78(v0.clone(), Str::lit("test_input_required_result_multi_round")) {
                            f84(v0)
                        } else {
                            if f78(v0.clone(), Str::lit("test_input_required_result_tampered_state")) {
                                Ok(f85(v0))
                            } else {
                                if f78(v0.clone(), Str::lit("test_input_required_result_capabilities")) {
                                    Ok(f86(v0))
                                } else {
                                    Ok(Err::<Adt99, Adt90>(Adt90::C9(Str::lit("not an input-required tool"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f37(v0: Adt14, v1: List<Adt36>, v2: Fn2, v3: Fn3, v4: Adt22) -> R<Result<Adt33, Adt90>> {
    let m135 = f75(match v4.clone() {
        Adt22::C0(_, h134, _, _, _) => {
            h134
        }
    }, v1.clone());
    match m135 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v4 {
                Adt22::C0(_, h136, _, _, _) => {
                    h136
                }
            })))
        }
        Some(v5) => {
            f87(v0, v5, v4.clone(), {
                let c137: Fn2 = v2;
                c137.apply(v4.clone())?
            }, f88(v1, v3, v4)?)
        }
    }
}

pub fn f38(v0: Adt22) -> R<Option<Adt100>> {
    if f89(v0.clone(), Str::lit("greet")) {
        Ok(Some(Adt100::C0(f90({
            let a138 = List::cons(Str::lit("Hello, "), List::cons(f91(v0, Str::lit("name"), Str::lit("world")), List::cons(Str::lit("!"), List::<Str>::nil())));
            let a139 = Str::lit("");
            join(a138, a139)
        }, false))))
    } else {
        if f89(v0.clone(), Str::lit("slow_compute")) {
            Ok(Some(f92(v0)?))
        } else {
            if f89(v0.clone(), Str::lit("confirm_delete")) {
                Ok(Some(f93(v0)?))
            } else {
                if f89(v0.clone(), Str::lit("multi_input")) {
                    Ok(Some(f94()?))
                } else {
                    if f89(v0.clone(), Str::lit("failing_job")) {
                        Ok(Some(f95()?))
                    } else {
                        if f89(v0.clone(), Str::lit("protocol_error_job")) {
                            Ok(Some(f96()))
                        } else {
                            if f89(v0.clone(), Str::lit("test_tool_with_task")) {
                                Ok(Some(f97(v0)?))
                            } else {
                                Ok(None::<Adt100>)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f39(v0: Adt22) -> R<Result<Adt50, Adt90>> {
    if f98(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt50, Adt90>(f99(List::cons(f100(Str::lit("This is a simple text response for testing.")), List::<Adt39>::nil()))))
    } else {
        if f98(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt50, Adt90>(f99(List::cons(f101(), List::<Adt39>::nil()))))
        } else {
            if f98(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt50, Adt90>(f99(List::cons(f102(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt39>::nil()))))
            } else {
                if f98(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt50, Adt90>(f99(List::cons(f103(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt39>::nil()))))
                } else {
                    if f98(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt50, Adt90>(f99(List::cons(f100(Str::lit("Multiple content types test:")), List::cons(f101(), List::cons(f103(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt39>::nil()))))))
                    } else {
                        if f98(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt50, Adt90>(f104(List::cons(f100(Str::lit("This tool intentionally returns an error for testing")), List::<Adt39>::nil()), None::<Adt6>, true, List::<Adt51>::nil())))
                        } else {
                            if f98(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt50, Adt90>(f105(v0, f106()?)))
                            } else {
                                if f98(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt50, Adt90>(f105(v0, f107()?)))
                                } else {
                                    if f98(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt50, Adt90>(f104(List::cons(f100(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt39>::nil()), Some(f108()?), false, List::<Adt51>::nil())))
                                    } else {
                                        if f98(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt50, Adt90>(f104(List::cons(f100(Str::lit("{\"sum\":\"three\"}")), List::<Adt39>::nil()), Some(f109()), false, List::<Adt51>::nil())))
                                        } else {
                                            if f98(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt50, Adt90>(f99(List::cons(f110(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt39>::nil()))))
                                            } else {
                                                if f98(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt50, Adt90>(f99(List::cons(f100(f111()), List::<Adt39>::nil()))))
                                                } else {
                                                    if f98(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt50, Adt90>(f99(List::cons(f100(Str::lit("Streaming complete")), List::<Adt39>::nil()))))
                                                    } else {
                                                        if f98(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt50, Adt90>(f99(List::cons(f100(Str::lit("Logging evaluated")), List::<Adt39>::nil()))))
                                                        } else {
                                                            Ok(f112(v0))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f40(v0: Adt14, v1: Adt30) -> R<Result<Adt33, Adt90>> {
    let m142 = f113(v0.clone(), match v1.clone() {
        Adt30::C0(h140, _) => {
            h140
        }
    }, match v1 {
        Adt30::C0(_, h141) => {
            h141
        }
    })?;
    match m142 {
        Ok(v3) => {
            f114(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f41(v0: Adt14, v1: Adt31) -> R<Result<Adt33, Adt90>> {
    let m145 = f113(v0.clone(), match v1.clone() {
        Adt31::C0(h143, _, _) => {
            h143
        }
    }, match v1.clone() {
        Adt31::C0(_, h144, _) => {
            h144
        }
    })?;
    match m145 {
        Ok(v3) => {
            f115(v0, v3, match v1 {
                Adt31::C0(_, _, h146) => {
                    h146
                }
            })
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f42(v0: Adt14, v1: Adt32) -> R<Result<Adt33, Adt90>> {
    let m149 = f113(v0.clone(), match v1.clone() {
        Adt32::C0(h147, _) => {
            h147
        }
    }, match v1 {
        Adt32::C0(_, h148) => {
            h148
        }
    })?;
    match m149 {
        Ok(v3) => {
            f116(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f43(v0: Adt15) -> R<Result<Adt33, Adt90>> {
    let m150 = v0;
    match m150 {
        Adt15::C0(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt15::C1(v1) => {
            Ok(f117(f10(), f12()?, v1))
        }
        Adt15::C2(v2) => {
            if f34(match v2.clone() {
                Adt22::C0(_, h151, _, _, _) => {
                    h151
                }
            }) {
                f35(f12()?, Fn1::F36, v2)
            } else {
                f88(f12()?, Fn3::F39, v2)
            }
        }
        Adt15::C3(v4) => {
            Ok(f118(v4))
        }
        Adt15::C4(v5) => {
            f119(v5)
        }
        Adt15::C5(v6) => {
            Ok(f120(v6))
        }
        Adt15::C6(v7) => {
            Ok(f121(v7))
        }
        Adt15::C7(v8) => {
            if f122(match v8.clone() {
                Adt24::C0(_, h152, _, _, _) => {
                    h152
                }
            }) {
                Ok(f123(v8))
            } else {
                Ok(f124(v8))
            }
        }
        Adt15::C8(v9) => {
            f125(v9)
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

pub fn f44(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a156 = {
        let a153 = v1;
        let a154 = 4u64;
        nat_add(a153, a154)?
    };
    let a157 = {
        let a155 = v0.clone();
        length_bytes(a155)
    };
    if nat_le(a156, a157) {
        Ok(Some({
            let a174 = {
                let a158 = f126(v0.clone(), v1)?;
                let a159 = 16777216u64;
                nat_mul(a158, a159)?
            };
            let a175 = {
                let a172 = {
                    let a162 = f126(v0.clone(), {
                        let a160 = v1;
                        let a161 = 1u64;
                        nat_add(a160, a161)?
                    })?;
                    let a163 = 65536u64;
                    nat_mul(a162, a163)?
                };
                let a173 = {
                    let a170 = {
                        let a166 = f126(v0.clone(), {
                            let a164 = v1;
                            let a165 = 2u64;
                            nat_add(a164, a165)?
                        })?;
                        let a167 = 256u64;
                        nat_mul(a166, a167)?
                    };
                    let a171 = f126(v0, {
                        let a168 = v1;
                        let a169 = 3u64;
                        nat_add(a168, a169)?
                    })?;
                    nat_add(a170, a171)?
                };
                nat_add(a172, a173)?
            };
            nat_add(a174, a175)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f45(v0: Bytes, v1: Adt96, v2: Adt96, v3: Option<u64>) -> R<Result<Adt92, Adt94>> {
    let m176 = v3;
    match m176 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(match v2 {
                Adt96::C0(_, h177) => {
                    h177
                }
            })))
        }
        Some(v4) => {
            let m181 = f127(v0.clone(), {
                let a179 = match v2.clone() {
                    Adt96::C0(_, h178) => {
                        h178
                    }
                };
                let a180 = 4u64;
                nat_add(a179, a180)?
            }, v4, List::<Adt93>::nil())?;
            match m181 {
                Ok(v6) => {
                    f128(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt92, Adt94>(v5))
                }
            }
        }
    }
}

pub fn f46(p186: List<Adt93>, p187: List<u64>) -> List<u64> {
    let m185 = (p186, p187);
    {
        let mut m185 = m185;
        loop {
            let step = {
                let (v0, v1) = m185;
                let m182 = v0;
                match m182.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<u64>), List<u64>>((v3, {
                            let a183 = f129(v2);
                            let a184 = v1;
                            append_list(a183, a184)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(p190: List<Adt93>, p191: List<Adt93>) -> List<Adt93> {
    let m189 = (p190, p191);
    {
        let mut m189 = m189;
        loop {
            let step = {
                let (v0, v1) = m189;
                let m188 = v0;
                match m188.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m189 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(p196: List<List<u64>>, p197: List<u64>) -> List<u64> {
    let m195 = (p196, p197);
    {
        let mut m195 = m195;
        loop {
            let step = {
                let (v0, v1) = m195;
                let m192 = v0;
                match m192.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a193 = f130(v2);
                            let a194 = v1;
                            append_list(a193, a194)
                        }))
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

pub fn f49(p200: List<List<u64>>, p201: List<List<u64>>) -> List<List<u64>> {
    let m199 = (p200, p201);
    {
        let mut m199 = m199;
        loop {
            let step = {
                let (v0, v1) = m199;
                let m198 = v0;
                match m198.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f51() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f52() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f53() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f54() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f55(v0: Adt97) -> R<Adt95> {
    f131(f132(v0.clone()), f133(None::<Adt102>, f134(v0)))
}

pub fn f56(v0: Adt0, v1: Fn0, v2: Adt92) -> R<Adt95> {
    if f135(match v2.clone() {
        Adt92::C0(h202, _, _, _, _, _, _, _) => {
            h202
        }
    }) {
        f136(match v2.clone() {
            Adt92::C0(_, _, _, _, _, _, _, h203) => {
                h203
            }
        }, f137(v0, match v2.clone() {
            Adt92::C0(_, _, _, _, _, _, h204, _) => {
                h204
            }
        }), Fn5::F138(v1, v2.clone()), match v2.clone() {
            Adt92::C0(_, _, _, _, h205, _, _, _) => {
                h205
            }
        }, match v2.clone() {
            Adt92::C0(_, _, h206, _, _, _, _, _) => {
                h206
            }
        }, f139(match v2 {
            Adt92::C0(_, _, _, h207, _, _, _, _) => {
                h207
            }
        })?)
    } else {
        Ok(f140())
    }
}

pub fn f57(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m208 = f141(v0, f142())?;
    match m208 {
        Adt109::C0 => {
            Ok(None::<Adt97>)
        }
        Adt109::C1 => {
            Ok(Some(Adt97::C0))
        }
        Adt109::C2 => {
            Ok(Some(Adt97::C0))
        }
        Adt109::C3(v2) => {
            f143(f144(v2), v1, Adt97::C0)
        }
    }
}

pub fn f58(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m209 = f141(v0, f145())?;
    match m209 {
        Adt109::C0 => {
            Ok(None::<Adt97>)
        }
        Adt109::C1 => {
            Ok(Some(Adt97::C1))
        }
        Adt109::C2 => {
            Ok(Some(Adt97::C1))
        }
        Adt109::C3(v2) => {
            f143(f146(v2), v1, Adt97::C1)
        }
    }
}

pub fn f59(v0: List<Adt36>) -> Option<Adt98> {
    let m210 = f147(v0, List::<Str>::nil());
    match m210 {
        None => {
            None::<Adt98>
        }
        Some(v1) => {
            Some(Adt98::C1(v1))
        }
    }
}

pub fn f60(p214: List<Adt36>, p215: List<Adt12>) -> R<Result<List<Adt12>, Adt98>> {
    let m213 = (p214, p215);
    {
        let mut m213 = m213;
        loop {
            let step = {
                let (v0, v1) = m213;
                let m211 = v0;
                match m211.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>(Ok(Ok::<List<Adt12>, Adt98>(f148(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m212 = f149(v2)?;
                        match m212 {
                            Ok(v5) => {
                                Ok::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>((v3, f148(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>(Ok(Err::<List<Adt12>, Adt98>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Str, v1: Str, v2: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f62() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f63() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f150(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f151()), List::cons((Str::lit("limit"), f150(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f150(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f64() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f65() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f66() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f151()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f152(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f152(0u64)?), List::cons((Str::lit("maximum"), f152(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f152(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
}

pub fn f67() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("target"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("zone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt6::C3(Str::lit("Zone"))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f68(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt37>, None::<Adt5>)
}

pub fn f69() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("sum"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("sum")), List::cons(Adt6::C3(Str::lit("label")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f70(v0: Str, v1: Str) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))), None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f71(v0: Str, v1: Str, v2: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f72(v0: Str) -> Adt5 {
    f153(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v0.clone(), f154(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v0), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f73() -> Adt5 {
    f153(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("seconds"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), Adt6::C2(Adt7::C0(0i64, 0i64))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("label"), f154(Str::lit("string"))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("seconds")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f74() -> Adt5 {
    f153(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f75(v0: Str, p221: List<Adt36>) -> Option<Adt36> {
    let m220 = p221;
    {
        let mut m220 = m220;
        loop {
            let step = {
                let v1 = m220;
                let m216 = v1;
                match m216.uncons() {
                    None => {
                        Err::<List<Adt36>, Option<Adt36>>(None::<Adt36>)
                    }
                    Some((v2, v3)) => {
                        let a218 = match v2.clone() {
                            Adt36::C0(h217, _, _, _, _, _, _, _) => {
                                h217
                            }
                        };
                        let a219 = v0.clone();
                        if equal(a218, a219) {
                            Err::<List<Adt36>, Option<Adt36>>(Some(v2))
                        } else {
                            Ok::<List<Adt36>, Option<Adt36>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: Adt36, v1: Adt22, v2: Fn4) -> R<Result<Adt33, Adt90>> {
    let m228 = f155(Adt6::C5({
        let m224 = match v0.clone() {
            Adt36::C0(_, _, _, _, h223, _, _, _) => {
                h223
            }
        };
        match m224 {
            Adt5::C0(h222) => {
                h222
            }
        }
    }), Adt6::C5({
        let m227 = match v0 {
            Adt36::C0(_, _, _, _, h226, _, _, _) => {
                h226
            }
        };
        match m227 {
            Adt5::C0(h225) => {
                h225
            }
        }
    }), f156(v1.clone()))?;
    match m228 {
        Adt111::C0 => {
            let c229: Fn4 = v2;
            c229.apply(v1)
        }
        Adt111::C1(v3, v4) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(f157(f158(v3, v4)?, true))))
        }
        Adt111::C2(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt111::C3(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt111::C4 => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f77(v0: Fn1, v1: Adt36, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    f159(v1, {
        let c230: Fn1 = v0;
        c230.apply(v2)?
    })
}

pub fn f78(v0: Adt22, v1: Str) -> bool {
    let a232 = match v0 {
        Adt22::C0(_, h231, _, _, _) => {
            h231
        }
    };
    let a233 = v1;
    equal(a232, a233)
}

pub fn f79(v0: Adt22) -> Result<Adt99, Adt90> {
    let m235 = f160(Str::lit("user_name"), Adt103::C0, match v0 {
        Adt22::C0(_, _, _, h234, _) => {
            h234
        }
    });
    match m235 {
        Adt104::C0 => {
            f164(f165(Str::lit("user_name"), f166()))
        }
        Adt104::C1 => {
            f167()
        }
        Adt104::C2(v1) => {
            f161(f162(List::cons(Str::lit("Hello, "), List::cons(f163(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f80(v0: Adt22) -> Result<Adt99, Adt90> {
    let m237 = f160(Str::lit("capital_question"), Adt103::C1, match v0 {
        Adt22::C0(_, _, _, h236, _) => {
            h236
        }
    });
    match m237 {
        Adt104::C0 => {
            f164(f165(Str::lit("capital_question"), f168(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt104::C1 => {
            f167()
        }
        Adt104::C2(v1) => {
            f161(f162(List::cons(Str::lit("Sampling result: "), List::cons(f163(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f81(v0: Adt22) -> R<Result<Adt99, Adt90>> {
    let m239 = f160(Str::lit("client_roots"), Adt103::C2, match v0 {
        Adt22::C0(_, _, _, h238, _) => {
            h238
        }
    });
    match m239 {
        Adt104::C0 => {
            Ok(f164(f165(Str::lit("client_roots"), f171())))
        }
        Adt104::C1 => {
            Ok(f167())
        }
        Adt104::C2(v1) => {
            Ok(f161(f162(List::cons(Str::lit("Found "), List::cons(f169(f170(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f82(v0: Adt22) -> Result<Adt99, Adt90> {
    f172(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f83(v0: Adt22) -> R<Result<Adt99, Adt90>> {
    if f173(v0.clone(), Str::lit("multiple-inputs")) {
        f174(f160(Str::lit("user_name"), Adt103::C0, match v0.clone() {
            Adt22::C0(_, _, _, h240, _) => {
                h240
            }
        }), f160(Str::lit("greeting"), Adt103::C1, match v0.clone() {
            Adt22::C0(_, _, _, h241, _) => {
                h241
            }
        }), f160(Str::lit("client_roots"), Adt103::C2, match v0 {
            Adt22::C0(_, _, _, h242, _) => {
                h242
            }
        }))
    } else {
        Ok(f175(f176(Adt104::C0, Adt104::C0, Adt104::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f84(v0: Adt22) -> R<Result<Adt99, Adt90>> {
    let v1: u64 = f177(f178(v0.clone()))?;
    let a243 = v1;
    let a244 = 49u64;
    if nat_eq(a243, a244) {
        Ok(f179(v0))
    } else {
        let a245 = v1;
        let a246 = 50u64;
        if nat_eq(a245, a246) {
            Ok(f180(v0))
        } else {
            Ok(f175(f165(Str::lit("step1"), f181()), Str::lit("1")))
        }
    }
}

pub fn f85(v0: Adt22) -> Result<Adt99, Adt90> {
    f172(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f86(v0: Adt22) -> Result<Adt99, Adt90> {
    let m248 = match v0.clone() {
        Adt22::C0(_, _, _, h247, _) => {
            h247
        }
    };
    match m248 {
        None => {
            f182(f183({
                let m251 = match v0 {
                    Adt22::C0(h250, _, _, _, _) => {
                        h250
                    }
                };
                match m251 {
                    Adt17::C0(_, h249, _, _, _, _) => {
                        h249
                    }
                }
            }))
        }
        Some(v1) => {
            let m253 = match v1.clone() {
                Adt5::C0(h252) => {
                    h252
                }
            };
            match m253.uncons() {
                None => {
                    f182(f183({
                        let m256 = match v0 {
                            Adt22::C0(h255, _, _, _, _) => {
                                h255
                            }
                        };
                        match m256 {
                            Adt17::C0(_, h254, _, _, _, _) => {
                                h254
                            }
                        }
                    }))
                }
                Some((_, _)) => {
                    f161(f162(List::cons(Str::lit("capabilities-ok: received "), List::cons({
                        let a258 = f184(match v1 {
                            Adt5::C0(h257) => {
                                h257
                            }
                        });
                        let a259 = Str::lit(",");
                        join(a258, a259)
                    }, List::<Str>::nil()))))
                }
            }
        }
    }
}

pub fn f87(v0: Adt14, v1: Adt36, v2: Adt22, v3: Option<Adt100>, v4: Result<Adt33, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m260 = v3;
    match m260 {
        None => {
            Ok(v4)
        }
        Some(v5) => {
            f185(v0, v1.clone(), v2.clone(), v5, f186(v1, v2)?)
        }
    }
}

pub fn f88(v0: List<Adt36>, v1: Fn3, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m262 = f75(match v2.clone() {
        Adt22::C0(_, h261, _, _, _) => {
            h261
        }
    }, v0);
    match m262 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h263, _, _, _) => {
                    h263
                }
            })))
        }
        Some(v3) => {
            f187(v3, v1, v2)
        }
    }
}

pub fn f89(v0: Adt22, v1: Str) -> bool {
    let a265 = match v0 {
        Adt22::C0(_, h264, _, _, _) => {
            h264
        }
    };
    let a266 = v1;
    equal(a265, a266)
}

pub fn f90(v0: Str, v1: bool) -> Adt50 {
    Adt50::C0(Adt38::C0(List::cons(f188(v0), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt51>::nil())
}

pub fn f91(v0: Adt22, v1: Str, v2: Str) -> Str {
    let m267 = f189(v0, v1);
    match m267 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            let m268 = f190(v3);
            match m268 {
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

pub fn f92(v0: Adt22) -> R<Adt100> {
    let v1: i64 = f191(v0.clone());
    let v2: Str = {
        let a270 = List::cons(Str::lit("slow_compute"), List::cons(f91(v0, Str::lit("label"), Str::lit("unnamed")), List::cons(Str::lit("finished after"), List::cons({
            let a269 = v1;
            format_i64(a269)
        }, List::cons(Str::lit("seconds"), List::<Str>::nil())))));
        let a271 = Str::lit(" ");
        join(a270, a271)
    };
    Ok(Adt100::C1(Adt101::C0, f90(v2.clone(), false), f192(0i64, List::<(Str, Adt65)>::nil(), {
        let a272 = v1;
        let a273 = 1000i64;
        int_mul(a272, a273)?
    }, f193(v2, false)?)))
}

pub fn f93(v0: Adt22) -> R<Adt100> {
    let v1: Str = {
        let a274 = List::cons(Str::lit("Deleted"), List::cons(f91(v0.clone(), Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a275 = Str::lit(" ");
        join(a274, a275)
    };
    Ok(Adt100::C1(Adt101::C0, f90(v1.clone(), false), f192(0i64, List::cons((Str::lit("confirm"), f194({
        let a276 = List::cons(Str::lit("Delete"), List::cons(f91(v0, Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a277 = Str::lit(" ");
        join(a276, a277)
    }, Str::lit("confirm"), Str::lit("boolean"))), List::<(Str, Adt65)>::nil()), 0i64, f193(v1, false)?)))
}

pub fn f94() -> R<Adt100> {
    Ok(Adt100::C1(Adt101::C0, f90(Str::lit("Both inputs received"), false), f192(0i64, List::cons((Str::lit("first"), f194(Str::lit("First name"), Str::lit("name"), Str::lit("string"))), List::cons((Str::lit("second"), f194(Str::lit("Second name"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt65)>::nil())), 0i64, f193(Str::lit("Both inputs received"), false)?)))
}

pub fn f95() -> R<Adt100> {
    Ok(Adt100::C1(Adt101::C1, f90(Str::lit("failing_job always fails"), true), f192(0i64, List::<(Str, Adt65)>::nil(), 1000i64, f193(Str::lit("failing_job always fails"), true)?)))
}

pub fn f96() -> Adt100 {
    Adt100::C1(Adt101::C0, f90(Str::lit("protocol_error_job has no result"), true), f192(0i64, List::<(Str, Adt65)>::nil(), 500i64, Adt87::C1(Adt81::C0(-32603i64, Str::lit("protocol_error_job failed"), None::<Adt6>))))
}

pub fn f97(v0: Adt22) -> R<Adt100> {
    let m279 = match v0 {
        Adt22::C0(_, _, _, h278, _) => {
            h278
        }
    };
    match m279 {
        None => {
            Ok(f195())
        }
        Some(v1) => {
            let m280 = f196(v1);
            match m280 {
                None => {
                    Ok(f195())
                }
                Some(v2) => {
                    f197(v2)
                }
            }
        }
    }
}

pub fn f98(v0: Adt22, v1: Str) -> bool {
    let a282 = match v0 {
        Adt22::C0(_, h281, _, _, _) => {
            h281
        }
    };
    let a283 = v1;
    equal(a282, a283)
}

pub fn f99(v0: List<Adt39>) -> Adt50 {
    f104(v0, None::<Adt6>, false, List::<Adt51>::nil())
}

pub fn f100(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f101() -> Adt39 {
    f110(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f102(v0: Str, v1: Str) -> Adt39 {
    Adt39::C2(Adt44::C0(v0, v1, None::<Adt41>, None::<Adt5>))
}

pub fn f103(v0: Str, v1: Str, v2: Str) -> Adt39 {
    Adt39::C4(Adt46::C0(Adt47::C0(Adt48::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt41>, None::<Adt5>))
}

pub fn f104(v0: List<Adt39>, v1: Option<Adt6>, v2: bool, v3: List<Adt51>) -> Adt50 {
    Adt50::C0(Adt38::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f105(v0: Adt22, v1: List<Adt51>) -> Adt50 {
    f104(List::cons(f100(f198({
        let m286 = match v0 {
            Adt22::C0(h285, _, _, _, _) => {
                h285
            }
        };
        match m286 {
            Adt17::C0(_, _, _, _, h284, _) => {
                h284
            }
        }
    })), List::<Adt39>::nil()), None::<Adt6>, false, v1)
}

pub fn f106() -> R<List<Adt51>> {
    Ok(List::cons(f199(0u64, 100u64)?, List::cons(f199(50u64, 100u64)?, List::cons(f199(100u64, 100u64)?, List::<Adt51>::nil()))))
}

pub fn f107() -> R<List<Adt51>> {
    Ok(List::cons(f199(50u64, 100u64)?, List::cons(f199(10u64, 100u64)?, List::<Adt51>::nil())))
}

pub fn f108() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f152(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f109() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f110(v0: Str, v1: Str) -> Adt39 {
    Adt39::C1(Adt43::C0(v0, v1, None::<Adt41>, None::<Adt5>))
}

pub fn f111() -> Str {
    let m288 = {
        let a287 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a287)
    };
    match m288 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f112(v0: Adt22) -> Result<Adt50, Adt90> {
    if f98(v0.clone(), Str::lit("test_missing_capability")) {
        let m294 = {
            let m293 = {
                let m292 = match v0 {
                    Adt22::C0(h291, _, _, _, _) => {
                        h291
                    }
                };
                match m292 {
                    Adt17::C0(_, h290, _, _, _, _) => {
                        h290
                    }
                }
            };
            match m293 {
                Adt18::C0(_, _, h289, _, _) => {
                    h289
                }
            }
        };
        match m294 {
            None => {
                Err::<Adt50, Adt90>(Adt90::C8(f200()))
            }
            Some(_) => {
                Ok::<Adt50, Adt90>(f201())
            }
        }
    } else {
        if f98(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt50, Adt90>(f99(List::cons(f100(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt39>::nil())))
        } else {
            if f98(v0, Str::lit("test_arguments")) {
                Ok::<Adt50, Adt90>(f99(List::cons(f100(Str::lit("arguments accepted")), List::<Adt39>::nil())))
            } else {
                Ok::<Adt50, Adt90>(f201())
            }
        }
    }
}

pub fn f113(v0: Adt14, v1: Adt17, v2: Str) -> R<Result<Adt85, Adt90>> {
    if f202(match v1 {
        Adt17::C0(_, h295, _, _, _, _) => {
            h295
        }
    }) {
        let m297 = f203(match v0.clone() {
            Adt14::C0(_, _, _, h296) => {
                h296
            }
        })?;
        match m297 {
            None => {
                Ok(Err::<Adt85, Adt90>(f204()))
            }
            Some(v3) => {
                let m299 = f205(v2.clone(), v3, match v0 {
                    Adt14::C0(h298, _, _, _) => {
                        h298
                    }
                })?;
                match m299 {
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
        Ok(Err::<Adt85, Adt90>(f206()))
    }
}

pub fn f114(v0: Adt14, v1: Adt85) -> R<Result<Adt33, Adt90>> {
    Ok(Ok::<Adt33, Adt90>(f207(f208(v1, match v0.clone() {
        Adt14::C0(h300, _, _, _) => {
            h300
        }
    }, match v0 {
        Adt14::C0(_, h301, _, _) => {
            h301
        }
    })?, Adt89::C0, false)))
}

pub fn f115(v0: Adt14, v1: Adt85, v2: Adt5) -> R<Result<Adt33, Adt90>> {
    let v3: Adt105 = f208(v1, match v0.clone() {
        Adt14::C0(h302, _, _, _) => {
            h302
        }
    }, match v0.clone() {
        Adt14::C0(_, h303, _, _) => {
            h303
        }
    })?;
    Ok(Ok::<Adt33, Adt90>(f207(f209(match v3.clone() {
        Adt105::C0(h304, _) => {
            h304
        }
    }, v2, match v0.clone() {
        Adt14::C0(h305, _, _, _) => {
            h305
        }
    }, match v0 {
        Adt14::C0(_, h306, _, _) => {
            h306
        }
    })?, Adt89::C1, match v3 {
        Adt105::C0(_, h307) => {
            h307
        }
    })))
}

pub fn f116(v0: Adt14, v1: Adt85) -> R<Result<Adt33, Adt90>> {
    Ok(Ok::<Adt33, Adt90>(f207(f210(v1, match v0.clone() {
        Adt14::C0(h308, _, _, _) => {
            h308
        }
    }, match v0 {
        Adt14::C0(_, h309, _, _) => {
            h309
        }
    })?, Adt89::C1, false)))
}

pub fn f117(v0: Adt10, v1: List<Adt36>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m311 = match v2 {
        Adt21::C0(_, h310) => {
            h310
        }
    };
    match m311 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C1(Adt35::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f118(v0: Adt21) -> Result<Adt33, Adt90> {
    f211(f212(), f213(), v0)
}

pub fn f119(v0: Adt23) -> R<Result<Adt33, Adt90>> {
    f214(f215(), Fn6::F216, v0)
}

pub fn f120(v0: Adt21) -> Result<Adt33, Adt90> {
    f217(f212(), f218(), v0)
}

pub fn f121(v0: Adt21) -> Result<Adt33, Adt90> {
    f219(f212(), f220(), v0)
}

pub fn f122(v0: Str) -> bool {
    let a312 = v0;
    let a313 = f221();
    equal(a312, a313)
}

pub fn f123(v0: Adt24) -> Result<Adt33, Adt90> {
    let m315 = f160(Str::lit("user_context"), Adt103::C0, match v0 {
        Adt24::C0(_, _, _, h314, _) => {
            h314
        }
    });
    match m315 {
        Adt104::C0 => {
            Ok::<Adt33, Adt90>(Adt33::C12(Adt82::C0(f165(Str::lit("user_context"), f223(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt104::C1 => {
            Err::<Adt33, Adt90>(Adt90::C11)
        }
        Adt104::C2(v1) => {
            Ok::<Adt33, Adt90>(Adt33::C8(f222(f163(v1, Str::lit("context"), Str::lit("unknown")))))
        }
    }
}

pub fn f124(v0: Adt24) -> Result<Adt33, Adt90> {
    f224(f220(), Fn7::F225, v0)
}

pub fn f125(v0: Adt25) -> R<Result<Adt33, Adt90>> {
    f226(Fn8::F227, v0)
}

pub fn f126(v0: Bytes, v1: u64) -> R<u64> {
    let m318 = {
        let a316 = v0;
        let a317 = v1;
        index_bytes(a316, a317)
    };
    match m318 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a363 = {
                let a339 = {
                    let a327 = {
                        let a321 = {
                            let a319 = v3;
                            let a320 = 128u8;
                            fixed_u8::bit_and(a319, a320)
                        };
                        let a322 = 128u8;
                        if equal(a321, a322) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a328 = {
                        let a325 = {
                            let a323 = v3;
                            let a324 = 64u8;
                            fixed_u8::bit_and(a323, a324)
                        };
                        let a326 = 64u8;
                        if equal(a325, a326) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a327, a328)?
                };
                let a340 = {
                    let a337 = {
                        let a331 = {
                            let a329 = v3;
                            let a330 = 32u8;
                            fixed_u8::bit_and(a329, a330)
                        };
                        let a332 = 32u8;
                        if equal(a331, a332) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a338 = {
                        let a335 = {
                            let a333 = v3;
                            let a334 = 16u8;
                            fixed_u8::bit_and(a333, a334)
                        };
                        let a336 = 16u8;
                        if equal(a335, a336) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a337, a338)?
                };
                nat_add(a339, a340)?
            };
            let a364 = {
                let a361 = {
                    let a349 = {
                        let a343 = {
                            let a341 = v3;
                            let a342 = 8u8;
                            fixed_u8::bit_and(a341, a342)
                        };
                        let a344 = 8u8;
                        if equal(a343, a344) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a350 = {
                        let a347 = {
                            let a345 = v3;
                            let a346 = 4u8;
                            fixed_u8::bit_and(a345, a346)
                        };
                        let a348 = 4u8;
                        if equal(a347, a348) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a349, a350)?
                };
                let a362 = {
                    let a359 = {
                        let a353 = {
                            let a351 = v3;
                            let a352 = 2u8;
                            fixed_u8::bit_and(a351, a352)
                        };
                        let a354 = 2u8;
                        if equal(a353, a354) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a360 = {
                        let a357 = {
                            let a355 = v3;
                            let a356 = 1u8;
                            fixed_u8::bit_and(a355, a356)
                        };
                        let a358 = 1u8;
                        if equal(a357, a358) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a359, a360)?
                };
                nat_add(a361, a362)?
            };
            nat_add(a363, a364)
        }
    }
}

pub fn f127(v0: Bytes, p374: u64, p375: u64, p376: List<Adt93>) -> R<Result<(List<Adt93>, u64), Adt94>> {
    let m373 = (p374, (p375, p376));
    {
        let mut m373 = m373;
        loop {
            let step = {
                let (v1, (v2, v3)) = m373;
                let m365 = v2;
                if m365 == 0 {
                    Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Ok::<(List<Adt93>, u64), Adt94>((f228(v3, List::<Adt93>::nil()), v1))))
                } else {
                    let v4 = m365 - 1;
                    let m366 = f16(v0.clone(), v1)?;
                    match m366 {
                        None => {
                            Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Err::<(List<Adt93>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            let m368 = f16(v0.clone(), match v5.clone() {
                                Adt96::C0(_, h367) => {
                                    h367
                                }
                            })?;
                            match m368 {
                                None => {
                                    Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Err::<(List<Adt93>, u64), Adt94>(Adt94::C0(match v5 {
                                        Adt96::C0(_, h369) => {
                                            h369
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>((match v6.clone() {
                                        Adt96::C0(_, h370) => {
                                            h370
                                        }
                                    }, (v4, List::cons(Adt93::C0(f229(match v5 {
                                        Adt96::C0(h371, _) => {
                                            h371
                                        }
                                    })?, f229(match v6 {
                                        Adt96::C0(h372, _) => {
                                            h372
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: Bytes, v1: Adt96, v2: Adt96, v3: (List<Adt93>, u64)) -> R<Result<Adt92, Adt94>> {
    let m378 = f16(v0.clone(), {
        let (_, h377) = v3.clone();
        h377
    })?;
    match m378 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0({
                let (_, h379) = v3;
                h379
            })))
        }
        Some(v4) => {
            let m381 = f44(v0.clone(), match v4.clone() {
                Adt96::C0(_, h380) => {
                    h380
                }
            })?;
            match m381 {
                None => {
                    Ok(Err::<Adt92, Adt94>(Adt94::C0(match v4 {
                        Adt96::C0(_, h382) => {
                            h382
                        }
                    })))
                }
                Some(v5) => {
                    let m386 = f230(v0.clone(), {
                        let a384 = match v4.clone() {
                            Adt96::C0(_, h383) => {
                                h383
                            }
                        };
                        let a385 = 4u64;
                        nat_add(a384, a385)?
                    })?;
                    match m386 {
                        None => {
                            Ok(Err::<Adt92, Adt94>(Adt94::C0({
                                let a388 = match v4 {
                                    Adt96::C0(_, h387) => {
                                        h387
                                    }
                                };
                                let a389 = 4u64;
                                nat_add(a388, a389)?
                            })))
                        }
                        Some(v6) => {
                            f231(v0, Adt92::C0(f229(match v1 {
                                Adt96::C0(h390, _) => {
                                    h390
                                }
                            })?, f229(match v2 {
                                Adt96::C0(h391, _) => {
                                    h391
                                }
                            })?, {
                                let (h392, _) = v3;
                                h392
                            }, match v4 {
                                Adt96::C0(h393, _) => {
                                    h393
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt96::C0(h394, _) => {
                                    h394
                                }
                            }, Adt14::C0(0i64, Str::lit(""), Str::lit(""), f232())), match v6 {
                                Adt96::C0(_, h395) => {
                                    h395
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f129(v0: Adt93) -> List<u64> {
    let a406 = f18({
        let a397 = match v0.clone() {
            Adt93::C0(h396, _) => {
                h396
            }
        };
        length_list(a397)
    });
    let a407 = {
        let a404 = match v0.clone() {
            Adt93::C0(h398, _) => {
                h398
            }
        };
        let a405 = {
            let a402 = f18({
                let a400 = match v0.clone() {
                    Adt93::C0(_, h399) => {
                        h399
                    }
                };
                length_list(a400)
            });
            let a403 = match v0 {
                Adt93::C0(_, h401) => {
                    h401
                }
            };
            append_list(a402, a403)
        };
        append_list(a404, a405)
    };
    append_list(a406, a407)
}

pub fn f130(v0: List<u64>) -> List<u64> {
    let a409 = f18({
        let a408 = v0.clone();
        length_list(a408)
    });
    let a410 = v0;
    append_list(a409, a410)
}

pub fn f131(v0: u64, v1: Adt6) -> R<Adt95> {
    let m411 = f233(v1)?;
    match m411 {
        Ok(v3) => {
            Ok(Adt95::C0(v0, f23(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f132(v0: Adt97) -> u64 {
    let m412 = v0;
    match m412 {
        Adt97::C0 => {
            f234()
        }
        Adt97::C1 => {
            f234()
        }
    }
}

pub fn f133(v0: Option<Adt102>, v1: Adt81) -> Adt6 {
    Adt6::C5(f235(v0, v1))
}

pub fn f134(v0: Adt97) -> Adt81 {
    Adt81::C0(f236(), f237(v0), None::<Adt6>)
}

pub fn f135(v0: List<u64>) -> bool {
    f238(v0, f239())
}

pub fn f136(v0: Adt14, v1: Adt0, v2: Fn5, v3: u64, v4: List<Adt93>, v5: Result<Adt6, Adt106>) -> R<Adt95> {
    let m413 = v5;
    match m413 {
        Ok(v7) => {
            f242(v0, v1, v2, v3, v4, f243(v7)?)
        }
        Err(_) => {
            f131(f240(), f133(None::<Adt102>, f241(Adt107::C0)))
        }
    }
}

pub fn f137(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h414, _, _, _, _, _, _) => {
            h414
        }
    }, match v0.clone() {
        Adt0::C0(_, h415, _, _, _, _, _) => {
            h415
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h416, _, _, _, _) => {
            h416
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h417, _, _, _) => {
            h417
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h418, _, _) => {
            h418
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, h419, _) => {
            h419
        }
    }, v1)
}

pub fn f138(v0: Fn0, v1: Adt92, v2: Adt15) -> R<Result<Adt33, Adt90>> {
    let c420: Fn0 = v0;
    c420.apply(match v1 {
        Adt92::C0(_, _, _, _, _, _, _, h421) => {
            h421
        }
    }, v2)
}

pub fn f139(v0: Bytes) -> R<Result<Adt6, Adt106>> {
    f244(f229(v0)?)
}

pub fn f140() -> Adt95 {
    Adt95::C0(f245(), List::cons(Adt93::C0(f246(), f247()), List::<Adt93>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f141(v0: List<Adt93>, v1: List<u64>) -> R<Adt109> {
    let m422 = f248(v0, v1, Adt110::C0)?;
    match m422 {
        Adt110::C0 => {
            Ok(Adt109::C0)
        }
        Adt110::C1(v2) => {
            let m423 = f249(v2);
            match m423 {
                None => {
                    Ok(Adt109::C1)
                }
                Some(v3) => {
                    Ok(Adt109::C3(v3))
                }
            }
        }
        Adt110::C2 => {
            Ok(Adt109::C2)
        }
    }
}

pub fn f142() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f143(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt97) -> R<Option<Adt97>> {
    let m424 = v0;
    match m424 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f250(v3, v1)? {
                Ok(None::<Adt97>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f144(v0: List<u64>) -> Option<List<u64>> {
    let m425 = f251(v0);
    match m425 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f146(v1)
        }
    }
}

pub fn f145() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f146(v0: List<u64>) -> Option<List<u64>> {
    let m426 = v0.clone();
    match m426.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a427 = v1;
            let a428 = 91u64;
            if nat_eq(a427, a428) {
                f252(f253(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f252(f254(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f147(p434: List<Adt36>, p435: List<Str>) -> Option<Str> {
    let m433 = (p434, p435);
    {
        let mut m433 = m433;
        loop {
            let step = {
                let (v0, v1) = m433;
                let m429 = v0;
                match m429.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f255(match v2.clone() {
                            Adt36::C0(h430, _, _, _, _, _, _, _) => {
                                h430
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt36>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt36::C0(h431, _, _, _, _, _, _, _) => {
                                    h431
                                }
                            }))
                        } else {
                            Ok::<(List<Adt36>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt36::C0(h432, _, _, _, _, _, _, _) => {
                                    h432
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(p438: List<Adt12>, p439: List<Adt12>) -> List<Adt12> {
    let m437 = (p438, p439);
    {
        let mut m437 = m437;
        loop {
            let step = {
                let (v0, v1) = m437;
                let m436 = v0;
                match m436.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Adt36) -> R<Result<List<Adt12>, Adt98>> {
    let m442 = f256(match v0.clone() {
        Adt36::C0(h440, _, _, _, _, _, _, _) => {
            h440
        }
    }, f257(match v0.clone() {
        Adt36::C0(h441, _, _, _, _, _, _, _) => {
            h441
        }
    })?);
    match m442 {
        None => {
            f258(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f150(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f151() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f152(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f259(v0, 0i64)?, 0i64)))
}

pub fn f153(v0: List<(Str, Adt6)>) -> Adt5 {
    Adt5::C0(v0)
}

pub fn f154(v0: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::<(Str, Adt6)>::nil()))
}

pub fn f155(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt111> {
    f260(v0, f261(), Adt113::C0, List::cons(Adt114::C0(v1, v2, List::<Adt112>::nil()), List::<Adt114>::nil()))
}

pub fn f156(v0: Adt22) -> Adt6 {
    let m444 = match v0 {
        Adt22::C0(_, _, h443, _, _) => {
            h443
        }
    };
    match m444 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h445) => {
                    h445
                }
            })
        }
    }
}

pub fn f157(v0: Str, v1: bool) -> Adt38 {
    Adt38::C0(List::cons(Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f158(v0: List<Adt112>, v1: Str) -> R<Str> {
    let v2: Str = f262(v0)?;
    let a448 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a446 = v2.clone();
        let a447 = Str::lit("");
        if equal(a446, a447) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a449 = Str::lit(" ");
    Ok(join(a448, a449))
}

pub fn f159(v0: Adt36, v1: Result<Adt99, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m450 = v1;
    match m450 {
        Ok(v3) => {
            let m451 = v3;
            match m451 {
                Adt99::C0(v4) => {
                    f263(v0, v4)
                }
                Adt99::C1(v5) => {
                    Ok(Ok::<Adt33, Adt90>(Adt33::C12(v5)))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f160(v0: Str, v1: Adt103, v2: Option<Adt5>) -> Adt104 {
    let m452 = v2;
    match m452 {
        None => {
            Adt104::C0
        }
        Some(v3) => {
            let m453 = f264(v0, v3);
            match m453 {
                Adt116::C0 => {
                    Adt104::C0
                }
                Adt116::C1(v4) => {
                    let m454 = f265(v4);
                    match m454 {
                        None => {
                            Adt104::C1
                        }
                        Some(v5) => {
                            if f266(v1, v5.clone()) {
                                Adt104::C2(v5)
                            } else {
                                Adt104::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f161(v0: Str) -> Result<Adt99, Adt90> {
    Ok::<Adt99, Adt90>(Adt99::C0(Adt50::C0(Adt38::C0(List::cons(Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)), List::<Adt39>::nil()), None::<Adt6>, None::<bool>), List::<Adt51>::nil())))
}

pub fn f162(v0: List<Str>) -> Str {
    let a455 = v0;
    let a456 = Str::lit("");
    join(a455, a456)
}

pub fn f163(v0: Adt5, v1: Str, v2: Str) -> Str {
    let m457 = f264(Str::lit("content"), v0);
    match m457 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            let m458 = f265(v3);
            match m458 {
                None => {
                    v2
                }
                Some(v4) => {
                    f267(v1, v4, v2)
                }
            }
        }
    }
}

pub fn f164(v0: List<(Str, Adt65)>) -> Result<Adt99, Adt90> {
    Ok::<Adt99, Adt90>(Adt99::C1(Adt82::C0(v0)))
}

pub fn f165(v0: Str, v1: Adt65) -> List<(Str, Adt65)> {
    List::cons((v0, v1), List::<(Str, Adt65)>::nil())
}

pub fn f166() -> Adt65 {
    f223(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f167() -> Result<Adt99, Adt90> {
    Err::<Adt99, Adt90>(Adt90::C11)
}

pub fn f168(v0: Str, v1: u64) -> Adt65 {
    Adt65::C1(Adt70::C0(List::cons(f268(v0), List::<Adt71>::nil()), None::<Adt76>, None::<Str>, None::<Adt78>, None::<Adt7>, v1, None::<List<Str>>, None::<Adt5>, None::<List<Adt36>>, None::<Adt79>))
}

pub fn f169(v0: u64) -> R<Str> {
    let a459 = f269(v0, 0i64)?;
    Ok(format_i64(a459))
}

pub fn f170(v0: Adt5) -> u64 {
    let m460 = f264(Str::lit("roots"), v0);
    match m460 {
        Adt116::C0 => {
            0u64
        }
        Adt116::C1(v1) => {
            let m461 = f270(v1);
            match m461 {
                None => {
                    0u64
                }
                Some(v2) => {
                    let a462 = v2;
                    length_list(a462)
                }
            }
        }
    }
}

pub fn f171() -> Adt65 {
    Adt65::C2(Some(f27()))
}

pub fn f172(v0: Adt22, v1: Str, v2: Str) -> Result<Adt99, Adt90> {
    if f173(v0.clone(), v1.clone()) {
        let m464 = f160(Str::lit("confirm"), Adt103::C0, match v0 {
            Adt22::C0(_, _, _, h463, _) => {
                h463
            }
        });
        match m464 {
            Adt104::C0 => {
                f175(f165(Str::lit("confirm"), f272()), v1)
            }
            Adt104::C1 => {
                f167()
            }
            Adt104::C2(v3) => {
                if f271(v3) {
                    f161(v2)
                } else {
                    f175(f165(Str::lit("confirm"), f272()), v1)
                }
            }
        }
    } else {
        f175(f165(Str::lit("confirm"), f272()), v1)
    }
}

pub fn f173(v0: Adt22, v1: Str) -> bool {
    let m466 = match v0 {
        Adt22::C0(_, _, _, _, h465) => {
            h465
        }
    };
    match m466 {
        None => {
            false
        }
        Some(v2) => {
            let a467 = v2;
            let a468 = v1;
            equal(a467, a468)
        }
    }
}

pub fn f174(v0: Adt104, v1: Adt104, v2: Adt104) -> R<Result<Adt99, Adt90>> {
    if if f273(v0.clone()) {
        true
    } else {
        if f273(v1.clone()) {
            true
        } else {
            f273(v2.clone())
        }
    } {
        Ok(f167())
    } else {
        let m469 = f176(v0.clone(), v1.clone(), v2.clone());
        match m469.uncons() {
            None => {
                Ok(f161(f162(List::cons(Str::lit("Name: "), List::cons(f163(f274(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f163(f274(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f169(f170(f274(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f175(f176(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f175(v0: List<(Str, Adt65)>, v1: Str) -> Result<Adt99, Adt90> {
    Ok::<Adt99, Adt90>(Adt99::C1(Adt82::C2(v0, v1)))
}

pub fn f176(v0: Adt104, v1: Adt104, v2: Adt104) -> List<(Str, Adt65)> {
    let a472 = if f275(v0) {
        f165(Str::lit("user_name"), f166())
    } else {
        List::<(Str, Adt65)>::nil()
    };
    let a473 = {
        let a470 = if f275(v1) {
            f165(Str::lit("greeting"), f168(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt65)>::nil()
        };
        let a471 = if f275(v2) {
            f165(Str::lit("client_roots"), f171())
        } else {
            List::<(Str, Adt65)>::nil()
        };
        append_list(a470, a471)
    };
    append_list(a472, a473)
}

pub fn f177(v0: Str) -> R<u64> {
    f126({
        let a474 = v0;
        utf8_encode(a474)
    }, 0u64)
}

pub fn f178(v0: Adt22) -> Str {
    let m476 = match v0 {
        Adt22::C0(_, _, _, _, h475) => {
            h475
        }
    };
    match m476 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f179(v0: Adt22) -> Result<Adt99, Adt90> {
    let m478 = f160(Str::lit("step1"), Adt103::C0, match v0 {
        Adt22::C0(_, _, _, h477, _) => {
            h477
        }
    });
    match m478 {
        Adt104::C0 => {
            f175(f165(Str::lit("step1"), f181()), Str::lit("1"))
        }
        Adt104::C1 => {
            f167()
        }
        Adt104::C2(v1) => {
            f175(f165(Str::lit("step2"), f276()), f162(List::cons(Str::lit("2"), List::cons(f163(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f180(v0: Adt22) -> Result<Adt99, Adt90> {
    let m480 = f160(Str::lit("step2"), Adt103::C0, match v0.clone() {
        Adt22::C0(_, _, _, h479, _) => {
            h479
        }
    });
    match m480 {
        Adt104::C0 => {
            f175(f165(Str::lit("step2"), f276()), f178(v0))
        }
        Adt104::C1 => {
            f167()
        }
        Adt104::C2(v1) => {
            f161(f162(List::cons(Str::lit("Multi-round complete for "), List::cons(f277(f178(v0)), List::cons(Str::lit(" who likes "), List::cons(f163(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f181() -> Adt65 {
    f223(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f182(v0: List<(Str, Adt65)>) -> Result<Adt99, Adt90> {
    let m481 = v0.clone();
    match m481.uncons() {
        None => {
            f161(Str::lit("No supported capabilities declared"))
        }
        Some((_, _)) => {
            f175(v0, Str::lit("capabilities-test"))
        }
    }
}

pub fn f183(v0: Adt18) -> List<(Str, Adt65)> {
    let a484 = if f278(match v0.clone() {
        Adt18::C0(_, _, _, h482, _) => {
            h482
        }
    }) {
        f165(Str::lit("elicit_input"), f223(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt65)>::nil()
    };
    let a485 = if f278(match v0 {
        Adt18::C0(_, _, h483, _, _) => {
            h483
        }
    }) {
        f165(Str::lit("sample_input"), f168(Str::lit("Sample request"), 50u64))
    } else {
        List::<(Str, Adt65)>::nil()
    };
    append_list(a484, a485)
}

pub fn f184(v0: List<(Str, Adt6)>) -> List<Str> {
    f279(f280(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f185(v0: Adt14, v1: Adt36, v2: Adt22, v3: Adt100, v4: Result<Option<Adt33>, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m486 = v4;
    match m486 {
        Ok(v6) => {
            let m487 = v6;
            match m487 {
                None => {
                    f281(v0, v1, v2, v3)
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

pub fn f186(v0: Adt36, v1: Adt22) -> R<Result<Option<Adt33>, Adt90>> {
    let m494 = f155(Adt6::C5({
        let m490 = match v0.clone() {
            Adt36::C0(_, _, _, _, h489, _, _, _) => {
                h489
            }
        };
        match m490 {
            Adt5::C0(h488) => {
                h488
            }
        }
    }), Adt6::C5({
        let m493 = match v0 {
            Adt36::C0(_, _, _, _, h492, _, _, _) => {
                h492
            }
        };
        match m493 {
            Adt5::C0(h491) => {
                h491
            }
        }
    }), f156(v1))?;
    match m494 {
        Adt111::C0 => {
            Ok(Ok::<Option<Adt33>, Adt90>(None::<Adt33>))
        }
        Adt111::C1(v2, v3) => {
            Ok(Ok::<Option<Adt33>, Adt90>(Some(Adt33::C2(f157(f158(v2, v3)?, true)))))
        }
        Adt111::C2(_) => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt111::C3(_) => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt111::C4 => {
            Ok(Err::<Option<Adt33>, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f187(v0: Adt36, v1: Fn3, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    f76(v0.clone(), v2, Fn4::F282(v1, v0))
}

pub fn f188(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f189(v0: Adt22, v1: Str) -> Adt116 {
    let m496 = match v0 {
        Adt22::C0(_, _, h495, _, _) => {
            h495
        }
    };
    match m496 {
        None => {
            Adt116::C0
        }
        Some(v2) => {
            f264(v1, v2)
        }
    }
}

pub fn f190(v0: Adt6) -> Option<Str> {
    let m497 = v0;
    match m497 {
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

pub fn f191(v0: Adt22) -> i64 {
    let m498 = f189(v0, Str::lit("seconds"));
    match m498 {
        Adt116::C0 => {
            0i64
        }
        Adt116::C1(v1) => {
            let m499 = f283(v1);
            match m499 {
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

pub fn f192(v0: i64, v1: List<(Str, Adt65)>, v2: i64, v3: Adt87) -> Adt86 {
    Adt86::C0(v0, v1, v2, v3)
}

pub fn f193(v0: Str, v1: bool) -> R<Adt87> {
    let m502 = f284(f285({
        let m501 = f90(v0, v1);
        match m501 {
            Adt50::C0(h500, _) => {
                h500
            }
        }
    })?)?;
    match m502 {
        None => {
            Ok(f286())
        }
        Some(v2) => {
            Ok(Adt87::C0(Adt5::C0(v2)))
        }
    }
}

pub fn f194(v0: Str, v1: Str, v2: Str) -> Adt65 {
    Adt65::C0(Adt66::C0(Adt67::C0(Some(Adt68::C0), v0, f153(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v1.clone(), f154(v2)), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v1), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))))))
}

pub fn f195() -> Adt100 {
    Adt100::C2(Adt82::C0(List::cons((Str::lit("user_name"), f194(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt65)>::nil())))
}

pub fn f196(v0: Adt5) -> Option<Str> {
    let m503 = f264(Str::lit("user_name"), v0);
    match m503 {
        Adt116::C0 => {
            None::<Str>
        }
        Adt116::C1(v1) => {
            let m504 = f265(v1);
            match m504 {
                None => {
                    None::<Str>
                }
                Some(v2) => {
                    let m505 = f264(Str::lit("content"), v2);
                    match m505 {
                        Adt116::C0 => {
                            None::<Str>
                        }
                        Adt116::C1(v3) => {
                            let m506 = f265(v3);
                            match m506 {
                                None => {
                                    None::<Str>
                                }
                                Some(v4) => {
                                    let m507 = f264(Str::lit("name"), v4);
                                    match m507 {
                                        Adt116::C0 => {
                                            None::<Str>
                                        }
                                        Adt116::C1(v5) => {
                                            f190(v5)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f197(v0: Str) -> R<Adt100> {
    let v1: Str = {
        let a508 = List::cons(Str::lit("Hello, "), List::cons(v0, List::cons(Str::lit("! (from a task)"), List::<Str>::nil())));
        let a509 = Str::lit("");
        join(a508, a509)
    };
    Ok(Adt100::C1(Adt101::C1, f90(v1.clone(), false), f192(0i64, List::<(Str, Adt65)>::nil(), 100i64, f193(v1, false)?)))
}

pub fn f198(v0: Option<Adt20>) -> Str {
    let m510 = v0;
    match m510 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m511 = v1;
            match m511 {
                Adt20::C0(v2) => {
                    v2
                }
                Adt20::C1(v3) => {
                    let a512 = v3;
                    format_i64(a512)
                }
            }
        }
    }
}

pub fn f199(v0: u64, v1: u64) -> R<Adt51> {
    Ok(Adt51::C0(f287(v0)?, Some(f287(v1)?), Some({
        let a515 = List::cons(Str::lit("Completed step"), List::cons({
            let a513 = f269(v0, 0i64)?;
            format_i64(a513)
        }, List::cons(Str::lit("of"), List::cons({
            let a514 = f269(v1, 0i64)?;
            format_i64(a514)
        }, List::<Str>::nil()))));
        let a516 = Str::lit(" ");
        join(a515, a516)
    })))
}

pub fn f200() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, Some(f27()), None::<Adt5>, None::<Adt5>)
}

pub fn f201() -> Adt50 {
    f99(List::cons(f100(Str::lit("ok")), List::<Adt39>::nil()))
}

pub fn f202(v0: Adt18) -> bool {
    f288(match v0 {
        Adt18::C0(_, _, _, _, h517) => {
            h517
        }
    }, f28())
}

pub fn f203(v0: Bytes) -> R<Option<List<Adt85>>> {
    let a519 = {
        let a518 = v0.clone();
        length_bytes(a518)
    };
    let a520 = 0u64;
    if nat_eq(a519, a520) {
        Ok(Some(List::<Adt85>::nil()))
    } else {
        let m521 = f139(v0)?;
        match m521 {
            Ok(v2) => {
                let m522 = f270(v2);
                match m522 {
                    None => {
                        Ok(None::<List<Adt85>>)
                    }
                    Some(v3) => {
                        Ok(f289(v3, List::<Adt85>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt85>>)
            }
        }
    }
}

pub fn f204() -> Adt90 {
    Adt90::C9(Str::lit("the task table is not readable"))
}

pub fn f205(v0: Str, p527: List<Adt85>, v2: i64) -> R<Option<Adt85>> {
    let m526 = p527;
    {
        let mut m526 = m526;
        loop {
            let step = {
                let v1 = m526;
                let m523 = v1;
                match m523.uncons() {
                    None => {
                        Err::<List<Adt85>, R<Option<Adt85>>>(Ok(None::<Adt85>))
                    }
                    Some((v3, v4)) => {
                        let a524 = f290(v3.clone());
                        let a525 = v0.clone();
                        if equal(a524, a525) {
                            if f291(v3.clone(), v2)? {
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
                Ok(next) => { m526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206() -> Adt90 {
    Adt90::C8(f292())
}

pub fn f207(v0: Adt105, v1: Adt89, v2: bool) -> Adt33 {
    Adt33::C14(Adt88::C0(match v0.clone() {
        Adt105::C0(h528, _) => {
            h528
        }
    }, v1, if match v0 {
        Adt105::C0(_, h529) => {
            h529
        }
    } {
        true
    } else {
        v2
    }))
}

pub fn f208(v0: Adt85, v1: i64, v2: Str) -> R<Adt105> {
    let m531 = match v0.clone() {
        Adt85::C0(h530, _, _) => {
            h530
        }
    };
    match m531 {
        Adt63::C0(v3) => {
            f293(v0, v3, v1, v2)
        }
        Adt63::C1(_, _) => {
            Ok(f294(v0))
        }
        Adt63::C2(_, _) => {
            Ok(f294(v0))
        }
        Adt63::C3(_, _) => {
            Ok(f294(v0))
        }
        Adt63::C4(_) => {
            Ok(f294(v0))
        }
    }
}

pub fn f209(v0: Adt85, v1: Adt5, v2: i64, v3: Str) -> R<Adt105> {
    let m533 = match v0.clone() {
        Adt85::C0(h532, _, _) => {
            h532
        }
    };
    match m533 {
        Adt63::C0(_) => {
            Ok(f294(v0))
        }
        Adt63::C1(v5, v6) => {
            f295(v0, v5, v6, v1, v2, v3)
        }
        Adt63::C2(_, _) => {
            Ok(f294(v0))
        }
        Adt63::C3(_, _) => {
            Ok(f294(v0))
        }
        Adt63::C4(_) => {
            Ok(f294(v0))
        }
    }
}

pub fn f210(v0: Adt85, v1: i64, v2: Str) -> R<Adt105> {
    Ok(f296(f208(v0, v1, v2.clone())?, v2))
}

pub fn f211(v0: Adt10, v1: List<Adt45>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m535 = match v2 {
        Adt21::C0(_, h534) => {
            h534
        }
    };
    match m535 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C4(Adt52::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f212() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C0)
}

pub fn f213() -> List<Adt45> {
    f297(f298())
}

pub fn f214(v0: Adt10, v1: Fn6, v2: Adt23) -> R<Result<Adt33, Adt90>> {
    Ok(f299(v0, match v2.clone() {
        Adt23::C0(_, h536, _, _) => {
            h536
        }
    }, {
        let c537: Fn6 = v1;
        c537.apply(match v2 {
            Adt23::C0(_, h538, _, _) => {
                h538
            }
        })?
    }))
}

pub fn f215() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C1)
}

pub fn f216(v0: Str) -> R<Option<List<Adt47>>> {
    let m539 = f300(v0.clone(), f298());
    match m539 {
        None => {
            f301(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f217(v0: Adt10, v1: List<Adt55>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m541 = match v2 {
        Adt21::C0(_, h540) => {
            h540
        }
    };
    match m541 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C6(Adt54::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f218() -> List<Adt55> {
    List::cons(Adt55::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt2>>, f302(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt41>, None::<Adt5>), List::<Adt55>::nil())
}

pub fn f219(v0: Adt10, v1: List<Adt57>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m543 = match v2 {
        Adt21::C0(_, h542) => {
            h542
        }
    };
    match m543 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C7(Adt56::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f220() -> List<Adt57> {
    List::cons(f303(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt58>>), List::cons(f303(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f304(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f304(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt58>::nil())))), List::cons(f303(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f304(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt58>::nil()))), List::cons(f303(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt58>>), List::<Adt57>::nil()))))
}

pub fn f221() -> Str {
    Str::lit("test_input_required_result_prompt")
}

pub fn f222(v0: Str) -> Adt59 {
    Adt59::C0(None::<Str>, List::cons(Adt60::C0(Adt42::C0, Adt39::C0(Adt40::C0(f162(List::cons(Str::lit("Prompt with context: "), List::cons(v0, List::<Str>::nil()))), None::<Adt41>, None::<Adt5>))), List::<Adt60>::nil()))
}

pub fn f223(v0: Str, v1: Str, v2: Str) -> Adt65 {
    Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v0, f305(v1, v2))))
}

pub fn f224(v0: List<Adt57>, v1: Fn7, v2: Adt24) -> Result<Adt33, Adt90> {
    let m545 = f306(match v2.clone() {
        Adt24::C0(_, h544, _, _, _) => {
            h544
        }
    }, v0);
    match m545 {
        None => {
            Err::<Adt33, Adt90>(Adt90::C1(match v2 {
                Adt24::C0(_, h546, _, _, _) => {
                    h546
                }
            }))
        }
        Some(v3) => {
            let m548 = f307(f308(v3), f309(match v2.clone() {
                Adt24::C0(_, _, h547, _, _) => {
                    h547
                }
            }));
            match m548 {
                None => {
                    Ok::<Adt33, Adt90>(Adt33::C8({
                        let c551: Fn7 = v1;
                        c551.apply(match v2.clone() {
                            Adt24::C0(_, h552, _, _, _) => {
                                h552
                            }
                        }, f309(match v2 {
                            Adt24::C0(_, _, h553, _, _) => {
                                h553
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt33, Adt90>(Adt90::C4({
                        let a549 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a550 = Str::lit(" ");
                        join(a549, a550)
                    }))
                }
            }
        }
    }
}

pub fn f225(v0: Str, v1: List<(Str, Str)>) -> Adt59 {
    let a554 = v0.clone();
    let a555 = Str::lit("test_simple_prompt");
    if equal(a554, a555) {
        f310(List::cons(f311(Str::lit("This is a simple prompt for testing.")), List::<Adt60>::nil()))
    } else {
        let a556 = v0.clone();
        let a557 = Str::lit("test_prompt_with_arguments");
        if equal(a556, a557) {
            f310(List::cons(f311({
                let a558 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f312(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f312(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a559 = Str::lit("");
                join(a558, a559)
            }), List::<Adt60>::nil()))
        } else {
            let a560 = v0.clone();
            let a561 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a560, a561) {
                f310(List::cons(Adt60::C0(Adt42::C0, Adt39::C4(Adt46::C0(f313(f312(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt41>, None::<Adt5>))), List::cons(f311(Str::lit("Please process the embedded resource above.")), List::<Adt60>::nil())))
            } else {
                let a562 = v0;
                let a563 = Str::lit("test_prompt_with_image");
                if equal(a562, a563) {
                    f310(List::cons(Adt60::C0(Adt42::C0, Adt39::C1(Adt43::C0(f314(), Str::lit("image/png"), None::<Adt41>, None::<Adt5>))), List::cons(f311(Str::lit("Please analyze the image above.")), List::<Adt60>::nil())))
                } else {
                    f310(List::<Adt60>::nil())
                }
            }
        }
    }
}

pub fn f226(v0: Fn8, v1: Adt25) -> R<Result<Adt33, Adt90>> {
    let m565 = {
        let c564: Fn8 = v0;
        c564.apply(v1)?
    };
    match m565 {
        Ok(v3) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C9(f315(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f227(v0: Adt25) -> R<Result<List<Str>, Adt90>> {
    let m567 = match v0.clone() {
        Adt25::C0(_, h566, _, _) => {
            h566
        }
    };
    match m567 {
        Adt26::C0(v1) => {
            let m569 = f306(match v1.clone() {
                Adt27::C0(h568, _) => {
                    h568
                }
            }, f220());
            match m569 {
                None => {
                    Ok(Err::<List<Str>, Adt90>(Adt90::C1(match v1 {
                        Adt27::C0(h570, _) => {
                            h570
                        }
                    })))
                }
                Some(v2) => {
                    if f316(f308(v2), {
                        let m573 = match v0.clone() {
                            Adt25::C0(_, _, h572, _) => {
                                h572
                            }
                        };
                        match m573 {
                            Adt28::C0(h571, _) => {
                                h571
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt90>(f317({
                            let m576 = match v0.clone() {
                                Adt25::C0(_, _, h575, _) => {
                                    h575
                                }
                            };
                            match m576 {
                                Adt28::C0(_, h574) => {
                                    h574
                                }
                            }
                        }, f318(match v1 {
                            Adt27::C0(h577, _) => {
                                h577
                            }
                        }, {
                            let m580 = match v0 {
                                Adt25::C0(_, _, h579, _) => {
                                    h579
                                }
                            };
                            match m580 {
                                Adt28::C0(h578, _) => {
                                    h578
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
            let a581 = v3.clone();
            let a582 = f302();
            if equal(a581, a582) {
                let a586 = {
                    let m585 = match v0.clone() {
                        Adt25::C0(_, _, h584, _) => {
                            h584
                        }
                    };
                    match m585 {
                        Adt28::C0(h583, _) => {
                            h583
                        }
                    }
                };
                let a587 = Str::lit("id");
                if equal(a586, a587) {
                    Ok(Ok::<List<Str>, Adt90>(f317({
                        let m590 = match v0 {
                            Adt25::C0(_, _, h589, _) => {
                                h589
                            }
                        };
                        match m590 {
                            Adt28::C0(_, h588) => {
                                h588
                            }
                        }
                    }, f319())?))
                } else {
                    Ok(Err::<List<Str>, Adt90>(Adt90::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt90>(Adt90::C2(v3)))
            }
        }
    }
}

pub fn f228(p593: List<Adt93>, p594: List<Adt93>) -> List<Adt93> {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v0, v1) = m592;
                let m591 = v0;
                match m591.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes) -> R<List<u64>> {
    f320(v0.clone(), {
        let a595 = v0;
        length_bytes(a595)
    }, List::<u64>::nil())
}

pub fn f230(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let a597 = v1;
    let a598 = {
        let a596 = v0.clone();
        length_bytes(a596)
    };
    if nat_eq(a597, a598) {
        Ok(Some(Adt96::C0(Bytes::lit(&[]), v1)))
    } else {
        f16(v0, v1)
    }
}

pub fn f231(v0: Bytes, v1: Adt92, v2: u64) -> R<Result<Adt92, Adt94>> {
    let a600 = v2;
    let a601 = {
        let a599 = v0.clone();
        length_bytes(a599)
    };
    if nat_eq(a600, a601) {
        Ok(Ok::<Adt92, Adt94>(v1))
    } else {
        let m602 = f44(v0.clone(), v2)?;
        match m602 {
            None => {
                Ok(Err::<Adt92, Adt94>(Adt94::C0(v2)))
            }
            Some(v3) => {
                let m605 = f321(v0.clone(), {
                    let a603 = v2;
                    let a604 = 4u64;
                    nat_add(a603, a604)?
                }, v3, List::<List<u64>>::nil())?;
                match m605 {
                    Ok(v5) => {
                        Ok(f322(v0.clone(), f323(v1, {
                            let (h606, _) = v5;
                            h606
                        }), f324(v0, {
                            let (_, h607) = v5;
                            h607
                        })?))
                    }
                    Err(v4) => {
                        Ok(Err::<Adt92, Adt94>(v4))
                    }
                }
            }
        }
    }
}

pub fn f232() -> Bytes {
    Bytes::lit(&[])
}

pub fn f233(v0: Adt6) -> R<Result<List<u64>, Adt106>> {
    f325(f326(), v0)
}

pub fn f234() -> u64 {
    403u64
}

pub fn f235(v0: Option<Adt102>, v1: Adt81) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f327())), {
        let a608 = f328(v0);
        let a609 = List::cons((Str::lit("error"), f329(v1)), List::<(Str, Adt6)>::nil());
        append_list(a608, a609)
    })
}

pub fn f236() -> i64 {
    -32000i64
}

pub fn f237(v0: Adt97) -> Str {
    let m610 = v0;
    match m610 {
        Adt97::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt97::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f238(p617: List<u64>, p618: List<u64>) -> bool {
    let m616 = (p617, p618);
    {
        let mut m616 = m616;
        loop {
            let step = {
                let (v0, v1) = m616;
                let m611 = v0;
                match m611.uncons() {
                    None => {
                        let m612 = v1;
                        match m612.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m613 = v1;
                        match m613.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a614 = v4;
                                let a615 = v6;
                                if nat_eq(a614, a615) {
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
                Ok(next) => { m616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f240() -> u64 {
    400u64
}

pub fn f241(v0: Adt107) -> Adt81 {
    Adt81::C0(f330(v0.clone()), f331(v0), None::<Adt6>)
}

pub fn f242(v0: Adt14, v1: Adt0, v2: Fn5, v3: u64, v4: List<Adt93>, v5: Result<Adt117, Adt107>) -> R<Adt95> {
    let m619 = v5;
    match m619 {
        Ok(v7) => {
            f334(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f131(f332(v6.clone()), f133(f333(v6.clone()), f241(v6)))
        }
    }
}

pub fn f243(v0: Adt6) -> R<Result<Adt117, Adt107>> {
    let m620 = v0;
    match m620 {
        Adt6::C0 => {
            Ok(f336(None::<Adt102>, Adt108::C0))
        }
        Adt6::C1(_) => {
            Ok(f336(None::<Adt102>, Adt108::C0))
        }
        Adt6::C2(_) => {
            Ok(f336(None::<Adt102>, Adt108::C0))
        }
        Adt6::C3(_) => {
            Ok(f336(None::<Adt102>, Adt108::C0))
        }
        Adt6::C4(_) => {
            Ok(f336(None::<Adt102>, Adt108::C1))
        }
        Adt6::C5(v1) => {
            f335(Adt5::C0(v1))
        }
    }
}

pub fn f244(v0: List<u64>) -> R<Result<Adt6, Adt106>> {
    let m621 = f337(v0.clone())?;
    match m621 {
        Adt126::C0 => {
            f338(f339(), 0u64, v0)
        }
        Adt126::C1(v1) => {
            Ok(Err::<Adt6, Adt106>(Adt106::C0(v1)))
        }
        Adt126::C2(v2) => {
            Ok(Err::<Adt6, Adt106>(Adt106::C1(v2)))
        }
    }
}

pub fn f245() -> u64 {
    405u64
}

pub fn f246() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f247() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f248(p626: List<Adt93>, v1: List<u64>, p627: Adt110) -> R<Adt110> {
    let m625 = (p626, p627);
    {
        let mut m625 = m625;
        loop {
            let step = {
                let (v0, v2) = m625;
                let m622 = v0;
                match m622.uncons() {
                    None => {
                        Err::<(List<Adt93>, Adt110), R<Adt110>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f340(match v3.clone() {
                            Adt93::C0(h623, _) => {
                                h623
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt93>, Adt110), R<Adt110>>((v4, f341(v2, match v3 {
                                Adt93::C0(_, h624) => {
                                    h624
                                }
                            })))
                        } else {
                            Ok::<(List<Adt93>, Adt110), R<Adt110>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f342(v0);
    if f343(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f250(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f340(v0.clone(), f344())? {
        Ok(true)
    } else {
        if f340(v0.clone(), f345())? {
            Ok(true)
        } else {
            if f340(v0.clone(), f346())? {
                Ok(true)
            } else {
                f347(v0, v1)
            }
        }
    }
}

pub fn f251(p632: List<u64>) -> Option<List<u64>> {
    let m631 = p632;
    {
        let mut m631 = m631;
        loop {
            let step = {
                let v0 = m631;
                let m628 = v0;
                match m628.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a629 = v1;
                        let a630 = 58u64;
                        if nat_eq(a629, a630) {
                            Err::<List<u64>, Option<List<u64>>>(f348(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m633 = v0;
    match m633 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f349({
                let (_, h634) = v1;
                h634
            }) {
                Some({
                    let (h635, _) = v1;
                    h635
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f253(p640: List<u64>, p641: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m639 = (p640, p641);
    {
        let mut m639 = m639;
        loop {
            let step = {
                let (v0, v1) = m639;
                let m636 = v0;
                match m636.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a637 = v2;
                        let a638 = 93u64;
                        if nat_eq(a637, a638) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f350(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f254(p646: List<u64>, p647: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m645 = (p646, p647);
    {
        let mut m645 = m645;
        loop {
            let step = {
                let (v0, v1) = m645;
                let m642 = v0;
                match m642.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f350(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a643 = v2;
                        let a644 = 58u64;
                        if nat_eq(a643, a644) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f350(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: Str, p652: List<Str>) -> bool {
    let m651 = p652;
    {
        let mut m651 = m651;
        loop {
            let step = {
                let v1 = m651;
                let m648 = v1;
                match m648.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a649 = v0.clone();
                        let a650 = v2;
                        if equal(a649, a650) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m651 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: Str, v1: List<u64>) -> Option<Adt98> {
    if f351(v1) {
        Some(Adt98::C0(v0))
    } else {
        None::<Adt98>
    }
}

pub fn f257(v0: Str) -> R<List<u64>> {
    f229({
        let a653 = v0;
        utf8_encode(a653)
    })
}

pub fn f258(v0: Adt36) -> R<Result<List<Adt12>, Adt98>> {
    let m654 = f352(v0.clone());
    match m654 {
        None => {
            f353(v0.clone(), f354(f355(), match v0.clone() {
                Adt36::C0(h655, _, _, _, _, _, _, _) => {
                    h655
                }
            }, Adt6::C5({
                let m658 = match v0.clone() {
                    Adt36::C0(_, _, _, _, h657, _, _, _) => {
                        h657
                    }
                };
                match m658 {
                    Adt5::C0(h656) => {
                        h656
                    }
                }
            }), List::cons(Adt118::C0(Adt6::C5({
                let m661 = match v0 {
                    Adt36::C0(_, _, _, _, h660, _, _, _) => {
                        h660
                    }
                };
                match m661 {
                    Adt5::C0(h659) => {
                        h659
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt118>::nil()), f356())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f259(p666: u64, p667: i64) -> R<i64> {
    let m665 = (p666, p667);
    {
        let mut m665 = m665;
        loop {
            let step = {
                let (v0, v1) = m665;
                let m662 = v0;
                if m662 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m662 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a663 = v1;
                        let a664 = -1i64;
                        int_sub(a663, a664)?
                    }))
                }
            };
            match step {
                Ok(next) => { m665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: Adt6, p686: u64, p687: Adt113, p688: List<Adt114>) -> R<Adt111> {
    let m685 = (p686, (p687, p688));
    {
        let mut m685 = m685;
        loop {
            let step = {
                let (v1, (v2, v3)) = m685;
                let m668 = v1;
                if m668 == 0 {
                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C4))
                } else {
                    let v4 = m668 - 1;
                    let m669 = v3;
                    match m669.uncons() {
                        None => {
                            let m670 = v2;
                            match m670 {
                                Adt113::C0 => {
                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C0))
                                }
                                Adt113::C1(v5, v6) => {
                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m671 = v2.clone();
                            match m671 {
                                Adt113::C0 => {
                                    let m672 = v7.clone();
                                    match m672 {
                                        Adt114::C0(v9, v10, v11) => {
                                            let m673 = f357(v0.clone(), v9, v10, v11.clone())?;
                                            match m673 {
                                                Adt135::C0(v12) => {
                                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (Adt113::C1(v11, v12), v8)))
                                                }
                                                Adt135::C1(v13) => {
                                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C2(v13)))
                                                }
                                                Adt135::C2(v14) => {
                                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C3(v14)))
                                                }
                                                Adt135::C3(v15) => {
                                                    let v16: List<Adt114> = {
                                                        let a674 = v15;
                                                        let a675 = v8;
                                                        append_list(a674, a675)
                                                    };
                                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt114::C1(_, _, _) => {
                                            Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (v2, v8)))
                                        }
                                        Adt114::C2(_, _, _, _) => {
                                            let v24: Adt119 = f358(v7, v8)?;
                                            Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (match v24.clone() {
                                                Adt119::C0(h676, _) => {
                                                    h676
                                                }
                                            }, match v24 {
                                                Adt119::C0(_, h677) => {
                                                    h677
                                                }
                                            })))
                                        }
                                        Adt114::C3(v25) => {
                                            Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (Adt113::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt114::C4(_, _, _, _) => {
                                            let v39: Adt119 = f358(v7, v8)?;
                                            Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (match v39.clone() {
                                                Adt119::C0(h681, _) => {
                                                    h681
                                                }
                                            }, match v39 {
                                                Adt119::C0(_, h682) => {
                                                    h682
                                                }
                                            })))
                                        }
                                        Adt114::C5(v26, v27, v28, v29) => {
                                            let m678 = f359(v26, v27, v28, v29.clone());
                                            match m678 {
                                                Adt135::C0(v30) => {
                                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (Adt113::C1(v29, v30), v8)))
                                                }
                                                Adt135::C1(v31) => {
                                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C2(v31)))
                                                }
                                                Adt135::C2(v32) => {
                                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C3(v32)))
                                                }
                                                Adt135::C3(v33) => {
                                                    let v34: List<Adt114> = {
                                                        let a679 = v33;
                                                        let a680 = v8;
                                                        append_list(a679, a680)
                                                    };
                                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt113::C1(v40, v41) => {
                                    let v42: Adt119 = f360(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (match v42.clone() {
                                        Adt119::C0(h683, _) => {
                                            h683
                                        }
                                    }, match v42 {
                                        Adt119::C0(_, h684) => {
                                            h684
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m685 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261() -> u64 {
    67108864u64
}

pub fn f262(v0: List<Adt112>) -> R<Str> {
    let a689 = List::cons(Str::lit(""), f361(v0, List::<Str>::nil())?);
    let a690 = Str::lit("/");
    Ok(join(a689, a690))
}

pub fn f263(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f362({
        let m693 = match v1.clone() {
            Adt50::C0(h692, _) => {
                h692
            }
        };
        match m693 {
            Adt38::C0(h691, _, _) => {
                h691
            }
        }
    })? {
        f363(v0, v1)
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f264(v0: Str, v1: Adt5) -> Adt116 {
    f364(v0, match v1 {
        Adt5::C0(h694) => {
            h694
        }
    })
}

pub fn f265(v0: Adt6) -> Option<Adt5> {
    let m695 = v0;
    match m695 {
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

pub fn f266(v0: Adt103, v1: Adt5) -> bool {
    let m696 = v0;
    match m696 {
        Adt103::C0 => {
            f365(v1)
        }
        Adt103::C1 => {
            f366(v1)
        }
        Adt103::C2 => {
            f367(v1)
        }
    }
}

pub fn f267(v0: Str, v1: Adt5, v2: Str) -> Str {
    let m697 = f264(v0, v1);
    match m697 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            let m698 = f190(v3);
            match m698 {
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

pub fn f268(v0: Str) -> Adt71 {
    Adt71::C0(Adt42::C0, Adt72::C0(Adt73::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))), None::<Adt5>)
}

pub fn f269(p701: u64, p702: i64) -> R<i64> {
    let m700 = (p701, p702);
    {
        let mut m700 = m700;
        loop {
            let step = {
                let (v0, v1) = m700;
                let m699 = v0;
                if m699 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m699 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f368(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Adt6) -> Option<List<Adt6>> {
    let m703 = v0;
    match m703 {
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

pub fn f271(v0: Adt5) -> bool {
    let m704 = f264(Str::lit("content"), v0);
    match m704 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v1) => {
            let m705 = f265(v1);
            match m705 {
                None => {
                    false
                }
                Some(v2) => {
                    let m706 = f264(Str::lit("ok"), v2);
                    match m706 {
                        Adt116::C0 => {
                            false
                        }
                        Adt116::C1(v3) => {
                            let m707 = f369(v3);
                            match m707 {
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

pub fn f272() -> Adt65 {
    f223(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f273(v0: Adt104) -> bool {
    let m708 = v0;
    match m708 {
        Adt104::C0 => {
            false
        }
        Adt104::C1 => {
            true
        }
        Adt104::C2(_) => {
            false
        }
    }
}

pub fn f274(v0: Adt104) -> Adt5 {
    let m709 = v0;
    match m709 {
        Adt104::C0 => {
            f27()
        }
        Adt104::C1 => {
            f27()
        }
        Adt104::C2(v1) => {
            v1
        }
    }
}

pub fn f275(v0: Adt104) -> bool {
    let m710 = v0;
    match m710 {
        Adt104::C0 => {
            true
        }
        Adt104::C1 => {
            false
        }
        Adt104::C2(_) => {
            false
        }
    }
}

pub fn f276() -> Adt65 {
    f223(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f277(v0: Str) -> Str {
    let v1: Bytes = {
        let a711 = v0;
        utf8_encode(a711)
    };
    let m718 = {
        let a715 = v1.clone();
        let a716 = 1u64;
        let a717 = {
            let a713 = {
                let a712 = v1;
                length_bytes(a712)
            };
            let a714 = 1u64;
            nat_sub(a713, a714)
        };
        slice_bytes(a715, a716, a717)
    };
    match m718 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m720 = {
                let a719 = v2;
                utf8_decode(a719)
            };
            match m720 {
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

pub fn f278(v0: Option<Adt5>) -> bool {
    let m721 = v0;
    match m721 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f279(p724: List<Str>, p725: List<Str>) -> List<Str> {
    let m723 = (p724, p725);
    {
        let mut m723 = m723;
        loop {
            let step = {
                let (v0, v1) = m723;
                let m722 = v0;
                match m722.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
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

pub fn f280(p729: List<(Str, Adt6)>, p730: List<Str>) -> List<Str> {
    let m728 = (p729, p730);
    {
        let mut m728 = m728;
        loop {
            let step = {
                let (v0, v1) = m728;
                let m726 = v0;
                match m726.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h727, _) = v2;
                            h727
                        }, v1)))
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

pub fn f281(v0: Adt14, v1: Adt36, v2: Adt22, v3: Adt100) -> R<Result<Adt33, Adt90>> {
    let m731 = v3;
    match m731 {
        Adt100::C0(v4) => {
            f263(v1, v4)
        }
        Adt100::C1(v6, v7, v8) => {
            if f370(v2) {
                f371(v0, v8)
            } else {
                f372(v6, v1, v7)
            }
        }
        Adt100::C2(v5) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C12(v5)))
        }
    }
}

pub fn f282(v0: Fn3, v1: Adt36, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    f373(v1, v0, v2)
}

pub fn f283(v0: Adt6) -> Option<i64> {
    let m732 = v0;
    match m732 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f374(match v2.clone() {
                Adt7::C0(_, h733) => {
                    h733
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h734, _) => {
                        h734
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

pub fn f284(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m735 = v0;
    match m735 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m736 = f375(Adt6::C5(v1))?;
            match m736 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m737 = f265(v2);
                    match m737 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h738) => {
                                    h738
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f285(v0: Adt38) -> R<Option<List<(Str, Adt6)>>> {
    let m740 = f376(match v0.clone() {
        Adt38::C0(h739, _, _) => {
            h739
        }
    })?;
    match m740 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a743 = f377(Str::lit("structuredContent"), match v0.clone() {
                    Adt38::C0(_, h741, _) => {
                        h741
                    }
                });
                let a744 = f378(Str::lit("isError"), match v0 {
                    Adt38::C0(_, _, h742) => {
                        h742
                    }
                });
                append_list(a743, a744)
            })))
        }
    }
}

pub fn f286() -> Adt87 {
    Adt87::C1(Adt81::C0(-32603i64, Str::lit("Internal error"), None::<Adt6>))
}

pub fn f287(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f269(v0, 0i64)?, 0i64))
}

pub fn f288(v0: Option<Adt5>, v1: Str) -> bool {
    let m745 = v0;
    match m745 {
        None => {
            false
        }
        Some(v2) => {
            f379(v1, v2)
        }
    }
}

pub fn f289(p749: List<Adt6>, p750: List<Adt85>) -> Option<List<Adt85>> {
    let m748 = (p749, p750);
    {
        let mut m748 = m748;
        loop {
            let step = {
                let (v0, v1) = m748;
                let m746 = v0;
                match m746.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(Some(f380(v1, List::<Adt85>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m747 = f381(v2);
                        match m747 {
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
                Ok(next) => { m748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: Adt85) -> Str {
    let m753 = f382(match v0 {
        Adt85::C0(h752, _, _) => {
            h752
        }
    });
    match m753 {
        Adt64::C0(h751, _, _, _, _, _) => {
            h751
        }
    }
}

pub fn f291(v0: Adt85, v1: i64) -> R<bool> {
    let m757 = {
        let m756 = f382(match v0.clone() {
            Adt85::C0(h755, _, _) => {
                h755
            }
        });
        match m756 {
            Adt64::C0(_, _, _, _, h754, _) => {
                h754
            }
        }
    };
    match m757 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a764 = {
                let m763 = {
                    let a761 = {
                        let a759 = v1;
                        let a760 = match v0 {
                            Adt85::C0(_, h758, _) => {
                                h758
                            }
                        };
                        int_sub(a759, a760)?
                    };
                    let a762 = v2;
                    compare(a761, a762)
                };
                match m763 {
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
            Ok(bool_not(a764))
        }
    }
}

pub fn f292() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f28(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f293(v0: Adt85, v1: Adt64, v2: i64, v3: Str) -> R<Adt105> {
    let m768 = {
        let m767 = match v0.clone() {
            Adt85::C0(_, _, h766) => {
                h766
            }
        };
        match m767 {
            Adt86::C0(_, h765, _, _) => {
                h765
            }
        }
    };
    match m768.uncons() {
        None => {
            if f383({
                let m771 = match v0.clone() {
                    Adt85::C0(_, _, h770) => {
                        h770
                    }
                };
                match m771 {
                    Adt86::C0(_, _, h769, _) => {
                        h769
                    }
                }
            }, match v0.clone() {
                Adt85::C0(_, h772, _) => {
                    h772
                }
            }, v2)? {
                Ok(f384(v0.clone(), f385({
                    let m775 = match v0.clone() {
                        Adt85::C0(_, _, h774) => {
                            h774
                        }
                    };
                    match m775 {
                        Adt86::C0(_, _, _, h773) => {
                            h773
                        }
                    }
                }, f386(v1, v3)), match v0 {
                    Adt85::C0(_, _, h776) => {
                        h776
                    }
                }))
            } else {
                Ok(f294(v0))
            }
        }
        Some((_, _)) => {
            if f383({
                let m779 = match v0.clone() {
                    Adt85::C0(_, _, h778) => {
                        h778
                    }
                };
                match m779 {
                    Adt86::C0(h777, _, _, _) => {
                        h777
                    }
                }
            }, match v0.clone() {
                Adt85::C0(_, h780, _) => {
                    h780
                }
            }, v2)? {
                Ok(f384(v0.clone(), Adt63::C1(f386(v1, v3), {
                    let m783 = match v0.clone() {
                        Adt85::C0(_, _, h782) => {
                            h782
                        }
                    };
                    match m783 {
                        Adt86::C0(_, h781, _, _) => {
                            h781
                        }
                    }
                }), f387(match v0 {
                    Adt85::C0(_, _, h784) => {
                        h784
                    }
                })))
            } else {
                Ok(f294(v0))
            }
        }
    }
}

pub fn f294(v0: Adt85) -> Adt105 {
    Adt105::C0(v0, false)
}

pub fn f295(v0: Adt85, v1: Adt64, v2: List<(Str, Adt65)>, v3: Adt5, v4: i64, v5: Str) -> R<Adt105> {
    let m785 = f388(v2.clone(), v3);
    match m785.uncons() {
        None => {
            f389(v0, v1, v4, v5)
        }
        Some((v6, v7)) => {
            let a788 = {
                let a786 = List::cons(v6.clone(), v7.clone());
                length_list(a786)
            };
            let a789 = {
                let a787 = v2;
                length_list(a787)
            };
            if nat_eq(a788, a789) {
                Ok(f294(v0))
            } else {
                Ok(f384(v0.clone(), Adt63::C1(f386(v1, v5), List::cons(v6, v7)), match v0 {
                    Adt85::C0(_, _, h790) => {
                        h790
                    }
                }))
            }
        }
    }
}

pub fn f296(v0: Adt105, v1: Str) -> Adt105 {
    let m794 = {
        let m793 = match v0.clone() {
            Adt105::C0(h792, _) => {
                h792
            }
        };
        match m793 {
            Adt85::C0(h791, _, _) => {
                h791
            }
        }
    };
    match m794 {
        Adt63::C0(v2) => {
            f384(match v0.clone() {
                Adt105::C0(h795, _) => {
                    h795
                }
            }, Adt63::C4(f386(v2, v1)), {
                let m798 = match v0 {
                    Adt105::C0(h797, _) => {
                        h797
                    }
                };
                match m798 {
                    Adt85::C0(_, _, h796) => {
                        h796
                    }
                }
            })
        }
        Adt63::C1(v3, _) => {
            f384(match v0.clone() {
                Adt105::C0(h799, _) => {
                    h799
                }
            }, Adt63::C4(f386(v3, v1)), {
                let m802 = match v0 {
                    Adt105::C0(h801, _) => {
                        h801
                    }
                };
                match m802 {
                    Adt85::C0(_, _, h800) => {
                        h800
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

pub fn f297(v0: List<(Adt45, List<Adt47>)>) -> List<Adt45> {
    f390(f391(v0, List::<Adt45>::nil()), List::<Adt45>::nil())
}

pub fn f298() -> List<(Adt45, List<Adt47>)> {
    List::cons(f392(f393(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f313(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f392(f393(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f394(Str::lit("test://static-binary"), Str::lit("image/png"), f314())), List::cons(f392(f393(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f313(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f392(f393(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f313(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt45, List<Adt47>)>::nil()))))
}

pub fn f299(v0: Adt10, v1: Str, v2: Option<List<Adt47>>) -> Result<Adt33, Adt90> {
    let m803 = v2;
    match m803 {
        None => {
            Err::<Adt33, Adt90>(Adt90::C2(v1))
        }
        Some(v3) => {
            let m804 = v3.clone();
            match m804.uncons() {
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

pub fn f300(v0: Str, p813: List<(Adt45, List<Adt47>)>) -> Option<List<Adt47>> {
    let m812 = p813;
    {
        let mut m812 = m812;
        loop {
            let step = {
                let v1 = m812;
                let m805 = v1;
                match m805.uncons() {
                    None => {
                        Err::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(None::<List<Adt47>>)
                    }
                    Some((v2, v3)) => {
                        let a809 = {
                            let m808 = {
                                let (h807, _) = v2;
                                h807
                            };
                            match m808 {
                                Adt45::C0(_, _, _, h806, _, _, _, _, _) => {
                                    h806
                                }
                            }
                        };
                        let a810 = v0.clone();
                        if equal(a809, a810) {
                            Err::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(Some({
                                let (_, h811) = v2;
                                h811
                            }))
                        } else {
                            Ok::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(v3)
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

pub fn f301(v0: Str) -> R<Option<List<Adt47>>> {
    let m814 = f395(v0.clone())?;
    match m814 {
        None => {
            Ok(None::<List<Adt47>>)
        }
        Some(v1) => {
            let m815 = f396(v1)?;
            match m815 {
                None => {
                    Ok(None::<List<Adt47>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f313(v0, Str::lit("application/json"), v2), List::<Adt47>::nil())))
                }
            }
        }
    }
}

pub fn f302() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f303(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt58>>) -> Adt57 {
    Adt57::C0(v0, Some(v1), None::<List<Adt2>>, Some(v2), v3, None::<Adt5>)
}

pub fn f304(v0: Str, v1: Str) -> Adt58 {
    Adt58::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f305(v0: Str, v1: Str) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v0.clone(), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v0), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f306(v0: Str, p821: List<Adt57>) -> Option<Adt57> {
    let m820 = p821;
    {
        let mut m820 = m820;
        loop {
            let step = {
                let v1 = m820;
                let m816 = v1;
                match m816.uncons() {
                    None => {
                        Err::<List<Adt57>, Option<Adt57>>(None::<Adt57>)
                    }
                    Some((v2, v3)) => {
                        let a818 = match v2.clone() {
                            Adt57::C0(h817, _, _, _, _, _) => {
                                h817
                            }
                        };
                        let a819 = v0.clone();
                        if equal(a818, a819) {
                            Err::<List<Adt57>, Option<Adt57>>(Some(v2))
                        } else {
                            Ok::<List<Adt57>, Option<Adt57>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307(p827: List<Adt58>, v1: List<(Str, Str)>) -> Option<Str> {
    let m826 = p827;
    {
        let mut m826 = m826;
        loop {
            let step = {
                let v0 = m826;
                let m822 = v0;
                match m822.uncons() {
                    None => {
                        Err::<List<Adt58>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f397(v2.clone()) {
                            let m824 = f398(match v2.clone() {
                                Adt58::C0(h823, _, _, _) => {
                                    h823
                                }
                            }, v1.clone());
                            match m824 {
                                None => {
                                    Err::<List<Adt58>, Option<Str>>(Some(match v2 {
                                        Adt58::C0(h825, _, _, _) => {
                                            h825
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
                Ok(next) => { m826 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f308(v0: Adt57) -> List<Adt58> {
    let m829 = match v0 {
        Adt57::C0(_, _, _, _, h828, _) => {
            h828
        }
    };
    match m829 {
        None => {
            List::<Adt58>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f309(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m830 = v0;
    match m830 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f310(v0: List<Adt60>) -> Adt59 {
    Adt59::C0(None::<Str>, v0)
}

pub fn f311(v0: Str) -> Adt60 {
    Adt60::C0(Adt42::C0, Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)))
}

pub fn f312(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m831 = f398(v0, v1);
    match m831 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f313(v0: Str, v1: Str, v2: Str) -> Adt47 {
    Adt47::C0(Adt48::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f314() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f315(v0: List<Str>) -> R<Adt61> {
    let v1: u64 = f399(v0.clone(), 0u64)?;
    Ok(Adt61::C0(Adt62::C0(f400(v0), Some(v1), Some({
        let a832 = f401();
        let a833 = v1;
        nat_lt(a832, a833)
    }))))
}

pub fn f316(p839: List<Adt58>, v1: Str) -> bool {
    let m838 = p839;
    {
        let mut m838 = m838;
        loop {
            let step = {
                let v0 = m838;
                let m834 = v0;
                match m834.uncons() {
                    None => {
                        Err::<List<Adt58>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a836 = match v2 {
                            Adt58::C0(h835, _, _, _) => {
                                h835
                            }
                        };
                        let a837 = v1.clone();
                        if equal(a836, a837) {
                            Err::<List<Adt58>, bool>(true)
                        } else {
                            Ok::<List<Adt58>, bool>(v3)
                        }
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

pub fn f317(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f279(f402(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f318(v0: Str, v1: Str) -> List<Str> {
    let a840 = v0;
    let a841 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a840, a841) {
        let a842 = v1;
        let a843 = Str::lit("resourceUri");
        equal(a842, a843)
    } else {
        false
    } {
        f403()
    } else {
        List::<Str>::nil()
    }
}

pub fn f319() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f320(v0: Bytes, p846: u64, p847: List<u64>) -> R<List<u64>> {
    let m845 = (p846, p847);
    {
        let mut m845 = m845;
        loop {
            let step = {
                let (v1, v2) = m845;
                let m844 = v1;
                if m844 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m844 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f126(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f321(v0: Bytes, p853: u64, p854: u64, p855: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt94>> {
    let m852 = (p853, (p854, p855));
    {
        let mut m852 = m852;
        loop {
            let step = {
                let (v1, (v2, v3)) = m852;
                let m848 = v2;
                if m848 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Ok::<(List<List<u64>>, u64), Adt94>((f49(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m848 - 1;
                    let m849 = f16(v0.clone(), v1)?;
                    match m849 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Err::<(List<List<u64>>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>((match v5.clone() {
                                Adt96::C0(_, h850) => {
                                    h850
                                }
                            }, (v4, List::cons(f229(match v5 {
                                Adt96::C0(h851, _) => {
                                    h851
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m852 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: Bytes, v1: Adt92, v2: Result<(Adt14, u64), Adt94>) -> Result<Adt92, Adt94> {
    let m856 = v2;
    match m856 {
        Ok(v4) => {
            let a859 = {
                let (_, h857) = v4.clone();
                h857
            };
            let a860 = {
                let a858 = v0;
                length_bytes(a858)
            };
            if nat_eq(a859, a860) {
                Ok::<Adt92, Adt94>(Adt92::C0(match v1.clone() {
                    Adt92::C0(h861, _, _, _, _, _, _, _) => {
                        h861
                    }
                }, match v1.clone() {
                    Adt92::C0(_, h862, _, _, _, _, _, _) => {
                        h862
                    }
                }, match v1.clone() {
                    Adt92::C0(_, _, h863, _, _, _, _, _) => {
                        h863
                    }
                }, match v1.clone() {
                    Adt92::C0(_, _, _, h864, _, _, _, _) => {
                        h864
                    }
                }, match v1.clone() {
                    Adt92::C0(_, _, _, _, h865, _, _, _) => {
                        h865
                    }
                }, match v1.clone() {
                    Adt92::C0(_, _, _, _, _, h866, _, _) => {
                        h866
                    }
                }, match v1 {
                    Adt92::C0(_, _, _, _, _, _, h867, _) => {
                        h867
                    }
                }, {
                    let (h868, _) = v4;
                    h868
                }))
            } else {
                Err::<Adt92, Adt94>(Adt94::C1({
                    let (_, h869) = v4;
                    h869
                }))
            }
        }
        Err(v3) => {
            Err::<Adt92, Adt94>(v3)
        }
    }
}

pub fn f323(v0: Adt92, v1: List<List<u64>>) -> Adt92 {
    Adt92::C0(match v0.clone() {
        Adt92::C0(h870, _, _, _, _, _, _, _) => {
            h870
        }
    }, match v0.clone() {
        Adt92::C0(_, h871, _, _, _, _, _, _) => {
            h871
        }
    }, match v0.clone() {
        Adt92::C0(_, _, h872, _, _, _, _, _) => {
            h872
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, h873, _, _, _, _) => {
            h873
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, _, h874, _, _, _) => {
            h874
        }
    }, v1, match v0.clone() {
        Adt92::C0(_, _, _, _, _, _, h875, _) => {
            h875
        }
    }, match v0 {
        Adt92::C0(_, _, _, _, _, _, _, h876) => {
            h876
        }
    })
}

pub fn f324(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt94>> {
    let a878 = v1;
    let a879 = {
        let a877 = v0.clone();
        length_bytes(a877)
    };
    if nat_eq(a878, a879) {
        Ok(Ok::<(Adt14, u64), Adt94>((Adt14::C0(0i64, Str::lit(""), Str::lit(""), f232()), v1)))
    } else {
        f404(v0, v1)
    }
}

pub fn f325(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt106>> {
    let m880 = f405(v0, List::cons(Adt120::C1(0u64, v1), List::<Adt120>::nil()), List::<u64>::nil())?;
    match m880 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt106>(f406(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt106>(v3))
        }
    }
}

pub fn f326() -> u64 {
    67108864u64
}

pub fn f327() -> Str {
    Str::lit("2.0")
}

pub fn f328(v0: Option<Adt102>) -> List<(Str, Adt6)> {
    let m881 = v0;
    match m881 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f407(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f329(v0: Adt81) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt81::C0(h882, _, _) => {
            h882
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt81::C0(_, h883, _) => {
            h883
        }
    })), f408(match v0 {
        Adt81::C0(_, _, h884) => {
            h884
        }
    }))))
}

pub fn f330(v0: Adt107) -> i64 {
    let m885 = v0;
    match m885 {
        Adt107::C0 => {
            f409()
        }
        Adt107::C1(_, _) => {
            f410()
        }
    }
}

pub fn f331(v0: Adt107) -> Str {
    let m886 = v0;
    match m886 {
        Adt107::C0 => {
            Str::lit("Parse error")
        }
        Adt107::C1(_, v2) => {
            f411(v2)
        }
    }
}

pub fn f332(v0: Adt107) -> u64 {
    let m887 = v0;
    match m887 {
        Adt107::C0 => {
            f240()
        }
        Adt107::C1(_, _) => {
            f240()
        }
    }
}

pub fn f333(v0: Adt107) -> Option<Adt102> {
    let m888 = v0;
    match m888 {
        Adt107::C0 => {
            None::<Adt102>
        }
        Adt107::C1(v1, _) => {
            v1
        }
    }
}

pub fn f334(v0: Adt14, v1: Adt0, v2: Fn5, v3: u64, v4: List<Adt93>, v5: Adt117) -> R<Adt95> {
    f412(v0, f413(v4.clone())?, f414(v1.clone(), v2, v3, Some(f415(v4, match v1 {
        Adt0::C0(_, _, _, _, h889, _, _) => {
            h889
        }
    }, f416(v5.clone()), match v5.clone() {
        Adt117::C0(_, h890, _) => {
            h890
        }
    })?), f417(), v5)?)
}

pub fn f335(v0: Adt5) -> R<Result<Adt117, Adt107>> {
    let m891 = f418(v0.clone())?;
    match m891 {
        Adt145::C0 => {
            let m892 = f264(Str::lit("method"), v0.clone());
            match m892 {
                Adt116::C0 => {
                    Ok(f336(None::<Adt102>, f419(v0)))
                }
                Adt116::C1(_) => {
                    Ok(f336(None::<Adt102>, if f420(v0) {
                        Adt108::C8
                    } else {
                        Adt108::C7
                    }))
                }
            }
        }
        Adt145::C1(v2) => {
            Ok(f421(v2, v0))
        }
        Adt145::C2 => {
            Ok(f336(None::<Adt102>, Adt108::C5))
        }
    }
}

pub fn f336(v0: Option<Adt102>, v1: Adt108) -> Result<Adt117, Adt107> {
    Err::<Adt117, Adt107>(Adt107::C1(v0, v1))
}

pub fn f337(v0: List<u64>) -> R<Adt126> {
    Ok(f422({
        let a893 = v0.clone();
        length_list(a893)
    }, f423(Adt127::C0, 0u64, 0u64, v0)?))
}

pub fn f338(p900: Adt128, p901: u64, p902: List<u64>) -> R<Result<Adt6, Adt106>> {
    let m899 = (p900, (p901, p902));
    {
        let mut m899 = m899;
        loop {
            let step = {
                let (v0, (v1, v2)) = m899;
                let m894 = v2;
                match m894.uncons() {
                    None => {
                        let m895 = f424(v0, v1)?;
                        match m895 {
                            Ok(v3) => {
                                Err::<(Adt128, (u64, List<u64>)), R<Result<Adt6, Adt106>>>(Ok(Ok::<Adt6, Adt106>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt128, (u64, List<u64>)), R<Result<Adt6, Adt106>>>(Ok(Err::<Adt6, Adt106>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m896 = f425(v0, v1, v5)?;
                        match m896 {
                            Adt146::C0(v7) => {
                                Err::<(Adt128, (u64, List<u64>)), R<Result<Adt6, Adt106>>>(Ok(Err::<Adt6, Adt106>(v7)))
                            }
                            Adt146::C1(v8) => {
                                Ok::<(Adt128, (u64, List<u64>)), R<Result<Adt6, Adt106>>>((v8, ({
                                    let a897 = v1;
                                    let a898 = 1u64;
                                    nat_add(a897, a898)?
                                }, v6)))
                            }
                        }
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

pub fn f339() -> Adt128 {
    Adt128::C0(Adt129::C0, List::<Adt133>::nil(), 0u64)
}

pub fn f340(p909: List<u64>, p910: List<u64>) -> R<bool> {
    let m908 = (p909, p910);
    {
        let mut m908 = m908;
        loop {
            let step = {
                let (v0, v1) = m908;
                let m903 = v0;
                match m903.uncons() {
                    None => {
                        let m904 = v1;
                        match m904.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m905 = v1;
                        match m905.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a906 = f426(v4)?;
                                let a907 = f426(v6)?;
                                if nat_eq(a906, a907) {
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
                Ok(next) => { m908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: Adt110, v1: List<u64>) -> Adt110 {
    let m911 = v0;
    match m911 {
        Adt110::C0 => {
            Adt110::C1(v1)
        }
        Adt110::C1(_) => {
            Adt110::C2
        }
        Adt110::C2 => {
            Adt110::C2
        }
    }
}

pub fn f342(v0: List<u64>) -> List<u64> {
    f350(f427(f350(f427(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f343(p914: List<u64>) -> bool {
    let m913 = p914;
    {
        let mut m913 = m913;
        loop {
            let step = {
                let v0 = m913;
                let m912 = v0;
                match m912.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f428(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m913 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f345() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f346() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f347(v0: List<u64>, p917: List<List<u64>>) -> R<bool> {
    let m916 = p917;
    {
        let mut m916 = m916;
        loop {
            let step = {
                let v1 = m916;
                let m915 = v1;
                match m915.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f340(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f348(v0: List<u64>) -> Option<List<u64>> {
    let m918 = v0;
    match m918.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m919 = v2;
            match m919.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a920 = v1;
                    let a921 = 47u64;
                    if if nat_eq(a920, a921) {
                        let a922 = v3;
                        let a923 = 47u64;
                        nat_eq(a922, a923)
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

pub fn f349(v0: List<u64>) -> bool {
    let m924 = v0;
    match m924.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a925 = v1;
            let a926 = 58u64;
            if nat_eq(a925, a926) {
                let m927 = v2.clone();
                match m927.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f429(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f350(p930: List<u64>, p931: List<u64>) -> List<u64> {
    let m929 = (p930, p931);
    {
        let mut m929 = m929;
        loop {
            let step = {
                let (v0, v1) = m929;
                let m928 = v0;
                match m928.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: List<u64>) -> bool {
    let m932 = v0.clone();
    match m932.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a936 = if f430(v0.clone(), false) {
                let a934 = {
                    let a933 = v0;
                    length_list(a933)
                };
                let a935 = f431();
                nat_le(a934, a935)
            } else {
                false
            };
            bool_not(a936)
        }
    }
}

pub fn f352(v0: Adt36) -> Option<Adt98> {
    let m941 = f432(match v0.clone() {
        Adt36::C0(h937, _, _, _, _, _, _, _) => {
            h937
        }
    }, {
        let m940 = match v0.clone() {
            Adt36::C0(_, _, _, _, h939, _, _, _) => {
                h939
            }
        };
        match m940 {
            Adt5::C0(h938) => {
                h938
            }
        }
    });
    match m941 {
        None => {
            f433(match v0.clone() {
                Adt36::C0(h942, _, _, _, _, _, _, _) => {
                    h942
                }
            }, {
                let m945 = match v0 {
                    Adt36::C0(_, _, _, _, h944, _, _, _) => {
                        h944
                    }
                };
                match m945 {
                    Adt5::C0(h943) => {
                        h943
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f353(v0: Adt36, v1: Result<Adt134, Adt98>) -> R<Result<List<Adt12>, Adt98>> {
    let m946 = v1;
    match m946 {
        Ok(v3) => {
            let m947 = f434(v0)?;
            match m947 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt98>(f148(match v3 {
                        Adt134::C0(h948, _) => {
                            h948
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

pub fn f354(p955: u64, v1: Str, v2: Adt6, p956: List<Adt118>, p957: Adt134) -> R<Result<Adt134, Adt98>> {
    let m954 = (p955, (p956, p957));
    {
        let mut m954 = m954;
        loop {
            let step = {
                let (v0, (v3, v4)) = m954;
                let m949 = v0;
                if m949 == 0 {
                    Err::<(u64, (List<Adt118>, Adt134)), R<Result<Adt134, Adt98>>>(Ok(Err::<Adt134, Adt98>(Adt98::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m949 - 1;
                    let m950 = v3;
                    match m950.uncons() {
                        None => {
                            Err::<(u64, (List<Adt118>, Adt134)), R<Result<Adt134, Adt98>>>(Ok(Ok::<Adt134, Adt98>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m951 = f435(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m951 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt118>, Adt134)), R<Result<Adt134, Adt98>>>((v5, ({
                                        let (h952, _) = v9;
                                        h952
                                    }, {
                                        let (_, h953) = v9;
                                        h953
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt118>, Adt134)), R<Result<Adt134, Adt98>>>(Ok(Err::<Adt134, Adt98>(v8)))
                                }
                            }
                        }
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

pub fn f355() -> u64 {
    67108864u64
}

pub fn f356() -> Adt134 {
    Adt134::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f357(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt112>) -> R<Adt135> {
    let m958 = f369(v1.clone());
    match m958 {
        None => {
            let m959 = f265(v1);
            match m959 {
                None => {
                    Ok(Adt135::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f436(v0, match v5 {
                        Adt5::C0(h960) => {
                            h960
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt135::C3(List::<Adt114>::nil()))
            } else {
                Ok(Adt135::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f358(v0: Adt114, v1: List<Adt114>) -> R<Adt119> {
    let m961 = v0;
    match m961 {
        Adt114::C0(_, _, _) => {
            Ok(Adt119::C0(Adt113::C0, v1))
        }
        Adt114::C1(_, _, _) => {
            Ok(Adt119::C0(Adt113::C0, v1))
        }
        Adt114::C2(v8, v9, v10, v11) => {
            Ok(f437(v8, v9, v10, {
                let a962 = v11;
                let a963 = 1u64;
                nat_add(a962, a963)?
            }, v1))
        }
        Adt114::C3(v12) => {
            Ok(Adt119::C0(Adt113::C1(v12, Str::lit("not")), v1))
        }
        Adt114::C4(v13, _, v15, v16) => {
            Ok(f438(v13, v15, v16, v1))
        }
        Adt114::C5(_, _, _, _) => {
            Ok(Adt119::C0(Adt113::C0, v1))
        }
    }
}

pub fn f359(v0: Adt115, v1: Adt6, v2: Adt6, v3: List<Adt112>) -> Adt135 {
    let m964 = f265(v1);
    match m964 {
        None => {
            Adt135::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m965 = v0;
            match m965 {
                Adt115::C0 => {
                    f439(match v4 {
                        Adt5::C0(h966) => {
                            h966
                        }
                    }, v2, v3)
                }
                Adt115::C1 => {
                    f440(match v4 {
                        Adt5::C0(h967) => {
                            h967
                        }
                    }, v2, v3)
                }
                Adt115::C2 => {
                    f441(match v4 {
                        Adt5::C0(h968) => {
                            h968
                        }
                    }, v2, v3)
                }
                Adt115::C3 => {
                    f442(match v4 {
                        Adt5::C0(h969) => {
                            h969
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f360(v0: Adt114, v1: List<Adt114>, v2: List<Adt112>, v3: Str) -> Adt119 {
    let m970 = v0;
    match m970 {
        Adt114::C0(_, _, _) => {
            Adt119::C0(Adt113::C1(v2, v3), v1)
        }
        Adt114::C1(v7, v8, v9) => {
            f443(v7, v8, v9, v1)
        }
        Adt114::C2(v10, v11, v12, v13) => {
            f437(v10, v11, v12, v13, v1)
        }
        Adt114::C3(_) => {
            Adt119::C0(Adt113::C0, v1)
        }
        Adt114::C4(_, v16, v17, v18) => {
            f438(v16, v17, v18, v1)
        }
        Adt114::C5(_, _, _, _) => {
            Adt119::C0(Adt113::C1(v2, v3), v1)
        }
    }
}

pub fn f361(p973: List<Adt112>, p974: List<Str>) -> R<List<Str>> {
    let m972 = (p973, p974);
    {
        let mut m972 = m972;
        loop {
            let step = {
                let (v0, v1) = m972;
                let m971 = v0;
                match m971.uncons() {
                    None => {
                        Err::<(List<Adt112>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt112>, List<Str>), R<List<Str>>>((v3, List::cons(f444(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m972 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(p983: List<Adt39>) -> R<bool> {
    let m982 = p983;
    {
        let mut m982 = m982;
        loop {
            let step = {
                let v0 = m982;
                let m975 = v0;
                match m975.uncons() {
                    None => {
                        Err::<List<Adt39>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m976 = v1;
                        match m976 {
                            Adt39::C0(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C1(v4) => {
                                if f445(match v4 {
                                    Adt43::C0(h977, _, _, _) => {
                                        h977
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C2(v5) => {
                                if f445(match v5 {
                                    Adt44::C0(h978, _, _, _) => {
                                        h978
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
                                let m980 = match v7 {
                                    Adt46::C0(h979, _, _) => {
                                        h979
                                    }
                                };
                                match m980 {
                                    Adt47::C0(_) => {
                                        Ok::<List<Adt39>, R<bool>>(v2)
                                    }
                                    Adt47::C1(v9) => {
                                        if f445(match v9 {
                                            Adt49::C0(_, _, h981, _) => {
                                                h981
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
                Ok(next) => { m982 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f446(match v1.clone() {
        Adt50::C0(h984, _) => {
            h984
        }
    }) {
        f447(v1)
    } else {
        let m986 = match v0 {
            Adt36::C0(_, _, _, _, _, h985, _, _) => {
                h985
            }
        };
        match m986 {
            None => {
                f447(v1)
            }
            Some(v2) => {
                f448(v2, v1)
            }
        }
    }
}

pub fn f364(v0: Str, p992: List<(Str, Adt6)>) -> Adt116 {
    let m991 = p992;
    {
        let mut m991 = m991;
        loop {
            let step = {
                let v1 = m991;
                let m987 = v1;
                match m987.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt116>(Adt116::C0)
                    }
                    Some((v2, v3)) => {
                        let m989 = f449({
                            let (h988, _) = v2;
                            h988
                        }, v0.clone());
                        match m989 {
                            Adt147::C0 => {
                                Err::<List<(Str, Adt6)>, Adt116>(Adt116::C1({
                                    let (_, h990) = v2;
                                    h990
                                }))
                            }
                            Adt147::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt116>(v3)
                            }
                        }
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

pub fn f365(v0: Adt5) -> bool {
    if f450(v0.clone()) {
        f451(v0)
    } else {
        false
    }
}

pub fn f366(v0: Adt5) -> bool {
    if f452(v0.clone()) {
        if f453(f264(Str::lit("model"), v0.clone())) {
            f454(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f367(v0: Adt5) -> bool {
    let m993 = f264(Str::lit("roots"), v0);
    match m993 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v1) => {
            let m994 = f270(v1);
            match m994 {
                None => {
                    false
                }
                Some(v2) => {
                    f455(v2)
                }
            }
        }
    }
}

pub fn f368(v0: i64, v1: i64) -> R<i64> {
    let a996 = v0;
    let a997 = {
        let a995 = v1;
        int_neg(a995)?
    };
    int_sub(a996, a997)
}

pub fn f369(v0: Adt6) -> Option<bool> {
    let m998 = v0;
    match m998 {
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

pub fn f370(v0: Adt22) -> bool {
    f202({
        let m1001 = match v0 {
            Adt22::C0(h1000, _, _, _, _) => {
                h1000
            }
        };
        match m1001 {
            Adt17::C0(_, h999, _, _, _, _) => {
                h999
            }
        }
    })
}

pub fn f371(v0: Adt14, v1: Adt86) -> R<Result<Adt33, Adt90>> {
    let a1003 = match v0.clone() {
        Adt14::C0(_, _, h1002, _) => {
            h1002
        }
    };
    let a1004 = Str::lit("");
    if equal(a1003, a1004) {
        Ok(Err::<Adt33, Adt90>(f456()))
    } else {
        let m1006 = f203(match v0.clone() {
            Adt14::C0(_, _, _, h1005) => {
                h1005
            }
        })?;
        match m1006 {
            None => {
                Ok(Err::<Adt33, Adt90>(f204()))
            }
            Some(_) => {
                Ok(Ok::<Adt33, Adt90>(Adt33::C13(Adt83::C0(Adt84::C0, f382({
                    let m1008 = f457(v0.clone(), v1.clone());
                    match m1008 {
                        Adt85::C0(h1007, _, _) => {
                            h1007
                        }
                    }
                }), f457(v0, v1)))))
            }
        }
    }
}

pub fn f372(v0: Adt101, v1: Adt36, v2: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1009 = v0;
    match m1009 {
        Adt101::C0 => {
            f263(v1, v2)
        }
        Adt101::C1 => {
            Ok(Err::<Adt33, Adt90>(f206()))
        }
    }
}

pub fn f373(v0: Adt36, v1: Fn3, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m1011 = {
        let c1010: Fn3 = v1;
        c1010.apply(v2)?
    };
    match m1011 {
        Ok(v4) => {
            f263(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f374(v0: i64) -> bool {
    let a1018 = {
        let m1014 = {
            let a1012 = v0;
            let a1013 = 0i64;
            compare(a1012, a1013)
        };
        if match m1014 {
            Ordering::Less => {
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
            let m1017 = {
                let a1015 = 0i64;
                let a1016 = v0;
                compare(a1015, a1016)
            };
            match m1017 {
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
    bool_not(a1018)
}

pub fn f375(v0: Adt6) -> R<Option<Adt6>> {
    f458(f459(), Adt136::C0(v0), List::<Adt137>::nil())
}

pub fn f376(v0: List<Adt39>) -> R<Option<List<Adt6>>> {
    let m1019 = f460(v0, List::<Adt6>::nil())?;
    match m1019 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f461(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f377(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1020 = v1;
    match m1020 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f378(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1021 = v1;
    match m1021 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f379(v0: Str, v1: Adt5) -> bool {
    f462(f264(v0, v1))
}

pub fn f380(p1024: List<Adt85>, p1025: List<Adt85>) -> List<Adt85> {
    let m1023 = (p1024, p1025);
    {
        let mut m1023 = m1023;
        loop {
            let step = {
                let (v0, v1) = m1023;
                let m1022 = v0;
                match m1022.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), List<Adt85>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt85>, List<Adt85>), List<Adt85>>((v3, List::cons(v2, v1)))
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

pub fn f381(v0: Adt6) -> Option<Adt85> {
    let m1026 = f265(v0);
    match m1026 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m1027 = f463(Str::lit("task"), v1.clone());
            match m1027 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m1028 = f464(v2);
                    match m1028 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m1029 = f465(Str::lit("createdMs"), v1.clone());
                            match m1029 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m1030 = f463(Str::lit("plan"), v1);
                                    match m1030 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            let m1031 = f466(v5);
                                            match m1031 {
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

pub fn f382(v0: Adt63) -> Adt64 {
    let m1032 = v0;
    match m1032 {
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

pub fn f383(v0: i64, v1: i64, v2: i64) -> R<bool> {
    let a1038 = {
        let m1037 = {
            let a1035 = {
                let a1033 = v2;
                let a1034 = v1;
                int_sub(a1033, a1034)?
            };
            let a1036 = v0;
            compare(a1035, a1036)
        };
        match m1037 {
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
    Ok(bool_not(a1038))
}

pub fn f384(v0: Adt85, v1: Adt63, v2: Adt86) -> Adt105 {
    Adt105::C0(Adt85::C0(v1, match v0 {
        Adt85::C0(_, h1039, _) => {
            h1039
        }
    }, v2), true)
}

pub fn f385(v0: Adt87, v1: Adt64) -> Adt63 {
    let m1040 = v0;
    match m1040 {
        Adt87::C0(v2) => {
            Adt63::C2(v1, v2)
        }
        Adt87::C1(v3) => {
            Adt63::C3(v1, v3)
        }
    }
}

pub fn f386(v0: Adt64, v1: Str) -> Adt64 {
    Adt64::C0(match v0.clone() {
        Adt64::C0(h1041, _, _, _, _, _) => {
            h1041
        }
    }, match v0.clone() {
        Adt64::C0(_, h1042, _, _, _, _) => {
            h1042
        }
    }, match v0.clone() {
        Adt64::C0(_, _, h1043, _, _, _) => {
            h1043
        }
    }, v1, match v0.clone() {
        Adt64::C0(_, _, _, _, h1044, _) => {
            h1044
        }
    }, match v0 {
        Adt64::C0(_, _, _, _, _, h1045) => {
            h1045
        }
    })
}

pub fn f387(v0: Adt86) -> Adt86 {
    Adt86::C0(match v0.clone() {
        Adt86::C0(h1046, _, _, _) => {
            h1046
        }
    }, List::<(Str, Adt65)>::nil(), match v0.clone() {
        Adt86::C0(_, _, h1047, _) => {
            h1047
        }
    }, match v0 {
        Adt86::C0(_, _, _, h1048) => {
            h1048
        }
    })
}

pub fn f388(v0: List<(Str, Adt65)>, v1: Adt5) -> List<(Str, Adt65)> {
    f467(f468(v0, v1, List::<(Str, Adt65)>::nil()), List::<(Str, Adt65)>::nil())
}

pub fn f389(v0: Adt85, v1: Adt64, v2: i64, v3: Str) -> R<Adt105> {
    let v4: Adt85 = Adt85::C0(Adt63::C0(f386(v1, v3.clone())), match v0.clone() {
        Adt85::C0(_, h1049, _) => {
            h1049
        }
    }, match v0 {
        Adt85::C0(_, _, h1050) => {
            h1050
        }
    });
    Ok(Adt105::C0({
        let m1052 = f208(v4, v2, v3)?;
        match m1052 {
            Adt105::C0(h1051, _) => {
                h1051
            }
        }
    }, true))
}

pub fn f390(p1055: List<Adt45>, p1056: List<Adt45>) -> List<Adt45> {
    let m1054 = (p1055, p1056);
    {
        let mut m1054 = m1054;
        loop {
            let step = {
                let (v0, v1) = m1054;
                let m1053 = v0;
                match m1053.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt45>), List<Adt45>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt45>), List<Adt45>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1054 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(p1060: List<(Adt45, List<Adt47>)>, p1061: List<Adt45>) -> List<Adt45> {
    let m1059 = (p1060, p1061);
    {
        let mut m1059 = m1059;
        loop {
            let step = {
                let (v0, v1) = m1059;
                let m1057 = v0;
                match m1057.uncons() {
                    None => {
                        Err::<(List<(Adt45, List<Adt47>)>, List<Adt45>), List<Adt45>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt45, List<Adt47>)>, List<Adt45>), List<Adt45>>((v3, List::cons({
                            let (h1058, _) = v2;
                            h1058
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1059 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: Adt45, v1: Adt47) -> (Adt45, List<Adt47>) {
    (v0, List::cons(v1, List::<Adt47>::nil()))
}

pub fn f393(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt45 {
    Adt45::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt41>, None::<u64>, None::<Adt5>)
}

pub fn f394(v0: Str, v1: Str, v2: Str) -> Adt47 {
    Adt47::C1(Adt49::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f395(v0: Str) -> R<Option<Str>> {
    let m1062 = f469(v0)?;
    match m1062 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a1063 = f470(f232(), v1);
            Ok(utf8_decode(a1063))
        }
    }
}

pub fn f396(v0: Str) -> R<Option<Str>> {
    let m1066 = f471(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a1064 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a1065 = Str::lit(" ");
        join(a1064, a1065)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m1066 {
        Ok(v1) => {
            let a1067 = v1;
            Ok(utf8_decode(a1067))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f397(v0: Adt58) -> bool {
    let m1069 = match v0 {
        Adt58::C0(_, _, _, h1068) => {
            h1068
        }
    };
    match m1069 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f398(v0: Str, p1076: List<(Str, Str)>) -> Option<Str> {
    let m1075 = p1076;
    {
        let mut m1075 = m1075;
        loop {
            let step = {
                let v1 = m1075;
                let m1070 = v1;
                match m1070.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a1072 = {
                            let (h1071, _) = v2;
                            h1071
                        };
                        let a1073 = v0.clone();
                        if equal(a1072, a1073) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h1074) = v2;
                                h1074
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(p1081: List<Str>, p1082: u64) -> R<u64> {
    let m1080 = (p1081, p1082);
    {
        let mut m1080 = m1080;
        loop {
            let step = {
                let (v0, v1) = m1080;
                let m1077 = v0;
                match m1077.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a1078 = v1;
                            let a1079 = 1u64;
                            nat_add(a1078, a1079)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1080 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: List<Str>) -> List<Str> {
    f279(f472(v0, f401(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f401() -> u64 {
    100u64
}

pub fn f402(v0: Str, p1085: List<Str>, p1086: List<Str>) -> R<List<Str>> {
    let m1084 = (p1085, p1086);
    {
        let mut m1084 = m1084;
        loop {
            let step = {
                let (v1, v2) = m1084;
                let m1083 = v1;
                match m1083.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f473(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403() -> List<Str> {
    f279(f474(f213(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f404(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt94>> {
    let m1087 = f475(v0.clone(), v1)?;
    match m1087 {
        None => {
            Ok(Err::<(Adt14, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m1090 = f476(v0.clone(), {
                let a1088 = v1;
                let a1089 = 8u64;
                nat_add(a1088, a1089)?
            })?;
            match m1090 {
                Ok(v4) => {
                    f477(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt14, u64), Adt94>(v3))
                }
            }
        }
    }
}

pub fn f405(p1095: u64, p1096: List<Adt120>, p1097: List<u64>) -> R<Result<List<u64>, Adt106>> {
    let m1094 = (p1095, (p1096, p1097));
    {
        let mut m1094 = m1094;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1094;
                let m1091 = v0;
                if m1091 == 0 {
                    Err::<(u64, (List<Adt120>, List<u64>)), R<Result<List<u64>, Adt106>>>(Ok(Err::<List<u64>, Adt106>(Adt106::C13)))
                } else {
                    let v3 = m1091 - 1;
                    let m1092 = v1;
                    match m1092.uncons() {
                        None => {
                            Err::<(u64, (List<Adt120>, List<u64>)), R<Result<List<u64>, Adt106>>>(Ok(Ok::<List<u64>, Adt106>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m1093 = f478(v4, v5, v2)?;
                            match m1093 {
                                Adt148::C0(v6) => {
                                    Err::<(u64, (List<Adt120>, List<u64>)), R<Result<List<u64>, Adt106>>>(Ok(Err::<List<u64>, Adt106>(v6)))
                                }
                                Adt148::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt120>, List<u64>)), R<Result<List<u64>, Adt106>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(p1100: List<u64>, p1101: List<u64>) -> List<u64> {
    let m1099 = (p1100, p1101);
    {
        let mut m1099 = m1099;
        loop {
            let step = {
                let (v0, v1) = m1099;
                let m1098 = v0;
                match m1098.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1099 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: Adt102) -> Adt6 {
    let m1102 = v0;
    match m1102 {
        Adt102::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt102::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f408(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1103 = v0;
    match m1103 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f409() -> i64 {
    -32700i64
}

pub fn f410() -> i64 {
    -32600i64
}

pub fn f411(v0: Adt108) -> Str {
    let m1104 = v0;
    match m1104 {
        Adt108::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt108::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt108::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt108::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt108::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt108::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt108::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt108::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt108::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f412(v0: Adt14, v1: bool, v2: Adt138) -> R<Adt95> {
    let m1106 = match v2.clone() {
        Adt138::C0(_, h1105) => {
            h1105
        }
    };
    match m1106 {
        Ok(v3) => {
            let m1107 = f479(v3.clone())?;
            match m1107 {
                None => {
                    Ok(f6())
                }
                Some(v4) => {
                    f480(v0, f481(match v3.clone() {
                        Adt139::C0(h1108, _, _) => {
                            h1108
                        }
                    }), f482(v1, f131(f483(), f484(match v2 {
                        Adt138::C0(h1109, _) => {
                            h1109
                        }
                    }, v4))?, f485(f486(match v3.clone() {
                        Adt139::C0(_, _, h1110) => {
                            h1110
                        }
                    }, List::<List<u64>>::nil())?, f487(f481(match v3 {
                        Adt139::C0(h1111, _, _) => {
                            h1111
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f131(f488(v5.clone()), f133(Some(match v2 {
                Adt138::C0(h1112, _) => {
                    h1112
                }
            }), f489(v5)?))
        }
    }
}

pub fn f413(p1117: List<Adt93>) -> R<bool> {
    let m1116 = p1117;
    {
        let mut m1116 = m1116;
        loop {
            let step = {
                let v0 = m1116;
                let m1113 = v0;
                match m1113.uncons() {
                    None => {
                        Err::<List<Adt93>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f340(match v1.clone() {
                            Adt93::C0(h1114, _) => {
                                h1114
                            }
                        }, f490())? {
                            if f491(f492(44u64, match v1 {
                                Adt93::C0(_, h1115) => {
                                    h1115
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
                Ok(next) => { m1116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414(v0: Adt0, v1: Fn5, v2: u64, v3: Option<Adt121>, v4: List<Str>, v5: Adt117) -> R<Adt138> {
    Ok(Adt138::C0(match v5.clone() {
        Adt117::C0(h1118, _, _) => {
            h1118
        }
    }, f493(v0, v1, v2, f494(v3, v4, v5)?)?))
}

pub fn f415(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt121> {
    let v4: Adt144 = f495(Adt123::C0, f141(v0.clone(), f496())?);
    let v5: Adt144 = f495(Adt123::C1, f141(v0.clone(), f497())?);
    let v6: Adt144 = if f498(v3) {
        f495(Adt123::C2, f499(f141(v0.clone(), f500())?)?)
    } else {
        f501()
    };
    Ok(Adt121::C0(match v4.clone() {
        Adt144::C0(h1119, _) => {
            h1119
        }
    }, match v5.clone() {
        Adt144::C0(h1120, _) => {
            h1120
        }
    }, match v6.clone() {
        Adt144::C0(h1121, _) => {
            h1121
        }
    }, f502(match v4 {
        Adt144::C0(_, h1122) => {
            h1122
        }
    }, f502(match v5 {
        Adt144::C0(_, h1123) => {
            h1123
        }
    }, match v6 {
        Adt144::C0(_, h1124) => {
            h1124
        }
    })), f503(v0, v1, v2)?))
}

pub fn f416(v0: Adt117) -> Option<Str> {
    let a1126 = match v0.clone() {
        Adt117::C0(_, h1125, _) => {
            h1125
        }
    };
    let a1127 = Str::lit("tools/call");
    if equal(a1126, a1127) {
        let m1129 = match v0 {
            Adt117::C0(_, _, h1128) => {
                h1128
            }
        };
        match m1129 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1130 = f264(Str::lit("name"), v1);
                match m1130 {
                    Adt116::C0 => {
                        None::<Str>
                    }
                    Adt116::C1(v2) => {
                        f190(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f417() -> List<Str> {
    List::cons(f504(), List::<Str>::nil())
}

pub fn f418(v0: Adt5) -> R<Adt145> {
    let m1131 = f264(Str::lit("id"), v0);
    match m1131 {
        Adt116::C0 => {
            Ok(Adt145::C0)
        }
        Adt116::C1(v1) => {
            let m1132 = v1;
            match m1132 {
                Adt6::C0 => {
                    Ok(Adt145::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt145::C2)
                }
                Adt6::C2(v3) => {
                    let m1133 = f505(v3)?;
                    match m1133 {
                        None => {
                            Ok(Adt145::C2)
                        }
                        Some(v4) => {
                            Ok(Adt145::C1(Adt102::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt145::C1(Adt102::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt145::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt145::C2)
                }
            }
        }
    }
}

pub fn f419(v0: Adt5) -> Adt108 {
    if f420(v0) {
        Adt108::C8
    } else {
        Adt108::C3
    }
}

pub fn f420(v0: Adt5) -> bool {
    if f379(Str::lit("result"), v0.clone()) {
        true
    } else {
        f379(Str::lit("error"), v0)
    }
}

pub fn f421(v0: Adt102, v1: Adt5) -> Result<Adt117, Adt107> {
    let m1134 = f264(Str::lit("jsonrpc"), v1.clone());
    match m1134 {
        Adt116::C0 => {
            f336(Some(v0), Adt108::C2)
        }
        Adt116::C1(v2) => {
            let m1135 = f190(v2);
            match m1135 {
                None => {
                    f336(Some(v0), Adt108::C2)
                }
                Some(v3) => {
                    let a1136 = v3;
                    let a1137 = f327();
                    if equal(a1136, a1137) {
                        f506(v0, v1)
                    } else {
                        f336(Some(v0), Adt108::C2)
                    }
                }
            }
        }
    }
}

pub fn f422(v0: u64, v1: u64) -> Adt126 {
    let a1138 = v1;
    let a1139 = 0u64;
    if nat_eq(a1138, a1139) {
        Adt126::C0
    } else {
        let a1140 = v1;
        let a1141 = 2u64;
        if nat_eq(a1140, a1141) {
            Adt126::C2(v0)
        } else {
            Adt126::C1({
                let a1145 = v0;
                let a1146 = {
                    let a1142 = v1;
                    let a1143 = 3u64;
                    let a1144 = 0u64;
                    nat_quot(a1142, a1143, a1144)
                };
                nat_sub(a1145, a1146)
            })
        }
    }
}

pub fn f423(p1174: Adt127, p1175: u64, p1176: u64, p1177: List<u64>) -> R<u64> {
    let m1173 = (p1174, (p1175, (p1176, p1177)));
    {
        let mut m1173 = m1173;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1173;
                let m1147 = v3;
                match m1147.uncons() {
                    None => {
                        let m1148 = v0;
                        match m1148 {
                            Adt127::C0 => {
                                Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt127::C1 => {
                                Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt127::C2 => {
                                Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt127::C3 => {
                                Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1149 = v0;
                        match m1149 {
                            Adt127::C0 => {
                                let a1150 = v4;
                                let a1151 = 128u64;
                                if nat_lt(a1150, a1151) {
                                    Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1152 = v4;
                                    let a1153 = 194u64;
                                    if nat_lt(a1152, a1153) {
                                        let v6: u64 = f507({
                                            let a1154 = List::cons(v4, v5);
                                            length_list(a1154)
                                        })?;
                                        Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1155 = v4;
                                        let a1156 = 224u64;
                                        if nat_lt(a1155, a1156) {
                                            Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1157 = v4;
                                            let a1158 = 224u64;
                                            if nat_eq(a1157, a1158) {
                                                Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1159 = v4;
                                                let a1160 = 237u64;
                                                if nat_eq(a1159, a1160) {
                                                    Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1161 = v4;
                                                    let a1162 = 240u64;
                                                    if nat_lt(a1161, a1162) {
                                                        Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1163 = v4;
                                                        let a1164 = 240u64;
                                                        if nat_eq(a1163, a1164) {
                                                            Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1165 = v4;
                                                            let a1166 = 244u64;
                                                            if nat_eq(a1165, a1166) {
                                                                Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1167 = v4;
                                                                let a1168 = 244u64;
                                                                if nat_lt(a1167, a1168) {
                                                                    Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f507({
                                                                        let a1169 = List::cons(v4, v5);
                                                                        length_list(a1169)
                                                                    })?;
                                                                    Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt127::C1 => {
                                if f508(v4, v1, v2) {
                                    Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f507({
                                        let a1170 = List::cons(v4, v5);
                                        length_list(a1170)
                                    })?;
                                    Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt127::C2 => {
                                if f508(v4, v1, v2) {
                                    Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f507({
                                        let a1171 = List::cons(v4, v5);
                                        length_list(a1171)
                                    })?;
                                    Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt127::C3 => {
                                if f508(v4, v1, v2) {
                                    Ok::<(Adt127, (u64, (u64, List<u64>))), R<u64>>((Adt127::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f507({
                                        let a1172 = List::cons(v4, v5);
                                        length_list(a1172)
                                    })?;
                                    Err::<(Adt127, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
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

pub fn f424(v0: Adt128, v1: u64) -> R<Result<Adt6, Adt106>> {
    let m1179 = match v0.clone() {
        Adt128::C0(h1178, _, _) => {
            h1178
        }
    };
    match m1179 {
        Adt129::C0 => {
            Ok(Err::<Adt6, Adt106>(Adt106::C3))
        }
        Adt129::C1 => {
            Ok(Err::<Adt6, Adt106>(Adt106::C3))
        }
        Adt129::C2 => {
            Ok(Err::<Adt6, Adt106>(Adt106::C3))
        }
        Adt129::C3 => {
            Ok(Err::<Adt6, Adt106>(Adt106::C3))
        }
        Adt129::C4 => {
            Ok(Err::<Adt6, Adt106>(Adt106::C3))
        }
        Adt129::C5 => {
            Ok(Err::<Adt6, Adt106>(Adt106::C3))
        }
        Adt129::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt106>(Adt106::C3))
        }
        Adt129::C7(_, _) => {
            Ok(Err::<Adt6, Adt106>(Adt106::C3))
        }
        Adt129::C8(v3) => {
            let m1180 = f509(v3)?;
            match m1180 {
                Adt151::C0 => {
                    Ok(Err::<Adt6, Adt106>(Adt106::C7(v1)))
                }
                Adt151::C1 => {
                    Ok(Err::<Adt6, Adt106>(Adt106::C8(v1)))
                }
                Adt151::C2(v4) => {
                    let m1185 = {
                        let m1184 = f510(v4, match v0.clone() {
                            Adt128::C0(_, h1182, _) => {
                                h1182
                            }
                        }, match v0 {
                            Adt128::C0(_, _, h1183) => {
                                h1183
                            }
                        });
                        match m1184 {
                            Adt128::C0(h1181, _, _) => {
                                h1181
                            }
                        }
                    };
                    match m1185 {
                        Adt129::C0 => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C1 => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C2 => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C3 => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C4 => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C5 => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C7(_, _) => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C8(_) => {
                            Ok(Err::<Adt6, Adt106>(Adt106::C3))
                        }
                        Adt129::C9(v5) => {
                            Ok(Ok::<Adt6, Adt106>(v5))
                        }
                    }
                }
            }
        }
        Adt129::C9(v2) => {
            Ok(Ok::<Adt6, Adt106>(v2))
        }
    }
}

pub fn f425(v0: Adt128, v1: u64, v2: u64) -> R<Adt146> {
    let m1187 = match v0.clone() {
        Adt128::C0(h1186, _, _) => {
            h1186
        }
    };
    match m1187 {
        Adt129::C0 => {
            if f511(v2) {
                Ok(Adt146::C1(v0))
            } else {
                f512(v0, v1, v2)
            }
        }
        Adt129::C1 => {
            if f511(v2) {
                Ok(Adt146::C1(v0))
            } else {
                let a1188 = v2;
                let a1189 = 93u64;
                if nat_eq(a1188, a1189) {
                    Ok(f513(v0, v1))
                } else {
                    f512(v0, v1, v2)
                }
            }
        }
        Adt129::C2 => {
            Ok(f514(v0, v1, v2))
        }
        Adt129::C3 => {
            if f511(v2) {
                Ok(Adt146::C1(v0))
            } else {
                let a1190 = v2;
                let a1191 = 125u64;
                if nat_eq(a1190, a1191) {
                    Ok(f515(v0, v1))
                } else {
                    let a1192 = v2;
                    let a1193 = 34u64;
                    if nat_eq(a1192, a1193) {
                        Ok(f516(v0, Adt129::C6(Adt130::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f517(Adt106::C2(v1)))
                    }
                }
            }
        }
        Adt129::C4 => {
            if f511(v2) {
                Ok(Adt146::C1(v0))
            } else {
                let a1194 = v2;
                let a1195 = 34u64;
                if nat_eq(a1194, a1195) {
                    Ok(f516(v0, Adt129::C6(Adt130::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f517(Adt106::C2(v1)))
                }
            }
        }
        Adt129::C5 => {
            if f511(v2) {
                Ok(Adt146::C1(v0))
            } else {
                let a1196 = v2;
                let a1197 = 58u64;
                if nat_eq(a1196, a1197) {
                    Ok(f516(v0, Adt129::C0))
                } else {
                    Ok(f517(Adt106::C2(v1)))
                }
            }
        }
        Adt129::C6(v3, v4, v5) => {
            f518(v0, v1, v2, v3, v4, v5)
        }
        Adt129::C7(v6, v7) => {
            f519(v0, v1, v2, v6, v7)
        }
        Adt129::C8(v8) => {
            f520(v0, v1, v2, v8)
        }
        Adt129::C9(_) => {
            Ok(f514(v0, v1, v2))
        }
    }
}

pub fn f426(v0: u64) -> R<u64> {
    let a1198 = 64u64;
    let a1199 = v0;
    if if nat_lt(a1198, a1199) {
        let a1200 = v0;
        let a1201 = 91u64;
        nat_lt(a1200, a1201)
    } else {
        false
    } {
        let a1202 = v0;
        let a1203 = 32u64;
        nat_add(a1202, a1203)
    } else {
        Ok(v0)
    }
}

pub fn f427(p1206: List<u64>) -> List<u64> {
    let m1205 = p1206;
    {
        let mut m1205 = m1205;
        loop {
            let step = {
                let v0 = m1205;
                let m1204 = v0;
                match m1204.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f521(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f428(v0: u64) -> bool {
    let a1207 = v0;
    let a1208 = 9u64;
    if nat_eq(a1207, a1208) {
        true
    } else {
        let a1209 = 31u64;
        let a1210 = v0;
        if nat_lt(a1209, a1210) {
            let a1211 = v0;
            let a1212 = 127u64;
            nat_lt(a1211, a1212)
        } else {
            false
        }
    }
}

pub fn f429(p1215: List<u64>) -> bool {
    let m1214 = p1215;
    {
        let mut m1214 = m1214;
        loop {
            let step = {
                let v0 = m1214;
                let m1213 = v0;
                match m1213.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f522(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f430(p1218: List<u64>, v1: bool) -> bool {
    let m1217 = p1218;
    {
        let mut m1217 = m1217;
        loop {
            let step = {
                let v0 = m1217;
                let m1216 = v0;
                match m1216.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f523(v2)
                        } else {
                            f524(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f431() -> u64 {
    64u64
}

pub fn f432(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1219 = f364(Str::lit("type"), v1);
    match m1219 {
        Adt116::C0 => {
            Some(Adt98::C3(v0))
        }
        Adt116::C1(v2) => {
            let m1220 = f190(v2);
            match m1220 {
                None => {
                    Some(Adt98::C3(v0))
                }
                Some(v3) => {
                    let a1221 = v3;
                    let a1222 = Str::lit("object");
                    if equal(a1221, a1222) {
                        None::<Adt98>
                    } else {
                        Some(Adt98::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f433(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1223 = f364(Str::lit("$schema"), v1);
    match m1223 {
        Adt116::C0 => {
            None::<Adt98>
        }
        Adt116::C1(v2) => {
            let m1224 = f190(v2);
            match m1224 {
                None => {
                    Some(Adt98::C2(v0))
                }
                Some(v3) => {
                    let a1225 = v3;
                    let a1226 = f525();
                    if equal(a1225, a1226) {
                        None::<Adt98>
                    } else {
                        Some(Adt98::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f434(v0: Adt36) -> R<Option<Adt98>> {
    let m1228 = match v0.clone() {
        Adt36::C0(_, _, _, _, _, h1227, _, _) => {
            h1227
        }
    };
    match m1228 {
        None => {
            Ok(None::<Adt98>)
        }
        Some(v1) => {
            let m1231 = f433(match v0.clone() {
                Adt36::C0(h1229, _, _, _, _, _, _, _) => {
                    h1229
                }
            }, match v1.clone() {
                Adt5::C0(h1230) => {
                    h1230
                }
            });
            match m1231 {
                None => {
                    let m1235 = f354(f355(), match v0 {
                        Adt36::C0(h1232, _, _, _, _, _, _, _) => {
                            h1232
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1233) => {
                            h1233
                        }
                    }), List::cons(Adt118::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1234) => {
                            h1234
                        }
                    }), false, List::<Str>::nil()), List::<Adt118>::nil()), f356())?;
                    match m1235 {
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

pub fn f435(v0: Str, v1: Adt6, v2: Adt118, v3: Adt134, v4: List<Adt118>) -> R<Result<(List<Adt118>, Adt134), Adt98>> {
    let m1236 = v2;
    match m1236 {
        Adt118::C0(v5, v6, v7) => {
            let m1237 = f369(v5.clone());
            match m1237 {
                None => {
                    let m1238 = f265(v5);
                    match m1238 {
                        None => {
                            Ok(Err::<(List<Adt118>, Adt134), Adt98>(Adt98::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f526(v0, v1, match v9 {
                                Adt5::C0(h1239) => {
                                    h1239
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt118>, Adt134), Adt98>((v4, v3)))
                }
            }
        }
    }
}

pub fn f436(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt112>) -> R<Adt135> {
    let m1240 = f527(v1.clone(), v2.clone())?;
    match m1240 {
        Adt135::C0(v4) => {
            Ok(Adt135::C0(v4))
        }
        Adt135::C1(v5) => {
            Ok(Adt135::C1(v5))
        }
        Adt135::C2(v6) => {
            Ok(Adt135::C2(v6))
        }
        Adt135::C3(_) => {
            f528(v0, v1, v2, v3)
        }
    }
}

pub fn f437(v0: List<Adt6>, v1: Adt6, v2: List<Adt112>, v3: u64, v4: List<Adt114>) -> Adt119 {
    let m1241 = v0;
    match m1241.uncons() {
        None => {
            let a1242 = v3;
            let a1243 = 1u64;
            if nat_eq(a1242, a1243) {
                Adt119::C0(Adt113::C0, v4)
            } else {
                Adt119::C0(Adt113::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt119::C0(Adt113::C0, List::cons(Adt114::C0(v5, v1.clone(), v2.clone()), List::cons(Adt114::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f438(v0: Option<Adt6>, v1: Adt6, v2: List<Adt112>, v3: List<Adt114>) -> Adt119 {
    let m1244 = v0;
    match m1244 {
        None => {
            Adt119::C0(Adt113::C0, v3)
        }
        Some(v4) => {
            Adt119::C0(Adt113::C0, List::cons(Adt114::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f439(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt112>) -> Adt135 {
    let m1245 = f364(Str::lit("anyOf"), v0);
    match m1245 {
        Adt116::C0 => {
            Adt135::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m1246 = f270(v3);
            match m1246 {
                None => {
                    Adt135::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1247 = v4;
                    match m1247.uncons() {
                        None => {
                            Adt135::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt135::C3(List::cons(Adt114::C0(v5, v1.clone(), v2.clone()), List::cons(Adt114::C1(v6, v1, v2), List::<Adt114>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f440(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt112>) -> Adt135 {
    let m1248 = f364(Str::lit("oneOf"), v0);
    match m1248 {
        Adt116::C0 => {
            Adt135::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m1249 = f270(v3);
            match m1249 {
                None => {
                    Adt135::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1250 = v4;
                    match m1250.uncons() {
                        None => {
                            Adt135::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt135::C3(List::cons(Adt114::C0(v5, v1.clone(), v2.clone()), List::cons(Adt114::C2(v6, v1, v2, 0u64), List::<Adt114>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f441(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt112>) -> Adt135 {
    let m1251 = f364(Str::lit("not"), v0);
    match m1251 {
        Adt116::C0 => {
            Adt135::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            Adt135::C3(List::cons(Adt114::C0(v3, v1, v2.clone()), List::cons(Adt114::C3(v2), List::<Adt114>::nil())))
        }
    }
}

pub fn f442(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt112>) -> Adt135 {
    let m1252 = f364(Str::lit("if"), v0.clone());
    match m1252 {
        Adt116::C0 => {
            Adt135::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            Adt135::C3(List::cons(Adt114::C0(v3, v1.clone(), v2.clone()), List::cons(Adt114::C4(f529(Str::lit("then"), v0.clone()), f529(Str::lit("else"), v0), v1, v2), List::<Adt114>::nil())))
        }
    }
}

pub fn f443(v0: List<Adt6>, v1: Adt6, v2: List<Adt112>, v3: List<Adt114>) -> Adt119 {
    let m1253 = v0;
    match m1253.uncons() {
        None => {
            Adt119::C0(Adt113::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt119::C0(Adt113::C0, List::cons(Adt114::C0(v4, v1.clone(), v2.clone()), List::cons(Adt114::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f444(v0: Adt112) -> R<Str> {
    let m1254 = v0;
    match m1254 {
        Adt112::C0(v1) => {
            f530(v1)
        }
        Adt112::C1(v2) => {
            let a1255 = f269(v2, 0i64)?;
            Ok(format_i64(a1255))
        }
    }
}

pub fn f445(v0: Str) -> R<bool> {
    let m1257 = f531(f229({
        let a1256 = v0;
        utf8_encode(a1256)
    })?)?;
    match m1257 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f446(v0: Adt38) -> bool {
    let m1259 = match v0 {
        Adt38::C0(_, _, h1258) => {
            h1258
        }
    };
    match m1259 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f447(v0: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1261 = match v0.clone() {
        Adt50::C0(_, h1260) => {
            h1260
        }
    };
    match m1261.uncons() {
        None => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(match v0 {
                Adt50::C0(h1262, _) => {
                    h1262
                }
            })))
        }
        Some((_, _)) => {
            if f532(match v0.clone() {
                Adt50::C0(_, h1263) => {
                    h1263
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
    let m1267 = {
        let m1266 = match v1.clone() {
            Adt50::C0(h1265, _) => {
                h1265
            }
        };
        match m1266 {
            Adt38::C0(_, h1264, _) => {
                h1264
            }
        }
    };
    match m1267 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1270 = f155(Adt6::C5(match v0.clone() {
                Adt5::C0(h1268) => {
                    h1268
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1269) => {
                    h1269
                }
            }), v2)?;
            match m1270 {
                Adt111::C0 => {
                    f447(v1)
                }
                Adt111::C1(_, _) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt111::C2(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt111::C3(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt111::C4 => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f449(v0: Str, v1: Str) -> Adt147 {
    let a1271 = v0;
    let a1272 = v1;
    if equal(a1271, a1272) {
        Adt147::C0
    } else {
        Adt147::C1
    }
}

pub fn f450(v0: Adt5) -> bool {
    if f533(f264(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f533(f264(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f533(f264(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f451(v0: Adt5) -> bool {
    let m1273 = f264(Str::lit("content"), v0);
    match m1273 {
        Adt116::C0 => {
            true
        }
        Adt116::C1(v1) => {
            let m1274 = f265(v1);
            match m1274 {
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

pub fn f452(v0: Adt5) -> bool {
    if f533(f264(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f533(f264(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f453(v0: Adt116) -> bool {
    let m1275 = v0;
    match m1275 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v1) => {
            let m1276 = f190(v1);
            match m1276 {
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

pub fn f454(v0: Adt5) -> bool {
    let m1277 = f264(Str::lit("content"), v0);
    match m1277 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v1) => {
            let m1278 = f265(v1.clone());
            match m1278 {
                None => {
                    let m1279 = f270(v1);
                    match m1279 {
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

pub fn f455(p1283: List<Adt6>) -> bool {
    let m1282 = p1283;
    {
        let mut m1282 = m1282;
        loop {
            let step = {
                let v0 = m1282;
                let m1280 = v0;
                match m1280.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m1281 = f265(v1);
                        match m1281 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f453(f264(Str::lit("uri"), v3)) {
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
                Ok(next) => { m1282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456() -> Adt90 {
    Adt90::C9(Str::lit("the host supplied no task id"))
}

pub fn f457(v0: Adt14, v1: Adt86) -> Adt85 {
    Adt85::C0(Adt63::C0(Adt64::C0(match v0.clone() {
        Adt14::C0(_, _, h1284, _) => {
            h1284
        }
    }, None::<Str>, match v0.clone() {
        Adt14::C0(_, h1285, _, _) => {
            h1285
        }
    }, match v0.clone() {
        Adt14::C0(_, h1286, _, _) => {
            h1286
        }
    }, Some(f534()), Some(f535()))), match v0 {
        Adt14::C0(h1287, _, _, _) => {
            h1287
        }
    }, v1)
}

pub fn f458(p1302: u64, p1303: Adt136, p1304: List<Adt137>) -> R<Option<Adt6>> {
    let m1301 = (p1302, (p1303, p1304));
    {
        let mut m1301 = m1301;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1301;
                let m1288 = v0;
                if m1288 == 0 {
                    Err::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1288 - 1;
                    let m1289 = v1;
                    match m1289 {
                        Adt136::C0(v4) => {
                            let m1290 = v4.clone();
                            match m1290 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f536(v7)?;
                                    Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1291 = v9;
                                    match m1291.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C0(v10), List::cons(Adt137::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1292 = v12;
                                    match m1292.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f536({
                                                let (h1293, _) = v13;
                                                h1293
                                            })?;
                                            Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C0({
                                                let (_, h1294) = v13;
                                                h1294
                                            }), List::cons(Adt137::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt136::C1(v16) => {
                            let m1295 = v2;
                            match m1295.uncons() {
                                None => {
                                    Err::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1296 = v17;
                                    match m1296 {
                                        Adt137::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1297 = v20;
                                            match m1297.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C1(Adt6::C4(f461(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C0(v22), List::cons(Adt137::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt137::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1298 = v26;
                                            match m1298.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C1(Adt6::C5(f537(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f536({
                                                        let (h1299, _) = v28;
                                                        h1299
                                                    })?;
                                                    Ok::<(u64, (Adt136, List<Adt137>)), R<Option<Adt6>>>((v3, (Adt136::C0({
                                                        let (_, h1300) = v28;
                                                        h1300
                                                    }), List::cons(Adt137::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m1301 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459() -> u64 {
    67108864u64
}

pub fn f460(p1308: List<Adt39>, p1309: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1307 = (p1308, p1309);
    {
        let mut m1307 = m1307;
        loop {
            let step = {
                let (v0, v1) = m1307;
                let m1305 = v0;
                match m1305.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1306 = f538(v2)?;
                        match m1306 {
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
                Ok(next) => { m1307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(p1312: List<Adt6>, p1313: List<Adt6>) -> List<Adt6> {
    let m1311 = (p1312, p1313);
    {
        let mut m1311 = m1311;
        loop {
            let step = {
                let (v0, v1) = m1311;
                let m1310 = v0;
                match m1310.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Adt116) -> bool {
    let m1314 = v0;
    match m1314 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(_) => {
            true
        }
    }
}

pub fn f463(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m1315 = f264(v0, v1);
    match m1315 {
        Adt116::C0 => {
            None::<Adt5>
        }
        Adt116::C1(v2) => {
            f265(v2)
        }
    }
}

pub fn f464(v0: Adt5) -> Option<Adt63> {
    let m1316 = f539(v0.clone());
    match m1316 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m1317 = f540(Str::lit("status"), v0.clone());
            match m1317 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let a1318 = v2.clone();
                    let a1319 = Str::lit("working");
                    if equal(a1318, a1319) {
                        Some(Adt63::C0(v1))
                    } else {
                        let a1320 = v2.clone();
                        let a1321 = Str::lit("cancelled");
                        if equal(a1320, a1321) {
                            Some(Adt63::C4(v1))
                        } else {
                            let a1322 = v2.clone();
                            let a1323 = Str::lit("input_required");
                            if equal(a1322, a1323) {
                                let m1324 = f541(Str::lit("inputRequests"), v0);
                                match m1324 {
                                    None => {
                                        None::<Adt63>
                                    }
                                    Some(v3) => {
                                        Some(Adt63::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a1325 = v2.clone();
                                let a1326 = Str::lit("completed");
                                if equal(a1325, a1326) {
                                    let m1327 = f463(Str::lit("result"), v0);
                                    match m1327 {
                                        None => {
                                            None::<Adt63>
                                        }
                                        Some(v4) => {
                                            Some(Adt63::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a1328 = v2;
                                    let a1329 = Str::lit("failed");
                                    if equal(a1328, a1329) {
                                        let m1330 = f463(Str::lit("error"), v0);
                                        match m1330 {
                                            None => {
                                                None::<Adt63>
                                            }
                                            Some(v5) => {
                                                let m1331 = f542(v5);
                                                match m1331 {
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

pub fn f465(v0: Str, v1: Adt5) -> Option<i64> {
    let m1332 = f264(v0, v1);
    match m1332 {
        Adt116::C0 => {
            None::<i64>
        }
        Adt116::C1(v2) => {
            f283(v2)
        }
    }
}

pub fn f466(v0: Adt5) -> Option<Adt86> {
    let m1333 = f465(Str::lit("parkAfterMs"), v0.clone());
    match m1333 {
        None => {
            None::<Adt86>
        }
        Some(v1) => {
            let m1334 = f541(Str::lit("parked"), v0.clone());
            match m1334 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m1335 = f465(Str::lit("finishAfterMs"), v0.clone());
                    match m1335 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            let m1336 = f463(Str::lit("outcome"), v0);
                            match m1336 {
                                None => {
                                    None::<Adt86>
                                }
                                Some(v4) => {
                                    let m1337 = f543(v4);
                                    match m1337 {
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

pub fn f467(p1340: List<(Str, Adt65)>, p1341: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m1339 = (p1340, p1341);
    {
        let mut m1339 = m1339;
        loop {
            let step = {
                let (v0, v1) = m1339;
                let m1338 = v0;
                match m1338.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f468(p1345: List<(Str, Adt65)>, v1: Adt5, p1346: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m1344 = (p1345, p1346);
    {
        let mut m1344 = m1344;
        loop {
            let step = {
                let (v0, v2) = m1344;
                let m1342 = v0;
                match m1342.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v2)
                    }
                    Some((v3, v4)) => {
                        if f379({
                            let (h1343, _) = v3.clone();
                            h1343
                        }, v1.clone()) {
                            Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v4, v2))
                        } else {
                            Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v4, List::cons(v3, v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: Str) -> R<Option<List<u64>>> {
    let m1347 = f544(f545(f546())?, f545(v0)?);
    match m1347 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1348 = f544(f547(List::cons(47u64, f545(f548())?)), f547(v1));
            match m1348 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f547(v2);
                    if if f549(v3.clone()) {
                        f550(v3.clone())
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

pub fn f470(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1349 = v0;
    let a1350 = f551(f232(), f552(List::<Bytes>::nil(), v1));
    append_bytes(a1349, a1350)
}

pub fn f471(v0: Adt6) -> R<Result<Bytes, Adt106>> {
    let m1351 = f233(v0)?;
    match m1351 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt106>(f470(f232(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt106>(v2))
        }
    }
}

pub fn f472(p1355: List<Str>, p1356: u64, p1357: List<Str>) -> List<Str> {
    let m1354 = (p1355, (p1356, p1357));
    {
        let mut m1354 = m1354;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1354;
                let m1352 = v0;
                match m1352.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m1353 = v1;
                        if m1353 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m1353 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: Str, v1: Str) -> R<bool> {
    Ok(f553(f545(v0)?, f545(v1)?))
}

pub fn f474(p1361: List<Adt45>, p1362: List<Str>) -> List<Str> {
    let m1360 = (p1361, p1362);
    {
        let mut m1360 = m1360;
        loop {
            let step = {
                let (v0, v1) = m1360;
                let m1358 = v0;
                match m1358.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt45::C0(_, _, _, h1359, _, _, _, _, _) => {
                                h1359
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a1366 = {
        let a1363 = v1;
        let a1364 = 8u64;
        nat_add(a1363, a1364)?
    };
    let a1367 = {
        let a1365 = v0.clone();
        length_bytes(a1365)
    };
    if nat_le(a1366, a1367) {
        Ok(Some(f554(f554(f554(f554(f554(f554(f554(f554(0i64, f126(v0.clone(), v1)?)?, f126(v0.clone(), {
            let a1368 = v1;
            let a1369 = 1u64;
            nat_add(a1368, a1369)?
        })?)?, f126(v0.clone(), {
            let a1370 = v1;
            let a1371 = 2u64;
            nat_add(a1370, a1371)?
        })?)?, f126(v0.clone(), {
            let a1372 = v1;
            let a1373 = 3u64;
            nat_add(a1372, a1373)?
        })?)?, f126(v0.clone(), {
            let a1374 = v1;
            let a1375 = 4u64;
            nat_add(a1374, a1375)?
        })?)?, f126(v0.clone(), {
            let a1376 = v1;
            let a1377 = 5u64;
            nat_add(a1376, a1377)?
        })?)?, f126(v0.clone(), {
            let a1378 = v1;
            let a1379 = 6u64;
            nat_add(a1378, a1379)?
        })?)?, f126(v0, {
            let a1380 = v1;
            let a1381 = 7u64;
            nat_add(a1380, a1381)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f476(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt94>> {
    let m1382 = f16(v0, v1)?;
    match m1382 {
        None => {
            Ok(Err::<(Str, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m1384 = f555(f229(match v2.clone() {
                Adt96::C0(h1383, _) => {
                    h1383
                }
            })?);
            match m1384 {
                None => {
                    Ok(Err::<(Str, u64), Adt94>(Adt94::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt94>((v3, match v2 {
                        Adt96::C0(_, h1385) => {
                            h1385
                        }
                    })))
                }
            }
        }
    }
}

pub fn f477(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt14, u64), Adt94>> {
    let m1387 = f476(v0.clone(), {
        let (_, h1386) = v2;
        h1386
    })?;
    match m1387 {
        Ok(v4) => {
            f556(v0, v1, {
                let (h1388, _) = v2;
                h1388
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt14, u64), Adt94>(v3))
        }
    }
}

pub fn f478(v0: Adt120, v1: List<Adt120>, v2: List<u64>) -> R<Adt148> {
    let m1389 = v0;
    match m1389 {
        Adt120::C0(v3) => {
            Ok(Adt148::C1(v1, f406(v3, v2)))
        }
        Adt120::C1(v4, v5) => {
            f557(v4, v5, v1, v2)
        }
        Adt120::C2(v6, v7) => {
            let m1390 = v7;
            match m1390.uncons() {
                None => {
                    Ok(Adt148::C1(List::cons(Adt120::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt148::C1(List::cons(Adt120::C1(v6, v8), List::cons(Adt120::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt120::C3(v10, v11) => {
            let m1391 = v11.clone();
            match m1391.uncons() {
                None => {
                    Ok(Adt148::C1(List::cons(Adt120::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt148::C1(List::cons(Adt120::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt120::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt120::C4(v14, v15) => {
            let m1392 = v15;
            match m1392.uncons() {
                None => {
                    Ok(Adt148::C1(List::cons(Adt120::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt148::C1(List::cons(Adt120::C0({
                        let a1394 = f558({
                            let (h1393, _) = v16;
                            h1393
                        })?;
                        let a1395 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1394, a1395)
                    }), List::cons(Adt120::C1(v14, {
                        let (_, h1396) = v16;
                        h1396
                    }), List::cons(Adt120::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt120::C5(v18, v19) => {
            let m1397 = v19.clone();
            match m1397.uncons() {
                None => {
                    Ok(Adt148::C1(List::cons(Adt120::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt148::C1(List::cons(Adt120::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt120::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f479(v0: Adt139) -> R<Option<Adt5>> {
    let m1399 = f559(match v0.clone() {
        Adt139::C0(h1398, _, _) => {
            h1398
        }
    })?;
    match m1399 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1402 = v1;
                let a1403 = {
                    let m1401 = f560(v0);
                    match m1401 {
                        Adt5::C0(h1400) => {
                            h1400
                        }
                    }
                };
                append_list(a1402, a1403)
            })))
        }
    }
}

pub fn f480(v0: Adt14, v1: Option<Adt88>, v2: Adt95) -> R<Adt95> {
    let m1404 = v1;
    match m1404 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a1406 = match v2.clone() {
                Adt95::C0(h1405, _, _, _, _) => {
                    h1405
                }
            };
            let a1407 = f483();
            if nat_eq(a1406, a1407) {
                Ok(f561(f562(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f481(v0: Adt33) -> Option<Adt88> {
    let m1408 = v0;
    match m1408 {
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
                Adt83::C0(_, _, h1409) => {
                    h1409
                }
            }, Adt89::C0, false))
        }
        Adt33::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f482(v0: bool, v1: Adt95, v2: Option<List<List<u64>>>) -> Adt95 {
    let m1410 = v2;
    match m1410 {
        None => {
            f6()
        }
        Some(v3) => {
            f563(v0, v1, f49(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f483() -> u64 {
    200u64
}

pub fn f484(v0: Adt102, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f327())), List::cons((Str::lit("id"), f407(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1411) => {
            h1411
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f485(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m1412 = v0;
    match m1412 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m1413 = v1;
            match m1413 {
                None => {
                    None::<List<List<u64>>>
                }
                Some(v3) => {
                    Some(f49(v3, v2))
                }
            }
        }
    }
}

pub fn f486(p1417: List<Adt141>, p1418: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1416 = (p1417, p1418);
    {
        let mut m1416 = m1416;
        loop {
            let step = {
                let (v0, v1) = m1416;
                let m1414 = v0;
                match m1414.uncons() {
                    None => {
                        Err::<(List<Adt141>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1415 = f564(v2)?;
                        match m1415 {
                            None => {
                                Err::<(List<Adt141>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt141>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
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

pub fn f487(v0: Option<Adt88>) -> R<Option<List<List<u64>>>> {
    let m1419 = v0;
    match m1419 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f565(v1)
        }
    }
}

pub fn f488(v0: Adt142) -> u64 {
    let m1420 = v0;
    match m1420 {
        Adt142::C0(_) => {
            f240()
        }
        Adt142::C1(_, _) => {
            f240()
        }
        Adt142::C2(_) => {
            f240()
        }
        Adt142::C3(_) => {
            f566()
        }
        Adt142::C4(v6) => {
            f567(v6)
        }
    }
}

pub fn f489(v0: Adt142) -> R<Adt81> {
    Ok(Adt81::C0(f568(v0.clone()), f569(v0.clone()), f570(v0)?))
}

pub fn f490() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f491(p1423: List<List<u64>>) -> R<bool> {
    let m1422 = p1423;
    {
        let mut m1422 = m1422;
        loop {
            let step = {
                let v0 = m1422;
                let m1421 = v0;
                match m1421.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f571(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(v0: u64, p1428: List<u64>, p1429: List<u64>, p1430: List<List<u64>>) -> List<List<u64>> {
    let m1427 = (p1428, (p1429, p1430));
    {
        let mut m1427 = m1427;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1427;
                let m1424 = v1;
                match m1424.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f49(List::cons(f350(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1425 = v4;
                        let a1426 = v0;
                        if nat_eq(a1425, a1426) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f350(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: Adt0, v1: Fn5, v2: u64, v3: Result<Adt149, Adt142>) -> R<Result<Adt139, Adt142>> {
    let m1431 = v3;
    match m1431 {
        Ok(v5) => {
            f572(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt139, Adt142>(v4))
        }
    }
}

pub fn f494(v0: Option<Adt121>, v1: List<Str>, v2: Adt117) -> R<Result<Adt149, Adt142>> {
    Ok(f573(f574(match v2.clone() {
        Adt117::C0(_, h1432, _) => {
            h1432
        }
    }), f575(v0, v1, v2)?))
}

pub fn f495(v0: Adt123, v1: Adt109) -> Adt144 {
    let m1433 = v1;
    match m1433 {
        Adt109::C0 => {
            f501()
        }
        Adt109::C1 => {
            Adt144::C0(None::<Str>, Some(Adt122::C6(v0)))
        }
        Adt109::C2 => {
            Adt144::C0(None::<Str>, Some(Adt122::C7(v0)))
        }
        Adt109::C3(v2) => {
            let m1434 = f555(v2);
            match m1434 {
                None => {
                    Adt144::C0(None::<Str>, Some(Adt122::C6(v0)))
                }
                Some(v3) => {
                    Adt144::C0(Some(v3), None::<Adt122>)
                }
            }
        }
    }
}

pub fn f496() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f497() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f498(v0: Str) -> bool {
    let m1435 = f576(v0);
    match m1435 {
        Adt156::C0 => {
            false
        }
        Adt156::C1 => {
            true
        }
        Adt156::C2 => {
            true
        }
        Adt156::C3 => {
            true
        }
    }
}

pub fn f499(v0: Adt109) -> R<Adt109> {
    let m1436 = v0.clone();
    match m1436 {
        Adt109::C0 => {
            Ok(v0)
        }
        Adt109::C1 => {
            Ok(v0)
        }
        Adt109::C2 => {
            Ok(v0)
        }
        Adt109::C3(v1) => {
            let m1437 = f577(v1);
            match m1437 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1438 = f531(v2)?;
                    match m1438 {
                        None => {
                            Ok(Adt109::C1)
                        }
                        Some(v3) => {
                            Ok(Adt109::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f500() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f501() -> Adt144 {
    Adt144::C0(None::<Str>, None::<Adt122>)
}

pub fn f502(v0: Option<Adt122>, v1: Option<Adt122>) -> Option<Adt122> {
    let m1439 = v0.clone();
    match m1439 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f503(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt124>> {
    Ok(f578(f579(v0, v1, v2, List::<Adt124>::nil())?, List::<Adt124>::nil()))
}

pub fn f504() -> Str {
    Str::lit("2026-07-28")
}

pub fn f505(v0: Adt7) -> R<Option<i64>> {
    let m1442 = f580(40u64, match v0.clone() {
        Adt7::C0(h1440, _) => {
            h1440
        }
    }, match v0 {
        Adt7::C0(_, h1441) => {
            h1441
        }
    })?;
    match m1442 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f581(v1))
        }
    }
}

pub fn f506(v0: Adt102, v1: Adt5) -> Result<Adt117, Adt107> {
    let m1443 = f264(Str::lit("method"), v1.clone());
    match m1443 {
        Adt116::C0 => {
            f336(Some(v0), f419(v1))
        }
        Adt116::C1(v2) => {
            let m1444 = f190(v2);
            match m1444 {
                None => {
                    f336(Some(v0), Adt108::C4)
                }
                Some(v3) => {
                    f582(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f507(v0: u64) -> R<u64> {
    let a1447 = {
        let a1445 = v0;
        let a1446 = 3u64;
        nat_mul(a1445, a1446)?
    };
    let a1448 = 1u64;
    nat_add(a1447, a1448)
}

pub fn f508(v0: u64, v1: u64, v2: u64) -> bool {
    let a1449 = v1;
    let a1450 = v0;
    if nat_le(a1449, a1450) {
        let a1451 = v0;
        let a1452 = v2;
        nat_le(a1451, a1452)
    } else {
        false
    }
}

pub fn f509(v0: Adt131) -> R<Adt151> {
    if f583(match v0.clone() {
        Adt131::C0(_, h1453, _, _, _, _, _) => {
            h1453
        }
    }) {
        let v1: i64 = {
            let a1459 = if match v0.clone() {
                Adt131::C0(_, _, _, _, _, h1454, _) => {
                    h1454
                }
            } {
                let a1456 = match v0.clone() {
                    Adt131::C0(_, _, _, _, _, _, h1455) => {
                        h1455
                    }
                };
                int_neg(a1456)?
            } else {
                match v0.clone() {
                    Adt131::C0(_, _, _, _, _, _, h1457) => {
                        h1457
                    }
                }
            };
            let a1460 = match v0.clone() {
                Adt131::C0(_, _, _, _, h1458, _, _) => {
                    h1458
                }
            };
            int_sub(a1459, a1460)?
        };
        let m1463 = {
            let a1461 = v1;
            let a1462 = -9999i64;
            compare(a1461, a1462)
        };
        if if match m1463 {
            Ordering::Less => {
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
            let m1466 = {
                let a1464 = 9999i64;
                let a1465 = v1;
                compare(a1464, a1465)
            };
            match m1466 {
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
                Adt131::C0(h1467, _, _, _, _, _, _) => {
                    h1467
                }
            } {
                let a1469 = match v0 {
                    Adt131::C0(_, _, h1468, _, _, _, _) => {
                        h1468
                    }
                };
                int_neg(a1469)?
            } else {
                match v0 {
                    Adt131::C0(_, _, h1470, _, _, _, _) => {
                        h1470
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt151::C0)
    }
}

pub fn f510(v0: Adt6, v1: List<Adt133>, v2: u64) -> Adt128 {
    let m1471 = v1.clone();
    match m1471.uncons() {
        None => {
            Adt128::C0(Adt129::C9(v0), List::<Adt133>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1472 = v3;
            match m1472 {
                Adt133::C0(v5) => {
                    Adt128::C0(Adt129::C2, List::cons(Adt133::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt133::C1(v6, v7) => {
                    let m1473 = v7;
                    match m1473 {
                        None => {
                            Adt128::C0(Adt129::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt128::C0(Adt129::C2, List::cons(Adt133::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f511(v0: u64) -> bool {
    let a1474 = v0;
    let a1475 = 32u64;
    if nat_eq(a1474, a1475) {
        true
    } else {
        let a1476 = v0;
        let a1477 = 10u64;
        if nat_eq(a1476, a1477) {
            true
        } else {
            let a1478 = v0;
            let a1479 = 13u64;
            if nat_eq(a1478, a1479) {
                true
            } else {
                let a1480 = v0;
                let a1481 = 9u64;
                nat_eq(a1480, a1481)
            }
        }
    }
}

pub fn f512(v0: Adt128, v1: u64, v2: u64) -> R<Adt146> {
    let a1482 = v2;
    let a1483 = 34u64;
    if nat_eq(a1482, a1483) {
        Ok(f516(v0, Adt129::C6(Adt130::C0, List::<u64>::nil(), false)))
    } else {
        let a1484 = v2;
        let a1485 = 123u64;
        if nat_eq(a1484, a1485) {
            f584(v0, Adt133::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt129::C3, v1)
        } else {
            let a1486 = v2;
            let a1487 = 91u64;
            if nat_eq(a1486, a1487) {
                f584(v0, Adt133::C0(List::<Adt6>::nil()), Adt129::C1, v1)
            } else {
                let a1488 = v2;
                let a1489 = 116u64;
                if nat_eq(a1488, a1489) {
                    Ok(f516(v0, Adt129::C7(0u64, 1u64)))
                } else {
                    let a1490 = v2;
                    let a1491 = 102u64;
                    if nat_eq(a1490, a1491) {
                        Ok(f516(v0, Adt129::C7(1u64, 1u64)))
                    } else {
                        let a1492 = v2;
                        let a1493 = 110u64;
                        if nat_eq(a1492, a1493) {
                            Ok(f516(v0, Adt129::C7(2u64, 1u64)))
                        } else {
                            let a1494 = v2;
                            let a1495 = 45u64;
                            if nat_eq(a1494, a1495) {
                                Ok(f516(v0, f585(true, Adt132::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1496 = v2;
                                let a1497 = 48u64;
                                if nat_eq(a1496, a1497) {
                                    Ok(f516(v0, f585(false, Adt132::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f508(v2, 49u64, 57u64) {
                                        Ok(f516(v0, f585(false, Adt132::C2, f586({
                                            let a1498 = v2;
                                            let a1499 = 48u64;
                                            nat_sub(a1498, a1499)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f517(Adt106::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f513(v0: Adt128, v1: u64) -> Adt146 {
    let m1501 = match v0.clone() {
        Adt128::C0(_, h1500, _) => {
            h1500
        }
    };
    match m1501.uncons() {
        None => {
            f517(Adt106::C2(v1))
        }
        Some((v2, v3)) => {
            let m1502 = v2;
            match m1502 {
                Adt133::C0(v4) => {
                    Adt146::C1(f510(Adt6::C4(f461(v4, List::<Adt6>::nil())), v3, {
                        let a1504 = match v0 {
                            Adt128::C0(_, _, h1503) => {
                                h1503
                            }
                        };
                        let a1505 = 1u64;
                        nat_sub(a1504, a1505)
                    }))
                }
                Adt133::C1(_, _) => {
                    f517(Adt106::C2(v1))
                }
            }
        }
    }
}

pub fn f514(v0: Adt128, v1: u64, v2: u64) -> Adt146 {
    let m1507 = match v0.clone() {
        Adt128::C0(h1506, _, _) => {
            h1506
        }
    };
    match m1507 {
        Adt129::C0 => {
            f517(Adt106::C2(v1))
        }
        Adt129::C1 => {
            f517(Adt106::C2(v1))
        }
        Adt129::C2 => {
            if f511(v2) {
                Adt146::C1(v0)
            } else {
                let a1508 = v2;
                let a1509 = 44u64;
                if nat_eq(a1508, a1509) {
                    f587(v0, v1)
                } else {
                    let a1510 = v2;
                    let a1511 = 93u64;
                    if nat_eq(a1510, a1511) {
                        f513(v0, v1)
                    } else {
                        let a1512 = v2;
                        let a1513 = 125u64;
                        if nat_eq(a1512, a1513) {
                            f515(v0, v1)
                        } else {
                            f517(Adt106::C2(v1))
                        }
                    }
                }
            }
        }
        Adt129::C3 => {
            f517(Adt106::C2(v1))
        }
        Adt129::C4 => {
            f517(Adt106::C2(v1))
        }
        Adt129::C5 => {
            f517(Adt106::C2(v1))
        }
        Adt129::C6(_, _, _) => {
            f517(Adt106::C2(v1))
        }
        Adt129::C7(_, _) => {
            f517(Adt106::C2(v1))
        }
        Adt129::C8(_) => {
            f517(Adt106::C2(v1))
        }
        Adt129::C9(_) => {
            if f511(v2) {
                Adt146::C1(v0)
            } else {
                f517(Adt106::C10(v1))
            }
        }
    }
}

pub fn f515(v0: Adt128, v1: u64) -> Adt146 {
    let m1515 = match v0.clone() {
        Adt128::C0(_, h1514, _) => {
            h1514
        }
    };
    match m1515.uncons() {
        None => {
            f517(Adt106::C2(v1))
        }
        Some((v2, v3)) => {
            let m1516 = v2;
            match m1516 {
                Adt133::C0(_) => {
                    f517(Adt106::C2(v1))
                }
                Adt133::C1(v5, _) => {
                    Adt146::C1(f510(Adt6::C5(f537(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1518 = match v0 {
                            Adt128::C0(_, _, h1517) => {
                                h1517
                            }
                        };
                        let a1519 = 1u64;
                        nat_sub(a1518, a1519)
                    }))
                }
            }
        }
    }
}

pub fn f516(v0: Adt128, v1: Adt129) -> Adt146 {
    Adt146::C1(f588(v0, v1))
}

pub fn f517(v0: Adt106) -> Adt146 {
    Adt146::C0(v0)
}

pub fn f518(v0: Adt128, v1: u64, v2: u64, v3: Adt130, v4: List<u64>, v5: bool) -> R<Adt146> {
    let m1520 = v3;
    match m1520 {
        Adt130::C0 => {
            let a1521 = v2;
            let a1522 = 34u64;
            if nat_eq(a1521, a1522) {
                Ok(f589(v0, v1, v4, v5))
            } else {
                let a1523 = v2;
                let a1524 = 92u64;
                if nat_eq(a1523, a1524) {
                    Ok(f516(v0, Adt129::C6(Adt130::C1, v4, v5)))
                } else {
                    let a1525 = v2;
                    let a1526 = 32u64;
                    if nat_lt(a1525, a1526) {
                        Ok(f517(Adt106::C4(v1)))
                    } else {
                        Ok(f516(v0, Adt129::C6(Adt130::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt130::C1 => {
            let a1527 = v2;
            let a1528 = 117u64;
            if nat_eq(a1527, a1528) {
                Ok(f516(v0, Adt129::C6(Adt130::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1529 = f590(v2);
                let a1530 = 0u64;
                if nat_eq(a1529, a1530) {
                    Ok(f517(Adt106::C5(v1)))
                } else {
                    Ok(f516(v0, Adt129::C6(Adt130::C0, List::cons(f590(v2), v4), v5)))
                }
            }
        }
        Adt130::C2(v6, v7, v8) => {
            let a1531 = 15u64;
            let a1532 = f591(v2);
            if nat_lt(a1531, a1532) {
                Ok(f517(Adt106::C5(v1)))
            } else {
                let a1535 = {
                    let a1533 = v6;
                    let a1534 = 1u64;
                    nat_add(a1533, a1534)?
                };
                let a1536 = 4u64;
                if nat_eq(a1535, a1536) {
                    f592(v0, v1, {
                        let a1539 = {
                            let a1537 = v7;
                            let a1538 = 16u64;
                            nat_mul(a1537, a1538)?
                        };
                        let a1540 = f591(v2);
                        nat_add(a1539, a1540)?
                    }, v8, v4, v5)
                } else {
                    Ok(f516(v0, Adt129::C6(Adt130::C2({
                        let a1541 = v6;
                        let a1542 = 1u64;
                        nat_add(a1541, a1542)?
                    }, {
                        let a1545 = {
                            let a1543 = v7;
                            let a1544 = 16u64;
                            nat_mul(a1543, a1544)?
                        };
                        let a1546 = f591(v2);
                        nat_add(a1545, a1546)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt130::C3(v9) => {
            let a1547 = v2;
            let a1548 = 92u64;
            if nat_eq(a1547, a1548) {
                Ok(f516(v0, Adt129::C6(Adt130::C4(v9), v4, v5)))
            } else {
                Ok(f517(Adt106::C6(v1)))
            }
        }
        Adt130::C4(v10) => {
            let a1549 = v2;
            let a1550 = 117u64;
            if nat_eq(a1549, a1550) {
                Ok(f516(v0, Adt129::C6(Adt130::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f517(Adt106::C6(v1)))
            }
        }
    }
}

pub fn f519(v0: Adt128, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt146> {
    let a1551 = v2;
    let a1552 = f593(v3, v4);
    if nat_eq(a1551, a1552) {
        let a1556 = {
            let a1553 = v4;
            let a1554 = 1u64;
            nat_add(a1553, a1554)?
        };
        let a1557 = {
            let a1555 = f594(v3);
            length_list(a1555)
        };
        if nat_eq(a1556, a1557) {
            Ok(f595(v0, f596(v3)))
        } else {
            Ok(f516(v0, Adt129::C7(v3, {
                let a1558 = v4;
                let a1559 = 1u64;
                nat_add(a1558, a1559)?
            })))
        }
    } else {
        Ok(f517(Adt106::C2(v1)))
    }
}

pub fn f520(v0: Adt128, v1: u64, v2: u64, v3: Adt131) -> R<Adt146> {
    let m1561 = match v3.clone() {
        Adt131::C0(_, h1560, _, _, _, _, _) => {
            h1560
        }
    };
    match m1561 {
        Adt132::C0 => {
            let a1562 = v2;
            let a1563 = 48u64;
            if nat_eq(a1562, a1563) {
                Ok(f597(v0, v3, Adt132::C1))
            } else {
                if f508(v2, 49u64, 57u64) {
                    f598(v0, v1, v2, v3, false)
                } else {
                    Ok(f517(Adt106::C7(v1)))
                }
            }
        }
        Adt132::C1 => {
            let a1564 = v2;
            let a1565 = 46u64;
            if nat_eq(a1564, a1565) {
                Ok(f597(v0, v3, Adt132::C3))
            } else {
                if f599(v2) {
                    Ok(f597(v0, v3, Adt132::C5))
                } else {
                    if f600(v2) {
                        Ok(f517(Adt106::C7(v1)))
                    } else {
                        f601(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt132::C2 => {
            if f600(v2) {
                f598(v0, v1, v2, v3, false)
            } else {
                let a1566 = v2;
                let a1567 = 46u64;
                if nat_eq(a1566, a1567) {
                    Ok(f597(v0, v3, Adt132::C3))
                } else {
                    if f599(v2) {
                        Ok(f597(v0, v3, Adt132::C5))
                    } else {
                        f601(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt132::C3 => {
            if f600(v2) {
                f598(v0, v1, v2, v3, true)
            } else {
                Ok(f517(Adt106::C7(v1)))
            }
        }
        Adt132::C4 => {
            if f600(v2) {
                f598(v0, v1, v2, v3, true)
            } else {
                if f599(v2) {
                    Ok(f597(v0, v3, Adt132::C5))
                } else {
                    f601(v0, v1, v2, v3)
                }
            }
        }
        Adt132::C5 => {
            let a1568 = v2;
            let a1569 = 43u64;
            if nat_eq(a1568, a1569) {
                Ok(f516(v0, f585(match v3.clone() {
                    Adt131::C0(h1570, _, _, _, _, _, _) => {
                        h1570
                    }
                }, Adt132::C6, match v3.clone() {
                    Adt131::C0(_, _, h1571, _, _, _, _) => {
                        h1571
                    }
                }, match v3.clone() {
                    Adt131::C0(_, _, _, h1572, _, _, _) => {
                        h1572
                    }
                }, match v3 {
                    Adt131::C0(_, _, _, _, h1573, _, _) => {
                        h1573
                    }
                }, false, 0i64)))
            } else {
                let a1574 = v2;
                let a1575 = 45u64;
                if nat_eq(a1574, a1575) {
                    Ok(f516(v0, f585(match v3.clone() {
                        Adt131::C0(h1576, _, _, _, _, _, _) => {
                            h1576
                        }
                    }, Adt132::C6, match v3.clone() {
                        Adt131::C0(_, _, h1577, _, _, _, _) => {
                            h1577
                        }
                    }, match v3.clone() {
                        Adt131::C0(_, _, _, h1578, _, _, _) => {
                            h1578
                        }
                    }, match v3 {
                        Adt131::C0(_, _, _, _, h1579, _, _) => {
                            h1579
                        }
                    }, true, 0i64)))
                } else {
                    if f600(v2) {
                        f602(v0, v1, v2, Adt131::C0(match v3.clone() {
                            Adt131::C0(h1580, _, _, _, _, _, _) => {
                                h1580
                            }
                        }, match v3.clone() {
                            Adt131::C0(_, h1581, _, _, _, _, _) => {
                                h1581
                            }
                        }, match v3.clone() {
                            Adt131::C0(_, _, h1582, _, _, _, _) => {
                                h1582
                            }
                        }, match v3.clone() {
                            Adt131::C0(_, _, _, h1583, _, _, _) => {
                                h1583
                            }
                        }, match v3 {
                            Adt131::C0(_, _, _, _, h1584, _, _) => {
                                h1584
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f517(Adt106::C7(v1)))
                    }
                }
            }
        }
        Adt132::C6 => {
            if f600(v2) {
                f602(v0, v1, v2, v3.clone(), match v3 {
                    Adt131::C0(_, _, _, _, _, h1585, _) => {
                        h1585
                    }
                })
            } else {
                Ok(f517(Adt106::C7(v1)))
            }
        }
        Adt132::C7 => {
            if f600(v2) {
                f602(v0, v1, v2, v3.clone(), match v3 {
                    Adt131::C0(_, _, _, _, _, h1586, _) => {
                        h1586
                    }
                })
            } else {
                f601(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f521(v0: u64) -> bool {
    let a1587 = v0;
    let a1588 = 9u64;
    if nat_eq(a1587, a1588) {
        true
    } else {
        let a1589 = v0;
        let a1590 = 32u64;
        nat_eq(a1589, a1590)
    }
}

pub fn f522(v0: u64) -> bool {
    let a1591 = 47u64;
    let a1592 = v0;
    if nat_lt(a1591, a1592) {
        let a1593 = v0;
        let a1594 = 58u64;
        nat_lt(a1593, a1594)
    } else {
        false
    }
}

pub fn f523(v0: u64) -> bool {
    let a1595 = 48u64;
    let a1596 = v0;
    if if nat_le(a1595, a1596) {
        let a1597 = v0;
        let a1598 = 57u64;
        nat_le(a1597, a1598)
    } else {
        false
    } {
        true
    } else {
        let a1599 = 65u64;
        let a1600 = v0;
        if if nat_le(a1599, a1600) {
            let a1601 = v0;
            let a1602 = 90u64;
            nat_le(a1601, a1602)
        } else {
            false
        } {
            true
        } else {
            let a1603 = 97u64;
            let a1604 = v0;
            if if nat_le(a1603, a1604) {
                let a1605 = v0;
                let a1606 = 122u64;
                nat_le(a1605, a1606)
            } else {
                false
            } {
                true
            } else {
                let a1607 = v0;
                let a1608 = 33u64;
                if nat_eq(a1607, a1608) {
                    true
                } else {
                    let a1609 = v0;
                    let a1610 = 35u64;
                    if nat_eq(a1609, a1610) {
                        true
                    } else {
                        let a1611 = v0;
                        let a1612 = 36u64;
                        if nat_eq(a1611, a1612) {
                            true
                        } else {
                            let a1613 = v0;
                            let a1614 = 37u64;
                            if nat_eq(a1613, a1614) {
                                true
                            } else {
                                let a1615 = v0;
                                let a1616 = 38u64;
                                if nat_eq(a1615, a1616) {
                                    true
                                } else {
                                    let a1617 = v0;
                                    let a1618 = 39u64;
                                    if nat_eq(a1617, a1618) {
                                        true
                                    } else {
                                        let a1619 = v0;
                                        let a1620 = 42u64;
                                        if nat_eq(a1619, a1620) {
                                            true
                                        } else {
                                            let a1621 = v0;
                                            let a1622 = 43u64;
                                            if nat_eq(a1621, a1622) {
                                                true
                                            } else {
                                                let a1623 = v0;
                                                let a1624 = 45u64;
                                                if nat_eq(a1623, a1624) {
                                                    true
                                                } else {
                                                    let a1625 = v0;
                                                    let a1626 = 46u64;
                                                    if nat_eq(a1625, a1626) {
                                                        true
                                                    } else {
                                                        let a1627 = v0;
                                                        let a1628 = 94u64;
                                                        if nat_eq(a1627, a1628) {
                                                            true
                                                        } else {
                                                            let a1629 = v0;
                                                            let a1630 = 95u64;
                                                            if nat_eq(a1629, a1630) {
                                                                true
                                                            } else {
                                                                let a1631 = v0;
                                                                let a1632 = 96u64;
                                                                if nat_eq(a1631, a1632) {
                                                                    true
                                                                } else {
                                                                    let a1633 = v0;
                                                                    let a1634 = 124u64;
                                                                    if nat_eq(a1633, a1634) {
                                                                        true
                                                                    } else {
                                                                        let a1635 = v0;
                                                                        let a1636 = 126u64;
                                                                        nat_eq(a1635, a1636)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f524(v0: u64) -> bool {
    let a1637 = 48u64;
    let a1638 = v0;
    if if nat_le(a1637, a1638) {
        let a1639 = v0;
        let a1640 = 57u64;
        nat_le(a1639, a1640)
    } else {
        false
    } {
        true
    } else {
        let a1641 = 65u64;
        let a1642 = v0;
        if if nat_le(a1641, a1642) {
            let a1643 = v0;
            let a1644 = 90u64;
            nat_le(a1643, a1644)
        } else {
            false
        } {
            true
        } else {
            let a1645 = 97u64;
            let a1646 = v0;
            if if nat_le(a1645, a1646) {
                let a1647 = v0;
                let a1648 = 122u64;
                nat_le(a1647, a1648)
            } else {
                false
            } {
                true
            } else {
                let a1649 = v0;
                let a1650 = 95u64;
                if nat_eq(a1649, a1650) {
                    true
                } else {
                    let a1651 = v0;
                    let a1652 = 45u64;
                    if nat_eq(a1651, a1652) {
                        true
                    } else {
                        let a1653 = v0;
                        let a1654 = 46u64;
                        nat_eq(a1653, a1654)
                    }
                }
            }
        }
    }
}

pub fn f525() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f526(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt134, v6: List<Adt118>) -> R<Result<(List<Adt118>, Adt134), Adt98>> {
    let m1655 = f603(v0.clone(), v1, v2.clone());
    match m1655 {
        None => {
            let m1656 = f604(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1656 {
                Adt157::C0(v8) => {
                    Ok(Err::<(List<Adt118>, Adt134), Adt98>(v8))
                }
                Adt157::C1(v9) => {
                    Ok(Ok::<(List<Adt118>, Adt134), Adt98>((f605(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt118>, Adt134), Adt98>(v7))
        }
    }
}

pub fn f527(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt135> {
    Ok(f606(List::cons((Str::lit("type"), f607(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f608(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f609(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f610(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f610(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f610(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f610(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1657 = f611(v0.clone(), v1.clone())?;
        let a1658 = List::cons((Str::lit("required"), f612(v0, v1)), List::<(Str, Adt152)>::nil());
        append_list(a1657, a1658)
    })))))))))
}

pub fn f528(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt112>) -> R<Adt135> {
    Ok(f613(f614(v0, v1.clone(), v2.clone(), v3.clone()), f613(f615(v1.clone(), v2.clone(), v3.clone()), f613(f616(v1.clone(), v2.clone(), v3.clone()), f613(f617(v1.clone(), v2.clone(), v3.clone())?, f613(f618(v1.clone(), v2.clone(), v3.clone()), f613(f619(Adt115::C0, f439(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f613(f619(Adt115::C1, f440(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f613(f619(Adt115::C2, f441(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f619(Adt115::C3, f442(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f529(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1659 = f364(v0, v1);
    match m1659 {
        Adt116::C0 => {
            None::<Adt6>
        }
        Adt116::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f530(v0: Str) -> R<Str> {
    let m1662 = {
        let a1661 = f470(f232(), f406(f620(f229({
            let a1660 = v0.clone();
            utf8_encode(a1660)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1661)
    };
    match m1662 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f531(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1664 = f621({
        let a1663 = v0.clone();
        length_list(a1663)
    }, v0, List::<u64>::nil())?;
    match m1664 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f406(v1, List::<u64>::nil())))
        }
    }
}

pub fn f532(v0: List<Adt51>) -> R<bool> {
    let m1665 = v0;
    match m1665.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f622(v2, match v1 {
                Adt51::C0(h1666, _, _) => {
                    h1666
                }
            })
        }
    }
}

pub fn f533(v0: Adt116, v1: Str) -> bool {
    let m1667 = v0;
    match m1667 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v2) => {
            let m1668 = f190(v2);
            match m1668 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1669 = v3;
                    let a1670 = v1;
                    equal(a1669, a1670)
                }
            }
        }
    }
}

pub fn f534() -> i64 {
    60000i64
}

pub fn f535() -> i64 {
    1000i64
}

pub fn f536(v0: Str) -> R<Str> {
    let m1673 = {
        let a1672 = f470(f232(), f406(f623(f229({
            let a1671 = v0;
            utf8_encode(a1671)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1672)
    };
    match m1673 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f537(p1676: List<(Str, Adt6)>, p1677: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1675 = (p1676, p1677);
    {
        let mut m1675 = m1675;
        loop {
            let step = {
                let (v0, v1) = m1675;
                let m1674 = v0;
                match m1674.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1675 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: Adt39) -> R<Option<Adt6>> {
    let m1678 = v0;
    match m1678 {
        Adt39::C0(v1) => {
            Ok(Some(f624(v1)))
        }
        Adt39::C1(v2) => {
            Ok(Some(f625(Str::lit("image"), match v2.clone() {
                Adt43::C0(h1679, _, _, _) => {
                    h1679
                }
            }, match v2.clone() {
                Adt43::C0(_, h1680, _, _) => {
                    h1680
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h1681, _) => {
                    h1681
                }
            }, match v2 {
                Adt43::C0(_, _, _, h1682) => {
                    h1682
                }
            })))
        }
        Adt39::C2(v3) => {
            Ok(Some(f625(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h1683, _, _, _) => {
                    h1683
                }
            }, match v3.clone() {
                Adt44::C0(_, h1684, _, _) => {
                    h1684
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h1685, _) => {
                    h1685
                }
            }, match v3 {
                Adt44::C0(_, _, _, h1686) => {
                    h1686
                }
            })))
        }
        Adt39::C3(v4) => {
            Ok(Some(f626(v4)?))
        }
        Adt39::C4(v5) => {
            Ok(Some(f627(v5)))
        }
    }
}

pub fn f539(v0: Adt5) -> Option<Adt64> {
    let m1687 = f540(Str::lit("taskId"), v0.clone());
    match m1687 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m1688 = f540(Str::lit("createdAt"), v0.clone());
            match m1688 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let m1689 = f540(Str::lit("lastUpdatedAt"), v0.clone());
                    match m1689 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v3) => {
                            Some(Adt64::C0(v1, f540(Str::lit("statusMessage"), v0.clone()), v2, v3, f465(Str::lit("ttlMs"), v0.clone()), f465(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f540(v0: Str, v1: Adt5) -> Option<Str> {
    let m1690 = f264(v0, v1);
    match m1690 {
        Adt116::C0 => {
            None::<Str>
        }
        Adt116::C1(v2) => {
            f190(v2)
        }
    }
}

pub fn f541(v0: Str, v1: Adt5) -> Option<List<(Str, Adt65)>> {
    let m1691 = f463(v0, v1);
    match m1691 {
        None => {
            None::<List<(Str, Adt65)>>
        }
        Some(v2) => {
            f628(match v2 {
                Adt5::C0(h1692) => {
                    h1692
                }
            }, List::<(Str, Adt65)>::nil())
        }
    }
}

pub fn f542(v0: Adt5) -> Option<Adt81> {
    let m1693 = f465(Str::lit("code"), v0.clone());
    match m1693 {
        None => {
            None::<Adt81>
        }
        Some(v1) => {
            let m1694 = f540(Str::lit("message"), v0.clone());
            match m1694 {
                None => {
                    None::<Adt81>
                }
                Some(v2) => {
                    Some(Adt81::C0(v1, v2, {
                        let m1695 = f264(Str::lit("data"), v0);
                        match m1695 {
                            Adt116::C0 => {
                                None::<Adt6>
                            }
                            Adt116::C1(v3) => {
                                Some(v3)
                            }
                        }
                    }))
                }
            }
        }
    }
}

pub fn f543(v0: Adt5) -> Option<Adt87> {
    let m1696 = f463(Str::lit("completes"), v0.clone());
    match m1696 {
        None => {
            let m1697 = f463(Str::lit("fails"), v0);
            match m1697 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m1698 = f542(v2);
                    match m1698 {
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

pub fn f544(p1704: List<u64>, p1705: List<u64>) -> Option<List<u64>> {
    let m1703 = (p1704, p1705);
    {
        let mut m1703 = m1703;
        loop {
            let step = {
                let (v0, v1) = m1703;
                let m1699 = v0;
                match m1699.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1700 = v1;
                        match m1700.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1701 = v2;
                                let a1702 = v4;
                                if nat_eq(a1701, a1702) {
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
                Ok(next) => { m1703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f545(v0: Str) -> R<List<u64>> {
    f229({
        let a1706 = v0;
        utf8_encode(a1706)
    })
}

pub fn f546() -> Str {
    Str::lit("test://template/")
}

pub fn f547(v0: List<u64>) -> List<u64> {
    f406(v0, List::<u64>::nil())
}

pub fn f548() -> Str {
    Str::lit("data")
}

pub fn f549(v0: List<u64>) -> bool {
    let m1707 = v0;
    match m1707.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f550(p1712: List<u64>) -> bool {
    let m1711 = p1712;
    {
        let mut m1711 = m1711;
        loop {
            let step = {
                let v0 = m1711;
                let m1708 = v0;
                match m1708.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1709 = v1;
                        let a1710 = 47u64;
                        if nat_eq(a1709, a1710) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551(p1717: Bytes, p1718: List<Bytes>) -> Bytes {
    let m1716 = (p1717, p1718);
    {
        let mut m1716 = m1716;
        loop {
            let step = {
                let (v0, v1) = m1716;
                let m1713 = v1;
                match m1713.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1714 = v2;
                            let a1715 = v0;
                            append_bytes(a1714, a1715)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552(p1721: List<Bytes>, p1722: List<u64>) -> List<Bytes> {
    let m1720 = (p1721, p1722);
    {
        let mut m1720 = m1720;
        loop {
            let step = {
                let (v0, v1) = m1720;
                let m1719 = v1;
                match m1719.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f629(f630(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(p1728: List<u64>, p1729: List<u64>) -> bool {
    let m1727 = (p1728, p1729);
    {
        let mut m1727 = m1727;
        loop {
            let step = {
                let (v0, v1) = m1727;
                let m1723 = v0;
                match m1723.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1724 = v1;
                        match m1724.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1725 = v2;
                                let a1726 = v4;
                                if nat_eq(a1725, a1726) {
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
                Ok(next) => { m1727 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: i64, v1: u64) -> R<i64> {
    f368({
        let a1730 = v0;
        let a1731 = 256i64;
        int_mul(a1730, a1731)?
    }, f269(v1, 0i64)?)
}

pub fn f555(v0: List<u64>) -> Option<Str> {
    let a1732 = f470(f232(), v0);
    utf8_decode(a1732)
}

pub fn f556(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt14, u64), Adt94>> {
    let m1734 = f16(v0, {
        let (_, h1733) = v3.clone();
        h1733
    })?;
    match m1734 {
        None => {
            Ok(Err::<(Adt14, u64), Adt94>(Adt94::C0({
                let (_, h1735) = v3;
                h1735
            })))
        }
        Some(v4) => {
            Ok(f631(v1, v2, {
                let (h1736, _) = v3;
                h1736
            }, v4))
        }
    }
}

pub fn f557(v0: u64, v1: Adt6, v2: List<Adt120>, v3: List<u64>) -> R<Adt148> {
    let m1737 = v1;
    match m1737 {
        Adt6::C0 => {
            Ok(Adt148::C1(v2, f406(f632(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt148::C1(v2, f406(if v4 {
                f632(0u64)
            } else {
                f632(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f633(v5.clone())? {
                Ok(Adt148::C1(v2, f406(f634(v5)?, v3)))
            } else {
                Ok(Adt148::C0(Adt106::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt148::C1(v2, f406(f558(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1738 = v0;
            let a1739 = f635();
            if nat_lt(a1738, a1739) {
                Ok(Adt148::C1(f636(91u64, Adt120::C2({
                    let a1740 = v0;
                    let a1741 = 1u64;
                    nat_add(a1740, a1741)?
                }, v7), v2), v3))
            } else {
                Ok(Adt148::C0(Adt106::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1742 = v0;
            let a1743 = f635();
            if nat_lt(a1742, a1743) {
                Ok(Adt148::C1(f636(123u64, Adt120::C4({
                    let a1744 = v0;
                    let a1745 = 1u64;
                    nat_add(a1744, a1745)?
                }, v8), v2), v3))
            } else {
                Ok(Adt148::C0(Adt106::C12))
            }
        }
    }
}

pub fn f558(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1747 = f406(f637(f229({
            let a1746 = v0;
            utf8_encode(a1746)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1748 = List::cons(34u64, List::<u64>::nil());
        append_list(a1747, a1748)
    }))
}

pub fn f559(v0: Adt33) -> R<Option<List<(Str, Adt6)>>> {
    let m1749 = v0;
    match m1749 {
        Adt33::C0(v1) => {
            Ok(Some(f638(v1)?))
        }
        Adt33::C1(v2) => {
            Ok(Some(f639(v2)?))
        }
        Adt33::C2(v3) => {
            f284(f285(v3)?)
        }
        Adt33::C3(v4) => {
            f284(f285(match v4 {
                Adt50::C0(h1750, _) => {
                    h1750
                }
            })?)
        }
        Adt33::C4(v5) => {
            Ok(Some(f640(v5)?))
        }
        Adt33::C5(v6) => {
            Ok(Some(f641(v6)?))
        }
        Adt33::C6(v7) => {
            Ok(Some(f642(v7)?))
        }
        Adt33::C7(v8) => {
            Ok(Some(f643(v8)?))
        }
        Adt33::C8(v9) => {
            f284(f644(v9)?)
        }
        Adt33::C9(v10) => {
            Ok(Some(f645(v10)?))
        }
        Adt33::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt33::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt33::C12(v12) => {
            f284(f646(v12)?)
        }
        Adt33::C13(v13) => {
            f284(f647({
                let m1753 = match v13 {
                    Adt83::C0(_, _, h1752) => {
                        h1752
                    }
                };
                match m1753 {
                    Adt85::C0(h1751, _, _) => {
                        h1751
                    }
                }
            }))
        }
        Adt33::C14(v14) => {
            Ok(f648(v14))
        }
    }
}

pub fn f560(v0: Adt139) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f649(f650(match v0.clone() {
        Adt139::C0(h1754, _, _) => {
            h1754
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f651({
        let m1757 = match v0 {
            Adt139::C0(_, h1756, _) => {
                h1756
            }
        };
        match m1757 {
            Adt140::C0(h1755, _) => {
                h1755
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f561(v0: Option<Option<List<u64>>>, v1: Adt95) -> Adt95 {
    let m1758 = v0;
    match m1758 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt95::C0(match v1.clone() {
                Adt95::C0(h1759, _, _, _, _) => {
                    h1759
                }
            }, match v1.clone() {
                Adt95::C0(_, h1760, _, _, _) => {
                    h1760
                }
            }, match v1.clone() {
                Adt95::C0(_, _, h1761, _, _) => {
                    h1761
                }
            }, match v1 {
                Adt95::C0(_, _, _, h1762, _) => {
                    h1762
                }
            }, v2)
        }
    }
}

pub fn f562(v0: Adt14, v1: Adt88) -> R<Option<Option<List<u64>>>> {
    let m1764 = f203(match v0.clone() {
        Adt14::C0(_, _, _, h1763) => {
            h1763
        }
    })?;
    match m1764 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1767 = f652(f653(match v1 {
                Adt88::C0(h1765, _, _) => {
                    h1765
                }
            }, v2, match v0 {
                Adt14::C0(h1766, _, _, _) => {
                    h1766
                }
            })?)?;
            match m1767 {
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

pub fn f563(v0: bool, v1: Adt95, v2: List<List<u64>>) -> Adt95 {
    if f654(v0, match v1.clone() {
        Adt95::C0(h1768, _, _, _, _) => {
            h1768
        }
    }, v2.clone()) {
        f655(match v1.clone() {
            Adt95::C0(h1769, _, _, _, _) => {
                h1769
            }
        }, {
            let a1771 = v2;
            let a1772 = List::cons(match v1 {
                Adt95::C0(_, _, _, h1770, _) => {
                    h1770
                }
            }, List::<List<u64>>::nil());
            append_list(a1771, a1772)
        })
    } else {
        v1
    }
}

pub fn f564(v0: Adt141) -> R<Option<List<u64>>> {
    let m1773 = f375(f656(v0))?;
    match m1773 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1774 = f233(v1)?;
            match m1774 {
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

pub fn f565(v0: Adt88) -> R<Option<List<List<u64>>>> {
    if f657(v0.clone()) {
        let m1778 = f658({
            let m1777 = match v0 {
                Adt88::C0(h1776, _, _) => {
                    h1776
                }
            };
            match m1777 {
                Adt85::C0(h1775, _, _) => {
                    h1775
                }
            }
        });
        match m1778 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1779 = f375(v1)?;
                match m1779 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1780 = f233(v2)?;
                        match m1780 {
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

pub fn f566() -> u64 {
    404u64
}

pub fn f567(v0: Adt90) -> u64 {
    let m1781 = v0;
    match m1781 {
        Adt90::C0(_) => {
            f240()
        }
        Adt90::C1(_) => {
            f240()
        }
        Adt90::C2(_) => {
            f240()
        }
        Adt90::C3(_) => {
            f240()
        }
        Adt90::C4(_) => {
            f240()
        }
        Adt90::C5(_) => {
            f240()
        }
        Adt90::C6(_) => {
            f22()
        }
        Adt90::C7(_) => {
            f659()
        }
        Adt90::C8(_) => {
            f240()
        }
        Adt90::C9(_) => {
            f22()
        }
        Adt90::C10 => {
            f240()
        }
        Adt90::C11 => {
            f240()
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

pub fn f568(v0: Adt142) -> i64 {
    let m1782 = v0;
    match m1782 {
        Adt142::C0(_) => {
            f660()
        }
        Adt142::C1(_, _) => {
            f661()
        }
        Adt142::C2(_) => {
            f662()
        }
        Adt142::C3(_) => {
            f663()
        }
        Adt142::C4(v6) => {
            f664(v6)
        }
    }
}

pub fn f569(v0: Adt142) -> Str {
    let m1783 = v0;
    match m1783 {
        Adt142::C0(v1) => {
            f665(v1)
        }
        Adt142::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt142::C2(v4) => {
            f666(v4)
        }
        Adt142::C3(_) => {
            Str::lit("Method not found")
        }
        Adt142::C4(v6) => {
            let m1784 = v6;
            match m1784 {
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

pub fn f570(v0: Adt142) -> R<Option<Adt6>> {
    let m1785 = v0;
    match m1785 {
        Adt142::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt142::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f667(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt142::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt142::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt142::C4(v6) => {
            let m1786 = v6;
            match m1786 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f269(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f668(v15)), List::<(Str, Adt6)>::nil()))))
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

pub fn f571(v0: List<u64>) -> R<bool> {
    let m1787 = f492(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1787.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f669(v2)? {
                Ok(false)
            } else {
                f340(f342(v1), f670())
            }
        }
    }
}

pub fn f572(v0: Adt0, v1: Fn5, v2: u64, v3: Adt149) -> R<Result<Adt139, Adt142>> {
    f671(v0.clone(), v1, v2, v3.clone(), f672(match v0 {
        Adt0::C0(_, _, h1788, _, _, _, _) => {
            h1788
        }
    }, match v3 {
        Adt149::C0(_, h1789, _, _) => {
            h1789
        }
    }))
}

pub fn f573(v0: Adt153, v1: Adt154) -> Result<Adt149, Adt142> {
    let m1790 = v1;
    match m1790 {
        Adt154::C0(v3) => {
            f673(v3, v0)
        }
        Adt154::C1(v2) => {
            Err::<Adt149, Adt142>(v2)
        }
    }
}

pub fn f574(v0: Str) -> Adt153 {
    let a1791 = v0.clone();
    let a1792 = Str::lit("server/discover");
    if equal(a1791, a1792) {
        Adt153::C1(Adt150::C0)
    } else {
        let a1793 = v0.clone();
        let a1794 = Str::lit("tools/list");
        if equal(a1793, a1794) {
            Adt153::C1(Adt150::C1)
        } else {
            let a1795 = v0.clone();
            let a1796 = Str::lit("tools/call");
            if equal(a1795, a1796) {
                Adt153::C1(Adt150::C2)
            } else {
                let a1797 = v0.clone();
                let a1798 = Str::lit("resources/list");
                if equal(a1797, a1798) {
                    Adt153::C1(Adt150::C3)
                } else {
                    let a1799 = v0.clone();
                    let a1800 = Str::lit("resources/read");
                    if equal(a1799, a1800) {
                        Adt153::C1(Adt150::C4)
                    } else {
                        let a1801 = v0.clone();
                        let a1802 = Str::lit("resources/templates/list");
                        if equal(a1801, a1802) {
                            Adt153::C1(Adt150::C5)
                        } else {
                            let a1803 = v0.clone();
                            let a1804 = Str::lit("prompts/list");
                            if equal(a1803, a1804) {
                                Adt153::C1(Adt150::C6)
                            } else {
                                let a1805 = v0.clone();
                                let a1806 = Str::lit("prompts/get");
                                if equal(a1805, a1806) {
                                    Adt153::C1(Adt150::C7)
                                } else {
                                    let a1807 = v0.clone();
                                    let a1808 = Str::lit("completion/complete");
                                    if equal(a1807, a1808) {
                                        Adt153::C1(Adt150::C8)
                                    } else {
                                        let a1809 = v0.clone();
                                        let a1810 = Str::lit("tasks/get");
                                        if equal(a1809, a1810) {
                                            Adt153::C1(Adt150::C9)
                                        } else {
                                            let a1811 = v0.clone();
                                            let a1812 = Str::lit("tasks/update");
                                            if equal(a1811, a1812) {
                                                Adt153::C1(Adt150::C10)
                                            } else {
                                                let a1813 = v0;
                                                let a1814 = Str::lit("tasks/cancel");
                                                if equal(a1813, a1814) {
                                                    Adt153::C1(Adt150::C11)
                                                } else {
                                                    Adt153::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f575(v0: Option<Adt121>, v1: List<Str>, v2: Adt117) -> R<Adt154> {
    f674(v0, v1, v2.clone(), f675(match v2 {
        Adt117::C0(_, _, h1815) => {
            h1815
        }
    }))
}

pub fn f576(v0: Str) -> Adt156 {
    let m1816 = f574(v0);
    match m1816 {
        Adt153::C0 => {
            Adt156::C0
        }
        Adt153::C1(v1) => {
            f676(v1)
        }
    }
}

pub fn f577(v0: List<u64>) -> Option<List<u64>> {
    let m1817 = f677(f678(), v0);
    match m1817 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1818 = f677(f679(), f350(v1, List::<u64>::nil()));
            match m1818 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f350(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f578(p1821: List<Adt124>, p1822: List<Adt124>) -> List<Adt124> {
    let m1820 = (p1821, p1822);
    {
        let mut m1820 = m1820;
        loop {
            let step = {
                let (v0, v1) = m1820;
                let m1819 = v0;
                match m1819.uncons() {
                    None => {
                        Err::<(List<Adt124>, List<Adt124>), List<Adt124>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt124>, List<Adt124>), List<Adt124>>((v3, List::cons(v2, v1)))
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

pub fn f579(v0: List<Adt93>, p1829: List<Adt12>, v2: Option<Str>, p1830: List<Adt124>) -> R<List<Adt124>> {
    let m1828 = (p1829, p1830);
    {
        let mut m1828 = m1828;
        loop {
            let step = {
                let (v1, v3) = m1828;
                let m1823 = v1;
                match m1823.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt124>), R<List<Adt124>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f680(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt124>), R<List<Adt124>>>((v5, List::cons(Adt124::C0(match v4.clone() {
                                Adt12::C0(_, h1824, _, _) => {
                                    h1824
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1825, _) => {
                                    h1825
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1826) => {
                                    h1826
                                }
                            }, f681(v0.clone(), match v4 {
                                Adt12::C0(_, h1827, _, _) => {
                                    h1827
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt124>), R<List<Adt124>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(p1846: u64, p1847: i64, p1848: i64) -> R<Option<i64>> {
    let m1845 = (p1846, (p1847, p1848));
    {
        let mut m1845 = m1845;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1845;
                let m1831 = v0;
                if m1831 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1831 - 1;
                    if f374(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1834 = {
                            let a1832 = v2;
                            let a1833 = 0i64;
                            compare(a1832, a1833)
                        };
                        if match m1834 {
                            Ordering::Less => {
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
                                let a1835 = v1;
                                let a1836 = 10i64;
                                let a1837 = 0i64;
                                int_rem(a1835, a1836, a1837)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1838 = v1;
                                    let a1839 = 10i64;
                                    let a1840 = 0i64;
                                    int_quot(a1838, a1839, a1840)?
                                }, f368(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1841 = v1;
                                let a1842 = 10i64;
                                int_mul(a1841, a1842)?
                            }, {
                                let a1843 = v2;
                                let a1844 = 1i64;
                                int_sub(a1843, a1844)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(v0: i64) -> Option<i64> {
    let m1851 = {
        let a1849 = 9007199254740991i64;
        let a1850 = v0;
        compare(a1849, a1850)
    };
    if if match m1851 {
        Ordering::Less => {
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
        let m1854 = {
            let a1852 = v0;
            let a1853 = -9007199254740991i64;
            compare(a1852, a1853)
        };
        match m1854 {
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

pub fn f582(v0: Adt102, v1: Str, v2: Adt5) -> Result<Adt117, Adt107> {
    let m1855 = f264(Str::lit("params"), v2);
    match m1855 {
        Adt116::C0 => {
            Ok::<Adt117, Adt107>(Adt117::C0(v0, v1, None::<Adt5>))
        }
        Adt116::C1(v3) => {
            let m1856 = f265(v3);
            match m1856 {
                None => {
                    f336(Some(v0), Adt108::C6)
                }
                Some(v4) => {
                    Ok::<Adt117, Adt107>(Adt117::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f583(v0: Adt132) -> bool {
    let m1857 = v0;
    match m1857 {
        Adt132::C0 => {
            false
        }
        Adt132::C1 => {
            true
        }
        Adt132::C2 => {
            true
        }
        Adt132::C3 => {
            false
        }
        Adt132::C4 => {
            true
        }
        Adt132::C5 => {
            false
        }
        Adt132::C6 => {
            false
        }
        Adt132::C7 => {
            true
        }
    }
}

pub fn f584(v0: Adt128, v1: Adt133, v2: Adt129, v3: u64) -> R<Adt146> {
    let a1859 = match v0.clone() {
        Adt128::C0(_, _, h1858) => {
            h1858
        }
    };
    let a1860 = f635();
    if nat_lt(a1859, a1860) {
        Ok(Adt146::C1(Adt128::C0(v2, List::cons(v1, match v0.clone() {
            Adt128::C0(_, h1861, _) => {
                h1861
            }
        }), {
            let a1863 = match v0 {
                Adt128::C0(_, _, h1862) => {
                    h1862
                }
            };
            let a1864 = 1u64;
            nat_add(a1863, a1864)?
        })))
    } else {
        Ok(f517(Adt106::C9(v3)))
    }
}

pub fn f585(v0: bool, v1: Adt132, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt129 {
    Adt129::C8(Adt131::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f586(v0: u64) -> i64 {
    let a1865 = v0;
    let a1866 = 0u64;
    if nat_eq(a1865, a1866) {
        0i64
    } else {
        let a1867 = v0;
        let a1868 = 1u64;
        if nat_eq(a1867, a1868) {
            1i64
        } else {
            let a1869 = v0;
            let a1870 = 2u64;
            if nat_eq(a1869, a1870) {
                2i64
            } else {
                let a1871 = v0;
                let a1872 = 3u64;
                if nat_eq(a1871, a1872) {
                    3i64
                } else {
                    let a1873 = v0;
                    let a1874 = 4u64;
                    if nat_eq(a1873, a1874) {
                        4i64
                    } else {
                        let a1875 = v0;
                        let a1876 = 5u64;
                        if nat_eq(a1875, a1876) {
                            5i64
                        } else {
                            let a1877 = v0;
                            let a1878 = 6u64;
                            if nat_eq(a1877, a1878) {
                                6i64
                            } else {
                                let a1879 = v0;
                                let a1880 = 7u64;
                                if nat_eq(a1879, a1880) {
                                    7i64
                                } else {
                                    let a1881 = v0;
                                    let a1882 = 8u64;
                                    if nat_eq(a1881, a1882) {
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

pub fn f587(v0: Adt128, v1: u64) -> Adt146 {
    let m1884 = match v0.clone() {
        Adt128::C0(_, h1883, _) => {
            h1883
        }
    };
    match m1884.uncons() {
        None => {
            f517(Adt106::C2(v1))
        }
        Some((v2, _)) => {
            let m1885 = v2;
            match m1885 {
                Adt133::C0(_) => {
                    f516(v0, Adt129::C0)
                }
                Adt133::C1(_, _) => {
                    f516(v0, Adt129::C4)
                }
            }
        }
    }
}

pub fn f588(v0: Adt128, v1: Adt129) -> Adt128 {
    Adt128::C0(v1, match v0.clone() {
        Adt128::C0(_, h1886, _) => {
            h1886
        }
    }, match v0 {
        Adt128::C0(_, _, h1887) => {
            h1887
        }
    })
}

pub fn f589(v0: Adt128, v1: u64, v2: List<u64>, v3: bool) -> Adt146 {
    let m1889 = {
        let a1888 = f470(f232(), f406(v2, List::<u64>::nil()));
        utf8_decode(a1888)
    };
    match m1889 {
        None => {
            f517(Adt106::C14)
        }
        Some(v4) => {
            if v3 {
                f682(v0, v1, v4)
            } else {
                f595(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f590(v0: u64) -> u64 {
    let a1890 = v0;
    let a1891 = 34u64;
    if nat_eq(a1890, a1891) {
        34u64
    } else {
        let a1892 = v0;
        let a1893 = 92u64;
        if nat_eq(a1892, a1893) {
            92u64
        } else {
            let a1894 = v0;
            let a1895 = 47u64;
            if nat_eq(a1894, a1895) {
                47u64
            } else {
                let a1896 = v0;
                let a1897 = 98u64;
                if nat_eq(a1896, a1897) {
                    8u64
                } else {
                    let a1898 = v0;
                    let a1899 = 102u64;
                    if nat_eq(a1898, a1899) {
                        12u64
                    } else {
                        let a1900 = v0;
                        let a1901 = 110u64;
                        if nat_eq(a1900, a1901) {
                            10u64
                        } else {
                            let a1902 = v0;
                            let a1903 = 114u64;
                            if nat_eq(a1902, a1903) {
                                13u64
                            } else {
                                let a1904 = v0;
                                let a1905 = 116u64;
                                if nat_eq(a1904, a1905) {
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

pub fn f591(v0: u64) -> u64 {
    let a1906 = v0;
    let a1907 = 48u64;
    if nat_lt(a1906, a1907) {
        16u64
    } else {
        let a1908 = v0;
        let a1909 = 58u64;
        if nat_lt(a1908, a1909) {
            let a1910 = v0;
            let a1911 = 48u64;
            nat_sub(a1910, a1911)
        } else {
            let a1912 = v0;
            let a1913 = 65u64;
            if nat_lt(a1912, a1913) {
                16u64
            } else {
                let a1914 = v0;
                let a1915 = 71u64;
                if nat_lt(a1914, a1915) {
                    let a1916 = v0;
                    let a1917 = 55u64;
                    nat_sub(a1916, a1917)
                } else {
                    let a1918 = v0;
                    let a1919 = 97u64;
                    if nat_lt(a1918, a1919) {
                        16u64
                    } else {
                        let a1920 = v0;
                        let a1921 = 103u64;
                        if nat_lt(a1920, a1921) {
                            let a1922 = v0;
                            let a1923 = 87u64;
                            nat_sub(a1922, a1923)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f592(v0: Adt128, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt146> {
    let a1924 = v3;
    let a1925 = 0u64;
    if nat_eq(a1924, a1925) {
        if f508(v2, 55296u64, 56319u64) {
            Ok(f516(v0, Adt129::C6(Adt130::C3(v2), v4, v5)))
        } else {
            if f508(v2, 56320u64, 57343u64) {
                Ok(f517(Adt106::C6(v1)))
            } else {
                Ok(f516(v0, Adt129::C6(Adt130::C0, f683(v2, v4)?, v5)))
            }
        }
    } else {
        if f508(v2, 56320u64, 57343u64) {
            Ok(f516(v0, Adt129::C6(Adt130::C0, f683({
                let a1934 = 65536u64;
                let a1935 = {
                    let a1932 = {
                        let a1928 = {
                            let a1926 = v3;
                            let a1927 = 55296u64;
                            nat_sub(a1926, a1927)
                        };
                        let a1929 = 1024u64;
                        nat_mul(a1928, a1929)?
                    };
                    let a1933 = {
                        let a1930 = v2;
                        let a1931 = 56320u64;
                        nat_sub(a1930, a1931)
                    };
                    nat_add(a1932, a1933)?
                };
                nat_add(a1934, a1935)?
            }, v4)?, v5)))
        } else {
            Ok(f517(Adt106::C6(v1)))
        }
    }
}

pub fn f593(v0: u64, v1: u64) -> u64 {
    let m1938 = {
        let a1936 = f594(v0);
        let a1937 = v1;
        index_list(a1936, a1937)
    };
    match m1938 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f594(v0: u64) -> List<u64> {
    let a1939 = v0;
    let a1940 = 0u64;
    if nat_eq(a1939, a1940) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1941 = v0;
        let a1942 = 1u64;
        if nat_eq(a1941, a1942) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f595(v0: Adt128, v1: Adt6) -> Adt146 {
    Adt146::C1(f510(v1, match v0.clone() {
        Adt128::C0(_, h1943, _) => {
            h1943
        }
    }, match v0 {
        Adt128::C0(_, _, h1944) => {
            h1944
        }
    }))
}

pub fn f596(v0: u64) -> Adt6 {
    let a1945 = v0;
    let a1946 = 0u64;
    if nat_eq(a1945, a1946) {
        Adt6::C1(true)
    } else {
        let a1947 = v0;
        let a1948 = 1u64;
        if nat_eq(a1947, a1948) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f597(v0: Adt128, v1: Adt131, v2: Adt132) -> Adt146 {
    f516(v0, Adt129::C8(Adt131::C0(match v1.clone() {
        Adt131::C0(h1949, _, _, _, _, _, _) => {
            h1949
        }
    }, v2, match v1.clone() {
        Adt131::C0(_, _, h1950, _, _, _, _) => {
            h1950
        }
    }, match v1.clone() {
        Adt131::C0(_, _, _, h1951, _, _, _) => {
            h1951
        }
    }, match v1.clone() {
        Adt131::C0(_, _, _, _, h1952, _, _) => {
            h1952
        }
    }, match v1.clone() {
        Adt131::C0(_, _, _, _, _, h1953, _) => {
            h1953
        }
    }, match v1 {
        Adt131::C0(_, _, _, _, _, _, h1954) => {
            h1954
        }
    })))
}

pub fn f598(v0: Adt128, v1: u64, v2: u64, v3: Adt131, v4: bool) -> R<Adt146> {
    let v5: u64 = {
        let a1956 = match v3.clone() {
            Adt131::C0(_, _, _, h1955, _, _, _) => {
                h1955
            }
        };
        let a1957 = 0u64;
        if if nat_eq(a1956, a1957) {
            let a1958 = v2;
            let a1959 = 48u64;
            nat_eq(a1958, a1959)
        } else {
            false
        } {
            0u64
        } else {
            let a1961 = match v3.clone() {
                Adt131::C0(_, _, _, h1960, _, _, _) => {
                    h1960
                }
            };
            let a1962 = 1u64;
            nat_add(a1961, a1962)?
        }
    };
    let v6: i64 = if v4 {
        f368(match v3.clone() {
            Adt131::C0(_, _, _, _, h1963, _, _) => {
                h1963
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt131::C0(_, _, _, _, h1964, _, _) => {
                h1964
            }
        }
    };
    let a1965 = f684();
    let a1966 = v5;
    if nat_lt(a1965, a1966) {
        Ok(f517(Adt106::C8(v1)))
    } else {
        let m1969 = {
            let a1967 = 99999i64;
            let a1968 = v6;
            compare(a1967, a1968)
        };
        if match m1969 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f517(Adt106::C8(v1)))
        } else {
            Ok(f516(v0, f585(match v3.clone() {
                Adt131::C0(h1970, _, _, _, _, _, _) => {
                    h1970
                }
            }, if v4 {
                Adt132::C4
            } else {
                Adt132::C2
            }, f368({
                let a1972 = match v3.clone() {
                    Adt131::C0(_, _, h1971, _, _, _, _) => {
                        h1971
                    }
                };
                let a1973 = 10i64;
                int_mul(a1972, a1973)?
            }, f586({
                let a1974 = v2;
                let a1975 = 48u64;
                nat_sub(a1974, a1975)
            }))?, v5, v6, match v3.clone() {
                Adt131::C0(_, _, _, _, _, h1976, _) => {
                    h1976
                }
            }, match v3 {
                Adt131::C0(_, _, _, _, _, _, h1977) => {
                    h1977
                }
            })))
        }
    }
}

pub fn f599(v0: u64) -> bool {
    let a1978 = v0;
    let a1979 = 101u64;
    if nat_eq(a1978, a1979) {
        true
    } else {
        let a1980 = v0;
        let a1981 = 69u64;
        nat_eq(a1980, a1981)
    }
}

pub fn f600(v0: u64) -> bool {
    f508(v0, 48u64, 57u64)
}

pub fn f601(v0: Adt128, v1: u64, v2: u64, v3: Adt131) -> R<Adt146> {
    let m1982 = f509(v3)?;
    match m1982 {
        Adt151::C0 => {
            Ok(f517(Adt106::C7(v1)))
        }
        Adt151::C1 => {
            Ok(f517(Adt106::C8(v1)))
        }
        Adt151::C2(v4) => {
            Ok(f514(f510(v4, match v0.clone() {
                Adt128::C0(_, h1983, _) => {
                    h1983
                }
            }, match v0 {
                Adt128::C0(_, _, h1984) => {
                    h1984
                }
            }), v1, v2))
        }
    }
}

pub fn f602(v0: Adt128, v1: u64, v2: u64, v3: Adt131, v4: bool) -> R<Adt146> {
    let v5: i64 = f368({
        let a1986 = match v3.clone() {
            Adt131::C0(_, _, _, _, _, _, h1985) => {
                h1985
            }
        };
        let a1987 = 10i64;
        int_mul(a1986, a1987)?
    }, f586({
        let a1988 = v2;
        let a1989 = 48u64;
        nat_sub(a1988, a1989)
    }))?;
    let m1992 = {
        let a1990 = 99999i64;
        let a1991 = v5;
        compare(a1990, a1991)
    };
    if match m1992 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f517(Adt106::C8(v1)))
    } else {
        Ok(f516(v0, f585(match v3.clone() {
            Adt131::C0(h1993, _, _, _, _, _, _) => {
                h1993
            }
        }, Adt132::C7, match v3.clone() {
            Adt131::C0(_, _, h1994, _, _, _, _) => {
                h1994
            }
        }, match v3.clone() {
            Adt131::C0(_, _, _, h1995, _, _, _) => {
                h1995
            }
        }, match v3 {
            Adt131::C0(_, _, _, _, h1996, _, _) => {
                h1996
            }
        }, v4, v5)))
    }
}

pub fn f603(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1997 = f685(f686(), v2.clone());
    match m1997 {
        None => {
            let m1998 = f687(f688(), v2.clone());
            match m1998 {
                None => {
                    f689(v0, v1, v2)
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

pub fn f604(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt134) -> R<Adt157> {
    let m1999 = f364(Str::lit("x-mcp-header"), v1.clone());
    match m1999 {
        Adt116::C0 => {
            Ok(Adt157::C1(v4))
        }
        Adt116::C1(v5) => {
            f690(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f605(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt118>) -> List<Adt118> {
    f691(Str::lit("properties"), v0.clone(), v1, v2, f691(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f692(Str::lit("items"), v0.clone(), f692(Str::lit("additionalProperties"), v0.clone(), f693(Str::lit("allOf"), v0.clone(), f693(Str::lit("anyOf"), v0.clone(), f693(Str::lit("oneOf"), v0.clone(), f692(Str::lit("not"), v0.clone(), f692(Str::lit("if"), v0.clone(), f692(Str::lit("then"), v0.clone(), f692(Str::lit("else"), v0, v3)))))))))))
}

pub fn f606(p2006: List<(Str, Adt152)>) -> Adt135 {
    let m2005 = p2006;
    {
        let mut m2005 = m2005;
        loop {
            let step = {
                let v0 = m2005;
                let m2000 = v0;
                match m2000.uncons() {
                    None => {
                        Err::<List<(Str, Adt152)>, Adt135>(Adt135::C3(List::<Adt114>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m2002 = {
                            let (_, h2001) = v1;
                            h2001
                        };
                        match m2002 {
                            Adt152::C0 => {
                                Ok::<List<(Str, Adt152)>, Adt135>(v2)
                            }
                            Adt152::C1 => {
                                Err::<List<(Str, Adt152)>, Adt135>(Adt135::C0({
                                    let (h2003, _) = v1;
                                    h2003
                                }))
                            }
                            Adt152::C2 => {
                                Err::<List<(Str, Adt152)>, Adt135>(Adt135::C1({
                                    let (h2004, _) = v1;
                                    h2004
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2005 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f607(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m2007 = f364(Str::lit("type"), v0);
    match m2007 {
        Adt116::C0 => {
            Ok(Adt152::C0)
        }
        Adt116::C1(v2) => {
            let m2008 = f190(v2.clone());
            match m2008 {
                None => {
                    let m2009 = f270(v2);
                    match m2009 {
                        None => {
                            Ok(Adt152::C2)
                        }
                        Some(v4) => {
                            f696(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f694(v3.clone()) {
                        if f695(v3, v1)? {
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

pub fn f608(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m2010 = f364(Str::lit("enum"), v0);
    match m2010 {
        Adt116::C0 => {
            Ok(Adt152::C0)
        }
        Adt116::C1(v2) => {
            let m2011 = f270(v2);
            match m2011 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v3) => {
                    f697(v3, v1)
                }
            }
        }
    }
}

pub fn f609(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m2012 = f364(Str::lit("const"), v0);
    match m2012 {
        Adt116::C0 => {
            Ok(Adt152::C0)
        }
        Adt116::C1(v2) => {
            let m2013 = f698(v2, v1)?;
            match m2013 {
                Adt159::C0 => {
                    Ok(Adt152::C0)
                }
                Adt159::C1 => {
                    Ok(Adt152::C1)
                }
                Adt159::C2 => {
                    Ok(Adt152::C2)
                }
            }
        }
    }
}

pub fn f610(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt152> {
    let m2014 = f364(v0, v2);
    match m2014 {
        Adt116::C0 => {
            Ok(Adt152::C0)
        }
        Adt116::C1(v4) => {
            let m2015 = f699(v4);
            match m2015 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v5) => {
                    let m2016 = f699(v3);
                    match m2016 {
                        None => {
                            Ok(Adt152::C0)
                        }
                        Some(v6) => {
                            if f700(v1, f701(v6, v5)?) {
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

pub fn f611(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt152)>> {
    let m2017 = v1;
    match m2017 {
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
            Ok(List::cons((Str::lit("minLength"), f702(Str::lit("minLength"), true, v0.clone(), f703(v4.clone())?)?), List::cons((Str::lit("maxLength"), f702(Str::lit("maxLength"), false, v0, f703(v4)?)?), List::<(Str, Adt152)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f702(Str::lit("minItems"), true, v0.clone(), f704(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f702(Str::lit("maxItems"), false, v0, f704(v5, 0u64)?)?), List::<(Str, Adt152)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f702(Str::lit("minProperties"), true, v0.clone(), f705(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f702(Str::lit("maxProperties"), false, v0, f705(v6, 0u64)?)?), List::<(Str, Adt152)>::nil())))
        }
    }
}

pub fn f612(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt152 {
    let m2018 = f364(Str::lit("required"), v0);
    match m2018 {
        Adt116::C0 => {
            Adt152::C0
        }
        Adt116::C1(v2) => {
            let m2019 = f270(v2);
            match m2019 {
                None => {
                    Adt152::C2
                }
                Some(v3) => {
                    let m2020 = f265(v1);
                    match m2020 {
                        None => {
                            Adt152::C0
                        }
                        Some(v4) => {
                            f706(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f613(v0: Adt135, v1: Adt135) -> Adt135 {
    let m2021 = v0.clone();
    match m2021 {
        Adt135::C0(_) => {
            v0
        }
        Adt135::C1(_) => {
            v0
        }
        Adt135::C2(_) => {
            v0
        }
        Adt135::C3(v5) => {
            let m2022 = v1.clone();
            match m2022 {
                Adt135::C0(_) => {
                    v1
                }
                Adt135::C1(_) => {
                    v1
                }
                Adt135::C2(_) => {
                    v1
                }
                Adt135::C3(v9) => {
                    Adt135::C3({
                        let a2023 = v5;
                        let a2024 = v9;
                        append_list(a2023, a2024)
                    })
                }
            }
        }
    }
}

pub fn f614(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt112>) -> Adt135 {
    let m2025 = f364(Str::lit("$ref"), v1);
    match m2025 {
        Adt116::C0 => {
            Adt135::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v4) => {
            let m2026 = f190(v4);
            match m2026 {
                None => {
                    Adt135::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m2027 = f707(v0, v5.clone());
                    match m2027 {
                        None => {
                            Adt135::C2(v5)
                        }
                        Some(v6) => {
                            Adt135::C3(List::cons(Adt114::C0(v6, v2, v3), List::<Adt114>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f615(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt112>) -> Adt135 {
    let m2028 = f364(Str::lit("properties"), v0);
    match m2028 {
        Adt116::C0 => {
            Adt135::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m2029 = f265(v3);
            match m2029 {
                None => {
                    Adt135::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m2030 = f265(v1);
                    match m2030 {
                        None => {
                            Adt135::C3(List::<Adt114>::nil())
                        }
                        Some(v5) => {
                            Adt135::C3(f708(match v4 {
                                Adt5::C0(h2031) => {
                                    h2031
                                }
                            }, match v5 {
                                Adt5::C0(h2032) => {
                                    h2032
                                }
                            }, v2, List::<Adt114>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f616(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt112>) -> Adt135 {
    let m2033 = f364(Str::lit("additionalProperties"), v0.clone());
    match m2033 {
        Adt116::C0 => {
            Adt135::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m2034 = f265(v1);
            match m2034 {
                None => {
                    Adt135::C3(List::<Adt114>::nil())
                }
                Some(v4) => {
                    let m2035 = f369(v3.clone());
                    match m2035 {
                        None => {
                            Adt135::C3(f711(match v4 {
                                Adt5::C0(h2037) => {
                                    h2037
                                }
                            }, f710(v0), v3, v2, List::<Adt114>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt135::C3(List::<Adt114>::nil())
                            } else {
                                if f709(match v4 {
                                    Adt5::C0(h2036) => {
                                        h2036
                                    }
                                }, f710(v0)) {
                                    Adt135::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt135::C3(List::<Adt114>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f617(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt112>) -> R<Adt135> {
    let m2038 = f364(Str::lit("items"), v0);
    match m2038 {
        Adt116::C0 => {
            Ok(Adt135::C3(List::<Adt114>::nil()))
        }
        Adt116::C1(v3) => {
            let m2039 = f270(v1);
            match m2039 {
                None => {
                    Ok(Adt135::C3(List::<Adt114>::nil()))
                }
                Some(v4) => {
                    Ok(Adt135::C3(f712(v4, 0u64, v3, v2, List::<Adt114>::nil())?))
                }
            }
        }
    }
}

pub fn f618(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt112>) -> Adt135 {
    let m2040 = f364(Str::lit("allOf"), v0);
    match m2040 {
        Adt116::C0 => {
            Adt135::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m2041 = f270(v3);
            match m2041 {
                None => {
                    Adt135::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m2042 = v4.clone();
                    match m2042.uncons() {
                        None => {
                            Adt135::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt135::C3(f713(v4, v1, v2, List::<Adt114>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f619(v0: Adt115, v1: Adt135, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt112>) -> Adt135 {
    let m2043 = v1.clone();
    match m2043 {
        Adt135::C0(_) => {
            v1
        }
        Adt135::C1(_) => {
            v1
        }
        Adt135::C2(_) => {
            v1
        }
        Adt135::C3(v8) => {
            let m2044 = v8;
            match m2044.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt135::C3(List::cons(Adt114::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt114>::nil()))
                }
            }
        }
    }
}

pub fn f620(p2051: List<u64>, p2052: List<u64>) -> List<u64> {
    let m2050 = (p2051, p2052);
    {
        let mut m2050 = m2050;
        loop {
            let step = {
                let (v0, v1) = m2050;
                let m2045 = v0;
                match m2045.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a2046 = v2;
                        let a2047 = 126u64;
                        if nat_eq(a2046, a2047) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a2048 = v2;
                            let a2049 = 47u64;
                            if nat_eq(a2048, a2049) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
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

pub fn f621(p2061: u64, p2062: List<u64>, p2063: List<u64>) -> R<Option<List<u64>>> {
    let m2060 = (p2061, (p2062, p2063));
    {
        let mut m2060 = m2060;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2060;
                let m2053 = v0;
                if m2053 == 0 {
                    let m2054 = v1;
                    match m2054.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m2053 - 1;
                    let m2055 = v1;
                    match m2055.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m2056 = v7;
                            match m2056.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m2057 = v9;
                                    match m2057.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m2058 = v11;
                                            match m2058.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m2059 = f714(v6, v8, v10, v12, f715(v13.clone()))?;
                                                    match m2059 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f406(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m2060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(p2068: List<Adt51>, p2069: Adt7) -> R<bool> {
    let m2067 = (p2068, p2069);
    {
        let mut m2067 = m2067;
        loop {
            let step = {
                let (v0, v1) = m2067;
                let m2064 = v0;
                match m2064.uncons() {
                    None => {
                        Err::<(List<Adt51>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f716(f701(match v2.clone() {
                            Adt51::C0(h2065, _, _) => {
                                h2065
                            }
                        }, v1)?) {
                            Ok::<(List<Adt51>, Adt7), R<bool>>((v3, match v2 {
                                Adt51::C0(h2066, _, _) => {
                                    h2066
                                }
                            }))
                        } else {
                            Err::<(List<Adt51>, Adt7), R<bool>>(Ok(false))
                        }
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

pub fn f623(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f717(v0, 0u64, v1)
}

pub fn f624(v0: Adt40) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt40::C0(h2070, _, _) => {
            h2070
        }
    })), {
        let a2073 = f718(match v0.clone() {
            Adt40::C0(_, h2071, _) => {
                h2071
            }
        });
        let a2074 = f719(match v0 {
            Adt40::C0(_, _, h2072) => {
                h2072
            }
        });
        append_list(a2073, a2074)
    })))
}

pub fn f625(v0: Str, v1: Str, v2: Str, v3: Option<Adt41>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2075 = f718(v3);
        let a2076 = f719(v4);
        append_list(a2075, a2076)
    }))))
}

pub fn f626(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2100 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2077, _, _, _, _, _, _, _, _) => {
                h2077
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2101 = {
            let a2098 = f720(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2078, _, _, _, _, _, _, _) => {
                    h2078
                }
            });
            let a2099 = {
                let a2096 = f721(match v0.clone() {
                    Adt45::C0(_, _, h2079, _, _, _, _, _, _) => {
                        h2079
                    }
                });
                let a2097 = {
                    let a2094 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2080, _, _, _, _, _) => {
                            h2080
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2095 = {
                        let a2092 = f720(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2081, _, _, _, _) => {
                                h2081
                            }
                        });
                        let a2093 = {
                            let a2090 = f720(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2082, _, _, _) => {
                                    h2082
                                }
                            });
                            let a2091 = {
                                let a2088 = f718(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2083, _, _) => {
                                        h2083
                                    }
                                });
                                let a2089 = {
                                    let a2086 = f722(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2084, _) => {
                                            h2084
                                        }
                                    })?;
                                    let a2087 = f719(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2085) => {
                                            h2085
                                        }
                                    });
                                    append_list(a2086, a2087)
                                };
                                append_list(a2088, a2089)
                            };
                            append_list(a2090, a2091)
                        };
                        append_list(a2092, a2093)
                    };
                    append_list(a2094, a2095)
                };
                append_list(a2096, a2097)
            };
            append_list(a2098, a2099)
        };
        append_list(a2100, a2101)
    }))
}

pub fn f627(v0: Adt46) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f723(match v0.clone() {
        Adt46::C0(h2102, _, _) => {
            h2102
        }
    })), {
        let a2105 = f718(match v0.clone() {
            Adt46::C0(_, h2103, _) => {
                h2103
            }
        });
        let a2106 = f719(match v0 {
            Adt46::C0(_, _, h2104) => {
                h2104
            }
        });
        append_list(a2105, a2106)
    })))
}

pub fn f628(p2112: List<(Str, Adt6)>, p2113: List<(Str, Adt65)>) -> Option<List<(Str, Adt65)>> {
    let m2111 = (p2112, p2113);
    {
        let mut m2111 = m2111;
        loop {
            let step = {
                let (v0, v1) = m2111;
                let m2107 = v0;
                match m2107.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(Some(f467(v1, List::<(Str, Adt65)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2109 = f724({
                            let (_, h2108) = v2;
                            h2108
                        });
                        match m2109 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(None::<List<(Str, Adt65)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>((v3, List::cons(({
                                    let (h2110, _) = v2;
                                    h2110
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f629(p2122: Bytes, p2123: List<Bytes>) -> List<Bytes> {
    let m2121 = (p2122, p2123);
    {
        let mut m2121 = m2121;
        loop {
            let step = {
                let (v0, v1) = m2121;
                let m2114 = v1;
                match m2114.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a2117 = {
                            let a2115 = v2.clone();
                            length_bytes(a2115)
                        };
                        let a2118 = {
                            let a2116 = v0.clone();
                            length_bytes(a2116)
                        };
                        if nat_eq(a2117, a2118) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a2119 = v2;
                                let a2120 = v0;
                                append_bytes(a2119, a2120)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f630(v0: u64) -> Bytes {
    let m2127 = {
        let a2124 = f725();
        let a2125 = v0;
        let a2126 = 1u64;
        slice_bytes(a2124, a2125, a2126)
    };
    match m2127 {
        None => {
            f232()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f631(v0: i64, v1: Str, v2: Str, v3: Adt96) -> Result<(Adt14, u64), Adt94> {
    Ok::<(Adt14, u64), Adt94>((Adt14::C0(v0, v1, v2, match v3.clone() {
        Adt96::C0(h2128, _) => {
            h2128
        }
    }), match v3 {
        Adt96::C0(_, h2129) => {
            h2129
        }
    }))
}

pub fn f632(v0: u64) -> List<u64> {
    f594(v0)
}

pub fn f633(v0: Adt7) -> R<bool> {
    let a2147 = {
        let m2133 = {
            let a2131 = f726();
            let a2132 = match v0.clone() {
                Adt7::C0(h2130, _) => {
                    h2130
                }
            };
            compare(a2131, a2132)
        };
        if match m2133 {
            Ordering::Less => {
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
            let m2138 = {
                let a2136 = match v0.clone() {
                    Adt7::C0(h2134, _) => {
                        h2134
                    }
                };
                let a2137 = {
                    let a2135 = f726();
                    int_neg(a2135)?
                };
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
                true
            } else {
                let m2142 = {
                    let a2140 = 9999i64;
                    let a2141 = match v0.clone() {
                        Adt7::C0(_, h2139) => {
                            h2139
                        }
                    };
                    compare(a2140, a2141)
                };
                if match m2142 {
                    Ordering::Less => {
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
                    let m2146 = {
                        let a2144 = match v0 {
                            Adt7::C0(_, h2143) => {
                                h2143
                            }
                        };
                        let a2145 = -9999i64;
                        compare(a2144, a2145)
                    };
                    match m2146 {
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
    Ok(bool_not(a2147))
}

pub fn f634(v0: Adt7) -> R<List<u64>> {
    let m2151 = {
        let a2149 = match v0.clone() {
            Adt7::C0(_, h2148) => {
                h2148
            }
        };
        let a2150 = 0i64;
        compare(a2149, a2150)
    };
    if if match m2151 {
        Ordering::Less => {
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
        let m2155 = {
            let a2153 = 0i64;
            let a2154 = match v0.clone() {
                Adt7::C0(_, h2152) => {
                    h2152
                }
            };
            compare(a2153, a2154)
        };
        match m2155 {
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
        let a2158 = f727(match v0.clone() {
            Adt7::C0(h2156, _) => {
                h2156
            }
        })?;
        let a2159 = List::cons(101u64, f727(match v0 {
            Adt7::C0(_, h2157) => {
                h2157
            }
        })?);
        Ok(append_list(a2158, a2159))
    } else {
        f727(match v0 {
            Adt7::C0(h2160, _) => {
                h2160
            }
        })
    }
}

pub fn f635() -> u64 {
    512u64
}

pub fn f636(v0: u64, v1: Adt120, v2: List<Adt120>) -> List<Adt120> {
    List::cons(Adt120::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f637(p2163: List<u64>, p2164: List<u64>) -> R<List<u64>> {
    let m2162 = (p2163, p2164);
    {
        let mut m2162 = m2162;
        loop {
            let step = {
                let (v0, v1) = m2162;
                let m2161 = v0;
                match m2161.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f406(f728(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f638(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f729(match v0.clone() {
        Adt34::C0(_, h2165, _, _) => {
            h2165
        }
    })), List::cons((Str::lit("capabilities"), f730(match v0.clone() {
        Adt34::C0(_, _, h2166, _) => {
            h2166
        }
    })), {
        let a2169 = f720(Str::lit("instructions"), match v0.clone() {
            Adt34::C0(_, _, _, h2167) => {
                h2167
            }
        });
        let a2170 = f731(match v0 {
            Adt34::C0(h2168, _, _, _) => {
                h2168
            }
        })?;
        append_list(a2169, a2170)
    })))
}

pub fn f639(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f732(match v0.clone() {
        Adt35::C0(_, _, h2171) => {
            h2171
        }
    }))), {
        let a2174 = f720(Str::lit("nextCursor"), match v0.clone() {
            Adt35::C0(_, h2172, _) => {
                h2172
            }
        });
        let a2175 = f731(match v0 {
            Adt35::C0(h2173, _, _) => {
                h2173
            }
        })?;
        append_list(a2174, a2175)
    }))
}

pub fn f640(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f733(match v0.clone() {
        Adt52::C0(_, _, h2176) => {
            h2176
        }
    })?)), {
        let a2179 = f720(Str::lit("nextCursor"), match v0.clone() {
            Adt52::C0(_, h2177, _) => {
                h2177
            }
        });
        let a2180 = f731(match v0 {
            Adt52::C0(h2178, _, _) => {
                h2178
            }
        })?;
        append_list(a2179, a2180)
    }))
}

pub fn f641(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f734(match v0.clone() {
        Adt53::C0(_, h2181) => {
            h2181
        }
    }))), f731(match v0 {
        Adt53::C0(h2182, _) => {
            h2182
        }
    })?))
}

pub fn f642(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f735(match v0.clone() {
        Adt54::C0(_, _, h2183) => {
            h2183
        }
    }))), {
        let a2186 = f720(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h2184, _) => {
                h2184
            }
        });
        let a2187 = f731(match v0 {
            Adt54::C0(h2185, _, _) => {
                h2185
            }
        })?;
        append_list(a2186, a2187)
    }))
}

pub fn f643(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f736(match v0.clone() {
        Adt56::C0(_, _, h2188) => {
            h2188
        }
    }))), {
        let a2191 = f720(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h2189, _) => {
                h2189
            }
        });
        let a2192 = f731(match v0 {
            Adt56::C0(h2190, _, _) => {
                h2190
            }
        })?;
        append_list(a2191, a2192)
    }))
}

pub fn f644(v0: Adt59) -> R<Option<List<(Str, Adt6)>>> {
    let m2194 = f737(match v0.clone() {
        Adt59::C0(_, h2193) => {
            h2193
        }
    })?;
    match m2194 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2196 = f720(Str::lit("description"), match v0 {
                    Adt59::C0(h2195, _) => {
                        h2195
                    }
                });
                let a2197 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a2196, a2197)
            }))
        }
    }
}

pub fn f645(v0: Adt61) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f667({
        let m2200 = match v0.clone() {
            Adt61::C0(h2199) => {
                h2199
            }
        };
        match m2200 {
            Adt62::C0(h2198, _, _) => {
                h2198
            }
        }
    }))), {
        let a2207 = f738(Str::lit("total"), {
            let m2203 = match v0.clone() {
                Adt61::C0(h2202) => {
                    h2202
                }
            };
            match m2203 {
                Adt62::C0(_, h2201, _) => {
                    h2201
                }
            }
        })?;
        let a2208 = f378(Str::lit("hasMore"), {
            let m2206 = match v0 {
                Adt61::C0(h2205) => {
                    h2205
                }
            };
            match m2206 {
                Adt62::C0(_, _, h2204) => {
                    h2204
                }
            }
        });
        append_list(a2207, a2208)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f646(v0: Adt82) -> R<Option<List<(Str, Adt6)>>> {
    let m2209 = f739(f740(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m2209 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2210 = f741(v1);
                let a2211 = f742(f743(v0));
                append_list(a2210, a2211)
            }))
        }
    }
}

pub fn f647(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m2212 = f744(v0.clone());
    match m2212 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a2213 = f745(f382(v0.clone()), f746(f747(v0)));
                let a2214 = v1;
                append_list(a2213, a2214)
            })
        }
    }
}

pub fn f648(v0: Adt88) -> Option<List<(Str, Adt6)>> {
    let m2216 = match v0.clone() {
        Adt88::C0(_, h2215, _) => {
            h2215
        }
    };
    match m2216 {
        Adt89::C0 => {
            f647({
                let m2219 = match v0 {
                    Adt88::C0(h2218, _, _) => {
                        h2218
                    }
                };
                match m2219 {
                    Adt85::C0(h2217, _, _) => {
                        h2217
                    }
                }
            })
        }
        Adt89::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f649(v0: Adt158) -> Str {
    let m2220 = v0;
    match m2220 {
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

pub fn f650(v0: Adt33) -> Adt158 {
    let m2221 = v0;
    match m2221 {
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

pub fn f651(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m2222 = v0;
    match m2222 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f748(), f749(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f652(v0: List<Adt85>) -> R<Option<List<u64>>> {
    let m2223 = f750(v0);
    match m2223 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m2224 = f233(v1)?;
            match m2224 {
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

pub fn f653(v0: Adt85, v1: List<Adt85>, v2: i64) -> R<List<Adt85>> {
    Ok(List::cons(v0.clone(), f380(f751(f290(v0), v1, v2, List::<Adt85>::nil())?, List::<Adt85>::nil())))
}

pub fn f654(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f752(v2) {
            let a2225 = v1;
            let a2226 = 200u64;
            nat_eq(a2225, a2226)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f655(v0: u64, v1: List<List<u64>>) -> Adt95 {
    Adt95::C0(v0, f753(), List::<List<u64>>::nil(), f754(v1), None::<List<u64>>)
}

pub fn f656(v0: Adt141) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f327())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f755(match v0.clone() {
        Adt141::C0(h2227, _) => {
            h2227
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m2230 = match v0.clone() {
            Adt141::C0(_, h2229) => {
                h2229
            }
        };
        match m2230 {
            Adt51::C0(h2228, _, _) => {
                h2228
            }
        }
    })), {
        let a2237 = f756({
            let m2233 = match v0.clone() {
                Adt141::C0(_, h2232) => {
                    h2232
                }
            };
            match m2233 {
                Adt51::C0(_, h2231, _) => {
                    h2231
                }
            }
        });
        let a2238 = f757({
            let m2236 = match v0 {
                Adt141::C0(_, h2235) => {
                    h2235
                }
            };
            match m2236 {
                Adt51::C0(_, _, h2234) => {
                    h2234
                }
            }
        });
        append_list(a2237, a2238)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f657(v0: Adt88) -> bool {
    if match v0.clone() {
        Adt88::C0(_, _, h2239) => {
            h2239
        }
    } {
        let m2241 = match v0 {
            Adt88::C0(_, h2240, _) => {
                h2240
            }
        };
        match m2241 {
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

pub fn f658(v0: Adt63) -> Option<Adt6> {
    let m2242 = f647(v0);
    match m2242 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f659() -> u64 {
    429u64
}

pub fn f660() -> i64 {
    -32602i64
}

pub fn f661() -> i64 {
    -32022i64
}

pub fn f662() -> i64 {
    -32020i64
}

pub fn f663() -> i64 {
    -32601i64
}

pub fn f664(v0: Adt90) -> i64 {
    let m2243 = v0;
    match m2243 {
        Adt90::C0(_) => {
            f660()
        }
        Adt90::C1(_) => {
            f660()
        }
        Adt90::C2(_) => {
            f660()
        }
        Adt90::C3(_) => {
            f660()
        }
        Adt90::C4(_) => {
            f660()
        }
        Adt90::C5(_) => {
            f660()
        }
        Adt90::C6(_) => {
            f758()
        }
        Adt90::C7(_) => {
            f759()
        }
        Adt90::C8(_) => {
            f760()
        }
        Adt90::C9(_) => {
            f758()
        }
        Adt90::C10 => {
            f660()
        }
        Adt90::C11 => {
            f660()
        }
        Adt90::C12(_) => {
            f758()
        }
        Adt90::C13 => {
            f758()
        }
        Adt90::C14 => {
            f758()
        }
        Adt90::C15(_) => {
            f758()
        }
    }
}

pub fn f665(v0: Adt143) -> Str {
    let m2244 = v0;
    match m2244 {
        Adt143::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt143::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt143::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt143::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt143::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt143::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt143::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt143::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt143::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f666(v0: Adt122) -> Str {
    let m2245 = v0;
    match m2245 {
        Adt122::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt122::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt122::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt122::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt122::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt122::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt122::C6(v1) => {
            let m2246 = v1;
            match m2246 {
                Adt123::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt123::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt123::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt122::C7(v2) => {
            let m2247 = v2;
            match m2247 {
                Adt123::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt123::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt123::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt122::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt122::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt122::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f667(v0: List<Str>) -> List<Adt6> {
    f461(f761(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f668(v0: Adt18) -> Adt6 {
    Adt6::C5({
        let a2259 = f762(Str::lit("experimental"), match v0.clone() {
            Adt18::C0(h2248, _, _, _, _) => {
                h2248
            }
        });
        let a2260 = {
            let a2257 = f762(Str::lit("roots"), match v0.clone() {
                Adt18::C0(_, h2249, _, _, _) => {
                    h2249
                }
            });
            let a2258 = {
                let a2255 = f762(Str::lit("sampling"), match v0.clone() {
                    Adt18::C0(_, _, h2250, _, _) => {
                        h2250
                    }
                });
                let a2256 = {
                    let a2253 = f762(Str::lit("elicitation"), match v0.clone() {
                        Adt18::C0(_, _, _, h2251, _) => {
                            h2251
                        }
                    });
                    let a2254 = f762(Str::lit("extensions"), match v0 {
                        Adt18::C0(_, _, _, _, h2252) => {
                            h2252
                        }
                    });
                    append_list(a2253, a2254)
                };
                append_list(a2255, a2256)
            };
            append_list(a2257, a2258)
        };
        append_list(a2259, a2260)
    })
}

pub fn f669(p2263: List<List<u64>>) -> R<bool> {
    let m2262 = p2263;
    {
        let mut m2262 = m2262;
        loop {
            let step = {
                let v0 = m2262;
                let m2261 = v0;
                match m2261.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f763(f342(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
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

pub fn f670() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f671(v0: Adt0, v1: Fn5, v2: u64, v3: Adt149, v4: bool) -> R<Result<Adt139, Adt142>> {
    if v4 {
        f764(v0, v1, v2, f765(match v3.clone() {
            Adt149::C0(_, h2264, _, _) => {
                h2264
            }
        }, match v3.clone() {
            Adt149::C0(_, _, h2265, _) => {
                h2265
            }
        }, match v3 {
            Adt149::C0(_, _, _, h2266) => {
                h2266
            }
        }))
    } else {
        Ok(Err::<Adt139, Adt142>(Adt142::C3(f766(match v3 {
            Adt149::C0(_, h2267, _, _) => {
                h2267
            }
        }))))
    }
}

pub fn f672(v0: Adt4, v1: Adt150) -> bool {
    let m2268 = v1;
    match m2268 {
        Adt150::C0 => {
            true
        }
        Adt150::C1 => {
            f767(match v0 {
                Adt4::C0(_, _, _, _, _, h2269, _) => {
                    h2269
                }
            })
        }
        Adt150::C2 => {
            f767(match v0 {
                Adt4::C0(_, _, _, _, _, h2270, _) => {
                    h2270
                }
            })
        }
        Adt150::C3 => {
            f768(match v0 {
                Adt4::C0(_, _, _, _, h2271, _, _) => {
                    h2271
                }
            })
        }
        Adt150::C4 => {
            f768(match v0 {
                Adt4::C0(_, _, _, _, h2272, _, _) => {
                    h2272
                }
            })
        }
        Adt150::C5 => {
            f768(match v0 {
                Adt4::C0(_, _, _, _, h2273, _, _) => {
                    h2273
                }
            })
        }
        Adt150::C6 => {
            f767(match v0 {
                Adt4::C0(_, _, _, h2274, _, _, _) => {
                    h2274
                }
            })
        }
        Adt150::C7 => {
            f767(match v0 {
                Adt4::C0(_, _, _, h2275, _, _, _) => {
                    h2275
                }
            })
        }
        Adt150::C8 => {
            f278(match v0 {
                Adt4::C0(_, _, h2276, _, _, _, _) => {
                    h2276
                }
            })
        }
        Adt150::C9 => {
            f288(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2277) => {
                    h2277
                }
            }, f28())
        }
        Adt150::C10 => {
            f288(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2278) => {
                    h2278
                }
            }, f28())
        }
        Adt150::C11 => {
            f288(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2279) => {
                    h2279
                }
            }, f28())
        }
    }
}

pub fn f673(v0: Adt155, v1: Adt153) -> Result<Adt149, Adt142> {
    let m2280 = v1;
    match m2280 {
        Adt153::C0 => {
            Err::<Adt149, Adt142>(Adt142::C3(match v0 {
                Adt155::C0(_, h2281, _, _) => {
                    h2281
                }
            }))
        }
        Adt153::C1(v2) => {
            Ok::<Adt149, Adt142>(Adt149::C0(match v0.clone() {
                Adt155::C0(h2282, _, _, _) => {
                    h2282
                }
            }, v2, match v0.clone() {
                Adt155::C0(_, _, h2283, _) => {
                    h2283
                }
            }, match v0 {
                Adt155::C0(_, _, _, h2284) => {
                    h2284
                }
            }))
        }
    }
}

pub fn f674(v0: Option<Adt121>, v1: List<Str>, v2: Adt117, v3: Result<Adt17, Adt143>) -> R<Adt154> {
    let m2285 = v3;
    match m2285 {
        Ok(v5) => {
            Ok(f769(v1, v2.clone(), v5.clone(), f770(v0, f576(match v2.clone() {
                Adt117::C0(_, h2286, _) => {
                    h2286
                }
            }), match v2.clone() {
                Adt117::C0(_, h2287, _) => {
                    h2287
                }
            }, match v5 {
                Adt17::C0(h2288, _, _, _, _, _) => {
                    h2288
                }
            }, f771(match v2 {
                Adt117::C0(_, _, h2289) => {
                    h2289
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt154::C1(Adt142::C0(v4)))
        }
    }
}

pub fn f675(v0: Option<Adt5>) -> Result<Adt17, Adt143> {
    let m2290 = v0;
    match m2290 {
        None => {
            Err::<Adt17, Adt143>(Adt143::C0)
        }
        Some(v1) => {
            let m2291 = f264(Str::lit("_meta"), v1);
            match m2291 {
                Adt116::C0 => {
                    Err::<Adt17, Adt143>(Adt143::C1)
                }
                Adt116::C1(v2) => {
                    let m2292 = f265(v2);
                    match m2292 {
                        None => {
                            Err::<Adt17, Adt143>(Adt143::C4)
                        }
                        Some(v3) => {
                            f772(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f676(v0: Adt150) -> Adt156 {
    let m2293 = v0;
    match m2293 {
        Adt150::C0 => {
            Adt156::C0
        }
        Adt150::C1 => {
            Adt156::C0
        }
        Adt150::C2 => {
            Adt156::C1
        }
        Adt150::C3 => {
            Adt156::C0
        }
        Adt150::C4 => {
            Adt156::C2
        }
        Adt150::C5 => {
            Adt156::C0
        }
        Adt150::C6 => {
            Adt156::C0
        }
        Adt150::C7 => {
            Adt156::C1
        }
        Adt150::C8 => {
            Adt156::C0
        }
        Adt150::C9 => {
            Adt156::C3
        }
        Adt150::C10 => {
            Adt156::C3
        }
        Adt150::C11 => {
            Adt156::C3
        }
    }
}

pub fn f677(p2299: List<u64>, p2300: List<u64>) -> Option<List<u64>> {
    let m2298 = (p2299, p2300);
    {
        let mut m2298 = m2298;
        loop {
            let step = {
                let (v0, v1) = m2298;
                let m2294 = v0;
                match m2294.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2295 = v1;
                        match m2295.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2296 = v4;
                                let a2297 = v2;
                                if nat_eq(a2296, a2297) {
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
                Ok(next) => { m2298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f678() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f679() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f680(v0: Adt12, v1: Option<Str>) -> bool {
    let m2301 = v1;
    match m2301 {
        None => {
            false
        }
        Some(v2) => {
            let a2303 = match v0 {
                Adt12::C0(h2302, _, _, _) => {
                    h2302
                }
            };
            let a2304 = v2;
            equal(a2303, a2304)
        }
    }
}

pub fn f681(v0: List<Adt93>, v1: Str) -> R<Adt125> {
    let m2307 = f499(f141(v0, {
        let a2305 = f773();
        let a2306 = f774(v1)?;
        append_list(a2305, a2306)
    })?)?;
    match m2307 {
        Adt109::C0 => {
            Ok(Adt125::C0)
        }
        Adt109::C1 => {
            Ok(Adt125::C1)
        }
        Adt109::C2 => {
            Ok(Adt125::C1)
        }
        Adt109::C3(v2) => {
            let m2308 = f555(v2);
            match m2308 {
                None => {
                    Ok(Adt125::C1)
                }
                Some(v3) => {
                    Ok(Adt125::C2(v3))
                }
            }
        }
    }
}

pub fn f682(v0: Adt128, v1: u64, v2: Str) -> Adt146 {
    let m2310 = match v0.clone() {
        Adt128::C0(_, h2309, _) => {
            h2309
        }
    };
    match m2310.uncons() {
        None => {
            f517(Adt106::C2(v1))
        }
        Some((v3, v4)) => {
            let m2311 = v3;
            match m2311 {
                Adt133::C0(_) => {
                    f517(Adt106::C2(v1))
                }
                Adt133::C1(v6, _) => {
                    Adt146::C1(Adt128::C0(Adt129::C5, List::cons(Adt133::C1(v6, Some(v2)), v4), match v0 {
                        Adt128::C0(_, _, h2312) => {
                            h2312
                        }
                    }))
                }
            }
        }
    }
}

pub fn f683(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2313 = v0;
    let a2314 = 128u64;
    if nat_lt(a2313, a2314) {
        Ok(List::cons(v0, v1))
    } else {
        let a2315 = v0;
        let a2316 = 2048u64;
        if nat_lt(a2315, a2316) {
            Ok(List::cons({
                let a2317 = 128u64;
                let a2318 = f775(v0, 64u64);
                nat_add(a2317, a2318)?
            }, List::cons({
                let a2319 = 192u64;
                let a2320 = f776(v0, 64u64);
                nat_add(a2319, a2320)?
            }, v1)))
        } else {
            let a2321 = v0;
            let a2322 = 65536u64;
            if nat_lt(a2321, a2322) {
                Ok(List::cons({
                    let a2323 = 128u64;
                    let a2324 = f775(v0, 64u64);
                    nat_add(a2323, a2324)?
                }, List::cons({
                    let a2325 = 128u64;
                    let a2326 = f775(f776(v0, 64u64), 64u64);
                    nat_add(a2325, a2326)?
                }, List::cons({
                    let a2327 = 224u64;
                    let a2328 = f776(v0, 4096u64);
                    nat_add(a2327, a2328)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2329 = 128u64;
                    let a2330 = f775(v0, 64u64);
                    nat_add(a2329, a2330)?
                }, List::cons({
                    let a2331 = 128u64;
                    let a2332 = f775(f776(v0, 64u64), 64u64);
                    nat_add(a2331, a2332)?
                }, List::cons({
                    let a2333 = 128u64;
                    let a2334 = f775(f776(v0, 4096u64), 64u64);
                    nat_add(a2333, a2334)?
                }, List::cons({
                    let a2335 = 240u64;
                    let a2336 = f776(v0, 262144u64);
                    nat_add(a2335, a2336)?
                }, v1)))))
            }
        }
    }
}

pub fn f684() -> u64 {
    18u64
}

pub fn f685(p2340: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2339 = p2340;
    {
        let mut m2339 = m2339;
        loop {
            let step = {
                let v0 = m2339;
                let m2337 = v0;
                match m2337.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2338 = f364(v2.clone(), v1.clone());
                        match m2338 {
                            Adt116::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt116::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f687(p2347: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2346 = p2347;
    {
        let mut m2346 = m2346;
        loop {
            let step = {
                let v0 = m2346;
                let m2341 = v0;
                match m2341.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2343 = f364({
                            let (h2342, _) = v2.clone();
                            h2342
                        }, v1.clone());
                        match m2343 {
                            Adt116::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt116::C1(v4) => {
                                if f777({
                                    let (_, h2344) = v2;
                                    h2344
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h2345, _) = v2;
                                        h2345
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f688() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f689(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt98> {
    let m2348 = f364(Str::lit("$ref"), v2);
    match m2348 {
        Adt116::C0 => {
            None::<Adt98>
        }
        Adt116::C1(v3) => {
            let m2349 = f190(v3);
            match m2349 {
                None => {
                    None::<Adt98>
                }
                Some(v4) => {
                    let m2350 = f707(v1, v4.clone());
                    match m2350 {
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

pub fn f690(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt134) -> R<Adt157> {
    if v3 {
        let m2351 = f190(v2);
        match m2351 {
            None => {
                Ok(Adt157::C0(Adt98::C7(v0)))
            }
            Some(v6) => {
                f778(v0, v1, v6.clone(), f257(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt157::C0(Adt98::C10(v0)))
    }
}

pub fn f691(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt118>) -> List<Adt118> {
    let m2352 = f364(v0.clone(), v1);
    match m2352 {
        Adt116::C0 => {
            v4
        }
        Adt116::C1(v5) => {
            let m2353 = f265(v5);
            match m2353 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a2354 = v0;
                    let a2355 = Str::lit("properties");
                    if equal(a2354, a2355) {
                        f779(match v6 {
                            Adt5::C0(h2356) => {
                                h2356
                            }
                        }, v2, v3, v4)
                    } else {
                        f780(match v6 {
                            Adt5::C0(h2357) => {
                                h2357
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f692(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt118>) -> List<Adt118> {
    let m2358 = f364(v0, v1);
    match m2358 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            List::cons(Adt118::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f693(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt118>) -> List<Adt118> {
    let m2359 = f364(v0, v1);
    match m2359 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            let m2360 = f270(v3);
            match m2360 {
                None => {
                    v2
                }
                Some(v4) => {
                    f781(v4, v2)
                }
            }
        }
    }
}

pub fn f694(v0: Str) -> bool {
    let a2361 = v0.clone();
    let a2362 = Str::lit("null");
    if equal(a2361, a2362) {
        true
    } else {
        let a2363 = v0.clone();
        let a2364 = Str::lit("boolean");
        if equal(a2363, a2364) {
            true
        } else {
            let a2365 = v0.clone();
            let a2366 = Str::lit("object");
            if equal(a2365, a2366) {
                true
            } else {
                let a2367 = v0.clone();
                let a2368 = Str::lit("array");
                if equal(a2367, a2368) {
                    true
                } else {
                    let a2369 = v0.clone();
                    let a2370 = Str::lit("number");
                    if equal(a2369, a2370) {
                        true
                    } else {
                        let a2371 = v0.clone();
                        let a2372 = Str::lit("string");
                        if equal(a2371, a2372) {
                            true
                        } else {
                            let a2373 = v0;
                            let a2374 = Str::lit("integer");
                            equal(a2373, a2374)
                        }
                    }
                }
            }
        }
    }
}

pub fn f695(v0: Str, v1: Adt6) -> R<bool> {
    let a2375 = v0.clone();
    let a2376 = f782(v1.clone());
    if equal(a2375, a2376) {
        Ok(true)
    } else {
        let a2377 = v0;
        let a2378 = Str::lit("integer");
        if equal(a2377, a2378) {
            let m2379 = f699(v1);
            match m2379 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f783(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f696(p2382: List<Adt6>, v1: Adt6, p2383: bool) -> R<Adt152> {
    let m2381 = (p2382, p2383);
    {
        let mut m2381 = m2381;
        loop {
            let step = {
                let (v0, v2) = m2381;
                let m2380 = v0;
                match m2380.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f694(f784(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt152>>((v4, if v2 {
                                true
                            } else {
                                f695(f784(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f697(p2387: List<Adt6>, v1: Adt6) -> R<Adt152> {
    let m2386 = p2387;
    {
        let mut m2386 = m2386;
        loop {
            let step = {
                let v0 = m2386;
                let m2384 = v0;
                match m2384.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt152>>(Ok(Adt152::C1))
                    }
                    Some((v2, v3)) => {
                        let m2385 = f698(v2, v1.clone())?;
                        match m2385 {
                            Adt159::C0 => {
                                Err::<List<Adt6>, R<Adt152>>(Ok(Adt152::C0))
                            }
                            Adt159::C1 => {
                                Ok::<List<Adt6>, R<Adt152>>(v3)
                            }
                            Adt159::C2 => {
                                Err::<List<Adt6>, R<Adt152>>(Ok(Adt152::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f698(v0: Adt6, v1: Adt6) -> R<Adt159> {
    f785(f786(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f699(v0: Adt6) -> Option<Adt7> {
    let m2388 = v0;
    match m2388 {
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

pub fn f700(v0: u64, v1: Adt160) -> bool {
    let a2389 = v0;
    let a2390 = 0u64;
    if nat_eq(a2389, a2390) {
        let a2391 = f787(v1);
        bool_not(a2391)
    } else {
        let a2392 = v0;
        let a2393 = 1u64;
        if nat_eq(a2392, a2393) {
            let a2394 = f716(v1);
            bool_not(a2394)
        } else {
            let a2395 = v0;
            let a2396 = 2u64;
            if nat_eq(a2395, a2396) {
                f716(v1)
            } else {
                f787(v1)
            }
        }
    }
}

pub fn f701(v0: Adt7, v1: Adt7) -> R<Adt160> {
    f788(v0.clone(), v1.clone(), f789(match v0 {
        Adt7::C0(h2397, _) => {
            h2397
        }
    }), f789(match v1 {
        Adt7::C0(h2398, _) => {
            h2398
        }
    }))
}

pub fn f702(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt152> {
    let m2399 = f364(v0, v2);
    match m2399 {
        Adt116::C0 => {
            Ok(Adt152::C0)
        }
        Adt116::C1(v4) => {
            let m2400 = f283(v4);
            match m2400 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v5) => {
                    let m2403 = {
                        let a2401 = v5;
                        let a2402 = 0i64;
                        compare(a2401, a2402)
                    };
                    if match m2403 {
                        Ordering::Less => {
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
                            if f787(f790(f269(v3, 0i64)?, v5)) {
                                Ok(Adt152::C1)
                            } else {
                                Ok(Adt152::C0)
                            }
                        } else {
                            if f716(f790(f269(v3, 0i64)?, v5)) {
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

pub fn f703(v0: Str) -> R<u64> {
    f791(f229({
        let a2404 = v0;
        utf8_encode(a2404)
    })?, 0u64)
}

pub fn f704(p2409: List<Adt6>, p2410: u64) -> R<u64> {
    let m2408 = (p2409, p2410);
    {
        let mut m2408 = m2408;
        loop {
            let step = {
                let (v0, v1) = m2408;
                let m2405 = v0;
                match m2405.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a2406 = v1;
                            let a2407 = 1u64;
                            nat_add(a2406, a2407)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f705(p2415: List<(Str, Adt6)>, p2416: u64) -> R<u64> {
    let m2414 = (p2415, p2416);
    {
        let mut m2414 = m2414;
        loop {
            let step = {
                let (v0, v1) = m2414;
                let m2411 = v0;
                match m2411.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a2412 = v1;
                            let a2413 = 1u64;
                            nat_add(a2412, a2413)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f706(p2420: List<Adt6>, v1: Adt5) -> Adt152 {
    let m2419 = p2420;
    {
        let mut m2419 = m2419;
        loop {
            let step = {
                let v0 = m2419;
                let m2417 = v0;
                match m2417.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt152>(Adt152::C0)
                    }
                    Some((v2, v3)) => {
                        let m2418 = f190(v2);
                        match m2418 {
                            None => {
                                Err::<List<Adt6>, Adt152>(Adt152::C2)
                            }
                            Some(v4) => {
                                if f379(v4, v1.clone()) {
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
                Ok(next) => { m2419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f707(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a2421 = v1.clone();
    let a2422 = Str::lit("#");
    if equal(a2421, a2422) {
        Some(v0)
    } else {
        let m2423 = f265(v0);
        match m2423 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m2424 = f264(Str::lit("$defs"), v2);
                match m2424 {
                    Adt116::C0 => {
                        None::<Adt6>
                    }
                    Adt116::C1(v3) => {
                        let m2425 = f265(v3);
                        match m2425 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f792(v1, match v4 {
                                    Adt5::C0(h2426) => {
                                        h2426
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

pub fn f708(p2433: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt112>, p2434: List<Adt114>) -> List<Adt114> {
    let m2432 = (p2433, p2434);
    {
        let mut m2432 = m2432;
        loop {
            let step = {
                let (v0, v3) = m2432;
                let m2427 = v0;
                match m2427.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt114>), List<Adt114>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2429 = f364({
                            let (h2428, _) = v4.clone();
                            h2428
                        }, v1.clone());
                        match m2429 {
                            Adt116::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt114>), List<Adt114>>((v5, v3))
                            }
                            Adt116::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt114>), List<Adt114>>((v5, List::cons(Adt114::C0({
                                    let (_, h2430) = v4;
                                    h2430
                                }, v6, List::cons(Adt112::C0({
                                    let (h2431, _) = v4;
                                    h2431
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f709(p2439: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2438 = p2439;
    {
        let mut m2438 = m2438;
        loop {
            let step = {
                let v0 = m2438;
                let m2435 = v0;
                match m2435.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2437 = f364({
                            let (h2436, _) = v2;
                            h2436
                        }, v1.clone());
                        match m2437 {
                            Adt116::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt116::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
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

pub fn f710(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2440 = f364(Str::lit("properties"), v0);
    match m2440 {
        Adt116::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt116::C1(v1) => {
            let m2441 = f265(v1);
            match m2441 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2442) => {
                            h2442
                        }
                    }
                }
            }
        }
    }
}

pub fn f711(p2449: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt112>, p2450: List<Adt114>) -> List<Adt114> {
    let m2448 = (p2449, p2450);
    {
        let mut m2448 = m2448;
        loop {
            let step = {
                let (v0, v4) = m2448;
                let m2443 = v0;
                match m2443.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt114>), List<Adt114>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2445 = f364({
                            let (h2444, _) = v5.clone();
                            h2444
                        }, v1.clone());
                        match m2445 {
                            Adt116::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt114>), List<Adt114>>((v6, List::cons(Adt114::C0(v2.clone(), {
                                    let (_, h2446) = v5;
                                    h2446
                                }, List::cons(Adt112::C0({
                                    let (h2447, _) = v5;
                                    h2447
                                }), v3.clone())), v4)))
                            }
                            Adt116::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt114>), List<Adt114>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f712(p2455: List<Adt6>, p2456: u64, v2: Adt6, v3: List<Adt112>, p2457: List<Adt114>) -> R<List<Adt114>> {
    let m2454 = (p2455, (p2456, p2457));
    {
        let mut m2454 = m2454;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2454;
                let m2451 = v0;
                match m2451.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt114>)), R<List<Adt114>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt114>)), R<List<Adt114>>>((v6, ({
                            let a2452 = v1;
                            let a2453 = 1u64;
                            nat_add(a2452, a2453)?
                        }, List::cons(Adt114::C0(v2.clone(), v5, List::cons(Adt112::C1(v1), v3.clone())), v4))))
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

pub fn f713(p2460: List<Adt6>, v1: Adt6, v2: List<Adt112>, p2461: List<Adt114>) -> List<Adt114> {
    let m2459 = (p2460, p2461);
    {
        let mut m2459 = m2459;
        loop {
            let step = {
                let (v0, v3) = m2459;
                let m2458 = v0;
                match m2458.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt114>), List<Adt114>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt114>), List<Adt114>>((v5, List::cons(Adt114::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f714(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2462 = f793(v0)?;
    match m2462 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2463 = f793(v1)?;
            match m2463 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2464 = v2;
                    let a2465 = 61u64;
                    if nat_eq(a2464, a2465) {
                        f794(v5, v6, v4, v3)
                    } else {
                        let m2466 = f793(v2)?;
                        match m2466 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2467 = v3;
                                let a2468 = 61u64;
                                if nat_eq(a2467, a2468) {
                                    f795(v5, v6, v7, v4)
                                } else {
                                    let m2469 = f793(v3)?;
                                    match m2469 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f796(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f715(v0: List<u64>) -> bool {
    let m2470 = v0;
    match m2470.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f716(v0: Adt160) -> bool {
    let m2471 = v0;
    match m2471 {
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

pub fn f717(p2476: List<u64>, p2477: u64, p2478: List<u64>) -> List<u64> {
    let m2475 = (p2476, (p2477, p2478));
    {
        let mut m2475 = m2475;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2475;
                let m2472 = v0;
                match m2472.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f797(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt161 = f798(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt161::C0(h2473, _) => {
                                h2473
                            }
                        }, match v5 {
                            Adt161::C0(_, h2474) => {
                                h2474
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: Option<Adt41>) -> List<(Str, Adt6)> {
    let m2479 = v0;
    match m2479 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2485 = f799(match v1.clone() {
                    Adt41::C0(h2480, _, _) => {
                        h2480
                    }
                });
                let a2486 = {
                    let a2483 = f800(match v1.clone() {
                        Adt41::C0(_, h2481, _) => {
                            h2481
                        }
                    });
                    let a2484 = f720(Str::lit("lastModified"), match v1 {
                        Adt41::C0(_, _, h2482) => {
                            h2482
                        }
                    });
                    append_list(a2483, a2484)
                };
                append_list(a2485, a2486)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f719(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f762(Str::lit("_meta"), v0)
}

pub fn f720(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2487 = v1;
    match m2487 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f721(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2488 = v0;
    match m2488 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f801(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f722(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2489 = v0;
    match m2489 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f152(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f723(v0: Adt47) -> Adt6 {
    let m2490 = v0;
    match m2490 {
        Adt47::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt48::C0(h2491, _, _, _) => {
                    h2491
                }
            })), {
                let a2495 = f720(Str::lit("mimeType"), match v1.clone() {
                    Adt48::C0(_, h2492, _, _) => {
                        h2492
                    }
                });
                let a2496 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt48::C0(_, _, h2493, _) => {
                        h2493
                    }
                })), f719(match v1 {
                    Adt48::C0(_, _, _, h2494) => {
                        h2494
                    }
                }));
                append_list(a2495, a2496)
            }))
        }
        Adt47::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt49::C0(h2497, _, _, _) => {
                    h2497
                }
            })), {
                let a2501 = f720(Str::lit("mimeType"), match v2.clone() {
                    Adt49::C0(_, h2498, _, _) => {
                        h2498
                    }
                });
                let a2502 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt49::C0(_, _, h2499, _) => {
                        h2499
                    }
                })), f719(match v2 {
                    Adt49::C0(_, _, _, h2500) => {
                        h2500
                    }
                }));
                append_list(a2501, a2502)
            }))
        }
    }
}

pub fn f724(v0: Adt6) -> Option<Adt65> {
    let m2503 = f265(v0);
    match m2503 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2504 = f540(Str::lit("method"), v1.clone());
            match m2504 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a2505 = v2.clone();
                    let a2506 = Str::lit("elicitation/create");
                    if equal(a2505, a2506) {
                        let m2507 = f463(Str::lit("params"), v1);
                        match m2507 {
                            None => {
                                None::<Adt65>
                            }
                            Some(v3) => {
                                f802(v3)
                            }
                        }
                    } else {
                        let a2508 = v2;
                        let a2509 = Str::lit("roots/list");
                        if equal(a2508, a2509) {
                            f803(v1)
                        } else {
                            None::<Adt65>
                        }
                    }
                }
            }
        }
    }
}

pub fn f725() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f726() -> i64 {
    999999999999999999i64
}

pub fn f727(v0: i64) -> R<List<u64>> {
    let m2512 = {
        let a2510 = v0;
        let a2511 = 0i64;
        compare(a2510, a2511)
    };
    if match m2512 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f804(20u64, {
            let a2513 = v0;
            int_neg(a2513)?
        }, List::<u64>::nil())?))
    } else {
        f804(20u64, v0, List::<u64>::nil())
    }
}

pub fn f728(v0: u64) -> R<List<u64>> {
    let a2514 = v0;
    let a2515 = 34u64;
    if nat_eq(a2514, a2515) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2516 = v0;
        let a2517 = 92u64;
        if nat_eq(a2516, a2517) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2518 = v0;
            let a2519 = 8u64;
            if nat_eq(a2518, a2519) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2520 = v0;
                let a2521 = 9u64;
                if nat_eq(a2520, a2521) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2522 = v0;
                    let a2523 = 10u64;
                    if nat_eq(a2522, a2523) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2524 = v0;
                        let a2525 = 12u64;
                        if nat_eq(a2524, a2525) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2526 = v0;
                            let a2527 = 13u64;
                            if nat_eq(a2526, a2527) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2528 = v0;
                                let a2529 = 32u64;
                                if nat_lt(a2528, a2529) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f805(f776(v0, 16u64))?, List::cons(f805(f775(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f729(v0: List<Str>) -> Adt6 {
    Adt6::C4(f667(v0))
}

pub fn f730(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2547 = f762(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2530, _, _, _, _, _, _) => {
                h2530
            }
        });
        let a2548 = {
            let a2545 = f762(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2531, _, _, _, _, _) => {
                    h2531
                }
            });
            let a2546 = {
                let a2543 = f762(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2532, _, _, _, _) => {
                        h2532
                    }
                });
                let a2544 = {
                    let a2541 = f806(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2533, _, _, _) => {
                            h2533
                        }
                    });
                    let a2542 = {
                        let a2539 = f807(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2534, _, _) => {
                                h2534
                            }
                        });
                        let a2540 = {
                            let a2537 = f806(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2535, _) => {
                                    h2535
                                }
                            });
                            let a2538 = f762(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2536) => {
                                    h2536
                                }
                            });
                            append_list(a2537, a2538)
                        };
                        append_list(a2539, a2540)
                    };
                    append_list(a2541, a2542)
                };
                append_list(a2543, a2544)
            };
            append_list(a2545, a2546)
        };
        append_list(a2547, a2548)
    })
}

pub fn f731(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f152(match v0.clone() {
        Adt10::C0(h2549, _) => {
            h2549
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f808(match v0 {
        Adt10::C0(_, h2550) => {
            h2550
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f732(v0: List<Adt36>) -> List<Adt6> {
    f461(f809(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f733(v0: List<Adt45>) -> R<List<Adt6>> {
    Ok(f461(f810(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f734(v0: List<Adt47>) -> List<Adt6> {
    f461(f811(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f735(v0: List<Adt55>) -> List<Adt6> {
    f461(f812(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f736(v0: List<Adt57>) -> List<Adt6> {
    f461(f813(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f737(v0: List<Adt60>) -> R<Option<List<Adt6>>> {
    let m2551 = f814(v0, List::<Adt6>::nil())?;
    match m2551 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f461(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f738(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2552 = v1;
    match m2552 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f152(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f739(p2558: List<(Str, Adt65)>, p2559: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m2557 = (p2558, p2559);
    {
        let mut m2557 = m2557;
        loop {
            let step = {
                let (v0, v1) = m2557;
                let m2553 = v0;
                match m2553.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2555 = f815({
                            let (_, h2554) = v2;
                            h2554
                        })?;
                        match m2555 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h2556, _) = v2;
                                    h2556
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f740(v0: Adt82) -> List<(Str, Adt65)> {
    let m2560 = v0;
    match m2560 {
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

pub fn f741(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2561 = v0.clone();
    match m2561.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f537(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f742(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2562 = v0;
    match m2562 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f743(v0: Adt82) -> Option<Str> {
    let m2563 = v0;
    match m2563 {
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

pub fn f744(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m2564 = v0;
    match m2564 {
        Adt63::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt63::C1(_, v3) => {
            let m2565 = f816(v3);
            match m2565 {
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
                Adt5::C0(h2566) => {
                    h2566
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f329(v8)), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C4(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f745(v0: Adt64, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt64::C0(h2567, _, _, _, _, _) => {
            h2567
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a2573 = f817(Str::lit("statusMessage"), match v0.clone() {
            Adt64::C0(_, h2568, _, _, _, _) => {
                h2568
            }
        });
        let a2574 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, h2569, _, _, _) => {
                h2569
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, _, h2570, _, _) => {
                h2570
            }
        })), List::cons((Str::lit("ttlMs"), f818(match v0.clone() {
            Adt64::C0(_, _, _, _, h2571, _) => {
                h2571
            }
        })), f819(Str::lit("pollIntervalMs"), match v0 {
            Adt64::C0(_, _, _, _, _, h2572) => {
                h2572
            }
        }))));
        append_list(a2573, a2574)
    }))
}

pub fn f746(v0: Adt84) -> Str {
    let m2575 = v0;
    match m2575 {
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

pub fn f747(v0: Adt63) -> Adt84 {
    let m2576 = v0;
    match m2576 {
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

pub fn f748() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f749(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2577, _, _, _, _, _) => {
            h2577
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2578, _, _, _) => {
            h2578
        }
    })), {
        let a2587 = f720(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2579, _, _, _, _) => {
                h2579
            }
        });
        let a2588 = {
            let a2585 = f720(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2580, _, _) => {
                    h2580
                }
            });
            let a2586 = {
                let a2583 = f720(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2581, _) => {
                        h2581
                    }
                });
                let a2584 = f721(match v0 {
                    Adt1::C0(_, _, _, _, _, h2582) => {
                        h2582
                    }
                });
                append_list(a2583, a2584)
            };
            append_list(a2585, a2586)
        };
        append_list(a2587, a2588)
    })))
}

pub fn f750(v0: List<Adt85>) -> Option<Adt6> {
    let m2589 = f820(v0, List::<Adt6>::nil());
    match m2589 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f461(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f751(v0: Str, p2594: List<Adt85>, v2: i64, p2595: List<Adt85>) -> R<List<Adt85>> {
    let m2593 = (p2594, p2595);
    {
        let mut m2593 = m2593;
        loop {
            let step = {
                let (v1, v3) = m2593;
                let m2590 = v1;
                match m2590.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a2591 = f290(v4.clone());
                        let a2592 = v0.clone();
                        if if equal(a2591, a2592) {
                            true
                        } else {
                            f291(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f752(v0: List<List<u64>>) -> bool {
    let m2596 = v0;
    match m2596.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f753() -> List<Adt93> {
    List::cons(Adt93::C0(f821(), f670()), List::cons(Adt93::C0(f822(), f823()), List::cons(Adt93::C0(f824(), f825()), List::<Adt93>::nil())))
}

pub fn f754(v0: List<List<u64>>) -> List<u64> {
    f826(f49(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f755(v0: Adt20) -> Adt6 {
    let m2597 = v0;
    match m2597 {
        Adt20::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt20::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f756(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2598 = v0;
    match m2598 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f757(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2599 = v0;
    match m2599 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f758() -> i64 {
    -32603i64
}

pub fn f759() -> i64 {
    429i64
}

pub fn f760() -> i64 {
    -32021i64
}

pub fn f761(p2602: List<Str>, p2603: List<Adt6>) -> List<Adt6> {
    let m2601 = (p2602, p2603);
    {
        let mut m2601 = m2601;
        loop {
            let step = {
                let (v0, v1) = m2601;
                let m2600 = v0;
                match m2600.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f762(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2604 = v1;
    match m2604 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2605) => {
                    h2605
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f763(v0: List<u64>) -> R<bool> {
    let m2606 = v0;
    match m2606.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2607 = v2;
            match m2607.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2608 = f426(v1)?;
                    let a2609 = 113u64;
                    if if nat_eq(a2608, a2609) {
                        let a2610 = v3;
                        let a2611 = 61u64;
                        nat_eq(a2610, a2611)
                    } else {
                        false
                    } {
                        Ok(f827(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f764(v0: Adt0, v1: Fn5, v2: u64, v3: Result<Adt15, Adt143>) -> R<Result<Adt139, Adt142>> {
    let m2612 = v3;
    match m2612 {
        Ok(v5) => {
            f828(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt139, Adt142>(Adt142::C0(v4)))
        }
    }
}

pub fn f765(v0: Adt150, v1: Adt17, v2: Adt5) -> Result<Adt15, Adt143> {
    let m2613 = v0;
    match m2613 {
        Adt150::C0 => {
            f829(v1, v2)
        }
        Adt150::C1 => {
            f830(v1, v2)
        }
        Adt150::C2 => {
            f831(v1, v2)
        }
        Adt150::C3 => {
            f832(v1, v2)
        }
        Adt150::C4 => {
            f833(v1, v2)
        }
        Adt150::C5 => {
            f834(v1, v2)
        }
        Adt150::C6 => {
            f835(v1, v2)
        }
        Adt150::C7 => {
            f836(v1, v2)
        }
        Adt150::C8 => {
            f837(v1, v2)
        }
        Adt150::C9 => {
            f838(v1, v2)
        }
        Adt150::C10 => {
            f839(v1, v2)
        }
        Adt150::C11 => {
            f840(v1, v2)
        }
    }
}

pub fn f766(v0: Adt150) -> Str {
    let m2614 = v0;
    match m2614 {
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

pub fn f767(v0: Option<Adt8>) -> bool {
    let m2615 = v0;
    match m2615 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f768(v0: Option<Adt9>) -> bool {
    let m2616 = v0;
    match m2616 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f769(v0: List<Str>, v1: Adt117, v2: Adt17, v3: Option<Adt122>) -> Adt154 {
    let m2617 = v3;
    match m2617 {
        None => {
            f841(v1, v2.clone(), f842(match v2 {
                Adt17::C0(h2618, _, _, _, _, _) => {
                    h2618
                }
            }, v0))
        }
        Some(v4) => {
            Adt154::C1(Adt142::C2(v4))
        }
    }
}

pub fn f770(v0: Option<Adt121>, v1: Adt156, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt122>> {
    let m2619 = v0;
    match m2619 {
        None => {
            Ok(None::<Adt122>)
        }
        Some(v5) => {
            let m2621 = match v5.clone() {
                Adt121::C0(_, _, _, h2620, _) => {
                    h2620
                }
            };
            match m2621 {
                None => {
                    f843(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f771(v0: Option<Adt5>) -> Adt5 {
    let m2622 = v0;
    match m2622 {
        None => {
            f27()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f772(v0: Adt5) -> Result<Adt17, Adt143> {
    let m2623 = f844(v0.clone());
    match m2623 {
        Adt162::C0 => {
            f845(v0.clone(), f264(f846(), v0.clone()), f264(f847(), v0))
        }
        Adt162::C1 => {
            Err::<Adt17, Adt143>(Adt143::C2)
        }
        Adt162::C2 => {
            Err::<Adt17, Adt143>(Adt143::C3)
        }
    }
}

pub fn f773() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f774(v0: Str) -> R<List<u64>> {
    f229({
        let a2624 = v0;
        utf8_encode(a2624)
    })
}

pub fn f775(v0: u64, v1: u64) -> u64 {
    let a2625 = v0;
    let a2626 = v1;
    let a2627 = 0u64;
    nat_rem(a2625, a2626, a2627)
}

pub fn f776(v0: u64, v1: u64) -> u64 {
    let a2628 = v0;
    let a2629 = v1;
    let a2630 = 0u64;
    nat_quot(a2628, a2629, a2630)
}

pub fn f777(v0: u64, v1: Adt6) -> bool {
    let a2631 = v0;
    let a2632 = 0u64;
    if nat_eq(a2631, a2632) {
        f848(v1)
    } else {
        let a2633 = v0;
        let a2634 = 1u64;
        if nat_eq(a2633, a2634) {
            f849(v1)
        } else {
            let a2635 = v0;
            let a2636 = 2u64;
            if nat_eq(a2635, a2636) {
                f850(v1)
            } else {
                let a2637 = v0;
                let a2638 = 3u64;
                if nat_eq(a2637, a2638) {
                    f851(v1)
                } else {
                    let a2639 = v0;
                    let a2640 = 4u64;
                    if nat_eq(a2639, a2640) {
                        f852(v1)
                    } else {
                        let a2641 = v0;
                        let a2642 = 5u64;
                        if nat_eq(a2641, a2642) {
                            f853(v1)
                        } else {
                            let a2643 = v0;
                            let a2644 = 6u64;
                            if nat_eq(a2643, a2644) {
                                f854(v1)
                            } else {
                                let a2645 = v0;
                                let a2646 = 7u64;
                                if nat_eq(a2645, a2646) {
                                    f855(v1)
                                } else {
                                    f856(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f778(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt134) -> R<Adt157> {
    let m2647 = v3.clone();
    match m2647.uncons() {
        None => {
            Ok(Adt157::C0(Adt98::C8(v0)))
        }
        Some((_, _)) => {
            if f430(v3.clone(), true) {
                let m2648 = f857(v1);
                match m2648 {
                    None => {
                        Ok(Adt157::C0(Adt98::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f858(v3)?;
                        if f859(v9.clone(), match v5.clone() {
                            Adt134::C0(_, h2649) => {
                                h2649
                            }
                        }) {
                            Ok(Adt157::C0(Adt98::C12(v0, v2)))
                        } else {
                            Ok(Adt157::C1(Adt134::C0(List::cons(Adt12::C0(v0, v2, f279(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt134::C0(h2650, _) => {
                                    h2650
                                }
                            }), List::cons(v9, match v5 {
                                Adt134::C0(_, h2651) => {
                                    h2651
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt157::C0(Adt98::C9(v0)))
            }
        }
    }
}

pub fn f779(p2656: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2657: List<Adt118>) -> List<Adt118> {
    let m2655 = (p2656, p2657);
    {
        let mut m2655 = m2655;
        loop {
            let step = {
                let (v0, v3) = m2655;
                let m2652 = v0;
                match m2652.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt118>), List<Adt118>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt118>), List<Adt118>>((v5, List::cons(Adt118::C0({
                            let (_, h2653) = v4;
                            h2653
                        }, v1, List::cons({
                            let (h2654, _) = v4;
                            h2654
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f780(p2661: List<(Str, Adt6)>, p2662: List<Adt118>) -> List<Adt118> {
    let m2660 = (p2661, p2662);
    {
        let mut m2660 = m2660;
        loop {
            let step = {
                let (v0, v1) = m2660;
                let m2658 = v0;
                match m2658.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt118>), List<Adt118>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt118>), List<Adt118>>((v3, List::cons(Adt118::C0({
                            let (_, h2659) = v2;
                            h2659
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f781(p2665: List<Adt6>, p2666: List<Adt118>) -> List<Adt118> {
    let m2664 = (p2665, p2666);
    {
        let mut m2664 = m2664;
        loop {
            let step = {
                let (v0, v1) = m2664;
                let m2663 = v0;
                match m2663.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt118>), List<Adt118>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt118>), List<Adt118>>((v3, List::cons(Adt118::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f782(v0: Adt6) -> Str {
    let m2667 = v0;
    match m2667 {
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

pub fn f783(v0: Adt7) -> R<bool> {
    if f374(match v0.clone() {
        Adt7::C0(h2668, _) => {
            h2668
        }
    }) {
        Ok(true)
    } else {
        f860(40u64, match v0.clone() {
            Adt7::C0(h2669, _) => {
                h2669
            }
        }, match v0 {
            Adt7::C0(_, h2670) => {
                h2670
            }
        })
    }
}

pub fn f784(v0: Adt6) -> Str {
    let m2671 = f190(v0);
    match m2671 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f785(p2678: u64, p2679: List<(Adt6, Adt6)>) -> R<Adt159> {
    let m2677 = (p2678, p2679);
    {
        let mut m2677 = m2677;
        loop {
            let step = {
                let (v0, v1) = m2677;
                let m2672 = v0;
                if m2672 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt159>>(Ok(Adt159::C2))
                } else {
                    let v2 = m2672 - 1;
                    let m2673 = v1;
                    match m2673.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt159>>(Ok(Adt159::C0))
                        }
                        Some((v3, v4)) => {
                            let m2676 = f861({
                                let (h2674, _) = v3;
                                h2674
                            }, {
                                let (_, h2675) = v3;
                                h2675
                            }, v4)?;
                            match m2676 {
                                Adt163::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt159>>(Ok(Adt159::C1))
                                }
                                Adt163::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt159>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f786() -> u64 {
    67108864u64
}

pub fn f787(v0: Adt160) -> bool {
    let m2680 = v0;
    match m2680 {
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

pub fn f788(v0: Adt7, v1: Adt7, v2: Adt160, v3: Adt160) -> R<Adt160> {
    let m2681 = v2;
    match m2681 {
        Adt160::C0 => {
            let m2683 = v3;
            match m2683 {
                Adt160::C0 => {
                    Ok(f862(Adt160::C0, f863(f864(match v0.clone() {
                        Adt7::C0(h2684, _) => {
                            h2684
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2685) => {
                            h2685
                        }
                    }, f864(match v1.clone() {
                        Adt7::C0(h2686, _) => {
                            h2686
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2687) => {
                            h2687
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
            let m2682 = v3;
            match m2682 {
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
            let m2688 = v3;
            match m2688 {
                Adt160::C0 => {
                    Ok(Adt160::C2)
                }
                Adt160::C1 => {
                    Ok(Adt160::C2)
                }
                Adt160::C2 => {
                    f863(match v0.clone() {
                        Adt7::C0(h2689, _) => {
                            h2689
                        }
                    }, match v0 {
                        Adt7::C0(_, h2690) => {
                            h2690
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2691, _) => {
                            h2691
                        }
                    }, match v1 {
                        Adt7::C0(_, h2692) => {
                            h2692
                        }
                    })
                }
            }
        }
    }
}

pub fn f789(v0: i64) -> Adt160 {
    f790(v0, 0i64)
}

pub fn f790(v0: i64, v1: i64) -> Adt160 {
    let m2695 = {
        let a2693 = v0;
        let a2694 = v1;
        compare(a2693, a2694)
    };
    if match m2695 {
        Ordering::Less => {
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
        let m2698 = {
            let a2696 = v1;
            let a2697 = v0;
            compare(a2696, a2697)
        };
        if match m2698 {
            Ordering::Less => {
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

pub fn f791(p2707: List<u64>, p2708: u64) -> R<u64> {
    let m2706 = (p2707, p2708);
    {
        let mut m2706 = m2706;
        loop {
            let step = {
                let (v0, v1) = m2706;
                let m2699 = v0;
                match m2699.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2700 = v2;
                        let a2701 = 128u64;
                        if if nat_lt(a2700, a2701) {
                            true
                        } else {
                            let a2702 = 191u64;
                            let a2703 = v2;
                            nat_lt(a2702, a2703)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2704 = v1;
                                let a2705 = 1u64;
                                nat_add(a2704, a2705)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f792(v0: Str, p2726: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2725 = p2726;
    {
        let mut m2725 = m2725;
        loop {
            let step = {
                let v1 = m2725;
                let m2709 = v1;
                match m2709.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2713 = v0.clone();
                        let a2714 = {
                            let a2711 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2710, _) = v2.clone();
                                h2710
                            }, List::<Str>::nil()));
                            let a2712 = Str::lit("");
                            join(a2711, a2712)
                        };
                        if equal(a2713, a2714) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2715) = v2;
                                h2715
                            }))
                        } else {
                            let m2717 = f265({
                                let (_, h2716) = v2.clone();
                                h2716
                            });
                            match m2717 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2718 = f264(Str::lit("$anchor"), v4);
                                    match m2718 {
                                        Adt116::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt116::C1(v5) => {
                                            let m2719 = f190(v5);
                                            match m2719 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2722 = v0.clone();
                                                    let a2723 = {
                                                        let a2720 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2721 = Str::lit("");
                                                        join(a2720, a2721)
                                                    };
                                                    if equal(a2722, a2723) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2724) = v2;
                                                            h2724
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
                Ok(next) => { m2725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f793(v0: u64) -> R<Option<u64>> {
    let a2727 = 64u64;
    let a2728 = v0;
    if if nat_lt(a2727, a2728) {
        let a2729 = v0;
        let a2730 = 91u64;
        nat_lt(a2729, a2730)
    } else {
        false
    } {
        Ok(Some({
            let a2731 = v0;
            let a2732 = 65u64;
            nat_sub(a2731, a2732)
        }))
    } else {
        let a2733 = 96u64;
        let a2734 = v0;
        if if nat_lt(a2733, a2734) {
            let a2735 = v0;
            let a2736 = 123u64;
            nat_lt(a2735, a2736)
        } else {
            false
        } {
            Ok(Some({
                let a2737 = v0;
                let a2738 = 71u64;
                nat_sub(a2737, a2738)
            }))
        } else {
            let a2739 = 47u64;
            let a2740 = v0;
            if if nat_lt(a2739, a2740) {
                let a2741 = v0;
                let a2742 = 58u64;
                nat_lt(a2741, a2742)
            } else {
                false
            } {
                Ok(Some({
                    let a2743 = v0;
                    let a2744 = 4u64;
                    nat_add(a2743, a2744)?
                }))
            } else {
                let a2745 = v0;
                let a2746 = 43u64;
                if nat_eq(a2745, a2746) {
                    Ok(Some(62u64))
                } else {
                    let a2747 = v0;
                    let a2748 = 47u64;
                    if nat_eq(a2747, a2748) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f794(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2749 = v3;
        let a2750 = 61u64;
        if nat_eq(a2749, a2750) {
            let a2751 = f865(v1, 16u64);
            let a2752 = 0u64;
            nat_eq(a2751, a2752)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f866(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f795(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2753 = f865(v2, 4u64);
        let a2754 = 0u64;
        nat_eq(a2753, a2754)
    } else {
        false
    } {
        Ok(Some(f867(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f796(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2757 = {
            let a2755 = v0;
            let a2756 = 4u64;
            nat_mul(a2755, a2756)?
        };
        let a2758 = f868(v1, 16u64);
        nat_add(a2757, a2758)?
    }, List::cons({
        let a2761 = {
            let a2759 = f865(v1, 16u64);
            let a2760 = 16u64;
            nat_mul(a2759, a2760)?
        };
        let a2762 = f868(v2, 4u64);
        nat_add(a2761, a2762)?
    }, List::cons({
        let a2765 = {
            let a2763 = f865(v2, 4u64);
            let a2764 = 64u64;
            nat_mul(a2763, a2764)?
        };
        let a2766 = v3;
        nat_add(a2765, a2766)?
    }, List::<u64>::nil()))))
}

pub fn f797(v0: u64, v1: List<u64>) -> List<u64> {
    let a2767 = v0;
    let a2768 = 0u64;
    if nat_eq(a2767, a2768) {
        v1
    } else {
        let a2769 = v0;
        let a2770 = 1u64;
        if nat_eq(a2769, a2770) {
            List::cons(194u64, v1)
        } else {
            let a2771 = v0;
            let a2772 = 2u64;
            if nat_eq(a2771, a2772) {
                List::cons(226u64, v1)
            } else {
                let a2773 = v0;
                let a2774 = 3u64;
                if nat_eq(a2773, a2774) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f798(v0: u64, v1: u64, v2: List<u64>) -> Adt161 {
    let a2775 = v0;
    let a2776 = 0u64;
    if nat_eq(a2775, a2776) {
        f869(v1, v2)
    } else {
        let a2777 = v0;
        let a2778 = 1u64;
        if nat_eq(a2777, a2778) {
            let a2779 = 128u64;
            let a2780 = v1;
            if if nat_le(a2779, a2780) {
                let a2781 = v1;
                let a2782 = 159u64;
                nat_le(a2781, a2782)
            } else {
                false
            } {
                Adt161::C0(0u64, f870(v2))
            } else {
                f869(v1, List::cons(194u64, v2))
            }
        } else {
            let a2783 = v0;
            let a2784 = 2u64;
            if nat_eq(a2783, a2784) {
                let a2785 = v1;
                let a2786 = 128u64;
                if nat_eq(a2785, a2786) {
                    Adt161::C0(3u64, v2)
                } else {
                    let a2787 = v1;
                    let a2788 = 129u64;
                    if nat_eq(a2787, a2788) {
                        Adt161::C0(4u64, v2)
                    } else {
                        f869(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2789 = v0;
                let a2790 = 3u64;
                if nat_eq(a2789, a2790) {
                    let a2791 = 170u64;
                    let a2792 = v1;
                    if if nat_le(a2791, a2792) {
                        let a2793 = v1;
                        let a2794 = 174u64;
                        nat_le(a2793, a2794)
                    } else {
                        false
                    } {
                        Adt161::C0(0u64, f870(v2))
                    } else {
                        f869(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2795 = 166u64;
                    let a2796 = v1;
                    if if nat_le(a2795, a2796) {
                        let a2797 = v1;
                        let a2798 = 169u64;
                        nat_le(a2797, a2798)
                    } else {
                        false
                    } {
                        Adt161::C0(0u64, f870(v2))
                    } else {
                        f869(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f799(v0: Option<List<Adt42>>) -> List<(Str, Adt6)> {
    let m2799 = v0;
    match m2799 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f871(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f800(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2800 = v0;
    match m2800 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f801(v0: List<Adt2>) -> List<Adt6> {
    f461(f872(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f802(v0: Adt5) -> Option<Adt65> {
    let m2801 = f540(Str::lit("message"), v0.clone());
    match m2801 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2802 = f264(Str::lit("mode"), v0.clone());
            match m2802 {
                Adt116::C0 => {
                    let m2803 = f463(Str::lit("requestedSchema"), v0);
                    match m2803 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v2) => {
                            Some(Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v1, v2))))
                        }
                    }
                }
                Adt116::C1(v3) => {
                    let m2804 = f190(v3);
                    match m2804 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v4) => {
                            let a2805 = v4.clone();
                            let a2806 = Str::lit("url");
                            if equal(a2805, a2806) {
                                let m2807 = f540(Str::lit("url"), v0);
                                match m2807 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v5) => {
                                        Some(Adt65::C0(Adt66::C1(Adt69::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a2808 = v4;
                                let a2809 = Str::lit("form");
                                if equal(a2808, a2809) {
                                    let m2810 = f463(Str::lit("requestedSchema"), v0);
                                    match m2810 {
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

pub fn f803(v0: Adt5) -> Option<Adt65> {
    let m2811 = f264(Str::lit("params"), v0);
    match m2811 {
        Adt116::C0 => {
            Some(Adt65::C2(None::<Adt5>))
        }
        Adt116::C1(v1) => {
            let m2812 = f265(v1);
            match m2812 {
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

pub fn f804(p2824: u64, p2825: i64, p2826: List<u64>) -> R<List<u64>> {
    let m2823 = (p2824, (p2825, p2826));
    {
        let mut m2823 = m2823;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2823;
                let m2813 = v0;
                if m2813 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2813 - 1;
                    let v4: i64 = {
                        let a2814 = v1;
                        let a2815 = 10i64;
                        let a2816 = 0i64;
                        int_quot(a2814, a2815, a2816)?
                    };
                    let v5: i64 = {
                        let a2817 = v1;
                        let a2818 = 10i64;
                        let a2819 = 0i64;
                        int_rem(a2817, a2818, a2819)
                    };
                    let m2822 = {
                        let a2820 = v4;
                        let a2821 = 1i64;
                        compare(a2820, a2821)
                    };
                    if match m2822 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f873(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f873(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f805(v0: u64) -> R<u64> {
    let a2827 = v0;
    let a2828 = 10u64;
    if nat_lt(a2827, a2828) {
        let a2829 = 48u64;
        let a2830 = v0;
        nat_add(a2829, a2830)
    } else {
        let a2831 = 87u64;
        let a2832 = v0;
        nat_add(a2831, a2832)
    }
}

pub fn f806(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2833 = v1;
    match m2833 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f874(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f807(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2834 = v0;
    match m2834 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2837 = f378(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2835, _) => {
                        h2835
                    }
                });
                let a2838 = f378(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2836) => {
                        h2836
                    }
                });
                append_list(a2837, a2838)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f808(v0: Adt11) -> Str {
    let m2839 = v0;
    match m2839 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f809(p2842: List<Adt36>, p2843: List<Adt6>) -> List<Adt6> {
    let m2841 = (p2842, p2843);
    {
        let mut m2841 = m2841;
        loop {
            let step = {
                let (v0, v1) = m2841;
                let m2840 = v0;
                match m2840.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt36>, List<Adt6>), List<Adt6>>((v3, List::cons(f875(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f810(p2846: List<Adt45>, p2847: List<Adt6>) -> R<List<Adt6>> {
    let m2845 = (p2846, p2847);
    {
        let mut m2845 = m2845;
        loop {
            let step = {
                let (v0, v1) = m2845;
                let m2844 = v0;
                match m2844.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f876(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f811(p2850: List<Adt47>, p2851: List<Adt6>) -> List<Adt6> {
    let m2849 = (p2850, p2851);
    {
        let mut m2849 = m2849;
        loop {
            let step = {
                let (v0, v1) = m2849;
                let m2848 = v0;
                match m2848.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), List<Adt6>>((v3, List::cons(f723(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f812(p2854: List<Adt55>, p2855: List<Adt6>) -> List<Adt6> {
    let m2853 = (p2854, p2855);
    {
        let mut m2853 = m2853;
        loop {
            let step = {
                let (v0, v1) = m2853;
                let m2852 = v0;
                match m2852.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt55>, List<Adt6>), List<Adt6>>((v3, List::cons(f877(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f813(p2858: List<Adt57>, p2859: List<Adt6>) -> List<Adt6> {
    let m2857 = (p2858, p2859);
    {
        let mut m2857 = m2857;
        loop {
            let step = {
                let (v0, v1) = m2857;
                let m2856 = v0;
                match m2856.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f878(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2857 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f814(p2863: List<Adt60>, p2864: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2862 = (p2863, p2864);
    {
        let mut m2862 = m2862;
        loop {
            let step = {
                let (v0, v1) = m2862;
                let m2860 = v0;
                match m2860.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2861 = f879(v2)?;
                        match m2861 {
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
                Ok(next) => { m2862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f815(v0: Adt65) -> R<Option<Adt6>> {
    let m2865 = v0;
    match m2865 {
        Adt65::C0(v1) => {
            Ok(Some(f880(Str::lit("elicitation/create"), Some(f881(v1)))))
        }
        Adt65::C1(v2) => {
            let m2866 = f882(v2)?;
            match m2866 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f880(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt65::C2(v4) => {
            Ok(Some(f880(Str::lit("roots/list"), {
                let m2867 = v4;
                match m2867 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h2868) => {
                                h2868
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f816(v0: List<(Str, Adt65)>) -> Option<Adt6> {
    let m2869 = f883(v0, List::<(Str, Adt6)>::nil());
    match m2869 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f537(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f817(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2870 = v1;
    match m2870 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f818(v0: Option<i64>) -> Adt6 {
    let m2871 = v0;
    match m2871 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f884(v1)
        }
    }
}

pub fn f819(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m2872 = v1;
    match m2872 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f884(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f820(p2876: List<Adt85>, p2877: List<Adt6>) -> Option<List<Adt6>> {
    let m2875 = (p2876, p2877);
    {
        let mut m2875 = m2875;
        loop {
            let step = {
                let (v0, v1) = m2875;
                let m2873 = v0;
                match m2873.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2874 = f885(v2);
                        match m2874 {
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
                Ok(next) => { m2875 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f821() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f822() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f823() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f824() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f825() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f826(p2882: List<List<u64>>, p2883: List<u64>) -> List<u64> {
    let m2881 = (p2882, p2883);
    {
        let mut m2881 = m2881;
        loop {
            let step = {
                let (v0, v1) = m2881;
                let m2878 = v0;
                match m2878.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2879 = f886(v2);
                            let a2880 = v1;
                            append_list(a2879, a2880)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f827(v0: List<u64>) -> bool {
    let m2884 = v0;
    match m2884.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2885 = v1;
            let a2886 = 48u64;
            if nat_eq(a2885, a2886) {
                f887(v2)
            } else {
                false
            }
        }
    }
}

pub fn f828(v0: Adt0, v1: Fn5, v2: u64, v3: Adt15) -> R<Result<Adt139, Adt142>> {
    if f888(f889(v3.clone())) {
        Ok(Ok::<Adt139, Adt142>(f890(v0.clone(), Adt33::C0(f891(v0)))))
    } else {
        Ok(f892(v0.clone(), v3.clone(), f893(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2887) => {
                h2887
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2888, _) => {
                h2888
            }
        }, v2, v1, v3)?))
    }
}

pub fn f829(v0: Adt17, _: Adt5) -> Result<Adt15, Adt143> {
    Ok::<Adt15, Adt143>(Adt15::C0(Adt16::C0(v0)))
}

pub fn f830(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2889 = f894(Str::lit("cursor"), v1);
    match m2889 {
        Ok(v2) => {
            Ok::<Adt15, Adt143>(Adt15::C1(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt143>(v3)
        }
    }
}

pub fn f831(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2890 = f895(Str::lit("name"), v1.clone());
    match m2890 {
        Ok(v2) => {
            let m2891 = f896(Str::lit("arguments"), v1.clone());
            match m2891 {
                Ok(v3) => {
                    let m2892 = f896(Str::lit("inputResponses"), v1.clone());
                    match m2892 {
                        Ok(v4) => {
                            let m2893 = f894(Str::lit("requestState"), v1);
                            match m2893 {
                                Ok(v5) => {
                                    Ok::<Adt15, Adt143>(Adt15::C2(Adt22::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt15, Adt143>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt15, Adt143>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt15, Adt143>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt15, Adt143>(v9)
        }
    }
}

pub fn f832(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2894 = f894(Str::lit("cursor"), v1);
    match m2894 {
        Ok(v2) => {
            Ok::<Adt15, Adt143>(Adt15::C3(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt143>(v3)
        }
    }
}

pub fn f833(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2895 = f895(Str::lit("uri"), v1.clone());
    match m2895 {
        Ok(v2) => {
            let m2896 = f896(Str::lit("inputResponses"), v1.clone());
            match m2896 {
                Ok(v3) => {
                    let m2897 = f894(Str::lit("requestState"), v1);
                    match m2897 {
                        Ok(v4) => {
                            Ok::<Adt15, Adt143>(Adt15::C4(Adt23::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt15, Adt143>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt15, Adt143>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt15, Adt143>(v7)
        }
    }
}

pub fn f834(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2898 = f894(Str::lit("cursor"), v1);
    match m2898 {
        Ok(v2) => {
            Ok::<Adt15, Adt143>(Adt15::C5(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt143>(v3)
        }
    }
}

pub fn f835(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2899 = f894(Str::lit("cursor"), v1);
    match m2899 {
        Ok(v2) => {
            Ok::<Adt15, Adt143>(Adt15::C6(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt143>(v3)
        }
    }
}

pub fn f836(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2900 = f895(Str::lit("name"), v1.clone());
    match m2900 {
        Ok(v2) => {
            let m2901 = f897(Str::lit("arguments"), v1.clone());
            match m2901 {
                Ok(v3) => {
                    let m2902 = f896(Str::lit("inputResponses"), v1.clone());
                    match m2902 {
                        Ok(v4) => {
                            let m2903 = f894(Str::lit("requestState"), v1);
                            match m2903 {
                                Ok(v5) => {
                                    Ok::<Adt15, Adt143>(Adt15::C7(Adt24::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt15, Adt143>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt15, Adt143>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt15, Adt143>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt15, Adt143>(v9)
        }
    }
}

pub fn f837(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2904 = f898(Str::lit("ref"), v1.clone());
    match m2904 {
        Ok(v2) => {
            let m2905 = f899(v2);
            match m2905 {
                Ok(v3) => {
                    let m2906 = f898(Str::lit("argument"), v1.clone());
                    match m2906 {
                        Ok(v4) => {
                            let m2907 = f900(v4);
                            match m2907 {
                                Ok(v5) => {
                                    let m2908 = f901(v1);
                                    match m2908 {
                                        Ok(v6) => {
                                            Ok::<Adt15, Adt143>(Adt15::C8(Adt25::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt15, Adt143>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt15, Adt143>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt15, Adt143>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt15, Adt143>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt15, Adt143>(v11)
        }
    }
}

pub fn f838(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2909 = f895(Str::lit("taskId"), v1);
    match m2909 {
        Ok(v2) => {
            Ok::<Adt15, Adt143>(Adt15::C9(Adt30::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt143>(v3)
        }
    }
}

pub fn f839(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2910 = f895(Str::lit("taskId"), v1.clone());
    match m2910 {
        Ok(v2) => {
            let m2911 = f898(Str::lit("inputResponses"), v1);
            match m2911 {
                Ok(v3) => {
                    Ok::<Adt15, Adt143>(Adt15::C10(Adt31::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt15, Adt143>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt15, Adt143>(v5)
        }
    }
}

pub fn f840(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt143> {
    let m2912 = f895(Str::lit("taskId"), v1);
    match m2912 {
        Ok(v2) => {
            Ok::<Adt15, Adt143>(Adt15::C11(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt143>(v3)
        }
    }
}

pub fn f841(v0: Adt117, v1: Adt17, v2: Option<Adt142>) -> Adt154 {
    let m2913 = v2;
    match m2913 {
        None => {
            Adt154::C0(Adt155::C0(match v0.clone() {
                Adt117::C0(h2914, _, _) => {
                    h2914
                }
            }, match v0.clone() {
                Adt117::C0(_, h2915, _) => {
                    h2915
                }
            }, v1, f771(match v0 {
                Adt117::C0(_, _, h2916) => {
                    h2916
                }
            })))
        }
        Some(v3) => {
            Adt154::C1(v3)
        }
    }
}

pub fn f842(v0: Str, v1: List<Str>) -> Option<Adt142> {
    f902(f903(v0.clone(), v1.clone()), v0, v1)
}

pub fn f843(v0: Adt121, v1: Adt156, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt122>> {
    let m2918 = match v0.clone() {
        Adt121::C0(h2917, _, _, _, _) => {
            h2917
        }
    };
    match m2918 {
        None => {
            Ok(Some(Adt122::C0))
        }
        Some(v5) => {
            let a2919 = v5;
            let a2920 = v3;
            if equal(a2919, a2920) {
                let m2922 = match v0.clone() {
                    Adt121::C0(_, h2921, _, _, _) => {
                        h2921
                    }
                };
                match m2922 {
                    None => {
                        Ok(Some(Adt122::C1))
                    }
                    Some(v6) => {
                        let a2923 = v6;
                        let a2924 = v2;
                        if equal(a2923, a2924) {
                            f904(f905(match v0.clone() {
                                Adt121::C0(_, _, h2925, _, _) => {
                                    h2925
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt121::C0(_, _, _, _, h2926) => {
                                    h2926
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt122::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt122::C3))
            }
        }
    }
}

pub fn f844(v0: Adt5) -> Adt162 {
    f906(f264(f846(), v0.clone()), f264(f847(), v0))
}

pub fn f845(v0: Adt5, v1: Adt116, v2: Adt116) -> Result<Adt17, Adt143> {
    let m2927 = v1;
    match m2927 {
        Adt116::C0 => {
            Err::<Adt17, Adt143>(Adt143::C2)
        }
        Adt116::C1(v3) => {
            let m2928 = v2;
            match m2928 {
                Adt116::C0 => {
                    Err::<Adt17, Adt143>(Adt143::C3)
                }
                Adt116::C1(v4) => {
                    f907(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f846() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f847() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f848(v0: Adt6) -> bool {
    let m2929 = f190(v0);
    match m2929 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f849(v0: Adt6) -> bool {
    let m2930 = f699(v0);
    match m2930 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f850(v0: Adt6) -> bool {
    let m2931 = f283(v0);
    match m2931 {
        None => {
            false
        }
        Some(v1) => {
            let a2935 = {
                let m2934 = {
                    let a2932 = v1;
                    let a2933 = 0i64;
                    compare(a2932, a2933)
                };
                match m2934 {
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
            bool_not(a2935)
        }
    }
}

pub fn f851(v0: Adt6) -> bool {
    let m2936 = f270(v0);
    match m2936 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f852(v0: Adt6) -> bool {
    let m2937 = f265(v0);
    match m2937 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f853(v0: Adt6) -> bool {
    let m2938 = f270(v0);
    match m2938 {
        None => {
            false
        }
        Some(v1) => {
            f908(v1)
        }
    }
}

pub fn f854(v0: Adt6) -> bool {
    let m2939 = f369(v0);
    match m2939 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f855(v0: Adt6) -> bool {
    let m2940 = f190(v0.clone());
    match m2940 {
        None => {
            let m2941 = f270(v0);
            match m2941 {
                None => {
                    false
                }
                Some(v2) => {
                    f909(v2)
                }
            }
        }
        Some(v1) => {
            f694(v1)
        }
    }
}

pub fn f856(v0: Adt6) -> bool {
    let m2942 = f270(v0);
    match m2942 {
        None => {
            false
        }
        Some(v1) => {
            let m2943 = v1;
            match m2943.uncons() {
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

pub fn f857(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2944 = f364(Str::lit("type"), v0);
    match m2944 {
        Adt116::C0 => {
            None::<Adt13>
        }
        Adt116::C1(v1) => {
            let m2945 = f190(v1);
            match m2945 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f910(v2)
                }
            }
        }
    }
}

pub fn f858(v0: List<u64>) -> R<List<u64>> {
    Ok(f406(f911(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f859(v0: List<u64>, p2948: List<List<u64>>) -> bool {
    let m2947 = p2948;
    {
        let mut m2947 = m2947;
        loop {
            let step = {
                let v1 = m2947;
                let m2946 = v1;
                match m2946.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f912(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2947 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f860(p2960: u64, p2961: i64, p2962: i64) -> R<bool> {
    let m2959 = (p2960, (p2961, p2962));
    {
        let mut m2959 = m2959;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2959;
                let m2949 = v0;
                if m2949 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2949 - 1;
                    let m2952 = {
                        let a2950 = v2;
                        let a2951 = 0i64;
                        compare(a2950, a2951)
                    };
                    if match m2952 {
                        Ordering::Less => {
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
                            let a2953 = v1;
                            let a2954 = 10i64;
                            let a2955 = 0i64;
                            int_rem(a2953, a2954, a2955)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2956 = v1;
                                let a2957 = 10i64;
                                let a2958 = 0i64;
                                int_quot(a2956, a2957, a2958)?
                            }, f368(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2959 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f861(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt163> {
    let a2963 = f913(v0.clone());
    let a2964 = f913(v1.clone());
    if nat_eq(a2963, a2964) {
        let m2965 = v0;
        match m2965 {
            Adt6::C0 => {
                Ok(Adt163::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2966 = f369(v1);
                match m2966 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v4) => {
                        let a2967 = v3;
                        let a2968 = v4;
                        if equal(a2967, a2968) {
                            Ok(Adt163::C1(v2))
                        } else {
                            Ok(Adt163::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2969 = f699(v1);
                match m2969 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v6) => {
                        if f914(f701(v5, v6)?) {
                            Ok(Adt163::C1(v2))
                        } else {
                            Ok(Adt163::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2970 = f190(v1);
                match m2970 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v8) => {
                        let a2971 = v7;
                        let a2972 = v8;
                        if equal(a2971, a2972) {
                            Ok(Adt163::C1(v2))
                        } else {
                            Ok(Adt163::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2973 = f270(v1);
                match m2973 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v10) => {
                        Ok(f915(f916(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2974 = f265(v1);
                match m2974 {
                    None => {
                        Ok(Adt163::C0)
                    }
                    Some(v12) => {
                        let a2976 = f705(v11.clone(), 0u64)?;
                        let a2977 = f705(match v12.clone() {
                            Adt5::C0(h2975) => {
                                h2975
                            }
                        }, 0u64)?;
                        if nat_eq(a2976, a2977) {
                            Ok(f915(f917(v11, match v12 {
                                Adt5::C0(h2978) => {
                                    h2978
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

pub fn f862(v0: Adt160, v1: Adt160) -> Adt160 {
    let m2979 = v0;
    match m2979 {
        Adt160::C0 => {
            f918(v1)
        }
        Adt160::C1 => {
            Adt160::C1
        }
        Adt160::C2 => {
            v1
        }
    }
}

pub fn f863(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt160> {
    let m2980 = f790(f919(v0, v1)?, f919(v2, v3)?);
    match m2980 {
        Adt160::C0 => {
            Ok(Adt160::C0)
        }
        Adt160::C1 => {
            f920(v0, f921(v0)?, v2, f921(v2)?)
        }
        Adt160::C2 => {
            Ok(Adt160::C2)
        }
    }
}

pub fn f864(v0: i64) -> R<i64> {
    let m2983 = {
        let a2981 = v0;
        let a2982 = 0i64;
        compare(a2981, a2982)
    };
    if match m2983 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2984 = v0;
        int_neg(a2984)
    } else {
        Ok(v0)
    }
}

pub fn f865(v0: u64, v1: u64) -> u64 {
    let a2985 = v0;
    let a2986 = v1;
    let a2987 = 0u64;
    nat_rem(a2985, a2986, a2987)
}

pub fn f866(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2990 = {
            let a2988 = v0;
            let a2989 = 4u64;
            nat_mul(a2988, a2989)?
        };
        let a2991 = f868(v1, 16u64);
        nat_add(a2990, a2991)?
    }, List::<u64>::nil()))
}

pub fn f867(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2994 = {
            let a2992 = v0;
            let a2993 = 4u64;
            nat_mul(a2992, a2993)?
        };
        let a2995 = f868(v1, 16u64);
        nat_add(a2994, a2995)?
    }, List::cons({
        let a2998 = {
            let a2996 = f865(v1, 16u64);
            let a2997 = 16u64;
            nat_mul(a2996, a2997)?
        };
        let a2999 = f868(v2, 4u64);
        nat_add(a2998, a2999)?
    }, List::<u64>::nil())))
}

pub fn f868(v0: u64, v1: u64) -> u64 {
    let a3000 = v0;
    let a3001 = v1;
    let a3002 = 0u64;
    nat_quot(a3000, a3001, a3002)
}

pub fn f869(v0: u64, v1: List<u64>) -> Adt161 {
    let a3003 = v0;
    let a3004 = 32u64;
    if nat_lt(a3003, a3004) {
        if f922(v0) {
            Adt161::C0(0u64, List::cons(v0, v1))
        } else {
            Adt161::C0(0u64, f870(v1))
        }
    } else {
        let a3005 = v0;
        let a3006 = 127u64;
        if nat_eq(a3005, a3006) {
            Adt161::C0(0u64, f870(v1))
        } else {
            let a3007 = v0;
            let a3008 = 194u64;
            if nat_eq(a3007, a3008) {
                Adt161::C0(1u64, v1)
            } else {
                let a3009 = v0;
                let a3010 = 226u64;
                if nat_eq(a3009, a3010) {
                    Adt161::C0(2u64, v1)
                } else {
                    Adt161::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f870(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f871(v0: List<Adt42>) -> List<Adt6> {
    f461(f923(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f872(p3013: List<Adt2>, p3014: List<Adt6>) -> List<Adt6> {
    let m3012 = (p3013, p3014);
    {
        let mut m3012 = m3012;
        loop {
            let step = {
                let (v0, v1) = m3012;
                let m3011 = v0;
                match m3011.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f924(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3012 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f873(v0: i64) -> u64 {
    let m3017 = {
        let a3015 = v0;
        let a3016 = 1i64;
        compare(a3015, a3016)
    };
    if match m3017 {
        Ordering::Less => {
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
        let m3020 = {
            let a3018 = v0;
            let a3019 = 2i64;
            compare(a3018, a3019)
        };
        if match m3020 {
            Ordering::Less => {
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
            let m3023 = {
                let a3021 = v0;
                let a3022 = 3i64;
                compare(a3021, a3022)
            };
            if match m3023 {
                Ordering::Less => {
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
                let m3026 = {
                    let a3024 = v0;
                    let a3025 = 4i64;
                    compare(a3024, a3025)
                };
                if match m3026 {
                    Ordering::Less => {
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
                    let m3029 = {
                        let a3027 = v0;
                        let a3028 = 5i64;
                        compare(a3027, a3028)
                    };
                    if match m3029 {
                        Ordering::Less => {
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
                        let m3032 = {
                            let a3030 = v0;
                            let a3031 = 6i64;
                            compare(a3030, a3031)
                        };
                        if match m3032 {
                            Ordering::Less => {
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
                            let m3035 = {
                                let a3033 = v0;
                                let a3034 = 7i64;
                                compare(a3033, a3034)
                            };
                            if match m3035 {
                                Ordering::Less => {
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
                                let m3038 = {
                                    let a3036 = v0;
                                    let a3037 = 8i64;
                                    compare(a3036, a3037)
                                };
                                if match m3038 {
                                    Ordering::Less => {
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
                                    let m3041 = {
                                        let a3039 = v0;
                                        let a3040 = 9i64;
                                        compare(a3039, a3040)
                                    };
                                    if match m3041 {
                                        Ordering::Less => {
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

pub fn f874(v0: Adt8) -> Adt6 {
    Adt6::C5(f378(Str::lit("listChanged"), match v0 {
        Adt8::C0(h3042) => {
            h3042
        }
    }))
}

pub fn f875(v0: Adt36) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt36::C0(h3043, _, _, _, _, _, _, _) => {
            h3043
        }
    })), {
        let a3061 = f720(Str::lit("title"), match v0.clone() {
            Adt36::C0(_, h3044, _, _, _, _, _, _) => {
                h3044
            }
        });
        let a3062 = {
            let a3059 = f721(match v0.clone() {
                Adt36::C0(_, _, h3045, _, _, _, _, _) => {
                    h3045
                }
            });
            let a3060 = {
                let a3057 = f720(Str::lit("description"), match v0.clone() {
                    Adt36::C0(_, _, _, h3046, _, _, _, _) => {
                        h3046
                    }
                });
                let a3058 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m3049 = match v0.clone() {
                        Adt36::C0(_, _, _, _, h3048, _, _, _) => {
                            h3048
                        }
                    };
                    match m3049 {
                        Adt5::C0(h3047) => {
                            h3047
                        }
                    }
                })), {
                    let a3055 = f762(Str::lit("outputSchema"), match v0.clone() {
                        Adt36::C0(_, _, _, _, _, h3050, _, _) => {
                            h3050
                        }
                    });
                    let a3056 = {
                        let a3053 = f925(match v0.clone() {
                            Adt36::C0(_, _, _, _, _, _, h3051, _) => {
                                h3051
                            }
                        });
                        let a3054 = f719(match v0 {
                            Adt36::C0(_, _, _, _, _, _, _, h3052) => {
                                h3052
                            }
                        });
                        append_list(a3053, a3054)
                    };
                    append_list(a3055, a3056)
                });
                append_list(a3057, a3058)
            };
            append_list(a3059, a3060)
        };
        append_list(a3061, a3062)
    }))
}

pub fn f876(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a3086 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h3063, _, _, _, _, _, _, _, _) => {
                h3063
            }
        })), List::<(Str, Adt6)>::nil());
        let a3087 = {
            let a3084 = f720(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h3064, _, _, _, _, _, _, _) => {
                    h3064
                }
            });
            let a3085 = {
                let a3082 = f721(match v0.clone() {
                    Adt45::C0(_, _, h3065, _, _, _, _, _, _) => {
                        h3065
                    }
                });
                let a3083 = {
                    let a3080 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h3066, _, _, _, _, _) => {
                            h3066
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a3081 = {
                        let a3078 = f720(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h3067, _, _, _, _) => {
                                h3067
                            }
                        });
                        let a3079 = {
                            let a3076 = f720(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h3068, _, _, _) => {
                                    h3068
                                }
                            });
                            let a3077 = {
                                let a3074 = f718(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h3069, _, _) => {
                                        h3069
                                    }
                                });
                                let a3075 = {
                                    let a3072 = f722(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h3070, _) => {
                                            h3070
                                        }
                                    })?;
                                    let a3073 = f719(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h3071) => {
                                            h3071
                                        }
                                    });
                                    append_list(a3072, a3073)
                                };
                                append_list(a3074, a3075)
                            };
                            append_list(a3076, a3077)
                        };
                        append_list(a3078, a3079)
                    };
                    append_list(a3080, a3081)
                };
                append_list(a3082, a3083)
            };
            append_list(a3084, a3085)
        };
        append_list(a3086, a3087)
    }))
}

pub fn f877(v0: Adt55) -> Adt6 {
    Adt6::C5({
        let a3108 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt55::C0(h3088, _, _, _, _, _, _, _) => {
                h3088
            }
        })), List::<(Str, Adt6)>::nil());
        let a3109 = {
            let a3106 = f720(Str::lit("title"), match v0.clone() {
                Adt55::C0(_, h3089, _, _, _, _, _, _) => {
                    h3089
                }
            });
            let a3107 = {
                let a3104 = f721(match v0.clone() {
                    Adt55::C0(_, _, h3090, _, _, _, _, _) => {
                        h3090
                    }
                });
                let a3105 = {
                    let a3102 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt55::C0(_, _, _, h3091, _, _, _, _) => {
                            h3091
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a3103 = {
                        let a3100 = f720(Str::lit("description"), match v0.clone() {
                            Adt55::C0(_, _, _, _, h3092, _, _, _) => {
                                h3092
                            }
                        });
                        let a3101 = {
                            let a3098 = f720(Str::lit("mimeType"), match v0.clone() {
                                Adt55::C0(_, _, _, _, _, h3093, _, _) => {
                                    h3093
                                }
                            });
                            let a3099 = {
                                let a3096 = f718(match v0.clone() {
                                    Adt55::C0(_, _, _, _, _, _, h3094, _) => {
                                        h3094
                                    }
                                });
                                let a3097 = f719(match v0 {
                                    Adt55::C0(_, _, _, _, _, _, _, h3095) => {
                                        h3095
                                    }
                                });
                                append_list(a3096, a3097)
                            };
                            append_list(a3098, a3099)
                        };
                        append_list(a3100, a3101)
                    };
                    append_list(a3102, a3103)
                };
                append_list(a3104, a3105)
            };
            append_list(a3106, a3107)
        };
        append_list(a3108, a3109)
    })
}

pub fn f878(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a3125 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h3110, _, _, _, _, _) => {
                h3110
            }
        })), List::<(Str, Adt6)>::nil());
        let a3126 = {
            let a3123 = f720(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h3111, _, _, _, _) => {
                    h3111
                }
            });
            let a3124 = {
                let a3121 = f721(match v0.clone() {
                    Adt57::C0(_, _, h3112, _, _, _) => {
                        h3112
                    }
                });
                let a3122 = {
                    let a3119 = f720(Str::lit("description"), match v0.clone() {
                        Adt57::C0(_, _, _, h3113, _, _) => {
                            h3113
                        }
                    });
                    let a3120 = {
                        let a3117 = {
                            let m3115 = match v0.clone() {
                                Adt57::C0(_, _, _, _, h3114, _) => {
                                    h3114
                                }
                            };
                            match m3115 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f926(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a3118 = f719(match v0 {
                            Adt57::C0(_, _, _, _, _, h3116) => {
                                h3116
                            }
                        });
                        append_list(a3117, a3118)
                    };
                    append_list(a3119, a3120)
                };
                append_list(a3121, a3122)
            };
            append_list(a3123, a3124)
        };
        append_list(a3125, a3126)
    })
}

pub fn f879(v0: Adt60) -> R<Option<Adt6>> {
    let m3128 = f538(match v0.clone() {
        Adt60::C0(_, h3127) => {
            h3127
        }
    })?;
    match m3128 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f927(match v0 {
                Adt60::C0(h3129, _) => {
                    h3129
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f880(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f377(Str::lit("params"), v1)))
}

pub fn f881(v0: Adt66) -> Adt6 {
    let m3130 = v0;
    match m3130 {
        Adt66::C0(v1) => {
            f928(v1)
        }
        Adt66::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt69::C0(h3131, _) => {
                    h3131
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt69::C0(_, h3132) => {
                    h3132
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f882(v0: Adt70) -> R<Option<Adt6>> {
    let m3134 = f929(match v0.clone() {
        Adt70::C0(h3133, _, _, _, _, _, _, _, _, _) => {
            h3133
        }
    })?;
    match m3134 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a3136 = f930(v0.clone());
                let a3137 = List::cons((Str::lit("maxTokens"), f152(match v0.clone() {
                    Adt70::C0(_, _, _, _, _, h3135, _, _, _, _) => {
                        h3135
                    }
                })?), f931(v0));
                append_list(a3136, a3137)
            }))))
        }
    }
}

pub fn f883(p3143: List<(Str, Adt65)>, p3144: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m3142 = (p3143, p3144);
    {
        let mut m3142 = m3142;
        loop {
            let step = {
                let (v0, v1) = m3142;
                let m3138 = v0;
                match m3138.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m3140 = f932({
                            let (_, h3139) = v2;
                            h3139
                        });
                        match m3140 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h3141, _) = v2;
                                    h3141
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f884(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f885(v0: Adt85) -> Option<Adt6> {
    let m3146 = f647(match v0.clone() {
        Adt85::C0(h3145, _, _) => {
            h3145
        }
    });
    match m3146 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m3148 = f933(match v0.clone() {
                Adt85::C0(_, _, h3147) => {
                    h3147
                }
            });
            match m3148 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f884(match v0 {
                        Adt85::C0(_, h3149, _) => {
                            h3149
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f886(v0: List<u64>) -> List<u64> {
    let a3152 = f934();
    let a3153 = {
        let a3150 = v0;
        let a3151 = f935();
        append_list(a3150, a3151)
    };
    append_list(a3152, a3153)
}

pub fn f887(p3160: List<u64>) -> bool {
    let m3159 = p3160;
    {
        let mut m3159 = m3159;
        loop {
            let step = {
                let v0 = m3159;
                let m3154 = v0;
                match m3154.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a3155 = v1;
                        let a3156 = 48u64;
                        if if nat_eq(a3155, a3156) {
                            true
                        } else {
                            let a3157 = v1;
                            let a3158 = 46u64;
                            nat_eq(a3157, a3158)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f888(v0: Adt150) -> bool {
    let m3161 = v0;
    match m3161 {
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

pub fn f889(v0: Adt15) -> Adt150 {
    let m3162 = v0;
    match m3162 {
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

pub fn f890(v0: Adt0, v1: Adt33) -> Adt139 {
    Adt139::C0(v1, Adt140::C0(Some(match v0 {
        Adt0::C0(h3163, _, _, _, _, _, _) => {
            h3163
        }
    }), None::<Adt5>), List::<Adt141>::nil())
}

pub fn f891(v0: Adt0) -> Adt34 {
    Adt34::C0(match v0.clone() {
        Adt0::C0(_, _, _, h3164, _, _, _) => {
            h3164
        }
    }, f417(), match v0.clone() {
        Adt0::C0(_, _, h3165, _, _, _, _) => {
            h3165
        }
    }, match v0 {
        Adt0::C0(_, h3166, _, _, _, _, _) => {
            h3166
        }
    })
}

pub fn f892(v0: Adt0, v1: Adt15, v2: Result<Adt33, Adt90>) -> Result<Adt139, Adt142> {
    let m3167 = v2;
    match m3167 {
        Ok(v4) => {
            if f936(f889(v1.clone()), v4.clone()) {
                f937(v0, v1, v4)
            } else {
                Err::<Adt139, Adt142>(Adt142::C4(f938()))
            }
        }
        Err(v3) => {
            Err::<Adt139, Adt142>(Adt142::C4(v3))
        }
    }
}

pub fn f893(v0: Bytes, v1: u64, v2: u64, v3: Fn5, v4: Adt15) -> R<Result<Adt33, Adt90>> {
    f939(v0.clone(), v1, v2, v3, v4.clone(), f940(v0, v4)?)
}

pub fn f894(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt143> {
    let m3168 = f264(v0.clone(), v1);
    match m3168 {
        Adt116::C0 => {
            Ok::<Option<Str>, Adt143>(None::<Str>)
        }
        Adt116::C1(v2) => {
            let m3169 = f190(v2);
            match m3169 {
                None => {
                    Err::<Option<Str>, Adt143>(Adt143::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt143>(Some(v3))
                }
            }
        }
    }
}

pub fn f895(v0: Str, v1: Adt5) -> Result<Str, Adt143> {
    let m3170 = f264(v0.clone(), v1);
    match m3170 {
        Adt116::C0 => {
            Err::<Str, Adt143>(Adt143::C7(v0))
        }
        Adt116::C1(v2) => {
            let m3171 = f190(v2);
            match m3171 {
                None => {
                    Err::<Str, Adt143>(Adt143::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt143>(v3)
                }
            }
        }
    }
}

pub fn f896(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt143> {
    let m3172 = f264(v0.clone(), v1);
    match m3172 {
        Adt116::C0 => {
            Ok::<Option<Adt5>, Adt143>(None::<Adt5>)
        }
        Adt116::C1(v2) => {
            let m3173 = f265(v2);
            match m3173 {
                None => {
                    Err::<Option<Adt5>, Adt143>(Adt143::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt143>(Some(v3))
                }
            }
        }
    }
}

pub fn f897(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt143> {
    let m3174 = f264(v0.clone(), v1);
    match m3174 {
        Adt116::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt143>(None::<List<(Str, Str)>>)
        }
        Adt116::C1(v2) => {
            let m3175 = f265(v2);
            match m3175 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt143>(Adt143::C8(v0))
                }
                Some(v3) => {
                    let m3177 = f941(v0, match v3 {
                        Adt5::C0(h3176) => {
                            h3176
                        }
                    });
                    match m3177 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt143>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt143>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f898(v0: Str, v1: Adt5) -> Result<Adt5, Adt143> {
    let m3178 = f264(v0.clone(), v1);
    match m3178 {
        Adt116::C0 => {
            Err::<Adt5, Adt143>(Adt143::C7(v0))
        }
        Adt116::C1(v2) => {
            let m3179 = f265(v2);
            match m3179 {
                None => {
                    Err::<Adt5, Adt143>(Adt143::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt143>(v3)
                }
            }
        }
    }
}

pub fn f899(v0: Adt5) -> Result<Adt26, Adt143> {
    let m3180 = f895(Str::lit("type"), v0.clone());
    match m3180 {
        Ok(v1) => {
            let a3181 = v1.clone();
            let a3182 = Str::lit("ref/prompt");
            if equal(a3181, a3182) {
                let m3183 = f895(Str::lit("name"), v0.clone());
                match m3183 {
                    Ok(v2) => {
                        let m3184 = f894(Str::lit("title"), v0);
                        match m3184 {
                            Ok(v3) => {
                                Ok::<Adt26, Adt143>(Adt26::C0(Adt27::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt26, Adt143>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt26, Adt143>(v5)
                    }
                }
            } else {
                let a3185 = v1;
                let a3186 = Str::lit("ref/resource");
                if equal(a3185, a3186) {
                    let m3187 = f895(Str::lit("uri"), v0);
                    match m3187 {
                        Ok(v6) => {
                            Ok::<Adt26, Adt143>(Adt26::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt26, Adt143>(v7)
                        }
                    }
                } else {
                    Err::<Adt26, Adt143>(Adt143::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt26, Adt143>(v8)
        }
    }
}

pub fn f900(v0: Adt5) -> Result<Adt28, Adt143> {
    let m3188 = f895(Str::lit("name"), v0.clone());
    match m3188 {
        Ok(v1) => {
            let m3189 = f895(Str::lit("value"), v0);
            match m3189 {
                Ok(v2) => {
                    Ok::<Adt28, Adt143>(Adt28::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt28, Adt143>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt28, Adt143>(v4)
        }
    }
}

pub fn f901(v0: Adt5) -> Result<Option<Adt29>, Adt143> {
    let m3190 = f264(Str::lit("context"), v0);
    match m3190 {
        Adt116::C0 => {
            Ok::<Option<Adt29>, Adt143>(None::<Adt29>)
        }
        Adt116::C1(v1) => {
            let m3191 = f265(v1);
            match m3191 {
                None => {
                    Err::<Option<Adt29>, Adt143>(Adt143::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m3192 = f897(Str::lit("arguments"), v2);
                    match m3192 {
                        Ok(v3) => {
                            Ok::<Option<Adt29>, Adt143>(Some(Adt29::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt29>, Adt143>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f902(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt142> {
    if v0 {
        None::<Adt142>
    } else {
        Some(Adt142::C1(v1, v2))
    }
}

pub fn f903(v0: Str, p3197: List<Str>) -> bool {
    let m3196 = p3197;
    {
        let mut m3196 = m3196;
        loop {
            let step = {
                let v1 = m3196;
                let m3193 = v1;
                match m3193.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3194 = v2;
                        let a3195 = v0.clone();
                        if equal(a3194, a3195) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
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

pub fn f904(v0: Option<Adt122>, v1: List<Adt124>, v2: Adt5) -> R<Option<Adt122>> {
    let m3198 = v0;
    match m3198 {
        None => {
            f942(v1, f943(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f905(v0: Option<Str>, v1: Adt156, v2: Adt5) -> Option<Adt122> {
    let m3199 = v1.clone();
    match m3199 {
        Adt156::C0 => {
            None::<Adt122>
        }
        Adt156::C1 => {
            f944(v0, f945(v1, v2))
        }
        Adt156::C2 => {
            f944(v0, f945(v1, v2))
        }
        Adt156::C3 => {
            f944(v0, f945(v1, v2))
        }
    }
}

pub fn f906(v0: Adt116, v1: Adt116) -> Adt162 {
    let m3200 = v0;
    match m3200 {
        Adt116::C0 => {
            Adt162::C1
        }
        Adt116::C1(_) => {
            let m3201 = v1;
            match m3201 {
                Adt116::C0 => {
                    Adt162::C2
                }
                Adt116::C1(_) => {
                    Adt162::C0
                }
            }
        }
    }
}

pub fn f907(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt17, Adt143> {
    let m3202 = f190(v1);
    match m3202 {
        None => {
            Err::<Adt17, Adt143>(Adt143::C5)
        }
        Some(v3) => {
            let m3203 = f946(v2);
            match m3203 {
                None => {
                    Err::<Adt17, Adt143>(Adt143::C6)
                }
                Some(v4) => {
                    let m3204 = f896(f947(), v0.clone());
                    match m3204 {
                        Ok(v6) => {
                            let m3205 = f948(v0.clone());
                            match m3205 {
                                Ok(v8) => {
                                    let m3206 = f949(v0.clone());
                                    match m3206 {
                                        Ok(v10) => {
                                            Ok::<Adt17, Adt143>(Adt17::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt17, Adt143>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt17, Adt143>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt17, Adt143>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f908(p3210: List<Adt6>) -> bool {
    let m3209 = p3210;
    {
        let mut m3209 = m3209;
        loop {
            let step = {
                let v0 = m3209;
                let m3207 = v0;
                match m3207.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3208 = f190(v1);
                        match m3208 {
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
                Ok(next) => { m3209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f909(p3214: List<Adt6>) -> bool {
    let m3213 = p3214;
    {
        let mut m3213 = m3213;
        loop {
            let step = {
                let v0 = m3213;
                let m3211 = v0;
                match m3211.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3212 = f190(v1);
                        match m3212 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f694(v3) {
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
                Ok(next) => { m3213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f910(v0: Str) -> Option<Adt13> {
    let a3215 = v0.clone();
    let a3216 = Str::lit("string");
    if equal(a3215, a3216) {
        Some(Adt13::C0)
    } else {
        let a3217 = v0.clone();
        let a3218 = Str::lit("integer");
        if equal(a3217, a3218) {
            Some(Adt13::C1)
        } else {
            let a3219 = v0;
            let a3220 = Str::lit("boolean");
            if equal(a3219, a3220) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f911(p3229: List<u64>, p3230: List<u64>) -> R<List<u64>> {
    let m3228 = (p3229, p3230);
    {
        let mut m3228 = m3228;
        loop {
            let step = {
                let (v0, v1) = m3228;
                let m3221 = v0;
                match m3221.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a3222 = 65u64;
                        let a3223 = v2;
                        if if nat_le(a3222, a3223) {
                            let a3224 = v2;
                            let a3225 = 90u64;
                            nat_le(a3224, a3225)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a3226 = v2;
                                let a3227 = 32u64;
                                nat_add(a3226, a3227)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f912(p3237: List<u64>, p3238: List<u64>) -> bool {
    let m3236 = (p3237, p3238);
    {
        let mut m3236 = m3236;
        loop {
            let step = {
                let (v0, v1) = m3236;
                let m3231 = v0;
                match m3231.uncons() {
                    None => {
                        let m3232 = v1;
                        match m3232.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m3233 = v1;
                        match m3233.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a3234 = v4;
                                let a3235 = v6;
                                if nat_eq(a3234, a3235) {
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
                Ok(next) => { m3236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f913(v0: Adt6) -> u64 {
    let m3239 = v0;
    match m3239 {
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

pub fn f914(v0: Adt160) -> bool {
    let m3240 = v0;
    match m3240 {
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

pub fn f915(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt163 {
    let m3241 = v0;
    match m3241 {
        None => {
            Adt163::C0
        }
        Some(v2) => {
            Adt163::C1({
                let a3242 = v2;
                let a3243 = v1;
                append_list(a3242, a3243)
            })
        }
    }
}

pub fn f916(p3248: List<Adt6>, p3249: List<Adt6>, p3250: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3247 = (p3248, (p3249, p3250));
    {
        let mut m3247 = m3247;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3247;
                let m3244 = v0;
                match m3244.uncons() {
                    None => {
                        let m3245 = v1;
                        match m3245.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m3246 = v1;
                        match m3246.uncons() {
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
                Ok(next) => { m3247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f917(p3256: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p3257: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3255 = (p3256, p3257);
    {
        let mut m3255 = m3255;
        loop {
            let step = {
                let (v0, v2) = m3255;
                let m3251 = v0;
                match m3251.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m3253 = f364({
                            let (h3252, _) = v3;
                            h3252
                        }, v1.clone());
                        match m3253 {
                            Adt116::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt116::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h3254) = v3;
                                    h3254
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f918(v0: Adt160) -> Adt160 {
    let m3258 = v0;
    match m3258 {
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

pub fn f919(v0: i64, v1: i64) -> R<i64> {
    f368(f269(f921(v0)?, 0i64)?, v1)
}

pub fn f920(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt160> {
    let a3259 = v1;
    let a3260 = v3;
    if nat_lt(a3259, a3260) {
        Ok(f790(f950({
            let a3261 = v3;
            let a3262 = v1;
            nat_sub(a3261, a3262)
        }, v0)?, v2))
    } else {
        let a3263 = v3;
        let a3264 = v1;
        if nat_lt(a3263, a3264) {
            Ok(f790(v0, f950({
                let a3265 = v1;
                let a3266 = v3;
                nat_sub(a3265, a3266)
            }, v2)?))
        } else {
            Ok(f790(v0, v2))
        }
    }
}

pub fn f921(v0: i64) -> R<u64> {
    f951(20u64, v0, 0u64)
}

pub fn f922(v0: u64) -> bool {
    let a3267 = v0;
    let a3268 = 9u64;
    if nat_eq(a3267, a3268) {
        true
    } else {
        let a3269 = v0;
        let a3270 = 10u64;
        if nat_eq(a3269, a3270) {
            true
        } else {
            let a3271 = v0;
            let a3272 = 13u64;
            nat_eq(a3271, a3272)
        }
    }
}

pub fn f923(p3275: List<Adt42>, p3276: List<Adt6>) -> List<Adt6> {
    let m3274 = (p3275, p3276);
    {
        let mut m3274 = m3274;
        loop {
            let step = {
                let (v0, v1) = m3274;
                let m3273 = v0;
                match m3273.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt42>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f927(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f924(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h3277, _, _, _) => {
            h3277
        }
    })), {
        let a3283 = f720(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h3278, _, _) => {
                h3278
            }
        });
        let a3284 = {
            let a3281 = f952(match v0.clone() {
                Adt2::C0(_, _, h3279, _) => {
                    h3279
                }
            });
            let a3282 = f953(match v0 {
                Adt2::C0(_, _, _, h3280) => {
                    h3280
                }
            });
            append_list(a3281, a3282)
        };
        append_list(a3283, a3284)
    }))
}

pub fn f925(v0: Option<Adt37>) -> List<(Str, Adt6)> {
    let m3285 = v0;
    match m3285 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a3297 = f720(Str::lit("title"), match v1.clone() {
                    Adt37::C0(h3286, _, _, _, _) => {
                        h3286
                    }
                });
                let a3298 = {
                    let a3295 = f378(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt37::C0(_, h3287, _, _, _) => {
                            h3287
                        }
                    });
                    let a3296 = {
                        let a3293 = f378(Str::lit("destructiveHint"), match v1.clone() {
                            Adt37::C0(_, _, h3288, _, _) => {
                                h3288
                            }
                        });
                        let a3294 = {
                            let a3291 = f378(Str::lit("idempotentHint"), match v1.clone() {
                                Adt37::C0(_, _, _, h3289, _) => {
                                    h3289
                                }
                            });
                            let a3292 = f378(Str::lit("openWorldHint"), match v1 {
                                Adt37::C0(_, _, _, _, h3290) => {
                                    h3290
                                }
                            });
                            append_list(a3291, a3292)
                        };
                        append_list(a3293, a3294)
                    };
                    append_list(a3295, a3296)
                };
                append_list(a3297, a3298)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f926(v0: List<Adt58>) -> List<Adt6> {
    f461(f954(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f927(v0: Adt42) -> Str {
    let m3299 = v0;
    match m3299 {
        Adt42::C0 => {
            Str::lit("user")
        }
        Adt42::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f928(v0: Adt67) -> Adt6 {
    Adt6::C5({
        let a3306 = {
            let m3301 = match v0.clone() {
                Adt67::C0(h3300, _, _) => {
                    h3300
                }
            };
            match m3301 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a3307 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt67::C0(_, h3302, _) => {
                h3302
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m3305 = match v0 {
                Adt67::C0(_, _, h3304) => {
                    h3304
                }
            };
            match m3305 {
                Adt5::C0(h3303) => {
                    h3303
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a3306, a3307)
    })
}

pub fn f929(v0: List<Adt71>) -> R<Option<List<Adt6>>> {
    let m3308 = f955(v0, List::<Adt6>::nil())?;
    match m3308 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f461(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f930(v0: Adt70) -> List<(Str, Adt6)> {
    let a3319 = {
        let m3310 = match v0.clone() {
            Adt70::C0(_, h3309, _, _, _, _, _, _, _, _) => {
                h3309
            }
        };
        match m3310 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f956(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a3320 = {
        let a3317 = f720(Str::lit("systemPrompt"), match v0.clone() {
            Adt70::C0(_, _, h3311, _, _, _, _, _, _, _) => {
                h3311
            }
        });
        let a3318 = {
            let a3315 = {
                let m3313 = match v0.clone() {
                    Adt70::C0(_, _, _, h3312, _, _, _, _, _, _) => {
                        h3312
                    }
                };
                match m3313 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f957(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3316 = f958(Str::lit("temperature"), match v0 {
                Adt70::C0(_, _, _, _, h3314, _, _, _, _, _) => {
                    h3314
                }
            });
            append_list(a3315, a3316)
        };
        append_list(a3317, a3318)
    };
    append_list(a3319, a3320)
}

pub fn f931(v0: Adt70) -> List<(Str, Adt6)> {
    let a3332 = {
        let m3322 = match v0.clone() {
            Adt70::C0(_, _, _, _, _, _, h3321, _, _, _) => {
                h3321
            }
        };
        match m3322 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f729(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a3333 = {
        let a3330 = f762(Str::lit("metadata"), match v0.clone() {
            Adt70::C0(_, _, _, _, _, _, _, h3323, _, _) => {
                h3323
            }
        });
        let a3331 = {
            let a3328 = {
                let m3325 = match v0.clone() {
                    Adt70::C0(_, _, _, _, _, _, _, _, h3324, _) => {
                        h3324
                    }
                };
                match m3325 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f732(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3329 = {
                let m3327 = match v0 {
                    Adt70::C0(_, _, _, _, _, _, _, _, _, h3326) => {
                        h3326
                    }
                };
                match m3327 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f959(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a3328, a3329)
        };
        append_list(a3330, a3331)
    };
    append_list(a3332, a3333)
}

pub fn f932(v0: Adt65) -> Option<Adt6> {
    let m3334 = v0;
    match m3334 {
        Adt65::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f960(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt65::C1(_) => {
            None::<Adt6>
        }
        Adt65::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m3335 = v3;
                match m3335 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h3336) => {
                                h3336
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f933(v0: Adt86) -> Option<Adt6> {
    let m3338 = f816(match v0.clone() {
        Adt86::C0(_, h3337, _, _) => {
            h3337
        }
    });
    match m3338 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f884(match v0.clone() {
                Adt86::C0(h3339, _, _, _) => {
                    h3339
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f884(match v0.clone() {
                Adt86::C0(_, _, h3340, _) => {
                    h3340
                }
            })), List::cons((Str::lit("outcome"), f961(match v0 {
                Adt86::C0(_, _, _, h3341) => {
                    h3341
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f934() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f935() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f936(v0: Adt150, v1: Adt33) -> bool {
    let m3342 = v1;
    match m3342 {
        Adt33::C0(_) => {
            f962(v0, Adt150::C0)
        }
        Adt33::C1(_) => {
            f962(v0, Adt150::C1)
        }
        Adt33::C2(_) => {
            f962(v0, Adt150::C2)
        }
        Adt33::C3(_) => {
            f962(v0, Adt150::C2)
        }
        Adt33::C4(_) => {
            f962(v0, Adt150::C3)
        }
        Adt33::C5(_) => {
            f962(v0, Adt150::C4)
        }
        Adt33::C6(_) => {
            f962(v0, Adt150::C5)
        }
        Adt33::C7(_) => {
            f962(v0, Adt150::C6)
        }
        Adt33::C8(_) => {
            f962(v0, Adt150::C7)
        }
        Adt33::C9(_) => {
            f962(v0, Adt150::C8)
        }
        Adt33::C10(_) => {
            f962(v0, Adt150::C9)
        }
        Adt33::C11 => {
            if f962(v0.clone(), Adt150::C10) {
                true
            } else {
                f962(v0, Adt150::C11)
            }
        }
        Adt33::C12(_) => {
            f963(v0)
        }
        Adt33::C13(_) => {
            f964(v0)
        }
        Adt33::C14(v15) => {
            let m3344 = match v15 {
                Adt88::C0(_, h3343, _) => {
                    h3343
                }
            };
            match m3344 {
                Adt89::C0 => {
                    f962(v0, Adt150::C9)
                }
                Adt89::C1 => {
                    if f962(v0.clone(), Adt150::C10) {
                        true
                    } else {
                        f962(v0, Adt150::C11)
                    }
                }
            }
        }
    }
}

pub fn f937(v0: Adt0, v1: Adt15, v2: Adt33) -> Result<Adt139, Adt142> {
    let m3345 = v2.clone();
    match m3345 {
        Adt33::C0(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C1(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C2(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C3(v9) => {
            Ok::<Adt139, Adt142>(f969(v0, f966(v1), v9))
        }
        Adt33::C4(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C5(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C6(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C7(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C8(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C9(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C10(_) => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C11 => {
            Ok::<Adt139, Adt142>(f890(v0, v2))
        }
        Adt33::C12(v3) => {
            f965(v0, f966(v1), v3)
        }
        Adt33::C13(v4) => {
            f967(v0, f966(v1), v4)
        }
        Adt33::C14(v5) => {
            f968(v0, f966(v1), v5)
        }
    }
}

pub fn f938() -> Adt90 {
    Adt90::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f939(v0: Bytes, v1: u64, v2: u64, v3: Fn5, v4: Adt15, v5: Result<Adt15, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m3346 = v5;
    match m3346 {
        Ok(v7) => {
            f970(v0, v4, f971(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt33, Adt90>(v6))
        }
    }
}

pub fn f940(v0: Bytes, v1: Adt15) -> R<Result<Adt15, Adt90>> {
    let m3347 = v1.clone();
    match m3347 {
        Adt15::C0(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C1(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C2(v2) => {
            f972(v0, v2)
        }
        Adt15::C3(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C4(v3) => {
            f973(v0, v3)
        }
        Adt15::C5(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C6(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C7(v4) => {
            f974(v0, v4)
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

pub fn f941(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt143> {
    let m3348 = f975(v0, v1, List::<(Str, Str)>::nil());
    match m3348 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt143>(f976(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt143>(v3)
        }
    }
}

pub fn f942(p3352: List<Adt124>, v1: Adt5) -> R<Option<Adt122>> {
    let m3351 = p3352;
    {
        let mut m3351 = m3351;
        loop {
            let step = {
                let v0 = m3351;
                let m3349 = v0;
                match m3349.uncons() {
                    None => {
                        Err::<List<Adt124>, R<Option<Adt122>>>(Ok(None::<Adt122>))
                    }
                    Some((v2, v3)) => {
                        let m3350 = f977(v2, v1.clone())?;
                        match m3350 {
                            None => {
                                Ok::<List<Adt124>, R<Option<Adt122>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt124>, R<Option<Adt122>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f943(v0: Adt5) -> Adt5 {
    let m3353 = f264(Str::lit("arguments"), v0);
    match m3353 {
        Adt116::C0 => {
            f27()
        }
        Adt116::C1(v1) => {
            let m3354 = f265(v1);
            match m3354 {
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

pub fn f944(v0: Option<Str>, v1: Option<Str>) -> Option<Adt122> {
    let m3355 = v0;
    match m3355 {
        None => {
            Some(Adt122::C2)
        }
        Some(v2) => {
            let m3356 = v1;
            match m3356 {
                None => {
                    None::<Adt122>
                }
                Some(v3) => {
                    let a3357 = v2;
                    let a3358 = v3;
                    if equal(a3357, a3358) {
                        None::<Adt122>
                    } else {
                        Some(Adt122::C5)
                    }
                }
            }
        }
    }
}

pub fn f945(v0: Adt156, v1: Adt5) -> Option<Str> {
    let m3359 = v0;
    match m3359 {
        Adt156::C0 => {
            None::<Str>
        }
        Adt156::C1 => {
            let m3360 = f264(Str::lit("name"), v1);
            match m3360 {
                Adt116::C0 => {
                    None::<Str>
                }
                Adt116::C1(v2) => {
                    f190(v2)
                }
            }
        }
        Adt156::C2 => {
            let m3361 = f264(Str::lit("uri"), v1);
            match m3361 {
                Adt116::C0 => {
                    None::<Str>
                }
                Adt116::C1(v3) => {
                    f190(v3)
                }
            }
        }
        Adt156::C3 => {
            let m3362 = f264(Str::lit("taskId"), v1);
            match m3362 {
                Adt116::C0 => {
                    None::<Str>
                }
                Adt116::C1(v4) => {
                    f190(v4)
                }
            }
        }
    }
}

pub fn f946(v0: Adt6) -> Option<Adt18> {
    let m3363 = f265(v0);
    match m3363 {
        None => {
            None::<Adt18>
        }
        Some(v1) => {
            let m3364 = f978(Str::lit("experimental"), v1.clone());
            match m3364 {
                None => {
                    None::<Adt18>
                }
                Some(v2) => {
                    let m3365 = f978(Str::lit("roots"), v1.clone());
                    match m3365 {
                        None => {
                            None::<Adt18>
                        }
                        Some(v3) => {
                            let m3366 = f978(Str::lit("sampling"), v1.clone());
                            match m3366 {
                                None => {
                                    None::<Adt18>
                                }
                                Some(v4) => {
                                    let m3367 = f978(Str::lit("elicitation"), v1.clone());
                                    match m3367 {
                                        None => {
                                            None::<Adt18>
                                        }
                                        Some(v5) => {
                                            let m3368 = f978(Str::lit("extensions"), v1);
                                            match m3368 {
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

pub fn f947() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f948(v0: Adt5) -> Result<Option<Adt19>, Adt143> {
    let m3369 = f894(f979(), v0);
    match m3369 {
        Ok(v2) => {
            let m3370 = v2;
            match m3370 {
                None => {
                    Ok::<Option<Adt19>, Adt143>(None::<Adt19>)
                }
                Some(v3) => {
                    let m3371 = f980(v3);
                    match m3371 {
                        Adt164::C0 => {
                            Err::<Option<Adt19>, Adt143>(Adt143::C8(f979()))
                        }
                        Adt164::C1(v4) => {
                            Ok::<Option<Adt19>, Adt143>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt19>, Adt143>(v1)
        }
    }
}

pub fn f949(v0: Adt5) -> Result<Option<Adt20>, Adt143> {
    let m3372 = f264(f981(), v0);
    match m3372 {
        Adt116::C0 => {
            Ok::<Option<Adt20>, Adt143>(None::<Adt20>)
        }
        Adt116::C1(v1) => {
            let m3373 = f190(v1.clone());
            match m3373 {
                None => {
                    let m3374 = f283(v1);
                    match m3374 {
                        None => {
                            Err::<Option<Adt20>, Adt143>(Adt143::C8(f981()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt20>, Adt143>(Some(Adt20::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt20>, Adt143>(Some(Adt20::C0(v2)))
                }
            }
        }
    }
}

pub fn f950(p3379: u64, p3380: i64) -> R<i64> {
    let m3378 = (p3379, p3380);
    {
        let mut m3378 = m3378;
        loop {
            let step = {
                let (v0, v1) = m3378;
                let m3375 = v0;
                if m3375 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3375 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3376 = v1;
                        let a3377 = 10i64;
                        int_mul(a3376, a3377)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f951(p3391: u64, p3392: i64, p3393: u64) -> R<u64> {
    let m3390 = (p3391, (p3392, p3393));
    {
        let mut m3390 = m3390;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3390;
                let m3381 = v0;
                if m3381 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3381 - 1;
                    let v4: u64 = {
                        let a3382 = v2;
                        let a3383 = 1u64;
                        nat_add(a3382, a3383)?
                    };
                    let m3386 = {
                        let a3384 = v1;
                        let a3385 = 10i64;
                        compare(a3384, a3385)
                    };
                    if match m3386 {
                        Ordering::Less => {
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
                            let a3387 = v1;
                            let a3388 = 10i64;
                            let a3389 = 0i64;
                            int_quot(a3387, a3388, a3389)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f952(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3394 = v0;
    match m3394 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f667(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f953(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3395 = v0;
    match m3395 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f982(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f954(p3398: List<Adt58>, p3399: List<Adt6>) -> List<Adt6> {
    let m3397 = (p3398, p3399);
    {
        let mut m3397 = m3397;
        loop {
            let step = {
                let (v0, v1) = m3397;
                let m3396 = v0;
                match m3396.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f983(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3397 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f955(p3403: List<Adt71>, p3404: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3402 = (p3403, p3404);
    {
        let mut m3402 = m3402;
        loop {
            let step = {
                let (v0, v1) = m3402;
                let m3400 = v0;
                match m3400.uncons() {
                    None => {
                        Err::<(List<Adt71>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3401 = f984(v2)?;
                        match m3401 {
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
                Ok(next) => { m3402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f956(v0: Adt76) -> Adt6 {
    Adt6::C5({
        let a3414 = {
            let m3406 = match v0.clone() {
                Adt76::C0(h3405, _, _, _) => {
                    h3405
                }
            };
            match m3406 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f461(f985(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a3415 = {
            let a3412 = f958(Str::lit("costPriority"), match v0.clone() {
                Adt76::C0(_, h3407, _, _) => {
                    h3407
                }
            });
            let a3413 = {
                let a3410 = f958(Str::lit("speedPriority"), match v0.clone() {
                    Adt76::C0(_, _, h3408, _) => {
                        h3408
                    }
                });
                let a3411 = f958(Str::lit("intelligencePriority"), match v0 {
                    Adt76::C0(_, _, _, h3409) => {
                        h3409
                    }
                });
                append_list(a3410, a3411)
            };
            append_list(a3412, a3413)
        };
        append_list(a3414, a3415)
    })
}

pub fn f957(v0: Adt78) -> Str {
    let m3416 = v0;
    match m3416 {
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

pub fn f958(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m3417 = v1;
    match m3417 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f959(v0: Adt79) -> List<(Str, Adt6)> {
    let m3419 = match v0 {
        Adt79::C0(h3418) => {
            h3418
        }
    };
    match m3419 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f986(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f960(v0: Adt66) -> List<(Str, Adt6)> {
    let m3420 = v0;
    match m3420 {
        Adt66::C0(v1) => {
            let a3427 = {
                let m3422 = match v1.clone() {
                    Adt67::C0(h3421, _, _) => {
                        h3421
                    }
                };
                match m3422 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3428 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt67::C0(_, h3423, _) => {
                    h3423
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m3426 = match v1 {
                    Adt67::C0(_, _, h3425) => {
                        h3425
                    }
                };
                match m3426 {
                    Adt5::C0(h3424) => {
                        h3424
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a3427, a3428)
        }
        Adt66::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt69::C0(h3429, _) => {
                    h3429
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt69::C0(_, h3430) => {
                    h3430
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f961(v0: Adt87) -> Adt6 {
    let m3431 = v0;
    match m3431 {
        Adt87::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h3432) => {
                    h3432
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt87::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f329(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f962(v0: Adt150, v1: Adt150) -> bool {
    let a3433 = f766(v0);
    let a3434 = f766(v1);
    equal(a3433, a3434)
}

pub fn f963(v0: Adt150) -> bool {
    let m3435 = v0;
    match m3435 {
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

pub fn f964(v0: Adt150) -> bool {
    let m3436 = v0;
    match m3436 {
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

pub fn f965(v0: Adt0, v1: Adt17, v2: Adt82) -> Result<Adt139, Adt142> {
    f987(v0, v2.clone(), f988(match v1 {
        Adt17::C0(_, h3437, _, _, _, _) => {
            h3437
        }
    }, f740(v2)))
}

pub fn f966(v0: Adt15) -> Adt17 {
    let m3438 = v0;
    match m3438 {
        Adt15::C0(v1) => {
            match v1 {
                Adt16::C0(h3439) => {
                    h3439
                }
            }
        }
        Adt15::C1(v2) => {
            match v2 {
                Adt21::C0(h3440, _) => {
                    h3440
                }
            }
        }
        Adt15::C2(v3) => {
            match v3 {
                Adt22::C0(h3441, _, _, _, _) => {
                    h3441
                }
            }
        }
        Adt15::C3(v4) => {
            match v4 {
                Adt21::C0(h3442, _) => {
                    h3442
                }
            }
        }
        Adt15::C4(v5) => {
            match v5 {
                Adt23::C0(h3443, _, _, _) => {
                    h3443
                }
            }
        }
        Adt15::C5(v6) => {
            match v6 {
                Adt21::C0(h3444, _) => {
                    h3444
                }
            }
        }
        Adt15::C6(v7) => {
            match v7 {
                Adt21::C0(h3445, _) => {
                    h3445
                }
            }
        }
        Adt15::C7(v8) => {
            match v8 {
                Adt24::C0(h3446, _, _, _, _) => {
                    h3446
                }
            }
        }
        Adt15::C8(v9) => {
            match v9 {
                Adt25::C0(h3447, _, _, _) => {
                    h3447
                }
            }
        }
        Adt15::C9(v10) => {
            match v10 {
                Adt30::C0(h3448, _) => {
                    h3448
                }
            }
        }
        Adt15::C10(v11) => {
            match v11 {
                Adt31::C0(h3449, _, _) => {
                    h3449
                }
            }
        }
        Adt15::C11(v12) => {
            match v12 {
                Adt32::C0(h3450, _) => {
                    h3450
                }
            }
        }
    }
}

pub fn f967(v0: Adt0, v1: Adt17, v2: Adt83) -> Result<Adt139, Adt142> {
    f989(v0, v2, f202(match v1 {
        Adt17::C0(_, h3451, _, _, _, _) => {
            h3451
        }
    }))
}

pub fn f968(v0: Adt0, v1: Adt17, v2: Adt88) -> Result<Adt139, Adt142> {
    f990(v0, v2, f202(match v1 {
        Adt17::C0(_, h3452, _, _, _, _) => {
            h3452
        }
    }))
}

pub fn f969(v0: Adt0, v1: Adt17, v2: Adt50) -> Adt139 {
    Adt139::C0(Adt33::C2(match v2.clone() {
        Adt50::C0(h3453, _) => {
            h3453
        }
    }), {
        let m3455 = f890(v0, Adt33::C11);
        match m3455 {
            Adt139::C0(_, h3454, _) => {
                h3454
            }
        }
    }, f991(v1, match v2 {
        Adt50::C0(_, h3456) => {
            h3456
        }
    }))
}

pub fn f970(v0: Bytes, v1: Adt15, v2: Result<Adt33, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m3457 = v2;
    match m3457 {
        Ok(v4) => {
            f992(v0, f889(v1.clone()), f993(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f971(v0: u64, v1: u64, v2: Fn5, v3: Adt15) -> R<Result<Adt33, Adt90>> {
    let m3458 = v3.clone();
    match m3458 {
        Adt15::C0(_) => {
            let c3460: Fn5 = v2;
            c3460.apply(v3)
        }
        Adt15::C1(_) => {
            let c3461: Fn5 = v2;
            c3461.apply(v3)
        }
        Adt15::C2(_) => {
            if f994(v0, v1) {
                let c3459: Fn5 = v2;
                c3459.apply(v3)
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C7(v0)))
            }
        }
        Adt15::C3(_) => {
            let c3462: Fn5 = v2;
            c3462.apply(v3)
        }
        Adt15::C4(_) => {
            let c3463: Fn5 = v2;
            c3463.apply(v3)
        }
        Adt15::C5(_) => {
            let c3464: Fn5 = v2;
            c3464.apply(v3)
        }
        Adt15::C6(_) => {
            let c3465: Fn5 = v2;
            c3465.apply(v3)
        }
        Adt15::C7(_) => {
            let c3466: Fn5 = v2;
            c3466.apply(v3)
        }
        Adt15::C8(_) => {
            let c3467: Fn5 = v2;
            c3467.apply(v3)
        }
        Adt15::C9(_) => {
            let c3468: Fn5 = v2;
            c3468.apply(v3)
        }
        Adt15::C10(_) => {
            let c3469: Fn5 = v2;
            c3469.apply(v3)
        }
        Adt15::C11(_) => {
            let c3470: Fn5 = v2;
            c3470.apply(v3)
        }
    }
}

pub fn f972(v0: Bytes, v1: Adt22) -> R<Result<Adt15, Adt90>> {
    let m3474 = f995(v0, Str::lit("tools/call"), match v1.clone() {
        Adt22::C0(_, h3471, _, _, _) => {
            h3471
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, _, h3472) => {
            h3472
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h3473, _) => {
            h3473
        }
    })?;
    match m3474 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C2(Adt22::C0(match v1.clone() {
                Adt22::C0(h3475, _, _, _, _) => {
                    h3475
                }
            }, match v1.clone() {
                Adt22::C0(_, h3476, _, _, _) => {
                    h3476
                }
            }, match v1.clone() {
                Adt22::C0(_, _, h3477, _, _) => {
                    h3477
                }
            }, match v1 {
                Adt22::C0(_, _, _, h3478, _) => {
                    h3478
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f973(v0: Bytes, v1: Adt23) -> R<Result<Adt15, Adt90>> {
    let m3482 = f995(v0, Str::lit("resources/read"), match v1.clone() {
        Adt23::C0(_, h3479, _, _) => {
            h3479
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h3480) => {
            h3480
        }
    }, match v1.clone() {
        Adt23::C0(_, _, h3481, _) => {
            h3481
        }
    })?;
    match m3482 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C4(Adt23::C0(match v1.clone() {
                Adt23::C0(h3483, _, _, _) => {
                    h3483
                }
            }, match v1.clone() {
                Adt23::C0(_, h3484, _, _) => {
                    h3484
                }
            }, match v1 {
                Adt23::C0(_, _, h3485, _) => {
                    h3485
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f974(v0: Bytes, v1: Adt24) -> R<Result<Adt15, Adt90>> {
    let m3489 = f995(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt24::C0(_, h3486, _, _, _) => {
            h3486
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, _, h3487) => {
            h3487
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, h3488, _) => {
            h3488
        }
    })?;
    match m3489 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C7(Adt24::C0(match v1.clone() {
                Adt24::C0(h3490, _, _, _, _) => {
                    h3490
                }
            }, match v1.clone() {
                Adt24::C0(_, h3491, _, _, _) => {
                    h3491
                }
            }, match v1.clone() {
                Adt24::C0(_, _, h3492, _, _) => {
                    h3492
                }
            }, match v1 {
                Adt24::C0(_, _, _, h3493, _) => {
                    h3493
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f975(v0: Str, p3499: List<(Str, Adt6)>, p3500: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt143> {
    let m3498 = (p3499, p3500);
    {
        let mut m3498 = m3498;
        loop {
            let step = {
                let (v1, v2) = m3498;
                let m3494 = v1;
                match m3494.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt143>>(Ok::<List<(Str, Str)>, Adt143>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3496 = f190({
                            let (_, h3495) = v3;
                            h3495
                        });
                        match m3496 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt143>>(Err::<List<(Str, Str)>, Adt143>(Adt143::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt143>>((v4, List::cons(({
                                    let (h3497, _) = v3;
                                    h3497
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f976(p3503: List<(Str, Str)>, p3504: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3502 = (p3503, p3504);
    {
        let mut m3502 = m3502;
        loop {
            let step = {
                let (v0, v1) = m3502;
                let m3501 = v0;
                match m3501.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f977(v0: Adt124, v1: Adt5) -> R<Option<Adt122>> {
    let m3506 = f996(match v0.clone() {
        Adt124::C0(_, h3505, _, _) => {
            h3505
        }
    }, v1);
    match m3506 {
        Adt116::C0 => {
            Ok(f997(v0))
        }
        Adt116::C1(v2) => {
            if f998(v2.clone()) {
                Ok(f997(v0))
            } else {
                f999(v0, v2)
            }
        }
    }
}

pub fn f978(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m3507 = f264(v0, v1);
    match m3507 {
        Adt116::C0 => {
            Some(None::<Adt5>)
        }
        Adt116::C1(v2) => {
            let m3508 = f265(v2);
            match m3508 {
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

pub fn f979() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f980(v0: Str) -> Adt164 {
    let a3509 = v0.clone();
    let a3510 = Str::lit("debug");
    if equal(a3509, a3510) {
        Adt164::C1(Adt19::C0)
    } else {
        let a3511 = v0.clone();
        let a3512 = Str::lit("info");
        if equal(a3511, a3512) {
            Adt164::C1(Adt19::C1)
        } else {
            let a3513 = v0.clone();
            let a3514 = Str::lit("notice");
            if equal(a3513, a3514) {
                Adt164::C1(Adt19::C2)
            } else {
                let a3515 = v0.clone();
                let a3516 = Str::lit("warning");
                if equal(a3515, a3516) {
                    Adt164::C1(Adt19::C3)
                } else {
                    let a3517 = v0.clone();
                    let a3518 = Str::lit("error");
                    if equal(a3517, a3518) {
                        Adt164::C1(Adt19::C4)
                    } else {
                        let a3519 = v0.clone();
                        let a3520 = Str::lit("critical");
                        if equal(a3519, a3520) {
                            Adt164::C1(Adt19::C5)
                        } else {
                            let a3521 = v0.clone();
                            let a3522 = Str::lit("alert");
                            if equal(a3521, a3522) {
                                Adt164::C1(Adt19::C6)
                            } else {
                                let a3523 = v0;
                                let a3524 = Str::lit("emergency");
                                if equal(a3523, a3524) {
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

pub fn f981() -> Str {
    Str::lit("progressToken")
}

pub fn f982(v0: Adt3) -> Str {
    let m3525 = v0;
    match m3525 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f983(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a3534 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h3526, _, _, _) => {
                h3526
            }
        })), List::<(Str, Adt6)>::nil());
        let a3535 = {
            let a3532 = f720(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h3527, _, _) => {
                    h3527
                }
            });
            let a3533 = {
                let a3530 = f720(Str::lit("description"), match v0.clone() {
                    Adt58::C0(_, _, h3528, _) => {
                        h3528
                    }
                });
                let a3531 = f378(Str::lit("required"), match v0 {
                    Adt58::C0(_, _, _, h3529) => {
                        h3529
                    }
                });
                append_list(a3530, a3531)
            };
            append_list(a3532, a3533)
        };
        append_list(a3534, a3535)
    })
}

pub fn f984(v0: Adt71) -> R<Option<Adt6>> {
    let m3537 = f1000(match v0.clone() {
        Adt71::C0(_, h3536, _) => {
            h3536
        }
    })?;
    match m3537 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f927(match v0.clone() {
                Adt71::C0(h3538, _, _) => {
                    h3538
                }
            }))), List::cons((Str::lit("content"), v1), f719(match v0 {
                Adt71::C0(_, _, h3539) => {
                    h3539
                }
            }))))))
        }
    }
}

pub fn f985(p3543: List<Adt77>, p3544: List<Adt6>) -> List<Adt6> {
    let m3542 = (p3543, p3544);
    {
        let mut m3542 = m3542;
        loop {
            let step = {
                let (v0, v1) = m3542;
                let m3540 = v0;
                match m3540.uncons() {
                    None => {
                        Err::<(List<Adt77>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt77>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f720(Str::lit("name"), match v2 {
                            Adt77::C0(h3541) => {
                                h3541
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f986(v0: Adt80) -> Str {
    let m3545 = v0;
    match m3545 {
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

pub fn f987(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt139, Adt142> {
    if v2 {
        Ok::<Adt139, Adt142>(f890(v0, Adt33::C12(v1)))
    } else {
        Err::<Adt139, Adt142>(Adt142::C4(Adt90::C8(f1001(f740(v1)))))
    }
}

pub fn f988(v0: Adt18, v1: List<(Str, Adt65)>) -> bool {
    let a3546 = f1002(v1.clone());
    if if if bool_not(a3546) {
        true
    } else {
        f278(match v0.clone() {
            Adt18::C0(_, _, _, h3547, _) => {
                h3547
            }
        })
    } {
        let a3548 = f1003(v1.clone());
        if bool_not(a3548) {
            true
        } else {
            f278(match v0.clone() {
                Adt18::C0(_, _, h3549, _, _) => {
                    h3549
                }
            })
        }
    } else {
        false
    } {
        let a3550 = f1004(v1);
        if bool_not(a3550) {
            true
        } else {
            f278(match v0 {
                Adt18::C0(_, h3551, _, _, _) => {
                    h3551
                }
            })
        }
    } else {
        false
    }
}

pub fn f989(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt139, Adt142> {
    if v2 {
        Ok::<Adt139, Adt142>(f890(v0, Adt33::C13(v1)))
    } else {
        Err::<Adt139, Adt142>(Adt142::C4(Adt90::C8(f292())))
    }
}

pub fn f990(v0: Adt0, v1: Adt88, v2: bool) -> Result<Adt139, Adt142> {
    if v2 {
        Ok::<Adt139, Adt142>(f890(v0, Adt33::C14(v1)))
    } else {
        Err::<Adt139, Adt142>(Adt142::C4(Adt90::C8(f292())))
    }
}

pub fn f991(v0: Adt17, v1: List<Adt51>) -> List<Adt141> {
    let m3553 = match v0 {
        Adt17::C0(_, _, _, _, h3552, _) => {
            h3552
        }
    };
    match m3553 {
        None => {
            List::<Adt141>::nil()
        }
        Some(v2) => {
            f1005(f1006(v2, v1, List::<Adt141>::nil()), List::<Adt141>::nil())
        }
    }
}

pub fn f992(v0: Bytes, v1: Adt150, v2: Str, v3: Adt33) -> R<Result<Adt33, Adt90>> {
    let m3554 = v3.clone();
    match m3554 {
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
            f1007(v0, v1, v2, v4)
        }
        Adt33::C13(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C14(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f993(v0: Adt15) -> Str {
    let m3555 = v0;
    match m3555 {
        Adt15::C0(_) => {
            Str::lit("")
        }
        Adt15::C1(_) => {
            Str::lit("")
        }
        Adt15::C2(v1) => {
            match v1 {
                Adt22::C0(_, h3556, _, _, _) => {
                    h3556
                }
            }
        }
        Adt15::C3(_) => {
            Str::lit("")
        }
        Adt15::C4(v2) => {
            match v2 {
                Adt23::C0(_, h3557, _, _) => {
                    h3557
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
                Adt24::C0(_, h3558, _, _, _) => {
                    h3558
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

pub fn f994(v0: u64, v1: u64) -> bool {
    let a3559 = v1;
    let a3560 = v0;
    nat_lt(a3559, a3560)
}

pub fn f995(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt90>> {
    Ok(f1008(f1009(v0, v1, v2, v3)?, f1010(v4)))
}

pub fn f996(p3566: List<Str>, p3567: Adt5) -> Adt116 {
    let m3565 = (p3566, p3567);
    {
        let mut m3565 = m3565;
        loop {
            let step = {
                let (v0, v1) = m3565;
                let m3561 = v0;
                match m3561.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt116>(Adt116::C0)
                    }
                    Some((v2, v3)) => {
                        let m3562 = f264(v2, v1);
                        match m3562 {
                            Adt116::C0 => {
                                Err::<(List<Str>, Adt5), Adt116>(Adt116::C0)
                            }
                            Adt116::C1(v4) => {
                                let m3563 = v3.clone();
                                match m3563.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt116>(Adt116::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3564 = f265(v4);
                                        match m3564 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt116>(Adt116::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt116>((v3, v7))
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
                Ok(next) => { m3565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f997(v0: Adt124) -> Option<Adt122> {
    let m3569 = match v0.clone() {
        Adt124::C0(_, _, _, h3568) => {
            h3568
        }
    };
    match m3569 {
        Adt125::C0 => {
            None::<Adt122>
        }
        Adt125::C1 => {
            Some(Adt122::C10(match v0 {
                Adt124::C0(h3570, _, _, _) => {
                    h3570
                }
            }))
        }
        Adt125::C2(_) => {
            None::<Adt122>
        }
    }
}

pub fn f998(v0: Adt6) -> bool {
    let m3571 = v0;
    match m3571 {
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

pub fn f999(v0: Adt124, v1: Adt6) -> R<Option<Adt122>> {
    let m3573 = match v0.clone() {
        Adt124::C0(_, _, _, h3572) => {
            h3572
        }
    };
    match m3573 {
        Adt125::C0 => {
            Ok(Some(Adt122::C8(match v0 {
                Adt124::C0(h3574, _, _, _) => {
                    h3574
                }
            })))
        }
        Adt125::C1 => {
            Ok(Some(Adt122::C10(match v0 {
                Adt124::C0(h3575, _, _, _) => {
                    h3575
                }
            })))
        }
        Adt125::C2(v2) => {
            if f1011(match v0.clone() {
                Adt124::C0(_, _, h3576, _) => {
                    h3576
                }
            }, v2, v1)? {
                Ok(None::<Adt122>)
            } else {
                Ok(Some(Adt122::C9(match v0 {
                    Adt124::C0(h3577, _, _, _) => {
                        h3577
                    }
                })))
            }
        }
    }
}

pub fn f1000(v0: Adt72) -> R<Option<Adt6>> {
    let m3578 = v0;
    match m3578 {
        Adt72::C0(v1) => {
            f1012(v1)
        }
        Adt72::C1(v2) => {
            let m3579 = f1013(v2, List::<Adt6>::nil())?;
            match m3579 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f461(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f1001(v0: List<(Str, Adt65)>) -> Adt18 {
    Adt18::C0(None::<Adt5>, if f1004(v0.clone()) {
        Some(f27())
    } else {
        None::<Adt5>
    }, if f1003(v0.clone()) {
        Some(f27())
    } else {
        None::<Adt5>
    }, if f1002(v0) {
        Some(f27())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f1002(p3584: List<(Str, Adt65)>) -> bool {
    let m3583 = p3584;
    {
        let mut m3583 = m3583;
        loop {
            let step = {
                let v0 = m3583;
                let m3580 = v0;
                match m3580.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3582 = {
                            let (_, h3581) = v1;
                            h3581
                        };
                        match m3582 {
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
                Ok(next) => { m3583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1003(p3589: List<(Str, Adt65)>) -> bool {
    let m3588 = p3589;
    {
        let mut m3588 = m3588;
        loop {
            let step = {
                let v0 = m3588;
                let m3585 = v0;
                match m3585.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3587 = {
                            let (_, h3586) = v1;
                            h3586
                        };
                        match m3587 {
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
                Ok(next) => { m3588 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1004(p3594: List<(Str, Adt65)>) -> bool {
    let m3593 = p3594;
    {
        let mut m3593 = m3593;
        loop {
            let step = {
                let v0 = m3593;
                let m3590 = v0;
                match m3590.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3592 = {
                            let (_, h3591) = v1;
                            h3591
                        };
                        match m3592 {
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
                Ok(next) => { m3593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1005(p3597: List<Adt141>, p3598: List<Adt141>) -> List<Adt141> {
    let m3596 = (p3597, p3598);
    {
        let mut m3596 = m3596;
        loop {
            let step = {
                let (v0, v1) = m3596;
                let m3595 = v0;
                match m3595.uncons() {
                    None => {
                        Err::<(List<Adt141>, List<Adt141>), List<Adt141>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt141>, List<Adt141>), List<Adt141>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1006(v0: Adt20, p3601: List<Adt51>, p3602: List<Adt141>) -> List<Adt141> {
    let m3600 = (p3601, p3602);
    {
        let mut m3600 = m3600;
        loop {
            let step = {
                let (v1, v2) = m3600;
                let m3599 = v1;
                match m3599.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt141>), List<Adt141>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt51>, List<Adt141>), List<Adt141>>((v4, List::cons(Adt141::C0(v0.clone(), v3), v2)))
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

pub fn f1007(v0: Bytes, v1: Adt150, v2: Str, v3: Adt82) -> R<Result<Adt33, Adt90>> {
    if f963(v1.clone()) {
        if f1014(v3.clone()) {
            let m3603 = f1015(v0, f766(v1), v2, v3)?;
            match m3603 {
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
        Ok(Err::<Adt33, Adt90>(Adt90::C12(f766(v1))))
    }
}

pub fn f1008(v0: Result<Option<Str>, Adt90>, v1: bool) -> Result<Option<Str>, Adt90> {
    let m3604 = v0;
    match m3604 {
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

pub fn f1009(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt90>> {
    let m3605 = v3;
    match m3605 {
        None => {
            Ok(Ok::<Option<Str>, Adt90>(None::<Str>))
        }
        Some(v4) => {
            Ok(f1016(f1017(v0, v1, v2, v4)?))
        }
    }
}

pub fn f1010(v0: Option<Adt5>) -> bool {
    let m3606 = v0;
    match m3606 {
        None => {
            true
        }
        Some(v1) => {
            f1018(match v1 {
                Adt5::C0(h3607) => {
                    h3607
                }
            })
        }
    }
}

pub fn f1011(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3608 = v0;
    match m3608 {
        Adt13::C0 => {
            let m3609 = f190(v2);
            match m3609 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3610 = v1;
                    let a3611 = v3;
                    Ok(equal(a3610, a3611))
                }
            }
        }
        Adt13::C1 => {
            f1019(v1, v2)
        }
        Adt13::C2 => {
            let m3612 = f369(v2);
            match m3612 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3613 = v1;
                        let a3614 = Str::lit("true");
                        Ok(equal(a3613, a3614))
                    } else {
                        let a3615 = v1;
                        let a3616 = Str::lit("false");
                        Ok(equal(a3615, a3616))
                    }
                }
            }
        }
    }
}

pub fn f1012(v0: Adt73) -> R<Option<Adt6>> {
    let m3617 = v0;
    match m3617 {
        Adt73::C0(v1) => {
            Ok(Some(f624(v1)))
        }
        Adt73::C1(v2) => {
            Ok(Some(f625(Str::lit("image"), match v2.clone() {
                Adt43::C0(h3618, _, _, _) => {
                    h3618
                }
            }, match v2.clone() {
                Adt43::C0(_, h3619, _, _) => {
                    h3619
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h3620, _) => {
                    h3620
                }
            }, match v2 {
                Adt43::C0(_, _, _, h3621) => {
                    h3621
                }
            })))
        }
        Adt73::C2(v3) => {
            Ok(Some(f625(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h3622, _, _, _) => {
                    h3622
                }
            }, match v3.clone() {
                Adt44::C0(_, h3623, _, _) => {
                    h3623
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h3624, _) => {
                    h3624
                }
            }, match v3 {
                Adt44::C0(_, _, _, h3625) => {
                    h3625
                }
            })))
        }
        Adt73::C3(v4) => {
            Ok(Some(f1020(v4)))
        }
        Adt73::C4(v5) => {
            f1021(v5)
        }
    }
}

pub fn f1013(p3629: List<Adt73>, p3630: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3628 = (p3629, p3630);
    {
        let mut m3628 = m3628;
        loop {
            let step = {
                let (v0, v1) = m3628;
                let m3626 = v0;
                match m3626.uncons() {
                    None => {
                        Err::<(List<Adt73>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3627 = f1012(v2)?;
                        match m3627 {
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
                Ok(next) => { m3628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1014(v0: Adt82) -> bool {
    let m3631 = v0;
    match m3631 {
        Adt82::C0(v1) => {
            let m3632 = v1;
            match m3632.uncons() {
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

pub fn f1015(v0: Bytes, v1: Str, v2: Str, v3: Adt82) -> R<Result<Adt82, Adt90>> {
    let m3633 = v3.clone();
    match m3633 {
        Adt82::C0(_) => {
            Ok(Ok::<Adt82, Adt90>(v3))
        }
        Adt82::C1(v5) => {
            Ok(f1022(f1023(v0, v1, v2, v5)?, List::<(Str, Adt65)>::nil()))
        }
        Adt82::C2(v6, v7) => {
            Ok(f1022(f1023(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f1016(v0: Option<Str>) -> Result<Option<Str>, Adt90> {
    let m3634 = v0;
    match m3634 {
        None => {
            Err::<Option<Str>, Adt90>(Adt90::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt90>(Some(v1))
        }
    }
}

pub fn f1017(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3638 = f1024(v0, {
        let a3635 = v1;
        utf8_encode(a3635)
    }, {
        let a3636 = v2;
        utf8_encode(a3636)
    }, {
        let a3637 = v3;
        utf8_encode(a3637)
    })?;
    match m3638 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3639 = v4;
            Ok(utf8_decode(a3639))
        }
    }
}

pub fn f1018(p3644: List<(Str, Adt6)>) -> bool {
    let m3643 = p3644;
    {
        let mut m3643 = m3643;
        loop {
            let step = {
                let v0 = m3643;
                let m3640 = v0;
                match m3640.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3642 = f265({
                            let (_, h3641) = v1;
                            h3641
                        });
                        match m3642 {
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
                Ok(next) => { m3643 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1019(v0: Str, v1: Adt6) -> R<bool> {
    let m3645 = f699(v1);
    match m3645 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3646 = f505(v2)?;
            match m3646 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3648 = f1025(f229({
                        let a3647 = v0;
                        utf8_encode(a3647)
                    })?)?;
                    match m3648 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f374({
                                let a3649 = v3;
                                let a3650 = v4;
                                int_sub(a3649, a3650)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f1020(v0: Adt74) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt74::C0(h3651, _, _, _) => {
            h3651
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt74::C0(_, h3652, _, _) => {
            h3652
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3655 = match v0.clone() {
            Adt74::C0(_, _, h3654, _) => {
                h3654
            }
        };
        match m3655 {
            Adt5::C0(h3653) => {
                h3653
            }
        }
    })), f719(match v0 {
        Adt74::C0(_, _, _, h3656) => {
            h3656
        }
    }))))))
}

pub fn f1021(v0: Adt75) -> R<Option<Adt6>> {
    let m3658 = f376(match v0.clone() {
        Adt75::C0(_, h3657, _, _, _) => {
            h3657
        }
    })?;
    match m3658 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt75::C0(h3659, _, _, _, _) => {
                    h3659
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3665 = f377(Str::lit("structuredContent"), match v0.clone() {
                    Adt75::C0(_, _, h3660, _, _) => {
                        h3660
                    }
                });
                let a3666 = {
                    let a3663 = f378(Str::lit("isError"), match v0.clone() {
                        Adt75::C0(_, _, _, h3661, _) => {
                            h3661
                        }
                    });
                    let a3664 = f719(match v0 {
                        Adt75::C0(_, _, _, _, h3662) => {
                            h3662
                        }
                    });
                    append_list(a3663, a3664)
                };
                append_list(a3665, a3666)
            }))))))
        }
    }
}

pub fn f1022(v0: Option<Str>, v1: List<(Str, Adt65)>) -> Result<Adt82, Adt90> {
    let m3667 = v0;
    match m3667 {
        None => {
            Err::<Adt82, Adt90>(Adt90::C14)
        }
        Some(v2) => {
            let m3668 = v1.clone();
            match m3668.uncons() {
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

pub fn f1023(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3672 = f1026(v0, {
        let a3669 = v1;
        utf8_encode(a3669)
    }, {
        let a3670 = v2;
        utf8_encode(a3670)
    }, {
        let a3671 = v3;
        utf8_encode(a3671)
    })?;
    match m3672 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3673 = v4;
            Ok(utf8_decode(a3673))
        }
    }
}

pub fn f1024(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3675 = f1027();
    let a3676 = {
        let a3674 = v0.clone();
        length_bytes(a3674)
    };
    if nat_le(a3675, a3676) {
        let a3678 = f1028(v3.clone())?;
        let a3679 = {
            let a3677 = v3.clone();
            length_bytes(a3677)
        };
        if nat_eq(a3678, a3679) {
            Ok(None::<Bytes>)
        } else {
            f1029(v0, v1, v2, v3.clone(), f1028(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1025(v0: List<u64>) -> R<Option<i64>> {
    let m3680 = v0.clone();
    match m3680.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3681 = v1;
            let a3682 = 45u64;
            if nat_eq(a3681, a3682) {
                let m3683 = v2.clone();
                match m3683.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3684 = f1030(v2, 0i64)?;
                        match m3684 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3685 = v5;
                                    int_neg(a3685)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f1030(v0, 0i64)
            }
        }
    }
}

pub fn f1026(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3687 = f1027();
    let a3688 = {
        let a3686 = v0.clone();
        length_bytes(a3686)
    };
    if nat_le(a3687, a3688) {
        Ok(Some({
            let a3691 = f1031(v3.clone())?;
            let a3692 = {
                let a3689 = Bytes::lit(&[0x2e]);
                let a3690 = f1031(f1032(v0, v1, v2, v3)?)?;
                append_bytes(a3689, a3690)
            };
            append_bytes(a3691, a3692)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1027() -> u64 {
    32u64
}

pub fn f1028(v0: Bytes) -> R<u64> {
    let (_, h3693) = f1033(Fn9::F1034(v0.clone()), {
        let a3694 = v0.clone();
        length_bytes(a3694)
    }, (0u64, {
        let a3695 = v0;
        length_bytes(a3695)
    }))?;
    Ok(h3693)
}

pub fn f1029(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3699 = {
        let a3696 = v3.clone();
        let a3697 = 0u64;
        let a3698 = v4;
        slice_bytes(a3696, a3697, a3698)
    };
    match m3699 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3710 = {
                let a3707 = v3.clone();
                let a3708 = {
                    let a3700 = v4;
                    let a3701 = 1u64;
                    nat_add(a3700, a3701)?
                };
                let a3709 = {
                    let a3705 = {
                        let a3702 = v3;
                        length_bytes(a3702)
                    };
                    let a3706 = {
                        let a3703 = v4;
                        let a3704 = 1u64;
                        nat_add(a3703, a3704)?
                    };
                    nat_sub(a3705, a3706)
                };
                slice_bytes(a3707, a3708, a3709)
            };
            match m3710 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f1035(v0, v1, v2, f1036(v5)?, f1036(v6)?)
                }
            }
        }
    }
}

pub fn f1030(p3716: List<u64>, p3717: i64) -> R<Option<i64>> {
    let m3715 = (p3716, p3717);
    {
        let mut m3715 = m3715;
        loop {
            let step = {
                let (v0, v1) = m3715;
                let m3711 = v0;
                match m3711.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3712 = f1037(v2);
                        match m3712 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f368({
                                    let a3713 = v1;
                                    let a3714 = 10i64;
                                    int_mul(a3713, a3714)?
                                }, f586(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1031(v0: Bytes) -> R<Bytes> {
    let (_, h3718) = f1038(Fn10::F1039(v0.clone()), {
        let a3719 = v0;
        length_bytes(a3719)
    }, (0u64, f232()))?;
    Ok(h3718)
}

pub fn f1032(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f1040(v0, {
        let a3720 = f1041(v1, v2);
        let a3721 = v3;
        append_bytes(a3720, a3721)
    })
}

pub fn f1033(v0: Fn9, p3725: u64, p3726: (u64, u64)) -> R<(u64, u64)> {
    let m3724 = (p3725, p3726);
    {
        let mut m3724 = m3724;
        loop {
            let step = {
                let (v1, v2) = m3724;
                let m3722 = v1;
                if m3722 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3722 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3723: Fn9 = v0.clone();
                        c3723.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1034(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3728 = {
            let (h3727, _) = v1;
            h3727
        };
        let a3729 = 1u64;
        nat_add(a3728, a3729)?
    }, {
        let a3732 = {
            let (_, h3730) = v1;
            h3730
        };
        let a3733 = {
            let a3731 = v0.clone();
            length_bytes(a3731)
        };
        if nat_eq(a3732, a3733) {
            let a3735 = f126(v0, {
                let (h3734, _) = v1;
                h3734
            })?;
            let a3736 = 46u64;
            if nat_eq(a3735, a3736) {
                let (h3737, _) = v1;
                h3737
            } else {
                let (_, h3738) = v1;
                h3738
            }
        } else {
            let (_, h3739) = v1;
            h3739
        }
    }))
}

pub fn f1035(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3740 = v3;
    match m3740 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3741 = v4;
            match m3741 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f1042(f1032(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f1036(v0: Bytes) -> R<Option<Bytes>> {
    let a3746 = {
        let a3743 = {
            let a3742 = v0.clone();
            length_bytes(a3742)
        };
        let a3744 = 2u64;
        let a3745 = 0u64;
        nat_rem(a3743, a3744, a3745)
    };
    let a3747 = 0u64;
    if nat_eq(a3746, a3747) {
        let v1: (u64, (Bytes, u64)) = f1043(Fn11::F1044(v0.clone()), {
            let a3749 = {
                let a3748 = v0;
                length_bytes(a3748)
            };
            let a3750 = 2u64;
            let a3751 = 0u64;
            nat_quot(a3749, a3750, a3751)
        }, (0u64, (f232(), 0u64)))?;
        let a3754 = {
            let (_, h3752) = {
                let (_, h3753) = v1.clone();
                h3753
            };
            h3752
        };
        let a3755 = 0u64;
        if nat_eq(a3754, a3755) {
            Ok(Some({
                let (h3756, _) = {
                    let (_, h3757) = v1;
                    h3757
                };
                h3756
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1037(v0: u64) -> Option<u64> {
    let a3758 = v0;
    let a3759 = 48u64;
    if if nat_lt(a3758, a3759) {
        true
    } else {
        let a3760 = 57u64;
        let a3761 = v0;
        nat_lt(a3760, a3761)
    } {
        None::<u64>
    } else {
        Some({
            let a3762 = v0;
            let a3763 = 48u64;
            nat_sub(a3762, a3763)
        })
    }
}

pub fn f1038(v0: Fn10, p3767: u64, p3768: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3766 = (p3767, p3768);
    {
        let mut m3766 = m3766;
        loop {
            let step = {
                let (v1, v2) = m3766;
                let m3764 = v1;
                if m3764 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3764 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3765: Fn10 = v0.clone();
                        c3765.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3766 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1039(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3770 = {
            let (h3769, _) = v1.clone();
            h3769
        };
        let a3771 = 1u64;
        nat_add(a3770, a3771)?
    }, {
        let a3774 = {
            let (_, h3772) = v1;
            h3772
        };
        let a3775 = f1045(f126(v0, {
            let (h3773, _) = v1;
            h3773
        })?)?;
        append_bytes(a3774, a3775)
    }))
}

pub fn f1040(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f1046(f1047(v0, v1)?)
}

pub fn f1041(v0: Bytes, v1: Bytes) -> Bytes {
    let a3782 = f1048({
        let a3776 = v0.clone();
        length_bytes(a3776)
    });
    let a3783 = {
        let a3780 = v0;
        let a3781 = {
            let a3778 = f1048({
                let a3777 = v1.clone();
                length_bytes(a3777)
            });
            let a3779 = v1;
            append_bytes(a3778, a3779)
        };
        append_bytes(a3780, a3781)
    };
    append_bytes(a3782, a3783)
}

pub fn f1042(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3786 = {
        let a3784 = v0.clone();
        length_bytes(a3784)
    };
    let a3787 = {
        let a3785 = v1.clone();
        length_bytes(a3785)
    };
    if nat_eq(a3786, a3787) {
        let a3788 = f1049(v0, v1)?;
        let a3789 = 0u64;
        Ok(nat_eq(a3788, a3789))
    } else {
        Ok(false)
    }
}

pub fn f1043(v0: Fn11, p3793: u64, p3794: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3792 = (p3793, p3794);
    {
        let mut m3792 = m3792;
        loop {
            let step = {
                let (v1, v2) = m3792;
                let m3790 = v1;
                if m3790 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3790 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3791: Fn11 = v0.clone();
                        c3791.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1044(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f1050(v0, v1)
}

pub fn f1045(v0: u64) -> R<Bytes> {
    let a3801 = f1051({
        let a3795 = v0;
        let a3796 = 16u64;
        let a3797 = 0u64;
        nat_quot(a3795, a3796, a3797)
    })?;
    let a3802 = f1051({
        let a3798 = v0;
        let a3799 = 16u64;
        let a3800 = 0u64;
        nat_rem(a3798, a3799, a3800)
    })?;
    Ok(append_bytes(a3801, a3802))
}

pub fn f1046(v0: Adt165) -> R<Bytes> {
    let a3823 = f1052(match v0.clone() {
        Adt165::C0(h3803, _, _, _, _, _, _, _) => {
            h3803
        }
    })?;
    let a3824 = {
        let a3821 = f1052(match v0.clone() {
            Adt165::C0(_, h3804, _, _, _, _, _, _) => {
                h3804
            }
        })?;
        let a3822 = {
            let a3819 = f1052(match v0.clone() {
                Adt165::C0(_, _, h3805, _, _, _, _, _) => {
                    h3805
                }
            })?;
            let a3820 = {
                let a3817 = f1052(match v0.clone() {
                    Adt165::C0(_, _, _, h3806, _, _, _, _) => {
                        h3806
                    }
                })?;
                let a3818 = {
                    let a3815 = f1052(match v0.clone() {
                        Adt165::C0(_, _, _, _, h3807, _, _, _) => {
                            h3807
                        }
                    })?;
                    let a3816 = {
                        let a3813 = f1052(match v0.clone() {
                            Adt165::C0(_, _, _, _, _, h3808, _, _) => {
                                h3808
                            }
                        })?;
                        let a3814 = {
                            let a3811 = f1052(match v0.clone() {
                                Adt165::C0(_, _, _, _, _, _, h3809, _) => {
                                    h3809
                                }
                            })?;
                            let a3812 = f1052(match v0 {
                                Adt165::C0(_, _, _, _, _, _, _, h3810) => {
                                    h3810
                                }
                            })?;
                            append_bytes(a3811, a3812)
                        };
                        append_bytes(a3813, a3814)
                    };
                    append_bytes(a3815, a3816)
                };
                append_bytes(a3817, a3818)
            };
            append_bytes(a3819, a3820)
        };
        append_bytes(a3821, a3822)
    };
    Ok(append_bytes(a3823, a3824))
}

pub fn f1047(v0: Bytes, v1: Bytes) -> R<Adt165> {
    let v2: Adt166 = f1053(v0)?;
    let v3: Adt165 = f1054(f1055(f1056(), f1057(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f1055(f1055(f1056(), f1057(v2, 1549556828u32))?, f1058(v3))
}

pub fn f1048(v0: u64) -> Bytes {
    let a3850 = f630({
        let a3828 = {
            let a3825 = v0;
            let a3826 = 16777216u64;
            let a3827 = 0u64;
            nat_quot(a3825, a3826, a3827)
        };
        let a3829 = 256u64;
        let a3830 = 0u64;
        nat_rem(a3828, a3829, a3830)
    });
    let a3851 = {
        let a3848 = f630({
            let a3834 = {
                let a3831 = v0;
                let a3832 = 65536u64;
                let a3833 = 0u64;
                nat_quot(a3831, a3832, a3833)
            };
            let a3835 = 256u64;
            let a3836 = 0u64;
            nat_rem(a3834, a3835, a3836)
        });
        let a3849 = {
            let a3846 = f630({
                let a3840 = {
                    let a3837 = v0;
                    let a3838 = 256u64;
                    let a3839 = 0u64;
                    nat_quot(a3837, a3838, a3839)
                };
                let a3841 = 256u64;
                let a3842 = 0u64;
                nat_rem(a3840, a3841, a3842)
            });
            let a3847 = f630({
                let a3843 = v0;
                let a3844 = 256u64;
                let a3845 = 0u64;
                nat_rem(a3843, a3844, a3845)
            });
            append_bytes(a3846, a3847)
        };
        append_bytes(a3848, a3849)
    };
    append_bytes(a3850, a3851)
}

pub fn f1049(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3852) = f1033(Fn9::F1059(v0.clone(), v1), {
        let a3853 = v0;
        length_bytes(a3853)
    }, (0u64, 0u64))?;
    Ok(h3852)
}

pub fn f1050(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f1060(f126(v0.clone(), {
        let (h3854, _) = v1.clone();
        h3854
    })?);
    let v3: u64 = f1060(f126(v0, {
        let a3856 = {
            let (h3855, _) = v1.clone();
            h3855
        };
        let a3857 = 1u64;
        nat_add(a3856, a3857)?
    })?);
    let a3858 = v2;
    let a3859 = 16u64;
    if if nat_lt(a3858, a3859) {
        let a3860 = v3;
        let a3861 = 16u64;
        nat_lt(a3860, a3861)
    } else {
        false
    } {
        Ok(({
            let a3863 = {
                let (h3862, _) = v1.clone();
                h3862
            };
            let a3864 = 2u64;
            nat_add(a3863, a3864)?
        }, ({
            let a3871 = {
                let (h3865, _) = {
                    let (_, h3866) = v1.clone();
                    h3866
                };
                h3865
            };
            let a3872 = f630({
                let a3869 = {
                    let a3867 = v2;
                    let a3868 = 16u64;
                    nat_mul(a3867, a3868)?
                };
                let a3870 = v3;
                nat_add(a3869, a3870)?
            });
            append_bytes(a3871, a3872)
        }, {
            let (_, h3873) = {
                let (_, h3874) = v1;
                h3874
            };
            h3873
        })))
    } else {
        Ok(({
            let a3876 = {
                let (h3875, _) = v1.clone();
                h3875
            };
            let a3877 = 2u64;
            nat_add(a3876, a3877)?
        }, ({
            let (h3878, _) = {
                let (_, h3879) = v1.clone();
                h3879
            };
            h3878
        }, {
            let a3882 = {
                let (_, h3880) = {
                    let (_, h3881) = v1;
                    h3881
                };
                h3880
            };
            let a3883 = 1u64;
            nat_add(a3882, a3883)?
        })))
    }
}

pub fn f1051(v0: u64) -> R<Bytes> {
    Ok(f630({
        let a3884 = v0;
        let a3885 = 10u64;
        if nat_lt(a3884, a3885) {
            let a3886 = 48u64;
            let a3887 = v0;
            nat_add(a3886, a3887)?
        } else {
            let a3888 = 87u64;
            let a3889 = v0;
            nat_add(a3888, a3889)?
        }
    }))
}

pub fn f1052(v0: u32) -> R<Bytes> {
    let a3894 = f1061(v0, 24u32)?;
    let a3895 = {
        let a3892 = f1061(v0, 16u32)?;
        let a3893 = {
            let a3890 = f1061(v0, 8u32)?;
            let a3891 = f1061(v0, 0u32)?;
            append_bytes(a3890, a3891)
        };
        append_bytes(a3892, a3893)
    };
    Ok(append_bytes(a3894, a3895))
}

pub fn f1053(v0: Bytes) -> R<Adt166> {
    let a3897 = f1062();
    let a3898 = {
        let a3896 = v0.clone();
        length_bytes(a3896)
    };
    if nat_lt(a3897, a3898) {
        Ok(f1063(f1064(v0)?))
    } else {
        f1065(v0)
    }
}

pub fn f1054(v0: Adt165, v1: u64, v2: Bytes) -> R<Adt165> {
    let v3: u64 = {
        let a3899 = v2.clone();
        length_bytes(a3899)
    };
    let v4: u64 = {
        let a3905 = {
            let a3902 = {
                let a3900 = v3;
                let a3901 = 72u64;
                nat_add(a3900, a3901)?
            };
            let a3903 = 64u64;
            let a3904 = 0u64;
            nat_quot(a3902, a3903, a3904)
        };
        let a3906 = 64u64;
        nat_mul(a3905, a3906)?
    };
    let (_, h3907) = f1066(Fn12::F1067(v2, v1, v3, v4), {
        let a3908 = v4;
        let a3909 = 64u64;
        let a3910 = 0u64;
        nat_quot(a3908, a3909, a3910)
    }, (0u64, v0))?;
    Ok(h3907)
}

pub fn f1055(v0: Adt165, v1: Adt166) -> R<Adt165> {
    let v2: Adt167 = f1068(Fn13::F1069, 64u64, Adt167::C0(match v0.clone() {
        Adt165::C0(h3911, _, _, _, _, _, _, _) => {
            h3911
        }
    }, match v0.clone() {
        Adt165::C0(_, h3912, _, _, _, _, _, _) => {
            h3912
        }
    }, match v0.clone() {
        Adt165::C0(_, _, h3913, _, _, _, _, _) => {
            h3913
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, h3914, _, _, _, _) => {
            h3914
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, h3915, _, _, _) => {
            h3915
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, h3916, _, _) => {
            h3916
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h3917, _) => {
            h3917
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, _, h3918) => {
            h3918
        }
    }, v1, 0u64))?;
    Ok(Adt165::C0(f1070(match v0.clone() {
        Adt165::C0(h3919, _, _, _, _, _, _, _) => {
            h3919
        }
    }, match v2.clone() {
        Adt167::C0(h3920, _, _, _, _, _, _, _, _, _) => {
            h3920
        }
    }), f1070(match v0.clone() {
        Adt165::C0(_, h3921, _, _, _, _, _, _) => {
            h3921
        }
    }, match v2.clone() {
        Adt167::C0(_, h3922, _, _, _, _, _, _, _, _) => {
            h3922
        }
    }), f1070(match v0.clone() {
        Adt165::C0(_, _, h3923, _, _, _, _, _) => {
            h3923
        }
    }, match v2.clone() {
        Adt167::C0(_, _, h3924, _, _, _, _, _, _, _) => {
            h3924
        }
    }), f1070(match v0.clone() {
        Adt165::C0(_, _, _, h3925, _, _, _, _) => {
            h3925
        }
    }, match v2.clone() {
        Adt167::C0(_, _, _, h3926, _, _, _, _, _, _) => {
            h3926
        }
    }), f1070(match v0.clone() {
        Adt165::C0(_, _, _, _, h3927, _, _, _) => {
            h3927
        }
    }, match v2.clone() {
        Adt167::C0(_, _, _, _, h3928, _, _, _, _, _) => {
            h3928
        }
    }), f1070(match v0.clone() {
        Adt165::C0(_, _, _, _, _, h3929, _, _) => {
            h3929
        }
    }, match v2.clone() {
        Adt167::C0(_, _, _, _, _, h3930, _, _, _, _) => {
            h3930
        }
    }), f1070(match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h3931, _) => {
            h3931
        }
    }, match v2.clone() {
        Adt167::C0(_, _, _, _, _, _, h3932, _, _, _) => {
            h3932
        }
    }), f1070(match v0 {
        Adt165::C0(_, _, _, _, _, _, _, h3933) => {
            h3933
        }
    }, match v2 {
        Adt167::C0(_, _, _, _, _, _, _, h3934, _, _) => {
            h3934
        }
    })))
}

pub fn f1056() -> Adt165 {
    Adt165::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f1057(v0: Adt166, v1: u32) -> Adt166 {
    Adt166::C0({
        let a3936 = match v0.clone() {
            Adt166::C0(h3935, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3935
            }
        };
        let a3937 = v1;
        fixed_u32::bit_xor(a3936, a3937)
    }, {
        let a3939 = match v0.clone() {
            Adt166::C0(_, h3938, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3938
            }
        };
        let a3940 = v1;
        fixed_u32::bit_xor(a3939, a3940)
    }, {
        let a3942 = match v0.clone() {
            Adt166::C0(_, _, h3941, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3941
            }
        };
        let a3943 = v1;
        fixed_u32::bit_xor(a3942, a3943)
    }, {
        let a3945 = match v0.clone() {
            Adt166::C0(_, _, _, h3944, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3944
            }
        };
        let a3946 = v1;
        fixed_u32::bit_xor(a3945, a3946)
    }, {
        let a3948 = match v0.clone() {
            Adt166::C0(_, _, _, _, h3947, _, _, _, _, _, _, _, _, _, _, _) => {
                h3947
            }
        };
        let a3949 = v1;
        fixed_u32::bit_xor(a3948, a3949)
    }, {
        let a3951 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, h3950, _, _, _, _, _, _, _, _, _, _) => {
                h3950
            }
        };
        let a3952 = v1;
        fixed_u32::bit_xor(a3951, a3952)
    }, {
        let a3954 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, h3953, _, _, _, _, _, _, _, _, _) => {
                h3953
            }
        };
        let a3955 = v1;
        fixed_u32::bit_xor(a3954, a3955)
    }, {
        let a3957 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, h3956, _, _, _, _, _, _, _, _) => {
                h3956
            }
        };
        let a3958 = v1;
        fixed_u32::bit_xor(a3957, a3958)
    }, {
        let a3960 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, h3959, _, _, _, _, _, _, _) => {
                h3959
            }
        };
        let a3961 = v1;
        fixed_u32::bit_xor(a3960, a3961)
    }, {
        let a3963 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, h3962, _, _, _, _, _, _) => {
                h3962
            }
        };
        let a3964 = v1;
        fixed_u32::bit_xor(a3963, a3964)
    }, {
        let a3966 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, h3965, _, _, _, _, _) => {
                h3965
            }
        };
        let a3967 = v1;
        fixed_u32::bit_xor(a3966, a3967)
    }, {
        let a3969 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, h3968, _, _, _, _) => {
                h3968
            }
        };
        let a3970 = v1;
        fixed_u32::bit_xor(a3969, a3970)
    }, {
        let a3972 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3971, _, _, _) => {
                h3971
            }
        };
        let a3973 = v1;
        fixed_u32::bit_xor(a3972, a3973)
    }, {
        let a3975 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3974, _, _) => {
                h3974
            }
        };
        let a3976 = v1;
        fixed_u32::bit_xor(a3975, a3976)
    }, {
        let a3978 = match v0.clone() {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3977, _) => {
                h3977
            }
        };
        let a3979 = v1;
        fixed_u32::bit_xor(a3978, a3979)
    }, {
        let a3981 = match v0 {
            Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3980) => {
                h3980
            }
        };
        let a3982 = v1;
        fixed_u32::bit_xor(a3981, a3982)
    })
}

pub fn f1058(v0: Adt165) -> Adt166 {
    Adt166::C0(match v0.clone() {
        Adt165::C0(h3983, _, _, _, _, _, _, _) => {
            h3983
        }
    }, match v0.clone() {
        Adt165::C0(_, h3984, _, _, _, _, _, _) => {
            h3984
        }
    }, match v0.clone() {
        Adt165::C0(_, _, h3985, _, _, _, _, _) => {
            h3985
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, h3986, _, _, _, _) => {
            h3986
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, h3987, _, _, _) => {
            h3987
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, h3988, _, _) => {
            h3988
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h3989, _) => {
            h3989
        }
    }, match v0 {
        Adt165::C0(_, _, _, _, _, _, _, h3990) => {
            h3990
        }
    }, 2147483648u32, f1071(), f1071(), f1071(), f1071(), f1071(), f1071(), 768u32)
}

pub fn f1059(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3992 = {
            let (h3991, _) = v2;
            h3991
        };
        let a3993 = 1u64;
        nat_add(a3992, a3993)?
    }, {
        let a4045 = {
            let (_, h3994) = v2;
            h3994
        };
        let a4046 = {
            let v3: u8 = {
                let a3997 = f1072(v0, {
                    let (h3995, _) = v2;
                    h3995
                });
                let a3998 = f1072(v1, {
                    let (h3996, _) = v2;
                    h3996
                });
                fixed_u8::bit_xor(a3997, a3998)
            };
            let a4043 = {
                let a4019 = {
                    let a4007 = {
                        let a4001 = {
                            let a3999 = v3;
                            let a4000 = 128u8;
                            fixed_u8::bit_and(a3999, a4000)
                        };
                        let a4002 = 128u8;
                        if equal(a4001, a4002) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a4008 = {
                        let a4005 = {
                            let a4003 = v3;
                            let a4004 = 64u8;
                            fixed_u8::bit_and(a4003, a4004)
                        };
                        let a4006 = 64u8;
                        if equal(a4005, a4006) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4007, a4008)?
                };
                let a4020 = {
                    let a4017 = {
                        let a4011 = {
                            let a4009 = v3;
                            let a4010 = 32u8;
                            fixed_u8::bit_and(a4009, a4010)
                        };
                        let a4012 = 32u8;
                        if equal(a4011, a4012) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a4018 = {
                        let a4015 = {
                            let a4013 = v3;
                            let a4014 = 16u8;
                            fixed_u8::bit_and(a4013, a4014)
                        };
                        let a4016 = 16u8;
                        if equal(a4015, a4016) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4017, a4018)?
                };
                nat_add(a4019, a4020)?
            };
            let a4044 = {
                let a4041 = {
                    let a4029 = {
                        let a4023 = {
                            let a4021 = v3;
                            let a4022 = 8u8;
                            fixed_u8::bit_and(a4021, a4022)
                        };
                        let a4024 = 8u8;
                        if equal(a4023, a4024) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a4030 = {
                        let a4027 = {
                            let a4025 = v3;
                            let a4026 = 4u8;
                            fixed_u8::bit_and(a4025, a4026)
                        };
                        let a4028 = 4u8;
                        if equal(a4027, a4028) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4029, a4030)?
                };
                let a4042 = {
                    let a4039 = {
                        let a4033 = {
                            let a4031 = v3;
                            let a4032 = 2u8;
                            fixed_u8::bit_and(a4031, a4032)
                        };
                        let a4034 = 2u8;
                        if equal(a4033, a4034) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a4040 = {
                        let a4037 = {
                            let a4035 = v3;
                            let a4036 = 1u8;
                            fixed_u8::bit_and(a4035, a4036)
                        };
                        let a4038 = 1u8;
                        if equal(a4037, a4038) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4039, a4040)?
                };
                nat_add(a4041, a4042)?
            };
            nat_add(a4043, a4044)?
        };
        nat_add(a4045, a4046)?
    }))
}

pub fn f1060(v0: u64) -> u64 {
    let a4047 = v0;
    let a4048 = 48u64;
    if nat_lt(a4047, a4048) {
        16u64
    } else {
        let a4049 = v0;
        let a4050 = 58u64;
        if nat_lt(a4049, a4050) {
            let a4051 = v0;
            let a4052 = 48u64;
            nat_sub(a4051, a4052)
        } else {
            let a4053 = v0;
            let a4054 = 97u64;
            if nat_lt(a4053, a4054) {
                16u64
            } else {
                let a4055 = v0;
                let a4056 = 103u64;
                if nat_lt(a4055, a4056) {
                    let a4057 = v0;
                    let a4058 = 87u64;
                    nat_sub(a4057, a4058)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f1061(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f630({
        let v2: u32 = {
            let a4059 = f1073(v0, v1);
            let a4060 = 255u32;
            fixed_u32::bit_and(a4059, a4060)
        };
        let a4249 = {
            let a4153 = {
                let a4105 = {
                    let a4081 = {
                        let a4069 = {
                            let a4063 = {
                                let a4061 = v2;
                                let a4062 = 2147483648u32;
                                fixed_u32::bit_and(a4061, a4062)
                            };
                            let a4064 = 2147483648u32;
                            if equal(a4063, a4064) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a4070 = {
                            let a4067 = {
                                let a4065 = v2;
                                let a4066 = 1073741824u32;
                                fixed_u32::bit_and(a4065, a4066)
                            };
                            let a4068 = 1073741824u32;
                            if equal(a4067, a4068) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4069, a4070)?
                    };
                    let a4082 = {
                        let a4079 = {
                            let a4073 = {
                                let a4071 = v2;
                                let a4072 = 536870912u32;
                                fixed_u32::bit_and(a4071, a4072)
                            };
                            let a4074 = 536870912u32;
                            if equal(a4073, a4074) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a4080 = {
                            let a4077 = {
                                let a4075 = v2;
                                let a4076 = 268435456u32;
                                fixed_u32::bit_and(a4075, a4076)
                            };
                            let a4078 = 268435456u32;
                            if equal(a4077, a4078) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4079, a4080)?
                    };
                    nat_add(a4081, a4082)?
                };
                let a4106 = {
                    let a4103 = {
                        let a4091 = {
                            let a4085 = {
                                let a4083 = v2;
                                let a4084 = 134217728u32;
                                fixed_u32::bit_and(a4083, a4084)
                            };
                            let a4086 = 134217728u32;
                            if equal(a4085, a4086) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a4092 = {
                            let a4089 = {
                                let a4087 = v2;
                                let a4088 = 67108864u32;
                                fixed_u32::bit_and(a4087, a4088)
                            };
                            let a4090 = 67108864u32;
                            if equal(a4089, a4090) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4091, a4092)?
                    };
                    let a4104 = {
                        let a4101 = {
                            let a4095 = {
                                let a4093 = v2;
                                let a4094 = 33554432u32;
                                fixed_u32::bit_and(a4093, a4094)
                            };
                            let a4096 = 33554432u32;
                            if equal(a4095, a4096) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a4102 = {
                            let a4099 = {
                                let a4097 = v2;
                                let a4098 = 16777216u32;
                                fixed_u32::bit_and(a4097, a4098)
                            };
                            let a4100 = 16777216u32;
                            if equal(a4099, a4100) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4101, a4102)?
                    };
                    nat_add(a4103, a4104)?
                };
                nat_add(a4105, a4106)?
            };
            let a4154 = {
                let a4151 = {
                    let a4127 = {
                        let a4115 = {
                            let a4109 = {
                                let a4107 = v2;
                                let a4108 = 8388608u32;
                                fixed_u32::bit_and(a4107, a4108)
                            };
                            let a4110 = 8388608u32;
                            if equal(a4109, a4110) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a4116 = {
                            let a4113 = {
                                let a4111 = v2;
                                let a4112 = 4194304u32;
                                fixed_u32::bit_and(a4111, a4112)
                            };
                            let a4114 = 4194304u32;
                            if equal(a4113, a4114) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4115, a4116)?
                    };
                    let a4128 = {
                        let a4125 = {
                            let a4119 = {
                                let a4117 = v2;
                                let a4118 = 2097152u32;
                                fixed_u32::bit_and(a4117, a4118)
                            };
                            let a4120 = 2097152u32;
                            if equal(a4119, a4120) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a4126 = {
                            let a4123 = {
                                let a4121 = v2;
                                let a4122 = 1048576u32;
                                fixed_u32::bit_and(a4121, a4122)
                            };
                            let a4124 = 1048576u32;
                            if equal(a4123, a4124) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4125, a4126)?
                    };
                    nat_add(a4127, a4128)?
                };
                let a4152 = {
                    let a4149 = {
                        let a4137 = {
                            let a4131 = {
                                let a4129 = v2;
                                let a4130 = 524288u32;
                                fixed_u32::bit_and(a4129, a4130)
                            };
                            let a4132 = 524288u32;
                            if equal(a4131, a4132) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a4138 = {
                            let a4135 = {
                                let a4133 = v2;
                                let a4134 = 262144u32;
                                fixed_u32::bit_and(a4133, a4134)
                            };
                            let a4136 = 262144u32;
                            if equal(a4135, a4136) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4137, a4138)?
                    };
                    let a4150 = {
                        let a4147 = {
                            let a4141 = {
                                let a4139 = v2;
                                let a4140 = 131072u32;
                                fixed_u32::bit_and(a4139, a4140)
                            };
                            let a4142 = 131072u32;
                            if equal(a4141, a4142) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a4148 = {
                            let a4145 = {
                                let a4143 = v2;
                                let a4144 = 65536u32;
                                fixed_u32::bit_and(a4143, a4144)
                            };
                            let a4146 = 65536u32;
                            if equal(a4145, a4146) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4147, a4148)?
                    };
                    nat_add(a4149, a4150)?
                };
                nat_add(a4151, a4152)?
            };
            nat_add(a4153, a4154)?
        };
        let a4250 = {
            let a4247 = {
                let a4199 = {
                    let a4175 = {
                        let a4163 = {
                            let a4157 = {
                                let a4155 = v2;
                                let a4156 = 32768u32;
                                fixed_u32::bit_and(a4155, a4156)
                            };
                            let a4158 = 32768u32;
                            if equal(a4157, a4158) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a4164 = {
                            let a4161 = {
                                let a4159 = v2;
                                let a4160 = 16384u32;
                                fixed_u32::bit_and(a4159, a4160)
                            };
                            let a4162 = 16384u32;
                            if equal(a4161, a4162) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4163, a4164)?
                    };
                    let a4176 = {
                        let a4173 = {
                            let a4167 = {
                                let a4165 = v2;
                                let a4166 = 8192u32;
                                fixed_u32::bit_and(a4165, a4166)
                            };
                            let a4168 = 8192u32;
                            if equal(a4167, a4168) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a4174 = {
                            let a4171 = {
                                let a4169 = v2;
                                let a4170 = 4096u32;
                                fixed_u32::bit_and(a4169, a4170)
                            };
                            let a4172 = 4096u32;
                            if equal(a4171, a4172) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4173, a4174)?
                    };
                    nat_add(a4175, a4176)?
                };
                let a4200 = {
                    let a4197 = {
                        let a4185 = {
                            let a4179 = {
                                let a4177 = v2;
                                let a4178 = 2048u32;
                                fixed_u32::bit_and(a4177, a4178)
                            };
                            let a4180 = 2048u32;
                            if equal(a4179, a4180) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a4186 = {
                            let a4183 = {
                                let a4181 = v2;
                                let a4182 = 1024u32;
                                fixed_u32::bit_and(a4181, a4182)
                            };
                            let a4184 = 1024u32;
                            if equal(a4183, a4184) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4185, a4186)?
                    };
                    let a4198 = {
                        let a4195 = {
                            let a4189 = {
                                let a4187 = v2;
                                let a4188 = 512u32;
                                fixed_u32::bit_and(a4187, a4188)
                            };
                            let a4190 = 512u32;
                            if equal(a4189, a4190) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a4196 = {
                            let a4193 = {
                                let a4191 = v2;
                                let a4192 = 256u32;
                                fixed_u32::bit_and(a4191, a4192)
                            };
                            let a4194 = 256u32;
                            if equal(a4193, a4194) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4195, a4196)?
                    };
                    nat_add(a4197, a4198)?
                };
                nat_add(a4199, a4200)?
            };
            let a4248 = {
                let a4245 = {
                    let a4221 = {
                        let a4209 = {
                            let a4203 = {
                                let a4201 = v2;
                                let a4202 = 128u32;
                                fixed_u32::bit_and(a4201, a4202)
                            };
                            let a4204 = 128u32;
                            if equal(a4203, a4204) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4210 = {
                            let a4207 = {
                                let a4205 = v2;
                                let a4206 = 64u32;
                                fixed_u32::bit_and(a4205, a4206)
                            };
                            let a4208 = 64u32;
                            if equal(a4207, a4208) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4209, a4210)?
                    };
                    let a4222 = {
                        let a4219 = {
                            let a4213 = {
                                let a4211 = v2;
                                let a4212 = 32u32;
                                fixed_u32::bit_and(a4211, a4212)
                            };
                            let a4214 = 32u32;
                            if equal(a4213, a4214) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4220 = {
                            let a4217 = {
                                let a4215 = v2;
                                let a4216 = 16u32;
                                fixed_u32::bit_and(a4215, a4216)
                            };
                            let a4218 = 16u32;
                            if equal(a4217, a4218) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4219, a4220)?
                    };
                    nat_add(a4221, a4222)?
                };
                let a4246 = {
                    let a4243 = {
                        let a4231 = {
                            let a4225 = {
                                let a4223 = v2;
                                let a4224 = 8u32;
                                fixed_u32::bit_and(a4223, a4224)
                            };
                            let a4226 = 8u32;
                            if equal(a4225, a4226) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4232 = {
                            let a4229 = {
                                let a4227 = v2;
                                let a4228 = 4u32;
                                fixed_u32::bit_and(a4227, a4228)
                            };
                            let a4230 = 4u32;
                            if equal(a4229, a4230) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4231, a4232)?
                    };
                    let a4244 = {
                        let a4241 = {
                            let a4235 = {
                                let a4233 = v2;
                                let a4234 = 2u32;
                                fixed_u32::bit_and(a4233, a4234)
                            };
                            let a4236 = 2u32;
                            if equal(a4235, a4236) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4242 = {
                            let a4239 = {
                                let a4237 = v2;
                                let a4238 = 1u32;
                                fixed_u32::bit_and(a4237, a4238)
                            };
                            let a4240 = 1u32;
                            if equal(a4239, a4240) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4241, a4242)?
                    };
                    nat_add(a4243, a4244)?
                };
                nat_add(a4245, a4246)?
            };
            nat_add(a4247, a4248)?
        };
        nat_add(a4249, a4250)?
    }))
}

pub fn f1062() -> u64 {
    64u64
}

pub fn f1063(v0: Adt165) -> Adt166 {
    Adt166::C0(match v0.clone() {
        Adt165::C0(h4251, _, _, _, _, _, _, _) => {
            h4251
        }
    }, match v0.clone() {
        Adt165::C0(_, h4252, _, _, _, _, _, _) => {
            h4252
        }
    }, match v0.clone() {
        Adt165::C0(_, _, h4253, _, _, _, _, _) => {
            h4253
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, h4254, _, _, _, _) => {
            h4254
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, h4255, _, _, _) => {
            h4255
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, h4256, _, _) => {
            h4256
        }
    }, match v0.clone() {
        Adt165::C0(_, _, _, _, _, _, h4257, _) => {
            h4257
        }
    }, match v0 {
        Adt165::C0(_, _, _, _, _, _, _, h4258) => {
            h4258
        }
    }, f1071(), f1071(), f1071(), f1071(), f1071(), f1071(), f1071(), f1071())
}

pub fn f1064(v0: Bytes) -> R<Adt165> {
    let v1: u64 = {
        let a4259 = v0.clone();
        length_bytes(a4259)
    };
    let v2: u64 = {
        let a4265 = {
            let a4262 = {
                let a4260 = v1;
                let a4261 = 72u64;
                nat_add(a4260, a4261)?
            };
            let a4263 = 64u64;
            let a4264 = 0u64;
            nat_quot(a4262, a4263, a4264)
        };
        let a4266 = 64u64;
        nat_mul(a4265, a4266)?
    };
    let (_, h4267) = f1066(Fn12::F1074(v0, v1, v2), {
        let a4268 = v2;
        let a4269 = 64u64;
        let a4270 = 0u64;
        nat_quot(a4268, a4269, a4270)
    }, (0u64, f1056()))?;
    Ok(h4267)
}

pub fn f1065(v0: Bytes) -> R<Adt166> {
    Ok(Adt166::C0(f1075(v0.clone(), 0u64)?, f1075(v0.clone(), 4u64)?, f1075(v0.clone(), 8u64)?, f1075(v0.clone(), 12u64)?, f1075(v0.clone(), 16u64)?, f1075(v0.clone(), 20u64)?, f1075(v0.clone(), 24u64)?, f1075(v0.clone(), 28u64)?, f1075(v0.clone(), 32u64)?, f1075(v0.clone(), 36u64)?, f1075(v0.clone(), 40u64)?, f1075(v0.clone(), 44u64)?, f1075(v0.clone(), 48u64)?, f1075(v0.clone(), 52u64)?, f1075(v0.clone(), 56u64)?, f1075(v0, 60u64)?))
}

pub fn f1066(v0: Fn12, p4274: u64, p4275: (u64, Adt165)) -> R<(u64, Adt165)> {
    let m4273 = (p4274, p4275);
    {
        let mut m4273 = m4273;
        loop {
            let step = {
                let (v1, v2) = m4273;
                let m4271 = v1;
                if m4271 == 0 {
                    Err::<(u64, (u64, Adt165)), R<(u64, Adt165)>>(Ok(v2))
                } else {
                    let v3 = m4271 - 1;
                    Ok::<(u64, (u64, Adt165)), R<(u64, Adt165)>>((v3, {
                        let c4272: Fn12 = v0.clone();
                        c4272.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1067(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt165)) -> R<(u64, Adt165)> {
    Ok(({
        let a4277 = {
            let (h4276, _) = v4.clone();
            h4276
        };
        let a4278 = 64u64;
        nat_add(a4277, a4278)?
    }, f1055({
        let (_, h4279) = v4;
        h4279
    }, f1076(v0, v2, v3, {
        let a4282 = {
            let a4280 = v1;
            let a4281 = v2;
            nat_add(a4280, a4281)?
        };
        let a4283 = 8u64;
        nat_mul(a4282, a4283)?
    }, {
        let (h4284, _) = v4;
        h4284
    })?)?))
}

pub fn f1068(v0: Fn13, p4288: u64, p4289: Adt167) -> R<Adt167> {
    let m4287 = (p4288, p4289);
    {
        let mut m4287 = m4287;
        loop {
            let step = {
                let (v1, v2) = m4287;
                let m4285 = v1;
                if m4285 == 0 {
                    Err::<(u64, Adt167), R<Adt167>>(Ok(v2))
                } else {
                    let v3 = m4285 - 1;
                    Ok::<(u64, Adt167), R<Adt167>>((v3, {
                        let c4286: Fn13 = v0.clone();
                        c4286.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1069(v0: Adt167) -> R<Adt167> {
    f1077(v0)
}

pub fn f1070(v0: u32, v1: u32) -> u32 {
    let m4292 = {
        let a4290 = f1078(v0);
        let a4291 = f1078(v1);
        fixed_u64::checked_add(a4290, a4291)
    };
    match m4292 {
        None => {
            0u32
        }
        Some(v2) => {
            f1079(v2)
        }
    }
}

pub fn f1071() -> u32 {
    0u32
}

pub fn f1072(v0: Bytes, v1: u64) -> u8 {
    let m4295 = {
        let a4293 = v0;
        let a4294 = v1;
        index_bytes(a4293, a4294)
    };
    match m4295 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1073(v0: u32, v1: u32) -> u32 {
    let m4298 = {
        let a4296 = v0;
        let a4297 = v1;
        fixed_u32::shift_right(a4296, a4297)
    };
    match m4298 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1074(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt165)) -> R<(u64, Adt165)> {
    Ok(({
        let a4300 = {
            let (h4299, _) = v3.clone();
            h4299
        };
        let a4301 = 64u64;
        nat_add(a4300, a4301)?
    }, f1055({
        let (_, h4302) = v3;
        h4302
    }, f1080(v0, v1, v2, {
        let (h4303, _) = v3;
        h4303
    })?)?))
}

pub fn f1075(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f1081(f1082(v0.clone(), v1), f1082(v0.clone(), {
        let a4304 = v1;
        let a4305 = 1u64;
        nat_add(a4304, a4305)?
    }), f1082(v0.clone(), {
        let a4306 = v1;
        let a4307 = 2u64;
        nat_add(a4306, a4307)?
    }), f1082(v0, {
        let a4308 = v1;
        let a4309 = 3u64;
        nat_add(a4308, a4309)?
    })))
}

pub fn f1076(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt166> {
    Ok(Adt166::C0(f1083(v0.clone(), v1, v2, v3, v4)?, f1083(v0.clone(), v1, v2, v3, {
        let a4310 = v4;
        let a4311 = 4u64;
        nat_add(a4310, a4311)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4312 = v4;
        let a4313 = 8u64;
        nat_add(a4312, a4313)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4314 = v4;
        let a4315 = 12u64;
        nat_add(a4314, a4315)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4316 = v4;
        let a4317 = 16u64;
        nat_add(a4316, a4317)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4318 = v4;
        let a4319 = 20u64;
        nat_add(a4318, a4319)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4320 = v4;
        let a4321 = 24u64;
        nat_add(a4320, a4321)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4322 = v4;
        let a4323 = 28u64;
        nat_add(a4322, a4323)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4324 = v4;
        let a4325 = 32u64;
        nat_add(a4324, a4325)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4326 = v4;
        let a4327 = 36u64;
        nat_add(a4326, a4327)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4328 = v4;
        let a4329 = 40u64;
        nat_add(a4328, a4329)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4330 = v4;
        let a4331 = 44u64;
        nat_add(a4330, a4331)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4332 = v4;
        let a4333 = 48u64;
        nat_add(a4332, a4333)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4334 = v4;
        let a4335 = 52u64;
        nat_add(a4334, a4335)?
    })?, f1083(v0.clone(), v1, v2, v3, {
        let a4336 = v4;
        let a4337 = 56u64;
        nat_add(a4336, a4337)?
    })?, f1083(v0, v1, v2, v3, {
        let a4338 = v4;
        let a4339 = 60u64;
        nat_add(a4338, a4339)?
    })?))
}

pub fn f1077(v0: Adt167) -> R<Adt167> {
    let v1: u32 = f1084(match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, _, _, _, h4340) => {
            h4340
        }
    });
    let v2: u32 = f1070(match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, _, h4341, _, _) => {
            h4341
        }
    }, f1070(f1085(match v0.clone() {
        Adt167::C0(_, _, _, _, h4342, _, _, _, _, _) => {
            h4342
        }
    }), f1070(f1086(match v0.clone() {
        Adt167::C0(_, _, _, _, h4343, _, _, _, _, _) => {
            h4343
        }
    }, match v0.clone() {
        Adt167::C0(_, _, _, _, _, h4344, _, _, _, _) => {
            h4344
        }
    }, match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, h4345, _, _, _) => {
            h4345
        }
    }), f1070(v1, {
        let m4348 = match v0.clone() {
            Adt167::C0(_, _, _, _, _, _, _, _, h4347, _) => {
                h4347
            }
        };
        match m4348 {
            Adt166::C0(h4346, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4346
            }
        }
    }))));
    let v3: u32 = f1070(f1087(match v0.clone() {
        Adt167::C0(h4349, _, _, _, _, _, _, _, _, _) => {
            h4349
        }
    }), f1088(match v0.clone() {
        Adt167::C0(h4350, _, _, _, _, _, _, _, _, _) => {
            h4350
        }
    }, match v0.clone() {
        Adt167::C0(_, h4351, _, _, _, _, _, _, _, _) => {
            h4351
        }
    }, match v0.clone() {
        Adt167::C0(_, _, h4352, _, _, _, _, _, _, _) => {
            h4352
        }
    }));
    Ok(Adt167::C0(f1070(v2, v3), match v0.clone() {
        Adt167::C0(h4353, _, _, _, _, _, _, _, _, _) => {
            h4353
        }
    }, match v0.clone() {
        Adt167::C0(_, h4354, _, _, _, _, _, _, _, _) => {
            h4354
        }
    }, match v0.clone() {
        Adt167::C0(_, _, h4355, _, _, _, _, _, _, _) => {
            h4355
        }
    }, f1070(match v0.clone() {
        Adt167::C0(_, _, _, h4356, _, _, _, _, _, _) => {
            h4356
        }
    }, v2), match v0.clone() {
        Adt167::C0(_, _, _, _, h4357, _, _, _, _, _) => {
            h4357
        }
    }, match v0.clone() {
        Adt167::C0(_, _, _, _, _, h4358, _, _, _, _) => {
            h4358
        }
    }, match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, h4359, _, _, _) => {
            h4359
        }
    }, f1089(match v0.clone() {
        Adt167::C0(_, _, _, _, _, _, _, _, h4360, _) => {
            h4360
        }
    }), {
        let a4362 = match v0 {
            Adt167::C0(_, _, _, _, _, _, _, _, _, h4361) => {
                h4361
            }
        };
        let a4363 = 1u64;
        nat_add(a4362, a4363)?
    }))
}

pub fn f1078(v0: u32) -> u64 {
    let m4365 = {
        let a4364 = v0;
        fixed_u64::convert(i128::from(a4364))
    };
    match m4365 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1079(v0: u64) -> u32 {
    let m4369 = {
        let a4368 = {
            let a4366 = v0;
            let a4367 = 4294967295u64;
            fixed_u64::bit_and(a4366, a4367)
        };
        fixed_u32::convert(i128::from(a4368))
    };
    match m4369 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1080(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt166> {
    Ok(Adt166::C0(f1090(v0.clone(), v1, v2, v3)?, f1090(v0.clone(), v1, v2, {
        let a4370 = v3;
        let a4371 = 4u64;
        nat_add(a4370, a4371)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4372 = v3;
        let a4373 = 8u64;
        nat_add(a4372, a4373)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4374 = v3;
        let a4375 = 12u64;
        nat_add(a4374, a4375)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4376 = v3;
        let a4377 = 16u64;
        nat_add(a4376, a4377)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4378 = v3;
        let a4379 = 20u64;
        nat_add(a4378, a4379)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4380 = v3;
        let a4381 = 24u64;
        nat_add(a4380, a4381)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4382 = v3;
        let a4383 = 28u64;
        nat_add(a4382, a4383)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4384 = v3;
        let a4385 = 32u64;
        nat_add(a4384, a4385)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4386 = v3;
        let a4387 = 36u64;
        nat_add(a4386, a4387)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4388 = v3;
        let a4389 = 40u64;
        nat_add(a4388, a4389)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4390 = v3;
        let a4391 = 44u64;
        nat_add(a4390, a4391)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4392 = v3;
        let a4393 = 48u64;
        nat_add(a4392, a4393)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4394 = v3;
        let a4395 = 52u64;
        nat_add(a4394, a4395)?
    })?, f1090(v0.clone(), v1, v2, {
        let a4396 = v3;
        let a4397 = 56u64;
        nat_add(a4396, a4397)?
    })?, f1090(v0, v1, v2, {
        let a4398 = v3;
        let a4399 = 60u64;
        nat_add(a4398, a4399)?
    })?))
}

pub fn f1081(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a4404 = f1091(v0, 24u32);
    let a4405 = {
        let a4402 = f1091(v1, 16u32);
        let a4403 = {
            let a4400 = f1091(v2, 8u32);
            let a4401 = v3;
            fixed_u32::bit_or(a4400, a4401)
        };
        fixed_u32::bit_or(a4402, a4403)
    };
    fixed_u32::bit_or(a4404, a4405)
}

pub fn f1082(v0: Bytes, v1: u64) -> u32 {
    let m4408 = {
        let a4406 = v0;
        let a4407 = v1;
        index_bytes(a4406, a4407)
    };
    match m4408 {
        None => {
            0u32
        }
        Some(v2) => {
            let m4410 = {
                let a4409 = v2;
                fixed_u32::convert(i128::from(a4409))
            };
            match m4410 {
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

pub fn f1083(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a4413 = {
        let a4411 = v4;
        let a4412 = 8u64;
        nat_add(a4411, a4412)?
    };
    let a4414 = v2;
    if nat_eq(a4413, a4414) {
        Ok(f1092(v3, true))
    } else {
        let a4417 = {
            let a4415 = v4;
            let a4416 = 4u64;
            nat_add(a4415, a4416)?
        };
        let a4418 = v2;
        if nat_eq(a4417, a4418) {
            Ok(f1092(v3, false))
        } else {
            Ok(f1081(f1093(v0.clone(), v1, v4), f1093(v0.clone(), v1, {
                let a4419 = v4;
                let a4420 = 1u64;
                nat_add(a4419, a4420)?
            }), f1093(v0.clone(), v1, {
                let a4421 = v4;
                let a4422 = 2u64;
                nat_add(a4421, a4422)?
            }), f1093(v0, v1, {
                let a4423 = v4;
                let a4424 = 3u64;
                nat_add(a4423, a4424)?
            })))
        }
    }
}

pub fn f1084(v0: u64) -> u32 {
    let a4425 = v0;
    let a4426 = 32u64;
    if nat_lt(a4425, a4426) {
        let a4427 = v0;
        let a4428 = 16u64;
        if nat_lt(a4427, a4428) {
            let a4429 = v0;
            let a4430 = 8u64;
            if nat_lt(a4429, a4430) {
                let a4431 = v0;
                let a4432 = 4u64;
                if nat_lt(a4431, a4432) {
                    let a4433 = v0;
                    let a4434 = 2u64;
                    if nat_lt(a4433, a4434) {
                        let a4435 = v0;
                        let a4436 = 1u64;
                        if nat_lt(a4435, a4436) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4437 = v0;
                        let a4438 = 3u64;
                        if nat_lt(a4437, a4438) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4439 = v0;
                    let a4440 = 6u64;
                    if nat_lt(a4439, a4440) {
                        let a4441 = v0;
                        let a4442 = 5u64;
                        if nat_lt(a4441, a4442) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4443 = v0;
                        let a4444 = 7u64;
                        if nat_lt(a4443, a4444) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4445 = v0;
                let a4446 = 12u64;
                if nat_lt(a4445, a4446) {
                    let a4447 = v0;
                    let a4448 = 10u64;
                    if nat_lt(a4447, a4448) {
                        let a4449 = v0;
                        let a4450 = 9u64;
                        if nat_lt(a4449, a4450) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4451 = v0;
                        let a4452 = 11u64;
                        if nat_lt(a4451, a4452) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4453 = v0;
                    let a4454 = 14u64;
                    if nat_lt(a4453, a4454) {
                        let a4455 = v0;
                        let a4456 = 13u64;
                        if nat_lt(a4455, a4456) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4457 = v0;
                        let a4458 = 15u64;
                        if nat_lt(a4457, a4458) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4459 = v0;
            let a4460 = 24u64;
            if nat_lt(a4459, a4460) {
                let a4461 = v0;
                let a4462 = 20u64;
                if nat_lt(a4461, a4462) {
                    let a4463 = v0;
                    let a4464 = 18u64;
                    if nat_lt(a4463, a4464) {
                        let a4465 = v0;
                        let a4466 = 17u64;
                        if nat_lt(a4465, a4466) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4467 = v0;
                        let a4468 = 19u64;
                        if nat_lt(a4467, a4468) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4469 = v0;
                    let a4470 = 22u64;
                    if nat_lt(a4469, a4470) {
                        let a4471 = v0;
                        let a4472 = 21u64;
                        if nat_lt(a4471, a4472) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4473 = v0;
                        let a4474 = 23u64;
                        if nat_lt(a4473, a4474) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4475 = v0;
                let a4476 = 28u64;
                if nat_lt(a4475, a4476) {
                    let a4477 = v0;
                    let a4478 = 26u64;
                    if nat_lt(a4477, a4478) {
                        let a4479 = v0;
                        let a4480 = 25u64;
                        if nat_lt(a4479, a4480) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4481 = v0;
                        let a4482 = 27u64;
                        if nat_lt(a4481, a4482) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4483 = v0;
                    let a4484 = 30u64;
                    if nat_lt(a4483, a4484) {
                        let a4485 = v0;
                        let a4486 = 29u64;
                        if nat_lt(a4485, a4486) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4487 = v0;
                        let a4488 = 31u64;
                        if nat_lt(a4487, a4488) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4489 = v0;
        let a4490 = 48u64;
        if nat_lt(a4489, a4490) {
            let a4491 = v0;
            let a4492 = 40u64;
            if nat_lt(a4491, a4492) {
                let a4493 = v0;
                let a4494 = 36u64;
                if nat_lt(a4493, a4494) {
                    let a4495 = v0;
                    let a4496 = 34u64;
                    if nat_lt(a4495, a4496) {
                        let a4497 = v0;
                        let a4498 = 33u64;
                        if nat_lt(a4497, a4498) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4499 = v0;
                        let a4500 = 35u64;
                        if nat_lt(a4499, a4500) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4501 = v0;
                    let a4502 = 38u64;
                    if nat_lt(a4501, a4502) {
                        let a4503 = v0;
                        let a4504 = 37u64;
                        if nat_lt(a4503, a4504) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4505 = v0;
                        let a4506 = 39u64;
                        if nat_lt(a4505, a4506) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4507 = v0;
                let a4508 = 44u64;
                if nat_lt(a4507, a4508) {
                    let a4509 = v0;
                    let a4510 = 42u64;
                    if nat_lt(a4509, a4510) {
                        let a4511 = v0;
                        let a4512 = 41u64;
                        if nat_lt(a4511, a4512) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4513 = v0;
                        let a4514 = 43u64;
                        if nat_lt(a4513, a4514) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4515 = v0;
                    let a4516 = 46u64;
                    if nat_lt(a4515, a4516) {
                        let a4517 = v0;
                        let a4518 = 45u64;
                        if nat_lt(a4517, a4518) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4519 = v0;
                        let a4520 = 47u64;
                        if nat_lt(a4519, a4520) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4521 = v0;
            let a4522 = 56u64;
            if nat_lt(a4521, a4522) {
                let a4523 = v0;
                let a4524 = 52u64;
                if nat_lt(a4523, a4524) {
                    let a4525 = v0;
                    let a4526 = 50u64;
                    if nat_lt(a4525, a4526) {
                        let a4527 = v0;
                        let a4528 = 49u64;
                        if nat_lt(a4527, a4528) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4529 = v0;
                        let a4530 = 51u64;
                        if nat_lt(a4529, a4530) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4531 = v0;
                    let a4532 = 54u64;
                    if nat_lt(a4531, a4532) {
                        let a4533 = v0;
                        let a4534 = 53u64;
                        if nat_lt(a4533, a4534) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4535 = v0;
                        let a4536 = 55u64;
                        if nat_lt(a4535, a4536) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4537 = v0;
                let a4538 = 60u64;
                if nat_lt(a4537, a4538) {
                    let a4539 = v0;
                    let a4540 = 58u64;
                    if nat_lt(a4539, a4540) {
                        let a4541 = v0;
                        let a4542 = 57u64;
                        if nat_lt(a4541, a4542) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4543 = v0;
                        let a4544 = 59u64;
                        if nat_lt(a4543, a4544) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4545 = v0;
                    let a4546 = 62u64;
                    if nat_lt(a4545, a4546) {
                        let a4547 = v0;
                        let a4548 = 61u64;
                        if nat_lt(a4547, a4548) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4549 = v0;
                        let a4550 = 63u64;
                        if nat_lt(a4549, a4550) {
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

pub fn f1085(v0: u32) -> u32 {
    let a4553 = {
        let a4551 = f1094(v0, 6u32, 26u32);
        let a4552 = f1094(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4551, a4552)
    };
    let a4554 = f1094(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4553, a4554)
}

pub fn f1086(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4560 = {
        let a4555 = v0;
        let a4556 = v1;
        fixed_u32::bit_and(a4555, a4556)
    };
    let a4561 = {
        let a4558 = {
            let a4557 = v0;
            fixed_u32::bit_not(a4557)
        };
        let a4559 = v2;
        fixed_u32::bit_and(a4558, a4559)
    };
    fixed_u32::bit_xor(a4560, a4561)
}

pub fn f1087(v0: u32) -> u32 {
    let a4564 = {
        let a4562 = f1094(v0, 2u32, 30u32);
        let a4563 = f1094(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4562, a4563)
    };
    let a4565 = f1094(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4564, a4565)
}

pub fn f1088(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4574 = {
        let a4570 = {
            let a4566 = v0;
            let a4567 = v1;
            fixed_u32::bit_and(a4566, a4567)
        };
        let a4571 = {
            let a4568 = v0;
            let a4569 = v2;
            fixed_u32::bit_and(a4568, a4569)
        };
        fixed_u32::bit_xor(a4570, a4571)
    };
    let a4575 = {
        let a4572 = v1;
        let a4573 = v2;
        fixed_u32::bit_and(a4572, a4573)
    };
    fixed_u32::bit_xor(a4574, a4575)
}

pub fn f1089(v0: Adt166) -> Adt166 {
    Adt166::C0(match v0.clone() {
        Adt166::C0(_, h4576, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4576
        }
    }, match v0.clone() {
        Adt166::C0(_, _, h4577, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4577
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, h4578, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4578
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, h4579, _, _, _, _, _, _, _, _, _, _, _) => {
            h4579
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, h4580, _, _, _, _, _, _, _, _, _, _) => {
            h4580
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, h4581, _, _, _, _, _, _, _, _, _) => {
            h4581
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, h4582, _, _, _, _, _, _, _, _) => {
            h4582
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, h4583, _, _, _, _, _, _, _) => {
            h4583
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, h4584, _, _, _, _, _, _) => {
            h4584
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, h4585, _, _, _, _, _) => {
            h4585
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, h4586, _, _, _, _) => {
            h4586
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4587, _, _, _) => {
            h4587
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4588, _, _) => {
            h4588
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4589, _) => {
            h4589
        }
    }, match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4590) => {
            h4590
        }
    }, f1070(f1095(match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4591, _) => {
            h4591
        }
    }), f1070(match v0.clone() {
        Adt166::C0(_, _, _, _, _, _, _, _, _, h4592, _, _, _, _, _, _) => {
            h4592
        }
    }, f1070(f1096(match v0.clone() {
        Adt166::C0(_, h4593, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4593
        }
    }), match v0 {
        Adt166::C0(h4594, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4594
        }
    }))))
}

pub fn f1090(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4597 = {
        let a4595 = v3;
        let a4596 = 8u64;
        nat_add(a4595, a4596)?
    };
    let a4598 = v2;
    if nat_eq(a4597, a4598) {
        Ok(f1097({
            let a4601 = {
                let a4599 = v1;
                let a4600 = 8u64;
                nat_mul(a4599, a4600)?
            };
            let a4602 = 4294967296u64;
            let a4603 = 0u64;
            nat_quot(a4601, a4602, a4603)
        }))
    } else {
        let a4606 = {
            let a4604 = v3;
            let a4605 = 4u64;
            nat_add(a4604, a4605)?
        };
        let a4607 = v2;
        if nat_eq(a4606, a4607) {
            Ok(f1097({
                let a4610 = {
                    let a4608 = v1;
                    let a4609 = 8u64;
                    nat_mul(a4608, a4609)?
                };
                let a4611 = 4294967296u64;
                let a4612 = 0u64;
                nat_rem(a4610, a4611, a4612)
            }))
        } else {
            Ok(f1081(f1093(v0.clone(), v1, v3), f1093(v0.clone(), v1, {
                let a4613 = v3;
                let a4614 = 1u64;
                nat_add(a4613, a4614)?
            }), f1093(v0.clone(), v1, {
                let a4615 = v3;
                let a4616 = 2u64;
                nat_add(a4615, a4616)?
            }), f1093(v0, v1, {
                let a4617 = v3;
                let a4618 = 3u64;
                nat_add(a4617, a4618)?
            })))
        }
    }
}

pub fn f1091(v0: u32, v1: u32) -> u32 {
    let m4621 = {
        let a4619 = v0;
        let a4620 = v1;
        fixed_u32::shift_left(a4619, a4620)
    };
    match m4621 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1092(v0: u64, v1: bool) -> u32 {
    if v1 {
        f1097({
            let a4622 = v0;
            let a4623 = 4294967296u64;
            let a4624 = 0u64;
            nat_quot(a4622, a4623, a4624)
        })
    } else {
        f1097({
            let a4625 = v0;
            let a4626 = 4294967296u64;
            let a4627 = 0u64;
            nat_rem(a4625, a4626, a4627)
        })
    }
}

pub fn f1093(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4628 = v2;
    let a4629 = v1;
    if nat_lt(a4628, a4629) {
        let m4632 = {
            let a4630 = v0;
            let a4631 = v2;
            index_bytes(a4630, a4631)
        };
        match m4632 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4634 = {
                    let a4633 = v3;
                    fixed_u32::convert(i128::from(a4633))
                };
                match m4634 {
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
        let a4635 = v2;
        let a4636 = v1;
        if nat_eq(a4635, a4636) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f1094(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4637 = f1073(v0, v1);
    let a4638 = f1091(v0, v2);
    fixed_u32::bit_or(a4637, a4638)
}

pub fn f1095(v0: u32) -> u32 {
    let a4641 = {
        let a4639 = f1094(v0, 17u32, 15u32);
        let a4640 = f1094(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4639, a4640)
    };
    let a4642 = f1073(v0, 10u32);
    fixed_u32::bit_xor(a4641, a4642)
}

pub fn f1096(v0: u32) -> u32 {
    let a4645 = {
        let a4643 = f1094(v0, 7u32, 25u32);
        let a4644 = f1094(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4643, a4644)
    };
    let a4646 = f1073(v0, 3u32);
    fixed_u32::bit_xor(a4645, a4646)
}

pub fn f1097(v0: u64) -> u32 {
    let (_, h4647) = {
        let (_, h4648) = f1098(Fn14::F1099, 32u64, (v0, (1u32, 0u32)));
        h4648
    };
    h4647
}

pub fn f1098(v0: Fn14, p4652: u64, p4653: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4651 = (p4652, p4653);
    {
        let mut m4651 = m4651;
        loop {
            let step = {
                let (v1, v2) = m4651;
                let m4649 = v1;
                if m4649 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4649 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4650: Fn14 = v0.clone();
                        c4650.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4651 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1099(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4654, _) = v0;
        h4654
    };
    let v2: u32 = {
        let (h4655, _) = {
            let (_, h4656) = v0;
            h4656
        };
        h4655
    };
    let v3: u32 = {
        let (_, h4657) = {
            let (_, h4658) = v0;
            h4658
        };
        h4657
    };
    ({
        let a4659 = v1;
        let a4660 = 2u64;
        let a4661 = 0u64;
        nat_quot(a4659, a4660, a4661)
    }, (f1091(v2, 1u32), {
        let a4665 = {
            let a4662 = v1;
            let a4663 = 2u64;
            let a4664 = 0u64;
            nat_rem(a4662, a4663, a4664)
        };
        let a4666 = 1u64;
        if nat_eq(a4665, a4666) {
            let a4667 = v3;
            let a4668 = v2;
            fixed_u32::bit_or(a4667, a4668)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
