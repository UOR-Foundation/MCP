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
    F42,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt49, Adt84>> {
        match self {
            Fn1::F42 => f42(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F100,
}

impl Fn2 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt46>>> {
        match self {
            Fn2::F100 => f100(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F106,
}

impl Fn3 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt58 {
        match self {
            Fn3::F106 => f106(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F108,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt24) -> R<Result<List<Str>, Adt84>> {
        match self {
            Fn4::F108 => f108(p0),
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
    Adt4::C0(None::<Adt5>, None::<Adt5>, Some(f28()), Some(Adt8::C0(None::<bool>)), Some(Adt9::C0(None::<bool>, None::<bool>)), Some(Adt8::C0(None::<bool>)), None::<Adt5>)
}

pub fn f10() -> Adt10 {
    Adt10::C0(0u64, Adt11::C1)
}

pub fn f11(v0: List<Adt35>) -> R<List<Adt12>> {
    let m31 = f29(v0)?;
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
    Ok(List::cons(f30(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f31()), List::cons(f30(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f32()), List::cons(f30(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f31()), List::cons(f30(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f33()), List::cons(f30(Str::lit("test_image_content"), Str::lit("Tests image content response"), f33()), List::cons(f30(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f33()), List::cons(f30(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f33()), List::cons(f30(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f33()), List::cons(f30(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f33()), List::cons(f30(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f33()), List::cons(f30(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f34()), List::cons(f30(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f35()?), List::cons(f30(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f36()), List::cons(f37(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f33(), f38()), List::cons(f37(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f33(), f38()), List::cons(f30(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f33()), List::cons(f30(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f33()), List::cons(f30(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f33()), List::<Adt35>::nil())))))))))))))))))))
}

pub fn f13() -> u64 {
    100u64
}

pub fn f14(v0: List<Adt35>) -> R<Option<Adt84>> {
    let m32 = f29(v0)?;
    match m32 {
        Ok(_) => {
            Ok(None::<Adt84>)
        }
        Err(v1) => {
            Ok(Some(Adt84::C9(f39(v1))))
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
            Ok(f40(f10(), f12()?, v1))
        }
        Adt14::C2(v2) => {
            f41(f12()?, Fn1::F42, v2)
        }
        Adt14::C3(v4) => {
            Ok(f43(v4))
        }
        Adt14::C4(v5) => {
            f44(v5)
        }
        Adt14::C5(v6) => {
            Ok(f45(v6))
        }
        Adt14::C6(v7) => {
            Ok(f46(v7))
        }
        Adt14::C7(v8) => {
            Ok(f47(v8))
        }
        Adt14::C8(v9) => {
            f48(v9)
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
    let m34 = f49(v0.clone(), v1)?;
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
            f50(v0.clone(), v1, v3.clone(), f49(v0, match v3 {
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
    f51(f52(v0, List::<Adt86>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f53(f54(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt86> {
    List::cons(Adt86::C0(f55(), f56()), List::<Adt86>::nil())
}

pub fn f23() -> List<u64> {
    let a71 = f57();
    let a72 = {
        let a69 = f58();
        let a70 = f59();
        append_list(a69, a70)
    };
    append_list(a71, a72)
}

pub fn f24(v0: List<u64>) -> bool {
    f60(v0, f61())
}

pub fn f25(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt90>) -> R<Adt88> {
    let m73 = v4;
    match m73 {
        Ok(v6) => {
            f66(v0, v1, v2, v3, f67(v6)?)
        }
        Err(_) => {
            f62(f63(), f64(None::<Adt91>, f65(Adt92::C0)))
        }
    }
}

pub fn f26(v0: Bytes) -> R<Result<Adt6, Adt90>> {
    f68(f69(v0)?)
}

pub fn f27() -> Adt88 {
    Adt88::C0(f70(), List::cons(Adt86::C0(f71(), f72()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f28() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f29(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt94>> {
    let m74 = f73(v0.clone());
    match m74 {
        None => {
            f74(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f30(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f31() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f32() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f75(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f76()), List::cons((Str::lit("limit"), f75(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f75(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f33() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f34() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f35() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f76()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f77(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f77(0u64)?), List::cons((Str::lit("maximum"), f77(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f77(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
}

pub fn f36() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("target"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("zone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt6::C3(Str::lit("Zone"))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f37(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt36>, None::<Adt5>)
}

pub fn f38() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("sum"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("sum")), List::cons(Adt6::C3(Str::lit("label")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f39(v0: Adt94) -> Str {
    let m75 = v0;
    match m75 {
        Adt94::C0(v1) => {
            let a76 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a77 = Str::lit(" ");
            join(a76, a77)
        }
        Adt94::C1(v2) => {
            let a78 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a79 = Str::lit(" ");
            join(a78, a79)
        }
        Adt94::C2(v3) => {
            let a80 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a81 = Str::lit(" ");
            join(a80, a81)
        }
        Adt94::C3(v4) => {
            let a82 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a83 = Str::lit(" ");
            join(a82, a83)
        }
        Adt94::C4(v5, v6) => {
            let a84 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a85 = Str::lit(" ");
            join(a84, a85)
        }
        Adt94::C5(v7, v8) => {
            let a86 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a87 = Str::lit(" ");
            join(a86, a87)
        }
        Adt94::C6(v9, v10) => {
            let a88 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a89 = Str::lit(" ");
            join(a88, a89)
        }
        Adt94::C7(v11) => {
            let a90 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a91 = Str::lit(" ");
            join(a90, a91)
        }
        Adt94::C8(v12) => {
            let a92 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a93 = Str::lit(" ");
            join(a92, a93)
        }
        Adt94::C9(v13) => {
            let a94 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a95 = Str::lit(" ");
            join(a94, a95)
        }
        Adt94::C10(v14) => {
            let a96 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a97 = Str::lit(" ");
            join(a96, a97)
        }
        Adt94::C11(v15) => {
            let a98 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a99 = Str::lit(" ");
            join(a98, a99)
        }
        Adt94::C12(v16, v17) => {
            let a100 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a101 = Str::lit(" ");
            join(a100, a101)
        }
    }
}

pub fn f40(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m103 = match v2 {
        Adt20::C0(_, h102) => {
            h102
        }
    };
    match m103 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C1(Adt34::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f41(v0: List<Adt35>, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m105 = f78(match v2.clone() {
        Adt21::C0(_, h104, _, _, _) => {
            h104
        }
    }, v0);
    match m105 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C0(match v2 {
                Adt21::C0(_, h106, _, _, _) => {
                    h106
                }
            })))
        }
        Some(v3) => {
            f79(v3, v1, v2)
        }
    }
}

pub fn f42(v0: Adt21) -> R<Result<Adt49, Adt84>> {
    if f80(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt49, Adt84>(f81(List::cons(f82(Str::lit("This is a simple text response for testing.")), List::<Adt38>::nil()))))
    } else {
        if f80(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt49, Adt84>(f81(List::cons(f83(), List::<Adt38>::nil()))))
        } else {
            if f80(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt49, Adt84>(f81(List::cons(f84(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt38>::nil()))))
            } else {
                if f80(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt49, Adt84>(f81(List::cons(f85(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt38>::nil()))))
                } else {
                    if f80(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt49, Adt84>(f81(List::cons(f82(Str::lit("Multiple content types test:")), List::cons(f83(), List::cons(f85(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt38>::nil()))))))
                    } else {
                        if f80(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt49, Adt84>(f86(List::cons(f82(Str::lit("This tool intentionally returns an error for testing")), List::<Adt38>::nil()), None::<Adt6>, true, List::<Adt50>::nil())))
                        } else {
                            if f80(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt49, Adt84>(f87(v0, f88()?)))
                            } else {
                                if f80(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt49, Adt84>(f87(v0, f89()?)))
                                } else {
                                    if f80(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt49, Adt84>(f86(List::cons(f82(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt38>::nil()), Some(f90()?), false, List::<Adt50>::nil())))
                                    } else {
                                        if f80(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt49, Adt84>(f86(List::cons(f82(Str::lit("{\"sum\":\"three\"}")), List::<Adt38>::nil()), Some(f91()), false, List::<Adt50>::nil())))
                                        } else {
                                            if f80(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt49, Adt84>(f81(List::cons(f92(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt38>::nil()))))
                                            } else {
                                                if f80(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt49, Adt84>(f81(List::cons(f82(f93()), List::<Adt38>::nil()))))
                                                } else {
                                                    Ok(f94(v0))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f43(v0: Adt20) -> Result<Adt32, Adt84> {
    f95(f96(), f97(), v0)
}

pub fn f44(v0: Adt22) -> R<Result<Adt32, Adt84>> {
    f98(f99(), Fn2::F100, v0)
}

pub fn f45(v0: Adt20) -> Result<Adt32, Adt84> {
    f101(f96(), f102(), v0)
}

pub fn f46(v0: Adt20) -> Result<Adt32, Adt84> {
    f103(f96(), f104(), v0)
}

pub fn f47(v0: Adt23) -> Result<Adt32, Adt84> {
    f105(f104(), Fn3::F106, v0)
}

pub fn f48(v0: Adt24) -> R<Result<Adt32, Adt84>> {
    f107(Fn4::F108, v0)
}

pub fn f49(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a110 = {
        let a107 = v1;
        let a108 = 4u64;
        nat_add(a107, a108)?
    };
    let a111 = {
        let a109 = v0.clone();
        length_bytes(a109)
    };
    if nat_le(a110, a111) {
        Ok(Some({
            let a128 = {
                let a112 = f109(v0.clone(), v1)?;
                let a113 = 16777216u64;
                nat_mul(a112, a113)?
            };
            let a129 = {
                let a126 = {
                    let a116 = f109(v0.clone(), {
                        let a114 = v1;
                        let a115 = 1u64;
                        nat_add(a114, a115)?
                    })?;
                    let a117 = 65536u64;
                    nat_mul(a116, a117)?
                };
                let a127 = {
                    let a124 = {
                        let a120 = f109(v0.clone(), {
                            let a118 = v1;
                            let a119 = 2u64;
                            nat_add(a118, a119)?
                        })?;
                        let a121 = 256u64;
                        nat_mul(a120, a121)?
                    };
                    let a125 = f109(v0, {
                        let a122 = v1;
                        let a123 = 3u64;
                        nat_add(a122, a123)?
                    })?;
                    nat_add(a124, a125)?
                };
                nat_add(a126, a127)?
            };
            nat_add(a128, a129)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f50(v0: Bytes, v1: Adt89, v2: Adt89, v3: Option<u64>) -> R<Result<Adt85, Adt87>> {
    let m130 = v3;
    match m130 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v2 {
                Adt89::C0(_, h131) => {
                    h131
                }
            })))
        }
        Some(v4) => {
            let m135 = f110(v0.clone(), {
                let a133 = match v2.clone() {
                    Adt89::C0(_, h132) => {
                        h132
                    }
                };
                let a134 = 4u64;
                nat_add(a133, a134)?
            }, v4, List::<Adt86>::nil())?;
            match m135 {
                Ok(v6) => {
                    f111(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt85, Adt87>(v5))
                }
            }
        }
    }
}

pub fn f51(p140: List<Adt86>, p141: List<u64>) -> List<u64> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v0, v1) = m139;
                let m136 = v0;
                match m136.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<u64>), List<u64>>((v3, {
                            let a137 = f112(v2);
                            let a138 = v1;
                            append_list(a137, a138)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f52(p144: List<Adt86>, p145: List<Adt86>) -> List<Adt86> {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v0, v1) = m143;
                let m142 = v0;
                match m142.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
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

pub fn f53(p150: List<List<u64>>, p151: List<u64>) -> List<u64> {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v0, v1) = m149;
                let m146 = v0;
                match m146.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a147 = f113(v2);
                            let a148 = v1;
                            append_list(a147, a148)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(p154: List<List<u64>>, p155: List<List<u64>>) -> List<List<u64>> {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v0, v1) = m153;
                let m152 = v0;
                match m152.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
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

pub fn f55() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f56() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f57() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f58() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f59() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f60(p162: List<u64>, p163: List<u64>) -> bool {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v0, v1) = m161;
                let m156 = v0;
                match m156.uncons() {
                    None => {
                        let m157 = v1;
                        match m157.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m158 = v1;
                        match m158.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a159 = v4;
                                let a160 = v6;
                                if nat_eq(a159, a160) {
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
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f62(v0: u64, v1: Adt6) -> R<Adt88> {
    let m164 = f114(v1)?;
    match m164 {
        Ok(v3) => {
            Ok(Adt88::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f63() -> u64 {
    400u64
}

pub fn f64(v0: Option<Adt91>, v1: Adt80) -> Adt6 {
    Adt6::C5(f115(v0, v1))
}

pub fn f65(v0: Adt92) -> Adt80 {
    Adt80::C0(f116(v0.clone()), f117(v0), None::<Adt6>)
}

pub fn f66(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt95, Adt92>) -> R<Adt88> {
    let m165 = v4;
    match m165 {
        Ok(v6) => {
            f120(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f62(f118(v5.clone()), f64(f119(v5.clone()), f65(v5)))
        }
    }
}

pub fn f67(v0: Adt6) -> R<Result<Adt95, Adt92>> {
    let m166 = v0;
    match m166 {
        Adt6::C0 => {
            Ok(f122(None::<Adt91>, Adt93::C0))
        }
        Adt6::C1(_) => {
            Ok(f122(None::<Adt91>, Adt93::C0))
        }
        Adt6::C2(_) => {
            Ok(f122(None::<Adt91>, Adt93::C0))
        }
        Adt6::C3(_) => {
            Ok(f122(None::<Adt91>, Adt93::C0))
        }
        Adt6::C4(_) => {
            Ok(f122(None::<Adt91>, Adt93::C1))
        }
        Adt6::C5(v1) => {
            f121(Adt5::C0(v1))
        }
    }
}

pub fn f68(v0: List<u64>) -> R<Result<Adt6, Adt90>> {
    let m167 = f123(v0.clone())?;
    match m167 {
        Adt101::C0 => {
            f124(f125(), 0u64, v0)
        }
        Adt101::C1(v1) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C0(v1)))
        }
        Adt101::C2(v2) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C1(v2)))
        }
    }
}

pub fn f69(v0: Bytes) -> R<List<u64>> {
    f126(v0.clone(), {
        let a168 = v0;
        length_bytes(a168)
    }, List::<u64>::nil())
}

pub fn f70() -> u64 {
    405u64
}

pub fn f71() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f72() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f73(v0: List<Adt35>) -> Option<Adt94> {
    let m169 = f127(v0, List::<Str>::nil());
    match m169 {
        None => {
            None::<Adt94>
        }
        Some(v1) => {
            Some(Adt94::C1(v1))
        }
    }
}

pub fn f74(p173: List<Adt35>, p174: List<Adt12>) -> R<Result<List<Adt12>, Adt94>> {
    let m172 = (p173, p174);
    {
        let mut m172 = m172;
        loop {
            let step = {
                let (v0, v1) = m172;
                let m170 = v0;
                match m170.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>(Ok(Ok::<List<Adt12>, Adt94>(f128(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m171 = f129(v2)?;
                        match m171 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>((v3, f128(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>(Ok(Err::<List<Adt12>, Adt94>(v4)))
                            }
                        }
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

pub fn f75(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f76() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f77(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f130(v0, 0i64)?, 0i64)))
}

pub fn f78(v0: Str, p180: List<Adt35>) -> Option<Adt35> {
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

pub fn f79(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m187 = f131(Adt6::C5({
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
    }), f132(v2.clone()))?;
    match m187 {
        Adt109::C0 => {
            f133(v0, v1, v2)
        }
        Adt109::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f134(f135(v3, v4)?, true))))
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

pub fn f80(v0: Adt21, v1: Str) -> bool {
    let a189 = match v0 {
        Adt21::C0(_, h188, _, _, _) => {
            h188
        }
    };
    let a190 = v1;
    equal(a189, a190)
}

pub fn f81(v0: List<Adt38>) -> Adt49 {
    f86(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f82(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f83() -> Adt38 {
    f92(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f84(v0: Str, v1: Str) -> Adt38 {
    Adt38::C2(Adt43::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f85(v0: Str, v1: Str, v2: Str) -> Adt38 {
    Adt38::C4(Adt45::C0(Adt46::C0(Adt47::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt40>, None::<Adt5>))
}

pub fn f86(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f87(v0: Adt21, v1: List<Adt50>) -> Adt49 {
    f86(List::cons(f82(f136({
        let m193 = match v0 {
            Adt21::C0(h192, _, _, _, _) => {
                h192
            }
        };
        match m193 {
            Adt16::C0(_, _, _, _, h191, _) => {
                h191
            }
        }
    })), List::<Adt38>::nil()), None::<Adt6>, false, v1)
}

pub fn f88() -> R<List<Adt50>> {
    Ok(List::cons(f137(0u64, 100u64)?, List::cons(f137(50u64, 100u64)?, List::cons(f137(100u64, 100u64)?, List::<Adt50>::nil()))))
}

pub fn f89() -> R<List<Adt50>> {
    Ok(List::cons(f137(50u64, 100u64)?, List::cons(f137(10u64, 100u64)?, List::<Adt50>::nil())))
}

pub fn f90() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f77(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f91() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f92(v0: Str, v1: Str) -> Adt38 {
    Adt38::C1(Adt42::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f93() -> Str {
    let m195 = {
        let a194 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a194)
    };
    match m195 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f94(v0: Adt21) -> Result<Adt49, Adt84> {
    if f80(v0.clone(), Str::lit("test_missing_capability")) {
        let m201 = {
            let m200 = {
                let m199 = match v0 {
                    Adt21::C0(h198, _, _, _, _) => {
                        h198
                    }
                };
                match m199 {
                    Adt16::C0(_, h197, _, _, _, _) => {
                        h197
                    }
                }
            };
            match m200 {
                Adt17::C0(_, _, h196, _, _) => {
                    h196
                }
            }
        };
        match m201 {
            None => {
                Err::<Adt49, Adt84>(Adt84::C8(f138()))
            }
            Some(_) => {
                Ok::<Adt49, Adt84>(f139())
            }
        }
    } else {
        if f80(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt49, Adt84>(f81(List::cons(f82(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt38>::nil())))
        } else {
            if f80(v0, Str::lit("test_arguments")) {
                Ok::<Adt49, Adt84>(f81(List::cons(f82(Str::lit("arguments accepted")), List::<Adt38>::nil())))
            } else {
                Ok::<Adt49, Adt84>(f139())
            }
        }
    }
}

pub fn f95(v0: Adt10, v1: List<Adt44>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m203 = match v2 {
        Adt20::C0(_, h202) => {
            h202
        }
    };
    match m203 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C4(Adt51::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f96() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C0)
}

pub fn f97() -> List<Adt44> {
    f140(f141())
}

pub fn f98(v0: Adt10, v1: Fn2, v2: Adt22) -> R<Result<Adt32, Adt84>> {
    Ok(f142(v0, match v2.clone() {
        Adt22::C0(_, h204, _, _) => {
            h204
        }
    }, {
        let c205: Fn2 = v1;
        c205.apply(match v2 {
            Adt22::C0(_, h206, _, _) => {
                h206
            }
        })?
    }))
}

pub fn f99() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C1)
}

pub fn f100(v0: Str) -> R<Option<List<Adt46>>> {
    let m207 = f143(v0.clone(), f141());
    match m207 {
        None => {
            f144(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f101(v0: Adt10, v1: List<Adt54>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m209 = match v2 {
        Adt20::C0(_, h208) => {
            h208
        }
    };
    match m209 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C6(Adt53::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f102() -> List<Adt54> {
    List::cons(Adt54::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt2>>, f145(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt40>, None::<Adt5>), List::<Adt54>::nil())
}

pub fn f103(v0: Adt10, v1: List<Adt56>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m211 = match v2 {
        Adt20::C0(_, h210) => {
            h210
        }
    };
    match m211 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C7(Adt55::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f104() -> List<Adt56> {
    List::cons(f146(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt57>>), List::cons(f146(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f147(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f147(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt57>::nil())))), List::cons(f146(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f147(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt57>::nil()))), List::cons(f146(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt57>>), List::<Adt56>::nil()))))
}

pub fn f105(v0: List<Adt56>, v1: Fn3, v2: Adt23) -> Result<Adt32, Adt84> {
    let m213 = f148(match v2.clone() {
        Adt23::C0(_, h212, _, _, _) => {
            h212
        }
    }, v0);
    match m213 {
        None => {
            Err::<Adt32, Adt84>(Adt84::C1(match v2 {
                Adt23::C0(_, h214, _, _, _) => {
                    h214
                }
            }))
        }
        Some(v3) => {
            let m216 = f149(f150(v3), f151(match v2.clone() {
                Adt23::C0(_, _, h215, _, _) => {
                    h215
                }
            }));
            match m216 {
                None => {
                    Ok::<Adt32, Adt84>(Adt32::C8({
                        let c219: Fn3 = v1;
                        c219.apply(match v2.clone() {
                            Adt23::C0(_, h220, _, _, _) => {
                                h220
                            }
                        }, f151(match v2 {
                            Adt23::C0(_, _, h221, _, _) => {
                                h221
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt32, Adt84>(Adt84::C4({
                        let a217 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a218 = Str::lit(" ");
                        join(a217, a218)
                    }))
                }
            }
        }
    }
}

pub fn f106(v0: Str, v1: List<(Str, Str)>) -> Adt58 {
    let a222 = v0.clone();
    let a223 = Str::lit("test_simple_prompt");
    if equal(a222, a223) {
        f152(List::cons(f153(Str::lit("This is a simple prompt for testing.")), List::<Adt59>::nil()))
    } else {
        let a224 = v0.clone();
        let a225 = Str::lit("test_prompt_with_arguments");
        if equal(a224, a225) {
            f152(List::cons(f153({
                let a226 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f154(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f154(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a227 = Str::lit("");
                join(a226, a227)
            }), List::<Adt59>::nil()))
        } else {
            let a228 = v0.clone();
            let a229 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a228, a229) {
                f152(List::cons(Adt59::C0(Adt41::C0, Adt38::C4(Adt45::C0(f155(f154(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt40>, None::<Adt5>))), List::cons(f153(Str::lit("Please process the embedded resource above.")), List::<Adt59>::nil())))
            } else {
                let a230 = v0;
                let a231 = Str::lit("test_prompt_with_image");
                if equal(a230, a231) {
                    f152(List::cons(Adt59::C0(Adt41::C0, Adt38::C1(Adt42::C0(f156(), Str::lit("image/png"), None::<Adt40>, None::<Adt5>))), List::cons(f153(Str::lit("Please analyze the image above.")), List::<Adt59>::nil())))
                } else {
                    f152(List::<Adt59>::nil())
                }
            }
        }
    }
}

pub fn f107(v0: Fn4, v1: Adt24) -> R<Result<Adt32, Adt84>> {
    let m233 = {
        let c232: Fn4 = v0;
        c232.apply(v1)?
    };
    match m233 {
        Ok(v3) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C9(f157(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt32, Adt84>(v2))
        }
    }
}

pub fn f108(v0: Adt24) -> R<Result<List<Str>, Adt84>> {
    let m235 = match v0.clone() {
        Adt24::C0(_, h234, _, _) => {
            h234
        }
    };
    match m235 {
        Adt25::C0(v1) => {
            let m237 = f148(match v1.clone() {
                Adt26::C0(h236, _) => {
                    h236
                }
            }, f104());
            match m237 {
                None => {
                    Ok(Err::<List<Str>, Adt84>(Adt84::C1(match v1 {
                        Adt26::C0(h238, _) => {
                            h238
                        }
                    })))
                }
                Some(v2) => {
                    if f158(f150(v2), {
                        let m241 = match v0.clone() {
                            Adt24::C0(_, _, h240, _) => {
                                h240
                            }
                        };
                        match m241 {
                            Adt27::C0(h239, _) => {
                                h239
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt84>(f159({
                            let m244 = match v0.clone() {
                                Adt24::C0(_, _, h243, _) => {
                                    h243
                                }
                            };
                            match m244 {
                                Adt27::C0(_, h242) => {
                                    h242
                                }
                            }
                        }, f160(match v1 {
                            Adt26::C0(h245, _) => {
                                h245
                            }
                        }, {
                            let m248 = match v0 {
                                Adt24::C0(_, _, h247, _) => {
                                    h247
                                }
                            };
                            match m248 {
                                Adt27::C0(h246, _) => {
                                    h246
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
            let a249 = v3.clone();
            let a250 = f145();
            if equal(a249, a250) {
                let a254 = {
                    let m253 = match v0.clone() {
                        Adt24::C0(_, _, h252, _) => {
                            h252
                        }
                    };
                    match m253 {
                        Adt27::C0(h251, _) => {
                            h251
                        }
                    }
                };
                let a255 = Str::lit("id");
                if equal(a254, a255) {
                    Ok(Ok::<List<Str>, Adt84>(f159({
                        let m258 = match v0 {
                            Adt24::C0(_, _, h257, _) => {
                                h257
                            }
                        };
                        match m258 {
                            Adt27::C0(_, h256) => {
                                h256
                            }
                        }
                    }, f161())?))
                } else {
                    Ok(Err::<List<Str>, Adt84>(Adt84::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt84>(Adt84::C2(v3)))
            }
        }
    }
}

pub fn f109(v0: Bytes, v1: u64) -> R<u64> {
    let m261 = {
        let a259 = v0;
        let a260 = v1;
        index_bytes(a259, a260)
    };
    match m261 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a306 = {
                let a282 = {
                    let a270 = {
                        let a264 = {
                            let a262 = v3;
                            let a263 = 128u8;
                            fixed_u8::bit_and(a262, a263)
                        };
                        let a265 = 128u8;
                        if equal(a264, a265) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a271 = {
                        let a268 = {
                            let a266 = v3;
                            let a267 = 64u8;
                            fixed_u8::bit_and(a266, a267)
                        };
                        let a269 = 64u8;
                        if equal(a268, a269) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a270, a271)?
                };
                let a283 = {
                    let a280 = {
                        let a274 = {
                            let a272 = v3;
                            let a273 = 32u8;
                            fixed_u8::bit_and(a272, a273)
                        };
                        let a275 = 32u8;
                        if equal(a274, a275) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a281 = {
                        let a278 = {
                            let a276 = v3;
                            let a277 = 16u8;
                            fixed_u8::bit_and(a276, a277)
                        };
                        let a279 = 16u8;
                        if equal(a278, a279) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a280, a281)?
                };
                nat_add(a282, a283)?
            };
            let a307 = {
                let a304 = {
                    let a292 = {
                        let a286 = {
                            let a284 = v3;
                            let a285 = 8u8;
                            fixed_u8::bit_and(a284, a285)
                        };
                        let a287 = 8u8;
                        if equal(a286, a287) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a293 = {
                        let a290 = {
                            let a288 = v3;
                            let a289 = 4u8;
                            fixed_u8::bit_and(a288, a289)
                        };
                        let a291 = 4u8;
                        if equal(a290, a291) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a292, a293)?
                };
                let a305 = {
                    let a302 = {
                        let a296 = {
                            let a294 = v3;
                            let a295 = 2u8;
                            fixed_u8::bit_and(a294, a295)
                        };
                        let a297 = 2u8;
                        if equal(a296, a297) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a303 = {
                        let a300 = {
                            let a298 = v3;
                            let a299 = 1u8;
                            fixed_u8::bit_and(a298, a299)
                        };
                        let a301 = 1u8;
                        if equal(a300, a301) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a302, a303)?
                };
                nat_add(a304, a305)?
            };
            nat_add(a306, a307)
        }
    }
}

pub fn f110(v0: Bytes, p317: u64, p318: u64, p319: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m316 = (p317, (p318, p319));
    {
        let mut m316 = m316;
        loop {
            let step = {
                let (v1, (v2, v3)) = m316;
                let m308 = v2;
                if m308 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f162(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m308 - 1;
                    let m309 = f16(v0.clone(), v1)?;
                    match m309 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m311 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h310) => {
                                    h310
                                }
                            })?;
                            match m311 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h312) => {
                                            h312
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h313) => {
                                            h313
                                        }
                                    }, (v4, List::cons(Adt86::C0(f69(match v5 {
                                        Adt89::C0(h314, _) => {
                                            h314
                                        }
                                    })?, f69(match v6 {
                                        Adt89::C0(h315, _) => {
                                            h315
                                        }
                                    })?), v3))))
                                }
                            }
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

pub fn f111(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m321 = f16(v0.clone(), {
        let (_, h320) = v3.clone();
        h320
    })?;
    match m321 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h322) = v3;
                h322
            })))
        }
        Some(v4) => {
            let m324 = f49(v0.clone(), match v4.clone() {
                Adt89::C0(_, h323) => {
                    h323
                }
            })?;
            match m324 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h325) => {
                            h325
                        }
                    })))
                }
                Some(v5) => {
                    let a330 = {
                        let a327 = match v4.clone() {
                            Adt89::C0(_, h326) => {
                                h326
                            }
                        };
                        let a328 = 4u64;
                        nat_add(a327, a328)?
                    };
                    let a331 = {
                        let a329 = v0;
                        length_bytes(a329)
                    };
                    if nat_eq(a330, a331) {
                        Ok(Ok::<Adt85, Adt87>(Adt85::C0(f69(match v1 {
                            Adt89::C0(h332, _) => {
                                h332
                            }
                        })?, f69(match v2 {
                            Adt89::C0(h333, _) => {
                                h333
                            }
                        })?, {
                            let (h334, _) = v3;
                            h334
                        }, match v4 {
                            Adt89::C0(h335, _) => {
                                h335
                            }
                        }, v5)))
                    } else {
                        Ok(Err::<Adt85, Adt87>(Adt87::C1({
                            let a337 = match v4 {
                                Adt89::C0(_, h336) => {
                                    h336
                                }
                            };
                            let a338 = 4u64;
                            nat_add(a337, a338)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f112(v0: Adt86) -> List<u64> {
    let a349 = f18({
        let a340 = match v0.clone() {
            Adt86::C0(h339, _) => {
                h339
            }
        };
        length_list(a340)
    });
    let a350 = {
        let a347 = match v0.clone() {
            Adt86::C0(h341, _) => {
                h341
            }
        };
        let a348 = {
            let a345 = f18({
                let a343 = match v0.clone() {
                    Adt86::C0(_, h342) => {
                        h342
                    }
                };
                length_list(a343)
            });
            let a346 = match v0 {
                Adt86::C0(_, h344) => {
                    h344
                }
            };
            append_list(a345, a346)
        };
        append_list(a347, a348)
    };
    append_list(a349, a350)
}

pub fn f113(v0: List<u64>) -> List<u64> {
    let a352 = f18({
        let a351 = v0.clone();
        length_list(a351)
    });
    let a353 = v0;
    append_list(a352, a353)
}

pub fn f114(v0: Adt6) -> R<Result<List<u64>, Adt90>> {
    f163(f164(), v0)
}

pub fn f115(v0: Option<Adt91>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f165())), {
        let a354 = f166(v0);
        let a355 = List::cons((Str::lit("error"), f167(v1)), List::<(Str, Adt6)>::nil());
        append_list(a354, a355)
    })
}

pub fn f116(v0: Adt92) -> i64 {
    let m356 = v0;
    match m356 {
        Adt92::C0 => {
            f168()
        }
        Adt92::C1(_, _) => {
            f169()
        }
    }
}

pub fn f117(v0: Adt92) -> Str {
    let m357 = v0;
    match m357 {
        Adt92::C0 => {
            Str::lit("Parse error")
        }
        Adt92::C1(_, v2) => {
            f170(v2)
        }
    }
}

pub fn f118(v0: Adt92) -> u64 {
    let m358 = v0;
    match m358 {
        Adt92::C0 => {
            f63()
        }
        Adt92::C1(_, _) => {
            f63()
        }
    }
}

pub fn f119(v0: Adt92) -> Option<Adt91> {
    let m359 = v0;
    match m359 {
        Adt92::C0 => {
            None::<Adt91>
        }
        Adt92::C1(v1, _) => {
            v1
        }
    }
}

pub fn f120(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt95) -> R<Adt88> {
    f171(f172(v0.clone(), v1, v2, Some(f173(v3, match v0 {
        Adt0::C0(_, _, _, _, h360, _) => {
            h360
        }
    }, f174(v4.clone()), match v4.clone() {
        Adt95::C0(_, h361, _) => {
            h361
        }
    })?), f175(), v4)?)
}

pub fn f121(v0: Adt5) -> R<Result<Adt95, Adt92>> {
    let m362 = f176(v0.clone())?;
    match m362 {
        Adt122::C0 => {
            let m363 = f177(Str::lit("method"), v0.clone());
            match m363 {
                Adt123::C0 => {
                    Ok(f122(None::<Adt91>, f178(v0)))
                }
                Adt123::C1(_) => {
                    Ok(f122(None::<Adt91>, if f179(v0) {
                        Adt93::C8
                    } else {
                        Adt93::C7
                    }))
                }
            }
        }
        Adt122::C1(v2) => {
            Ok(f180(v2, v0))
        }
        Adt122::C2 => {
            Ok(f122(None::<Adt91>, Adt93::C5))
        }
    }
}

pub fn f122(v0: Option<Adt91>, v1: Adt93) -> Result<Adt95, Adt92> {
    Err::<Adt95, Adt92>(Adt92::C1(v0, v1))
}

pub fn f123(v0: List<u64>) -> R<Adt101> {
    Ok(f181({
        let a364 = v0.clone();
        length_list(a364)
    }, f182(Adt102::C0, 0u64, 0u64, v0)?))
}

pub fn f124(p371: Adt103, p372: u64, p373: List<u64>) -> R<Result<Adt6, Adt90>> {
    let m370 = (p371, (p372, p373));
    {
        let mut m370 = m370;
        loop {
            let step = {
                let (v0, (v1, v2)) = m370;
                let m365 = v2;
                match m365.uncons() {
                    None => {
                        let m366 = f183(v0, v1)?;
                        match m366 {
                            Ok(v3) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Ok::<Adt6, Adt90>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Err::<Adt6, Adt90>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m367 = f184(v0, v1, v5)?;
                        match m367 {
                            Adt124::C0(v7) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Err::<Adt6, Adt90>(v7)))
                            }
                            Adt124::C1(v8) => {
                                Ok::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>((v8, ({
                                    let a368 = v1;
                                    let a369 = 1u64;
                                    nat_add(a368, a369)?
                                }, v6)))
                            }
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

pub fn f125() -> Adt103 {
    Adt103::C0(Adt104::C0, List::<Adt108>::nil(), 0u64)
}

pub fn f126(v0: Bytes, p376: u64, p377: List<u64>) -> R<List<u64>> {
    let m375 = (p376, p377);
    {
        let mut m375 = m375;
        loop {
            let step = {
                let (v1, v2) = m375;
                let m374 = v1;
                if m374 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m374 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f109(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(p383: List<Adt35>, p384: List<Str>) -> Option<Str> {
    let m382 = (p383, p384);
    {
        let mut m382 = m382;
        loop {
            let step = {
                let (v0, v1) = m382;
                let m378 = v0;
                match m378.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f185(match v2.clone() {
                            Adt35::C0(h379, _, _, _, _, _, _, _) => {
                                h379
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h380, _, _, _, _, _, _, _) => {
                                    h380
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h381, _, _, _, _, _, _, _) => {
                                    h381
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(p387: List<Adt12>, p388: List<Adt12>) -> List<Adt12> {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v0, v1) = m386;
                let m385 = v0;
                match m385.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Adt35) -> R<Result<List<Adt12>, Adt94>> {
    let m391 = f186(match v0.clone() {
        Adt35::C0(h389, _, _, _, _, _, _, _) => {
            h389
        }
    }, f187(match v0.clone() {
        Adt35::C0(h390, _, _, _, _, _, _, _) => {
            h390
        }
    })?);
    match m391 {
        None => {
            f188(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f130(p396: u64, p397: i64) -> R<i64> {
    let m395 = (p396, p397);
    {
        let mut m395 = m395;
        loop {
            let step = {
                let (v0, v1) = m395;
                let m392 = v0;
                if m392 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m392 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a393 = v1;
                        let a394 = -1i64;
                        int_sub(a393, a394)?
                    }))
                }
            };
            match step {
                Ok(next) => { m395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt109> {
    f189(v0, f190(), Adt111::C0, List::cons(Adt112::C0(v1, v2, List::<Adt110>::nil()), List::<Adt112>::nil()))
}

pub fn f132(v0: Adt21) -> Adt6 {
    let m399 = match v0 {
        Adt21::C0(_, _, h398, _, _) => {
            h398
        }
    };
    match m399 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h400) => {
                    h400
                }
            })
        }
    }
}

pub fn f133(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m402 = {
        let c401: Fn1 = v1;
        c401.apply(v2)?
    };
    match m402 {
        Ok(v4) => {
            f191(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f134(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f135(v0: List<Adt110>, v1: Str) -> R<Str> {
    let v2: Str = f192(v0)?;
    let a405 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a403 = v2.clone();
        let a404 = Str::lit("");
        if equal(a403, a404) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a406 = Str::lit(" ");
    Ok(join(a405, a406))
}

pub fn f136(v0: Option<Adt19>) -> Str {
    let m407 = v0;
    match m407 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m408 = v1;
            match m408 {
                Adt19::C0(v2) => {
                    v2
                }
                Adt19::C1(v3) => {
                    let a409 = v3;
                    format_i64(a409)
                }
            }
        }
    }
}

pub fn f137(v0: u64, v1: u64) -> R<Adt50> {
    Ok(Adt50::C0(f193(v0)?, Some(f193(v1)?), Some({
        let a412 = List::cons(Str::lit("Completed step"), List::cons({
            let a410 = f194(v0, 0i64)?;
            format_i64(a410)
        }, List::cons(Str::lit("of"), List::cons({
            let a411 = f194(v1, 0i64)?;
            format_i64(a411)
        }, List::<Str>::nil()))));
        let a413 = Str::lit(" ");
        join(a412, a413)
    })))
}

pub fn f138() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, Some(f28()), None::<Adt5>, None::<Adt5>)
}

pub fn f139() -> Adt49 {
    f81(List::cons(f82(Str::lit("ok")), List::<Adt38>::nil()))
}

pub fn f140(v0: List<(Adt44, List<Adt46>)>) -> List<Adt44> {
    f195(f196(v0, List::<Adt44>::nil()), List::<Adt44>::nil())
}

pub fn f141() -> List<(Adt44, List<Adt46>)> {
    List::cons(f197(f198(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f155(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f197(f198(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f199(Str::lit("test://static-binary"), Str::lit("image/png"), f156())), List::cons(f197(f198(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f155(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f197(f198(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f155(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt44, List<Adt46>)>::nil()))))
}

pub fn f142(v0: Adt10, v1: Str, v2: Option<List<Adt46>>) -> Result<Adt32, Adt84> {
    let m414 = v2;
    match m414 {
        None => {
            Err::<Adt32, Adt84>(Adt84::C2(v1))
        }
        Some(v3) => {
            let m415 = v3.clone();
            match m415.uncons() {
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

pub fn f143(v0: Str, p424: List<(Adt44, List<Adt46>)>) -> Option<List<Adt46>> {
    let m423 = p424;
    {
        let mut m423 = m423;
        loop {
            let step = {
                let v1 = m423;
                let m416 = v1;
                match m416.uncons() {
                    None => {
                        Err::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(None::<List<Adt46>>)
                    }
                    Some((v2, v3)) => {
                        let a420 = {
                            let m419 = {
                                let (h418, _) = v2;
                                h418
                            };
                            match m419 {
                                Adt44::C0(_, _, _, h417, _, _, _, _, _) => {
                                    h417
                                }
                            }
                        };
                        let a421 = v0.clone();
                        if equal(a420, a421) {
                            Err::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(Some({
                                let (_, h422) = v2;
                                h422
                            }))
                        } else {
                            Ok::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(v3)
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

pub fn f144(v0: Str) -> R<Option<List<Adt46>>> {
    let m425 = f200(v0.clone())?;
    match m425 {
        None => {
            Ok(None::<List<Adt46>>)
        }
        Some(v1) => {
            let m426 = f201(v1)?;
            match m426 {
                None => {
                    Ok(None::<List<Adt46>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f155(v0, Str::lit("application/json"), v2), List::<Adt46>::nil())))
                }
            }
        }
    }
}

pub fn f145() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f146(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt57>>) -> Adt56 {
    Adt56::C0(v0, Some(v1), None::<List<Adt2>>, Some(v2), v3, None::<Adt5>)
}

pub fn f147(v0: Str, v1: Str) -> Adt57 {
    Adt57::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f148(v0: Str, p432: List<Adt56>) -> Option<Adt56> {
    let m431 = p432;
    {
        let mut m431 = m431;
        loop {
            let step = {
                let v1 = m431;
                let m427 = v1;
                match m427.uncons() {
                    None => {
                        Err::<List<Adt56>, Option<Adt56>>(None::<Adt56>)
                    }
                    Some((v2, v3)) => {
                        let a429 = match v2.clone() {
                            Adt56::C0(h428, _, _, _, _, _) => {
                                h428
                            }
                        };
                        let a430 = v0.clone();
                        if equal(a429, a430) {
                            Err::<List<Adt56>, Option<Adt56>>(Some(v2))
                        } else {
                            Ok::<List<Adt56>, Option<Adt56>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(p438: List<Adt57>, v1: List<(Str, Str)>) -> Option<Str> {
    let m437 = p438;
    {
        let mut m437 = m437;
        loop {
            let step = {
                let v0 = m437;
                let m433 = v0;
                match m433.uncons() {
                    None => {
                        Err::<List<Adt57>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f202(v2.clone()) {
                            let m435 = f203(match v2.clone() {
                                Adt57::C0(h434, _, _, _) => {
                                    h434
                                }
                            }, v1.clone());
                            match m435 {
                                None => {
                                    Err::<List<Adt57>, Option<Str>>(Some(match v2 {
                                        Adt57::C0(h436, _, _, _) => {
                                            h436
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
                Ok(next) => { m437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Adt56) -> List<Adt57> {
    let m440 = match v0 {
        Adt56::C0(_, _, _, _, h439, _) => {
            h439
        }
    };
    match m440 {
        None => {
            List::<Adt57>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f151(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m441 = v0;
    match m441 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f152(v0: List<Adt59>) -> Adt58 {
    Adt58::C0(None::<Str>, v0)
}

pub fn f153(v0: Str) -> Adt59 {
    Adt59::C0(Adt41::C0, Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)))
}

pub fn f154(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m442 = f203(v0, v1);
    match m442 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f155(v0: Str, v1: Str, v2: Str) -> Adt46 {
    Adt46::C0(Adt47::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f156() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f157(v0: List<Str>) -> R<Adt60> {
    let v1: u64 = f204(v0.clone(), 0u64)?;
    Ok(Adt60::C0(Adt61::C0(f205(v0), Some(v1), Some({
        let a443 = f206();
        let a444 = v1;
        nat_lt(a443, a444)
    }))))
}

pub fn f158(p450: List<Adt57>, v1: Str) -> bool {
    let m449 = p450;
    {
        let mut m449 = m449;
        loop {
            let step = {
                let v0 = m449;
                let m445 = v0;
                match m445.uncons() {
                    None => {
                        Err::<List<Adt57>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a447 = match v2 {
                            Adt57::C0(h446, _, _, _) => {
                                h446
                            }
                        };
                        let a448 = v1.clone();
                        if equal(a447, a448) {
                            Err::<List<Adt57>, bool>(true)
                        } else {
                            Ok::<List<Adt57>, bool>(v3)
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

pub fn f159(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f207(f208(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f160(v0: Str, v1: Str) -> List<Str> {
    let a451 = v0;
    let a452 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a451, a452) {
        let a453 = v1;
        let a454 = Str::lit("resourceUri");
        equal(a453, a454)
    } else {
        false
    } {
        f209()
    } else {
        List::<Str>::nil()
    }
}

pub fn f161() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f162(p457: List<Adt86>, p458: List<Adt86>) -> List<Adt86> {
    let m456 = (p457, p458);
    {
        let mut m456 = m456;
        loop {
            let step = {
                let (v0, v1) = m456;
                let m455 = v0;
                match m455.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt90>> {
    let m459 = f210(v0, List::cons(Adt114::C1(0u64, v1), List::<Adt114>::nil()), List::<u64>::nil())?;
    match m459 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt90>(f211(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt90>(v3))
        }
    }
}

pub fn f164() -> u64 {
    67108864u64
}

pub fn f165() -> Str {
    Str::lit("2.0")
}

pub fn f166(v0: Option<Adt91>) -> List<(Str, Adt6)> {
    let m460 = v0;
    match m460 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f212(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f167(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h461, _, _) => {
            h461
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h462, _) => {
            h462
        }
    })), f213(match v0 {
        Adt80::C0(_, _, h463) => {
            h463
        }
    }))))
}

pub fn f168() -> i64 {
    -32700i64
}

pub fn f169() -> i64 {
    -32600i64
}

pub fn f170(v0: Adt93) -> Str {
    let m464 = v0;
    match m464 {
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

pub fn f171(v0: Adt115) -> R<Adt88> {
    let m466 = match v0.clone() {
        Adt115::C0(_, h465) => {
            h465
        }
    };
    match m466 {
        Ok(v1) => {
            let m467 = f214(v1.clone())?;
            match m467 {
                None => {
                    Ok(f6())
                }
                Some(v2) => {
                    Ok(f215(f62(f216(), f217(match v0 {
                        Adt115::C0(h468, _) => {
                            h468
                        }
                    }, v2))?, f218(match v1 {
                        Adt116::C0(_, _, h469) => {
                            h469
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v3) => {
            f62(f219(v3.clone()), f64(Some(match v0 {
                Adt115::C0(h470, _) => {
                    h470
                }
            }), f220(v3)?))
        }
    }
}

pub fn f172(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt96>, v4: List<Str>, v5: Adt95) -> R<Adt115> {
    Ok(Adt115::C0(match v5.clone() {
        Adt95::C0(h471, _, _) => {
            h471
        }
    }, f221(v0, v1, v2, f222(v3, v4, v5)?)?))
}

pub fn f173(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt96> {
    let v4: Adt121 = f223(Adt98::C0, f224(v0.clone(), f225())?);
    let v5: Adt121 = f223(Adt98::C1, f224(v0.clone(), f226())?);
    let v6: Adt121 = if f227(v3) {
        f223(Adt98::C2, f228(f224(v0.clone(), f229())?)?)
    } else {
        f230()
    };
    Ok(Adt96::C0(match v4.clone() {
        Adt121::C0(h472, _) => {
            h472
        }
    }, match v5.clone() {
        Adt121::C0(h473, _) => {
            h473
        }
    }, match v6.clone() {
        Adt121::C0(h474, _) => {
            h474
        }
    }, f231(match v4 {
        Adt121::C0(_, h475) => {
            h475
        }
    }, f231(match v5 {
        Adt121::C0(_, h476) => {
            h476
        }
    }, match v6 {
        Adt121::C0(_, h477) => {
            h477
        }
    })), f232(v0, v1, v2)?))
}

pub fn f174(v0: Adt95) -> Option<Str> {
    let a479 = match v0.clone() {
        Adt95::C0(_, h478, _) => {
            h478
        }
    };
    let a480 = Str::lit("tools/call");
    if equal(a479, a480) {
        let m482 = match v0 {
            Adt95::C0(_, _, h481) => {
                h481
            }
        };
        match m482 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m483 = f177(Str::lit("name"), v1);
                match m483 {
                    Adt123::C0 => {
                        None::<Str>
                    }
                    Adt123::C1(v2) => {
                        f233(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f175() -> List<Str> {
    List::cons(f234(), List::<Str>::nil())
}

pub fn f176(v0: Adt5) -> R<Adt122> {
    let m484 = f177(Str::lit("id"), v0);
    match m484 {
        Adt123::C0 => {
            Ok(Adt122::C0)
        }
        Adt123::C1(v1) => {
            let m485 = v1;
            match m485 {
                Adt6::C0 => {
                    Ok(Adt122::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt122::C2)
                }
                Adt6::C2(v3) => {
                    let m486 = f235(v3)?;
                    match m486 {
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

pub fn f177(v0: Str, v1: Adt5) -> Adt123 {
    f236(v0, match v1 {
        Adt5::C0(h487) => {
            h487
        }
    })
}

pub fn f178(v0: Adt5) -> Adt93 {
    if f179(v0) {
        Adt93::C8
    } else {
        Adt93::C3
    }
}

pub fn f179(v0: Adt5) -> bool {
    if f237(Str::lit("result"), v0.clone()) {
        true
    } else {
        f237(Str::lit("error"), v0)
    }
}

pub fn f180(v0: Adt91, v1: Adt5) -> Result<Adt95, Adt92> {
    let m488 = f177(Str::lit("jsonrpc"), v1.clone());
    match m488 {
        Adt123::C0 => {
            f122(Some(v0), Adt93::C2)
        }
        Adt123::C1(v2) => {
            let m489 = f233(v2);
            match m489 {
                None => {
                    f122(Some(v0), Adt93::C2)
                }
                Some(v3) => {
                    let a490 = v3;
                    let a491 = f165();
                    if equal(a490, a491) {
                        f238(v0, v1)
                    } else {
                        f122(Some(v0), Adt93::C2)
                    }
                }
            }
        }
    }
}

pub fn f181(v0: u64, v1: u64) -> Adt101 {
    let a492 = v1;
    let a493 = 0u64;
    if nat_eq(a492, a493) {
        Adt101::C0
    } else {
        let a494 = v1;
        let a495 = 2u64;
        if nat_eq(a494, a495) {
            Adt101::C2(v0)
        } else {
            Adt101::C1({
                let a499 = v0;
                let a500 = {
                    let a496 = v1;
                    let a497 = 3u64;
                    let a498 = 0u64;
                    nat_quot(a496, a497, a498)
                };
                nat_sub(a499, a500)
            })
        }
    }
}

pub fn f182(p528: Adt102, p529: u64, p530: u64, p531: List<u64>) -> R<u64> {
    let m527 = (p528, (p529, (p530, p531)));
    {
        let mut m527 = m527;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m527;
                let m501 = v3;
                match m501.uncons() {
                    None => {
                        let m502 = v0;
                        match m502 {
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
                        let m503 = v0;
                        match m503 {
                            Adt102::C0 => {
                                let a504 = v4;
                                let a505 = 128u64;
                                if nat_lt(a504, a505) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a506 = v4;
                                    let a507 = 194u64;
                                    if nat_lt(a506, a507) {
                                        let v6: u64 = f239({
                                            let a508 = List::cons(v4, v5);
                                            length_list(a508)
                                        })?;
                                        Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a509 = v4;
                                        let a510 = 224u64;
                                        if nat_lt(a509, a510) {
                                            Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a511 = v4;
                                            let a512 = 224u64;
                                            if nat_eq(a511, a512) {
                                                Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a513 = v4;
                                                let a514 = 237u64;
                                                if nat_eq(a513, a514) {
                                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a515 = v4;
                                                    let a516 = 240u64;
                                                    if nat_lt(a515, a516) {
                                                        Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a517 = v4;
                                                        let a518 = 240u64;
                                                        if nat_eq(a517, a518) {
                                                            Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a519 = v4;
                                                            let a520 = 244u64;
                                                            if nat_eq(a519, a520) {
                                                                Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a521 = v4;
                                                                let a522 = 244u64;
                                                                if nat_lt(a521, a522) {
                                                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f239({
                                                                        let a523 = List::cons(v4, v5);
                                                                        length_list(a523)
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
                                if f240(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f239({
                                        let a524 = List::cons(v4, v5);
                                        length_list(a524)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt102::C2 => {
                                if f240(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f239({
                                        let a525 = List::cons(v4, v5);
                                        length_list(a525)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt102::C3 => {
                                if f240(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f239({
                                        let a526 = List::cons(v4, v5);
                                        length_list(a526)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: Adt103, v1: u64) -> R<Result<Adt6, Adt90>> {
    let m533 = match v0.clone() {
        Adt103::C0(h532, _, _) => {
            h532
        }
    };
    match m533 {
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
            let m534 = f241(v3)?;
            match m534 {
                Adt131::C0 => {
                    Ok(Err::<Adt6, Adt90>(Adt90::C7(v1)))
                }
                Adt131::C1 => {
                    Ok(Err::<Adt6, Adt90>(Adt90::C8(v1)))
                }
                Adt131::C2(v4) => {
                    let m539 = {
                        let m538 = f242(v4, match v0.clone() {
                            Adt103::C0(_, h536, _) => {
                                h536
                            }
                        }, match v0 {
                            Adt103::C0(_, _, h537) => {
                                h537
                            }
                        });
                        match m538 {
                            Adt103::C0(h535, _, _) => {
                                h535
                            }
                        }
                    };
                    match m539 {
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

pub fn f184(v0: Adt103, v1: u64, v2: u64) -> R<Adt124> {
    let m541 = match v0.clone() {
        Adt103::C0(h540, _, _) => {
            h540
        }
    };
    match m541 {
        Adt104::C0 => {
            if f243(v2) {
                Ok(Adt124::C1(v0))
            } else {
                f244(v0, v1, v2)
            }
        }
        Adt104::C1 => {
            if f243(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a542 = v2;
                let a543 = 93u64;
                if nat_eq(a542, a543) {
                    Ok(f245(v0, v1))
                } else {
                    f244(v0, v1, v2)
                }
            }
        }
        Adt104::C2 => {
            Ok(f246(v0, v1, v2))
        }
        Adt104::C3 => {
            if f243(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a544 = v2;
                let a545 = 125u64;
                if nat_eq(a544, a545) {
                    Ok(f247(v0, v1))
                } else {
                    let a546 = v2;
                    let a547 = 34u64;
                    if nat_eq(a546, a547) {
                        Ok(f248(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f249(Adt90::C2(v1)))
                    }
                }
            }
        }
        Adt104::C4 => {
            if f243(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a548 = v2;
                let a549 = 34u64;
                if nat_eq(a548, a549) {
                    Ok(f248(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f249(Adt90::C2(v1)))
                }
            }
        }
        Adt104::C5 => {
            if f243(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a550 = v2;
                let a551 = 58u64;
                if nat_eq(a550, a551) {
                    Ok(f248(v0, Adt104::C0))
                } else {
                    Ok(f249(Adt90::C2(v1)))
                }
            }
        }
        Adt104::C6(v3, v4, v5) => {
            f250(v0, v1, v2, v3, v4, v5)
        }
        Adt104::C7(v6, v7) => {
            f251(v0, v1, v2, v6, v7)
        }
        Adt104::C8(v8) => {
            f252(v0, v1, v2, v8)
        }
        Adt104::C9(_) => {
            Ok(f246(v0, v1, v2))
        }
    }
}

pub fn f185(v0: Str, p556: List<Str>) -> bool {
    let m555 = p556;
    {
        let mut m555 = m555;
        loop {
            let step = {
                let v1 = m555;
                let m552 = v1;
                match m552.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a553 = v0.clone();
                        let a554 = v2;
                        if equal(a553, a554) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
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

pub fn f186(v0: Str, v1: List<u64>) -> Option<Adt94> {
    if f253(v1) {
        Some(Adt94::C0(v0))
    } else {
        None::<Adt94>
    }
}

pub fn f187(v0: Str) -> R<List<u64>> {
    f69({
        let a557 = v0;
        utf8_encode(a557)
    })
}

pub fn f188(v0: Adt35) -> R<Result<List<Adt12>, Adt94>> {
    let m558 = f254(v0.clone());
    match m558 {
        None => {
            f255(v0.clone(), f256(f257(), match v0.clone() {
                Adt35::C0(h559, _, _, _, _, _, _, _) => {
                    h559
                }
            }, Adt6::C5({
                let m562 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h561, _, _, _) => {
                        h561
                    }
                };
                match m562 {
                    Adt5::C0(h560) => {
                        h560
                    }
                }
            }), List::cons(Adt125::C0(Adt6::C5({
                let m565 = match v0 {
                    Adt35::C0(_, _, _, _, h564, _, _, _) => {
                        h564
                    }
                };
                match m565 {
                    Adt5::C0(h563) => {
                        h563
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt125>::nil()), f258())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f189(v0: Adt6, p584: u64, p585: Adt111, p586: List<Adt112>) -> R<Adt109> {
    let m583 = (p584, (p585, p586));
    {
        let mut m583 = m583;
        loop {
            let step = {
                let (v1, (v2, v3)) = m583;
                let m566 = v1;
                if m566 == 0 {
                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C4))
                } else {
                    let v4 = m566 - 1;
                    let m567 = v3;
                    match m567.uncons() {
                        None => {
                            let m568 = v2;
                            match m568 {
                                Adt111::C0 => {
                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C0))
                                }
                                Adt111::C1(v5, v6) => {
                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m569 = v2.clone();
                            match m569 {
                                Adt111::C0 => {
                                    let m570 = v7.clone();
                                    match m570 {
                                        Adt112::C0(v9, v10, v11) => {
                                            let m571 = f259(v0.clone(), v9, v10, v11.clone())?;
                                            match m571 {
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
                                                        let a572 = v15;
                                                        let a573 = v8;
                                                        append_list(a572, a573)
                                                    };
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt112::C1(_, _, _) => {
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v8)))
                                        }
                                        Adt112::C2(_, _, _, _) => {
                                            let v24: Adt126 = f260(v7, v8)?;
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v24.clone() {
                                                Adt126::C0(h574, _) => {
                                                    h574
                                                }
                                            }, match v24 {
                                                Adt126::C0(_, h575) => {
                                                    h575
                                                }
                                            })))
                                        }
                                        Adt112::C3(v25) => {
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (Adt111::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt112::C4(_, _, _, _) => {
                                            let v39: Adt126 = f260(v7, v8)?;
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v39.clone() {
                                                Adt126::C0(h579, _) => {
                                                    h579
                                                }
                                            }, match v39 {
                                                Adt126::C0(_, h580) => {
                                                    h580
                                                }
                                            })))
                                        }
                                        Adt112::C5(v26, v27, v28, v29) => {
                                            let m576 = f261(v26, v27, v28, v29.clone());
                                            match m576 {
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
                                                        let a577 = v33;
                                                        let a578 = v8;
                                                        append_list(a577, a578)
                                                    };
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt111::C1(v40, v41) => {
                                    let v42: Adt126 = f262(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v42.clone() {
                                        Adt126::C0(h581, _) => {
                                            h581
                                        }
                                    }, match v42 {
                                        Adt126::C0(_, h582) => {
                                            h582
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190() -> u64 {
    67108864u64
}

pub fn f191(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f263({
        let m589 = match v1.clone() {
            Adt49::C0(h588, _) => {
                h588
            }
        };
        match m589 {
            Adt37::C0(h587, _, _) => {
                h587
            }
        }
    })? {
        f264(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f192(v0: List<Adt110>) -> R<Str> {
    let a590 = List::cons(Str::lit(""), f265(v0, List::<Str>::nil())?);
    let a591 = Str::lit("/");
    Ok(join(a590, a591))
}

pub fn f193(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f194(v0, 0i64)?, 0i64))
}

pub fn f194(p594: u64, p595: i64) -> R<i64> {
    let m593 = (p594, p595);
    {
        let mut m593 = m593;
        loop {
            let step = {
                let (v0, v1) = m593;
                let m592 = v0;
                if m592 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m592 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f266(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(p598: List<Adt44>, p599: List<Adt44>) -> List<Adt44> {
    let m597 = (p598, p599);
    {
        let mut m597 = m597;
        loop {
            let step = {
                let (v0, v1) = m597;
                let m596 = v0;
                match m596.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt44>), List<Adt44>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt44>), List<Adt44>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(p603: List<(Adt44, List<Adt46>)>, p604: List<Adt44>) -> List<Adt44> {
    let m602 = (p603, p604);
    {
        let mut m602 = m602;
        loop {
            let step = {
                let (v0, v1) = m602;
                let m600 = v0;
                match m600.uncons() {
                    None => {
                        Err::<(List<(Adt44, List<Adt46>)>, List<Adt44>), List<Adt44>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt44, List<Adt46>)>, List<Adt44>), List<Adt44>>((v3, List::cons({
                            let (h601, _) = v2;
                            h601
                        }, v1)))
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

pub fn f197(v0: Adt44, v1: Adt46) -> (Adt44, List<Adt46>) {
    (v0, List::cons(v1, List::<Adt46>::nil()))
}

pub fn f198(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt44 {
    Adt44::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt40>, None::<u64>, None::<Adt5>)
}

pub fn f199(v0: Str, v1: Str, v2: Str) -> Adt46 {
    Adt46::C1(Adt48::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f200(v0: Str) -> R<Option<Str>> {
    let m605 = f267(v0)?;
    match m605 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a606 = f268(f269(), v1);
            Ok(utf8_decode(a606))
        }
    }
}

pub fn f201(v0: Str) -> R<Option<Str>> {
    let m609 = f270(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a607 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a608 = Str::lit(" ");
        join(a607, a608)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m609 {
        Ok(v1) => {
            let a610 = v1;
            Ok(utf8_decode(a610))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f202(v0: Adt57) -> bool {
    let m612 = match v0 {
        Adt57::C0(_, _, _, h611) => {
            h611
        }
    };
    match m612 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f203(v0: Str, p619: List<(Str, Str)>) -> Option<Str> {
    let m618 = p619;
    {
        let mut m618 = m618;
        loop {
            let step = {
                let v1 = m618;
                let m613 = v1;
                match m613.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a615 = {
                            let (h614, _) = v2;
                            h614
                        };
                        let a616 = v0.clone();
                        if equal(a615, a616) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h617) = v2;
                                h617
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m618 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(p624: List<Str>, p625: u64) -> R<u64> {
    let m623 = (p624, p625);
    {
        let mut m623 = m623;
        loop {
            let step = {
                let (v0, v1) = m623;
                let m620 = v0;
                match m620.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a621 = v1;
                            let a622 = 1u64;
                            nat_add(a621, a622)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: List<Str>) -> List<Str> {
    f207(f271(v0, f206(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f206() -> u64 {
    100u64
}

pub fn f207(p628: List<Str>, p629: List<Str>) -> List<Str> {
    let m627 = (p628, p629);
    {
        let mut m627 = m627;
        loop {
            let step = {
                let (v0, v1) = m627;
                let m626 = v0;
                match m626.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m627 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Str, p632: List<Str>, p633: List<Str>) -> R<List<Str>> {
    let m631 = (p632, p633);
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v1, v2) = m631;
                let m630 = v1;
                match m630.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f272(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
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

pub fn f209() -> List<Str> {
    f207(f273(f97(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f210(p638: u64, p639: List<Adt114>, p640: List<u64>) -> R<Result<List<u64>, Adt90>> {
    let m637 = (p638, (p639, p640));
    {
        let mut m637 = m637;
        loop {
            let step = {
                let (v0, (v1, v2)) = m637;
                let m634 = v0;
                if m634 == 0 {
                    Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Err::<List<u64>, Adt90>(Adt90::C13)))
                } else {
                    let v3 = m634 - 1;
                    let m635 = v1;
                    match m635.uncons() {
                        None => {
                            Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Ok::<List<u64>, Adt90>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m636 = f274(v4, v5, v2)?;
                            match m636 {
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
                Ok(next) => { m637 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(p643: List<u64>, p644: List<u64>) -> List<u64> {
    let m642 = (p643, p644);
    {
        let mut m642 = m642;
        loop {
            let step = {
                let (v0, v1) = m642;
                let m641 = v0;
                match m641.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Adt91) -> Adt6 {
    let m645 = v0;
    match m645 {
        Adt91::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt91::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f213(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m646 = v0;
    match m646 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f214(v0: Adt116) -> R<Option<Adt5>> {
    let m648 = f275(match v0.clone() {
        Adt116::C0(h647, _, _) => {
            h647
        }
    })?;
    match m648 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a651 = v1;
                let a652 = {
                    let m650 = f276(v0);
                    match m650 {
                        Adt5::C0(h649) => {
                            h649
                        }
                    }
                };
                append_list(a651, a652)
            })))
        }
    }
}

pub fn f215(v0: Adt88, v1: Option<List<List<u64>>>) -> Adt88 {
    let m653 = v1;
    match m653 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt88::C0(match v0.clone() {
                Adt88::C0(h654, _, _, _) => {
                    h654
                }
            }, match v0.clone() {
                Adt88::C0(_, h655, _, _) => {
                    h655
                }
            }, f54(v2, List::<List<u64>>::nil()), match v0 {
                Adt88::C0(_, _, _, h656) => {
                    h656
                }
            })
        }
    }
}

pub fn f216() -> u64 {
    200u64
}

pub fn f217(v0: Adt91, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f165())), List::cons((Str::lit("id"), f212(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h657) => {
            h657
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f218(p661: List<Adt118>, p662: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m660 = (p661, p662);
    {
        let mut m660 = m660;
        loop {
            let step = {
                let (v0, v1) = m660;
                let m658 = v0;
                match m658.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m659 = f277(v2)?;
                        match m659 {
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
                Ok(next) => { m660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Adt119) -> u64 {
    let m663 = v0;
    match m663 {
        Adt119::C0(_) => {
            f63()
        }
        Adt119::C1(_, _) => {
            f63()
        }
        Adt119::C2(_) => {
            f63()
        }
        Adt119::C3(_) => {
            f278()
        }
        Adt119::C4(v6) => {
            f279(v6)
        }
    }
}

pub fn f220(v0: Adt119) -> R<Adt80> {
    Ok(Adt80::C0(f280(v0.clone()), f281(v0.clone()), f282(v0)?))
}

pub fn f221(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt127, Adt119>) -> R<Result<Adt116, Adt119>> {
    let m664 = v3;
    match m664 {
        Ok(v5) => {
            f283(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt116, Adt119>(v4))
        }
    }
}

pub fn f222(v0: Option<Adt96>, v1: List<Str>, v2: Adt95) -> R<Result<Adt127, Adt119>> {
    Ok(f284(f285(match v2.clone() {
        Adt95::C0(_, h665, _) => {
            h665
        }
    }), f286(v0, v1, v2)?))
}

pub fn f223(v0: Adt98, v1: Adt129) -> Adt121 {
    let m666 = v1;
    match m666 {
        Adt129::C0 => {
            f230()
        }
        Adt129::C1 => {
            Adt121::C0(None::<Str>, Some(Adt97::C6(v0)))
        }
        Adt129::C2 => {
            Adt121::C0(None::<Str>, Some(Adt97::C7(v0)))
        }
        Adt129::C3(v2) => {
            let m667 = f287(v2);
            match m667 {
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

pub fn f224(v0: List<Adt86>, v1: List<u64>) -> R<Adt129> {
    let m668 = f288(v0, v1, Adt130::C0)?;
    match m668 {
        Adt130::C0 => {
            Ok(Adt129::C0)
        }
        Adt130::C1(v2) => {
            let m669 = f289(v2);
            match m669 {
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

pub fn f225() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f226() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f227(v0: Str) -> bool {
    let m670 = f290(v0);
    match m670 {
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

pub fn f228(v0: Adt129) -> R<Adt129> {
    let m671 = v0.clone();
    match m671 {
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
            let m672 = f291(v1);
            match m672 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m673 = f292(v2)?;
                    match m673 {
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

pub fn f229() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f230() -> Adt121 {
    Adt121::C0(None::<Str>, None::<Adt97>)
}

pub fn f231(v0: Option<Adt97>, v1: Option<Adt97>) -> Option<Adt97> {
    let m674 = v0.clone();
    match m674 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f232(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt99>> {
    Ok(f293(f294(v0, v1, v2, List::<Adt99>::nil())?, List::<Adt99>::nil()))
}

pub fn f233(v0: Adt6) -> Option<Str> {
    let m675 = v0;
    match m675 {
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
    let m678 = f295(40u64, match v0.clone() {
        Adt7::C0(h676, _) => {
            h676
        }
    }, match v0 {
        Adt7::C0(_, h677) => {
            h677
        }
    })?;
    match m678 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f296(v1))
        }
    }
}

pub fn f236(v0: Str, p684: List<(Str, Adt6)>) -> Adt123 {
    let m683 = p684;
    {
        let mut m683 = m683;
        loop {
            let step = {
                let v1 = m683;
                let m679 = v1;
                match m679.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt123>(Adt123::C0)
                    }
                    Some((v2, v3)) => {
                        let m681 = f297({
                            let (h680, _) = v2;
                            h680
                        }, v0.clone());
                        match m681 {
                            Adt139::C0 => {
                                Err::<List<(Str, Adt6)>, Adt123>(Adt123::C1({
                                    let (_, h682) = v2;
                                    h682
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
                Ok(next) => { m683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Str, v1: Adt5) -> bool {
    f298(f177(v0, v1))
}

pub fn f238(v0: Adt91, v1: Adt5) -> Result<Adt95, Adt92> {
    let m685 = f177(Str::lit("method"), v1.clone());
    match m685 {
        Adt123::C0 => {
            f122(Some(v0), f178(v1))
        }
        Adt123::C1(v2) => {
            let m686 = f233(v2);
            match m686 {
                None => {
                    f122(Some(v0), Adt93::C4)
                }
                Some(v3) => {
                    f299(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f239(v0: u64) -> R<u64> {
    let a689 = {
        let a687 = v0;
        let a688 = 3u64;
        nat_mul(a687, a688)?
    };
    let a690 = 1u64;
    nat_add(a689, a690)
}

pub fn f240(v0: u64, v1: u64, v2: u64) -> bool {
    let a691 = v1;
    let a692 = v0;
    if nat_le(a691, a692) {
        let a693 = v0;
        let a694 = v2;
        nat_le(a693, a694)
    } else {
        false
    }
}

pub fn f241(v0: Adt106) -> R<Adt131> {
    if f300(match v0.clone() {
        Adt106::C0(_, h695, _, _, _, _, _) => {
            h695
        }
    }) {
        let v1: i64 = {
            let a701 = if match v0.clone() {
                Adt106::C0(_, _, _, _, _, h696, _) => {
                    h696
                }
            } {
                let a698 = match v0.clone() {
                    Adt106::C0(_, _, _, _, _, _, h697) => {
                        h697
                    }
                };
                int_neg(a698)?
            } else {
                match v0.clone() {
                    Adt106::C0(_, _, _, _, _, _, h699) => {
                        h699
                    }
                }
            };
            let a702 = match v0.clone() {
                Adt106::C0(_, _, _, _, h700, _, _) => {
                    h700
                }
            };
            int_sub(a701, a702)?
        };
        let m705 = {
            let a703 = v1;
            let a704 = -9999i64;
            compare(a703, a704)
        };
        if if match m705 {
            Ordering::Less => {
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
            let m708 = {
                let a706 = 9999i64;
                let a707 = v1;
                compare(a706, a707)
            };
            match m708 {
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
                Adt106::C0(h709, _, _, _, _, _, _) => {
                    h709
                }
            } {
                let a711 = match v0 {
                    Adt106::C0(_, _, h710, _, _, _, _) => {
                        h710
                    }
                };
                int_neg(a711)?
            } else {
                match v0 {
                    Adt106::C0(_, _, h712, _, _, _, _) => {
                        h712
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt131::C0)
    }
}

pub fn f242(v0: Adt6, v1: List<Adt108>, v2: u64) -> Adt103 {
    let m713 = v1.clone();
    match m713.uncons() {
        None => {
            Adt103::C0(Adt104::C9(v0), List::<Adt108>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m714 = v3;
            match m714 {
                Adt108::C0(v5) => {
                    Adt103::C0(Adt104::C2, List::cons(Adt108::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt108::C1(v6, v7) => {
                    let m715 = v7;
                    match m715 {
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

pub fn f243(v0: u64) -> bool {
    let a716 = v0;
    let a717 = 32u64;
    if nat_eq(a716, a717) {
        true
    } else {
        let a718 = v0;
        let a719 = 10u64;
        if nat_eq(a718, a719) {
            true
        } else {
            let a720 = v0;
            let a721 = 13u64;
            if nat_eq(a720, a721) {
                true
            } else {
                let a722 = v0;
                let a723 = 9u64;
                nat_eq(a722, a723)
            }
        }
    }
}

pub fn f244(v0: Adt103, v1: u64, v2: u64) -> R<Adt124> {
    let a724 = v2;
    let a725 = 34u64;
    if nat_eq(a724, a725) {
        Ok(f248(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), false)))
    } else {
        let a726 = v2;
        let a727 = 123u64;
        if nat_eq(a726, a727) {
            f301(v0, Adt108::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt104::C3, v1)
        } else {
            let a728 = v2;
            let a729 = 91u64;
            if nat_eq(a728, a729) {
                f301(v0, Adt108::C0(List::<Adt6>::nil()), Adt104::C1, v1)
            } else {
                let a730 = v2;
                let a731 = 116u64;
                if nat_eq(a730, a731) {
                    Ok(f248(v0, Adt104::C7(0u64, 1u64)))
                } else {
                    let a732 = v2;
                    let a733 = 102u64;
                    if nat_eq(a732, a733) {
                        Ok(f248(v0, Adt104::C7(1u64, 1u64)))
                    } else {
                        let a734 = v2;
                        let a735 = 110u64;
                        if nat_eq(a734, a735) {
                            Ok(f248(v0, Adt104::C7(2u64, 1u64)))
                        } else {
                            let a736 = v2;
                            let a737 = 45u64;
                            if nat_eq(a736, a737) {
                                Ok(f248(v0, f302(true, Adt107::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a738 = v2;
                                let a739 = 48u64;
                                if nat_eq(a738, a739) {
                                    Ok(f248(v0, f302(false, Adt107::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f240(v2, 49u64, 57u64) {
                                        Ok(f248(v0, f302(false, Adt107::C2, f303({
                                            let a740 = v2;
                                            let a741 = 48u64;
                                            nat_sub(a740, a741)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f249(Adt90::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f245(v0: Adt103, v1: u64) -> Adt124 {
    let m743 = match v0.clone() {
        Adt103::C0(_, h742, _) => {
            h742
        }
    };
    match m743.uncons() {
        None => {
            f249(Adt90::C2(v1))
        }
        Some((v2, v3)) => {
            let m744 = v2;
            match m744 {
                Adt108::C0(v4) => {
                    Adt124::C1(f242(Adt6::C4(f304(v4, List::<Adt6>::nil())), v3, {
                        let a746 = match v0 {
                            Adt103::C0(_, _, h745) => {
                                h745
                            }
                        };
                        let a747 = 1u64;
                        nat_sub(a746, a747)
                    }))
                }
                Adt108::C1(_, _) => {
                    f249(Adt90::C2(v1))
                }
            }
        }
    }
}

pub fn f246(v0: Adt103, v1: u64, v2: u64) -> Adt124 {
    let m749 = match v0.clone() {
        Adt103::C0(h748, _, _) => {
            h748
        }
    };
    match m749 {
        Adt104::C0 => {
            f249(Adt90::C2(v1))
        }
        Adt104::C1 => {
            f249(Adt90::C2(v1))
        }
        Adt104::C2 => {
            if f243(v2) {
                Adt124::C1(v0)
            } else {
                let a750 = v2;
                let a751 = 44u64;
                if nat_eq(a750, a751) {
                    f305(v0, v1)
                } else {
                    let a752 = v2;
                    let a753 = 93u64;
                    if nat_eq(a752, a753) {
                        f245(v0, v1)
                    } else {
                        let a754 = v2;
                        let a755 = 125u64;
                        if nat_eq(a754, a755) {
                            f247(v0, v1)
                        } else {
                            f249(Adt90::C2(v1))
                        }
                    }
                }
            }
        }
        Adt104::C3 => {
            f249(Adt90::C2(v1))
        }
        Adt104::C4 => {
            f249(Adt90::C2(v1))
        }
        Adt104::C5 => {
            f249(Adt90::C2(v1))
        }
        Adt104::C6(_, _, _) => {
            f249(Adt90::C2(v1))
        }
        Adt104::C7(_, _) => {
            f249(Adt90::C2(v1))
        }
        Adt104::C8(_) => {
            f249(Adt90::C2(v1))
        }
        Adt104::C9(_) => {
            if f243(v2) {
                Adt124::C1(v0)
            } else {
                f249(Adt90::C10(v1))
            }
        }
    }
}

pub fn f247(v0: Adt103, v1: u64) -> Adt124 {
    let m757 = match v0.clone() {
        Adt103::C0(_, h756, _) => {
            h756
        }
    };
    match m757.uncons() {
        None => {
            f249(Adt90::C2(v1))
        }
        Some((v2, v3)) => {
            let m758 = v2;
            match m758 {
                Adt108::C0(_) => {
                    f249(Adt90::C2(v1))
                }
                Adt108::C1(v5, _) => {
                    Adt124::C1(f242(Adt6::C5(f306(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a760 = match v0 {
                            Adt103::C0(_, _, h759) => {
                                h759
                            }
                        };
                        let a761 = 1u64;
                        nat_sub(a760, a761)
                    }))
                }
            }
        }
    }
}

pub fn f248(v0: Adt103, v1: Adt104) -> Adt124 {
    Adt124::C1(f307(v0, v1))
}

pub fn f249(v0: Adt90) -> Adt124 {
    Adt124::C0(v0)
}

pub fn f250(v0: Adt103, v1: u64, v2: u64, v3: Adt105, v4: List<u64>, v5: bool) -> R<Adt124> {
    let m762 = v3;
    match m762 {
        Adt105::C0 => {
            let a763 = v2;
            let a764 = 34u64;
            if nat_eq(a763, a764) {
                Ok(f308(v0, v1, v4, v5))
            } else {
                let a765 = v2;
                let a766 = 92u64;
                if nat_eq(a765, a766) {
                    Ok(f248(v0, Adt104::C6(Adt105::C1, v4, v5)))
                } else {
                    let a767 = v2;
                    let a768 = 32u64;
                    if nat_lt(a767, a768) {
                        Ok(f249(Adt90::C4(v1)))
                    } else {
                        Ok(f248(v0, Adt104::C6(Adt105::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt105::C1 => {
            let a769 = v2;
            let a770 = 117u64;
            if nat_eq(a769, a770) {
                Ok(f248(v0, Adt104::C6(Adt105::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a771 = f309(v2);
                let a772 = 0u64;
                if nat_eq(a771, a772) {
                    Ok(f249(Adt90::C5(v1)))
                } else {
                    Ok(f248(v0, Adt104::C6(Adt105::C0, List::cons(f309(v2), v4), v5)))
                }
            }
        }
        Adt105::C2(v6, v7, v8) => {
            let a773 = 15u64;
            let a774 = f310(v2);
            if nat_lt(a773, a774) {
                Ok(f249(Adt90::C5(v1)))
            } else {
                let a777 = {
                    let a775 = v6;
                    let a776 = 1u64;
                    nat_add(a775, a776)?
                };
                let a778 = 4u64;
                if nat_eq(a777, a778) {
                    f311(v0, v1, {
                        let a781 = {
                            let a779 = v7;
                            let a780 = 16u64;
                            nat_mul(a779, a780)?
                        };
                        let a782 = f310(v2);
                        nat_add(a781, a782)?
                    }, v8, v4, v5)
                } else {
                    Ok(f248(v0, Adt104::C6(Adt105::C2({
                        let a783 = v6;
                        let a784 = 1u64;
                        nat_add(a783, a784)?
                    }, {
                        let a787 = {
                            let a785 = v7;
                            let a786 = 16u64;
                            nat_mul(a785, a786)?
                        };
                        let a788 = f310(v2);
                        nat_add(a787, a788)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt105::C3(v9) => {
            let a789 = v2;
            let a790 = 92u64;
            if nat_eq(a789, a790) {
                Ok(f248(v0, Adt104::C6(Adt105::C4(v9), v4, v5)))
            } else {
                Ok(f249(Adt90::C6(v1)))
            }
        }
        Adt105::C4(v10) => {
            let a791 = v2;
            let a792 = 117u64;
            if nat_eq(a791, a792) {
                Ok(f248(v0, Adt104::C6(Adt105::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f249(Adt90::C6(v1)))
            }
        }
    }
}

pub fn f251(v0: Adt103, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt124> {
    let a793 = v2;
    let a794 = f312(v3, v4);
    if nat_eq(a793, a794) {
        let a798 = {
            let a795 = v4;
            let a796 = 1u64;
            nat_add(a795, a796)?
        };
        let a799 = {
            let a797 = f313(v3);
            length_list(a797)
        };
        if nat_eq(a798, a799) {
            Ok(f314(v0, f315(v3)))
        } else {
            Ok(f248(v0, Adt104::C7(v3, {
                let a800 = v4;
                let a801 = 1u64;
                nat_add(a800, a801)?
            })))
        }
    } else {
        Ok(f249(Adt90::C2(v1)))
    }
}

pub fn f252(v0: Adt103, v1: u64, v2: u64, v3: Adt106) -> R<Adt124> {
    let m803 = match v3.clone() {
        Adt106::C0(_, h802, _, _, _, _, _) => {
            h802
        }
    };
    match m803 {
        Adt107::C0 => {
            let a804 = v2;
            let a805 = 48u64;
            if nat_eq(a804, a805) {
                Ok(f316(v0, v3, Adt107::C1))
            } else {
                if f240(v2, 49u64, 57u64) {
                    f317(v0, v1, v2, v3, false)
                } else {
                    Ok(f249(Adt90::C7(v1)))
                }
            }
        }
        Adt107::C1 => {
            let a806 = v2;
            let a807 = 46u64;
            if nat_eq(a806, a807) {
                Ok(f316(v0, v3, Adt107::C3))
            } else {
                if f318(v2) {
                    Ok(f316(v0, v3, Adt107::C5))
                } else {
                    if f319(v2) {
                        Ok(f249(Adt90::C7(v1)))
                    } else {
                        f320(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt107::C2 => {
            if f319(v2) {
                f317(v0, v1, v2, v3, false)
            } else {
                let a808 = v2;
                let a809 = 46u64;
                if nat_eq(a808, a809) {
                    Ok(f316(v0, v3, Adt107::C3))
                } else {
                    if f318(v2) {
                        Ok(f316(v0, v3, Adt107::C5))
                    } else {
                        f320(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt107::C3 => {
            if f319(v2) {
                f317(v0, v1, v2, v3, true)
            } else {
                Ok(f249(Adt90::C7(v1)))
            }
        }
        Adt107::C4 => {
            if f319(v2) {
                f317(v0, v1, v2, v3, true)
            } else {
                if f318(v2) {
                    Ok(f316(v0, v3, Adt107::C5))
                } else {
                    f320(v0, v1, v2, v3)
                }
            }
        }
        Adt107::C5 => {
            let a810 = v2;
            let a811 = 43u64;
            if nat_eq(a810, a811) {
                Ok(f248(v0, f302(match v3.clone() {
                    Adt106::C0(h812, _, _, _, _, _, _) => {
                        h812
                    }
                }, Adt107::C6, match v3.clone() {
                    Adt106::C0(_, _, h813, _, _, _, _) => {
                        h813
                    }
                }, match v3.clone() {
                    Adt106::C0(_, _, _, h814, _, _, _) => {
                        h814
                    }
                }, match v3 {
                    Adt106::C0(_, _, _, _, h815, _, _) => {
                        h815
                    }
                }, false, 0i64)))
            } else {
                let a816 = v2;
                let a817 = 45u64;
                if nat_eq(a816, a817) {
                    Ok(f248(v0, f302(match v3.clone() {
                        Adt106::C0(h818, _, _, _, _, _, _) => {
                            h818
                        }
                    }, Adt107::C6, match v3.clone() {
                        Adt106::C0(_, _, h819, _, _, _, _) => {
                            h819
                        }
                    }, match v3.clone() {
                        Adt106::C0(_, _, _, h820, _, _, _) => {
                            h820
                        }
                    }, match v3 {
                        Adt106::C0(_, _, _, _, h821, _, _) => {
                            h821
                        }
                    }, true, 0i64)))
                } else {
                    if f319(v2) {
                        f321(v0, v1, v2, Adt106::C0(match v3.clone() {
                            Adt106::C0(h822, _, _, _, _, _, _) => {
                                h822
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, h823, _, _, _, _, _) => {
                                h823
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, _, h824, _, _, _, _) => {
                                h824
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, _, _, h825, _, _, _) => {
                                h825
                            }
                        }, match v3 {
                            Adt106::C0(_, _, _, _, h826, _, _) => {
                                h826
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f249(Adt90::C7(v1)))
                    }
                }
            }
        }
        Adt107::C6 => {
            if f319(v2) {
                f321(v0, v1, v2, v3.clone(), match v3 {
                    Adt106::C0(_, _, _, _, _, h827, _) => {
                        h827
                    }
                })
            } else {
                Ok(f249(Adt90::C7(v1)))
            }
        }
        Adt107::C7 => {
            if f319(v2) {
                f321(v0, v1, v2, v3.clone(), match v3 {
                    Adt106::C0(_, _, _, _, _, h828, _) => {
                        h828
                    }
                })
            } else {
                f320(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f253(v0: List<u64>) -> bool {
    let m829 = v0.clone();
    match m829.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a833 = if f322(v0.clone(), false) {
                let a831 = {
                    let a830 = v0;
                    length_list(a830)
                };
                let a832 = f323();
                nat_le(a831, a832)
            } else {
                false
            };
            bool_not(a833)
        }
    }
}

pub fn f254(v0: Adt35) -> Option<Adt94> {
    let m838 = f324(match v0.clone() {
        Adt35::C0(h834, _, _, _, _, _, _, _) => {
            h834
        }
    }, {
        let m837 = match v0.clone() {
            Adt35::C0(_, _, _, _, h836, _, _, _) => {
                h836
            }
        };
        match m837 {
            Adt5::C0(h835) => {
                h835
            }
        }
    });
    match m838 {
        None => {
            f325(match v0.clone() {
                Adt35::C0(h839, _, _, _, _, _, _, _) => {
                    h839
                }
            }, {
                let m842 = match v0 {
                    Adt35::C0(_, _, _, _, h841, _, _, _) => {
                        h841
                    }
                };
                match m842 {
                    Adt5::C0(h840) => {
                        h840
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f255(v0: Adt35, v1: Result<Adt132, Adt94>) -> R<Result<List<Adt12>, Adt94>> {
    let m843 = v1;
    match m843 {
        Ok(v3) => {
            let m844 = f326(v0)?;
            match m844 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt94>(f128(match v3 {
                        Adt132::C0(h845, _) => {
                            h845
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

pub fn f256(p852: u64, v1: Str, v2: Adt6, p853: List<Adt125>, p854: Adt132) -> R<Result<Adt132, Adt94>> {
    let m851 = (p852, (p853, p854));
    {
        let mut m851 = m851;
        loop {
            let step = {
                let (v0, (v3, v4)) = m851;
                let m846 = v0;
                if m846 == 0 {
                    Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Err::<Adt132, Adt94>(Adt94::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m846 - 1;
                    let m847 = v3;
                    match m847.uncons() {
                        None => {
                            Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Ok::<Adt132, Adt94>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m848 = f327(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m848 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>((v5, ({
                                        let (h849, _) = v9;
                                        h849
                                    }, {
                                        let (_, h850) = v9;
                                        h850
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
                Ok(next) => { m851 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f257() -> u64 {
    67108864u64
}

pub fn f258() -> Adt132 {
    Adt132::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f259(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    let m855 = f328(v1.clone());
    match m855 {
        None => {
            let m856 = f329(v1);
            match m856 {
                None => {
                    Ok(Adt133::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f330(v0, match v5 {
                        Adt5::C0(h857) => {
                            h857
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

pub fn f260(v0: Adt112, v1: List<Adt112>) -> R<Adt126> {
    let m858 = v0;
    match m858 {
        Adt112::C0(_, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
        Adt112::C1(_, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
        Adt112::C2(v8, v9, v10, v11) => {
            Ok(f331(v8, v9, v10, {
                let a859 = v11;
                let a860 = 1u64;
                nat_add(a859, a860)?
            }, v1))
        }
        Adt112::C3(v12) => {
            Ok(Adt126::C0(Adt111::C1(v12, Str::lit("not")), v1))
        }
        Adt112::C4(v13, _, v15, v16) => {
            Ok(f332(v13, v15, v16, v1))
        }
        Adt112::C5(_, _, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
    }
}

pub fn f261(v0: Adt113, v1: Adt6, v2: Adt6, v3: List<Adt110>) -> Adt133 {
    let m861 = f329(v1);
    match m861 {
        None => {
            Adt133::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m862 = v0;
            match m862 {
                Adt113::C0 => {
                    f333(match v4 {
                        Adt5::C0(h863) => {
                            h863
                        }
                    }, v2, v3)
                }
                Adt113::C1 => {
                    f334(match v4 {
                        Adt5::C0(h864) => {
                            h864
                        }
                    }, v2, v3)
                }
                Adt113::C2 => {
                    f335(match v4 {
                        Adt5::C0(h865) => {
                            h865
                        }
                    }, v2, v3)
                }
                Adt113::C3 => {
                    f336(match v4 {
                        Adt5::C0(h866) => {
                            h866
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f262(v0: Adt112, v1: List<Adt112>, v2: List<Adt110>, v3: Str) -> Adt126 {
    let m867 = v0;
    match m867 {
        Adt112::C0(_, _, _) => {
            Adt126::C0(Adt111::C1(v2, v3), v1)
        }
        Adt112::C1(v7, v8, v9) => {
            f337(v7, v8, v9, v1)
        }
        Adt112::C2(v10, v11, v12, v13) => {
            f331(v10, v11, v12, v13, v1)
        }
        Adt112::C3(_) => {
            Adt126::C0(Adt111::C0, v1)
        }
        Adt112::C4(_, v16, v17, v18) => {
            f332(v16, v17, v18, v1)
        }
        Adt112::C5(_, _, _, _) => {
            Adt126::C0(Adt111::C1(v2, v3), v1)
        }
    }
}

pub fn f263(p876: List<Adt38>) -> R<bool> {
    let m875 = p876;
    {
        let mut m875 = m875;
        loop {
            let step = {
                let v0 = m875;
                let m868 = v0;
                match m868.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m869 = v1;
                        match m869 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f338(match v4 {
                                    Adt42::C0(h870, _, _, _) => {
                                        h870
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f338(match v5 {
                                    Adt43::C0(h871, _, _, _) => {
                                        h871
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
                                let m873 = match v7 {
                                    Adt45::C0(h872, _, _) => {
                                        h872
                                    }
                                };
                                match m873 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f338(match v9 {
                                            Adt48::C0(_, _, h874, _) => {
                                                h874
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
                Ok(next) => { m875 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f339(match v1.clone() {
        Adt49::C0(h877, _) => {
            h877
        }
    }) {
        f340(v1)
    } else {
        let m879 = match v0 {
            Adt35::C0(_, _, _, _, _, h878, _, _) => {
                h878
            }
        };
        match m879 {
            None => {
                f340(v1)
            }
            Some(v2) => {
                f341(v2, v1)
            }
        }
    }
}

pub fn f265(p882: List<Adt110>, p883: List<Str>) -> R<List<Str>> {
    let m881 = (p882, p883);
    {
        let mut m881 = m881;
        loop {
            let step = {
                let (v0, v1) = m881;
                let m880 = v0;
                match m880.uncons() {
                    None => {
                        Err::<(List<Adt110>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt110>, List<Str>), R<List<Str>>>((v3, List::cons(f342(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: i64, v1: i64) -> R<i64> {
    let a885 = v0;
    let a886 = {
        let a884 = v1;
        int_neg(a884)?
    };
    int_sub(a885, a886)
}

pub fn f267(v0: Str) -> R<Option<List<u64>>> {
    let m887 = f343(f344(f345())?, f344(v0)?);
    match m887 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m888 = f343(f346(List::cons(47u64, f344(f347())?)), f346(v1));
            match m888 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f346(v2);
                    if if f348(v3.clone()) {
                        f349(v3.clone())
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

pub fn f268(v0: Bytes, v1: List<u64>) -> Bytes {
    let a889 = v0;
    let a890 = f350(f269(), f351(List::<Bytes>::nil(), v1));
    append_bytes(a889, a890)
}

pub fn f269() -> Bytes {
    Bytes::lit(&[])
}

pub fn f270(v0: Adt6) -> R<Result<Bytes, Adt90>> {
    let m891 = f114(v0)?;
    match m891 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt90>(f268(f269(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt90>(v2))
        }
    }
}

pub fn f271(p895: List<Str>, p896: u64, p897: List<Str>) -> List<Str> {
    let m894 = (p895, (p896, p897));
    {
        let mut m894 = m894;
        loop {
            let step = {
                let (v0, (v1, v2)) = m894;
                let m892 = v0;
                match m892.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m893 = v1;
                        if m893 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m893 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272(v0: Str, v1: Str) -> R<bool> {
    Ok(f352(f344(v0)?, f344(v1)?))
}

pub fn f273(p901: List<Adt44>, p902: List<Str>) -> List<Str> {
    let m900 = (p901, p902);
    {
        let mut m900 = m900;
        loop {
            let step = {
                let (v0, v1) = m900;
                let m898 = v0;
                match m898.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt44::C0(_, _, _, h899, _, _, _, _, _) => {
                                h899
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: Adt114, v1: List<Adt114>, v2: List<u64>) -> R<Adt134> {
    let m903 = v0;
    match m903 {
        Adt114::C0(v3) => {
            Ok(Adt134::C1(v1, f211(v3, v2)))
        }
        Adt114::C1(v4, v5) => {
            f353(v4, v5, v1, v2)
        }
        Adt114::C2(v6, v7) => {
            let m904 = v7;
            match m904.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C1(v6, v8), List::cons(Adt114::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt114::C3(v10, v11) => {
            let m905 = v11.clone();
            match m905.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt114::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt114::C4(v14, v15) => {
            let m906 = v15;
            match m906.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0({
                        let a908 = f354({
                            let (h907, _) = v16;
                            h907
                        })?;
                        let a909 = List::cons(58u64, List::<u64>::nil());
                        append_list(a908, a909)
                    }), List::cons(Adt114::C1(v14, {
                        let (_, h910) = v16;
                        h910
                    }), List::cons(Adt114::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt114::C5(v18, v19) => {
            let m911 = v19.clone();
            match m911.uncons() {
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

pub fn f275(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m912 = v0;
    match m912 {
        Adt32::C0(v1) => {
            Ok(Some(f355(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f356(v2)?))
        }
        Adt32::C2(v3) => {
            f357(f358(v3)?)
        }
        Adt32::C3(v4) => {
            f357(f358(match v4 {
                Adt49::C0(h913, _) => {
                    h913
                }
            })?)
        }
        Adt32::C4(v5) => {
            Ok(Some(f359(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f360(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f361(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f362(v8)?))
        }
        Adt32::C8(v9) => {
            f357(f363(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f364(v10)?))
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

pub fn f276(v0: Adt116) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f365(f366(match v0.clone() {
        Adt116::C0(h914, _, _) => {
            h914
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f367({
        let m917 = match v0 {
            Adt116::C0(_, h916, _) => {
                h916
            }
        };
        match m917 {
            Adt117::C0(h915, _) => {
                h915
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f277(v0: Adt118) -> R<Option<List<u64>>> {
    let m918 = f368(f369(v0))?;
    match m918 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m919 = f114(v1)?;
            match m919 {
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

pub fn f278() -> u64 {
    404u64
}

pub fn f279(v0: Adt84) -> u64 {
    let m920 = v0;
    match m920 {
        Adt84::C0(_) => {
            f63()
        }
        Adt84::C1(_) => {
            f63()
        }
        Adt84::C2(_) => {
            f63()
        }
        Adt84::C3(_) => {
            f63()
        }
        Adt84::C4(_) => {
            f63()
        }
        Adt84::C5(_) => {
            f63()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f370()
        }
        Adt84::C8(_) => {
            f63()
        }
        Adt84::C9(_) => {
            f21()
        }
    }
}

pub fn f280(v0: Adt119) -> i64 {
    let m921 = v0;
    match m921 {
        Adt119::C0(_) => {
            f371()
        }
        Adt119::C1(_, _) => {
            f372()
        }
        Adt119::C2(_) => {
            f373()
        }
        Adt119::C3(_) => {
            f374()
        }
        Adt119::C4(v6) => {
            f375(v6)
        }
    }
}

pub fn f281(v0: Adt119) -> Str {
    let m922 = v0;
    match m922 {
        Adt119::C0(v1) => {
            f376(v1)
        }
        Adt119::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt119::C2(v4) => {
            f377(v4)
        }
        Adt119::C3(_) => {
            Str::lit("Method not found")
        }
        Adt119::C4(v6) => {
            let m923 = v6;
            match m923 {
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

pub fn f282(v0: Adt119) -> R<Option<Adt6>> {
    let m924 = v0;
    match m924 {
        Adt119::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f378(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt119::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C4(v6) => {
            let m925 = v6;
            match m925 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f194(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f379(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f283(v0: Adt0, v1: Fn0, v2: u64, v3: Adt127) -> R<Result<Adt116, Adt119>> {
    f380(v0.clone(), v1, v2, v3.clone(), f381(match v0 {
        Adt0::C0(_, _, h926, _, _, _) => {
            h926
        }
    }, match v3 {
        Adt127::C0(_, h927, _, _) => {
            h927
        }
    }))
}

pub fn f284(v0: Adt135, v1: Adt136) -> Result<Adt127, Adt119> {
    let m928 = v1;
    match m928 {
        Adt136::C0(v3) => {
            f382(v3, v0)
        }
        Adt136::C1(v2) => {
            Err::<Adt127, Adt119>(v2)
        }
    }
}

pub fn f285(v0: Str) -> Adt135 {
    let a929 = v0.clone();
    let a930 = Str::lit("server/discover");
    if equal(a929, a930) {
        Adt135::C1(Adt128::C0)
    } else {
        let a931 = v0.clone();
        let a932 = Str::lit("tools/list");
        if equal(a931, a932) {
            Adt135::C1(Adt128::C1)
        } else {
            let a933 = v0.clone();
            let a934 = Str::lit("tools/call");
            if equal(a933, a934) {
                Adt135::C1(Adt128::C2)
            } else {
                let a935 = v0.clone();
                let a936 = Str::lit("resources/list");
                if equal(a935, a936) {
                    Adt135::C1(Adt128::C3)
                } else {
                    let a937 = v0.clone();
                    let a938 = Str::lit("resources/read");
                    if equal(a937, a938) {
                        Adt135::C1(Adt128::C4)
                    } else {
                        let a939 = v0.clone();
                        let a940 = Str::lit("resources/templates/list");
                        if equal(a939, a940) {
                            Adt135::C1(Adt128::C5)
                        } else {
                            let a941 = v0.clone();
                            let a942 = Str::lit("prompts/list");
                            if equal(a941, a942) {
                                Adt135::C1(Adt128::C6)
                            } else {
                                let a943 = v0.clone();
                                let a944 = Str::lit("prompts/get");
                                if equal(a943, a944) {
                                    Adt135::C1(Adt128::C7)
                                } else {
                                    let a945 = v0.clone();
                                    let a946 = Str::lit("completion/complete");
                                    if equal(a945, a946) {
                                        Adt135::C1(Adt128::C8)
                                    } else {
                                        let a947 = v0.clone();
                                        let a948 = Str::lit("tasks/get");
                                        if equal(a947, a948) {
                                            Adt135::C1(Adt128::C9)
                                        } else {
                                            let a949 = v0.clone();
                                            let a950 = Str::lit("tasks/update");
                                            if equal(a949, a950) {
                                                Adt135::C1(Adt128::C10)
                                            } else {
                                                let a951 = v0;
                                                let a952 = Str::lit("tasks/cancel");
                                                if equal(a951, a952) {
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

pub fn f286(v0: Option<Adt96>, v1: List<Str>, v2: Adt95) -> R<Adt136> {
    f383(v0, v1, v2.clone(), f384(match v2 {
        Adt95::C0(_, _, h953) => {
            h953
        }
    }))
}

pub fn f287(v0: List<u64>) -> Option<Str> {
    let a954 = f268(f269(), v0);
    utf8_decode(a954)
}

pub fn f288(p959: List<Adt86>, v1: List<u64>, p960: Adt130) -> R<Adt130> {
    let m958 = (p959, p960);
    {
        let mut m958 = m958;
        loop {
            let step = {
                let (v0, v2) = m958;
                let m955 = v0;
                match m955.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt130), R<Adt130>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f385(match v3.clone() {
                            Adt86::C0(h956, _) => {
                                h956
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt130), R<Adt130>>((v4, f386(v2, match v3 {
                                Adt86::C0(_, h957) => {
                                    h957
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt130), R<Adt130>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m958 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f387(v0);
    if f388(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f290(v0: Str) -> Adt138 {
    let m961 = f285(v0);
    match m961 {
        Adt135::C0 => {
            Adt138::C0
        }
        Adt135::C1(v1) => {
            f389(v1)
        }
    }
}

pub fn f291(v0: List<u64>) -> Option<List<u64>> {
    let m962 = f390(f391(), v0);
    match m962 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m963 = f390(f392(), f393(v1, List::<u64>::nil()));
            match m963 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f393(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f292(v0: List<u64>) -> R<Option<List<u64>>> {
    let m965 = f394({
        let a964 = v0.clone();
        length_list(a964)
    }, v0, List::<u64>::nil())?;
    match m965 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f211(v1, List::<u64>::nil())))
        }
    }
}

pub fn f293(p968: List<Adt99>, p969: List<Adt99>) -> List<Adt99> {
    let m967 = (p968, p969);
    {
        let mut m967 = m967;
        loop {
            let step = {
                let (v0, v1) = m967;
                let m966 = v0;
                match m966.uncons() {
                    None => {
                        Err::<(List<Adt99>, List<Adt99>), List<Adt99>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt99>, List<Adt99>), List<Adt99>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: List<Adt86>, p976: List<Adt12>, v2: Option<Str>, p977: List<Adt99>) -> R<List<Adt99>> {
    let m975 = (p976, p977);
    {
        let mut m975 = m975;
        loop {
            let step = {
                let (v1, v3) = m975;
                let m970 = v1;
                match m970.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f395(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>((v5, List::cons(Adt99::C0(match v4.clone() {
                                Adt12::C0(_, h971, _, _) => {
                                    h971
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h972, _) => {
                                    h972
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h973) => {
                                    h973
                                }
                            }, f396(v0.clone(), match v4 {
                                Adt12::C0(_, h974, _, _) => {
                                    h974
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m975 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(p993: u64, p994: i64, p995: i64) -> R<Option<i64>> {
    let m992 = (p993, (p994, p995));
    {
        let mut m992 = m992;
        loop {
            let step = {
                let (v0, (v1, v2)) = m992;
                let m978 = v0;
                if m978 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m978 - 1;
                    if f397(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m981 = {
                            let a979 = v2;
                            let a980 = 0i64;
                            compare(a979, a980)
                        };
                        if match m981 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f397({
                                let a982 = v1;
                                let a983 = 10i64;
                                let a984 = 0i64;
                                int_rem(a982, a983, a984)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a985 = v1;
                                    let a986 = 10i64;
                                    let a987 = 0i64;
                                    int_quot(a985, a986, a987)?
                                }, f266(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a988 = v1;
                                let a989 = 10i64;
                                int_mul(a988, a989)?
                            }, {
                                let a990 = v2;
                                let a991 = 1i64;
                                int_sub(a990, a991)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: i64) -> Option<i64> {
    let m998 = {
        let a996 = 9007199254740991i64;
        let a997 = v0;
        compare(a996, a997)
    };
    if if match m998 {
        Ordering::Less => {
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
        let m1001 = {
            let a999 = v0;
            let a1000 = -9007199254740991i64;
            compare(a999, a1000)
        };
        match m1001 {
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

pub fn f297(v0: Str, v1: Str) -> Adt139 {
    let a1002 = v0;
    let a1003 = v1;
    if equal(a1002, a1003) {
        Adt139::C0
    } else {
        Adt139::C1
    }
}

pub fn f298(v0: Adt123) -> bool {
    let m1004 = v0;
    match m1004 {
        Adt123::C0 => {
            false
        }
        Adt123::C1(_) => {
            true
        }
    }
}

pub fn f299(v0: Adt91, v1: Str, v2: Adt5) -> Result<Adt95, Adt92> {
    let m1005 = f177(Str::lit("params"), v2);
    match m1005 {
        Adt123::C0 => {
            Ok::<Adt95, Adt92>(Adt95::C0(v0, v1, None::<Adt5>))
        }
        Adt123::C1(v3) => {
            let m1006 = f329(v3);
            match m1006 {
                None => {
                    f122(Some(v0), Adt93::C6)
                }
                Some(v4) => {
                    Ok::<Adt95, Adt92>(Adt95::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f300(v0: Adt107) -> bool {
    let m1007 = v0;
    match m1007 {
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

pub fn f301(v0: Adt103, v1: Adt108, v2: Adt104, v3: u64) -> R<Adt124> {
    let a1009 = match v0.clone() {
        Adt103::C0(_, _, h1008) => {
            h1008
        }
    };
    let a1010 = f398();
    if nat_lt(a1009, a1010) {
        Ok(Adt124::C1(Adt103::C0(v2, List::cons(v1, match v0.clone() {
            Adt103::C0(_, h1011, _) => {
                h1011
            }
        }), {
            let a1013 = match v0 {
                Adt103::C0(_, _, h1012) => {
                    h1012
                }
            };
            let a1014 = 1u64;
            nat_add(a1013, a1014)?
        })))
    } else {
        Ok(f249(Adt90::C9(v3)))
    }
}

pub fn f302(v0: bool, v1: Adt107, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt104 {
    Adt104::C8(Adt106::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f303(v0: u64) -> i64 {
    let a1015 = v0;
    let a1016 = 0u64;
    if nat_eq(a1015, a1016) {
        0i64
    } else {
        let a1017 = v0;
        let a1018 = 1u64;
        if nat_eq(a1017, a1018) {
            1i64
        } else {
            let a1019 = v0;
            let a1020 = 2u64;
            if nat_eq(a1019, a1020) {
                2i64
            } else {
                let a1021 = v0;
                let a1022 = 3u64;
                if nat_eq(a1021, a1022) {
                    3i64
                } else {
                    let a1023 = v0;
                    let a1024 = 4u64;
                    if nat_eq(a1023, a1024) {
                        4i64
                    } else {
                        let a1025 = v0;
                        let a1026 = 5u64;
                        if nat_eq(a1025, a1026) {
                            5i64
                        } else {
                            let a1027 = v0;
                            let a1028 = 6u64;
                            if nat_eq(a1027, a1028) {
                                6i64
                            } else {
                                let a1029 = v0;
                                let a1030 = 7u64;
                                if nat_eq(a1029, a1030) {
                                    7i64
                                } else {
                                    let a1031 = v0;
                                    let a1032 = 8u64;
                                    if nat_eq(a1031, a1032) {
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

pub fn f304(p1035: List<Adt6>, p1036: List<Adt6>) -> List<Adt6> {
    let m1034 = (p1035, p1036);
    {
        let mut m1034 = m1034;
        loop {
            let step = {
                let (v0, v1) = m1034;
                let m1033 = v0;
                match m1033.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: Adt103, v1: u64) -> Adt124 {
    let m1038 = match v0.clone() {
        Adt103::C0(_, h1037, _) => {
            h1037
        }
    };
    match m1038.uncons() {
        None => {
            f249(Adt90::C2(v1))
        }
        Some((v2, _)) => {
            let m1039 = v2;
            match m1039 {
                Adt108::C0(_) => {
                    f248(v0, Adt104::C0)
                }
                Adt108::C1(_, _) => {
                    f248(v0, Adt104::C4)
                }
            }
        }
    }
}

pub fn f306(p1042: List<(Str, Adt6)>, p1043: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1041 = (p1042, p1043);
    {
        let mut m1041 = m1041;
        loop {
            let step = {
                let (v0, v1) = m1041;
                let m1040 = v0;
                match m1040.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307(v0: Adt103, v1: Adt104) -> Adt103 {
    Adt103::C0(v1, match v0.clone() {
        Adt103::C0(_, h1044, _) => {
            h1044
        }
    }, match v0 {
        Adt103::C0(_, _, h1045) => {
            h1045
        }
    })
}

pub fn f308(v0: Adt103, v1: u64, v2: List<u64>, v3: bool) -> Adt124 {
    let m1047 = {
        let a1046 = f268(f269(), f211(v2, List::<u64>::nil()));
        utf8_decode(a1046)
    };
    match m1047 {
        None => {
            f249(Adt90::C14)
        }
        Some(v4) => {
            if v3 {
                f399(v0, v1, v4)
            } else {
                f314(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f309(v0: u64) -> u64 {
    let a1048 = v0;
    let a1049 = 34u64;
    if nat_eq(a1048, a1049) {
        34u64
    } else {
        let a1050 = v0;
        let a1051 = 92u64;
        if nat_eq(a1050, a1051) {
            92u64
        } else {
            let a1052 = v0;
            let a1053 = 47u64;
            if nat_eq(a1052, a1053) {
                47u64
            } else {
                let a1054 = v0;
                let a1055 = 98u64;
                if nat_eq(a1054, a1055) {
                    8u64
                } else {
                    let a1056 = v0;
                    let a1057 = 102u64;
                    if nat_eq(a1056, a1057) {
                        12u64
                    } else {
                        let a1058 = v0;
                        let a1059 = 110u64;
                        if nat_eq(a1058, a1059) {
                            10u64
                        } else {
                            let a1060 = v0;
                            let a1061 = 114u64;
                            if nat_eq(a1060, a1061) {
                                13u64
                            } else {
                                let a1062 = v0;
                                let a1063 = 116u64;
                                if nat_eq(a1062, a1063) {
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

pub fn f310(v0: u64) -> u64 {
    let a1064 = v0;
    let a1065 = 48u64;
    if nat_lt(a1064, a1065) {
        16u64
    } else {
        let a1066 = v0;
        let a1067 = 58u64;
        if nat_lt(a1066, a1067) {
            let a1068 = v0;
            let a1069 = 48u64;
            nat_sub(a1068, a1069)
        } else {
            let a1070 = v0;
            let a1071 = 65u64;
            if nat_lt(a1070, a1071) {
                16u64
            } else {
                let a1072 = v0;
                let a1073 = 71u64;
                if nat_lt(a1072, a1073) {
                    let a1074 = v0;
                    let a1075 = 55u64;
                    nat_sub(a1074, a1075)
                } else {
                    let a1076 = v0;
                    let a1077 = 97u64;
                    if nat_lt(a1076, a1077) {
                        16u64
                    } else {
                        let a1078 = v0;
                        let a1079 = 103u64;
                        if nat_lt(a1078, a1079) {
                            let a1080 = v0;
                            let a1081 = 87u64;
                            nat_sub(a1080, a1081)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f311(v0: Adt103, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt124> {
    let a1082 = v3;
    let a1083 = 0u64;
    if nat_eq(a1082, a1083) {
        if f240(v2, 55296u64, 56319u64) {
            Ok(f248(v0, Adt104::C6(Adt105::C3(v2), v4, v5)))
        } else {
            if f240(v2, 56320u64, 57343u64) {
                Ok(f249(Adt90::C6(v1)))
            } else {
                Ok(f248(v0, Adt104::C6(Adt105::C0, f400(v2, v4)?, v5)))
            }
        }
    } else {
        if f240(v2, 56320u64, 57343u64) {
            Ok(f248(v0, Adt104::C6(Adt105::C0, f400({
                let a1092 = 65536u64;
                let a1093 = {
                    let a1090 = {
                        let a1086 = {
                            let a1084 = v3;
                            let a1085 = 55296u64;
                            nat_sub(a1084, a1085)
                        };
                        let a1087 = 1024u64;
                        nat_mul(a1086, a1087)?
                    };
                    let a1091 = {
                        let a1088 = v2;
                        let a1089 = 56320u64;
                        nat_sub(a1088, a1089)
                    };
                    nat_add(a1090, a1091)?
                };
                nat_add(a1092, a1093)?
            }, v4)?, v5)))
        } else {
            Ok(f249(Adt90::C6(v1)))
        }
    }
}

pub fn f312(v0: u64, v1: u64) -> u64 {
    let m1096 = {
        let a1094 = f313(v0);
        let a1095 = v1;
        index_list(a1094, a1095)
    };
    match m1096 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f313(v0: u64) -> List<u64> {
    let a1097 = v0;
    let a1098 = 0u64;
    if nat_eq(a1097, a1098) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1099 = v0;
        let a1100 = 1u64;
        if nat_eq(a1099, a1100) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f314(v0: Adt103, v1: Adt6) -> Adt124 {
    Adt124::C1(f242(v1, match v0.clone() {
        Adt103::C0(_, h1101, _) => {
            h1101
        }
    }, match v0 {
        Adt103::C0(_, _, h1102) => {
            h1102
        }
    }))
}

pub fn f315(v0: u64) -> Adt6 {
    let a1103 = v0;
    let a1104 = 0u64;
    if nat_eq(a1103, a1104) {
        Adt6::C1(true)
    } else {
        let a1105 = v0;
        let a1106 = 1u64;
        if nat_eq(a1105, a1106) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f316(v0: Adt103, v1: Adt106, v2: Adt107) -> Adt124 {
    f248(v0, Adt104::C8(Adt106::C0(match v1.clone() {
        Adt106::C0(h1107, _, _, _, _, _, _) => {
            h1107
        }
    }, v2, match v1.clone() {
        Adt106::C0(_, _, h1108, _, _, _, _) => {
            h1108
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, h1109, _, _, _) => {
            h1109
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, _, h1110, _, _) => {
            h1110
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, _, _, h1111, _) => {
            h1111
        }
    }, match v1 {
        Adt106::C0(_, _, _, _, _, _, h1112) => {
            h1112
        }
    })))
}

pub fn f317(v0: Adt103, v1: u64, v2: u64, v3: Adt106, v4: bool) -> R<Adt124> {
    let v5: u64 = {
        let a1114 = match v3.clone() {
            Adt106::C0(_, _, _, h1113, _, _, _) => {
                h1113
            }
        };
        let a1115 = 0u64;
        if if nat_eq(a1114, a1115) {
            let a1116 = v2;
            let a1117 = 48u64;
            nat_eq(a1116, a1117)
        } else {
            false
        } {
            0u64
        } else {
            let a1119 = match v3.clone() {
                Adt106::C0(_, _, _, h1118, _, _, _) => {
                    h1118
                }
            };
            let a1120 = 1u64;
            nat_add(a1119, a1120)?
        }
    };
    let v6: i64 = if v4 {
        f266(match v3.clone() {
            Adt106::C0(_, _, _, _, h1121, _, _) => {
                h1121
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt106::C0(_, _, _, _, h1122, _, _) => {
                h1122
            }
        }
    };
    let a1123 = f401();
    let a1124 = v5;
    if nat_lt(a1123, a1124) {
        Ok(f249(Adt90::C8(v1)))
    } else {
        let m1127 = {
            let a1125 = 99999i64;
            let a1126 = v6;
            compare(a1125, a1126)
        };
        if match m1127 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f249(Adt90::C8(v1)))
        } else {
            Ok(f248(v0, f302(match v3.clone() {
                Adt106::C0(h1128, _, _, _, _, _, _) => {
                    h1128
                }
            }, if v4 {
                Adt107::C4
            } else {
                Adt107::C2
            }, f266({
                let a1130 = match v3.clone() {
                    Adt106::C0(_, _, h1129, _, _, _, _) => {
                        h1129
                    }
                };
                let a1131 = 10i64;
                int_mul(a1130, a1131)?
            }, f303({
                let a1132 = v2;
                let a1133 = 48u64;
                nat_sub(a1132, a1133)
            }))?, v5, v6, match v3.clone() {
                Adt106::C0(_, _, _, _, _, h1134, _) => {
                    h1134
                }
            }, match v3 {
                Adt106::C0(_, _, _, _, _, _, h1135) => {
                    h1135
                }
            })))
        }
    }
}

pub fn f318(v0: u64) -> bool {
    let a1136 = v0;
    let a1137 = 101u64;
    if nat_eq(a1136, a1137) {
        true
    } else {
        let a1138 = v0;
        let a1139 = 69u64;
        nat_eq(a1138, a1139)
    }
}

pub fn f319(v0: u64) -> bool {
    f240(v0, 48u64, 57u64)
}

pub fn f320(v0: Adt103, v1: u64, v2: u64, v3: Adt106) -> R<Adt124> {
    let m1140 = f241(v3)?;
    match m1140 {
        Adt131::C0 => {
            Ok(f249(Adt90::C7(v1)))
        }
        Adt131::C1 => {
            Ok(f249(Adt90::C8(v1)))
        }
        Adt131::C2(v4) => {
            Ok(f246(f242(v4, match v0.clone() {
                Adt103::C0(_, h1141, _) => {
                    h1141
                }
            }, match v0 {
                Adt103::C0(_, _, h1142) => {
                    h1142
                }
            }), v1, v2))
        }
    }
}

pub fn f321(v0: Adt103, v1: u64, v2: u64, v3: Adt106, v4: bool) -> R<Adt124> {
    let v5: i64 = f266({
        let a1144 = match v3.clone() {
            Adt106::C0(_, _, _, _, _, _, h1143) => {
                h1143
            }
        };
        let a1145 = 10i64;
        int_mul(a1144, a1145)?
    }, f303({
        let a1146 = v2;
        let a1147 = 48u64;
        nat_sub(a1146, a1147)
    }))?;
    let m1150 = {
        let a1148 = 99999i64;
        let a1149 = v5;
        compare(a1148, a1149)
    };
    if match m1150 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f249(Adt90::C8(v1)))
    } else {
        Ok(f248(v0, f302(match v3.clone() {
            Adt106::C0(h1151, _, _, _, _, _, _) => {
                h1151
            }
        }, Adt107::C7, match v3.clone() {
            Adt106::C0(_, _, h1152, _, _, _, _) => {
                h1152
            }
        }, match v3.clone() {
            Adt106::C0(_, _, _, h1153, _, _, _) => {
                h1153
            }
        }, match v3 {
            Adt106::C0(_, _, _, _, h1154, _, _) => {
                h1154
            }
        }, v4, v5)))
    }
}

pub fn f322(p1157: List<u64>, v1: bool) -> bool {
    let m1156 = p1157;
    {
        let mut m1156 = m1156;
        loop {
            let step = {
                let v0 = m1156;
                let m1155 = v0;
                match m1155.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f402(v2)
                        } else {
                            f403(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f323() -> u64 {
    64u64
}

pub fn f324(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1158 = f236(Str::lit("type"), v1);
    match m1158 {
        Adt123::C0 => {
            Some(Adt94::C3(v0))
        }
        Adt123::C1(v2) => {
            let m1159 = f233(v2);
            match m1159 {
                None => {
                    Some(Adt94::C3(v0))
                }
                Some(v3) => {
                    let a1160 = v3;
                    let a1161 = Str::lit("object");
                    if equal(a1160, a1161) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f325(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1162 = f236(Str::lit("$schema"), v1);
    match m1162 {
        Adt123::C0 => {
            None::<Adt94>
        }
        Adt123::C1(v2) => {
            let m1163 = f233(v2);
            match m1163 {
                None => {
                    Some(Adt94::C2(v0))
                }
                Some(v3) => {
                    let a1164 = v3;
                    let a1165 = f404();
                    if equal(a1164, a1165) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f326(v0: Adt35) -> R<Option<Adt94>> {
    let m1167 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h1166, _, _) => {
            h1166
        }
    };
    match m1167 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v1) => {
            let m1170 = f325(match v0.clone() {
                Adt35::C0(h1168, _, _, _, _, _, _, _) => {
                    h1168
                }
            }, match v1.clone() {
                Adt5::C0(h1169) => {
                    h1169
                }
            });
            match m1170 {
                None => {
                    let m1174 = f256(f257(), match v0 {
                        Adt35::C0(h1171, _, _, _, _, _, _, _) => {
                            h1171
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1172) => {
                            h1172
                        }
                    }), List::cons(Adt125::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1173) => {
                            h1173
                        }
                    }), false, List::<Str>::nil()), List::<Adt125>::nil()), f258())?;
                    match m1174 {
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

pub fn f327(v0: Str, v1: Adt6, v2: Adt125, v3: Adt132, v4: List<Adt125>) -> R<Result<(List<Adt125>, Adt132), Adt94>> {
    let m1175 = v2;
    match m1175 {
        Adt125::C0(v5, v6, v7) => {
            let m1176 = f328(v5.clone());
            match m1176 {
                None => {
                    let m1177 = f329(v5);
                    match m1177 {
                        None => {
                            Ok(Err::<(List<Adt125>, Adt132), Adt94>(Adt94::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f405(v0, v1, match v9 {
                                Adt5::C0(h1178) => {
                                    h1178
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

pub fn f328(v0: Adt6) -> Option<bool> {
    let m1179 = v0;
    match m1179 {
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

pub fn f329(v0: Adt6) -> Option<Adt5> {
    let m1180 = v0;
    match m1180 {
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

pub fn f330(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    let m1181 = f406(v1.clone(), v2.clone())?;
    match m1181 {
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
            f407(v0, v1, v2, v3)
        }
    }
}

pub fn f331(v0: List<Adt6>, v1: Adt6, v2: List<Adt110>, v3: u64, v4: List<Adt112>) -> Adt126 {
    let m1182 = v0;
    match m1182.uncons() {
        None => {
            let a1183 = v3;
            let a1184 = 1u64;
            if nat_eq(a1183, a1184) {
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

pub fn f332(v0: Option<Adt6>, v1: Adt6, v2: List<Adt110>, v3: List<Adt112>) -> Adt126 {
    let m1185 = v0;
    match m1185 {
        None => {
            Adt126::C0(Adt111::C0, v3)
        }
        Some(v4) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f333(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1186 = f236(Str::lit("anyOf"), v0);
    match m1186 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1187 = f408(v3);
            match m1187 {
                None => {
                    Adt133::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1188 = v4;
                    match m1188.uncons() {
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

pub fn f334(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1189 = f236(Str::lit("oneOf"), v0);
    match m1189 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1190 = f408(v3);
            match m1190 {
                None => {
                    Adt133::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1191 = v4;
                    match m1191.uncons() {
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

pub fn f335(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1192 = f236(Str::lit("not"), v0);
    match m1192 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            Adt133::C3(List::cons(Adt112::C0(v3, v1, v2.clone()), List::cons(Adt112::C3(v2), List::<Adt112>::nil())))
        }
    }
}

pub fn f336(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1193 = f236(Str::lit("if"), v0.clone());
    match m1193 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            Adt133::C3(List::cons(Adt112::C0(v3, v1.clone(), v2.clone()), List::cons(Adt112::C4(f409(Str::lit("then"), v0.clone()), f409(Str::lit("else"), v0), v1, v2), List::<Adt112>::nil())))
        }
    }
}

pub fn f337(v0: List<Adt6>, v1: Adt6, v2: List<Adt110>, v3: List<Adt112>) -> Adt126 {
    let m1194 = v0;
    match m1194.uncons() {
        None => {
            Adt126::C0(Adt111::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v4, v1.clone(), v2.clone()), List::cons(Adt112::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f338(v0: Str) -> R<bool> {
    let m1196 = f292(f69({
        let a1195 = v0;
        utf8_encode(a1195)
    })?)?;
    match m1196 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f339(v0: Adt37) -> bool {
    let m1198 = match v0 {
        Adt37::C0(_, _, h1197) => {
            h1197
        }
    };
    match m1198 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f340(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1200 = match v0.clone() {
        Adt49::C0(_, h1199) => {
            h1199
        }
    };
    match m1200.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h1201, _) => {
                    h1201
                }
            })))
        }
        Some((_, _)) => {
            if f410(match v0.clone() {
                Adt49::C0(_, h1202) => {
                    h1202
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f341(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1206 = {
        let m1205 = match v1.clone() {
            Adt49::C0(h1204, _) => {
                h1204
            }
        };
        match m1205 {
            Adt37::C0(_, h1203, _) => {
                h1203
            }
        }
    };
    match m1206 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1209 = f131(Adt6::C5(match v0.clone() {
                Adt5::C0(h1207) => {
                    h1207
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1208) => {
                    h1208
                }
            }), v2)?;
            match m1209 {
                Adt109::C0 => {
                    f340(v1)
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

pub fn f342(v0: Adt110) -> R<Str> {
    let m1210 = v0;
    match m1210 {
        Adt110::C0(v1) => {
            f411(v1)
        }
        Adt110::C1(v2) => {
            let a1211 = f194(v2, 0i64)?;
            Ok(format_i64(a1211))
        }
    }
}

pub fn f343(p1217: List<u64>, p1218: List<u64>) -> Option<List<u64>> {
    let m1216 = (p1217, p1218);
    {
        let mut m1216 = m1216;
        loop {
            let step = {
                let (v0, v1) = m1216;
                let m1212 = v0;
                match m1212.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1213 = v1;
                        match m1213.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1214 = v2;
                                let a1215 = v4;
                                if nat_eq(a1214, a1215) {
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
                Ok(next) => { m1216 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: Str) -> R<List<u64>> {
    f69({
        let a1219 = v0;
        utf8_encode(a1219)
    })
}

pub fn f345() -> Str {
    Str::lit("test://template/")
}

pub fn f346(v0: List<u64>) -> List<u64> {
    f211(v0, List::<u64>::nil())
}

pub fn f347() -> Str {
    Str::lit("data")
}

pub fn f348(v0: List<u64>) -> bool {
    let m1220 = v0;
    match m1220.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f349(p1225: List<u64>) -> bool {
    let m1224 = p1225;
    {
        let mut m1224 = m1224;
        loop {
            let step = {
                let v0 = m1224;
                let m1221 = v0;
                match m1221.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1222 = v1;
                        let a1223 = 47u64;
                        if nat_eq(a1222, a1223) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(p1230: Bytes, p1231: List<Bytes>) -> Bytes {
    let m1229 = (p1230, p1231);
    {
        let mut m1229 = m1229;
        loop {
            let step = {
                let (v0, v1) = m1229;
                let m1226 = v1;
                match m1226.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1227 = v2;
                            let a1228 = v0;
                            append_bytes(a1227, a1228)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(p1234: List<Bytes>, p1235: List<u64>) -> List<Bytes> {
    let m1233 = (p1234, p1235);
    {
        let mut m1233 = m1233;
        loop {
            let step = {
                let (v0, v1) = m1233;
                let m1232 = v1;
                match m1232.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f412(f413(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(p1241: List<u64>, p1242: List<u64>) -> bool {
    let m1240 = (p1241, p1242);
    {
        let mut m1240 = m1240;
        loop {
            let step = {
                let (v0, v1) = m1240;
                let m1236 = v0;
                match m1236.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1237 = v1;
                        match m1237.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1238 = v2;
                                let a1239 = v4;
                                if nat_eq(a1238, a1239) {
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
                Ok(next) => { m1240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: u64, v1: Adt6, v2: List<Adt114>, v3: List<u64>) -> R<Adt134> {
    let m1243 = v1;
    match m1243 {
        Adt6::C0 => {
            Ok(Adt134::C1(v2, f211(f414(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt134::C1(v2, f211(if v4 {
                f414(0u64)
            } else {
                f414(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f415(v5.clone())? {
                Ok(Adt134::C1(v2, f211(f416(v5)?, v3)))
            } else {
                Ok(Adt134::C0(Adt90::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt134::C1(v2, f211(f354(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1244 = v0;
            let a1245 = f398();
            if nat_lt(a1244, a1245) {
                Ok(Adt134::C1(f417(91u64, Adt114::C2({
                    let a1246 = v0;
                    let a1247 = 1u64;
                    nat_add(a1246, a1247)?
                }, v7), v2), v3))
            } else {
                Ok(Adt134::C0(Adt90::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1248 = v0;
            let a1249 = f398();
            if nat_lt(a1248, a1249) {
                Ok(Adt134::C1(f417(123u64, Adt114::C4({
                    let a1250 = v0;
                    let a1251 = 1u64;
                    nat_add(a1250, a1251)?
                }, v8), v2), v3))
            } else {
                Ok(Adt134::C0(Adt90::C12))
            }
        }
    }
}

pub fn f354(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1253 = f211(f418(f69({
            let a1252 = v0;
            utf8_encode(a1252)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1254 = List::cons(34u64, List::<u64>::nil());
        append_list(a1253, a1254)
    }))
}

pub fn f355(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f419(match v0.clone() {
        Adt33::C0(_, h1255, _, _) => {
            h1255
        }
    })), List::cons((Str::lit("capabilities"), f420(match v0.clone() {
        Adt33::C0(_, _, h1256, _) => {
            h1256
        }
    })), {
        let a1259 = f421(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1257) => {
                h1257
            }
        });
        let a1260 = f422(match v0 {
            Adt33::C0(h1258, _, _, _) => {
                h1258
            }
        })?;
        append_list(a1259, a1260)
    })))
}

pub fn f356(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f423(match v0.clone() {
        Adt34::C0(_, _, h1261) => {
            h1261
        }
    }))), {
        let a1264 = f421(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1262, _) => {
                h1262
            }
        });
        let a1265 = f422(match v0 {
            Adt34::C0(h1263, _, _) => {
                h1263
            }
        })?;
        append_list(a1264, a1265)
    }))
}

pub fn f357(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1266 = v0;
    match m1266 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1267 = f368(Adt6::C5(v1))?;
            match m1267 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1268 = f329(v2);
                    match m1268 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1269) => {
                                    h1269
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f358(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1271 = f424(match v0.clone() {
        Adt37::C0(h1270, _, _) => {
            h1270
        }
    })?;
    match m1271 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1274 = f425(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1272, _) => {
                        h1272
                    }
                });
                let a1275 = f426(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1273) => {
                        h1273
                    }
                });
                append_list(a1274, a1275)
            })))
        }
    }
}

pub fn f359(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f427(match v0.clone() {
        Adt51::C0(_, _, h1276) => {
            h1276
        }
    })?)), {
        let a1279 = f421(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1277, _) => {
                h1277
            }
        });
        let a1280 = f422(match v0 {
            Adt51::C0(h1278, _, _) => {
                h1278
            }
        })?;
        append_list(a1279, a1280)
    }))
}

pub fn f360(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f428(match v0.clone() {
        Adt52::C0(_, h1281) => {
            h1281
        }
    }))), f422(match v0 {
        Adt52::C0(h1282, _) => {
            h1282
        }
    })?))
}

pub fn f361(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f429(match v0.clone() {
        Adt53::C0(_, _, h1283) => {
            h1283
        }
    }))), {
        let a1286 = f421(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1284, _) => {
                h1284
            }
        });
        let a1287 = f422(match v0 {
            Adt53::C0(h1285, _, _) => {
                h1285
            }
        })?;
        append_list(a1286, a1287)
    }))
}

pub fn f362(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f430(match v0.clone() {
        Adt55::C0(_, _, h1288) => {
            h1288
        }
    }))), {
        let a1291 = f421(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1289, _) => {
                h1289
            }
        });
        let a1292 = f422(match v0 {
            Adt55::C0(h1290, _, _) => {
                h1290
            }
        })?;
        append_list(a1291, a1292)
    }))
}

pub fn f363(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1294 = f431(match v0.clone() {
        Adt58::C0(_, h1293) => {
            h1293
        }
    })?;
    match m1294 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1296 = f421(Str::lit("description"), match v0 {
                    Adt58::C0(h1295, _) => {
                        h1295
                    }
                });
                let a1297 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1296, a1297)
            }))
        }
    }
}

pub fn f364(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f378({
        let m1300 = match v0.clone() {
            Adt60::C0(h1299) => {
                h1299
            }
        };
        match m1300 {
            Adt61::C0(h1298, _, _) => {
                h1298
            }
        }
    }))), {
        let a1307 = f432(Str::lit("total"), {
            let m1303 = match v0.clone() {
                Adt60::C0(h1302) => {
                    h1302
                }
            };
            match m1303 {
                Adt61::C0(_, h1301, _) => {
                    h1301
                }
            }
        })?;
        let a1308 = f426(Str::lit("hasMore"), {
            let m1306 = match v0 {
                Adt60::C0(h1305) => {
                    h1305
                }
            };
            match m1306 {
                Adt61::C0(_, _, h1304) => {
                    h1304
                }
            }
        });
        append_list(a1307, a1308)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f365(v0: Adt140) -> Str {
    let m1309 = v0;
    match m1309 {
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

pub fn f366(v0: Adt32) -> Adt140 {
    let m1310 = v0;
    match m1310 {
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

pub fn f367(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1311 = v0;
    match m1311 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f433(), f434(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f368(v0: Adt6) -> R<Option<Adt6>> {
    f435(f436(), Adt141::C0(v0), List::<Adt142>::nil())
}

pub fn f369(v0: Adt118) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f165())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f437(match v0.clone() {
        Adt118::C0(h1312, _) => {
            h1312
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1315 = match v0.clone() {
            Adt118::C0(_, h1314) => {
                h1314
            }
        };
        match m1315 {
            Adt50::C0(h1313, _, _) => {
                h1313
            }
        }
    })), {
        let a1322 = f438({
            let m1318 = match v0.clone() {
                Adt118::C0(_, h1317) => {
                    h1317
                }
            };
            match m1318 {
                Adt50::C0(_, h1316, _) => {
                    h1316
                }
            }
        });
        let a1323 = f439({
            let m1321 = match v0 {
                Adt118::C0(_, h1320) => {
                    h1320
                }
            };
            match m1321 {
                Adt50::C0(_, _, h1319) => {
                    h1319
                }
            }
        });
        append_list(a1322, a1323)
    })))), List::<(Str, Adt6)>::nil()))))
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

pub fn f375(v0: Adt84) -> i64 {
    let m1324 = v0;
    match m1324 {
        Adt84::C0(_) => {
            f371()
        }
        Adt84::C1(_) => {
            f371()
        }
        Adt84::C2(_) => {
            f371()
        }
        Adt84::C3(_) => {
            f371()
        }
        Adt84::C4(_) => {
            f371()
        }
        Adt84::C5(_) => {
            f371()
        }
        Adt84::C6(_) => {
            f440()
        }
        Adt84::C7(_) => {
            f441()
        }
        Adt84::C8(_) => {
            f442()
        }
        Adt84::C9(_) => {
            f440()
        }
    }
}

pub fn f376(v0: Adt120) -> Str {
    let m1325 = v0;
    match m1325 {
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

pub fn f377(v0: Adt97) -> Str {
    let m1326 = v0;
    match m1326 {
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
            let m1327 = v1;
            match m1327 {
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
            let m1328 = v2;
            match m1328 {
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

pub fn f378(v0: List<Str>) -> List<Adt6> {
    f304(f443(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f379(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1340 = f444(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1329, _, _, _, _) => {
                h1329
            }
        });
        let a1341 = {
            let a1338 = f444(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1330, _, _, _) => {
                    h1330
                }
            });
            let a1339 = {
                let a1336 = f444(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1331, _, _) => {
                        h1331
                    }
                });
                let a1337 = {
                    let a1334 = f444(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1332, _) => {
                            h1332
                        }
                    });
                    let a1335 = f444(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1333) => {
                            h1333
                        }
                    });
                    append_list(a1334, a1335)
                };
                append_list(a1336, a1337)
            };
            append_list(a1338, a1339)
        };
        append_list(a1340, a1341)
    })
}

pub fn f380(v0: Adt0, v1: Fn0, v2: u64, v3: Adt127, v4: bool) -> R<Result<Adt116, Adt119>> {
    if v4 {
        f445(v0, v1, v2, f446(match v3.clone() {
            Adt127::C0(_, h1342, _, _) => {
                h1342
            }
        }, match v3.clone() {
            Adt127::C0(_, _, h1343, _) => {
                h1343
            }
        }, match v3 {
            Adt127::C0(_, _, _, h1344) => {
                h1344
            }
        }))
    } else {
        Ok(Err::<Adt116, Adt119>(Adt119::C3(f447(match v3 {
            Adt127::C0(_, h1345, _, _) => {
                h1345
            }
        }))))
    }
}

pub fn f381(v0: Adt4, v1: Adt128) -> bool {
    let m1346 = v1;
    match m1346 {
        Adt128::C0 => {
            true
        }
        Adt128::C1 => {
            f448(match v0 {
                Adt4::C0(_, _, _, _, _, h1347, _) => {
                    h1347
                }
            })
        }
        Adt128::C2 => {
            f448(match v0 {
                Adt4::C0(_, _, _, _, _, h1348, _) => {
                    h1348
                }
            })
        }
        Adt128::C3 => {
            f449(match v0 {
                Adt4::C0(_, _, _, _, h1349, _, _) => {
                    h1349
                }
            })
        }
        Adt128::C4 => {
            f449(match v0 {
                Adt4::C0(_, _, _, _, h1350, _, _) => {
                    h1350
                }
            })
        }
        Adt128::C5 => {
            f449(match v0 {
                Adt4::C0(_, _, _, _, h1351, _, _) => {
                    h1351
                }
            })
        }
        Adt128::C6 => {
            f448(match v0 {
                Adt4::C0(_, _, _, h1352, _, _, _) => {
                    h1352
                }
            })
        }
        Adt128::C7 => {
            f448(match v0 {
                Adt4::C0(_, _, _, h1353, _, _, _) => {
                    h1353
                }
            })
        }
        Adt128::C8 => {
            f450(match v0 {
                Adt4::C0(_, _, h1354, _, _, _, _) => {
                    h1354
                }
            })
        }
        Adt128::C9 => {
            f451(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1355) => {
                    h1355
                }
            }, f452())
        }
        Adt128::C10 => {
            f451(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1356) => {
                    h1356
                }
            }, f452())
        }
        Adt128::C11 => {
            f451(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1357) => {
                    h1357
                }
            }, f452())
        }
    }
}

pub fn f382(v0: Adt137, v1: Adt135) -> Result<Adt127, Adt119> {
    let m1358 = v1;
    match m1358 {
        Adt135::C0 => {
            Err::<Adt127, Adt119>(Adt119::C3(match v0 {
                Adt137::C0(_, h1359, _, _) => {
                    h1359
                }
            }))
        }
        Adt135::C1(v2) => {
            Ok::<Adt127, Adt119>(Adt127::C0(match v0.clone() {
                Adt137::C0(h1360, _, _, _) => {
                    h1360
                }
            }, v2, match v0.clone() {
                Adt137::C0(_, _, h1361, _) => {
                    h1361
                }
            }, match v0 {
                Adt137::C0(_, _, _, h1362) => {
                    h1362
                }
            }))
        }
    }
}

pub fn f383(v0: Option<Adt96>, v1: List<Str>, v2: Adt95, v3: Result<Adt16, Adt120>) -> R<Adt136> {
    let m1363 = v3;
    match m1363 {
        Ok(v5) => {
            Ok(f453(v1, v2.clone(), v5.clone(), f454(v0, f290(match v2.clone() {
                Adt95::C0(_, h1364, _) => {
                    h1364
                }
            }), match v2.clone() {
                Adt95::C0(_, h1365, _) => {
                    h1365
                }
            }, match v5 {
                Adt16::C0(h1366, _, _, _, _, _) => {
                    h1366
                }
            }, f455(match v2 {
                Adt95::C0(_, _, h1367) => {
                    h1367
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt136::C1(Adt119::C0(v4)))
        }
    }
}

pub fn f384(v0: Option<Adt5>) -> Result<Adt16, Adt120> {
    let m1368 = v0;
    match m1368 {
        None => {
            Err::<Adt16, Adt120>(Adt120::C0)
        }
        Some(v1) => {
            let m1369 = f177(Str::lit("_meta"), v1);
            match m1369 {
                Adt123::C0 => {
                    Err::<Adt16, Adt120>(Adt120::C1)
                }
                Adt123::C1(v2) => {
                    let m1370 = f329(v2);
                    match m1370 {
                        None => {
                            Err::<Adt16, Adt120>(Adt120::C4)
                        }
                        Some(v3) => {
                            f456(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f385(p1377: List<u64>, p1378: List<u64>) -> R<bool> {
    let m1376 = (p1377, p1378);
    {
        let mut m1376 = m1376;
        loop {
            let step = {
                let (v0, v1) = m1376;
                let m1371 = v0;
                match m1371.uncons() {
                    None => {
                        let m1372 = v1;
                        match m1372.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1373 = v1;
                        match m1373.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a1374 = f457(v4)?;
                                let a1375 = f457(v6)?;
                                if nat_eq(a1374, a1375) {
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
                Ok(next) => { m1376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: Adt130, v1: List<u64>) -> Adt130 {
    let m1379 = v0;
    match m1379 {
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

pub fn f387(v0: List<u64>) -> List<u64> {
    f393(f458(f393(f458(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f388(p1382: List<u64>) -> bool {
    let m1381 = p1382;
    {
        let mut m1381 = m1381;
        loop {
            let step = {
                let v0 = m1381;
                let m1380 = v0;
                match m1380.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f459(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389(v0: Adt128) -> Adt138 {
    let m1383 = v0;
    match m1383 {
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

pub fn f390(p1389: List<u64>, p1390: List<u64>) -> Option<List<u64>> {
    let m1388 = (p1389, p1390);
    {
        let mut m1388 = m1388;
        loop {
            let step = {
                let (v0, v1) = m1388;
                let m1384 = v0;
                match m1384.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1385 = v1;
                        match m1385.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1386 = v4;
                                let a1387 = v2;
                                if nat_eq(a1386, a1387) {
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
                Ok(next) => { m1388 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f392() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f393(p1393: List<u64>, p1394: List<u64>) -> List<u64> {
    let m1392 = (p1393, p1394);
    {
        let mut m1392 = m1392;
        loop {
            let step = {
                let (v0, v1) = m1392;
                let m1391 = v0;
                match m1391.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(p1403: u64, p1404: List<u64>, p1405: List<u64>) -> R<Option<List<u64>>> {
    let m1402 = (p1403, (p1404, p1405));
    {
        let mut m1402 = m1402;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1402;
                let m1395 = v0;
                if m1395 == 0 {
                    let m1396 = v1;
                    match m1396.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1395 - 1;
                    let m1397 = v1;
                    match m1397.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1398 = v7;
                            match m1398.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1399 = v9;
                                    match m1399.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1400 = v11;
                                            match m1400.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1401 = f460(v6, v8, v10, v12, f461(v13.clone()))?;
                                                    match m1401 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f211(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: Adt12, v1: Option<Str>) -> bool {
    let m1406 = v1;
    match m1406 {
        None => {
            false
        }
        Some(v2) => {
            let a1408 = match v0 {
                Adt12::C0(h1407, _, _, _) => {
                    h1407
                }
            };
            let a1409 = v2;
            equal(a1408, a1409)
        }
    }
}

pub fn f396(v0: List<Adt86>, v1: Str) -> R<Adt100> {
    let m1412 = f228(f224(v0, {
        let a1410 = f462();
        let a1411 = f463(v1)?;
        append_list(a1410, a1411)
    })?)?;
    match m1412 {
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
            let m1413 = f287(v2);
            match m1413 {
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

pub fn f397(v0: i64) -> bool {
    let a1420 = {
        let m1416 = {
            let a1414 = v0;
            let a1415 = 0i64;
            compare(a1414, a1415)
        };
        if match m1416 {
            Ordering::Less => {
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
            let m1419 = {
                let a1417 = 0i64;
                let a1418 = v0;
                compare(a1417, a1418)
            };
            match m1419 {
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
    bool_not(a1420)
}

pub fn f398() -> u64 {
    512u64
}

pub fn f399(v0: Adt103, v1: u64, v2: Str) -> Adt124 {
    let m1422 = match v0.clone() {
        Adt103::C0(_, h1421, _) => {
            h1421
        }
    };
    match m1422.uncons() {
        None => {
            f249(Adt90::C2(v1))
        }
        Some((v3, v4)) => {
            let m1423 = v3;
            match m1423 {
                Adt108::C0(_) => {
                    f249(Adt90::C2(v1))
                }
                Adt108::C1(v6, _) => {
                    Adt124::C1(Adt103::C0(Adt104::C5, List::cons(Adt108::C1(v6, Some(v2)), v4), match v0 {
                        Adt103::C0(_, _, h1424) => {
                            h1424
                        }
                    }))
                }
            }
        }
    }
}

pub fn f400(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1425 = v0;
    let a1426 = 128u64;
    if nat_lt(a1425, a1426) {
        Ok(List::cons(v0, v1))
    } else {
        let a1427 = v0;
        let a1428 = 2048u64;
        if nat_lt(a1427, a1428) {
            Ok(List::cons({
                let a1429 = 128u64;
                let a1430 = f464(v0, 64u64);
                nat_add(a1429, a1430)?
            }, List::cons({
                let a1431 = 192u64;
                let a1432 = f465(v0, 64u64);
                nat_add(a1431, a1432)?
            }, v1)))
        } else {
            let a1433 = v0;
            let a1434 = 65536u64;
            if nat_lt(a1433, a1434) {
                Ok(List::cons({
                    let a1435 = 128u64;
                    let a1436 = f464(v0, 64u64);
                    nat_add(a1435, a1436)?
                }, List::cons({
                    let a1437 = 128u64;
                    let a1438 = f464(f465(v0, 64u64), 64u64);
                    nat_add(a1437, a1438)?
                }, List::cons({
                    let a1439 = 224u64;
                    let a1440 = f465(v0, 4096u64);
                    nat_add(a1439, a1440)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1441 = 128u64;
                    let a1442 = f464(v0, 64u64);
                    nat_add(a1441, a1442)?
                }, List::cons({
                    let a1443 = 128u64;
                    let a1444 = f464(f465(v0, 64u64), 64u64);
                    nat_add(a1443, a1444)?
                }, List::cons({
                    let a1445 = 128u64;
                    let a1446 = f464(f465(v0, 4096u64), 64u64);
                    nat_add(a1445, a1446)?
                }, List::cons({
                    let a1447 = 240u64;
                    let a1448 = f465(v0, 262144u64);
                    nat_add(a1447, a1448)?
                }, v1)))))
            }
        }
    }
}

pub fn f401() -> u64 {
    18u64
}

pub fn f402(v0: u64) -> bool {
    let a1449 = 48u64;
    let a1450 = v0;
    if if nat_le(a1449, a1450) {
        let a1451 = v0;
        let a1452 = 57u64;
        nat_le(a1451, a1452)
    } else {
        false
    } {
        true
    } else {
        let a1453 = 65u64;
        let a1454 = v0;
        if if nat_le(a1453, a1454) {
            let a1455 = v0;
            let a1456 = 90u64;
            nat_le(a1455, a1456)
        } else {
            false
        } {
            true
        } else {
            let a1457 = 97u64;
            let a1458 = v0;
            if if nat_le(a1457, a1458) {
                let a1459 = v0;
                let a1460 = 122u64;
                nat_le(a1459, a1460)
            } else {
                false
            } {
                true
            } else {
                let a1461 = v0;
                let a1462 = 33u64;
                if nat_eq(a1461, a1462) {
                    true
                } else {
                    let a1463 = v0;
                    let a1464 = 35u64;
                    if nat_eq(a1463, a1464) {
                        true
                    } else {
                        let a1465 = v0;
                        let a1466 = 36u64;
                        if nat_eq(a1465, a1466) {
                            true
                        } else {
                            let a1467 = v0;
                            let a1468 = 37u64;
                            if nat_eq(a1467, a1468) {
                                true
                            } else {
                                let a1469 = v0;
                                let a1470 = 38u64;
                                if nat_eq(a1469, a1470) {
                                    true
                                } else {
                                    let a1471 = v0;
                                    let a1472 = 39u64;
                                    if nat_eq(a1471, a1472) {
                                        true
                                    } else {
                                        let a1473 = v0;
                                        let a1474 = 42u64;
                                        if nat_eq(a1473, a1474) {
                                            true
                                        } else {
                                            let a1475 = v0;
                                            let a1476 = 43u64;
                                            if nat_eq(a1475, a1476) {
                                                true
                                            } else {
                                                let a1477 = v0;
                                                let a1478 = 45u64;
                                                if nat_eq(a1477, a1478) {
                                                    true
                                                } else {
                                                    let a1479 = v0;
                                                    let a1480 = 46u64;
                                                    if nat_eq(a1479, a1480) {
                                                        true
                                                    } else {
                                                        let a1481 = v0;
                                                        let a1482 = 94u64;
                                                        if nat_eq(a1481, a1482) {
                                                            true
                                                        } else {
                                                            let a1483 = v0;
                                                            let a1484 = 95u64;
                                                            if nat_eq(a1483, a1484) {
                                                                true
                                                            } else {
                                                                let a1485 = v0;
                                                                let a1486 = 96u64;
                                                                if nat_eq(a1485, a1486) {
                                                                    true
                                                                } else {
                                                                    let a1487 = v0;
                                                                    let a1488 = 124u64;
                                                                    if nat_eq(a1487, a1488) {
                                                                        true
                                                                    } else {
                                                                        let a1489 = v0;
                                                                        let a1490 = 126u64;
                                                                        nat_eq(a1489, a1490)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f403(v0: u64) -> bool {
    let a1491 = 48u64;
    let a1492 = v0;
    if if nat_le(a1491, a1492) {
        let a1493 = v0;
        let a1494 = 57u64;
        nat_le(a1493, a1494)
    } else {
        false
    } {
        true
    } else {
        let a1495 = 65u64;
        let a1496 = v0;
        if if nat_le(a1495, a1496) {
            let a1497 = v0;
            let a1498 = 90u64;
            nat_le(a1497, a1498)
        } else {
            false
        } {
            true
        } else {
            let a1499 = 97u64;
            let a1500 = v0;
            if if nat_le(a1499, a1500) {
                let a1501 = v0;
                let a1502 = 122u64;
                nat_le(a1501, a1502)
            } else {
                false
            } {
                true
            } else {
                let a1503 = v0;
                let a1504 = 95u64;
                if nat_eq(a1503, a1504) {
                    true
                } else {
                    let a1505 = v0;
                    let a1506 = 45u64;
                    if nat_eq(a1505, a1506) {
                        true
                    } else {
                        let a1507 = v0;
                        let a1508 = 46u64;
                        nat_eq(a1507, a1508)
                    }
                }
            }
        }
    }
}

pub fn f404() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f405(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt132, v6: List<Adt125>) -> R<Result<(List<Adt125>, Adt132), Adt94>> {
    let m1509 = f466(v0.clone(), v1, v2.clone());
    match m1509 {
        None => {
            let m1510 = f467(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1510 {
                Adt144::C0(v8) => {
                    Ok(Err::<(List<Adt125>, Adt132), Adt94>(v8))
                }
                Adt144::C1(v9) => {
                    Ok(Ok::<(List<Adt125>, Adt132), Adt94>((f468(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt125>, Adt132), Adt94>(v7))
        }
    }
}

pub fn f406(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt133> {
    Ok(f469(List::cons((Str::lit("type"), f470(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f471(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f472(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f473(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f473(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f473(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f473(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1511 = f474(v0.clone(), v1.clone())?;
        let a1512 = List::cons((Str::lit("required"), f475(v0, v1)), List::<(Str, Adt143)>::nil());
        append_list(a1511, a1512)
    })))))))))
}

pub fn f407(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    Ok(f476(f477(v0, v1.clone(), v2.clone(), v3.clone()), f476(f478(v1.clone(), v2.clone(), v3.clone()), f476(f479(v1.clone(), v2.clone(), v3.clone()), f476(f480(v1.clone(), v2.clone(), v3.clone())?, f476(f481(v1.clone(), v2.clone(), v3.clone()), f476(f482(Adt113::C0, f333(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f476(f482(Adt113::C1, f334(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f476(f482(Adt113::C2, f335(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f482(Adt113::C3, f336(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f408(v0: Adt6) -> Option<List<Adt6>> {
    let m1513 = v0;
    match m1513 {
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

pub fn f409(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1514 = f236(v0, v1);
    match m1514 {
        Adt123::C0 => {
            None::<Adt6>
        }
        Adt123::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f410(v0: List<Adt50>) -> R<bool> {
    let m1515 = v0;
    match m1515.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f483(v2, match v1 {
                Adt50::C0(h1516, _, _) => {
                    h1516
                }
            })
        }
    }
}

pub fn f411(v0: Str) -> R<Str> {
    let m1519 = {
        let a1518 = f268(f269(), f211(f484(f69({
            let a1517 = v0.clone();
            utf8_encode(a1517)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1518)
    };
    match m1519 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f412(p1528: Bytes, p1529: List<Bytes>) -> List<Bytes> {
    let m1527 = (p1528, p1529);
    {
        let mut m1527 = m1527;
        loop {
            let step = {
                let (v0, v1) = m1527;
                let m1520 = v1;
                match m1520.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1523 = {
                            let a1521 = v2.clone();
                            length_bytes(a1521)
                        };
                        let a1524 = {
                            let a1522 = v0.clone();
                            length_bytes(a1522)
                        };
                        if nat_eq(a1523, a1524) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1525 = v2;
                                let a1526 = v0;
                                append_bytes(a1525, a1526)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
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

pub fn f413(v0: u64) -> Bytes {
    let m1533 = {
        let a1530 = f485();
        let a1531 = v0;
        let a1532 = 1u64;
        slice_bytes(a1530, a1531, a1532)
    };
    match m1533 {
        None => {
            f269()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f414(v0: u64) -> List<u64> {
    f313(v0)
}

pub fn f415(v0: Adt7) -> R<bool> {
    let a1551 = {
        let m1537 = {
            let a1535 = f486();
            let a1536 = match v0.clone() {
                Adt7::C0(h1534, _) => {
                    h1534
                }
            };
            compare(a1535, a1536)
        };
        if match m1537 {
            Ordering::Less => {
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
            let m1542 = {
                let a1540 = match v0.clone() {
                    Adt7::C0(h1538, _) => {
                        h1538
                    }
                };
                let a1541 = {
                    let a1539 = f486();
                    int_neg(a1539)?
                };
                compare(a1540, a1541)
            };
            if match m1542 {
                Ordering::Less => {
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
                let m1546 = {
                    let a1544 = 9999i64;
                    let a1545 = match v0.clone() {
                        Adt7::C0(_, h1543) => {
                            h1543
                        }
                    };
                    compare(a1544, a1545)
                };
                if match m1546 {
                    Ordering::Less => {
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
                    let m1550 = {
                        let a1548 = match v0 {
                            Adt7::C0(_, h1547) => {
                                h1547
                            }
                        };
                        let a1549 = -9999i64;
                        compare(a1548, a1549)
                    };
                    match m1550 {
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
    Ok(bool_not(a1551))
}

pub fn f416(v0: Adt7) -> R<List<u64>> {
    let m1555 = {
        let a1553 = match v0.clone() {
            Adt7::C0(_, h1552) => {
                h1552
            }
        };
        let a1554 = 0i64;
        compare(a1553, a1554)
    };
    if if match m1555 {
        Ordering::Less => {
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
        let m1559 = {
            let a1557 = 0i64;
            let a1558 = match v0.clone() {
                Adt7::C0(_, h1556) => {
                    h1556
                }
            };
            compare(a1557, a1558)
        };
        match m1559 {
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
        let a1562 = f487(match v0.clone() {
            Adt7::C0(h1560, _) => {
                h1560
            }
        })?;
        let a1563 = List::cons(101u64, f487(match v0 {
            Adt7::C0(_, h1561) => {
                h1561
            }
        })?);
        Ok(append_list(a1562, a1563))
    } else {
        f487(match v0 {
            Adt7::C0(h1564, _) => {
                h1564
            }
        })
    }
}

pub fn f417(v0: u64, v1: Adt114, v2: List<Adt114>) -> List<Adt114> {
    List::cons(Adt114::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f418(p1567: List<u64>, p1568: List<u64>) -> R<List<u64>> {
    let m1566 = (p1567, p1568);
    {
        let mut m1566 = m1566;
        loop {
            let step = {
                let (v0, v1) = m1566;
                let m1565 = v0;
                match m1565.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f211(f488(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1566 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f419(v0: List<Str>) -> Adt6 {
    Adt6::C4(f378(v0))
}

pub fn f420(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1586 = f444(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1569, _, _, _, _, _, _) => {
                h1569
            }
        });
        let a1587 = {
            let a1584 = f444(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1570, _, _, _, _, _) => {
                    h1570
                }
            });
            let a1585 = {
                let a1582 = f444(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1571, _, _, _, _) => {
                        h1571
                    }
                });
                let a1583 = {
                    let a1580 = f489(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1572, _, _, _) => {
                            h1572
                        }
                    });
                    let a1581 = {
                        let a1578 = f490(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1573, _, _) => {
                                h1573
                            }
                        });
                        let a1579 = {
                            let a1576 = f489(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1574, _) => {
                                    h1574
                                }
                            });
                            let a1577 = f444(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1575) => {
                                    h1575
                                }
                            });
                            append_list(a1576, a1577)
                        };
                        append_list(a1578, a1579)
                    };
                    append_list(a1580, a1581)
                };
                append_list(a1582, a1583)
            };
            append_list(a1584, a1585)
        };
        append_list(a1586, a1587)
    })
}

pub fn f421(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1588 = v1;
    match m1588 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f422(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f77(match v0.clone() {
        Adt10::C0(h1589, _) => {
            h1589
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f491(match v0 {
        Adt10::C0(_, h1590) => {
            h1590
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f423(v0: List<Adt35>) -> List<Adt6> {
    f304(f492(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f424(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1591 = f493(v0, List::<Adt6>::nil())?;
    match m1591 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f304(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f425(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1592 = v1;
    match m1592 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f426(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1593 = v1;
    match m1593 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f427(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f304(f494(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f428(v0: List<Adt46>) -> List<Adt6> {
    f304(f495(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f429(v0: List<Adt54>) -> List<Adt6> {
    f304(f496(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f430(v0: List<Adt56>) -> List<Adt6> {
    f304(f497(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f431(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m1594 = f498(v0, List::<Adt6>::nil())?;
    match m1594 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f304(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f432(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1595 = v1;
    match m1595 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f77(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f433() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f434(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1596, _, _, _, _, _) => {
            h1596
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1597, _, _, _) => {
            h1597
        }
    })), {
        let a1606 = f421(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1598, _, _, _, _) => {
                h1598
            }
        });
        let a1607 = {
            let a1604 = f421(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1599, _, _) => {
                    h1599
                }
            });
            let a1605 = {
                let a1602 = f421(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1600, _) => {
                        h1600
                    }
                });
                let a1603 = f499(match v0 {
                    Adt1::C0(_, _, _, _, _, h1601) => {
                        h1601
                    }
                });
                append_list(a1602, a1603)
            };
            append_list(a1604, a1605)
        };
        append_list(a1606, a1607)
    })))
}

pub fn f435(p1622: u64, p1623: Adt141, p1624: List<Adt142>) -> R<Option<Adt6>> {
    let m1621 = (p1622, (p1623, p1624));
    {
        let mut m1621 = m1621;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1621;
                let m1608 = v0;
                if m1608 == 0 {
                    Err::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1608 - 1;
                    let m1609 = v1;
                    match m1609 {
                        Adt141::C0(v4) => {
                            let m1610 = v4.clone();
                            match m1610 {
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
                                    let v8: Str = f500(v7)?;
                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1611 = v9;
                                    match m1611.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0(v10), List::cons(Adt142::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1612 = v12;
                                    match m1612.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f500({
                                                let (h1613, _) = v13;
                                                h1613
                                            })?;
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0({
                                                let (_, h1614) = v13;
                                                h1614
                                            }), List::cons(Adt142::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt141::C1(v16) => {
                            let m1615 = v2;
                            match m1615.uncons() {
                                None => {
                                    Err::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1616 = v17;
                                    match m1616 {
                                        Adt142::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1617 = v20;
                                            match m1617.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C4(f304(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0(v22), List::cons(Adt142::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt142::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1618 = v26;
                                            match m1618.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C5(f306(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f500({
                                                        let (h1619, _) = v28;
                                                        h1619
                                                    })?;
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0({
                                                        let (_, h1620) = v28;
                                                        h1620
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
                Ok(next) => { m1621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436() -> u64 {
    67108864u64
}

pub fn f437(v0: Adt19) -> Adt6 {
    let m1625 = v0;
    match m1625 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f438(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1626 = v0;
    match m1626 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f439(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1627 = v0;
    match m1627 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f440() -> i64 {
    -32603i64
}

pub fn f441() -> i64 {
    429i64
}

pub fn f442() -> i64 {
    -32021i64
}

pub fn f443(p1630: List<Str>, p1631: List<Adt6>) -> List<Adt6> {
    let m1629 = (p1630, p1631);
    {
        let mut m1629 = m1629;
        loop {
            let step = {
                let (v0, v1) = m1629;
                let m1628 = v0;
                match m1628.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f444(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1632 = v1;
    match m1632 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1633) => {
                    h1633
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f445(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt120>) -> R<Result<Adt116, Adt119>> {
    let m1634 = v3;
    match m1634 {
        Ok(v5) => {
            f501(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt116, Adt119>(Adt119::C0(v4)))
        }
    }
}

pub fn f446(v0: Adt128, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt120> {
    let m1635 = v0;
    match m1635 {
        Adt128::C0 => {
            f502(v1, v2)
        }
        Adt128::C1 => {
            f503(v1, v2)
        }
        Adt128::C2 => {
            f504(v1, v2)
        }
        Adt128::C3 => {
            f505(v1, v2)
        }
        Adt128::C4 => {
            f506(v1, v2)
        }
        Adt128::C5 => {
            f507(v1, v2)
        }
        Adt128::C6 => {
            f508(v1, v2)
        }
        Adt128::C7 => {
            f509(v1, v2)
        }
        Adt128::C8 => {
            f510(v1, v2)
        }
        Adt128::C9 => {
            f511(v1, v2)
        }
        Adt128::C10 => {
            f512(v1, v2)
        }
        Adt128::C11 => {
            f513(v1, v2)
        }
    }
}

pub fn f447(v0: Adt128) -> Str {
    let m1636 = v0;
    match m1636 {
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

pub fn f448(v0: Option<Adt8>) -> bool {
    let m1637 = v0;
    match m1637 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f449(v0: Option<Adt9>) -> bool {
    let m1638 = v0;
    match m1638 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f450(v0: Option<Adt5>) -> bool {
    let m1639 = v0;
    match m1639 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f451(v0: Option<Adt5>, v1: Str) -> bool {
    let m1640 = v0;
    match m1640 {
        None => {
            false
        }
        Some(v2) => {
            f237(v1, v2)
        }
    }
}

pub fn f452() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f453(v0: List<Str>, v1: Adt95, v2: Adt16, v3: Option<Adt97>) -> Adt136 {
    let m1641 = v3;
    match m1641 {
        None => {
            f514(v1, v2.clone(), f515(match v2 {
                Adt16::C0(h1642, _, _, _, _, _) => {
                    h1642
                }
            }, v0))
        }
        Some(v4) => {
            Adt136::C1(Adt119::C2(v4))
        }
    }
}

pub fn f454(v0: Option<Adt96>, v1: Adt138, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt97>> {
    let m1643 = v0;
    match m1643 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v5) => {
            let m1645 = match v5.clone() {
                Adt96::C0(_, _, _, h1644, _) => {
                    h1644
                }
            };
            match m1645 {
                None => {
                    f516(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f455(v0: Option<Adt5>) -> Adt5 {
    let m1646 = v0;
    match m1646 {
        None => {
            f28()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f456(v0: Adt5) -> Result<Adt16, Adt120> {
    let m1647 = f517(v0.clone());
    match m1647 {
        Adt145::C0 => {
            f518(v0.clone(), f177(f519(), v0.clone()), f177(f520(), v0))
        }
        Adt145::C1 => {
            Err::<Adt16, Adt120>(Adt120::C2)
        }
        Adt145::C2 => {
            Err::<Adt16, Adt120>(Adt120::C3)
        }
    }
}

pub fn f457(v0: u64) -> R<u64> {
    let a1648 = 64u64;
    let a1649 = v0;
    if if nat_lt(a1648, a1649) {
        let a1650 = v0;
        let a1651 = 91u64;
        nat_lt(a1650, a1651)
    } else {
        false
    } {
        let a1652 = v0;
        let a1653 = 32u64;
        nat_add(a1652, a1653)
    } else {
        Ok(v0)
    }
}

pub fn f458(p1656: List<u64>) -> List<u64> {
    let m1655 = p1656;
    {
        let mut m1655 = m1655;
        loop {
            let step = {
                let v0 = m1655;
                let m1654 = v0;
                match m1654.uncons() {
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
                Ok(next) => { m1655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: u64) -> bool {
    let a1657 = v0;
    let a1658 = 9u64;
    if nat_eq(a1657, a1658) {
        true
    } else {
        let a1659 = 31u64;
        let a1660 = v0;
        if nat_lt(a1659, a1660) {
            let a1661 = v0;
            let a1662 = 127u64;
            nat_lt(a1661, a1662)
        } else {
            false
        }
    }
}

pub fn f460(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1663 = f522(v0)?;
    match m1663 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1664 = f522(v1)?;
            match m1664 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1665 = v2;
                    let a1666 = 61u64;
                    if nat_eq(a1665, a1666) {
                        f523(v5, v6, v4, v3)
                    } else {
                        let m1667 = f522(v2)?;
                        match m1667 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1668 = v3;
                                let a1669 = 61u64;
                                if nat_eq(a1668, a1669) {
                                    f524(v5, v6, v7, v4)
                                } else {
                                    let m1670 = f522(v3)?;
                                    match m1670 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f525(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f461(v0: List<u64>) -> bool {
    let m1671 = v0;
    match m1671.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f462() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f463(v0: Str) -> R<List<u64>> {
    f69({
        let a1672 = v0;
        utf8_encode(a1672)
    })
}

pub fn f464(v0: u64, v1: u64) -> u64 {
    let a1673 = v0;
    let a1674 = v1;
    let a1675 = 0u64;
    nat_rem(a1673, a1674, a1675)
}

pub fn f465(v0: u64, v1: u64) -> u64 {
    let a1676 = v0;
    let a1677 = v1;
    let a1678 = 0u64;
    nat_quot(a1676, a1677, a1678)
}

pub fn f466(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1679 = f526(f527(), v2.clone());
    match m1679 {
        None => {
            let m1680 = f528(f529(), v2.clone());
            match m1680 {
                None => {
                    f530(v0, v1, v2)
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

pub fn f467(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt132) -> R<Adt144> {
    let m1681 = f236(Str::lit("x-mcp-header"), v1.clone());
    match m1681 {
        Adt123::C0 => {
            Ok(Adt144::C1(v4))
        }
        Adt123::C1(v5) => {
            f531(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f468(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt125>) -> List<Adt125> {
    f532(Str::lit("properties"), v0.clone(), v1, v2, f532(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f533(Str::lit("items"), v0.clone(), f533(Str::lit("additionalProperties"), v0.clone(), f534(Str::lit("allOf"), v0.clone(), f534(Str::lit("anyOf"), v0.clone(), f534(Str::lit("oneOf"), v0.clone(), f533(Str::lit("not"), v0.clone(), f533(Str::lit("if"), v0.clone(), f533(Str::lit("then"), v0.clone(), f533(Str::lit("else"), v0, v3)))))))))))
}

pub fn f469(p1688: List<(Str, Adt143)>) -> Adt133 {
    let m1687 = p1688;
    {
        let mut m1687 = m1687;
        loop {
            let step = {
                let v0 = m1687;
                let m1682 = v0;
                match m1682.uncons() {
                    None => {
                        Err::<List<(Str, Adt143)>, Adt133>(Adt133::C3(List::<Adt112>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1684 = {
                            let (_, h1683) = v1;
                            h1683
                        };
                        match m1684 {
                            Adt143::C0 => {
                                Ok::<List<(Str, Adt143)>, Adt133>(v2)
                            }
                            Adt143::C1 => {
                                Err::<List<(Str, Adt143)>, Adt133>(Adt133::C0({
                                    let (h1685, _) = v1;
                                    h1685
                                }))
                            }
                            Adt143::C2 => {
                                Err::<List<(Str, Adt143)>, Adt133>(Adt133::C1({
                                    let (h1686, _) = v1;
                                    h1686
                                }))
                            }
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

pub fn f470(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1689 = f236(Str::lit("type"), v0);
    match m1689 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1690 = f233(v2.clone());
            match m1690 {
                None => {
                    let m1691 = f408(v2);
                    match m1691 {
                        None => {
                            Ok(Adt143::C2)
                        }
                        Some(v4) => {
                            f537(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f535(v3.clone()) {
                        if f536(v3, v1)? {
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

pub fn f471(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1692 = f236(Str::lit("enum"), v0);
    match m1692 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1693 = f408(v2);
            match m1693 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v3) => {
                    f538(v3, v1)
                }
            }
        }
    }
}

pub fn f472(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1694 = f236(Str::lit("const"), v0);
    match m1694 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1695 = f539(v2, v1)?;
            match m1695 {
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

pub fn f473(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt143> {
    let m1696 = f236(v0, v2);
    match m1696 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v4) => {
            let m1697 = f540(v4);
            match m1697 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v5) => {
                    let m1698 = f540(v3);
                    match m1698 {
                        None => {
                            Ok(Adt143::C0)
                        }
                        Some(v6) => {
                            if f541(v1, f542(v6, v5)?) {
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

pub fn f474(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt143)>> {
    let m1699 = v1;
    match m1699 {
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
            Ok(List::cons((Str::lit("minLength"), f543(Str::lit("minLength"), true, v0.clone(), f544(v4.clone())?)?), List::cons((Str::lit("maxLength"), f543(Str::lit("maxLength"), false, v0, f544(v4)?)?), List::<(Str, Adt143)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f543(Str::lit("minItems"), true, v0.clone(), f545(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f543(Str::lit("maxItems"), false, v0, f545(v5, 0u64)?)?), List::<(Str, Adt143)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f543(Str::lit("minProperties"), true, v0.clone(), f546(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f543(Str::lit("maxProperties"), false, v0, f546(v6, 0u64)?)?), List::<(Str, Adt143)>::nil())))
        }
    }
}

pub fn f475(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt143 {
    let m1700 = f236(Str::lit("required"), v0);
    match m1700 {
        Adt123::C0 => {
            Adt143::C0
        }
        Adt123::C1(v2) => {
            let m1701 = f408(v2);
            match m1701 {
                None => {
                    Adt143::C2
                }
                Some(v3) => {
                    let m1702 = f329(v1);
                    match m1702 {
                        None => {
                            Adt143::C0
                        }
                        Some(v4) => {
                            f547(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f476(v0: Adt133, v1: Adt133) -> Adt133 {
    let m1703 = v0.clone();
    match m1703 {
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
            let m1704 = v1.clone();
            match m1704 {
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
                        let a1705 = v5;
                        let a1706 = v9;
                        append_list(a1705, a1706)
                    })
                }
            }
        }
    }
}

pub fn f477(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> Adt133 {
    let m1707 = f236(Str::lit("$ref"), v1);
    match m1707 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v4) => {
            let m1708 = f233(v4);
            match m1708 {
                None => {
                    Adt133::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1709 = f548(v0, v5.clone());
                    match m1709 {
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

pub fn f478(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1710 = f236(Str::lit("properties"), v0);
    match m1710 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1711 = f329(v3);
            match m1711 {
                None => {
                    Adt133::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1712 = f329(v1);
                    match m1712 {
                        None => {
                            Adt133::C3(List::<Adt112>::nil())
                        }
                        Some(v5) => {
                            Adt133::C3(f549(match v4 {
                                Adt5::C0(h1713) => {
                                    h1713
                                }
                            }, match v5 {
                                Adt5::C0(h1714) => {
                                    h1714
                                }
                            }, v2, List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f479(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1715 = f236(Str::lit("additionalProperties"), v0.clone());
    match m1715 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1716 = f329(v1);
            match m1716 {
                None => {
                    Adt133::C3(List::<Adt112>::nil())
                }
                Some(v4) => {
                    let m1717 = f328(v3.clone());
                    match m1717 {
                        None => {
                            Adt133::C3(f552(match v4 {
                                Adt5::C0(h1719) => {
                                    h1719
                                }
                            }, f551(v0), v3, v2, List::<Adt112>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt133::C3(List::<Adt112>::nil())
                            } else {
                                if f550(match v4 {
                                    Adt5::C0(h1718) => {
                                        h1718
                                    }
                                }, f551(v0)) {
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

pub fn f480(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> R<Adt133> {
    let m1720 = f236(Str::lit("items"), v0);
    match m1720 {
        Adt123::C0 => {
            Ok(Adt133::C3(List::<Adt112>::nil()))
        }
        Adt123::C1(v3) => {
            let m1721 = f408(v1);
            match m1721 {
                None => {
                    Ok(Adt133::C3(List::<Adt112>::nil()))
                }
                Some(v4) => {
                    Ok(Adt133::C3(f553(v4, 0u64, v3, v2, List::<Adt112>::nil())?))
                }
            }
        }
    }
}

pub fn f481(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1722 = f236(Str::lit("allOf"), v0);
    match m1722 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1723 = f408(v3);
            match m1723 {
                None => {
                    Adt133::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1724 = v4.clone();
                    match m1724.uncons() {
                        None => {
                            Adt133::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt133::C3(f554(v4, v1, v2, List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f482(v0: Adt113, v1: Adt133, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt110>) -> Adt133 {
    let m1725 = v1.clone();
    match m1725 {
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
            let m1726 = v8;
            match m1726.uncons() {
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

pub fn f483(p1731: List<Adt50>, p1732: Adt7) -> R<bool> {
    let m1730 = (p1731, p1732);
    {
        let mut m1730 = m1730;
        loop {
            let step = {
                let (v0, v1) = m1730;
                let m1727 = v0;
                match m1727.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f555(f542(match v2.clone() {
                            Adt50::C0(h1728, _, _) => {
                                h1728
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1729, _, _) => {
                                    h1729
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1730 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(p1739: List<u64>, p1740: List<u64>) -> List<u64> {
    let m1738 = (p1739, p1740);
    {
        let mut m1738 = m1738;
        loop {
            let step = {
                let (v0, v1) = m1738;
                let m1733 = v0;
                match m1733.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1734 = v2;
                        let a1735 = 126u64;
                        if nat_eq(a1734, a1735) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1736 = v2;
                            let a1737 = 47u64;
                            if nat_eq(a1736, a1737) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f486() -> i64 {
    999999999999999999i64
}

pub fn f487(v0: i64) -> R<List<u64>> {
    let m1743 = {
        let a1741 = v0;
        let a1742 = 0i64;
        compare(a1741, a1742)
    };
    if match m1743 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f556(20u64, {
            let a1744 = v0;
            int_neg(a1744)?
        }, List::<u64>::nil())?))
    } else {
        f556(20u64, v0, List::<u64>::nil())
    }
}

pub fn f488(v0: u64) -> R<List<u64>> {
    let a1745 = v0;
    let a1746 = 34u64;
    if nat_eq(a1745, a1746) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1747 = v0;
        let a1748 = 92u64;
        if nat_eq(a1747, a1748) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1749 = v0;
            let a1750 = 8u64;
            if nat_eq(a1749, a1750) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1751 = v0;
                let a1752 = 9u64;
                if nat_eq(a1751, a1752) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1753 = v0;
                    let a1754 = 10u64;
                    if nat_eq(a1753, a1754) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1755 = v0;
                        let a1756 = 12u64;
                        if nat_eq(a1755, a1756) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1757 = v0;
                            let a1758 = 13u64;
                            if nat_eq(a1757, a1758) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1759 = v0;
                                let a1760 = 32u64;
                                if nat_lt(a1759, a1760) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f557(f465(v0, 16u64))?, List::cons(f557(f464(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f489(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1761 = v1;
    match m1761 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f558(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f490(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1762 = v0;
    match m1762 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1765 = f426(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1763, _) => {
                        h1763
                    }
                });
                let a1766 = f426(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1764) => {
                        h1764
                    }
                });
                append_list(a1765, a1766)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f491(v0: Adt11) -> Str {
    let m1767 = v0;
    match m1767 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f492(p1770: List<Adt35>, p1771: List<Adt6>) -> List<Adt6> {
    let m1769 = (p1770, p1771);
    {
        let mut m1769 = m1769;
        loop {
            let step = {
                let (v0, v1) = m1769;
                let m1768 = v0;
                match m1768.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f559(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(p1775: List<Adt38>, p1776: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1774 = (p1775, p1776);
    {
        let mut m1774 = m1774;
        loop {
            let step = {
                let (v0, v1) = m1774;
                let m1772 = v0;
                match m1772.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1773 = f560(v2)?;
                        match m1773 {
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
                Ok(next) => { m1774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(p1779: List<Adt44>, p1780: List<Adt6>) -> R<List<Adt6>> {
    let m1778 = (p1779, p1780);
    {
        let mut m1778 = m1778;
        loop {
            let step = {
                let (v0, v1) = m1778;
                let m1777 = v0;
                match m1777.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f561(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1778 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(p1783: List<Adt46>, p1784: List<Adt6>) -> List<Adt6> {
    let m1782 = (p1783, p1784);
    {
        let mut m1782 = m1782;
        loop {
            let step = {
                let (v0, v1) = m1782;
                let m1781 = v0;
                match m1781.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f562(v2), v1)))
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

pub fn f496(p1787: List<Adt54>, p1788: List<Adt6>) -> List<Adt6> {
    let m1786 = (p1787, p1788);
    {
        let mut m1786 = m1786;
        loop {
            let step = {
                let (v0, v1) = m1786;
                let m1785 = v0;
                match m1785.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f563(v2), v1)))
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

pub fn f497(p1791: List<Adt56>, p1792: List<Adt6>) -> List<Adt6> {
    let m1790 = (p1791, p1792);
    {
        let mut m1790 = m1790;
        loop {
            let step = {
                let (v0, v1) = m1790;
                let m1789 = v0;
                match m1789.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f564(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(p1796: List<Adt59>, p1797: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1795 = (p1796, p1797);
    {
        let mut m1795 = m1795;
        loop {
            let step = {
                let (v0, v1) = m1795;
                let m1793 = v0;
                match m1793.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1794 = f565(v2)?;
                        match m1794 {
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
                Ok(next) => { m1795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1798 = v0;
    match m1798 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f566(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f500(v0: Str) -> R<Str> {
    let m1801 = {
        let a1800 = f268(f269(), f211(f567(f69({
            let a1799 = v0;
            utf8_encode(a1799)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1800)
    };
    match m1801 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f501(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt116, Adt119>> {
    if f568(f569(v3.clone())) {
        Ok(Ok::<Adt116, Adt119>(f570(v0.clone(), Adt32::C0(f571(v0)))))
    } else {
        Ok(f572(v0.clone(), v3.clone(), f573(match v0 {
            Adt0::C0(_, _, _, _, _, h1802) => {
                h1802
            }
        }, v2, v1, v3)?))
    }
}

pub fn f502(v0: Adt16, _: Adt5) -> Result<Adt14, Adt120> {
    Ok::<Adt14, Adt120>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f503(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1803 = f574(Str::lit("cursor"), v1);
    match m1803 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f504(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1804 = f575(Str::lit("name"), v1.clone());
    match m1804 {
        Ok(v2) => {
            let m1805 = f576(Str::lit("arguments"), v1.clone());
            match m1805 {
                Ok(v3) => {
                    let m1806 = f576(Str::lit("inputResponses"), v1.clone());
                    match m1806 {
                        Ok(v4) => {
                            let m1807 = f574(Str::lit("requestState"), v1);
                            match m1807 {
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

pub fn f505(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1808 = f574(Str::lit("cursor"), v1);
    match m1808 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f506(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1809 = f575(Str::lit("uri"), v1.clone());
    match m1809 {
        Ok(v2) => {
            let m1810 = f576(Str::lit("inputResponses"), v1.clone());
            match m1810 {
                Ok(v3) => {
                    let m1811 = f574(Str::lit("requestState"), v1);
                    match m1811 {
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

pub fn f507(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1812 = f574(Str::lit("cursor"), v1);
    match m1812 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f508(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1813 = f574(Str::lit("cursor"), v1);
    match m1813 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f509(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1814 = f575(Str::lit("name"), v1.clone());
    match m1814 {
        Ok(v2) => {
            let m1815 = f577(Str::lit("arguments"), v1.clone());
            match m1815 {
                Ok(v3) => {
                    let m1816 = f576(Str::lit("inputResponses"), v1.clone());
                    match m1816 {
                        Ok(v4) => {
                            let m1817 = f574(Str::lit("requestState"), v1);
                            match m1817 {
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

pub fn f510(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1818 = f578(Str::lit("ref"), v1.clone());
    match m1818 {
        Ok(v2) => {
            let m1819 = f579(v2);
            match m1819 {
                Ok(v3) => {
                    let m1820 = f578(Str::lit("argument"), v1.clone());
                    match m1820 {
                        Ok(v4) => {
                            let m1821 = f580(v4);
                            match m1821 {
                                Ok(v5) => {
                                    let m1822 = f581(v1);
                                    match m1822 {
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

pub fn f511(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1823 = f575(Str::lit("taskId"), v1);
    match m1823 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f512(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1824 = f575(Str::lit("taskId"), v1.clone());
    match m1824 {
        Ok(v2) => {
            let m1825 = f578(Str::lit("inputResponses"), v1);
            match m1825 {
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

pub fn f513(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1826 = f575(Str::lit("taskId"), v1);
    match m1826 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f514(v0: Adt95, v1: Adt16, v2: Option<Adt119>) -> Adt136 {
    let m1827 = v2;
    match m1827 {
        None => {
            Adt136::C0(Adt137::C0(match v0.clone() {
                Adt95::C0(h1828, _, _) => {
                    h1828
                }
            }, match v0.clone() {
                Adt95::C0(_, h1829, _) => {
                    h1829
                }
            }, v1, f455(match v0 {
                Adt95::C0(_, _, h1830) => {
                    h1830
                }
            })))
        }
        Some(v3) => {
            Adt136::C1(v3)
        }
    }
}

pub fn f515(v0: Str, v1: List<Str>) -> Option<Adt119> {
    f582(f583(v0.clone(), v1.clone()), v0, v1)
}

pub fn f516(v0: Adt96, v1: Adt138, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt97>> {
    let m1832 = match v0.clone() {
        Adt96::C0(h1831, _, _, _, _) => {
            h1831
        }
    };
    match m1832 {
        None => {
            Ok(Some(Adt97::C0))
        }
        Some(v5) => {
            let a1833 = v5;
            let a1834 = v3;
            if equal(a1833, a1834) {
                let m1836 = match v0.clone() {
                    Adt96::C0(_, h1835, _, _, _) => {
                        h1835
                    }
                };
                match m1836 {
                    None => {
                        Ok(Some(Adt97::C1))
                    }
                    Some(v6) => {
                        let a1837 = v6;
                        let a1838 = v2;
                        if equal(a1837, a1838) {
                            f584(f585(match v0.clone() {
                                Adt96::C0(_, _, h1839, _, _) => {
                                    h1839
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt96::C0(_, _, _, _, h1840) => {
                                    h1840
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

pub fn f517(v0: Adt5) -> Adt145 {
    f586(f177(f519(), v0.clone()), f177(f520(), v0))
}

pub fn f518(v0: Adt5, v1: Adt123, v2: Adt123) -> Result<Adt16, Adt120> {
    let m1841 = v1;
    match m1841 {
        Adt123::C0 => {
            Err::<Adt16, Adt120>(Adt120::C2)
        }
        Adt123::C1(v3) => {
            let m1842 = v2;
            match m1842 {
                Adt123::C0 => {
                    Err::<Adt16, Adt120>(Adt120::C3)
                }
                Adt123::C1(v4) => {
                    f587(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f519() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f520() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f521(v0: u64) -> bool {
    let a1843 = v0;
    let a1844 = 9u64;
    if nat_eq(a1843, a1844) {
        true
    } else {
        let a1845 = v0;
        let a1846 = 32u64;
        nat_eq(a1845, a1846)
    }
}

pub fn f522(v0: u64) -> R<Option<u64>> {
    let a1847 = 64u64;
    let a1848 = v0;
    if if nat_lt(a1847, a1848) {
        let a1849 = v0;
        let a1850 = 91u64;
        nat_lt(a1849, a1850)
    } else {
        false
    } {
        Ok(Some({
            let a1851 = v0;
            let a1852 = 65u64;
            nat_sub(a1851, a1852)
        }))
    } else {
        let a1853 = 96u64;
        let a1854 = v0;
        if if nat_lt(a1853, a1854) {
            let a1855 = v0;
            let a1856 = 123u64;
            nat_lt(a1855, a1856)
        } else {
            false
        } {
            Ok(Some({
                let a1857 = v0;
                let a1858 = 71u64;
                nat_sub(a1857, a1858)
            }))
        } else {
            let a1859 = 47u64;
            let a1860 = v0;
            if if nat_lt(a1859, a1860) {
                let a1861 = v0;
                let a1862 = 58u64;
                nat_lt(a1861, a1862)
            } else {
                false
            } {
                Ok(Some({
                    let a1863 = v0;
                    let a1864 = 4u64;
                    nat_add(a1863, a1864)?
                }))
            } else {
                let a1865 = v0;
                let a1866 = 43u64;
                if nat_eq(a1865, a1866) {
                    Ok(Some(62u64))
                } else {
                    let a1867 = v0;
                    let a1868 = 47u64;
                    if nat_eq(a1867, a1868) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f523(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1869 = v3;
        let a1870 = 61u64;
        if nat_eq(a1869, a1870) {
            let a1871 = f588(v1, 16u64);
            let a1872 = 0u64;
            nat_eq(a1871, a1872)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f589(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f524(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1873 = f588(v2, 4u64);
        let a1874 = 0u64;
        nat_eq(a1873, a1874)
    } else {
        false
    } {
        Ok(Some(f590(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f525(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1877 = {
            let a1875 = v0;
            let a1876 = 4u64;
            nat_mul(a1875, a1876)?
        };
        let a1878 = f591(v1, 16u64);
        nat_add(a1877, a1878)?
    }, List::cons({
        let a1881 = {
            let a1879 = f588(v1, 16u64);
            let a1880 = 16u64;
            nat_mul(a1879, a1880)?
        };
        let a1882 = f591(v2, 4u64);
        nat_add(a1881, a1882)?
    }, List::cons({
        let a1885 = {
            let a1883 = f588(v2, 4u64);
            let a1884 = 64u64;
            nat_mul(a1883, a1884)?
        };
        let a1886 = v3;
        nat_add(a1885, a1886)?
    }, List::<u64>::nil()))))
}

pub fn f526(p1890: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1889 = p1890;
    {
        let mut m1889 = m1889;
        loop {
            let step = {
                let v0 = m1889;
                let m1887 = v0;
                match m1887.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1888 = f236(v2.clone(), v1.clone());
                        match m1888 {
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
                Ok(next) => { m1889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f528(p1897: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1896 = p1897;
    {
        let mut m1896 = m1896;
        loop {
            let step = {
                let v0 = m1896;
                let m1891 = v0;
                match m1891.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1893 = f236({
                            let (h1892, _) = v2.clone();
                            h1892
                        }, v1.clone());
                        match m1893 {
                            Adt123::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt123::C1(v4) => {
                                if f592({
                                    let (_, h1894) = v2;
                                    h1894
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1895, _) = v2;
                                        h1895
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f530(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1898 = f236(Str::lit("$ref"), v2);
    match m1898 {
        Adt123::C0 => {
            None::<Adt94>
        }
        Adt123::C1(v3) => {
            let m1899 = f233(v3);
            match m1899 {
                None => {
                    None::<Adt94>
                }
                Some(v4) => {
                    let m1900 = f548(v1, v4.clone());
                    match m1900 {
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

pub fn f531(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt132) -> R<Adt144> {
    if v3 {
        let m1901 = f233(v2);
        match m1901 {
            None => {
                Ok(Adt144::C0(Adt94::C7(v0)))
            }
            Some(v6) => {
                f593(v0, v1, v6.clone(), f187(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt144::C0(Adt94::C10(v0)))
    }
}

pub fn f532(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt125>) -> List<Adt125> {
    let m1902 = f236(v0.clone(), v1);
    match m1902 {
        Adt123::C0 => {
            v4
        }
        Adt123::C1(v5) => {
            let m1903 = f329(v5);
            match m1903 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1904 = v0;
                    let a1905 = Str::lit("properties");
                    if equal(a1904, a1905) {
                        f594(match v6 {
                            Adt5::C0(h1906) => {
                                h1906
                            }
                        }, v2, v3, v4)
                    } else {
                        f595(match v6 {
                            Adt5::C0(h1907) => {
                                h1907
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f533(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt125>) -> List<Adt125> {
    let m1908 = f236(v0, v1);
    match m1908 {
        Adt123::C0 => {
            v2
        }
        Adt123::C1(v3) => {
            List::cons(Adt125::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f534(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt125>) -> List<Adt125> {
    let m1909 = f236(v0, v1);
    match m1909 {
        Adt123::C0 => {
            v2
        }
        Adt123::C1(v3) => {
            let m1910 = f408(v3);
            match m1910 {
                None => {
                    v2
                }
                Some(v4) => {
                    f596(v4, v2)
                }
            }
        }
    }
}

pub fn f535(v0: Str) -> bool {
    let a1911 = v0.clone();
    let a1912 = Str::lit("null");
    if equal(a1911, a1912) {
        true
    } else {
        let a1913 = v0.clone();
        let a1914 = Str::lit("boolean");
        if equal(a1913, a1914) {
            true
        } else {
            let a1915 = v0.clone();
            let a1916 = Str::lit("object");
            if equal(a1915, a1916) {
                true
            } else {
                let a1917 = v0.clone();
                let a1918 = Str::lit("array");
                if equal(a1917, a1918) {
                    true
                } else {
                    let a1919 = v0.clone();
                    let a1920 = Str::lit("number");
                    if equal(a1919, a1920) {
                        true
                    } else {
                        let a1921 = v0.clone();
                        let a1922 = Str::lit("string");
                        if equal(a1921, a1922) {
                            true
                        } else {
                            let a1923 = v0;
                            let a1924 = Str::lit("integer");
                            equal(a1923, a1924)
                        }
                    }
                }
            }
        }
    }
}

pub fn f536(v0: Str, v1: Adt6) -> R<bool> {
    let a1925 = v0.clone();
    let a1926 = f597(v1.clone());
    if equal(a1925, a1926) {
        Ok(true)
    } else {
        let a1927 = v0;
        let a1928 = Str::lit("integer");
        if equal(a1927, a1928) {
            let m1929 = f540(v1);
            match m1929 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f598(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f537(p1932: List<Adt6>, v1: Adt6, p1933: bool) -> R<Adt143> {
    let m1931 = (p1932, p1933);
    {
        let mut m1931 = m1931;
        loop {
            let step = {
                let (v0, v2) = m1931;
                let m1930 = v0;
                match m1930.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f535(f599(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt143>>((v4, if v2 {
                                true
                            } else {
                                f536(f599(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(p1937: List<Adt6>, v1: Adt6) -> R<Adt143> {
    let m1936 = p1937;
    {
        let mut m1936 = m1936;
        loop {
            let step = {
                let v0 = m1936;
                let m1934 = v0;
                match m1934.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt143>>(Ok(Adt143::C1))
                    }
                    Some((v2, v3)) => {
                        let m1935 = f539(v2, v1.clone())?;
                        match m1935 {
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
                Ok(next) => { m1936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f600(f601(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f540(v0: Adt6) -> Option<Adt7> {
    let m1938 = v0;
    match m1938 {
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

pub fn f541(v0: u64, v1: Adt147) -> bool {
    let a1939 = v0;
    let a1940 = 0u64;
    if nat_eq(a1939, a1940) {
        let a1941 = f602(v1);
        bool_not(a1941)
    } else {
        let a1942 = v0;
        let a1943 = 1u64;
        if nat_eq(a1942, a1943) {
            let a1944 = f555(v1);
            bool_not(a1944)
        } else {
            let a1945 = v0;
            let a1946 = 2u64;
            if nat_eq(a1945, a1946) {
                f555(v1)
            } else {
                f602(v1)
            }
        }
    }
}

pub fn f542(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f603(v0.clone(), v1.clone(), f604(match v0 {
        Adt7::C0(h1947, _) => {
            h1947
        }
    }), f604(match v1 {
        Adt7::C0(h1948, _) => {
            h1948
        }
    }))
}

pub fn f543(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt143> {
    let m1949 = f236(v0, v2);
    match m1949 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v4) => {
            let m1950 = f605(v4);
            match m1950 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v5) => {
                    let m1953 = {
                        let a1951 = v5;
                        let a1952 = 0i64;
                        compare(a1951, a1952)
                    };
                    if match m1953 {
                        Ordering::Less => {
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
                            if f602(f606(f194(v3, 0i64)?, v5)) {
                                Ok(Adt143::C1)
                            } else {
                                Ok(Adt143::C0)
                            }
                        } else {
                            if f555(f606(f194(v3, 0i64)?, v5)) {
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

pub fn f544(v0: Str) -> R<u64> {
    f607(f69({
        let a1954 = v0;
        utf8_encode(a1954)
    })?, 0u64)
}

pub fn f545(p1959: List<Adt6>, p1960: u64) -> R<u64> {
    let m1958 = (p1959, p1960);
    {
        let mut m1958 = m1958;
        loop {
            let step = {
                let (v0, v1) = m1958;
                let m1955 = v0;
                match m1955.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1956 = v1;
                            let a1957 = 1u64;
                            nat_add(a1956, a1957)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1958 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(p1965: List<(Str, Adt6)>, p1966: u64) -> R<u64> {
    let m1964 = (p1965, p1966);
    {
        let mut m1964 = m1964;
        loop {
            let step = {
                let (v0, v1) = m1964;
                let m1961 = v0;
                match m1961.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1962 = v1;
                            let a1963 = 1u64;
                            nat_add(a1962, a1963)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1964 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(p1970: List<Adt6>, v1: Adt5) -> Adt143 {
    let m1969 = p1970;
    {
        let mut m1969 = m1969;
        loop {
            let step = {
                let v0 = m1969;
                let m1967 = v0;
                match m1967.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt143>(Adt143::C0)
                    }
                    Some((v2, v3)) => {
                        let m1968 = f233(v2);
                        match m1968 {
                            None => {
                                Err::<List<Adt6>, Adt143>(Adt143::C2)
                            }
                            Some(v4) => {
                                if f237(v4, v1.clone()) {
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
                Ok(next) => { m1969 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1971 = v1.clone();
    let a1972 = Str::lit("#");
    if equal(a1971, a1972) {
        Some(v0)
    } else {
        let m1973 = f329(v0);
        match m1973 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1974 = f177(Str::lit("$defs"), v2);
                match m1974 {
                    Adt123::C0 => {
                        None::<Adt6>
                    }
                    Adt123::C1(v3) => {
                        let m1975 = f329(v3);
                        match m1975 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f608(v1, match v4 {
                                    Adt5::C0(h1976) => {
                                        h1976
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

pub fn f549(p1983: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt110>, p1984: List<Adt112>) -> List<Adt112> {
    let m1982 = (p1983, p1984);
    {
        let mut m1982 = m1982;
        loop {
            let step = {
                let (v0, v3) = m1982;
                let m1977 = v0;
                match m1977.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1979 = f236({
                            let (h1978, _) = v4.clone();
                            h1978
                        }, v1.clone());
                        match m1979 {
                            Adt123::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v5, v3))
                            }
                            Adt123::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0({
                                    let (_, h1980) = v4;
                                    h1980
                                }, v6, List::cons(Adt110::C0({
                                    let (h1981, _) = v4;
                                    h1981
                                }), v2.clone())), v3)))
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

pub fn f550(p1989: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1988 = p1989;
    {
        let mut m1988 = m1988;
        loop {
            let step = {
                let v0 = m1988;
                let m1985 = v0;
                match m1985.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1987 = f236({
                            let (h1986, _) = v2;
                            h1986
                        }, v1.clone());
                        match m1987 {
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
                Ok(next) => { m1988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1990 = f236(Str::lit("properties"), v0);
    match m1990 {
        Adt123::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt123::C1(v1) => {
            let m1991 = f329(v1);
            match m1991 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1992) => {
                            h1992
                        }
                    }
                }
            }
        }
    }
}

pub fn f552(p1999: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>, p2000: List<Adt112>) -> List<Adt112> {
    let m1998 = (p1999, p2000);
    {
        let mut m1998 = m1998;
        loop {
            let step = {
                let (v0, v4) = m1998;
                let m1993 = v0;
                match m1993.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1995 = f236({
                            let (h1994, _) = v5.clone();
                            h1994
                        }, v1.clone());
                        match m1995 {
                            Adt123::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v6, List::cons(Adt112::C0(v2.clone(), {
                                    let (_, h1996) = v5;
                                    h1996
                                }, List::cons(Adt110::C0({
                                    let (h1997, _) = v5;
                                    h1997
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
                Ok(next) => { m1998 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(p2005: List<Adt6>, p2006: u64, v2: Adt6, v3: List<Adt110>, p2007: List<Adt112>) -> R<List<Adt112>> {
    let m2004 = (p2005, (p2006, p2007));
    {
        let mut m2004 = m2004;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2004;
                let m2001 = v0;
                match m2001.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt112>)), R<List<Adt112>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt112>)), R<List<Adt112>>>((v6, ({
                            let a2002 = v1;
                            let a2003 = 1u64;
                            nat_add(a2002, a2003)?
                        }, List::cons(Adt112::C0(v2.clone(), v5, List::cons(Adt110::C1(v1), v3.clone())), v4))))
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

pub fn f554(p2010: List<Adt6>, v1: Adt6, v2: List<Adt110>, p2011: List<Adt112>) -> List<Adt112> {
    let m2009 = (p2010, p2011);
    {
        let mut m2009 = m2009;
        loop {
            let step = {
                let (v0, v3) = m2009;
                let m2008 = v0;
                match m2008.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2009 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(v0: Adt147) -> bool {
    let m2012 = v0;
    match m2012 {
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

pub fn f556(p2024: u64, p2025: i64, p2026: List<u64>) -> R<List<u64>> {
    let m2023 = (p2024, (p2025, p2026));
    {
        let mut m2023 = m2023;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2023;
                let m2013 = v0;
                if m2013 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2013 - 1;
                    let v4: i64 = {
                        let a2014 = v1;
                        let a2015 = 10i64;
                        let a2016 = 0i64;
                        int_quot(a2014, a2015, a2016)?
                    };
                    let v5: i64 = {
                        let a2017 = v1;
                        let a2018 = 10i64;
                        let a2019 = 0i64;
                        int_rem(a2017, a2018, a2019)
                    };
                    let m2022 = {
                        let a2020 = v4;
                        let a2021 = 1i64;
                        compare(a2020, a2021)
                    };
                    if match m2022 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f609(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f609(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2023 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f557(v0: u64) -> R<u64> {
    let a2027 = v0;
    let a2028 = 10u64;
    if nat_lt(a2027, a2028) {
        let a2029 = 48u64;
        let a2030 = v0;
        nat_add(a2029, a2030)
    } else {
        let a2031 = 87u64;
        let a2032 = v0;
        nat_add(a2031, a2032)
    }
}

pub fn f558(v0: Adt8) -> Adt6 {
    Adt6::C5(f426(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2033) => {
            h2033
        }
    }))
}

pub fn f559(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2034, _, _, _, _, _, _, _) => {
            h2034
        }
    })), {
        let a2052 = f421(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2035, _, _, _, _, _, _) => {
                h2035
            }
        });
        let a2053 = {
            let a2050 = f499(match v0.clone() {
                Adt35::C0(_, _, h2036, _, _, _, _, _) => {
                    h2036
                }
            });
            let a2051 = {
                let a2048 = f421(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2037, _, _, _, _) => {
                        h2037
                    }
                });
                let a2049 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2040 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2039, _, _, _) => {
                            h2039
                        }
                    };
                    match m2040 {
                        Adt5::C0(h2038) => {
                            h2038
                        }
                    }
                })), {
                    let a2046 = f444(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2041, _, _) => {
                            h2041
                        }
                    });
                    let a2047 = {
                        let a2044 = f610(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2042, _) => {
                                h2042
                            }
                        });
                        let a2045 = f611(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2043) => {
                                h2043
                            }
                        });
                        append_list(a2044, a2045)
                    };
                    append_list(a2046, a2047)
                });
                append_list(a2048, a2049)
            };
            append_list(a2050, a2051)
        };
        append_list(a2052, a2053)
    }))
}

pub fn f560(v0: Adt38) -> R<Option<Adt6>> {
    let m2054 = v0;
    match m2054 {
        Adt38::C0(v1) => {
            Ok(Some(f612(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f613(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2055, _, _, _) => {
                    h2055
                }
            }, match v2.clone() {
                Adt42::C0(_, h2056, _, _) => {
                    h2056
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2057, _) => {
                    h2057
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2058) => {
                    h2058
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f613(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2059, _, _, _) => {
                    h2059
                }
            }, match v3.clone() {
                Adt43::C0(_, h2060, _, _) => {
                    h2060
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2061, _) => {
                    h2061
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2062) => {
                    h2062
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f614(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f615(v5)))
        }
    }
}

pub fn f561(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2086 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2063, _, _, _, _, _, _, _, _) => {
                h2063
            }
        })), List::<(Str, Adt6)>::nil());
        let a2087 = {
            let a2084 = f421(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2064, _, _, _, _, _, _, _) => {
                    h2064
                }
            });
            let a2085 = {
                let a2082 = f499(match v0.clone() {
                    Adt44::C0(_, _, h2065, _, _, _, _, _, _) => {
                        h2065
                    }
                });
                let a2083 = {
                    let a2080 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2066, _, _, _, _, _) => {
                            h2066
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2081 = {
                        let a2078 = f421(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2067, _, _, _, _) => {
                                h2067
                            }
                        });
                        let a2079 = {
                            let a2076 = f421(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2068, _, _, _) => {
                                    h2068
                                }
                            });
                            let a2077 = {
                                let a2074 = f616(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2069, _, _) => {
                                        h2069
                                    }
                                });
                                let a2075 = {
                                    let a2072 = f617(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2070, _) => {
                                            h2070
                                        }
                                    })?;
                                    let a2073 = f611(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2071) => {
                                            h2071
                                        }
                                    });
                                    append_list(a2072, a2073)
                                };
                                append_list(a2074, a2075)
                            };
                            append_list(a2076, a2077)
                        };
                        append_list(a2078, a2079)
                    };
                    append_list(a2080, a2081)
                };
                append_list(a2082, a2083)
            };
            append_list(a2084, a2085)
        };
        append_list(a2086, a2087)
    }))
}

pub fn f562(v0: Adt46) -> Adt6 {
    let m2088 = v0;
    match m2088 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2089, _, _, _) => {
                    h2089
                }
            })), {
                let a2093 = f421(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2090, _, _) => {
                        h2090
                    }
                });
                let a2094 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2091, _) => {
                        h2091
                    }
                })), f611(match v1 {
                    Adt47::C0(_, _, _, h2092) => {
                        h2092
                    }
                }));
                append_list(a2093, a2094)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2095, _, _, _) => {
                    h2095
                }
            })), {
                let a2099 = f421(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2096, _, _) => {
                        h2096
                    }
                });
                let a2100 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2097, _) => {
                        h2097
                    }
                })), f611(match v2 {
                    Adt48::C0(_, _, _, h2098) => {
                        h2098
                    }
                }));
                append_list(a2099, a2100)
            }))
        }
    }
}

pub fn f563(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a2121 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h2101, _, _, _, _, _, _, _) => {
                h2101
            }
        })), List::<(Str, Adt6)>::nil());
        let a2122 = {
            let a2119 = f421(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h2102, _, _, _, _, _, _) => {
                    h2102
                }
            });
            let a2120 = {
                let a2117 = f499(match v0.clone() {
                    Adt54::C0(_, _, h2103, _, _, _, _, _) => {
                        h2103
                    }
                });
                let a2118 = {
                    let a2115 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h2104, _, _, _, _) => {
                            h2104
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2116 = {
                        let a2113 = f421(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h2105, _, _, _) => {
                                h2105
                            }
                        });
                        let a2114 = {
                            let a2111 = f421(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h2106, _, _) => {
                                    h2106
                                }
                            });
                            let a2112 = {
                                let a2109 = f616(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h2107, _) => {
                                        h2107
                                    }
                                });
                                let a2110 = f611(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h2108) => {
                                        h2108
                                    }
                                });
                                append_list(a2109, a2110)
                            };
                            append_list(a2111, a2112)
                        };
                        append_list(a2113, a2114)
                    };
                    append_list(a2115, a2116)
                };
                append_list(a2117, a2118)
            };
            append_list(a2119, a2120)
        };
        append_list(a2121, a2122)
    })
}

pub fn f564(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2138 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2123, _, _, _, _, _) => {
                h2123
            }
        })), List::<(Str, Adt6)>::nil());
        let a2139 = {
            let a2136 = f421(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2124, _, _, _, _) => {
                    h2124
                }
            });
            let a2137 = {
                let a2134 = f499(match v0.clone() {
                    Adt56::C0(_, _, h2125, _, _, _) => {
                        h2125
                    }
                });
                let a2135 = {
                    let a2132 = f421(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h2126, _, _) => {
                            h2126
                        }
                    });
                    let a2133 = {
                        let a2130 = {
                            let m2128 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h2127, _) => {
                                    h2127
                                }
                            };
                            match m2128 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f618(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2131 = f611(match v0 {
                            Adt56::C0(_, _, _, _, _, h2129) => {
                                h2129
                            }
                        });
                        append_list(a2130, a2131)
                    };
                    append_list(a2132, a2133)
                };
                append_list(a2134, a2135)
            };
            append_list(a2136, a2137)
        };
        append_list(a2138, a2139)
    })
}

pub fn f565(v0: Adt59) -> R<Option<Adt6>> {
    let m2141 = f560(match v0.clone() {
        Adt59::C0(_, h2140) => {
            h2140
        }
    })?;
    match m2141 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f619(match v0 {
                Adt59::C0(h2142, _) => {
                    h2142
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f566(v0: List<Adt2>) -> List<Adt6> {
    f304(f620(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f567(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f621(v0, 0u64, v1)
}

pub fn f568(v0: Adt128) -> bool {
    let m2143 = v0;
    match m2143 {
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

pub fn f569(v0: Adt14) -> Adt128 {
    let m2144 = v0;
    match m2144 {
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

pub fn f570(v0: Adt0, v1: Adt32) -> Adt116 {
    Adt116::C0(v1, Adt117::C0(Some(match v0 {
        Adt0::C0(h2145, _, _, _, _, _) => {
            h2145
        }
    }), None::<Adt5>), List::<Adt118>::nil())
}

pub fn f571(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2146, _, _) => {
            h2146
        }
    }, f175(), match v0.clone() {
        Adt0::C0(_, _, h2147, _, _, _) => {
            h2147
        }
    }, match v0 {
        Adt0::C0(_, h2148, _, _, _, _) => {
            h2148
        }
    })
}

pub fn f572(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt116, Adt119> {
    let m2149 = v2;
    match m2149 {
        Ok(v4) => {
            if f622(f569(v1.clone()), v4.clone()) {
                f623(v0, v1, v4)
            } else {
                Err::<Adt116, Adt119>(Adt119::C4(f624()))
            }
        }
        Err(v3) => {
            Err::<Adt116, Adt119>(Adt119::C4(v3))
        }
    }
}

pub fn f573(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2150 = v3.clone();
    match m2150 {
        Adt14::C0(_) => {
            let c2152: Fn0 = v2;
            c2152.apply(v3)
        }
        Adt14::C1(_) => {
            let c2153: Fn0 = v2;
            c2153.apply(v3)
        }
        Adt14::C2(_) => {
            if f625(v0, v1) {
                let c2151: Fn0 = v2;
                c2151.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2154: Fn0 = v2;
            c2154.apply(v3)
        }
        Adt14::C4(_) => {
            let c2155: Fn0 = v2;
            c2155.apply(v3)
        }
        Adt14::C5(_) => {
            let c2156: Fn0 = v2;
            c2156.apply(v3)
        }
        Adt14::C6(_) => {
            let c2157: Fn0 = v2;
            c2157.apply(v3)
        }
        Adt14::C7(_) => {
            let c2158: Fn0 = v2;
            c2158.apply(v3)
        }
        Adt14::C8(_) => {
            let c2159: Fn0 = v2;
            c2159.apply(v3)
        }
        Adt14::C9(_) => {
            let c2160: Fn0 = v2;
            c2160.apply(v3)
        }
        Adt14::C10(_) => {
            let c2161: Fn0 = v2;
            c2161.apply(v3)
        }
        Adt14::C11(_) => {
            let c2162: Fn0 = v2;
            c2162.apply(v3)
        }
    }
}

pub fn f574(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt120> {
    let m2163 = f177(v0.clone(), v1);
    match m2163 {
        Adt123::C0 => {
            Ok::<Option<Str>, Adt120>(None::<Str>)
        }
        Adt123::C1(v2) => {
            let m2164 = f233(v2);
            match m2164 {
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

pub fn f575(v0: Str, v1: Adt5) -> Result<Str, Adt120> {
    let m2165 = f177(v0.clone(), v1);
    match m2165 {
        Adt123::C0 => {
            Err::<Str, Adt120>(Adt120::C7(v0))
        }
        Adt123::C1(v2) => {
            let m2166 = f233(v2);
            match m2166 {
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

pub fn f576(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt120> {
    let m2167 = f177(v0.clone(), v1);
    match m2167 {
        Adt123::C0 => {
            Ok::<Option<Adt5>, Adt120>(None::<Adt5>)
        }
        Adt123::C1(v2) => {
            let m2168 = f329(v2);
            match m2168 {
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

pub fn f577(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt120> {
    let m2169 = f177(v0.clone(), v1);
    match m2169 {
        Adt123::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt120>(None::<List<(Str, Str)>>)
        }
        Adt123::C1(v2) => {
            let m2170 = f329(v2);
            match m2170 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt120>(Adt120::C8(v0))
                }
                Some(v3) => {
                    let m2172 = f626(v0, match v3 {
                        Adt5::C0(h2171) => {
                            h2171
                        }
                    });
                    match m2172 {
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

pub fn f578(v0: Str, v1: Adt5) -> Result<Adt5, Adt120> {
    let m2173 = f177(v0.clone(), v1);
    match m2173 {
        Adt123::C0 => {
            Err::<Adt5, Adt120>(Adt120::C7(v0))
        }
        Adt123::C1(v2) => {
            let m2174 = f329(v2);
            match m2174 {
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

pub fn f579(v0: Adt5) -> Result<Adt25, Adt120> {
    let m2175 = f575(Str::lit("type"), v0.clone());
    match m2175 {
        Ok(v1) => {
            let a2176 = v1.clone();
            let a2177 = Str::lit("ref/prompt");
            if equal(a2176, a2177) {
                let m2178 = f575(Str::lit("name"), v0.clone());
                match m2178 {
                    Ok(v2) => {
                        let m2179 = f574(Str::lit("title"), v0);
                        match m2179 {
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
                let a2180 = v1;
                let a2181 = Str::lit("ref/resource");
                if equal(a2180, a2181) {
                    let m2182 = f575(Str::lit("uri"), v0);
                    match m2182 {
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

pub fn f580(v0: Adt5) -> Result<Adt27, Adt120> {
    let m2183 = f575(Str::lit("name"), v0.clone());
    match m2183 {
        Ok(v1) => {
            let m2184 = f575(Str::lit("value"), v0);
            match m2184 {
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

pub fn f581(v0: Adt5) -> Result<Option<Adt28>, Adt120> {
    let m2185 = f177(Str::lit("context"), v0);
    match m2185 {
        Adt123::C0 => {
            Ok::<Option<Adt28>, Adt120>(None::<Adt28>)
        }
        Adt123::C1(v1) => {
            let m2186 = f329(v1);
            match m2186 {
                None => {
                    Err::<Option<Adt28>, Adt120>(Adt120::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2187 = f577(Str::lit("arguments"), v2);
                    match m2187 {
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

pub fn f582(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt119> {
    if v0 {
        None::<Adt119>
    } else {
        Some(Adt119::C1(v1, v2))
    }
}

pub fn f583(v0: Str, p2192: List<Str>) -> bool {
    let m2191 = p2192;
    {
        let mut m2191 = m2191;
        loop {
            let step = {
                let v1 = m2191;
                let m2188 = v1;
                match m2188.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2189 = v2;
                        let a2190 = v0.clone();
                        if equal(a2189, a2190) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
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

pub fn f584(v0: Option<Adt97>, v1: List<Adt99>, v2: Adt5) -> R<Option<Adt97>> {
    let m2193 = v0;
    match m2193 {
        None => {
            f627(v1, f628(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f585(v0: Option<Str>, v1: Adt138, v2: Adt5) -> Option<Adt97> {
    let m2194 = v1.clone();
    match m2194 {
        Adt138::C0 => {
            None::<Adt97>
        }
        Adt138::C1 => {
            f629(v0, f630(v1, v2))
        }
        Adt138::C2 => {
            f629(v0, f630(v1, v2))
        }
        Adt138::C3 => {
            f629(v0, f630(v1, v2))
        }
    }
}

pub fn f586(v0: Adt123, v1: Adt123) -> Adt145 {
    let m2195 = v0;
    match m2195 {
        Adt123::C0 => {
            Adt145::C1
        }
        Adt123::C1(_) => {
            let m2196 = v1;
            match m2196 {
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

pub fn f587(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt120> {
    let m2197 = f233(v1);
    match m2197 {
        None => {
            Err::<Adt16, Adt120>(Adt120::C5)
        }
        Some(v3) => {
            let m2198 = f631(v2);
            match m2198 {
                None => {
                    Err::<Adt16, Adt120>(Adt120::C6)
                }
                Some(v4) => {
                    let m2199 = f576(f632(), v0.clone());
                    match m2199 {
                        Ok(v6) => {
                            let m2200 = f633(v0.clone());
                            match m2200 {
                                Ok(v8) => {
                                    let m2201 = f634(v0.clone());
                                    match m2201 {
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

pub fn f588(v0: u64, v1: u64) -> u64 {
    let a2202 = v0;
    let a2203 = v1;
    let a2204 = 0u64;
    nat_rem(a2202, a2203, a2204)
}

pub fn f589(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2207 = {
            let a2205 = v0;
            let a2206 = 4u64;
            nat_mul(a2205, a2206)?
        };
        let a2208 = f591(v1, 16u64);
        nat_add(a2207, a2208)?
    }, List::<u64>::nil()))
}

pub fn f590(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2211 = {
            let a2209 = v0;
            let a2210 = 4u64;
            nat_mul(a2209, a2210)?
        };
        let a2212 = f591(v1, 16u64);
        nat_add(a2211, a2212)?
    }, List::cons({
        let a2215 = {
            let a2213 = f588(v1, 16u64);
            let a2214 = 16u64;
            nat_mul(a2213, a2214)?
        };
        let a2216 = f591(v2, 4u64);
        nat_add(a2215, a2216)?
    }, List::<u64>::nil())))
}

pub fn f591(v0: u64, v1: u64) -> u64 {
    let a2217 = v0;
    let a2218 = v1;
    let a2219 = 0u64;
    nat_quot(a2217, a2218, a2219)
}

pub fn f592(v0: u64, v1: Adt6) -> bool {
    let a2220 = v0;
    let a2221 = 0u64;
    if nat_eq(a2220, a2221) {
        f635(v1)
    } else {
        let a2222 = v0;
        let a2223 = 1u64;
        if nat_eq(a2222, a2223) {
            f636(v1)
        } else {
            let a2224 = v0;
            let a2225 = 2u64;
            if nat_eq(a2224, a2225) {
                f637(v1)
            } else {
                let a2226 = v0;
                let a2227 = 3u64;
                if nat_eq(a2226, a2227) {
                    f638(v1)
                } else {
                    let a2228 = v0;
                    let a2229 = 4u64;
                    if nat_eq(a2228, a2229) {
                        f639(v1)
                    } else {
                        let a2230 = v0;
                        let a2231 = 5u64;
                        if nat_eq(a2230, a2231) {
                            f640(v1)
                        } else {
                            let a2232 = v0;
                            let a2233 = 6u64;
                            if nat_eq(a2232, a2233) {
                                f641(v1)
                            } else {
                                let a2234 = v0;
                                let a2235 = 7u64;
                                if nat_eq(a2234, a2235) {
                                    f642(v1)
                                } else {
                                    f643(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f593(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt132) -> R<Adt144> {
    let m2236 = v3.clone();
    match m2236.uncons() {
        None => {
            Ok(Adt144::C0(Adt94::C8(v0)))
        }
        Some((_, _)) => {
            if f322(v3.clone(), true) {
                let m2237 = f644(v1);
                match m2237 {
                    None => {
                        Ok(Adt144::C0(Adt94::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f645(v3)?;
                        if f646(v9.clone(), match v5.clone() {
                            Adt132::C0(_, h2238) => {
                                h2238
                            }
                        }) {
                            Ok(Adt144::C0(Adt94::C12(v0, v2)))
                        } else {
                            Ok(Adt144::C1(Adt132::C0(List::cons(Adt12::C0(v0, v2, f207(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt132::C0(h2239, _) => {
                                    h2239
                                }
                            }), List::cons(v9, match v5 {
                                Adt132::C0(_, h2240) => {
                                    h2240
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

pub fn f594(p2245: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2246: List<Adt125>) -> List<Adt125> {
    let m2244 = (p2245, p2246);
    {
        let mut m2244 = m2244;
        loop {
            let step = {
                let (v0, v3) = m2244;
                let m2241 = v0;
                match m2241.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v5, List::cons(Adt125::C0({
                            let (_, h2242) = v4;
                            h2242
                        }, v1, List::cons({
                            let (h2243, _) = v4;
                            h2243
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(p2250: List<(Str, Adt6)>, p2251: List<Adt125>) -> List<Adt125> {
    let m2249 = (p2250, p2251);
    {
        let mut m2249 = m2249;
        loop {
            let step = {
                let (v0, v1) = m2249;
                let m2247 = v0;
                match m2247.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v3, List::cons(Adt125::C0({
                            let (_, h2248) = v2;
                            h2248
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(p2254: List<Adt6>, p2255: List<Adt125>) -> List<Adt125> {
    let m2253 = (p2254, p2255);
    {
        let mut m2253 = m2253;
        loop {
            let step = {
                let (v0, v1) = m2253;
                let m2252 = v0;
                match m2252.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt125>), List<Adt125>>((v3, List::cons(Adt125::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(v0: Adt6) -> Str {
    let m2256 = v0;
    match m2256 {
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

pub fn f598(v0: Adt7) -> R<bool> {
    if f397(match v0.clone() {
        Adt7::C0(h2257, _) => {
            h2257
        }
    }) {
        Ok(true)
    } else {
        f647(40u64, match v0.clone() {
            Adt7::C0(h2258, _) => {
                h2258
            }
        }, match v0 {
            Adt7::C0(_, h2259) => {
                h2259
            }
        })
    }
}

pub fn f599(v0: Adt6) -> Str {
    let m2260 = f233(v0);
    match m2260 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f600(p2267: u64, p2268: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m2266 = (p2267, p2268);
    {
        let mut m2266 = m2266;
        loop {
            let step = {
                let (v0, v1) = m2266;
                let m2261 = v0;
                if m2261 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m2261 - 1;
                    let m2262 = v1;
                    match m2262.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m2265 = f648({
                                let (h2263, _) = v3;
                                h2263
                            }, {
                                let (_, h2264) = v3;
                                h2264
                            }, v4)?;
                            match m2265 {
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
                Ok(next) => { m2266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f601() -> u64 {
    67108864u64
}

pub fn f602(v0: Adt147) -> bool {
    let m2269 = v0;
    match m2269 {
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

pub fn f603(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m2270 = v2;
    match m2270 {
        Adt147::C0 => {
            let m2272 = v3;
            match m2272 {
                Adt147::C0 => {
                    Ok(f649(Adt147::C0, f650(f651(match v0.clone() {
                        Adt7::C0(h2273, _) => {
                            h2273
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2274) => {
                            h2274
                        }
                    }, f651(match v1.clone() {
                        Adt7::C0(h2275, _) => {
                            h2275
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2276) => {
                            h2276
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
            let m2271 = v3;
            match m2271 {
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
            let m2277 = v3;
            match m2277 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f650(match v0.clone() {
                        Adt7::C0(h2278, _) => {
                            h2278
                        }
                    }, match v0 {
                        Adt7::C0(_, h2279) => {
                            h2279
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2280, _) => {
                            h2280
                        }
                    }, match v1 {
                        Adt7::C0(_, h2281) => {
                            h2281
                        }
                    })
                }
            }
        }
    }
}

pub fn f604(v0: i64) -> Adt147 {
    f606(v0, 0i64)
}

pub fn f605(v0: Adt6) -> Option<i64> {
    let m2282 = v0;
    match m2282 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f397(match v2.clone() {
                Adt7::C0(_, h2283) => {
                    h2283
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2284, _) => {
                        h2284
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

pub fn f606(v0: i64, v1: i64) -> Adt147 {
    let m2287 = {
        let a2285 = v0;
        let a2286 = v1;
        compare(a2285, a2286)
    };
    if match m2287 {
        Ordering::Less => {
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
        let m2290 = {
            let a2288 = v1;
            let a2289 = v0;
            compare(a2288, a2289)
        };
        if match m2290 {
            Ordering::Less => {
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

pub fn f607(p2299: List<u64>, p2300: u64) -> R<u64> {
    let m2298 = (p2299, p2300);
    {
        let mut m2298 = m2298;
        loop {
            let step = {
                let (v0, v1) = m2298;
                let m2291 = v0;
                match m2291.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2292 = v2;
                        let a2293 = 128u64;
                        if if nat_lt(a2292, a2293) {
                            true
                        } else {
                            let a2294 = 191u64;
                            let a2295 = v2;
                            nat_lt(a2294, a2295)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2296 = v1;
                                let a2297 = 1u64;
                                nat_add(a2296, a2297)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
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

pub fn f608(v0: Str, p2318: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2317 = p2318;
    {
        let mut m2317 = m2317;
        loop {
            let step = {
                let v1 = m2317;
                let m2301 = v1;
                match m2301.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2305 = v0.clone();
                        let a2306 = {
                            let a2303 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2302, _) = v2.clone();
                                h2302
                            }, List::<Str>::nil()));
                            let a2304 = Str::lit("");
                            join(a2303, a2304)
                        };
                        if equal(a2305, a2306) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2307) = v2;
                                h2307
                            }))
                        } else {
                            let m2309 = f329({
                                let (_, h2308) = v2.clone();
                                h2308
                            });
                            match m2309 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2310 = f177(Str::lit("$anchor"), v4);
                                    match m2310 {
                                        Adt123::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt123::C1(v5) => {
                                            let m2311 = f233(v5);
                                            match m2311 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2314 = v0.clone();
                                                    let a2315 = {
                                                        let a2312 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2313 = Str::lit("");
                                                        join(a2312, a2313)
                                                    };
                                                    if equal(a2314, a2315) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2316) = v2;
                                                            h2316
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
                Ok(next) => { m2317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f609(v0: i64) -> u64 {
    let m2321 = {
        let a2319 = v0;
        let a2320 = 1i64;
        compare(a2319, a2320)
    };
    if match m2321 {
        Ordering::Less => {
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
        let m2324 = {
            let a2322 = v0;
            let a2323 = 2i64;
            compare(a2322, a2323)
        };
        if match m2324 {
            Ordering::Less => {
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
            let m2327 = {
                let a2325 = v0;
                let a2326 = 3i64;
                compare(a2325, a2326)
            };
            if match m2327 {
                Ordering::Less => {
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
                let m2330 = {
                    let a2328 = v0;
                    let a2329 = 4i64;
                    compare(a2328, a2329)
                };
                if match m2330 {
                    Ordering::Less => {
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
                    let m2333 = {
                        let a2331 = v0;
                        let a2332 = 5i64;
                        compare(a2331, a2332)
                    };
                    if match m2333 {
                        Ordering::Less => {
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
                        let m2336 = {
                            let a2334 = v0;
                            let a2335 = 6i64;
                            compare(a2334, a2335)
                        };
                        if match m2336 {
                            Ordering::Less => {
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
                            let m2339 = {
                                let a2337 = v0;
                                let a2338 = 7i64;
                                compare(a2337, a2338)
                            };
                            if match m2339 {
                                Ordering::Less => {
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
                                let m2342 = {
                                    let a2340 = v0;
                                    let a2341 = 8i64;
                                    compare(a2340, a2341)
                                };
                                if match m2342 {
                                    Ordering::Less => {
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
                                    let m2345 = {
                                        let a2343 = v0;
                                        let a2344 = 9i64;
                                        compare(a2343, a2344)
                                    };
                                    if match m2345 {
                                        Ordering::Less => {
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

pub fn f610(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2346 = v0;
    match m2346 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2358 = f421(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2347, _, _, _, _) => {
                        h2347
                    }
                });
                let a2359 = {
                    let a2356 = f426(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2348, _, _, _) => {
                            h2348
                        }
                    });
                    let a2357 = {
                        let a2354 = f426(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2349, _, _) => {
                                h2349
                            }
                        });
                        let a2355 = {
                            let a2352 = f426(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2350, _) => {
                                    h2350
                                }
                            });
                            let a2353 = f426(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2351) => {
                                    h2351
                                }
                            });
                            append_list(a2352, a2353)
                        };
                        append_list(a2354, a2355)
                    };
                    append_list(a2356, a2357)
                };
                append_list(a2358, a2359)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f611(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f444(Str::lit("_meta"), v0)
}

pub fn f612(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2360, _, _) => {
            h2360
        }
    })), {
        let a2363 = f616(match v0.clone() {
            Adt39::C0(_, h2361, _) => {
                h2361
            }
        });
        let a2364 = f611(match v0 {
            Adt39::C0(_, _, h2362) => {
                h2362
            }
        });
        append_list(a2363, a2364)
    })))
}

pub fn f613(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2365 = f616(v3);
        let a2366 = f611(v4);
        append_list(a2365, a2366)
    }))))
}

pub fn f614(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2390 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2367, _, _, _, _, _, _, _, _) => {
                h2367
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2391 = {
            let a2388 = f421(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2368, _, _, _, _, _, _, _) => {
                    h2368
                }
            });
            let a2389 = {
                let a2386 = f499(match v0.clone() {
                    Adt44::C0(_, _, h2369, _, _, _, _, _, _) => {
                        h2369
                    }
                });
                let a2387 = {
                    let a2384 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2370, _, _, _, _, _) => {
                            h2370
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2385 = {
                        let a2382 = f421(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2371, _, _, _, _) => {
                                h2371
                            }
                        });
                        let a2383 = {
                            let a2380 = f421(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2372, _, _, _) => {
                                    h2372
                                }
                            });
                            let a2381 = {
                                let a2378 = f616(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2373, _, _) => {
                                        h2373
                                    }
                                });
                                let a2379 = {
                                    let a2376 = f617(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2374, _) => {
                                            h2374
                                        }
                                    })?;
                                    let a2377 = f611(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2375) => {
                                            h2375
                                        }
                                    });
                                    append_list(a2376, a2377)
                                };
                                append_list(a2378, a2379)
                            };
                            append_list(a2380, a2381)
                        };
                        append_list(a2382, a2383)
                    };
                    append_list(a2384, a2385)
                };
                append_list(a2386, a2387)
            };
            append_list(a2388, a2389)
        };
        append_list(a2390, a2391)
    }))
}

pub fn f615(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f562(match v0.clone() {
        Adt45::C0(h2392, _, _) => {
            h2392
        }
    })), {
        let a2395 = f616(match v0.clone() {
            Adt45::C0(_, h2393, _) => {
                h2393
            }
        });
        let a2396 = f611(match v0 {
            Adt45::C0(_, _, h2394) => {
                h2394
            }
        });
        append_list(a2395, a2396)
    })))
}

pub fn f616(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2397 = v0;
    match m2397 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2403 = f652(match v1.clone() {
                    Adt40::C0(h2398, _, _) => {
                        h2398
                    }
                });
                let a2404 = {
                    let a2401 = f653(match v1.clone() {
                        Adt40::C0(_, h2399, _) => {
                            h2399
                        }
                    });
                    let a2402 = f421(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2400) => {
                            h2400
                        }
                    });
                    append_list(a2401, a2402)
                };
                append_list(a2403, a2404)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f617(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2405 = v0;
    match m2405 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f77(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f618(v0: List<Adt57>) -> List<Adt6> {
    f304(f654(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f619(v0: Adt41) -> Str {
    let m2406 = v0;
    match m2406 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f620(p2409: List<Adt2>, p2410: List<Adt6>) -> List<Adt6> {
    let m2408 = (p2409, p2410);
    {
        let mut m2408 = m2408;
        loop {
            let step = {
                let (v0, v1) = m2408;
                let m2407 = v0;
                match m2407.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f655(v2), v1)))
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

pub fn f621(p2415: List<u64>, p2416: u64, p2417: List<u64>) -> List<u64> {
    let m2414 = (p2415, (p2416, p2417));
    {
        let mut m2414 = m2414;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2414;
                let m2411 = v0;
                match m2411.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f656(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt148 = f657(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt148::C0(h2412, _) => {
                                h2412
                            }
                        }, match v5 {
                            Adt148::C0(_, h2413) => {
                                h2413
                            }
                        })))
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

pub fn f622(v0: Adt128, v1: Adt32) -> bool {
    let m2418 = v1;
    match m2418 {
        Adt32::C0(_) => {
            f658(v0, Adt128::C0)
        }
        Adt32::C1(_) => {
            f658(v0, Adt128::C1)
        }
        Adt32::C2(_) => {
            f658(v0, Adt128::C2)
        }
        Adt32::C3(_) => {
            f658(v0, Adt128::C2)
        }
        Adt32::C4(_) => {
            f658(v0, Adt128::C3)
        }
        Adt32::C5(_) => {
            f658(v0, Adt128::C4)
        }
        Adt32::C6(_) => {
            f658(v0, Adt128::C5)
        }
        Adt32::C7(_) => {
            f658(v0, Adt128::C6)
        }
        Adt32::C8(_) => {
            f658(v0, Adt128::C7)
        }
        Adt32::C9(_) => {
            f658(v0, Adt128::C8)
        }
        Adt32::C10(_) => {
            f658(v0, Adt128::C9)
        }
        Adt32::C11 => {
            if f658(v0.clone(), Adt128::C10) {
                true
            } else {
                f658(v0, Adt128::C11)
            }
        }
        Adt32::C12(_) => {
            f659(v0)
        }
        Adt32::C13(_) => {
            f660(v0)
        }
    }
}

pub fn f623(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt116, Adt119> {
    let m2419 = v2.clone();
    match m2419 {
        Adt32::C0(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt116, Adt119>(f664(v0, f662(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt116, Adt119>(f570(v0, v2))
        }
        Adt32::C12(v3) => {
            f661(v0, f662(v1), v3)
        }
        Adt32::C13(v4) => {
            f663(v0, f662(v1), v4)
        }
    }
}

pub fn f624() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f625(v0: u64, v1: u64) -> bool {
    let a2420 = v1;
    let a2421 = v0;
    nat_lt(a2420, a2421)
}

pub fn f626(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt120> {
    let m2422 = f665(v0, v1, List::<(Str, Str)>::nil());
    match m2422 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt120>(f666(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt120>(v3)
        }
    }
}

pub fn f627(p2426: List<Adt99>, v1: Adt5) -> R<Option<Adt97>> {
    let m2425 = p2426;
    {
        let mut m2425 = m2425;
        loop {
            let step = {
                let v0 = m2425;
                let m2423 = v0;
                match m2423.uncons() {
                    None => {
                        Err::<List<Adt99>, R<Option<Adt97>>>(Ok(None::<Adt97>))
                    }
                    Some((v2, v3)) => {
                        let m2424 = f667(v2, v1.clone())?;
                        match m2424 {
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
                Ok(next) => { m2425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f628(v0: Adt5) -> Adt5 {
    let m2427 = f177(Str::lit("arguments"), v0);
    match m2427 {
        Adt123::C0 => {
            f28()
        }
        Adt123::C1(v1) => {
            let m2428 = f329(v1);
            match m2428 {
                None => {
                    f28()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f629(v0: Option<Str>, v1: Option<Str>) -> Option<Adt97> {
    let m2429 = v0;
    match m2429 {
        None => {
            Some(Adt97::C2)
        }
        Some(v2) => {
            let m2430 = v1;
            match m2430 {
                None => {
                    None::<Adt97>
                }
                Some(v3) => {
                    let a2431 = v2;
                    let a2432 = v3;
                    if equal(a2431, a2432) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C5)
                    }
                }
            }
        }
    }
}

pub fn f630(v0: Adt138, v1: Adt5) -> Option<Str> {
    let m2433 = v0;
    match m2433 {
        Adt138::C0 => {
            None::<Str>
        }
        Adt138::C1 => {
            let m2434 = f177(Str::lit("name"), v1);
            match m2434 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v2) => {
                    f233(v2)
                }
            }
        }
        Adt138::C2 => {
            let m2435 = f177(Str::lit("uri"), v1);
            match m2435 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v3) => {
                    f233(v3)
                }
            }
        }
        Adt138::C3 => {
            let m2436 = f177(Str::lit("taskId"), v1);
            match m2436 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v4) => {
                    f233(v4)
                }
            }
        }
    }
}

pub fn f631(v0: Adt6) -> Option<Adt17> {
    let m2437 = f329(v0);
    match m2437 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2438 = f668(Str::lit("experimental"), v1.clone());
            match m2438 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2439 = f668(Str::lit("roots"), v1.clone());
                    match m2439 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2440 = f668(Str::lit("sampling"), v1.clone());
                            match m2440 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2441 = f668(Str::lit("elicitation"), v1.clone());
                                    match m2441 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2442 = f668(Str::lit("extensions"), v1);
                                            match m2442 {
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

pub fn f632() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f633(v0: Adt5) -> Result<Option<Adt18>, Adt120> {
    let m2443 = f574(f669(), v0);
    match m2443 {
        Ok(v2) => {
            let m2444 = v2;
            match m2444 {
                None => {
                    Ok::<Option<Adt18>, Adt120>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2445 = f670(v3);
                    match m2445 {
                        Adt150::C0 => {
                            Err::<Option<Adt18>, Adt120>(Adt120::C8(f669()))
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

pub fn f634(v0: Adt5) -> Result<Option<Adt19>, Adt120> {
    let m2446 = f177(f671(), v0);
    match m2446 {
        Adt123::C0 => {
            Ok::<Option<Adt19>, Adt120>(None::<Adt19>)
        }
        Adt123::C1(v1) => {
            let m2447 = f233(v1.clone());
            match m2447 {
                None => {
                    let m2448 = f605(v1);
                    match m2448 {
                        None => {
                            Err::<Option<Adt19>, Adt120>(Adt120::C8(f671()))
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

pub fn f635(v0: Adt6) -> bool {
    let m2449 = f233(v0);
    match m2449 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f636(v0: Adt6) -> bool {
    let m2450 = f540(v0);
    match m2450 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f637(v0: Adt6) -> bool {
    let m2451 = f605(v0);
    match m2451 {
        None => {
            false
        }
        Some(v1) => {
            let a2455 = {
                let m2454 = {
                    let a2452 = v1;
                    let a2453 = 0i64;
                    compare(a2452, a2453)
                };
                match m2454 {
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
            bool_not(a2455)
        }
    }
}

pub fn f638(v0: Adt6) -> bool {
    let m2456 = f408(v0);
    match m2456 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f639(v0: Adt6) -> bool {
    let m2457 = f329(v0);
    match m2457 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f640(v0: Adt6) -> bool {
    let m2458 = f408(v0);
    match m2458 {
        None => {
            false
        }
        Some(v1) => {
            f672(v1)
        }
    }
}

pub fn f641(v0: Adt6) -> bool {
    let m2459 = f328(v0);
    match m2459 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f642(v0: Adt6) -> bool {
    let m2460 = f233(v0.clone());
    match m2460 {
        None => {
            let m2461 = f408(v0);
            match m2461 {
                None => {
                    false
                }
                Some(v2) => {
                    f673(v2)
                }
            }
        }
        Some(v1) => {
            f535(v1)
        }
    }
}

pub fn f643(v0: Adt6) -> bool {
    let m2462 = f408(v0);
    match m2462 {
        None => {
            false
        }
        Some(v1) => {
            let m2463 = v1;
            match m2463.uncons() {
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

pub fn f644(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2464 = f236(Str::lit("type"), v0);
    match m2464 {
        Adt123::C0 => {
            None::<Adt13>
        }
        Adt123::C1(v1) => {
            let m2465 = f233(v1);
            match m2465 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f674(v2)
                }
            }
        }
    }
}

pub fn f645(v0: List<u64>) -> R<List<u64>> {
    Ok(f211(f675(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f646(v0: List<u64>, p2468: List<List<u64>>) -> bool {
    let m2467 = p2468;
    {
        let mut m2467 = m2467;
        loop {
            let step = {
                let v1 = m2467;
                let m2466 = v1;
                match m2466.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f676(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
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

pub fn f647(p2480: u64, p2481: i64, p2482: i64) -> R<bool> {
    let m2479 = (p2480, (p2481, p2482));
    {
        let mut m2479 = m2479;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2479;
                let m2469 = v0;
                if m2469 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2469 - 1;
                    let m2472 = {
                        let a2470 = v2;
                        let a2471 = 0i64;
                        compare(a2470, a2471)
                    };
                    if match m2472 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f397({
                            let a2473 = v1;
                            let a2474 = 10i64;
                            let a2475 = 0i64;
                            int_rem(a2473, a2474, a2475)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2476 = v1;
                                let a2477 = 10i64;
                                let a2478 = 0i64;
                                int_quot(a2476, a2477, a2478)?
                            }, f266(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2483 = f677(v0.clone());
    let a2484 = f677(v1.clone());
    if nat_eq(a2483, a2484) {
        let m2485 = v0;
        match m2485 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2486 = f328(v1);
                match m2486 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2487 = v3;
                        let a2488 = v4;
                        if equal(a2487, a2488) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2489 = f540(v1);
                match m2489 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f678(f542(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2490 = f233(v1);
                match m2490 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2491 = v7;
                        let a2492 = v8;
                        if equal(a2491, a2492) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2493 = f408(v1);
                match m2493 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f679(f680(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2494 = f329(v1);
                match m2494 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2496 = f546(v11.clone(), 0u64)?;
                        let a2497 = f546(match v12.clone() {
                            Adt5::C0(h2495) => {
                                h2495
                            }
                        }, 0u64)?;
                        if nat_eq(a2496, a2497) {
                            Ok(f679(f681(v11, match v12 {
                                Adt5::C0(h2498) => {
                                    h2498
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

pub fn f649(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2499 = v0;
    match m2499 {
        Adt147::C0 => {
            f682(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f650(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2500 = f606(f683(v0, v1)?, f683(v2, v3)?);
    match m2500 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f684(v0, f685(v0)?, v2, f685(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f651(v0: i64) -> R<i64> {
    let m2503 = {
        let a2501 = v0;
        let a2502 = 0i64;
        compare(a2501, a2502)
    };
    if match m2503 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2504 = v0;
        int_neg(a2504)
    } else {
        Ok(v0)
    }
}

pub fn f652(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2505 = v0;
    match m2505 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f686(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f653(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2506 = v0;
    match m2506 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f654(p2509: List<Adt57>, p2510: List<Adt6>) -> List<Adt6> {
    let m2508 = (p2509, p2510);
    {
        let mut m2508 = m2508;
        loop {
            let step = {
                let (v0, v1) = m2508;
                let m2507 = v0;
                match m2507.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f687(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f655(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2511, _, _, _) => {
            h2511
        }
    })), {
        let a2517 = f421(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2512, _, _) => {
                h2512
            }
        });
        let a2518 = {
            let a2515 = f688(match v0.clone() {
                Adt2::C0(_, _, h2513, _) => {
                    h2513
                }
            });
            let a2516 = f689(match v0 {
                Adt2::C0(_, _, _, h2514) => {
                    h2514
                }
            });
            append_list(a2515, a2516)
        };
        append_list(a2517, a2518)
    }))
}

pub fn f656(v0: u64, v1: List<u64>) -> List<u64> {
    let a2519 = v0;
    let a2520 = 0u64;
    if nat_eq(a2519, a2520) {
        v1
    } else {
        let a2521 = v0;
        let a2522 = 1u64;
        if nat_eq(a2521, a2522) {
            List::cons(194u64, v1)
        } else {
            let a2523 = v0;
            let a2524 = 2u64;
            if nat_eq(a2523, a2524) {
                List::cons(226u64, v1)
            } else {
                let a2525 = v0;
                let a2526 = 3u64;
                if nat_eq(a2525, a2526) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f657(v0: u64, v1: u64, v2: List<u64>) -> Adt148 {
    let a2527 = v0;
    let a2528 = 0u64;
    if nat_eq(a2527, a2528) {
        f690(v1, v2)
    } else {
        let a2529 = v0;
        let a2530 = 1u64;
        if nat_eq(a2529, a2530) {
            let a2531 = 128u64;
            let a2532 = v1;
            if if nat_le(a2531, a2532) {
                let a2533 = v1;
                let a2534 = 159u64;
                nat_le(a2533, a2534)
            } else {
                false
            } {
                Adt148::C0(0u64, f691(v2))
            } else {
                f690(v1, List::cons(194u64, v2))
            }
        } else {
            let a2535 = v0;
            let a2536 = 2u64;
            if nat_eq(a2535, a2536) {
                let a2537 = v1;
                let a2538 = 128u64;
                if nat_eq(a2537, a2538) {
                    Adt148::C0(3u64, v2)
                } else {
                    let a2539 = v1;
                    let a2540 = 129u64;
                    if nat_eq(a2539, a2540) {
                        Adt148::C0(4u64, v2)
                    } else {
                        f690(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2541 = v0;
                let a2542 = 3u64;
                if nat_eq(a2541, a2542) {
                    let a2543 = 170u64;
                    let a2544 = v1;
                    if if nat_le(a2543, a2544) {
                        let a2545 = v1;
                        let a2546 = 174u64;
                        nat_le(a2545, a2546)
                    } else {
                        false
                    } {
                        Adt148::C0(0u64, f691(v2))
                    } else {
                        f690(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2547 = 166u64;
                    let a2548 = v1;
                    if if nat_le(a2547, a2548) {
                        let a2549 = v1;
                        let a2550 = 169u64;
                        nat_le(a2549, a2550)
                    } else {
                        false
                    } {
                        Adt148::C0(0u64, f691(v2))
                    } else {
                        f690(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f658(v0: Adt128, v1: Adt128) -> bool {
    let a2551 = f447(v0);
    let a2552 = f447(v1);
    equal(a2551, a2552)
}

pub fn f659(v0: Adt128) -> bool {
    let m2553 = v0;
    match m2553 {
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

pub fn f660(v0: Adt128) -> bool {
    let m2554 = v0;
    match m2554 {
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

pub fn f661(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt116, Adt119> {
    f692(v0, v2.clone(), f693(match v1 {
        Adt16::C0(_, h2555, _, _, _, _) => {
            h2555
        }
    }, f694(v2)))
}

pub fn f662(v0: Adt14) -> Adt16 {
    let m2556 = v0;
    match m2556 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2557) => {
                    h2557
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2558, _) => {
                    h2558
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2559, _, _, _, _) => {
                    h2559
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2560, _) => {
                    h2560
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2561, _, _, _) => {
                    h2561
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2562, _) => {
                    h2562
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2563, _) => {
                    h2563
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2564, _, _, _, _) => {
                    h2564
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2565, _, _, _) => {
                    h2565
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2566, _) => {
                    h2566
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2567, _, _) => {
                    h2567
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2568, _) => {
                    h2568
                }
            }
        }
    }
}

pub fn f663(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt116, Adt119> {
    f695(v0, v2, f696(match v1 {
        Adt16::C0(_, h2569, _, _, _, _) => {
            h2569
        }
    }))
}

pub fn f664(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt116 {
    Adt116::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2570, _) => {
            h2570
        }
    }), {
        let m2572 = f570(v0, Adt32::C11);
        match m2572 {
            Adt116::C0(_, h2571, _) => {
                h2571
            }
        }
    }, f697(v1, match v2 {
        Adt49::C0(_, h2573) => {
            h2573
        }
    }))
}

pub fn f665(v0: Str, p2579: List<(Str, Adt6)>, p2580: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt120> {
    let m2578 = (p2579, p2580);
    {
        let mut m2578 = m2578;
        loop {
            let step = {
                let (v1, v2) = m2578;
                let m2574 = v1;
                match m2574.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>(Ok::<List<(Str, Str)>, Adt120>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2576 = f233({
                            let (_, h2575) = v3;
                            h2575
                        });
                        match m2576 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>(Err::<List<(Str, Str)>, Adt120>(Adt120::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>((v4, List::cons(({
                                    let (h2577, _) = v3;
                                    h2577
                                }, v5), v2)))
                            }
                        }
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

pub fn f666(p2583: List<(Str, Str)>, p2584: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2582 = (p2583, p2584);
    {
        let mut m2582 = m2582;
        loop {
            let step = {
                let (v0, v1) = m2582;
                let m2581 = v0;
                match m2581.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
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

pub fn f667(v0: Adt99, v1: Adt5) -> R<Option<Adt97>> {
    let m2586 = f698(match v0.clone() {
        Adt99::C0(_, h2585, _, _) => {
            h2585
        }
    }, v1);
    match m2586 {
        Adt123::C0 => {
            Ok(f699(v0))
        }
        Adt123::C1(v2) => {
            if f700(v2.clone()) {
                Ok(f699(v0))
            } else {
                f701(v0, v2)
            }
        }
    }
}

pub fn f668(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2587 = f177(v0, v1);
    match m2587 {
        Adt123::C0 => {
            Some(None::<Adt5>)
        }
        Adt123::C1(v2) => {
            let m2588 = f329(v2);
            match m2588 {
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

pub fn f669() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f670(v0: Str) -> Adt150 {
    let a2589 = v0.clone();
    let a2590 = Str::lit("debug");
    if equal(a2589, a2590) {
        Adt150::C1(Adt18::C0)
    } else {
        let a2591 = v0.clone();
        let a2592 = Str::lit("info");
        if equal(a2591, a2592) {
            Adt150::C1(Adt18::C1)
        } else {
            let a2593 = v0.clone();
            let a2594 = Str::lit("notice");
            if equal(a2593, a2594) {
                Adt150::C1(Adt18::C2)
            } else {
                let a2595 = v0.clone();
                let a2596 = Str::lit("warning");
                if equal(a2595, a2596) {
                    Adt150::C1(Adt18::C3)
                } else {
                    let a2597 = v0.clone();
                    let a2598 = Str::lit("error");
                    if equal(a2597, a2598) {
                        Adt150::C1(Adt18::C4)
                    } else {
                        let a2599 = v0.clone();
                        let a2600 = Str::lit("critical");
                        if equal(a2599, a2600) {
                            Adt150::C1(Adt18::C5)
                        } else {
                            let a2601 = v0.clone();
                            let a2602 = Str::lit("alert");
                            if equal(a2601, a2602) {
                                Adt150::C1(Adt18::C6)
                            } else {
                                let a2603 = v0;
                                let a2604 = Str::lit("emergency");
                                if equal(a2603, a2604) {
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

pub fn f671() -> Str {
    Str::lit("progressToken")
}

pub fn f672(p2608: List<Adt6>) -> bool {
    let m2607 = p2608;
    {
        let mut m2607 = m2607;
        loop {
            let step = {
                let v0 = m2607;
                let m2605 = v0;
                match m2605.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2606 = f233(v1);
                        match m2606 {
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
                Ok(next) => { m2607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f673(p2612: List<Adt6>) -> bool {
    let m2611 = p2612;
    {
        let mut m2611 = m2611;
        loop {
            let step = {
                let v0 = m2611;
                let m2609 = v0;
                match m2609.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2610 = f233(v1);
                        match m2610 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f535(v3) {
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
                Ok(next) => { m2611 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f674(v0: Str) -> Option<Adt13> {
    let a2613 = v0.clone();
    let a2614 = Str::lit("string");
    if equal(a2613, a2614) {
        Some(Adt13::C0)
    } else {
        let a2615 = v0.clone();
        let a2616 = Str::lit("integer");
        if equal(a2615, a2616) {
            Some(Adt13::C1)
        } else {
            let a2617 = v0;
            let a2618 = Str::lit("boolean");
            if equal(a2617, a2618) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f675(p2627: List<u64>, p2628: List<u64>) -> R<List<u64>> {
    let m2626 = (p2627, p2628);
    {
        let mut m2626 = m2626;
        loop {
            let step = {
                let (v0, v1) = m2626;
                let m2619 = v0;
                match m2619.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2620 = 65u64;
                        let a2621 = v2;
                        if if nat_le(a2620, a2621) {
                            let a2622 = v2;
                            let a2623 = 90u64;
                            nat_le(a2622, a2623)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2624 = v2;
                                let a2625 = 32u64;
                                nat_add(a2624, a2625)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f676(p2635: List<u64>, p2636: List<u64>) -> bool {
    let m2634 = (p2635, p2636);
    {
        let mut m2634 = m2634;
        loop {
            let step = {
                let (v0, v1) = m2634;
                let m2629 = v0;
                match m2629.uncons() {
                    None => {
                        let m2630 = v1;
                        match m2630.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2631 = v1;
                        match m2631.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2632 = v4;
                                let a2633 = v6;
                                if nat_eq(a2632, a2633) {
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
                Ok(next) => { m2634 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677(v0: Adt6) -> u64 {
    let m2637 = v0;
    match m2637 {
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

pub fn f678(v0: Adt147) -> bool {
    let m2638 = v0;
    match m2638 {
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

pub fn f679(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2639 = v0;
    match m2639 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2640 = v2;
                let a2641 = v1;
                append_list(a2640, a2641)
            })
        }
    }
}

pub fn f680(p2646: List<Adt6>, p2647: List<Adt6>, p2648: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2645 = (p2646, (p2647, p2648));
    {
        let mut m2645 = m2645;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2645;
                let m2642 = v0;
                match m2642.uncons() {
                    None => {
                        let m2643 = v1;
                        match m2643.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2644 = v1;
                        match m2644.uncons() {
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
                Ok(next) => { m2645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f681(p2654: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2655: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2653 = (p2654, p2655);
    {
        let mut m2653 = m2653;
        loop {
            let step = {
                let (v0, v2) = m2653;
                let m2649 = v0;
                match m2649.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2651 = f236({
                            let (h2650, _) = v3;
                            h2650
                        }, v1.clone());
                        match m2651 {
                            Adt123::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt123::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2652) = v3;
                                    h2652
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2653 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f682(v0: Adt147) -> Adt147 {
    let m2656 = v0;
    match m2656 {
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

pub fn f683(v0: i64, v1: i64) -> R<i64> {
    f266(f194(f685(v0)?, 0i64)?, v1)
}

pub fn f684(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2657 = v1;
    let a2658 = v3;
    if nat_lt(a2657, a2658) {
        Ok(f606(f702({
            let a2659 = v3;
            let a2660 = v1;
            nat_sub(a2659, a2660)
        }, v0)?, v2))
    } else {
        let a2661 = v3;
        let a2662 = v1;
        if nat_lt(a2661, a2662) {
            Ok(f606(v0, f702({
                let a2663 = v1;
                let a2664 = v3;
                nat_sub(a2663, a2664)
            }, v2)?))
        } else {
            Ok(f606(v0, v2))
        }
    }
}

pub fn f685(v0: i64) -> R<u64> {
    f703(20u64, v0, 0u64)
}

pub fn f686(v0: List<Adt41>) -> List<Adt6> {
    f304(f704(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f687(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2673 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2665, _, _, _) => {
                h2665
            }
        })), List::<(Str, Adt6)>::nil());
        let a2674 = {
            let a2671 = f421(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2666, _, _) => {
                    h2666
                }
            });
            let a2672 = {
                let a2669 = f421(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h2667, _) => {
                        h2667
                    }
                });
                let a2670 = f426(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h2668) => {
                        h2668
                    }
                });
                append_list(a2669, a2670)
            };
            append_list(a2671, a2672)
        };
        append_list(a2673, a2674)
    })
}

pub fn f688(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2675 = v0;
    match m2675 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f378(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f689(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2676 = v0;
    match m2676 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f705(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f690(v0: u64, v1: List<u64>) -> Adt148 {
    let a2677 = v0;
    let a2678 = 32u64;
    if nat_lt(a2677, a2678) {
        if f706(v0) {
            Adt148::C0(0u64, List::cons(v0, v1))
        } else {
            Adt148::C0(0u64, f691(v1))
        }
    } else {
        let a2679 = v0;
        let a2680 = 127u64;
        if nat_eq(a2679, a2680) {
            Adt148::C0(0u64, f691(v1))
        } else {
            let a2681 = v0;
            let a2682 = 194u64;
            if nat_eq(a2681, a2682) {
                Adt148::C0(1u64, v1)
            } else {
                let a2683 = v0;
                let a2684 = 226u64;
                if nat_eq(a2683, a2684) {
                    Adt148::C0(2u64, v1)
                } else {
                    Adt148::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f691(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f692(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt116, Adt119> {
    if v2 {
        Ok::<Adt116, Adt119>(f570(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt116, Adt119>(Adt119::C4(Adt84::C8(f707(f694(v1)))))
    }
}

pub fn f693(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2685 = f708(v1.clone());
    if if if bool_not(a2685) {
        true
    } else {
        f450(match v0.clone() {
            Adt17::C0(_, _, _, h2686, _) => {
                h2686
            }
        })
    } {
        let a2687 = f709(v1.clone());
        if bool_not(a2687) {
            true
        } else {
            f450(match v0.clone() {
                Adt17::C0(_, _, h2688, _, _) => {
                    h2688
                }
            })
        }
    } else {
        false
    } {
        let a2689 = f710(v1);
        if bool_not(a2689) {
            true
        } else {
            f450(match v0 {
                Adt17::C0(_, h2690, _, _, _) => {
                    h2690
                }
            })
        }
    } else {
        false
    }
}

pub fn f694(v0: Adt81) -> List<(Str, Adt64)> {
    let m2691 = v0;
    match m2691 {
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

pub fn f695(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt116, Adt119> {
    if v2 {
        Ok::<Adt116, Adt119>(f570(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt116, Adt119>(Adt119::C4(Adt84::C8(f711())))
    }
}

pub fn f696(v0: Adt17) -> bool {
    f451(match v0 {
        Adt17::C0(_, _, _, _, h2692) => {
            h2692
        }
    }, f452())
}

pub fn f697(v0: Adt16, v1: List<Adt50>) -> List<Adt118> {
    let m2694 = match v0 {
        Adt16::C0(_, _, _, _, h2693, _) => {
            h2693
        }
    };
    match m2694 {
        None => {
            List::<Adt118>::nil()
        }
        Some(v2) => {
            f712(f713(v2, v1, List::<Adt118>::nil()), List::<Adt118>::nil())
        }
    }
}

pub fn f698(p2700: List<Str>, p2701: Adt5) -> Adt123 {
    let m2699 = (p2700, p2701);
    {
        let mut m2699 = m2699;
        loop {
            let step = {
                let (v0, v1) = m2699;
                let m2695 = v0;
                match m2695.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                    }
                    Some((v2, v3)) => {
                        let m2696 = f177(v2, v1);
                        match m2696 {
                            Adt123::C0 => {
                                Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                            }
                            Adt123::C1(v4) => {
                                let m2697 = v3.clone();
                                match m2697.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt123>(Adt123::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2698 = f329(v4);
                                        match m2698 {
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
                Ok(next) => { m2699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f699(v0: Adt99) -> Option<Adt97> {
    let m2703 = match v0.clone() {
        Adt99::C0(_, _, _, h2702) => {
            h2702
        }
    };
    match m2703 {
        Adt100::C0 => {
            None::<Adt97>
        }
        Adt100::C1 => {
            Some(Adt97::C10(match v0 {
                Adt99::C0(h2704, _, _, _) => {
                    h2704
                }
            }))
        }
        Adt100::C2(_) => {
            None::<Adt97>
        }
    }
}

pub fn f700(v0: Adt6) -> bool {
    let m2705 = v0;
    match m2705 {
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

pub fn f701(v0: Adt99, v1: Adt6) -> R<Option<Adt97>> {
    let m2707 = match v0.clone() {
        Adt99::C0(_, _, _, h2706) => {
            h2706
        }
    };
    match m2707 {
        Adt100::C0 => {
            Ok(Some(Adt97::C8(match v0 {
                Adt99::C0(h2708, _, _, _) => {
                    h2708
                }
            })))
        }
        Adt100::C1 => {
            Ok(Some(Adt97::C10(match v0 {
                Adt99::C0(h2709, _, _, _) => {
                    h2709
                }
            })))
        }
        Adt100::C2(v2) => {
            if f714(match v0.clone() {
                Adt99::C0(_, _, h2710, _) => {
                    h2710
                }
            }, v2, v1)? {
                Ok(None::<Adt97>)
            } else {
                Ok(Some(Adt97::C9(match v0 {
                    Adt99::C0(h2711, _, _, _) => {
                        h2711
                    }
                })))
            }
        }
    }
}

pub fn f702(p2716: u64, p2717: i64) -> R<i64> {
    let m2715 = (p2716, p2717);
    {
        let mut m2715 = m2715;
        loop {
            let step = {
                let (v0, v1) = m2715;
                let m2712 = v0;
                if m2712 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2712 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2713 = v1;
                        let a2714 = 10i64;
                        int_mul(a2713, a2714)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f703(p2728: u64, p2729: i64, p2730: u64) -> R<u64> {
    let m2727 = (p2728, (p2729, p2730));
    {
        let mut m2727 = m2727;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2727;
                let m2718 = v0;
                if m2718 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2718 - 1;
                    let v4: u64 = {
                        let a2719 = v2;
                        let a2720 = 1u64;
                        nat_add(a2719, a2720)?
                    };
                    let m2723 = {
                        let a2721 = v1;
                        let a2722 = 10i64;
                        compare(a2721, a2722)
                    };
                    if match m2723 {
                        Ordering::Less => {
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
                            let a2724 = v1;
                            let a2725 = 10i64;
                            let a2726 = 0i64;
                            int_quot(a2724, a2725, a2726)?
                        }, v4)))
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

pub fn f704(p2733: List<Adt41>, p2734: List<Adt6>) -> List<Adt6> {
    let m2732 = (p2733, p2734);
    {
        let mut m2732 = m2732;
        loop {
            let step = {
                let (v0, v1) = m2732;
                let m2731 = v0;
                match m2731.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f619(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f705(v0: Adt3) -> Str {
    let m2735 = v0;
    match m2735 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f706(v0: u64) -> bool {
    let a2736 = v0;
    let a2737 = 9u64;
    if nat_eq(a2736, a2737) {
        true
    } else {
        let a2738 = v0;
        let a2739 = 10u64;
        if nat_eq(a2738, a2739) {
            true
        } else {
            let a2740 = v0;
            let a2741 = 13u64;
            nat_eq(a2740, a2741)
        }
    }
}

pub fn f707(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f710(v0.clone()) {
        Some(f28())
    } else {
        None::<Adt5>
    }, if f709(v0.clone()) {
        Some(f28())
    } else {
        None::<Adt5>
    }, if f708(v0) {
        Some(f28())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f708(p2746: List<(Str, Adt64)>) -> bool {
    let m2745 = p2746;
    {
        let mut m2745 = m2745;
        loop {
            let step = {
                let v0 = m2745;
                let m2742 = v0;
                match m2742.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2744 = {
                            let (_, h2743) = v1;
                            h2743
                        };
                        match m2744 {
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
                Ok(next) => { m2745 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f709(p2751: List<(Str, Adt64)>) -> bool {
    let m2750 = p2751;
    {
        let mut m2750 = m2750;
        loop {
            let step = {
                let v0 = m2750;
                let m2747 = v0;
                match m2747.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2749 = {
                            let (_, h2748) = v1;
                            h2748
                        };
                        match m2749 {
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
                Ok(next) => { m2750 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f710(p2756: List<(Str, Adt64)>) -> bool {
    let m2755 = p2756;
    {
        let mut m2755 = m2755;
        loop {
            let step = {
                let v0 = m2755;
                let m2752 = v0;
                match m2752.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2754 = {
                            let (_, h2753) = v1;
                            h2753
                        };
                        match m2754 {
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
                Ok(next) => { m2755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f711() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f452(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f712(p2759: List<Adt118>, p2760: List<Adt118>) -> List<Adt118> {
    let m2758 = (p2759, p2760);
    {
        let mut m2758 = m2758;
        loop {
            let step = {
                let (v0, v1) = m2758;
                let m2757 = v0;
                match m2757.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<Adt118>), List<Adt118>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt118>, List<Adt118>), List<Adt118>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f713(v0: Adt19, p2763: List<Adt50>, p2764: List<Adt118>) -> List<Adt118> {
    let m2762 = (p2763, p2764);
    {
        let mut m2762 = m2762;
        loop {
            let step = {
                let (v1, v2) = m2762;
                let m2761 = v1;
                match m2761.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt118>), List<Adt118>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt118>), List<Adt118>>((v4, List::cons(Adt118::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m2762 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f714(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2765 = v0;
    match m2765 {
        Adt13::C0 => {
            let m2766 = f233(v2);
            match m2766 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2767 = v1;
                    let a2768 = v3;
                    Ok(equal(a2767, a2768))
                }
            }
        }
        Adt13::C1 => {
            f715(v1, v2)
        }
        Adt13::C2 => {
            let m2769 = f328(v2);
            match m2769 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2770 = v1;
                        let a2771 = Str::lit("true");
                        Ok(equal(a2770, a2771))
                    } else {
                        let a2772 = v1;
                        let a2773 = Str::lit("false");
                        Ok(equal(a2772, a2773))
                    }
                }
            }
        }
    }
}

pub fn f715(v0: Str, v1: Adt6) -> R<bool> {
    let m2774 = f540(v1);
    match m2774 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2775 = f235(v2)?;
            match m2775 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2777 = f716(f69({
                        let a2776 = v0;
                        utf8_encode(a2776)
                    })?)?;
                    match m2777 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f397({
                                let a2778 = v3;
                                let a2779 = v4;
                                int_sub(a2778, a2779)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f716(v0: List<u64>) -> R<Option<i64>> {
    let m2780 = v0.clone();
    match m2780.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2781 = v1;
            let a2782 = 45u64;
            if nat_eq(a2781, a2782) {
                let m2783 = v2.clone();
                match m2783.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2784 = f717(v2, 0i64)?;
                        match m2784 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2785 = v5;
                                    int_neg(a2785)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f717(v0, 0i64)
            }
        }
    }
}

pub fn f717(p2791: List<u64>, p2792: i64) -> R<Option<i64>> {
    let m2790 = (p2791, p2792);
    {
        let mut m2790 = m2790;
        loop {
            let step = {
                let (v0, v1) = m2790;
                let m2786 = v0;
                match m2786.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2787 = f718(v2);
                        match m2787 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f266({
                                    let a2788 = v1;
                                    let a2789 = 10i64;
                                    int_mul(a2788, a2789)?
                                }, f303(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: u64) -> Option<u64> {
    let a2793 = v0;
    let a2794 = 48u64;
    if if nat_lt(a2793, a2794) {
        true
    } else {
        let a2795 = 57u64;
        let a2796 = v0;
        nat_lt(a2795, a2796)
    } {
        None::<u64>
    } else {
        Some({
            let a2797 = v0;
            let a2798 = 48u64;
            nat_sub(a2797, a2798)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
