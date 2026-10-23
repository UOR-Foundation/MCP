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
            let a73 = f18(0u64);
            let a74 = f18(0u64);
            append_list(a73, a74)
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

pub fn f23() -> List<Adt92> {
    List::cons(Adt92::C0(f46(), f47()), List::<Adt92>::nil())
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

pub fn f25(v0: Adt0, v1: Fn0, v2: Adt91, v3: Option<Adt96>) -> R<Adt94> {
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

pub fn f26(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
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

pub fn f29(v0: List<Adt36>) -> R<Result<List<Adt12>, Adt97>> {
    let m86 = f55(v0.clone());
    match m86 {
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
    let m87 = v0;
    match m87 {
        Adt97::C0(v1) => {
            let a88 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a89 = Str::lit(" ");
            join(a88, a89)
        }
        Adt97::C1(v2) => {
            let a90 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a91 = Str::lit(" ");
            join(a90, a91)
        }
        Adt97::C2(v3) => {
            let a92 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a93 = Str::lit(" ");
            join(a92, a93)
        }
        Adt97::C3(v4) => {
            let a94 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a95 = Str::lit(" ");
            join(a94, a95)
        }
        Adt97::C4(v5, v6) => {
            let a96 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a97 = Str::lit(" ");
            join(a96, a97)
        }
        Adt97::C5(v7, v8) => {
            let a98 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a99 = Str::lit(" ");
            join(a98, a99)
        }
        Adt97::C6(v9, v10) => {
            let a100 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a101 = Str::lit(" ");
            join(a100, a101)
        }
        Adt97::C7(v11) => {
            let a102 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a103 = Str::lit(" ");
            join(a102, a103)
        }
        Adt97::C8(v12) => {
            let a104 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a105 = Str::lit(" ");
            join(a104, a105)
        }
        Adt97::C9(v13) => {
            let a106 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a107 = Str::lit(" ");
            join(a106, a107)
        }
        Adt97::C10(v14) => {
            let a108 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a109 = Str::lit(" ");
            join(a108, a109)
        }
        Adt97::C11(v15) => {
            let a110 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a111 = Str::lit(" ");
            join(a110, a111)
        }
        Adt97::C12(v16, v17) => {
            let a112 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a113 = Str::lit(" ");
            join(a112, a113)
        }
    }
}

pub fn f33(v0: Adt14, v1: List<Adt36>, v2: Fn1, v3: Fn2, v4: Adt22) -> R<Result<Adt33, Adt90>> {
    let m115 = f70(match v4.clone() {
        Adt22::C0(_, h114, _, _, _) => {
            h114
        }
    }, v1.clone());
    match m115 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v4 {
                Adt22::C0(_, h116, _, _, _) => {
                    h116
                }
            })))
        }
        Some(v5) => {
            f71(v0, v5, v4.clone(), {
                let c117: Fn1 = v2;
                c117.apply(v4.clone())?
            }, f72(v1, v3, v4)?)
        }
    }
}

pub fn f34(v0: Adt22) -> R<Option<Adt98>> {
    if f73(v0.clone(), Str::lit("greet")) {
        Ok(Some(Adt98::C0(f74({
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
    let m122 = f97(v0.clone(), match v1.clone() {
        Adt30::C0(h120, _) => {
            h120
        }
    }, match v1 {
        Adt30::C0(_, h121) => {
            h121
        }
    })?;
    match m122 {
        Ok(v3) => {
            f98(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f37(v0: Adt14, v1: Adt31) -> R<Result<Adt33, Adt90>> {
    let m125 = f97(v0.clone(), match v1.clone() {
        Adt31::C0(h123, _, _) => {
            h123
        }
    }, match v1.clone() {
        Adt31::C0(_, h124, _) => {
            h124
        }
    })?;
    match m125 {
        Ok(v3) => {
            f99(v0, v3, match v1 {
                Adt31::C0(_, _, h126) => {
                    h126
                }
            })
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f38(v0: Adt14, v1: Adt32) -> R<Result<Adt33, Adt90>> {
    let m129 = f97(v0.clone(), match v1.clone() {
        Adt32::C0(h127, _) => {
            h127
        }
    }, match v1 {
        Adt32::C0(_, h128) => {
            h128
        }
    })?;
    match m129 {
        Ok(v3) => {
            f100(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f39(v0: Adt15) -> R<Result<Adt33, Adt90>> {
    let m130 = v0;
    match m130 {
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
    let a134 = {
        let a131 = v1;
        let a132 = 4u64;
        nat_add(a131, a132)?
    };
    let a135 = {
        let a133 = v0.clone();
        length_bytes(a133)
    };
    if nat_le(a134, a135) {
        Ok(Some({
            let a152 = {
                let a136 = f108(v0.clone(), v1)?;
                let a137 = 16777216u64;
                nat_mul(a136, a137)?
            };
            let a153 = {
                let a150 = {
                    let a140 = f108(v0.clone(), {
                        let a138 = v1;
                        let a139 = 1u64;
                        nat_add(a138, a139)?
                    })?;
                    let a141 = 65536u64;
                    nat_mul(a140, a141)?
                };
                let a151 = {
                    let a148 = {
                        let a144 = f108(v0.clone(), {
                            let a142 = v1;
                            let a143 = 2u64;
                            nat_add(a142, a143)?
                        })?;
                        let a145 = 256u64;
                        nat_mul(a144, a145)?
                    };
                    let a149 = f108(v0, {
                        let a146 = v1;
                        let a147 = 3u64;
                        nat_add(a146, a147)?
                    })?;
                    nat_add(a148, a149)?
                };
                nat_add(a150, a151)?
            };
            nat_add(a152, a153)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f41(v0: Bytes, v1: Adt95, v2: Adt95, v3: Option<u64>) -> R<Result<Adt91, Adt93>> {
    let m154 = v3;
    match m154 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(match v2 {
                Adt95::C0(_, h155) => {
                    h155
                }
            })))
        }
        Some(v4) => {
            let m159 = f109(v0.clone(), {
                let a157 = match v2.clone() {
                    Adt95::C0(_, h156) => {
                        h156
                    }
                };
                let a158 = 4u64;
                nat_add(a157, a158)?
            }, v4, List::<Adt92>::nil())?;
            match m159 {
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

pub fn f42(p164: List<Adt92>, p165: List<u64>) -> List<u64> {
    let m163 = (p164, p165);
    {
        let mut m163 = m163;
        loop {
            let step = {
                let (v0, v1) = m163;
                let m160 = v0;
                match m160.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<u64>), List<u64>>((v3, {
                            let a161 = f111(v2);
                            let a162 = v1;
                            append_list(a161, a162)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(p168: List<Adt92>, p169: List<Adt92>) -> List<Adt92> {
    let m167 = (p168, p169);
    {
        let mut m167 = m167;
        loop {
            let step = {
                let (v0, v1) = m167;
                let m166 = v0;
                match m166.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(p174: List<List<u64>>, p175: List<u64>) -> List<u64> {
    let m173 = (p174, p175);
    {
        let mut m173 = m173;
        loop {
            let step = {
                let (v0, v1) = m173;
                let m170 = v0;
                match m170.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a171 = f112(v2);
                            let a172 = v1;
                            append_list(a171, a172)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(p178: List<List<u64>>, p179: List<List<u64>>) -> List<List<u64>> {
    let m177 = (p178, p179);
    {
        let mut m177 = m177;
        loop {
            let step = {
                let (v0, v1) = m177;
                let m176 = v0;
                match m176.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
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
        Adt91::C0(h180, _, _, _, _, _, _) => {
            h180
        }
    }) {
        f118(match v2.clone() {
            Adt91::C0(_, _, _, _, _, _, h181) => {
                h181
            }
        }, v0, Fn3::F119(v1, v2.clone()), match v2.clone() {
            Adt91::C0(_, _, _, _, h182, _, _) => {
                h182
            }
        }, match v2.clone() {
            Adt91::C0(_, _, h183, _, _, _, _) => {
                h183
            }
        }, f120(match v2 {
            Adt91::C0(_, _, _, h184, _, _, _) => {
                h184
            }
        })?)
    } else {
        Ok(f121())
    }
}

pub fn f53(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m185 = f122(v0, f123())?;
    match m185 {
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
    let m186 = f122(v0, f126())?;
    match m186 {
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
    let m187 = f128(v0, List::<Str>::nil());
    match m187 {
        None => {
            None::<Adt97>
        }
        Some(v1) => {
            Some(Adt97::C1(v1))
        }
    }
}

pub fn f56(p191: List<Adt36>, p192: List<Adt12>) -> R<Result<List<Adt12>, Adt97>> {
    let m190 = (p191, p192);
    {
        let mut m190 = m190;
        loop {
            let step = {
                let (v0, v1) = m190;
                let m188 = v0;
                match m188.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>(Ok(Ok::<List<Adt12>, Adt97>(f129(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m189 = f130(v2)?;
                        match m189 {
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
                Ok(next) => { m190 = next; }
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

pub fn f70(v0: Str, p198: List<Adt36>) -> Option<Adt36> {
    let m197 = p198;
    {
        let mut m197 = m197;
        loop {
            let step = {
                let v1 = m197;
                let m193 = v1;
                match m193.uncons() {
                    None => {
                        Err::<List<Adt36>, Option<Adt36>>(None::<Adt36>)
                    }
                    Some((v2, v3)) => {
                        let a195 = match v2.clone() {
                            Adt36::C0(h194, _, _, _, _, _, _, _) => {
                                h194
                            }
                        };
                        let a196 = v0.clone();
                        if equal(a195, a196) {
                            Err::<List<Adt36>, Option<Adt36>>(Some(v2))
                        } else {
                            Ok::<List<Adt36>, Option<Adt36>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: Adt14, v1: Adt36, v2: Adt22, v3: Option<Adt98>, v4: Result<Adt33, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m199 = v3;
    match m199 {
        None => {
            Ok(v4)
        }
        Some(v5) => {
            f136(v0, v1.clone(), v2.clone(), v5, f137(v1, v2)?)
        }
    }
}

pub fn f72(v0: List<Adt36>, v1: Fn2, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m201 = f70(match v2.clone() {
        Adt22::C0(_, h200, _, _, _) => {
            h200
        }
    }, v0);
    match m201 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h202, _, _, _) => {
                    h202
                }
            })))
        }
        Some(v3) => {
            f138(v3, v1, v2)
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
    Adt50::C0(Adt38::C0(List::cons(f139(v0), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt51>::nil())
}

pub fn f75(v0: Adt22, v1: Str, v2: Str) -> Str {
    let m206 = f140(v0, v1);
    match m206 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            let m207 = f141(v3);
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

pub fn f76(v0: Adt22) -> R<Adt98> {
    let v1: i64 = f142(v0.clone());
    let v2: Str = {
        let a209 = List::cons(Str::lit("slow_compute"), List::cons(f75(v0, Str::lit("label"), Str::lit("unnamed")), List::cons(Str::lit("finished after"), List::cons({
            let a208 = v1;
            format_i64(a208)
        }, List::cons(Str::lit("seconds"), List::<Str>::nil())))));
        let a210 = Str::lit(" ");
        join(a209, a210)
    };
    Ok(Adt98::C1(Adt99::C0, f74(v2.clone(), false), f143(0i64, List::<(Str, Adt65)>::nil(), {
        let a211 = v1;
        let a212 = 1000i64;
        int_mul(a211, a212)?
    }, f144(v2, false)?)))
}

pub fn f77(v0: Adt22) -> R<Adt98> {
    let v1: Str = {
        let a213 = List::cons(Str::lit("Deleted"), List::cons(f75(v0.clone(), Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a214 = Str::lit(" ");
        join(a213, a214)
    };
    Ok(Adt98::C1(Adt99::C0, f74(v1.clone(), false), f143(0i64, List::cons((Str::lit("confirm"), f145({
        let a215 = List::cons(Str::lit("Delete"), List::cons(f75(v0, Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a216 = Str::lit(" ");
        join(a215, a216)
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
    let m218 = match v0 {
        Adt22::C0(_, _, _, h217, _) => {
            h217
        }
    };
    match m218 {
        None => {
            Ok(f146())
        }
        Some(v1) => {
            let m219 = f147(v1);
            match m219 {
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
    let a221 = match v0 {
        Adt22::C0(_, h220, _, _, _) => {
            h220
        }
    };
    let a222 = v1;
    equal(a221, a222)
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
        let m225 = match v0 {
            Adt22::C0(h224, _, _, _, _) => {
                h224
            }
        };
        match m225 {
            Adt17::C0(_, _, _, _, h223, _) => {
                h223
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
    let m227 = {
        let a226 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a226)
    };
    match m227 {
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
        let m233 = {
            let m232 = {
                let m231 = match v0 {
                    Adt22::C0(h230, _, _, _, _) => {
                        h230
                    }
                };
                match m231 {
                    Adt17::C0(_, h229, _, _, _, _) => {
                        h229
                    }
                }
            };
            match m232 {
                Adt18::C0(_, _, h228, _, _) => {
                    h228
                }
            }
        };
        match m233 {
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
        Adt17::C0(_, h234, _, _, _, _) => {
            h234
        }
    }) {
        let m236 = f154(match v0.clone() {
            Adt14::C0(_, _, _, h235) => {
                h235
            }
        })?;
        match m236 {
            None => {
                Ok(Err::<Adt85, Adt90>(f155()))
            }
            Some(v3) => {
                let m238 = f156(v2.clone(), v3, match v0 {
                    Adt14::C0(h237, _, _, _) => {
                        h237
                    }
                })?;
                match m238 {
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
        Adt14::C0(h239, _, _, _) => {
            h239
        }
    }, match v0 {
        Adt14::C0(_, h240, _, _) => {
            h240
        }
    })?, Adt89::C0, false)))
}

pub fn f99(v0: Adt14, v1: Adt85, v2: Adt5) -> R<Result<Adt33, Adt90>> {
    let v3: Adt101 = f159(v1, match v0.clone() {
        Adt14::C0(h241, _, _, _) => {
            h241
        }
    }, match v0.clone() {
        Adt14::C0(_, h242, _, _) => {
            h242
        }
    })?;
    Ok(Ok::<Adt33, Adt90>(f158(f160(match v3.clone() {
        Adt101::C0(h243, _) => {
            h243
        }
    }, v2, match v0.clone() {
        Adt14::C0(h244, _, _, _) => {
            h244
        }
    }, match v0 {
        Adt14::C0(_, h245, _, _) => {
            h245
        }
    })?, Adt89::C1, match v3 {
        Adt101::C0(_, h246) => {
            h246
        }
    })))
}

pub fn f100(v0: Adt14, v1: Adt85) -> R<Result<Adt33, Adt90>> {
    Ok(Ok::<Adt33, Adt90>(f158(f161(v1, match v0.clone() {
        Adt14::C0(h247, _, _, _) => {
            h247
        }
    }, match v0 {
        Adt14::C0(_, h248, _, _) => {
            h248
        }
    })?, Adt89::C1, false)))
}

pub fn f101(v0: Adt10, v1: List<Adt36>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m250 = match v2 {
        Adt21::C0(_, h249) => {
            h249
        }
    };
    match m250 {
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
    let m253 = {
        let a251 = v0;
        let a252 = v1;
        index_bytes(a251, a252)
    };
    match m253 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a298 = {
                let a274 = {
                    let a262 = {
                        let a256 = {
                            let a254 = v3;
                            let a255 = 128u8;
                            fixed_u8::bit_and(a254, a255)
                        };
                        let a257 = 128u8;
                        if equal(a256, a257) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a263 = {
                        let a260 = {
                            let a258 = v3;
                            let a259 = 64u8;
                            fixed_u8::bit_and(a258, a259)
                        };
                        let a261 = 64u8;
                        if equal(a260, a261) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a262, a263)?
                };
                let a275 = {
                    let a272 = {
                        let a266 = {
                            let a264 = v3;
                            let a265 = 32u8;
                            fixed_u8::bit_and(a264, a265)
                        };
                        let a267 = 32u8;
                        if equal(a266, a267) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a273 = {
                        let a270 = {
                            let a268 = v3;
                            let a269 = 16u8;
                            fixed_u8::bit_and(a268, a269)
                        };
                        let a271 = 16u8;
                        if equal(a270, a271) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a272, a273)?
                };
                nat_add(a274, a275)?
            };
            let a299 = {
                let a296 = {
                    let a284 = {
                        let a278 = {
                            let a276 = v3;
                            let a277 = 8u8;
                            fixed_u8::bit_and(a276, a277)
                        };
                        let a279 = 8u8;
                        if equal(a278, a279) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a285 = {
                        let a282 = {
                            let a280 = v3;
                            let a281 = 4u8;
                            fixed_u8::bit_and(a280, a281)
                        };
                        let a283 = 4u8;
                        if equal(a282, a283) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a284, a285)?
                };
                let a297 = {
                    let a294 = {
                        let a288 = {
                            let a286 = v3;
                            let a287 = 2u8;
                            fixed_u8::bit_and(a286, a287)
                        };
                        let a289 = 2u8;
                        if equal(a288, a289) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a295 = {
                        let a292 = {
                            let a290 = v3;
                            let a291 = 1u8;
                            fixed_u8::bit_and(a290, a291)
                        };
                        let a293 = 1u8;
                        if equal(a292, a293) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a294, a295)?
                };
                nat_add(a296, a297)?
            };
            nat_add(a298, a299)
        }
    }
}

pub fn f109(v0: Bytes, p309: u64, p310: u64, p311: List<Adt92>) -> R<Result<(List<Adt92>, u64), Adt93>> {
    let m308 = (p309, (p310, p311));
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v1, (v2, v3)) = m308;
                let m300 = v2;
                if m300 == 0 {
                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Ok::<(List<Adt92>, u64), Adt93>((f176(v3, List::<Adt92>::nil()), v1))))
                } else {
                    let v4 = m300 - 1;
                    let m301 = f16(v0.clone(), v1)?;
                    match m301 {
                        None => {
                            Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Err::<(List<Adt92>, u64), Adt93>(Adt93::C0(v1))))
                        }
                        Some(v5) => {
                            let m303 = f16(v0.clone(), match v5.clone() {
                                Adt95::C0(_, h302) => {
                                    h302
                                }
                            })?;
                            match m303 {
                                None => {
                                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Err::<(List<Adt92>, u64), Adt93>(Adt93::C0(match v5 {
                                        Adt95::C0(_, h304) => {
                                            h304
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>((match v6.clone() {
                                        Adt95::C0(_, h305) => {
                                            h305
                                        }
                                    }, (v4, List::cons(Adt92::C0(f177(match v5 {
                                        Adt95::C0(h306, _) => {
                                            h306
                                        }
                                    })?, f177(match v6 {
                                        Adt95::C0(h307, _) => {
                                            h307
                                        }
                                    })?), v3))))
                                }
                            }
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

pub fn f110(v0: Bytes, v1: Adt95, v2: Adt95, v3: (List<Adt92>, u64)) -> R<Result<Adt91, Adt93>> {
    let m313 = f16(v0.clone(), {
        let (_, h312) = v3.clone();
        h312
    })?;
    match m313 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0({
                let (_, h314) = v3;
                h314
            })))
        }
        Some(v4) => {
            let m316 = f40(v0.clone(), match v4.clone() {
                Adt95::C0(_, h315) => {
                    h315
                }
            })?;
            match m316 {
                None => {
                    Ok(Err::<Adt91, Adt93>(Adt93::C0(match v4 {
                        Adt95::C0(_, h317) => {
                            h317
                        }
                    })))
                }
                Some(v5) => {
                    f178(v0.clone(), Adt91::C0(f177(match v1 {
                        Adt95::C0(h318, _) => {
                            h318
                        }
                    })?, f177(match v2 {
                        Adt95::C0(h319, _) => {
                            h319
                        }
                    })?, {
                        let (h320, _) = v3;
                        h320
                    }, match v4.clone() {
                        Adt95::C0(h321, _) => {
                            h321
                        }
                    }, v5, List::<List<u64>>::nil(), Adt14::C0(0i64, Str::lit(""), Str::lit(""), f179())), f180(v0, {
                        let a323 = match v4 {
                            Adt95::C0(_, h322) => {
                                h322
                            }
                        };
                        let a324 = 4u64;
                        nat_add(a323, a324)?
                    })?)
                }
            }
        }
    }
}

pub fn f111(v0: Adt92) -> List<u64> {
    let a335 = f18({
        let a326 = match v0.clone() {
            Adt92::C0(h325, _) => {
                h325
            }
        };
        length_list(a326)
    });
    let a336 = {
        let a333 = match v0.clone() {
            Adt92::C0(h327, _) => {
                h327
            }
        };
        let a334 = {
            let a331 = f18({
                let a329 = match v0.clone() {
                    Adt92::C0(_, h328) => {
                        h328
                    }
                };
                length_list(a329)
            });
            let a332 = match v0 {
                Adt92::C0(_, h330) => {
                    h330
                }
            };
            append_list(a331, a332)
        };
        append_list(a333, a334)
    };
    append_list(a335, a336)
}

pub fn f112(v0: List<u64>) -> List<u64> {
    let a338 = f18({
        let a337 = v0.clone();
        length_list(a337)
    });
    let a339 = v0;
    append_list(a338, a339)
}

pub fn f113(v0: u64, v1: Adt6) -> R<Adt94> {
    let m340 = f181(v1)?;
    match m340 {
        Ok(v3) => {
            Ok(Adt94::C0(v0, f23(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f114(v0: Adt96) -> u64 {
    let m341 = v0;
    match m341 {
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
    let m342 = v5;
    match m342 {
        Ok(v7) => {
            f190(v0, v1, v2, v3, v4, f191(v7)?)
        }
        Err(_) => {
            f113(f188(), f115(None::<Adt100>, f189(Adt103::C0)))
        }
    }
}

pub fn f119(v0: Fn0, v1: Adt91, v2: Adt15) -> R<Result<Adt33, Adt90>> {
    let c343: Fn0 = v0;
    c343.apply(match v1 {
        Adt91::C0(_, _, _, _, _, _, h344) => {
            h344
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
    let m345 = f196(v0, v1, Adt106::C0)?;
    match m345 {
        Adt106::C0 => {
            Ok(Adt105::C0)
        }
        Adt106::C1(v2) => {
            let m346 = f197(v2);
            match m346 {
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
    let m347 = v0;
    match m347 {
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
    let m348 = f199(v0);
    match m348 {
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
    let m349 = v0.clone();
    match m349.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a350 = v1;
            let a351 = 91u64;
            if nat_eq(a350, a351) {
                f200(f201(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f200(f202(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f128(p357: List<Adt36>, p358: List<Str>) -> Option<Str> {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v0, v1) = m356;
                let m352 = v0;
                match m352.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f203(match v2.clone() {
                            Adt36::C0(h353, _, _, _, _, _, _, _) => {
                                h353
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt36>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt36::C0(h354, _, _, _, _, _, _, _) => {
                                    h354
                                }
                            }))
                        } else {
                            Ok::<(List<Adt36>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt36::C0(h355, _, _, _, _, _, _, _) => {
                                    h355
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(p361: List<Adt12>, p362: List<Adt12>) -> List<Adt12> {
    let m360 = (p361, p362);
    {
        let mut m360 = m360;
        loop {
            let step = {
                let (v0, v1) = m360;
                let m359 = v0;
                match m359.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
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

pub fn f130(v0: Adt36) -> R<Result<List<Adt12>, Adt97>> {
    let m365 = f204(match v0.clone() {
        Adt36::C0(h363, _, _, _, _, _, _, _) => {
            h363
        }
    }, f205(match v0.clone() {
        Adt36::C0(h364, _, _, _, _, _, _, _) => {
            h364
        }
    })?);
    match m365 {
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
    let m366 = v4;
    match m366 {
        Ok(v6) => {
            let m367 = v6;
            match m367 {
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
    let m374 = f209(Adt6::C5({
        let m370 = match v0.clone() {
            Adt36::C0(_, _, _, _, h369, _, _, _) => {
                h369
            }
        };
        match m370 {
            Adt5::C0(h368) => {
                h368
            }
        }
    }), Adt6::C5({
        let m373 = match v0 {
            Adt36::C0(_, _, _, _, h372, _, _, _) => {
                h372
            }
        };
        match m373 {
            Adt5::C0(h371) => {
                h371
            }
        }
    }), f210(v1))?;
    match m374 {
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
    let m381 = f209(Adt6::C5({
        let m377 = match v0.clone() {
            Adt36::C0(_, _, _, _, h376, _, _, _) => {
                h376
            }
        };
        match m377 {
            Adt5::C0(h375) => {
                h375
            }
        }
    }), Adt6::C5({
        let m380 = match v0.clone() {
            Adt36::C0(_, _, _, _, h379, _, _, _) => {
                h379
            }
        };
        match m380 {
            Adt5::C0(h378) => {
                h378
            }
        }
    }), f210(v2.clone()))?;
    match m381 {
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
    let m383 = match v0 {
        Adt22::C0(_, _, h382, _, _) => {
            h382
        }
    };
    match m383 {
        None => {
            Adt107::C0
        }
        Some(v2) => {
            f214(v1, v2)
        }
    }
}

pub fn f141(v0: Adt6) -> Option<Str> {
    let m384 = v0;
    match m384 {
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
    let m385 = f140(v0, Str::lit("seconds"));
    match m385 {
        Adt107::C0 => {
            0i64
        }
        Adt107::C1(v1) => {
            let m386 = f215(v1);
            match m386 {
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
    let m389 = f216(f217({
        let m388 = f74(v0, v1);
        match m388 {
            Adt50::C0(h387, _) => {
                h387
            }
        }
    })?)?;
    match m389 {
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
    let m390 = f214(Str::lit("user_name"), v0);
    match m390 {
        Adt107::C0 => {
            None::<Str>
        }
        Adt107::C1(v1) => {
            let m391 = f219(v1);
            match m391 {
                None => {
                    None::<Str>
                }
                Some(v2) => {
                    let m392 = f214(Str::lit("content"), v2);
                    match m392 {
                        Adt107::C0 => {
                            None::<Str>
                        }
                        Adt107::C1(v3) => {
                            let m393 = f219(v3);
                            match m393 {
                                None => {
                                    None::<Str>
                                }
                                Some(v4) => {
                                    let m394 = f214(Str::lit("name"), v4);
                                    match m394 {
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
        let a395 = List::cons(Str::lit("Hello, "), List::cons(v0, List::cons(Str::lit("! (from a task)"), List::<Str>::nil())));
        let a396 = Str::lit("");
        join(a395, a396)
    };
    Ok(Adt98::C1(Adt99::C1, f74(v1.clone(), false), f143(0i64, List::<(Str, Adt65)>::nil(), 100i64, f144(v1, false)?)))
}

pub fn f149(v0: Option<Adt20>) -> Str {
    let m397 = v0;
    match m397 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m398 = v1;
            match m398 {
                Adt20::C0(v2) => {
                    v2
                }
                Adt20::C1(v3) => {
                    let a399 = v3;
                    format_i64(a399)
                }
            }
        }
    }
}

pub fn f150(v0: u64, v1: u64) -> R<Adt51> {
    Ok(Adt51::C0(f220(v0)?, Some(f220(v1)?), Some({
        let a402 = List::cons(Str::lit("Completed step"), List::cons({
            let a400 = f221(v0, 0i64)?;
            format_i64(a400)
        }, List::cons(Str::lit("of"), List::cons({
            let a401 = f221(v1, 0i64)?;
            format_i64(a401)
        }, List::<Str>::nil()))));
        let a403 = Str::lit(" ");
        join(a402, a403)
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
        Adt18::C0(_, _, _, _, h404) => {
            h404
        }
    }, f28())
}

pub fn f154(v0: Bytes) -> R<Option<List<Adt85>>> {
    let a406 = {
        let a405 = v0.clone();
        length_bytes(a405)
    };
    let a407 = 0u64;
    if nat_eq(a406, a407) {
        Ok(Some(List::<Adt85>::nil()))
    } else {
        let m408 = f120(v0)?;
        match m408 {
            Ok(v2) => {
                let m409 = f223(v2);
                match m409 {
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

pub fn f156(v0: Str, p414: List<Adt85>, v2: i64) -> R<Option<Adt85>> {
    let m413 = p414;
    {
        let mut m413 = m413;
        loop {
            let step = {
                let v1 = m413;
                let m410 = v1;
                match m410.uncons() {
                    None => {
                        Err::<List<Adt85>, R<Option<Adt85>>>(Ok(None::<Adt85>))
                    }
                    Some((v3, v4)) => {
                        let a411 = f225(v3.clone());
                        let a412 = v0.clone();
                        if equal(a411, a412) {
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
                Ok(next) => { m413 = next; }
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
        Adt101::C0(h415, _) => {
            h415
        }
    }, v1, if match v0 {
        Adt101::C0(_, h416) => {
            h416
        }
    } {
        true
    } else {
        v2
    }))
}

pub fn f159(v0: Adt85, v1: i64, v2: Str) -> R<Adt101> {
    let m418 = match v0.clone() {
        Adt85::C0(h417, _, _) => {
            h417
        }
    };
    match m418 {
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
    let m420 = match v0.clone() {
        Adt85::C0(h419, _, _) => {
            h419
        }
    };
    match m420 {
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
    let m422 = match v2 {
        Adt21::C0(_, h421) => {
            h421
        }
    };
    match m422 {
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
        Adt23::C0(_, h423, _, _) => {
            h423
        }
    }, {
        let c424: Fn4 = v1;
        c424.apply(match v2 {
            Adt23::C0(_, h425, _, _) => {
                h425
            }
        })?
    }))
}

pub fn f166() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C1)
}

pub fn f167(v0: Str) -> R<Option<List<Adt47>>> {
    let m426 = f235(v0.clone(), f233());
    match m426 {
        None => {
            f236(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f168(v0: Adt10, v1: List<Adt55>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m428 = match v2 {
        Adt21::C0(_, h427) => {
            h427
        }
    };
    match m428 {
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
    let m430 = match v2 {
        Adt21::C0(_, h429) => {
            h429
        }
    };
    match m430 {
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
    let m432 = f240(match v2.clone() {
        Adt24::C0(_, h431, _, _, _) => {
            h431
        }
    }, v0);
    match m432 {
        None => {
            Err::<Adt33, Adt90>(Adt90::C1(match v2 {
                Adt24::C0(_, h433, _, _, _) => {
                    h433
                }
            }))
        }
        Some(v3) => {
            let m435 = f241(f242(v3), f243(match v2.clone() {
                Adt24::C0(_, _, h434, _, _) => {
                    h434
                }
            }));
            match m435 {
                None => {
                    Ok::<Adt33, Adt90>(Adt33::C8({
                        let c438: Fn5 = v1;
                        c438.apply(match v2.clone() {
                            Adt24::C0(_, h439, _, _, _) => {
                                h439
                            }
                        }, f243(match v2 {
                            Adt24::C0(_, _, h440, _, _) => {
                                h440
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt33, Adt90>(Adt90::C4({
                        let a436 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a437 = Str::lit(" ");
                        join(a436, a437)
                    }))
                }
            }
        }
    }
}

pub fn f173(v0: Str, v1: List<(Str, Str)>) -> Adt59 {
    let a441 = v0.clone();
    let a442 = Str::lit("test_simple_prompt");
    if equal(a441, a442) {
        f244(List::cons(f245(Str::lit("This is a simple prompt for testing.")), List::<Adt60>::nil()))
    } else {
        let a443 = v0.clone();
        let a444 = Str::lit("test_prompt_with_arguments");
        if equal(a443, a444) {
            f244(List::cons(f245({
                let a445 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f246(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f246(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a446 = Str::lit("");
                join(a445, a446)
            }), List::<Adt60>::nil()))
        } else {
            let a447 = v0.clone();
            let a448 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a447, a448) {
                f244(List::cons(Adt60::C0(Adt42::C0, Adt39::C4(Adt46::C0(f247(f246(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt41>, None::<Adt5>))), List::cons(f245(Str::lit("Please process the embedded resource above.")), List::<Adt60>::nil())))
            } else {
                let a449 = v0;
                let a450 = Str::lit("test_prompt_with_image");
                if equal(a449, a450) {
                    f244(List::cons(Adt60::C0(Adt42::C0, Adt39::C1(Adt43::C0(f248(), Str::lit("image/png"), None::<Adt41>, None::<Adt5>))), List::cons(f245(Str::lit("Please analyze the image above.")), List::<Adt60>::nil())))
                } else {
                    f244(List::<Adt60>::nil())
                }
            }
        }
    }
}

pub fn f174(v0: Fn6, v1: Adt25) -> R<Result<Adt33, Adt90>> {
    let m452 = {
        let c451: Fn6 = v0;
        c451.apply(v1)?
    };
    match m452 {
        Ok(v3) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C9(f249(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt33, Adt90>(v2))
        }
    }
}

pub fn f175(v0: Adt25) -> R<Result<List<Str>, Adt90>> {
    let m454 = match v0.clone() {
        Adt25::C0(_, h453, _, _) => {
            h453
        }
    };
    match m454 {
        Adt26::C0(v1) => {
            let m456 = f240(match v1.clone() {
                Adt27::C0(h455, _) => {
                    h455
                }
            }, f171());
            match m456 {
                None => {
                    Ok(Err::<List<Str>, Adt90>(Adt90::C1(match v1 {
                        Adt27::C0(h457, _) => {
                            h457
                        }
                    })))
                }
                Some(v2) => {
                    if f250(f242(v2), {
                        let m460 = match v0.clone() {
                            Adt25::C0(_, _, h459, _) => {
                                h459
                            }
                        };
                        match m460 {
                            Adt28::C0(h458, _) => {
                                h458
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt90>(f251({
                            let m463 = match v0.clone() {
                                Adt25::C0(_, _, h462, _) => {
                                    h462
                                }
                            };
                            match m463 {
                                Adt28::C0(_, h461) => {
                                    h461
                                }
                            }
                        }, f252(match v1 {
                            Adt27::C0(h464, _) => {
                                h464
                            }
                        }, {
                            let m467 = match v0 {
                                Adt25::C0(_, _, h466, _) => {
                                    h466
                                }
                            };
                            match m467 {
                                Adt28::C0(h465, _) => {
                                    h465
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
            let a468 = v3.clone();
            let a469 = f237();
            if equal(a468, a469) {
                let a473 = {
                    let m472 = match v0.clone() {
                        Adt25::C0(_, _, h471, _) => {
                            h471
                        }
                    };
                    match m472 {
                        Adt28::C0(h470, _) => {
                            h470
                        }
                    }
                };
                let a474 = Str::lit("id");
                if equal(a473, a474) {
                    Ok(Ok::<List<Str>, Adt90>(f251({
                        let m477 = match v0 {
                            Adt25::C0(_, _, h476, _) => {
                                h476
                            }
                        };
                        match m477 {
                            Adt28::C0(_, h475) => {
                                h475
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

pub fn f176(p480: List<Adt92>, p481: List<Adt92>) -> List<Adt92> {
    let m479 = (p480, p481);
    {
        let mut m479 = m479;
        loop {
            let step = {
                let (v0, v1) = m479;
                let m478 = v0;
                match m478.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes) -> R<List<u64>> {
    f254(v0.clone(), {
        let a482 = v0;
        length_bytes(a482)
    }, List::<u64>::nil())
}

pub fn f178(v0: Bytes, v1: Adt91, v2: Result<(Adt14, u64), Adt93>) -> R<Result<Adt91, Adt93>> {
    let m483 = v2;
    match m483 {
        Ok(v4) => {
            f255(v0, Adt91::C0(match v1.clone() {
                Adt91::C0(h484, _, _, _, _, _, _) => {
                    h484
                }
            }, match v1.clone() {
                Adt91::C0(_, h485, _, _, _, _, _) => {
                    h485
                }
            }, match v1.clone() {
                Adt91::C0(_, _, h486, _, _, _, _) => {
                    h486
                }
            }, match v1.clone() {
                Adt91::C0(_, _, _, h487, _, _, _) => {
                    h487
                }
            }, match v1.clone() {
                Adt91::C0(_, _, _, _, h488, _, _) => {
                    h488
                }
            }, match v1 {
                Adt91::C0(_, _, _, _, _, h489, _) => {
                    h489
                }
            }, {
                let (h490, _) = v4;
                h490
            }), {
                let (_, h491) = v4;
                h491
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
    let m492 = f256(v0.clone(), v1)?;
    match m492 {
        None => {
            Ok(Err::<(Adt14, u64), Adt93>(Adt93::C0(v1)))
        }
        Some(v2) => {
            let m495 = f257(v0.clone(), {
                let a493 = v1;
                let a494 = 8u64;
                nat_add(a493, a494)?
            })?;
            match m495 {
                Ok(v4) => {
                    f258(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt14, u64), Adt93>(v3))
                }
            }
        }
    }
}

pub fn f181(v0: Adt6) -> R<Result<List<u64>, Adt102>> {
    f259(f260(), v0)
}

pub fn f182() -> u64 {
    403u64
}

pub fn f183(v0: Option<Adt100>, v1: Adt81) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f261())), {
        let a496 = f262(v0);
        let a497 = List::cons((Str::lit("error"), f263(v1)), List::<(Str, Adt6)>::nil());
        append_list(a496, a497)
    })
}

pub fn f184() -> i64 {
    -32000i64
}

pub fn f185(v0: Adt96) -> Str {
    let m498 = v0;
    match m498 {
        Adt96::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt96::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f186(p505: List<u64>, p506: List<u64>) -> bool {
    let m504 = (p505, p506);
    {
        let mut m504 = m504;
        loop {
            let step = {
                let (v0, v1) = m504;
                let m499 = v0;
                match m499.uncons() {
                    None => {
                        let m500 = v1;
                        match m500.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m501 = v1;
                        match m501.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a502 = v4;
                                let a503 = v6;
                                if nat_eq(a502, a503) {
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
                Ok(next) => { m504 = next; }
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
    Adt81::C0(f264(v0.clone()), f265(v0), None::<Adt6>)
}

pub fn f190(v0: Adt14, v1: Adt0, v2: Fn3, v3: u64, v4: List<Adt92>, v5: Result<Adt108, Adt103>) -> R<Adt94> {
    let m507 = v5;
    match m507 {
        Ok(v7) => {
            f268(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f113(f266(v6.clone()), f115(f267(v6.clone()), f189(v6)))
        }
    }
}

pub fn f191(v0: Adt6) -> R<Result<Adt108, Adt103>> {
    let m508 = v0;
    match m508 {
        Adt6::C0 => {
            Ok(f270(None::<Adt100>, Adt104::C0))
        }
        Adt6::C1(_) => {
            Ok(f270(None::<Adt100>, Adt104::C0))
        }
        Adt6::C2(_) => {
            Ok(f270(None::<Adt100>, Adt104::C0))
        }
        Adt6::C3(_) => {
            Ok(f270(None::<Adt100>, Adt104::C0))
        }
        Adt6::C4(_) => {
            Ok(f270(None::<Adt100>, Adt104::C1))
        }
        Adt6::C5(v1) => {
            f269(Adt5::C0(v1))
        }
    }
}

pub fn f192(v0: List<u64>) -> R<Result<Adt6, Adt102>> {
    let m509 = f271(v0.clone())?;
    match m509 {
        Adt121::C0 => {
            f272(f273(), 0u64, v0)
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

pub fn f196(p514: List<Adt92>, v1: List<u64>, p515: Adt106) -> R<Adt106> {
    let m513 = (p514, p515);
    {
        let mut m513 = m513;
        loop {
            let step = {
                let (v0, v2) = m513;
                let m510 = v0;
                match m510.uncons() {
                    None => {
                        Err::<(List<Adt92>, Adt106), R<Adt106>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f274(match v3.clone() {
                            Adt92::C0(h511, _) => {
                                h511
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt92>, Adt106), R<Adt106>>((v4, f275(v2, match v3 {
                                Adt92::C0(_, h512) => {
                                    h512
                                }
                            })))
                        } else {
                            Ok::<(List<Adt92>, Adt106), R<Adt106>>((v4, v2))
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

pub fn f197(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f276(v0);
    if f277(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f198(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f274(v0.clone(), f278())? {
        Ok(true)
    } else {
        if f274(v0.clone(), f279())? {
            Ok(true)
        } else {
            if f274(v0.clone(), f280())? {
                Ok(true)
            } else {
                f281(v0, v1)
            }
        }
    }
}

pub fn f199(p520: List<u64>) -> Option<List<u64>> {
    let m519 = p520;
    {
        let mut m519 = m519;
        loop {
            let step = {
                let v0 = m519;
                let m516 = v0;
                match m516.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a517 = v1;
                        let a518 = 58u64;
                        if nat_eq(a517, a518) {
                            Err::<List<u64>, Option<List<u64>>>(f282(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m521 = v0;
    match m521 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f283({
                let (_, h522) = v1;
                h522
            }) {
                Some({
                    let (h523, _) = v1;
                    h523
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f201(p528: List<u64>, p529: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m527 = (p528, p529);
    {
        let mut m527 = m527;
        loop {
            let step = {
                let (v0, v1) = m527;
                let m524 = v0;
                match m524.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a525 = v2;
                        let a526 = 93u64;
                        if nat_eq(a525, a526) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f284(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f202(p534: List<u64>, p535: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m533 = (p534, p535);
    {
        let mut m533 = m533;
        loop {
            let step = {
                let (v0, v1) = m533;
                let m530 = v0;
                match m530.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f284(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a531 = v2;
                        let a532 = 58u64;
                        if nat_eq(a531, a532) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f284(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Str, p540: List<Str>) -> bool {
    let m539 = p540;
    {
        let mut m539 = m539;
        loop {
            let step = {
                let v1 = m539;
                let m536 = v1;
                match m536.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a537 = v0.clone();
                        let a538 = v2;
                        if equal(a537, a538) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Str, v1: List<u64>) -> Option<Adt97> {
    if f285(v1) {
        Some(Adt97::C0(v0))
    } else {
        None::<Adt97>
    }
}

pub fn f205(v0: Str) -> R<List<u64>> {
    f177({
        let a541 = v0;
        utf8_encode(a541)
    })
}

pub fn f206(v0: Adt36) -> R<Result<List<Adt12>, Adt97>> {
    let m542 = f286(v0.clone());
    match m542 {
        None => {
            f287(v0.clone(), f288(f289(), match v0.clone() {
                Adt36::C0(h543, _, _, _, _, _, _, _) => {
                    h543
                }
            }, Adt6::C5({
                let m546 = match v0.clone() {
                    Adt36::C0(_, _, _, _, h545, _, _, _) => {
                        h545
                    }
                };
                match m546 {
                    Adt5::C0(h544) => {
                        h544
                    }
                }
            }), List::cons(Adt109::C0(Adt6::C5({
                let m549 = match v0 {
                    Adt36::C0(_, _, _, _, h548, _, _, _) => {
                        h548
                    }
                };
                match m549 {
                    Adt5::C0(h547) => {
                        h547
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt109>::nil()), f290())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f207(p554: u64, p555: i64) -> R<i64> {
    let m553 = (p554, p555);
    {
        let mut m553 = m553;
        loop {
            let step = {
                let (v0, v1) = m553;
                let m550 = v0;
                if m550 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m550 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a551 = v1;
                        let a552 = -1i64;
                        int_sub(a551, a552)?
                    }))
                }
            };
            match step {
                Ok(next) => { m553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Adt14, v1: Adt36, v2: Adt22, v3: Adt98) -> R<Result<Adt33, Adt90>> {
    let m556 = v3;
    match m556 {
        Adt98::C0(v4) => {
            f291(v1, v4)
        }
        Adt98::C1(v6, v7, v8) => {
            if f292(v2) {
                f293(v0, v8)
            } else {
                f294(v6, v1, v7)
            }
        }
        Adt98::C2(v5) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C12(v5)))
        }
    }
}

pub fn f209(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt110> {
    f295(v0, f296(), Adt112::C0, List::cons(Adt113::C0(v1, v2, List::<Adt111>::nil()), List::<Adt113>::nil()))
}

pub fn f210(v0: Adt22) -> Adt6 {
    let m558 = match v0 {
        Adt22::C0(_, _, h557, _, _) => {
            h557
        }
    };
    match m558 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h559) => {
                    h559
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
    let v2: Str = f297(v0)?;
    let a562 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a560 = v2.clone();
        let a561 = Str::lit("");
        if equal(a560, a561) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a563 = Str::lit(" ");
    Ok(join(a562, a563))
}

pub fn f213(v0: Adt36, v1: Fn2, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m565 = {
        let c564: Fn2 = v1;
        c564.apply(v2)?
    };
    match m565 {
        Ok(v4) => {
            f291(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f214(v0: Str, v1: Adt5) -> Adt107 {
    f298(v0, match v1 {
        Adt5::C0(h566) => {
            h566
        }
    })
}

pub fn f215(v0: Adt6) -> Option<i64> {
    let m567 = v0;
    match m567 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f299(match v2.clone() {
                Adt7::C0(_, h568) => {
                    h568
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h569, _) => {
                        h569
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
    let m570 = v0;
    match m570 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m571 = f300(Adt6::C5(v1))?;
            match m571 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m572 = f219(v2);
                    match m572 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h573) => {
                                    h573
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
    let m575 = f301(match v0.clone() {
        Adt38::C0(h574, _, _) => {
            h574
        }
    })?;
    match m575 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a578 = f302(Str::lit("structuredContent"), match v0.clone() {
                    Adt38::C0(_, h576, _) => {
                        h576
                    }
                });
                let a579 = f303(Str::lit("isError"), match v0 {
                    Adt38::C0(_, _, h577) => {
                        h577
                    }
                });
                append_list(a578, a579)
            })))
        }
    }
}

pub fn f218() -> Adt87 {
    Adt87::C1(Adt81::C0(-32603i64, Str::lit("Internal error"), None::<Adt6>))
}

pub fn f219(v0: Adt6) -> Option<Adt5> {
    let m580 = v0;
    match m580 {
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

pub fn f221(p583: u64, p584: i64) -> R<i64> {
    let m582 = (p583, p584);
    {
        let mut m582 = m582;
        loop {
            let step = {
                let (v0, v1) = m582;
                let m581 = v0;
                if m581 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m581 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f304(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: Option<Adt5>, v1: Str) -> bool {
    let m585 = v0;
    match m585 {
        None => {
            false
        }
        Some(v2) => {
            f305(v1, v2)
        }
    }
}

pub fn f223(v0: Adt6) -> Option<List<Adt6>> {
    let m586 = v0;
    match m586 {
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

pub fn f224(p590: List<Adt6>, p591: List<Adt85>) -> Option<List<Adt85>> {
    let m589 = (p590, p591);
    {
        let mut m589 = m589;
        loop {
            let step = {
                let (v0, v1) = m589;
                let m587 = v0;
                match m587.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(Some(f306(v1, List::<Adt85>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m588 = f307(v2);
                        match m588 {
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
                Ok(next) => { m589 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: Adt85) -> Str {
    let m594 = f308(match v0 {
        Adt85::C0(h593, _, _) => {
            h593
        }
    });
    match m594 {
        Adt64::C0(h592, _, _, _, _, _) => {
            h592
        }
    }
}

pub fn f226(v0: Adt85, v1: i64) -> R<bool> {
    let m598 = {
        let m597 = f308(match v0.clone() {
            Adt85::C0(h596, _, _) => {
                h596
            }
        });
        match m597 {
            Adt64::C0(_, _, _, _, h595, _) => {
                h595
            }
        }
    };
    match m598 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a605 = {
                let m604 = {
                    let a602 = {
                        let a600 = v1;
                        let a601 = match v0 {
                            Adt85::C0(_, h599, _) => {
                                h599
                            }
                        };
                        int_sub(a600, a601)?
                    };
                    let a603 = v2;
                    compare(a602, a603)
                };
                match m604 {
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
            Ok(bool_not(a605))
        }
    }
}

pub fn f227() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f28(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f228(v0: Adt85, v1: Adt64, v2: i64, v3: Str) -> R<Adt101> {
    let m609 = {
        let m608 = match v0.clone() {
            Adt85::C0(_, _, h607) => {
                h607
            }
        };
        match m608 {
            Adt86::C0(_, h606, _, _) => {
                h606
            }
        }
    };
    match m609.uncons() {
        None => {
            if f309({
                let m612 = match v0.clone() {
                    Adt85::C0(_, _, h611) => {
                        h611
                    }
                };
                match m612 {
                    Adt86::C0(_, _, h610, _) => {
                        h610
                    }
                }
            }, match v0.clone() {
                Adt85::C0(_, h613, _) => {
                    h613
                }
            }, v2)? {
                Ok(f310(v0.clone(), f311({
                    let m616 = match v0.clone() {
                        Adt85::C0(_, _, h615) => {
                            h615
                        }
                    };
                    match m616 {
                        Adt86::C0(_, _, _, h614) => {
                            h614
                        }
                    }
                }, f312(v1, v3)), match v0 {
                    Adt85::C0(_, _, h617) => {
                        h617
                    }
                }))
            } else {
                Ok(f229(v0))
            }
        }
        Some((_, _)) => {
            if f309({
                let m620 = match v0.clone() {
                    Adt85::C0(_, _, h619) => {
                        h619
                    }
                };
                match m620 {
                    Adt86::C0(h618, _, _, _) => {
                        h618
                    }
                }
            }, match v0.clone() {
                Adt85::C0(_, h621, _) => {
                    h621
                }
            }, v2)? {
                Ok(f310(v0.clone(), Adt63::C1(f312(v1, v3), {
                    let m624 = match v0.clone() {
                        Adt85::C0(_, _, h623) => {
                            h623
                        }
                    };
                    match m624 {
                        Adt86::C0(_, h622, _, _) => {
                            h622
                        }
                    }
                }), f313(match v0 {
                    Adt85::C0(_, _, h625) => {
                        h625
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
    let m626 = f314(v2.clone(), v3);
    match m626.uncons() {
        None => {
            f315(v0, v1, v4, v5)
        }
        Some((v6, v7)) => {
            let a629 = {
                let a627 = List::cons(v6.clone(), v7.clone());
                length_list(a627)
            };
            let a630 = {
                let a628 = v2;
                length_list(a628)
            };
            if nat_eq(a629, a630) {
                Ok(f229(v0))
            } else {
                Ok(f310(v0.clone(), Adt63::C1(f312(v1, v5), List::cons(v6, v7)), match v0 {
                    Adt85::C0(_, _, h631) => {
                        h631
                    }
                }))
            }
        }
    }
}

pub fn f231(v0: Adt101, v1: Str) -> Adt101 {
    let m635 = {
        let m634 = match v0.clone() {
            Adt101::C0(h633, _) => {
                h633
            }
        };
        match m634 {
            Adt85::C0(h632, _, _) => {
                h632
            }
        }
    };
    match m635 {
        Adt63::C0(v2) => {
            f310(match v0.clone() {
                Adt101::C0(h636, _) => {
                    h636
                }
            }, Adt63::C4(f312(v2, v1)), {
                let m639 = match v0 {
                    Adt101::C0(h638, _) => {
                        h638
                    }
                };
                match m639 {
                    Adt85::C0(_, _, h637) => {
                        h637
                    }
                }
            })
        }
        Adt63::C1(v3, _) => {
            f310(match v0.clone() {
                Adt101::C0(h640, _) => {
                    h640
                }
            }, Adt63::C4(f312(v3, v1)), {
                let m643 = match v0 {
                    Adt101::C0(h642, _) => {
                        h642
                    }
                };
                match m643 {
                    Adt85::C0(_, _, h641) => {
                        h641
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
    f316(f317(v0, List::<Adt45>::nil()), List::<Adt45>::nil())
}

pub fn f233() -> List<(Adt45, List<Adt47>)> {
    List::cons(f318(f319(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f247(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f318(f319(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f320(Str::lit("test://static-binary"), Str::lit("image/png"), f248())), List::cons(f318(f319(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f247(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f318(f319(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f247(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt45, List<Adt47>)>::nil()))))
}

pub fn f234(v0: Adt10, v1: Str, v2: Option<List<Adt47>>) -> Result<Adt33, Adt90> {
    let m644 = v2;
    match m644 {
        None => {
            Err::<Adt33, Adt90>(Adt90::C2(v1))
        }
        Some(v3) => {
            let m645 = v3.clone();
            match m645.uncons() {
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

pub fn f235(v0: Str, p654: List<(Adt45, List<Adt47>)>) -> Option<List<Adt47>> {
    let m653 = p654;
    {
        let mut m653 = m653;
        loop {
            let step = {
                let v1 = m653;
                let m646 = v1;
                match m646.uncons() {
                    None => {
                        Err::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(None::<List<Adt47>>)
                    }
                    Some((v2, v3)) => {
                        let a650 = {
                            let m649 = {
                                let (h648, _) = v2;
                                h648
                            };
                            match m649 {
                                Adt45::C0(_, _, _, h647, _, _, _, _, _) => {
                                    h647
                                }
                            }
                        };
                        let a651 = v0.clone();
                        if equal(a650, a651) {
                            Err::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(Some({
                                let (_, h652) = v2;
                                h652
                            }))
                        } else {
                            Ok::<List<(Adt45, List<Adt47>)>, Option<List<Adt47>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m653 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Str) -> R<Option<List<Adt47>>> {
    let m655 = f321(v0.clone())?;
    match m655 {
        None => {
            Ok(None::<List<Adt47>>)
        }
        Some(v1) => {
            let m656 = f322(v1)?;
            match m656 {
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

pub fn f240(v0: Str, p662: List<Adt57>) -> Option<Adt57> {
    let m661 = p662;
    {
        let mut m661 = m661;
        loop {
            let step = {
                let v1 = m661;
                let m657 = v1;
                match m657.uncons() {
                    None => {
                        Err::<List<Adt57>, Option<Adt57>>(None::<Adt57>)
                    }
                    Some((v2, v3)) => {
                        let a659 = match v2.clone() {
                            Adt57::C0(h658, _, _, _, _, _) => {
                                h658
                            }
                        };
                        let a660 = v0.clone();
                        if equal(a659, a660) {
                            Err::<List<Adt57>, Option<Adt57>>(Some(v2))
                        } else {
                            Ok::<List<Adt57>, Option<Adt57>>(v3)
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

pub fn f241(p668: List<Adt58>, v1: List<(Str, Str)>) -> Option<Str> {
    let m667 = p668;
    {
        let mut m667 = m667;
        loop {
            let step = {
                let v0 = m667;
                let m663 = v0;
                match m663.uncons() {
                    None => {
                        Err::<List<Adt58>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f323(v2.clone()) {
                            let m665 = f324(match v2.clone() {
                                Adt58::C0(h664, _, _, _) => {
                                    h664
                                }
                            }, v1.clone());
                            match m665 {
                                None => {
                                    Err::<List<Adt58>, Option<Str>>(Some(match v2 {
                                        Adt58::C0(h666, _, _, _) => {
                                            h666
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
                Ok(next) => { m667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt57) -> List<Adt58> {
    let m670 = match v0 {
        Adt57::C0(_, _, _, _, h669, _) => {
            h669
        }
    };
    match m670 {
        None => {
            List::<Adt58>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f243(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m671 = v0;
    match m671 {
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
    let m672 = f324(v0, v1);
    match m672 {
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
    let v1: u64 = f325(v0.clone(), 0u64)?;
    Ok(Adt61::C0(Adt62::C0(f326(v0), Some(v1), Some({
        let a673 = f327();
        let a674 = v1;
        nat_lt(a673, a674)
    }))))
}

pub fn f250(p680: List<Adt58>, v1: Str) -> bool {
    let m679 = p680;
    {
        let mut m679 = m679;
        loop {
            let step = {
                let v0 = m679;
                let m675 = v0;
                match m675.uncons() {
                    None => {
                        Err::<List<Adt58>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a677 = match v2 {
                            Adt58::C0(h676, _, _, _) => {
                                h676
                            }
                        };
                        let a678 = v1.clone();
                        if equal(a677, a678) {
                            Err::<List<Adt58>, bool>(true)
                        } else {
                            Ok::<List<Adt58>, bool>(v3)
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

pub fn f251(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f328(f329(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f252(v0: Str, v1: Str) -> List<Str> {
    let a681 = v0;
    let a682 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a681, a682) {
        let a683 = v1;
        let a684 = Str::lit("resourceUri");
        equal(a683, a684)
    } else {
        false
    } {
        f330()
    } else {
        List::<Str>::nil()
    }
}

pub fn f253() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f254(v0: Bytes, p687: u64, p688: List<u64>) -> R<List<u64>> {
    let m686 = (p687, p688);
    {
        let mut m686 = m686;
        loop {
            let step = {
                let (v1, v2) = m686;
                let m685 = v1;
                if m685 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m685 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f108(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: Bytes, v1: Adt91, v2: u64) -> R<Result<Adt91, Adt93>> {
    let a690 = v2;
    let a691 = {
        let a689 = v0.clone();
        length_bytes(a689)
    };
    if nat_eq(a690, a691) {
        Ok(Ok::<Adt91, Adt93>(v1))
    } else {
        let m692 = f40(v0.clone(), v2)?;
        match m692 {
            None => {
                Ok(Err::<Adt91, Adt93>(Adt93::C0(v2)))
            }
            Some(v3) => {
                let m695 = f331(v0.clone(), {
                    let a693 = v2;
                    let a694 = 4u64;
                    nat_add(a693, a694)?
                }, v3, List::<List<u64>>::nil())?;
                match m695 {
                    Ok(v5) => {
                        let a698 = {
                            let (_, h696) = v5.clone();
                            h696
                        };
                        let a699 = {
                            let a697 = v0;
                            length_bytes(a697)
                        };
                        if nat_eq(a698, a699) {
                            Ok(Ok::<Adt91, Adt93>(f332(v1, {
                                let (h700, _) = v5;
                                h700
                            })))
                        } else {
                            Ok(Err::<Adt91, Adt93>(Adt93::C1({
                                let (_, h701) = v5;
                                h701
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

pub fn f256(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a705 = {
        let a702 = v1;
        let a703 = 8u64;
        nat_add(a702, a703)?
    };
    let a706 = {
        let a704 = v0.clone();
        length_bytes(a704)
    };
    if nat_le(a705, a706) {
        Ok(Some(f333(f333(f333(f333(f333(f333(f333(f333(0i64, f108(v0.clone(), v1)?)?, f108(v0.clone(), {
            let a707 = v1;
            let a708 = 1u64;
            nat_add(a707, a708)?
        })?)?, f108(v0.clone(), {
            let a709 = v1;
            let a710 = 2u64;
            nat_add(a709, a710)?
        })?)?, f108(v0.clone(), {
            let a711 = v1;
            let a712 = 3u64;
            nat_add(a711, a712)?
        })?)?, f108(v0.clone(), {
            let a713 = v1;
            let a714 = 4u64;
            nat_add(a713, a714)?
        })?)?, f108(v0.clone(), {
            let a715 = v1;
            let a716 = 5u64;
            nat_add(a715, a716)?
        })?)?, f108(v0.clone(), {
            let a717 = v1;
            let a718 = 6u64;
            nat_add(a717, a718)?
        })?)?, f108(v0, {
            let a719 = v1;
            let a720 = 7u64;
            nat_add(a719, a720)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f257(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt93>> {
    let m721 = f16(v0, v1)?;
    match m721 {
        None => {
            Ok(Err::<(Str, u64), Adt93>(Adt93::C0(v1)))
        }
        Some(v2) => {
            let m723 = f334(f177(match v2.clone() {
                Adt95::C0(h722, _) => {
                    h722
                }
            })?);
            match m723 {
                None => {
                    Ok(Err::<(Str, u64), Adt93>(Adt93::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt93>((v3, match v2 {
                        Adt95::C0(_, h724) => {
                            h724
                        }
                    })))
                }
            }
        }
    }
}

pub fn f258(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt14, u64), Adt93>> {
    let m726 = f257(v0.clone(), {
        let (_, h725) = v2;
        h725
    })?;
    match m726 {
        Ok(v4) => {
            f335(v0, v1, {
                let (h727, _) = v2;
                h727
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt14, u64), Adt93>(v3))
        }
    }
}

pub fn f259(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt102>> {
    let m728 = f336(v0, List::cons(Adt115::C1(0u64, v1), List::<Adt115>::nil()), List::<u64>::nil())?;
    match m728 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt102>(f337(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt102>(v3))
        }
    }
}

pub fn f260() -> u64 {
    67108864u64
}

pub fn f261() -> Str {
    Str::lit("2.0")
}

pub fn f262(v0: Option<Adt100>) -> List<(Str, Adt6)> {
    let m729 = v0;
    match m729 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f338(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f263(v0: Adt81) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt81::C0(h730, _, _) => {
            h730
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt81::C0(_, h731, _) => {
            h731
        }
    })), f339(match v0 {
        Adt81::C0(_, _, h732) => {
            h732
        }
    }))))
}

pub fn f264(v0: Adt103) -> i64 {
    let m733 = v0;
    match m733 {
        Adt103::C0 => {
            f340()
        }
        Adt103::C1(_, _) => {
            f341()
        }
    }
}

pub fn f265(v0: Adt103) -> Str {
    let m734 = v0;
    match m734 {
        Adt103::C0 => {
            Str::lit("Parse error")
        }
        Adt103::C1(_, v2) => {
            f342(v2)
        }
    }
}

pub fn f266(v0: Adt103) -> u64 {
    let m735 = v0;
    match m735 {
        Adt103::C0 => {
            f188()
        }
        Adt103::C1(_, _) => {
            f188()
        }
    }
}

pub fn f267(v0: Adt103) -> Option<Adt100> {
    let m736 = v0;
    match m736 {
        Adt103::C0 => {
            None::<Adt100>
        }
        Adt103::C1(v1, _) => {
            v1
        }
    }
}

pub fn f268(v0: Adt14, v1: Adt0, v2: Fn3, v3: u64, v4: List<Adt92>, v5: Adt108) -> R<Adt94> {
    f343(v0, f344(v4.clone())?, f345(v1.clone(), v2, v3, Some(f346(v4, match v1 {
        Adt0::C0(_, _, _, _, h737, _) => {
            h737
        }
    }, f347(v5.clone()), match v5.clone() {
        Adt108::C0(_, h738, _) => {
            h738
        }
    })?), f348(), v5)?)
}

pub fn f269(v0: Adt5) -> R<Result<Adt108, Adt103>> {
    let m739 = f349(v0.clone())?;
    match m739 {
        Adt140::C0 => {
            let m740 = f214(Str::lit("method"), v0.clone());
            match m740 {
                Adt107::C0 => {
                    Ok(f270(None::<Adt100>, f350(v0)))
                }
                Adt107::C1(_) => {
                    Ok(f270(None::<Adt100>, if f351(v0) {
                        Adt104::C8
                    } else {
                        Adt104::C7
                    }))
                }
            }
        }
        Adt140::C1(v2) => {
            Ok(f352(v2, v0))
        }
        Adt140::C2 => {
            Ok(f270(None::<Adt100>, Adt104::C5))
        }
    }
}

pub fn f270(v0: Option<Adt100>, v1: Adt104) -> Result<Adt108, Adt103> {
    Err::<Adt108, Adt103>(Adt103::C1(v0, v1))
}

pub fn f271(v0: List<u64>) -> R<Adt121> {
    Ok(f353({
        let a741 = v0.clone();
        length_list(a741)
    }, f354(Adt122::C0, 0u64, 0u64, v0)?))
}

pub fn f272(p748: Adt123, p749: u64, p750: List<u64>) -> R<Result<Adt6, Adt102>> {
    let m747 = (p748, (p749, p750));
    {
        let mut m747 = m747;
        loop {
            let step = {
                let (v0, (v1, v2)) = m747;
                let m742 = v2;
                match m742.uncons() {
                    None => {
                        let m743 = f355(v0, v1)?;
                        match m743 {
                            Ok(v3) => {
                                Err::<(Adt123, (u64, List<u64>)), R<Result<Adt6, Adt102>>>(Ok(Ok::<Adt6, Adt102>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt123, (u64, List<u64>)), R<Result<Adt6, Adt102>>>(Ok(Err::<Adt6, Adt102>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m744 = f356(v0, v1, v5)?;
                        match m744 {
                            Adt141::C0(v7) => {
                                Err::<(Adt123, (u64, List<u64>)), R<Result<Adt6, Adt102>>>(Ok(Err::<Adt6, Adt102>(v7)))
                            }
                            Adt141::C1(v8) => {
                                Ok::<(Adt123, (u64, List<u64>)), R<Result<Adt6, Adt102>>>((v8, ({
                                    let a745 = v1;
                                    let a746 = 1u64;
                                    nat_add(a745, a746)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m747 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273() -> Adt123 {
    Adt123::C0(Adt124::C0, List::<Adt128>::nil(), 0u64)
}

pub fn f274(p757: List<u64>, p758: List<u64>) -> R<bool> {
    let m756 = (p757, p758);
    {
        let mut m756 = m756;
        loop {
            let step = {
                let (v0, v1) = m756;
                let m751 = v0;
                match m751.uncons() {
                    None => {
                        let m752 = v1;
                        match m752.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m753 = v1;
                        match m753.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a754 = f357(v4)?;
                                let a755 = f357(v6)?;
                                if nat_eq(a754, a755) {
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
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Adt106, v1: List<u64>) -> Adt106 {
    let m759 = v0;
    match m759 {
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

pub fn f276(v0: List<u64>) -> List<u64> {
    f284(f358(f284(f358(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f277(p762: List<u64>) -> bool {
    let m761 = p762;
    {
        let mut m761 = m761;
        loop {
            let step = {
                let v0 = m761;
                let m760 = v0;
                match m760.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f359(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f279() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f280() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f281(v0: List<u64>, p765: List<List<u64>>) -> R<bool> {
    let m764 = p765;
    {
        let mut m764 = m764;
        loop {
            let step = {
                let v1 = m764;
                let m763 = v1;
                match m763.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f274(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m764 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: List<u64>) -> Option<List<u64>> {
    let m766 = v0;
    match m766.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m767 = v2;
            match m767.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a768 = v1;
                    let a769 = 47u64;
                    if if nat_eq(a768, a769) {
                        let a770 = v3;
                        let a771 = 47u64;
                        nat_eq(a770, a771)
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

pub fn f283(v0: List<u64>) -> bool {
    let m772 = v0;
    match m772.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a773 = v1;
            let a774 = 58u64;
            if nat_eq(a773, a774) {
                let m775 = v2.clone();
                match m775.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f360(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f284(p778: List<u64>, p779: List<u64>) -> List<u64> {
    let m777 = (p778, p779);
    {
        let mut m777 = m777;
        loop {
            let step = {
                let (v0, v1) = m777;
                let m776 = v0;
                match m776.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f285(v0: List<u64>) -> bool {
    let m780 = v0.clone();
    match m780.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a784 = if f361(v0.clone(), false) {
                let a782 = {
                    let a781 = v0;
                    length_list(a781)
                };
                let a783 = f362();
                nat_le(a782, a783)
            } else {
                false
            };
            bool_not(a784)
        }
    }
}

pub fn f286(v0: Adt36) -> Option<Adt97> {
    let m789 = f363(match v0.clone() {
        Adt36::C0(h785, _, _, _, _, _, _, _) => {
            h785
        }
    }, {
        let m788 = match v0.clone() {
            Adt36::C0(_, _, _, _, h787, _, _, _) => {
                h787
            }
        };
        match m788 {
            Adt5::C0(h786) => {
                h786
            }
        }
    });
    match m789 {
        None => {
            f364(match v0.clone() {
                Adt36::C0(h790, _, _, _, _, _, _, _) => {
                    h790
                }
            }, {
                let m793 = match v0 {
                    Adt36::C0(_, _, _, _, h792, _, _, _) => {
                        h792
                    }
                };
                match m793 {
                    Adt5::C0(h791) => {
                        h791
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f287(v0: Adt36, v1: Result<Adt129, Adt97>) -> R<Result<List<Adt12>, Adt97>> {
    let m794 = v1;
    match m794 {
        Ok(v3) => {
            let m795 = f365(v0)?;
            match m795 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt97>(f129(match v3 {
                        Adt129::C0(h796, _) => {
                            h796
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

pub fn f288(p803: u64, v1: Str, v2: Adt6, p804: List<Adt109>, p805: Adt129) -> R<Result<Adt129, Adt97>> {
    let m802 = (p803, (p804, p805));
    {
        let mut m802 = m802;
        loop {
            let step = {
                let (v0, (v3, v4)) = m802;
                let m797 = v0;
                if m797 == 0 {
                    Err::<(u64, (List<Adt109>, Adt129)), R<Result<Adt129, Adt97>>>(Ok(Err::<Adt129, Adt97>(Adt97::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m797 - 1;
                    let m798 = v3;
                    match m798.uncons() {
                        None => {
                            Err::<(u64, (List<Adt109>, Adt129)), R<Result<Adt129, Adt97>>>(Ok(Ok::<Adt129, Adt97>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m799 = f366(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m799 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt109>, Adt129)), R<Result<Adt129, Adt97>>>((v5, ({
                                        let (h800, _) = v9;
                                        h800
                                    }, {
                                        let (_, h801) = v9;
                                        h801
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
                Ok(next) => { m802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289() -> u64 {
    67108864u64
}

pub fn f290() -> Adt129 {
    Adt129::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f291(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f367({
        let m808 = match v1.clone() {
            Adt50::C0(h807, _) => {
                h807
            }
        };
        match m808 {
            Adt38::C0(h806, _, _) => {
                h806
            }
        }
    })? {
        f368(v0, v1)
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f292(v0: Adt22) -> bool {
    f153({
        let m811 = match v0 {
            Adt22::C0(h810, _, _, _, _) => {
                h810
            }
        };
        match m811 {
            Adt17::C0(_, h809, _, _, _, _) => {
                h809
            }
        }
    })
}

pub fn f293(v0: Adt14, v1: Adt86) -> R<Result<Adt33, Adt90>> {
    let a813 = match v0.clone() {
        Adt14::C0(_, _, h812, _) => {
            h812
        }
    };
    let a814 = Str::lit("");
    if equal(a813, a814) {
        Ok(Err::<Adt33, Adt90>(f369()))
    } else {
        let m816 = f154(match v0.clone() {
            Adt14::C0(_, _, _, h815) => {
                h815
            }
        })?;
        match m816 {
            None => {
                Ok(Err::<Adt33, Adt90>(f155()))
            }
            Some(_) => {
                Ok(Ok::<Adt33, Adt90>(Adt33::C13(Adt83::C0(Adt84::C0, f308({
                    let m818 = f370(v0.clone(), v1.clone());
                    match m818 {
                        Adt85::C0(h817, _, _) => {
                            h817
                        }
                    }
                }), f370(v0, v1)))))
            }
        }
    }
}

pub fn f294(v0: Adt99, v1: Adt36, v2: Adt50) -> R<Result<Adt33, Adt90>> {
    let m819 = v0;
    match m819 {
        Adt99::C0 => {
            f291(v1, v2)
        }
        Adt99::C1 => {
            Ok(Err::<Adt33, Adt90>(f157()))
        }
    }
}

pub fn f295(v0: Adt6, p838: u64, p839: Adt112, p840: List<Adt113>) -> R<Adt110> {
    let m837 = (p838, (p839, p840));
    {
        let mut m837 = m837;
        loop {
            let step = {
                let (v1, (v2, v3)) = m837;
                let m820 = v1;
                if m820 == 0 {
                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C4))
                } else {
                    let v4 = m820 - 1;
                    let m821 = v3;
                    match m821.uncons() {
                        None => {
                            let m822 = v2;
                            match m822 {
                                Adt112::C0 => {
                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C0))
                                }
                                Adt112::C1(v5, v6) => {
                                    Err::<(u64, (Adt112, List<Adt113>)), R<Adt110>>(Ok(Adt110::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m823 = v2.clone();
                            match m823 {
                                Adt112::C0 => {
                                    let m824 = v7.clone();
                                    match m824 {
                                        Adt113::C0(v9, v10, v11) => {
                                            let m825 = f371(v0.clone(), v9, v10, v11.clone())?;
                                            match m825 {
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
                                                        let a826 = v15;
                                                        let a827 = v8;
                                                        append_list(a826, a827)
                                                    };
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt113::C1(_, _, _) => {
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v8)))
                                        }
                                        Adt113::C2(_, _, _, _) => {
                                            let v24: Adt130 = f372(v7, v8)?;
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v24.clone() {
                                                Adt130::C0(h828, _) => {
                                                    h828
                                                }
                                            }, match v24 {
                                                Adt130::C0(_, h829) => {
                                                    h829
                                                }
                                            })))
                                        }
                                        Adt113::C3(v25) => {
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (Adt112::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt113::C4(_, _, _, _) => {
                                            let v39: Adt130 = f372(v7, v8)?;
                                            Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v39.clone() {
                                                Adt130::C0(h833, _) => {
                                                    h833
                                                }
                                            }, match v39 {
                                                Adt130::C0(_, h834) => {
                                                    h834
                                                }
                                            })))
                                        }
                                        Adt113::C5(v26, v27, v28, v29) => {
                                            let m830 = f373(v26, v27, v28, v29.clone());
                                            match m830 {
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
                                                        let a831 = v33;
                                                        let a832 = v8;
                                                        append_list(a831, a832)
                                                    };
                                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt112::C1(v40, v41) => {
                                    let v42: Adt130 = f374(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt112, List<Adt113>)), R<Adt110>>((v4, (match v42.clone() {
                                        Adt130::C0(h835, _) => {
                                            h835
                                        }
                                    }, match v42 {
                                        Adt130::C0(_, h836) => {
                                            h836
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m837 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296() -> u64 {
    67108864u64
}

pub fn f297(v0: List<Adt111>) -> R<Str> {
    let a841 = List::cons(Str::lit(""), f375(v0, List::<Str>::nil())?);
    let a842 = Str::lit("/");
    Ok(join(a841, a842))
}

pub fn f298(v0: Str, p848: List<(Str, Adt6)>) -> Adt107 {
    let m847 = p848;
    {
        let mut m847 = m847;
        loop {
            let step = {
                let v1 = m847;
                let m843 = v1;
                match m843.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt107>(Adt107::C0)
                    }
                    Some((v2, v3)) => {
                        let m845 = f376({
                            let (h844, _) = v2;
                            h844
                        }, v0.clone());
                        match m845 {
                            Adt143::C0 => {
                                Err::<List<(Str, Adt6)>, Adt107>(Adt107::C1({
                                    let (_, h846) = v2;
                                    h846
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
                Ok(next) => { m847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: i64) -> bool {
    let a855 = {
        let m851 = {
            let a849 = v0;
            let a850 = 0i64;
            compare(a849, a850)
        };
        if match m851 {
            Ordering::Less => {
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
            let m854 = {
                let a852 = 0i64;
                let a853 = v0;
                compare(a852, a853)
            };
            match m854 {
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
    bool_not(a855)
}

pub fn f300(v0: Adt6) -> R<Option<Adt6>> {
    f377(f378(), Adt131::C0(v0), List::<Adt132>::nil())
}

pub fn f301(v0: List<Adt39>) -> R<Option<List<Adt6>>> {
    let m856 = f379(v0, List::<Adt6>::nil())?;
    match m856 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f380(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f302(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m857 = v1;
    match m857 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f303(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m858 = v1;
    match m858 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f304(v0: i64, v1: i64) -> R<i64> {
    let a860 = v0;
    let a861 = {
        let a859 = v1;
        int_neg(a859)?
    };
    int_sub(a860, a861)
}

pub fn f305(v0: Str, v1: Adt5) -> bool {
    f381(f214(v0, v1))
}

pub fn f306(p864: List<Adt85>, p865: List<Adt85>) -> List<Adt85> {
    let m863 = (p864, p865);
    {
        let mut m863 = m863;
        loop {
            let step = {
                let (v0, v1) = m863;
                let m862 = v0;
                match m862.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), List<Adt85>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt85>, List<Adt85>), List<Adt85>>((v3, List::cons(v2, v1)))
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

pub fn f307(v0: Adt6) -> Option<Adt85> {
    let m866 = f219(v0);
    match m866 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m867 = f382(Str::lit("task"), v1.clone());
            match m867 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m868 = f383(v2);
                    match m868 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m869 = f384(Str::lit("createdMs"), v1.clone());
                            match m869 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m870 = f382(Str::lit("plan"), v1);
                                    match m870 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            let m871 = f385(v5);
                                            match m871 {
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

pub fn f308(v0: Adt63) -> Adt64 {
    let m872 = v0;
    match m872 {
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

pub fn f309(v0: i64, v1: i64, v2: i64) -> R<bool> {
    let a878 = {
        let m877 = {
            let a875 = {
                let a873 = v2;
                let a874 = v1;
                int_sub(a873, a874)?
            };
            let a876 = v0;
            compare(a875, a876)
        };
        match m877 {
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
    Ok(bool_not(a878))
}

pub fn f310(v0: Adt85, v1: Adt63, v2: Adt86) -> Adt101 {
    Adt101::C0(Adt85::C0(v1, match v0 {
        Adt85::C0(_, h879, _) => {
            h879
        }
    }, v2), true)
}

pub fn f311(v0: Adt87, v1: Adt64) -> Adt63 {
    let m880 = v0;
    match m880 {
        Adt87::C0(v2) => {
            Adt63::C2(v1, v2)
        }
        Adt87::C1(v3) => {
            Adt63::C3(v1, v3)
        }
    }
}

pub fn f312(v0: Adt64, v1: Str) -> Adt64 {
    Adt64::C0(match v0.clone() {
        Adt64::C0(h881, _, _, _, _, _) => {
            h881
        }
    }, match v0.clone() {
        Adt64::C0(_, h882, _, _, _, _) => {
            h882
        }
    }, match v0.clone() {
        Adt64::C0(_, _, h883, _, _, _) => {
            h883
        }
    }, v1, match v0.clone() {
        Adt64::C0(_, _, _, _, h884, _) => {
            h884
        }
    }, match v0 {
        Adt64::C0(_, _, _, _, _, h885) => {
            h885
        }
    })
}

pub fn f313(v0: Adt86) -> Adt86 {
    Adt86::C0(match v0.clone() {
        Adt86::C0(h886, _, _, _) => {
            h886
        }
    }, List::<(Str, Adt65)>::nil(), match v0.clone() {
        Adt86::C0(_, _, h887, _) => {
            h887
        }
    }, match v0 {
        Adt86::C0(_, _, _, h888) => {
            h888
        }
    })
}

pub fn f314(v0: List<(Str, Adt65)>, v1: Adt5) -> List<(Str, Adt65)> {
    f386(f387(v0, v1, List::<(Str, Adt65)>::nil()), List::<(Str, Adt65)>::nil())
}

pub fn f315(v0: Adt85, v1: Adt64, v2: i64, v3: Str) -> R<Adt101> {
    let v4: Adt85 = Adt85::C0(Adt63::C0(f312(v1, v3.clone())), match v0.clone() {
        Adt85::C0(_, h889, _) => {
            h889
        }
    }, match v0 {
        Adt85::C0(_, _, h890) => {
            h890
        }
    });
    Ok(Adt101::C0({
        let m892 = f159(v4, v2, v3)?;
        match m892 {
            Adt101::C0(h891, _) => {
                h891
            }
        }
    }, true))
}

pub fn f316(p895: List<Adt45>, p896: List<Adt45>) -> List<Adt45> {
    let m894 = (p895, p896);
    {
        let mut m894 = m894;
        loop {
            let step = {
                let (v0, v1) = m894;
                let m893 = v0;
                match m893.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt45>), List<Adt45>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt45>), List<Adt45>>((v3, List::cons(v2, v1)))
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

pub fn f317(p900: List<(Adt45, List<Adt47>)>, p901: List<Adt45>) -> List<Adt45> {
    let m899 = (p900, p901);
    {
        let mut m899 = m899;
        loop {
            let step = {
                let (v0, v1) = m899;
                let m897 = v0;
                match m897.uncons() {
                    None => {
                        Err::<(List<(Adt45, List<Adt47>)>, List<Adt45>), List<Adt45>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt45, List<Adt47>)>, List<Adt45>), List<Adt45>>((v3, List::cons({
                            let (h898, _) = v2;
                            h898
                        }, v1)))
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

pub fn f318(v0: Adt45, v1: Adt47) -> (Adt45, List<Adt47>) {
    (v0, List::cons(v1, List::<Adt47>::nil()))
}

pub fn f319(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt45 {
    Adt45::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt41>, None::<u64>, None::<Adt5>)
}

pub fn f320(v0: Str, v1: Str, v2: Str) -> Adt47 {
    Adt47::C1(Adt49::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f321(v0: Str) -> R<Option<Str>> {
    let m902 = f388(v0)?;
    match m902 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a903 = f389(f179(), v1);
            Ok(utf8_decode(a903))
        }
    }
}

pub fn f322(v0: Str) -> R<Option<Str>> {
    let m906 = f390(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a904 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a905 = Str::lit(" ");
        join(a904, a905)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m906 {
        Ok(v1) => {
            let a907 = v1;
            Ok(utf8_decode(a907))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f323(v0: Adt58) -> bool {
    let m909 = match v0 {
        Adt58::C0(_, _, _, h908) => {
            h908
        }
    };
    match m909 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f324(v0: Str, p916: List<(Str, Str)>) -> Option<Str> {
    let m915 = p916;
    {
        let mut m915 = m915;
        loop {
            let step = {
                let v1 = m915;
                let m910 = v1;
                match m910.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a912 = {
                            let (h911, _) = v2;
                            h911
                        };
                        let a913 = v0.clone();
                        if equal(a912, a913) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h914) = v2;
                                h914
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f325(p921: List<Str>, p922: u64) -> R<u64> {
    let m920 = (p921, p922);
    {
        let mut m920 = m920;
        loop {
            let step = {
                let (v0, v1) = m920;
                let m917 = v0;
                match m917.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a918 = v1;
                            let a919 = 1u64;
                            nat_add(a918, a919)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: List<Str>) -> List<Str> {
    f328(f391(v0, f327(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f327() -> u64 {
    100u64
}

pub fn f328(p925: List<Str>, p926: List<Str>) -> List<Str> {
    let m924 = (p925, p926);
    {
        let mut m924 = m924;
        loop {
            let step = {
                let (v0, v1) = m924;
                let m923 = v0;
                match m923.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m924 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(v0: Str, p929: List<Str>, p930: List<Str>) -> R<List<Str>> {
    let m928 = (p929, p930);
    {
        let mut m928 = m928;
        loop {
            let step = {
                let (v1, v2) = m928;
                let m927 = v1;
                match m927.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f392(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m928 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330() -> List<Str> {
    f328(f393(f164(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f331(v0: Bytes, p936: u64, p937: u64, p938: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt93>> {
    let m935 = (p936, (p937, p938));
    {
        let mut m935 = m935;
        loop {
            let step = {
                let (v1, (v2, v3)) = m935;
                let m931 = v2;
                if m931 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>(Ok(Ok::<(List<List<u64>>, u64), Adt93>((f45(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m931 - 1;
                    let m932 = f16(v0.clone(), v1)?;
                    match m932 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>(Ok(Err::<(List<List<u64>>, u64), Adt93>(Adt93::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>((match v5.clone() {
                                Adt95::C0(_, h933) => {
                                    h933
                                }
                            }, (v4, List::cons(f177(match v5 {
                                Adt95::C0(h934, _) => {
                                    h934
                                }
                            })?, v3))))
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

pub fn f332(v0: Adt91, v1: List<List<u64>>) -> Adt91 {
    Adt91::C0(match v0.clone() {
        Adt91::C0(h939, _, _, _, _, _, _) => {
            h939
        }
    }, match v0.clone() {
        Adt91::C0(_, h940, _, _, _, _, _) => {
            h940
        }
    }, match v0.clone() {
        Adt91::C0(_, _, h941, _, _, _, _) => {
            h941
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, h942, _, _, _) => {
            h942
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, _, h943, _, _) => {
            h943
        }
    }, v1, match v0 {
        Adt91::C0(_, _, _, _, _, _, h944) => {
            h944
        }
    })
}

pub fn f333(v0: i64, v1: u64) -> R<i64> {
    f304({
        let a945 = v0;
        let a946 = 256i64;
        int_mul(a945, a946)?
    }, f221(v1, 0i64)?)
}

pub fn f334(v0: List<u64>) -> Option<Str> {
    let a947 = f389(f179(), v0);
    utf8_decode(a947)
}

pub fn f335(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt14, u64), Adt93>> {
    let m949 = f16(v0, {
        let (_, h948) = v3.clone();
        h948
    })?;
    match m949 {
        None => {
            Ok(Err::<(Adt14, u64), Adt93>(Adt93::C0({
                let (_, h950) = v3;
                h950
            })))
        }
        Some(v4) => {
            Ok(f394(v1, v2, {
                let (h951, _) = v3;
                h951
            }, v4))
        }
    }
}

pub fn f336(p956: u64, p957: List<Adt115>, p958: List<u64>) -> R<Result<List<u64>, Adt102>> {
    let m955 = (p956, (p957, p958));
    {
        let mut m955 = m955;
        loop {
            let step = {
                let (v0, (v1, v2)) = m955;
                let m952 = v0;
                if m952 == 0 {
                    Err::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt102>>>(Ok(Err::<List<u64>, Adt102>(Adt102::C13)))
                } else {
                    let v3 = m952 - 1;
                    let m953 = v1;
                    match m953.uncons() {
                        None => {
                            Err::<(u64, (List<Adt115>, List<u64>)), R<Result<List<u64>, Adt102>>>(Ok(Ok::<List<u64>, Adt102>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m954 = f395(v4, v5, v2)?;
                            match m954 {
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
                Ok(next) => { m955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(p961: List<u64>, p962: List<u64>) -> List<u64> {
    let m960 = (p961, p962);
    {
        let mut m960 = m960;
        loop {
            let step = {
                let (v0, v1) = m960;
                let m959 = v0;
                match m959.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m960 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: Adt100) -> Adt6 {
    let m963 = v0;
    match m963 {
        Adt100::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt100::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f339(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m964 = v0;
    match m964 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f340() -> i64 {
    -32700i64
}

pub fn f341() -> i64 {
    -32600i64
}

pub fn f342(v0: Adt104) -> Str {
    let m965 = v0;
    match m965 {
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

pub fn f343(v0: Adt14, v1: bool, v2: Adt133) -> R<Adt94> {
    let m967 = match v2.clone() {
        Adt133::C0(_, h966) => {
            h966
        }
    };
    match m967 {
        Ok(v3) => {
            let m968 = f396(v3.clone())?;
            match m968 {
                None => {
                    Ok(f6())
                }
                Some(v4) => {
                    f397(v0, f398(match v3.clone() {
                        Adt134::C0(h969, _, _) => {
                            h969
                        }
                    }), f399(v1, f113(f400(), f401(match v2 {
                        Adt133::C0(h970, _) => {
                            h970
                        }
                    }, v4))?, f402(f403(match v3.clone() {
                        Adt134::C0(_, _, h971) => {
                            h971
                        }
                    }, List::<List<u64>>::nil())?, f404(f398(match v3 {
                        Adt134::C0(h972, _, _) => {
                            h972
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f113(f405(v5.clone()), f115(Some(match v2 {
                Adt133::C0(h973, _) => {
                    h973
                }
            }), f406(v5)?))
        }
    }
}

pub fn f344(p978: List<Adt92>) -> R<bool> {
    let m977 = p978;
    {
        let mut m977 = m977;
        loop {
            let step = {
                let v0 = m977;
                let m974 = v0;
                match m974.uncons() {
                    None => {
                        Err::<List<Adt92>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f274(match v1.clone() {
                            Adt92::C0(h975, _) => {
                                h975
                            }
                        }, f407())? {
                            if f408(f409(44u64, match v1 {
                                Adt92::C0(_, h976) => {
                                    h976
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
                Ok(next) => { m977 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: Adt0, v1: Fn3, v2: u64, v3: Option<Adt116>, v4: List<Str>, v5: Adt108) -> R<Adt133> {
    Ok(Adt133::C0(match v5.clone() {
        Adt108::C0(h979, _, _) => {
            h979
        }
    }, f410(v0, v1, v2, f411(v3, v4, v5)?)?))
}

pub fn f346(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt116> {
    let v4: Adt139 = f412(Adt118::C0, f122(v0.clone(), f413())?);
    let v5: Adt139 = f412(Adt118::C1, f122(v0.clone(), f414())?);
    let v6: Adt139 = if f415(v3) {
        f412(Adt118::C2, f416(f122(v0.clone(), f417())?)?)
    } else {
        f418()
    };
    Ok(Adt116::C0(match v4.clone() {
        Adt139::C0(h980, _) => {
            h980
        }
    }, match v5.clone() {
        Adt139::C0(h981, _) => {
            h981
        }
    }, match v6.clone() {
        Adt139::C0(h982, _) => {
            h982
        }
    }, f419(match v4 {
        Adt139::C0(_, h983) => {
            h983
        }
    }, f419(match v5 {
        Adt139::C0(_, h984) => {
            h984
        }
    }, match v6 {
        Adt139::C0(_, h985) => {
            h985
        }
    })), f420(v0, v1, v2)?))
}

pub fn f347(v0: Adt108) -> Option<Str> {
    let a987 = match v0.clone() {
        Adt108::C0(_, h986, _) => {
            h986
        }
    };
    let a988 = Str::lit("tools/call");
    if equal(a987, a988) {
        let m990 = match v0 {
            Adt108::C0(_, _, h989) => {
                h989
            }
        };
        match m990 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m991 = f214(Str::lit("name"), v1);
                match m991 {
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

pub fn f348() -> List<Str> {
    List::cons(f421(), List::<Str>::nil())
}

pub fn f349(v0: Adt5) -> R<Adt140> {
    let m992 = f214(Str::lit("id"), v0);
    match m992 {
        Adt107::C0 => {
            Ok(Adt140::C0)
        }
        Adt107::C1(v1) => {
            let m993 = v1;
            match m993 {
                Adt6::C0 => {
                    Ok(Adt140::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt140::C2)
                }
                Adt6::C2(v3) => {
                    let m994 = f422(v3)?;
                    match m994 {
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

pub fn f350(v0: Adt5) -> Adt104 {
    if f351(v0) {
        Adt104::C8
    } else {
        Adt104::C3
    }
}

pub fn f351(v0: Adt5) -> bool {
    if f305(Str::lit("result"), v0.clone()) {
        true
    } else {
        f305(Str::lit("error"), v0)
    }
}

pub fn f352(v0: Adt100, v1: Adt5) -> Result<Adt108, Adt103> {
    let m995 = f214(Str::lit("jsonrpc"), v1.clone());
    match m995 {
        Adt107::C0 => {
            f270(Some(v0), Adt104::C2)
        }
        Adt107::C1(v2) => {
            let m996 = f141(v2);
            match m996 {
                None => {
                    f270(Some(v0), Adt104::C2)
                }
                Some(v3) => {
                    let a997 = v3;
                    let a998 = f261();
                    if equal(a997, a998) {
                        f423(v0, v1)
                    } else {
                        f270(Some(v0), Adt104::C2)
                    }
                }
            }
        }
    }
}

pub fn f353(v0: u64, v1: u64) -> Adt121 {
    let a999 = v1;
    let a1000 = 0u64;
    if nat_eq(a999, a1000) {
        Adt121::C0
    } else {
        let a1001 = v1;
        let a1002 = 2u64;
        if nat_eq(a1001, a1002) {
            Adt121::C2(v0)
        } else {
            Adt121::C1({
                let a1006 = v0;
                let a1007 = {
                    let a1003 = v1;
                    let a1004 = 3u64;
                    let a1005 = 0u64;
                    nat_quot(a1003, a1004, a1005)
                };
                nat_sub(a1006, a1007)
            })
        }
    }
}

pub fn f354(p1035: Adt122, p1036: u64, p1037: u64, p1038: List<u64>) -> R<u64> {
    let m1034 = (p1035, (p1036, (p1037, p1038)));
    {
        let mut m1034 = m1034;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1034;
                let m1008 = v3;
                match m1008.uncons() {
                    None => {
                        let m1009 = v0;
                        match m1009 {
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
                        let m1010 = v0;
                        match m1010 {
                            Adt122::C0 => {
                                let a1011 = v4;
                                let a1012 = 128u64;
                                if nat_lt(a1011, a1012) {
                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1013 = v4;
                                    let a1014 = 194u64;
                                    if nat_lt(a1013, a1014) {
                                        let v6: u64 = f424({
                                            let a1015 = List::cons(v4, v5);
                                            length_list(a1015)
                                        })?;
                                        Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1016 = v4;
                                        let a1017 = 224u64;
                                        if nat_lt(a1016, a1017) {
                                            Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1018 = v4;
                                            let a1019 = 224u64;
                                            if nat_eq(a1018, a1019) {
                                                Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1020 = v4;
                                                let a1021 = 237u64;
                                                if nat_eq(a1020, a1021) {
                                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1022 = v4;
                                                    let a1023 = 240u64;
                                                    if nat_lt(a1022, a1023) {
                                                        Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1024 = v4;
                                                        let a1025 = 240u64;
                                                        if nat_eq(a1024, a1025) {
                                                            Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1026 = v4;
                                                            let a1027 = 244u64;
                                                            if nat_eq(a1026, a1027) {
                                                                Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1028 = v4;
                                                                let a1029 = 244u64;
                                                                if nat_lt(a1028, a1029) {
                                                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f424({
                                                                        let a1030 = List::cons(v4, v5);
                                                                        length_list(a1030)
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
                                        let a1031 = List::cons(v4, v5);
                                        length_list(a1031)
                                    })?;
                                    Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt122::C2 => {
                                if f425(v4, v1, v2) {
                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f424({
                                        let a1032 = List::cons(v4, v5);
                                        length_list(a1032)
                                    })?;
                                    Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt122::C3 => {
                                if f425(v4, v1, v2) {
                                    Ok::<(Adt122, (u64, (u64, List<u64>))), R<u64>>((Adt122::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f424({
                                        let a1033 = List::cons(v4, v5);
                                        length_list(a1033)
                                    })?;
                                    Err::<(Adt122, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
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

pub fn f355(v0: Adt123, v1: u64) -> R<Result<Adt6, Adt102>> {
    let m1040 = match v0.clone() {
        Adt123::C0(h1039, _, _) => {
            h1039
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
        Adt124::C8(v3) => {
            let m1041 = f426(v3)?;
            match m1041 {
                Adt147::C0 => {
                    Ok(Err::<Adt6, Adt102>(Adt102::C7(v1)))
                }
                Adt147::C1 => {
                    Ok(Err::<Adt6, Adt102>(Adt102::C8(v1)))
                }
                Adt147::C2(v4) => {
                    let m1046 = {
                        let m1045 = f427(v4, match v0.clone() {
                            Adt123::C0(_, h1043, _) => {
                                h1043
                            }
                        }, match v0 {
                            Adt123::C0(_, _, h1044) => {
                                h1044
                            }
                        });
                        match m1045 {
                            Adt123::C0(h1042, _, _) => {
                                h1042
                            }
                        }
                    };
                    match m1046 {
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

pub fn f356(v0: Adt123, v1: u64, v2: u64) -> R<Adt141> {
    let m1048 = match v0.clone() {
        Adt123::C0(h1047, _, _) => {
            h1047
        }
    };
    match m1048 {
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
                let a1049 = v2;
                let a1050 = 93u64;
                if nat_eq(a1049, a1050) {
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
                let a1051 = v2;
                let a1052 = 125u64;
                if nat_eq(a1051, a1052) {
                    Ok(f432(v0, v1))
                } else {
                    let a1053 = v2;
                    let a1054 = 34u64;
                    if nat_eq(a1053, a1054) {
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
                let a1055 = v2;
                let a1056 = 34u64;
                if nat_eq(a1055, a1056) {
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
                let a1057 = v2;
                let a1058 = 58u64;
                if nat_eq(a1057, a1058) {
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

pub fn f357(v0: u64) -> R<u64> {
    let a1059 = 64u64;
    let a1060 = v0;
    if if nat_lt(a1059, a1060) {
        let a1061 = v0;
        let a1062 = 91u64;
        nat_lt(a1061, a1062)
    } else {
        false
    } {
        let a1063 = v0;
        let a1064 = 32u64;
        nat_add(a1063, a1064)
    } else {
        Ok(v0)
    }
}

pub fn f358(p1067: List<u64>) -> List<u64> {
    let m1066 = p1067;
    {
        let mut m1066 = m1066;
        loop {
            let step = {
                let v0 = m1066;
                let m1065 = v0;
                match m1065.uncons() {
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
                Ok(next) => { m1066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: u64) -> bool {
    let a1068 = v0;
    let a1069 = 9u64;
    if nat_eq(a1068, a1069) {
        true
    } else {
        let a1070 = 31u64;
        let a1071 = v0;
        if nat_lt(a1070, a1071) {
            let a1072 = v0;
            let a1073 = 127u64;
            nat_lt(a1072, a1073)
        } else {
            false
        }
    }
}

pub fn f360(p1076: List<u64>) -> bool {
    let m1075 = p1076;
    {
        let mut m1075 = m1075;
        loop {
            let step = {
                let v0 = m1075;
                let m1074 = v0;
                match m1074.uncons() {
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
                Ok(next) => { m1075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(p1079: List<u64>, v1: bool) -> bool {
    let m1078 = p1079;
    {
        let mut m1078 = m1078;
        loop {
            let step = {
                let v0 = m1078;
                let m1077 = v0;
                match m1077.uncons() {
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
                Ok(next) => { m1078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362() -> u64 {
    64u64
}

pub fn f363(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1080 = f298(Str::lit("type"), v1);
    match m1080 {
        Adt107::C0 => {
            Some(Adt97::C3(v0))
        }
        Adt107::C1(v2) => {
            let m1081 = f141(v2);
            match m1081 {
                None => {
                    Some(Adt97::C3(v0))
                }
                Some(v3) => {
                    let a1082 = v3;
                    let a1083 = Str::lit("object");
                    if equal(a1082, a1083) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f364(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1084 = f298(Str::lit("$schema"), v1);
    match m1084 {
        Adt107::C0 => {
            None::<Adt97>
        }
        Adt107::C1(v2) => {
            let m1085 = f141(v2);
            match m1085 {
                None => {
                    Some(Adt97::C2(v0))
                }
                Some(v3) => {
                    let a1086 = v3;
                    let a1087 = f442();
                    if equal(a1086, a1087) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f365(v0: Adt36) -> R<Option<Adt97>> {
    let m1089 = match v0.clone() {
        Adt36::C0(_, _, _, _, _, h1088, _, _) => {
            h1088
        }
    };
    match m1089 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v1) => {
            let m1092 = f364(match v0.clone() {
                Adt36::C0(h1090, _, _, _, _, _, _, _) => {
                    h1090
                }
            }, match v1.clone() {
                Adt5::C0(h1091) => {
                    h1091
                }
            });
            match m1092 {
                None => {
                    let m1096 = f288(f289(), match v0 {
                        Adt36::C0(h1093, _, _, _, _, _, _, _) => {
                            h1093
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1094) => {
                            h1094
                        }
                    }), List::cons(Adt109::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1095) => {
                            h1095
                        }
                    }), false, List::<Str>::nil()), List::<Adt109>::nil()), f290())?;
                    match m1096 {
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

pub fn f366(v0: Str, v1: Adt6, v2: Adt109, v3: Adt129, v4: List<Adt109>) -> R<Result<(List<Adt109>, Adt129), Adt97>> {
    let m1097 = v2;
    match m1097 {
        Adt109::C0(v5, v6, v7) => {
            let m1098 = f443(v5.clone());
            match m1098 {
                None => {
                    let m1099 = f219(v5);
                    match m1099 {
                        None => {
                            Ok(Err::<(List<Adt109>, Adt129), Adt97>(Adt97::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f444(v0, v1, match v9 {
                                Adt5::C0(h1100) => {
                                    h1100
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

pub fn f367(p1109: List<Adt39>) -> R<bool> {
    let m1108 = p1109;
    {
        let mut m1108 = m1108;
        loop {
            let step = {
                let v0 = m1108;
                let m1101 = v0;
                match m1101.uncons() {
                    None => {
                        Err::<List<Adt39>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m1102 = v1;
                        match m1102 {
                            Adt39::C0(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C1(v4) => {
                                if f445(match v4 {
                                    Adt43::C0(h1103, _, _, _) => {
                                        h1103
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C2(v5) => {
                                if f445(match v5 {
                                    Adt44::C0(h1104, _, _, _) => {
                                        h1104
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
                                let m1106 = match v7 {
                                    Adt46::C0(h1105, _, _) => {
                                        h1105
                                    }
                                };
                                match m1106 {
                                    Adt47::C0(_) => {
                                        Ok::<List<Adt39>, R<bool>>(v2)
                                    }
                                    Adt47::C1(v9) => {
                                        if f445(match v9 {
                                            Adt49::C0(_, _, h1107, _) => {
                                                h1107
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
                Ok(next) => { m1108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f446(match v1.clone() {
        Adt50::C0(h1110, _) => {
            h1110
        }
    }) {
        f447(v1)
    } else {
        let m1112 = match v0 {
            Adt36::C0(_, _, _, _, _, h1111, _, _) => {
                h1111
            }
        };
        match m1112 {
            None => {
                f447(v1)
            }
            Some(v2) => {
                f448(v2, v1)
            }
        }
    }
}

pub fn f369() -> Adt90 {
    Adt90::C9(Str::lit("the host supplied no task id"))
}

pub fn f370(v0: Adt14, v1: Adt86) -> Adt85 {
    Adt85::C0(Adt63::C0(Adt64::C0(match v0.clone() {
        Adt14::C0(_, _, h1113, _) => {
            h1113
        }
    }, None::<Str>, match v0.clone() {
        Adt14::C0(_, h1114, _, _) => {
            h1114
        }
    }, match v0.clone() {
        Adt14::C0(_, h1115, _, _) => {
            h1115
        }
    }, Some(f449()), Some(f450()))), match v0 {
        Adt14::C0(h1116, _, _, _) => {
            h1116
        }
    }, v1)
}

pub fn f371(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt111>) -> R<Adt142> {
    let m1117 = f443(v1.clone());
    match m1117 {
        None => {
            let m1118 = f219(v1);
            match m1118 {
                None => {
                    Ok(Adt142::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f451(v0, match v5 {
                        Adt5::C0(h1119) => {
                            h1119
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

pub fn f372(v0: Adt113, v1: List<Adt113>) -> R<Adt130> {
    let m1120 = v0;
    match m1120 {
        Adt113::C0(_, _, _) => {
            Ok(Adt130::C0(Adt112::C0, v1))
        }
        Adt113::C1(_, _, _) => {
            Ok(Adt130::C0(Adt112::C0, v1))
        }
        Adt113::C2(v8, v9, v10, v11) => {
            Ok(f452(v8, v9, v10, {
                let a1121 = v11;
                let a1122 = 1u64;
                nat_add(a1121, a1122)?
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

pub fn f373(v0: Adt114, v1: Adt6, v2: Adt6, v3: List<Adt111>) -> Adt142 {
    let m1123 = f219(v1);
    match m1123 {
        None => {
            Adt142::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m1124 = v0;
            match m1124 {
                Adt114::C0 => {
                    f454(match v4 {
                        Adt5::C0(h1125) => {
                            h1125
                        }
                    }, v2, v3)
                }
                Adt114::C1 => {
                    f455(match v4 {
                        Adt5::C0(h1126) => {
                            h1126
                        }
                    }, v2, v3)
                }
                Adt114::C2 => {
                    f456(match v4 {
                        Adt5::C0(h1127) => {
                            h1127
                        }
                    }, v2, v3)
                }
                Adt114::C3 => {
                    f457(match v4 {
                        Adt5::C0(h1128) => {
                            h1128
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f374(v0: Adt113, v1: List<Adt113>, v2: List<Adt111>, v3: Str) -> Adt130 {
    let m1129 = v0;
    match m1129 {
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

pub fn f375(p1132: List<Adt111>, p1133: List<Str>) -> R<List<Str>> {
    let m1131 = (p1132, p1133);
    {
        let mut m1131 = m1131;
        loop {
            let step = {
                let (v0, v1) = m1131;
                let m1130 = v0;
                match m1130.uncons() {
                    None => {
                        Err::<(List<Adt111>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt111>, List<Str>), R<List<Str>>>((v3, List::cons(f459(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Str, v1: Str) -> Adt143 {
    let a1134 = v0;
    let a1135 = v1;
    if equal(a1134, a1135) {
        Adt143::C0
    } else {
        Adt143::C1
    }
}

pub fn f377(p1150: u64, p1151: Adt131, p1152: List<Adt132>) -> R<Option<Adt6>> {
    let m1149 = (p1150, (p1151, p1152));
    {
        let mut m1149 = m1149;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1149;
                let m1136 = v0;
                if m1136 == 0 {
                    Err::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1136 - 1;
                    let m1137 = v1;
                    match m1137 {
                        Adt131::C0(v4) => {
                            let m1138 = v4.clone();
                            match m1138 {
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
                                    let m1139 = v9;
                                    match m1139.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C0(v10), List::cons(Adt132::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1140 = v12;
                                    match m1140.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f460({
                                                let (h1141, _) = v13;
                                                h1141
                                            })?;
                                            Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C0({
                                                let (_, h1142) = v13;
                                                h1142
                                            }), List::cons(Adt132::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt131::C1(v16) => {
                            let m1143 = v2;
                            match m1143.uncons() {
                                None => {
                                    Err::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1144 = v17;
                                    match m1144 {
                                        Adt132::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1145 = v20;
                                            match m1145.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(Adt6::C4(f380(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C0(v22), List::cons(Adt132::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt132::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1146 = v26;
                                            match m1146.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C1(Adt6::C5(f461(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f460({
                                                        let (h1147, _) = v28;
                                                        h1147
                                                    })?;
                                                    Ok::<(u64, (Adt131, List<Adt132>)), R<Option<Adt6>>>((v3, (Adt131::C0({
                                                        let (_, h1148) = v28;
                                                        h1148
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
                Ok(next) => { m1149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378() -> u64 {
    67108864u64
}

pub fn f379(p1156: List<Adt39>, p1157: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1155 = (p1156, p1157);
    {
        let mut m1155 = m1155;
        loop {
            let step = {
                let (v0, v1) = m1155;
                let m1153 = v0;
                match m1153.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1154 = f462(v2)?;
                        match m1154 {
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
                Ok(next) => { m1155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(p1160: List<Adt6>, p1161: List<Adt6>) -> List<Adt6> {
    let m1159 = (p1160, p1161);
    {
        let mut m1159 = m1159;
        loop {
            let step = {
                let (v0, v1) = m1159;
                let m1158 = v0;
                match m1158.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381(v0: Adt107) -> bool {
    let m1162 = v0;
    match m1162 {
        Adt107::C0 => {
            false
        }
        Adt107::C1(_) => {
            true
        }
    }
}

pub fn f382(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m1163 = f214(v0, v1);
    match m1163 {
        Adt107::C0 => {
            None::<Adt5>
        }
        Adt107::C1(v2) => {
            f219(v2)
        }
    }
}

pub fn f383(v0: Adt5) -> Option<Adt63> {
    let m1164 = f463(v0.clone());
    match m1164 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m1165 = f464(Str::lit("status"), v0.clone());
            match m1165 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let a1166 = v2.clone();
                    let a1167 = Str::lit("working");
                    if equal(a1166, a1167) {
                        Some(Adt63::C0(v1))
                    } else {
                        let a1168 = v2.clone();
                        let a1169 = Str::lit("cancelled");
                        if equal(a1168, a1169) {
                            Some(Adt63::C4(v1))
                        } else {
                            let a1170 = v2.clone();
                            let a1171 = Str::lit("input_required");
                            if equal(a1170, a1171) {
                                let m1172 = f465(Str::lit("inputRequests"), v0);
                                match m1172 {
                                    None => {
                                        None::<Adt63>
                                    }
                                    Some(v3) => {
                                        Some(Adt63::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a1173 = v2.clone();
                                let a1174 = Str::lit("completed");
                                if equal(a1173, a1174) {
                                    let m1175 = f382(Str::lit("result"), v0);
                                    match m1175 {
                                        None => {
                                            None::<Adt63>
                                        }
                                        Some(v4) => {
                                            Some(Adt63::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a1176 = v2;
                                    let a1177 = Str::lit("failed");
                                    if equal(a1176, a1177) {
                                        let m1178 = f382(Str::lit("error"), v0);
                                        match m1178 {
                                            None => {
                                                None::<Adt63>
                                            }
                                            Some(v5) => {
                                                let m1179 = f466(v5);
                                                match m1179 {
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

pub fn f384(v0: Str, v1: Adt5) -> Option<i64> {
    let m1180 = f214(v0, v1);
    match m1180 {
        Adt107::C0 => {
            None::<i64>
        }
        Adt107::C1(v2) => {
            f215(v2)
        }
    }
}

pub fn f385(v0: Adt5) -> Option<Adt86> {
    let m1181 = f384(Str::lit("parkAfterMs"), v0.clone());
    match m1181 {
        None => {
            None::<Adt86>
        }
        Some(v1) => {
            let m1182 = f465(Str::lit("parked"), v0.clone());
            match m1182 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m1183 = f384(Str::lit("finishAfterMs"), v0.clone());
                    match m1183 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            let m1184 = f382(Str::lit("outcome"), v0);
                            match m1184 {
                                None => {
                                    None::<Adt86>
                                }
                                Some(v4) => {
                                    let m1185 = f467(v4);
                                    match m1185 {
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

pub fn f386(p1188: List<(Str, Adt65)>, p1189: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m1187 = (p1188, p1189);
    {
        let mut m1187 = m1187;
        loop {
            let step = {
                let (v0, v1) = m1187;
                let m1186 = v0;
                match m1186.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(p1193: List<(Str, Adt65)>, v1: Adt5, p1194: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m1192 = (p1193, p1194);
    {
        let mut m1192 = m1192;
        loop {
            let step = {
                let (v0, v2) = m1192;
                let m1190 = v0;
                match m1190.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v2)
                    }
                    Some((v3, v4)) => {
                        if f305({
                            let (h1191, _) = v3.clone();
                            h1191
                        }, v1.clone()) {
                            Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v4, v2))
                        } else {
                            Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v4, List::cons(v3, v2)))
                        }
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

pub fn f388(v0: Str) -> R<Option<List<u64>>> {
    let m1195 = f468(f469(f470())?, f469(v0)?);
    match m1195 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1196 = f468(f471(List::cons(47u64, f469(f472())?)), f471(v1));
            match m1196 {
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

pub fn f389(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1197 = v0;
    let a1198 = f475(f179(), f476(List::<Bytes>::nil(), v1));
    append_bytes(a1197, a1198)
}

pub fn f390(v0: Adt6) -> R<Result<Bytes, Adt102>> {
    let m1199 = f181(v0)?;
    match m1199 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt102>(f389(f179(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt102>(v2))
        }
    }
}

pub fn f391(p1203: List<Str>, p1204: u64, p1205: List<Str>) -> List<Str> {
    let m1202 = (p1203, (p1204, p1205));
    {
        let mut m1202 = m1202;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1202;
                let m1200 = v0;
                match m1200.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m1201 = v1;
                        if m1201 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m1201 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
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

pub fn f392(v0: Str, v1: Str) -> R<bool> {
    Ok(f477(f469(v0)?, f469(v1)?))
}

pub fn f393(p1209: List<Adt45>, p1210: List<Str>) -> List<Str> {
    let m1208 = (p1209, p1210);
    {
        let mut m1208 = m1208;
        loop {
            let step = {
                let (v0, v1) = m1208;
                let m1206 = v0;
                match m1206.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt45::C0(_, _, _, h1207, _, _, _, _, _) => {
                                h1207
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: i64, v1: Str, v2: Str, v3: Adt95) -> Result<(Adt14, u64), Adt93> {
    Ok::<(Adt14, u64), Adt93>((Adt14::C0(v0, v1, v2, match v3.clone() {
        Adt95::C0(h1211, _) => {
            h1211
        }
    }), match v3 {
        Adt95::C0(_, h1212) => {
            h1212
        }
    }))
}

pub fn f395(v0: Adt115, v1: List<Adt115>, v2: List<u64>) -> R<Adt144> {
    let m1213 = v0;
    match m1213 {
        Adt115::C0(v3) => {
            Ok(Adt144::C1(v1, f337(v3, v2)))
        }
        Adt115::C1(v4, v5) => {
            f478(v4, v5, v1, v2)
        }
        Adt115::C2(v6, v7) => {
            let m1214 = v7;
            match m1214.uncons() {
                None => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt144::C1(List::cons(Adt115::C1(v6, v8), List::cons(Adt115::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt115::C3(v10, v11) => {
            let m1215 = v11.clone();
            match m1215.uncons() {
                None => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt115::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt115::C4(v14, v15) => {
            let m1216 = v15;
            match m1216.uncons() {
                None => {
                    Ok(Adt144::C1(List::cons(Adt115::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt144::C1(List::cons(Adt115::C0({
                        let a1218 = f479({
                            let (h1217, _) = v16;
                            h1217
                        })?;
                        let a1219 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1218, a1219)
                    }), List::cons(Adt115::C1(v14, {
                        let (_, h1220) = v16;
                        h1220
                    }), List::cons(Adt115::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt115::C5(v18, v19) => {
            let m1221 = v19.clone();
            match m1221.uncons() {
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
    let m1223 = f480(match v0.clone() {
        Adt134::C0(h1222, _, _) => {
            h1222
        }
    })?;
    match m1223 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1226 = v1;
                let a1227 = {
                    let m1225 = f481(v0);
                    match m1225 {
                        Adt5::C0(h1224) => {
                            h1224
                        }
                    }
                };
                append_list(a1226, a1227)
            })))
        }
    }
}

pub fn f397(v0: Adt14, v1: Option<Adt88>, v2: Adt94) -> R<Adt94> {
    let m1228 = v1;
    match m1228 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a1230 = match v2.clone() {
                Adt94::C0(h1229, _, _, _, _) => {
                    h1229
                }
            };
            let a1231 = f400();
            if nat_eq(a1230, a1231) {
                Ok(f482(f483(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f398(v0: Adt33) -> Option<Adt88> {
    let m1232 = v0;
    match m1232 {
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
                Adt83::C0(_, _, h1233) => {
                    h1233
                }
            }, Adt89::C0, false))
        }
        Adt33::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f399(v0: bool, v1: Adt94, v2: Option<List<List<u64>>>) -> Adt94 {
    let m1234 = v2;
    match m1234 {
        None => {
            f6()
        }
        Some(v3) => {
            f484(v0, v1, f45(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f400() -> u64 {
    200u64
}

pub fn f401(v0: Adt100, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f261())), List::cons((Str::lit("id"), f338(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1235) => {
            h1235
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f402(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m1236 = v0;
    match m1236 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m1237 = v1;
            match m1237 {
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

pub fn f403(p1241: List<Adt136>, p1242: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1240 = (p1241, p1242);
    {
        let mut m1240 = m1240;
        loop {
            let step = {
                let (v0, v1) = m1240;
                let m1238 = v0;
                match m1238.uncons() {
                    None => {
                        Err::<(List<Adt136>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1239 = f485(v2)?;
                        match m1239 {
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
                Ok(next) => { m1240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: Option<Adt88>) -> R<Option<List<List<u64>>>> {
    let m1243 = v0;
    match m1243 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f486(v1)
        }
    }
}

pub fn f405(v0: Adt137) -> u64 {
    let m1244 = v0;
    match m1244 {
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
            f487()
        }
        Adt137::C4(v6) => {
            f488(v6)
        }
    }
}

pub fn f406(v0: Adt137) -> R<Adt81> {
    Ok(Adt81::C0(f489(v0.clone()), f490(v0.clone()), f491(v0)?))
}

pub fn f407() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f408(p1247: List<List<u64>>) -> R<bool> {
    let m1246 = p1247;
    {
        let mut m1246 = m1246;
        loop {
            let step = {
                let v0 = m1246;
                let m1245 = v0;
                match m1245.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f492(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f409(v0: u64, p1252: List<u64>, p1253: List<u64>, p1254: List<List<u64>>) -> List<List<u64>> {
    let m1251 = (p1252, (p1253, p1254));
    {
        let mut m1251 = m1251;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1251;
                let m1248 = v1;
                match m1248.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f45(List::cons(f284(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1249 = v4;
                        let a1250 = v0;
                        if nat_eq(a1249, a1250) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f284(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: Adt0, v1: Fn3, v2: u64, v3: Result<Adt145, Adt137>) -> R<Result<Adt134, Adt137>> {
    let m1255 = v3;
    match m1255 {
        Ok(v5) => {
            f493(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt134, Adt137>(v4))
        }
    }
}

pub fn f411(v0: Option<Adt116>, v1: List<Str>, v2: Adt108) -> R<Result<Adt145, Adt137>> {
    Ok(f494(f495(match v2.clone() {
        Adt108::C0(_, h1256, _) => {
            h1256
        }
    }), f496(v0, v1, v2)?))
}

pub fn f412(v0: Adt118, v1: Adt105) -> Adt139 {
    let m1257 = v1;
    match m1257 {
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
            let m1258 = f334(v2);
            match m1258 {
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
    let m1259 = f497(v0);
    match m1259 {
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
    let m1260 = v0.clone();
    match m1260 {
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
            let m1261 = f498(v1);
            match m1261 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1262 = f499(v2)?;
                    match m1262 {
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
    let m1263 = v0.clone();
    match m1263 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f420(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt119>> {
    Ok(f500(f501(v0, v1, v2, List::<Adt119>::nil())?, List::<Adt119>::nil()))
}

pub fn f421() -> Str {
    Str::lit("2026-07-28")
}

pub fn f422(v0: Adt7) -> R<Option<i64>> {
    let m1266 = f502(40u64, match v0.clone() {
        Adt7::C0(h1264, _) => {
            h1264
        }
    }, match v0 {
        Adt7::C0(_, h1265) => {
            h1265
        }
    })?;
    match m1266 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f503(v1))
        }
    }
}

pub fn f423(v0: Adt100, v1: Adt5) -> Result<Adt108, Adt103> {
    let m1267 = f214(Str::lit("method"), v1.clone());
    match m1267 {
        Adt107::C0 => {
            f270(Some(v0), f350(v1))
        }
        Adt107::C1(v2) => {
            let m1268 = f141(v2);
            match m1268 {
                None => {
                    f270(Some(v0), Adt104::C4)
                }
                Some(v3) => {
                    f504(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f424(v0: u64) -> R<u64> {
    let a1271 = {
        let a1269 = v0;
        let a1270 = 3u64;
        nat_mul(a1269, a1270)?
    };
    let a1272 = 1u64;
    nat_add(a1271, a1272)
}

pub fn f425(v0: u64, v1: u64, v2: u64) -> bool {
    let a1273 = v1;
    let a1274 = v0;
    if nat_le(a1273, a1274) {
        let a1275 = v0;
        let a1276 = v2;
        nat_le(a1275, a1276)
    } else {
        false
    }
}

pub fn f426(v0: Adt126) -> R<Adt147> {
    if f505(match v0.clone() {
        Adt126::C0(_, h1277, _, _, _, _, _) => {
            h1277
        }
    }) {
        let v1: i64 = {
            let a1283 = if match v0.clone() {
                Adt126::C0(_, _, _, _, _, h1278, _) => {
                    h1278
                }
            } {
                let a1280 = match v0.clone() {
                    Adt126::C0(_, _, _, _, _, _, h1279) => {
                        h1279
                    }
                };
                int_neg(a1280)?
            } else {
                match v0.clone() {
                    Adt126::C0(_, _, _, _, _, _, h1281) => {
                        h1281
                    }
                }
            };
            let a1284 = match v0.clone() {
                Adt126::C0(_, _, _, _, h1282, _, _) => {
                    h1282
                }
            };
            int_sub(a1283, a1284)?
        };
        let m1287 = {
            let a1285 = v1;
            let a1286 = -9999i64;
            compare(a1285, a1286)
        };
        if if match m1287 {
            Ordering::Less => {
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
            let m1290 = {
                let a1288 = 9999i64;
                let a1289 = v1;
                compare(a1288, a1289)
            };
            match m1290 {
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
                Adt126::C0(h1291, _, _, _, _, _, _) => {
                    h1291
                }
            } {
                let a1293 = match v0 {
                    Adt126::C0(_, _, h1292, _, _, _, _) => {
                        h1292
                    }
                };
                int_neg(a1293)?
            } else {
                match v0 {
                    Adt126::C0(_, _, h1294, _, _, _, _) => {
                        h1294
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt147::C0)
    }
}

pub fn f427(v0: Adt6, v1: List<Adt128>, v2: u64) -> Adt123 {
    let m1295 = v1.clone();
    match m1295.uncons() {
        None => {
            Adt123::C0(Adt124::C9(v0), List::<Adt128>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1296 = v3;
            match m1296 {
                Adt128::C0(v5) => {
                    Adt123::C0(Adt124::C2, List::cons(Adt128::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt128::C1(v6, v7) => {
                    let m1297 = v7;
                    match m1297 {
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
    let a1298 = v0;
    let a1299 = 32u64;
    if nat_eq(a1298, a1299) {
        true
    } else {
        let a1300 = v0;
        let a1301 = 10u64;
        if nat_eq(a1300, a1301) {
            true
        } else {
            let a1302 = v0;
            let a1303 = 13u64;
            if nat_eq(a1302, a1303) {
                true
            } else {
                let a1304 = v0;
                let a1305 = 9u64;
                nat_eq(a1304, a1305)
            }
        }
    }
}

pub fn f429(v0: Adt123, v1: u64, v2: u64) -> R<Adt141> {
    let a1306 = v2;
    let a1307 = 34u64;
    if nat_eq(a1306, a1307) {
        Ok(f433(v0, Adt124::C6(Adt125::C0, List::<u64>::nil(), false)))
    } else {
        let a1308 = v2;
        let a1309 = 123u64;
        if nat_eq(a1308, a1309) {
            f506(v0, Adt128::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt124::C3, v1)
        } else {
            let a1310 = v2;
            let a1311 = 91u64;
            if nat_eq(a1310, a1311) {
                f506(v0, Adt128::C0(List::<Adt6>::nil()), Adt124::C1, v1)
            } else {
                let a1312 = v2;
                let a1313 = 116u64;
                if nat_eq(a1312, a1313) {
                    Ok(f433(v0, Adt124::C7(0u64, 1u64)))
                } else {
                    let a1314 = v2;
                    let a1315 = 102u64;
                    if nat_eq(a1314, a1315) {
                        Ok(f433(v0, Adt124::C7(1u64, 1u64)))
                    } else {
                        let a1316 = v2;
                        let a1317 = 110u64;
                        if nat_eq(a1316, a1317) {
                            Ok(f433(v0, Adt124::C7(2u64, 1u64)))
                        } else {
                            let a1318 = v2;
                            let a1319 = 45u64;
                            if nat_eq(a1318, a1319) {
                                Ok(f433(v0, f507(true, Adt127::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1320 = v2;
                                let a1321 = 48u64;
                                if nat_eq(a1320, a1321) {
                                    Ok(f433(v0, f507(false, Adt127::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f425(v2, 49u64, 57u64) {
                                        Ok(f433(v0, f507(false, Adt127::C2, f508({
                                            let a1322 = v2;
                                            let a1323 = 48u64;
                                            nat_sub(a1322, a1323)
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
    let m1325 = match v0.clone() {
        Adt123::C0(_, h1324, _) => {
            h1324
        }
    };
    match m1325.uncons() {
        None => {
            f434(Adt102::C2(v1))
        }
        Some((v2, v3)) => {
            let m1326 = v2;
            match m1326 {
                Adt128::C0(v4) => {
                    Adt141::C1(f427(Adt6::C4(f380(v4, List::<Adt6>::nil())), v3, {
                        let a1328 = match v0 {
                            Adt123::C0(_, _, h1327) => {
                                h1327
                            }
                        };
                        let a1329 = 1u64;
                        nat_sub(a1328, a1329)
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
    let m1331 = match v0.clone() {
        Adt123::C0(h1330, _, _) => {
            h1330
        }
    };
    match m1331 {
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
                let a1332 = v2;
                let a1333 = 44u64;
                if nat_eq(a1332, a1333) {
                    f509(v0, v1)
                } else {
                    let a1334 = v2;
                    let a1335 = 93u64;
                    if nat_eq(a1334, a1335) {
                        f430(v0, v1)
                    } else {
                        let a1336 = v2;
                        let a1337 = 125u64;
                        if nat_eq(a1336, a1337) {
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
    let m1339 = match v0.clone() {
        Adt123::C0(_, h1338, _) => {
            h1338
        }
    };
    match m1339.uncons() {
        None => {
            f434(Adt102::C2(v1))
        }
        Some((v2, v3)) => {
            let m1340 = v2;
            match m1340 {
                Adt128::C0(_) => {
                    f434(Adt102::C2(v1))
                }
                Adt128::C1(v5, _) => {
                    Adt141::C1(f427(Adt6::C5(f461(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1342 = match v0 {
                            Adt123::C0(_, _, h1341) => {
                                h1341
                            }
                        };
                        let a1343 = 1u64;
                        nat_sub(a1342, a1343)
                    }))
                }
            }
        }
    }
}

pub fn f433(v0: Adt123, v1: Adt124) -> Adt141 {
    Adt141::C1(f510(v0, v1))
}

pub fn f434(v0: Adt102) -> Adt141 {
    Adt141::C0(v0)
}

pub fn f435(v0: Adt123, v1: u64, v2: u64, v3: Adt125, v4: List<u64>, v5: bool) -> R<Adt141> {
    let m1344 = v3;
    match m1344 {
        Adt125::C0 => {
            let a1345 = v2;
            let a1346 = 34u64;
            if nat_eq(a1345, a1346) {
                Ok(f511(v0, v1, v4, v5))
            } else {
                let a1347 = v2;
                let a1348 = 92u64;
                if nat_eq(a1347, a1348) {
                    Ok(f433(v0, Adt124::C6(Adt125::C1, v4, v5)))
                } else {
                    let a1349 = v2;
                    let a1350 = 32u64;
                    if nat_lt(a1349, a1350) {
                        Ok(f434(Adt102::C4(v1)))
                    } else {
                        Ok(f433(v0, Adt124::C6(Adt125::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt125::C1 => {
            let a1351 = v2;
            let a1352 = 117u64;
            if nat_eq(a1351, a1352) {
                Ok(f433(v0, Adt124::C6(Adt125::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1353 = f512(v2);
                let a1354 = 0u64;
                if nat_eq(a1353, a1354) {
                    Ok(f434(Adt102::C5(v1)))
                } else {
                    Ok(f433(v0, Adt124::C6(Adt125::C0, List::cons(f512(v2), v4), v5)))
                }
            }
        }
        Adt125::C2(v6, v7, v8) => {
            let a1355 = 15u64;
            let a1356 = f513(v2);
            if nat_lt(a1355, a1356) {
                Ok(f434(Adt102::C5(v1)))
            } else {
                let a1359 = {
                    let a1357 = v6;
                    let a1358 = 1u64;
                    nat_add(a1357, a1358)?
                };
                let a1360 = 4u64;
                if nat_eq(a1359, a1360) {
                    f514(v0, v1, {
                        let a1363 = {
                            let a1361 = v7;
                            let a1362 = 16u64;
                            nat_mul(a1361, a1362)?
                        };
                        let a1364 = f513(v2);
                        nat_add(a1363, a1364)?
                    }, v8, v4, v5)
                } else {
                    Ok(f433(v0, Adt124::C6(Adt125::C2({
                        let a1365 = v6;
                        let a1366 = 1u64;
                        nat_add(a1365, a1366)?
                    }, {
                        let a1369 = {
                            let a1367 = v7;
                            let a1368 = 16u64;
                            nat_mul(a1367, a1368)?
                        };
                        let a1370 = f513(v2);
                        nat_add(a1369, a1370)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt125::C3(v9) => {
            let a1371 = v2;
            let a1372 = 92u64;
            if nat_eq(a1371, a1372) {
                Ok(f433(v0, Adt124::C6(Adt125::C4(v9), v4, v5)))
            } else {
                Ok(f434(Adt102::C6(v1)))
            }
        }
        Adt125::C4(v10) => {
            let a1373 = v2;
            let a1374 = 117u64;
            if nat_eq(a1373, a1374) {
                Ok(f433(v0, Adt124::C6(Adt125::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f434(Adt102::C6(v1)))
            }
        }
    }
}

pub fn f436(v0: Adt123, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt141> {
    let a1375 = v2;
    let a1376 = f515(v3, v4);
    if nat_eq(a1375, a1376) {
        let a1380 = {
            let a1377 = v4;
            let a1378 = 1u64;
            nat_add(a1377, a1378)?
        };
        let a1381 = {
            let a1379 = f516(v3);
            length_list(a1379)
        };
        if nat_eq(a1380, a1381) {
            Ok(f517(v0, f518(v3)))
        } else {
            Ok(f433(v0, Adt124::C7(v3, {
                let a1382 = v4;
                let a1383 = 1u64;
                nat_add(a1382, a1383)?
            })))
        }
    } else {
        Ok(f434(Adt102::C2(v1)))
    }
}

pub fn f437(v0: Adt123, v1: u64, v2: u64, v3: Adt126) -> R<Adt141> {
    let m1385 = match v3.clone() {
        Adt126::C0(_, h1384, _, _, _, _, _) => {
            h1384
        }
    };
    match m1385 {
        Adt127::C0 => {
            let a1386 = v2;
            let a1387 = 48u64;
            if nat_eq(a1386, a1387) {
                Ok(f519(v0, v3, Adt127::C1))
            } else {
                if f425(v2, 49u64, 57u64) {
                    f520(v0, v1, v2, v3, false)
                } else {
                    Ok(f434(Adt102::C7(v1)))
                }
            }
        }
        Adt127::C1 => {
            let a1388 = v2;
            let a1389 = 46u64;
            if nat_eq(a1388, a1389) {
                Ok(f519(v0, v3, Adt127::C3))
            } else {
                if f521(v2) {
                    Ok(f519(v0, v3, Adt127::C5))
                } else {
                    if f522(v2) {
                        Ok(f434(Adt102::C7(v1)))
                    } else {
                        f523(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt127::C2 => {
            if f522(v2) {
                f520(v0, v1, v2, v3, false)
            } else {
                let a1390 = v2;
                let a1391 = 46u64;
                if nat_eq(a1390, a1391) {
                    Ok(f519(v0, v3, Adt127::C3))
                } else {
                    if f521(v2) {
                        Ok(f519(v0, v3, Adt127::C5))
                    } else {
                        f523(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt127::C3 => {
            if f522(v2) {
                f520(v0, v1, v2, v3, true)
            } else {
                Ok(f434(Adt102::C7(v1)))
            }
        }
        Adt127::C4 => {
            if f522(v2) {
                f520(v0, v1, v2, v3, true)
            } else {
                if f521(v2) {
                    Ok(f519(v0, v3, Adt127::C5))
                } else {
                    f523(v0, v1, v2, v3)
                }
            }
        }
        Adt127::C5 => {
            let a1392 = v2;
            let a1393 = 43u64;
            if nat_eq(a1392, a1393) {
                Ok(f433(v0, f507(match v3.clone() {
                    Adt126::C0(h1394, _, _, _, _, _, _) => {
                        h1394
                    }
                }, Adt127::C6, match v3.clone() {
                    Adt126::C0(_, _, h1395, _, _, _, _) => {
                        h1395
                    }
                }, match v3.clone() {
                    Adt126::C0(_, _, _, h1396, _, _, _) => {
                        h1396
                    }
                }, match v3 {
                    Adt126::C0(_, _, _, _, h1397, _, _) => {
                        h1397
                    }
                }, false, 0i64)))
            } else {
                let a1398 = v2;
                let a1399 = 45u64;
                if nat_eq(a1398, a1399) {
                    Ok(f433(v0, f507(match v3.clone() {
                        Adt126::C0(h1400, _, _, _, _, _, _) => {
                            h1400
                        }
                    }, Adt127::C6, match v3.clone() {
                        Adt126::C0(_, _, h1401, _, _, _, _) => {
                            h1401
                        }
                    }, match v3.clone() {
                        Adt126::C0(_, _, _, h1402, _, _, _) => {
                            h1402
                        }
                    }, match v3 {
                        Adt126::C0(_, _, _, _, h1403, _, _) => {
                            h1403
                        }
                    }, true, 0i64)))
                } else {
                    if f522(v2) {
                        f524(v0, v1, v2, Adt126::C0(match v3.clone() {
                            Adt126::C0(h1404, _, _, _, _, _, _) => {
                                h1404
                            }
                        }, match v3.clone() {
                            Adt126::C0(_, h1405, _, _, _, _, _) => {
                                h1405
                            }
                        }, match v3.clone() {
                            Adt126::C0(_, _, h1406, _, _, _, _) => {
                                h1406
                            }
                        }, match v3.clone() {
                            Adt126::C0(_, _, _, h1407, _, _, _) => {
                                h1407
                            }
                        }, match v3 {
                            Adt126::C0(_, _, _, _, h1408, _, _) => {
                                h1408
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f434(Adt102::C7(v1)))
                    }
                }
            }
        }
        Adt127::C6 => {
            if f522(v2) {
                f524(v0, v1, v2, v3.clone(), match v3 {
                    Adt126::C0(_, _, _, _, _, h1409, _) => {
                        h1409
                    }
                })
            } else {
                Ok(f434(Adt102::C7(v1)))
            }
        }
        Adt127::C7 => {
            if f522(v2) {
                f524(v0, v1, v2, v3.clone(), match v3 {
                    Adt126::C0(_, _, _, _, _, h1410, _) => {
                        h1410
                    }
                })
            } else {
                f523(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f438(v0: u64) -> bool {
    let a1411 = v0;
    let a1412 = 9u64;
    if nat_eq(a1411, a1412) {
        true
    } else {
        let a1413 = v0;
        let a1414 = 32u64;
        nat_eq(a1413, a1414)
    }
}

pub fn f439(v0: u64) -> bool {
    let a1415 = 47u64;
    let a1416 = v0;
    if nat_lt(a1415, a1416) {
        let a1417 = v0;
        let a1418 = 58u64;
        nat_lt(a1417, a1418)
    } else {
        false
    }
}

pub fn f440(v0: u64) -> bool {
    let a1419 = 48u64;
    let a1420 = v0;
    if if nat_le(a1419, a1420) {
        let a1421 = v0;
        let a1422 = 57u64;
        nat_le(a1421, a1422)
    } else {
        false
    } {
        true
    } else {
        let a1423 = 65u64;
        let a1424 = v0;
        if if nat_le(a1423, a1424) {
            let a1425 = v0;
            let a1426 = 90u64;
            nat_le(a1425, a1426)
        } else {
            false
        } {
            true
        } else {
            let a1427 = 97u64;
            let a1428 = v0;
            if if nat_le(a1427, a1428) {
                let a1429 = v0;
                let a1430 = 122u64;
                nat_le(a1429, a1430)
            } else {
                false
            } {
                true
            } else {
                let a1431 = v0;
                let a1432 = 33u64;
                if nat_eq(a1431, a1432) {
                    true
                } else {
                    let a1433 = v0;
                    let a1434 = 35u64;
                    if nat_eq(a1433, a1434) {
                        true
                    } else {
                        let a1435 = v0;
                        let a1436 = 36u64;
                        if nat_eq(a1435, a1436) {
                            true
                        } else {
                            let a1437 = v0;
                            let a1438 = 37u64;
                            if nat_eq(a1437, a1438) {
                                true
                            } else {
                                let a1439 = v0;
                                let a1440 = 38u64;
                                if nat_eq(a1439, a1440) {
                                    true
                                } else {
                                    let a1441 = v0;
                                    let a1442 = 39u64;
                                    if nat_eq(a1441, a1442) {
                                        true
                                    } else {
                                        let a1443 = v0;
                                        let a1444 = 42u64;
                                        if nat_eq(a1443, a1444) {
                                            true
                                        } else {
                                            let a1445 = v0;
                                            let a1446 = 43u64;
                                            if nat_eq(a1445, a1446) {
                                                true
                                            } else {
                                                let a1447 = v0;
                                                let a1448 = 45u64;
                                                if nat_eq(a1447, a1448) {
                                                    true
                                                } else {
                                                    let a1449 = v0;
                                                    let a1450 = 46u64;
                                                    if nat_eq(a1449, a1450) {
                                                        true
                                                    } else {
                                                        let a1451 = v0;
                                                        let a1452 = 94u64;
                                                        if nat_eq(a1451, a1452) {
                                                            true
                                                        } else {
                                                            let a1453 = v0;
                                                            let a1454 = 95u64;
                                                            if nat_eq(a1453, a1454) {
                                                                true
                                                            } else {
                                                                let a1455 = v0;
                                                                let a1456 = 96u64;
                                                                if nat_eq(a1455, a1456) {
                                                                    true
                                                                } else {
                                                                    let a1457 = v0;
                                                                    let a1458 = 124u64;
                                                                    if nat_eq(a1457, a1458) {
                                                                        true
                                                                    } else {
                                                                        let a1459 = v0;
                                                                        let a1460 = 126u64;
                                                                        nat_eq(a1459, a1460)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
    let a1461 = 48u64;
    let a1462 = v0;
    if if nat_le(a1461, a1462) {
        let a1463 = v0;
        let a1464 = 57u64;
        nat_le(a1463, a1464)
    } else {
        false
    } {
        true
    } else {
        let a1465 = 65u64;
        let a1466 = v0;
        if if nat_le(a1465, a1466) {
            let a1467 = v0;
            let a1468 = 90u64;
            nat_le(a1467, a1468)
        } else {
            false
        } {
            true
        } else {
            let a1469 = 97u64;
            let a1470 = v0;
            if if nat_le(a1469, a1470) {
                let a1471 = v0;
                let a1472 = 122u64;
                nat_le(a1471, a1472)
            } else {
                false
            } {
                true
            } else {
                let a1473 = v0;
                let a1474 = 95u64;
                if nat_eq(a1473, a1474) {
                    true
                } else {
                    let a1475 = v0;
                    let a1476 = 45u64;
                    if nat_eq(a1475, a1476) {
                        true
                    } else {
                        let a1477 = v0;
                        let a1478 = 46u64;
                        nat_eq(a1477, a1478)
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
    let m1479 = v0;
    match m1479 {
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
    let m1480 = f525(v0.clone(), v1, v2.clone());
    match m1480 {
        None => {
            let m1481 = f526(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1481 {
                Adt152::C0(v8) => {
                    Ok(Err::<(List<Adt109>, Adt129), Adt97>(v8))
                }
                Adt152::C1(v9) => {
                    Ok(Ok::<(List<Adt109>, Adt129), Adt97>((f527(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt109>, Adt129), Adt97>(v7))
        }
    }
}

pub fn f445(v0: Str) -> R<bool> {
    let m1483 = f499(f177({
        let a1482 = v0;
        utf8_encode(a1482)
    })?)?;
    match m1483 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f446(v0: Adt38) -> bool {
    let m1485 = match v0 {
        Adt38::C0(_, _, h1484) => {
            h1484
        }
    };
    match m1485 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f447(v0: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1487 = match v0.clone() {
        Adt50::C0(_, h1486) => {
            h1486
        }
    };
    match m1487.uncons() {
        None => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(match v0 {
                Adt50::C0(h1488, _) => {
                    h1488
                }
            })))
        }
        Some((_, _)) => {
            if f528(match v0.clone() {
                Adt50::C0(_, h1489) => {
                    h1489
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
    let m1493 = {
        let m1492 = match v1.clone() {
            Adt50::C0(h1491, _) => {
                h1491
            }
        };
        match m1492 {
            Adt38::C0(_, h1490, _) => {
                h1490
            }
        }
    };
    match m1493 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1496 = f209(Adt6::C5(match v0.clone() {
                Adt5::C0(h1494) => {
                    h1494
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1495) => {
                    h1495
                }
            }), v2)?;
            match m1496 {
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
    let m1497 = f529(v1.clone(), v2.clone())?;
    match m1497 {
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
            f530(v0, v1, v2, v3)
        }
    }
}

pub fn f452(v0: List<Adt6>, v1: Adt6, v2: List<Adt111>, v3: u64, v4: List<Adt113>) -> Adt130 {
    let m1498 = v0;
    match m1498.uncons() {
        None => {
            let a1499 = v3;
            let a1500 = 1u64;
            if nat_eq(a1499, a1500) {
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
    let m1501 = v0;
    match m1501 {
        None => {
            Adt130::C0(Adt112::C0, v3)
        }
        Some(v4) => {
            Adt130::C0(Adt112::C0, List::cons(Adt113::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f454(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m1502 = f298(Str::lit("anyOf"), v0);
    match m1502 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m1503 = f223(v3);
            match m1503 {
                None => {
                    Adt142::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1504 = v4;
                    match m1504.uncons() {
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
    let m1505 = f298(Str::lit("oneOf"), v0);
    match m1505 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m1506 = f223(v3);
            match m1506 {
                None => {
                    Adt142::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1507 = v4;
                    match m1507.uncons() {
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
    let m1508 = f298(Str::lit("not"), v0);
    match m1508 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            Adt142::C3(List::cons(Adt113::C0(v3, v1, v2.clone()), List::cons(Adt113::C3(v2), List::<Adt113>::nil())))
        }
    }
}

pub fn f457(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m1509 = f298(Str::lit("if"), v0.clone());
    match m1509 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            Adt142::C3(List::cons(Adt113::C0(v3, v1.clone(), v2.clone()), List::cons(Adt113::C4(f531(Str::lit("then"), v0.clone()), f531(Str::lit("else"), v0), v1, v2), List::<Adt113>::nil())))
        }
    }
}

pub fn f458(v0: List<Adt6>, v1: Adt6, v2: List<Adt111>, v3: List<Adt113>) -> Adt130 {
    let m1510 = v0;
    match m1510.uncons() {
        None => {
            Adt130::C0(Adt112::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt130::C0(Adt112::C0, List::cons(Adt113::C0(v4, v1.clone(), v2.clone()), List::cons(Adt113::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f459(v0: Adt111) -> R<Str> {
    let m1511 = v0;
    match m1511 {
        Adt111::C0(v1) => {
            f532(v1)
        }
        Adt111::C1(v2) => {
            let a1512 = f221(v2, 0i64)?;
            Ok(format_i64(a1512))
        }
    }
}

pub fn f460(v0: Str) -> R<Str> {
    let m1515 = {
        let a1514 = f389(f179(), f337(f533(f177({
            let a1513 = v0;
            utf8_encode(a1513)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1514)
    };
    match m1515 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f461(p1518: List<(Str, Adt6)>, p1519: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1517 = (p1518, p1519);
    {
        let mut m1517 = m1517;
        loop {
            let step = {
                let (v0, v1) = m1517;
                let m1516 = v0;
                match m1516.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Adt39) -> R<Option<Adt6>> {
    let m1520 = v0;
    match m1520 {
        Adt39::C0(v1) => {
            Ok(Some(f534(v1)))
        }
        Adt39::C1(v2) => {
            Ok(Some(f535(Str::lit("image"), match v2.clone() {
                Adt43::C0(h1521, _, _, _) => {
                    h1521
                }
            }, match v2.clone() {
                Adt43::C0(_, h1522, _, _) => {
                    h1522
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h1523, _) => {
                    h1523
                }
            }, match v2 {
                Adt43::C0(_, _, _, h1524) => {
                    h1524
                }
            })))
        }
        Adt39::C2(v3) => {
            Ok(Some(f535(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h1525, _, _, _) => {
                    h1525
                }
            }, match v3.clone() {
                Adt44::C0(_, h1526, _, _) => {
                    h1526
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h1527, _) => {
                    h1527
                }
            }, match v3 {
                Adt44::C0(_, _, _, h1528) => {
                    h1528
                }
            })))
        }
        Adt39::C3(v4) => {
            Ok(Some(f536(v4)?))
        }
        Adt39::C4(v5) => {
            Ok(Some(f537(v5)))
        }
    }
}

pub fn f463(v0: Adt5) -> Option<Adt64> {
    let m1529 = f464(Str::lit("taskId"), v0.clone());
    match m1529 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m1530 = f464(Str::lit("createdAt"), v0.clone());
            match m1530 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let m1531 = f464(Str::lit("lastUpdatedAt"), v0.clone());
                    match m1531 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v3) => {
                            Some(Adt64::C0(v1, f464(Str::lit("statusMessage"), v0.clone()), v2, v3, f384(Str::lit("ttlMs"), v0.clone()), f384(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f464(v0: Str, v1: Adt5) -> Option<Str> {
    let m1532 = f214(v0, v1);
    match m1532 {
        Adt107::C0 => {
            None::<Str>
        }
        Adt107::C1(v2) => {
            f141(v2)
        }
    }
}

pub fn f465(v0: Str, v1: Adt5) -> Option<List<(Str, Adt65)>> {
    let m1533 = f382(v0, v1);
    match m1533 {
        None => {
            None::<List<(Str, Adt65)>>
        }
        Some(v2) => {
            f538(match v2 {
                Adt5::C0(h1534) => {
                    h1534
                }
            }, List::<(Str, Adt65)>::nil())
        }
    }
}

pub fn f466(v0: Adt5) -> Option<Adt81> {
    let m1535 = f384(Str::lit("code"), v0.clone());
    match m1535 {
        None => {
            None::<Adt81>
        }
        Some(v1) => {
            let m1536 = f464(Str::lit("message"), v0.clone());
            match m1536 {
                None => {
                    None::<Adt81>
                }
                Some(v2) => {
                    Some(Adt81::C0(v1, v2, {
                        let m1537 = f214(Str::lit("data"), v0);
                        match m1537 {
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
    let m1538 = f382(Str::lit("completes"), v0.clone());
    match m1538 {
        None => {
            let m1539 = f382(Str::lit("fails"), v0);
            match m1539 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m1540 = f466(v2);
                    match m1540 {
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

pub fn f468(p1546: List<u64>, p1547: List<u64>) -> Option<List<u64>> {
    let m1545 = (p1546, p1547);
    {
        let mut m1545 = m1545;
        loop {
            let step = {
                let (v0, v1) = m1545;
                let m1541 = v0;
                match m1541.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1542 = v1;
                        match m1542.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1543 = v2;
                                let a1544 = v4;
                                if nat_eq(a1543, a1544) {
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
                Ok(next) => { m1545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: Str) -> R<List<u64>> {
    f177({
        let a1548 = v0;
        utf8_encode(a1548)
    })
}

pub fn f470() -> Str {
    Str::lit("test://template/")
}

pub fn f471(v0: List<u64>) -> List<u64> {
    f337(v0, List::<u64>::nil())
}

pub fn f472() -> Str {
    Str::lit("data")
}

pub fn f473(v0: List<u64>) -> bool {
    let m1549 = v0;
    match m1549.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f474(p1554: List<u64>) -> bool {
    let m1553 = p1554;
    {
        let mut m1553 = m1553;
        loop {
            let step = {
                let v0 = m1553;
                let m1550 = v0;
                match m1550.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1551 = v1;
                        let a1552 = 47u64;
                        if nat_eq(a1551, a1552) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(p1559: Bytes, p1560: List<Bytes>) -> Bytes {
    let m1558 = (p1559, p1560);
    {
        let mut m1558 = m1558;
        loop {
            let step = {
                let (v0, v1) = m1558;
                let m1555 = v1;
                match m1555.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1556 = v2;
                            let a1557 = v0;
                            append_bytes(a1556, a1557)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(p1563: List<Bytes>, p1564: List<u64>) -> List<Bytes> {
    let m1562 = (p1563, p1564);
    {
        let mut m1562 = m1562;
        loop {
            let step = {
                let (v0, v1) = m1562;
                let m1561 = v1;
                match m1561.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f539(f540(v2), v0), v3))
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

pub fn f477(p1570: List<u64>, p1571: List<u64>) -> bool {
    let m1569 = (p1570, p1571);
    {
        let mut m1569 = m1569;
        loop {
            let step = {
                let (v0, v1) = m1569;
                let m1565 = v0;
                match m1565.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1566 = v1;
                        match m1566.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1567 = v2;
                                let a1568 = v4;
                                if nat_eq(a1567, a1568) {
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
                Ok(next) => { m1569 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: u64, v1: Adt6, v2: List<Adt115>, v3: List<u64>) -> R<Adt144> {
    let m1572 = v1;
    match m1572 {
        Adt6::C0 => {
            Ok(Adt144::C1(v2, f337(f541(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt144::C1(v2, f337(if v4 {
                f541(0u64)
            } else {
                f541(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f542(v5.clone())? {
                Ok(Adt144::C1(v2, f337(f543(v5)?, v3)))
            } else {
                Ok(Adt144::C0(Adt102::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt144::C1(v2, f337(f479(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1573 = v0;
            let a1574 = f544();
            if nat_lt(a1573, a1574) {
                Ok(Adt144::C1(f545(91u64, Adt115::C2({
                    let a1575 = v0;
                    let a1576 = 1u64;
                    nat_add(a1575, a1576)?
                }, v7), v2), v3))
            } else {
                Ok(Adt144::C0(Adt102::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1577 = v0;
            let a1578 = f544();
            if nat_lt(a1577, a1578) {
                Ok(Adt144::C1(f545(123u64, Adt115::C4({
                    let a1579 = v0;
                    let a1580 = 1u64;
                    nat_add(a1579, a1580)?
                }, v8), v2), v3))
            } else {
                Ok(Adt144::C0(Adt102::C12))
            }
        }
    }
}

pub fn f479(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1582 = f337(f546(f177({
            let a1581 = v0;
            utf8_encode(a1581)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1583 = List::cons(34u64, List::<u64>::nil());
        append_list(a1582, a1583)
    }))
}

pub fn f480(v0: Adt33) -> R<Option<List<(Str, Adt6)>>> {
    let m1584 = v0;
    match m1584 {
        Adt33::C0(v1) => {
            Ok(Some(f547(v1)?))
        }
        Adt33::C1(v2) => {
            Ok(Some(f548(v2)?))
        }
        Adt33::C2(v3) => {
            f216(f217(v3)?)
        }
        Adt33::C3(v4) => {
            f216(f217(match v4 {
                Adt50::C0(h1585, _) => {
                    h1585
                }
            })?)
        }
        Adt33::C4(v5) => {
            Ok(Some(f549(v5)?))
        }
        Adt33::C5(v6) => {
            Ok(Some(f550(v6)?))
        }
        Adt33::C6(v7) => {
            Ok(Some(f551(v7)?))
        }
        Adt33::C7(v8) => {
            Ok(Some(f552(v8)?))
        }
        Adt33::C8(v9) => {
            f216(f553(v9)?)
        }
        Adt33::C9(v10) => {
            Ok(Some(f554(v10)?))
        }
        Adt33::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt33::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt33::C12(v12) => {
            f216(f555(v12))
        }
        Adt33::C13(v13) => {
            f216(f556({
                let m1588 = match v13 {
                    Adt83::C0(_, _, h1587) => {
                        h1587
                    }
                };
                match m1588 {
                    Adt85::C0(h1586, _, _) => {
                        h1586
                    }
                }
            }))
        }
        Adt33::C14(v14) => {
            Ok(f557(v14))
        }
    }
}

pub fn f481(v0: Adt134) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f558(f559(match v0.clone() {
        Adt134::C0(h1589, _, _) => {
            h1589
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f560({
        let m1592 = match v0 {
            Adt134::C0(_, h1591, _) => {
                h1591
            }
        };
        match m1592 {
            Adt135::C0(h1590, _) => {
                h1590
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f482(v0: Option<Option<List<u64>>>, v1: Adt94) -> Adt94 {
    let m1593 = v0;
    match m1593 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt94::C0(match v1.clone() {
                Adt94::C0(h1594, _, _, _, _) => {
                    h1594
                }
            }, match v1.clone() {
                Adt94::C0(_, h1595, _, _, _) => {
                    h1595
                }
            }, match v1.clone() {
                Adt94::C0(_, _, h1596, _, _) => {
                    h1596
                }
            }, match v1 {
                Adt94::C0(_, _, _, h1597, _) => {
                    h1597
                }
            }, v2)
        }
    }
}

pub fn f483(v0: Adt14, v1: Adt88) -> R<Option<Option<List<u64>>>> {
    let m1599 = f154(match v0.clone() {
        Adt14::C0(_, _, _, h1598) => {
            h1598
        }
    })?;
    match m1599 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1602 = f561(f562(match v1 {
                Adt88::C0(h1600, _, _) => {
                    h1600
                }
            }, v2, match v0 {
                Adt14::C0(h1601, _, _, _) => {
                    h1601
                }
            })?)?;
            match m1602 {
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

pub fn f484(v0: bool, v1: Adt94, v2: List<List<u64>>) -> Adt94 {
    if f563(v0, match v1.clone() {
        Adt94::C0(h1603, _, _, _, _) => {
            h1603
        }
    }, v2.clone()) {
        f564(match v1.clone() {
            Adt94::C0(h1604, _, _, _, _) => {
                h1604
            }
        }, {
            let a1606 = v2;
            let a1607 = List::cons(match v1 {
                Adt94::C0(_, _, _, h1605, _) => {
                    h1605
                }
            }, List::<List<u64>>::nil());
            append_list(a1606, a1607)
        })
    } else {
        v1
    }
}

pub fn f485(v0: Adt136) -> R<Option<List<u64>>> {
    let m1608 = f300(f565(v0))?;
    match m1608 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1609 = f181(v1)?;
            match m1609 {
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

pub fn f486(v0: Adt88) -> R<Option<List<List<u64>>>> {
    if f566(v0.clone()) {
        let m1613 = f567({
            let m1612 = match v0 {
                Adt88::C0(h1611, _, _) => {
                    h1611
                }
            };
            match m1612 {
                Adt85::C0(h1610, _, _) => {
                    h1610
                }
            }
        });
        match m1613 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1614 = f300(v1)?;
                match m1614 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1615 = f181(v2)?;
                        match m1615 {
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

pub fn f487() -> u64 {
    404u64
}

pub fn f488(v0: Adt90) -> u64 {
    let m1616 = v0;
    match m1616 {
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
            f568()
        }
        Adt90::C8(_) => {
            f188()
        }
        Adt90::C9(_) => {
            f22()
        }
    }
}

pub fn f489(v0: Adt137) -> i64 {
    let m1617 = v0;
    match m1617 {
        Adt137::C0(_) => {
            f569()
        }
        Adt137::C1(_, _) => {
            f570()
        }
        Adt137::C2(_) => {
            f571()
        }
        Adt137::C3(_) => {
            f572()
        }
        Adt137::C4(v6) => {
            f573(v6)
        }
    }
}

pub fn f490(v0: Adt137) -> Str {
    let m1618 = v0;
    match m1618 {
        Adt137::C0(v1) => {
            f574(v1)
        }
        Adt137::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt137::C2(v4) => {
            f575(v4)
        }
        Adt137::C3(_) => {
            Str::lit("Method not found")
        }
        Adt137::C4(v6) => {
            let m1619 = v6;
            match m1619 {
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

pub fn f491(v0: Adt137) -> R<Option<Adt6>> {
    let m1620 = v0;
    match m1620 {
        Adt137::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt137::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f576(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt137::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt137::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt137::C4(v6) => {
            let m1621 = v6;
            match m1621 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f577(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f492(v0: List<u64>) -> R<bool> {
    let m1622 = f409(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1622.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f578(v2)? {
                Ok(false)
            } else {
                f274(f276(v1), f579())
            }
        }
    }
}

pub fn f493(v0: Adt0, v1: Fn3, v2: u64, v3: Adt145) -> R<Result<Adt134, Adt137>> {
    f580(v0.clone(), v1, v2, v3.clone(), f581(match v0 {
        Adt0::C0(_, _, h1623, _, _, _) => {
            h1623
        }
    }, match v3 {
        Adt145::C0(_, h1624, _, _) => {
            h1624
        }
    }))
}

pub fn f494(v0: Adt148, v1: Adt149) -> Result<Adt145, Adt137> {
    let m1625 = v1;
    match m1625 {
        Adt149::C0(v3) => {
            f582(v3, v0)
        }
        Adt149::C1(v2) => {
            Err::<Adt145, Adt137>(v2)
        }
    }
}

pub fn f495(v0: Str) -> Adt148 {
    let a1626 = v0.clone();
    let a1627 = Str::lit("server/discover");
    if equal(a1626, a1627) {
        Adt148::C1(Adt146::C0)
    } else {
        let a1628 = v0.clone();
        let a1629 = Str::lit("tools/list");
        if equal(a1628, a1629) {
            Adt148::C1(Adt146::C1)
        } else {
            let a1630 = v0.clone();
            let a1631 = Str::lit("tools/call");
            if equal(a1630, a1631) {
                Adt148::C1(Adt146::C2)
            } else {
                let a1632 = v0.clone();
                let a1633 = Str::lit("resources/list");
                if equal(a1632, a1633) {
                    Adt148::C1(Adt146::C3)
                } else {
                    let a1634 = v0.clone();
                    let a1635 = Str::lit("resources/read");
                    if equal(a1634, a1635) {
                        Adt148::C1(Adt146::C4)
                    } else {
                        let a1636 = v0.clone();
                        let a1637 = Str::lit("resources/templates/list");
                        if equal(a1636, a1637) {
                            Adt148::C1(Adt146::C5)
                        } else {
                            let a1638 = v0.clone();
                            let a1639 = Str::lit("prompts/list");
                            if equal(a1638, a1639) {
                                Adt148::C1(Adt146::C6)
                            } else {
                                let a1640 = v0.clone();
                                let a1641 = Str::lit("prompts/get");
                                if equal(a1640, a1641) {
                                    Adt148::C1(Adt146::C7)
                                } else {
                                    let a1642 = v0.clone();
                                    let a1643 = Str::lit("completion/complete");
                                    if equal(a1642, a1643) {
                                        Adt148::C1(Adt146::C8)
                                    } else {
                                        let a1644 = v0.clone();
                                        let a1645 = Str::lit("tasks/get");
                                        if equal(a1644, a1645) {
                                            Adt148::C1(Adt146::C9)
                                        } else {
                                            let a1646 = v0.clone();
                                            let a1647 = Str::lit("tasks/update");
                                            if equal(a1646, a1647) {
                                                Adt148::C1(Adt146::C10)
                                            } else {
                                                let a1648 = v0;
                                                let a1649 = Str::lit("tasks/cancel");
                                                if equal(a1648, a1649) {
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

pub fn f496(v0: Option<Adt116>, v1: List<Str>, v2: Adt108) -> R<Adt149> {
    f583(v0, v1, v2.clone(), f584(match v2 {
        Adt108::C0(_, _, h1650) => {
            h1650
        }
    }))
}

pub fn f497(v0: Str) -> Adt151 {
    let m1651 = f495(v0);
    match m1651 {
        Adt148::C0 => {
            Adt151::C0
        }
        Adt148::C1(v1) => {
            f585(v1)
        }
    }
}

pub fn f498(v0: List<u64>) -> Option<List<u64>> {
    let m1652 = f586(f587(), v0);
    match m1652 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1653 = f586(f588(), f284(v1, List::<u64>::nil()));
            match m1653 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f284(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f499(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1655 = f589({
        let a1654 = v0.clone();
        length_list(a1654)
    }, v0, List::<u64>::nil())?;
    match m1655 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f337(v1, List::<u64>::nil())))
        }
    }
}

pub fn f500(p1658: List<Adt119>, p1659: List<Adt119>) -> List<Adt119> {
    let m1657 = (p1658, p1659);
    {
        let mut m1657 = m1657;
        loop {
            let step = {
                let (v0, v1) = m1657;
                let m1656 = v0;
                match m1656.uncons() {
                    None => {
                        Err::<(List<Adt119>, List<Adt119>), List<Adt119>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt119>, List<Adt119>), List<Adt119>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: List<Adt92>, p1666: List<Adt12>, v2: Option<Str>, p1667: List<Adt119>) -> R<List<Adt119>> {
    let m1665 = (p1666, p1667);
    {
        let mut m1665 = m1665;
        loop {
            let step = {
                let (v1, v3) = m1665;
                let m1660 = v1;
                match m1660.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt119>), R<List<Adt119>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f590(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt119>), R<List<Adt119>>>((v5, List::cons(Adt119::C0(match v4.clone() {
                                Adt12::C0(_, h1661, _, _) => {
                                    h1661
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1662, _) => {
                                    h1662
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1663) => {
                                    h1663
                                }
                            }, f591(v0.clone(), match v4 {
                                Adt12::C0(_, h1664, _, _) => {
                                    h1664
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt119>), R<List<Adt119>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(p1683: u64, p1684: i64, p1685: i64) -> R<Option<i64>> {
    let m1682 = (p1683, (p1684, p1685));
    {
        let mut m1682 = m1682;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1682;
                let m1668 = v0;
                if m1668 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1668 - 1;
                    if f299(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1671 = {
                            let a1669 = v2;
                            let a1670 = 0i64;
                            compare(a1669, a1670)
                        };
                        if match m1671 {
                            Ordering::Less => {
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
                                let a1672 = v1;
                                let a1673 = 10i64;
                                let a1674 = 0i64;
                                int_rem(a1672, a1673, a1674)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1675 = v1;
                                    let a1676 = 10i64;
                                    let a1677 = 0i64;
                                    int_quot(a1675, a1676, a1677)?
                                }, f304(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1678 = v1;
                                let a1679 = 10i64;
                                int_mul(a1678, a1679)?
                            }, {
                                let a1680 = v2;
                                let a1681 = 1i64;
                                int_sub(a1680, a1681)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: i64) -> Option<i64> {
    let m1688 = {
        let a1686 = 9007199254740991i64;
        let a1687 = v0;
        compare(a1686, a1687)
    };
    if if match m1688 {
        Ordering::Less => {
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
        let m1691 = {
            let a1689 = v0;
            let a1690 = -9007199254740991i64;
            compare(a1689, a1690)
        };
        match m1691 {
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

pub fn f504(v0: Adt100, v1: Str, v2: Adt5) -> Result<Adt108, Adt103> {
    let m1692 = f214(Str::lit("params"), v2);
    match m1692 {
        Adt107::C0 => {
            Ok::<Adt108, Adt103>(Adt108::C0(v0, v1, None::<Adt5>))
        }
        Adt107::C1(v3) => {
            let m1693 = f219(v3);
            match m1693 {
                None => {
                    f270(Some(v0), Adt104::C6)
                }
                Some(v4) => {
                    Ok::<Adt108, Adt103>(Adt108::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f505(v0: Adt127) -> bool {
    let m1694 = v0;
    match m1694 {
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

pub fn f506(v0: Adt123, v1: Adt128, v2: Adt124, v3: u64) -> R<Adt141> {
    let a1696 = match v0.clone() {
        Adt123::C0(_, _, h1695) => {
            h1695
        }
    };
    let a1697 = f544();
    if nat_lt(a1696, a1697) {
        Ok(Adt141::C1(Adt123::C0(v2, List::cons(v1, match v0.clone() {
            Adt123::C0(_, h1698, _) => {
                h1698
            }
        }), {
            let a1700 = match v0 {
                Adt123::C0(_, _, h1699) => {
                    h1699
                }
            };
            let a1701 = 1u64;
            nat_add(a1700, a1701)?
        })))
    } else {
        Ok(f434(Adt102::C9(v3)))
    }
}

pub fn f507(v0: bool, v1: Adt127, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt124 {
    Adt124::C8(Adt126::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f508(v0: u64) -> i64 {
    let a1702 = v0;
    let a1703 = 0u64;
    if nat_eq(a1702, a1703) {
        0i64
    } else {
        let a1704 = v0;
        let a1705 = 1u64;
        if nat_eq(a1704, a1705) {
            1i64
        } else {
            let a1706 = v0;
            let a1707 = 2u64;
            if nat_eq(a1706, a1707) {
                2i64
            } else {
                let a1708 = v0;
                let a1709 = 3u64;
                if nat_eq(a1708, a1709) {
                    3i64
                } else {
                    let a1710 = v0;
                    let a1711 = 4u64;
                    if nat_eq(a1710, a1711) {
                        4i64
                    } else {
                        let a1712 = v0;
                        let a1713 = 5u64;
                        if nat_eq(a1712, a1713) {
                            5i64
                        } else {
                            let a1714 = v0;
                            let a1715 = 6u64;
                            if nat_eq(a1714, a1715) {
                                6i64
                            } else {
                                let a1716 = v0;
                                let a1717 = 7u64;
                                if nat_eq(a1716, a1717) {
                                    7i64
                                } else {
                                    let a1718 = v0;
                                    let a1719 = 8u64;
                                    if nat_eq(a1718, a1719) {
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

pub fn f509(v0: Adt123, v1: u64) -> Adt141 {
    let m1721 = match v0.clone() {
        Adt123::C0(_, h1720, _) => {
            h1720
        }
    };
    match m1721.uncons() {
        None => {
            f434(Adt102::C2(v1))
        }
        Some((v2, _)) => {
            let m1722 = v2;
            match m1722 {
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

pub fn f510(v0: Adt123, v1: Adt124) -> Adt123 {
    Adt123::C0(v1, match v0.clone() {
        Adt123::C0(_, h1723, _) => {
            h1723
        }
    }, match v0 {
        Adt123::C0(_, _, h1724) => {
            h1724
        }
    })
}

pub fn f511(v0: Adt123, v1: u64, v2: List<u64>, v3: bool) -> Adt141 {
    let m1726 = {
        let a1725 = f389(f179(), f337(v2, List::<u64>::nil()));
        utf8_decode(a1725)
    };
    match m1726 {
        None => {
            f434(Adt102::C14)
        }
        Some(v4) => {
            if v3 {
                f592(v0, v1, v4)
            } else {
                f517(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f512(v0: u64) -> u64 {
    let a1727 = v0;
    let a1728 = 34u64;
    if nat_eq(a1727, a1728) {
        34u64
    } else {
        let a1729 = v0;
        let a1730 = 92u64;
        if nat_eq(a1729, a1730) {
            92u64
        } else {
            let a1731 = v0;
            let a1732 = 47u64;
            if nat_eq(a1731, a1732) {
                47u64
            } else {
                let a1733 = v0;
                let a1734 = 98u64;
                if nat_eq(a1733, a1734) {
                    8u64
                } else {
                    let a1735 = v0;
                    let a1736 = 102u64;
                    if nat_eq(a1735, a1736) {
                        12u64
                    } else {
                        let a1737 = v0;
                        let a1738 = 110u64;
                        if nat_eq(a1737, a1738) {
                            10u64
                        } else {
                            let a1739 = v0;
                            let a1740 = 114u64;
                            if nat_eq(a1739, a1740) {
                                13u64
                            } else {
                                let a1741 = v0;
                                let a1742 = 116u64;
                                if nat_eq(a1741, a1742) {
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

pub fn f513(v0: u64) -> u64 {
    let a1743 = v0;
    let a1744 = 48u64;
    if nat_lt(a1743, a1744) {
        16u64
    } else {
        let a1745 = v0;
        let a1746 = 58u64;
        if nat_lt(a1745, a1746) {
            let a1747 = v0;
            let a1748 = 48u64;
            nat_sub(a1747, a1748)
        } else {
            let a1749 = v0;
            let a1750 = 65u64;
            if nat_lt(a1749, a1750) {
                16u64
            } else {
                let a1751 = v0;
                let a1752 = 71u64;
                if nat_lt(a1751, a1752) {
                    let a1753 = v0;
                    let a1754 = 55u64;
                    nat_sub(a1753, a1754)
                } else {
                    let a1755 = v0;
                    let a1756 = 97u64;
                    if nat_lt(a1755, a1756) {
                        16u64
                    } else {
                        let a1757 = v0;
                        let a1758 = 103u64;
                        if nat_lt(a1757, a1758) {
                            let a1759 = v0;
                            let a1760 = 87u64;
                            nat_sub(a1759, a1760)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f514(v0: Adt123, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt141> {
    let a1761 = v3;
    let a1762 = 0u64;
    if nat_eq(a1761, a1762) {
        if f425(v2, 55296u64, 56319u64) {
            Ok(f433(v0, Adt124::C6(Adt125::C3(v2), v4, v5)))
        } else {
            if f425(v2, 56320u64, 57343u64) {
                Ok(f434(Adt102::C6(v1)))
            } else {
                Ok(f433(v0, Adt124::C6(Adt125::C0, f593(v2, v4)?, v5)))
            }
        }
    } else {
        if f425(v2, 56320u64, 57343u64) {
            Ok(f433(v0, Adt124::C6(Adt125::C0, f593({
                let a1771 = 65536u64;
                let a1772 = {
                    let a1769 = {
                        let a1765 = {
                            let a1763 = v3;
                            let a1764 = 55296u64;
                            nat_sub(a1763, a1764)
                        };
                        let a1766 = 1024u64;
                        nat_mul(a1765, a1766)?
                    };
                    let a1770 = {
                        let a1767 = v2;
                        let a1768 = 56320u64;
                        nat_sub(a1767, a1768)
                    };
                    nat_add(a1769, a1770)?
                };
                nat_add(a1771, a1772)?
            }, v4)?, v5)))
        } else {
            Ok(f434(Adt102::C6(v1)))
        }
    }
}

pub fn f515(v0: u64, v1: u64) -> u64 {
    let m1775 = {
        let a1773 = f516(v0);
        let a1774 = v1;
        index_list(a1773, a1774)
    };
    match m1775 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f516(v0: u64) -> List<u64> {
    let a1776 = v0;
    let a1777 = 0u64;
    if nat_eq(a1776, a1777) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1778 = v0;
        let a1779 = 1u64;
        if nat_eq(a1778, a1779) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f517(v0: Adt123, v1: Adt6) -> Adt141 {
    Adt141::C1(f427(v1, match v0.clone() {
        Adt123::C0(_, h1780, _) => {
            h1780
        }
    }, match v0 {
        Adt123::C0(_, _, h1781) => {
            h1781
        }
    }))
}

pub fn f518(v0: u64) -> Adt6 {
    let a1782 = v0;
    let a1783 = 0u64;
    if nat_eq(a1782, a1783) {
        Adt6::C1(true)
    } else {
        let a1784 = v0;
        let a1785 = 1u64;
        if nat_eq(a1784, a1785) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f519(v0: Adt123, v1: Adt126, v2: Adt127) -> Adt141 {
    f433(v0, Adt124::C8(Adt126::C0(match v1.clone() {
        Adt126::C0(h1786, _, _, _, _, _, _) => {
            h1786
        }
    }, v2, match v1.clone() {
        Adt126::C0(_, _, h1787, _, _, _, _) => {
            h1787
        }
    }, match v1.clone() {
        Adt126::C0(_, _, _, h1788, _, _, _) => {
            h1788
        }
    }, match v1.clone() {
        Adt126::C0(_, _, _, _, h1789, _, _) => {
            h1789
        }
    }, match v1.clone() {
        Adt126::C0(_, _, _, _, _, h1790, _) => {
            h1790
        }
    }, match v1 {
        Adt126::C0(_, _, _, _, _, _, h1791) => {
            h1791
        }
    })))
}

pub fn f520(v0: Adt123, v1: u64, v2: u64, v3: Adt126, v4: bool) -> R<Adt141> {
    let v5: u64 = {
        let a1793 = match v3.clone() {
            Adt126::C0(_, _, _, h1792, _, _, _) => {
                h1792
            }
        };
        let a1794 = 0u64;
        if if nat_eq(a1793, a1794) {
            let a1795 = v2;
            let a1796 = 48u64;
            nat_eq(a1795, a1796)
        } else {
            false
        } {
            0u64
        } else {
            let a1798 = match v3.clone() {
                Adt126::C0(_, _, _, h1797, _, _, _) => {
                    h1797
                }
            };
            let a1799 = 1u64;
            nat_add(a1798, a1799)?
        }
    };
    let v6: i64 = if v4 {
        f304(match v3.clone() {
            Adt126::C0(_, _, _, _, h1800, _, _) => {
                h1800
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt126::C0(_, _, _, _, h1801, _, _) => {
                h1801
            }
        }
    };
    let a1802 = f594();
    let a1803 = v5;
    if nat_lt(a1802, a1803) {
        Ok(f434(Adt102::C8(v1)))
    } else {
        let m1806 = {
            let a1804 = 99999i64;
            let a1805 = v6;
            compare(a1804, a1805)
        };
        if match m1806 {
            Ordering::Less => {
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
            Ok(f433(v0, f507(match v3.clone() {
                Adt126::C0(h1807, _, _, _, _, _, _) => {
                    h1807
                }
            }, if v4 {
                Adt127::C4
            } else {
                Adt127::C2
            }, f304({
                let a1809 = match v3.clone() {
                    Adt126::C0(_, _, h1808, _, _, _, _) => {
                        h1808
                    }
                };
                let a1810 = 10i64;
                int_mul(a1809, a1810)?
            }, f508({
                let a1811 = v2;
                let a1812 = 48u64;
                nat_sub(a1811, a1812)
            }))?, v5, v6, match v3.clone() {
                Adt126::C0(_, _, _, _, _, h1813, _) => {
                    h1813
                }
            }, match v3 {
                Adt126::C0(_, _, _, _, _, _, h1814) => {
                    h1814
                }
            })))
        }
    }
}

pub fn f521(v0: u64) -> bool {
    let a1815 = v0;
    let a1816 = 101u64;
    if nat_eq(a1815, a1816) {
        true
    } else {
        let a1817 = v0;
        let a1818 = 69u64;
        nat_eq(a1817, a1818)
    }
}

pub fn f522(v0: u64) -> bool {
    f425(v0, 48u64, 57u64)
}

pub fn f523(v0: Adt123, v1: u64, v2: u64, v3: Adt126) -> R<Adt141> {
    let m1819 = f426(v3)?;
    match m1819 {
        Adt147::C0 => {
            Ok(f434(Adt102::C7(v1)))
        }
        Adt147::C1 => {
            Ok(f434(Adt102::C8(v1)))
        }
        Adt147::C2(v4) => {
            Ok(f431(f427(v4, match v0.clone() {
                Adt123::C0(_, h1820, _) => {
                    h1820
                }
            }, match v0 {
                Adt123::C0(_, _, h1821) => {
                    h1821
                }
            }), v1, v2))
        }
    }
}

pub fn f524(v0: Adt123, v1: u64, v2: u64, v3: Adt126, v4: bool) -> R<Adt141> {
    let v5: i64 = f304({
        let a1823 = match v3.clone() {
            Adt126::C0(_, _, _, _, _, _, h1822) => {
                h1822
            }
        };
        let a1824 = 10i64;
        int_mul(a1823, a1824)?
    }, f508({
        let a1825 = v2;
        let a1826 = 48u64;
        nat_sub(a1825, a1826)
    }))?;
    let m1829 = {
        let a1827 = 99999i64;
        let a1828 = v5;
        compare(a1827, a1828)
    };
    if match m1829 {
        Ordering::Less => {
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
        Ok(f433(v0, f507(match v3.clone() {
            Adt126::C0(h1830, _, _, _, _, _, _) => {
                h1830
            }
        }, Adt127::C7, match v3.clone() {
            Adt126::C0(_, _, h1831, _, _, _, _) => {
                h1831
            }
        }, match v3.clone() {
            Adt126::C0(_, _, _, h1832, _, _, _) => {
                h1832
            }
        }, match v3 {
            Adt126::C0(_, _, _, _, h1833, _, _) => {
                h1833
            }
        }, v4, v5)))
    }
}

pub fn f525(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1834 = f595(f596(), v2.clone());
    match m1834 {
        None => {
            let m1835 = f597(f598(), v2.clone());
            match m1835 {
                None => {
                    f599(v0, v1, v2)
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

pub fn f526(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt129) -> R<Adt152> {
    let m1836 = f298(Str::lit("x-mcp-header"), v1.clone());
    match m1836 {
        Adt107::C0 => {
            Ok(Adt152::C1(v4))
        }
        Adt107::C1(v5) => {
            f600(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f527(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt109>) -> List<Adt109> {
    f601(Str::lit("properties"), v0.clone(), v1, v2, f601(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f602(Str::lit("items"), v0.clone(), f602(Str::lit("additionalProperties"), v0.clone(), f603(Str::lit("allOf"), v0.clone(), f603(Str::lit("anyOf"), v0.clone(), f603(Str::lit("oneOf"), v0.clone(), f602(Str::lit("not"), v0.clone(), f602(Str::lit("if"), v0.clone(), f602(Str::lit("then"), v0.clone(), f602(Str::lit("else"), v0, v3)))))))))))
}

pub fn f528(v0: List<Adt51>) -> R<bool> {
    let m1837 = v0;
    match m1837.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f604(v2, match v1 {
                Adt51::C0(h1838, _, _) => {
                    h1838
                }
            })
        }
    }
}

pub fn f529(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    Ok(f605(List::cons((Str::lit("type"), f606(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f607(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f608(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f609(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f609(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f609(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f609(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1839 = f610(v0.clone(), v1.clone())?;
        let a1840 = List::cons((Str::lit("required"), f611(v0, v1)), List::<(Str, Adt153)>::nil());
        append_list(a1839, a1840)
    })))))))))
}

pub fn f530(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt111>) -> R<Adt142> {
    Ok(f612(f613(v0, v1.clone(), v2.clone(), v3.clone()), f612(f614(v1.clone(), v2.clone(), v3.clone()), f612(f615(v1.clone(), v2.clone(), v3.clone()), f612(f616(v1.clone(), v2.clone(), v3.clone())?, f612(f617(v1.clone(), v2.clone(), v3.clone()), f612(f618(Adt114::C0, f454(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f612(f618(Adt114::C1, f455(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f612(f618(Adt114::C2, f456(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f618(Adt114::C3, f457(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f531(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1841 = f298(v0, v1);
    match m1841 {
        Adt107::C0 => {
            None::<Adt6>
        }
        Adt107::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f532(v0: Str) -> R<Str> {
    let m1844 = {
        let a1843 = f389(f179(), f337(f619(f177({
            let a1842 = v0.clone();
            utf8_encode(a1842)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1843)
    };
    match m1844 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f533(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f620(v0, 0u64, v1)
}

pub fn f534(v0: Adt40) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt40::C0(h1845, _, _) => {
            h1845
        }
    })), {
        let a1848 = f621(match v0.clone() {
            Adt40::C0(_, h1846, _) => {
                h1846
            }
        });
        let a1849 = f622(match v0 {
            Adt40::C0(_, _, h1847) => {
                h1847
            }
        });
        append_list(a1848, a1849)
    })))
}

pub fn f535(v0: Str, v1: Str, v2: Str, v3: Option<Adt41>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a1850 = f621(v3);
        let a1851 = f622(v4);
        append_list(a1850, a1851)
    }))))
}

pub fn f536(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a1875 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h1852, _, _, _, _, _, _, _, _) => {
                h1852
            }
        })), List::<(Str, Adt6)>::nil()));
        let a1876 = {
            let a1873 = f623(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h1853, _, _, _, _, _, _, _) => {
                    h1853
                }
            });
            let a1874 = {
                let a1871 = f624(match v0.clone() {
                    Adt45::C0(_, _, h1854, _, _, _, _, _, _) => {
                        h1854
                    }
                });
                let a1872 = {
                    let a1869 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h1855, _, _, _, _, _) => {
                            h1855
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a1870 = {
                        let a1867 = f623(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h1856, _, _, _, _) => {
                                h1856
                            }
                        });
                        let a1868 = {
                            let a1865 = f623(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h1857, _, _, _) => {
                                    h1857
                                }
                            });
                            let a1866 = {
                                let a1863 = f621(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h1858, _, _) => {
                                        h1858
                                    }
                                });
                                let a1864 = {
                                    let a1861 = f625(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h1859, _) => {
                                            h1859
                                        }
                                    })?;
                                    let a1862 = f622(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h1860) => {
                                            h1860
                                        }
                                    });
                                    append_list(a1861, a1862)
                                };
                                append_list(a1863, a1864)
                            };
                            append_list(a1865, a1866)
                        };
                        append_list(a1867, a1868)
                    };
                    append_list(a1869, a1870)
                };
                append_list(a1871, a1872)
            };
            append_list(a1873, a1874)
        };
        append_list(a1875, a1876)
    }))
}

pub fn f537(v0: Adt46) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f626(match v0.clone() {
        Adt46::C0(h1877, _, _) => {
            h1877
        }
    })), {
        let a1880 = f621(match v0.clone() {
            Adt46::C0(_, h1878, _) => {
                h1878
            }
        });
        let a1881 = f622(match v0 {
            Adt46::C0(_, _, h1879) => {
                h1879
            }
        });
        append_list(a1880, a1881)
    })))
}

pub fn f538(p1887: List<(Str, Adt6)>, p1888: List<(Str, Adt65)>) -> Option<List<(Str, Adt65)>> {
    let m1886 = (p1887, p1888);
    {
        let mut m1886 = m1886;
        loop {
            let step = {
                let (v0, v1) = m1886;
                let m1882 = v0;
                match m1882.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(Some(f386(v1, List::<(Str, Adt65)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1884 = f627({
                            let (_, h1883) = v2;
                            h1883
                        });
                        match m1884 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(None::<List<(Str, Adt65)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>((v3, List::cons(({
                                    let (h1885, _) = v2;
                                    h1885
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(p1897: Bytes, p1898: List<Bytes>) -> List<Bytes> {
    let m1896 = (p1897, p1898);
    {
        let mut m1896 = m1896;
        loop {
            let step = {
                let (v0, v1) = m1896;
                let m1889 = v1;
                match m1889.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1892 = {
                            let a1890 = v2.clone();
                            length_bytes(a1890)
                        };
                        let a1893 = {
                            let a1891 = v0.clone();
                            length_bytes(a1891)
                        };
                        if nat_eq(a1892, a1893) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1894 = v2;
                                let a1895 = v0;
                                append_bytes(a1894, a1895)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
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

pub fn f540(v0: u64) -> Bytes {
    let m1902 = {
        let a1899 = f628();
        let a1900 = v0;
        let a1901 = 1u64;
        slice_bytes(a1899, a1900, a1901)
    };
    match m1902 {
        None => {
            f179()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f541(v0: u64) -> List<u64> {
    f516(v0)
}

pub fn f542(v0: Adt7) -> R<bool> {
    let a1920 = {
        let m1906 = {
            let a1904 = f629();
            let a1905 = match v0.clone() {
                Adt7::C0(h1903, _) => {
                    h1903
                }
            };
            compare(a1904, a1905)
        };
        if match m1906 {
            Ordering::Less => {
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
            let m1911 = {
                let a1909 = match v0.clone() {
                    Adt7::C0(h1907, _) => {
                        h1907
                    }
                };
                let a1910 = {
                    let a1908 = f629();
                    int_neg(a1908)?
                };
                compare(a1909, a1910)
            };
            if match m1911 {
                Ordering::Less => {
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
                let m1915 = {
                    let a1913 = 9999i64;
                    let a1914 = match v0.clone() {
                        Adt7::C0(_, h1912) => {
                            h1912
                        }
                    };
                    compare(a1913, a1914)
                };
                if match m1915 {
                    Ordering::Less => {
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
                    let m1919 = {
                        let a1917 = match v0 {
                            Adt7::C0(_, h1916) => {
                                h1916
                            }
                        };
                        let a1918 = -9999i64;
                        compare(a1917, a1918)
                    };
                    match m1919 {
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
    Ok(bool_not(a1920))
}

pub fn f543(v0: Adt7) -> R<List<u64>> {
    let m1924 = {
        let a1922 = match v0.clone() {
            Adt7::C0(_, h1921) => {
                h1921
            }
        };
        let a1923 = 0i64;
        compare(a1922, a1923)
    };
    if if match m1924 {
        Ordering::Less => {
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
        let m1928 = {
            let a1926 = 0i64;
            let a1927 = match v0.clone() {
                Adt7::C0(_, h1925) => {
                    h1925
                }
            };
            compare(a1926, a1927)
        };
        match m1928 {
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
        let a1931 = f630(match v0.clone() {
            Adt7::C0(h1929, _) => {
                h1929
            }
        })?;
        let a1932 = List::cons(101u64, f630(match v0 {
            Adt7::C0(_, h1930) => {
                h1930
            }
        })?);
        Ok(append_list(a1931, a1932))
    } else {
        f630(match v0 {
            Adt7::C0(h1933, _) => {
                h1933
            }
        })
    }
}

pub fn f544() -> u64 {
    512u64
}

pub fn f545(v0: u64, v1: Adt115, v2: List<Adt115>) -> List<Adt115> {
    List::cons(Adt115::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f546(p1936: List<u64>, p1937: List<u64>) -> R<List<u64>> {
    let m1935 = (p1936, p1937);
    {
        let mut m1935 = m1935;
        loop {
            let step = {
                let (v0, v1) = m1935;
                let m1934 = v0;
                match m1934.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f337(f631(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1935 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f632(match v0.clone() {
        Adt34::C0(_, h1938, _, _) => {
            h1938
        }
    })), List::cons((Str::lit("capabilities"), f633(match v0.clone() {
        Adt34::C0(_, _, h1939, _) => {
            h1939
        }
    })), {
        let a1942 = f623(Str::lit("instructions"), match v0.clone() {
            Adt34::C0(_, _, _, h1940) => {
                h1940
            }
        });
        let a1943 = f634(match v0 {
            Adt34::C0(h1941, _, _, _) => {
                h1941
            }
        })?;
        append_list(a1942, a1943)
    })))
}

pub fn f548(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f635(match v0.clone() {
        Adt35::C0(_, _, h1944) => {
            h1944
        }
    }))), {
        let a1947 = f623(Str::lit("nextCursor"), match v0.clone() {
            Adt35::C0(_, h1945, _) => {
                h1945
            }
        });
        let a1948 = f634(match v0 {
            Adt35::C0(h1946, _, _) => {
                h1946
            }
        })?;
        append_list(a1947, a1948)
    }))
}

pub fn f549(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f636(match v0.clone() {
        Adt52::C0(_, _, h1949) => {
            h1949
        }
    })?)), {
        let a1952 = f623(Str::lit("nextCursor"), match v0.clone() {
            Adt52::C0(_, h1950, _) => {
                h1950
            }
        });
        let a1953 = f634(match v0 {
            Adt52::C0(h1951, _, _) => {
                h1951
            }
        })?;
        append_list(a1952, a1953)
    }))
}

pub fn f550(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f637(match v0.clone() {
        Adt53::C0(_, h1954) => {
            h1954
        }
    }))), f634(match v0 {
        Adt53::C0(h1955, _) => {
            h1955
        }
    })?))
}

pub fn f551(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f638(match v0.clone() {
        Adt54::C0(_, _, h1956) => {
            h1956
        }
    }))), {
        let a1959 = f623(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h1957, _) => {
                h1957
            }
        });
        let a1960 = f634(match v0 {
            Adt54::C0(h1958, _, _) => {
                h1958
            }
        })?;
        append_list(a1959, a1960)
    }))
}

pub fn f552(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f639(match v0.clone() {
        Adt56::C0(_, _, h1961) => {
            h1961
        }
    }))), {
        let a1964 = f623(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h1962, _) => {
                h1962
            }
        });
        let a1965 = f634(match v0 {
            Adt56::C0(h1963, _, _) => {
                h1963
            }
        })?;
        append_list(a1964, a1965)
    }))
}

pub fn f553(v0: Adt59) -> R<Option<List<(Str, Adt6)>>> {
    let m1967 = f640(match v0.clone() {
        Adt59::C0(_, h1966) => {
            h1966
        }
    })?;
    match m1967 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1969 = f623(Str::lit("description"), match v0 {
                    Adt59::C0(h1968, _) => {
                        h1968
                    }
                });
                let a1970 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1969, a1970)
            }))
        }
    }
}

pub fn f554(v0: Adt61) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f576({
        let m1973 = match v0.clone() {
            Adt61::C0(h1972) => {
                h1972
            }
        };
        match m1973 {
            Adt62::C0(h1971, _, _) => {
                h1971
            }
        }
    }))), {
        let a1980 = f641(Str::lit("total"), {
            let m1976 = match v0.clone() {
                Adt61::C0(h1975) => {
                    h1975
                }
            };
            match m1976 {
                Adt62::C0(_, h1974, _) => {
                    h1974
                }
            }
        })?;
        let a1981 = f303(Str::lit("hasMore"), {
            let m1979 = match v0 {
                Adt61::C0(h1978) => {
                    h1978
                }
            };
            match m1979 {
                Adt62::C0(_, _, h1977) => {
                    h1977
                }
            }
        });
        append_list(a1980, a1981)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f555(v0: Adt82) -> Option<List<(Str, Adt6)>> {
    let m1982 = f642(f643(v0.clone()));
    match m1982 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1984 = {
                    let m1983 = f643(v0.clone());
                    match m1983.uncons() {
                        None => {
                            List::<(Str, Adt6)>::nil()
                        }
                        Some((_, _)) => {
                            List::cons((Str::lit("inputRequests"), v1), List::<(Str, Adt6)>::nil())
                        }
                    }
                };
                let a1985 = f644(Str::lit("requestState"), f645(v0));
                append_list(a1984, a1985)
            })
        }
    }
}

pub fn f556(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1986 = f646(v0.clone());
    match m1986 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1987 = f647(f308(v0.clone()), f648(f649(v0)));
                let a1988 = v1;
                append_list(a1987, a1988)
            })
        }
    }
}

pub fn f557(v0: Adt88) -> Option<List<(Str, Adt6)>> {
    let m1990 = match v0.clone() {
        Adt88::C0(_, h1989, _) => {
            h1989
        }
    };
    match m1990 {
        Adt89::C0 => {
            f556({
                let m1993 = match v0 {
                    Adt88::C0(h1992, _, _) => {
                        h1992
                    }
                };
                match m1993 {
                    Adt85::C0(h1991, _, _) => {
                        h1991
                    }
                }
            })
        }
        Adt89::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f558(v0: Adt154) -> Str {
    let m1994 = v0;
    match m1994 {
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

pub fn f559(v0: Adt33) -> Adt154 {
    let m1995 = v0;
    match m1995 {
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

pub fn f560(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1996 = v0;
    match m1996 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f650(), f651(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f561(v0: List<Adt85>) -> R<Option<List<u64>>> {
    let m1997 = f652(v0);
    match m1997 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1998 = f181(v1)?;
            match m1998 {
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

pub fn f562(v0: Adt85, v1: List<Adt85>, v2: i64) -> R<List<Adt85>> {
    Ok(List::cons(v0.clone(), f306(f653(f225(v0), v1, v2, List::<Adt85>::nil())?, List::<Adt85>::nil())))
}

pub fn f563(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f654(v2) {
            let a1999 = v1;
            let a2000 = 200u64;
            nat_eq(a1999, a2000)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f564(v0: u64, v1: List<List<u64>>) -> Adt94 {
    Adt94::C0(v0, f655(), List::<List<u64>>::nil(), f656(v1), None::<List<u64>>)
}

pub fn f565(v0: Adt136) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f261())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f657(match v0.clone() {
        Adt136::C0(h2001, _) => {
            h2001
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m2004 = match v0.clone() {
            Adt136::C0(_, h2003) => {
                h2003
            }
        };
        match m2004 {
            Adt51::C0(h2002, _, _) => {
                h2002
            }
        }
    })), {
        let a2011 = f658({
            let m2007 = match v0.clone() {
                Adt136::C0(_, h2006) => {
                    h2006
                }
            };
            match m2007 {
                Adt51::C0(_, h2005, _) => {
                    h2005
                }
            }
        });
        let a2012 = f659({
            let m2010 = match v0 {
                Adt136::C0(_, h2009) => {
                    h2009
                }
            };
            match m2010 {
                Adt51::C0(_, _, h2008) => {
                    h2008
                }
            }
        });
        append_list(a2011, a2012)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f566(v0: Adt88) -> bool {
    if match v0.clone() {
        Adt88::C0(_, _, h2013) => {
            h2013
        }
    } {
        let m2015 = match v0 {
            Adt88::C0(_, h2014, _) => {
                h2014
            }
        };
        match m2015 {
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

pub fn f567(v0: Adt63) -> Option<Adt6> {
    let m2016 = f556(v0);
    match m2016 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f568() -> u64 {
    429u64
}

pub fn f569() -> i64 {
    -32602i64
}

pub fn f570() -> i64 {
    -32022i64
}

pub fn f571() -> i64 {
    -32020i64
}

pub fn f572() -> i64 {
    -32601i64
}

pub fn f573(v0: Adt90) -> i64 {
    let m2017 = v0;
    match m2017 {
        Adt90::C0(_) => {
            f569()
        }
        Adt90::C1(_) => {
            f569()
        }
        Adt90::C2(_) => {
            f569()
        }
        Adt90::C3(_) => {
            f569()
        }
        Adt90::C4(_) => {
            f569()
        }
        Adt90::C5(_) => {
            f569()
        }
        Adt90::C6(_) => {
            f660()
        }
        Adt90::C7(_) => {
            f661()
        }
        Adt90::C8(_) => {
            f662()
        }
        Adt90::C9(_) => {
            f660()
        }
    }
}

pub fn f574(v0: Adt138) -> Str {
    let m2018 = v0;
    match m2018 {
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

pub fn f575(v0: Adt117) -> Str {
    let m2019 = v0;
    match m2019 {
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
            let m2020 = v1;
            match m2020 {
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
            let m2021 = v2;
            match m2021 {
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

pub fn f576(v0: List<Str>) -> List<Adt6> {
    f380(f663(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f577(v0: Adt18) -> Adt6 {
    Adt6::C5({
        let a2033 = f664(Str::lit("experimental"), match v0.clone() {
            Adt18::C0(h2022, _, _, _, _) => {
                h2022
            }
        });
        let a2034 = {
            let a2031 = f664(Str::lit("roots"), match v0.clone() {
                Adt18::C0(_, h2023, _, _, _) => {
                    h2023
                }
            });
            let a2032 = {
                let a2029 = f664(Str::lit("sampling"), match v0.clone() {
                    Adt18::C0(_, _, h2024, _, _) => {
                        h2024
                    }
                });
                let a2030 = {
                    let a2027 = f664(Str::lit("elicitation"), match v0.clone() {
                        Adt18::C0(_, _, _, h2025, _) => {
                            h2025
                        }
                    });
                    let a2028 = f664(Str::lit("extensions"), match v0 {
                        Adt18::C0(_, _, _, _, h2026) => {
                            h2026
                        }
                    });
                    append_list(a2027, a2028)
                };
                append_list(a2029, a2030)
            };
            append_list(a2031, a2032)
        };
        append_list(a2033, a2034)
    })
}

pub fn f578(p2037: List<List<u64>>) -> R<bool> {
    let m2036 = p2037;
    {
        let mut m2036 = m2036;
        loop {
            let step = {
                let v0 = m2036;
                let m2035 = v0;
                match m2035.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f665(f276(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2036 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f580(v0: Adt0, v1: Fn3, v2: u64, v3: Adt145, v4: bool) -> R<Result<Adt134, Adt137>> {
    if v4 {
        f666(v0, v1, v2, f667(match v3.clone() {
            Adt145::C0(_, h2038, _, _) => {
                h2038
            }
        }, match v3.clone() {
            Adt145::C0(_, _, h2039, _) => {
                h2039
            }
        }, match v3 {
            Adt145::C0(_, _, _, h2040) => {
                h2040
            }
        }))
    } else {
        Ok(Err::<Adt134, Adt137>(Adt137::C3(f668(match v3 {
            Adt145::C0(_, h2041, _, _) => {
                h2041
            }
        }))))
    }
}

pub fn f581(v0: Adt4, v1: Adt146) -> bool {
    let m2042 = v1;
    match m2042 {
        Adt146::C0 => {
            true
        }
        Adt146::C1 => {
            f669(match v0 {
                Adt4::C0(_, _, _, _, _, h2043, _) => {
                    h2043
                }
            })
        }
        Adt146::C2 => {
            f669(match v0 {
                Adt4::C0(_, _, _, _, _, h2044, _) => {
                    h2044
                }
            })
        }
        Adt146::C3 => {
            f670(match v0 {
                Adt4::C0(_, _, _, _, h2045, _, _) => {
                    h2045
                }
            })
        }
        Adt146::C4 => {
            f670(match v0 {
                Adt4::C0(_, _, _, _, h2046, _, _) => {
                    h2046
                }
            })
        }
        Adt146::C5 => {
            f670(match v0 {
                Adt4::C0(_, _, _, _, h2047, _, _) => {
                    h2047
                }
            })
        }
        Adt146::C6 => {
            f669(match v0 {
                Adt4::C0(_, _, _, h2048, _, _, _) => {
                    h2048
                }
            })
        }
        Adt146::C7 => {
            f669(match v0 {
                Adt4::C0(_, _, _, h2049, _, _, _) => {
                    h2049
                }
            })
        }
        Adt146::C8 => {
            f671(match v0 {
                Adt4::C0(_, _, h2050, _, _, _, _) => {
                    h2050
                }
            })
        }
        Adt146::C9 => {
            f222(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2051) => {
                    h2051
                }
            }, f28())
        }
        Adt146::C10 => {
            f222(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2052) => {
                    h2052
                }
            }, f28())
        }
        Adt146::C11 => {
            f222(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2053) => {
                    h2053
                }
            }, f28())
        }
    }
}

pub fn f582(v0: Adt150, v1: Adt148) -> Result<Adt145, Adt137> {
    let m2054 = v1;
    match m2054 {
        Adt148::C0 => {
            Err::<Adt145, Adt137>(Adt137::C3(match v0 {
                Adt150::C0(_, h2055, _, _) => {
                    h2055
                }
            }))
        }
        Adt148::C1(v2) => {
            Ok::<Adt145, Adt137>(Adt145::C0(match v0.clone() {
                Adt150::C0(h2056, _, _, _) => {
                    h2056
                }
            }, v2, match v0.clone() {
                Adt150::C0(_, _, h2057, _) => {
                    h2057
                }
            }, match v0 {
                Adt150::C0(_, _, _, h2058) => {
                    h2058
                }
            }))
        }
    }
}

pub fn f583(v0: Option<Adt116>, v1: List<Str>, v2: Adt108, v3: Result<Adt17, Adt138>) -> R<Adt149> {
    let m2059 = v3;
    match m2059 {
        Ok(v5) => {
            Ok(f672(v1, v2.clone(), v5.clone(), f673(v0, f497(match v2.clone() {
                Adt108::C0(_, h2060, _) => {
                    h2060
                }
            }), match v2.clone() {
                Adt108::C0(_, h2061, _) => {
                    h2061
                }
            }, match v5 {
                Adt17::C0(h2062, _, _, _, _, _) => {
                    h2062
                }
            }, f674(match v2 {
                Adt108::C0(_, _, h2063) => {
                    h2063
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt149::C1(Adt137::C0(v4)))
        }
    }
}

pub fn f584(v0: Option<Adt5>) -> Result<Adt17, Adt138> {
    let m2064 = v0;
    match m2064 {
        None => {
            Err::<Adt17, Adt138>(Adt138::C0)
        }
        Some(v1) => {
            let m2065 = f214(Str::lit("_meta"), v1);
            match m2065 {
                Adt107::C0 => {
                    Err::<Adt17, Adt138>(Adt138::C1)
                }
                Adt107::C1(v2) => {
                    let m2066 = f219(v2);
                    match m2066 {
                        None => {
                            Err::<Adt17, Adt138>(Adt138::C4)
                        }
                        Some(v3) => {
                            f675(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f585(v0: Adt146) -> Adt151 {
    let m2067 = v0;
    match m2067 {
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

pub fn f586(p2073: List<u64>, p2074: List<u64>) -> Option<List<u64>> {
    let m2072 = (p2073, p2074);
    {
        let mut m2072 = m2072;
        loop {
            let step = {
                let (v0, v1) = m2072;
                let m2068 = v0;
                match m2068.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2069 = v1;
                        match m2069.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2070 = v4;
                                let a2071 = v2;
                                if nat_eq(a2070, a2071) {
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
                Ok(next) => { m2072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f588() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f589(p2083: u64, p2084: List<u64>, p2085: List<u64>) -> R<Option<List<u64>>> {
    let m2082 = (p2083, (p2084, p2085));
    {
        let mut m2082 = m2082;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2082;
                let m2075 = v0;
                if m2075 == 0 {
                    let m2076 = v1;
                    match m2076.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m2075 - 1;
                    let m2077 = v1;
                    match m2077.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m2078 = v7;
                            match m2078.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m2079 = v9;
                                    match m2079.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m2080 = v11;
                                            match m2080.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m2081 = f676(v6, v8, v10, v12, f677(v13.clone()))?;
                                                    match m2081 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f337(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m2082 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: Adt12, v1: Option<Str>) -> bool {
    let m2086 = v1;
    match m2086 {
        None => {
            false
        }
        Some(v2) => {
            let a2088 = match v0 {
                Adt12::C0(h2087, _, _, _) => {
                    h2087
                }
            };
            let a2089 = v2;
            equal(a2088, a2089)
        }
    }
}

pub fn f591(v0: List<Adt92>, v1: Str) -> R<Adt120> {
    let m2092 = f416(f122(v0, {
        let a2090 = f678();
        let a2091 = f679(v1)?;
        append_list(a2090, a2091)
    })?)?;
    match m2092 {
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
            let m2093 = f334(v2);
            match m2093 {
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

pub fn f592(v0: Adt123, v1: u64, v2: Str) -> Adt141 {
    let m2095 = match v0.clone() {
        Adt123::C0(_, h2094, _) => {
            h2094
        }
    };
    match m2095.uncons() {
        None => {
            f434(Adt102::C2(v1))
        }
        Some((v3, v4)) => {
            let m2096 = v3;
            match m2096 {
                Adt128::C0(_) => {
                    f434(Adt102::C2(v1))
                }
                Adt128::C1(v6, _) => {
                    Adt141::C1(Adt123::C0(Adt124::C5, List::cons(Adt128::C1(v6, Some(v2)), v4), match v0 {
                        Adt123::C0(_, _, h2097) => {
                            h2097
                        }
                    }))
                }
            }
        }
    }
}

pub fn f593(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2098 = v0;
    let a2099 = 128u64;
    if nat_lt(a2098, a2099) {
        Ok(List::cons(v0, v1))
    } else {
        let a2100 = v0;
        let a2101 = 2048u64;
        if nat_lt(a2100, a2101) {
            Ok(List::cons({
                let a2102 = 128u64;
                let a2103 = f680(v0, 64u64);
                nat_add(a2102, a2103)?
            }, List::cons({
                let a2104 = 192u64;
                let a2105 = f681(v0, 64u64);
                nat_add(a2104, a2105)?
            }, v1)))
        } else {
            let a2106 = v0;
            let a2107 = 65536u64;
            if nat_lt(a2106, a2107) {
                Ok(List::cons({
                    let a2108 = 128u64;
                    let a2109 = f680(v0, 64u64);
                    nat_add(a2108, a2109)?
                }, List::cons({
                    let a2110 = 128u64;
                    let a2111 = f680(f681(v0, 64u64), 64u64);
                    nat_add(a2110, a2111)?
                }, List::cons({
                    let a2112 = 224u64;
                    let a2113 = f681(v0, 4096u64);
                    nat_add(a2112, a2113)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2114 = 128u64;
                    let a2115 = f680(v0, 64u64);
                    nat_add(a2114, a2115)?
                }, List::cons({
                    let a2116 = 128u64;
                    let a2117 = f680(f681(v0, 64u64), 64u64);
                    nat_add(a2116, a2117)?
                }, List::cons({
                    let a2118 = 128u64;
                    let a2119 = f680(f681(v0, 4096u64), 64u64);
                    nat_add(a2118, a2119)?
                }, List::cons({
                    let a2120 = 240u64;
                    let a2121 = f681(v0, 262144u64);
                    nat_add(a2120, a2121)?
                }, v1)))))
            }
        }
    }
}

pub fn f594() -> u64 {
    18u64
}

pub fn f595(p2125: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2124 = p2125;
    {
        let mut m2124 = m2124;
        loop {
            let step = {
                let v0 = m2124;
                let m2122 = v0;
                match m2122.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2123 = f298(v2.clone(), v1.clone());
                        match m2123 {
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
                Ok(next) => { m2124 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f597(p2132: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2131 = p2132;
    {
        let mut m2131 = m2131;
        loop {
            let step = {
                let v0 = m2131;
                let m2126 = v0;
                match m2126.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2128 = f298({
                            let (h2127, _) = v2.clone();
                            h2127
                        }, v1.clone());
                        match m2128 {
                            Adt107::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt107::C1(v4) => {
                                if f682({
                                    let (_, h2129) = v2;
                                    h2129
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h2130, _) = v2;
                                        h2130
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f599(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt97> {
    let m2133 = f298(Str::lit("$ref"), v2);
    match m2133 {
        Adt107::C0 => {
            None::<Adt97>
        }
        Adt107::C1(v3) => {
            let m2134 = f141(v3);
            match m2134 {
                None => {
                    None::<Adt97>
                }
                Some(v4) => {
                    let m2135 = f683(v1, v4.clone());
                    match m2135 {
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

pub fn f600(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt129) -> R<Adt152> {
    if v3 {
        let m2136 = f141(v2);
        match m2136 {
            None => {
                Ok(Adt152::C0(Adt97::C7(v0)))
            }
            Some(v6) => {
                f684(v0, v1, v6.clone(), f205(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt152::C0(Adt97::C10(v0)))
    }
}

pub fn f601(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt109>) -> List<Adt109> {
    let m2137 = f298(v0.clone(), v1);
    match m2137 {
        Adt107::C0 => {
            v4
        }
        Adt107::C1(v5) => {
            let m2138 = f219(v5);
            match m2138 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a2139 = v0;
                    let a2140 = Str::lit("properties");
                    if equal(a2139, a2140) {
                        f685(match v6 {
                            Adt5::C0(h2141) => {
                                h2141
                            }
                        }, v2, v3, v4)
                    } else {
                        f686(match v6 {
                            Adt5::C0(h2142) => {
                                h2142
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f602(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt109>) -> List<Adt109> {
    let m2143 = f298(v0, v1);
    match m2143 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            List::cons(Adt109::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f603(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt109>) -> List<Adt109> {
    let m2144 = f298(v0, v1);
    match m2144 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            let m2145 = f223(v3);
            match m2145 {
                None => {
                    v2
                }
                Some(v4) => {
                    f687(v4, v2)
                }
            }
        }
    }
}

pub fn f604(p2150: List<Adt51>, p2151: Adt7) -> R<bool> {
    let m2149 = (p2150, p2151);
    {
        let mut m2149 = m2149;
        loop {
            let step = {
                let (v0, v1) = m2149;
                let m2146 = v0;
                match m2146.uncons() {
                    None => {
                        Err::<(List<Adt51>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f688(f689(match v2.clone() {
                            Adt51::C0(h2147, _, _) => {
                                h2147
                            }
                        }, v1)?) {
                            Ok::<(List<Adt51>, Adt7), R<bool>>((v3, match v2 {
                                Adt51::C0(h2148, _, _) => {
                                    h2148
                                }
                            }))
                        } else {
                            Err::<(List<Adt51>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(p2158: List<(Str, Adt153)>) -> Adt142 {
    let m2157 = p2158;
    {
        let mut m2157 = m2157;
        loop {
            let step = {
                let v0 = m2157;
                let m2152 = v0;
                match m2152.uncons() {
                    None => {
                        Err::<List<(Str, Adt153)>, Adt142>(Adt142::C3(List::<Adt113>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m2154 = {
                            let (_, h2153) = v1;
                            h2153
                        };
                        match m2154 {
                            Adt153::C0 => {
                                Ok::<List<(Str, Adt153)>, Adt142>(v2)
                            }
                            Adt153::C1 => {
                                Err::<List<(Str, Adt153)>, Adt142>(Adt142::C0({
                                    let (h2155, _) = v1;
                                    h2155
                                }))
                            }
                            Adt153::C2 => {
                                Err::<List<(Str, Adt153)>, Adt142>(Adt142::C1({
                                    let (h2156, _) = v1;
                                    h2156
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt153> {
    let m2159 = f298(Str::lit("type"), v0);
    match m2159 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v2) => {
            let m2160 = f141(v2.clone());
            match m2160 {
                None => {
                    let m2161 = f223(v2);
                    match m2161 {
                        None => {
                            Ok(Adt153::C2)
                        }
                        Some(v4) => {
                            f692(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f690(v3.clone()) {
                        if f691(v3, v1)? {
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

pub fn f607(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt153> {
    let m2162 = f298(Str::lit("enum"), v0);
    match m2162 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v2) => {
            let m2163 = f223(v2);
            match m2163 {
                None => {
                    Ok(Adt153::C2)
                }
                Some(v3) => {
                    f693(v3, v1)
                }
            }
        }
    }
}

pub fn f608(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt153> {
    let m2164 = f298(Str::lit("const"), v0);
    match m2164 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v2) => {
            let m2165 = f694(v2, v1)?;
            match m2165 {
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

pub fn f609(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt153> {
    let m2166 = f298(v0, v2);
    match m2166 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v4) => {
            let m2167 = f695(v4);
            match m2167 {
                None => {
                    Ok(Adt153::C2)
                }
                Some(v5) => {
                    let m2168 = f695(v3);
                    match m2168 {
                        None => {
                            Ok(Adt153::C0)
                        }
                        Some(v6) => {
                            if f696(v1, f689(v6, v5)?) {
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

pub fn f610(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt153)>> {
    let m2169 = v1;
    match m2169 {
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
            Ok(List::cons((Str::lit("minLength"), f697(Str::lit("minLength"), true, v0.clone(), f698(v4.clone())?)?), List::cons((Str::lit("maxLength"), f697(Str::lit("maxLength"), false, v0, f698(v4)?)?), List::<(Str, Adt153)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f697(Str::lit("minItems"), true, v0.clone(), f699(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f697(Str::lit("maxItems"), false, v0, f699(v5, 0u64)?)?), List::<(Str, Adt153)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f697(Str::lit("minProperties"), true, v0.clone(), f700(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f697(Str::lit("maxProperties"), false, v0, f700(v6, 0u64)?)?), List::<(Str, Adt153)>::nil())))
        }
    }
}

pub fn f611(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt153 {
    let m2170 = f298(Str::lit("required"), v0);
    match m2170 {
        Adt107::C0 => {
            Adt153::C0
        }
        Adt107::C1(v2) => {
            let m2171 = f223(v2);
            match m2171 {
                None => {
                    Adt153::C2
                }
                Some(v3) => {
                    let m2172 = f219(v1);
                    match m2172 {
                        None => {
                            Adt153::C0
                        }
                        Some(v4) => {
                            f701(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f612(v0: Adt142, v1: Adt142) -> Adt142 {
    let m2173 = v0.clone();
    match m2173 {
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
            let m2174 = v1.clone();
            match m2174 {
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
                        let a2175 = v5;
                        let a2176 = v9;
                        append_list(a2175, a2176)
                    })
                }
            }
        }
    }
}

pub fn f613(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt111>) -> Adt142 {
    let m2177 = f298(Str::lit("$ref"), v1);
    match m2177 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v4) => {
            let m2178 = f141(v4);
            match m2178 {
                None => {
                    Adt142::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m2179 = f683(v0, v5.clone());
                    match m2179 {
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

pub fn f614(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m2180 = f298(Str::lit("properties"), v0);
    match m2180 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m2181 = f219(v3);
            match m2181 {
                None => {
                    Adt142::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m2182 = f219(v1);
                    match m2182 {
                        None => {
                            Adt142::C3(List::<Adt113>::nil())
                        }
                        Some(v5) => {
                            Adt142::C3(f702(match v4 {
                                Adt5::C0(h2183) => {
                                    h2183
                                }
                            }, match v5 {
                                Adt5::C0(h2184) => {
                                    h2184
                                }
                            }, v2, List::<Adt113>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f615(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m2185 = f298(Str::lit("additionalProperties"), v0.clone());
    match m2185 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m2186 = f219(v1);
            match m2186 {
                None => {
                    Adt142::C3(List::<Adt113>::nil())
                }
                Some(v4) => {
                    let m2187 = f443(v3.clone());
                    match m2187 {
                        None => {
                            Adt142::C3(f705(match v4 {
                                Adt5::C0(h2189) => {
                                    h2189
                                }
                            }, f704(v0), v3, v2, List::<Adt113>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt142::C3(List::<Adt113>::nil())
                            } else {
                                if f703(match v4 {
                                    Adt5::C0(h2188) => {
                                        h2188
                                    }
                                }, f704(v0)) {
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

pub fn f616(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> R<Adt142> {
    let m2190 = f298(Str::lit("items"), v0);
    match m2190 {
        Adt107::C0 => {
            Ok(Adt142::C3(List::<Adt113>::nil()))
        }
        Adt107::C1(v3) => {
            let m2191 = f223(v1);
            match m2191 {
                None => {
                    Ok(Adt142::C3(List::<Adt113>::nil()))
                }
                Some(v4) => {
                    Ok(Adt142::C3(f706(v4, 0u64, v3, v2, List::<Adt113>::nil())?))
                }
            }
        }
    }
}

pub fn f617(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt111>) -> Adt142 {
    let m2192 = f298(Str::lit("allOf"), v0);
    match m2192 {
        Adt107::C0 => {
            Adt142::C3(List::<Adt113>::nil())
        }
        Adt107::C1(v3) => {
            let m2193 = f223(v3);
            match m2193 {
                None => {
                    Adt142::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m2194 = v4.clone();
                    match m2194.uncons() {
                        None => {
                            Adt142::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt142::C3(f707(v4, v1, v2, List::<Adt113>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f618(v0: Adt114, v1: Adt142, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt111>) -> Adt142 {
    let m2195 = v1.clone();
    match m2195 {
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
            let m2196 = v8;
            match m2196.uncons() {
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

pub fn f619(p2203: List<u64>, p2204: List<u64>) -> List<u64> {
    let m2202 = (p2203, p2204);
    {
        let mut m2202 = m2202;
        loop {
            let step = {
                let (v0, v1) = m2202;
                let m2197 = v0;
                match m2197.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a2198 = v2;
                        let a2199 = 126u64;
                        if nat_eq(a2198, a2199) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a2200 = v2;
                            let a2201 = 47u64;
                            if nat_eq(a2200, a2201) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(p2209: List<u64>, p2210: u64, p2211: List<u64>) -> List<u64> {
    let m2208 = (p2209, (p2210, p2211));
    {
        let mut m2208 = m2208;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2208;
                let m2205 = v0;
                match m2205.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f708(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt155 = f709(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt155::C0(h2206, _) => {
                                h2206
                            }
                        }, match v5 {
                            Adt155::C0(_, h2207) => {
                                h2207
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: Option<Adt41>) -> List<(Str, Adt6)> {
    let m2212 = v0;
    match m2212 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2218 = f710(match v1.clone() {
                    Adt41::C0(h2213, _, _) => {
                        h2213
                    }
                });
                let a2219 = {
                    let a2216 = f711(match v1.clone() {
                        Adt41::C0(_, h2214, _) => {
                            h2214
                        }
                    });
                    let a2217 = f623(Str::lit("lastModified"), match v1 {
                        Adt41::C0(_, _, h2215) => {
                            h2215
                        }
                    });
                    append_list(a2216, a2217)
                };
                append_list(a2218, a2219)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f622(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f664(Str::lit("_meta"), v0)
}

pub fn f623(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2220 = v1;
    match m2220 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f624(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2221 = v0;
    match m2221 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f712(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f625(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2222 = v0;
    match m2222 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f133(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f626(v0: Adt47) -> Adt6 {
    let m2223 = v0;
    match m2223 {
        Adt47::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt48::C0(h2224, _, _, _) => {
                    h2224
                }
            })), {
                let a2228 = f623(Str::lit("mimeType"), match v1.clone() {
                    Adt48::C0(_, h2225, _, _) => {
                        h2225
                    }
                });
                let a2229 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt48::C0(_, _, h2226, _) => {
                        h2226
                    }
                })), f622(match v1 {
                    Adt48::C0(_, _, _, h2227) => {
                        h2227
                    }
                }));
                append_list(a2228, a2229)
            }))
        }
        Adt47::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt49::C0(h2230, _, _, _) => {
                    h2230
                }
            })), {
                let a2234 = f623(Str::lit("mimeType"), match v2.clone() {
                    Adt49::C0(_, h2231, _, _) => {
                        h2231
                    }
                });
                let a2235 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt49::C0(_, _, h2232, _) => {
                        h2232
                    }
                })), f622(match v2 {
                    Adt49::C0(_, _, _, h2233) => {
                        h2233
                    }
                }));
                append_list(a2234, a2235)
            }))
        }
    }
}

pub fn f627(v0: Adt6) -> Option<Adt65> {
    let m2236 = f219(v0);
    match m2236 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2237 = f464(Str::lit("method"), v1.clone());
            match m2237 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a2238 = v2.clone();
                    let a2239 = Str::lit("elicitation/create");
                    if equal(a2238, a2239) {
                        let m2240 = f382(Str::lit("params"), v1);
                        match m2240 {
                            None => {
                                None::<Adt65>
                            }
                            Some(v3) => {
                                f713(v3)
                            }
                        }
                    } else {
                        let a2241 = v2;
                        let a2242 = Str::lit("roots/list");
                        if equal(a2241, a2242) {
                            f714(v1)
                        } else {
                            None::<Adt65>
                        }
                    }
                }
            }
        }
    }
}

pub fn f628() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f629() -> i64 {
    999999999999999999i64
}

pub fn f630(v0: i64) -> R<List<u64>> {
    let m2245 = {
        let a2243 = v0;
        let a2244 = 0i64;
        compare(a2243, a2244)
    };
    if match m2245 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f715(20u64, {
            let a2246 = v0;
            int_neg(a2246)?
        }, List::<u64>::nil())?))
    } else {
        f715(20u64, v0, List::<u64>::nil())
    }
}

pub fn f631(v0: u64) -> R<List<u64>> {
    let a2247 = v0;
    let a2248 = 34u64;
    if nat_eq(a2247, a2248) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2249 = v0;
        let a2250 = 92u64;
        if nat_eq(a2249, a2250) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2251 = v0;
            let a2252 = 8u64;
            if nat_eq(a2251, a2252) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2253 = v0;
                let a2254 = 9u64;
                if nat_eq(a2253, a2254) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2255 = v0;
                    let a2256 = 10u64;
                    if nat_eq(a2255, a2256) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2257 = v0;
                        let a2258 = 12u64;
                        if nat_eq(a2257, a2258) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2259 = v0;
                            let a2260 = 13u64;
                            if nat_eq(a2259, a2260) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2261 = v0;
                                let a2262 = 32u64;
                                if nat_lt(a2261, a2262) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f716(f681(v0, 16u64))?, List::cons(f716(f680(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f632(v0: List<Str>) -> Adt6 {
    Adt6::C4(f576(v0))
}

pub fn f633(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2280 = f664(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2263, _, _, _, _, _, _) => {
                h2263
            }
        });
        let a2281 = {
            let a2278 = f664(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2264, _, _, _, _, _) => {
                    h2264
                }
            });
            let a2279 = {
                let a2276 = f664(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2265, _, _, _, _) => {
                        h2265
                    }
                });
                let a2277 = {
                    let a2274 = f717(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2266, _, _, _) => {
                            h2266
                        }
                    });
                    let a2275 = {
                        let a2272 = f718(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2267, _, _) => {
                                h2267
                            }
                        });
                        let a2273 = {
                            let a2270 = f717(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2268, _) => {
                                    h2268
                                }
                            });
                            let a2271 = f664(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2269) => {
                                    h2269
                                }
                            });
                            append_list(a2270, a2271)
                        };
                        append_list(a2272, a2273)
                    };
                    append_list(a2274, a2275)
                };
                append_list(a2276, a2277)
            };
            append_list(a2278, a2279)
        };
        append_list(a2280, a2281)
    })
}

pub fn f634(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f133(match v0.clone() {
        Adt10::C0(h2282, _) => {
            h2282
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f719(match v0 {
        Adt10::C0(_, h2283) => {
            h2283
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f635(v0: List<Adt36>) -> List<Adt6> {
    f380(f720(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f636(v0: List<Adt45>) -> R<List<Adt6>> {
    Ok(f380(f721(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f637(v0: List<Adt47>) -> List<Adt6> {
    f380(f722(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f638(v0: List<Adt55>) -> List<Adt6> {
    f380(f723(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f639(v0: List<Adt57>) -> List<Adt6> {
    f380(f724(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f640(v0: List<Adt60>) -> R<Option<List<Adt6>>> {
    let m2284 = f725(v0, List::<Adt6>::nil())?;
    match m2284 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f380(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f641(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2285 = v1;
    match m2285 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f133(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f642(v0: List<(Str, Adt65)>) -> Option<Adt6> {
    let m2286 = f726(v0, List::<(Str, Adt6)>::nil());
    match m2286 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f461(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f643(v0: Adt82) -> List<(Str, Adt65)> {
    let m2287 = v0;
    match m2287 {
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

pub fn f644(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2288 = v1;
    match m2288 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f645(v0: Adt82) -> Option<Str> {
    let m2289 = v0;
    match m2289 {
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

pub fn f646(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m2290 = v0;
    match m2290 {
        Adt63::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt63::C1(_, v3) => {
            let m2291 = f642(v3);
            match m2291 {
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
                Adt5::C0(h2292) => {
                    h2292
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f263(v8)), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C4(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f647(v0: Adt64, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt64::C0(h2293, _, _, _, _, _) => {
            h2293
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a2299 = f644(Str::lit("statusMessage"), match v0.clone() {
            Adt64::C0(_, h2294, _, _, _, _) => {
                h2294
            }
        });
        let a2300 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, h2295, _, _, _) => {
                h2295
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, _, h2296, _, _) => {
                h2296
            }
        })), List::cons((Str::lit("ttlMs"), f727(match v0.clone() {
            Adt64::C0(_, _, _, _, h2297, _) => {
                h2297
            }
        })), f728(Str::lit("pollIntervalMs"), match v0 {
            Adt64::C0(_, _, _, _, _, h2298) => {
                h2298
            }
        }))));
        append_list(a2299, a2300)
    }))
}

pub fn f648(v0: Adt84) -> Str {
    let m2301 = v0;
    match m2301 {
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

pub fn f649(v0: Adt63) -> Adt84 {
    let m2302 = v0;
    match m2302 {
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

pub fn f650() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f651(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2303, _, _, _, _, _) => {
            h2303
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2304, _, _, _) => {
            h2304
        }
    })), {
        let a2313 = f623(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2305, _, _, _, _) => {
                h2305
            }
        });
        let a2314 = {
            let a2311 = f623(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2306, _, _) => {
                    h2306
                }
            });
            let a2312 = {
                let a2309 = f623(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2307, _) => {
                        h2307
                    }
                });
                let a2310 = f624(match v0 {
                    Adt1::C0(_, _, _, _, _, h2308) => {
                        h2308
                    }
                });
                append_list(a2309, a2310)
            };
            append_list(a2311, a2312)
        };
        append_list(a2313, a2314)
    })))
}

pub fn f652(v0: List<Adt85>) -> Option<Adt6> {
    let m2315 = f729(v0, List::<Adt6>::nil());
    match m2315 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f380(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f653(v0: Str, p2320: List<Adt85>, v2: i64, p2321: List<Adt85>) -> R<List<Adt85>> {
    let m2319 = (p2320, p2321);
    {
        let mut m2319 = m2319;
        loop {
            let step = {
                let (v1, v3) = m2319;
                let m2316 = v1;
                match m2316.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a2317 = f225(v4.clone());
                        let a2318 = v0.clone();
                        if if equal(a2317, a2318) {
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
                Ok(next) => { m2319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f654(v0: List<List<u64>>) -> bool {
    let m2322 = v0;
    match m2322.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f655() -> List<Adt92> {
    List::cons(Adt92::C0(f730(), f579()), List::cons(Adt92::C0(f731(), f732()), List::cons(Adt92::C0(f733(), f734()), List::<Adt92>::nil())))
}

pub fn f656(v0: List<List<u64>>) -> List<u64> {
    f735(f45(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f657(v0: Adt20) -> Adt6 {
    let m2323 = v0;
    match m2323 {
        Adt20::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt20::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f658(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2324 = v0;
    match m2324 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f659(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2325 = v0;
    match m2325 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f660() -> i64 {
    -32603i64
}

pub fn f661() -> i64 {
    429i64
}

pub fn f662() -> i64 {
    -32021i64
}

pub fn f663(p2328: List<Str>, p2329: List<Adt6>) -> List<Adt6> {
    let m2327 = (p2328, p2329);
    {
        let mut m2327 = m2327;
        loop {
            let step = {
                let (v0, v1) = m2327;
                let m2326 = v0;
                match m2326.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f664(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2330 = v1;
    match m2330 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2331) => {
                    h2331
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f665(v0: List<u64>) -> R<bool> {
    let m2332 = v0;
    match m2332.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2333 = v2;
            match m2333.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2334 = f357(v1)?;
                    let a2335 = 113u64;
                    if if nat_eq(a2334, a2335) {
                        let a2336 = v3;
                        let a2337 = 61u64;
                        nat_eq(a2336, a2337)
                    } else {
                        false
                    } {
                        Ok(f736(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f666(v0: Adt0, v1: Fn3, v2: u64, v3: Result<Adt15, Adt138>) -> R<Result<Adt134, Adt137>> {
    let m2338 = v3;
    match m2338 {
        Ok(v5) => {
            f737(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt134, Adt137>(Adt137::C0(v4)))
        }
    }
}

pub fn f667(v0: Adt146, v1: Adt17, v2: Adt5) -> Result<Adt15, Adt138> {
    let m2339 = v0;
    match m2339 {
        Adt146::C0 => {
            f738(v1, v2)
        }
        Adt146::C1 => {
            f739(v1, v2)
        }
        Adt146::C2 => {
            f740(v1, v2)
        }
        Adt146::C3 => {
            f741(v1, v2)
        }
        Adt146::C4 => {
            f742(v1, v2)
        }
        Adt146::C5 => {
            f743(v1, v2)
        }
        Adt146::C6 => {
            f744(v1, v2)
        }
        Adt146::C7 => {
            f745(v1, v2)
        }
        Adt146::C8 => {
            f746(v1, v2)
        }
        Adt146::C9 => {
            f747(v1, v2)
        }
        Adt146::C10 => {
            f748(v1, v2)
        }
        Adt146::C11 => {
            f749(v1, v2)
        }
    }
}

pub fn f668(v0: Adt146) -> Str {
    let m2340 = v0;
    match m2340 {
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

pub fn f669(v0: Option<Adt8>) -> bool {
    let m2341 = v0;
    match m2341 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f670(v0: Option<Adt9>) -> bool {
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

pub fn f671(v0: Option<Adt5>) -> bool {
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

pub fn f672(v0: List<Str>, v1: Adt108, v2: Adt17, v3: Option<Adt117>) -> Adt149 {
    let m2344 = v3;
    match m2344 {
        None => {
            f750(v1, v2.clone(), f751(match v2 {
                Adt17::C0(h2345, _, _, _, _, _) => {
                    h2345
                }
            }, v0))
        }
        Some(v4) => {
            Adt149::C1(Adt137::C2(v4))
        }
    }
}

pub fn f673(v0: Option<Adt116>, v1: Adt151, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt117>> {
    let m2346 = v0;
    match m2346 {
        None => {
            Ok(None::<Adt117>)
        }
        Some(v5) => {
            let m2348 = match v5.clone() {
                Adt116::C0(_, _, _, h2347, _) => {
                    h2347
                }
            };
            match m2348 {
                None => {
                    f752(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f674(v0: Option<Adt5>) -> Adt5 {
    let m2349 = v0;
    match m2349 {
        None => {
            f27()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f675(v0: Adt5) -> Result<Adt17, Adt138> {
    let m2350 = f753(v0.clone());
    match m2350 {
        Adt158::C0 => {
            f754(v0.clone(), f214(f755(), v0.clone()), f214(f756(), v0))
        }
        Adt158::C1 => {
            Err::<Adt17, Adt138>(Adt138::C2)
        }
        Adt158::C2 => {
            Err::<Adt17, Adt138>(Adt138::C3)
        }
    }
}

pub fn f676(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2351 = f757(v0)?;
    match m2351 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2352 = f757(v1)?;
            match m2352 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2353 = v2;
                    let a2354 = 61u64;
                    if nat_eq(a2353, a2354) {
                        f758(v5, v6, v4, v3)
                    } else {
                        let m2355 = f757(v2)?;
                        match m2355 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2356 = v3;
                                let a2357 = 61u64;
                                if nat_eq(a2356, a2357) {
                                    f759(v5, v6, v7, v4)
                                } else {
                                    let m2358 = f757(v3)?;
                                    match m2358 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f760(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f677(v0: List<u64>) -> bool {
    let m2359 = v0;
    match m2359.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f678() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f679(v0: Str) -> R<List<u64>> {
    f177({
        let a2360 = v0;
        utf8_encode(a2360)
    })
}

pub fn f680(v0: u64, v1: u64) -> u64 {
    let a2361 = v0;
    let a2362 = v1;
    let a2363 = 0u64;
    nat_rem(a2361, a2362, a2363)
}

pub fn f681(v0: u64, v1: u64) -> u64 {
    let a2364 = v0;
    let a2365 = v1;
    let a2366 = 0u64;
    nat_quot(a2364, a2365, a2366)
}

pub fn f682(v0: u64, v1: Adt6) -> bool {
    let a2367 = v0;
    let a2368 = 0u64;
    if nat_eq(a2367, a2368) {
        f761(v1)
    } else {
        let a2369 = v0;
        let a2370 = 1u64;
        if nat_eq(a2369, a2370) {
            f762(v1)
        } else {
            let a2371 = v0;
            let a2372 = 2u64;
            if nat_eq(a2371, a2372) {
                f763(v1)
            } else {
                let a2373 = v0;
                let a2374 = 3u64;
                if nat_eq(a2373, a2374) {
                    f764(v1)
                } else {
                    let a2375 = v0;
                    let a2376 = 4u64;
                    if nat_eq(a2375, a2376) {
                        f765(v1)
                    } else {
                        let a2377 = v0;
                        let a2378 = 5u64;
                        if nat_eq(a2377, a2378) {
                            f766(v1)
                        } else {
                            let a2379 = v0;
                            let a2380 = 6u64;
                            if nat_eq(a2379, a2380) {
                                f767(v1)
                            } else {
                                let a2381 = v0;
                                let a2382 = 7u64;
                                if nat_eq(a2381, a2382) {
                                    f768(v1)
                                } else {
                                    f769(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f683(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a2383 = v1.clone();
    let a2384 = Str::lit("#");
    if equal(a2383, a2384) {
        Some(v0)
    } else {
        let m2385 = f219(v0);
        match m2385 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m2386 = f214(Str::lit("$defs"), v2);
                match m2386 {
                    Adt107::C0 => {
                        None::<Adt6>
                    }
                    Adt107::C1(v3) => {
                        let m2387 = f219(v3);
                        match m2387 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f770(v1, match v4 {
                                    Adt5::C0(h2388) => {
                                        h2388
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

pub fn f684(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt129) -> R<Adt152> {
    let m2389 = v3.clone();
    match m2389.uncons() {
        None => {
            Ok(Adt152::C0(Adt97::C8(v0)))
        }
        Some((_, _)) => {
            if f361(v3.clone(), true) {
                let m2390 = f771(v1);
                match m2390 {
                    None => {
                        Ok(Adt152::C0(Adt97::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f772(v3)?;
                        if f773(v9.clone(), match v5.clone() {
                            Adt129::C0(_, h2391) => {
                                h2391
                            }
                        }) {
                            Ok(Adt152::C0(Adt97::C12(v0, v2)))
                        } else {
                            Ok(Adt152::C1(Adt129::C0(List::cons(Adt12::C0(v0, v2, f328(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt129::C0(h2392, _) => {
                                    h2392
                                }
                            }), List::cons(v9, match v5 {
                                Adt129::C0(_, h2393) => {
                                    h2393
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

pub fn f685(p2398: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2399: List<Adt109>) -> List<Adt109> {
    let m2397 = (p2398, p2399);
    {
        let mut m2397 = m2397;
        loop {
            let step = {
                let (v0, v3) = m2397;
                let m2394 = v0;
                match m2394.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0({
                            let (_, h2395) = v4;
                            h2395
                        }, v1, List::cons({
                            let (h2396, _) = v4;
                            h2396
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2397 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(p2403: List<(Str, Adt6)>, p2404: List<Adt109>) -> List<Adt109> {
    let m2402 = (p2403, p2404);
    {
        let mut m2402 = m2402;
        loop {
            let step = {
                let (v0, v1) = m2402;
                let m2400 = v0;
                match m2400.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v3, List::cons(Adt109::C0({
                            let (_, h2401) = v2;
                            h2401
                        }, false, List::<Str>::nil()), v1)))
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

pub fn f687(p2407: List<Adt6>, p2408: List<Adt109>) -> List<Adt109> {
    let m2406 = (p2407, p2408);
    {
        let mut m2406 = m2406;
        loop {
            let step = {
                let (v0, v1) = m2406;
                let m2405 = v0;
                match m2405.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt109>), List<Adt109>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt109>), List<Adt109>>((v3, List::cons(Adt109::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f688(v0: Adt156) -> bool {
    let m2409 = v0;
    match m2409 {
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

pub fn f689(v0: Adt7, v1: Adt7) -> R<Adt156> {
    f774(v0.clone(), v1.clone(), f775(match v0 {
        Adt7::C0(h2410, _) => {
            h2410
        }
    }), f775(match v1 {
        Adt7::C0(h2411, _) => {
            h2411
        }
    }))
}

pub fn f690(v0: Str) -> bool {
    let a2412 = v0.clone();
    let a2413 = Str::lit("null");
    if equal(a2412, a2413) {
        true
    } else {
        let a2414 = v0.clone();
        let a2415 = Str::lit("boolean");
        if equal(a2414, a2415) {
            true
        } else {
            let a2416 = v0.clone();
            let a2417 = Str::lit("object");
            if equal(a2416, a2417) {
                true
            } else {
                let a2418 = v0.clone();
                let a2419 = Str::lit("array");
                if equal(a2418, a2419) {
                    true
                } else {
                    let a2420 = v0.clone();
                    let a2421 = Str::lit("number");
                    if equal(a2420, a2421) {
                        true
                    } else {
                        let a2422 = v0.clone();
                        let a2423 = Str::lit("string");
                        if equal(a2422, a2423) {
                            true
                        } else {
                            let a2424 = v0;
                            let a2425 = Str::lit("integer");
                            equal(a2424, a2425)
                        }
                    }
                }
            }
        }
    }
}

pub fn f691(v0: Str, v1: Adt6) -> R<bool> {
    let a2426 = v0.clone();
    let a2427 = f776(v1.clone());
    if equal(a2426, a2427) {
        Ok(true)
    } else {
        let a2428 = v0;
        let a2429 = Str::lit("integer");
        if equal(a2428, a2429) {
            let m2430 = f695(v1);
            match m2430 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f777(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f692(p2433: List<Adt6>, v1: Adt6, p2434: bool) -> R<Adt153> {
    let m2432 = (p2433, p2434);
    {
        let mut m2432 = m2432;
        loop {
            let step = {
                let (v0, v2) = m2432;
                let m2431 = v0;
                match m2431.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt153>>(Ok(Adt153::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt153>>(Ok(Adt153::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f690(f778(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt153>>((v4, if v2 {
                                true
                            } else {
                                f691(f778(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt153>>(Ok(Adt153::C2))
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

pub fn f693(p2438: List<Adt6>, v1: Adt6) -> R<Adt153> {
    let m2437 = p2438;
    {
        let mut m2437 = m2437;
        loop {
            let step = {
                let v0 = m2437;
                let m2435 = v0;
                match m2435.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt153>>(Ok(Adt153::C1))
                    }
                    Some((v2, v3)) => {
                        let m2436 = f694(v2, v1.clone())?;
                        match m2436 {
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
                Ok(next) => { m2437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f694(v0: Adt6, v1: Adt6) -> R<Adt157> {
    f779(f780(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f695(v0: Adt6) -> Option<Adt7> {
    let m2439 = v0;
    match m2439 {
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

pub fn f696(v0: u64, v1: Adt156) -> bool {
    let a2440 = v0;
    let a2441 = 0u64;
    if nat_eq(a2440, a2441) {
        let a2442 = f781(v1);
        bool_not(a2442)
    } else {
        let a2443 = v0;
        let a2444 = 1u64;
        if nat_eq(a2443, a2444) {
            let a2445 = f688(v1);
            bool_not(a2445)
        } else {
            let a2446 = v0;
            let a2447 = 2u64;
            if nat_eq(a2446, a2447) {
                f688(v1)
            } else {
                f781(v1)
            }
        }
    }
}

pub fn f697(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt153> {
    let m2448 = f298(v0, v2);
    match m2448 {
        Adt107::C0 => {
            Ok(Adt153::C0)
        }
        Adt107::C1(v4) => {
            let m2449 = f215(v4);
            match m2449 {
                None => {
                    Ok(Adt153::C2)
                }
                Some(v5) => {
                    let m2452 = {
                        let a2450 = v5;
                        let a2451 = 0i64;
                        compare(a2450, a2451)
                    };
                    if match m2452 {
                        Ordering::Less => {
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
                            if f781(f782(f221(v3, 0i64)?, v5)) {
                                Ok(Adt153::C1)
                            } else {
                                Ok(Adt153::C0)
                            }
                        } else {
                            if f688(f782(f221(v3, 0i64)?, v5)) {
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

pub fn f698(v0: Str) -> R<u64> {
    f783(f177({
        let a2453 = v0;
        utf8_encode(a2453)
    })?, 0u64)
}

pub fn f699(p2458: List<Adt6>, p2459: u64) -> R<u64> {
    let m2457 = (p2458, p2459);
    {
        let mut m2457 = m2457;
        loop {
            let step = {
                let (v0, v1) = m2457;
                let m2454 = v0;
                match m2454.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a2455 = v1;
                            let a2456 = 1u64;
                            nat_add(a2455, a2456)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2457 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f700(p2464: List<(Str, Adt6)>, p2465: u64) -> R<u64> {
    let m2463 = (p2464, p2465);
    {
        let mut m2463 = m2463;
        loop {
            let step = {
                let (v0, v1) = m2463;
                let m2460 = v0;
                match m2460.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a2461 = v1;
                            let a2462 = 1u64;
                            nat_add(a2461, a2462)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2463 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f701(p2469: List<Adt6>, v1: Adt5) -> Adt153 {
    let m2468 = p2469;
    {
        let mut m2468 = m2468;
        loop {
            let step = {
                let v0 = m2468;
                let m2466 = v0;
                match m2466.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt153>(Adt153::C0)
                    }
                    Some((v2, v3)) => {
                        let m2467 = f141(v2);
                        match m2467 {
                            None => {
                                Err::<List<Adt6>, Adt153>(Adt153::C2)
                            }
                            Some(v4) => {
                                if f305(v4, v1.clone()) {
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
                Ok(next) => { m2468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(p2476: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt111>, p2477: List<Adt113>) -> List<Adt113> {
    let m2475 = (p2476, p2477);
    {
        let mut m2475 = m2475;
        loop {
            let step = {
                let (v0, v3) = m2475;
                let m2470 = v0;
                match m2470.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2472 = f298({
                            let (h2471, _) = v4.clone();
                            h2471
                        }, v1.clone());
                        match m2472 {
                            Adt107::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v5, v3))
                            }
                            Adt107::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v5, List::cons(Adt113::C0({
                                    let (_, h2473) = v4;
                                    h2473
                                }, v6, List::cons(Adt111::C0({
                                    let (h2474, _) = v4;
                                    h2474
                                }), v2.clone())), v3)))
                            }
                        }
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

pub fn f703(p2482: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2481 = p2482;
    {
        let mut m2481 = m2481;
        loop {
            let step = {
                let v0 = m2481;
                let m2478 = v0;
                match m2478.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2480 = f298({
                            let (h2479, _) = v2;
                            h2479
                        }, v1.clone());
                        match m2480 {
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
                Ok(next) => { m2481 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f704(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2483 = f298(Str::lit("properties"), v0);
    match m2483 {
        Adt107::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt107::C1(v1) => {
            let m2484 = f219(v1);
            match m2484 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2485) => {
                            h2485
                        }
                    }
                }
            }
        }
    }
}

pub fn f705(p2492: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt111>, p2493: List<Adt113>) -> List<Adt113> {
    let m2491 = (p2492, p2493);
    {
        let mut m2491 = m2491;
        loop {
            let step = {
                let (v0, v4) = m2491;
                let m2486 = v0;
                match m2486.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2488 = f298({
                            let (h2487, _) = v5.clone();
                            h2487
                        }, v1.clone());
                        match m2488 {
                            Adt107::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt113>), List<Adt113>>((v6, List::cons(Adt113::C0(v2.clone(), {
                                    let (_, h2489) = v5;
                                    h2489
                                }, List::cons(Adt111::C0({
                                    let (h2490, _) = v5;
                                    h2490
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
                Ok(next) => { m2491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f706(p2498: List<Adt6>, p2499: u64, v2: Adt6, v3: List<Adt111>, p2500: List<Adt113>) -> R<List<Adt113>> {
    let m2497 = (p2498, (p2499, p2500));
    {
        let mut m2497 = m2497;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2497;
                let m2494 = v0;
                match m2494.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt113>)), R<List<Adt113>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt113>)), R<List<Adt113>>>((v6, ({
                            let a2495 = v1;
                            let a2496 = 1u64;
                            nat_add(a2495, a2496)?
                        }, List::cons(Adt113::C0(v2.clone(), v5, List::cons(Adt111::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f707(p2503: List<Adt6>, v1: Adt6, v2: List<Adt111>, p2504: List<Adt113>) -> List<Adt113> {
    let m2502 = (p2503, p2504);
    {
        let mut m2502 = m2502;
        loop {
            let step = {
                let (v0, v3) = m2502;
                let m2501 = v0;
                match m2501.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt113>), List<Adt113>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt113>), List<Adt113>>((v5, List::cons(Adt113::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f708(v0: u64, v1: List<u64>) -> List<u64> {
    let a2505 = v0;
    let a2506 = 0u64;
    if nat_eq(a2505, a2506) {
        v1
    } else {
        let a2507 = v0;
        let a2508 = 1u64;
        if nat_eq(a2507, a2508) {
            List::cons(194u64, v1)
        } else {
            let a2509 = v0;
            let a2510 = 2u64;
            if nat_eq(a2509, a2510) {
                List::cons(226u64, v1)
            } else {
                let a2511 = v0;
                let a2512 = 3u64;
                if nat_eq(a2511, a2512) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f709(v0: u64, v1: u64, v2: List<u64>) -> Adt155 {
    let a2513 = v0;
    let a2514 = 0u64;
    if nat_eq(a2513, a2514) {
        f784(v1, v2)
    } else {
        let a2515 = v0;
        let a2516 = 1u64;
        if nat_eq(a2515, a2516) {
            let a2517 = 128u64;
            let a2518 = v1;
            if if nat_le(a2517, a2518) {
                let a2519 = v1;
                let a2520 = 159u64;
                nat_le(a2519, a2520)
            } else {
                false
            } {
                Adt155::C0(0u64, f785(v2))
            } else {
                f784(v1, List::cons(194u64, v2))
            }
        } else {
            let a2521 = v0;
            let a2522 = 2u64;
            if nat_eq(a2521, a2522) {
                let a2523 = v1;
                let a2524 = 128u64;
                if nat_eq(a2523, a2524) {
                    Adt155::C0(3u64, v2)
                } else {
                    let a2525 = v1;
                    let a2526 = 129u64;
                    if nat_eq(a2525, a2526) {
                        Adt155::C0(4u64, v2)
                    } else {
                        f784(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2527 = v0;
                let a2528 = 3u64;
                if nat_eq(a2527, a2528) {
                    let a2529 = 170u64;
                    let a2530 = v1;
                    if if nat_le(a2529, a2530) {
                        let a2531 = v1;
                        let a2532 = 174u64;
                        nat_le(a2531, a2532)
                    } else {
                        false
                    } {
                        Adt155::C0(0u64, f785(v2))
                    } else {
                        f784(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2533 = 166u64;
                    let a2534 = v1;
                    if if nat_le(a2533, a2534) {
                        let a2535 = v1;
                        let a2536 = 169u64;
                        nat_le(a2535, a2536)
                    } else {
                        false
                    } {
                        Adt155::C0(0u64, f785(v2))
                    } else {
                        f784(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f710(v0: Option<List<Adt42>>) -> List<(Str, Adt6)> {
    let m2537 = v0;
    match m2537 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f786(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f711(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2538 = v0;
    match m2538 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f712(v0: List<Adt2>) -> List<Adt6> {
    f380(f787(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f713(v0: Adt5) -> Option<Adt65> {
    let m2539 = f464(Str::lit("message"), v0.clone());
    match m2539 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2540 = f214(Str::lit("mode"), v0.clone());
            match m2540 {
                Adt107::C0 => {
                    let m2541 = f382(Str::lit("requestedSchema"), v0);
                    match m2541 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v2) => {
                            Some(Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v1, v2))))
                        }
                    }
                }
                Adt107::C1(v3) => {
                    let m2542 = f141(v3);
                    match m2542 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v4) => {
                            let a2543 = v4.clone();
                            let a2544 = Str::lit("url");
                            if equal(a2543, a2544) {
                                let m2545 = f464(Str::lit("url"), v0);
                                match m2545 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v5) => {
                                        Some(Adt65::C0(Adt66::C1(Adt69::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a2546 = v4;
                                let a2547 = Str::lit("form");
                                if equal(a2546, a2547) {
                                    let m2548 = f382(Str::lit("requestedSchema"), v0);
                                    match m2548 {
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

pub fn f714(v0: Adt5) -> Option<Adt65> {
    let m2549 = f214(Str::lit("params"), v0);
    match m2549 {
        Adt107::C0 => {
            Some(Adt65::C2(None::<Adt5>))
        }
        Adt107::C1(v1) => {
            let m2550 = f219(v1);
            match m2550 {
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

pub fn f715(p2562: u64, p2563: i64, p2564: List<u64>) -> R<List<u64>> {
    let m2561 = (p2562, (p2563, p2564));
    {
        let mut m2561 = m2561;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2561;
                let m2551 = v0;
                if m2551 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2551 - 1;
                    let v4: i64 = {
                        let a2552 = v1;
                        let a2553 = 10i64;
                        let a2554 = 0i64;
                        int_quot(a2552, a2553, a2554)?
                    };
                    let v5: i64 = {
                        let a2555 = v1;
                        let a2556 = 10i64;
                        let a2557 = 0i64;
                        int_rem(a2555, a2556, a2557)
                    };
                    let m2560 = {
                        let a2558 = v4;
                        let a2559 = 1i64;
                        compare(a2558, a2559)
                    };
                    if match m2560 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f788(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f788(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2561 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716(v0: u64) -> R<u64> {
    let a2565 = v0;
    let a2566 = 10u64;
    if nat_lt(a2565, a2566) {
        let a2567 = 48u64;
        let a2568 = v0;
        nat_add(a2567, a2568)
    } else {
        let a2569 = 87u64;
        let a2570 = v0;
        nat_add(a2569, a2570)
    }
}

pub fn f717(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2571 = v1;
    match m2571 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f789(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f718(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2572 = v0;
    match m2572 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2575 = f303(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2573, _) => {
                        h2573
                    }
                });
                let a2576 = f303(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2574) => {
                        h2574
                    }
                });
                append_list(a2575, a2576)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f719(v0: Adt11) -> Str {
    let m2577 = v0;
    match m2577 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f720(p2580: List<Adt36>, p2581: List<Adt6>) -> List<Adt6> {
    let m2579 = (p2580, p2581);
    {
        let mut m2579 = m2579;
        loop {
            let step = {
                let (v0, v1) = m2579;
                let m2578 = v0;
                match m2578.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt36>, List<Adt6>), List<Adt6>>((v3, List::cons(f790(v2), v1)))
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

pub fn f721(p2584: List<Adt45>, p2585: List<Adt6>) -> R<List<Adt6>> {
    let m2583 = (p2584, p2585);
    {
        let mut m2583 = m2583;
        loop {
            let step = {
                let (v0, v1) = m2583;
                let m2582 = v0;
                match m2582.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f791(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f722(p2588: List<Adt47>, p2589: List<Adt6>) -> List<Adt6> {
    let m2587 = (p2588, p2589);
    {
        let mut m2587 = m2587;
        loop {
            let step = {
                let (v0, v1) = m2587;
                let m2586 = v0;
                match m2586.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), List<Adt6>>((v3, List::cons(f626(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f723(p2592: List<Adt55>, p2593: List<Adt6>) -> List<Adt6> {
    let m2591 = (p2592, p2593);
    {
        let mut m2591 = m2591;
        loop {
            let step = {
                let (v0, v1) = m2591;
                let m2590 = v0;
                match m2590.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt55>, List<Adt6>), List<Adt6>>((v3, List::cons(f792(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f724(p2596: List<Adt57>, p2597: List<Adt6>) -> List<Adt6> {
    let m2595 = (p2596, p2597);
    {
        let mut m2595 = m2595;
        loop {
            let step = {
                let (v0, v1) = m2595;
                let m2594 = v0;
                match m2594.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f793(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f725(p2601: List<Adt60>, p2602: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2600 = (p2601, p2602);
    {
        let mut m2600 = m2600;
        loop {
            let step = {
                let (v0, v1) = m2600;
                let m2598 = v0;
                match m2598.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2599 = f794(v2)?;
                        match m2599 {
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
                Ok(next) => { m2600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f726(p2608: List<(Str, Adt65)>, p2609: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m2607 = (p2608, p2609);
    {
        let mut m2607 = m2607;
        loop {
            let step = {
                let (v0, v1) = m2607;
                let m2603 = v0;
                match m2603.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2605 = f795({
                            let (_, h2604) = v2;
                            h2604
                        });
                        match m2605 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h2606, _) = v2;
                                    h2606
                                }, v4), v1)))
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

pub fn f727(v0: Option<i64>) -> Adt6 {
    let m2610 = v0;
    match m2610 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f796(v1)
        }
    }
}

pub fn f728(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m2611 = v1;
    match m2611 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f796(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f729(p2615: List<Adt85>, p2616: List<Adt6>) -> Option<List<Adt6>> {
    let m2614 = (p2615, p2616);
    {
        let mut m2614 = m2614;
        loop {
            let step = {
                let (v0, v1) = m2614;
                let m2612 = v0;
                match m2612.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2613 = f797(v2);
                        match m2613 {
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
                Ok(next) => { m2614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f730() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f731() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f732() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f733() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f734() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f735(p2621: List<List<u64>>, p2622: List<u64>) -> List<u64> {
    let m2620 = (p2621, p2622);
    {
        let mut m2620 = m2620;
        loop {
            let step = {
                let (v0, v1) = m2620;
                let m2617 = v0;
                match m2617.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2618 = f798(v2);
                            let a2619 = v1;
                            append_list(a2618, a2619)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f736(v0: List<u64>) -> bool {
    let m2623 = v0;
    match m2623.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2624 = v1;
            let a2625 = 48u64;
            if nat_eq(a2624, a2625) {
                f799(v2)
            } else {
                false
            }
        }
    }
}

pub fn f737(v0: Adt0, v1: Fn3, v2: u64, v3: Adt15) -> R<Result<Adt134, Adt137>> {
    if f800(f801(v3.clone())) {
        Ok(Ok::<Adt134, Adt137>(f802(v0.clone(), Adt33::C0(f803(v0)))))
    } else {
        Ok(f804(v0.clone(), v3.clone(), f805(match v0 {
            Adt0::C0(_, _, _, _, _, h2626) => {
                h2626
            }
        }, v2, v1, v3)?))
    }
}

pub fn f738(v0: Adt17, _: Adt5) -> Result<Adt15, Adt138> {
    Ok::<Adt15, Adt138>(Adt15::C0(Adt16::C0(v0)))
}

pub fn f739(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2627 = f806(Str::lit("cursor"), v1);
    match m2627 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C1(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f740(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2628 = f807(Str::lit("name"), v1.clone());
    match m2628 {
        Ok(v2) => {
            let m2629 = f808(Str::lit("arguments"), v1.clone());
            match m2629 {
                Ok(v3) => {
                    let m2630 = f808(Str::lit("inputResponses"), v1.clone());
                    match m2630 {
                        Ok(v4) => {
                            let m2631 = f806(Str::lit("requestState"), v1);
                            match m2631 {
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

pub fn f741(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2632 = f806(Str::lit("cursor"), v1);
    match m2632 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C3(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f742(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2633 = f807(Str::lit("uri"), v1.clone());
    match m2633 {
        Ok(v2) => {
            let m2634 = f808(Str::lit("inputResponses"), v1.clone());
            match m2634 {
                Ok(v3) => {
                    let m2635 = f806(Str::lit("requestState"), v1);
                    match m2635 {
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

pub fn f743(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2636 = f806(Str::lit("cursor"), v1);
    match m2636 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C5(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f744(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2637 = f806(Str::lit("cursor"), v1);
    match m2637 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C6(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f745(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2638 = f807(Str::lit("name"), v1.clone());
    match m2638 {
        Ok(v2) => {
            let m2639 = f809(Str::lit("arguments"), v1.clone());
            match m2639 {
                Ok(v3) => {
                    let m2640 = f808(Str::lit("inputResponses"), v1.clone());
                    match m2640 {
                        Ok(v4) => {
                            let m2641 = f806(Str::lit("requestState"), v1);
                            match m2641 {
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

pub fn f746(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2642 = f810(Str::lit("ref"), v1.clone());
    match m2642 {
        Ok(v2) => {
            let m2643 = f811(v2);
            match m2643 {
                Ok(v3) => {
                    let m2644 = f810(Str::lit("argument"), v1.clone());
                    match m2644 {
                        Ok(v4) => {
                            let m2645 = f812(v4);
                            match m2645 {
                                Ok(v5) => {
                                    let m2646 = f813(v1);
                                    match m2646 {
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

pub fn f747(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2647 = f807(Str::lit("taskId"), v1);
    match m2647 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C9(Adt30::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f748(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2648 = f807(Str::lit("taskId"), v1.clone());
    match m2648 {
        Ok(v2) => {
            let m2649 = f810(Str::lit("inputResponses"), v1);
            match m2649 {
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

pub fn f749(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt138> {
    let m2650 = f807(Str::lit("taskId"), v1);
    match m2650 {
        Ok(v2) => {
            Ok::<Adt15, Adt138>(Adt15::C11(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt138>(v3)
        }
    }
}

pub fn f750(v0: Adt108, v1: Adt17, v2: Option<Adt137>) -> Adt149 {
    let m2651 = v2;
    match m2651 {
        None => {
            Adt149::C0(Adt150::C0(match v0.clone() {
                Adt108::C0(h2652, _, _) => {
                    h2652
                }
            }, match v0.clone() {
                Adt108::C0(_, h2653, _) => {
                    h2653
                }
            }, v1, f674(match v0 {
                Adt108::C0(_, _, h2654) => {
                    h2654
                }
            })))
        }
        Some(v3) => {
            Adt149::C1(v3)
        }
    }
}

pub fn f751(v0: Str, v1: List<Str>) -> Option<Adt137> {
    f814(f815(v0.clone(), v1.clone()), v0, v1)
}

pub fn f752(v0: Adt116, v1: Adt151, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt117>> {
    let m2656 = match v0.clone() {
        Adt116::C0(h2655, _, _, _, _) => {
            h2655
        }
    };
    match m2656 {
        None => {
            Ok(Some(Adt117::C0))
        }
        Some(v5) => {
            let a2657 = v5;
            let a2658 = v3;
            if equal(a2657, a2658) {
                let m2660 = match v0.clone() {
                    Adt116::C0(_, h2659, _, _, _) => {
                        h2659
                    }
                };
                match m2660 {
                    None => {
                        Ok(Some(Adt117::C1))
                    }
                    Some(v6) => {
                        let a2661 = v6;
                        let a2662 = v2;
                        if equal(a2661, a2662) {
                            f816(f817(match v0.clone() {
                                Adt116::C0(_, _, h2663, _, _) => {
                                    h2663
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt116::C0(_, _, _, _, h2664) => {
                                    h2664
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

pub fn f753(v0: Adt5) -> Adt158 {
    f818(f214(f755(), v0.clone()), f214(f756(), v0))
}

pub fn f754(v0: Adt5, v1: Adt107, v2: Adt107) -> Result<Adt17, Adt138> {
    let m2665 = v1;
    match m2665 {
        Adt107::C0 => {
            Err::<Adt17, Adt138>(Adt138::C2)
        }
        Adt107::C1(v3) => {
            let m2666 = v2;
            match m2666 {
                Adt107::C0 => {
                    Err::<Adt17, Adt138>(Adt138::C3)
                }
                Adt107::C1(v4) => {
                    f819(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f755() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f756() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f757(v0: u64) -> R<Option<u64>> {
    let a2667 = 64u64;
    let a2668 = v0;
    if if nat_lt(a2667, a2668) {
        let a2669 = v0;
        let a2670 = 91u64;
        nat_lt(a2669, a2670)
    } else {
        false
    } {
        Ok(Some({
            let a2671 = v0;
            let a2672 = 65u64;
            nat_sub(a2671, a2672)
        }))
    } else {
        let a2673 = 96u64;
        let a2674 = v0;
        if if nat_lt(a2673, a2674) {
            let a2675 = v0;
            let a2676 = 123u64;
            nat_lt(a2675, a2676)
        } else {
            false
        } {
            Ok(Some({
                let a2677 = v0;
                let a2678 = 71u64;
                nat_sub(a2677, a2678)
            }))
        } else {
            let a2679 = 47u64;
            let a2680 = v0;
            if if nat_lt(a2679, a2680) {
                let a2681 = v0;
                let a2682 = 58u64;
                nat_lt(a2681, a2682)
            } else {
                false
            } {
                Ok(Some({
                    let a2683 = v0;
                    let a2684 = 4u64;
                    nat_add(a2683, a2684)?
                }))
            } else {
                let a2685 = v0;
                let a2686 = 43u64;
                if nat_eq(a2685, a2686) {
                    Ok(Some(62u64))
                } else {
                    let a2687 = v0;
                    let a2688 = 47u64;
                    if nat_eq(a2687, a2688) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f758(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2689 = v3;
        let a2690 = 61u64;
        if nat_eq(a2689, a2690) {
            let a2691 = f820(v1, 16u64);
            let a2692 = 0u64;
            nat_eq(a2691, a2692)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f821(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f759(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2693 = f820(v2, 4u64);
        let a2694 = 0u64;
        nat_eq(a2693, a2694)
    } else {
        false
    } {
        Ok(Some(f822(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f760(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2697 = {
            let a2695 = v0;
            let a2696 = 4u64;
            nat_mul(a2695, a2696)?
        };
        let a2698 = f823(v1, 16u64);
        nat_add(a2697, a2698)?
    }, List::cons({
        let a2701 = {
            let a2699 = f820(v1, 16u64);
            let a2700 = 16u64;
            nat_mul(a2699, a2700)?
        };
        let a2702 = f823(v2, 4u64);
        nat_add(a2701, a2702)?
    }, List::cons({
        let a2705 = {
            let a2703 = f820(v2, 4u64);
            let a2704 = 64u64;
            nat_mul(a2703, a2704)?
        };
        let a2706 = v3;
        nat_add(a2705, a2706)?
    }, List::<u64>::nil()))))
}

pub fn f761(v0: Adt6) -> bool {
    let m2707 = f141(v0);
    match m2707 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f762(v0: Adt6) -> bool {
    let m2708 = f695(v0);
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
    let m2709 = f215(v0);
    match m2709 {
        None => {
            false
        }
        Some(v1) => {
            let a2713 = {
                let m2712 = {
                    let a2710 = v1;
                    let a2711 = 0i64;
                    compare(a2710, a2711)
                };
                match m2712 {
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
            bool_not(a2713)
        }
    }
}

pub fn f764(v0: Adt6) -> bool {
    let m2714 = f223(v0);
    match m2714 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f765(v0: Adt6) -> bool {
    let m2715 = f219(v0);
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
    let m2716 = f223(v0);
    match m2716 {
        None => {
            false
        }
        Some(v1) => {
            f824(v1)
        }
    }
}

pub fn f767(v0: Adt6) -> bool {
    let m2717 = f443(v0);
    match m2717 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f768(v0: Adt6) -> bool {
    let m2718 = f141(v0.clone());
    match m2718 {
        None => {
            let m2719 = f223(v0);
            match m2719 {
                None => {
                    false
                }
                Some(v2) => {
                    f825(v2)
                }
            }
        }
        Some(v1) => {
            f690(v1)
        }
    }
}

pub fn f769(v0: Adt6) -> bool {
    let m2720 = f223(v0);
    match m2720 {
        None => {
            false
        }
        Some(v1) => {
            let m2721 = v1;
            match m2721.uncons() {
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

pub fn f770(v0: Str, p2739: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2738 = p2739;
    {
        let mut m2738 = m2738;
        loop {
            let step = {
                let v1 = m2738;
                let m2722 = v1;
                match m2722.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2726 = v0.clone();
                        let a2727 = {
                            let a2724 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2723, _) = v2.clone();
                                h2723
                            }, List::<Str>::nil()));
                            let a2725 = Str::lit("");
                            join(a2724, a2725)
                        };
                        if equal(a2726, a2727) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2728) = v2;
                                h2728
                            }))
                        } else {
                            let m2730 = f219({
                                let (_, h2729) = v2.clone();
                                h2729
                            });
                            match m2730 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2731 = f214(Str::lit("$anchor"), v4);
                                    match m2731 {
                                        Adt107::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt107::C1(v5) => {
                                            let m2732 = f141(v5);
                                            match m2732 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2735 = v0.clone();
                                                    let a2736 = {
                                                        let a2733 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2734 = Str::lit("");
                                                        join(a2733, a2734)
                                                    };
                                                    if equal(a2735, a2736) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2737) = v2;
                                                            h2737
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
                Ok(next) => { m2738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f771(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2740 = f298(Str::lit("type"), v0);
    match m2740 {
        Adt107::C0 => {
            None::<Adt13>
        }
        Adt107::C1(v1) => {
            let m2741 = f141(v1);
            match m2741 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f826(v2)
                }
            }
        }
    }
}

pub fn f772(v0: List<u64>) -> R<List<u64>> {
    Ok(f337(f827(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f773(v0: List<u64>, p2744: List<List<u64>>) -> bool {
    let m2743 = p2744;
    {
        let mut m2743 = m2743;
        loop {
            let step = {
                let v1 = m2743;
                let m2742 = v1;
                match m2742.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f828(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f774(v0: Adt7, v1: Adt7, v2: Adt156, v3: Adt156) -> R<Adt156> {
    let m2745 = v2;
    match m2745 {
        Adt156::C0 => {
            let m2747 = v3;
            match m2747 {
                Adt156::C0 => {
                    Ok(f829(Adt156::C0, f830(f831(match v0.clone() {
                        Adt7::C0(h2748, _) => {
                            h2748
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2749) => {
                            h2749
                        }
                    }, f831(match v1.clone() {
                        Adt7::C0(h2750, _) => {
                            h2750
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2751) => {
                            h2751
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
            let m2746 = v3;
            match m2746 {
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
            let m2752 = v3;
            match m2752 {
                Adt156::C0 => {
                    Ok(Adt156::C2)
                }
                Adt156::C1 => {
                    Ok(Adt156::C2)
                }
                Adt156::C2 => {
                    f830(match v0.clone() {
                        Adt7::C0(h2753, _) => {
                            h2753
                        }
                    }, match v0 {
                        Adt7::C0(_, h2754) => {
                            h2754
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2755, _) => {
                            h2755
                        }
                    }, match v1 {
                        Adt7::C0(_, h2756) => {
                            h2756
                        }
                    })
                }
            }
        }
    }
}

pub fn f775(v0: i64) -> Adt156 {
    f782(v0, 0i64)
}

pub fn f776(v0: Adt6) -> Str {
    let m2757 = v0;
    match m2757 {
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

pub fn f777(v0: Adt7) -> R<bool> {
    if f299(match v0.clone() {
        Adt7::C0(h2758, _) => {
            h2758
        }
    }) {
        Ok(true)
    } else {
        f832(40u64, match v0.clone() {
            Adt7::C0(h2759, _) => {
                h2759
            }
        }, match v0 {
            Adt7::C0(_, h2760) => {
                h2760
            }
        })
    }
}

pub fn f778(v0: Adt6) -> Str {
    let m2761 = f141(v0);
    match m2761 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f779(p2768: u64, p2769: List<(Adt6, Adt6)>) -> R<Adt157> {
    let m2767 = (p2768, p2769);
    {
        let mut m2767 = m2767;
        loop {
            let step = {
                let (v0, v1) = m2767;
                let m2762 = v0;
                if m2762 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt157>>(Ok(Adt157::C2))
                } else {
                    let v2 = m2762 - 1;
                    let m2763 = v1;
                    match m2763.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt157>>(Ok(Adt157::C0))
                        }
                        Some((v3, v4)) => {
                            let m2766 = f833({
                                let (h2764, _) = v3;
                                h2764
                            }, {
                                let (_, h2765) = v3;
                                h2765
                            }, v4)?;
                            match m2766 {
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
                Ok(next) => { m2767 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f780() -> u64 {
    67108864u64
}

pub fn f781(v0: Adt156) -> bool {
    let m2770 = v0;
    match m2770 {
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

pub fn f782(v0: i64, v1: i64) -> Adt156 {
    let m2773 = {
        let a2771 = v0;
        let a2772 = v1;
        compare(a2771, a2772)
    };
    if match m2773 {
        Ordering::Less => {
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
        let m2776 = {
            let a2774 = v1;
            let a2775 = v0;
            compare(a2774, a2775)
        };
        if match m2776 {
            Ordering::Less => {
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

pub fn f783(p2785: List<u64>, p2786: u64) -> R<u64> {
    let m2784 = (p2785, p2786);
    {
        let mut m2784 = m2784;
        loop {
            let step = {
                let (v0, v1) = m2784;
                let m2777 = v0;
                match m2777.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2778 = v2;
                        let a2779 = 128u64;
                        if if nat_lt(a2778, a2779) {
                            true
                        } else {
                            let a2780 = 191u64;
                            let a2781 = v2;
                            nat_lt(a2780, a2781)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2782 = v1;
                                let a2783 = 1u64;
                                nat_add(a2782, a2783)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f784(v0: u64, v1: List<u64>) -> Adt155 {
    let a2787 = v0;
    let a2788 = 32u64;
    if nat_lt(a2787, a2788) {
        if f834(v0) {
            Adt155::C0(0u64, List::cons(v0, v1))
        } else {
            Adt155::C0(0u64, f785(v1))
        }
    } else {
        let a2789 = v0;
        let a2790 = 127u64;
        if nat_eq(a2789, a2790) {
            Adt155::C0(0u64, f785(v1))
        } else {
            let a2791 = v0;
            let a2792 = 194u64;
            if nat_eq(a2791, a2792) {
                Adt155::C0(1u64, v1)
            } else {
                let a2793 = v0;
                let a2794 = 226u64;
                if nat_eq(a2793, a2794) {
                    Adt155::C0(2u64, v1)
                } else {
                    Adt155::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f785(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f786(v0: List<Adt42>) -> List<Adt6> {
    f380(f835(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f787(p2797: List<Adt2>, p2798: List<Adt6>) -> List<Adt6> {
    let m2796 = (p2797, p2798);
    {
        let mut m2796 = m2796;
        loop {
            let step = {
                let (v0, v1) = m2796;
                let m2795 = v0;
                match m2795.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f836(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f788(v0: i64) -> u64 {
    let m2801 = {
        let a2799 = v0;
        let a2800 = 1i64;
        compare(a2799, a2800)
    };
    if match m2801 {
        Ordering::Less => {
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
        let m2804 = {
            let a2802 = v0;
            let a2803 = 2i64;
            compare(a2802, a2803)
        };
        if match m2804 {
            Ordering::Less => {
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
            let m2807 = {
                let a2805 = v0;
                let a2806 = 3i64;
                compare(a2805, a2806)
            };
            if match m2807 {
                Ordering::Less => {
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
                let m2810 = {
                    let a2808 = v0;
                    let a2809 = 4i64;
                    compare(a2808, a2809)
                };
                if match m2810 {
                    Ordering::Less => {
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
                    let m2813 = {
                        let a2811 = v0;
                        let a2812 = 5i64;
                        compare(a2811, a2812)
                    };
                    if match m2813 {
                        Ordering::Less => {
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
                        let m2816 = {
                            let a2814 = v0;
                            let a2815 = 6i64;
                            compare(a2814, a2815)
                        };
                        if match m2816 {
                            Ordering::Less => {
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
                            let m2819 = {
                                let a2817 = v0;
                                let a2818 = 7i64;
                                compare(a2817, a2818)
                            };
                            if match m2819 {
                                Ordering::Less => {
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
                                let m2822 = {
                                    let a2820 = v0;
                                    let a2821 = 8i64;
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
                                    55u64
                                } else {
                                    let m2825 = {
                                        let a2823 = v0;
                                        let a2824 = 9i64;
                                        compare(a2823, a2824)
                                    };
                                    if match m2825 {
                                        Ordering::Less => {
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

pub fn f789(v0: Adt8) -> Adt6 {
    Adt6::C5(f303(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2826) => {
            h2826
        }
    }))
}

pub fn f790(v0: Adt36) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt36::C0(h2827, _, _, _, _, _, _, _) => {
            h2827
        }
    })), {
        let a2845 = f623(Str::lit("title"), match v0.clone() {
            Adt36::C0(_, h2828, _, _, _, _, _, _) => {
                h2828
            }
        });
        let a2846 = {
            let a2843 = f624(match v0.clone() {
                Adt36::C0(_, _, h2829, _, _, _, _, _) => {
                    h2829
                }
            });
            let a2844 = {
                let a2841 = f623(Str::lit("description"), match v0.clone() {
                    Adt36::C0(_, _, _, h2830, _, _, _, _) => {
                        h2830
                    }
                });
                let a2842 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2833 = match v0.clone() {
                        Adt36::C0(_, _, _, _, h2832, _, _, _) => {
                            h2832
                        }
                    };
                    match m2833 {
                        Adt5::C0(h2831) => {
                            h2831
                        }
                    }
                })), {
                    let a2839 = f664(Str::lit("outputSchema"), match v0.clone() {
                        Adt36::C0(_, _, _, _, _, h2834, _, _) => {
                            h2834
                        }
                    });
                    let a2840 = {
                        let a2837 = f837(match v0.clone() {
                            Adt36::C0(_, _, _, _, _, _, h2835, _) => {
                                h2835
                            }
                        });
                        let a2838 = f622(match v0 {
                            Adt36::C0(_, _, _, _, _, _, _, h2836) => {
                                h2836
                            }
                        });
                        append_list(a2837, a2838)
                    };
                    append_list(a2839, a2840)
                });
                append_list(a2841, a2842)
            };
            append_list(a2843, a2844)
        };
        append_list(a2845, a2846)
    }))
}

pub fn f791(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2870 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2847, _, _, _, _, _, _, _, _) => {
                h2847
            }
        })), List::<(Str, Adt6)>::nil());
        let a2871 = {
            let a2868 = f623(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2848, _, _, _, _, _, _, _) => {
                    h2848
                }
            });
            let a2869 = {
                let a2866 = f624(match v0.clone() {
                    Adt45::C0(_, _, h2849, _, _, _, _, _, _) => {
                        h2849
                    }
                });
                let a2867 = {
                    let a2864 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2850, _, _, _, _, _) => {
                            h2850
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2865 = {
                        let a2862 = f623(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2851, _, _, _, _) => {
                                h2851
                            }
                        });
                        let a2863 = {
                            let a2860 = f623(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2852, _, _, _) => {
                                    h2852
                                }
                            });
                            let a2861 = {
                                let a2858 = f621(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2853, _, _) => {
                                        h2853
                                    }
                                });
                                let a2859 = {
                                    let a2856 = f625(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2854, _) => {
                                            h2854
                                        }
                                    })?;
                                    let a2857 = f622(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2855) => {
                                            h2855
                                        }
                                    });
                                    append_list(a2856, a2857)
                                };
                                append_list(a2858, a2859)
                            };
                            append_list(a2860, a2861)
                        };
                        append_list(a2862, a2863)
                    };
                    append_list(a2864, a2865)
                };
                append_list(a2866, a2867)
            };
            append_list(a2868, a2869)
        };
        append_list(a2870, a2871)
    }))
}

pub fn f792(v0: Adt55) -> Adt6 {
    Adt6::C5({
        let a2892 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt55::C0(h2872, _, _, _, _, _, _, _) => {
                h2872
            }
        })), List::<(Str, Adt6)>::nil());
        let a2893 = {
            let a2890 = f623(Str::lit("title"), match v0.clone() {
                Adt55::C0(_, h2873, _, _, _, _, _, _) => {
                    h2873
                }
            });
            let a2891 = {
                let a2888 = f624(match v0.clone() {
                    Adt55::C0(_, _, h2874, _, _, _, _, _) => {
                        h2874
                    }
                });
                let a2889 = {
                    let a2886 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt55::C0(_, _, _, h2875, _, _, _, _) => {
                            h2875
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2887 = {
                        let a2884 = f623(Str::lit("description"), match v0.clone() {
                            Adt55::C0(_, _, _, _, h2876, _, _, _) => {
                                h2876
                            }
                        });
                        let a2885 = {
                            let a2882 = f623(Str::lit("mimeType"), match v0.clone() {
                                Adt55::C0(_, _, _, _, _, h2877, _, _) => {
                                    h2877
                                }
                            });
                            let a2883 = {
                                let a2880 = f621(match v0.clone() {
                                    Adt55::C0(_, _, _, _, _, _, h2878, _) => {
                                        h2878
                                    }
                                });
                                let a2881 = f622(match v0 {
                                    Adt55::C0(_, _, _, _, _, _, _, h2879) => {
                                        h2879
                                    }
                                });
                                append_list(a2880, a2881)
                            };
                            append_list(a2882, a2883)
                        };
                        append_list(a2884, a2885)
                    };
                    append_list(a2886, a2887)
                };
                append_list(a2888, a2889)
            };
            append_list(a2890, a2891)
        };
        append_list(a2892, a2893)
    })
}

pub fn f793(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2909 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2894, _, _, _, _, _) => {
                h2894
            }
        })), List::<(Str, Adt6)>::nil());
        let a2910 = {
            let a2907 = f623(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2895, _, _, _, _) => {
                    h2895
                }
            });
            let a2908 = {
                let a2905 = f624(match v0.clone() {
                    Adt57::C0(_, _, h2896, _, _, _) => {
                        h2896
                    }
                });
                let a2906 = {
                    let a2903 = f623(Str::lit("description"), match v0.clone() {
                        Adt57::C0(_, _, _, h2897, _, _) => {
                            h2897
                        }
                    });
                    let a2904 = {
                        let a2901 = {
                            let m2899 = match v0.clone() {
                                Adt57::C0(_, _, _, _, h2898, _) => {
                                    h2898
                                }
                            };
                            match m2899 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f838(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2902 = f622(match v0 {
                            Adt57::C0(_, _, _, _, _, h2900) => {
                                h2900
                            }
                        });
                        append_list(a2901, a2902)
                    };
                    append_list(a2903, a2904)
                };
                append_list(a2905, a2906)
            };
            append_list(a2907, a2908)
        };
        append_list(a2909, a2910)
    })
}

pub fn f794(v0: Adt60) -> R<Option<Adt6>> {
    let m2912 = f462(match v0.clone() {
        Adt60::C0(_, h2911) => {
            h2911
        }
    })?;
    match m2912 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f839(match v0 {
                Adt60::C0(h2913, _) => {
                    h2913
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f795(v0: Adt65) -> Option<Adt6> {
    let m2914 = v0;
    match m2914 {
        Adt65::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f840(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt65::C1(_) => {
            None::<Adt6>
        }
        Adt65::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m2915 = v3;
                match m2915 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h2916) => {
                                h2916
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f796(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f797(v0: Adt85) -> Option<Adt6> {
    let m2918 = f556(match v0.clone() {
        Adt85::C0(h2917, _, _) => {
            h2917
        }
    });
    match m2918 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m2920 = f841(match v0.clone() {
                Adt85::C0(_, _, h2919) => {
                    h2919
                }
            });
            match m2920 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f796(match v0 {
                        Adt85::C0(_, h2921, _) => {
                            h2921
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f798(v0: List<u64>) -> List<u64> {
    let a2924 = f842();
    let a2925 = {
        let a2922 = v0;
        let a2923 = f843();
        append_list(a2922, a2923)
    };
    append_list(a2924, a2925)
}

pub fn f799(p2932: List<u64>) -> bool {
    let m2931 = p2932;
    {
        let mut m2931 = m2931;
        loop {
            let step = {
                let v0 = m2931;
                let m2926 = v0;
                match m2926.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2927 = v1;
                        let a2928 = 48u64;
                        if if nat_eq(a2927, a2928) {
                            true
                        } else {
                            let a2929 = v1;
                            let a2930 = 46u64;
                            nat_eq(a2929, a2930)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f800(v0: Adt146) -> bool {
    let m2933 = v0;
    match m2933 {
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

pub fn f801(v0: Adt15) -> Adt146 {
    let m2934 = v0;
    match m2934 {
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

pub fn f802(v0: Adt0, v1: Adt33) -> Adt134 {
    Adt134::C0(v1, Adt135::C0(Some(match v0 {
        Adt0::C0(h2935, _, _, _, _, _) => {
            h2935
        }
    }), None::<Adt5>), List::<Adt136>::nil())
}

pub fn f803(v0: Adt0) -> Adt34 {
    Adt34::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2936, _, _) => {
            h2936
        }
    }, f348(), match v0.clone() {
        Adt0::C0(_, _, h2937, _, _, _) => {
            h2937
        }
    }, match v0 {
        Adt0::C0(_, h2938, _, _, _, _) => {
            h2938
        }
    })
}

pub fn f804(v0: Adt0, v1: Adt15, v2: Result<Adt33, Adt90>) -> Result<Adt134, Adt137> {
    let m2939 = v2;
    match m2939 {
        Ok(v4) => {
            if f844(f801(v1.clone()), v4.clone()) {
                f845(v0, v1, v4)
            } else {
                Err::<Adt134, Adt137>(Adt137::C4(f846()))
            }
        }
        Err(v3) => {
            Err::<Adt134, Adt137>(Adt137::C4(v3))
        }
    }
}

pub fn f805(v0: u64, v1: u64, v2: Fn3, v3: Adt15) -> R<Result<Adt33, Adt90>> {
    let m2940 = v3.clone();
    match m2940 {
        Adt15::C0(_) => {
            let c2942: Fn3 = v2;
            c2942.apply(v3)
        }
        Adt15::C1(_) => {
            let c2943: Fn3 = v2;
            c2943.apply(v3)
        }
        Adt15::C2(_) => {
            if f847(v0, v1) {
                let c2941: Fn3 = v2;
                c2941.apply(v3)
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C7(v0)))
            }
        }
        Adt15::C3(_) => {
            let c2944: Fn3 = v2;
            c2944.apply(v3)
        }
        Adt15::C4(_) => {
            let c2945: Fn3 = v2;
            c2945.apply(v3)
        }
        Adt15::C5(_) => {
            let c2946: Fn3 = v2;
            c2946.apply(v3)
        }
        Adt15::C6(_) => {
            let c2947: Fn3 = v2;
            c2947.apply(v3)
        }
        Adt15::C7(_) => {
            let c2948: Fn3 = v2;
            c2948.apply(v3)
        }
        Adt15::C8(_) => {
            let c2949: Fn3 = v2;
            c2949.apply(v3)
        }
        Adt15::C9(_) => {
            let c2950: Fn3 = v2;
            c2950.apply(v3)
        }
        Adt15::C10(_) => {
            let c2951: Fn3 = v2;
            c2951.apply(v3)
        }
        Adt15::C11(_) => {
            let c2952: Fn3 = v2;
            c2952.apply(v3)
        }
    }
}

pub fn f806(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt138> {
    let m2953 = f214(v0.clone(), v1);
    match m2953 {
        Adt107::C0 => {
            Ok::<Option<Str>, Adt138>(None::<Str>)
        }
        Adt107::C1(v2) => {
            let m2954 = f141(v2);
            match m2954 {
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

pub fn f807(v0: Str, v1: Adt5) -> Result<Str, Adt138> {
    let m2955 = f214(v0.clone(), v1);
    match m2955 {
        Adt107::C0 => {
            Err::<Str, Adt138>(Adt138::C7(v0))
        }
        Adt107::C1(v2) => {
            let m2956 = f141(v2);
            match m2956 {
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

pub fn f808(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt138> {
    let m2957 = f214(v0.clone(), v1);
    match m2957 {
        Adt107::C0 => {
            Ok::<Option<Adt5>, Adt138>(None::<Adt5>)
        }
        Adt107::C1(v2) => {
            let m2958 = f219(v2);
            match m2958 {
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

pub fn f809(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt138> {
    let m2959 = f214(v0.clone(), v1);
    match m2959 {
        Adt107::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt138>(None::<List<(Str, Str)>>)
        }
        Adt107::C1(v2) => {
            let m2960 = f219(v2);
            match m2960 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt138>(Adt138::C8(v0))
                }
                Some(v3) => {
                    let m2962 = f848(v0, match v3 {
                        Adt5::C0(h2961) => {
                            h2961
                        }
                    });
                    match m2962 {
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

pub fn f810(v0: Str, v1: Adt5) -> Result<Adt5, Adt138> {
    let m2963 = f214(v0.clone(), v1);
    match m2963 {
        Adt107::C0 => {
            Err::<Adt5, Adt138>(Adt138::C7(v0))
        }
        Adt107::C1(v2) => {
            let m2964 = f219(v2);
            match m2964 {
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

pub fn f811(v0: Adt5) -> Result<Adt26, Adt138> {
    let m2965 = f807(Str::lit("type"), v0.clone());
    match m2965 {
        Ok(v1) => {
            let a2966 = v1.clone();
            let a2967 = Str::lit("ref/prompt");
            if equal(a2966, a2967) {
                let m2968 = f807(Str::lit("name"), v0.clone());
                match m2968 {
                    Ok(v2) => {
                        let m2969 = f806(Str::lit("title"), v0);
                        match m2969 {
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
                let a2970 = v1;
                let a2971 = Str::lit("ref/resource");
                if equal(a2970, a2971) {
                    let m2972 = f807(Str::lit("uri"), v0);
                    match m2972 {
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

pub fn f812(v0: Adt5) -> Result<Adt28, Adt138> {
    let m2973 = f807(Str::lit("name"), v0.clone());
    match m2973 {
        Ok(v1) => {
            let m2974 = f807(Str::lit("value"), v0);
            match m2974 {
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

pub fn f813(v0: Adt5) -> Result<Option<Adt29>, Adt138> {
    let m2975 = f214(Str::lit("context"), v0);
    match m2975 {
        Adt107::C0 => {
            Ok::<Option<Adt29>, Adt138>(None::<Adt29>)
        }
        Adt107::C1(v1) => {
            let m2976 = f219(v1);
            match m2976 {
                None => {
                    Err::<Option<Adt29>, Adt138>(Adt138::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2977 = f809(Str::lit("arguments"), v2);
                    match m2977 {
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

pub fn f814(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt137> {
    if v0 {
        None::<Adt137>
    } else {
        Some(Adt137::C1(v1, v2))
    }
}

pub fn f815(v0: Str, p2982: List<Str>) -> bool {
    let m2981 = p2982;
    {
        let mut m2981 = m2981;
        loop {
            let step = {
                let v1 = m2981;
                let m2978 = v1;
                match m2978.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2979 = v2;
                        let a2980 = v0.clone();
                        if equal(a2979, a2980) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f816(v0: Option<Adt117>, v1: List<Adt119>, v2: Adt5) -> R<Option<Adt117>> {
    let m2983 = v0;
    match m2983 {
        None => {
            f849(v1, f850(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f817(v0: Option<Str>, v1: Adt151, v2: Adt5) -> Option<Adt117> {
    let m2984 = v1.clone();
    match m2984 {
        Adt151::C0 => {
            None::<Adt117>
        }
        Adt151::C1 => {
            f851(v0, f852(v1, v2))
        }
        Adt151::C2 => {
            f851(v0, f852(v1, v2))
        }
        Adt151::C3 => {
            f851(v0, f852(v1, v2))
        }
    }
}

pub fn f818(v0: Adt107, v1: Adt107) -> Adt158 {
    let m2985 = v0;
    match m2985 {
        Adt107::C0 => {
            Adt158::C1
        }
        Adt107::C1(_) => {
            let m2986 = v1;
            match m2986 {
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

pub fn f819(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt17, Adt138> {
    let m2987 = f141(v1);
    match m2987 {
        None => {
            Err::<Adt17, Adt138>(Adt138::C5)
        }
        Some(v3) => {
            let m2988 = f853(v2);
            match m2988 {
                None => {
                    Err::<Adt17, Adt138>(Adt138::C6)
                }
                Some(v4) => {
                    let m2989 = f808(f854(), v0.clone());
                    match m2989 {
                        Ok(v6) => {
                            let m2990 = f855(v0.clone());
                            match m2990 {
                                Ok(v8) => {
                                    let m2991 = f856(v0.clone());
                                    match m2991 {
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

pub fn f820(v0: u64, v1: u64) -> u64 {
    let a2992 = v0;
    let a2993 = v1;
    let a2994 = 0u64;
    nat_rem(a2992, a2993, a2994)
}

pub fn f821(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2997 = {
            let a2995 = v0;
            let a2996 = 4u64;
            nat_mul(a2995, a2996)?
        };
        let a2998 = f823(v1, 16u64);
        nat_add(a2997, a2998)?
    }, List::<u64>::nil()))
}

pub fn f822(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a3001 = {
            let a2999 = v0;
            let a3000 = 4u64;
            nat_mul(a2999, a3000)?
        };
        let a3002 = f823(v1, 16u64);
        nat_add(a3001, a3002)?
    }, List::cons({
        let a3005 = {
            let a3003 = f820(v1, 16u64);
            let a3004 = 16u64;
            nat_mul(a3003, a3004)?
        };
        let a3006 = f823(v2, 4u64);
        nat_add(a3005, a3006)?
    }, List::<u64>::nil())))
}

pub fn f823(v0: u64, v1: u64) -> u64 {
    let a3007 = v0;
    let a3008 = v1;
    let a3009 = 0u64;
    nat_quot(a3007, a3008, a3009)
}

pub fn f824(p3013: List<Adt6>) -> bool {
    let m3012 = p3013;
    {
        let mut m3012 = m3012;
        loop {
            let step = {
                let v0 = m3012;
                let m3010 = v0;
                match m3010.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3011 = f141(v1);
                        match m3011 {
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
                Ok(next) => { m3012 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f825(p3017: List<Adt6>) -> bool {
    let m3016 = p3017;
    {
        let mut m3016 = m3016;
        loop {
            let step = {
                let v0 = m3016;
                let m3014 = v0;
                match m3014.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3015 = f141(v1);
                        match m3015 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f690(v3) {
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
                Ok(next) => { m3016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f826(v0: Str) -> Option<Adt13> {
    let a3018 = v0.clone();
    let a3019 = Str::lit("string");
    if equal(a3018, a3019) {
        Some(Adt13::C0)
    } else {
        let a3020 = v0.clone();
        let a3021 = Str::lit("integer");
        if equal(a3020, a3021) {
            Some(Adt13::C1)
        } else {
            let a3022 = v0;
            let a3023 = Str::lit("boolean");
            if equal(a3022, a3023) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f827(p3032: List<u64>, p3033: List<u64>) -> R<List<u64>> {
    let m3031 = (p3032, p3033);
    {
        let mut m3031 = m3031;
        loop {
            let step = {
                let (v0, v1) = m3031;
                let m3024 = v0;
                match m3024.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a3025 = 65u64;
                        let a3026 = v2;
                        if if nat_le(a3025, a3026) {
                            let a3027 = v2;
                            let a3028 = 90u64;
                            nat_le(a3027, a3028)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a3029 = v2;
                                let a3030 = 32u64;
                                nat_add(a3029, a3030)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3031 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f828(p3040: List<u64>, p3041: List<u64>) -> bool {
    let m3039 = (p3040, p3041);
    {
        let mut m3039 = m3039;
        loop {
            let step = {
                let (v0, v1) = m3039;
                let m3034 = v0;
                match m3034.uncons() {
                    None => {
                        let m3035 = v1;
                        match m3035.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m3036 = v1;
                        match m3036.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a3037 = v4;
                                let a3038 = v6;
                                if nat_eq(a3037, a3038) {
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
                Ok(next) => { m3039 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f829(v0: Adt156, v1: Adt156) -> Adt156 {
    let m3042 = v0;
    match m3042 {
        Adt156::C0 => {
            f857(v1)
        }
        Adt156::C1 => {
            Adt156::C1
        }
        Adt156::C2 => {
            v1
        }
    }
}

pub fn f830(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt156> {
    let m3043 = f782(f858(v0, v1)?, f858(v2, v3)?);
    match m3043 {
        Adt156::C0 => {
            Ok(Adt156::C0)
        }
        Adt156::C1 => {
            f859(v0, f860(v0)?, v2, f860(v2)?)
        }
        Adt156::C2 => {
            Ok(Adt156::C2)
        }
    }
}

pub fn f831(v0: i64) -> R<i64> {
    let m3046 = {
        let a3044 = v0;
        let a3045 = 0i64;
        compare(a3044, a3045)
    };
    if match m3046 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a3047 = v0;
        int_neg(a3047)
    } else {
        Ok(v0)
    }
}

pub fn f832(p3059: u64, p3060: i64, p3061: i64) -> R<bool> {
    let m3058 = (p3059, (p3060, p3061));
    {
        let mut m3058 = m3058;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3058;
                let m3048 = v0;
                if m3048 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m3048 - 1;
                    let m3051 = {
                        let a3049 = v2;
                        let a3050 = 0i64;
                        compare(a3049, a3050)
                    };
                    if match m3051 {
                        Ordering::Less => {
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
                            let a3052 = v1;
                            let a3053 = 10i64;
                            let a3054 = 0i64;
                            int_rem(a3052, a3053, a3054)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a3055 = v1;
                                let a3056 = 10i64;
                                let a3057 = 0i64;
                                int_quot(a3055, a3056, a3057)?
                            }, f304(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m3058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f833(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt159> {
    let a3062 = f861(v0.clone());
    let a3063 = f861(v1.clone());
    if nat_eq(a3062, a3063) {
        let m3064 = v0;
        match m3064 {
            Adt6::C0 => {
                Ok(Adt159::C1(v2))
            }
            Adt6::C1(v3) => {
                let m3065 = f443(v1);
                match m3065 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v4) => {
                        let a3066 = v3;
                        let a3067 = v4;
                        if equal(a3066, a3067) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m3068 = f695(v1);
                match m3068 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v6) => {
                        if f862(f689(v5, v6)?) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m3069 = f141(v1);
                match m3069 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v8) => {
                        let a3070 = v7;
                        let a3071 = v8;
                        if equal(a3070, a3071) {
                            Ok(Adt159::C1(v2))
                        } else {
                            Ok(Adt159::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m3072 = f223(v1);
                match m3072 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v10) => {
                        Ok(f863(f864(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m3073 = f219(v1);
                match m3073 {
                    None => {
                        Ok(Adt159::C0)
                    }
                    Some(v12) => {
                        let a3075 = f700(v11.clone(), 0u64)?;
                        let a3076 = f700(match v12.clone() {
                            Adt5::C0(h3074) => {
                                h3074
                            }
                        }, 0u64)?;
                        if nat_eq(a3075, a3076) {
                            Ok(f863(f865(v11, match v12 {
                                Adt5::C0(h3077) => {
                                    h3077
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

pub fn f834(v0: u64) -> bool {
    let a3078 = v0;
    let a3079 = 9u64;
    if nat_eq(a3078, a3079) {
        true
    } else {
        let a3080 = v0;
        let a3081 = 10u64;
        if nat_eq(a3080, a3081) {
            true
        } else {
            let a3082 = v0;
            let a3083 = 13u64;
            nat_eq(a3082, a3083)
        }
    }
}

pub fn f835(p3086: List<Adt42>, p3087: List<Adt6>) -> List<Adt6> {
    let m3085 = (p3086, p3087);
    {
        let mut m3085 = m3085;
        loop {
            let step = {
                let (v0, v1) = m3085;
                let m3084 = v0;
                match m3084.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt42>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f839(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3085 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f836(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h3088, _, _, _) => {
            h3088
        }
    })), {
        let a3094 = f623(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h3089, _, _) => {
                h3089
            }
        });
        let a3095 = {
            let a3092 = f866(match v0.clone() {
                Adt2::C0(_, _, h3090, _) => {
                    h3090
                }
            });
            let a3093 = f867(match v0 {
                Adt2::C0(_, _, _, h3091) => {
                    h3091
                }
            });
            append_list(a3092, a3093)
        };
        append_list(a3094, a3095)
    }))
}

pub fn f837(v0: Option<Adt37>) -> List<(Str, Adt6)> {
    let m3096 = v0;
    match m3096 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a3108 = f623(Str::lit("title"), match v1.clone() {
                    Adt37::C0(h3097, _, _, _, _) => {
                        h3097
                    }
                });
                let a3109 = {
                    let a3106 = f303(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt37::C0(_, h3098, _, _, _) => {
                            h3098
                        }
                    });
                    let a3107 = {
                        let a3104 = f303(Str::lit("destructiveHint"), match v1.clone() {
                            Adt37::C0(_, _, h3099, _, _) => {
                                h3099
                            }
                        });
                        let a3105 = {
                            let a3102 = f303(Str::lit("idempotentHint"), match v1.clone() {
                                Adt37::C0(_, _, _, h3100, _) => {
                                    h3100
                                }
                            });
                            let a3103 = f303(Str::lit("openWorldHint"), match v1 {
                                Adt37::C0(_, _, _, _, h3101) => {
                                    h3101
                                }
                            });
                            append_list(a3102, a3103)
                        };
                        append_list(a3104, a3105)
                    };
                    append_list(a3106, a3107)
                };
                append_list(a3108, a3109)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f838(v0: List<Adt58>) -> List<Adt6> {
    f380(f868(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f839(v0: Adt42) -> Str {
    let m3110 = v0;
    match m3110 {
        Adt42::C0 => {
            Str::lit("user")
        }
        Adt42::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f840(v0: Adt66) -> List<(Str, Adt6)> {
    let m3111 = v0;
    match m3111 {
        Adt66::C0(v1) => {
            let a3118 = {
                let m3113 = match v1.clone() {
                    Adt67::C0(h3112, _, _) => {
                        h3112
                    }
                };
                match m3113 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3119 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt67::C0(_, h3114, _) => {
                    h3114
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m3117 = match v1 {
                    Adt67::C0(_, _, h3116) => {
                        h3116
                    }
                };
                match m3117 {
                    Adt5::C0(h3115) => {
                        h3115
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a3118, a3119)
        }
        Adt66::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt69::C0(h3120, _) => {
                    h3120
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt69::C0(_, h3121) => {
                    h3121
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f841(v0: Adt86) -> Option<Adt6> {
    let m3123 = f642(match v0.clone() {
        Adt86::C0(_, h3122, _, _) => {
            h3122
        }
    });
    match m3123 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f796(match v0.clone() {
                Adt86::C0(h3124, _, _, _) => {
                    h3124
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f796(match v0.clone() {
                Adt86::C0(_, _, h3125, _) => {
                    h3125
                }
            })), List::cons((Str::lit("outcome"), f869(match v0 {
                Adt86::C0(_, _, _, h3126) => {
                    h3126
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f842() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f843() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f844(v0: Adt146, v1: Adt33) -> bool {
    let m3127 = v1;
    match m3127 {
        Adt33::C0(_) => {
            f870(v0, Adt146::C0)
        }
        Adt33::C1(_) => {
            f870(v0, Adt146::C1)
        }
        Adt33::C2(_) => {
            f870(v0, Adt146::C2)
        }
        Adt33::C3(_) => {
            f870(v0, Adt146::C2)
        }
        Adt33::C4(_) => {
            f870(v0, Adt146::C3)
        }
        Adt33::C5(_) => {
            f870(v0, Adt146::C4)
        }
        Adt33::C6(_) => {
            f870(v0, Adt146::C5)
        }
        Adt33::C7(_) => {
            f870(v0, Adt146::C6)
        }
        Adt33::C8(_) => {
            f870(v0, Adt146::C7)
        }
        Adt33::C9(_) => {
            f870(v0, Adt146::C8)
        }
        Adt33::C10(_) => {
            f870(v0, Adt146::C9)
        }
        Adt33::C11 => {
            if f870(v0.clone(), Adt146::C10) {
                true
            } else {
                f870(v0, Adt146::C11)
            }
        }
        Adt33::C12(_) => {
            f871(v0)
        }
        Adt33::C13(_) => {
            f872(v0)
        }
        Adt33::C14(v15) => {
            let m3129 = match v15 {
                Adt88::C0(_, h3128, _) => {
                    h3128
                }
            };
            match m3129 {
                Adt89::C0 => {
                    f870(v0, Adt146::C9)
                }
                Adt89::C1 => {
                    if f870(v0.clone(), Adt146::C10) {
                        true
                    } else {
                        f870(v0, Adt146::C11)
                    }
                }
            }
        }
    }
}

pub fn f845(v0: Adt0, v1: Adt15, v2: Adt33) -> Result<Adt134, Adt137> {
    let m3130 = v2.clone();
    match m3130 {
        Adt33::C0(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C1(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C2(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C3(v9) => {
            Ok::<Adt134, Adt137>(f877(v0, f874(v1), v9))
        }
        Adt33::C4(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C5(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C6(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C7(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C8(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C9(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C10(_) => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C11 => {
            Ok::<Adt134, Adt137>(f802(v0, v2))
        }
        Adt33::C12(v3) => {
            f873(v0, f874(v1), v3)
        }
        Adt33::C13(v4) => {
            f875(v0, f874(v1), v4)
        }
        Adt33::C14(v5) => {
            f876(v0, f874(v1), v5)
        }
    }
}

pub fn f846() -> Adt90 {
    Adt90::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f847(v0: u64, v1: u64) -> bool {
    let a3131 = v1;
    let a3132 = v0;
    nat_lt(a3131, a3132)
}

pub fn f848(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt138> {
    let m3133 = f878(v0, v1, List::<(Str, Str)>::nil());
    match m3133 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt138>(f879(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt138>(v3)
        }
    }
}

pub fn f849(p3137: List<Adt119>, v1: Adt5) -> R<Option<Adt117>> {
    let m3136 = p3137;
    {
        let mut m3136 = m3136;
        loop {
            let step = {
                let v0 = m3136;
                let m3134 = v0;
                match m3134.uncons() {
                    None => {
                        Err::<List<Adt119>, R<Option<Adt117>>>(Ok(None::<Adt117>))
                    }
                    Some((v2, v3)) => {
                        let m3135 = f880(v2, v1.clone())?;
                        match m3135 {
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
                Ok(next) => { m3136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f850(v0: Adt5) -> Adt5 {
    let m3138 = f214(Str::lit("arguments"), v0);
    match m3138 {
        Adt107::C0 => {
            f27()
        }
        Adt107::C1(v1) => {
            let m3139 = f219(v1);
            match m3139 {
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

pub fn f851(v0: Option<Str>, v1: Option<Str>) -> Option<Adt117> {
    let m3140 = v0;
    match m3140 {
        None => {
            Some(Adt117::C2)
        }
        Some(v2) => {
            let m3141 = v1;
            match m3141 {
                None => {
                    None::<Adt117>
                }
                Some(v3) => {
                    let a3142 = v2;
                    let a3143 = v3;
                    if equal(a3142, a3143) {
                        None::<Adt117>
                    } else {
                        Some(Adt117::C5)
                    }
                }
            }
        }
    }
}

pub fn f852(v0: Adt151, v1: Adt5) -> Option<Str> {
    let m3144 = v0;
    match m3144 {
        Adt151::C0 => {
            None::<Str>
        }
        Adt151::C1 => {
            let m3145 = f214(Str::lit("name"), v1);
            match m3145 {
                Adt107::C0 => {
                    None::<Str>
                }
                Adt107::C1(v2) => {
                    f141(v2)
                }
            }
        }
        Adt151::C2 => {
            let m3146 = f214(Str::lit("uri"), v1);
            match m3146 {
                Adt107::C0 => {
                    None::<Str>
                }
                Adt107::C1(v3) => {
                    f141(v3)
                }
            }
        }
        Adt151::C3 => {
            let m3147 = f214(Str::lit("taskId"), v1);
            match m3147 {
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

pub fn f853(v0: Adt6) -> Option<Adt18> {
    let m3148 = f219(v0);
    match m3148 {
        None => {
            None::<Adt18>
        }
        Some(v1) => {
            let m3149 = f881(Str::lit("experimental"), v1.clone());
            match m3149 {
                None => {
                    None::<Adt18>
                }
                Some(v2) => {
                    let m3150 = f881(Str::lit("roots"), v1.clone());
                    match m3150 {
                        None => {
                            None::<Adt18>
                        }
                        Some(v3) => {
                            let m3151 = f881(Str::lit("sampling"), v1.clone());
                            match m3151 {
                                None => {
                                    None::<Adt18>
                                }
                                Some(v4) => {
                                    let m3152 = f881(Str::lit("elicitation"), v1.clone());
                                    match m3152 {
                                        None => {
                                            None::<Adt18>
                                        }
                                        Some(v5) => {
                                            let m3153 = f881(Str::lit("extensions"), v1);
                                            match m3153 {
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

pub fn f854() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f855(v0: Adt5) -> Result<Option<Adt19>, Adt138> {
    let m3154 = f806(f882(), v0);
    match m3154 {
        Ok(v2) => {
            let m3155 = v2;
            match m3155 {
                None => {
                    Ok::<Option<Adt19>, Adt138>(None::<Adt19>)
                }
                Some(v3) => {
                    let m3156 = f883(v3);
                    match m3156 {
                        Adt160::C0 => {
                            Err::<Option<Adt19>, Adt138>(Adt138::C8(f882()))
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

pub fn f856(v0: Adt5) -> Result<Option<Adt20>, Adt138> {
    let m3157 = f214(f884(), v0);
    match m3157 {
        Adt107::C0 => {
            Ok::<Option<Adt20>, Adt138>(None::<Adt20>)
        }
        Adt107::C1(v1) => {
            let m3158 = f141(v1.clone());
            match m3158 {
                None => {
                    let m3159 = f215(v1);
                    match m3159 {
                        None => {
                            Err::<Option<Adt20>, Adt138>(Adt138::C8(f884()))
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

pub fn f857(v0: Adt156) -> Adt156 {
    let m3160 = v0;
    match m3160 {
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

pub fn f858(v0: i64, v1: i64) -> R<i64> {
    f304(f221(f860(v0)?, 0i64)?, v1)
}

pub fn f859(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt156> {
    let a3161 = v1;
    let a3162 = v3;
    if nat_lt(a3161, a3162) {
        Ok(f782(f885({
            let a3163 = v3;
            let a3164 = v1;
            nat_sub(a3163, a3164)
        }, v0)?, v2))
    } else {
        let a3165 = v3;
        let a3166 = v1;
        if nat_lt(a3165, a3166) {
            Ok(f782(v0, f885({
                let a3167 = v1;
                let a3168 = v3;
                nat_sub(a3167, a3168)
            }, v2)?))
        } else {
            Ok(f782(v0, v2))
        }
    }
}

pub fn f860(v0: i64) -> R<u64> {
    f886(20u64, v0, 0u64)
}

pub fn f861(v0: Adt6) -> u64 {
    let m3169 = v0;
    match m3169 {
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

pub fn f862(v0: Adt156) -> bool {
    let m3170 = v0;
    match m3170 {
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

pub fn f863(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt159 {
    let m3171 = v0;
    match m3171 {
        None => {
            Adt159::C0
        }
        Some(v2) => {
            Adt159::C1({
                let a3172 = v2;
                let a3173 = v1;
                append_list(a3172, a3173)
            })
        }
    }
}

pub fn f864(p3178: List<Adt6>, p3179: List<Adt6>, p3180: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3177 = (p3178, (p3179, p3180));
    {
        let mut m3177 = m3177;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3177;
                let m3174 = v0;
                match m3174.uncons() {
                    None => {
                        let m3175 = v1;
                        match m3175.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m3176 = v1;
                        match m3176.uncons() {
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
                Ok(next) => { m3177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f865(p3186: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p3187: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3185 = (p3186, p3187);
    {
        let mut m3185 = m3185;
        loop {
            let step = {
                let (v0, v2) = m3185;
                let m3181 = v0;
                match m3181.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m3183 = f298({
                            let (h3182, _) = v3;
                            h3182
                        }, v1.clone());
                        match m3183 {
                            Adt107::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt107::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h3184) = v3;
                                    h3184
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f866(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3188 = v0;
    match m3188 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f576(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f867(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3189 = v0;
    match m3189 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f887(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f868(p3192: List<Adt58>, p3193: List<Adt6>) -> List<Adt6> {
    let m3191 = (p3192, p3193);
    {
        let mut m3191 = m3191;
        loop {
            let step = {
                let (v0, v1) = m3191;
                let m3190 = v0;
                match m3190.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f888(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3191 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f869(v0: Adt87) -> Adt6 {
    let m3194 = v0;
    match m3194 {
        Adt87::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h3195) => {
                    h3195
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt87::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f263(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f870(v0: Adt146, v1: Adt146) -> bool {
    let a3196 = f668(v0);
    let a3197 = f668(v1);
    equal(a3196, a3197)
}

pub fn f871(v0: Adt146) -> bool {
    let m3198 = v0;
    match m3198 {
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

pub fn f873(v0: Adt0, v1: Adt17, v2: Adt82) -> Result<Adt134, Adt137> {
    f889(v0, v2.clone(), f890(match v1 {
        Adt17::C0(_, h3200, _, _, _, _) => {
            h3200
        }
    }, f643(v2)))
}

pub fn f874(v0: Adt15) -> Adt17 {
    let m3201 = v0;
    match m3201 {
        Adt15::C0(v1) => {
            match v1 {
                Adt16::C0(h3202) => {
                    h3202
                }
            }
        }
        Adt15::C1(v2) => {
            match v2 {
                Adt21::C0(h3203, _) => {
                    h3203
                }
            }
        }
        Adt15::C2(v3) => {
            match v3 {
                Adt22::C0(h3204, _, _, _, _) => {
                    h3204
                }
            }
        }
        Adt15::C3(v4) => {
            match v4 {
                Adt21::C0(h3205, _) => {
                    h3205
                }
            }
        }
        Adt15::C4(v5) => {
            match v5 {
                Adt23::C0(h3206, _, _, _) => {
                    h3206
                }
            }
        }
        Adt15::C5(v6) => {
            match v6 {
                Adt21::C0(h3207, _) => {
                    h3207
                }
            }
        }
        Adt15::C6(v7) => {
            match v7 {
                Adt21::C0(h3208, _) => {
                    h3208
                }
            }
        }
        Adt15::C7(v8) => {
            match v8 {
                Adt24::C0(h3209, _, _, _, _) => {
                    h3209
                }
            }
        }
        Adt15::C8(v9) => {
            match v9 {
                Adt25::C0(h3210, _, _, _) => {
                    h3210
                }
            }
        }
        Adt15::C9(v10) => {
            match v10 {
                Adt30::C0(h3211, _) => {
                    h3211
                }
            }
        }
        Adt15::C10(v11) => {
            match v11 {
                Adt31::C0(h3212, _, _) => {
                    h3212
                }
            }
        }
        Adt15::C11(v12) => {
            match v12 {
                Adt32::C0(h3213, _) => {
                    h3213
                }
            }
        }
    }
}

pub fn f875(v0: Adt0, v1: Adt17, v2: Adt83) -> Result<Adt134, Adt137> {
    f891(v0, v2, f153(match v1 {
        Adt17::C0(_, h3214, _, _, _, _) => {
            h3214
        }
    }))
}

pub fn f876(v0: Adt0, v1: Adt17, v2: Adt88) -> Result<Adt134, Adt137> {
    f892(v0, v2, f153(match v1 {
        Adt17::C0(_, h3215, _, _, _, _) => {
            h3215
        }
    }))
}

pub fn f877(v0: Adt0, v1: Adt17, v2: Adt50) -> Adt134 {
    Adt134::C0(Adt33::C2(match v2.clone() {
        Adt50::C0(h3216, _) => {
            h3216
        }
    }), {
        let m3218 = f802(v0, Adt33::C11);
        match m3218 {
            Adt134::C0(_, h3217, _) => {
                h3217
            }
        }
    }, f893(v1, match v2 {
        Adt50::C0(_, h3219) => {
            h3219
        }
    }))
}

pub fn f878(v0: Str, p3225: List<(Str, Adt6)>, p3226: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt138> {
    let m3224 = (p3225, p3226);
    {
        let mut m3224 = m3224;
        loop {
            let step = {
                let (v1, v2) = m3224;
                let m3220 = v1;
                match m3220.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt138>>(Ok::<List<(Str, Str)>, Adt138>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3222 = f141({
                            let (_, h3221) = v3;
                            h3221
                        });
                        match m3222 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt138>>(Err::<List<(Str, Str)>, Adt138>(Adt138::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt138>>((v4, List::cons(({
                                    let (h3223, _) = v3;
                                    h3223
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f879(p3229: List<(Str, Str)>, p3230: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3228 = (p3229, p3230);
    {
        let mut m3228 = m3228;
        loop {
            let step = {
                let (v0, v1) = m3228;
                let m3227 = v0;
                match m3227.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
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

pub fn f880(v0: Adt119, v1: Adt5) -> R<Option<Adt117>> {
    let m3232 = f894(match v0.clone() {
        Adt119::C0(_, h3231, _, _) => {
            h3231
        }
    }, v1);
    match m3232 {
        Adt107::C0 => {
            Ok(f895(v0))
        }
        Adt107::C1(v2) => {
            if f896(v2.clone()) {
                Ok(f895(v0))
            } else {
                f897(v0, v2)
            }
        }
    }
}

pub fn f881(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m3233 = f214(v0, v1);
    match m3233 {
        Adt107::C0 => {
            Some(None::<Adt5>)
        }
        Adt107::C1(v2) => {
            let m3234 = f219(v2);
            match m3234 {
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

pub fn f882() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f883(v0: Str) -> Adt160 {
    let a3235 = v0.clone();
    let a3236 = Str::lit("debug");
    if equal(a3235, a3236) {
        Adt160::C1(Adt19::C0)
    } else {
        let a3237 = v0.clone();
        let a3238 = Str::lit("info");
        if equal(a3237, a3238) {
            Adt160::C1(Adt19::C1)
        } else {
            let a3239 = v0.clone();
            let a3240 = Str::lit("notice");
            if equal(a3239, a3240) {
                Adt160::C1(Adt19::C2)
            } else {
                let a3241 = v0.clone();
                let a3242 = Str::lit("warning");
                if equal(a3241, a3242) {
                    Adt160::C1(Adt19::C3)
                } else {
                    let a3243 = v0.clone();
                    let a3244 = Str::lit("error");
                    if equal(a3243, a3244) {
                        Adt160::C1(Adt19::C4)
                    } else {
                        let a3245 = v0.clone();
                        let a3246 = Str::lit("critical");
                        if equal(a3245, a3246) {
                            Adt160::C1(Adt19::C5)
                        } else {
                            let a3247 = v0.clone();
                            let a3248 = Str::lit("alert");
                            if equal(a3247, a3248) {
                                Adt160::C1(Adt19::C6)
                            } else {
                                let a3249 = v0;
                                let a3250 = Str::lit("emergency");
                                if equal(a3249, a3250) {
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

pub fn f884() -> Str {
    Str::lit("progressToken")
}

pub fn f885(p3255: u64, p3256: i64) -> R<i64> {
    let m3254 = (p3255, p3256);
    {
        let mut m3254 = m3254;
        loop {
            let step = {
                let (v0, v1) = m3254;
                let m3251 = v0;
                if m3251 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3251 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3252 = v1;
                        let a3253 = 10i64;
                        int_mul(a3252, a3253)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f886(p3267: u64, p3268: i64, p3269: u64) -> R<u64> {
    let m3266 = (p3267, (p3268, p3269));
    {
        let mut m3266 = m3266;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3266;
                let m3257 = v0;
                if m3257 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3257 - 1;
                    let v4: u64 = {
                        let a3258 = v2;
                        let a3259 = 1u64;
                        nat_add(a3258, a3259)?
                    };
                    let m3262 = {
                        let a3260 = v1;
                        let a3261 = 10i64;
                        compare(a3260, a3261)
                    };
                    if match m3262 {
                        Ordering::Less => {
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
                            let a3263 = v1;
                            let a3264 = 10i64;
                            let a3265 = 0i64;
                            int_quot(a3263, a3264, a3265)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f887(v0: Adt3) -> Str {
    let m3270 = v0;
    match m3270 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f888(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a3279 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h3271, _, _, _) => {
                h3271
            }
        })), List::<(Str, Adt6)>::nil());
        let a3280 = {
            let a3277 = f623(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h3272, _, _) => {
                    h3272
                }
            });
            let a3278 = {
                let a3275 = f623(Str::lit("description"), match v0.clone() {
                    Adt58::C0(_, _, h3273, _) => {
                        h3273
                    }
                });
                let a3276 = f303(Str::lit("required"), match v0 {
                    Adt58::C0(_, _, _, h3274) => {
                        h3274
                    }
                });
                append_list(a3275, a3276)
            };
            append_list(a3277, a3278)
        };
        append_list(a3279, a3280)
    })
}

pub fn f889(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt134, Adt137> {
    if v2 {
        Ok::<Adt134, Adt137>(f802(v0, Adt33::C12(v1)))
    } else {
        Err::<Adt134, Adt137>(Adt137::C4(Adt90::C8(f898(f643(v1)))))
    }
}

pub fn f890(v0: Adt18, v1: List<(Str, Adt65)>) -> bool {
    let a3281 = f899(v1.clone());
    if if if bool_not(a3281) {
        true
    } else {
        f671(match v0.clone() {
            Adt18::C0(_, _, _, h3282, _) => {
                h3282
            }
        })
    } {
        let a3283 = f900(v1.clone());
        if bool_not(a3283) {
            true
        } else {
            f671(match v0.clone() {
                Adt18::C0(_, _, h3284, _, _) => {
                    h3284
                }
            })
        }
    } else {
        false
    } {
        let a3285 = f901(v1);
        if bool_not(a3285) {
            true
        } else {
            f671(match v0 {
                Adt18::C0(_, h3286, _, _, _) => {
                    h3286
                }
            })
        }
    } else {
        false
    }
}

pub fn f891(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt134, Adt137> {
    if v2 {
        Ok::<Adt134, Adt137>(f802(v0, Adt33::C13(v1)))
    } else {
        Err::<Adt134, Adt137>(Adt137::C4(Adt90::C8(f227())))
    }
}

pub fn f892(v0: Adt0, v1: Adt88, v2: bool) -> Result<Adt134, Adt137> {
    if v2 {
        Ok::<Adt134, Adt137>(f802(v0, Adt33::C14(v1)))
    } else {
        Err::<Adt134, Adt137>(Adt137::C4(Adt90::C8(f227())))
    }
}

pub fn f893(v0: Adt17, v1: List<Adt51>) -> List<Adt136> {
    let m3288 = match v0 {
        Adt17::C0(_, _, _, _, h3287, _) => {
            h3287
        }
    };
    match m3288 {
        None => {
            List::<Adt136>::nil()
        }
        Some(v2) => {
            f902(f903(v2, v1, List::<Adt136>::nil()), List::<Adt136>::nil())
        }
    }
}

pub fn f894(p3294: List<Str>, p3295: Adt5) -> Adt107 {
    let m3293 = (p3294, p3295);
    {
        let mut m3293 = m3293;
        loop {
            let step = {
                let (v0, v1) = m3293;
                let m3289 = v0;
                match m3289.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt107>(Adt107::C0)
                    }
                    Some((v2, v3)) => {
                        let m3290 = f214(v2, v1);
                        match m3290 {
                            Adt107::C0 => {
                                Err::<(List<Str>, Adt5), Adt107>(Adt107::C0)
                            }
                            Adt107::C1(v4) => {
                                let m3291 = v3.clone();
                                match m3291.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt107>(Adt107::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3292 = f219(v4);
                                        match m3292 {
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
                Ok(next) => { m3293 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f895(v0: Adt119) -> Option<Adt117> {
    let m3297 = match v0.clone() {
        Adt119::C0(_, _, _, h3296) => {
            h3296
        }
    };
    match m3297 {
        Adt120::C0 => {
            None::<Adt117>
        }
        Adt120::C1 => {
            Some(Adt117::C10(match v0 {
                Adt119::C0(h3298, _, _, _) => {
                    h3298
                }
            }))
        }
        Adt120::C2(_) => {
            None::<Adt117>
        }
    }
}

pub fn f896(v0: Adt6) -> bool {
    let m3299 = v0;
    match m3299 {
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

pub fn f897(v0: Adt119, v1: Adt6) -> R<Option<Adt117>> {
    let m3301 = match v0.clone() {
        Adt119::C0(_, _, _, h3300) => {
            h3300
        }
    };
    match m3301 {
        Adt120::C0 => {
            Ok(Some(Adt117::C8(match v0 {
                Adt119::C0(h3302, _, _, _) => {
                    h3302
                }
            })))
        }
        Adt120::C1 => {
            Ok(Some(Adt117::C10(match v0 {
                Adt119::C0(h3303, _, _, _) => {
                    h3303
                }
            })))
        }
        Adt120::C2(v2) => {
            if f904(match v0.clone() {
                Adt119::C0(_, _, h3304, _) => {
                    h3304
                }
            }, v2, v1)? {
                Ok(None::<Adt117>)
            } else {
                Ok(Some(Adt117::C9(match v0 {
                    Adt119::C0(h3305, _, _, _) => {
                        h3305
                    }
                })))
            }
        }
    }
}

pub fn f898(v0: List<(Str, Adt65)>) -> Adt18 {
    Adt18::C0(None::<Adt5>, if f901(v0.clone()) {
        Some(f27())
    } else {
        None::<Adt5>
    }, if f900(v0.clone()) {
        Some(f27())
    } else {
        None::<Adt5>
    }, if f899(v0) {
        Some(f27())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f899(p3310: List<(Str, Adt65)>) -> bool {
    let m3309 = p3310;
    {
        let mut m3309 = m3309;
        loop {
            let step = {
                let v0 = m3309;
                let m3306 = v0;
                match m3306.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3308 = {
                            let (_, h3307) = v1;
                            h3307
                        };
                        match m3308 {
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
                Ok(next) => { m3309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f900(p3315: List<(Str, Adt65)>) -> bool {
    let m3314 = p3315;
    {
        let mut m3314 = m3314;
        loop {
            let step = {
                let v0 = m3314;
                let m3311 = v0;
                match m3311.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3313 = {
                            let (_, h3312) = v1;
                            h3312
                        };
                        match m3313 {
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
                Ok(next) => { m3314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f901(p3320: List<(Str, Adt65)>) -> bool {
    let m3319 = p3320;
    {
        let mut m3319 = m3319;
        loop {
            let step = {
                let v0 = m3319;
                let m3316 = v0;
                match m3316.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3318 = {
                            let (_, h3317) = v1;
                            h3317
                        };
                        match m3318 {
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
                Ok(next) => { m3319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f902(p3323: List<Adt136>, p3324: List<Adt136>) -> List<Adt136> {
    let m3322 = (p3323, p3324);
    {
        let mut m3322 = m3322;
        loop {
            let step = {
                let (v0, v1) = m3322;
                let m3321 = v0;
                match m3321.uncons() {
                    None => {
                        Err::<(List<Adt136>, List<Adt136>), List<Adt136>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt136>, List<Adt136>), List<Adt136>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f903(v0: Adt20, p3327: List<Adt51>, p3328: List<Adt136>) -> List<Adt136> {
    let m3326 = (p3327, p3328);
    {
        let mut m3326 = m3326;
        loop {
            let step = {
                let (v1, v2) = m3326;
                let m3325 = v1;
                match m3325.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt136>), List<Adt136>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt51>, List<Adt136>), List<Adt136>>((v4, List::cons(Adt136::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3326 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f904(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3329 = v0;
    match m3329 {
        Adt13::C0 => {
            let m3330 = f141(v2);
            match m3330 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3331 = v1;
                    let a3332 = v3;
                    Ok(equal(a3331, a3332))
                }
            }
        }
        Adt13::C1 => {
            f905(v1, v2)
        }
        Adt13::C2 => {
            let m3333 = f443(v2);
            match m3333 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3334 = v1;
                        let a3335 = Str::lit("true");
                        Ok(equal(a3334, a3335))
                    } else {
                        let a3336 = v1;
                        let a3337 = Str::lit("false");
                        Ok(equal(a3336, a3337))
                    }
                }
            }
        }
    }
}

pub fn f905(v0: Str, v1: Adt6) -> R<bool> {
    let m3338 = f695(v1);
    match m3338 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3339 = f422(v2)?;
            match m3339 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3341 = f906(f177({
                        let a3340 = v0;
                        utf8_encode(a3340)
                    })?)?;
                    match m3341 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f299({
                                let a3342 = v3;
                                let a3343 = v4;
                                int_sub(a3342, a3343)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f906(v0: List<u64>) -> R<Option<i64>> {
    let m3344 = v0.clone();
    match m3344.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3345 = v1;
            let a3346 = 45u64;
            if nat_eq(a3345, a3346) {
                let m3347 = v2.clone();
                match m3347.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3348 = f907(v2, 0i64)?;
                        match m3348 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3349 = v5;
                                    int_neg(a3349)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f907(v0, 0i64)
            }
        }
    }
}

pub fn f907(p3355: List<u64>, p3356: i64) -> R<Option<i64>> {
    let m3354 = (p3355, p3356);
    {
        let mut m3354 = m3354;
        loop {
            let step = {
                let (v0, v1) = m3354;
                let m3350 = v0;
                match m3350.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3351 = f908(v2);
                        match m3351 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f304({
                                    let a3352 = v1;
                                    let a3353 = 10i64;
                                    int_mul(a3352, a3353)?
                                }, f508(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f908(v0: u64) -> Option<u64> {
    let a3357 = v0;
    let a3358 = 48u64;
    if if nat_lt(a3357, a3358) {
        true
    } else {
        let a3359 = 57u64;
        let a3360 = v0;
        nat_lt(a3359, a3360)
    } {
        None::<u64>
    } else {
        Some({
            let a3361 = v0;
            let a3362 = 48u64;
            nat_sub(a3361, a3362)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
