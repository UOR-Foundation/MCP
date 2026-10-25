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
    C15(Adt85),
}

#[derive(Clone)]
pub enum Adt85 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt86 {
    C0(List<u64>, List<u64>, List<Adt87>, Bytes, u64, List<List<u64>>, Bytes),
}

#[derive(Clone)]
pub enum Adt87 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt88 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt89 {
    C0(u64, List<Adt87>, List<List<u64>>, List<u64>),
}

#[derive(Clone)]
pub enum Adt90 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt91 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt92 {
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
pub enum Adt93 {
    C0(Adt49),
    C1(Adt81),
}

#[derive(Clone)]
pub enum Adt94 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt95 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt96 {
    C0,
    C1,
    C2(Adt5),
}

#[derive(Clone)]
pub enum Adt97 {
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
pub enum Adt98 {
    C0,
    C1(Option<Adt95>, Adt99),
}

#[derive(Clone)]
pub enum Adt99 {
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
pub enum Adt100 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt101 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt102 {
    C0,
    C1(List<Adt103>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt103 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt104 {
    C0,
    C1(List<Adt103>, Str),
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt6, Adt6, List<Adt103>),
    C1(List<Adt6>, Adt6, List<Adt103>),
    C2(List<Adt6>, Adt6, List<Adt103>, u64),
    C3(List<Adt103>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt103>),
    C5(Adt106, Adt6, Adt6, List<Adt103>),
}

#[derive(Clone)]
pub enum Adt106 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt107 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt108 {
    C0(Adt95, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt109 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt110 {
    C0(Adt104, List<Adt105>),
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
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt105>),
}

#[derive(Clone)]
pub enum Adt127 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt128 {
    C0(Adt95, Result<Adt129, Adt132>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0(Adt32, Adt130, List<Adt131>),
}

#[derive(Clone)]
pub enum Adt130 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Adt19, Adt50),
}

#[derive(Clone)]
pub enum Adt132 {
    C0(Adt133),
    C1(Str, List<Str>),
    C2(Adt113),
    C3(Str),
    C4(Adt84),
}

#[derive(Clone)]
pub enum Adt133 {
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
pub enum Adt134 {
    C0(Option<Str>, Option<Adt113>),
}

#[derive(Clone)]
pub enum Adt135 {
    C0,
    C1(Adt95),
    C2,
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt97),
    C1(Adt119),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Adt97),
    C1(List<Adt111>, List<u64>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt95, Adt139, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt139 {
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
pub enum Adt140 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt141 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt142 {
    C0,
    C1(Adt139),
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Adt144),
    C1(Adt132),
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt95, Str, Adt16, Adt5),
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
    C0(Adt92),
    C1(Adt125),
}

#[derive(Clone)]
pub enum Adt147 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt148 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt149 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
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
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt153 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt154 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt155 {
    C0,
    C1(Adt18),
}

#[derive(Clone)]
pub enum Adt156 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt157 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt158 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt157, u64),
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
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt93, Adt84>> {
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
    F903(Bytes),
    F928(Bytes, Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn7::F903(k0) => f903(k0.clone(), p0),
            Fn7::F928(k0, k1) => f928(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F908(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn8::F908(k0) => f908(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F913(Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn9::F913(k0) => f913(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F936(Bytes, u64, u64, u64),
    F943(Bytes, u64, u64),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Adt156)) -> R<(u64, Adt156)> {
        match self {
            Fn10::F936(k0, k1, k2, k3) => f936(k0.clone(), *k1, *k2, *k3, p0),
            Fn10::F943(k0, k1, k2) => f943(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F938,
}

impl Fn11 {
    pub fn apply(&self, p0: Adt158) -> R<Adt158> {
        match self {
            Fn11::F938 => f938(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F968,
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn12::F968 => f968(p0),
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

pub fn f4(v0: Bytes) -> R<Result<Adt86, Adt88>> {
    let m3 = f16(v0.clone(), 0u64)?;
    match m3 {
        None => {
            Ok(Err::<Adt86, Adt88>(Adt88::C0(0u64)))
        }
        Some(v1) => {
            f17(v0.clone(), v1.clone(), f16(v0, match v1 {
                Adt90::C0(_, h4) => {
                    h4
                }
            })?)
        }
    }
}

pub fn f5(v0: Adt89) -> List<u64> {
    let a25 = f18(match v0.clone() {
        Adt89::C0(h5, _, _, _) => {
            h5
        }
    });
    let a26 = {
        let a23 = f18({
            let a7 = match v0.clone() {
                Adt89::C0(_, h6, _, _) => {
                    h6
                }
            };
            length_list(a7)
        });
        let a24 = {
            let a21 = f19(match v0.clone() {
                Adt89::C0(_, h8, _, _) => {
                    h8
                }
            });
            let a22 = {
                let a19 = f18({
                    let a10 = match v0.clone() {
                        Adt89::C0(_, _, h9, _) => {
                            h9
                        }
                    };
                    length_list(a10)
                });
                let a20 = {
                    let a17 = f20(match v0.clone() {
                        Adt89::C0(_, _, h11, _) => {
                            h11
                        }
                    });
                    let a18 = {
                        let a15 = f18({
                            let a13 = match v0.clone() {
                                Adt89::C0(_, _, _, h12) => {
                                    h12
                                }
                            };
                            length_list(a13)
                        });
                        let a16 = match v0 {
                            Adt89::C0(_, _, _, h14) => {
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

pub fn f6() -> Adt89 {
    Adt89::C0(f21(), f22(), List::<List<u64>>::nil(), f23())
}

pub fn f7(v0: Adt0, v1: Fn0, v2: Adt86) -> R<Adt89> {
    f24(v0, v1, v2.clone(), f25(match v2.clone() {
        Adt86::C0(_, _, h27, _, _, _, _) => {
            h27
        }
    }, match v2 {
        Adt86::C0(_, _, _, _, _, h28, _) => {
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

pub fn f16(v0: Bytes, v1: u64) -> R<Option<Adt90>> {
    let m36 = f45(v0.clone(), v1)?;
    match m36 {
        None => {
            Ok(None::<Adt90>)
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
                    Ok(None::<Adt90>)
                }
                Some(v3) => {
                    Ok(Some(Adt90::C0(v3, {
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

pub fn f17(v0: Bytes, v1: Adt90, v2: Option<Adt90>) -> R<Result<Adt86, Adt88>> {
    let m47 = v2;
    match m47 {
        None => {
            Ok(Err::<Adt86, Adt88>(Adt88::C0(match v1 {
                Adt90::C0(_, h48) => {
                    h48
                }
            })))
        }
        Some(v3) => {
            f46(v0.clone(), v1, v3.clone(), f45(v0, match v3 {
                Adt90::C0(_, h49) => {
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

pub fn f19(v0: List<Adt87>) -> List<u64> {
    f47(f48(v0, List::<Adt87>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f49(f50(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt87> {
    List::cons(Adt87::C0(f51(), f52()), List::<Adt87>::nil())
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

pub fn f24(v0: Adt0, v1: Fn0, v2: Adt86, v3: Option<Adt91>) -> R<Adt89> {
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

pub fn f25(v0: List<Adt87>, v1: List<List<u64>>) -> R<Option<Adt91>> {
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

pub fn f27(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt92>> {
    let m77 = f60(v0.clone());
    match m77 {
        None => {
            f61(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt92>(v1))
        }
    }
}

pub fn f28() -> R<List<Adt35>> {
    Ok(List::cons(f62(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f63()), List::cons(f62(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f64()), List::cons(f62(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f63()), List::cons(f62(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f65()), List::cons(f62(Str::lit("test_image_content"), Str::lit("Tests image content response"), f65()), List::cons(f62(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f65()), List::cons(f62(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f65()), List::cons(f62(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f65()), List::cons(f62(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f65()), List::cons(f62(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f65()), List::cons(f62(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f66()), List::cons(f62(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f67()?), List::cons(f62(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f68()), List::cons(f69(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f65(), f70()), List::cons(f69(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f65(), f70()), List::cons(f62(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f65()), List::cons(f62(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f65()), List::cons(f62(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f65()), List::cons(f62(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f65()), List::cons(f62(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f65()), List::<Adt35>::nil())))))))))))))))))))))
}

pub fn f29() -> List<Adt35> {
    List::cons(f71(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f71(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f71(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f71(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f71(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f71(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f71(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f71(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt35>::nil()))))))))
}

pub fn f30(v0: Adt92) -> Str {
    let m78 = v0;
    match m78 {
        Adt92::C0(v1) => {
            let a79 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a80 = Str::lit(" ");
            join(a79, a80)
        }
        Adt92::C1(v2) => {
            let a81 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a82 = Str::lit(" ");
            join(a81, a82)
        }
        Adt92::C2(v3) => {
            let a83 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a84 = Str::lit(" ");
            join(a83, a84)
        }
        Adt92::C3(v4) => {
            let a85 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a86 = Str::lit(" ");
            join(a85, a86)
        }
        Adt92::C4(v5, v6) => {
            let a87 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a88 = Str::lit(" ");
            join(a87, a88)
        }
        Adt92::C5(v7, v8) => {
            let a89 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a90 = Str::lit(" ");
            join(a89, a90)
        }
        Adt92::C6(v9, v10) => {
            let a91 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a92 = Str::lit(" ");
            join(a91, a92)
        }
        Adt92::C7(v11) => {
            let a93 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a94 = Str::lit(" ");
            join(a93, a94)
        }
        Adt92::C8(v12) => {
            let a95 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a96 = Str::lit(" ");
            join(a95, a96)
        }
        Adt92::C9(v13) => {
            let a97 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a98 = Str::lit(" ");
            join(a97, a98)
        }
        Adt92::C10(v14) => {
            let a99 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a100 = Str::lit(" ");
            join(a99, a100)
        }
        Adt92::C11(v15) => {
            let a101 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a102 = Str::lit(" ");
            join(a101, a102)
        }
        Adt92::C12(v16, v17) => {
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

pub fn f34(v0: Adt21) -> R<Result<Adt93, Adt84>> {
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
                                    Ok(Err::<Adt93, Adt84>(Adt84::C9(Str::lit("not an input-required tool"))))
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
                                                    if f85(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt49, Adt84>(f86(List::cons(f87(Str::lit("Streaming complete")), List::<Adt38>::nil()))))
                                                    } else {
                                                        if f85(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt49, Adt84>(f86(List::cons(f87(Str::lit("Logging evaluated")), List::<Adt38>::nil()))))
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
    let m132 = f111(Str::lit("user_context"), Adt94::C0, match v0 {
        Adt23::C0(_, _, _, h131, _) => {
            h131
        }
    });
    match m132 {
        Adt96::C0 => {
            Ok::<Adt32, Adt84>(Adt32::C12(Adt81::C0(f114(Str::lit("user_context"), f115(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt96::C1 => {
            Err::<Adt32, Adt84>(Adt84::C11)
        }
        Adt96::C2(v1) => {
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

pub fn f46(v0: Bytes, v1: Adt90, v2: Adt90, v3: Option<u64>) -> R<Result<Adt86, Adt88>> {
    let m156 = v3;
    match m156 {
        None => {
            Ok(Err::<Adt86, Adt88>(Adt88::C0(match v2 {
                Adt90::C0(_, h157) => {
                    h157
                }
            })))
        }
        Some(v4) => {
            let m161 = f121(v0.clone(), {
                let a159 = match v2.clone() {
                    Adt90::C0(_, h158) => {
                        h158
                    }
                };
                let a160 = 4u64;
                nat_add(a159, a160)?
            }, v4, List::<Adt87>::nil())?;
            match m161 {
                Ok(v6) => {
                    f122(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt86, Adt88>(v5))
                }
            }
        }
    }
}

pub fn f47(p166: List<Adt87>, p167: List<u64>) -> List<u64> {
    let m165 = (p166, p167);
    {
        let mut m165 = m165;
        loop {
            let step = {
                let (v0, v1) = m165;
                let m162 = v0;
                match m162.uncons() {
                    None => {
                        Err::<(List<Adt87>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt87>, List<u64>), List<u64>>((v3, {
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

pub fn f48(p170: List<Adt87>, p171: List<Adt87>) -> List<Adt87> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v0, v1) = m169;
                let m168 = v0;
                match m168.uncons() {
                    None => {
                        Err::<(List<Adt87>, List<Adt87>), List<Adt87>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt87>, List<Adt87>), List<Adt87>>((v3, List::cons(v2, v1)))
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

pub fn f56(v0: Adt91) -> R<Adt89> {
    f125(f126(v0.clone()), f127(None::<Adt95>, f128(v0)))
}

pub fn f57(v0: Adt0, v1: Fn0, v2: Adt86) -> R<Adt89> {
    if f129(match v2.clone() {
        Adt86::C0(h182, _, _, _, _, _, _) => {
            h182
        }
    }) {
        f130(f131(v0, match v2.clone() {
            Adt86::C0(_, _, _, _, _, _, h183) => {
                h183
            }
        }), v1, match v2.clone() {
            Adt86::C0(_, _, _, _, h184, _, _) => {
                h184
            }
        }, match v2.clone() {
            Adt86::C0(_, _, h185, _, _, _, _) => {
                h185
            }
        }, f132(match v2 {
            Adt86::C0(_, _, _, h186, _, _, _) => {
                h186
            }
        })?)
    } else {
        Ok(f133())
    }
}

pub fn f58(v0: List<Adt87>, v1: List<List<u64>>) -> R<Option<Adt91>> {
    let m187 = f134(v0, f135())?;
    match m187 {
        Adt100::C0 => {
            Ok(None::<Adt91>)
        }
        Adt100::C1 => {
            Ok(Some(Adt91::C0))
        }
        Adt100::C2 => {
            Ok(Some(Adt91::C0))
        }
        Adt100::C3(v2) => {
            f136(f137(v2), v1, Adt91::C0)
        }
    }
}

pub fn f59(v0: List<Adt87>, v1: List<List<u64>>) -> R<Option<Adt91>> {
    let m188 = f134(v0, f138())?;
    match m188 {
        Adt100::C0 => {
            Ok(None::<Adt91>)
        }
        Adt100::C1 => {
            Ok(Some(Adt91::C1))
        }
        Adt100::C2 => {
            Ok(Some(Adt91::C1))
        }
        Adt100::C3(v2) => {
            f136(f139(v2), v1, Adt91::C1)
        }
    }
}

pub fn f60(v0: List<Adt35>) -> Option<Adt92> {
    let m189 = f140(v0, List::<Str>::nil());
    match m189 {
        None => {
            None::<Adt92>
        }
        Some(v1) => {
            Some(Adt92::C1(v1))
        }
    }
}

pub fn f61(p193: List<Adt35>, p194: List<Adt12>) -> R<Result<List<Adt12>, Adt92>> {
    let m192 = (p193, p194);
    {
        let mut m192 = m192;
        loop {
            let step = {
                let (v0, v1) = m192;
                let m190 = v0;
                match m190.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt92>>>(Ok(Ok::<List<Adt12>, Adt92>(f141(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m191 = f142(v2)?;
                        match m191 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt92>>>((v3, f141(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt92>>>(Ok(Err::<List<Adt12>, Adt92>(v4)))
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
        Adt102::C0 => {
            let c208: Fn3 = v2;
            c208.apply(v1)
        }
        Adt102::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f148(f149(v3, v4)?, true))))
        }
        Adt102::C2(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt102::C3(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt102::C4 => {
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

pub fn f76(v0: Adt21) -> Result<Adt93, Adt84> {
    let m214 = f111(Str::lit("user_name"), Adt94::C0, match v0 {
        Adt21::C0(_, _, _, h213, _) => {
            h213
        }
    });
    match m214 {
        Adt96::C0 => {
            f153(f114(Str::lit("user_name"), f154()))
        }
        Adt96::C1 => {
            f155()
        }
        Adt96::C2(v1) => {
            f151(f152(List::cons(Str::lit("Hello, "), List::cons(f113(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f77(v0: Adt21) -> Result<Adt93, Adt84> {
    let m216 = f111(Str::lit("capital_question"), Adt94::C1, match v0 {
        Adt21::C0(_, _, _, h215, _) => {
            h215
        }
    });
    match m216 {
        Adt96::C0 => {
            f153(f114(Str::lit("capital_question"), f156(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt96::C1 => {
            f155()
        }
        Adt96::C2(v1) => {
            f151(f152(List::cons(Str::lit("Sampling result: "), List::cons(f113(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f78(v0: Adt21) -> R<Result<Adt93, Adt84>> {
    let m218 = f111(Str::lit("client_roots"), Adt94::C2, match v0 {
        Adt21::C0(_, _, _, h217, _) => {
            h217
        }
    });
    match m218 {
        Adt96::C0 => {
            Ok(f153(f114(Str::lit("client_roots"), f159())))
        }
        Adt96::C1 => {
            Ok(f155())
        }
        Adt96::C2(v1) => {
            Ok(f151(f152(List::cons(Str::lit("Found "), List::cons(f157(f158(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f79(v0: Adt21) -> Result<Adt93, Adt84> {
    f160(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f80(v0: Adt21) -> R<Result<Adt93, Adt84>> {
    if f161(v0.clone(), Str::lit("multiple-inputs")) {
        f162(f111(Str::lit("user_name"), Adt94::C0, match v0.clone() {
            Adt21::C0(_, _, _, h219, _) => {
                h219
            }
        }), f111(Str::lit("greeting"), Adt94::C1, match v0.clone() {
            Adt21::C0(_, _, _, h220, _) => {
                h220
            }
        }), f111(Str::lit("client_roots"), Adt94::C2, match v0 {
            Adt21::C0(_, _, _, h221, _) => {
                h221
            }
        }))
    } else {
        Ok(f163(f164(Adt96::C0, Adt96::C0, Adt96::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f81(v0: Adt21) -> R<Result<Adt93, Adt84>> {
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

pub fn f82(v0: Adt21) -> Result<Adt93, Adt84> {
    f160(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f83(v0: Adt21) -> Result<Adt93, Adt84> {
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

pub fn f111(v0: Str, v1: Adt94, v2: Option<Adt5>) -> Adt96 {
    let m263 = v2;
    match m263 {
        None => {
            Adt96::C0
        }
        Some(v3) => {
            let m264 = f186(v0, v3);
            match m264 {
                Adt107::C0 => {
                    Adt96::C0
                }
                Adt107::C1(v4) => {
                    let m265 = f187(v4);
                    match m265 {
                        None => {
                            Adt96::C1
                        }
                        Some(v5) => {
                            if f188(v1, v5.clone()) {
                                Adt96::C2(v5)
                            } else {
                                Adt96::C1
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
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
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

pub fn f121(v0: Bytes, p373: u64, p374: u64, p375: List<Adt87>) -> R<Result<(List<Adt87>, u64), Adt88>> {
    let m372 = (p373, (p374, p375));
    {
        let mut m372 = m372;
        loop {
            let step = {
                let (v1, (v2, v3)) = m372;
                let m364 = v2;
                if m364 == 0 {
                    Err::<(u64, (u64, List<Adt87>)), R<Result<(List<Adt87>, u64), Adt88>>>(Ok(Ok::<(List<Adt87>, u64), Adt88>((f205(v3, List::<Adt87>::nil()), v1))))
                } else {
                    let v4 = m364 - 1;
                    let m365 = f16(v0.clone(), v1)?;
                    match m365 {
                        None => {
                            Err::<(u64, (u64, List<Adt87>)), R<Result<(List<Adt87>, u64), Adt88>>>(Ok(Err::<(List<Adt87>, u64), Adt88>(Adt88::C0(v1))))
                        }
                        Some(v5) => {
                            let m367 = f16(v0.clone(), match v5.clone() {
                                Adt90::C0(_, h366) => {
                                    h366
                                }
                            })?;
                            match m367 {
                                None => {
                                    Err::<(u64, (u64, List<Adt87>)), R<Result<(List<Adt87>, u64), Adt88>>>(Ok(Err::<(List<Adt87>, u64), Adt88>(Adt88::C0(match v5 {
                                        Adt90::C0(_, h368) => {
                                            h368
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt87>)), R<Result<(List<Adt87>, u64), Adt88>>>((match v6.clone() {
                                        Adt90::C0(_, h369) => {
                                            h369
                                        }
                                    }, (v4, List::cons(Adt87::C0(f206(match v5 {
                                        Adt90::C0(h370, _) => {
                                            h370
                                        }
                                    })?, f206(match v6 {
                                        Adt90::C0(h371, _) => {
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

pub fn f122(v0: Bytes, v1: Adt90, v2: Adt90, v3: (List<Adt87>, u64)) -> R<Result<Adt86, Adt88>> {
    let m377 = f16(v0.clone(), {
        let (_, h376) = v3.clone();
        h376
    })?;
    match m377 {
        None => {
            Ok(Err::<Adt86, Adt88>(Adt88::C0({
                let (_, h378) = v3;
                h378
            })))
        }
        Some(v4) => {
            let m380 = f45(v0.clone(), match v4.clone() {
                Adt90::C0(_, h379) => {
                    h379
                }
            })?;
            match m380 {
                None => {
                    Ok(Err::<Adt86, Adt88>(Adt88::C0(match v4 {
                        Adt90::C0(_, h381) => {
                            h381
                        }
                    })))
                }
                Some(v5) => {
                    let m385 = f207(v0.clone(), {
                        let a383 = match v4.clone() {
                            Adt90::C0(_, h382) => {
                                h382
                            }
                        };
                        let a384 = 4u64;
                        nat_add(a383, a384)?
                    })?;
                    match m385 {
                        None => {
                            Ok(Err::<Adt86, Adt88>(Adt88::C0({
                                let a387 = match v4 {
                                    Adt90::C0(_, h386) => {
                                        h386
                                    }
                                };
                                let a388 = 4u64;
                                nat_add(a387, a388)?
                            })))
                        }
                        Some(v6) => {
                            f208(v0, Adt86::C0(f206(match v1 {
                                Adt90::C0(h389, _) => {
                                    h389
                                }
                            })?, f206(match v2 {
                                Adt90::C0(h390, _) => {
                                    h390
                                }
                            })?, {
                                let (h391, _) = v3;
                                h391
                            }, match v4 {
                                Adt90::C0(h392, _) => {
                                    h392
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt90::C0(h393, _) => {
                                    h393
                                }
                            }), match v6 {
                                Adt90::C0(_, h394) => {
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

pub fn f123(v0: Adt87) -> List<u64> {
    let a405 = f18({
        let a396 = match v0.clone() {
            Adt87::C0(h395, _) => {
                h395
            }
        };
        length_list(a396)
    });
    let a406 = {
        let a403 = match v0.clone() {
            Adt87::C0(h397, _) => {
                h397
            }
        };
        let a404 = {
            let a401 = f18({
                let a399 = match v0.clone() {
                    Adt87::C0(_, h398) => {
                        h398
                    }
                };
                length_list(a399)
            });
            let a402 = match v0 {
                Adt87::C0(_, h400) => {
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

pub fn f125(v0: u64, v1: Adt6) -> R<Adt89> {
    let m410 = f209(v1)?;
    match m410 {
        Ok(v3) => {
            Ok(Adt89::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f126(v0: Adt91) -> u64 {
    let m411 = v0;
    match m411 {
        Adt91::C0 => {
            f210()
        }
        Adt91::C1 => {
            f210()
        }
    }
}

pub fn f127(v0: Option<Adt95>, v1: Adt80) -> Adt6 {
    Adt6::C5(f211(v0, v1))
}

pub fn f128(v0: Adt91) -> Adt80 {
    Adt80::C0(f212(), f213(v0), None::<Adt6>)
}

pub fn f129(v0: List<u64>) -> bool {
    f214(v0, f215())
}

pub fn f130(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt87>, v4: Result<Adt6, Adt97>) -> R<Adt89> {
    let m412 = v4;
    match m412 {
        Ok(v6) => {
            f218(v0, v1, v2, v3, f219(v6)?)
        }
        Err(_) => {
            f125(f216(), f127(None::<Adt95>, f217(Adt98::C0)))
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

pub fn f132(v0: Bytes) -> R<Result<Adt6, Adt97>> {
    f220(f206(v0)?)
}

pub fn f133() -> Adt89 {
    Adt89::C0(f221(), List::cons(Adt87::C0(f222(), f223()), List::<Adt87>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f134(v0: List<Adt87>, v1: List<u64>) -> R<Adt100> {
    let m419 = f224(v0, v1, Adt101::C0)?;
    match m419 {
        Adt101::C0 => {
            Ok(Adt100::C0)
        }
        Adt101::C1(v2) => {
            let m420 = f225(v2);
            match m420 {
                None => {
                    Ok(Adt100::C1)
                }
                Some(v3) => {
                    Ok(Adt100::C3(v3))
                }
            }
        }
        Adt101::C2 => {
            Ok(Adt100::C2)
        }
    }
}

pub fn f135() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f136(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt91) -> R<Option<Adt91>> {
    let m421 = v0;
    match m421 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f226(v3, v1)? {
                Ok(None::<Adt91>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f137(v0: List<u64>) -> Option<List<u64>> {
    let m422 = f227(v0);
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
                f228(f229(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f228(f230(v0, List::<u64>::nil()))
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
                        if f231(match v2.clone() {
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

pub fn f142(v0: Adt35) -> R<Result<List<Adt12>, Adt92>> {
    let m439 = f232(match v0.clone() {
        Adt35::C0(h437, _, _, _, _, _, _, _) => {
            h437
        }
    }, f233(match v0.clone() {
        Adt35::C0(h438, _, _, _, _, _, _, _) => {
            h438
        }
    })?);
    match m439 {
        None => {
            f234(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt92>(v1))
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
    Ok(Adt6::C2(Adt7::C0(f235(v0, 0i64)?, 0i64)))
}

pub fn f146(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt102> {
    f236(v0, f237(), Adt104::C0, List::cons(Adt105::C0(v1, v2, List::<Adt103>::nil()), List::<Adt105>::nil()))
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

pub fn f149(v0: List<Adt103>, v1: Str) -> R<Str> {
    let v2: Str = f238(v0)?;
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

pub fn f150(v0: Adt35, v1: Result<Adt93, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m447 = v1;
    match m447 {
        Ok(v3) => {
            let m448 = v3;
            match m448 {
                Adt93::C0(v4) => {
                    f239(v0, v4)
                }
                Adt93::C1(v5) => {
                    Ok(Ok::<Adt32, Adt84>(Adt32::C12(v5)))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt32, Adt84>(v2))
        }
    }
}

pub fn f151(v0: Str) -> Result<Adt93, Adt84> {
    Ok::<Adt93, Adt84>(Adt93::C0(Adt49::C0(Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, None::<bool>), List::<Adt50>::nil())))
}

pub fn f152(v0: List<Str>) -> Str {
    let a449 = v0;
    let a450 = Str::lit("");
    join(a449, a450)
}

pub fn f153(v0: List<(Str, Adt64)>) -> Result<Adt93, Adt84> {
    Ok::<Adt93, Adt84>(Adt93::C1(Adt81::C0(v0)))
}

pub fn f154() -> Adt64 {
    f115(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f155() -> Result<Adt93, Adt84> {
    Err::<Adt93, Adt84>(Adt84::C11)
}

pub fn f156(v0: Str, v1: u64) -> Adt64 {
    Adt64::C1(Adt69::C0(List::cons(f240(v0), List::<Adt70>::nil()), None::<Adt75>, None::<Str>, None::<Adt77>, None::<Adt7>, v1, None::<List<Str>>, None::<Adt5>, None::<List<Adt35>>, None::<Adt78>))
}

pub fn f157(v0: u64) -> R<Str> {
    let a451 = f241(v0, 0i64)?;
    Ok(format_i64(a451))
}

pub fn f158(v0: Adt5) -> u64 {
    let m452 = f186(Str::lit("roots"), v0);
    match m452 {
        Adt107::C0 => {
            0u64
        }
        Adt107::C1(v1) => {
            let m453 = f242(v1);
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

pub fn f160(v0: Adt21, v1: Str, v2: Str) -> Result<Adt93, Adt84> {
    if f161(v0.clone(), v1.clone()) {
        let m456 = f111(Str::lit("confirm"), Adt94::C0, match v0 {
            Adt21::C0(_, _, _, h455, _) => {
                h455
            }
        });
        match m456 {
            Adt96::C0 => {
                f163(f114(Str::lit("confirm"), f244()), v1)
            }
            Adt96::C1 => {
                f155()
            }
            Adt96::C2(v3) => {
                if f243(v3) {
                    f151(v2)
                } else {
                    f163(f114(Str::lit("confirm"), f244()), v1)
                }
            }
        }
    } else {
        f163(f114(Str::lit("confirm"), f244()), v1)
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

pub fn f162(v0: Adt96, v1: Adt96, v2: Adt96) -> R<Result<Adt93, Adt84>> {
    if if f245(v0.clone()) {
        true
    } else {
        if f245(v1.clone()) {
            true
        } else {
            f245(v2.clone())
        }
    } {
        Ok(f155())
    } else {
        let m461 = f164(v0.clone(), v1.clone(), v2.clone());
        match m461.uncons() {
            None => {
                Ok(f151(f152(List::cons(Str::lit("Name: "), List::cons(f113(f246(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f113(f246(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f157(f158(f246(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f163(f164(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f163(v0: List<(Str, Adt64)>, v1: Str) -> Result<Adt93, Adt84> {
    Ok::<Adt93, Adt84>(Adt93::C1(Adt81::C2(v0, v1)))
}

pub fn f164(v0: Adt96, v1: Adt96, v2: Adt96) -> List<(Str, Adt64)> {
    let a464 = if f247(v0) {
        f114(Str::lit("user_name"), f154())
    } else {
        List::<(Str, Adt64)>::nil()
    };
    let a465 = {
        let a462 = if f247(v1) {
            f114(Str::lit("greeting"), f156(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt64)>::nil()
        };
        let a463 = if f247(v2) {
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

pub fn f167(v0: Adt21) -> Result<Adt93, Adt84> {
    let m470 = f111(Str::lit("step1"), Adt94::C0, match v0 {
        Adt21::C0(_, _, _, h469, _) => {
            h469
        }
    });
    match m470 {
        Adt96::C0 => {
            f163(f114(Str::lit("step1"), f169()), Str::lit("1"))
        }
        Adt96::C1 => {
            f155()
        }
        Adt96::C2(v1) => {
            f163(f114(Str::lit("step2"), f248()), f152(List::cons(Str::lit("2"), List::cons(f113(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f168(v0: Adt21) -> Result<Adt93, Adt84> {
    let m472 = f111(Str::lit("step2"), Adt94::C0, match v0.clone() {
        Adt21::C0(_, _, _, h471, _) => {
            h471
        }
    });
    match m472 {
        Adt96::C0 => {
            f163(f114(Str::lit("step2"), f248()), f166(v0))
        }
        Adt96::C1 => {
            f155()
        }
        Adt96::C2(v1) => {
            f151(f152(List::cons(Str::lit("Multi-round complete for "), List::cons(f249(f166(v0)), List::cons(Str::lit(" who likes "), List::cons(f113(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f169() -> Adt64 {
    f115(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f170(v0: List<(Str, Adt64)>) -> Result<Adt93, Adt84> {
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
    let a476 = if f250(match v0.clone() {
        Adt17::C0(_, _, _, h474, _) => {
            h474
        }
    }) {
        f114(Str::lit("elicit_input"), f115(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt64)>::nil()
    };
    let a477 = if f250(match v0 {
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
    f251(f252(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f173(v0: Fn2, v1: Adt35, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    f253(v1, v0, v2)
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
    Ok(Adt50::C0(f254(v0)?, Some(f254(v1)?), Some({
        let a483 = List::cons(Str::lit("Completed step"), List::cons({
            let a481 = f241(v0, 0i64)?;
            format_i64(a481)
        }, List::cons(Str::lit("of"), List::cons({
            let a482 = f241(v1, 0i64)?;
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
    f255(f256(v0, List::<Adt44>::nil()), List::<Adt44>::nil())
}

pub fn f179() -> List<(Adt44, List<Adt46>)> {
    List::cons(f257(f258(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f198(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f257(f258(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f259(Str::lit("test://static-binary"), Str::lit("image/png"), f199())), List::cons(f257(f258(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f198(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f257(f258(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f198(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt44, List<Adt46>)>::nil()))))
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
    let m496 = f260(v0.clone())?;
    match m496 {
        None => {
            Ok(None::<List<Adt46>>)
        }
        Some(v1) => {
            let m497 = f261(v1)?;
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

pub fn f186(v0: Str, v1: Adt5) -> Adt107 {
    f262(v0, match v1 {
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

pub fn f188(v0: Adt94, v1: Adt5) -> bool {
    let m500 = v0;
    match m500 {
        Adt94::C0 => {
            f263(v1)
        }
        Adt94::C1 => {
            f264(v1)
        }
        Adt94::C2 => {
            f265(v1)
        }
    }
}

pub fn f189(v0: Str, v1: Adt5, v2: Str) -> Str {
    let m501 = f186(v0, v1);
    match m501 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            let m502 = f266(v3);
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
                        if f267(v2.clone()) {
                            let m511 = f268(match v2.clone() {
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
    let m518 = f268(v0, v1);
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
    let v1: u64 = f269(v0.clone(), 0u64)?;
    Ok(Adt60::C0(Adt61::C0(f270(v0), Some(v1), Some({
        let a519 = f271();
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
    Ok(f251(f272(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
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
        f273()
    } else {
        List::<Str>::nil()
    }
}

pub fn f204() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f205(p533: List<Adt87>, p534: List<Adt87>) -> List<Adt87> {
    let m532 = (p533, p534);
    {
        let mut m532 = m532;
        loop {
            let step = {
                let (v0, v1) = m532;
                let m531 = v0;
                match m531.uncons() {
                    None => {
                        Err::<(List<Adt87>, List<Adt87>), List<Adt87>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt87>, List<Adt87>), List<Adt87>>((v3, List::cons(v2, v1)))
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
    f274(v0.clone(), {
        let a535 = v0;
        length_bytes(a535)
    }, List::<u64>::nil())
}

pub fn f207(v0: Bytes, v1: u64) -> R<Option<Adt90>> {
    let a537 = v1;
    let a538 = {
        let a536 = v0.clone();
        length_bytes(a536)
    };
    if nat_eq(a537, a538) {
        Ok(Some(Adt90::C0(Bytes::lit(&[]), v1)))
    } else {
        f16(v0, v1)
    }
}

pub fn f208(v0: Bytes, v1: Adt86, v2: u64) -> R<Result<Adt86, Adt88>> {
    let a540 = v2;
    let a541 = {
        let a539 = v0.clone();
        length_bytes(a539)
    };
    if nat_eq(a540, a541) {
        Ok(Ok::<Adt86, Adt88>(v1))
    } else {
        let m542 = f45(v0.clone(), v2)?;
        match m542 {
            None => {
                Ok(Err::<Adt86, Adt88>(Adt88::C0(v2)))
            }
            Some(v3) => {
                let m545 = f275(v0.clone(), {
                    let a543 = v2;
                    let a544 = 4u64;
                    nat_add(a543, a544)?
                }, v3, List::<List<u64>>::nil())?;
                match m545 {
                    Ok(v5) => {
                        let a548 = {
                            let (_, h546) = v5.clone();
                            h546
                        };
                        let a549 = {
                            let a547 = v0;
                            length_bytes(a547)
                        };
                        if nat_eq(a548, a549) {
                            Ok(Ok::<Adt86, Adt88>(f276(v1, {
                                let (h550, _) = v5;
                                h550
                            })))
                        } else {
                            Ok(Err::<Adt86, Adt88>(Adt88::C1({
                                let (_, h551) = v5;
                                h551
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt86, Adt88>(v4))
                    }
                }
            }
        }
    }
}

pub fn f209(v0: Adt6) -> R<Result<List<u64>, Adt97>> {
    f277(f278(), v0)
}

pub fn f210() -> u64 {
    403u64
}

pub fn f211(v0: Option<Adt95>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f279())), {
        let a552 = f280(v0);
        let a553 = List::cons((Str::lit("error"), f281(v1)), List::<(Str, Adt6)>::nil());
        append_list(a552, a553)
    })
}

pub fn f212() -> i64 {
    -32000i64
}

pub fn f213(v0: Adt91) -> Str {
    let m554 = v0;
    match m554 {
        Adt91::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt91::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f214(p561: List<u64>, p562: List<u64>) -> bool {
    let m560 = (p561, p562);
    {
        let mut m560 = m560;
        loop {
            let step = {
                let (v0, v1) = m560;
                let m555 = v0;
                match m555.uncons() {
                    None => {
                        let m556 = v1;
                        match m556.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m557 = v1;
                        match m557.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a558 = v4;
                                let a559 = v6;
                                if nat_eq(a558, a559) {
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
                Ok(next) => { m560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f216() -> u64 {
    400u64
}

pub fn f217(v0: Adt98) -> Adt80 {
    Adt80::C0(f282(v0.clone()), f283(v0), None::<Adt6>)
}

pub fn f218(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt87>, v4: Result<Adt108, Adt98>) -> R<Adt89> {
    let m563 = v4;
    match m563 {
        Ok(v6) => {
            f286(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f125(f284(v5.clone()), f127(f285(v5.clone()), f217(v5)))
        }
    }
}

pub fn f219(v0: Adt6) -> R<Result<Adt108, Adt98>> {
    let m564 = v0;
    match m564 {
        Adt6::C0 => {
            Ok(f288(None::<Adt95>, Adt99::C0))
        }
        Adt6::C1(_) => {
            Ok(f288(None::<Adt95>, Adt99::C0))
        }
        Adt6::C2(_) => {
            Ok(f288(None::<Adt95>, Adt99::C0))
        }
        Adt6::C3(_) => {
            Ok(f288(None::<Adt95>, Adt99::C0))
        }
        Adt6::C4(_) => {
            Ok(f288(None::<Adt95>, Adt99::C1))
        }
        Adt6::C5(v1) => {
            f287(Adt5::C0(v1))
        }
    }
}

pub fn f220(v0: List<u64>) -> R<Result<Adt6, Adt97>> {
    let m565 = f289(v0.clone())?;
    match m565 {
        Adt117::C0 => {
            f290(f291(), 0u64, v0)
        }
        Adt117::C1(v1) => {
            Ok(Err::<Adt6, Adt97>(Adt97::C0(v1)))
        }
        Adt117::C2(v2) => {
            Ok(Err::<Adt6, Adt97>(Adt97::C1(v2)))
        }
    }
}

pub fn f221() -> u64 {
    405u64
}

pub fn f222() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f223() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f224(p570: List<Adt87>, v1: List<u64>, p571: Adt101) -> R<Adt101> {
    let m569 = (p570, p571);
    {
        let mut m569 = m569;
        loop {
            let step = {
                let (v0, v2) = m569;
                let m566 = v0;
                match m566.uncons() {
                    None => {
                        Err::<(List<Adt87>, Adt101), R<Adt101>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f292(match v3.clone() {
                            Adt87::C0(h567, _) => {
                                h567
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt87>, Adt101), R<Adt101>>((v4, f293(v2, match v3 {
                                Adt87::C0(_, h568) => {
                                    h568
                                }
                            })))
                        } else {
                            Ok::<(List<Adt87>, Adt101), R<Adt101>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m569 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f294(v0);
    if f295(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f226(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f292(v0.clone(), f296())? {
        Ok(true)
    } else {
        if f292(v0.clone(), f297())? {
            Ok(true)
        } else {
            if f292(v0.clone(), f298())? {
                Ok(true)
            } else {
                f299(v0, v1)
            }
        }
    }
}

pub fn f227(p576: List<u64>) -> Option<List<u64>> {
    let m575 = p576;
    {
        let mut m575 = m575;
        loop {
            let step = {
                let v0 = m575;
                let m572 = v0;
                match m572.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a573 = v1;
                        let a574 = 58u64;
                        if nat_eq(a573, a574) {
                            Err::<List<u64>, Option<List<u64>>>(f300(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
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

pub fn f228(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m577 = v0;
    match m577 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f301({
                let (_, h578) = v1;
                h578
            }) {
                Some({
                    let (h579, _) = v1;
                    h579
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f229(p584: List<u64>, p585: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m583 = (p584, p585);
    {
        let mut m583 = m583;
        loop {
            let step = {
                let (v0, v1) = m583;
                let m580 = v0;
                match m580.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a581 = v2;
                        let a582 = 93u64;
                        if nat_eq(a581, a582) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f302(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f230(p590: List<u64>, p591: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m589 = (p590, p591);
    {
        let mut m589 = m589;
        loop {
            let step = {
                let (v0, v1) = m589;
                let m586 = v0;
                match m586.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f302(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a587 = v2;
                        let a588 = 58u64;
                        if nat_eq(a587, a588) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f302(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f231(v0: Str, p596: List<Str>) -> bool {
    let m595 = p596;
    {
        let mut m595 = m595;
        loop {
            let step = {
                let v1 = m595;
                let m592 = v1;
                match m592.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a593 = v0.clone();
                        let a594 = v2;
                        if equal(a593, a594) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Str, v1: List<u64>) -> Option<Adt92> {
    if f303(v1) {
        Some(Adt92::C0(v0))
    } else {
        None::<Adt92>
    }
}

pub fn f233(v0: Str) -> R<List<u64>> {
    f206({
        let a597 = v0;
        utf8_encode(a597)
    })
}

pub fn f234(v0: Adt35) -> R<Result<List<Adt12>, Adt92>> {
    let m598 = f304(v0.clone());
    match m598 {
        None => {
            f305(v0.clone(), f306(f307(), match v0.clone() {
                Adt35::C0(h599, _, _, _, _, _, _, _) => {
                    h599
                }
            }, Adt6::C5({
                let m602 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h601, _, _, _) => {
                        h601
                    }
                };
                match m602 {
                    Adt5::C0(h600) => {
                        h600
                    }
                }
            }), List::cons(Adt109::C0(Adt6::C5({
                let m605 = match v0 {
                    Adt35::C0(_, _, _, _, h604, _, _, _) => {
                        h604
                    }
                };
                match m605 {
                    Adt5::C0(h603) => {
                        h603
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt109>::nil()), f308())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt92>(v1))
        }
    }
}

pub fn f235(p610: u64, p611: i64) -> R<i64> {
    let m609 = (p610, p611);
    {
        let mut m609 = m609;
        loop {
            let step = {
                let (v0, v1) = m609;
                let m606 = v0;
                if m606 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m606 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a607 = v1;
                        let a608 = -1i64;
                        int_sub(a607, a608)?
                    }))
                }
            };
            match step {
                Ok(next) => { m609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Adt6, p630: u64, p631: Adt104, p632: List<Adt105>) -> R<Adt102> {
    let m629 = (p630, (p631, p632));
    {
        let mut m629 = m629;
        loop {
            let step = {
                let (v1, (v2, v3)) = m629;
                let m612 = v1;
                if m612 == 0 {
                    Err::<(u64, (Adt104, List<Adt105>)), R<Adt102>>(Ok(Adt102::C4))
                } else {
                    let v4 = m612 - 1;
                    let m613 = v3;
                    match m613.uncons() {
                        None => {
                            let m614 = v2;
                            match m614 {
                                Adt104::C0 => {
                                    Err::<(u64, (Adt104, List<Adt105>)), R<Adt102>>(Ok(Adt102::C0))
                                }
                                Adt104::C1(v5, v6) => {
                                    Err::<(u64, (Adt104, List<Adt105>)), R<Adt102>>(Ok(Adt102::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m615 = v2.clone();
                            match m615 {
                                Adt104::C0 => {
                                    let m616 = v7.clone();
                                    match m616 {
                                        Adt105::C0(v9, v10, v11) => {
                                            let m617 = f309(v0.clone(), v9, v10, v11.clone())?;
                                            match m617 {
                                                Adt126::C0(v12) => {
                                                    Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (Adt104::C1(v11, v12), v8)))
                                                }
                                                Adt126::C1(v13) => {
                                                    Err::<(u64, (Adt104, List<Adt105>)), R<Adt102>>(Ok(Adt102::C2(v13)))
                                                }
                                                Adt126::C2(v14) => {
                                                    Err::<(u64, (Adt104, List<Adt105>)), R<Adt102>>(Ok(Adt102::C3(v14)))
                                                }
                                                Adt126::C3(v15) => {
                                                    let v16: List<Adt105> = {
                                                        let a618 = v15;
                                                        let a619 = v8;
                                                        append_list(a618, a619)
                                                    };
                                                    Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt105::C1(_, _, _) => {
                                            Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (v2, v8)))
                                        }
                                        Adt105::C2(_, _, _, _) => {
                                            let v24: Adt110 = f310(v7, v8)?;
                                            Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (match v24.clone() {
                                                Adt110::C0(h620, _) => {
                                                    h620
                                                }
                                            }, match v24 {
                                                Adt110::C0(_, h621) => {
                                                    h621
                                                }
                                            })))
                                        }
                                        Adt105::C3(v25) => {
                                            Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (Adt104::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt105::C4(_, _, _, _) => {
                                            let v39: Adt110 = f310(v7, v8)?;
                                            Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (match v39.clone() {
                                                Adt110::C0(h625, _) => {
                                                    h625
                                                }
                                            }, match v39 {
                                                Adt110::C0(_, h626) => {
                                                    h626
                                                }
                                            })))
                                        }
                                        Adt105::C5(v26, v27, v28, v29) => {
                                            let m622 = f311(v26, v27, v28, v29.clone());
                                            match m622 {
                                                Adt126::C0(v30) => {
                                                    Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (Adt104::C1(v29, v30), v8)))
                                                }
                                                Adt126::C1(v31) => {
                                                    Err::<(u64, (Adt104, List<Adt105>)), R<Adt102>>(Ok(Adt102::C2(v31)))
                                                }
                                                Adt126::C2(v32) => {
                                                    Err::<(u64, (Adt104, List<Adt105>)), R<Adt102>>(Ok(Adt102::C3(v32)))
                                                }
                                                Adt126::C3(v33) => {
                                                    let v34: List<Adt105> = {
                                                        let a623 = v33;
                                                        let a624 = v8;
                                                        append_list(a623, a624)
                                                    };
                                                    Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt104::C1(v40, v41) => {
                                    let v42: Adt110 = f312(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt104, List<Adt105>)), R<Adt102>>((v4, (match v42.clone() {
                                        Adt110::C0(h627, _) => {
                                            h627
                                        }
                                    }, match v42 {
                                        Adt110::C0(_, h628) => {
                                            h628
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m629 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237() -> u64 {
    67108864u64
}

pub fn f238(v0: List<Adt103>) -> R<Str> {
    let a633 = List::cons(Str::lit(""), f313(v0, List::<Str>::nil())?);
    let a634 = Str::lit("/");
    Ok(join(a633, a634))
}

pub fn f239(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f314({
        let m637 = match v1.clone() {
            Adt49::C0(h636, _) => {
                h636
            }
        };
        match m637 {
            Adt37::C0(h635, _, _) => {
                h635
            }
        }
    })? {
        f315(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f240(v0: Str) -> Adt70 {
    Adt70::C0(Adt41::C0, Adt71::C0(Adt72::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))), None::<Adt5>)
}

pub fn f241(p640: u64, p641: i64) -> R<i64> {
    let m639 = (p640, p641);
    {
        let mut m639 = m639;
        loop {
            let step = {
                let (v0, v1) = m639;
                let m638 = v0;
                if m638 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m638 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f316(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt6) -> Option<List<Adt6>> {
    let m642 = v0;
    match m642 {
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

pub fn f243(v0: Adt5) -> bool {
    let m643 = f186(Str::lit("content"), v0);
    match m643 {
        Adt107::C0 => {
            false
        }
        Adt107::C1(v1) => {
            let m644 = f187(v1);
            match m644 {
                None => {
                    false
                }
                Some(v2) => {
                    let m645 = f186(Str::lit("ok"), v2);
                    match m645 {
                        Adt107::C0 => {
                            false
                        }
                        Adt107::C1(v3) => {
                            let m646 = f317(v3);
                            match m646 {
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

pub fn f244() -> Adt64 {
    f115(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f245(v0: Adt96) -> bool {
    let m647 = v0;
    match m647 {
        Adt96::C0 => {
            false
        }
        Adt96::C1 => {
            true
        }
        Adt96::C2(_) => {
            false
        }
    }
}

pub fn f246(v0: Adt96) -> Adt5 {
    let m648 = v0;
    match m648 {
        Adt96::C0 => {
            f26()
        }
        Adt96::C1 => {
            f26()
        }
        Adt96::C2(v1) => {
            v1
        }
    }
}

pub fn f247(v0: Adt96) -> bool {
    let m649 = v0;
    match m649 {
        Adt96::C0 => {
            true
        }
        Adt96::C1 => {
            false
        }
        Adt96::C2(_) => {
            false
        }
    }
}

pub fn f248() -> Adt64 {
    f115(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f249(v0: Str) -> Str {
    let v1: Bytes = {
        let a650 = v0;
        utf8_encode(a650)
    };
    let m657 = {
        let a654 = v1.clone();
        let a655 = 1u64;
        let a656 = {
            let a652 = {
                let a651 = v1;
                length_bytes(a651)
            };
            let a653 = 1u64;
            nat_sub(a652, a653)
        };
        slice_bytes(a654, a655, a656)
    };
    match m657 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m659 = {
                let a658 = v2;
                utf8_decode(a658)
            };
            match m659 {
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

pub fn f250(v0: Option<Adt5>) -> bool {
    let m660 = v0;
    match m660 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f251(p663: List<Str>, p664: List<Str>) -> List<Str> {
    let m662 = (p663, p664);
    {
        let mut m662 = m662;
        loop {
            let step = {
                let (v0, v1) = m662;
                let m661 = v0;
                match m661.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(p668: List<(Str, Adt6)>, p669: List<Str>) -> List<Str> {
    let m667 = (p668, p669);
    {
        let mut m667 = m667;
        loop {
            let step = {
                let (v0, v1) = m667;
                let m665 = v0;
                match m665.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h666, _) = v2;
                            h666
                        }, v1)))
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

pub fn f253(v0: Adt35, v1: Fn2, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m671 = {
        let c670: Fn2 = v1;
        c670.apply(v2)?
    };
    match m671 {
        Ok(v4) => {
            f239(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f254(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f241(v0, 0i64)?, 0i64))
}

pub fn f255(p674: List<Adt44>, p675: List<Adt44>) -> List<Adt44> {
    let m673 = (p674, p675);
    {
        let mut m673 = m673;
        loop {
            let step = {
                let (v0, v1) = m673;
                let m672 = v0;
                match m672.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt44>), List<Adt44>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt44>), List<Adt44>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m673 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(p679: List<(Adt44, List<Adt46>)>, p680: List<Adt44>) -> List<Adt44> {
    let m678 = (p679, p680);
    {
        let mut m678 = m678;
        loop {
            let step = {
                let (v0, v1) = m678;
                let m676 = v0;
                match m676.uncons() {
                    None => {
                        Err::<(List<(Adt44, List<Adt46>)>, List<Adt44>), List<Adt44>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt44, List<Adt46>)>, List<Adt44>), List<Adt44>>((v3, List::cons({
                            let (h677, _) = v2;
                            h677
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f257(v0: Adt44, v1: Adt46) -> (Adt44, List<Adt46>) {
    (v0, List::cons(v1, List::<Adt46>::nil()))
}

pub fn f258(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt44 {
    Adt44::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt40>, None::<u64>, None::<Adt5>)
}

pub fn f259(v0: Str, v1: Str, v2: Str) -> Adt46 {
    Adt46::C1(Adt48::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f260(v0: Str) -> R<Option<Str>> {
    let m681 = f318(v0)?;
    match m681 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a682 = f319(f320(), v1);
            Ok(utf8_decode(a682))
        }
    }
}

pub fn f261(v0: Str) -> R<Option<Str>> {
    let m685 = f321(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a683 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a684 = Str::lit(" ");
        join(a683, a684)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m685 {
        Ok(v1) => {
            let a686 = v1;
            Ok(utf8_decode(a686))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f262(v0: Str, p692: List<(Str, Adt6)>) -> Adt107 {
    let m691 = p692;
    {
        let mut m691 = m691;
        loop {
            let step = {
                let v1 = m691;
                let m687 = v1;
                match m687.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt107>(Adt107::C0)
                    }
                    Some((v2, v3)) => {
                        let m689 = f322({
                            let (h688, _) = v2;
                            h688
                        }, v0.clone());
                        match m689 {
                            Adt127::C0 => {
                                Err::<List<(Str, Adt6)>, Adt107>(Adt107::C1({
                                    let (_, h690) = v2;
                                    h690
                                }))
                            }
                            Adt127::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt107>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: Adt5) -> bool {
    if f323(v0.clone()) {
        f324(v0)
    } else {
        false
    }
}

pub fn f264(v0: Adt5) -> bool {
    if f325(v0.clone()) {
        if f326(f186(Str::lit("model"), v0.clone())) {
            f327(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f265(v0: Adt5) -> bool {
    let m693 = f186(Str::lit("roots"), v0);
    match m693 {
        Adt107::C0 => {
            false
        }
        Adt107::C1(v1) => {
            let m694 = f242(v1);
            match m694 {
                None => {
                    false
                }
                Some(v2) => {
                    f328(v2)
                }
            }
        }
    }
}

pub fn f266(v0: Adt6) -> Option<Str> {
    let m695 = v0;
    match m695 {
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

pub fn f267(v0: Adt57) -> bool {
    let m697 = match v0 {
        Adt57::C0(_, _, _, h696) => {
            h696
        }
    };
    match m697 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f268(v0: Str, p704: List<(Str, Str)>) -> Option<Str> {
    let m703 = p704;
    {
        let mut m703 = m703;
        loop {
            let step = {
                let v1 = m703;
                let m698 = v1;
                match m698.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a700 = {
                            let (h699, _) = v2;
                            h699
                        };
                        let a701 = v0.clone();
                        if equal(a700, a701) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h702) = v2;
                                h702
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f269(p709: List<Str>, p710: u64) -> R<u64> {
    let m708 = (p709, p710);
    {
        let mut m708 = m708;
        loop {
            let step = {
                let (v0, v1) = m708;
                let m705 = v0;
                match m705.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a706 = v1;
                            let a707 = 1u64;
                            nat_add(a706, a707)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: List<Str>) -> List<Str> {
    f251(f329(v0, f271(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f271() -> u64 {
    100u64
}

pub fn f272(v0: Str, p713: List<Str>, p714: List<Str>) -> R<List<Str>> {
    let m712 = (p713, p714);
    {
        let mut m712 = m712;
        loop {
            let step = {
                let (v1, v2) = m712;
                let m711 = v1;
                match m711.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f330(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m712 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273() -> List<Str> {
    f251(f331(f102(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f274(v0: Bytes, p717: u64, p718: List<u64>) -> R<List<u64>> {
    let m716 = (p717, p718);
    {
        let mut m716 = m716;
        loop {
            let step = {
                let (v1, v2) = m716;
                let m715 = v1;
                if m715 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m715 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f120(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes, p724: u64, p725: u64, p726: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt88>> {
    let m723 = (p724, (p725, p726));
    {
        let mut m723 = m723;
        loop {
            let step = {
                let (v1, (v2, v3)) = m723;
                let m719 = v2;
                if m719 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt88>>>(Ok(Ok::<(List<List<u64>>, u64), Adt88>((f50(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m719 - 1;
                    let m720 = f16(v0.clone(), v1)?;
                    match m720 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt88>>>(Ok(Err::<(List<List<u64>>, u64), Adt88>(Adt88::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt88>>>((match v5.clone() {
                                Adt90::C0(_, h721) => {
                                    h721
                                }
                            }, (v4, List::cons(f206(match v5 {
                                Adt90::C0(h722, _) => {
                                    h722
                                }
                            })?, v3))))
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

pub fn f276(v0: Adt86, v1: List<List<u64>>) -> Adt86 {
    Adt86::C0(match v0.clone() {
        Adt86::C0(h727, _, _, _, _, _, _) => {
            h727
        }
    }, match v0.clone() {
        Adt86::C0(_, h728, _, _, _, _, _) => {
            h728
        }
    }, match v0.clone() {
        Adt86::C0(_, _, h729, _, _, _, _) => {
            h729
        }
    }, match v0.clone() {
        Adt86::C0(_, _, _, h730, _, _, _) => {
            h730
        }
    }, match v0.clone() {
        Adt86::C0(_, _, _, _, h731, _, _) => {
            h731
        }
    }, v1, match v0 {
        Adt86::C0(_, _, _, _, _, _, h732) => {
            h732
        }
    })
}

pub fn f277(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt97>> {
    let m733 = f332(v0, List::cons(Adt111::C1(0u64, v1), List::<Adt111>::nil()), List::<u64>::nil())?;
    match m733 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt97>(f333(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt97>(v3))
        }
    }
}

pub fn f278() -> u64 {
    67108864u64
}

pub fn f279() -> Str {
    Str::lit("2.0")
}

pub fn f280(v0: Option<Adt95>) -> List<(Str, Adt6)> {
    let m734 = v0;
    match m734 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f334(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f281(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h735, _, _) => {
            h735
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h736, _) => {
            h736
        }
    })), f335(match v0 {
        Adt80::C0(_, _, h737) => {
            h737
        }
    }))))
}

pub fn f282(v0: Adt98) -> i64 {
    let m738 = v0;
    match m738 {
        Adt98::C0 => {
            f336()
        }
        Adt98::C1(_, _) => {
            f337()
        }
    }
}

pub fn f283(v0: Adt98) -> Str {
    let m739 = v0;
    match m739 {
        Adt98::C0 => {
            Str::lit("Parse error")
        }
        Adt98::C1(_, v2) => {
            f338(v2)
        }
    }
}

pub fn f284(v0: Adt98) -> u64 {
    let m740 = v0;
    match m740 {
        Adt98::C0 => {
            f216()
        }
        Adt98::C1(_, _) => {
            f216()
        }
    }
}

pub fn f285(v0: Adt98) -> Option<Adt95> {
    let m741 = v0;
    match m741 {
        Adt98::C0 => {
            None::<Adt95>
        }
        Adt98::C1(v1, _) => {
            v1
        }
    }
}

pub fn f286(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt87>, v4: Adt108) -> R<Adt89> {
    f339(f340(v3.clone())?, f341(v0.clone(), v1, v2, Some(f342(v3, match v0 {
        Adt0::C0(_, _, _, _, h742, _, _) => {
            h742
        }
    }, f343(v4.clone()), match v4.clone() {
        Adt108::C0(_, h743, _) => {
            h743
        }
    })?), f344(), v4)?)
}

pub fn f287(v0: Adt5) -> R<Result<Adt108, Adt98>> {
    let m744 = f345(v0.clone())?;
    match m744 {
        Adt135::C0 => {
            let m745 = f186(Str::lit("method"), v0.clone());
            match m745 {
                Adt107::C0 => {
                    Ok(f288(None::<Adt95>, f346(v0)))
                }
                Adt107::C1(_) => {
                    Ok(f288(None::<Adt95>, if f347(v0) {
                        Adt99::C8
                    } else {
                        Adt99::C7
                    }))
                }
            }
        }
        Adt135::C1(v2) => {
            Ok(f348(v2, v0))
        }
        Adt135::C2 => {
            Ok(f288(None::<Adt95>, Adt99::C5))
        }
    }
}

pub fn f288(v0: Option<Adt95>, v1: Adt99) -> Result<Adt108, Adt98> {
    Err::<Adt108, Adt98>(Adt98::C1(v0, v1))
}

pub fn f289(v0: List<u64>) -> R<Adt117> {
    Ok(f349({
        let a746 = v0.clone();
        length_list(a746)
    }, f350(Adt118::C0, 0u64, 0u64, v0)?))
}

pub fn f290(p753: Adt119, p754: u64, p755: List<u64>) -> R<Result<Adt6, Adt97>> {
    let m752 = (p753, (p754, p755));
    {
        let mut m752 = m752;
        loop {
            let step = {
                let (v0, (v1, v2)) = m752;
                let m747 = v2;
                match m747.uncons() {
                    None => {
                        let m748 = f351(v0, v1)?;
                        match m748 {
                            Ok(v3) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt97>>>(Ok(Ok::<Adt6, Adt97>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt97>>>(Ok(Err::<Adt6, Adt97>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m749 = f352(v0, v1, v5)?;
                        match m749 {
                            Adt136::C0(v7) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt97>>>(Ok(Err::<Adt6, Adt97>(v7)))
                            }
                            Adt136::C1(v8) => {
                                Ok::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt97>>>((v8, ({
                                    let a750 = v1;
                                    let a751 = 1u64;
                                    nat_add(a750, a751)?
                                }, v6)))
                            }
                        }
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

pub fn f291() -> Adt119 {
    Adt119::C0(Adt120::C0, List::<Adt124>::nil(), 0u64)
}

pub fn f292(p762: List<u64>, p763: List<u64>) -> R<bool> {
    let m761 = (p762, p763);
    {
        let mut m761 = m761;
        loop {
            let step = {
                let (v0, v1) = m761;
                let m756 = v0;
                match m756.uncons() {
                    None => {
                        let m757 = v1;
                        match m757.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m758 = v1;
                        match m758.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a759 = f353(v4)?;
                                let a760 = f353(v6)?;
                                if nat_eq(a759, a760) {
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
                Ok(next) => { m761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293(v0: Adt101, v1: List<u64>) -> Adt101 {
    let m764 = v0;
    match m764 {
        Adt101::C0 => {
            Adt101::C1(v1)
        }
        Adt101::C1(_) => {
            Adt101::C2
        }
        Adt101::C2 => {
            Adt101::C2
        }
    }
}

pub fn f294(v0: List<u64>) -> List<u64> {
    f302(f354(f302(f354(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f295(p767: List<u64>) -> bool {
    let m766 = p767;
    {
        let mut m766 = m766;
        loop {
            let step = {
                let v0 = m766;
                let m765 = v0;
                match m765.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f355(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f296() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f297() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f298() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f299(v0: List<u64>, p770: List<List<u64>>) -> R<bool> {
    let m769 = p770;
    {
        let mut m769 = m769;
        loop {
            let step = {
                let v1 = m769;
                let m768 = v1;
                match m768.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f292(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<u64>) -> Option<List<u64>> {
    let m771 = v0;
    match m771.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m772 = v2;
            match m772.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a773 = v1;
                    let a774 = 47u64;
                    if if nat_eq(a773, a774) {
                        let a775 = v3;
                        let a776 = 47u64;
                        nat_eq(a775, a776)
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

pub fn f301(v0: List<u64>) -> bool {
    let m777 = v0;
    match m777.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a778 = v1;
            let a779 = 58u64;
            if nat_eq(a778, a779) {
                let m780 = v2.clone();
                match m780.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f356(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f302(p783: List<u64>, p784: List<u64>) -> List<u64> {
    let m782 = (p783, p784);
    {
        let mut m782 = m782;
        loop {
            let step = {
                let (v0, v1) = m782;
                let m781 = v0;
                match m781.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f303(v0: List<u64>) -> bool {
    let m785 = v0.clone();
    match m785.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a789 = if f357(v0.clone(), false) {
                let a787 = {
                    let a786 = v0;
                    length_list(a786)
                };
                let a788 = f358();
                nat_le(a787, a788)
            } else {
                false
            };
            bool_not(a789)
        }
    }
}

pub fn f304(v0: Adt35) -> Option<Adt92> {
    let m794 = f359(match v0.clone() {
        Adt35::C0(h790, _, _, _, _, _, _, _) => {
            h790
        }
    }, {
        let m793 = match v0.clone() {
            Adt35::C0(_, _, _, _, h792, _, _, _) => {
                h792
            }
        };
        match m793 {
            Adt5::C0(h791) => {
                h791
            }
        }
    });
    match m794 {
        None => {
            f360(match v0.clone() {
                Adt35::C0(h795, _, _, _, _, _, _, _) => {
                    h795
                }
            }, {
                let m798 = match v0 {
                    Adt35::C0(_, _, _, _, h797, _, _, _) => {
                        h797
                    }
                };
                match m798 {
                    Adt5::C0(h796) => {
                        h796
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f305(v0: Adt35, v1: Result<Adt125, Adt92>) -> R<Result<List<Adt12>, Adt92>> {
    let m799 = v1;
    match m799 {
        Ok(v3) => {
            let m800 = f361(v0)?;
            match m800 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt92>(f141(match v3 {
                        Adt125::C0(h801, _) => {
                            h801
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt92>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt92>(v2))
        }
    }
}

pub fn f306(p808: u64, v1: Str, v2: Adt6, p809: List<Adt109>, p810: Adt125) -> R<Result<Adt125, Adt92>> {
    let m807 = (p808, (p809, p810));
    {
        let mut m807 = m807;
        loop {
            let step = {
                let (v0, (v3, v4)) = m807;
                let m802 = v0;
                if m802 == 0 {
                    Err::<(u64, (List<Adt109>, Adt125)), R<Result<Adt125, Adt92>>>(Ok(Err::<Adt125, Adt92>(Adt92::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m802 - 1;
                    let m803 = v3;
                    match m803.uncons() {
                        None => {
                            Err::<(u64, (List<Adt109>, Adt125)), R<Result<Adt125, Adt92>>>(Ok(Ok::<Adt125, Adt92>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m804 = f362(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m804 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt109>, Adt125)), R<Result<Adt125, Adt92>>>((v5, ({
                                        let (h805, _) = v9;
                                        h805
                                    }, {
                                        let (_, h806) = v9;
                                        h806
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt109>, Adt125)), R<Result<Adt125, Adt92>>>(Ok(Err::<Adt125, Adt92>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307() -> u64 {
    67108864u64
}

pub fn f308() -> Adt125 {
    Adt125::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f309(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt103>) -> R<Adt126> {
    let m811 = f317(v1.clone());
    match m811 {
        None => {
            let m812 = f187(v1);
            match m812 {
                None => {
                    Ok(Adt126::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f363(v0, match v5 {
                        Adt5::C0(h813) => {
                            h813
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt126::C3(List::<Adt105>::nil()))
            } else {
                Ok(Adt126::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f310(v0: Adt105, v1: List<Adt105>) -> R<Adt110> {
    let m814 = v0;
    match m814 {
        Adt105::C0(_, _, _) => {
            Ok(Adt110::C0(Adt104::C0, v1))
        }
        Adt105::C1(_, _, _) => {
            Ok(Adt110::C0(Adt104::C0, v1))
        }
        Adt105::C2(v8, v9, v10, v11) => {
            Ok(f364(v8, v9, v10, {
                let a815 = v11;
                let a816 = 1u64;
                nat_add(a815, a816)?
            }, v1))
        }
        Adt105::C3(v12) => {
            Ok(Adt110::C0(Adt104::C1(v12, Str::lit("not")), v1))
        }
        Adt105::C4(v13, _, v15, v16) => {
            Ok(f365(v13, v15, v16, v1))
        }
        Adt105::C5(_, _, _, _) => {
            Ok(Adt110::C0(Adt104::C0, v1))
        }
    }
}

pub fn f311(v0: Adt106, v1: Adt6, v2: Adt6, v3: List<Adt103>) -> Adt126 {
    let m817 = f187(v1);
    match m817 {
        None => {
            Adt126::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m818 = v0;
            match m818 {
                Adt106::C0 => {
                    f366(match v4 {
                        Adt5::C0(h819) => {
                            h819
                        }
                    }, v2, v3)
                }
                Adt106::C1 => {
                    f367(match v4 {
                        Adt5::C0(h820) => {
                            h820
                        }
                    }, v2, v3)
                }
                Adt106::C2 => {
                    f368(match v4 {
                        Adt5::C0(h821) => {
                            h821
                        }
                    }, v2, v3)
                }
                Adt106::C3 => {
                    f369(match v4 {
                        Adt5::C0(h822) => {
                            h822
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f312(v0: Adt105, v1: List<Adt105>, v2: List<Adt103>, v3: Str) -> Adt110 {
    let m823 = v0;
    match m823 {
        Adt105::C0(_, _, _) => {
            Adt110::C0(Adt104::C1(v2, v3), v1)
        }
        Adt105::C1(v7, v8, v9) => {
            f370(v7, v8, v9, v1)
        }
        Adt105::C2(v10, v11, v12, v13) => {
            f364(v10, v11, v12, v13, v1)
        }
        Adt105::C3(_) => {
            Adt110::C0(Adt104::C0, v1)
        }
        Adt105::C4(_, v16, v17, v18) => {
            f365(v16, v17, v18, v1)
        }
        Adt105::C5(_, _, _, _) => {
            Adt110::C0(Adt104::C1(v2, v3), v1)
        }
    }
}

pub fn f313(p826: List<Adt103>, p827: List<Str>) -> R<List<Str>> {
    let m825 = (p826, p827);
    {
        let mut m825 = m825;
        loop {
            let step = {
                let (v0, v1) = m825;
                let m824 = v0;
                match m824.uncons() {
                    None => {
                        Err::<(List<Adt103>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt103>, List<Str>), R<List<Str>>>((v3, List::cons(f371(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m825 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f314(p836: List<Adt38>) -> R<bool> {
    let m835 = p836;
    {
        let mut m835 = m835;
        loop {
            let step = {
                let v0 = m835;
                let m828 = v0;
                match m828.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m829 = v1;
                        match m829 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f372(match v4 {
                                    Adt42::C0(h830, _, _, _) => {
                                        h830
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f372(match v5 {
                                    Adt43::C0(h831, _, _, _) => {
                                        h831
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
                                let m833 = match v7 {
                                    Adt45::C0(h832, _, _) => {
                                        h832
                                    }
                                };
                                match m833 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f372(match v9 {
                                            Adt48::C0(_, _, h834, _) => {
                                                h834
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
                Ok(next) => { m835 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f373(match v1.clone() {
        Adt49::C0(h837, _) => {
            h837
        }
    }) {
        f374(v1)
    } else {
        let m839 = match v0 {
            Adt35::C0(_, _, _, _, _, h838, _, _) => {
                h838
            }
        };
        match m839 {
            None => {
                f374(v1)
            }
            Some(v2) => {
                f375(v2, v1)
            }
        }
    }
}

pub fn f316(v0: i64, v1: i64) -> R<i64> {
    let a841 = v0;
    let a842 = {
        let a840 = v1;
        int_neg(a840)?
    };
    int_sub(a841, a842)
}

pub fn f317(v0: Adt6) -> Option<bool> {
    let m843 = v0;
    match m843 {
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

pub fn f318(v0: Str) -> R<Option<List<u64>>> {
    let m844 = f376(f377(f378())?, f377(v0)?);
    match m844 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m845 = f376(f379(List::cons(47u64, f377(f380())?)), f379(v1));
            match m845 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f379(v2);
                    if if f381(v3.clone()) {
                        f382(v3.clone())
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

pub fn f319(v0: Bytes, v1: List<u64>) -> Bytes {
    let a846 = v0;
    let a847 = f383(f320(), f384(List::<Bytes>::nil(), v1));
    append_bytes(a846, a847)
}

pub fn f320() -> Bytes {
    Bytes::lit(&[])
}

pub fn f321(v0: Adt6) -> R<Result<Bytes, Adt97>> {
    let m848 = f209(v0)?;
    match m848 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt97>(f319(f320(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt97>(v2))
        }
    }
}

pub fn f322(v0: Str, v1: Str) -> Adt127 {
    let a849 = v0;
    let a850 = v1;
    if equal(a849, a850) {
        Adt127::C0
    } else {
        Adt127::C1
    }
}

pub fn f323(v0: Adt5) -> bool {
    if f385(f186(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f385(f186(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f385(f186(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f324(v0: Adt5) -> bool {
    let m851 = f186(Str::lit("content"), v0);
    match m851 {
        Adt107::C0 => {
            true
        }
        Adt107::C1(v1) => {
            let m852 = f187(v1);
            match m852 {
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

pub fn f325(v0: Adt5) -> bool {
    if f385(f186(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f385(f186(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f326(v0: Adt107) -> bool {
    let m853 = v0;
    match m853 {
        Adt107::C0 => {
            false
        }
        Adt107::C1(v1) => {
            let m854 = f266(v1);
            match m854 {
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

pub fn f327(v0: Adt5) -> bool {
    let m855 = f186(Str::lit("content"), v0);
    match m855 {
        Adt107::C0 => {
            false
        }
        Adt107::C1(v1) => {
            let m856 = f187(v1.clone());
            match m856 {
                None => {
                    let m857 = f242(v1);
                    match m857 {
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

pub fn f328(p861: List<Adt6>) -> bool {
    let m860 = p861;
    {
        let mut m860 = m860;
        loop {
            let step = {
                let v0 = m860;
                let m858 = v0;
                match m858.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m859 = f187(v1);
                        match m859 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f326(f186(Str::lit("uri"), v3)) {
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
                Ok(next) => { m860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(p865: List<Str>, p866: u64, p867: List<Str>) -> List<Str> {
    let m864 = (p865, (p866, p867));
    {
        let mut m864 = m864;
        loop {
            let step = {
                let (v0, (v1, v2)) = m864;
                let m862 = v0;
                match m862.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m863 = v1;
                        if m863 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m863 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(v0: Str, v1: Str) -> R<bool> {
    Ok(f386(f377(v0)?, f377(v1)?))
}

pub fn f331(p871: List<Adt44>, p872: List<Str>) -> List<Str> {
    let m870 = (p871, p872);
    {
        let mut m870 = m870;
        loop {
            let step = {
                let (v0, v1) = m870;
                let m868 = v0;
                match m868.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt44::C0(_, _, _, h869, _, _, _, _, _) => {
                                h869
                            }
                        }, v1)))
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

pub fn f332(p877: u64, p878: List<Adt111>, p879: List<u64>) -> R<Result<List<u64>, Adt97>> {
    let m876 = (p877, (p878, p879));
    {
        let mut m876 = m876;
        loop {
            let step = {
                let (v0, (v1, v2)) = m876;
                let m873 = v0;
                if m873 == 0 {
                    Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt97>>>(Ok(Err::<List<u64>, Adt97>(Adt97::C13)))
                } else {
                    let v3 = m873 - 1;
                    let m874 = v1;
                    match m874.uncons() {
                        None => {
                            Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt97>>>(Ok(Ok::<List<u64>, Adt97>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m875 = f387(v4, v5, v2)?;
                            match m875 {
                                Adt137::C0(v6) => {
                                    Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt97>>>(Ok(Err::<List<u64>, Adt97>(v6)))
                                }
                                Adt137::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt97>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m876 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333(p882: List<u64>, p883: List<u64>) -> List<u64> {
    let m881 = (p882, p883);
    {
        let mut m881 = m881;
        loop {
            let step = {
                let (v0, v1) = m881;
                let m880 = v0;
                match m880.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f334(v0: Adt95) -> Adt6 {
    let m884 = v0;
    match m884 {
        Adt95::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt95::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f335(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m885 = v0;
    match m885 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f336() -> i64 {
    -32700i64
}

pub fn f337() -> i64 {
    -32600i64
}

pub fn f338(v0: Adt99) -> Str {
    let m886 = v0;
    match m886 {
        Adt99::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt99::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt99::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt99::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt99::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt99::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt99::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt99::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt99::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f339(v0: bool, v1: Adt128) -> R<Adt89> {
    let m888 = match v1.clone() {
        Adt128::C0(_, h887) => {
            h887
        }
    };
    match m888 {
        Ok(v2) => {
            let m889 = f388(v2.clone())?;
            match m889 {
                None => {
                    Ok(f6())
                }
                Some(v3) => {
                    Ok(f389(v0, f125(f390(), f391(match v1 {
                        Adt128::C0(h890, _) => {
                            h890
                        }
                    }, v3))?, f392(match v2 {
                        Adt129::C0(_, _, h891) => {
                            h891
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f125(f393(v4.clone()), f127(Some(match v1 {
                Adt128::C0(h892, _) => {
                    h892
                }
            }), f394(v4)?))
        }
    }
}

pub fn f340(p897: List<Adt87>) -> R<bool> {
    let m896 = p897;
    {
        let mut m896 = m896;
        loop {
            let step = {
                let v0 = m896;
                let m893 = v0;
                match m893.uncons() {
                    None => {
                        Err::<List<Adt87>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f292(match v1.clone() {
                            Adt87::C0(h894, _) => {
                                h894
                            }
                        }, f395())? {
                            if f396(f397(44u64, match v1 {
                                Adt87::C0(_, h895) => {
                                    h895
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt87>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt87>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt87>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt112>, v4: List<Str>, v5: Adt108) -> R<Adt128> {
    Ok(Adt128::C0(match v5.clone() {
        Adt108::C0(h898, _, _) => {
            h898
        }
    }, f398(v0, v1, v2, f399(v3, v4, v5)?)?))
}

pub fn f342(v0: List<Adt87>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt112> {
    let v4: Adt134 = f400(Adt114::C0, f134(v0.clone(), f401())?);
    let v5: Adt134 = f400(Adt114::C1, f134(v0.clone(), f402())?);
    let v6: Adt134 = if f403(v3) {
        f400(Adt114::C2, f404(f134(v0.clone(), f405())?)?)
    } else {
        f406()
    };
    Ok(Adt112::C0(match v4.clone() {
        Adt134::C0(h899, _) => {
            h899
        }
    }, match v5.clone() {
        Adt134::C0(h900, _) => {
            h900
        }
    }, match v6.clone() {
        Adt134::C0(h901, _) => {
            h901
        }
    }, f407(match v4 {
        Adt134::C0(_, h902) => {
            h902
        }
    }, f407(match v5 {
        Adt134::C0(_, h903) => {
            h903
        }
    }, match v6 {
        Adt134::C0(_, h904) => {
            h904
        }
    })), f408(v0, v1, v2)?))
}

pub fn f343(v0: Adt108) -> Option<Str> {
    let a906 = match v0.clone() {
        Adt108::C0(_, h905, _) => {
            h905
        }
    };
    let a907 = Str::lit("tools/call");
    if equal(a906, a907) {
        let m909 = match v0 {
            Adt108::C0(_, _, h908) => {
                h908
            }
        };
        match m909 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m910 = f186(Str::lit("name"), v1);
                match m910 {
                    Adt107::C0 => {
                        None::<Str>
                    }
                    Adt107::C1(v2) => {
                        f266(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f344() -> List<Str> {
    List::cons(f409(), List::<Str>::nil())
}

pub fn f345(v0: Adt5) -> R<Adt135> {
    let m911 = f186(Str::lit("id"), v0);
    match m911 {
        Adt107::C0 => {
            Ok(Adt135::C0)
        }
        Adt107::C1(v1) => {
            let m912 = v1;
            match m912 {
                Adt6::C0 => {
                    Ok(Adt135::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt135::C2)
                }
                Adt6::C2(v3) => {
                    let m913 = f410(v3)?;
                    match m913 {
                        None => {
                            Ok(Adt135::C2)
                        }
                        Some(v4) => {
                            Ok(Adt135::C1(Adt95::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt135::C1(Adt95::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt135::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt135::C2)
                }
            }
        }
    }
}

pub fn f346(v0: Adt5) -> Adt99 {
    if f347(v0) {
        Adt99::C8
    } else {
        Adt99::C3
    }
}

pub fn f347(v0: Adt5) -> bool {
    if f411(Str::lit("result"), v0.clone()) {
        true
    } else {
        f411(Str::lit("error"), v0)
    }
}

pub fn f348(v0: Adt95, v1: Adt5) -> Result<Adt108, Adt98> {
    let m914 = f186(Str::lit("jsonrpc"), v1.clone());
    match m914 {
        Adt107::C0 => {
            f288(Some(v0), Adt99::C2)
        }
        Adt107::C1(v2) => {
            let m915 = f266(v2);
            match m915 {
                None => {
                    f288(Some(v0), Adt99::C2)
                }
                Some(v3) => {
                    let a916 = v3;
                    let a917 = f279();
                    if equal(a916, a917) {
                        f412(v0, v1)
                    } else {
                        f288(Some(v0), Adt99::C2)
                    }
                }
            }
        }
    }
}

pub fn f349(v0: u64, v1: u64) -> Adt117 {
    let a918 = v1;
    let a919 = 0u64;
    if nat_eq(a918, a919) {
        Adt117::C0
    } else {
        let a920 = v1;
        let a921 = 2u64;
        if nat_eq(a920, a921) {
            Adt117::C2(v0)
        } else {
            Adt117::C1({
                let a925 = v0;
                let a926 = {
                    let a922 = v1;
                    let a923 = 3u64;
                    let a924 = 0u64;
                    nat_quot(a922, a923, a924)
                };
                nat_sub(a925, a926)
            })
        }
    }
}

pub fn f350(p954: Adt118, p955: u64, p956: u64, p957: List<u64>) -> R<u64> {
    let m953 = (p954, (p955, (p956, p957)));
    {
        let mut m953 = m953;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m953;
                let m927 = v3;
                match m927.uncons() {
                    None => {
                        let m928 = v0;
                        match m928 {
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
                        let m929 = v0;
                        match m929 {
                            Adt118::C0 => {
                                let a930 = v4;
                                let a931 = 128u64;
                                if nat_lt(a930, a931) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a932 = v4;
                                    let a933 = 194u64;
                                    if nat_lt(a932, a933) {
                                        let v6: u64 = f413({
                                            let a934 = List::cons(v4, v5);
                                            length_list(a934)
                                        })?;
                                        Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a935 = v4;
                                        let a936 = 224u64;
                                        if nat_lt(a935, a936) {
                                            Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a937 = v4;
                                            let a938 = 224u64;
                                            if nat_eq(a937, a938) {
                                                Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a939 = v4;
                                                let a940 = 237u64;
                                                if nat_eq(a939, a940) {
                                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a941 = v4;
                                                    let a942 = 240u64;
                                                    if nat_lt(a941, a942) {
                                                        Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a943 = v4;
                                                        let a944 = 240u64;
                                                        if nat_eq(a943, a944) {
                                                            Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a945 = v4;
                                                            let a946 = 244u64;
                                                            if nat_eq(a945, a946) {
                                                                Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a947 = v4;
                                                                let a948 = 244u64;
                                                                if nat_lt(a947, a948) {
                                                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f413({
                                                                        let a949 = List::cons(v4, v5);
                                                                        length_list(a949)
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
                                if f414(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f413({
                                        let a950 = List::cons(v4, v5);
                                        length_list(a950)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt118::C2 => {
                                if f414(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f413({
                                        let a951 = List::cons(v4, v5);
                                        length_list(a951)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt118::C3 => {
                                if f414(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f413({
                                        let a952 = List::cons(v4, v5);
                                        length_list(a952)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: Adt119, v1: u64) -> R<Result<Adt6, Adt97>> {
    let m959 = match v0.clone() {
        Adt119::C0(h958, _, _) => {
            h958
        }
    };
    match m959 {
        Adt120::C0 => {
            Ok(Err::<Adt6, Adt97>(Adt97::C3))
        }
        Adt120::C1 => {
            Ok(Err::<Adt6, Adt97>(Adt97::C3))
        }
        Adt120::C2 => {
            Ok(Err::<Adt6, Adt97>(Adt97::C3))
        }
        Adt120::C3 => {
            Ok(Err::<Adt6, Adt97>(Adt97::C3))
        }
        Adt120::C4 => {
            Ok(Err::<Adt6, Adt97>(Adt97::C3))
        }
        Adt120::C5 => {
            Ok(Err::<Adt6, Adt97>(Adt97::C3))
        }
        Adt120::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt97>(Adt97::C3))
        }
        Adt120::C7(_, _) => {
            Ok(Err::<Adt6, Adt97>(Adt97::C3))
        }
        Adt120::C8(v3) => {
            let m960 = f415(v3)?;
            match m960 {
                Adt140::C0 => {
                    Ok(Err::<Adt6, Adt97>(Adt97::C7(v1)))
                }
                Adt140::C1 => {
                    Ok(Err::<Adt6, Adt97>(Adt97::C8(v1)))
                }
                Adt140::C2(v4) => {
                    let m965 = {
                        let m964 = f416(v4, match v0.clone() {
                            Adt119::C0(_, h962, _) => {
                                h962
                            }
                        }, match v0 {
                            Adt119::C0(_, _, h963) => {
                                h963
                            }
                        });
                        match m964 {
                            Adt119::C0(h961, _, _) => {
                                h961
                            }
                        }
                    };
                    match m965 {
                        Adt120::C0 => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C1 => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C2 => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C3 => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C4 => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C5 => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C7(_, _) => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C8(_) => {
                            Ok(Err::<Adt6, Adt97>(Adt97::C3))
                        }
                        Adt120::C9(v5) => {
                            Ok(Ok::<Adt6, Adt97>(v5))
                        }
                    }
                }
            }
        }
        Adt120::C9(v2) => {
            Ok(Ok::<Adt6, Adt97>(v2))
        }
    }
}

pub fn f352(v0: Adt119, v1: u64, v2: u64) -> R<Adt136> {
    let m967 = match v0.clone() {
        Adt119::C0(h966, _, _) => {
            h966
        }
    };
    match m967 {
        Adt120::C0 => {
            if f417(v2) {
                Ok(Adt136::C1(v0))
            } else {
                f418(v0, v1, v2)
            }
        }
        Adt120::C1 => {
            if f417(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a968 = v2;
                let a969 = 93u64;
                if nat_eq(a968, a969) {
                    Ok(f419(v0, v1))
                } else {
                    f418(v0, v1, v2)
                }
            }
        }
        Adt120::C2 => {
            Ok(f420(v0, v1, v2))
        }
        Adt120::C3 => {
            if f417(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a970 = v2;
                let a971 = 125u64;
                if nat_eq(a970, a971) {
                    Ok(f421(v0, v1))
                } else {
                    let a972 = v2;
                    let a973 = 34u64;
                    if nat_eq(a972, a973) {
                        Ok(f422(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f423(Adt97::C2(v1)))
                    }
                }
            }
        }
        Adt120::C4 => {
            if f417(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a974 = v2;
                let a975 = 34u64;
                if nat_eq(a974, a975) {
                    Ok(f422(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f423(Adt97::C2(v1)))
                }
            }
        }
        Adt120::C5 => {
            if f417(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a976 = v2;
                let a977 = 58u64;
                if nat_eq(a976, a977) {
                    Ok(f422(v0, Adt120::C0))
                } else {
                    Ok(f423(Adt97::C2(v1)))
                }
            }
        }
        Adt120::C6(v3, v4, v5) => {
            f424(v0, v1, v2, v3, v4, v5)
        }
        Adt120::C7(v6, v7) => {
            f425(v0, v1, v2, v6, v7)
        }
        Adt120::C8(v8) => {
            f426(v0, v1, v2, v8)
        }
        Adt120::C9(_) => {
            Ok(f420(v0, v1, v2))
        }
    }
}

pub fn f353(v0: u64) -> R<u64> {
    let a978 = 64u64;
    let a979 = v0;
    if if nat_lt(a978, a979) {
        let a980 = v0;
        let a981 = 91u64;
        nat_lt(a980, a981)
    } else {
        false
    } {
        let a982 = v0;
        let a983 = 32u64;
        nat_add(a982, a983)
    } else {
        Ok(v0)
    }
}

pub fn f354(p986: List<u64>) -> List<u64> {
    let m985 = p986;
    {
        let mut m985 = m985;
        loop {
            let step = {
                let v0 = m985;
                let m984 = v0;
                match m984.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f427(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
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

pub fn f355(v0: u64) -> bool {
    let a987 = v0;
    let a988 = 9u64;
    if nat_eq(a987, a988) {
        true
    } else {
        let a989 = 31u64;
        let a990 = v0;
        if nat_lt(a989, a990) {
            let a991 = v0;
            let a992 = 127u64;
            nat_lt(a991, a992)
        } else {
            false
        }
    }
}

pub fn f356(p995: List<u64>) -> bool {
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
                Ok(next) => { m994 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(p998: List<u64>, v1: bool) -> bool {
    let m997 = p998;
    {
        let mut m997 = m997;
        loop {
            let step = {
                let v0 = m997;
                let m996 = v0;
                match m996.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f429(v2)
                        } else {
                            f430(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358() -> u64 {
    64u64
}

pub fn f359(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt92> {
    let m999 = f262(Str::lit("type"), v1);
    match m999 {
        Adt107::C0 => {
            Some(Adt92::C3(v0))
        }
        Adt107::C1(v2) => {
            let m1000 = f266(v2);
            match m1000 {
                None => {
                    Some(Adt92::C3(v0))
                }
                Some(v3) => {
                    let a1001 = v3;
                    let a1002 = Str::lit("object");
                    if equal(a1001, a1002) {
                        None::<Adt92>
                    } else {
                        Some(Adt92::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f360(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt92> {
    let m1003 = f262(Str::lit("$schema"), v1);
    match m1003 {
        Adt107::C0 => {
            None::<Adt92>
        }
        Adt107::C1(v2) => {
            let m1004 = f266(v2);
            match m1004 {
                None => {
                    Some(Adt92::C2(v0))
                }
                Some(v3) => {
                    let a1005 = v3;
                    let a1006 = f431();
                    if equal(a1005, a1006) {
                        None::<Adt92>
                    } else {
                        Some(Adt92::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f361(v0: Adt35) -> R<Option<Adt92>> {
    let m1008 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h1007, _, _) => {
            h1007
        }
    };
    match m1008 {
        None => {
            Ok(None::<Adt92>)
        }
        Some(v1) => {
            let m1011 = f360(match v0.clone() {
                Adt35::C0(h1009, _, _, _, _, _, _, _) => {
                    h1009
                }
            }, match v1.clone() {
                Adt5::C0(h1010) => {
                    h1010
                }
            });
            match m1011 {
                None => {
                    let m1015 = f306(f307(), match v0 {
                        Adt35::C0(h1012, _, _, _, _, _, _, _) => {
                            h1012
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1013) => {
                            h1013
                        }
                    }), List::cons(Adt109::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1014) => {
                            h1014
                        }
                    }), false, List::<Str>::nil()), List::<Adt109>::nil()), f308())?;
                    match m1015 {
                        Ok(_) => {
                            Ok(None::<Adt92>)
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

pub fn f362(v0: Str, v1: Adt6, v2: Adt109, v3: Adt125, v4: List<Adt109>) -> R<Result<(List<Adt109>, Adt125), Adt92>> {
    let m1016 = v2;
    match m1016 {
        Adt109::C0(v5, v6, v7) => {
            let m1017 = f317(v5.clone());
            match m1017 {
                None => {
                    let m1018 = f187(v5);
                    match m1018 {
                        None => {
                            Ok(Err::<(List<Adt109>, Adt125), Adt92>(Adt92::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f432(v0, v1, match v9 {
                                Adt5::C0(h1019) => {
                                    h1019
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt109>, Adt125), Adt92>((v4, v3)))
                }
            }
        }
    }
}

pub fn f363(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt103>) -> R<Adt126> {
    let m1020 = f433(v1.clone(), v2.clone())?;
    match m1020 {
        Adt126::C0(v4) => {
            Ok(Adt126::C0(v4))
        }
        Adt126::C1(v5) => {
            Ok(Adt126::C1(v5))
        }
        Adt126::C2(v6) => {
            Ok(Adt126::C2(v6))
        }
        Adt126::C3(_) => {
            f434(v0, v1, v2, v3)
        }
    }
}

pub fn f364(v0: List<Adt6>, v1: Adt6, v2: List<Adt103>, v3: u64, v4: List<Adt105>) -> Adt110 {
    let m1021 = v0;
    match m1021.uncons() {
        None => {
            let a1022 = v3;
            let a1023 = 1u64;
            if nat_eq(a1022, a1023) {
                Adt110::C0(Adt104::C0, v4)
            } else {
                Adt110::C0(Adt104::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt110::C0(Adt104::C0, List::cons(Adt105::C0(v5, v1.clone(), v2.clone()), List::cons(Adt105::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f365(v0: Option<Adt6>, v1: Adt6, v2: List<Adt103>, v3: List<Adt105>) -> Adt110 {
    let m1024 = v0;
    match m1024 {
        None => {
            Adt110::C0(Adt104::C0, v3)
        }
        Some(v4) => {
            Adt110::C0(Adt104::C0, List::cons(Adt105::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f366(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt103>) -> Adt126 {
    let m1025 = f262(Str::lit("anyOf"), v0);
    match m1025 {
        Adt107::C0 => {
            Adt126::C3(List::<Adt105>::nil())
        }
        Adt107::C1(v3) => {
            let m1026 = f242(v3);
            match m1026 {
                None => {
                    Adt126::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1027 = v4;
                    match m1027.uncons() {
                        None => {
                            Adt126::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt126::C3(List::cons(Adt105::C0(v5, v1.clone(), v2.clone()), List::cons(Adt105::C1(v6, v1, v2), List::<Adt105>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f367(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt103>) -> Adt126 {
    let m1028 = f262(Str::lit("oneOf"), v0);
    match m1028 {
        Adt107::C0 => {
            Adt126::C3(List::<Adt105>::nil())
        }
        Adt107::C1(v3) => {
            let m1029 = f242(v3);
            match m1029 {
                None => {
                    Adt126::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1030 = v4;
                    match m1030.uncons() {
                        None => {
                            Adt126::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt126::C3(List::cons(Adt105::C0(v5, v1.clone(), v2.clone()), List::cons(Adt105::C2(v6, v1, v2, 0u64), List::<Adt105>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f368(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt103>) -> Adt126 {
    let m1031 = f262(Str::lit("not"), v0);
    match m1031 {
        Adt107::C0 => {
            Adt126::C3(List::<Adt105>::nil())
        }
        Adt107::C1(v3) => {
            Adt126::C3(List::cons(Adt105::C0(v3, v1, v2.clone()), List::cons(Adt105::C3(v2), List::<Adt105>::nil())))
        }
    }
}

pub fn f369(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt103>) -> Adt126 {
    let m1032 = f262(Str::lit("if"), v0.clone());
    match m1032 {
        Adt107::C0 => {
            Adt126::C3(List::<Adt105>::nil())
        }
        Adt107::C1(v3) => {
            Adt126::C3(List::cons(Adt105::C0(v3, v1.clone(), v2.clone()), List::cons(Adt105::C4(f435(Str::lit("then"), v0.clone()), f435(Str::lit("else"), v0), v1, v2), List::<Adt105>::nil())))
        }
    }
}

pub fn f370(v0: List<Adt6>, v1: Adt6, v2: List<Adt103>, v3: List<Adt105>) -> Adt110 {
    let m1033 = v0;
    match m1033.uncons() {
        None => {
            Adt110::C0(Adt104::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt110::C0(Adt104::C0, List::cons(Adt105::C0(v4, v1.clone(), v2.clone()), List::cons(Adt105::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f371(v0: Adt103) -> R<Str> {
    let m1034 = v0;
    match m1034 {
        Adt103::C0(v1) => {
            f436(v1)
        }
        Adt103::C1(v2) => {
            let a1035 = f241(v2, 0i64)?;
            Ok(format_i64(a1035))
        }
    }
}

pub fn f372(v0: Str) -> R<bool> {
    let m1037 = f437(f206({
        let a1036 = v0;
        utf8_encode(a1036)
    })?)?;
    match m1037 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f373(v0: Adt37) -> bool {
    let m1039 = match v0 {
        Adt37::C0(_, _, h1038) => {
            h1038
        }
    };
    match m1039 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f374(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1041 = match v0.clone() {
        Adt49::C0(_, h1040) => {
            h1040
        }
    };
    match m1041.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h1042, _) => {
                    h1042
                }
            })))
        }
        Some((_, _)) => {
            if f438(match v0.clone() {
                Adt49::C0(_, h1043) => {
                    h1043
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f375(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1047 = {
        let m1046 = match v1.clone() {
            Adt49::C0(h1045, _) => {
                h1045
            }
        };
        match m1046 {
            Adt37::C0(_, h1044, _) => {
                h1044
            }
        }
    };
    match m1047 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1050 = f146(Adt6::C5(match v0.clone() {
                Adt5::C0(h1048) => {
                    h1048
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1049) => {
                    h1049
                }
            }), v2)?;
            match m1050 {
                Adt102::C0 => {
                    f374(v1)
                }
                Adt102::C1(_, _) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt102::C2(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt102::C3(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt102::C4 => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f376(p1056: List<u64>, p1057: List<u64>) -> Option<List<u64>> {
    let m1055 = (p1056, p1057);
    {
        let mut m1055 = m1055;
        loop {
            let step = {
                let (v0, v1) = m1055;
                let m1051 = v0;
                match m1051.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1052 = v1;
                        match m1052.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1053 = v2;
                                let a1054 = v4;
                                if nat_eq(a1053, a1054) {
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
                Ok(next) => { m1055 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377(v0: Str) -> R<List<u64>> {
    f206({
        let a1058 = v0;
        utf8_encode(a1058)
    })
}

pub fn f378() -> Str {
    Str::lit("test://template/")
}

pub fn f379(v0: List<u64>) -> List<u64> {
    f333(v0, List::<u64>::nil())
}

pub fn f380() -> Str {
    Str::lit("data")
}

pub fn f381(v0: List<u64>) -> bool {
    let m1059 = v0;
    match m1059.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f382(p1064: List<u64>) -> bool {
    let m1063 = p1064;
    {
        let mut m1063 = m1063;
        loop {
            let step = {
                let v0 = m1063;
                let m1060 = v0;
                match m1060.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1061 = v1;
                        let a1062 = 47u64;
                        if nat_eq(a1061, a1062) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(p1069: Bytes, p1070: List<Bytes>) -> Bytes {
    let m1068 = (p1069, p1070);
    {
        let mut m1068 = m1068;
        loop {
            let step = {
                let (v0, v1) = m1068;
                let m1065 = v1;
                match m1065.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1066 = v2;
                            let a1067 = v0;
                            append_bytes(a1066, a1067)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(p1073: List<Bytes>, p1074: List<u64>) -> List<Bytes> {
    let m1072 = (p1073, p1074);
    {
        let mut m1072 = m1072;
        loop {
            let step = {
                let (v0, v1) = m1072;
                let m1071 = v1;
                match m1071.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f439(f440(v2), v0), v3))
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

pub fn f385(v0: Adt107, v1: Str) -> bool {
    let m1075 = v0;
    match m1075 {
        Adt107::C0 => {
            false
        }
        Adt107::C1(v2) => {
            let m1076 = f266(v2);
            match m1076 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1077 = v3;
                    let a1078 = v1;
                    equal(a1077, a1078)
                }
            }
        }
    }
}

pub fn f386(p1084: List<u64>, p1085: List<u64>) -> bool {
    let m1083 = (p1084, p1085);
    {
        let mut m1083 = m1083;
        loop {
            let step = {
                let (v0, v1) = m1083;
                let m1079 = v0;
                match m1079.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1080 = v1;
                        match m1080.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1081 = v2;
                                let a1082 = v4;
                                if nat_eq(a1081, a1082) {
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
                Ok(next) => { m1083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: Adt111, v1: List<Adt111>, v2: List<u64>) -> R<Adt137> {
    let m1086 = v0;
    match m1086 {
        Adt111::C0(v3) => {
            Ok(Adt137::C1(v1, f333(v3, v2)))
        }
        Adt111::C1(v4, v5) => {
            f441(v4, v5, v1, v2)
        }
        Adt111::C2(v6, v7) => {
            let m1087 = v7;
            match m1087.uncons() {
                None => {
                    Ok(Adt137::C1(List::cons(Adt111::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt137::C1(List::cons(Adt111::C1(v6, v8), List::cons(Adt111::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt111::C3(v10, v11) => {
            let m1088 = v11.clone();
            match m1088.uncons() {
                None => {
                    Ok(Adt137::C1(List::cons(Adt111::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt137::C1(List::cons(Adt111::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt111::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt111::C4(v14, v15) => {
            let m1089 = v15;
            match m1089.uncons() {
                None => {
                    Ok(Adt137::C1(List::cons(Adt111::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt137::C1(List::cons(Adt111::C0({
                        let a1091 = f442({
                            let (h1090, _) = v16;
                            h1090
                        })?;
                        let a1092 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1091, a1092)
                    }), List::cons(Adt111::C1(v14, {
                        let (_, h1093) = v16;
                        h1093
                    }), List::cons(Adt111::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt111::C5(v18, v19) => {
            let m1094 = v19.clone();
            match m1094.uncons() {
                None => {
                    Ok(Adt137::C1(List::cons(Adt111::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt137::C1(List::cons(Adt111::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt111::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f388(v0: Adt129) -> R<Option<Adt5>> {
    let m1096 = f443(match v0.clone() {
        Adt129::C0(h1095, _, _) => {
            h1095
        }
    })?;
    match m1096 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1099 = v1;
                let a1100 = {
                    let m1098 = f444(v0);
                    match m1098 {
                        Adt5::C0(h1097) => {
                            h1097
                        }
                    }
                };
                append_list(a1099, a1100)
            })))
        }
    }
}

pub fn f389(v0: bool, v1: Adt89, v2: Option<List<List<u64>>>) -> Adt89 {
    let m1101 = v2;
    match m1101 {
        None => {
            f6()
        }
        Some(v3) => {
            f445(v0, v1, f50(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f390() -> u64 {
    200u64
}

pub fn f391(v0: Adt95, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f279())), List::cons((Str::lit("id"), f334(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1102) => {
            h1102
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f392(p1106: List<Adt131>, p1107: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1105 = (p1106, p1107);
    {
        let mut m1105 = m1105;
        loop {
            let step = {
                let (v0, v1) = m1105;
                let m1103 = v0;
                match m1103.uncons() {
                    None => {
                        Err::<(List<Adt131>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1104 = f446(v2)?;
                        match m1104 {
                            None => {
                                Err::<(List<Adt131>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt131>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
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

pub fn f393(v0: Adt132) -> u64 {
    let m1108 = v0;
    match m1108 {
        Adt132::C0(_) => {
            f216()
        }
        Adt132::C1(_, _) => {
            f216()
        }
        Adt132::C2(_) => {
            f216()
        }
        Adt132::C3(_) => {
            f447()
        }
        Adt132::C4(v6) => {
            f448(v6)
        }
    }
}

pub fn f394(v0: Adt132) -> R<Adt80> {
    Ok(Adt80::C0(f449(v0.clone()), f450(v0.clone()), f451(v0)?))
}

pub fn f395() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f396(p1111: List<List<u64>>) -> R<bool> {
    let m1110 = p1111;
    {
        let mut m1110 = m1110;
        loop {
            let step = {
                let v0 = m1110;
                let m1109 = v0;
                match m1109.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f452(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: u64, p1116: List<u64>, p1117: List<u64>, p1118: List<List<u64>>) -> List<List<u64>> {
    let m1115 = (p1116, (p1117, p1118));
    {
        let mut m1115 = m1115;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1115;
                let m1112 = v1;
                match m1112.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f50(List::cons(f302(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1113 = v4;
                        let a1114 = v0;
                        if nat_eq(a1113, a1114) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f302(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt138, Adt132>) -> R<Result<Adt129, Adt132>> {
    let m1119 = v3;
    match m1119 {
        Ok(v5) => {
            f453(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt129, Adt132>(v4))
        }
    }
}

pub fn f399(v0: Option<Adt112>, v1: List<Str>, v2: Adt108) -> R<Result<Adt138, Adt132>> {
    Ok(f454(f455(match v2.clone() {
        Adt108::C0(_, h1120, _) => {
            h1120
        }
    }), f456(v0, v1, v2)?))
}

pub fn f400(v0: Adt114, v1: Adt100) -> Adt134 {
    let m1121 = v1;
    match m1121 {
        Adt100::C0 => {
            f406()
        }
        Adt100::C1 => {
            Adt134::C0(None::<Str>, Some(Adt113::C6(v0)))
        }
        Adt100::C2 => {
            Adt134::C0(None::<Str>, Some(Adt113::C7(v0)))
        }
        Adt100::C3(v2) => {
            let m1122 = f457(v2);
            match m1122 {
                None => {
                    Adt134::C0(None::<Str>, Some(Adt113::C6(v0)))
                }
                Some(v3) => {
                    Adt134::C0(Some(v3), None::<Adt113>)
                }
            }
        }
    }
}

pub fn f401() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f402() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f403(v0: Str) -> bool {
    let m1123 = f458(v0);
    match m1123 {
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

pub fn f404(v0: Adt100) -> R<Adt100> {
    let m1124 = v0.clone();
    match m1124 {
        Adt100::C0 => {
            Ok(v0)
        }
        Adt100::C1 => {
            Ok(v0)
        }
        Adt100::C2 => {
            Ok(v0)
        }
        Adt100::C3(v1) => {
            let m1125 = f459(v1);
            match m1125 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1126 = f437(v2)?;
                    match m1126 {
                        None => {
                            Ok(Adt100::C1)
                        }
                        Some(v3) => {
                            Ok(Adt100::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f405() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f406() -> Adt134 {
    Adt134::C0(None::<Str>, None::<Adt113>)
}

pub fn f407(v0: Option<Adt113>, v1: Option<Adt113>) -> Option<Adt113> {
    let m1127 = v0.clone();
    match m1127 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f408(v0: List<Adt87>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt115>> {
    Ok(f460(f461(v0, v1, v2, List::<Adt115>::nil())?, List::<Adt115>::nil()))
}

pub fn f409() -> Str {
    Str::lit("2026-07-28")
}

pub fn f410(v0: Adt7) -> R<Option<i64>> {
    let m1130 = f462(40u64, match v0.clone() {
        Adt7::C0(h1128, _) => {
            h1128
        }
    }, match v0 {
        Adt7::C0(_, h1129) => {
            h1129
        }
    })?;
    match m1130 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f463(v1))
        }
    }
}

pub fn f411(v0: Str, v1: Adt5) -> bool {
    f464(f186(v0, v1))
}

pub fn f412(v0: Adt95, v1: Adt5) -> Result<Adt108, Adt98> {
    let m1131 = f186(Str::lit("method"), v1.clone());
    match m1131 {
        Adt107::C0 => {
            f288(Some(v0), f346(v1))
        }
        Adt107::C1(v2) => {
            let m1132 = f266(v2);
            match m1132 {
                None => {
                    f288(Some(v0), Adt99::C4)
                }
                Some(v3) => {
                    f465(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f413(v0: u64) -> R<u64> {
    let a1135 = {
        let a1133 = v0;
        let a1134 = 3u64;
        nat_mul(a1133, a1134)?
    };
    let a1136 = 1u64;
    nat_add(a1135, a1136)
}

pub fn f414(v0: u64, v1: u64, v2: u64) -> bool {
    let a1137 = v1;
    let a1138 = v0;
    if nat_le(a1137, a1138) {
        let a1139 = v0;
        let a1140 = v2;
        nat_le(a1139, a1140)
    } else {
        false
    }
}

pub fn f415(v0: Adt122) -> R<Adt140> {
    if f466(match v0.clone() {
        Adt122::C0(_, h1141, _, _, _, _, _) => {
            h1141
        }
    }) {
        let v1: i64 = {
            let a1147 = if match v0.clone() {
                Adt122::C0(_, _, _, _, _, h1142, _) => {
                    h1142
                }
            } {
                let a1144 = match v0.clone() {
                    Adt122::C0(_, _, _, _, _, _, h1143) => {
                        h1143
                    }
                };
                int_neg(a1144)?
            } else {
                match v0.clone() {
                    Adt122::C0(_, _, _, _, _, _, h1145) => {
                        h1145
                    }
                }
            };
            let a1148 = match v0.clone() {
                Adt122::C0(_, _, _, _, h1146, _, _) => {
                    h1146
                }
            };
            int_sub(a1147, a1148)?
        };
        let m1151 = {
            let a1149 = v1;
            let a1150 = -9999i64;
            compare(a1149, a1150)
        };
        if if match m1151 {
            Ordering::Less => {
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
            let m1154 = {
                let a1152 = 9999i64;
                let a1153 = v1;
                compare(a1152, a1153)
            };
            match m1154 {
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
            Ok(Adt140::C1)
        } else {
            Ok(Adt140::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt122::C0(h1155, _, _, _, _, _, _) => {
                    h1155
                }
            } {
                let a1157 = match v0 {
                    Adt122::C0(_, _, h1156, _, _, _, _) => {
                        h1156
                    }
                };
                int_neg(a1157)?
            } else {
                match v0 {
                    Adt122::C0(_, _, h1158, _, _, _, _) => {
                        h1158
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt140::C0)
    }
}

pub fn f416(v0: Adt6, v1: List<Adt124>, v2: u64) -> Adt119 {
    let m1159 = v1.clone();
    match m1159.uncons() {
        None => {
            Adt119::C0(Adt120::C9(v0), List::<Adt124>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1160 = v3;
            match m1160 {
                Adt124::C0(v5) => {
                    Adt119::C0(Adt120::C2, List::cons(Adt124::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt124::C1(v6, v7) => {
                    let m1161 = v7;
                    match m1161 {
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

pub fn f417(v0: u64) -> bool {
    let a1162 = v0;
    let a1163 = 32u64;
    if nat_eq(a1162, a1163) {
        true
    } else {
        let a1164 = v0;
        let a1165 = 10u64;
        if nat_eq(a1164, a1165) {
            true
        } else {
            let a1166 = v0;
            let a1167 = 13u64;
            if nat_eq(a1166, a1167) {
                true
            } else {
                let a1168 = v0;
                let a1169 = 9u64;
                nat_eq(a1168, a1169)
            }
        }
    }
}

pub fn f418(v0: Adt119, v1: u64, v2: u64) -> R<Adt136> {
    let a1170 = v2;
    let a1171 = 34u64;
    if nat_eq(a1170, a1171) {
        Ok(f422(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), false)))
    } else {
        let a1172 = v2;
        let a1173 = 123u64;
        if nat_eq(a1172, a1173) {
            f467(v0, Adt124::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt120::C3, v1)
        } else {
            let a1174 = v2;
            let a1175 = 91u64;
            if nat_eq(a1174, a1175) {
                f467(v0, Adt124::C0(List::<Adt6>::nil()), Adt120::C1, v1)
            } else {
                let a1176 = v2;
                let a1177 = 116u64;
                if nat_eq(a1176, a1177) {
                    Ok(f422(v0, Adt120::C7(0u64, 1u64)))
                } else {
                    let a1178 = v2;
                    let a1179 = 102u64;
                    if nat_eq(a1178, a1179) {
                        Ok(f422(v0, Adt120::C7(1u64, 1u64)))
                    } else {
                        let a1180 = v2;
                        let a1181 = 110u64;
                        if nat_eq(a1180, a1181) {
                            Ok(f422(v0, Adt120::C7(2u64, 1u64)))
                        } else {
                            let a1182 = v2;
                            let a1183 = 45u64;
                            if nat_eq(a1182, a1183) {
                                Ok(f422(v0, f468(true, Adt123::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1184 = v2;
                                let a1185 = 48u64;
                                if nat_eq(a1184, a1185) {
                                    Ok(f422(v0, f468(false, Adt123::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f414(v2, 49u64, 57u64) {
                                        Ok(f422(v0, f468(false, Adt123::C2, f469({
                                            let a1186 = v2;
                                            let a1187 = 48u64;
                                            nat_sub(a1186, a1187)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f423(Adt97::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f419(v0: Adt119, v1: u64) -> Adt136 {
    let m1189 = match v0.clone() {
        Adt119::C0(_, h1188, _) => {
            h1188
        }
    };
    match m1189.uncons() {
        None => {
            f423(Adt97::C2(v1))
        }
        Some((v2, v3)) => {
            let m1190 = v2;
            match m1190 {
                Adt124::C0(v4) => {
                    Adt136::C1(f416(Adt6::C4(f470(v4, List::<Adt6>::nil())), v3, {
                        let a1192 = match v0 {
                            Adt119::C0(_, _, h1191) => {
                                h1191
                            }
                        };
                        let a1193 = 1u64;
                        nat_sub(a1192, a1193)
                    }))
                }
                Adt124::C1(_, _) => {
                    f423(Adt97::C2(v1))
                }
            }
        }
    }
}

pub fn f420(v0: Adt119, v1: u64, v2: u64) -> Adt136 {
    let m1195 = match v0.clone() {
        Adt119::C0(h1194, _, _) => {
            h1194
        }
    };
    match m1195 {
        Adt120::C0 => {
            f423(Adt97::C2(v1))
        }
        Adt120::C1 => {
            f423(Adt97::C2(v1))
        }
        Adt120::C2 => {
            if f417(v2) {
                Adt136::C1(v0)
            } else {
                let a1196 = v2;
                let a1197 = 44u64;
                if nat_eq(a1196, a1197) {
                    f471(v0, v1)
                } else {
                    let a1198 = v2;
                    let a1199 = 93u64;
                    if nat_eq(a1198, a1199) {
                        f419(v0, v1)
                    } else {
                        let a1200 = v2;
                        let a1201 = 125u64;
                        if nat_eq(a1200, a1201) {
                            f421(v0, v1)
                        } else {
                            f423(Adt97::C2(v1))
                        }
                    }
                }
            }
        }
        Adt120::C3 => {
            f423(Adt97::C2(v1))
        }
        Adt120::C4 => {
            f423(Adt97::C2(v1))
        }
        Adt120::C5 => {
            f423(Adt97::C2(v1))
        }
        Adt120::C6(_, _, _) => {
            f423(Adt97::C2(v1))
        }
        Adt120::C7(_, _) => {
            f423(Adt97::C2(v1))
        }
        Adt120::C8(_) => {
            f423(Adt97::C2(v1))
        }
        Adt120::C9(_) => {
            if f417(v2) {
                Adt136::C1(v0)
            } else {
                f423(Adt97::C10(v1))
            }
        }
    }
}

pub fn f421(v0: Adt119, v1: u64) -> Adt136 {
    let m1203 = match v0.clone() {
        Adt119::C0(_, h1202, _) => {
            h1202
        }
    };
    match m1203.uncons() {
        None => {
            f423(Adt97::C2(v1))
        }
        Some((v2, v3)) => {
            let m1204 = v2;
            match m1204 {
                Adt124::C0(_) => {
                    f423(Adt97::C2(v1))
                }
                Adt124::C1(v5, _) => {
                    Adt136::C1(f416(Adt6::C5(f472(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1206 = match v0 {
                            Adt119::C0(_, _, h1205) => {
                                h1205
                            }
                        };
                        let a1207 = 1u64;
                        nat_sub(a1206, a1207)
                    }))
                }
            }
        }
    }
}

pub fn f422(v0: Adt119, v1: Adt120) -> Adt136 {
    Adt136::C1(f473(v0, v1))
}

pub fn f423(v0: Adt97) -> Adt136 {
    Adt136::C0(v0)
}

pub fn f424(v0: Adt119, v1: u64, v2: u64, v3: Adt121, v4: List<u64>, v5: bool) -> R<Adt136> {
    let m1208 = v3;
    match m1208 {
        Adt121::C0 => {
            let a1209 = v2;
            let a1210 = 34u64;
            if nat_eq(a1209, a1210) {
                Ok(f474(v0, v1, v4, v5))
            } else {
                let a1211 = v2;
                let a1212 = 92u64;
                if nat_eq(a1211, a1212) {
                    Ok(f422(v0, Adt120::C6(Adt121::C1, v4, v5)))
                } else {
                    let a1213 = v2;
                    let a1214 = 32u64;
                    if nat_lt(a1213, a1214) {
                        Ok(f423(Adt97::C4(v1)))
                    } else {
                        Ok(f422(v0, Adt120::C6(Adt121::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt121::C1 => {
            let a1215 = v2;
            let a1216 = 117u64;
            if nat_eq(a1215, a1216) {
                Ok(f422(v0, Adt120::C6(Adt121::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1217 = f475(v2);
                let a1218 = 0u64;
                if nat_eq(a1217, a1218) {
                    Ok(f423(Adt97::C5(v1)))
                } else {
                    Ok(f422(v0, Adt120::C6(Adt121::C0, List::cons(f475(v2), v4), v5)))
                }
            }
        }
        Adt121::C2(v6, v7, v8) => {
            let a1219 = 15u64;
            let a1220 = f476(v2);
            if nat_lt(a1219, a1220) {
                Ok(f423(Adt97::C5(v1)))
            } else {
                let a1223 = {
                    let a1221 = v6;
                    let a1222 = 1u64;
                    nat_add(a1221, a1222)?
                };
                let a1224 = 4u64;
                if nat_eq(a1223, a1224) {
                    f477(v0, v1, {
                        let a1227 = {
                            let a1225 = v7;
                            let a1226 = 16u64;
                            nat_mul(a1225, a1226)?
                        };
                        let a1228 = f476(v2);
                        nat_add(a1227, a1228)?
                    }, v8, v4, v5)
                } else {
                    Ok(f422(v0, Adt120::C6(Adt121::C2({
                        let a1229 = v6;
                        let a1230 = 1u64;
                        nat_add(a1229, a1230)?
                    }, {
                        let a1233 = {
                            let a1231 = v7;
                            let a1232 = 16u64;
                            nat_mul(a1231, a1232)?
                        };
                        let a1234 = f476(v2);
                        nat_add(a1233, a1234)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt121::C3(v9) => {
            let a1235 = v2;
            let a1236 = 92u64;
            if nat_eq(a1235, a1236) {
                Ok(f422(v0, Adt120::C6(Adt121::C4(v9), v4, v5)))
            } else {
                Ok(f423(Adt97::C6(v1)))
            }
        }
        Adt121::C4(v10) => {
            let a1237 = v2;
            let a1238 = 117u64;
            if nat_eq(a1237, a1238) {
                Ok(f422(v0, Adt120::C6(Adt121::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f423(Adt97::C6(v1)))
            }
        }
    }
}

pub fn f425(v0: Adt119, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt136> {
    let a1239 = v2;
    let a1240 = f478(v3, v4);
    if nat_eq(a1239, a1240) {
        let a1244 = {
            let a1241 = v4;
            let a1242 = 1u64;
            nat_add(a1241, a1242)?
        };
        let a1245 = {
            let a1243 = f479(v3);
            length_list(a1243)
        };
        if nat_eq(a1244, a1245) {
            Ok(f480(v0, f481(v3)))
        } else {
            Ok(f422(v0, Adt120::C7(v3, {
                let a1246 = v4;
                let a1247 = 1u64;
                nat_add(a1246, a1247)?
            })))
        }
    } else {
        Ok(f423(Adt97::C2(v1)))
    }
}

pub fn f426(v0: Adt119, v1: u64, v2: u64, v3: Adt122) -> R<Adt136> {
    let m1249 = match v3.clone() {
        Adt122::C0(_, h1248, _, _, _, _, _) => {
            h1248
        }
    };
    match m1249 {
        Adt123::C0 => {
            let a1250 = v2;
            let a1251 = 48u64;
            if nat_eq(a1250, a1251) {
                Ok(f482(v0, v3, Adt123::C1))
            } else {
                if f414(v2, 49u64, 57u64) {
                    f483(v0, v1, v2, v3, false)
                } else {
                    Ok(f423(Adt97::C7(v1)))
                }
            }
        }
        Adt123::C1 => {
            let a1252 = v2;
            let a1253 = 46u64;
            if nat_eq(a1252, a1253) {
                Ok(f482(v0, v3, Adt123::C3))
            } else {
                if f484(v2) {
                    Ok(f482(v0, v3, Adt123::C5))
                } else {
                    if f485(v2) {
                        Ok(f423(Adt97::C7(v1)))
                    } else {
                        f486(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt123::C2 => {
            if f485(v2) {
                f483(v0, v1, v2, v3, false)
            } else {
                let a1254 = v2;
                let a1255 = 46u64;
                if nat_eq(a1254, a1255) {
                    Ok(f482(v0, v3, Adt123::C3))
                } else {
                    if f484(v2) {
                        Ok(f482(v0, v3, Adt123::C5))
                    } else {
                        f486(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt123::C3 => {
            if f485(v2) {
                f483(v0, v1, v2, v3, true)
            } else {
                Ok(f423(Adt97::C7(v1)))
            }
        }
        Adt123::C4 => {
            if f485(v2) {
                f483(v0, v1, v2, v3, true)
            } else {
                if f484(v2) {
                    Ok(f482(v0, v3, Adt123::C5))
                } else {
                    f486(v0, v1, v2, v3)
                }
            }
        }
        Adt123::C5 => {
            let a1256 = v2;
            let a1257 = 43u64;
            if nat_eq(a1256, a1257) {
                Ok(f422(v0, f468(match v3.clone() {
                    Adt122::C0(h1258, _, _, _, _, _, _) => {
                        h1258
                    }
                }, Adt123::C6, match v3.clone() {
                    Adt122::C0(_, _, h1259, _, _, _, _) => {
                        h1259
                    }
                }, match v3.clone() {
                    Adt122::C0(_, _, _, h1260, _, _, _) => {
                        h1260
                    }
                }, match v3 {
                    Adt122::C0(_, _, _, _, h1261, _, _) => {
                        h1261
                    }
                }, false, 0i64)))
            } else {
                let a1262 = v2;
                let a1263 = 45u64;
                if nat_eq(a1262, a1263) {
                    Ok(f422(v0, f468(match v3.clone() {
                        Adt122::C0(h1264, _, _, _, _, _, _) => {
                            h1264
                        }
                    }, Adt123::C6, match v3.clone() {
                        Adt122::C0(_, _, h1265, _, _, _, _) => {
                            h1265
                        }
                    }, match v3.clone() {
                        Adt122::C0(_, _, _, h1266, _, _, _) => {
                            h1266
                        }
                    }, match v3 {
                        Adt122::C0(_, _, _, _, h1267, _, _) => {
                            h1267
                        }
                    }, true, 0i64)))
                } else {
                    if f485(v2) {
                        f487(v0, v1, v2, Adt122::C0(match v3.clone() {
                            Adt122::C0(h1268, _, _, _, _, _, _) => {
                                h1268
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, h1269, _, _, _, _, _) => {
                                h1269
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, _, h1270, _, _, _, _) => {
                                h1270
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, _, _, h1271, _, _, _) => {
                                h1271
                            }
                        }, match v3 {
                            Adt122::C0(_, _, _, _, h1272, _, _) => {
                                h1272
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f423(Adt97::C7(v1)))
                    }
                }
            }
        }
        Adt123::C6 => {
            if f485(v2) {
                f487(v0, v1, v2, v3.clone(), match v3 {
                    Adt122::C0(_, _, _, _, _, h1273, _) => {
                        h1273
                    }
                })
            } else {
                Ok(f423(Adt97::C7(v1)))
            }
        }
        Adt123::C7 => {
            if f485(v2) {
                f487(v0, v1, v2, v3.clone(), match v3 {
                    Adt122::C0(_, _, _, _, _, h1274, _) => {
                        h1274
                    }
                })
            } else {
                f486(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f427(v0: u64) -> bool {
    let a1275 = v0;
    let a1276 = 9u64;
    if nat_eq(a1275, a1276) {
        true
    } else {
        let a1277 = v0;
        let a1278 = 32u64;
        nat_eq(a1277, a1278)
    }
}

pub fn f428(v0: u64) -> bool {
    let a1279 = 47u64;
    let a1280 = v0;
    if nat_lt(a1279, a1280) {
        let a1281 = v0;
        let a1282 = 58u64;
        nat_lt(a1281, a1282)
    } else {
        false
    }
}

pub fn f429(v0: u64) -> bool {
    let a1283 = 48u64;
    let a1284 = v0;
    if if nat_le(a1283, a1284) {
        let a1285 = v0;
        let a1286 = 57u64;
        nat_le(a1285, a1286)
    } else {
        false
    } {
        true
    } else {
        let a1287 = 65u64;
        let a1288 = v0;
        if if nat_le(a1287, a1288) {
            let a1289 = v0;
            let a1290 = 90u64;
            nat_le(a1289, a1290)
        } else {
            false
        } {
            true
        } else {
            let a1291 = 97u64;
            let a1292 = v0;
            if if nat_le(a1291, a1292) {
                let a1293 = v0;
                let a1294 = 122u64;
                nat_le(a1293, a1294)
            } else {
                false
            } {
                true
            } else {
                let a1295 = v0;
                let a1296 = 33u64;
                if nat_eq(a1295, a1296) {
                    true
                } else {
                    let a1297 = v0;
                    let a1298 = 35u64;
                    if nat_eq(a1297, a1298) {
                        true
                    } else {
                        let a1299 = v0;
                        let a1300 = 36u64;
                        if nat_eq(a1299, a1300) {
                            true
                        } else {
                            let a1301 = v0;
                            let a1302 = 37u64;
                            if nat_eq(a1301, a1302) {
                                true
                            } else {
                                let a1303 = v0;
                                let a1304 = 38u64;
                                if nat_eq(a1303, a1304) {
                                    true
                                } else {
                                    let a1305 = v0;
                                    let a1306 = 39u64;
                                    if nat_eq(a1305, a1306) {
                                        true
                                    } else {
                                        let a1307 = v0;
                                        let a1308 = 42u64;
                                        if nat_eq(a1307, a1308) {
                                            true
                                        } else {
                                            let a1309 = v0;
                                            let a1310 = 43u64;
                                            if nat_eq(a1309, a1310) {
                                                true
                                            } else {
                                                let a1311 = v0;
                                                let a1312 = 45u64;
                                                if nat_eq(a1311, a1312) {
                                                    true
                                                } else {
                                                    let a1313 = v0;
                                                    let a1314 = 46u64;
                                                    if nat_eq(a1313, a1314) {
                                                        true
                                                    } else {
                                                        let a1315 = v0;
                                                        let a1316 = 94u64;
                                                        if nat_eq(a1315, a1316) {
                                                            true
                                                        } else {
                                                            let a1317 = v0;
                                                            let a1318 = 95u64;
                                                            if nat_eq(a1317, a1318) {
                                                                true
                                                            } else {
                                                                let a1319 = v0;
                                                                let a1320 = 96u64;
                                                                if nat_eq(a1319, a1320) {
                                                                    true
                                                                } else {
                                                                    let a1321 = v0;
                                                                    let a1322 = 124u64;
                                                                    if nat_eq(a1321, a1322) {
                                                                        true
                                                                    } else {
                                                                        let a1323 = v0;
                                                                        let a1324 = 126u64;
                                                                        nat_eq(a1323, a1324)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f430(v0: u64) -> bool {
    let a1325 = 48u64;
    let a1326 = v0;
    if if nat_le(a1325, a1326) {
        let a1327 = v0;
        let a1328 = 57u64;
        nat_le(a1327, a1328)
    } else {
        false
    } {
        true
    } else {
        let a1329 = 65u64;
        let a1330 = v0;
        if if nat_le(a1329, a1330) {
            let a1331 = v0;
            let a1332 = 90u64;
            nat_le(a1331, a1332)
        } else {
            false
        } {
            true
        } else {
            let a1333 = 97u64;
            let a1334 = v0;
            if if nat_le(a1333, a1334) {
                let a1335 = v0;
                let a1336 = 122u64;
                nat_le(a1335, a1336)
            } else {
                false
            } {
                true
            } else {
                let a1337 = v0;
                let a1338 = 95u64;
                if nat_eq(a1337, a1338) {
                    true
                } else {
                    let a1339 = v0;
                    let a1340 = 45u64;
                    if nat_eq(a1339, a1340) {
                        true
                    } else {
                        let a1341 = v0;
                        let a1342 = 46u64;
                        nat_eq(a1341, a1342)
                    }
                }
            }
        }
    }
}

pub fn f431() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f432(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt125, v6: List<Adt109>) -> R<Result<(List<Adt109>, Adt125), Adt92>> {
    let m1343 = f488(v0.clone(), v1, v2.clone());
    match m1343 {
        None => {
            let m1344 = f489(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1344 {
                Adt146::C0(v8) => {
                    Ok(Err::<(List<Adt109>, Adt125), Adt92>(v8))
                }
                Adt146::C1(v9) => {
                    Ok(Ok::<(List<Adt109>, Adt125), Adt92>((f490(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt109>, Adt125), Adt92>(v7))
        }
    }
}

pub fn f433(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt126> {
    Ok(f491(List::cons((Str::lit("type"), f492(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f493(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f494(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f495(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f495(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f495(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f495(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1345 = f496(v0.clone(), v1.clone())?;
        let a1346 = List::cons((Str::lit("required"), f497(v0, v1)), List::<(Str, Adt141)>::nil());
        append_list(a1345, a1346)
    })))))))))
}

pub fn f434(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt103>) -> R<Adt126> {
    Ok(f498(f499(v0, v1.clone(), v2.clone(), v3.clone()), f498(f500(v1.clone(), v2.clone(), v3.clone()), f498(f501(v1.clone(), v2.clone(), v3.clone()), f498(f502(v1.clone(), v2.clone(), v3.clone())?, f498(f503(v1.clone(), v2.clone(), v3.clone()), f498(f504(Adt106::C0, f366(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f498(f504(Adt106::C1, f367(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f498(f504(Adt106::C2, f368(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f504(Adt106::C3, f369(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f435(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1347 = f262(v0, v1);
    match m1347 {
        Adt107::C0 => {
            None::<Adt6>
        }
        Adt107::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f436(v0: Str) -> R<Str> {
    let m1350 = {
        let a1349 = f319(f320(), f333(f505(f206({
            let a1348 = v0.clone();
            utf8_encode(a1348)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1349)
    };
    match m1350 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f437(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1352 = f506({
        let a1351 = v0.clone();
        length_list(a1351)
    }, v0, List::<u64>::nil())?;
    match m1352 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f333(v1, List::<u64>::nil())))
        }
    }
}

pub fn f438(v0: List<Adt50>) -> R<bool> {
    let m1353 = v0;
    match m1353.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f507(v2, match v1 {
                Adt50::C0(h1354, _, _) => {
                    h1354
                }
            })
        }
    }
}

pub fn f439(p1363: Bytes, p1364: List<Bytes>) -> List<Bytes> {
    let m1362 = (p1363, p1364);
    {
        let mut m1362 = m1362;
        loop {
            let step = {
                let (v0, v1) = m1362;
                let m1355 = v1;
                match m1355.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1358 = {
                            let a1356 = v2.clone();
                            length_bytes(a1356)
                        };
                        let a1359 = {
                            let a1357 = v0.clone();
                            length_bytes(a1357)
                        };
                        if nat_eq(a1358, a1359) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1360 = v2;
                                let a1361 = v0;
                                append_bytes(a1360, a1361)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1362 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: u64) -> Bytes {
    let m1368 = {
        let a1365 = f508();
        let a1366 = v0;
        let a1367 = 1u64;
        slice_bytes(a1365, a1366, a1367)
    };
    match m1368 {
        None => {
            f320()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f441(v0: u64, v1: Adt6, v2: List<Adt111>, v3: List<u64>) -> R<Adt137> {
    let m1369 = v1;
    match m1369 {
        Adt6::C0 => {
            Ok(Adt137::C1(v2, f333(f509(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt137::C1(v2, f333(if v4 {
                f509(0u64)
            } else {
                f509(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f510(v5.clone())? {
                Ok(Adt137::C1(v2, f333(f511(v5)?, v3)))
            } else {
                Ok(Adt137::C0(Adt97::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt137::C1(v2, f333(f442(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1370 = v0;
            let a1371 = f512();
            if nat_lt(a1370, a1371) {
                Ok(Adt137::C1(f513(91u64, Adt111::C2({
                    let a1372 = v0;
                    let a1373 = 1u64;
                    nat_add(a1372, a1373)?
                }, v7), v2), v3))
            } else {
                Ok(Adt137::C0(Adt97::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1374 = v0;
            let a1375 = f512();
            if nat_lt(a1374, a1375) {
                Ok(Adt137::C1(f513(123u64, Adt111::C4({
                    let a1376 = v0;
                    let a1377 = 1u64;
                    nat_add(a1376, a1377)?
                }, v8), v2), v3))
            } else {
                Ok(Adt137::C0(Adt97::C12))
            }
        }
    }
}

pub fn f442(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1379 = f333(f514(f206({
            let a1378 = v0;
            utf8_encode(a1378)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1380 = List::cons(34u64, List::<u64>::nil());
        append_list(a1379, a1380)
    }))
}

pub fn f443(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m1381 = v0;
    match m1381 {
        Adt32::C0(v1) => {
            Ok(Some(f515(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f516(v2)?))
        }
        Adt32::C2(v3) => {
            f517(f518(v3)?)
        }
        Adt32::C3(v4) => {
            f517(f518(match v4 {
                Adt49::C0(h1382, _) => {
                    h1382
                }
            })?)
        }
        Adt32::C4(v5) => {
            Ok(Some(f519(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f520(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f521(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f522(v8)?))
        }
        Adt32::C8(v9) => {
            f517(f523(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f524(v10)?))
        }
        Adt32::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt32::C12(v12) => {
            f517(f525(v12)?)
        }
        Adt32::C13(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f444(v0: Adt129) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f526(f527(match v0.clone() {
        Adt129::C0(h1383, _, _) => {
            h1383
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f528({
        let m1386 = match v0 {
            Adt129::C0(_, h1385, _) => {
                h1385
            }
        };
        match m1386 {
            Adt130::C0(h1384, _) => {
                h1384
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f445(v0: bool, v1: Adt89, v2: List<List<u64>>) -> Adt89 {
    if f529(v0, match v1.clone() {
        Adt89::C0(h1387, _, _, _) => {
            h1387
        }
    }, v2.clone()) {
        f530(match v1.clone() {
            Adt89::C0(h1388, _, _, _) => {
                h1388
            }
        }, {
            let a1390 = v2;
            let a1391 = List::cons(match v1 {
                Adt89::C0(_, _, _, h1389) => {
                    h1389
                }
            }, List::<List<u64>>::nil());
            append_list(a1390, a1391)
        })
    } else {
        v1
    }
}

pub fn f446(v0: Adt131) -> R<Option<List<u64>>> {
    let m1392 = f531(f532(v0))?;
    match m1392 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1393 = f209(v1)?;
            match m1393 {
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

pub fn f447() -> u64 {
    404u64
}

pub fn f448(v0: Adt84) -> u64 {
    let m1394 = v0;
    match m1394 {
        Adt84::C0(_) => {
            f216()
        }
        Adt84::C1(_) => {
            f216()
        }
        Adt84::C2(_) => {
            f216()
        }
        Adt84::C3(_) => {
            f216()
        }
        Adt84::C4(_) => {
            f216()
        }
        Adt84::C5(_) => {
            f216()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f533()
        }
        Adt84::C8(_) => {
            f216()
        }
        Adt84::C9(_) => {
            f21()
        }
        Adt84::C10 => {
            f216()
        }
        Adt84::C11 => {
            f216()
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
        Adt84::C15(_) => {
            f21()
        }
    }
}

pub fn f449(v0: Adt132) -> i64 {
    let m1395 = v0;
    match m1395 {
        Adt132::C0(_) => {
            f534()
        }
        Adt132::C1(_, _) => {
            f535()
        }
        Adt132::C2(_) => {
            f536()
        }
        Adt132::C3(_) => {
            f537()
        }
        Adt132::C4(v6) => {
            f538(v6)
        }
    }
}

pub fn f450(v0: Adt132) -> Str {
    let m1396 = v0;
    match m1396 {
        Adt132::C0(v1) => {
            f539(v1)
        }
        Adt132::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt132::C2(v4) => {
            f540(v4)
        }
        Adt132::C3(_) => {
            Str::lit("Method not found")
        }
        Adt132::C4(v6) => {
            let m1397 = v6;
            match m1397 {
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
                Adt84::C15(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f451(v0: Adt132) -> R<Option<Adt6>> {
    let m1398 = v0;
    match m1398 {
        Adt132::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt132::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f541(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt132::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt132::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt132::C4(v6) => {
            let m1399 = v6;
            match m1399 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f241(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f542(v15)), List::<(Str, Adt6)>::nil()))))
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
                Adt84::C15(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f452(v0: List<u64>) -> R<bool> {
    let m1400 = f397(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1400.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f543(v2)? {
                Ok(false)
            } else {
                f292(f294(v1), f544())
            }
        }
    }
}

pub fn f453(v0: Adt0, v1: Fn0, v2: u64, v3: Adt138) -> R<Result<Adt129, Adt132>> {
    f545(v0.clone(), v1, v2, v3.clone(), f546(match v0 {
        Adt0::C0(_, _, h1401, _, _, _, _) => {
            h1401
        }
    }, match v3 {
        Adt138::C0(_, h1402, _, _) => {
            h1402
        }
    }))
}

pub fn f454(v0: Adt142, v1: Adt143) -> Result<Adt138, Adt132> {
    let m1403 = v1;
    match m1403 {
        Adt143::C0(v3) => {
            f547(v3, v0)
        }
        Adt143::C1(v2) => {
            Err::<Adt138, Adt132>(v2)
        }
    }
}

pub fn f455(v0: Str) -> Adt142 {
    let a1404 = v0.clone();
    let a1405 = Str::lit("server/discover");
    if equal(a1404, a1405) {
        Adt142::C1(Adt139::C0)
    } else {
        let a1406 = v0.clone();
        let a1407 = Str::lit("tools/list");
        if equal(a1406, a1407) {
            Adt142::C1(Adt139::C1)
        } else {
            let a1408 = v0.clone();
            let a1409 = Str::lit("tools/call");
            if equal(a1408, a1409) {
                Adt142::C1(Adt139::C2)
            } else {
                let a1410 = v0.clone();
                let a1411 = Str::lit("resources/list");
                if equal(a1410, a1411) {
                    Adt142::C1(Adt139::C3)
                } else {
                    let a1412 = v0.clone();
                    let a1413 = Str::lit("resources/read");
                    if equal(a1412, a1413) {
                        Adt142::C1(Adt139::C4)
                    } else {
                        let a1414 = v0.clone();
                        let a1415 = Str::lit("resources/templates/list");
                        if equal(a1414, a1415) {
                            Adt142::C1(Adt139::C5)
                        } else {
                            let a1416 = v0.clone();
                            let a1417 = Str::lit("prompts/list");
                            if equal(a1416, a1417) {
                                Adt142::C1(Adt139::C6)
                            } else {
                                let a1418 = v0.clone();
                                let a1419 = Str::lit("prompts/get");
                                if equal(a1418, a1419) {
                                    Adt142::C1(Adt139::C7)
                                } else {
                                    let a1420 = v0.clone();
                                    let a1421 = Str::lit("completion/complete");
                                    if equal(a1420, a1421) {
                                        Adt142::C1(Adt139::C8)
                                    } else {
                                        let a1422 = v0.clone();
                                        let a1423 = Str::lit("tasks/get");
                                        if equal(a1422, a1423) {
                                            Adt142::C1(Adt139::C9)
                                        } else {
                                            let a1424 = v0.clone();
                                            let a1425 = Str::lit("tasks/update");
                                            if equal(a1424, a1425) {
                                                Adt142::C1(Adt139::C10)
                                            } else {
                                                let a1426 = v0;
                                                let a1427 = Str::lit("tasks/cancel");
                                                if equal(a1426, a1427) {
                                                    Adt142::C1(Adt139::C11)
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

pub fn f456(v0: Option<Adt112>, v1: List<Str>, v2: Adt108) -> R<Adt143> {
    f548(v0, v1, v2.clone(), f549(match v2 {
        Adt108::C0(_, _, h1428) => {
            h1428
        }
    }))
}

pub fn f457(v0: List<u64>) -> Option<Str> {
    let a1429 = f319(f320(), v0);
    utf8_decode(a1429)
}

pub fn f458(v0: Str) -> Adt145 {
    let m1430 = f455(v0);
    match m1430 {
        Adt142::C0 => {
            Adt145::C0
        }
        Adt142::C1(v1) => {
            f550(v1)
        }
    }
}

pub fn f459(v0: List<u64>) -> Option<List<u64>> {
    let m1431 = f551(f552(), v0);
    match m1431 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1432 = f551(f553(), f302(v1, List::<u64>::nil()));
            match m1432 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f302(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f460(p1435: List<Adt115>, p1436: List<Adt115>) -> List<Adt115> {
    let m1434 = (p1435, p1436);
    {
        let mut m1434 = m1434;
        loop {
            let step = {
                let (v0, v1) = m1434;
                let m1433 = v0;
                match m1433.uncons() {
                    None => {
                        Err::<(List<Adt115>, List<Adt115>), List<Adt115>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt115>, List<Adt115>), List<Adt115>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: List<Adt87>, p1443: List<Adt12>, v2: Option<Str>, p1444: List<Adt115>) -> R<List<Adt115>> {
    let m1442 = (p1443, p1444);
    {
        let mut m1442 = m1442;
        loop {
            let step = {
                let (v1, v3) = m1442;
                let m1437 = v1;
                match m1437.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f554(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>((v5, List::cons(Adt115::C0(match v4.clone() {
                                Adt12::C0(_, h1438, _, _) => {
                                    h1438
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1439, _) => {
                                    h1439
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1440) => {
                                    h1440
                                }
                            }, f555(v0.clone(), match v4 {
                                Adt12::C0(_, h1441, _, _) => {
                                    h1441
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(p1460: u64, p1461: i64, p1462: i64) -> R<Option<i64>> {
    let m1459 = (p1460, (p1461, p1462));
    {
        let mut m1459 = m1459;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1459;
                let m1445 = v0;
                if m1445 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1445 - 1;
                    if f556(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1448 = {
                            let a1446 = v2;
                            let a1447 = 0i64;
                            compare(a1446, a1447)
                        };
                        if match m1448 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f556({
                                let a1449 = v1;
                                let a1450 = 10i64;
                                let a1451 = 0i64;
                                int_rem(a1449, a1450, a1451)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1452 = v1;
                                    let a1453 = 10i64;
                                    let a1454 = 0i64;
                                    int_quot(a1452, a1453, a1454)?
                                }, f316(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1455 = v1;
                                let a1456 = 10i64;
                                int_mul(a1455, a1456)?
                            }, {
                                let a1457 = v2;
                                let a1458 = 1i64;
                                int_sub(a1457, a1458)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: i64) -> Option<i64> {
    let m1465 = {
        let a1463 = 9007199254740991i64;
        let a1464 = v0;
        compare(a1463, a1464)
    };
    if if match m1465 {
        Ordering::Less => {
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
        let m1468 = {
            let a1466 = v0;
            let a1467 = -9007199254740991i64;
            compare(a1466, a1467)
        };
        match m1468 {
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

pub fn f464(v0: Adt107) -> bool {
    let m1469 = v0;
    match m1469 {
        Adt107::C0 => {
            false
        }
        Adt107::C1(_) => {
            true
        }
    }
}

pub fn f465(v0: Adt95, v1: Str, v2: Adt5) -> Result<Adt108, Adt98> {
    let m1470 = f186(Str::lit("params"), v2);
    match m1470 {
        Adt107::C0 => {
            Ok::<Adt108, Adt98>(Adt108::C0(v0, v1, None::<Adt5>))
        }
        Adt107::C1(v3) => {
            let m1471 = f187(v3);
            match m1471 {
                None => {
                    f288(Some(v0), Adt99::C6)
                }
                Some(v4) => {
                    Ok::<Adt108, Adt98>(Adt108::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f466(v0: Adt123) -> bool {
    let m1472 = v0;
    match m1472 {
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

pub fn f467(v0: Adt119, v1: Adt124, v2: Adt120, v3: u64) -> R<Adt136> {
    let a1474 = match v0.clone() {
        Adt119::C0(_, _, h1473) => {
            h1473
        }
    };
    let a1475 = f512();
    if nat_lt(a1474, a1475) {
        Ok(Adt136::C1(Adt119::C0(v2, List::cons(v1, match v0.clone() {
            Adt119::C0(_, h1476, _) => {
                h1476
            }
        }), {
            let a1478 = match v0 {
                Adt119::C0(_, _, h1477) => {
                    h1477
                }
            };
            let a1479 = 1u64;
            nat_add(a1478, a1479)?
        })))
    } else {
        Ok(f423(Adt97::C9(v3)))
    }
}

pub fn f468(v0: bool, v1: Adt123, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt120 {
    Adt120::C8(Adt122::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f469(v0: u64) -> i64 {
    let a1480 = v0;
    let a1481 = 0u64;
    if nat_eq(a1480, a1481) {
        0i64
    } else {
        let a1482 = v0;
        let a1483 = 1u64;
        if nat_eq(a1482, a1483) {
            1i64
        } else {
            let a1484 = v0;
            let a1485 = 2u64;
            if nat_eq(a1484, a1485) {
                2i64
            } else {
                let a1486 = v0;
                let a1487 = 3u64;
                if nat_eq(a1486, a1487) {
                    3i64
                } else {
                    let a1488 = v0;
                    let a1489 = 4u64;
                    if nat_eq(a1488, a1489) {
                        4i64
                    } else {
                        let a1490 = v0;
                        let a1491 = 5u64;
                        if nat_eq(a1490, a1491) {
                            5i64
                        } else {
                            let a1492 = v0;
                            let a1493 = 6u64;
                            if nat_eq(a1492, a1493) {
                                6i64
                            } else {
                                let a1494 = v0;
                                let a1495 = 7u64;
                                if nat_eq(a1494, a1495) {
                                    7i64
                                } else {
                                    let a1496 = v0;
                                    let a1497 = 8u64;
                                    if nat_eq(a1496, a1497) {
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

pub fn f470(p1500: List<Adt6>, p1501: List<Adt6>) -> List<Adt6> {
    let m1499 = (p1500, p1501);
    {
        let mut m1499 = m1499;
        loop {
            let step = {
                let (v0, v1) = m1499;
                let m1498 = v0;
                match m1498.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1499 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: Adt119, v1: u64) -> Adt136 {
    let m1503 = match v0.clone() {
        Adt119::C0(_, h1502, _) => {
            h1502
        }
    };
    match m1503.uncons() {
        None => {
            f423(Adt97::C2(v1))
        }
        Some((v2, _)) => {
            let m1504 = v2;
            match m1504 {
                Adt124::C0(_) => {
                    f422(v0, Adt120::C0)
                }
                Adt124::C1(_, _) => {
                    f422(v0, Adt120::C4)
                }
            }
        }
    }
}

pub fn f472(p1507: List<(Str, Adt6)>, p1508: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1506 = (p1507, p1508);
    {
        let mut m1506 = m1506;
        loop {
            let step = {
                let (v0, v1) = m1506;
                let m1505 = v0;
                match m1505.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
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

pub fn f473(v0: Adt119, v1: Adt120) -> Adt119 {
    Adt119::C0(v1, match v0.clone() {
        Adt119::C0(_, h1509, _) => {
            h1509
        }
    }, match v0 {
        Adt119::C0(_, _, h1510) => {
            h1510
        }
    })
}

pub fn f474(v0: Adt119, v1: u64, v2: List<u64>, v3: bool) -> Adt136 {
    let m1512 = {
        let a1511 = f319(f320(), f333(v2, List::<u64>::nil()));
        utf8_decode(a1511)
    };
    match m1512 {
        None => {
            f423(Adt97::C14)
        }
        Some(v4) => {
            if v3 {
                f557(v0, v1, v4)
            } else {
                f480(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f475(v0: u64) -> u64 {
    let a1513 = v0;
    let a1514 = 34u64;
    if nat_eq(a1513, a1514) {
        34u64
    } else {
        let a1515 = v0;
        let a1516 = 92u64;
        if nat_eq(a1515, a1516) {
            92u64
        } else {
            let a1517 = v0;
            let a1518 = 47u64;
            if nat_eq(a1517, a1518) {
                47u64
            } else {
                let a1519 = v0;
                let a1520 = 98u64;
                if nat_eq(a1519, a1520) {
                    8u64
                } else {
                    let a1521 = v0;
                    let a1522 = 102u64;
                    if nat_eq(a1521, a1522) {
                        12u64
                    } else {
                        let a1523 = v0;
                        let a1524 = 110u64;
                        if nat_eq(a1523, a1524) {
                            10u64
                        } else {
                            let a1525 = v0;
                            let a1526 = 114u64;
                            if nat_eq(a1525, a1526) {
                                13u64
                            } else {
                                let a1527 = v0;
                                let a1528 = 116u64;
                                if nat_eq(a1527, a1528) {
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

pub fn f476(v0: u64) -> u64 {
    let a1529 = v0;
    let a1530 = 48u64;
    if nat_lt(a1529, a1530) {
        16u64
    } else {
        let a1531 = v0;
        let a1532 = 58u64;
        if nat_lt(a1531, a1532) {
            let a1533 = v0;
            let a1534 = 48u64;
            nat_sub(a1533, a1534)
        } else {
            let a1535 = v0;
            let a1536 = 65u64;
            if nat_lt(a1535, a1536) {
                16u64
            } else {
                let a1537 = v0;
                let a1538 = 71u64;
                if nat_lt(a1537, a1538) {
                    let a1539 = v0;
                    let a1540 = 55u64;
                    nat_sub(a1539, a1540)
                } else {
                    let a1541 = v0;
                    let a1542 = 97u64;
                    if nat_lt(a1541, a1542) {
                        16u64
                    } else {
                        let a1543 = v0;
                        let a1544 = 103u64;
                        if nat_lt(a1543, a1544) {
                            let a1545 = v0;
                            let a1546 = 87u64;
                            nat_sub(a1545, a1546)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f477(v0: Adt119, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt136> {
    let a1547 = v3;
    let a1548 = 0u64;
    if nat_eq(a1547, a1548) {
        if f414(v2, 55296u64, 56319u64) {
            Ok(f422(v0, Adt120::C6(Adt121::C3(v2), v4, v5)))
        } else {
            if f414(v2, 56320u64, 57343u64) {
                Ok(f423(Adt97::C6(v1)))
            } else {
                Ok(f422(v0, Adt120::C6(Adt121::C0, f558(v2, v4)?, v5)))
            }
        }
    } else {
        if f414(v2, 56320u64, 57343u64) {
            Ok(f422(v0, Adt120::C6(Adt121::C0, f558({
                let a1557 = 65536u64;
                let a1558 = {
                    let a1555 = {
                        let a1551 = {
                            let a1549 = v3;
                            let a1550 = 55296u64;
                            nat_sub(a1549, a1550)
                        };
                        let a1552 = 1024u64;
                        nat_mul(a1551, a1552)?
                    };
                    let a1556 = {
                        let a1553 = v2;
                        let a1554 = 56320u64;
                        nat_sub(a1553, a1554)
                    };
                    nat_add(a1555, a1556)?
                };
                nat_add(a1557, a1558)?
            }, v4)?, v5)))
        } else {
            Ok(f423(Adt97::C6(v1)))
        }
    }
}

pub fn f478(v0: u64, v1: u64) -> u64 {
    let m1561 = {
        let a1559 = f479(v0);
        let a1560 = v1;
        index_list(a1559, a1560)
    };
    match m1561 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f479(v0: u64) -> List<u64> {
    let a1562 = v0;
    let a1563 = 0u64;
    if nat_eq(a1562, a1563) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1564 = v0;
        let a1565 = 1u64;
        if nat_eq(a1564, a1565) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f480(v0: Adt119, v1: Adt6) -> Adt136 {
    Adt136::C1(f416(v1, match v0.clone() {
        Adt119::C0(_, h1566, _) => {
            h1566
        }
    }, match v0 {
        Adt119::C0(_, _, h1567) => {
            h1567
        }
    }))
}

pub fn f481(v0: u64) -> Adt6 {
    let a1568 = v0;
    let a1569 = 0u64;
    if nat_eq(a1568, a1569) {
        Adt6::C1(true)
    } else {
        let a1570 = v0;
        let a1571 = 1u64;
        if nat_eq(a1570, a1571) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f482(v0: Adt119, v1: Adt122, v2: Adt123) -> Adt136 {
    f422(v0, Adt120::C8(Adt122::C0(match v1.clone() {
        Adt122::C0(h1572, _, _, _, _, _, _) => {
            h1572
        }
    }, v2, match v1.clone() {
        Adt122::C0(_, _, h1573, _, _, _, _) => {
            h1573
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, h1574, _, _, _) => {
            h1574
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, _, h1575, _, _) => {
            h1575
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, _, _, h1576, _) => {
            h1576
        }
    }, match v1 {
        Adt122::C0(_, _, _, _, _, _, h1577) => {
            h1577
        }
    })))
}

pub fn f483(v0: Adt119, v1: u64, v2: u64, v3: Adt122, v4: bool) -> R<Adt136> {
    let v5: u64 = {
        let a1579 = match v3.clone() {
            Adt122::C0(_, _, _, h1578, _, _, _) => {
                h1578
            }
        };
        let a1580 = 0u64;
        if if nat_eq(a1579, a1580) {
            let a1581 = v2;
            let a1582 = 48u64;
            nat_eq(a1581, a1582)
        } else {
            false
        } {
            0u64
        } else {
            let a1584 = match v3.clone() {
                Adt122::C0(_, _, _, h1583, _, _, _) => {
                    h1583
                }
            };
            let a1585 = 1u64;
            nat_add(a1584, a1585)?
        }
    };
    let v6: i64 = if v4 {
        f316(match v3.clone() {
            Adt122::C0(_, _, _, _, h1586, _, _) => {
                h1586
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt122::C0(_, _, _, _, h1587, _, _) => {
                h1587
            }
        }
    };
    let a1588 = f559();
    let a1589 = v5;
    if nat_lt(a1588, a1589) {
        Ok(f423(Adt97::C8(v1)))
    } else {
        let m1592 = {
            let a1590 = 99999i64;
            let a1591 = v6;
            compare(a1590, a1591)
        };
        if match m1592 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f423(Adt97::C8(v1)))
        } else {
            Ok(f422(v0, f468(match v3.clone() {
                Adt122::C0(h1593, _, _, _, _, _, _) => {
                    h1593
                }
            }, if v4 {
                Adt123::C4
            } else {
                Adt123::C2
            }, f316({
                let a1595 = match v3.clone() {
                    Adt122::C0(_, _, h1594, _, _, _, _) => {
                        h1594
                    }
                };
                let a1596 = 10i64;
                int_mul(a1595, a1596)?
            }, f469({
                let a1597 = v2;
                let a1598 = 48u64;
                nat_sub(a1597, a1598)
            }))?, v5, v6, match v3.clone() {
                Adt122::C0(_, _, _, _, _, h1599, _) => {
                    h1599
                }
            }, match v3 {
                Adt122::C0(_, _, _, _, _, _, h1600) => {
                    h1600
                }
            })))
        }
    }
}

pub fn f484(v0: u64) -> bool {
    let a1601 = v0;
    let a1602 = 101u64;
    if nat_eq(a1601, a1602) {
        true
    } else {
        let a1603 = v0;
        let a1604 = 69u64;
        nat_eq(a1603, a1604)
    }
}

pub fn f485(v0: u64) -> bool {
    f414(v0, 48u64, 57u64)
}

pub fn f486(v0: Adt119, v1: u64, v2: u64, v3: Adt122) -> R<Adt136> {
    let m1605 = f415(v3)?;
    match m1605 {
        Adt140::C0 => {
            Ok(f423(Adt97::C7(v1)))
        }
        Adt140::C1 => {
            Ok(f423(Adt97::C8(v1)))
        }
        Adt140::C2(v4) => {
            Ok(f420(f416(v4, match v0.clone() {
                Adt119::C0(_, h1606, _) => {
                    h1606
                }
            }, match v0 {
                Adt119::C0(_, _, h1607) => {
                    h1607
                }
            }), v1, v2))
        }
    }
}

pub fn f487(v0: Adt119, v1: u64, v2: u64, v3: Adt122, v4: bool) -> R<Adt136> {
    let v5: i64 = f316({
        let a1609 = match v3.clone() {
            Adt122::C0(_, _, _, _, _, _, h1608) => {
                h1608
            }
        };
        let a1610 = 10i64;
        int_mul(a1609, a1610)?
    }, f469({
        let a1611 = v2;
        let a1612 = 48u64;
        nat_sub(a1611, a1612)
    }))?;
    let m1615 = {
        let a1613 = 99999i64;
        let a1614 = v5;
        compare(a1613, a1614)
    };
    if match m1615 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f423(Adt97::C8(v1)))
    } else {
        Ok(f422(v0, f468(match v3.clone() {
            Adt122::C0(h1616, _, _, _, _, _, _) => {
                h1616
            }
        }, Adt123::C7, match v3.clone() {
            Adt122::C0(_, _, h1617, _, _, _, _) => {
                h1617
            }
        }, match v3.clone() {
            Adt122::C0(_, _, _, h1618, _, _, _) => {
                h1618
            }
        }, match v3 {
            Adt122::C0(_, _, _, _, h1619, _, _) => {
                h1619
            }
        }, v4, v5)))
    }
}

pub fn f488(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt92> {
    let m1620 = f560(f561(), v2.clone());
    match m1620 {
        None => {
            let m1621 = f562(f563(), v2.clone());
            match m1621 {
                None => {
                    f564(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt92::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt92::C4(v0, v3))
        }
    }
}

pub fn f489(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt125) -> R<Adt146> {
    let m1622 = f262(Str::lit("x-mcp-header"), v1.clone());
    match m1622 {
        Adt107::C0 => {
            Ok(Adt146::C1(v4))
        }
        Adt107::C1(v5) => {
            f565(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f490(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt109>) -> List<Adt109> {
    f566(Str::lit("properties"), v0.clone(), v1, v2, f566(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f567(Str::lit("items"), v0.clone(), f567(Str::lit("additionalProperties"), v0.clone(), f568(Str::lit("allOf"), v0.clone(), f568(Str::lit("anyOf"), v0.clone(), f568(Str::lit("oneOf"), v0.clone(), f567(Str::lit("not"), v0.clone(), f567(Str::lit("if"), v0.clone(), f567(Str::lit("then"), v0.clone(), f567(Str::lit("else"), v0, v3)))))))))))
}

pub fn f491(p1629: List<(Str, Adt141)>) -> Adt126 {
    let m1628 = p1629;
    {
        let mut m1628 = m1628;
        loop {
            let step = {
                let v0 = m1628;
                let m1623 = v0;
                match m1623.uncons() {
                    None => {
                        Err::<List<(Str, Adt141)>, Adt126>(Adt126::C3(List::<Adt105>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1625 = {
                            let (_, h1624) = v1;
                            h1624
                        };
                        match m1625 {
                            Adt141::C0 => {
                                Ok::<List<(Str, Adt141)>, Adt126>(v2)
                            }
                            Adt141::C1 => {
                                Err::<List<(Str, Adt141)>, Adt126>(Adt126::C0({
                                    let (h1626, _) = v1;
                                    h1626
                                }))
                            }
                            Adt141::C2 => {
                                Err::<List<(Str, Adt141)>, Adt126>(Adt126::C1({
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

pub fn f492(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt141> {
    let m1630 = f262(Str::lit("type"), v0);
    match m1630 {
        Adt107::C0 => {
            Ok(Adt141::C0)
        }
        Adt107::C1(v2) => {
            let m1631 = f266(v2.clone());
            match m1631 {
                None => {
                    let m1632 = f242(v2);
                    match m1632 {
                        None => {
                            Ok(Adt141::C2)
                        }
                        Some(v4) => {
                            f571(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f569(v3.clone()) {
                        if f570(v3, v1)? {
                            Ok(Adt141::C0)
                        } else {
                            Ok(Adt141::C1)
                        }
                    } else {
                        Ok(Adt141::C2)
                    }
                }
            }
        }
    }
}

pub fn f493(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt141> {
    let m1633 = f262(Str::lit("enum"), v0);
    match m1633 {
        Adt107::C0 => {
            Ok(Adt141::C0)
        }
        Adt107::C1(v2) => {
            let m1634 = f242(v2);
            match m1634 {
                None => {
                    Ok(Adt141::C2)
                }
                Some(v3) => {
                    f572(v3, v1)
                }
            }
        }
    }
}

pub fn f494(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt141> {
    let m1635 = f262(Str::lit("const"), v0);
    match m1635 {
        Adt107::C0 => {
            Ok(Adt141::C0)
        }
        Adt107::C1(v2) => {
            let m1636 = f573(v2, v1)?;
            match m1636 {
                Adt150::C0 => {
                    Ok(Adt141::C0)
                }
                Adt150::C1 => {
                    Ok(Adt141::C1)
                }
                Adt150::C2 => {
                    Ok(Adt141::C2)
                }
            }
        }
    }
}

pub fn f495(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt141> {
    let m1637 = f262(v0, v2);
    match m1637 {
        Adt107::C0 => {
            Ok(Adt141::C0)
        }
        Adt107::C1(v4) => {
            let m1638 = f574(v4);
            match m1638 {
                None => {
                    Ok(Adt141::C2)
                }
                Some(v5) => {
                    let m1639 = f574(v3);
                    match m1639 {
                        None => {
                            Ok(Adt141::C0)
                        }
                        Some(v6) => {
                            if f575(v1, f576(v6, v5)?) {
                                Ok(Adt141::C0)
                            } else {
                                Ok(Adt141::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f496(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt141)>> {
    let m1640 = v1;
    match m1640 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt141)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt141)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt141)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f577(Str::lit("minLength"), true, v0.clone(), f578(v4.clone())?)?), List::cons((Str::lit("maxLength"), f577(Str::lit("maxLength"), false, v0, f578(v4)?)?), List::<(Str, Adt141)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f577(Str::lit("minItems"), true, v0.clone(), f579(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f577(Str::lit("maxItems"), false, v0, f579(v5, 0u64)?)?), List::<(Str, Adt141)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f577(Str::lit("minProperties"), true, v0.clone(), f580(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f577(Str::lit("maxProperties"), false, v0, f580(v6, 0u64)?)?), List::<(Str, Adt141)>::nil())))
        }
    }
}

pub fn f497(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt141 {
    let m1641 = f262(Str::lit("required"), v0);
    match m1641 {
        Adt107::C0 => {
            Adt141::C0
        }
        Adt107::C1(v2) => {
            let m1642 = f242(v2);
            match m1642 {
                None => {
                    Adt141::C2
                }
                Some(v3) => {
                    let m1643 = f187(v1);
                    match m1643 {
                        None => {
                            Adt141::C0
                        }
                        Some(v4) => {
                            f581(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f498(v0: Adt126, v1: Adt126) -> Adt126 {
    let m1644 = v0.clone();
    match m1644 {
        Adt126::C0(_) => {
            v0
        }
        Adt126::C1(_) => {
            v0
        }
        Adt126::C2(_) => {
            v0
        }
        Adt126::C3(v5) => {
            let m1645 = v1.clone();
            match m1645 {
                Adt126::C0(_) => {
                    v1
                }
                Adt126::C1(_) => {
                    v1
                }
                Adt126::C2(_) => {
                    v1
                }
                Adt126::C3(v9) => {
                    Adt126::C3({
                        let a1646 = v5;
                        let a1647 = v9;
                        append_list(a1646, a1647)
                    })
                }
            }
        }
    }
}

pub fn f499(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt103>) -> Adt126 {
    let m1648 = f262(Str::lit("$ref"), v1);
    match m1648 {
        Adt107::C0 => {
            Adt126::C3(List::<Adt105>::nil())
        }
        Adt107::C1(v4) => {
            let m1649 = f266(v4);
            match m1649 {
                None => {
                    Adt126::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1650 = f582(v0, v5.clone());
                    match m1650 {
                        None => {
                            Adt126::C2(v5)
                        }
                        Some(v6) => {
                            Adt126::C3(List::cons(Adt105::C0(v6, v2, v3), List::<Adt105>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f500(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt103>) -> Adt126 {
    let m1651 = f262(Str::lit("properties"), v0);
    match m1651 {
        Adt107::C0 => {
            Adt126::C3(List::<Adt105>::nil())
        }
        Adt107::C1(v3) => {
            let m1652 = f187(v3);
            match m1652 {
                None => {
                    Adt126::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1653 = f187(v1);
                    match m1653 {
                        None => {
                            Adt126::C3(List::<Adt105>::nil())
                        }
                        Some(v5) => {
                            Adt126::C3(f583(match v4 {
                                Adt5::C0(h1654) => {
                                    h1654
                                }
                            }, match v5 {
                                Adt5::C0(h1655) => {
                                    h1655
                                }
                            }, v2, List::<Adt105>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f501(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt103>) -> Adt126 {
    let m1656 = f262(Str::lit("additionalProperties"), v0.clone());
    match m1656 {
        Adt107::C0 => {
            Adt126::C3(List::<Adt105>::nil())
        }
        Adt107::C1(v3) => {
            let m1657 = f187(v1);
            match m1657 {
                None => {
                    Adt126::C3(List::<Adt105>::nil())
                }
                Some(v4) => {
                    let m1658 = f317(v3.clone());
                    match m1658 {
                        None => {
                            Adt126::C3(f586(match v4 {
                                Adt5::C0(h1660) => {
                                    h1660
                                }
                            }, f585(v0), v3, v2, List::<Adt105>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt126::C3(List::<Adt105>::nil())
                            } else {
                                if f584(match v4 {
                                    Adt5::C0(h1659) => {
                                        h1659
                                    }
                                }, f585(v0)) {
                                    Adt126::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt126::C3(List::<Adt105>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f502(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt103>) -> R<Adt126> {
    let m1661 = f262(Str::lit("items"), v0);
    match m1661 {
        Adt107::C0 => {
            Ok(Adt126::C3(List::<Adt105>::nil()))
        }
        Adt107::C1(v3) => {
            let m1662 = f242(v1);
            match m1662 {
                None => {
                    Ok(Adt126::C3(List::<Adt105>::nil()))
                }
                Some(v4) => {
                    Ok(Adt126::C3(f587(v4, 0u64, v3, v2, List::<Adt105>::nil())?))
                }
            }
        }
    }
}

pub fn f503(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt103>) -> Adt126 {
    let m1663 = f262(Str::lit("allOf"), v0);
    match m1663 {
        Adt107::C0 => {
            Adt126::C3(List::<Adt105>::nil())
        }
        Adt107::C1(v3) => {
            let m1664 = f242(v3);
            match m1664 {
                None => {
                    Adt126::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1665 = v4.clone();
                    match m1665.uncons() {
                        None => {
                            Adt126::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt126::C3(f588(v4, v1, v2, List::<Adt105>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f504(v0: Adt106, v1: Adt126, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt103>) -> Adt126 {
    let m1666 = v1.clone();
    match m1666 {
        Adt126::C0(_) => {
            v1
        }
        Adt126::C1(_) => {
            v1
        }
        Adt126::C2(_) => {
            v1
        }
        Adt126::C3(v8) => {
            let m1667 = v8;
            match m1667.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt126::C3(List::cons(Adt105::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt105>::nil()))
                }
            }
        }
    }
}

pub fn f505(p1674: List<u64>, p1675: List<u64>) -> List<u64> {
    let m1673 = (p1674, p1675);
    {
        let mut m1673 = m1673;
        loop {
            let step = {
                let (v0, v1) = m1673;
                let m1668 = v0;
                match m1668.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1669 = v2;
                        let a1670 = 126u64;
                        if nat_eq(a1669, a1670) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1671 = v2;
                            let a1672 = 47u64;
                            if nat_eq(a1671, a1672) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1673 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(p1684: u64, p1685: List<u64>, p1686: List<u64>) -> R<Option<List<u64>>> {
    let m1683 = (p1684, (p1685, p1686));
    {
        let mut m1683 = m1683;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1683;
                let m1676 = v0;
                if m1676 == 0 {
                    let m1677 = v1;
                    match m1677.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1676 - 1;
                    let m1678 = v1;
                    match m1678.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1679 = v7;
                            match m1679.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1680 = v9;
                                    match m1680.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1681 = v11;
                                            match m1681.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1682 = f589(v6, v8, v10, v12, f590(v13.clone()))?;
                                                    match m1682 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f333(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(p1691: List<Adt50>, p1692: Adt7) -> R<bool> {
    let m1690 = (p1691, p1692);
    {
        let mut m1690 = m1690;
        loop {
            let step = {
                let (v0, v1) = m1690;
                let m1687 = v0;
                match m1687.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f591(f576(match v2.clone() {
                            Adt50::C0(h1688, _, _) => {
                                h1688
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1689, _, _) => {
                                    h1689
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
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

pub fn f508() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f509(v0: u64) -> List<u64> {
    f479(v0)
}

pub fn f510(v0: Adt7) -> R<bool> {
    let a1710 = {
        let m1696 = {
            let a1694 = f592();
            let a1695 = match v0.clone() {
                Adt7::C0(h1693, _) => {
                    h1693
                }
            };
            compare(a1694, a1695)
        };
        if match m1696 {
            Ordering::Less => {
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
            let m1701 = {
                let a1699 = match v0.clone() {
                    Adt7::C0(h1697, _) => {
                        h1697
                    }
                };
                let a1700 = {
                    let a1698 = f592();
                    int_neg(a1698)?
                };
                compare(a1699, a1700)
            };
            if match m1701 {
                Ordering::Less => {
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
                let m1705 = {
                    let a1703 = 9999i64;
                    let a1704 = match v0.clone() {
                        Adt7::C0(_, h1702) => {
                            h1702
                        }
                    };
                    compare(a1703, a1704)
                };
                if match m1705 {
                    Ordering::Less => {
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
                    let m1709 = {
                        let a1707 = match v0 {
                            Adt7::C0(_, h1706) => {
                                h1706
                            }
                        };
                        let a1708 = -9999i64;
                        compare(a1707, a1708)
                    };
                    match m1709 {
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
    Ok(bool_not(a1710))
}

pub fn f511(v0: Adt7) -> R<List<u64>> {
    let m1714 = {
        let a1712 = match v0.clone() {
            Adt7::C0(_, h1711) => {
                h1711
            }
        };
        let a1713 = 0i64;
        compare(a1712, a1713)
    };
    if if match m1714 {
        Ordering::Less => {
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
        let m1718 = {
            let a1716 = 0i64;
            let a1717 = match v0.clone() {
                Adt7::C0(_, h1715) => {
                    h1715
                }
            };
            compare(a1716, a1717)
        };
        match m1718 {
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
        let a1721 = f593(match v0.clone() {
            Adt7::C0(h1719, _) => {
                h1719
            }
        })?;
        let a1722 = List::cons(101u64, f593(match v0 {
            Adt7::C0(_, h1720) => {
                h1720
            }
        })?);
        Ok(append_list(a1721, a1722))
    } else {
        f593(match v0 {
            Adt7::C0(h1723, _) => {
                h1723
            }
        })
    }
}

pub fn f512() -> u64 {
    512u64
}

pub fn f513(v0: u64, v1: Adt111, v2: List<Adt111>) -> List<Adt111> {
    List::cons(Adt111::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f514(p1726: List<u64>, p1727: List<u64>) -> R<List<u64>> {
    let m1725 = (p1726, p1727);
    {
        let mut m1725 = m1725;
        loop {
            let step = {
                let (v0, v1) = m1725;
                let m1724 = v0;
                match m1724.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f333(f594(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f595(match v0.clone() {
        Adt33::C0(_, h1728, _, _) => {
            h1728
        }
    })), List::cons((Str::lit("capabilities"), f596(match v0.clone() {
        Adt33::C0(_, _, h1729, _) => {
            h1729
        }
    })), {
        let a1732 = f597(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1730) => {
                h1730
            }
        });
        let a1733 = f598(match v0 {
            Adt33::C0(h1731, _, _, _) => {
                h1731
            }
        })?;
        append_list(a1732, a1733)
    })))
}

pub fn f516(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f599(match v0.clone() {
        Adt34::C0(_, _, h1734) => {
            h1734
        }
    }))), {
        let a1737 = f597(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1735, _) => {
                h1735
            }
        });
        let a1738 = f598(match v0 {
            Adt34::C0(h1736, _, _) => {
                h1736
            }
        })?;
        append_list(a1737, a1738)
    }))
}

pub fn f517(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1739 = v0;
    match m1739 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1740 = f531(Adt6::C5(v1))?;
            match m1740 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1741 = f187(v2);
                    match m1741 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1742) => {
                                    h1742
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f518(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1744 = f600(match v0.clone() {
        Adt37::C0(h1743, _, _) => {
            h1743
        }
    })?;
    match m1744 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1747 = f601(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1745, _) => {
                        h1745
                    }
                });
                let a1748 = f602(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1746) => {
                        h1746
                    }
                });
                append_list(a1747, a1748)
            })))
        }
    }
}

pub fn f519(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f603(match v0.clone() {
        Adt51::C0(_, _, h1749) => {
            h1749
        }
    })?)), {
        let a1752 = f597(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1750, _) => {
                h1750
            }
        });
        let a1753 = f598(match v0 {
            Adt51::C0(h1751, _, _) => {
                h1751
            }
        })?;
        append_list(a1752, a1753)
    }))
}

pub fn f520(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f604(match v0.clone() {
        Adt52::C0(_, h1754) => {
            h1754
        }
    }))), f598(match v0 {
        Adt52::C0(h1755, _) => {
            h1755
        }
    })?))
}

pub fn f521(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f605(match v0.clone() {
        Adt53::C0(_, _, h1756) => {
            h1756
        }
    }))), {
        let a1759 = f597(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1757, _) => {
                h1757
            }
        });
        let a1760 = f598(match v0 {
            Adt53::C0(h1758, _, _) => {
                h1758
            }
        })?;
        append_list(a1759, a1760)
    }))
}

pub fn f522(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f606(match v0.clone() {
        Adt55::C0(_, _, h1761) => {
            h1761
        }
    }))), {
        let a1764 = f597(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1762, _) => {
                h1762
            }
        });
        let a1765 = f598(match v0 {
            Adt55::C0(h1763, _, _) => {
                h1763
            }
        })?;
        append_list(a1764, a1765)
    }))
}

pub fn f523(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1767 = f607(match v0.clone() {
        Adt58::C0(_, h1766) => {
            h1766
        }
    })?;
    match m1767 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1769 = f597(Str::lit("description"), match v0 {
                    Adt58::C0(h1768, _) => {
                        h1768
                    }
                });
                let a1770 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1769, a1770)
            }))
        }
    }
}

pub fn f524(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f541({
        let m1773 = match v0.clone() {
            Adt60::C0(h1772) => {
                h1772
            }
        };
        match m1773 {
            Adt61::C0(h1771, _, _) => {
                h1771
            }
        }
    }))), {
        let a1780 = f608(Str::lit("total"), {
            let m1776 = match v0.clone() {
                Adt60::C0(h1775) => {
                    h1775
                }
            };
            match m1776 {
                Adt61::C0(_, h1774, _) => {
                    h1774
                }
            }
        })?;
        let a1781 = f602(Str::lit("hasMore"), {
            let m1779 = match v0 {
                Adt60::C0(h1778) => {
                    h1778
                }
            };
            match m1779 {
                Adt61::C0(_, _, h1777) => {
                    h1777
                }
            }
        });
        append_list(a1780, a1781)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f525(v0: Adt81) -> R<Option<List<(Str, Adt6)>>> {
    let m1782 = f609(f610(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m1782 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1783 = f611(v1);
                let a1784 = f612(f613(v0));
                append_list(a1783, a1784)
            }))
        }
    }
}

pub fn f526(v0: Adt147) -> Str {
    let m1785 = v0;
    match m1785 {
        Adt147::C0 => {
            Str::lit("complete")
        }
        Adt147::C1 => {
            Str::lit("input_required")
        }
        Adt147::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f527(v0: Adt32) -> Adt147 {
    let m1786 = v0;
    match m1786 {
        Adt32::C0(_) => {
            Adt147::C0
        }
        Adt32::C1(_) => {
            Adt147::C0
        }
        Adt32::C2(_) => {
            Adt147::C0
        }
        Adt32::C3(_) => {
            Adt147::C0
        }
        Adt32::C4(_) => {
            Adt147::C0
        }
        Adt32::C5(_) => {
            Adt147::C0
        }
        Adt32::C6(_) => {
            Adt147::C0
        }
        Adt32::C7(_) => {
            Adt147::C0
        }
        Adt32::C8(_) => {
            Adt147::C0
        }
        Adt32::C9(_) => {
            Adt147::C0
        }
        Adt32::C10(_) => {
            Adt147::C0
        }
        Adt32::C11 => {
            Adt147::C0
        }
        Adt32::C12(_) => {
            Adt147::C1
        }
        Adt32::C13(_) => {
            Adt147::C2
        }
    }
}

pub fn f528(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1787 = v0;
    match m1787 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f614(), f615(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f529(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f616(v2) {
            let a1788 = v1;
            let a1789 = 200u64;
            nat_eq(a1788, a1789)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f530(v0: u64, v1: List<List<u64>>) -> Adt89 {
    Adt89::C0(v0, f617(), List::<List<u64>>::nil(), f618(v1))
}

pub fn f531(v0: Adt6) -> R<Option<Adt6>> {
    f619(f620(), Adt148::C0(v0), List::<Adt149>::nil())
}

pub fn f532(v0: Adt131) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f279())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f621(match v0.clone() {
        Adt131::C0(h1790, _) => {
            h1790
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1793 = match v0.clone() {
            Adt131::C0(_, h1792) => {
                h1792
            }
        };
        match m1793 {
            Adt50::C0(h1791, _, _) => {
                h1791
            }
        }
    })), {
        let a1800 = f622({
            let m1796 = match v0.clone() {
                Adt131::C0(_, h1795) => {
                    h1795
                }
            };
            match m1796 {
                Adt50::C0(_, h1794, _) => {
                    h1794
                }
            }
        });
        let a1801 = f623({
            let m1799 = match v0 {
                Adt131::C0(_, h1798) => {
                    h1798
                }
            };
            match m1799 {
                Adt50::C0(_, _, h1797) => {
                    h1797
                }
            }
        });
        append_list(a1800, a1801)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f533() -> u64 {
    429u64
}

pub fn f534() -> i64 {
    -32602i64
}

pub fn f535() -> i64 {
    -32022i64
}

pub fn f536() -> i64 {
    -32020i64
}

pub fn f537() -> i64 {
    -32601i64
}

pub fn f538(v0: Adt84) -> i64 {
    let m1802 = v0;
    match m1802 {
        Adt84::C0(_) => {
            f534()
        }
        Adt84::C1(_) => {
            f534()
        }
        Adt84::C2(_) => {
            f534()
        }
        Adt84::C3(_) => {
            f534()
        }
        Adt84::C4(_) => {
            f534()
        }
        Adt84::C5(_) => {
            f534()
        }
        Adt84::C6(_) => {
            f624()
        }
        Adt84::C7(_) => {
            f625()
        }
        Adt84::C8(_) => {
            f626()
        }
        Adt84::C9(_) => {
            f624()
        }
        Adt84::C10 => {
            f534()
        }
        Adt84::C11 => {
            f534()
        }
        Adt84::C12(_) => {
            f624()
        }
        Adt84::C13 => {
            f624()
        }
        Adt84::C14 => {
            f624()
        }
        Adt84::C15(_) => {
            f624()
        }
    }
}

pub fn f539(v0: Adt133) -> Str {
    let m1803 = v0;
    match m1803 {
        Adt133::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt133::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt133::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt133::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt133::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt133::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt133::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt133::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt133::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f540(v0: Adt113) -> Str {
    let m1804 = v0;
    match m1804 {
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
            let m1805 = v1;
            match m1805 {
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
            let m1806 = v2;
            match m1806 {
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

pub fn f541(v0: List<Str>) -> List<Adt6> {
    f470(f627(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f542(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1818 = f628(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1807, _, _, _, _) => {
                h1807
            }
        });
        let a1819 = {
            let a1816 = f628(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1808, _, _, _) => {
                    h1808
                }
            });
            let a1817 = {
                let a1814 = f628(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1809, _, _) => {
                        h1809
                    }
                });
                let a1815 = {
                    let a1812 = f628(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1810, _) => {
                            h1810
                        }
                    });
                    let a1813 = f628(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1811) => {
                            h1811
                        }
                    });
                    append_list(a1812, a1813)
                };
                append_list(a1814, a1815)
            };
            append_list(a1816, a1817)
        };
        append_list(a1818, a1819)
    })
}

pub fn f543(p1822: List<List<u64>>) -> R<bool> {
    let m1821 = p1822;
    {
        let mut m1821 = m1821;
        loop {
            let step = {
                let v0 = m1821;
                let m1820 = v0;
                match m1820.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f629(f294(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f545(v0: Adt0, v1: Fn0, v2: u64, v3: Adt138, v4: bool) -> R<Result<Adt129, Adt132>> {
    if v4 {
        f630(v0, v1, v2, f631(match v3.clone() {
            Adt138::C0(_, h1823, _, _) => {
                h1823
            }
        }, match v3.clone() {
            Adt138::C0(_, _, h1824, _) => {
                h1824
            }
        }, match v3 {
            Adt138::C0(_, _, _, h1825) => {
                h1825
            }
        }))
    } else {
        Ok(Err::<Adt129, Adt132>(Adt132::C3(f632(match v3 {
            Adt138::C0(_, h1826, _, _) => {
                h1826
            }
        }))))
    }
}

pub fn f546(v0: Adt4, v1: Adt139) -> bool {
    let m1827 = v1;
    match m1827 {
        Adt139::C0 => {
            true
        }
        Adt139::C1 => {
            f633(match v0 {
                Adt4::C0(_, _, _, _, _, h1828, _) => {
                    h1828
                }
            })
        }
        Adt139::C2 => {
            f633(match v0 {
                Adt4::C0(_, _, _, _, _, h1829, _) => {
                    h1829
                }
            })
        }
        Adt139::C3 => {
            f634(match v0 {
                Adt4::C0(_, _, _, _, h1830, _, _) => {
                    h1830
                }
            })
        }
        Adt139::C4 => {
            f634(match v0 {
                Adt4::C0(_, _, _, _, h1831, _, _) => {
                    h1831
                }
            })
        }
        Adt139::C5 => {
            f634(match v0 {
                Adt4::C0(_, _, _, _, h1832, _, _) => {
                    h1832
                }
            })
        }
        Adt139::C6 => {
            f633(match v0 {
                Adt4::C0(_, _, _, h1833, _, _, _) => {
                    h1833
                }
            })
        }
        Adt139::C7 => {
            f633(match v0 {
                Adt4::C0(_, _, _, h1834, _, _, _) => {
                    h1834
                }
            })
        }
        Adt139::C8 => {
            f250(match v0 {
                Adt4::C0(_, _, h1835, _, _, _, _) => {
                    h1835
                }
            })
        }
        Adt139::C9 => {
            f635(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1836) => {
                    h1836
                }
            }, f636())
        }
        Adt139::C10 => {
            f635(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1837) => {
                    h1837
                }
            }, f636())
        }
        Adt139::C11 => {
            f635(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1838) => {
                    h1838
                }
            }, f636())
        }
    }
}

pub fn f547(v0: Adt144, v1: Adt142) -> Result<Adt138, Adt132> {
    let m1839 = v1;
    match m1839 {
        Adt142::C0 => {
            Err::<Adt138, Adt132>(Adt132::C3(match v0 {
                Adt144::C0(_, h1840, _, _) => {
                    h1840
                }
            }))
        }
        Adt142::C1(v2) => {
            Ok::<Adt138, Adt132>(Adt138::C0(match v0.clone() {
                Adt144::C0(h1841, _, _, _) => {
                    h1841
                }
            }, v2, match v0.clone() {
                Adt144::C0(_, _, h1842, _) => {
                    h1842
                }
            }, match v0 {
                Adt144::C0(_, _, _, h1843) => {
                    h1843
                }
            }))
        }
    }
}

pub fn f548(v0: Option<Adt112>, v1: List<Str>, v2: Adt108, v3: Result<Adt16, Adt133>) -> R<Adt143> {
    let m1844 = v3;
    match m1844 {
        Ok(v5) => {
            Ok(f637(v1, v2.clone(), v5.clone(), f638(v0, f458(match v2.clone() {
                Adt108::C0(_, h1845, _) => {
                    h1845
                }
            }), match v2.clone() {
                Adt108::C0(_, h1846, _) => {
                    h1846
                }
            }, match v5 {
                Adt16::C0(h1847, _, _, _, _, _) => {
                    h1847
                }
            }, f639(match v2 {
                Adt108::C0(_, _, h1848) => {
                    h1848
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt143::C1(Adt132::C0(v4)))
        }
    }
}

pub fn f549(v0: Option<Adt5>) -> Result<Adt16, Adt133> {
    let m1849 = v0;
    match m1849 {
        None => {
            Err::<Adt16, Adt133>(Adt133::C0)
        }
        Some(v1) => {
            let m1850 = f186(Str::lit("_meta"), v1);
            match m1850 {
                Adt107::C0 => {
                    Err::<Adt16, Adt133>(Adt133::C1)
                }
                Adt107::C1(v2) => {
                    let m1851 = f187(v2);
                    match m1851 {
                        None => {
                            Err::<Adt16, Adt133>(Adt133::C4)
                        }
                        Some(v3) => {
                            f640(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f550(v0: Adt139) -> Adt145 {
    let m1852 = v0;
    match m1852 {
        Adt139::C0 => {
            Adt145::C0
        }
        Adt139::C1 => {
            Adt145::C0
        }
        Adt139::C2 => {
            Adt145::C1
        }
        Adt139::C3 => {
            Adt145::C0
        }
        Adt139::C4 => {
            Adt145::C2
        }
        Adt139::C5 => {
            Adt145::C0
        }
        Adt139::C6 => {
            Adt145::C0
        }
        Adt139::C7 => {
            Adt145::C1
        }
        Adt139::C8 => {
            Adt145::C0
        }
        Adt139::C9 => {
            Adt145::C3
        }
        Adt139::C10 => {
            Adt145::C3
        }
        Adt139::C11 => {
            Adt145::C3
        }
    }
}

pub fn f551(p1858: List<u64>, p1859: List<u64>) -> Option<List<u64>> {
    let m1857 = (p1858, p1859);
    {
        let mut m1857 = m1857;
        loop {
            let step = {
                let (v0, v1) = m1857;
                let m1853 = v0;
                match m1853.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1854 = v1;
                        match m1854.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1855 = v4;
                                let a1856 = v2;
                                if nat_eq(a1855, a1856) {
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
                Ok(next) => { m1857 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f553() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f554(v0: Adt12, v1: Option<Str>) -> bool {
    let m1860 = v1;
    match m1860 {
        None => {
            false
        }
        Some(v2) => {
            let a1862 = match v0 {
                Adt12::C0(h1861, _, _, _) => {
                    h1861
                }
            };
            let a1863 = v2;
            equal(a1862, a1863)
        }
    }
}

pub fn f555(v0: List<Adt87>, v1: Str) -> R<Adt116> {
    let m1866 = f404(f134(v0, {
        let a1864 = f641();
        let a1865 = f642(v1)?;
        append_list(a1864, a1865)
    })?)?;
    match m1866 {
        Adt100::C0 => {
            Ok(Adt116::C0)
        }
        Adt100::C1 => {
            Ok(Adt116::C1)
        }
        Adt100::C2 => {
            Ok(Adt116::C1)
        }
        Adt100::C3(v2) => {
            let m1867 = f457(v2);
            match m1867 {
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

pub fn f556(v0: i64) -> bool {
    let a1874 = {
        let m1870 = {
            let a1868 = v0;
            let a1869 = 0i64;
            compare(a1868, a1869)
        };
        if match m1870 {
            Ordering::Less => {
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
            let m1873 = {
                let a1871 = 0i64;
                let a1872 = v0;
                compare(a1871, a1872)
            };
            match m1873 {
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
    bool_not(a1874)
}

pub fn f557(v0: Adt119, v1: u64, v2: Str) -> Adt136 {
    let m1876 = match v0.clone() {
        Adt119::C0(_, h1875, _) => {
            h1875
        }
    };
    match m1876.uncons() {
        None => {
            f423(Adt97::C2(v1))
        }
        Some((v3, v4)) => {
            let m1877 = v3;
            match m1877 {
                Adt124::C0(_) => {
                    f423(Adt97::C2(v1))
                }
                Adt124::C1(v6, _) => {
                    Adt136::C1(Adt119::C0(Adt120::C5, List::cons(Adt124::C1(v6, Some(v2)), v4), match v0 {
                        Adt119::C0(_, _, h1878) => {
                            h1878
                        }
                    }))
                }
            }
        }
    }
}

pub fn f558(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1879 = v0;
    let a1880 = 128u64;
    if nat_lt(a1879, a1880) {
        Ok(List::cons(v0, v1))
    } else {
        let a1881 = v0;
        let a1882 = 2048u64;
        if nat_lt(a1881, a1882) {
            Ok(List::cons({
                let a1883 = 128u64;
                let a1884 = f643(v0, 64u64);
                nat_add(a1883, a1884)?
            }, List::cons({
                let a1885 = 192u64;
                let a1886 = f644(v0, 64u64);
                nat_add(a1885, a1886)?
            }, v1)))
        } else {
            let a1887 = v0;
            let a1888 = 65536u64;
            if nat_lt(a1887, a1888) {
                Ok(List::cons({
                    let a1889 = 128u64;
                    let a1890 = f643(v0, 64u64);
                    nat_add(a1889, a1890)?
                }, List::cons({
                    let a1891 = 128u64;
                    let a1892 = f643(f644(v0, 64u64), 64u64);
                    nat_add(a1891, a1892)?
                }, List::cons({
                    let a1893 = 224u64;
                    let a1894 = f644(v0, 4096u64);
                    nat_add(a1893, a1894)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1895 = 128u64;
                    let a1896 = f643(v0, 64u64);
                    nat_add(a1895, a1896)?
                }, List::cons({
                    let a1897 = 128u64;
                    let a1898 = f643(f644(v0, 64u64), 64u64);
                    nat_add(a1897, a1898)?
                }, List::cons({
                    let a1899 = 128u64;
                    let a1900 = f643(f644(v0, 4096u64), 64u64);
                    nat_add(a1899, a1900)?
                }, List::cons({
                    let a1901 = 240u64;
                    let a1902 = f644(v0, 262144u64);
                    nat_add(a1901, a1902)?
                }, v1)))))
            }
        }
    }
}

pub fn f559() -> u64 {
    18u64
}

pub fn f560(p1906: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1905 = p1906;
    {
        let mut m1905 = m1905;
        loop {
            let step = {
                let v0 = m1905;
                let m1903 = v0;
                match m1903.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1904 = f262(v2.clone(), v1.clone());
                        match m1904 {
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
                Ok(next) => { m1905 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f562(p1913: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1912 = p1913;
    {
        let mut m1912 = m1912;
        loop {
            let step = {
                let v0 = m1912;
                let m1907 = v0;
                match m1907.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1909 = f262({
                            let (h1908, _) = v2.clone();
                            h1908
                        }, v1.clone());
                        match m1909 {
                            Adt107::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt107::C1(v4) => {
                                if f645({
                                    let (_, h1910) = v2;
                                    h1910
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1911, _) = v2;
                                        h1911
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f564(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt92> {
    let m1914 = f262(Str::lit("$ref"), v2);
    match m1914 {
        Adt107::C0 => {
            None::<Adt92>
        }
        Adt107::C1(v3) => {
            let m1915 = f266(v3);
            match m1915 {
                None => {
                    None::<Adt92>
                }
                Some(v4) => {
                    let m1916 = f582(v1, v4.clone());
                    match m1916 {
                        None => {
                            Some(Adt92::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt92>
                        }
                    }
                }
            }
        }
    }
}

pub fn f565(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt125) -> R<Adt146> {
    if v3 {
        let m1917 = f266(v2);
        match m1917 {
            None => {
                Ok(Adt146::C0(Adt92::C7(v0)))
            }
            Some(v6) => {
                f646(v0, v1, v6.clone(), f233(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt146::C0(Adt92::C10(v0)))
    }
}

pub fn f566(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt109>) -> List<Adt109> {
    let m1918 = f262(v0.clone(), v1);
    match m1918 {
        Adt107::C0 => {
            v4
        }
        Adt107::C1(v5) => {
            let m1919 = f187(v5);
            match m1919 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1920 = v0;
                    let a1921 = Str::lit("properties");
                    if equal(a1920, a1921) {
                        f647(match v6 {
                            Adt5::C0(h1922) => {
                                h1922
                            }
                        }, v2, v3, v4)
                    } else {
                        f648(match v6 {
                            Adt5::C0(h1923) => {
                                h1923
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f567(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt109>) -> List<Adt109> {
    let m1924 = f262(v0, v1);
    match m1924 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            List::cons(Adt109::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f568(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt109>) -> List<Adt109> {
    let m1925 = f262(v0, v1);
    match m1925 {
        Adt107::C0 => {
            v2
        }
        Adt107::C1(v3) => {
            let m1926 = f242(v3);
            match m1926 {
                None => {
                    v2
                }
                Some(v4) => {
                    f649(v4, v2)
                }
            }
        }
    }
}

pub fn f569(v0: Str) -> bool {
    let a1927 = v0.clone();
    let a1928 = Str::lit("null");
    if equal(a1927, a1928) {
        true
    } else {
        let a1929 = v0.clone();
        let a1930 = Str::lit("boolean");
        if equal(a1929, a1930) {
            true
        } else {
            let a1931 = v0.clone();
            let a1932 = Str::lit("object");
            if equal(a1931, a1932) {
                true
            } else {
                let a1933 = v0.clone();
                let a1934 = Str::lit("array");
                if equal(a1933, a1934) {
                    true
                } else {
                    let a1935 = v0.clone();
                    let a1936 = Str::lit("number");
                    if equal(a1935, a1936) {
                        true
                    } else {
                        let a1937 = v0.clone();
                        let a1938 = Str::lit("string");
                        if equal(a1937, a1938) {
                            true
                        } else {
                            let a1939 = v0;
                            let a1940 = Str::lit("integer");
                            equal(a1939, a1940)
                        }
                    }
                }
            }
        }
    }
}

pub fn f570(v0: Str, v1: Adt6) -> R<bool> {
    let a1941 = v0.clone();
    let a1942 = f650(v1.clone());
    if equal(a1941, a1942) {
        Ok(true)
    } else {
        let a1943 = v0;
        let a1944 = Str::lit("integer");
        if equal(a1943, a1944) {
            let m1945 = f574(v1);
            match m1945 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f651(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f571(p1948: List<Adt6>, v1: Adt6, p1949: bool) -> R<Adt141> {
    let m1947 = (p1948, p1949);
    {
        let mut m1947 = m1947;
        loop {
            let step = {
                let (v0, v2) = m1947;
                let m1946 = v0;
                match m1946.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt141>>(Ok(Adt141::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt141>>(Ok(Adt141::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f569(f652(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt141>>((v4, if v2 {
                                true
                            } else {
                                f570(f652(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt141>>(Ok(Adt141::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1947 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(p1953: List<Adt6>, v1: Adt6) -> R<Adt141> {
    let m1952 = p1953;
    {
        let mut m1952 = m1952;
        loop {
            let step = {
                let v0 = m1952;
                let m1950 = v0;
                match m1950.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt141>>(Ok(Adt141::C1))
                    }
                    Some((v2, v3)) => {
                        let m1951 = f573(v2, v1.clone())?;
                        match m1951 {
                            Adt150::C0 => {
                                Err::<List<Adt6>, R<Adt141>>(Ok(Adt141::C0))
                            }
                            Adt150::C1 => {
                                Ok::<List<Adt6>, R<Adt141>>(v3)
                            }
                            Adt150::C2 => {
                                Err::<List<Adt6>, R<Adt141>>(Ok(Adt141::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1952 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: Adt6, v1: Adt6) -> R<Adt150> {
    f653(f654(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f574(v0: Adt6) -> Option<Adt7> {
    let m1954 = v0;
    match m1954 {
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

pub fn f575(v0: u64, v1: Adt151) -> bool {
    let a1955 = v0;
    let a1956 = 0u64;
    if nat_eq(a1955, a1956) {
        let a1957 = f655(v1);
        bool_not(a1957)
    } else {
        let a1958 = v0;
        let a1959 = 1u64;
        if nat_eq(a1958, a1959) {
            let a1960 = f591(v1);
            bool_not(a1960)
        } else {
            let a1961 = v0;
            let a1962 = 2u64;
            if nat_eq(a1961, a1962) {
                f591(v1)
            } else {
                f655(v1)
            }
        }
    }
}

pub fn f576(v0: Adt7, v1: Adt7) -> R<Adt151> {
    f656(v0.clone(), v1.clone(), f657(match v0 {
        Adt7::C0(h1963, _) => {
            h1963
        }
    }), f657(match v1 {
        Adt7::C0(h1964, _) => {
            h1964
        }
    }))
}

pub fn f577(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt141> {
    let m1965 = f262(v0, v2);
    match m1965 {
        Adt107::C0 => {
            Ok(Adt141::C0)
        }
        Adt107::C1(v4) => {
            let m1966 = f658(v4);
            match m1966 {
                None => {
                    Ok(Adt141::C2)
                }
                Some(v5) => {
                    let m1969 = {
                        let a1967 = v5;
                        let a1968 = 0i64;
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
                        Ok(Adt141::C2)
                    } else {
                        if v1 {
                            if f655(f659(f241(v3, 0i64)?, v5)) {
                                Ok(Adt141::C1)
                            } else {
                                Ok(Adt141::C0)
                            }
                        } else {
                            if f591(f659(f241(v3, 0i64)?, v5)) {
                                Ok(Adt141::C1)
                            } else {
                                Ok(Adt141::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f578(v0: Str) -> R<u64> {
    f660(f206({
        let a1970 = v0;
        utf8_encode(a1970)
    })?, 0u64)
}

pub fn f579(p1975: List<Adt6>, p1976: u64) -> R<u64> {
    let m1974 = (p1975, p1976);
    {
        let mut m1974 = m1974;
        loop {
            let step = {
                let (v0, v1) = m1974;
                let m1971 = v0;
                match m1971.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1972 = v1;
                            let a1973 = 1u64;
                            nat_add(a1972, a1973)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1974 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(p1981: List<(Str, Adt6)>, p1982: u64) -> R<u64> {
    let m1980 = (p1981, p1982);
    {
        let mut m1980 = m1980;
        loop {
            let step = {
                let (v0, v1) = m1980;
                let m1977 = v0;
                match m1977.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1978 = v1;
                            let a1979 = 1u64;
                            nat_add(a1978, a1979)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1980 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(p1986: List<Adt6>, v1: Adt5) -> Adt141 {
    let m1985 = p1986;
    {
        let mut m1985 = m1985;
        loop {
            let step = {
                let v0 = m1985;
                let m1983 = v0;
                match m1983.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt141>(Adt141::C0)
                    }
                    Some((v2, v3)) => {
                        let m1984 = f266(v2);
                        match m1984 {
                            None => {
                                Err::<List<Adt6>, Adt141>(Adt141::C2)
                            }
                            Some(v4) => {
                                if f411(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt141>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt141>(Adt141::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1985 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1987 = v1.clone();
    let a1988 = Str::lit("#");
    if equal(a1987, a1988) {
        Some(v0)
    } else {
        let m1989 = f187(v0);
        match m1989 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1990 = f186(Str::lit("$defs"), v2);
                match m1990 {
                    Adt107::C0 => {
                        None::<Adt6>
                    }
                    Adt107::C1(v3) => {
                        let m1991 = f187(v3);
                        match m1991 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f661(v1, match v4 {
                                    Adt5::C0(h1992) => {
                                        h1992
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

pub fn f583(p1999: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt103>, p2000: List<Adt105>) -> List<Adt105> {
    let m1998 = (p1999, p2000);
    {
        let mut m1998 = m1998;
        loop {
            let step = {
                let (v0, v3) = m1998;
                let m1993 = v0;
                match m1993.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1995 = f262({
                            let (h1994, _) = v4.clone();
                            h1994
                        }, v1.clone());
                        match m1995 {
                            Adt107::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v5, v3))
                            }
                            Adt107::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v5, List::cons(Adt105::C0({
                                    let (_, h1996) = v4;
                                    h1996
                                }, v6, List::cons(Adt103::C0({
                                    let (h1997, _) = v4;
                                    h1997
                                }), v2.clone())), v3)))
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

pub fn f584(p2005: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2004 = p2005;
    {
        let mut m2004 = m2004;
        loop {
            let step = {
                let v0 = m2004;
                let m2001 = v0;
                match m2001.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2003 = f262({
                            let (h2002, _) = v2;
                            h2002
                        }, v1.clone());
                        match m2003 {
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
                Ok(next) => { m2004 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f585(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2006 = f262(Str::lit("properties"), v0);
    match m2006 {
        Adt107::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt107::C1(v1) => {
            let m2007 = f187(v1);
            match m2007 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2008) => {
                            h2008
                        }
                    }
                }
            }
        }
    }
}

pub fn f586(p2015: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt103>, p2016: List<Adt105>) -> List<Adt105> {
    let m2014 = (p2015, p2016);
    {
        let mut m2014 = m2014;
        loop {
            let step = {
                let (v0, v4) = m2014;
                let m2009 = v0;
                match m2009.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2011 = f262({
                            let (h2010, _) = v5.clone();
                            h2010
                        }, v1.clone());
                        match m2011 {
                            Adt107::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v6, List::cons(Adt105::C0(v2.clone(), {
                                    let (_, h2012) = v5;
                                    h2012
                                }, List::cons(Adt103::C0({
                                    let (h2013, _) = v5;
                                    h2013
                                }), v3.clone())), v4)))
                            }
                            Adt107::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2014 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(p2021: List<Adt6>, p2022: u64, v2: Adt6, v3: List<Adt103>, p2023: List<Adt105>) -> R<List<Adt105>> {
    let m2020 = (p2021, (p2022, p2023));
    {
        let mut m2020 = m2020;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2020;
                let m2017 = v0;
                match m2017.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt105>)), R<List<Adt105>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt105>)), R<List<Adt105>>>((v6, ({
                            let a2018 = v1;
                            let a2019 = 1u64;
                            nat_add(a2018, a2019)?
                        }, List::cons(Adt105::C0(v2.clone(), v5, List::cons(Adt103::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(p2026: List<Adt6>, v1: Adt6, v2: List<Adt103>, p2027: List<Adt105>) -> List<Adt105> {
    let m2025 = (p2026, p2027);
    {
        let mut m2025 = m2025;
        loop {
            let step = {
                let (v0, v3) = m2025;
                let m2024 = v0;
                match m2024.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt105>), List<Adt105>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt105>), List<Adt105>>((v5, List::cons(Adt105::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2025 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2028 = f662(v0)?;
    match m2028 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2029 = f662(v1)?;
            match m2029 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2030 = v2;
                    let a2031 = 61u64;
                    if nat_eq(a2030, a2031) {
                        f663(v5, v6, v4, v3)
                    } else {
                        let m2032 = f662(v2)?;
                        match m2032 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2033 = v3;
                                let a2034 = 61u64;
                                if nat_eq(a2033, a2034) {
                                    f664(v5, v6, v7, v4)
                                } else {
                                    let m2035 = f662(v3)?;
                                    match m2035 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f665(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f590(v0: List<u64>) -> bool {
    let m2036 = v0;
    match m2036.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f591(v0: Adt151) -> bool {
    let m2037 = v0;
    match m2037 {
        Adt151::C0 => {
            false
        }
        Adt151::C1 => {
            false
        }
        Adt151::C2 => {
            true
        }
    }
}

pub fn f592() -> i64 {
    999999999999999999i64
}

pub fn f593(v0: i64) -> R<List<u64>> {
    let m2040 = {
        let a2038 = v0;
        let a2039 = 0i64;
        compare(a2038, a2039)
    };
    if match m2040 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f666(20u64, {
            let a2041 = v0;
            int_neg(a2041)?
        }, List::<u64>::nil())?))
    } else {
        f666(20u64, v0, List::<u64>::nil())
    }
}

pub fn f594(v0: u64) -> R<List<u64>> {
    let a2042 = v0;
    let a2043 = 34u64;
    if nat_eq(a2042, a2043) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2044 = v0;
        let a2045 = 92u64;
        if nat_eq(a2044, a2045) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2046 = v0;
            let a2047 = 8u64;
            if nat_eq(a2046, a2047) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2048 = v0;
                let a2049 = 9u64;
                if nat_eq(a2048, a2049) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2050 = v0;
                    let a2051 = 10u64;
                    if nat_eq(a2050, a2051) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2052 = v0;
                        let a2053 = 12u64;
                        if nat_eq(a2052, a2053) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2054 = v0;
                            let a2055 = 13u64;
                            if nat_eq(a2054, a2055) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2056 = v0;
                                let a2057 = 32u64;
                                if nat_lt(a2056, a2057) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f667(f644(v0, 16u64))?, List::cons(f667(f643(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f595(v0: List<Str>) -> Adt6 {
    Adt6::C4(f541(v0))
}

pub fn f596(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2075 = f628(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2058, _, _, _, _, _, _) => {
                h2058
            }
        });
        let a2076 = {
            let a2073 = f628(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2059, _, _, _, _, _) => {
                    h2059
                }
            });
            let a2074 = {
                let a2071 = f628(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2060, _, _, _, _) => {
                        h2060
                    }
                });
                let a2072 = {
                    let a2069 = f668(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2061, _, _, _) => {
                            h2061
                        }
                    });
                    let a2070 = {
                        let a2067 = f669(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2062, _, _) => {
                                h2062
                            }
                        });
                        let a2068 = {
                            let a2065 = f668(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2063, _) => {
                                    h2063
                                }
                            });
                            let a2066 = f628(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2064) => {
                                    h2064
                                }
                            });
                            append_list(a2065, a2066)
                        };
                        append_list(a2067, a2068)
                    };
                    append_list(a2069, a2070)
                };
                append_list(a2071, a2072)
            };
            append_list(a2073, a2074)
        };
        append_list(a2075, a2076)
    })
}

pub fn f597(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2077 = v1;
    match m2077 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f598(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f145(match v0.clone() {
        Adt10::C0(h2078, _) => {
            h2078
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f670(match v0 {
        Adt10::C0(_, h2079) => {
            h2079
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f599(v0: List<Adt35>) -> List<Adt6> {
    f470(f671(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f600(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m2080 = f672(v0, List::<Adt6>::nil())?;
    match m2080 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f470(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f601(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m2081 = v1;
    match m2081 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f602(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m2082 = v1;
    match m2082 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f603(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f470(f673(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f604(v0: List<Adt46>) -> List<Adt6> {
    f470(f674(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f605(v0: List<Adt54>) -> List<Adt6> {
    f470(f675(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f606(v0: List<Adt56>) -> List<Adt6> {
    f470(f676(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f607(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m2083 = f677(v0, List::<Adt6>::nil())?;
    match m2083 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f470(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f608(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2084 = v1;
    match m2084 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f145(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f609(p2090: List<(Str, Adt64)>, p2091: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m2089 = (p2090, p2091);
    {
        let mut m2089 = m2089;
        loop {
            let step = {
                let (v0, v1) = m2089;
                let m2085 = v0;
                match m2085.uncons() {
                    None => {
                        Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2087 = f678({
                            let (_, h2086) = v2;
                            h2086
                        })?;
                        match m2087 {
                            None => {
                                Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h2088, _) = v2;
                                    h2088
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2089 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f610(v0: Adt81) -> List<(Str, Adt64)> {
    let m2092 = v0;
    match m2092 {
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

pub fn f611(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2093 = v0.clone();
    match m2093.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f472(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f612(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2094 = v0;
    match m2094 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f613(v0: Adt81) -> Option<Str> {
    let m2095 = v0;
    match m2095 {
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

pub fn f614() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f615(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2096, _, _, _, _, _) => {
            h2096
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2097, _, _, _) => {
            h2097
        }
    })), {
        let a2106 = f597(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2098, _, _, _, _) => {
                h2098
            }
        });
        let a2107 = {
            let a2104 = f597(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2099, _, _) => {
                    h2099
                }
            });
            let a2105 = {
                let a2102 = f597(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2100, _) => {
                        h2100
                    }
                });
                let a2103 = f679(match v0 {
                    Adt1::C0(_, _, _, _, _, h2101) => {
                        h2101
                    }
                });
                append_list(a2102, a2103)
            };
            append_list(a2104, a2105)
        };
        append_list(a2106, a2107)
    })))
}

pub fn f616(v0: List<List<u64>>) -> bool {
    let m2108 = v0;
    match m2108.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f617() -> List<Adt87> {
    List::cons(Adt87::C0(f680(), f544()), List::cons(Adt87::C0(f681(), f682()), List::cons(Adt87::C0(f683(), f684()), List::<Adt87>::nil())))
}

pub fn f618(v0: List<List<u64>>) -> List<u64> {
    f685(f50(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f619(p2123: u64, p2124: Adt148, p2125: List<Adt149>) -> R<Option<Adt6>> {
    let m2122 = (p2123, (p2124, p2125));
    {
        let mut m2122 = m2122;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2122;
                let m2109 = v0;
                if m2109 == 0 {
                    Err::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m2109 - 1;
                    let m2110 = v1;
                    match m2110 {
                        Adt148::C0(v4) => {
                            let m2111 = v4.clone();
                            match m2111 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f686(v7)?;
                                    Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m2112 = v9;
                                    match m2112.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C0(v10), List::cons(Adt149::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m2113 = v12;
                                    match m2113.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f686({
                                                let (h2114, _) = v13;
                                                h2114
                                            })?;
                                            Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C0({
                                                let (_, h2115) = v13;
                                                h2115
                                            }), List::cons(Adt149::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt148::C1(v16) => {
                            let m2116 = v2;
                            match m2116.uncons() {
                                None => {
                                    Err::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m2117 = v17;
                                    match m2117 {
                                        Adt149::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m2118 = v20;
                                            match m2118.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C1(Adt6::C4(f470(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C0(v22), List::cons(Adt149::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt149::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m2119 = v26;
                                            match m2119.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C1(Adt6::C5(f472(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f686({
                                                        let (h2120, _) = v28;
                                                        h2120
                                                    })?;
                                                    Ok::<(u64, (Adt148, List<Adt149>)), R<Option<Adt6>>>((v3, (Adt148::C0({
                                                        let (_, h2121) = v28;
                                                        h2121
                                                    }), List::cons(Adt149::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m2122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620() -> u64 {
    67108864u64
}

pub fn f621(v0: Adt19) -> Adt6 {
    let m2126 = v0;
    match m2126 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f622(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2127 = v0;
    match m2127 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f623(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2128 = v0;
    match m2128 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f624() -> i64 {
    -32603i64
}

pub fn f625() -> i64 {
    429i64
}

pub fn f626() -> i64 {
    -32021i64
}

pub fn f627(p2131: List<Str>, p2132: List<Adt6>) -> List<Adt6> {
    let m2130 = (p2131, p2132);
    {
        let mut m2130 = m2130;
        loop {
            let step = {
                let (v0, v1) = m2130;
                let m2129 = v0;
                match m2129.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f628(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2133 = v1;
    match m2133 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2134) => {
                    h2134
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f629(v0: List<u64>) -> R<bool> {
    let m2135 = v0;
    match m2135.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2136 = v2;
            match m2136.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2137 = f353(v1)?;
                    let a2138 = 113u64;
                    if if nat_eq(a2137, a2138) {
                        let a2139 = v3;
                        let a2140 = 61u64;
                        nat_eq(a2139, a2140)
                    } else {
                        false
                    } {
                        Ok(f687(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f630(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt133>) -> R<Result<Adt129, Adt132>> {
    let m2141 = v3;
    match m2141 {
        Ok(v5) => {
            f688(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt129, Adt132>(Adt132::C0(v4)))
        }
    }
}

pub fn f631(v0: Adt139, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt133> {
    let m2142 = v0;
    match m2142 {
        Adt139::C0 => {
            f689(v1, v2)
        }
        Adt139::C1 => {
            f690(v1, v2)
        }
        Adt139::C2 => {
            f691(v1, v2)
        }
        Adt139::C3 => {
            f692(v1, v2)
        }
        Adt139::C4 => {
            f693(v1, v2)
        }
        Adt139::C5 => {
            f694(v1, v2)
        }
        Adt139::C6 => {
            f695(v1, v2)
        }
        Adt139::C7 => {
            f696(v1, v2)
        }
        Adt139::C8 => {
            f697(v1, v2)
        }
        Adt139::C9 => {
            f698(v1, v2)
        }
        Adt139::C10 => {
            f699(v1, v2)
        }
        Adt139::C11 => {
            f700(v1, v2)
        }
    }
}

pub fn f632(v0: Adt139) -> Str {
    let m2143 = v0;
    match m2143 {
        Adt139::C0 => {
            Str::lit("server/discover")
        }
        Adt139::C1 => {
            Str::lit("tools/list")
        }
        Adt139::C2 => {
            Str::lit("tools/call")
        }
        Adt139::C3 => {
            Str::lit("resources/list")
        }
        Adt139::C4 => {
            Str::lit("resources/read")
        }
        Adt139::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt139::C6 => {
            Str::lit("prompts/list")
        }
        Adt139::C7 => {
            Str::lit("prompts/get")
        }
        Adt139::C8 => {
            Str::lit("completion/complete")
        }
        Adt139::C9 => {
            Str::lit("tasks/get")
        }
        Adt139::C10 => {
            Str::lit("tasks/update")
        }
        Adt139::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f633(v0: Option<Adt8>) -> bool {
    let m2144 = v0;
    match m2144 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f634(v0: Option<Adt9>) -> bool {
    let m2145 = v0;
    match m2145 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f635(v0: Option<Adt5>, v1: Str) -> bool {
    let m2146 = v0;
    match m2146 {
        None => {
            false
        }
        Some(v2) => {
            f411(v1, v2)
        }
    }
}

pub fn f636() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f637(v0: List<Str>, v1: Adt108, v2: Adt16, v3: Option<Adt113>) -> Adt143 {
    let m2147 = v3;
    match m2147 {
        None => {
            f701(v1, v2.clone(), f702(match v2 {
                Adt16::C0(h2148, _, _, _, _, _) => {
                    h2148
                }
            }, v0))
        }
        Some(v4) => {
            Adt143::C1(Adt132::C2(v4))
        }
    }
}

pub fn f638(v0: Option<Adt112>, v1: Adt145, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt113>> {
    let m2149 = v0;
    match m2149 {
        None => {
            Ok(None::<Adt113>)
        }
        Some(v5) => {
            let m2151 = match v5.clone() {
                Adt112::C0(_, _, _, h2150, _) => {
                    h2150
                }
            };
            match m2151 {
                None => {
                    f703(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f639(v0: Option<Adt5>) -> Adt5 {
    let m2152 = v0;
    match m2152 {
        None => {
            f26()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f640(v0: Adt5) -> Result<Adt16, Adt133> {
    let m2153 = f704(v0.clone());
    match m2153 {
        Adt152::C0 => {
            f705(v0.clone(), f186(f706(), v0.clone()), f186(f707(), v0))
        }
        Adt152::C1 => {
            Err::<Adt16, Adt133>(Adt133::C2)
        }
        Adt152::C2 => {
            Err::<Adt16, Adt133>(Adt133::C3)
        }
    }
}

pub fn f641() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f642(v0: Str) -> R<List<u64>> {
    f206({
        let a2154 = v0;
        utf8_encode(a2154)
    })
}

pub fn f643(v0: u64, v1: u64) -> u64 {
    let a2155 = v0;
    let a2156 = v1;
    let a2157 = 0u64;
    nat_rem(a2155, a2156, a2157)
}

pub fn f644(v0: u64, v1: u64) -> u64 {
    let a2158 = v0;
    let a2159 = v1;
    let a2160 = 0u64;
    nat_quot(a2158, a2159, a2160)
}

pub fn f645(v0: u64, v1: Adt6) -> bool {
    let a2161 = v0;
    let a2162 = 0u64;
    if nat_eq(a2161, a2162) {
        f708(v1)
    } else {
        let a2163 = v0;
        let a2164 = 1u64;
        if nat_eq(a2163, a2164) {
            f709(v1)
        } else {
            let a2165 = v0;
            let a2166 = 2u64;
            if nat_eq(a2165, a2166) {
                f710(v1)
            } else {
                let a2167 = v0;
                let a2168 = 3u64;
                if nat_eq(a2167, a2168) {
                    f711(v1)
                } else {
                    let a2169 = v0;
                    let a2170 = 4u64;
                    if nat_eq(a2169, a2170) {
                        f712(v1)
                    } else {
                        let a2171 = v0;
                        let a2172 = 5u64;
                        if nat_eq(a2171, a2172) {
                            f713(v1)
                        } else {
                            let a2173 = v0;
                            let a2174 = 6u64;
                            if nat_eq(a2173, a2174) {
                                f714(v1)
                            } else {
                                let a2175 = v0;
                                let a2176 = 7u64;
                                if nat_eq(a2175, a2176) {
                                    f715(v1)
                                } else {
                                    f716(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f646(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt125) -> R<Adt146> {
    let m2177 = v3.clone();
    match m2177.uncons() {
        None => {
            Ok(Adt146::C0(Adt92::C8(v0)))
        }
        Some((_, _)) => {
            if f357(v3.clone(), true) {
                let m2178 = f717(v1);
                match m2178 {
                    None => {
                        Ok(Adt146::C0(Adt92::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f718(v3)?;
                        if f719(v9.clone(), match v5.clone() {
                            Adt125::C0(_, h2179) => {
                                h2179
                            }
                        }) {
                            Ok(Adt146::C0(Adt92::C12(v0, v2)))
                        } else {
                            Ok(Adt146::C1(Adt125::C0(List::cons(Adt12::C0(v0, v2, f251(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt125::C0(h2180, _) => {
                                    h2180
                                }
                            }), List::cons(v9, match v5 {
                                Adt125::C0(_, h2181) => {
                                    h2181
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt146::C0(Adt92::C9(v0)))
            }
        }
    }
}

pub fn f647(p2186: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2187: List<Adt109>) -> List<Adt109> {
    let m2185 = (p2186, p2187);
    {
        let mut m2185 = m2185;
        loop {
            let step = {
                let (v0, v3) = m2185;
                let m2182 = v0;
                match m2182.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0({
                            let (_, h2183) = v4;
                            h2183
                        }, v1, List::cons({
                            let (h2184, _) = v4;
                            h2184
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(p2191: List<(Str, Adt6)>, p2192: List<Adt109>) -> List<Adt109> {
    let m2190 = (p2191, p2192);
    {
        let mut m2190 = m2190;
        loop {
            let step = {
                let (v0, v1) = m2190;
                let m2188 = v0;
                match m2188.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v3, List::cons(Adt109::C0({
                            let (_, h2189) = v2;
                            h2189
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f649(p2195: List<Adt6>, p2196: List<Adt109>) -> List<Adt109> {
    let m2194 = (p2195, p2196);
    {
        let mut m2194 = m2194;
        loop {
            let step = {
                let (v0, v1) = m2194;
                let m2193 = v0;
                match m2193.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt109>), List<Adt109>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt109>), List<Adt109>>((v3, List::cons(Adt109::C0(v2, false, List::<Str>::nil()), v1)))
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

pub fn f650(v0: Adt6) -> Str {
    let m2197 = v0;
    match m2197 {
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

pub fn f651(v0: Adt7) -> R<bool> {
    if f556(match v0.clone() {
        Adt7::C0(h2198, _) => {
            h2198
        }
    }) {
        Ok(true)
    } else {
        f720(40u64, match v0.clone() {
            Adt7::C0(h2199, _) => {
                h2199
            }
        }, match v0 {
            Adt7::C0(_, h2200) => {
                h2200
            }
        })
    }
}

pub fn f652(v0: Adt6) -> Str {
    let m2201 = f266(v0);
    match m2201 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f653(p2208: u64, p2209: List<(Adt6, Adt6)>) -> R<Adt150> {
    let m2207 = (p2208, p2209);
    {
        let mut m2207 = m2207;
        loop {
            let step = {
                let (v0, v1) = m2207;
                let m2202 = v0;
                if m2202 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt150>>(Ok(Adt150::C2))
                } else {
                    let v2 = m2202 - 1;
                    let m2203 = v1;
                    match m2203.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt150>>(Ok(Adt150::C0))
                        }
                        Some((v3, v4)) => {
                            let m2206 = f721({
                                let (h2204, _) = v3;
                                h2204
                            }, {
                                let (_, h2205) = v3;
                                h2205
                            }, v4)?;
                            match m2206 {
                                Adt153::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt150>>(Ok(Adt150::C1))
                                }
                                Adt153::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt150>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f654() -> u64 {
    67108864u64
}

pub fn f655(v0: Adt151) -> bool {
    let m2210 = v0;
    match m2210 {
        Adt151::C0 => {
            true
        }
        Adt151::C1 => {
            false
        }
        Adt151::C2 => {
            false
        }
    }
}

pub fn f656(v0: Adt7, v1: Adt7, v2: Adt151, v3: Adt151) -> R<Adt151> {
    let m2211 = v2;
    match m2211 {
        Adt151::C0 => {
            let m2213 = v3;
            match m2213 {
                Adt151::C0 => {
                    Ok(f722(Adt151::C0, f723(f724(match v0.clone() {
                        Adt7::C0(h2214, _) => {
                            h2214
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2215) => {
                            h2215
                        }
                    }, f724(match v1.clone() {
                        Adt7::C0(h2216, _) => {
                            h2216
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2217) => {
                            h2217
                        }
                    })?))
                }
                Adt151::C1 => {
                    Ok(Adt151::C0)
                }
                Adt151::C2 => {
                    Ok(Adt151::C0)
                }
            }
        }
        Adt151::C1 => {
            let m2212 = v3;
            match m2212 {
                Adt151::C0 => {
                    Ok(Adt151::C2)
                }
                Adt151::C1 => {
                    Ok(Adt151::C1)
                }
                Adt151::C2 => {
                    Ok(Adt151::C0)
                }
            }
        }
        Adt151::C2 => {
            let m2218 = v3;
            match m2218 {
                Adt151::C0 => {
                    Ok(Adt151::C2)
                }
                Adt151::C1 => {
                    Ok(Adt151::C2)
                }
                Adt151::C2 => {
                    f723(match v0.clone() {
                        Adt7::C0(h2219, _) => {
                            h2219
                        }
                    }, match v0 {
                        Adt7::C0(_, h2220) => {
                            h2220
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2221, _) => {
                            h2221
                        }
                    }, match v1 {
                        Adt7::C0(_, h2222) => {
                            h2222
                        }
                    })
                }
            }
        }
    }
}

pub fn f657(v0: i64) -> Adt151 {
    f659(v0, 0i64)
}

pub fn f658(v0: Adt6) -> Option<i64> {
    let m2223 = v0;
    match m2223 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f556(match v2.clone() {
                Adt7::C0(_, h2224) => {
                    h2224
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2225, _) => {
                        h2225
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

pub fn f659(v0: i64, v1: i64) -> Adt151 {
    let m2228 = {
        let a2226 = v0;
        let a2227 = v1;
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
        Adt151::C0
    } else {
        let m2231 = {
            let a2229 = v1;
            let a2230 = v0;
            compare(a2229, a2230)
        };
        if match m2231 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt151::C2
        } else {
            Adt151::C1
        }
    }
}

pub fn f660(p2240: List<u64>, p2241: u64) -> R<u64> {
    let m2239 = (p2240, p2241);
    {
        let mut m2239 = m2239;
        loop {
            let step = {
                let (v0, v1) = m2239;
                let m2232 = v0;
                match m2232.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2233 = v2;
                        let a2234 = 128u64;
                        if if nat_lt(a2233, a2234) {
                            true
                        } else {
                            let a2235 = 191u64;
                            let a2236 = v2;
                            nat_lt(a2235, a2236)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2237 = v1;
                                let a2238 = 1u64;
                                nat_add(a2237, a2238)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f661(v0: Str, p2259: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2258 = p2259;
    {
        let mut m2258 = m2258;
        loop {
            let step = {
                let v1 = m2258;
                let m2242 = v1;
                match m2242.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2246 = v0.clone();
                        let a2247 = {
                            let a2244 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2243, _) = v2.clone();
                                h2243
                            }, List::<Str>::nil()));
                            let a2245 = Str::lit("");
                            join(a2244, a2245)
                        };
                        if equal(a2246, a2247) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2248) = v2;
                                h2248
                            }))
                        } else {
                            let m2250 = f187({
                                let (_, h2249) = v2.clone();
                                h2249
                            });
                            match m2250 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2251 = f186(Str::lit("$anchor"), v4);
                                    match m2251 {
                                        Adt107::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt107::C1(v5) => {
                                            let m2252 = f266(v5);
                                            match m2252 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2255 = v0.clone();
                                                    let a2256 = {
                                                        let a2253 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2254 = Str::lit("");
                                                        join(a2253, a2254)
                                                    };
                                                    if equal(a2255, a2256) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2257) = v2;
                                                            h2257
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
                Ok(next) => { m2258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f662(v0: u64) -> R<Option<u64>> {
    let a2260 = 64u64;
    let a2261 = v0;
    if if nat_lt(a2260, a2261) {
        let a2262 = v0;
        let a2263 = 91u64;
        nat_lt(a2262, a2263)
    } else {
        false
    } {
        Ok(Some({
            let a2264 = v0;
            let a2265 = 65u64;
            nat_sub(a2264, a2265)
        }))
    } else {
        let a2266 = 96u64;
        let a2267 = v0;
        if if nat_lt(a2266, a2267) {
            let a2268 = v0;
            let a2269 = 123u64;
            nat_lt(a2268, a2269)
        } else {
            false
        } {
            Ok(Some({
                let a2270 = v0;
                let a2271 = 71u64;
                nat_sub(a2270, a2271)
            }))
        } else {
            let a2272 = 47u64;
            let a2273 = v0;
            if if nat_lt(a2272, a2273) {
                let a2274 = v0;
                let a2275 = 58u64;
                nat_lt(a2274, a2275)
            } else {
                false
            } {
                Ok(Some({
                    let a2276 = v0;
                    let a2277 = 4u64;
                    nat_add(a2276, a2277)?
                }))
            } else {
                let a2278 = v0;
                let a2279 = 43u64;
                if nat_eq(a2278, a2279) {
                    Ok(Some(62u64))
                } else {
                    let a2280 = v0;
                    let a2281 = 47u64;
                    if nat_eq(a2280, a2281) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f663(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2282 = v3;
        let a2283 = 61u64;
        if nat_eq(a2282, a2283) {
            let a2284 = f725(v1, 16u64);
            let a2285 = 0u64;
            nat_eq(a2284, a2285)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f726(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f664(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2286 = f725(v2, 4u64);
        let a2287 = 0u64;
        nat_eq(a2286, a2287)
    } else {
        false
    } {
        Ok(Some(f727(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f665(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2290 = {
            let a2288 = v0;
            let a2289 = 4u64;
            nat_mul(a2288, a2289)?
        };
        let a2291 = f728(v1, 16u64);
        nat_add(a2290, a2291)?
    }, List::cons({
        let a2294 = {
            let a2292 = f725(v1, 16u64);
            let a2293 = 16u64;
            nat_mul(a2292, a2293)?
        };
        let a2295 = f728(v2, 4u64);
        nat_add(a2294, a2295)?
    }, List::cons({
        let a2298 = {
            let a2296 = f725(v2, 4u64);
            let a2297 = 64u64;
            nat_mul(a2296, a2297)?
        };
        let a2299 = v3;
        nat_add(a2298, a2299)?
    }, List::<u64>::nil()))))
}

pub fn f666(p2311: u64, p2312: i64, p2313: List<u64>) -> R<List<u64>> {
    let m2310 = (p2311, (p2312, p2313));
    {
        let mut m2310 = m2310;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2310;
                let m2300 = v0;
                if m2300 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2300 - 1;
                    let v4: i64 = {
                        let a2301 = v1;
                        let a2302 = 10i64;
                        let a2303 = 0i64;
                        int_quot(a2301, a2302, a2303)?
                    };
                    let v5: i64 = {
                        let a2304 = v1;
                        let a2305 = 10i64;
                        let a2306 = 0i64;
                        int_rem(a2304, a2305, a2306)
                    };
                    let m2309 = {
                        let a2307 = v4;
                        let a2308 = 1i64;
                        compare(a2307, a2308)
                    };
                    if match m2309 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f729(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f729(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f667(v0: u64) -> R<u64> {
    let a2314 = v0;
    let a2315 = 10u64;
    if nat_lt(a2314, a2315) {
        let a2316 = 48u64;
        let a2317 = v0;
        nat_add(a2316, a2317)
    } else {
        let a2318 = 87u64;
        let a2319 = v0;
        nat_add(a2318, a2319)
    }
}

pub fn f668(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2320 = v1;
    match m2320 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f730(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f669(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2321 = v0;
    match m2321 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2324 = f602(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2322, _) => {
                        h2322
                    }
                });
                let a2325 = f602(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2323) => {
                        h2323
                    }
                });
                append_list(a2324, a2325)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f670(v0: Adt11) -> Str {
    let m2326 = v0;
    match m2326 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f671(p2329: List<Adt35>, p2330: List<Adt6>) -> List<Adt6> {
    let m2328 = (p2329, p2330);
    {
        let mut m2328 = m2328;
        loop {
            let step = {
                let (v0, v1) = m2328;
                let m2327 = v0;
                match m2327.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f731(v2), v1)))
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

pub fn f672(p2334: List<Adt38>, p2335: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2333 = (p2334, p2335);
    {
        let mut m2333 = m2333;
        loop {
            let step = {
                let (v0, v1) = m2333;
                let m2331 = v0;
                match m2331.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2332 = f732(v2)?;
                        match m2332 {
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
                Ok(next) => { m2333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f673(p2338: List<Adt44>, p2339: List<Adt6>) -> R<List<Adt6>> {
    let m2337 = (p2338, p2339);
    {
        let mut m2337 = m2337;
        loop {
            let step = {
                let (v0, v1) = m2337;
                let m2336 = v0;
                match m2336.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f733(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f674(p2342: List<Adt46>, p2343: List<Adt6>) -> List<Adt6> {
    let m2341 = (p2342, p2343);
    {
        let mut m2341 = m2341;
        loop {
            let step = {
                let (v0, v1) = m2341;
                let m2340 = v0;
                match m2340.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f734(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2341 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(p2346: List<Adt54>, p2347: List<Adt6>) -> List<Adt6> {
    let m2345 = (p2346, p2347);
    {
        let mut m2345 = m2345;
        loop {
            let step = {
                let (v0, v1) = m2345;
                let m2344 = v0;
                match m2344.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f735(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f676(p2350: List<Adt56>, p2351: List<Adt6>) -> List<Adt6> {
    let m2349 = (p2350, p2351);
    {
        let mut m2349 = m2349;
        loop {
            let step = {
                let (v0, v1) = m2349;
                let m2348 = v0;
                match m2348.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f736(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677(p2355: List<Adt59>, p2356: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2354 = (p2355, p2356);
    {
        let mut m2354 = m2354;
        loop {
            let step = {
                let (v0, v1) = m2354;
                let m2352 = v0;
                match m2352.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2353 = f737(v2)?;
                        match m2353 {
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
                Ok(next) => { m2354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f678(v0: Adt64) -> R<Option<Adt6>> {
    let m2357 = v0;
    match m2357 {
        Adt64::C0(v1) => {
            Ok(Some(f738(Str::lit("elicitation/create"), Some(f739(v1)))))
        }
        Adt64::C1(v2) => {
            let m2358 = f740(v2)?;
            match m2358 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f738(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt64::C2(v4) => {
            Ok(Some(f738(Str::lit("roots/list"), {
                let m2359 = v4;
                match m2359 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h2360) => {
                                h2360
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f679(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2361 = v0;
    match m2361 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f741(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f680() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f681() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f682() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f683() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f684() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f685(p2366: List<List<u64>>, p2367: List<u64>) -> List<u64> {
    let m2365 = (p2366, p2367);
    {
        let mut m2365 = m2365;
        loop {
            let step = {
                let (v0, v1) = m2365;
                let m2362 = v0;
                match m2362.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2363 = f742(v2);
                            let a2364 = v1;
                            append_list(a2363, a2364)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Str) -> R<Str> {
    let m2370 = {
        let a2369 = f319(f320(), f333(f743(f206({
            let a2368 = v0;
            utf8_encode(a2368)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2369)
    };
    match m2370 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f687(v0: List<u64>) -> bool {
    let m2371 = v0;
    match m2371.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2372 = v1;
            let a2373 = 48u64;
            if nat_eq(a2372, a2373) {
                f744(v2)
            } else {
                false
            }
        }
    }
}

pub fn f688(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt129, Adt132>> {
    if f745(f746(v3.clone())) {
        Ok(Ok::<Adt129, Adt132>(f747(v0.clone(), Adt32::C0(f748(v0)))))
    } else {
        Ok(f749(v0.clone(), v3.clone(), f750(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2374) => {
                h2374
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2375, _) => {
                h2375
            }
        }, v2, v1, v3)?))
    }
}

pub fn f689(v0: Adt16, _: Adt5) -> Result<Adt14, Adt133> {
    Ok::<Adt14, Adt133>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f690(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2376 = f751(Str::lit("cursor"), v1);
    match m2376 {
        Ok(v2) => {
            Ok::<Adt14, Adt133>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt133>(v3)
        }
    }
}

pub fn f691(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2377 = f752(Str::lit("name"), v1.clone());
    match m2377 {
        Ok(v2) => {
            let m2378 = f753(Str::lit("arguments"), v1.clone());
            match m2378 {
                Ok(v3) => {
                    let m2379 = f753(Str::lit("inputResponses"), v1.clone());
                    match m2379 {
                        Ok(v4) => {
                            let m2380 = f751(Str::lit("requestState"), v1);
                            match m2380 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt133>(Adt14::C2(Adt21::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt133>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt133>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt133>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt133>(v9)
        }
    }
}

pub fn f692(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2381 = f751(Str::lit("cursor"), v1);
    match m2381 {
        Ok(v2) => {
            Ok::<Adt14, Adt133>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt133>(v3)
        }
    }
}

pub fn f693(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2382 = f752(Str::lit("uri"), v1.clone());
    match m2382 {
        Ok(v2) => {
            let m2383 = f753(Str::lit("inputResponses"), v1.clone());
            match m2383 {
                Ok(v3) => {
                    let m2384 = f751(Str::lit("requestState"), v1);
                    match m2384 {
                        Ok(v4) => {
                            Ok::<Adt14, Adt133>(Adt14::C4(Adt22::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt14, Adt133>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt14, Adt133>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt14, Adt133>(v7)
        }
    }
}

pub fn f694(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2385 = f751(Str::lit("cursor"), v1);
    match m2385 {
        Ok(v2) => {
            Ok::<Adt14, Adt133>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt133>(v3)
        }
    }
}

pub fn f695(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2386 = f751(Str::lit("cursor"), v1);
    match m2386 {
        Ok(v2) => {
            Ok::<Adt14, Adt133>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt133>(v3)
        }
    }
}

pub fn f696(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2387 = f752(Str::lit("name"), v1.clone());
    match m2387 {
        Ok(v2) => {
            let m2388 = f754(Str::lit("arguments"), v1.clone());
            match m2388 {
                Ok(v3) => {
                    let m2389 = f753(Str::lit("inputResponses"), v1.clone());
                    match m2389 {
                        Ok(v4) => {
                            let m2390 = f751(Str::lit("requestState"), v1);
                            match m2390 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt133>(Adt14::C7(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt133>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt133>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt133>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt133>(v9)
        }
    }
}

pub fn f697(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2391 = f755(Str::lit("ref"), v1.clone());
    match m2391 {
        Ok(v2) => {
            let m2392 = f756(v2);
            match m2392 {
                Ok(v3) => {
                    let m2393 = f755(Str::lit("argument"), v1.clone());
                    match m2393 {
                        Ok(v4) => {
                            let m2394 = f757(v4);
                            match m2394 {
                                Ok(v5) => {
                                    let m2395 = f758(v1);
                                    match m2395 {
                                        Ok(v6) => {
                                            Ok::<Adt14, Adt133>(Adt14::C8(Adt24::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt14, Adt133>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt14, Adt133>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt14, Adt133>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt14, Adt133>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt14, Adt133>(v11)
        }
    }
}

pub fn f698(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2396 = f752(Str::lit("taskId"), v1);
    match m2396 {
        Ok(v2) => {
            Ok::<Adt14, Adt133>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt133>(v3)
        }
    }
}

pub fn f699(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2397 = f752(Str::lit("taskId"), v1.clone());
    match m2397 {
        Ok(v2) => {
            let m2398 = f755(Str::lit("inputResponses"), v1);
            match m2398 {
                Ok(v3) => {
                    Ok::<Adt14, Adt133>(Adt14::C10(Adt30::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt14, Adt133>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt14, Adt133>(v5)
        }
    }
}

pub fn f700(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt133> {
    let m2399 = f752(Str::lit("taskId"), v1);
    match m2399 {
        Ok(v2) => {
            Ok::<Adt14, Adt133>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt133>(v3)
        }
    }
}

pub fn f701(v0: Adt108, v1: Adt16, v2: Option<Adt132>) -> Adt143 {
    let m2400 = v2;
    match m2400 {
        None => {
            Adt143::C0(Adt144::C0(match v0.clone() {
                Adt108::C0(h2401, _, _) => {
                    h2401
                }
            }, match v0.clone() {
                Adt108::C0(_, h2402, _) => {
                    h2402
                }
            }, v1, f639(match v0 {
                Adt108::C0(_, _, h2403) => {
                    h2403
                }
            })))
        }
        Some(v3) => {
            Adt143::C1(v3)
        }
    }
}

pub fn f702(v0: Str, v1: List<Str>) -> Option<Adt132> {
    f759(f760(v0.clone(), v1.clone()), v0, v1)
}

pub fn f703(v0: Adt112, v1: Adt145, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt113>> {
    let m2405 = match v0.clone() {
        Adt112::C0(h2404, _, _, _, _) => {
            h2404
        }
    };
    match m2405 {
        None => {
            Ok(Some(Adt113::C0))
        }
        Some(v5) => {
            let a2406 = v5;
            let a2407 = v3;
            if equal(a2406, a2407) {
                let m2409 = match v0.clone() {
                    Adt112::C0(_, h2408, _, _, _) => {
                        h2408
                    }
                };
                match m2409 {
                    None => {
                        Ok(Some(Adt113::C1))
                    }
                    Some(v6) => {
                        let a2410 = v6;
                        let a2411 = v2;
                        if equal(a2410, a2411) {
                            f761(f762(match v0.clone() {
                                Adt112::C0(_, _, h2412, _, _) => {
                                    h2412
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt112::C0(_, _, _, _, h2413) => {
                                    h2413
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

pub fn f704(v0: Adt5) -> Adt152 {
    f763(f186(f706(), v0.clone()), f186(f707(), v0))
}

pub fn f705(v0: Adt5, v1: Adt107, v2: Adt107) -> Result<Adt16, Adt133> {
    let m2414 = v1;
    match m2414 {
        Adt107::C0 => {
            Err::<Adt16, Adt133>(Adt133::C2)
        }
        Adt107::C1(v3) => {
            let m2415 = v2;
            match m2415 {
                Adt107::C0 => {
                    Err::<Adt16, Adt133>(Adt133::C3)
                }
                Adt107::C1(v4) => {
                    f764(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f706() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f707() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f708(v0: Adt6) -> bool {
    let m2416 = f266(v0);
    match m2416 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f709(v0: Adt6) -> bool {
    let m2417 = f574(v0);
    match m2417 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f710(v0: Adt6) -> bool {
    let m2418 = f658(v0);
    match m2418 {
        None => {
            false
        }
        Some(v1) => {
            let a2422 = {
                let m2421 = {
                    let a2419 = v1;
                    let a2420 = 0i64;
                    compare(a2419, a2420)
                };
                match m2421 {
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
            bool_not(a2422)
        }
    }
}

pub fn f711(v0: Adt6) -> bool {
    let m2423 = f242(v0);
    match m2423 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f712(v0: Adt6) -> bool {
    let m2424 = f187(v0);
    match m2424 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f713(v0: Adt6) -> bool {
    let m2425 = f242(v0);
    match m2425 {
        None => {
            false
        }
        Some(v1) => {
            f765(v1)
        }
    }
}

pub fn f714(v0: Adt6) -> bool {
    let m2426 = f317(v0);
    match m2426 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f715(v0: Adt6) -> bool {
    let m2427 = f266(v0.clone());
    match m2427 {
        None => {
            let m2428 = f242(v0);
            match m2428 {
                None => {
                    false
                }
                Some(v2) => {
                    f766(v2)
                }
            }
        }
        Some(v1) => {
            f569(v1)
        }
    }
}

pub fn f716(v0: Adt6) -> bool {
    let m2429 = f242(v0);
    match m2429 {
        None => {
            false
        }
        Some(v1) => {
            let m2430 = v1;
            match m2430.uncons() {
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

pub fn f717(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2431 = f262(Str::lit("type"), v0);
    match m2431 {
        Adt107::C0 => {
            None::<Adt13>
        }
        Adt107::C1(v1) => {
            let m2432 = f266(v1);
            match m2432 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f767(v2)
                }
            }
        }
    }
}

pub fn f718(v0: List<u64>) -> R<List<u64>> {
    Ok(f333(f768(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f719(v0: List<u64>, p2435: List<List<u64>>) -> bool {
    let m2434 = p2435;
    {
        let mut m2434 = m2434;
        loop {
            let step = {
                let v1 = m2434;
                let m2433 = v1;
                match m2433.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f769(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f720(p2447: u64, p2448: i64, p2449: i64) -> R<bool> {
    let m2446 = (p2447, (p2448, p2449));
    {
        let mut m2446 = m2446;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2446;
                let m2436 = v0;
                if m2436 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2436 - 1;
                    let m2439 = {
                        let a2437 = v2;
                        let a2438 = 0i64;
                        compare(a2437, a2438)
                    };
                    if match m2439 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f556({
                            let a2440 = v1;
                            let a2441 = 10i64;
                            let a2442 = 0i64;
                            int_rem(a2440, a2441, a2442)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2443 = v1;
                                let a2444 = 10i64;
                                let a2445 = 0i64;
                                int_quot(a2443, a2444, a2445)?
                            }, f316(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
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

pub fn f721(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt153> {
    let a2450 = f770(v0.clone());
    let a2451 = f770(v1.clone());
    if nat_eq(a2450, a2451) {
        let m2452 = v0;
        match m2452 {
            Adt6::C0 => {
                Ok(Adt153::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2453 = f317(v1);
                match m2453 {
                    None => {
                        Ok(Adt153::C0)
                    }
                    Some(v4) => {
                        let a2454 = v3;
                        let a2455 = v4;
                        if equal(a2454, a2455) {
                            Ok(Adt153::C1(v2))
                        } else {
                            Ok(Adt153::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2456 = f574(v1);
                match m2456 {
                    None => {
                        Ok(Adt153::C0)
                    }
                    Some(v6) => {
                        if f771(f576(v5, v6)?) {
                            Ok(Adt153::C1(v2))
                        } else {
                            Ok(Adt153::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2457 = f266(v1);
                match m2457 {
                    None => {
                        Ok(Adt153::C0)
                    }
                    Some(v8) => {
                        let a2458 = v7;
                        let a2459 = v8;
                        if equal(a2458, a2459) {
                            Ok(Adt153::C1(v2))
                        } else {
                            Ok(Adt153::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2460 = f242(v1);
                match m2460 {
                    None => {
                        Ok(Adt153::C0)
                    }
                    Some(v10) => {
                        Ok(f772(f773(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2461 = f187(v1);
                match m2461 {
                    None => {
                        Ok(Adt153::C0)
                    }
                    Some(v12) => {
                        let a2463 = f580(v11.clone(), 0u64)?;
                        let a2464 = f580(match v12.clone() {
                            Adt5::C0(h2462) => {
                                h2462
                            }
                        }, 0u64)?;
                        if nat_eq(a2463, a2464) {
                            Ok(f772(f774(v11, match v12 {
                                Adt5::C0(h2465) => {
                                    h2465
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt153::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt153::C0)
    }
}

pub fn f722(v0: Adt151, v1: Adt151) -> Adt151 {
    let m2466 = v0;
    match m2466 {
        Adt151::C0 => {
            f775(v1)
        }
        Adt151::C1 => {
            Adt151::C1
        }
        Adt151::C2 => {
            v1
        }
    }
}

pub fn f723(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt151> {
    let m2467 = f659(f776(v0, v1)?, f776(v2, v3)?);
    match m2467 {
        Adt151::C0 => {
            Ok(Adt151::C0)
        }
        Adt151::C1 => {
            f777(v0, f778(v0)?, v2, f778(v2)?)
        }
        Adt151::C2 => {
            Ok(Adt151::C2)
        }
    }
}

pub fn f724(v0: i64) -> R<i64> {
    let m2470 = {
        let a2468 = v0;
        let a2469 = 0i64;
        compare(a2468, a2469)
    };
    if match m2470 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2471 = v0;
        int_neg(a2471)
    } else {
        Ok(v0)
    }
}

pub fn f725(v0: u64, v1: u64) -> u64 {
    let a2472 = v0;
    let a2473 = v1;
    let a2474 = 0u64;
    nat_rem(a2472, a2473, a2474)
}

pub fn f726(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2477 = {
            let a2475 = v0;
            let a2476 = 4u64;
            nat_mul(a2475, a2476)?
        };
        let a2478 = f728(v1, 16u64);
        nat_add(a2477, a2478)?
    }, List::<u64>::nil()))
}

pub fn f727(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2481 = {
            let a2479 = v0;
            let a2480 = 4u64;
            nat_mul(a2479, a2480)?
        };
        let a2482 = f728(v1, 16u64);
        nat_add(a2481, a2482)?
    }, List::cons({
        let a2485 = {
            let a2483 = f725(v1, 16u64);
            let a2484 = 16u64;
            nat_mul(a2483, a2484)?
        };
        let a2486 = f728(v2, 4u64);
        nat_add(a2485, a2486)?
    }, List::<u64>::nil())))
}

pub fn f728(v0: u64, v1: u64) -> u64 {
    let a2487 = v0;
    let a2488 = v1;
    let a2489 = 0u64;
    nat_quot(a2487, a2488, a2489)
}

pub fn f729(v0: i64) -> u64 {
    let m2492 = {
        let a2490 = v0;
        let a2491 = 1i64;
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
        48u64
    } else {
        let m2495 = {
            let a2493 = v0;
            let a2494 = 2i64;
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
            49u64
        } else {
            let m2498 = {
                let a2496 = v0;
                let a2497 = 3i64;
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
                50u64
            } else {
                let m2501 = {
                    let a2499 = v0;
                    let a2500 = 4i64;
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
                    51u64
                } else {
                    let m2504 = {
                        let a2502 = v0;
                        let a2503 = 5i64;
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
                        52u64
                    } else {
                        let m2507 = {
                            let a2505 = v0;
                            let a2506 = 6i64;
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
                            53u64
                        } else {
                            let m2510 = {
                                let a2508 = v0;
                                let a2509 = 7i64;
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
                                54u64
                            } else {
                                let m2513 = {
                                    let a2511 = v0;
                                    let a2512 = 8i64;
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
                                    55u64
                                } else {
                                    let m2516 = {
                                        let a2514 = v0;
                                        let a2515 = 9i64;
                                        compare(a2514, a2515)
                                    };
                                    if match m2516 {
                                        Ordering::Less => {
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

pub fn f730(v0: Adt8) -> Adt6 {
    Adt6::C5(f602(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2517) => {
            h2517
        }
    }))
}

pub fn f731(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2518, _, _, _, _, _, _, _) => {
            h2518
        }
    })), {
        let a2536 = f597(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2519, _, _, _, _, _, _) => {
                h2519
            }
        });
        let a2537 = {
            let a2534 = f679(match v0.clone() {
                Adt35::C0(_, _, h2520, _, _, _, _, _) => {
                    h2520
                }
            });
            let a2535 = {
                let a2532 = f597(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2521, _, _, _, _) => {
                        h2521
                    }
                });
                let a2533 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2524 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2523, _, _, _) => {
                            h2523
                        }
                    };
                    match m2524 {
                        Adt5::C0(h2522) => {
                            h2522
                        }
                    }
                })), {
                    let a2530 = f628(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2525, _, _) => {
                            h2525
                        }
                    });
                    let a2531 = {
                        let a2528 = f779(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2526, _) => {
                                h2526
                            }
                        });
                        let a2529 = f780(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2527) => {
                                h2527
                            }
                        });
                        append_list(a2528, a2529)
                    };
                    append_list(a2530, a2531)
                });
                append_list(a2532, a2533)
            };
            append_list(a2534, a2535)
        };
        append_list(a2536, a2537)
    }))
}

pub fn f732(v0: Adt38) -> R<Option<Adt6>> {
    let m2538 = v0;
    match m2538 {
        Adt38::C0(v1) => {
            Ok(Some(f781(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f782(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2539, _, _, _) => {
                    h2539
                }
            }, match v2.clone() {
                Adt42::C0(_, h2540, _, _) => {
                    h2540
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2541, _) => {
                    h2541
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2542) => {
                    h2542
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f782(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2543, _, _, _) => {
                    h2543
                }
            }, match v3.clone() {
                Adt43::C0(_, h2544, _, _) => {
                    h2544
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2545, _) => {
                    h2545
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2546) => {
                    h2546
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f783(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f784(v5)))
        }
    }
}

pub fn f733(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2570 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2547, _, _, _, _, _, _, _, _) => {
                h2547
            }
        })), List::<(Str, Adt6)>::nil());
        let a2571 = {
            let a2568 = f597(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2548, _, _, _, _, _, _, _) => {
                    h2548
                }
            });
            let a2569 = {
                let a2566 = f679(match v0.clone() {
                    Adt44::C0(_, _, h2549, _, _, _, _, _, _) => {
                        h2549
                    }
                });
                let a2567 = {
                    let a2564 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2550, _, _, _, _, _) => {
                            h2550
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2565 = {
                        let a2562 = f597(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2551, _, _, _, _) => {
                                h2551
                            }
                        });
                        let a2563 = {
                            let a2560 = f597(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2552, _, _, _) => {
                                    h2552
                                }
                            });
                            let a2561 = {
                                let a2558 = f785(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2553, _, _) => {
                                        h2553
                                    }
                                });
                                let a2559 = {
                                    let a2556 = f786(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2554, _) => {
                                            h2554
                                        }
                                    })?;
                                    let a2557 = f780(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2555) => {
                                            h2555
                                        }
                                    });
                                    append_list(a2556, a2557)
                                };
                                append_list(a2558, a2559)
                            };
                            append_list(a2560, a2561)
                        };
                        append_list(a2562, a2563)
                    };
                    append_list(a2564, a2565)
                };
                append_list(a2566, a2567)
            };
            append_list(a2568, a2569)
        };
        append_list(a2570, a2571)
    }))
}

pub fn f734(v0: Adt46) -> Adt6 {
    let m2572 = v0;
    match m2572 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2573, _, _, _) => {
                    h2573
                }
            })), {
                let a2577 = f597(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2574, _, _) => {
                        h2574
                    }
                });
                let a2578 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2575, _) => {
                        h2575
                    }
                })), f780(match v1 {
                    Adt47::C0(_, _, _, h2576) => {
                        h2576
                    }
                }));
                append_list(a2577, a2578)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2579, _, _, _) => {
                    h2579
                }
            })), {
                let a2583 = f597(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2580, _, _) => {
                        h2580
                    }
                });
                let a2584 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2581, _) => {
                        h2581
                    }
                })), f780(match v2 {
                    Adt48::C0(_, _, _, h2582) => {
                        h2582
                    }
                }));
                append_list(a2583, a2584)
            }))
        }
    }
}

pub fn f735(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a2605 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h2585, _, _, _, _, _, _, _) => {
                h2585
            }
        })), List::<(Str, Adt6)>::nil());
        let a2606 = {
            let a2603 = f597(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h2586, _, _, _, _, _, _) => {
                    h2586
                }
            });
            let a2604 = {
                let a2601 = f679(match v0.clone() {
                    Adt54::C0(_, _, h2587, _, _, _, _, _) => {
                        h2587
                    }
                });
                let a2602 = {
                    let a2599 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h2588, _, _, _, _) => {
                            h2588
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2600 = {
                        let a2597 = f597(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h2589, _, _, _) => {
                                h2589
                            }
                        });
                        let a2598 = {
                            let a2595 = f597(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h2590, _, _) => {
                                    h2590
                                }
                            });
                            let a2596 = {
                                let a2593 = f785(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h2591, _) => {
                                        h2591
                                    }
                                });
                                let a2594 = f780(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h2592) => {
                                        h2592
                                    }
                                });
                                append_list(a2593, a2594)
                            };
                            append_list(a2595, a2596)
                        };
                        append_list(a2597, a2598)
                    };
                    append_list(a2599, a2600)
                };
                append_list(a2601, a2602)
            };
            append_list(a2603, a2604)
        };
        append_list(a2605, a2606)
    })
}

pub fn f736(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2622 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2607, _, _, _, _, _) => {
                h2607
            }
        })), List::<(Str, Adt6)>::nil());
        let a2623 = {
            let a2620 = f597(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2608, _, _, _, _) => {
                    h2608
                }
            });
            let a2621 = {
                let a2618 = f679(match v0.clone() {
                    Adt56::C0(_, _, h2609, _, _, _) => {
                        h2609
                    }
                });
                let a2619 = {
                    let a2616 = f597(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h2610, _, _) => {
                            h2610
                        }
                    });
                    let a2617 = {
                        let a2614 = {
                            let m2612 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h2611, _) => {
                                    h2611
                                }
                            };
                            match m2612 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f787(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2615 = f780(match v0 {
                            Adt56::C0(_, _, _, _, _, h2613) => {
                                h2613
                            }
                        });
                        append_list(a2614, a2615)
                    };
                    append_list(a2616, a2617)
                };
                append_list(a2618, a2619)
            };
            append_list(a2620, a2621)
        };
        append_list(a2622, a2623)
    })
}

pub fn f737(v0: Adt59) -> R<Option<Adt6>> {
    let m2625 = f732(match v0.clone() {
        Adt59::C0(_, h2624) => {
            h2624
        }
    })?;
    match m2625 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f788(match v0 {
                Adt59::C0(h2626, _) => {
                    h2626
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f738(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f601(Str::lit("params"), v1)))
}

pub fn f739(v0: Adt65) -> Adt6 {
    let m2627 = v0;
    match m2627 {
        Adt65::C0(v1) => {
            f789(v1)
        }
        Adt65::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt68::C0(h2628, _) => {
                    h2628
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt68::C0(_, h2629) => {
                    h2629
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f740(v0: Adt69) -> R<Option<Adt6>> {
    let m2631 = f790(match v0.clone() {
        Adt69::C0(h2630, _, _, _, _, _, _, _, _, _) => {
            h2630
        }
    })?;
    match m2631 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2633 = f791(v0.clone());
                let a2634 = List::cons((Str::lit("maxTokens"), f145(match v0.clone() {
                    Adt69::C0(_, _, _, _, _, h2632, _, _, _, _) => {
                        h2632
                    }
                })?), f792(v0));
                append_list(a2633, a2634)
            }))))
        }
    }
}

pub fn f741(v0: List<Adt2>) -> List<Adt6> {
    f470(f793(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f742(v0: List<u64>) -> List<u64> {
    let a2637 = f794();
    let a2638 = {
        let a2635 = v0;
        let a2636 = f795();
        append_list(a2635, a2636)
    };
    append_list(a2637, a2638)
}

pub fn f743(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f796(v0, 0u64, v1)
}

pub fn f744(p2645: List<u64>) -> bool {
    let m2644 = p2645;
    {
        let mut m2644 = m2644;
        loop {
            let step = {
                let v0 = m2644;
                let m2639 = v0;
                match m2639.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2640 = v1;
                        let a2641 = 48u64;
                        if if nat_eq(a2640, a2641) {
                            true
                        } else {
                            let a2642 = v1;
                            let a2643 = 46u64;
                            nat_eq(a2642, a2643)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2644 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f745(v0: Adt139) -> bool {
    let m2646 = v0;
    match m2646 {
        Adt139::C0 => {
            true
        }
        Adt139::C1 => {
            false
        }
        Adt139::C2 => {
            false
        }
        Adt139::C3 => {
            false
        }
        Adt139::C4 => {
            false
        }
        Adt139::C5 => {
            false
        }
        Adt139::C6 => {
            false
        }
        Adt139::C7 => {
            false
        }
        Adt139::C8 => {
            false
        }
        Adt139::C9 => {
            false
        }
        Adt139::C10 => {
            false
        }
        Adt139::C11 => {
            false
        }
    }
}

pub fn f746(v0: Adt14) -> Adt139 {
    let m2647 = v0;
    match m2647 {
        Adt14::C0(_) => {
            Adt139::C0
        }
        Adt14::C1(_) => {
            Adt139::C1
        }
        Adt14::C2(_) => {
            Adt139::C2
        }
        Adt14::C3(_) => {
            Adt139::C3
        }
        Adt14::C4(_) => {
            Adt139::C4
        }
        Adt14::C5(_) => {
            Adt139::C5
        }
        Adt14::C6(_) => {
            Adt139::C6
        }
        Adt14::C7(_) => {
            Adt139::C7
        }
        Adt14::C8(_) => {
            Adt139::C8
        }
        Adt14::C9(_) => {
            Adt139::C9
        }
        Adt14::C10(_) => {
            Adt139::C10
        }
        Adt14::C11(_) => {
            Adt139::C11
        }
    }
}

pub fn f747(v0: Adt0, v1: Adt32) -> Adt129 {
    Adt129::C0(v1, Adt130::C0(Some(match v0 {
        Adt0::C0(h2648, _, _, _, _, _, _) => {
            h2648
        }
    }), None::<Adt5>), List::<Adt131>::nil())
}

pub fn f748(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2649, _, _, _) => {
            h2649
        }
    }, f344(), match v0.clone() {
        Adt0::C0(_, _, h2650, _, _, _, _) => {
            h2650
        }
    }, match v0 {
        Adt0::C0(_, h2651, _, _, _, _, _) => {
            h2651
        }
    })
}

pub fn f749(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt129, Adt132> {
    let m2652 = v2;
    match m2652 {
        Ok(v4) => {
            if f797(f746(v1.clone()), v4.clone()) {
                f798(v0, v1, v4)
            } else {
                Err::<Adt129, Adt132>(Adt132::C4(f799()))
            }
        }
        Err(v3) => {
            Err::<Adt129, Adt132>(Adt132::C4(v3))
        }
    }
}

pub fn f750(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14) -> R<Result<Adt32, Adt84>> {
    f800(v0.clone(), v1, v2, v3, v4.clone(), f801(v0, v4)?)
}

pub fn f751(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt133> {
    let m2653 = f186(v0.clone(), v1);
    match m2653 {
        Adt107::C0 => {
            Ok::<Option<Str>, Adt133>(None::<Str>)
        }
        Adt107::C1(v2) => {
            let m2654 = f266(v2);
            match m2654 {
                None => {
                    Err::<Option<Str>, Adt133>(Adt133::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt133>(Some(v3))
                }
            }
        }
    }
}

pub fn f752(v0: Str, v1: Adt5) -> Result<Str, Adt133> {
    let m2655 = f186(v0.clone(), v1);
    match m2655 {
        Adt107::C0 => {
            Err::<Str, Adt133>(Adt133::C7(v0))
        }
        Adt107::C1(v2) => {
            let m2656 = f266(v2);
            match m2656 {
                None => {
                    Err::<Str, Adt133>(Adt133::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt133>(v3)
                }
            }
        }
    }
}

pub fn f753(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt133> {
    let m2657 = f186(v0.clone(), v1);
    match m2657 {
        Adt107::C0 => {
            Ok::<Option<Adt5>, Adt133>(None::<Adt5>)
        }
        Adt107::C1(v2) => {
            let m2658 = f187(v2);
            match m2658 {
                None => {
                    Err::<Option<Adt5>, Adt133>(Adt133::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt133>(Some(v3))
                }
            }
        }
    }
}

pub fn f754(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt133> {
    let m2659 = f186(v0.clone(), v1);
    match m2659 {
        Adt107::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt133>(None::<List<(Str, Str)>>)
        }
        Adt107::C1(v2) => {
            let m2660 = f187(v2);
            match m2660 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt133>(Adt133::C8(v0))
                }
                Some(v3) => {
                    let m2662 = f802(v0, match v3 {
                        Adt5::C0(h2661) => {
                            h2661
                        }
                    });
                    match m2662 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt133>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt133>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f755(v0: Str, v1: Adt5) -> Result<Adt5, Adt133> {
    let m2663 = f186(v0.clone(), v1);
    match m2663 {
        Adt107::C0 => {
            Err::<Adt5, Adt133>(Adt133::C7(v0))
        }
        Adt107::C1(v2) => {
            let m2664 = f187(v2);
            match m2664 {
                None => {
                    Err::<Adt5, Adt133>(Adt133::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt133>(v3)
                }
            }
        }
    }
}

pub fn f756(v0: Adt5) -> Result<Adt25, Adt133> {
    let m2665 = f752(Str::lit("type"), v0.clone());
    match m2665 {
        Ok(v1) => {
            let a2666 = v1.clone();
            let a2667 = Str::lit("ref/prompt");
            if equal(a2666, a2667) {
                let m2668 = f752(Str::lit("name"), v0.clone());
                match m2668 {
                    Ok(v2) => {
                        let m2669 = f751(Str::lit("title"), v0);
                        match m2669 {
                            Ok(v3) => {
                                Ok::<Adt25, Adt133>(Adt25::C0(Adt26::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt25, Adt133>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt25, Adt133>(v5)
                    }
                }
            } else {
                let a2670 = v1;
                let a2671 = Str::lit("ref/resource");
                if equal(a2670, a2671) {
                    let m2672 = f752(Str::lit("uri"), v0);
                    match m2672 {
                        Ok(v6) => {
                            Ok::<Adt25, Adt133>(Adt25::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt25, Adt133>(v7)
                        }
                    }
                } else {
                    Err::<Adt25, Adt133>(Adt133::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt25, Adt133>(v8)
        }
    }
}

pub fn f757(v0: Adt5) -> Result<Adt27, Adt133> {
    let m2673 = f752(Str::lit("name"), v0.clone());
    match m2673 {
        Ok(v1) => {
            let m2674 = f752(Str::lit("value"), v0);
            match m2674 {
                Ok(v2) => {
                    Ok::<Adt27, Adt133>(Adt27::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt27, Adt133>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt27, Adt133>(v4)
        }
    }
}

pub fn f758(v0: Adt5) -> Result<Option<Adt28>, Adt133> {
    let m2675 = f186(Str::lit("context"), v0);
    match m2675 {
        Adt107::C0 => {
            Ok::<Option<Adt28>, Adt133>(None::<Adt28>)
        }
        Adt107::C1(v1) => {
            let m2676 = f187(v1);
            match m2676 {
                None => {
                    Err::<Option<Adt28>, Adt133>(Adt133::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2677 = f754(Str::lit("arguments"), v2);
                    match m2677 {
                        Ok(v3) => {
                            Ok::<Option<Adt28>, Adt133>(Some(Adt28::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt28>, Adt133>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f759(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt132> {
    if v0 {
        None::<Adt132>
    } else {
        Some(Adt132::C1(v1, v2))
    }
}

pub fn f760(v0: Str, p2682: List<Str>) -> bool {
    let m2681 = p2682;
    {
        let mut m2681 = m2681;
        loop {
            let step = {
                let v1 = m2681;
                let m2678 = v1;
                match m2678.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2679 = v2;
                        let a2680 = v0.clone();
                        if equal(a2679, a2680) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2681 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f761(v0: Option<Adt113>, v1: List<Adt115>, v2: Adt5) -> R<Option<Adt113>> {
    let m2683 = v0;
    match m2683 {
        None => {
            f803(v1, f804(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f762(v0: Option<Str>, v1: Adt145, v2: Adt5) -> Option<Adt113> {
    let m2684 = v1.clone();
    match m2684 {
        Adt145::C0 => {
            None::<Adt113>
        }
        Adt145::C1 => {
            f805(v0, f806(v1, v2))
        }
        Adt145::C2 => {
            f805(v0, f806(v1, v2))
        }
        Adt145::C3 => {
            f805(v0, f806(v1, v2))
        }
    }
}

pub fn f763(v0: Adt107, v1: Adt107) -> Adt152 {
    let m2685 = v0;
    match m2685 {
        Adt107::C0 => {
            Adt152::C1
        }
        Adt107::C1(_) => {
            let m2686 = v1;
            match m2686 {
                Adt107::C0 => {
                    Adt152::C2
                }
                Adt107::C1(_) => {
                    Adt152::C0
                }
            }
        }
    }
}

pub fn f764(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt133> {
    let m2687 = f266(v1);
    match m2687 {
        None => {
            Err::<Adt16, Adt133>(Adt133::C5)
        }
        Some(v3) => {
            let m2688 = f807(v2);
            match m2688 {
                None => {
                    Err::<Adt16, Adt133>(Adt133::C6)
                }
                Some(v4) => {
                    let m2689 = f753(f808(), v0.clone());
                    match m2689 {
                        Ok(v6) => {
                            let m2690 = f809(v0.clone());
                            match m2690 {
                                Ok(v8) => {
                                    let m2691 = f810(v0.clone());
                                    match m2691 {
                                        Ok(v10) => {
                                            Ok::<Adt16, Adt133>(Adt16::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt16, Adt133>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt16, Adt133>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt133>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f765(p2695: List<Adt6>) -> bool {
    let m2694 = p2695;
    {
        let mut m2694 = m2694;
        loop {
            let step = {
                let v0 = m2694;
                let m2692 = v0;
                match m2692.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2693 = f266(v1);
                        match m2693 {
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
                Ok(next) => { m2694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f766(p2699: List<Adt6>) -> bool {
    let m2698 = p2699;
    {
        let mut m2698 = m2698;
        loop {
            let step = {
                let v0 = m2698;
                let m2696 = v0;
                match m2696.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2697 = f266(v1);
                        match m2697 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f569(v3) {
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
                Ok(next) => { m2698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f767(v0: Str) -> Option<Adt13> {
    let a2700 = v0.clone();
    let a2701 = Str::lit("string");
    if equal(a2700, a2701) {
        Some(Adt13::C0)
    } else {
        let a2702 = v0.clone();
        let a2703 = Str::lit("integer");
        if equal(a2702, a2703) {
            Some(Adt13::C1)
        } else {
            let a2704 = v0;
            let a2705 = Str::lit("boolean");
            if equal(a2704, a2705) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f768(p2714: List<u64>, p2715: List<u64>) -> R<List<u64>> {
    let m2713 = (p2714, p2715);
    {
        let mut m2713 = m2713;
        loop {
            let step = {
                let (v0, v1) = m2713;
                let m2706 = v0;
                match m2706.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2707 = 65u64;
                        let a2708 = v2;
                        if if nat_le(a2707, a2708) {
                            let a2709 = v2;
                            let a2710 = 90u64;
                            nat_le(a2709, a2710)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2711 = v2;
                                let a2712 = 32u64;
                                nat_add(a2711, a2712)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f769(p2722: List<u64>, p2723: List<u64>) -> bool {
    let m2721 = (p2722, p2723);
    {
        let mut m2721 = m2721;
        loop {
            let step = {
                let (v0, v1) = m2721;
                let m2716 = v0;
                match m2716.uncons() {
                    None => {
                        let m2717 = v1;
                        match m2717.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2718 = v1;
                        match m2718.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2719 = v4;
                                let a2720 = v6;
                                if nat_eq(a2719, a2720) {
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
                Ok(next) => { m2721 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f770(v0: Adt6) -> u64 {
    let m2724 = v0;
    match m2724 {
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

pub fn f771(v0: Adt151) -> bool {
    let m2725 = v0;
    match m2725 {
        Adt151::C0 => {
            false
        }
        Adt151::C1 => {
            true
        }
        Adt151::C2 => {
            false
        }
    }
}

pub fn f772(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt153 {
    let m2726 = v0;
    match m2726 {
        None => {
            Adt153::C0
        }
        Some(v2) => {
            Adt153::C1({
                let a2727 = v2;
                let a2728 = v1;
                append_list(a2727, a2728)
            })
        }
    }
}

pub fn f773(p2733: List<Adt6>, p2734: List<Adt6>, p2735: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2732 = (p2733, (p2734, p2735));
    {
        let mut m2732 = m2732;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2732;
                let m2729 = v0;
                match m2729.uncons() {
                    None => {
                        let m2730 = v1;
                        match m2730.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2731 = v1;
                        match m2731.uncons() {
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
                Ok(next) => { m2732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f774(p2741: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2742: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2740 = (p2741, p2742);
    {
        let mut m2740 = m2740;
        loop {
            let step = {
                let (v0, v2) = m2740;
                let m2736 = v0;
                match m2736.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2738 = f262({
                            let (h2737, _) = v3;
                            h2737
                        }, v1.clone());
                        match m2738 {
                            Adt107::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt107::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2739) = v3;
                                    h2739
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f775(v0: Adt151) -> Adt151 {
    let m2743 = v0;
    match m2743 {
        Adt151::C0 => {
            Adt151::C2
        }
        Adt151::C1 => {
            Adt151::C1
        }
        Adt151::C2 => {
            Adt151::C0
        }
    }
}

pub fn f776(v0: i64, v1: i64) -> R<i64> {
    f316(f241(f778(v0)?, 0i64)?, v1)
}

pub fn f777(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt151> {
    let a2744 = v1;
    let a2745 = v3;
    if nat_lt(a2744, a2745) {
        Ok(f659(f811({
            let a2746 = v3;
            let a2747 = v1;
            nat_sub(a2746, a2747)
        }, v0)?, v2))
    } else {
        let a2748 = v3;
        let a2749 = v1;
        if nat_lt(a2748, a2749) {
            Ok(f659(v0, f811({
                let a2750 = v1;
                let a2751 = v3;
                nat_sub(a2750, a2751)
            }, v2)?))
        } else {
            Ok(f659(v0, v2))
        }
    }
}

pub fn f778(v0: i64) -> R<u64> {
    f812(20u64, v0, 0u64)
}

pub fn f779(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2752 = v0;
    match m2752 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2764 = f597(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2753, _, _, _, _) => {
                        h2753
                    }
                });
                let a2765 = {
                    let a2762 = f602(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2754, _, _, _) => {
                            h2754
                        }
                    });
                    let a2763 = {
                        let a2760 = f602(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2755, _, _) => {
                                h2755
                            }
                        });
                        let a2761 = {
                            let a2758 = f602(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2756, _) => {
                                    h2756
                                }
                            });
                            let a2759 = f602(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2757) => {
                                    h2757
                                }
                            });
                            append_list(a2758, a2759)
                        };
                        append_list(a2760, a2761)
                    };
                    append_list(a2762, a2763)
                };
                append_list(a2764, a2765)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f780(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f628(Str::lit("_meta"), v0)
}

pub fn f781(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2766, _, _) => {
            h2766
        }
    })), {
        let a2769 = f785(match v0.clone() {
            Adt39::C0(_, h2767, _) => {
                h2767
            }
        });
        let a2770 = f780(match v0 {
            Adt39::C0(_, _, h2768) => {
                h2768
            }
        });
        append_list(a2769, a2770)
    })))
}

pub fn f782(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2771 = f785(v3);
        let a2772 = f780(v4);
        append_list(a2771, a2772)
    }))))
}

pub fn f783(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2796 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2773, _, _, _, _, _, _, _, _) => {
                h2773
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2797 = {
            let a2794 = f597(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2774, _, _, _, _, _, _, _) => {
                    h2774
                }
            });
            let a2795 = {
                let a2792 = f679(match v0.clone() {
                    Adt44::C0(_, _, h2775, _, _, _, _, _, _) => {
                        h2775
                    }
                });
                let a2793 = {
                    let a2790 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2776, _, _, _, _, _) => {
                            h2776
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2791 = {
                        let a2788 = f597(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2777, _, _, _, _) => {
                                h2777
                            }
                        });
                        let a2789 = {
                            let a2786 = f597(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2778, _, _, _) => {
                                    h2778
                                }
                            });
                            let a2787 = {
                                let a2784 = f785(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2779, _, _) => {
                                        h2779
                                    }
                                });
                                let a2785 = {
                                    let a2782 = f786(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2780, _) => {
                                            h2780
                                        }
                                    })?;
                                    let a2783 = f780(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2781) => {
                                            h2781
                                        }
                                    });
                                    append_list(a2782, a2783)
                                };
                                append_list(a2784, a2785)
                            };
                            append_list(a2786, a2787)
                        };
                        append_list(a2788, a2789)
                    };
                    append_list(a2790, a2791)
                };
                append_list(a2792, a2793)
            };
            append_list(a2794, a2795)
        };
        append_list(a2796, a2797)
    }))
}

pub fn f784(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f734(match v0.clone() {
        Adt45::C0(h2798, _, _) => {
            h2798
        }
    })), {
        let a2801 = f785(match v0.clone() {
            Adt45::C0(_, h2799, _) => {
                h2799
            }
        });
        let a2802 = f780(match v0 {
            Adt45::C0(_, _, h2800) => {
                h2800
            }
        });
        append_list(a2801, a2802)
    })))
}

pub fn f785(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2803 = v0;
    match m2803 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2809 = f813(match v1.clone() {
                    Adt40::C0(h2804, _, _) => {
                        h2804
                    }
                });
                let a2810 = {
                    let a2807 = f814(match v1.clone() {
                        Adt40::C0(_, h2805, _) => {
                            h2805
                        }
                    });
                    let a2808 = f597(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2806) => {
                            h2806
                        }
                    });
                    append_list(a2807, a2808)
                };
                append_list(a2809, a2810)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f786(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2811 = v0;
    match m2811 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f145(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f787(v0: List<Adt57>) -> List<Adt6> {
    f470(f815(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f788(v0: Adt41) -> Str {
    let m2812 = v0;
    match m2812 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f789(v0: Adt66) -> Adt6 {
    Adt6::C5({
        let a2819 = {
            let m2814 = match v0.clone() {
                Adt66::C0(h2813, _, _) => {
                    h2813
                }
            };
            match m2814 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2820 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt66::C0(_, h2815, _) => {
                h2815
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2818 = match v0 {
                Adt66::C0(_, _, h2817) => {
                    h2817
                }
            };
            match m2818 {
                Adt5::C0(h2816) => {
                    h2816
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2819, a2820)
    })
}

pub fn f790(v0: List<Adt70>) -> R<Option<List<Adt6>>> {
    let m2821 = f816(v0, List::<Adt6>::nil())?;
    match m2821 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f470(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f791(v0: Adt69) -> List<(Str, Adt6)> {
    let a2832 = {
        let m2823 = match v0.clone() {
            Adt69::C0(_, h2822, _, _, _, _, _, _, _, _) => {
                h2822
            }
        };
        match m2823 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f817(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2833 = {
        let a2830 = f597(Str::lit("systemPrompt"), match v0.clone() {
            Adt69::C0(_, _, h2824, _, _, _, _, _, _, _) => {
                h2824
            }
        });
        let a2831 = {
            let a2828 = {
                let m2826 = match v0.clone() {
                    Adt69::C0(_, _, _, h2825, _, _, _, _, _, _) => {
                        h2825
                    }
                };
                match m2826 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f818(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2829 = f819(Str::lit("temperature"), match v0 {
                Adt69::C0(_, _, _, _, h2827, _, _, _, _, _) => {
                    h2827
                }
            });
            append_list(a2828, a2829)
        };
        append_list(a2830, a2831)
    };
    append_list(a2832, a2833)
}

pub fn f792(v0: Adt69) -> List<(Str, Adt6)> {
    let a2845 = {
        let m2835 = match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, h2834, _, _, _) => {
                h2834
            }
        };
        match m2835 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f595(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2846 = {
        let a2843 = f628(Str::lit("metadata"), match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, _, h2836, _, _) => {
                h2836
            }
        });
        let a2844 = {
            let a2841 = {
                let m2838 = match v0.clone() {
                    Adt69::C0(_, _, _, _, _, _, _, _, h2837, _) => {
                        h2837
                    }
                };
                match m2838 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f599(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2842 = {
                let m2840 = match v0 {
                    Adt69::C0(_, _, _, _, _, _, _, _, _, h2839) => {
                        h2839
                    }
                };
                match m2840 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f820(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2841, a2842)
        };
        append_list(a2843, a2844)
    };
    append_list(a2845, a2846)
}

pub fn f793(p2849: List<Adt2>, p2850: List<Adt6>) -> List<Adt6> {
    let m2848 = (p2849, p2850);
    {
        let mut m2848 = m2848;
        loop {
            let step = {
                let (v0, v1) = m2848;
                let m2847 = v0;
                match m2847.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f821(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f794() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f795() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f796(p2855: List<u64>, p2856: u64, p2857: List<u64>) -> List<u64> {
    let m2854 = (p2855, (p2856, p2857));
    {
        let mut m2854 = m2854;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2854;
                let m2851 = v0;
                match m2851.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f822(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt154 = f823(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt154::C0(h2852, _) => {
                                h2852
                            }
                        }, match v5 {
                            Adt154::C0(_, h2853) => {
                                h2853
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f797(v0: Adt139, v1: Adt32) -> bool {
    let m2858 = v1;
    match m2858 {
        Adt32::C0(_) => {
            f824(v0, Adt139::C0)
        }
        Adt32::C1(_) => {
            f824(v0, Adt139::C1)
        }
        Adt32::C2(_) => {
            f824(v0, Adt139::C2)
        }
        Adt32::C3(_) => {
            f824(v0, Adt139::C2)
        }
        Adt32::C4(_) => {
            f824(v0, Adt139::C3)
        }
        Adt32::C5(_) => {
            f824(v0, Adt139::C4)
        }
        Adt32::C6(_) => {
            f824(v0, Adt139::C5)
        }
        Adt32::C7(_) => {
            f824(v0, Adt139::C6)
        }
        Adt32::C8(_) => {
            f824(v0, Adt139::C7)
        }
        Adt32::C9(_) => {
            f824(v0, Adt139::C8)
        }
        Adt32::C10(_) => {
            f824(v0, Adt139::C9)
        }
        Adt32::C11 => {
            if f824(v0.clone(), Adt139::C10) {
                true
            } else {
                f824(v0, Adt139::C11)
            }
        }
        Adt32::C12(_) => {
            f825(v0)
        }
        Adt32::C13(_) => {
            f826(v0)
        }
    }
}

pub fn f798(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt129, Adt132> {
    let m2859 = v2.clone();
    match m2859 {
        Adt32::C0(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt129, Adt132>(f830(v0, f828(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt129, Adt132>(f747(v0, v2))
        }
        Adt32::C12(v3) => {
            f827(v0, f828(v1), v3)
        }
        Adt32::C13(v4) => {
            f829(v0, f828(v1), v4)
        }
    }
}

pub fn f799() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f800(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14, v5: Result<Adt14, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m2860 = v5;
    match m2860 {
        Ok(v7) => {
            f831(v0, v4, f832(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt32, Adt84>(v6))
        }
    }
}

pub fn f801(v0: Bytes, v1: Adt14) -> R<Result<Adt14, Adt84>> {
    let m2861 = v1.clone();
    match m2861 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C1(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C2(v2) => {
            f833(v0, v2)
        }
        Adt14::C3(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C4(v3) => {
            f834(v0, v3)
        }
        Adt14::C5(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C6(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C7(v4) => {
            f835(v0, v4)
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

pub fn f802(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt133> {
    let m2862 = f836(v0, v1, List::<(Str, Str)>::nil());
    match m2862 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt133>(f837(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt133>(v3)
        }
    }
}

pub fn f803(p2866: List<Adt115>, v1: Adt5) -> R<Option<Adt113>> {
    let m2865 = p2866;
    {
        let mut m2865 = m2865;
        loop {
            let step = {
                let v0 = m2865;
                let m2863 = v0;
                match m2863.uncons() {
                    None => {
                        Err::<List<Adt115>, R<Option<Adt113>>>(Ok(None::<Adt113>))
                    }
                    Some((v2, v3)) => {
                        let m2864 = f838(v2, v1.clone())?;
                        match m2864 {
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
                Ok(next) => { m2865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f804(v0: Adt5) -> Adt5 {
    let m2867 = f186(Str::lit("arguments"), v0);
    match m2867 {
        Adt107::C0 => {
            f26()
        }
        Adt107::C1(v1) => {
            let m2868 = f187(v1);
            match m2868 {
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

pub fn f805(v0: Option<Str>, v1: Option<Str>) -> Option<Adt113> {
    let m2869 = v0;
    match m2869 {
        None => {
            Some(Adt113::C2)
        }
        Some(v2) => {
            let m2870 = v1;
            match m2870 {
                None => {
                    None::<Adt113>
                }
                Some(v3) => {
                    let a2871 = v2;
                    let a2872 = v3;
                    if equal(a2871, a2872) {
                        None::<Adt113>
                    } else {
                        Some(Adt113::C5)
                    }
                }
            }
        }
    }
}

pub fn f806(v0: Adt145, v1: Adt5) -> Option<Str> {
    let m2873 = v0;
    match m2873 {
        Adt145::C0 => {
            None::<Str>
        }
        Adt145::C1 => {
            let m2874 = f186(Str::lit("name"), v1);
            match m2874 {
                Adt107::C0 => {
                    None::<Str>
                }
                Adt107::C1(v2) => {
                    f266(v2)
                }
            }
        }
        Adt145::C2 => {
            let m2875 = f186(Str::lit("uri"), v1);
            match m2875 {
                Adt107::C0 => {
                    None::<Str>
                }
                Adt107::C1(v3) => {
                    f266(v3)
                }
            }
        }
        Adt145::C3 => {
            let m2876 = f186(Str::lit("taskId"), v1);
            match m2876 {
                Adt107::C0 => {
                    None::<Str>
                }
                Adt107::C1(v4) => {
                    f266(v4)
                }
            }
        }
    }
}

pub fn f807(v0: Adt6) -> Option<Adt17> {
    let m2877 = f187(v0);
    match m2877 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2878 = f839(Str::lit("experimental"), v1.clone());
            match m2878 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2879 = f839(Str::lit("roots"), v1.clone());
                    match m2879 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2880 = f839(Str::lit("sampling"), v1.clone());
                            match m2880 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2881 = f839(Str::lit("elicitation"), v1.clone());
                                    match m2881 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2882 = f839(Str::lit("extensions"), v1);
                                            match m2882 {
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

pub fn f808() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f809(v0: Adt5) -> Result<Option<Adt18>, Adt133> {
    let m2883 = f751(f840(), v0);
    match m2883 {
        Ok(v2) => {
            let m2884 = v2;
            match m2884 {
                None => {
                    Ok::<Option<Adt18>, Adt133>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2885 = f841(v3);
                    match m2885 {
                        Adt155::C0 => {
                            Err::<Option<Adt18>, Adt133>(Adt133::C8(f840()))
                        }
                        Adt155::C1(v4) => {
                            Ok::<Option<Adt18>, Adt133>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt18>, Adt133>(v1)
        }
    }
}

pub fn f810(v0: Adt5) -> Result<Option<Adt19>, Adt133> {
    let m2886 = f186(f842(), v0);
    match m2886 {
        Adt107::C0 => {
            Ok::<Option<Adt19>, Adt133>(None::<Adt19>)
        }
        Adt107::C1(v1) => {
            let m2887 = f266(v1.clone());
            match m2887 {
                None => {
                    let m2888 = f658(v1);
                    match m2888 {
                        None => {
                            Err::<Option<Adt19>, Adt133>(Adt133::C8(f842()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt19>, Adt133>(Some(Adt19::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt19>, Adt133>(Some(Adt19::C0(v2)))
                }
            }
        }
    }
}

pub fn f811(p2893: u64, p2894: i64) -> R<i64> {
    let m2892 = (p2893, p2894);
    {
        let mut m2892 = m2892;
        loop {
            let step = {
                let (v0, v1) = m2892;
                let m2889 = v0;
                if m2889 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2889 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2890 = v1;
                        let a2891 = 10i64;
                        int_mul(a2890, a2891)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2892 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f812(p2905: u64, p2906: i64, p2907: u64) -> R<u64> {
    let m2904 = (p2905, (p2906, p2907));
    {
        let mut m2904 = m2904;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2904;
                let m2895 = v0;
                if m2895 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2895 - 1;
                    let v4: u64 = {
                        let a2896 = v2;
                        let a2897 = 1u64;
                        nat_add(a2896, a2897)?
                    };
                    let m2900 = {
                        let a2898 = v1;
                        let a2899 = 10i64;
                        compare(a2898, a2899)
                    };
                    if match m2900 {
                        Ordering::Less => {
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
                            let a2901 = v1;
                            let a2902 = 10i64;
                            let a2903 = 0i64;
                            int_quot(a2901, a2902, a2903)?
                        }, v4)))
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

pub fn f813(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2908 = v0;
    match m2908 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f843(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f814(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2909 = v0;
    match m2909 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f815(p2912: List<Adt57>, p2913: List<Adt6>) -> List<Adt6> {
    let m2911 = (p2912, p2913);
    {
        let mut m2911 = m2911;
        loop {
            let step = {
                let (v0, v1) = m2911;
                let m2910 = v0;
                match m2910.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f844(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f816(p2917: List<Adt70>, p2918: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2916 = (p2917, p2918);
    {
        let mut m2916 = m2916;
        loop {
            let step = {
                let (v0, v1) = m2916;
                let m2914 = v0;
                match m2914.uncons() {
                    None => {
                        Err::<(List<Adt70>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2915 = f845(v2)?;
                        match m2915 {
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
                Ok(next) => { m2916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f817(v0: Adt75) -> Adt6 {
    Adt6::C5({
        let a2928 = {
            let m2920 = match v0.clone() {
                Adt75::C0(h2919, _, _, _) => {
                    h2919
                }
            };
            match m2920 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f470(f846(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2929 = {
            let a2926 = f819(Str::lit("costPriority"), match v0.clone() {
                Adt75::C0(_, h2921, _, _) => {
                    h2921
                }
            });
            let a2927 = {
                let a2924 = f819(Str::lit("speedPriority"), match v0.clone() {
                    Adt75::C0(_, _, h2922, _) => {
                        h2922
                    }
                });
                let a2925 = f819(Str::lit("intelligencePriority"), match v0 {
                    Adt75::C0(_, _, _, h2923) => {
                        h2923
                    }
                });
                append_list(a2924, a2925)
            };
            append_list(a2926, a2927)
        };
        append_list(a2928, a2929)
    })
}

pub fn f818(v0: Adt77) -> Str {
    let m2930 = v0;
    match m2930 {
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

pub fn f819(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2931 = v1;
    match m2931 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f820(v0: Adt78) -> List<(Str, Adt6)> {
    let m2933 = match v0 {
        Adt78::C0(h2932) => {
            h2932
        }
    };
    match m2933 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f847(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f821(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2934, _, _, _) => {
            h2934
        }
    })), {
        let a2940 = f597(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2935, _, _) => {
                h2935
            }
        });
        let a2941 = {
            let a2938 = f848(match v0.clone() {
                Adt2::C0(_, _, h2936, _) => {
                    h2936
                }
            });
            let a2939 = f849(match v0 {
                Adt2::C0(_, _, _, h2937) => {
                    h2937
                }
            });
            append_list(a2938, a2939)
        };
        append_list(a2940, a2941)
    }))
}

pub fn f822(v0: u64, v1: List<u64>) -> List<u64> {
    let a2942 = v0;
    let a2943 = 0u64;
    if nat_eq(a2942, a2943) {
        v1
    } else {
        let a2944 = v0;
        let a2945 = 1u64;
        if nat_eq(a2944, a2945) {
            List::cons(194u64, v1)
        } else {
            let a2946 = v0;
            let a2947 = 2u64;
            if nat_eq(a2946, a2947) {
                List::cons(226u64, v1)
            } else {
                let a2948 = v0;
                let a2949 = 3u64;
                if nat_eq(a2948, a2949) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f823(v0: u64, v1: u64, v2: List<u64>) -> Adt154 {
    let a2950 = v0;
    let a2951 = 0u64;
    if nat_eq(a2950, a2951) {
        f850(v1, v2)
    } else {
        let a2952 = v0;
        let a2953 = 1u64;
        if nat_eq(a2952, a2953) {
            let a2954 = 128u64;
            let a2955 = v1;
            if if nat_le(a2954, a2955) {
                let a2956 = v1;
                let a2957 = 159u64;
                nat_le(a2956, a2957)
            } else {
                false
            } {
                Adt154::C0(0u64, f851(v2))
            } else {
                f850(v1, List::cons(194u64, v2))
            }
        } else {
            let a2958 = v0;
            let a2959 = 2u64;
            if nat_eq(a2958, a2959) {
                let a2960 = v1;
                let a2961 = 128u64;
                if nat_eq(a2960, a2961) {
                    Adt154::C0(3u64, v2)
                } else {
                    let a2962 = v1;
                    let a2963 = 129u64;
                    if nat_eq(a2962, a2963) {
                        Adt154::C0(4u64, v2)
                    } else {
                        f850(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2964 = v0;
                let a2965 = 3u64;
                if nat_eq(a2964, a2965) {
                    let a2966 = 170u64;
                    let a2967 = v1;
                    if if nat_le(a2966, a2967) {
                        let a2968 = v1;
                        let a2969 = 174u64;
                        nat_le(a2968, a2969)
                    } else {
                        false
                    } {
                        Adt154::C0(0u64, f851(v2))
                    } else {
                        f850(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2970 = 166u64;
                    let a2971 = v1;
                    if if nat_le(a2970, a2971) {
                        let a2972 = v1;
                        let a2973 = 169u64;
                        nat_le(a2972, a2973)
                    } else {
                        false
                    } {
                        Adt154::C0(0u64, f851(v2))
                    } else {
                        f850(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f824(v0: Adt139, v1: Adt139) -> bool {
    let a2974 = f632(v0);
    let a2975 = f632(v1);
    equal(a2974, a2975)
}

pub fn f825(v0: Adt139) -> bool {
    let m2976 = v0;
    match m2976 {
        Adt139::C0 => {
            false
        }
        Adt139::C1 => {
            false
        }
        Adt139::C2 => {
            true
        }
        Adt139::C3 => {
            false
        }
        Adt139::C4 => {
            true
        }
        Adt139::C5 => {
            false
        }
        Adt139::C6 => {
            false
        }
        Adt139::C7 => {
            true
        }
        Adt139::C8 => {
            false
        }
        Adt139::C9 => {
            false
        }
        Adt139::C10 => {
            false
        }
        Adt139::C11 => {
            false
        }
    }
}

pub fn f826(v0: Adt139) -> bool {
    let m2977 = v0;
    match m2977 {
        Adt139::C0 => {
            false
        }
        Adt139::C1 => {
            false
        }
        Adt139::C2 => {
            true
        }
        Adt139::C3 => {
            false
        }
        Adt139::C4 => {
            false
        }
        Adt139::C5 => {
            false
        }
        Adt139::C6 => {
            false
        }
        Adt139::C7 => {
            false
        }
        Adt139::C8 => {
            false
        }
        Adt139::C9 => {
            false
        }
        Adt139::C10 => {
            false
        }
        Adt139::C11 => {
            false
        }
    }
}

pub fn f827(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt129, Adt132> {
    f852(v0, v2.clone(), f853(match v1 {
        Adt16::C0(_, h2978, _, _, _, _) => {
            h2978
        }
    }, f610(v2)))
}

pub fn f828(v0: Adt14) -> Adt16 {
    let m2979 = v0;
    match m2979 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2980) => {
                    h2980
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2981, _) => {
                    h2981
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2982, _, _, _, _) => {
                    h2982
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2983, _) => {
                    h2983
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2984, _, _, _) => {
                    h2984
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2985, _) => {
                    h2985
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2986, _) => {
                    h2986
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2987, _, _, _, _) => {
                    h2987
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2988, _, _, _) => {
                    h2988
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2989, _) => {
                    h2989
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2990, _, _) => {
                    h2990
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2991, _) => {
                    h2991
                }
            }
        }
    }
}

pub fn f829(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt129, Adt132> {
    f854(v0, v2, f855(match v1 {
        Adt16::C0(_, h2992, _, _, _, _) => {
            h2992
        }
    }))
}

pub fn f830(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt129 {
    Adt129::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2993, _) => {
            h2993
        }
    }), {
        let m2995 = f747(v0, Adt32::C11);
        match m2995 {
            Adt129::C0(_, h2994, _) => {
                h2994
            }
        }
    }, f856(v1, match v2 {
        Adt49::C0(_, h2996) => {
            h2996
        }
    }))
}

pub fn f831(v0: Bytes, v1: Adt14, v2: Result<Adt32, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m2997 = v2;
    match m2997 {
        Ok(v4) => {
            f857(v0, f746(v1.clone()), f858(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f832(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2998 = v3.clone();
    match m2998 {
        Adt14::C0(_) => {
            let c3000: Fn0 = v2;
            c3000.apply(v3)
        }
        Adt14::C1(_) => {
            let c3001: Fn0 = v2;
            c3001.apply(v3)
        }
        Adt14::C2(_) => {
            if f859(v0, v1) {
                let c2999: Fn0 = v2;
                c2999.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c3002: Fn0 = v2;
            c3002.apply(v3)
        }
        Adt14::C4(_) => {
            let c3003: Fn0 = v2;
            c3003.apply(v3)
        }
        Adt14::C5(_) => {
            let c3004: Fn0 = v2;
            c3004.apply(v3)
        }
        Adt14::C6(_) => {
            let c3005: Fn0 = v2;
            c3005.apply(v3)
        }
        Adt14::C7(_) => {
            let c3006: Fn0 = v2;
            c3006.apply(v3)
        }
        Adt14::C8(_) => {
            let c3007: Fn0 = v2;
            c3007.apply(v3)
        }
        Adt14::C9(_) => {
            let c3008: Fn0 = v2;
            c3008.apply(v3)
        }
        Adt14::C10(_) => {
            let c3009: Fn0 = v2;
            c3009.apply(v3)
        }
        Adt14::C11(_) => {
            let c3010: Fn0 = v2;
            c3010.apply(v3)
        }
    }
}

pub fn f833(v0: Bytes, v1: Adt21) -> R<Result<Adt14, Adt84>> {
    let m3014 = f860(v0, Str::lit("tools/call"), match v1.clone() {
        Adt21::C0(_, h3011, _, _, _) => {
            h3011
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, _, h3012) => {
            h3012
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, h3013, _) => {
            h3013
        }
    })?;
    match m3014 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C2(Adt21::C0(match v1.clone() {
                Adt21::C0(h3015, _, _, _, _) => {
                    h3015
                }
            }, match v1.clone() {
                Adt21::C0(_, h3016, _, _, _) => {
                    h3016
                }
            }, match v1.clone() {
                Adt21::C0(_, _, h3017, _, _) => {
                    h3017
                }
            }, match v1 {
                Adt21::C0(_, _, _, h3018, _) => {
                    h3018
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f834(v0: Bytes, v1: Adt22) -> R<Result<Adt14, Adt84>> {
    let m3022 = f860(v0, Str::lit("resources/read"), match v1.clone() {
        Adt22::C0(_, h3019, _, _) => {
            h3019
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h3020) => {
            h3020
        }
    }, match v1.clone() {
        Adt22::C0(_, _, h3021, _) => {
            h3021
        }
    })?;
    match m3022 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C4(Adt22::C0(match v1.clone() {
                Adt22::C0(h3023, _, _, _) => {
                    h3023
                }
            }, match v1.clone() {
                Adt22::C0(_, h3024, _, _) => {
                    h3024
                }
            }, match v1 {
                Adt22::C0(_, _, h3025, _) => {
                    h3025
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f835(v0: Bytes, v1: Adt23) -> R<Result<Adt14, Adt84>> {
    let m3029 = f860(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt23::C0(_, h3026, _, _, _) => {
            h3026
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, _, h3027) => {
            h3027
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h3028, _) => {
            h3028
        }
    })?;
    match m3029 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C7(Adt23::C0(match v1.clone() {
                Adt23::C0(h3030, _, _, _, _) => {
                    h3030
                }
            }, match v1.clone() {
                Adt23::C0(_, h3031, _, _, _) => {
                    h3031
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h3032, _, _) => {
                    h3032
                }
            }, match v1 {
                Adt23::C0(_, _, _, h3033, _) => {
                    h3033
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f836(v0: Str, p3039: List<(Str, Adt6)>, p3040: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt133> {
    let m3038 = (p3039, p3040);
    {
        let mut m3038 = m3038;
        loop {
            let step = {
                let (v1, v2) = m3038;
                let m3034 = v1;
                match m3034.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt133>>(Ok::<List<(Str, Str)>, Adt133>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3036 = f266({
                            let (_, h3035) = v3;
                            h3035
                        });
                        match m3036 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt133>>(Err::<List<(Str, Str)>, Adt133>(Adt133::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt133>>((v4, List::cons(({
                                    let (h3037, _) = v3;
                                    h3037
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3038 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f837(p3043: List<(Str, Str)>, p3044: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3042 = (p3043, p3044);
    {
        let mut m3042 = m3042;
        loop {
            let step = {
                let (v0, v1) = m3042;
                let m3041 = v0;
                match m3041.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3042 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f838(v0: Adt115, v1: Adt5) -> R<Option<Adt113>> {
    let m3046 = f861(match v0.clone() {
        Adt115::C0(_, h3045, _, _) => {
            h3045
        }
    }, v1);
    match m3046 {
        Adt107::C0 => {
            Ok(f862(v0))
        }
        Adt107::C1(v2) => {
            if f863(v2.clone()) {
                Ok(f862(v0))
            } else {
                f864(v0, v2)
            }
        }
    }
}

pub fn f839(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m3047 = f186(v0, v1);
    match m3047 {
        Adt107::C0 => {
            Some(None::<Adt5>)
        }
        Adt107::C1(v2) => {
            let m3048 = f187(v2);
            match m3048 {
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

pub fn f840() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f841(v0: Str) -> Adt155 {
    let a3049 = v0.clone();
    let a3050 = Str::lit("debug");
    if equal(a3049, a3050) {
        Adt155::C1(Adt18::C0)
    } else {
        let a3051 = v0.clone();
        let a3052 = Str::lit("info");
        if equal(a3051, a3052) {
            Adt155::C1(Adt18::C1)
        } else {
            let a3053 = v0.clone();
            let a3054 = Str::lit("notice");
            if equal(a3053, a3054) {
                Adt155::C1(Adt18::C2)
            } else {
                let a3055 = v0.clone();
                let a3056 = Str::lit("warning");
                if equal(a3055, a3056) {
                    Adt155::C1(Adt18::C3)
                } else {
                    let a3057 = v0.clone();
                    let a3058 = Str::lit("error");
                    if equal(a3057, a3058) {
                        Adt155::C1(Adt18::C4)
                    } else {
                        let a3059 = v0.clone();
                        let a3060 = Str::lit("critical");
                        if equal(a3059, a3060) {
                            Adt155::C1(Adt18::C5)
                        } else {
                            let a3061 = v0.clone();
                            let a3062 = Str::lit("alert");
                            if equal(a3061, a3062) {
                                Adt155::C1(Adt18::C6)
                            } else {
                                let a3063 = v0;
                                let a3064 = Str::lit("emergency");
                                if equal(a3063, a3064) {
                                    Adt155::C1(Adt18::C7)
                                } else {
                                    Adt155::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f842() -> Str {
    Str::lit("progressToken")
}

pub fn f843(v0: List<Adt41>) -> List<Adt6> {
    f470(f865(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f844(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a3073 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h3065, _, _, _) => {
                h3065
            }
        })), List::<(Str, Adt6)>::nil());
        let a3074 = {
            let a3071 = f597(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h3066, _, _) => {
                    h3066
                }
            });
            let a3072 = {
                let a3069 = f597(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h3067, _) => {
                        h3067
                    }
                });
                let a3070 = f602(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h3068) => {
                        h3068
                    }
                });
                append_list(a3069, a3070)
            };
            append_list(a3071, a3072)
        };
        append_list(a3073, a3074)
    })
}

pub fn f845(v0: Adt70) -> R<Option<Adt6>> {
    let m3076 = f866(match v0.clone() {
        Adt70::C0(_, h3075, _) => {
            h3075
        }
    })?;
    match m3076 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f788(match v0.clone() {
                Adt70::C0(h3077, _, _) => {
                    h3077
                }
            }))), List::cons((Str::lit("content"), v1), f780(match v0 {
                Adt70::C0(_, _, h3078) => {
                    h3078
                }
            }))))))
        }
    }
}

pub fn f846(p3082: List<Adt76>, p3083: List<Adt6>) -> List<Adt6> {
    let m3081 = (p3082, p3083);
    {
        let mut m3081 = m3081;
        loop {
            let step = {
                let (v0, v1) = m3081;
                let m3079 = v0;
                match m3079.uncons() {
                    None => {
                        Err::<(List<Adt76>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt76>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f597(Str::lit("name"), match v2 {
                            Adt76::C0(h3080) => {
                                h3080
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3081 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f847(v0: Adt79) -> Str {
    let m3084 = v0;
    match m3084 {
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

pub fn f848(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3085 = v0;
    match m3085 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f541(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f849(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3086 = v0;
    match m3086 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f867(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f850(v0: u64, v1: List<u64>) -> Adt154 {
    let a3087 = v0;
    let a3088 = 32u64;
    if nat_lt(a3087, a3088) {
        if f868(v0) {
            Adt154::C0(0u64, List::cons(v0, v1))
        } else {
            Adt154::C0(0u64, f851(v1))
        }
    } else {
        let a3089 = v0;
        let a3090 = 127u64;
        if nat_eq(a3089, a3090) {
            Adt154::C0(0u64, f851(v1))
        } else {
            let a3091 = v0;
            let a3092 = 194u64;
            if nat_eq(a3091, a3092) {
                Adt154::C0(1u64, v1)
            } else {
                let a3093 = v0;
                let a3094 = 226u64;
                if nat_eq(a3093, a3094) {
                    Adt154::C0(2u64, v1)
                } else {
                    Adt154::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f851(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f852(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt129, Adt132> {
    if v2 {
        Ok::<Adt129, Adt132>(f747(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt129, Adt132>(Adt132::C4(Adt84::C8(f869(f610(v1)))))
    }
}

pub fn f853(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a3095 = f870(v1.clone());
    if if if bool_not(a3095) {
        true
    } else {
        f250(match v0.clone() {
            Adt17::C0(_, _, _, h3096, _) => {
                h3096
            }
        })
    } {
        let a3097 = f871(v1.clone());
        if bool_not(a3097) {
            true
        } else {
            f250(match v0.clone() {
                Adt17::C0(_, _, h3098, _, _) => {
                    h3098
                }
            })
        }
    } else {
        false
    } {
        let a3099 = f872(v1);
        if bool_not(a3099) {
            true
        } else {
            f250(match v0 {
                Adt17::C0(_, h3100, _, _, _) => {
                    h3100
                }
            })
        }
    } else {
        false
    }
}

pub fn f854(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt129, Adt132> {
    if v2 {
        Ok::<Adt129, Adt132>(f747(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt129, Adt132>(Adt132::C4(Adt84::C8(f873())))
    }
}

pub fn f855(v0: Adt17) -> bool {
    f635(match v0 {
        Adt17::C0(_, _, _, _, h3101) => {
            h3101
        }
    }, f636())
}

pub fn f856(v0: Adt16, v1: List<Adt50>) -> List<Adt131> {
    let m3103 = match v0 {
        Adt16::C0(_, _, _, _, h3102, _) => {
            h3102
        }
    };
    match m3103 {
        None => {
            List::<Adt131>::nil()
        }
        Some(v2) => {
            f874(f875(v2, v1, List::<Adt131>::nil()), List::<Adt131>::nil())
        }
    }
}

pub fn f857(v0: Bytes, v1: Adt139, v2: Str, v3: Adt32) -> R<Result<Adt32, Adt84>> {
    let m3104 = v3.clone();
    match m3104 {
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
            f876(v0, v1, v2, v4)
        }
        Adt32::C13(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f858(v0: Adt14) -> Str {
    let m3105 = v0;
    match m3105 {
        Adt14::C0(_) => {
            Str::lit("")
        }
        Adt14::C1(_) => {
            Str::lit("")
        }
        Adt14::C2(v1) => {
            match v1 {
                Adt21::C0(_, h3106, _, _, _) => {
                    h3106
                }
            }
        }
        Adt14::C3(_) => {
            Str::lit("")
        }
        Adt14::C4(v2) => {
            match v2 {
                Adt22::C0(_, h3107, _, _) => {
                    h3107
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
                Adt23::C0(_, h3108, _, _, _) => {
                    h3108
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

pub fn f859(v0: u64, v1: u64) -> bool {
    let a3109 = v1;
    let a3110 = v0;
    nat_lt(a3109, a3110)
}

pub fn f860(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt84>> {
    Ok(f877(f878(v0, v1, v2, v3)?, f879(v4)))
}

pub fn f861(p3116: List<Str>, p3117: Adt5) -> Adt107 {
    let m3115 = (p3116, p3117);
    {
        let mut m3115 = m3115;
        loop {
            let step = {
                let (v0, v1) = m3115;
                let m3111 = v0;
                match m3111.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt107>(Adt107::C0)
                    }
                    Some((v2, v3)) => {
                        let m3112 = f186(v2, v1);
                        match m3112 {
                            Adt107::C0 => {
                                Err::<(List<Str>, Adt5), Adt107>(Adt107::C0)
                            }
                            Adt107::C1(v4) => {
                                let m3113 = v3.clone();
                                match m3113.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt107>(Adt107::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3114 = f187(v4);
                                        match m3114 {
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
                Ok(next) => { m3115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f862(v0: Adt115) -> Option<Adt113> {
    let m3119 = match v0.clone() {
        Adt115::C0(_, _, _, h3118) => {
            h3118
        }
    };
    match m3119 {
        Adt116::C0 => {
            None::<Adt113>
        }
        Adt116::C1 => {
            Some(Adt113::C10(match v0 {
                Adt115::C0(h3120, _, _, _) => {
                    h3120
                }
            }))
        }
        Adt116::C2(_) => {
            None::<Adt113>
        }
    }
}

pub fn f863(v0: Adt6) -> bool {
    let m3121 = v0;
    match m3121 {
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

pub fn f864(v0: Adt115, v1: Adt6) -> R<Option<Adt113>> {
    let m3123 = match v0.clone() {
        Adt115::C0(_, _, _, h3122) => {
            h3122
        }
    };
    match m3123 {
        Adt116::C0 => {
            Ok(Some(Adt113::C8(match v0 {
                Adt115::C0(h3124, _, _, _) => {
                    h3124
                }
            })))
        }
        Adt116::C1 => {
            Ok(Some(Adt113::C10(match v0 {
                Adt115::C0(h3125, _, _, _) => {
                    h3125
                }
            })))
        }
        Adt116::C2(v2) => {
            if f880(match v0.clone() {
                Adt115::C0(_, _, h3126, _) => {
                    h3126
                }
            }, v2, v1)? {
                Ok(None::<Adt113>)
            } else {
                Ok(Some(Adt113::C9(match v0 {
                    Adt115::C0(h3127, _, _, _) => {
                        h3127
                    }
                })))
            }
        }
    }
}

pub fn f865(p3130: List<Adt41>, p3131: List<Adt6>) -> List<Adt6> {
    let m3129 = (p3130, p3131);
    {
        let mut m3129 = m3129;
        loop {
            let step = {
                let (v0, v1) = m3129;
                let m3128 = v0;
                match m3128.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f788(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f866(v0: Adt71) -> R<Option<Adt6>> {
    let m3132 = v0;
    match m3132 {
        Adt71::C0(v1) => {
            f881(v1)
        }
        Adt71::C1(v2) => {
            let m3133 = f882(v2, List::<Adt6>::nil())?;
            match m3133 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f470(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f867(v0: Adt3) -> Str {
    let m3134 = v0;
    match m3134 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f868(v0: u64) -> bool {
    let a3135 = v0;
    let a3136 = 9u64;
    if nat_eq(a3135, a3136) {
        true
    } else {
        let a3137 = v0;
        let a3138 = 10u64;
        if nat_eq(a3137, a3138) {
            true
        } else {
            let a3139 = v0;
            let a3140 = 13u64;
            nat_eq(a3139, a3140)
        }
    }
}

pub fn f869(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f872(v0.clone()) {
        Some(f26())
    } else {
        None::<Adt5>
    }, if f871(v0.clone()) {
        Some(f26())
    } else {
        None::<Adt5>
    }, if f870(v0) {
        Some(f26())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f870(p3145: List<(Str, Adt64)>) -> bool {
    let m3144 = p3145;
    {
        let mut m3144 = m3144;
        loop {
            let step = {
                let v0 = m3144;
                let m3141 = v0;
                match m3141.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3143 = {
                            let (_, h3142) = v1;
                            h3142
                        };
                        match m3143 {
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
                Ok(next) => { m3144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f871(p3150: List<(Str, Adt64)>) -> bool {
    let m3149 = p3150;
    {
        let mut m3149 = m3149;
        loop {
            let step = {
                let v0 = m3149;
                let m3146 = v0;
                match m3146.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3148 = {
                            let (_, h3147) = v1;
                            h3147
                        };
                        match m3148 {
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
                Ok(next) => { m3149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f872(p3155: List<(Str, Adt64)>) -> bool {
    let m3154 = p3155;
    {
        let mut m3154 = m3154;
        loop {
            let step = {
                let v0 = m3154;
                let m3151 = v0;
                match m3151.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3153 = {
                            let (_, h3152) = v1;
                            h3152
                        };
                        match m3153 {
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
                Ok(next) => { m3154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f873() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f636(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f874(p3158: List<Adt131>, p3159: List<Adt131>) -> List<Adt131> {
    let m3157 = (p3158, p3159);
    {
        let mut m3157 = m3157;
        loop {
            let step = {
                let (v0, v1) = m3157;
                let m3156 = v0;
                match m3156.uncons() {
                    None => {
                        Err::<(List<Adt131>, List<Adt131>), List<Adt131>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt131>, List<Adt131>), List<Adt131>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f875(v0: Adt19, p3162: List<Adt50>, p3163: List<Adt131>) -> List<Adt131> {
    let m3161 = (p3162, p3163);
    {
        let mut m3161 = m3161;
        loop {
            let step = {
                let (v1, v2) = m3161;
                let m3160 = v1;
                match m3160.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt131>), List<Adt131>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt131>), List<Adt131>>((v4, List::cons(Adt131::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f876(v0: Bytes, v1: Adt139, v2: Str, v3: Adt81) -> R<Result<Adt32, Adt84>> {
    if f825(v1.clone()) {
        if f883(v3.clone()) {
            let m3164 = f884(v0, f632(v1), v2, v3)?;
            match m3164 {
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
        Ok(Err::<Adt32, Adt84>(Adt84::C12(f632(v1))))
    }
}

pub fn f877(v0: Result<Option<Str>, Adt84>, v1: bool) -> Result<Option<Str>, Adt84> {
    let m3165 = v0;
    match m3165 {
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

pub fn f878(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt84>> {
    let m3166 = v3;
    match m3166 {
        None => {
            Ok(Ok::<Option<Str>, Adt84>(None::<Str>))
        }
        Some(v4) => {
            Ok(f885(f886(v0, v1, v2, v4)?))
        }
    }
}

pub fn f879(v0: Option<Adt5>) -> bool {
    let m3167 = v0;
    match m3167 {
        None => {
            true
        }
        Some(v1) => {
            f887(match v1 {
                Adt5::C0(h3168) => {
                    h3168
                }
            })
        }
    }
}

pub fn f880(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3169 = v0;
    match m3169 {
        Adt13::C0 => {
            let m3170 = f266(v2);
            match m3170 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3171 = v1;
                    let a3172 = v3;
                    Ok(equal(a3171, a3172))
                }
            }
        }
        Adt13::C1 => {
            f888(v1, v2)
        }
        Adt13::C2 => {
            let m3173 = f317(v2);
            match m3173 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3174 = v1;
                        let a3175 = Str::lit("true");
                        Ok(equal(a3174, a3175))
                    } else {
                        let a3176 = v1;
                        let a3177 = Str::lit("false");
                        Ok(equal(a3176, a3177))
                    }
                }
            }
        }
    }
}

pub fn f881(v0: Adt72) -> R<Option<Adt6>> {
    let m3178 = v0;
    match m3178 {
        Adt72::C0(v1) => {
            Ok(Some(f781(v1)))
        }
        Adt72::C1(v2) => {
            Ok(Some(f782(Str::lit("image"), match v2.clone() {
                Adt42::C0(h3179, _, _, _) => {
                    h3179
                }
            }, match v2.clone() {
                Adt42::C0(_, h3180, _, _) => {
                    h3180
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h3181, _) => {
                    h3181
                }
            }, match v2 {
                Adt42::C0(_, _, _, h3182) => {
                    h3182
                }
            })))
        }
        Adt72::C2(v3) => {
            Ok(Some(f782(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h3183, _, _, _) => {
                    h3183
                }
            }, match v3.clone() {
                Adt43::C0(_, h3184, _, _) => {
                    h3184
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h3185, _) => {
                    h3185
                }
            }, match v3 {
                Adt43::C0(_, _, _, h3186) => {
                    h3186
                }
            })))
        }
        Adt72::C3(v4) => {
            Ok(Some(f889(v4)))
        }
        Adt72::C4(v5) => {
            f890(v5)
        }
    }
}

pub fn f882(p3190: List<Adt72>, p3191: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3189 = (p3190, p3191);
    {
        let mut m3189 = m3189;
        loop {
            let step = {
                let (v0, v1) = m3189;
                let m3187 = v0;
                match m3187.uncons() {
                    None => {
                        Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3188 = f881(v2)?;
                        match m3188 {
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
                Ok(next) => { m3189 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f883(v0: Adt81) -> bool {
    let m3192 = v0;
    match m3192 {
        Adt81::C0(v1) => {
            let m3193 = v1;
            match m3193.uncons() {
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

pub fn f884(v0: Bytes, v1: Str, v2: Str, v3: Adt81) -> R<Result<Adt81, Adt84>> {
    let m3194 = v3.clone();
    match m3194 {
        Adt81::C0(_) => {
            Ok(Ok::<Adt81, Adt84>(v3))
        }
        Adt81::C1(v5) => {
            Ok(f891(f892(v0, v1, v2, v5)?, List::<(Str, Adt64)>::nil()))
        }
        Adt81::C2(v6, v7) => {
            Ok(f891(f892(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f885(v0: Option<Str>) -> Result<Option<Str>, Adt84> {
    let m3195 = v0;
    match m3195 {
        None => {
            Err::<Option<Str>, Adt84>(Adt84::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt84>(Some(v1))
        }
    }
}

pub fn f886(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3199 = f893(v0, {
        let a3196 = v1;
        utf8_encode(a3196)
    }, {
        let a3197 = v2;
        utf8_encode(a3197)
    }, {
        let a3198 = v3;
        utf8_encode(a3198)
    })?;
    match m3199 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3200 = v4;
            Ok(utf8_decode(a3200))
        }
    }
}

pub fn f887(p3205: List<(Str, Adt6)>) -> bool {
    let m3204 = p3205;
    {
        let mut m3204 = m3204;
        loop {
            let step = {
                let v0 = m3204;
                let m3201 = v0;
                match m3201.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3203 = f187({
                            let (_, h3202) = v1;
                            h3202
                        });
                        match m3203 {
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
                Ok(next) => { m3204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f888(v0: Str, v1: Adt6) -> R<bool> {
    let m3206 = f574(v1);
    match m3206 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3207 = f410(v2)?;
            match m3207 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3209 = f894(f206({
                        let a3208 = v0;
                        utf8_encode(a3208)
                    })?)?;
                    match m3209 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f556({
                                let a3210 = v3;
                                let a3211 = v4;
                                int_sub(a3210, a3211)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f889(v0: Adt73) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt73::C0(h3212, _, _, _) => {
            h3212
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt73::C0(_, h3213, _, _) => {
            h3213
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3216 = match v0.clone() {
            Adt73::C0(_, _, h3215, _) => {
                h3215
            }
        };
        match m3216 {
            Adt5::C0(h3214) => {
                h3214
            }
        }
    })), f780(match v0 {
        Adt73::C0(_, _, _, h3217) => {
            h3217
        }
    }))))))
}

pub fn f890(v0: Adt74) -> R<Option<Adt6>> {
    let m3219 = f600(match v0.clone() {
        Adt74::C0(_, h3218, _, _, _) => {
            h3218
        }
    })?;
    match m3219 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt74::C0(h3220, _, _, _, _) => {
                    h3220
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3226 = f601(Str::lit("structuredContent"), match v0.clone() {
                    Adt74::C0(_, _, h3221, _, _) => {
                        h3221
                    }
                });
                let a3227 = {
                    let a3224 = f602(Str::lit("isError"), match v0.clone() {
                        Adt74::C0(_, _, _, h3222, _) => {
                            h3222
                        }
                    });
                    let a3225 = f780(match v0 {
                        Adt74::C0(_, _, _, _, h3223) => {
                            h3223
                        }
                    });
                    append_list(a3224, a3225)
                };
                append_list(a3226, a3227)
            }))))))
        }
    }
}

pub fn f891(v0: Option<Str>, v1: List<(Str, Adt64)>) -> Result<Adt81, Adt84> {
    let m3228 = v0;
    match m3228 {
        None => {
            Err::<Adt81, Adt84>(Adt84::C14)
        }
        Some(v2) => {
            let m3229 = v1.clone();
            match m3229.uncons() {
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

pub fn f892(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3233 = f895(v0, {
        let a3230 = v1;
        utf8_encode(a3230)
    }, {
        let a3231 = v2;
        utf8_encode(a3231)
    }, {
        let a3232 = v3;
        utf8_encode(a3232)
    })?;
    match m3233 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3234 = v4;
            Ok(utf8_decode(a3234))
        }
    }
}

pub fn f893(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3236 = f896();
    let a3237 = {
        let a3235 = v0.clone();
        length_bytes(a3235)
    };
    if nat_le(a3236, a3237) {
        let a3239 = f897(v3.clone())?;
        let a3240 = {
            let a3238 = v3.clone();
            length_bytes(a3238)
        };
        if nat_eq(a3239, a3240) {
            Ok(None::<Bytes>)
        } else {
            f898(v0, v1, v2, v3.clone(), f897(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f894(v0: List<u64>) -> R<Option<i64>> {
    let m3241 = v0.clone();
    match m3241.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3242 = v1;
            let a3243 = 45u64;
            if nat_eq(a3242, a3243) {
                let m3244 = v2.clone();
                match m3244.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3245 = f899(v2, 0i64)?;
                        match m3245 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3246 = v5;
                                    int_neg(a3246)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f899(v0, 0i64)
            }
        }
    }
}

pub fn f895(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3248 = f896();
    let a3249 = {
        let a3247 = v0.clone();
        length_bytes(a3247)
    };
    if nat_le(a3248, a3249) {
        Ok(Some({
            let a3252 = f900(v3.clone())?;
            let a3253 = {
                let a3250 = Bytes::lit(&[0x2e]);
                let a3251 = f900(f901(v0, v1, v2, v3)?)?;
                append_bytes(a3250, a3251)
            };
            append_bytes(a3252, a3253)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f896() -> u64 {
    32u64
}

pub fn f897(v0: Bytes) -> R<u64> {
    let (_, h3254) = f902(Fn7::F903(v0.clone()), {
        let a3255 = v0.clone();
        length_bytes(a3255)
    }, (0u64, {
        let a3256 = v0;
        length_bytes(a3256)
    }))?;
    Ok(h3254)
}

pub fn f898(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3260 = {
        let a3257 = v3.clone();
        let a3258 = 0u64;
        let a3259 = v4;
        slice_bytes(a3257, a3258, a3259)
    };
    match m3260 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3271 = {
                let a3268 = v3.clone();
                let a3269 = {
                    let a3261 = v4;
                    let a3262 = 1u64;
                    nat_add(a3261, a3262)?
                };
                let a3270 = {
                    let a3266 = {
                        let a3263 = v3;
                        length_bytes(a3263)
                    };
                    let a3267 = {
                        let a3264 = v4;
                        let a3265 = 1u64;
                        nat_add(a3264, a3265)?
                    };
                    nat_sub(a3266, a3267)
                };
                slice_bytes(a3268, a3269, a3270)
            };
            match m3271 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f904(v0, v1, v2, f905(v5)?, f905(v6)?)
                }
            }
        }
    }
}

pub fn f899(p3277: List<u64>, p3278: i64) -> R<Option<i64>> {
    let m3276 = (p3277, p3278);
    {
        let mut m3276 = m3276;
        loop {
            let step = {
                let (v0, v1) = m3276;
                let m3272 = v0;
                match m3272.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3273 = f906(v2);
                        match m3273 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f316({
                                    let a3274 = v1;
                                    let a3275 = 10i64;
                                    int_mul(a3274, a3275)?
                                }, f469(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f900(v0: Bytes) -> R<Bytes> {
    let (_, h3279) = f907(Fn8::F908(v0.clone()), {
        let a3280 = v0;
        length_bytes(a3280)
    }, (0u64, f320()))?;
    Ok(h3279)
}

pub fn f901(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f909(v0, {
        let a3281 = f910(v1, v2);
        let a3282 = v3;
        append_bytes(a3281, a3282)
    })
}

pub fn f902(v0: Fn7, p3286: u64, p3287: (u64, u64)) -> R<(u64, u64)> {
    let m3285 = (p3286, p3287);
    {
        let mut m3285 = m3285;
        loop {
            let step = {
                let (v1, v2) = m3285;
                let m3283 = v1;
                if m3283 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3283 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3284: Fn7 = v0.clone();
                        c3284.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f903(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3289 = {
            let (h3288, _) = v1;
            h3288
        };
        let a3290 = 1u64;
        nat_add(a3289, a3290)?
    }, {
        let a3293 = {
            let (_, h3291) = v1;
            h3291
        };
        let a3294 = {
            let a3292 = v0.clone();
            length_bytes(a3292)
        };
        if nat_eq(a3293, a3294) {
            let a3296 = f120(v0, {
                let (h3295, _) = v1;
                h3295
            })?;
            let a3297 = 46u64;
            if nat_eq(a3296, a3297) {
                let (h3298, _) = v1;
                h3298
            } else {
                let (_, h3299) = v1;
                h3299
            }
        } else {
            let (_, h3300) = v1;
            h3300
        }
    }))
}

pub fn f904(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3301 = v3;
    match m3301 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3302 = v4;
            match m3302 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f911(f901(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f905(v0: Bytes) -> R<Option<Bytes>> {
    let a3307 = {
        let a3304 = {
            let a3303 = v0.clone();
            length_bytes(a3303)
        };
        let a3305 = 2u64;
        let a3306 = 0u64;
        nat_rem(a3304, a3305, a3306)
    };
    let a3308 = 0u64;
    if nat_eq(a3307, a3308) {
        let v1: (u64, (Bytes, u64)) = f912(Fn9::F913(v0.clone()), {
            let a3310 = {
                let a3309 = v0;
                length_bytes(a3309)
            };
            let a3311 = 2u64;
            let a3312 = 0u64;
            nat_quot(a3310, a3311, a3312)
        }, (0u64, (f320(), 0u64)))?;
        let a3315 = {
            let (_, h3313) = {
                let (_, h3314) = v1.clone();
                h3314
            };
            h3313
        };
        let a3316 = 0u64;
        if nat_eq(a3315, a3316) {
            Ok(Some({
                let (h3317, _) = {
                    let (_, h3318) = v1;
                    h3318
                };
                h3317
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f906(v0: u64) -> Option<u64> {
    let a3319 = v0;
    let a3320 = 48u64;
    if if nat_lt(a3319, a3320) {
        true
    } else {
        let a3321 = 57u64;
        let a3322 = v0;
        nat_lt(a3321, a3322)
    } {
        None::<u64>
    } else {
        Some({
            let a3323 = v0;
            let a3324 = 48u64;
            nat_sub(a3323, a3324)
        })
    }
}

pub fn f907(v0: Fn8, p3328: u64, p3329: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3327 = (p3328, p3329);
    {
        let mut m3327 = m3327;
        loop {
            let step = {
                let (v1, v2) = m3327;
                let m3325 = v1;
                if m3325 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3325 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3326: Fn8 = v0.clone();
                        c3326.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f908(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3331 = {
            let (h3330, _) = v1.clone();
            h3330
        };
        let a3332 = 1u64;
        nat_add(a3331, a3332)?
    }, {
        let a3335 = {
            let (_, h3333) = v1;
            h3333
        };
        let a3336 = f914(f120(v0, {
            let (h3334, _) = v1;
            h3334
        })?)?;
        append_bytes(a3335, a3336)
    }))
}

pub fn f909(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f915(f916(v0, v1)?)
}

pub fn f910(v0: Bytes, v1: Bytes) -> Bytes {
    let a3343 = f917({
        let a3337 = v0.clone();
        length_bytes(a3337)
    });
    let a3344 = {
        let a3341 = v0;
        let a3342 = {
            let a3339 = f917({
                let a3338 = v1.clone();
                length_bytes(a3338)
            });
            let a3340 = v1;
            append_bytes(a3339, a3340)
        };
        append_bytes(a3341, a3342)
    };
    append_bytes(a3343, a3344)
}

pub fn f911(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3347 = {
        let a3345 = v0.clone();
        length_bytes(a3345)
    };
    let a3348 = {
        let a3346 = v1.clone();
        length_bytes(a3346)
    };
    if nat_eq(a3347, a3348) {
        let a3349 = f918(v0, v1)?;
        let a3350 = 0u64;
        Ok(nat_eq(a3349, a3350))
    } else {
        Ok(false)
    }
}

pub fn f912(v0: Fn9, p3354: u64, p3355: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3353 = (p3354, p3355);
    {
        let mut m3353 = m3353;
        loop {
            let step = {
                let (v1, v2) = m3353;
                let m3351 = v1;
                if m3351 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3351 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3352: Fn9 = v0.clone();
                        c3352.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3353 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f913(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f919(v0, v1)
}

pub fn f914(v0: u64) -> R<Bytes> {
    let a3362 = f920({
        let a3356 = v0;
        let a3357 = 16u64;
        let a3358 = 0u64;
        nat_quot(a3356, a3357, a3358)
    })?;
    let a3363 = f920({
        let a3359 = v0;
        let a3360 = 16u64;
        let a3361 = 0u64;
        nat_rem(a3359, a3360, a3361)
    })?;
    Ok(append_bytes(a3362, a3363))
}

pub fn f915(v0: Adt156) -> R<Bytes> {
    let a3384 = f921(match v0.clone() {
        Adt156::C0(h3364, _, _, _, _, _, _, _) => {
            h3364
        }
    })?;
    let a3385 = {
        let a3382 = f921(match v0.clone() {
            Adt156::C0(_, h3365, _, _, _, _, _, _) => {
                h3365
            }
        })?;
        let a3383 = {
            let a3380 = f921(match v0.clone() {
                Adt156::C0(_, _, h3366, _, _, _, _, _) => {
                    h3366
                }
            })?;
            let a3381 = {
                let a3378 = f921(match v0.clone() {
                    Adt156::C0(_, _, _, h3367, _, _, _, _) => {
                        h3367
                    }
                })?;
                let a3379 = {
                    let a3376 = f921(match v0.clone() {
                        Adt156::C0(_, _, _, _, h3368, _, _, _) => {
                            h3368
                        }
                    })?;
                    let a3377 = {
                        let a3374 = f921(match v0.clone() {
                            Adt156::C0(_, _, _, _, _, h3369, _, _) => {
                                h3369
                            }
                        })?;
                        let a3375 = {
                            let a3372 = f921(match v0.clone() {
                                Adt156::C0(_, _, _, _, _, _, h3370, _) => {
                                    h3370
                                }
                            })?;
                            let a3373 = f921(match v0 {
                                Adt156::C0(_, _, _, _, _, _, _, h3371) => {
                                    h3371
                                }
                            })?;
                            append_bytes(a3372, a3373)
                        };
                        append_bytes(a3374, a3375)
                    };
                    append_bytes(a3376, a3377)
                };
                append_bytes(a3378, a3379)
            };
            append_bytes(a3380, a3381)
        };
        append_bytes(a3382, a3383)
    };
    Ok(append_bytes(a3384, a3385))
}

pub fn f916(v0: Bytes, v1: Bytes) -> R<Adt156> {
    let v2: Adt157 = f922(v0)?;
    let v3: Adt156 = f923(f924(f925(), f926(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f924(f924(f925(), f926(v2, 1549556828u32))?, f927(v3))
}

pub fn f917(v0: u64) -> Bytes {
    let a3411 = f440({
        let a3389 = {
            let a3386 = v0;
            let a3387 = 16777216u64;
            let a3388 = 0u64;
            nat_quot(a3386, a3387, a3388)
        };
        let a3390 = 256u64;
        let a3391 = 0u64;
        nat_rem(a3389, a3390, a3391)
    });
    let a3412 = {
        let a3409 = f440({
            let a3395 = {
                let a3392 = v0;
                let a3393 = 65536u64;
                let a3394 = 0u64;
                nat_quot(a3392, a3393, a3394)
            };
            let a3396 = 256u64;
            let a3397 = 0u64;
            nat_rem(a3395, a3396, a3397)
        });
        let a3410 = {
            let a3407 = f440({
                let a3401 = {
                    let a3398 = v0;
                    let a3399 = 256u64;
                    let a3400 = 0u64;
                    nat_quot(a3398, a3399, a3400)
                };
                let a3402 = 256u64;
                let a3403 = 0u64;
                nat_rem(a3401, a3402, a3403)
            });
            let a3408 = f440({
                let a3404 = v0;
                let a3405 = 256u64;
                let a3406 = 0u64;
                nat_rem(a3404, a3405, a3406)
            });
            append_bytes(a3407, a3408)
        };
        append_bytes(a3409, a3410)
    };
    append_bytes(a3411, a3412)
}

pub fn f918(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3413) = f902(Fn7::F928(v0.clone(), v1), {
        let a3414 = v0;
        length_bytes(a3414)
    }, (0u64, 0u64))?;
    Ok(h3413)
}

pub fn f919(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f929(f120(v0.clone(), {
        let (h3415, _) = v1.clone();
        h3415
    })?);
    let v3: u64 = f929(f120(v0, {
        let a3417 = {
            let (h3416, _) = v1.clone();
            h3416
        };
        let a3418 = 1u64;
        nat_add(a3417, a3418)?
    })?);
    let a3419 = v2;
    let a3420 = 16u64;
    if if nat_lt(a3419, a3420) {
        let a3421 = v3;
        let a3422 = 16u64;
        nat_lt(a3421, a3422)
    } else {
        false
    } {
        Ok(({
            let a3424 = {
                let (h3423, _) = v1.clone();
                h3423
            };
            let a3425 = 2u64;
            nat_add(a3424, a3425)?
        }, ({
            let a3432 = {
                let (h3426, _) = {
                    let (_, h3427) = v1.clone();
                    h3427
                };
                h3426
            };
            let a3433 = f440({
                let a3430 = {
                    let a3428 = v2;
                    let a3429 = 16u64;
                    nat_mul(a3428, a3429)?
                };
                let a3431 = v3;
                nat_add(a3430, a3431)?
            });
            append_bytes(a3432, a3433)
        }, {
            let (_, h3434) = {
                let (_, h3435) = v1;
                h3435
            };
            h3434
        })))
    } else {
        Ok(({
            let a3437 = {
                let (h3436, _) = v1.clone();
                h3436
            };
            let a3438 = 2u64;
            nat_add(a3437, a3438)?
        }, ({
            let (h3439, _) = {
                let (_, h3440) = v1.clone();
                h3440
            };
            h3439
        }, {
            let a3443 = {
                let (_, h3441) = {
                    let (_, h3442) = v1;
                    h3442
                };
                h3441
            };
            let a3444 = 1u64;
            nat_add(a3443, a3444)?
        })))
    }
}

pub fn f920(v0: u64) -> R<Bytes> {
    Ok(f440({
        let a3445 = v0;
        let a3446 = 10u64;
        if nat_lt(a3445, a3446) {
            let a3447 = 48u64;
            let a3448 = v0;
            nat_add(a3447, a3448)?
        } else {
            let a3449 = 87u64;
            let a3450 = v0;
            nat_add(a3449, a3450)?
        }
    }))
}

pub fn f921(v0: u32) -> R<Bytes> {
    let a3455 = f930(v0, 24u32)?;
    let a3456 = {
        let a3453 = f930(v0, 16u32)?;
        let a3454 = {
            let a3451 = f930(v0, 8u32)?;
            let a3452 = f930(v0, 0u32)?;
            append_bytes(a3451, a3452)
        };
        append_bytes(a3453, a3454)
    };
    Ok(append_bytes(a3455, a3456))
}

pub fn f922(v0: Bytes) -> R<Adt157> {
    let a3458 = f931();
    let a3459 = {
        let a3457 = v0.clone();
        length_bytes(a3457)
    };
    if nat_lt(a3458, a3459) {
        Ok(f932(f933(v0)?))
    } else {
        f934(v0)
    }
}

pub fn f923(v0: Adt156, v1: u64, v2: Bytes) -> R<Adt156> {
    let v3: u64 = {
        let a3460 = v2.clone();
        length_bytes(a3460)
    };
    let v4: u64 = {
        let a3466 = {
            let a3463 = {
                let a3461 = v3;
                let a3462 = 72u64;
                nat_add(a3461, a3462)?
            };
            let a3464 = 64u64;
            let a3465 = 0u64;
            nat_quot(a3463, a3464, a3465)
        };
        let a3467 = 64u64;
        nat_mul(a3466, a3467)?
    };
    let (_, h3468) = f935(Fn10::F936(v2, v1, v3, v4), {
        let a3469 = v4;
        let a3470 = 64u64;
        let a3471 = 0u64;
        nat_quot(a3469, a3470, a3471)
    }, (0u64, v0))?;
    Ok(h3468)
}

pub fn f924(v0: Adt156, v1: Adt157) -> R<Adt156> {
    let v2: Adt158 = f937(Fn11::F938, 64u64, Adt158::C0(match v0.clone() {
        Adt156::C0(h3472, _, _, _, _, _, _, _) => {
            h3472
        }
    }, match v0.clone() {
        Adt156::C0(_, h3473, _, _, _, _, _, _) => {
            h3473
        }
    }, match v0.clone() {
        Adt156::C0(_, _, h3474, _, _, _, _, _) => {
            h3474
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, h3475, _, _, _, _) => {
            h3475
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, h3476, _, _, _) => {
            h3476
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, h3477, _, _) => {
            h3477
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, h3478, _) => {
            h3478
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, _, h3479) => {
            h3479
        }
    }, v1, 0u64))?;
    Ok(Adt156::C0(f939(match v0.clone() {
        Adt156::C0(h3480, _, _, _, _, _, _, _) => {
            h3480
        }
    }, match v2.clone() {
        Adt158::C0(h3481, _, _, _, _, _, _, _, _, _) => {
            h3481
        }
    }), f939(match v0.clone() {
        Adt156::C0(_, h3482, _, _, _, _, _, _) => {
            h3482
        }
    }, match v2.clone() {
        Adt158::C0(_, h3483, _, _, _, _, _, _, _, _) => {
            h3483
        }
    }), f939(match v0.clone() {
        Adt156::C0(_, _, h3484, _, _, _, _, _) => {
            h3484
        }
    }, match v2.clone() {
        Adt158::C0(_, _, h3485, _, _, _, _, _, _, _) => {
            h3485
        }
    }), f939(match v0.clone() {
        Adt156::C0(_, _, _, h3486, _, _, _, _) => {
            h3486
        }
    }, match v2.clone() {
        Adt158::C0(_, _, _, h3487, _, _, _, _, _, _) => {
            h3487
        }
    }), f939(match v0.clone() {
        Adt156::C0(_, _, _, _, h3488, _, _, _) => {
            h3488
        }
    }, match v2.clone() {
        Adt158::C0(_, _, _, _, h3489, _, _, _, _, _) => {
            h3489
        }
    }), f939(match v0.clone() {
        Adt156::C0(_, _, _, _, _, h3490, _, _) => {
            h3490
        }
    }, match v2.clone() {
        Adt158::C0(_, _, _, _, _, h3491, _, _, _, _) => {
            h3491
        }
    }), f939(match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, h3492, _) => {
            h3492
        }
    }, match v2.clone() {
        Adt158::C0(_, _, _, _, _, _, h3493, _, _, _) => {
            h3493
        }
    }), f939(match v0 {
        Adt156::C0(_, _, _, _, _, _, _, h3494) => {
            h3494
        }
    }, match v2 {
        Adt158::C0(_, _, _, _, _, _, _, h3495, _, _) => {
            h3495
        }
    })))
}

pub fn f925() -> Adt156 {
    Adt156::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f926(v0: Adt157, v1: u32) -> Adt157 {
    Adt157::C0({
        let a3497 = match v0.clone() {
            Adt157::C0(h3496, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3496
            }
        };
        let a3498 = v1;
        fixed_u32::bit_xor(a3497, a3498)
    }, {
        let a3500 = match v0.clone() {
            Adt157::C0(_, h3499, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3499
            }
        };
        let a3501 = v1;
        fixed_u32::bit_xor(a3500, a3501)
    }, {
        let a3503 = match v0.clone() {
            Adt157::C0(_, _, h3502, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3502
            }
        };
        let a3504 = v1;
        fixed_u32::bit_xor(a3503, a3504)
    }, {
        let a3506 = match v0.clone() {
            Adt157::C0(_, _, _, h3505, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3505
            }
        };
        let a3507 = v1;
        fixed_u32::bit_xor(a3506, a3507)
    }, {
        let a3509 = match v0.clone() {
            Adt157::C0(_, _, _, _, h3508, _, _, _, _, _, _, _, _, _, _, _) => {
                h3508
            }
        };
        let a3510 = v1;
        fixed_u32::bit_xor(a3509, a3510)
    }, {
        let a3512 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, h3511, _, _, _, _, _, _, _, _, _, _) => {
                h3511
            }
        };
        let a3513 = v1;
        fixed_u32::bit_xor(a3512, a3513)
    }, {
        let a3515 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, h3514, _, _, _, _, _, _, _, _, _) => {
                h3514
            }
        };
        let a3516 = v1;
        fixed_u32::bit_xor(a3515, a3516)
    }, {
        let a3518 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, h3517, _, _, _, _, _, _, _, _) => {
                h3517
            }
        };
        let a3519 = v1;
        fixed_u32::bit_xor(a3518, a3519)
    }, {
        let a3521 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, _, h3520, _, _, _, _, _, _, _) => {
                h3520
            }
        };
        let a3522 = v1;
        fixed_u32::bit_xor(a3521, a3522)
    }, {
        let a3524 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, _, _, h3523, _, _, _, _, _, _) => {
                h3523
            }
        };
        let a3525 = v1;
        fixed_u32::bit_xor(a3524, a3525)
    }, {
        let a3527 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, _, _, _, h3526, _, _, _, _, _) => {
                h3526
            }
        };
        let a3528 = v1;
        fixed_u32::bit_xor(a3527, a3528)
    }, {
        let a3530 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, h3529, _, _, _, _) => {
                h3529
            }
        };
        let a3531 = v1;
        fixed_u32::bit_xor(a3530, a3531)
    }, {
        let a3533 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3532, _, _, _) => {
                h3532
            }
        };
        let a3534 = v1;
        fixed_u32::bit_xor(a3533, a3534)
    }, {
        let a3536 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3535, _, _) => {
                h3535
            }
        };
        let a3537 = v1;
        fixed_u32::bit_xor(a3536, a3537)
    }, {
        let a3539 = match v0.clone() {
            Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3538, _) => {
                h3538
            }
        };
        let a3540 = v1;
        fixed_u32::bit_xor(a3539, a3540)
    }, {
        let a3542 = match v0 {
            Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3541) => {
                h3541
            }
        };
        let a3543 = v1;
        fixed_u32::bit_xor(a3542, a3543)
    })
}

pub fn f927(v0: Adt156) -> Adt157 {
    Adt157::C0(match v0.clone() {
        Adt156::C0(h3544, _, _, _, _, _, _, _) => {
            h3544
        }
    }, match v0.clone() {
        Adt156::C0(_, h3545, _, _, _, _, _, _) => {
            h3545
        }
    }, match v0.clone() {
        Adt156::C0(_, _, h3546, _, _, _, _, _) => {
            h3546
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, h3547, _, _, _, _) => {
            h3547
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, h3548, _, _, _) => {
            h3548
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, h3549, _, _) => {
            h3549
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, h3550, _) => {
            h3550
        }
    }, match v0 {
        Adt156::C0(_, _, _, _, _, _, _, h3551) => {
            h3551
        }
    }, 2147483648u32, f940(), f940(), f940(), f940(), f940(), f940(), 768u32)
}

pub fn f928(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3553 = {
            let (h3552, _) = v2;
            h3552
        };
        let a3554 = 1u64;
        nat_add(a3553, a3554)?
    }, {
        let a3606 = {
            let (_, h3555) = v2;
            h3555
        };
        let a3607 = {
            let v3: u8 = {
                let a3558 = f941(v0, {
                    let (h3556, _) = v2;
                    h3556
                });
                let a3559 = f941(v1, {
                    let (h3557, _) = v2;
                    h3557
                });
                fixed_u8::bit_xor(a3558, a3559)
            };
            let a3604 = {
                let a3580 = {
                    let a3568 = {
                        let a3562 = {
                            let a3560 = v3;
                            let a3561 = 128u8;
                            fixed_u8::bit_and(a3560, a3561)
                        };
                        let a3563 = 128u8;
                        if equal(a3562, a3563) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3569 = {
                        let a3566 = {
                            let a3564 = v3;
                            let a3565 = 64u8;
                            fixed_u8::bit_and(a3564, a3565)
                        };
                        let a3567 = 64u8;
                        if equal(a3566, a3567) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3568, a3569)?
                };
                let a3581 = {
                    let a3578 = {
                        let a3572 = {
                            let a3570 = v3;
                            let a3571 = 32u8;
                            fixed_u8::bit_and(a3570, a3571)
                        };
                        let a3573 = 32u8;
                        if equal(a3572, a3573) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3579 = {
                        let a3576 = {
                            let a3574 = v3;
                            let a3575 = 16u8;
                            fixed_u8::bit_and(a3574, a3575)
                        };
                        let a3577 = 16u8;
                        if equal(a3576, a3577) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3578, a3579)?
                };
                nat_add(a3580, a3581)?
            };
            let a3605 = {
                let a3602 = {
                    let a3590 = {
                        let a3584 = {
                            let a3582 = v3;
                            let a3583 = 8u8;
                            fixed_u8::bit_and(a3582, a3583)
                        };
                        let a3585 = 8u8;
                        if equal(a3584, a3585) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3591 = {
                        let a3588 = {
                            let a3586 = v3;
                            let a3587 = 4u8;
                            fixed_u8::bit_and(a3586, a3587)
                        };
                        let a3589 = 4u8;
                        if equal(a3588, a3589) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3590, a3591)?
                };
                let a3603 = {
                    let a3600 = {
                        let a3594 = {
                            let a3592 = v3;
                            let a3593 = 2u8;
                            fixed_u8::bit_and(a3592, a3593)
                        };
                        let a3595 = 2u8;
                        if equal(a3594, a3595) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3601 = {
                        let a3598 = {
                            let a3596 = v3;
                            let a3597 = 1u8;
                            fixed_u8::bit_and(a3596, a3597)
                        };
                        let a3599 = 1u8;
                        if equal(a3598, a3599) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3600, a3601)?
                };
                nat_add(a3602, a3603)?
            };
            nat_add(a3604, a3605)?
        };
        nat_add(a3606, a3607)?
    }))
}

pub fn f929(v0: u64) -> u64 {
    let a3608 = v0;
    let a3609 = 48u64;
    if nat_lt(a3608, a3609) {
        16u64
    } else {
        let a3610 = v0;
        let a3611 = 58u64;
        if nat_lt(a3610, a3611) {
            let a3612 = v0;
            let a3613 = 48u64;
            nat_sub(a3612, a3613)
        } else {
            let a3614 = v0;
            let a3615 = 97u64;
            if nat_lt(a3614, a3615) {
                16u64
            } else {
                let a3616 = v0;
                let a3617 = 103u64;
                if nat_lt(a3616, a3617) {
                    let a3618 = v0;
                    let a3619 = 87u64;
                    nat_sub(a3618, a3619)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f930(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f440({
        let v2: u32 = {
            let a3620 = f942(v0, v1);
            let a3621 = 255u32;
            fixed_u32::bit_and(a3620, a3621)
        };
        let a3810 = {
            let a3714 = {
                let a3666 = {
                    let a3642 = {
                        let a3630 = {
                            let a3624 = {
                                let a3622 = v2;
                                let a3623 = 2147483648u32;
                                fixed_u32::bit_and(a3622, a3623)
                            };
                            let a3625 = 2147483648u32;
                            if equal(a3624, a3625) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3631 = {
                            let a3628 = {
                                let a3626 = v2;
                                let a3627 = 1073741824u32;
                                fixed_u32::bit_and(a3626, a3627)
                            };
                            let a3629 = 1073741824u32;
                            if equal(a3628, a3629) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3630, a3631)?
                    };
                    let a3643 = {
                        let a3640 = {
                            let a3634 = {
                                let a3632 = v2;
                                let a3633 = 536870912u32;
                                fixed_u32::bit_and(a3632, a3633)
                            };
                            let a3635 = 536870912u32;
                            if equal(a3634, a3635) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3641 = {
                            let a3638 = {
                                let a3636 = v2;
                                let a3637 = 268435456u32;
                                fixed_u32::bit_and(a3636, a3637)
                            };
                            let a3639 = 268435456u32;
                            if equal(a3638, a3639) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3640, a3641)?
                    };
                    nat_add(a3642, a3643)?
                };
                let a3667 = {
                    let a3664 = {
                        let a3652 = {
                            let a3646 = {
                                let a3644 = v2;
                                let a3645 = 134217728u32;
                                fixed_u32::bit_and(a3644, a3645)
                            };
                            let a3647 = 134217728u32;
                            if equal(a3646, a3647) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3653 = {
                            let a3650 = {
                                let a3648 = v2;
                                let a3649 = 67108864u32;
                                fixed_u32::bit_and(a3648, a3649)
                            };
                            let a3651 = 67108864u32;
                            if equal(a3650, a3651) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3652, a3653)?
                    };
                    let a3665 = {
                        let a3662 = {
                            let a3656 = {
                                let a3654 = v2;
                                let a3655 = 33554432u32;
                                fixed_u32::bit_and(a3654, a3655)
                            };
                            let a3657 = 33554432u32;
                            if equal(a3656, a3657) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3663 = {
                            let a3660 = {
                                let a3658 = v2;
                                let a3659 = 16777216u32;
                                fixed_u32::bit_and(a3658, a3659)
                            };
                            let a3661 = 16777216u32;
                            if equal(a3660, a3661) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3662, a3663)?
                    };
                    nat_add(a3664, a3665)?
                };
                nat_add(a3666, a3667)?
            };
            let a3715 = {
                let a3712 = {
                    let a3688 = {
                        let a3676 = {
                            let a3670 = {
                                let a3668 = v2;
                                let a3669 = 8388608u32;
                                fixed_u32::bit_and(a3668, a3669)
                            };
                            let a3671 = 8388608u32;
                            if equal(a3670, a3671) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3677 = {
                            let a3674 = {
                                let a3672 = v2;
                                let a3673 = 4194304u32;
                                fixed_u32::bit_and(a3672, a3673)
                            };
                            let a3675 = 4194304u32;
                            if equal(a3674, a3675) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3676, a3677)?
                    };
                    let a3689 = {
                        let a3686 = {
                            let a3680 = {
                                let a3678 = v2;
                                let a3679 = 2097152u32;
                                fixed_u32::bit_and(a3678, a3679)
                            };
                            let a3681 = 2097152u32;
                            if equal(a3680, a3681) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3687 = {
                            let a3684 = {
                                let a3682 = v2;
                                let a3683 = 1048576u32;
                                fixed_u32::bit_and(a3682, a3683)
                            };
                            let a3685 = 1048576u32;
                            if equal(a3684, a3685) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3686, a3687)?
                    };
                    nat_add(a3688, a3689)?
                };
                let a3713 = {
                    let a3710 = {
                        let a3698 = {
                            let a3692 = {
                                let a3690 = v2;
                                let a3691 = 524288u32;
                                fixed_u32::bit_and(a3690, a3691)
                            };
                            let a3693 = 524288u32;
                            if equal(a3692, a3693) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3699 = {
                            let a3696 = {
                                let a3694 = v2;
                                let a3695 = 262144u32;
                                fixed_u32::bit_and(a3694, a3695)
                            };
                            let a3697 = 262144u32;
                            if equal(a3696, a3697) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3698, a3699)?
                    };
                    let a3711 = {
                        let a3708 = {
                            let a3702 = {
                                let a3700 = v2;
                                let a3701 = 131072u32;
                                fixed_u32::bit_and(a3700, a3701)
                            };
                            let a3703 = 131072u32;
                            if equal(a3702, a3703) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3709 = {
                            let a3706 = {
                                let a3704 = v2;
                                let a3705 = 65536u32;
                                fixed_u32::bit_and(a3704, a3705)
                            };
                            let a3707 = 65536u32;
                            if equal(a3706, a3707) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3708, a3709)?
                    };
                    nat_add(a3710, a3711)?
                };
                nat_add(a3712, a3713)?
            };
            nat_add(a3714, a3715)?
        };
        let a3811 = {
            let a3808 = {
                let a3760 = {
                    let a3736 = {
                        let a3724 = {
                            let a3718 = {
                                let a3716 = v2;
                                let a3717 = 32768u32;
                                fixed_u32::bit_and(a3716, a3717)
                            };
                            let a3719 = 32768u32;
                            if equal(a3718, a3719) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3725 = {
                            let a3722 = {
                                let a3720 = v2;
                                let a3721 = 16384u32;
                                fixed_u32::bit_and(a3720, a3721)
                            };
                            let a3723 = 16384u32;
                            if equal(a3722, a3723) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3724, a3725)?
                    };
                    let a3737 = {
                        let a3734 = {
                            let a3728 = {
                                let a3726 = v2;
                                let a3727 = 8192u32;
                                fixed_u32::bit_and(a3726, a3727)
                            };
                            let a3729 = 8192u32;
                            if equal(a3728, a3729) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3735 = {
                            let a3732 = {
                                let a3730 = v2;
                                let a3731 = 4096u32;
                                fixed_u32::bit_and(a3730, a3731)
                            };
                            let a3733 = 4096u32;
                            if equal(a3732, a3733) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3734, a3735)?
                    };
                    nat_add(a3736, a3737)?
                };
                let a3761 = {
                    let a3758 = {
                        let a3746 = {
                            let a3740 = {
                                let a3738 = v2;
                                let a3739 = 2048u32;
                                fixed_u32::bit_and(a3738, a3739)
                            };
                            let a3741 = 2048u32;
                            if equal(a3740, a3741) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3747 = {
                            let a3744 = {
                                let a3742 = v2;
                                let a3743 = 1024u32;
                                fixed_u32::bit_and(a3742, a3743)
                            };
                            let a3745 = 1024u32;
                            if equal(a3744, a3745) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3746, a3747)?
                    };
                    let a3759 = {
                        let a3756 = {
                            let a3750 = {
                                let a3748 = v2;
                                let a3749 = 512u32;
                                fixed_u32::bit_and(a3748, a3749)
                            };
                            let a3751 = 512u32;
                            if equal(a3750, a3751) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3757 = {
                            let a3754 = {
                                let a3752 = v2;
                                let a3753 = 256u32;
                                fixed_u32::bit_and(a3752, a3753)
                            };
                            let a3755 = 256u32;
                            if equal(a3754, a3755) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3756, a3757)?
                    };
                    nat_add(a3758, a3759)?
                };
                nat_add(a3760, a3761)?
            };
            let a3809 = {
                let a3806 = {
                    let a3782 = {
                        let a3770 = {
                            let a3764 = {
                                let a3762 = v2;
                                let a3763 = 128u32;
                                fixed_u32::bit_and(a3762, a3763)
                            };
                            let a3765 = 128u32;
                            if equal(a3764, a3765) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3771 = {
                            let a3768 = {
                                let a3766 = v2;
                                let a3767 = 64u32;
                                fixed_u32::bit_and(a3766, a3767)
                            };
                            let a3769 = 64u32;
                            if equal(a3768, a3769) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3770, a3771)?
                    };
                    let a3783 = {
                        let a3780 = {
                            let a3774 = {
                                let a3772 = v2;
                                let a3773 = 32u32;
                                fixed_u32::bit_and(a3772, a3773)
                            };
                            let a3775 = 32u32;
                            if equal(a3774, a3775) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3781 = {
                            let a3778 = {
                                let a3776 = v2;
                                let a3777 = 16u32;
                                fixed_u32::bit_and(a3776, a3777)
                            };
                            let a3779 = 16u32;
                            if equal(a3778, a3779) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3780, a3781)?
                    };
                    nat_add(a3782, a3783)?
                };
                let a3807 = {
                    let a3804 = {
                        let a3792 = {
                            let a3786 = {
                                let a3784 = v2;
                                let a3785 = 8u32;
                                fixed_u32::bit_and(a3784, a3785)
                            };
                            let a3787 = 8u32;
                            if equal(a3786, a3787) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3793 = {
                            let a3790 = {
                                let a3788 = v2;
                                let a3789 = 4u32;
                                fixed_u32::bit_and(a3788, a3789)
                            };
                            let a3791 = 4u32;
                            if equal(a3790, a3791) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3792, a3793)?
                    };
                    let a3805 = {
                        let a3802 = {
                            let a3796 = {
                                let a3794 = v2;
                                let a3795 = 2u32;
                                fixed_u32::bit_and(a3794, a3795)
                            };
                            let a3797 = 2u32;
                            if equal(a3796, a3797) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3803 = {
                            let a3800 = {
                                let a3798 = v2;
                                let a3799 = 1u32;
                                fixed_u32::bit_and(a3798, a3799)
                            };
                            let a3801 = 1u32;
                            if equal(a3800, a3801) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3802, a3803)?
                    };
                    nat_add(a3804, a3805)?
                };
                nat_add(a3806, a3807)?
            };
            nat_add(a3808, a3809)?
        };
        nat_add(a3810, a3811)?
    }))
}

pub fn f931() -> u64 {
    64u64
}

pub fn f932(v0: Adt156) -> Adt157 {
    Adt157::C0(match v0.clone() {
        Adt156::C0(h3812, _, _, _, _, _, _, _) => {
            h3812
        }
    }, match v0.clone() {
        Adt156::C0(_, h3813, _, _, _, _, _, _) => {
            h3813
        }
    }, match v0.clone() {
        Adt156::C0(_, _, h3814, _, _, _, _, _) => {
            h3814
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, h3815, _, _, _, _) => {
            h3815
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, h3816, _, _, _) => {
            h3816
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, h3817, _, _) => {
            h3817
        }
    }, match v0.clone() {
        Adt156::C0(_, _, _, _, _, _, h3818, _) => {
            h3818
        }
    }, match v0 {
        Adt156::C0(_, _, _, _, _, _, _, h3819) => {
            h3819
        }
    }, f940(), f940(), f940(), f940(), f940(), f940(), f940(), f940())
}

pub fn f933(v0: Bytes) -> R<Adt156> {
    let v1: u64 = {
        let a3820 = v0.clone();
        length_bytes(a3820)
    };
    let v2: u64 = {
        let a3826 = {
            let a3823 = {
                let a3821 = v1;
                let a3822 = 72u64;
                nat_add(a3821, a3822)?
            };
            let a3824 = 64u64;
            let a3825 = 0u64;
            nat_quot(a3823, a3824, a3825)
        };
        let a3827 = 64u64;
        nat_mul(a3826, a3827)?
    };
    let (_, h3828) = f935(Fn10::F943(v0, v1, v2), {
        let a3829 = v2;
        let a3830 = 64u64;
        let a3831 = 0u64;
        nat_quot(a3829, a3830, a3831)
    }, (0u64, f925()))?;
    Ok(h3828)
}

pub fn f934(v0: Bytes) -> R<Adt157> {
    Ok(Adt157::C0(f944(v0.clone(), 0u64)?, f944(v0.clone(), 4u64)?, f944(v0.clone(), 8u64)?, f944(v0.clone(), 12u64)?, f944(v0.clone(), 16u64)?, f944(v0.clone(), 20u64)?, f944(v0.clone(), 24u64)?, f944(v0.clone(), 28u64)?, f944(v0.clone(), 32u64)?, f944(v0.clone(), 36u64)?, f944(v0.clone(), 40u64)?, f944(v0.clone(), 44u64)?, f944(v0.clone(), 48u64)?, f944(v0.clone(), 52u64)?, f944(v0.clone(), 56u64)?, f944(v0, 60u64)?))
}

pub fn f935(v0: Fn10, p3835: u64, p3836: (u64, Adt156)) -> R<(u64, Adt156)> {
    let m3834 = (p3835, p3836);
    {
        let mut m3834 = m3834;
        loop {
            let step = {
                let (v1, v2) = m3834;
                let m3832 = v1;
                if m3832 == 0 {
                    Err::<(u64, (u64, Adt156)), R<(u64, Adt156)>>(Ok(v2))
                } else {
                    let v3 = m3832 - 1;
                    Ok::<(u64, (u64, Adt156)), R<(u64, Adt156)>>((v3, {
                        let c3833: Fn10 = v0.clone();
                        c3833.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3834 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f936(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt156)) -> R<(u64, Adt156)> {
    Ok(({
        let a3838 = {
            let (h3837, _) = v4.clone();
            h3837
        };
        let a3839 = 64u64;
        nat_add(a3838, a3839)?
    }, f924({
        let (_, h3840) = v4;
        h3840
    }, f945(v0, v2, v3, {
        let a3843 = {
            let a3841 = v1;
            let a3842 = v2;
            nat_add(a3841, a3842)?
        };
        let a3844 = 8u64;
        nat_mul(a3843, a3844)?
    }, {
        let (h3845, _) = v4;
        h3845
    })?)?))
}

pub fn f937(v0: Fn11, p3849: u64, p3850: Adt158) -> R<Adt158> {
    let m3848 = (p3849, p3850);
    {
        let mut m3848 = m3848;
        loop {
            let step = {
                let (v1, v2) = m3848;
                let m3846 = v1;
                if m3846 == 0 {
                    Err::<(u64, Adt158), R<Adt158>>(Ok(v2))
                } else {
                    let v3 = m3846 - 1;
                    Ok::<(u64, Adt158), R<Adt158>>((v3, {
                        let c3847: Fn11 = v0.clone();
                        c3847.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f938(v0: Adt158) -> R<Adt158> {
    f946(v0)
}

pub fn f939(v0: u32, v1: u32) -> u32 {
    let m3853 = {
        let a3851 = f947(v0);
        let a3852 = f947(v1);
        fixed_u64::checked_add(a3851, a3852)
    };
    match m3853 {
        None => {
            0u32
        }
        Some(v2) => {
            f948(v2)
        }
    }
}

pub fn f940() -> u32 {
    0u32
}

pub fn f941(v0: Bytes, v1: u64) -> u8 {
    let m3856 = {
        let a3854 = v0;
        let a3855 = v1;
        index_bytes(a3854, a3855)
    };
    match m3856 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f942(v0: u32, v1: u32) -> u32 {
    let m3859 = {
        let a3857 = v0;
        let a3858 = v1;
        fixed_u32::shift_right(a3857, a3858)
    };
    match m3859 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f943(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt156)) -> R<(u64, Adt156)> {
    Ok(({
        let a3861 = {
            let (h3860, _) = v3.clone();
            h3860
        };
        let a3862 = 64u64;
        nat_add(a3861, a3862)?
    }, f924({
        let (_, h3863) = v3;
        h3863
    }, f949(v0, v1, v2, {
        let (h3864, _) = v3;
        h3864
    })?)?))
}

pub fn f944(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f950(f951(v0.clone(), v1), f951(v0.clone(), {
        let a3865 = v1;
        let a3866 = 1u64;
        nat_add(a3865, a3866)?
    }), f951(v0.clone(), {
        let a3867 = v1;
        let a3868 = 2u64;
        nat_add(a3867, a3868)?
    }), f951(v0, {
        let a3869 = v1;
        let a3870 = 3u64;
        nat_add(a3869, a3870)?
    })))
}

pub fn f945(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt157> {
    Ok(Adt157::C0(f952(v0.clone(), v1, v2, v3, v4)?, f952(v0.clone(), v1, v2, v3, {
        let a3871 = v4;
        let a3872 = 4u64;
        nat_add(a3871, a3872)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3873 = v4;
        let a3874 = 8u64;
        nat_add(a3873, a3874)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3875 = v4;
        let a3876 = 12u64;
        nat_add(a3875, a3876)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3877 = v4;
        let a3878 = 16u64;
        nat_add(a3877, a3878)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3879 = v4;
        let a3880 = 20u64;
        nat_add(a3879, a3880)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3881 = v4;
        let a3882 = 24u64;
        nat_add(a3881, a3882)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3883 = v4;
        let a3884 = 28u64;
        nat_add(a3883, a3884)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3885 = v4;
        let a3886 = 32u64;
        nat_add(a3885, a3886)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3887 = v4;
        let a3888 = 36u64;
        nat_add(a3887, a3888)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3889 = v4;
        let a3890 = 40u64;
        nat_add(a3889, a3890)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3891 = v4;
        let a3892 = 44u64;
        nat_add(a3891, a3892)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3893 = v4;
        let a3894 = 48u64;
        nat_add(a3893, a3894)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3895 = v4;
        let a3896 = 52u64;
        nat_add(a3895, a3896)?
    })?, f952(v0.clone(), v1, v2, v3, {
        let a3897 = v4;
        let a3898 = 56u64;
        nat_add(a3897, a3898)?
    })?, f952(v0, v1, v2, v3, {
        let a3899 = v4;
        let a3900 = 60u64;
        nat_add(a3899, a3900)?
    })?))
}

pub fn f946(v0: Adt158) -> R<Adt158> {
    let v1: u32 = f953(match v0.clone() {
        Adt158::C0(_, _, _, _, _, _, _, _, _, h3901) => {
            h3901
        }
    });
    let v2: u32 = f939(match v0.clone() {
        Adt158::C0(_, _, _, _, _, _, _, h3902, _, _) => {
            h3902
        }
    }, f939(f954(match v0.clone() {
        Adt158::C0(_, _, _, _, h3903, _, _, _, _, _) => {
            h3903
        }
    }), f939(f955(match v0.clone() {
        Adt158::C0(_, _, _, _, h3904, _, _, _, _, _) => {
            h3904
        }
    }, match v0.clone() {
        Adt158::C0(_, _, _, _, _, h3905, _, _, _, _) => {
            h3905
        }
    }, match v0.clone() {
        Adt158::C0(_, _, _, _, _, _, h3906, _, _, _) => {
            h3906
        }
    }), f939(v1, {
        let m3909 = match v0.clone() {
            Adt158::C0(_, _, _, _, _, _, _, _, h3908, _) => {
                h3908
            }
        };
        match m3909 {
            Adt157::C0(h3907, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3907
            }
        }
    }))));
    let v3: u32 = f939(f956(match v0.clone() {
        Adt158::C0(h3910, _, _, _, _, _, _, _, _, _) => {
            h3910
        }
    }), f957(match v0.clone() {
        Adt158::C0(h3911, _, _, _, _, _, _, _, _, _) => {
            h3911
        }
    }, match v0.clone() {
        Adt158::C0(_, h3912, _, _, _, _, _, _, _, _) => {
            h3912
        }
    }, match v0.clone() {
        Adt158::C0(_, _, h3913, _, _, _, _, _, _, _) => {
            h3913
        }
    }));
    Ok(Adt158::C0(f939(v2, v3), match v0.clone() {
        Adt158::C0(h3914, _, _, _, _, _, _, _, _, _) => {
            h3914
        }
    }, match v0.clone() {
        Adt158::C0(_, h3915, _, _, _, _, _, _, _, _) => {
            h3915
        }
    }, match v0.clone() {
        Adt158::C0(_, _, h3916, _, _, _, _, _, _, _) => {
            h3916
        }
    }, f939(match v0.clone() {
        Adt158::C0(_, _, _, h3917, _, _, _, _, _, _) => {
            h3917
        }
    }, v2), match v0.clone() {
        Adt158::C0(_, _, _, _, h3918, _, _, _, _, _) => {
            h3918
        }
    }, match v0.clone() {
        Adt158::C0(_, _, _, _, _, h3919, _, _, _, _) => {
            h3919
        }
    }, match v0.clone() {
        Adt158::C0(_, _, _, _, _, _, h3920, _, _, _) => {
            h3920
        }
    }, f958(match v0.clone() {
        Adt158::C0(_, _, _, _, _, _, _, _, h3921, _) => {
            h3921
        }
    }), {
        let a3923 = match v0 {
            Adt158::C0(_, _, _, _, _, _, _, _, _, h3922) => {
                h3922
            }
        };
        let a3924 = 1u64;
        nat_add(a3923, a3924)?
    }))
}

pub fn f947(v0: u32) -> u64 {
    let m3926 = {
        let a3925 = v0;
        fixed_u64::convert(i128::from(a3925))
    };
    match m3926 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f948(v0: u64) -> u32 {
    let m3930 = {
        let a3929 = {
            let a3927 = v0;
            let a3928 = 4294967295u64;
            fixed_u64::bit_and(a3927, a3928)
        };
        fixed_u32::convert(i128::from(a3929))
    };
    match m3930 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f949(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt157> {
    Ok(Adt157::C0(f959(v0.clone(), v1, v2, v3)?, f959(v0.clone(), v1, v2, {
        let a3931 = v3;
        let a3932 = 4u64;
        nat_add(a3931, a3932)?
    })?, f959(v0.clone(), v1, v2, {
        let a3933 = v3;
        let a3934 = 8u64;
        nat_add(a3933, a3934)?
    })?, f959(v0.clone(), v1, v2, {
        let a3935 = v3;
        let a3936 = 12u64;
        nat_add(a3935, a3936)?
    })?, f959(v0.clone(), v1, v2, {
        let a3937 = v3;
        let a3938 = 16u64;
        nat_add(a3937, a3938)?
    })?, f959(v0.clone(), v1, v2, {
        let a3939 = v3;
        let a3940 = 20u64;
        nat_add(a3939, a3940)?
    })?, f959(v0.clone(), v1, v2, {
        let a3941 = v3;
        let a3942 = 24u64;
        nat_add(a3941, a3942)?
    })?, f959(v0.clone(), v1, v2, {
        let a3943 = v3;
        let a3944 = 28u64;
        nat_add(a3943, a3944)?
    })?, f959(v0.clone(), v1, v2, {
        let a3945 = v3;
        let a3946 = 32u64;
        nat_add(a3945, a3946)?
    })?, f959(v0.clone(), v1, v2, {
        let a3947 = v3;
        let a3948 = 36u64;
        nat_add(a3947, a3948)?
    })?, f959(v0.clone(), v1, v2, {
        let a3949 = v3;
        let a3950 = 40u64;
        nat_add(a3949, a3950)?
    })?, f959(v0.clone(), v1, v2, {
        let a3951 = v3;
        let a3952 = 44u64;
        nat_add(a3951, a3952)?
    })?, f959(v0.clone(), v1, v2, {
        let a3953 = v3;
        let a3954 = 48u64;
        nat_add(a3953, a3954)?
    })?, f959(v0.clone(), v1, v2, {
        let a3955 = v3;
        let a3956 = 52u64;
        nat_add(a3955, a3956)?
    })?, f959(v0.clone(), v1, v2, {
        let a3957 = v3;
        let a3958 = 56u64;
        nat_add(a3957, a3958)?
    })?, f959(v0, v1, v2, {
        let a3959 = v3;
        let a3960 = 60u64;
        nat_add(a3959, a3960)?
    })?))
}

pub fn f950(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3965 = f960(v0, 24u32);
    let a3966 = {
        let a3963 = f960(v1, 16u32);
        let a3964 = {
            let a3961 = f960(v2, 8u32);
            let a3962 = v3;
            fixed_u32::bit_or(a3961, a3962)
        };
        fixed_u32::bit_or(a3963, a3964)
    };
    fixed_u32::bit_or(a3965, a3966)
}

pub fn f951(v0: Bytes, v1: u64) -> u32 {
    let m3969 = {
        let a3967 = v0;
        let a3968 = v1;
        index_bytes(a3967, a3968)
    };
    match m3969 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3971 = {
                let a3970 = v2;
                fixed_u32::convert(i128::from(a3970))
            };
            match m3971 {
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

pub fn f952(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3974 = {
        let a3972 = v4;
        let a3973 = 8u64;
        nat_add(a3972, a3973)?
    };
    let a3975 = v2;
    if nat_eq(a3974, a3975) {
        Ok(f961(v3, true))
    } else {
        let a3978 = {
            let a3976 = v4;
            let a3977 = 4u64;
            nat_add(a3976, a3977)?
        };
        let a3979 = v2;
        if nat_eq(a3978, a3979) {
            Ok(f961(v3, false))
        } else {
            Ok(f950(f962(v0.clone(), v1, v4), f962(v0.clone(), v1, {
                let a3980 = v4;
                let a3981 = 1u64;
                nat_add(a3980, a3981)?
            }), f962(v0.clone(), v1, {
                let a3982 = v4;
                let a3983 = 2u64;
                nat_add(a3982, a3983)?
            }), f962(v0, v1, {
                let a3984 = v4;
                let a3985 = 3u64;
                nat_add(a3984, a3985)?
            })))
        }
    }
}

pub fn f953(v0: u64) -> u32 {
    let a3986 = v0;
    let a3987 = 32u64;
    if nat_lt(a3986, a3987) {
        let a3988 = v0;
        let a3989 = 16u64;
        if nat_lt(a3988, a3989) {
            let a3990 = v0;
            let a3991 = 8u64;
            if nat_lt(a3990, a3991) {
                let a3992 = v0;
                let a3993 = 4u64;
                if nat_lt(a3992, a3993) {
                    let a3994 = v0;
                    let a3995 = 2u64;
                    if nat_lt(a3994, a3995) {
                        let a3996 = v0;
                        let a3997 = 1u64;
                        if nat_lt(a3996, a3997) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3998 = v0;
                        let a3999 = 3u64;
                        if nat_lt(a3998, a3999) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4000 = v0;
                    let a4001 = 6u64;
                    if nat_lt(a4000, a4001) {
                        let a4002 = v0;
                        let a4003 = 5u64;
                        if nat_lt(a4002, a4003) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4004 = v0;
                        let a4005 = 7u64;
                        if nat_lt(a4004, a4005) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4006 = v0;
                let a4007 = 12u64;
                if nat_lt(a4006, a4007) {
                    let a4008 = v0;
                    let a4009 = 10u64;
                    if nat_lt(a4008, a4009) {
                        let a4010 = v0;
                        let a4011 = 9u64;
                        if nat_lt(a4010, a4011) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4012 = v0;
                        let a4013 = 11u64;
                        if nat_lt(a4012, a4013) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4014 = v0;
                    let a4015 = 14u64;
                    if nat_lt(a4014, a4015) {
                        let a4016 = v0;
                        let a4017 = 13u64;
                        if nat_lt(a4016, a4017) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4018 = v0;
                        let a4019 = 15u64;
                        if nat_lt(a4018, a4019) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4020 = v0;
            let a4021 = 24u64;
            if nat_lt(a4020, a4021) {
                let a4022 = v0;
                let a4023 = 20u64;
                if nat_lt(a4022, a4023) {
                    let a4024 = v0;
                    let a4025 = 18u64;
                    if nat_lt(a4024, a4025) {
                        let a4026 = v0;
                        let a4027 = 17u64;
                        if nat_lt(a4026, a4027) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4028 = v0;
                        let a4029 = 19u64;
                        if nat_lt(a4028, a4029) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4030 = v0;
                    let a4031 = 22u64;
                    if nat_lt(a4030, a4031) {
                        let a4032 = v0;
                        let a4033 = 21u64;
                        if nat_lt(a4032, a4033) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4034 = v0;
                        let a4035 = 23u64;
                        if nat_lt(a4034, a4035) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4036 = v0;
                let a4037 = 28u64;
                if nat_lt(a4036, a4037) {
                    let a4038 = v0;
                    let a4039 = 26u64;
                    if nat_lt(a4038, a4039) {
                        let a4040 = v0;
                        let a4041 = 25u64;
                        if nat_lt(a4040, a4041) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4042 = v0;
                        let a4043 = 27u64;
                        if nat_lt(a4042, a4043) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4044 = v0;
                    let a4045 = 30u64;
                    if nat_lt(a4044, a4045) {
                        let a4046 = v0;
                        let a4047 = 29u64;
                        if nat_lt(a4046, a4047) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4048 = v0;
                        let a4049 = 31u64;
                        if nat_lt(a4048, a4049) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4050 = v0;
        let a4051 = 48u64;
        if nat_lt(a4050, a4051) {
            let a4052 = v0;
            let a4053 = 40u64;
            if nat_lt(a4052, a4053) {
                let a4054 = v0;
                let a4055 = 36u64;
                if nat_lt(a4054, a4055) {
                    let a4056 = v0;
                    let a4057 = 34u64;
                    if nat_lt(a4056, a4057) {
                        let a4058 = v0;
                        let a4059 = 33u64;
                        if nat_lt(a4058, a4059) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4060 = v0;
                        let a4061 = 35u64;
                        if nat_lt(a4060, a4061) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4062 = v0;
                    let a4063 = 38u64;
                    if nat_lt(a4062, a4063) {
                        let a4064 = v0;
                        let a4065 = 37u64;
                        if nat_lt(a4064, a4065) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4066 = v0;
                        let a4067 = 39u64;
                        if nat_lt(a4066, a4067) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4068 = v0;
                let a4069 = 44u64;
                if nat_lt(a4068, a4069) {
                    let a4070 = v0;
                    let a4071 = 42u64;
                    if nat_lt(a4070, a4071) {
                        let a4072 = v0;
                        let a4073 = 41u64;
                        if nat_lt(a4072, a4073) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4074 = v0;
                        let a4075 = 43u64;
                        if nat_lt(a4074, a4075) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4076 = v0;
                    let a4077 = 46u64;
                    if nat_lt(a4076, a4077) {
                        let a4078 = v0;
                        let a4079 = 45u64;
                        if nat_lt(a4078, a4079) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4080 = v0;
                        let a4081 = 47u64;
                        if nat_lt(a4080, a4081) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4082 = v0;
            let a4083 = 56u64;
            if nat_lt(a4082, a4083) {
                let a4084 = v0;
                let a4085 = 52u64;
                if nat_lt(a4084, a4085) {
                    let a4086 = v0;
                    let a4087 = 50u64;
                    if nat_lt(a4086, a4087) {
                        let a4088 = v0;
                        let a4089 = 49u64;
                        if nat_lt(a4088, a4089) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4090 = v0;
                        let a4091 = 51u64;
                        if nat_lt(a4090, a4091) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4092 = v0;
                    let a4093 = 54u64;
                    if nat_lt(a4092, a4093) {
                        let a4094 = v0;
                        let a4095 = 53u64;
                        if nat_lt(a4094, a4095) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4096 = v0;
                        let a4097 = 55u64;
                        if nat_lt(a4096, a4097) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4098 = v0;
                let a4099 = 60u64;
                if nat_lt(a4098, a4099) {
                    let a4100 = v0;
                    let a4101 = 58u64;
                    if nat_lt(a4100, a4101) {
                        let a4102 = v0;
                        let a4103 = 57u64;
                        if nat_lt(a4102, a4103) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4104 = v0;
                        let a4105 = 59u64;
                        if nat_lt(a4104, a4105) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4106 = v0;
                    let a4107 = 62u64;
                    if nat_lt(a4106, a4107) {
                        let a4108 = v0;
                        let a4109 = 61u64;
                        if nat_lt(a4108, a4109) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4110 = v0;
                        let a4111 = 63u64;
                        if nat_lt(a4110, a4111) {
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

pub fn f954(v0: u32) -> u32 {
    let a4114 = {
        let a4112 = f963(v0, 6u32, 26u32);
        let a4113 = f963(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4112, a4113)
    };
    let a4115 = f963(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4114, a4115)
}

pub fn f955(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4121 = {
        let a4116 = v0;
        let a4117 = v1;
        fixed_u32::bit_and(a4116, a4117)
    };
    let a4122 = {
        let a4119 = {
            let a4118 = v0;
            fixed_u32::bit_not(a4118)
        };
        let a4120 = v2;
        fixed_u32::bit_and(a4119, a4120)
    };
    fixed_u32::bit_xor(a4121, a4122)
}

pub fn f956(v0: u32) -> u32 {
    let a4125 = {
        let a4123 = f963(v0, 2u32, 30u32);
        let a4124 = f963(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4123, a4124)
    };
    let a4126 = f963(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4125, a4126)
}

pub fn f957(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4135 = {
        let a4131 = {
            let a4127 = v0;
            let a4128 = v1;
            fixed_u32::bit_and(a4127, a4128)
        };
        let a4132 = {
            let a4129 = v0;
            let a4130 = v2;
            fixed_u32::bit_and(a4129, a4130)
        };
        fixed_u32::bit_xor(a4131, a4132)
    };
    let a4136 = {
        let a4133 = v1;
        let a4134 = v2;
        fixed_u32::bit_and(a4133, a4134)
    };
    fixed_u32::bit_xor(a4135, a4136)
}

pub fn f958(v0: Adt157) -> Adt157 {
    Adt157::C0(match v0.clone() {
        Adt157::C0(_, h4137, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4137
        }
    }, match v0.clone() {
        Adt157::C0(_, _, h4138, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4138
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, h4139, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4139
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, h4140, _, _, _, _, _, _, _, _, _, _, _) => {
            h4140
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, h4141, _, _, _, _, _, _, _, _, _, _) => {
            h4141
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, h4142, _, _, _, _, _, _, _, _, _) => {
            h4142
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, h4143, _, _, _, _, _, _, _, _) => {
            h4143
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, h4144, _, _, _, _, _, _, _) => {
            h4144
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, h4145, _, _, _, _, _, _) => {
            h4145
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, _, h4146, _, _, _, _, _) => {
            h4146
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, h4147, _, _, _, _) => {
            h4147
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4148, _, _, _) => {
            h4148
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4149, _, _) => {
            h4149
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4150, _) => {
            h4150
        }
    }, match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4151) => {
            h4151
        }
    }, f939(f964(match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4152, _) => {
            h4152
        }
    }), f939(match v0.clone() {
        Adt157::C0(_, _, _, _, _, _, _, _, _, h4153, _, _, _, _, _, _) => {
            h4153
        }
    }, f939(f965(match v0.clone() {
        Adt157::C0(_, h4154, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4154
        }
    }), match v0 {
        Adt157::C0(h4155, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4155
        }
    }))))
}

pub fn f959(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4158 = {
        let a4156 = v3;
        let a4157 = 8u64;
        nat_add(a4156, a4157)?
    };
    let a4159 = v2;
    if nat_eq(a4158, a4159) {
        Ok(f966({
            let a4162 = {
                let a4160 = v1;
                let a4161 = 8u64;
                nat_mul(a4160, a4161)?
            };
            let a4163 = 4294967296u64;
            let a4164 = 0u64;
            nat_quot(a4162, a4163, a4164)
        }))
    } else {
        let a4167 = {
            let a4165 = v3;
            let a4166 = 4u64;
            nat_add(a4165, a4166)?
        };
        let a4168 = v2;
        if nat_eq(a4167, a4168) {
            Ok(f966({
                let a4171 = {
                    let a4169 = v1;
                    let a4170 = 8u64;
                    nat_mul(a4169, a4170)?
                };
                let a4172 = 4294967296u64;
                let a4173 = 0u64;
                nat_rem(a4171, a4172, a4173)
            }))
        } else {
            Ok(f950(f962(v0.clone(), v1, v3), f962(v0.clone(), v1, {
                let a4174 = v3;
                let a4175 = 1u64;
                nat_add(a4174, a4175)?
            }), f962(v0.clone(), v1, {
                let a4176 = v3;
                let a4177 = 2u64;
                nat_add(a4176, a4177)?
            }), f962(v0, v1, {
                let a4178 = v3;
                let a4179 = 3u64;
                nat_add(a4178, a4179)?
            })))
        }
    }
}

pub fn f960(v0: u32, v1: u32) -> u32 {
    let m4182 = {
        let a4180 = v0;
        let a4181 = v1;
        fixed_u32::shift_left(a4180, a4181)
    };
    match m4182 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f961(v0: u64, v1: bool) -> u32 {
    if v1 {
        f966({
            let a4183 = v0;
            let a4184 = 4294967296u64;
            let a4185 = 0u64;
            nat_quot(a4183, a4184, a4185)
        })
    } else {
        f966({
            let a4186 = v0;
            let a4187 = 4294967296u64;
            let a4188 = 0u64;
            nat_rem(a4186, a4187, a4188)
        })
    }
}

pub fn f962(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4189 = v2;
    let a4190 = v1;
    if nat_lt(a4189, a4190) {
        let m4193 = {
            let a4191 = v0;
            let a4192 = v2;
            index_bytes(a4191, a4192)
        };
        match m4193 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4195 = {
                    let a4194 = v3;
                    fixed_u32::convert(i128::from(a4194))
                };
                match m4195 {
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
        let a4196 = v2;
        let a4197 = v1;
        if nat_eq(a4196, a4197) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f963(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4198 = f942(v0, v1);
    let a4199 = f960(v0, v2);
    fixed_u32::bit_or(a4198, a4199)
}

pub fn f964(v0: u32) -> u32 {
    let a4202 = {
        let a4200 = f963(v0, 17u32, 15u32);
        let a4201 = f963(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4200, a4201)
    };
    let a4203 = f942(v0, 10u32);
    fixed_u32::bit_xor(a4202, a4203)
}

pub fn f965(v0: u32) -> u32 {
    let a4206 = {
        let a4204 = f963(v0, 7u32, 25u32);
        let a4205 = f963(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4204, a4205)
    };
    let a4207 = f942(v0, 3u32);
    fixed_u32::bit_xor(a4206, a4207)
}

pub fn f966(v0: u64) -> u32 {
    let (_, h4208) = {
        let (_, h4209) = f967(Fn12::F968, 32u64, (v0, (1u32, 0u32)));
        h4209
    };
    h4208
}

pub fn f967(v0: Fn12, p4213: u64, p4214: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4212 = (p4213, p4214);
    {
        let mut m4212 = m4212;
        loop {
            let step = {
                let (v1, v2) = m4212;
                let m4210 = v1;
                if m4210 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4210 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4211: Fn12 = v0.clone();
                        c4211.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4212 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f968(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4215, _) = v0;
        h4215
    };
    let v2: u32 = {
        let (h4216, _) = {
            let (_, h4217) = v0;
            h4217
        };
        h4216
    };
    let v3: u32 = {
        let (_, h4218) = {
            let (_, h4219) = v0;
            h4219
        };
        h4218
    };
    ({
        let a4220 = v1;
        let a4221 = 2u64;
        let a4222 = 0u64;
        nat_quot(a4220, a4221, a4222)
    }, (f960(v2, 1u32), {
        let a4226 = {
            let a4223 = v1;
            let a4224 = 2u64;
            let a4225 = 0u64;
            nat_rem(a4223, a4224, a4225)
        };
        let a4227 = 1u64;
        if nat_eq(a4226, a4227) {
            let a4228 = v3;
            let a4229 = v2;
            fixed_u32::bit_or(a4228, a4229)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
