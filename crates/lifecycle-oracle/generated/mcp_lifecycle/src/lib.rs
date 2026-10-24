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
    C10,
    C11,
    C12(Str),
    C13,
    C14,
}

#[derive(Clone)]
pub enum Adt85 {
    C0(List<u64>, List<u64>, List<Adt86>, Bytes, u64, List<List<u64>>, Bytes),
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
    C0(Adt49),
    C1(Adt81),
}

#[derive(Clone)]
pub enum Adt93 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt94 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt95 {
    C0,
    C1,
    C2(Adt5),
}

#[derive(Clone)]
pub enum Adt96 {
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
pub enum Adt97 {
    C0,
    C1(Option<Adt94>, Adt98),
}

#[derive(Clone)]
pub enum Adt98 {
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
pub enum Adt99 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt100 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt101 {
    C0,
    C1(List<Adt102>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt102 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt103 {
    C0,
    C1(List<Adt102>, Str),
}

#[derive(Clone)]
pub enum Adt104 {
    C0(Adt6, Adt6, List<Adt102>),
    C1(List<Adt6>, Adt6, List<Adt102>),
    C2(List<Adt6>, Adt6, List<Adt102>, u64),
    C3(List<Adt102>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt102>),
    C5(Adt105, Adt6, Adt6, List<Adt102>),
}

#[derive(Clone)]
pub enum Adt105 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt106 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt107 {
    C0(Adt94, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt108 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt109 {
    C0(Adt103, List<Adt104>),
}

#[derive(Clone)]
pub enum Adt110 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt111 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt112>, List<Adt114>),
}

#[derive(Clone)]
pub enum Adt112 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt113),
    C7(Adt113),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt114 {
    C0(Str, List<Str>, Adt13, Adt115),
}

#[derive(Clone)]
pub enum Adt115 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt117 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt118 {
    C0(Adt119, List<Adt123>, u64),
}

#[derive(Clone)]
pub enum Adt119 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt120, List<u64>, bool),
    C7(u64, u64),
    C8(Adt121),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt121 {
    C0(bool, Adt122, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt122 {
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
pub enum Adt123 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt104>),
}

#[derive(Clone)]
pub enum Adt126 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt127 {
    C0(Adt94, Result<Adt128, Adt131>),
}

#[derive(Clone)]
pub enum Adt128 {
    C0(Adt32, Adt129, List<Adt130>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt130 {
    C0(Adt19, Adt50),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Adt132),
    C1(Str, List<Str>),
    C2(Adt112),
    C3(Str),
    C4(Adt84),
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
    C0(Option<Str>, Option<Adt112>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0,
    C1(Adt94),
    C2,
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Adt96),
    C1(Adt118),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt96),
    C1(List<Adt110>, List<u64>),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Adt94, Adt138, Adt16, Adt5),
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
    C7,
    C8,
    C9,
    C10,
    C11,
}

#[derive(Clone)]
pub enum Adt139 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt140 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt141 {
    C0,
    C1(Adt138),
}

#[derive(Clone)]
pub enum Adt142 {
    C0(Adt143),
    C1(Adt131),
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Adt94, Str, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt144 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt145 {
    C0(Adt91),
    C1(Adt124),
}

#[derive(Clone)]
pub enum Adt146 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt148 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt149 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt150 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt151 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt152 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt153 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt154 {
    C0,
    C1(Adt18),
}

#[derive(Clone)]
pub enum Adt155 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt156 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt157 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt156, u64),
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
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt92, Adt84>> {
        match self {
            Fn1::F34 => f34(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F36,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt49, Adt84>> {
        match self {
            Fn2::F36 => f36(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F74(Fn1, Adt35),
    F173(Fn2, Adt35),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt32, Adt84>> {
        match self {
            Fn3::F74(k0, k1) => f74(k0.clone(), k1.clone(), p0),
            Fn3::F173(k0, k1) => f173(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F105,
}

impl Fn4 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt46>>> {
        match self {
            Fn4::F105 => f105(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F117,
}

impl Fn5 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt58 {
        match self {
            Fn5::F117 => f117(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F119,
}

impl Fn6 {
    pub fn apply(&self, p0: Adt24) -> R<Result<List<Str>, Adt84>> {
        match self {
            Fn6::F119 => f119(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F902(Bytes),
    F927(Bytes, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn7::F902(k0) => f902(k0.clone(), p0),
            Fn7::F927(k0, k1) => f927(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F907(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn8::F907(k0) => f907(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F912(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn9::F912(k0) => f912(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F935(Bytes, u64, u64, u64),
    F942(Bytes, u64, u64),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Adt155)) -> R<(u64, Adt155)> {
        match self {
            Fn10::F935(k0, k1, k2, k3) => f935(k0.clone(), *k1, *k2, *k3, p0),
            Fn10::F942(k0, k1, k2) => f942(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F937,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt157) -> R<Adt157> {
        match self {
            Fn11::F937 => f937(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F967,
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn12::F967 => f967(p0),
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
        Adt85::C0(_, _, h27, _, _, _, _) => {
            h27
        }
    }, match v2 {
        Adt85::C0(_, _, _, _, _, h28, _) => {
            h28
        }
    })?)
}

pub fn f8() -> Adt1 {
    Adt1::C0(Str::lit("mcp-core-fixture"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt2>>)
}

pub fn f9() -> Adt4 {
    Adt4::C0(None::<Adt5>, None::<Adt5>, Some(f26()), Some(Adt8::C0(None::<bool>)), Some(Adt9::C0(None::<bool>, None::<bool>)), Some(Adt8::C0(None::<bool>)), None::<Adt5>)
}

pub fn f10() -> Adt10 {
    Adt10::C0(0u64, Adt11::C1)
}

pub fn f11(v0: List<Adt35>) -> R<List<Adt12>> {
    let m29 = f27(v0)?;
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
    let a30 = f28()?;
    let a31 = f29();
    Ok(append_list(a30, a31))
}

pub fn f13() -> u64 {
    100u64
}

pub fn f14(v0: List<Adt35>) -> R<Option<Adt84>> {
    let m32 = f27(v0)?;
    match m32 {
        Ok(_) => {
            Ok(None::<Adt84>)
        }
        Err(v1) => {
            Ok(Some(Adt84::C9(f30(v1))))
        }
    }
}

pub fn f15(v0: Adt14) -> R<Result<Adt32, Adt84>> {
    let m33 = v0;
    match m33 {
        Adt14::C0(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C1(v1) => {
            Ok(f31(f10(), f12()?, v1))
        }
        Adt14::C2(v2) => {
            if f32(match v2.clone() {
                Adt21::C0(_, h34, _, _, _) => {
                    h34
                }
            }) {
                f33(f12()?, Fn1::F34, v2)
            } else {
                f35(f12()?, Fn2::F36, v2)
            }
        }
        Adt14::C3(v4) => {
            Ok(f37(v4))
        }
        Adt14::C4(v5) => {
            f38(v5)
        }
        Adt14::C5(v6) => {
            Ok(f39(v6))
        }
        Adt14::C6(v7) => {
            Ok(f40(v7))
        }
        Adt14::C7(v8) => {
            if f41(match v8.clone() {
                Adt23::C0(_, h35, _, _, _) => {
                    h35
                }
            }) {
                Ok(f42(v8))
            } else {
                Ok(f43(v8))
            }
        }
        Adt14::C8(v9) => {
            f44(v9)
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
    let m36 = f45(v0.clone(), v1)?;
    match m36 {
        None => {
            Ok(None::<Adt89>)
        }
        Some(v2) => {
            let m42 = {
                let a39 = v0;
                let a40 = {
                    let a37 = v1;
                    let a38 = 4u64;
                    nat_add(a37, a38)?
                };
                let a41 = v2;
                slice_bytes(a39, a40, a41)
            };
            match m42 {
                None => {
                    Ok(None::<Adt89>)
                }
                Some(v3) => {
                    Ok(Some(Adt89::C0(v3, {
                        let a45 = {
                            let a43 = v1;
                            let a44 = 4u64;
                            nat_add(a43, a44)?
                        };
                        let a46 = v2;
                        nat_add(a45, a46)?
                    })))
                }
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: Adt89, v2: Option<Adt89>) -> R<Result<Adt85, Adt87>> {
    let m47 = v2;
    match m47 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v1 {
                Adt89::C0(_, h48) => {
                    h48
                }
            })))
        }
        Some(v3) => {
            f46(v0.clone(), v1, v3.clone(), f45(v0, match v3 {
                Adt89::C0(_, h49) => {
                    h49
                }
            })?)
        }
    }
}

pub fn f18(v0: u64) -> List<u64> {
    List::cons({
        let a53 = {
            let a50 = v0;
            let a51 = 16777216u64;
            let a52 = 0u64;
            nat_quot(a50, a51, a52)
        };
        let a54 = 256u64;
        let a55 = 0u64;
        nat_rem(a53, a54, a55)
    }, List::cons({
        let a59 = {
            let a56 = v0;
            let a57 = 65536u64;
            let a58 = 0u64;
            nat_quot(a56, a57, a58)
        };
        let a60 = 256u64;
        let a61 = 0u64;
        nat_rem(a59, a60, a61)
    }, List::cons({
        let a65 = {
            let a62 = v0;
            let a63 = 256u64;
            let a64 = 0u64;
            nat_quot(a62, a63, a64)
        };
        let a66 = 256u64;
        let a67 = 0u64;
        nat_rem(a65, a66, a67)
    }, List::cons({
        let a68 = v0;
        let a69 = 256u64;
        let a70 = 0u64;
        nat_rem(a68, a69, a70)
    }, List::<u64>::nil()))))
}

pub fn f19(v0: List<Adt86>) -> List<u64> {
    f47(f48(v0, List::<Adt86>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f49(f50(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt86> {
    List::cons(Adt86::C0(f51(), f52()), List::<Adt86>::nil())
}

pub fn f23() -> List<u64> {
    let a73 = f53();
    let a74 = {
        let a71 = f54();
        let a72 = f55();
        append_list(a71, a72)
    };
    append_list(a73, a74)
}

pub fn f24(v0: Adt0, v1: Fn0, v2: Adt85, v3: Option<Adt90>) -> R<Adt88> {
    let m75 = v3;
    match m75 {
        None => {
            f57(v0, v1, v2)
        }
        Some(v4) => {
            f56(v4)
        }
    }
}

pub fn f25(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m76 = f58(v0.clone(), v1.clone())?;
    match m76 {
        None => {
            f59(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f26() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f27(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt91>> {
    let m77 = f60(v0.clone());
    match m77 {
        None => {
            f61(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f28() -> R<List<Adt35>> {
    Ok(List::cons(f62(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f63()), List::cons(f62(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f64()), List::cons(f62(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f63()), List::cons(f62(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f65()), List::cons(f62(Str::lit("test_image_content"), Str::lit("Tests image content response"), f65()), List::cons(f62(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f65()), List::cons(f62(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f65()), List::cons(f62(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f65()), List::cons(f62(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f65()), List::cons(f62(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f65()), List::cons(f62(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f66()), List::cons(f62(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f67()?), List::cons(f62(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f68()), List::cons(f69(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f65(), f70()), List::cons(f69(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f65(), f70()), List::cons(f62(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f65()), List::cons(f62(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f65()), List::cons(f62(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f65()), List::<Adt35>::nil())))))))))))))))))))
}

pub fn f29() -> List<Adt35> {
    List::cons(f71(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f71(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f71(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f71(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f71(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f71(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f71(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f71(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt35>::nil()))))))))
}

pub fn f30(v0: Adt91) -> Str {
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

pub fn f31(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt84> {
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

pub fn f32(v0: Str) -> bool {
    let a107 = v0.clone();
    let a108 = Str::lit("test_input_required_result_elicitation");
    if equal(a107, a108) {
        true
    } else {
        let a109 = v0.clone();
        let a110 = Str::lit("test_input_required_result_sampling");
        if equal(a109, a110) {
            true
        } else {
            let a111 = v0.clone();
            let a112 = Str::lit("test_input_required_result_list_roots");
            if equal(a111, a112) {
                true
            } else {
                let a113 = v0.clone();
                let a114 = Str::lit("test_input_required_result_request_state");
                if equal(a113, a114) {
                    true
                } else {
                    let a115 = v0.clone();
                    let a116 = Str::lit("test_input_required_result_multiple_inputs");
                    if equal(a115, a116) {
                        true
                    } else {
                        let a117 = v0.clone();
                        let a118 = Str::lit("test_input_required_result_multi_round");
                        if equal(a117, a118) {
                            true
                        } else {
                            let a119 = v0.clone();
                            let a120 = Str::lit("test_input_required_result_tampered_state");
                            if equal(a119, a120) {
                                true
                            } else {
                                let a121 = v0;
                                let a122 = Str::lit("test_input_required_result_capabilities");
                                equal(a121, a122)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f33(v0: List<Adt35>, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m124 = f72(match v2.clone() {
        Adt21::C0(_, h123, _, _, _) => {
            h123
        }
    }, v0);
    match m124 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C0(match v2 {
                Adt21::C0(_, h125, _, _, _) => {
                    h125
                }
            })))
        }
        Some(v3) => {
            f73(v3.clone(), v2, Fn3::F74(v1, v3))
        }
    }
}

pub fn f34(v0: Adt21) -> R<Result<Adt92, Adt84>> {
    if f75(v0.clone(), Str::lit("test_input_required_result_elicitation")) {
        Ok(f76(v0))
    } else {
        if f75(v0.clone(), Str::lit("test_input_required_result_sampling")) {
            Ok(f77(v0))
        } else {
            if f75(v0.clone(), Str::lit("test_input_required_result_list_roots")) {
                f78(v0)
            } else {
                if f75(v0.clone(), Str::lit("test_input_required_result_request_state")) {
                    Ok(f79(v0))
                } else {
                    if f75(v0.clone(), Str::lit("test_input_required_result_multiple_inputs")) {
                        f80(v0)
                    } else {
                        if f75(v0.clone(), Str::lit("test_input_required_result_multi_round")) {
                            f81(v0)
                        } else {
                            if f75(v0.clone(), Str::lit("test_input_required_result_tampered_state")) {
                                Ok(f82(v0))
                            } else {
                                if f75(v0.clone(), Str::lit("test_input_required_result_capabilities")) {
                                    Ok(f83(v0))
                                } else {
                                    Ok(Err::<Adt92, Adt84>(Adt84::C9(Str::lit("not an input-required tool"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f35(v0: List<Adt35>, v1: Fn2, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m127 = f72(match v2.clone() {
        Adt21::C0(_, h126, _, _, _) => {
            h126
        }
    }, v0);
    match m127 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C0(match v2 {
                Adt21::C0(_, h128, _, _, _) => {
                    h128
                }
            })))
        }
        Some(v3) => {
            f84(v3, v1, v2)
        }
    }
}

pub fn f36(v0: Adt21) -> R<Result<Adt49, Adt84>> {
    if f85(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt49, Adt84>(f86(List::cons(f87(Str::lit("This is a simple text response for testing.")), List::<Adt38>::nil()))))
    } else {
        if f85(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt49, Adt84>(f86(List::cons(f88(), List::<Adt38>::nil()))))
        } else {
            if f85(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt49, Adt84>(f86(List::cons(f89(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt38>::nil()))))
            } else {
                if f85(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt49, Adt84>(f86(List::cons(f90(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt38>::nil()))))
                } else {
                    if f85(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt49, Adt84>(f86(List::cons(f87(Str::lit("Multiple content types test:")), List::cons(f88(), List::cons(f90(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt38>::nil()))))))
                    } else {
                        if f85(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt49, Adt84>(f91(List::cons(f87(Str::lit("This tool intentionally returns an error for testing")), List::<Adt38>::nil()), None::<Adt6>, true, List::<Adt50>::nil())))
                        } else {
                            if f85(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt49, Adt84>(f92(v0, f93()?)))
                            } else {
                                if f85(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt49, Adt84>(f92(v0, f94()?)))
                                } else {
                                    if f85(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt49, Adt84>(f91(List::cons(f87(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt38>::nil()), Some(f95()?), false, List::<Adt50>::nil())))
                                    } else {
                                        if f85(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt49, Adt84>(f91(List::cons(f87(Str::lit("{\"sum\":\"three\"}")), List::<Adt38>::nil()), Some(f96()), false, List::<Adt50>::nil())))
                                        } else {
                                            if f85(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt49, Adt84>(f86(List::cons(f97(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt38>::nil()))))
                                            } else {
                                                if f85(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt49, Adt84>(f86(List::cons(f87(f98()), List::<Adt38>::nil()))))
                                                } else {
                                                    Ok(f99(v0))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f37(v0: Adt20) -> Result<Adt32, Adt84> {
    f100(f101(), f102(), v0)
}

pub fn f38(v0: Adt22) -> R<Result<Adt32, Adt84>> {
    f103(f104(), Fn4::F105, v0)
}

pub fn f39(v0: Adt20) -> Result<Adt32, Adt84> {
    f106(f101(), f107(), v0)
}

pub fn f40(v0: Adt20) -> Result<Adt32, Adt84> {
    f108(f101(), f109(), v0)
}

pub fn f41(v0: Str) -> bool {
    let a129 = v0;
    let a130 = f110();
    equal(a129, a130)
}

pub fn f42(v0: Adt23) -> Result<Adt32, Adt84> {
    let m132 = f111(Str::lit("user_context"), Adt93::C0, match v0 {
        Adt23::C0(_, _, _, h131, _) => {
            h131
        }
    });
    match m132 {
        Adt95::C0 => {
            Ok::<Adt32, Adt84>(Adt32::C12(Adt81::C0(f114(Str::lit("user_context"), f115(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt95::C1 => {
            Err::<Adt32, Adt84>(Adt84::C11)
        }
        Adt95::C2(v1) => {
            Ok::<Adt32, Adt84>(Adt32::C8(f112(f113(v1, Str::lit("context"), Str::lit("unknown")))))
        }
    }
}

pub fn f43(v0: Adt23) -> Result<Adt32, Adt84> {
    f116(f109(), Fn5::F117, v0)
}

pub fn f44(v0: Adt24) -> R<Result<Adt32, Adt84>> {
    f118(Fn6::F119, v0)
}

pub fn f45(v0: Bytes, v1: u64) -> R<Option<u64>> {
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
                let a138 = f120(v0.clone(), v1)?;
                let a139 = 16777216u64;
                nat_mul(a138, a139)?
            };
            let a155 = {
                let a152 = {
                    let a142 = f120(v0.clone(), {
                        let a140 = v1;
                        let a141 = 1u64;
                        nat_add(a140, a141)?
                    })?;
                    let a143 = 65536u64;
                    nat_mul(a142, a143)?
                };
                let a153 = {
                    let a150 = {
                        let a146 = f120(v0.clone(), {
                            let a144 = v1;
                            let a145 = 2u64;
                            nat_add(a144, a145)?
                        })?;
                        let a147 = 256u64;
                        nat_mul(a146, a147)?
                    };
                    let a151 = f120(v0, {
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

pub fn f46(v0: Bytes, v1: Adt89, v2: Adt89, v3: Option<u64>) -> R<Result<Adt85, Adt87>> {
    let m156 = v3;
    match m156 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v2 {
                Adt89::C0(_, h157) => {
                    h157
                }
            })))
        }
        Some(v4) => {
            let m161 = f121(v0.clone(), {
                let a159 = match v2.clone() {
                    Adt89::C0(_, h158) => {
                        h158
                    }
                };
                let a160 = 4u64;
                nat_add(a159, a160)?
            }, v4, List::<Adt86>::nil())?;
            match m161 {
                Ok(v6) => {
                    f122(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt85, Adt87>(v5))
                }
            }
        }
    }
}

pub fn f47(p166: List<Adt86>, p167: List<u64>) -> List<u64> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v0, v1) = m165;
                let m162 = v0;
                match m162.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<u64>), List<u64>>((v3, {
                            let a163 = f123(v2);
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

pub fn f48(p170: List<Adt86>, p171: List<Adt86>) -> List<Adt86> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v0, v1) = m169;
                let m168 = v0;
                match m168.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
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

pub fn f49(p176: List<List<u64>>, p177: List<u64>) -> List<u64> {
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
                            let a173 = f124(v2);
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

pub fn f50(p180: List<List<u64>>, p181: List<List<u64>>) -> List<List<u64>> {
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

pub fn f51() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f52() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f53() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f54() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f55() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f56(v0: Adt90) -> R<Adt88> {
    f125(f126(v0.clone()), f127(None::<Adt94>, f128(v0)))
}

pub fn f57(v0: Adt0, v1: Fn0, v2: Adt85) -> R<Adt88> {
    if f129(match v2.clone() {
        Adt85::C0(h182, _, _, _, _, _, _) => {
            h182
        }
    }) {
        f130(f131(v0, match v2.clone() {
            Adt85::C0(_, _, _, _, _, _, h183) => {
                h183
            }
        }), v1, match v2.clone() {
            Adt85::C0(_, _, _, _, h184, _, _) => {
                h184
            }
        }, match v2.clone() {
            Adt85::C0(_, _, h185, _, _, _, _) => {
                h185
            }
        }, f132(match v2 {
            Adt85::C0(_, _, _, h186, _, _, _) => {
                h186
            }
        })?)
    } else {
        Ok(f133())
    }
}

pub fn f58(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m187 = f134(v0, f135())?;
    match m187 {
        Adt99::C0 => {
            Ok(None::<Adt90>)
        }
        Adt99::C1 => {
            Ok(Some(Adt90::C0))
        }
        Adt99::C2 => {
            Ok(Some(Adt90::C0))
        }
        Adt99::C3(v2) => {
            f136(f137(v2), v1, Adt90::C0)
        }
    }
}

pub fn f59(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m188 = f134(v0, f138())?;
    match m188 {
        Adt99::C0 => {
            Ok(None::<Adt90>)
        }
        Adt99::C1 => {
            Ok(Some(Adt90::C1))
        }
        Adt99::C2 => {
            Ok(Some(Adt90::C1))
        }
        Adt99::C3(v2) => {
            f136(f139(v2), v1, Adt90::C1)
        }
    }
}

pub fn f60(v0: List<Adt35>) -> Option<Adt91> {
    let m189 = f140(v0, List::<Str>::nil());
    match m189 {
        None => {
            None::<Adt91>
        }
        Some(v1) => {
            Some(Adt91::C1(v1))
        }
    }
}

pub fn f61(p193: List<Adt35>, p194: List<Adt12>) -> R<Result<List<Adt12>, Adt91>> {
    let m192 = (p193, p194);
    {
        let mut m192 = m192;
        loop {
            let step = {
                let (v0, v1) = m192;
                let m190 = v0;
                match m190.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Ok::<List<Adt12>, Adt91>(f141(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m191 = f142(v2)?;
                        match m191 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>((v3, f141(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Err::<List<Adt12>, Adt91>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m192 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f63() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f64() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f143(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f144()), List::cons((Str::lit("limit"), f143(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f143(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f65() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f66() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f67() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f144()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f145(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f145(0u64)?), List::cons((Str::lit("maximum"), f145(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f145(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
}

pub fn f68() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("target"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("zone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt6::C3(Str::lit("Zone"))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f69(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt36>, None::<Adt5>)
}

pub fn f70() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("sum"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("sum")), List::cons(Adt6::C3(Str::lit("label")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f71(v0: Str, v1: Str) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))), None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f72(v0: Str, p200: List<Adt35>) -> Option<Adt35> {
    let m199 = p200;
    {
        let mut m199 = m199;
        loop {
            let step = {
                let v1 = m199;
                let m195 = v1;
                match m195.uncons() {
                    None => {
                        Err::<List<Adt35>, Option<Adt35>>(None::<Adt35>)
                    }
                    Some((v2, v3)) => {
                        let a197 = match v2.clone() {
                            Adt35::C0(h196, _, _, _, _, _, _, _) => {
                                h196
                            }
                        };
                        let a198 = v0.clone();
                        if equal(a197, a198) {
                            Err::<List<Adt35>, Option<Adt35>>(Some(v2))
                        } else {
                            Ok::<List<Adt35>, Option<Adt35>>(v3)
                        }
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

pub fn f73(v0: Adt35, v1: Adt21, v2: Fn3) -> R<Result<Adt32, Adt84>> {
    let m207 = f146(Adt6::C5({
        let m203 = match v0.clone() {
            Adt35::C0(_, _, _, _, h202, _, _, _) => {
                h202
            }
        };
        match m203 {
            Adt5::C0(h201) => {
                h201
            }
        }
    }), Adt6::C5({
        let m206 = match v0 {
            Adt35::C0(_, _, _, _, h205, _, _, _) => {
                h205
            }
        };
        match m206 {
            Adt5::C0(h204) => {
                h204
            }
        }
    }), f147(v1.clone()))?;
    match m207 {
        Adt101::C0 => {
            let c208: Fn3 = v2;
            c208.apply(v1)
        }
        Adt101::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f148(f149(v3, v4)?, true))))
        }
        Adt101::C2(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt101::C3(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt101::C4 => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f74(v0: Fn1, v1: Adt35, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    f150(v1, {
        let c209: Fn1 = v0;
        c209.apply(v2)?
    })
}

pub fn f75(v0: Adt21, v1: Str) -> bool {
    let a211 = match v0 {
        Adt21::C0(_, h210, _, _, _) => {
            h210
        }
    };
    let a212 = v1;
    equal(a211, a212)
}

pub fn f76(v0: Adt21) -> Result<Adt92, Adt84> {
    let m214 = f111(Str::lit("user_name"), Adt93::C0, match v0 {
        Adt21::C0(_, _, _, h213, _) => {
            h213
        }
    });
    match m214 {
        Adt95::C0 => {
            f153(f114(Str::lit("user_name"), f154()))
        }
        Adt95::C1 => {
            f155()
        }
        Adt95::C2(v1) => {
            f151(f152(List::cons(Str::lit("Hello, "), List::cons(f113(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f77(v0: Adt21) -> Result<Adt92, Adt84> {
    let m216 = f111(Str::lit("capital_question"), Adt93::C1, match v0 {
        Adt21::C0(_, _, _, h215, _) => {
            h215
        }
    });
    match m216 {
        Adt95::C0 => {
            f153(f114(Str::lit("capital_question"), f156(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt95::C1 => {
            f155()
        }
        Adt95::C2(v1) => {
            f151(f152(List::cons(Str::lit("Sampling result: "), List::cons(f113(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f78(v0: Adt21) -> R<Result<Adt92, Adt84>> {
    let m218 = f111(Str::lit("client_roots"), Adt93::C2, match v0 {
        Adt21::C0(_, _, _, h217, _) => {
            h217
        }
    });
    match m218 {
        Adt95::C0 => {
            Ok(f153(f114(Str::lit("client_roots"), f159())))
        }
        Adt95::C1 => {
            Ok(f155())
        }
        Adt95::C2(v1) => {
            Ok(f151(f152(List::cons(Str::lit("Found "), List::cons(f157(f158(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f79(v0: Adt21) -> Result<Adt92, Adt84> {
    f160(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f80(v0: Adt21) -> R<Result<Adt92, Adt84>> {
    if f161(v0.clone(), Str::lit("multiple-inputs")) {
        f162(f111(Str::lit("user_name"), Adt93::C0, match v0.clone() {
            Adt21::C0(_, _, _, h219, _) => {
                h219
            }
        }), f111(Str::lit("greeting"), Adt93::C1, match v0.clone() {
            Adt21::C0(_, _, _, h220, _) => {
                h220
            }
        }), f111(Str::lit("client_roots"), Adt93::C2, match v0 {
            Adt21::C0(_, _, _, h221, _) => {
                h221
            }
        }))
    } else {
        Ok(f163(f164(Adt95::C0, Adt95::C0, Adt95::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f81(v0: Adt21) -> R<Result<Adt92, Adt84>> {
    let v1: u64 = f165(f166(v0.clone()))?;
    let a222 = v1;
    let a223 = 49u64;
    if nat_eq(a222, a223) {
        Ok(f167(v0))
    } else {
        let a224 = v1;
        let a225 = 50u64;
        if nat_eq(a224, a225) {
            Ok(f168(v0))
        } else {
            Ok(f163(f114(Str::lit("step1"), f169()), Str::lit("1")))
        }
    }
}

pub fn f82(v0: Adt21) -> Result<Adt92, Adt84> {
    f160(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f83(v0: Adt21) -> Result<Adt92, Adt84> {
    let m227 = match v0.clone() {
        Adt21::C0(_, _, _, h226, _) => {
            h226
        }
    };
    match m227 {
        None => {
            f170(f171({
                let m230 = match v0 {
                    Adt21::C0(h229, _, _, _, _) => {
                        h229
                    }
                };
                match m230 {
                    Adt16::C0(_, h228, _, _, _, _) => {
                        h228
                    }
                }
            }))
        }
        Some(v1) => {
            let m232 = match v1.clone() {
                Adt5::C0(h231) => {
                    h231
                }
            };
            match m232.uncons() {
                None => {
                    f170(f171({
                        let m235 = match v0 {
                            Adt21::C0(h234, _, _, _, _) => {
                                h234
                            }
                        };
                        match m235 {
                            Adt16::C0(_, h233, _, _, _, _) => {
                                h233
                            }
                        }
                    }))
                }
                Some((_, _)) => {
                    f151(f152(List::cons(Str::lit("capabilities-ok: received "), List::cons({
                        let a237 = f172(match v1 {
                            Adt5::C0(h236) => {
                                h236
                            }
                        });
                        let a238 = Str::lit(",");
                        join(a237, a238)
                    }, List::<Str>::nil()))))
                }
            }
        }
    }
}

pub fn f84(v0: Adt35, v1: Fn2, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    f73(v0.clone(), v2, Fn3::F173(v1, v0))
}

pub fn f85(v0: Adt21, v1: Str) -> bool {
    let a240 = match v0 {
        Adt21::C0(_, h239, _, _, _) => {
            h239
        }
    };
    let a241 = v1;
    equal(a240, a241)
}

pub fn f86(v0: List<Adt38>) -> Adt49 {
    f91(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f87(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f88() -> Adt38 {
    f97(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f89(v0: Str, v1: Str) -> Adt38 {
    Adt38::C2(Adt43::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f90(v0: Str, v1: Str, v2: Str) -> Adt38 {
    Adt38::C4(Adt45::C0(Adt46::C0(Adt47::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt40>, None::<Adt5>))
}

pub fn f91(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f92(v0: Adt21, v1: List<Adt50>) -> Adt49 {
    f91(List::cons(f87(f174({
        let m244 = match v0 {
            Adt21::C0(h243, _, _, _, _) => {
                h243
            }
        };
        match m244 {
            Adt16::C0(_, _, _, _, h242, _) => {
                h242
            }
        }
    })), List::<Adt38>::nil()), None::<Adt6>, false, v1)
}

pub fn f93() -> R<List<Adt50>> {
    Ok(List::cons(f175(0u64, 100u64)?, List::cons(f175(50u64, 100u64)?, List::cons(f175(100u64, 100u64)?, List::<Adt50>::nil()))))
}

pub fn f94() -> R<List<Adt50>> {
    Ok(List::cons(f175(50u64, 100u64)?, List::cons(f175(10u64, 100u64)?, List::<Adt50>::nil())))
}

pub fn f95() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f145(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f96() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f97(v0: Str, v1: Str) -> Adt38 {
    Adt38::C1(Adt42::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f98() -> Str {
    let m246 = {
        let a245 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a245)
    };
    match m246 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f99(v0: Adt21) -> Result<Adt49, Adt84> {
    if f85(v0.clone(), Str::lit("test_missing_capability")) {
        let m252 = {
            let m251 = {
                let m250 = match v0 {
                    Adt21::C0(h249, _, _, _, _) => {
                        h249
                    }
                };
                match m250 {
                    Adt16::C0(_, h248, _, _, _, _) => {
                        h248
                    }
                }
            };
            match m251 {
                Adt17::C0(_, _, h247, _, _) => {
                    h247
                }
            }
        };
        match m252 {
            None => {
                Err::<Adt49, Adt84>(Adt84::C8(f176()))
            }
            Some(_) => {
                Ok::<Adt49, Adt84>(f177())
            }
        }
    } else {
        if f85(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt49, Adt84>(f86(List::cons(f87(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt38>::nil())))
        } else {
            if f85(v0, Str::lit("test_arguments")) {
                Ok::<Adt49, Adt84>(f86(List::cons(f87(Str::lit("arguments accepted")), List::<Adt38>::nil())))
            } else {
                Ok::<Adt49, Adt84>(f177())
            }
        }
    }
}

pub fn f100(v0: Adt10, v1: List<Adt44>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m254 = match v2 {
        Adt20::C0(_, h253) => {
            h253
        }
    };
    match m254 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C4(Adt51::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f101() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C0)
}

pub fn f102() -> List<Adt44> {
    f178(f179())
}

pub fn f103(v0: Adt10, v1: Fn4, v2: Adt22) -> R<Result<Adt32, Adt84>> {
    Ok(f180(v0, match v2.clone() {
        Adt22::C0(_, h255, _, _) => {
            h255
        }
    }, {
        let c256: Fn4 = v1;
        c256.apply(match v2 {
            Adt22::C0(_, h257, _, _) => {
                h257
            }
        })?
    }))
}

pub fn f104() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C1)
}

pub fn f105(v0: Str) -> R<Option<List<Adt46>>> {
    let m258 = f181(v0.clone(), f179());
    match m258 {
        None => {
            f182(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f106(v0: Adt10, v1: List<Adt54>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m260 = match v2 {
        Adt20::C0(_, h259) => {
            h259
        }
    };
    match m260 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C6(Adt53::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f107() -> List<Adt54> {
    List::cons(Adt54::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt2>>, f183(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt40>, None::<Adt5>), List::<Adt54>::nil())
}

pub fn f108(v0: Adt10, v1: List<Adt56>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m262 = match v2 {
        Adt20::C0(_, h261) => {
            h261
        }
    };
    match m262 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C7(Adt55::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f109() -> List<Adt56> {
    List::cons(f184(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt57>>), List::cons(f184(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f185(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f185(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt57>::nil())))), List::cons(f184(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f185(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt57>::nil()))), List::cons(f184(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt57>>), List::<Adt56>::nil()))))
}

pub fn f110() -> Str {
    Str::lit("test_input_required_result_prompt")
}

pub fn f111(v0: Str, v1: Adt93, v2: Option<Adt5>) -> Adt95 {
    let m263 = v2;
    match m263 {
        None => {
            Adt95::C0
        }
        Some(v3) => {
            let m264 = f186(v0, v3);
            match m264 {
                Adt106::C0 => {
                    Adt95::C0
                }
                Adt106::C1(v4) => {
                    let m265 = f187(v4);
                    match m265 {
                        None => {
                            Adt95::C1
                        }
                        Some(v5) => {
                            if f188(v1, v5.clone()) {
                                Adt95::C2(v5)
                            } else {
                                Adt95::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f112(v0: Str) -> Adt58 {
    Adt58::C0(None::<Str>, List::cons(Adt59::C0(Adt41::C0, Adt38::C0(Adt39::C0(f152(List::cons(Str::lit("Prompt with context: "), List::cons(v0, List::<Str>::nil()))), None::<Adt40>, None::<Adt5>))), List::<Adt59>::nil()))
}

pub fn f113(v0: Adt5, v1: Str, v2: Str) -> Str {
    let m266 = f186(Str::lit("content"), v0);
    match m266 {
        Adt106::C0 => {
            v2
        }
        Adt106::C1(v3) => {
            let m267 = f187(v3);
            match m267 {
                None => {
                    v2
                }
                Some(v4) => {
                    f189(v1, v4, v2)
                }
            }
        }
    }
}

pub fn f114(v0: Str, v1: Adt64) -> List<(Str, Adt64)> {
    List::cons((v0, v1), List::<(Str, Adt64)>::nil())
}

pub fn f115(v0: Str, v1: Str, v2: Str) -> Adt64 {
    Adt64::C0(Adt65::C0(Adt66::C0(None::<Adt67>, v0, f190(v1, v2))))
}

pub fn f116(v0: List<Adt56>, v1: Fn5, v2: Adt23) -> Result<Adt32, Adt84> {
    let m269 = f191(match v2.clone() {
        Adt23::C0(_, h268, _, _, _) => {
            h268
        }
    }, v0);
    match m269 {
        None => {
            Err::<Adt32, Adt84>(Adt84::C1(match v2 {
                Adt23::C0(_, h270, _, _, _) => {
                    h270
                }
            }))
        }
        Some(v3) => {
            let m272 = f192(f193(v3), f194(match v2.clone() {
                Adt23::C0(_, _, h271, _, _) => {
                    h271
                }
            }));
            match m272 {
                None => {
                    Ok::<Adt32, Adt84>(Adt32::C8({
                        let c275: Fn5 = v1;
                        c275.apply(match v2.clone() {
                            Adt23::C0(_, h276, _, _, _) => {
                                h276
                            }
                        }, f194(match v2 {
                            Adt23::C0(_, _, h277, _, _) => {
                                h277
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt32, Adt84>(Adt84::C4({
                        let a273 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a274 = Str::lit(" ");
                        join(a273, a274)
                    }))
                }
            }
        }
    }
}

pub fn f117(v0: Str, v1: List<(Str, Str)>) -> Adt58 {
    let a278 = v0.clone();
    let a279 = Str::lit("test_simple_prompt");
    if equal(a278, a279) {
        f195(List::cons(f196(Str::lit("This is a simple prompt for testing.")), List::<Adt59>::nil()))
    } else {
        let a280 = v0.clone();
        let a281 = Str::lit("test_prompt_with_arguments");
        if equal(a280, a281) {
            f195(List::cons(f196({
                let a282 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f197(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f197(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a283 = Str::lit("");
                join(a282, a283)
            }), List::<Adt59>::nil()))
        } else {
            let a284 = v0.clone();
            let a285 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a284, a285) {
                f195(List::cons(Adt59::C0(Adt41::C0, Adt38::C4(Adt45::C0(f198(f197(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt40>, None::<Adt5>))), List::cons(f196(Str::lit("Please process the embedded resource above.")), List::<Adt59>::nil())))
            } else {
                let a286 = v0;
                let a287 = Str::lit("test_prompt_with_image");
                if equal(a286, a287) {
                    f195(List::cons(Adt59::C0(Adt41::C0, Adt38::C1(Adt42::C0(f199(), Str::lit("image/png"), None::<Adt40>, None::<Adt5>))), List::cons(f196(Str::lit("Please analyze the image above.")), List::<Adt59>::nil())))
                } else {
                    f195(List::<Adt59>::nil())
                }
            }
        }
    }
}

pub fn f118(v0: Fn6, v1: Adt24) -> R<Result<Adt32, Adt84>> {
    let m289 = {
        let c288: Fn6 = v0;
        c288.apply(v1)?
    };
    match m289 {
        Ok(v3) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C9(f200(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt32, Adt84>(v2))
        }
    }
}

pub fn f119(v0: Adt24) -> R<Result<List<Str>, Adt84>> {
    let m291 = match v0.clone() {
        Adt24::C0(_, h290, _, _) => {
            h290
        }
    };
    match m291 {
        Adt25::C0(v1) => {
            let m293 = f191(match v1.clone() {
                Adt26::C0(h292, _) => {
                    h292
                }
            }, f109());
            match m293 {
                None => {
                    Ok(Err::<List<Str>, Adt84>(Adt84::C1(match v1 {
                        Adt26::C0(h294, _) => {
                            h294
                        }
                    })))
                }
                Some(v2) => {
                    if f201(f193(v2), {
                        let m297 = match v0.clone() {
                            Adt24::C0(_, _, h296, _) => {
                                h296
                            }
                        };
                        match m297 {
                            Adt27::C0(h295, _) => {
                                h295
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt84>(f202({
                            let m300 = match v0.clone() {
                                Adt24::C0(_, _, h299, _) => {
                                    h299
                                }
                            };
                            match m300 {
                                Adt27::C0(_, h298) => {
                                    h298
                                }
                            }
                        }, f203(match v1 {
                            Adt26::C0(h301, _) => {
                                h301
                            }
                        }, {
                            let m304 = match v0 {
                                Adt24::C0(_, _, h303, _) => {
                                    h303
                                }
                            };
                            match m304 {
                                Adt27::C0(h302, _) => {
                                    h302
                                }
                            }
                        }))?))
                    } else {
                        Ok(Err::<List<Str>, Adt84>(Adt84::C4(Str::lit("the prompt declares no such argument"))))
                    }
                }
            }
        }
        Adt25::C1(v3) => {
            let a305 = v3.clone();
            let a306 = f183();
            if equal(a305, a306) {
                let a310 = {
                    let m309 = match v0.clone() {
                        Adt24::C0(_, _, h308, _) => {
                            h308
                        }
                    };
                    match m309 {
                        Adt27::C0(h307, _) => {
                            h307
                        }
                    }
                };
                let a311 = Str::lit("id");
                if equal(a310, a311) {
                    Ok(Ok::<List<Str>, Adt84>(f202({
                        let m314 = match v0 {
                            Adt24::C0(_, _, h313, _) => {
                                h313
                            }
                        };
                        match m314 {
                            Adt27::C0(_, h312) => {
                                h312
                            }
                        }
                    }, f204())?))
                } else {
                    Ok(Err::<List<Str>, Adt84>(Adt84::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt84>(Adt84::C2(v3)))
            }
        }
    }
}

pub fn f120(v0: Bytes, v1: u64) -> R<u64> {
    let m317 = {
        let a315 = v0;
        let a316 = v1;
        index_bytes(a315, a316)
    };
    match m317 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a362 = {
                let a338 = {
                    let a326 = {
                        let a320 = {
                            let a318 = v3;
                            let a319 = 128u8;
                            fixed_u8::bit_and(a318, a319)
                        };
                        let a321 = 128u8;
                        if equal(a320, a321) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a327 = {
                        let a324 = {
                            let a322 = v3;
                            let a323 = 64u8;
                            fixed_u8::bit_and(a322, a323)
                        };
                        let a325 = 64u8;
                        if equal(a324, a325) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a326, a327)?
                };
                let a339 = {
                    let a336 = {
                        let a330 = {
                            let a328 = v3;
                            let a329 = 32u8;
                            fixed_u8::bit_and(a328, a329)
                        };
                        let a331 = 32u8;
                        if equal(a330, a331) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a337 = {
                        let a334 = {
                            let a332 = v3;
                            let a333 = 16u8;
                            fixed_u8::bit_and(a332, a333)
                        };
                        let a335 = 16u8;
                        if equal(a334, a335) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a336, a337)?
                };
                nat_add(a338, a339)?
            };
            let a363 = {
                let a360 = {
                    let a348 = {
                        let a342 = {
                            let a340 = v3;
                            let a341 = 8u8;
                            fixed_u8::bit_and(a340, a341)
                        };
                        let a343 = 8u8;
                        if equal(a342, a343) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a349 = {
                        let a346 = {
                            let a344 = v3;
                            let a345 = 4u8;
                            fixed_u8::bit_and(a344, a345)
                        };
                        let a347 = 4u8;
                        if equal(a346, a347) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a348, a349)?
                };
                let a361 = {
                    let a358 = {
                        let a352 = {
                            let a350 = v3;
                            let a351 = 2u8;
                            fixed_u8::bit_and(a350, a351)
                        };
                        let a353 = 2u8;
                        if equal(a352, a353) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a359 = {
                        let a356 = {
                            let a354 = v3;
                            let a355 = 1u8;
                            fixed_u8::bit_and(a354, a355)
                        };
                        let a357 = 1u8;
                        if equal(a356, a357) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a358, a359)?
                };
                nat_add(a360, a361)?
            };
            nat_add(a362, a363)
        }
    }
}

pub fn f121(v0: Bytes, p373: u64, p374: u64, p375: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m372 = (p373, (p374, p375));
    {
        let mut m372 = m372;
        loop {
            let step = {
                let (v1, (v2, v3)) = m372;
                let m364 = v2;
                if m364 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f205(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m364 - 1;
                    let m365 = f16(v0.clone(), v1)?;
                    match m365 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m367 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h366) => {
                                    h366
                                }
                            })?;
                            match m367 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h368) => {
                                            h368
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h369) => {
                                            h369
                                        }
                                    }, (v4, List::cons(Adt86::C0(f206(match v5 {
                                        Adt89::C0(h370, _) => {
                                            h370
                                        }
                                    })?, f206(match v6 {
                                        Adt89::C0(h371, _) => {
                                            h371
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m377 = f16(v0.clone(), {
        let (_, h376) = v3.clone();
        h376
    })?;
    match m377 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h378) = v3;
                h378
            })))
        }
        Some(v4) => {
            let m380 = f45(v0.clone(), match v4.clone() {
                Adt89::C0(_, h379) => {
                    h379
                }
            })?;
            match m380 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h381) => {
                            h381
                        }
                    })))
                }
                Some(v5) => {
                    let m385 = f16(v0.clone(), {
                        let a383 = match v4.clone() {
                            Adt89::C0(_, h382) => {
                                h382
                            }
                        };
                        let a384 = 4u64;
                        nat_add(a383, a384)?
                    })?;
                    match m385 {
                        None => {
                            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                                let a387 = match v4 {
                                    Adt89::C0(_, h386) => {
                                        h386
                                    }
                                };
                                let a388 = 4u64;
                                nat_add(a387, a388)?
                            })))
                        }
                        Some(v6) => {
                            f207(v0, Adt85::C0(f206(match v1 {
                                Adt89::C0(h389, _) => {
                                    h389
                                }
                            })?, f206(match v2 {
                                Adt89::C0(h390, _) => {
                                    h390
                                }
                            })?, {
                                let (h391, _) = v3;
                                h391
                            }, match v4 {
                                Adt89::C0(h392, _) => {
                                    h392
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt89::C0(h393, _) => {
                                    h393
                                }
                            }), match v6 {
                                Adt89::C0(_, h394) => {
                                    h394
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f123(v0: Adt86) -> List<u64> {
    let a405 = f18({
        let a396 = match v0.clone() {
            Adt86::C0(h395, _) => {
                h395
            }
        };
        length_list(a396)
    });
    let a406 = {
        let a403 = match v0.clone() {
            Adt86::C0(h397, _) => {
                h397
            }
        };
        let a404 = {
            let a401 = f18({
                let a399 = match v0.clone() {
                    Adt86::C0(_, h398) => {
                        h398
                    }
                };
                length_list(a399)
            });
            let a402 = match v0 {
                Adt86::C0(_, h400) => {
                    h400
                }
            };
            append_list(a401, a402)
        };
        append_list(a403, a404)
    };
    append_list(a405, a406)
}

pub fn f124(v0: List<u64>) -> List<u64> {
    let a408 = f18({
        let a407 = v0.clone();
        length_list(a407)
    });
    let a409 = v0;
    append_list(a408, a409)
}

pub fn f125(v0: u64, v1: Adt6) -> R<Adt88> {
    let m410 = f208(v1)?;
    match m410 {
        Ok(v3) => {
            Ok(Adt88::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f126(v0: Adt90) -> u64 {
    let m411 = v0;
    match m411 {
        Adt90::C0 => {
            f209()
        }
        Adt90::C1 => {
            f209()
        }
    }
}

pub fn f127(v0: Option<Adt94>, v1: Adt80) -> Adt6 {
    Adt6::C5(f210(v0, v1))
}

pub fn f128(v0: Adt90) -> Adt80 {
    Adt80::C0(f211(), f212(v0), None::<Adt6>)
}

pub fn f129(v0: List<u64>) -> bool {
    f213(v0, f214())
}

pub fn f130(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt96>) -> R<Adt88> {
    let m412 = v4;
    match m412 {
        Ok(v6) => {
            f217(v0, v1, v2, v3, f218(v6)?)
        }
        Err(_) => {
            f125(f215(), f127(None::<Adt94>, f216(Adt97::C0)))
        }
    }
}

pub fn f131(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h413, _, _, _, _, _, _) => {
            h413
        }
    }, match v0.clone() {
        Adt0::C0(_, h414, _, _, _, _, _) => {
            h414
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h415, _, _, _, _) => {
            h415
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h416, _, _, _) => {
            h416
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h417, _, _) => {
            h417
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, h418, _) => {
            h418
        }
    }, v1)
}

pub fn f132(v0: Bytes) -> R<Result<Adt6, Adt96>> {
    f219(f206(v0)?)
}

pub fn f133() -> Adt88 {
    Adt88::C0(f220(), List::cons(Adt86::C0(f221(), f222()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f134(v0: List<Adt86>, v1: List<u64>) -> R<Adt99> {
    let m419 = f223(v0, v1, Adt100::C0)?;
    match m419 {
        Adt100::C0 => {
            Ok(Adt99::C0)
        }
        Adt100::C1(v2) => {
            let m420 = f224(v2);
            match m420 {
                None => {
                    Ok(Adt99::C1)
                }
                Some(v3) => {
                    Ok(Adt99::C3(v3))
                }
            }
        }
        Adt100::C2 => {
            Ok(Adt99::C2)
        }
    }
}

pub fn f135() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f136(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt90) -> R<Option<Adt90>> {
    let m421 = v0;
    match m421 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f225(v3, v1)? {
                Ok(None::<Adt90>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f137(v0: List<u64>) -> Option<List<u64>> {
    let m422 = f226(v0);
    match m422 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f139(v1)
        }
    }
}

pub fn f138() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f139(v0: List<u64>) -> Option<List<u64>> {
    let m423 = v0.clone();
    match m423.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a424 = v1;
            let a425 = 91u64;
            if nat_eq(a424, a425) {
                f227(f228(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f227(f229(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f140(p431: List<Adt35>, p432: List<Str>) -> Option<Str> {
    let m430 = (p431, p432);
    {
        let mut m430 = m430;
        loop {
            let step = {
                let (v0, v1) = m430;
                let m426 = v0;
                match m426.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f230(match v2.clone() {
                            Adt35::C0(h427, _, _, _, _, _, _, _) => {
                                h427
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h428, _, _, _, _, _, _, _) => {
                                    h428
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h429, _, _, _, _, _, _, _) => {
                                    h429
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m430 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(p435: List<Adt12>, p436: List<Adt12>) -> List<Adt12> {
    let m434 = (p435, p436);
    {
        let mut m434 = m434;
        loop {
            let step = {
                let (v0, v1) = m434;
                let m433 = v0;
                match m433.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
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

pub fn f142(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m439 = f231(match v0.clone() {
        Adt35::C0(h437, _, _, _, _, _, _, _) => {
            h437
        }
    }, f232(match v0.clone() {
        Adt35::C0(h438, _, _, _, _, _, _, _) => {
            h438
        }
    })?);
    match m439 {
        None => {
            f233(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f143(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f144() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f145(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f234(v0, 0i64)?, 0i64)))
}

pub fn f146(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt101> {
    f235(v0, f236(), Adt103::C0, List::cons(Adt104::C0(v1, v2, List::<Adt102>::nil()), List::<Adt104>::nil()))
}

pub fn f147(v0: Adt21) -> Adt6 {
    let m441 = match v0 {
        Adt21::C0(_, _, h440, _, _) => {
            h440
        }
    };
    match m441 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h442) => {
                    h442
                }
            })
        }
    }
}

pub fn f148(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f149(v0: List<Adt102>, v1: Str) -> R<Str> {
    let v2: Str = f237(v0)?;
    let a445 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a443 = v2.clone();
        let a444 = Str::lit("");
        if equal(a443, a444) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a446 = Str::lit(" ");
    Ok(join(a445, a446))
}

pub fn f150(v0: Adt35, v1: Result<Adt92, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m447 = v1;
    match m447 {
        Ok(v3) => {
            let m448 = v3;
            match m448 {
                Adt92::C0(v4) => {
                    f238(v0, v4)
                }
                Adt92::C1(v5) => {
                    Ok(Ok::<Adt32, Adt84>(Adt32::C12(v5)))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt32, Adt84>(v2))
        }
    }
}

pub fn f151(v0: Str) -> Result<Adt92, Adt84> {
    Ok::<Adt92, Adt84>(Adt92::C0(Adt49::C0(Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, None::<bool>), List::<Adt50>::nil())))
}

pub fn f152(v0: List<Str>) -> Str {
    let a449 = v0;
    let a450 = Str::lit("");
    join(a449, a450)
}

pub fn f153(v0: List<(Str, Adt64)>) -> Result<Adt92, Adt84> {
    Ok::<Adt92, Adt84>(Adt92::C1(Adt81::C0(v0)))
}

pub fn f154() -> Adt64 {
    f115(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f155() -> Result<Adt92, Adt84> {
    Err::<Adt92, Adt84>(Adt84::C11)
}

pub fn f156(v0: Str, v1: u64) -> Adt64 {
    Adt64::C1(Adt69::C0(List::cons(f239(v0), List::<Adt70>::nil()), None::<Adt75>, None::<Str>, None::<Adt77>, None::<Adt7>, v1, None::<List<Str>>, None::<Adt5>, None::<List<Adt35>>, None::<Adt78>))
}

pub fn f157(v0: u64) -> R<Str> {
    let a451 = f240(v0, 0i64)?;
    Ok(format_i64(a451))
}

pub fn f158(v0: Adt5) -> u64 {
    let m452 = f186(Str::lit("roots"), v0);
    match m452 {
        Adt106::C0 => {
            0u64
        }
        Adt106::C1(v1) => {
            let m453 = f241(v1);
            match m453 {
                None => {
                    0u64
                }
                Some(v2) => {
                    let a454 = v2;
                    length_list(a454)
                }
            }
        }
    }
}

pub fn f159() -> Adt64 {
    Adt64::C2(Some(f26()))
}

pub fn f160(v0: Adt21, v1: Str, v2: Str) -> Result<Adt92, Adt84> {
    if f161(v0.clone(), v1.clone()) {
        let m456 = f111(Str::lit("confirm"), Adt93::C0, match v0 {
            Adt21::C0(_, _, _, h455, _) => {
                h455
            }
        });
        match m456 {
            Adt95::C0 => {
                f163(f114(Str::lit("confirm"), f243()), v1)
            }
            Adt95::C1 => {
                f155()
            }
            Adt95::C2(v3) => {
                if f242(v3) {
                    f151(v2)
                } else {
                    f163(f114(Str::lit("confirm"), f243()), v1)
                }
            }
        }
    } else {
        f163(f114(Str::lit("confirm"), f243()), v1)
    }
}

pub fn f161(v0: Adt21, v1: Str) -> bool {
    let m458 = match v0 {
        Adt21::C0(_, _, _, _, h457) => {
            h457
        }
    };
    match m458 {
        None => {
            false
        }
        Some(v2) => {
            let a459 = v2;
            let a460 = v1;
            equal(a459, a460)
        }
    }
}

pub fn f162(v0: Adt95, v1: Adt95, v2: Adt95) -> R<Result<Adt92, Adt84>> {
    if if f244(v0.clone()) {
        true
    } else {
        if f244(v1.clone()) {
            true
        } else {
            f244(v2.clone())
        }
    } {
        Ok(f155())
    } else {
        let m461 = f164(v0.clone(), v1.clone(), v2.clone());
        match m461.uncons() {
            None => {
                Ok(f151(f152(List::cons(Str::lit("Name: "), List::cons(f113(f245(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f113(f245(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f157(f158(f245(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f163(f164(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f163(v0: List<(Str, Adt64)>, v1: Str) -> Result<Adt92, Adt84> {
    Ok::<Adt92, Adt84>(Adt92::C1(Adt81::C2(v0, v1)))
}

pub fn f164(v0: Adt95, v1: Adt95, v2: Adt95) -> List<(Str, Adt64)> {
    let a464 = if f246(v0) {
        f114(Str::lit("user_name"), f154())
    } else {
        List::<(Str, Adt64)>::nil()
    };
    let a465 = {
        let a462 = if f246(v1) {
            f114(Str::lit("greeting"), f156(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt64)>::nil()
        };
        let a463 = if f246(v2) {
            f114(Str::lit("client_roots"), f159())
        } else {
            List::<(Str, Adt64)>::nil()
        };
        append_list(a462, a463)
    };
    append_list(a464, a465)
}

pub fn f165(v0: Str) -> R<u64> {
    f120({
        let a466 = v0;
        utf8_encode(a466)
    }, 0u64)
}

pub fn f166(v0: Adt21) -> Str {
    let m468 = match v0 {
        Adt21::C0(_, _, _, _, h467) => {
            h467
        }
    };
    match m468 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f167(v0: Adt21) -> Result<Adt92, Adt84> {
    let m470 = f111(Str::lit("step1"), Adt93::C0, match v0 {
        Adt21::C0(_, _, _, h469, _) => {
            h469
        }
    });
    match m470 {
        Adt95::C0 => {
            f163(f114(Str::lit("step1"), f169()), Str::lit("1"))
        }
        Adt95::C1 => {
            f155()
        }
        Adt95::C2(v1) => {
            f163(f114(Str::lit("step2"), f247()), f152(List::cons(Str::lit("2"), List::cons(f113(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f168(v0: Adt21) -> Result<Adt92, Adt84> {
    let m472 = f111(Str::lit("step2"), Adt93::C0, match v0.clone() {
        Adt21::C0(_, _, _, h471, _) => {
            h471
        }
    });
    match m472 {
        Adt95::C0 => {
            f163(f114(Str::lit("step2"), f247()), f166(v0))
        }
        Adt95::C1 => {
            f155()
        }
        Adt95::C2(v1) => {
            f151(f152(List::cons(Str::lit("Multi-round complete for "), List::cons(f248(f166(v0)), List::cons(Str::lit(" who likes "), List::cons(f113(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f169() -> Adt64 {
    f115(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f170(v0: List<(Str, Adt64)>) -> Result<Adt92, Adt84> {
    let m473 = v0.clone();
    match m473.uncons() {
        None => {
            f151(Str::lit("No supported capabilities declared"))
        }
        Some((_, _)) => {
            f163(v0, Str::lit("capabilities-test"))
        }
    }
}

pub fn f171(v0: Adt17) -> List<(Str, Adt64)> {
    let a476 = if f249(match v0.clone() {
        Adt17::C0(_, _, _, h474, _) => {
            h474
        }
    }) {
        f114(Str::lit("elicit_input"), f115(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt64)>::nil()
    };
    let a477 = if f249(match v0 {
        Adt17::C0(_, _, h475, _, _) => {
            h475
        }
    }) {
        f114(Str::lit("sample_input"), f156(Str::lit("Sample request"), 50u64))
    } else {
        List::<(Str, Adt64)>::nil()
    };
    append_list(a476, a477)
}

pub fn f172(v0: List<(Str, Adt6)>) -> List<Str> {
    f250(f251(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f173(v0: Fn2, v1: Adt35, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    f252(v1, v0, v2)
}

pub fn f174(v0: Option<Adt19>) -> Str {
    let m478 = v0;
    match m478 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m479 = v1;
            match m479 {
                Adt19::C0(v2) => {
                    v2
                }
                Adt19::C1(v3) => {
                    let a480 = v3;
                    format_i64(a480)
                }
            }
        }
    }
}

pub fn f175(v0: u64, v1: u64) -> R<Adt50> {
    Ok(Adt50::C0(f253(v0)?, Some(f253(v1)?), Some({
        let a483 = List::cons(Str::lit("Completed step"), List::cons({
            let a481 = f240(v0, 0i64)?;
            format_i64(a481)
        }, List::cons(Str::lit("of"), List::cons({
            let a482 = f240(v1, 0i64)?;
            format_i64(a482)
        }, List::<Str>::nil()))));
        let a484 = Str::lit(" ");
        join(a483, a484)
    })))
}

pub fn f176() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, Some(f26()), None::<Adt5>, None::<Adt5>)
}

pub fn f177() -> Adt49 {
    f86(List::cons(f87(Str::lit("ok")), List::<Adt38>::nil()))
}

pub fn f178(v0: List<(Adt44, List<Adt46>)>) -> List<Adt44> {
    f254(f255(v0, List::<Adt44>::nil()), List::<Adt44>::nil())
}

pub fn f179() -> List<(Adt44, List<Adt46>)> {
    List::cons(f256(f257(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f198(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f256(f257(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f258(Str::lit("test://static-binary"), Str::lit("image/png"), f199())), List::cons(f256(f257(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f198(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f256(f257(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f198(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt44, List<Adt46>)>::nil()))))
}

pub fn f180(v0: Adt10, v1: Str, v2: Option<List<Adt46>>) -> Result<Adt32, Adt84> {
    let m485 = v2;
    match m485 {
        None => {
            Err::<Adt32, Adt84>(Adt84::C2(v1))
        }
        Some(v3) => {
            let m486 = v3.clone();
            match m486.uncons() {
                None => {
                    Err::<Adt32, Adt84>(Adt84::C9(Str::lit("a listed resource has no contents")))
                }
                Some((_, _)) => {
                    Ok::<Adt32, Adt84>(Adt32::C5(Adt52::C0(v0, v3)))
                }
            }
        }
    }
}

pub fn f181(v0: Str, p495: List<(Adt44, List<Adt46>)>) -> Option<List<Adt46>> {
    let m494 = p495;
    {
        let mut m494 = m494;
        loop {
            let step = {
                let v1 = m494;
                let m487 = v1;
                match m487.uncons() {
                    None => {
                        Err::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(None::<List<Adt46>>)
                    }
                    Some((v2, v3)) => {
                        let a491 = {
                            let m490 = {
                                let (h489, _) = v2;
                                h489
                            };
                            match m490 {
                                Adt44::C0(_, _, _, h488, _, _, _, _, _) => {
                                    h488
                                }
                            }
                        };
                        let a492 = v0.clone();
                        if equal(a491, a492) {
                            Err::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(Some({
                                let (_, h493) = v2;
                                h493
                            }))
                        } else {
                            Ok::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Str) -> R<Option<List<Adt46>>> {
    let m496 = f259(v0.clone())?;
    match m496 {
        None => {
            Ok(None::<List<Adt46>>)
        }
        Some(v1) => {
            let m497 = f260(v1)?;
            match m497 {
                None => {
                    Ok(None::<List<Adt46>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f198(v0, Str::lit("application/json"), v2), List::<Adt46>::nil())))
                }
            }
        }
    }
}

pub fn f183() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f184(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt57>>) -> Adt56 {
    Adt56::C0(v0, Some(v1), None::<List<Adt2>>, Some(v2), v3, None::<Adt5>)
}

pub fn f185(v0: Str, v1: Str) -> Adt57 {
    Adt57::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f186(v0: Str, v1: Adt5) -> Adt106 {
    f261(v0, match v1 {
        Adt5::C0(h498) => {
            h498
        }
    })
}

pub fn f187(v0: Adt6) -> Option<Adt5> {
    let m499 = v0;
    match m499 {
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

pub fn f188(v0: Adt93, v1: Adt5) -> bool {
    let m500 = v0;
    match m500 {
        Adt93::C0 => {
            f262(v1)
        }
        Adt93::C1 => {
            f263(v1)
        }
        Adt93::C2 => {
            f264(v1)
        }
    }
}

pub fn f189(v0: Str, v1: Adt5, v2: Str) -> Str {
    let m501 = f186(v0, v1);
    match m501 {
        Adt106::C0 => {
            v2
        }
        Adt106::C1(v3) => {
            let m502 = f265(v3);
            match m502 {
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

pub fn f190(v0: Str, v1: Str) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v0.clone(), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v0), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f191(v0: Str, p508: List<Adt56>) -> Option<Adt56> {
    let m507 = p508;
    {
        let mut m507 = m507;
        loop {
            let step = {
                let v1 = m507;
                let m503 = v1;
                match m503.uncons() {
                    None => {
                        Err::<List<Adt56>, Option<Adt56>>(None::<Adt56>)
                    }
                    Some((v2, v3)) => {
                        let a505 = match v2.clone() {
                            Adt56::C0(h504, _, _, _, _, _) => {
                                h504
                            }
                        };
                        let a506 = v0.clone();
                        if equal(a505, a506) {
                            Err::<List<Adt56>, Option<Adt56>>(Some(v2))
                        } else {
                            Ok::<List<Adt56>, Option<Adt56>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(p514: List<Adt57>, v1: List<(Str, Str)>) -> Option<Str> {
    let m513 = p514;
    {
        let mut m513 = m513;
        loop {
            let step = {
                let v0 = m513;
                let m509 = v0;
                match m509.uncons() {
                    None => {
                        Err::<List<Adt57>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f266(v2.clone()) {
                            let m511 = f267(match v2.clone() {
                                Adt57::C0(h510, _, _, _) => {
                                    h510
                                }
                            }, v1.clone());
                            match m511 {
                                None => {
                                    Err::<List<Adt57>, Option<Str>>(Some(match v2 {
                                        Adt57::C0(h512, _, _, _) => {
                                            h512
                                        }
                                    }))
                                }
                                Some(_) => {
                                    Ok::<List<Adt57>, Option<Str>>(v3)
                                }
                            }
                        } else {
                            Ok::<List<Adt57>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Adt56) -> List<Adt57> {
    let m516 = match v0 {
        Adt56::C0(_, _, _, _, h515, _) => {
            h515
        }
    };
    match m516 {
        None => {
            List::<Adt57>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f194(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m517 = v0;
    match m517 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f195(v0: List<Adt59>) -> Adt58 {
    Adt58::C0(None::<Str>, v0)
}

pub fn f196(v0: Str) -> Adt59 {
    Adt59::C0(Adt41::C0, Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)))
}

pub fn f197(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m518 = f267(v0, v1);
    match m518 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f198(v0: Str, v1: Str, v2: Str) -> Adt46 {
    Adt46::C0(Adt47::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f199() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f200(v0: List<Str>) -> R<Adt60> {
    let v1: u64 = f268(v0.clone(), 0u64)?;
    Ok(Adt60::C0(Adt61::C0(f269(v0), Some(v1), Some({
        let a519 = f270();
        let a520 = v1;
        nat_lt(a519, a520)
    }))))
}

pub fn f201(p526: List<Adt57>, v1: Str) -> bool {
    let m525 = p526;
    {
        let mut m525 = m525;
        loop {
            let step = {
                let v0 = m525;
                let m521 = v0;
                match m521.uncons() {
                    None => {
                        Err::<List<Adt57>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a523 = match v2 {
                            Adt57::C0(h522, _, _, _) => {
                                h522
                            }
                        };
                        let a524 = v1.clone();
                        if equal(a523, a524) {
                            Err::<List<Adt57>, bool>(true)
                        } else {
                            Ok::<List<Adt57>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m525 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f250(f271(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f203(v0: Str, v1: Str) -> List<Str> {
    let a527 = v0;
    let a528 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a527, a528) {
        let a529 = v1;
        let a530 = Str::lit("resourceUri");
        equal(a529, a530)
    } else {
        false
    } {
        f272()
    } else {
        List::<Str>::nil()
    }
}

pub fn f204() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f205(p533: List<Adt86>, p534: List<Adt86>) -> List<Adt86> {
    let m532 = (p533, p534);
    {
        let mut m532 = m532;
        loop {
            let step = {
                let (v0, v1) = m532;
                let m531 = v0;
                match m531.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
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

pub fn f206(v0: Bytes) -> R<List<u64>> {
    f273(v0.clone(), {
        let a535 = v0;
        length_bytes(a535)
    }, List::<u64>::nil())
}

pub fn f207(v0: Bytes, v1: Adt85, v2: u64) -> R<Result<Adt85, Adt87>> {
    let a537 = v2;
    let a538 = {
        let a536 = v0.clone();
        length_bytes(a536)
    };
    if nat_eq(a537, a538) {
        Ok(Ok::<Adt85, Adt87>(v1))
    } else {
        let m539 = f45(v0.clone(), v2)?;
        match m539 {
            None => {
                Ok(Err::<Adt85, Adt87>(Adt87::C0(v2)))
            }
            Some(v3) => {
                let m542 = f274(v0.clone(), {
                    let a540 = v2;
                    let a541 = 4u64;
                    nat_add(a540, a541)?
                }, v3, List::<List<u64>>::nil())?;
                match m542 {
                    Ok(v5) => {
                        let a545 = {
                            let (_, h543) = v5.clone();
                            h543
                        };
                        let a546 = {
                            let a544 = v0;
                            length_bytes(a544)
                        };
                        if nat_eq(a545, a546) {
                            Ok(Ok::<Adt85, Adt87>(f275(v1, {
                                let (h547, _) = v5;
                                h547
                            })))
                        } else {
                            Ok(Err::<Adt85, Adt87>(Adt87::C1({
                                let (_, h548) = v5;
                                h548
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

pub fn f208(v0: Adt6) -> R<Result<List<u64>, Adt96>> {
    f276(f277(), v0)
}

pub fn f209() -> u64 {
    403u64
}

pub fn f210(v0: Option<Adt94>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f278())), {
        let a549 = f279(v0);
        let a550 = List::cons((Str::lit("error"), f280(v1)), List::<(Str, Adt6)>::nil());
        append_list(a549, a550)
    })
}

pub fn f211() -> i64 {
    -32000i64
}

pub fn f212(v0: Adt90) -> Str {
    let m551 = v0;
    match m551 {
        Adt90::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt90::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f213(p558: List<u64>, p559: List<u64>) -> bool {
    let m557 = (p558, p559);
    {
        let mut m557 = m557;
        loop {
            let step = {
                let (v0, v1) = m557;
                let m552 = v0;
                match m552.uncons() {
                    None => {
                        let m553 = v1;
                        match m553.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m554 = v1;
                        match m554.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a555 = v4;
                                let a556 = v6;
                                if nat_eq(a555, a556) {
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
                Ok(next) => { m557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f215() -> u64 {
    400u64
}

pub fn f216(v0: Adt97) -> Adt80 {
    Adt80::C0(f281(v0.clone()), f282(v0), None::<Adt6>)
}

pub fn f217(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt107, Adt97>) -> R<Adt88> {
    let m560 = v4;
    match m560 {
        Ok(v6) => {
            f285(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f125(f283(v5.clone()), f127(f284(v5.clone()), f216(v5)))
        }
    }
}

pub fn f218(v0: Adt6) -> R<Result<Adt107, Adt97>> {
    let m561 = v0;
    match m561 {
        Adt6::C0 => {
            Ok(f287(None::<Adt94>, Adt98::C0))
        }
        Adt6::C1(_) => {
            Ok(f287(None::<Adt94>, Adt98::C0))
        }
        Adt6::C2(_) => {
            Ok(f287(None::<Adt94>, Adt98::C0))
        }
        Adt6::C3(_) => {
            Ok(f287(None::<Adt94>, Adt98::C0))
        }
        Adt6::C4(_) => {
            Ok(f287(None::<Adt94>, Adt98::C1))
        }
        Adt6::C5(v1) => {
            f286(Adt5::C0(v1))
        }
    }
}

pub fn f219(v0: List<u64>) -> R<Result<Adt6, Adt96>> {
    let m562 = f288(v0.clone())?;
    match m562 {
        Adt116::C0 => {
            f289(f290(), 0u64, v0)
        }
        Adt116::C1(v1) => {
            Ok(Err::<Adt6, Adt96>(Adt96::C0(v1)))
        }
        Adt116::C2(v2) => {
            Ok(Err::<Adt6, Adt96>(Adt96::C1(v2)))
        }
    }
}

pub fn f220() -> u64 {
    405u64
}

pub fn f221() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f222() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f223(p567: List<Adt86>, v1: List<u64>, p568: Adt100) -> R<Adt100> {
    let m566 = (p567, p568);
    {
        let mut m566 = m566;
        loop {
            let step = {
                let (v0, v2) = m566;
                let m563 = v0;
                match m563.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt100), R<Adt100>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f291(match v3.clone() {
                            Adt86::C0(h564, _) => {
                                h564
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt100), R<Adt100>>((v4, f292(v2, match v3 {
                                Adt86::C0(_, h565) => {
                                    h565
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt100), R<Adt100>>((v4, v2))
                        }
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

pub fn f224(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f293(v0);
    if f294(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f225(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f291(v0.clone(), f295())? {
        Ok(true)
    } else {
        if f291(v0.clone(), f296())? {
            Ok(true)
        } else {
            if f291(v0.clone(), f297())? {
                Ok(true)
            } else {
                f298(v0, v1)
            }
        }
    }
}

pub fn f226(p573: List<u64>) -> Option<List<u64>> {
    let m572 = p573;
    {
        let mut m572 = m572;
        loop {
            let step = {
                let v0 = m572;
                let m569 = v0;
                match m569.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a570 = v1;
                        let a571 = 58u64;
                        if nat_eq(a570, a571) {
                            Err::<List<u64>, Option<List<u64>>>(f299(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m574 = v0;
    match m574 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f300({
                let (_, h575) = v1;
                h575
            }) {
                Some({
                    let (h576, _) = v1;
                    h576
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f228(p581: List<u64>, p582: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m580 = (p581, p582);
    {
        let mut m580 = m580;
        loop {
            let step = {
                let (v0, v1) = m580;
                let m577 = v0;
                match m577.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a578 = v2;
                        let a579 = 93u64;
                        if nat_eq(a578, a579) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f301(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(p587: List<u64>, p588: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m586 = (p587, p588);
    {
        let mut m586 = m586;
        loop {
            let step = {
                let (v0, v1) = m586;
                let m583 = v0;
                match m583.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f301(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a584 = v2;
                        let a585 = 58u64;
                        if nat_eq(a584, a585) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f301(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f230(v0: Str, p593: List<Str>) -> bool {
    let m592 = p593;
    {
        let mut m592 = m592;
        loop {
            let step = {
                let v1 = m592;
                let m589 = v1;
                match m589.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a590 = v0.clone();
                        let a591 = v2;
                        if equal(a590, a591) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
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

pub fn f231(v0: Str, v1: List<u64>) -> Option<Adt91> {
    if f302(v1) {
        Some(Adt91::C0(v0))
    } else {
        None::<Adt91>
    }
}

pub fn f232(v0: Str) -> R<List<u64>> {
    f206({
        let a594 = v0;
        utf8_encode(a594)
    })
}

pub fn f233(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m595 = f303(v0.clone());
    match m595 {
        None => {
            f304(v0.clone(), f305(f306(), match v0.clone() {
                Adt35::C0(h596, _, _, _, _, _, _, _) => {
                    h596
                }
            }, Adt6::C5({
                let m599 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h598, _, _, _) => {
                        h598
                    }
                };
                match m599 {
                    Adt5::C0(h597) => {
                        h597
                    }
                }
            }), List::cons(Adt108::C0(Adt6::C5({
                let m602 = match v0 {
                    Adt35::C0(_, _, _, _, h601, _, _, _) => {
                        h601
                    }
                };
                match m602 {
                    Adt5::C0(h600) => {
                        h600
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt108>::nil()), f307())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f234(p607: u64, p608: i64) -> R<i64> {
    let m606 = (p607, p608);
    {
        let mut m606 = m606;
        loop {
            let step = {
                let (v0, v1) = m606;
                let m603 = v0;
                if m603 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m603 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a604 = v1;
                        let a605 = -1i64;
                        int_sub(a604, a605)?
                    }))
                }
            };
            match step {
                Ok(next) => { m606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: Adt6, p627: u64, p628: Adt103, p629: List<Adt104>) -> R<Adt101> {
    let m626 = (p627, (p628, p629));
    {
        let mut m626 = m626;
        loop {
            let step = {
                let (v1, (v2, v3)) = m626;
                let m609 = v1;
                if m609 == 0 {
                    Err::<(u64, (Adt103, List<Adt104>)), R<Adt101>>(Ok(Adt101::C4))
                } else {
                    let v4 = m609 - 1;
                    let m610 = v3;
                    match m610.uncons() {
                        None => {
                            let m611 = v2;
                            match m611 {
                                Adt103::C0 => {
                                    Err::<(u64, (Adt103, List<Adt104>)), R<Adt101>>(Ok(Adt101::C0))
                                }
                                Adt103::C1(v5, v6) => {
                                    Err::<(u64, (Adt103, List<Adt104>)), R<Adt101>>(Ok(Adt101::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m612 = v2.clone();
                            match m612 {
                                Adt103::C0 => {
                                    let m613 = v7.clone();
                                    match m613 {
                                        Adt104::C0(v9, v10, v11) => {
                                            let m614 = f308(v0.clone(), v9, v10, v11.clone())?;
                                            match m614 {
                                                Adt125::C0(v12) => {
                                                    Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (Adt103::C1(v11, v12), v8)))
                                                }
                                                Adt125::C1(v13) => {
                                                    Err::<(u64, (Adt103, List<Adt104>)), R<Adt101>>(Ok(Adt101::C2(v13)))
                                                }
                                                Adt125::C2(v14) => {
                                                    Err::<(u64, (Adt103, List<Adt104>)), R<Adt101>>(Ok(Adt101::C3(v14)))
                                                }
                                                Adt125::C3(v15) => {
                                                    let v16: List<Adt104> = {
                                                        let a615 = v15;
                                                        let a616 = v8;
                                                        append_list(a615, a616)
                                                    };
                                                    Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt104::C1(_, _, _) => {
                                            Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (v2, v8)))
                                        }
                                        Adt104::C2(_, _, _, _) => {
                                            let v24: Adt109 = f309(v7, v8)?;
                                            Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (match v24.clone() {
                                                Adt109::C0(h617, _) => {
                                                    h617
                                                }
                                            }, match v24 {
                                                Adt109::C0(_, h618) => {
                                                    h618
                                                }
                                            })))
                                        }
                                        Adt104::C3(v25) => {
                                            Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (Adt103::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt104::C4(_, _, _, _) => {
                                            let v39: Adt109 = f309(v7, v8)?;
                                            Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (match v39.clone() {
                                                Adt109::C0(h622, _) => {
                                                    h622
                                                }
                                            }, match v39 {
                                                Adt109::C0(_, h623) => {
                                                    h623
                                                }
                                            })))
                                        }
                                        Adt104::C5(v26, v27, v28, v29) => {
                                            let m619 = f310(v26, v27, v28, v29.clone());
                                            match m619 {
                                                Adt125::C0(v30) => {
                                                    Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (Adt103::C1(v29, v30), v8)))
                                                }
                                                Adt125::C1(v31) => {
                                                    Err::<(u64, (Adt103, List<Adt104>)), R<Adt101>>(Ok(Adt101::C2(v31)))
                                                }
                                                Adt125::C2(v32) => {
                                                    Err::<(u64, (Adt103, List<Adt104>)), R<Adt101>>(Ok(Adt101::C3(v32)))
                                                }
                                                Adt125::C3(v33) => {
                                                    let v34: List<Adt104> = {
                                                        let a620 = v33;
                                                        let a621 = v8;
                                                        append_list(a620, a621)
                                                    };
                                                    Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt103::C1(v40, v41) => {
                                    let v42: Adt109 = f311(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt103, List<Adt104>)), R<Adt101>>((v4, (match v42.clone() {
                                        Adt109::C0(h624, _) => {
                                            h624
                                        }
                                    }, match v42 {
                                        Adt109::C0(_, h625) => {
                                            h625
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236() -> u64 {
    67108864u64
}

pub fn f237(v0: List<Adt102>) -> R<Str> {
    let a630 = List::cons(Str::lit(""), f312(v0, List::<Str>::nil())?);
    let a631 = Str::lit("/");
    Ok(join(a630, a631))
}

pub fn f238(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f313({
        let m634 = match v1.clone() {
            Adt49::C0(h633, _) => {
                h633
            }
        };
        match m634 {
            Adt37::C0(h632, _, _) => {
                h632
            }
        }
    })? {
        f314(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f239(v0: Str) -> Adt70 {
    Adt70::C0(Adt41::C0, Adt71::C0(Adt72::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))), None::<Adt5>)
}

pub fn f240(p637: u64, p638: i64) -> R<i64> {
    let m636 = (p637, p638);
    {
        let mut m636 = m636;
        loop {
            let step = {
                let (v0, v1) = m636;
                let m635 = v0;
                if m635 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m635 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f315(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m636 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Adt6) -> Option<List<Adt6>> {
    let m639 = v0;
    match m639 {
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

pub fn f242(v0: Adt5) -> bool {
    let m640 = f186(Str::lit("content"), v0);
    match m640 {
        Adt106::C0 => {
            false
        }
        Adt106::C1(v1) => {
            let m641 = f187(v1);
            match m641 {
                None => {
                    false
                }
                Some(v2) => {
                    let m642 = f186(Str::lit("ok"), v2);
                    match m642 {
                        Adt106::C0 => {
                            false
                        }
                        Adt106::C1(v3) => {
                            let m643 = f316(v3);
                            match m643 {
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

pub fn f243() -> Adt64 {
    f115(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f244(v0: Adt95) -> bool {
    let m644 = v0;
    match m644 {
        Adt95::C0 => {
            false
        }
        Adt95::C1 => {
            true
        }
        Adt95::C2(_) => {
            false
        }
    }
}

pub fn f245(v0: Adt95) -> Adt5 {
    let m645 = v0;
    match m645 {
        Adt95::C0 => {
            f26()
        }
        Adt95::C1 => {
            f26()
        }
        Adt95::C2(v1) => {
            v1
        }
    }
}

pub fn f246(v0: Adt95) -> bool {
    let m646 = v0;
    match m646 {
        Adt95::C0 => {
            true
        }
        Adt95::C1 => {
            false
        }
        Adt95::C2(_) => {
            false
        }
    }
}

pub fn f247() -> Adt64 {
    f115(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f248(v0: Str) -> Str {
    let v1: Bytes = {
        let a647 = v0;
        utf8_encode(a647)
    };
    let m654 = {
        let a651 = v1.clone();
        let a652 = 1u64;
        let a653 = {
            let a649 = {
                let a648 = v1;
                length_bytes(a648)
            };
            let a650 = 1u64;
            nat_sub(a649, a650)
        };
        slice_bytes(a651, a652, a653)
    };
    match m654 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m656 = {
                let a655 = v2;
                utf8_decode(a655)
            };
            match m656 {
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

pub fn f249(v0: Option<Adt5>) -> bool {
    let m657 = v0;
    match m657 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f250(p660: List<Str>, p661: List<Str>) -> List<Str> {
    let m659 = (p660, p661);
    {
        let mut m659 = m659;
        loop {
            let step = {
                let (v0, v1) = m659;
                let m658 = v0;
                match m658.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m659 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(p665: List<(Str, Adt6)>, p666: List<Str>) -> List<Str> {
    let m664 = (p665, p666);
    {
        let mut m664 = m664;
        loop {
            let step = {
                let (v0, v1) = m664;
                let m662 = v0;
                match m662.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h663, _) = v2;
                            h663
                        }, v1)))
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

pub fn f252(v0: Adt35, v1: Fn2, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m668 = {
        let c667: Fn2 = v1;
        c667.apply(v2)?
    };
    match m668 {
        Ok(v4) => {
            f238(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f253(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f240(v0, 0i64)?, 0i64))
}

pub fn f254(p671: List<Adt44>, p672: List<Adt44>) -> List<Adt44> {
    let m670 = (p671, p672);
    {
        let mut m670 = m670;
        loop {
            let step = {
                let (v0, v1) = m670;
                let m669 = v0;
                match m669.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt44>), List<Adt44>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt44>), List<Adt44>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(p676: List<(Adt44, List<Adt46>)>, p677: List<Adt44>) -> List<Adt44> {
    let m675 = (p676, p677);
    {
        let mut m675 = m675;
        loop {
            let step = {
                let (v0, v1) = m675;
                let m673 = v0;
                match m673.uncons() {
                    None => {
                        Err::<(List<(Adt44, List<Adt46>)>, List<Adt44>), List<Adt44>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt44, List<Adt46>)>, List<Adt44>), List<Adt44>>((v3, List::cons({
                            let (h674, _) = v2;
                            h674
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m675 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: Adt44, v1: Adt46) -> (Adt44, List<Adt46>) {
    (v0, List::cons(v1, List::<Adt46>::nil()))
}

pub fn f257(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt44 {
    Adt44::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt40>, None::<u64>, None::<Adt5>)
}

pub fn f258(v0: Str, v1: Str, v2: Str) -> Adt46 {
    Adt46::C1(Adt48::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f259(v0: Str) -> R<Option<Str>> {
    let m678 = f317(v0)?;
    match m678 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a679 = f318(f319(), v1);
            Ok(utf8_decode(a679))
        }
    }
}

pub fn f260(v0: Str) -> R<Option<Str>> {
    let m682 = f320(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a680 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a681 = Str::lit(" ");
        join(a680, a681)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m682 {
        Ok(v1) => {
            let a683 = v1;
            Ok(utf8_decode(a683))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f261(v0: Str, p689: List<(Str, Adt6)>) -> Adt106 {
    let m688 = p689;
    {
        let mut m688 = m688;
        loop {
            let step = {
                let v1 = m688;
                let m684 = v1;
                match m684.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt106>(Adt106::C0)
                    }
                    Some((v2, v3)) => {
                        let m686 = f321({
                            let (h685, _) = v2;
                            h685
                        }, v0.clone());
                        match m686 {
                            Adt126::C0 => {
                                Err::<List<(Str, Adt6)>, Adt106>(Adt106::C1({
                                    let (_, h687) = v2;
                                    h687
                                }))
                            }
                            Adt126::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt106>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m688 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f262(v0: Adt5) -> bool {
    if f322(v0.clone()) {
        f323(v0)
    } else {
        false
    }
}

pub fn f263(v0: Adt5) -> bool {
    if f324(v0.clone()) {
        if f325(f186(Str::lit("model"), v0.clone())) {
            f326(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f264(v0: Adt5) -> bool {
    let m690 = f186(Str::lit("roots"), v0);
    match m690 {
        Adt106::C0 => {
            false
        }
        Adt106::C1(v1) => {
            let m691 = f241(v1);
            match m691 {
                None => {
                    false
                }
                Some(v2) => {
                    f327(v2)
                }
            }
        }
    }
}

pub fn f265(v0: Adt6) -> Option<Str> {
    let m692 = v0;
    match m692 {
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

pub fn f266(v0: Adt57) -> bool {
    let m694 = match v0 {
        Adt57::C0(_, _, _, h693) => {
            h693
        }
    };
    match m694 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f267(v0: Str, p701: List<(Str, Str)>) -> Option<Str> {
    let m700 = p701;
    {
        let mut m700 = m700;
        loop {
            let step = {
                let v1 = m700;
                let m695 = v1;
                match m695.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a697 = {
                            let (h696, _) = v2;
                            h696
                        };
                        let a698 = v0.clone();
                        if equal(a697, a698) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h699) = v2;
                                h699
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
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

pub fn f268(p706: List<Str>, p707: u64) -> R<u64> {
    let m705 = (p706, p707);
    {
        let mut m705 = m705;
        loop {
            let step = {
                let (v0, v1) = m705;
                let m702 = v0;
                match m702.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a703 = v1;
                            let a704 = 1u64;
                            nat_add(a703, a704)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f269(v0: List<Str>) -> List<Str> {
    f250(f328(v0, f270(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f270() -> u64 {
    100u64
}

pub fn f271(v0: Str, p710: List<Str>, p711: List<Str>) -> R<List<Str>> {
    let m709 = (p710, p711);
    {
        let mut m709 = m709;
        loop {
            let step = {
                let (v1, v2) = m709;
                let m708 = v1;
                match m708.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f329(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m709 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272() -> List<Str> {
    f250(f330(f102(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f273(v0: Bytes, p714: u64, p715: List<u64>) -> R<List<u64>> {
    let m713 = (p714, p715);
    {
        let mut m713 = m713;
        loop {
            let step = {
                let (v1, v2) = m713;
                let m712 = v1;
                if m712 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m712 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f120(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: Bytes, p721: u64, p722: u64, p723: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt87>> {
    let m720 = (p721, (p722, p723));
    {
        let mut m720 = m720;
        loop {
            let step = {
                let (v1, (v2, v3)) = m720;
                let m716 = v2;
                if m716 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Ok::<(List<List<u64>>, u64), Adt87>((f50(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m716 - 1;
                    let m717 = f16(v0.clone(), v1)?;
                    match m717 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Err::<(List<List<u64>>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>((match v5.clone() {
                                Adt89::C0(_, h718) => {
                                    h718
                                }
                            }, (v4, List::cons(f206(match v5 {
                                Adt89::C0(h719, _) => {
                                    h719
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Adt85, v1: List<List<u64>>) -> Adt85 {
    Adt85::C0(match v0.clone() {
        Adt85::C0(h724, _, _, _, _, _, _) => {
            h724
        }
    }, match v0.clone() {
        Adt85::C0(_, h725, _, _, _, _, _) => {
            h725
        }
    }, match v0.clone() {
        Adt85::C0(_, _, h726, _, _, _, _) => {
            h726
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, h727, _, _, _) => {
            h727
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, _, h728, _, _) => {
            h728
        }
    }, v1, match v0 {
        Adt85::C0(_, _, _, _, _, _, h729) => {
            h729
        }
    })
}

pub fn f276(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt96>> {
    let m730 = f331(v0, List::cons(Adt110::C1(0u64, v1), List::<Adt110>::nil()), List::<u64>::nil())?;
    match m730 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt96>(f332(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt96>(v3))
        }
    }
}

pub fn f277() -> u64 {
    67108864u64
}

pub fn f278() -> Str {
    Str::lit("2.0")
}

pub fn f279(v0: Option<Adt94>) -> List<(Str, Adt6)> {
    let m731 = v0;
    match m731 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f333(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f280(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h732, _, _) => {
            h732
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h733, _) => {
            h733
        }
    })), f334(match v0 {
        Adt80::C0(_, _, h734) => {
            h734
        }
    }))))
}

pub fn f281(v0: Adt97) -> i64 {
    let m735 = v0;
    match m735 {
        Adt97::C0 => {
            f335()
        }
        Adt97::C1(_, _) => {
            f336()
        }
    }
}

pub fn f282(v0: Adt97) -> Str {
    let m736 = v0;
    match m736 {
        Adt97::C0 => {
            Str::lit("Parse error")
        }
        Adt97::C1(_, v2) => {
            f337(v2)
        }
    }
}

pub fn f283(v0: Adt97) -> u64 {
    let m737 = v0;
    match m737 {
        Adt97::C0 => {
            f215()
        }
        Adt97::C1(_, _) => {
            f215()
        }
    }
}

pub fn f284(v0: Adt97) -> Option<Adt94> {
    let m738 = v0;
    match m738 {
        Adt97::C0 => {
            None::<Adt94>
        }
        Adt97::C1(v1, _) => {
            v1
        }
    }
}

pub fn f285(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt107) -> R<Adt88> {
    f338(f339(v3.clone())?, f340(v0.clone(), v1, v2, Some(f341(v3, match v0 {
        Adt0::C0(_, _, _, _, h739, _, _) => {
            h739
        }
    }, f342(v4.clone()), match v4.clone() {
        Adt107::C0(_, h740, _) => {
            h740
        }
    })?), f343(), v4)?)
}

pub fn f286(v0: Adt5) -> R<Result<Adt107, Adt97>> {
    let m741 = f344(v0.clone())?;
    match m741 {
        Adt134::C0 => {
            let m742 = f186(Str::lit("method"), v0.clone());
            match m742 {
                Adt106::C0 => {
                    Ok(f287(None::<Adt94>, f345(v0)))
                }
                Adt106::C1(_) => {
                    Ok(f287(None::<Adt94>, if f346(v0) {
                        Adt98::C8
                    } else {
                        Adt98::C7
                    }))
                }
            }
        }
        Adt134::C1(v2) => {
            Ok(f347(v2, v0))
        }
        Adt134::C2 => {
            Ok(f287(None::<Adt94>, Adt98::C5))
        }
    }
}

pub fn f287(v0: Option<Adt94>, v1: Adt98) -> Result<Adt107, Adt97> {
    Err::<Adt107, Adt97>(Adt97::C1(v0, v1))
}

pub fn f288(v0: List<u64>) -> R<Adt116> {
    Ok(f348({
        let a743 = v0.clone();
        length_list(a743)
    }, f349(Adt117::C0, 0u64, 0u64, v0)?))
}

pub fn f289(p750: Adt118, p751: u64, p752: List<u64>) -> R<Result<Adt6, Adt96>> {
    let m749 = (p750, (p751, p752));
    {
        let mut m749 = m749;
        loop {
            let step = {
                let (v0, (v1, v2)) = m749;
                let m744 = v2;
                match m744.uncons() {
                    None => {
                        let m745 = f350(v0, v1)?;
                        match m745 {
                            Ok(v3) => {
                                Err::<(Adt118, (u64, List<u64>)), R<Result<Adt6, Adt96>>>(Ok(Ok::<Adt6, Adt96>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt118, (u64, List<u64>)), R<Result<Adt6, Adt96>>>(Ok(Err::<Adt6, Adt96>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m746 = f351(v0, v1, v5)?;
                        match m746 {
                            Adt135::C0(v7) => {
                                Err::<(Adt118, (u64, List<u64>)), R<Result<Adt6, Adt96>>>(Ok(Err::<Adt6, Adt96>(v7)))
                            }
                            Adt135::C1(v8) => {
                                Ok::<(Adt118, (u64, List<u64>)), R<Result<Adt6, Adt96>>>((v8, ({
                                    let a747 = v1;
                                    let a748 = 1u64;
                                    nat_add(a747, a748)?
                                }, v6)))
                            }
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

pub fn f290() -> Adt118 {
    Adt118::C0(Adt119::C0, List::<Adt123>::nil(), 0u64)
}

pub fn f291(p759: List<u64>, p760: List<u64>) -> R<bool> {
    let m758 = (p759, p760);
    {
        let mut m758 = m758;
        loop {
            let step = {
                let (v0, v1) = m758;
                let m753 = v0;
                match m753.uncons() {
                    None => {
                        let m754 = v1;
                        match m754.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m755 = v1;
                        match m755.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a756 = f352(v4)?;
                                let a757 = f352(v6)?;
                                if nat_eq(a756, a757) {
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
                Ok(next) => { m758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: Adt100, v1: List<u64>) -> Adt100 {
    let m761 = v0;
    match m761 {
        Adt100::C0 => {
            Adt100::C1(v1)
        }
        Adt100::C1(_) => {
            Adt100::C2
        }
        Adt100::C2 => {
            Adt100::C2
        }
    }
}

pub fn f293(v0: List<u64>) -> List<u64> {
    f301(f353(f301(f353(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f294(p764: List<u64>) -> bool {
    let m763 = p764;
    {
        let mut m763 = m763;
        loop {
            let step = {
                let v0 = m763;
                let m762 = v0;
                match m762.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f354(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f295() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f296() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f297() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f298(v0: List<u64>, p767: List<List<u64>>) -> R<bool> {
    let m766 = p767;
    {
        let mut m766 = m766;
        loop {
            let step = {
                let v1 = m766;
                let m765 = v1;
                match m765.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f291(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m766 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: List<u64>) -> Option<List<u64>> {
    let m768 = v0;
    match m768.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m769 = v2;
            match m769.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a770 = v1;
                    let a771 = 47u64;
                    if if nat_eq(a770, a771) {
                        let a772 = v3;
                        let a773 = 47u64;
                        nat_eq(a772, a773)
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

pub fn f300(v0: List<u64>) -> bool {
    let m774 = v0;
    match m774.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a775 = v1;
            let a776 = 58u64;
            if nat_eq(a775, a776) {
                let m777 = v2.clone();
                match m777.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f355(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f301(p780: List<u64>, p781: List<u64>) -> List<u64> {
    let m779 = (p780, p781);
    {
        let mut m779 = m779;
        loop {
            let step = {
                let (v0, v1) = m779;
                let m778 = v0;
                match m778.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: List<u64>) -> bool {
    let m782 = v0.clone();
    match m782.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a786 = if f356(v0.clone(), false) {
                let a784 = {
                    let a783 = v0;
                    length_list(a783)
                };
                let a785 = f357();
                nat_le(a784, a785)
            } else {
                false
            };
            bool_not(a786)
        }
    }
}

pub fn f303(v0: Adt35) -> Option<Adt91> {
    let m791 = f358(match v0.clone() {
        Adt35::C0(h787, _, _, _, _, _, _, _) => {
            h787
        }
    }, {
        let m790 = match v0.clone() {
            Adt35::C0(_, _, _, _, h789, _, _, _) => {
                h789
            }
        };
        match m790 {
            Adt5::C0(h788) => {
                h788
            }
        }
    });
    match m791 {
        None => {
            f359(match v0.clone() {
                Adt35::C0(h792, _, _, _, _, _, _, _) => {
                    h792
                }
            }, {
                let m795 = match v0 {
                    Adt35::C0(_, _, _, _, h794, _, _, _) => {
                        h794
                    }
                };
                match m795 {
                    Adt5::C0(h793) => {
                        h793
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f304(v0: Adt35, v1: Result<Adt124, Adt91>) -> R<Result<List<Adt12>, Adt91>> {
    let m796 = v1;
    match m796 {
        Ok(v3) => {
            let m797 = f360(v0)?;
            match m797 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt91>(f141(match v3 {
                        Adt124::C0(h798, _) => {
                            h798
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

pub fn f305(p805: u64, v1: Str, v2: Adt6, p806: List<Adt108>, p807: Adt124) -> R<Result<Adt124, Adt91>> {
    let m804 = (p805, (p806, p807));
    {
        let mut m804 = m804;
        loop {
            let step = {
                let (v0, (v3, v4)) = m804;
                let m799 = v0;
                if m799 == 0 {
                    Err::<(u64, (List<Adt108>, Adt124)), R<Result<Adt124, Adt91>>>(Ok(Err::<Adt124, Adt91>(Adt91::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m799 - 1;
                    let m800 = v3;
                    match m800.uncons() {
                        None => {
                            Err::<(u64, (List<Adt108>, Adt124)), R<Result<Adt124, Adt91>>>(Ok(Ok::<Adt124, Adt91>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m801 = f361(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m801 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt108>, Adt124)), R<Result<Adt124, Adt91>>>((v5, ({
                                        let (h802, _) = v9;
                                        h802
                                    }, {
                                        let (_, h803) = v9;
                                        h803
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt108>, Adt124)), R<Result<Adt124, Adt91>>>(Ok(Err::<Adt124, Adt91>(v8)))
                                }
                            }
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

pub fn f306() -> u64 {
    67108864u64
}

pub fn f307() -> Adt124 {
    Adt124::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f308(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt102>) -> R<Adt125> {
    let m808 = f316(v1.clone());
    match m808 {
        None => {
            let m809 = f187(v1);
            match m809 {
                None => {
                    Ok(Adt125::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f362(v0, match v5 {
                        Adt5::C0(h810) => {
                            h810
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt125::C3(List::<Adt104>::nil()))
            } else {
                Ok(Adt125::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f309(v0: Adt104, v1: List<Adt104>) -> R<Adt109> {
    let m811 = v0;
    match m811 {
        Adt104::C0(_, _, _) => {
            Ok(Adt109::C0(Adt103::C0, v1))
        }
        Adt104::C1(_, _, _) => {
            Ok(Adt109::C0(Adt103::C0, v1))
        }
        Adt104::C2(v8, v9, v10, v11) => {
            Ok(f363(v8, v9, v10, {
                let a812 = v11;
                let a813 = 1u64;
                nat_add(a812, a813)?
            }, v1))
        }
        Adt104::C3(v12) => {
            Ok(Adt109::C0(Adt103::C1(v12, Str::lit("not")), v1))
        }
        Adt104::C4(v13, _, v15, v16) => {
            Ok(f364(v13, v15, v16, v1))
        }
        Adt104::C5(_, _, _, _) => {
            Ok(Adt109::C0(Adt103::C0, v1))
        }
    }
}

pub fn f310(v0: Adt105, v1: Adt6, v2: Adt6, v3: List<Adt102>) -> Adt125 {
    let m814 = f187(v1);
    match m814 {
        None => {
            Adt125::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m815 = v0;
            match m815 {
                Adt105::C0 => {
                    f365(match v4 {
                        Adt5::C0(h816) => {
                            h816
                        }
                    }, v2, v3)
                }
                Adt105::C1 => {
                    f366(match v4 {
                        Adt5::C0(h817) => {
                            h817
                        }
                    }, v2, v3)
                }
                Adt105::C2 => {
                    f367(match v4 {
                        Adt5::C0(h818) => {
                            h818
                        }
                    }, v2, v3)
                }
                Adt105::C3 => {
                    f368(match v4 {
                        Adt5::C0(h819) => {
                            h819
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f311(v0: Adt104, v1: List<Adt104>, v2: List<Adt102>, v3: Str) -> Adt109 {
    let m820 = v0;
    match m820 {
        Adt104::C0(_, _, _) => {
            Adt109::C0(Adt103::C1(v2, v3), v1)
        }
        Adt104::C1(v7, v8, v9) => {
            f369(v7, v8, v9, v1)
        }
        Adt104::C2(v10, v11, v12, v13) => {
            f363(v10, v11, v12, v13, v1)
        }
        Adt104::C3(_) => {
            Adt109::C0(Adt103::C0, v1)
        }
        Adt104::C4(_, v16, v17, v18) => {
            f364(v16, v17, v18, v1)
        }
        Adt104::C5(_, _, _, _) => {
            Adt109::C0(Adt103::C1(v2, v3), v1)
        }
    }
}

pub fn f312(p823: List<Adt102>, p824: List<Str>) -> R<List<Str>> {
    let m822 = (p823, p824);
    {
        let mut m822 = m822;
        loop {
            let step = {
                let (v0, v1) = m822;
                let m821 = v0;
                match m821.uncons() {
                    None => {
                        Err::<(List<Adt102>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt102>, List<Str>), R<List<Str>>>((v3, List::cons(f370(v2)?, v1)))
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

pub fn f313(p833: List<Adt38>) -> R<bool> {
    let m832 = p833;
    {
        let mut m832 = m832;
        loop {
            let step = {
                let v0 = m832;
                let m825 = v0;
                match m825.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m826 = v1;
                        match m826 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f371(match v4 {
                                    Adt42::C0(h827, _, _, _) => {
                                        h827
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f371(match v5 {
                                    Adt43::C0(h828, _, _, _) => {
                                        h828
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
                                let m830 = match v7 {
                                    Adt45::C0(h829, _, _) => {
                                        h829
                                    }
                                };
                                match m830 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f371(match v9 {
                                            Adt48::C0(_, _, h831, _) => {
                                                h831
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
                Ok(next) => { m832 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f314(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f372(match v1.clone() {
        Adt49::C0(h834, _) => {
            h834
        }
    }) {
        f373(v1)
    } else {
        let m836 = match v0 {
            Adt35::C0(_, _, _, _, _, h835, _, _) => {
                h835
            }
        };
        match m836 {
            None => {
                f373(v1)
            }
            Some(v2) => {
                f374(v2, v1)
            }
        }
    }
}

pub fn f315(v0: i64, v1: i64) -> R<i64> {
    let a838 = v0;
    let a839 = {
        let a837 = v1;
        int_neg(a837)?
    };
    int_sub(a838, a839)
}

pub fn f316(v0: Adt6) -> Option<bool> {
    let m840 = v0;
    match m840 {
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

pub fn f317(v0: Str) -> R<Option<List<u64>>> {
    let m841 = f375(f376(f377())?, f376(v0)?);
    match m841 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m842 = f375(f378(List::cons(47u64, f376(f379())?)), f378(v1));
            match m842 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f378(v2);
                    if if f380(v3.clone()) {
                        f381(v3.clone())
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

pub fn f318(v0: Bytes, v1: List<u64>) -> Bytes {
    let a843 = v0;
    let a844 = f382(f319(), f383(List::<Bytes>::nil(), v1));
    append_bytes(a843, a844)
}

pub fn f319() -> Bytes {
    Bytes::lit(&[])
}

pub fn f320(v0: Adt6) -> R<Result<Bytes, Adt96>> {
    let m845 = f208(v0)?;
    match m845 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt96>(f318(f319(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt96>(v2))
        }
    }
}

pub fn f321(v0: Str, v1: Str) -> Adt126 {
    let a846 = v0;
    let a847 = v1;
    if equal(a846, a847) {
        Adt126::C0
    } else {
        Adt126::C1
    }
}

pub fn f322(v0: Adt5) -> bool {
    if f384(f186(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f384(f186(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f384(f186(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f323(v0: Adt5) -> bool {
    let m848 = f186(Str::lit("content"), v0);
    match m848 {
        Adt106::C0 => {
            true
        }
        Adt106::C1(v1) => {
            let m849 = f187(v1);
            match m849 {
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

pub fn f324(v0: Adt5) -> bool {
    if f384(f186(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f384(f186(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f325(v0: Adt106) -> bool {
    let m850 = v0;
    match m850 {
        Adt106::C0 => {
            false
        }
        Adt106::C1(v1) => {
            let m851 = f265(v1);
            match m851 {
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

pub fn f326(v0: Adt5) -> bool {
    let m852 = f186(Str::lit("content"), v0);
    match m852 {
        Adt106::C0 => {
            false
        }
        Adt106::C1(v1) => {
            let m853 = f187(v1.clone());
            match m853 {
                None => {
                    let m854 = f241(v1);
                    match m854 {
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

pub fn f327(p858: List<Adt6>) -> bool {
    let m857 = p858;
    {
        let mut m857 = m857;
        loop {
            let step = {
                let v0 = m857;
                let m855 = v0;
                match m855.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m856 = f187(v1);
                        match m856 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f325(f186(Str::lit("uri"), v3)) {
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
                Ok(next) => { m857 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(p862: List<Str>, p863: u64, p864: List<Str>) -> List<Str> {
    let m861 = (p862, (p863, p864));
    {
        let mut m861 = m861;
        loop {
            let step = {
                let (v0, (v1, v2)) = m861;
                let m859 = v0;
                match m859.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m860 = v1;
                        if m860 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m860 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m861 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(v0: Str, v1: Str) -> R<bool> {
    Ok(f385(f376(v0)?, f376(v1)?))
}

pub fn f330(p868: List<Adt44>, p869: List<Str>) -> List<Str> {
    let m867 = (p868, p869);
    {
        let mut m867 = m867;
        loop {
            let step = {
                let (v0, v1) = m867;
                let m865 = v0;
                match m865.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt44::C0(_, _, _, h866, _, _, _, _, _) => {
                                h866
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m867 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(p874: u64, p875: List<Adt110>, p876: List<u64>) -> R<Result<List<u64>, Adt96>> {
    let m873 = (p874, (p875, p876));
    {
        let mut m873 = m873;
        loop {
            let step = {
                let (v0, (v1, v2)) = m873;
                let m870 = v0;
                if m870 == 0 {
                    Err::<(u64, (List<Adt110>, List<u64>)), R<Result<List<u64>, Adt96>>>(Ok(Err::<List<u64>, Adt96>(Adt96::C13)))
                } else {
                    let v3 = m870 - 1;
                    let m871 = v1;
                    match m871.uncons() {
                        None => {
                            Err::<(u64, (List<Adt110>, List<u64>)), R<Result<List<u64>, Adt96>>>(Ok(Ok::<List<u64>, Adt96>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m872 = f386(v4, v5, v2)?;
                            match m872 {
                                Adt136::C0(v6) => {
                                    Err::<(u64, (List<Adt110>, List<u64>)), R<Result<List<u64>, Adt96>>>(Ok(Err::<List<u64>, Adt96>(v6)))
                                }
                                Adt136::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt110>, List<u64>)), R<Result<List<u64>, Adt96>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(p879: List<u64>, p880: List<u64>) -> List<u64> {
    let m878 = (p879, p880);
    {
        let mut m878 = m878;
        loop {
            let step = {
                let (v0, v1) = m878;
                let m877 = v0;
                match m877.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m878 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333(v0: Adt94) -> Adt6 {
    let m881 = v0;
    match m881 {
        Adt94::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt94::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f334(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m882 = v0;
    match m882 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f335() -> i64 {
    -32700i64
}

pub fn f336() -> i64 {
    -32600i64
}

pub fn f337(v0: Adt98) -> Str {
    let m883 = v0;
    match m883 {
        Adt98::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt98::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt98::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt98::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt98::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt98::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt98::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt98::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt98::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f338(v0: bool, v1: Adt127) -> R<Adt88> {
    let m885 = match v1.clone() {
        Adt127::C0(_, h884) => {
            h884
        }
    };
    match m885 {
        Ok(v2) => {
            let m886 = f387(v2.clone())?;
            match m886 {
                None => {
                    Ok(f6())
                }
                Some(v3) => {
                    Ok(f388(v0, f125(f389(), f390(match v1 {
                        Adt127::C0(h887, _) => {
                            h887
                        }
                    }, v3))?, f391(match v2 {
                        Adt128::C0(_, _, h888) => {
                            h888
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f125(f392(v4.clone()), f127(Some(match v1 {
                Adt127::C0(h889, _) => {
                    h889
                }
            }), f393(v4)?))
        }
    }
}

pub fn f339(p894: List<Adt86>) -> R<bool> {
    let m893 = p894;
    {
        let mut m893 = m893;
        loop {
            let step = {
                let v0 = m893;
                let m890 = v0;
                match m890.uncons() {
                    None => {
                        Err::<List<Adt86>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f291(match v1.clone() {
                            Adt86::C0(h891, _) => {
                                h891
                            }
                        }, f394())? {
                            if f395(f396(44u64, match v1 {
                                Adt86::C0(_, h892) => {
                                    h892
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
                Ok(next) => { m893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f340(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt111>, v4: List<Str>, v5: Adt107) -> R<Adt127> {
    Ok(Adt127::C0(match v5.clone() {
        Adt107::C0(h895, _, _) => {
            h895
        }
    }, f397(v0, v1, v2, f398(v3, v4, v5)?)?))
}

pub fn f341(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt111> {
    let v4: Adt133 = f399(Adt113::C0, f134(v0.clone(), f400())?);
    let v5: Adt133 = f399(Adt113::C1, f134(v0.clone(), f401())?);
    let v6: Adt133 = if f402(v3) {
        f399(Adt113::C2, f403(f134(v0.clone(), f404())?)?)
    } else {
        f405()
    };
    Ok(Adt111::C0(match v4.clone() {
        Adt133::C0(h896, _) => {
            h896
        }
    }, match v5.clone() {
        Adt133::C0(h897, _) => {
            h897
        }
    }, match v6.clone() {
        Adt133::C0(h898, _) => {
            h898
        }
    }, f406(match v4 {
        Adt133::C0(_, h899) => {
            h899
        }
    }, f406(match v5 {
        Adt133::C0(_, h900) => {
            h900
        }
    }, match v6 {
        Adt133::C0(_, h901) => {
            h901
        }
    })), f407(v0, v1, v2)?))
}

pub fn f342(v0: Adt107) -> Option<Str> {
    let a903 = match v0.clone() {
        Adt107::C0(_, h902, _) => {
            h902
        }
    };
    let a904 = Str::lit("tools/call");
    if equal(a903, a904) {
        let m906 = match v0 {
            Adt107::C0(_, _, h905) => {
                h905
            }
        };
        match m906 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m907 = f186(Str::lit("name"), v1);
                match m907 {
                    Adt106::C0 => {
                        None::<Str>
                    }
                    Adt106::C1(v2) => {
                        f265(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f343() -> List<Str> {
    List::cons(f408(), List::<Str>::nil())
}

pub fn f344(v0: Adt5) -> R<Adt134> {
    let m908 = f186(Str::lit("id"), v0);
    match m908 {
        Adt106::C0 => {
            Ok(Adt134::C0)
        }
        Adt106::C1(v1) => {
            let m909 = v1;
            match m909 {
                Adt6::C0 => {
                    Ok(Adt134::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt134::C2)
                }
                Adt6::C2(v3) => {
                    let m910 = f409(v3)?;
                    match m910 {
                        None => {
                            Ok(Adt134::C2)
                        }
                        Some(v4) => {
                            Ok(Adt134::C1(Adt94::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt134::C1(Adt94::C0(v2)))
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

pub fn f345(v0: Adt5) -> Adt98 {
    if f346(v0) {
        Adt98::C8
    } else {
        Adt98::C3
    }
}

pub fn f346(v0: Adt5) -> bool {
    if f410(Str::lit("result"), v0.clone()) {
        true
    } else {
        f410(Str::lit("error"), v0)
    }
}

pub fn f347(v0: Adt94, v1: Adt5) -> Result<Adt107, Adt97> {
    let m911 = f186(Str::lit("jsonrpc"), v1.clone());
    match m911 {
        Adt106::C0 => {
            f287(Some(v0), Adt98::C2)
        }
        Adt106::C1(v2) => {
            let m912 = f265(v2);
            match m912 {
                None => {
                    f287(Some(v0), Adt98::C2)
                }
                Some(v3) => {
                    let a913 = v3;
                    let a914 = f278();
                    if equal(a913, a914) {
                        f411(v0, v1)
                    } else {
                        f287(Some(v0), Adt98::C2)
                    }
                }
            }
        }
    }
}

pub fn f348(v0: u64, v1: u64) -> Adt116 {
    let a915 = v1;
    let a916 = 0u64;
    if nat_eq(a915, a916) {
        Adt116::C0
    } else {
        let a917 = v1;
        let a918 = 2u64;
        if nat_eq(a917, a918) {
            Adt116::C2(v0)
        } else {
            Adt116::C1({
                let a922 = v0;
                let a923 = {
                    let a919 = v1;
                    let a920 = 3u64;
                    let a921 = 0u64;
                    nat_quot(a919, a920, a921)
                };
                nat_sub(a922, a923)
            })
        }
    }
}

pub fn f349(p951: Adt117, p952: u64, p953: u64, p954: List<u64>) -> R<u64> {
    let m950 = (p951, (p952, (p953, p954)));
    {
        let mut m950 = m950;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m950;
                let m924 = v3;
                match m924.uncons() {
                    None => {
                        let m925 = v0;
                        match m925 {
                            Adt117::C0 => {
                                Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt117::C1 => {
                                Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt117::C2 => {
                                Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt117::C3 => {
                                Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m926 = v0;
                        match m926 {
                            Adt117::C0 => {
                                let a927 = v4;
                                let a928 = 128u64;
                                if nat_lt(a927, a928) {
                                    Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a929 = v4;
                                    let a930 = 194u64;
                                    if nat_lt(a929, a930) {
                                        let v6: u64 = f412({
                                            let a931 = List::cons(v4, v5);
                                            length_list(a931)
                                        })?;
                                        Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a932 = v4;
                                        let a933 = 224u64;
                                        if nat_lt(a932, a933) {
                                            Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a934 = v4;
                                            let a935 = 224u64;
                                            if nat_eq(a934, a935) {
                                                Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a936 = v4;
                                                let a937 = 237u64;
                                                if nat_eq(a936, a937) {
                                                    Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a938 = v4;
                                                    let a939 = 240u64;
                                                    if nat_lt(a938, a939) {
                                                        Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a940 = v4;
                                                        let a941 = 240u64;
                                                        if nat_eq(a940, a941) {
                                                            Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a942 = v4;
                                                            let a943 = 244u64;
                                                            if nat_eq(a942, a943) {
                                                                Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a944 = v4;
                                                                let a945 = 244u64;
                                                                if nat_lt(a944, a945) {
                                                                    Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f412({
                                                                        let a946 = List::cons(v4, v5);
                                                                        length_list(a946)
                                                                    })?;
                                                                    Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt117::C1 => {
                                if f413(v4, v1, v2) {
                                    Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f412({
                                        let a947 = List::cons(v4, v5);
                                        length_list(a947)
                                    })?;
                                    Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt117::C2 => {
                                if f413(v4, v1, v2) {
                                    Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f412({
                                        let a948 = List::cons(v4, v5);
                                        length_list(a948)
                                    })?;
                                    Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt117::C3 => {
                                if f413(v4, v1, v2) {
                                    Ok::<(Adt117, (u64, (u64, List<u64>))), R<u64>>((Adt117::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f412({
                                        let a949 = List::cons(v4, v5);
                                        length_list(a949)
                                    })?;
                                    Err::<(Adt117, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m950 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Adt118, v1: u64) -> R<Result<Adt6, Adt96>> {
    let m956 = match v0.clone() {
        Adt118::C0(h955, _, _) => {
            h955
        }
    };
    match m956 {
        Adt119::C0 => {
            Ok(Err::<Adt6, Adt96>(Adt96::C3))
        }
        Adt119::C1 => {
            Ok(Err::<Adt6, Adt96>(Adt96::C3))
        }
        Adt119::C2 => {
            Ok(Err::<Adt6, Adt96>(Adt96::C3))
        }
        Adt119::C3 => {
            Ok(Err::<Adt6, Adt96>(Adt96::C3))
        }
        Adt119::C4 => {
            Ok(Err::<Adt6, Adt96>(Adt96::C3))
        }
        Adt119::C5 => {
            Ok(Err::<Adt6, Adt96>(Adt96::C3))
        }
        Adt119::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt96>(Adt96::C3))
        }
        Adt119::C7(_, _) => {
            Ok(Err::<Adt6, Adt96>(Adt96::C3))
        }
        Adt119::C8(v3) => {
            let m957 = f414(v3)?;
            match m957 {
                Adt139::C0 => {
                    Ok(Err::<Adt6, Adt96>(Adt96::C7(v1)))
                }
                Adt139::C1 => {
                    Ok(Err::<Adt6, Adt96>(Adt96::C8(v1)))
                }
                Adt139::C2(v4) => {
                    let m962 = {
                        let m961 = f415(v4, match v0.clone() {
                            Adt118::C0(_, h959, _) => {
                                h959
                            }
                        }, match v0 {
                            Adt118::C0(_, _, h960) => {
                                h960
                            }
                        });
                        match m961 {
                            Adt118::C0(h958, _, _) => {
                                h958
                            }
                        }
                    };
                    match m962 {
                        Adt119::C0 => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C1 => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C2 => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C3 => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C4 => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C5 => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C7(_, _) => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C8(_) => {
                            Ok(Err::<Adt6, Adt96>(Adt96::C3))
                        }
                        Adt119::C9(v5) => {
                            Ok(Ok::<Adt6, Adt96>(v5))
                        }
                    }
                }
            }
        }
        Adt119::C9(v2) => {
            Ok(Ok::<Adt6, Adt96>(v2))
        }
    }
}

pub fn f351(v0: Adt118, v1: u64, v2: u64) -> R<Adt135> {
    let m964 = match v0.clone() {
        Adt118::C0(h963, _, _) => {
            h963
        }
    };
    match m964 {
        Adt119::C0 => {
            if f416(v2) {
                Ok(Adt135::C1(v0))
            } else {
                f417(v0, v1, v2)
            }
        }
        Adt119::C1 => {
            if f416(v2) {
                Ok(Adt135::C1(v0))
            } else {
                let a965 = v2;
                let a966 = 93u64;
                if nat_eq(a965, a966) {
                    Ok(f418(v0, v1))
                } else {
                    f417(v0, v1, v2)
                }
            }
        }
        Adt119::C2 => {
            Ok(f419(v0, v1, v2))
        }
        Adt119::C3 => {
            if f416(v2) {
                Ok(Adt135::C1(v0))
            } else {
                let a967 = v2;
                let a968 = 125u64;
                if nat_eq(a967, a968) {
                    Ok(f420(v0, v1))
                } else {
                    let a969 = v2;
                    let a970 = 34u64;
                    if nat_eq(a969, a970) {
                        Ok(f421(v0, Adt119::C6(Adt120::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f422(Adt96::C2(v1)))
                    }
                }
            }
        }
        Adt119::C4 => {
            if f416(v2) {
                Ok(Adt135::C1(v0))
            } else {
                let a971 = v2;
                let a972 = 34u64;
                if nat_eq(a971, a972) {
                    Ok(f421(v0, Adt119::C6(Adt120::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f422(Adt96::C2(v1)))
                }
            }
        }
        Adt119::C5 => {
            if f416(v2) {
                Ok(Adt135::C1(v0))
            } else {
                let a973 = v2;
                let a974 = 58u64;
                if nat_eq(a973, a974) {
                    Ok(f421(v0, Adt119::C0))
                } else {
                    Ok(f422(Adt96::C2(v1)))
                }
            }
        }
        Adt119::C6(v3, v4, v5) => {
            f423(v0, v1, v2, v3, v4, v5)
        }
        Adt119::C7(v6, v7) => {
            f424(v0, v1, v2, v6, v7)
        }
        Adt119::C8(v8) => {
            f425(v0, v1, v2, v8)
        }
        Adt119::C9(_) => {
            Ok(f419(v0, v1, v2))
        }
    }
}

pub fn f352(v0: u64) -> R<u64> {
    let a975 = 64u64;
    let a976 = v0;
    if if nat_lt(a975, a976) {
        let a977 = v0;
        let a978 = 91u64;
        nat_lt(a977, a978)
    } else {
        false
    } {
        let a979 = v0;
        let a980 = 32u64;
        nat_add(a979, a980)
    } else {
        Ok(v0)
    }
}

pub fn f353(p983: List<u64>) -> List<u64> {
    let m982 = p983;
    {
        let mut m982 = m982;
        loop {
            let step = {
                let v0 = m982;
                let m981 = v0;
                match m981.uncons() {
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
                Ok(next) => { m982 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: u64) -> bool {
    let a984 = v0;
    let a985 = 9u64;
    if nat_eq(a984, a985) {
        true
    } else {
        let a986 = 31u64;
        let a987 = v0;
        if nat_lt(a986, a987) {
            let a988 = v0;
            let a989 = 127u64;
            nat_lt(a988, a989)
        } else {
            false
        }
    }
}

pub fn f355(p992: List<u64>) -> bool {
    let m991 = p992;
    {
        let mut m991 = m991;
        loop {
            let step = {
                let v0 = m991;
                let m990 = v0;
                match m990.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f427(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f356(p995: List<u64>, v1: bool) -> bool {
    let m994 = p995;
    {
        let mut m994 = m994;
        loop {
            let step = {
                let v0 = m994;
                let m993 = v0;
                match m993.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f428(v2)
                        } else {
                            f429(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m994 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357() -> u64 {
    64u64
}

pub fn f358(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m996 = f261(Str::lit("type"), v1);
    match m996 {
        Adt106::C0 => {
            Some(Adt91::C3(v0))
        }
        Adt106::C1(v2) => {
            let m997 = f265(v2);
            match m997 {
                None => {
                    Some(Adt91::C3(v0))
                }
                Some(v3) => {
                    let a998 = v3;
                    let a999 = Str::lit("object");
                    if equal(a998, a999) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f359(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1000 = f261(Str::lit("$schema"), v1);
    match m1000 {
        Adt106::C0 => {
            None::<Adt91>
        }
        Adt106::C1(v2) => {
            let m1001 = f265(v2);
            match m1001 {
                None => {
                    Some(Adt91::C2(v0))
                }
                Some(v3) => {
                    let a1002 = v3;
                    let a1003 = f430();
                    if equal(a1002, a1003) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f360(v0: Adt35) -> R<Option<Adt91>> {
    let m1005 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h1004, _, _) => {
            h1004
        }
    };
    match m1005 {
        None => {
            Ok(None::<Adt91>)
        }
        Some(v1) => {
            let m1008 = f359(match v0.clone() {
                Adt35::C0(h1006, _, _, _, _, _, _, _) => {
                    h1006
                }
            }, match v1.clone() {
                Adt5::C0(h1007) => {
                    h1007
                }
            });
            match m1008 {
                None => {
                    let m1012 = f305(f306(), match v0 {
                        Adt35::C0(h1009, _, _, _, _, _, _, _) => {
                            h1009
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1010) => {
                            h1010
                        }
                    }), List::cons(Adt108::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1011) => {
                            h1011
                        }
                    }), false, List::<Str>::nil()), List::<Adt108>::nil()), f307())?;
                    match m1012 {
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

pub fn f361(v0: Str, v1: Adt6, v2: Adt108, v3: Adt124, v4: List<Adt108>) -> R<Result<(List<Adt108>, Adt124), Adt91>> {
    let m1013 = v2;
    match m1013 {
        Adt108::C0(v5, v6, v7) => {
            let m1014 = f316(v5.clone());
            match m1014 {
                None => {
                    let m1015 = f187(v5);
                    match m1015 {
                        None => {
                            Ok(Err::<(List<Adt108>, Adt124), Adt91>(Adt91::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f431(v0, v1, match v9 {
                                Adt5::C0(h1016) => {
                                    h1016
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt108>, Adt124), Adt91>((v4, v3)))
                }
            }
        }
    }
}

pub fn f362(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt102>) -> R<Adt125> {
    let m1017 = f432(v1.clone(), v2.clone())?;
    match m1017 {
        Adt125::C0(v4) => {
            Ok(Adt125::C0(v4))
        }
        Adt125::C1(v5) => {
            Ok(Adt125::C1(v5))
        }
        Adt125::C2(v6) => {
            Ok(Adt125::C2(v6))
        }
        Adt125::C3(_) => {
            f433(v0, v1, v2, v3)
        }
    }
}

pub fn f363(v0: List<Adt6>, v1: Adt6, v2: List<Adt102>, v3: u64, v4: List<Adt104>) -> Adt109 {
    let m1018 = v0;
    match m1018.uncons() {
        None => {
            let a1019 = v3;
            let a1020 = 1u64;
            if nat_eq(a1019, a1020) {
                Adt109::C0(Adt103::C0, v4)
            } else {
                Adt109::C0(Adt103::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt109::C0(Adt103::C0, List::cons(Adt104::C0(v5, v1.clone(), v2.clone()), List::cons(Adt104::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f364(v0: Option<Adt6>, v1: Adt6, v2: List<Adt102>, v3: List<Adt104>) -> Adt109 {
    let m1021 = v0;
    match m1021 {
        None => {
            Adt109::C0(Adt103::C0, v3)
        }
        Some(v4) => {
            Adt109::C0(Adt103::C0, List::cons(Adt104::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f365(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt102>) -> Adt125 {
    let m1022 = f261(Str::lit("anyOf"), v0);
    match m1022 {
        Adt106::C0 => {
            Adt125::C3(List::<Adt104>::nil())
        }
        Adt106::C1(v3) => {
            let m1023 = f241(v3);
            match m1023 {
                None => {
                    Adt125::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1024 = v4;
                    match m1024.uncons() {
                        None => {
                            Adt125::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt125::C3(List::cons(Adt104::C0(v5, v1.clone(), v2.clone()), List::cons(Adt104::C1(v6, v1, v2), List::<Adt104>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f366(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt102>) -> Adt125 {
    let m1025 = f261(Str::lit("oneOf"), v0);
    match m1025 {
        Adt106::C0 => {
            Adt125::C3(List::<Adt104>::nil())
        }
        Adt106::C1(v3) => {
            let m1026 = f241(v3);
            match m1026 {
                None => {
                    Adt125::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1027 = v4;
                    match m1027.uncons() {
                        None => {
                            Adt125::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt125::C3(List::cons(Adt104::C0(v5, v1.clone(), v2.clone()), List::cons(Adt104::C2(v6, v1, v2, 0u64), List::<Adt104>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f367(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt102>) -> Adt125 {
    let m1028 = f261(Str::lit("not"), v0);
    match m1028 {
        Adt106::C0 => {
            Adt125::C3(List::<Adt104>::nil())
        }
        Adt106::C1(v3) => {
            Adt125::C3(List::cons(Adt104::C0(v3, v1, v2.clone()), List::cons(Adt104::C3(v2), List::<Adt104>::nil())))
        }
    }
}

pub fn f368(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt102>) -> Adt125 {
    let m1029 = f261(Str::lit("if"), v0.clone());
    match m1029 {
        Adt106::C0 => {
            Adt125::C3(List::<Adt104>::nil())
        }
        Adt106::C1(v3) => {
            Adt125::C3(List::cons(Adt104::C0(v3, v1.clone(), v2.clone()), List::cons(Adt104::C4(f434(Str::lit("then"), v0.clone()), f434(Str::lit("else"), v0), v1, v2), List::<Adt104>::nil())))
        }
    }
}

pub fn f369(v0: List<Adt6>, v1: Adt6, v2: List<Adt102>, v3: List<Adt104>) -> Adt109 {
    let m1030 = v0;
    match m1030.uncons() {
        None => {
            Adt109::C0(Adt103::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt109::C0(Adt103::C0, List::cons(Adt104::C0(v4, v1.clone(), v2.clone()), List::cons(Adt104::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f370(v0: Adt102) -> R<Str> {
    let m1031 = v0;
    match m1031 {
        Adt102::C0(v1) => {
            f435(v1)
        }
        Adt102::C1(v2) => {
            let a1032 = f240(v2, 0i64)?;
            Ok(format_i64(a1032))
        }
    }
}

pub fn f371(v0: Str) -> R<bool> {
    let m1034 = f436(f206({
        let a1033 = v0;
        utf8_encode(a1033)
    })?)?;
    match m1034 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f372(v0: Adt37) -> bool {
    let m1036 = match v0 {
        Adt37::C0(_, _, h1035) => {
            h1035
        }
    };
    match m1036 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f373(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1038 = match v0.clone() {
        Adt49::C0(_, h1037) => {
            h1037
        }
    };
    match m1038.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h1039, _) => {
                    h1039
                }
            })))
        }
        Some((_, _)) => {
            if f437(match v0.clone() {
                Adt49::C0(_, h1040) => {
                    h1040
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f374(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1044 = {
        let m1043 = match v1.clone() {
            Adt49::C0(h1042, _) => {
                h1042
            }
        };
        match m1043 {
            Adt37::C0(_, h1041, _) => {
                h1041
            }
        }
    };
    match m1044 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1047 = f146(Adt6::C5(match v0.clone() {
                Adt5::C0(h1045) => {
                    h1045
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1046) => {
                    h1046
                }
            }), v2)?;
            match m1047 {
                Adt101::C0 => {
                    f373(v1)
                }
                Adt101::C1(_, _) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt101::C2(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt101::C3(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt101::C4 => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f375(p1053: List<u64>, p1054: List<u64>) -> Option<List<u64>> {
    let m1052 = (p1053, p1054);
    {
        let mut m1052 = m1052;
        loop {
            let step = {
                let (v0, v1) = m1052;
                let m1048 = v0;
                match m1048.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1049 = v1;
                        match m1049.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1050 = v2;
                                let a1051 = v4;
                                if nat_eq(a1050, a1051) {
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
                Ok(next) => { m1052 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Str) -> R<List<u64>> {
    f206({
        let a1055 = v0;
        utf8_encode(a1055)
    })
}

pub fn f377() -> Str {
    Str::lit("test://template/")
}

pub fn f378(v0: List<u64>) -> List<u64> {
    f332(v0, List::<u64>::nil())
}

pub fn f379() -> Str {
    Str::lit("data")
}

pub fn f380(v0: List<u64>) -> bool {
    let m1056 = v0;
    match m1056.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f381(p1061: List<u64>) -> bool {
    let m1060 = p1061;
    {
        let mut m1060 = m1060;
        loop {
            let step = {
                let v0 = m1060;
                let m1057 = v0;
                match m1057.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1058 = v1;
                        let a1059 = 47u64;
                        if nat_eq(a1058, a1059) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
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

pub fn f382(p1066: Bytes, p1067: List<Bytes>) -> Bytes {
    let m1065 = (p1066, p1067);
    {
        let mut m1065 = m1065;
        loop {
            let step = {
                let (v0, v1) = m1065;
                let m1062 = v1;
                match m1062.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1063 = v2;
                            let a1064 = v0;
                            append_bytes(a1063, a1064)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(p1070: List<Bytes>, p1071: List<u64>) -> List<Bytes> {
    let m1069 = (p1070, p1071);
    {
        let mut m1069 = m1069;
        loop {
            let step = {
                let (v0, v1) = m1069;
                let m1068 = v1;
                match m1068.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f438(f439(v2), v0), v3))
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

pub fn f384(v0: Adt106, v1: Str) -> bool {
    let m1072 = v0;
    match m1072 {
        Adt106::C0 => {
            false
        }
        Adt106::C1(v2) => {
            let m1073 = f265(v2);
            match m1073 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1074 = v3;
                    let a1075 = v1;
                    equal(a1074, a1075)
                }
            }
        }
    }
}

pub fn f385(p1081: List<u64>, p1082: List<u64>) -> bool {
    let m1080 = (p1081, p1082);
    {
        let mut m1080 = m1080;
        loop {
            let step = {
                let (v0, v1) = m1080;
                let m1076 = v0;
                match m1076.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1077 = v1;
                        match m1077.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1078 = v2;
                                let a1079 = v4;
                                if nat_eq(a1078, a1079) {
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
                Ok(next) => { m1080 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: Adt110, v1: List<Adt110>, v2: List<u64>) -> R<Adt136> {
    let m1083 = v0;
    match m1083 {
        Adt110::C0(v3) => {
            Ok(Adt136::C1(v1, f332(v3, v2)))
        }
        Adt110::C1(v4, v5) => {
            f440(v4, v5, v1, v2)
        }
        Adt110::C2(v6, v7) => {
            let m1084 = v7;
            match m1084.uncons() {
                None => {
                    Ok(Adt136::C1(List::cons(Adt110::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt136::C1(List::cons(Adt110::C1(v6, v8), List::cons(Adt110::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt110::C3(v10, v11) => {
            let m1085 = v11.clone();
            match m1085.uncons() {
                None => {
                    Ok(Adt136::C1(List::cons(Adt110::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt136::C1(List::cons(Adt110::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt110::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt110::C4(v14, v15) => {
            let m1086 = v15;
            match m1086.uncons() {
                None => {
                    Ok(Adt136::C1(List::cons(Adt110::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt136::C1(List::cons(Adt110::C0({
                        let a1088 = f441({
                            let (h1087, _) = v16;
                            h1087
                        })?;
                        let a1089 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1088, a1089)
                    }), List::cons(Adt110::C1(v14, {
                        let (_, h1090) = v16;
                        h1090
                    }), List::cons(Adt110::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt110::C5(v18, v19) => {
            let m1091 = v19.clone();
            match m1091.uncons() {
                None => {
                    Ok(Adt136::C1(List::cons(Adt110::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt136::C1(List::cons(Adt110::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt110::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f387(v0: Adt128) -> R<Option<Adt5>> {
    let m1093 = f442(match v0.clone() {
        Adt128::C0(h1092, _, _) => {
            h1092
        }
    })?;
    match m1093 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1096 = v1;
                let a1097 = {
                    let m1095 = f443(v0);
                    match m1095 {
                        Adt5::C0(h1094) => {
                            h1094
                        }
                    }
                };
                append_list(a1096, a1097)
            })))
        }
    }
}

pub fn f388(v0: bool, v1: Adt88, v2: Option<List<List<u64>>>) -> Adt88 {
    let m1098 = v2;
    match m1098 {
        None => {
            f6()
        }
        Some(v3) => {
            f444(v0, v1, f50(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f389() -> u64 {
    200u64
}

pub fn f390(v0: Adt94, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f278())), List::cons((Str::lit("id"), f333(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1099) => {
            h1099
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f391(p1103: List<Adt130>, p1104: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1102 = (p1103, p1104);
    {
        let mut m1102 = m1102;
        loop {
            let step = {
                let (v0, v1) = m1102;
                let m1100 = v0;
                match m1100.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1101 = f445(v2)?;
                        match m1101 {
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
                Ok(next) => { m1102 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: Adt131) -> u64 {
    let m1105 = v0;
    match m1105 {
        Adt131::C0(_) => {
            f215()
        }
        Adt131::C1(_, _) => {
            f215()
        }
        Adt131::C2(_) => {
            f215()
        }
        Adt131::C3(_) => {
            f446()
        }
        Adt131::C4(v6) => {
            f447(v6)
        }
    }
}

pub fn f393(v0: Adt131) -> R<Adt80> {
    Ok(Adt80::C0(f448(v0.clone()), f449(v0.clone()), f450(v0)?))
}

pub fn f394() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f395(p1108: List<List<u64>>) -> R<bool> {
    let m1107 = p1108;
    {
        let mut m1107 = m1107;
        loop {
            let step = {
                let v0 = m1107;
                let m1106 = v0;
                match m1106.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f451(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: u64, p1113: List<u64>, p1114: List<u64>, p1115: List<List<u64>>) -> List<List<u64>> {
    let m1112 = (p1113, (p1114, p1115));
    {
        let mut m1112 = m1112;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1112;
                let m1109 = v1;
                match m1109.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f50(List::cons(f301(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1110 = v4;
                        let a1111 = v0;
                        if nat_eq(a1110, a1111) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f301(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt137, Adt131>) -> R<Result<Adt128, Adt131>> {
    let m1116 = v3;
    match m1116 {
        Ok(v5) => {
            f452(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(v4))
        }
    }
}

pub fn f398(v0: Option<Adt111>, v1: List<Str>, v2: Adt107) -> R<Result<Adt137, Adt131>> {
    Ok(f453(f454(match v2.clone() {
        Adt107::C0(_, h1117, _) => {
            h1117
        }
    }), f455(v0, v1, v2)?))
}

pub fn f399(v0: Adt113, v1: Adt99) -> Adt133 {
    let m1118 = v1;
    match m1118 {
        Adt99::C0 => {
            f405()
        }
        Adt99::C1 => {
            Adt133::C0(None::<Str>, Some(Adt112::C6(v0)))
        }
        Adt99::C2 => {
            Adt133::C0(None::<Str>, Some(Adt112::C7(v0)))
        }
        Adt99::C3(v2) => {
            let m1119 = f456(v2);
            match m1119 {
                None => {
                    Adt133::C0(None::<Str>, Some(Adt112::C6(v0)))
                }
                Some(v3) => {
                    Adt133::C0(Some(v3), None::<Adt112>)
                }
            }
        }
    }
}

pub fn f400() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f401() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f402(v0: Str) -> bool {
    let m1120 = f457(v0);
    match m1120 {
        Adt144::C0 => {
            false
        }
        Adt144::C1 => {
            true
        }
        Adt144::C2 => {
            true
        }
        Adt144::C3 => {
            true
        }
    }
}

pub fn f403(v0: Adt99) -> R<Adt99> {
    let m1121 = v0.clone();
    match m1121 {
        Adt99::C0 => {
            Ok(v0)
        }
        Adt99::C1 => {
            Ok(v0)
        }
        Adt99::C2 => {
            Ok(v0)
        }
        Adt99::C3(v1) => {
            let m1122 = f458(v1);
            match m1122 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1123 = f436(v2)?;
                    match m1123 {
                        None => {
                            Ok(Adt99::C1)
                        }
                        Some(v3) => {
                            Ok(Adt99::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f404() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f405() -> Adt133 {
    Adt133::C0(None::<Str>, None::<Adt112>)
}

pub fn f406(v0: Option<Adt112>, v1: Option<Adt112>) -> Option<Adt112> {
    let m1124 = v0.clone();
    match m1124 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f407(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt114>> {
    Ok(f459(f460(v0, v1, v2, List::<Adt114>::nil())?, List::<Adt114>::nil()))
}

pub fn f408() -> Str {
    Str::lit("2026-07-28")
}

pub fn f409(v0: Adt7) -> R<Option<i64>> {
    let m1127 = f461(40u64, match v0.clone() {
        Adt7::C0(h1125, _) => {
            h1125
        }
    }, match v0 {
        Adt7::C0(_, h1126) => {
            h1126
        }
    })?;
    match m1127 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f462(v1))
        }
    }
}

pub fn f410(v0: Str, v1: Adt5) -> bool {
    f463(f186(v0, v1))
}

pub fn f411(v0: Adt94, v1: Adt5) -> Result<Adt107, Adt97> {
    let m1128 = f186(Str::lit("method"), v1.clone());
    match m1128 {
        Adt106::C0 => {
            f287(Some(v0), f345(v1))
        }
        Adt106::C1(v2) => {
            let m1129 = f265(v2);
            match m1129 {
                None => {
                    f287(Some(v0), Adt98::C4)
                }
                Some(v3) => {
                    f464(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f412(v0: u64) -> R<u64> {
    let a1132 = {
        let a1130 = v0;
        let a1131 = 3u64;
        nat_mul(a1130, a1131)?
    };
    let a1133 = 1u64;
    nat_add(a1132, a1133)
}

pub fn f413(v0: u64, v1: u64, v2: u64) -> bool {
    let a1134 = v1;
    let a1135 = v0;
    if nat_le(a1134, a1135) {
        let a1136 = v0;
        let a1137 = v2;
        nat_le(a1136, a1137)
    } else {
        false
    }
}

pub fn f414(v0: Adt121) -> R<Adt139> {
    if f465(match v0.clone() {
        Adt121::C0(_, h1138, _, _, _, _, _) => {
            h1138
        }
    }) {
        let v1: i64 = {
            let a1144 = if match v0.clone() {
                Adt121::C0(_, _, _, _, _, h1139, _) => {
                    h1139
                }
            } {
                let a1141 = match v0.clone() {
                    Adt121::C0(_, _, _, _, _, _, h1140) => {
                        h1140
                    }
                };
                int_neg(a1141)?
            } else {
                match v0.clone() {
                    Adt121::C0(_, _, _, _, _, _, h1142) => {
                        h1142
                    }
                }
            };
            let a1145 = match v0.clone() {
                Adt121::C0(_, _, _, _, h1143, _, _) => {
                    h1143
                }
            };
            int_sub(a1144, a1145)?
        };
        let m1148 = {
            let a1146 = v1;
            let a1147 = -9999i64;
            compare(a1146, a1147)
        };
        if if match m1148 {
            Ordering::Less => {
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
            let m1151 = {
                let a1149 = 9999i64;
                let a1150 = v1;
                compare(a1149, a1150)
            };
            match m1151 {
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
            Ok(Adt139::C1)
        } else {
            Ok(Adt139::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt121::C0(h1152, _, _, _, _, _, _) => {
                    h1152
                }
            } {
                let a1154 = match v0 {
                    Adt121::C0(_, _, h1153, _, _, _, _) => {
                        h1153
                    }
                };
                int_neg(a1154)?
            } else {
                match v0 {
                    Adt121::C0(_, _, h1155, _, _, _, _) => {
                        h1155
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt139::C0)
    }
}

pub fn f415(v0: Adt6, v1: List<Adt123>, v2: u64) -> Adt118 {
    let m1156 = v1.clone();
    match m1156.uncons() {
        None => {
            Adt118::C0(Adt119::C9(v0), List::<Adt123>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1157 = v3;
            match m1157 {
                Adt123::C0(v5) => {
                    Adt118::C0(Adt119::C2, List::cons(Adt123::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt123::C1(v6, v7) => {
                    let m1158 = v7;
                    match m1158 {
                        None => {
                            Adt118::C0(Adt119::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt118::C0(Adt119::C2, List::cons(Adt123::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f416(v0: u64) -> bool {
    let a1159 = v0;
    let a1160 = 32u64;
    if nat_eq(a1159, a1160) {
        true
    } else {
        let a1161 = v0;
        let a1162 = 10u64;
        if nat_eq(a1161, a1162) {
            true
        } else {
            let a1163 = v0;
            let a1164 = 13u64;
            if nat_eq(a1163, a1164) {
                true
            } else {
                let a1165 = v0;
                let a1166 = 9u64;
                nat_eq(a1165, a1166)
            }
        }
    }
}

pub fn f417(v0: Adt118, v1: u64, v2: u64) -> R<Adt135> {
    let a1167 = v2;
    let a1168 = 34u64;
    if nat_eq(a1167, a1168) {
        Ok(f421(v0, Adt119::C6(Adt120::C0, List::<u64>::nil(), false)))
    } else {
        let a1169 = v2;
        let a1170 = 123u64;
        if nat_eq(a1169, a1170) {
            f466(v0, Adt123::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt119::C3, v1)
        } else {
            let a1171 = v2;
            let a1172 = 91u64;
            if nat_eq(a1171, a1172) {
                f466(v0, Adt123::C0(List::<Adt6>::nil()), Adt119::C1, v1)
            } else {
                let a1173 = v2;
                let a1174 = 116u64;
                if nat_eq(a1173, a1174) {
                    Ok(f421(v0, Adt119::C7(0u64, 1u64)))
                } else {
                    let a1175 = v2;
                    let a1176 = 102u64;
                    if nat_eq(a1175, a1176) {
                        Ok(f421(v0, Adt119::C7(1u64, 1u64)))
                    } else {
                        let a1177 = v2;
                        let a1178 = 110u64;
                        if nat_eq(a1177, a1178) {
                            Ok(f421(v0, Adt119::C7(2u64, 1u64)))
                        } else {
                            let a1179 = v2;
                            let a1180 = 45u64;
                            if nat_eq(a1179, a1180) {
                                Ok(f421(v0, f467(true, Adt122::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1181 = v2;
                                let a1182 = 48u64;
                                if nat_eq(a1181, a1182) {
                                    Ok(f421(v0, f467(false, Adt122::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f413(v2, 49u64, 57u64) {
                                        Ok(f421(v0, f467(false, Adt122::C2, f468({
                                            let a1183 = v2;
                                            let a1184 = 48u64;
                                            nat_sub(a1183, a1184)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f422(Adt96::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f418(v0: Adt118, v1: u64) -> Adt135 {
    let m1186 = match v0.clone() {
        Adt118::C0(_, h1185, _) => {
            h1185
        }
    };
    match m1186.uncons() {
        None => {
            f422(Adt96::C2(v1))
        }
        Some((v2, v3)) => {
            let m1187 = v2;
            match m1187 {
                Adt123::C0(v4) => {
                    Adt135::C1(f415(Adt6::C4(f469(v4, List::<Adt6>::nil())), v3, {
                        let a1189 = match v0 {
                            Adt118::C0(_, _, h1188) => {
                                h1188
                            }
                        };
                        let a1190 = 1u64;
                        nat_sub(a1189, a1190)
                    }))
                }
                Adt123::C1(_, _) => {
                    f422(Adt96::C2(v1))
                }
            }
        }
    }
}

pub fn f419(v0: Adt118, v1: u64, v2: u64) -> Adt135 {
    let m1192 = match v0.clone() {
        Adt118::C0(h1191, _, _) => {
            h1191
        }
    };
    match m1192 {
        Adt119::C0 => {
            f422(Adt96::C2(v1))
        }
        Adt119::C1 => {
            f422(Adt96::C2(v1))
        }
        Adt119::C2 => {
            if f416(v2) {
                Adt135::C1(v0)
            } else {
                let a1193 = v2;
                let a1194 = 44u64;
                if nat_eq(a1193, a1194) {
                    f470(v0, v1)
                } else {
                    let a1195 = v2;
                    let a1196 = 93u64;
                    if nat_eq(a1195, a1196) {
                        f418(v0, v1)
                    } else {
                        let a1197 = v2;
                        let a1198 = 125u64;
                        if nat_eq(a1197, a1198) {
                            f420(v0, v1)
                        } else {
                            f422(Adt96::C2(v1))
                        }
                    }
                }
            }
        }
        Adt119::C3 => {
            f422(Adt96::C2(v1))
        }
        Adt119::C4 => {
            f422(Adt96::C2(v1))
        }
        Adt119::C5 => {
            f422(Adt96::C2(v1))
        }
        Adt119::C6(_, _, _) => {
            f422(Adt96::C2(v1))
        }
        Adt119::C7(_, _) => {
            f422(Adt96::C2(v1))
        }
        Adt119::C8(_) => {
            f422(Adt96::C2(v1))
        }
        Adt119::C9(_) => {
            if f416(v2) {
                Adt135::C1(v0)
            } else {
                f422(Adt96::C10(v1))
            }
        }
    }
}

pub fn f420(v0: Adt118, v1: u64) -> Adt135 {
    let m1200 = match v0.clone() {
        Adt118::C0(_, h1199, _) => {
            h1199
        }
    };
    match m1200.uncons() {
        None => {
            f422(Adt96::C2(v1))
        }
        Some((v2, v3)) => {
            let m1201 = v2;
            match m1201 {
                Adt123::C0(_) => {
                    f422(Adt96::C2(v1))
                }
                Adt123::C1(v5, _) => {
                    Adt135::C1(f415(Adt6::C5(f471(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1203 = match v0 {
                            Adt118::C0(_, _, h1202) => {
                                h1202
                            }
                        };
                        let a1204 = 1u64;
                        nat_sub(a1203, a1204)
                    }))
                }
            }
        }
    }
}

pub fn f421(v0: Adt118, v1: Adt119) -> Adt135 {
    Adt135::C1(f472(v0, v1))
}

pub fn f422(v0: Adt96) -> Adt135 {
    Adt135::C0(v0)
}

pub fn f423(v0: Adt118, v1: u64, v2: u64, v3: Adt120, v4: List<u64>, v5: bool) -> R<Adt135> {
    let m1205 = v3;
    match m1205 {
        Adt120::C0 => {
            let a1206 = v2;
            let a1207 = 34u64;
            if nat_eq(a1206, a1207) {
                Ok(f473(v0, v1, v4, v5))
            } else {
                let a1208 = v2;
                let a1209 = 92u64;
                if nat_eq(a1208, a1209) {
                    Ok(f421(v0, Adt119::C6(Adt120::C1, v4, v5)))
                } else {
                    let a1210 = v2;
                    let a1211 = 32u64;
                    if nat_lt(a1210, a1211) {
                        Ok(f422(Adt96::C4(v1)))
                    } else {
                        Ok(f421(v0, Adt119::C6(Adt120::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt120::C1 => {
            let a1212 = v2;
            let a1213 = 117u64;
            if nat_eq(a1212, a1213) {
                Ok(f421(v0, Adt119::C6(Adt120::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1214 = f474(v2);
                let a1215 = 0u64;
                if nat_eq(a1214, a1215) {
                    Ok(f422(Adt96::C5(v1)))
                } else {
                    Ok(f421(v0, Adt119::C6(Adt120::C0, List::cons(f474(v2), v4), v5)))
                }
            }
        }
        Adt120::C2(v6, v7, v8) => {
            let a1216 = 15u64;
            let a1217 = f475(v2);
            if nat_lt(a1216, a1217) {
                Ok(f422(Adt96::C5(v1)))
            } else {
                let a1220 = {
                    let a1218 = v6;
                    let a1219 = 1u64;
                    nat_add(a1218, a1219)?
                };
                let a1221 = 4u64;
                if nat_eq(a1220, a1221) {
                    f476(v0, v1, {
                        let a1224 = {
                            let a1222 = v7;
                            let a1223 = 16u64;
                            nat_mul(a1222, a1223)?
                        };
                        let a1225 = f475(v2);
                        nat_add(a1224, a1225)?
                    }, v8, v4, v5)
                } else {
                    Ok(f421(v0, Adt119::C6(Adt120::C2({
                        let a1226 = v6;
                        let a1227 = 1u64;
                        nat_add(a1226, a1227)?
                    }, {
                        let a1230 = {
                            let a1228 = v7;
                            let a1229 = 16u64;
                            nat_mul(a1228, a1229)?
                        };
                        let a1231 = f475(v2);
                        nat_add(a1230, a1231)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt120::C3(v9) => {
            let a1232 = v2;
            let a1233 = 92u64;
            if nat_eq(a1232, a1233) {
                Ok(f421(v0, Adt119::C6(Adt120::C4(v9), v4, v5)))
            } else {
                Ok(f422(Adt96::C6(v1)))
            }
        }
        Adt120::C4(v10) => {
            let a1234 = v2;
            let a1235 = 117u64;
            if nat_eq(a1234, a1235) {
                Ok(f421(v0, Adt119::C6(Adt120::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f422(Adt96::C6(v1)))
            }
        }
    }
}

pub fn f424(v0: Adt118, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt135> {
    let a1236 = v2;
    let a1237 = f477(v3, v4);
    if nat_eq(a1236, a1237) {
        let a1241 = {
            let a1238 = v4;
            let a1239 = 1u64;
            nat_add(a1238, a1239)?
        };
        let a1242 = {
            let a1240 = f478(v3);
            length_list(a1240)
        };
        if nat_eq(a1241, a1242) {
            Ok(f479(v0, f480(v3)))
        } else {
            Ok(f421(v0, Adt119::C7(v3, {
                let a1243 = v4;
                let a1244 = 1u64;
                nat_add(a1243, a1244)?
            })))
        }
    } else {
        Ok(f422(Adt96::C2(v1)))
    }
}

pub fn f425(v0: Adt118, v1: u64, v2: u64, v3: Adt121) -> R<Adt135> {
    let m1246 = match v3.clone() {
        Adt121::C0(_, h1245, _, _, _, _, _) => {
            h1245
        }
    };
    match m1246 {
        Adt122::C0 => {
            let a1247 = v2;
            let a1248 = 48u64;
            if nat_eq(a1247, a1248) {
                Ok(f481(v0, v3, Adt122::C1))
            } else {
                if f413(v2, 49u64, 57u64) {
                    f482(v0, v1, v2, v3, false)
                } else {
                    Ok(f422(Adt96::C7(v1)))
                }
            }
        }
        Adt122::C1 => {
            let a1249 = v2;
            let a1250 = 46u64;
            if nat_eq(a1249, a1250) {
                Ok(f481(v0, v3, Adt122::C3))
            } else {
                if f483(v2) {
                    Ok(f481(v0, v3, Adt122::C5))
                } else {
                    if f484(v2) {
                        Ok(f422(Adt96::C7(v1)))
                    } else {
                        f485(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt122::C2 => {
            if f484(v2) {
                f482(v0, v1, v2, v3, false)
            } else {
                let a1251 = v2;
                let a1252 = 46u64;
                if nat_eq(a1251, a1252) {
                    Ok(f481(v0, v3, Adt122::C3))
                } else {
                    if f483(v2) {
                        Ok(f481(v0, v3, Adt122::C5))
                    } else {
                        f485(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt122::C3 => {
            if f484(v2) {
                f482(v0, v1, v2, v3, true)
            } else {
                Ok(f422(Adt96::C7(v1)))
            }
        }
        Adt122::C4 => {
            if f484(v2) {
                f482(v0, v1, v2, v3, true)
            } else {
                if f483(v2) {
                    Ok(f481(v0, v3, Adt122::C5))
                } else {
                    f485(v0, v1, v2, v3)
                }
            }
        }
        Adt122::C5 => {
            let a1253 = v2;
            let a1254 = 43u64;
            if nat_eq(a1253, a1254) {
                Ok(f421(v0, f467(match v3.clone() {
                    Adt121::C0(h1255, _, _, _, _, _, _) => {
                        h1255
                    }
                }, Adt122::C6, match v3.clone() {
                    Adt121::C0(_, _, h1256, _, _, _, _) => {
                        h1256
                    }
                }, match v3.clone() {
                    Adt121::C0(_, _, _, h1257, _, _, _) => {
                        h1257
                    }
                }, match v3 {
                    Adt121::C0(_, _, _, _, h1258, _, _) => {
                        h1258
                    }
                }, false, 0i64)))
            } else {
                let a1259 = v2;
                let a1260 = 45u64;
                if nat_eq(a1259, a1260) {
                    Ok(f421(v0, f467(match v3.clone() {
                        Adt121::C0(h1261, _, _, _, _, _, _) => {
                            h1261
                        }
                    }, Adt122::C6, match v3.clone() {
                        Adt121::C0(_, _, h1262, _, _, _, _) => {
                            h1262
                        }
                    }, match v3.clone() {
                        Adt121::C0(_, _, _, h1263, _, _, _) => {
                            h1263
                        }
                    }, match v3 {
                        Adt121::C0(_, _, _, _, h1264, _, _) => {
                            h1264
                        }
                    }, true, 0i64)))
                } else {
                    if f484(v2) {
                        f486(v0, v1, v2, Adt121::C0(match v3.clone() {
                            Adt121::C0(h1265, _, _, _, _, _, _) => {
                                h1265
                            }
                        }, match v3.clone() {
                            Adt121::C0(_, h1266, _, _, _, _, _) => {
                                h1266
                            }
                        }, match v3.clone() {
                            Adt121::C0(_, _, h1267, _, _, _, _) => {
                                h1267
                            }
                        }, match v3.clone() {
                            Adt121::C0(_, _, _, h1268, _, _, _) => {
                                h1268
                            }
                        }, match v3 {
                            Adt121::C0(_, _, _, _, h1269, _, _) => {
                                h1269
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f422(Adt96::C7(v1)))
                    }
                }
            }
        }
        Adt122::C6 => {
            if f484(v2) {
                f486(v0, v1, v2, v3.clone(), match v3 {
                    Adt121::C0(_, _, _, _, _, h1270, _) => {
                        h1270
                    }
                })
            } else {
                Ok(f422(Adt96::C7(v1)))
            }
        }
        Adt122::C7 => {
            if f484(v2) {
                f486(v0, v1, v2, v3.clone(), match v3 {
                    Adt121::C0(_, _, _, _, _, h1271, _) => {
                        h1271
                    }
                })
            } else {
                f485(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f426(v0: u64) -> bool {
    let a1272 = v0;
    let a1273 = 9u64;
    if nat_eq(a1272, a1273) {
        true
    } else {
        let a1274 = v0;
        let a1275 = 32u64;
        nat_eq(a1274, a1275)
    }
}

pub fn f427(v0: u64) -> bool {
    let a1276 = 47u64;
    let a1277 = v0;
    if nat_lt(a1276, a1277) {
        let a1278 = v0;
        let a1279 = 58u64;
        nat_lt(a1278, a1279)
    } else {
        false
    }
}

pub fn f428(v0: u64) -> bool {
    let a1280 = 48u64;
    let a1281 = v0;
    if if nat_le(a1280, a1281) {
        let a1282 = v0;
        let a1283 = 57u64;
        nat_le(a1282, a1283)
    } else {
        false
    } {
        true
    } else {
        let a1284 = 65u64;
        let a1285 = v0;
        if if nat_le(a1284, a1285) {
            let a1286 = v0;
            let a1287 = 90u64;
            nat_le(a1286, a1287)
        } else {
            false
        } {
            true
        } else {
            let a1288 = 97u64;
            let a1289 = v0;
            if if nat_le(a1288, a1289) {
                let a1290 = v0;
                let a1291 = 122u64;
                nat_le(a1290, a1291)
            } else {
                false
            } {
                true
            } else {
                let a1292 = v0;
                let a1293 = 33u64;
                if nat_eq(a1292, a1293) {
                    true
                } else {
                    let a1294 = v0;
                    let a1295 = 35u64;
                    if nat_eq(a1294, a1295) {
                        true
                    } else {
                        let a1296 = v0;
                        let a1297 = 36u64;
                        if nat_eq(a1296, a1297) {
                            true
                        } else {
                            let a1298 = v0;
                            let a1299 = 37u64;
                            if nat_eq(a1298, a1299) {
                                true
                            } else {
                                let a1300 = v0;
                                let a1301 = 38u64;
                                if nat_eq(a1300, a1301) {
                                    true
                                } else {
                                    let a1302 = v0;
                                    let a1303 = 39u64;
                                    if nat_eq(a1302, a1303) {
                                        true
                                    } else {
                                        let a1304 = v0;
                                        let a1305 = 42u64;
                                        if nat_eq(a1304, a1305) {
                                            true
                                        } else {
                                            let a1306 = v0;
                                            let a1307 = 43u64;
                                            if nat_eq(a1306, a1307) {
                                                true
                                            } else {
                                                let a1308 = v0;
                                                let a1309 = 45u64;
                                                if nat_eq(a1308, a1309) {
                                                    true
                                                } else {
                                                    let a1310 = v0;
                                                    let a1311 = 46u64;
                                                    if nat_eq(a1310, a1311) {
                                                        true
                                                    } else {
                                                        let a1312 = v0;
                                                        let a1313 = 94u64;
                                                        if nat_eq(a1312, a1313) {
                                                            true
                                                        } else {
                                                            let a1314 = v0;
                                                            let a1315 = 95u64;
                                                            if nat_eq(a1314, a1315) {
                                                                true
                                                            } else {
                                                                let a1316 = v0;
                                                                let a1317 = 96u64;
                                                                if nat_eq(a1316, a1317) {
                                                                    true
                                                                } else {
                                                                    let a1318 = v0;
                                                                    let a1319 = 124u64;
                                                                    if nat_eq(a1318, a1319) {
                                                                        true
                                                                    } else {
                                                                        let a1320 = v0;
                                                                        let a1321 = 126u64;
                                                                        nat_eq(a1320, a1321)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f429(v0: u64) -> bool {
    let a1322 = 48u64;
    let a1323 = v0;
    if if nat_le(a1322, a1323) {
        let a1324 = v0;
        let a1325 = 57u64;
        nat_le(a1324, a1325)
    } else {
        false
    } {
        true
    } else {
        let a1326 = 65u64;
        let a1327 = v0;
        if if nat_le(a1326, a1327) {
            let a1328 = v0;
            let a1329 = 90u64;
            nat_le(a1328, a1329)
        } else {
            false
        } {
            true
        } else {
            let a1330 = 97u64;
            let a1331 = v0;
            if if nat_le(a1330, a1331) {
                let a1332 = v0;
                let a1333 = 122u64;
                nat_le(a1332, a1333)
            } else {
                false
            } {
                true
            } else {
                let a1334 = v0;
                let a1335 = 95u64;
                if nat_eq(a1334, a1335) {
                    true
                } else {
                    let a1336 = v0;
                    let a1337 = 45u64;
                    if nat_eq(a1336, a1337) {
                        true
                    } else {
                        let a1338 = v0;
                        let a1339 = 46u64;
                        nat_eq(a1338, a1339)
                    }
                }
            }
        }
    }
}

pub fn f430() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f431(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt124, v6: List<Adt108>) -> R<Result<(List<Adt108>, Adt124), Adt91>> {
    let m1340 = f487(v0.clone(), v1, v2.clone());
    match m1340 {
        None => {
            let m1341 = f488(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1341 {
                Adt145::C0(v8) => {
                    Ok(Err::<(List<Adt108>, Adt124), Adt91>(v8))
                }
                Adt145::C1(v9) => {
                    Ok(Ok::<(List<Adt108>, Adt124), Adt91>((f489(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt108>, Adt124), Adt91>(v7))
        }
    }
}

pub fn f432(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt125> {
    Ok(f490(List::cons((Str::lit("type"), f491(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f492(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f493(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f494(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f494(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f494(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f494(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1342 = f495(v0.clone(), v1.clone())?;
        let a1343 = List::cons((Str::lit("required"), f496(v0, v1)), List::<(Str, Adt140)>::nil());
        append_list(a1342, a1343)
    })))))))))
}

pub fn f433(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt102>) -> R<Adt125> {
    Ok(f497(f498(v0, v1.clone(), v2.clone(), v3.clone()), f497(f499(v1.clone(), v2.clone(), v3.clone()), f497(f500(v1.clone(), v2.clone(), v3.clone()), f497(f501(v1.clone(), v2.clone(), v3.clone())?, f497(f502(v1.clone(), v2.clone(), v3.clone()), f497(f503(Adt105::C0, f365(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f497(f503(Adt105::C1, f366(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f497(f503(Adt105::C2, f367(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f503(Adt105::C3, f368(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f434(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1344 = f261(v0, v1);
    match m1344 {
        Adt106::C0 => {
            None::<Adt6>
        }
        Adt106::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f435(v0: Str) -> R<Str> {
    let m1347 = {
        let a1346 = f318(f319(), f332(f504(f206({
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

pub fn f436(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1349 = f505({
        let a1348 = v0.clone();
        length_list(a1348)
    }, v0, List::<u64>::nil())?;
    match m1349 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f332(v1, List::<u64>::nil())))
        }
    }
}

pub fn f437(v0: List<Adt50>) -> R<bool> {
    let m1350 = v0;
    match m1350.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f506(v2, match v1 {
                Adt50::C0(h1351, _, _) => {
                    h1351
                }
            })
        }
    }
}

pub fn f438(p1360: Bytes, p1361: List<Bytes>) -> List<Bytes> {
    let m1359 = (p1360, p1361);
    {
        let mut m1359 = m1359;
        loop {
            let step = {
                let (v0, v1) = m1359;
                let m1352 = v1;
                match m1352.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1355 = {
                            let a1353 = v2.clone();
                            length_bytes(a1353)
                        };
                        let a1356 = {
                            let a1354 = v0.clone();
                            length_bytes(a1354)
                        };
                        if nat_eq(a1355, a1356) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1357 = v2;
                                let a1358 = v0;
                                append_bytes(a1357, a1358)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: u64) -> Bytes {
    let m1365 = {
        let a1362 = f507();
        let a1363 = v0;
        let a1364 = 1u64;
        slice_bytes(a1362, a1363, a1364)
    };
    match m1365 {
        None => {
            f319()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f440(v0: u64, v1: Adt6, v2: List<Adt110>, v3: List<u64>) -> R<Adt136> {
    let m1366 = v1;
    match m1366 {
        Adt6::C0 => {
            Ok(Adt136::C1(v2, f332(f508(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt136::C1(v2, f332(if v4 {
                f508(0u64)
            } else {
                f508(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f509(v5.clone())? {
                Ok(Adt136::C1(v2, f332(f510(v5)?, v3)))
            } else {
                Ok(Adt136::C0(Adt96::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt136::C1(v2, f332(f441(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1367 = v0;
            let a1368 = f511();
            if nat_lt(a1367, a1368) {
                Ok(Adt136::C1(f512(91u64, Adt110::C2({
                    let a1369 = v0;
                    let a1370 = 1u64;
                    nat_add(a1369, a1370)?
                }, v7), v2), v3))
            } else {
                Ok(Adt136::C0(Adt96::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1371 = v0;
            let a1372 = f511();
            if nat_lt(a1371, a1372) {
                Ok(Adt136::C1(f512(123u64, Adt110::C4({
                    let a1373 = v0;
                    let a1374 = 1u64;
                    nat_add(a1373, a1374)?
                }, v8), v2), v3))
            } else {
                Ok(Adt136::C0(Adt96::C12))
            }
        }
    }
}

pub fn f441(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1376 = f332(f513(f206({
            let a1375 = v0;
            utf8_encode(a1375)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1377 = List::cons(34u64, List::<u64>::nil());
        append_list(a1376, a1377)
    }))
}

pub fn f442(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m1378 = v0;
    match m1378 {
        Adt32::C0(v1) => {
            Ok(Some(f514(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f515(v2)?))
        }
        Adt32::C2(v3) => {
            f516(f517(v3)?)
        }
        Adt32::C3(v4) => {
            f516(f517(match v4 {
                Adt49::C0(h1379, _) => {
                    h1379
                }
            })?)
        }
        Adt32::C4(v5) => {
            Ok(Some(f518(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f519(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f520(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f521(v8)?))
        }
        Adt32::C8(v9) => {
            f516(f522(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f523(v10)?))
        }
        Adt32::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt32::C12(v12) => {
            f516(f524(v12)?)
        }
        Adt32::C13(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f443(v0: Adt128) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f525(f526(match v0.clone() {
        Adt128::C0(h1380, _, _) => {
            h1380
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f527({
        let m1383 = match v0 {
            Adt128::C0(_, h1382, _) => {
                h1382
            }
        };
        match m1383 {
            Adt129::C0(h1381, _) => {
                h1381
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f444(v0: bool, v1: Adt88, v2: List<List<u64>>) -> Adt88 {
    if f528(v0, match v1.clone() {
        Adt88::C0(h1384, _, _, _) => {
            h1384
        }
    }, v2.clone()) {
        f529(match v1.clone() {
            Adt88::C0(h1385, _, _, _) => {
                h1385
            }
        }, {
            let a1387 = v2;
            let a1388 = List::cons(match v1 {
                Adt88::C0(_, _, _, h1386) => {
                    h1386
                }
            }, List::<List<u64>>::nil());
            append_list(a1387, a1388)
        })
    } else {
        v1
    }
}

pub fn f445(v0: Adt130) -> R<Option<List<u64>>> {
    let m1389 = f530(f531(v0))?;
    match m1389 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1390 = f208(v1)?;
            match m1390 {
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

pub fn f446() -> u64 {
    404u64
}

pub fn f447(v0: Adt84) -> u64 {
    let m1391 = v0;
    match m1391 {
        Adt84::C0(_) => {
            f215()
        }
        Adt84::C1(_) => {
            f215()
        }
        Adt84::C2(_) => {
            f215()
        }
        Adt84::C3(_) => {
            f215()
        }
        Adt84::C4(_) => {
            f215()
        }
        Adt84::C5(_) => {
            f215()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f532()
        }
        Adt84::C8(_) => {
            f215()
        }
        Adt84::C9(_) => {
            f21()
        }
        Adt84::C10 => {
            f215()
        }
        Adt84::C11 => {
            f215()
        }
        Adt84::C12(_) => {
            f21()
        }
        Adt84::C13 => {
            f21()
        }
        Adt84::C14 => {
            f21()
        }
    }
}

pub fn f448(v0: Adt131) -> i64 {
    let m1392 = v0;
    match m1392 {
        Adt131::C0(_) => {
            f533()
        }
        Adt131::C1(_, _) => {
            f534()
        }
        Adt131::C2(_) => {
            f535()
        }
        Adt131::C3(_) => {
            f536()
        }
        Adt131::C4(v6) => {
            f537(v6)
        }
    }
}

pub fn f449(v0: Adt131) -> Str {
    let m1393 = v0;
    match m1393 {
        Adt131::C0(v1) => {
            f538(v1)
        }
        Adt131::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt131::C2(v4) => {
            f539(v4)
        }
        Adt131::C3(_) => {
            Str::lit("Method not found")
        }
        Adt131::C4(v6) => {
            let m1394 = v6;
            match m1394 {
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
                Adt84::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt84::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt84::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt84::C13 => {
                    Str::lit("Internal error")
                }
                Adt84::C14 => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f450(v0: Adt131) -> R<Option<Adt6>> {
    let m1395 = v0;
    match m1395 {
        Adt131::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f540(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt131::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C4(v6) => {
            let m1396 = v6;
            match m1396 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f240(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f541(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C10 => {
                    Ok(None::<Adt6>)
                }
                Adt84::C11 => {
                    Ok(None::<Adt6>)
                }
                Adt84::C12(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C13 => {
                    Ok(None::<Adt6>)
                }
                Adt84::C14 => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f451(v0: List<u64>) -> R<bool> {
    let m1397 = f396(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1397.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f542(v2)? {
                Ok(false)
            } else {
                f291(f293(v1), f543())
            }
        }
    }
}

pub fn f452(v0: Adt0, v1: Fn0, v2: u64, v3: Adt137) -> R<Result<Adt128, Adt131>> {
    f544(v0.clone(), v1, v2, v3.clone(), f545(match v0 {
        Adt0::C0(_, _, h1398, _, _, _, _) => {
            h1398
        }
    }, match v3 {
        Adt137::C0(_, h1399, _, _) => {
            h1399
        }
    }))
}

pub fn f453(v0: Adt141, v1: Adt142) -> Result<Adt137, Adt131> {
    let m1400 = v1;
    match m1400 {
        Adt142::C0(v3) => {
            f546(v3, v0)
        }
        Adt142::C1(v2) => {
            Err::<Adt137, Adt131>(v2)
        }
    }
}

pub fn f454(v0: Str) -> Adt141 {
    let a1401 = v0.clone();
    let a1402 = Str::lit("server/discover");
    if equal(a1401, a1402) {
        Adt141::C1(Adt138::C0)
    } else {
        let a1403 = v0.clone();
        let a1404 = Str::lit("tools/list");
        if equal(a1403, a1404) {
            Adt141::C1(Adt138::C1)
        } else {
            let a1405 = v0.clone();
            let a1406 = Str::lit("tools/call");
            if equal(a1405, a1406) {
                Adt141::C1(Adt138::C2)
            } else {
                let a1407 = v0.clone();
                let a1408 = Str::lit("resources/list");
                if equal(a1407, a1408) {
                    Adt141::C1(Adt138::C3)
                } else {
                    let a1409 = v0.clone();
                    let a1410 = Str::lit("resources/read");
                    if equal(a1409, a1410) {
                        Adt141::C1(Adt138::C4)
                    } else {
                        let a1411 = v0.clone();
                        let a1412 = Str::lit("resources/templates/list");
                        if equal(a1411, a1412) {
                            Adt141::C1(Adt138::C5)
                        } else {
                            let a1413 = v0.clone();
                            let a1414 = Str::lit("prompts/list");
                            if equal(a1413, a1414) {
                                Adt141::C1(Adt138::C6)
                            } else {
                                let a1415 = v0.clone();
                                let a1416 = Str::lit("prompts/get");
                                if equal(a1415, a1416) {
                                    Adt141::C1(Adt138::C7)
                                } else {
                                    let a1417 = v0.clone();
                                    let a1418 = Str::lit("completion/complete");
                                    if equal(a1417, a1418) {
                                        Adt141::C1(Adt138::C8)
                                    } else {
                                        let a1419 = v0.clone();
                                        let a1420 = Str::lit("tasks/get");
                                        if equal(a1419, a1420) {
                                            Adt141::C1(Adt138::C9)
                                        } else {
                                            let a1421 = v0.clone();
                                            let a1422 = Str::lit("tasks/update");
                                            if equal(a1421, a1422) {
                                                Adt141::C1(Adt138::C10)
                                            } else {
                                                let a1423 = v0;
                                                let a1424 = Str::lit("tasks/cancel");
                                                if equal(a1423, a1424) {
                                                    Adt141::C1(Adt138::C11)
                                                } else {
                                                    Adt141::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f455(v0: Option<Adt111>, v1: List<Str>, v2: Adt107) -> R<Adt142> {
    f547(v0, v1, v2.clone(), f548(match v2 {
        Adt107::C0(_, _, h1425) => {
            h1425
        }
    }))
}

pub fn f456(v0: List<u64>) -> Option<Str> {
    let a1426 = f318(f319(), v0);
    utf8_decode(a1426)
}

pub fn f457(v0: Str) -> Adt144 {
    let m1427 = f454(v0);
    match m1427 {
        Adt141::C0 => {
            Adt144::C0
        }
        Adt141::C1(v1) => {
            f549(v1)
        }
    }
}

pub fn f458(v0: List<u64>) -> Option<List<u64>> {
    let m1428 = f550(f551(), v0);
    match m1428 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1429 = f550(f552(), f301(v1, List::<u64>::nil()));
            match m1429 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f301(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f459(p1432: List<Adt114>, p1433: List<Adt114>) -> List<Adt114> {
    let m1431 = (p1432, p1433);
    {
        let mut m1431 = m1431;
        loop {
            let step = {
                let (v0, v1) = m1431;
                let m1430 = v0;
                match m1430.uncons() {
                    None => {
                        Err::<(List<Adt114>, List<Adt114>), List<Adt114>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt114>, List<Adt114>), List<Adt114>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: List<Adt86>, p1440: List<Adt12>, v2: Option<Str>, p1441: List<Adt114>) -> R<List<Adt114>> {
    let m1439 = (p1440, p1441);
    {
        let mut m1439 = m1439;
        loop {
            let step = {
                let (v1, v3) = m1439;
                let m1434 = v1;
                match m1434.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt114>), R<List<Adt114>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f553(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt114>), R<List<Adt114>>>((v5, List::cons(Adt114::C0(match v4.clone() {
                                Adt12::C0(_, h1435, _, _) => {
                                    h1435
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1436, _) => {
                                    h1436
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1437) => {
                                    h1437
                                }
                            }, f554(v0.clone(), match v4 {
                                Adt12::C0(_, h1438, _, _) => {
                                    h1438
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt114>), R<List<Adt114>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(p1457: u64, p1458: i64, p1459: i64) -> R<Option<i64>> {
    let m1456 = (p1457, (p1458, p1459));
    {
        let mut m1456 = m1456;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1456;
                let m1442 = v0;
                if m1442 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1442 - 1;
                    if f555(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1445 = {
                            let a1443 = v2;
                            let a1444 = 0i64;
                            compare(a1443, a1444)
                        };
                        if match m1445 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f555({
                                let a1446 = v1;
                                let a1447 = 10i64;
                                let a1448 = 0i64;
                                int_rem(a1446, a1447, a1448)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1449 = v1;
                                    let a1450 = 10i64;
                                    let a1451 = 0i64;
                                    int_quot(a1449, a1450, a1451)?
                                }, f315(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1452 = v1;
                                let a1453 = 10i64;
                                int_mul(a1452, a1453)?
                            }, {
                                let a1454 = v2;
                                let a1455 = 1i64;
                                int_sub(a1454, a1455)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: i64) -> Option<i64> {
    let m1462 = {
        let a1460 = 9007199254740991i64;
        let a1461 = v0;
        compare(a1460, a1461)
    };
    if if match m1462 {
        Ordering::Less => {
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
        let m1465 = {
            let a1463 = v0;
            let a1464 = -9007199254740991i64;
            compare(a1463, a1464)
        };
        match m1465 {
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

pub fn f463(v0: Adt106) -> bool {
    let m1466 = v0;
    match m1466 {
        Adt106::C0 => {
            false
        }
        Adt106::C1(_) => {
            true
        }
    }
}

pub fn f464(v0: Adt94, v1: Str, v2: Adt5) -> Result<Adt107, Adt97> {
    let m1467 = f186(Str::lit("params"), v2);
    match m1467 {
        Adt106::C0 => {
            Ok::<Adt107, Adt97>(Adt107::C0(v0, v1, None::<Adt5>))
        }
        Adt106::C1(v3) => {
            let m1468 = f187(v3);
            match m1468 {
                None => {
                    f287(Some(v0), Adt98::C6)
                }
                Some(v4) => {
                    Ok::<Adt107, Adt97>(Adt107::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f465(v0: Adt122) -> bool {
    let m1469 = v0;
    match m1469 {
        Adt122::C0 => {
            false
        }
        Adt122::C1 => {
            true
        }
        Adt122::C2 => {
            true
        }
        Adt122::C3 => {
            false
        }
        Adt122::C4 => {
            true
        }
        Adt122::C5 => {
            false
        }
        Adt122::C6 => {
            false
        }
        Adt122::C7 => {
            true
        }
    }
}

pub fn f466(v0: Adt118, v1: Adt123, v2: Adt119, v3: u64) -> R<Adt135> {
    let a1471 = match v0.clone() {
        Adt118::C0(_, _, h1470) => {
            h1470
        }
    };
    let a1472 = f511();
    if nat_lt(a1471, a1472) {
        Ok(Adt135::C1(Adt118::C0(v2, List::cons(v1, match v0.clone() {
            Adt118::C0(_, h1473, _) => {
                h1473
            }
        }), {
            let a1475 = match v0 {
                Adt118::C0(_, _, h1474) => {
                    h1474
                }
            };
            let a1476 = 1u64;
            nat_add(a1475, a1476)?
        })))
    } else {
        Ok(f422(Adt96::C9(v3)))
    }
}

pub fn f467(v0: bool, v1: Adt122, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt119 {
    Adt119::C8(Adt121::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f468(v0: u64) -> i64 {
    let a1477 = v0;
    let a1478 = 0u64;
    if nat_eq(a1477, a1478) {
        0i64
    } else {
        let a1479 = v0;
        let a1480 = 1u64;
        if nat_eq(a1479, a1480) {
            1i64
        } else {
            let a1481 = v0;
            let a1482 = 2u64;
            if nat_eq(a1481, a1482) {
                2i64
            } else {
                let a1483 = v0;
                let a1484 = 3u64;
                if nat_eq(a1483, a1484) {
                    3i64
                } else {
                    let a1485 = v0;
                    let a1486 = 4u64;
                    if nat_eq(a1485, a1486) {
                        4i64
                    } else {
                        let a1487 = v0;
                        let a1488 = 5u64;
                        if nat_eq(a1487, a1488) {
                            5i64
                        } else {
                            let a1489 = v0;
                            let a1490 = 6u64;
                            if nat_eq(a1489, a1490) {
                                6i64
                            } else {
                                let a1491 = v0;
                                let a1492 = 7u64;
                                if nat_eq(a1491, a1492) {
                                    7i64
                                } else {
                                    let a1493 = v0;
                                    let a1494 = 8u64;
                                    if nat_eq(a1493, a1494) {
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

pub fn f469(p1497: List<Adt6>, p1498: List<Adt6>) -> List<Adt6> {
    let m1496 = (p1497, p1498);
    {
        let mut m1496 = m1496;
        loop {
            let step = {
                let (v0, v1) = m1496;
                let m1495 = v0;
                match m1495.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f470(v0: Adt118, v1: u64) -> Adt135 {
    let m1500 = match v0.clone() {
        Adt118::C0(_, h1499, _) => {
            h1499
        }
    };
    match m1500.uncons() {
        None => {
            f422(Adt96::C2(v1))
        }
        Some((v2, _)) => {
            let m1501 = v2;
            match m1501 {
                Adt123::C0(_) => {
                    f421(v0, Adt119::C0)
                }
                Adt123::C1(_, _) => {
                    f421(v0, Adt119::C4)
                }
            }
        }
    }
}

pub fn f471(p1504: List<(Str, Adt6)>, p1505: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1503 = (p1504, p1505);
    {
        let mut m1503 = m1503;
        loop {
            let step = {
                let (v0, v1) = m1503;
                let m1502 = v0;
                match m1502.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Adt118, v1: Adt119) -> Adt118 {
    Adt118::C0(v1, match v0.clone() {
        Adt118::C0(_, h1506, _) => {
            h1506
        }
    }, match v0 {
        Adt118::C0(_, _, h1507) => {
            h1507
        }
    })
}

pub fn f473(v0: Adt118, v1: u64, v2: List<u64>, v3: bool) -> Adt135 {
    let m1509 = {
        let a1508 = f318(f319(), f332(v2, List::<u64>::nil()));
        utf8_decode(a1508)
    };
    match m1509 {
        None => {
            f422(Adt96::C14)
        }
        Some(v4) => {
            if v3 {
                f556(v0, v1, v4)
            } else {
                f479(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f474(v0: u64) -> u64 {
    let a1510 = v0;
    let a1511 = 34u64;
    if nat_eq(a1510, a1511) {
        34u64
    } else {
        let a1512 = v0;
        let a1513 = 92u64;
        if nat_eq(a1512, a1513) {
            92u64
        } else {
            let a1514 = v0;
            let a1515 = 47u64;
            if nat_eq(a1514, a1515) {
                47u64
            } else {
                let a1516 = v0;
                let a1517 = 98u64;
                if nat_eq(a1516, a1517) {
                    8u64
                } else {
                    let a1518 = v0;
                    let a1519 = 102u64;
                    if nat_eq(a1518, a1519) {
                        12u64
                    } else {
                        let a1520 = v0;
                        let a1521 = 110u64;
                        if nat_eq(a1520, a1521) {
                            10u64
                        } else {
                            let a1522 = v0;
                            let a1523 = 114u64;
                            if nat_eq(a1522, a1523) {
                                13u64
                            } else {
                                let a1524 = v0;
                                let a1525 = 116u64;
                                if nat_eq(a1524, a1525) {
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

pub fn f475(v0: u64) -> u64 {
    let a1526 = v0;
    let a1527 = 48u64;
    if nat_lt(a1526, a1527) {
        16u64
    } else {
        let a1528 = v0;
        let a1529 = 58u64;
        if nat_lt(a1528, a1529) {
            let a1530 = v0;
            let a1531 = 48u64;
            nat_sub(a1530, a1531)
        } else {
            let a1532 = v0;
            let a1533 = 65u64;
            if nat_lt(a1532, a1533) {
                16u64
            } else {
                let a1534 = v0;
                let a1535 = 71u64;
                if nat_lt(a1534, a1535) {
                    let a1536 = v0;
                    let a1537 = 55u64;
                    nat_sub(a1536, a1537)
                } else {
                    let a1538 = v0;
                    let a1539 = 97u64;
                    if nat_lt(a1538, a1539) {
                        16u64
                    } else {
                        let a1540 = v0;
                        let a1541 = 103u64;
                        if nat_lt(a1540, a1541) {
                            let a1542 = v0;
                            let a1543 = 87u64;
                            nat_sub(a1542, a1543)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f476(v0: Adt118, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt135> {
    let a1544 = v3;
    let a1545 = 0u64;
    if nat_eq(a1544, a1545) {
        if f413(v2, 55296u64, 56319u64) {
            Ok(f421(v0, Adt119::C6(Adt120::C3(v2), v4, v5)))
        } else {
            if f413(v2, 56320u64, 57343u64) {
                Ok(f422(Adt96::C6(v1)))
            } else {
                Ok(f421(v0, Adt119::C6(Adt120::C0, f557(v2, v4)?, v5)))
            }
        }
    } else {
        if f413(v2, 56320u64, 57343u64) {
            Ok(f421(v0, Adt119::C6(Adt120::C0, f557({
                let a1554 = 65536u64;
                let a1555 = {
                    let a1552 = {
                        let a1548 = {
                            let a1546 = v3;
                            let a1547 = 55296u64;
                            nat_sub(a1546, a1547)
                        };
                        let a1549 = 1024u64;
                        nat_mul(a1548, a1549)?
                    };
                    let a1553 = {
                        let a1550 = v2;
                        let a1551 = 56320u64;
                        nat_sub(a1550, a1551)
                    };
                    nat_add(a1552, a1553)?
                };
                nat_add(a1554, a1555)?
            }, v4)?, v5)))
        } else {
            Ok(f422(Adt96::C6(v1)))
        }
    }
}

pub fn f477(v0: u64, v1: u64) -> u64 {
    let m1558 = {
        let a1556 = f478(v0);
        let a1557 = v1;
        index_list(a1556, a1557)
    };
    match m1558 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f478(v0: u64) -> List<u64> {
    let a1559 = v0;
    let a1560 = 0u64;
    if nat_eq(a1559, a1560) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1561 = v0;
        let a1562 = 1u64;
        if nat_eq(a1561, a1562) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f479(v0: Adt118, v1: Adt6) -> Adt135 {
    Adt135::C1(f415(v1, match v0.clone() {
        Adt118::C0(_, h1563, _) => {
            h1563
        }
    }, match v0 {
        Adt118::C0(_, _, h1564) => {
            h1564
        }
    }))
}

pub fn f480(v0: u64) -> Adt6 {
    let a1565 = v0;
    let a1566 = 0u64;
    if nat_eq(a1565, a1566) {
        Adt6::C1(true)
    } else {
        let a1567 = v0;
        let a1568 = 1u64;
        if nat_eq(a1567, a1568) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f481(v0: Adt118, v1: Adt121, v2: Adt122) -> Adt135 {
    f421(v0, Adt119::C8(Adt121::C0(match v1.clone() {
        Adt121::C0(h1569, _, _, _, _, _, _) => {
            h1569
        }
    }, v2, match v1.clone() {
        Adt121::C0(_, _, h1570, _, _, _, _) => {
            h1570
        }
    }, match v1.clone() {
        Adt121::C0(_, _, _, h1571, _, _, _) => {
            h1571
        }
    }, match v1.clone() {
        Adt121::C0(_, _, _, _, h1572, _, _) => {
            h1572
        }
    }, match v1.clone() {
        Adt121::C0(_, _, _, _, _, h1573, _) => {
            h1573
        }
    }, match v1 {
        Adt121::C0(_, _, _, _, _, _, h1574) => {
            h1574
        }
    })))
}

pub fn f482(v0: Adt118, v1: u64, v2: u64, v3: Adt121, v4: bool) -> R<Adt135> {
    let v5: u64 = {
        let a1576 = match v3.clone() {
            Adt121::C0(_, _, _, h1575, _, _, _) => {
                h1575
            }
        };
        let a1577 = 0u64;
        if if nat_eq(a1576, a1577) {
            let a1578 = v2;
            let a1579 = 48u64;
            nat_eq(a1578, a1579)
        } else {
            false
        } {
            0u64
        } else {
            let a1581 = match v3.clone() {
                Adt121::C0(_, _, _, h1580, _, _, _) => {
                    h1580
                }
            };
            let a1582 = 1u64;
            nat_add(a1581, a1582)?
        }
    };
    let v6: i64 = if v4 {
        f315(match v3.clone() {
            Adt121::C0(_, _, _, _, h1583, _, _) => {
                h1583
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt121::C0(_, _, _, _, h1584, _, _) => {
                h1584
            }
        }
    };
    let a1585 = f558();
    let a1586 = v5;
    if nat_lt(a1585, a1586) {
        Ok(f422(Adt96::C8(v1)))
    } else {
        let m1589 = {
            let a1587 = 99999i64;
            let a1588 = v6;
            compare(a1587, a1588)
        };
        if match m1589 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f422(Adt96::C8(v1)))
        } else {
            Ok(f421(v0, f467(match v3.clone() {
                Adt121::C0(h1590, _, _, _, _, _, _) => {
                    h1590
                }
            }, if v4 {
                Adt122::C4
            } else {
                Adt122::C2
            }, f315({
                let a1592 = match v3.clone() {
                    Adt121::C0(_, _, h1591, _, _, _, _) => {
                        h1591
                    }
                };
                let a1593 = 10i64;
                int_mul(a1592, a1593)?
            }, f468({
                let a1594 = v2;
                let a1595 = 48u64;
                nat_sub(a1594, a1595)
            }))?, v5, v6, match v3.clone() {
                Adt121::C0(_, _, _, _, _, h1596, _) => {
                    h1596
                }
            }, match v3 {
                Adt121::C0(_, _, _, _, _, _, h1597) => {
                    h1597
                }
            })))
        }
    }
}

pub fn f483(v0: u64) -> bool {
    let a1598 = v0;
    let a1599 = 101u64;
    if nat_eq(a1598, a1599) {
        true
    } else {
        let a1600 = v0;
        let a1601 = 69u64;
        nat_eq(a1600, a1601)
    }
}

pub fn f484(v0: u64) -> bool {
    f413(v0, 48u64, 57u64)
}

pub fn f485(v0: Adt118, v1: u64, v2: u64, v3: Adt121) -> R<Adt135> {
    let m1602 = f414(v3)?;
    match m1602 {
        Adt139::C0 => {
            Ok(f422(Adt96::C7(v1)))
        }
        Adt139::C1 => {
            Ok(f422(Adt96::C8(v1)))
        }
        Adt139::C2(v4) => {
            Ok(f419(f415(v4, match v0.clone() {
                Adt118::C0(_, h1603, _) => {
                    h1603
                }
            }, match v0 {
                Adt118::C0(_, _, h1604) => {
                    h1604
                }
            }), v1, v2))
        }
    }
}

pub fn f486(v0: Adt118, v1: u64, v2: u64, v3: Adt121, v4: bool) -> R<Adt135> {
    let v5: i64 = f315({
        let a1606 = match v3.clone() {
            Adt121::C0(_, _, _, _, _, _, h1605) => {
                h1605
            }
        };
        let a1607 = 10i64;
        int_mul(a1606, a1607)?
    }, f468({
        let a1608 = v2;
        let a1609 = 48u64;
        nat_sub(a1608, a1609)
    }))?;
    let m1612 = {
        let a1610 = 99999i64;
        let a1611 = v5;
        compare(a1610, a1611)
    };
    if match m1612 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f422(Adt96::C8(v1)))
    } else {
        Ok(f421(v0, f467(match v3.clone() {
            Adt121::C0(h1613, _, _, _, _, _, _) => {
                h1613
            }
        }, Adt122::C7, match v3.clone() {
            Adt121::C0(_, _, h1614, _, _, _, _) => {
                h1614
            }
        }, match v3.clone() {
            Adt121::C0(_, _, _, h1615, _, _, _) => {
                h1615
            }
        }, match v3 {
            Adt121::C0(_, _, _, _, h1616, _, _) => {
                h1616
            }
        }, v4, v5)))
    }
}

pub fn f487(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1617 = f559(f560(), v2.clone());
    match m1617 {
        None => {
            let m1618 = f561(f562(), v2.clone());
            match m1618 {
                None => {
                    f563(v0, v1, v2)
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

pub fn f488(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt124) -> R<Adt145> {
    let m1619 = f261(Str::lit("x-mcp-header"), v1.clone());
    match m1619 {
        Adt106::C0 => {
            Ok(Adt145::C1(v4))
        }
        Adt106::C1(v5) => {
            f564(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f489(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt108>) -> List<Adt108> {
    f565(Str::lit("properties"), v0.clone(), v1, v2, f565(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f566(Str::lit("items"), v0.clone(), f566(Str::lit("additionalProperties"), v0.clone(), f567(Str::lit("allOf"), v0.clone(), f567(Str::lit("anyOf"), v0.clone(), f567(Str::lit("oneOf"), v0.clone(), f566(Str::lit("not"), v0.clone(), f566(Str::lit("if"), v0.clone(), f566(Str::lit("then"), v0.clone(), f566(Str::lit("else"), v0, v3)))))))))))
}

pub fn f490(p1626: List<(Str, Adt140)>) -> Adt125 {
    let m1625 = p1626;
    {
        let mut m1625 = m1625;
        loop {
            let step = {
                let v0 = m1625;
                let m1620 = v0;
                match m1620.uncons() {
                    None => {
                        Err::<List<(Str, Adt140)>, Adt125>(Adt125::C3(List::<Adt104>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1622 = {
                            let (_, h1621) = v1;
                            h1621
                        };
                        match m1622 {
                            Adt140::C0 => {
                                Ok::<List<(Str, Adt140)>, Adt125>(v2)
                            }
                            Adt140::C1 => {
                                Err::<List<(Str, Adt140)>, Adt125>(Adt125::C0({
                                    let (h1623, _) = v1;
                                    h1623
                                }))
                            }
                            Adt140::C2 => {
                                Err::<List<(Str, Adt140)>, Adt125>(Adt125::C1({
                                    let (h1624, _) = v1;
                                    h1624
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f491(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt140> {
    let m1627 = f261(Str::lit("type"), v0);
    match m1627 {
        Adt106::C0 => {
            Ok(Adt140::C0)
        }
        Adt106::C1(v2) => {
            let m1628 = f265(v2.clone());
            match m1628 {
                None => {
                    let m1629 = f241(v2);
                    match m1629 {
                        None => {
                            Ok(Adt140::C2)
                        }
                        Some(v4) => {
                            f570(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f568(v3.clone()) {
                        if f569(v3, v1)? {
                            Ok(Adt140::C0)
                        } else {
                            Ok(Adt140::C1)
                        }
                    } else {
                        Ok(Adt140::C2)
                    }
                }
            }
        }
    }
}

pub fn f492(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt140> {
    let m1630 = f261(Str::lit("enum"), v0);
    match m1630 {
        Adt106::C0 => {
            Ok(Adt140::C0)
        }
        Adt106::C1(v2) => {
            let m1631 = f241(v2);
            match m1631 {
                None => {
                    Ok(Adt140::C2)
                }
                Some(v3) => {
                    f571(v3, v1)
                }
            }
        }
    }
}

pub fn f493(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt140> {
    let m1632 = f261(Str::lit("const"), v0);
    match m1632 {
        Adt106::C0 => {
            Ok(Adt140::C0)
        }
        Adt106::C1(v2) => {
            let m1633 = f572(v2, v1)?;
            match m1633 {
                Adt149::C0 => {
                    Ok(Adt140::C0)
                }
                Adt149::C1 => {
                    Ok(Adt140::C1)
                }
                Adt149::C2 => {
                    Ok(Adt140::C2)
                }
            }
        }
    }
}

pub fn f494(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt140> {
    let m1634 = f261(v0, v2);
    match m1634 {
        Adt106::C0 => {
            Ok(Adt140::C0)
        }
        Adt106::C1(v4) => {
            let m1635 = f573(v4);
            match m1635 {
                None => {
                    Ok(Adt140::C2)
                }
                Some(v5) => {
                    let m1636 = f573(v3);
                    match m1636 {
                        None => {
                            Ok(Adt140::C0)
                        }
                        Some(v6) => {
                            if f574(v1, f575(v6, v5)?) {
                                Ok(Adt140::C0)
                            } else {
                                Ok(Adt140::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f495(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt140)>> {
    let m1637 = v1;
    match m1637 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt140)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt140)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt140)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f576(Str::lit("minLength"), true, v0.clone(), f577(v4.clone())?)?), List::cons((Str::lit("maxLength"), f576(Str::lit("maxLength"), false, v0, f577(v4)?)?), List::<(Str, Adt140)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f576(Str::lit("minItems"), true, v0.clone(), f578(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f576(Str::lit("maxItems"), false, v0, f578(v5, 0u64)?)?), List::<(Str, Adt140)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f576(Str::lit("minProperties"), true, v0.clone(), f579(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f576(Str::lit("maxProperties"), false, v0, f579(v6, 0u64)?)?), List::<(Str, Adt140)>::nil())))
        }
    }
}

pub fn f496(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt140 {
    let m1638 = f261(Str::lit("required"), v0);
    match m1638 {
        Adt106::C0 => {
            Adt140::C0
        }
        Adt106::C1(v2) => {
            let m1639 = f241(v2);
            match m1639 {
                None => {
                    Adt140::C2
                }
                Some(v3) => {
                    let m1640 = f187(v1);
                    match m1640 {
                        None => {
                            Adt140::C0
                        }
                        Some(v4) => {
                            f580(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f497(v0: Adt125, v1: Adt125) -> Adt125 {
    let m1641 = v0.clone();
    match m1641 {
        Adt125::C0(_) => {
            v0
        }
        Adt125::C1(_) => {
            v0
        }
        Adt125::C2(_) => {
            v0
        }
        Adt125::C3(v5) => {
            let m1642 = v1.clone();
            match m1642 {
                Adt125::C0(_) => {
                    v1
                }
                Adt125::C1(_) => {
                    v1
                }
                Adt125::C2(_) => {
                    v1
                }
                Adt125::C3(v9) => {
                    Adt125::C3({
                        let a1643 = v5;
                        let a1644 = v9;
                        append_list(a1643, a1644)
                    })
                }
            }
        }
    }
}

pub fn f498(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt102>) -> Adt125 {
    let m1645 = f261(Str::lit("$ref"), v1);
    match m1645 {
        Adt106::C0 => {
            Adt125::C3(List::<Adt104>::nil())
        }
        Adt106::C1(v4) => {
            let m1646 = f265(v4);
            match m1646 {
                None => {
                    Adt125::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1647 = f581(v0, v5.clone());
                    match m1647 {
                        None => {
                            Adt125::C2(v5)
                        }
                        Some(v6) => {
                            Adt125::C3(List::cons(Adt104::C0(v6, v2, v3), List::<Adt104>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f499(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt102>) -> Adt125 {
    let m1648 = f261(Str::lit("properties"), v0);
    match m1648 {
        Adt106::C0 => {
            Adt125::C3(List::<Adt104>::nil())
        }
        Adt106::C1(v3) => {
            let m1649 = f187(v3);
            match m1649 {
                None => {
                    Adt125::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1650 = f187(v1);
                    match m1650 {
                        None => {
                            Adt125::C3(List::<Adt104>::nil())
                        }
                        Some(v5) => {
                            Adt125::C3(f582(match v4 {
                                Adt5::C0(h1651) => {
                                    h1651
                                }
                            }, match v5 {
                                Adt5::C0(h1652) => {
                                    h1652
                                }
                            }, v2, List::<Adt104>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f500(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt102>) -> Adt125 {
    let m1653 = f261(Str::lit("additionalProperties"), v0.clone());
    match m1653 {
        Adt106::C0 => {
            Adt125::C3(List::<Adt104>::nil())
        }
        Adt106::C1(v3) => {
            let m1654 = f187(v1);
            match m1654 {
                None => {
                    Adt125::C3(List::<Adt104>::nil())
                }
                Some(v4) => {
                    let m1655 = f316(v3.clone());
                    match m1655 {
                        None => {
                            Adt125::C3(f585(match v4 {
                                Adt5::C0(h1657) => {
                                    h1657
                                }
                            }, f584(v0), v3, v2, List::<Adt104>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt125::C3(List::<Adt104>::nil())
                            } else {
                                if f583(match v4 {
                                    Adt5::C0(h1656) => {
                                        h1656
                                    }
                                }, f584(v0)) {
                                    Adt125::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt125::C3(List::<Adt104>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f501(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt102>) -> R<Adt125> {
    let m1658 = f261(Str::lit("items"), v0);
    match m1658 {
        Adt106::C0 => {
            Ok(Adt125::C3(List::<Adt104>::nil()))
        }
        Adt106::C1(v3) => {
            let m1659 = f241(v1);
            match m1659 {
                None => {
                    Ok(Adt125::C3(List::<Adt104>::nil()))
                }
                Some(v4) => {
                    Ok(Adt125::C3(f586(v4, 0u64, v3, v2, List::<Adt104>::nil())?))
                }
            }
        }
    }
}

pub fn f502(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt102>) -> Adt125 {
    let m1660 = f261(Str::lit("allOf"), v0);
    match m1660 {
        Adt106::C0 => {
            Adt125::C3(List::<Adt104>::nil())
        }
        Adt106::C1(v3) => {
            let m1661 = f241(v3);
            match m1661 {
                None => {
                    Adt125::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1662 = v4.clone();
                    match m1662.uncons() {
                        None => {
                            Adt125::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt125::C3(f587(v4, v1, v2, List::<Adt104>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f503(v0: Adt105, v1: Adt125, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt102>) -> Adt125 {
    let m1663 = v1.clone();
    match m1663 {
        Adt125::C0(_) => {
            v1
        }
        Adt125::C1(_) => {
            v1
        }
        Adt125::C2(_) => {
            v1
        }
        Adt125::C3(v8) => {
            let m1664 = v8;
            match m1664.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt125::C3(List::cons(Adt104::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt104>::nil()))
                }
            }
        }
    }
}

pub fn f504(p1671: List<u64>, p1672: List<u64>) -> List<u64> {
    let m1670 = (p1671, p1672);
    {
        let mut m1670 = m1670;
        loop {
            let step = {
                let (v0, v1) = m1670;
                let m1665 = v0;
                match m1665.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1666 = v2;
                        let a1667 = 126u64;
                        if nat_eq(a1666, a1667) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1668 = v2;
                            let a1669 = 47u64;
                            if nat_eq(a1668, a1669) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
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

pub fn f505(p1681: u64, p1682: List<u64>, p1683: List<u64>) -> R<Option<List<u64>>> {
    let m1680 = (p1681, (p1682, p1683));
    {
        let mut m1680 = m1680;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1680;
                let m1673 = v0;
                if m1673 == 0 {
                    let m1674 = v1;
                    match m1674.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1673 - 1;
                    let m1675 = v1;
                    match m1675.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1676 = v7;
                            match m1676.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1677 = v9;
                                    match m1677.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1678 = v11;
                                            match m1678.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1679 = f588(v6, v8, v10, v12, f589(v13.clone()))?;
                                                    match m1679 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f332(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(p1688: List<Adt50>, p1689: Adt7) -> R<bool> {
    let m1687 = (p1688, p1689);
    {
        let mut m1687 = m1687;
        loop {
            let step = {
                let (v0, v1) = m1687;
                let m1684 = v0;
                match m1684.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f590(f575(match v2.clone() {
                            Adt50::C0(h1685, _, _) => {
                                h1685
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1686, _, _) => {
                                    h1686
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f508(v0: u64) -> List<u64> {
    f478(v0)
}

pub fn f509(v0: Adt7) -> R<bool> {
    let a1707 = {
        let m1693 = {
            let a1691 = f591();
            let a1692 = match v0.clone() {
                Adt7::C0(h1690, _) => {
                    h1690
                }
            };
            compare(a1691, a1692)
        };
        if match m1693 {
            Ordering::Less => {
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
            let m1698 = {
                let a1696 = match v0.clone() {
                    Adt7::C0(h1694, _) => {
                        h1694
                    }
                };
                let a1697 = {
                    let a1695 = f591();
                    int_neg(a1695)?
                };
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
                true
            } else {
                let m1702 = {
                    let a1700 = 9999i64;
                    let a1701 = match v0.clone() {
                        Adt7::C0(_, h1699) => {
                            h1699
                        }
                    };
                    compare(a1700, a1701)
                };
                if match m1702 {
                    Ordering::Less => {
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
                    let m1706 = {
                        let a1704 = match v0 {
                            Adt7::C0(_, h1703) => {
                                h1703
                            }
                        };
                        let a1705 = -9999i64;
                        compare(a1704, a1705)
                    };
                    match m1706 {
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
    Ok(bool_not(a1707))
}

pub fn f510(v0: Adt7) -> R<List<u64>> {
    let m1711 = {
        let a1709 = match v0.clone() {
            Adt7::C0(_, h1708) => {
                h1708
            }
        };
        let a1710 = 0i64;
        compare(a1709, a1710)
    };
    if if match m1711 {
        Ordering::Less => {
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
        let m1715 = {
            let a1713 = 0i64;
            let a1714 = match v0.clone() {
                Adt7::C0(_, h1712) => {
                    h1712
                }
            };
            compare(a1713, a1714)
        };
        match m1715 {
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
        let a1718 = f592(match v0.clone() {
            Adt7::C0(h1716, _) => {
                h1716
            }
        })?;
        let a1719 = List::cons(101u64, f592(match v0 {
            Adt7::C0(_, h1717) => {
                h1717
            }
        })?);
        Ok(append_list(a1718, a1719))
    } else {
        f592(match v0 {
            Adt7::C0(h1720, _) => {
                h1720
            }
        })
    }
}

pub fn f511() -> u64 {
    512u64
}

pub fn f512(v0: u64, v1: Adt110, v2: List<Adt110>) -> List<Adt110> {
    List::cons(Adt110::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f513(p1723: List<u64>, p1724: List<u64>) -> R<List<u64>> {
    let m1722 = (p1723, p1724);
    {
        let mut m1722 = m1722;
        loop {
            let step = {
                let (v0, v1) = m1722;
                let m1721 = v0;
                match m1721.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f332(f593(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1722 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f594(match v0.clone() {
        Adt33::C0(_, h1725, _, _) => {
            h1725
        }
    })), List::cons((Str::lit("capabilities"), f595(match v0.clone() {
        Adt33::C0(_, _, h1726, _) => {
            h1726
        }
    })), {
        let a1729 = f596(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1727) => {
                h1727
            }
        });
        let a1730 = f597(match v0 {
            Adt33::C0(h1728, _, _, _) => {
                h1728
            }
        })?;
        append_list(a1729, a1730)
    })))
}

pub fn f515(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f598(match v0.clone() {
        Adt34::C0(_, _, h1731) => {
            h1731
        }
    }))), {
        let a1734 = f596(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1732, _) => {
                h1732
            }
        });
        let a1735 = f597(match v0 {
            Adt34::C0(h1733, _, _) => {
                h1733
            }
        })?;
        append_list(a1734, a1735)
    }))
}

pub fn f516(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1736 = v0;
    match m1736 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1737 = f530(Adt6::C5(v1))?;
            match m1737 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1738 = f187(v2);
                    match m1738 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1739) => {
                                    h1739
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f517(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1741 = f599(match v0.clone() {
        Adt37::C0(h1740, _, _) => {
            h1740
        }
    })?;
    match m1741 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1744 = f600(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1742, _) => {
                        h1742
                    }
                });
                let a1745 = f601(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1743) => {
                        h1743
                    }
                });
                append_list(a1744, a1745)
            })))
        }
    }
}

pub fn f518(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f602(match v0.clone() {
        Adt51::C0(_, _, h1746) => {
            h1746
        }
    })?)), {
        let a1749 = f596(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1747, _) => {
                h1747
            }
        });
        let a1750 = f597(match v0 {
            Adt51::C0(h1748, _, _) => {
                h1748
            }
        })?;
        append_list(a1749, a1750)
    }))
}

pub fn f519(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f603(match v0.clone() {
        Adt52::C0(_, h1751) => {
            h1751
        }
    }))), f597(match v0 {
        Adt52::C0(h1752, _) => {
            h1752
        }
    })?))
}

pub fn f520(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f604(match v0.clone() {
        Adt53::C0(_, _, h1753) => {
            h1753
        }
    }))), {
        let a1756 = f596(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1754, _) => {
                h1754
            }
        });
        let a1757 = f597(match v0 {
            Adt53::C0(h1755, _, _) => {
                h1755
            }
        })?;
        append_list(a1756, a1757)
    }))
}

pub fn f521(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f605(match v0.clone() {
        Adt55::C0(_, _, h1758) => {
            h1758
        }
    }))), {
        let a1761 = f596(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1759, _) => {
                h1759
            }
        });
        let a1762 = f597(match v0 {
            Adt55::C0(h1760, _, _) => {
                h1760
            }
        })?;
        append_list(a1761, a1762)
    }))
}

pub fn f522(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1764 = f606(match v0.clone() {
        Adt58::C0(_, h1763) => {
            h1763
        }
    })?;
    match m1764 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1766 = f596(Str::lit("description"), match v0 {
                    Adt58::C0(h1765, _) => {
                        h1765
                    }
                });
                let a1767 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1766, a1767)
            }))
        }
    }
}

pub fn f523(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f540({
        let m1770 = match v0.clone() {
            Adt60::C0(h1769) => {
                h1769
            }
        };
        match m1770 {
            Adt61::C0(h1768, _, _) => {
                h1768
            }
        }
    }))), {
        let a1777 = f607(Str::lit("total"), {
            let m1773 = match v0.clone() {
                Adt60::C0(h1772) => {
                    h1772
                }
            };
            match m1773 {
                Adt61::C0(_, h1771, _) => {
                    h1771
                }
            }
        })?;
        let a1778 = f601(Str::lit("hasMore"), {
            let m1776 = match v0 {
                Adt60::C0(h1775) => {
                    h1775
                }
            };
            match m1776 {
                Adt61::C0(_, _, h1774) => {
                    h1774
                }
            }
        });
        append_list(a1777, a1778)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f524(v0: Adt81) -> R<Option<List<(Str, Adt6)>>> {
    let m1779 = f608(f609(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m1779 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1780 = f610(v1);
                let a1781 = f611(f612(v0));
                append_list(a1780, a1781)
            }))
        }
    }
}

pub fn f525(v0: Adt146) -> Str {
    let m1782 = v0;
    match m1782 {
        Adt146::C0 => {
            Str::lit("complete")
        }
        Adt146::C1 => {
            Str::lit("input_required")
        }
        Adt146::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f526(v0: Adt32) -> Adt146 {
    let m1783 = v0;
    match m1783 {
        Adt32::C0(_) => {
            Adt146::C0
        }
        Adt32::C1(_) => {
            Adt146::C0
        }
        Adt32::C2(_) => {
            Adt146::C0
        }
        Adt32::C3(_) => {
            Adt146::C0
        }
        Adt32::C4(_) => {
            Adt146::C0
        }
        Adt32::C5(_) => {
            Adt146::C0
        }
        Adt32::C6(_) => {
            Adt146::C0
        }
        Adt32::C7(_) => {
            Adt146::C0
        }
        Adt32::C8(_) => {
            Adt146::C0
        }
        Adt32::C9(_) => {
            Adt146::C0
        }
        Adt32::C10(_) => {
            Adt146::C0
        }
        Adt32::C11 => {
            Adt146::C0
        }
        Adt32::C12(_) => {
            Adt146::C1
        }
        Adt32::C13(_) => {
            Adt146::C2
        }
    }
}

pub fn f527(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1784 = v0;
    match m1784 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f613(), f614(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f528(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f615(v2) {
            let a1785 = v1;
            let a1786 = 200u64;
            nat_eq(a1785, a1786)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f529(v0: u64, v1: List<List<u64>>) -> Adt88 {
    Adt88::C0(v0, f616(), List::<List<u64>>::nil(), f617(v1))
}

pub fn f530(v0: Adt6) -> R<Option<Adt6>> {
    f618(f619(), Adt147::C0(v0), List::<Adt148>::nil())
}

pub fn f531(v0: Adt130) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f278())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f620(match v0.clone() {
        Adt130::C0(h1787, _) => {
            h1787
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1790 = match v0.clone() {
            Adt130::C0(_, h1789) => {
                h1789
            }
        };
        match m1790 {
            Adt50::C0(h1788, _, _) => {
                h1788
            }
        }
    })), {
        let a1797 = f621({
            let m1793 = match v0.clone() {
                Adt130::C0(_, h1792) => {
                    h1792
                }
            };
            match m1793 {
                Adt50::C0(_, h1791, _) => {
                    h1791
                }
            }
        });
        let a1798 = f622({
            let m1796 = match v0 {
                Adt130::C0(_, h1795) => {
                    h1795
                }
            };
            match m1796 {
                Adt50::C0(_, _, h1794) => {
                    h1794
                }
            }
        });
        append_list(a1797, a1798)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f532() -> u64 {
    429u64
}

pub fn f533() -> i64 {
    -32602i64
}

pub fn f534() -> i64 {
    -32022i64
}

pub fn f535() -> i64 {
    -32020i64
}

pub fn f536() -> i64 {
    -32601i64
}

pub fn f537(v0: Adt84) -> i64 {
    let m1799 = v0;
    match m1799 {
        Adt84::C0(_) => {
            f533()
        }
        Adt84::C1(_) => {
            f533()
        }
        Adt84::C2(_) => {
            f533()
        }
        Adt84::C3(_) => {
            f533()
        }
        Adt84::C4(_) => {
            f533()
        }
        Adt84::C5(_) => {
            f533()
        }
        Adt84::C6(_) => {
            f623()
        }
        Adt84::C7(_) => {
            f624()
        }
        Adt84::C8(_) => {
            f625()
        }
        Adt84::C9(_) => {
            f623()
        }
        Adt84::C10 => {
            f533()
        }
        Adt84::C11 => {
            f533()
        }
        Adt84::C12(_) => {
            f623()
        }
        Adt84::C13 => {
            f623()
        }
        Adt84::C14 => {
            f623()
        }
    }
}

pub fn f538(v0: Adt132) -> Str {
    let m1800 = v0;
    match m1800 {
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

pub fn f539(v0: Adt112) -> Str {
    let m1801 = v0;
    match m1801 {
        Adt112::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt112::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt112::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt112::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt112::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt112::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt112::C6(v1) => {
            let m1802 = v1;
            match m1802 {
                Adt113::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt113::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt113::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt112::C7(v2) => {
            let m1803 = v2;
            match m1803 {
                Adt113::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt113::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt113::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt112::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt112::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt112::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f540(v0: List<Str>) -> List<Adt6> {
    f469(f626(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f541(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1815 = f627(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1804, _, _, _, _) => {
                h1804
            }
        });
        let a1816 = {
            let a1813 = f627(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1805, _, _, _) => {
                    h1805
                }
            });
            let a1814 = {
                let a1811 = f627(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1806, _, _) => {
                        h1806
                    }
                });
                let a1812 = {
                    let a1809 = f627(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1807, _) => {
                            h1807
                        }
                    });
                    let a1810 = f627(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1808) => {
                            h1808
                        }
                    });
                    append_list(a1809, a1810)
                };
                append_list(a1811, a1812)
            };
            append_list(a1813, a1814)
        };
        append_list(a1815, a1816)
    })
}

pub fn f542(p1819: List<List<u64>>) -> R<bool> {
    let m1818 = p1819;
    {
        let mut m1818 = m1818;
        loop {
            let step = {
                let v0 = m1818;
                let m1817 = v0;
                match m1817.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f628(f293(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
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

pub fn f543() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f544(v0: Adt0, v1: Fn0, v2: u64, v3: Adt137, v4: bool) -> R<Result<Adt128, Adt131>> {
    if v4 {
        f629(v0, v1, v2, f630(match v3.clone() {
            Adt137::C0(_, h1820, _, _) => {
                h1820
            }
        }, match v3.clone() {
            Adt137::C0(_, _, h1821, _) => {
                h1821
            }
        }, match v3 {
            Adt137::C0(_, _, _, h1822) => {
                h1822
            }
        }))
    } else {
        Ok(Err::<Adt128, Adt131>(Adt131::C3(f631(match v3 {
            Adt137::C0(_, h1823, _, _) => {
                h1823
            }
        }))))
    }
}

pub fn f545(v0: Adt4, v1: Adt138) -> bool {
    let m1824 = v1;
    match m1824 {
        Adt138::C0 => {
            true
        }
        Adt138::C1 => {
            f632(match v0 {
                Adt4::C0(_, _, _, _, _, h1825, _) => {
                    h1825
                }
            })
        }
        Adt138::C2 => {
            f632(match v0 {
                Adt4::C0(_, _, _, _, _, h1826, _) => {
                    h1826
                }
            })
        }
        Adt138::C3 => {
            f633(match v0 {
                Adt4::C0(_, _, _, _, h1827, _, _) => {
                    h1827
                }
            })
        }
        Adt138::C4 => {
            f633(match v0 {
                Adt4::C0(_, _, _, _, h1828, _, _) => {
                    h1828
                }
            })
        }
        Adt138::C5 => {
            f633(match v0 {
                Adt4::C0(_, _, _, _, h1829, _, _) => {
                    h1829
                }
            })
        }
        Adt138::C6 => {
            f632(match v0 {
                Adt4::C0(_, _, _, h1830, _, _, _) => {
                    h1830
                }
            })
        }
        Adt138::C7 => {
            f632(match v0 {
                Adt4::C0(_, _, _, h1831, _, _, _) => {
                    h1831
                }
            })
        }
        Adt138::C8 => {
            f249(match v0 {
                Adt4::C0(_, _, h1832, _, _, _, _) => {
                    h1832
                }
            })
        }
        Adt138::C9 => {
            f634(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1833) => {
                    h1833
                }
            }, f635())
        }
        Adt138::C10 => {
            f634(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1834) => {
                    h1834
                }
            }, f635())
        }
        Adt138::C11 => {
            f634(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1835) => {
                    h1835
                }
            }, f635())
        }
    }
}

pub fn f546(v0: Adt143, v1: Adt141) -> Result<Adt137, Adt131> {
    let m1836 = v1;
    match m1836 {
        Adt141::C0 => {
            Err::<Adt137, Adt131>(Adt131::C3(match v0 {
                Adt143::C0(_, h1837, _, _) => {
                    h1837
                }
            }))
        }
        Adt141::C1(v2) => {
            Ok::<Adt137, Adt131>(Adt137::C0(match v0.clone() {
                Adt143::C0(h1838, _, _, _) => {
                    h1838
                }
            }, v2, match v0.clone() {
                Adt143::C0(_, _, h1839, _) => {
                    h1839
                }
            }, match v0 {
                Adt143::C0(_, _, _, h1840) => {
                    h1840
                }
            }))
        }
    }
}

pub fn f547(v0: Option<Adt111>, v1: List<Str>, v2: Adt107, v3: Result<Adt16, Adt132>) -> R<Adt142> {
    let m1841 = v3;
    match m1841 {
        Ok(v5) => {
            Ok(f636(v1, v2.clone(), v5.clone(), f637(v0, f457(match v2.clone() {
                Adt107::C0(_, h1842, _) => {
                    h1842
                }
            }), match v2.clone() {
                Adt107::C0(_, h1843, _) => {
                    h1843
                }
            }, match v5 {
                Adt16::C0(h1844, _, _, _, _, _) => {
                    h1844
                }
            }, f638(match v2 {
                Adt107::C0(_, _, h1845) => {
                    h1845
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt142::C1(Adt131::C0(v4)))
        }
    }
}

pub fn f548(v0: Option<Adt5>) -> Result<Adt16, Adt132> {
    let m1846 = v0;
    match m1846 {
        None => {
            Err::<Adt16, Adt132>(Adt132::C0)
        }
        Some(v1) => {
            let m1847 = f186(Str::lit("_meta"), v1);
            match m1847 {
                Adt106::C0 => {
                    Err::<Adt16, Adt132>(Adt132::C1)
                }
                Adt106::C1(v2) => {
                    let m1848 = f187(v2);
                    match m1848 {
                        None => {
                            Err::<Adt16, Adt132>(Adt132::C4)
                        }
                        Some(v3) => {
                            f639(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f549(v0: Adt138) -> Adt144 {
    let m1849 = v0;
    match m1849 {
        Adt138::C0 => {
            Adt144::C0
        }
        Adt138::C1 => {
            Adt144::C0
        }
        Adt138::C2 => {
            Adt144::C1
        }
        Adt138::C3 => {
            Adt144::C0
        }
        Adt138::C4 => {
            Adt144::C2
        }
        Adt138::C5 => {
            Adt144::C0
        }
        Adt138::C6 => {
            Adt144::C0
        }
        Adt138::C7 => {
            Adt144::C1
        }
        Adt138::C8 => {
            Adt144::C0
        }
        Adt138::C9 => {
            Adt144::C3
        }
        Adt138::C10 => {
            Adt144::C3
        }
        Adt138::C11 => {
            Adt144::C3
        }
    }
}

pub fn f550(p1855: List<u64>, p1856: List<u64>) -> Option<List<u64>> {
    let m1854 = (p1855, p1856);
    {
        let mut m1854 = m1854;
        loop {
            let step = {
                let (v0, v1) = m1854;
                let m1850 = v0;
                match m1850.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1851 = v1;
                        match m1851.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1852 = v4;
                                let a1853 = v2;
                                if nat_eq(a1852, a1853) {
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
                Ok(next) => { m1854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f552() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f553(v0: Adt12, v1: Option<Str>) -> bool {
    let m1857 = v1;
    match m1857 {
        None => {
            false
        }
        Some(v2) => {
            let a1859 = match v0 {
                Adt12::C0(h1858, _, _, _) => {
                    h1858
                }
            };
            let a1860 = v2;
            equal(a1859, a1860)
        }
    }
}

pub fn f554(v0: List<Adt86>, v1: Str) -> R<Adt115> {
    let m1863 = f403(f134(v0, {
        let a1861 = f640();
        let a1862 = f641(v1)?;
        append_list(a1861, a1862)
    })?)?;
    match m1863 {
        Adt99::C0 => {
            Ok(Adt115::C0)
        }
        Adt99::C1 => {
            Ok(Adt115::C1)
        }
        Adt99::C2 => {
            Ok(Adt115::C1)
        }
        Adt99::C3(v2) => {
            let m1864 = f456(v2);
            match m1864 {
                None => {
                    Ok(Adt115::C1)
                }
                Some(v3) => {
                    Ok(Adt115::C2(v3))
                }
            }
        }
    }
}

pub fn f555(v0: i64) -> bool {
    let a1871 = {
        let m1867 = {
            let a1865 = v0;
            let a1866 = 0i64;
            compare(a1865, a1866)
        };
        if match m1867 {
            Ordering::Less => {
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
            let m1870 = {
                let a1868 = 0i64;
                let a1869 = v0;
                compare(a1868, a1869)
            };
            match m1870 {
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
    bool_not(a1871)
}

pub fn f556(v0: Adt118, v1: u64, v2: Str) -> Adt135 {
    let m1873 = match v0.clone() {
        Adt118::C0(_, h1872, _) => {
            h1872
        }
    };
    match m1873.uncons() {
        None => {
            f422(Adt96::C2(v1))
        }
        Some((v3, v4)) => {
            let m1874 = v3;
            match m1874 {
                Adt123::C0(_) => {
                    f422(Adt96::C2(v1))
                }
                Adt123::C1(v6, _) => {
                    Adt135::C1(Adt118::C0(Adt119::C5, List::cons(Adt123::C1(v6, Some(v2)), v4), match v0 {
                        Adt118::C0(_, _, h1875) => {
                            h1875
                        }
                    }))
                }
            }
        }
    }
}

pub fn f557(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1876 = v0;
    let a1877 = 128u64;
    if nat_lt(a1876, a1877) {
        Ok(List::cons(v0, v1))
    } else {
        let a1878 = v0;
        let a1879 = 2048u64;
        if nat_lt(a1878, a1879) {
            Ok(List::cons({
                let a1880 = 128u64;
                let a1881 = f642(v0, 64u64);
                nat_add(a1880, a1881)?
            }, List::cons({
                let a1882 = 192u64;
                let a1883 = f643(v0, 64u64);
                nat_add(a1882, a1883)?
            }, v1)))
        } else {
            let a1884 = v0;
            let a1885 = 65536u64;
            if nat_lt(a1884, a1885) {
                Ok(List::cons({
                    let a1886 = 128u64;
                    let a1887 = f642(v0, 64u64);
                    nat_add(a1886, a1887)?
                }, List::cons({
                    let a1888 = 128u64;
                    let a1889 = f642(f643(v0, 64u64), 64u64);
                    nat_add(a1888, a1889)?
                }, List::cons({
                    let a1890 = 224u64;
                    let a1891 = f643(v0, 4096u64);
                    nat_add(a1890, a1891)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1892 = 128u64;
                    let a1893 = f642(v0, 64u64);
                    nat_add(a1892, a1893)?
                }, List::cons({
                    let a1894 = 128u64;
                    let a1895 = f642(f643(v0, 64u64), 64u64);
                    nat_add(a1894, a1895)?
                }, List::cons({
                    let a1896 = 128u64;
                    let a1897 = f642(f643(v0, 4096u64), 64u64);
                    nat_add(a1896, a1897)?
                }, List::cons({
                    let a1898 = 240u64;
                    let a1899 = f643(v0, 262144u64);
                    nat_add(a1898, a1899)?
                }, v1)))))
            }
        }
    }
}

pub fn f558() -> u64 {
    18u64
}

pub fn f559(p1903: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1902 = p1903;
    {
        let mut m1902 = m1902;
        loop {
            let step = {
                let v0 = m1902;
                let m1900 = v0;
                match m1900.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1901 = f261(v2.clone(), v1.clone());
                        match m1901 {
                            Adt106::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt106::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1902 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f560() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f561(p1910: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1909 = p1910;
    {
        let mut m1909 = m1909;
        loop {
            let step = {
                let v0 = m1909;
                let m1904 = v0;
                match m1904.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1906 = f261({
                            let (h1905, _) = v2.clone();
                            h1905
                        }, v1.clone());
                        match m1906 {
                            Adt106::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt106::C1(v4) => {
                                if f644({
                                    let (_, h1907) = v2;
                                    h1907
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1908, _) = v2;
                                        h1908
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f562() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f563(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1911 = f261(Str::lit("$ref"), v2);
    match m1911 {
        Adt106::C0 => {
            None::<Adt91>
        }
        Adt106::C1(v3) => {
            let m1912 = f265(v3);
            match m1912 {
                None => {
                    None::<Adt91>
                }
                Some(v4) => {
                    let m1913 = f581(v1, v4.clone());
                    match m1913 {
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

pub fn f564(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt124) -> R<Adt145> {
    if v3 {
        let m1914 = f265(v2);
        match m1914 {
            None => {
                Ok(Adt145::C0(Adt91::C7(v0)))
            }
            Some(v6) => {
                f645(v0, v1, v6.clone(), f232(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt145::C0(Adt91::C10(v0)))
    }
}

pub fn f565(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt108>) -> List<Adt108> {
    let m1915 = f261(v0.clone(), v1);
    match m1915 {
        Adt106::C0 => {
            v4
        }
        Adt106::C1(v5) => {
            let m1916 = f187(v5);
            match m1916 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1917 = v0;
                    let a1918 = Str::lit("properties");
                    if equal(a1917, a1918) {
                        f646(match v6 {
                            Adt5::C0(h1919) => {
                                h1919
                            }
                        }, v2, v3, v4)
                    } else {
                        f647(match v6 {
                            Adt5::C0(h1920) => {
                                h1920
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f566(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt108>) -> List<Adt108> {
    let m1921 = f261(v0, v1);
    match m1921 {
        Adt106::C0 => {
            v2
        }
        Adt106::C1(v3) => {
            List::cons(Adt108::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f567(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt108>) -> List<Adt108> {
    let m1922 = f261(v0, v1);
    match m1922 {
        Adt106::C0 => {
            v2
        }
        Adt106::C1(v3) => {
            let m1923 = f241(v3);
            match m1923 {
                None => {
                    v2
                }
                Some(v4) => {
                    f648(v4, v2)
                }
            }
        }
    }
}

pub fn f568(v0: Str) -> bool {
    let a1924 = v0.clone();
    let a1925 = Str::lit("null");
    if equal(a1924, a1925) {
        true
    } else {
        let a1926 = v0.clone();
        let a1927 = Str::lit("boolean");
        if equal(a1926, a1927) {
            true
        } else {
            let a1928 = v0.clone();
            let a1929 = Str::lit("object");
            if equal(a1928, a1929) {
                true
            } else {
                let a1930 = v0.clone();
                let a1931 = Str::lit("array");
                if equal(a1930, a1931) {
                    true
                } else {
                    let a1932 = v0.clone();
                    let a1933 = Str::lit("number");
                    if equal(a1932, a1933) {
                        true
                    } else {
                        let a1934 = v0.clone();
                        let a1935 = Str::lit("string");
                        if equal(a1934, a1935) {
                            true
                        } else {
                            let a1936 = v0;
                            let a1937 = Str::lit("integer");
                            equal(a1936, a1937)
                        }
                    }
                }
            }
        }
    }
}

pub fn f569(v0: Str, v1: Adt6) -> R<bool> {
    let a1938 = v0.clone();
    let a1939 = f649(v1.clone());
    if equal(a1938, a1939) {
        Ok(true)
    } else {
        let a1940 = v0;
        let a1941 = Str::lit("integer");
        if equal(a1940, a1941) {
            let m1942 = f573(v1);
            match m1942 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f650(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f570(p1945: List<Adt6>, v1: Adt6, p1946: bool) -> R<Adt140> {
    let m1944 = (p1945, p1946);
    {
        let mut m1944 = m1944;
        loop {
            let step = {
                let (v0, v2) = m1944;
                let m1943 = v0;
                match m1943.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt140>>(Ok(Adt140::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt140>>(Ok(Adt140::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f568(f651(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt140>>((v4, if v2 {
                                true
                            } else {
                                f569(f651(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt140>>(Ok(Adt140::C2))
                        }
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

pub fn f571(p1950: List<Adt6>, v1: Adt6) -> R<Adt140> {
    let m1949 = p1950;
    {
        let mut m1949 = m1949;
        loop {
            let step = {
                let v0 = m1949;
                let m1947 = v0;
                match m1947.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt140>>(Ok(Adt140::C1))
                    }
                    Some((v2, v3)) => {
                        let m1948 = f572(v2, v1.clone())?;
                        match m1948 {
                            Adt149::C0 => {
                                Err::<List<Adt6>, R<Adt140>>(Ok(Adt140::C0))
                            }
                            Adt149::C1 => {
                                Ok::<List<Adt6>, R<Adt140>>(v3)
                            }
                            Adt149::C2 => {
                                Err::<List<Adt6>, R<Adt140>>(Ok(Adt140::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1949 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: Adt6, v1: Adt6) -> R<Adt149> {
    f652(f653(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f573(v0: Adt6) -> Option<Adt7> {
    let m1951 = v0;
    match m1951 {
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

pub fn f574(v0: u64, v1: Adt150) -> bool {
    let a1952 = v0;
    let a1953 = 0u64;
    if nat_eq(a1952, a1953) {
        let a1954 = f654(v1);
        bool_not(a1954)
    } else {
        let a1955 = v0;
        let a1956 = 1u64;
        if nat_eq(a1955, a1956) {
            let a1957 = f590(v1);
            bool_not(a1957)
        } else {
            let a1958 = v0;
            let a1959 = 2u64;
            if nat_eq(a1958, a1959) {
                f590(v1)
            } else {
                f654(v1)
            }
        }
    }
}

pub fn f575(v0: Adt7, v1: Adt7) -> R<Adt150> {
    f655(v0.clone(), v1.clone(), f656(match v0 {
        Adt7::C0(h1960, _) => {
            h1960
        }
    }), f656(match v1 {
        Adt7::C0(h1961, _) => {
            h1961
        }
    }))
}

pub fn f576(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt140> {
    let m1962 = f261(v0, v2);
    match m1962 {
        Adt106::C0 => {
            Ok(Adt140::C0)
        }
        Adt106::C1(v4) => {
            let m1963 = f657(v4);
            match m1963 {
                None => {
                    Ok(Adt140::C2)
                }
                Some(v5) => {
                    let m1966 = {
                        let a1964 = v5;
                        let a1965 = 0i64;
                        compare(a1964, a1965)
                    };
                    if match m1966 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt140::C2)
                    } else {
                        if v1 {
                            if f654(f658(f240(v3, 0i64)?, v5)) {
                                Ok(Adt140::C1)
                            } else {
                                Ok(Adt140::C0)
                            }
                        } else {
                            if f590(f658(f240(v3, 0i64)?, v5)) {
                                Ok(Adt140::C1)
                            } else {
                                Ok(Adt140::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f577(v0: Str) -> R<u64> {
    f659(f206({
        let a1967 = v0;
        utf8_encode(a1967)
    })?, 0u64)
}

pub fn f578(p1972: List<Adt6>, p1973: u64) -> R<u64> {
    let m1971 = (p1972, p1973);
    {
        let mut m1971 = m1971;
        loop {
            let step = {
                let (v0, v1) = m1971;
                let m1968 = v0;
                match m1968.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1969 = v1;
                            let a1970 = 1u64;
                            nat_add(a1969, a1970)?
                        }))
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

pub fn f579(p1978: List<(Str, Adt6)>, p1979: u64) -> R<u64> {
    let m1977 = (p1978, p1979);
    {
        let mut m1977 = m1977;
        loop {
            let step = {
                let (v0, v1) = m1977;
                let m1974 = v0;
                match m1974.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1975 = v1;
                            let a1976 = 1u64;
                            nat_add(a1975, a1976)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1977 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(p1983: List<Adt6>, v1: Adt5) -> Adt140 {
    let m1982 = p1983;
    {
        let mut m1982 = m1982;
        loop {
            let step = {
                let v0 = m1982;
                let m1980 = v0;
                match m1980.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt140>(Adt140::C0)
                    }
                    Some((v2, v3)) => {
                        let m1981 = f265(v2);
                        match m1981 {
                            None => {
                                Err::<List<Adt6>, Adt140>(Adt140::C2)
                            }
                            Some(v4) => {
                                if f410(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt140>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt140>(Adt140::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1982 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1984 = v1.clone();
    let a1985 = Str::lit("#");
    if equal(a1984, a1985) {
        Some(v0)
    } else {
        let m1986 = f187(v0);
        match m1986 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1987 = f186(Str::lit("$defs"), v2);
                match m1987 {
                    Adt106::C0 => {
                        None::<Adt6>
                    }
                    Adt106::C1(v3) => {
                        let m1988 = f187(v3);
                        match m1988 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f660(v1, match v4 {
                                    Adt5::C0(h1989) => {
                                        h1989
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

pub fn f582(p1996: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt102>, p1997: List<Adt104>) -> List<Adt104> {
    let m1995 = (p1996, p1997);
    {
        let mut m1995 = m1995;
        loop {
            let step = {
                let (v0, v3) = m1995;
                let m1990 = v0;
                match m1990.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1992 = f261({
                            let (h1991, _) = v4.clone();
                            h1991
                        }, v1.clone());
                        match m1992 {
                            Adt106::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v5, v3))
                            }
                            Adt106::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v5, List::cons(Adt104::C0({
                                    let (_, h1993) = v4;
                                    h1993
                                }, v6, List::cons(Adt102::C0({
                                    let (h1994, _) = v4;
                                    h1994
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(p2002: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2001 = p2002;
    {
        let mut m2001 = m2001;
        loop {
            let step = {
                let v0 = m2001;
                let m1998 = v0;
                match m1998.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2000 = f261({
                            let (h1999, _) = v2;
                            h1999
                        }, v1.clone());
                        match m2000 {
                            Adt106::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt106::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2003 = f261(Str::lit("properties"), v0);
    match m2003 {
        Adt106::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt106::C1(v1) => {
            let m2004 = f187(v1);
            match m2004 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2005) => {
                            h2005
                        }
                    }
                }
            }
        }
    }
}

pub fn f585(p2012: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt102>, p2013: List<Adt104>) -> List<Adt104> {
    let m2011 = (p2012, p2013);
    {
        let mut m2011 = m2011;
        loop {
            let step = {
                let (v0, v4) = m2011;
                let m2006 = v0;
                match m2006.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2008 = f261({
                            let (h2007, _) = v5.clone();
                            h2007
                        }, v1.clone());
                        match m2008 {
                            Adt106::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v6, List::cons(Adt104::C0(v2.clone(), {
                                    let (_, h2009) = v5;
                                    h2009
                                }, List::cons(Adt102::C0({
                                    let (h2010, _) = v5;
                                    h2010
                                }), v3.clone())), v4)))
                            }
                            Adt106::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2011 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(p2018: List<Adt6>, p2019: u64, v2: Adt6, v3: List<Adt102>, p2020: List<Adt104>) -> R<List<Adt104>> {
    let m2017 = (p2018, (p2019, p2020));
    {
        let mut m2017 = m2017;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2017;
                let m2014 = v0;
                match m2014.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt104>)), R<List<Adt104>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt104>)), R<List<Adt104>>>((v6, ({
                            let a2015 = v1;
                            let a2016 = 1u64;
                            nat_add(a2015, a2016)?
                        }, List::cons(Adt104::C0(v2.clone(), v5, List::cons(Adt102::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2017 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(p2023: List<Adt6>, v1: Adt6, v2: List<Adt102>, p2024: List<Adt104>) -> List<Adt104> {
    let m2022 = (p2023, p2024);
    {
        let mut m2022 = m2022;
        loop {
            let step = {
                let (v0, v3) = m2022;
                let m2021 = v0;
                match m2021.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt104>), List<Adt104>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt104>), List<Adt104>>((v5, List::cons(Adt104::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2025 = f661(v0)?;
    match m2025 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2026 = f661(v1)?;
            match m2026 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2027 = v2;
                    let a2028 = 61u64;
                    if nat_eq(a2027, a2028) {
                        f662(v5, v6, v4, v3)
                    } else {
                        let m2029 = f661(v2)?;
                        match m2029 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2030 = v3;
                                let a2031 = 61u64;
                                if nat_eq(a2030, a2031) {
                                    f663(v5, v6, v7, v4)
                                } else {
                                    let m2032 = f661(v3)?;
                                    match m2032 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f664(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f589(v0: List<u64>) -> bool {
    let m2033 = v0;
    match m2033.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f590(v0: Adt150) -> bool {
    let m2034 = v0;
    match m2034 {
        Adt150::C0 => {
            false
        }
        Adt150::C1 => {
            false
        }
        Adt150::C2 => {
            true
        }
    }
}

pub fn f591() -> i64 {
    999999999999999999i64
}

pub fn f592(v0: i64) -> R<List<u64>> {
    let m2037 = {
        let a2035 = v0;
        let a2036 = 0i64;
        compare(a2035, a2036)
    };
    if match m2037 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f665(20u64, {
            let a2038 = v0;
            int_neg(a2038)?
        }, List::<u64>::nil())?))
    } else {
        f665(20u64, v0, List::<u64>::nil())
    }
}

pub fn f593(v0: u64) -> R<List<u64>> {
    let a2039 = v0;
    let a2040 = 34u64;
    if nat_eq(a2039, a2040) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2041 = v0;
        let a2042 = 92u64;
        if nat_eq(a2041, a2042) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2043 = v0;
            let a2044 = 8u64;
            if nat_eq(a2043, a2044) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2045 = v0;
                let a2046 = 9u64;
                if nat_eq(a2045, a2046) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2047 = v0;
                    let a2048 = 10u64;
                    if nat_eq(a2047, a2048) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2049 = v0;
                        let a2050 = 12u64;
                        if nat_eq(a2049, a2050) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2051 = v0;
                            let a2052 = 13u64;
                            if nat_eq(a2051, a2052) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2053 = v0;
                                let a2054 = 32u64;
                                if nat_lt(a2053, a2054) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f666(f643(v0, 16u64))?, List::cons(f666(f642(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f594(v0: List<Str>) -> Adt6 {
    Adt6::C4(f540(v0))
}

pub fn f595(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2072 = f627(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2055, _, _, _, _, _, _) => {
                h2055
            }
        });
        let a2073 = {
            let a2070 = f627(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2056, _, _, _, _, _) => {
                    h2056
                }
            });
            let a2071 = {
                let a2068 = f627(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2057, _, _, _, _) => {
                        h2057
                    }
                });
                let a2069 = {
                    let a2066 = f667(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2058, _, _, _) => {
                            h2058
                        }
                    });
                    let a2067 = {
                        let a2064 = f668(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2059, _, _) => {
                                h2059
                            }
                        });
                        let a2065 = {
                            let a2062 = f667(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2060, _) => {
                                    h2060
                                }
                            });
                            let a2063 = f627(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2061) => {
                                    h2061
                                }
                            });
                            append_list(a2062, a2063)
                        };
                        append_list(a2064, a2065)
                    };
                    append_list(a2066, a2067)
                };
                append_list(a2068, a2069)
            };
            append_list(a2070, a2071)
        };
        append_list(a2072, a2073)
    })
}

pub fn f596(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2074 = v1;
    match m2074 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f597(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f145(match v0.clone() {
        Adt10::C0(h2075, _) => {
            h2075
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f669(match v0 {
        Adt10::C0(_, h2076) => {
            h2076
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f598(v0: List<Adt35>) -> List<Adt6> {
    f469(f670(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f599(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m2077 = f671(v0, List::<Adt6>::nil())?;
    match m2077 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f469(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f600(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m2078 = v1;
    match m2078 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f601(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m2079 = v1;
    match m2079 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f602(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f469(f672(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f603(v0: List<Adt46>) -> List<Adt6> {
    f469(f673(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f604(v0: List<Adt54>) -> List<Adt6> {
    f469(f674(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f605(v0: List<Adt56>) -> List<Adt6> {
    f469(f675(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f606(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m2080 = f676(v0, List::<Adt6>::nil())?;
    match m2080 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f469(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f607(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2081 = v1;
    match m2081 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f145(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f608(p2087: List<(Str, Adt64)>, p2088: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m2086 = (p2087, p2088);
    {
        let mut m2086 = m2086;
        loop {
            let step = {
                let (v0, v1) = m2086;
                let m2082 = v0;
                match m2082.uncons() {
                    None => {
                        Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2084 = f677({
                            let (_, h2083) = v2;
                            h2083
                        })?;
                        match m2084 {
                            None => {
                                Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h2085, _) = v2;
                                    h2085
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2086 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f609(v0: Adt81) -> List<(Str, Adt64)> {
    let m2089 = v0;
    match m2089 {
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

pub fn f610(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2090 = v0.clone();
    match m2090.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f471(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f611(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2091 = v0;
    match m2091 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f612(v0: Adt81) -> Option<Str> {
    let m2092 = v0;
    match m2092 {
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

pub fn f613() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f614(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2093, _, _, _, _, _) => {
            h2093
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2094, _, _, _) => {
            h2094
        }
    })), {
        let a2103 = f596(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2095, _, _, _, _) => {
                h2095
            }
        });
        let a2104 = {
            let a2101 = f596(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2096, _, _) => {
                    h2096
                }
            });
            let a2102 = {
                let a2099 = f596(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2097, _) => {
                        h2097
                    }
                });
                let a2100 = f678(match v0 {
                    Adt1::C0(_, _, _, _, _, h2098) => {
                        h2098
                    }
                });
                append_list(a2099, a2100)
            };
            append_list(a2101, a2102)
        };
        append_list(a2103, a2104)
    })))
}

pub fn f615(v0: List<List<u64>>) -> bool {
    let m2105 = v0;
    match m2105.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f616() -> List<Adt86> {
    List::cons(Adt86::C0(f679(), f543()), List::cons(Adt86::C0(f680(), f681()), List::cons(Adt86::C0(f682(), f683()), List::<Adt86>::nil())))
}

pub fn f617(v0: List<List<u64>>) -> List<u64> {
    f684(f50(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f618(p2120: u64, p2121: Adt147, p2122: List<Adt148>) -> R<Option<Adt6>> {
    let m2119 = (p2120, (p2121, p2122));
    {
        let mut m2119 = m2119;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2119;
                let m2106 = v0;
                if m2106 == 0 {
                    Err::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m2106 - 1;
                    let m2107 = v1;
                    match m2107 {
                        Adt147::C0(v4) => {
                            let m2108 = v4.clone();
                            match m2108 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f685(v7)?;
                                    Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m2109 = v9;
                                    match m2109.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C0(v10), List::cons(Adt148::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m2110 = v12;
                                    match m2110.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f685({
                                                let (h2111, _) = v13;
                                                h2111
                                            })?;
                                            Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C0({
                                                let (_, h2112) = v13;
                                                h2112
                                            }), List::cons(Adt148::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt147::C1(v16) => {
                            let m2113 = v2;
                            match m2113.uncons() {
                                None => {
                                    Err::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m2114 = v17;
                                    match m2114 {
                                        Adt148::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m2115 = v20;
                                            match m2115.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C1(Adt6::C4(f469(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C0(v22), List::cons(Adt148::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt148::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m2116 = v26;
                                            match m2116.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C1(Adt6::C5(f471(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f685({
                                                        let (h2117, _) = v28;
                                                        h2117
                                                    })?;
                                                    Ok::<(u64, (Adt147, List<Adt148>)), R<Option<Adt6>>>((v3, (Adt147::C0({
                                                        let (_, h2118) = v28;
                                                        h2118
                                                    }), List::cons(Adt148::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m2119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f619() -> u64 {
    67108864u64
}

pub fn f620(v0: Adt19) -> Adt6 {
    let m2123 = v0;
    match m2123 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f621(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2124 = v0;
    match m2124 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f622(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2125 = v0;
    match m2125 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f623() -> i64 {
    -32603i64
}

pub fn f624() -> i64 {
    429i64
}

pub fn f625() -> i64 {
    -32021i64
}

pub fn f626(p2128: List<Str>, p2129: List<Adt6>) -> List<Adt6> {
    let m2127 = (p2128, p2129);
    {
        let mut m2127 = m2127;
        loop {
            let step = {
                let (v0, v1) = m2127;
                let m2126 = v0;
                match m2126.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2130 = v1;
    match m2130 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2131) => {
                    h2131
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f628(v0: List<u64>) -> R<bool> {
    let m2132 = v0;
    match m2132.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2133 = v2;
            match m2133.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2134 = f352(v1)?;
                    let a2135 = 113u64;
                    if if nat_eq(a2134, a2135) {
                        let a2136 = v3;
                        let a2137 = 61u64;
                        nat_eq(a2136, a2137)
                    } else {
                        false
                    } {
                        Ok(f686(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f629(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt132>) -> R<Result<Adt128, Adt131>> {
    let m2138 = v3;
    match m2138 {
        Ok(v5) => {
            f687(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(Adt131::C0(v4)))
        }
    }
}

pub fn f630(v0: Adt138, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt132> {
    let m2139 = v0;
    match m2139 {
        Adt138::C0 => {
            f688(v1, v2)
        }
        Adt138::C1 => {
            f689(v1, v2)
        }
        Adt138::C2 => {
            f690(v1, v2)
        }
        Adt138::C3 => {
            f691(v1, v2)
        }
        Adt138::C4 => {
            f692(v1, v2)
        }
        Adt138::C5 => {
            f693(v1, v2)
        }
        Adt138::C6 => {
            f694(v1, v2)
        }
        Adt138::C7 => {
            f695(v1, v2)
        }
        Adt138::C8 => {
            f696(v1, v2)
        }
        Adt138::C9 => {
            f697(v1, v2)
        }
        Adt138::C10 => {
            f698(v1, v2)
        }
        Adt138::C11 => {
            f699(v1, v2)
        }
    }
}

pub fn f631(v0: Adt138) -> Str {
    let m2140 = v0;
    match m2140 {
        Adt138::C0 => {
            Str::lit("server/discover")
        }
        Adt138::C1 => {
            Str::lit("tools/list")
        }
        Adt138::C2 => {
            Str::lit("tools/call")
        }
        Adt138::C3 => {
            Str::lit("resources/list")
        }
        Adt138::C4 => {
            Str::lit("resources/read")
        }
        Adt138::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt138::C6 => {
            Str::lit("prompts/list")
        }
        Adt138::C7 => {
            Str::lit("prompts/get")
        }
        Adt138::C8 => {
            Str::lit("completion/complete")
        }
        Adt138::C9 => {
            Str::lit("tasks/get")
        }
        Adt138::C10 => {
            Str::lit("tasks/update")
        }
        Adt138::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f632(v0: Option<Adt8>) -> bool {
    let m2141 = v0;
    match m2141 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f633(v0: Option<Adt9>) -> bool {
    let m2142 = v0;
    match m2142 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f634(v0: Option<Adt5>, v1: Str) -> bool {
    let m2143 = v0;
    match m2143 {
        None => {
            false
        }
        Some(v2) => {
            f410(v1, v2)
        }
    }
}

pub fn f635() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f636(v0: List<Str>, v1: Adt107, v2: Adt16, v3: Option<Adt112>) -> Adt142 {
    let m2144 = v3;
    match m2144 {
        None => {
            f700(v1, v2.clone(), f701(match v2 {
                Adt16::C0(h2145, _, _, _, _, _) => {
                    h2145
                }
            }, v0))
        }
        Some(v4) => {
            Adt142::C1(Adt131::C2(v4))
        }
    }
}

pub fn f637(v0: Option<Adt111>, v1: Adt144, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt112>> {
    let m2146 = v0;
    match m2146 {
        None => {
            Ok(None::<Adt112>)
        }
        Some(v5) => {
            let m2148 = match v5.clone() {
                Adt111::C0(_, _, _, h2147, _) => {
                    h2147
                }
            };
            match m2148 {
                None => {
                    f702(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f638(v0: Option<Adt5>) -> Adt5 {
    let m2149 = v0;
    match m2149 {
        None => {
            f26()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f639(v0: Adt5) -> Result<Adt16, Adt132> {
    let m2150 = f703(v0.clone());
    match m2150 {
        Adt151::C0 => {
            f704(v0.clone(), f186(f705(), v0.clone()), f186(f706(), v0))
        }
        Adt151::C1 => {
            Err::<Adt16, Adt132>(Adt132::C2)
        }
        Adt151::C2 => {
            Err::<Adt16, Adt132>(Adt132::C3)
        }
    }
}

pub fn f640() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f641(v0: Str) -> R<List<u64>> {
    f206({
        let a2151 = v0;
        utf8_encode(a2151)
    })
}

pub fn f642(v0: u64, v1: u64) -> u64 {
    let a2152 = v0;
    let a2153 = v1;
    let a2154 = 0u64;
    nat_rem(a2152, a2153, a2154)
}

pub fn f643(v0: u64, v1: u64) -> u64 {
    let a2155 = v0;
    let a2156 = v1;
    let a2157 = 0u64;
    nat_quot(a2155, a2156, a2157)
}

pub fn f644(v0: u64, v1: Adt6) -> bool {
    let a2158 = v0;
    let a2159 = 0u64;
    if nat_eq(a2158, a2159) {
        f707(v1)
    } else {
        let a2160 = v0;
        let a2161 = 1u64;
        if nat_eq(a2160, a2161) {
            f708(v1)
        } else {
            let a2162 = v0;
            let a2163 = 2u64;
            if nat_eq(a2162, a2163) {
                f709(v1)
            } else {
                let a2164 = v0;
                let a2165 = 3u64;
                if nat_eq(a2164, a2165) {
                    f710(v1)
                } else {
                    let a2166 = v0;
                    let a2167 = 4u64;
                    if nat_eq(a2166, a2167) {
                        f711(v1)
                    } else {
                        let a2168 = v0;
                        let a2169 = 5u64;
                        if nat_eq(a2168, a2169) {
                            f712(v1)
                        } else {
                            let a2170 = v0;
                            let a2171 = 6u64;
                            if nat_eq(a2170, a2171) {
                                f713(v1)
                            } else {
                                let a2172 = v0;
                                let a2173 = 7u64;
                                if nat_eq(a2172, a2173) {
                                    f714(v1)
                                } else {
                                    f715(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f645(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt124) -> R<Adt145> {
    let m2174 = v3.clone();
    match m2174.uncons() {
        None => {
            Ok(Adt145::C0(Adt91::C8(v0)))
        }
        Some((_, _)) => {
            if f356(v3.clone(), true) {
                let m2175 = f716(v1);
                match m2175 {
                    None => {
                        Ok(Adt145::C0(Adt91::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f717(v3)?;
                        if f718(v9.clone(), match v5.clone() {
                            Adt124::C0(_, h2176) => {
                                h2176
                            }
                        }) {
                            Ok(Adt145::C0(Adt91::C12(v0, v2)))
                        } else {
                            Ok(Adt145::C1(Adt124::C0(List::cons(Adt12::C0(v0, v2, f250(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt124::C0(h2177, _) => {
                                    h2177
                                }
                            }), List::cons(v9, match v5 {
                                Adt124::C0(_, h2178) => {
                                    h2178
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt145::C0(Adt91::C9(v0)))
            }
        }
    }
}

pub fn f646(p2183: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2184: List<Adt108>) -> List<Adt108> {
    let m2182 = (p2183, p2184);
    {
        let mut m2182 = m2182;
        loop {
            let step = {
                let (v0, v3) = m2182;
                let m2179 = v0;
                match m2179.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt108>), List<Adt108>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt108>), List<Adt108>>((v5, List::cons(Adt108::C0({
                            let (_, h2180) = v4;
                            h2180
                        }, v1, List::cons({
                            let (h2181, _) = v4;
                            h2181
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647(p2188: List<(Str, Adt6)>, p2189: List<Adt108>) -> List<Adt108> {
    let m2187 = (p2188, p2189);
    {
        let mut m2187 = m2187;
        loop {
            let step = {
                let (v0, v1) = m2187;
                let m2185 = v0;
                match m2185.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt108>), List<Adt108>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt108>), List<Adt108>>((v3, List::cons(Adt108::C0({
                            let (_, h2186) = v2;
                            h2186
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(p2192: List<Adt6>, p2193: List<Adt108>) -> List<Adt108> {
    let m2191 = (p2192, p2193);
    {
        let mut m2191 = m2191;
        loop {
            let step = {
                let (v0, v1) = m2191;
                let m2190 = v0;
                match m2190.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt108>), List<Adt108>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt108>), List<Adt108>>((v3, List::cons(Adt108::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2191 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f649(v0: Adt6) -> Str {
    let m2194 = v0;
    match m2194 {
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

pub fn f650(v0: Adt7) -> R<bool> {
    if f555(match v0.clone() {
        Adt7::C0(h2195, _) => {
            h2195
        }
    }) {
        Ok(true)
    } else {
        f719(40u64, match v0.clone() {
            Adt7::C0(h2196, _) => {
                h2196
            }
        }, match v0 {
            Adt7::C0(_, h2197) => {
                h2197
            }
        })
    }
}

pub fn f651(v0: Adt6) -> Str {
    let m2198 = f265(v0);
    match m2198 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f652(p2205: u64, p2206: List<(Adt6, Adt6)>) -> R<Adt149> {
    let m2204 = (p2205, p2206);
    {
        let mut m2204 = m2204;
        loop {
            let step = {
                let (v0, v1) = m2204;
                let m2199 = v0;
                if m2199 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt149>>(Ok(Adt149::C2))
                } else {
                    let v2 = m2199 - 1;
                    let m2200 = v1;
                    match m2200.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt149>>(Ok(Adt149::C0))
                        }
                        Some((v3, v4)) => {
                            let m2203 = f720({
                                let (h2201, _) = v3;
                                h2201
                            }, {
                                let (_, h2202) = v3;
                                h2202
                            }, v4)?;
                            match m2203 {
                                Adt152::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt149>>(Ok(Adt149::C1))
                                }
                                Adt152::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt149>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f653() -> u64 {
    67108864u64
}

pub fn f654(v0: Adt150) -> bool {
    let m2207 = v0;
    match m2207 {
        Adt150::C0 => {
            true
        }
        Adt150::C1 => {
            false
        }
        Adt150::C2 => {
            false
        }
    }
}

pub fn f655(v0: Adt7, v1: Adt7, v2: Adt150, v3: Adt150) -> R<Adt150> {
    let m2208 = v2;
    match m2208 {
        Adt150::C0 => {
            let m2210 = v3;
            match m2210 {
                Adt150::C0 => {
                    Ok(f721(Adt150::C0, f722(f723(match v0.clone() {
                        Adt7::C0(h2211, _) => {
                            h2211
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2212) => {
                            h2212
                        }
                    }, f723(match v1.clone() {
                        Adt7::C0(h2213, _) => {
                            h2213
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2214) => {
                            h2214
                        }
                    })?))
                }
                Adt150::C1 => {
                    Ok(Adt150::C0)
                }
                Adt150::C2 => {
                    Ok(Adt150::C0)
                }
            }
        }
        Adt150::C1 => {
            let m2209 = v3;
            match m2209 {
                Adt150::C0 => {
                    Ok(Adt150::C2)
                }
                Adt150::C1 => {
                    Ok(Adt150::C1)
                }
                Adt150::C2 => {
                    Ok(Adt150::C0)
                }
            }
        }
        Adt150::C2 => {
            let m2215 = v3;
            match m2215 {
                Adt150::C0 => {
                    Ok(Adt150::C2)
                }
                Adt150::C1 => {
                    Ok(Adt150::C2)
                }
                Adt150::C2 => {
                    f722(match v0.clone() {
                        Adt7::C0(h2216, _) => {
                            h2216
                        }
                    }, match v0 {
                        Adt7::C0(_, h2217) => {
                            h2217
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2218, _) => {
                            h2218
                        }
                    }, match v1 {
                        Adt7::C0(_, h2219) => {
                            h2219
                        }
                    })
                }
            }
        }
    }
}

pub fn f656(v0: i64) -> Adt150 {
    f658(v0, 0i64)
}

pub fn f657(v0: Adt6) -> Option<i64> {
    let m2220 = v0;
    match m2220 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f555(match v2.clone() {
                Adt7::C0(_, h2221) => {
                    h2221
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2222, _) => {
                        h2222
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

pub fn f658(v0: i64, v1: i64) -> Adt150 {
    let m2225 = {
        let a2223 = v0;
        let a2224 = v1;
        compare(a2223, a2224)
    };
    if match m2225 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Adt150::C0
    } else {
        let m2228 = {
            let a2226 = v1;
            let a2227 = v0;
            compare(a2226, a2227)
        };
        if match m2228 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt150::C2
        } else {
            Adt150::C1
        }
    }
}

pub fn f659(p2237: List<u64>, p2238: u64) -> R<u64> {
    let m2236 = (p2237, p2238);
    {
        let mut m2236 = m2236;
        loop {
            let step = {
                let (v0, v1) = m2236;
                let m2229 = v0;
                match m2229.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2230 = v2;
                        let a2231 = 128u64;
                        if if nat_lt(a2230, a2231) {
                            true
                        } else {
                            let a2232 = 191u64;
                            let a2233 = v2;
                            nat_lt(a2232, a2233)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2234 = v1;
                                let a2235 = 1u64;
                                nat_add(a2234, a2235)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f660(v0: Str, p2256: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2255 = p2256;
    {
        let mut m2255 = m2255;
        loop {
            let step = {
                let v1 = m2255;
                let m2239 = v1;
                match m2239.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2243 = v0.clone();
                        let a2244 = {
                            let a2241 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2240, _) = v2.clone();
                                h2240
                            }, List::<Str>::nil()));
                            let a2242 = Str::lit("");
                            join(a2241, a2242)
                        };
                        if equal(a2243, a2244) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2245) = v2;
                                h2245
                            }))
                        } else {
                            let m2247 = f187({
                                let (_, h2246) = v2.clone();
                                h2246
                            });
                            match m2247 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2248 = f186(Str::lit("$anchor"), v4);
                                    match m2248 {
                                        Adt106::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt106::C1(v5) => {
                                            let m2249 = f265(v5);
                                            match m2249 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2252 = v0.clone();
                                                    let a2253 = {
                                                        let a2250 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2251 = Str::lit("");
                                                        join(a2250, a2251)
                                                    };
                                                    if equal(a2252, a2253) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2254) = v2;
                                                            h2254
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
                Ok(next) => { m2255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f661(v0: u64) -> R<Option<u64>> {
    let a2257 = 64u64;
    let a2258 = v0;
    if if nat_lt(a2257, a2258) {
        let a2259 = v0;
        let a2260 = 91u64;
        nat_lt(a2259, a2260)
    } else {
        false
    } {
        Ok(Some({
            let a2261 = v0;
            let a2262 = 65u64;
            nat_sub(a2261, a2262)
        }))
    } else {
        let a2263 = 96u64;
        let a2264 = v0;
        if if nat_lt(a2263, a2264) {
            let a2265 = v0;
            let a2266 = 123u64;
            nat_lt(a2265, a2266)
        } else {
            false
        } {
            Ok(Some({
                let a2267 = v0;
                let a2268 = 71u64;
                nat_sub(a2267, a2268)
            }))
        } else {
            let a2269 = 47u64;
            let a2270 = v0;
            if if nat_lt(a2269, a2270) {
                let a2271 = v0;
                let a2272 = 58u64;
                nat_lt(a2271, a2272)
            } else {
                false
            } {
                Ok(Some({
                    let a2273 = v0;
                    let a2274 = 4u64;
                    nat_add(a2273, a2274)?
                }))
            } else {
                let a2275 = v0;
                let a2276 = 43u64;
                if nat_eq(a2275, a2276) {
                    Ok(Some(62u64))
                } else {
                    let a2277 = v0;
                    let a2278 = 47u64;
                    if nat_eq(a2277, a2278) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f662(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2279 = v3;
        let a2280 = 61u64;
        if nat_eq(a2279, a2280) {
            let a2281 = f724(v1, 16u64);
            let a2282 = 0u64;
            nat_eq(a2281, a2282)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f725(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f663(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2283 = f724(v2, 4u64);
        let a2284 = 0u64;
        nat_eq(a2283, a2284)
    } else {
        false
    } {
        Ok(Some(f726(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f664(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2287 = {
            let a2285 = v0;
            let a2286 = 4u64;
            nat_mul(a2285, a2286)?
        };
        let a2288 = f727(v1, 16u64);
        nat_add(a2287, a2288)?
    }, List::cons({
        let a2291 = {
            let a2289 = f724(v1, 16u64);
            let a2290 = 16u64;
            nat_mul(a2289, a2290)?
        };
        let a2292 = f727(v2, 4u64);
        nat_add(a2291, a2292)?
    }, List::cons({
        let a2295 = {
            let a2293 = f724(v2, 4u64);
            let a2294 = 64u64;
            nat_mul(a2293, a2294)?
        };
        let a2296 = v3;
        nat_add(a2295, a2296)?
    }, List::<u64>::nil()))))
}

pub fn f665(p2308: u64, p2309: i64, p2310: List<u64>) -> R<List<u64>> {
    let m2307 = (p2308, (p2309, p2310));
    {
        let mut m2307 = m2307;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2307;
                let m2297 = v0;
                if m2297 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2297 - 1;
                    let v4: i64 = {
                        let a2298 = v1;
                        let a2299 = 10i64;
                        let a2300 = 0i64;
                        int_quot(a2298, a2299, a2300)?
                    };
                    let v5: i64 = {
                        let a2301 = v1;
                        let a2302 = 10i64;
                        let a2303 = 0i64;
                        int_rem(a2301, a2302, a2303)
                    };
                    let m2306 = {
                        let a2304 = v4;
                        let a2305 = 1i64;
                        compare(a2304, a2305)
                    };
                    if match m2306 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f728(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f728(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f666(v0: u64) -> R<u64> {
    let a2311 = v0;
    let a2312 = 10u64;
    if nat_lt(a2311, a2312) {
        let a2313 = 48u64;
        let a2314 = v0;
        nat_add(a2313, a2314)
    } else {
        let a2315 = 87u64;
        let a2316 = v0;
        nat_add(a2315, a2316)
    }
}

pub fn f667(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2317 = v1;
    match m2317 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f729(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f668(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2318 = v0;
    match m2318 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2321 = f601(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2319, _) => {
                        h2319
                    }
                });
                let a2322 = f601(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2320) => {
                        h2320
                    }
                });
                append_list(a2321, a2322)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f669(v0: Adt11) -> Str {
    let m2323 = v0;
    match m2323 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f670(p2326: List<Adt35>, p2327: List<Adt6>) -> List<Adt6> {
    let m2325 = (p2326, p2327);
    {
        let mut m2325 = m2325;
        loop {
            let step = {
                let (v0, v1) = m2325;
                let m2324 = v0;
                match m2324.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f730(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f671(p2331: List<Adt38>, p2332: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2330 = (p2331, p2332);
    {
        let mut m2330 = m2330;
        loop {
            let step = {
                let (v0, v1) = m2330;
                let m2328 = v0;
                match m2328.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2329 = f731(v2)?;
                        match m2329 {
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
                Ok(next) => { m2330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f672(p2335: List<Adt44>, p2336: List<Adt6>) -> R<List<Adt6>> {
    let m2334 = (p2335, p2336);
    {
        let mut m2334 = m2334;
        loop {
            let step = {
                let (v0, v1) = m2334;
                let m2333 = v0;
                match m2333.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f732(v2)?, v1)))
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

pub fn f673(p2339: List<Adt46>, p2340: List<Adt6>) -> List<Adt6> {
    let m2338 = (p2339, p2340);
    {
        let mut m2338 = m2338;
        loop {
            let step = {
                let (v0, v1) = m2338;
                let m2337 = v0;
                match m2337.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f733(v2), v1)))
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

pub fn f674(p2343: List<Adt54>, p2344: List<Adt6>) -> List<Adt6> {
    let m2342 = (p2343, p2344);
    {
        let mut m2342 = m2342;
        loop {
            let step = {
                let (v0, v1) = m2342;
                let m2341 = v0;
                match m2341.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f734(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(p2347: List<Adt56>, p2348: List<Adt6>) -> List<Adt6> {
    let m2346 = (p2347, p2348);
    {
        let mut m2346 = m2346;
        loop {
            let step = {
                let (v0, v1) = m2346;
                let m2345 = v0;
                match m2345.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f735(v2), v1)))
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

pub fn f676(p2352: List<Adt59>, p2353: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2351 = (p2352, p2353);
    {
        let mut m2351 = m2351;
        loop {
            let step = {
                let (v0, v1) = m2351;
                let m2349 = v0;
                match m2349.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2350 = f736(v2)?;
                        match m2350 {
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
                Ok(next) => { m2351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677(v0: Adt64) -> R<Option<Adt6>> {
    let m2354 = v0;
    match m2354 {
        Adt64::C0(v1) => {
            Ok(Some(f737(Str::lit("elicitation/create"), Some(f738(v1)))))
        }
        Adt64::C1(v2) => {
            let m2355 = f739(v2)?;
            match m2355 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f737(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt64::C2(v4) => {
            Ok(Some(f737(Str::lit("roots/list"), {
                let m2356 = v4;
                match m2356 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h2357) => {
                                h2357
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f678(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2358 = v0;
    match m2358 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f740(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f679() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f680() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f681() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f682() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f683() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f684(p2363: List<List<u64>>, p2364: List<u64>) -> List<u64> {
    let m2362 = (p2363, p2364);
    {
        let mut m2362 = m2362;
        loop {
            let step = {
                let (v0, v1) = m2362;
                let m2359 = v0;
                match m2359.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2360 = f741(v2);
                            let a2361 = v1;
                            append_list(a2360, a2361)
                        }))
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

pub fn f685(v0: Str) -> R<Str> {
    let m2367 = {
        let a2366 = f318(f319(), f332(f742(f206({
            let a2365 = v0;
            utf8_encode(a2365)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2366)
    };
    match m2367 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f686(v0: List<u64>) -> bool {
    let m2368 = v0;
    match m2368.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2369 = v1;
            let a2370 = 48u64;
            if nat_eq(a2369, a2370) {
                f743(v2)
            } else {
                false
            }
        }
    }
}

pub fn f687(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt128, Adt131>> {
    if f744(f745(v3.clone())) {
        Ok(Ok::<Adt128, Adt131>(f746(v0.clone(), Adt32::C0(f747(v0)))))
    } else {
        Ok(f748(v0.clone(), v3.clone(), f749(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2371) => {
                h2371
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2372, _) => {
                h2372
            }
        }, v2, v1, v3)?))
    }
}

pub fn f688(v0: Adt16, _: Adt5) -> Result<Adt14, Adt132> {
    Ok::<Adt14, Adt132>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f689(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2373 = f750(Str::lit("cursor"), v1);
    match m2373 {
        Ok(v2) => {
            Ok::<Adt14, Adt132>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt132>(v3)
        }
    }
}

pub fn f690(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2374 = f751(Str::lit("name"), v1.clone());
    match m2374 {
        Ok(v2) => {
            let m2375 = f752(Str::lit("arguments"), v1.clone());
            match m2375 {
                Ok(v3) => {
                    let m2376 = f752(Str::lit("inputResponses"), v1.clone());
                    match m2376 {
                        Ok(v4) => {
                            let m2377 = f750(Str::lit("requestState"), v1);
                            match m2377 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt132>(Adt14::C2(Adt21::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt132>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt132>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt132>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt132>(v9)
        }
    }
}

pub fn f691(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2378 = f750(Str::lit("cursor"), v1);
    match m2378 {
        Ok(v2) => {
            Ok::<Adt14, Adt132>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt132>(v3)
        }
    }
}

pub fn f692(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2379 = f751(Str::lit("uri"), v1.clone());
    match m2379 {
        Ok(v2) => {
            let m2380 = f752(Str::lit("inputResponses"), v1.clone());
            match m2380 {
                Ok(v3) => {
                    let m2381 = f750(Str::lit("requestState"), v1);
                    match m2381 {
                        Ok(v4) => {
                            Ok::<Adt14, Adt132>(Adt14::C4(Adt22::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt14, Adt132>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt14, Adt132>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt14, Adt132>(v7)
        }
    }
}

pub fn f693(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2382 = f750(Str::lit("cursor"), v1);
    match m2382 {
        Ok(v2) => {
            Ok::<Adt14, Adt132>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt132>(v3)
        }
    }
}

pub fn f694(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2383 = f750(Str::lit("cursor"), v1);
    match m2383 {
        Ok(v2) => {
            Ok::<Adt14, Adt132>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt132>(v3)
        }
    }
}

pub fn f695(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2384 = f751(Str::lit("name"), v1.clone());
    match m2384 {
        Ok(v2) => {
            let m2385 = f753(Str::lit("arguments"), v1.clone());
            match m2385 {
                Ok(v3) => {
                    let m2386 = f752(Str::lit("inputResponses"), v1.clone());
                    match m2386 {
                        Ok(v4) => {
                            let m2387 = f750(Str::lit("requestState"), v1);
                            match m2387 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt132>(Adt14::C7(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt132>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt132>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt132>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt132>(v9)
        }
    }
}

pub fn f696(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2388 = f754(Str::lit("ref"), v1.clone());
    match m2388 {
        Ok(v2) => {
            let m2389 = f755(v2);
            match m2389 {
                Ok(v3) => {
                    let m2390 = f754(Str::lit("argument"), v1.clone());
                    match m2390 {
                        Ok(v4) => {
                            let m2391 = f756(v4);
                            match m2391 {
                                Ok(v5) => {
                                    let m2392 = f757(v1);
                                    match m2392 {
                                        Ok(v6) => {
                                            Ok::<Adt14, Adt132>(Adt14::C8(Adt24::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt14, Adt132>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt14, Adt132>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt14, Adt132>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt14, Adt132>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt14, Adt132>(v11)
        }
    }
}

pub fn f697(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2393 = f751(Str::lit("taskId"), v1);
    match m2393 {
        Ok(v2) => {
            Ok::<Adt14, Adt132>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt132>(v3)
        }
    }
}

pub fn f698(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2394 = f751(Str::lit("taskId"), v1.clone());
    match m2394 {
        Ok(v2) => {
            let m2395 = f754(Str::lit("inputResponses"), v1);
            match m2395 {
                Ok(v3) => {
                    Ok::<Adt14, Adt132>(Adt14::C10(Adt30::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt14, Adt132>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt14, Adt132>(v5)
        }
    }
}

pub fn f699(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt132> {
    let m2396 = f751(Str::lit("taskId"), v1);
    match m2396 {
        Ok(v2) => {
            Ok::<Adt14, Adt132>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt132>(v3)
        }
    }
}

pub fn f700(v0: Adt107, v1: Adt16, v2: Option<Adt131>) -> Adt142 {
    let m2397 = v2;
    match m2397 {
        None => {
            Adt142::C0(Adt143::C0(match v0.clone() {
                Adt107::C0(h2398, _, _) => {
                    h2398
                }
            }, match v0.clone() {
                Adt107::C0(_, h2399, _) => {
                    h2399
                }
            }, v1, f638(match v0 {
                Adt107::C0(_, _, h2400) => {
                    h2400
                }
            })))
        }
        Some(v3) => {
            Adt142::C1(v3)
        }
    }
}

pub fn f701(v0: Str, v1: List<Str>) -> Option<Adt131> {
    f758(f759(v0.clone(), v1.clone()), v0, v1)
}

pub fn f702(v0: Adt111, v1: Adt144, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt112>> {
    let m2402 = match v0.clone() {
        Adt111::C0(h2401, _, _, _, _) => {
            h2401
        }
    };
    match m2402 {
        None => {
            Ok(Some(Adt112::C0))
        }
        Some(v5) => {
            let a2403 = v5;
            let a2404 = v3;
            if equal(a2403, a2404) {
                let m2406 = match v0.clone() {
                    Adt111::C0(_, h2405, _, _, _) => {
                        h2405
                    }
                };
                match m2406 {
                    None => {
                        Ok(Some(Adt112::C1))
                    }
                    Some(v6) => {
                        let a2407 = v6;
                        let a2408 = v2;
                        if equal(a2407, a2408) {
                            f760(f761(match v0.clone() {
                                Adt111::C0(_, _, h2409, _, _) => {
                                    h2409
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt111::C0(_, _, _, _, h2410) => {
                                    h2410
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt112::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt112::C3))
            }
        }
    }
}

pub fn f703(v0: Adt5) -> Adt151 {
    f762(f186(f705(), v0.clone()), f186(f706(), v0))
}

pub fn f704(v0: Adt5, v1: Adt106, v2: Adt106) -> Result<Adt16, Adt132> {
    let m2411 = v1;
    match m2411 {
        Adt106::C0 => {
            Err::<Adt16, Adt132>(Adt132::C2)
        }
        Adt106::C1(v3) => {
            let m2412 = v2;
            match m2412 {
                Adt106::C0 => {
                    Err::<Adt16, Adt132>(Adt132::C3)
                }
                Adt106::C1(v4) => {
                    f763(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f705() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f706() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f707(v0: Adt6) -> bool {
    let m2413 = f265(v0);
    match m2413 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f708(v0: Adt6) -> bool {
    let m2414 = f573(v0);
    match m2414 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f709(v0: Adt6) -> bool {
    let m2415 = f657(v0);
    match m2415 {
        None => {
            false
        }
        Some(v1) => {
            let a2419 = {
                let m2418 = {
                    let a2416 = v1;
                    let a2417 = 0i64;
                    compare(a2416, a2417)
                };
                match m2418 {
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
            bool_not(a2419)
        }
    }
}

pub fn f710(v0: Adt6) -> bool {
    let m2420 = f241(v0);
    match m2420 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f711(v0: Adt6) -> bool {
    let m2421 = f187(v0);
    match m2421 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f712(v0: Adt6) -> bool {
    let m2422 = f241(v0);
    match m2422 {
        None => {
            false
        }
        Some(v1) => {
            f764(v1)
        }
    }
}

pub fn f713(v0: Adt6) -> bool {
    let m2423 = f316(v0);
    match m2423 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f714(v0: Adt6) -> bool {
    let m2424 = f265(v0.clone());
    match m2424 {
        None => {
            let m2425 = f241(v0);
            match m2425 {
                None => {
                    false
                }
                Some(v2) => {
                    f765(v2)
                }
            }
        }
        Some(v1) => {
            f568(v1)
        }
    }
}

pub fn f715(v0: Adt6) -> bool {
    let m2426 = f241(v0);
    match m2426 {
        None => {
            false
        }
        Some(v1) => {
            let m2427 = v1;
            match m2427.uncons() {
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

pub fn f716(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2428 = f261(Str::lit("type"), v0);
    match m2428 {
        Adt106::C0 => {
            None::<Adt13>
        }
        Adt106::C1(v1) => {
            let m2429 = f265(v1);
            match m2429 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f766(v2)
                }
            }
        }
    }
}

pub fn f717(v0: List<u64>) -> R<List<u64>> {
    Ok(f332(f767(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f718(v0: List<u64>, p2432: List<List<u64>>) -> bool {
    let m2431 = p2432;
    {
        let mut m2431 = m2431;
        loop {
            let step = {
                let v1 = m2431;
                let m2430 = v1;
                match m2430.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f768(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
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

pub fn f719(p2444: u64, p2445: i64, p2446: i64) -> R<bool> {
    let m2443 = (p2444, (p2445, p2446));
    {
        let mut m2443 = m2443;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2443;
                let m2433 = v0;
                if m2433 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2433 - 1;
                    let m2436 = {
                        let a2434 = v2;
                        let a2435 = 0i64;
                        compare(a2434, a2435)
                    };
                    if match m2436 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f555({
                            let a2437 = v1;
                            let a2438 = 10i64;
                            let a2439 = 0i64;
                            int_rem(a2437, a2438, a2439)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2440 = v1;
                                let a2441 = 10i64;
                                let a2442 = 0i64;
                                int_quot(a2440, a2441, a2442)?
                            }, f315(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f720(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt152> {
    let a2447 = f769(v0.clone());
    let a2448 = f769(v1.clone());
    if nat_eq(a2447, a2448) {
        let m2449 = v0;
        match m2449 {
            Adt6::C0 => {
                Ok(Adt152::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2450 = f316(v1);
                match m2450 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v4) => {
                        let a2451 = v3;
                        let a2452 = v4;
                        if equal(a2451, a2452) {
                            Ok(Adt152::C1(v2))
                        } else {
                            Ok(Adt152::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2453 = f573(v1);
                match m2453 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v6) => {
                        if f770(f575(v5, v6)?) {
                            Ok(Adt152::C1(v2))
                        } else {
                            Ok(Adt152::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2454 = f265(v1);
                match m2454 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v8) => {
                        let a2455 = v7;
                        let a2456 = v8;
                        if equal(a2455, a2456) {
                            Ok(Adt152::C1(v2))
                        } else {
                            Ok(Adt152::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2457 = f241(v1);
                match m2457 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v10) => {
                        Ok(f771(f772(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2458 = f187(v1);
                match m2458 {
                    None => {
                        Ok(Adt152::C0)
                    }
                    Some(v12) => {
                        let a2460 = f579(v11.clone(), 0u64)?;
                        let a2461 = f579(match v12.clone() {
                            Adt5::C0(h2459) => {
                                h2459
                            }
                        }, 0u64)?;
                        if nat_eq(a2460, a2461) {
                            Ok(f771(f773(v11, match v12 {
                                Adt5::C0(h2462) => {
                                    h2462
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt152::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt152::C0)
    }
}

pub fn f721(v0: Adt150, v1: Adt150) -> Adt150 {
    let m2463 = v0;
    match m2463 {
        Adt150::C0 => {
            f774(v1)
        }
        Adt150::C1 => {
            Adt150::C1
        }
        Adt150::C2 => {
            v1
        }
    }
}

pub fn f722(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt150> {
    let m2464 = f658(f775(v0, v1)?, f775(v2, v3)?);
    match m2464 {
        Adt150::C0 => {
            Ok(Adt150::C0)
        }
        Adt150::C1 => {
            f776(v0, f777(v0)?, v2, f777(v2)?)
        }
        Adt150::C2 => {
            Ok(Adt150::C2)
        }
    }
}

pub fn f723(v0: i64) -> R<i64> {
    let m2467 = {
        let a2465 = v0;
        let a2466 = 0i64;
        compare(a2465, a2466)
    };
    if match m2467 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2468 = v0;
        int_neg(a2468)
    } else {
        Ok(v0)
    }
}

pub fn f724(v0: u64, v1: u64) -> u64 {
    let a2469 = v0;
    let a2470 = v1;
    let a2471 = 0u64;
    nat_rem(a2469, a2470, a2471)
}

pub fn f725(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2474 = {
            let a2472 = v0;
            let a2473 = 4u64;
            nat_mul(a2472, a2473)?
        };
        let a2475 = f727(v1, 16u64);
        nat_add(a2474, a2475)?
    }, List::<u64>::nil()))
}

pub fn f726(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2478 = {
            let a2476 = v0;
            let a2477 = 4u64;
            nat_mul(a2476, a2477)?
        };
        let a2479 = f727(v1, 16u64);
        nat_add(a2478, a2479)?
    }, List::cons({
        let a2482 = {
            let a2480 = f724(v1, 16u64);
            let a2481 = 16u64;
            nat_mul(a2480, a2481)?
        };
        let a2483 = f727(v2, 4u64);
        nat_add(a2482, a2483)?
    }, List::<u64>::nil())))
}

pub fn f727(v0: u64, v1: u64) -> u64 {
    let a2484 = v0;
    let a2485 = v1;
    let a2486 = 0u64;
    nat_quot(a2484, a2485, a2486)
}

pub fn f728(v0: i64) -> u64 {
    let m2489 = {
        let a2487 = v0;
        let a2488 = 1i64;
        compare(a2487, a2488)
    };
    if match m2489 {
        Ordering::Less => {
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
        let m2492 = {
            let a2490 = v0;
            let a2491 = 2i64;
            compare(a2490, a2491)
        };
        if match m2492 {
            Ordering::Less => {
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
            let m2495 = {
                let a2493 = v0;
                let a2494 = 3i64;
                compare(a2493, a2494)
            };
            if match m2495 {
                Ordering::Less => {
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
                let m2498 = {
                    let a2496 = v0;
                    let a2497 = 4i64;
                    compare(a2496, a2497)
                };
                if match m2498 {
                    Ordering::Less => {
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
                    let m2501 = {
                        let a2499 = v0;
                        let a2500 = 5i64;
                        compare(a2499, a2500)
                    };
                    if match m2501 {
                        Ordering::Less => {
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
                        let m2504 = {
                            let a2502 = v0;
                            let a2503 = 6i64;
                            compare(a2502, a2503)
                        };
                        if match m2504 {
                            Ordering::Less => {
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
                            let m2507 = {
                                let a2505 = v0;
                                let a2506 = 7i64;
                                compare(a2505, a2506)
                            };
                            if match m2507 {
                                Ordering::Less => {
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
                                let m2510 = {
                                    let a2508 = v0;
                                    let a2509 = 8i64;
                                    compare(a2508, a2509)
                                };
                                if match m2510 {
                                    Ordering::Less => {
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
                                    let m2513 = {
                                        let a2511 = v0;
                                        let a2512 = 9i64;
                                        compare(a2511, a2512)
                                    };
                                    if match m2513 {
                                        Ordering::Less => {
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

pub fn f729(v0: Adt8) -> Adt6 {
    Adt6::C5(f601(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2514) => {
            h2514
        }
    }))
}

pub fn f730(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2515, _, _, _, _, _, _, _) => {
            h2515
        }
    })), {
        let a2533 = f596(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2516, _, _, _, _, _, _) => {
                h2516
            }
        });
        let a2534 = {
            let a2531 = f678(match v0.clone() {
                Adt35::C0(_, _, h2517, _, _, _, _, _) => {
                    h2517
                }
            });
            let a2532 = {
                let a2529 = f596(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2518, _, _, _, _) => {
                        h2518
                    }
                });
                let a2530 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2521 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2520, _, _, _) => {
                            h2520
                        }
                    };
                    match m2521 {
                        Adt5::C0(h2519) => {
                            h2519
                        }
                    }
                })), {
                    let a2527 = f627(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2522, _, _) => {
                            h2522
                        }
                    });
                    let a2528 = {
                        let a2525 = f778(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2523, _) => {
                                h2523
                            }
                        });
                        let a2526 = f779(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2524) => {
                                h2524
                            }
                        });
                        append_list(a2525, a2526)
                    };
                    append_list(a2527, a2528)
                });
                append_list(a2529, a2530)
            };
            append_list(a2531, a2532)
        };
        append_list(a2533, a2534)
    }))
}

pub fn f731(v0: Adt38) -> R<Option<Adt6>> {
    let m2535 = v0;
    match m2535 {
        Adt38::C0(v1) => {
            Ok(Some(f780(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f781(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2536, _, _, _) => {
                    h2536
                }
            }, match v2.clone() {
                Adt42::C0(_, h2537, _, _) => {
                    h2537
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2538, _) => {
                    h2538
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2539) => {
                    h2539
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f781(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2540, _, _, _) => {
                    h2540
                }
            }, match v3.clone() {
                Adt43::C0(_, h2541, _, _) => {
                    h2541
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2542, _) => {
                    h2542
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2543) => {
                    h2543
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f782(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f783(v5)))
        }
    }
}

pub fn f732(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2567 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2544, _, _, _, _, _, _, _, _) => {
                h2544
            }
        })), List::<(Str, Adt6)>::nil());
        let a2568 = {
            let a2565 = f596(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2545, _, _, _, _, _, _, _) => {
                    h2545
                }
            });
            let a2566 = {
                let a2563 = f678(match v0.clone() {
                    Adt44::C0(_, _, h2546, _, _, _, _, _, _) => {
                        h2546
                    }
                });
                let a2564 = {
                    let a2561 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2547, _, _, _, _, _) => {
                            h2547
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2562 = {
                        let a2559 = f596(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2548, _, _, _, _) => {
                                h2548
                            }
                        });
                        let a2560 = {
                            let a2557 = f596(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2549, _, _, _) => {
                                    h2549
                                }
                            });
                            let a2558 = {
                                let a2555 = f784(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2550, _, _) => {
                                        h2550
                                    }
                                });
                                let a2556 = {
                                    let a2553 = f785(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2551, _) => {
                                            h2551
                                        }
                                    })?;
                                    let a2554 = f779(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2552) => {
                                            h2552
                                        }
                                    });
                                    append_list(a2553, a2554)
                                };
                                append_list(a2555, a2556)
                            };
                            append_list(a2557, a2558)
                        };
                        append_list(a2559, a2560)
                    };
                    append_list(a2561, a2562)
                };
                append_list(a2563, a2564)
            };
            append_list(a2565, a2566)
        };
        append_list(a2567, a2568)
    }))
}

pub fn f733(v0: Adt46) -> Adt6 {
    let m2569 = v0;
    match m2569 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2570, _, _, _) => {
                    h2570
                }
            })), {
                let a2574 = f596(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2571, _, _) => {
                        h2571
                    }
                });
                let a2575 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2572, _) => {
                        h2572
                    }
                })), f779(match v1 {
                    Adt47::C0(_, _, _, h2573) => {
                        h2573
                    }
                }));
                append_list(a2574, a2575)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2576, _, _, _) => {
                    h2576
                }
            })), {
                let a2580 = f596(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2577, _, _) => {
                        h2577
                    }
                });
                let a2581 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2578, _) => {
                        h2578
                    }
                })), f779(match v2 {
                    Adt48::C0(_, _, _, h2579) => {
                        h2579
                    }
                }));
                append_list(a2580, a2581)
            }))
        }
    }
}

pub fn f734(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a2602 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h2582, _, _, _, _, _, _, _) => {
                h2582
            }
        })), List::<(Str, Adt6)>::nil());
        let a2603 = {
            let a2600 = f596(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h2583, _, _, _, _, _, _) => {
                    h2583
                }
            });
            let a2601 = {
                let a2598 = f678(match v0.clone() {
                    Adt54::C0(_, _, h2584, _, _, _, _, _) => {
                        h2584
                    }
                });
                let a2599 = {
                    let a2596 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h2585, _, _, _, _) => {
                            h2585
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2597 = {
                        let a2594 = f596(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h2586, _, _, _) => {
                                h2586
                            }
                        });
                        let a2595 = {
                            let a2592 = f596(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h2587, _, _) => {
                                    h2587
                                }
                            });
                            let a2593 = {
                                let a2590 = f784(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h2588, _) => {
                                        h2588
                                    }
                                });
                                let a2591 = f779(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h2589) => {
                                        h2589
                                    }
                                });
                                append_list(a2590, a2591)
                            };
                            append_list(a2592, a2593)
                        };
                        append_list(a2594, a2595)
                    };
                    append_list(a2596, a2597)
                };
                append_list(a2598, a2599)
            };
            append_list(a2600, a2601)
        };
        append_list(a2602, a2603)
    })
}

pub fn f735(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2619 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2604, _, _, _, _, _) => {
                h2604
            }
        })), List::<(Str, Adt6)>::nil());
        let a2620 = {
            let a2617 = f596(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2605, _, _, _, _) => {
                    h2605
                }
            });
            let a2618 = {
                let a2615 = f678(match v0.clone() {
                    Adt56::C0(_, _, h2606, _, _, _) => {
                        h2606
                    }
                });
                let a2616 = {
                    let a2613 = f596(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h2607, _, _) => {
                            h2607
                        }
                    });
                    let a2614 = {
                        let a2611 = {
                            let m2609 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h2608, _) => {
                                    h2608
                                }
                            };
                            match m2609 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f786(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2612 = f779(match v0 {
                            Adt56::C0(_, _, _, _, _, h2610) => {
                                h2610
                            }
                        });
                        append_list(a2611, a2612)
                    };
                    append_list(a2613, a2614)
                };
                append_list(a2615, a2616)
            };
            append_list(a2617, a2618)
        };
        append_list(a2619, a2620)
    })
}

pub fn f736(v0: Adt59) -> R<Option<Adt6>> {
    let m2622 = f731(match v0.clone() {
        Adt59::C0(_, h2621) => {
            h2621
        }
    })?;
    match m2622 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f787(match v0 {
                Adt59::C0(h2623, _) => {
                    h2623
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f737(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f600(Str::lit("params"), v1)))
}

pub fn f738(v0: Adt65) -> Adt6 {
    let m2624 = v0;
    match m2624 {
        Adt65::C0(v1) => {
            f788(v1)
        }
        Adt65::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt68::C0(h2625, _) => {
                    h2625
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt68::C0(_, h2626) => {
                    h2626
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f739(v0: Adt69) -> R<Option<Adt6>> {
    let m2628 = f789(match v0.clone() {
        Adt69::C0(h2627, _, _, _, _, _, _, _, _, _) => {
            h2627
        }
    })?;
    match m2628 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2630 = f790(v0.clone());
                let a2631 = List::cons((Str::lit("maxTokens"), f145(match v0.clone() {
                    Adt69::C0(_, _, _, _, _, h2629, _, _, _, _) => {
                        h2629
                    }
                })?), f791(v0));
                append_list(a2630, a2631)
            }))))
        }
    }
}

pub fn f740(v0: List<Adt2>) -> List<Adt6> {
    f469(f792(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f741(v0: List<u64>) -> List<u64> {
    let a2634 = f793();
    let a2635 = {
        let a2632 = v0;
        let a2633 = f794();
        append_list(a2632, a2633)
    };
    append_list(a2634, a2635)
}

pub fn f742(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f795(v0, 0u64, v1)
}

pub fn f743(p2642: List<u64>) -> bool {
    let m2641 = p2642;
    {
        let mut m2641 = m2641;
        loop {
            let step = {
                let v0 = m2641;
                let m2636 = v0;
                match m2636.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2637 = v1;
                        let a2638 = 48u64;
                        if if nat_eq(a2637, a2638) {
                            true
                        } else {
                            let a2639 = v1;
                            let a2640 = 46u64;
                            nat_eq(a2639, a2640)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f744(v0: Adt138) -> bool {
    let m2643 = v0;
    match m2643 {
        Adt138::C0 => {
            true
        }
        Adt138::C1 => {
            false
        }
        Adt138::C2 => {
            false
        }
        Adt138::C3 => {
            false
        }
        Adt138::C4 => {
            false
        }
        Adt138::C5 => {
            false
        }
        Adt138::C6 => {
            false
        }
        Adt138::C7 => {
            false
        }
        Adt138::C8 => {
            false
        }
        Adt138::C9 => {
            false
        }
        Adt138::C10 => {
            false
        }
        Adt138::C11 => {
            false
        }
    }
}

pub fn f745(v0: Adt14) -> Adt138 {
    let m2644 = v0;
    match m2644 {
        Adt14::C0(_) => {
            Adt138::C0
        }
        Adt14::C1(_) => {
            Adt138::C1
        }
        Adt14::C2(_) => {
            Adt138::C2
        }
        Adt14::C3(_) => {
            Adt138::C3
        }
        Adt14::C4(_) => {
            Adt138::C4
        }
        Adt14::C5(_) => {
            Adt138::C5
        }
        Adt14::C6(_) => {
            Adt138::C6
        }
        Adt14::C7(_) => {
            Adt138::C7
        }
        Adt14::C8(_) => {
            Adt138::C8
        }
        Adt14::C9(_) => {
            Adt138::C9
        }
        Adt14::C10(_) => {
            Adt138::C10
        }
        Adt14::C11(_) => {
            Adt138::C11
        }
    }
}

pub fn f746(v0: Adt0, v1: Adt32) -> Adt128 {
    Adt128::C0(v1, Adt129::C0(Some(match v0 {
        Adt0::C0(h2645, _, _, _, _, _, _) => {
            h2645
        }
    }), None::<Adt5>), List::<Adt130>::nil())
}

pub fn f747(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2646, _, _, _) => {
            h2646
        }
    }, f343(), match v0.clone() {
        Adt0::C0(_, _, h2647, _, _, _, _) => {
            h2647
        }
    }, match v0 {
        Adt0::C0(_, h2648, _, _, _, _, _) => {
            h2648
        }
    })
}

pub fn f748(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt128, Adt131> {
    let m2649 = v2;
    match m2649 {
        Ok(v4) => {
            if f796(f745(v1.clone()), v4.clone()) {
                f797(v0, v1, v4)
            } else {
                Err::<Adt128, Adt131>(Adt131::C4(f798()))
            }
        }
        Err(v3) => {
            Err::<Adt128, Adt131>(Adt131::C4(v3))
        }
    }
}

pub fn f749(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14) -> R<Result<Adt32, Adt84>> {
    f799(v0.clone(), v1, v2, v3, v4.clone(), f800(v0, v4)?)
}

pub fn f750(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt132> {
    let m2650 = f186(v0.clone(), v1);
    match m2650 {
        Adt106::C0 => {
            Ok::<Option<Str>, Adt132>(None::<Str>)
        }
        Adt106::C1(v2) => {
            let m2651 = f265(v2);
            match m2651 {
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

pub fn f751(v0: Str, v1: Adt5) -> Result<Str, Adt132> {
    let m2652 = f186(v0.clone(), v1);
    match m2652 {
        Adt106::C0 => {
            Err::<Str, Adt132>(Adt132::C7(v0))
        }
        Adt106::C1(v2) => {
            let m2653 = f265(v2);
            match m2653 {
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

pub fn f752(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt132> {
    let m2654 = f186(v0.clone(), v1);
    match m2654 {
        Adt106::C0 => {
            Ok::<Option<Adt5>, Adt132>(None::<Adt5>)
        }
        Adt106::C1(v2) => {
            let m2655 = f187(v2);
            match m2655 {
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

pub fn f753(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt132> {
    let m2656 = f186(v0.clone(), v1);
    match m2656 {
        Adt106::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt132>(None::<List<(Str, Str)>>)
        }
        Adt106::C1(v2) => {
            let m2657 = f187(v2);
            match m2657 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    let m2659 = f801(v0, match v3 {
                        Adt5::C0(h2658) => {
                            h2658
                        }
                    });
                    match m2659 {
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

pub fn f754(v0: Str, v1: Adt5) -> Result<Adt5, Adt132> {
    let m2660 = f186(v0.clone(), v1);
    match m2660 {
        Adt106::C0 => {
            Err::<Adt5, Adt132>(Adt132::C7(v0))
        }
        Adt106::C1(v2) => {
            let m2661 = f187(v2);
            match m2661 {
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

pub fn f755(v0: Adt5) -> Result<Adt25, Adt132> {
    let m2662 = f751(Str::lit("type"), v0.clone());
    match m2662 {
        Ok(v1) => {
            let a2663 = v1.clone();
            let a2664 = Str::lit("ref/prompt");
            if equal(a2663, a2664) {
                let m2665 = f751(Str::lit("name"), v0.clone());
                match m2665 {
                    Ok(v2) => {
                        let m2666 = f750(Str::lit("title"), v0);
                        match m2666 {
                            Ok(v3) => {
                                Ok::<Adt25, Adt132>(Adt25::C0(Adt26::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt25, Adt132>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt25, Adt132>(v5)
                    }
                }
            } else {
                let a2667 = v1;
                let a2668 = Str::lit("ref/resource");
                if equal(a2667, a2668) {
                    let m2669 = f751(Str::lit("uri"), v0);
                    match m2669 {
                        Ok(v6) => {
                            Ok::<Adt25, Adt132>(Adt25::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt25, Adt132>(v7)
                        }
                    }
                } else {
                    Err::<Adt25, Adt132>(Adt132::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt25, Adt132>(v8)
        }
    }
}

pub fn f756(v0: Adt5) -> Result<Adt27, Adt132> {
    let m2670 = f751(Str::lit("name"), v0.clone());
    match m2670 {
        Ok(v1) => {
            let m2671 = f751(Str::lit("value"), v0);
            match m2671 {
                Ok(v2) => {
                    Ok::<Adt27, Adt132>(Adt27::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt27, Adt132>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt27, Adt132>(v4)
        }
    }
}

pub fn f757(v0: Adt5) -> Result<Option<Adt28>, Adt132> {
    let m2672 = f186(Str::lit("context"), v0);
    match m2672 {
        Adt106::C0 => {
            Ok::<Option<Adt28>, Adt132>(None::<Adt28>)
        }
        Adt106::C1(v1) => {
            let m2673 = f187(v1);
            match m2673 {
                None => {
                    Err::<Option<Adt28>, Adt132>(Adt132::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2674 = f753(Str::lit("arguments"), v2);
                    match m2674 {
                        Ok(v3) => {
                            Ok::<Option<Adt28>, Adt132>(Some(Adt28::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt28>, Adt132>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f758(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt131> {
    if v0 {
        None::<Adt131>
    } else {
        Some(Adt131::C1(v1, v2))
    }
}

pub fn f759(v0: Str, p2679: List<Str>) -> bool {
    let m2678 = p2679;
    {
        let mut m2678 = m2678;
        loop {
            let step = {
                let v1 = m2678;
                let m2675 = v1;
                match m2675.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2676 = v2;
                        let a2677 = v0.clone();
                        if equal(a2676, a2677) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f760(v0: Option<Adt112>, v1: List<Adt114>, v2: Adt5) -> R<Option<Adt112>> {
    let m2680 = v0;
    match m2680 {
        None => {
            f802(v1, f803(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f761(v0: Option<Str>, v1: Adt144, v2: Adt5) -> Option<Adt112> {
    let m2681 = v1.clone();
    match m2681 {
        Adt144::C0 => {
            None::<Adt112>
        }
        Adt144::C1 => {
            f804(v0, f805(v1, v2))
        }
        Adt144::C2 => {
            f804(v0, f805(v1, v2))
        }
        Adt144::C3 => {
            f804(v0, f805(v1, v2))
        }
    }
}

pub fn f762(v0: Adt106, v1: Adt106) -> Adt151 {
    let m2682 = v0;
    match m2682 {
        Adt106::C0 => {
            Adt151::C1
        }
        Adt106::C1(_) => {
            let m2683 = v1;
            match m2683 {
                Adt106::C0 => {
                    Adt151::C2
                }
                Adt106::C1(_) => {
                    Adt151::C0
                }
            }
        }
    }
}

pub fn f763(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt132> {
    let m2684 = f265(v1);
    match m2684 {
        None => {
            Err::<Adt16, Adt132>(Adt132::C5)
        }
        Some(v3) => {
            let m2685 = f806(v2);
            match m2685 {
                None => {
                    Err::<Adt16, Adt132>(Adt132::C6)
                }
                Some(v4) => {
                    let m2686 = f752(f807(), v0.clone());
                    match m2686 {
                        Ok(v6) => {
                            let m2687 = f808(v0.clone());
                            match m2687 {
                                Ok(v8) => {
                                    let m2688 = f809(v0.clone());
                                    match m2688 {
                                        Ok(v10) => {
                                            Ok::<Adt16, Adt132>(Adt16::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt16, Adt132>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt16, Adt132>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt132>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f764(p2692: List<Adt6>) -> bool {
    let m2691 = p2692;
    {
        let mut m2691 = m2691;
        loop {
            let step = {
                let v0 = m2691;
                let m2689 = v0;
                match m2689.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2690 = f265(v1);
                        match m2690 {
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
                Ok(next) => { m2691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f765(p2696: List<Adt6>) -> bool {
    let m2695 = p2696;
    {
        let mut m2695 = m2695;
        loop {
            let step = {
                let v0 = m2695;
                let m2693 = v0;
                match m2693.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2694 = f265(v1);
                        match m2694 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f568(v3) {
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
                Ok(next) => { m2695 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f766(v0: Str) -> Option<Adt13> {
    let a2697 = v0.clone();
    let a2698 = Str::lit("string");
    if equal(a2697, a2698) {
        Some(Adt13::C0)
    } else {
        let a2699 = v0.clone();
        let a2700 = Str::lit("integer");
        if equal(a2699, a2700) {
            Some(Adt13::C1)
        } else {
            let a2701 = v0;
            let a2702 = Str::lit("boolean");
            if equal(a2701, a2702) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f767(p2711: List<u64>, p2712: List<u64>) -> R<List<u64>> {
    let m2710 = (p2711, p2712);
    {
        let mut m2710 = m2710;
        loop {
            let step = {
                let (v0, v1) = m2710;
                let m2703 = v0;
                match m2703.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2704 = 65u64;
                        let a2705 = v2;
                        if if nat_le(a2704, a2705) {
                            let a2706 = v2;
                            let a2707 = 90u64;
                            nat_le(a2706, a2707)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2708 = v2;
                                let a2709 = 32u64;
                                nat_add(a2708, a2709)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f768(p2719: List<u64>, p2720: List<u64>) -> bool {
    let m2718 = (p2719, p2720);
    {
        let mut m2718 = m2718;
        loop {
            let step = {
                let (v0, v1) = m2718;
                let m2713 = v0;
                match m2713.uncons() {
                    None => {
                        let m2714 = v1;
                        match m2714.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2715 = v1;
                        match m2715.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2716 = v4;
                                let a2717 = v6;
                                if nat_eq(a2716, a2717) {
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
                Ok(next) => { m2718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f769(v0: Adt6) -> u64 {
    let m2721 = v0;
    match m2721 {
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

pub fn f770(v0: Adt150) -> bool {
    let m2722 = v0;
    match m2722 {
        Adt150::C0 => {
            false
        }
        Adt150::C1 => {
            true
        }
        Adt150::C2 => {
            false
        }
    }
}

pub fn f771(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt152 {
    let m2723 = v0;
    match m2723 {
        None => {
            Adt152::C0
        }
        Some(v2) => {
            Adt152::C1({
                let a2724 = v2;
                let a2725 = v1;
                append_list(a2724, a2725)
            })
        }
    }
}

pub fn f772(p2730: List<Adt6>, p2731: List<Adt6>, p2732: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2729 = (p2730, (p2731, p2732));
    {
        let mut m2729 = m2729;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2729;
                let m2726 = v0;
                match m2726.uncons() {
                    None => {
                        let m2727 = v1;
                        match m2727.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2728 = v1;
                        match m2728.uncons() {
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
                Ok(next) => { m2729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f773(p2738: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2739: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2737 = (p2738, p2739);
    {
        let mut m2737 = m2737;
        loop {
            let step = {
                let (v0, v2) = m2737;
                let m2733 = v0;
                match m2733.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2735 = f261({
                            let (h2734, _) = v3;
                            h2734
                        }, v1.clone());
                        match m2735 {
                            Adt106::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt106::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2736) = v3;
                                    h2736
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f774(v0: Adt150) -> Adt150 {
    let m2740 = v0;
    match m2740 {
        Adt150::C0 => {
            Adt150::C2
        }
        Adt150::C1 => {
            Adt150::C1
        }
        Adt150::C2 => {
            Adt150::C0
        }
    }
}

pub fn f775(v0: i64, v1: i64) -> R<i64> {
    f315(f240(f777(v0)?, 0i64)?, v1)
}

pub fn f776(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt150> {
    let a2741 = v1;
    let a2742 = v3;
    if nat_lt(a2741, a2742) {
        Ok(f658(f810({
            let a2743 = v3;
            let a2744 = v1;
            nat_sub(a2743, a2744)
        }, v0)?, v2))
    } else {
        let a2745 = v3;
        let a2746 = v1;
        if nat_lt(a2745, a2746) {
            Ok(f658(v0, f810({
                let a2747 = v1;
                let a2748 = v3;
                nat_sub(a2747, a2748)
            }, v2)?))
        } else {
            Ok(f658(v0, v2))
        }
    }
}

pub fn f777(v0: i64) -> R<u64> {
    f811(20u64, v0, 0u64)
}

pub fn f778(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2749 = v0;
    match m2749 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2761 = f596(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2750, _, _, _, _) => {
                        h2750
                    }
                });
                let a2762 = {
                    let a2759 = f601(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2751, _, _, _) => {
                            h2751
                        }
                    });
                    let a2760 = {
                        let a2757 = f601(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2752, _, _) => {
                                h2752
                            }
                        });
                        let a2758 = {
                            let a2755 = f601(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2753, _) => {
                                    h2753
                                }
                            });
                            let a2756 = f601(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2754) => {
                                    h2754
                                }
                            });
                            append_list(a2755, a2756)
                        };
                        append_list(a2757, a2758)
                    };
                    append_list(a2759, a2760)
                };
                append_list(a2761, a2762)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f779(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f627(Str::lit("_meta"), v0)
}

pub fn f780(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2763, _, _) => {
            h2763
        }
    })), {
        let a2766 = f784(match v0.clone() {
            Adt39::C0(_, h2764, _) => {
                h2764
            }
        });
        let a2767 = f779(match v0 {
            Adt39::C0(_, _, h2765) => {
                h2765
            }
        });
        append_list(a2766, a2767)
    })))
}

pub fn f781(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2768 = f784(v3);
        let a2769 = f779(v4);
        append_list(a2768, a2769)
    }))))
}

pub fn f782(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2793 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2770, _, _, _, _, _, _, _, _) => {
                h2770
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2794 = {
            let a2791 = f596(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2771, _, _, _, _, _, _, _) => {
                    h2771
                }
            });
            let a2792 = {
                let a2789 = f678(match v0.clone() {
                    Adt44::C0(_, _, h2772, _, _, _, _, _, _) => {
                        h2772
                    }
                });
                let a2790 = {
                    let a2787 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2773, _, _, _, _, _) => {
                            h2773
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2788 = {
                        let a2785 = f596(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2774, _, _, _, _) => {
                                h2774
                            }
                        });
                        let a2786 = {
                            let a2783 = f596(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2775, _, _, _) => {
                                    h2775
                                }
                            });
                            let a2784 = {
                                let a2781 = f784(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2776, _, _) => {
                                        h2776
                                    }
                                });
                                let a2782 = {
                                    let a2779 = f785(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2777, _) => {
                                            h2777
                                        }
                                    })?;
                                    let a2780 = f779(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2778) => {
                                            h2778
                                        }
                                    });
                                    append_list(a2779, a2780)
                                };
                                append_list(a2781, a2782)
                            };
                            append_list(a2783, a2784)
                        };
                        append_list(a2785, a2786)
                    };
                    append_list(a2787, a2788)
                };
                append_list(a2789, a2790)
            };
            append_list(a2791, a2792)
        };
        append_list(a2793, a2794)
    }))
}

pub fn f783(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f733(match v0.clone() {
        Adt45::C0(h2795, _, _) => {
            h2795
        }
    })), {
        let a2798 = f784(match v0.clone() {
            Adt45::C0(_, h2796, _) => {
                h2796
            }
        });
        let a2799 = f779(match v0 {
            Adt45::C0(_, _, h2797) => {
                h2797
            }
        });
        append_list(a2798, a2799)
    })))
}

pub fn f784(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2800 = v0;
    match m2800 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2806 = f812(match v1.clone() {
                    Adt40::C0(h2801, _, _) => {
                        h2801
                    }
                });
                let a2807 = {
                    let a2804 = f813(match v1.clone() {
                        Adt40::C0(_, h2802, _) => {
                            h2802
                        }
                    });
                    let a2805 = f596(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2803) => {
                            h2803
                        }
                    });
                    append_list(a2804, a2805)
                };
                append_list(a2806, a2807)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f785(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2808 = v0;
    match m2808 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f145(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f786(v0: List<Adt57>) -> List<Adt6> {
    f469(f814(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f787(v0: Adt41) -> Str {
    let m2809 = v0;
    match m2809 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f788(v0: Adt66) -> Adt6 {
    Adt6::C5({
        let a2816 = {
            let m2811 = match v0.clone() {
                Adt66::C0(h2810, _, _) => {
                    h2810
                }
            };
            match m2811 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2817 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt66::C0(_, h2812, _) => {
                h2812
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2815 = match v0 {
                Adt66::C0(_, _, h2814) => {
                    h2814
                }
            };
            match m2815 {
                Adt5::C0(h2813) => {
                    h2813
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2816, a2817)
    })
}

pub fn f789(v0: List<Adt70>) -> R<Option<List<Adt6>>> {
    let m2818 = f815(v0, List::<Adt6>::nil())?;
    match m2818 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f469(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f790(v0: Adt69) -> List<(Str, Adt6)> {
    let a2829 = {
        let m2820 = match v0.clone() {
            Adt69::C0(_, h2819, _, _, _, _, _, _, _, _) => {
                h2819
            }
        };
        match m2820 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f816(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2830 = {
        let a2827 = f596(Str::lit("systemPrompt"), match v0.clone() {
            Adt69::C0(_, _, h2821, _, _, _, _, _, _, _) => {
                h2821
            }
        });
        let a2828 = {
            let a2825 = {
                let m2823 = match v0.clone() {
                    Adt69::C0(_, _, _, h2822, _, _, _, _, _, _) => {
                        h2822
                    }
                };
                match m2823 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f817(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2826 = f818(Str::lit("temperature"), match v0 {
                Adt69::C0(_, _, _, _, h2824, _, _, _, _, _) => {
                    h2824
                }
            });
            append_list(a2825, a2826)
        };
        append_list(a2827, a2828)
    };
    append_list(a2829, a2830)
}

pub fn f791(v0: Adt69) -> List<(Str, Adt6)> {
    let a2842 = {
        let m2832 = match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, h2831, _, _, _) => {
                h2831
            }
        };
        match m2832 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f594(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2843 = {
        let a2840 = f627(Str::lit("metadata"), match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, _, h2833, _, _) => {
                h2833
            }
        });
        let a2841 = {
            let a2838 = {
                let m2835 = match v0.clone() {
                    Adt69::C0(_, _, _, _, _, _, _, _, h2834, _) => {
                        h2834
                    }
                };
                match m2835 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f598(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2839 = {
                let m2837 = match v0 {
                    Adt69::C0(_, _, _, _, _, _, _, _, _, h2836) => {
                        h2836
                    }
                };
                match m2837 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f819(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2838, a2839)
        };
        append_list(a2840, a2841)
    };
    append_list(a2842, a2843)
}

pub fn f792(p2846: List<Adt2>, p2847: List<Adt6>) -> List<Adt6> {
    let m2845 = (p2846, p2847);
    {
        let mut m2845 = m2845;
        loop {
            let step = {
                let (v0, v1) = m2845;
                let m2844 = v0;
                match m2844.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f820(v2), v1)))
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

pub fn f793() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f794() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f795(p2852: List<u64>, p2853: u64, p2854: List<u64>) -> List<u64> {
    let m2851 = (p2852, (p2853, p2854));
    {
        let mut m2851 = m2851;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2851;
                let m2848 = v0;
                match m2848.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f821(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt153 = f822(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt153::C0(h2849, _) => {
                                h2849
                            }
                        }, match v5 {
                            Adt153::C0(_, h2850) => {
                                h2850
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2851 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f796(v0: Adt138, v1: Adt32) -> bool {
    let m2855 = v1;
    match m2855 {
        Adt32::C0(_) => {
            f823(v0, Adt138::C0)
        }
        Adt32::C1(_) => {
            f823(v0, Adt138::C1)
        }
        Adt32::C2(_) => {
            f823(v0, Adt138::C2)
        }
        Adt32::C3(_) => {
            f823(v0, Adt138::C2)
        }
        Adt32::C4(_) => {
            f823(v0, Adt138::C3)
        }
        Adt32::C5(_) => {
            f823(v0, Adt138::C4)
        }
        Adt32::C6(_) => {
            f823(v0, Adt138::C5)
        }
        Adt32::C7(_) => {
            f823(v0, Adt138::C6)
        }
        Adt32::C8(_) => {
            f823(v0, Adt138::C7)
        }
        Adt32::C9(_) => {
            f823(v0, Adt138::C8)
        }
        Adt32::C10(_) => {
            f823(v0, Adt138::C9)
        }
        Adt32::C11 => {
            if f823(v0.clone(), Adt138::C10) {
                true
            } else {
                f823(v0, Adt138::C11)
            }
        }
        Adt32::C12(_) => {
            f824(v0)
        }
        Adt32::C13(_) => {
            f825(v0)
        }
    }
}

pub fn f797(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt128, Adt131> {
    let m2856 = v2.clone();
    match m2856 {
        Adt32::C0(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt128, Adt131>(f829(v0, f827(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt128, Adt131>(f746(v0, v2))
        }
        Adt32::C12(v3) => {
            f826(v0, f827(v1), v3)
        }
        Adt32::C13(v4) => {
            f828(v0, f827(v1), v4)
        }
    }
}

pub fn f798() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f799(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14, v5: Result<Adt14, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m2857 = v5;
    match m2857 {
        Ok(v7) => {
            f830(v0, v4, f831(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt32, Adt84>(v6))
        }
    }
}

pub fn f800(v0: Bytes, v1: Adt14) -> R<Result<Adt14, Adt84>> {
    let m2858 = v1.clone();
    match m2858 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C1(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C2(v2) => {
            f832(v0, v2)
        }
        Adt14::C3(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C4(v3) => {
            f833(v0, v3)
        }
        Adt14::C5(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C6(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C7(v4) => {
            f834(v0, v4)
        }
        Adt14::C8(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C9(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C10(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C11(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
    }
}

pub fn f801(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt132> {
    let m2859 = f835(v0, v1, List::<(Str, Str)>::nil());
    match m2859 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt132>(f836(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt132>(v3)
        }
    }
}

pub fn f802(p2863: List<Adt114>, v1: Adt5) -> R<Option<Adt112>> {
    let m2862 = p2863;
    {
        let mut m2862 = m2862;
        loop {
            let step = {
                let v0 = m2862;
                let m2860 = v0;
                match m2860.uncons() {
                    None => {
                        Err::<List<Adt114>, R<Option<Adt112>>>(Ok(None::<Adt112>))
                    }
                    Some((v2, v3)) => {
                        let m2861 = f837(v2, v1.clone())?;
                        match m2861 {
                            None => {
                                Ok::<List<Adt114>, R<Option<Adt112>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt114>, R<Option<Adt112>>>(Ok(Some(v4)))
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

pub fn f803(v0: Adt5) -> Adt5 {
    let m2864 = f186(Str::lit("arguments"), v0);
    match m2864 {
        Adt106::C0 => {
            f26()
        }
        Adt106::C1(v1) => {
            let m2865 = f187(v1);
            match m2865 {
                None => {
                    f26()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f804(v0: Option<Str>, v1: Option<Str>) -> Option<Adt112> {
    let m2866 = v0;
    match m2866 {
        None => {
            Some(Adt112::C2)
        }
        Some(v2) => {
            let m2867 = v1;
            match m2867 {
                None => {
                    None::<Adt112>
                }
                Some(v3) => {
                    let a2868 = v2;
                    let a2869 = v3;
                    if equal(a2868, a2869) {
                        None::<Adt112>
                    } else {
                        Some(Adt112::C5)
                    }
                }
            }
        }
    }
}

pub fn f805(v0: Adt144, v1: Adt5) -> Option<Str> {
    let m2870 = v0;
    match m2870 {
        Adt144::C0 => {
            None::<Str>
        }
        Adt144::C1 => {
            let m2871 = f186(Str::lit("name"), v1);
            match m2871 {
                Adt106::C0 => {
                    None::<Str>
                }
                Adt106::C1(v2) => {
                    f265(v2)
                }
            }
        }
        Adt144::C2 => {
            let m2872 = f186(Str::lit("uri"), v1);
            match m2872 {
                Adt106::C0 => {
                    None::<Str>
                }
                Adt106::C1(v3) => {
                    f265(v3)
                }
            }
        }
        Adt144::C3 => {
            let m2873 = f186(Str::lit("taskId"), v1);
            match m2873 {
                Adt106::C0 => {
                    None::<Str>
                }
                Adt106::C1(v4) => {
                    f265(v4)
                }
            }
        }
    }
}

pub fn f806(v0: Adt6) -> Option<Adt17> {
    let m2874 = f187(v0);
    match m2874 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2875 = f838(Str::lit("experimental"), v1.clone());
            match m2875 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2876 = f838(Str::lit("roots"), v1.clone());
                    match m2876 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2877 = f838(Str::lit("sampling"), v1.clone());
                            match m2877 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2878 = f838(Str::lit("elicitation"), v1.clone());
                                    match m2878 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2879 = f838(Str::lit("extensions"), v1);
                                            match m2879 {
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

pub fn f807() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f808(v0: Adt5) -> Result<Option<Adt18>, Adt132> {
    let m2880 = f750(f839(), v0);
    match m2880 {
        Ok(v2) => {
            let m2881 = v2;
            match m2881 {
                None => {
                    Ok::<Option<Adt18>, Adt132>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2882 = f840(v3);
                    match m2882 {
                        Adt154::C0 => {
                            Err::<Option<Adt18>, Adt132>(Adt132::C8(f839()))
                        }
                        Adt154::C1(v4) => {
                            Ok::<Option<Adt18>, Adt132>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt18>, Adt132>(v1)
        }
    }
}

pub fn f809(v0: Adt5) -> Result<Option<Adt19>, Adt132> {
    let m2883 = f186(f841(), v0);
    match m2883 {
        Adt106::C0 => {
            Ok::<Option<Adt19>, Adt132>(None::<Adt19>)
        }
        Adt106::C1(v1) => {
            let m2884 = f265(v1.clone());
            match m2884 {
                None => {
                    let m2885 = f657(v1);
                    match m2885 {
                        None => {
                            Err::<Option<Adt19>, Adt132>(Adt132::C8(f841()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt19>, Adt132>(Some(Adt19::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt19>, Adt132>(Some(Adt19::C0(v2)))
                }
            }
        }
    }
}

pub fn f810(p2890: u64, p2891: i64) -> R<i64> {
    let m2889 = (p2890, p2891);
    {
        let mut m2889 = m2889;
        loop {
            let step = {
                let (v0, v1) = m2889;
                let m2886 = v0;
                if m2886 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2886 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2887 = v1;
                        let a2888 = 10i64;
                        int_mul(a2887, a2888)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f811(p2902: u64, p2903: i64, p2904: u64) -> R<u64> {
    let m2901 = (p2902, (p2903, p2904));
    {
        let mut m2901 = m2901;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2901;
                let m2892 = v0;
                if m2892 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2892 - 1;
                    let v4: u64 = {
                        let a2893 = v2;
                        let a2894 = 1u64;
                        nat_add(a2893, a2894)?
                    };
                    let m2897 = {
                        let a2895 = v1;
                        let a2896 = 10i64;
                        compare(a2895, a2896)
                    };
                    if match m2897 {
                        Ordering::Less => {
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
                            let a2898 = v1;
                            let a2899 = 10i64;
                            let a2900 = 0i64;
                            int_quot(a2898, a2899, a2900)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f812(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2905 = v0;
    match m2905 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f842(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f813(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2906 = v0;
    match m2906 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f814(p2909: List<Adt57>, p2910: List<Adt6>) -> List<Adt6> {
    let m2908 = (p2909, p2910);
    {
        let mut m2908 = m2908;
        loop {
            let step = {
                let (v0, v1) = m2908;
                let m2907 = v0;
                match m2907.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f843(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f815(p2914: List<Adt70>, p2915: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2913 = (p2914, p2915);
    {
        let mut m2913 = m2913;
        loop {
            let step = {
                let (v0, v1) = m2913;
                let m2911 = v0;
                match m2911.uncons() {
                    None => {
                        Err::<(List<Adt70>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2912 = f844(v2)?;
                        match m2912 {
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
                Ok(next) => { m2913 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f816(v0: Adt75) -> Adt6 {
    Adt6::C5({
        let a2925 = {
            let m2917 = match v0.clone() {
                Adt75::C0(h2916, _, _, _) => {
                    h2916
                }
            };
            match m2917 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f469(f845(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2926 = {
            let a2923 = f818(Str::lit("costPriority"), match v0.clone() {
                Adt75::C0(_, h2918, _, _) => {
                    h2918
                }
            });
            let a2924 = {
                let a2921 = f818(Str::lit("speedPriority"), match v0.clone() {
                    Adt75::C0(_, _, h2919, _) => {
                        h2919
                    }
                });
                let a2922 = f818(Str::lit("intelligencePriority"), match v0 {
                    Adt75::C0(_, _, _, h2920) => {
                        h2920
                    }
                });
                append_list(a2921, a2922)
            };
            append_list(a2923, a2924)
        };
        append_list(a2925, a2926)
    })
}

pub fn f817(v0: Adt77) -> Str {
    let m2927 = v0;
    match m2927 {
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

pub fn f818(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2928 = v1;
    match m2928 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f819(v0: Adt78) -> List<(Str, Adt6)> {
    let m2930 = match v0 {
        Adt78::C0(h2929) => {
            h2929
        }
    };
    match m2930 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f846(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f820(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2931, _, _, _) => {
            h2931
        }
    })), {
        let a2937 = f596(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2932, _, _) => {
                h2932
            }
        });
        let a2938 = {
            let a2935 = f847(match v0.clone() {
                Adt2::C0(_, _, h2933, _) => {
                    h2933
                }
            });
            let a2936 = f848(match v0 {
                Adt2::C0(_, _, _, h2934) => {
                    h2934
                }
            });
            append_list(a2935, a2936)
        };
        append_list(a2937, a2938)
    }))
}

pub fn f821(v0: u64, v1: List<u64>) -> List<u64> {
    let a2939 = v0;
    let a2940 = 0u64;
    if nat_eq(a2939, a2940) {
        v1
    } else {
        let a2941 = v0;
        let a2942 = 1u64;
        if nat_eq(a2941, a2942) {
            List::cons(194u64, v1)
        } else {
            let a2943 = v0;
            let a2944 = 2u64;
            if nat_eq(a2943, a2944) {
                List::cons(226u64, v1)
            } else {
                let a2945 = v0;
                let a2946 = 3u64;
                if nat_eq(a2945, a2946) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f822(v0: u64, v1: u64, v2: List<u64>) -> Adt153 {
    let a2947 = v0;
    let a2948 = 0u64;
    if nat_eq(a2947, a2948) {
        f849(v1, v2)
    } else {
        let a2949 = v0;
        let a2950 = 1u64;
        if nat_eq(a2949, a2950) {
            let a2951 = 128u64;
            let a2952 = v1;
            if if nat_le(a2951, a2952) {
                let a2953 = v1;
                let a2954 = 159u64;
                nat_le(a2953, a2954)
            } else {
                false
            } {
                Adt153::C0(0u64, f850(v2))
            } else {
                f849(v1, List::cons(194u64, v2))
            }
        } else {
            let a2955 = v0;
            let a2956 = 2u64;
            if nat_eq(a2955, a2956) {
                let a2957 = v1;
                let a2958 = 128u64;
                if nat_eq(a2957, a2958) {
                    Adt153::C0(3u64, v2)
                } else {
                    let a2959 = v1;
                    let a2960 = 129u64;
                    if nat_eq(a2959, a2960) {
                        Adt153::C0(4u64, v2)
                    } else {
                        f849(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2961 = v0;
                let a2962 = 3u64;
                if nat_eq(a2961, a2962) {
                    let a2963 = 170u64;
                    let a2964 = v1;
                    if if nat_le(a2963, a2964) {
                        let a2965 = v1;
                        let a2966 = 174u64;
                        nat_le(a2965, a2966)
                    } else {
                        false
                    } {
                        Adt153::C0(0u64, f850(v2))
                    } else {
                        f849(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2967 = 166u64;
                    let a2968 = v1;
                    if if nat_le(a2967, a2968) {
                        let a2969 = v1;
                        let a2970 = 169u64;
                        nat_le(a2969, a2970)
                    } else {
                        false
                    } {
                        Adt153::C0(0u64, f850(v2))
                    } else {
                        f849(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f823(v0: Adt138, v1: Adt138) -> bool {
    let a2971 = f631(v0);
    let a2972 = f631(v1);
    equal(a2971, a2972)
}

pub fn f824(v0: Adt138) -> bool {
    let m2973 = v0;
    match m2973 {
        Adt138::C0 => {
            false
        }
        Adt138::C1 => {
            false
        }
        Adt138::C2 => {
            true
        }
        Adt138::C3 => {
            false
        }
        Adt138::C4 => {
            true
        }
        Adt138::C5 => {
            false
        }
        Adt138::C6 => {
            false
        }
        Adt138::C7 => {
            true
        }
        Adt138::C8 => {
            false
        }
        Adt138::C9 => {
            false
        }
        Adt138::C10 => {
            false
        }
        Adt138::C11 => {
            false
        }
    }
}

pub fn f825(v0: Adt138) -> bool {
    let m2974 = v0;
    match m2974 {
        Adt138::C0 => {
            false
        }
        Adt138::C1 => {
            false
        }
        Adt138::C2 => {
            true
        }
        Adt138::C3 => {
            false
        }
        Adt138::C4 => {
            false
        }
        Adt138::C5 => {
            false
        }
        Adt138::C6 => {
            false
        }
        Adt138::C7 => {
            false
        }
        Adt138::C8 => {
            false
        }
        Adt138::C9 => {
            false
        }
        Adt138::C10 => {
            false
        }
        Adt138::C11 => {
            false
        }
    }
}

pub fn f826(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt128, Adt131> {
    f851(v0, v2.clone(), f852(match v1 {
        Adt16::C0(_, h2975, _, _, _, _) => {
            h2975
        }
    }, f609(v2)))
}

pub fn f827(v0: Adt14) -> Adt16 {
    let m2976 = v0;
    match m2976 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2977) => {
                    h2977
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2978, _) => {
                    h2978
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2979, _, _, _, _) => {
                    h2979
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2980, _) => {
                    h2980
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2981, _, _, _) => {
                    h2981
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2982, _) => {
                    h2982
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2983, _) => {
                    h2983
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2984, _, _, _, _) => {
                    h2984
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2985, _, _, _) => {
                    h2985
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2986, _) => {
                    h2986
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2987, _, _) => {
                    h2987
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2988, _) => {
                    h2988
                }
            }
        }
    }
}

pub fn f828(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt128, Adt131> {
    f853(v0, v2, f854(match v1 {
        Adt16::C0(_, h2989, _, _, _, _) => {
            h2989
        }
    }))
}

pub fn f829(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt128 {
    Adt128::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2990, _) => {
            h2990
        }
    }), {
        let m2992 = f746(v0, Adt32::C11);
        match m2992 {
            Adt128::C0(_, h2991, _) => {
                h2991
            }
        }
    }, f855(v1, match v2 {
        Adt49::C0(_, h2993) => {
            h2993
        }
    }))
}

pub fn f830(v0: Bytes, v1: Adt14, v2: Result<Adt32, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m2994 = v2;
    match m2994 {
        Ok(v4) => {
            f856(v0, f745(v1.clone()), f857(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f831(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2995 = v3.clone();
    match m2995 {
        Adt14::C0(_) => {
            let c2997: Fn0 = v2;
            c2997.apply(v3)
        }
        Adt14::C1(_) => {
            let c2998: Fn0 = v2;
            c2998.apply(v3)
        }
        Adt14::C2(_) => {
            if f858(v0, v1) {
                let c2996: Fn0 = v2;
                c2996.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2999: Fn0 = v2;
            c2999.apply(v3)
        }
        Adt14::C4(_) => {
            let c3000: Fn0 = v2;
            c3000.apply(v3)
        }
        Adt14::C5(_) => {
            let c3001: Fn0 = v2;
            c3001.apply(v3)
        }
        Adt14::C6(_) => {
            let c3002: Fn0 = v2;
            c3002.apply(v3)
        }
        Adt14::C7(_) => {
            let c3003: Fn0 = v2;
            c3003.apply(v3)
        }
        Adt14::C8(_) => {
            let c3004: Fn0 = v2;
            c3004.apply(v3)
        }
        Adt14::C9(_) => {
            let c3005: Fn0 = v2;
            c3005.apply(v3)
        }
        Adt14::C10(_) => {
            let c3006: Fn0 = v2;
            c3006.apply(v3)
        }
        Adt14::C11(_) => {
            let c3007: Fn0 = v2;
            c3007.apply(v3)
        }
    }
}

pub fn f832(v0: Bytes, v1: Adt21) -> R<Result<Adt14, Adt84>> {
    let m3011 = f859(v0, Str::lit("tools/call"), match v1.clone() {
        Adt21::C0(_, h3008, _, _, _) => {
            h3008
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, _, h3009) => {
            h3009
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, h3010, _) => {
            h3010
        }
    })?;
    match m3011 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C2(Adt21::C0(match v1.clone() {
                Adt21::C0(h3012, _, _, _, _) => {
                    h3012
                }
            }, match v1.clone() {
                Adt21::C0(_, h3013, _, _, _) => {
                    h3013
                }
            }, match v1.clone() {
                Adt21::C0(_, _, h3014, _, _) => {
                    h3014
                }
            }, match v1 {
                Adt21::C0(_, _, _, h3015, _) => {
                    h3015
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f833(v0: Bytes, v1: Adt22) -> R<Result<Adt14, Adt84>> {
    let m3019 = f859(v0, Str::lit("resources/read"), match v1.clone() {
        Adt22::C0(_, h3016, _, _) => {
            h3016
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h3017) => {
            h3017
        }
    }, match v1.clone() {
        Adt22::C0(_, _, h3018, _) => {
            h3018
        }
    })?;
    match m3019 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C4(Adt22::C0(match v1.clone() {
                Adt22::C0(h3020, _, _, _) => {
                    h3020
                }
            }, match v1.clone() {
                Adt22::C0(_, h3021, _, _) => {
                    h3021
                }
            }, match v1 {
                Adt22::C0(_, _, h3022, _) => {
                    h3022
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f834(v0: Bytes, v1: Adt23) -> R<Result<Adt14, Adt84>> {
    let m3026 = f859(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt23::C0(_, h3023, _, _, _) => {
            h3023
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, _, h3024) => {
            h3024
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h3025, _) => {
            h3025
        }
    })?;
    match m3026 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C7(Adt23::C0(match v1.clone() {
                Adt23::C0(h3027, _, _, _, _) => {
                    h3027
                }
            }, match v1.clone() {
                Adt23::C0(_, h3028, _, _, _) => {
                    h3028
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h3029, _, _) => {
                    h3029
                }
            }, match v1 {
                Adt23::C0(_, _, _, h3030, _) => {
                    h3030
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f835(v0: Str, p3036: List<(Str, Adt6)>, p3037: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt132> {
    let m3035 = (p3036, p3037);
    {
        let mut m3035 = m3035;
        loop {
            let step = {
                let (v1, v2) = m3035;
                let m3031 = v1;
                match m3031.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Ok::<List<(Str, Str)>, Adt132>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3033 = f265({
                            let (_, h3032) = v3;
                            h3032
                        });
                        match m3033 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Err::<List<(Str, Str)>, Adt132>(Adt132::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>((v4, List::cons(({
                                    let (h3034, _) = v3;
                                    h3034
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f836(p3040: List<(Str, Str)>, p3041: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3039 = (p3040, p3041);
    {
        let mut m3039 = m3039;
        loop {
            let step = {
                let (v0, v1) = m3039;
                let m3038 = v0;
                match m3038.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3039 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f837(v0: Adt114, v1: Adt5) -> R<Option<Adt112>> {
    let m3043 = f860(match v0.clone() {
        Adt114::C0(_, h3042, _, _) => {
            h3042
        }
    }, v1);
    match m3043 {
        Adt106::C0 => {
            Ok(f861(v0))
        }
        Adt106::C1(v2) => {
            if f862(v2.clone()) {
                Ok(f861(v0))
            } else {
                f863(v0, v2)
            }
        }
    }
}

pub fn f838(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m3044 = f186(v0, v1);
    match m3044 {
        Adt106::C0 => {
            Some(None::<Adt5>)
        }
        Adt106::C1(v2) => {
            let m3045 = f187(v2);
            match m3045 {
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

pub fn f839() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f840(v0: Str) -> Adt154 {
    let a3046 = v0.clone();
    let a3047 = Str::lit("debug");
    if equal(a3046, a3047) {
        Adt154::C1(Adt18::C0)
    } else {
        let a3048 = v0.clone();
        let a3049 = Str::lit("info");
        if equal(a3048, a3049) {
            Adt154::C1(Adt18::C1)
        } else {
            let a3050 = v0.clone();
            let a3051 = Str::lit("notice");
            if equal(a3050, a3051) {
                Adt154::C1(Adt18::C2)
            } else {
                let a3052 = v0.clone();
                let a3053 = Str::lit("warning");
                if equal(a3052, a3053) {
                    Adt154::C1(Adt18::C3)
                } else {
                    let a3054 = v0.clone();
                    let a3055 = Str::lit("error");
                    if equal(a3054, a3055) {
                        Adt154::C1(Adt18::C4)
                    } else {
                        let a3056 = v0.clone();
                        let a3057 = Str::lit("critical");
                        if equal(a3056, a3057) {
                            Adt154::C1(Adt18::C5)
                        } else {
                            let a3058 = v0.clone();
                            let a3059 = Str::lit("alert");
                            if equal(a3058, a3059) {
                                Adt154::C1(Adt18::C6)
                            } else {
                                let a3060 = v0;
                                let a3061 = Str::lit("emergency");
                                if equal(a3060, a3061) {
                                    Adt154::C1(Adt18::C7)
                                } else {
                                    Adt154::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f841() -> Str {
    Str::lit("progressToken")
}

pub fn f842(v0: List<Adt41>) -> List<Adt6> {
    f469(f864(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f843(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a3070 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h3062, _, _, _) => {
                h3062
            }
        })), List::<(Str, Adt6)>::nil());
        let a3071 = {
            let a3068 = f596(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h3063, _, _) => {
                    h3063
                }
            });
            let a3069 = {
                let a3066 = f596(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h3064, _) => {
                        h3064
                    }
                });
                let a3067 = f601(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h3065) => {
                        h3065
                    }
                });
                append_list(a3066, a3067)
            };
            append_list(a3068, a3069)
        };
        append_list(a3070, a3071)
    })
}

pub fn f844(v0: Adt70) -> R<Option<Adt6>> {
    let m3073 = f865(match v0.clone() {
        Adt70::C0(_, h3072, _) => {
            h3072
        }
    })?;
    match m3073 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f787(match v0.clone() {
                Adt70::C0(h3074, _, _) => {
                    h3074
                }
            }))), List::cons((Str::lit("content"), v1), f779(match v0 {
                Adt70::C0(_, _, h3075) => {
                    h3075
                }
            }))))))
        }
    }
}

pub fn f845(p3079: List<Adt76>, p3080: List<Adt6>) -> List<Adt6> {
    let m3078 = (p3079, p3080);
    {
        let mut m3078 = m3078;
        loop {
            let step = {
                let (v0, v1) = m3078;
                let m3076 = v0;
                match m3076.uncons() {
                    None => {
                        Err::<(List<Adt76>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt76>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f596(Str::lit("name"), match v2 {
                            Adt76::C0(h3077) => {
                                h3077
                            }
                        })), v1)))
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

pub fn f846(v0: Adt79) -> Str {
    let m3081 = v0;
    match m3081 {
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

pub fn f847(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3082 = v0;
    match m3082 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f540(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f848(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3083 = v0;
    match m3083 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f866(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f849(v0: u64, v1: List<u64>) -> Adt153 {
    let a3084 = v0;
    let a3085 = 32u64;
    if nat_lt(a3084, a3085) {
        if f867(v0) {
            Adt153::C0(0u64, List::cons(v0, v1))
        } else {
            Adt153::C0(0u64, f850(v1))
        }
    } else {
        let a3086 = v0;
        let a3087 = 127u64;
        if nat_eq(a3086, a3087) {
            Adt153::C0(0u64, f850(v1))
        } else {
            let a3088 = v0;
            let a3089 = 194u64;
            if nat_eq(a3088, a3089) {
                Adt153::C0(1u64, v1)
            } else {
                let a3090 = v0;
                let a3091 = 226u64;
                if nat_eq(a3090, a3091) {
                    Adt153::C0(2u64, v1)
                } else {
                    Adt153::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f850(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f851(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f746(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt84::C8(f868(f609(v1)))))
    }
}

pub fn f852(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a3092 = f869(v1.clone());
    if if if bool_not(a3092) {
        true
    } else {
        f249(match v0.clone() {
            Adt17::C0(_, _, _, h3093, _) => {
                h3093
            }
        })
    } {
        let a3094 = f870(v1.clone());
        if bool_not(a3094) {
            true
        } else {
            f249(match v0.clone() {
                Adt17::C0(_, _, h3095, _, _) => {
                    h3095
                }
            })
        }
    } else {
        false
    } {
        let a3096 = f871(v1);
        if bool_not(a3096) {
            true
        } else {
            f249(match v0 {
                Adt17::C0(_, h3097, _, _, _) => {
                    h3097
                }
            })
        }
    } else {
        false
    }
}

pub fn f853(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f746(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt84::C8(f872())))
    }
}

pub fn f854(v0: Adt17) -> bool {
    f634(match v0 {
        Adt17::C0(_, _, _, _, h3098) => {
            h3098
        }
    }, f635())
}

pub fn f855(v0: Adt16, v1: List<Adt50>) -> List<Adt130> {
    let m3100 = match v0 {
        Adt16::C0(_, _, _, _, h3099, _) => {
            h3099
        }
    };
    match m3100 {
        None => {
            List::<Adt130>::nil()
        }
        Some(v2) => {
            f873(f874(v2, v1, List::<Adt130>::nil()), List::<Adt130>::nil())
        }
    }
}

pub fn f856(v0: Bytes, v1: Adt138, v2: Str, v3: Adt32) -> R<Result<Adt32, Adt84>> {
    let m3101 = v3.clone();
    match m3101 {
        Adt32::C0(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C1(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C2(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C3(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C4(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C5(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C6(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C7(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C8(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C9(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C10(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C11 => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C12(v4) => {
            f875(v0, v1, v2, v4)
        }
        Adt32::C13(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f857(v0: Adt14) -> Str {
    let m3102 = v0;
    match m3102 {
        Adt14::C0(_) => {
            Str::lit("")
        }
        Adt14::C1(_) => {
            Str::lit("")
        }
        Adt14::C2(v1) => {
            match v1 {
                Adt21::C0(_, h3103, _, _, _) => {
                    h3103
                }
            }
        }
        Adt14::C3(_) => {
            Str::lit("")
        }
        Adt14::C4(v2) => {
            match v2 {
                Adt22::C0(_, h3104, _, _) => {
                    h3104
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
                Adt23::C0(_, h3105, _, _, _) => {
                    h3105
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

pub fn f858(v0: u64, v1: u64) -> bool {
    let a3106 = v1;
    let a3107 = v0;
    nat_lt(a3106, a3107)
}

pub fn f859(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt84>> {
    Ok(f876(f877(v0, v1, v2, v3)?, f878(v4)))
}

pub fn f860(p3113: List<Str>, p3114: Adt5) -> Adt106 {
    let m3112 = (p3113, p3114);
    {
        let mut m3112 = m3112;
        loop {
            let step = {
                let (v0, v1) = m3112;
                let m3108 = v0;
                match m3108.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt106>(Adt106::C0)
                    }
                    Some((v2, v3)) => {
                        let m3109 = f186(v2, v1);
                        match m3109 {
                            Adt106::C0 => {
                                Err::<(List<Str>, Adt5), Adt106>(Adt106::C0)
                            }
                            Adt106::C1(v4) => {
                                let m3110 = v3.clone();
                                match m3110.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt106>(Adt106::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3111 = f187(v4);
                                        match m3111 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt106>(Adt106::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt106>((v3, v7))
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
                Ok(next) => { m3112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f861(v0: Adt114) -> Option<Adt112> {
    let m3116 = match v0.clone() {
        Adt114::C0(_, _, _, h3115) => {
            h3115
        }
    };
    match m3116 {
        Adt115::C0 => {
            None::<Adt112>
        }
        Adt115::C1 => {
            Some(Adt112::C10(match v0 {
                Adt114::C0(h3117, _, _, _) => {
                    h3117
                }
            }))
        }
        Adt115::C2(_) => {
            None::<Adt112>
        }
    }
}

pub fn f862(v0: Adt6) -> bool {
    let m3118 = v0;
    match m3118 {
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

pub fn f863(v0: Adt114, v1: Adt6) -> R<Option<Adt112>> {
    let m3120 = match v0.clone() {
        Adt114::C0(_, _, _, h3119) => {
            h3119
        }
    };
    match m3120 {
        Adt115::C0 => {
            Ok(Some(Adt112::C8(match v0 {
                Adt114::C0(h3121, _, _, _) => {
                    h3121
                }
            })))
        }
        Adt115::C1 => {
            Ok(Some(Adt112::C10(match v0 {
                Adt114::C0(h3122, _, _, _) => {
                    h3122
                }
            })))
        }
        Adt115::C2(v2) => {
            if f879(match v0.clone() {
                Adt114::C0(_, _, h3123, _) => {
                    h3123
                }
            }, v2, v1)? {
                Ok(None::<Adt112>)
            } else {
                Ok(Some(Adt112::C9(match v0 {
                    Adt114::C0(h3124, _, _, _) => {
                        h3124
                    }
                })))
            }
        }
    }
}

pub fn f864(p3127: List<Adt41>, p3128: List<Adt6>) -> List<Adt6> {
    let m3126 = (p3127, p3128);
    {
        let mut m3126 = m3126;
        loop {
            let step = {
                let (v0, v1) = m3126;
                let m3125 = v0;
                match m3125.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f787(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f865(v0: Adt71) -> R<Option<Adt6>> {
    let m3129 = v0;
    match m3129 {
        Adt71::C0(v1) => {
            f880(v1)
        }
        Adt71::C1(v2) => {
            let m3130 = f881(v2, List::<Adt6>::nil())?;
            match m3130 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f469(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f866(v0: Adt3) -> Str {
    let m3131 = v0;
    match m3131 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f867(v0: u64) -> bool {
    let a3132 = v0;
    let a3133 = 9u64;
    if nat_eq(a3132, a3133) {
        true
    } else {
        let a3134 = v0;
        let a3135 = 10u64;
        if nat_eq(a3134, a3135) {
            true
        } else {
            let a3136 = v0;
            let a3137 = 13u64;
            nat_eq(a3136, a3137)
        }
    }
}

pub fn f868(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f871(v0.clone()) {
        Some(f26())
    } else {
        None::<Adt5>
    }, if f870(v0.clone()) {
        Some(f26())
    } else {
        None::<Adt5>
    }, if f869(v0) {
        Some(f26())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f869(p3142: List<(Str, Adt64)>) -> bool {
    let m3141 = p3142;
    {
        let mut m3141 = m3141;
        loop {
            let step = {
                let v0 = m3141;
                let m3138 = v0;
                match m3138.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3140 = {
                            let (_, h3139) = v1;
                            h3139
                        };
                        match m3140 {
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
                Ok(next) => { m3141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f870(p3147: List<(Str, Adt64)>) -> bool {
    let m3146 = p3147;
    {
        let mut m3146 = m3146;
        loop {
            let step = {
                let v0 = m3146;
                let m3143 = v0;
                match m3143.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3145 = {
                            let (_, h3144) = v1;
                            h3144
                        };
                        match m3145 {
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
                Ok(next) => { m3146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f871(p3152: List<(Str, Adt64)>) -> bool {
    let m3151 = p3152;
    {
        let mut m3151 = m3151;
        loop {
            let step = {
                let v0 = m3151;
                let m3148 = v0;
                match m3148.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3150 = {
                            let (_, h3149) = v1;
                            h3149
                        };
                        match m3150 {
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
                Ok(next) => { m3151 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f872() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f635(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f873(p3155: List<Adt130>, p3156: List<Adt130>) -> List<Adt130> {
    let m3154 = (p3155, p3156);
    {
        let mut m3154 = m3154;
        loop {
            let step = {
                let (v0, v1) = m3154;
                let m3153 = v0;
                match m3153.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<Adt130>), List<Adt130>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt130>, List<Adt130>), List<Adt130>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f874(v0: Adt19, p3159: List<Adt50>, p3160: List<Adt130>) -> List<Adt130> {
    let m3158 = (p3159, p3160);
    {
        let mut m3158 = m3158;
        loop {
            let step = {
                let (v1, v2) = m3158;
                let m3157 = v1;
                match m3157.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt130>), List<Adt130>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt130>), List<Adt130>>((v4, List::cons(Adt130::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f875(v0: Bytes, v1: Adt138, v2: Str, v3: Adt81) -> R<Result<Adt32, Adt84>> {
    if f824(v1.clone()) {
        if f882(v3.clone()) {
            let m3161 = f883(v0, f631(v1), v2, v3)?;
            match m3161 {
                Ok(v5) => {
                    Ok(Ok::<Adt32, Adt84>(Adt32::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt32, Adt84>(v4))
                }
            }
        } else {
            Ok(Err::<Adt32, Adt84>(Adt84::C13))
        }
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C12(f631(v1))))
    }
}

pub fn f876(v0: Result<Option<Str>, Adt84>, v1: bool) -> Result<Option<Str>, Adt84> {
    let m3162 = v0;
    match m3162 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt84>(v3)
            } else {
                Err::<Option<Str>, Adt84>(Adt84::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt84>(v2)
        }
    }
}

pub fn f877(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt84>> {
    let m3163 = v3;
    match m3163 {
        None => {
            Ok(Ok::<Option<Str>, Adt84>(None::<Str>))
        }
        Some(v4) => {
            Ok(f884(f885(v0, v1, v2, v4)?))
        }
    }
}

pub fn f878(v0: Option<Adt5>) -> bool {
    let m3164 = v0;
    match m3164 {
        None => {
            true
        }
        Some(v1) => {
            f886(match v1 {
                Adt5::C0(h3165) => {
                    h3165
                }
            })
        }
    }
}

pub fn f879(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3166 = v0;
    match m3166 {
        Adt13::C0 => {
            let m3167 = f265(v2);
            match m3167 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3168 = v1;
                    let a3169 = v3;
                    Ok(equal(a3168, a3169))
                }
            }
        }
        Adt13::C1 => {
            f887(v1, v2)
        }
        Adt13::C2 => {
            let m3170 = f316(v2);
            match m3170 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3171 = v1;
                        let a3172 = Str::lit("true");
                        Ok(equal(a3171, a3172))
                    } else {
                        let a3173 = v1;
                        let a3174 = Str::lit("false");
                        Ok(equal(a3173, a3174))
                    }
                }
            }
        }
    }
}

pub fn f880(v0: Adt72) -> R<Option<Adt6>> {
    let m3175 = v0;
    match m3175 {
        Adt72::C0(v1) => {
            Ok(Some(f780(v1)))
        }
        Adt72::C1(v2) => {
            Ok(Some(f781(Str::lit("image"), match v2.clone() {
                Adt42::C0(h3176, _, _, _) => {
                    h3176
                }
            }, match v2.clone() {
                Adt42::C0(_, h3177, _, _) => {
                    h3177
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h3178, _) => {
                    h3178
                }
            }, match v2 {
                Adt42::C0(_, _, _, h3179) => {
                    h3179
                }
            })))
        }
        Adt72::C2(v3) => {
            Ok(Some(f781(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h3180, _, _, _) => {
                    h3180
                }
            }, match v3.clone() {
                Adt43::C0(_, h3181, _, _) => {
                    h3181
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h3182, _) => {
                    h3182
                }
            }, match v3 {
                Adt43::C0(_, _, _, h3183) => {
                    h3183
                }
            })))
        }
        Adt72::C3(v4) => {
            Ok(Some(f888(v4)))
        }
        Adt72::C4(v5) => {
            f889(v5)
        }
    }
}

pub fn f881(p3187: List<Adt72>, p3188: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3186 = (p3187, p3188);
    {
        let mut m3186 = m3186;
        loop {
            let step = {
                let (v0, v1) = m3186;
                let m3184 = v0;
                match m3184.uncons() {
                    None => {
                        Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3185 = f880(v2)?;
                        match m3185 {
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
                Ok(next) => { m3186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f882(v0: Adt81) -> bool {
    let m3189 = v0;
    match m3189 {
        Adt81::C0(v1) => {
            let m3190 = v1;
            match m3190.uncons() {
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

pub fn f883(v0: Bytes, v1: Str, v2: Str, v3: Adt81) -> R<Result<Adt81, Adt84>> {
    let m3191 = v3.clone();
    match m3191 {
        Adt81::C0(_) => {
            Ok(Ok::<Adt81, Adt84>(v3))
        }
        Adt81::C1(v5) => {
            Ok(f890(f891(v0, v1, v2, v5)?, List::<(Str, Adt64)>::nil()))
        }
        Adt81::C2(v6, v7) => {
            Ok(f890(f891(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f884(v0: Option<Str>) -> Result<Option<Str>, Adt84> {
    let m3192 = v0;
    match m3192 {
        None => {
            Err::<Option<Str>, Adt84>(Adt84::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt84>(Some(v1))
        }
    }
}

pub fn f885(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3196 = f892(v0, {
        let a3193 = v1;
        utf8_encode(a3193)
    }, {
        let a3194 = v2;
        utf8_encode(a3194)
    }, {
        let a3195 = v3;
        utf8_encode(a3195)
    })?;
    match m3196 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3197 = v4;
            Ok(utf8_decode(a3197))
        }
    }
}

pub fn f886(p3202: List<(Str, Adt6)>) -> bool {
    let m3201 = p3202;
    {
        let mut m3201 = m3201;
        loop {
            let step = {
                let v0 = m3201;
                let m3198 = v0;
                match m3198.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3200 = f187({
                            let (_, h3199) = v1;
                            h3199
                        });
                        match m3200 {
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
                Ok(next) => { m3201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f887(v0: Str, v1: Adt6) -> R<bool> {
    let m3203 = f573(v1);
    match m3203 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3204 = f409(v2)?;
            match m3204 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3206 = f893(f206({
                        let a3205 = v0;
                        utf8_encode(a3205)
                    })?)?;
                    match m3206 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f555({
                                let a3207 = v3;
                                let a3208 = v4;
                                int_sub(a3207, a3208)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f888(v0: Adt73) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt73::C0(h3209, _, _, _) => {
            h3209
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt73::C0(_, h3210, _, _) => {
            h3210
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3213 = match v0.clone() {
            Adt73::C0(_, _, h3212, _) => {
                h3212
            }
        };
        match m3213 {
            Adt5::C0(h3211) => {
                h3211
            }
        }
    })), f779(match v0 {
        Adt73::C0(_, _, _, h3214) => {
            h3214
        }
    }))))))
}

pub fn f889(v0: Adt74) -> R<Option<Adt6>> {
    let m3216 = f599(match v0.clone() {
        Adt74::C0(_, h3215, _, _, _) => {
            h3215
        }
    })?;
    match m3216 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt74::C0(h3217, _, _, _, _) => {
                    h3217
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3223 = f600(Str::lit("structuredContent"), match v0.clone() {
                    Adt74::C0(_, _, h3218, _, _) => {
                        h3218
                    }
                });
                let a3224 = {
                    let a3221 = f601(Str::lit("isError"), match v0.clone() {
                        Adt74::C0(_, _, _, h3219, _) => {
                            h3219
                        }
                    });
                    let a3222 = f779(match v0 {
                        Adt74::C0(_, _, _, _, h3220) => {
                            h3220
                        }
                    });
                    append_list(a3221, a3222)
                };
                append_list(a3223, a3224)
            }))))))
        }
    }
}

pub fn f890(v0: Option<Str>, v1: List<(Str, Adt64)>) -> Result<Adt81, Adt84> {
    let m3225 = v0;
    match m3225 {
        None => {
            Err::<Adt81, Adt84>(Adt84::C14)
        }
        Some(v2) => {
            let m3226 = v1.clone();
            match m3226.uncons() {
                None => {
                    Ok::<Adt81, Adt84>(Adt81::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt81, Adt84>(Adt81::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f891(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3230 = f894(v0, {
        let a3227 = v1;
        utf8_encode(a3227)
    }, {
        let a3228 = v2;
        utf8_encode(a3228)
    }, {
        let a3229 = v3;
        utf8_encode(a3229)
    })?;
    match m3230 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3231 = v4;
            Ok(utf8_decode(a3231))
        }
    }
}

pub fn f892(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3233 = f895();
    let a3234 = {
        let a3232 = v0.clone();
        length_bytes(a3232)
    };
    if nat_le(a3233, a3234) {
        let a3236 = f896(v3.clone())?;
        let a3237 = {
            let a3235 = v3.clone();
            length_bytes(a3235)
        };
        if nat_eq(a3236, a3237) {
            Ok(None::<Bytes>)
        } else {
            f897(v0, v1, v2, v3.clone(), f896(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f893(v0: List<u64>) -> R<Option<i64>> {
    let m3238 = v0.clone();
    match m3238.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3239 = v1;
            let a3240 = 45u64;
            if nat_eq(a3239, a3240) {
                let m3241 = v2.clone();
                match m3241.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3242 = f898(v2, 0i64)?;
                        match m3242 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3243 = v5;
                                    int_neg(a3243)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f898(v0, 0i64)
            }
        }
    }
}

pub fn f894(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3245 = f895();
    let a3246 = {
        let a3244 = v0.clone();
        length_bytes(a3244)
    };
    if nat_le(a3245, a3246) {
        Ok(Some({
            let a3249 = f899(v3.clone())?;
            let a3250 = {
                let a3247 = Bytes::lit(&[0x2e]);
                let a3248 = f899(f900(v0, v1, v2, v3)?)?;
                append_bytes(a3247, a3248)
            };
            append_bytes(a3249, a3250)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f895() -> u64 {
    32u64
}

pub fn f896(v0: Bytes) -> R<u64> {
    let (_, h3251) = f901(Fn7::F902(v0.clone()), {
        let a3252 = v0.clone();
        length_bytes(a3252)
    }, (0u64, {
        let a3253 = v0;
        length_bytes(a3253)
    }))?;
    Ok(h3251)
}

pub fn f897(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3257 = {
        let a3254 = v3.clone();
        let a3255 = 0u64;
        let a3256 = v4;
        slice_bytes(a3254, a3255, a3256)
    };
    match m3257 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3268 = {
                let a3265 = v3.clone();
                let a3266 = {
                    let a3258 = v4;
                    let a3259 = 1u64;
                    nat_add(a3258, a3259)?
                };
                let a3267 = {
                    let a3263 = {
                        let a3260 = v3;
                        length_bytes(a3260)
                    };
                    let a3264 = {
                        let a3261 = v4;
                        let a3262 = 1u64;
                        nat_add(a3261, a3262)?
                    };
                    nat_sub(a3263, a3264)
                };
                slice_bytes(a3265, a3266, a3267)
            };
            match m3268 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f903(v0, v1, v2, f904(v5)?, f904(v6)?)
                }
            }
        }
    }
}

pub fn f898(p3274: List<u64>, p3275: i64) -> R<Option<i64>> {
    let m3273 = (p3274, p3275);
    {
        let mut m3273 = m3273;
        loop {
            let step = {
                let (v0, v1) = m3273;
                let m3269 = v0;
                match m3269.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3270 = f905(v2);
                        match m3270 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f315({
                                    let a3271 = v1;
                                    let a3272 = 10i64;
                                    int_mul(a3271, a3272)?
                                }, f468(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f899(v0: Bytes) -> R<Bytes> {
    let (_, h3276) = f906(Fn8::F907(v0.clone()), {
        let a3277 = v0;
        length_bytes(a3277)
    }, (0u64, f319()))?;
    Ok(h3276)
}

pub fn f900(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f908(v0, {
        let a3278 = f909(v1, v2);
        let a3279 = v3;
        append_bytes(a3278, a3279)
    })
}

pub fn f901(v0: Fn7, p3283: u64, p3284: (u64, u64)) -> R<(u64, u64)> {
    let m3282 = (p3283, p3284);
    {
        let mut m3282 = m3282;
        loop {
            let step = {
                let (v1, v2) = m3282;
                let m3280 = v1;
                if m3280 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3280 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3281: Fn7 = v0.clone();
                        c3281.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f902(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3286 = {
            let (h3285, _) = v1;
            h3285
        };
        let a3287 = 1u64;
        nat_add(a3286, a3287)?
    }, {
        let a3290 = {
            let (_, h3288) = v1;
            h3288
        };
        let a3291 = {
            let a3289 = v0.clone();
            length_bytes(a3289)
        };
        if nat_eq(a3290, a3291) {
            let a3293 = f120(v0, {
                let (h3292, _) = v1;
                h3292
            })?;
            let a3294 = 46u64;
            if nat_eq(a3293, a3294) {
                let (h3295, _) = v1;
                h3295
            } else {
                let (_, h3296) = v1;
                h3296
            }
        } else {
            let (_, h3297) = v1;
            h3297
        }
    }))
}

pub fn f903(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3298 = v3;
    match m3298 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3299 = v4;
            match m3299 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f910(f900(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f904(v0: Bytes) -> R<Option<Bytes>> {
    let a3304 = {
        let a3301 = {
            let a3300 = v0.clone();
            length_bytes(a3300)
        };
        let a3302 = 2u64;
        let a3303 = 0u64;
        nat_rem(a3301, a3302, a3303)
    };
    let a3305 = 0u64;
    if nat_eq(a3304, a3305) {
        let v1: (u64, (Bytes, u64)) = f911(Fn9::F912(v0.clone()), {
            let a3307 = {
                let a3306 = v0;
                length_bytes(a3306)
            };
            let a3308 = 2u64;
            let a3309 = 0u64;
            nat_quot(a3307, a3308, a3309)
        }, (0u64, (f319(), 0u64)))?;
        let a3312 = {
            let (_, h3310) = {
                let (_, h3311) = v1.clone();
                h3311
            };
            h3310
        };
        let a3313 = 0u64;
        if nat_eq(a3312, a3313) {
            Ok(Some({
                let (h3314, _) = {
                    let (_, h3315) = v1;
                    h3315
                };
                h3314
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f905(v0: u64) -> Option<u64> {
    let a3316 = v0;
    let a3317 = 48u64;
    if if nat_lt(a3316, a3317) {
        true
    } else {
        let a3318 = 57u64;
        let a3319 = v0;
        nat_lt(a3318, a3319)
    } {
        None::<u64>
    } else {
        Some({
            let a3320 = v0;
            let a3321 = 48u64;
            nat_sub(a3320, a3321)
        })
    }
}

pub fn f906(v0: Fn8, p3325: u64, p3326: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3324 = (p3325, p3326);
    {
        let mut m3324 = m3324;
        loop {
            let step = {
                let (v1, v2) = m3324;
                let m3322 = v1;
                if m3322 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3322 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3323: Fn8 = v0.clone();
                        c3323.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f907(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3328 = {
            let (h3327, _) = v1.clone();
            h3327
        };
        let a3329 = 1u64;
        nat_add(a3328, a3329)?
    }, {
        let a3332 = {
            let (_, h3330) = v1;
            h3330
        };
        let a3333 = f913(f120(v0, {
            let (h3331, _) = v1;
            h3331
        })?)?;
        append_bytes(a3332, a3333)
    }))
}

pub fn f908(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f914(f915(v0, v1)?)
}

pub fn f909(v0: Bytes, v1: Bytes) -> Bytes {
    let a3340 = f916({
        let a3334 = v0.clone();
        length_bytes(a3334)
    });
    let a3341 = {
        let a3338 = v0;
        let a3339 = {
            let a3336 = f916({
                let a3335 = v1.clone();
                length_bytes(a3335)
            });
            let a3337 = v1;
            append_bytes(a3336, a3337)
        };
        append_bytes(a3338, a3339)
    };
    append_bytes(a3340, a3341)
}

pub fn f910(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3344 = {
        let a3342 = v0.clone();
        length_bytes(a3342)
    };
    let a3345 = {
        let a3343 = v1.clone();
        length_bytes(a3343)
    };
    if nat_eq(a3344, a3345) {
        let a3346 = f917(v0, v1)?;
        let a3347 = 0u64;
        Ok(nat_eq(a3346, a3347))
    } else {
        Ok(false)
    }
}

pub fn f911(v0: Fn9, p3351: u64, p3352: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3350 = (p3351, p3352);
    {
        let mut m3350 = m3350;
        loop {
            let step = {
                let (v1, v2) = m3350;
                let m3348 = v1;
                if m3348 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3348 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3349: Fn9 = v0.clone();
                        c3349.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f912(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f918(v0, v1)
}

pub fn f913(v0: u64) -> R<Bytes> {
    let a3359 = f919({
        let a3353 = v0;
        let a3354 = 16u64;
        let a3355 = 0u64;
        nat_quot(a3353, a3354, a3355)
    })?;
    let a3360 = f919({
        let a3356 = v0;
        let a3357 = 16u64;
        let a3358 = 0u64;
        nat_rem(a3356, a3357, a3358)
    })?;
    Ok(append_bytes(a3359, a3360))
}

pub fn f914(v0: Adt155) -> R<Bytes> {
    let a3381 = f920(match v0.clone() {
        Adt155::C0(h3361, _, _, _, _, _, _, _) => {
            h3361
        }
    })?;
    let a3382 = {
        let a3379 = f920(match v0.clone() {
            Adt155::C0(_, h3362, _, _, _, _, _, _) => {
                h3362
            }
        })?;
        let a3380 = {
            let a3377 = f920(match v0.clone() {
                Adt155::C0(_, _, h3363, _, _, _, _, _) => {
                    h3363
                }
            })?;
            let a3378 = {
                let a3375 = f920(match v0.clone() {
                    Adt155::C0(_, _, _, h3364, _, _, _, _) => {
                        h3364
                    }
                })?;
                let a3376 = {
                    let a3373 = f920(match v0.clone() {
                        Adt155::C0(_, _, _, _, h3365, _, _, _) => {
                            h3365
                        }
                    })?;
                    let a3374 = {
                        let a3371 = f920(match v0.clone() {
                            Adt155::C0(_, _, _, _, _, h3366, _, _) => {
                                h3366
                            }
                        })?;
                        let a3372 = {
                            let a3369 = f920(match v0.clone() {
                                Adt155::C0(_, _, _, _, _, _, h3367, _) => {
                                    h3367
                                }
                            })?;
                            let a3370 = f920(match v0 {
                                Adt155::C0(_, _, _, _, _, _, _, h3368) => {
                                    h3368
                                }
                            })?;
                            append_bytes(a3369, a3370)
                        };
                        append_bytes(a3371, a3372)
                    };
                    append_bytes(a3373, a3374)
                };
                append_bytes(a3375, a3376)
            };
            append_bytes(a3377, a3378)
        };
        append_bytes(a3379, a3380)
    };
    Ok(append_bytes(a3381, a3382))
}

pub fn f915(v0: Bytes, v1: Bytes) -> R<Adt155> {
    let v2: Adt156 = f921(v0)?;
    let v3: Adt155 = f922(f923(f924(), f925(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f923(f923(f924(), f925(v2, 1549556828u32))?, f926(v3))
}

pub fn f916(v0: u64) -> Bytes {
    let a3408 = f439({
        let a3386 = {
            let a3383 = v0;
            let a3384 = 16777216u64;
            let a3385 = 0u64;
            nat_quot(a3383, a3384, a3385)
        };
        let a3387 = 256u64;
        let a3388 = 0u64;
        nat_rem(a3386, a3387, a3388)
    });
    let a3409 = {
        let a3406 = f439({
            let a3392 = {
                let a3389 = v0;
                let a3390 = 65536u64;
                let a3391 = 0u64;
                nat_quot(a3389, a3390, a3391)
            };
            let a3393 = 256u64;
            let a3394 = 0u64;
            nat_rem(a3392, a3393, a3394)
        });
        let a3407 = {
            let a3404 = f439({
                let a3398 = {
                    let a3395 = v0;
                    let a3396 = 256u64;
                    let a3397 = 0u64;
                    nat_quot(a3395, a3396, a3397)
                };
                let a3399 = 256u64;
                let a3400 = 0u64;
                nat_rem(a3398, a3399, a3400)
            });
            let a3405 = f439({
                let a3401 = v0;
                let a3402 = 256u64;
                let a3403 = 0u64;
                nat_rem(a3401, a3402, a3403)
            });
            append_bytes(a3404, a3405)
        };
        append_bytes(a3406, a3407)
    };
    append_bytes(a3408, a3409)
}

pub fn f917(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3410) = f901(Fn7::F927(v0.clone(), v1), {
        let a3411 = v0;
        length_bytes(a3411)
    }, (0u64, 0u64))?;
    Ok(h3410)
}

pub fn f918(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f928(f120(v0.clone(), {
        let (h3412, _) = v1.clone();
        h3412
    })?);
    let v3: u64 = f928(f120(v0, {
        let a3414 = {
            let (h3413, _) = v1.clone();
            h3413
        };
        let a3415 = 1u64;
        nat_add(a3414, a3415)?
    })?);
    let a3416 = v2;
    let a3417 = 16u64;
    if if nat_lt(a3416, a3417) {
        let a3418 = v3;
        let a3419 = 16u64;
        nat_lt(a3418, a3419)
    } else {
        false
    } {
        Ok(({
            let a3421 = {
                let (h3420, _) = v1.clone();
                h3420
            };
            let a3422 = 2u64;
            nat_add(a3421, a3422)?
        }, ({
            let a3429 = {
                let (h3423, _) = {
                    let (_, h3424) = v1.clone();
                    h3424
                };
                h3423
            };
            let a3430 = f439({
                let a3427 = {
                    let a3425 = v2;
                    let a3426 = 16u64;
                    nat_mul(a3425, a3426)?
                };
                let a3428 = v3;
                nat_add(a3427, a3428)?
            });
            append_bytes(a3429, a3430)
        }, {
            let (_, h3431) = {
                let (_, h3432) = v1;
                h3432
            };
            h3431
        })))
    } else {
        Ok(({
            let a3434 = {
                let (h3433, _) = v1.clone();
                h3433
            };
            let a3435 = 2u64;
            nat_add(a3434, a3435)?
        }, ({
            let (h3436, _) = {
                let (_, h3437) = v1.clone();
                h3437
            };
            h3436
        }, {
            let a3440 = {
                let (_, h3438) = {
                    let (_, h3439) = v1;
                    h3439
                };
                h3438
            };
            let a3441 = 1u64;
            nat_add(a3440, a3441)?
        })))
    }
}

pub fn f919(v0: u64) -> R<Bytes> {
    Ok(f439({
        let a3442 = v0;
        let a3443 = 10u64;
        if nat_lt(a3442, a3443) {
            let a3444 = 48u64;
            let a3445 = v0;
            nat_add(a3444, a3445)?
        } else {
            let a3446 = 87u64;
            let a3447 = v0;
            nat_add(a3446, a3447)?
        }
    }))
}

pub fn f920(v0: u32) -> R<Bytes> {
    let a3452 = f929(v0, 24u32)?;
    let a3453 = {
        let a3450 = f929(v0, 16u32)?;
        let a3451 = {
            let a3448 = f929(v0, 8u32)?;
            let a3449 = f929(v0, 0u32)?;
            append_bytes(a3448, a3449)
        };
        append_bytes(a3450, a3451)
    };
    Ok(append_bytes(a3452, a3453))
}

pub fn f921(v0: Bytes) -> R<Adt156> {
    let a3455 = f930();
    let a3456 = {
        let a3454 = v0.clone();
        length_bytes(a3454)
    };
    if nat_lt(a3455, a3456) {
        Ok(f931(f932(v0)?))
    } else {
        f933(v0)
    }
}

pub fn f922(v0: Adt155, v1: u64, v2: Bytes) -> R<Adt155> {
    let v3: u64 = {
        let a3457 = v2.clone();
        length_bytes(a3457)
    };
    let v4: u64 = {
        let a3463 = {
            let a3460 = {
                let a3458 = v3;
                let a3459 = 72u64;
                nat_add(a3458, a3459)?
            };
            let a3461 = 64u64;
            let a3462 = 0u64;
            nat_quot(a3460, a3461, a3462)
        };
        let a3464 = 64u64;
        nat_mul(a3463, a3464)?
    };
    let (_, h3465) = f934(Fn10::F935(v2, v1, v3, v4), {
        let a3466 = v4;
        let a3467 = 64u64;
        let a3468 = 0u64;
        nat_quot(a3466, a3467, a3468)
    }, (0u64, v0))?;
    Ok(h3465)
}

pub fn f923(v0: Adt155, v1: Adt156) -> R<Adt155> {
    let v2: Adt157 = f936(Fn11::F937, 64u64, Adt157::C0(match v0.clone() {
        Adt155::C0(h3469, _, _, _, _, _, _, _) => {
            h3469
        }
    }, match v0.clone() {
        Adt155::C0(_, h3470, _, _, _, _, _, _) => {
            h3470
        }
    }, match v0.clone() {
        Adt155::C0(_, _, h3471, _, _, _, _, _) => {
            h3471
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, h3472, _, _, _, _) => {
            h3472
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, h3473, _, _, _) => {
            h3473
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, _, h3474, _, _) => {
            h3474
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, _, _, h3475, _) => {
            h3475
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, _, _, _, h3476) => {
            h3476
        }
    }, v1, 0u64))?;
    Ok(Adt155::C0(f938(match v0.clone() {
        Adt155::C0(h3477, _, _, _, _, _, _, _) => {
            h3477
        }
    }, match v2.clone() {
        Adt157::C0(h3478, _, _, _, _, _, _, _, _, _) => {
            h3478
        }
    }), f938(match v0.clone() {
        Adt155::C0(_, h3479, _, _, _, _, _, _) => {
            h3479
        }
    }, match v2.clone() {
        Adt157::C0(_, h3480, _, _, _, _, _, _, _, _) => {
            h3480
        }
    }), f938(match v0.clone() {
        Adt155::C0(_, _, h3481, _, _, _, _, _) => {
            h3481
        }
    }, match v2.clone() {
        Adt157::C0(_, _, h3482, _, _, _, _, _, _, _) => {
            h3482
        }
    }), f938(match v0.clone() {
        Adt155::C0(_, _, _, h3483, _, _, _, _) => {
            h3483
        }
    }, match v2.clone() {
        Adt157::C0(_, _, _, h3484, _, _, _, _, _, _) => {
            h3484
        }
    }), f938(match v0.clone() {
        Adt155::C0(_, _, _, _, h3485, _, _, _) => {
            h3485
        }
    }, match v2.clone() {
        Adt157::C0(_, _, _, _, h3486, _, _, _, _, _) => {
            h3486
        }
    }), f938(match v0.clone() {
        Adt155::C0(_, _, _, _, _, h3487, _, _) => {
            h3487
        }
    }, match v2.clone() {
        Adt157::C0(_, _, _, _, _, h3488, _, _, _, _) => {
            h3488
        }
    }), f938(match v0.clone() {
        Adt155::C0(_, _, _, _, _, _, h3489, _) => {
            h3489
        }
    }, match v2.clone() {
        Adt157::C0(_, _, _, _, _, _, h3490, _, _, _) => {
            h3490
        }
    }), f938(match v0 {
        Adt155::C0(_, _, _, _, _, _, _, h3491) => {
            h3491
        }
    }, match v2 {
        Adt157::C0(_, _, _, _, _, _, _, h3492, _, _) => {
            h3492
        }
    })))
}

pub fn f924() -> Adt155 {
    Adt155::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f925(v0: Adt156, v1: u32) -> Adt156 {
    Adt156::C0({
        let a3494 = match v0.clone() {
            Adt156::C0(h3493, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3493
            }
        };
        let a3495 = v1;
        fixed_u32::bit_xor(a3494, a3495)
    }, {
        let a3497 = match v0.clone() {
            Adt156::C0(_, h3496, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3496
            }
        };
        let a3498 = v1;
        fixed_u32::bit_xor(a3497, a3498)
    }, {
        let a3500 = match v0.clone() {
            Adt156::C0(_, _, h3499, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3499
            }
        };
        let a3501 = v1;
        fixed_u32::bit_xor(a3500, a3501)
    }, {
        let a3503 = match v0.clone() {
            Adt156::C0(_, _, _, h3502, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3502
            }
        };
        let a3504 = v1;
        fixed_u32::bit_xor(a3503, a3504)
    }, {
        let a3506 = match v0.clone() {
            Adt156::C0(_, _, _, _, h3505, _, _, _, _, _, _, _, _, _, _, _) => {
                h3505
            }
        };
        let a3507 = v1;
        fixed_u32::bit_xor(a3506, a3507)
    }, {
        let a3509 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, h3508, _, _, _, _, _, _, _, _, _, _) => {
                h3508
            }
        };
        let a3510 = v1;
        fixed_u32::bit_xor(a3509, a3510)
    }, {
        let a3512 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, h3511, _, _, _, _, _, _, _, _, _) => {
                h3511
            }
        };
        let a3513 = v1;
        fixed_u32::bit_xor(a3512, a3513)
    }, {
        let a3515 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, _, h3514, _, _, _, _, _, _, _, _) => {
                h3514
            }
        };
        let a3516 = v1;
        fixed_u32::bit_xor(a3515, a3516)
    }, {
        let a3518 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, _, _, h3517, _, _, _, _, _, _, _) => {
                h3517
            }
        };
        let a3519 = v1;
        fixed_u32::bit_xor(a3518, a3519)
    }, {
        let a3521 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, _, _, _, h3520, _, _, _, _, _, _) => {
                h3520
            }
        };
        let a3522 = v1;
        fixed_u32::bit_xor(a3521, a3522)
    }, {
        let a3524 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, _, _, _, _, h3523, _, _, _, _, _) => {
                h3523
            }
        };
        let a3525 = v1;
        fixed_u32::bit_xor(a3524, a3525)
    }, {
        let a3527 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, h3526, _, _, _, _) => {
                h3526
            }
        };
        let a3528 = v1;
        fixed_u32::bit_xor(a3527, a3528)
    }, {
        let a3530 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3529, _, _, _) => {
                h3529
            }
        };
        let a3531 = v1;
        fixed_u32::bit_xor(a3530, a3531)
    }, {
        let a3533 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3532, _, _) => {
                h3532
            }
        };
        let a3534 = v1;
        fixed_u32::bit_xor(a3533, a3534)
    }, {
        let a3536 = match v0.clone() {
            Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3535, _) => {
                h3535
            }
        };
        let a3537 = v1;
        fixed_u32::bit_xor(a3536, a3537)
    }, {
        let a3539 = match v0 {
            Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3538) => {
                h3538
            }
        };
        let a3540 = v1;
        fixed_u32::bit_xor(a3539, a3540)
    })
}

pub fn f926(v0: Adt155) -> Adt156 {
    Adt156::C0(match v0.clone() {
        Adt155::C0(h3541, _, _, _, _, _, _, _) => {
            h3541
        }
    }, match v0.clone() {
        Adt155::C0(_, h3542, _, _, _, _, _, _) => {
            h3542
        }
    }, match v0.clone() {
        Adt155::C0(_, _, h3543, _, _, _, _, _) => {
            h3543
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, h3544, _, _, _, _) => {
            h3544
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, h3545, _, _, _) => {
            h3545
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, _, h3546, _, _) => {
            h3546
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, _, _, h3547, _) => {
            h3547
        }
    }, match v0 {
        Adt155::C0(_, _, _, _, _, _, _, h3548) => {
            h3548
        }
    }, 2147483648u32, f939(), f939(), f939(), f939(), f939(), f939(), 768u32)
}

pub fn f927(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3550 = {
            let (h3549, _) = v2;
            h3549
        };
        let a3551 = 1u64;
        nat_add(a3550, a3551)?
    }, {
        let a3603 = {
            let (_, h3552) = v2;
            h3552
        };
        let a3604 = {
            let v3: u8 = {
                let a3555 = f940(v0, {
                    let (h3553, _) = v2;
                    h3553
                });
                let a3556 = f940(v1, {
                    let (h3554, _) = v2;
                    h3554
                });
                fixed_u8::bit_xor(a3555, a3556)
            };
            let a3601 = {
                let a3577 = {
                    let a3565 = {
                        let a3559 = {
                            let a3557 = v3;
                            let a3558 = 128u8;
                            fixed_u8::bit_and(a3557, a3558)
                        };
                        let a3560 = 128u8;
                        if equal(a3559, a3560) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3566 = {
                        let a3563 = {
                            let a3561 = v3;
                            let a3562 = 64u8;
                            fixed_u8::bit_and(a3561, a3562)
                        };
                        let a3564 = 64u8;
                        if equal(a3563, a3564) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3565, a3566)?
                };
                let a3578 = {
                    let a3575 = {
                        let a3569 = {
                            let a3567 = v3;
                            let a3568 = 32u8;
                            fixed_u8::bit_and(a3567, a3568)
                        };
                        let a3570 = 32u8;
                        if equal(a3569, a3570) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3576 = {
                        let a3573 = {
                            let a3571 = v3;
                            let a3572 = 16u8;
                            fixed_u8::bit_and(a3571, a3572)
                        };
                        let a3574 = 16u8;
                        if equal(a3573, a3574) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3575, a3576)?
                };
                nat_add(a3577, a3578)?
            };
            let a3602 = {
                let a3599 = {
                    let a3587 = {
                        let a3581 = {
                            let a3579 = v3;
                            let a3580 = 8u8;
                            fixed_u8::bit_and(a3579, a3580)
                        };
                        let a3582 = 8u8;
                        if equal(a3581, a3582) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3588 = {
                        let a3585 = {
                            let a3583 = v3;
                            let a3584 = 4u8;
                            fixed_u8::bit_and(a3583, a3584)
                        };
                        let a3586 = 4u8;
                        if equal(a3585, a3586) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3587, a3588)?
                };
                let a3600 = {
                    let a3597 = {
                        let a3591 = {
                            let a3589 = v3;
                            let a3590 = 2u8;
                            fixed_u8::bit_and(a3589, a3590)
                        };
                        let a3592 = 2u8;
                        if equal(a3591, a3592) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3598 = {
                        let a3595 = {
                            let a3593 = v3;
                            let a3594 = 1u8;
                            fixed_u8::bit_and(a3593, a3594)
                        };
                        let a3596 = 1u8;
                        if equal(a3595, a3596) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3597, a3598)?
                };
                nat_add(a3599, a3600)?
            };
            nat_add(a3601, a3602)?
        };
        nat_add(a3603, a3604)?
    }))
}

pub fn f928(v0: u64) -> u64 {
    let a3605 = v0;
    let a3606 = 48u64;
    if nat_lt(a3605, a3606) {
        16u64
    } else {
        let a3607 = v0;
        let a3608 = 58u64;
        if nat_lt(a3607, a3608) {
            let a3609 = v0;
            let a3610 = 48u64;
            nat_sub(a3609, a3610)
        } else {
            let a3611 = v0;
            let a3612 = 97u64;
            if nat_lt(a3611, a3612) {
                16u64
            } else {
                let a3613 = v0;
                let a3614 = 103u64;
                if nat_lt(a3613, a3614) {
                    let a3615 = v0;
                    let a3616 = 87u64;
                    nat_sub(a3615, a3616)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f929(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f439({
        let v2: u32 = {
            let a3617 = f941(v0, v1);
            let a3618 = 255u32;
            fixed_u32::bit_and(a3617, a3618)
        };
        let a3807 = {
            let a3711 = {
                let a3663 = {
                    let a3639 = {
                        let a3627 = {
                            let a3621 = {
                                let a3619 = v2;
                                let a3620 = 2147483648u32;
                                fixed_u32::bit_and(a3619, a3620)
                            };
                            let a3622 = 2147483648u32;
                            if equal(a3621, a3622) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3628 = {
                            let a3625 = {
                                let a3623 = v2;
                                let a3624 = 1073741824u32;
                                fixed_u32::bit_and(a3623, a3624)
                            };
                            let a3626 = 1073741824u32;
                            if equal(a3625, a3626) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3627, a3628)?
                    };
                    let a3640 = {
                        let a3637 = {
                            let a3631 = {
                                let a3629 = v2;
                                let a3630 = 536870912u32;
                                fixed_u32::bit_and(a3629, a3630)
                            };
                            let a3632 = 536870912u32;
                            if equal(a3631, a3632) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3638 = {
                            let a3635 = {
                                let a3633 = v2;
                                let a3634 = 268435456u32;
                                fixed_u32::bit_and(a3633, a3634)
                            };
                            let a3636 = 268435456u32;
                            if equal(a3635, a3636) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3637, a3638)?
                    };
                    nat_add(a3639, a3640)?
                };
                let a3664 = {
                    let a3661 = {
                        let a3649 = {
                            let a3643 = {
                                let a3641 = v2;
                                let a3642 = 134217728u32;
                                fixed_u32::bit_and(a3641, a3642)
                            };
                            let a3644 = 134217728u32;
                            if equal(a3643, a3644) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3650 = {
                            let a3647 = {
                                let a3645 = v2;
                                let a3646 = 67108864u32;
                                fixed_u32::bit_and(a3645, a3646)
                            };
                            let a3648 = 67108864u32;
                            if equal(a3647, a3648) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3649, a3650)?
                    };
                    let a3662 = {
                        let a3659 = {
                            let a3653 = {
                                let a3651 = v2;
                                let a3652 = 33554432u32;
                                fixed_u32::bit_and(a3651, a3652)
                            };
                            let a3654 = 33554432u32;
                            if equal(a3653, a3654) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3660 = {
                            let a3657 = {
                                let a3655 = v2;
                                let a3656 = 16777216u32;
                                fixed_u32::bit_and(a3655, a3656)
                            };
                            let a3658 = 16777216u32;
                            if equal(a3657, a3658) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3659, a3660)?
                    };
                    nat_add(a3661, a3662)?
                };
                nat_add(a3663, a3664)?
            };
            let a3712 = {
                let a3709 = {
                    let a3685 = {
                        let a3673 = {
                            let a3667 = {
                                let a3665 = v2;
                                let a3666 = 8388608u32;
                                fixed_u32::bit_and(a3665, a3666)
                            };
                            let a3668 = 8388608u32;
                            if equal(a3667, a3668) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3674 = {
                            let a3671 = {
                                let a3669 = v2;
                                let a3670 = 4194304u32;
                                fixed_u32::bit_and(a3669, a3670)
                            };
                            let a3672 = 4194304u32;
                            if equal(a3671, a3672) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3673, a3674)?
                    };
                    let a3686 = {
                        let a3683 = {
                            let a3677 = {
                                let a3675 = v2;
                                let a3676 = 2097152u32;
                                fixed_u32::bit_and(a3675, a3676)
                            };
                            let a3678 = 2097152u32;
                            if equal(a3677, a3678) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3684 = {
                            let a3681 = {
                                let a3679 = v2;
                                let a3680 = 1048576u32;
                                fixed_u32::bit_and(a3679, a3680)
                            };
                            let a3682 = 1048576u32;
                            if equal(a3681, a3682) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3683, a3684)?
                    };
                    nat_add(a3685, a3686)?
                };
                let a3710 = {
                    let a3707 = {
                        let a3695 = {
                            let a3689 = {
                                let a3687 = v2;
                                let a3688 = 524288u32;
                                fixed_u32::bit_and(a3687, a3688)
                            };
                            let a3690 = 524288u32;
                            if equal(a3689, a3690) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3696 = {
                            let a3693 = {
                                let a3691 = v2;
                                let a3692 = 262144u32;
                                fixed_u32::bit_and(a3691, a3692)
                            };
                            let a3694 = 262144u32;
                            if equal(a3693, a3694) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3695, a3696)?
                    };
                    let a3708 = {
                        let a3705 = {
                            let a3699 = {
                                let a3697 = v2;
                                let a3698 = 131072u32;
                                fixed_u32::bit_and(a3697, a3698)
                            };
                            let a3700 = 131072u32;
                            if equal(a3699, a3700) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3706 = {
                            let a3703 = {
                                let a3701 = v2;
                                let a3702 = 65536u32;
                                fixed_u32::bit_and(a3701, a3702)
                            };
                            let a3704 = 65536u32;
                            if equal(a3703, a3704) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3705, a3706)?
                    };
                    nat_add(a3707, a3708)?
                };
                nat_add(a3709, a3710)?
            };
            nat_add(a3711, a3712)?
        };
        let a3808 = {
            let a3805 = {
                let a3757 = {
                    let a3733 = {
                        let a3721 = {
                            let a3715 = {
                                let a3713 = v2;
                                let a3714 = 32768u32;
                                fixed_u32::bit_and(a3713, a3714)
                            };
                            let a3716 = 32768u32;
                            if equal(a3715, a3716) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3722 = {
                            let a3719 = {
                                let a3717 = v2;
                                let a3718 = 16384u32;
                                fixed_u32::bit_and(a3717, a3718)
                            };
                            let a3720 = 16384u32;
                            if equal(a3719, a3720) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3721, a3722)?
                    };
                    let a3734 = {
                        let a3731 = {
                            let a3725 = {
                                let a3723 = v2;
                                let a3724 = 8192u32;
                                fixed_u32::bit_and(a3723, a3724)
                            };
                            let a3726 = 8192u32;
                            if equal(a3725, a3726) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3732 = {
                            let a3729 = {
                                let a3727 = v2;
                                let a3728 = 4096u32;
                                fixed_u32::bit_and(a3727, a3728)
                            };
                            let a3730 = 4096u32;
                            if equal(a3729, a3730) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3731, a3732)?
                    };
                    nat_add(a3733, a3734)?
                };
                let a3758 = {
                    let a3755 = {
                        let a3743 = {
                            let a3737 = {
                                let a3735 = v2;
                                let a3736 = 2048u32;
                                fixed_u32::bit_and(a3735, a3736)
                            };
                            let a3738 = 2048u32;
                            if equal(a3737, a3738) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3744 = {
                            let a3741 = {
                                let a3739 = v2;
                                let a3740 = 1024u32;
                                fixed_u32::bit_and(a3739, a3740)
                            };
                            let a3742 = 1024u32;
                            if equal(a3741, a3742) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3743, a3744)?
                    };
                    let a3756 = {
                        let a3753 = {
                            let a3747 = {
                                let a3745 = v2;
                                let a3746 = 512u32;
                                fixed_u32::bit_and(a3745, a3746)
                            };
                            let a3748 = 512u32;
                            if equal(a3747, a3748) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3754 = {
                            let a3751 = {
                                let a3749 = v2;
                                let a3750 = 256u32;
                                fixed_u32::bit_and(a3749, a3750)
                            };
                            let a3752 = 256u32;
                            if equal(a3751, a3752) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3753, a3754)?
                    };
                    nat_add(a3755, a3756)?
                };
                nat_add(a3757, a3758)?
            };
            let a3806 = {
                let a3803 = {
                    let a3779 = {
                        let a3767 = {
                            let a3761 = {
                                let a3759 = v2;
                                let a3760 = 128u32;
                                fixed_u32::bit_and(a3759, a3760)
                            };
                            let a3762 = 128u32;
                            if equal(a3761, a3762) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3768 = {
                            let a3765 = {
                                let a3763 = v2;
                                let a3764 = 64u32;
                                fixed_u32::bit_and(a3763, a3764)
                            };
                            let a3766 = 64u32;
                            if equal(a3765, a3766) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3767, a3768)?
                    };
                    let a3780 = {
                        let a3777 = {
                            let a3771 = {
                                let a3769 = v2;
                                let a3770 = 32u32;
                                fixed_u32::bit_and(a3769, a3770)
                            };
                            let a3772 = 32u32;
                            if equal(a3771, a3772) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3778 = {
                            let a3775 = {
                                let a3773 = v2;
                                let a3774 = 16u32;
                                fixed_u32::bit_and(a3773, a3774)
                            };
                            let a3776 = 16u32;
                            if equal(a3775, a3776) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3777, a3778)?
                    };
                    nat_add(a3779, a3780)?
                };
                let a3804 = {
                    let a3801 = {
                        let a3789 = {
                            let a3783 = {
                                let a3781 = v2;
                                let a3782 = 8u32;
                                fixed_u32::bit_and(a3781, a3782)
                            };
                            let a3784 = 8u32;
                            if equal(a3783, a3784) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3790 = {
                            let a3787 = {
                                let a3785 = v2;
                                let a3786 = 4u32;
                                fixed_u32::bit_and(a3785, a3786)
                            };
                            let a3788 = 4u32;
                            if equal(a3787, a3788) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3789, a3790)?
                    };
                    let a3802 = {
                        let a3799 = {
                            let a3793 = {
                                let a3791 = v2;
                                let a3792 = 2u32;
                                fixed_u32::bit_and(a3791, a3792)
                            };
                            let a3794 = 2u32;
                            if equal(a3793, a3794) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3800 = {
                            let a3797 = {
                                let a3795 = v2;
                                let a3796 = 1u32;
                                fixed_u32::bit_and(a3795, a3796)
                            };
                            let a3798 = 1u32;
                            if equal(a3797, a3798) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3799, a3800)?
                    };
                    nat_add(a3801, a3802)?
                };
                nat_add(a3803, a3804)?
            };
            nat_add(a3805, a3806)?
        };
        nat_add(a3807, a3808)?
    }))
}

pub fn f930() -> u64 {
    64u64
}

pub fn f931(v0: Adt155) -> Adt156 {
    Adt156::C0(match v0.clone() {
        Adt155::C0(h3809, _, _, _, _, _, _, _) => {
            h3809
        }
    }, match v0.clone() {
        Adt155::C0(_, h3810, _, _, _, _, _, _) => {
            h3810
        }
    }, match v0.clone() {
        Adt155::C0(_, _, h3811, _, _, _, _, _) => {
            h3811
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, h3812, _, _, _, _) => {
            h3812
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, h3813, _, _, _) => {
            h3813
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, _, h3814, _, _) => {
            h3814
        }
    }, match v0.clone() {
        Adt155::C0(_, _, _, _, _, _, h3815, _) => {
            h3815
        }
    }, match v0 {
        Adt155::C0(_, _, _, _, _, _, _, h3816) => {
            h3816
        }
    }, f939(), f939(), f939(), f939(), f939(), f939(), f939(), f939())
}

pub fn f932(v0: Bytes) -> R<Adt155> {
    let v1: u64 = {
        let a3817 = v0.clone();
        length_bytes(a3817)
    };
    let v2: u64 = {
        let a3823 = {
            let a3820 = {
                let a3818 = v1;
                let a3819 = 72u64;
                nat_add(a3818, a3819)?
            };
            let a3821 = 64u64;
            let a3822 = 0u64;
            nat_quot(a3820, a3821, a3822)
        };
        let a3824 = 64u64;
        nat_mul(a3823, a3824)?
    };
    let (_, h3825) = f934(Fn10::F942(v0, v1, v2), {
        let a3826 = v2;
        let a3827 = 64u64;
        let a3828 = 0u64;
        nat_quot(a3826, a3827, a3828)
    }, (0u64, f924()))?;
    Ok(h3825)
}

pub fn f933(v0: Bytes) -> R<Adt156> {
    Ok(Adt156::C0(f943(v0.clone(), 0u64)?, f943(v0.clone(), 4u64)?, f943(v0.clone(), 8u64)?, f943(v0.clone(), 12u64)?, f943(v0.clone(), 16u64)?, f943(v0.clone(), 20u64)?, f943(v0.clone(), 24u64)?, f943(v0.clone(), 28u64)?, f943(v0.clone(), 32u64)?, f943(v0.clone(), 36u64)?, f943(v0.clone(), 40u64)?, f943(v0.clone(), 44u64)?, f943(v0.clone(), 48u64)?, f943(v0.clone(), 52u64)?, f943(v0.clone(), 56u64)?, f943(v0, 60u64)?))
}

pub fn f934(v0: Fn10, p3832: u64, p3833: (u64, Adt155)) -> R<(u64, Adt155)> {
    let m3831 = (p3832, p3833);
    {
        let mut m3831 = m3831;
        loop {
            let step = {
                let (v1, v2) = m3831;
                let m3829 = v1;
                if m3829 == 0 {
                    Err::<(u64, (u64, Adt155)), R<(u64, Adt155)>>(Ok(v2))
                } else {
                    let v3 = m3829 - 1;
                    Ok::<(u64, (u64, Adt155)), R<(u64, Adt155)>>((v3, {
                        let c3830: Fn10 = v0.clone();
                        c3830.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3831 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f935(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt155)) -> R<(u64, Adt155)> {
    Ok(({
        let a3835 = {
            let (h3834, _) = v4.clone();
            h3834
        };
        let a3836 = 64u64;
        nat_add(a3835, a3836)?
    }, f923({
        let (_, h3837) = v4;
        h3837
    }, f944(v0, v2, v3, {
        let a3840 = {
            let a3838 = v1;
            let a3839 = v2;
            nat_add(a3838, a3839)?
        };
        let a3841 = 8u64;
        nat_mul(a3840, a3841)?
    }, {
        let (h3842, _) = v4;
        h3842
    })?)?))
}

pub fn f936(v0: Fn11, p3846: u64, p3847: Adt157) -> R<Adt157> {
    let m3845 = (p3846, p3847);
    {
        let mut m3845 = m3845;
        loop {
            let step = {
                let (v1, v2) = m3845;
                let m3843 = v1;
                if m3843 == 0 {
                    Err::<(u64, Adt157), R<Adt157>>(Ok(v2))
                } else {
                    let v3 = m3843 - 1;
                    Ok::<(u64, Adt157), R<Adt157>>((v3, {
                        let c3844: Fn11 = v0.clone();
                        c3844.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f937(v0: Adt157) -> R<Adt157> {
    f945(v0)
}

pub fn f938(v0: u32, v1: u32) -> u32 {
    let m3850 = {
        let a3848 = f946(v0);
        let a3849 = f946(v1);
        fixed_u64::checked_add(a3848, a3849)
    };
    match m3850 {
        None => {
            0u32
        }
        Some(v2) => {
            f947(v2)
        }
    }
}

pub fn f939() -> u32 {
    0u32
}

pub fn f940(v0: Bytes, v1: u64) -> u8 {
    let m3853 = {
        let a3851 = v0;
        let a3852 = v1;
        index_bytes(a3851, a3852)
    };
    match m3853 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f941(v0: u32, v1: u32) -> u32 {
    let m3856 = {
        let a3854 = v0;
        let a3855 = v1;
        fixed_u32::shift_right(a3854, a3855)
    };
    match m3856 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f942(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt155)) -> R<(u64, Adt155)> {
    Ok(({
        let a3858 = {
            let (h3857, _) = v3.clone();
            h3857
        };
        let a3859 = 64u64;
        nat_add(a3858, a3859)?
    }, f923({
        let (_, h3860) = v3;
        h3860
    }, f948(v0, v1, v2, {
        let (h3861, _) = v3;
        h3861
    })?)?))
}

pub fn f943(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f949(f950(v0.clone(), v1), f950(v0.clone(), {
        let a3862 = v1;
        let a3863 = 1u64;
        nat_add(a3862, a3863)?
    }), f950(v0.clone(), {
        let a3864 = v1;
        let a3865 = 2u64;
        nat_add(a3864, a3865)?
    }), f950(v0, {
        let a3866 = v1;
        let a3867 = 3u64;
        nat_add(a3866, a3867)?
    })))
}

pub fn f944(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt156> {
    Ok(Adt156::C0(f951(v0.clone(), v1, v2, v3, v4)?, f951(v0.clone(), v1, v2, v3, {
        let a3868 = v4;
        let a3869 = 4u64;
        nat_add(a3868, a3869)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3870 = v4;
        let a3871 = 8u64;
        nat_add(a3870, a3871)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3872 = v4;
        let a3873 = 12u64;
        nat_add(a3872, a3873)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3874 = v4;
        let a3875 = 16u64;
        nat_add(a3874, a3875)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3876 = v4;
        let a3877 = 20u64;
        nat_add(a3876, a3877)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3878 = v4;
        let a3879 = 24u64;
        nat_add(a3878, a3879)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3880 = v4;
        let a3881 = 28u64;
        nat_add(a3880, a3881)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3882 = v4;
        let a3883 = 32u64;
        nat_add(a3882, a3883)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3884 = v4;
        let a3885 = 36u64;
        nat_add(a3884, a3885)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3886 = v4;
        let a3887 = 40u64;
        nat_add(a3886, a3887)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3888 = v4;
        let a3889 = 44u64;
        nat_add(a3888, a3889)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3890 = v4;
        let a3891 = 48u64;
        nat_add(a3890, a3891)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3892 = v4;
        let a3893 = 52u64;
        nat_add(a3892, a3893)?
    })?, f951(v0.clone(), v1, v2, v3, {
        let a3894 = v4;
        let a3895 = 56u64;
        nat_add(a3894, a3895)?
    })?, f951(v0, v1, v2, v3, {
        let a3896 = v4;
        let a3897 = 60u64;
        nat_add(a3896, a3897)?
    })?))
}

pub fn f945(v0: Adt157) -> R<Adt157> {
    let v1: u32 = f952(match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, h3898) => {
            h3898
        }
    });
    let v2: u32 = f938(match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, h3899, _, _) => {
            h3899
        }
    }, f938(f953(match v0.clone() {
        Adt157::C0(_, _, _, _, h3900, _, _, _, _, _) => {
            h3900
        }
    }), f938(f954(match v0.clone() {
        Adt157::C0(_, _, _, _, h3901, _, _, _, _, _) => {
            h3901
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, h3902, _, _, _, _) => {
            h3902
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, h3903, _, _, _) => {
            h3903
        }
    }), f938(v1, {
        let m3906 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, _, h3905, _) => {
                h3905
            }
        };
        match m3906 {
            Adt156::C0(h3904, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3904
            }
        }
    }))));
    let v3: u32 = f938(f955(match v0.clone() {
        Adt157::C0(h3907, _, _, _, _, _, _, _, _, _) => {
            h3907
        }
    }), f956(match v0.clone() {
        Adt157::C0(h3908, _, _, _, _, _, _, _, _, _) => {
            h3908
        }
    }, match v0.clone() {
        Adt157::C0(_, h3909, _, _, _, _, _, _, _, _) => {
            h3909
        }
    }, match v0.clone() {
        Adt157::C0(_, _, h3910, _, _, _, _, _, _, _) => {
            h3910
        }
    }));
    Ok(Adt157::C0(f938(v2, v3), match v0.clone() {
        Adt157::C0(h3911, _, _, _, _, _, _, _, _, _) => {
            h3911
        }
    }, match v0.clone() {
        Adt157::C0(_, h3912, _, _, _, _, _, _, _, _) => {
            h3912
        }
    }, match v0.clone() {
        Adt157::C0(_, _, h3913, _, _, _, _, _, _, _) => {
            h3913
        }
    }, f938(match v0.clone() {
        Adt157::C0(_, _, _, h3914, _, _, _, _, _, _) => {
            h3914
        }
    }, v2), match v0.clone() {
        Adt157::C0(_, _, _, _, h3915, _, _, _, _, _) => {
            h3915
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, h3916, _, _, _, _) => {
            h3916
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, h3917, _, _, _) => {
            h3917
        }
    }, f957(match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, h3918, _) => {
            h3918
        }
    }), {
        let a3920 = match v0 {
            Adt157::C0(_, _, _, _, _, _, _, _, _, h3919) => {
                h3919
            }
        };
        let a3921 = 1u64;
        nat_add(a3920, a3921)?
    }))
}

pub fn f946(v0: u32) -> u64 {
    let m3923 = {
        let a3922 = v0;
        fixed_u64::convert(i128::from(a3922))
    };
    match m3923 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f947(v0: u64) -> u32 {
    let m3927 = {
        let a3926 = {
            let a3924 = v0;
            let a3925 = 4294967295u64;
            fixed_u64::bit_and(a3924, a3925)
        };
        fixed_u32::convert(i128::from(a3926))
    };
    match m3927 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f948(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt156> {
    Ok(Adt156::C0(f958(v0.clone(), v1, v2, v3)?, f958(v0.clone(), v1, v2, {
        let a3928 = v3;
        let a3929 = 4u64;
        nat_add(a3928, a3929)?
    })?, f958(v0.clone(), v1, v2, {
        let a3930 = v3;
        let a3931 = 8u64;
        nat_add(a3930, a3931)?
    })?, f958(v0.clone(), v1, v2, {
        let a3932 = v3;
        let a3933 = 12u64;
        nat_add(a3932, a3933)?
    })?, f958(v0.clone(), v1, v2, {
        let a3934 = v3;
        let a3935 = 16u64;
        nat_add(a3934, a3935)?
    })?, f958(v0.clone(), v1, v2, {
        let a3936 = v3;
        let a3937 = 20u64;
        nat_add(a3936, a3937)?
    })?, f958(v0.clone(), v1, v2, {
        let a3938 = v3;
        let a3939 = 24u64;
        nat_add(a3938, a3939)?
    })?, f958(v0.clone(), v1, v2, {
        let a3940 = v3;
        let a3941 = 28u64;
        nat_add(a3940, a3941)?
    })?, f958(v0.clone(), v1, v2, {
        let a3942 = v3;
        let a3943 = 32u64;
        nat_add(a3942, a3943)?
    })?, f958(v0.clone(), v1, v2, {
        let a3944 = v3;
        let a3945 = 36u64;
        nat_add(a3944, a3945)?
    })?, f958(v0.clone(), v1, v2, {
        let a3946 = v3;
        let a3947 = 40u64;
        nat_add(a3946, a3947)?
    })?, f958(v0.clone(), v1, v2, {
        let a3948 = v3;
        let a3949 = 44u64;
        nat_add(a3948, a3949)?
    })?, f958(v0.clone(), v1, v2, {
        let a3950 = v3;
        let a3951 = 48u64;
        nat_add(a3950, a3951)?
    })?, f958(v0.clone(), v1, v2, {
        let a3952 = v3;
        let a3953 = 52u64;
        nat_add(a3952, a3953)?
    })?, f958(v0.clone(), v1, v2, {
        let a3954 = v3;
        let a3955 = 56u64;
        nat_add(a3954, a3955)?
    })?, f958(v0, v1, v2, {
        let a3956 = v3;
        let a3957 = 60u64;
        nat_add(a3956, a3957)?
    })?))
}

pub fn f949(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3962 = f959(v0, 24u32);
    let a3963 = {
        let a3960 = f959(v1, 16u32);
        let a3961 = {
            let a3958 = f959(v2, 8u32);
            let a3959 = v3;
            fixed_u32::bit_or(a3958, a3959)
        };
        fixed_u32::bit_or(a3960, a3961)
    };
    fixed_u32::bit_or(a3962, a3963)
}

pub fn f950(v0: Bytes, v1: u64) -> u32 {
    let m3966 = {
        let a3964 = v0;
        let a3965 = v1;
        index_bytes(a3964, a3965)
    };
    match m3966 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3968 = {
                let a3967 = v2;
                fixed_u32::convert(i128::from(a3967))
            };
            match m3968 {
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

pub fn f951(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3971 = {
        let a3969 = v4;
        let a3970 = 8u64;
        nat_add(a3969, a3970)?
    };
    let a3972 = v2;
    if nat_eq(a3971, a3972) {
        Ok(f960(v3, true))
    } else {
        let a3975 = {
            let a3973 = v4;
            let a3974 = 4u64;
            nat_add(a3973, a3974)?
        };
        let a3976 = v2;
        if nat_eq(a3975, a3976) {
            Ok(f960(v3, false))
        } else {
            Ok(f949(f961(v0.clone(), v1, v4), f961(v0.clone(), v1, {
                let a3977 = v4;
                let a3978 = 1u64;
                nat_add(a3977, a3978)?
            }), f961(v0.clone(), v1, {
                let a3979 = v4;
                let a3980 = 2u64;
                nat_add(a3979, a3980)?
            }), f961(v0, v1, {
                let a3981 = v4;
                let a3982 = 3u64;
                nat_add(a3981, a3982)?
            })))
        }
    }
}

pub fn f952(v0: u64) -> u32 {
    let a3983 = v0;
    let a3984 = 32u64;
    if nat_lt(a3983, a3984) {
        let a3985 = v0;
        let a3986 = 16u64;
        if nat_lt(a3985, a3986) {
            let a3987 = v0;
            let a3988 = 8u64;
            if nat_lt(a3987, a3988) {
                let a3989 = v0;
                let a3990 = 4u64;
                if nat_lt(a3989, a3990) {
                    let a3991 = v0;
                    let a3992 = 2u64;
                    if nat_lt(a3991, a3992) {
                        let a3993 = v0;
                        let a3994 = 1u64;
                        if nat_lt(a3993, a3994) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3995 = v0;
                        let a3996 = 3u64;
                        if nat_lt(a3995, a3996) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3997 = v0;
                    let a3998 = 6u64;
                    if nat_lt(a3997, a3998) {
                        let a3999 = v0;
                        let a4000 = 5u64;
                        if nat_lt(a3999, a4000) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4001 = v0;
                        let a4002 = 7u64;
                        if nat_lt(a4001, a4002) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4003 = v0;
                let a4004 = 12u64;
                if nat_lt(a4003, a4004) {
                    let a4005 = v0;
                    let a4006 = 10u64;
                    if nat_lt(a4005, a4006) {
                        let a4007 = v0;
                        let a4008 = 9u64;
                        if nat_lt(a4007, a4008) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4009 = v0;
                        let a4010 = 11u64;
                        if nat_lt(a4009, a4010) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4011 = v0;
                    let a4012 = 14u64;
                    if nat_lt(a4011, a4012) {
                        let a4013 = v0;
                        let a4014 = 13u64;
                        if nat_lt(a4013, a4014) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4015 = v0;
                        let a4016 = 15u64;
                        if nat_lt(a4015, a4016) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4017 = v0;
            let a4018 = 24u64;
            if nat_lt(a4017, a4018) {
                let a4019 = v0;
                let a4020 = 20u64;
                if nat_lt(a4019, a4020) {
                    let a4021 = v0;
                    let a4022 = 18u64;
                    if nat_lt(a4021, a4022) {
                        let a4023 = v0;
                        let a4024 = 17u64;
                        if nat_lt(a4023, a4024) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4025 = v0;
                        let a4026 = 19u64;
                        if nat_lt(a4025, a4026) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4027 = v0;
                    let a4028 = 22u64;
                    if nat_lt(a4027, a4028) {
                        let a4029 = v0;
                        let a4030 = 21u64;
                        if nat_lt(a4029, a4030) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4031 = v0;
                        let a4032 = 23u64;
                        if nat_lt(a4031, a4032) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4033 = v0;
                let a4034 = 28u64;
                if nat_lt(a4033, a4034) {
                    let a4035 = v0;
                    let a4036 = 26u64;
                    if nat_lt(a4035, a4036) {
                        let a4037 = v0;
                        let a4038 = 25u64;
                        if nat_lt(a4037, a4038) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4039 = v0;
                        let a4040 = 27u64;
                        if nat_lt(a4039, a4040) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4041 = v0;
                    let a4042 = 30u64;
                    if nat_lt(a4041, a4042) {
                        let a4043 = v0;
                        let a4044 = 29u64;
                        if nat_lt(a4043, a4044) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4045 = v0;
                        let a4046 = 31u64;
                        if nat_lt(a4045, a4046) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4047 = v0;
        let a4048 = 48u64;
        if nat_lt(a4047, a4048) {
            let a4049 = v0;
            let a4050 = 40u64;
            if nat_lt(a4049, a4050) {
                let a4051 = v0;
                let a4052 = 36u64;
                if nat_lt(a4051, a4052) {
                    let a4053 = v0;
                    let a4054 = 34u64;
                    if nat_lt(a4053, a4054) {
                        let a4055 = v0;
                        let a4056 = 33u64;
                        if nat_lt(a4055, a4056) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4057 = v0;
                        let a4058 = 35u64;
                        if nat_lt(a4057, a4058) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4059 = v0;
                    let a4060 = 38u64;
                    if nat_lt(a4059, a4060) {
                        let a4061 = v0;
                        let a4062 = 37u64;
                        if nat_lt(a4061, a4062) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4063 = v0;
                        let a4064 = 39u64;
                        if nat_lt(a4063, a4064) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4065 = v0;
                let a4066 = 44u64;
                if nat_lt(a4065, a4066) {
                    let a4067 = v0;
                    let a4068 = 42u64;
                    if nat_lt(a4067, a4068) {
                        let a4069 = v0;
                        let a4070 = 41u64;
                        if nat_lt(a4069, a4070) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4071 = v0;
                        let a4072 = 43u64;
                        if nat_lt(a4071, a4072) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4073 = v0;
                    let a4074 = 46u64;
                    if nat_lt(a4073, a4074) {
                        let a4075 = v0;
                        let a4076 = 45u64;
                        if nat_lt(a4075, a4076) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4077 = v0;
                        let a4078 = 47u64;
                        if nat_lt(a4077, a4078) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4079 = v0;
            let a4080 = 56u64;
            if nat_lt(a4079, a4080) {
                let a4081 = v0;
                let a4082 = 52u64;
                if nat_lt(a4081, a4082) {
                    let a4083 = v0;
                    let a4084 = 50u64;
                    if nat_lt(a4083, a4084) {
                        let a4085 = v0;
                        let a4086 = 49u64;
                        if nat_lt(a4085, a4086) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4087 = v0;
                        let a4088 = 51u64;
                        if nat_lt(a4087, a4088) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4089 = v0;
                    let a4090 = 54u64;
                    if nat_lt(a4089, a4090) {
                        let a4091 = v0;
                        let a4092 = 53u64;
                        if nat_lt(a4091, a4092) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4093 = v0;
                        let a4094 = 55u64;
                        if nat_lt(a4093, a4094) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4095 = v0;
                let a4096 = 60u64;
                if nat_lt(a4095, a4096) {
                    let a4097 = v0;
                    let a4098 = 58u64;
                    if nat_lt(a4097, a4098) {
                        let a4099 = v0;
                        let a4100 = 57u64;
                        if nat_lt(a4099, a4100) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4101 = v0;
                        let a4102 = 59u64;
                        if nat_lt(a4101, a4102) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4103 = v0;
                    let a4104 = 62u64;
                    if nat_lt(a4103, a4104) {
                        let a4105 = v0;
                        let a4106 = 61u64;
                        if nat_lt(a4105, a4106) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4107 = v0;
                        let a4108 = 63u64;
                        if nat_lt(a4107, a4108) {
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

pub fn f953(v0: u32) -> u32 {
    let a4111 = {
        let a4109 = f962(v0, 6u32, 26u32);
        let a4110 = f962(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4109, a4110)
    };
    let a4112 = f962(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4111, a4112)
}

pub fn f954(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4118 = {
        let a4113 = v0;
        let a4114 = v1;
        fixed_u32::bit_and(a4113, a4114)
    };
    let a4119 = {
        let a4116 = {
            let a4115 = v0;
            fixed_u32::bit_not(a4115)
        };
        let a4117 = v2;
        fixed_u32::bit_and(a4116, a4117)
    };
    fixed_u32::bit_xor(a4118, a4119)
}

pub fn f955(v0: u32) -> u32 {
    let a4122 = {
        let a4120 = f962(v0, 2u32, 30u32);
        let a4121 = f962(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4120, a4121)
    };
    let a4123 = f962(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4122, a4123)
}

pub fn f956(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4132 = {
        let a4128 = {
            let a4124 = v0;
            let a4125 = v1;
            fixed_u32::bit_and(a4124, a4125)
        };
        let a4129 = {
            let a4126 = v0;
            let a4127 = v2;
            fixed_u32::bit_and(a4126, a4127)
        };
        fixed_u32::bit_xor(a4128, a4129)
    };
    let a4133 = {
        let a4130 = v1;
        let a4131 = v2;
        fixed_u32::bit_and(a4130, a4131)
    };
    fixed_u32::bit_xor(a4132, a4133)
}

pub fn f957(v0: Adt156) -> Adt156 {
    Adt156::C0(match v0.clone() {
        Adt156::C0(_, h4134, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4134
        }
    }, match v0.clone() {
        Adt156::C0(_, _, h4135, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4135
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, h4136, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4136
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, h4137, _, _, _, _, _, _, _, _, _, _, _) => {
            h4137
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, h4138, _, _, _, _, _, _, _, _, _, _) => {
            h4138
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, h4139, _, _, _, _, _, _, _, _, _) => {
            h4139
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, h4140, _, _, _, _, _, _, _, _) => {
            h4140
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, h4141, _, _, _, _, _, _, _) => {
            h4141
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, h4142, _, _, _, _, _, _) => {
            h4142
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, _, h4143, _, _, _, _, _) => {
            h4143
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, h4144, _, _, _, _) => {
            h4144
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4145, _, _, _) => {
            h4145
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4146, _, _) => {
            h4146
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4147, _) => {
            h4147
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4148) => {
            h4148
        }
    }, f938(f963(match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4149, _) => {
            h4149
        }
    }), f938(match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, _, _, h4150, _, _, _, _, _, _) => {
            h4150
        }
    }, f938(f964(match v0.clone() {
        Adt156::C0(_, h4151, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4151
        }
    }), match v0 {
        Adt156::C0(h4152, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4152
        }
    }))))
}

pub fn f958(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4155 = {
        let a4153 = v3;
        let a4154 = 8u64;
        nat_add(a4153, a4154)?
    };
    let a4156 = v2;
    if nat_eq(a4155, a4156) {
        Ok(f965({
            let a4159 = {
                let a4157 = v1;
                let a4158 = 8u64;
                nat_mul(a4157, a4158)?
            };
            let a4160 = 4294967296u64;
            let a4161 = 0u64;
            nat_quot(a4159, a4160, a4161)
        }))
    } else {
        let a4164 = {
            let a4162 = v3;
            let a4163 = 4u64;
            nat_add(a4162, a4163)?
        };
        let a4165 = v2;
        if nat_eq(a4164, a4165) {
            Ok(f965({
                let a4168 = {
                    let a4166 = v1;
                    let a4167 = 8u64;
                    nat_mul(a4166, a4167)?
                };
                let a4169 = 4294967296u64;
                let a4170 = 0u64;
                nat_rem(a4168, a4169, a4170)
            }))
        } else {
            Ok(f949(f961(v0.clone(), v1, v3), f961(v0.clone(), v1, {
                let a4171 = v3;
                let a4172 = 1u64;
                nat_add(a4171, a4172)?
            }), f961(v0.clone(), v1, {
                let a4173 = v3;
                let a4174 = 2u64;
                nat_add(a4173, a4174)?
            }), f961(v0, v1, {
                let a4175 = v3;
                let a4176 = 3u64;
                nat_add(a4175, a4176)?
            })))
        }
    }
}

pub fn f959(v0: u32, v1: u32) -> u32 {
    let m4179 = {
        let a4177 = v0;
        let a4178 = v1;
        fixed_u32::shift_left(a4177, a4178)
    };
    match m4179 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f960(v0: u64, v1: bool) -> u32 {
    if v1 {
        f965({
            let a4180 = v0;
            let a4181 = 4294967296u64;
            let a4182 = 0u64;
            nat_quot(a4180, a4181, a4182)
        })
    } else {
        f965({
            let a4183 = v0;
            let a4184 = 4294967296u64;
            let a4185 = 0u64;
            nat_rem(a4183, a4184, a4185)
        })
    }
}

pub fn f961(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4186 = v2;
    let a4187 = v1;
    if nat_lt(a4186, a4187) {
        let m4190 = {
            let a4188 = v0;
            let a4189 = v2;
            index_bytes(a4188, a4189)
        };
        match m4190 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4192 = {
                    let a4191 = v3;
                    fixed_u32::convert(i128::from(a4191))
                };
                match m4192 {
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
        let a4193 = v2;
        let a4194 = v1;
        if nat_eq(a4193, a4194) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f962(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4195 = f941(v0, v1);
    let a4196 = f959(v0, v2);
    fixed_u32::bit_or(a4195, a4196)
}

pub fn f963(v0: u32) -> u32 {
    let a4199 = {
        let a4197 = f962(v0, 17u32, 15u32);
        let a4198 = f962(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4197, a4198)
    };
    let a4200 = f941(v0, 10u32);
    fixed_u32::bit_xor(a4199, a4200)
}

pub fn f964(v0: u32) -> u32 {
    let a4203 = {
        let a4201 = f962(v0, 7u32, 25u32);
        let a4202 = f962(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4201, a4202)
    };
    let a4204 = f941(v0, 3u32);
    fixed_u32::bit_xor(a4203, a4204)
}

pub fn f965(v0: u64) -> u32 {
    let (_, h4205) = {
        let (_, h4206) = f966(Fn12::F967, 32u64, (v0, (1u32, 0u32)));
        h4206
    };
    h4205
}

pub fn f966(v0: Fn12, p4210: u64, p4211: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4209 = (p4210, p4211);
    {
        let mut m4209 = m4209;
        loop {
            let step = {
                let (v1, v2) = m4209;
                let m4207 = v1;
                if m4207 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4207 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4208: Fn12 = v0.clone();
                        c4208.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f967(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4212, _) = v0;
        h4212
    };
    let v2: u32 = {
        let (h4213, _) = {
            let (_, h4214) = v0;
            h4214
        };
        h4213
    };
    let v3: u32 = {
        let (_, h4215) = {
            let (_, h4216) = v0;
            h4216
        };
        h4215
    };
    ({
        let a4217 = v1;
        let a4218 = 2u64;
        let a4219 = 0u64;
        nat_quot(a4217, a4218, a4219)
    }, (f959(v2, 1u32), {
        let a4223 = {
            let a4220 = v1;
            let a4221 = 2u64;
            let a4222 = 0u64;
            nat_rem(a4220, a4221, a4222)
        };
        let a4224 = 1u64;
        if nat_eq(a4223, a4224) {
            let a4225 = v3;
            let a4226 = v2;
            fixed_u32::bit_or(a4225, a4226)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
