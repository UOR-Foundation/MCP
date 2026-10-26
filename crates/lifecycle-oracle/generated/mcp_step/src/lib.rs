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
    C0(Bytes, List<Adt1>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(u64),
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt4, Option<Str>, Adt7, Adt13, List<Adt15>, u64, Bytes, u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Str, Option<Str>, Str, Option<Str>, Option<Str>, Option<List<Adt5>>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Str, Option<Str>, Option<List<Str>>, Option<Adt6>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Option<Adt8>, Option<Adt8>, Option<Adt8>, Option<Adt11>, Option<Adt12>, Option<Adt11>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(List<(Str, Adt9)>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1(bool),
    C2(Adt10),
    C3(Str),
    C4(List<Adt9>),
    C5(List<(Str, Adt9)>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(i64, i64),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Option<bool>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, Adt14),
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Str, Str, List<Str>, Adt16),
}

#[derive(Clone)]
pub enum Adt16 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(List<Adt1>, Adt18, Adt19),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(i64, Str, Str, Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Adt20),
    C1(Adt25),
    C2(Adt26),
    C3(Adt25),
    C4(Adt27),
    C5(Adt25),
    C6(Adt25),
    C7(Adt28),
    C8(Adt29),
    C9(Adt34),
    C10(Adt35),
    C11(Adt36),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Adt21),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Str, Adt22, Option<Adt8>, Option<Adt23>, Option<Adt24>, Adt8),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Option<Adt8>, Option<Adt8>, Option<Adt8>, Option<Adt8>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt23 {
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
pub enum Adt24 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt21, Option<Str>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Adt21, Str, Option<Adt8>, Option<Adt8>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Adt21, Str, Option<Adt8>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Adt21, Str, Option<List<(Str, Str)>>, Option<Adt8>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Adt21, Adt30, Adt32, Option<Adt33>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Adt31),
    C1(Str),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Str, Option<Str>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Option<List<(Str, Str)>>),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt21, Str),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Adt21, Str, Adt8),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Adt21, Str),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Adt38),
    C1(Adt39),
    C2(Adt42),
    C3(Adt54),
    C4(Adt56),
    C5(Adt57),
    C6(Adt58),
    C7(Adt60),
    C8(Adt63),
    C9(Adt65),
    C10(Adt67),
    C11,
    C12(Adt86),
    C13(Adt87),
    C14(Adt92),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(Adt13, List<Str>, Adt7, Option<Str>),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(Adt13, Option<Str>, List<Adt40>),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(Str, Option<Str>, Option<List<Adt5>>, Option<Str>, Adt8, Option<Adt8>, Option<Adt41>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(Option<Str>, Option<bool>, Option<bool>, Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt42 {
    C0(List<Adt43>, Option<Adt9>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt43 {
    C0(Adt44),
    C1(Adt47),
    C2(Adt48),
    C3(Adt49),
    C4(Adt50),
}

#[derive(Clone)]
pub enum Adt44 {
    C0(Str, Option<Adt45>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt45 {
    C0(Option<List<Adt46>>, Option<Adt10>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt46 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt47 {
    C0(Str, Str, Option<Adt45>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt48 {
    C0(Str, Str, Option<Adt45>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt49 {
    C0(Str, Option<Str>, Option<List<Adt5>>, Str, Option<Str>, Option<Str>, Option<Adt45>, Option<u64>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(Adt51, Option<Adt45>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(Adt52),
    C1(Adt53),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(Str, Option<Str>, Str, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Str, Option<Str>, Str, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(Adt42, List<Adt55>),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(Adt10, Option<Adt10>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(Adt13, Option<Str>, List<Adt49>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(Adt13, List<Adt51>),
}

#[derive(Clone)]
pub enum Adt58 {
    C0(Adt13, Option<Str>, List<Adt59>),
}

#[derive(Clone)]
pub enum Adt59 {
    C0(Str, Option<Str>, Option<List<Adt5>>, Str, Option<Str>, Option<Str>, Option<Adt45>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt60 {
    C0(Adt13, Option<Str>, List<Adt61>),
}

#[derive(Clone)]
pub enum Adt61 {
    C0(Str, Option<Str>, Option<List<Adt5>>, Option<Str>, Option<List<Adt62>>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt62 {
    C0(Str, Option<Str>, Option<Str>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt63 {
    C0(Option<Str>, List<Adt64>),
}

#[derive(Clone)]
pub enum Adt64 {
    C0(Adt46, Adt43),
}

#[derive(Clone)]
pub enum Adt65 {
    C0(Adt66),
}

#[derive(Clone)]
pub enum Adt66 {
    C0(List<Str>, Option<u64>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt67 {
    C0(Adt68),
    C1(Adt68, List<(Str, Adt69)>),
    C2(Adt68, Adt8),
    C3(Adt68, Adt85),
    C4(Adt68),
}

#[derive(Clone)]
pub enum Adt68 {
    C0(Str, Option<Str>, Str, Str, Option<i64>, Option<i64>),
}

#[derive(Clone)]
pub enum Adt69 {
    C0(Adt70),
    C1(Adt74),
    C2(Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt70 {
    C0(Adt71),
    C1(Adt73),
}

#[derive(Clone)]
pub enum Adt71 {
    C0(Option<Adt72>, Str, Adt8),
}

#[derive(Clone)]
pub enum Adt72 {
    C0,
}

#[derive(Clone)]
pub enum Adt73 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt74 {
    C0(List<Adt75>, Option<Adt80>, Option<Str>, Option<Adt82>, Option<Adt10>, u64, Option<List<Str>>, Option<Adt8>, Option<List<Adt40>>, Option<Adt83>),
}

#[derive(Clone)]
pub enum Adt75 {
    C0(Adt46, Adt76, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt76 {
    C0(Adt77),
    C1(List<Adt77>),
}

#[derive(Clone)]
pub enum Adt77 {
    C0(Adt44),
    C1(Adt47),
    C2(Adt48),
    C3(Adt78),
    C4(Adt79),
}

#[derive(Clone)]
pub enum Adt78 {
    C0(Str, Str, Adt8, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt79 {
    C0(Str, List<Adt43>, Option<Adt9>, Option<bool>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt80 {
    C0(Option<List<Adt81>>, Option<Adt10>, Option<Adt10>, Option<Adt10>),
}

#[derive(Clone)]
pub enum Adt81 {
    C0(Option<Str>),
}

#[derive(Clone)]
pub enum Adt82 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt83 {
    C0(Option<Adt84>),
}

#[derive(Clone)]
pub enum Adt84 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt85 {
    C0(i64, Str, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt86 {
    C0(List<(Str, Adt69)>),
    C1(Str),
    C2(List<(Str, Adt69)>, Str),
}

#[derive(Clone)]
pub enum Adt87 {
    C0(Adt88, Adt68, Adt89),
}

#[derive(Clone)]
pub enum Adt88 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt89 {
    C0(Adt67, i64, Adt90),
}

#[derive(Clone)]
pub enum Adt90 {
    C0(i64, List<(Str, Adt69)>, i64, Adt91),
}

#[derive(Clone)]
pub enum Adt91 {
    C0(Adt8),
    C1(Adt85),
}

#[derive(Clone)]
pub enum Adt92 {
    C0(Adt89, Adt93, bool),
}

#[derive(Clone)]
pub enum Adt93 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt94 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str),
    C5(Str),
    C6(Str),
    C7(u64),
    C8(Adt22),
    C9(Str),
    C10,
    C11,
    C12(Str),
    C13,
    C14,
    C15(Adt95),
}

#[derive(Clone)]
pub enum Adt95 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt96 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt97 {
    C0(List<u64>),
    C1(Adt95),
}

#[derive(Clone)]
pub enum Adt98 {
    C0(Adt57),
    C1(Adt86),
    C2(Adt95),
}

#[derive(Clone)]
pub enum Adt99 {
    C0(Adt63),
    C1(Adt86),
    C2(Adt95),
}

#[derive(Clone)]
pub enum Adt100 {
    C0(List<u64>, List<u64>, List<Adt101>, Bytes, u64, List<List<u64>>, Bytes, Adt18),
}

#[derive(Clone)]
pub enum Adt101 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt102 {
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
pub enum Adt103 {
    C0(Adt42),
    C1(Adt86),
    C2(Adt87),
    C3(Adt95),
}

#[derive(Clone)]
pub enum Adt104 {
    C0(u64, List<Adt101>, List<List<u64>>, List<u64>, Option<List<u64>>),
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt54),
    C1(Adt106, Adt54, Adt90),
    C2(Adt86),
}

#[derive(Clone)]
pub enum Adt106 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt107 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt108 {
    C0(Adt54),
    C1(Adt86),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt110 {
    C0(Adt89, bool),
}

#[derive(Clone)]
pub enum Adt111 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt112 {
    C0(Adt9, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1(List<Adt114>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt114 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt115 {
    C0,
    C1(List<Adt114>, Str),
}

#[derive(Clone)]
pub enum Adt116 {
    C0(Adt9, Adt9, List<Adt114>),
    C1(List<Adt9>, Adt9, List<Adt114>),
    C2(List<Adt9>, Adt9, List<Adt114>, u64),
    C3(List<Adt114>),
    C4(Option<Adt9>, Option<Adt9>, Adt9, List<Adt114>),
    C5(Adt117, Adt9, Adt9, List<Adt114>),
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
    C0,
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt119 {
    C0,
    C1,
    C2(Adt8),
}

#[derive(Clone)]
pub enum Adt120 {
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
pub enum Adt121 {
    C0,
    C1(Option<Adt111>, Adt122),
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
    C8,
}

#[derive(Clone)]
pub enum Adt123 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt124 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt125 {
    C0(List<Adt15>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt126 {
    C0(Adt115, List<Adt116>),
}

#[derive(Clone)]
pub enum Adt127 {
    C0(Adt111, Str, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt128 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt116>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0(Adt9),
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt130 {
    C0(List<Adt9>, List<Adt9>),
    C1(Str, List<(Str, Adt9)>, List<(Str, Adt9)>),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(List<u64>),
    C1(u64, Adt9),
    C2(u64, List<Adt9>),
    C3(u64, List<Adt9>),
    C4(u64, List<(Str, Adt9)>),
    C5(u64, List<(Str, Adt9)>),
}

#[derive(Clone)]
pub enum Adt132 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt133>, List<Adt135>),
}

#[derive(Clone)]
pub enum Adt133 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt134),
    C7(Adt134),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt134 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Str, List<Str>, Adt16, Adt136),
}

#[derive(Clone)]
pub enum Adt136 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt137 {
    C0,
    C1(u64),
    C2(u64),
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
    C0(Adt140, List<Adt144>, u64),
}

#[derive(Clone)]
pub enum Adt140 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt141, List<u64>, bool),
    C7(u64, u64),
    C8(Adt142),
    C9(Adt9),
}

#[derive(Clone)]
pub enum Adt141 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt142 {
    C0(bool, Adt143, i64, u64, i64, bool, i64),
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
    C7,
}

#[derive(Clone)]
pub enum Adt144 {
    C0(List<Adt9>),
    C1(List<(Str, Adt9)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt145 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt146 {
    C0(Adt111, Result<Adt147, Adt150>),
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Adt37, Adt148, List<Adt149>),
}

#[derive(Clone)]
pub enum Adt148 {
    C0(Option<Adt4>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt149 {
    C0(Adt24, Adt55),
}

#[derive(Clone)]
pub enum Adt150 {
    C0(Adt151),
    C1(Str, List<Str>),
    C2(Adt133),
    C3(Str),
    C4(Adt94),
}

#[derive(Clone)]
pub enum Adt151 {
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
pub enum Adt152 {
    C0(Option<Str>, Option<Adt133>),
}

#[derive(Clone)]
pub enum Adt153 {
    C0,
    C1(Adt111),
    C2,
}

#[derive(Clone)]
pub enum Adt154 {
    C0(Adt120),
    C1(Adt139),
}

#[derive(Clone)]
pub enum Adt155 {
    C0(Adt102),
    C1(Adt125),
}

#[derive(Clone)]
pub enum Adt156 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt157 {
    C0(Adt120),
    C1(List<Adt131>, List<u64>),
}

#[derive(Clone)]
pub enum Adt158 {
    C0(Adt111, Adt159, Adt21, Adt8),
}

#[derive(Clone)]
pub enum Adt159 {
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
pub enum Adt160 {
    C0,
    C1,
    C2(Adt9),
}

#[derive(Clone)]
pub enum Adt161 {
    C0,
    C1(Adt159),
}

#[derive(Clone)]
pub enum Adt162 {
    C0(Adt163),
    C1(Adt150),
}

#[derive(Clone)]
pub enum Adt163 {
    C0(Adt111, Str, Adt21, Adt8),
}

#[derive(Clone)]
pub enum Adt164 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt165 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt166 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt167 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt168 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt169 {
    C0,
    C1(List<(Adt9, Adt9)>),
}

#[derive(Clone)]
pub enum Adt170 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt171 {
    C0,
    C1(Adt23),
}

#[derive(Clone)]
pub enum Adt172 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt173 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt174 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt173, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F4,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt17) -> R<Result<Adt37, Adt94>> {
        match self {
            Fn0::F4 => f4(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F25,
    F64(Fn0, Adt0),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt18, p1: Adt19) -> R<Result<Adt37, Adt94>> {
        match self {
            Fn1::F25 => f25(p0, p1),
            Fn1::F64(k0, k1) => f64(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F41(List<Adt1>),
    F135,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt26) -> R<Result<Adt54, Adt94>> {
        match self {
            Fn2::F41(k0) => Ok(f41(k0.clone(), p0)),
            Fn2::F135 => f135(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F90,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt26) -> R<Option<Adt105>> {
        match self {
            Fn3::F90 => f90(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F117(Fn2, Adt40),
    F189(Fn5, Adt40),
}

impl Fn4 {
    pub fn apply(&self, p0: Adt26) -> R<Result<Adt37, Adt94>> {
        match self {
            Fn4::F117(k0, k1) => f117(k0.clone(), k1.clone(), p0),
            Fn4::F189(k0, k1) => f189(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F134,
}

impl Fn5 {
    pub fn apply(&self, p0: Adt26) -> R<Result<Adt108, Adt94>> {
        match self {
            Fn5::F134 => f134(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F217,
}

impl Fn6 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt51>>> {
        match self {
            Fn6::F217 => f217(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F229,
}

impl Fn7 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt63 {
        match self {
            Fn7::F229 => f229(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F231,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt29) -> R<Result<List<Str>, Adt94>> {
        match self {
            Fn8::F231 => f231(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F254(Fn1, Adt100),
    F264(Fn1, Adt100),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt19) -> R<Result<Adt37, Adt94>> {
        match self {
            Fn9::F254(k0, k1) => f254(k0.clone(), k1.clone(), p0),
            Fn9::F264(k0, k1) => f264(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F1098(Bytes),
    F1123(Bytes, Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn10::F1098(k0) => f1098(k0.clone(), p0),
            Fn10::F1123(k0, k1) => f1123(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F1103(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn11::F1103(k0) => f1103(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F1108(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn12::F1108(k0) => f1108(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F1131(Bytes, u64, u64, u64),
    F1138(Bytes, u64, u64),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, Adt172)) -> R<(u64, Adt172)> {
        match self {
            Fn13::F1131(k0, k1, k2, k3) => f1131(k0.clone(), *k1, *k2, *k3, p0),
            Fn13::F1138(k0, k1, k2) => f1138(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F1133,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt174) -> R<Adt174> {
        match self {
            Fn14::F1133 => f1133(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F1163,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F1163 => f1163(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<List<u64>> {
    let m1 = f1(v0.clone())?;
    match m1 {
        Ok(v2) => {
            f5(v0, v2)
        }
        Err(_) => {
            f2(f3()?, Fn0::F4, v0)
        }
    }
}

pub fn f1(v0: Bytes) -> R<Result<Adt0, Adt2>> {
    f6(v0.clone(), f7(v0, 0u64)?)
}

pub fn f2(v0: Adt3, v1: Fn0, v2: Bytes) -> R<List<u64>> {
    f8(f9(v0, v1, v2)?)
}

pub fn f3() -> R<Adt3> {
    Ok(Adt3::C0(f10(), None::<Str>, f11(), f12(), f13(f14()?)?, f15(), Bytes::lit(&[]), 0u64))
}

pub fn f4(v0: Adt17) -> R<Result<Adt37, Adt94>> {
    let m3 = match v0.clone() {
        Adt17::C0(_, _, h2) => {
            h2
        }
    };
    match m3 {
        Adt19::C0(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h19, _) => {
                    h19
                }
            }, match v0 {
                Adt17::C0(_, _, h20) => {
                    h20
                }
            })
        }
        Adt19::C1(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h21, _) => {
                    h21
                }
            }, match v0 {
                Adt17::C0(_, _, h22) => {
                    h22
                }
            })
        }
        Adt19::C2(v1) => {
            f16(match v0.clone() {
                Adt17::C0(h4, _, _) => {
                    h4
                }
            }, match v0.clone() {
                Adt17::C0(_, h5, _) => {
                    h5
                }
            }, match v0 {
                Adt17::C0(_, _, h6) => {
                    h6
                }
            }, v1)
        }
        Adt19::C3(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h23, _) => {
                    h23
                }
            }, match v0 {
                Adt17::C0(_, _, h24) => {
                    h24
                }
            })
        }
        Adt19::C4(v2) => {
            let a8 = match v2.clone() {
                Adt27::C0(_, h7, _, _) => {
                    h7
                }
            };
            let a9 = Str::lit("effect://item");
            if equal(a8, a9) {
                Ok(f17(f18(match v0 {
                    Adt17::C0(h10, _, _) => {
                        h10
                    }
                }, v2)))
            } else {
                f19(match v0.clone() {
                    Adt17::C0(_, h11, _) => {
                        h11
                    }
                }, match v0 {
                    Adt17::C0(_, _, h12) => {
                        h12
                    }
                })
            }
        }
        Adt19::C5(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h25, _) => {
                    h25
                }
            }, match v0 {
                Adt17::C0(_, _, h26) => {
                    h26
                }
            })
        }
        Adt19::C6(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h27, _) => {
                    h27
                }
            }, match v0 {
                Adt17::C0(_, _, h28) => {
                    h28
                }
            })
        }
        Adt19::C7(v3) => {
            let a14 = match v3.clone() {
                Adt28::C0(_, h13, _, _, _) => {
                    h13
                }
            };
            let a15 = Str::lit("effect_prompt");
            if equal(a14, a15) {
                Ok(f20(f21(match v0 {
                    Adt17::C0(h16, _, _) => {
                        h16
                    }
                }, v3)))
            } else {
                f19(match v0.clone() {
                    Adt17::C0(_, h17, _) => {
                        h17
                    }
                }, match v0 {
                    Adt17::C0(_, _, h18) => {
                        h18
                    }
                })
            }
        }
        Adt19::C8(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h29, _) => {
                    h29
                }
            }, match v0 {
                Adt17::C0(_, _, h30) => {
                    h30
                }
            })
        }
        Adt19::C9(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h31, _) => {
                    h31
                }
            }, match v0 {
                Adt17::C0(_, _, h32) => {
                    h32
                }
            })
        }
        Adt19::C10(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h33, _) => {
                    h33
                }
            }, match v0 {
                Adt17::C0(_, _, h34) => {
                    h34
                }
            })
        }
        Adt19::C11(_) => {
            f19(match v0.clone() {
                Adt17::C0(_, h35, _) => {
                    h35
                }
            }, match v0 {
                Adt17::C0(_, _, h36) => {
                    h36
                }
            })
        }
    }
}

pub fn f5(v0: Bytes, v1: Adt0) -> R<List<u64>> {
    if f22(f23(match v1.clone() {
        Adt0::C0(h37, _) => {
            h37
        }
    })?)? {
        f24(f3()?, Fn1::F25, match v1 {
            Adt0::C0(h38, _) => {
                h38
            }
        })
    } else {
        f2(f3()?, Fn0::F4, v0)
    }
}

pub fn f6(v0: Bytes, v1: Option<Adt96>) -> R<Result<Adt0, Adt2>> {
    let m39 = v1;
    match m39 {
        None => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(0u64)))
        }
        Some(v2) => {
            f26(v0.clone(), v2.clone(), f27(v0, match v2 {
                Adt96::C0(_, h40) => {
                    h40
                }
            })?)
        }
    }
}

pub fn f7(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let m41 = f27(v0.clone(), v1)?;
    match m41 {
        None => {
            Ok(None::<Adt96>)
        }
        Some(v2) => {
            let m47 = {
                let a44 = v0;
                let a45 = {
                    let a42 = v1;
                    let a43 = 4u64;
                    nat_add(a42, a43)?
                };
                let a46 = v2;
                slice_bytes(a44, a45, a46)
            };
            match m47 {
                None => {
                    Ok(None::<Adt96>)
                }
                Some(v3) => {
                    Ok(Some(Adt96::C0(v3, {
                        let a50 = {
                            let a48 = v1;
                            let a49 = 4u64;
                            nat_add(a48, a49)?
                        };
                        let a51 = v2;
                        nat_add(a50, a51)?
                    })))
                }
            }
        }
    }
}

pub fn f8(v0: Adt97) -> R<List<u64>> {
    let m52 = v0;
    match m52 {
        Adt97::C0(v1) => {
            let a53 = f28(0u64);
            let a54 = v1;
            Ok(append_list(a53, a54))
        }
        Adt97::C1(v2) => {
            let a59 = f28(1u64);
            let a60 = {
                let a57 = f28(match v2.clone() {
                    Adt95::C0(h55, _) => {
                        h55
                    }
                });
                let a58 = f29(match v2 {
                    Adt95::C0(_, h56) => {
                        h56
                    }
                })?;
                append_list(a57, a58)
            };
            Ok(append_list(a59, a60))
        }
    }
}

pub fn f9(v0: Adt3, v1: Fn0, v2: Bytes) -> R<Adt97> {
    let m61 = f1(v2)?;
    match m61 {
        Ok(v4) => {
            f31(v0, v1, v4.clone(), f23(match v4 {
                Adt0::C0(h62, _) => {
                    h62
                }
            })?)
        }
        Err(_) => {
            Ok(f30())
        }
    }
}

pub fn f10() -> Adt4 {
    Adt4::C0(Str::lit("mcp-core-fixture"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt5>>)
}

pub fn f11() -> Adt7 {
    Adt7::C0(None::<Adt8>, None::<Adt8>, Some(f32()), Some(Adt11::C0(None::<bool>)), Some(Adt12::C0(None::<bool>, None::<bool>)), Some(Adt11::C0(None::<bool>)), Some(Adt8::C0(List::cons((f33(), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f12() -> Adt13 {
    Adt13::C0(0u64, Adt14::C1)
}

pub fn f13(v0: List<Adt40>) -> R<List<Adt15>> {
    let m63 = f34(v0)?;
    match m63 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt15>::nil())
        }
    }
}

pub fn f14() -> R<List<Adt40>> {
    let a66 = {
        let a64 = f35()?;
        let a65 = f36();
        append_list(a64, a65)
    };
    let a67 = f37();
    Ok(append_list(a66, a67))
}

pub fn f15() -> u64 {
    100u64
}

pub fn f16(v0: List<Adt1>, v1: Adt18, v2: Adt19, v3: Adt26) -> R<Result<Adt37, Adt94>> {
    if f38(v3.clone()) {
        f39(f40()?, Fn2::F41(v0), v3)
    } else {
        if f42(v3, Str::lit("provider_effect")) {
            Ok(f43(f44(v0)))
        } else {
            f19(v1, v2)
        }
    }
}

pub fn f17(v0: Result<Adt98, Adt94>) -> Result<Adt37, Adt94> {
    let m68 = v0;
    match m68 {
        Ok(v1) => {
            let m69 = v1;
            match m69 {
                Adt98::C0(v2) => {
                    Ok::<Adt37, Adt94>(Adt37::C5(v2))
                }
                Adt98::C1(v3) => {
                    Ok::<Adt37, Adt94>(Adt37::C12(v3))
                }
                Adt98::C2(v4) => {
                    Err::<Adt37, Adt94>(Adt94::C15(v4))
                }
            }
        }
        Err(v5) => {
            Err::<Adt37, Adt94>(v5)
        }
    }
}

pub fn f18(v0: List<Adt1>, v1: Adt27) -> Result<Adt98, Adt94> {
    let m70 = v0;
    match m70.uncons() {
        None => {
            Ok::<Adt98, Adt94>(Adt98::C2(f45(4u64, match v1 {
                Adt27::C0(_, h71, _, _) => {
                    h71
                }
            })))
        }
        Some((v2, v3)) => {
            let m72 = v3;
            match m72.uncons() {
                None => {
                    let a74 = match v2.clone() {
                        Adt1::C0(h73, _) => {
                            h73
                        }
                    };
                    let a75 = 4u64;
                    if nat_eq(a74, a75) {
                        Ok::<Adt98, Adt94>(Adt98::C0(f46(match v1 {
                            Adt27::C0(_, h76, _, _) => {
                                h76
                            }
                        }, f47(Str::lit("item:"), f48(match v2 {
                            Adt1::C0(_, h77) => {
                                h77
                            }
                        })))))
                    } else {
                        Err::<Adt98, Adt94>(f49())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt98, Adt94>(f49())
                }
            }
        }
    }
}

pub fn f19(v0: Adt18, v1: Adt19) -> R<Result<Adt37, Adt94>> {
    let m78 = f50(f14()?)?;
    match m78 {
        None => {
            f51(v0, v1)
        }
        Some(v2) => {
            Ok(Err::<Adt37, Adt94>(v2))
        }
    }
}

pub fn f20(v0: Result<Adt99, Adt94>) -> Result<Adt37, Adt94> {
    let m79 = v0;
    match m79 {
        Ok(v1) => {
            let m80 = v1;
            match m80 {
                Adt99::C0(v2) => {
                    Ok::<Adt37, Adt94>(Adt37::C8(v2))
                }
                Adt99::C1(v3) => {
                    Ok::<Adt37, Adt94>(Adt37::C12(v3))
                }
                Adt99::C2(v4) => {
                    Err::<Adt37, Adt94>(Adt94::C15(v4))
                }
            }
        }
        Err(v5) => {
            Err::<Adt37, Adt94>(v5)
        }
    }
}

pub fn f21(v0: List<Adt1>, v1: Adt28) -> Result<Adt99, Adt94> {
    let m81 = v0;
    match m81.uncons() {
        None => {
            Ok::<Adt99, Adt94>(Adt99::C2(f45(5u64, match v1 {
                Adt28::C0(_, h82, _, _, _) => {
                    h82
                }
            })))
        }
        Some((v2, v3)) => {
            let m83 = v3;
            match m83.uncons() {
                None => {
                    let a85 = match v2.clone() {
                        Adt1::C0(h84, _) => {
                            h84
                        }
                    };
                    let a86 = 5u64;
                    if nat_eq(a85, a86) {
                        Ok::<Adt99, Adt94>(Adt99::C0(f52(f47(Str::lit("prompt:"), f48(match v2 {
                            Adt1::C0(_, h87) => {
                                h87
                            }
                        })))))
                    } else {
                        Err::<Adt99, Adt94>(f49())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt99, Adt94>(f49())
                }
            }
        }
    }
}

pub fn f22(v0: Result<Adt100, Adt2>) -> R<bool> {
    let m88 = v0;
    match m88 {
        Ok(v2) => {
            Ok(f53(match v2 {
                Adt100::C0(_, h89, _, _, _, _, _, _) => {
                    h89
                }
            }, f54(Str::lit("byte-serve"))?))
        }
        Err(_) => {
            Ok(false)
        }
    }
}

pub fn f23(v0: Bytes) -> R<Result<Adt100, Adt2>> {
    let m90 = f7(v0.clone(), 0u64)?;
    match m90 {
        None => {
            Ok(Err::<Adt100, Adt2>(Adt2::C0(0u64)))
        }
        Some(v1) => {
            f55(v0.clone(), v1.clone(), f7(v0, match v1 {
                Adt96::C0(_, h91) => {
                    h91
                }
            })?)
        }
    }
}

pub fn f24(v0: Adt3, v1: Fn1, v2: Bytes) -> R<List<u64>> {
    let m92 = f23(v2)?;
    match m92 {
        Ok(v4) => {
            Ok(f56(f58(v0, v1, v4)?))
        }
        Err(_) => {
            Ok(f56(f57()))
        }
    }
}

pub fn f25(v0: Adt18, v1: Adt19) -> R<Result<Adt37, Adt94>> {
    f4(Adt17::C0(List::<Adt1>::nil(), v0, v1))
}

pub fn f26(v0: Bytes, v1: Adt96, v2: Option<u64>) -> R<Result<Adt0, Adt2>> {
    let m93 = v2;
    match m93 {
        None => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(match v1 {
                Adt96::C0(_, h94) => {
                    h94
                }
            })))
        }
        Some(v3) => {
            let m98 = f59(v0.clone(), {
                let a96 = match v1.clone() {
                    Adt96::C0(_, h95) => {
                        h95
                    }
                };
                let a97 = 4u64;
                nat_add(a96, a97)?
            }, v3, List::<Adt1>::nil())?;
            match m98 {
                Ok(v5) => {
                    Ok(f60(v0, match v1 {
                        Adt96::C0(h99, _) => {
                            h99
                        }
                    }, v5))
                }
                Err(v4) => {
                    Ok(Err::<Adt0, Adt2>(v4))
                }
            }
        }
    }
}

pub fn f27(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a103 = {
        let a100 = v1;
        let a101 = 4u64;
        nat_add(a100, a101)?
    };
    let a104 = {
        let a102 = v0.clone();
        length_bytes(a102)
    };
    if nat_le(a103, a104) {
        Ok(Some({
            let a121 = {
                let a105 = f61(v0.clone(), v1)?;
                let a106 = 16777216u64;
                nat_mul(a105, a106)?
            };
            let a122 = {
                let a119 = {
                    let a109 = f61(v0.clone(), {
                        let a107 = v1;
                        let a108 = 1u64;
                        nat_add(a107, a108)?
                    })?;
                    let a110 = 65536u64;
                    nat_mul(a109, a110)?
                };
                let a120 = {
                    let a117 = {
                        let a113 = f61(v0.clone(), {
                            let a111 = v1;
                            let a112 = 2u64;
                            nat_add(a111, a112)?
                        })?;
                        let a114 = 256u64;
                        nat_mul(a113, a114)?
                    };
                    let a118 = f61(v0, {
                        let a115 = v1;
                        let a116 = 3u64;
                        nat_add(a115, a116)?
                    })?;
                    nat_add(a117, a118)?
                };
                nat_add(a119, a120)?
            };
            nat_add(a121, a122)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f28(v0: u64) -> List<u64> {
    List::cons({
        let a126 = {
            let a123 = v0;
            let a124 = 16777216u64;
            let a125 = 0u64;
            nat_quot(a123, a124, a125)
        };
        let a127 = 256u64;
        let a128 = 0u64;
        nat_rem(a126, a127, a128)
    }, List::cons({
        let a132 = {
            let a129 = v0;
            let a130 = 65536u64;
            let a131 = 0u64;
            nat_quot(a129, a130, a131)
        };
        let a133 = 256u64;
        let a134 = 0u64;
        nat_rem(a132, a133, a134)
    }, List::cons({
        let a138 = {
            let a135 = v0;
            let a136 = 256u64;
            let a137 = 0u64;
            nat_quot(a135, a136, a137)
        };
        let a139 = 256u64;
        let a140 = 0u64;
        nat_rem(a138, a139, a140)
    }, List::cons({
        let a141 = v0;
        let a142 = 256u64;
        let a143 = 0u64;
        nat_rem(a141, a142, a143)
    }, List::<u64>::nil()))))
}

pub fn f29(v0: Bytes) -> R<List<u64>> {
    f62(v0.clone(), {
        let a144 = v0;
        length_bytes(a144)
    }, List::<u64>::nil())
}

pub fn f30() -> Adt97 {
    Adt97::C0(f56(f57()))
}

pub fn f31(v0: Adt3, v1: Fn0, v2: Adt0, v3: Result<Adt100, Adt2>) -> R<Adt97> {
    let m145 = v3;
    match m145 {
        Ok(v5) => {
            f63(v0.clone(), Fn1::F64(v1, v2.clone()), f65(match v2 {
                Adt0::C0(_, h146) => {
                    h146
                }
            }, match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h147) => {
                    h147
                }
            }), v5)
        }
        Err(_) => {
            Ok(f30())
        }
    }
}

pub fn f32() -> Adt8 {
    Adt8::C0(List::<(Str, Adt9)>::nil())
}

pub fn f33() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f34(v0: List<Adt40>) -> R<Result<List<Adt15>, Adt102>> {
    let m148 = f66(v0.clone());
    match m148 {
        None => {
            f67(v0, List::<Adt15>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt102>(v1))
        }
    }
}

pub fn f35() -> R<List<Adt40>> {
    Ok(List::cons(f68(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f69()), List::cons(f68(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f70()), List::cons(f68(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f69()), List::cons(f68(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f71()), List::cons(f68(Str::lit("test_image_content"), Str::lit("Tests image content response"), f71()), List::cons(f68(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f71()), List::cons(f68(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f71()), List::cons(f68(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f71()), List::cons(f68(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f71()), List::cons(f68(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f71()), List::cons(f68(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f72()), List::cons(f68(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f73()?), List::cons(f68(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f74()), List::cons(f75(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f71(), f76()), List::cons(f75(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f71(), f76()), List::cons(f68(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f71()), List::cons(f68(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f71()), List::cons(f68(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f71()), List::cons(f68(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f71()), List::cons(f68(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f71()), List::<Adt40>::nil())))))))))))))))))))))
}

pub fn f36() -> List<Adt40> {
    List::cons(f77(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f77(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f77(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f77(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f77(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f77(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f77(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f77(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt40>::nil()))))))))
}

pub fn f37() -> List<Adt40> {
    List::cons(f78(Str::lit("greet"), Str::lit("Answers a greeting at once; never a task"), f79(Str::lit("name"))), List::cons(f78(Str::lit("slow_compute"), Str::lit("Computes for a number of seconds; a task for a client that declared the tasks extension"), f80()), List::cons(f78(Str::lit("failing_job"), Str::lit("Always fails; always a task, so the client must declare the tasks extension"), f81()), List::cons(f78(Str::lit("protocol_error_job"), Str::lit("Ends in a protocol error; a task for a client that declared the tasks extension"), f81()), List::cons(f78(Str::lit("confirm_delete"), Str::lit("Asks the client to confirm a deletion; a task that waits for input"), f79(Str::lit("filename"))), List::cons(f78(Str::lit("multi_input"), Str::lit("Asks the client two questions at once; a task that waits for both answers"), f81()), List::cons(f78(Str::lit("test_tool_with_task"), Str::lit("Asks for a name, then becomes a task; the client must declare the tasks extension"), f81()), List::<Adt40>::nil())))))))
}

pub fn f38(v0: Adt26) -> bool {
    if f42(v0.clone(), Str::lit("effect_read")) {
        true
    } else {
        if f42(v0.clone(), Str::lit("effect_chain")) {
            true
        } else {
            if f42(v0.clone(), Str::lit("effect_edge_ok")) {
                true
            } else {
                f42(v0, Str::lit("effect_edge_over"))
            }
        }
    }
}

pub fn f39(v0: List<Adt40>, v1: Fn2, v2: Adt26) -> R<Result<Adt37, Adt94>> {
    let m150 = f82(match v2.clone() {
        Adt26::C0(_, h149, _, _, _) => {
            h149
        }
    }, v0);
    match m150 {
        None => {
            Ok(Err::<Adt37, Adt94>(Adt94::C0(match v2 {
                Adt26::C0(_, h151, _, _, _) => {
                    h151
                }
            })))
        }
        Some(v3) => {
            f83(v3, v1, v2)
        }
    }
}

pub fn f40() -> R<List<Adt40>> {
    Ok(List::cons(f68(Str::lit("effect_read"), Str::lit("Reads a key through the host"), f84()?), List::cons(f68(Str::lit("effect_chain"), Str::lit("Reads two keys through the host, the second after the first"), f85()?), List::cons(f68(Str::lit("effect_edge_ok"), Str::lit("Asks for the largest effect tag"), f69()), List::cons(f68(Str::lit("effect_edge_over"), Str::lit("Asks for a tag that does not fit thirty-two bits"), f69()), List::<Adt40>::nil())))))
}

pub fn f41(v0: List<Adt1>, v1: Adt26) -> Result<Adt54, Adt94> {
    f86(v0, v1)
}

pub fn f42(v0: Adt26, v1: Str) -> bool {
    let a153 = match v0 {
        Adt26::C0(_, h152, _, _, _) => {
            h152
        }
    };
    let a154 = v1;
    equal(a153, a154)
}

pub fn f43(v0: Result<Adt103, Adt94>) -> Result<Adt37, Adt94> {
    let m155 = v0;
    match m155 {
        Ok(v1) => {
            let m156 = v1;
            match m156 {
                Adt103::C0(v2) => {
                    Ok::<Adt37, Adt94>(Adt37::C2(v2))
                }
                Adt103::C1(v3) => {
                    Ok::<Adt37, Adt94>(Adt37::C12(v3))
                }
                Adt103::C2(v4) => {
                    Ok::<Adt37, Adt94>(Adt37::C13(v4))
                }
                Adt103::C3(v5) => {
                    Err::<Adt37, Adt94>(Adt94::C15(v5))
                }
            }
        }
        Err(v6) => {
            Err::<Adt37, Adt94>(v6)
        }
    }
}

pub fn f44(v0: List<Adt1>) -> Result<Adt103, Adt94> {
    let m157 = v0;
    match m157.uncons() {
        None => {
            Ok::<Adt103, Adt94>(Adt103::C3(f45(3u64, Str::lit("call"))))
        }
        Some((v1, v2)) => {
            let m158 = v2;
            match m158.uncons() {
                None => {
                    let a160 = match v1.clone() {
                        Adt1::C0(h159, _) => {
                            h159
                        }
                    };
                    let a161 = 3u64;
                    if nat_eq(a160, a161) {
                        Ok::<Adt103, Adt94>(Adt103::C0(f87(f47(Str::lit("call:"), f48(match v1 {
                            Adt1::C0(_, h162) => {
                                h162
                            }
                        })), false)))
                    } else {
                        Err::<Adt103, Adt94>(f49())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt103, Adt94>(f49())
                }
            }
        }
    }
}

pub fn f45(v0: u64, v1: Str) -> Adt95 {
    Adt95::C0(v0, {
        let a163 = v1;
        utf8_encode(a163)
    })
}

pub fn f46(v0: Str, v1: Str) -> Adt57 {
    Adt57::C0(f12(), List::cons(Adt51::C0(Adt52::C0(v0, Some(Str::lit("text/plain")), v1, None::<Adt8>)), List::<Adt51>::nil()))
}

pub fn f47(v0: Str, v1: Str) -> Str {
    let a164 = List::cons(v0, List::cons(v1, List::<Str>::nil()));
    let a165 = Str::lit("");
    join(a164, a165)
}

pub fn f48(v0: Bytes) -> Str {
    let m167 = {
        let a166 = v0;
        utf8_decode(a166)
    };
    match m167 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f49() -> Adt94 {
    Adt94::C9(Str::lit("the outcomes of the effects do not match the request"))
}

pub fn f50(v0: List<Adt40>) -> R<Option<Adt94>> {
    let m168 = f34(v0)?;
    match m168 {
        Ok(_) => {
            Ok(None::<Adt94>)
        }
        Err(v1) => {
            Ok(Some(Adt94::C9(f88(v1))))
        }
    }
}

pub fn f51(v0: Adt18, v1: Adt19) -> R<Result<Adt37, Adt94>> {
    let m169 = v1.clone();
    match m169 {
        Adt19::C0(_) => {
            f91(v1)
        }
        Adt19::C1(_) => {
            f91(v1)
        }
        Adt19::C2(v2) => {
            f89(v0, f14()?, Fn3::F90, f91(v1)?, v2)
        }
        Adt19::C3(_) => {
            f91(v1)
        }
        Adt19::C4(_) => {
            f91(v1)
        }
        Adt19::C5(_) => {
            f91(v1)
        }
        Adt19::C6(_) => {
            f91(v1)
        }
        Adt19::C7(_) => {
            f91(v1)
        }
        Adt19::C8(_) => {
            f91(v1)
        }
        Adt19::C9(v3) => {
            f92(v0, v3)
        }
        Adt19::C10(v4) => {
            f93(v0, v4)
        }
        Adt19::C11(v5) => {
            f94(v0, v5)
        }
    }
}

pub fn f52(v0: Str) -> Adt63 {
    Adt63::C0(None::<Str>, List::cons(Adt64::C0(Adt46::C0, f95(v0)), List::<Adt64>::nil()))
}

pub fn f53(p176: List<u64>, p177: List<u64>) -> bool {
    let m175 = (p176, p177);
    {
        let mut m175 = m175;
        loop {
            let step = {
                let (v0, v1) = m175;
                let m170 = v0;
                match m170.uncons() {
                    None => {
                        let m171 = v1;
                        match m171.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m172 = v1;
                        match m172.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a173 = v4;
                                let a174 = v6;
                                if nat_eq(a173, a174) {
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
                Ok(next) => { m175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Str) -> R<List<u64>> {
    f29({
        let a178 = v0;
        utf8_encode(a178)
    })
}

pub fn f55(v0: Bytes, v1: Adt96, v2: Option<Adt96>) -> R<Result<Adt100, Adt2>> {
    let m179 = v2;
    match m179 {
        None => {
            Ok(Err::<Adt100, Adt2>(Adt2::C0(match v1 {
                Adt96::C0(_, h180) => {
                    h180
                }
            })))
        }
        Some(v3) => {
            f96(v0.clone(), v1, v3.clone(), f27(v0, match v3 {
                Adt96::C0(_, h181) => {
                    h181
                }
            })?)
        }
    }
}

pub fn f56(v0: Adt104) -> List<u64> {
    let a205 = f28(match v0.clone() {
        Adt104::C0(h182, _, _, _, _) => {
            h182
        }
    });
    let a206 = {
        let a203 = f28({
            let a184 = match v0.clone() {
                Adt104::C0(_, h183, _, _, _) => {
                    h183
                }
            };
            length_list(a184)
        });
        let a204 = {
            let a201 = f97(match v0.clone() {
                Adt104::C0(_, h185, _, _, _) => {
                    h185
                }
            });
            let a202 = {
                let a199 = f28({
                    let a187 = match v0.clone() {
                        Adt104::C0(_, _, h186, _, _) => {
                            h186
                        }
                    };
                    length_list(a187)
                });
                let a200 = {
                    let a197 = f98(match v0.clone() {
                        Adt104::C0(_, _, h188, _, _) => {
                            h188
                        }
                    });
                    let a198 = {
                        let a195 = f28({
                            let a190 = match v0.clone() {
                                Adt104::C0(_, _, _, h189, _) => {
                                    h189
                                }
                            };
                            length_list(a190)
                        });
                        let a196 = {
                            let a193 = match v0.clone() {
                                Adt104::C0(_, _, _, h191, _) => {
                                    h191
                                }
                            };
                            let a194 = f99(match v0 {
                                Adt104::C0(_, _, _, _, h192) => {
                                    h192
                                }
                            });
                            append_list(a193, a194)
                        };
                        append_list(a195, a196)
                    };
                    append_list(a197, a198)
                };
                append_list(a199, a200)
            };
            append_list(a201, a202)
        };
        append_list(a203, a204)
    };
    append_list(a205, a206)
}

pub fn f57() -> Adt104 {
    Adt104::C0(f100(), f101(), List::<List<u64>>::nil(), f102(), None::<List<u64>>)
}

pub fn f58(v0: Adt3, v1: Fn1, v2: Adt100) -> R<Adt104> {
    f103(v0, v1, v2.clone(), f104(match v2.clone() {
        Adt100::C0(_, _, h207, _, _, _, _, _) => {
            h207
        }
    }, match v2 {
        Adt100::C0(_, _, _, _, _, h208, _, _) => {
            h208
        }
    })?)
}

pub fn f59(v0: Bytes, p219: u64, p220: u64, p221: List<Adt1>) -> R<Result<(List<Adt1>, u64), Adt2>> {
    let m218 = (p219, (p220, p221));
    {
        let mut m218 = m218;
        loop {
            let step = {
                let (v1, (v2, v3)) = m218;
                let m209 = v2;
                if m209 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Ok::<(List<Adt1>, u64), Adt2>((f105(v3, List::<Adt1>::nil()), v1))))
                } else {
                    let v4 = m209 - 1;
                    let m210 = f27(v0.clone(), v1)?;
                    match m210 {
                        None => {
                            Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Err::<(List<Adt1>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            let m213 = f7(v0.clone(), {
                                let a211 = v1;
                                let a212 = 4u64;
                                nat_add(a211, a212)?
                            })?;
                            match m213 {
                                None => {
                                    Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Err::<(List<Adt1>, u64), Adt2>(Adt2::C0({
                                        let a214 = v1;
                                        let a215 = 4u64;
                                        nat_add(a214, a215)?
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>((match v6.clone() {
                                        Adt96::C0(_, h216) => {
                                            h216
                                        }
                                    }, (v4, List::cons(Adt1::C0(v5, match v6 {
                                        Adt96::C0(h217, _) => {
                                            h217
                                        }
                                    }), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: Bytes, v1: Bytes, v2: (List<Adt1>, u64)) -> Result<Adt0, Adt2> {
    f106({
        let a224 = {
            let (_, h222) = v2.clone();
            h222
        };
        let a225 = {
            let a223 = v0;
            length_bytes(a223)
        };
        nat_eq(a224, a225)
    }, v1, {
        let (h226, _) = v2;
        h226
    }, {
        let (_, h227) = v2;
        h227
    })
}

pub fn f61(v0: Bytes, v1: u64) -> R<u64> {
    let m230 = {
        let a228 = v0;
        let a229 = v1;
        index_bytes(a228, a229)
    };
    match m230 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a275 = {
                let a251 = {
                    let a239 = {
                        let a233 = {
                            let a231 = v3;
                            let a232 = 128u8;
                            fixed_u8::bit_and(a231, a232)
                        };
                        let a234 = 128u8;
                        if equal(a233, a234) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a240 = {
                        let a237 = {
                            let a235 = v3;
                            let a236 = 64u8;
                            fixed_u8::bit_and(a235, a236)
                        };
                        let a238 = 64u8;
                        if equal(a237, a238) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a239, a240)?
                };
                let a252 = {
                    let a249 = {
                        let a243 = {
                            let a241 = v3;
                            let a242 = 32u8;
                            fixed_u8::bit_and(a241, a242)
                        };
                        let a244 = 32u8;
                        if equal(a243, a244) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a250 = {
                        let a247 = {
                            let a245 = v3;
                            let a246 = 16u8;
                            fixed_u8::bit_and(a245, a246)
                        };
                        let a248 = 16u8;
                        if equal(a247, a248) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a249, a250)?
                };
                nat_add(a251, a252)?
            };
            let a276 = {
                let a273 = {
                    let a261 = {
                        let a255 = {
                            let a253 = v3;
                            let a254 = 8u8;
                            fixed_u8::bit_and(a253, a254)
                        };
                        let a256 = 8u8;
                        if equal(a255, a256) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a262 = {
                        let a259 = {
                            let a257 = v3;
                            let a258 = 4u8;
                            fixed_u8::bit_and(a257, a258)
                        };
                        let a260 = 4u8;
                        if equal(a259, a260) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a261, a262)?
                };
                let a274 = {
                    let a271 = {
                        let a265 = {
                            let a263 = v3;
                            let a264 = 2u8;
                            fixed_u8::bit_and(a263, a264)
                        };
                        let a266 = 2u8;
                        if equal(a265, a266) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a272 = {
                        let a269 = {
                            let a267 = v3;
                            let a268 = 1u8;
                            fixed_u8::bit_and(a267, a268)
                        };
                        let a270 = 1u8;
                        if equal(a269, a270) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a271, a272)?
                };
                nat_add(a273, a274)?
            };
            nat_add(a275, a276)
        }
    }
}

pub fn f62(v0: Bytes, p279: u64, p280: List<u64>) -> R<List<u64>> {
    let m278 = (p279, p280);
    {
        let mut m278 = m278;
        loop {
            let step = {
                let (v1, v2) = m278;
                let m277 = v1;
                if m277 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m277 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f61(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63(v0: Adt3, v1: Fn1, v2: bool, v3: Adt100) -> R<Adt97> {
    f107(v0, v1, v2, v3.clone(), f104(match v3.clone() {
        Adt100::C0(_, _, h281, _, _, _, _, _) => {
            h281
        }
    }, match v3 {
        Adt100::C0(_, _, _, _, _, h282, _, _) => {
            h282
        }
    })?)
}

pub fn f64(v0: Fn0, v1: Adt0, v2: Adt18, v3: Adt19) -> R<Result<Adt37, Adt94>> {
    let c283: Fn0 = v0;
    c283.apply(Adt17::C0(match v1 {
        Adt0::C0(_, h284) => {
            h284
        }
    }, v2, v3))
}

pub fn f65(p290: List<Adt1>, v1: u64) -> bool {
    let m289 = p290;
    {
        let mut m289 = m289;
        loop {
            let step = {
                let v0 = m289;
                let m285 = v0;
                match m285.uncons() {
                    None => {
                        Err::<List<Adt1>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a287 = match v2 {
                            Adt1::C0(h286, _) => {
                                h286
                            }
                        };
                        let a288 = v1;
                        if nat_eq(a287, a288) {
                            Err::<List<Adt1>, bool>(true)
                        } else {
                            Ok::<List<Adt1>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<Adt40>) -> Option<Adt102> {
    let m291 = f108(v0, List::<Str>::nil());
    match m291 {
        None => {
            None::<Adt102>
        }
        Some(v1) => {
            Some(Adt102::C1(v1))
        }
    }
}

pub fn f67(p295: List<Adt40>, p296: List<Adt15>) -> R<Result<List<Adt15>, Adt102>> {
    let m294 = (p295, p296);
    {
        let mut m294 = m294;
        loop {
            let step = {
                let (v0, v1) = m294;
                let m292 = v0;
                match m292.uncons() {
                    None => {
                        Err::<(List<Adt40>, List<Adt15>), R<Result<List<Adt15>, Adt102>>>(Ok(Ok::<List<Adt15>, Adt102>(f109(v1, List::<Adt15>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m293 = f110(v2)?;
                        match m293 {
                            Ok(v5) => {
                                Ok::<(List<Adt40>, List<Adt15>), R<Result<List<Adt15>, Adt102>>>((v3, f109(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt40>, List<Adt15>), R<Result<List<Adt15>, Adt102>>>(Ok(Err::<List<Adt15>, Adt102>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Str, v1: Str, v2: Adt8) -> Adt40 {
    Adt40::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, None::<Adt8>, None::<Adt41>, None::<Adt8>)
}

pub fn f69() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::<(Str, Adt9)>::nil())), List::cons((Str::lit("required"), Adt9::C4(List::<Adt9>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f70() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("region"), f111(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f112()), List::cons((Str::lit("limit"), f111(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f111(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt9)>::nil())))))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("region")), List::cons(Adt9::C3(Str::lit("query")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))))
}

pub fn f71() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil())))
}

pub fn f72() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("$schema"), Adt9::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt9::C5(List::cons((Str::lit("address"), Adt9::C5(List::cons((Str::lit("$anchor"), Adt9::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("street"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("city"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("name"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("address"), Adt9::C5(List::cons((Str::lit("$ref"), Adt9::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("contactMethod"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("phone"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("email"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))))))), List::cons((Str::lit("allOf"), Adt9::C4(List::cons(Adt9::C5(List::cons((Str::lit("anyOf"), Adt9::C4(List::cons(Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())), List::cons(Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil())), List::<Adt9>::nil()))), List::cons((Str::lit("if"), Adt9::C5(List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("contactMethod"), Adt9::C5(List::cons((Str::lit("const"), Adt9::C3(Str::lit("phone"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("contactMethod")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("then"), Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("else"), Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))))))
}

pub fn f73() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("name"), f112()), List::cons((Str::lit("label"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f113(1u64)?), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("count"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f113(0u64)?), List::cons((Str::lit("maximum"), f113(10u64)?), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("mode"), Adt9::C5(List::cons((Str::lit("enum"), Adt9::C4(List::cons(Adt9::C3(Str::lit("fast")), List::cons(Adt9::C3(Str::lit("slow")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("tags"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("maxItems"), f113(3u64)?), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("nested"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("flag"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("boolean"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))))))))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("name")), List::<Adt9>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f74() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("target"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("zone"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt9::C3(Str::lit("Zone"))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::<Adt9>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f75(v0: Str, v1: Str, v2: Adt8, v3: Adt8) -> Adt40 {
    Adt40::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, Some(v3), None::<Adt41>, None::<Adt8>)
}

pub fn f76() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("sum"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("label"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("sum")), List::cons(Adt9::C3(Str::lit("label")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))))
}

pub fn f77(v0: Str, v1: Str) -> Adt40 {
    Adt40::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))), None::<Adt8>, None::<Adt41>, None::<Adt8>)
}

pub fn f78(v0: Str, v1: Str, v2: Adt8) -> Adt40 {
    Adt40::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, None::<Adt8>, None::<Adt41>, None::<Adt8>)
}

pub fn f79(v0: Str) -> Adt8 {
    f114(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((v0.clone(), f115(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(v0), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f80() -> Adt8 {
    f114(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("seconds"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), Adt9::C2(Adt10::C0(0i64, 0i64))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("label"), f115(Str::lit("string"))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("seconds")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f81() -> Adt8 {
    f114(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil())))
}

pub fn f82(v0: Str, p302: List<Adt40>) -> Option<Adt40> {
    let m301 = p302;
    {
        let mut m301 = m301;
        loop {
            let step = {
                let v1 = m301;
                let m297 = v1;
                match m297.uncons() {
                    None => {
                        Err::<List<Adt40>, Option<Adt40>>(None::<Adt40>)
                    }
                    Some((v2, v3)) => {
                        let a299 = match v2.clone() {
                            Adt40::C0(h298, _, _, _, _, _, _, _) => {
                                h298
                            }
                        };
                        let a300 = v0.clone();
                        if equal(a299, a300) {
                            Err::<List<Adt40>, Option<Adt40>>(Some(v2))
                        } else {
                            Ok::<List<Adt40>, Option<Adt40>>(v3)
                        }
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

pub fn f83(v0: Adt40, v1: Fn2, v2: Adt26) -> R<Result<Adt37, Adt94>> {
    f116(v0.clone(), v2, Fn4::F117(v1, v0))
}

pub fn f84() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("key"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f113(1u64)?), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("key")), List::<Adt9>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f85() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("first"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f113(1u64)?), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("second"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f113(1u64)?), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("first")), List::cons(Adt9::C3(Str::lit("second")), List::<Adt9>::nil())))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f86(v0: List<Adt1>, v1: Adt26) -> Result<Adt54, Adt94> {
    if f42(v1.clone(), Str::lit("effect_read")) {
        f118(v0, v1)
    } else {
        if f42(v1.clone(), Str::lit("effect_chain")) {
            f119(v0, v1)
        } else {
            if f42(v1, Str::lit("effect_edge_ok")) {
                f120(v0, 4294967295u64)
            } else {
                f120(v0, 4294967296u64)
            }
        }
    }
}

pub fn f87(v0: Str, v1: bool) -> Adt42 {
    Adt42::C0(List::cons(Adt43::C0(Adt44::C0(v0, None::<Adt45>, None::<Adt8>)), List::<Adt43>::nil()), None::<Adt9>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f88(v0: Adt102) -> Str {
    let m303 = v0;
    match m303 {
        Adt102::C0(v1) => {
            let a304 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a305 = Str::lit(" ");
            join(a304, a305)
        }
        Adt102::C1(v2) => {
            let a306 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a307 = Str::lit(" ");
            join(a306, a307)
        }
        Adt102::C2(v3) => {
            let a308 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a309 = Str::lit(" ");
            join(a308, a309)
        }
        Adt102::C3(v4) => {
            let a310 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a311 = Str::lit(" ");
            join(a310, a311)
        }
        Adt102::C4(v5, v6) => {
            let a312 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a313 = Str::lit(" ");
            join(a312, a313)
        }
        Adt102::C5(v7, v8) => {
            let a314 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a315 = Str::lit(" ");
            join(a314, a315)
        }
        Adt102::C6(v9, v10) => {
            let a316 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a317 = Str::lit(" ");
            join(a316, a317)
        }
        Adt102::C7(v11) => {
            let a318 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a319 = Str::lit(" ");
            join(a318, a319)
        }
        Adt102::C8(v12) => {
            let a320 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a321 = Str::lit(" ");
            join(a320, a321)
        }
        Adt102::C9(v13) => {
            let a322 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a323 = Str::lit(" ");
            join(a322, a323)
        }
        Adt102::C10(v14) => {
            let a324 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a325 = Str::lit(" ");
            join(a324, a325)
        }
        Adt102::C11(v15) => {
            let a326 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a327 = Str::lit(" ");
            join(a326, a327)
        }
        Adt102::C12(v16, v17) => {
            let a328 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a329 = Str::lit(" ");
            join(a328, a329)
        }
    }
}

pub fn f89(v0: Adt18, v1: List<Adt40>, v2: Fn3, v3: Result<Adt37, Adt94>, v4: Adt26) -> R<Result<Adt37, Adt94>> {
    let m331 = f82(match v4.clone() {
        Adt26::C0(_, h330, _, _, _) => {
            h330
        }
    }, v1);
    match m331 {
        None => {
            Ok(Err::<Adt37, Adt94>(Adt94::C0(match v4 {
                Adt26::C0(_, h332, _, _, _) => {
                    h332
                }
            })))
        }
        Some(v5) => {
            f121(v0, v5, v4.clone(), {
                let c333: Fn3 = v2;
                c333.apply(v4)?
            }, v3)
        }
    }
}

pub fn f90(v0: Adt26) -> R<Option<Adt105>> {
    if f122(v0.clone(), Str::lit("greet")) {
        Ok(Some(Adt105::C0(f123({
            let a334 = List::cons(Str::lit("Hello, "), List::cons(f124(v0, Str::lit("name"), Str::lit("world")), List::cons(Str::lit("!"), List::<Str>::nil())));
            let a335 = Str::lit("");
            join(a334, a335)
        }, false))))
    } else {
        if f122(v0.clone(), Str::lit("slow_compute")) {
            Ok(Some(f125(v0)?))
        } else {
            if f122(v0.clone(), Str::lit("confirm_delete")) {
                Ok(Some(f126(v0)?))
            } else {
                if f122(v0.clone(), Str::lit("multi_input")) {
                    Ok(Some(f127()?))
                } else {
                    if f122(v0.clone(), Str::lit("failing_job")) {
                        Ok(Some(f128()?))
                    } else {
                        if f122(v0.clone(), Str::lit("protocol_error_job")) {
                            Ok(Some(f129()))
                        } else {
                            if f122(v0.clone(), Str::lit("test_tool_with_task")) {
                                Ok(Some(f130(v0)?))
                            } else {
                                Ok(None::<Adt105>)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f91(v0: Adt19) -> R<Result<Adt37, Adt94>> {
    let m336 = v0;
    match m336 {
        Adt19::C0(_) => {
            Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt19::C1(v1) => {
            Ok(f131(f12(), f14()?, v1))
        }
        Adt19::C2(v2) => {
            if f132(match v2.clone() {
                Adt26::C0(_, h337, _, _, _) => {
                    h337
                }
            }) {
                f133(f14()?, Fn5::F134, v2)
            } else {
                f39(f14()?, Fn2::F135, v2)
            }
        }
        Adt19::C3(v4) => {
            Ok(f136(v4))
        }
        Adt19::C4(v5) => {
            f137(v5)
        }
        Adt19::C5(v6) => {
            Ok(f138(v6))
        }
        Adt19::C6(v7) => {
            Ok(f139(v7))
        }
        Adt19::C7(v8) => {
            if f140(match v8.clone() {
                Adt28::C0(_, h338, _, _, _) => {
                    h338
                }
            }) {
                Ok(f141(v8))
            } else {
                Ok(f142(v8))
            }
        }
        Adt19::C8(v9) => {
            f143(v9)
        }
        Adt19::C9(_) => {
            Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt19::C10(_) => {
            Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt19::C11(_) => {
            Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the fixture offers tools only"))))
        }
    }
}

pub fn f92(v0: Adt18, v1: Adt34) -> R<Result<Adt37, Adt94>> {
    let m341 = f144(v0.clone(), match v1.clone() {
        Adt34::C0(h339, _) => {
            h339
        }
    }, match v1 {
        Adt34::C0(_, h340) => {
            h340
        }
    })?;
    match m341 {
        Ok(v3) => {
            f145(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt37, Adt94>(v2))
        }
    }
}

pub fn f93(v0: Adt18, v1: Adt35) -> R<Result<Adt37, Adt94>> {
    let m344 = f144(v0.clone(), match v1.clone() {
        Adt35::C0(h342, _, _) => {
            h342
        }
    }, match v1.clone() {
        Adt35::C0(_, h343, _) => {
            h343
        }
    })?;
    match m344 {
        Ok(v3) => {
            f146(v0, v3, match v1 {
                Adt35::C0(_, _, h345) => {
                    h345
                }
            })
        }
        Err(v2) => {
            Ok(Err::<Adt37, Adt94>(v2))
        }
    }
}

pub fn f94(v0: Adt18, v1: Adt36) -> R<Result<Adt37, Adt94>> {
    let m348 = f144(v0.clone(), match v1.clone() {
        Adt36::C0(h346, _) => {
            h346
        }
    }, match v1 {
        Adt36::C0(_, h347) => {
            h347
        }
    })?;
    match m348 {
        Ok(v3) => {
            f147(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt37, Adt94>(v2))
        }
    }
}

pub fn f95(v0: Str) -> Adt43 {
    Adt43::C0(Adt44::C0(v0, None::<Adt45>, None::<Adt8>))
}

pub fn f96(v0: Bytes, v1: Adt96, v2: Adt96, v3: Option<u64>) -> R<Result<Adt100, Adt2>> {
    let m349 = v3;
    match m349 {
        None => {
            Ok(Err::<Adt100, Adt2>(Adt2::C0(match v2 {
                Adt96::C0(_, h350) => {
                    h350
                }
            })))
        }
        Some(v4) => {
            let m354 = f148(v0.clone(), {
                let a352 = match v2.clone() {
                    Adt96::C0(_, h351) => {
                        h351
                    }
                };
                let a353 = 4u64;
                nat_add(a352, a353)?
            }, v4, List::<Adt101>::nil())?;
            match m354 {
                Ok(v6) => {
                    f149(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt100, Adt2>(v5))
                }
            }
        }
    }
}

pub fn f97(v0: List<Adt101>) -> List<u64> {
    f150(f151(v0, List::<Adt101>::nil()), List::<u64>::nil())
}

pub fn f98(v0: List<List<u64>>) -> List<u64> {
    f152(f153(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f99(v0: Option<List<u64>>) -> List<u64> {
    let m355 = v0;
    match m355 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a359 = f28(1u64);
            let a360 = {
                let a357 = f28({
                    let a356 = v1.clone();
                    length_list(a356)
                });
                let a358 = v1;
                append_list(a357, a358)
            };
            append_list(a359, a360)
        }
    }
}

pub fn f100() -> u64 {
    500u64
}

pub fn f101() -> List<Adt101> {
    List::cons(Adt101::C0(f154(), f155()), List::<Adt101>::nil())
}

pub fn f102() -> List<u64> {
    let a363 = f156();
    let a364 = {
        let a361 = f157();
        let a362 = f158();
        append_list(a361, a362)
    };
    append_list(a363, a364)
}

pub fn f103(v0: Adt3, v1: Fn1, v2: Adt100, v3: Option<Adt107>) -> R<Adt104> {
    let m365 = v3;
    match m365 {
        None => {
            f160(v0, v1, v2)
        }
        Some(v4) => {
            f159(v4)
        }
    }
}

pub fn f104(v0: List<Adt101>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m366 = f161(v0.clone(), v1.clone())?;
    match m366 {
        None => {
            f162(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f105(p369: List<Adt1>, p370: List<Adt1>) -> List<Adt1> {
    let m368 = (p369, p370);
    {
        let mut m368 = m368;
        loop {
            let step = {
                let (v0, v1) = m368;
                let m367 = v0;
                match m367.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: bool, v1: Bytes, v2: List<Adt1>, v3: u64) -> Result<Adt0, Adt2> {
    if v0 {
        Ok::<Adt0, Adt2>(Adt0::C0(v1, v2))
    } else {
        Err::<Adt0, Adt2>(Adt2::C1(v3))
    }
}

pub fn f107(v0: Adt3, v1: Fn1, v2: bool, v3: Adt100, v4: Option<Adt107>) -> R<Adt97> {
    let m371 = v4;
    match m371 {
        None => {
            f163(v0, v1, v2, v3)
        }
        Some(v5) => {
            Ok(Adt97::C0(f56(f159(v5)?)))
        }
    }
}

pub fn f108(p377: List<Adt40>, p378: List<Str>) -> Option<Str> {
    let m376 = (p377, p378);
    {
        let mut m376 = m376;
        loop {
            let step = {
                let (v0, v1) = m376;
                let m372 = v0;
                match m372.uncons() {
                    None => {
                        Err::<(List<Adt40>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f164(match v2.clone() {
                            Adt40::C0(h373, _, _, _, _, _, _, _) => {
                                h373
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt40>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt40::C0(h374, _, _, _, _, _, _, _) => {
                                    h374
                                }
                            }))
                        } else {
                            Ok::<(List<Adt40>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt40::C0(h375, _, _, _, _, _, _, _) => {
                                    h375
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(p381: List<Adt15>, p382: List<Adt15>) -> List<Adt15> {
    let m380 = (p381, p382);
    {
        let mut m380 = m380;
        loop {
            let step = {
                let (v0, v1) = m380;
                let m379 = v0;
                match m379.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: Adt40) -> R<Result<List<Adt15>, Adt102>> {
    let m385 = f165(match v0.clone() {
        Adt40::C0(h383, _, _, _, _, _, _, _) => {
            h383
        }
    }, f166(match v0.clone() {
        Adt40::C0(h384, _, _, _, _, _, _, _) => {
            h384
        }
    })?);
    match m385 {
        None => {
            f167(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt102>(v1))
        }
    }
}

pub fn f111(v0: Str, v1: Str) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())))
}

pub fn f112() -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))
}

pub fn f113(v0: u64) -> R<Adt9> {
    Ok(Adt9::C2(Adt10::C0(f168(v0, 0i64)?, 0i64)))
}

pub fn f114(v0: List<(Str, Adt9)>) -> Adt8 {
    Adt8::C0(v0)
}

pub fn f115(v0: Str) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::<(Str, Adt9)>::nil()))
}

pub fn f116(v0: Adt40, v1: Adt26, v2: Fn4) -> R<Result<Adt37, Adt94>> {
    let m392 = f169(Adt9::C5({
        let m388 = match v0.clone() {
            Adt40::C0(_, _, _, _, h387, _, _, _) => {
                h387
            }
        };
        match m388 {
            Adt8::C0(h386) => {
                h386
            }
        }
    }), Adt9::C5({
        let m391 = match v0 {
            Adt40::C0(_, _, _, _, h390, _, _, _) => {
                h390
            }
        };
        match m391 {
            Adt8::C0(h389) => {
                h389
            }
        }
    }), f170(v1.clone()))?;
    match m392 {
        Adt113::C0 => {
            let c393: Fn4 = v2;
            c393.apply(v1)
        }
        Adt113::C1(v3, v4) => {
            Ok(Ok::<Adt37, Adt94>(Adt37::C2(f87(f171(v3, v4)?, true))))
        }
        Adt113::C2(_) => {
            Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt113::C3(_) => {
            Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt113::C4 => {
            Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f117(v0: Fn2, v1: Adt40, v2: Adt26) -> R<Result<Adt37, Adt94>> {
    f172(v1, v0, v2)
}

pub fn f118(v0: List<Adt1>, v1: Adt26) -> Result<Adt54, Adt94> {
    let m394 = v0;
    match m394.uncons() {
        None => {
            f173(1u64, f174(Str::lit("key"), v1))
        }
        Some((v2, v3)) => {
            let m395 = v3;
            match m395.uncons() {
                None => {
                    let a397 = match v2.clone() {
                        Adt1::C0(h396, _) => {
                            h396
                        }
                    };
                    let a398 = 1u64;
                    if nat_eq(a397, a398) {
                        f175(f47(Str::lit("read:"), f48(match v2 {
                            Adt1::C0(_, h399) => {
                                h399
                            }
                        })))
                    } else {
                        f176()
                    }
                }
                Some((_, _)) => {
                    f176()
                }
            }
        }
    }
}

pub fn f119(v0: List<Adt1>, v1: Adt26) -> Result<Adt54, Adt94> {
    let m400 = v0;
    match m400.uncons() {
        None => {
            f173(1u64, f174(Str::lit("first"), v1))
        }
        Some((v2, v3)) => {
            let m401 = v3;
            match m401.uncons() {
                None => {
                    let a403 = match v2.clone() {
                        Adt1::C0(h402, _) => {
                            h402
                        }
                    };
                    let a404 = 1u64;
                    if nat_eq(a403, a404) {
                        f173(2u64, f47(f48(match v2 {
                            Adt1::C0(_, h405) => {
                                h405
                            }
                        }), f174(Str::lit("second"), v1)))
                    } else {
                        f176()
                    }
                }
                Some((v4, v5)) => {
                    let m406 = v5;
                    match m406.uncons() {
                        None => {
                            let a408 = match v2.clone() {
                                Adt1::C0(h407, _) => {
                                    h407
                                }
                            };
                            let a409 = 1u64;
                            if nat_eq(a408, a409) {
                                let a411 = match v4.clone() {
                                    Adt1::C0(h410, _) => {
                                        h410
                                    }
                                };
                                let a412 = 2u64;
                                if nat_eq(a411, a412) {
                                    f175(f47(f47(Str::lit("chain:"), f48(match v2 {
                                        Adt1::C0(_, h413) => {
                                            h413
                                        }
                                    })), f47(Str::lit("|"), f48(match v4 {
                                        Adt1::C0(_, h414) => {
                                            h414
                                        }
                                    }))))
                                } else {
                                    f176()
                                }
                            } else {
                                f176()
                            }
                        }
                        Some((_, _)) => {
                            f176()
                        }
                    }
                }
            }
        }
    }
}

pub fn f120(v0: List<Adt1>, v1: u64) -> Result<Adt54, Adt94> {
    let m415 = v0;
    match m415.uncons() {
        None => {
            f173(v1, Str::lit("edge"))
        }
        Some((_, _)) => {
            f176()
        }
    }
}

pub fn f121(v0: Adt18, v1: Adt40, v2: Adt26, v3: Option<Adt105>, v4: Result<Adt37, Adt94>) -> R<Result<Adt37, Adt94>> {
    let m416 = v3;
    match m416 {
        None => {
            Ok(v4)
        }
        Some(v5) => {
            f177(v0, v1.clone(), v2.clone(), v5, f178(v1, v2)?)
        }
    }
}

pub fn f122(v0: Adt26, v1: Str) -> bool {
    let a418 = match v0 {
        Adt26::C0(_, h417, _, _, _) => {
            h417
        }
    };
    let a419 = v1;
    equal(a418, a419)
}

pub fn f123(v0: Str, v1: bool) -> Adt54 {
    Adt54::C0(Adt42::C0(List::cons(f179(v0), List::<Adt43>::nil()), None::<Adt9>, if v1 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt55>::nil())
}

pub fn f124(v0: Adt26, v1: Str, v2: Str) -> Str {
    let m420 = f180(v0, v1);
    match m420 {
        Adt118::C0 => {
            v2
        }
        Adt118::C1(v3) => {
            let m421 = f181(v3);
            match m421 {
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

pub fn f125(v0: Adt26) -> R<Adt105> {
    let v1: i64 = f182(v0.clone());
    let v2: Str = {
        let a423 = List::cons(Str::lit("slow_compute"), List::cons(f124(v0, Str::lit("label"), Str::lit("unnamed")), List::cons(Str::lit("finished after"), List::cons({
            let a422 = v1;
            format_i64(a422)
        }, List::cons(Str::lit("seconds"), List::<Str>::nil())))));
        let a424 = Str::lit(" ");
        join(a423, a424)
    };
    Ok(Adt105::C1(Adt106::C0, f123(v2.clone(), false), f183(0i64, List::<(Str, Adt69)>::nil(), {
        let a425 = v1;
        let a426 = 1000i64;
        int_mul(a425, a426)?
    }, f184(v2, false)?)))
}

pub fn f126(v0: Adt26) -> R<Adt105> {
    let v1: Str = {
        let a427 = List::cons(Str::lit("Deleted"), List::cons(f124(v0.clone(), Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a428 = Str::lit(" ");
        join(a427, a428)
    };
    Ok(Adt105::C1(Adt106::C0, f123(v1.clone(), false), f183(0i64, List::cons((Str::lit("confirm"), f185({
        let a429 = List::cons(Str::lit("Delete"), List::cons(f124(v0, Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a430 = Str::lit(" ");
        join(a429, a430)
    }, Str::lit("confirm"), Str::lit("boolean"))), List::<(Str, Adt69)>::nil()), 0i64, f184(v1, false)?)))
}

pub fn f127() -> R<Adt105> {
    Ok(Adt105::C1(Adt106::C0, f123(Str::lit("Both inputs received"), false), f183(0i64, List::cons((Str::lit("first"), f185(Str::lit("First name"), Str::lit("name"), Str::lit("string"))), List::cons((Str::lit("second"), f185(Str::lit("Second name"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt69)>::nil())), 0i64, f184(Str::lit("Both inputs received"), false)?)))
}

pub fn f128() -> R<Adt105> {
    Ok(Adt105::C1(Adt106::C1, f123(Str::lit("failing_job always fails"), true), f183(0i64, List::<(Str, Adt69)>::nil(), 1000i64, f184(Str::lit("failing_job always fails"), true)?)))
}

pub fn f129() -> Adt105 {
    Adt105::C1(Adt106::C0, f123(Str::lit("protocol_error_job has no result"), true), f183(0i64, List::<(Str, Adt69)>::nil(), 500i64, Adt91::C1(Adt85::C0(-32603i64, Str::lit("protocol_error_job failed"), None::<Adt9>))))
}

pub fn f130(v0: Adt26) -> R<Adt105> {
    let m432 = match v0 {
        Adt26::C0(_, _, _, h431, _) => {
            h431
        }
    };
    match m432 {
        None => {
            Ok(f186())
        }
        Some(v1) => {
            let m433 = f187(v1);
            match m433 {
                None => {
                    Ok(f186())
                }
                Some(v2) => {
                    f188(v2)
                }
            }
        }
    }
}

pub fn f131(v0: Adt13, v1: List<Adt40>, v2: Adt25) -> Result<Adt37, Adt94> {
    let m435 = match v2 {
        Adt25::C0(_, h434) => {
            h434
        }
    };
    match m435 {
        None => {
            Ok::<Adt37, Adt94>(Adt37::C1(Adt39::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt37, Adt94>(Adt94::C5(v3))
        }
    }
}

pub fn f132(v0: Str) -> bool {
    let a436 = v0.clone();
    let a437 = Str::lit("test_input_required_result_elicitation");
    if equal(a436, a437) {
        true
    } else {
        let a438 = v0.clone();
        let a439 = Str::lit("test_input_required_result_sampling");
        if equal(a438, a439) {
            true
        } else {
            let a440 = v0.clone();
            let a441 = Str::lit("test_input_required_result_list_roots");
            if equal(a440, a441) {
                true
            } else {
                let a442 = v0.clone();
                let a443 = Str::lit("test_input_required_result_request_state");
                if equal(a442, a443) {
                    true
                } else {
                    let a444 = v0.clone();
                    let a445 = Str::lit("test_input_required_result_multiple_inputs");
                    if equal(a444, a445) {
                        true
                    } else {
                        let a446 = v0.clone();
                        let a447 = Str::lit("test_input_required_result_multi_round");
                        if equal(a446, a447) {
                            true
                        } else {
                            let a448 = v0.clone();
                            let a449 = Str::lit("test_input_required_result_tampered_state");
                            if equal(a448, a449) {
                                true
                            } else {
                                let a450 = v0;
                                let a451 = Str::lit("test_input_required_result_capabilities");
                                equal(a450, a451)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f133(v0: List<Adt40>, v1: Fn5, v2: Adt26) -> R<Result<Adt37, Adt94>> {
    let m453 = f82(match v2.clone() {
        Adt26::C0(_, h452, _, _, _) => {
            h452
        }
    }, v0);
    match m453 {
        None => {
            Ok(Err::<Adt37, Adt94>(Adt94::C0(match v2 {
                Adt26::C0(_, h454, _, _, _) => {
                    h454
                }
            })))
        }
        Some(v3) => {
            f116(v3.clone(), v2, Fn4::F189(v1, v3))
        }
    }
}

pub fn f134(v0: Adt26) -> R<Result<Adt108, Adt94>> {
    if f190(v0.clone(), Str::lit("test_input_required_result_elicitation")) {
        Ok(f191(v0))
    } else {
        if f190(v0.clone(), Str::lit("test_input_required_result_sampling")) {
            Ok(f192(v0))
        } else {
            if f190(v0.clone(), Str::lit("test_input_required_result_list_roots")) {
                f193(v0)
            } else {
                if f190(v0.clone(), Str::lit("test_input_required_result_request_state")) {
                    Ok(f194(v0))
                } else {
                    if f190(v0.clone(), Str::lit("test_input_required_result_multiple_inputs")) {
                        f195(v0)
                    } else {
                        if f190(v0.clone(), Str::lit("test_input_required_result_multi_round")) {
                            f196(v0)
                        } else {
                            if f190(v0.clone(), Str::lit("test_input_required_result_tampered_state")) {
                                Ok(f197(v0))
                            } else {
                                if f190(v0.clone(), Str::lit("test_input_required_result_capabilities")) {
                                    Ok(f198(v0))
                                } else {
                                    Ok(Err::<Adt108, Adt94>(Adt94::C9(Str::lit("not an input-required tool"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f135(v0: Adt26) -> R<Result<Adt54, Adt94>> {
    if f42(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt54, Adt94>(f199(List::cons(f95(Str::lit("This is a simple text response for testing.")), List::<Adt43>::nil()))))
    } else {
        if f42(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt54, Adt94>(f199(List::cons(f200(), List::<Adt43>::nil()))))
        } else {
            if f42(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt54, Adt94>(f199(List::cons(f201(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt43>::nil()))))
            } else {
                if f42(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt54, Adt94>(f199(List::cons(f202(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt43>::nil()))))
                } else {
                    if f42(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt54, Adt94>(f199(List::cons(f95(Str::lit("Multiple content types test:")), List::cons(f200(), List::cons(f202(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt43>::nil()))))))
                    } else {
                        if f42(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt54, Adt94>(f203(List::cons(f95(Str::lit("This tool intentionally returns an error for testing")), List::<Adt43>::nil()), None::<Adt9>, true, List::<Adt55>::nil())))
                        } else {
                            if f42(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt54, Adt94>(f204(v0, f205()?)))
                            } else {
                                if f42(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt54, Adt94>(f204(v0, f206()?)))
                                } else {
                                    if f42(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt54, Adt94>(f203(List::cons(f95(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt43>::nil()), Some(f207()?), false, List::<Adt55>::nil())))
                                    } else {
                                        if f42(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt54, Adt94>(f203(List::cons(f95(Str::lit("{\"sum\":\"three\"}")), List::<Adt43>::nil()), Some(f208()), false, List::<Adt55>::nil())))
                                        } else {
                                            if f42(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt54, Adt94>(f199(List::cons(f209(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt43>::nil()))))
                                            } else {
                                                if f42(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt54, Adt94>(f199(List::cons(f95(f210()), List::<Adt43>::nil()))))
                                                } else {
                                                    if f42(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt54, Adt94>(f199(List::cons(f95(Str::lit("Streaming complete")), List::<Adt43>::nil()))))
                                                    } else {
                                                        if f42(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt54, Adt94>(f199(List::cons(f95(Str::lit("Logging evaluated")), List::<Adt43>::nil()))))
                                                        } else {
                                                            Ok(f211(v0))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f136(v0: Adt25) -> Result<Adt37, Adt94> {
    f212(f213(), f214(), v0)
}

pub fn f137(v0: Adt27) -> R<Result<Adt37, Adt94>> {
    f215(f216(), Fn6::F217, v0)
}

pub fn f138(v0: Adt25) -> Result<Adt37, Adt94> {
    f218(f213(), f219(), v0)
}

pub fn f139(v0: Adt25) -> Result<Adt37, Adt94> {
    f220(f213(), f221(), v0)
}

pub fn f140(v0: Str) -> bool {
    let a455 = v0;
    let a456 = f222();
    equal(a455, a456)
}

pub fn f141(v0: Adt28) -> Result<Adt37, Adt94> {
    let m458 = f223(Str::lit("user_context"), Adt109::C0, match v0 {
        Adt28::C0(_, _, _, h457, _) => {
            h457
        }
    });
    match m458 {
        Adt119::C0 => {
            Ok::<Adt37, Adt94>(Adt37::C12(Adt86::C0(f226(Str::lit("user_context"), f227(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt119::C1 => {
            Err::<Adt37, Adt94>(Adt94::C11)
        }
        Adt119::C2(v1) => {
            Ok::<Adt37, Adt94>(Adt37::C8(f224(f225(v1, Str::lit("context"), Str::lit("unknown")))))
        }
    }
}

pub fn f142(v0: Adt28) -> Result<Adt37, Adt94> {
    f228(f221(), Fn7::F229, v0)
}

pub fn f143(v0: Adt29) -> R<Result<Adt37, Adt94>> {
    f230(Fn8::F231, v0)
}

pub fn f144(v0: Adt18, v1: Adt21, v2: Str) -> R<Result<Adt89, Adt94>> {
    if f232(match v1 {
        Adt21::C0(_, h459, _, _, _, _) => {
            h459
        }
    }) {
        let m461 = f233(match v0.clone() {
            Adt18::C0(_, _, _, h460) => {
                h460
            }
        })?;
        match m461 {
            None => {
                Ok(Err::<Adt89, Adt94>(f234()))
            }
            Some(v3) => {
                let m463 = f235(v2.clone(), v3, match v0 {
                    Adt18::C0(h462, _, _, _) => {
                        h462
                    }
                })?;
                match m463 {
                    None => {
                        Ok(Err::<Adt89, Adt94>(Adt94::C3(v2)))
                    }
                    Some(v4) => {
                        Ok(Ok::<Adt89, Adt94>(v4))
                    }
                }
            }
        }
    } else {
        Ok(Err::<Adt89, Adt94>(f236()))
    }
}

pub fn f145(v0: Adt18, v1: Adt89) -> R<Result<Adt37, Adt94>> {
    Ok(Ok::<Adt37, Adt94>(f237(f238(v1, match v0.clone() {
        Adt18::C0(h464, _, _, _) => {
            h464
        }
    }, match v0 {
        Adt18::C0(_, h465, _, _) => {
            h465
        }
    })?, Adt93::C0, false)))
}

pub fn f146(v0: Adt18, v1: Adt89, v2: Adt8) -> R<Result<Adt37, Adt94>> {
    let v3: Adt110 = f238(v1, match v0.clone() {
        Adt18::C0(h466, _, _, _) => {
            h466
        }
    }, match v0.clone() {
        Adt18::C0(_, h467, _, _) => {
            h467
        }
    })?;
    Ok(Ok::<Adt37, Adt94>(f237(f239(match v3.clone() {
        Adt110::C0(h468, _) => {
            h468
        }
    }, v2, match v0.clone() {
        Adt18::C0(h469, _, _, _) => {
            h469
        }
    }, match v0 {
        Adt18::C0(_, h470, _, _) => {
            h470
        }
    })?, Adt93::C1, match v3 {
        Adt110::C0(_, h471) => {
            h471
        }
    })))
}

pub fn f147(v0: Adt18, v1: Adt89) -> R<Result<Adt37, Adt94>> {
    Ok(Ok::<Adt37, Adt94>(f237(f240(v1, match v0.clone() {
        Adt18::C0(h472, _, _, _) => {
            h472
        }
    }, match v0 {
        Adt18::C0(_, h473, _, _) => {
            h473
        }
    })?, Adt93::C1, false)))
}

pub fn f148(v0: Bytes, p483: u64, p484: u64, p485: List<Adt101>) -> R<Result<(List<Adt101>, u64), Adt2>> {
    let m482 = (p483, (p484, p485));
    {
        let mut m482 = m482;
        loop {
            let step = {
                let (v1, (v2, v3)) = m482;
                let m474 = v2;
                if m474 == 0 {
                    Err::<(u64, (u64, List<Adt101>)), R<Result<(List<Adt101>, u64), Adt2>>>(Ok(Ok::<(List<Adt101>, u64), Adt2>((f241(v3, List::<Adt101>::nil()), v1))))
                } else {
                    let v4 = m474 - 1;
                    let m475 = f7(v0.clone(), v1)?;
                    match m475 {
                        None => {
                            Err::<(u64, (u64, List<Adt101>)), R<Result<(List<Adt101>, u64), Adt2>>>(Ok(Err::<(List<Adt101>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            let m477 = f7(v0.clone(), match v5.clone() {
                                Adt96::C0(_, h476) => {
                                    h476
                                }
                            })?;
                            match m477 {
                                None => {
                                    Err::<(u64, (u64, List<Adt101>)), R<Result<(List<Adt101>, u64), Adt2>>>(Ok(Err::<(List<Adt101>, u64), Adt2>(Adt2::C0(match v5 {
                                        Adt96::C0(_, h478) => {
                                            h478
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt101>)), R<Result<(List<Adt101>, u64), Adt2>>>((match v6.clone() {
                                        Adt96::C0(_, h479) => {
                                            h479
                                        }
                                    }, (v4, List::cons(Adt101::C0(f29(match v5 {
                                        Adt96::C0(h480, _) => {
                                            h480
                                        }
                                    })?, f29(match v6 {
                                        Adt96::C0(h481, _) => {
                                            h481
                                        }
                                    })?), v3))))
                                }
                            }
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

pub fn f149(v0: Bytes, v1: Adt96, v2: Adt96, v3: (List<Adt101>, u64)) -> R<Result<Adt100, Adt2>> {
    let m487 = f7(v0.clone(), {
        let (_, h486) = v3.clone();
        h486
    })?;
    match m487 {
        None => {
            Ok(Err::<Adt100, Adt2>(Adt2::C0({
                let (_, h488) = v3;
                h488
            })))
        }
        Some(v4) => {
            let m490 = f27(v0.clone(), match v4.clone() {
                Adt96::C0(_, h489) => {
                    h489
                }
            })?;
            match m490 {
                None => {
                    Ok(Err::<Adt100, Adt2>(Adt2::C0(match v4 {
                        Adt96::C0(_, h491) => {
                            h491
                        }
                    })))
                }
                Some(v5) => {
                    f242(v0.clone(), Adt100::C0(f29(match v1 {
                        Adt96::C0(h492, _) => {
                            h492
                        }
                    })?, f29(match v2 {
                        Adt96::C0(h493, _) => {
                            h493
                        }
                    })?, {
                        let (h494, _) = v3;
                        h494
                    }, match v4.clone() {
                        Adt96::C0(h495, _) => {
                            h495
                        }
                    }, v5, List::<List<u64>>::nil(), Bytes::lit(&[]), Adt18::C0(0i64, Str::lit(""), Str::lit(""), f243())), f244(v0, {
                        let a497 = match v4 {
                            Adt96::C0(_, h496) => {
                                h496
                            }
                        };
                        let a498 = 4u64;
                        nat_add(a497, a498)?
                    })?)
                }
            }
        }
    }
}

pub fn f150(p503: List<Adt101>, p504: List<u64>) -> List<u64> {
    let m502 = (p503, p504);
    {
        let mut m502 = m502;
        loop {
            let step = {
                let (v0, v1) = m502;
                let m499 = v0;
                match m499.uncons() {
                    None => {
                        Err::<(List<Adt101>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt101>, List<u64>), List<u64>>((v3, {
                            let a500 = f245(v2);
                            let a501 = v1;
                            append_list(a500, a501)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(p507: List<Adt101>, p508: List<Adt101>) -> List<Adt101> {
    let m506 = (p507, p508);
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v0, v1) = m506;
                let m505 = v0;
                match m505.uncons() {
                    None => {
                        Err::<(List<Adt101>, List<Adt101>), List<Adt101>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt101>, List<Adt101>), List<Adt101>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(p513: List<List<u64>>, p514: List<u64>) -> List<u64> {
    let m512 = (p513, p514);
    {
        let mut m512 = m512;
        loop {
            let step = {
                let (v0, v1) = m512;
                let m509 = v0;
                match m509.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a510 = f246(v2);
                            let a511 = v1;
                            append_list(a510, a511)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(p517: List<List<u64>>, p518: List<List<u64>>) -> List<List<u64>> {
    let m516 = (p517, p518);
    {
        let mut m516 = m516;
        loop {
            let step = {
                let (v0, v1) = m516;
                let m515 = v0;
                match m515.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
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

pub fn f154() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f155() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f156() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f157() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f158() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f159(v0: Adt107) -> R<Adt104> {
    f247(f248(v0.clone()), f249(None::<Adt111>, f250(v0)))
}

pub fn f160(v0: Adt3, v1: Fn1, v2: Adt100) -> R<Adt104> {
    if f251(match v2.clone() {
        Adt100::C0(h519, _, _, _, _, _, _, _) => {
            h519
        }
    }) {
        f252(match v2.clone() {
            Adt100::C0(_, _, _, _, _, _, _, h520) => {
                h520
            }
        }, f253(v0, match v2.clone() {
            Adt100::C0(_, _, _, _, _, _, h521, _) => {
                h521
            }
        }), Fn9::F254(v1, v2.clone()), match v2.clone() {
            Adt100::C0(_, _, _, _, h522, _, _, _) => {
                h522
            }
        }, match v2.clone() {
            Adt100::C0(_, _, h523, _, _, _, _, _) => {
                h523
            }
        }, f255(match v2 {
            Adt100::C0(_, _, _, h524, _, _, _, _) => {
                h524
            }
        })?)
    } else {
        Ok(f256())
    }
}

pub fn f161(v0: List<Adt101>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m525 = f257(v0, f258())?;
    match m525 {
        Adt123::C0 => {
            Ok(None::<Adt107>)
        }
        Adt123::C1 => {
            Ok(Some(Adt107::C0))
        }
        Adt123::C2 => {
            Ok(Some(Adt107::C0))
        }
        Adt123::C3(v2) => {
            f259(f260(v2), v1, Adt107::C0)
        }
    }
}

pub fn f162(v0: List<Adt101>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m526 = f257(v0, f261())?;
    match m526 {
        Adt123::C0 => {
            Ok(None::<Adt107>)
        }
        Adt123::C1 => {
            Ok(Some(Adt107::C1))
        }
        Adt123::C2 => {
            Ok(Some(Adt107::C1))
        }
        Adt123::C3(v2) => {
            f259(f262(v2), v1, Adt107::C1)
        }
    }
}

pub fn f163(v0: Adt3, v1: Fn1, v2: bool, v3: Adt100) -> R<Adt97> {
    if f251(match v3.clone() {
        Adt100::C0(h527, _, _, _, _, _, _, _) => {
            h527
        }
    }) {
        f263(match v3.clone() {
            Adt100::C0(_, _, _, _, _, _, _, h528) => {
                h528
            }
        }, f253(v0, match v3.clone() {
            Adt100::C0(_, _, _, _, _, _, h529, _) => {
                h529
            }
        }), Fn9::F264(v1, v3.clone()), v2, match v3.clone() {
            Adt100::C0(_, _, _, _, h530, _, _, _) => {
                h530
            }
        }, match v3.clone() {
            Adt100::C0(_, _, h531, _, _, _, _, _) => {
                h531
            }
        }, f255(match v3 {
            Adt100::C0(_, _, _, h532, _, _, _, _) => {
                h532
            }
        })?)
    } else {
        Ok(Adt97::C0(f56(f256())))
    }
}

pub fn f164(v0: Str, p537: List<Str>) -> bool {
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

pub fn f165(v0: Str, v1: List<u64>) -> Option<Adt102> {
    if f265(v1) {
        Some(Adt102::C0(v0))
    } else {
        None::<Adt102>
    }
}

pub fn f166(v0: Str) -> R<List<u64>> {
    f29({
        let a538 = v0;
        utf8_encode(a538)
    })
}

pub fn f167(v0: Adt40) -> R<Result<List<Adt15>, Adt102>> {
    let m539 = f266(v0.clone());
    match m539 {
        None => {
            f267(v0.clone(), f268(f269(), match v0.clone() {
                Adt40::C0(h540, _, _, _, _, _, _, _) => {
                    h540
                }
            }, Adt9::C5({
                let m543 = match v0.clone() {
                    Adt40::C0(_, _, _, _, h542, _, _, _) => {
                        h542
                    }
                };
                match m543 {
                    Adt8::C0(h541) => {
                        h541
                    }
                }
            }), List::cons(Adt112::C0(Adt9::C5({
                let m546 = match v0 {
                    Adt40::C0(_, _, _, _, h545, _, _, _) => {
                        h545
                    }
                };
                match m546 {
                    Adt8::C0(h544) => {
                        h544
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt112>::nil()), f270())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt102>(v1))
        }
    }
}

pub fn f168(p551: u64, p552: i64) -> R<i64> {
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

pub fn f169(v0: Adt9, v1: Adt9, v2: Adt9) -> R<Adt113> {
    f271(v0, f272(), Adt115::C0, List::cons(Adt116::C0(v1, v2, List::<Adt114>::nil()), List::<Adt116>::nil()))
}

pub fn f170(v0: Adt26) -> Adt9 {
    let m554 = match v0 {
        Adt26::C0(_, _, h553, _, _) => {
            h553
        }
    };
    match m554 {
        None => {
            Adt9::C5(List::<(Str, Adt9)>::nil())
        }
        Some(v1) => {
            Adt9::C5(match v1 {
                Adt8::C0(h555) => {
                    h555
                }
            })
        }
    }
}

pub fn f171(v0: List<Adt114>, v1: Str) -> R<Str> {
    let v2: Str = f273(v0)?;
    let a558 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a556 = v2.clone();
        let a557 = Str::lit("");
        if equal(a556, a557) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a559 = Str::lit(" ");
    Ok(join(a558, a559))
}

pub fn f172(v0: Adt40, v1: Fn2, v2: Adt26) -> R<Result<Adt37, Adt94>> {
    let m561 = {
        let c560: Fn2 = v1;
        c560.apply(v2)?
    };
    match m561 {
        Ok(v4) => {
            f274(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt37, Adt94>(v3))
        }
    }
}

pub fn f173(v0: u64, v1: Str) -> Result<Adt54, Adt94> {
    Err::<Adt54, Adt94>(Adt94::C15(f45(v0, v1)))
}

pub fn f174(v0: Str, v1: Adt26) -> Str {
    let m563 = match v1 {
        Adt26::C0(_, _, h562, _, _) => {
            h562
        }
    };
    match m563 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m564 = f275(v0, v2);
            match m564 {
                Adt118::C0 => {
                    Str::lit("")
                }
                Adt118::C1(v3) => {
                    let m565 = f181(v3);
                    match m565 {
                        None => {
                            Str::lit("")
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

pub fn f175(v0: Str) -> Result<Adt54, Adt94> {
    Ok::<Adt54, Adt94>(f199(List::cons(f95(v0), List::<Adt43>::nil())))
}

pub fn f176() -> Result<Adt54, Adt94> {
    Err::<Adt54, Adt94>(f49())
}

pub fn f177(v0: Adt18, v1: Adt40, v2: Adt26, v3: Adt105, v4: Result<Option<Adt37>, Adt94>) -> R<Result<Adt37, Adt94>> {
    let m566 = v4;
    match m566 {
        Ok(v6) => {
            let m567 = v6;
            match m567 {
                None => {
                    f276(v0, v1, v2, v3)
                }
                Some(v7) => {
                    Ok(Ok::<Adt37, Adt94>(v7))
                }
            }
        }
        Err(v5) => {
            Ok(Err::<Adt37, Adt94>(v5))
        }
    }
}

pub fn f178(v0: Adt40, v1: Adt26) -> R<Result<Option<Adt37>, Adt94>> {
    let m574 = f169(Adt9::C5({
        let m570 = match v0.clone() {
            Adt40::C0(_, _, _, _, h569, _, _, _) => {
                h569
            }
        };
        match m570 {
            Adt8::C0(h568) => {
                h568
            }
        }
    }), Adt9::C5({
        let m573 = match v0 {
            Adt40::C0(_, _, _, _, h572, _, _, _) => {
                h572
            }
        };
        match m573 {
            Adt8::C0(h571) => {
                h571
            }
        }
    }), f170(v1))?;
    match m574 {
        Adt113::C0 => {
            Ok(Ok::<Option<Adt37>, Adt94>(None::<Adt37>))
        }
        Adt113::C1(v2, v3) => {
            Ok(Ok::<Option<Adt37>, Adt94>(Some(Adt37::C2(f87(f171(v2, v3)?, true)))))
        }
        Adt113::C2(_) => {
            Ok(Err::<Option<Adt37>, Adt94>(Adt94::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt113::C3(_) => {
            Ok(Err::<Option<Adt37>, Adt94>(Adt94::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt113::C4 => {
            Ok(Err::<Option<Adt37>, Adt94>(Adt94::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f179(v0: Str) -> Adt43 {
    Adt43::C0(Adt44::C0(v0, None::<Adt45>, None::<Adt8>))
}

pub fn f180(v0: Adt26, v1: Str) -> Adt118 {
    let m576 = match v0 {
        Adt26::C0(_, _, h575, _, _) => {
            h575
        }
    };
    match m576 {
        None => {
            Adt118::C0
        }
        Some(v2) => {
            f275(v1, v2)
        }
    }
}

pub fn f181(v0: Adt9) -> Option<Str> {
    let m577 = v0;
    match m577 {
        Adt9::C0 => {
            None::<Str>
        }
        Adt9::C1(_) => {
            None::<Str>
        }
        Adt9::C2(_) => {
            None::<Str>
        }
        Adt9::C3(v3) => {
            Some(v3)
        }
        Adt9::C4(_) => {
            None::<Str>
        }
        Adt9::C5(_) => {
            None::<Str>
        }
    }
}

pub fn f182(v0: Adt26) -> i64 {
    let m578 = f180(v0, Str::lit("seconds"));
    match m578 {
        Adt118::C0 => {
            0i64
        }
        Adt118::C1(v1) => {
            let m579 = f277(v1);
            match m579 {
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

pub fn f183(v0: i64, v1: List<(Str, Adt69)>, v2: i64, v3: Adt91) -> Adt90 {
    Adt90::C0(v0, v1, v2, v3)
}

pub fn f184(v0: Str, v1: bool) -> R<Adt91> {
    let m582 = f278(f279({
        let m581 = f123(v0, v1);
        match m581 {
            Adt54::C0(h580, _) => {
                h580
            }
        }
    })?)?;
    match m582 {
        None => {
            Ok(f280())
        }
        Some(v2) => {
            Ok(Adt91::C0(Adt8::C0(v2)))
        }
    }
}

pub fn f185(v0: Str, v1: Str, v2: Str) -> Adt69 {
    Adt69::C0(Adt70::C0(Adt71::C0(Some(Adt72::C0), v0, f114(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((v1.clone(), f115(v2)), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(v1), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())))))))
}

pub fn f186() -> Adt105 {
    Adt105::C2(Adt86::C0(List::cons((Str::lit("user_name"), f185(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt69)>::nil())))
}

pub fn f187(v0: Adt8) -> Option<Str> {
    let m583 = f275(Str::lit("user_name"), v0);
    match m583 {
        Adt118::C0 => {
            None::<Str>
        }
        Adt118::C1(v1) => {
            let m584 = f281(v1);
            match m584 {
                None => {
                    None::<Str>
                }
                Some(v2) => {
                    let m585 = f275(Str::lit("content"), v2);
                    match m585 {
                        Adt118::C0 => {
                            None::<Str>
                        }
                        Adt118::C1(v3) => {
                            let m586 = f281(v3);
                            match m586 {
                                None => {
                                    None::<Str>
                                }
                                Some(v4) => {
                                    let m587 = f275(Str::lit("name"), v4);
                                    match m587 {
                                        Adt118::C0 => {
                                            None::<Str>
                                        }
                                        Adt118::C1(v5) => {
                                            f181(v5)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f188(v0: Str) -> R<Adt105> {
    let v1: Str = {
        let a588 = List::cons(Str::lit("Hello, "), List::cons(v0, List::cons(Str::lit("! (from a task)"), List::<Str>::nil())));
        let a589 = Str::lit("");
        join(a588, a589)
    };
    Ok(Adt105::C1(Adt106::C1, f123(v1.clone(), false), f183(0i64, List::<(Str, Adt69)>::nil(), 100i64, f184(v1, false)?)))
}

pub fn f189(v0: Fn5, v1: Adt40, v2: Adt26) -> R<Result<Adt37, Adt94>> {
    f282(v1, {
        let c590: Fn5 = v0;
        c590.apply(v2)?
    })
}

pub fn f190(v0: Adt26, v1: Str) -> bool {
    let a592 = match v0 {
        Adt26::C0(_, h591, _, _, _) => {
            h591
        }
    };
    let a593 = v1;
    equal(a592, a593)
}

pub fn f191(v0: Adt26) -> Result<Adt108, Adt94> {
    let m595 = f223(Str::lit("user_name"), Adt109::C0, match v0 {
        Adt26::C0(_, _, _, h594, _) => {
            h594
        }
    });
    match m595 {
        Adt119::C0 => {
            f285(f226(Str::lit("user_name"), f286()))
        }
        Adt119::C1 => {
            f287()
        }
        Adt119::C2(v1) => {
            f283(f284(List::cons(Str::lit("Hello, "), List::cons(f225(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f192(v0: Adt26) -> Result<Adt108, Adt94> {
    let m597 = f223(Str::lit("capital_question"), Adt109::C1, match v0 {
        Adt26::C0(_, _, _, h596, _) => {
            h596
        }
    });
    match m597 {
        Adt119::C0 => {
            f285(f226(Str::lit("capital_question"), f288(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt119::C1 => {
            f287()
        }
        Adt119::C2(v1) => {
            f283(f284(List::cons(Str::lit("Sampling result: "), List::cons(f225(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f193(v0: Adt26) -> R<Result<Adt108, Adt94>> {
    let m599 = f223(Str::lit("client_roots"), Adt109::C2, match v0 {
        Adt26::C0(_, _, _, h598, _) => {
            h598
        }
    });
    match m599 {
        Adt119::C0 => {
            Ok(f285(f226(Str::lit("client_roots"), f291())))
        }
        Adt119::C1 => {
            Ok(f287())
        }
        Adt119::C2(v1) => {
            Ok(f283(f284(List::cons(Str::lit("Found "), List::cons(f289(f290(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f194(v0: Adt26) -> Result<Adt108, Adt94> {
    f292(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f195(v0: Adt26) -> R<Result<Adt108, Adt94>> {
    if f293(v0.clone(), Str::lit("multiple-inputs")) {
        f294(f223(Str::lit("user_name"), Adt109::C0, match v0.clone() {
            Adt26::C0(_, _, _, h600, _) => {
                h600
            }
        }), f223(Str::lit("greeting"), Adt109::C1, match v0.clone() {
            Adt26::C0(_, _, _, h601, _) => {
                h601
            }
        }), f223(Str::lit("client_roots"), Adt109::C2, match v0 {
            Adt26::C0(_, _, _, h602, _) => {
                h602
            }
        }))
    } else {
        Ok(f295(f296(Adt119::C0, Adt119::C0, Adt119::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f196(v0: Adt26) -> R<Result<Adt108, Adt94>> {
    let v1: u64 = f297(f298(v0.clone()))?;
    let a603 = v1;
    let a604 = 49u64;
    if nat_eq(a603, a604) {
        Ok(f299(v0))
    } else {
        let a605 = v1;
        let a606 = 50u64;
        if nat_eq(a605, a606) {
            Ok(f300(v0))
        } else {
            Ok(f295(f226(Str::lit("step1"), f301()), Str::lit("1")))
        }
    }
}

pub fn f197(v0: Adt26) -> Result<Adt108, Adt94> {
    f292(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f198(v0: Adt26) -> Result<Adt108, Adt94> {
    let m608 = match v0.clone() {
        Adt26::C0(_, _, _, h607, _) => {
            h607
        }
    };
    match m608 {
        None => {
            f302(f303({
                let m611 = match v0 {
                    Adt26::C0(h610, _, _, _, _) => {
                        h610
                    }
                };
                match m611 {
                    Adt21::C0(_, h609, _, _, _, _) => {
                        h609
                    }
                }
            }))
        }
        Some(v1) => {
            let m613 = match v1.clone() {
                Adt8::C0(h612) => {
                    h612
                }
            };
            match m613.uncons() {
                None => {
                    f302(f303({
                        let m616 = match v0 {
                            Adt26::C0(h615, _, _, _, _) => {
                                h615
                            }
                        };
                        match m616 {
                            Adt21::C0(_, h614, _, _, _, _) => {
                                h614
                            }
                        }
                    }))
                }
                Some((_, _)) => {
                    f283(f284(List::cons(Str::lit("capabilities-ok: received "), List::cons({
                        let a618 = f304(match v1 {
                            Adt8::C0(h617) => {
                                h617
                            }
                        });
                        let a619 = Str::lit(",");
                        join(a618, a619)
                    }, List::<Str>::nil()))))
                }
            }
        }
    }
}

pub fn f199(v0: List<Adt43>) -> Adt54 {
    f203(v0, None::<Adt9>, false, List::<Adt55>::nil())
}

pub fn f200() -> Adt43 {
    f209(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f201(v0: Str, v1: Str) -> Adt43 {
    Adt43::C2(Adt48::C0(v0, v1, None::<Adt45>, None::<Adt8>))
}

pub fn f202(v0: Str, v1: Str, v2: Str) -> Adt43 {
    Adt43::C4(Adt50::C0(Adt51::C0(Adt52::C0(v0, Some(v1), v2, None::<Adt8>)), None::<Adt45>, None::<Adt8>))
}

pub fn f203(v0: List<Adt43>, v1: Option<Adt9>, v2: bool, v3: List<Adt55>) -> Adt54 {
    Adt54::C0(Adt42::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f204(v0: Adt26, v1: List<Adt55>) -> Adt54 {
    f203(List::cons(f95(f305({
        let m622 = match v0 {
            Adt26::C0(h621, _, _, _, _) => {
                h621
            }
        };
        match m622 {
            Adt21::C0(_, _, _, _, h620, _) => {
                h620
            }
        }
    })), List::<Adt43>::nil()), None::<Adt9>, false, v1)
}

pub fn f205() -> R<List<Adt55>> {
    Ok(List::cons(f306(0u64, 100u64)?, List::cons(f306(50u64, 100u64)?, List::cons(f306(100u64, 100u64)?, List::<Adt55>::nil()))))
}

pub fn f206() -> R<List<Adt55>> {
    Ok(List::cons(f306(50u64, 100u64)?, List::cons(f306(10u64, 100u64)?, List::<Adt55>::nil())))
}

pub fn f207() -> R<Adt9> {
    Ok(Adt9::C5(List::cons((Str::lit("sum"), f113(3u64)?), List::cons((Str::lit("label"), Adt9::C3(Str::lit("three"))), List::<(Str, Adt9)>::nil()))))
}

pub fn f208() -> Adt9 {
    Adt9::C5(List::cons((Str::lit("sum"), Adt9::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt9::C3(Str::lit("three"))), List::<(Str, Adt9)>::nil())))
}

pub fn f209(v0: Str, v1: Str) -> Adt43 {
    Adt43::C1(Adt47::C0(v0, v1, None::<Adt45>, None::<Adt8>))
}

pub fn f210() -> Str {
    let m624 = {
        let a623 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a623)
    };
    match m624 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f211(v0: Adt26) -> Result<Adt54, Adt94> {
    if f42(v0.clone(), Str::lit("test_missing_capability")) {
        let m630 = {
            let m629 = {
                let m628 = match v0 {
                    Adt26::C0(h627, _, _, _, _) => {
                        h627
                    }
                };
                match m628 {
                    Adt21::C0(_, h626, _, _, _, _) => {
                        h626
                    }
                }
            };
            match m629 {
                Adt22::C0(_, _, h625, _, _) => {
                    h625
                }
            }
        };
        match m630 {
            None => {
                Err::<Adt54, Adt94>(Adt94::C8(f307()))
            }
            Some(_) => {
                Ok::<Adt54, Adt94>(f308())
            }
        }
    } else {
        if f42(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt54, Adt94>(f199(List::cons(f95(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt43>::nil())))
        } else {
            if f42(v0, Str::lit("test_arguments")) {
                Ok::<Adt54, Adt94>(f199(List::cons(f95(Str::lit("arguments accepted")), List::<Adt43>::nil())))
            } else {
                Ok::<Adt54, Adt94>(f308())
            }
        }
    }
}

pub fn f212(v0: Adt13, v1: List<Adt49>, v2: Adt25) -> Result<Adt37, Adt94> {
    let m632 = match v2 {
        Adt25::C0(_, h631) => {
            h631
        }
    };
    match m632 {
        None => {
            Ok::<Adt37, Adt94>(Adt37::C4(Adt56::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt37, Adt94>(Adt94::C5(v3))
        }
    }
}

pub fn f213() -> Adt13 {
    Adt13::C0(300000u64, Adt14::C0)
}

pub fn f214() -> List<Adt49> {
    f309(f310())
}

pub fn f215(v0: Adt13, v1: Fn6, v2: Adt27) -> R<Result<Adt37, Adt94>> {
    Ok(f311(v0, match v2.clone() {
        Adt27::C0(_, h633, _, _) => {
            h633
        }
    }, {
        let c634: Fn6 = v1;
        c634.apply(match v2 {
            Adt27::C0(_, h635, _, _) => {
                h635
            }
        })?
    }))
}

pub fn f216() -> Adt13 {
    Adt13::C0(300000u64, Adt14::C1)
}

pub fn f217(v0: Str) -> R<Option<List<Adt51>>> {
    let m636 = f312(v0.clone(), f310());
    match m636 {
        None => {
            f313(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f218(v0: Adt13, v1: List<Adt59>, v2: Adt25) -> Result<Adt37, Adt94> {
    let m638 = match v2 {
        Adt25::C0(_, h637) => {
            h637
        }
    };
    match m638 {
        None => {
            Ok::<Adt37, Adt94>(Adt37::C6(Adt58::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt37, Adt94>(Adt94::C5(v3))
        }
    }
}

pub fn f219() -> List<Adt59> {
    List::cons(Adt59::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt5>>, f314(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt45>, None::<Adt8>), List::<Adt59>::nil())
}

pub fn f220(v0: Adt13, v1: List<Adt61>, v2: Adt25) -> Result<Adt37, Adt94> {
    let m640 = match v2 {
        Adt25::C0(_, h639) => {
            h639
        }
    };
    match m640 {
        None => {
            Ok::<Adt37, Adt94>(Adt37::C7(Adt60::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt37, Adt94>(Adt94::C5(v3))
        }
    }
}

pub fn f221() -> List<Adt61> {
    List::cons(f315(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt62>>), List::cons(f315(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f316(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f316(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt62>::nil())))), List::cons(f315(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f316(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt62>::nil()))), List::cons(f315(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt62>>), List::<Adt61>::nil()))))
}

pub fn f222() -> Str {
    Str::lit("test_input_required_result_prompt")
}

pub fn f223(v0: Str, v1: Adt109, v2: Option<Adt8>) -> Adt119 {
    let m641 = v2;
    match m641 {
        None => {
            Adt119::C0
        }
        Some(v3) => {
            let m642 = f275(v0, v3);
            match m642 {
                Adt118::C0 => {
                    Adt119::C0
                }
                Adt118::C1(v4) => {
                    let m643 = f281(v4);
                    match m643 {
                        None => {
                            Adt119::C1
                        }
                        Some(v5) => {
                            if f317(v1, v5.clone()) {
                                Adt119::C2(v5)
                            } else {
                                Adt119::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f224(v0: Str) -> Adt63 {
    Adt63::C0(None::<Str>, List::cons(Adt64::C0(Adt46::C0, Adt43::C0(Adt44::C0(f284(List::cons(Str::lit("Prompt with context: "), List::cons(v0, List::<Str>::nil()))), None::<Adt45>, None::<Adt8>))), List::<Adt64>::nil()))
}

pub fn f225(v0: Adt8, v1: Str, v2: Str) -> Str {
    let m644 = f275(Str::lit("content"), v0);
    match m644 {
        Adt118::C0 => {
            v2
        }
        Adt118::C1(v3) => {
            let m645 = f281(v3);
            match m645 {
                None => {
                    v2
                }
                Some(v4) => {
                    f318(v1, v4, v2)
                }
            }
        }
    }
}

pub fn f226(v0: Str, v1: Adt69) -> List<(Str, Adt69)> {
    List::cons((v0, v1), List::<(Str, Adt69)>::nil())
}

pub fn f227(v0: Str, v1: Str, v2: Str) -> Adt69 {
    Adt69::C0(Adt70::C0(Adt71::C0(None::<Adt72>, v0, f319(v1, v2))))
}

pub fn f228(v0: List<Adt61>, v1: Fn7, v2: Adt28) -> Result<Adt37, Adt94> {
    let m647 = f320(match v2.clone() {
        Adt28::C0(_, h646, _, _, _) => {
            h646
        }
    }, v0);
    match m647 {
        None => {
            Err::<Adt37, Adt94>(Adt94::C1(match v2 {
                Adt28::C0(_, h648, _, _, _) => {
                    h648
                }
            }))
        }
        Some(v3) => {
            let m650 = f321(f322(v3), f323(match v2.clone() {
                Adt28::C0(_, _, h649, _, _) => {
                    h649
                }
            }));
            match m650 {
                None => {
                    Ok::<Adt37, Adt94>(Adt37::C8({
                        let c653: Fn7 = v1;
                        c653.apply(match v2.clone() {
                            Adt28::C0(_, h654, _, _, _) => {
                                h654
                            }
                        }, f323(match v2 {
                            Adt28::C0(_, _, h655, _, _) => {
                                h655
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt37, Adt94>(Adt94::C4({
                        let a651 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a652 = Str::lit(" ");
                        join(a651, a652)
                    }))
                }
            }
        }
    }
}

pub fn f229(v0: Str, v1: List<(Str, Str)>) -> Adt63 {
    let a656 = v0.clone();
    let a657 = Str::lit("test_simple_prompt");
    if equal(a656, a657) {
        f324(List::cons(f325(Str::lit("This is a simple prompt for testing.")), List::<Adt64>::nil()))
    } else {
        let a658 = v0.clone();
        let a659 = Str::lit("test_prompt_with_arguments");
        if equal(a658, a659) {
            f324(List::cons(f325({
                let a660 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f326(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f326(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a661 = Str::lit("");
                join(a660, a661)
            }), List::<Adt64>::nil()))
        } else {
            let a662 = v0.clone();
            let a663 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a662, a663) {
                f324(List::cons(Adt64::C0(Adt46::C0, Adt43::C4(Adt50::C0(f327(f326(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt45>, None::<Adt8>))), List::cons(f325(Str::lit("Please process the embedded resource above.")), List::<Adt64>::nil())))
            } else {
                let a664 = v0;
                let a665 = Str::lit("test_prompt_with_image");
                if equal(a664, a665) {
                    f324(List::cons(Adt64::C0(Adt46::C0, Adt43::C1(Adt47::C0(f328(), Str::lit("image/png"), None::<Adt45>, None::<Adt8>))), List::cons(f325(Str::lit("Please analyze the image above.")), List::<Adt64>::nil())))
                } else {
                    f324(List::<Adt64>::nil())
                }
            }
        }
    }
}

pub fn f230(v0: Fn8, v1: Adt29) -> R<Result<Adt37, Adt94>> {
    let m667 = {
        let c666: Fn8 = v0;
        c666.apply(v1)?
    };
    match m667 {
        Ok(v3) => {
            Ok(Ok::<Adt37, Adt94>(Adt37::C9(f329(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt37, Adt94>(v2))
        }
    }
}

pub fn f231(v0: Adt29) -> R<Result<List<Str>, Adt94>> {
    let m669 = match v0.clone() {
        Adt29::C0(_, h668, _, _) => {
            h668
        }
    };
    match m669 {
        Adt30::C0(v1) => {
            let m671 = f320(match v1.clone() {
                Adt31::C0(h670, _) => {
                    h670
                }
            }, f221());
            match m671 {
                None => {
                    Ok(Err::<List<Str>, Adt94>(Adt94::C1(match v1 {
                        Adt31::C0(h672, _) => {
                            h672
                        }
                    })))
                }
                Some(v2) => {
                    if f330(f322(v2), {
                        let m675 = match v0.clone() {
                            Adt29::C0(_, _, h674, _) => {
                                h674
                            }
                        };
                        match m675 {
                            Adt32::C0(h673, _) => {
                                h673
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt94>(f331({
                            let m678 = match v0.clone() {
                                Adt29::C0(_, _, h677, _) => {
                                    h677
                                }
                            };
                            match m678 {
                                Adt32::C0(_, h676) => {
                                    h676
                                }
                            }
                        }, f332(match v1 {
                            Adt31::C0(h679, _) => {
                                h679
                            }
                        }, {
                            let m682 = match v0 {
                                Adt29::C0(_, _, h681, _) => {
                                    h681
                                }
                            };
                            match m682 {
                                Adt32::C0(h680, _) => {
                                    h680
                                }
                            }
                        }))?))
                    } else {
                        Ok(Err::<List<Str>, Adt94>(Adt94::C4(Str::lit("the prompt declares no such argument"))))
                    }
                }
            }
        }
        Adt30::C1(v3) => {
            let a683 = v3.clone();
            let a684 = f314();
            if equal(a683, a684) {
                let a688 = {
                    let m687 = match v0.clone() {
                        Adt29::C0(_, _, h686, _) => {
                            h686
                        }
                    };
                    match m687 {
                        Adt32::C0(h685, _) => {
                            h685
                        }
                    }
                };
                let a689 = Str::lit("id");
                if equal(a688, a689) {
                    Ok(Ok::<List<Str>, Adt94>(f331({
                        let m692 = match v0 {
                            Adt29::C0(_, _, h691, _) => {
                                h691
                            }
                        };
                        match m692 {
                            Adt32::C0(_, h690) => {
                                h690
                            }
                        }
                    }, f333())?))
                } else {
                    Ok(Err::<List<Str>, Adt94>(Adt94::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt94>(Adt94::C2(v3)))
            }
        }
    }
}

pub fn f232(v0: Adt22) -> bool {
    f334(match v0 {
        Adt22::C0(_, _, _, _, h693) => {
            h693
        }
    }, f33())
}

pub fn f233(v0: Bytes) -> R<Option<List<Adt89>>> {
    let a695 = {
        let a694 = v0.clone();
        length_bytes(a694)
    };
    let a696 = 0u64;
    if nat_eq(a695, a696) {
        Ok(Some(List::<Adt89>::nil()))
    } else {
        let m697 = f255(v0)?;
        match m697 {
            Ok(v2) => {
                let m698 = f335(v2);
                match m698 {
                    None => {
                        Ok(None::<List<Adt89>>)
                    }
                    Some(v3) => {
                        Ok(f336(v3, List::<Adt89>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt89>>)
            }
        }
    }
}

pub fn f234() -> Adt94 {
    Adt94::C9(Str::lit("the task table is not readable"))
}

pub fn f235(v0: Str, p703: List<Adt89>, v2: i64) -> R<Option<Adt89>> {
    let m702 = p703;
    {
        let mut m702 = m702;
        loop {
            let step = {
                let v1 = m702;
                let m699 = v1;
                match m699.uncons() {
                    None => {
                        Err::<List<Adt89>, R<Option<Adt89>>>(Ok(None::<Adt89>))
                    }
                    Some((v3, v4)) => {
                        let a700 = f337(v3.clone());
                        let a701 = v0.clone();
                        if equal(a700, a701) {
                            if f338(v3.clone(), v2)? {
                                Err::<List<Adt89>, R<Option<Adt89>>>(Ok(None::<Adt89>))
                            } else {
                                Err::<List<Adt89>, R<Option<Adt89>>>(Ok(Some(v3)))
                            }
                        } else {
                            Ok::<List<Adt89>, R<Option<Adt89>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236() -> Adt94 {
    Adt94::C8(f339())
}

pub fn f237(v0: Adt110, v1: Adt93, v2: bool) -> Adt37 {
    Adt37::C14(Adt92::C0(match v0.clone() {
        Adt110::C0(h704, _) => {
            h704
        }
    }, v1, if match v0 {
        Adt110::C0(_, h705) => {
            h705
        }
    } {
        true
    } else {
        v2
    }))
}

pub fn f238(v0: Adt89, v1: i64, v2: Str) -> R<Adt110> {
    let m707 = match v0.clone() {
        Adt89::C0(h706, _, _) => {
            h706
        }
    };
    match m707 {
        Adt67::C0(v3) => {
            f340(v0, v3, v1, v2)
        }
        Adt67::C1(_, _) => {
            Ok(f341(v0))
        }
        Adt67::C2(_, _) => {
            Ok(f341(v0))
        }
        Adt67::C3(_, _) => {
            Ok(f341(v0))
        }
        Adt67::C4(_) => {
            Ok(f341(v0))
        }
    }
}

pub fn f239(v0: Adt89, v1: Adt8, v2: i64, v3: Str) -> R<Adt110> {
    let m709 = match v0.clone() {
        Adt89::C0(h708, _, _) => {
            h708
        }
    };
    match m709 {
        Adt67::C0(_) => {
            Ok(f341(v0))
        }
        Adt67::C1(v5, v6) => {
            f342(v0, v5, v6, v1, v2, v3)
        }
        Adt67::C2(_, _) => {
            Ok(f341(v0))
        }
        Adt67::C3(_, _) => {
            Ok(f341(v0))
        }
        Adt67::C4(_) => {
            Ok(f341(v0))
        }
    }
}

pub fn f240(v0: Adt89, v1: i64, v2: Str) -> R<Adt110> {
    Ok(f343(f238(v0, v1, v2.clone())?, v2))
}

pub fn f241(p712: List<Adt101>, p713: List<Adt101>) -> List<Adt101> {
    let m711 = (p712, p713);
    {
        let mut m711 = m711;
        loop {
            let step = {
                let (v0, v1) = m711;
                let m710 = v0;
                match m710.uncons() {
                    None => {
                        Err::<(List<Adt101>, List<Adt101>), List<Adt101>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt101>, List<Adt101>), List<Adt101>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Bytes, v1: Adt100, v2: Result<(Adt18, u64), Adt2>) -> R<Result<Adt100, Adt2>> {
    let m714 = v2;
    match m714 {
        Ok(v4) => {
            f344(v0, Adt100::C0(match v1.clone() {
                Adt100::C0(h715, _, _, _, _, _, _, _) => {
                    h715
                }
            }, match v1.clone() {
                Adt100::C0(_, h716, _, _, _, _, _, _) => {
                    h716
                }
            }, match v1.clone() {
                Adt100::C0(_, _, h717, _, _, _, _, _) => {
                    h717
                }
            }, match v1.clone() {
                Adt100::C0(_, _, _, h718, _, _, _, _) => {
                    h718
                }
            }, match v1.clone() {
                Adt100::C0(_, _, _, _, h719, _, _, _) => {
                    h719
                }
            }, match v1.clone() {
                Adt100::C0(_, _, _, _, _, h720, _, _) => {
                    h720
                }
            }, match v1 {
                Adt100::C0(_, _, _, _, _, _, h721, _) => {
                    h721
                }
            }, {
                let (h722, _) = v4;
                h722
            }), {
                let (_, h723) = v4;
                h723
            })
        }
        Err(v3) => {
            Ok(Err::<Adt100, Adt2>(v3))
        }
    }
}

pub fn f243() -> Bytes {
    Bytes::lit(&[])
}

pub fn f244(v0: Bytes, v1: u64) -> R<Result<(Adt18, u64), Adt2>> {
    let m724 = f345(v0.clone(), v1)?;
    match m724 {
        None => {
            Ok(Err::<(Adt18, u64), Adt2>(Adt2::C0(v1)))
        }
        Some(v2) => {
            let m727 = f346(v0.clone(), {
                let a725 = v1;
                let a726 = 8u64;
                nat_add(a725, a726)?
            })?;
            match m727 {
                Ok(v4) => {
                    f347(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt18, u64), Adt2>(v3))
                }
            }
        }
    }
}

pub fn f245(v0: Adt101) -> List<u64> {
    let a738 = f28({
        let a729 = match v0.clone() {
            Adt101::C0(h728, _) => {
                h728
            }
        };
        length_list(a729)
    });
    let a739 = {
        let a736 = match v0.clone() {
            Adt101::C0(h730, _) => {
                h730
            }
        };
        let a737 = {
            let a734 = f28({
                let a732 = match v0.clone() {
                    Adt101::C0(_, h731) => {
                        h731
                    }
                };
                length_list(a732)
            });
            let a735 = match v0 {
                Adt101::C0(_, h733) => {
                    h733
                }
            };
            append_list(a734, a735)
        };
        append_list(a736, a737)
    };
    append_list(a738, a739)
}

pub fn f246(v0: List<u64>) -> List<u64> {
    let a741 = f28({
        let a740 = v0.clone();
        length_list(a740)
    });
    let a742 = v0;
    append_list(a741, a742)
}

pub fn f247(v0: u64, v1: Adt9) -> R<Adt104> {
    let m743 = f348(v1)?;
    match m743 {
        Ok(v3) => {
            Ok(Adt104::C0(v0, f101(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f57())
        }
    }
}

pub fn f248(v0: Adt107) -> u64 {
    let m744 = v0;
    match m744 {
        Adt107::C0 => {
            f349()
        }
        Adt107::C1 => {
            f349()
        }
    }
}

pub fn f249(v0: Option<Adt111>, v1: Adt85) -> Adt9 {
    Adt9::C5(f350(v0, v1))
}

pub fn f250(v0: Adt107) -> Adt85 {
    Adt85::C0(f351(), f352(v0), None::<Adt9>)
}

pub fn f251(v0: List<u64>) -> bool {
    f53(v0, f353())
}

pub fn f252(v0: Adt18, v1: Adt3, v2: Fn9, v3: u64, v4: List<Adt101>, v5: Result<Adt9, Adt120>) -> R<Adt104> {
    let m745 = v5;
    match m745 {
        Ok(v7) => {
            f356(v0, v1, v2, v3, v4, f357(v7)?)
        }
        Err(_) => {
            f247(f354(), f249(None::<Adt111>, f355(Adt121::C0)))
        }
    }
}

pub fn f253(v0: Adt3, v1: Bytes) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h746, _, _, _, _, _, _, _) => {
            h746
        }
    }, match v0.clone() {
        Adt3::C0(_, h747, _, _, _, _, _, _) => {
            h747
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h748, _, _, _, _, _) => {
            h748
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h749, _, _, _, _) => {
            h749
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h750, _, _, _) => {
            h750
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h751, _, _) => {
            h751
        }
    }, v1, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h752) => {
            h752
        }
    })
}

pub fn f254(v0: Fn1, v1: Adt100, v2: Adt19) -> R<Result<Adt37, Adt94>> {
    let c753: Fn1 = v0;
    c753.apply(match v1 {
        Adt100::C0(_, _, _, _, _, _, _, h754) => {
            h754
        }
    }, v2)
}

pub fn f255(v0: Bytes) -> R<Result<Adt9, Adt120>> {
    f358(f29(v0)?)
}

pub fn f256() -> Adt104 {
    Adt104::C0(f359(), List::cons(Adt101::C0(f360(), f361()), List::<Adt101>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f257(v0: List<Adt101>, v1: List<u64>) -> R<Adt123> {
    let m755 = f362(v0, v1, Adt124::C0)?;
    match m755 {
        Adt124::C0 => {
            Ok(Adt123::C0)
        }
        Adt124::C1(v2) => {
            let m756 = f363(v2);
            match m756 {
                None => {
                    Ok(Adt123::C1)
                }
                Some(v3) => {
                    Ok(Adt123::C3(v3))
                }
            }
        }
        Adt124::C2 => {
            Ok(Adt123::C2)
        }
    }
}

pub fn f258() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f259(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt107) -> R<Option<Adt107>> {
    let m757 = v0;
    match m757 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f364(v3, v1)? {
                Ok(None::<Adt107>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f260(v0: List<u64>) -> Option<List<u64>> {
    let m758 = f365(v0);
    match m758 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f262(v1)
        }
    }
}

pub fn f261() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f262(v0: List<u64>) -> Option<List<u64>> {
    let m759 = v0.clone();
    match m759.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a760 = v1;
            let a761 = 91u64;
            if nat_eq(a760, a761) {
                f366(f367(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f366(f368(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f263(v0: Adt18, v1: Adt3, v2: Fn9, v3: bool, v4: u64, v5: List<Adt101>, v6: Result<Adt9, Adt120>) -> R<Adt97> {
    let m762 = v6;
    match m762 {
        Ok(v8) => {
            f369(v0, v1, v2, v3, v4, v5, f357(v8)?)
        }
        Err(_) => {
            Ok(Adt97::C0(f56(f247(f354(), f249(None::<Adt111>, f355(Adt121::C0)))?)))
        }
    }
}

pub fn f264(v0: Fn1, v1: Adt100, v2: Adt19) -> R<Result<Adt37, Adt94>> {
    let c763: Fn1 = v0;
    c763.apply(match v1 {
        Adt100::C0(_, _, _, _, _, _, _, h764) => {
            h764
        }
    }, v2)
}

pub fn f265(v0: List<u64>) -> bool {
    let m765 = v0.clone();
    match m765.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a769 = if f370(v0.clone(), false) {
                let a767 = {
                    let a766 = v0;
                    length_list(a766)
                };
                let a768 = f371();
                nat_le(a767, a768)
            } else {
                false
            };
            bool_not(a769)
        }
    }
}

pub fn f266(v0: Adt40) -> Option<Adt102> {
    let m774 = f372(match v0.clone() {
        Adt40::C0(h770, _, _, _, _, _, _, _) => {
            h770
        }
    }, {
        let m773 = match v0.clone() {
            Adt40::C0(_, _, _, _, h772, _, _, _) => {
                h772
            }
        };
        match m773 {
            Adt8::C0(h771) => {
                h771
            }
        }
    });
    match m774 {
        None => {
            f373(match v0.clone() {
                Adt40::C0(h775, _, _, _, _, _, _, _) => {
                    h775
                }
            }, {
                let m778 = match v0 {
                    Adt40::C0(_, _, _, _, h777, _, _, _) => {
                        h777
                    }
                };
                match m778 {
                    Adt8::C0(h776) => {
                        h776
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f267(v0: Adt40, v1: Result<Adt125, Adt102>) -> R<Result<List<Adt15>, Adt102>> {
    let m779 = v1;
    match m779 {
        Ok(v3) => {
            let m780 = f374(v0)?;
            match m780 {
                None => {
                    Ok(Ok::<List<Adt15>, Adt102>(f109(match v3 {
                        Adt125::C0(h781, _) => {
                            h781
                        }
                    }, List::<Adt15>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt15>, Adt102>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt15>, Adt102>(v2))
        }
    }
}

pub fn f268(p788: u64, v1: Str, v2: Adt9, p789: List<Adt112>, p790: Adt125) -> R<Result<Adt125, Adt102>> {
    let m787 = (p788, (p789, p790));
    {
        let mut m787 = m787;
        loop {
            let step = {
                let (v0, (v3, v4)) = m787;
                let m782 = v0;
                if m782 == 0 {
                    Err::<(u64, (List<Adt112>, Adt125)), R<Result<Adt125, Adt102>>>(Ok(Err::<Adt125, Adt102>(Adt102::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m782 - 1;
                    let m783 = v3;
                    match m783.uncons() {
                        None => {
                            Err::<(u64, (List<Adt112>, Adt125)), R<Result<Adt125, Adt102>>>(Ok(Ok::<Adt125, Adt102>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m784 = f375(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m784 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt112>, Adt125)), R<Result<Adt125, Adt102>>>((v5, ({
                                        let (h785, _) = v9;
                                        h785
                                    }, {
                                        let (_, h786) = v9;
                                        h786
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt112>, Adt125)), R<Result<Adt125, Adt102>>>(Ok(Err::<Adt125, Adt102>(v8)))
                                }
                            }
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

pub fn f269() -> u64 {
    67108864u64
}

pub fn f270() -> Adt125 {
    Adt125::C0(List::<Adt15>::nil(), List::<List<u64>>::nil())
}

pub fn f271(v0: Adt9, p809: u64, p810: Adt115, p811: List<Adt116>) -> R<Adt113> {
    let m808 = (p809, (p810, p811));
    {
        let mut m808 = m808;
        loop {
            let step = {
                let (v1, (v2, v3)) = m808;
                let m791 = v1;
                if m791 == 0 {
                    Err::<(u64, (Adt115, List<Adt116>)), R<Adt113>>(Ok(Adt113::C4))
                } else {
                    let v4 = m791 - 1;
                    let m792 = v3;
                    match m792.uncons() {
                        None => {
                            let m793 = v2;
                            match m793 {
                                Adt115::C0 => {
                                    Err::<(u64, (Adt115, List<Adt116>)), R<Adt113>>(Ok(Adt113::C0))
                                }
                                Adt115::C1(v5, v6) => {
                                    Err::<(u64, (Adt115, List<Adt116>)), R<Adt113>>(Ok(Adt113::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m794 = v2.clone();
                            match m794 {
                                Adt115::C0 => {
                                    let m795 = v7.clone();
                                    match m795 {
                                        Adt116::C0(v9, v10, v11) => {
                                            let m796 = f376(v0.clone(), v9, v10, v11.clone())?;
                                            match m796 {
                                                Adt128::C0(v12) => {
                                                    Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (Adt115::C1(v11, v12), v8)))
                                                }
                                                Adt128::C1(v13) => {
                                                    Err::<(u64, (Adt115, List<Adt116>)), R<Adt113>>(Ok(Adt113::C2(v13)))
                                                }
                                                Adt128::C2(v14) => {
                                                    Err::<(u64, (Adt115, List<Adt116>)), R<Adt113>>(Ok(Adt113::C3(v14)))
                                                }
                                                Adt128::C3(v15) => {
                                                    let v16: List<Adt116> = {
                                                        let a797 = v15;
                                                        let a798 = v8;
                                                        append_list(a797, a798)
                                                    };
                                                    Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt116::C1(_, _, _) => {
                                            Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (v2, v8)))
                                        }
                                        Adt116::C2(_, _, _, _) => {
                                            let v24: Adt126 = f377(v7, v8)?;
                                            Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (match v24.clone() {
                                                Adt126::C0(h799, _) => {
                                                    h799
                                                }
                                            }, match v24 {
                                                Adt126::C0(_, h800) => {
                                                    h800
                                                }
                                            })))
                                        }
                                        Adt116::C3(v25) => {
                                            Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (Adt115::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt116::C4(_, _, _, _) => {
                                            let v39: Adt126 = f377(v7, v8)?;
                                            Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (match v39.clone() {
                                                Adt126::C0(h804, _) => {
                                                    h804
                                                }
                                            }, match v39 {
                                                Adt126::C0(_, h805) => {
                                                    h805
                                                }
                                            })))
                                        }
                                        Adt116::C5(v26, v27, v28, v29) => {
                                            let m801 = f378(v26, v27, v28, v29.clone());
                                            match m801 {
                                                Adt128::C0(v30) => {
                                                    Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (Adt115::C1(v29, v30), v8)))
                                                }
                                                Adt128::C1(v31) => {
                                                    Err::<(u64, (Adt115, List<Adt116>)), R<Adt113>>(Ok(Adt113::C2(v31)))
                                                }
                                                Adt128::C2(v32) => {
                                                    Err::<(u64, (Adt115, List<Adt116>)), R<Adt113>>(Ok(Adt113::C3(v32)))
                                                }
                                                Adt128::C3(v33) => {
                                                    let v34: List<Adt116> = {
                                                        let a802 = v33;
                                                        let a803 = v8;
                                                        append_list(a802, a803)
                                                    };
                                                    Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt115::C1(v40, v41) => {
                                    let v42: Adt126 = f379(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt115, List<Adt116>)), R<Adt113>>((v4, (match v42.clone() {
                                        Adt126::C0(h806, _) => {
                                            h806
                                        }
                                    }, match v42 {
                                        Adt126::C0(_, h807) => {
                                            h807
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m808 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272() -> u64 {
    67108864u64
}

pub fn f273(v0: List<Adt114>) -> R<Str> {
    let a812 = List::cons(Str::lit(""), f380(v0, List::<Str>::nil())?);
    let a813 = Str::lit("/");
    Ok(join(a812, a813))
}

pub fn f274(v0: Adt40, v1: Adt54) -> R<Result<Adt37, Adt94>> {
    if f381({
        let m816 = match v1.clone() {
            Adt54::C0(h815, _) => {
                h815
            }
        };
        match m816 {
            Adt42::C0(h814, _, _) => {
                h814
            }
        }
    })? {
        f382(v0, v1)
    } else {
        Ok(Err::<Adt37, Adt94>(Adt94::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f275(v0: Str, v1: Adt8) -> Adt118 {
    f383(v0, match v1 {
        Adt8::C0(h817) => {
            h817
        }
    })
}

pub fn f276(v0: Adt18, v1: Adt40, v2: Adt26, v3: Adt105) -> R<Result<Adt37, Adt94>> {
    let m818 = v3;
    match m818 {
        Adt105::C0(v4) => {
            f274(v1, v4)
        }
        Adt105::C1(v6, v7, v8) => {
            if f384(v2) {
                f385(v0, v8)
            } else {
                f386(v6, v1, v7)
            }
        }
        Adt105::C2(v5) => {
            Ok(Ok::<Adt37, Adt94>(Adt37::C12(v5)))
        }
    }
}

pub fn f277(v0: Adt9) -> Option<i64> {
    let m819 = v0;
    match m819 {
        Adt9::C0 => {
            None::<i64>
        }
        Adt9::C1(_) => {
            None::<i64>
        }
        Adt9::C2(v2) => {
            if f387(match v2.clone() {
                Adt10::C0(_, h820) => {
                    h820
                }
            }) {
                Some(match v2 {
                    Adt10::C0(h821, _) => {
                        h821
                    }
                })
            } else {
                None::<i64>
            }
        }
        Adt9::C3(_) => {
            None::<i64>
        }
        Adt9::C4(_) => {
            None::<i64>
        }
        Adt9::C5(_) => {
            None::<i64>
        }
    }
}

pub fn f278(v0: Option<List<(Str, Adt9)>>) -> R<Option<List<(Str, Adt9)>>> {
    let m822 = v0;
    match m822 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            let m823 = f388(Adt9::C5(v1))?;
            match m823 {
                None => {
                    Ok(None::<List<(Str, Adt9)>>)
                }
                Some(v2) => {
                    let m824 = f281(v2);
                    match m824 {
                        None => {
                            Ok(None::<List<(Str, Adt9)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt8::C0(h825) => {
                                    h825
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f279(v0: Adt42) -> R<Option<List<(Str, Adt9)>>> {
    let m827 = f389(match v0.clone() {
        Adt42::C0(h826, _, _) => {
            h826
        }
    })?;
    match m827 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt9::C4(v1)), {
                let a830 = f390(Str::lit("structuredContent"), match v0.clone() {
                    Adt42::C0(_, h828, _) => {
                        h828
                    }
                });
                let a831 = f391(Str::lit("isError"), match v0 {
                    Adt42::C0(_, _, h829) => {
                        h829
                    }
                });
                append_list(a830, a831)
            })))
        }
    }
}

pub fn f280() -> Adt91 {
    Adt91::C1(Adt85::C0(-32603i64, Str::lit("Internal error"), None::<Adt9>))
}

pub fn f281(v0: Adt9) -> Option<Adt8> {
    let m832 = v0;
    match m832 {
        Adt9::C0 => {
            None::<Adt8>
        }
        Adt9::C1(_) => {
            None::<Adt8>
        }
        Adt9::C2(_) => {
            None::<Adt8>
        }
        Adt9::C3(_) => {
            None::<Adt8>
        }
        Adt9::C4(_) => {
            None::<Adt8>
        }
        Adt9::C5(v5) => {
            Some(Adt8::C0(v5))
        }
    }
}

pub fn f282(v0: Adt40, v1: Result<Adt108, Adt94>) -> R<Result<Adt37, Adt94>> {
    let m833 = v1;
    match m833 {
        Ok(v3) => {
            let m834 = v3;
            match m834 {
                Adt108::C0(v4) => {
                    f274(v0, v4)
                }
                Adt108::C1(v5) => {
                    Ok(Ok::<Adt37, Adt94>(Adt37::C12(v5)))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt37, Adt94>(v2))
        }
    }
}

pub fn f283(v0: Str) -> Result<Adt108, Adt94> {
    Ok::<Adt108, Adt94>(Adt108::C0(Adt54::C0(Adt42::C0(List::cons(Adt43::C0(Adt44::C0(v0, None::<Adt45>, None::<Adt8>)), List::<Adt43>::nil()), None::<Adt9>, None::<bool>), List::<Adt55>::nil())))
}

pub fn f284(v0: List<Str>) -> Str {
    let a835 = v0;
    let a836 = Str::lit("");
    join(a835, a836)
}

pub fn f285(v0: List<(Str, Adt69)>) -> Result<Adt108, Adt94> {
    Ok::<Adt108, Adt94>(Adt108::C1(Adt86::C0(v0)))
}

pub fn f286() -> Adt69 {
    f227(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f287() -> Result<Adt108, Adt94> {
    Err::<Adt108, Adt94>(Adt94::C11)
}

pub fn f288(v0: Str, v1: u64) -> Adt69 {
    Adt69::C1(Adt74::C0(List::cons(f392(v0), List::<Adt75>::nil()), None::<Adt80>, None::<Str>, None::<Adt82>, None::<Adt10>, v1, None::<List<Str>>, None::<Adt8>, None::<List<Adt40>>, None::<Adt83>))
}

pub fn f289(v0: u64) -> R<Str> {
    let a837 = f393(v0, 0i64)?;
    Ok(format_i64(a837))
}

pub fn f290(v0: Adt8) -> u64 {
    let m838 = f275(Str::lit("roots"), v0);
    match m838 {
        Adt118::C0 => {
            0u64
        }
        Adt118::C1(v1) => {
            let m839 = f335(v1);
            match m839 {
                None => {
                    0u64
                }
                Some(v2) => {
                    let a840 = v2;
                    length_list(a840)
                }
            }
        }
    }
}

pub fn f291() -> Adt69 {
    Adt69::C2(Some(f32()))
}

pub fn f292(v0: Adt26, v1: Str, v2: Str) -> Result<Adt108, Adt94> {
    if f293(v0.clone(), v1.clone()) {
        let m842 = f223(Str::lit("confirm"), Adt109::C0, match v0 {
            Adt26::C0(_, _, _, h841, _) => {
                h841
            }
        });
        match m842 {
            Adt119::C0 => {
                f295(f226(Str::lit("confirm"), f395()), v1)
            }
            Adt119::C1 => {
                f287()
            }
            Adt119::C2(v3) => {
                if f394(v3) {
                    f283(v2)
                } else {
                    f295(f226(Str::lit("confirm"), f395()), v1)
                }
            }
        }
    } else {
        f295(f226(Str::lit("confirm"), f395()), v1)
    }
}

pub fn f293(v0: Adt26, v1: Str) -> bool {
    let m844 = match v0 {
        Adt26::C0(_, _, _, _, h843) => {
            h843
        }
    };
    match m844 {
        None => {
            false
        }
        Some(v2) => {
            let a845 = v2;
            let a846 = v1;
            equal(a845, a846)
        }
    }
}

pub fn f294(v0: Adt119, v1: Adt119, v2: Adt119) -> R<Result<Adt108, Adt94>> {
    if if f396(v0.clone()) {
        true
    } else {
        if f396(v1.clone()) {
            true
        } else {
            f396(v2.clone())
        }
    } {
        Ok(f287())
    } else {
        let m847 = f296(v0.clone(), v1.clone(), v2.clone());
        match m847.uncons() {
            None => {
                Ok(f283(f284(List::cons(Str::lit("Name: "), List::cons(f225(f397(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f225(f397(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f289(f290(f397(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f295(f296(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f295(v0: List<(Str, Adt69)>, v1: Str) -> Result<Adt108, Adt94> {
    Ok::<Adt108, Adt94>(Adt108::C1(Adt86::C2(v0, v1)))
}

pub fn f296(v0: Adt119, v1: Adt119, v2: Adt119) -> List<(Str, Adt69)> {
    let a850 = if f398(v0) {
        f226(Str::lit("user_name"), f286())
    } else {
        List::<(Str, Adt69)>::nil()
    };
    let a851 = {
        let a848 = if f398(v1) {
            f226(Str::lit("greeting"), f288(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt69)>::nil()
        };
        let a849 = if f398(v2) {
            f226(Str::lit("client_roots"), f291())
        } else {
            List::<(Str, Adt69)>::nil()
        };
        append_list(a848, a849)
    };
    append_list(a850, a851)
}

pub fn f297(v0: Str) -> R<u64> {
    f61({
        let a852 = v0;
        utf8_encode(a852)
    }, 0u64)
}

pub fn f298(v0: Adt26) -> Str {
    let m854 = match v0 {
        Adt26::C0(_, _, _, _, h853) => {
            h853
        }
    };
    match m854 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f299(v0: Adt26) -> Result<Adt108, Adt94> {
    let m856 = f223(Str::lit("step1"), Adt109::C0, match v0 {
        Adt26::C0(_, _, _, h855, _) => {
            h855
        }
    });
    match m856 {
        Adt119::C0 => {
            f295(f226(Str::lit("step1"), f301()), Str::lit("1"))
        }
        Adt119::C1 => {
            f287()
        }
        Adt119::C2(v1) => {
            f295(f226(Str::lit("step2"), f399()), f284(List::cons(Str::lit("2"), List::cons(f225(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f300(v0: Adt26) -> Result<Adt108, Adt94> {
    let m858 = f223(Str::lit("step2"), Adt109::C0, match v0.clone() {
        Adt26::C0(_, _, _, h857, _) => {
            h857
        }
    });
    match m858 {
        Adt119::C0 => {
            f295(f226(Str::lit("step2"), f399()), f298(v0))
        }
        Adt119::C1 => {
            f287()
        }
        Adt119::C2(v1) => {
            f283(f284(List::cons(Str::lit("Multi-round complete for "), List::cons(f400(f298(v0)), List::cons(Str::lit(" who likes "), List::cons(f225(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f301() -> Adt69 {
    f227(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f302(v0: List<(Str, Adt69)>) -> Result<Adt108, Adt94> {
    let m859 = v0.clone();
    match m859.uncons() {
        None => {
            f283(Str::lit("No supported capabilities declared"))
        }
        Some((_, _)) => {
            f295(v0, Str::lit("capabilities-test"))
        }
    }
}

pub fn f303(v0: Adt22) -> List<(Str, Adt69)> {
    let a862 = if f401(match v0.clone() {
        Adt22::C0(_, _, _, h860, _) => {
            h860
        }
    }) {
        f226(Str::lit("elicit_input"), f227(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt69)>::nil()
    };
    let a863 = if f401(match v0 {
        Adt22::C0(_, _, h861, _, _) => {
            h861
        }
    }) {
        f226(Str::lit("sample_input"), f288(Str::lit("Sample request"), 50u64))
    } else {
        List::<(Str, Adt69)>::nil()
    };
    append_list(a862, a863)
}

pub fn f304(v0: List<(Str, Adt9)>) -> List<Str> {
    f402(f403(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f305(v0: Option<Adt24>) -> Str {
    let m864 = v0;
    match m864 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m865 = v1;
            match m865 {
                Adt24::C0(v2) => {
                    v2
                }
                Adt24::C1(v3) => {
                    let a866 = v3;
                    format_i64(a866)
                }
            }
        }
    }
}

pub fn f306(v0: u64, v1: u64) -> R<Adt55> {
    Ok(Adt55::C0(f404(v0)?, Some(f404(v1)?), Some({
        let a869 = List::cons(Str::lit("Completed step"), List::cons({
            let a867 = f393(v0, 0i64)?;
            format_i64(a867)
        }, List::cons(Str::lit("of"), List::cons({
            let a868 = f393(v1, 0i64)?;
            format_i64(a868)
        }, List::<Str>::nil()))));
        let a870 = Str::lit(" ");
        join(a869, a870)
    })))
}

pub fn f307() -> Adt22 {
    Adt22::C0(None::<Adt8>, None::<Adt8>, Some(f32()), None::<Adt8>, None::<Adt8>)
}

pub fn f308() -> Adt54 {
    f199(List::cons(f95(Str::lit("ok")), List::<Adt43>::nil()))
}

pub fn f309(v0: List<(Adt49, List<Adt51>)>) -> List<Adt49> {
    f405(f406(v0, List::<Adt49>::nil()), List::<Adt49>::nil())
}

pub fn f310() -> List<(Adt49, List<Adt51>)> {
    List::cons(f407(f408(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f327(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f407(f408(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f409(Str::lit("test://static-binary"), Str::lit("image/png"), f328())), List::cons(f407(f408(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f327(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f407(f408(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f327(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt49, List<Adt51>)>::nil()))))
}

pub fn f311(v0: Adt13, v1: Str, v2: Option<List<Adt51>>) -> Result<Adt37, Adt94> {
    let m871 = v2;
    match m871 {
        None => {
            Err::<Adt37, Adt94>(Adt94::C2(v1))
        }
        Some(v3) => {
            let m872 = v3.clone();
            match m872.uncons() {
                None => {
                    Err::<Adt37, Adt94>(Adt94::C9(Str::lit("a listed resource has no contents")))
                }
                Some((_, _)) => {
                    Ok::<Adt37, Adt94>(Adt37::C5(Adt57::C0(v0, v3)))
                }
            }
        }
    }
}

pub fn f312(v0: Str, p881: List<(Adt49, List<Adt51>)>) -> Option<List<Adt51>> {
    let m880 = p881;
    {
        let mut m880 = m880;
        loop {
            let step = {
                let v1 = m880;
                let m873 = v1;
                match m873.uncons() {
                    None => {
                        Err::<List<(Adt49, List<Adt51>)>, Option<List<Adt51>>>(None::<List<Adt51>>)
                    }
                    Some((v2, v3)) => {
                        let a877 = {
                            let m876 = {
                                let (h875, _) = v2;
                                h875
                            };
                            match m876 {
                                Adt49::C0(_, _, _, h874, _, _, _, _, _) => {
                                    h874
                                }
                            }
                        };
                        let a878 = v0.clone();
                        if equal(a877, a878) {
                            Err::<List<(Adt49, List<Adt51>)>, Option<List<Adt51>>>(Some({
                                let (_, h879) = v2;
                                h879
                            }))
                        } else {
                            Ok::<List<(Adt49, List<Adt51>)>, Option<List<Adt51>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m880 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Str) -> R<Option<List<Adt51>>> {
    let m882 = f410(v0.clone())?;
    match m882 {
        None => {
            Ok(None::<List<Adt51>>)
        }
        Some(v1) => {
            let m883 = f411(v1)?;
            match m883 {
                None => {
                    Ok(None::<List<Adt51>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f327(v0, Str::lit("application/json"), v2), List::<Adt51>::nil())))
                }
            }
        }
    }
}

pub fn f314() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f315(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt62>>) -> Adt61 {
    Adt61::C0(v0, Some(v1), None::<List<Adt5>>, Some(v2), v3, None::<Adt8>)
}

pub fn f316(v0: Str, v1: Str) -> Adt62 {
    Adt62::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f317(v0: Adt109, v1: Adt8) -> bool {
    let m884 = v0;
    match m884 {
        Adt109::C0 => {
            f412(v1)
        }
        Adt109::C1 => {
            f413(v1)
        }
        Adt109::C2 => {
            f414(v1)
        }
    }
}

pub fn f318(v0: Str, v1: Adt8, v2: Str) -> Str {
    let m885 = f275(v0, v1);
    match m885 {
        Adt118::C0 => {
            v2
        }
        Adt118::C1(v3) => {
            let m886 = f181(v3);
            match m886 {
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

pub fn f319(v0: Str, v1: Str) -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((v0.clone(), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(v0), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f320(v0: Str, p892: List<Adt61>) -> Option<Adt61> {
    let m891 = p892;
    {
        let mut m891 = m891;
        loop {
            let step = {
                let v1 = m891;
                let m887 = v1;
                match m887.uncons() {
                    None => {
                        Err::<List<Adt61>, Option<Adt61>>(None::<Adt61>)
                    }
                    Some((v2, v3)) => {
                        let a889 = match v2.clone() {
                            Adt61::C0(h888, _, _, _, _, _) => {
                                h888
                            }
                        };
                        let a890 = v0.clone();
                        if equal(a889, a890) {
                            Err::<List<Adt61>, Option<Adt61>>(Some(v2))
                        } else {
                            Ok::<List<Adt61>, Option<Adt61>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m891 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f321(p898: List<Adt62>, v1: List<(Str, Str)>) -> Option<Str> {
    let m897 = p898;
    {
        let mut m897 = m897;
        loop {
            let step = {
                let v0 = m897;
                let m893 = v0;
                match m893.uncons() {
                    None => {
                        Err::<List<Adt62>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f415(v2.clone()) {
                            let m895 = f416(match v2.clone() {
                                Adt62::C0(h894, _, _, _) => {
                                    h894
                                }
                            }, v1.clone());
                            match m895 {
                                None => {
                                    Err::<List<Adt62>, Option<Str>>(Some(match v2 {
                                        Adt62::C0(h896, _, _, _) => {
                                            h896
                                        }
                                    }))
                                }
                                Some(_) => {
                                    Ok::<List<Adt62>, Option<Str>>(v3)
                                }
                            }
                        } else {
                            Ok::<List<Adt62>, Option<Str>>(v3)
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

pub fn f322(v0: Adt61) -> List<Adt62> {
    let m900 = match v0 {
        Adt61::C0(_, _, _, _, h899, _) => {
            h899
        }
    };
    match m900 {
        None => {
            List::<Adt62>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f323(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m901 = v0;
    match m901 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f324(v0: List<Adt64>) -> Adt63 {
    Adt63::C0(None::<Str>, v0)
}

pub fn f325(v0: Str) -> Adt64 {
    Adt64::C0(Adt46::C0, Adt43::C0(Adt44::C0(v0, None::<Adt45>, None::<Adt8>)))
}

pub fn f326(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m902 = f416(v0, v1);
    match m902 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f327(v0: Str, v1: Str, v2: Str) -> Adt51 {
    Adt51::C0(Adt52::C0(v0, Some(v1), v2, None::<Adt8>))
}

pub fn f328() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f329(v0: List<Str>) -> R<Adt65> {
    let v1: u64 = f417(v0.clone(), 0u64)?;
    Ok(Adt65::C0(Adt66::C0(f418(v0), Some(v1), Some({
        let a903 = f419();
        let a904 = v1;
        nat_lt(a903, a904)
    }))))
}

pub fn f330(p910: List<Adt62>, v1: Str) -> bool {
    let m909 = p910;
    {
        let mut m909 = m909;
        loop {
            let step = {
                let v0 = m909;
                let m905 = v0;
                match m905.uncons() {
                    None => {
                        Err::<List<Adt62>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a907 = match v2 {
                            Adt62::C0(h906, _, _, _) => {
                                h906
                            }
                        };
                        let a908 = v1.clone();
                        if equal(a907, a908) {
                            Err::<List<Adt62>, bool>(true)
                        } else {
                            Ok::<List<Adt62>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f402(f420(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f332(v0: Str, v1: Str) -> List<Str> {
    let a911 = v0;
    let a912 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a911, a912) {
        let a913 = v1;
        let a914 = Str::lit("resourceUri");
        equal(a913, a914)
    } else {
        false
    } {
        f421()
    } else {
        List::<Str>::nil()
    }
}

pub fn f333() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f334(v0: Option<Adt8>, v1: Str) -> bool {
    let m915 = v0;
    match m915 {
        None => {
            false
        }
        Some(v2) => {
            f422(v1, v2)
        }
    }
}

pub fn f335(v0: Adt9) -> Option<List<Adt9>> {
    let m916 = v0;
    match m916 {
        Adt9::C0 => {
            None::<List<Adt9>>
        }
        Adt9::C1(_) => {
            None::<List<Adt9>>
        }
        Adt9::C2(_) => {
            None::<List<Adt9>>
        }
        Adt9::C3(_) => {
            None::<List<Adt9>>
        }
        Adt9::C4(v4) => {
            Some(v4)
        }
        Adt9::C5(_) => {
            None::<List<Adt9>>
        }
    }
}

pub fn f336(p920: List<Adt9>, p921: List<Adt89>) -> Option<List<Adt89>> {
    let m919 = (p920, p921);
    {
        let mut m919 = m919;
        loop {
            let step = {
                let (v0, v1) = m919;
                let m917 = v0;
                match m917.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt89>), Option<List<Adt89>>>(Some(f423(v1, List::<Adt89>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m918 = f424(v2);
                        match m918 {
                            None => {
                                Err::<(List<Adt9>, List<Adt89>), Option<List<Adt89>>>(None::<List<Adt89>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt9>, List<Adt89>), Option<List<Adt89>>>((v3, List::cons(v4, v1)))
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

pub fn f337(v0: Adt89) -> Str {
    let m924 = f425(match v0 {
        Adt89::C0(h923, _, _) => {
            h923
        }
    });
    match m924 {
        Adt68::C0(h922, _, _, _, _, _) => {
            h922
        }
    }
}

pub fn f338(v0: Adt89, v1: i64) -> R<bool> {
    let m928 = {
        let m927 = f425(match v0.clone() {
            Adt89::C0(h926, _, _) => {
                h926
            }
        });
        match m927 {
            Adt68::C0(_, _, _, _, h925, _) => {
                h925
            }
        }
    };
    match m928 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a935 = {
                let m934 = {
                    let a932 = {
                        let a930 = v1;
                        let a931 = match v0 {
                            Adt89::C0(_, h929, _) => {
                                h929
                            }
                        };
                        int_sub(a930, a931)?
                    };
                    let a933 = v2;
                    compare(a932, a933)
                };
                match m934 {
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
            Ok(bool_not(a935))
        }
    }
}

pub fn f339() -> Adt22 {
    Adt22::C0(None::<Adt8>, None::<Adt8>, None::<Adt8>, None::<Adt8>, Some(Adt8::C0(List::cons((f33(), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f340(v0: Adt89, v1: Adt68, v2: i64, v3: Str) -> R<Adt110> {
    let m939 = {
        let m938 = match v0.clone() {
            Adt89::C0(_, _, h937) => {
                h937
            }
        };
        match m938 {
            Adt90::C0(_, h936, _, _) => {
                h936
            }
        }
    };
    match m939.uncons() {
        None => {
            if f426({
                let m942 = match v0.clone() {
                    Adt89::C0(_, _, h941) => {
                        h941
                    }
                };
                match m942 {
                    Adt90::C0(_, _, h940, _) => {
                        h940
                    }
                }
            }, match v0.clone() {
                Adt89::C0(_, h943, _) => {
                    h943
                }
            }, v2)? {
                Ok(f427(v0.clone(), f428({
                    let m946 = match v0.clone() {
                        Adt89::C0(_, _, h945) => {
                            h945
                        }
                    };
                    match m946 {
                        Adt90::C0(_, _, _, h944) => {
                            h944
                        }
                    }
                }, f429(v1, v3)), match v0 {
                    Adt89::C0(_, _, h947) => {
                        h947
                    }
                }))
            } else {
                Ok(f341(v0))
            }
        }
        Some((_, _)) => {
            if f426({
                let m950 = match v0.clone() {
                    Adt89::C0(_, _, h949) => {
                        h949
                    }
                };
                match m950 {
                    Adt90::C0(h948, _, _, _) => {
                        h948
                    }
                }
            }, match v0.clone() {
                Adt89::C0(_, h951, _) => {
                    h951
                }
            }, v2)? {
                Ok(f427(v0.clone(), Adt67::C1(f429(v1, v3), {
                    let m954 = match v0.clone() {
                        Adt89::C0(_, _, h953) => {
                            h953
                        }
                    };
                    match m954 {
                        Adt90::C0(_, h952, _, _) => {
                            h952
                        }
                    }
                }), f430(match v0 {
                    Adt89::C0(_, _, h955) => {
                        h955
                    }
                })))
            } else {
                Ok(f341(v0))
            }
        }
    }
}

pub fn f341(v0: Adt89) -> Adt110 {
    Adt110::C0(v0, false)
}

pub fn f342(v0: Adt89, v1: Adt68, v2: List<(Str, Adt69)>, v3: Adt8, v4: i64, v5: Str) -> R<Adt110> {
    let m956 = f431(v2.clone(), v3);
    match m956.uncons() {
        None => {
            f432(v0, v1, v4, v5)
        }
        Some((v6, v7)) => {
            let a959 = {
                let a957 = List::cons(v6.clone(), v7.clone());
                length_list(a957)
            };
            let a960 = {
                let a958 = v2;
                length_list(a958)
            };
            if nat_eq(a959, a960) {
                Ok(f341(v0))
            } else {
                Ok(f427(v0.clone(), Adt67::C1(f429(v1, v5), List::cons(v6, v7)), match v0 {
                    Adt89::C0(_, _, h961) => {
                        h961
                    }
                }))
            }
        }
    }
}

pub fn f343(v0: Adt110, v1: Str) -> Adt110 {
    let m965 = {
        let m964 = match v0.clone() {
            Adt110::C0(h963, _) => {
                h963
            }
        };
        match m964 {
            Adt89::C0(h962, _, _) => {
                h962
            }
        }
    };
    match m965 {
        Adt67::C0(v2) => {
            f427(match v0.clone() {
                Adt110::C0(h966, _) => {
                    h966
                }
            }, Adt67::C4(f429(v2, v1)), {
                let m969 = match v0 {
                    Adt110::C0(h968, _) => {
                        h968
                    }
                };
                match m969 {
                    Adt89::C0(_, _, h967) => {
                        h967
                    }
                }
            })
        }
        Adt67::C1(v3, _) => {
            f427(match v0.clone() {
                Adt110::C0(h970, _) => {
                    h970
                }
            }, Adt67::C4(f429(v3, v1)), {
                let m973 = match v0 {
                    Adt110::C0(h972, _) => {
                        h972
                    }
                };
                match m973 {
                    Adt89::C0(_, _, h971) => {
                        h971
                    }
                }
            })
        }
        Adt67::C2(_, _) => {
            v0
        }
        Adt67::C3(_, _) => {
            v0
        }
        Adt67::C4(_) => {
            v0
        }
    }
}

pub fn f344(v0: Bytes, v1: Adt100, v2: u64) -> R<Result<Adt100, Adt2>> {
    let m974 = f433(v0.clone(), v2)?;
    match m974 {
        None => {
            Ok(Err::<Adt100, Adt2>(Adt2::C0(v2)))
        }
        Some(v3) => {
            f434(v0, Adt100::C0(match v1.clone() {
                Adt100::C0(h975, _, _, _, _, _, _, _) => {
                    h975
                }
            }, match v1.clone() {
                Adt100::C0(_, h976, _, _, _, _, _, _) => {
                    h976
                }
            }, match v1.clone() {
                Adt100::C0(_, _, h977, _, _, _, _, _) => {
                    h977
                }
            }, match v1.clone() {
                Adt100::C0(_, _, _, h978, _, _, _, _) => {
                    h978
                }
            }, match v1.clone() {
                Adt100::C0(_, _, _, _, h979, _, _, _) => {
                    h979
                }
            }, match v1.clone() {
                Adt100::C0(_, _, _, _, _, h980, _, _) => {
                    h980
                }
            }, match v3.clone() {
                Adt96::C0(h981, _) => {
                    h981
                }
            }, match v1 {
                Adt100::C0(_, _, _, _, _, _, _, h982) => {
                    h982
                }
            }), match v3 {
                Adt96::C0(_, h983) => {
                    h983
                }
            })
        }
    }
}

pub fn f345(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a987 = {
        let a984 = v1;
        let a985 = 8u64;
        nat_add(a984, a985)?
    };
    let a988 = {
        let a986 = v0.clone();
        length_bytes(a986)
    };
    if nat_le(a987, a988) {
        Ok(Some(f435(f435(f435(f435(f435(f435(f435(f435(0i64, f61(v0.clone(), v1)?)?, f61(v0.clone(), {
            let a989 = v1;
            let a990 = 1u64;
            nat_add(a989, a990)?
        })?)?, f61(v0.clone(), {
            let a991 = v1;
            let a992 = 2u64;
            nat_add(a991, a992)?
        })?)?, f61(v0.clone(), {
            let a993 = v1;
            let a994 = 3u64;
            nat_add(a993, a994)?
        })?)?, f61(v0.clone(), {
            let a995 = v1;
            let a996 = 4u64;
            nat_add(a995, a996)?
        })?)?, f61(v0.clone(), {
            let a997 = v1;
            let a998 = 5u64;
            nat_add(a997, a998)?
        })?)?, f61(v0.clone(), {
            let a999 = v1;
            let a1000 = 6u64;
            nat_add(a999, a1000)?
        })?)?, f61(v0, {
            let a1001 = v1;
            let a1002 = 7u64;
            nat_add(a1001, a1002)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f346(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt2>> {
    let m1003 = f7(v0, v1)?;
    match m1003 {
        None => {
            Ok(Err::<(Str, u64), Adt2>(Adt2::C0(v1)))
        }
        Some(v2) => {
            let m1005 = f436(f29(match v2.clone() {
                Adt96::C0(h1004, _) => {
                    h1004
                }
            })?);
            match m1005 {
                None => {
                    Ok(Err::<(Str, u64), Adt2>(Adt2::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt2>((v3, match v2 {
                        Adt96::C0(_, h1006) => {
                            h1006
                        }
                    })))
                }
            }
        }
    }
}

pub fn f347(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt18, u64), Adt2>> {
    let m1008 = f346(v0.clone(), {
        let (_, h1007) = v2;
        h1007
    })?;
    match m1008 {
        Ok(v4) => {
            f437(v0, v1, {
                let (h1009, _) = v2;
                h1009
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt18, u64), Adt2>(v3))
        }
    }
}

pub fn f348(v0: Adt9) -> R<Result<List<u64>, Adt120>> {
    f438(f439(), v0)
}

pub fn f349() -> u64 {
    403u64
}

pub fn f350(v0: Option<Adt111>, v1: Adt85) -> List<(Str, Adt9)> {
    List::cons((Str::lit("jsonrpc"), Adt9::C3(f440())), {
        let a1010 = f441(v0);
        let a1011 = List::cons((Str::lit("error"), f442(v1)), List::<(Str, Adt9)>::nil());
        append_list(a1010, a1011)
    })
}

pub fn f351() -> i64 {
    -32000i64
}

pub fn f352(v0: Adt107) -> Str {
    let m1012 = v0;
    match m1012 {
        Adt107::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt107::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f353() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f354() -> u64 {
    400u64
}

pub fn f355(v0: Adt121) -> Adt85 {
    Adt85::C0(f443(v0.clone()), f444(v0), None::<Adt9>)
}

pub fn f356(v0: Adt18, v1: Adt3, v2: Fn9, v3: u64, v4: List<Adt101>, v5: Result<Adt127, Adt121>) -> R<Adt104> {
    let m1013 = v5;
    match m1013 {
        Ok(v7) => {
            f447(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f247(f445(v6.clone()), f249(f446(v6.clone()), f355(v6)))
        }
    }
}

pub fn f357(v0: Adt9) -> R<Result<Adt127, Adt121>> {
    let m1014 = v0;
    match m1014 {
        Adt9::C0 => {
            Ok(f449(None::<Adt111>, Adt122::C0))
        }
        Adt9::C1(_) => {
            Ok(f449(None::<Adt111>, Adt122::C0))
        }
        Adt9::C2(_) => {
            Ok(f449(None::<Adt111>, Adt122::C0))
        }
        Adt9::C3(_) => {
            Ok(f449(None::<Adt111>, Adt122::C0))
        }
        Adt9::C4(_) => {
            Ok(f449(None::<Adt111>, Adt122::C1))
        }
        Adt9::C5(v1) => {
            f448(Adt8::C0(v1))
        }
    }
}

pub fn f358(v0: List<u64>) -> R<Result<Adt9, Adt120>> {
    let m1015 = f450(v0.clone())?;
    match m1015 {
        Adt137::C0 => {
            f451(f452(), 0u64, v0)
        }
        Adt137::C1(v1) => {
            Ok(Err::<Adt9, Adt120>(Adt120::C0(v1)))
        }
        Adt137::C2(v2) => {
            Ok(Err::<Adt9, Adt120>(Adt120::C1(v2)))
        }
    }
}

pub fn f359() -> u64 {
    405u64
}

pub fn f360() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f361() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f362(p1020: List<Adt101>, v1: List<u64>, p1021: Adt124) -> R<Adt124> {
    let m1019 = (p1020, p1021);
    {
        let mut m1019 = m1019;
        loop {
            let step = {
                let (v0, v2) = m1019;
                let m1016 = v0;
                match m1016.uncons() {
                    None => {
                        Err::<(List<Adt101>, Adt124), R<Adt124>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f453(match v3.clone() {
                            Adt101::C0(h1017, _) => {
                                h1017
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt101>, Adt124), R<Adt124>>((v4, f454(v2, match v3 {
                                Adt101::C0(_, h1018) => {
                                    h1018
                                }
                            })))
                        } else {
                            Ok::<(List<Adt101>, Adt124), R<Adt124>>((v4, v2))
                        }
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

pub fn f363(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f455(v0);
    if f456(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f364(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f453(v0.clone(), f457())? {
        Ok(true)
    } else {
        if f453(v0.clone(), f458())? {
            Ok(true)
        } else {
            if f453(v0.clone(), f459())? {
                Ok(true)
            } else {
                f460(v0, v1)
            }
        }
    }
}

pub fn f365(p1026: List<u64>) -> Option<List<u64>> {
    let m1025 = p1026;
    {
        let mut m1025 = m1025;
        loop {
            let step = {
                let v0 = m1025;
                let m1022 = v0;
                match m1022.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a1023 = v1;
                        let a1024 = 58u64;
                        if nat_eq(a1023, a1024) {
                            Err::<List<u64>, Option<List<u64>>>(f461(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1025 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m1027 = v0;
    match m1027 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f462({
                let (_, h1028) = v1;
                h1028
            }) {
                Some({
                    let (h1029, _) = v1;
                    h1029
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f367(p1034: List<u64>, p1035: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m1033 = (p1034, p1035);
    {
        let mut m1033 = m1033;
        loop {
            let step = {
                let (v0, v1) = m1033;
                let m1030 = v0;
                match m1030.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a1031 = v2;
                        let a1032 = 93u64;
                        if nat_eq(a1031, a1032) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f463(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1033 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(p1040: List<u64>, p1041: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m1039 = (p1040, p1041);
    {
        let mut m1039 = m1039;
        loop {
            let step = {
                let (v0, v1) = m1039;
                let m1036 = v0;
                match m1036.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f463(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a1037 = v2;
                        let a1038 = 58u64;
                        if nat_eq(a1037, a1038) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f463(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1039 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: Adt18, v1: Adt3, v2: Fn9, v3: bool, v4: u64, v5: List<Adt101>, v6: Result<Adt127, Adt121>) -> R<Adt97> {
    let m1042 = v6;
    match m1042 {
        Ok(v8) => {
            f464(v0, v1, v2, v3, v4, v5, v8)
        }
        Err(v7) => {
            Ok(Adt97::C0(f56(f247(f445(v7.clone()), f249(f446(v7.clone()), f355(v7)))?)))
        }
    }
}

pub fn f370(p1045: List<u64>, v1: bool) -> bool {
    let m1044 = p1045;
    {
        let mut m1044 = m1044;
        loop {
            let step = {
                let v0 = m1044;
                let m1043 = v0;
                match m1043.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f465(v2)
                        } else {
                            f466(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1044 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371() -> u64 {
    64u64
}

pub fn f372(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt102> {
    let m1046 = f383(Str::lit("type"), v1);
    match m1046 {
        Adt118::C0 => {
            Some(Adt102::C3(v0))
        }
        Adt118::C1(v2) => {
            let m1047 = f181(v2);
            match m1047 {
                None => {
                    Some(Adt102::C3(v0))
                }
                Some(v3) => {
                    let a1048 = v3;
                    let a1049 = Str::lit("object");
                    if equal(a1048, a1049) {
                        None::<Adt102>
                    } else {
                        Some(Adt102::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f373(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt102> {
    let m1050 = f383(Str::lit("$schema"), v1);
    match m1050 {
        Adt118::C0 => {
            None::<Adt102>
        }
        Adt118::C1(v2) => {
            let m1051 = f181(v2);
            match m1051 {
                None => {
                    Some(Adt102::C2(v0))
                }
                Some(v3) => {
                    let a1052 = v3;
                    let a1053 = f467();
                    if equal(a1052, a1053) {
                        None::<Adt102>
                    } else {
                        Some(Adt102::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f374(v0: Adt40) -> R<Option<Adt102>> {
    let m1055 = match v0.clone() {
        Adt40::C0(_, _, _, _, _, h1054, _, _) => {
            h1054
        }
    };
    match m1055 {
        None => {
            Ok(None::<Adt102>)
        }
        Some(v1) => {
            let m1058 = f373(match v0.clone() {
                Adt40::C0(h1056, _, _, _, _, _, _, _) => {
                    h1056
                }
            }, match v1.clone() {
                Adt8::C0(h1057) => {
                    h1057
                }
            });
            match m1058 {
                None => {
                    let m1062 = f268(f269(), match v0 {
                        Adt40::C0(h1059, _, _, _, _, _, _, _) => {
                            h1059
                        }
                    }, Adt9::C5(match v1.clone() {
                        Adt8::C0(h1060) => {
                            h1060
                        }
                    }), List::cons(Adt112::C0(Adt9::C5(match v1 {
                        Adt8::C0(h1061) => {
                            h1061
                        }
                    }), false, List::<Str>::nil()), List::<Adt112>::nil()), f270())?;
                    match m1062 {
                        Ok(_) => {
                            Ok(None::<Adt102>)
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

pub fn f375(v0: Str, v1: Adt9, v2: Adt112, v3: Adt125, v4: List<Adt112>) -> R<Result<(List<Adt112>, Adt125), Adt102>> {
    let m1063 = v2;
    match m1063 {
        Adt112::C0(v5, v6, v7) => {
            let m1064 = f468(v5.clone());
            match m1064 {
                None => {
                    let m1065 = f281(v5);
                    match m1065 {
                        None => {
                            Ok(Err::<(List<Adt112>, Adt125), Adt102>(Adt102::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f469(v0, v1, match v9 {
                                Adt8::C0(h1066) => {
                                    h1066
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt112>, Adt125), Adt102>((v4, v3)))
                }
            }
        }
    }
}

pub fn f376(v0: Adt9, v1: Adt9, v2: Adt9, v3: List<Adt114>) -> R<Adt128> {
    let m1067 = f468(v1.clone());
    match m1067 {
        None => {
            let m1068 = f281(v1);
            match m1068 {
                None => {
                    Ok(Adt128::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f470(v0, match v5 {
                        Adt8::C0(h1069) => {
                            h1069
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt128::C3(List::<Adt116>::nil()))
            } else {
                Ok(Adt128::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f377(v0: Adt116, v1: List<Adt116>) -> R<Adt126> {
    let m1070 = v0;
    match m1070 {
        Adt116::C0(_, _, _) => {
            Ok(Adt126::C0(Adt115::C0, v1))
        }
        Adt116::C1(_, _, _) => {
            Ok(Adt126::C0(Adt115::C0, v1))
        }
        Adt116::C2(v8, v9, v10, v11) => {
            Ok(f471(v8, v9, v10, {
                let a1071 = v11;
                let a1072 = 1u64;
                nat_add(a1071, a1072)?
            }, v1))
        }
        Adt116::C3(v12) => {
            Ok(Adt126::C0(Adt115::C1(v12, Str::lit("not")), v1))
        }
        Adt116::C4(v13, _, v15, v16) => {
            Ok(f472(v13, v15, v16, v1))
        }
        Adt116::C5(_, _, _, _) => {
            Ok(Adt126::C0(Adt115::C0, v1))
        }
    }
}

pub fn f378(v0: Adt117, v1: Adt9, v2: Adt9, v3: List<Adt114>) -> Adt128 {
    let m1073 = f281(v1);
    match m1073 {
        None => {
            Adt128::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m1074 = v0;
            match m1074 {
                Adt117::C0 => {
                    f473(match v4 {
                        Adt8::C0(h1075) => {
                            h1075
                        }
                    }, v2, v3)
                }
                Adt117::C1 => {
                    f474(match v4 {
                        Adt8::C0(h1076) => {
                            h1076
                        }
                    }, v2, v3)
                }
                Adt117::C2 => {
                    f475(match v4 {
                        Adt8::C0(h1077) => {
                            h1077
                        }
                    }, v2, v3)
                }
                Adt117::C3 => {
                    f476(match v4 {
                        Adt8::C0(h1078) => {
                            h1078
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f379(v0: Adt116, v1: List<Adt116>, v2: List<Adt114>, v3: Str) -> Adt126 {
    let m1079 = v0;
    match m1079 {
        Adt116::C0(_, _, _) => {
            Adt126::C0(Adt115::C1(v2, v3), v1)
        }
        Adt116::C1(v7, v8, v9) => {
            f477(v7, v8, v9, v1)
        }
        Adt116::C2(v10, v11, v12, v13) => {
            f471(v10, v11, v12, v13, v1)
        }
        Adt116::C3(_) => {
            Adt126::C0(Adt115::C0, v1)
        }
        Adt116::C4(_, v16, v17, v18) => {
            f472(v16, v17, v18, v1)
        }
        Adt116::C5(_, _, _, _) => {
            Adt126::C0(Adt115::C1(v2, v3), v1)
        }
    }
}

pub fn f380(p1082: List<Adt114>, p1083: List<Str>) -> R<List<Str>> {
    let m1081 = (p1082, p1083);
    {
        let mut m1081 = m1081;
        loop {
            let step = {
                let (v0, v1) = m1081;
                let m1080 = v0;
                match m1080.uncons() {
                    None => {
                        Err::<(List<Adt114>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt114>, List<Str>), R<List<Str>>>((v3, List::cons(f478(v2)?, v1)))
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

pub fn f381(p1092: List<Adt43>) -> R<bool> {
    let m1091 = p1092;
    {
        let mut m1091 = m1091;
        loop {
            let step = {
                let v0 = m1091;
                let m1084 = v0;
                match m1084.uncons() {
                    None => {
                        Err::<List<Adt43>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m1085 = v1;
                        match m1085 {
                            Adt43::C0(_) => {
                                Ok::<List<Adt43>, R<bool>>(v2)
                            }
                            Adt43::C1(v4) => {
                                if f479(match v4 {
                                    Adt47::C0(h1086, _, _, _) => {
                                        h1086
                                    }
                                })? {
                                    Ok::<List<Adt43>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt43>, R<bool>>(Ok(false))
                                }
                            }
                            Adt43::C2(v5) => {
                                if f479(match v5 {
                                    Adt48::C0(h1087, _, _, _) => {
                                        h1087
                                    }
                                })? {
                                    Ok::<List<Adt43>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt43>, R<bool>>(Ok(false))
                                }
                            }
                            Adt43::C3(_) => {
                                Ok::<List<Adt43>, R<bool>>(v2)
                            }
                            Adt43::C4(v7) => {
                                let m1089 = match v7 {
                                    Adt50::C0(h1088, _, _) => {
                                        h1088
                                    }
                                };
                                match m1089 {
                                    Adt51::C0(_) => {
                                        Ok::<List<Adt43>, R<bool>>(v2)
                                    }
                                    Adt51::C1(v9) => {
                                        if f479(match v9 {
                                            Adt53::C0(_, _, h1090, _) => {
                                                h1090
                                            }
                                        })? {
                                            Ok::<List<Adt43>, R<bool>>(v2)
                                        } else {
                                            Err::<List<Adt43>, R<bool>>(Ok(false))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1091 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: Adt40, v1: Adt54) -> R<Result<Adt37, Adt94>> {
    if f480(match v1.clone() {
        Adt54::C0(h1093, _) => {
            h1093
        }
    }) {
        f481(v1)
    } else {
        let m1095 = match v0 {
            Adt40::C0(_, _, _, _, _, h1094, _, _) => {
                h1094
            }
        };
        match m1095 {
            None => {
                f481(v1)
            }
            Some(v2) => {
                f482(v2, v1)
            }
        }
    }
}

pub fn f383(v0: Str, p1101: List<(Str, Adt9)>) -> Adt118 {
    let m1100 = p1101;
    {
        let mut m1100 = m1100;
        loop {
            let step = {
                let v1 = m1100;
                let m1096 = v1;
                match m1096.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, Adt118>(Adt118::C0)
                    }
                    Some((v2, v3)) => {
                        let m1098 = f483({
                            let (h1097, _) = v2;
                            h1097
                        }, v0.clone());
                        match m1098 {
                            Adt145::C0 => {
                                Err::<List<(Str, Adt9)>, Adt118>(Adt118::C1({
                                    let (_, h1099) = v2;
                                    h1099
                                }))
                            }
                            Adt145::C1 => {
                                Ok::<List<(Str, Adt9)>, Adt118>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1100 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: Adt26) -> bool {
    f232({
        let m1104 = match v0 {
            Adt26::C0(h1103, _, _, _, _) => {
                h1103
            }
        };
        match m1104 {
            Adt21::C0(_, h1102, _, _, _, _) => {
                h1102
            }
        }
    })
}

pub fn f385(v0: Adt18, v1: Adt90) -> R<Result<Adt37, Adt94>> {
    let a1106 = match v0.clone() {
        Adt18::C0(_, _, h1105, _) => {
            h1105
        }
    };
    let a1107 = Str::lit("");
    if equal(a1106, a1107) {
        Ok(Err::<Adt37, Adt94>(f484()))
    } else {
        let m1109 = f233(match v0.clone() {
            Adt18::C0(_, _, _, h1108) => {
                h1108
            }
        })?;
        match m1109 {
            None => {
                Ok(Err::<Adt37, Adt94>(f234()))
            }
            Some(v2) => {
                let m1112 = f235(match v0.clone() {
                    Adt18::C0(_, _, h1110, _) => {
                        h1110
                    }
                }, v2, match v0.clone() {
                    Adt18::C0(h1111, _, _, _) => {
                        h1111
                    }
                })?;
                match m1112 {
                    None => {
                        Ok(Ok::<Adt37, Adt94>(Adt37::C13(Adt87::C0(Adt88::C0, f425({
                            let m1114 = f486(v0.clone(), v1.clone());
                            match m1114 {
                                Adt89::C0(h1113, _, _) => {
                                    h1113
                                }
                            }
                        }), f486(v0, v1)))))
                    }
                    Some(_) => {
                        Ok(Err::<Adt37, Adt94>(f485()))
                    }
                }
            }
        }
    }
}

pub fn f386(v0: Adt106, v1: Adt40, v2: Adt54) -> R<Result<Adt37, Adt94>> {
    let m1115 = v0;
    match m1115 {
        Adt106::C0 => {
            f274(v1, v2)
        }
        Adt106::C1 => {
            Ok(Err::<Adt37, Adt94>(f236()))
        }
    }
}

pub fn f387(v0: i64) -> bool {
    let a1122 = {
        let m1118 = {
            let a1116 = v0;
            let a1117 = 0i64;
            compare(a1116, a1117)
        };
        if match m1118 {
            Ordering::Less => {
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
            let m1121 = {
                let a1119 = 0i64;
                let a1120 = v0;
                compare(a1119, a1120)
            };
            match m1121 {
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
    bool_not(a1122)
}

pub fn f388(v0: Adt9) -> R<Option<Adt9>> {
    f487(f488(), Adt129::C0(v0), List::<Adt130>::nil())
}

pub fn f389(v0: List<Adt43>) -> R<Option<List<Adt9>>> {
    let m1123 = f489(v0, List::<Adt9>::nil())?;
    match m1123 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f490(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f390(v0: Str, v1: Option<Adt9>) -> List<(Str, Adt9)> {
    let m1124 = v1;
    match m1124 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f391(v0: Str, v1: Option<bool>) -> List<(Str, Adt9)> {
    let m1125 = v1;
    match m1125 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C1(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f392(v0: Str) -> Adt75 {
    Adt75::C0(Adt46::C0, Adt76::C0(Adt77::C0(Adt44::C0(v0, None::<Adt45>, None::<Adt8>))), None::<Adt8>)
}

pub fn f393(p1128: u64, p1129: i64) -> R<i64> {
    let m1127 = (p1128, p1129);
    {
        let mut m1127 = m1127;
        loop {
            let step = {
                let (v0, v1) = m1127;
                let m1126 = v0;
                if m1126 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m1126 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f491(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m1127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: Adt8) -> bool {
    let m1130 = f275(Str::lit("content"), v0);
    match m1130 {
        Adt118::C0 => {
            false
        }
        Adt118::C1(v1) => {
            let m1131 = f281(v1);
            match m1131 {
                None => {
                    false
                }
                Some(v2) => {
                    let m1132 = f275(Str::lit("ok"), v2);
                    match m1132 {
                        Adt118::C0 => {
                            false
                        }
                        Adt118::C1(v3) => {
                            let m1133 = f468(v3);
                            match m1133 {
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

pub fn f395() -> Adt69 {
    f227(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f396(v0: Adt119) -> bool {
    let m1134 = v0;
    match m1134 {
        Adt119::C0 => {
            false
        }
        Adt119::C1 => {
            true
        }
        Adt119::C2(_) => {
            false
        }
    }
}

pub fn f397(v0: Adt119) -> Adt8 {
    let m1135 = v0;
    match m1135 {
        Adt119::C0 => {
            f32()
        }
        Adt119::C1 => {
            f32()
        }
        Adt119::C2(v1) => {
            v1
        }
    }
}

pub fn f398(v0: Adt119) -> bool {
    let m1136 = v0;
    match m1136 {
        Adt119::C0 => {
            true
        }
        Adt119::C1 => {
            false
        }
        Adt119::C2(_) => {
            false
        }
    }
}

pub fn f399() -> Adt69 {
    f227(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f400(v0: Str) -> Str {
    let v1: Bytes = {
        let a1137 = v0;
        utf8_encode(a1137)
    };
    let m1144 = {
        let a1141 = v1.clone();
        let a1142 = 1u64;
        let a1143 = {
            let a1139 = {
                let a1138 = v1;
                length_bytes(a1138)
            };
            let a1140 = 1u64;
            nat_sub(a1139, a1140)
        };
        slice_bytes(a1141, a1142, a1143)
    };
    match m1144 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m1146 = {
                let a1145 = v2;
                utf8_decode(a1145)
            };
            match m1146 {
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

pub fn f401(v0: Option<Adt8>) -> bool {
    let m1147 = v0;
    match m1147 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f402(p1150: List<Str>, p1151: List<Str>) -> List<Str> {
    let m1149 = (p1150, p1151);
    {
        let mut m1149 = m1149;
        loop {
            let step = {
                let (v0, v1) = m1149;
                let m1148 = v0;
                match m1148.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
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

pub fn f403(p1155: List<(Str, Adt9)>, p1156: List<Str>) -> List<Str> {
    let m1154 = (p1155, p1156);
    {
        let mut m1154 = m1154;
        loop {
            let step = {
                let (v0, v1) = m1154;
                let m1152 = v0;
                match m1152.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h1153, _) = v2;
                            h1153
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: u64) -> R<Adt10> {
    Ok(Adt10::C0(f393(v0, 0i64)?, 0i64))
}

pub fn f405(p1159: List<Adt49>, p1160: List<Adt49>) -> List<Adt49> {
    let m1158 = (p1159, p1160);
    {
        let mut m1158 = m1158;
        loop {
            let step = {
                let (v0, v1) = m1158;
                let m1157 = v0;
                match m1157.uncons() {
                    None => {
                        Err::<(List<Adt49>, List<Adt49>), List<Adt49>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt49>, List<Adt49>), List<Adt49>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(p1164: List<(Adt49, List<Adt51>)>, p1165: List<Adt49>) -> List<Adt49> {
    let m1163 = (p1164, p1165);
    {
        let mut m1163 = m1163;
        loop {
            let step = {
                let (v0, v1) = m1163;
                let m1161 = v0;
                match m1161.uncons() {
                    None => {
                        Err::<(List<(Adt49, List<Adt51>)>, List<Adt49>), List<Adt49>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt49, List<Adt51>)>, List<Adt49>), List<Adt49>>((v3, List::cons({
                            let (h1162, _) = v2;
                            h1162
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: Adt49, v1: Adt51) -> (Adt49, List<Adt51>) {
    (v0, List::cons(v1, List::<Adt51>::nil()))
}

pub fn f408(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt49 {
    Adt49::C0(v0, v1, None::<List<Adt5>>, v2, Some(v3), Some(v4), None::<Adt45>, None::<u64>, None::<Adt8>)
}

pub fn f409(v0: Str, v1: Str, v2: Str) -> Adt51 {
    Adt51::C1(Adt53::C0(v0, Some(v1), v2, None::<Adt8>))
}

pub fn f410(v0: Str) -> R<Option<Str>> {
    let m1166 = f492(v0)?;
    match m1166 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a1167 = f493(f243(), v1);
            Ok(utf8_decode(a1167))
        }
    }
}

pub fn f411(v0: Str) -> R<Option<Str>> {
    let m1170 = f494(Adt9::C5(List::cons((Str::lit("id"), Adt9::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt9::C1(true)), List::cons((Str::lit("data"), Adt9::C3({
        let a1168 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a1169 = Str::lit(" ");
        join(a1168, a1169)
    })), List::<(Str, Adt9)>::nil())))))?;
    match m1170 {
        Ok(v1) => {
            let a1171 = v1;
            Ok(utf8_decode(a1171))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f412(v0: Adt8) -> bool {
    if f495(v0.clone()) {
        f496(v0)
    } else {
        false
    }
}

pub fn f413(v0: Adt8) -> bool {
    if f497(v0.clone()) {
        if f498(f275(Str::lit("model"), v0.clone())) {
            f499(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f414(v0: Adt8) -> bool {
    let m1172 = f275(Str::lit("roots"), v0);
    match m1172 {
        Adt118::C0 => {
            false
        }
        Adt118::C1(v1) => {
            let m1173 = f335(v1);
            match m1173 {
                None => {
                    false
                }
                Some(v2) => {
                    f500(v2)
                }
            }
        }
    }
}

pub fn f415(v0: Adt62) -> bool {
    let m1175 = match v0 {
        Adt62::C0(_, _, _, h1174) => {
            h1174
        }
    };
    match m1175 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f416(v0: Str, p1182: List<(Str, Str)>) -> Option<Str> {
    let m1181 = p1182;
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let v1 = m1181;
                let m1176 = v1;
                match m1176.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a1178 = {
                            let (h1177, _) = v2;
                            h1177
                        };
                        let a1179 = v0.clone();
                        if equal(a1178, a1179) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h1180) = v2;
                                h1180
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
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

pub fn f417(p1187: List<Str>, p1188: u64) -> R<u64> {
    let m1186 = (p1187, p1188);
    {
        let mut m1186 = m1186;
        loop {
            let step = {
                let (v0, v1) = m1186;
                let m1183 = v0;
                match m1183.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a1184 = v1;
                            let a1185 = 1u64;
                            nat_add(a1184, a1185)?
                        }))
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

pub fn f418(v0: List<Str>) -> List<Str> {
    f402(f501(v0, f419(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f419() -> u64 {
    100u64
}

pub fn f420(v0: Str, p1191: List<Str>, p1192: List<Str>) -> R<List<Str>> {
    let m1190 = (p1191, p1192);
    {
        let mut m1190 = m1190;
        loop {
            let step = {
                let (v1, v2) = m1190;
                let m1189 = v1;
                match m1189.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f502(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421() -> List<Str> {
    f402(f503(f214(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f422(v0: Str, v1: Adt8) -> bool {
    f504(f275(v0, v1))
}

pub fn f423(p1195: List<Adt89>, p1196: List<Adt89>) -> List<Adt89> {
    let m1194 = (p1195, p1196);
    {
        let mut m1194 = m1194;
        loop {
            let step = {
                let (v0, v1) = m1194;
                let m1193 = v0;
                match m1193.uncons() {
                    None => {
                        Err::<(List<Adt89>, List<Adt89>), List<Adt89>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt89>, List<Adt89>), List<Adt89>>((v3, List::cons(v2, v1)))
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

pub fn f424(v0: Adt9) -> Option<Adt89> {
    let m1197 = f281(v0);
    match m1197 {
        None => {
            None::<Adt89>
        }
        Some(v1) => {
            let m1198 = f505(Str::lit("task"), v1.clone());
            match m1198 {
                None => {
                    None::<Adt89>
                }
                Some(v2) => {
                    let m1199 = f506(v2);
                    match m1199 {
                        None => {
                            None::<Adt89>
                        }
                        Some(v3) => {
                            let m1200 = f507(Str::lit("createdMs"), v1.clone());
                            match m1200 {
                                None => {
                                    None::<Adt89>
                                }
                                Some(v4) => {
                                    let m1201 = f505(Str::lit("plan"), v1);
                                    match m1201 {
                                        None => {
                                            None::<Adt89>
                                        }
                                        Some(v5) => {
                                            let m1202 = f508(v5);
                                            match m1202 {
                                                None => {
                                                    None::<Adt89>
                                                }
                                                Some(v6) => {
                                                    Some(Adt89::C0(v3, v4, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f425(v0: Adt67) -> Adt68 {
    let m1203 = v0;
    match m1203 {
        Adt67::C0(v1) => {
            v1
        }
        Adt67::C1(v2, _) => {
            v2
        }
        Adt67::C2(v4, _) => {
            v4
        }
        Adt67::C3(v6, _) => {
            v6
        }
        Adt67::C4(v8) => {
            v8
        }
    }
}

pub fn f426(v0: i64, v1: i64, v2: i64) -> R<bool> {
    let a1209 = {
        let m1208 = {
            let a1206 = {
                let a1204 = v2;
                let a1205 = v1;
                int_sub(a1204, a1205)?
            };
            let a1207 = v0;
            compare(a1206, a1207)
        };
        match m1208 {
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
    Ok(bool_not(a1209))
}

pub fn f427(v0: Adt89, v1: Adt67, v2: Adt90) -> Adt110 {
    Adt110::C0(Adt89::C0(v1, match v0 {
        Adt89::C0(_, h1210, _) => {
            h1210
        }
    }, v2), true)
}

pub fn f428(v0: Adt91, v1: Adt68) -> Adt67 {
    let m1211 = v0;
    match m1211 {
        Adt91::C0(v2) => {
            Adt67::C2(v1, v2)
        }
        Adt91::C1(v3) => {
            Adt67::C3(v1, v3)
        }
    }
}

pub fn f429(v0: Adt68, v1: Str) -> Adt68 {
    Adt68::C0(match v0.clone() {
        Adt68::C0(h1212, _, _, _, _, _) => {
            h1212
        }
    }, match v0.clone() {
        Adt68::C0(_, h1213, _, _, _, _) => {
            h1213
        }
    }, match v0.clone() {
        Adt68::C0(_, _, h1214, _, _, _) => {
            h1214
        }
    }, v1, match v0.clone() {
        Adt68::C0(_, _, _, _, h1215, _) => {
            h1215
        }
    }, match v0 {
        Adt68::C0(_, _, _, _, _, h1216) => {
            h1216
        }
    })
}

pub fn f430(v0: Adt90) -> Adt90 {
    Adt90::C0(match v0.clone() {
        Adt90::C0(h1217, _, _, _) => {
            h1217
        }
    }, List::<(Str, Adt69)>::nil(), match v0.clone() {
        Adt90::C0(_, _, h1218, _) => {
            h1218
        }
    }, match v0 {
        Adt90::C0(_, _, _, h1219) => {
            h1219
        }
    })
}

pub fn f431(v0: List<(Str, Adt69)>, v1: Adt8) -> List<(Str, Adt69)> {
    f509(f510(v0, v1, List::<(Str, Adt69)>::nil()), List::<(Str, Adt69)>::nil())
}

pub fn f432(v0: Adt89, v1: Adt68, v2: i64, v3: Str) -> R<Adt110> {
    let v4: Adt89 = Adt89::C0(Adt67::C0(f429(v1, v3.clone())), match v0.clone() {
        Adt89::C0(_, h1220, _) => {
            h1220
        }
    }, match v0 {
        Adt89::C0(_, _, h1221) => {
            h1221
        }
    });
    Ok(Adt110::C0({
        let m1223 = f238(v4, v2, v3)?;
        match m1223 {
            Adt110::C0(h1222, _) => {
                h1222
            }
        }
    }, true))
}

pub fn f433(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let a1225 = v1;
    let a1226 = {
        let a1224 = v0.clone();
        length_bytes(a1224)
    };
    if nat_eq(a1225, a1226) {
        Ok(Some(Adt96::C0(Bytes::lit(&[]), v1)))
    } else {
        f7(v0, v1)
    }
}

pub fn f434(v0: Bytes, v1: Adt100, v2: u64) -> R<Result<Adt100, Adt2>> {
    let a1228 = v2;
    let a1229 = {
        let a1227 = v0.clone();
        length_bytes(a1227)
    };
    if nat_eq(a1228, a1229) {
        Ok(Ok::<Adt100, Adt2>(v1))
    } else {
        let m1230 = f27(v0.clone(), v2)?;
        match m1230 {
            None => {
                Ok(Err::<Adt100, Adt2>(Adt2::C0(v2)))
            }
            Some(v3) => {
                let m1233 = f511(v0.clone(), {
                    let a1231 = v2;
                    let a1232 = 4u64;
                    nat_add(a1231, a1232)?
                }, v3, List::<List<u64>>::nil())?;
                match m1233 {
                    Ok(v5) => {
                        let a1236 = {
                            let (_, h1234) = v5.clone();
                            h1234
                        };
                        let a1237 = {
                            let a1235 = v0;
                            length_bytes(a1235)
                        };
                        if nat_eq(a1236, a1237) {
                            Ok(Ok::<Adt100, Adt2>(f512(v1, {
                                let (h1238, _) = v5;
                                h1238
                            })))
                        } else {
                            Ok(Err::<Adt100, Adt2>(Adt2::C1({
                                let (_, h1239) = v5;
                                h1239
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt100, Adt2>(v4))
                    }
                }
            }
        }
    }
}

pub fn f435(v0: i64, v1: u64) -> R<i64> {
    f491({
        let a1240 = v0;
        let a1241 = 256i64;
        int_mul(a1240, a1241)?
    }, f393(v1, 0i64)?)
}

pub fn f436(v0: List<u64>) -> Option<Str> {
    let a1242 = f493(f243(), v0);
    utf8_decode(a1242)
}

pub fn f437(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt18, u64), Adt2>> {
    let m1244 = f7(v0, {
        let (_, h1243) = v3.clone();
        h1243
    })?;
    match m1244 {
        None => {
            Ok(Err::<(Adt18, u64), Adt2>(Adt2::C0({
                let (_, h1245) = v3;
                h1245
            })))
        }
        Some(v4) => {
            Ok(f513(v1, v2, {
                let (h1246, _) = v3;
                h1246
            }, v4))
        }
    }
}

pub fn f438(v0: u64, v1: Adt9) -> R<Result<List<u64>, Adt120>> {
    let m1247 = f514(v0, List::cons(Adt131::C1(0u64, v1), List::<Adt131>::nil()), List::<u64>::nil())?;
    match m1247 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt120>(f515(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt120>(v3))
        }
    }
}

pub fn f439() -> u64 {
    67108864u64
}

pub fn f440() -> Str {
    Str::lit("2.0")
}

pub fn f441(v0: Option<Adt111>) -> List<(Str, Adt9)> {
    let m1248 = v0;
    match m1248 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f516(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f442(v0: Adt85) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("code"), Adt9::C2(Adt10::C0(match v0.clone() {
        Adt85::C0(h1249, _, _) => {
            h1249
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt9::C3(match v0.clone() {
        Adt85::C0(_, h1250, _) => {
            h1250
        }
    })), f517(match v0 {
        Adt85::C0(_, _, h1251) => {
            h1251
        }
    }))))
}

pub fn f443(v0: Adt121) -> i64 {
    let m1252 = v0;
    match m1252 {
        Adt121::C0 => {
            f518()
        }
        Adt121::C1(_, _) => {
            f519()
        }
    }
}

pub fn f444(v0: Adt121) -> Str {
    let m1253 = v0;
    match m1253 {
        Adt121::C0 => {
            Str::lit("Parse error")
        }
        Adt121::C1(_, v2) => {
            f520(v2)
        }
    }
}

pub fn f445(v0: Adt121) -> u64 {
    let m1254 = v0;
    match m1254 {
        Adt121::C0 => {
            f354()
        }
        Adt121::C1(_, _) => {
            f354()
        }
    }
}

pub fn f446(v0: Adt121) -> Option<Adt111> {
    let m1255 = v0;
    match m1255 {
        Adt121::C0 => {
            None::<Adt111>
        }
        Adt121::C1(v1, _) => {
            v1
        }
    }
}

pub fn f447(v0: Adt18, v1: Adt3, v2: Fn9, v3: u64, v4: List<Adt101>, v5: Adt127) -> R<Adt104> {
    f521(v0, f522(v4.clone())?, f523(v1.clone(), v2, v3, Some(f524(v4, match v1 {
        Adt3::C0(_, _, _, _, h1256, _, _, _) => {
            h1256
        }
    }, f525(v5.clone()), match v5.clone() {
        Adt127::C0(_, h1257, _) => {
            h1257
        }
    })?), f526(), v5)?)
}

pub fn f448(v0: Adt8) -> R<Result<Adt127, Adt121>> {
    let m1258 = f527(v0.clone())?;
    match m1258 {
        Adt153::C0 => {
            let m1259 = f275(Str::lit("method"), v0.clone());
            match m1259 {
                Adt118::C0 => {
                    Ok(f449(None::<Adt111>, f528(v0)))
                }
                Adt118::C1(_) => {
                    Ok(f449(None::<Adt111>, if f529(v0) {
                        Adt122::C8
                    } else {
                        Adt122::C7
                    }))
                }
            }
        }
        Adt153::C1(v2) => {
            Ok(f530(v2, v0))
        }
        Adt153::C2 => {
            Ok(f449(None::<Adt111>, Adt122::C5))
        }
    }
}

pub fn f449(v0: Option<Adt111>, v1: Adt122) -> Result<Adt127, Adt121> {
    Err::<Adt127, Adt121>(Adt121::C1(v0, v1))
}

pub fn f450(v0: List<u64>) -> R<Adt137> {
    Ok(f531({
        let a1260 = v0.clone();
        length_list(a1260)
    }, f532(Adt138::C0, 0u64, 0u64, v0)?))
}

pub fn f451(p1267: Adt139, p1268: u64, p1269: List<u64>) -> R<Result<Adt9, Adt120>> {
    let m1266 = (p1267, (p1268, p1269));
    {
        let mut m1266 = m1266;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1266;
                let m1261 = v2;
                match m1261.uncons() {
                    None => {
                        let m1262 = f533(v0, v1)?;
                        match m1262 {
                            Ok(v3) => {
                                Err::<(Adt139, (u64, List<u64>)), R<Result<Adt9, Adt120>>>(Ok(Ok::<Adt9, Adt120>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt139, (u64, List<u64>)), R<Result<Adt9, Adt120>>>(Ok(Err::<Adt9, Adt120>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m1263 = f534(v0, v1, v5)?;
                        match m1263 {
                            Adt154::C0(v7) => {
                                Err::<(Adt139, (u64, List<u64>)), R<Result<Adt9, Adt120>>>(Ok(Err::<Adt9, Adt120>(v7)))
                            }
                            Adt154::C1(v8) => {
                                Ok::<(Adt139, (u64, List<u64>)), R<Result<Adt9, Adt120>>>((v8, ({
                                    let a1264 = v1;
                                    let a1265 = 1u64;
                                    nat_add(a1264, a1265)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452() -> Adt139 {
    Adt139::C0(Adt140::C0, List::<Adt144>::nil(), 0u64)
}

pub fn f453(p1276: List<u64>, p1277: List<u64>) -> R<bool> {
    let m1275 = (p1276, p1277);
    {
        let mut m1275 = m1275;
        loop {
            let step = {
                let (v0, v1) = m1275;
                let m1270 = v0;
                match m1270.uncons() {
                    None => {
                        let m1271 = v1;
                        match m1271.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1272 = v1;
                        match m1272.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a1273 = f535(v4)?;
                                let a1274 = f535(v6)?;
                                if nat_eq(a1273, a1274) {
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
                Ok(next) => { m1275 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: Adt124, v1: List<u64>) -> Adt124 {
    let m1278 = v0;
    match m1278 {
        Adt124::C0 => {
            Adt124::C1(v1)
        }
        Adt124::C1(_) => {
            Adt124::C2
        }
        Adt124::C2 => {
            Adt124::C2
        }
    }
}

pub fn f455(v0: List<u64>) -> List<u64> {
    f463(f536(f463(f536(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f456(p1281: List<u64>) -> bool {
    let m1280 = p1281;
    {
        let mut m1280 = m1280;
        loop {
            let step = {
                let v0 = m1280;
                let m1279 = v0;
                match m1279.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f537(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1280 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f458() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f459() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f460(v0: List<u64>, p1284: List<List<u64>>) -> R<bool> {
    let m1283 = p1284;
    {
        let mut m1283 = m1283;
        loop {
            let step = {
                let v1 = m1283;
                let m1282 = v1;
                match m1282.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f453(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
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

pub fn f461(v0: List<u64>) -> Option<List<u64>> {
    let m1285 = v0;
    match m1285.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m1286 = v2;
            match m1286.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a1287 = v1;
                    let a1288 = 47u64;
                    if if nat_eq(a1287, a1288) {
                        let a1289 = v3;
                        let a1290 = 47u64;
                        nat_eq(a1289, a1290)
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

pub fn f462(v0: List<u64>) -> bool {
    let m1291 = v0;
    match m1291.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a1292 = v1;
            let a1293 = 58u64;
            if nat_eq(a1292, a1293) {
                let m1294 = v2.clone();
                match m1294.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f538(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f463(p1297: List<u64>, p1298: List<u64>) -> List<u64> {
    let m1296 = (p1297, p1298);
    {
        let mut m1296 = m1296;
        loop {
            let step = {
                let (v0, v1) = m1296;
                let m1295 = v0;
                match m1295.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464(v0: Adt18, v1: Adt3, v2: Fn9, v3: bool, v4: u64, v5: List<Adt101>, v6: Adt127) -> R<Adt97> {
    f539(v1.clone(), v3, v0, f522(v5.clone())?, f523(v1.clone(), v2, v4, Some(f524(v5, match v1 {
        Adt3::C0(_, _, _, _, h1299, _, _, _) => {
            h1299
        }
    }, f525(v6.clone()), match v6.clone() {
        Adt127::C0(_, h1300, _) => {
            h1300
        }
    })?), f526(), v6)?)
}

pub fn f465(v0: u64) -> bool {
    let a1301 = 48u64;
    let a1302 = v0;
    if if nat_le(a1301, a1302) {
        let a1303 = v0;
        let a1304 = 57u64;
        nat_le(a1303, a1304)
    } else {
        false
    } {
        true
    } else {
        let a1305 = 65u64;
        let a1306 = v0;
        if if nat_le(a1305, a1306) {
            let a1307 = v0;
            let a1308 = 90u64;
            nat_le(a1307, a1308)
        } else {
            false
        } {
            true
        } else {
            let a1309 = 97u64;
            let a1310 = v0;
            if if nat_le(a1309, a1310) {
                let a1311 = v0;
                let a1312 = 122u64;
                nat_le(a1311, a1312)
            } else {
                false
            } {
                true
            } else {
                let a1313 = v0;
                let a1314 = 33u64;
                if nat_eq(a1313, a1314) {
                    true
                } else {
                    let a1315 = v0;
                    let a1316 = 35u64;
                    if nat_eq(a1315, a1316) {
                        true
                    } else {
                        let a1317 = v0;
                        let a1318 = 36u64;
                        if nat_eq(a1317, a1318) {
                            true
                        } else {
                            let a1319 = v0;
                            let a1320 = 37u64;
                            if nat_eq(a1319, a1320) {
                                true
                            } else {
                                let a1321 = v0;
                                let a1322 = 38u64;
                                if nat_eq(a1321, a1322) {
                                    true
                                } else {
                                    let a1323 = v0;
                                    let a1324 = 39u64;
                                    if nat_eq(a1323, a1324) {
                                        true
                                    } else {
                                        let a1325 = v0;
                                        let a1326 = 42u64;
                                        if nat_eq(a1325, a1326) {
                                            true
                                        } else {
                                            let a1327 = v0;
                                            let a1328 = 43u64;
                                            if nat_eq(a1327, a1328) {
                                                true
                                            } else {
                                                let a1329 = v0;
                                                let a1330 = 45u64;
                                                if nat_eq(a1329, a1330) {
                                                    true
                                                } else {
                                                    let a1331 = v0;
                                                    let a1332 = 46u64;
                                                    if nat_eq(a1331, a1332) {
                                                        true
                                                    } else {
                                                        let a1333 = v0;
                                                        let a1334 = 94u64;
                                                        if nat_eq(a1333, a1334) {
                                                            true
                                                        } else {
                                                            let a1335 = v0;
                                                            let a1336 = 95u64;
                                                            if nat_eq(a1335, a1336) {
                                                                true
                                                            } else {
                                                                let a1337 = v0;
                                                                let a1338 = 96u64;
                                                                if nat_eq(a1337, a1338) {
                                                                    true
                                                                } else {
                                                                    let a1339 = v0;
                                                                    let a1340 = 124u64;
                                                                    if nat_eq(a1339, a1340) {
                                                                        true
                                                                    } else {
                                                                        let a1341 = v0;
                                                                        let a1342 = 126u64;
                                                                        nat_eq(a1341, a1342)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f466(v0: u64) -> bool {
    let a1343 = 48u64;
    let a1344 = v0;
    if if nat_le(a1343, a1344) {
        let a1345 = v0;
        let a1346 = 57u64;
        nat_le(a1345, a1346)
    } else {
        false
    } {
        true
    } else {
        let a1347 = 65u64;
        let a1348 = v0;
        if if nat_le(a1347, a1348) {
            let a1349 = v0;
            let a1350 = 90u64;
            nat_le(a1349, a1350)
        } else {
            false
        } {
            true
        } else {
            let a1351 = 97u64;
            let a1352 = v0;
            if if nat_le(a1351, a1352) {
                let a1353 = v0;
                let a1354 = 122u64;
                nat_le(a1353, a1354)
            } else {
                false
            } {
                true
            } else {
                let a1355 = v0;
                let a1356 = 95u64;
                if nat_eq(a1355, a1356) {
                    true
                } else {
                    let a1357 = v0;
                    let a1358 = 45u64;
                    if nat_eq(a1357, a1358) {
                        true
                    } else {
                        let a1359 = v0;
                        let a1360 = 46u64;
                        nat_eq(a1359, a1360)
                    }
                }
            }
        }
    }
}

pub fn f467() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f468(v0: Adt9) -> Option<bool> {
    let m1361 = v0;
    match m1361 {
        Adt9::C0 => {
            None::<bool>
        }
        Adt9::C1(v1) => {
            Some(v1)
        }
        Adt9::C2(_) => {
            None::<bool>
        }
        Adt9::C3(_) => {
            None::<bool>
        }
        Adt9::C4(_) => {
            None::<bool>
        }
        Adt9::C5(_) => {
            None::<bool>
        }
    }
}

pub fn f469(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>, v3: bool, v4: List<Str>, v5: Adt125, v6: List<Adt112>) -> R<Result<(List<Adt112>, Adt125), Adt102>> {
    let m1362 = f540(v0.clone(), v1, v2.clone());
    match m1362 {
        None => {
            let m1363 = f541(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1363 {
                Adt155::C0(v8) => {
                    Ok(Err::<(List<Adt112>, Adt125), Adt102>(v8))
                }
                Adt155::C1(v9) => {
                    Ok(Ok::<(List<Adt112>, Adt125), Adt102>((f542(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt112>, Adt125), Adt102>(v7))
        }
    }
}

pub fn f470(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt114>) -> R<Adt128> {
    let m1364 = f543(v1.clone(), v2.clone())?;
    match m1364 {
        Adt128::C0(v4) => {
            Ok(Adt128::C0(v4))
        }
        Adt128::C1(v5) => {
            Ok(Adt128::C1(v5))
        }
        Adt128::C2(v6) => {
            Ok(Adt128::C2(v6))
        }
        Adt128::C3(_) => {
            f544(v0, v1, v2, v3)
        }
    }
}

pub fn f471(v0: List<Adt9>, v1: Adt9, v2: List<Adt114>, v3: u64, v4: List<Adt116>) -> Adt126 {
    let m1365 = v0;
    match m1365.uncons() {
        None => {
            let a1366 = v3;
            let a1367 = 1u64;
            if nat_eq(a1366, a1367) {
                Adt126::C0(Adt115::C0, v4)
            } else {
                Adt126::C0(Adt115::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt126::C0(Adt115::C0, List::cons(Adt116::C0(v5, v1.clone(), v2.clone()), List::cons(Adt116::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f472(v0: Option<Adt9>, v1: Adt9, v2: List<Adt114>, v3: List<Adt116>) -> Adt126 {
    let m1368 = v0;
    match m1368 {
        None => {
            Adt126::C0(Adt115::C0, v3)
        }
        Some(v4) => {
            Adt126::C0(Adt115::C0, List::cons(Adt116::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f473(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt114>) -> Adt128 {
    let m1369 = f383(Str::lit("anyOf"), v0);
    match m1369 {
        Adt118::C0 => {
            Adt128::C3(List::<Adt116>::nil())
        }
        Adt118::C1(v3) => {
            let m1370 = f335(v3);
            match m1370 {
                None => {
                    Adt128::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1371 = v4;
                    match m1371.uncons() {
                        None => {
                            Adt128::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt128::C3(List::cons(Adt116::C0(v5, v1.clone(), v2.clone()), List::cons(Adt116::C1(v6, v1, v2), List::<Adt116>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f474(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt114>) -> Adt128 {
    let m1372 = f383(Str::lit("oneOf"), v0);
    match m1372 {
        Adt118::C0 => {
            Adt128::C3(List::<Adt116>::nil())
        }
        Adt118::C1(v3) => {
            let m1373 = f335(v3);
            match m1373 {
                None => {
                    Adt128::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1374 = v4;
                    match m1374.uncons() {
                        None => {
                            Adt128::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt128::C3(List::cons(Adt116::C0(v5, v1.clone(), v2.clone()), List::cons(Adt116::C2(v6, v1, v2, 0u64), List::<Adt116>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f475(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt114>) -> Adt128 {
    let m1375 = f383(Str::lit("not"), v0);
    match m1375 {
        Adt118::C0 => {
            Adt128::C3(List::<Adt116>::nil())
        }
        Adt118::C1(v3) => {
            Adt128::C3(List::cons(Adt116::C0(v3, v1, v2.clone()), List::cons(Adt116::C3(v2), List::<Adt116>::nil())))
        }
    }
}

pub fn f476(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt114>) -> Adt128 {
    let m1376 = f383(Str::lit("if"), v0.clone());
    match m1376 {
        Adt118::C0 => {
            Adt128::C3(List::<Adt116>::nil())
        }
        Adt118::C1(v3) => {
            Adt128::C3(List::cons(Adt116::C0(v3, v1.clone(), v2.clone()), List::cons(Adt116::C4(f545(Str::lit("then"), v0.clone()), f545(Str::lit("else"), v0), v1, v2), List::<Adt116>::nil())))
        }
    }
}

pub fn f477(v0: List<Adt9>, v1: Adt9, v2: List<Adt114>, v3: List<Adt116>) -> Adt126 {
    let m1377 = v0;
    match m1377.uncons() {
        None => {
            Adt126::C0(Adt115::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt126::C0(Adt115::C0, List::cons(Adt116::C0(v4, v1.clone(), v2.clone()), List::cons(Adt116::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f478(v0: Adt114) -> R<Str> {
    let m1378 = v0;
    match m1378 {
        Adt114::C0(v1) => {
            f546(v1)
        }
        Adt114::C1(v2) => {
            let a1379 = f393(v2, 0i64)?;
            Ok(format_i64(a1379))
        }
    }
}

pub fn f479(v0: Str) -> R<bool> {
    let m1381 = f547(f29({
        let a1380 = v0;
        utf8_encode(a1380)
    })?)?;
    match m1381 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f480(v0: Adt42) -> bool {
    let m1383 = match v0 {
        Adt42::C0(_, _, h1382) => {
            h1382
        }
    };
    match m1383 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f481(v0: Adt54) -> R<Result<Adt37, Adt94>> {
    let m1385 = match v0.clone() {
        Adt54::C0(_, h1384) => {
            h1384
        }
    };
    match m1385.uncons() {
        None => {
            Ok(Ok::<Adt37, Adt94>(Adt37::C2(match v0 {
                Adt54::C0(h1386, _) => {
                    h1386
                }
            })))
        }
        Some((_, _)) => {
            if f548(match v0.clone() {
                Adt54::C0(_, h1387) => {
                    h1387
                }
            })? {
                Ok(Ok::<Adt37, Adt94>(Adt37::C3(v0)))
            } else {
                Ok(Err::<Adt37, Adt94>(Adt94::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f482(v0: Adt8, v1: Adt54) -> R<Result<Adt37, Adt94>> {
    let m1391 = {
        let m1390 = match v1.clone() {
            Adt54::C0(h1389, _) => {
                h1389
            }
        };
        match m1390 {
            Adt42::C0(_, h1388, _) => {
                h1388
            }
        }
    };
    match m1391 {
        None => {
            Ok(Err::<Adt37, Adt94>(Adt94::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1394 = f169(Adt9::C5(match v0.clone() {
                Adt8::C0(h1392) => {
                    h1392
                }
            }), Adt9::C5(match v0 {
                Adt8::C0(h1393) => {
                    h1393
                }
            }), v2)?;
            match m1394 {
                Adt113::C0 => {
                    f481(v1)
                }
                Adt113::C1(_, _) => {
                    Ok(Err::<Adt37, Adt94>(Adt94::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt113::C2(_) => {
                    Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt113::C3(_) => {
                    Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt113::C4 => {
                    Ok(Err::<Adt37, Adt94>(Adt94::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f483(v0: Str, v1: Str) -> Adt145 {
    let a1395 = v0;
    let a1396 = v1;
    if equal(a1395, a1396) {
        Adt145::C0
    } else {
        Adt145::C1
    }
}

pub fn f484() -> Adt94 {
    Adt94::C9(Str::lit("the host supplied no task id"))
}

pub fn f485() -> Adt94 {
    Adt94::C9(Str::lit("the host supplied a task id that a task already has"))
}

pub fn f486(v0: Adt18, v1: Adt90) -> Adt89 {
    Adt89::C0(Adt67::C0(Adt68::C0(match v0.clone() {
        Adt18::C0(_, _, h1397, _) => {
            h1397
        }
    }, None::<Str>, match v0.clone() {
        Adt18::C0(_, h1398, _, _) => {
            h1398
        }
    }, match v0.clone() {
        Adt18::C0(_, h1399, _, _) => {
            h1399
        }
    }, Some(f549()), Some(f550()))), match v0 {
        Adt18::C0(h1400, _, _, _) => {
            h1400
        }
    }, v1)
}

pub fn f487(p1415: u64, p1416: Adt129, p1417: List<Adt130>) -> R<Option<Adt9>> {
    let m1414 = (p1415, (p1416, p1417));
    {
        let mut m1414 = m1414;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1414;
                let m1401 = v0;
                if m1401 == 0 {
                    Err::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>(Ok(None::<Adt9>))
                } else {
                    let v3 = m1401 - 1;
                    let m1402 = v1;
                    match m1402 {
                        Adt129::C0(v4) => {
                            let m1403 = v4.clone();
                            match m1403 {
                                Adt9::C0 => {
                                    Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C1(v4), v2)))
                                }
                                Adt9::C1(_) => {
                                    Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C1(v4), v2)))
                                }
                                Adt9::C2(_) => {
                                    Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C1(v4), v2)))
                                }
                                Adt9::C3(v7) => {
                                    let v8: Str = f551(v7)?;
                                    Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C1(Adt9::C3(v8)), v2)))
                                }
                                Adt9::C4(v9) => {
                                    let m1404 = v9;
                                    match m1404.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C0(v10), List::cons(Adt130::C0(List::<Adt9>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt9::C5(v12) => {
                                    let m1405 = v12;
                                    match m1405.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f551({
                                                let (h1406, _) = v13;
                                                h1406
                                            })?;
                                            Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C0({
                                                let (_, h1407) = v13;
                                                h1407
                                            }), List::cons(Adt130::C1(v15, List::<(Str, Adt9)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt129::C1(v16) => {
                            let m1408 = v2;
                            match m1408.uncons() {
                                None => {
                                    Err::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1409 = v17;
                                    match m1409 {
                                        Adt130::C0(v19, v20) => {
                                            let v21: List<Adt9> = List::cons(v16, v19);
                                            let m1410 = v20;
                                            match m1410.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C1(Adt9::C4(f490(v21, List::<Adt9>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C0(v22), List::cons(Adt130::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt130::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt9)> = List::cons((v24, v16), v25);
                                            let m1411 = v26;
                                            match m1411.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C1(Adt9::C5(f552(v27, List::<(Str, Adt9)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f551({
                                                        let (h1412, _) = v28;
                                                        h1412
                                                    })?;
                                                    Ok::<(u64, (Adt129, List<Adt130>)), R<Option<Adt9>>>((v3, (Adt129::C0({
                                                        let (_, h1413) = v28;
                                                        h1413
                                                    }), List::cons(Adt130::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m1414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488() -> u64 {
    67108864u64
}

pub fn f489(p1421: List<Adt43>, p1422: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m1420 = (p1421, p1422);
    {
        let mut m1420 = m1420;
        loop {
            let step = {
                let (v0, v1) = m1420;
                let m1418 = v0;
                match m1418.uncons() {
                    None => {
                        Err::<(List<Adt43>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1419 = f553(v2)?;
                        match m1419 {
                            None => {
                                Err::<(List<Adt43>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(None::<List<Adt9>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt43>, List<Adt9>), R<Option<List<Adt9>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1420 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(p1425: List<Adt9>, p1426: List<Adt9>) -> List<Adt9> {
    let m1424 = (p1425, p1426);
    {
        let mut m1424 = m1424;
        loop {
            let step = {
                let (v0, v1) = m1424;
                let m1423 = v0;
                match m1423.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1424 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f491(v0: i64, v1: i64) -> R<i64> {
    let a1428 = v0;
    let a1429 = {
        let a1427 = v1;
        int_neg(a1427)?
    };
    int_sub(a1428, a1429)
}

pub fn f492(v0: Str) -> R<Option<List<u64>>> {
    let m1430 = f554(f555(f556())?, f555(v0)?);
    match m1430 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1431 = f554(f557(List::cons(47u64, f555(f558())?)), f557(v1));
            match m1431 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f557(v2);
                    if if f559(v3.clone()) {
                        f560(v3.clone())
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

pub fn f493(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1432 = v0;
    let a1433 = f561(f243(), f562(List::<Bytes>::nil(), v1));
    append_bytes(a1432, a1433)
}

pub fn f494(v0: Adt9) -> R<Result<Bytes, Adt120>> {
    let m1434 = f348(v0)?;
    match m1434 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt120>(f493(f243(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt120>(v2))
        }
    }
}

pub fn f495(v0: Adt8) -> bool {
    if f563(f275(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f563(f275(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f563(f275(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f496(v0: Adt8) -> bool {
    let m1435 = f275(Str::lit("content"), v0);
    match m1435 {
        Adt118::C0 => {
            true
        }
        Adt118::C1(v1) => {
            let m1436 = f281(v1);
            match m1436 {
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

pub fn f497(v0: Adt8) -> bool {
    if f563(f275(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f563(f275(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f498(v0: Adt118) -> bool {
    let m1437 = v0;
    match m1437 {
        Adt118::C0 => {
            false
        }
        Adt118::C1(v1) => {
            let m1438 = f181(v1);
            match m1438 {
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

pub fn f499(v0: Adt8) -> bool {
    let m1439 = f275(Str::lit("content"), v0);
    match m1439 {
        Adt118::C0 => {
            false
        }
        Adt118::C1(v1) => {
            let m1440 = f281(v1.clone());
            match m1440 {
                None => {
                    let m1441 = f335(v1);
                    match m1441 {
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

pub fn f500(p1445: List<Adt9>) -> bool {
    let m1444 = p1445;
    {
        let mut m1444 = m1444;
        loop {
            let step = {
                let v0 = m1444;
                let m1442 = v0;
                match m1442.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m1443 = f281(v1);
                        match m1443 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(v3) => {
                                if f498(f275(Str::lit("uri"), v3)) {
                                    Ok::<List<Adt9>, bool>(v2)
                                } else {
                                    Err::<List<Adt9>, bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(p1449: List<Str>, p1450: u64, p1451: List<Str>) -> List<Str> {
    let m1448 = (p1449, (p1450, p1451));
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1448;
                let m1446 = v0;
                match m1446.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m1447 = v1;
                        if m1447 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m1447 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: Str, v1: Str) -> R<bool> {
    Ok(f564(f555(v0)?, f555(v1)?))
}

pub fn f503(p1455: List<Adt49>, p1456: List<Str>) -> List<Str> {
    let m1454 = (p1455, p1456);
    {
        let mut m1454 = m1454;
        loop {
            let step = {
                let (v0, v1) = m1454;
                let m1452 = v0;
                match m1452.uncons() {
                    None => {
                        Err::<(List<Adt49>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt49>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt49::C0(_, _, _, h1453, _, _, _, _, _) => {
                                h1453
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: Adt118) -> bool {
    let m1457 = v0;
    match m1457 {
        Adt118::C0 => {
            false
        }
        Adt118::C1(_) => {
            true
        }
    }
}

pub fn f505(v0: Str, v1: Adt8) -> Option<Adt8> {
    let m1458 = f275(v0, v1);
    match m1458 {
        Adt118::C0 => {
            None::<Adt8>
        }
        Adt118::C1(v2) => {
            f281(v2)
        }
    }
}

pub fn f506(v0: Adt8) -> Option<Adt67> {
    let m1459 = f565(v0.clone());
    match m1459 {
        None => {
            None::<Adt67>
        }
        Some(v1) => {
            let m1460 = f566(Str::lit("status"), v0.clone());
            match m1460 {
                None => {
                    None::<Adt67>
                }
                Some(v2) => {
                    let a1461 = v2.clone();
                    let a1462 = Str::lit("working");
                    if equal(a1461, a1462) {
                        Some(Adt67::C0(v1))
                    } else {
                        let a1463 = v2.clone();
                        let a1464 = Str::lit("cancelled");
                        if equal(a1463, a1464) {
                            Some(Adt67::C4(v1))
                        } else {
                            let a1465 = v2.clone();
                            let a1466 = Str::lit("input_required");
                            if equal(a1465, a1466) {
                                let m1467 = f567(Str::lit("inputRequests"), v0);
                                match m1467 {
                                    None => {
                                        None::<Adt67>
                                    }
                                    Some(v3) => {
                                        Some(Adt67::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a1468 = v2.clone();
                                let a1469 = Str::lit("completed");
                                if equal(a1468, a1469) {
                                    let m1470 = f505(Str::lit("result"), v0);
                                    match m1470 {
                                        None => {
                                            None::<Adt67>
                                        }
                                        Some(v4) => {
                                            Some(Adt67::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a1471 = v2;
                                    let a1472 = Str::lit("failed");
                                    if equal(a1471, a1472) {
                                        let m1473 = f505(Str::lit("error"), v0);
                                        match m1473 {
                                            None => {
                                                None::<Adt67>
                                            }
                                            Some(v5) => {
                                                let m1474 = f568(v5);
                                                match m1474 {
                                                    None => {
                                                        None::<Adt67>
                                                    }
                                                    Some(v6) => {
                                                        Some(Adt67::C3(v1, v6))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        None::<Adt67>
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f507(v0: Str, v1: Adt8) -> Option<i64> {
    let m1475 = f275(v0, v1);
    match m1475 {
        Adt118::C0 => {
            None::<i64>
        }
        Adt118::C1(v2) => {
            f277(v2)
        }
    }
}

pub fn f508(v0: Adt8) -> Option<Adt90> {
    let m1476 = f507(Str::lit("parkAfterMs"), v0.clone());
    match m1476 {
        None => {
            None::<Adt90>
        }
        Some(v1) => {
            let m1477 = f567(Str::lit("parked"), v0.clone());
            match m1477 {
                None => {
                    None::<Adt90>
                }
                Some(v2) => {
                    let m1478 = f507(Str::lit("finishAfterMs"), v0.clone());
                    match m1478 {
                        None => {
                            None::<Adt90>
                        }
                        Some(v3) => {
                            let m1479 = f505(Str::lit("outcome"), v0);
                            match m1479 {
                                None => {
                                    None::<Adt90>
                                }
                                Some(v4) => {
                                    let m1480 = f569(v4);
                                    match m1480 {
                                        None => {
                                            None::<Adt90>
                                        }
                                        Some(v5) => {
                                            Some(Adt90::C0(v1, v2, v3, v5))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f509(p1483: List<(Str, Adt69)>, p1484: List<(Str, Adt69)>) -> List<(Str, Adt69)> {
    let m1482 = (p1483, p1484);
    {
        let mut m1482 = m1482;
        loop {
            let step = {
                let (v0, v1) = m1482;
                let m1481 = v0;
                match m1481.uncons() {
                    None => {
                        Err::<(List<(Str, Adt69)>, List<(Str, Adt69)>), List<(Str, Adt69)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt69)>, List<(Str, Adt69)>), List<(Str, Adt69)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f510(p1488: List<(Str, Adt69)>, v1: Adt8, p1489: List<(Str, Adt69)>) -> List<(Str, Adt69)> {
    let m1487 = (p1488, p1489);
    {
        let mut m1487 = m1487;
        loop {
            let step = {
                let (v0, v2) = m1487;
                let m1485 = v0;
                match m1485.uncons() {
                    None => {
                        Err::<(List<(Str, Adt69)>, List<(Str, Adt69)>), List<(Str, Adt69)>>(v2)
                    }
                    Some((v3, v4)) => {
                        if f422({
                            let (h1486, _) = v3.clone();
                            h1486
                        }, v1.clone()) {
                            Ok::<(List<(Str, Adt69)>, List<(Str, Adt69)>), List<(Str, Adt69)>>((v4, v2))
                        } else {
                            Ok::<(List<(Str, Adt69)>, List<(Str, Adt69)>), List<(Str, Adt69)>>((v4, List::cons(v3, v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: Bytes, p1495: u64, p1496: u64, p1497: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt2>> {
    let m1494 = (p1495, (p1496, p1497));
    {
        let mut m1494 = m1494;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1494;
                let m1490 = v2;
                if m1490 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>(Ok(Ok::<(List<List<u64>>, u64), Adt2>((f153(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m1490 - 1;
                    let m1491 = f7(v0.clone(), v1)?;
                    match m1491 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>(Ok(Err::<(List<List<u64>>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>((match v5.clone() {
                                Adt96::C0(_, h1492) => {
                                    h1492
                                }
                            }, (v4, List::cons(f29(match v5 {
                                Adt96::C0(h1493, _) => {
                                    h1493
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(v0: Adt100, v1: List<List<u64>>) -> Adt100 {
    Adt100::C0(match v0.clone() {
        Adt100::C0(h1498, _, _, _, _, _, _, _) => {
            h1498
        }
    }, match v0.clone() {
        Adt100::C0(_, h1499, _, _, _, _, _, _) => {
            h1499
        }
    }, match v0.clone() {
        Adt100::C0(_, _, h1500, _, _, _, _, _) => {
            h1500
        }
    }, match v0.clone() {
        Adt100::C0(_, _, _, h1501, _, _, _, _) => {
            h1501
        }
    }, match v0.clone() {
        Adt100::C0(_, _, _, _, h1502, _, _, _) => {
            h1502
        }
    }, v1, match v0.clone() {
        Adt100::C0(_, _, _, _, _, _, h1503, _) => {
            h1503
        }
    }, match v0 {
        Adt100::C0(_, _, _, _, _, _, _, h1504) => {
            h1504
        }
    })
}

pub fn f513(v0: i64, v1: Str, v2: Str, v3: Adt96) -> Result<(Adt18, u64), Adt2> {
    Ok::<(Adt18, u64), Adt2>((Adt18::C0(v0, v1, v2, match v3.clone() {
        Adt96::C0(h1505, _) => {
            h1505
        }
    }), match v3 {
        Adt96::C0(_, h1506) => {
            h1506
        }
    }))
}

pub fn f514(p1511: u64, p1512: List<Adt131>, p1513: List<u64>) -> R<Result<List<u64>, Adt120>> {
    let m1510 = (p1511, (p1512, p1513));
    {
        let mut m1510 = m1510;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1510;
                let m1507 = v0;
                if m1507 == 0 {
                    Err::<(u64, (List<Adt131>, List<u64>)), R<Result<List<u64>, Adt120>>>(Ok(Err::<List<u64>, Adt120>(Adt120::C13)))
                } else {
                    let v3 = m1507 - 1;
                    let m1508 = v1;
                    match m1508.uncons() {
                        None => {
                            Err::<(u64, (List<Adt131>, List<u64>)), R<Result<List<u64>, Adt120>>>(Ok(Ok::<List<u64>, Adt120>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m1509 = f570(v4, v5, v2)?;
                            match m1509 {
                                Adt157::C0(v6) => {
                                    Err::<(u64, (List<Adt131>, List<u64>)), R<Result<List<u64>, Adt120>>>(Ok(Err::<List<u64>, Adt120>(v6)))
                                }
                                Adt157::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt131>, List<u64>)), R<Result<List<u64>, Adt120>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1510 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(p1516: List<u64>, p1517: List<u64>) -> List<u64> {
    let m1515 = (p1516, p1517);
    {
        let mut m1515 = m1515;
        loop {
            let step = {
                let (v0, v1) = m1515;
                let m1514 = v0;
                match m1514.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: Adt111) -> Adt9 {
    let m1518 = v0;
    match m1518 {
        Adt111::C0(v1) => {
            Adt9::C3(v1)
        }
        Adt111::C1(v2) => {
            Adt9::C2(Adt10::C0(v2, 0i64))
        }
    }
}

pub fn f517(v0: Option<Adt9>) -> List<(Str, Adt9)> {
    let m1519 = v0;
    match m1519 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f518() -> i64 {
    -32700i64
}

pub fn f519() -> i64 {
    -32600i64
}

pub fn f520(v0: Adt122) -> Str {
    let m1520 = v0;
    match m1520 {
        Adt122::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt122::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt122::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt122::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt122::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt122::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt122::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt122::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt122::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f521(v0: Adt18, v1: bool, v2: Adt146) -> R<Adt104> {
    let m1522 = match v2.clone() {
        Adt146::C0(_, h1521) => {
            h1521
        }
    };
    match m1522 {
        Ok(v3) => {
            let m1523 = f571(v3.clone())?;
            match m1523 {
                None => {
                    Ok(f57())
                }
                Some(v4) => {
                    f572(v0, f573(match v3.clone() {
                        Adt147::C0(h1524, _, _) => {
                            h1524
                        }
                    }), f574(v1, f247(f575(), f576(match v2 {
                        Adt146::C0(h1525, _) => {
                            h1525
                        }
                    }, v4))?, f577(f578(match v3.clone() {
                        Adt147::C0(_, _, h1526) => {
                            h1526
                        }
                    }, List::<List<u64>>::nil())?, f579(f573(match v3 {
                        Adt147::C0(h1527, _, _) => {
                            h1527
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f247(f580(v5.clone()), f249(Some(match v2 {
                Adt146::C0(h1528, _) => {
                    h1528
                }
            }), f581(v5)?))
        }
    }
}

pub fn f522(p1533: List<Adt101>) -> R<bool> {
    let m1532 = p1533;
    {
        let mut m1532 = m1532;
        loop {
            let step = {
                let v0 = m1532;
                let m1529 = v0;
                match m1529.uncons() {
                    None => {
                        Err::<List<Adt101>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f453(match v1.clone() {
                            Adt101::C0(h1530, _) => {
                                h1530
                            }
                        }, f582())? {
                            if f583(f584(44u64, match v1 {
                                Adt101::C0(_, h1531) => {
                                    h1531
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt101>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt101>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt101>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1532 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: Adt3, v1: Fn9, v2: u64, v3: Option<Adt132>, v4: List<Str>, v5: Adt127) -> R<Adt146> {
    Ok(Adt146::C0(match v5.clone() {
        Adt127::C0(h1534, _, _) => {
            h1534
        }
    }, f585(v0, v1, v2, f586(v3, v4, v5)?)?))
}

pub fn f524(v0: List<Adt101>, v1: List<Adt15>, v2: Option<Str>, v3: Str) -> R<Adt132> {
    let v4: Adt152 = f587(Adt134::C0, f257(v0.clone(), f588())?);
    let v5: Adt152 = f587(Adt134::C1, f257(v0.clone(), f589())?);
    let v6: Adt152 = if f590(v3) {
        f587(Adt134::C2, f591(f257(v0.clone(), f592())?)?)
    } else {
        f593()
    };
    Ok(Adt132::C0(match v4.clone() {
        Adt152::C0(h1535, _) => {
            h1535
        }
    }, match v5.clone() {
        Adt152::C0(h1536, _) => {
            h1536
        }
    }, match v6.clone() {
        Adt152::C0(h1537, _) => {
            h1537
        }
    }, f594(match v4 {
        Adt152::C0(_, h1538) => {
            h1538
        }
    }, f594(match v5 {
        Adt152::C0(_, h1539) => {
            h1539
        }
    }, match v6 {
        Adt152::C0(_, h1540) => {
            h1540
        }
    })), f595(v0, v1, v2)?))
}

pub fn f525(v0: Adt127) -> Option<Str> {
    let a1542 = match v0.clone() {
        Adt127::C0(_, h1541, _) => {
            h1541
        }
    };
    let a1543 = Str::lit("tools/call");
    if equal(a1542, a1543) {
        let m1545 = match v0 {
            Adt127::C0(_, _, h1544) => {
                h1544
            }
        };
        match m1545 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1546 = f275(Str::lit("name"), v1);
                match m1546 {
                    Adt118::C0 => {
                        None::<Str>
                    }
                    Adt118::C1(v2) => {
                        f181(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f526() -> List<Str> {
    List::cons(f596(), List::<Str>::nil())
}

pub fn f527(v0: Adt8) -> R<Adt153> {
    let m1547 = f275(Str::lit("id"), v0);
    match m1547 {
        Adt118::C0 => {
            Ok(Adt153::C0)
        }
        Adt118::C1(v1) => {
            let m1548 = v1;
            match m1548 {
                Adt9::C0 => {
                    Ok(Adt153::C2)
                }
                Adt9::C1(_) => {
                    Ok(Adt153::C2)
                }
                Adt9::C2(v3) => {
                    let m1549 = f597(v3)?;
                    match m1549 {
                        None => {
                            Ok(Adt153::C2)
                        }
                        Some(v4) => {
                            Ok(Adt153::C1(Adt111::C1(v4)))
                        }
                    }
                }
                Adt9::C3(v2) => {
                    Ok(Adt153::C1(Adt111::C0(v2)))
                }
                Adt9::C4(_) => {
                    Ok(Adt153::C2)
                }
                Adt9::C5(_) => {
                    Ok(Adt153::C2)
                }
            }
        }
    }
}

pub fn f528(v0: Adt8) -> Adt122 {
    if f529(v0) {
        Adt122::C8
    } else {
        Adt122::C3
    }
}

pub fn f529(v0: Adt8) -> bool {
    if f422(Str::lit("result"), v0.clone()) {
        true
    } else {
        f422(Str::lit("error"), v0)
    }
}

pub fn f530(v0: Adt111, v1: Adt8) -> Result<Adt127, Adt121> {
    let m1550 = f275(Str::lit("jsonrpc"), v1.clone());
    match m1550 {
        Adt118::C0 => {
            f449(Some(v0), Adt122::C2)
        }
        Adt118::C1(v2) => {
            let m1551 = f181(v2);
            match m1551 {
                None => {
                    f449(Some(v0), Adt122::C2)
                }
                Some(v3) => {
                    let a1552 = v3;
                    let a1553 = f440();
                    if equal(a1552, a1553) {
                        f598(v0, v1)
                    } else {
                        f449(Some(v0), Adt122::C2)
                    }
                }
            }
        }
    }
}

pub fn f531(v0: u64, v1: u64) -> Adt137 {
    let a1554 = v1;
    let a1555 = 0u64;
    if nat_eq(a1554, a1555) {
        Adt137::C0
    } else {
        let a1556 = v1;
        let a1557 = 2u64;
        if nat_eq(a1556, a1557) {
            Adt137::C2(v0)
        } else {
            Adt137::C1({
                let a1561 = v0;
                let a1562 = {
                    let a1558 = v1;
                    let a1559 = 3u64;
                    let a1560 = 0u64;
                    nat_quot(a1558, a1559, a1560)
                };
                nat_sub(a1561, a1562)
            })
        }
    }
}

pub fn f532(p1590: Adt138, p1591: u64, p1592: u64, p1593: List<u64>) -> R<u64> {
    let m1589 = (p1590, (p1591, (p1592, p1593)));
    {
        let mut m1589 = m1589;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1589;
                let m1563 = v3;
                match m1563.uncons() {
                    None => {
                        let m1564 = v0;
                        match m1564 {
                            Adt138::C0 => {
                                Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt138::C1 => {
                                Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt138::C2 => {
                                Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt138::C3 => {
                                Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1565 = v0;
                        match m1565 {
                            Adt138::C0 => {
                                let a1566 = v4;
                                let a1567 = 128u64;
                                if nat_lt(a1566, a1567) {
                                    Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1568 = v4;
                                    let a1569 = 194u64;
                                    if nat_lt(a1568, a1569) {
                                        let v6: u64 = f599({
                                            let a1570 = List::cons(v4, v5);
                                            length_list(a1570)
                                        })?;
                                        Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1571 = v4;
                                        let a1572 = 224u64;
                                        if nat_lt(a1571, a1572) {
                                            Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1573 = v4;
                                            let a1574 = 224u64;
                                            if nat_eq(a1573, a1574) {
                                                Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1575 = v4;
                                                let a1576 = 237u64;
                                                if nat_eq(a1575, a1576) {
                                                    Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1577 = v4;
                                                    let a1578 = 240u64;
                                                    if nat_lt(a1577, a1578) {
                                                        Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1579 = v4;
                                                        let a1580 = 240u64;
                                                        if nat_eq(a1579, a1580) {
                                                            Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1581 = v4;
                                                            let a1582 = 244u64;
                                                            if nat_eq(a1581, a1582) {
                                                                Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1583 = v4;
                                                                let a1584 = 244u64;
                                                                if nat_lt(a1583, a1584) {
                                                                    Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f599({
                                                                        let a1585 = List::cons(v4, v5);
                                                                        length_list(a1585)
                                                                    })?;
                                                                    Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt138::C1 => {
                                if f600(v4, v1, v2) {
                                    Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f599({
                                        let a1586 = List::cons(v4, v5);
                                        length_list(a1586)
                                    })?;
                                    Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt138::C2 => {
                                if f600(v4, v1, v2) {
                                    Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f599({
                                        let a1587 = List::cons(v4, v5);
                                        length_list(a1587)
                                    })?;
                                    Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt138::C3 => {
                                if f600(v4, v1, v2) {
                                    Ok::<(Adt138, (u64, (u64, List<u64>))), R<u64>>((Adt138::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f599({
                                        let a1588 = List::cons(v4, v5);
                                        length_list(a1588)
                                    })?;
                                    Err::<(Adt138, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
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

pub fn f533(v0: Adt139, v1: u64) -> R<Result<Adt9, Adt120>> {
    let m1595 = match v0.clone() {
        Adt139::C0(h1594, _, _) => {
            h1594
        }
    };
    match m1595 {
        Adt140::C0 => {
            Ok(Err::<Adt9, Adt120>(Adt120::C3))
        }
        Adt140::C1 => {
            Ok(Err::<Adt9, Adt120>(Adt120::C3))
        }
        Adt140::C2 => {
            Ok(Err::<Adt9, Adt120>(Adt120::C3))
        }
        Adt140::C3 => {
            Ok(Err::<Adt9, Adt120>(Adt120::C3))
        }
        Adt140::C4 => {
            Ok(Err::<Adt9, Adt120>(Adt120::C3))
        }
        Adt140::C5 => {
            Ok(Err::<Adt9, Adt120>(Adt120::C3))
        }
        Adt140::C6(_, _, _) => {
            Ok(Err::<Adt9, Adt120>(Adt120::C3))
        }
        Adt140::C7(_, _) => {
            Ok(Err::<Adt9, Adt120>(Adt120::C3))
        }
        Adt140::C8(v3) => {
            let m1596 = f601(v3)?;
            match m1596 {
                Adt160::C0 => {
                    Ok(Err::<Adt9, Adt120>(Adt120::C7(v1)))
                }
                Adt160::C1 => {
                    Ok(Err::<Adt9, Adt120>(Adt120::C8(v1)))
                }
                Adt160::C2(v4) => {
                    let m1601 = {
                        let m1600 = f602(v4, match v0.clone() {
                            Adt139::C0(_, h1598, _) => {
                                h1598
                            }
                        }, match v0 {
                            Adt139::C0(_, _, h1599) => {
                                h1599
                            }
                        });
                        match m1600 {
                            Adt139::C0(h1597, _, _) => {
                                h1597
                            }
                        }
                    };
                    match m1601 {
                        Adt140::C0 => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C1 => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C2 => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C3 => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C4 => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C5 => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C6(_, _, _) => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C7(_, _) => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C8(_) => {
                            Ok(Err::<Adt9, Adt120>(Adt120::C3))
                        }
                        Adt140::C9(v5) => {
                            Ok(Ok::<Adt9, Adt120>(v5))
                        }
                    }
                }
            }
        }
        Adt140::C9(v2) => {
            Ok(Ok::<Adt9, Adt120>(v2))
        }
    }
}

pub fn f534(v0: Adt139, v1: u64, v2: u64) -> R<Adt154> {
    let m1603 = match v0.clone() {
        Adt139::C0(h1602, _, _) => {
            h1602
        }
    };
    match m1603 {
        Adt140::C0 => {
            if f603(v2) {
                Ok(Adt154::C1(v0))
            } else {
                f604(v0, v1, v2)
            }
        }
        Adt140::C1 => {
            if f603(v2) {
                Ok(Adt154::C1(v0))
            } else {
                let a1604 = v2;
                let a1605 = 93u64;
                if nat_eq(a1604, a1605) {
                    Ok(f605(v0, v1))
                } else {
                    f604(v0, v1, v2)
                }
            }
        }
        Adt140::C2 => {
            Ok(f606(v0, v1, v2))
        }
        Adt140::C3 => {
            if f603(v2) {
                Ok(Adt154::C1(v0))
            } else {
                let a1606 = v2;
                let a1607 = 125u64;
                if nat_eq(a1606, a1607) {
                    Ok(f607(v0, v1))
                } else {
                    let a1608 = v2;
                    let a1609 = 34u64;
                    if nat_eq(a1608, a1609) {
                        Ok(f608(v0, Adt140::C6(Adt141::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f609(Adt120::C2(v1)))
                    }
                }
            }
        }
        Adt140::C4 => {
            if f603(v2) {
                Ok(Adt154::C1(v0))
            } else {
                let a1610 = v2;
                let a1611 = 34u64;
                if nat_eq(a1610, a1611) {
                    Ok(f608(v0, Adt140::C6(Adt141::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f609(Adt120::C2(v1)))
                }
            }
        }
        Adt140::C5 => {
            if f603(v2) {
                Ok(Adt154::C1(v0))
            } else {
                let a1612 = v2;
                let a1613 = 58u64;
                if nat_eq(a1612, a1613) {
                    Ok(f608(v0, Adt140::C0))
                } else {
                    Ok(f609(Adt120::C2(v1)))
                }
            }
        }
        Adt140::C6(v3, v4, v5) => {
            f610(v0, v1, v2, v3, v4, v5)
        }
        Adt140::C7(v6, v7) => {
            f611(v0, v1, v2, v6, v7)
        }
        Adt140::C8(v8) => {
            f612(v0, v1, v2, v8)
        }
        Adt140::C9(_) => {
            Ok(f606(v0, v1, v2))
        }
    }
}

pub fn f535(v0: u64) -> R<u64> {
    let a1614 = 64u64;
    let a1615 = v0;
    if if nat_lt(a1614, a1615) {
        let a1616 = v0;
        let a1617 = 91u64;
        nat_lt(a1616, a1617)
    } else {
        false
    } {
        let a1618 = v0;
        let a1619 = 32u64;
        nat_add(a1618, a1619)
    } else {
        Ok(v0)
    }
}

pub fn f536(p1622: List<u64>) -> List<u64> {
    let m1621 = p1622;
    {
        let mut m1621 = m1621;
        loop {
            let step = {
                let v0 = m1621;
                let m1620 = v0;
                match m1620.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f613(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
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

pub fn f537(v0: u64) -> bool {
    let a1623 = v0;
    let a1624 = 9u64;
    if nat_eq(a1623, a1624) {
        true
    } else {
        let a1625 = 31u64;
        let a1626 = v0;
        if nat_lt(a1625, a1626) {
            let a1627 = v0;
            let a1628 = 127u64;
            nat_lt(a1627, a1628)
        } else {
            false
        }
    }
}

pub fn f538(p1631: List<u64>) -> bool {
    let m1630 = p1631;
    {
        let mut m1630 = m1630;
        loop {
            let step = {
                let v0 = m1630;
                let m1629 = v0;
                match m1629.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f614(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1630 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(v0: Adt3, v1: bool, v2: Adt18, v3: bool, v4: Adt146) -> R<Adt97> {
    let m1633 = f615(match v4.clone() {
        Adt146::C0(_, h1632) => {
            h1632
        }
    });
    match m1633 {
        None => {
            Ok(f617(v0, v1, f521(v2, v3, v4)?))
        }
        Some(v5) => {
            f616(v2, v3, v4, v5)
        }
    }
}

pub fn f540(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>) -> Option<Adt102> {
    let m1634 = f618(f619(), v2.clone());
    match m1634 {
        None => {
            let m1635 = f620(f621(), v2.clone());
            match m1635 {
                None => {
                    f622(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt102::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt102::C4(v0, v3))
        }
    }
}

pub fn f541(v0: Str, v1: List<(Str, Adt9)>, v2: bool, v3: List<Str>, v4: Adt125) -> R<Adt155> {
    let m1636 = f383(Str::lit("x-mcp-header"), v1.clone());
    match m1636 {
        Adt118::C0 => {
            Ok(Adt155::C1(v4))
        }
        Adt118::C1(v5) => {
            f623(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f542(v0: List<(Str, Adt9)>, v1: bool, v2: List<Str>, v3: List<Adt112>) -> List<Adt112> {
    f624(Str::lit("properties"), v0.clone(), v1, v2, f624(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f625(Str::lit("items"), v0.clone(), f625(Str::lit("additionalProperties"), v0.clone(), f626(Str::lit("allOf"), v0.clone(), f626(Str::lit("anyOf"), v0.clone(), f626(Str::lit("oneOf"), v0.clone(), f625(Str::lit("not"), v0.clone(), f625(Str::lit("if"), v0.clone(), f625(Str::lit("then"), v0.clone(), f625(Str::lit("else"), v0, v3)))))))))))
}

pub fn f543(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt128> {
    Ok(f627(List::cons((Str::lit("type"), f628(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f629(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f630(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f631(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f631(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f631(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f631(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1637 = f632(v0.clone(), v1.clone())?;
        let a1638 = List::cons((Str::lit("required"), f633(v0, v1)), List::<(Str, Adt156)>::nil());
        append_list(a1637, a1638)
    })))))))))
}

pub fn f544(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt114>) -> R<Adt128> {
    Ok(f634(f635(v0, v1.clone(), v2.clone(), v3.clone()), f634(f636(v1.clone(), v2.clone(), v3.clone()), f634(f637(v1.clone(), v2.clone(), v3.clone()), f634(f638(v1.clone(), v2.clone(), v3.clone())?, f634(f639(v1.clone(), v2.clone(), v3.clone()), f634(f640(Adt117::C0, f473(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f634(f640(Adt117::C1, f474(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f634(f640(Adt117::C2, f475(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f640(Adt117::C3, f476(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f545(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt9> {
    let m1639 = f383(v0, v1);
    match m1639 {
        Adt118::C0 => {
            None::<Adt9>
        }
        Adt118::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f546(v0: Str) -> R<Str> {
    let m1642 = {
        let a1641 = f493(f243(), f515(f641(f29({
            let a1640 = v0.clone();
            utf8_encode(a1640)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1641)
    };
    match m1642 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f547(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1644 = f642({
        let a1643 = v0.clone();
        length_list(a1643)
    }, v0, List::<u64>::nil())?;
    match m1644 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f515(v1, List::<u64>::nil())))
        }
    }
}

pub fn f548(v0: List<Adt55>) -> R<bool> {
    let m1645 = v0;
    match m1645.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f643(v2, match v1 {
                Adt55::C0(h1646, _, _) => {
                    h1646
                }
            })
        }
    }
}

pub fn f549() -> i64 {
    60000i64
}

pub fn f550() -> i64 {
    1000i64
}

pub fn f551(v0: Str) -> R<Str> {
    let m1649 = {
        let a1648 = f493(f243(), f515(f644(f29({
            let a1647 = v0;
            utf8_encode(a1647)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1648)
    };
    match m1649 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f552(p1652: List<(Str, Adt9)>, p1653: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m1651 = (p1652, p1653);
    {
        let mut m1651 = m1651;
        loop {
            let step = {
                let (v0, v1) = m1651;
                let m1650 = v0;
                match m1650.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Adt9)>), List<(Str, Adt9)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<(Str, Adt9)>), List<(Str, Adt9)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1651 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(v0: Adt43) -> R<Option<Adt9>> {
    let m1654 = v0;
    match m1654 {
        Adt43::C0(v1) => {
            Ok(Some(f645(v1)))
        }
        Adt43::C1(v2) => {
            Ok(Some(f646(Str::lit("image"), match v2.clone() {
                Adt47::C0(h1655, _, _, _) => {
                    h1655
                }
            }, match v2.clone() {
                Adt47::C0(_, h1656, _, _) => {
                    h1656
                }
            }, match v2.clone() {
                Adt47::C0(_, _, h1657, _) => {
                    h1657
                }
            }, match v2 {
                Adt47::C0(_, _, _, h1658) => {
                    h1658
                }
            })))
        }
        Adt43::C2(v3) => {
            Ok(Some(f646(Str::lit("audio"), match v3.clone() {
                Adt48::C0(h1659, _, _, _) => {
                    h1659
                }
            }, match v3.clone() {
                Adt48::C0(_, h1660, _, _) => {
                    h1660
                }
            }, match v3.clone() {
                Adt48::C0(_, _, h1661, _) => {
                    h1661
                }
            }, match v3 {
                Adt48::C0(_, _, _, h1662) => {
                    h1662
                }
            })))
        }
        Adt43::C3(v4) => {
            Ok(Some(f647(v4)?))
        }
        Adt43::C4(v5) => {
            Ok(Some(f648(v5)))
        }
    }
}

pub fn f554(p1668: List<u64>, p1669: List<u64>) -> Option<List<u64>> {
    let m1667 = (p1668, p1669);
    {
        let mut m1667 = m1667;
        loop {
            let step = {
                let (v0, v1) = m1667;
                let m1663 = v0;
                match m1663.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1664 = v1;
                        match m1664.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1665 = v2;
                                let a1666 = v4;
                                if nat_eq(a1665, a1666) {
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
                Ok(next) => { m1667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(v0: Str) -> R<List<u64>> {
    f29({
        let a1670 = v0;
        utf8_encode(a1670)
    })
}

pub fn f556() -> Str {
    Str::lit("test://template/")
}

pub fn f557(v0: List<u64>) -> List<u64> {
    f515(v0, List::<u64>::nil())
}

pub fn f558() -> Str {
    Str::lit("data")
}

pub fn f559(v0: List<u64>) -> bool {
    let m1671 = v0;
    match m1671.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f560(p1676: List<u64>) -> bool {
    let m1675 = p1676;
    {
        let mut m1675 = m1675;
        loop {
            let step = {
                let v0 = m1675;
                let m1672 = v0;
                match m1672.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1673 = v1;
                        let a1674 = 47u64;
                        if nat_eq(a1673, a1674) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
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

pub fn f561(p1681: Bytes, p1682: List<Bytes>) -> Bytes {
    let m1680 = (p1681, p1682);
    {
        let mut m1680 = m1680;
        loop {
            let step = {
                let (v0, v1) = m1680;
                let m1677 = v1;
                match m1677.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1678 = v2;
                            let a1679 = v0;
                            append_bytes(a1678, a1679)
                        }, v3))
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

pub fn f562(p1685: List<Bytes>, p1686: List<u64>) -> List<Bytes> {
    let m1684 = (p1685, p1686);
    {
        let mut m1684 = m1684;
        loop {
            let step = {
                let (v0, v1) = m1684;
                let m1683 = v1;
                match m1683.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f649(f650(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563(v0: Adt118, v1: Str) -> bool {
    let m1687 = v0;
    match m1687 {
        Adt118::C0 => {
            false
        }
        Adt118::C1(v2) => {
            let m1688 = f181(v2);
            match m1688 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1689 = v3;
                    let a1690 = v1;
                    equal(a1689, a1690)
                }
            }
        }
    }
}

pub fn f564(p1696: List<u64>, p1697: List<u64>) -> bool {
    let m1695 = (p1696, p1697);
    {
        let mut m1695 = m1695;
        loop {
            let step = {
                let (v0, v1) = m1695;
                let m1691 = v0;
                match m1691.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1692 = v1;
                        match m1692.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1693 = v2;
                                let a1694 = v4;
                                if nat_eq(a1693, a1694) {
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
                Ok(next) => { m1695 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f565(v0: Adt8) -> Option<Adt68> {
    let m1698 = f566(Str::lit("taskId"), v0.clone());
    match m1698 {
        None => {
            None::<Adt68>
        }
        Some(v1) => {
            let m1699 = f566(Str::lit("createdAt"), v0.clone());
            match m1699 {
                None => {
                    None::<Adt68>
                }
                Some(v2) => {
                    let m1700 = f566(Str::lit("lastUpdatedAt"), v0.clone());
                    match m1700 {
                        None => {
                            None::<Adt68>
                        }
                        Some(v3) => {
                            Some(Adt68::C0(v1, f566(Str::lit("statusMessage"), v0.clone()), v2, v3, f507(Str::lit("ttlMs"), v0.clone()), f507(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f566(v0: Str, v1: Adt8) -> Option<Str> {
    let m1701 = f275(v0, v1);
    match m1701 {
        Adt118::C0 => {
            None::<Str>
        }
        Adt118::C1(v2) => {
            f181(v2)
        }
    }
}

pub fn f567(v0: Str, v1: Adt8) -> Option<List<(Str, Adt69)>> {
    let m1702 = f505(v0, v1);
    match m1702 {
        None => {
            None::<List<(Str, Adt69)>>
        }
        Some(v2) => {
            f651(match v2 {
                Adt8::C0(h1703) => {
                    h1703
                }
            }, List::<(Str, Adt69)>::nil())
        }
    }
}

pub fn f568(v0: Adt8) -> Option<Adt85> {
    let m1704 = f507(Str::lit("code"), v0.clone());
    match m1704 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m1705 = f566(Str::lit("message"), v0.clone());
            match m1705 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    Some(Adt85::C0(v1, v2, {
                        let m1706 = f275(Str::lit("data"), v0);
                        match m1706 {
                            Adt118::C0 => {
                                None::<Adt9>
                            }
                            Adt118::C1(v3) => {
                                Some(v3)
                            }
                        }
                    }))
                }
            }
        }
    }
}

pub fn f569(v0: Adt8) -> Option<Adt91> {
    let m1707 = f505(Str::lit("completes"), v0.clone());
    match m1707 {
        None => {
            let m1708 = f505(Str::lit("fails"), v0);
            match m1708 {
                None => {
                    None::<Adt91>
                }
                Some(v2) => {
                    let m1709 = f568(v2);
                    match m1709 {
                        None => {
                            None::<Adt91>
                        }
                        Some(v3) => {
                            Some(Adt91::C1(v3))
                        }
                    }
                }
            }
        }
        Some(v1) => {
            Some(Adt91::C0(v1))
        }
    }
}

pub fn f570(v0: Adt131, v1: List<Adt131>, v2: List<u64>) -> R<Adt157> {
    let m1710 = v0;
    match m1710 {
        Adt131::C0(v3) => {
            Ok(Adt157::C1(v1, f515(v3, v2)))
        }
        Adt131::C1(v4, v5) => {
            f652(v4, v5, v1, v2)
        }
        Adt131::C2(v6, v7) => {
            let m1711 = v7;
            match m1711.uncons() {
                None => {
                    Ok(Adt157::C1(List::cons(Adt131::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt157::C1(List::cons(Adt131::C1(v6, v8), List::cons(Adt131::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt131::C3(v10, v11) => {
            let m1712 = v11.clone();
            match m1712.uncons() {
                None => {
                    Ok(Adt157::C1(List::cons(Adt131::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt157::C1(List::cons(Adt131::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt131::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt131::C4(v14, v15) => {
            let m1713 = v15;
            match m1713.uncons() {
                None => {
                    Ok(Adt157::C1(List::cons(Adt131::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt157::C1(List::cons(Adt131::C0({
                        let a1715 = f653({
                            let (h1714, _) = v16;
                            h1714
                        })?;
                        let a1716 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1715, a1716)
                    }), List::cons(Adt131::C1(v14, {
                        let (_, h1717) = v16;
                        h1717
                    }), List::cons(Adt131::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt131::C5(v18, v19) => {
            let m1718 = v19.clone();
            match m1718.uncons() {
                None => {
                    Ok(Adt157::C1(List::cons(Adt131::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt157::C1(List::cons(Adt131::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt131::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f571(v0: Adt147) -> R<Option<Adt8>> {
    let m1720 = f654(match v0.clone() {
        Adt147::C0(h1719, _, _) => {
            h1719
        }
    })?;
    match m1720 {
        None => {
            Ok(None::<Adt8>)
        }
        Some(v1) => {
            Ok(Some(Adt8::C0({
                let a1723 = v1;
                let a1724 = {
                    let m1722 = f655(v0);
                    match m1722 {
                        Adt8::C0(h1721) => {
                            h1721
                        }
                    }
                };
                append_list(a1723, a1724)
            })))
        }
    }
}

pub fn f572(v0: Adt18, v1: Option<Adt92>, v2: Adt104) -> R<Adt104> {
    let m1725 = v1;
    match m1725 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a1727 = match v2.clone() {
                Adt104::C0(h1726, _, _, _, _) => {
                    h1726
                }
            };
            let a1728 = f575();
            if nat_eq(a1727, a1728) {
                Ok(f656(f657(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f573(v0: Adt37) -> Option<Adt92> {
    let m1729 = v0;
    match m1729 {
        Adt37::C0(_) => {
            None::<Adt92>
        }
        Adt37::C1(_) => {
            None::<Adt92>
        }
        Adt37::C2(_) => {
            None::<Adt92>
        }
        Adt37::C3(_) => {
            None::<Adt92>
        }
        Adt37::C4(_) => {
            None::<Adt92>
        }
        Adt37::C5(_) => {
            None::<Adt92>
        }
        Adt37::C6(_) => {
            None::<Adt92>
        }
        Adt37::C7(_) => {
            None::<Adt92>
        }
        Adt37::C8(_) => {
            None::<Adt92>
        }
        Adt37::C9(_) => {
            None::<Adt92>
        }
        Adt37::C10(_) => {
            None::<Adt92>
        }
        Adt37::C11 => {
            None::<Adt92>
        }
        Adt37::C12(_) => {
            None::<Adt92>
        }
        Adt37::C13(v13) => {
            Some(Adt92::C0(match v13 {
                Adt87::C0(_, _, h1730) => {
                    h1730
                }
            }, Adt93::C0, false))
        }
        Adt37::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f574(v0: bool, v1: Adt104, v2: Option<List<List<u64>>>) -> Adt104 {
    let m1731 = v2;
    match m1731 {
        None => {
            f57()
        }
        Some(v3) => {
            f658(v0, v1, f153(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f575() -> u64 {
    200u64
}

pub fn f576(v0: Adt111, v1: Adt8) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(f440())), List::cons((Str::lit("id"), f516(v0)), List::cons((Str::lit("result"), Adt9::C5(match v1 {
        Adt8::C0(h1732) => {
            h1732
        }
    })), List::<(Str, Adt9)>::nil()))))
}

pub fn f577(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m1733 = v0;
    match m1733 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m1734 = v1;
            match m1734 {
                None => {
                    None::<List<List<u64>>>
                }
                Some(v3) => {
                    Some(f153(v3, v2))
                }
            }
        }
    }
}

pub fn f578(p1738: List<Adt149>, p1739: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1737 = (p1738, p1739);
    {
        let mut m1737 = m1737;
        loop {
            let step = {
                let (v0, v1) = m1737;
                let m1735 = v0;
                match m1735.uncons() {
                    None => {
                        Err::<(List<Adt149>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1736 = f659(v2)?;
                        match m1736 {
                            None => {
                                Err::<(List<Adt149>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt149>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: Option<Adt92>) -> R<Option<List<List<u64>>>> {
    let m1740 = v0;
    match m1740 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f660(v1)
        }
    }
}

pub fn f580(v0: Adt150) -> u64 {
    let m1741 = v0;
    match m1741 {
        Adt150::C0(_) => {
            f354()
        }
        Adt150::C1(_, _) => {
            f354()
        }
        Adt150::C2(_) => {
            f354()
        }
        Adt150::C3(_) => {
            f661()
        }
        Adt150::C4(v6) => {
            f662(v6)
        }
    }
}

pub fn f581(v0: Adt150) -> R<Adt85> {
    Ok(Adt85::C0(f663(v0.clone()), f664(v0.clone()), f665(v0)?))
}

pub fn f582() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f583(p1744: List<List<u64>>) -> R<bool> {
    let m1743 = p1744;
    {
        let mut m1743 = m1743;
        loop {
            let step = {
                let v0 = m1743;
                let m1742 = v0;
                match m1742.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f666(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: u64, p1749: List<u64>, p1750: List<u64>, p1751: List<List<u64>>) -> List<List<u64>> {
    let m1748 = (p1749, (p1750, p1751));
    {
        let mut m1748 = m1748;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1748;
                let m1745 = v1;
                match m1745.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f153(List::cons(f463(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1746 = v4;
                        let a1747 = v0;
                        if nat_eq(a1746, a1747) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f463(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
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

pub fn f585(v0: Adt3, v1: Fn9, v2: u64, v3: Result<Adt158, Adt150>) -> R<Result<Adt147, Adt150>> {
    let m1752 = v3;
    match m1752 {
        Ok(v5) => {
            f667(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt147, Adt150>(v4))
        }
    }
}

pub fn f586(v0: Option<Adt132>, v1: List<Str>, v2: Adt127) -> R<Result<Adt158, Adt150>> {
    Ok(f668(f669(match v2.clone() {
        Adt127::C0(_, h1753, _) => {
            h1753
        }
    }), f670(v0, v1, v2)?))
}

pub fn f587(v0: Adt134, v1: Adt123) -> Adt152 {
    let m1754 = v1;
    match m1754 {
        Adt123::C0 => {
            f593()
        }
        Adt123::C1 => {
            Adt152::C0(None::<Str>, Some(Adt133::C6(v0)))
        }
        Adt123::C2 => {
            Adt152::C0(None::<Str>, Some(Adt133::C7(v0)))
        }
        Adt123::C3(v2) => {
            let m1755 = f436(v2);
            match m1755 {
                None => {
                    Adt152::C0(None::<Str>, Some(Adt133::C6(v0)))
                }
                Some(v3) => {
                    Adt152::C0(Some(v3), None::<Adt133>)
                }
            }
        }
    }
}

pub fn f588() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f589() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f590(v0: Str) -> bool {
    let m1756 = f671(v0);
    match m1756 {
        Adt164::C0 => {
            false
        }
        Adt164::C1 => {
            true
        }
        Adt164::C2 => {
            true
        }
        Adt164::C3 => {
            true
        }
    }
}

pub fn f591(v0: Adt123) -> R<Adt123> {
    let m1757 = v0.clone();
    match m1757 {
        Adt123::C0 => {
            Ok(v0)
        }
        Adt123::C1 => {
            Ok(v0)
        }
        Adt123::C2 => {
            Ok(v0)
        }
        Adt123::C3(v1) => {
            let m1758 = f672(v1);
            match m1758 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1759 = f547(v2)?;
                    match m1759 {
                        None => {
                            Ok(Adt123::C1)
                        }
                        Some(v3) => {
                            Ok(Adt123::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f592() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f593() -> Adt152 {
    Adt152::C0(None::<Str>, None::<Adt133>)
}

pub fn f594(v0: Option<Adt133>, v1: Option<Adt133>) -> Option<Adt133> {
    let m1760 = v0.clone();
    match m1760 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f595(v0: List<Adt101>, v1: List<Adt15>, v2: Option<Str>) -> R<List<Adt135>> {
    Ok(f673(f674(v0, v1, v2, List::<Adt135>::nil())?, List::<Adt135>::nil()))
}

pub fn f596() -> Str {
    Str::lit("2026-07-28")
}

pub fn f597(v0: Adt10) -> R<Option<i64>> {
    let m1763 = f675(40u64, match v0.clone() {
        Adt10::C0(h1761, _) => {
            h1761
        }
    }, match v0 {
        Adt10::C0(_, h1762) => {
            h1762
        }
    })?;
    match m1763 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f676(v1))
        }
    }
}

pub fn f598(v0: Adt111, v1: Adt8) -> Result<Adt127, Adt121> {
    let m1764 = f275(Str::lit("method"), v1.clone());
    match m1764 {
        Adt118::C0 => {
            f449(Some(v0), f528(v1))
        }
        Adt118::C1(v2) => {
            let m1765 = f181(v2);
            match m1765 {
                None => {
                    f449(Some(v0), Adt122::C4)
                }
                Some(v3) => {
                    f677(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f599(v0: u64) -> R<u64> {
    let a1768 = {
        let a1766 = v0;
        let a1767 = 3u64;
        nat_mul(a1766, a1767)?
    };
    let a1769 = 1u64;
    nat_add(a1768, a1769)
}

pub fn f600(v0: u64, v1: u64, v2: u64) -> bool {
    let a1770 = v1;
    let a1771 = v0;
    if nat_le(a1770, a1771) {
        let a1772 = v0;
        let a1773 = v2;
        nat_le(a1772, a1773)
    } else {
        false
    }
}

pub fn f601(v0: Adt142) -> R<Adt160> {
    if f678(match v0.clone() {
        Adt142::C0(_, h1774, _, _, _, _, _) => {
            h1774
        }
    }) {
        let v1: i64 = {
            let a1780 = if match v0.clone() {
                Adt142::C0(_, _, _, _, _, h1775, _) => {
                    h1775
                }
            } {
                let a1777 = match v0.clone() {
                    Adt142::C0(_, _, _, _, _, _, h1776) => {
                        h1776
                    }
                };
                int_neg(a1777)?
            } else {
                match v0.clone() {
                    Adt142::C0(_, _, _, _, _, _, h1778) => {
                        h1778
                    }
                }
            };
            let a1781 = match v0.clone() {
                Adt142::C0(_, _, _, _, h1779, _, _) => {
                    h1779
                }
            };
            int_sub(a1780, a1781)?
        };
        let m1784 = {
            let a1782 = v1;
            let a1783 = -9999i64;
            compare(a1782, a1783)
        };
        if if match m1784 {
            Ordering::Less => {
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
            let m1787 = {
                let a1785 = 9999i64;
                let a1786 = v1;
                compare(a1785, a1786)
            };
            match m1787 {
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
            Ok(Adt160::C1)
        } else {
            Ok(Adt160::C2(Adt9::C2(Adt10::C0(if match v0.clone() {
                Adt142::C0(h1788, _, _, _, _, _, _) => {
                    h1788
                }
            } {
                let a1790 = match v0 {
                    Adt142::C0(_, _, h1789, _, _, _, _) => {
                        h1789
                    }
                };
                int_neg(a1790)?
            } else {
                match v0 {
                    Adt142::C0(_, _, h1791, _, _, _, _) => {
                        h1791
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt160::C0)
    }
}

pub fn f602(v0: Adt9, v1: List<Adt144>, v2: u64) -> Adt139 {
    let m1792 = v1.clone();
    match m1792.uncons() {
        None => {
            Adt139::C0(Adt140::C9(v0), List::<Adt144>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1793 = v3;
            match m1793 {
                Adt144::C0(v5) => {
                    Adt139::C0(Adt140::C2, List::cons(Adt144::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt144::C1(v6, v7) => {
                    let m1794 = v7;
                    match m1794 {
                        None => {
                            Adt139::C0(Adt140::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt139::C0(Adt140::C2, List::cons(Adt144::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f603(v0: u64) -> bool {
    let a1795 = v0;
    let a1796 = 32u64;
    if nat_eq(a1795, a1796) {
        true
    } else {
        let a1797 = v0;
        let a1798 = 10u64;
        if nat_eq(a1797, a1798) {
            true
        } else {
            let a1799 = v0;
            let a1800 = 13u64;
            if nat_eq(a1799, a1800) {
                true
            } else {
                let a1801 = v0;
                let a1802 = 9u64;
                nat_eq(a1801, a1802)
            }
        }
    }
}

pub fn f604(v0: Adt139, v1: u64, v2: u64) -> R<Adt154> {
    let a1803 = v2;
    let a1804 = 34u64;
    if nat_eq(a1803, a1804) {
        Ok(f608(v0, Adt140::C6(Adt141::C0, List::<u64>::nil(), false)))
    } else {
        let a1805 = v2;
        let a1806 = 123u64;
        if nat_eq(a1805, a1806) {
            f679(v0, Adt144::C1(List::<(Str, Adt9)>::nil(), None::<Str>), Adt140::C3, v1)
        } else {
            let a1807 = v2;
            let a1808 = 91u64;
            if nat_eq(a1807, a1808) {
                f679(v0, Adt144::C0(List::<Adt9>::nil()), Adt140::C1, v1)
            } else {
                let a1809 = v2;
                let a1810 = 116u64;
                if nat_eq(a1809, a1810) {
                    Ok(f608(v0, Adt140::C7(0u64, 1u64)))
                } else {
                    let a1811 = v2;
                    let a1812 = 102u64;
                    if nat_eq(a1811, a1812) {
                        Ok(f608(v0, Adt140::C7(1u64, 1u64)))
                    } else {
                        let a1813 = v2;
                        let a1814 = 110u64;
                        if nat_eq(a1813, a1814) {
                            Ok(f608(v0, Adt140::C7(2u64, 1u64)))
                        } else {
                            let a1815 = v2;
                            let a1816 = 45u64;
                            if nat_eq(a1815, a1816) {
                                Ok(f608(v0, f680(true, Adt143::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1817 = v2;
                                let a1818 = 48u64;
                                if nat_eq(a1817, a1818) {
                                    Ok(f608(v0, f680(false, Adt143::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f600(v2, 49u64, 57u64) {
                                        Ok(f608(v0, f680(false, Adt143::C2, f681({
                                            let a1819 = v2;
                                            let a1820 = 48u64;
                                            nat_sub(a1819, a1820)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f609(Adt120::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f605(v0: Adt139, v1: u64) -> Adt154 {
    let m1822 = match v0.clone() {
        Adt139::C0(_, h1821, _) => {
            h1821
        }
    };
    match m1822.uncons() {
        None => {
            f609(Adt120::C2(v1))
        }
        Some((v2, v3)) => {
            let m1823 = v2;
            match m1823 {
                Adt144::C0(v4) => {
                    Adt154::C1(f602(Adt9::C4(f490(v4, List::<Adt9>::nil())), v3, {
                        let a1825 = match v0 {
                            Adt139::C0(_, _, h1824) => {
                                h1824
                            }
                        };
                        let a1826 = 1u64;
                        nat_sub(a1825, a1826)
                    }))
                }
                Adt144::C1(_, _) => {
                    f609(Adt120::C2(v1))
                }
            }
        }
    }
}

pub fn f606(v0: Adt139, v1: u64, v2: u64) -> Adt154 {
    let m1828 = match v0.clone() {
        Adt139::C0(h1827, _, _) => {
            h1827
        }
    };
    match m1828 {
        Adt140::C0 => {
            f609(Adt120::C2(v1))
        }
        Adt140::C1 => {
            f609(Adt120::C2(v1))
        }
        Adt140::C2 => {
            if f603(v2) {
                Adt154::C1(v0)
            } else {
                let a1829 = v2;
                let a1830 = 44u64;
                if nat_eq(a1829, a1830) {
                    f682(v0, v1)
                } else {
                    let a1831 = v2;
                    let a1832 = 93u64;
                    if nat_eq(a1831, a1832) {
                        f605(v0, v1)
                    } else {
                        let a1833 = v2;
                        let a1834 = 125u64;
                        if nat_eq(a1833, a1834) {
                            f607(v0, v1)
                        } else {
                            f609(Adt120::C2(v1))
                        }
                    }
                }
            }
        }
        Adt140::C3 => {
            f609(Adt120::C2(v1))
        }
        Adt140::C4 => {
            f609(Adt120::C2(v1))
        }
        Adt140::C5 => {
            f609(Adt120::C2(v1))
        }
        Adt140::C6(_, _, _) => {
            f609(Adt120::C2(v1))
        }
        Adt140::C7(_, _) => {
            f609(Adt120::C2(v1))
        }
        Adt140::C8(_) => {
            f609(Adt120::C2(v1))
        }
        Adt140::C9(_) => {
            if f603(v2) {
                Adt154::C1(v0)
            } else {
                f609(Adt120::C10(v1))
            }
        }
    }
}

pub fn f607(v0: Adt139, v1: u64) -> Adt154 {
    let m1836 = match v0.clone() {
        Adt139::C0(_, h1835, _) => {
            h1835
        }
    };
    match m1836.uncons() {
        None => {
            f609(Adt120::C2(v1))
        }
        Some((v2, v3)) => {
            let m1837 = v2;
            match m1837 {
                Adt144::C0(_) => {
                    f609(Adt120::C2(v1))
                }
                Adt144::C1(v5, _) => {
                    Adt154::C1(f602(Adt9::C5(f552(v5, List::<(Str, Adt9)>::nil())), v3, {
                        let a1839 = match v0 {
                            Adt139::C0(_, _, h1838) => {
                                h1838
                            }
                        };
                        let a1840 = 1u64;
                        nat_sub(a1839, a1840)
                    }))
                }
            }
        }
    }
}

pub fn f608(v0: Adt139, v1: Adt140) -> Adt154 {
    Adt154::C1(f683(v0, v1))
}

pub fn f609(v0: Adt120) -> Adt154 {
    Adt154::C0(v0)
}

pub fn f610(v0: Adt139, v1: u64, v2: u64, v3: Adt141, v4: List<u64>, v5: bool) -> R<Adt154> {
    let m1841 = v3;
    match m1841 {
        Adt141::C0 => {
            let a1842 = v2;
            let a1843 = 34u64;
            if nat_eq(a1842, a1843) {
                Ok(f684(v0, v1, v4, v5))
            } else {
                let a1844 = v2;
                let a1845 = 92u64;
                if nat_eq(a1844, a1845) {
                    Ok(f608(v0, Adt140::C6(Adt141::C1, v4, v5)))
                } else {
                    let a1846 = v2;
                    let a1847 = 32u64;
                    if nat_lt(a1846, a1847) {
                        Ok(f609(Adt120::C4(v1)))
                    } else {
                        Ok(f608(v0, Adt140::C6(Adt141::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt141::C1 => {
            let a1848 = v2;
            let a1849 = 117u64;
            if nat_eq(a1848, a1849) {
                Ok(f608(v0, Adt140::C6(Adt141::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1850 = f685(v2);
                let a1851 = 0u64;
                if nat_eq(a1850, a1851) {
                    Ok(f609(Adt120::C5(v1)))
                } else {
                    Ok(f608(v0, Adt140::C6(Adt141::C0, List::cons(f685(v2), v4), v5)))
                }
            }
        }
        Adt141::C2(v6, v7, v8) => {
            let a1852 = 15u64;
            let a1853 = f686(v2);
            if nat_lt(a1852, a1853) {
                Ok(f609(Adt120::C5(v1)))
            } else {
                let a1856 = {
                    let a1854 = v6;
                    let a1855 = 1u64;
                    nat_add(a1854, a1855)?
                };
                let a1857 = 4u64;
                if nat_eq(a1856, a1857) {
                    f687(v0, v1, {
                        let a1860 = {
                            let a1858 = v7;
                            let a1859 = 16u64;
                            nat_mul(a1858, a1859)?
                        };
                        let a1861 = f686(v2);
                        nat_add(a1860, a1861)?
                    }, v8, v4, v5)
                } else {
                    Ok(f608(v0, Adt140::C6(Adt141::C2({
                        let a1862 = v6;
                        let a1863 = 1u64;
                        nat_add(a1862, a1863)?
                    }, {
                        let a1866 = {
                            let a1864 = v7;
                            let a1865 = 16u64;
                            nat_mul(a1864, a1865)?
                        };
                        let a1867 = f686(v2);
                        nat_add(a1866, a1867)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt141::C3(v9) => {
            let a1868 = v2;
            let a1869 = 92u64;
            if nat_eq(a1868, a1869) {
                Ok(f608(v0, Adt140::C6(Adt141::C4(v9), v4, v5)))
            } else {
                Ok(f609(Adt120::C6(v1)))
            }
        }
        Adt141::C4(v10) => {
            let a1870 = v2;
            let a1871 = 117u64;
            if nat_eq(a1870, a1871) {
                Ok(f608(v0, Adt140::C6(Adt141::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f609(Adt120::C6(v1)))
            }
        }
    }
}

pub fn f611(v0: Adt139, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt154> {
    let a1872 = v2;
    let a1873 = f688(v3, v4);
    if nat_eq(a1872, a1873) {
        let a1877 = {
            let a1874 = v4;
            let a1875 = 1u64;
            nat_add(a1874, a1875)?
        };
        let a1878 = {
            let a1876 = f689(v3);
            length_list(a1876)
        };
        if nat_eq(a1877, a1878) {
            Ok(f690(v0, f691(v3)))
        } else {
            Ok(f608(v0, Adt140::C7(v3, {
                let a1879 = v4;
                let a1880 = 1u64;
                nat_add(a1879, a1880)?
            })))
        }
    } else {
        Ok(f609(Adt120::C2(v1)))
    }
}

pub fn f612(v0: Adt139, v1: u64, v2: u64, v3: Adt142) -> R<Adt154> {
    let m1882 = match v3.clone() {
        Adt142::C0(_, h1881, _, _, _, _, _) => {
            h1881
        }
    };
    match m1882 {
        Adt143::C0 => {
            let a1883 = v2;
            let a1884 = 48u64;
            if nat_eq(a1883, a1884) {
                Ok(f692(v0, v3, Adt143::C1))
            } else {
                if f600(v2, 49u64, 57u64) {
                    f693(v0, v1, v2, v3, false)
                } else {
                    Ok(f609(Adt120::C7(v1)))
                }
            }
        }
        Adt143::C1 => {
            let a1885 = v2;
            let a1886 = 46u64;
            if nat_eq(a1885, a1886) {
                Ok(f692(v0, v3, Adt143::C3))
            } else {
                if f694(v2) {
                    Ok(f692(v0, v3, Adt143::C5))
                } else {
                    if f695(v2) {
                        Ok(f609(Adt120::C7(v1)))
                    } else {
                        f696(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt143::C2 => {
            if f695(v2) {
                f693(v0, v1, v2, v3, false)
            } else {
                let a1887 = v2;
                let a1888 = 46u64;
                if nat_eq(a1887, a1888) {
                    Ok(f692(v0, v3, Adt143::C3))
                } else {
                    if f694(v2) {
                        Ok(f692(v0, v3, Adt143::C5))
                    } else {
                        f696(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt143::C3 => {
            if f695(v2) {
                f693(v0, v1, v2, v3, true)
            } else {
                Ok(f609(Adt120::C7(v1)))
            }
        }
        Adt143::C4 => {
            if f695(v2) {
                f693(v0, v1, v2, v3, true)
            } else {
                if f694(v2) {
                    Ok(f692(v0, v3, Adt143::C5))
                } else {
                    f696(v0, v1, v2, v3)
                }
            }
        }
        Adt143::C5 => {
            let a1889 = v2;
            let a1890 = 43u64;
            if nat_eq(a1889, a1890) {
                Ok(f608(v0, f680(match v3.clone() {
                    Adt142::C0(h1891, _, _, _, _, _, _) => {
                        h1891
                    }
                }, Adt143::C6, match v3.clone() {
                    Adt142::C0(_, _, h1892, _, _, _, _) => {
                        h1892
                    }
                }, match v3.clone() {
                    Adt142::C0(_, _, _, h1893, _, _, _) => {
                        h1893
                    }
                }, match v3 {
                    Adt142::C0(_, _, _, _, h1894, _, _) => {
                        h1894
                    }
                }, false, 0i64)))
            } else {
                let a1895 = v2;
                let a1896 = 45u64;
                if nat_eq(a1895, a1896) {
                    Ok(f608(v0, f680(match v3.clone() {
                        Adt142::C0(h1897, _, _, _, _, _, _) => {
                            h1897
                        }
                    }, Adt143::C6, match v3.clone() {
                        Adt142::C0(_, _, h1898, _, _, _, _) => {
                            h1898
                        }
                    }, match v3.clone() {
                        Adt142::C0(_, _, _, h1899, _, _, _) => {
                            h1899
                        }
                    }, match v3 {
                        Adt142::C0(_, _, _, _, h1900, _, _) => {
                            h1900
                        }
                    }, true, 0i64)))
                } else {
                    if f695(v2) {
                        f697(v0, v1, v2, Adt142::C0(match v3.clone() {
                            Adt142::C0(h1901, _, _, _, _, _, _) => {
                                h1901
                            }
                        }, match v3.clone() {
                            Adt142::C0(_, h1902, _, _, _, _, _) => {
                                h1902
                            }
                        }, match v3.clone() {
                            Adt142::C0(_, _, h1903, _, _, _, _) => {
                                h1903
                            }
                        }, match v3.clone() {
                            Adt142::C0(_, _, _, h1904, _, _, _) => {
                                h1904
                            }
                        }, match v3 {
                            Adt142::C0(_, _, _, _, h1905, _, _) => {
                                h1905
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f609(Adt120::C7(v1)))
                    }
                }
            }
        }
        Adt143::C6 => {
            if f695(v2) {
                f697(v0, v1, v2, v3.clone(), match v3 {
                    Adt142::C0(_, _, _, _, _, h1906, _) => {
                        h1906
                    }
                })
            } else {
                Ok(f609(Adt120::C7(v1)))
            }
        }
        Adt143::C7 => {
            if f695(v2) {
                f697(v0, v1, v2, v3.clone(), match v3 {
                    Adt142::C0(_, _, _, _, _, h1907, _) => {
                        h1907
                    }
                })
            } else {
                f696(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f613(v0: u64) -> bool {
    let a1908 = v0;
    let a1909 = 9u64;
    if nat_eq(a1908, a1909) {
        true
    } else {
        let a1910 = v0;
        let a1911 = 32u64;
        nat_eq(a1910, a1911)
    }
}

pub fn f614(v0: u64) -> bool {
    let a1912 = 47u64;
    let a1913 = v0;
    if nat_lt(a1912, a1913) {
        let a1914 = v0;
        let a1915 = 58u64;
        nat_lt(a1914, a1915)
    } else {
        false
    }
}

pub fn f615(v0: Result<Adt147, Adt150>) -> Option<Adt95> {
    let m1916 = v0;
    match m1916 {
        Ok(_) => {
            None::<Adt95>
        }
        Err(v2) => {
            let m1917 = v2;
            match m1917 {
                Adt150::C0(_) => {
                    None::<Adt95>
                }
                Adt150::C1(_, _) => {
                    None::<Adt95>
                }
                Adt150::C2(_) => {
                    None::<Adt95>
                }
                Adt150::C3(_) => {
                    None::<Adt95>
                }
                Adt150::C4(v8) => {
                    f698(v8)
                }
            }
        }
    }
}

pub fn f616(v0: Adt18, v1: bool, v2: Adt146, v3: Adt95) -> R<Adt97> {
    if f699(match v3.clone() {
        Adt95::C0(h1918, _) => {
            h1918
        }
    }) {
        Ok(Adt97::C1(v3))
    } else {
        Ok(Adt97::C0(f56(f521(v0, v1, Adt146::C0(match v2 {
            Adt146::C0(h1919, _) => {
                h1919
            }
        }, Err::<Adt147, Adt150>(f700())))?)))
    }
}

pub fn f617(v0: Adt3, v1: bool, v2: Adt104) -> Adt97 {
    let m1921 = match v2.clone() {
        Adt104::C0(_, _, _, _, h1920) => {
            h1920
        }
    };
    match m1921 {
        None => {
            Adt97::C0(f56(v2))
        }
        Some(v3) => {
            f701(v0, v1, v3, v2)
        }
    }
}

pub fn f618(p1925: List<Str>, v1: List<(Str, Adt9)>) -> Option<Str> {
    let m1924 = p1925;
    {
        let mut m1924 = m1924;
        loop {
            let step = {
                let v0 = m1924;
                let m1922 = v0;
                match m1922.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1923 = f383(v2.clone(), v1.clone());
                        match m1923 {
                            Adt118::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt118::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1924 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f619() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f620(p1932: List<(Str, u64)>, v1: List<(Str, Adt9)>) -> Option<Str> {
    let m1931 = p1932;
    {
        let mut m1931 = m1931;
        loop {
            let step = {
                let v0 = m1931;
                let m1926 = v0;
                match m1926.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1928 = f383({
                            let (h1927, _) = v2.clone();
                            h1927
                        }, v1.clone());
                        match m1928 {
                            Adt118::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt118::C1(v4) => {
                                if f702({
                                    let (_, h1929) = v2;
                                    h1929
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1930, _) = v2;
                                        h1930
                                    }))
                                }
                            }
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

pub fn f621() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f622(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>) -> Option<Adt102> {
    let m1933 = f383(Str::lit("$ref"), v2);
    match m1933 {
        Adt118::C0 => {
            None::<Adt102>
        }
        Adt118::C1(v3) => {
            let m1934 = f181(v3);
            match m1934 {
                None => {
                    None::<Adt102>
                }
                Some(v4) => {
                    let m1935 = f703(v1, v4.clone());
                    match m1935 {
                        None => {
                            Some(Adt102::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt102>
                        }
                    }
                }
            }
        }
    }
}

pub fn f623(v0: Str, v1: List<(Str, Adt9)>, v2: Adt9, v3: bool, v4: List<Str>, v5: Adt125) -> R<Adt155> {
    if v3 {
        let m1936 = f181(v2);
        match m1936 {
            None => {
                Ok(Adt155::C0(Adt102::C7(v0)))
            }
            Some(v6) => {
                f704(v0, v1, v6.clone(), f166(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt155::C0(Adt102::C10(v0)))
    }
}

pub fn f624(v0: Str, v1: List<(Str, Adt9)>, v2: bool, v3: List<Str>, v4: List<Adt112>) -> List<Adt112> {
    let m1937 = f383(v0.clone(), v1);
    match m1937 {
        Adt118::C0 => {
            v4
        }
        Adt118::C1(v5) => {
            let m1938 = f281(v5);
            match m1938 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1939 = v0;
                    let a1940 = Str::lit("properties");
                    if equal(a1939, a1940) {
                        f705(match v6 {
                            Adt8::C0(h1941) => {
                                h1941
                            }
                        }, v2, v3, v4)
                    } else {
                        f706(match v6 {
                            Adt8::C0(h1942) => {
                                h1942
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f625(v0: Str, v1: List<(Str, Adt9)>, v2: List<Adt112>) -> List<Adt112> {
    let m1943 = f383(v0, v1);
    match m1943 {
        Adt118::C0 => {
            v2
        }
        Adt118::C1(v3) => {
            List::cons(Adt112::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f626(v0: Str, v1: List<(Str, Adt9)>, v2: List<Adt112>) -> List<Adt112> {
    let m1944 = f383(v0, v1);
    match m1944 {
        Adt118::C0 => {
            v2
        }
        Adt118::C1(v3) => {
            let m1945 = f335(v3);
            match m1945 {
                None => {
                    v2
                }
                Some(v4) => {
                    f707(v4, v2)
                }
            }
        }
    }
}

pub fn f627(p1952: List<(Str, Adt156)>) -> Adt128 {
    let m1951 = p1952;
    {
        let mut m1951 = m1951;
        loop {
            let step = {
                let v0 = m1951;
                let m1946 = v0;
                match m1946.uncons() {
                    None => {
                        Err::<List<(Str, Adt156)>, Adt128>(Adt128::C3(List::<Adt116>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1948 = {
                            let (_, h1947) = v1;
                            h1947
                        };
                        match m1948 {
                            Adt156::C0 => {
                                Ok::<List<(Str, Adt156)>, Adt128>(v2)
                            }
                            Adt156::C1 => {
                                Err::<List<(Str, Adt156)>, Adt128>(Adt128::C0({
                                    let (h1949, _) = v1;
                                    h1949
                                }))
                            }
                            Adt156::C2 => {
                                Err::<List<(Str, Adt156)>, Adt128>(Adt128::C1({
                                    let (h1950, _) = v1;
                                    h1950
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1951 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f628(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt156> {
    let m1953 = f383(Str::lit("type"), v0);
    match m1953 {
        Adt118::C0 => {
            Ok(Adt156::C0)
        }
        Adt118::C1(v2) => {
            let m1954 = f181(v2.clone());
            match m1954 {
                None => {
                    let m1955 = f335(v2);
                    match m1955 {
                        None => {
                            Ok(Adt156::C2)
                        }
                        Some(v4) => {
                            f710(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f708(v3.clone()) {
                        if f709(v3, v1)? {
                            Ok(Adt156::C0)
                        } else {
                            Ok(Adt156::C1)
                        }
                    } else {
                        Ok(Adt156::C2)
                    }
                }
            }
        }
    }
}

pub fn f629(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt156> {
    let m1956 = f383(Str::lit("enum"), v0);
    match m1956 {
        Adt118::C0 => {
            Ok(Adt156::C0)
        }
        Adt118::C1(v2) => {
            let m1957 = f335(v2);
            match m1957 {
                None => {
                    Ok(Adt156::C2)
                }
                Some(v3) => {
                    f711(v3, v1)
                }
            }
        }
    }
}

pub fn f630(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt156> {
    let m1958 = f383(Str::lit("const"), v0);
    match m1958 {
        Adt118::C0 => {
            Ok(Adt156::C0)
        }
        Adt118::C1(v2) => {
            let m1959 = f712(v2, v1)?;
            match m1959 {
                Adt165::C0 => {
                    Ok(Adt156::C0)
                }
                Adt165::C1 => {
                    Ok(Adt156::C1)
                }
                Adt165::C2 => {
                    Ok(Adt156::C2)
                }
            }
        }
    }
}

pub fn f631(v0: Str, v1: u64, v2: List<(Str, Adt9)>, v3: Adt9) -> R<Adt156> {
    let m1960 = f383(v0, v2);
    match m1960 {
        Adt118::C0 => {
            Ok(Adt156::C0)
        }
        Adt118::C1(v4) => {
            let m1961 = f713(v4);
            match m1961 {
                None => {
                    Ok(Adt156::C2)
                }
                Some(v5) => {
                    let m1962 = f713(v3);
                    match m1962 {
                        None => {
                            Ok(Adt156::C0)
                        }
                        Some(v6) => {
                            if f714(v1, f715(v6, v5)?) {
                                Ok(Adt156::C0)
                            } else {
                                Ok(Adt156::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f632(v0: List<(Str, Adt9)>, v1: Adt9) -> R<List<(Str, Adt156)>> {
    let m1963 = v1;
    match m1963 {
        Adt9::C0 => {
            Ok(List::<(Str, Adt156)>::nil())
        }
        Adt9::C1(_) => {
            Ok(List::<(Str, Adt156)>::nil())
        }
        Adt9::C2(_) => {
            Ok(List::<(Str, Adt156)>::nil())
        }
        Adt9::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f716(Str::lit("minLength"), true, v0.clone(), f717(v4.clone())?)?), List::cons((Str::lit("maxLength"), f716(Str::lit("maxLength"), false, v0, f717(v4)?)?), List::<(Str, Adt156)>::nil())))
        }
        Adt9::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f716(Str::lit("minItems"), true, v0.clone(), f718(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f716(Str::lit("maxItems"), false, v0, f718(v5, 0u64)?)?), List::<(Str, Adt156)>::nil())))
        }
        Adt9::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f716(Str::lit("minProperties"), true, v0.clone(), f719(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f716(Str::lit("maxProperties"), false, v0, f719(v6, 0u64)?)?), List::<(Str, Adt156)>::nil())))
        }
    }
}

pub fn f633(v0: List<(Str, Adt9)>, v1: Adt9) -> Adt156 {
    let m1964 = f383(Str::lit("required"), v0);
    match m1964 {
        Adt118::C0 => {
            Adt156::C0
        }
        Adt118::C1(v2) => {
            let m1965 = f335(v2);
            match m1965 {
                None => {
                    Adt156::C2
                }
                Some(v3) => {
                    let m1966 = f281(v1);
                    match m1966 {
                        None => {
                            Adt156::C0
                        }
                        Some(v4) => {
                            f720(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f634(v0: Adt128, v1: Adt128) -> Adt128 {
    let m1967 = v0.clone();
    match m1967 {
        Adt128::C0(_) => {
            v0
        }
        Adt128::C1(_) => {
            v0
        }
        Adt128::C2(_) => {
            v0
        }
        Adt128::C3(v5) => {
            let m1968 = v1.clone();
            match m1968 {
                Adt128::C0(_) => {
                    v1
                }
                Adt128::C1(_) => {
                    v1
                }
                Adt128::C2(_) => {
                    v1
                }
                Adt128::C3(v9) => {
                    Adt128::C3({
                        let a1969 = v5;
                        let a1970 = v9;
                        append_list(a1969, a1970)
                    })
                }
            }
        }
    }
}

pub fn f635(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt114>) -> Adt128 {
    let m1971 = f383(Str::lit("$ref"), v1);
    match m1971 {
        Adt118::C0 => {
            Adt128::C3(List::<Adt116>::nil())
        }
        Adt118::C1(v4) => {
            let m1972 = f181(v4);
            match m1972 {
                None => {
                    Adt128::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1973 = f703(v0, v5.clone());
                    match m1973 {
                        None => {
                            Adt128::C2(v5)
                        }
                        Some(v6) => {
                            Adt128::C3(List::cons(Adt116::C0(v6, v2, v3), List::<Adt116>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f636(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt114>) -> Adt128 {
    let m1974 = f383(Str::lit("properties"), v0);
    match m1974 {
        Adt118::C0 => {
            Adt128::C3(List::<Adt116>::nil())
        }
        Adt118::C1(v3) => {
            let m1975 = f281(v3);
            match m1975 {
                None => {
                    Adt128::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1976 = f281(v1);
                    match m1976 {
                        None => {
                            Adt128::C3(List::<Adt116>::nil())
                        }
                        Some(v5) => {
                            Adt128::C3(f721(match v4 {
                                Adt8::C0(h1977) => {
                                    h1977
                                }
                            }, match v5 {
                                Adt8::C0(h1978) => {
                                    h1978
                                }
                            }, v2, List::<Adt116>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f637(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt114>) -> Adt128 {
    let m1979 = f383(Str::lit("additionalProperties"), v0.clone());
    match m1979 {
        Adt118::C0 => {
            Adt128::C3(List::<Adt116>::nil())
        }
        Adt118::C1(v3) => {
            let m1980 = f281(v1);
            match m1980 {
                None => {
                    Adt128::C3(List::<Adt116>::nil())
                }
                Some(v4) => {
                    let m1981 = f468(v3.clone());
                    match m1981 {
                        None => {
                            Adt128::C3(f724(match v4 {
                                Adt8::C0(h1983) => {
                                    h1983
                                }
                            }, f723(v0), v3, v2, List::<Adt116>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt128::C3(List::<Adt116>::nil())
                            } else {
                                if f722(match v4 {
                                    Adt8::C0(h1982) => {
                                        h1982
                                    }
                                }, f723(v0)) {
                                    Adt128::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt128::C3(List::<Adt116>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f638(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt114>) -> R<Adt128> {
    let m1984 = f383(Str::lit("items"), v0);
    match m1984 {
        Adt118::C0 => {
            Ok(Adt128::C3(List::<Adt116>::nil()))
        }
        Adt118::C1(v3) => {
            let m1985 = f335(v1);
            match m1985 {
                None => {
                    Ok(Adt128::C3(List::<Adt116>::nil()))
                }
                Some(v4) => {
                    Ok(Adt128::C3(f725(v4, 0u64, v3, v2, List::<Adt116>::nil())?))
                }
            }
        }
    }
}

pub fn f639(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt114>) -> Adt128 {
    let m1986 = f383(Str::lit("allOf"), v0);
    match m1986 {
        Adt118::C0 => {
            Adt128::C3(List::<Adt116>::nil())
        }
        Adt118::C1(v3) => {
            let m1987 = f335(v3);
            match m1987 {
                None => {
                    Adt128::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1988 = v4.clone();
                    match m1988.uncons() {
                        None => {
                            Adt128::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt128::C3(f726(v4, v1, v2, List::<Adt116>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f640(v0: Adt117, v1: Adt128, v2: List<(Str, Adt9)>, v3: Adt9, v4: List<Adt114>) -> Adt128 {
    let m1989 = v1.clone();
    match m1989 {
        Adt128::C0(_) => {
            v1
        }
        Adt128::C1(_) => {
            v1
        }
        Adt128::C2(_) => {
            v1
        }
        Adt128::C3(v8) => {
            let m1990 = v8;
            match m1990.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt128::C3(List::cons(Adt116::C5(v0, Adt9::C5(v2), v3, v4), List::<Adt116>::nil()))
                }
            }
        }
    }
}

pub fn f641(p1997: List<u64>, p1998: List<u64>) -> List<u64> {
    let m1996 = (p1997, p1998);
    {
        let mut m1996 = m1996;
        loop {
            let step = {
                let (v0, v1) = m1996;
                let m1991 = v0;
                match m1991.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1992 = v2;
                        let a1993 = 126u64;
                        if nat_eq(a1992, a1993) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1994 = v2;
                            let a1995 = 47u64;
                            if nat_eq(a1994, a1995) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1996 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f642(p2007: u64, p2008: List<u64>, p2009: List<u64>) -> R<Option<List<u64>>> {
    let m2006 = (p2007, (p2008, p2009));
    {
        let mut m2006 = m2006;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2006;
                let m1999 = v0;
                if m1999 == 0 {
                    let m2000 = v1;
                    match m2000.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1999 - 1;
                    let m2001 = v1;
                    match m2001.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m2002 = v7;
                            match m2002.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m2003 = v9;
                                    match m2003.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m2004 = v11;
                                            match m2004.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m2005 = f727(v6, v8, v10, v12, f728(v13.clone()))?;
                                                    match m2005 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f515(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m2006 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(p2014: List<Adt55>, p2015: Adt10) -> R<bool> {
    let m2013 = (p2014, p2015);
    {
        let mut m2013 = m2013;
        loop {
            let step = {
                let (v0, v1) = m2013;
                let m2010 = v0;
                match m2010.uncons() {
                    None => {
                        Err::<(List<Adt55>, Adt10), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f729(f715(match v2.clone() {
                            Adt55::C0(h2011, _, _) => {
                                h2011
                            }
                        }, v1)?) {
                            Ok::<(List<Adt55>, Adt10), R<bool>>((v3, match v2 {
                                Adt55::C0(h2012, _, _) => {
                                    h2012
                                }
                            }))
                        } else {
                            Err::<(List<Adt55>, Adt10), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f730(v0, 0u64, v1)
}

pub fn f645(v0: Adt44) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt9::C3(match v0.clone() {
        Adt44::C0(h2016, _, _) => {
            h2016
        }
    })), {
        let a2019 = f731(match v0.clone() {
            Adt44::C0(_, h2017, _) => {
                h2017
            }
        });
        let a2020 = f732(match v0 {
            Adt44::C0(_, _, h2018) => {
                h2018
            }
        });
        append_list(a2019, a2020)
    })))
}

pub fn f646(v0: Str, v1: Str, v2: Str, v3: Option<Adt45>, v4: Option<Adt8>) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::cons((Str::lit("data"), Adt9::C3(v1)), List::cons((Str::lit("mimeType"), Adt9::C3(v2)), {
        let a2021 = f731(v3);
        let a2022 = f732(v4);
        append_list(a2021, a2022)
    }))))
}

pub fn f647(v0: Adt49) -> R<Adt9> {
    Ok(Adt9::C5({
        let a2046 = List::cons((Str::lit("type"), Adt9::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt49::C0(h2023, _, _, _, _, _, _, _, _) => {
                h2023
            }
        })), List::<(Str, Adt9)>::nil()));
        let a2047 = {
            let a2044 = f733(Str::lit("title"), match v0.clone() {
                Adt49::C0(_, h2024, _, _, _, _, _, _, _) => {
                    h2024
                }
            });
            let a2045 = {
                let a2042 = f734(match v0.clone() {
                    Adt49::C0(_, _, h2025, _, _, _, _, _, _) => {
                        h2025
                    }
                });
                let a2043 = {
                    let a2040 = List::cons((Str::lit("uri"), Adt9::C3(match v0.clone() {
                        Adt49::C0(_, _, _, h2026, _, _, _, _, _) => {
                            h2026
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a2041 = {
                        let a2038 = f733(Str::lit("description"), match v0.clone() {
                            Adt49::C0(_, _, _, _, h2027, _, _, _, _) => {
                                h2027
                            }
                        });
                        let a2039 = {
                            let a2036 = f733(Str::lit("mimeType"), match v0.clone() {
                                Adt49::C0(_, _, _, _, _, h2028, _, _, _) => {
                                    h2028
                                }
                            });
                            let a2037 = {
                                let a2034 = f731(match v0.clone() {
                                    Adt49::C0(_, _, _, _, _, _, h2029, _, _) => {
                                        h2029
                                    }
                                });
                                let a2035 = {
                                    let a2032 = f735(match v0.clone() {
                                        Adt49::C0(_, _, _, _, _, _, _, h2030, _) => {
                                            h2030
                                        }
                                    })?;
                                    let a2033 = f732(match v0 {
                                        Adt49::C0(_, _, _, _, _, _, _, _, h2031) => {
                                            h2031
                                        }
                                    });
                                    append_list(a2032, a2033)
                                };
                                append_list(a2034, a2035)
                            };
                            append_list(a2036, a2037)
                        };
                        append_list(a2038, a2039)
                    };
                    append_list(a2040, a2041)
                };
                append_list(a2042, a2043)
            };
            append_list(a2044, a2045)
        };
        append_list(a2046, a2047)
    }))
}

pub fn f648(v0: Adt50) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f736(match v0.clone() {
        Adt50::C0(h2048, _, _) => {
            h2048
        }
    })), {
        let a2051 = f731(match v0.clone() {
            Adt50::C0(_, h2049, _) => {
                h2049
            }
        });
        let a2052 = f732(match v0 {
            Adt50::C0(_, _, h2050) => {
                h2050
            }
        });
        append_list(a2051, a2052)
    })))
}

pub fn f649(p2061: Bytes, p2062: List<Bytes>) -> List<Bytes> {
    let m2060 = (p2061, p2062);
    {
        let mut m2060 = m2060;
        loop {
            let step = {
                let (v0, v1) = m2060;
                let m2053 = v1;
                match m2053.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a2056 = {
                            let a2054 = v2.clone();
                            length_bytes(a2054)
                        };
                        let a2057 = {
                            let a2055 = v0.clone();
                            length_bytes(a2055)
                        };
                        if nat_eq(a2056, a2057) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a2058 = v2;
                                let a2059 = v0;
                                append_bytes(a2058, a2059)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
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

pub fn f650(v0: u64) -> Bytes {
    let m2066 = {
        let a2063 = f737();
        let a2064 = v0;
        let a2065 = 1u64;
        slice_bytes(a2063, a2064, a2065)
    };
    match m2066 {
        None => {
            f243()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f651(p2072: List<(Str, Adt9)>, p2073: List<(Str, Adt69)>) -> Option<List<(Str, Adt69)>> {
    let m2071 = (p2072, p2073);
    {
        let mut m2071 = m2071;
        loop {
            let step = {
                let (v0, v1) = m2071;
                let m2067 = v0;
                match m2067.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Adt69)>), Option<List<(Str, Adt69)>>>(Some(f509(v1, List::<(Str, Adt69)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2069 = f738({
                            let (_, h2068) = v2;
                            h2068
                        });
                        match m2069 {
                            None => {
                                Err::<(List<(Str, Adt9)>, List<(Str, Adt69)>), Option<List<(Str, Adt69)>>>(None::<List<(Str, Adt69)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt9)>, List<(Str, Adt69)>), Option<List<(Str, Adt69)>>>((v3, List::cons(({
                                    let (h2070, _) = v2;
                                    h2070
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2071 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f652(v0: u64, v1: Adt9, v2: List<Adt131>, v3: List<u64>) -> R<Adt157> {
    let m2074 = v1;
    match m2074 {
        Adt9::C0 => {
            Ok(Adt157::C1(v2, f515(f739(2u64), v3)))
        }
        Adt9::C1(v4) => {
            Ok(Adt157::C1(v2, f515(if v4 {
                f739(0u64)
            } else {
                f739(1u64)
            }, v3)))
        }
        Adt9::C2(v5) => {
            if f740(v5.clone())? {
                Ok(Adt157::C1(v2, f515(f741(v5)?, v3)))
            } else {
                Ok(Adt157::C0(Adt120::C11))
            }
        }
        Adt9::C3(v6) => {
            Ok(Adt157::C1(v2, f515(f653(v6)?, v3)))
        }
        Adt9::C4(v7) => {
            let a2075 = v0;
            let a2076 = f742();
            if nat_lt(a2075, a2076) {
                Ok(Adt157::C1(f743(91u64, Adt131::C2({
                    let a2077 = v0;
                    let a2078 = 1u64;
                    nat_add(a2077, a2078)?
                }, v7), v2), v3))
            } else {
                Ok(Adt157::C0(Adt120::C12))
            }
        }
        Adt9::C5(v8) => {
            let a2079 = v0;
            let a2080 = f742();
            if nat_lt(a2079, a2080) {
                Ok(Adt157::C1(f743(123u64, Adt131::C4({
                    let a2081 = v0;
                    let a2082 = 1u64;
                    nat_add(a2081, a2082)?
                }, v8), v2), v3))
            } else {
                Ok(Adt157::C0(Adt120::C12))
            }
        }
    }
}

pub fn f653(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a2084 = f515(f744(f29({
            let a2083 = v0;
            utf8_encode(a2083)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a2085 = List::cons(34u64, List::<u64>::nil());
        append_list(a2084, a2085)
    }))
}

pub fn f654(v0: Adt37) -> R<Option<List<(Str, Adt9)>>> {
    let m2086 = v0;
    match m2086 {
        Adt37::C0(v1) => {
            Ok(Some(f745(v1)?))
        }
        Adt37::C1(v2) => {
            Ok(Some(f746(v2)?))
        }
        Adt37::C2(v3) => {
            f278(f279(v3)?)
        }
        Adt37::C3(v4) => {
            f278(f279(match v4 {
                Adt54::C0(h2087, _) => {
                    h2087
                }
            })?)
        }
        Adt37::C4(v5) => {
            Ok(Some(f747(v5)?))
        }
        Adt37::C5(v6) => {
            Ok(Some(f748(v6)?))
        }
        Adt37::C6(v7) => {
            Ok(Some(f749(v7)?))
        }
        Adt37::C7(v8) => {
            Ok(Some(f750(v8)?))
        }
        Adt37::C8(v9) => {
            f278(f751(v9)?)
        }
        Adt37::C9(v10) => {
            Ok(Some(f752(v10)?))
        }
        Adt37::C10(_) => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Adt37::C11 => {
            Ok(Some(List::<(Str, Adt9)>::nil()))
        }
        Adt37::C12(v12) => {
            f278(f753(v12)?)
        }
        Adt37::C13(v13) => {
            f278(f754({
                let m2090 = match v13 {
                    Adt87::C0(_, _, h2089) => {
                        h2089
                    }
                };
                match m2090 {
                    Adt89::C0(h2088, _, _) => {
                        h2088
                    }
                }
            }))
        }
        Adt37::C14(v14) => {
            Ok(f755(v14))
        }
    }
}

pub fn f655(v0: Adt147) -> Adt8 {
    Adt8::C0(List::cons((Str::lit("resultType"), Adt9::C3(f756(f757(match v0.clone() {
        Adt147::C0(h2091, _, _) => {
            h2091
        }
    })))), List::cons((Str::lit("_meta"), Adt9::C5(f758({
        let m2094 = match v0 {
            Adt147::C0(_, h2093, _) => {
                h2093
            }
        };
        match m2094 {
            Adt148::C0(h2092, _) => {
                h2092
            }
        }
    }))), List::<(Str, Adt9)>::nil())))
}

pub fn f656(v0: Option<Option<List<u64>>>, v1: Adt104) -> Adt104 {
    let m2095 = v0;
    match m2095 {
        None => {
            f57()
        }
        Some(v2) => {
            Adt104::C0(match v1.clone() {
                Adt104::C0(h2096, _, _, _, _) => {
                    h2096
                }
            }, match v1.clone() {
                Adt104::C0(_, h2097, _, _, _) => {
                    h2097
                }
            }, match v1.clone() {
                Adt104::C0(_, _, h2098, _, _) => {
                    h2098
                }
            }, match v1 {
                Adt104::C0(_, _, _, h2099, _) => {
                    h2099
                }
            }, v2)
        }
    }
}

pub fn f657(v0: Adt18, v1: Adt92) -> R<Option<Option<List<u64>>>> {
    let m2101 = f233(match v0.clone() {
        Adt18::C0(_, _, _, h2100) => {
            h2100
        }
    })?;
    match m2101 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m2104 = f759(f760(match v1 {
                Adt92::C0(h2102, _, _) => {
                    h2102
                }
            }, v2, match v0 {
                Adt18::C0(h2103, _, _, _) => {
                    h2103
                }
            })?)?;
            match m2104 {
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

pub fn f658(v0: bool, v1: Adt104, v2: List<List<u64>>) -> Adt104 {
    if f761(v0, match v1.clone() {
        Adt104::C0(h2105, _, _, _, _) => {
            h2105
        }
    }, v2.clone()) {
        f762(match v1.clone() {
            Adt104::C0(h2106, _, _, _, _) => {
                h2106
            }
        }, {
            let a2108 = v2;
            let a2109 = List::cons(match v1 {
                Adt104::C0(_, _, _, h2107, _) => {
                    h2107
                }
            }, List::<List<u64>>::nil());
            append_list(a2108, a2109)
        })
    } else {
        v1
    }
}

pub fn f659(v0: Adt149) -> R<Option<List<u64>>> {
    let m2110 = f388(f763(v0))?;
    match m2110 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m2111 = f348(v1)?;
            match m2111 {
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

pub fn f660(v0: Adt92) -> R<Option<List<List<u64>>>> {
    if f764(v0.clone()) {
        let m2115 = f765({
            let m2114 = match v0 {
                Adt92::C0(h2113, _, _) => {
                    h2113
                }
            };
            match m2114 {
                Adt89::C0(h2112, _, _) => {
                    h2112
                }
            }
        });
        match m2115 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m2116 = f388(v1)?;
                match m2116 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m2117 = f348(v2)?;
                        match m2117 {
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

pub fn f661() -> u64 {
    404u64
}

pub fn f662(v0: Adt94) -> u64 {
    let m2118 = v0;
    match m2118 {
        Adt94::C0(_) => {
            f354()
        }
        Adt94::C1(_) => {
            f354()
        }
        Adt94::C2(_) => {
            f354()
        }
        Adt94::C3(_) => {
            f354()
        }
        Adt94::C4(_) => {
            f354()
        }
        Adt94::C5(_) => {
            f354()
        }
        Adt94::C6(_) => {
            f100()
        }
        Adt94::C7(_) => {
            f766()
        }
        Adt94::C8(_) => {
            f354()
        }
        Adt94::C9(_) => {
            f100()
        }
        Adt94::C10 => {
            f354()
        }
        Adt94::C11 => {
            f354()
        }
        Adt94::C12(_) => {
            f100()
        }
        Adt94::C13 => {
            f100()
        }
        Adt94::C14 => {
            f100()
        }
        Adt94::C15(_) => {
            f100()
        }
    }
}

pub fn f663(v0: Adt150) -> i64 {
    let m2119 = v0;
    match m2119 {
        Adt150::C0(_) => {
            f767()
        }
        Adt150::C1(_, _) => {
            f768()
        }
        Adt150::C2(_) => {
            f769()
        }
        Adt150::C3(_) => {
            f770()
        }
        Adt150::C4(v6) => {
            f771(v6)
        }
    }
}

pub fn f664(v0: Adt150) -> Str {
    let m2120 = v0;
    match m2120 {
        Adt150::C0(v1) => {
            f772(v1)
        }
        Adt150::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt150::C2(v4) => {
            f773(v4)
        }
        Adt150::C3(_) => {
            Str::lit("Method not found")
        }
        Adt150::C4(v6) => {
            let m2121 = v6;
            match m2121 {
                Adt94::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt94::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt94::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt94::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt94::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt94::C5(_) => {
                    Str::lit("Invalid params: invalid cursor")
                }
                Adt94::C6(_) => {
                    Str::lit("Internal error")
                }
                Adt94::C7(_) => {
                    Str::lit("Rate limit exceeded: too many tool calls")
                }
                Adt94::C8(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt94::C9(_) => {
                    Str::lit("Internal error")
                }
                Adt94::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt94::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt94::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt94::C13 => {
                    Str::lit("Internal error")
                }
                Adt94::C14 => {
                    Str::lit("Internal error")
                }
                Adt94::C15(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f665(v0: Adt150) -> R<Option<Adt9>> {
    let m2122 = v0;
    match m2122 {
        Adt150::C0(_) => {
            Ok(None::<Adt9>)
        }
        Adt150::C1(v2, v3) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("supported"), Adt9::C4(f774(v3))), List::cons((Str::lit("requested"), Adt9::C3(v2)), List::<(Str, Adt9)>::nil())))))
        }
        Adt150::C2(_) => {
            Ok(None::<Adt9>)
        }
        Adt150::C3(_) => {
            Ok(None::<Adt9>)
        }
        Adt150::C4(v6) => {
            let m2123 = v6;
            match m2123 {
                Adt94::C0(_) => {
                    Ok(None::<Adt9>)
                }
                Adt94::C1(_) => {
                    Ok(None::<Adt9>)
                }
                Adt94::C2(v9) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(v9)), List::<(Str, Adt9)>::nil()))))
                }
                Adt94::C3(_) => {
                    Ok(None::<Adt9>)
                }
                Adt94::C4(v11) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("reason"), Adt9::C3(v11)), List::<(Str, Adt9)>::nil()))))
                }
                Adt94::C5(_) => {
                    Ok(None::<Adt9>)
                }
                Adt94::C6(_) => {
                    Ok(None::<Adt9>)
                }
                Adt94::C7(v14) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("limit"), Adt9::C2(Adt10::C0(f393(v14, 0i64)?, 0i64))), List::<(Str, Adt9)>::nil()))))
                }
                Adt94::C8(v15) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("requiredCapabilities"), f775(v15)), List::<(Str, Adt9)>::nil()))))
                }
                Adt94::C9(_) => {
                    Ok(None::<Adt9>)
                }
                Adt94::C10 => {
                    Ok(None::<Adt9>)
                }
                Adt94::C11 => {
                    Ok(None::<Adt9>)
                }
                Adt94::C12(_) => {
                    Ok(None::<Adt9>)
                }
                Adt94::C13 => {
                    Ok(None::<Adt9>)
                }
                Adt94::C14 => {
                    Ok(None::<Adt9>)
                }
                Adt94::C15(_) => {
                    Ok(None::<Adt9>)
                }
            }
        }
    }
}

pub fn f666(v0: List<u64>) -> R<bool> {
    let m2124 = f584(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m2124.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f776(v2)? {
                Ok(false)
            } else {
                f453(f455(v1), f777())
            }
        }
    }
}

pub fn f667(v0: Adt3, v1: Fn9, v2: u64, v3: Adt158) -> R<Result<Adt147, Adt150>> {
    f778(v0.clone(), v1, v2, v3.clone(), f779(match v0 {
        Adt3::C0(_, _, h2125, _, _, _, _, _) => {
            h2125
        }
    }, match v3 {
        Adt158::C0(_, h2126, _, _) => {
            h2126
        }
    }))
}

pub fn f668(v0: Adt161, v1: Adt162) -> Result<Adt158, Adt150> {
    let m2127 = v1;
    match m2127 {
        Adt162::C0(v3) => {
            f780(v3, v0)
        }
        Adt162::C1(v2) => {
            Err::<Adt158, Adt150>(v2)
        }
    }
}

pub fn f669(v0: Str) -> Adt161 {
    let a2128 = v0.clone();
    let a2129 = Str::lit("server/discover");
    if equal(a2128, a2129) {
        Adt161::C1(Adt159::C0)
    } else {
        let a2130 = v0.clone();
        let a2131 = Str::lit("tools/list");
        if equal(a2130, a2131) {
            Adt161::C1(Adt159::C1)
        } else {
            let a2132 = v0.clone();
            let a2133 = Str::lit("tools/call");
            if equal(a2132, a2133) {
                Adt161::C1(Adt159::C2)
            } else {
                let a2134 = v0.clone();
                let a2135 = Str::lit("resources/list");
                if equal(a2134, a2135) {
                    Adt161::C1(Adt159::C3)
                } else {
                    let a2136 = v0.clone();
                    let a2137 = Str::lit("resources/read");
                    if equal(a2136, a2137) {
                        Adt161::C1(Adt159::C4)
                    } else {
                        let a2138 = v0.clone();
                        let a2139 = Str::lit("resources/templates/list");
                        if equal(a2138, a2139) {
                            Adt161::C1(Adt159::C5)
                        } else {
                            let a2140 = v0.clone();
                            let a2141 = Str::lit("prompts/list");
                            if equal(a2140, a2141) {
                                Adt161::C1(Adt159::C6)
                            } else {
                                let a2142 = v0.clone();
                                let a2143 = Str::lit("prompts/get");
                                if equal(a2142, a2143) {
                                    Adt161::C1(Adt159::C7)
                                } else {
                                    let a2144 = v0.clone();
                                    let a2145 = Str::lit("completion/complete");
                                    if equal(a2144, a2145) {
                                        Adt161::C1(Adt159::C8)
                                    } else {
                                        let a2146 = v0.clone();
                                        let a2147 = Str::lit("tasks/get");
                                        if equal(a2146, a2147) {
                                            Adt161::C1(Adt159::C9)
                                        } else {
                                            let a2148 = v0.clone();
                                            let a2149 = Str::lit("tasks/update");
                                            if equal(a2148, a2149) {
                                                Adt161::C1(Adt159::C10)
                                            } else {
                                                let a2150 = v0;
                                                let a2151 = Str::lit("tasks/cancel");
                                                if equal(a2150, a2151) {
                                                    Adt161::C1(Adt159::C11)
                                                } else {
                                                    Adt161::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f670(v0: Option<Adt132>, v1: List<Str>, v2: Adt127) -> R<Adt162> {
    f781(v0, v1, v2.clone(), f782(match v2 {
        Adt127::C0(_, _, h2152) => {
            h2152
        }
    }))
}

pub fn f671(v0: Str) -> Adt164 {
    let m2153 = f669(v0);
    match m2153 {
        Adt161::C0 => {
            Adt164::C0
        }
        Adt161::C1(v1) => {
            f783(v1)
        }
    }
}

pub fn f672(v0: List<u64>) -> Option<List<u64>> {
    let m2154 = f784(f785(), v0);
    match m2154 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m2155 = f784(f786(), f463(v1, List::<u64>::nil()));
            match m2155 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f463(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f673(p2158: List<Adt135>, p2159: List<Adt135>) -> List<Adt135> {
    let m2157 = (p2158, p2159);
    {
        let mut m2157 = m2157;
        loop {
            let step = {
                let (v0, v1) = m2157;
                let m2156 = v0;
                match m2156.uncons() {
                    None => {
                        Err::<(List<Adt135>, List<Adt135>), List<Adt135>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt135>, List<Adt135>), List<Adt135>>((v3, List::cons(v2, v1)))
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

pub fn f674(v0: List<Adt101>, p2166: List<Adt15>, v2: Option<Str>, p2167: List<Adt135>) -> R<List<Adt135>> {
    let m2165 = (p2166, p2167);
    {
        let mut m2165 = m2165;
        loop {
            let step = {
                let (v1, v3) = m2165;
                let m2160 = v1;
                match m2160.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt135>), R<List<Adt135>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f787(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt15>, List<Adt135>), R<List<Adt135>>>((v5, List::cons(Adt135::C0(match v4.clone() {
                                Adt15::C0(_, h2161, _, _) => {
                                    h2161
                                }
                            }, match v4.clone() {
                                Adt15::C0(_, _, h2162, _) => {
                                    h2162
                                }
                            }, match v4.clone() {
                                Adt15::C0(_, _, _, h2163) => {
                                    h2163
                                }
                            }, f788(v0.clone(), match v4 {
                                Adt15::C0(_, h2164, _, _) => {
                                    h2164
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt15>, List<Adt135>), R<List<Adt135>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(p2183: u64, p2184: i64, p2185: i64) -> R<Option<i64>> {
    let m2182 = (p2183, (p2184, p2185));
    {
        let mut m2182 = m2182;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2182;
                let m2168 = v0;
                if m2168 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m2168 - 1;
                    if f387(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m2171 = {
                            let a2169 = v2;
                            let a2170 = 0i64;
                            compare(a2169, a2170)
                        };
                        if match m2171 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f387({
                                let a2172 = v1;
                                let a2173 = 10i64;
                                let a2174 = 0i64;
                                int_rem(a2172, a2173, a2174)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a2175 = v1;
                                    let a2176 = 10i64;
                                    let a2177 = 0i64;
                                    int_quot(a2175, a2176, a2177)?
                                }, f491(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a2178 = v1;
                                let a2179 = 10i64;
                                int_mul(a2178, a2179)?
                            }, {
                                let a2180 = v2;
                                let a2181 = 1i64;
                                int_sub(a2180, a2181)?
                            })))
                        }
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

pub fn f676(v0: i64) -> Option<i64> {
    let m2188 = {
        let a2186 = 9007199254740991i64;
        let a2187 = v0;
        compare(a2186, a2187)
    };
    if if match m2188 {
        Ordering::Less => {
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
        let m2191 = {
            let a2189 = v0;
            let a2190 = -9007199254740991i64;
            compare(a2189, a2190)
        };
        match m2191 {
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

pub fn f677(v0: Adt111, v1: Str, v2: Adt8) -> Result<Adt127, Adt121> {
    let m2192 = f275(Str::lit("params"), v2);
    match m2192 {
        Adt118::C0 => {
            Ok::<Adt127, Adt121>(Adt127::C0(v0, v1, None::<Adt8>))
        }
        Adt118::C1(v3) => {
            let m2193 = f281(v3);
            match m2193 {
                None => {
                    f449(Some(v0), Adt122::C6)
                }
                Some(v4) => {
                    Ok::<Adt127, Adt121>(Adt127::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f678(v0: Adt143) -> bool {
    let m2194 = v0;
    match m2194 {
        Adt143::C0 => {
            false
        }
        Adt143::C1 => {
            true
        }
        Adt143::C2 => {
            true
        }
        Adt143::C3 => {
            false
        }
        Adt143::C4 => {
            true
        }
        Adt143::C5 => {
            false
        }
        Adt143::C6 => {
            false
        }
        Adt143::C7 => {
            true
        }
    }
}

pub fn f679(v0: Adt139, v1: Adt144, v2: Adt140, v3: u64) -> R<Adt154> {
    let a2196 = match v0.clone() {
        Adt139::C0(_, _, h2195) => {
            h2195
        }
    };
    let a2197 = f742();
    if nat_lt(a2196, a2197) {
        Ok(Adt154::C1(Adt139::C0(v2, List::cons(v1, match v0.clone() {
            Adt139::C0(_, h2198, _) => {
                h2198
            }
        }), {
            let a2200 = match v0 {
                Adt139::C0(_, _, h2199) => {
                    h2199
                }
            };
            let a2201 = 1u64;
            nat_add(a2200, a2201)?
        })))
    } else {
        Ok(f609(Adt120::C9(v3)))
    }
}

pub fn f680(v0: bool, v1: Adt143, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt140 {
    Adt140::C8(Adt142::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f681(v0: u64) -> i64 {
    let a2202 = v0;
    let a2203 = 0u64;
    if nat_eq(a2202, a2203) {
        0i64
    } else {
        let a2204 = v0;
        let a2205 = 1u64;
        if nat_eq(a2204, a2205) {
            1i64
        } else {
            let a2206 = v0;
            let a2207 = 2u64;
            if nat_eq(a2206, a2207) {
                2i64
            } else {
                let a2208 = v0;
                let a2209 = 3u64;
                if nat_eq(a2208, a2209) {
                    3i64
                } else {
                    let a2210 = v0;
                    let a2211 = 4u64;
                    if nat_eq(a2210, a2211) {
                        4i64
                    } else {
                        let a2212 = v0;
                        let a2213 = 5u64;
                        if nat_eq(a2212, a2213) {
                            5i64
                        } else {
                            let a2214 = v0;
                            let a2215 = 6u64;
                            if nat_eq(a2214, a2215) {
                                6i64
                            } else {
                                let a2216 = v0;
                                let a2217 = 7u64;
                                if nat_eq(a2216, a2217) {
                                    7i64
                                } else {
                                    let a2218 = v0;
                                    let a2219 = 8u64;
                                    if nat_eq(a2218, a2219) {
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

pub fn f682(v0: Adt139, v1: u64) -> Adt154 {
    let m2221 = match v0.clone() {
        Adt139::C0(_, h2220, _) => {
            h2220
        }
    };
    match m2221.uncons() {
        None => {
            f609(Adt120::C2(v1))
        }
        Some((v2, _)) => {
            let m2222 = v2;
            match m2222 {
                Adt144::C0(_) => {
                    f608(v0, Adt140::C0)
                }
                Adt144::C1(_, _) => {
                    f608(v0, Adt140::C4)
                }
            }
        }
    }
}

pub fn f683(v0: Adt139, v1: Adt140) -> Adt139 {
    Adt139::C0(v1, match v0.clone() {
        Adt139::C0(_, h2223, _) => {
            h2223
        }
    }, match v0 {
        Adt139::C0(_, _, h2224) => {
            h2224
        }
    })
}

pub fn f684(v0: Adt139, v1: u64, v2: List<u64>, v3: bool) -> Adt154 {
    let m2226 = {
        let a2225 = f493(f243(), f515(v2, List::<u64>::nil()));
        utf8_decode(a2225)
    };
    match m2226 {
        None => {
            f609(Adt120::C14)
        }
        Some(v4) => {
            if v3 {
                f789(v0, v1, v4)
            } else {
                f690(v0, Adt9::C3(v4))
            }
        }
    }
}

pub fn f685(v0: u64) -> u64 {
    let a2227 = v0;
    let a2228 = 34u64;
    if nat_eq(a2227, a2228) {
        34u64
    } else {
        let a2229 = v0;
        let a2230 = 92u64;
        if nat_eq(a2229, a2230) {
            92u64
        } else {
            let a2231 = v0;
            let a2232 = 47u64;
            if nat_eq(a2231, a2232) {
                47u64
            } else {
                let a2233 = v0;
                let a2234 = 98u64;
                if nat_eq(a2233, a2234) {
                    8u64
                } else {
                    let a2235 = v0;
                    let a2236 = 102u64;
                    if nat_eq(a2235, a2236) {
                        12u64
                    } else {
                        let a2237 = v0;
                        let a2238 = 110u64;
                        if nat_eq(a2237, a2238) {
                            10u64
                        } else {
                            let a2239 = v0;
                            let a2240 = 114u64;
                            if nat_eq(a2239, a2240) {
                                13u64
                            } else {
                                let a2241 = v0;
                                let a2242 = 116u64;
                                if nat_eq(a2241, a2242) {
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

pub fn f686(v0: u64) -> u64 {
    let a2243 = v0;
    let a2244 = 48u64;
    if nat_lt(a2243, a2244) {
        16u64
    } else {
        let a2245 = v0;
        let a2246 = 58u64;
        if nat_lt(a2245, a2246) {
            let a2247 = v0;
            let a2248 = 48u64;
            nat_sub(a2247, a2248)
        } else {
            let a2249 = v0;
            let a2250 = 65u64;
            if nat_lt(a2249, a2250) {
                16u64
            } else {
                let a2251 = v0;
                let a2252 = 71u64;
                if nat_lt(a2251, a2252) {
                    let a2253 = v0;
                    let a2254 = 55u64;
                    nat_sub(a2253, a2254)
                } else {
                    let a2255 = v0;
                    let a2256 = 97u64;
                    if nat_lt(a2255, a2256) {
                        16u64
                    } else {
                        let a2257 = v0;
                        let a2258 = 103u64;
                        if nat_lt(a2257, a2258) {
                            let a2259 = v0;
                            let a2260 = 87u64;
                            nat_sub(a2259, a2260)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f687(v0: Adt139, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt154> {
    let a2261 = v3;
    let a2262 = 0u64;
    if nat_eq(a2261, a2262) {
        if f600(v2, 55296u64, 56319u64) {
            Ok(f608(v0, Adt140::C6(Adt141::C3(v2), v4, v5)))
        } else {
            if f600(v2, 56320u64, 57343u64) {
                Ok(f609(Adt120::C6(v1)))
            } else {
                Ok(f608(v0, Adt140::C6(Adt141::C0, f790(v2, v4)?, v5)))
            }
        }
    } else {
        if f600(v2, 56320u64, 57343u64) {
            Ok(f608(v0, Adt140::C6(Adt141::C0, f790({
                let a2271 = 65536u64;
                let a2272 = {
                    let a2269 = {
                        let a2265 = {
                            let a2263 = v3;
                            let a2264 = 55296u64;
                            nat_sub(a2263, a2264)
                        };
                        let a2266 = 1024u64;
                        nat_mul(a2265, a2266)?
                    };
                    let a2270 = {
                        let a2267 = v2;
                        let a2268 = 56320u64;
                        nat_sub(a2267, a2268)
                    };
                    nat_add(a2269, a2270)?
                };
                nat_add(a2271, a2272)?
            }, v4)?, v5)))
        } else {
            Ok(f609(Adt120::C6(v1)))
        }
    }
}

pub fn f688(v0: u64, v1: u64) -> u64 {
    let m2275 = {
        let a2273 = f689(v0);
        let a2274 = v1;
        index_list(a2273, a2274)
    };
    match m2275 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f689(v0: u64) -> List<u64> {
    let a2276 = v0;
    let a2277 = 0u64;
    if nat_eq(a2276, a2277) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a2278 = v0;
        let a2279 = 1u64;
        if nat_eq(a2278, a2279) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f690(v0: Adt139, v1: Adt9) -> Adt154 {
    Adt154::C1(f602(v1, match v0.clone() {
        Adt139::C0(_, h2280, _) => {
            h2280
        }
    }, match v0 {
        Adt139::C0(_, _, h2281) => {
            h2281
        }
    }))
}

pub fn f691(v0: u64) -> Adt9 {
    let a2282 = v0;
    let a2283 = 0u64;
    if nat_eq(a2282, a2283) {
        Adt9::C1(true)
    } else {
        let a2284 = v0;
        let a2285 = 1u64;
        if nat_eq(a2284, a2285) {
            Adt9::C1(false)
        } else {
            Adt9::C0
        }
    }
}

pub fn f692(v0: Adt139, v1: Adt142, v2: Adt143) -> Adt154 {
    f608(v0, Adt140::C8(Adt142::C0(match v1.clone() {
        Adt142::C0(h2286, _, _, _, _, _, _) => {
            h2286
        }
    }, v2, match v1.clone() {
        Adt142::C0(_, _, h2287, _, _, _, _) => {
            h2287
        }
    }, match v1.clone() {
        Adt142::C0(_, _, _, h2288, _, _, _) => {
            h2288
        }
    }, match v1.clone() {
        Adt142::C0(_, _, _, _, h2289, _, _) => {
            h2289
        }
    }, match v1.clone() {
        Adt142::C0(_, _, _, _, _, h2290, _) => {
            h2290
        }
    }, match v1 {
        Adt142::C0(_, _, _, _, _, _, h2291) => {
            h2291
        }
    })))
}

pub fn f693(v0: Adt139, v1: u64, v2: u64, v3: Adt142, v4: bool) -> R<Adt154> {
    let v5: u64 = {
        let a2293 = match v3.clone() {
            Adt142::C0(_, _, _, h2292, _, _, _) => {
                h2292
            }
        };
        let a2294 = 0u64;
        if if nat_eq(a2293, a2294) {
            let a2295 = v2;
            let a2296 = 48u64;
            nat_eq(a2295, a2296)
        } else {
            false
        } {
            0u64
        } else {
            let a2298 = match v3.clone() {
                Adt142::C0(_, _, _, h2297, _, _, _) => {
                    h2297
                }
            };
            let a2299 = 1u64;
            nat_add(a2298, a2299)?
        }
    };
    let v6: i64 = if v4 {
        f491(match v3.clone() {
            Adt142::C0(_, _, _, _, h2300, _, _) => {
                h2300
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt142::C0(_, _, _, _, h2301, _, _) => {
                h2301
            }
        }
    };
    let a2302 = f791();
    let a2303 = v5;
    if nat_lt(a2302, a2303) {
        Ok(f609(Adt120::C8(v1)))
    } else {
        let m2306 = {
            let a2304 = 99999i64;
            let a2305 = v6;
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
            Ok(f609(Adt120::C8(v1)))
        } else {
            Ok(f608(v0, f680(match v3.clone() {
                Adt142::C0(h2307, _, _, _, _, _, _) => {
                    h2307
                }
            }, if v4 {
                Adt143::C4
            } else {
                Adt143::C2
            }, f491({
                let a2309 = match v3.clone() {
                    Adt142::C0(_, _, h2308, _, _, _, _) => {
                        h2308
                    }
                };
                let a2310 = 10i64;
                int_mul(a2309, a2310)?
            }, f681({
                let a2311 = v2;
                let a2312 = 48u64;
                nat_sub(a2311, a2312)
            }))?, v5, v6, match v3.clone() {
                Adt142::C0(_, _, _, _, _, h2313, _) => {
                    h2313
                }
            }, match v3 {
                Adt142::C0(_, _, _, _, _, _, h2314) => {
                    h2314
                }
            })))
        }
    }
}

pub fn f694(v0: u64) -> bool {
    let a2315 = v0;
    let a2316 = 101u64;
    if nat_eq(a2315, a2316) {
        true
    } else {
        let a2317 = v0;
        let a2318 = 69u64;
        nat_eq(a2317, a2318)
    }
}

pub fn f695(v0: u64) -> bool {
    f600(v0, 48u64, 57u64)
}

pub fn f696(v0: Adt139, v1: u64, v2: u64, v3: Adt142) -> R<Adt154> {
    let m2319 = f601(v3)?;
    match m2319 {
        Adt160::C0 => {
            Ok(f609(Adt120::C7(v1)))
        }
        Adt160::C1 => {
            Ok(f609(Adt120::C8(v1)))
        }
        Adt160::C2(v4) => {
            Ok(f606(f602(v4, match v0.clone() {
                Adt139::C0(_, h2320, _) => {
                    h2320
                }
            }, match v0 {
                Adt139::C0(_, _, h2321) => {
                    h2321
                }
            }), v1, v2))
        }
    }
}

pub fn f697(v0: Adt139, v1: u64, v2: u64, v3: Adt142, v4: bool) -> R<Adt154> {
    let v5: i64 = f491({
        let a2323 = match v3.clone() {
            Adt142::C0(_, _, _, _, _, _, h2322) => {
                h2322
            }
        };
        let a2324 = 10i64;
        int_mul(a2323, a2324)?
    }, f681({
        let a2325 = v2;
        let a2326 = 48u64;
        nat_sub(a2325, a2326)
    }))?;
    let m2329 = {
        let a2327 = 99999i64;
        let a2328 = v5;
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
        Ok(f609(Adt120::C8(v1)))
    } else {
        Ok(f608(v0, f680(match v3.clone() {
            Adt142::C0(h2330, _, _, _, _, _, _) => {
                h2330
            }
        }, Adt143::C7, match v3.clone() {
            Adt142::C0(_, _, h2331, _, _, _, _) => {
                h2331
            }
        }, match v3.clone() {
            Adt142::C0(_, _, _, h2332, _, _, _) => {
                h2332
            }
        }, match v3 {
            Adt142::C0(_, _, _, _, h2333, _, _) => {
                h2333
            }
        }, v4, v5)))
    }
}

pub fn f698(v0: Adt94) -> Option<Adt95> {
    let m2334 = v0;
    match m2334 {
        Adt94::C0(_) => {
            None::<Adt95>
        }
        Adt94::C1(_) => {
            None::<Adt95>
        }
        Adt94::C2(_) => {
            None::<Adt95>
        }
        Adt94::C3(_) => {
            None::<Adt95>
        }
        Adt94::C4(_) => {
            None::<Adt95>
        }
        Adt94::C5(_) => {
            None::<Adt95>
        }
        Adt94::C6(_) => {
            None::<Adt95>
        }
        Adt94::C7(_) => {
            None::<Adt95>
        }
        Adt94::C8(_) => {
            None::<Adt95>
        }
        Adt94::C9(_) => {
            None::<Adt95>
        }
        Adt94::C10 => {
            None::<Adt95>
        }
        Adt94::C11 => {
            None::<Adt95>
        }
        Adt94::C12(_) => {
            None::<Adt95>
        }
        Adt94::C13 => {
            None::<Adt95>
        }
        Adt94::C14 => {
            None::<Adt95>
        }
        Adt94::C15(v12) => {
            Some(v12)
        }
    }
}

pub fn f699(v0: u64) -> bool {
    let a2335 = v0;
    let a2336 = f792();
    nat_lt(a2335, a2336)
}

pub fn f700() -> Adt150 {
    Adt150::C4(Adt94::C9(Str::lit("the effect tag does not fit thirty-two bits")))
}

pub fn f701(v0: Adt3, v1: bool, v2: List<u64>, v3: Adt104) -> Adt97 {
    if v1 {
        Adt97::C0(f56(f793(v3)))
    } else {
        if f699(match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, _, h2337) => {
                h2337
            }
        }) {
            Adt97::C1(f794(v0, v2))
        } else {
            f795()
        }
    }
}

pub fn f702(v0: u64, v1: Adt9) -> bool {
    let a2338 = v0;
    let a2339 = 0u64;
    if nat_eq(a2338, a2339) {
        f796(v1)
    } else {
        let a2340 = v0;
        let a2341 = 1u64;
        if nat_eq(a2340, a2341) {
            f797(v1)
        } else {
            let a2342 = v0;
            let a2343 = 2u64;
            if nat_eq(a2342, a2343) {
                f798(v1)
            } else {
                let a2344 = v0;
                let a2345 = 3u64;
                if nat_eq(a2344, a2345) {
                    f799(v1)
                } else {
                    let a2346 = v0;
                    let a2347 = 4u64;
                    if nat_eq(a2346, a2347) {
                        f800(v1)
                    } else {
                        let a2348 = v0;
                        let a2349 = 5u64;
                        if nat_eq(a2348, a2349) {
                            f801(v1)
                        } else {
                            let a2350 = v0;
                            let a2351 = 6u64;
                            if nat_eq(a2350, a2351) {
                                f802(v1)
                            } else {
                                let a2352 = v0;
                                let a2353 = 7u64;
                                if nat_eq(a2352, a2353) {
                                    f803(v1)
                                } else {
                                    f804(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f703(v0: Adt9, v1: Str) -> Option<Adt9> {
    let a2354 = v1.clone();
    let a2355 = Str::lit("#");
    if equal(a2354, a2355) {
        Some(v0)
    } else {
        let m2356 = f281(v0);
        match m2356 {
            None => {
                None::<Adt9>
            }
            Some(v2) => {
                let m2357 = f275(Str::lit("$defs"), v2);
                match m2357 {
                    Adt118::C0 => {
                        None::<Adt9>
                    }
                    Adt118::C1(v3) => {
                        let m2358 = f281(v3);
                        match m2358 {
                            None => {
                                None::<Adt9>
                            }
                            Some(v4) => {
                                f805(v1, match v4 {
                                    Adt8::C0(h2359) => {
                                        h2359
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

pub fn f704(v0: Str, v1: List<(Str, Adt9)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt125) -> R<Adt155> {
    let m2360 = v3.clone();
    match m2360.uncons() {
        None => {
            Ok(Adt155::C0(Adt102::C8(v0)))
        }
        Some((_, _)) => {
            if f370(v3.clone(), true) {
                let m2361 = f806(v1);
                match m2361 {
                    None => {
                        Ok(Adt155::C0(Adt102::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f807(v3)?;
                        if f808(v9.clone(), match v5.clone() {
                            Adt125::C0(_, h2362) => {
                                h2362
                            }
                        }) {
                            Ok(Adt155::C0(Adt102::C12(v0, v2)))
                        } else {
                            Ok(Adt155::C1(Adt125::C0(List::cons(Adt15::C0(v0, v2, f402(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt125::C0(h2363, _) => {
                                    h2363
                                }
                            }), List::cons(v9, match v5 {
                                Adt125::C0(_, h2364) => {
                                    h2364
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt155::C0(Adt102::C9(v0)))
            }
        }
    }
}

pub fn f705(p2369: List<(Str, Adt9)>, v1: bool, v2: List<Str>, p2370: List<Adt112>) -> List<Adt112> {
    let m2368 = (p2369, p2370);
    {
        let mut m2368 = m2368;
        loop {
            let step = {
                let (v0, v3) = m2368;
                let m2365 = v0;
                match m2365.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt9)>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0({
                            let (_, h2366) = v4;
                            h2366
                        }, v1, List::cons({
                            let (h2367, _) = v4;
                            h2367
                        }, v2.clone())), v3)))
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

pub fn f706(p2374: List<(Str, Adt9)>, p2375: List<Adt112>) -> List<Adt112> {
    let m2373 = (p2374, p2375);
    {
        let mut m2373 = m2373;
        loop {
            let step = {
                let (v0, v1) = m2373;
                let m2371 = v0;
                match m2371.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt112>), List<Adt112>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<Adt112>), List<Adt112>>((v3, List::cons(Adt112::C0({
                            let (_, h2372) = v2;
                            h2372
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f707(p2378: List<Adt9>, p2379: List<Adt112>) -> List<Adt112> {
    let m2377 = (p2378, p2379);
    {
        let mut m2377 = m2377;
        loop {
            let step = {
                let (v0, v1) = m2377;
                let m2376 = v0;
                match m2376.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt112>), List<Adt112>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt9>, List<Adt112>), List<Adt112>>((v3, List::cons(Adt112::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f708(v0: Str) -> bool {
    let a2380 = v0.clone();
    let a2381 = Str::lit("null");
    if equal(a2380, a2381) {
        true
    } else {
        let a2382 = v0.clone();
        let a2383 = Str::lit("boolean");
        if equal(a2382, a2383) {
            true
        } else {
            let a2384 = v0.clone();
            let a2385 = Str::lit("object");
            if equal(a2384, a2385) {
                true
            } else {
                let a2386 = v0.clone();
                let a2387 = Str::lit("array");
                if equal(a2386, a2387) {
                    true
                } else {
                    let a2388 = v0.clone();
                    let a2389 = Str::lit("number");
                    if equal(a2388, a2389) {
                        true
                    } else {
                        let a2390 = v0.clone();
                        let a2391 = Str::lit("string");
                        if equal(a2390, a2391) {
                            true
                        } else {
                            let a2392 = v0;
                            let a2393 = Str::lit("integer");
                            equal(a2392, a2393)
                        }
                    }
                }
            }
        }
    }
}

pub fn f709(v0: Str, v1: Adt9) -> R<bool> {
    let a2394 = v0.clone();
    let a2395 = f809(v1.clone());
    if equal(a2394, a2395) {
        Ok(true)
    } else {
        let a2396 = v0;
        let a2397 = Str::lit("integer");
        if equal(a2396, a2397) {
            let m2398 = f713(v1);
            match m2398 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f810(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f710(p2401: List<Adt9>, v1: Adt9, p2402: bool) -> R<Adt156> {
    let m2400 = (p2401, p2402);
    {
        let mut m2400 = m2400;
        loop {
            let step = {
                let (v0, v2) = m2400;
                let m2399 = v0;
                match m2399.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt9>, bool), R<Adt156>>(Ok(Adt156::C0))
                        } else {
                            Err::<(List<Adt9>, bool), R<Adt156>>(Ok(Adt156::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f708(f811(v3.clone())) {
                            Ok::<(List<Adt9>, bool), R<Adt156>>((v4, if v2 {
                                true
                            } else {
                                f709(f811(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt9>, bool), R<Adt156>>(Ok(Adt156::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f711(p2406: List<Adt9>, v1: Adt9) -> R<Adt156> {
    let m2405 = p2406;
    {
        let mut m2405 = m2405;
        loop {
            let step = {
                let v0 = m2405;
                let m2403 = v0;
                match m2403.uncons() {
                    None => {
                        Err::<List<Adt9>, R<Adt156>>(Ok(Adt156::C1))
                    }
                    Some((v2, v3)) => {
                        let m2404 = f712(v2, v1.clone())?;
                        match m2404 {
                            Adt165::C0 => {
                                Err::<List<Adt9>, R<Adt156>>(Ok(Adt156::C0))
                            }
                            Adt165::C1 => {
                                Ok::<List<Adt9>, R<Adt156>>(v3)
                            }
                            Adt165::C2 => {
                                Err::<List<Adt9>, R<Adt156>>(Ok(Adt156::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f712(v0: Adt9, v1: Adt9) -> R<Adt165> {
    f812(f813(), List::cons((v0, v1), List::<(Adt9, Adt9)>::nil()))
}

pub fn f713(v0: Adt9) -> Option<Adt10> {
    let m2407 = v0;
    match m2407 {
        Adt9::C0 => {
            None::<Adt10>
        }
        Adt9::C1(_) => {
            None::<Adt10>
        }
        Adt9::C2(v2) => {
            Some(v2)
        }
        Adt9::C3(_) => {
            None::<Adt10>
        }
        Adt9::C4(_) => {
            None::<Adt10>
        }
        Adt9::C5(_) => {
            None::<Adt10>
        }
    }
}

pub fn f714(v0: u64, v1: Adt166) -> bool {
    let a2408 = v0;
    let a2409 = 0u64;
    if nat_eq(a2408, a2409) {
        let a2410 = f814(v1);
        bool_not(a2410)
    } else {
        let a2411 = v0;
        let a2412 = 1u64;
        if nat_eq(a2411, a2412) {
            let a2413 = f729(v1);
            bool_not(a2413)
        } else {
            let a2414 = v0;
            let a2415 = 2u64;
            if nat_eq(a2414, a2415) {
                f729(v1)
            } else {
                f814(v1)
            }
        }
    }
}

pub fn f715(v0: Adt10, v1: Adt10) -> R<Adt166> {
    f815(v0.clone(), v1.clone(), f816(match v0 {
        Adt10::C0(h2416, _) => {
            h2416
        }
    }), f816(match v1 {
        Adt10::C0(h2417, _) => {
            h2417
        }
    }))
}

pub fn f716(v0: Str, v1: bool, v2: List<(Str, Adt9)>, v3: u64) -> R<Adt156> {
    let m2418 = f383(v0, v2);
    match m2418 {
        Adt118::C0 => {
            Ok(Adt156::C0)
        }
        Adt118::C1(v4) => {
            let m2419 = f277(v4);
            match m2419 {
                None => {
                    Ok(Adt156::C2)
                }
                Some(v5) => {
                    let m2422 = {
                        let a2420 = v5;
                        let a2421 = 0i64;
                        compare(a2420, a2421)
                    };
                    if match m2422 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt156::C2)
                    } else {
                        if v1 {
                            if f814(f817(f393(v3, 0i64)?, v5)) {
                                Ok(Adt156::C1)
                            } else {
                                Ok(Adt156::C0)
                            }
                        } else {
                            if f729(f817(f393(v3, 0i64)?, v5)) {
                                Ok(Adt156::C1)
                            } else {
                                Ok(Adt156::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f717(v0: Str) -> R<u64> {
    f818(f29({
        let a2423 = v0;
        utf8_encode(a2423)
    })?, 0u64)
}

pub fn f718(p2428: List<Adt9>, p2429: u64) -> R<u64> {
    let m2427 = (p2428, p2429);
    {
        let mut m2427 = m2427;
        loop {
            let step = {
                let (v0, v1) = m2427;
                let m2424 = v0;
                match m2424.uncons() {
                    None => {
                        Err::<(List<Adt9>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt9>, u64), R<u64>>((v3, {
                            let a2425 = v1;
                            let a2426 = 1u64;
                            nat_add(a2425, a2426)?
                        }))
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

pub fn f719(p2434: List<(Str, Adt9)>, p2435: u64) -> R<u64> {
    let m2433 = (p2434, p2435);
    {
        let mut m2433 = m2433;
        loop {
            let step = {
                let (v0, v1) = m2433;
                let m2430 = v0;
                match m2430.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt9)>, u64), R<u64>>((v3, {
                            let a2431 = v1;
                            let a2432 = 1u64;
                            nat_add(a2431, a2432)?
                        }))
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

pub fn f720(p2439: List<Adt9>, v1: Adt8) -> Adt156 {
    let m2438 = p2439;
    {
        let mut m2438 = m2438;
        loop {
            let step = {
                let v0 = m2438;
                let m2436 = v0;
                match m2436.uncons() {
                    None => {
                        Err::<List<Adt9>, Adt156>(Adt156::C0)
                    }
                    Some((v2, v3)) => {
                        let m2437 = f181(v2);
                        match m2437 {
                            None => {
                                Err::<List<Adt9>, Adt156>(Adt156::C2)
                            }
                            Some(v4) => {
                                if f422(v4, v1.clone()) {
                                    Ok::<List<Adt9>, Adt156>(v3)
                                } else {
                                    Err::<List<Adt9>, Adt156>(Adt156::C1)
                                }
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

pub fn f721(p2446: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, v2: List<Adt114>, p2447: List<Adt116>) -> List<Adt116> {
    let m2445 = (p2446, p2447);
    {
        let mut m2445 = m2445;
        loop {
            let step = {
                let (v0, v3) = m2445;
                let m2440 = v0;
                match m2440.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt116>), List<Adt116>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2442 = f383({
                            let (h2441, _) = v4.clone();
                            h2441
                        }, v1.clone());
                        match m2442 {
                            Adt118::C0 => {
                                Ok::<(List<(Str, Adt9)>, List<Adt116>), List<Adt116>>((v5, v3))
                            }
                            Adt118::C1(v6) => {
                                Ok::<(List<(Str, Adt9)>, List<Adt116>), List<Adt116>>((v5, List::cons(Adt116::C0({
                                    let (_, h2443) = v4;
                                    h2443
                                }, v6, List::cons(Adt114::C0({
                                    let (h2444, _) = v4;
                                    h2444
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f722(p2452: List<(Str, Adt9)>, v1: List<(Str, Adt9)>) -> bool {
    let m2451 = p2452;
    {
        let mut m2451 = m2451;
        loop {
            let step = {
                let v0 = m2451;
                let m2448 = v0;
                match m2448.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2450 = f383({
                            let (h2449, _) = v2;
                            h2449
                        }, v1.clone());
                        match m2450 {
                            Adt118::C0 => {
                                Err::<List<(Str, Adt9)>, bool>(true)
                            }
                            Adt118::C1(_) => {
                                Ok::<List<(Str, Adt9)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f723(v0: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m2453 = f383(Str::lit("properties"), v0);
    match m2453 {
        Adt118::C0 => {
            List::<(Str, Adt9)>::nil()
        }
        Adt118::C1(v1) => {
            let m2454 = f281(v1);
            match m2454 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt8::C0(h2455) => {
                            h2455
                        }
                    }
                }
            }
        }
    }
}

pub fn f724(p2462: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt114>, p2463: List<Adt116>) -> List<Adt116> {
    let m2461 = (p2462, p2463);
    {
        let mut m2461 = m2461;
        loop {
            let step = {
                let (v0, v4) = m2461;
                let m2456 = v0;
                match m2456.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt116>), List<Adt116>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2458 = f383({
                            let (h2457, _) = v5.clone();
                            h2457
                        }, v1.clone());
                        match m2458 {
                            Adt118::C0 => {
                                Ok::<(List<(Str, Adt9)>, List<Adt116>), List<Adt116>>((v6, List::cons(Adt116::C0(v2.clone(), {
                                    let (_, h2459) = v5;
                                    h2459
                                }, List::cons(Adt114::C0({
                                    let (h2460, _) = v5;
                                    h2460
                                }), v3.clone())), v4)))
                            }
                            Adt118::C1(_) => {
                                Ok::<(List<(Str, Adt9)>, List<Adt116>), List<Adt116>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f725(p2468: List<Adt9>, p2469: u64, v2: Adt9, v3: List<Adt114>, p2470: List<Adt116>) -> R<List<Adt116>> {
    let m2467 = (p2468, (p2469, p2470));
    {
        let mut m2467 = m2467;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2467;
                let m2464 = v0;
                match m2464.uncons() {
                    None => {
                        Err::<(List<Adt9>, (u64, List<Adt116>)), R<List<Adt116>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt9>, (u64, List<Adt116>)), R<List<Adt116>>>((v6, ({
                            let a2465 = v1;
                            let a2466 = 1u64;
                            nat_add(a2465, a2466)?
                        }, List::cons(Adt116::C0(v2.clone(), v5, List::cons(Adt114::C1(v1), v3.clone())), v4))))
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

pub fn f726(p2473: List<Adt9>, v1: Adt9, v2: List<Adt114>, p2474: List<Adt116>) -> List<Adt116> {
    let m2472 = (p2473, p2474);
    {
        let mut m2472 = m2472;
        loop {
            let step = {
                let (v0, v3) = m2472;
                let m2471 = v0;
                match m2471.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt116>), List<Adt116>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt9>, List<Adt116>), List<Adt116>>((v5, List::cons(Adt116::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2472 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f727(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2475 = f819(v0)?;
    match m2475 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2476 = f819(v1)?;
            match m2476 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2477 = v2;
                    let a2478 = 61u64;
                    if nat_eq(a2477, a2478) {
                        f820(v5, v6, v4, v3)
                    } else {
                        let m2479 = f819(v2)?;
                        match m2479 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2480 = v3;
                                let a2481 = 61u64;
                                if nat_eq(a2480, a2481) {
                                    f821(v5, v6, v7, v4)
                                } else {
                                    let m2482 = f819(v3)?;
                                    match m2482 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f822(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f728(v0: List<u64>) -> bool {
    let m2483 = v0;
    match m2483.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f729(v0: Adt166) -> bool {
    let m2484 = v0;
    match m2484 {
        Adt166::C0 => {
            false
        }
        Adt166::C1 => {
            false
        }
        Adt166::C2 => {
            true
        }
    }
}

pub fn f730(p2489: List<u64>, p2490: u64, p2491: List<u64>) -> List<u64> {
    let m2488 = (p2489, (p2490, p2491));
    {
        let mut m2488 = m2488;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2488;
                let m2485 = v0;
                match m2485.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f823(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt167 = f824(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt167::C0(h2486, _) => {
                                h2486
                            }
                        }, match v5 {
                            Adt167::C0(_, h2487) => {
                                h2487
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f731(v0: Option<Adt45>) -> List<(Str, Adt9)> {
    let m2492 = v0;
    match m2492 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt9::C5({
                let a2498 = f825(match v1.clone() {
                    Adt45::C0(h2493, _, _) => {
                        h2493
                    }
                });
                let a2499 = {
                    let a2496 = f826(match v1.clone() {
                        Adt45::C0(_, h2494, _) => {
                            h2494
                        }
                    });
                    let a2497 = f733(Str::lit("lastModified"), match v1 {
                        Adt45::C0(_, _, h2495) => {
                            h2495
                        }
                    });
                    append_list(a2496, a2497)
                };
                append_list(a2498, a2499)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f732(v0: Option<Adt8>) -> List<(Str, Adt9)> {
    f827(Str::lit("_meta"), v0)
}

pub fn f733(v0: Str, v1: Option<Str>) -> List<(Str, Adt9)> {
    let m2500 = v1;
    match m2500 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C3(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f734(v0: Option<List<Adt5>>) -> List<(Str, Adt9)> {
    let m2501 = v0;
    match m2501 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt9::C4(f828(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f735(v0: Option<u64>) -> R<List<(Str, Adt9)>> {
    let m2502 = v0;
    match m2502 {
        None => {
            Ok(List::<(Str, Adt9)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f113(v1)?), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f736(v0: Adt51) -> Adt9 {
    let m2503 = v0;
    match m2503 {
        Adt51::C0(v1) => {
            Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(match v1.clone() {
                Adt52::C0(h2504, _, _, _) => {
                    h2504
                }
            })), {
                let a2508 = f733(Str::lit("mimeType"), match v1.clone() {
                    Adt52::C0(_, h2505, _, _) => {
                        h2505
                    }
                });
                let a2509 = List::cons((Str::lit("text"), Adt9::C3(match v1.clone() {
                    Adt52::C0(_, _, h2506, _) => {
                        h2506
                    }
                })), f732(match v1 {
                    Adt52::C0(_, _, _, h2507) => {
                        h2507
                    }
                }));
                append_list(a2508, a2509)
            }))
        }
        Adt51::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(match v2.clone() {
                Adt53::C0(h2510, _, _, _) => {
                    h2510
                }
            })), {
                let a2514 = f733(Str::lit("mimeType"), match v2.clone() {
                    Adt53::C0(_, h2511, _, _) => {
                        h2511
                    }
                });
                let a2515 = List::cons((Str::lit("blob"), Adt9::C3(match v2.clone() {
                    Adt53::C0(_, _, h2512, _) => {
                        h2512
                    }
                })), f732(match v2 {
                    Adt53::C0(_, _, _, h2513) => {
                        h2513
                    }
                }));
                append_list(a2514, a2515)
            }))
        }
    }
}

pub fn f737() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f738(v0: Adt9) -> Option<Adt69> {
    let m2516 = f281(v0);
    match m2516 {
        None => {
            None::<Adt69>
        }
        Some(v1) => {
            let m2517 = f566(Str::lit("method"), v1.clone());
            match m2517 {
                None => {
                    None::<Adt69>
                }
                Some(v2) => {
                    let a2518 = v2.clone();
                    let a2519 = Str::lit("elicitation/create");
                    if equal(a2518, a2519) {
                        let m2520 = f505(Str::lit("params"), v1);
                        match m2520 {
                            None => {
                                None::<Adt69>
                            }
                            Some(v3) => {
                                f829(v3)
                            }
                        }
                    } else {
                        let a2521 = v2;
                        let a2522 = Str::lit("roots/list");
                        if equal(a2521, a2522) {
                            f830(v1)
                        } else {
                            None::<Adt69>
                        }
                    }
                }
            }
        }
    }
}

pub fn f739(v0: u64) -> List<u64> {
    f689(v0)
}

pub fn f740(v0: Adt10) -> R<bool> {
    let a2540 = {
        let m2526 = {
            let a2524 = f831();
            let a2525 = match v0.clone() {
                Adt10::C0(h2523, _) => {
                    h2523
                }
            };
            compare(a2524, a2525)
        };
        if match m2526 {
            Ordering::Less => {
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
            let m2531 = {
                let a2529 = match v0.clone() {
                    Adt10::C0(h2527, _) => {
                        h2527
                    }
                };
                let a2530 = {
                    let a2528 = f831();
                    int_neg(a2528)?
                };
                compare(a2529, a2530)
            };
            if match m2531 {
                Ordering::Less => {
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
                let m2535 = {
                    let a2533 = 9999i64;
                    let a2534 = match v0.clone() {
                        Adt10::C0(_, h2532) => {
                            h2532
                        }
                    };
                    compare(a2533, a2534)
                };
                if match m2535 {
                    Ordering::Less => {
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
                    let m2539 = {
                        let a2537 = match v0 {
                            Adt10::C0(_, h2536) => {
                                h2536
                            }
                        };
                        let a2538 = -9999i64;
                        compare(a2537, a2538)
                    };
                    match m2539 {
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
    Ok(bool_not(a2540))
}

pub fn f741(v0: Adt10) -> R<List<u64>> {
    let m2544 = {
        let a2542 = match v0.clone() {
            Adt10::C0(_, h2541) => {
                h2541
            }
        };
        let a2543 = 0i64;
        compare(a2542, a2543)
    };
    if if match m2544 {
        Ordering::Less => {
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
        let m2548 = {
            let a2546 = 0i64;
            let a2547 = match v0.clone() {
                Adt10::C0(_, h2545) => {
                    h2545
                }
            };
            compare(a2546, a2547)
        };
        match m2548 {
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
        let a2551 = f832(match v0.clone() {
            Adt10::C0(h2549, _) => {
                h2549
            }
        })?;
        let a2552 = List::cons(101u64, f832(match v0 {
            Adt10::C0(_, h2550) => {
                h2550
            }
        })?);
        Ok(append_list(a2551, a2552))
    } else {
        f832(match v0 {
            Adt10::C0(h2553, _) => {
                h2553
            }
        })
    }
}

pub fn f742() -> u64 {
    512u64
}

pub fn f743(v0: u64, v1: Adt131, v2: List<Adt131>) -> List<Adt131> {
    List::cons(Adt131::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f744(p2556: List<u64>, p2557: List<u64>) -> R<List<u64>> {
    let m2555 = (p2556, p2557);
    {
        let mut m2555 = m2555;
        loop {
            let step = {
                let (v0, v1) = m2555;
                let m2554 = v0;
                match m2554.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f515(f833(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f745(v0: Adt38) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f834(match v0.clone() {
        Adt38::C0(_, h2558, _, _) => {
            h2558
        }
    })), List::cons((Str::lit("capabilities"), f835(match v0.clone() {
        Adt38::C0(_, _, h2559, _) => {
            h2559
        }
    })), {
        let a2562 = f733(Str::lit("instructions"), match v0.clone() {
            Adt38::C0(_, _, _, h2560) => {
                h2560
            }
        });
        let a2563 = f836(match v0 {
            Adt38::C0(h2561, _, _, _) => {
                h2561
            }
        })?;
        append_list(a2562, a2563)
    })))
}

pub fn f746(v0: Adt39) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("tools"), Adt9::C4(f837(match v0.clone() {
        Adt39::C0(_, _, h2564) => {
            h2564
        }
    }))), {
        let a2567 = f733(Str::lit("nextCursor"), match v0.clone() {
            Adt39::C0(_, h2565, _) => {
                h2565
            }
        });
        let a2568 = f836(match v0 {
            Adt39::C0(h2566, _, _) => {
                h2566
            }
        })?;
        append_list(a2567, a2568)
    }))
}

pub fn f747(v0: Adt56) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("resources"), Adt9::C4(f838(match v0.clone() {
        Adt56::C0(_, _, h2569) => {
            h2569
        }
    })?)), {
        let a2572 = f733(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h2570, _) => {
                h2570
            }
        });
        let a2573 = f836(match v0 {
            Adt56::C0(h2571, _, _) => {
                h2571
            }
        })?;
        append_list(a2572, a2573)
    }))
}

pub fn f748(v0: Adt57) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("contents"), Adt9::C4(f839(match v0.clone() {
        Adt57::C0(_, h2574) => {
            h2574
        }
    }))), f836(match v0 {
        Adt57::C0(h2575, _) => {
            h2575
        }
    })?))
}

pub fn f749(v0: Adt58) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt9::C4(f840(match v0.clone() {
        Adt58::C0(_, _, h2576) => {
            h2576
        }
    }))), {
        let a2579 = f733(Str::lit("nextCursor"), match v0.clone() {
            Adt58::C0(_, h2577, _) => {
                h2577
            }
        });
        let a2580 = f836(match v0 {
            Adt58::C0(h2578, _, _) => {
                h2578
            }
        })?;
        append_list(a2579, a2580)
    }))
}

pub fn f750(v0: Adt60) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("prompts"), Adt9::C4(f841(match v0.clone() {
        Adt60::C0(_, _, h2581) => {
            h2581
        }
    }))), {
        let a2584 = f733(Str::lit("nextCursor"), match v0.clone() {
            Adt60::C0(_, h2582, _) => {
                h2582
            }
        });
        let a2585 = f836(match v0 {
            Adt60::C0(h2583, _, _) => {
                h2583
            }
        })?;
        append_list(a2584, a2585)
    }))
}

pub fn f751(v0: Adt63) -> R<Option<List<(Str, Adt9)>>> {
    let m2587 = f842(match v0.clone() {
        Adt63::C0(_, h2586) => {
            h2586
        }
    })?;
    match m2587 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2589 = f733(Str::lit("description"), match v0 {
                    Adt63::C0(h2588, _) => {
                        h2588
                    }
                });
                let a2590 = List::cons((Str::lit("messages"), Adt9::C4(v1)), List::<(Str, Adt9)>::nil());
                append_list(a2589, a2590)
            }))
        }
    }
}

pub fn f752(v0: Adt65) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("completion"), Adt9::C5(List::cons((Str::lit("values"), Adt9::C4(f774({
        let m2593 = match v0.clone() {
            Adt65::C0(h2592) => {
                h2592
            }
        };
        match m2593 {
            Adt66::C0(h2591, _, _) => {
                h2591
            }
        }
    }))), {
        let a2600 = f843(Str::lit("total"), {
            let m2596 = match v0.clone() {
                Adt65::C0(h2595) => {
                    h2595
                }
            };
            match m2596 {
                Adt66::C0(_, h2594, _) => {
                    h2594
                }
            }
        })?;
        let a2601 = f391(Str::lit("hasMore"), {
            let m2599 = match v0 {
                Adt65::C0(h2598) => {
                    h2598
                }
            };
            match m2599 {
                Adt66::C0(_, _, h2597) => {
                    h2597
                }
            }
        });
        append_list(a2600, a2601)
    }))), List::<(Str, Adt9)>::nil()))
}

pub fn f753(v0: Adt86) -> R<Option<List<(Str, Adt9)>>> {
    let m2602 = f844(f845(v0.clone()), List::<(Str, Adt9)>::nil())?;
    match m2602 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2603 = f846(v1);
                let a2604 = f847(f848(v0));
                append_list(a2603, a2604)
            }))
        }
    }
}

pub fn f754(v0: Adt67) -> Option<List<(Str, Adt9)>> {
    let m2605 = f849(v0.clone());
    match m2605 {
        None => {
            None::<List<(Str, Adt9)>>
        }
        Some(v1) => {
            Some({
                let a2606 = f850(f425(v0.clone()), f851(f852(v0)));
                let a2607 = v1;
                append_list(a2606, a2607)
            })
        }
    }
}

pub fn f755(v0: Adt92) -> Option<List<(Str, Adt9)>> {
    let m2609 = match v0.clone() {
        Adt92::C0(_, h2608, _) => {
            h2608
        }
    };
    match m2609 {
        Adt93::C0 => {
            f754({
                let m2612 = match v0 {
                    Adt92::C0(h2611, _, _) => {
                        h2611
                    }
                };
                match m2612 {
                    Adt89::C0(h2610, _, _) => {
                        h2610
                    }
                }
            })
        }
        Adt93::C1 => {
            Some(List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f756(v0: Adt168) -> Str {
    let m2613 = v0;
    match m2613 {
        Adt168::C0 => {
            Str::lit("complete")
        }
        Adt168::C1 => {
            Str::lit("input_required")
        }
        Adt168::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f757(v0: Adt37) -> Adt168 {
    let m2614 = v0;
    match m2614 {
        Adt37::C0(_) => {
            Adt168::C0
        }
        Adt37::C1(_) => {
            Adt168::C0
        }
        Adt37::C2(_) => {
            Adt168::C0
        }
        Adt37::C3(_) => {
            Adt168::C0
        }
        Adt37::C4(_) => {
            Adt168::C0
        }
        Adt37::C5(_) => {
            Adt168::C0
        }
        Adt37::C6(_) => {
            Adt168::C0
        }
        Adt37::C7(_) => {
            Adt168::C0
        }
        Adt37::C8(_) => {
            Adt168::C0
        }
        Adt37::C9(_) => {
            Adt168::C0
        }
        Adt37::C10(_) => {
            Adt168::C0
        }
        Adt37::C11 => {
            Adt168::C0
        }
        Adt37::C12(_) => {
            Adt168::C1
        }
        Adt37::C13(_) => {
            Adt168::C2
        }
        Adt37::C14(_) => {
            Adt168::C0
        }
    }
}

pub fn f758(v0: Option<Adt4>) -> List<(Str, Adt9)> {
    let m2615 = v0;
    match m2615 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((f853(), f854(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f759(v0: List<Adt89>) -> R<Option<List<u64>>> {
    let m2616 = f855(v0);
    match m2616 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m2617 = f348(v1)?;
            match m2617 {
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

pub fn f760(v0: Adt89, v1: List<Adt89>, v2: i64) -> R<List<Adt89>> {
    Ok(List::cons(v0.clone(), f423(f856(f337(v0), v1, v2, List::<Adt89>::nil())?, List::<Adt89>::nil())))
}

pub fn f761(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f857(v2) {
            let a2618 = v1;
            let a2619 = 200u64;
            nat_eq(a2618, a2619)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f762(v0: u64, v1: List<List<u64>>) -> Adt104 {
    Adt104::C0(v0, f858(), List::<List<u64>>::nil(), f859(v1), None::<List<u64>>)
}

pub fn f763(v0: Adt149) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(f440())), List::cons((Str::lit("method"), Adt9::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt9::C5(List::cons((Str::lit("progressToken"), f860(match v0.clone() {
        Adt149::C0(h2620, _) => {
            h2620
        }
    })), List::cons((Str::lit("progress"), Adt9::C2({
        let m2623 = match v0.clone() {
            Adt149::C0(_, h2622) => {
                h2622
            }
        };
        match m2623 {
            Adt55::C0(h2621, _, _) => {
                h2621
            }
        }
    })), {
        let a2630 = f861({
            let m2626 = match v0.clone() {
                Adt149::C0(_, h2625) => {
                    h2625
                }
            };
            match m2626 {
                Adt55::C0(_, h2624, _) => {
                    h2624
                }
            }
        });
        let a2631 = f862({
            let m2629 = match v0 {
                Adt149::C0(_, h2628) => {
                    h2628
                }
            };
            match m2629 {
                Adt55::C0(_, _, h2627) => {
                    h2627
                }
            }
        });
        append_list(a2630, a2631)
    })))), List::<(Str, Adt9)>::nil()))))
}

pub fn f764(v0: Adt92) -> bool {
    if match v0.clone() {
        Adt92::C0(_, _, h2632) => {
            h2632
        }
    } {
        let m2634 = match v0 {
            Adt92::C0(_, h2633, _) => {
                h2633
            }
        };
        match m2634 {
            Adt93::C0 => {
                true
            }
            Adt93::C1 => {
                true
            }
        }
    } else {
        false
    }
}

pub fn f765(v0: Adt67) -> Option<Adt9> {
    let m2635 = f754(v0);
    match m2635 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt9::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt9::C5(v1)), List::<(Str, Adt9)>::nil())))))
        }
    }
}

pub fn f766() -> u64 {
    429u64
}

pub fn f767() -> i64 {
    -32602i64
}

pub fn f768() -> i64 {
    -32022i64
}

pub fn f769() -> i64 {
    -32020i64
}

pub fn f770() -> i64 {
    -32601i64
}

pub fn f771(v0: Adt94) -> i64 {
    let m2636 = v0;
    match m2636 {
        Adt94::C0(_) => {
            f767()
        }
        Adt94::C1(_) => {
            f767()
        }
        Adt94::C2(_) => {
            f767()
        }
        Adt94::C3(_) => {
            f767()
        }
        Adt94::C4(_) => {
            f767()
        }
        Adt94::C5(_) => {
            f767()
        }
        Adt94::C6(_) => {
            f863()
        }
        Adt94::C7(_) => {
            f864()
        }
        Adt94::C8(_) => {
            f865()
        }
        Adt94::C9(_) => {
            f863()
        }
        Adt94::C10 => {
            f767()
        }
        Adt94::C11 => {
            f767()
        }
        Adt94::C12(_) => {
            f863()
        }
        Adt94::C13 => {
            f863()
        }
        Adt94::C14 => {
            f863()
        }
        Adt94::C15(_) => {
            f863()
        }
    }
}

pub fn f772(v0: Adt151) -> Str {
    let m2637 = v0;
    match m2637 {
        Adt151::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt151::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt151::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt151::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt151::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt151::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt151::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt151::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt151::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f773(v0: Adt133) -> Str {
    let m2638 = v0;
    match m2638 {
        Adt133::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt133::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt133::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt133::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt133::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt133::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt133::C6(v1) => {
            let m2639 = v1;
            match m2639 {
                Adt134::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt134::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt134::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt133::C7(v2) => {
            let m2640 = v2;
            match m2640 {
                Adt134::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt134::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt134::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt133::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt133::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt133::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f774(v0: List<Str>) -> List<Adt9> {
    f490(f866(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f775(v0: Adt22) -> Adt9 {
    Adt9::C5({
        let a2652 = f827(Str::lit("experimental"), match v0.clone() {
            Adt22::C0(h2641, _, _, _, _) => {
                h2641
            }
        });
        let a2653 = {
            let a2650 = f827(Str::lit("roots"), match v0.clone() {
                Adt22::C0(_, h2642, _, _, _) => {
                    h2642
                }
            });
            let a2651 = {
                let a2648 = f827(Str::lit("sampling"), match v0.clone() {
                    Adt22::C0(_, _, h2643, _, _) => {
                        h2643
                    }
                });
                let a2649 = {
                    let a2646 = f827(Str::lit("elicitation"), match v0.clone() {
                        Adt22::C0(_, _, _, h2644, _) => {
                            h2644
                        }
                    });
                    let a2647 = f827(Str::lit("extensions"), match v0 {
                        Adt22::C0(_, _, _, _, h2645) => {
                            h2645
                        }
                    });
                    append_list(a2646, a2647)
                };
                append_list(a2648, a2649)
            };
            append_list(a2650, a2651)
        };
        append_list(a2652, a2653)
    })
}

pub fn f776(p2656: List<List<u64>>) -> R<bool> {
    let m2655 = p2656;
    {
        let mut m2655 = m2655;
        loop {
            let step = {
                let v0 = m2655;
                let m2654 = v0;
                match m2654.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f867(f455(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
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

pub fn f777() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f778(v0: Adt3, v1: Fn9, v2: u64, v3: Adt158, v4: bool) -> R<Result<Adt147, Adt150>> {
    if v4 {
        f868(v0, v1, v2, f869(match v3.clone() {
            Adt158::C0(_, h2657, _, _) => {
                h2657
            }
        }, match v3.clone() {
            Adt158::C0(_, _, h2658, _) => {
                h2658
            }
        }, match v3 {
            Adt158::C0(_, _, _, h2659) => {
                h2659
            }
        }))
    } else {
        Ok(Err::<Adt147, Adt150>(Adt150::C3(f870(match v3 {
            Adt158::C0(_, h2660, _, _) => {
                h2660
            }
        }))))
    }
}

pub fn f779(v0: Adt7, v1: Adt159) -> bool {
    let m2661 = v1;
    match m2661 {
        Adt159::C0 => {
            true
        }
        Adt159::C1 => {
            f871(match v0 {
                Adt7::C0(_, _, _, _, _, h2662, _) => {
                    h2662
                }
            })
        }
        Adt159::C2 => {
            f871(match v0 {
                Adt7::C0(_, _, _, _, _, h2663, _) => {
                    h2663
                }
            })
        }
        Adt159::C3 => {
            f872(match v0 {
                Adt7::C0(_, _, _, _, h2664, _, _) => {
                    h2664
                }
            })
        }
        Adt159::C4 => {
            f872(match v0 {
                Adt7::C0(_, _, _, _, h2665, _, _) => {
                    h2665
                }
            })
        }
        Adt159::C5 => {
            f872(match v0 {
                Adt7::C0(_, _, _, _, h2666, _, _) => {
                    h2666
                }
            })
        }
        Adt159::C6 => {
            f871(match v0 {
                Adt7::C0(_, _, _, h2667, _, _, _) => {
                    h2667
                }
            })
        }
        Adt159::C7 => {
            f871(match v0 {
                Adt7::C0(_, _, _, h2668, _, _, _) => {
                    h2668
                }
            })
        }
        Adt159::C8 => {
            f401(match v0 {
                Adt7::C0(_, _, h2669, _, _, _, _) => {
                    h2669
                }
            })
        }
        Adt159::C9 => {
            f334(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2670) => {
                    h2670
                }
            }, f33())
        }
        Adt159::C10 => {
            f334(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2671) => {
                    h2671
                }
            }, f33())
        }
        Adt159::C11 => {
            f334(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2672) => {
                    h2672
                }
            }, f33())
        }
    }
}

pub fn f780(v0: Adt163, v1: Adt161) -> Result<Adt158, Adt150> {
    let m2673 = v1;
    match m2673 {
        Adt161::C0 => {
            Err::<Adt158, Adt150>(Adt150::C3(match v0 {
                Adt163::C0(_, h2674, _, _) => {
                    h2674
                }
            }))
        }
        Adt161::C1(v2) => {
            Ok::<Adt158, Adt150>(Adt158::C0(match v0.clone() {
                Adt163::C0(h2675, _, _, _) => {
                    h2675
                }
            }, v2, match v0.clone() {
                Adt163::C0(_, _, h2676, _) => {
                    h2676
                }
            }, match v0 {
                Adt163::C0(_, _, _, h2677) => {
                    h2677
                }
            }))
        }
    }
}

pub fn f781(v0: Option<Adt132>, v1: List<Str>, v2: Adt127, v3: Result<Adt21, Adt151>) -> R<Adt162> {
    let m2678 = v3;
    match m2678 {
        Ok(v5) => {
            Ok(f873(v1, v2.clone(), v5.clone(), f874(v0, f671(match v2.clone() {
                Adt127::C0(_, h2679, _) => {
                    h2679
                }
            }), match v2.clone() {
                Adt127::C0(_, h2680, _) => {
                    h2680
                }
            }, match v5 {
                Adt21::C0(h2681, _, _, _, _, _) => {
                    h2681
                }
            }, f875(match v2 {
                Adt127::C0(_, _, h2682) => {
                    h2682
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt162::C1(Adt150::C0(v4)))
        }
    }
}

pub fn f782(v0: Option<Adt8>) -> Result<Adt21, Adt151> {
    let m2683 = v0;
    match m2683 {
        None => {
            Err::<Adt21, Adt151>(Adt151::C0)
        }
        Some(v1) => {
            let m2684 = f275(Str::lit("_meta"), v1);
            match m2684 {
                Adt118::C0 => {
                    Err::<Adt21, Adt151>(Adt151::C1)
                }
                Adt118::C1(v2) => {
                    let m2685 = f281(v2);
                    match m2685 {
                        None => {
                            Err::<Adt21, Adt151>(Adt151::C4)
                        }
                        Some(v3) => {
                            f876(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f783(v0: Adt159) -> Adt164 {
    let m2686 = v0;
    match m2686 {
        Adt159::C0 => {
            Adt164::C0
        }
        Adt159::C1 => {
            Adt164::C0
        }
        Adt159::C2 => {
            Adt164::C1
        }
        Adt159::C3 => {
            Adt164::C0
        }
        Adt159::C4 => {
            Adt164::C2
        }
        Adt159::C5 => {
            Adt164::C0
        }
        Adt159::C6 => {
            Adt164::C0
        }
        Adt159::C7 => {
            Adt164::C1
        }
        Adt159::C8 => {
            Adt164::C0
        }
        Adt159::C9 => {
            Adt164::C3
        }
        Adt159::C10 => {
            Adt164::C3
        }
        Adt159::C11 => {
            Adt164::C3
        }
    }
}

pub fn f784(p2692: List<u64>, p2693: List<u64>) -> Option<List<u64>> {
    let m2691 = (p2692, p2693);
    {
        let mut m2691 = m2691;
        loop {
            let step = {
                let (v0, v1) = m2691;
                let m2687 = v0;
                match m2687.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2688 = v1;
                        match m2688.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2689 = v4;
                                let a2690 = v2;
                                if nat_eq(a2689, a2690) {
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
                Ok(next) => { m2691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f785() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f786() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f787(v0: Adt15, v1: Option<Str>) -> bool {
    let m2694 = v1;
    match m2694 {
        None => {
            false
        }
        Some(v2) => {
            let a2696 = match v0 {
                Adt15::C0(h2695, _, _, _) => {
                    h2695
                }
            };
            let a2697 = v2;
            equal(a2696, a2697)
        }
    }
}

pub fn f788(v0: List<Adt101>, v1: Str) -> R<Adt136> {
    let m2700 = f591(f257(v0, {
        let a2698 = f877();
        let a2699 = f54(v1)?;
        append_list(a2698, a2699)
    })?)?;
    match m2700 {
        Adt123::C0 => {
            Ok(Adt136::C0)
        }
        Adt123::C1 => {
            Ok(Adt136::C1)
        }
        Adt123::C2 => {
            Ok(Adt136::C1)
        }
        Adt123::C3(v2) => {
            let m2701 = f436(v2);
            match m2701 {
                None => {
                    Ok(Adt136::C1)
                }
                Some(v3) => {
                    Ok(Adt136::C2(v3))
                }
            }
        }
    }
}

pub fn f789(v0: Adt139, v1: u64, v2: Str) -> Adt154 {
    let m2703 = match v0.clone() {
        Adt139::C0(_, h2702, _) => {
            h2702
        }
    };
    match m2703.uncons() {
        None => {
            f609(Adt120::C2(v1))
        }
        Some((v3, v4)) => {
            let m2704 = v3;
            match m2704 {
                Adt144::C0(_) => {
                    f609(Adt120::C2(v1))
                }
                Adt144::C1(v6, _) => {
                    Adt154::C1(Adt139::C0(Adt140::C5, List::cons(Adt144::C1(v6, Some(v2)), v4), match v0 {
                        Adt139::C0(_, _, h2705) => {
                            h2705
                        }
                    }))
                }
            }
        }
    }
}

pub fn f790(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2706 = v0;
    let a2707 = 128u64;
    if nat_lt(a2706, a2707) {
        Ok(List::cons(v0, v1))
    } else {
        let a2708 = v0;
        let a2709 = 2048u64;
        if nat_lt(a2708, a2709) {
            Ok(List::cons({
                let a2710 = 128u64;
                let a2711 = f878(v0, 64u64);
                nat_add(a2710, a2711)?
            }, List::cons({
                let a2712 = 192u64;
                let a2713 = f879(v0, 64u64);
                nat_add(a2712, a2713)?
            }, v1)))
        } else {
            let a2714 = v0;
            let a2715 = 65536u64;
            if nat_lt(a2714, a2715) {
                Ok(List::cons({
                    let a2716 = 128u64;
                    let a2717 = f878(v0, 64u64);
                    nat_add(a2716, a2717)?
                }, List::cons({
                    let a2718 = 128u64;
                    let a2719 = f878(f879(v0, 64u64), 64u64);
                    nat_add(a2718, a2719)?
                }, List::cons({
                    let a2720 = 224u64;
                    let a2721 = f879(v0, 4096u64);
                    nat_add(a2720, a2721)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2722 = 128u64;
                    let a2723 = f878(v0, 64u64);
                    nat_add(a2722, a2723)?
                }, List::cons({
                    let a2724 = 128u64;
                    let a2725 = f878(f879(v0, 64u64), 64u64);
                    nat_add(a2724, a2725)?
                }, List::cons({
                    let a2726 = 128u64;
                    let a2727 = f878(f879(v0, 4096u64), 64u64);
                    nat_add(a2726, a2727)?
                }, List::cons({
                    let a2728 = 240u64;
                    let a2729 = f879(v0, 262144u64);
                    nat_add(a2728, a2729)?
                }, v1)))))
            }
        }
    }
}

pub fn f791() -> u64 {
    18u64
}

pub fn f792() -> u64 {
    4294967296u64
}

pub fn f793(v0: Adt104) -> Adt104 {
    Adt104::C0(match v0.clone() {
        Adt104::C0(h2730, _, _, _, _) => {
            h2730
        }
    }, match v0.clone() {
        Adt104::C0(_, h2731, _, _, _) => {
            h2731
        }
    }, match v0.clone() {
        Adt104::C0(_, _, h2732, _, _) => {
            h2732
        }
    }, match v0 {
        Adt104::C0(_, _, _, h2733, _) => {
            h2733
        }
    }, None::<List<u64>>)
}

pub fn f794(v0: Adt3, v1: List<u64>) -> Adt95 {
    Adt95::C0(match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2734) => {
            h2734
        }
    }, f493(f243(), {
        let a2736 = f28({
            let a2735 = v1.clone();
            length_list(a2735)
        });
        let a2737 = v1;
        append_list(a2736, a2737)
    }))
}

pub fn f795() -> Adt97 {
    Adt97::C0(f56(f57()))
}

pub fn f796(v0: Adt9) -> bool {
    let m2738 = f181(v0);
    match m2738 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f797(v0: Adt9) -> bool {
    let m2739 = f713(v0);
    match m2739 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f798(v0: Adt9) -> bool {
    let m2740 = f277(v0);
    match m2740 {
        None => {
            false
        }
        Some(v1) => {
            let a2744 = {
                let m2743 = {
                    let a2741 = v1;
                    let a2742 = 0i64;
                    compare(a2741, a2742)
                };
                match m2743 {
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
            bool_not(a2744)
        }
    }
}

pub fn f799(v0: Adt9) -> bool {
    let m2745 = f335(v0);
    match m2745 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f800(v0: Adt9) -> bool {
    let m2746 = f281(v0);
    match m2746 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f801(v0: Adt9) -> bool {
    let m2747 = f335(v0);
    match m2747 {
        None => {
            false
        }
        Some(v1) => {
            f880(v1)
        }
    }
}

pub fn f802(v0: Adt9) -> bool {
    let m2748 = f468(v0);
    match m2748 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f803(v0: Adt9) -> bool {
    let m2749 = f181(v0.clone());
    match m2749 {
        None => {
            let m2750 = f335(v0);
            match m2750 {
                None => {
                    false
                }
                Some(v2) => {
                    f881(v2)
                }
            }
        }
        Some(v1) => {
            f708(v1)
        }
    }
}

pub fn f804(v0: Adt9) -> bool {
    let m2751 = f335(v0);
    match m2751 {
        None => {
            false
        }
        Some(v1) => {
            let m2752 = v1;
            match m2752.uncons() {
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

pub fn f805(v0: Str, p2770: List<(Str, Adt9)>) -> Option<Adt9> {
    let m2769 = p2770;
    {
        let mut m2769 = m2769;
        loop {
            let step = {
                let v1 = m2769;
                let m2753 = v1;
                match m2753.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, Option<Adt9>>(None::<Adt9>)
                    }
                    Some((v2, v3)) => {
                        let a2757 = v0.clone();
                        let a2758 = {
                            let a2755 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2754, _) = v2.clone();
                                h2754
                            }, List::<Str>::nil()));
                            let a2756 = Str::lit("");
                            join(a2755, a2756)
                        };
                        if equal(a2757, a2758) {
                            Err::<List<(Str, Adt9)>, Option<Adt9>>(Some({
                                let (_, h2759) = v2;
                                h2759
                            }))
                        } else {
                            let m2761 = f281({
                                let (_, h2760) = v2.clone();
                                h2760
                            });
                            match m2761 {
                                None => {
                                    Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                }
                                Some(v4) => {
                                    let m2762 = f275(Str::lit("$anchor"), v4);
                                    match m2762 {
                                        Adt118::C0 => {
                                            Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                        }
                                        Adt118::C1(v5) => {
                                            let m2763 = f181(v5);
                                            match m2763 {
                                                None => {
                                                    Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2766 = v0.clone();
                                                    let a2767 = {
                                                        let a2764 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2765 = Str::lit("");
                                                        join(a2764, a2765)
                                                    };
                                                    if equal(a2766, a2767) {
                                                        Err::<List<(Str, Adt9)>, Option<Adt9>>(Some({
                                                            let (_, h2768) = v2;
                                                            h2768
                                                        }))
                                                    } else {
                                                        Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                                    }
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
                Ok(next) => { m2769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f806(v0: List<(Str, Adt9)>) -> Option<Adt16> {
    let m2771 = f383(Str::lit("type"), v0);
    match m2771 {
        Adt118::C0 => {
            None::<Adt16>
        }
        Adt118::C1(v1) => {
            let m2772 = f181(v1);
            match m2772 {
                None => {
                    None::<Adt16>
                }
                Some(v2) => {
                    f882(v2)
                }
            }
        }
    }
}

pub fn f807(v0: List<u64>) -> R<List<u64>> {
    Ok(f515(f883(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f808(v0: List<u64>, p2775: List<List<u64>>) -> bool {
    let m2774 = p2775;
    {
        let mut m2774 = m2774;
        loop {
            let step = {
                let v1 = m2774;
                let m2773 = v1;
                match m2773.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f884(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f809(v0: Adt9) -> Str {
    let m2776 = v0;
    match m2776 {
        Adt9::C0 => {
            Str::lit("null")
        }
        Adt9::C1(_) => {
            Str::lit("boolean")
        }
        Adt9::C2(_) => {
            Str::lit("number")
        }
        Adt9::C3(_) => {
            Str::lit("string")
        }
        Adt9::C4(_) => {
            Str::lit("array")
        }
        Adt9::C5(_) => {
            Str::lit("object")
        }
    }
}

pub fn f810(v0: Adt10) -> R<bool> {
    if f387(match v0.clone() {
        Adt10::C0(h2777, _) => {
            h2777
        }
    }) {
        Ok(true)
    } else {
        f885(40u64, match v0.clone() {
            Adt10::C0(h2778, _) => {
                h2778
            }
        }, match v0 {
            Adt10::C0(_, h2779) => {
                h2779
            }
        })
    }
}

pub fn f811(v0: Adt9) -> Str {
    let m2780 = f181(v0);
    match m2780 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f812(p2787: u64, p2788: List<(Adt9, Adt9)>) -> R<Adt165> {
    let m2786 = (p2787, p2788);
    {
        let mut m2786 = m2786;
        loop {
            let step = {
                let (v0, v1) = m2786;
                let m2781 = v0;
                if m2781 == 0 {
                    Err::<(u64, List<(Adt9, Adt9)>), R<Adt165>>(Ok(Adt165::C2))
                } else {
                    let v2 = m2781 - 1;
                    let m2782 = v1;
                    match m2782.uncons() {
                        None => {
                            Err::<(u64, List<(Adt9, Adt9)>), R<Adt165>>(Ok(Adt165::C0))
                        }
                        Some((v3, v4)) => {
                            let m2785 = f886({
                                let (h2783, _) = v3;
                                h2783
                            }, {
                                let (_, h2784) = v3;
                                h2784
                            }, v4)?;
                            match m2785 {
                                Adt169::C0 => {
                                    Err::<(u64, List<(Adt9, Adt9)>), R<Adt165>>(Ok(Adt165::C1))
                                }
                                Adt169::C1(v5) => {
                                    Ok::<(u64, List<(Adt9, Adt9)>), R<Adt165>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2786 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f813() -> u64 {
    67108864u64
}

pub fn f814(v0: Adt166) -> bool {
    let m2789 = v0;
    match m2789 {
        Adt166::C0 => {
            true
        }
        Adt166::C1 => {
            false
        }
        Adt166::C2 => {
            false
        }
    }
}

pub fn f815(v0: Adt10, v1: Adt10, v2: Adt166, v3: Adt166) -> R<Adt166> {
    let m2790 = v2;
    match m2790 {
        Adt166::C0 => {
            let m2792 = v3;
            match m2792 {
                Adt166::C0 => {
                    Ok(f887(Adt166::C0, f888(f889(match v0.clone() {
                        Adt10::C0(h2793, _) => {
                            h2793
                        }
                    })?, match v0 {
                        Adt10::C0(_, h2794) => {
                            h2794
                        }
                    }, f889(match v1.clone() {
                        Adt10::C0(h2795, _) => {
                            h2795
                        }
                    })?, match v1 {
                        Adt10::C0(_, h2796) => {
                            h2796
                        }
                    })?))
                }
                Adt166::C1 => {
                    Ok(Adt166::C0)
                }
                Adt166::C2 => {
                    Ok(Adt166::C0)
                }
            }
        }
        Adt166::C1 => {
            let m2791 = v3;
            match m2791 {
                Adt166::C0 => {
                    Ok(Adt166::C2)
                }
                Adt166::C1 => {
                    Ok(Adt166::C1)
                }
                Adt166::C2 => {
                    Ok(Adt166::C0)
                }
            }
        }
        Adt166::C2 => {
            let m2797 = v3;
            match m2797 {
                Adt166::C0 => {
                    Ok(Adt166::C2)
                }
                Adt166::C1 => {
                    Ok(Adt166::C2)
                }
                Adt166::C2 => {
                    f888(match v0.clone() {
                        Adt10::C0(h2798, _) => {
                            h2798
                        }
                    }, match v0 {
                        Adt10::C0(_, h2799) => {
                            h2799
                        }
                    }, match v1.clone() {
                        Adt10::C0(h2800, _) => {
                            h2800
                        }
                    }, match v1 {
                        Adt10::C0(_, h2801) => {
                            h2801
                        }
                    })
                }
            }
        }
    }
}

pub fn f816(v0: i64) -> Adt166 {
    f817(v0, 0i64)
}

pub fn f817(v0: i64, v1: i64) -> Adt166 {
    let m2804 = {
        let a2802 = v0;
        let a2803 = v1;
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
        Adt166::C0
    } else {
        let m2807 = {
            let a2805 = v1;
            let a2806 = v0;
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
            Adt166::C2
        } else {
            Adt166::C1
        }
    }
}

pub fn f818(p2816: List<u64>, p2817: u64) -> R<u64> {
    let m2815 = (p2816, p2817);
    {
        let mut m2815 = m2815;
        loop {
            let step = {
                let (v0, v1) = m2815;
                let m2808 = v0;
                match m2808.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2809 = v2;
                        let a2810 = 128u64;
                        if if nat_lt(a2809, a2810) {
                            true
                        } else {
                            let a2811 = 191u64;
                            let a2812 = v2;
                            nat_lt(a2811, a2812)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2813 = v1;
                                let a2814 = 1u64;
                                nat_add(a2813, a2814)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f819(v0: u64) -> R<Option<u64>> {
    let a2818 = 64u64;
    let a2819 = v0;
    if if nat_lt(a2818, a2819) {
        let a2820 = v0;
        let a2821 = 91u64;
        nat_lt(a2820, a2821)
    } else {
        false
    } {
        Ok(Some({
            let a2822 = v0;
            let a2823 = 65u64;
            nat_sub(a2822, a2823)
        }))
    } else {
        let a2824 = 96u64;
        let a2825 = v0;
        if if nat_lt(a2824, a2825) {
            let a2826 = v0;
            let a2827 = 123u64;
            nat_lt(a2826, a2827)
        } else {
            false
        } {
            Ok(Some({
                let a2828 = v0;
                let a2829 = 71u64;
                nat_sub(a2828, a2829)
            }))
        } else {
            let a2830 = 47u64;
            let a2831 = v0;
            if if nat_lt(a2830, a2831) {
                let a2832 = v0;
                let a2833 = 58u64;
                nat_lt(a2832, a2833)
            } else {
                false
            } {
                Ok(Some({
                    let a2834 = v0;
                    let a2835 = 4u64;
                    nat_add(a2834, a2835)?
                }))
            } else {
                let a2836 = v0;
                let a2837 = 43u64;
                if nat_eq(a2836, a2837) {
                    Ok(Some(62u64))
                } else {
                    let a2838 = v0;
                    let a2839 = 47u64;
                    if nat_eq(a2838, a2839) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f820(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2840 = v3;
        let a2841 = 61u64;
        if nat_eq(a2840, a2841) {
            let a2842 = f890(v1, 16u64);
            let a2843 = 0u64;
            nat_eq(a2842, a2843)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f891(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f821(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2844 = f890(v2, 4u64);
        let a2845 = 0u64;
        nat_eq(a2844, a2845)
    } else {
        false
    } {
        Ok(Some(f892(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f822(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2848 = {
            let a2846 = v0;
            let a2847 = 4u64;
            nat_mul(a2846, a2847)?
        };
        let a2849 = f893(v1, 16u64);
        nat_add(a2848, a2849)?
    }, List::cons({
        let a2852 = {
            let a2850 = f890(v1, 16u64);
            let a2851 = 16u64;
            nat_mul(a2850, a2851)?
        };
        let a2853 = f893(v2, 4u64);
        nat_add(a2852, a2853)?
    }, List::cons({
        let a2856 = {
            let a2854 = f890(v2, 4u64);
            let a2855 = 64u64;
            nat_mul(a2854, a2855)?
        };
        let a2857 = v3;
        nat_add(a2856, a2857)?
    }, List::<u64>::nil()))))
}

pub fn f823(v0: u64, v1: List<u64>) -> List<u64> {
    let a2858 = v0;
    let a2859 = 0u64;
    if nat_eq(a2858, a2859) {
        v1
    } else {
        let a2860 = v0;
        let a2861 = 1u64;
        if nat_eq(a2860, a2861) {
            List::cons(194u64, v1)
        } else {
            let a2862 = v0;
            let a2863 = 2u64;
            if nat_eq(a2862, a2863) {
                List::cons(226u64, v1)
            } else {
                let a2864 = v0;
                let a2865 = 3u64;
                if nat_eq(a2864, a2865) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f824(v0: u64, v1: u64, v2: List<u64>) -> Adt167 {
    let a2866 = v0;
    let a2867 = 0u64;
    if nat_eq(a2866, a2867) {
        f894(v1, v2)
    } else {
        let a2868 = v0;
        let a2869 = 1u64;
        if nat_eq(a2868, a2869) {
            let a2870 = 128u64;
            let a2871 = v1;
            if if nat_le(a2870, a2871) {
                let a2872 = v1;
                let a2873 = 159u64;
                nat_le(a2872, a2873)
            } else {
                false
            } {
                Adt167::C0(0u64, f895(v2))
            } else {
                f894(v1, List::cons(194u64, v2))
            }
        } else {
            let a2874 = v0;
            let a2875 = 2u64;
            if nat_eq(a2874, a2875) {
                let a2876 = v1;
                let a2877 = 128u64;
                if nat_eq(a2876, a2877) {
                    Adt167::C0(3u64, v2)
                } else {
                    let a2878 = v1;
                    let a2879 = 129u64;
                    if nat_eq(a2878, a2879) {
                        Adt167::C0(4u64, v2)
                    } else {
                        f894(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2880 = v0;
                let a2881 = 3u64;
                if nat_eq(a2880, a2881) {
                    let a2882 = 170u64;
                    let a2883 = v1;
                    if if nat_le(a2882, a2883) {
                        let a2884 = v1;
                        let a2885 = 174u64;
                        nat_le(a2884, a2885)
                    } else {
                        false
                    } {
                        Adt167::C0(0u64, f895(v2))
                    } else {
                        f894(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2886 = 166u64;
                    let a2887 = v1;
                    if if nat_le(a2886, a2887) {
                        let a2888 = v1;
                        let a2889 = 169u64;
                        nat_le(a2888, a2889)
                    } else {
                        false
                    } {
                        Adt167::C0(0u64, f895(v2))
                    } else {
                        f894(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f825(v0: Option<List<Adt46>>) -> List<(Str, Adt9)> {
    let m2890 = v0;
    match m2890 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt9::C4(f896(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f826(v0: Option<Adt10>) -> List<(Str, Adt9)> {
    let m2891 = v0;
    match m2891 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt9::C2(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f827(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt9)> {
    let m2892 = v1;
    match m2892 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C5(match v2 {
                Adt8::C0(h2893) => {
                    h2893
                }
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f828(v0: List<Adt5>) -> List<Adt9> {
    f490(f897(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f829(v0: Adt8) -> Option<Adt69> {
    let m2894 = f566(Str::lit("message"), v0.clone());
    match m2894 {
        None => {
            None::<Adt69>
        }
        Some(v1) => {
            let m2895 = f275(Str::lit("mode"), v0.clone());
            match m2895 {
                Adt118::C0 => {
                    let m2896 = f505(Str::lit("requestedSchema"), v0);
                    match m2896 {
                        None => {
                            None::<Adt69>
                        }
                        Some(v2) => {
                            Some(Adt69::C0(Adt70::C0(Adt71::C0(None::<Adt72>, v1, v2))))
                        }
                    }
                }
                Adt118::C1(v3) => {
                    let m2897 = f181(v3);
                    match m2897 {
                        None => {
                            None::<Adt69>
                        }
                        Some(v4) => {
                            let a2898 = v4.clone();
                            let a2899 = Str::lit("url");
                            if equal(a2898, a2899) {
                                let m2900 = f566(Str::lit("url"), v0);
                                match m2900 {
                                    None => {
                                        None::<Adt69>
                                    }
                                    Some(v5) => {
                                        Some(Adt69::C0(Adt70::C1(Adt73::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a2901 = v4;
                                let a2902 = Str::lit("form");
                                if equal(a2901, a2902) {
                                    let m2903 = f505(Str::lit("requestedSchema"), v0);
                                    match m2903 {
                                        None => {
                                            None::<Adt69>
                                        }
                                        Some(v6) => {
                                            Some(Adt69::C0(Adt70::C0(Adt71::C0(Some(Adt72::C0), v1, v6))))
                                        }
                                    }
                                } else {
                                    None::<Adt69>
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f830(v0: Adt8) -> Option<Adt69> {
    let m2904 = f275(Str::lit("params"), v0);
    match m2904 {
        Adt118::C0 => {
            Some(Adt69::C2(None::<Adt8>))
        }
        Adt118::C1(v1) => {
            let m2905 = f281(v1);
            match m2905 {
                None => {
                    None::<Adt69>
                }
                Some(v2) => {
                    Some(Adt69::C2(Some(v2)))
                }
            }
        }
    }
}

pub fn f831() -> i64 {
    999999999999999999i64
}

pub fn f832(v0: i64) -> R<List<u64>> {
    let m2908 = {
        let a2906 = v0;
        let a2907 = 0i64;
        compare(a2906, a2907)
    };
    if match m2908 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f898(20u64, {
            let a2909 = v0;
            int_neg(a2909)?
        }, List::<u64>::nil())?))
    } else {
        f898(20u64, v0, List::<u64>::nil())
    }
}

pub fn f833(v0: u64) -> R<List<u64>> {
    let a2910 = v0;
    let a2911 = 34u64;
    if nat_eq(a2910, a2911) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2912 = v0;
        let a2913 = 92u64;
        if nat_eq(a2912, a2913) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2914 = v0;
            let a2915 = 8u64;
            if nat_eq(a2914, a2915) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2916 = v0;
                let a2917 = 9u64;
                if nat_eq(a2916, a2917) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2918 = v0;
                    let a2919 = 10u64;
                    if nat_eq(a2918, a2919) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2920 = v0;
                        let a2921 = 12u64;
                        if nat_eq(a2920, a2921) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2922 = v0;
                            let a2923 = 13u64;
                            if nat_eq(a2922, a2923) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2924 = v0;
                                let a2925 = 32u64;
                                if nat_lt(a2924, a2925) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f899(f879(v0, 16u64))?, List::cons(f899(f878(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f834(v0: List<Str>) -> Adt9 {
    Adt9::C4(f774(v0))
}

pub fn f835(v0: Adt7) -> Adt9 {
    Adt9::C5({
        let a2943 = f827(Str::lit("experimental"), match v0.clone() {
            Adt7::C0(h2926, _, _, _, _, _, _) => {
                h2926
            }
        });
        let a2944 = {
            let a2941 = f827(Str::lit("logging"), match v0.clone() {
                Adt7::C0(_, h2927, _, _, _, _, _) => {
                    h2927
                }
            });
            let a2942 = {
                let a2939 = f827(Str::lit("completions"), match v0.clone() {
                    Adt7::C0(_, _, h2928, _, _, _, _) => {
                        h2928
                    }
                });
                let a2940 = {
                    let a2937 = f900(Str::lit("prompts"), match v0.clone() {
                        Adt7::C0(_, _, _, h2929, _, _, _) => {
                            h2929
                        }
                    });
                    let a2938 = {
                        let a2935 = f901(match v0.clone() {
                            Adt7::C0(_, _, _, _, h2930, _, _) => {
                                h2930
                            }
                        });
                        let a2936 = {
                            let a2933 = f900(Str::lit("tools"), match v0.clone() {
                                Adt7::C0(_, _, _, _, _, h2931, _) => {
                                    h2931
                                }
                            });
                            let a2934 = f827(Str::lit("extensions"), match v0 {
                                Adt7::C0(_, _, _, _, _, _, h2932) => {
                                    h2932
                                }
                            });
                            append_list(a2933, a2934)
                        };
                        append_list(a2935, a2936)
                    };
                    append_list(a2937, a2938)
                };
                append_list(a2939, a2940)
            };
            append_list(a2941, a2942)
        };
        append_list(a2943, a2944)
    })
}

pub fn f836(v0: Adt13) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("ttlMs"), f113(match v0.clone() {
        Adt13::C0(h2945, _) => {
            h2945
        }
    })?), List::cons((Str::lit("cacheScope"), Adt9::C3(f902(match v0 {
        Adt13::C0(_, h2946) => {
            h2946
        }
    }))), List::<(Str, Adt9)>::nil())))
}

pub fn f837(v0: List<Adt40>) -> List<Adt9> {
    f490(f903(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f838(v0: List<Adt49>) -> R<List<Adt9>> {
    Ok(f490(f904(v0, List::<Adt9>::nil())?, List::<Adt9>::nil()))
}

pub fn f839(v0: List<Adt51>) -> List<Adt9> {
    f490(f905(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f840(v0: List<Adt59>) -> List<Adt9> {
    f490(f906(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f841(v0: List<Adt61>) -> List<Adt9> {
    f490(f907(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f842(v0: List<Adt64>) -> R<Option<List<Adt9>>> {
    let m2947 = f908(v0, List::<Adt9>::nil())?;
    match m2947 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f490(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f843(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt9)>> {
    let m2948 = v1;
    match m2948 {
        None => {
            Ok(List::<(Str, Adt9)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f113(v2)?), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f844(p2954: List<(Str, Adt69)>, p2955: List<(Str, Adt9)>) -> R<Option<List<(Str, Adt9)>>> {
    let m2953 = (p2954, p2955);
    {
        let mut m2953 = m2953;
        loop {
            let step = {
                let (v0, v1) = m2953;
                let m2949 = v0;
                match m2949.uncons() {
                    None => {
                        Err::<(List<(Str, Adt69)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2951 = f909({
                            let (_, h2950) = v2;
                            h2950
                        })?;
                        match m2951 {
                            None => {
                                Err::<(List<(Str, Adt69)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>(Ok(None::<List<(Str, Adt9)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt69)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>((v3, List::cons(({
                                    let (h2952, _) = v2;
                                    h2952
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f845(v0: Adt86) -> List<(Str, Adt69)> {
    let m2956 = v0;
    match m2956 {
        Adt86::C0(v1) => {
            v1
        }
        Adt86::C1(_) => {
            List::<(Str, Adt69)>::nil()
        }
        Adt86::C2(v3, _) => {
            v3
        }
    }
}

pub fn f846(v0: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m2957 = v0.clone();
    match m2957.uncons() {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt9::C5(f552(v0, List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f847(v0: Option<Str>) -> List<(Str, Adt9)> {
    let m2958 = v0;
    match m2958 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f848(v0: Adt86) -> Option<Str> {
    let m2959 = v0;
    match m2959 {
        Adt86::C0(_) => {
            None::<Str>
        }
        Adt86::C1(v2) => {
            Some(v2)
        }
        Adt86::C2(_, v4) => {
            Some(v4)
        }
    }
}

pub fn f849(v0: Adt67) -> Option<List<(Str, Adt9)>> {
    let m2960 = v0;
    match m2960 {
        Adt67::C0(_) => {
            Some(List::<(Str, Adt9)>::nil())
        }
        Adt67::C1(_, v3) => {
            let m2961 = f910(v3);
            match m2961 {
                None => {
                    None::<List<(Str, Adt9)>>
                }
                Some(v4) => {
                    Some(List::cons((Str::lit("inputRequests"), v4), List::<(Str, Adt9)>::nil()))
                }
            }
        }
        Adt67::C2(_, v6) => {
            Some(List::cons((Str::lit("result"), Adt9::C5(match v6 {
                Adt8::C0(h2962) => {
                    h2962
                }
            })), List::<(Str, Adt9)>::nil()))
        }
        Adt67::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f442(v8)), List::<(Str, Adt9)>::nil()))
        }
        Adt67::C4(_) => {
            Some(List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f850(v0: Adt68, v1: Str) -> List<(Str, Adt9)> {
    List::cons((Str::lit("taskId"), Adt9::C3(match v0.clone() {
        Adt68::C0(h2963, _, _, _, _, _) => {
            h2963
        }
    })), List::cons((Str::lit("status"), Adt9::C3(v1)), {
        let a2969 = f911(Str::lit("statusMessage"), match v0.clone() {
            Adt68::C0(_, h2964, _, _, _, _) => {
                h2964
            }
        });
        let a2970 = List::cons((Str::lit("createdAt"), Adt9::C3(match v0.clone() {
            Adt68::C0(_, _, h2965, _, _, _) => {
                h2965
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt9::C3(match v0.clone() {
            Adt68::C0(_, _, _, h2966, _, _) => {
                h2966
            }
        })), List::cons((Str::lit("ttlMs"), f912(match v0.clone() {
            Adt68::C0(_, _, _, _, h2967, _) => {
                h2967
            }
        })), f913(Str::lit("pollIntervalMs"), match v0 {
            Adt68::C0(_, _, _, _, _, h2968) => {
                h2968
            }
        }))));
        append_list(a2969, a2970)
    }))
}

pub fn f851(v0: Adt88) -> Str {
    let m2971 = v0;
    match m2971 {
        Adt88::C0 => {
            Str::lit("working")
        }
        Adt88::C1 => {
            Str::lit("input_required")
        }
        Adt88::C2 => {
            Str::lit("completed")
        }
        Adt88::C3 => {
            Str::lit("failed")
        }
        Adt88::C4 => {
            Str::lit("cancelled")
        }
    }
}

pub fn f852(v0: Adt67) -> Adt88 {
    let m2972 = v0;
    match m2972 {
        Adt67::C0(_) => {
            Adt88::C0
        }
        Adt67::C1(_, _) => {
            Adt88::C1
        }
        Adt67::C2(_, _) => {
            Adt88::C2
        }
        Adt67::C3(_, _) => {
            Adt88::C3
        }
        Adt67::C4(_) => {
            Adt88::C4
        }
    }
}

pub fn f853() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f854(v0: Adt4) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt4::C0(h2973, _, _, _, _, _) => {
            h2973
        }
    })), List::cons((Str::lit("version"), Adt9::C3(match v0.clone() {
        Adt4::C0(_, _, h2974, _, _, _) => {
            h2974
        }
    })), {
        let a2983 = f733(Str::lit("title"), match v0.clone() {
            Adt4::C0(_, h2975, _, _, _, _) => {
                h2975
            }
        });
        let a2984 = {
            let a2981 = f733(Str::lit("description"), match v0.clone() {
                Adt4::C0(_, _, _, h2976, _, _) => {
                    h2976
                }
            });
            let a2982 = {
                let a2979 = f733(Str::lit("websiteUrl"), match v0.clone() {
                    Adt4::C0(_, _, _, _, h2977, _) => {
                        h2977
                    }
                });
                let a2980 = f734(match v0 {
                    Adt4::C0(_, _, _, _, _, h2978) => {
                        h2978
                    }
                });
                append_list(a2979, a2980)
            };
            append_list(a2981, a2982)
        };
        append_list(a2983, a2984)
    })))
}

pub fn f855(v0: List<Adt89>) -> Option<Adt9> {
    let m2985 = f914(v0, List::<Adt9>::nil());
    match m2985 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C4(f490(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f856(v0: Str, p2990: List<Adt89>, v2: i64, p2991: List<Adt89>) -> R<List<Adt89>> {
    let m2989 = (p2990, p2991);
    {
        let mut m2989 = m2989;
        loop {
            let step = {
                let (v1, v3) = m2989;
                let m2986 = v1;
                match m2986.uncons() {
                    None => {
                        Err::<(List<Adt89>, List<Adt89>), R<List<Adt89>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a2987 = f337(v4.clone());
                        let a2988 = v0.clone();
                        if if equal(a2987, a2988) {
                            true
                        } else {
                            f338(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt89>, List<Adt89>), R<List<Adt89>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt89>, List<Adt89>), R<List<Adt89>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2989 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f857(v0: List<List<u64>>) -> bool {
    let m2992 = v0;
    match m2992.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f858() -> List<Adt101> {
    List::cons(Adt101::C0(f915(), f777()), List::cons(Adt101::C0(f916(), f917()), List::cons(Adt101::C0(f918(), f919()), List::<Adt101>::nil())))
}

pub fn f859(v0: List<List<u64>>) -> List<u64> {
    f920(f153(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f860(v0: Adt24) -> Adt9 {
    let m2993 = v0;
    match m2993 {
        Adt24::C0(v1) => {
            Adt9::C3(v1)
        }
        Adt24::C1(v2) => {
            Adt9::C2(Adt10::C0(v2, 0i64))
        }
    }
}

pub fn f861(v0: Option<Adt10>) -> List<(Str, Adt9)> {
    let m2994 = v0;
    match m2994 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt9::C2(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f862(v0: Option<Str>) -> List<(Str, Adt9)> {
    let m2995 = v0;
    match m2995 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f863() -> i64 {
    -32603i64
}

pub fn f864() -> i64 {
    429i64
}

pub fn f865() -> i64 {
    -32021i64
}

pub fn f866(p2998: List<Str>, p2999: List<Adt9>) -> List<Adt9> {
    let m2997 = (p2998, p2999);
    {
        let mut m2997 = m2997;
        loop {
            let step = {
                let (v0, v1) = m2997;
                let m2996 = v0;
                match m2996.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f867(v0: List<u64>) -> R<bool> {
    let m3000 = v0;
    match m3000.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m3001 = v2;
            match m3001.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a3002 = f535(v1)?;
                    let a3003 = 113u64;
                    if if nat_eq(a3002, a3003) {
                        let a3004 = v3;
                        let a3005 = 61u64;
                        nat_eq(a3004, a3005)
                    } else {
                        false
                    } {
                        Ok(f921(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f868(v0: Adt3, v1: Fn9, v2: u64, v3: Result<Adt19, Adt151>) -> R<Result<Adt147, Adt150>> {
    let m3006 = v3;
    match m3006 {
        Ok(v5) => {
            f922(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt147, Adt150>(Adt150::C0(v4)))
        }
    }
}

pub fn f869(v0: Adt159, v1: Adt21, v2: Adt8) -> Result<Adt19, Adt151> {
    let m3007 = v0;
    match m3007 {
        Adt159::C0 => {
            f923(v1, v2)
        }
        Adt159::C1 => {
            f924(v1, v2)
        }
        Adt159::C2 => {
            f925(v1, v2)
        }
        Adt159::C3 => {
            f926(v1, v2)
        }
        Adt159::C4 => {
            f927(v1, v2)
        }
        Adt159::C5 => {
            f928(v1, v2)
        }
        Adt159::C6 => {
            f929(v1, v2)
        }
        Adt159::C7 => {
            f930(v1, v2)
        }
        Adt159::C8 => {
            f931(v1, v2)
        }
        Adt159::C9 => {
            f932(v1, v2)
        }
        Adt159::C10 => {
            f933(v1, v2)
        }
        Adt159::C11 => {
            f934(v1, v2)
        }
    }
}

pub fn f870(v0: Adt159) -> Str {
    let m3008 = v0;
    match m3008 {
        Adt159::C0 => {
            Str::lit("server/discover")
        }
        Adt159::C1 => {
            Str::lit("tools/list")
        }
        Adt159::C2 => {
            Str::lit("tools/call")
        }
        Adt159::C3 => {
            Str::lit("resources/list")
        }
        Adt159::C4 => {
            Str::lit("resources/read")
        }
        Adt159::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt159::C6 => {
            Str::lit("prompts/list")
        }
        Adt159::C7 => {
            Str::lit("prompts/get")
        }
        Adt159::C8 => {
            Str::lit("completion/complete")
        }
        Adt159::C9 => {
            Str::lit("tasks/get")
        }
        Adt159::C10 => {
            Str::lit("tasks/update")
        }
        Adt159::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f871(v0: Option<Adt11>) -> bool {
    let m3009 = v0;
    match m3009 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f872(v0: Option<Adt12>) -> bool {
    let m3010 = v0;
    match m3010 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f873(v0: List<Str>, v1: Adt127, v2: Adt21, v3: Option<Adt133>) -> Adt162 {
    let m3011 = v3;
    match m3011 {
        None => {
            f935(v1, v2.clone(), f936(match v2 {
                Adt21::C0(h3012, _, _, _, _, _) => {
                    h3012
                }
            }, v0))
        }
        Some(v4) => {
            Adt162::C1(Adt150::C2(v4))
        }
    }
}

pub fn f874(v0: Option<Adt132>, v1: Adt164, v2: Str, v3: Str, v4: Adt8) -> R<Option<Adt133>> {
    let m3013 = v0;
    match m3013 {
        None => {
            Ok(None::<Adt133>)
        }
        Some(v5) => {
            let m3015 = match v5.clone() {
                Adt132::C0(_, _, _, h3014, _) => {
                    h3014
                }
            };
            match m3015 {
                None => {
                    f937(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f875(v0: Option<Adt8>) -> Adt8 {
    let m3016 = v0;
    match m3016 {
        None => {
            f32()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f876(v0: Adt8) -> Result<Adt21, Adt151> {
    let m3017 = f938(v0.clone());
    match m3017 {
        Adt170::C0 => {
            f939(v0.clone(), f275(f940(), v0.clone()), f275(f941(), v0))
        }
        Adt170::C1 => {
            Err::<Adt21, Adt151>(Adt151::C2)
        }
        Adt170::C2 => {
            Err::<Adt21, Adt151>(Adt151::C3)
        }
    }
}

pub fn f877() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f878(v0: u64, v1: u64) -> u64 {
    let a3018 = v0;
    let a3019 = v1;
    let a3020 = 0u64;
    nat_rem(a3018, a3019, a3020)
}

pub fn f879(v0: u64, v1: u64) -> u64 {
    let a3021 = v0;
    let a3022 = v1;
    let a3023 = 0u64;
    nat_quot(a3021, a3022, a3023)
}

pub fn f880(p3027: List<Adt9>) -> bool {
    let m3026 = p3027;
    {
        let mut m3026 = m3026;
        loop {
            let step = {
                let v0 = m3026;
                let m3024 = v0;
                match m3024.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3025 = f181(v1);
                        match m3025 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(_) => {
                                Ok::<List<Adt9>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f881(p3031: List<Adt9>) -> bool {
    let m3030 = p3031;
    {
        let mut m3030 = m3030;
        loop {
            let step = {
                let v0 = m3030;
                let m3028 = v0;
                match m3028.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3029 = f181(v1);
                        match m3029 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(v3) => {
                                if f708(v3) {
                                    Ok::<List<Adt9>, bool>(v2)
                                } else {
                                    Err::<List<Adt9>, bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3030 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f882(v0: Str) -> Option<Adt16> {
    let a3032 = v0.clone();
    let a3033 = Str::lit("string");
    if equal(a3032, a3033) {
        Some(Adt16::C0)
    } else {
        let a3034 = v0.clone();
        let a3035 = Str::lit("integer");
        if equal(a3034, a3035) {
            Some(Adt16::C1)
        } else {
            let a3036 = v0;
            let a3037 = Str::lit("boolean");
            if equal(a3036, a3037) {
                Some(Adt16::C2)
            } else {
                None::<Adt16>
            }
        }
    }
}

pub fn f883(p3046: List<u64>, p3047: List<u64>) -> R<List<u64>> {
    let m3045 = (p3046, p3047);
    {
        let mut m3045 = m3045;
        loop {
            let step = {
                let (v0, v1) = m3045;
                let m3038 = v0;
                match m3038.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a3039 = 65u64;
                        let a3040 = v2;
                        if if nat_le(a3039, a3040) {
                            let a3041 = v2;
                            let a3042 = 90u64;
                            nat_le(a3041, a3042)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a3043 = v2;
                                let a3044 = 32u64;
                                nat_add(a3043, a3044)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3045 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f884(p3054: List<u64>, p3055: List<u64>) -> bool {
    let m3053 = (p3054, p3055);
    {
        let mut m3053 = m3053;
        loop {
            let step = {
                let (v0, v1) = m3053;
                let m3048 = v0;
                match m3048.uncons() {
                    None => {
                        let m3049 = v1;
                        match m3049.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m3050 = v1;
                        match m3050.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a3051 = v4;
                                let a3052 = v6;
                                if nat_eq(a3051, a3052) {
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
                Ok(next) => { m3053 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f885(p3067: u64, p3068: i64, p3069: i64) -> R<bool> {
    let m3066 = (p3067, (p3068, p3069));
    {
        let mut m3066 = m3066;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3066;
                let m3056 = v0;
                if m3056 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m3056 - 1;
                    let m3059 = {
                        let a3057 = v2;
                        let a3058 = 0i64;
                        compare(a3057, a3058)
                    };
                    if match m3059 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f387({
                            let a3060 = v1;
                            let a3061 = 10i64;
                            let a3062 = 0i64;
                            int_rem(a3060, a3061, a3062)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a3063 = v1;
                                let a3064 = 10i64;
                                let a3065 = 0i64;
                                int_quot(a3063, a3064, a3065)?
                            }, f491(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m3066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f886(v0: Adt9, v1: Adt9, v2: List<(Adt9, Adt9)>) -> R<Adt169> {
    let a3070 = f942(v0.clone());
    let a3071 = f942(v1.clone());
    if nat_eq(a3070, a3071) {
        let m3072 = v0;
        match m3072 {
            Adt9::C0 => {
                Ok(Adt169::C1(v2))
            }
            Adt9::C1(v3) => {
                let m3073 = f468(v1);
                match m3073 {
                    None => {
                        Ok(Adt169::C0)
                    }
                    Some(v4) => {
                        let a3074 = v3;
                        let a3075 = v4;
                        if equal(a3074, a3075) {
                            Ok(Adt169::C1(v2))
                        } else {
                            Ok(Adt169::C0)
                        }
                    }
                }
            }
            Adt9::C2(v5) => {
                let m3076 = f713(v1);
                match m3076 {
                    None => {
                        Ok(Adt169::C0)
                    }
                    Some(v6) => {
                        if f943(f715(v5, v6)?) {
                            Ok(Adt169::C1(v2))
                        } else {
                            Ok(Adt169::C0)
                        }
                    }
                }
            }
            Adt9::C3(v7) => {
                let m3077 = f181(v1);
                match m3077 {
                    None => {
                        Ok(Adt169::C0)
                    }
                    Some(v8) => {
                        let a3078 = v7;
                        let a3079 = v8;
                        if equal(a3078, a3079) {
                            Ok(Adt169::C1(v2))
                        } else {
                            Ok(Adt169::C0)
                        }
                    }
                }
            }
            Adt9::C4(v9) => {
                let m3080 = f335(v1);
                match m3080 {
                    None => {
                        Ok(Adt169::C0)
                    }
                    Some(v10) => {
                        Ok(f944(f945(v9, v10, List::<(Adt9, Adt9)>::nil()), v2))
                    }
                }
            }
            Adt9::C5(v11) => {
                let m3081 = f281(v1);
                match m3081 {
                    None => {
                        Ok(Adt169::C0)
                    }
                    Some(v12) => {
                        let a3083 = f719(v11.clone(), 0u64)?;
                        let a3084 = f719(match v12.clone() {
                            Adt8::C0(h3082) => {
                                h3082
                            }
                        }, 0u64)?;
                        if nat_eq(a3083, a3084) {
                            Ok(f944(f946(v11, match v12 {
                                Adt8::C0(h3085) => {
                                    h3085
                                }
                            }, List::<(Adt9, Adt9)>::nil()), v2))
                        } else {
                            Ok(Adt169::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt169::C0)
    }
}

pub fn f887(v0: Adt166, v1: Adt166) -> Adt166 {
    let m3086 = v0;
    match m3086 {
        Adt166::C0 => {
            f947(v1)
        }
        Adt166::C1 => {
            Adt166::C1
        }
        Adt166::C2 => {
            v1
        }
    }
}

pub fn f888(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt166> {
    let m3087 = f817(f948(v0, v1)?, f948(v2, v3)?);
    match m3087 {
        Adt166::C0 => {
            Ok(Adt166::C0)
        }
        Adt166::C1 => {
            f949(v0, f950(v0)?, v2, f950(v2)?)
        }
        Adt166::C2 => {
            Ok(Adt166::C2)
        }
    }
}

pub fn f889(v0: i64) -> R<i64> {
    let m3090 = {
        let a3088 = v0;
        let a3089 = 0i64;
        compare(a3088, a3089)
    };
    if match m3090 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a3091 = v0;
        int_neg(a3091)
    } else {
        Ok(v0)
    }
}

pub fn f890(v0: u64, v1: u64) -> u64 {
    let a3092 = v0;
    let a3093 = v1;
    let a3094 = 0u64;
    nat_rem(a3092, a3093, a3094)
}

pub fn f891(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a3097 = {
            let a3095 = v0;
            let a3096 = 4u64;
            nat_mul(a3095, a3096)?
        };
        let a3098 = f893(v1, 16u64);
        nat_add(a3097, a3098)?
    }, List::<u64>::nil()))
}

pub fn f892(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a3101 = {
            let a3099 = v0;
            let a3100 = 4u64;
            nat_mul(a3099, a3100)?
        };
        let a3102 = f893(v1, 16u64);
        nat_add(a3101, a3102)?
    }, List::cons({
        let a3105 = {
            let a3103 = f890(v1, 16u64);
            let a3104 = 16u64;
            nat_mul(a3103, a3104)?
        };
        let a3106 = f893(v2, 4u64);
        nat_add(a3105, a3106)?
    }, List::<u64>::nil())))
}

pub fn f893(v0: u64, v1: u64) -> u64 {
    let a3107 = v0;
    let a3108 = v1;
    let a3109 = 0u64;
    nat_quot(a3107, a3108, a3109)
}

pub fn f894(v0: u64, v1: List<u64>) -> Adt167 {
    let a3110 = v0;
    let a3111 = 32u64;
    if nat_lt(a3110, a3111) {
        if f951(v0) {
            Adt167::C0(0u64, List::cons(v0, v1))
        } else {
            Adt167::C0(0u64, f895(v1))
        }
    } else {
        let a3112 = v0;
        let a3113 = 127u64;
        if nat_eq(a3112, a3113) {
            Adt167::C0(0u64, f895(v1))
        } else {
            let a3114 = v0;
            let a3115 = 194u64;
            if nat_eq(a3114, a3115) {
                Adt167::C0(1u64, v1)
            } else {
                let a3116 = v0;
                let a3117 = 226u64;
                if nat_eq(a3116, a3117) {
                    Adt167::C0(2u64, v1)
                } else {
                    Adt167::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f895(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f896(v0: List<Adt46>) -> List<Adt9> {
    f490(f952(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f897(p3120: List<Adt5>, p3121: List<Adt9>) -> List<Adt9> {
    let m3119 = (p3120, p3121);
    {
        let mut m3119 = m3119;
        loop {
            let step = {
                let (v0, v1) = m3119;
                let m3118 = v0;
                match m3118.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt5>, List<Adt9>), List<Adt9>>((v3, List::cons(f953(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f898(p3133: u64, p3134: i64, p3135: List<u64>) -> R<List<u64>> {
    let m3132 = (p3133, (p3134, p3135));
    {
        let mut m3132 = m3132;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3132;
                let m3122 = v0;
                if m3122 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m3122 - 1;
                    let v4: i64 = {
                        let a3123 = v1;
                        let a3124 = 10i64;
                        let a3125 = 0i64;
                        int_quot(a3123, a3124, a3125)?
                    };
                    let v5: i64 = {
                        let a3126 = v1;
                        let a3127 = 10i64;
                        let a3128 = 0i64;
                        int_rem(a3126, a3127, a3128)
                    };
                    let m3131 = {
                        let a3129 = v4;
                        let a3130 = 1i64;
                        compare(a3129, a3130)
                    };
                    if match m3131 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f954(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f954(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m3132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f899(v0: u64) -> R<u64> {
    let a3136 = v0;
    let a3137 = 10u64;
    if nat_lt(a3136, a3137) {
        let a3138 = 48u64;
        let a3139 = v0;
        nat_add(a3138, a3139)
    } else {
        let a3140 = 87u64;
        let a3141 = v0;
        nat_add(a3140, a3141)
    }
}

pub fn f900(v0: Str, v1: Option<Adt11>) -> List<(Str, Adt9)> {
    let m3142 = v1;
    match m3142 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f955(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f901(v0: Option<Adt12>) -> List<(Str, Adt9)> {
    let m3143 = v0;
    match m3143 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt9::C5({
                let a3146 = f391(Str::lit("subscribe"), match v1.clone() {
                    Adt12::C0(h3144, _) => {
                        h3144
                    }
                });
                let a3147 = f391(Str::lit("listChanged"), match v1 {
                    Adt12::C0(_, h3145) => {
                        h3145
                    }
                });
                append_list(a3146, a3147)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f902(v0: Adt14) -> Str {
    let m3148 = v0;
    match m3148 {
        Adt14::C0 => {
            Str::lit("public")
        }
        Adt14::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f903(p3151: List<Adt40>, p3152: List<Adt9>) -> List<Adt9> {
    let m3150 = (p3151, p3152);
    {
        let mut m3150 = m3150;
        loop {
            let step = {
                let (v0, v1) = m3150;
                let m3149 = v0;
                match m3149.uncons() {
                    None => {
                        Err::<(List<Adt40>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt40>, List<Adt9>), List<Adt9>>((v3, List::cons(f956(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f904(p3155: List<Adt49>, p3156: List<Adt9>) -> R<List<Adt9>> {
    let m3154 = (p3155, p3156);
    {
        let mut m3154 = m3154;
        loop {
            let step = {
                let (v0, v1) = m3154;
                let m3153 = v0;
                match m3153.uncons() {
                    None => {
                        Err::<(List<Adt49>, List<Adt9>), R<List<Adt9>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt49>, List<Adt9>), R<List<Adt9>>>((v3, List::cons(f957(v2)?, v1)))
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

pub fn f905(p3159: List<Adt51>, p3160: List<Adt9>) -> List<Adt9> {
    let m3158 = (p3159, p3160);
    {
        let mut m3158 = m3158;
        loop {
            let step = {
                let (v0, v1) = m3158;
                let m3157 = v0;
                match m3157.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt51>, List<Adt9>), List<Adt9>>((v3, List::cons(f736(v2), v1)))
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

pub fn f906(p3163: List<Adt59>, p3164: List<Adt9>) -> List<Adt9> {
    let m3162 = (p3163, p3164);
    {
        let mut m3162 = m3162;
        loop {
            let step = {
                let (v0, v1) = m3162;
                let m3161 = v0;
                match m3161.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt59>, List<Adt9>), List<Adt9>>((v3, List::cons(f958(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f907(p3167: List<Adt61>, p3168: List<Adt9>) -> List<Adt9> {
    let m3166 = (p3167, p3168);
    {
        let mut m3166 = m3166;
        loop {
            let step = {
                let (v0, v1) = m3166;
                let m3165 = v0;
                match m3165.uncons() {
                    None => {
                        Err::<(List<Adt61>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt61>, List<Adt9>), List<Adt9>>((v3, List::cons(f959(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f908(p3172: List<Adt64>, p3173: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3171 = (p3172, p3173);
    {
        let mut m3171 = m3171;
        loop {
            let step = {
                let (v0, v1) = m3171;
                let m3169 = v0;
                match m3169.uncons() {
                    None => {
                        Err::<(List<Adt64>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3170 = f960(v2)?;
                        match m3170 {
                            None => {
                                Err::<(List<Adt64>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(None::<List<Adt9>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt64>, List<Adt9>), R<Option<List<Adt9>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f909(v0: Adt69) -> R<Option<Adt9>> {
    let m3174 = v0;
    match m3174 {
        Adt69::C0(v1) => {
            Ok(Some(f961(Str::lit("elicitation/create"), Some(f962(v1)))))
        }
        Adt69::C1(v2) => {
            let m3175 = f963(v2)?;
            match m3175 {
                None => {
                    Ok(None::<Adt9>)
                }
                Some(v3) => {
                    Ok(Some(f961(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt69::C2(v4) => {
            Ok(Some(f961(Str::lit("roots/list"), {
                let m3176 = v4;
                match m3176 {
                    None => {
                        None::<Adt9>
                    }
                    Some(v5) => {
                        Some(Adt9::C5(match v5 {
                            Adt8::C0(h3177) => {
                                h3177
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f910(v0: List<(Str, Adt69)>) -> Option<Adt9> {
    let m3178 = f964(v0, List::<(Str, Adt9)>::nil());
    match m3178 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(f552(v1, List::<(Str, Adt9)>::nil())))
        }
    }
}

pub fn f911(v0: Str, v1: Option<Str>) -> List<(Str, Adt9)> {
    let m3179 = v1;
    match m3179 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C3(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f912(v0: Option<i64>) -> Adt9 {
    let m3180 = v0;
    match m3180 {
        None => {
            Adt9::C0
        }
        Some(v1) => {
            f965(v1)
        }
    }
}

pub fn f913(v0: Str, v1: Option<i64>) -> List<(Str, Adt9)> {
    let m3181 = v1;
    match m3181 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f965(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f914(p3185: List<Adt89>, p3186: List<Adt9>) -> Option<List<Adt9>> {
    let m3184 = (p3185, p3186);
    {
        let mut m3184 = m3184;
        loop {
            let step = {
                let (v0, v1) = m3184;
                let m3182 = v0;
                match m3182.uncons() {
                    None => {
                        Err::<(List<Adt89>, List<Adt9>), Option<List<Adt9>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m3183 = f966(v2);
                        match m3183 {
                            None => {
                                Err::<(List<Adt89>, List<Adt9>), Option<List<Adt9>>>(None::<List<Adt9>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt89>, List<Adt9>), Option<List<Adt9>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f915() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f916() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f917() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f918() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f919() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f920(p3191: List<List<u64>>, p3192: List<u64>) -> List<u64> {
    let m3190 = (p3191, p3192);
    {
        let mut m3190 = m3190;
        loop {
            let step = {
                let (v0, v1) = m3190;
                let m3187 = v0;
                match m3187.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a3188 = f967(v2);
                            let a3189 = v1;
                            append_list(a3188, a3189)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m3190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f921(v0: List<u64>) -> bool {
    let m3193 = v0;
    match m3193.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a3194 = v1;
            let a3195 = 48u64;
            if nat_eq(a3194, a3195) {
                f968(v2)
            } else {
                false
            }
        }
    }
}

pub fn f922(v0: Adt3, v1: Fn9, v2: u64, v3: Adt19) -> R<Result<Adt147, Adt150>> {
    if f969(f970(v3.clone())) {
        Ok(Ok::<Adt147, Adt150>(f971(v0.clone(), Adt37::C0(f972(v0)))))
    } else {
        Ok(f973(v0.clone(), v3.clone(), f974(match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h3196, _) => {
                h3196
            }
        }, match v0 {
            Adt3::C0(_, _, _, _, _, h3197, _, _) => {
                h3197
            }
        }, v2, v1, v3)?))
    }
}

pub fn f923(v0: Adt21, _: Adt8) -> Result<Adt19, Adt151> {
    Ok::<Adt19, Adt151>(Adt19::C0(Adt20::C0(v0)))
}

pub fn f924(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3198 = f975(Str::lit("cursor"), v1);
    match m3198 {
        Ok(v2) => {
            Ok::<Adt19, Adt151>(Adt19::C1(Adt25::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt19, Adt151>(v3)
        }
    }
}

pub fn f925(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3199 = f976(Str::lit("name"), v1.clone());
    match m3199 {
        Ok(v2) => {
            let m3200 = f977(Str::lit("arguments"), v1.clone());
            match m3200 {
                Ok(v3) => {
                    let m3201 = f977(Str::lit("inputResponses"), v1.clone());
                    match m3201 {
                        Ok(v4) => {
                            let m3202 = f975(Str::lit("requestState"), v1);
                            match m3202 {
                                Ok(v5) => {
                                    Ok::<Adt19, Adt151>(Adt19::C2(Adt26::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt19, Adt151>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt19, Adt151>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt19, Adt151>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt19, Adt151>(v9)
        }
    }
}

pub fn f926(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3203 = f975(Str::lit("cursor"), v1);
    match m3203 {
        Ok(v2) => {
            Ok::<Adt19, Adt151>(Adt19::C3(Adt25::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt19, Adt151>(v3)
        }
    }
}

pub fn f927(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3204 = f976(Str::lit("uri"), v1.clone());
    match m3204 {
        Ok(v2) => {
            let m3205 = f977(Str::lit("inputResponses"), v1.clone());
            match m3205 {
                Ok(v3) => {
                    let m3206 = f975(Str::lit("requestState"), v1);
                    match m3206 {
                        Ok(v4) => {
                            Ok::<Adt19, Adt151>(Adt19::C4(Adt27::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt19, Adt151>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt19, Adt151>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt19, Adt151>(v7)
        }
    }
}

pub fn f928(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3207 = f975(Str::lit("cursor"), v1);
    match m3207 {
        Ok(v2) => {
            Ok::<Adt19, Adt151>(Adt19::C5(Adt25::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt19, Adt151>(v3)
        }
    }
}

pub fn f929(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3208 = f975(Str::lit("cursor"), v1);
    match m3208 {
        Ok(v2) => {
            Ok::<Adt19, Adt151>(Adt19::C6(Adt25::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt19, Adt151>(v3)
        }
    }
}

pub fn f930(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3209 = f976(Str::lit("name"), v1.clone());
    match m3209 {
        Ok(v2) => {
            let m3210 = f978(Str::lit("arguments"), v1.clone());
            match m3210 {
                Ok(v3) => {
                    let m3211 = f977(Str::lit("inputResponses"), v1.clone());
                    match m3211 {
                        Ok(v4) => {
                            let m3212 = f975(Str::lit("requestState"), v1);
                            match m3212 {
                                Ok(v5) => {
                                    Ok::<Adt19, Adt151>(Adt19::C7(Adt28::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt19, Adt151>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt19, Adt151>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt19, Adt151>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt19, Adt151>(v9)
        }
    }
}

pub fn f931(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3213 = f979(Str::lit("ref"), v1.clone());
    match m3213 {
        Ok(v2) => {
            let m3214 = f980(v2);
            match m3214 {
                Ok(v3) => {
                    let m3215 = f979(Str::lit("argument"), v1.clone());
                    match m3215 {
                        Ok(v4) => {
                            let m3216 = f981(v4);
                            match m3216 {
                                Ok(v5) => {
                                    let m3217 = f982(v1);
                                    match m3217 {
                                        Ok(v6) => {
                                            Ok::<Adt19, Adt151>(Adt19::C8(Adt29::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt19, Adt151>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt19, Adt151>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt19, Adt151>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt19, Adt151>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt19, Adt151>(v11)
        }
    }
}

pub fn f932(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3218 = f976(Str::lit("taskId"), v1);
    match m3218 {
        Ok(v2) => {
            Ok::<Adt19, Adt151>(Adt19::C9(Adt34::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt19, Adt151>(v3)
        }
    }
}

pub fn f933(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3219 = f976(Str::lit("taskId"), v1.clone());
    match m3219 {
        Ok(v2) => {
            let m3220 = f979(Str::lit("inputResponses"), v1);
            match m3220 {
                Ok(v3) => {
                    Ok::<Adt19, Adt151>(Adt19::C10(Adt35::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt19, Adt151>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt19, Adt151>(v5)
        }
    }
}

pub fn f934(v0: Adt21, v1: Adt8) -> Result<Adt19, Adt151> {
    let m3221 = f976(Str::lit("taskId"), v1);
    match m3221 {
        Ok(v2) => {
            Ok::<Adt19, Adt151>(Adt19::C11(Adt36::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt19, Adt151>(v3)
        }
    }
}

pub fn f935(v0: Adt127, v1: Adt21, v2: Option<Adt150>) -> Adt162 {
    let m3222 = v2;
    match m3222 {
        None => {
            Adt162::C0(Adt163::C0(match v0.clone() {
                Adt127::C0(h3223, _, _) => {
                    h3223
                }
            }, match v0.clone() {
                Adt127::C0(_, h3224, _) => {
                    h3224
                }
            }, v1, f875(match v0 {
                Adt127::C0(_, _, h3225) => {
                    h3225
                }
            })))
        }
        Some(v3) => {
            Adt162::C1(v3)
        }
    }
}

pub fn f936(v0: Str, v1: List<Str>) -> Option<Adt150> {
    f983(f984(v0.clone(), v1.clone()), v0, v1)
}

pub fn f937(v0: Adt132, v1: Adt164, v2: Str, v3: Str, v4: Adt8) -> R<Option<Adt133>> {
    let m3227 = match v0.clone() {
        Adt132::C0(h3226, _, _, _, _) => {
            h3226
        }
    };
    match m3227 {
        None => {
            Ok(Some(Adt133::C0))
        }
        Some(v5) => {
            let a3228 = v5;
            let a3229 = v3;
            if equal(a3228, a3229) {
                let m3231 = match v0.clone() {
                    Adt132::C0(_, h3230, _, _, _) => {
                        h3230
                    }
                };
                match m3231 {
                    None => {
                        Ok(Some(Adt133::C1))
                    }
                    Some(v6) => {
                        let a3232 = v6;
                        let a3233 = v2;
                        if equal(a3232, a3233) {
                            f985(f986(match v0.clone() {
                                Adt132::C0(_, _, h3234, _, _) => {
                                    h3234
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt132::C0(_, _, _, _, h3235) => {
                                    h3235
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt133::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt133::C3))
            }
        }
    }
}

pub fn f938(v0: Adt8) -> Adt170 {
    f987(f275(f940(), v0.clone()), f275(f941(), v0))
}

pub fn f939(v0: Adt8, v1: Adt118, v2: Adt118) -> Result<Adt21, Adt151> {
    let m3236 = v1;
    match m3236 {
        Adt118::C0 => {
            Err::<Adt21, Adt151>(Adt151::C2)
        }
        Adt118::C1(v3) => {
            let m3237 = v2;
            match m3237 {
                Adt118::C0 => {
                    Err::<Adt21, Adt151>(Adt151::C3)
                }
                Adt118::C1(v4) => {
                    f988(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f940() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f941() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f942(v0: Adt9) -> u64 {
    let m3238 = v0;
    match m3238 {
        Adt9::C0 => {
            0u64
        }
        Adt9::C1(_) => {
            1u64
        }
        Adt9::C2(_) => {
            2u64
        }
        Adt9::C3(_) => {
            3u64
        }
        Adt9::C4(_) => {
            4u64
        }
        Adt9::C5(_) => {
            5u64
        }
    }
}

pub fn f943(v0: Adt166) -> bool {
    let m3239 = v0;
    match m3239 {
        Adt166::C0 => {
            false
        }
        Adt166::C1 => {
            true
        }
        Adt166::C2 => {
            false
        }
    }
}

pub fn f944(v0: Option<List<(Adt9, Adt9)>>, v1: List<(Adt9, Adt9)>) -> Adt169 {
    let m3240 = v0;
    match m3240 {
        None => {
            Adt169::C0
        }
        Some(v2) => {
            Adt169::C1({
                let a3241 = v2;
                let a3242 = v1;
                append_list(a3241, a3242)
            })
        }
    }
}

pub fn f945(p3247: List<Adt9>, p3248: List<Adt9>, p3249: List<(Adt9, Adt9)>) -> Option<List<(Adt9, Adt9)>> {
    let m3246 = (p3247, (p3248, p3249));
    {
        let mut m3246 = m3246;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3246;
                let m3243 = v0;
                match m3243.uncons() {
                    None => {
                        let m3244 = v1;
                        match m3244.uncons() {
                            None => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m3245 = v1;
                        match m3245.uncons() {
                            None => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                            Some((v7, v8)) => {
                                Ok::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>((v6, (v8, List::cons((v5, v7), v2))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f946(p3255: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, p3256: List<(Adt9, Adt9)>) -> Option<List<(Adt9, Adt9)>> {
    let m3254 = (p3255, p3256);
    {
        let mut m3254 = m3254;
        loop {
            let step = {
                let (v0, v2) = m3254;
                let m3250 = v0;
                match m3250.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m3252 = f383({
                            let (h3251, _) = v3;
                            h3251
                        }, v1.clone());
                        match m3252 {
                            Adt118::C0 => {
                                Err::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                            Adt118::C1(v5) => {
                                Ok::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>((v4, List::cons(({
                                    let (_, h3253) = v3;
                                    h3253
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f947(v0: Adt166) -> Adt166 {
    let m3257 = v0;
    match m3257 {
        Adt166::C0 => {
            Adt166::C2
        }
        Adt166::C1 => {
            Adt166::C1
        }
        Adt166::C2 => {
            Adt166::C0
        }
    }
}

pub fn f948(v0: i64, v1: i64) -> R<i64> {
    f491(f393(f950(v0)?, 0i64)?, v1)
}

pub fn f949(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt166> {
    let a3258 = v1;
    let a3259 = v3;
    if nat_lt(a3258, a3259) {
        Ok(f817(f989({
            let a3260 = v3;
            let a3261 = v1;
            nat_sub(a3260, a3261)
        }, v0)?, v2))
    } else {
        let a3262 = v3;
        let a3263 = v1;
        if nat_lt(a3262, a3263) {
            Ok(f817(v0, f989({
                let a3264 = v1;
                let a3265 = v3;
                nat_sub(a3264, a3265)
            }, v2)?))
        } else {
            Ok(f817(v0, v2))
        }
    }
}

pub fn f950(v0: i64) -> R<u64> {
    f990(20u64, v0, 0u64)
}

pub fn f951(v0: u64) -> bool {
    let a3266 = v0;
    let a3267 = 9u64;
    if nat_eq(a3266, a3267) {
        true
    } else {
        let a3268 = v0;
        let a3269 = 10u64;
        if nat_eq(a3268, a3269) {
            true
        } else {
            let a3270 = v0;
            let a3271 = 13u64;
            nat_eq(a3270, a3271)
        }
    }
}

pub fn f952(p3274: List<Adt46>, p3275: List<Adt9>) -> List<Adt9> {
    let m3273 = (p3274, p3275);
    {
        let mut m3273 = m3273;
        loop {
            let step = {
                let (v0, v1) = m3273;
                let m3272 = v0;
                match m3272.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C3(f991(v2)), v1)))
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

pub fn f953(v0: Adt5) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("src"), Adt9::C3(match v0.clone() {
        Adt5::C0(h3276, _, _, _) => {
            h3276
        }
    })), {
        let a3282 = f733(Str::lit("mimeType"), match v0.clone() {
            Adt5::C0(_, h3277, _, _) => {
                h3277
            }
        });
        let a3283 = {
            let a3280 = f992(match v0.clone() {
                Adt5::C0(_, _, h3278, _) => {
                    h3278
                }
            });
            let a3281 = f993(match v0 {
                Adt5::C0(_, _, _, h3279) => {
                    h3279
                }
            });
            append_list(a3280, a3281)
        };
        append_list(a3282, a3283)
    }))
}

pub fn f954(v0: i64) -> u64 {
    let m3286 = {
        let a3284 = v0;
        let a3285 = 1i64;
        compare(a3284, a3285)
    };
    if match m3286 {
        Ordering::Less => {
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
        let m3289 = {
            let a3287 = v0;
            let a3288 = 2i64;
            compare(a3287, a3288)
        };
        if match m3289 {
            Ordering::Less => {
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
            let m3292 = {
                let a3290 = v0;
                let a3291 = 3i64;
                compare(a3290, a3291)
            };
            if match m3292 {
                Ordering::Less => {
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
                let m3295 = {
                    let a3293 = v0;
                    let a3294 = 4i64;
                    compare(a3293, a3294)
                };
                if match m3295 {
                    Ordering::Less => {
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
                    let m3298 = {
                        let a3296 = v0;
                        let a3297 = 5i64;
                        compare(a3296, a3297)
                    };
                    if match m3298 {
                        Ordering::Less => {
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
                        let m3301 = {
                            let a3299 = v0;
                            let a3300 = 6i64;
                            compare(a3299, a3300)
                        };
                        if match m3301 {
                            Ordering::Less => {
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
                            let m3304 = {
                                let a3302 = v0;
                                let a3303 = 7i64;
                                compare(a3302, a3303)
                            };
                            if match m3304 {
                                Ordering::Less => {
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
                                let m3307 = {
                                    let a3305 = v0;
                                    let a3306 = 8i64;
                                    compare(a3305, a3306)
                                };
                                if match m3307 {
                                    Ordering::Less => {
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
                                    let m3310 = {
                                        let a3308 = v0;
                                        let a3309 = 9i64;
                                        compare(a3308, a3309)
                                    };
                                    if match m3310 {
                                        Ordering::Less => {
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

pub fn f955(v0: Adt11) -> Adt9 {
    Adt9::C5(f391(Str::lit("listChanged"), match v0 {
        Adt11::C0(h3311) => {
            h3311
        }
    }))
}

pub fn f956(v0: Adt40) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt40::C0(h3312, _, _, _, _, _, _, _) => {
            h3312
        }
    })), {
        let a3330 = f733(Str::lit("title"), match v0.clone() {
            Adt40::C0(_, h3313, _, _, _, _, _, _) => {
                h3313
            }
        });
        let a3331 = {
            let a3328 = f734(match v0.clone() {
                Adt40::C0(_, _, h3314, _, _, _, _, _) => {
                    h3314
                }
            });
            let a3329 = {
                let a3326 = f733(Str::lit("description"), match v0.clone() {
                    Adt40::C0(_, _, _, h3315, _, _, _, _) => {
                        h3315
                    }
                });
                let a3327 = List::cons((Str::lit("inputSchema"), Adt9::C5({
                    let m3318 = match v0.clone() {
                        Adt40::C0(_, _, _, _, h3317, _, _, _) => {
                            h3317
                        }
                    };
                    match m3318 {
                        Adt8::C0(h3316) => {
                            h3316
                        }
                    }
                })), {
                    let a3324 = f827(Str::lit("outputSchema"), match v0.clone() {
                        Adt40::C0(_, _, _, _, _, h3319, _, _) => {
                            h3319
                        }
                    });
                    let a3325 = {
                        let a3322 = f994(match v0.clone() {
                            Adt40::C0(_, _, _, _, _, _, h3320, _) => {
                                h3320
                            }
                        });
                        let a3323 = f732(match v0 {
                            Adt40::C0(_, _, _, _, _, _, _, h3321) => {
                                h3321
                            }
                        });
                        append_list(a3322, a3323)
                    };
                    append_list(a3324, a3325)
                });
                append_list(a3326, a3327)
            };
            append_list(a3328, a3329)
        };
        append_list(a3330, a3331)
    }))
}

pub fn f957(v0: Adt49) -> R<Adt9> {
    Ok(Adt9::C5({
        let a3355 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt49::C0(h3332, _, _, _, _, _, _, _, _) => {
                h3332
            }
        })), List::<(Str, Adt9)>::nil());
        let a3356 = {
            let a3353 = f733(Str::lit("title"), match v0.clone() {
                Adt49::C0(_, h3333, _, _, _, _, _, _, _) => {
                    h3333
                }
            });
            let a3354 = {
                let a3351 = f734(match v0.clone() {
                    Adt49::C0(_, _, h3334, _, _, _, _, _, _) => {
                        h3334
                    }
                });
                let a3352 = {
                    let a3349 = List::cons((Str::lit("uri"), Adt9::C3(match v0.clone() {
                        Adt49::C0(_, _, _, h3335, _, _, _, _, _) => {
                            h3335
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a3350 = {
                        let a3347 = f733(Str::lit("description"), match v0.clone() {
                            Adt49::C0(_, _, _, _, h3336, _, _, _, _) => {
                                h3336
                            }
                        });
                        let a3348 = {
                            let a3345 = f733(Str::lit("mimeType"), match v0.clone() {
                                Adt49::C0(_, _, _, _, _, h3337, _, _, _) => {
                                    h3337
                                }
                            });
                            let a3346 = {
                                let a3343 = f731(match v0.clone() {
                                    Adt49::C0(_, _, _, _, _, _, h3338, _, _) => {
                                        h3338
                                    }
                                });
                                let a3344 = {
                                    let a3341 = f735(match v0.clone() {
                                        Adt49::C0(_, _, _, _, _, _, _, h3339, _) => {
                                            h3339
                                        }
                                    })?;
                                    let a3342 = f732(match v0 {
                                        Adt49::C0(_, _, _, _, _, _, _, _, h3340) => {
                                            h3340
                                        }
                                    });
                                    append_list(a3341, a3342)
                                };
                                append_list(a3343, a3344)
                            };
                            append_list(a3345, a3346)
                        };
                        append_list(a3347, a3348)
                    };
                    append_list(a3349, a3350)
                };
                append_list(a3351, a3352)
            };
            append_list(a3353, a3354)
        };
        append_list(a3355, a3356)
    }))
}

pub fn f958(v0: Adt59) -> Adt9 {
    Adt9::C5({
        let a3377 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt59::C0(h3357, _, _, _, _, _, _, _) => {
                h3357
            }
        })), List::<(Str, Adt9)>::nil());
        let a3378 = {
            let a3375 = f733(Str::lit("title"), match v0.clone() {
                Adt59::C0(_, h3358, _, _, _, _, _, _) => {
                    h3358
                }
            });
            let a3376 = {
                let a3373 = f734(match v0.clone() {
                    Adt59::C0(_, _, h3359, _, _, _, _, _) => {
                        h3359
                    }
                });
                let a3374 = {
                    let a3371 = List::cons((Str::lit("uriTemplate"), Adt9::C3(match v0.clone() {
                        Adt59::C0(_, _, _, h3360, _, _, _, _) => {
                            h3360
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a3372 = {
                        let a3369 = f733(Str::lit("description"), match v0.clone() {
                            Adt59::C0(_, _, _, _, h3361, _, _, _) => {
                                h3361
                            }
                        });
                        let a3370 = {
                            let a3367 = f733(Str::lit("mimeType"), match v0.clone() {
                                Adt59::C0(_, _, _, _, _, h3362, _, _) => {
                                    h3362
                                }
                            });
                            let a3368 = {
                                let a3365 = f731(match v0.clone() {
                                    Adt59::C0(_, _, _, _, _, _, h3363, _) => {
                                        h3363
                                    }
                                });
                                let a3366 = f732(match v0 {
                                    Adt59::C0(_, _, _, _, _, _, _, h3364) => {
                                        h3364
                                    }
                                });
                                append_list(a3365, a3366)
                            };
                            append_list(a3367, a3368)
                        };
                        append_list(a3369, a3370)
                    };
                    append_list(a3371, a3372)
                };
                append_list(a3373, a3374)
            };
            append_list(a3375, a3376)
        };
        append_list(a3377, a3378)
    })
}

pub fn f959(v0: Adt61) -> Adt9 {
    Adt9::C5({
        let a3394 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt61::C0(h3379, _, _, _, _, _) => {
                h3379
            }
        })), List::<(Str, Adt9)>::nil());
        let a3395 = {
            let a3392 = f733(Str::lit("title"), match v0.clone() {
                Adt61::C0(_, h3380, _, _, _, _) => {
                    h3380
                }
            });
            let a3393 = {
                let a3390 = f734(match v0.clone() {
                    Adt61::C0(_, _, h3381, _, _, _) => {
                        h3381
                    }
                });
                let a3391 = {
                    let a3388 = f733(Str::lit("description"), match v0.clone() {
                        Adt61::C0(_, _, _, h3382, _, _) => {
                            h3382
                        }
                    });
                    let a3389 = {
                        let a3386 = {
                            let m3384 = match v0.clone() {
                                Adt61::C0(_, _, _, _, h3383, _) => {
                                    h3383
                                }
                            };
                            match m3384 {
                                None => {
                                    List::<(Str, Adt9)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt9::C4(f995(v1))), List::<(Str, Adt9)>::nil())
                                }
                            }
                        };
                        let a3387 = f732(match v0 {
                            Adt61::C0(_, _, _, _, _, h3385) => {
                                h3385
                            }
                        });
                        append_list(a3386, a3387)
                    };
                    append_list(a3388, a3389)
                };
                append_list(a3390, a3391)
            };
            append_list(a3392, a3393)
        };
        append_list(a3394, a3395)
    })
}

pub fn f960(v0: Adt64) -> R<Option<Adt9>> {
    let m3397 = f553(match v0.clone() {
        Adt64::C0(_, h3396) => {
            h3396
        }
    })?;
    match m3397 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("role"), Adt9::C3(f991(match v0 {
                Adt64::C0(h3398, _) => {
                    h3398
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt9)>::nil())))))
        }
    }
}

pub fn f961(v0: Str, v1: Option<Adt9>) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(v0)), f390(Str::lit("params"), v1)))
}

pub fn f962(v0: Adt70) -> Adt9 {
    let m3399 = v0;
    match m3399 {
        Adt70::C0(v1) => {
            f996(v1)
        }
        Adt70::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("mode"), Adt9::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt9::C3(match v2.clone() {
                Adt73::C0(h3400, _) => {
                    h3400
                }
            })), List::cons((Str::lit("url"), Adt9::C3(match v2 {
                Adt73::C0(_, h3401) => {
                    h3401
                }
            })), List::<(Str, Adt9)>::nil()))))
        }
    }
}

pub fn f963(v0: Adt74) -> R<Option<Adt9>> {
    let m3403 = f997(match v0.clone() {
        Adt74::C0(h3402, _, _, _, _, _, _, _, _, _) => {
            h3402
        }
    })?;
    match m3403 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("messages"), Adt9::C4(v1)), {
                let a3405 = f998(v0.clone());
                let a3406 = List::cons((Str::lit("maxTokens"), f113(match v0.clone() {
                    Adt74::C0(_, _, _, _, _, h3404, _, _, _, _) => {
                        h3404
                    }
                })?), f999(v0));
                append_list(a3405, a3406)
            }))))
        }
    }
}

pub fn f964(p3412: List<(Str, Adt69)>, p3413: List<(Str, Adt9)>) -> Option<List<(Str, Adt9)>> {
    let m3411 = (p3412, p3413);
    {
        let mut m3411 = m3411;
        loop {
            let step = {
                let (v0, v1) = m3411;
                let m3407 = v0;
                match m3407.uncons() {
                    None => {
                        Err::<(List<(Str, Adt69)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m3409 = f1000({
                            let (_, h3408) = v2;
                            h3408
                        });
                        match m3409 {
                            None => {
                                Err::<(List<(Str, Adt69)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>(None::<List<(Str, Adt9)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt69)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>((v3, List::cons(({
                                    let (h3410, _) = v2;
                                    h3410
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f965(v0: i64) -> Adt9 {
    Adt9::C2(Adt10::C0(v0, 0i64))
}

pub fn f966(v0: Adt89) -> Option<Adt9> {
    let m3415 = f754(match v0.clone() {
        Adt89::C0(h3414, _, _) => {
            h3414
        }
    });
    match m3415 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            let m3417 = f1001(match v0.clone() {
                Adt89::C0(_, _, h3416) => {
                    h3416
                }
            });
            match m3417 {
                None => {
                    None::<Adt9>
                }
                Some(v2) => {
                    Some(Adt9::C5(List::cons((Str::lit("task"), Adt9::C5(v1)), List::cons((Str::lit("createdMs"), f965(match v0 {
                        Adt89::C0(_, h3418, _) => {
                            h3418
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt9)>::nil())))))
                }
            }
        }
    }
}

pub fn f967(v0: List<u64>) -> List<u64> {
    let a3421 = f1002();
    let a3422 = {
        let a3419 = v0;
        let a3420 = f1003();
        append_list(a3419, a3420)
    };
    append_list(a3421, a3422)
}

pub fn f968(p3429: List<u64>) -> bool {
    let m3428 = p3429;
    {
        let mut m3428 = m3428;
        loop {
            let step = {
                let v0 = m3428;
                let m3423 = v0;
                match m3423.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a3424 = v1;
                        let a3425 = 48u64;
                        if if nat_eq(a3424, a3425) {
                            true
                        } else {
                            let a3426 = v1;
                            let a3427 = 46u64;
                            nat_eq(a3426, a3427)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f969(v0: Adt159) -> bool {
    let m3430 = v0;
    match m3430 {
        Adt159::C0 => {
            true
        }
        Adt159::C1 => {
            false
        }
        Adt159::C2 => {
            false
        }
        Adt159::C3 => {
            false
        }
        Adt159::C4 => {
            false
        }
        Adt159::C5 => {
            false
        }
        Adt159::C6 => {
            false
        }
        Adt159::C7 => {
            false
        }
        Adt159::C8 => {
            false
        }
        Adt159::C9 => {
            false
        }
        Adt159::C10 => {
            false
        }
        Adt159::C11 => {
            false
        }
    }
}

pub fn f970(v0: Adt19) -> Adt159 {
    let m3431 = v0;
    match m3431 {
        Adt19::C0(_) => {
            Adt159::C0
        }
        Adt19::C1(_) => {
            Adt159::C1
        }
        Adt19::C2(_) => {
            Adt159::C2
        }
        Adt19::C3(_) => {
            Adt159::C3
        }
        Adt19::C4(_) => {
            Adt159::C4
        }
        Adt19::C5(_) => {
            Adt159::C5
        }
        Adt19::C6(_) => {
            Adt159::C6
        }
        Adt19::C7(_) => {
            Adt159::C7
        }
        Adt19::C8(_) => {
            Adt159::C8
        }
        Adt19::C9(_) => {
            Adt159::C9
        }
        Adt19::C10(_) => {
            Adt159::C10
        }
        Adt19::C11(_) => {
            Adt159::C11
        }
    }
}

pub fn f971(v0: Adt3, v1: Adt37) -> Adt147 {
    Adt147::C0(v1, Adt148::C0(Some(match v0 {
        Adt3::C0(h3432, _, _, _, _, _, _, _) => {
            h3432
        }
    }), None::<Adt8>), List::<Adt149>::nil())
}

pub fn f972(v0: Adt3) -> Adt38 {
    Adt38::C0(match v0.clone() {
        Adt3::C0(_, _, _, h3433, _, _, _, _) => {
            h3433
        }
    }, f526(), match v0.clone() {
        Adt3::C0(_, _, h3434, _, _, _, _, _) => {
            h3434
        }
    }, match v0 {
        Adt3::C0(_, h3435, _, _, _, _, _, _) => {
            h3435
        }
    })
}

pub fn f973(v0: Adt3, v1: Adt19, v2: Result<Adt37, Adt94>) -> Result<Adt147, Adt150> {
    let m3436 = v2;
    match m3436 {
        Ok(v4) => {
            if f1004(f970(v1.clone()), v4.clone()) {
                f1005(v0, v1, v4)
            } else {
                Err::<Adt147, Adt150>(Adt150::C4(f1006()))
            }
        }
        Err(v3) => {
            Err::<Adt147, Adt150>(Adt150::C4(v3))
        }
    }
}

pub fn f974(v0: Bytes, v1: u64, v2: u64, v3: Fn9, v4: Adt19) -> R<Result<Adt37, Adt94>> {
    f1007(v0.clone(), v1, v2, v3, v4.clone(), f1008(v0, v4)?)
}

pub fn f975(v0: Str, v1: Adt8) -> Result<Option<Str>, Adt151> {
    let m3437 = f275(v0.clone(), v1);
    match m3437 {
        Adt118::C0 => {
            Ok::<Option<Str>, Adt151>(None::<Str>)
        }
        Adt118::C1(v2) => {
            let m3438 = f181(v2);
            match m3438 {
                None => {
                    Err::<Option<Str>, Adt151>(Adt151::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt151>(Some(v3))
                }
            }
        }
    }
}

pub fn f976(v0: Str, v1: Adt8) -> Result<Str, Adt151> {
    let m3439 = f275(v0.clone(), v1);
    match m3439 {
        Adt118::C0 => {
            Err::<Str, Adt151>(Adt151::C7(v0))
        }
        Adt118::C1(v2) => {
            let m3440 = f181(v2);
            match m3440 {
                None => {
                    Err::<Str, Adt151>(Adt151::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt151>(v3)
                }
            }
        }
    }
}

pub fn f977(v0: Str, v1: Adt8) -> Result<Option<Adt8>, Adt151> {
    let m3441 = f275(v0.clone(), v1);
    match m3441 {
        Adt118::C0 => {
            Ok::<Option<Adt8>, Adt151>(None::<Adt8>)
        }
        Adt118::C1(v2) => {
            let m3442 = f281(v2);
            match m3442 {
                None => {
                    Err::<Option<Adt8>, Adt151>(Adt151::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt8>, Adt151>(Some(v3))
                }
            }
        }
    }
}

pub fn f978(v0: Str, v1: Adt8) -> Result<Option<List<(Str, Str)>>, Adt151> {
    let m3443 = f275(v0.clone(), v1);
    match m3443 {
        Adt118::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt151>(None::<List<(Str, Str)>>)
        }
        Adt118::C1(v2) => {
            let m3444 = f281(v2);
            match m3444 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt151>(Adt151::C8(v0))
                }
                Some(v3) => {
                    let m3446 = f1009(v0, match v3 {
                        Adt8::C0(h3445) => {
                            h3445
                        }
                    });
                    match m3446 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt151>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt151>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f979(v0: Str, v1: Adt8) -> Result<Adt8, Adt151> {
    let m3447 = f275(v0.clone(), v1);
    match m3447 {
        Adt118::C0 => {
            Err::<Adt8, Adt151>(Adt151::C7(v0))
        }
        Adt118::C1(v2) => {
            let m3448 = f281(v2);
            match m3448 {
                None => {
                    Err::<Adt8, Adt151>(Adt151::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt8, Adt151>(v3)
                }
            }
        }
    }
}

pub fn f980(v0: Adt8) -> Result<Adt30, Adt151> {
    let m3449 = f976(Str::lit("type"), v0.clone());
    match m3449 {
        Ok(v1) => {
            let a3450 = v1.clone();
            let a3451 = Str::lit("ref/prompt");
            if equal(a3450, a3451) {
                let m3452 = f976(Str::lit("name"), v0.clone());
                match m3452 {
                    Ok(v2) => {
                        let m3453 = f975(Str::lit("title"), v0);
                        match m3453 {
                            Ok(v3) => {
                                Ok::<Adt30, Adt151>(Adt30::C0(Adt31::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt30, Adt151>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt30, Adt151>(v5)
                    }
                }
            } else {
                let a3454 = v1;
                let a3455 = Str::lit("ref/resource");
                if equal(a3454, a3455) {
                    let m3456 = f976(Str::lit("uri"), v0);
                    match m3456 {
                        Ok(v6) => {
                            Ok::<Adt30, Adt151>(Adt30::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt30, Adt151>(v7)
                        }
                    }
                } else {
                    Err::<Adt30, Adt151>(Adt151::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt30, Adt151>(v8)
        }
    }
}

pub fn f981(v0: Adt8) -> Result<Adt32, Adt151> {
    let m3457 = f976(Str::lit("name"), v0.clone());
    match m3457 {
        Ok(v1) => {
            let m3458 = f976(Str::lit("value"), v0);
            match m3458 {
                Ok(v2) => {
                    Ok::<Adt32, Adt151>(Adt32::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt32, Adt151>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt32, Adt151>(v4)
        }
    }
}

pub fn f982(v0: Adt8) -> Result<Option<Adt33>, Adt151> {
    let m3459 = f275(Str::lit("context"), v0);
    match m3459 {
        Adt118::C0 => {
            Ok::<Option<Adt33>, Adt151>(None::<Adt33>)
        }
        Adt118::C1(v1) => {
            let m3460 = f281(v1);
            match m3460 {
                None => {
                    Err::<Option<Adt33>, Adt151>(Adt151::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m3461 = f978(Str::lit("arguments"), v2);
                    match m3461 {
                        Ok(v3) => {
                            Ok::<Option<Adt33>, Adt151>(Some(Adt33::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt33>, Adt151>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f983(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt150> {
    if v0 {
        None::<Adt150>
    } else {
        Some(Adt150::C1(v1, v2))
    }
}

pub fn f984(v0: Str, p3466: List<Str>) -> bool {
    let m3465 = p3466;
    {
        let mut m3465 = m3465;
        loop {
            let step = {
                let v1 = m3465;
                let m3462 = v1;
                match m3462.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3463 = v2;
                        let a3464 = v0.clone();
                        if equal(a3463, a3464) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f985(v0: Option<Adt133>, v1: List<Adt135>, v2: Adt8) -> R<Option<Adt133>> {
    let m3467 = v0;
    match m3467 {
        None => {
            f1010(v1, f1011(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f986(v0: Option<Str>, v1: Adt164, v2: Adt8) -> Option<Adt133> {
    let m3468 = v1.clone();
    match m3468 {
        Adt164::C0 => {
            None::<Adt133>
        }
        Adt164::C1 => {
            f1012(v0, f1013(v1, v2))
        }
        Adt164::C2 => {
            f1012(v0, f1013(v1, v2))
        }
        Adt164::C3 => {
            f1012(v0, f1013(v1, v2))
        }
    }
}

pub fn f987(v0: Adt118, v1: Adt118) -> Adt170 {
    let m3469 = v0;
    match m3469 {
        Adt118::C0 => {
            Adt170::C1
        }
        Adt118::C1(_) => {
            let m3470 = v1;
            match m3470 {
                Adt118::C0 => {
                    Adt170::C2
                }
                Adt118::C1(_) => {
                    Adt170::C0
                }
            }
        }
    }
}

pub fn f988(v0: Adt8, v1: Adt9, v2: Adt9) -> Result<Adt21, Adt151> {
    let m3471 = f181(v1);
    match m3471 {
        None => {
            Err::<Adt21, Adt151>(Adt151::C5)
        }
        Some(v3) => {
            let m3472 = f1014(v2);
            match m3472 {
                None => {
                    Err::<Adt21, Adt151>(Adt151::C6)
                }
                Some(v4) => {
                    let m3473 = f977(f1015(), v0.clone());
                    match m3473 {
                        Ok(v6) => {
                            let m3474 = f1016(v0.clone());
                            match m3474 {
                                Ok(v8) => {
                                    let m3475 = f1017(v0.clone());
                                    match m3475 {
                                        Ok(v10) => {
                                            Ok::<Adt21, Adt151>(Adt21::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt21, Adt151>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt21, Adt151>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt21, Adt151>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f989(p3480: u64, p3481: i64) -> R<i64> {
    let m3479 = (p3480, p3481);
    {
        let mut m3479 = m3479;
        loop {
            let step = {
                let (v0, v1) = m3479;
                let m3476 = v0;
                if m3476 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3476 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3477 = v1;
                        let a3478 = 10i64;
                        int_mul(a3477, a3478)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f990(p3492: u64, p3493: i64, p3494: u64) -> R<u64> {
    let m3491 = (p3492, (p3493, p3494));
    {
        let mut m3491 = m3491;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3491;
                let m3482 = v0;
                if m3482 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3482 - 1;
                    let v4: u64 = {
                        let a3483 = v2;
                        let a3484 = 1u64;
                        nat_add(a3483, a3484)?
                    };
                    let m3487 = {
                        let a3485 = v1;
                        let a3486 = 10i64;
                        compare(a3485, a3486)
                    };
                    if match m3487 {
                        Ordering::Less => {
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
                            let a3488 = v1;
                            let a3489 = 10i64;
                            let a3490 = 0i64;
                            int_quot(a3488, a3489, a3490)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f991(v0: Adt46) -> Str {
    let m3495 = v0;
    match m3495 {
        Adt46::C0 => {
            Str::lit("user")
        }
        Adt46::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f992(v0: Option<List<Str>>) -> List<(Str, Adt9)> {
    let m3496 = v0;
    match m3496 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt9::C4(f774(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f993(v0: Option<Adt6>) -> List<(Str, Adt9)> {
    let m3497 = v0;
    match m3497 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt9::C3(f1018(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f994(v0: Option<Adt41>) -> List<(Str, Adt9)> {
    let m3498 = v0;
    match m3498 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt9::C5({
                let a3510 = f733(Str::lit("title"), match v1.clone() {
                    Adt41::C0(h3499, _, _, _, _) => {
                        h3499
                    }
                });
                let a3511 = {
                    let a3508 = f391(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt41::C0(_, h3500, _, _, _) => {
                            h3500
                        }
                    });
                    let a3509 = {
                        let a3506 = f391(Str::lit("destructiveHint"), match v1.clone() {
                            Adt41::C0(_, _, h3501, _, _) => {
                                h3501
                            }
                        });
                        let a3507 = {
                            let a3504 = f391(Str::lit("idempotentHint"), match v1.clone() {
                                Adt41::C0(_, _, _, h3502, _) => {
                                    h3502
                                }
                            });
                            let a3505 = f391(Str::lit("openWorldHint"), match v1 {
                                Adt41::C0(_, _, _, _, h3503) => {
                                    h3503
                                }
                            });
                            append_list(a3504, a3505)
                        };
                        append_list(a3506, a3507)
                    };
                    append_list(a3508, a3509)
                };
                append_list(a3510, a3511)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f995(v0: List<Adt62>) -> List<Adt9> {
    f490(f1019(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f996(v0: Adt71) -> Adt9 {
    Adt9::C5({
        let a3518 = {
            let m3513 = match v0.clone() {
                Adt71::C0(h3512, _, _) => {
                    h3512
                }
            };
            match m3513 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt9::C3(Str::lit("form"))), List::<(Str, Adt9)>::nil())
                }
            }
        };
        let a3519 = List::cons((Str::lit("message"), Adt9::C3(match v0.clone() {
            Adt71::C0(_, h3514, _) => {
                h3514
            }
        })), List::cons((Str::lit("requestedSchema"), Adt9::C5({
            let m3517 = match v0 {
                Adt71::C0(_, _, h3516) => {
                    h3516
                }
            };
            match m3517 {
                Adt8::C0(h3515) => {
                    h3515
                }
            }
        })), List::<(Str, Adt9)>::nil()));
        append_list(a3518, a3519)
    })
}

pub fn f997(v0: List<Adt75>) -> R<Option<List<Adt9>>> {
    let m3520 = f1020(v0, List::<Adt9>::nil())?;
    match m3520 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f490(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f998(v0: Adt74) -> List<(Str, Adt9)> {
    let a3531 = {
        let m3522 = match v0.clone() {
            Adt74::C0(_, h3521, _, _, _, _, _, _, _, _) => {
                h3521
            }
        };
        match m3522 {
            None => {
                List::<(Str, Adt9)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f1021(v1)), List::<(Str, Adt9)>::nil())
            }
        }
    };
    let a3532 = {
        let a3529 = f733(Str::lit("systemPrompt"), match v0.clone() {
            Adt74::C0(_, _, h3523, _, _, _, _, _, _, _) => {
                h3523
            }
        });
        let a3530 = {
            let a3527 = {
                let m3525 = match v0.clone() {
                    Adt74::C0(_, _, _, h3524, _, _, _, _, _, _) => {
                        h3524
                    }
                };
                match m3525 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt9::C3(f1022(v2))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3528 = f1023(Str::lit("temperature"), match v0 {
                Adt74::C0(_, _, _, _, h3526, _, _, _, _, _) => {
                    h3526
                }
            });
            append_list(a3527, a3528)
        };
        append_list(a3529, a3530)
    };
    append_list(a3531, a3532)
}

pub fn f999(v0: Adt74) -> List<(Str, Adt9)> {
    let a3544 = {
        let m3534 = match v0.clone() {
            Adt74::C0(_, _, _, _, _, _, h3533, _, _, _) => {
                h3533
            }
        };
        match m3534 {
            None => {
                List::<(Str, Adt9)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f834(v1)), List::<(Str, Adt9)>::nil())
            }
        }
    };
    let a3545 = {
        let a3542 = f827(Str::lit("metadata"), match v0.clone() {
            Adt74::C0(_, _, _, _, _, _, _, h3535, _, _) => {
                h3535
            }
        });
        let a3543 = {
            let a3540 = {
                let m3537 = match v0.clone() {
                    Adt74::C0(_, _, _, _, _, _, _, _, h3536, _) => {
                        h3536
                    }
                };
                match m3537 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt9::C4(f837(v2))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3541 = {
                let m3539 = match v0 {
                    Adt74::C0(_, _, _, _, _, _, _, _, _, h3538) => {
                        h3538
                    }
                };
                match m3539 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt9::C5(f1024(v3))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            append_list(a3540, a3541)
        };
        append_list(a3542, a3543)
    };
    append_list(a3544, a3545)
}

pub fn f1000(v0: Adt69) -> Option<Adt9> {
    let m3546 = v0;
    match m3546 {
        Adt69::C0(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt9::C5(f1025(v1))), List::<(Str, Adt9)>::nil()))))
        }
        Adt69::C1(_) => {
            None::<Adt9>
        }
        Adt69::C2(v3) => {
            Some(Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(Str::lit("roots/list"))), {
                let m3547 = v3;
                match m3547 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt9::C5(match v4 {
                            Adt8::C0(h3548) => {
                                h3548
                            }
                        })), List::<(Str, Adt9)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f1001(v0: Adt90) -> Option<Adt9> {
    let m3550 = f910(match v0.clone() {
        Adt90::C0(_, h3549, _, _) => {
            h3549
        }
    });
    match m3550 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("parkAfterMs"), f965(match v0.clone() {
                Adt90::C0(h3551, _, _, _) => {
                    h3551
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f965(match v0.clone() {
                Adt90::C0(_, _, h3552, _) => {
                    h3552
                }
            })), List::cons((Str::lit("outcome"), f1026(match v0 {
                Adt90::C0(_, _, _, h3553) => {
                    h3553
                }
            })), List::<(Str, Adt9)>::nil()))))))
        }
    }
}

pub fn f1002() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f1003() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f1004(v0: Adt159, v1: Adt37) -> bool {
    let m3554 = v1;
    match m3554 {
        Adt37::C0(_) => {
            f1027(v0, Adt159::C0)
        }
        Adt37::C1(_) => {
            f1027(v0, Adt159::C1)
        }
        Adt37::C2(_) => {
            f1027(v0, Adt159::C2)
        }
        Adt37::C3(_) => {
            f1027(v0, Adt159::C2)
        }
        Adt37::C4(_) => {
            f1027(v0, Adt159::C3)
        }
        Adt37::C5(_) => {
            f1027(v0, Adt159::C4)
        }
        Adt37::C6(_) => {
            f1027(v0, Adt159::C5)
        }
        Adt37::C7(_) => {
            f1027(v0, Adt159::C6)
        }
        Adt37::C8(_) => {
            f1027(v0, Adt159::C7)
        }
        Adt37::C9(_) => {
            f1027(v0, Adt159::C8)
        }
        Adt37::C10(_) => {
            f1027(v0, Adt159::C9)
        }
        Adt37::C11 => {
            if f1027(v0.clone(), Adt159::C10) {
                true
            } else {
                f1027(v0, Adt159::C11)
            }
        }
        Adt37::C12(_) => {
            f1028(v0)
        }
        Adt37::C13(_) => {
            f1029(v0)
        }
        Adt37::C14(v15) => {
            let m3556 = match v15 {
                Adt92::C0(_, h3555, _) => {
                    h3555
                }
            };
            match m3556 {
                Adt93::C0 => {
                    f1027(v0, Adt159::C9)
                }
                Adt93::C1 => {
                    if f1027(v0.clone(), Adt159::C10) {
                        true
                    } else {
                        f1027(v0, Adt159::C11)
                    }
                }
            }
        }
    }
}

pub fn f1005(v0: Adt3, v1: Adt19, v2: Adt37) -> Result<Adt147, Adt150> {
    let m3557 = v2.clone();
    match m3557 {
        Adt37::C0(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C1(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C2(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C3(v9) => {
            Ok::<Adt147, Adt150>(f1034(v0, f1031(v1), v9))
        }
        Adt37::C4(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C5(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C6(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C7(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C8(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C9(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C10(_) => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C11 => {
            Ok::<Adt147, Adt150>(f971(v0, v2))
        }
        Adt37::C12(v3) => {
            f1030(v0, f1031(v1), v3)
        }
        Adt37::C13(v4) => {
            f1032(v0, f1031(v1), v4)
        }
        Adt37::C14(v5) => {
            f1033(v0, f1031(v1), v5)
        }
    }
}

pub fn f1006() -> Adt94 {
    Adt94::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f1007(v0: Bytes, v1: u64, v2: u64, v3: Fn9, v4: Adt19, v5: Result<Adt19, Adt94>) -> R<Result<Adt37, Adt94>> {
    let m3558 = v5;
    match m3558 {
        Ok(v7) => {
            f1035(v0, v4, f1036(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt37, Adt94>(v6))
        }
    }
}

pub fn f1008(v0: Bytes, v1: Adt19) -> R<Result<Adt19, Adt94>> {
    let m3559 = v1.clone();
    match m3559 {
        Adt19::C0(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
        Adt19::C1(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
        Adt19::C2(v2) => {
            f1037(v0, v2)
        }
        Adt19::C3(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
        Adt19::C4(v3) => {
            f1038(v0, v3)
        }
        Adt19::C5(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
        Adt19::C6(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
        Adt19::C7(v4) => {
            f1039(v0, v4)
        }
        Adt19::C8(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
        Adt19::C9(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
        Adt19::C10(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
        Adt19::C11(_) => {
            Ok(Ok::<Adt19, Adt94>(v1))
        }
    }
}

pub fn f1009(v0: Str, v1: List<(Str, Adt9)>) -> Result<List<(Str, Str)>, Adt151> {
    let m3560 = f1040(v0, v1, List::<(Str, Str)>::nil());
    match m3560 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt151>(f1041(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt151>(v3)
        }
    }
}

pub fn f1010(p3564: List<Adt135>, v1: Adt8) -> R<Option<Adt133>> {
    let m3563 = p3564;
    {
        let mut m3563 = m3563;
        loop {
            let step = {
                let v0 = m3563;
                let m3561 = v0;
                match m3561.uncons() {
                    None => {
                        Err::<List<Adt135>, R<Option<Adt133>>>(Ok(None::<Adt133>))
                    }
                    Some((v2, v3)) => {
                        let m3562 = f1042(v2, v1.clone())?;
                        match m3562 {
                            None => {
                                Ok::<List<Adt135>, R<Option<Adt133>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt135>, R<Option<Adt133>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1011(v0: Adt8) -> Adt8 {
    let m3565 = f275(Str::lit("arguments"), v0);
    match m3565 {
        Adt118::C0 => {
            f32()
        }
        Adt118::C1(v1) => {
            let m3566 = f281(v1);
            match m3566 {
                None => {
                    f32()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f1012(v0: Option<Str>, v1: Option<Str>) -> Option<Adt133> {
    let m3567 = v0;
    match m3567 {
        None => {
            Some(Adt133::C2)
        }
        Some(v2) => {
            let m3568 = v1;
            match m3568 {
                None => {
                    None::<Adt133>
                }
                Some(v3) => {
                    let a3569 = v2;
                    let a3570 = v3;
                    if equal(a3569, a3570) {
                        None::<Adt133>
                    } else {
                        Some(Adt133::C5)
                    }
                }
            }
        }
    }
}

pub fn f1013(v0: Adt164, v1: Adt8) -> Option<Str> {
    let m3571 = v0;
    match m3571 {
        Adt164::C0 => {
            None::<Str>
        }
        Adt164::C1 => {
            let m3572 = f275(Str::lit("name"), v1);
            match m3572 {
                Adt118::C0 => {
                    None::<Str>
                }
                Adt118::C1(v2) => {
                    f181(v2)
                }
            }
        }
        Adt164::C2 => {
            let m3573 = f275(Str::lit("uri"), v1);
            match m3573 {
                Adt118::C0 => {
                    None::<Str>
                }
                Adt118::C1(v3) => {
                    f181(v3)
                }
            }
        }
        Adt164::C3 => {
            let m3574 = f275(Str::lit("taskId"), v1);
            match m3574 {
                Adt118::C0 => {
                    None::<Str>
                }
                Adt118::C1(v4) => {
                    f181(v4)
                }
            }
        }
    }
}

pub fn f1014(v0: Adt9) -> Option<Adt22> {
    let m3575 = f281(v0);
    match m3575 {
        None => {
            None::<Adt22>
        }
        Some(v1) => {
            let m3576 = f1043(Str::lit("experimental"), v1.clone());
            match m3576 {
                None => {
                    None::<Adt22>
                }
                Some(v2) => {
                    let m3577 = f1043(Str::lit("roots"), v1.clone());
                    match m3577 {
                        None => {
                            None::<Adt22>
                        }
                        Some(v3) => {
                            let m3578 = f1043(Str::lit("sampling"), v1.clone());
                            match m3578 {
                                None => {
                                    None::<Adt22>
                                }
                                Some(v4) => {
                                    let m3579 = f1043(Str::lit("elicitation"), v1.clone());
                                    match m3579 {
                                        None => {
                                            None::<Adt22>
                                        }
                                        Some(v5) => {
                                            let m3580 = f1043(Str::lit("extensions"), v1);
                                            match m3580 {
                                                None => {
                                                    None::<Adt22>
                                                }
                                                Some(v6) => {
                                                    Some(Adt22::C0(v2, v3, v4, v5, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f1015() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f1016(v0: Adt8) -> Result<Option<Adt23>, Adt151> {
    let m3581 = f975(f1044(), v0);
    match m3581 {
        Ok(v2) => {
            let m3582 = v2;
            match m3582 {
                None => {
                    Ok::<Option<Adt23>, Adt151>(None::<Adt23>)
                }
                Some(v3) => {
                    let m3583 = f1045(v3);
                    match m3583 {
                        Adt171::C0 => {
                            Err::<Option<Adt23>, Adt151>(Adt151::C8(f1044()))
                        }
                        Adt171::C1(v4) => {
                            Ok::<Option<Adt23>, Adt151>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt23>, Adt151>(v1)
        }
    }
}

pub fn f1017(v0: Adt8) -> Result<Option<Adt24>, Adt151> {
    let m3584 = f275(f1046(), v0);
    match m3584 {
        Adt118::C0 => {
            Ok::<Option<Adt24>, Adt151>(None::<Adt24>)
        }
        Adt118::C1(v1) => {
            let m3585 = f181(v1.clone());
            match m3585 {
                None => {
                    let m3586 = f277(v1);
                    match m3586 {
                        None => {
                            Err::<Option<Adt24>, Adt151>(Adt151::C8(f1046()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt24>, Adt151>(Some(Adt24::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt24>, Adt151>(Some(Adt24::C0(v2)))
                }
            }
        }
    }
}

pub fn f1018(v0: Adt6) -> Str {
    let m3587 = v0;
    match m3587 {
        Adt6::C0 => {
            Str::lit("light")
        }
        Adt6::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f1019(p3590: List<Adt62>, p3591: List<Adt9>) -> List<Adt9> {
    let m3589 = (p3590, p3591);
    {
        let mut m3589 = m3589;
        loop {
            let step = {
                let (v0, v1) = m3589;
                let m3588 = v0;
                match m3588.uncons() {
                    None => {
                        Err::<(List<Adt62>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt62>, List<Adt9>), List<Adt9>>((v3, List::cons(f1047(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3589 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1020(p3595: List<Adt75>, p3596: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3594 = (p3595, p3596);
    {
        let mut m3594 = m3594;
        loop {
            let step = {
                let (v0, v1) = m3594;
                let m3592 = v0;
                match m3592.uncons() {
                    None => {
                        Err::<(List<Adt75>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3593 = f1048(v2)?;
                        match m3593 {
                            None => {
                                Err::<(List<Adt75>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(None::<List<Adt9>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt75>, List<Adt9>), R<Option<List<Adt9>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1021(v0: Adt80) -> Adt9 {
    Adt9::C5({
        let a3606 = {
            let m3598 = match v0.clone() {
                Adt80::C0(h3597, _, _, _) => {
                    h3597
                }
            };
            match m3598 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt9::C4(f490(f1049(v1, List::<Adt9>::nil()), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())
                }
            }
        };
        let a3607 = {
            let a3604 = f1023(Str::lit("costPriority"), match v0.clone() {
                Adt80::C0(_, h3599, _, _) => {
                    h3599
                }
            });
            let a3605 = {
                let a3602 = f1023(Str::lit("speedPriority"), match v0.clone() {
                    Adt80::C0(_, _, h3600, _) => {
                        h3600
                    }
                });
                let a3603 = f1023(Str::lit("intelligencePriority"), match v0 {
                    Adt80::C0(_, _, _, h3601) => {
                        h3601
                    }
                });
                append_list(a3602, a3603)
            };
            append_list(a3604, a3605)
        };
        append_list(a3606, a3607)
    })
}

pub fn f1022(v0: Adt82) -> Str {
    let m3608 = v0;
    match m3608 {
        Adt82::C0 => {
            Str::lit("none")
        }
        Adt82::C1 => {
            Str::lit("thisServer")
        }
        Adt82::C2 => {
            Str::lit("allServers")
        }
    }
}

pub fn f1023(v0: Str, v1: Option<Adt10>) -> List<(Str, Adt9)> {
    let m3609 = v1;
    match m3609 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C2(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f1024(v0: Adt83) -> List<(Str, Adt9)> {
    let m3611 = match v0 {
        Adt83::C0(h3610) => {
            h3610
        }
    };
    match m3611 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt9::C3(f1050(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f1025(v0: Adt70) -> List<(Str, Adt9)> {
    let m3612 = v0;
    match m3612 {
        Adt70::C0(v1) => {
            let a3619 = {
                let m3614 = match v1.clone() {
                    Adt71::C0(h3613, _, _) => {
                        h3613
                    }
                };
                match m3614 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt9::C3(Str::lit("form"))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3620 = List::cons((Str::lit("message"), Adt9::C3(match v1.clone() {
                Adt71::C0(_, h3615, _) => {
                    h3615
                }
            })), List::cons((Str::lit("requestedSchema"), Adt9::C5({
                let m3618 = match v1 {
                    Adt71::C0(_, _, h3617) => {
                        h3617
                    }
                };
                match m3618 {
                    Adt8::C0(h3616) => {
                        h3616
                    }
                }
            })), List::<(Str, Adt9)>::nil()));
            append_list(a3619, a3620)
        }
        Adt70::C1(v3) => {
            List::cons((Str::lit("mode"), Adt9::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt9::C3(match v3.clone() {
                Adt73::C0(h3621, _) => {
                    h3621
                }
            })), List::cons((Str::lit("url"), Adt9::C3(match v3 {
                Adt73::C0(_, h3622) => {
                    h3622
                }
            })), List::<(Str, Adt9)>::nil())))
        }
    }
}

pub fn f1026(v0: Adt91) -> Adt9 {
    let m3623 = v0;
    match m3623 {
        Adt91::C0(v1) => {
            Adt9::C5(List::cons((Str::lit("completes"), Adt9::C5(match v1 {
                Adt8::C0(h3624) => {
                    h3624
                }
            })), List::<(Str, Adt9)>::nil()))
        }
        Adt91::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("fails"), f442(v2)), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f1027(v0: Adt159, v1: Adt159) -> bool {
    let a3625 = f870(v0);
    let a3626 = f870(v1);
    equal(a3625, a3626)
}

pub fn f1028(v0: Adt159) -> bool {
    let m3627 = v0;
    match m3627 {
        Adt159::C0 => {
            false
        }
        Adt159::C1 => {
            false
        }
        Adt159::C2 => {
            true
        }
        Adt159::C3 => {
            false
        }
        Adt159::C4 => {
            true
        }
        Adt159::C5 => {
            false
        }
        Adt159::C6 => {
            false
        }
        Adt159::C7 => {
            true
        }
        Adt159::C8 => {
            false
        }
        Adt159::C9 => {
            false
        }
        Adt159::C10 => {
            false
        }
        Adt159::C11 => {
            false
        }
    }
}

pub fn f1029(v0: Adt159) -> bool {
    let m3628 = v0;
    match m3628 {
        Adt159::C0 => {
            false
        }
        Adt159::C1 => {
            false
        }
        Adt159::C2 => {
            true
        }
        Adt159::C3 => {
            false
        }
        Adt159::C4 => {
            false
        }
        Adt159::C5 => {
            false
        }
        Adt159::C6 => {
            false
        }
        Adt159::C7 => {
            false
        }
        Adt159::C8 => {
            false
        }
        Adt159::C9 => {
            false
        }
        Adt159::C10 => {
            false
        }
        Adt159::C11 => {
            false
        }
    }
}

pub fn f1030(v0: Adt3, v1: Adt21, v2: Adt86) -> Result<Adt147, Adt150> {
    f1051(v0, v2.clone(), f1052(match v1 {
        Adt21::C0(_, h3629, _, _, _, _) => {
            h3629
        }
    }, f845(v2)))
}

pub fn f1031(v0: Adt19) -> Adt21 {
    let m3630 = v0;
    match m3630 {
        Adt19::C0(v1) => {
            match v1 {
                Adt20::C0(h3631) => {
                    h3631
                }
            }
        }
        Adt19::C1(v2) => {
            match v2 {
                Adt25::C0(h3632, _) => {
                    h3632
                }
            }
        }
        Adt19::C2(v3) => {
            match v3 {
                Adt26::C0(h3633, _, _, _, _) => {
                    h3633
                }
            }
        }
        Adt19::C3(v4) => {
            match v4 {
                Adt25::C0(h3634, _) => {
                    h3634
                }
            }
        }
        Adt19::C4(v5) => {
            match v5 {
                Adt27::C0(h3635, _, _, _) => {
                    h3635
                }
            }
        }
        Adt19::C5(v6) => {
            match v6 {
                Adt25::C0(h3636, _) => {
                    h3636
                }
            }
        }
        Adt19::C6(v7) => {
            match v7 {
                Adt25::C0(h3637, _) => {
                    h3637
                }
            }
        }
        Adt19::C7(v8) => {
            match v8 {
                Adt28::C0(h3638, _, _, _, _) => {
                    h3638
                }
            }
        }
        Adt19::C8(v9) => {
            match v9 {
                Adt29::C0(h3639, _, _, _) => {
                    h3639
                }
            }
        }
        Adt19::C9(v10) => {
            match v10 {
                Adt34::C0(h3640, _) => {
                    h3640
                }
            }
        }
        Adt19::C10(v11) => {
            match v11 {
                Adt35::C0(h3641, _, _) => {
                    h3641
                }
            }
        }
        Adt19::C11(v12) => {
            match v12 {
                Adt36::C0(h3642, _) => {
                    h3642
                }
            }
        }
    }
}

pub fn f1032(v0: Adt3, v1: Adt21, v2: Adt87) -> Result<Adt147, Adt150> {
    f1053(v0, v2, f232(match v1 {
        Adt21::C0(_, h3643, _, _, _, _) => {
            h3643
        }
    }))
}

pub fn f1033(v0: Adt3, v1: Adt21, v2: Adt92) -> Result<Adt147, Adt150> {
    f1054(v0, v2, f232(match v1 {
        Adt21::C0(_, h3644, _, _, _, _) => {
            h3644
        }
    }))
}

pub fn f1034(v0: Adt3, v1: Adt21, v2: Adt54) -> Adt147 {
    Adt147::C0(Adt37::C2(match v2.clone() {
        Adt54::C0(h3645, _) => {
            h3645
        }
    }), {
        let m3647 = f971(v0, Adt37::C11);
        match m3647 {
            Adt147::C0(_, h3646, _) => {
                h3646
            }
        }
    }, f1055(v1, match v2 {
        Adt54::C0(_, h3648) => {
            h3648
        }
    }))
}

pub fn f1035(v0: Bytes, v1: Adt19, v2: Result<Adt37, Adt94>) -> R<Result<Adt37, Adt94>> {
    let m3649 = v2;
    match m3649 {
        Ok(v4) => {
            f1056(v0, f970(v1.clone()), f1057(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt37, Adt94>(v3))
        }
    }
}

pub fn f1036(v0: u64, v1: u64, v2: Fn9, v3: Adt19) -> R<Result<Adt37, Adt94>> {
    let m3650 = v3.clone();
    match m3650 {
        Adt19::C0(_) => {
            let c3652: Fn9 = v2;
            c3652.apply(v3)
        }
        Adt19::C1(_) => {
            let c3653: Fn9 = v2;
            c3653.apply(v3)
        }
        Adt19::C2(_) => {
            if f1058(v0, v1) {
                let c3651: Fn9 = v2;
                c3651.apply(v3)
            } else {
                Ok(Err::<Adt37, Adt94>(Adt94::C7(v0)))
            }
        }
        Adt19::C3(_) => {
            let c3654: Fn9 = v2;
            c3654.apply(v3)
        }
        Adt19::C4(_) => {
            let c3655: Fn9 = v2;
            c3655.apply(v3)
        }
        Adt19::C5(_) => {
            let c3656: Fn9 = v2;
            c3656.apply(v3)
        }
        Adt19::C6(_) => {
            let c3657: Fn9 = v2;
            c3657.apply(v3)
        }
        Adt19::C7(_) => {
            let c3658: Fn9 = v2;
            c3658.apply(v3)
        }
        Adt19::C8(_) => {
            let c3659: Fn9 = v2;
            c3659.apply(v3)
        }
        Adt19::C9(_) => {
            let c3660: Fn9 = v2;
            c3660.apply(v3)
        }
        Adt19::C10(_) => {
            let c3661: Fn9 = v2;
            c3661.apply(v3)
        }
        Adt19::C11(_) => {
            let c3662: Fn9 = v2;
            c3662.apply(v3)
        }
    }
}

pub fn f1037(v0: Bytes, v1: Adt26) -> R<Result<Adt19, Adt94>> {
    let m3666 = f1059(v0, Str::lit("tools/call"), match v1.clone() {
        Adt26::C0(_, h3663, _, _, _) => {
            h3663
        }
    }, match v1.clone() {
        Adt26::C0(_, _, _, _, h3664) => {
            h3664
        }
    }, match v1.clone() {
        Adt26::C0(_, _, _, h3665, _) => {
            h3665
        }
    })?;
    match m3666 {
        Ok(v3) => {
            Ok(Ok::<Adt19, Adt94>(Adt19::C2(Adt26::C0(match v1.clone() {
                Adt26::C0(h3667, _, _, _, _) => {
                    h3667
                }
            }, match v1.clone() {
                Adt26::C0(_, h3668, _, _, _) => {
                    h3668
                }
            }, match v1.clone() {
                Adt26::C0(_, _, h3669, _, _) => {
                    h3669
                }
            }, match v1 {
                Adt26::C0(_, _, _, h3670, _) => {
                    h3670
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt19, Adt94>(v2))
        }
    }
}

pub fn f1038(v0: Bytes, v1: Adt27) -> R<Result<Adt19, Adt94>> {
    let m3674 = f1059(v0, Str::lit("resources/read"), match v1.clone() {
        Adt27::C0(_, h3671, _, _) => {
            h3671
        }
    }, match v1.clone() {
        Adt27::C0(_, _, _, h3672) => {
            h3672
        }
    }, match v1.clone() {
        Adt27::C0(_, _, h3673, _) => {
            h3673
        }
    })?;
    match m3674 {
        Ok(v3) => {
            Ok(Ok::<Adt19, Adt94>(Adt19::C4(Adt27::C0(match v1.clone() {
                Adt27::C0(h3675, _, _, _) => {
                    h3675
                }
            }, match v1.clone() {
                Adt27::C0(_, h3676, _, _) => {
                    h3676
                }
            }, match v1 {
                Adt27::C0(_, _, h3677, _) => {
                    h3677
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt19, Adt94>(v2))
        }
    }
}

pub fn f1039(v0: Bytes, v1: Adt28) -> R<Result<Adt19, Adt94>> {
    let m3681 = f1059(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt28::C0(_, h3678, _, _, _) => {
            h3678
        }
    }, match v1.clone() {
        Adt28::C0(_, _, _, _, h3679) => {
            h3679
        }
    }, match v1.clone() {
        Adt28::C0(_, _, _, h3680, _) => {
            h3680
        }
    })?;
    match m3681 {
        Ok(v3) => {
            Ok(Ok::<Adt19, Adt94>(Adt19::C7(Adt28::C0(match v1.clone() {
                Adt28::C0(h3682, _, _, _, _) => {
                    h3682
                }
            }, match v1.clone() {
                Adt28::C0(_, h3683, _, _, _) => {
                    h3683
                }
            }, match v1.clone() {
                Adt28::C0(_, _, h3684, _, _) => {
                    h3684
                }
            }, match v1 {
                Adt28::C0(_, _, _, h3685, _) => {
                    h3685
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt19, Adt94>(v2))
        }
    }
}

pub fn f1040(v0: Str, p3691: List<(Str, Adt9)>, p3692: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt151> {
    let m3690 = (p3691, p3692);
    {
        let mut m3690 = m3690;
        loop {
            let step = {
                let (v1, v2) = m3690;
                let m3686 = v1;
                match m3686.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt151>>(Ok::<List<(Str, Str)>, Adt151>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3688 = f181({
                            let (_, h3687) = v3;
                            h3687
                        });
                        match m3688 {
                            None => {
                                Err::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt151>>(Err::<List<(Str, Str)>, Adt151>(Adt151::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt151>>((v4, List::cons(({
                                    let (h3689, _) = v3;
                                    h3689
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1041(p3695: List<(Str, Str)>, p3696: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3694 = (p3695, p3696);
    {
        let mut m3694 = m3694;
        loop {
            let step = {
                let (v0, v1) = m3694;
                let m3693 = v0;
                match m3693.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1042(v0: Adt135, v1: Adt8) -> R<Option<Adt133>> {
    let m3698 = f1060(match v0.clone() {
        Adt135::C0(_, h3697, _, _) => {
            h3697
        }
    }, v1);
    match m3698 {
        Adt118::C0 => {
            Ok(f1061(v0))
        }
        Adt118::C1(v2) => {
            if f1062(v2.clone()) {
                Ok(f1061(v0))
            } else {
                f1063(v0, v2)
            }
        }
    }
}

pub fn f1043(v0: Str, v1: Adt8) -> Option<Option<Adt8>> {
    let m3699 = f275(v0, v1);
    match m3699 {
        Adt118::C0 => {
            Some(None::<Adt8>)
        }
        Adt118::C1(v2) => {
            let m3700 = f281(v2);
            match m3700 {
                None => {
                    None::<Option<Adt8>>
                }
                Some(v3) => {
                    Some(Some(v3))
                }
            }
        }
    }
}

pub fn f1044() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f1045(v0: Str) -> Adt171 {
    let a3701 = v0.clone();
    let a3702 = Str::lit("debug");
    if equal(a3701, a3702) {
        Adt171::C1(Adt23::C0)
    } else {
        let a3703 = v0.clone();
        let a3704 = Str::lit("info");
        if equal(a3703, a3704) {
            Adt171::C1(Adt23::C1)
        } else {
            let a3705 = v0.clone();
            let a3706 = Str::lit("notice");
            if equal(a3705, a3706) {
                Adt171::C1(Adt23::C2)
            } else {
                let a3707 = v0.clone();
                let a3708 = Str::lit("warning");
                if equal(a3707, a3708) {
                    Adt171::C1(Adt23::C3)
                } else {
                    let a3709 = v0.clone();
                    let a3710 = Str::lit("error");
                    if equal(a3709, a3710) {
                        Adt171::C1(Adt23::C4)
                    } else {
                        let a3711 = v0.clone();
                        let a3712 = Str::lit("critical");
                        if equal(a3711, a3712) {
                            Adt171::C1(Adt23::C5)
                        } else {
                            let a3713 = v0.clone();
                            let a3714 = Str::lit("alert");
                            if equal(a3713, a3714) {
                                Adt171::C1(Adt23::C6)
                            } else {
                                let a3715 = v0;
                                let a3716 = Str::lit("emergency");
                                if equal(a3715, a3716) {
                                    Adt171::C1(Adt23::C7)
                                } else {
                                    Adt171::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f1046() -> Str {
    Str::lit("progressToken")
}

pub fn f1047(v0: Adt62) -> Adt9 {
    Adt9::C5({
        let a3725 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt62::C0(h3717, _, _, _) => {
                h3717
            }
        })), List::<(Str, Adt9)>::nil());
        let a3726 = {
            let a3723 = f733(Str::lit("title"), match v0.clone() {
                Adt62::C0(_, h3718, _, _) => {
                    h3718
                }
            });
            let a3724 = {
                let a3721 = f733(Str::lit("description"), match v0.clone() {
                    Adt62::C0(_, _, h3719, _) => {
                        h3719
                    }
                });
                let a3722 = f391(Str::lit("required"), match v0 {
                    Adt62::C0(_, _, _, h3720) => {
                        h3720
                    }
                });
                append_list(a3721, a3722)
            };
            append_list(a3723, a3724)
        };
        append_list(a3725, a3726)
    })
}

pub fn f1048(v0: Adt75) -> R<Option<Adt9>> {
    let m3728 = f1064(match v0.clone() {
        Adt75::C0(_, h3727, _) => {
            h3727
        }
    })?;
    match m3728 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("role"), Adt9::C3(f991(match v0.clone() {
                Adt75::C0(h3729, _, _) => {
                    h3729
                }
            }))), List::cons((Str::lit("content"), v1), f732(match v0 {
                Adt75::C0(_, _, h3730) => {
                    h3730
                }
            }))))))
        }
    }
}

pub fn f1049(p3734: List<Adt81>, p3735: List<Adt9>) -> List<Adt9> {
    let m3733 = (p3734, p3735);
    {
        let mut m3733 = m3733;
        loop {
            let step = {
                let (v0, v1) = m3733;
                let m3731 = v0;
                match m3731.uncons() {
                    None => {
                        Err::<(List<Adt81>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt81>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C5(f733(Str::lit("name"), match v2 {
                            Adt81::C0(h3732) => {
                                h3732
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1050(v0: Adt84) -> Str {
    let m3736 = v0;
    match m3736 {
        Adt84::C0 => {
            Str::lit("auto")
        }
        Adt84::C1 => {
            Str::lit("required")
        }
        Adt84::C2 => {
            Str::lit("none")
        }
    }
}

pub fn f1051(v0: Adt3, v1: Adt86, v2: bool) -> Result<Adt147, Adt150> {
    if v2 {
        Ok::<Adt147, Adt150>(f971(v0, Adt37::C12(v1)))
    } else {
        Err::<Adt147, Adt150>(Adt150::C4(Adt94::C8(f1065(f845(v1)))))
    }
}

pub fn f1052(v0: Adt22, v1: List<(Str, Adt69)>) -> bool {
    let a3737 = f1066(v1.clone());
    if if if bool_not(a3737) {
        true
    } else {
        f401(match v0.clone() {
            Adt22::C0(_, _, _, h3738, _) => {
                h3738
            }
        })
    } {
        let a3739 = f1067(v1.clone());
        if bool_not(a3739) {
            true
        } else {
            f401(match v0.clone() {
                Adt22::C0(_, _, h3740, _, _) => {
                    h3740
                }
            })
        }
    } else {
        false
    } {
        let a3741 = f1068(v1);
        if bool_not(a3741) {
            true
        } else {
            f401(match v0 {
                Adt22::C0(_, h3742, _, _, _) => {
                    h3742
                }
            })
        }
    } else {
        false
    }
}

pub fn f1053(v0: Adt3, v1: Adt87, v2: bool) -> Result<Adt147, Adt150> {
    if v2 {
        Ok::<Adt147, Adt150>(f971(v0, Adt37::C13(v1)))
    } else {
        Err::<Adt147, Adt150>(Adt150::C4(Adt94::C8(f339())))
    }
}

pub fn f1054(v0: Adt3, v1: Adt92, v2: bool) -> Result<Adt147, Adt150> {
    if v2 {
        Ok::<Adt147, Adt150>(f971(v0, Adt37::C14(v1)))
    } else {
        Err::<Adt147, Adt150>(Adt150::C4(Adt94::C8(f339())))
    }
}

pub fn f1055(v0: Adt21, v1: List<Adt55>) -> List<Adt149> {
    let m3744 = match v0 {
        Adt21::C0(_, _, _, _, h3743, _) => {
            h3743
        }
    };
    match m3744 {
        None => {
            List::<Adt149>::nil()
        }
        Some(v2) => {
            f1069(f1070(v2, v1, List::<Adt149>::nil()), List::<Adt149>::nil())
        }
    }
}

pub fn f1056(v0: Bytes, v1: Adt159, v2: Str, v3: Adt37) -> R<Result<Adt37, Adt94>> {
    let m3745 = v3.clone();
    match m3745 {
        Adt37::C0(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C1(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C2(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C3(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C4(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C5(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C6(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C7(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C8(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C9(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C10(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C11 => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C12(v4) => {
            f1071(v0, v1, v2, v4)
        }
        Adt37::C13(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
        Adt37::C14(_) => {
            Ok(Ok::<Adt37, Adt94>(v3))
        }
    }
}

pub fn f1057(v0: Adt19) -> Str {
    let m3746 = v0;
    match m3746 {
        Adt19::C0(_) => {
            Str::lit("")
        }
        Adt19::C1(_) => {
            Str::lit("")
        }
        Adt19::C2(v1) => {
            match v1 {
                Adt26::C0(_, h3747, _, _, _) => {
                    h3747
                }
            }
        }
        Adt19::C3(_) => {
            Str::lit("")
        }
        Adt19::C4(v2) => {
            match v2 {
                Adt27::C0(_, h3748, _, _) => {
                    h3748
                }
            }
        }
        Adt19::C5(_) => {
            Str::lit("")
        }
        Adt19::C6(_) => {
            Str::lit("")
        }
        Adt19::C7(v3) => {
            match v3 {
                Adt28::C0(_, h3749, _, _, _) => {
                    h3749
                }
            }
        }
        Adt19::C8(_) => {
            Str::lit("")
        }
        Adt19::C9(_) => {
            Str::lit("")
        }
        Adt19::C10(_) => {
            Str::lit("")
        }
        Adt19::C11(_) => {
            Str::lit("")
        }
    }
}

pub fn f1058(v0: u64, v1: u64) -> bool {
    let a3750 = v1;
    let a3751 = v0;
    nat_lt(a3750, a3751)
}

pub fn f1059(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt8>) -> R<Result<Option<Str>, Adt94>> {
    Ok(f1072(f1073(v0, v1, v2, v3)?, f1074(v4)))
}

pub fn f1060(p3757: List<Str>, p3758: Adt8) -> Adt118 {
    let m3756 = (p3757, p3758);
    {
        let mut m3756 = m3756;
        loop {
            let step = {
                let (v0, v1) = m3756;
                let m3752 = v0;
                match m3752.uncons() {
                    None => {
                        Err::<(List<Str>, Adt8), Adt118>(Adt118::C0)
                    }
                    Some((v2, v3)) => {
                        let m3753 = f275(v2, v1);
                        match m3753 {
                            Adt118::C0 => {
                                Err::<(List<Str>, Adt8), Adt118>(Adt118::C0)
                            }
                            Adt118::C1(v4) => {
                                let m3754 = v3.clone();
                                match m3754.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt8), Adt118>(Adt118::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3755 = f281(v4);
                                        match m3755 {
                                            None => {
                                                Err::<(List<Str>, Adt8), Adt118>(Adt118::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt8), Adt118>((v3, v7))
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
                Ok(next) => { m3756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1061(v0: Adt135) -> Option<Adt133> {
    let m3760 = match v0.clone() {
        Adt135::C0(_, _, _, h3759) => {
            h3759
        }
    };
    match m3760 {
        Adt136::C0 => {
            None::<Adt133>
        }
        Adt136::C1 => {
            Some(Adt133::C10(match v0 {
                Adt135::C0(h3761, _, _, _) => {
                    h3761
                }
            }))
        }
        Adt136::C2(_) => {
            None::<Adt133>
        }
    }
}

pub fn f1062(v0: Adt9) -> bool {
    let m3762 = v0;
    match m3762 {
        Adt9::C0 => {
            true
        }
        Adt9::C1(_) => {
            false
        }
        Adt9::C2(_) => {
            false
        }
        Adt9::C3(_) => {
            false
        }
        Adt9::C4(_) => {
            false
        }
        Adt9::C5(_) => {
            false
        }
    }
}

pub fn f1063(v0: Adt135, v1: Adt9) -> R<Option<Adt133>> {
    let m3764 = match v0.clone() {
        Adt135::C0(_, _, _, h3763) => {
            h3763
        }
    };
    match m3764 {
        Adt136::C0 => {
            Ok(Some(Adt133::C8(match v0 {
                Adt135::C0(h3765, _, _, _) => {
                    h3765
                }
            })))
        }
        Adt136::C1 => {
            Ok(Some(Adt133::C10(match v0 {
                Adt135::C0(h3766, _, _, _) => {
                    h3766
                }
            })))
        }
        Adt136::C2(v2) => {
            if f1075(match v0.clone() {
                Adt135::C0(_, _, h3767, _) => {
                    h3767
                }
            }, v2, v1)? {
                Ok(None::<Adt133>)
            } else {
                Ok(Some(Adt133::C9(match v0 {
                    Adt135::C0(h3768, _, _, _) => {
                        h3768
                    }
                })))
            }
        }
    }
}

pub fn f1064(v0: Adt76) -> R<Option<Adt9>> {
    let m3769 = v0;
    match m3769 {
        Adt76::C0(v1) => {
            f1076(v1)
        }
        Adt76::C1(v2) => {
            let m3770 = f1077(v2, List::<Adt9>::nil())?;
            match m3770 {
                None => {
                    Ok(None::<Adt9>)
                }
                Some(v3) => {
                    Ok(Some(Adt9::C4(f490(v3, List::<Adt9>::nil()))))
                }
            }
        }
    }
}

pub fn f1065(v0: List<(Str, Adt69)>) -> Adt22 {
    Adt22::C0(None::<Adt8>, if f1068(v0.clone()) {
        Some(f32())
    } else {
        None::<Adt8>
    }, if f1067(v0.clone()) {
        Some(f32())
    } else {
        None::<Adt8>
    }, if f1066(v0) {
        Some(f32())
    } else {
        None::<Adt8>
    }, None::<Adt8>)
}

pub fn f1066(p3775: List<(Str, Adt69)>) -> bool {
    let m3774 = p3775;
    {
        let mut m3774 = m3774;
        loop {
            let step = {
                let v0 = m3774;
                let m3771 = v0;
                match m3771.uncons() {
                    None => {
                        Err::<List<(Str, Adt69)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3773 = {
                            let (_, h3772) = v1;
                            h3772
                        };
                        match m3773 {
                            Adt69::C0(_) => {
                                Err::<List<(Str, Adt69)>, bool>(true)
                            }
                            Adt69::C1(_) => {
                                Ok::<List<(Str, Adt69)>, bool>(v2)
                            }
                            Adt69::C2(_) => {
                                Ok::<List<(Str, Adt69)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1067(p3780: List<(Str, Adt69)>) -> bool {
    let m3779 = p3780;
    {
        let mut m3779 = m3779;
        loop {
            let step = {
                let v0 = m3779;
                let m3776 = v0;
                match m3776.uncons() {
                    None => {
                        Err::<List<(Str, Adt69)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3778 = {
                            let (_, h3777) = v1;
                            h3777
                        };
                        match m3778 {
                            Adt69::C0(_) => {
                                Ok::<List<(Str, Adt69)>, bool>(v2)
                            }
                            Adt69::C1(_) => {
                                Err::<List<(Str, Adt69)>, bool>(true)
                            }
                            Adt69::C2(_) => {
                                Ok::<List<(Str, Adt69)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1068(p3785: List<(Str, Adt69)>) -> bool {
    let m3784 = p3785;
    {
        let mut m3784 = m3784;
        loop {
            let step = {
                let v0 = m3784;
                let m3781 = v0;
                match m3781.uncons() {
                    None => {
                        Err::<List<(Str, Adt69)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3783 = {
                            let (_, h3782) = v1;
                            h3782
                        };
                        match m3783 {
                            Adt69::C0(_) => {
                                Ok::<List<(Str, Adt69)>, bool>(v2)
                            }
                            Adt69::C1(_) => {
                                Ok::<List<(Str, Adt69)>, bool>(v2)
                            }
                            Adt69::C2(_) => {
                                Err::<List<(Str, Adt69)>, bool>(true)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1069(p3788: List<Adt149>, p3789: List<Adt149>) -> List<Adt149> {
    let m3787 = (p3788, p3789);
    {
        let mut m3787 = m3787;
        loop {
            let step = {
                let (v0, v1) = m3787;
                let m3786 = v0;
                match m3786.uncons() {
                    None => {
                        Err::<(List<Adt149>, List<Adt149>), List<Adt149>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt149>, List<Adt149>), List<Adt149>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3787 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1070(v0: Adt24, p3792: List<Adt55>, p3793: List<Adt149>) -> List<Adt149> {
    let m3791 = (p3792, p3793);
    {
        let mut m3791 = m3791;
        loop {
            let step = {
                let (v1, v2) = m3791;
                let m3790 = v1;
                match m3790.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt149>), List<Adt149>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt55>, List<Adt149>), List<Adt149>>((v4, List::cons(Adt149::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3791 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1071(v0: Bytes, v1: Adt159, v2: Str, v3: Adt86) -> R<Result<Adt37, Adt94>> {
    if f1028(v1.clone()) {
        if f1078(v3.clone()) {
            let m3794 = f1079(v0, f870(v1), v2, v3)?;
            match m3794 {
                Ok(v5) => {
                    Ok(Ok::<Adt37, Adt94>(Adt37::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt37, Adt94>(v4))
                }
            }
        } else {
            Ok(Err::<Adt37, Adt94>(Adt94::C13))
        }
    } else {
        Ok(Err::<Adt37, Adt94>(Adt94::C12(f870(v1))))
    }
}

pub fn f1072(v0: Result<Option<Str>, Adt94>, v1: bool) -> Result<Option<Str>, Adt94> {
    let m3795 = v0;
    match m3795 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt94>(v3)
            } else {
                Err::<Option<Str>, Adt94>(Adt94::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt94>(v2)
        }
    }
}

pub fn f1073(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt94>> {
    let m3796 = v3;
    match m3796 {
        None => {
            Ok(Ok::<Option<Str>, Adt94>(None::<Str>))
        }
        Some(v4) => {
            Ok(f1080(f1081(v0, v1, v2, v4)?))
        }
    }
}

pub fn f1074(v0: Option<Adt8>) -> bool {
    let m3797 = v0;
    match m3797 {
        None => {
            true
        }
        Some(v1) => {
            f1082(match v1 {
                Adt8::C0(h3798) => {
                    h3798
                }
            })
        }
    }
}

pub fn f1075(v0: Adt16, v1: Str, v2: Adt9) -> R<bool> {
    let m3799 = v0;
    match m3799 {
        Adt16::C0 => {
            let m3800 = f181(v2);
            match m3800 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3801 = v1;
                    let a3802 = v3;
                    Ok(equal(a3801, a3802))
                }
            }
        }
        Adt16::C1 => {
            f1083(v1, v2)
        }
        Adt16::C2 => {
            let m3803 = f468(v2);
            match m3803 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3804 = v1;
                        let a3805 = Str::lit("true");
                        Ok(equal(a3804, a3805))
                    } else {
                        let a3806 = v1;
                        let a3807 = Str::lit("false");
                        Ok(equal(a3806, a3807))
                    }
                }
            }
        }
    }
}

pub fn f1076(v0: Adt77) -> R<Option<Adt9>> {
    let m3808 = v0;
    match m3808 {
        Adt77::C0(v1) => {
            Ok(Some(f645(v1)))
        }
        Adt77::C1(v2) => {
            Ok(Some(f646(Str::lit("image"), match v2.clone() {
                Adt47::C0(h3809, _, _, _) => {
                    h3809
                }
            }, match v2.clone() {
                Adt47::C0(_, h3810, _, _) => {
                    h3810
                }
            }, match v2.clone() {
                Adt47::C0(_, _, h3811, _) => {
                    h3811
                }
            }, match v2 {
                Adt47::C0(_, _, _, h3812) => {
                    h3812
                }
            })))
        }
        Adt77::C2(v3) => {
            Ok(Some(f646(Str::lit("audio"), match v3.clone() {
                Adt48::C0(h3813, _, _, _) => {
                    h3813
                }
            }, match v3.clone() {
                Adt48::C0(_, h3814, _, _) => {
                    h3814
                }
            }, match v3.clone() {
                Adt48::C0(_, _, h3815, _) => {
                    h3815
                }
            }, match v3 {
                Adt48::C0(_, _, _, h3816) => {
                    h3816
                }
            })))
        }
        Adt77::C3(v4) => {
            Ok(Some(f1084(v4)))
        }
        Adt77::C4(v5) => {
            f1085(v5)
        }
    }
}

pub fn f1077(p3820: List<Adt77>, p3821: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3819 = (p3820, p3821);
    {
        let mut m3819 = m3819;
        loop {
            let step = {
                let (v0, v1) = m3819;
                let m3817 = v0;
                match m3817.uncons() {
                    None => {
                        Err::<(List<Adt77>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3818 = f1076(v2)?;
                        match m3818 {
                            None => {
                                Err::<(List<Adt77>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(None::<List<Adt9>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt77>, List<Adt9>), R<Option<List<Adt9>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3819 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1078(v0: Adt86) -> bool {
    let m3822 = v0;
    match m3822 {
        Adt86::C0(v1) => {
            let m3823 = v1;
            match m3823.uncons() {
                None => {
                    false
                }
                Some((_, _)) => {
                    true
                }
            }
        }
        Adt86::C1(_) => {
            true
        }
        Adt86::C2(_, _) => {
            true
        }
    }
}

pub fn f1079(v0: Bytes, v1: Str, v2: Str, v3: Adt86) -> R<Result<Adt86, Adt94>> {
    let m3824 = v3.clone();
    match m3824 {
        Adt86::C0(_) => {
            Ok(Ok::<Adt86, Adt94>(v3))
        }
        Adt86::C1(v5) => {
            Ok(f1086(f1087(v0, v1, v2, v5)?, List::<(Str, Adt69)>::nil()))
        }
        Adt86::C2(v6, v7) => {
            Ok(f1086(f1087(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f1080(v0: Option<Str>) -> Result<Option<Str>, Adt94> {
    let m3825 = v0;
    match m3825 {
        None => {
            Err::<Option<Str>, Adt94>(Adt94::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt94>(Some(v1))
        }
    }
}

pub fn f1081(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3829 = f1088(v0, {
        let a3826 = v1;
        utf8_encode(a3826)
    }, {
        let a3827 = v2;
        utf8_encode(a3827)
    }, {
        let a3828 = v3;
        utf8_encode(a3828)
    })?;
    match m3829 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3830 = v4;
            Ok(utf8_decode(a3830))
        }
    }
}

pub fn f1082(p3835: List<(Str, Adt9)>) -> bool {
    let m3834 = p3835;
    {
        let mut m3834 = m3834;
        loop {
            let step = {
                let v0 = m3834;
                let m3831 = v0;
                match m3831.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3833 = f281({
                            let (_, h3832) = v1;
                            h3832
                        });
                        match m3833 {
                            None => {
                                Err::<List<(Str, Adt9)>, bool>(false)
                            }
                            Some(_) => {
                                Ok::<List<(Str, Adt9)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3834 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1083(v0: Str, v1: Adt9) -> R<bool> {
    let m3836 = f713(v1);
    match m3836 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3837 = f597(v2)?;
            match m3837 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3839 = f1089(f29({
                        let a3838 = v0;
                        utf8_encode(a3838)
                    })?)?;
                    match m3839 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f387({
                                let a3840 = v3;
                                let a3841 = v4;
                                int_sub(a3840, a3841)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f1084(v0: Adt78) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt9::C3(match v0.clone() {
        Adt78::C0(h3842, _, _, _) => {
            h3842
        }
    })), List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt78::C0(_, h3843, _, _) => {
            h3843
        }
    })), List::cons((Str::lit("input"), Adt9::C5({
        let m3846 = match v0.clone() {
            Adt78::C0(_, _, h3845, _) => {
                h3845
            }
        };
        match m3846 {
            Adt8::C0(h3844) => {
                h3844
            }
        }
    })), f732(match v0 {
        Adt78::C0(_, _, _, h3847) => {
            h3847
        }
    }))))))
}

pub fn f1085(v0: Adt79) -> R<Option<Adt9>> {
    let m3849 = f389(match v0.clone() {
        Adt79::C0(_, h3848, _, _, _) => {
            h3848
        }
    })?;
    match m3849 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt9::C3(match v0.clone() {
                Adt79::C0(h3850, _, _, _, _) => {
                    h3850
                }
            })), List::cons((Str::lit("content"), Adt9::C4(v1)), {
                let a3856 = f390(Str::lit("structuredContent"), match v0.clone() {
                    Adt79::C0(_, _, h3851, _, _) => {
                        h3851
                    }
                });
                let a3857 = {
                    let a3854 = f391(Str::lit("isError"), match v0.clone() {
                        Adt79::C0(_, _, _, h3852, _) => {
                            h3852
                        }
                    });
                    let a3855 = f732(match v0 {
                        Adt79::C0(_, _, _, _, h3853) => {
                            h3853
                        }
                    });
                    append_list(a3854, a3855)
                };
                append_list(a3856, a3857)
            }))))))
        }
    }
}

pub fn f1086(v0: Option<Str>, v1: List<(Str, Adt69)>) -> Result<Adt86, Adt94> {
    let m3858 = v0;
    match m3858 {
        None => {
            Err::<Adt86, Adt94>(Adt94::C14)
        }
        Some(v2) => {
            let m3859 = v1.clone();
            match m3859.uncons() {
                None => {
                    Ok::<Adt86, Adt94>(Adt86::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt86, Adt94>(Adt86::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f1087(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3863 = f1090(v0, {
        let a3860 = v1;
        utf8_encode(a3860)
    }, {
        let a3861 = v2;
        utf8_encode(a3861)
    }, {
        let a3862 = v3;
        utf8_encode(a3862)
    })?;
    match m3863 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3864 = v4;
            Ok(utf8_decode(a3864))
        }
    }
}

pub fn f1088(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3866 = f1091();
    let a3867 = {
        let a3865 = v0.clone();
        length_bytes(a3865)
    };
    if nat_le(a3866, a3867) {
        let a3869 = f1092(v3.clone())?;
        let a3870 = {
            let a3868 = v3.clone();
            length_bytes(a3868)
        };
        if nat_eq(a3869, a3870) {
            Ok(None::<Bytes>)
        } else {
            f1093(v0, v1, v2, v3.clone(), f1092(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1089(v0: List<u64>) -> R<Option<i64>> {
    let m3871 = v0.clone();
    match m3871.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3872 = v1;
            let a3873 = 45u64;
            if nat_eq(a3872, a3873) {
                let m3874 = v2.clone();
                match m3874.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3875 = f1094(v2, 0i64)?;
                        match m3875 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3876 = v5;
                                    int_neg(a3876)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f1094(v0, 0i64)
            }
        }
    }
}

pub fn f1090(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3878 = f1091();
    let a3879 = {
        let a3877 = v0.clone();
        length_bytes(a3877)
    };
    if nat_le(a3878, a3879) {
        Ok(Some({
            let a3882 = f1095(v3.clone())?;
            let a3883 = {
                let a3880 = Bytes::lit(&[0x2e]);
                let a3881 = f1095(f1096(v0, v1, v2, v3)?)?;
                append_bytes(a3880, a3881)
            };
            append_bytes(a3882, a3883)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1091() -> u64 {
    32u64
}

pub fn f1092(v0: Bytes) -> R<u64> {
    let (_, h3884) = f1097(Fn10::F1098(v0.clone()), {
        let a3885 = v0.clone();
        length_bytes(a3885)
    }, (0u64, {
        let a3886 = v0;
        length_bytes(a3886)
    }))?;
    Ok(h3884)
}

pub fn f1093(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3890 = {
        let a3887 = v3.clone();
        let a3888 = 0u64;
        let a3889 = v4;
        slice_bytes(a3887, a3888, a3889)
    };
    match m3890 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3901 = {
                let a3898 = v3.clone();
                let a3899 = {
                    let a3891 = v4;
                    let a3892 = 1u64;
                    nat_add(a3891, a3892)?
                };
                let a3900 = {
                    let a3896 = {
                        let a3893 = v3;
                        length_bytes(a3893)
                    };
                    let a3897 = {
                        let a3894 = v4;
                        let a3895 = 1u64;
                        nat_add(a3894, a3895)?
                    };
                    nat_sub(a3896, a3897)
                };
                slice_bytes(a3898, a3899, a3900)
            };
            match m3901 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f1099(v0, v1, v2, f1100(v5)?, f1100(v6)?)
                }
            }
        }
    }
}

pub fn f1094(p3907: List<u64>, p3908: i64) -> R<Option<i64>> {
    let m3906 = (p3907, p3908);
    {
        let mut m3906 = m3906;
        loop {
            let step = {
                let (v0, v1) = m3906;
                let m3902 = v0;
                match m3902.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3903 = f1101(v2);
                        match m3903 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f491({
                                    let a3904 = v1;
                                    let a3905 = 10i64;
                                    int_mul(a3904, a3905)?
                                }, f681(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1095(v0: Bytes) -> R<Bytes> {
    let (_, h3909) = f1102(Fn11::F1103(v0.clone()), {
        let a3910 = v0;
        length_bytes(a3910)
    }, (0u64, f243()))?;
    Ok(h3909)
}

pub fn f1096(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f1104(v0, {
        let a3911 = f1105(v1, v2);
        let a3912 = v3;
        append_bytes(a3911, a3912)
    })
}

pub fn f1097(v0: Fn10, p3916: u64, p3917: (u64, u64)) -> R<(u64, u64)> {
    let m3915 = (p3916, p3917);
    {
        let mut m3915 = m3915;
        loop {
            let step = {
                let (v1, v2) = m3915;
                let m3913 = v1;
                if m3913 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3913 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3914: Fn10 = v0.clone();
                        c3914.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1098(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3919 = {
            let (h3918, _) = v1;
            h3918
        };
        let a3920 = 1u64;
        nat_add(a3919, a3920)?
    }, {
        let a3923 = {
            let (_, h3921) = v1;
            h3921
        };
        let a3924 = {
            let a3922 = v0.clone();
            length_bytes(a3922)
        };
        if nat_eq(a3923, a3924) {
            let a3926 = f61(v0, {
                let (h3925, _) = v1;
                h3925
            })?;
            let a3927 = 46u64;
            if nat_eq(a3926, a3927) {
                let (h3928, _) = v1;
                h3928
            } else {
                let (_, h3929) = v1;
                h3929
            }
        } else {
            let (_, h3930) = v1;
            h3930
        }
    }))
}

pub fn f1099(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3931 = v3;
    match m3931 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3932 = v4;
            match m3932 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f1106(f1096(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f1100(v0: Bytes) -> R<Option<Bytes>> {
    let a3937 = {
        let a3934 = {
            let a3933 = v0.clone();
            length_bytes(a3933)
        };
        let a3935 = 2u64;
        let a3936 = 0u64;
        nat_rem(a3934, a3935, a3936)
    };
    let a3938 = 0u64;
    if nat_eq(a3937, a3938) {
        let v1: (u64, (Bytes, u64)) = f1107(Fn12::F1108(v0.clone()), {
            let a3940 = {
                let a3939 = v0;
                length_bytes(a3939)
            };
            let a3941 = 2u64;
            let a3942 = 0u64;
            nat_quot(a3940, a3941, a3942)
        }, (0u64, (f243(), 0u64)))?;
        let a3945 = {
            let (_, h3943) = {
                let (_, h3944) = v1.clone();
                h3944
            };
            h3943
        };
        let a3946 = 0u64;
        if nat_eq(a3945, a3946) {
            Ok(Some({
                let (h3947, _) = {
                    let (_, h3948) = v1;
                    h3948
                };
                h3947
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1101(v0: u64) -> Option<u64> {
    let a3949 = v0;
    let a3950 = 48u64;
    if if nat_lt(a3949, a3950) {
        true
    } else {
        let a3951 = 57u64;
        let a3952 = v0;
        nat_lt(a3951, a3952)
    } {
        None::<u64>
    } else {
        Some({
            let a3953 = v0;
            let a3954 = 48u64;
            nat_sub(a3953, a3954)
        })
    }
}

pub fn f1102(v0: Fn11, p3958: u64, p3959: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3957 = (p3958, p3959);
    {
        let mut m3957 = m3957;
        loop {
            let step = {
                let (v1, v2) = m3957;
                let m3955 = v1;
                if m3955 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3955 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3956: Fn11 = v0.clone();
                        c3956.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3957 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1103(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3961 = {
            let (h3960, _) = v1.clone();
            h3960
        };
        let a3962 = 1u64;
        nat_add(a3961, a3962)?
    }, {
        let a3965 = {
            let (_, h3963) = v1;
            h3963
        };
        let a3966 = f1109(f61(v0, {
            let (h3964, _) = v1;
            h3964
        })?)?;
        append_bytes(a3965, a3966)
    }))
}

pub fn f1104(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f1110(f1111(v0, v1)?)
}

pub fn f1105(v0: Bytes, v1: Bytes) -> Bytes {
    let a3973 = f1112({
        let a3967 = v0.clone();
        length_bytes(a3967)
    });
    let a3974 = {
        let a3971 = v0;
        let a3972 = {
            let a3969 = f1112({
                let a3968 = v1.clone();
                length_bytes(a3968)
            });
            let a3970 = v1;
            append_bytes(a3969, a3970)
        };
        append_bytes(a3971, a3972)
    };
    append_bytes(a3973, a3974)
}

pub fn f1106(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3977 = {
        let a3975 = v0.clone();
        length_bytes(a3975)
    };
    let a3978 = {
        let a3976 = v1.clone();
        length_bytes(a3976)
    };
    if nat_eq(a3977, a3978) {
        let a3979 = f1113(v0, v1)?;
        let a3980 = 0u64;
        Ok(nat_eq(a3979, a3980))
    } else {
        Ok(false)
    }
}

pub fn f1107(v0: Fn12, p3984: u64, p3985: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3983 = (p3984, p3985);
    {
        let mut m3983 = m3983;
        loop {
            let step = {
                let (v1, v2) = m3983;
                let m3981 = v1;
                if m3981 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3981 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3982: Fn12 = v0.clone();
                        c3982.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1108(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f1114(v0, v1)
}

pub fn f1109(v0: u64) -> R<Bytes> {
    let a3992 = f1115({
        let a3986 = v0;
        let a3987 = 16u64;
        let a3988 = 0u64;
        nat_quot(a3986, a3987, a3988)
    })?;
    let a3993 = f1115({
        let a3989 = v0;
        let a3990 = 16u64;
        let a3991 = 0u64;
        nat_rem(a3989, a3990, a3991)
    })?;
    Ok(append_bytes(a3992, a3993))
}

pub fn f1110(v0: Adt172) -> R<Bytes> {
    let a4014 = f1116(match v0.clone() {
        Adt172::C0(h3994, _, _, _, _, _, _, _) => {
            h3994
        }
    })?;
    let a4015 = {
        let a4012 = f1116(match v0.clone() {
            Adt172::C0(_, h3995, _, _, _, _, _, _) => {
                h3995
            }
        })?;
        let a4013 = {
            let a4010 = f1116(match v0.clone() {
                Adt172::C0(_, _, h3996, _, _, _, _, _) => {
                    h3996
                }
            })?;
            let a4011 = {
                let a4008 = f1116(match v0.clone() {
                    Adt172::C0(_, _, _, h3997, _, _, _, _) => {
                        h3997
                    }
                })?;
                let a4009 = {
                    let a4006 = f1116(match v0.clone() {
                        Adt172::C0(_, _, _, _, h3998, _, _, _) => {
                            h3998
                        }
                    })?;
                    let a4007 = {
                        let a4004 = f1116(match v0.clone() {
                            Adt172::C0(_, _, _, _, _, h3999, _, _) => {
                                h3999
                            }
                        })?;
                        let a4005 = {
                            let a4002 = f1116(match v0.clone() {
                                Adt172::C0(_, _, _, _, _, _, h4000, _) => {
                                    h4000
                                }
                            })?;
                            let a4003 = f1116(match v0 {
                                Adt172::C0(_, _, _, _, _, _, _, h4001) => {
                                    h4001
                                }
                            })?;
                            append_bytes(a4002, a4003)
                        };
                        append_bytes(a4004, a4005)
                    };
                    append_bytes(a4006, a4007)
                };
                append_bytes(a4008, a4009)
            };
            append_bytes(a4010, a4011)
        };
        append_bytes(a4012, a4013)
    };
    Ok(append_bytes(a4014, a4015))
}

pub fn f1111(v0: Bytes, v1: Bytes) -> R<Adt172> {
    let v2: Adt173 = f1117(v0)?;
    let v3: Adt172 = f1118(f1119(f1120(), f1121(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f1119(f1119(f1120(), f1121(v2, 1549556828u32))?, f1122(v3))
}

pub fn f1112(v0: u64) -> Bytes {
    let a4041 = f650({
        let a4019 = {
            let a4016 = v0;
            let a4017 = 16777216u64;
            let a4018 = 0u64;
            nat_quot(a4016, a4017, a4018)
        };
        let a4020 = 256u64;
        let a4021 = 0u64;
        nat_rem(a4019, a4020, a4021)
    });
    let a4042 = {
        let a4039 = f650({
            let a4025 = {
                let a4022 = v0;
                let a4023 = 65536u64;
                let a4024 = 0u64;
                nat_quot(a4022, a4023, a4024)
            };
            let a4026 = 256u64;
            let a4027 = 0u64;
            nat_rem(a4025, a4026, a4027)
        });
        let a4040 = {
            let a4037 = f650({
                let a4031 = {
                    let a4028 = v0;
                    let a4029 = 256u64;
                    let a4030 = 0u64;
                    nat_quot(a4028, a4029, a4030)
                };
                let a4032 = 256u64;
                let a4033 = 0u64;
                nat_rem(a4031, a4032, a4033)
            });
            let a4038 = f650({
                let a4034 = v0;
                let a4035 = 256u64;
                let a4036 = 0u64;
                nat_rem(a4034, a4035, a4036)
            });
            append_bytes(a4037, a4038)
        };
        append_bytes(a4039, a4040)
    };
    append_bytes(a4041, a4042)
}

pub fn f1113(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h4043) = f1097(Fn10::F1123(v0.clone(), v1), {
        let a4044 = v0;
        length_bytes(a4044)
    }, (0u64, 0u64))?;
    Ok(h4043)
}

pub fn f1114(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f1124(f61(v0.clone(), {
        let (h4045, _) = v1.clone();
        h4045
    })?);
    let v3: u64 = f1124(f61(v0, {
        let a4047 = {
            let (h4046, _) = v1.clone();
            h4046
        };
        let a4048 = 1u64;
        nat_add(a4047, a4048)?
    })?);
    let a4049 = v2;
    let a4050 = 16u64;
    if if nat_lt(a4049, a4050) {
        let a4051 = v3;
        let a4052 = 16u64;
        nat_lt(a4051, a4052)
    } else {
        false
    } {
        Ok(({
            let a4054 = {
                let (h4053, _) = v1.clone();
                h4053
            };
            let a4055 = 2u64;
            nat_add(a4054, a4055)?
        }, ({
            let a4062 = {
                let (h4056, _) = {
                    let (_, h4057) = v1.clone();
                    h4057
                };
                h4056
            };
            let a4063 = f650({
                let a4060 = {
                    let a4058 = v2;
                    let a4059 = 16u64;
                    nat_mul(a4058, a4059)?
                };
                let a4061 = v3;
                nat_add(a4060, a4061)?
            });
            append_bytes(a4062, a4063)
        }, {
            let (_, h4064) = {
                let (_, h4065) = v1;
                h4065
            };
            h4064
        })))
    } else {
        Ok(({
            let a4067 = {
                let (h4066, _) = v1.clone();
                h4066
            };
            let a4068 = 2u64;
            nat_add(a4067, a4068)?
        }, ({
            let (h4069, _) = {
                let (_, h4070) = v1.clone();
                h4070
            };
            h4069
        }, {
            let a4073 = {
                let (_, h4071) = {
                    let (_, h4072) = v1;
                    h4072
                };
                h4071
            };
            let a4074 = 1u64;
            nat_add(a4073, a4074)?
        })))
    }
}

pub fn f1115(v0: u64) -> R<Bytes> {
    Ok(f650({
        let a4075 = v0;
        let a4076 = 10u64;
        if nat_lt(a4075, a4076) {
            let a4077 = 48u64;
            let a4078 = v0;
            nat_add(a4077, a4078)?
        } else {
            let a4079 = 87u64;
            let a4080 = v0;
            nat_add(a4079, a4080)?
        }
    }))
}

pub fn f1116(v0: u32) -> R<Bytes> {
    let a4085 = f1125(v0, 24u32)?;
    let a4086 = {
        let a4083 = f1125(v0, 16u32)?;
        let a4084 = {
            let a4081 = f1125(v0, 8u32)?;
            let a4082 = f1125(v0, 0u32)?;
            append_bytes(a4081, a4082)
        };
        append_bytes(a4083, a4084)
    };
    Ok(append_bytes(a4085, a4086))
}

pub fn f1117(v0: Bytes) -> R<Adt173> {
    let a4088 = f1126();
    let a4089 = {
        let a4087 = v0.clone();
        length_bytes(a4087)
    };
    if nat_lt(a4088, a4089) {
        Ok(f1127(f1128(v0)?))
    } else {
        f1129(v0)
    }
}

pub fn f1118(v0: Adt172, v1: u64, v2: Bytes) -> R<Adt172> {
    let v3: u64 = {
        let a4090 = v2.clone();
        length_bytes(a4090)
    };
    let v4: u64 = {
        let a4096 = {
            let a4093 = {
                let a4091 = v3;
                let a4092 = 72u64;
                nat_add(a4091, a4092)?
            };
            let a4094 = 64u64;
            let a4095 = 0u64;
            nat_quot(a4093, a4094, a4095)
        };
        let a4097 = 64u64;
        nat_mul(a4096, a4097)?
    };
    let (_, h4098) = f1130(Fn13::F1131(v2, v1, v3, v4), {
        let a4099 = v4;
        let a4100 = 64u64;
        let a4101 = 0u64;
        nat_quot(a4099, a4100, a4101)
    }, (0u64, v0))?;
    Ok(h4098)
}

pub fn f1119(v0: Adt172, v1: Adt173) -> R<Adt172> {
    let v2: Adt174 = f1132(Fn14::F1133, 64u64, Adt174::C0(match v0.clone() {
        Adt172::C0(h4102, _, _, _, _, _, _, _) => {
            h4102
        }
    }, match v0.clone() {
        Adt172::C0(_, h4103, _, _, _, _, _, _) => {
            h4103
        }
    }, match v0.clone() {
        Adt172::C0(_, _, h4104, _, _, _, _, _) => {
            h4104
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, h4105, _, _, _, _) => {
            h4105
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, h4106, _, _, _) => {
            h4106
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, _, h4107, _, _) => {
            h4107
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, _, _, h4108, _) => {
            h4108
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, _, _, _, h4109) => {
            h4109
        }
    }, v1, 0u64))?;
    Ok(Adt172::C0(f1134(match v0.clone() {
        Adt172::C0(h4110, _, _, _, _, _, _, _) => {
            h4110
        }
    }, match v2.clone() {
        Adt174::C0(h4111, _, _, _, _, _, _, _, _, _) => {
            h4111
        }
    }), f1134(match v0.clone() {
        Adt172::C0(_, h4112, _, _, _, _, _, _) => {
            h4112
        }
    }, match v2.clone() {
        Adt174::C0(_, h4113, _, _, _, _, _, _, _, _) => {
            h4113
        }
    }), f1134(match v0.clone() {
        Adt172::C0(_, _, h4114, _, _, _, _, _) => {
            h4114
        }
    }, match v2.clone() {
        Adt174::C0(_, _, h4115, _, _, _, _, _, _, _) => {
            h4115
        }
    }), f1134(match v0.clone() {
        Adt172::C0(_, _, _, h4116, _, _, _, _) => {
            h4116
        }
    }, match v2.clone() {
        Adt174::C0(_, _, _, h4117, _, _, _, _, _, _) => {
            h4117
        }
    }), f1134(match v0.clone() {
        Adt172::C0(_, _, _, _, h4118, _, _, _) => {
            h4118
        }
    }, match v2.clone() {
        Adt174::C0(_, _, _, _, h4119, _, _, _, _, _) => {
            h4119
        }
    }), f1134(match v0.clone() {
        Adt172::C0(_, _, _, _, _, h4120, _, _) => {
            h4120
        }
    }, match v2.clone() {
        Adt174::C0(_, _, _, _, _, h4121, _, _, _, _) => {
            h4121
        }
    }), f1134(match v0.clone() {
        Adt172::C0(_, _, _, _, _, _, h4122, _) => {
            h4122
        }
    }, match v2.clone() {
        Adt174::C0(_, _, _, _, _, _, h4123, _, _, _) => {
            h4123
        }
    }), f1134(match v0 {
        Adt172::C0(_, _, _, _, _, _, _, h4124) => {
            h4124
        }
    }, match v2 {
        Adt174::C0(_, _, _, _, _, _, _, h4125, _, _) => {
            h4125
        }
    })))
}

pub fn f1120() -> Adt172 {
    Adt172::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f1121(v0: Adt173, v1: u32) -> Adt173 {
    Adt173::C0({
        let a4127 = match v0.clone() {
            Adt173::C0(h4126, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4126
            }
        };
        let a4128 = v1;
        fixed_u32::bit_xor(a4127, a4128)
    }, {
        let a4130 = match v0.clone() {
            Adt173::C0(_, h4129, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4129
            }
        };
        let a4131 = v1;
        fixed_u32::bit_xor(a4130, a4131)
    }, {
        let a4133 = match v0.clone() {
            Adt173::C0(_, _, h4132, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4132
            }
        };
        let a4134 = v1;
        fixed_u32::bit_xor(a4133, a4134)
    }, {
        let a4136 = match v0.clone() {
            Adt173::C0(_, _, _, h4135, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4135
            }
        };
        let a4137 = v1;
        fixed_u32::bit_xor(a4136, a4137)
    }, {
        let a4139 = match v0.clone() {
            Adt173::C0(_, _, _, _, h4138, _, _, _, _, _, _, _, _, _, _, _) => {
                h4138
            }
        };
        let a4140 = v1;
        fixed_u32::bit_xor(a4139, a4140)
    }, {
        let a4142 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, h4141, _, _, _, _, _, _, _, _, _, _) => {
                h4141
            }
        };
        let a4143 = v1;
        fixed_u32::bit_xor(a4142, a4143)
    }, {
        let a4145 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, h4144, _, _, _, _, _, _, _, _, _) => {
                h4144
            }
        };
        let a4146 = v1;
        fixed_u32::bit_xor(a4145, a4146)
    }, {
        let a4148 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, _, h4147, _, _, _, _, _, _, _, _) => {
                h4147
            }
        };
        let a4149 = v1;
        fixed_u32::bit_xor(a4148, a4149)
    }, {
        let a4151 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, _, _, h4150, _, _, _, _, _, _, _) => {
                h4150
            }
        };
        let a4152 = v1;
        fixed_u32::bit_xor(a4151, a4152)
    }, {
        let a4154 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, _, _, _, h4153, _, _, _, _, _, _) => {
                h4153
            }
        };
        let a4155 = v1;
        fixed_u32::bit_xor(a4154, a4155)
    }, {
        let a4157 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, _, _, _, _, h4156, _, _, _, _, _) => {
                h4156
            }
        };
        let a4158 = v1;
        fixed_u32::bit_xor(a4157, a4158)
    }, {
        let a4160 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, h4159, _, _, _, _) => {
                h4159
            }
        };
        let a4161 = v1;
        fixed_u32::bit_xor(a4160, a4161)
    }, {
        let a4163 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4162, _, _, _) => {
                h4162
            }
        };
        let a4164 = v1;
        fixed_u32::bit_xor(a4163, a4164)
    }, {
        let a4166 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4165, _, _) => {
                h4165
            }
        };
        let a4167 = v1;
        fixed_u32::bit_xor(a4166, a4167)
    }, {
        let a4169 = match v0.clone() {
            Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4168, _) => {
                h4168
            }
        };
        let a4170 = v1;
        fixed_u32::bit_xor(a4169, a4170)
    }, {
        let a4172 = match v0 {
            Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4171) => {
                h4171
            }
        };
        let a4173 = v1;
        fixed_u32::bit_xor(a4172, a4173)
    })
}

pub fn f1122(v0: Adt172) -> Adt173 {
    Adt173::C0(match v0.clone() {
        Adt172::C0(h4174, _, _, _, _, _, _, _) => {
            h4174
        }
    }, match v0.clone() {
        Adt172::C0(_, h4175, _, _, _, _, _, _) => {
            h4175
        }
    }, match v0.clone() {
        Adt172::C0(_, _, h4176, _, _, _, _, _) => {
            h4176
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, h4177, _, _, _, _) => {
            h4177
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, h4178, _, _, _) => {
            h4178
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, _, h4179, _, _) => {
            h4179
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, _, _, h4180, _) => {
            h4180
        }
    }, match v0 {
        Adt172::C0(_, _, _, _, _, _, _, h4181) => {
            h4181
        }
    }, 2147483648u32, f1135(), f1135(), f1135(), f1135(), f1135(), f1135(), 768u32)
}

pub fn f1123(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a4183 = {
            let (h4182, _) = v2;
            h4182
        };
        let a4184 = 1u64;
        nat_add(a4183, a4184)?
    }, {
        let a4236 = {
            let (_, h4185) = v2;
            h4185
        };
        let a4237 = {
            let v3: u8 = {
                let a4188 = f1136(v0, {
                    let (h4186, _) = v2;
                    h4186
                });
                let a4189 = f1136(v1, {
                    let (h4187, _) = v2;
                    h4187
                });
                fixed_u8::bit_xor(a4188, a4189)
            };
            let a4234 = {
                let a4210 = {
                    let a4198 = {
                        let a4192 = {
                            let a4190 = v3;
                            let a4191 = 128u8;
                            fixed_u8::bit_and(a4190, a4191)
                        };
                        let a4193 = 128u8;
                        if equal(a4192, a4193) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a4199 = {
                        let a4196 = {
                            let a4194 = v3;
                            let a4195 = 64u8;
                            fixed_u8::bit_and(a4194, a4195)
                        };
                        let a4197 = 64u8;
                        if equal(a4196, a4197) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4198, a4199)?
                };
                let a4211 = {
                    let a4208 = {
                        let a4202 = {
                            let a4200 = v3;
                            let a4201 = 32u8;
                            fixed_u8::bit_and(a4200, a4201)
                        };
                        let a4203 = 32u8;
                        if equal(a4202, a4203) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a4209 = {
                        let a4206 = {
                            let a4204 = v3;
                            let a4205 = 16u8;
                            fixed_u8::bit_and(a4204, a4205)
                        };
                        let a4207 = 16u8;
                        if equal(a4206, a4207) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4208, a4209)?
                };
                nat_add(a4210, a4211)?
            };
            let a4235 = {
                let a4232 = {
                    let a4220 = {
                        let a4214 = {
                            let a4212 = v3;
                            let a4213 = 8u8;
                            fixed_u8::bit_and(a4212, a4213)
                        };
                        let a4215 = 8u8;
                        if equal(a4214, a4215) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a4221 = {
                        let a4218 = {
                            let a4216 = v3;
                            let a4217 = 4u8;
                            fixed_u8::bit_and(a4216, a4217)
                        };
                        let a4219 = 4u8;
                        if equal(a4218, a4219) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4220, a4221)?
                };
                let a4233 = {
                    let a4230 = {
                        let a4224 = {
                            let a4222 = v3;
                            let a4223 = 2u8;
                            fixed_u8::bit_and(a4222, a4223)
                        };
                        let a4225 = 2u8;
                        if equal(a4224, a4225) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a4231 = {
                        let a4228 = {
                            let a4226 = v3;
                            let a4227 = 1u8;
                            fixed_u8::bit_and(a4226, a4227)
                        };
                        let a4229 = 1u8;
                        if equal(a4228, a4229) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4230, a4231)?
                };
                nat_add(a4232, a4233)?
            };
            nat_add(a4234, a4235)?
        };
        nat_add(a4236, a4237)?
    }))
}

pub fn f1124(v0: u64) -> u64 {
    let a4238 = v0;
    let a4239 = 48u64;
    if nat_lt(a4238, a4239) {
        16u64
    } else {
        let a4240 = v0;
        let a4241 = 58u64;
        if nat_lt(a4240, a4241) {
            let a4242 = v0;
            let a4243 = 48u64;
            nat_sub(a4242, a4243)
        } else {
            let a4244 = v0;
            let a4245 = 97u64;
            if nat_lt(a4244, a4245) {
                16u64
            } else {
                let a4246 = v0;
                let a4247 = 103u64;
                if nat_lt(a4246, a4247) {
                    let a4248 = v0;
                    let a4249 = 87u64;
                    nat_sub(a4248, a4249)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f1125(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f650({
        let v2: u32 = {
            let a4250 = f1137(v0, v1);
            let a4251 = 255u32;
            fixed_u32::bit_and(a4250, a4251)
        };
        let a4440 = {
            let a4344 = {
                let a4296 = {
                    let a4272 = {
                        let a4260 = {
                            let a4254 = {
                                let a4252 = v2;
                                let a4253 = 2147483648u32;
                                fixed_u32::bit_and(a4252, a4253)
                            };
                            let a4255 = 2147483648u32;
                            if equal(a4254, a4255) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a4261 = {
                            let a4258 = {
                                let a4256 = v2;
                                let a4257 = 1073741824u32;
                                fixed_u32::bit_and(a4256, a4257)
                            };
                            let a4259 = 1073741824u32;
                            if equal(a4258, a4259) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4260, a4261)?
                    };
                    let a4273 = {
                        let a4270 = {
                            let a4264 = {
                                let a4262 = v2;
                                let a4263 = 536870912u32;
                                fixed_u32::bit_and(a4262, a4263)
                            };
                            let a4265 = 536870912u32;
                            if equal(a4264, a4265) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a4271 = {
                            let a4268 = {
                                let a4266 = v2;
                                let a4267 = 268435456u32;
                                fixed_u32::bit_and(a4266, a4267)
                            };
                            let a4269 = 268435456u32;
                            if equal(a4268, a4269) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4270, a4271)?
                    };
                    nat_add(a4272, a4273)?
                };
                let a4297 = {
                    let a4294 = {
                        let a4282 = {
                            let a4276 = {
                                let a4274 = v2;
                                let a4275 = 134217728u32;
                                fixed_u32::bit_and(a4274, a4275)
                            };
                            let a4277 = 134217728u32;
                            if equal(a4276, a4277) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a4283 = {
                            let a4280 = {
                                let a4278 = v2;
                                let a4279 = 67108864u32;
                                fixed_u32::bit_and(a4278, a4279)
                            };
                            let a4281 = 67108864u32;
                            if equal(a4280, a4281) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4282, a4283)?
                    };
                    let a4295 = {
                        let a4292 = {
                            let a4286 = {
                                let a4284 = v2;
                                let a4285 = 33554432u32;
                                fixed_u32::bit_and(a4284, a4285)
                            };
                            let a4287 = 33554432u32;
                            if equal(a4286, a4287) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a4293 = {
                            let a4290 = {
                                let a4288 = v2;
                                let a4289 = 16777216u32;
                                fixed_u32::bit_and(a4288, a4289)
                            };
                            let a4291 = 16777216u32;
                            if equal(a4290, a4291) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4292, a4293)?
                    };
                    nat_add(a4294, a4295)?
                };
                nat_add(a4296, a4297)?
            };
            let a4345 = {
                let a4342 = {
                    let a4318 = {
                        let a4306 = {
                            let a4300 = {
                                let a4298 = v2;
                                let a4299 = 8388608u32;
                                fixed_u32::bit_and(a4298, a4299)
                            };
                            let a4301 = 8388608u32;
                            if equal(a4300, a4301) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a4307 = {
                            let a4304 = {
                                let a4302 = v2;
                                let a4303 = 4194304u32;
                                fixed_u32::bit_and(a4302, a4303)
                            };
                            let a4305 = 4194304u32;
                            if equal(a4304, a4305) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4306, a4307)?
                    };
                    let a4319 = {
                        let a4316 = {
                            let a4310 = {
                                let a4308 = v2;
                                let a4309 = 2097152u32;
                                fixed_u32::bit_and(a4308, a4309)
                            };
                            let a4311 = 2097152u32;
                            if equal(a4310, a4311) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a4317 = {
                            let a4314 = {
                                let a4312 = v2;
                                let a4313 = 1048576u32;
                                fixed_u32::bit_and(a4312, a4313)
                            };
                            let a4315 = 1048576u32;
                            if equal(a4314, a4315) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4316, a4317)?
                    };
                    nat_add(a4318, a4319)?
                };
                let a4343 = {
                    let a4340 = {
                        let a4328 = {
                            let a4322 = {
                                let a4320 = v2;
                                let a4321 = 524288u32;
                                fixed_u32::bit_and(a4320, a4321)
                            };
                            let a4323 = 524288u32;
                            if equal(a4322, a4323) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a4329 = {
                            let a4326 = {
                                let a4324 = v2;
                                let a4325 = 262144u32;
                                fixed_u32::bit_and(a4324, a4325)
                            };
                            let a4327 = 262144u32;
                            if equal(a4326, a4327) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4328, a4329)?
                    };
                    let a4341 = {
                        let a4338 = {
                            let a4332 = {
                                let a4330 = v2;
                                let a4331 = 131072u32;
                                fixed_u32::bit_and(a4330, a4331)
                            };
                            let a4333 = 131072u32;
                            if equal(a4332, a4333) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a4339 = {
                            let a4336 = {
                                let a4334 = v2;
                                let a4335 = 65536u32;
                                fixed_u32::bit_and(a4334, a4335)
                            };
                            let a4337 = 65536u32;
                            if equal(a4336, a4337) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4338, a4339)?
                    };
                    nat_add(a4340, a4341)?
                };
                nat_add(a4342, a4343)?
            };
            nat_add(a4344, a4345)?
        };
        let a4441 = {
            let a4438 = {
                let a4390 = {
                    let a4366 = {
                        let a4354 = {
                            let a4348 = {
                                let a4346 = v2;
                                let a4347 = 32768u32;
                                fixed_u32::bit_and(a4346, a4347)
                            };
                            let a4349 = 32768u32;
                            if equal(a4348, a4349) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a4355 = {
                            let a4352 = {
                                let a4350 = v2;
                                let a4351 = 16384u32;
                                fixed_u32::bit_and(a4350, a4351)
                            };
                            let a4353 = 16384u32;
                            if equal(a4352, a4353) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4354, a4355)?
                    };
                    let a4367 = {
                        let a4364 = {
                            let a4358 = {
                                let a4356 = v2;
                                let a4357 = 8192u32;
                                fixed_u32::bit_and(a4356, a4357)
                            };
                            let a4359 = 8192u32;
                            if equal(a4358, a4359) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a4365 = {
                            let a4362 = {
                                let a4360 = v2;
                                let a4361 = 4096u32;
                                fixed_u32::bit_and(a4360, a4361)
                            };
                            let a4363 = 4096u32;
                            if equal(a4362, a4363) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4364, a4365)?
                    };
                    nat_add(a4366, a4367)?
                };
                let a4391 = {
                    let a4388 = {
                        let a4376 = {
                            let a4370 = {
                                let a4368 = v2;
                                let a4369 = 2048u32;
                                fixed_u32::bit_and(a4368, a4369)
                            };
                            let a4371 = 2048u32;
                            if equal(a4370, a4371) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a4377 = {
                            let a4374 = {
                                let a4372 = v2;
                                let a4373 = 1024u32;
                                fixed_u32::bit_and(a4372, a4373)
                            };
                            let a4375 = 1024u32;
                            if equal(a4374, a4375) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4376, a4377)?
                    };
                    let a4389 = {
                        let a4386 = {
                            let a4380 = {
                                let a4378 = v2;
                                let a4379 = 512u32;
                                fixed_u32::bit_and(a4378, a4379)
                            };
                            let a4381 = 512u32;
                            if equal(a4380, a4381) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a4387 = {
                            let a4384 = {
                                let a4382 = v2;
                                let a4383 = 256u32;
                                fixed_u32::bit_and(a4382, a4383)
                            };
                            let a4385 = 256u32;
                            if equal(a4384, a4385) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4386, a4387)?
                    };
                    nat_add(a4388, a4389)?
                };
                nat_add(a4390, a4391)?
            };
            let a4439 = {
                let a4436 = {
                    let a4412 = {
                        let a4400 = {
                            let a4394 = {
                                let a4392 = v2;
                                let a4393 = 128u32;
                                fixed_u32::bit_and(a4392, a4393)
                            };
                            let a4395 = 128u32;
                            if equal(a4394, a4395) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4401 = {
                            let a4398 = {
                                let a4396 = v2;
                                let a4397 = 64u32;
                                fixed_u32::bit_and(a4396, a4397)
                            };
                            let a4399 = 64u32;
                            if equal(a4398, a4399) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4400, a4401)?
                    };
                    let a4413 = {
                        let a4410 = {
                            let a4404 = {
                                let a4402 = v2;
                                let a4403 = 32u32;
                                fixed_u32::bit_and(a4402, a4403)
                            };
                            let a4405 = 32u32;
                            if equal(a4404, a4405) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4411 = {
                            let a4408 = {
                                let a4406 = v2;
                                let a4407 = 16u32;
                                fixed_u32::bit_and(a4406, a4407)
                            };
                            let a4409 = 16u32;
                            if equal(a4408, a4409) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4410, a4411)?
                    };
                    nat_add(a4412, a4413)?
                };
                let a4437 = {
                    let a4434 = {
                        let a4422 = {
                            let a4416 = {
                                let a4414 = v2;
                                let a4415 = 8u32;
                                fixed_u32::bit_and(a4414, a4415)
                            };
                            let a4417 = 8u32;
                            if equal(a4416, a4417) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4423 = {
                            let a4420 = {
                                let a4418 = v2;
                                let a4419 = 4u32;
                                fixed_u32::bit_and(a4418, a4419)
                            };
                            let a4421 = 4u32;
                            if equal(a4420, a4421) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4422, a4423)?
                    };
                    let a4435 = {
                        let a4432 = {
                            let a4426 = {
                                let a4424 = v2;
                                let a4425 = 2u32;
                                fixed_u32::bit_and(a4424, a4425)
                            };
                            let a4427 = 2u32;
                            if equal(a4426, a4427) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4433 = {
                            let a4430 = {
                                let a4428 = v2;
                                let a4429 = 1u32;
                                fixed_u32::bit_and(a4428, a4429)
                            };
                            let a4431 = 1u32;
                            if equal(a4430, a4431) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4432, a4433)?
                    };
                    nat_add(a4434, a4435)?
                };
                nat_add(a4436, a4437)?
            };
            nat_add(a4438, a4439)?
        };
        nat_add(a4440, a4441)?
    }))
}

pub fn f1126() -> u64 {
    64u64
}

pub fn f1127(v0: Adt172) -> Adt173 {
    Adt173::C0(match v0.clone() {
        Adt172::C0(h4442, _, _, _, _, _, _, _) => {
            h4442
        }
    }, match v0.clone() {
        Adt172::C0(_, h4443, _, _, _, _, _, _) => {
            h4443
        }
    }, match v0.clone() {
        Adt172::C0(_, _, h4444, _, _, _, _, _) => {
            h4444
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, h4445, _, _, _, _) => {
            h4445
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, h4446, _, _, _) => {
            h4446
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, _, h4447, _, _) => {
            h4447
        }
    }, match v0.clone() {
        Adt172::C0(_, _, _, _, _, _, h4448, _) => {
            h4448
        }
    }, match v0 {
        Adt172::C0(_, _, _, _, _, _, _, h4449) => {
            h4449
        }
    }, f1135(), f1135(), f1135(), f1135(), f1135(), f1135(), f1135(), f1135())
}

pub fn f1128(v0: Bytes) -> R<Adt172> {
    let v1: u64 = {
        let a4450 = v0.clone();
        length_bytes(a4450)
    };
    let v2: u64 = {
        let a4456 = {
            let a4453 = {
                let a4451 = v1;
                let a4452 = 72u64;
                nat_add(a4451, a4452)?
            };
            let a4454 = 64u64;
            let a4455 = 0u64;
            nat_quot(a4453, a4454, a4455)
        };
        let a4457 = 64u64;
        nat_mul(a4456, a4457)?
    };
    let (_, h4458) = f1130(Fn13::F1138(v0, v1, v2), {
        let a4459 = v2;
        let a4460 = 64u64;
        let a4461 = 0u64;
        nat_quot(a4459, a4460, a4461)
    }, (0u64, f1120()))?;
    Ok(h4458)
}

pub fn f1129(v0: Bytes) -> R<Adt173> {
    Ok(Adt173::C0(f1139(v0.clone(), 0u64)?, f1139(v0.clone(), 4u64)?, f1139(v0.clone(), 8u64)?, f1139(v0.clone(), 12u64)?, f1139(v0.clone(), 16u64)?, f1139(v0.clone(), 20u64)?, f1139(v0.clone(), 24u64)?, f1139(v0.clone(), 28u64)?, f1139(v0.clone(), 32u64)?, f1139(v0.clone(), 36u64)?, f1139(v0.clone(), 40u64)?, f1139(v0.clone(), 44u64)?, f1139(v0.clone(), 48u64)?, f1139(v0.clone(), 52u64)?, f1139(v0.clone(), 56u64)?, f1139(v0, 60u64)?))
}

pub fn f1130(v0: Fn13, p4465: u64, p4466: (u64, Adt172)) -> R<(u64, Adt172)> {
    let m4464 = (p4465, p4466);
    {
        let mut m4464 = m4464;
        loop {
            let step = {
                let (v1, v2) = m4464;
                let m4462 = v1;
                if m4462 == 0 {
                    Err::<(u64, (u64, Adt172)), R<(u64, Adt172)>>(Ok(v2))
                } else {
                    let v3 = m4462 - 1;
                    Ok::<(u64, (u64, Adt172)), R<(u64, Adt172)>>((v3, {
                        let c4463: Fn13 = v0.clone();
                        c4463.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1131(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt172)) -> R<(u64, Adt172)> {
    Ok(({
        let a4468 = {
            let (h4467, _) = v4.clone();
            h4467
        };
        let a4469 = 64u64;
        nat_add(a4468, a4469)?
    }, f1119({
        let (_, h4470) = v4;
        h4470
    }, f1140(v0, v2, v3, {
        let a4473 = {
            let a4471 = v1;
            let a4472 = v2;
            nat_add(a4471, a4472)?
        };
        let a4474 = 8u64;
        nat_mul(a4473, a4474)?
    }, {
        let (h4475, _) = v4;
        h4475
    })?)?))
}

pub fn f1132(v0: Fn14, p4479: u64, p4480: Adt174) -> R<Adt174> {
    let m4478 = (p4479, p4480);
    {
        let mut m4478 = m4478;
        loop {
            let step = {
                let (v1, v2) = m4478;
                let m4476 = v1;
                if m4476 == 0 {
                    Err::<(u64, Adt174), R<Adt174>>(Ok(v2))
                } else {
                    let v3 = m4476 - 1;
                    Ok::<(u64, Adt174), R<Adt174>>((v3, {
                        let c4477: Fn14 = v0.clone();
                        c4477.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1133(v0: Adt174) -> R<Adt174> {
    f1141(v0)
}

pub fn f1134(v0: u32, v1: u32) -> u32 {
    let m4483 = {
        let a4481 = f1142(v0);
        let a4482 = f1142(v1);
        fixed_u64::checked_add(a4481, a4482)
    };
    match m4483 {
        None => {
            0u32
        }
        Some(v2) => {
            f1143(v2)
        }
    }
}

pub fn f1135() -> u32 {
    0u32
}

pub fn f1136(v0: Bytes, v1: u64) -> u8 {
    let m4486 = {
        let a4484 = v0;
        let a4485 = v1;
        index_bytes(a4484, a4485)
    };
    match m4486 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1137(v0: u32, v1: u32) -> u32 {
    let m4489 = {
        let a4487 = v0;
        let a4488 = v1;
        fixed_u32::shift_right(a4487, a4488)
    };
    match m4489 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1138(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt172)) -> R<(u64, Adt172)> {
    Ok(({
        let a4491 = {
            let (h4490, _) = v3.clone();
            h4490
        };
        let a4492 = 64u64;
        nat_add(a4491, a4492)?
    }, f1119({
        let (_, h4493) = v3;
        h4493
    }, f1144(v0, v1, v2, {
        let (h4494, _) = v3;
        h4494
    })?)?))
}

pub fn f1139(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f1145(f1146(v0.clone(), v1), f1146(v0.clone(), {
        let a4495 = v1;
        let a4496 = 1u64;
        nat_add(a4495, a4496)?
    }), f1146(v0.clone(), {
        let a4497 = v1;
        let a4498 = 2u64;
        nat_add(a4497, a4498)?
    }), f1146(v0, {
        let a4499 = v1;
        let a4500 = 3u64;
        nat_add(a4499, a4500)?
    })))
}

pub fn f1140(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt173> {
    Ok(Adt173::C0(f1147(v0.clone(), v1, v2, v3, v4)?, f1147(v0.clone(), v1, v2, v3, {
        let a4501 = v4;
        let a4502 = 4u64;
        nat_add(a4501, a4502)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4503 = v4;
        let a4504 = 8u64;
        nat_add(a4503, a4504)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4505 = v4;
        let a4506 = 12u64;
        nat_add(a4505, a4506)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4507 = v4;
        let a4508 = 16u64;
        nat_add(a4507, a4508)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4509 = v4;
        let a4510 = 20u64;
        nat_add(a4509, a4510)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4511 = v4;
        let a4512 = 24u64;
        nat_add(a4511, a4512)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4513 = v4;
        let a4514 = 28u64;
        nat_add(a4513, a4514)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4515 = v4;
        let a4516 = 32u64;
        nat_add(a4515, a4516)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4517 = v4;
        let a4518 = 36u64;
        nat_add(a4517, a4518)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4519 = v4;
        let a4520 = 40u64;
        nat_add(a4519, a4520)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4521 = v4;
        let a4522 = 44u64;
        nat_add(a4521, a4522)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4523 = v4;
        let a4524 = 48u64;
        nat_add(a4523, a4524)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4525 = v4;
        let a4526 = 52u64;
        nat_add(a4525, a4526)?
    })?, f1147(v0.clone(), v1, v2, v3, {
        let a4527 = v4;
        let a4528 = 56u64;
        nat_add(a4527, a4528)?
    })?, f1147(v0, v1, v2, v3, {
        let a4529 = v4;
        let a4530 = 60u64;
        nat_add(a4529, a4530)?
    })?))
}

pub fn f1141(v0: Adt174) -> R<Adt174> {
    let v1: u32 = f1148(match v0.clone() {
        Adt174::C0(_, _, _, _, _, _, _, _, _, h4531) => {
            h4531
        }
    });
    let v2: u32 = f1134(match v0.clone() {
        Adt174::C0(_, _, _, _, _, _, _, h4532, _, _) => {
            h4532
        }
    }, f1134(f1149(match v0.clone() {
        Adt174::C0(_, _, _, _, h4533, _, _, _, _, _) => {
            h4533
        }
    }), f1134(f1150(match v0.clone() {
        Adt174::C0(_, _, _, _, h4534, _, _, _, _, _) => {
            h4534
        }
    }, match v0.clone() {
        Adt174::C0(_, _, _, _, _, h4535, _, _, _, _) => {
            h4535
        }
    }, match v0.clone() {
        Adt174::C0(_, _, _, _, _, _, h4536, _, _, _) => {
            h4536
        }
    }), f1134(v1, {
        let m4539 = match v0.clone() {
            Adt174::C0(_, _, _, _, _, _, _, _, h4538, _) => {
                h4538
            }
        };
        match m4539 {
            Adt173::C0(h4537, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4537
            }
        }
    }))));
    let v3: u32 = f1134(f1151(match v0.clone() {
        Adt174::C0(h4540, _, _, _, _, _, _, _, _, _) => {
            h4540
        }
    }), f1152(match v0.clone() {
        Adt174::C0(h4541, _, _, _, _, _, _, _, _, _) => {
            h4541
        }
    }, match v0.clone() {
        Adt174::C0(_, h4542, _, _, _, _, _, _, _, _) => {
            h4542
        }
    }, match v0.clone() {
        Adt174::C0(_, _, h4543, _, _, _, _, _, _, _) => {
            h4543
        }
    }));
    Ok(Adt174::C0(f1134(v2, v3), match v0.clone() {
        Adt174::C0(h4544, _, _, _, _, _, _, _, _, _) => {
            h4544
        }
    }, match v0.clone() {
        Adt174::C0(_, h4545, _, _, _, _, _, _, _, _) => {
            h4545
        }
    }, match v0.clone() {
        Adt174::C0(_, _, h4546, _, _, _, _, _, _, _) => {
            h4546
        }
    }, f1134(match v0.clone() {
        Adt174::C0(_, _, _, h4547, _, _, _, _, _, _) => {
            h4547
        }
    }, v2), match v0.clone() {
        Adt174::C0(_, _, _, _, h4548, _, _, _, _, _) => {
            h4548
        }
    }, match v0.clone() {
        Adt174::C0(_, _, _, _, _, h4549, _, _, _, _) => {
            h4549
        }
    }, match v0.clone() {
        Adt174::C0(_, _, _, _, _, _, h4550, _, _, _) => {
            h4550
        }
    }, f1153(match v0.clone() {
        Adt174::C0(_, _, _, _, _, _, _, _, h4551, _) => {
            h4551
        }
    }), {
        let a4553 = match v0 {
            Adt174::C0(_, _, _, _, _, _, _, _, _, h4552) => {
                h4552
            }
        };
        let a4554 = 1u64;
        nat_add(a4553, a4554)?
    }))
}

pub fn f1142(v0: u32) -> u64 {
    let m4556 = {
        let a4555 = v0;
        fixed_u64::convert(i128::from(a4555))
    };
    match m4556 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1143(v0: u64) -> u32 {
    let m4560 = {
        let a4559 = {
            let a4557 = v0;
            let a4558 = 4294967295u64;
            fixed_u64::bit_and(a4557, a4558)
        };
        fixed_u32::convert(i128::from(a4559))
    };
    match m4560 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1144(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt173> {
    Ok(Adt173::C0(f1154(v0.clone(), v1, v2, v3)?, f1154(v0.clone(), v1, v2, {
        let a4561 = v3;
        let a4562 = 4u64;
        nat_add(a4561, a4562)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4563 = v3;
        let a4564 = 8u64;
        nat_add(a4563, a4564)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4565 = v3;
        let a4566 = 12u64;
        nat_add(a4565, a4566)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4567 = v3;
        let a4568 = 16u64;
        nat_add(a4567, a4568)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4569 = v3;
        let a4570 = 20u64;
        nat_add(a4569, a4570)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4571 = v3;
        let a4572 = 24u64;
        nat_add(a4571, a4572)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4573 = v3;
        let a4574 = 28u64;
        nat_add(a4573, a4574)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4575 = v3;
        let a4576 = 32u64;
        nat_add(a4575, a4576)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4577 = v3;
        let a4578 = 36u64;
        nat_add(a4577, a4578)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4579 = v3;
        let a4580 = 40u64;
        nat_add(a4579, a4580)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4581 = v3;
        let a4582 = 44u64;
        nat_add(a4581, a4582)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4583 = v3;
        let a4584 = 48u64;
        nat_add(a4583, a4584)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4585 = v3;
        let a4586 = 52u64;
        nat_add(a4585, a4586)?
    })?, f1154(v0.clone(), v1, v2, {
        let a4587 = v3;
        let a4588 = 56u64;
        nat_add(a4587, a4588)?
    })?, f1154(v0, v1, v2, {
        let a4589 = v3;
        let a4590 = 60u64;
        nat_add(a4589, a4590)?
    })?))
}

pub fn f1145(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a4595 = f1155(v0, 24u32);
    let a4596 = {
        let a4593 = f1155(v1, 16u32);
        let a4594 = {
            let a4591 = f1155(v2, 8u32);
            let a4592 = v3;
            fixed_u32::bit_or(a4591, a4592)
        };
        fixed_u32::bit_or(a4593, a4594)
    };
    fixed_u32::bit_or(a4595, a4596)
}

pub fn f1146(v0: Bytes, v1: u64) -> u32 {
    let m4599 = {
        let a4597 = v0;
        let a4598 = v1;
        index_bytes(a4597, a4598)
    };
    match m4599 {
        None => {
            0u32
        }
        Some(v2) => {
            let m4601 = {
                let a4600 = v2;
                fixed_u32::convert(i128::from(a4600))
            };
            match m4601 {
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

pub fn f1147(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a4604 = {
        let a4602 = v4;
        let a4603 = 8u64;
        nat_add(a4602, a4603)?
    };
    let a4605 = v2;
    if nat_eq(a4604, a4605) {
        Ok(f1156(v3, true))
    } else {
        let a4608 = {
            let a4606 = v4;
            let a4607 = 4u64;
            nat_add(a4606, a4607)?
        };
        let a4609 = v2;
        if nat_eq(a4608, a4609) {
            Ok(f1156(v3, false))
        } else {
            Ok(f1145(f1157(v0.clone(), v1, v4), f1157(v0.clone(), v1, {
                let a4610 = v4;
                let a4611 = 1u64;
                nat_add(a4610, a4611)?
            }), f1157(v0.clone(), v1, {
                let a4612 = v4;
                let a4613 = 2u64;
                nat_add(a4612, a4613)?
            }), f1157(v0, v1, {
                let a4614 = v4;
                let a4615 = 3u64;
                nat_add(a4614, a4615)?
            })))
        }
    }
}

pub fn f1148(v0: u64) -> u32 {
    let a4616 = v0;
    let a4617 = 32u64;
    if nat_lt(a4616, a4617) {
        let a4618 = v0;
        let a4619 = 16u64;
        if nat_lt(a4618, a4619) {
            let a4620 = v0;
            let a4621 = 8u64;
            if nat_lt(a4620, a4621) {
                let a4622 = v0;
                let a4623 = 4u64;
                if nat_lt(a4622, a4623) {
                    let a4624 = v0;
                    let a4625 = 2u64;
                    if nat_lt(a4624, a4625) {
                        let a4626 = v0;
                        let a4627 = 1u64;
                        if nat_lt(a4626, a4627) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4628 = v0;
                        let a4629 = 3u64;
                        if nat_lt(a4628, a4629) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4630 = v0;
                    let a4631 = 6u64;
                    if nat_lt(a4630, a4631) {
                        let a4632 = v0;
                        let a4633 = 5u64;
                        if nat_lt(a4632, a4633) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4634 = v0;
                        let a4635 = 7u64;
                        if nat_lt(a4634, a4635) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4636 = v0;
                let a4637 = 12u64;
                if nat_lt(a4636, a4637) {
                    let a4638 = v0;
                    let a4639 = 10u64;
                    if nat_lt(a4638, a4639) {
                        let a4640 = v0;
                        let a4641 = 9u64;
                        if nat_lt(a4640, a4641) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4642 = v0;
                        let a4643 = 11u64;
                        if nat_lt(a4642, a4643) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4644 = v0;
                    let a4645 = 14u64;
                    if nat_lt(a4644, a4645) {
                        let a4646 = v0;
                        let a4647 = 13u64;
                        if nat_lt(a4646, a4647) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4648 = v0;
                        let a4649 = 15u64;
                        if nat_lt(a4648, a4649) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4650 = v0;
            let a4651 = 24u64;
            if nat_lt(a4650, a4651) {
                let a4652 = v0;
                let a4653 = 20u64;
                if nat_lt(a4652, a4653) {
                    let a4654 = v0;
                    let a4655 = 18u64;
                    if nat_lt(a4654, a4655) {
                        let a4656 = v0;
                        let a4657 = 17u64;
                        if nat_lt(a4656, a4657) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4658 = v0;
                        let a4659 = 19u64;
                        if nat_lt(a4658, a4659) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4660 = v0;
                    let a4661 = 22u64;
                    if nat_lt(a4660, a4661) {
                        let a4662 = v0;
                        let a4663 = 21u64;
                        if nat_lt(a4662, a4663) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4664 = v0;
                        let a4665 = 23u64;
                        if nat_lt(a4664, a4665) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4666 = v0;
                let a4667 = 28u64;
                if nat_lt(a4666, a4667) {
                    let a4668 = v0;
                    let a4669 = 26u64;
                    if nat_lt(a4668, a4669) {
                        let a4670 = v0;
                        let a4671 = 25u64;
                        if nat_lt(a4670, a4671) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4672 = v0;
                        let a4673 = 27u64;
                        if nat_lt(a4672, a4673) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4674 = v0;
                    let a4675 = 30u64;
                    if nat_lt(a4674, a4675) {
                        let a4676 = v0;
                        let a4677 = 29u64;
                        if nat_lt(a4676, a4677) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4678 = v0;
                        let a4679 = 31u64;
                        if nat_lt(a4678, a4679) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4680 = v0;
        let a4681 = 48u64;
        if nat_lt(a4680, a4681) {
            let a4682 = v0;
            let a4683 = 40u64;
            if nat_lt(a4682, a4683) {
                let a4684 = v0;
                let a4685 = 36u64;
                if nat_lt(a4684, a4685) {
                    let a4686 = v0;
                    let a4687 = 34u64;
                    if nat_lt(a4686, a4687) {
                        let a4688 = v0;
                        let a4689 = 33u64;
                        if nat_lt(a4688, a4689) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4690 = v0;
                        let a4691 = 35u64;
                        if nat_lt(a4690, a4691) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4692 = v0;
                    let a4693 = 38u64;
                    if nat_lt(a4692, a4693) {
                        let a4694 = v0;
                        let a4695 = 37u64;
                        if nat_lt(a4694, a4695) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4696 = v0;
                        let a4697 = 39u64;
                        if nat_lt(a4696, a4697) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4698 = v0;
                let a4699 = 44u64;
                if nat_lt(a4698, a4699) {
                    let a4700 = v0;
                    let a4701 = 42u64;
                    if nat_lt(a4700, a4701) {
                        let a4702 = v0;
                        let a4703 = 41u64;
                        if nat_lt(a4702, a4703) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4704 = v0;
                        let a4705 = 43u64;
                        if nat_lt(a4704, a4705) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4706 = v0;
                    let a4707 = 46u64;
                    if nat_lt(a4706, a4707) {
                        let a4708 = v0;
                        let a4709 = 45u64;
                        if nat_lt(a4708, a4709) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4710 = v0;
                        let a4711 = 47u64;
                        if nat_lt(a4710, a4711) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4712 = v0;
            let a4713 = 56u64;
            if nat_lt(a4712, a4713) {
                let a4714 = v0;
                let a4715 = 52u64;
                if nat_lt(a4714, a4715) {
                    let a4716 = v0;
                    let a4717 = 50u64;
                    if nat_lt(a4716, a4717) {
                        let a4718 = v0;
                        let a4719 = 49u64;
                        if nat_lt(a4718, a4719) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4720 = v0;
                        let a4721 = 51u64;
                        if nat_lt(a4720, a4721) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4722 = v0;
                    let a4723 = 54u64;
                    if nat_lt(a4722, a4723) {
                        let a4724 = v0;
                        let a4725 = 53u64;
                        if nat_lt(a4724, a4725) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4726 = v0;
                        let a4727 = 55u64;
                        if nat_lt(a4726, a4727) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4728 = v0;
                let a4729 = 60u64;
                if nat_lt(a4728, a4729) {
                    let a4730 = v0;
                    let a4731 = 58u64;
                    if nat_lt(a4730, a4731) {
                        let a4732 = v0;
                        let a4733 = 57u64;
                        if nat_lt(a4732, a4733) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4734 = v0;
                        let a4735 = 59u64;
                        if nat_lt(a4734, a4735) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4736 = v0;
                    let a4737 = 62u64;
                    if nat_lt(a4736, a4737) {
                        let a4738 = v0;
                        let a4739 = 61u64;
                        if nat_lt(a4738, a4739) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4740 = v0;
                        let a4741 = 63u64;
                        if nat_lt(a4740, a4741) {
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

pub fn f1149(v0: u32) -> u32 {
    let a4744 = {
        let a4742 = f1158(v0, 6u32, 26u32);
        let a4743 = f1158(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4742, a4743)
    };
    let a4745 = f1158(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4744, a4745)
}

pub fn f1150(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4751 = {
        let a4746 = v0;
        let a4747 = v1;
        fixed_u32::bit_and(a4746, a4747)
    };
    let a4752 = {
        let a4749 = {
            let a4748 = v0;
            fixed_u32::bit_not(a4748)
        };
        let a4750 = v2;
        fixed_u32::bit_and(a4749, a4750)
    };
    fixed_u32::bit_xor(a4751, a4752)
}

pub fn f1151(v0: u32) -> u32 {
    let a4755 = {
        let a4753 = f1158(v0, 2u32, 30u32);
        let a4754 = f1158(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4753, a4754)
    };
    let a4756 = f1158(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4755, a4756)
}

pub fn f1152(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4765 = {
        let a4761 = {
            let a4757 = v0;
            let a4758 = v1;
            fixed_u32::bit_and(a4757, a4758)
        };
        let a4762 = {
            let a4759 = v0;
            let a4760 = v2;
            fixed_u32::bit_and(a4759, a4760)
        };
        fixed_u32::bit_xor(a4761, a4762)
    };
    let a4766 = {
        let a4763 = v1;
        let a4764 = v2;
        fixed_u32::bit_and(a4763, a4764)
    };
    fixed_u32::bit_xor(a4765, a4766)
}

pub fn f1153(v0: Adt173) -> Adt173 {
    Adt173::C0(match v0.clone() {
        Adt173::C0(_, h4767, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4767
        }
    }, match v0.clone() {
        Adt173::C0(_, _, h4768, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4768
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, h4769, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4769
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, h4770, _, _, _, _, _, _, _, _, _, _, _) => {
            h4770
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, h4771, _, _, _, _, _, _, _, _, _, _) => {
            h4771
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, h4772, _, _, _, _, _, _, _, _, _) => {
            h4772
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, h4773, _, _, _, _, _, _, _, _) => {
            h4773
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, h4774, _, _, _, _, _, _, _) => {
            h4774
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, h4775, _, _, _, _, _, _) => {
            h4775
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, _, h4776, _, _, _, _, _) => {
            h4776
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, h4777, _, _, _, _) => {
            h4777
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4778, _, _, _) => {
            h4778
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4779, _, _) => {
            h4779
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4780, _) => {
            h4780
        }
    }, match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4781) => {
            h4781
        }
    }, f1134(f1159(match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4782, _) => {
            h4782
        }
    }), f1134(match v0.clone() {
        Adt173::C0(_, _, _, _, _, _, _, _, _, h4783, _, _, _, _, _, _) => {
            h4783
        }
    }, f1134(f1160(match v0.clone() {
        Adt173::C0(_, h4784, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4784
        }
    }), match v0 {
        Adt173::C0(h4785, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4785
        }
    }))))
}

pub fn f1154(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4788 = {
        let a4786 = v3;
        let a4787 = 8u64;
        nat_add(a4786, a4787)?
    };
    let a4789 = v2;
    if nat_eq(a4788, a4789) {
        Ok(f1161({
            let a4792 = {
                let a4790 = v1;
                let a4791 = 8u64;
                nat_mul(a4790, a4791)?
            };
            let a4793 = 4294967296u64;
            let a4794 = 0u64;
            nat_quot(a4792, a4793, a4794)
        }))
    } else {
        let a4797 = {
            let a4795 = v3;
            let a4796 = 4u64;
            nat_add(a4795, a4796)?
        };
        let a4798 = v2;
        if nat_eq(a4797, a4798) {
            Ok(f1161({
                let a4801 = {
                    let a4799 = v1;
                    let a4800 = 8u64;
                    nat_mul(a4799, a4800)?
                };
                let a4802 = 4294967296u64;
                let a4803 = 0u64;
                nat_rem(a4801, a4802, a4803)
            }))
        } else {
            Ok(f1145(f1157(v0.clone(), v1, v3), f1157(v0.clone(), v1, {
                let a4804 = v3;
                let a4805 = 1u64;
                nat_add(a4804, a4805)?
            }), f1157(v0.clone(), v1, {
                let a4806 = v3;
                let a4807 = 2u64;
                nat_add(a4806, a4807)?
            }), f1157(v0, v1, {
                let a4808 = v3;
                let a4809 = 3u64;
                nat_add(a4808, a4809)?
            })))
        }
    }
}

pub fn f1155(v0: u32, v1: u32) -> u32 {
    let m4812 = {
        let a4810 = v0;
        let a4811 = v1;
        fixed_u32::shift_left(a4810, a4811)
    };
    match m4812 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1156(v0: u64, v1: bool) -> u32 {
    if v1 {
        f1161({
            let a4813 = v0;
            let a4814 = 4294967296u64;
            let a4815 = 0u64;
            nat_quot(a4813, a4814, a4815)
        })
    } else {
        f1161({
            let a4816 = v0;
            let a4817 = 4294967296u64;
            let a4818 = 0u64;
            nat_rem(a4816, a4817, a4818)
        })
    }
}

pub fn f1157(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4819 = v2;
    let a4820 = v1;
    if nat_lt(a4819, a4820) {
        let m4823 = {
            let a4821 = v0;
            let a4822 = v2;
            index_bytes(a4821, a4822)
        };
        match m4823 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4825 = {
                    let a4824 = v3;
                    fixed_u32::convert(i128::from(a4824))
                };
                match m4825 {
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
        let a4826 = v2;
        let a4827 = v1;
        if nat_eq(a4826, a4827) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f1158(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4828 = f1137(v0, v1);
    let a4829 = f1155(v0, v2);
    fixed_u32::bit_or(a4828, a4829)
}

pub fn f1159(v0: u32) -> u32 {
    let a4832 = {
        let a4830 = f1158(v0, 17u32, 15u32);
        let a4831 = f1158(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4830, a4831)
    };
    let a4833 = f1137(v0, 10u32);
    fixed_u32::bit_xor(a4832, a4833)
}

pub fn f1160(v0: u32) -> u32 {
    let a4836 = {
        let a4834 = f1158(v0, 7u32, 25u32);
        let a4835 = f1158(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4834, a4835)
    };
    let a4837 = f1137(v0, 3u32);
    fixed_u32::bit_xor(a4836, a4837)
}

pub fn f1161(v0: u64) -> u32 {
    let (_, h4838) = {
        let (_, h4839) = f1162(Fn15::F1163, 32u64, (v0, (1u32, 0u32)));
        h4839
    };
    h4838
}

pub fn f1162(v0: Fn15, p4843: u64, p4844: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4842 = (p4843, p4844);
    {
        let mut m4842 = m4842;
        loop {
            let step = {
                let (v1, v2) = m4842;
                let m4840 = v1;
                if m4840 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4840 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4841: Fn15 = v0.clone();
                        c4841.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1163(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4845, _) = v0;
        h4845
    };
    let v2: u32 = {
        let (h4846, _) = {
            let (_, h4847) = v0;
            h4847
        };
        h4846
    };
    let v3: u32 = {
        let (_, h4848) = {
            let (_, h4849) = v0;
            h4849
        };
        h4848
    };
    ({
        let a4850 = v1;
        let a4851 = 2u64;
        let a4852 = 0u64;
        nat_quot(a4850, a4851, a4852)
    }, (f1155(v2, 1u32), {
        let a4856 = {
            let a4853 = v1;
            let a4854 = 2u64;
            let a4855 = 0u64;
            nat_rem(a4853, a4854, a4855)
        };
        let a4857 = 1u64;
        if nat_eq(a4856, a4857) {
            let a4858 = v3;
            let a4859 = v2;
            fixed_u32::bit_or(a4858, a4859)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
