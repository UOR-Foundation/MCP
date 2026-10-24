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
    F40,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt49, Adt84>> {
        match self {
            Fn1::F40 => f40(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F89,
}

impl Fn2 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt46>>> {
        match self {
            Fn2::F89 => f89(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F95,
}

impl Fn3 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt58 {
        match self {
            Fn3::F95 => f95(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F97,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt24) -> R<Result<List<Str>, Adt84>> {
        match self {
            Fn4::F97 => f97(p0),
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
    Ok(List::cons(f28(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f29()), List::cons(f28(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f30()), List::cons(f28(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f29()), List::cons(f28(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f31()), List::cons(f28(Str::lit("test_image_content"), Str::lit("Tests image content response"), f31()), List::cons(f28(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f31()), List::cons(f28(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f31()), List::cons(f28(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f31()), List::cons(f28(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f31()), List::cons(f28(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f31()), List::cons(f28(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f32()), List::cons(f28(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f33()?), List::cons(f28(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f34()), List::cons(f35(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f31(), f36()), List::cons(f35(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f31(), f36()), List::cons(f28(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f31()), List::cons(f28(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f31()), List::cons(f28(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f31()), List::cons(f28(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f31()), List::cons(f28(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f31()), List::<Adt35>::nil())))))))))))))))))))))
}

pub fn f13() -> u64 {
    100u64
}

pub fn f14(v0: List<Adt35>) -> R<Option<Adt84>> {
    let m30 = f27(v0)?;
    match m30 {
        Ok(_) => {
            Ok(None::<Adt84>)
        }
        Err(v1) => {
            Ok(Some(Adt84::C9(f37(v1))))
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
            Ok(f38(f10(), f12()?, v1))
        }
        Adt14::C2(v2) => {
            f39(f12()?, Fn1::F40, v2)
        }
        Adt14::C3(v4) => {
            Ok(f41(v4))
        }
        Adt14::C4(v5) => {
            f42(v5)
        }
        Adt14::C5(v6) => {
            Ok(f43(v6))
        }
        Adt14::C6(v7) => {
            Ok(f44(v7))
        }
        Adt14::C7(v8) => {
            Ok(f45(v8))
        }
        Adt14::C8(v9) => {
            f46(v9)
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
    let m32 = f47(v0.clone(), v1)?;
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
            f48(v0.clone(), v1, v3.clone(), f47(v0, match v3 {
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
    f49(f50(v0, List::<Adt86>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f51(f52(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt86> {
    List::cons(Adt86::C0(f53(), f54()), List::<Adt86>::nil())
}

pub fn f23() -> List<u64> {
    let a69 = f55();
    let a70 = {
        let a67 = f56();
        let a68 = f57();
        append_list(a67, a68)
    };
    append_list(a69, a70)
}

pub fn f24(v0: Adt0, v1: Fn0, v2: Adt85, v3: Option<Adt90>) -> R<Adt88> {
    let m71 = v3;
    match m71 {
        None => {
            f59(v0, v1, v2)
        }
        Some(v4) => {
            f58(v4)
        }
    }
}

pub fn f25(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m72 = f60(v0.clone(), v1.clone())?;
    match m72 {
        None => {
            f61(v0, v1)
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
    let m73 = f62(v0.clone());
    match m73 {
        None => {
            f63(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f28(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f29() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f30() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f64(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f65()), List::cons((Str::lit("limit"), f64(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f64(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f31() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f32() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f33() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f65()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f66(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f66(0u64)?), List::cons((Str::lit("maximum"), f66(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f66(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
}

pub fn f34() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("target"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("zone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt6::C3(Str::lit("Zone"))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f35(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt36>, None::<Adt5>)
}

pub fn f36() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("sum"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("sum")), List::cons(Adt6::C3(Str::lit("label")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f37(v0: Adt91) -> Str {
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

pub fn f38(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt84> {
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

pub fn f39(v0: List<Adt35>, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m104 = f67(match v2.clone() {
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
            f68(v3, v1, v2)
        }
    }
}

pub fn f40(v0: Adt21) -> R<Result<Adt49, Adt84>> {
    if f69(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt49, Adt84>(f70(List::cons(f71(Str::lit("This is a simple text response for testing.")), List::<Adt38>::nil()))))
    } else {
        if f69(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt49, Adt84>(f70(List::cons(f72(), List::<Adt38>::nil()))))
        } else {
            if f69(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt49, Adt84>(f70(List::cons(f73(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt38>::nil()))))
            } else {
                if f69(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt49, Adt84>(f70(List::cons(f74(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt38>::nil()))))
                } else {
                    if f69(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt49, Adt84>(f70(List::cons(f71(Str::lit("Multiple content types test:")), List::cons(f72(), List::cons(f74(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt38>::nil()))))))
                    } else {
                        if f69(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt49, Adt84>(f75(List::cons(f71(Str::lit("This tool intentionally returns an error for testing")), List::<Adt38>::nil()), None::<Adt6>, true, List::<Adt50>::nil())))
                        } else {
                            if f69(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt49, Adt84>(f76(v0, f77()?)))
                            } else {
                                if f69(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt49, Adt84>(f76(v0, f78()?)))
                                } else {
                                    if f69(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt49, Adt84>(f75(List::cons(f71(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt38>::nil()), Some(f79()?), false, List::<Adt50>::nil())))
                                    } else {
                                        if f69(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt49, Adt84>(f75(List::cons(f71(Str::lit("{\"sum\":\"three\"}")), List::<Adt38>::nil()), Some(f80()), false, List::<Adt50>::nil())))
                                        } else {
                                            if f69(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt49, Adt84>(f70(List::cons(f81(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt38>::nil()))))
                                            } else {
                                                if f69(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt49, Adt84>(f70(List::cons(f71(f82()), List::<Adt38>::nil()))))
                                                } else {
                                                    if f69(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt49, Adt84>(f70(List::cons(f71(Str::lit("Streaming complete")), List::<Adt38>::nil()))))
                                                    } else {
                                                        if f69(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt49, Adt84>(f70(List::cons(f71(Str::lit("Logging evaluated")), List::<Adt38>::nil()))))
                                                        } else {
                                                            Ok(f83(v0))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f41(v0: Adt20) -> Result<Adt32, Adt84> {
    f84(f85(), f86(), v0)
}

pub fn f42(v0: Adt22) -> R<Result<Adt32, Adt84>> {
    f87(f88(), Fn2::F89, v0)
}

pub fn f43(v0: Adt20) -> Result<Adt32, Adt84> {
    f90(f85(), f91(), v0)
}

pub fn f44(v0: Adt20) -> Result<Adt32, Adt84> {
    f92(f85(), f93(), v0)
}

pub fn f45(v0: Adt23) -> Result<Adt32, Adt84> {
    f94(f93(), Fn3::F95, v0)
}

pub fn f46(v0: Adt24) -> R<Result<Adt32, Adt84>> {
    f96(Fn4::F97, v0)
}

pub fn f47(v0: Bytes, v1: u64) -> R<Option<u64>> {
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
                let a111 = f98(v0.clone(), v1)?;
                let a112 = 16777216u64;
                nat_mul(a111, a112)?
            };
            let a128 = {
                let a125 = {
                    let a115 = f98(v0.clone(), {
                        let a113 = v1;
                        let a114 = 1u64;
                        nat_add(a113, a114)?
                    })?;
                    let a116 = 65536u64;
                    nat_mul(a115, a116)?
                };
                let a126 = {
                    let a123 = {
                        let a119 = f98(v0.clone(), {
                            let a117 = v1;
                            let a118 = 2u64;
                            nat_add(a117, a118)?
                        })?;
                        let a120 = 256u64;
                        nat_mul(a119, a120)?
                    };
                    let a124 = f98(v0, {
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

pub fn f48(v0: Bytes, v1: Adt89, v2: Adt89, v3: Option<u64>) -> R<Result<Adt85, Adt87>> {
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
            let m134 = f99(v0.clone(), {
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
                    f100(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt85, Adt87>(v5))
                }
            }
        }
    }
}

pub fn f49(p139: List<Adt86>, p140: List<u64>) -> List<u64> {
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
                            let a136 = f101(v2);
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

pub fn f50(p143: List<Adt86>, p144: List<Adt86>) -> List<Adt86> {
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

pub fn f51(p149: List<List<u64>>, p150: List<u64>) -> List<u64> {
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
                            let a146 = f102(v2);
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

pub fn f52(p153: List<List<u64>>, p154: List<List<u64>>) -> List<List<u64>> {
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

pub fn f53() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f54() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f55() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f56() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f57() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f58(v0: Adt90) -> R<Adt88> {
    f103(f104(v0.clone()), f105(None::<Adt92>, f106(v0)))
}

pub fn f59(v0: Adt0, v1: Fn0, v2: Adt85) -> R<Adt88> {
    if f107(match v2.clone() {
        Adt85::C0(h155, _, _, _, _, _) => {
            h155
        }
    }) {
        f108(v0, v1, match v2.clone() {
            Adt85::C0(_, _, _, _, h156, _) => {
                h156
            }
        }, match v2.clone() {
            Adt85::C0(_, _, h157, _, _, _) => {
                h157
            }
        }, f109(match v2 {
            Adt85::C0(_, _, _, h158, _, _) => {
                h158
            }
        })?)
    } else {
        Ok(f110())
    }
}

pub fn f60(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m159 = f111(v0, f112())?;
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
            f113(f114(v2), v1, Adt90::C0)
        }
    }
}

pub fn f61(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m160 = f111(v0, f115())?;
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
            f113(f116(v2), v1, Adt90::C1)
        }
    }
}

pub fn f62(v0: List<Adt35>) -> Option<Adt91> {
    let m161 = f117(v0, List::<Str>::nil());
    match m161 {
        None => {
            None::<Adt91>
        }
        Some(v1) => {
            Some(Adt91::C1(v1))
        }
    }
}

pub fn f63(p165: List<Adt35>, p166: List<Adt12>) -> R<Result<List<Adt12>, Adt91>> {
    let m164 = (p165, p166);
    {
        let mut m164 = m164;
        loop {
            let step = {
                let (v0, v1) = m164;
                let m162 = v0;
                match m162.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Ok::<List<Adt12>, Adt91>(f118(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m163 = f119(v2)?;
                        match m163 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>((v3, f118(v5, v1)))
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

pub fn f64(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f65() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f66(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f120(v0, 0i64)?, 0i64)))
}

pub fn f67(v0: Str, p172: List<Adt35>) -> Option<Adt35> {
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

pub fn f68(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m179 = f121(Adt6::C5({
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
    }), f122(v2.clone()))?;
    match m179 {
        Adt98::C0 => {
            f123(v0, v1, v2)
        }
        Adt98::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f124(f125(v3, v4)?, true))))
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

pub fn f69(v0: Adt21, v1: Str) -> bool {
    let a181 = match v0 {
        Adt21::C0(_, h180, _, _, _) => {
            h180
        }
    };
    let a182 = v1;
    equal(a181, a182)
}

pub fn f70(v0: List<Adt38>) -> Adt49 {
    f75(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f71(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f72() -> Adt38 {
    f81(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f73(v0: Str, v1: Str) -> Adt38 {
    Adt38::C2(Adt43::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f74(v0: Str, v1: Str, v2: Str) -> Adt38 {
    Adt38::C4(Adt45::C0(Adt46::C0(Adt47::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt40>, None::<Adt5>))
}

pub fn f75(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f76(v0: Adt21, v1: List<Adt50>) -> Adt49 {
    f75(List::cons(f71(f126({
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

pub fn f77() -> R<List<Adt50>> {
    Ok(List::cons(f127(0u64, 100u64)?, List::cons(f127(50u64, 100u64)?, List::cons(f127(100u64, 100u64)?, List::<Adt50>::nil()))))
}

pub fn f78() -> R<List<Adt50>> {
    Ok(List::cons(f127(50u64, 100u64)?, List::cons(f127(10u64, 100u64)?, List::<Adt50>::nil())))
}

pub fn f79() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f66(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f80() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f81(v0: Str, v1: Str) -> Adt38 {
    Adt38::C1(Adt42::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f82() -> Str {
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

pub fn f83(v0: Adt21) -> Result<Adt49, Adt84> {
    if f69(v0.clone(), Str::lit("test_missing_capability")) {
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
                Err::<Adt49, Adt84>(Adt84::C8(f128()))
            }
            Some(_) => {
                Ok::<Adt49, Adt84>(f129())
            }
        }
    } else {
        if f69(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt49, Adt84>(f70(List::cons(f71(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt38>::nil())))
        } else {
            if f69(v0, Str::lit("test_arguments")) {
                Ok::<Adt49, Adt84>(f70(List::cons(f71(Str::lit("arguments accepted")), List::<Adt38>::nil())))
            } else {
                Ok::<Adt49, Adt84>(f129())
            }
        }
    }
}

pub fn f84(v0: Adt10, v1: List<Adt44>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m195 = match v2 {
        Adt20::C0(_, h194) => {
            h194
        }
    };
    match m195 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C4(Adt51::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f85() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C0)
}

pub fn f86() -> List<Adt44> {
    f130(f131())
}

pub fn f87(v0: Adt10, v1: Fn2, v2: Adt22) -> R<Result<Adt32, Adt84>> {
    Ok(f132(v0, match v2.clone() {
        Adt22::C0(_, h196, _, _) => {
            h196
        }
    }, {
        let c197: Fn2 = v1;
        c197.apply(match v2 {
            Adt22::C0(_, h198, _, _) => {
                h198
            }
        })?
    }))
}

pub fn f88() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C1)
}

pub fn f89(v0: Str) -> R<Option<List<Adt46>>> {
    let m199 = f133(v0.clone(), f131());
    match m199 {
        None => {
            f134(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f90(v0: Adt10, v1: List<Adt54>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m201 = match v2 {
        Adt20::C0(_, h200) => {
            h200
        }
    };
    match m201 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C6(Adt53::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f91() -> List<Adt54> {
    List::cons(Adt54::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt2>>, f135(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt40>, None::<Adt5>), List::<Adt54>::nil())
}

pub fn f92(v0: Adt10, v1: List<Adt56>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m203 = match v2 {
        Adt20::C0(_, h202) => {
            h202
        }
    };
    match m203 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C7(Adt55::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f93() -> List<Adt56> {
    List::cons(f136(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt57>>), List::cons(f136(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f137(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f137(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt57>::nil())))), List::cons(f136(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f137(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt57>::nil()))), List::cons(f136(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt57>>), List::<Adt56>::nil()))))
}

pub fn f94(v0: List<Adt56>, v1: Fn3, v2: Adt23) -> Result<Adt32, Adt84> {
    let m205 = f138(match v2.clone() {
        Adt23::C0(_, h204, _, _, _) => {
            h204
        }
    }, v0);
    match m205 {
        None => {
            Err::<Adt32, Adt84>(Adt84::C1(match v2 {
                Adt23::C0(_, h206, _, _, _) => {
                    h206
                }
            }))
        }
        Some(v3) => {
            let m208 = f139(f140(v3), f141(match v2.clone() {
                Adt23::C0(_, _, h207, _, _) => {
                    h207
                }
            }));
            match m208 {
                None => {
                    Ok::<Adt32, Adt84>(Adt32::C8({
                        let c211: Fn3 = v1;
                        c211.apply(match v2.clone() {
                            Adt23::C0(_, h212, _, _, _) => {
                                h212
                            }
                        }, f141(match v2 {
                            Adt23::C0(_, _, h213, _, _) => {
                                h213
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt32, Adt84>(Adt84::C4({
                        let a209 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a210 = Str::lit(" ");
                        join(a209, a210)
                    }))
                }
            }
        }
    }
}

pub fn f95(v0: Str, v1: List<(Str, Str)>) -> Adt58 {
    let a214 = v0.clone();
    let a215 = Str::lit("test_simple_prompt");
    if equal(a214, a215) {
        f142(List::cons(f143(Str::lit("This is a simple prompt for testing.")), List::<Adt59>::nil()))
    } else {
        let a216 = v0.clone();
        let a217 = Str::lit("test_prompt_with_arguments");
        if equal(a216, a217) {
            f142(List::cons(f143({
                let a218 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f144(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f144(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a219 = Str::lit("");
                join(a218, a219)
            }), List::<Adt59>::nil()))
        } else {
            let a220 = v0.clone();
            let a221 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a220, a221) {
                f142(List::cons(Adt59::C0(Adt41::C0, Adt38::C4(Adt45::C0(f145(f144(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt40>, None::<Adt5>))), List::cons(f143(Str::lit("Please process the embedded resource above.")), List::<Adt59>::nil())))
            } else {
                let a222 = v0;
                let a223 = Str::lit("test_prompt_with_image");
                if equal(a222, a223) {
                    f142(List::cons(Adt59::C0(Adt41::C0, Adt38::C1(Adt42::C0(f146(), Str::lit("image/png"), None::<Adt40>, None::<Adt5>))), List::cons(f143(Str::lit("Please analyze the image above.")), List::<Adt59>::nil())))
                } else {
                    f142(List::<Adt59>::nil())
                }
            }
        }
    }
}

pub fn f96(v0: Fn4, v1: Adt24) -> R<Result<Adt32, Adt84>> {
    let m225 = {
        let c224: Fn4 = v0;
        c224.apply(v1)?
    };
    match m225 {
        Ok(v3) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C9(f147(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt32, Adt84>(v2))
        }
    }
}

pub fn f97(v0: Adt24) -> R<Result<List<Str>, Adt84>> {
    let m227 = match v0.clone() {
        Adt24::C0(_, h226, _, _) => {
            h226
        }
    };
    match m227 {
        Adt25::C0(v1) => {
            let m229 = f138(match v1.clone() {
                Adt26::C0(h228, _) => {
                    h228
                }
            }, f93());
            match m229 {
                None => {
                    Ok(Err::<List<Str>, Adt84>(Adt84::C1(match v1 {
                        Adt26::C0(h230, _) => {
                            h230
                        }
                    })))
                }
                Some(v2) => {
                    if f148(f140(v2), {
                        let m233 = match v0.clone() {
                            Adt24::C0(_, _, h232, _) => {
                                h232
                            }
                        };
                        match m233 {
                            Adt27::C0(h231, _) => {
                                h231
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt84>(f149({
                            let m236 = match v0.clone() {
                                Adt24::C0(_, _, h235, _) => {
                                    h235
                                }
                            };
                            match m236 {
                                Adt27::C0(_, h234) => {
                                    h234
                                }
                            }
                        }, f150(match v1 {
                            Adt26::C0(h237, _) => {
                                h237
                            }
                        }, {
                            let m240 = match v0 {
                                Adt24::C0(_, _, h239, _) => {
                                    h239
                                }
                            };
                            match m240 {
                                Adt27::C0(h238, _) => {
                                    h238
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
            let a241 = v3.clone();
            let a242 = f135();
            if equal(a241, a242) {
                let a246 = {
                    let m245 = match v0.clone() {
                        Adt24::C0(_, _, h244, _) => {
                            h244
                        }
                    };
                    match m245 {
                        Adt27::C0(h243, _) => {
                            h243
                        }
                    }
                };
                let a247 = Str::lit("id");
                if equal(a246, a247) {
                    Ok(Ok::<List<Str>, Adt84>(f149({
                        let m250 = match v0 {
                            Adt24::C0(_, _, h249, _) => {
                                h249
                            }
                        };
                        match m250 {
                            Adt27::C0(_, h248) => {
                                h248
                            }
                        }
                    }, f151())?))
                } else {
                    Ok(Err::<List<Str>, Adt84>(Adt84::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt84>(Adt84::C2(v3)))
            }
        }
    }
}

pub fn f98(v0: Bytes, v1: u64) -> R<u64> {
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

pub fn f99(v0: Bytes, p309: u64, p310: u64, p311: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m308 = (p309, (p310, p311));
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v1, (v2, v3)) = m308;
                let m300 = v2;
                if m300 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f152(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m300 - 1;
                    let m301 = f16(v0.clone(), v1)?;
                    match m301 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m303 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h302) => {
                                    h302
                                }
                            })?;
                            match m303 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h304) => {
                                            h304
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h305) => {
                                            h305
                                        }
                                    }, (v4, List::cons(Adt86::C0(f153(match v5 {
                                        Adt89::C0(h306, _) => {
                                            h306
                                        }
                                    })?, f153(match v6 {
                                        Adt89::C0(h307, _) => {
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

pub fn f100(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m313 = f16(v0.clone(), {
        let (_, h312) = v3.clone();
        h312
    })?;
    match m313 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h314) = v3;
                h314
            })))
        }
        Some(v4) => {
            let m316 = f47(v0.clone(), match v4.clone() {
                Adt89::C0(_, h315) => {
                    h315
                }
            })?;
            match m316 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h317) => {
                            h317
                        }
                    })))
                }
                Some(v5) => {
                    f154(v0, Adt85::C0(f153(match v1 {
                        Adt89::C0(h318, _) => {
                            h318
                        }
                    })?, f153(match v2 {
                        Adt89::C0(h319, _) => {
                            h319
                        }
                    })?, {
                        let (h320, _) = v3;
                        h320
                    }, match v4.clone() {
                        Adt89::C0(h321, _) => {
                            h321
                        }
                    }, v5, List::<List<u64>>::nil()), {
                        let a323 = match v4 {
                            Adt89::C0(_, h322) => {
                                h322
                            }
                        };
                        let a324 = 4u64;
                        nat_add(a323, a324)?
                    })
                }
            }
        }
    }
}

pub fn f101(v0: Adt86) -> List<u64> {
    let a335 = f18({
        let a326 = match v0.clone() {
            Adt86::C0(h325, _) => {
                h325
            }
        };
        length_list(a326)
    });
    let a336 = {
        let a333 = match v0.clone() {
            Adt86::C0(h327, _) => {
                h327
            }
        };
        let a334 = {
            let a331 = f18({
                let a329 = match v0.clone() {
                    Adt86::C0(_, h328) => {
                        h328
                    }
                };
                length_list(a329)
            });
            let a332 = match v0 {
                Adt86::C0(_, h330) => {
                    h330
                }
            };
            append_list(a331, a332)
        };
        append_list(a333, a334)
    };
    append_list(a335, a336)
}

pub fn f102(v0: List<u64>) -> List<u64> {
    let a338 = f18({
        let a337 = v0.clone();
        length_list(a337)
    });
    let a339 = v0;
    append_list(a338, a339)
}

pub fn f103(v0: u64, v1: Adt6) -> R<Adt88> {
    let m340 = f155(v1)?;
    match m340 {
        Ok(v3) => {
            Ok(Adt88::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f104(v0: Adt90) -> u64 {
    let m341 = v0;
    match m341 {
        Adt90::C0 => {
            f156()
        }
        Adt90::C1 => {
            f156()
        }
    }
}

pub fn f105(v0: Option<Adt92>, v1: Adt80) -> Adt6 {
    Adt6::C5(f157(v0, v1))
}

pub fn f106(v0: Adt90) -> Adt80 {
    Adt80::C0(f158(), f159(v0), None::<Adt6>)
}

pub fn f107(v0: List<u64>) -> bool {
    f160(v0, f161())
}

pub fn f108(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt93>) -> R<Adt88> {
    let m342 = v4;
    match m342 {
        Ok(v6) => {
            f164(v0, v1, v2, v3, f165(v6)?)
        }
        Err(_) => {
            f103(f162(), f105(None::<Adt92>, f163(Adt94::C0)))
        }
    }
}

pub fn f109(v0: Bytes) -> R<Result<Adt6, Adt93>> {
    f166(f153(v0)?)
}

pub fn f110() -> Adt88 {
    Adt88::C0(f167(), List::cons(Adt86::C0(f168(), f169()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f111(v0: List<Adt86>, v1: List<u64>) -> R<Adt96> {
    let m343 = f170(v0, v1, Adt97::C0)?;
    match m343 {
        Adt97::C0 => {
            Ok(Adt96::C0)
        }
        Adt97::C1(v2) => {
            let m344 = f171(v2);
            match m344 {
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

pub fn f112() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f113(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt90) -> R<Option<Adt90>> {
    let m345 = v0;
    match m345 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f172(v3, v1)? {
                Ok(None::<Adt90>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f114(v0: List<u64>) -> Option<List<u64>> {
    let m346 = f173(v0);
    match m346 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f116(v1)
        }
    }
}

pub fn f115() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f116(v0: List<u64>) -> Option<List<u64>> {
    let m347 = v0.clone();
    match m347.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a348 = v1;
            let a349 = 91u64;
            if nat_eq(a348, a349) {
                f174(f175(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f174(f176(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f117(p355: List<Adt35>, p356: List<Str>) -> Option<Str> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v0, v1) = m354;
                let m350 = v0;
                match m350.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f177(match v2.clone() {
                            Adt35::C0(h351, _, _, _, _, _, _, _) => {
                                h351
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h352, _, _, _, _, _, _, _) => {
                                    h352
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h353, _, _, _, _, _, _, _) => {
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

pub fn f118(p359: List<Adt12>, p360: List<Adt12>) -> List<Adt12> {
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

pub fn f119(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m363 = f178(match v0.clone() {
        Adt35::C0(h361, _, _, _, _, _, _, _) => {
            h361
        }
    }, f179(match v0.clone() {
        Adt35::C0(h362, _, _, _, _, _, _, _) => {
            h362
        }
    })?);
    match m363 {
        None => {
            f180(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f120(p368: u64, p369: i64) -> R<i64> {
    let m367 = (p368, p369);
    {
        let mut m367 = m367;
        loop {
            let step = {
                let (v0, v1) = m367;
                let m364 = v0;
                if m364 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m364 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a365 = v1;
                        let a366 = -1i64;
                        int_sub(a365, a366)?
                    }))
                }
            };
            match step {
                Ok(next) => { m367 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt98> {
    f181(v0, f182(), Adt100::C0, List::cons(Adt101::C0(v1, v2, List::<Adt99>::nil()), List::<Adt101>::nil()))
}

pub fn f122(v0: Adt21) -> Adt6 {
    let m371 = match v0 {
        Adt21::C0(_, _, h370, _, _) => {
            h370
        }
    };
    match m371 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h372) => {
                    h372
                }
            })
        }
    }
}

pub fn f123(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m374 = {
        let c373: Fn1 = v1;
        c373.apply(v2)?
    };
    match m374 {
        Ok(v4) => {
            f183(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f124(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f125(v0: List<Adt99>, v1: Str) -> R<Str> {
    let v2: Str = f184(v0)?;
    let a377 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a375 = v2.clone();
        let a376 = Str::lit("");
        if equal(a375, a376) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a378 = Str::lit(" ");
    Ok(join(a377, a378))
}

pub fn f126(v0: Option<Adt19>) -> Str {
    let m379 = v0;
    match m379 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m380 = v1;
            match m380 {
                Adt19::C0(v2) => {
                    v2
                }
                Adt19::C1(v3) => {
                    let a381 = v3;
                    format_i64(a381)
                }
            }
        }
    }
}

pub fn f127(v0: u64, v1: u64) -> R<Adt50> {
    Ok(Adt50::C0(f185(v0)?, Some(f185(v1)?), Some({
        let a384 = List::cons(Str::lit("Completed step"), List::cons({
            let a382 = f186(v0, 0i64)?;
            format_i64(a382)
        }, List::cons(Str::lit("of"), List::cons({
            let a383 = f186(v1, 0i64)?;
            format_i64(a383)
        }, List::<Str>::nil()))));
        let a385 = Str::lit(" ");
        join(a384, a385)
    })))
}

pub fn f128() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, Some(f26()), None::<Adt5>, None::<Adt5>)
}

pub fn f129() -> Adt49 {
    f70(List::cons(f71(Str::lit("ok")), List::<Adt38>::nil()))
}

pub fn f130(v0: List<(Adt44, List<Adt46>)>) -> List<Adt44> {
    f187(f188(v0, List::<Adt44>::nil()), List::<Adt44>::nil())
}

pub fn f131() -> List<(Adt44, List<Adt46>)> {
    List::cons(f189(f190(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f145(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f189(f190(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f191(Str::lit("test://static-binary"), Str::lit("image/png"), f146())), List::cons(f189(f190(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f145(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f189(f190(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f145(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt44, List<Adt46>)>::nil()))))
}

pub fn f132(v0: Adt10, v1: Str, v2: Option<List<Adt46>>) -> Result<Adt32, Adt84> {
    let m386 = v2;
    match m386 {
        None => {
            Err::<Adt32, Adt84>(Adt84::C2(v1))
        }
        Some(v3) => {
            let m387 = v3.clone();
            match m387.uncons() {
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

pub fn f133(v0: Str, p396: List<(Adt44, List<Adt46>)>) -> Option<List<Adt46>> {
    let m395 = p396;
    {
        let mut m395 = m395;
        loop {
            let step = {
                let v1 = m395;
                let m388 = v1;
                match m388.uncons() {
                    None => {
                        Err::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(None::<List<Adt46>>)
                    }
                    Some((v2, v3)) => {
                        let a392 = {
                            let m391 = {
                                let (h390, _) = v2;
                                h390
                            };
                            match m391 {
                                Adt44::C0(_, _, _, h389, _, _, _, _, _) => {
                                    h389
                                }
                            }
                        };
                        let a393 = v0.clone();
                        if equal(a392, a393) {
                            Err::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(Some({
                                let (_, h394) = v2;
                                h394
                            }))
                        } else {
                            Ok::<List<(Adt44, List<Adt46>)>, Option<List<Adt46>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: Str) -> R<Option<List<Adt46>>> {
    let m397 = f192(v0.clone())?;
    match m397 {
        None => {
            Ok(None::<List<Adt46>>)
        }
        Some(v1) => {
            let m398 = f193(v1)?;
            match m398 {
                None => {
                    Ok(None::<List<Adt46>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f145(v0, Str::lit("application/json"), v2), List::<Adt46>::nil())))
                }
            }
        }
    }
}

pub fn f135() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f136(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt57>>) -> Adt56 {
    Adt56::C0(v0, Some(v1), None::<List<Adt2>>, Some(v2), v3, None::<Adt5>)
}

pub fn f137(v0: Str, v1: Str) -> Adt57 {
    Adt57::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f138(v0: Str, p404: List<Adt56>) -> Option<Adt56> {
    let m403 = p404;
    {
        let mut m403 = m403;
        loop {
            let step = {
                let v1 = m403;
                let m399 = v1;
                match m399.uncons() {
                    None => {
                        Err::<List<Adt56>, Option<Adt56>>(None::<Adt56>)
                    }
                    Some((v2, v3)) => {
                        let a401 = match v2.clone() {
                            Adt56::C0(h400, _, _, _, _, _) => {
                                h400
                            }
                        };
                        let a402 = v0.clone();
                        if equal(a401, a402) {
                            Err::<List<Adt56>, Option<Adt56>>(Some(v2))
                        } else {
                            Ok::<List<Adt56>, Option<Adt56>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(p410: List<Adt57>, v1: List<(Str, Str)>) -> Option<Str> {
    let m409 = p410;
    {
        let mut m409 = m409;
        loop {
            let step = {
                let v0 = m409;
                let m405 = v0;
                match m405.uncons() {
                    None => {
                        Err::<List<Adt57>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f194(v2.clone()) {
                            let m407 = f195(match v2.clone() {
                                Adt57::C0(h406, _, _, _) => {
                                    h406
                                }
                            }, v1.clone());
                            match m407 {
                                None => {
                                    Err::<List<Adt57>, Option<Str>>(Some(match v2 {
                                        Adt57::C0(h408, _, _, _) => {
                                            h408
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
                Ok(next) => { m409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: Adt56) -> List<Adt57> {
    let m412 = match v0 {
        Adt56::C0(_, _, _, _, h411, _) => {
            h411
        }
    };
    match m412 {
        None => {
            List::<Adt57>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f141(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m413 = v0;
    match m413 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f142(v0: List<Adt59>) -> Adt58 {
    Adt58::C0(None::<Str>, v0)
}

pub fn f143(v0: Str) -> Adt59 {
    Adt59::C0(Adt41::C0, Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)))
}

pub fn f144(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m414 = f195(v0, v1);
    match m414 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f145(v0: Str, v1: Str, v2: Str) -> Adt46 {
    Adt46::C0(Adt47::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f146() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f147(v0: List<Str>) -> R<Adt60> {
    let v1: u64 = f196(v0.clone(), 0u64)?;
    Ok(Adt60::C0(Adt61::C0(f197(v0), Some(v1), Some({
        let a415 = f198();
        let a416 = v1;
        nat_lt(a415, a416)
    }))))
}

pub fn f148(p422: List<Adt57>, v1: Str) -> bool {
    let m421 = p422;
    {
        let mut m421 = m421;
        loop {
            let step = {
                let v0 = m421;
                let m417 = v0;
                match m417.uncons() {
                    None => {
                        Err::<List<Adt57>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a419 = match v2 {
                            Adt57::C0(h418, _, _, _) => {
                                h418
                            }
                        };
                        let a420 = v1.clone();
                        if equal(a419, a420) {
                            Err::<List<Adt57>, bool>(true)
                        } else {
                            Ok::<List<Adt57>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f199(f200(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f150(v0: Str, v1: Str) -> List<Str> {
    let a423 = v0;
    let a424 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a423, a424) {
        let a425 = v1;
        let a426 = Str::lit("resourceUri");
        equal(a425, a426)
    } else {
        false
    } {
        f201()
    } else {
        List::<Str>::nil()
    }
}

pub fn f151() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f152(p429: List<Adt86>, p430: List<Adt86>) -> List<Adt86> {
    let m428 = (p429, p430);
    {
        let mut m428 = m428;
        loop {
            let step = {
                let (v0, v1) = m428;
                let m427 = v0;
                match m427.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: Bytes) -> R<List<u64>> {
    f202(v0.clone(), {
        let a431 = v0;
        length_bytes(a431)
    }, List::<u64>::nil())
}

pub fn f154(v0: Bytes, v1: Adt85, v2: u64) -> R<Result<Adt85, Adt87>> {
    let a433 = v2;
    let a434 = {
        let a432 = v0.clone();
        length_bytes(a432)
    };
    if nat_eq(a433, a434) {
        Ok(Ok::<Adt85, Adt87>(v1))
    } else {
        let m435 = f47(v0.clone(), v2)?;
        match m435 {
            None => {
                Ok(Err::<Adt85, Adt87>(Adt87::C0(v2)))
            }
            Some(v3) => {
                let m438 = f203(v0.clone(), {
                    let a436 = v2;
                    let a437 = 4u64;
                    nat_add(a436, a437)?
                }, v3, List::<List<u64>>::nil())?;
                match m438 {
                    Ok(v5) => {
                        let a441 = {
                            let (_, h439) = v5.clone();
                            h439
                        };
                        let a442 = {
                            let a440 = v0;
                            length_bytes(a440)
                        };
                        if nat_eq(a441, a442) {
                            Ok(Ok::<Adt85, Adt87>(f204(v1, {
                                let (h443, _) = v5;
                                h443
                            })))
                        } else {
                            Ok(Err::<Adt85, Adt87>(Adt87::C1({
                                let (_, h444) = v5;
                                h444
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

pub fn f155(v0: Adt6) -> R<Result<List<u64>, Adt93>> {
    f205(f206(), v0)
}

pub fn f156() -> u64 {
    403u64
}

pub fn f157(v0: Option<Adt92>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f207())), {
        let a445 = f208(v0);
        let a446 = List::cons((Str::lit("error"), f209(v1)), List::<(Str, Adt6)>::nil());
        append_list(a445, a446)
    })
}

pub fn f158() -> i64 {
    -32000i64
}

pub fn f159(v0: Adt90) -> Str {
    let m447 = v0;
    match m447 {
        Adt90::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt90::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f160(p454: List<u64>, p455: List<u64>) -> bool {
    let m453 = (p454, p455);
    {
        let mut m453 = m453;
        loop {
            let step = {
                let (v0, v1) = m453;
                let m448 = v0;
                match m448.uncons() {
                    None => {
                        let m449 = v1;
                        match m449.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m450 = v1;
                        match m450.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a451 = v4;
                                let a452 = v6;
                                if nat_eq(a451, a452) {
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
                Ok(next) => { m453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f162() -> u64 {
    400u64
}

pub fn f163(v0: Adt94) -> Adt80 {
    Adt80::C0(f210(v0.clone()), f211(v0), None::<Adt6>)
}

pub fn f164(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt103, Adt94>) -> R<Adt88> {
    let m456 = v4;
    match m456 {
        Ok(v6) => {
            f214(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f103(f212(v5.clone()), f105(f213(v5.clone()), f163(v5)))
        }
    }
}

pub fn f165(v0: Adt6) -> R<Result<Adt103, Adt94>> {
    let m457 = v0;
    match m457 {
        Adt6::C0 => {
            Ok(f216(None::<Adt92>, Adt95::C0))
        }
        Adt6::C1(_) => {
            Ok(f216(None::<Adt92>, Adt95::C0))
        }
        Adt6::C2(_) => {
            Ok(f216(None::<Adt92>, Adt95::C0))
        }
        Adt6::C3(_) => {
            Ok(f216(None::<Adt92>, Adt95::C0))
        }
        Adt6::C4(_) => {
            Ok(f216(None::<Adt92>, Adt95::C1))
        }
        Adt6::C5(v1) => {
            f215(Adt5::C0(v1))
        }
    }
}

pub fn f166(v0: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m458 = f217(v0.clone())?;
    match m458 {
        Adt112::C0 => {
            f218(f219(), 0u64, v0)
        }
        Adt112::C1(v1) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C0(v1)))
        }
        Adt112::C2(v2) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C1(v2)))
        }
    }
}

pub fn f167() -> u64 {
    405u64
}

pub fn f168() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f169() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f170(p463: List<Adt86>, v1: List<u64>, p464: Adt97) -> R<Adt97> {
    let m462 = (p463, p464);
    {
        let mut m462 = m462;
        loop {
            let step = {
                let (v0, v2) = m462;
                let m459 = v0;
                match m459.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt97), R<Adt97>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f220(match v3.clone() {
                            Adt86::C0(h460, _) => {
                                h460
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, f221(v2, match v3 {
                                Adt86::C0(_, h461) => {
                                    h461
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f222(v0);
    if f223(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f172(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f220(v0.clone(), f224())? {
        Ok(true)
    } else {
        if f220(v0.clone(), f225())? {
            Ok(true)
        } else {
            if f220(v0.clone(), f226())? {
                Ok(true)
            } else {
                f227(v0, v1)
            }
        }
    }
}

pub fn f173(p469: List<u64>) -> Option<List<u64>> {
    let m468 = p469;
    {
        let mut m468 = m468;
        loop {
            let step = {
                let v0 = m468;
                let m465 = v0;
                match m465.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a466 = v1;
                        let a467 = 58u64;
                        if nat_eq(a466, a467) {
                            Err::<List<u64>, Option<List<u64>>>(f228(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m470 = v0;
    match m470 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f229({
                let (_, h471) = v1;
                h471
            }) {
                Some({
                    let (h472, _) = v1;
                    h472
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f175(p477: List<u64>, p478: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m476 = (p477, p478);
    {
        let mut m476 = m476;
        loop {
            let step = {
                let (v0, v1) = m476;
                let m473 = v0;
                match m473.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a474 = v2;
                        let a475 = 93u64;
                        if nat_eq(a474, a475) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f230(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f176(p483: List<u64>, p484: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m482 = (p483, p484);
    {
        let mut m482 = m482;
        loop {
            let step = {
                let (v0, v1) = m482;
                let m479 = v0;
                match m479.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f230(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a480 = v2;
                        let a481 = 58u64;
                        if nat_eq(a480, a481) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f230(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Str, p489: List<Str>) -> bool {
    let m488 = p489;
    {
        let mut m488 = m488;
        loop {
            let step = {
                let v1 = m488;
                let m485 = v1;
                match m485.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a486 = v0.clone();
                        let a487 = v2;
                        if equal(a486, a487) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Str, v1: List<u64>) -> Option<Adt91> {
    if f231(v1) {
        Some(Adt91::C0(v0))
    } else {
        None::<Adt91>
    }
}

pub fn f179(v0: Str) -> R<List<u64>> {
    f153({
        let a490 = v0;
        utf8_encode(a490)
    })
}

pub fn f180(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m491 = f232(v0.clone());
    match m491 {
        None => {
            f233(v0.clone(), f234(f235(), match v0.clone() {
                Adt35::C0(h492, _, _, _, _, _, _, _) => {
                    h492
                }
            }, Adt6::C5({
                let m495 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h494, _, _, _) => {
                        h494
                    }
                };
                match m495 {
                    Adt5::C0(h493) => {
                        h493
                    }
                }
            }), List::cons(Adt104::C0(Adt6::C5({
                let m498 = match v0 {
                    Adt35::C0(_, _, _, _, h497, _, _, _) => {
                        h497
                    }
                };
                match m498 {
                    Adt5::C0(h496) => {
                        h496
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt104>::nil()), f236())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f181(v0: Adt6, p517: u64, p518: Adt100, p519: List<Adt101>) -> R<Adt98> {
    let m516 = (p517, (p518, p519));
    {
        let mut m516 = m516;
        loop {
            let step = {
                let (v1, (v2, v3)) = m516;
                let m499 = v1;
                if m499 == 0 {
                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C4))
                } else {
                    let v4 = m499 - 1;
                    let m500 = v3;
                    match m500.uncons() {
                        None => {
                            let m501 = v2;
                            match m501 {
                                Adt100::C0 => {
                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C0))
                                }
                                Adt100::C1(v5, v6) => {
                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m502 = v2.clone();
                            match m502 {
                                Adt100::C0 => {
                                    let m503 = v7.clone();
                                    match m503 {
                                        Adt101::C0(v9, v10, v11) => {
                                            let m504 = f237(v0.clone(), v9, v10, v11.clone())?;
                                            match m504 {
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
                                                        let a505 = v15;
                                                        let a506 = v8;
                                                        append_list(a505, a506)
                                                    };
                                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt101::C1(_, _, _) => {
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v8)))
                                        }
                                        Adt101::C2(_, _, _, _) => {
                                            let v24: Adt105 = f238(v7, v8)?;
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v24.clone() {
                                                Adt105::C0(h507, _) => {
                                                    h507
                                                }
                                            }, match v24 {
                                                Adt105::C0(_, h508) => {
                                                    h508
                                                }
                                            })))
                                        }
                                        Adt101::C3(v25) => {
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (Adt100::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt101::C4(_, _, _, _) => {
                                            let v39: Adt105 = f238(v7, v8)?;
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v39.clone() {
                                                Adt105::C0(h512, _) => {
                                                    h512
                                                }
                                            }, match v39 {
                                                Adt105::C0(_, h513) => {
                                                    h513
                                                }
                                            })))
                                        }
                                        Adt101::C5(v26, v27, v28, v29) => {
                                            let m509 = f239(v26, v27, v28, v29.clone());
                                            match m509 {
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
                                                        let a510 = v33;
                                                        let a511 = v8;
                                                        append_list(a510, a511)
                                                    };
                                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt100::C1(v40, v41) => {
                                    let v42: Adt105 = f240(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v42.clone() {
                                        Adt105::C0(h514, _) => {
                                            h514
                                        }
                                    }, match v42 {
                                        Adt105::C0(_, h515) => {
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

pub fn f182() -> u64 {
    67108864u64
}

pub fn f183(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f241({
        let m522 = match v1.clone() {
            Adt49::C0(h521, _) => {
                h521
            }
        };
        match m522 {
            Adt37::C0(h520, _, _) => {
                h520
            }
        }
    })? {
        f242(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f184(v0: List<Adt99>) -> R<Str> {
    let a523 = List::cons(Str::lit(""), f243(v0, List::<Str>::nil())?);
    let a524 = Str::lit("/");
    Ok(join(a523, a524))
}

pub fn f185(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f186(v0, 0i64)?, 0i64))
}

pub fn f186(p527: u64, p528: i64) -> R<i64> {
    let m526 = (p527, p528);
    {
        let mut m526 = m526;
        loop {
            let step = {
                let (v0, v1) = m526;
                let m525 = v0;
                if m525 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m525 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f244(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(p531: List<Adt44>, p532: List<Adt44>) -> List<Adt44> {
    let m530 = (p531, p532);
    {
        let mut m530 = m530;
        loop {
            let step = {
                let (v0, v1) = m530;
                let m529 = v0;
                match m529.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt44>), List<Adt44>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt44>), List<Adt44>>((v3, List::cons(v2, v1)))
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

pub fn f188(p536: List<(Adt44, List<Adt46>)>, p537: List<Adt44>) -> List<Adt44> {
    let m535 = (p536, p537);
    {
        let mut m535 = m535;
        loop {
            let step = {
                let (v0, v1) = m535;
                let m533 = v0;
                match m533.uncons() {
                    None => {
                        Err::<(List<(Adt44, List<Adt46>)>, List<Adt44>), List<Adt44>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt44, List<Adt46>)>, List<Adt44>), List<Adt44>>((v3, List::cons({
                            let (h534, _) = v2;
                            h534
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Adt44, v1: Adt46) -> (Adt44, List<Adt46>) {
    (v0, List::cons(v1, List::<Adt46>::nil()))
}

pub fn f190(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt44 {
    Adt44::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt40>, None::<u64>, None::<Adt5>)
}

pub fn f191(v0: Str, v1: Str, v2: Str) -> Adt46 {
    Adt46::C1(Adt48::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f192(v0: Str) -> R<Option<Str>> {
    let m538 = f245(v0)?;
    match m538 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a539 = f246(f247(), v1);
            Ok(utf8_decode(a539))
        }
    }
}

pub fn f193(v0: Str) -> R<Option<Str>> {
    let m542 = f248(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a540 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a541 = Str::lit(" ");
        join(a540, a541)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m542 {
        Ok(v1) => {
            let a543 = v1;
            Ok(utf8_decode(a543))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f194(v0: Adt57) -> bool {
    let m545 = match v0 {
        Adt57::C0(_, _, _, h544) => {
            h544
        }
    };
    match m545 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f195(v0: Str, p552: List<(Str, Str)>) -> Option<Str> {
    let m551 = p552;
    {
        let mut m551 = m551;
        loop {
            let step = {
                let v1 = m551;
                let m546 = v1;
                match m546.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a548 = {
                            let (h547, _) = v2;
                            h547
                        };
                        let a549 = v0.clone();
                        if equal(a548, a549) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h550) = v2;
                                h550
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(p557: List<Str>, p558: u64) -> R<u64> {
    let m556 = (p557, p558);
    {
        let mut m556 = m556;
        loop {
            let step = {
                let (v0, v1) = m556;
                let m553 = v0;
                match m553.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a554 = v1;
                            let a555 = 1u64;
                            nat_add(a554, a555)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: List<Str>) -> List<Str> {
    f199(f249(v0, f198(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f198() -> u64 {
    100u64
}

pub fn f199(p561: List<Str>, p562: List<Str>) -> List<Str> {
    let m560 = (p561, p562);
    {
        let mut m560 = m560;
        loop {
            let step = {
                let (v0, v1) = m560;
                let m559 = v0;
                match m559.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
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

pub fn f200(v0: Str, p565: List<Str>, p566: List<Str>) -> R<List<Str>> {
    let m564 = (p565, p566);
    {
        let mut m564 = m564;
        loop {
            let step = {
                let (v1, v2) = m564;
                let m563 = v1;
                match m563.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f250(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201() -> List<Str> {
    f199(f251(f86(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f202(v0: Bytes, p569: u64, p570: List<u64>) -> R<List<u64>> {
    let m568 = (p569, p570);
    {
        let mut m568 = m568;
        loop {
            let step = {
                let (v1, v2) = m568;
                let m567 = v1;
                if m567 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m567 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f98(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes, p576: u64, p577: u64, p578: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt87>> {
    let m575 = (p576, (p577, p578));
    {
        let mut m575 = m575;
        loop {
            let step = {
                let (v1, (v2, v3)) = m575;
                let m571 = v2;
                if m571 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Ok::<(List<List<u64>>, u64), Adt87>((f52(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m571 - 1;
                    let m572 = f16(v0.clone(), v1)?;
                    match m572 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Err::<(List<List<u64>>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>((match v5.clone() {
                                Adt89::C0(_, h573) => {
                                    h573
                                }
                            }, (v4, List::cons(f153(match v5 {
                                Adt89::C0(h574, _) => {
                                    h574
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Adt85, v1: List<List<u64>>) -> Adt85 {
    Adt85::C0(match v0.clone() {
        Adt85::C0(h579, _, _, _, _, _) => {
            h579
        }
    }, match v0.clone() {
        Adt85::C0(_, h580, _, _, _, _) => {
            h580
        }
    }, match v0.clone() {
        Adt85::C0(_, _, h581, _, _, _) => {
            h581
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, h582, _, _) => {
            h582
        }
    }, match v0 {
        Adt85::C0(_, _, _, _, h583, _) => {
            h583
        }
    }, v1)
}

pub fn f205(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt93>> {
    let m584 = f252(v0, List::cons(Adt106::C1(0u64, v1), List::<Adt106>::nil()), List::<u64>::nil())?;
    match m584 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt93>(f253(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt93>(v3))
        }
    }
}

pub fn f206() -> u64 {
    67108864u64
}

pub fn f207() -> Str {
    Str::lit("2.0")
}

pub fn f208(v0: Option<Adt92>) -> List<(Str, Adt6)> {
    let m585 = v0;
    match m585 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f254(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f209(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h586, _, _) => {
            h586
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h587, _) => {
            h587
        }
    })), f255(match v0 {
        Adt80::C0(_, _, h588) => {
            h588
        }
    }))))
}

pub fn f210(v0: Adt94) -> i64 {
    let m589 = v0;
    match m589 {
        Adt94::C0 => {
            f256()
        }
        Adt94::C1(_, _) => {
            f257()
        }
    }
}

pub fn f211(v0: Adt94) -> Str {
    let m590 = v0;
    match m590 {
        Adt94::C0 => {
            Str::lit("Parse error")
        }
        Adt94::C1(_, v2) => {
            f258(v2)
        }
    }
}

pub fn f212(v0: Adt94) -> u64 {
    let m591 = v0;
    match m591 {
        Adt94::C0 => {
            f162()
        }
        Adt94::C1(_, _) => {
            f162()
        }
    }
}

pub fn f213(v0: Adt94) -> Option<Adt92> {
    let m592 = v0;
    match m592 {
        Adt94::C0 => {
            None::<Adt92>
        }
        Adt94::C1(v1, _) => {
            v1
        }
    }
}

pub fn f214(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt103) -> R<Adt88> {
    f259(f260(v3.clone())?, f261(v0.clone(), v1, v2, Some(f262(v3, match v0 {
        Adt0::C0(_, _, _, _, h593, _) => {
            h593
        }
    }, f263(v4.clone()), match v4.clone() {
        Adt103::C0(_, h594, _) => {
            h594
        }
    })?), f264(), v4)?)
}

pub fn f215(v0: Adt5) -> R<Result<Adt103, Adt94>> {
    let m595 = f265(v0.clone())?;
    match m595 {
        Adt129::C0 => {
            let m596 = f266(Str::lit("method"), v0.clone());
            match m596 {
                Adt130::C0 => {
                    Ok(f216(None::<Adt92>, f267(v0)))
                }
                Adt130::C1(_) => {
                    Ok(f216(None::<Adt92>, if f268(v0) {
                        Adt95::C8
                    } else {
                        Adt95::C7
                    }))
                }
            }
        }
        Adt129::C1(v2) => {
            Ok(f269(v2, v0))
        }
        Adt129::C2 => {
            Ok(f216(None::<Adt92>, Adt95::C5))
        }
    }
}

pub fn f216(v0: Option<Adt92>, v1: Adt95) -> Result<Adt103, Adt94> {
    Err::<Adt103, Adt94>(Adt94::C1(v0, v1))
}

pub fn f217(v0: List<u64>) -> R<Adt112> {
    Ok(f270({
        let a597 = v0.clone();
        length_list(a597)
    }, f271(Adt113::C0, 0u64, 0u64, v0)?))
}

pub fn f218(p604: Adt114, p605: u64, p606: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m603 = (p604, (p605, p606));
    {
        let mut m603 = m603;
        loop {
            let step = {
                let (v0, (v1, v2)) = m603;
                let m598 = v2;
                match m598.uncons() {
                    None => {
                        let m599 = f272(v0, v1)?;
                        match m599 {
                            Ok(v3) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Ok::<Adt6, Adt93>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m600 = f273(v0, v1, v5)?;
                        match m600 {
                            Adt131::C0(v7) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v7)))
                            }
                            Adt131::C1(v8) => {
                                Ok::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>((v8, ({
                                    let a601 = v1;
                                    let a602 = 1u64;
                                    nat_add(a601, a602)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m603 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219() -> Adt114 {
    Adt114::C0(Adt115::C0, List::<Adt119>::nil(), 0u64)
}

pub fn f220(p613: List<u64>, p614: List<u64>) -> R<bool> {
    let m612 = (p613, p614);
    {
        let mut m612 = m612;
        loop {
            let step = {
                let (v0, v1) = m612;
                let m607 = v0;
                match m607.uncons() {
                    None => {
                        let m608 = v1;
                        match m608.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m609 = v1;
                        match m609.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a610 = f274(v4)?;
                                let a611 = f274(v6)?;
                                if nat_eq(a610, a611) {
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
                Ok(next) => { m612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Adt97, v1: List<u64>) -> Adt97 {
    let m615 = v0;
    match m615 {
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

pub fn f222(v0: List<u64>) -> List<u64> {
    f230(f275(f230(f275(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f223(p618: List<u64>) -> bool {
    let m617 = p618;
    {
        let mut m617 = m617;
        loop {
            let step = {
                let v0 = m617;
                let m616 = v0;
                match m616.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f276(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f225() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f226() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f227(v0: List<u64>, p621: List<List<u64>>) -> R<bool> {
    let m620 = p621;
    {
        let mut m620 = m620;
        loop {
            let step = {
                let v1 = m620;
                let m619 = v1;
                match m619.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f220(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228(v0: List<u64>) -> Option<List<u64>> {
    let m622 = v0;
    match m622.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m623 = v2;
            match m623.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a624 = v1;
                    let a625 = 47u64;
                    if if nat_eq(a624, a625) {
                        let a626 = v3;
                        let a627 = 47u64;
                        nat_eq(a626, a627)
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

pub fn f229(v0: List<u64>) -> bool {
    let m628 = v0;
    match m628.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a629 = v1;
            let a630 = 58u64;
            if nat_eq(a629, a630) {
                let m631 = v2.clone();
                match m631.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f277(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f230(p634: List<u64>, p635: List<u64>) -> List<u64> {
    let m633 = (p634, p635);
    {
        let mut m633 = m633;
        loop {
            let step = {
                let (v0, v1) = m633;
                let m632 = v0;
                match m632.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m633 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: List<u64>) -> bool {
    let m636 = v0.clone();
    match m636.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a640 = if f278(v0.clone(), false) {
                let a638 = {
                    let a637 = v0;
                    length_list(a637)
                };
                let a639 = f279();
                nat_le(a638, a639)
            } else {
                false
            };
            bool_not(a640)
        }
    }
}

pub fn f232(v0: Adt35) -> Option<Adt91> {
    let m645 = f280(match v0.clone() {
        Adt35::C0(h641, _, _, _, _, _, _, _) => {
            h641
        }
    }, {
        let m644 = match v0.clone() {
            Adt35::C0(_, _, _, _, h643, _, _, _) => {
                h643
            }
        };
        match m644 {
            Adt5::C0(h642) => {
                h642
            }
        }
    });
    match m645 {
        None => {
            f281(match v0.clone() {
                Adt35::C0(h646, _, _, _, _, _, _, _) => {
                    h646
                }
            }, {
                let m649 = match v0 {
                    Adt35::C0(_, _, _, _, h648, _, _, _) => {
                        h648
                    }
                };
                match m649 {
                    Adt5::C0(h647) => {
                        h647
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f233(v0: Adt35, v1: Result<Adt120, Adt91>) -> R<Result<List<Adt12>, Adt91>> {
    let m650 = v1;
    match m650 {
        Ok(v3) => {
            let m651 = f282(v0)?;
            match m651 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt91>(f118(match v3 {
                        Adt120::C0(h652, _) => {
                            h652
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

pub fn f234(p659: u64, v1: Str, v2: Adt6, p660: List<Adt104>, p661: Adt120) -> R<Result<Adt120, Adt91>> {
    let m658 = (p659, (p660, p661));
    {
        let mut m658 = m658;
        loop {
            let step = {
                let (v0, (v3, v4)) = m658;
                let m653 = v0;
                if m653 == 0 {
                    Err::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>(Ok(Err::<Adt120, Adt91>(Adt91::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m653 - 1;
                    let m654 = v3;
                    match m654.uncons() {
                        None => {
                            Err::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>(Ok(Ok::<Adt120, Adt91>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m655 = f283(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m655 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>((v5, ({
                                        let (h656, _) = v9;
                                        h656
                                    }, {
                                        let (_, h657) = v9;
                                        h657
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
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235() -> u64 {
    67108864u64
}

pub fn f236() -> Adt120 {
    Adt120::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f237(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    let m662 = f284(v1.clone());
    match m662 {
        None => {
            let m663 = f285(v1);
            match m663 {
                None => {
                    Ok(Adt121::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f286(v0, match v5 {
                        Adt5::C0(h664) => {
                            h664
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

pub fn f238(v0: Adt101, v1: List<Adt101>) -> R<Adt105> {
    let m665 = v0;
    match m665 {
        Adt101::C0(_, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
        Adt101::C1(_, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
        Adt101::C2(v8, v9, v10, v11) => {
            Ok(f287(v8, v9, v10, {
                let a666 = v11;
                let a667 = 1u64;
                nat_add(a666, a667)?
            }, v1))
        }
        Adt101::C3(v12) => {
            Ok(Adt105::C0(Adt100::C1(v12, Str::lit("not")), v1))
        }
        Adt101::C4(v13, _, v15, v16) => {
            Ok(f288(v13, v15, v16, v1))
        }
        Adt101::C5(_, _, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
    }
}

pub fn f239(v0: Adt102, v1: Adt6, v2: Adt6, v3: List<Adt99>) -> Adt121 {
    let m668 = f285(v1);
    match m668 {
        None => {
            Adt121::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m669 = v0;
            match m669 {
                Adt102::C0 => {
                    f289(match v4 {
                        Adt5::C0(h670) => {
                            h670
                        }
                    }, v2, v3)
                }
                Adt102::C1 => {
                    f290(match v4 {
                        Adt5::C0(h671) => {
                            h671
                        }
                    }, v2, v3)
                }
                Adt102::C2 => {
                    f291(match v4 {
                        Adt5::C0(h672) => {
                            h672
                        }
                    }, v2, v3)
                }
                Adt102::C3 => {
                    f292(match v4 {
                        Adt5::C0(h673) => {
                            h673
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f240(v0: Adt101, v1: List<Adt101>, v2: List<Adt99>, v3: Str) -> Adt105 {
    let m674 = v0;
    match m674 {
        Adt101::C0(_, _, _) => {
            Adt105::C0(Adt100::C1(v2, v3), v1)
        }
        Adt101::C1(v7, v8, v9) => {
            f293(v7, v8, v9, v1)
        }
        Adt101::C2(v10, v11, v12, v13) => {
            f287(v10, v11, v12, v13, v1)
        }
        Adt101::C3(_) => {
            Adt105::C0(Adt100::C0, v1)
        }
        Adt101::C4(_, v16, v17, v18) => {
            f288(v16, v17, v18, v1)
        }
        Adt101::C5(_, _, _, _) => {
            Adt105::C0(Adt100::C1(v2, v3), v1)
        }
    }
}

pub fn f241(p683: List<Adt38>) -> R<bool> {
    let m682 = p683;
    {
        let mut m682 = m682;
        loop {
            let step = {
                let v0 = m682;
                let m675 = v0;
                match m675.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m676 = v1;
                        match m676 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f294(match v4 {
                                    Adt42::C0(h677, _, _, _) => {
                                        h677
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f294(match v5 {
                                    Adt43::C0(h678, _, _, _) => {
                                        h678
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
                                let m680 = match v7 {
                                    Adt45::C0(h679, _, _) => {
                                        h679
                                    }
                                };
                                match m680 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f294(match v9 {
                                            Adt48::C0(_, _, h681, _) => {
                                                h681
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
                Ok(next) => { m682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f295(match v1.clone() {
        Adt49::C0(h684, _) => {
            h684
        }
    }) {
        f296(v1)
    } else {
        let m686 = match v0 {
            Adt35::C0(_, _, _, _, _, h685, _, _) => {
                h685
            }
        };
        match m686 {
            None => {
                f296(v1)
            }
            Some(v2) => {
                f297(v2, v1)
            }
        }
    }
}

pub fn f243(p689: List<Adt99>, p690: List<Str>) -> R<List<Str>> {
    let m688 = (p689, p690);
    {
        let mut m688 = m688;
        loop {
            let step = {
                let (v0, v1) = m688;
                let m687 = v0;
                match m687.uncons() {
                    None => {
                        Err::<(List<Adt99>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt99>, List<Str>), R<List<Str>>>((v3, List::cons(f298(v2)?, v1)))
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

pub fn f244(v0: i64, v1: i64) -> R<i64> {
    let a692 = v0;
    let a693 = {
        let a691 = v1;
        int_neg(a691)?
    };
    int_sub(a692, a693)
}

pub fn f245(v0: Str) -> R<Option<List<u64>>> {
    let m694 = f299(f300(f301())?, f300(v0)?);
    match m694 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m695 = f299(f302(List::cons(47u64, f300(f303())?)), f302(v1));
            match m695 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f302(v2);
                    if if f304(v3.clone()) {
                        f305(v3.clone())
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

pub fn f246(v0: Bytes, v1: List<u64>) -> Bytes {
    let a696 = v0;
    let a697 = f306(f247(), f307(List::<Bytes>::nil(), v1));
    append_bytes(a696, a697)
}

pub fn f247() -> Bytes {
    Bytes::lit(&[])
}

pub fn f248(v0: Adt6) -> R<Result<Bytes, Adt93>> {
    let m698 = f155(v0)?;
    match m698 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt93>(f246(f247(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt93>(v2))
        }
    }
}

pub fn f249(p702: List<Str>, p703: u64, p704: List<Str>) -> List<Str> {
    let m701 = (p702, (p703, p704));
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v0, (v1, v2)) = m701;
                let m699 = v0;
                match m699.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m700 = v1;
                        if m700 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m700 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: Str, v1: Str) -> R<bool> {
    Ok(f308(f300(v0)?, f300(v1)?))
}

pub fn f251(p708: List<Adt44>, p709: List<Str>) -> List<Str> {
    let m707 = (p708, p709);
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v0, v1) = m707;
                let m705 = v0;
                match m705.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt44::C0(_, _, _, h706, _, _, _, _, _) => {
                                h706
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(p714: u64, p715: List<Adt106>, p716: List<u64>) -> R<Result<List<u64>, Adt93>> {
    let m713 = (p714, (p715, p716));
    {
        let mut m713 = m713;
        loop {
            let step = {
                let (v0, (v1, v2)) = m713;
                let m710 = v0;
                if m710 == 0 {
                    Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Err::<List<u64>, Adt93>(Adt93::C13)))
                } else {
                    let v3 = m710 - 1;
                    let m711 = v1;
                    match m711.uncons() {
                        None => {
                            Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Ok::<List<u64>, Adt93>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m712 = f309(v4, v5, v2)?;
                            match m712 {
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
                Ok(next) => { m713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(p719: List<u64>, p720: List<u64>) -> List<u64> {
    let m718 = (p719, p720);
    {
        let mut m718 = m718;
        loop {
            let step = {
                let (v0, v1) = m718;
                let m717 = v0;
                match m717.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f254(v0: Adt92) -> Adt6 {
    let m721 = v0;
    match m721 {
        Adt92::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt92::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f255(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m722 = v0;
    match m722 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f256() -> i64 {
    -32700i64
}

pub fn f257() -> i64 {
    -32600i64
}

pub fn f258(v0: Adt95) -> Str {
    let m723 = v0;
    match m723 {
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

pub fn f259(v0: bool, v1: Adt122) -> R<Adt88> {
    let m725 = match v1.clone() {
        Adt122::C0(_, h724) => {
            h724
        }
    };
    match m725 {
        Ok(v2) => {
            let m726 = f310(v2.clone())?;
            match m726 {
                None => {
                    Ok(f6())
                }
                Some(v3) => {
                    Ok(f311(v0, f103(f312(), f313(match v1 {
                        Adt122::C0(h727, _) => {
                            h727
                        }
                    }, v3))?, f314(match v2 {
                        Adt123::C0(_, _, h728) => {
                            h728
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f103(f315(v4.clone()), f105(Some(match v1 {
                Adt122::C0(h729, _) => {
                    h729
                }
            }), f316(v4)?))
        }
    }
}

pub fn f260(p734: List<Adt86>) -> R<bool> {
    let m733 = p734;
    {
        let mut m733 = m733;
        loop {
            let step = {
                let v0 = m733;
                let m730 = v0;
                match m730.uncons() {
                    None => {
                        Err::<List<Adt86>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f220(match v1.clone() {
                            Adt86::C0(h731, _) => {
                                h731
                            }
                        }, f317())? {
                            if f318(f319(44u64, match v1 {
                                Adt86::C0(_, h732) => {
                                    h732
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
                Ok(next) => { m733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt107>, v4: List<Str>, v5: Adt103) -> R<Adt122> {
    Ok(Adt122::C0(match v5.clone() {
        Adt103::C0(h735, _, _) => {
            h735
        }
    }, f320(v0, v1, v2, f321(v3, v4, v5)?)?))
}

pub fn f262(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt107> {
    let v4: Adt128 = f322(Adt109::C0, f111(v0.clone(), f323())?);
    let v5: Adt128 = f322(Adt109::C1, f111(v0.clone(), f324())?);
    let v6: Adt128 = if f325(v3) {
        f322(Adt109::C2, f326(f111(v0.clone(), f327())?)?)
    } else {
        f328()
    };
    Ok(Adt107::C0(match v4.clone() {
        Adt128::C0(h736, _) => {
            h736
        }
    }, match v5.clone() {
        Adt128::C0(h737, _) => {
            h737
        }
    }, match v6.clone() {
        Adt128::C0(h738, _) => {
            h738
        }
    }, f329(match v4 {
        Adt128::C0(_, h739) => {
            h739
        }
    }, f329(match v5 {
        Adt128::C0(_, h740) => {
            h740
        }
    }, match v6 {
        Adt128::C0(_, h741) => {
            h741
        }
    })), f330(v0, v1, v2)?))
}

pub fn f263(v0: Adt103) -> Option<Str> {
    let a743 = match v0.clone() {
        Adt103::C0(_, h742, _) => {
            h742
        }
    };
    let a744 = Str::lit("tools/call");
    if equal(a743, a744) {
        let m746 = match v0 {
            Adt103::C0(_, _, h745) => {
                h745
            }
        };
        match m746 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m747 = f266(Str::lit("name"), v1);
                match m747 {
                    Adt130::C0 => {
                        None::<Str>
                    }
                    Adt130::C1(v2) => {
                        f331(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f264() -> List<Str> {
    List::cons(f332(), List::<Str>::nil())
}

pub fn f265(v0: Adt5) -> R<Adt129> {
    let m748 = f266(Str::lit("id"), v0);
    match m748 {
        Adt130::C0 => {
            Ok(Adt129::C0)
        }
        Adt130::C1(v1) => {
            let m749 = v1;
            match m749 {
                Adt6::C0 => {
                    Ok(Adt129::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt129::C2)
                }
                Adt6::C2(v3) => {
                    let m750 = f333(v3)?;
                    match m750 {
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

pub fn f266(v0: Str, v1: Adt5) -> Adt130 {
    f334(v0, match v1 {
        Adt5::C0(h751) => {
            h751
        }
    })
}

pub fn f267(v0: Adt5) -> Adt95 {
    if f268(v0) {
        Adt95::C8
    } else {
        Adt95::C3
    }
}

pub fn f268(v0: Adt5) -> bool {
    if f335(Str::lit("result"), v0.clone()) {
        true
    } else {
        f335(Str::lit("error"), v0)
    }
}

pub fn f269(v0: Adt92, v1: Adt5) -> Result<Adt103, Adt94> {
    let m752 = f266(Str::lit("jsonrpc"), v1.clone());
    match m752 {
        Adt130::C0 => {
            f216(Some(v0), Adt95::C2)
        }
        Adt130::C1(v2) => {
            let m753 = f331(v2);
            match m753 {
                None => {
                    f216(Some(v0), Adt95::C2)
                }
                Some(v3) => {
                    let a754 = v3;
                    let a755 = f207();
                    if equal(a754, a755) {
                        f336(v0, v1)
                    } else {
                        f216(Some(v0), Adt95::C2)
                    }
                }
            }
        }
    }
}

pub fn f270(v0: u64, v1: u64) -> Adt112 {
    let a756 = v1;
    let a757 = 0u64;
    if nat_eq(a756, a757) {
        Adt112::C0
    } else {
        let a758 = v1;
        let a759 = 2u64;
        if nat_eq(a758, a759) {
            Adt112::C2(v0)
        } else {
            Adt112::C1({
                let a763 = v0;
                let a764 = {
                    let a760 = v1;
                    let a761 = 3u64;
                    let a762 = 0u64;
                    nat_quot(a760, a761, a762)
                };
                nat_sub(a763, a764)
            })
        }
    }
}

pub fn f271(p792: Adt113, p793: u64, p794: u64, p795: List<u64>) -> R<u64> {
    let m791 = (p792, (p793, (p794, p795)));
    {
        let mut m791 = m791;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m791;
                let m765 = v3;
                match m765.uncons() {
                    None => {
                        let m766 = v0;
                        match m766 {
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
                        let m767 = v0;
                        match m767 {
                            Adt113::C0 => {
                                let a768 = v4;
                                let a769 = 128u64;
                                if nat_lt(a768, a769) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a770 = v4;
                                    let a771 = 194u64;
                                    if nat_lt(a770, a771) {
                                        let v6: u64 = f337({
                                            let a772 = List::cons(v4, v5);
                                            length_list(a772)
                                        })?;
                                        Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a773 = v4;
                                        let a774 = 224u64;
                                        if nat_lt(a773, a774) {
                                            Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a775 = v4;
                                            let a776 = 224u64;
                                            if nat_eq(a775, a776) {
                                                Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a777 = v4;
                                                let a778 = 237u64;
                                                if nat_eq(a777, a778) {
                                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a779 = v4;
                                                    let a780 = 240u64;
                                                    if nat_lt(a779, a780) {
                                                        Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a781 = v4;
                                                        let a782 = 240u64;
                                                        if nat_eq(a781, a782) {
                                                            Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a783 = v4;
                                                            let a784 = 244u64;
                                                            if nat_eq(a783, a784) {
                                                                Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a785 = v4;
                                                                let a786 = 244u64;
                                                                if nat_lt(a785, a786) {
                                                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f337({
                                                                        let a787 = List::cons(v4, v5);
                                                                        length_list(a787)
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
                                if f338(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f337({
                                        let a788 = List::cons(v4, v5);
                                        length_list(a788)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt113::C2 => {
                                if f338(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f337({
                                        let a789 = List::cons(v4, v5);
                                        length_list(a789)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt113::C3 => {
                                if f338(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f337({
                                        let a790 = List::cons(v4, v5);
                                        length_list(a790)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m791 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272(v0: Adt114, v1: u64) -> R<Result<Adt6, Adt93>> {
    let m797 = match v0.clone() {
        Adt114::C0(h796, _, _) => {
            h796
        }
    };
    match m797 {
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
            let m798 = f339(v3)?;
            match m798 {
                Adt135::C0 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C7(v1)))
                }
                Adt135::C1 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C8(v1)))
                }
                Adt135::C2(v4) => {
                    let m803 = {
                        let m802 = f340(v4, match v0.clone() {
                            Adt114::C0(_, h800, _) => {
                                h800
                            }
                        }, match v0 {
                            Adt114::C0(_, _, h801) => {
                                h801
                            }
                        });
                        match m802 {
                            Adt114::C0(h799, _, _) => {
                                h799
                            }
                        }
                    };
                    match m803 {
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

pub fn f273(v0: Adt114, v1: u64, v2: u64) -> R<Adt131> {
    let m805 = match v0.clone() {
        Adt114::C0(h804, _, _) => {
            h804
        }
    };
    match m805 {
        Adt115::C0 => {
            if f341(v2) {
                Ok(Adt131::C1(v0))
            } else {
                f342(v0, v1, v2)
            }
        }
        Adt115::C1 => {
            if f341(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a806 = v2;
                let a807 = 93u64;
                if nat_eq(a806, a807) {
                    Ok(f343(v0, v1))
                } else {
                    f342(v0, v1, v2)
                }
            }
        }
        Adt115::C2 => {
            Ok(f344(v0, v1, v2))
        }
        Adt115::C3 => {
            if f341(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a808 = v2;
                let a809 = 125u64;
                if nat_eq(a808, a809) {
                    Ok(f345(v0, v1))
                } else {
                    let a810 = v2;
                    let a811 = 34u64;
                    if nat_eq(a810, a811) {
                        Ok(f346(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f347(Adt93::C2(v1)))
                    }
                }
            }
        }
        Adt115::C4 => {
            if f341(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a812 = v2;
                let a813 = 34u64;
                if nat_eq(a812, a813) {
                    Ok(f346(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f347(Adt93::C2(v1)))
                }
            }
        }
        Adt115::C5 => {
            if f341(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a814 = v2;
                let a815 = 58u64;
                if nat_eq(a814, a815) {
                    Ok(f346(v0, Adt115::C0))
                } else {
                    Ok(f347(Adt93::C2(v1)))
                }
            }
        }
        Adt115::C6(v3, v4, v5) => {
            f348(v0, v1, v2, v3, v4, v5)
        }
        Adt115::C7(v6, v7) => {
            f349(v0, v1, v2, v6, v7)
        }
        Adt115::C8(v8) => {
            f350(v0, v1, v2, v8)
        }
        Adt115::C9(_) => {
            Ok(f344(v0, v1, v2))
        }
    }
}

pub fn f274(v0: u64) -> R<u64> {
    let a816 = 64u64;
    let a817 = v0;
    if if nat_lt(a816, a817) {
        let a818 = v0;
        let a819 = 91u64;
        nat_lt(a818, a819)
    } else {
        false
    } {
        let a820 = v0;
        let a821 = 32u64;
        nat_add(a820, a821)
    } else {
        Ok(v0)
    }
}

pub fn f275(p824: List<u64>) -> List<u64> {
    let m823 = p824;
    {
        let mut m823 = m823;
        loop {
            let step = {
                let v0 = m823;
                let m822 = v0;
                match m822.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f351(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: u64) -> bool {
    let a825 = v0;
    let a826 = 9u64;
    if nat_eq(a825, a826) {
        true
    } else {
        let a827 = 31u64;
        let a828 = v0;
        if nat_lt(a827, a828) {
            let a829 = v0;
            let a830 = 127u64;
            nat_lt(a829, a830)
        } else {
            false
        }
    }
}

pub fn f277(p833: List<u64>) -> bool {
    let m832 = p833;
    {
        let mut m832 = m832;
        loop {
            let step = {
                let v0 = m832;
                let m831 = v0;
                match m831.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f352(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f278(p836: List<u64>, v1: bool) -> bool {
    let m835 = p836;
    {
        let mut m835 = m835;
        loop {
            let step = {
                let v0 = m835;
                let m834 = v0;
                match m834.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f353(v2)
                        } else {
                            f354(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m835 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279() -> u64 {
    64u64
}

pub fn f280(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m837 = f334(Str::lit("type"), v1);
    match m837 {
        Adt130::C0 => {
            Some(Adt91::C3(v0))
        }
        Adt130::C1(v2) => {
            let m838 = f331(v2);
            match m838 {
                None => {
                    Some(Adt91::C3(v0))
                }
                Some(v3) => {
                    let a839 = v3;
                    let a840 = Str::lit("object");
                    if equal(a839, a840) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f281(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m841 = f334(Str::lit("$schema"), v1);
    match m841 {
        Adt130::C0 => {
            None::<Adt91>
        }
        Adt130::C1(v2) => {
            let m842 = f331(v2);
            match m842 {
                None => {
                    Some(Adt91::C2(v0))
                }
                Some(v3) => {
                    let a843 = v3;
                    let a844 = f355();
                    if equal(a843, a844) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f282(v0: Adt35) -> R<Option<Adt91>> {
    let m846 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h845, _, _) => {
            h845
        }
    };
    match m846 {
        None => {
            Ok(None::<Adt91>)
        }
        Some(v1) => {
            let m849 = f281(match v0.clone() {
                Adt35::C0(h847, _, _, _, _, _, _, _) => {
                    h847
                }
            }, match v1.clone() {
                Adt5::C0(h848) => {
                    h848
                }
            });
            match m849 {
                None => {
                    let m853 = f234(f235(), match v0 {
                        Adt35::C0(h850, _, _, _, _, _, _, _) => {
                            h850
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h851) => {
                            h851
                        }
                    }), List::cons(Adt104::C0(Adt6::C5(match v1 {
                        Adt5::C0(h852) => {
                            h852
                        }
                    }), false, List::<Str>::nil()), List::<Adt104>::nil()), f236())?;
                    match m853 {
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

pub fn f283(v0: Str, v1: Adt6, v2: Adt104, v3: Adt120, v4: List<Adt104>) -> R<Result<(List<Adt104>, Adt120), Adt91>> {
    let m854 = v2;
    match m854 {
        Adt104::C0(v5, v6, v7) => {
            let m855 = f284(v5.clone());
            match m855 {
                None => {
                    let m856 = f285(v5);
                    match m856 {
                        None => {
                            Ok(Err::<(List<Adt104>, Adt120), Adt91>(Adt91::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f356(v0, v1, match v9 {
                                Adt5::C0(h857) => {
                                    h857
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

pub fn f284(v0: Adt6) -> Option<bool> {
    let m858 = v0;
    match m858 {
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

pub fn f285(v0: Adt6) -> Option<Adt5> {
    let m859 = v0;
    match m859 {
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

pub fn f286(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    let m860 = f357(v1.clone(), v2.clone())?;
    match m860 {
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
            f358(v0, v1, v2, v3)
        }
    }
}

pub fn f287(v0: List<Adt6>, v1: Adt6, v2: List<Adt99>, v3: u64, v4: List<Adt101>) -> Adt105 {
    let m861 = v0;
    match m861.uncons() {
        None => {
            let a862 = v3;
            let a863 = 1u64;
            if nat_eq(a862, a863) {
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

pub fn f288(v0: Option<Adt6>, v1: Adt6, v2: List<Adt99>, v3: List<Adt101>) -> Adt105 {
    let m864 = v0;
    match m864 {
        None => {
            Adt105::C0(Adt100::C0, v3)
        }
        Some(v4) => {
            Adt105::C0(Adt100::C0, List::cons(Adt101::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f289(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m865 = f334(Str::lit("anyOf"), v0);
    match m865 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m866 = f359(v3);
            match m866 {
                None => {
                    Adt121::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m867 = v4;
                    match m867.uncons() {
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

pub fn f290(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m868 = f334(Str::lit("oneOf"), v0);
    match m868 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m869 = f359(v3);
            match m869 {
                None => {
                    Adt121::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m870 = v4;
                    match m870.uncons() {
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

pub fn f291(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m871 = f334(Str::lit("not"), v0);
    match m871 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            Adt121::C3(List::cons(Adt101::C0(v3, v1, v2.clone()), List::cons(Adt101::C3(v2), List::<Adt101>::nil())))
        }
    }
}

pub fn f292(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m872 = f334(Str::lit("if"), v0.clone());
    match m872 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            Adt121::C3(List::cons(Adt101::C0(v3, v1.clone(), v2.clone()), List::cons(Adt101::C4(f360(Str::lit("then"), v0.clone()), f360(Str::lit("else"), v0), v1, v2), List::<Adt101>::nil())))
        }
    }
}

pub fn f293(v0: List<Adt6>, v1: Adt6, v2: List<Adt99>, v3: List<Adt101>) -> Adt105 {
    let m873 = v0;
    match m873.uncons() {
        None => {
            Adt105::C0(Adt100::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt105::C0(Adt100::C0, List::cons(Adt101::C0(v4, v1.clone(), v2.clone()), List::cons(Adt101::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f294(v0: Str) -> R<bool> {
    let m875 = f361(f153({
        let a874 = v0;
        utf8_encode(a874)
    })?)?;
    match m875 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f295(v0: Adt37) -> bool {
    let m877 = match v0 {
        Adt37::C0(_, _, h876) => {
            h876
        }
    };
    match m877 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f296(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m879 = match v0.clone() {
        Adt49::C0(_, h878) => {
            h878
        }
    };
    match m879.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h880, _) => {
                    h880
                }
            })))
        }
        Some((_, _)) => {
            if f362(match v0.clone() {
                Adt49::C0(_, h881) => {
                    h881
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f297(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m885 = {
        let m884 = match v1.clone() {
            Adt49::C0(h883, _) => {
                h883
            }
        };
        match m884 {
            Adt37::C0(_, h882, _) => {
                h882
            }
        }
    };
    match m885 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m888 = f121(Adt6::C5(match v0.clone() {
                Adt5::C0(h886) => {
                    h886
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h887) => {
                    h887
                }
            }), v2)?;
            match m888 {
                Adt98::C0 => {
                    f296(v1)
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

pub fn f298(v0: Adt99) -> R<Str> {
    let m889 = v0;
    match m889 {
        Adt99::C0(v1) => {
            f363(v1)
        }
        Adt99::C1(v2) => {
            let a890 = f186(v2, 0i64)?;
            Ok(format_i64(a890))
        }
    }
}

pub fn f299(p896: List<u64>, p897: List<u64>) -> Option<List<u64>> {
    let m895 = (p896, p897);
    {
        let mut m895 = m895;
        loop {
            let step = {
                let (v0, v1) = m895;
                let m891 = v0;
                match m891.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m892 = v1;
                        match m892.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a893 = v2;
                                let a894 = v4;
                                if nat_eq(a893, a894) {
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
                Ok(next) => { m895 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Str) -> R<List<u64>> {
    f153({
        let a898 = v0;
        utf8_encode(a898)
    })
}

pub fn f301() -> Str {
    Str::lit("test://template/")
}

pub fn f302(v0: List<u64>) -> List<u64> {
    f253(v0, List::<u64>::nil())
}

pub fn f303() -> Str {
    Str::lit("data")
}

pub fn f304(v0: List<u64>) -> bool {
    let m899 = v0;
    match m899.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f305(p904: List<u64>) -> bool {
    let m903 = p904;
    {
        let mut m903 = m903;
        loop {
            let step = {
                let v0 = m903;
                let m900 = v0;
                match m900.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a901 = v1;
                        let a902 = 47u64;
                        if nat_eq(a901, a902) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
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

pub fn f306(p909: Bytes, p910: List<Bytes>) -> Bytes {
    let m908 = (p909, p910);
    {
        let mut m908 = m908;
        loop {
            let step = {
                let (v0, v1) = m908;
                let m905 = v1;
                match m905.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a906 = v2;
                            let a907 = v0;
                            append_bytes(a906, a907)
                        }, v3))
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

pub fn f307(p913: List<Bytes>, p914: List<u64>) -> List<Bytes> {
    let m912 = (p913, p914);
    {
        let mut m912 = m912;
        loop {
            let step = {
                let (v0, v1) = m912;
                let m911 = v1;
                match m911.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f364(f365(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f308(p920: List<u64>, p921: List<u64>) -> bool {
    let m919 = (p920, p921);
    {
        let mut m919 = m919;
        loop {
            let step = {
                let (v0, v1) = m919;
                let m915 = v0;
                match m915.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m916 = v1;
                        match m916.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a917 = v2;
                                let a918 = v4;
                                if nat_eq(a917, a918) {
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
                Ok(next) => { m919 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(v0: Adt106, v1: List<Adt106>, v2: List<u64>) -> R<Adt132> {
    let m922 = v0;
    match m922 {
        Adt106::C0(v3) => {
            Ok(Adt132::C1(v1, f253(v3, v2)))
        }
        Adt106::C1(v4, v5) => {
            f366(v4, v5, v1, v2)
        }
        Adt106::C2(v6, v7) => {
            let m923 = v7;
            match m923.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C1(v6, v8), List::cons(Adt106::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt106::C3(v10, v11) => {
            let m924 = v11.clone();
            match m924.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt106::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt106::C4(v14, v15) => {
            let m925 = v15;
            match m925.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C0({
                        let a927 = f367({
                            let (h926, _) = v16;
                            h926
                        })?;
                        let a928 = List::cons(58u64, List::<u64>::nil());
                        append_list(a927, a928)
                    }), List::cons(Adt106::C1(v14, {
                        let (_, h929) = v16;
                        h929
                    }), List::cons(Adt106::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt106::C5(v18, v19) => {
            let m930 = v19.clone();
            match m930.uncons() {
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

pub fn f310(v0: Adt123) -> R<Option<Adt5>> {
    let m932 = f368(match v0.clone() {
        Adt123::C0(h931, _, _) => {
            h931
        }
    })?;
    match m932 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a935 = v1;
                let a936 = {
                    let m934 = f369(v0);
                    match m934 {
                        Adt5::C0(h933) => {
                            h933
                        }
                    }
                };
                append_list(a935, a936)
            })))
        }
    }
}

pub fn f311(v0: bool, v1: Adt88, v2: Option<List<List<u64>>>) -> Adt88 {
    let m937 = v2;
    match m937 {
        None => {
            f6()
        }
        Some(v3) => {
            f370(v0, v1, f52(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f312() -> u64 {
    200u64
}

pub fn f313(v0: Adt92, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f207())), List::cons((Str::lit("id"), f254(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h938) => {
            h938
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f314(p942: List<Adt125>, p943: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m941 = (p942, p943);
    {
        let mut m941 = m941;
        loop {
            let step = {
                let (v0, v1) = m941;
                let m939 = v0;
                match m939.uncons() {
                    None => {
                        Err::<(List<Adt125>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m940 = f371(v2)?;
                        match m940 {
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
                Ok(next) => { m941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: Adt126) -> u64 {
    let m944 = v0;
    match m944 {
        Adt126::C0(_) => {
            f162()
        }
        Adt126::C1(_, _) => {
            f162()
        }
        Adt126::C2(_) => {
            f162()
        }
        Adt126::C3(_) => {
            f372()
        }
        Adt126::C4(v6) => {
            f373(v6)
        }
    }
}

pub fn f316(v0: Adt126) -> R<Adt80> {
    Ok(Adt80::C0(f374(v0.clone()), f375(v0.clone()), f376(v0)?))
}

pub fn f317() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f318(p947: List<List<u64>>) -> R<bool> {
    let m946 = p947;
    {
        let mut m946 = m946;
        loop {
            let step = {
                let v0 = m946;
                let m945 = v0;
                match m945.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f377(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m946 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: u64, p952: List<u64>, p953: List<u64>, p954: List<List<u64>>) -> List<List<u64>> {
    let m951 = (p952, (p953, p954));
    {
        let mut m951 = m951;
        loop {
            let step = {
                let (v1, (v2, v3)) = m951;
                let m948 = v1;
                match m948.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f52(List::cons(f230(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a949 = v4;
                        let a950 = v0;
                        if nat_eq(a949, a950) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f230(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m951 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f320(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt133, Adt126>) -> R<Result<Adt123, Adt126>> {
    let m955 = v3;
    match m955 {
        Ok(v5) => {
            f378(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt123, Adt126>(v4))
        }
    }
}

pub fn f321(v0: Option<Adt107>, v1: List<Str>, v2: Adt103) -> R<Result<Adt133, Adt126>> {
    Ok(f379(f380(match v2.clone() {
        Adt103::C0(_, h956, _) => {
            h956
        }
    }), f381(v0, v1, v2)?))
}

pub fn f322(v0: Adt109, v1: Adt96) -> Adt128 {
    let m957 = v1;
    match m957 {
        Adt96::C0 => {
            f328()
        }
        Adt96::C1 => {
            Adt128::C0(None::<Str>, Some(Adt108::C6(v0)))
        }
        Adt96::C2 => {
            Adt128::C0(None::<Str>, Some(Adt108::C7(v0)))
        }
        Adt96::C3(v2) => {
            let m958 = f382(v2);
            match m958 {
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

pub fn f323() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f324() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f325(v0: Str) -> bool {
    let m959 = f383(v0);
    match m959 {
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

pub fn f326(v0: Adt96) -> R<Adt96> {
    let m960 = v0.clone();
    match m960 {
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
            let m961 = f384(v1);
            match m961 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m962 = f361(v2)?;
                    match m962 {
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

pub fn f327() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f328() -> Adt128 {
    Adt128::C0(None::<Str>, None::<Adt108>)
}

pub fn f329(v0: Option<Adt108>, v1: Option<Adt108>) -> Option<Adt108> {
    let m963 = v0.clone();
    match m963 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f330(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt110>> {
    Ok(f385(f386(v0, v1, v2, List::<Adt110>::nil())?, List::<Adt110>::nil()))
}

pub fn f331(v0: Adt6) -> Option<Str> {
    let m964 = v0;
    match m964 {
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

pub fn f332() -> Str {
    Str::lit("2026-07-28")
}

pub fn f333(v0: Adt7) -> R<Option<i64>> {
    let m967 = f387(40u64, match v0.clone() {
        Adt7::C0(h965, _) => {
            h965
        }
    }, match v0 {
        Adt7::C0(_, h966) => {
            h966
        }
    })?;
    match m967 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f388(v1))
        }
    }
}

pub fn f334(v0: Str, p973: List<(Str, Adt6)>) -> Adt130 {
    let m972 = p973;
    {
        let mut m972 = m972;
        loop {
            let step = {
                let v1 = m972;
                let m968 = v1;
                match m968.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt130>(Adt130::C0)
                    }
                    Some((v2, v3)) => {
                        let m970 = f389({
                            let (h969, _) = v2;
                            h969
                        }, v0.clone());
                        match m970 {
                            Adt141::C0 => {
                                Err::<List<(Str, Adt6)>, Adt130>(Adt130::C1({
                                    let (_, h971) = v2;
                                    h971
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
                Ok(next) => { m972 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Str, v1: Adt5) -> bool {
    f390(f266(v0, v1))
}

pub fn f336(v0: Adt92, v1: Adt5) -> Result<Adt103, Adt94> {
    let m974 = f266(Str::lit("method"), v1.clone());
    match m974 {
        Adt130::C0 => {
            f216(Some(v0), f267(v1))
        }
        Adt130::C1(v2) => {
            let m975 = f331(v2);
            match m975 {
                None => {
                    f216(Some(v0), Adt95::C4)
                }
                Some(v3) => {
                    f391(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f337(v0: u64) -> R<u64> {
    let a978 = {
        let a976 = v0;
        let a977 = 3u64;
        nat_mul(a976, a977)?
    };
    let a979 = 1u64;
    nat_add(a978, a979)
}

pub fn f338(v0: u64, v1: u64, v2: u64) -> bool {
    let a980 = v1;
    let a981 = v0;
    if nat_le(a980, a981) {
        let a982 = v0;
        let a983 = v2;
        nat_le(a982, a983)
    } else {
        false
    }
}

pub fn f339(v0: Adt117) -> R<Adt135> {
    if f392(match v0.clone() {
        Adt117::C0(_, h984, _, _, _, _, _) => {
            h984
        }
    }) {
        let v1: i64 = {
            let a990 = if match v0.clone() {
                Adt117::C0(_, _, _, _, _, h985, _) => {
                    h985
                }
            } {
                let a987 = match v0.clone() {
                    Adt117::C0(_, _, _, _, _, _, h986) => {
                        h986
                    }
                };
                int_neg(a987)?
            } else {
                match v0.clone() {
                    Adt117::C0(_, _, _, _, _, _, h988) => {
                        h988
                    }
                }
            };
            let a991 = match v0.clone() {
                Adt117::C0(_, _, _, _, h989, _, _) => {
                    h989
                }
            };
            int_sub(a990, a991)?
        };
        let m994 = {
            let a992 = v1;
            let a993 = -9999i64;
            compare(a992, a993)
        };
        if if match m994 {
            Ordering::Less => {
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
            let m997 = {
                let a995 = 9999i64;
                let a996 = v1;
                compare(a995, a996)
            };
            match m997 {
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
                Adt117::C0(h998, _, _, _, _, _, _) => {
                    h998
                }
            } {
                let a1000 = match v0 {
                    Adt117::C0(_, _, h999, _, _, _, _) => {
                        h999
                    }
                };
                int_neg(a1000)?
            } else {
                match v0 {
                    Adt117::C0(_, _, h1001, _, _, _, _) => {
                        h1001
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt135::C0)
    }
}

pub fn f340(v0: Adt6, v1: List<Adt119>, v2: u64) -> Adt114 {
    let m1002 = v1.clone();
    match m1002.uncons() {
        None => {
            Adt114::C0(Adt115::C9(v0), List::<Adt119>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1003 = v3;
            match m1003 {
                Adt119::C0(v5) => {
                    Adt114::C0(Adt115::C2, List::cons(Adt119::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt119::C1(v6, v7) => {
                    let m1004 = v7;
                    match m1004 {
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

pub fn f341(v0: u64) -> bool {
    let a1005 = v0;
    let a1006 = 32u64;
    if nat_eq(a1005, a1006) {
        true
    } else {
        let a1007 = v0;
        let a1008 = 10u64;
        if nat_eq(a1007, a1008) {
            true
        } else {
            let a1009 = v0;
            let a1010 = 13u64;
            if nat_eq(a1009, a1010) {
                true
            } else {
                let a1011 = v0;
                let a1012 = 9u64;
                nat_eq(a1011, a1012)
            }
        }
    }
}

pub fn f342(v0: Adt114, v1: u64, v2: u64) -> R<Adt131> {
    let a1013 = v2;
    let a1014 = 34u64;
    if nat_eq(a1013, a1014) {
        Ok(f346(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), false)))
    } else {
        let a1015 = v2;
        let a1016 = 123u64;
        if nat_eq(a1015, a1016) {
            f393(v0, Adt119::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt115::C3, v1)
        } else {
            let a1017 = v2;
            let a1018 = 91u64;
            if nat_eq(a1017, a1018) {
                f393(v0, Adt119::C0(List::<Adt6>::nil()), Adt115::C1, v1)
            } else {
                let a1019 = v2;
                let a1020 = 116u64;
                if nat_eq(a1019, a1020) {
                    Ok(f346(v0, Adt115::C7(0u64, 1u64)))
                } else {
                    let a1021 = v2;
                    let a1022 = 102u64;
                    if nat_eq(a1021, a1022) {
                        Ok(f346(v0, Adt115::C7(1u64, 1u64)))
                    } else {
                        let a1023 = v2;
                        let a1024 = 110u64;
                        if nat_eq(a1023, a1024) {
                            Ok(f346(v0, Adt115::C7(2u64, 1u64)))
                        } else {
                            let a1025 = v2;
                            let a1026 = 45u64;
                            if nat_eq(a1025, a1026) {
                                Ok(f346(v0, f394(true, Adt118::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1027 = v2;
                                let a1028 = 48u64;
                                if nat_eq(a1027, a1028) {
                                    Ok(f346(v0, f394(false, Adt118::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f338(v2, 49u64, 57u64) {
                                        Ok(f346(v0, f394(false, Adt118::C2, f395({
                                            let a1029 = v2;
                                            let a1030 = 48u64;
                                            nat_sub(a1029, a1030)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f347(Adt93::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f343(v0: Adt114, v1: u64) -> Adt131 {
    let m1032 = match v0.clone() {
        Adt114::C0(_, h1031, _) => {
            h1031
        }
    };
    match m1032.uncons() {
        None => {
            f347(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m1033 = v2;
            match m1033 {
                Adt119::C0(v4) => {
                    Adt131::C1(f340(Adt6::C4(f396(v4, List::<Adt6>::nil())), v3, {
                        let a1035 = match v0 {
                            Adt114::C0(_, _, h1034) => {
                                h1034
                            }
                        };
                        let a1036 = 1u64;
                        nat_sub(a1035, a1036)
                    }))
                }
                Adt119::C1(_, _) => {
                    f347(Adt93::C2(v1))
                }
            }
        }
    }
}

pub fn f344(v0: Adt114, v1: u64, v2: u64) -> Adt131 {
    let m1038 = match v0.clone() {
        Adt114::C0(h1037, _, _) => {
            h1037
        }
    };
    match m1038 {
        Adt115::C0 => {
            f347(Adt93::C2(v1))
        }
        Adt115::C1 => {
            f347(Adt93::C2(v1))
        }
        Adt115::C2 => {
            if f341(v2) {
                Adt131::C1(v0)
            } else {
                let a1039 = v2;
                let a1040 = 44u64;
                if nat_eq(a1039, a1040) {
                    f397(v0, v1)
                } else {
                    let a1041 = v2;
                    let a1042 = 93u64;
                    if nat_eq(a1041, a1042) {
                        f343(v0, v1)
                    } else {
                        let a1043 = v2;
                        let a1044 = 125u64;
                        if nat_eq(a1043, a1044) {
                            f345(v0, v1)
                        } else {
                            f347(Adt93::C2(v1))
                        }
                    }
                }
            }
        }
        Adt115::C3 => {
            f347(Adt93::C2(v1))
        }
        Adt115::C4 => {
            f347(Adt93::C2(v1))
        }
        Adt115::C5 => {
            f347(Adt93::C2(v1))
        }
        Adt115::C6(_, _, _) => {
            f347(Adt93::C2(v1))
        }
        Adt115::C7(_, _) => {
            f347(Adt93::C2(v1))
        }
        Adt115::C8(_) => {
            f347(Adt93::C2(v1))
        }
        Adt115::C9(_) => {
            if f341(v2) {
                Adt131::C1(v0)
            } else {
                f347(Adt93::C10(v1))
            }
        }
    }
}

pub fn f345(v0: Adt114, v1: u64) -> Adt131 {
    let m1046 = match v0.clone() {
        Adt114::C0(_, h1045, _) => {
            h1045
        }
    };
    match m1046.uncons() {
        None => {
            f347(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m1047 = v2;
            match m1047 {
                Adt119::C0(_) => {
                    f347(Adt93::C2(v1))
                }
                Adt119::C1(v5, _) => {
                    Adt131::C1(f340(Adt6::C5(f398(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1049 = match v0 {
                            Adt114::C0(_, _, h1048) => {
                                h1048
                            }
                        };
                        let a1050 = 1u64;
                        nat_sub(a1049, a1050)
                    }))
                }
            }
        }
    }
}

pub fn f346(v0: Adt114, v1: Adt115) -> Adt131 {
    Adt131::C1(f399(v0, v1))
}

pub fn f347(v0: Adt93) -> Adt131 {
    Adt131::C0(v0)
}

pub fn f348(v0: Adt114, v1: u64, v2: u64, v3: Adt116, v4: List<u64>, v5: bool) -> R<Adt131> {
    let m1051 = v3;
    match m1051 {
        Adt116::C0 => {
            let a1052 = v2;
            let a1053 = 34u64;
            if nat_eq(a1052, a1053) {
                Ok(f400(v0, v1, v4, v5))
            } else {
                let a1054 = v2;
                let a1055 = 92u64;
                if nat_eq(a1054, a1055) {
                    Ok(f346(v0, Adt115::C6(Adt116::C1, v4, v5)))
                } else {
                    let a1056 = v2;
                    let a1057 = 32u64;
                    if nat_lt(a1056, a1057) {
                        Ok(f347(Adt93::C4(v1)))
                    } else {
                        Ok(f346(v0, Adt115::C6(Adt116::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt116::C1 => {
            let a1058 = v2;
            let a1059 = 117u64;
            if nat_eq(a1058, a1059) {
                Ok(f346(v0, Adt115::C6(Adt116::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1060 = f401(v2);
                let a1061 = 0u64;
                if nat_eq(a1060, a1061) {
                    Ok(f347(Adt93::C5(v1)))
                } else {
                    Ok(f346(v0, Adt115::C6(Adt116::C0, List::cons(f401(v2), v4), v5)))
                }
            }
        }
        Adt116::C2(v6, v7, v8) => {
            let a1062 = 15u64;
            let a1063 = f402(v2);
            if nat_lt(a1062, a1063) {
                Ok(f347(Adt93::C5(v1)))
            } else {
                let a1066 = {
                    let a1064 = v6;
                    let a1065 = 1u64;
                    nat_add(a1064, a1065)?
                };
                let a1067 = 4u64;
                if nat_eq(a1066, a1067) {
                    f403(v0, v1, {
                        let a1070 = {
                            let a1068 = v7;
                            let a1069 = 16u64;
                            nat_mul(a1068, a1069)?
                        };
                        let a1071 = f402(v2);
                        nat_add(a1070, a1071)?
                    }, v8, v4, v5)
                } else {
                    Ok(f346(v0, Adt115::C6(Adt116::C2({
                        let a1072 = v6;
                        let a1073 = 1u64;
                        nat_add(a1072, a1073)?
                    }, {
                        let a1076 = {
                            let a1074 = v7;
                            let a1075 = 16u64;
                            nat_mul(a1074, a1075)?
                        };
                        let a1077 = f402(v2);
                        nat_add(a1076, a1077)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt116::C3(v9) => {
            let a1078 = v2;
            let a1079 = 92u64;
            if nat_eq(a1078, a1079) {
                Ok(f346(v0, Adt115::C6(Adt116::C4(v9), v4, v5)))
            } else {
                Ok(f347(Adt93::C6(v1)))
            }
        }
        Adt116::C4(v10) => {
            let a1080 = v2;
            let a1081 = 117u64;
            if nat_eq(a1080, a1081) {
                Ok(f346(v0, Adt115::C6(Adt116::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f347(Adt93::C6(v1)))
            }
        }
    }
}

pub fn f349(v0: Adt114, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt131> {
    let a1082 = v2;
    let a1083 = f404(v3, v4);
    if nat_eq(a1082, a1083) {
        let a1087 = {
            let a1084 = v4;
            let a1085 = 1u64;
            nat_add(a1084, a1085)?
        };
        let a1088 = {
            let a1086 = f405(v3);
            length_list(a1086)
        };
        if nat_eq(a1087, a1088) {
            Ok(f406(v0, f407(v3)))
        } else {
            Ok(f346(v0, Adt115::C7(v3, {
                let a1089 = v4;
                let a1090 = 1u64;
                nat_add(a1089, a1090)?
            })))
        }
    } else {
        Ok(f347(Adt93::C2(v1)))
    }
}

pub fn f350(v0: Adt114, v1: u64, v2: u64, v3: Adt117) -> R<Adt131> {
    let m1092 = match v3.clone() {
        Adt117::C0(_, h1091, _, _, _, _, _) => {
            h1091
        }
    };
    match m1092 {
        Adt118::C0 => {
            let a1093 = v2;
            let a1094 = 48u64;
            if nat_eq(a1093, a1094) {
                Ok(f408(v0, v3, Adt118::C1))
            } else {
                if f338(v2, 49u64, 57u64) {
                    f409(v0, v1, v2, v3, false)
                } else {
                    Ok(f347(Adt93::C7(v1)))
                }
            }
        }
        Adt118::C1 => {
            let a1095 = v2;
            let a1096 = 46u64;
            if nat_eq(a1095, a1096) {
                Ok(f408(v0, v3, Adt118::C3))
            } else {
                if f410(v2) {
                    Ok(f408(v0, v3, Adt118::C5))
                } else {
                    if f411(v2) {
                        Ok(f347(Adt93::C7(v1)))
                    } else {
                        f412(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt118::C2 => {
            if f411(v2) {
                f409(v0, v1, v2, v3, false)
            } else {
                let a1097 = v2;
                let a1098 = 46u64;
                if nat_eq(a1097, a1098) {
                    Ok(f408(v0, v3, Adt118::C3))
                } else {
                    if f410(v2) {
                        Ok(f408(v0, v3, Adt118::C5))
                    } else {
                        f412(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt118::C3 => {
            if f411(v2) {
                f409(v0, v1, v2, v3, true)
            } else {
                Ok(f347(Adt93::C7(v1)))
            }
        }
        Adt118::C4 => {
            if f411(v2) {
                f409(v0, v1, v2, v3, true)
            } else {
                if f410(v2) {
                    Ok(f408(v0, v3, Adt118::C5))
                } else {
                    f412(v0, v1, v2, v3)
                }
            }
        }
        Adt118::C5 => {
            let a1099 = v2;
            let a1100 = 43u64;
            if nat_eq(a1099, a1100) {
                Ok(f346(v0, f394(match v3.clone() {
                    Adt117::C0(h1101, _, _, _, _, _, _) => {
                        h1101
                    }
                }, Adt118::C6, match v3.clone() {
                    Adt117::C0(_, _, h1102, _, _, _, _) => {
                        h1102
                    }
                }, match v3.clone() {
                    Adt117::C0(_, _, _, h1103, _, _, _) => {
                        h1103
                    }
                }, match v3 {
                    Adt117::C0(_, _, _, _, h1104, _, _) => {
                        h1104
                    }
                }, false, 0i64)))
            } else {
                let a1105 = v2;
                let a1106 = 45u64;
                if nat_eq(a1105, a1106) {
                    Ok(f346(v0, f394(match v3.clone() {
                        Adt117::C0(h1107, _, _, _, _, _, _) => {
                            h1107
                        }
                    }, Adt118::C6, match v3.clone() {
                        Adt117::C0(_, _, h1108, _, _, _, _) => {
                            h1108
                        }
                    }, match v3.clone() {
                        Adt117::C0(_, _, _, h1109, _, _, _) => {
                            h1109
                        }
                    }, match v3 {
                        Adt117::C0(_, _, _, _, h1110, _, _) => {
                            h1110
                        }
                    }, true, 0i64)))
                } else {
                    if f411(v2) {
                        f413(v0, v1, v2, Adt117::C0(match v3.clone() {
                            Adt117::C0(h1111, _, _, _, _, _, _) => {
                                h1111
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, h1112, _, _, _, _, _) => {
                                h1112
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, _, h1113, _, _, _, _) => {
                                h1113
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, _, _, h1114, _, _, _) => {
                                h1114
                            }
                        }, match v3 {
                            Adt117::C0(_, _, _, _, h1115, _, _) => {
                                h1115
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f347(Adt93::C7(v1)))
                    }
                }
            }
        }
        Adt118::C6 => {
            if f411(v2) {
                f413(v0, v1, v2, v3.clone(), match v3 {
                    Adt117::C0(_, _, _, _, _, h1116, _) => {
                        h1116
                    }
                })
            } else {
                Ok(f347(Adt93::C7(v1)))
            }
        }
        Adt118::C7 => {
            if f411(v2) {
                f413(v0, v1, v2, v3.clone(), match v3 {
                    Adt117::C0(_, _, _, _, _, h1117, _) => {
                        h1117
                    }
                })
            } else {
                f412(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f351(v0: u64) -> bool {
    let a1118 = v0;
    let a1119 = 9u64;
    if nat_eq(a1118, a1119) {
        true
    } else {
        let a1120 = v0;
        let a1121 = 32u64;
        nat_eq(a1120, a1121)
    }
}

pub fn f352(v0: u64) -> bool {
    let a1122 = 47u64;
    let a1123 = v0;
    if nat_lt(a1122, a1123) {
        let a1124 = v0;
        let a1125 = 58u64;
        nat_lt(a1124, a1125)
    } else {
        false
    }
}

pub fn f353(v0: u64) -> bool {
    let a1126 = 48u64;
    let a1127 = v0;
    if if nat_le(a1126, a1127) {
        let a1128 = v0;
        let a1129 = 57u64;
        nat_le(a1128, a1129)
    } else {
        false
    } {
        true
    } else {
        let a1130 = 65u64;
        let a1131 = v0;
        if if nat_le(a1130, a1131) {
            let a1132 = v0;
            let a1133 = 90u64;
            nat_le(a1132, a1133)
        } else {
            false
        } {
            true
        } else {
            let a1134 = 97u64;
            let a1135 = v0;
            if if nat_le(a1134, a1135) {
                let a1136 = v0;
                let a1137 = 122u64;
                nat_le(a1136, a1137)
            } else {
                false
            } {
                true
            } else {
                let a1138 = v0;
                let a1139 = 33u64;
                if nat_eq(a1138, a1139) {
                    true
                } else {
                    let a1140 = v0;
                    let a1141 = 35u64;
                    if nat_eq(a1140, a1141) {
                        true
                    } else {
                        let a1142 = v0;
                        let a1143 = 36u64;
                        if nat_eq(a1142, a1143) {
                            true
                        } else {
                            let a1144 = v0;
                            let a1145 = 37u64;
                            if nat_eq(a1144, a1145) {
                                true
                            } else {
                                let a1146 = v0;
                                let a1147 = 38u64;
                                if nat_eq(a1146, a1147) {
                                    true
                                } else {
                                    let a1148 = v0;
                                    let a1149 = 39u64;
                                    if nat_eq(a1148, a1149) {
                                        true
                                    } else {
                                        let a1150 = v0;
                                        let a1151 = 42u64;
                                        if nat_eq(a1150, a1151) {
                                            true
                                        } else {
                                            let a1152 = v0;
                                            let a1153 = 43u64;
                                            if nat_eq(a1152, a1153) {
                                                true
                                            } else {
                                                let a1154 = v0;
                                                let a1155 = 45u64;
                                                if nat_eq(a1154, a1155) {
                                                    true
                                                } else {
                                                    let a1156 = v0;
                                                    let a1157 = 46u64;
                                                    if nat_eq(a1156, a1157) {
                                                        true
                                                    } else {
                                                        let a1158 = v0;
                                                        let a1159 = 94u64;
                                                        if nat_eq(a1158, a1159) {
                                                            true
                                                        } else {
                                                            let a1160 = v0;
                                                            let a1161 = 95u64;
                                                            if nat_eq(a1160, a1161) {
                                                                true
                                                            } else {
                                                                let a1162 = v0;
                                                                let a1163 = 96u64;
                                                                if nat_eq(a1162, a1163) {
                                                                    true
                                                                } else {
                                                                    let a1164 = v0;
                                                                    let a1165 = 124u64;
                                                                    if nat_eq(a1164, a1165) {
                                                                        true
                                                                    } else {
                                                                        let a1166 = v0;
                                                                        let a1167 = 126u64;
                                                                        nat_eq(a1166, a1167)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f354(v0: u64) -> bool {
    let a1168 = 48u64;
    let a1169 = v0;
    if if nat_le(a1168, a1169) {
        let a1170 = v0;
        let a1171 = 57u64;
        nat_le(a1170, a1171)
    } else {
        false
    } {
        true
    } else {
        let a1172 = 65u64;
        let a1173 = v0;
        if if nat_le(a1172, a1173) {
            let a1174 = v0;
            let a1175 = 90u64;
            nat_le(a1174, a1175)
        } else {
            false
        } {
            true
        } else {
            let a1176 = 97u64;
            let a1177 = v0;
            if if nat_le(a1176, a1177) {
                let a1178 = v0;
                let a1179 = 122u64;
                nat_le(a1178, a1179)
            } else {
                false
            } {
                true
            } else {
                let a1180 = v0;
                let a1181 = 95u64;
                if nat_eq(a1180, a1181) {
                    true
                } else {
                    let a1182 = v0;
                    let a1183 = 45u64;
                    if nat_eq(a1182, a1183) {
                        true
                    } else {
                        let a1184 = v0;
                        let a1185 = 46u64;
                        nat_eq(a1184, a1185)
                    }
                }
            }
        }
    }
}

pub fn f355() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f356(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt120, v6: List<Adt104>) -> R<Result<(List<Adt104>, Adt120), Adt91>> {
    let m1186 = f414(v0.clone(), v1, v2.clone());
    match m1186 {
        None => {
            let m1187 = f415(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1187 {
                Adt142::C0(v8) => {
                    Ok(Err::<(List<Adt104>, Adt120), Adt91>(v8))
                }
                Adt142::C1(v9) => {
                    Ok(Ok::<(List<Adt104>, Adt120), Adt91>((f416(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt104>, Adt120), Adt91>(v7))
        }
    }
}

pub fn f357(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt121> {
    Ok(f417(List::cons((Str::lit("type"), f418(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f419(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f420(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f421(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f421(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f421(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f421(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1188 = f422(v0.clone(), v1.clone())?;
        let a1189 = List::cons((Str::lit("required"), f423(v0, v1)), List::<(Str, Adt136)>::nil());
        append_list(a1188, a1189)
    })))))))))
}

pub fn f358(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    Ok(f424(f425(v0, v1.clone(), v2.clone(), v3.clone()), f424(f426(v1.clone(), v2.clone(), v3.clone()), f424(f427(v1.clone(), v2.clone(), v3.clone()), f424(f428(v1.clone(), v2.clone(), v3.clone())?, f424(f429(v1.clone(), v2.clone(), v3.clone()), f424(f430(Adt102::C0, f289(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f424(f430(Adt102::C1, f290(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f424(f430(Adt102::C2, f291(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f430(Adt102::C3, f292(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f359(v0: Adt6) -> Option<List<Adt6>> {
    let m1190 = v0;
    match m1190 {
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

pub fn f360(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1191 = f334(v0, v1);
    match m1191 {
        Adt130::C0 => {
            None::<Adt6>
        }
        Adt130::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f361(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1193 = f431({
        let a1192 = v0.clone();
        length_list(a1192)
    }, v0, List::<u64>::nil())?;
    match m1193 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f253(v1, List::<u64>::nil())))
        }
    }
}

pub fn f362(v0: List<Adt50>) -> R<bool> {
    let m1194 = v0;
    match m1194.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f432(v2, match v1 {
                Adt50::C0(h1195, _, _) => {
                    h1195
                }
            })
        }
    }
}

pub fn f363(v0: Str) -> R<Str> {
    let m1198 = {
        let a1197 = f246(f247(), f253(f433(f153({
            let a1196 = v0.clone();
            utf8_encode(a1196)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1197)
    };
    match m1198 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f364(p1207: Bytes, p1208: List<Bytes>) -> List<Bytes> {
    let m1206 = (p1207, p1208);
    {
        let mut m1206 = m1206;
        loop {
            let step = {
                let (v0, v1) = m1206;
                let m1199 = v1;
                match m1199.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1202 = {
                            let a1200 = v2.clone();
                            length_bytes(a1200)
                        };
                        let a1203 = {
                            let a1201 = v0.clone();
                            length_bytes(a1201)
                        };
                        if nat_eq(a1202, a1203) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1204 = v2;
                                let a1205 = v0;
                                append_bytes(a1204, a1205)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
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

pub fn f365(v0: u64) -> Bytes {
    let m1212 = {
        let a1209 = f434();
        let a1210 = v0;
        let a1211 = 1u64;
        slice_bytes(a1209, a1210, a1211)
    };
    match m1212 {
        None => {
            f247()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f366(v0: u64, v1: Adt6, v2: List<Adt106>, v3: List<u64>) -> R<Adt132> {
    let m1213 = v1;
    match m1213 {
        Adt6::C0 => {
            Ok(Adt132::C1(v2, f253(f435(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt132::C1(v2, f253(if v4 {
                f435(0u64)
            } else {
                f435(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f436(v5.clone())? {
                Ok(Adt132::C1(v2, f253(f437(v5)?, v3)))
            } else {
                Ok(Adt132::C0(Adt93::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt132::C1(v2, f253(f367(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1214 = v0;
            let a1215 = f438();
            if nat_lt(a1214, a1215) {
                Ok(Adt132::C1(f439(91u64, Adt106::C2({
                    let a1216 = v0;
                    let a1217 = 1u64;
                    nat_add(a1216, a1217)?
                }, v7), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1218 = v0;
            let a1219 = f438();
            if nat_lt(a1218, a1219) {
                Ok(Adt132::C1(f439(123u64, Adt106::C4({
                    let a1220 = v0;
                    let a1221 = 1u64;
                    nat_add(a1220, a1221)?
                }, v8), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
    }
}

pub fn f367(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1223 = f253(f440(f153({
            let a1222 = v0;
            utf8_encode(a1222)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1224 = List::cons(34u64, List::<u64>::nil());
        append_list(a1223, a1224)
    }))
}

pub fn f368(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m1225 = v0;
    match m1225 {
        Adt32::C0(v1) => {
            Ok(Some(f441(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f442(v2)?))
        }
        Adt32::C2(v3) => {
            f443(f444(v3)?)
        }
        Adt32::C3(v4) => {
            f443(f444(match v4 {
                Adt49::C0(h1226, _) => {
                    h1226
                }
            })?)
        }
        Adt32::C4(v5) => {
            Ok(Some(f445(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f446(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f447(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f448(v8)?))
        }
        Adt32::C8(v9) => {
            f443(f449(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f450(v10)?))
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

pub fn f369(v0: Adt123) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f451(f452(match v0.clone() {
        Adt123::C0(h1227, _, _) => {
            h1227
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f453({
        let m1230 = match v0 {
            Adt123::C0(_, h1229, _) => {
                h1229
            }
        };
        match m1230 {
            Adt124::C0(h1228, _) => {
                h1228
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f370(v0: bool, v1: Adt88, v2: List<List<u64>>) -> Adt88 {
    if f454(v0, match v1.clone() {
        Adt88::C0(h1231, _, _, _) => {
            h1231
        }
    }, v2.clone()) {
        f455(match v1.clone() {
            Adt88::C0(h1232, _, _, _) => {
                h1232
            }
        }, {
            let a1234 = v2;
            let a1235 = List::cons(match v1 {
                Adt88::C0(_, _, _, h1233) => {
                    h1233
                }
            }, List::<List<u64>>::nil());
            append_list(a1234, a1235)
        })
    } else {
        v1
    }
}

pub fn f371(v0: Adt125) -> R<Option<List<u64>>> {
    let m1236 = f456(f457(v0))?;
    match m1236 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1237 = f155(v1)?;
            match m1237 {
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

pub fn f372() -> u64 {
    404u64
}

pub fn f373(v0: Adt84) -> u64 {
    let m1238 = v0;
    match m1238 {
        Adt84::C0(_) => {
            f162()
        }
        Adt84::C1(_) => {
            f162()
        }
        Adt84::C2(_) => {
            f162()
        }
        Adt84::C3(_) => {
            f162()
        }
        Adt84::C4(_) => {
            f162()
        }
        Adt84::C5(_) => {
            f162()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f458()
        }
        Adt84::C8(_) => {
            f162()
        }
        Adt84::C9(_) => {
            f21()
        }
    }
}

pub fn f374(v0: Adt126) -> i64 {
    let m1239 = v0;
    match m1239 {
        Adt126::C0(_) => {
            f459()
        }
        Adt126::C1(_, _) => {
            f460()
        }
        Adt126::C2(_) => {
            f461()
        }
        Adt126::C3(_) => {
            f462()
        }
        Adt126::C4(v6) => {
            f463(v6)
        }
    }
}

pub fn f375(v0: Adt126) -> Str {
    let m1240 = v0;
    match m1240 {
        Adt126::C0(v1) => {
            f464(v1)
        }
        Adt126::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt126::C2(v4) => {
            f465(v4)
        }
        Adt126::C3(_) => {
            Str::lit("Method not found")
        }
        Adt126::C4(v6) => {
            let m1241 = v6;
            match m1241 {
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

pub fn f376(v0: Adt126) -> R<Option<Adt6>> {
    let m1242 = v0;
    match m1242 {
        Adt126::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f466(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt126::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C4(v6) => {
            let m1243 = v6;
            match m1243 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f186(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f467(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f377(v0: List<u64>) -> R<bool> {
    let m1244 = f319(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1244.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f468(v2)? {
                Ok(false)
            } else {
                f220(f222(v1), f469())
            }
        }
    }
}

pub fn f378(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133) -> R<Result<Adt123, Adt126>> {
    f470(v0.clone(), v1, v2, v3.clone(), f471(match v0 {
        Adt0::C0(_, _, h1245, _, _, _) => {
            h1245
        }
    }, match v3 {
        Adt133::C0(_, h1246, _, _) => {
            h1246
        }
    }))
}

pub fn f379(v0: Adt137, v1: Adt138) -> Result<Adt133, Adt126> {
    let m1247 = v1;
    match m1247 {
        Adt138::C0(v3) => {
            f472(v3, v0)
        }
        Adt138::C1(v2) => {
            Err::<Adt133, Adt126>(v2)
        }
    }
}

pub fn f380(v0: Str) -> Adt137 {
    let a1248 = v0.clone();
    let a1249 = Str::lit("server/discover");
    if equal(a1248, a1249) {
        Adt137::C1(Adt134::C0)
    } else {
        let a1250 = v0.clone();
        let a1251 = Str::lit("tools/list");
        if equal(a1250, a1251) {
            Adt137::C1(Adt134::C1)
        } else {
            let a1252 = v0.clone();
            let a1253 = Str::lit("tools/call");
            if equal(a1252, a1253) {
                Adt137::C1(Adt134::C2)
            } else {
                let a1254 = v0.clone();
                let a1255 = Str::lit("resources/list");
                if equal(a1254, a1255) {
                    Adt137::C1(Adt134::C3)
                } else {
                    let a1256 = v0.clone();
                    let a1257 = Str::lit("resources/read");
                    if equal(a1256, a1257) {
                        Adt137::C1(Adt134::C4)
                    } else {
                        let a1258 = v0.clone();
                        let a1259 = Str::lit("resources/templates/list");
                        if equal(a1258, a1259) {
                            Adt137::C1(Adt134::C5)
                        } else {
                            let a1260 = v0.clone();
                            let a1261 = Str::lit("prompts/list");
                            if equal(a1260, a1261) {
                                Adt137::C1(Adt134::C6)
                            } else {
                                let a1262 = v0.clone();
                                let a1263 = Str::lit("prompts/get");
                                if equal(a1262, a1263) {
                                    Adt137::C1(Adt134::C7)
                                } else {
                                    let a1264 = v0.clone();
                                    let a1265 = Str::lit("completion/complete");
                                    if equal(a1264, a1265) {
                                        Adt137::C1(Adt134::C8)
                                    } else {
                                        let a1266 = v0.clone();
                                        let a1267 = Str::lit("tasks/get");
                                        if equal(a1266, a1267) {
                                            Adt137::C1(Adt134::C9)
                                        } else {
                                            let a1268 = v0.clone();
                                            let a1269 = Str::lit("tasks/update");
                                            if equal(a1268, a1269) {
                                                Adt137::C1(Adt134::C10)
                                            } else {
                                                let a1270 = v0;
                                                let a1271 = Str::lit("tasks/cancel");
                                                if equal(a1270, a1271) {
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

pub fn f381(v0: Option<Adt107>, v1: List<Str>, v2: Adt103) -> R<Adt138> {
    f473(v0, v1, v2.clone(), f474(match v2 {
        Adt103::C0(_, _, h1272) => {
            h1272
        }
    }))
}

pub fn f382(v0: List<u64>) -> Option<Str> {
    let a1273 = f246(f247(), v0);
    utf8_decode(a1273)
}

pub fn f383(v0: Str) -> Adt140 {
    let m1274 = f380(v0);
    match m1274 {
        Adt137::C0 => {
            Adt140::C0
        }
        Adt137::C1(v1) => {
            f475(v1)
        }
    }
}

pub fn f384(v0: List<u64>) -> Option<List<u64>> {
    let m1275 = f476(f477(), v0);
    match m1275 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1276 = f476(f478(), f230(v1, List::<u64>::nil()));
            match m1276 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f230(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f385(p1279: List<Adt110>, p1280: List<Adt110>) -> List<Adt110> {
    let m1278 = (p1279, p1280);
    {
        let mut m1278 = m1278;
        loop {
            let step = {
                let (v0, v1) = m1278;
                let m1277 = v0;
                match m1277.uncons() {
                    None => {
                        Err::<(List<Adt110>, List<Adt110>), List<Adt110>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt110>, List<Adt110>), List<Adt110>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: List<Adt86>, p1287: List<Adt12>, v2: Option<Str>, p1288: List<Adt110>) -> R<List<Adt110>> {
    let m1286 = (p1287, p1288);
    {
        let mut m1286 = m1286;
        loop {
            let step = {
                let (v1, v3) = m1286;
                let m1281 = v1;
                match m1281.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f479(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>((v5, List::cons(Adt110::C0(match v4.clone() {
                                Adt12::C0(_, h1282, _, _) => {
                                    h1282
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1283, _) => {
                                    h1283
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1284) => {
                                    h1284
                                }
                            }, f480(v0.clone(), match v4 {
                                Adt12::C0(_, h1285, _, _) => {
                                    h1285
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(p1304: u64, p1305: i64, p1306: i64) -> R<Option<i64>> {
    let m1303 = (p1304, (p1305, p1306));
    {
        let mut m1303 = m1303;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1303;
                let m1289 = v0;
                if m1289 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1289 - 1;
                    if f481(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1292 = {
                            let a1290 = v2;
                            let a1291 = 0i64;
                            compare(a1290, a1291)
                        };
                        if match m1292 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f481({
                                let a1293 = v1;
                                let a1294 = 10i64;
                                let a1295 = 0i64;
                                int_rem(a1293, a1294, a1295)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1296 = v1;
                                    let a1297 = 10i64;
                                    let a1298 = 0i64;
                                    int_quot(a1296, a1297, a1298)?
                                }, f244(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1299 = v1;
                                let a1300 = 10i64;
                                int_mul(a1299, a1300)?
                            }, {
                                let a1301 = v2;
                                let a1302 = 1i64;
                                int_sub(a1301, a1302)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: i64) -> Option<i64> {
    let m1309 = {
        let a1307 = 9007199254740991i64;
        let a1308 = v0;
        compare(a1307, a1308)
    };
    if if match m1309 {
        Ordering::Less => {
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
        let m1312 = {
            let a1310 = v0;
            let a1311 = -9007199254740991i64;
            compare(a1310, a1311)
        };
        match m1312 {
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

pub fn f389(v0: Str, v1: Str) -> Adt141 {
    let a1313 = v0;
    let a1314 = v1;
    if equal(a1313, a1314) {
        Adt141::C0
    } else {
        Adt141::C1
    }
}

pub fn f390(v0: Adt130) -> bool {
    let m1315 = v0;
    match m1315 {
        Adt130::C0 => {
            false
        }
        Adt130::C1(_) => {
            true
        }
    }
}

pub fn f391(v0: Adt92, v1: Str, v2: Adt5) -> Result<Adt103, Adt94> {
    let m1316 = f266(Str::lit("params"), v2);
    match m1316 {
        Adt130::C0 => {
            Ok::<Adt103, Adt94>(Adt103::C0(v0, v1, None::<Adt5>))
        }
        Adt130::C1(v3) => {
            let m1317 = f285(v3);
            match m1317 {
                None => {
                    f216(Some(v0), Adt95::C6)
                }
                Some(v4) => {
                    Ok::<Adt103, Adt94>(Adt103::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f392(v0: Adt118) -> bool {
    let m1318 = v0;
    match m1318 {
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

pub fn f393(v0: Adt114, v1: Adt119, v2: Adt115, v3: u64) -> R<Adt131> {
    let a1320 = match v0.clone() {
        Adt114::C0(_, _, h1319) => {
            h1319
        }
    };
    let a1321 = f438();
    if nat_lt(a1320, a1321) {
        Ok(Adt131::C1(Adt114::C0(v2, List::cons(v1, match v0.clone() {
            Adt114::C0(_, h1322, _) => {
                h1322
            }
        }), {
            let a1324 = match v0 {
                Adt114::C0(_, _, h1323) => {
                    h1323
                }
            };
            let a1325 = 1u64;
            nat_add(a1324, a1325)?
        })))
    } else {
        Ok(f347(Adt93::C9(v3)))
    }
}

pub fn f394(v0: bool, v1: Adt118, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt115 {
    Adt115::C8(Adt117::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f395(v0: u64) -> i64 {
    let a1326 = v0;
    let a1327 = 0u64;
    if nat_eq(a1326, a1327) {
        0i64
    } else {
        let a1328 = v0;
        let a1329 = 1u64;
        if nat_eq(a1328, a1329) {
            1i64
        } else {
            let a1330 = v0;
            let a1331 = 2u64;
            if nat_eq(a1330, a1331) {
                2i64
            } else {
                let a1332 = v0;
                let a1333 = 3u64;
                if nat_eq(a1332, a1333) {
                    3i64
                } else {
                    let a1334 = v0;
                    let a1335 = 4u64;
                    if nat_eq(a1334, a1335) {
                        4i64
                    } else {
                        let a1336 = v0;
                        let a1337 = 5u64;
                        if nat_eq(a1336, a1337) {
                            5i64
                        } else {
                            let a1338 = v0;
                            let a1339 = 6u64;
                            if nat_eq(a1338, a1339) {
                                6i64
                            } else {
                                let a1340 = v0;
                                let a1341 = 7u64;
                                if nat_eq(a1340, a1341) {
                                    7i64
                                } else {
                                    let a1342 = v0;
                                    let a1343 = 8u64;
                                    if nat_eq(a1342, a1343) {
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

pub fn f396(p1346: List<Adt6>, p1347: List<Adt6>) -> List<Adt6> {
    let m1345 = (p1346, p1347);
    {
        let mut m1345 = m1345;
        loop {
            let step = {
                let (v0, v1) = m1345;
                let m1344 = v0;
                match m1344.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Adt114, v1: u64) -> Adt131 {
    let m1349 = match v0.clone() {
        Adt114::C0(_, h1348, _) => {
            h1348
        }
    };
    match m1349.uncons() {
        None => {
            f347(Adt93::C2(v1))
        }
        Some((v2, _)) => {
            let m1350 = v2;
            match m1350 {
                Adt119::C0(_) => {
                    f346(v0, Adt115::C0)
                }
                Adt119::C1(_, _) => {
                    f346(v0, Adt115::C4)
                }
            }
        }
    }
}

pub fn f398(p1353: List<(Str, Adt6)>, p1354: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1352 = (p1353, p1354);
    {
        let mut m1352 = m1352;
        loop {
            let step = {
                let (v0, v1) = m1352;
                let m1351 = v0;
                match m1351.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
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

pub fn f399(v0: Adt114, v1: Adt115) -> Adt114 {
    Adt114::C0(v1, match v0.clone() {
        Adt114::C0(_, h1355, _) => {
            h1355
        }
    }, match v0 {
        Adt114::C0(_, _, h1356) => {
            h1356
        }
    })
}

pub fn f400(v0: Adt114, v1: u64, v2: List<u64>, v3: bool) -> Adt131 {
    let m1358 = {
        let a1357 = f246(f247(), f253(v2, List::<u64>::nil()));
        utf8_decode(a1357)
    };
    match m1358 {
        None => {
            f347(Adt93::C14)
        }
        Some(v4) => {
            if v3 {
                f482(v0, v1, v4)
            } else {
                f406(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f401(v0: u64) -> u64 {
    let a1359 = v0;
    let a1360 = 34u64;
    if nat_eq(a1359, a1360) {
        34u64
    } else {
        let a1361 = v0;
        let a1362 = 92u64;
        if nat_eq(a1361, a1362) {
            92u64
        } else {
            let a1363 = v0;
            let a1364 = 47u64;
            if nat_eq(a1363, a1364) {
                47u64
            } else {
                let a1365 = v0;
                let a1366 = 98u64;
                if nat_eq(a1365, a1366) {
                    8u64
                } else {
                    let a1367 = v0;
                    let a1368 = 102u64;
                    if nat_eq(a1367, a1368) {
                        12u64
                    } else {
                        let a1369 = v0;
                        let a1370 = 110u64;
                        if nat_eq(a1369, a1370) {
                            10u64
                        } else {
                            let a1371 = v0;
                            let a1372 = 114u64;
                            if nat_eq(a1371, a1372) {
                                13u64
                            } else {
                                let a1373 = v0;
                                let a1374 = 116u64;
                                if nat_eq(a1373, a1374) {
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

pub fn f402(v0: u64) -> u64 {
    let a1375 = v0;
    let a1376 = 48u64;
    if nat_lt(a1375, a1376) {
        16u64
    } else {
        let a1377 = v0;
        let a1378 = 58u64;
        if nat_lt(a1377, a1378) {
            let a1379 = v0;
            let a1380 = 48u64;
            nat_sub(a1379, a1380)
        } else {
            let a1381 = v0;
            let a1382 = 65u64;
            if nat_lt(a1381, a1382) {
                16u64
            } else {
                let a1383 = v0;
                let a1384 = 71u64;
                if nat_lt(a1383, a1384) {
                    let a1385 = v0;
                    let a1386 = 55u64;
                    nat_sub(a1385, a1386)
                } else {
                    let a1387 = v0;
                    let a1388 = 97u64;
                    if nat_lt(a1387, a1388) {
                        16u64
                    } else {
                        let a1389 = v0;
                        let a1390 = 103u64;
                        if nat_lt(a1389, a1390) {
                            let a1391 = v0;
                            let a1392 = 87u64;
                            nat_sub(a1391, a1392)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f403(v0: Adt114, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt131> {
    let a1393 = v3;
    let a1394 = 0u64;
    if nat_eq(a1393, a1394) {
        if f338(v2, 55296u64, 56319u64) {
            Ok(f346(v0, Adt115::C6(Adt116::C3(v2), v4, v5)))
        } else {
            if f338(v2, 56320u64, 57343u64) {
                Ok(f347(Adt93::C6(v1)))
            } else {
                Ok(f346(v0, Adt115::C6(Adt116::C0, f483(v2, v4)?, v5)))
            }
        }
    } else {
        if f338(v2, 56320u64, 57343u64) {
            Ok(f346(v0, Adt115::C6(Adt116::C0, f483({
                let a1403 = 65536u64;
                let a1404 = {
                    let a1401 = {
                        let a1397 = {
                            let a1395 = v3;
                            let a1396 = 55296u64;
                            nat_sub(a1395, a1396)
                        };
                        let a1398 = 1024u64;
                        nat_mul(a1397, a1398)?
                    };
                    let a1402 = {
                        let a1399 = v2;
                        let a1400 = 56320u64;
                        nat_sub(a1399, a1400)
                    };
                    nat_add(a1401, a1402)?
                };
                nat_add(a1403, a1404)?
            }, v4)?, v5)))
        } else {
            Ok(f347(Adt93::C6(v1)))
        }
    }
}

pub fn f404(v0: u64, v1: u64) -> u64 {
    let m1407 = {
        let a1405 = f405(v0);
        let a1406 = v1;
        index_list(a1405, a1406)
    };
    match m1407 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f405(v0: u64) -> List<u64> {
    let a1408 = v0;
    let a1409 = 0u64;
    if nat_eq(a1408, a1409) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1410 = v0;
        let a1411 = 1u64;
        if nat_eq(a1410, a1411) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f406(v0: Adt114, v1: Adt6) -> Adt131 {
    Adt131::C1(f340(v1, match v0.clone() {
        Adt114::C0(_, h1412, _) => {
            h1412
        }
    }, match v0 {
        Adt114::C0(_, _, h1413) => {
            h1413
        }
    }))
}

pub fn f407(v0: u64) -> Adt6 {
    let a1414 = v0;
    let a1415 = 0u64;
    if nat_eq(a1414, a1415) {
        Adt6::C1(true)
    } else {
        let a1416 = v0;
        let a1417 = 1u64;
        if nat_eq(a1416, a1417) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f408(v0: Adt114, v1: Adt117, v2: Adt118) -> Adt131 {
    f346(v0, Adt115::C8(Adt117::C0(match v1.clone() {
        Adt117::C0(h1418, _, _, _, _, _, _) => {
            h1418
        }
    }, v2, match v1.clone() {
        Adt117::C0(_, _, h1419, _, _, _, _) => {
            h1419
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, h1420, _, _, _) => {
            h1420
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, _, h1421, _, _) => {
            h1421
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, _, _, h1422, _) => {
            h1422
        }
    }, match v1 {
        Adt117::C0(_, _, _, _, _, _, h1423) => {
            h1423
        }
    })))
}

pub fn f409(v0: Adt114, v1: u64, v2: u64, v3: Adt117, v4: bool) -> R<Adt131> {
    let v5: u64 = {
        let a1425 = match v3.clone() {
            Adt117::C0(_, _, _, h1424, _, _, _) => {
                h1424
            }
        };
        let a1426 = 0u64;
        if if nat_eq(a1425, a1426) {
            let a1427 = v2;
            let a1428 = 48u64;
            nat_eq(a1427, a1428)
        } else {
            false
        } {
            0u64
        } else {
            let a1430 = match v3.clone() {
                Adt117::C0(_, _, _, h1429, _, _, _) => {
                    h1429
                }
            };
            let a1431 = 1u64;
            nat_add(a1430, a1431)?
        }
    };
    let v6: i64 = if v4 {
        f244(match v3.clone() {
            Adt117::C0(_, _, _, _, h1432, _, _) => {
                h1432
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt117::C0(_, _, _, _, h1433, _, _) => {
                h1433
            }
        }
    };
    let a1434 = f484();
    let a1435 = v5;
    if nat_lt(a1434, a1435) {
        Ok(f347(Adt93::C8(v1)))
    } else {
        let m1438 = {
            let a1436 = 99999i64;
            let a1437 = v6;
            compare(a1436, a1437)
        };
        if match m1438 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f347(Adt93::C8(v1)))
        } else {
            Ok(f346(v0, f394(match v3.clone() {
                Adt117::C0(h1439, _, _, _, _, _, _) => {
                    h1439
                }
            }, if v4 {
                Adt118::C4
            } else {
                Adt118::C2
            }, f244({
                let a1441 = match v3.clone() {
                    Adt117::C0(_, _, h1440, _, _, _, _) => {
                        h1440
                    }
                };
                let a1442 = 10i64;
                int_mul(a1441, a1442)?
            }, f395({
                let a1443 = v2;
                let a1444 = 48u64;
                nat_sub(a1443, a1444)
            }))?, v5, v6, match v3.clone() {
                Adt117::C0(_, _, _, _, _, h1445, _) => {
                    h1445
                }
            }, match v3 {
                Adt117::C0(_, _, _, _, _, _, h1446) => {
                    h1446
                }
            })))
        }
    }
}

pub fn f410(v0: u64) -> bool {
    let a1447 = v0;
    let a1448 = 101u64;
    if nat_eq(a1447, a1448) {
        true
    } else {
        let a1449 = v0;
        let a1450 = 69u64;
        nat_eq(a1449, a1450)
    }
}

pub fn f411(v0: u64) -> bool {
    f338(v0, 48u64, 57u64)
}

pub fn f412(v0: Adt114, v1: u64, v2: u64, v3: Adt117) -> R<Adt131> {
    let m1451 = f339(v3)?;
    match m1451 {
        Adt135::C0 => {
            Ok(f347(Adt93::C7(v1)))
        }
        Adt135::C1 => {
            Ok(f347(Adt93::C8(v1)))
        }
        Adt135::C2(v4) => {
            Ok(f344(f340(v4, match v0.clone() {
                Adt114::C0(_, h1452, _) => {
                    h1452
                }
            }, match v0 {
                Adt114::C0(_, _, h1453) => {
                    h1453
                }
            }), v1, v2))
        }
    }
}

pub fn f413(v0: Adt114, v1: u64, v2: u64, v3: Adt117, v4: bool) -> R<Adt131> {
    let v5: i64 = f244({
        let a1455 = match v3.clone() {
            Adt117::C0(_, _, _, _, _, _, h1454) => {
                h1454
            }
        };
        let a1456 = 10i64;
        int_mul(a1455, a1456)?
    }, f395({
        let a1457 = v2;
        let a1458 = 48u64;
        nat_sub(a1457, a1458)
    }))?;
    let m1461 = {
        let a1459 = 99999i64;
        let a1460 = v5;
        compare(a1459, a1460)
    };
    if match m1461 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f347(Adt93::C8(v1)))
    } else {
        Ok(f346(v0, f394(match v3.clone() {
            Adt117::C0(h1462, _, _, _, _, _, _) => {
                h1462
            }
        }, Adt118::C7, match v3.clone() {
            Adt117::C0(_, _, h1463, _, _, _, _) => {
                h1463
            }
        }, match v3.clone() {
            Adt117::C0(_, _, _, h1464, _, _, _) => {
                h1464
            }
        }, match v3 {
            Adt117::C0(_, _, _, _, h1465, _, _) => {
                h1465
            }
        }, v4, v5)))
    }
}

pub fn f414(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1466 = f485(f486(), v2.clone());
    match m1466 {
        None => {
            let m1467 = f487(f488(), v2.clone());
            match m1467 {
                None => {
                    f489(v0, v1, v2)
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

pub fn f415(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt120) -> R<Adt142> {
    let m1468 = f334(Str::lit("x-mcp-header"), v1.clone());
    match m1468 {
        Adt130::C0 => {
            Ok(Adt142::C1(v4))
        }
        Adt130::C1(v5) => {
            f490(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f416(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt104>) -> List<Adt104> {
    f491(Str::lit("properties"), v0.clone(), v1, v2, f491(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f492(Str::lit("items"), v0.clone(), f492(Str::lit("additionalProperties"), v0.clone(), f493(Str::lit("allOf"), v0.clone(), f493(Str::lit("anyOf"), v0.clone(), f493(Str::lit("oneOf"), v0.clone(), f492(Str::lit("not"), v0.clone(), f492(Str::lit("if"), v0.clone(), f492(Str::lit("then"), v0.clone(), f492(Str::lit("else"), v0, v3)))))))))))
}

pub fn f417(p1475: List<(Str, Adt136)>) -> Adt121 {
    let m1474 = p1475;
    {
        let mut m1474 = m1474;
        loop {
            let step = {
                let v0 = m1474;
                let m1469 = v0;
                match m1469.uncons() {
                    None => {
                        Err::<List<(Str, Adt136)>, Adt121>(Adt121::C3(List::<Adt101>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1471 = {
                            let (_, h1470) = v1;
                            h1470
                        };
                        match m1471 {
                            Adt136::C0 => {
                                Ok::<List<(Str, Adt136)>, Adt121>(v2)
                            }
                            Adt136::C1 => {
                                Err::<List<(Str, Adt136)>, Adt121>(Adt121::C0({
                                    let (h1472, _) = v1;
                                    h1472
                                }))
                            }
                            Adt136::C2 => {
                                Err::<List<(Str, Adt136)>, Adt121>(Adt121::C1({
                                    let (h1473, _) = v1;
                                    h1473
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1476 = f334(Str::lit("type"), v0);
    match m1476 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1477 = f331(v2.clone());
            match m1477 {
                None => {
                    let m1478 = f359(v2);
                    match m1478 {
                        None => {
                            Ok(Adt136::C2)
                        }
                        Some(v4) => {
                            f496(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f494(v3.clone()) {
                        if f495(v3, v1)? {
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

pub fn f419(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1479 = f334(Str::lit("enum"), v0);
    match m1479 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1480 = f359(v2);
            match m1480 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v3) => {
                    f497(v3, v1)
                }
            }
        }
    }
}

pub fn f420(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1481 = f334(Str::lit("const"), v0);
    match m1481 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1482 = f498(v2, v1)?;
            match m1482 {
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

pub fn f421(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt136> {
    let m1483 = f334(v0, v2);
    match m1483 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v4) => {
            let m1484 = f499(v4);
            match m1484 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v5) => {
                    let m1485 = f499(v3);
                    match m1485 {
                        None => {
                            Ok(Adt136::C0)
                        }
                        Some(v6) => {
                            if f500(v1, f501(v6, v5)?) {
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

pub fn f422(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt136)>> {
    let m1486 = v1;
    match m1486 {
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
            Ok(List::cons((Str::lit("minLength"), f502(Str::lit("minLength"), true, v0.clone(), f503(v4.clone())?)?), List::cons((Str::lit("maxLength"), f502(Str::lit("maxLength"), false, v0, f503(v4)?)?), List::<(Str, Adt136)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f502(Str::lit("minItems"), true, v0.clone(), f504(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f502(Str::lit("maxItems"), false, v0, f504(v5, 0u64)?)?), List::<(Str, Adt136)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f502(Str::lit("minProperties"), true, v0.clone(), f505(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f502(Str::lit("maxProperties"), false, v0, f505(v6, 0u64)?)?), List::<(Str, Adt136)>::nil())))
        }
    }
}

pub fn f423(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt136 {
    let m1487 = f334(Str::lit("required"), v0);
    match m1487 {
        Adt130::C0 => {
            Adt136::C0
        }
        Adt130::C1(v2) => {
            let m1488 = f359(v2);
            match m1488 {
                None => {
                    Adt136::C2
                }
                Some(v3) => {
                    let m1489 = f285(v1);
                    match m1489 {
                        None => {
                            Adt136::C0
                        }
                        Some(v4) => {
                            f506(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f424(v0: Adt121, v1: Adt121) -> Adt121 {
    let m1490 = v0.clone();
    match m1490 {
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
            let m1491 = v1.clone();
            match m1491 {
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
                        let a1492 = v5;
                        let a1493 = v9;
                        append_list(a1492, a1493)
                    })
                }
            }
        }
    }
}

pub fn f425(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> Adt121 {
    let m1494 = f334(Str::lit("$ref"), v1);
    match m1494 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v4) => {
            let m1495 = f331(v4);
            match m1495 {
                None => {
                    Adt121::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1496 = f507(v0, v5.clone());
                    match m1496 {
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

pub fn f426(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1497 = f334(Str::lit("properties"), v0);
    match m1497 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1498 = f285(v3);
            match m1498 {
                None => {
                    Adt121::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1499 = f285(v1);
                    match m1499 {
                        None => {
                            Adt121::C3(List::<Adt101>::nil())
                        }
                        Some(v5) => {
                            Adt121::C3(f508(match v4 {
                                Adt5::C0(h1500) => {
                                    h1500
                                }
                            }, match v5 {
                                Adt5::C0(h1501) => {
                                    h1501
                                }
                            }, v2, List::<Adt101>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f427(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1502 = f334(Str::lit("additionalProperties"), v0.clone());
    match m1502 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1503 = f285(v1);
            match m1503 {
                None => {
                    Adt121::C3(List::<Adt101>::nil())
                }
                Some(v4) => {
                    let m1504 = f284(v3.clone());
                    match m1504 {
                        None => {
                            Adt121::C3(f511(match v4 {
                                Adt5::C0(h1506) => {
                                    h1506
                                }
                            }, f510(v0), v3, v2, List::<Adt101>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt121::C3(List::<Adt101>::nil())
                            } else {
                                if f509(match v4 {
                                    Adt5::C0(h1505) => {
                                        h1505
                                    }
                                }, f510(v0)) {
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

pub fn f428(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> R<Adt121> {
    let m1507 = f334(Str::lit("items"), v0);
    match m1507 {
        Adt130::C0 => {
            Ok(Adt121::C3(List::<Adt101>::nil()))
        }
        Adt130::C1(v3) => {
            let m1508 = f359(v1);
            match m1508 {
                None => {
                    Ok(Adt121::C3(List::<Adt101>::nil()))
                }
                Some(v4) => {
                    Ok(Adt121::C3(f512(v4, 0u64, v3, v2, List::<Adt101>::nil())?))
                }
            }
        }
    }
}

pub fn f429(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1509 = f334(Str::lit("allOf"), v0);
    match m1509 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1510 = f359(v3);
            match m1510 {
                None => {
                    Adt121::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1511 = v4.clone();
                    match m1511.uncons() {
                        None => {
                            Adt121::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt121::C3(f513(v4, v1, v2, List::<Adt101>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f430(v0: Adt102, v1: Adt121, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt99>) -> Adt121 {
    let m1512 = v1.clone();
    match m1512 {
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
            let m1513 = v8;
            match m1513.uncons() {
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

pub fn f431(p1522: u64, p1523: List<u64>, p1524: List<u64>) -> R<Option<List<u64>>> {
    let m1521 = (p1522, (p1523, p1524));
    {
        let mut m1521 = m1521;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1521;
                let m1514 = v0;
                if m1514 == 0 {
                    let m1515 = v1;
                    match m1515.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1514 - 1;
                    let m1516 = v1;
                    match m1516.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1517 = v7;
                            match m1517.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1518 = v9;
                                    match m1518.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1519 = v11;
                                            match m1519.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1520 = f514(v6, v8, v10, v12, f515(v13.clone()))?;
                                                    match m1520 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f253(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(p1529: List<Adt50>, p1530: Adt7) -> R<bool> {
    let m1528 = (p1529, p1530);
    {
        let mut m1528 = m1528;
        loop {
            let step = {
                let (v0, v1) = m1528;
                let m1525 = v0;
                match m1525.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f516(f501(match v2.clone() {
                            Adt50::C0(h1526, _, _) => {
                                h1526
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1527, _, _) => {
                                    h1527
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(p1537: List<u64>, p1538: List<u64>) -> List<u64> {
    let m1536 = (p1537, p1538);
    {
        let mut m1536 = m1536;
        loop {
            let step = {
                let (v0, v1) = m1536;
                let m1531 = v0;
                match m1531.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1532 = v2;
                        let a1533 = 126u64;
                        if nat_eq(a1532, a1533) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1534 = v2;
                            let a1535 = 47u64;
                            if nat_eq(a1534, a1535) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f435(v0: u64) -> List<u64> {
    f405(v0)
}

pub fn f436(v0: Adt7) -> R<bool> {
    let a1556 = {
        let m1542 = {
            let a1540 = f517();
            let a1541 = match v0.clone() {
                Adt7::C0(h1539, _) => {
                    h1539
                }
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
            let m1547 = {
                let a1545 = match v0.clone() {
                    Adt7::C0(h1543, _) => {
                        h1543
                    }
                };
                let a1546 = {
                    let a1544 = f517();
                    int_neg(a1544)?
                };
                compare(a1545, a1546)
            };
            if match m1547 {
                Ordering::Less => {
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
                let m1551 = {
                    let a1549 = 9999i64;
                    let a1550 = match v0.clone() {
                        Adt7::C0(_, h1548) => {
                            h1548
                        }
                    };
                    compare(a1549, a1550)
                };
                if match m1551 {
                    Ordering::Less => {
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
                    let m1555 = {
                        let a1553 = match v0 {
                            Adt7::C0(_, h1552) => {
                                h1552
                            }
                        };
                        let a1554 = -9999i64;
                        compare(a1553, a1554)
                    };
                    match m1555 {
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
    Ok(bool_not(a1556))
}

pub fn f437(v0: Adt7) -> R<List<u64>> {
    let m1560 = {
        let a1558 = match v0.clone() {
            Adt7::C0(_, h1557) => {
                h1557
            }
        };
        let a1559 = 0i64;
        compare(a1558, a1559)
    };
    if if match m1560 {
        Ordering::Less => {
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
        let m1564 = {
            let a1562 = 0i64;
            let a1563 = match v0.clone() {
                Adt7::C0(_, h1561) => {
                    h1561
                }
            };
            compare(a1562, a1563)
        };
        match m1564 {
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
        let a1567 = f518(match v0.clone() {
            Adt7::C0(h1565, _) => {
                h1565
            }
        })?;
        let a1568 = List::cons(101u64, f518(match v0 {
            Adt7::C0(_, h1566) => {
                h1566
            }
        })?);
        Ok(append_list(a1567, a1568))
    } else {
        f518(match v0 {
            Adt7::C0(h1569, _) => {
                h1569
            }
        })
    }
}

pub fn f438() -> u64 {
    512u64
}

pub fn f439(v0: u64, v1: Adt106, v2: List<Adt106>) -> List<Adt106> {
    List::cons(Adt106::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f440(p1572: List<u64>, p1573: List<u64>) -> R<List<u64>> {
    let m1571 = (p1572, p1573);
    {
        let mut m1571 = m1571;
        loop {
            let step = {
                let (v0, v1) = m1571;
                let m1570 = v0;
                match m1570.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f253(f519(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f520(match v0.clone() {
        Adt33::C0(_, h1574, _, _) => {
            h1574
        }
    })), List::cons((Str::lit("capabilities"), f521(match v0.clone() {
        Adt33::C0(_, _, h1575, _) => {
            h1575
        }
    })), {
        let a1578 = f522(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1576) => {
                h1576
            }
        });
        let a1579 = f523(match v0 {
            Adt33::C0(h1577, _, _, _) => {
                h1577
            }
        })?;
        append_list(a1578, a1579)
    })))
}

pub fn f442(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f524(match v0.clone() {
        Adt34::C0(_, _, h1580) => {
            h1580
        }
    }))), {
        let a1583 = f522(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1581, _) => {
                h1581
            }
        });
        let a1584 = f523(match v0 {
            Adt34::C0(h1582, _, _) => {
                h1582
            }
        })?;
        append_list(a1583, a1584)
    }))
}

pub fn f443(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1585 = v0;
    match m1585 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1586 = f456(Adt6::C5(v1))?;
            match m1586 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1587 = f285(v2);
                    match m1587 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1588) => {
                                    h1588
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f444(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1590 = f525(match v0.clone() {
        Adt37::C0(h1589, _, _) => {
            h1589
        }
    })?;
    match m1590 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1593 = f526(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1591, _) => {
                        h1591
                    }
                });
                let a1594 = f527(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1592) => {
                        h1592
                    }
                });
                append_list(a1593, a1594)
            })))
        }
    }
}

pub fn f445(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f528(match v0.clone() {
        Adt51::C0(_, _, h1595) => {
            h1595
        }
    })?)), {
        let a1598 = f522(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1596, _) => {
                h1596
            }
        });
        let a1599 = f523(match v0 {
            Adt51::C0(h1597, _, _) => {
                h1597
            }
        })?;
        append_list(a1598, a1599)
    }))
}

pub fn f446(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f529(match v0.clone() {
        Adt52::C0(_, h1600) => {
            h1600
        }
    }))), f523(match v0 {
        Adt52::C0(h1601, _) => {
            h1601
        }
    })?))
}

pub fn f447(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f530(match v0.clone() {
        Adt53::C0(_, _, h1602) => {
            h1602
        }
    }))), {
        let a1605 = f522(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1603, _) => {
                h1603
            }
        });
        let a1606 = f523(match v0 {
            Adt53::C0(h1604, _, _) => {
                h1604
            }
        })?;
        append_list(a1605, a1606)
    }))
}

pub fn f448(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f531(match v0.clone() {
        Adt55::C0(_, _, h1607) => {
            h1607
        }
    }))), {
        let a1610 = f522(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1608, _) => {
                h1608
            }
        });
        let a1611 = f523(match v0 {
            Adt55::C0(h1609, _, _) => {
                h1609
            }
        })?;
        append_list(a1610, a1611)
    }))
}

pub fn f449(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1613 = f532(match v0.clone() {
        Adt58::C0(_, h1612) => {
            h1612
        }
    })?;
    match m1613 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1615 = f522(Str::lit("description"), match v0 {
                    Adt58::C0(h1614, _) => {
                        h1614
                    }
                });
                let a1616 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1615, a1616)
            }))
        }
    }
}

pub fn f450(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f466({
        let m1619 = match v0.clone() {
            Adt60::C0(h1618) => {
                h1618
            }
        };
        match m1619 {
            Adt61::C0(h1617, _, _) => {
                h1617
            }
        }
    }))), {
        let a1626 = f533(Str::lit("total"), {
            let m1622 = match v0.clone() {
                Adt60::C0(h1621) => {
                    h1621
                }
            };
            match m1622 {
                Adt61::C0(_, h1620, _) => {
                    h1620
                }
            }
        })?;
        let a1627 = f527(Str::lit("hasMore"), {
            let m1625 = match v0 {
                Adt60::C0(h1624) => {
                    h1624
                }
            };
            match m1625 {
                Adt61::C0(_, _, h1623) => {
                    h1623
                }
            }
        });
        append_list(a1626, a1627)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f451(v0: Adt143) -> Str {
    let m1628 = v0;
    match m1628 {
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

pub fn f452(v0: Adt32) -> Adt143 {
    let m1629 = v0;
    match m1629 {
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

pub fn f453(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1630 = v0;
    match m1630 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f534(), f535(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f454(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f536(v2) {
            let a1631 = v1;
            let a1632 = 200u64;
            nat_eq(a1631, a1632)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f455(v0: u64, v1: List<List<u64>>) -> Adt88 {
    Adt88::C0(v0, f537(), List::<List<u64>>::nil(), f538(v1))
}

pub fn f456(v0: Adt6) -> R<Option<Adt6>> {
    f539(f540(), Adt144::C0(v0), List::<Adt145>::nil())
}

pub fn f457(v0: Adt125) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f207())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f541(match v0.clone() {
        Adt125::C0(h1633, _) => {
            h1633
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1636 = match v0.clone() {
            Adt125::C0(_, h1635) => {
                h1635
            }
        };
        match m1636 {
            Adt50::C0(h1634, _, _) => {
                h1634
            }
        }
    })), {
        let a1643 = f542({
            let m1639 = match v0.clone() {
                Adt125::C0(_, h1638) => {
                    h1638
                }
            };
            match m1639 {
                Adt50::C0(_, h1637, _) => {
                    h1637
                }
            }
        });
        let a1644 = f543({
            let m1642 = match v0 {
                Adt125::C0(_, h1641) => {
                    h1641
                }
            };
            match m1642 {
                Adt50::C0(_, _, h1640) => {
                    h1640
                }
            }
        });
        append_list(a1643, a1644)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f458() -> u64 {
    429u64
}

pub fn f459() -> i64 {
    -32602i64
}

pub fn f460() -> i64 {
    -32022i64
}

pub fn f461() -> i64 {
    -32020i64
}

pub fn f462() -> i64 {
    -32601i64
}

pub fn f463(v0: Adt84) -> i64 {
    let m1645 = v0;
    match m1645 {
        Adt84::C0(_) => {
            f459()
        }
        Adt84::C1(_) => {
            f459()
        }
        Adt84::C2(_) => {
            f459()
        }
        Adt84::C3(_) => {
            f459()
        }
        Adt84::C4(_) => {
            f459()
        }
        Adt84::C5(_) => {
            f459()
        }
        Adt84::C6(_) => {
            f544()
        }
        Adt84::C7(_) => {
            f545()
        }
        Adt84::C8(_) => {
            f546()
        }
        Adt84::C9(_) => {
            f544()
        }
    }
}

pub fn f464(v0: Adt127) -> Str {
    let m1646 = v0;
    match m1646 {
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

pub fn f465(v0: Adt108) -> Str {
    let m1647 = v0;
    match m1647 {
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
            let m1648 = v1;
            match m1648 {
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
            let m1649 = v2;
            match m1649 {
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

pub fn f466(v0: List<Str>) -> List<Adt6> {
    f396(f547(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f467(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1661 = f548(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1650, _, _, _, _) => {
                h1650
            }
        });
        let a1662 = {
            let a1659 = f548(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1651, _, _, _) => {
                    h1651
                }
            });
            let a1660 = {
                let a1657 = f548(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1652, _, _) => {
                        h1652
                    }
                });
                let a1658 = {
                    let a1655 = f548(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1653, _) => {
                            h1653
                        }
                    });
                    let a1656 = f548(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1654) => {
                            h1654
                        }
                    });
                    append_list(a1655, a1656)
                };
                append_list(a1657, a1658)
            };
            append_list(a1659, a1660)
        };
        append_list(a1661, a1662)
    })
}

pub fn f468(p1665: List<List<u64>>) -> R<bool> {
    let m1664 = p1665;
    {
        let mut m1664 = m1664;
        loop {
            let step = {
                let v0 = m1664;
                let m1663 = v0;
                match m1663.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f549(f222(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f470(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133, v4: bool) -> R<Result<Adt123, Adt126>> {
    if v4 {
        f550(v0, v1, v2, f551(match v3.clone() {
            Adt133::C0(_, h1666, _, _) => {
                h1666
            }
        }, match v3.clone() {
            Adt133::C0(_, _, h1667, _) => {
                h1667
            }
        }, match v3 {
            Adt133::C0(_, _, _, h1668) => {
                h1668
            }
        }))
    } else {
        Ok(Err::<Adt123, Adt126>(Adt126::C3(f552(match v3 {
            Adt133::C0(_, h1669, _, _) => {
                h1669
            }
        }))))
    }
}

pub fn f471(v0: Adt4, v1: Adt134) -> bool {
    let m1670 = v1;
    match m1670 {
        Adt134::C0 => {
            true
        }
        Adt134::C1 => {
            f553(match v0 {
                Adt4::C0(_, _, _, _, _, h1671, _) => {
                    h1671
                }
            })
        }
        Adt134::C2 => {
            f553(match v0 {
                Adt4::C0(_, _, _, _, _, h1672, _) => {
                    h1672
                }
            })
        }
        Adt134::C3 => {
            f554(match v0 {
                Adt4::C0(_, _, _, _, h1673, _, _) => {
                    h1673
                }
            })
        }
        Adt134::C4 => {
            f554(match v0 {
                Adt4::C0(_, _, _, _, h1674, _, _) => {
                    h1674
                }
            })
        }
        Adt134::C5 => {
            f554(match v0 {
                Adt4::C0(_, _, _, _, h1675, _, _) => {
                    h1675
                }
            })
        }
        Adt134::C6 => {
            f553(match v0 {
                Adt4::C0(_, _, _, h1676, _, _, _) => {
                    h1676
                }
            })
        }
        Adt134::C7 => {
            f553(match v0 {
                Adt4::C0(_, _, _, h1677, _, _, _) => {
                    h1677
                }
            })
        }
        Adt134::C8 => {
            f555(match v0 {
                Adt4::C0(_, _, h1678, _, _, _, _) => {
                    h1678
                }
            })
        }
        Adt134::C9 => {
            f556(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1679) => {
                    h1679
                }
            }, f557())
        }
        Adt134::C10 => {
            f556(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1680) => {
                    h1680
                }
            }, f557())
        }
        Adt134::C11 => {
            f556(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1681) => {
                    h1681
                }
            }, f557())
        }
    }
}

pub fn f472(v0: Adt139, v1: Adt137) -> Result<Adt133, Adt126> {
    let m1682 = v1;
    match m1682 {
        Adt137::C0 => {
            Err::<Adt133, Adt126>(Adt126::C3(match v0 {
                Adt139::C0(_, h1683, _, _) => {
                    h1683
                }
            }))
        }
        Adt137::C1(v2) => {
            Ok::<Adt133, Adt126>(Adt133::C0(match v0.clone() {
                Adt139::C0(h1684, _, _, _) => {
                    h1684
                }
            }, v2, match v0.clone() {
                Adt139::C0(_, _, h1685, _) => {
                    h1685
                }
            }, match v0 {
                Adt139::C0(_, _, _, h1686) => {
                    h1686
                }
            }))
        }
    }
}

pub fn f473(v0: Option<Adt107>, v1: List<Str>, v2: Adt103, v3: Result<Adt16, Adt127>) -> R<Adt138> {
    let m1687 = v3;
    match m1687 {
        Ok(v5) => {
            Ok(f558(v1, v2.clone(), v5.clone(), f559(v0, f383(match v2.clone() {
                Adt103::C0(_, h1688, _) => {
                    h1688
                }
            }), match v2.clone() {
                Adt103::C0(_, h1689, _) => {
                    h1689
                }
            }, match v5 {
                Adt16::C0(h1690, _, _, _, _, _) => {
                    h1690
                }
            }, f560(match v2 {
                Adt103::C0(_, _, h1691) => {
                    h1691
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt138::C1(Adt126::C0(v4)))
        }
    }
}

pub fn f474(v0: Option<Adt5>) -> Result<Adt16, Adt127> {
    let m1692 = v0;
    match m1692 {
        None => {
            Err::<Adt16, Adt127>(Adt127::C0)
        }
        Some(v1) => {
            let m1693 = f266(Str::lit("_meta"), v1);
            match m1693 {
                Adt130::C0 => {
                    Err::<Adt16, Adt127>(Adt127::C1)
                }
                Adt130::C1(v2) => {
                    let m1694 = f285(v2);
                    match m1694 {
                        None => {
                            Err::<Adt16, Adt127>(Adt127::C4)
                        }
                        Some(v3) => {
                            f561(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f475(v0: Adt134) -> Adt140 {
    let m1695 = v0;
    match m1695 {
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

pub fn f476(p1701: List<u64>, p1702: List<u64>) -> Option<List<u64>> {
    let m1700 = (p1701, p1702);
    {
        let mut m1700 = m1700;
        loop {
            let step = {
                let (v0, v1) = m1700;
                let m1696 = v0;
                match m1696.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1697 = v1;
                        match m1697.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1698 = v4;
                                let a1699 = v2;
                                if nat_eq(a1698, a1699) {
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
                Ok(next) => { m1700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f478() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f479(v0: Adt12, v1: Option<Str>) -> bool {
    let m1703 = v1;
    match m1703 {
        None => {
            false
        }
        Some(v2) => {
            let a1705 = match v0 {
                Adt12::C0(h1704, _, _, _) => {
                    h1704
                }
            };
            let a1706 = v2;
            equal(a1705, a1706)
        }
    }
}

pub fn f480(v0: List<Adt86>, v1: Str) -> R<Adt111> {
    let m1709 = f326(f111(v0, {
        let a1707 = f562();
        let a1708 = f563(v1)?;
        append_list(a1707, a1708)
    })?)?;
    match m1709 {
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
            let m1710 = f382(v2);
            match m1710 {
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

pub fn f481(v0: i64) -> bool {
    let a1717 = {
        let m1713 = {
            let a1711 = v0;
            let a1712 = 0i64;
            compare(a1711, a1712)
        };
        if match m1713 {
            Ordering::Less => {
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
            let m1716 = {
                let a1714 = 0i64;
                let a1715 = v0;
                compare(a1714, a1715)
            };
            match m1716 {
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
    bool_not(a1717)
}

pub fn f482(v0: Adt114, v1: u64, v2: Str) -> Adt131 {
    let m1719 = match v0.clone() {
        Adt114::C0(_, h1718, _) => {
            h1718
        }
    };
    match m1719.uncons() {
        None => {
            f347(Adt93::C2(v1))
        }
        Some((v3, v4)) => {
            let m1720 = v3;
            match m1720 {
                Adt119::C0(_) => {
                    f347(Adt93::C2(v1))
                }
                Adt119::C1(v6, _) => {
                    Adt131::C1(Adt114::C0(Adt115::C5, List::cons(Adt119::C1(v6, Some(v2)), v4), match v0 {
                        Adt114::C0(_, _, h1721) => {
                            h1721
                        }
                    }))
                }
            }
        }
    }
}

pub fn f483(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1722 = v0;
    let a1723 = 128u64;
    if nat_lt(a1722, a1723) {
        Ok(List::cons(v0, v1))
    } else {
        let a1724 = v0;
        let a1725 = 2048u64;
        if nat_lt(a1724, a1725) {
            Ok(List::cons({
                let a1726 = 128u64;
                let a1727 = f564(v0, 64u64);
                nat_add(a1726, a1727)?
            }, List::cons({
                let a1728 = 192u64;
                let a1729 = f565(v0, 64u64);
                nat_add(a1728, a1729)?
            }, v1)))
        } else {
            let a1730 = v0;
            let a1731 = 65536u64;
            if nat_lt(a1730, a1731) {
                Ok(List::cons({
                    let a1732 = 128u64;
                    let a1733 = f564(v0, 64u64);
                    nat_add(a1732, a1733)?
                }, List::cons({
                    let a1734 = 128u64;
                    let a1735 = f564(f565(v0, 64u64), 64u64);
                    nat_add(a1734, a1735)?
                }, List::cons({
                    let a1736 = 224u64;
                    let a1737 = f565(v0, 4096u64);
                    nat_add(a1736, a1737)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1738 = 128u64;
                    let a1739 = f564(v0, 64u64);
                    nat_add(a1738, a1739)?
                }, List::cons({
                    let a1740 = 128u64;
                    let a1741 = f564(f565(v0, 64u64), 64u64);
                    nat_add(a1740, a1741)?
                }, List::cons({
                    let a1742 = 128u64;
                    let a1743 = f564(f565(v0, 4096u64), 64u64);
                    nat_add(a1742, a1743)?
                }, List::cons({
                    let a1744 = 240u64;
                    let a1745 = f565(v0, 262144u64);
                    nat_add(a1744, a1745)?
                }, v1)))))
            }
        }
    }
}

pub fn f484() -> u64 {
    18u64
}

pub fn f485(p1749: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1748 = p1749;
    {
        let mut m1748 = m1748;
        loop {
            let step = {
                let v0 = m1748;
                let m1746 = v0;
                match m1746.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1747 = f334(v2.clone(), v1.clone());
                        match m1747 {
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
                Ok(next) => { m1748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f486() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f487(p1756: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1755 = p1756;
    {
        let mut m1755 = m1755;
        loop {
            let step = {
                let v0 = m1755;
                let m1750 = v0;
                match m1750.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1752 = f334({
                            let (h1751, _) = v2.clone();
                            h1751
                        }, v1.clone());
                        match m1752 {
                            Adt130::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt130::C1(v4) => {
                                if f566({
                                    let (_, h1753) = v2;
                                    h1753
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1754, _) = v2;
                                        h1754
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f489(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1757 = f334(Str::lit("$ref"), v2);
    match m1757 {
        Adt130::C0 => {
            None::<Adt91>
        }
        Adt130::C1(v3) => {
            let m1758 = f331(v3);
            match m1758 {
                None => {
                    None::<Adt91>
                }
                Some(v4) => {
                    let m1759 = f507(v1, v4.clone());
                    match m1759 {
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

pub fn f490(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt120) -> R<Adt142> {
    if v3 {
        let m1760 = f331(v2);
        match m1760 {
            None => {
                Ok(Adt142::C0(Adt91::C7(v0)))
            }
            Some(v6) => {
                f567(v0, v1, v6.clone(), f179(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt142::C0(Adt91::C10(v0)))
    }
}

pub fn f491(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt104>) -> List<Adt104> {
    let m1761 = f334(v0.clone(), v1);
    match m1761 {
        Adt130::C0 => {
            v4
        }
        Adt130::C1(v5) => {
            let m1762 = f285(v5);
            match m1762 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1763 = v0;
                    let a1764 = Str::lit("properties");
                    if equal(a1763, a1764) {
                        f568(match v6 {
                            Adt5::C0(h1765) => {
                                h1765
                            }
                        }, v2, v3, v4)
                    } else {
                        f569(match v6 {
                            Adt5::C0(h1766) => {
                                h1766
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f492(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt104>) -> List<Adt104> {
    let m1767 = f334(v0, v1);
    match m1767 {
        Adt130::C0 => {
            v2
        }
        Adt130::C1(v3) => {
            List::cons(Adt104::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f493(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt104>) -> List<Adt104> {
    let m1768 = f334(v0, v1);
    match m1768 {
        Adt130::C0 => {
            v2
        }
        Adt130::C1(v3) => {
            let m1769 = f359(v3);
            match m1769 {
                None => {
                    v2
                }
                Some(v4) => {
                    f570(v4, v2)
                }
            }
        }
    }
}

pub fn f494(v0: Str) -> bool {
    let a1770 = v0.clone();
    let a1771 = Str::lit("null");
    if equal(a1770, a1771) {
        true
    } else {
        let a1772 = v0.clone();
        let a1773 = Str::lit("boolean");
        if equal(a1772, a1773) {
            true
        } else {
            let a1774 = v0.clone();
            let a1775 = Str::lit("object");
            if equal(a1774, a1775) {
                true
            } else {
                let a1776 = v0.clone();
                let a1777 = Str::lit("array");
                if equal(a1776, a1777) {
                    true
                } else {
                    let a1778 = v0.clone();
                    let a1779 = Str::lit("number");
                    if equal(a1778, a1779) {
                        true
                    } else {
                        let a1780 = v0.clone();
                        let a1781 = Str::lit("string");
                        if equal(a1780, a1781) {
                            true
                        } else {
                            let a1782 = v0;
                            let a1783 = Str::lit("integer");
                            equal(a1782, a1783)
                        }
                    }
                }
            }
        }
    }
}

pub fn f495(v0: Str, v1: Adt6) -> R<bool> {
    let a1784 = v0.clone();
    let a1785 = f571(v1.clone());
    if equal(a1784, a1785) {
        Ok(true)
    } else {
        let a1786 = v0;
        let a1787 = Str::lit("integer");
        if equal(a1786, a1787) {
            let m1788 = f499(v1);
            match m1788 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f572(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f496(p1791: List<Adt6>, v1: Adt6, p1792: bool) -> R<Adt136> {
    let m1790 = (p1791, p1792);
    {
        let mut m1790 = m1790;
        loop {
            let step = {
                let (v0, v2) = m1790;
                let m1789 = v0;
                match m1789.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f494(f573(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt136>>((v4, if v2 {
                                true
                            } else {
                                f495(f573(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C2))
                        }
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

pub fn f497(p1796: List<Adt6>, v1: Adt6) -> R<Adt136> {
    let m1795 = p1796;
    {
        let mut m1795 = m1795;
        loop {
            let step = {
                let v0 = m1795;
                let m1793 = v0;
                match m1793.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt136>>(Ok(Adt136::C1))
                    }
                    Some((v2, v3)) => {
                        let m1794 = f498(v2, v1.clone())?;
                        match m1794 {
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
                Ok(next) => { m1795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f574(f575(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f499(v0: Adt6) -> Option<Adt7> {
    let m1797 = v0;
    match m1797 {
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

pub fn f500(v0: u64, v1: Adt147) -> bool {
    let a1798 = v0;
    let a1799 = 0u64;
    if nat_eq(a1798, a1799) {
        let a1800 = f576(v1);
        bool_not(a1800)
    } else {
        let a1801 = v0;
        let a1802 = 1u64;
        if nat_eq(a1801, a1802) {
            let a1803 = f516(v1);
            bool_not(a1803)
        } else {
            let a1804 = v0;
            let a1805 = 2u64;
            if nat_eq(a1804, a1805) {
                f516(v1)
            } else {
                f576(v1)
            }
        }
    }
}

pub fn f501(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f577(v0.clone(), v1.clone(), f578(match v0 {
        Adt7::C0(h1806, _) => {
            h1806
        }
    }), f578(match v1 {
        Adt7::C0(h1807, _) => {
            h1807
        }
    }))
}

pub fn f502(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt136> {
    let m1808 = f334(v0, v2);
    match m1808 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v4) => {
            let m1809 = f579(v4);
            match m1809 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v5) => {
                    let m1812 = {
                        let a1810 = v5;
                        let a1811 = 0i64;
                        compare(a1810, a1811)
                    };
                    if match m1812 {
                        Ordering::Less => {
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
                            if f576(f580(f186(v3, 0i64)?, v5)) {
                                Ok(Adt136::C1)
                            } else {
                                Ok(Adt136::C0)
                            }
                        } else {
                            if f516(f580(f186(v3, 0i64)?, v5)) {
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

pub fn f503(v0: Str) -> R<u64> {
    f581(f153({
        let a1813 = v0;
        utf8_encode(a1813)
    })?, 0u64)
}

pub fn f504(p1818: List<Adt6>, p1819: u64) -> R<u64> {
    let m1817 = (p1818, p1819);
    {
        let mut m1817 = m1817;
        loop {
            let step = {
                let (v0, v1) = m1817;
                let m1814 = v0;
                match m1814.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1815 = v1;
                            let a1816 = 1u64;
                            nat_add(a1815, a1816)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(p1824: List<(Str, Adt6)>, p1825: u64) -> R<u64> {
    let m1823 = (p1824, p1825);
    {
        let mut m1823 = m1823;
        loop {
            let step = {
                let (v0, v1) = m1823;
                let m1820 = v0;
                match m1820.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1821 = v1;
                            let a1822 = 1u64;
                            nat_add(a1821, a1822)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(p1829: List<Adt6>, v1: Adt5) -> Adt136 {
    let m1828 = p1829;
    {
        let mut m1828 = m1828;
        loop {
            let step = {
                let v0 = m1828;
                let m1826 = v0;
                match m1826.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt136>(Adt136::C0)
                    }
                    Some((v2, v3)) => {
                        let m1827 = f331(v2);
                        match m1827 {
                            None => {
                                Err::<List<Adt6>, Adt136>(Adt136::C2)
                            }
                            Some(v4) => {
                                if f335(v4, v1.clone()) {
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
                Ok(next) => { m1828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1830 = v1.clone();
    let a1831 = Str::lit("#");
    if equal(a1830, a1831) {
        Some(v0)
    } else {
        let m1832 = f285(v0);
        match m1832 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1833 = f266(Str::lit("$defs"), v2);
                match m1833 {
                    Adt130::C0 => {
                        None::<Adt6>
                    }
                    Adt130::C1(v3) => {
                        let m1834 = f285(v3);
                        match m1834 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f582(v1, match v4 {
                                    Adt5::C0(h1835) => {
                                        h1835
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

pub fn f508(p1842: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt99>, p1843: List<Adt101>) -> List<Adt101> {
    let m1841 = (p1842, p1843);
    {
        let mut m1841 = m1841;
        loop {
            let step = {
                let (v0, v3) = m1841;
                let m1836 = v0;
                match m1836.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1838 = f334({
                            let (h1837, _) = v4.clone();
                            h1837
                        }, v1.clone());
                        match m1838 {
                            Adt130::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v5, v3))
                            }
                            Adt130::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v5, List::cons(Adt101::C0({
                                    let (_, h1839) = v4;
                                    h1839
                                }, v6, List::cons(Adt99::C0({
                                    let (h1840, _) = v4;
                                    h1840
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(p1848: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1847 = p1848;
    {
        let mut m1847 = m1847;
        loop {
            let step = {
                let v0 = m1847;
                let m1844 = v0;
                match m1844.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1846 = f334({
                            let (h1845, _) = v2;
                            h1845
                        }, v1.clone());
                        match m1846 {
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
                Ok(next) => { m1847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f510(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1849 = f334(Str::lit("properties"), v0);
    match m1849 {
        Adt130::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt130::C1(v1) => {
            let m1850 = f285(v1);
            match m1850 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1851) => {
                            h1851
                        }
                    }
                }
            }
        }
    }
}

pub fn f511(p1858: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>, p1859: List<Adt101>) -> List<Adt101> {
    let m1857 = (p1858, p1859);
    {
        let mut m1857 = m1857;
        loop {
            let step = {
                let (v0, v4) = m1857;
                let m1852 = v0;
                match m1852.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1854 = f334({
                            let (h1853, _) = v5.clone();
                            h1853
                        }, v1.clone());
                        match m1854 {
                            Adt130::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v6, List::cons(Adt101::C0(v2.clone(), {
                                    let (_, h1855) = v5;
                                    h1855
                                }, List::cons(Adt99::C0({
                                    let (h1856, _) = v5;
                                    h1856
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
                Ok(next) => { m1857 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(p1864: List<Adt6>, p1865: u64, v2: Adt6, v3: List<Adt99>, p1866: List<Adt101>) -> R<List<Adt101>> {
    let m1863 = (p1864, (p1865, p1866));
    {
        let mut m1863 = m1863;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1863;
                let m1860 = v0;
                match m1860.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt101>)), R<List<Adt101>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt101>)), R<List<Adt101>>>((v6, ({
                            let a1861 = v1;
                            let a1862 = 1u64;
                            nat_add(a1861, a1862)?
                        }, List::cons(Adt101::C0(v2.clone(), v5, List::cons(Adt99::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(p1869: List<Adt6>, v1: Adt6, v2: List<Adt99>, p1870: List<Adt101>) -> List<Adt101> {
    let m1868 = (p1869, p1870);
    {
        let mut m1868 = m1868;
        loop {
            let step = {
                let (v0, v3) = m1868;
                let m1867 = v0;
                match m1867.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt101>), List<Adt101>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt101>), List<Adt101>>((v5, List::cons(Adt101::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1871 = f583(v0)?;
    match m1871 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1872 = f583(v1)?;
            match m1872 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1873 = v2;
                    let a1874 = 61u64;
                    if nat_eq(a1873, a1874) {
                        f584(v5, v6, v4, v3)
                    } else {
                        let m1875 = f583(v2)?;
                        match m1875 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1876 = v3;
                                let a1877 = 61u64;
                                if nat_eq(a1876, a1877) {
                                    f585(v5, v6, v7, v4)
                                } else {
                                    let m1878 = f583(v3)?;
                                    match m1878 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f586(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f515(v0: List<u64>) -> bool {
    let m1879 = v0;
    match m1879.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f516(v0: Adt147) -> bool {
    let m1880 = v0;
    match m1880 {
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

pub fn f517() -> i64 {
    999999999999999999i64
}

pub fn f518(v0: i64) -> R<List<u64>> {
    let m1883 = {
        let a1881 = v0;
        let a1882 = 0i64;
        compare(a1881, a1882)
    };
    if match m1883 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f587(20u64, {
            let a1884 = v0;
            int_neg(a1884)?
        }, List::<u64>::nil())?))
    } else {
        f587(20u64, v0, List::<u64>::nil())
    }
}

pub fn f519(v0: u64) -> R<List<u64>> {
    let a1885 = v0;
    let a1886 = 34u64;
    if nat_eq(a1885, a1886) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1887 = v0;
        let a1888 = 92u64;
        if nat_eq(a1887, a1888) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1889 = v0;
            let a1890 = 8u64;
            if nat_eq(a1889, a1890) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1891 = v0;
                let a1892 = 9u64;
                if nat_eq(a1891, a1892) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1893 = v0;
                    let a1894 = 10u64;
                    if nat_eq(a1893, a1894) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1895 = v0;
                        let a1896 = 12u64;
                        if nat_eq(a1895, a1896) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1897 = v0;
                            let a1898 = 13u64;
                            if nat_eq(a1897, a1898) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1899 = v0;
                                let a1900 = 32u64;
                                if nat_lt(a1899, a1900) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f588(f565(v0, 16u64))?, List::cons(f588(f564(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f520(v0: List<Str>) -> Adt6 {
    Adt6::C4(f466(v0))
}

pub fn f521(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1918 = f548(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1901, _, _, _, _, _, _) => {
                h1901
            }
        });
        let a1919 = {
            let a1916 = f548(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1902, _, _, _, _, _) => {
                    h1902
                }
            });
            let a1917 = {
                let a1914 = f548(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1903, _, _, _, _) => {
                        h1903
                    }
                });
                let a1915 = {
                    let a1912 = f589(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1904, _, _, _) => {
                            h1904
                        }
                    });
                    let a1913 = {
                        let a1910 = f590(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1905, _, _) => {
                                h1905
                            }
                        });
                        let a1911 = {
                            let a1908 = f589(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1906, _) => {
                                    h1906
                                }
                            });
                            let a1909 = f548(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1907) => {
                                    h1907
                                }
                            });
                            append_list(a1908, a1909)
                        };
                        append_list(a1910, a1911)
                    };
                    append_list(a1912, a1913)
                };
                append_list(a1914, a1915)
            };
            append_list(a1916, a1917)
        };
        append_list(a1918, a1919)
    })
}

pub fn f522(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1920 = v1;
    match m1920 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f523(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f66(match v0.clone() {
        Adt10::C0(h1921, _) => {
            h1921
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f591(match v0 {
        Adt10::C0(_, h1922) => {
            h1922
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f524(v0: List<Adt35>) -> List<Adt6> {
    f396(f592(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f525(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1923 = f593(v0, List::<Adt6>::nil())?;
    match m1923 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f396(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f526(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1924 = v1;
    match m1924 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f527(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1925 = v1;
    match m1925 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f528(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f396(f594(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f529(v0: List<Adt46>) -> List<Adt6> {
    f396(f595(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f530(v0: List<Adt54>) -> List<Adt6> {
    f396(f596(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f531(v0: List<Adt56>) -> List<Adt6> {
    f396(f597(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f532(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m1926 = f598(v0, List::<Adt6>::nil())?;
    match m1926 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f396(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f533(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1927 = v1;
    match m1927 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f66(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f534() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f535(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1928, _, _, _, _, _) => {
            h1928
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1929, _, _, _) => {
            h1929
        }
    })), {
        let a1938 = f522(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1930, _, _, _, _) => {
                h1930
            }
        });
        let a1939 = {
            let a1936 = f522(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1931, _, _) => {
                    h1931
                }
            });
            let a1937 = {
                let a1934 = f522(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1932, _) => {
                        h1932
                    }
                });
                let a1935 = f599(match v0 {
                    Adt1::C0(_, _, _, _, _, h1933) => {
                        h1933
                    }
                });
                append_list(a1934, a1935)
            };
            append_list(a1936, a1937)
        };
        append_list(a1938, a1939)
    })))
}

pub fn f536(v0: List<List<u64>>) -> bool {
    let m1940 = v0;
    match m1940.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f537() -> List<Adt86> {
    List::cons(Adt86::C0(f600(), f469()), List::cons(Adt86::C0(f601(), f602()), List::cons(Adt86::C0(f603(), f604()), List::<Adt86>::nil())))
}

pub fn f538(v0: List<List<u64>>) -> List<u64> {
    f605(f52(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f539(p1955: u64, p1956: Adt144, p1957: List<Adt145>) -> R<Option<Adt6>> {
    let m1954 = (p1955, (p1956, p1957));
    {
        let mut m1954 = m1954;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1954;
                let m1941 = v0;
                if m1941 == 0 {
                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1941 - 1;
                    let m1942 = v1;
                    match m1942 {
                        Adt144::C0(v4) => {
                            let m1943 = v4.clone();
                            match m1943 {
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
                                    let v8: Str = f606(v7)?;
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1944 = v9;
                                    match m1944.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v10), List::cons(Adt145::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1945 = v12;
                                    match m1945.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f606({
                                                let (h1946, _) = v13;
                                                h1946
                                            })?;
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                let (_, h1947) = v13;
                                                h1947
                                            }), List::cons(Adt145::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt144::C1(v16) => {
                            let m1948 = v2;
                            match m1948.uncons() {
                                None => {
                                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1949 = v17;
                                    match m1949 {
                                        Adt145::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1950 = v20;
                                            match m1950.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C4(f396(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v22), List::cons(Adt145::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt145::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1951 = v26;
                                            match m1951.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C5(f398(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f606({
                                                        let (h1952, _) = v28;
                                                        h1952
                                                    })?;
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                        let (_, h1953) = v28;
                                                        h1953
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
                Ok(next) => { m1954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540() -> u64 {
    67108864u64
}

pub fn f541(v0: Adt19) -> Adt6 {
    let m1958 = v0;
    match m1958 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f542(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1959 = v0;
    match m1959 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f543(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1960 = v0;
    match m1960 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f544() -> i64 {
    -32603i64
}

pub fn f545() -> i64 {
    429i64
}

pub fn f546() -> i64 {
    -32021i64
}

pub fn f547(p1963: List<Str>, p1964: List<Adt6>) -> List<Adt6> {
    let m1962 = (p1963, p1964);
    {
        let mut m1962 = m1962;
        loop {
            let step = {
                let (v0, v1) = m1962;
                let m1961 = v0;
                match m1961.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f548(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1965 = v1;
    match m1965 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1966) => {
                    h1966
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f549(v0: List<u64>) -> R<bool> {
    let m1967 = v0;
    match m1967.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1968 = v2;
            match m1968.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1969 = f274(v1)?;
                    let a1970 = 113u64;
                    if if nat_eq(a1969, a1970) {
                        let a1971 = v3;
                        let a1972 = 61u64;
                        nat_eq(a1971, a1972)
                    } else {
                        false
                    } {
                        Ok(f607(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f550(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt127>) -> R<Result<Adt123, Adt126>> {
    let m1973 = v3;
    match m1973 {
        Ok(v5) => {
            f608(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt123, Adt126>(Adt126::C0(v4)))
        }
    }
}

pub fn f551(v0: Adt134, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt127> {
    let m1974 = v0;
    match m1974 {
        Adt134::C0 => {
            f609(v1, v2)
        }
        Adt134::C1 => {
            f610(v1, v2)
        }
        Adt134::C2 => {
            f611(v1, v2)
        }
        Adt134::C3 => {
            f612(v1, v2)
        }
        Adt134::C4 => {
            f613(v1, v2)
        }
        Adt134::C5 => {
            f614(v1, v2)
        }
        Adt134::C6 => {
            f615(v1, v2)
        }
        Adt134::C7 => {
            f616(v1, v2)
        }
        Adt134::C8 => {
            f617(v1, v2)
        }
        Adt134::C9 => {
            f618(v1, v2)
        }
        Adt134::C10 => {
            f619(v1, v2)
        }
        Adt134::C11 => {
            f620(v1, v2)
        }
    }
}

pub fn f552(v0: Adt134) -> Str {
    let m1975 = v0;
    match m1975 {
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

pub fn f553(v0: Option<Adt8>) -> bool {
    let m1976 = v0;
    match m1976 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f554(v0: Option<Adt9>) -> bool {
    let m1977 = v0;
    match m1977 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f555(v0: Option<Adt5>) -> bool {
    let m1978 = v0;
    match m1978 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f556(v0: Option<Adt5>, v1: Str) -> bool {
    let m1979 = v0;
    match m1979 {
        None => {
            false
        }
        Some(v2) => {
            f335(v1, v2)
        }
    }
}

pub fn f557() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f558(v0: List<Str>, v1: Adt103, v2: Adt16, v3: Option<Adt108>) -> Adt138 {
    let m1980 = v3;
    match m1980 {
        None => {
            f621(v1, v2.clone(), f622(match v2 {
                Adt16::C0(h1981, _, _, _, _, _) => {
                    h1981
                }
            }, v0))
        }
        Some(v4) => {
            Adt138::C1(Adt126::C2(v4))
        }
    }
}

pub fn f559(v0: Option<Adt107>, v1: Adt140, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt108>> {
    let m1982 = v0;
    match m1982 {
        None => {
            Ok(None::<Adt108>)
        }
        Some(v5) => {
            let m1984 = match v5.clone() {
                Adt107::C0(_, _, _, h1983, _) => {
                    h1983
                }
            };
            match m1984 {
                None => {
                    f623(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f560(v0: Option<Adt5>) -> Adt5 {
    let m1985 = v0;
    match m1985 {
        None => {
            f26()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f561(v0: Adt5) -> Result<Adt16, Adt127> {
    let m1986 = f624(v0.clone());
    match m1986 {
        Adt148::C0 => {
            f625(v0.clone(), f266(f626(), v0.clone()), f266(f627(), v0))
        }
        Adt148::C1 => {
            Err::<Adt16, Adt127>(Adt127::C2)
        }
        Adt148::C2 => {
            Err::<Adt16, Adt127>(Adt127::C3)
        }
    }
}

pub fn f562() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f563(v0: Str) -> R<List<u64>> {
    f153({
        let a1987 = v0;
        utf8_encode(a1987)
    })
}

pub fn f564(v0: u64, v1: u64) -> u64 {
    let a1988 = v0;
    let a1989 = v1;
    let a1990 = 0u64;
    nat_rem(a1988, a1989, a1990)
}

pub fn f565(v0: u64, v1: u64) -> u64 {
    let a1991 = v0;
    let a1992 = v1;
    let a1993 = 0u64;
    nat_quot(a1991, a1992, a1993)
}

pub fn f566(v0: u64, v1: Adt6) -> bool {
    let a1994 = v0;
    let a1995 = 0u64;
    if nat_eq(a1994, a1995) {
        f628(v1)
    } else {
        let a1996 = v0;
        let a1997 = 1u64;
        if nat_eq(a1996, a1997) {
            f629(v1)
        } else {
            let a1998 = v0;
            let a1999 = 2u64;
            if nat_eq(a1998, a1999) {
                f630(v1)
            } else {
                let a2000 = v0;
                let a2001 = 3u64;
                if nat_eq(a2000, a2001) {
                    f631(v1)
                } else {
                    let a2002 = v0;
                    let a2003 = 4u64;
                    if nat_eq(a2002, a2003) {
                        f632(v1)
                    } else {
                        let a2004 = v0;
                        let a2005 = 5u64;
                        if nat_eq(a2004, a2005) {
                            f633(v1)
                        } else {
                            let a2006 = v0;
                            let a2007 = 6u64;
                            if nat_eq(a2006, a2007) {
                                f634(v1)
                            } else {
                                let a2008 = v0;
                                let a2009 = 7u64;
                                if nat_eq(a2008, a2009) {
                                    f635(v1)
                                } else {
                                    f636(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f567(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt120) -> R<Adt142> {
    let m2010 = v3.clone();
    match m2010.uncons() {
        None => {
            Ok(Adt142::C0(Adt91::C8(v0)))
        }
        Some((_, _)) => {
            if f278(v3.clone(), true) {
                let m2011 = f637(v1);
                match m2011 {
                    None => {
                        Ok(Adt142::C0(Adt91::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f638(v3)?;
                        if f639(v9.clone(), match v5.clone() {
                            Adt120::C0(_, h2012) => {
                                h2012
                            }
                        }) {
                            Ok(Adt142::C0(Adt91::C12(v0, v2)))
                        } else {
                            Ok(Adt142::C1(Adt120::C0(List::cons(Adt12::C0(v0, v2, f199(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt120::C0(h2013, _) => {
                                    h2013
                                }
                            }), List::cons(v9, match v5 {
                                Adt120::C0(_, h2014) => {
                                    h2014
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

pub fn f568(p2019: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2020: List<Adt104>) -> List<Adt104> {
    let m2018 = (p2019, p2020);
    {
        let mut m2018 = m2018;
        loop {
            let step = {
                let (v0, v3) = m2018;
                let m2015 = v0;
                match m2015.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v5, List::cons(Adt104::C0({
                            let (_, h2016) = v4;
                            h2016
                        }, v1, List::cons({
                            let (h2017, _) = v4;
                            h2017
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2018 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(p2024: List<(Str, Adt6)>, p2025: List<Adt104>) -> List<Adt104> {
    let m2023 = (p2024, p2025);
    {
        let mut m2023 = m2023;
        loop {
            let step = {
                let (v0, v1) = m2023;
                let m2021 = v0;
                match m2021.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0({
                            let (_, h2022) = v2;
                            h2022
                        }, false, List::<Str>::nil()), v1)))
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

pub fn f570(p2028: List<Adt6>, p2029: List<Adt104>) -> List<Adt104> {
    let m2027 = (p2028, p2029);
    {
        let mut m2027 = m2027;
        loop {
            let step = {
                let (v0, v1) = m2027;
                let m2026 = v0;
                match m2026.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f571(v0: Adt6) -> Str {
    let m2030 = v0;
    match m2030 {
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

pub fn f572(v0: Adt7) -> R<bool> {
    if f481(match v0.clone() {
        Adt7::C0(h2031, _) => {
            h2031
        }
    }) {
        Ok(true)
    } else {
        f640(40u64, match v0.clone() {
            Adt7::C0(h2032, _) => {
                h2032
            }
        }, match v0 {
            Adt7::C0(_, h2033) => {
                h2033
            }
        })
    }
}

pub fn f573(v0: Adt6) -> Str {
    let m2034 = f331(v0);
    match m2034 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f574(p2041: u64, p2042: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m2040 = (p2041, p2042);
    {
        let mut m2040 = m2040;
        loop {
            let step = {
                let (v0, v1) = m2040;
                let m2035 = v0;
                if m2035 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m2035 - 1;
                    let m2036 = v1;
                    match m2036.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m2039 = f641({
                                let (h2037, _) = v3;
                                h2037
                            }, {
                                let (_, h2038) = v3;
                                h2038
                            }, v4)?;
                            match m2039 {
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
                Ok(next) => { m2040 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f575() -> u64 {
    67108864u64
}

pub fn f576(v0: Adt147) -> bool {
    let m2043 = v0;
    match m2043 {
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

pub fn f577(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m2044 = v2;
    match m2044 {
        Adt147::C0 => {
            let m2046 = v3;
            match m2046 {
                Adt147::C0 => {
                    Ok(f642(Adt147::C0, f643(f644(match v0.clone() {
                        Adt7::C0(h2047, _) => {
                            h2047
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2048) => {
                            h2048
                        }
                    }, f644(match v1.clone() {
                        Adt7::C0(h2049, _) => {
                            h2049
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2050) => {
                            h2050
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
            let m2045 = v3;
            match m2045 {
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
            let m2051 = v3;
            match m2051 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f643(match v0.clone() {
                        Adt7::C0(h2052, _) => {
                            h2052
                        }
                    }, match v0 {
                        Adt7::C0(_, h2053) => {
                            h2053
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2054, _) => {
                            h2054
                        }
                    }, match v1 {
                        Adt7::C0(_, h2055) => {
                            h2055
                        }
                    })
                }
            }
        }
    }
}

pub fn f578(v0: i64) -> Adt147 {
    f580(v0, 0i64)
}

pub fn f579(v0: Adt6) -> Option<i64> {
    let m2056 = v0;
    match m2056 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f481(match v2.clone() {
                Adt7::C0(_, h2057) => {
                    h2057
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2058, _) => {
                        h2058
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

pub fn f580(v0: i64, v1: i64) -> Adt147 {
    let m2061 = {
        let a2059 = v0;
        let a2060 = v1;
        compare(a2059, a2060)
    };
    if match m2061 {
        Ordering::Less => {
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
        let m2064 = {
            let a2062 = v1;
            let a2063 = v0;
            compare(a2062, a2063)
        };
        if match m2064 {
            Ordering::Less => {
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

pub fn f581(p2073: List<u64>, p2074: u64) -> R<u64> {
    let m2072 = (p2073, p2074);
    {
        let mut m2072 = m2072;
        loop {
            let step = {
                let (v0, v1) = m2072;
                let m2065 = v0;
                match m2065.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2066 = v2;
                        let a2067 = 128u64;
                        if if nat_lt(a2066, a2067) {
                            true
                        } else {
                            let a2068 = 191u64;
                            let a2069 = v2;
                            nat_lt(a2068, a2069)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2070 = v1;
                                let a2071 = 1u64;
                                nat_add(a2070, a2071)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
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

pub fn f582(v0: Str, p2092: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2091 = p2092;
    {
        let mut m2091 = m2091;
        loop {
            let step = {
                let v1 = m2091;
                let m2075 = v1;
                match m2075.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2079 = v0.clone();
                        let a2080 = {
                            let a2077 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2076, _) = v2.clone();
                                h2076
                            }, List::<Str>::nil()));
                            let a2078 = Str::lit("");
                            join(a2077, a2078)
                        };
                        if equal(a2079, a2080) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2081) = v2;
                                h2081
                            }))
                        } else {
                            let m2083 = f285({
                                let (_, h2082) = v2.clone();
                                h2082
                            });
                            match m2083 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2084 = f266(Str::lit("$anchor"), v4);
                                    match m2084 {
                                        Adt130::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt130::C1(v5) => {
                                            let m2085 = f331(v5);
                                            match m2085 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2088 = v0.clone();
                                                    let a2089 = {
                                                        let a2086 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2087 = Str::lit("");
                                                        join(a2086, a2087)
                                                    };
                                                    if equal(a2088, a2089) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2090) = v2;
                                                            h2090
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
                Ok(next) => { m2091 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: u64) -> R<Option<u64>> {
    let a2093 = 64u64;
    let a2094 = v0;
    if if nat_lt(a2093, a2094) {
        let a2095 = v0;
        let a2096 = 91u64;
        nat_lt(a2095, a2096)
    } else {
        false
    } {
        Ok(Some({
            let a2097 = v0;
            let a2098 = 65u64;
            nat_sub(a2097, a2098)
        }))
    } else {
        let a2099 = 96u64;
        let a2100 = v0;
        if if nat_lt(a2099, a2100) {
            let a2101 = v0;
            let a2102 = 123u64;
            nat_lt(a2101, a2102)
        } else {
            false
        } {
            Ok(Some({
                let a2103 = v0;
                let a2104 = 71u64;
                nat_sub(a2103, a2104)
            }))
        } else {
            let a2105 = 47u64;
            let a2106 = v0;
            if if nat_lt(a2105, a2106) {
                let a2107 = v0;
                let a2108 = 58u64;
                nat_lt(a2107, a2108)
            } else {
                false
            } {
                Ok(Some({
                    let a2109 = v0;
                    let a2110 = 4u64;
                    nat_add(a2109, a2110)?
                }))
            } else {
                let a2111 = v0;
                let a2112 = 43u64;
                if nat_eq(a2111, a2112) {
                    Ok(Some(62u64))
                } else {
                    let a2113 = v0;
                    let a2114 = 47u64;
                    if nat_eq(a2113, a2114) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f584(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2115 = v3;
        let a2116 = 61u64;
        if nat_eq(a2115, a2116) {
            let a2117 = f645(v1, 16u64);
            let a2118 = 0u64;
            nat_eq(a2117, a2118)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f646(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f585(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2119 = f645(v2, 4u64);
        let a2120 = 0u64;
        nat_eq(a2119, a2120)
    } else {
        false
    } {
        Ok(Some(f647(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f586(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2123 = {
            let a2121 = v0;
            let a2122 = 4u64;
            nat_mul(a2121, a2122)?
        };
        let a2124 = f648(v1, 16u64);
        nat_add(a2123, a2124)?
    }, List::cons({
        let a2127 = {
            let a2125 = f645(v1, 16u64);
            let a2126 = 16u64;
            nat_mul(a2125, a2126)?
        };
        let a2128 = f648(v2, 4u64);
        nat_add(a2127, a2128)?
    }, List::cons({
        let a2131 = {
            let a2129 = f645(v2, 4u64);
            let a2130 = 64u64;
            nat_mul(a2129, a2130)?
        };
        let a2132 = v3;
        nat_add(a2131, a2132)?
    }, List::<u64>::nil()))))
}

pub fn f587(p2144: u64, p2145: i64, p2146: List<u64>) -> R<List<u64>> {
    let m2143 = (p2144, (p2145, p2146));
    {
        let mut m2143 = m2143;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2143;
                let m2133 = v0;
                if m2133 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2133 - 1;
                    let v4: i64 = {
                        let a2134 = v1;
                        let a2135 = 10i64;
                        let a2136 = 0i64;
                        int_quot(a2134, a2135, a2136)?
                    };
                    let v5: i64 = {
                        let a2137 = v1;
                        let a2138 = 10i64;
                        let a2139 = 0i64;
                        int_rem(a2137, a2138, a2139)
                    };
                    let m2142 = {
                        let a2140 = v4;
                        let a2141 = 1i64;
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f649(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f649(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: u64) -> R<u64> {
    let a2147 = v0;
    let a2148 = 10u64;
    if nat_lt(a2147, a2148) {
        let a2149 = 48u64;
        let a2150 = v0;
        nat_add(a2149, a2150)
    } else {
        let a2151 = 87u64;
        let a2152 = v0;
        nat_add(a2151, a2152)
    }
}

pub fn f589(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2153 = v1;
    match m2153 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f650(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f590(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2154 = v0;
    match m2154 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2157 = f527(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2155, _) => {
                        h2155
                    }
                });
                let a2158 = f527(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2156) => {
                        h2156
                    }
                });
                append_list(a2157, a2158)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f591(v0: Adt11) -> Str {
    let m2159 = v0;
    match m2159 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f592(p2162: List<Adt35>, p2163: List<Adt6>) -> List<Adt6> {
    let m2161 = (p2162, p2163);
    {
        let mut m2161 = m2161;
        loop {
            let step = {
                let (v0, v1) = m2161;
                let m2160 = v0;
                match m2160.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f651(v2), v1)))
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

pub fn f593(p2167: List<Adt38>, p2168: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2166 = (p2167, p2168);
    {
        let mut m2166 = m2166;
        loop {
            let step = {
                let (v0, v1) = m2166;
                let m2164 = v0;
                match m2164.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2165 = f652(v2)?;
                        match m2165 {
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
                Ok(next) => { m2166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(p2171: List<Adt44>, p2172: List<Adt6>) -> R<List<Adt6>> {
    let m2170 = (p2171, p2172);
    {
        let mut m2170 = m2170;
        loop {
            let step = {
                let (v0, v1) = m2170;
                let m2169 = v0;
                match m2169.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f653(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(p2175: List<Adt46>, p2176: List<Adt6>) -> List<Adt6> {
    let m2174 = (p2175, p2176);
    {
        let mut m2174 = m2174;
        loop {
            let step = {
                let (v0, v1) = m2174;
                let m2173 = v0;
                match m2173.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f654(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(p2179: List<Adt54>, p2180: List<Adt6>) -> List<Adt6> {
    let m2178 = (p2179, p2180);
    {
        let mut m2178 = m2178;
        loop {
            let step = {
                let (v0, v1) = m2178;
                let m2177 = v0;
                match m2177.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f655(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(p2183: List<Adt56>, p2184: List<Adt6>) -> List<Adt6> {
    let m2182 = (p2183, p2184);
    {
        let mut m2182 = m2182;
        loop {
            let step = {
                let (v0, v1) = m2182;
                let m2181 = v0;
                match m2181.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f656(v2), v1)))
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

pub fn f598(p2188: List<Adt59>, p2189: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2187 = (p2188, p2189);
    {
        let mut m2187 = m2187;
        loop {
            let step = {
                let (v0, v1) = m2187;
                let m2185 = v0;
                match m2185.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2186 = f657(v2)?;
                        match m2186 {
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
                Ok(next) => { m2187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f599(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2190 = v0;
    match m2190 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f658(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f600() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f601() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f602() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f603() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f604() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f605(p2195: List<List<u64>>, p2196: List<u64>) -> List<u64> {
    let m2194 = (p2195, p2196);
    {
        let mut m2194 = m2194;
        loop {
            let step = {
                let (v0, v1) = m2194;
                let m2191 = v0;
                match m2191.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2192 = f659(v2);
                            let a2193 = v1;
                            append_list(a2192, a2193)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606(v0: Str) -> R<Str> {
    let m2199 = {
        let a2198 = f246(f247(), f253(f660(f153({
            let a2197 = v0;
            utf8_encode(a2197)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2198)
    };
    match m2199 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f607(v0: List<u64>) -> bool {
    let m2200 = v0;
    match m2200.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2201 = v1;
            let a2202 = 48u64;
            if nat_eq(a2201, a2202) {
                f661(v2)
            } else {
                false
            }
        }
    }
}

pub fn f608(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt123, Adt126>> {
    if f662(f663(v3.clone())) {
        Ok(Ok::<Adt123, Adt126>(f664(v0.clone(), Adt32::C0(f665(v0)))))
    } else {
        Ok(f666(v0.clone(), v3.clone(), f667(match v0 {
            Adt0::C0(_, _, _, _, _, h2203) => {
                h2203
            }
        }, v2, v1, v3)?))
    }
}

pub fn f609(v0: Adt16, _: Adt5) -> Result<Adt14, Adt127> {
    Ok::<Adt14, Adt127>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f610(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2204 = f668(Str::lit("cursor"), v1);
    match m2204 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f611(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2205 = f669(Str::lit("name"), v1.clone());
    match m2205 {
        Ok(v2) => {
            let m2206 = f670(Str::lit("arguments"), v1.clone());
            match m2206 {
                Ok(v3) => {
                    let m2207 = f670(Str::lit("inputResponses"), v1.clone());
                    match m2207 {
                        Ok(v4) => {
                            let m2208 = f668(Str::lit("requestState"), v1);
                            match m2208 {
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

pub fn f612(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2209 = f668(Str::lit("cursor"), v1);
    match m2209 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f613(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2210 = f669(Str::lit("uri"), v1.clone());
    match m2210 {
        Ok(v2) => {
            let m2211 = f670(Str::lit("inputResponses"), v1.clone());
            match m2211 {
                Ok(v3) => {
                    let m2212 = f668(Str::lit("requestState"), v1);
                    match m2212 {
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

pub fn f614(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2213 = f668(Str::lit("cursor"), v1);
    match m2213 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f615(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2214 = f668(Str::lit("cursor"), v1);
    match m2214 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f616(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2215 = f669(Str::lit("name"), v1.clone());
    match m2215 {
        Ok(v2) => {
            let m2216 = f671(Str::lit("arguments"), v1.clone());
            match m2216 {
                Ok(v3) => {
                    let m2217 = f670(Str::lit("inputResponses"), v1.clone());
                    match m2217 {
                        Ok(v4) => {
                            let m2218 = f668(Str::lit("requestState"), v1);
                            match m2218 {
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

pub fn f617(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2219 = f672(Str::lit("ref"), v1.clone());
    match m2219 {
        Ok(v2) => {
            let m2220 = f673(v2);
            match m2220 {
                Ok(v3) => {
                    let m2221 = f672(Str::lit("argument"), v1.clone());
                    match m2221 {
                        Ok(v4) => {
                            let m2222 = f674(v4);
                            match m2222 {
                                Ok(v5) => {
                                    let m2223 = f675(v1);
                                    match m2223 {
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

pub fn f618(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2224 = f669(Str::lit("taskId"), v1);
    match m2224 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f619(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2225 = f669(Str::lit("taskId"), v1.clone());
    match m2225 {
        Ok(v2) => {
            let m2226 = f672(Str::lit("inputResponses"), v1);
            match m2226 {
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

pub fn f620(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m2227 = f669(Str::lit("taskId"), v1);
    match m2227 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f621(v0: Adt103, v1: Adt16, v2: Option<Adt126>) -> Adt138 {
    let m2228 = v2;
    match m2228 {
        None => {
            Adt138::C0(Adt139::C0(match v0.clone() {
                Adt103::C0(h2229, _, _) => {
                    h2229
                }
            }, match v0.clone() {
                Adt103::C0(_, h2230, _) => {
                    h2230
                }
            }, v1, f560(match v0 {
                Adt103::C0(_, _, h2231) => {
                    h2231
                }
            })))
        }
        Some(v3) => {
            Adt138::C1(v3)
        }
    }
}

pub fn f622(v0: Str, v1: List<Str>) -> Option<Adt126> {
    f676(f677(v0.clone(), v1.clone()), v0, v1)
}

pub fn f623(v0: Adt107, v1: Adt140, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt108>> {
    let m2233 = match v0.clone() {
        Adt107::C0(h2232, _, _, _, _) => {
            h2232
        }
    };
    match m2233 {
        None => {
            Ok(Some(Adt108::C0))
        }
        Some(v5) => {
            let a2234 = v5;
            let a2235 = v3;
            if equal(a2234, a2235) {
                let m2237 = match v0.clone() {
                    Adt107::C0(_, h2236, _, _, _) => {
                        h2236
                    }
                };
                match m2237 {
                    None => {
                        Ok(Some(Adt108::C1))
                    }
                    Some(v6) => {
                        let a2238 = v6;
                        let a2239 = v2;
                        if equal(a2238, a2239) {
                            f678(f679(match v0.clone() {
                                Adt107::C0(_, _, h2240, _, _) => {
                                    h2240
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt107::C0(_, _, _, _, h2241) => {
                                    h2241
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

pub fn f624(v0: Adt5) -> Adt148 {
    f680(f266(f626(), v0.clone()), f266(f627(), v0))
}

pub fn f625(v0: Adt5, v1: Adt130, v2: Adt130) -> Result<Adt16, Adt127> {
    let m2242 = v1;
    match m2242 {
        Adt130::C0 => {
            Err::<Adt16, Adt127>(Adt127::C2)
        }
        Adt130::C1(v3) => {
            let m2243 = v2;
            match m2243 {
                Adt130::C0 => {
                    Err::<Adt16, Adt127>(Adt127::C3)
                }
                Adt130::C1(v4) => {
                    f681(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f626() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f627() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f628(v0: Adt6) -> bool {
    let m2244 = f331(v0);
    match m2244 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f629(v0: Adt6) -> bool {
    let m2245 = f499(v0);
    match m2245 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f630(v0: Adt6) -> bool {
    let m2246 = f579(v0);
    match m2246 {
        None => {
            false
        }
        Some(v1) => {
            let a2250 = {
                let m2249 = {
                    let a2247 = v1;
                    let a2248 = 0i64;
                    compare(a2247, a2248)
                };
                match m2249 {
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
            bool_not(a2250)
        }
    }
}

pub fn f631(v0: Adt6) -> bool {
    let m2251 = f359(v0);
    match m2251 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f632(v0: Adt6) -> bool {
    let m2252 = f285(v0);
    match m2252 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f633(v0: Adt6) -> bool {
    let m2253 = f359(v0);
    match m2253 {
        None => {
            false
        }
        Some(v1) => {
            f682(v1)
        }
    }
}

pub fn f634(v0: Adt6) -> bool {
    let m2254 = f284(v0);
    match m2254 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f635(v0: Adt6) -> bool {
    let m2255 = f331(v0.clone());
    match m2255 {
        None => {
            let m2256 = f359(v0);
            match m2256 {
                None => {
                    false
                }
                Some(v2) => {
                    f683(v2)
                }
            }
        }
        Some(v1) => {
            f494(v1)
        }
    }
}

pub fn f636(v0: Adt6) -> bool {
    let m2257 = f359(v0);
    match m2257 {
        None => {
            false
        }
        Some(v1) => {
            let m2258 = v1;
            match m2258.uncons() {
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

pub fn f637(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2259 = f334(Str::lit("type"), v0);
    match m2259 {
        Adt130::C0 => {
            None::<Adt13>
        }
        Adt130::C1(v1) => {
            let m2260 = f331(v1);
            match m2260 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f684(v2)
                }
            }
        }
    }
}

pub fn f638(v0: List<u64>) -> R<List<u64>> {
    Ok(f253(f685(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f639(v0: List<u64>, p2263: List<List<u64>>) -> bool {
    let m2262 = p2263;
    {
        let mut m2262 = m2262;
        loop {
            let step = {
                let v1 = m2262;
                let m2261 = v1;
                match m2261.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f686(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
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

pub fn f640(p2275: u64, p2276: i64, p2277: i64) -> R<bool> {
    let m2274 = (p2275, (p2276, p2277));
    {
        let mut m2274 = m2274;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2274;
                let m2264 = v0;
                if m2264 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2264 - 1;
                    let m2267 = {
                        let a2265 = v2;
                        let a2266 = 0i64;
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
                        if f481({
                            let a2268 = v1;
                            let a2269 = 10i64;
                            let a2270 = 0i64;
                            int_rem(a2268, a2269, a2270)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2271 = v1;
                                let a2272 = 10i64;
                                let a2273 = 0i64;
                                int_quot(a2271, a2272, a2273)?
                            }, f244(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f641(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2278 = f687(v0.clone());
    let a2279 = f687(v1.clone());
    if nat_eq(a2278, a2279) {
        let m2280 = v0;
        match m2280 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2281 = f284(v1);
                match m2281 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2282 = v3;
                        let a2283 = v4;
                        if equal(a2282, a2283) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2284 = f499(v1);
                match m2284 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f688(f501(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2285 = f331(v1);
                match m2285 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2286 = v7;
                        let a2287 = v8;
                        if equal(a2286, a2287) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2288 = f359(v1);
                match m2288 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f689(f690(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2289 = f285(v1);
                match m2289 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2291 = f505(v11.clone(), 0u64)?;
                        let a2292 = f505(match v12.clone() {
                            Adt5::C0(h2290) => {
                                h2290
                            }
                        }, 0u64)?;
                        if nat_eq(a2291, a2292) {
                            Ok(f689(f691(v11, match v12 {
                                Adt5::C0(h2293) => {
                                    h2293
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

pub fn f642(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2294 = v0;
    match m2294 {
        Adt147::C0 => {
            f692(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f643(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2295 = f580(f693(v0, v1)?, f693(v2, v3)?);
    match m2295 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f694(v0, f695(v0)?, v2, f695(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f644(v0: i64) -> R<i64> {
    let m2298 = {
        let a2296 = v0;
        let a2297 = 0i64;
        compare(a2296, a2297)
    };
    if match m2298 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2299 = v0;
        int_neg(a2299)
    } else {
        Ok(v0)
    }
}

pub fn f645(v0: u64, v1: u64) -> u64 {
    let a2300 = v0;
    let a2301 = v1;
    let a2302 = 0u64;
    nat_rem(a2300, a2301, a2302)
}

pub fn f646(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2305 = {
            let a2303 = v0;
            let a2304 = 4u64;
            nat_mul(a2303, a2304)?
        };
        let a2306 = f648(v1, 16u64);
        nat_add(a2305, a2306)?
    }, List::<u64>::nil()))
}

pub fn f647(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2309 = {
            let a2307 = v0;
            let a2308 = 4u64;
            nat_mul(a2307, a2308)?
        };
        let a2310 = f648(v1, 16u64);
        nat_add(a2309, a2310)?
    }, List::cons({
        let a2313 = {
            let a2311 = f645(v1, 16u64);
            let a2312 = 16u64;
            nat_mul(a2311, a2312)?
        };
        let a2314 = f648(v2, 4u64);
        nat_add(a2313, a2314)?
    }, List::<u64>::nil())))
}

pub fn f648(v0: u64, v1: u64) -> u64 {
    let a2315 = v0;
    let a2316 = v1;
    let a2317 = 0u64;
    nat_quot(a2315, a2316, a2317)
}

pub fn f649(v0: i64) -> u64 {
    let m2320 = {
        let a2318 = v0;
        let a2319 = 1i64;
        compare(a2318, a2319)
    };
    if match m2320 {
        Ordering::Less => {
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
        let m2323 = {
            let a2321 = v0;
            let a2322 = 2i64;
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
            49u64
        } else {
            let m2326 = {
                let a2324 = v0;
                let a2325 = 3i64;
                compare(a2324, a2325)
            };
            if match m2326 {
                Ordering::Less => {
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
                let m2329 = {
                    let a2327 = v0;
                    let a2328 = 4i64;
                    compare(a2327, a2328)
                };
                if match m2329 {
                    Ordering::Less => {
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
                    let m2332 = {
                        let a2330 = v0;
                        let a2331 = 5i64;
                        compare(a2330, a2331)
                    };
                    if match m2332 {
                        Ordering::Less => {
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
                        let m2335 = {
                            let a2333 = v0;
                            let a2334 = 6i64;
                            compare(a2333, a2334)
                        };
                        if match m2335 {
                            Ordering::Less => {
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
                            let m2338 = {
                                let a2336 = v0;
                                let a2337 = 7i64;
                                compare(a2336, a2337)
                            };
                            if match m2338 {
                                Ordering::Less => {
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
                                let m2341 = {
                                    let a2339 = v0;
                                    let a2340 = 8i64;
                                    compare(a2339, a2340)
                                };
                                if match m2341 {
                                    Ordering::Less => {
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
                                    let m2344 = {
                                        let a2342 = v0;
                                        let a2343 = 9i64;
                                        compare(a2342, a2343)
                                    };
                                    if match m2344 {
                                        Ordering::Less => {
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

pub fn f650(v0: Adt8) -> Adt6 {
    Adt6::C5(f527(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2345) => {
            h2345
        }
    }))
}

pub fn f651(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2346, _, _, _, _, _, _, _) => {
            h2346
        }
    })), {
        let a2364 = f522(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2347, _, _, _, _, _, _) => {
                h2347
            }
        });
        let a2365 = {
            let a2362 = f599(match v0.clone() {
                Adt35::C0(_, _, h2348, _, _, _, _, _) => {
                    h2348
                }
            });
            let a2363 = {
                let a2360 = f522(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2349, _, _, _, _) => {
                        h2349
                    }
                });
                let a2361 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2352 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2351, _, _, _) => {
                            h2351
                        }
                    };
                    match m2352 {
                        Adt5::C0(h2350) => {
                            h2350
                        }
                    }
                })), {
                    let a2358 = f548(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2353, _, _) => {
                            h2353
                        }
                    });
                    let a2359 = {
                        let a2356 = f696(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2354, _) => {
                                h2354
                            }
                        });
                        let a2357 = f697(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2355) => {
                                h2355
                            }
                        });
                        append_list(a2356, a2357)
                    };
                    append_list(a2358, a2359)
                });
                append_list(a2360, a2361)
            };
            append_list(a2362, a2363)
        };
        append_list(a2364, a2365)
    }))
}

pub fn f652(v0: Adt38) -> R<Option<Adt6>> {
    let m2366 = v0;
    match m2366 {
        Adt38::C0(v1) => {
            Ok(Some(f698(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f699(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2367, _, _, _) => {
                    h2367
                }
            }, match v2.clone() {
                Adt42::C0(_, h2368, _, _) => {
                    h2368
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2369, _) => {
                    h2369
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2370) => {
                    h2370
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f699(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2371, _, _, _) => {
                    h2371
                }
            }, match v3.clone() {
                Adt43::C0(_, h2372, _, _) => {
                    h2372
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2373, _) => {
                    h2373
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2374) => {
                    h2374
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f700(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f701(v5)))
        }
    }
}

pub fn f653(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2398 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2375, _, _, _, _, _, _, _, _) => {
                h2375
            }
        })), List::<(Str, Adt6)>::nil());
        let a2399 = {
            let a2396 = f522(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2376, _, _, _, _, _, _, _) => {
                    h2376
                }
            });
            let a2397 = {
                let a2394 = f599(match v0.clone() {
                    Adt44::C0(_, _, h2377, _, _, _, _, _, _) => {
                        h2377
                    }
                });
                let a2395 = {
                    let a2392 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2378, _, _, _, _, _) => {
                            h2378
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2393 = {
                        let a2390 = f522(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2379, _, _, _, _) => {
                                h2379
                            }
                        });
                        let a2391 = {
                            let a2388 = f522(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2380, _, _, _) => {
                                    h2380
                                }
                            });
                            let a2389 = {
                                let a2386 = f702(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2381, _, _) => {
                                        h2381
                                    }
                                });
                                let a2387 = {
                                    let a2384 = f703(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2382, _) => {
                                            h2382
                                        }
                                    })?;
                                    let a2385 = f697(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2383) => {
                                            h2383
                                        }
                                    });
                                    append_list(a2384, a2385)
                                };
                                append_list(a2386, a2387)
                            };
                            append_list(a2388, a2389)
                        };
                        append_list(a2390, a2391)
                    };
                    append_list(a2392, a2393)
                };
                append_list(a2394, a2395)
            };
            append_list(a2396, a2397)
        };
        append_list(a2398, a2399)
    }))
}

pub fn f654(v0: Adt46) -> Adt6 {
    let m2400 = v0;
    match m2400 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2401, _, _, _) => {
                    h2401
                }
            })), {
                let a2405 = f522(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2402, _, _) => {
                        h2402
                    }
                });
                let a2406 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2403, _) => {
                        h2403
                    }
                })), f697(match v1 {
                    Adt47::C0(_, _, _, h2404) => {
                        h2404
                    }
                }));
                append_list(a2405, a2406)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2407, _, _, _) => {
                    h2407
                }
            })), {
                let a2411 = f522(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2408, _, _) => {
                        h2408
                    }
                });
                let a2412 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2409, _) => {
                        h2409
                    }
                })), f697(match v2 {
                    Adt48::C0(_, _, _, h2410) => {
                        h2410
                    }
                }));
                append_list(a2411, a2412)
            }))
        }
    }
}

pub fn f655(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a2433 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h2413, _, _, _, _, _, _, _) => {
                h2413
            }
        })), List::<(Str, Adt6)>::nil());
        let a2434 = {
            let a2431 = f522(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h2414, _, _, _, _, _, _) => {
                    h2414
                }
            });
            let a2432 = {
                let a2429 = f599(match v0.clone() {
                    Adt54::C0(_, _, h2415, _, _, _, _, _) => {
                        h2415
                    }
                });
                let a2430 = {
                    let a2427 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h2416, _, _, _, _) => {
                            h2416
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2428 = {
                        let a2425 = f522(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h2417, _, _, _) => {
                                h2417
                            }
                        });
                        let a2426 = {
                            let a2423 = f522(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h2418, _, _) => {
                                    h2418
                                }
                            });
                            let a2424 = {
                                let a2421 = f702(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h2419, _) => {
                                        h2419
                                    }
                                });
                                let a2422 = f697(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h2420) => {
                                        h2420
                                    }
                                });
                                append_list(a2421, a2422)
                            };
                            append_list(a2423, a2424)
                        };
                        append_list(a2425, a2426)
                    };
                    append_list(a2427, a2428)
                };
                append_list(a2429, a2430)
            };
            append_list(a2431, a2432)
        };
        append_list(a2433, a2434)
    })
}

pub fn f656(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2450 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2435, _, _, _, _, _) => {
                h2435
            }
        })), List::<(Str, Adt6)>::nil());
        let a2451 = {
            let a2448 = f522(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2436, _, _, _, _) => {
                    h2436
                }
            });
            let a2449 = {
                let a2446 = f599(match v0.clone() {
                    Adt56::C0(_, _, h2437, _, _, _) => {
                        h2437
                    }
                });
                let a2447 = {
                    let a2444 = f522(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h2438, _, _) => {
                            h2438
                        }
                    });
                    let a2445 = {
                        let a2442 = {
                            let m2440 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h2439, _) => {
                                    h2439
                                }
                            };
                            match m2440 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f704(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2443 = f697(match v0 {
                            Adt56::C0(_, _, _, _, _, h2441) => {
                                h2441
                            }
                        });
                        append_list(a2442, a2443)
                    };
                    append_list(a2444, a2445)
                };
                append_list(a2446, a2447)
            };
            append_list(a2448, a2449)
        };
        append_list(a2450, a2451)
    })
}

pub fn f657(v0: Adt59) -> R<Option<Adt6>> {
    let m2453 = f652(match v0.clone() {
        Adt59::C0(_, h2452) => {
            h2452
        }
    })?;
    match m2453 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f705(match v0 {
                Adt59::C0(h2454, _) => {
                    h2454
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f658(v0: List<Adt2>) -> List<Adt6> {
    f396(f706(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f659(v0: List<u64>) -> List<u64> {
    let a2457 = f707();
    let a2458 = {
        let a2455 = v0;
        let a2456 = f708();
        append_list(a2455, a2456)
    };
    append_list(a2457, a2458)
}

pub fn f660(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f709(v0, 0u64, v1)
}

pub fn f661(p2465: List<u64>) -> bool {
    let m2464 = p2465;
    {
        let mut m2464 = m2464;
        loop {
            let step = {
                let v0 = m2464;
                let m2459 = v0;
                match m2459.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2460 = v1;
                        let a2461 = 48u64;
                        if if nat_eq(a2460, a2461) {
                            true
                        } else {
                            let a2462 = v1;
                            let a2463 = 46u64;
                            nat_eq(a2462, a2463)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f662(v0: Adt134) -> bool {
    let m2466 = v0;
    match m2466 {
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

pub fn f663(v0: Adt14) -> Adt134 {
    let m2467 = v0;
    match m2467 {
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

pub fn f664(v0: Adt0, v1: Adt32) -> Adt123 {
    Adt123::C0(v1, Adt124::C0(Some(match v0 {
        Adt0::C0(h2468, _, _, _, _, _) => {
            h2468
        }
    }), None::<Adt5>), List::<Adt125>::nil())
}

pub fn f665(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2469, _, _) => {
            h2469
        }
    }, f264(), match v0.clone() {
        Adt0::C0(_, _, h2470, _, _, _) => {
            h2470
        }
    }, match v0 {
        Adt0::C0(_, h2471, _, _, _, _) => {
            h2471
        }
    })
}

pub fn f666(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt123, Adt126> {
    let m2472 = v2;
    match m2472 {
        Ok(v4) => {
            if f710(f663(v1.clone()), v4.clone()) {
                f711(v0, v1, v4)
            } else {
                Err::<Adt123, Adt126>(Adt126::C4(f712()))
            }
        }
        Err(v3) => {
            Err::<Adt123, Adt126>(Adt126::C4(v3))
        }
    }
}

pub fn f667(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2473 = v3.clone();
    match m2473 {
        Adt14::C0(_) => {
            let c2475: Fn0 = v2;
            c2475.apply(v3)
        }
        Adt14::C1(_) => {
            let c2476: Fn0 = v2;
            c2476.apply(v3)
        }
        Adt14::C2(_) => {
            if f713(v0, v1) {
                let c2474: Fn0 = v2;
                c2474.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2477: Fn0 = v2;
            c2477.apply(v3)
        }
        Adt14::C4(_) => {
            let c2478: Fn0 = v2;
            c2478.apply(v3)
        }
        Adt14::C5(_) => {
            let c2479: Fn0 = v2;
            c2479.apply(v3)
        }
        Adt14::C6(_) => {
            let c2480: Fn0 = v2;
            c2480.apply(v3)
        }
        Adt14::C7(_) => {
            let c2481: Fn0 = v2;
            c2481.apply(v3)
        }
        Adt14::C8(_) => {
            let c2482: Fn0 = v2;
            c2482.apply(v3)
        }
        Adt14::C9(_) => {
            let c2483: Fn0 = v2;
            c2483.apply(v3)
        }
        Adt14::C10(_) => {
            let c2484: Fn0 = v2;
            c2484.apply(v3)
        }
        Adt14::C11(_) => {
            let c2485: Fn0 = v2;
            c2485.apply(v3)
        }
    }
}

pub fn f668(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt127> {
    let m2486 = f266(v0.clone(), v1);
    match m2486 {
        Adt130::C0 => {
            Ok::<Option<Str>, Adt127>(None::<Str>)
        }
        Adt130::C1(v2) => {
            let m2487 = f331(v2);
            match m2487 {
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

pub fn f669(v0: Str, v1: Adt5) -> Result<Str, Adt127> {
    let m2488 = f266(v0.clone(), v1);
    match m2488 {
        Adt130::C0 => {
            Err::<Str, Adt127>(Adt127::C7(v0))
        }
        Adt130::C1(v2) => {
            let m2489 = f331(v2);
            match m2489 {
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

pub fn f670(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt127> {
    let m2490 = f266(v0.clone(), v1);
    match m2490 {
        Adt130::C0 => {
            Ok::<Option<Adt5>, Adt127>(None::<Adt5>)
        }
        Adt130::C1(v2) => {
            let m2491 = f285(v2);
            match m2491 {
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

pub fn f671(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt127> {
    let m2492 = f266(v0.clone(), v1);
    match m2492 {
        Adt130::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt127>(None::<List<(Str, Str)>>)
        }
        Adt130::C1(v2) => {
            let m2493 = f285(v2);
            match m2493 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt127>(Adt127::C8(v0))
                }
                Some(v3) => {
                    let m2495 = f714(v0, match v3 {
                        Adt5::C0(h2494) => {
                            h2494
                        }
                    });
                    match m2495 {
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

pub fn f672(v0: Str, v1: Adt5) -> Result<Adt5, Adt127> {
    let m2496 = f266(v0.clone(), v1);
    match m2496 {
        Adt130::C0 => {
            Err::<Adt5, Adt127>(Adt127::C7(v0))
        }
        Adt130::C1(v2) => {
            let m2497 = f285(v2);
            match m2497 {
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

pub fn f673(v0: Adt5) -> Result<Adt25, Adt127> {
    let m2498 = f669(Str::lit("type"), v0.clone());
    match m2498 {
        Ok(v1) => {
            let a2499 = v1.clone();
            let a2500 = Str::lit("ref/prompt");
            if equal(a2499, a2500) {
                let m2501 = f669(Str::lit("name"), v0.clone());
                match m2501 {
                    Ok(v2) => {
                        let m2502 = f668(Str::lit("title"), v0);
                        match m2502 {
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
                let a2503 = v1;
                let a2504 = Str::lit("ref/resource");
                if equal(a2503, a2504) {
                    let m2505 = f669(Str::lit("uri"), v0);
                    match m2505 {
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

pub fn f674(v0: Adt5) -> Result<Adt27, Adt127> {
    let m2506 = f669(Str::lit("name"), v0.clone());
    match m2506 {
        Ok(v1) => {
            let m2507 = f669(Str::lit("value"), v0);
            match m2507 {
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

pub fn f675(v0: Adt5) -> Result<Option<Adt28>, Adt127> {
    let m2508 = f266(Str::lit("context"), v0);
    match m2508 {
        Adt130::C0 => {
            Ok::<Option<Adt28>, Adt127>(None::<Adt28>)
        }
        Adt130::C1(v1) => {
            let m2509 = f285(v1);
            match m2509 {
                None => {
                    Err::<Option<Adt28>, Adt127>(Adt127::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2510 = f671(Str::lit("arguments"), v2);
                    match m2510 {
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

pub fn f676(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt126> {
    if v0 {
        None::<Adt126>
    } else {
        Some(Adt126::C1(v1, v2))
    }
}

pub fn f677(v0: Str, p2515: List<Str>) -> bool {
    let m2514 = p2515;
    {
        let mut m2514 = m2514;
        loop {
            let step = {
                let v1 = m2514;
                let m2511 = v1;
                match m2511.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2512 = v2;
                        let a2513 = v0.clone();
                        if equal(a2512, a2513) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
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

pub fn f678(v0: Option<Adt108>, v1: List<Adt110>, v2: Adt5) -> R<Option<Adt108>> {
    let m2516 = v0;
    match m2516 {
        None => {
            f715(v1, f716(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f679(v0: Option<Str>, v1: Adt140, v2: Adt5) -> Option<Adt108> {
    let m2517 = v1.clone();
    match m2517 {
        Adt140::C0 => {
            None::<Adt108>
        }
        Adt140::C1 => {
            f717(v0, f718(v1, v2))
        }
        Adt140::C2 => {
            f717(v0, f718(v1, v2))
        }
        Adt140::C3 => {
            f717(v0, f718(v1, v2))
        }
    }
}

pub fn f680(v0: Adt130, v1: Adt130) -> Adt148 {
    let m2518 = v0;
    match m2518 {
        Adt130::C0 => {
            Adt148::C1
        }
        Adt130::C1(_) => {
            let m2519 = v1;
            match m2519 {
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

pub fn f681(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt127> {
    let m2520 = f331(v1);
    match m2520 {
        None => {
            Err::<Adt16, Adt127>(Adt127::C5)
        }
        Some(v3) => {
            let m2521 = f719(v2);
            match m2521 {
                None => {
                    Err::<Adt16, Adt127>(Adt127::C6)
                }
                Some(v4) => {
                    let m2522 = f670(f720(), v0.clone());
                    match m2522 {
                        Ok(v6) => {
                            let m2523 = f721(v0.clone());
                            match m2523 {
                                Ok(v8) => {
                                    let m2524 = f722(v0.clone());
                                    match m2524 {
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

pub fn f682(p2528: List<Adt6>) -> bool {
    let m2527 = p2528;
    {
        let mut m2527 = m2527;
        loop {
            let step = {
                let v0 = m2527;
                let m2525 = v0;
                match m2525.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2526 = f331(v1);
                        match m2526 {
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
                Ok(next) => { m2527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f683(p2532: List<Adt6>) -> bool {
    let m2531 = p2532;
    {
        let mut m2531 = m2531;
        loop {
            let step = {
                let v0 = m2531;
                let m2529 = v0;
                match m2529.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2530 = f331(v1);
                        match m2530 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f494(v3) {
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
                Ok(next) => { m2531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f684(v0: Str) -> Option<Adt13> {
    let a2533 = v0.clone();
    let a2534 = Str::lit("string");
    if equal(a2533, a2534) {
        Some(Adt13::C0)
    } else {
        let a2535 = v0.clone();
        let a2536 = Str::lit("integer");
        if equal(a2535, a2536) {
            Some(Adt13::C1)
        } else {
            let a2537 = v0;
            let a2538 = Str::lit("boolean");
            if equal(a2537, a2538) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f685(p2547: List<u64>, p2548: List<u64>) -> R<List<u64>> {
    let m2546 = (p2547, p2548);
    {
        let mut m2546 = m2546;
        loop {
            let step = {
                let (v0, v1) = m2546;
                let m2539 = v0;
                match m2539.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2540 = 65u64;
                        let a2541 = v2;
                        if if nat_le(a2540, a2541) {
                            let a2542 = v2;
                            let a2543 = 90u64;
                            nat_le(a2542, a2543)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2544 = v2;
                                let a2545 = 32u64;
                                nat_add(a2544, a2545)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(p2555: List<u64>, p2556: List<u64>) -> bool {
    let m2554 = (p2555, p2556);
    {
        let mut m2554 = m2554;
        loop {
            let step = {
                let (v0, v1) = m2554;
                let m2549 = v0;
                match m2549.uncons() {
                    None => {
                        let m2550 = v1;
                        match m2550.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2551 = v1;
                        match m2551.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2552 = v4;
                                let a2553 = v6;
                                if nat_eq(a2552, a2553) {
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
                Ok(next) => { m2554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f687(v0: Adt6) -> u64 {
    let m2557 = v0;
    match m2557 {
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

pub fn f688(v0: Adt147) -> bool {
    let m2558 = v0;
    match m2558 {
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

pub fn f689(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2559 = v0;
    match m2559 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2560 = v2;
                let a2561 = v1;
                append_list(a2560, a2561)
            })
        }
    }
}

pub fn f690(p2566: List<Adt6>, p2567: List<Adt6>, p2568: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2565 = (p2566, (p2567, p2568));
    {
        let mut m2565 = m2565;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2565;
                let m2562 = v0;
                match m2562.uncons() {
                    None => {
                        let m2563 = v1;
                        match m2563.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2564 = v1;
                        match m2564.uncons() {
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
                Ok(next) => { m2565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f691(p2574: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2575: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2573 = (p2574, p2575);
    {
        let mut m2573 = m2573;
        loop {
            let step = {
                let (v0, v2) = m2573;
                let m2569 = v0;
                match m2569.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2571 = f334({
                            let (h2570, _) = v3;
                            h2570
                        }, v1.clone());
                        match m2571 {
                            Adt130::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt130::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2572) = v3;
                                    h2572
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f692(v0: Adt147) -> Adt147 {
    let m2576 = v0;
    match m2576 {
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

pub fn f693(v0: i64, v1: i64) -> R<i64> {
    f244(f186(f695(v0)?, 0i64)?, v1)
}

pub fn f694(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2577 = v1;
    let a2578 = v3;
    if nat_lt(a2577, a2578) {
        Ok(f580(f723({
            let a2579 = v3;
            let a2580 = v1;
            nat_sub(a2579, a2580)
        }, v0)?, v2))
    } else {
        let a2581 = v3;
        let a2582 = v1;
        if nat_lt(a2581, a2582) {
            Ok(f580(v0, f723({
                let a2583 = v1;
                let a2584 = v3;
                nat_sub(a2583, a2584)
            }, v2)?))
        } else {
            Ok(f580(v0, v2))
        }
    }
}

pub fn f695(v0: i64) -> R<u64> {
    f724(20u64, v0, 0u64)
}

pub fn f696(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2585 = v0;
    match m2585 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2597 = f522(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2586, _, _, _, _) => {
                        h2586
                    }
                });
                let a2598 = {
                    let a2595 = f527(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2587, _, _, _) => {
                            h2587
                        }
                    });
                    let a2596 = {
                        let a2593 = f527(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2588, _, _) => {
                                h2588
                            }
                        });
                        let a2594 = {
                            let a2591 = f527(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2589, _) => {
                                    h2589
                                }
                            });
                            let a2592 = f527(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2590) => {
                                    h2590
                                }
                            });
                            append_list(a2591, a2592)
                        };
                        append_list(a2593, a2594)
                    };
                    append_list(a2595, a2596)
                };
                append_list(a2597, a2598)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f697(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f548(Str::lit("_meta"), v0)
}

pub fn f698(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2599, _, _) => {
            h2599
        }
    })), {
        let a2602 = f702(match v0.clone() {
            Adt39::C0(_, h2600, _) => {
                h2600
            }
        });
        let a2603 = f697(match v0 {
            Adt39::C0(_, _, h2601) => {
                h2601
            }
        });
        append_list(a2602, a2603)
    })))
}

pub fn f699(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2604 = f702(v3);
        let a2605 = f697(v4);
        append_list(a2604, a2605)
    }))))
}

pub fn f700(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2629 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2606, _, _, _, _, _, _, _, _) => {
                h2606
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2630 = {
            let a2627 = f522(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2607, _, _, _, _, _, _, _) => {
                    h2607
                }
            });
            let a2628 = {
                let a2625 = f599(match v0.clone() {
                    Adt44::C0(_, _, h2608, _, _, _, _, _, _) => {
                        h2608
                    }
                });
                let a2626 = {
                    let a2623 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2609, _, _, _, _, _) => {
                            h2609
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2624 = {
                        let a2621 = f522(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2610, _, _, _, _) => {
                                h2610
                            }
                        });
                        let a2622 = {
                            let a2619 = f522(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2611, _, _, _) => {
                                    h2611
                                }
                            });
                            let a2620 = {
                                let a2617 = f702(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2612, _, _) => {
                                        h2612
                                    }
                                });
                                let a2618 = {
                                    let a2615 = f703(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2613, _) => {
                                            h2613
                                        }
                                    })?;
                                    let a2616 = f697(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2614) => {
                                            h2614
                                        }
                                    });
                                    append_list(a2615, a2616)
                                };
                                append_list(a2617, a2618)
                            };
                            append_list(a2619, a2620)
                        };
                        append_list(a2621, a2622)
                    };
                    append_list(a2623, a2624)
                };
                append_list(a2625, a2626)
            };
            append_list(a2627, a2628)
        };
        append_list(a2629, a2630)
    }))
}

pub fn f701(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f654(match v0.clone() {
        Adt45::C0(h2631, _, _) => {
            h2631
        }
    })), {
        let a2634 = f702(match v0.clone() {
            Adt45::C0(_, h2632, _) => {
                h2632
            }
        });
        let a2635 = f697(match v0 {
            Adt45::C0(_, _, h2633) => {
                h2633
            }
        });
        append_list(a2634, a2635)
    })))
}

pub fn f702(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2636 = v0;
    match m2636 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2642 = f725(match v1.clone() {
                    Adt40::C0(h2637, _, _) => {
                        h2637
                    }
                });
                let a2643 = {
                    let a2640 = f726(match v1.clone() {
                        Adt40::C0(_, h2638, _) => {
                            h2638
                        }
                    });
                    let a2641 = f522(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2639) => {
                            h2639
                        }
                    });
                    append_list(a2640, a2641)
                };
                append_list(a2642, a2643)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f703(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2644 = v0;
    match m2644 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f66(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f704(v0: List<Adt57>) -> List<Adt6> {
    f396(f727(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f705(v0: Adt41) -> Str {
    let m2645 = v0;
    match m2645 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f706(p2648: List<Adt2>, p2649: List<Adt6>) -> List<Adt6> {
    let m2647 = (p2648, p2649);
    {
        let mut m2647 = m2647;
        loop {
            let step = {
                let (v0, v1) = m2647;
                let m2646 = v0;
                match m2646.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f728(v2), v1)))
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

pub fn f707() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f708() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f709(p2654: List<u64>, p2655: u64, p2656: List<u64>) -> List<u64> {
    let m2653 = (p2654, (p2655, p2656));
    {
        let mut m2653 = m2653;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2653;
                let m2650 = v0;
                match m2650.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f729(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt150 = f730(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt150::C0(h2651, _) => {
                                h2651
                            }
                        }, match v5 {
                            Adt150::C0(_, h2652) => {
                                h2652
                            }
                        })))
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

pub fn f710(v0: Adt134, v1: Adt32) -> bool {
    let m2657 = v1;
    match m2657 {
        Adt32::C0(_) => {
            f731(v0, Adt134::C0)
        }
        Adt32::C1(_) => {
            f731(v0, Adt134::C1)
        }
        Adt32::C2(_) => {
            f731(v0, Adt134::C2)
        }
        Adt32::C3(_) => {
            f731(v0, Adt134::C2)
        }
        Adt32::C4(_) => {
            f731(v0, Adt134::C3)
        }
        Adt32::C5(_) => {
            f731(v0, Adt134::C4)
        }
        Adt32::C6(_) => {
            f731(v0, Adt134::C5)
        }
        Adt32::C7(_) => {
            f731(v0, Adt134::C6)
        }
        Adt32::C8(_) => {
            f731(v0, Adt134::C7)
        }
        Adt32::C9(_) => {
            f731(v0, Adt134::C8)
        }
        Adt32::C10(_) => {
            f731(v0, Adt134::C9)
        }
        Adt32::C11 => {
            if f731(v0.clone(), Adt134::C10) {
                true
            } else {
                f731(v0, Adt134::C11)
            }
        }
        Adt32::C12(_) => {
            f732(v0)
        }
        Adt32::C13(_) => {
            f733(v0)
        }
    }
}

pub fn f711(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt123, Adt126> {
    let m2658 = v2.clone();
    match m2658 {
        Adt32::C0(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt123, Adt126>(f737(v0, f735(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt123, Adt126>(f664(v0, v2))
        }
        Adt32::C12(v3) => {
            f734(v0, f735(v1), v3)
        }
        Adt32::C13(v4) => {
            f736(v0, f735(v1), v4)
        }
    }
}

pub fn f712() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f713(v0: u64, v1: u64) -> bool {
    let a2659 = v1;
    let a2660 = v0;
    nat_lt(a2659, a2660)
}

pub fn f714(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt127> {
    let m2661 = f738(v0, v1, List::<(Str, Str)>::nil());
    match m2661 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt127>(f739(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt127>(v3)
        }
    }
}

pub fn f715(p2665: List<Adt110>, v1: Adt5) -> R<Option<Adt108>> {
    let m2664 = p2665;
    {
        let mut m2664 = m2664;
        loop {
            let step = {
                let v0 = m2664;
                let m2662 = v0;
                match m2662.uncons() {
                    None => {
                        Err::<List<Adt110>, R<Option<Adt108>>>(Ok(None::<Adt108>))
                    }
                    Some((v2, v3)) => {
                        let m2663 = f740(v2, v1.clone())?;
                        match m2663 {
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
                Ok(next) => { m2664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716(v0: Adt5) -> Adt5 {
    let m2666 = f266(Str::lit("arguments"), v0);
    match m2666 {
        Adt130::C0 => {
            f26()
        }
        Adt130::C1(v1) => {
            let m2667 = f285(v1);
            match m2667 {
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

pub fn f717(v0: Option<Str>, v1: Option<Str>) -> Option<Adt108> {
    let m2668 = v0;
    match m2668 {
        None => {
            Some(Adt108::C2)
        }
        Some(v2) => {
            let m2669 = v1;
            match m2669 {
                None => {
                    None::<Adt108>
                }
                Some(v3) => {
                    let a2670 = v2;
                    let a2671 = v3;
                    if equal(a2670, a2671) {
                        None::<Adt108>
                    } else {
                        Some(Adt108::C5)
                    }
                }
            }
        }
    }
}

pub fn f718(v0: Adt140, v1: Adt5) -> Option<Str> {
    let m2672 = v0;
    match m2672 {
        Adt140::C0 => {
            None::<Str>
        }
        Adt140::C1 => {
            let m2673 = f266(Str::lit("name"), v1);
            match m2673 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v2) => {
                    f331(v2)
                }
            }
        }
        Adt140::C2 => {
            let m2674 = f266(Str::lit("uri"), v1);
            match m2674 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v3) => {
                    f331(v3)
                }
            }
        }
        Adt140::C3 => {
            let m2675 = f266(Str::lit("taskId"), v1);
            match m2675 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v4) => {
                    f331(v4)
                }
            }
        }
    }
}

pub fn f719(v0: Adt6) -> Option<Adt17> {
    let m2676 = f285(v0);
    match m2676 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2677 = f741(Str::lit("experimental"), v1.clone());
            match m2677 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2678 = f741(Str::lit("roots"), v1.clone());
                    match m2678 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2679 = f741(Str::lit("sampling"), v1.clone());
                            match m2679 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2680 = f741(Str::lit("elicitation"), v1.clone());
                                    match m2680 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2681 = f741(Str::lit("extensions"), v1);
                                            match m2681 {
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

pub fn f720() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f721(v0: Adt5) -> Result<Option<Adt18>, Adt127> {
    let m2682 = f668(f742(), v0);
    match m2682 {
        Ok(v2) => {
            let m2683 = v2;
            match m2683 {
                None => {
                    Ok::<Option<Adt18>, Adt127>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2684 = f743(v3);
                    match m2684 {
                        Adt151::C0 => {
                            Err::<Option<Adt18>, Adt127>(Adt127::C8(f742()))
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

pub fn f722(v0: Adt5) -> Result<Option<Adt19>, Adt127> {
    let m2685 = f266(f744(), v0);
    match m2685 {
        Adt130::C0 => {
            Ok::<Option<Adt19>, Adt127>(None::<Adt19>)
        }
        Adt130::C1(v1) => {
            let m2686 = f331(v1.clone());
            match m2686 {
                None => {
                    let m2687 = f579(v1);
                    match m2687 {
                        None => {
                            Err::<Option<Adt19>, Adt127>(Adt127::C8(f744()))
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

pub fn f723(p2692: u64, p2693: i64) -> R<i64> {
    let m2691 = (p2692, p2693);
    {
        let mut m2691 = m2691;
        loop {
            let step = {
                let (v0, v1) = m2691;
                let m2688 = v0;
                if m2688 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2688 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2689 = v1;
                        let a2690 = 10i64;
                        int_mul(a2689, a2690)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f724(p2704: u64, p2705: i64, p2706: u64) -> R<u64> {
    let m2703 = (p2704, (p2705, p2706));
    {
        let mut m2703 = m2703;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2703;
                let m2694 = v0;
                if m2694 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2694 - 1;
                    let v4: u64 = {
                        let a2695 = v2;
                        let a2696 = 1u64;
                        nat_add(a2695, a2696)?
                    };
                    let m2699 = {
                        let a2697 = v1;
                        let a2698 = 10i64;
                        compare(a2697, a2698)
                    };
                    if match m2699 {
                        Ordering::Less => {
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
                            let a2700 = v1;
                            let a2701 = 10i64;
                            let a2702 = 0i64;
                            int_quot(a2700, a2701, a2702)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f725(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2707 = v0;
    match m2707 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f745(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f726(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2708 = v0;
    match m2708 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f727(p2711: List<Adt57>, p2712: List<Adt6>) -> List<Adt6> {
    let m2710 = (p2711, p2712);
    {
        let mut m2710 = m2710;
        loop {
            let step = {
                let (v0, v1) = m2710;
                let m2709 = v0;
                match m2709.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f746(v2), v1)))
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

pub fn f728(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2713, _, _, _) => {
            h2713
        }
    })), {
        let a2719 = f522(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2714, _, _) => {
                h2714
            }
        });
        let a2720 = {
            let a2717 = f747(match v0.clone() {
                Adt2::C0(_, _, h2715, _) => {
                    h2715
                }
            });
            let a2718 = f748(match v0 {
                Adt2::C0(_, _, _, h2716) => {
                    h2716
                }
            });
            append_list(a2717, a2718)
        };
        append_list(a2719, a2720)
    }))
}

pub fn f729(v0: u64, v1: List<u64>) -> List<u64> {
    let a2721 = v0;
    let a2722 = 0u64;
    if nat_eq(a2721, a2722) {
        v1
    } else {
        let a2723 = v0;
        let a2724 = 1u64;
        if nat_eq(a2723, a2724) {
            List::cons(194u64, v1)
        } else {
            let a2725 = v0;
            let a2726 = 2u64;
            if nat_eq(a2725, a2726) {
                List::cons(226u64, v1)
            } else {
                let a2727 = v0;
                let a2728 = 3u64;
                if nat_eq(a2727, a2728) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f730(v0: u64, v1: u64, v2: List<u64>) -> Adt150 {
    let a2729 = v0;
    let a2730 = 0u64;
    if nat_eq(a2729, a2730) {
        f749(v1, v2)
    } else {
        let a2731 = v0;
        let a2732 = 1u64;
        if nat_eq(a2731, a2732) {
            let a2733 = 128u64;
            let a2734 = v1;
            if if nat_le(a2733, a2734) {
                let a2735 = v1;
                let a2736 = 159u64;
                nat_le(a2735, a2736)
            } else {
                false
            } {
                Adt150::C0(0u64, f750(v2))
            } else {
                f749(v1, List::cons(194u64, v2))
            }
        } else {
            let a2737 = v0;
            let a2738 = 2u64;
            if nat_eq(a2737, a2738) {
                let a2739 = v1;
                let a2740 = 128u64;
                if nat_eq(a2739, a2740) {
                    Adt150::C0(3u64, v2)
                } else {
                    let a2741 = v1;
                    let a2742 = 129u64;
                    if nat_eq(a2741, a2742) {
                        Adt150::C0(4u64, v2)
                    } else {
                        f749(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2743 = v0;
                let a2744 = 3u64;
                if nat_eq(a2743, a2744) {
                    let a2745 = 170u64;
                    let a2746 = v1;
                    if if nat_le(a2745, a2746) {
                        let a2747 = v1;
                        let a2748 = 174u64;
                        nat_le(a2747, a2748)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f750(v2))
                    } else {
                        f749(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2749 = 166u64;
                    let a2750 = v1;
                    if if nat_le(a2749, a2750) {
                        let a2751 = v1;
                        let a2752 = 169u64;
                        nat_le(a2751, a2752)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f750(v2))
                    } else {
                        f749(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f731(v0: Adt134, v1: Adt134) -> bool {
    let a2753 = f552(v0);
    let a2754 = f552(v1);
    equal(a2753, a2754)
}

pub fn f732(v0: Adt134) -> bool {
    let m2755 = v0;
    match m2755 {
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

pub fn f733(v0: Adt134) -> bool {
    let m2756 = v0;
    match m2756 {
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

pub fn f734(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt123, Adt126> {
    f751(v0, v2.clone(), f752(match v1 {
        Adt16::C0(_, h2757, _, _, _, _) => {
            h2757
        }
    }, f753(v2)))
}

pub fn f735(v0: Adt14) -> Adt16 {
    let m2758 = v0;
    match m2758 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2759) => {
                    h2759
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2760, _) => {
                    h2760
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2761, _, _, _, _) => {
                    h2761
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2762, _) => {
                    h2762
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2763, _, _, _) => {
                    h2763
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2764, _) => {
                    h2764
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2765, _) => {
                    h2765
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2766, _, _, _, _) => {
                    h2766
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2767, _, _, _) => {
                    h2767
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2768, _) => {
                    h2768
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2769, _, _) => {
                    h2769
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2770, _) => {
                    h2770
                }
            }
        }
    }
}

pub fn f736(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt123, Adt126> {
    f754(v0, v2, f755(match v1 {
        Adt16::C0(_, h2771, _, _, _, _) => {
            h2771
        }
    }))
}

pub fn f737(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt123 {
    Adt123::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2772, _) => {
            h2772
        }
    }), {
        let m2774 = f664(v0, Adt32::C11);
        match m2774 {
            Adt123::C0(_, h2773, _) => {
                h2773
            }
        }
    }, f756(v1, match v2 {
        Adt49::C0(_, h2775) => {
            h2775
        }
    }))
}

pub fn f738(v0: Str, p2781: List<(Str, Adt6)>, p2782: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt127> {
    let m2780 = (p2781, p2782);
    {
        let mut m2780 = m2780;
        loop {
            let step = {
                let (v1, v2) = m2780;
                let m2776 = v1;
                match m2776.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>(Ok::<List<(Str, Str)>, Adt127>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2778 = f331({
                            let (_, h2777) = v3;
                            h2777
                        });
                        match m2778 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>(Err::<List<(Str, Str)>, Adt127>(Adt127::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>((v4, List::cons(({
                                    let (h2779, _) = v3;
                                    h2779
                                }, v5), v2)))
                            }
                        }
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

pub fn f739(p2785: List<(Str, Str)>, p2786: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2784 = (p2785, p2786);
    {
        let mut m2784 = m2784;
        loop {
            let step = {
                let (v0, v1) = m2784;
                let m2783 = v0;
                match m2783.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
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

pub fn f740(v0: Adt110, v1: Adt5) -> R<Option<Adt108>> {
    let m2788 = f757(match v0.clone() {
        Adt110::C0(_, h2787, _, _) => {
            h2787
        }
    }, v1);
    match m2788 {
        Adt130::C0 => {
            Ok(f758(v0))
        }
        Adt130::C1(v2) => {
            if f759(v2.clone()) {
                Ok(f758(v0))
            } else {
                f760(v0, v2)
            }
        }
    }
}

pub fn f741(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2789 = f266(v0, v1);
    match m2789 {
        Adt130::C0 => {
            Some(None::<Adt5>)
        }
        Adt130::C1(v2) => {
            let m2790 = f285(v2);
            match m2790 {
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

pub fn f742() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f743(v0: Str) -> Adt151 {
    let a2791 = v0.clone();
    let a2792 = Str::lit("debug");
    if equal(a2791, a2792) {
        Adt151::C1(Adt18::C0)
    } else {
        let a2793 = v0.clone();
        let a2794 = Str::lit("info");
        if equal(a2793, a2794) {
            Adt151::C1(Adt18::C1)
        } else {
            let a2795 = v0.clone();
            let a2796 = Str::lit("notice");
            if equal(a2795, a2796) {
                Adt151::C1(Adt18::C2)
            } else {
                let a2797 = v0.clone();
                let a2798 = Str::lit("warning");
                if equal(a2797, a2798) {
                    Adt151::C1(Adt18::C3)
                } else {
                    let a2799 = v0.clone();
                    let a2800 = Str::lit("error");
                    if equal(a2799, a2800) {
                        Adt151::C1(Adt18::C4)
                    } else {
                        let a2801 = v0.clone();
                        let a2802 = Str::lit("critical");
                        if equal(a2801, a2802) {
                            Adt151::C1(Adt18::C5)
                        } else {
                            let a2803 = v0.clone();
                            let a2804 = Str::lit("alert");
                            if equal(a2803, a2804) {
                                Adt151::C1(Adt18::C6)
                            } else {
                                let a2805 = v0;
                                let a2806 = Str::lit("emergency");
                                if equal(a2805, a2806) {
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

pub fn f744() -> Str {
    Str::lit("progressToken")
}

pub fn f745(v0: List<Adt41>) -> List<Adt6> {
    f396(f761(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f746(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2815 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2807, _, _, _) => {
                h2807
            }
        })), List::<(Str, Adt6)>::nil());
        let a2816 = {
            let a2813 = f522(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2808, _, _) => {
                    h2808
                }
            });
            let a2814 = {
                let a2811 = f522(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h2809, _) => {
                        h2809
                    }
                });
                let a2812 = f527(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h2810) => {
                        h2810
                    }
                });
                append_list(a2811, a2812)
            };
            append_list(a2813, a2814)
        };
        append_list(a2815, a2816)
    })
}

pub fn f747(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2817 = v0;
    match m2817 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f466(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f748(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2818 = v0;
    match m2818 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f762(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f749(v0: u64, v1: List<u64>) -> Adt150 {
    let a2819 = v0;
    let a2820 = 32u64;
    if nat_lt(a2819, a2820) {
        if f763(v0) {
            Adt150::C0(0u64, List::cons(v0, v1))
        } else {
            Adt150::C0(0u64, f750(v1))
        }
    } else {
        let a2821 = v0;
        let a2822 = 127u64;
        if nat_eq(a2821, a2822) {
            Adt150::C0(0u64, f750(v1))
        } else {
            let a2823 = v0;
            let a2824 = 194u64;
            if nat_eq(a2823, a2824) {
                Adt150::C0(1u64, v1)
            } else {
                let a2825 = v0;
                let a2826 = 226u64;
                if nat_eq(a2825, a2826) {
                    Adt150::C0(2u64, v1)
                } else {
                    Adt150::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f750(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f751(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt123, Adt126> {
    if v2 {
        Ok::<Adt123, Adt126>(f664(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt123, Adt126>(Adt126::C4(Adt84::C8(f764(f753(v1)))))
    }
}

pub fn f752(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2827 = f765(v1.clone());
    if if if bool_not(a2827) {
        true
    } else {
        f555(match v0.clone() {
            Adt17::C0(_, _, _, h2828, _) => {
                h2828
            }
        })
    } {
        let a2829 = f766(v1.clone());
        if bool_not(a2829) {
            true
        } else {
            f555(match v0.clone() {
                Adt17::C0(_, _, h2830, _, _) => {
                    h2830
                }
            })
        }
    } else {
        false
    } {
        let a2831 = f767(v1);
        if bool_not(a2831) {
            true
        } else {
            f555(match v0 {
                Adt17::C0(_, h2832, _, _, _) => {
                    h2832
                }
            })
        }
    } else {
        false
    }
}

pub fn f753(v0: Adt81) -> List<(Str, Adt64)> {
    let m2833 = v0;
    match m2833 {
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

pub fn f754(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt123, Adt126> {
    if v2 {
        Ok::<Adt123, Adt126>(f664(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt123, Adt126>(Adt126::C4(Adt84::C8(f768())))
    }
}

pub fn f755(v0: Adt17) -> bool {
    f556(match v0 {
        Adt17::C0(_, _, _, _, h2834) => {
            h2834
        }
    }, f557())
}

pub fn f756(v0: Adt16, v1: List<Adt50>) -> List<Adt125> {
    let m2836 = match v0 {
        Adt16::C0(_, _, _, _, h2835, _) => {
            h2835
        }
    };
    match m2836 {
        None => {
            List::<Adt125>::nil()
        }
        Some(v2) => {
            f769(f770(v2, v1, List::<Adt125>::nil()), List::<Adt125>::nil())
        }
    }
}

pub fn f757(p2842: List<Str>, p2843: Adt5) -> Adt130 {
    let m2841 = (p2842, p2843);
    {
        let mut m2841 = m2841;
        loop {
            let step = {
                let (v0, v1) = m2841;
                let m2837 = v0;
                match m2837.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                    }
                    Some((v2, v3)) => {
                        let m2838 = f266(v2, v1);
                        match m2838 {
                            Adt130::C0 => {
                                Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                            }
                            Adt130::C1(v4) => {
                                let m2839 = v3.clone();
                                match m2839.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt130>(Adt130::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2840 = f285(v4);
                                        match m2840 {
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
                Ok(next) => { m2841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f758(v0: Adt110) -> Option<Adt108> {
    let m2845 = match v0.clone() {
        Adt110::C0(_, _, _, h2844) => {
            h2844
        }
    };
    match m2845 {
        Adt111::C0 => {
            None::<Adt108>
        }
        Adt111::C1 => {
            Some(Adt108::C10(match v0 {
                Adt110::C0(h2846, _, _, _) => {
                    h2846
                }
            }))
        }
        Adt111::C2(_) => {
            None::<Adt108>
        }
    }
}

pub fn f759(v0: Adt6) -> bool {
    let m2847 = v0;
    match m2847 {
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

pub fn f760(v0: Adt110, v1: Adt6) -> R<Option<Adt108>> {
    let m2849 = match v0.clone() {
        Adt110::C0(_, _, _, h2848) => {
            h2848
        }
    };
    match m2849 {
        Adt111::C0 => {
            Ok(Some(Adt108::C8(match v0 {
                Adt110::C0(h2850, _, _, _) => {
                    h2850
                }
            })))
        }
        Adt111::C1 => {
            Ok(Some(Adt108::C10(match v0 {
                Adt110::C0(h2851, _, _, _) => {
                    h2851
                }
            })))
        }
        Adt111::C2(v2) => {
            if f771(match v0.clone() {
                Adt110::C0(_, _, h2852, _) => {
                    h2852
                }
            }, v2, v1)? {
                Ok(None::<Adt108>)
            } else {
                Ok(Some(Adt108::C9(match v0 {
                    Adt110::C0(h2853, _, _, _) => {
                        h2853
                    }
                })))
            }
        }
    }
}

pub fn f761(p2856: List<Adt41>, p2857: List<Adt6>) -> List<Adt6> {
    let m2855 = (p2856, p2857);
    {
        let mut m2855 = m2855;
        loop {
            let step = {
                let (v0, v1) = m2855;
                let m2854 = v0;
                match m2854.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f705(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f762(v0: Adt3) -> Str {
    let m2858 = v0;
    match m2858 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f763(v0: u64) -> bool {
    let a2859 = v0;
    let a2860 = 9u64;
    if nat_eq(a2859, a2860) {
        true
    } else {
        let a2861 = v0;
        let a2862 = 10u64;
        if nat_eq(a2861, a2862) {
            true
        } else {
            let a2863 = v0;
            let a2864 = 13u64;
            nat_eq(a2863, a2864)
        }
    }
}

pub fn f764(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f767(v0.clone()) {
        Some(f26())
    } else {
        None::<Adt5>
    }, if f766(v0.clone()) {
        Some(f26())
    } else {
        None::<Adt5>
    }, if f765(v0) {
        Some(f26())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f765(p2869: List<(Str, Adt64)>) -> bool {
    let m2868 = p2869;
    {
        let mut m2868 = m2868;
        loop {
            let step = {
                let v0 = m2868;
                let m2865 = v0;
                match m2865.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2867 = {
                            let (_, h2866) = v1;
                            h2866
                        };
                        match m2867 {
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
                Ok(next) => { m2868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f766(p2874: List<(Str, Adt64)>) -> bool {
    let m2873 = p2874;
    {
        let mut m2873 = m2873;
        loop {
            let step = {
                let v0 = m2873;
                let m2870 = v0;
                match m2870.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2872 = {
                            let (_, h2871) = v1;
                            h2871
                        };
                        match m2872 {
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
                Ok(next) => { m2873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f767(p2879: List<(Str, Adt64)>) -> bool {
    let m2878 = p2879;
    {
        let mut m2878 = m2878;
        loop {
            let step = {
                let v0 = m2878;
                let m2875 = v0;
                match m2875.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2877 = {
                            let (_, h2876) = v1;
                            h2876
                        };
                        match m2877 {
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
                Ok(next) => { m2878 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f768() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f557(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f769(p2882: List<Adt125>, p2883: List<Adt125>) -> List<Adt125> {
    let m2881 = (p2882, p2883);
    {
        let mut m2881 = m2881;
        loop {
            let step = {
                let (v0, v1) = m2881;
                let m2880 = v0;
                match m2880.uncons() {
                    None => {
                        Err::<(List<Adt125>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt125>, List<Adt125>), List<Adt125>>((v3, List::cons(v2, v1)))
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

pub fn f770(v0: Adt19, p2886: List<Adt50>, p2887: List<Adt125>) -> List<Adt125> {
    let m2885 = (p2886, p2887);
    {
        let mut m2885 = m2885;
        loop {
            let step = {
                let (v1, v2) = m2885;
                let m2884 = v1;
                match m2884.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt125>), List<Adt125>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt125>), List<Adt125>>((v4, List::cons(Adt125::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m2885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f771(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2888 = v0;
    match m2888 {
        Adt13::C0 => {
            let m2889 = f331(v2);
            match m2889 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2890 = v1;
                    let a2891 = v3;
                    Ok(equal(a2890, a2891))
                }
            }
        }
        Adt13::C1 => {
            f772(v1, v2)
        }
        Adt13::C2 => {
            let m2892 = f284(v2);
            match m2892 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2893 = v1;
                        let a2894 = Str::lit("true");
                        Ok(equal(a2893, a2894))
                    } else {
                        let a2895 = v1;
                        let a2896 = Str::lit("false");
                        Ok(equal(a2895, a2896))
                    }
                }
            }
        }
    }
}

pub fn f772(v0: Str, v1: Adt6) -> R<bool> {
    let m2897 = f499(v1);
    match m2897 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2898 = f333(v2)?;
            match m2898 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2900 = f773(f153({
                        let a2899 = v0;
                        utf8_encode(a2899)
                    })?)?;
                    match m2900 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f481({
                                let a2901 = v3;
                                let a2902 = v4;
                                int_sub(a2901, a2902)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f773(v0: List<u64>) -> R<Option<i64>> {
    let m2903 = v0.clone();
    match m2903.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2904 = v1;
            let a2905 = 45u64;
            if nat_eq(a2904, a2905) {
                let m2906 = v2.clone();
                match m2906.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2907 = f774(v2, 0i64)?;
                        match m2907 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2908 = v5;
                                    int_neg(a2908)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f774(v0, 0i64)
            }
        }
    }
}

pub fn f774(p2914: List<u64>, p2915: i64) -> R<Option<i64>> {
    let m2913 = (p2914, p2915);
    {
        let mut m2913 = m2913;
        loop {
            let step = {
                let (v0, v1) = m2913;
                let m2909 = v0;
                match m2909.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2910 = f775(v2);
                        match m2910 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f244({
                                    let a2911 = v1;
                                    let a2912 = 10i64;
                                    int_mul(a2911, a2912)?
                                }, f395(v4))?))
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

pub fn f775(v0: u64) -> Option<u64> {
    let a2916 = v0;
    let a2917 = 48u64;
    if if nat_lt(a2916, a2917) {
        true
    } else {
        let a2918 = 57u64;
        let a2919 = v0;
        nat_lt(a2918, a2919)
    } {
        None::<u64>
    } else {
        Some({
            let a2920 = v0;
            let a2921 = 48u64;
            nat_sub(a2920, a2921)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
