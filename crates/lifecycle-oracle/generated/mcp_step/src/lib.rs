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
    C0(Adt4, Option<Str>, Adt7, Adt13, List<Adt15>, u64, Bytes),
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
    C0(List<Adt1>, Adt18),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt19),
    C1(Adt24),
    C2(Adt25),
    C3(Adt24),
    C4(Adt26),
    C5(Adt24),
    C6(Adt24),
    C7(Adt27),
    C8(Adt28),
    C9(Adt33),
    C10(Adt34),
    C11(Adt35),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Adt20),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Str, Adt21, Option<Adt8>, Option<Adt22>, Option<Adt23>, Adt8),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Option<Adt8>, Option<Adt8>, Option<Adt8>, Option<Adt8>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt22 {
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
pub enum Adt23 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt20, Option<Str>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt20, Str, Option<Adt8>, Option<Adt8>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Adt20, Str, Option<Adt8>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Adt20, Str, Option<List<(Str, Str)>>, Option<Adt8>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Adt20, Adt29, Adt31, Option<Adt32>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Adt30),
    C1(Str),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Str, Option<Str>),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Option<List<(Str, Str)>>),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt20, Str),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt20, Str, Adt8),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Adt20, Str),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Adt37),
    C1(Adt38),
    C2(Adt41),
    C3(Adt53),
    C4(Adt55),
    C5(Adt56),
    C6(Adt57),
    C7(Adt59),
    C8(Adt62),
    C9(Adt64),
    C10(Adt66),
    C11,
    C12(Adt85),
    C13(Adt86),
    C14(Adt91),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Adt13, List<Str>, Adt7, Option<Str>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(Adt13, Option<Str>, List<Adt39>),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(Str, Option<Str>, Option<List<Adt5>>, Option<Str>, Adt8, Option<Adt8>, Option<Adt40>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(Option<Str>, Option<bool>, Option<bool>, Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(List<Adt42>, Option<Adt9>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt42 {
    C0(Adt43),
    C1(Adt46),
    C2(Adt47),
    C3(Adt48),
    C4(Adt49),
}

#[derive(Clone)]
pub enum Adt43 {
    C0(Str, Option<Adt44>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt44 {
    C0(Option<List<Adt45>>, Option<Adt10>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt45 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt46 {
    C0(Str, Str, Option<Adt44>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt47 {
    C0(Str, Str, Option<Adt44>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt48 {
    C0(Str, Option<Str>, Option<List<Adt5>>, Str, Option<Str>, Option<Str>, Option<Adt44>, Option<u64>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt49 {
    C0(Adt50, Option<Adt44>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(Adt51),
    C1(Adt52),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(Str, Option<Str>, Str, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(Str, Option<Str>, Str, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Adt41, List<Adt54>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(Adt10, Option<Adt10>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(Adt13, Option<Str>, List<Adt48>),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(Adt13, List<Adt50>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(Adt13, Option<Str>, List<Adt58>),
}

#[derive(Clone)]
pub enum Adt58 {
    C0(Str, Option<Str>, Option<List<Adt5>>, Str, Option<Str>, Option<Str>, Option<Adt44>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt59 {
    C0(Adt13, Option<Str>, List<Adt60>),
}

#[derive(Clone)]
pub enum Adt60 {
    C0(Str, Option<Str>, Option<List<Adt5>>, Option<Str>, Option<List<Adt61>>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt61 {
    C0(Str, Option<Str>, Option<Str>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt62 {
    C0(Option<Str>, List<Adt63>),
}

#[derive(Clone)]
pub enum Adt63 {
    C0(Adt45, Adt42),
}

#[derive(Clone)]
pub enum Adt64 {
    C0(Adt65),
}

#[derive(Clone)]
pub enum Adt65 {
    C0(List<Str>, Option<u64>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt66 {
    C0(Adt67),
    C1(Adt67, List<(Str, Adt68)>),
    C2(Adt67, Adt8),
    C3(Adt67, Adt84),
    C4(Adt67),
}

#[derive(Clone)]
pub enum Adt67 {
    C0(Str, Option<Str>, Str, Str, Option<i64>, Option<i64>),
}

#[derive(Clone)]
pub enum Adt68 {
    C0(Adt69),
    C1(Adt73),
    C2(Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt69 {
    C0(Adt70),
    C1(Adt72),
}

#[derive(Clone)]
pub enum Adt70 {
    C0(Option<Adt71>, Str, Adt8),
}

#[derive(Clone)]
pub enum Adt71 {
    C0,
}

#[derive(Clone)]
pub enum Adt72 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt73 {
    C0(List<Adt74>, Option<Adt79>, Option<Str>, Option<Adt81>, Option<Adt10>, u64, Option<List<Str>>, Option<Adt8>, Option<List<Adt39>>, Option<Adt82>),
}

#[derive(Clone)]
pub enum Adt74 {
    C0(Adt45, Adt75, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt75 {
    C0(Adt76),
    C1(List<Adt76>),
}

#[derive(Clone)]
pub enum Adt76 {
    C0(Adt43),
    C1(Adt46),
    C2(Adt47),
    C3(Adt77),
    C4(Adt78),
}

#[derive(Clone)]
pub enum Adt77 {
    C0(Str, Str, Adt8, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt78 {
    C0(Str, List<Adt42>, Option<Adt9>, Option<bool>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt79 {
    C0(Option<List<Adt80>>, Option<Adt10>, Option<Adt10>, Option<Adt10>),
}

#[derive(Clone)]
pub enum Adt80 {
    C0(Option<Str>),
}

#[derive(Clone)]
pub enum Adt81 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt82 {
    C0(Option<Adt83>),
}

#[derive(Clone)]
pub enum Adt83 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt84 {
    C0(i64, Str, Option<Adt9>),
}

#[derive(Clone)]
pub enum Adt85 {
    C0(List<(Str, Adt68)>),
    C1(Str),
    C2(List<(Str, Adt68)>, Str),
}

#[derive(Clone)]
pub enum Adt86 {
    C0(Adt87, Adt67, Adt88),
}

#[derive(Clone)]
pub enum Adt87 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt88 {
    C0(Adt66, i64, Adt89),
}

#[derive(Clone)]
pub enum Adt89 {
    C0(i64, List<(Str, Adt68)>, i64, Adt90),
}

#[derive(Clone)]
pub enum Adt90 {
    C0(Adt8),
    C1(Adt84),
}

#[derive(Clone)]
pub enum Adt91 {
    C0(Adt88, Adt92, bool),
}

#[derive(Clone)]
pub enum Adt92 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt93 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str),
    C5(Str),
    C6(Str),
    C7(u64),
    C8(Adt21),
    C9(Str),
    C10,
    C11,
    C12(Str),
    C13,
    C14,
    C15(Adt94),
}

#[derive(Clone)]
pub enum Adt94 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt95 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt96 {
    C0(List<u64>),
    C1(Adt94),
}

#[derive(Clone)]
pub enum Adt97 {
    C0(Adt56),
    C1(Adt85),
    C2(Adt94),
}

#[derive(Clone)]
pub enum Adt98 {
    C0(Adt62),
    C1(Adt85),
    C2(Adt94),
}

#[derive(Clone)]
pub enum Adt99 {
    C0(List<u64>, List<u64>, List<Adt100>, Bytes, u64, List<List<u64>>, Bytes, Adt101),
}

#[derive(Clone)]
pub enum Adt100 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt101 {
    C0(i64, Str, Str, Bytes),
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
    C0(Adt41),
    C1(Adt85),
    C2(Adt86),
    C3(Adt94),
}

#[derive(Clone)]
pub enum Adt104 {
    C0(u64, List<Adt100>, List<List<u64>>, List<u64>, Option<List<u64>>),
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt53),
    C1(Adt85),
}

#[derive(Clone)]
pub enum Adt106 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt107 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt108 {
    C0,
    C1,
    C2(Adt8),
}

#[derive(Clone)]
pub enum Adt109 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt110 {
    C0(Adt9, bool, List<Str>),
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
    C0(Adt9, Adt9, List<Adt112>),
    C1(List<Adt9>, Adt9, List<Adt112>),
    C2(List<Adt9>, Adt9, List<Adt112>, u64),
    C3(List<Adt112>),
    C4(Option<Adt9>, Option<Adt9>, Adt9, List<Adt112>),
    C5(Adt115, Adt9, Adt9, List<Adt112>),
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
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt117 {
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
pub enum Adt118 {
    C0,
    C1(Option<Adt109>, Adt119),
}

#[derive(Clone)]
pub enum Adt119 {
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
pub enum Adt120 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt122 {
    C0(List<Adt15>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt123 {
    C0(Adt113, List<Adt114>),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(Adt109, Str, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt114>),
}

#[derive(Clone)]
pub enum Adt126 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt127 {
    C0(List<u64>),
    C1(u64, Adt9),
    C2(u64, List<Adt9>),
    C3(u64, List<Adt9>),
    C4(u64, List<(Str, Adt9)>),
    C5(u64, List<(Str, Adt9)>),
}

#[derive(Clone)]
pub enum Adt128 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt129>, List<Adt131>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt130),
    C7(Adt130),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt130 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Str, List<Str>, Adt16, Adt132),
}

#[derive(Clone)]
pub enum Adt132 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt133 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt134 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Adt136, List<Adt140>, u64),
}

#[derive(Clone)]
pub enum Adt136 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt137, List<u64>, bool),
    C7(u64, u64),
    C8(Adt138),
    C9(Adt9),
}

#[derive(Clone)]
pub enum Adt137 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(bool, Adt139, i64, u64, i64, bool, i64),
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
}

#[derive(Clone)]
pub enum Adt140 {
    C0(List<Adt9>),
    C1(List<(Str, Adt9)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Adt109, Result<Adt142, Adt145>),
}

#[derive(Clone)]
pub enum Adt142 {
    C0(Adt36, Adt143, List<Adt144>),
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Option<Adt4>, Option<Adt8>),
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt23, Adt54),
}

#[derive(Clone)]
pub enum Adt145 {
    C0(Adt146),
    C1(Str, List<Str>),
    C2(Adt129),
    C3(Str),
    C4(Adt93),
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
    C7(Str),
    C8(Str),
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Option<Str>, Option<Adt129>),
}

#[derive(Clone)]
pub enum Adt148 {
    C0,
    C1(Adt109),
    C2,
}

#[derive(Clone)]
pub enum Adt149 {
    C0(Adt117),
    C1(Adt135),
}

#[derive(Clone)]
pub enum Adt150 {
    C0(Adt102),
    C1(Adt122),
}

#[derive(Clone)]
pub enum Adt151 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt152 {
    C0(Adt117),
    C1(List<Adt127>, List<u64>),
}

#[derive(Clone)]
pub enum Adt153 {
    C0(Adt109, Adt154, Adt20, Adt8),
}

#[derive(Clone)]
pub enum Adt154 {
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
pub enum Adt155 {
    C0,
    C1,
    C2(Adt9),
}

#[derive(Clone)]
pub enum Adt156 {
    C0,
    C1(Adt154),
}

#[derive(Clone)]
pub enum Adt157 {
    C0(Adt158),
    C1(Adt145),
}

#[derive(Clone)]
pub enum Adt158 {
    C0(Adt109, Str, Adt20, Adt8),
}

#[derive(Clone)]
pub enum Adt159 {
    C0,
    C1,
    C2,
    C3,
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
    C0(Adt9),
    C1(Adt9),
}

#[derive(Clone)]
pub enum Adt164 {
    C0(List<Adt9>, List<Adt9>),
    C1(Str, List<(Str, Adt9)>, List<(Str, Adt9)>),
}

#[derive(Clone)]
pub enum Adt165 {
    C0,
    C1(List<(Adt9, Adt9)>),
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
    C1(Adt22),
}

#[derive(Clone)]
pub enum Adt169 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt170 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt171 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt170, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F4,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt17) -> R<Result<Adt36, Adt93>> {
        match self {
            Fn0::F4 => f4(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F25,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt101, p1: Adt18) -> R<Result<Adt36, Adt93>> {
        match self {
            Fn1::F25 => f25(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F41(List<Adt1>),
    F93,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt53, Adt93>> {
        match self {
            Fn2::F41(k0) => Ok(f41(k0.clone(), p0)),
            Fn2::F93 => f93(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F65(Fn0, Adt0),
    F267(Fn1, Adt99),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt18) -> R<Result<Adt36, Adt93>> {
        match self {
            Fn3::F65(k0, k1) => f65(k0.clone(), k1.clone(), p0),
            Fn3::F267(k0, k1) => f267(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F92,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt105, Adt93>> {
        match self {
            Fn4::F92 => f92(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F124(Fn2, Adt39),
    F128(Fn4, Adt39),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt25) -> R<Result<Adt36, Adt93>> {
        match self {
            Fn5::F124(k0, k1) => f124(k0.clone(), k1.clone(), p0),
            Fn5::F128(k0, k1) => f128(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F156,
}

impl Fn6 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt50>>> {
        match self {
            Fn6::F156 => f156(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F168,
}

impl Fn7 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt62 {
        match self {
            Fn7::F168 => f168(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F170,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt28) -> R<Result<List<Str>, Adt93>> {
        match self {
            Fn8::F170 => f170(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F1033(Bytes),
    F1058(Bytes, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn9::F1033(k0) => f1033(k0.clone(), p0),
            Fn9::F1058(k0, k1) => f1058(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F1038(Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn10::F1038(k0) => f1038(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F1043(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn11::F1043(k0) => f1043(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F1066(Bytes, u64, u64, u64),
    F1073(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt169)) -> R<(u64, Adt169)> {
        match self {
            Fn12::F1066(k0, k1, k2, k3) => f1066(k0.clone(), *k1, *k2, *k3, p0),
            Fn12::F1073(k0, k1, k2) => f1073(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F1068,
}

impl Fn13 {
    pub fn apply(&self, p0: Adt171) -> R<Adt171> {
        match self {
            Fn13::F1068 => f1068(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F1098,
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn14::F1098 => f1098(p0),
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
    Ok(Adt3::C0(f10(), None::<Str>, f11(), f12(), f13(f14()?)?, f15(), Bytes::lit(&[])))
}

pub fn f4(v0: Adt17) -> R<Result<Adt36, Adt93>> {
    let m3 = match v0.clone() {
        Adt17::C0(_, h2) => {
            h2
        }
    };
    match m3 {
        Adt18::C0(_) => {
            f19(match v0 {
                Adt17::C0(_, h16) => {
                    h16
                }
            })
        }
        Adt18::C1(_) => {
            f19(match v0 {
                Adt17::C0(_, h17) => {
                    h17
                }
            })
        }
        Adt18::C2(v1) => {
            f16(match v0.clone() {
                Adt17::C0(h4, _) => {
                    h4
                }
            }, match v0 {
                Adt17::C0(_, h5) => {
                    h5
                }
            }, v1)
        }
        Adt18::C3(_) => {
            f19(match v0 {
                Adt17::C0(_, h18) => {
                    h18
                }
            })
        }
        Adt18::C4(v2) => {
            let a7 = match v2.clone() {
                Adt26::C0(_, h6, _, _) => {
                    h6
                }
            };
            let a8 = Str::lit("effect://item");
            if equal(a7, a8) {
                Ok(f17(f18(match v0 {
                    Adt17::C0(h9, _) => {
                        h9
                    }
                }, v2)))
            } else {
                f19(match v0 {
                    Adt17::C0(_, h10) => {
                        h10
                    }
                })
            }
        }
        Adt18::C5(_) => {
            f19(match v0 {
                Adt17::C0(_, h19) => {
                    h19
                }
            })
        }
        Adt18::C6(_) => {
            f19(match v0 {
                Adt17::C0(_, h20) => {
                    h20
                }
            })
        }
        Adt18::C7(v3) => {
            let a12 = match v3.clone() {
                Adt27::C0(_, h11, _, _, _) => {
                    h11
                }
            };
            let a13 = Str::lit("effect_prompt");
            if equal(a12, a13) {
                Ok(f20(f21(match v0 {
                    Adt17::C0(h14, _) => {
                        h14
                    }
                }, v3)))
            } else {
                f19(match v0 {
                    Adt17::C0(_, h15) => {
                        h15
                    }
                })
            }
        }
        Adt18::C8(_) => {
            f19(match v0 {
                Adt17::C0(_, h21) => {
                    h21
                }
            })
        }
        Adt18::C9(_) => {
            f19(match v0 {
                Adt17::C0(_, h22) => {
                    h22
                }
            })
        }
        Adt18::C10(_) => {
            f19(match v0 {
                Adt17::C0(_, h23) => {
                    h23
                }
            })
        }
        Adt18::C11(_) => {
            f19(match v0 {
                Adt17::C0(_, h24) => {
                    h24
                }
            })
        }
    }
}

pub fn f5(v0: Bytes, v1: Adt0) -> R<List<u64>> {
    if f22(f23(match v1.clone() {
        Adt0::C0(h25, _) => {
            h25
        }
    })?)? {
        f24(f3()?, Fn1::F25, match v1 {
            Adt0::C0(h26, _) => {
                h26
            }
        })
    } else {
        f2(f3()?, Fn0::F4, v0)
    }
}

pub fn f6(v0: Bytes, v1: Option<Adt95>) -> R<Result<Adt0, Adt2>> {
    let m27 = v1;
    match m27 {
        None => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(0u64)))
        }
        Some(v2) => {
            f26(v0.clone(), v2.clone(), f27(v0, match v2 {
                Adt95::C0(_, h28) => {
                    h28
                }
            })?)
        }
    }
}

pub fn f7(v0: Bytes, v1: u64) -> R<Option<Adt95>> {
    let m29 = f27(v0.clone(), v1)?;
    match m29 {
        None => {
            Ok(None::<Adt95>)
        }
        Some(v2) => {
            let m35 = {
                let a32 = v0;
                let a33 = {
                    let a30 = v1;
                    let a31 = 4u64;
                    nat_add(a30, a31)?
                };
                let a34 = v2;
                slice_bytes(a32, a33, a34)
            };
            match m35 {
                None => {
                    Ok(None::<Adt95>)
                }
                Some(v3) => {
                    Ok(Some(Adt95::C0(v3, {
                        let a38 = {
                            let a36 = v1;
                            let a37 = 4u64;
                            nat_add(a36, a37)?
                        };
                        let a39 = v2;
                        nat_add(a38, a39)?
                    })))
                }
            }
        }
    }
}

pub fn f8(v0: Adt96) -> R<List<u64>> {
    let m40 = v0;
    match m40 {
        Adt96::C0(v1) => {
            let a41 = f28(0u64);
            let a42 = v1;
            Ok(append_list(a41, a42))
        }
        Adt96::C1(v2) => {
            let a47 = f28(1u64);
            let a48 = {
                let a45 = f28(match v2.clone() {
                    Adt94::C0(h43, _) => {
                        h43
                    }
                });
                let a46 = f29(match v2 {
                    Adt94::C0(_, h44) => {
                        h44
                    }
                })?;
                append_list(a45, a46)
            };
            Ok(append_list(a47, a48))
        }
    }
}

pub fn f9(v0: Adt3, v1: Fn0, v2: Bytes) -> R<Adt96> {
    let m49 = f1(v2)?;
    match m49 {
        Ok(v4) => {
            f31(v0, v1, v4.clone(), f23(match v4 {
                Adt0::C0(h50, _) => {
                    h50
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

pub fn f13(v0: List<Adt39>) -> R<List<Adt15>> {
    let m51 = f34(v0)?;
    match m51 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt15>::nil())
        }
    }
}

pub fn f14() -> R<List<Adt39>> {
    let a54 = {
        let a52 = f35()?;
        let a53 = f36();
        append_list(a52, a53)
    };
    let a55 = f37();
    Ok(append_list(a54, a55))
}

pub fn f15() -> u64 {
    100u64
}

pub fn f16(v0: List<Adt1>, v1: Adt18, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    if f38(v2.clone()) {
        f39(f40()?, Fn2::F41(v0), v2)
    } else {
        if f42(v2, Str::lit("provider_effect")) {
            Ok(f43(f44(v0)))
        } else {
            f19(v1)
        }
    }
}

pub fn f17(v0: Result<Adt97, Adt93>) -> Result<Adt36, Adt93> {
    let m56 = v0;
    match m56 {
        Ok(v1) => {
            let m57 = v1;
            match m57 {
                Adt97::C0(v2) => {
                    Ok::<Adt36, Adt93>(Adt36::C5(v2))
                }
                Adt97::C1(v3) => {
                    Ok::<Adt36, Adt93>(Adt36::C12(v3))
                }
                Adt97::C2(v4) => {
                    Err::<Adt36, Adt93>(Adt93::C15(v4))
                }
            }
        }
        Err(v5) => {
            Err::<Adt36, Adt93>(v5)
        }
    }
}

pub fn f18(v0: List<Adt1>, v1: Adt26) -> Result<Adt97, Adt93> {
    let m58 = v0;
    match m58.uncons() {
        None => {
            Ok::<Adt97, Adt93>(Adt97::C2(f45(4u64, match v1 {
                Adt26::C0(_, h59, _, _) => {
                    h59
                }
            })))
        }
        Some((v2, v3)) => {
            let m60 = v3;
            match m60.uncons() {
                None => {
                    let a62 = match v2.clone() {
                        Adt1::C0(h61, _) => {
                            h61
                        }
                    };
                    let a63 = 4u64;
                    if nat_eq(a62, a63) {
                        Ok::<Adt97, Adt93>(Adt97::C0(f46(match v1 {
                            Adt26::C0(_, h64, _, _) => {
                                h64
                            }
                        }, f47(Str::lit("item:"), f48(match v2 {
                            Adt1::C0(_, h65) => {
                                h65
                            }
                        })))))
                    } else {
                        Err::<Adt97, Adt93>(f49())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt97, Adt93>(f49())
                }
            }
        }
    }
}

pub fn f19(v0: Adt18) -> R<Result<Adt36, Adt93>> {
    let m66 = f50(f14()?)?;
    match m66 {
        None => {
            f51(v0)
        }
        Some(v1) => {
            Ok(Err::<Adt36, Adt93>(v1))
        }
    }
}

pub fn f20(v0: Result<Adt98, Adt93>) -> Result<Adt36, Adt93> {
    let m67 = v0;
    match m67 {
        Ok(v1) => {
            let m68 = v1;
            match m68 {
                Adt98::C0(v2) => {
                    Ok::<Adt36, Adt93>(Adt36::C8(v2))
                }
                Adt98::C1(v3) => {
                    Ok::<Adt36, Adt93>(Adt36::C12(v3))
                }
                Adt98::C2(v4) => {
                    Err::<Adt36, Adt93>(Adt93::C15(v4))
                }
            }
        }
        Err(v5) => {
            Err::<Adt36, Adt93>(v5)
        }
    }
}

pub fn f21(v0: List<Adt1>, v1: Adt27) -> Result<Adt98, Adt93> {
    let m69 = v0;
    match m69.uncons() {
        None => {
            Ok::<Adt98, Adt93>(Adt98::C2(f45(5u64, match v1 {
                Adt27::C0(_, h70, _, _, _) => {
                    h70
                }
            })))
        }
        Some((v2, v3)) => {
            let m71 = v3;
            match m71.uncons() {
                None => {
                    let a73 = match v2.clone() {
                        Adt1::C0(h72, _) => {
                            h72
                        }
                    };
                    let a74 = 5u64;
                    if nat_eq(a73, a74) {
                        Ok::<Adt98, Adt93>(Adt98::C0(f52(f47(Str::lit("prompt:"), f48(match v2 {
                            Adt1::C0(_, h75) => {
                                h75
                            }
                        })))))
                    } else {
                        Err::<Adt98, Adt93>(f49())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt98, Adt93>(f49())
                }
            }
        }
    }
}

pub fn f22(v0: Result<Adt99, Adt2>) -> R<bool> {
    let m76 = v0;
    match m76 {
        Ok(v2) => {
            Ok(f53(match v2 {
                Adt99::C0(_, h77, _, _, _, _, _, _) => {
                    h77
                }
            }, f54(Str::lit("byte-serve"))?))
        }
        Err(_) => {
            Ok(false)
        }
    }
}

pub fn f23(v0: Bytes) -> R<Result<Adt99, Adt2>> {
    let m78 = f7(v0.clone(), 0u64)?;
    match m78 {
        None => {
            Ok(Err::<Adt99, Adt2>(Adt2::C0(0u64)))
        }
        Some(v1) => {
            f55(v0.clone(), v1.clone(), f7(v0, match v1 {
                Adt95::C0(_, h79) => {
                    h79
                }
            })?)
        }
    }
}

pub fn f24(v0: Adt3, v1: Fn1, v2: Bytes) -> R<List<u64>> {
    let m80 = f23(v2)?;
    match m80 {
        Ok(v4) => {
            Ok(f56(f58(v0, v1, v4)?))
        }
        Err(_) => {
            Ok(f56(f57()))
        }
    }
}

pub fn f25(_: Adt101, v1: Adt18) -> R<Result<Adt36, Adt93>> {
    f59(v1)
}

pub fn f26(v0: Bytes, v1: Adt95, v2: Option<u64>) -> R<Result<Adt0, Adt2>> {
    let m81 = v2;
    match m81 {
        None => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(match v1 {
                Adt95::C0(_, h82) => {
                    h82
                }
            })))
        }
        Some(v3) => {
            let m86 = f60(v0.clone(), {
                let a84 = match v1.clone() {
                    Adt95::C0(_, h83) => {
                        h83
                    }
                };
                let a85 = 4u64;
                nat_add(a84, a85)?
            }, v3, List::<Adt1>::nil())?;
            match m86 {
                Ok(v5) => {
                    Ok(f61(v0, match v1 {
                        Adt95::C0(h87, _) => {
                            h87
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
    let a91 = {
        let a88 = v1;
        let a89 = 4u64;
        nat_add(a88, a89)?
    };
    let a92 = {
        let a90 = v0.clone();
        length_bytes(a90)
    };
    if nat_le(a91, a92) {
        Ok(Some({
            let a109 = {
                let a93 = f62(v0.clone(), v1)?;
                let a94 = 16777216u64;
                nat_mul(a93, a94)?
            };
            let a110 = {
                let a107 = {
                    let a97 = f62(v0.clone(), {
                        let a95 = v1;
                        let a96 = 1u64;
                        nat_add(a95, a96)?
                    })?;
                    let a98 = 65536u64;
                    nat_mul(a97, a98)?
                };
                let a108 = {
                    let a105 = {
                        let a101 = f62(v0.clone(), {
                            let a99 = v1;
                            let a100 = 2u64;
                            nat_add(a99, a100)?
                        })?;
                        let a102 = 256u64;
                        nat_mul(a101, a102)?
                    };
                    let a106 = f62(v0, {
                        let a103 = v1;
                        let a104 = 3u64;
                        nat_add(a103, a104)?
                    })?;
                    nat_add(a105, a106)?
                };
                nat_add(a107, a108)?
            };
            nat_add(a109, a110)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f28(v0: u64) -> List<u64> {
    List::cons({
        let a114 = {
            let a111 = v0;
            let a112 = 16777216u64;
            let a113 = 0u64;
            nat_quot(a111, a112, a113)
        };
        let a115 = 256u64;
        let a116 = 0u64;
        nat_rem(a114, a115, a116)
    }, List::cons({
        let a120 = {
            let a117 = v0;
            let a118 = 65536u64;
            let a119 = 0u64;
            nat_quot(a117, a118, a119)
        };
        let a121 = 256u64;
        let a122 = 0u64;
        nat_rem(a120, a121, a122)
    }, List::cons({
        let a126 = {
            let a123 = v0;
            let a124 = 256u64;
            let a125 = 0u64;
            nat_quot(a123, a124, a125)
        };
        let a127 = 256u64;
        let a128 = 0u64;
        nat_rem(a126, a127, a128)
    }, List::cons({
        let a129 = v0;
        let a130 = 256u64;
        let a131 = 0u64;
        nat_rem(a129, a130, a131)
    }, List::<u64>::nil()))))
}

pub fn f29(v0: Bytes) -> R<List<u64>> {
    f63(v0.clone(), {
        let a132 = v0;
        length_bytes(a132)
    }, List::<u64>::nil())
}

pub fn f30() -> Adt96 {
    Adt96::C0(f56(f57()))
}

pub fn f31(v0: Adt3, v1: Fn0, v2: Adt0, v3: Result<Adt99, Adt2>) -> R<Adt96> {
    let m133 = v3;
    match m133 {
        Ok(v5) => {
            f64(v0, Fn3::F65(v1, v2), v5)
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

pub fn f34(v0: List<Adt39>) -> R<Result<List<Adt15>, Adt102>> {
    let m134 = f66(v0.clone());
    match m134 {
        None => {
            f67(v0, List::<Adt15>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt102>(v1))
        }
    }
}

pub fn f35() -> R<List<Adt39>> {
    Ok(List::cons(f68(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f69()), List::cons(f68(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f70()), List::cons(f68(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f69()), List::cons(f68(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f71()), List::cons(f68(Str::lit("test_image_content"), Str::lit("Tests image content response"), f71()), List::cons(f68(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f71()), List::cons(f68(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f71()), List::cons(f68(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f71()), List::cons(f68(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f71()), List::cons(f68(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f71()), List::cons(f68(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f72()), List::cons(f68(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f73()?), List::cons(f68(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f74()), List::cons(f75(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f71(), f76()), List::cons(f75(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f71(), f76()), List::cons(f68(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f71()), List::cons(f68(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f71()), List::cons(f68(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f71()), List::cons(f68(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f71()), List::cons(f68(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f71()), List::<Adt39>::nil())))))))))))))))))))))
}

pub fn f36() -> List<Adt39> {
    List::cons(f77(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f77(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f77(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f77(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f77(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f77(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f77(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f77(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt39>::nil()))))))))
}

pub fn f37() -> List<Adt39> {
    List::cons(f78(Str::lit("greet"), Str::lit("Answers a greeting at once; never a task"), f79(Str::lit("name"))), List::cons(f78(Str::lit("slow_compute"), Str::lit("Computes for a number of seconds; a task for a client that declared the tasks extension"), f80()), List::cons(f78(Str::lit("failing_job"), Str::lit("Always fails; always a task, so the client must declare the tasks extension"), f81()), List::cons(f78(Str::lit("protocol_error_job"), Str::lit("Ends in a protocol error; a task for a client that declared the tasks extension"), f81()), List::cons(f78(Str::lit("confirm_delete"), Str::lit("Asks the client to confirm a deletion; a task that waits for input"), f79(Str::lit("filename"))), List::cons(f78(Str::lit("multi_input"), Str::lit("Asks the client two questions at once; a task that waits for both answers"), f81()), List::cons(f78(Str::lit("test_tool_with_task"), Str::lit("Asks for a name, then becomes a task; the client must declare the tasks extension"), f81()), List::<Adt39>::nil())))))))
}

pub fn f38(v0: Adt25) -> bool {
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

pub fn f39(v0: List<Adt39>, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    let m136 = f82(match v2.clone() {
        Adt25::C0(_, h135, _, _, _) => {
            h135
        }
    }, v0);
    match m136 {
        None => {
            Ok(Err::<Adt36, Adt93>(Adt93::C0(match v2 {
                Adt25::C0(_, h137, _, _, _) => {
                    h137
                }
            })))
        }
        Some(v3) => {
            f83(v3, v1, v2)
        }
    }
}

pub fn f40() -> R<List<Adt39>> {
    Ok(List::cons(f68(Str::lit("effect_read"), Str::lit("Reads a key through the host"), f84()?), List::cons(f68(Str::lit("effect_chain"), Str::lit("Reads two keys through the host, the second after the first"), f85()?), List::cons(f68(Str::lit("effect_edge_ok"), Str::lit("Asks for the largest effect tag"), f69()), List::cons(f68(Str::lit("effect_edge_over"), Str::lit("Asks for a tag that does not fit thirty-two bits"), f69()), List::<Adt39>::nil())))))
}

pub fn f41(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt93> {
    f86(v0, v1)
}

pub fn f42(v0: Adt25, v1: Str) -> bool {
    let a139 = match v0 {
        Adt25::C0(_, h138, _, _, _) => {
            h138
        }
    };
    let a140 = v1;
    equal(a139, a140)
}

pub fn f43(v0: Result<Adt103, Adt93>) -> Result<Adt36, Adt93> {
    let m141 = v0;
    match m141 {
        Ok(v1) => {
            let m142 = v1;
            match m142 {
                Adt103::C0(v2) => {
                    Ok::<Adt36, Adt93>(Adt36::C2(v2))
                }
                Adt103::C1(v3) => {
                    Ok::<Adt36, Adt93>(Adt36::C12(v3))
                }
                Adt103::C2(v4) => {
                    Ok::<Adt36, Adt93>(Adt36::C13(v4))
                }
                Adt103::C3(v5) => {
                    Err::<Adt36, Adt93>(Adt93::C15(v5))
                }
            }
        }
        Err(v6) => {
            Err::<Adt36, Adt93>(v6)
        }
    }
}

pub fn f44(v0: List<Adt1>) -> Result<Adt103, Adt93> {
    let m143 = v0;
    match m143.uncons() {
        None => {
            Ok::<Adt103, Adt93>(Adt103::C3(f45(3u64, Str::lit("call"))))
        }
        Some((v1, v2)) => {
            let m144 = v2;
            match m144.uncons() {
                None => {
                    let a146 = match v1.clone() {
                        Adt1::C0(h145, _) => {
                            h145
                        }
                    };
                    let a147 = 3u64;
                    if nat_eq(a146, a147) {
                        Ok::<Adt103, Adt93>(Adt103::C0(f87(f47(Str::lit("call:"), f48(match v1 {
                            Adt1::C0(_, h148) => {
                                h148
                            }
                        })), false)))
                    } else {
                        Err::<Adt103, Adt93>(f49())
                    }
                }
                Some((_, _)) => {
                    Err::<Adt103, Adt93>(f49())
                }
            }
        }
    }
}

pub fn f45(v0: u64, v1: Str) -> Adt94 {
    Adt94::C0(v0, {
        let a149 = v1;
        utf8_encode(a149)
    })
}

pub fn f46(v0: Str, v1: Str) -> Adt56 {
    Adt56::C0(f12(), List::cons(Adt50::C0(Adt51::C0(v0, Some(Str::lit("text/plain")), v1, None::<Adt8>)), List::<Adt50>::nil()))
}

pub fn f47(v0: Str, v1: Str) -> Str {
    let a150 = List::cons(v0, List::cons(v1, List::<Str>::nil()));
    let a151 = Str::lit("");
    join(a150, a151)
}

pub fn f48(v0: Bytes) -> Str {
    let m153 = {
        let a152 = v0;
        utf8_decode(a152)
    };
    match m153 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f49() -> Adt93 {
    Adt93::C9(Str::lit("the outcomes of the effects do not match the request"))
}

pub fn f50(v0: List<Adt39>) -> R<Option<Adt93>> {
    let m154 = f34(v0)?;
    match m154 {
        Ok(_) => {
            Ok(None::<Adt93>)
        }
        Err(v1) => {
            Ok(Some(Adt93::C9(f88(v1))))
        }
    }
}

pub fn f51(v0: Adt18) -> R<Result<Adt36, Adt93>> {
    let m155 = v0;
    match m155 {
        Adt18::C0(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt18::C1(v1) => {
            Ok(f89(f12(), f14()?, v1))
        }
        Adt18::C2(v2) => {
            if f90(match v2.clone() {
                Adt25::C0(_, h156, _, _, _) => {
                    h156
                }
            }) {
                f91(f14()?, Fn4::F92, v2)
            } else {
                f39(f14()?, Fn2::F93, v2)
            }
        }
        Adt18::C3(v4) => {
            Ok(f94(v4))
        }
        Adt18::C4(v5) => {
            f95(v5)
        }
        Adt18::C5(v6) => {
            Ok(f96(v6))
        }
        Adt18::C6(v7) => {
            Ok(f97(v7))
        }
        Adt18::C7(v8) => {
            if f98(match v8.clone() {
                Adt27::C0(_, h157, _, _, _) => {
                    h157
                }
            }) {
                Ok(f99(v8))
            } else {
                Ok(f100(v8))
            }
        }
        Adt18::C8(v9) => {
            f101(v9)
        }
        Adt18::C9(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt18::C10(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt18::C11(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the fixture offers tools only"))))
        }
    }
}

pub fn f52(v0: Str) -> Adt62 {
    Adt62::C0(None::<Str>, List::cons(Adt63::C0(Adt45::C0, f102(v0)), List::<Adt63>::nil()))
}

pub fn f53(p164: List<u64>, p165: List<u64>) -> bool {
    let m163 = (p164, p165);
    {
        let mut m163 = m163;
        loop {
            let step = {
                let (v0, v1) = m163;
                let m158 = v0;
                match m158.uncons() {
                    None => {
                        let m159 = v1;
                        match m159.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m160 = v1;
                        match m160.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a161 = v4;
                                let a162 = v6;
                                if nat_eq(a161, a162) {
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
                Ok(next) => { m163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Str) -> R<List<u64>> {
    f29({
        let a166 = v0;
        utf8_encode(a166)
    })
}

pub fn f55(v0: Bytes, v1: Adt95, v2: Option<Adt95>) -> R<Result<Adt99, Adt2>> {
    let m167 = v2;
    match m167 {
        None => {
            Ok(Err::<Adt99, Adt2>(Adt2::C0(match v1 {
                Adt95::C0(_, h168) => {
                    h168
                }
            })))
        }
        Some(v3) => {
            f103(v0.clone(), v1, v3.clone(), f27(v0, match v3 {
                Adt95::C0(_, h169) => {
                    h169
                }
            })?)
        }
    }
}

pub fn f56(v0: Adt104) -> List<u64> {
    let a193 = f28(match v0.clone() {
        Adt104::C0(h170, _, _, _, _) => {
            h170
        }
    });
    let a194 = {
        let a191 = f28({
            let a172 = match v0.clone() {
                Adt104::C0(_, h171, _, _, _) => {
                    h171
                }
            };
            length_list(a172)
        });
        let a192 = {
            let a189 = f104(match v0.clone() {
                Adt104::C0(_, h173, _, _, _) => {
                    h173
                }
            });
            let a190 = {
                let a187 = f28({
                    let a175 = match v0.clone() {
                        Adt104::C0(_, _, h174, _, _) => {
                            h174
                        }
                    };
                    length_list(a175)
                });
                let a188 = {
                    let a185 = f105(match v0.clone() {
                        Adt104::C0(_, _, h176, _, _) => {
                            h176
                        }
                    });
                    let a186 = {
                        let a183 = f28({
                            let a178 = match v0.clone() {
                                Adt104::C0(_, _, _, h177, _) => {
                                    h177
                                }
                            };
                            length_list(a178)
                        });
                        let a184 = {
                            let a181 = match v0.clone() {
                                Adt104::C0(_, _, _, h179, _) => {
                                    h179
                                }
                            };
                            let a182 = f106(match v0 {
                                Adt104::C0(_, _, _, _, h180) => {
                                    h180
                                }
                            });
                            append_list(a181, a182)
                        };
                        append_list(a183, a184)
                    };
                    append_list(a185, a186)
                };
                append_list(a187, a188)
            };
            append_list(a189, a190)
        };
        append_list(a191, a192)
    };
    append_list(a193, a194)
}

pub fn f57() -> Adt104 {
    Adt104::C0(f107(), f108(), List::<List<u64>>::nil(), f109(), None::<List<u64>>)
}

pub fn f58(v0: Adt3, v1: Fn1, v2: Adt99) -> R<Adt104> {
    f110(v0, v1, v2.clone(), f111(match v2.clone() {
        Adt99::C0(_, _, h195, _, _, _, _, _) => {
            h195
        }
    }, match v2 {
        Adt99::C0(_, _, _, _, _, h196, _, _) => {
            h196
        }
    })?)
}

pub fn f59(v0: Adt18) -> R<Result<Adt36, Adt93>> {
    f4(Adt17::C0(List::<Adt1>::nil(), v0))
}

pub fn f60(v0: Bytes, p207: u64, p208: u64, p209: List<Adt1>) -> R<Result<(List<Adt1>, u64), Adt2>> {
    let m206 = (p207, (p208, p209));
    {
        let mut m206 = m206;
        loop {
            let step = {
                let (v1, (v2, v3)) = m206;
                let m197 = v2;
                if m197 == 0 {
                    Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Ok::<(List<Adt1>, u64), Adt2>((f112(v3, List::<Adt1>::nil()), v1))))
                } else {
                    let v4 = m197 - 1;
                    let m198 = f27(v0.clone(), v1)?;
                    match m198 {
                        None => {
                            Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Err::<(List<Adt1>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            let m201 = f7(v0.clone(), {
                                let a199 = v1;
                                let a200 = 4u64;
                                nat_add(a199, a200)?
                            })?;
                            match m201 {
                                None => {
                                    Err::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>(Ok(Err::<(List<Adt1>, u64), Adt2>(Adt2::C0({
                                        let a202 = v1;
                                        let a203 = 4u64;
                                        nat_add(a202, a203)?
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt1>)), R<Result<(List<Adt1>, u64), Adt2>>>((match v6.clone() {
                                        Adt95::C0(_, h204) => {
                                            h204
                                        }
                                    }, (v4, List::cons(Adt1::C0(v5, match v6 {
                                        Adt95::C0(h205, _) => {
                                            h205
                                        }
                                    }), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Bytes, v1: Bytes, v2: (List<Adt1>, u64)) -> Result<Adt0, Adt2> {
    f113({
        let a212 = {
            let (_, h210) = v2.clone();
            h210
        };
        let a213 = {
            let a211 = v0;
            length_bytes(a211)
        };
        nat_eq(a212, a213)
    }, v1, {
        let (h214, _) = v2;
        h214
    }, {
        let (_, h215) = v2;
        h215
    })
}

pub fn f62(v0: Bytes, v1: u64) -> R<u64> {
    let m218 = {
        let a216 = v0;
        let a217 = v1;
        index_bytes(a216, a217)
    };
    match m218 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a263 = {
                let a239 = {
                    let a227 = {
                        let a221 = {
                            let a219 = v3;
                            let a220 = 128u8;
                            fixed_u8::bit_and(a219, a220)
                        };
                        let a222 = 128u8;
                        if equal(a221, a222) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a228 = {
                        let a225 = {
                            let a223 = v3;
                            let a224 = 64u8;
                            fixed_u8::bit_and(a223, a224)
                        };
                        let a226 = 64u8;
                        if equal(a225, a226) {
                            64u64
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
                            let a230 = 32u8;
                            fixed_u8::bit_and(a229, a230)
                        };
                        let a232 = 32u8;
                        if equal(a231, a232) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a238 = {
                        let a235 = {
                            let a233 = v3;
                            let a234 = 16u8;
                            fixed_u8::bit_and(a233, a234)
                        };
                        let a236 = 16u8;
                        if equal(a235, a236) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a237, a238)?
                };
                nat_add(a239, a240)?
            };
            let a264 = {
                let a261 = {
                    let a249 = {
                        let a243 = {
                            let a241 = v3;
                            let a242 = 8u8;
                            fixed_u8::bit_and(a241, a242)
                        };
                        let a244 = 8u8;
                        if equal(a243, a244) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a250 = {
                        let a247 = {
                            let a245 = v3;
                            let a246 = 4u8;
                            fixed_u8::bit_and(a245, a246)
                        };
                        let a248 = 4u8;
                        if equal(a247, a248) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a249, a250)?
                };
                let a262 = {
                    let a259 = {
                        let a253 = {
                            let a251 = v3;
                            let a252 = 2u8;
                            fixed_u8::bit_and(a251, a252)
                        };
                        let a254 = 2u8;
                        if equal(a253, a254) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a260 = {
                        let a257 = {
                            let a255 = v3;
                            let a256 = 1u8;
                            fixed_u8::bit_and(a255, a256)
                        };
                        let a258 = 1u8;
                        if equal(a257, a258) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a259, a260)?
                };
                nat_add(a261, a262)?
            };
            nat_add(a263, a264)
        }
    }
}

pub fn f63(v0: Bytes, p267: u64, p268: List<u64>) -> R<List<u64>> {
    let m266 = (p267, p268);
    {
        let mut m266 = m266;
        loop {
            let step = {
                let (v1, v2) = m266;
                let m265 = v1;
                if m265 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m265 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f62(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt3, v1: Fn3, v2: Adt99) -> R<Adt96> {
    f114(v0, v1, v2.clone(), f111(match v2.clone() {
        Adt99::C0(_, _, h269, _, _, _, _, _) => {
            h269
        }
    }, match v2 {
        Adt99::C0(_, _, _, _, _, h270, _, _) => {
            h270
        }
    })?)
}

pub fn f65(v0: Fn0, v1: Adt0, v2: Adt18) -> R<Result<Adt36, Adt93>> {
    let c271: Fn0 = v0;
    c271.apply(Adt17::C0(match v1 {
        Adt0::C0(_, h272) => {
            h272
        }
    }, v2))
}

pub fn f66(v0: List<Adt39>) -> Option<Adt102> {
    let m273 = f115(v0, List::<Str>::nil());
    match m273 {
        None => {
            None::<Adt102>
        }
        Some(v1) => {
            Some(Adt102::C1(v1))
        }
    }
}

pub fn f67(p277: List<Adt39>, p278: List<Adt15>) -> R<Result<List<Adt15>, Adt102>> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v0, v1) = m276;
                let m274 = v0;
                match m274.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt102>>>(Ok(Ok::<List<Adt15>, Adt102>(f116(v1, List::<Adt15>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m275 = f117(v2)?;
                        match m275 {
                            Ok(v5) => {
                                Ok::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt102>>>((v3, f116(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt39>, List<Adt15>), R<Result<List<Adt15>, Adt102>>>(Ok(Err::<List<Adt15>, Adt102>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Str, v1: Str, v2: Adt8) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, None::<Adt8>, None::<Adt40>, None::<Adt8>)
}

pub fn f69() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::<(Str, Adt9)>::nil())), List::cons((Str::lit("required"), Adt9::C4(List::<Adt9>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f70() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("region"), f118(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f119()), List::cons((Str::lit("limit"), f118(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f118(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt9)>::nil())))))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("region")), List::cons(Adt9::C3(Str::lit("query")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))))
}

pub fn f71() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil())))
}

pub fn f72() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("$schema"), Adt9::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt9::C5(List::cons((Str::lit("address"), Adt9::C5(List::cons((Str::lit("$anchor"), Adt9::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("street"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("city"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("name"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("address"), Adt9::C5(List::cons((Str::lit("$ref"), Adt9::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("contactMethod"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("phone"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("email"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))))))), List::cons((Str::lit("allOf"), Adt9::C4(List::cons(Adt9::C5(List::cons((Str::lit("anyOf"), Adt9::C4(List::cons(Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())), List::cons(Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil())), List::<Adt9>::nil()))), List::cons((Str::lit("if"), Adt9::C5(List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("contactMethod"), Adt9::C5(List::cons((Str::lit("const"), Adt9::C3(Str::lit("phone"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("contactMethod")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("then"), Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("phone")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("else"), Adt9::C5(List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("email")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))))))
}

pub fn f73() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("name"), f119()), List::cons((Str::lit("label"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f120(1u64)?), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("count"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f120(0u64)?), List::cons((Str::lit("maximum"), f120(10u64)?), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("mode"), Adt9::C5(List::cons((Str::lit("enum"), Adt9::C4(List::cons(Adt9::C3(Str::lit("fast")), List::cons(Adt9::C3(Str::lit("slow")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("tags"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("maxItems"), f120(3u64)?), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("nested"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("flag"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("boolean"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))))))))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("name")), List::<Adt9>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f74() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("target"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("zone"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt9::C3(Str::lit("Zone"))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::<Adt9>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f75(v0: Str, v1: Str, v2: Adt8, v3: Adt8) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, Some(v3), None::<Adt40>, None::<Adt8>)
}

pub fn f76() -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("sum"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("label"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("sum")), List::cons(Adt9::C3(Str::lit("label")), List::<Adt9>::nil())))), List::<(Str, Adt9)>::nil()))))
}

pub fn f77(v0: Str, v1: Str) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))), None::<Adt8>, None::<Adt40>, None::<Adt8>)
}

pub fn f78(v0: Str, v1: Str, v2: Adt8) -> Adt39 {
    Adt39::C0(v0, None::<Str>, None::<List<Adt5>>, Some(v1), v2, None::<Adt8>, None::<Adt40>, None::<Adt8>)
}

pub fn f79(v0: Str) -> Adt8 {
    f121(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((v0.clone(), f122(Str::lit("string"))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(v0), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f80() -> Adt8 {
    f121(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("seconds"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), Adt9::C2(Adt10::C0(0i64, 0i64))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("label"), f122(Str::lit("string"))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("seconds")), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f81() -> Adt8 {
    f121(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil())))
}

pub fn f82(v0: Str, p284: List<Adt39>) -> Option<Adt39> {
    let m283 = p284;
    {
        let mut m283 = m283;
        loop {
            let step = {
                let v1 = m283;
                let m279 = v1;
                match m279.uncons() {
                    None => {
                        Err::<List<Adt39>, Option<Adt39>>(None::<Adt39>)
                    }
                    Some((v2, v3)) => {
                        let a281 = match v2.clone() {
                            Adt39::C0(h280, _, _, _, _, _, _, _) => {
                                h280
                            }
                        };
                        let a282 = v0.clone();
                        if equal(a281, a282) {
                            Err::<List<Adt39>, Option<Adt39>>(Some(v2))
                        } else {
                            Ok::<List<Adt39>, Option<Adt39>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m283 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Adt39, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    f123(v0.clone(), v2, Fn5::F124(v1, v0))
}

pub fn f84() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("key"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f120(1u64)?), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("key")), List::<Adt9>::nil()))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f85() -> R<Adt8> {
    Ok(Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((Str::lit("first"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f120(1u64)?), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("second"), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f120(1u64)?), List::<(Str, Adt9)>::nil())))), List::<(Str, Adt9)>::nil())))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(Str::lit("first")), List::cons(Adt9::C3(Str::lit("second")), List::<Adt9>::nil())))), List::cons((Str::lit("additionalProperties"), Adt9::C1(false)), List::<(Str, Adt9)>::nil()))))))
}

pub fn f86(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt93> {
    if f42(v1.clone(), Str::lit("effect_read")) {
        f125(v0, v1)
    } else {
        if f42(v1.clone(), Str::lit("effect_chain")) {
            f126(v0, v1)
        } else {
            if f42(v1, Str::lit("effect_edge_ok")) {
                f127(v0, 4294967295u64)
            } else {
                f127(v0, 4294967296u64)
            }
        }
    }
}

pub fn f87(v0: Str, v1: bool) -> Adt41 {
    Adt41::C0(List::cons(Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)), List::<Adt42>::nil()), None::<Adt9>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f88(v0: Adt102) -> Str {
    let m285 = v0;
    match m285 {
        Adt102::C0(v1) => {
            let a286 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a287 = Str::lit(" ");
            join(a286, a287)
        }
        Adt102::C1(v2) => {
            let a288 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a289 = Str::lit(" ");
            join(a288, a289)
        }
        Adt102::C2(v3) => {
            let a290 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a291 = Str::lit(" ");
            join(a290, a291)
        }
        Adt102::C3(v4) => {
            let a292 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a293 = Str::lit(" ");
            join(a292, a293)
        }
        Adt102::C4(v5, v6) => {
            let a294 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a295 = Str::lit(" ");
            join(a294, a295)
        }
        Adt102::C5(v7, v8) => {
            let a296 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a297 = Str::lit(" ");
            join(a296, a297)
        }
        Adt102::C6(v9, v10) => {
            let a298 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a299 = Str::lit(" ");
            join(a298, a299)
        }
        Adt102::C7(v11) => {
            let a300 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a301 = Str::lit(" ");
            join(a300, a301)
        }
        Adt102::C8(v12) => {
            let a302 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a303 = Str::lit(" ");
            join(a302, a303)
        }
        Adt102::C9(v13) => {
            let a304 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a305 = Str::lit(" ");
            join(a304, a305)
        }
        Adt102::C10(v14) => {
            let a306 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a307 = Str::lit(" ");
            join(a306, a307)
        }
        Adt102::C11(v15) => {
            let a308 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a309 = Str::lit(" ");
            join(a308, a309)
        }
        Adt102::C12(v16, v17) => {
            let a310 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a311 = Str::lit(" ");
            join(a310, a311)
        }
    }
}

pub fn f89(v0: Adt13, v1: List<Adt39>, v2: Adt24) -> Result<Adt36, Adt93> {
    let m313 = match v2 {
        Adt24::C0(_, h312) => {
            h312
        }
    };
    match m313 {
        None => {
            Ok::<Adt36, Adt93>(Adt36::C1(Adt38::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt93>(Adt93::C5(v3))
        }
    }
}

pub fn f90(v0: Str) -> bool {
    let a314 = v0.clone();
    let a315 = Str::lit("test_input_required_result_elicitation");
    if equal(a314, a315) {
        true
    } else {
        let a316 = v0.clone();
        let a317 = Str::lit("test_input_required_result_sampling");
        if equal(a316, a317) {
            true
        } else {
            let a318 = v0.clone();
            let a319 = Str::lit("test_input_required_result_list_roots");
            if equal(a318, a319) {
                true
            } else {
                let a320 = v0.clone();
                let a321 = Str::lit("test_input_required_result_request_state");
                if equal(a320, a321) {
                    true
                } else {
                    let a322 = v0.clone();
                    let a323 = Str::lit("test_input_required_result_multiple_inputs");
                    if equal(a322, a323) {
                        true
                    } else {
                        let a324 = v0.clone();
                        let a325 = Str::lit("test_input_required_result_multi_round");
                        if equal(a324, a325) {
                            true
                        } else {
                            let a326 = v0.clone();
                            let a327 = Str::lit("test_input_required_result_tampered_state");
                            if equal(a326, a327) {
                                true
                            } else {
                                let a328 = v0;
                                let a329 = Str::lit("test_input_required_result_capabilities");
                                equal(a328, a329)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f91(v0: List<Adt39>, v1: Fn4, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    let m331 = f82(match v2.clone() {
        Adt25::C0(_, h330, _, _, _) => {
            h330
        }
    }, v0);
    match m331 {
        None => {
            Ok(Err::<Adt36, Adt93>(Adt93::C0(match v2 {
                Adt25::C0(_, h332, _, _, _) => {
                    h332
                }
            })))
        }
        Some(v3) => {
            f123(v3.clone(), v2, Fn5::F128(v1, v3))
        }
    }
}

pub fn f92(v0: Adt25) -> R<Result<Adt105, Adt93>> {
    if f129(v0.clone(), Str::lit("test_input_required_result_elicitation")) {
        Ok(f130(v0))
    } else {
        if f129(v0.clone(), Str::lit("test_input_required_result_sampling")) {
            Ok(f131(v0))
        } else {
            if f129(v0.clone(), Str::lit("test_input_required_result_list_roots")) {
                f132(v0)
            } else {
                if f129(v0.clone(), Str::lit("test_input_required_result_request_state")) {
                    Ok(f133(v0))
                } else {
                    if f129(v0.clone(), Str::lit("test_input_required_result_multiple_inputs")) {
                        f134(v0)
                    } else {
                        if f129(v0.clone(), Str::lit("test_input_required_result_multi_round")) {
                            f135(v0)
                        } else {
                            if f129(v0.clone(), Str::lit("test_input_required_result_tampered_state")) {
                                Ok(f136(v0))
                            } else {
                                if f129(v0.clone(), Str::lit("test_input_required_result_capabilities")) {
                                    Ok(f137(v0))
                                } else {
                                    Ok(Err::<Adt105, Adt93>(Adt93::C9(Str::lit("not an input-required tool"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f93(v0: Adt25) -> R<Result<Adt53, Adt93>> {
    if f42(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt53, Adt93>(f138(List::cons(f102(Str::lit("This is a simple text response for testing.")), List::<Adt42>::nil()))))
    } else {
        if f42(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt53, Adt93>(f138(List::cons(f139(), List::<Adt42>::nil()))))
        } else {
            if f42(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt53, Adt93>(f138(List::cons(f140(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt42>::nil()))))
            } else {
                if f42(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt53, Adt93>(f138(List::cons(f141(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt42>::nil()))))
                } else {
                    if f42(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt53, Adt93>(f138(List::cons(f102(Str::lit("Multiple content types test:")), List::cons(f139(), List::cons(f141(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt42>::nil()))))))
                    } else {
                        if f42(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt53, Adt93>(f142(List::cons(f102(Str::lit("This tool intentionally returns an error for testing")), List::<Adt42>::nil()), None::<Adt9>, true, List::<Adt54>::nil())))
                        } else {
                            if f42(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt53, Adt93>(f143(v0, f144()?)))
                            } else {
                                if f42(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt53, Adt93>(f143(v0, f145()?)))
                                } else {
                                    if f42(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt53, Adt93>(f142(List::cons(f102(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt42>::nil()), Some(f146()?), false, List::<Adt54>::nil())))
                                    } else {
                                        if f42(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt53, Adt93>(f142(List::cons(f102(Str::lit("{\"sum\":\"three\"}")), List::<Adt42>::nil()), Some(f147()), false, List::<Adt54>::nil())))
                                        } else {
                                            if f42(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt53, Adt93>(f138(List::cons(f148(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt42>::nil()))))
                                            } else {
                                                if f42(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt53, Adt93>(f138(List::cons(f102(f149()), List::<Adt42>::nil()))))
                                                } else {
                                                    if f42(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt53, Adt93>(f138(List::cons(f102(Str::lit("Streaming complete")), List::<Adt42>::nil()))))
                                                    } else {
                                                        if f42(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt53, Adt93>(f138(List::cons(f102(Str::lit("Logging evaluated")), List::<Adt42>::nil()))))
                                                        } else {
                                                            Ok(f150(v0))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f94(v0: Adt24) -> Result<Adt36, Adt93> {
    f151(f152(), f153(), v0)
}

pub fn f95(v0: Adt26) -> R<Result<Adt36, Adt93>> {
    f154(f155(), Fn6::F156, v0)
}

pub fn f96(v0: Adt24) -> Result<Adt36, Adt93> {
    f157(f152(), f158(), v0)
}

pub fn f97(v0: Adt24) -> Result<Adt36, Adt93> {
    f159(f152(), f160(), v0)
}

pub fn f98(v0: Str) -> bool {
    let a333 = v0;
    let a334 = f161();
    equal(a333, a334)
}

pub fn f99(v0: Adt27) -> Result<Adt36, Adt93> {
    let m336 = f162(Str::lit("user_context"), Adt106::C0, match v0 {
        Adt27::C0(_, _, _, h335, _) => {
            h335
        }
    });
    match m336 {
        Adt108::C0 => {
            Ok::<Adt36, Adt93>(Adt36::C12(Adt85::C0(f165(Str::lit("user_context"), f166(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt108::C1 => {
            Err::<Adt36, Adt93>(Adt93::C11)
        }
        Adt108::C2(v1) => {
            Ok::<Adt36, Adt93>(Adt36::C8(f163(f164(v1, Str::lit("context"), Str::lit("unknown")))))
        }
    }
}

pub fn f100(v0: Adt27) -> Result<Adt36, Adt93> {
    f167(f160(), Fn7::F168, v0)
}

pub fn f101(v0: Adt28) -> R<Result<Adt36, Adt93>> {
    f169(Fn8::F170, v0)
}

pub fn f102(v0: Str) -> Adt42 {
    Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>))
}

pub fn f103(v0: Bytes, v1: Adt95, v2: Adt95, v3: Option<u64>) -> R<Result<Adt99, Adt2>> {
    let m337 = v3;
    match m337 {
        None => {
            Ok(Err::<Adt99, Adt2>(Adt2::C0(match v2 {
                Adt95::C0(_, h338) => {
                    h338
                }
            })))
        }
        Some(v4) => {
            let m342 = f171(v0.clone(), {
                let a340 = match v2.clone() {
                    Adt95::C0(_, h339) => {
                        h339
                    }
                };
                let a341 = 4u64;
                nat_add(a340, a341)?
            }, v4, List::<Adt100>::nil())?;
            match m342 {
                Ok(v6) => {
                    f172(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt99, Adt2>(v5))
                }
            }
        }
    }
}

pub fn f104(v0: List<Adt100>) -> List<u64> {
    f173(f174(v0, List::<Adt100>::nil()), List::<u64>::nil())
}

pub fn f105(v0: List<List<u64>>) -> List<u64> {
    f175(f176(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f106(v0: Option<List<u64>>) -> List<u64> {
    let m343 = v0;
    match m343 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a347 = f28(1u64);
            let a348 = {
                let a345 = f28({
                    let a344 = v1.clone();
                    length_list(a344)
                });
                let a346 = v1;
                append_list(a345, a346)
            };
            append_list(a347, a348)
        }
    }
}

pub fn f107() -> u64 {
    500u64
}

pub fn f108() -> List<Adt100> {
    List::cons(Adt100::C0(f177(), f178()), List::<Adt100>::nil())
}

pub fn f109() -> List<u64> {
    let a351 = f179();
    let a352 = {
        let a349 = f180();
        let a350 = f181();
        append_list(a349, a350)
    };
    append_list(a351, a352)
}

pub fn f110(v0: Adt3, v1: Fn1, v2: Adt99, v3: Option<Adt107>) -> R<Adt104> {
    let m353 = v3;
    match m353 {
        None => {
            f183(v0, v1, v2)
        }
        Some(v4) => {
            f182(v4)
        }
    }
}

pub fn f111(v0: List<Adt100>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m354 = f184(v0.clone(), v1.clone())?;
    match m354 {
        None => {
            f185(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f112(p357: List<Adt1>, p358: List<Adt1>) -> List<Adt1> {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v0, v1) = m356;
                let m355 = v0;
                match m355.uncons() {
                    None => {
                        Err::<(List<Adt1>, List<Adt1>), List<Adt1>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt1>, List<Adt1>), List<Adt1>>((v3, List::cons(v2, v1)))
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

pub fn f113(v0: bool, v1: Bytes, v2: List<Adt1>, v3: u64) -> Result<Adt0, Adt2> {
    if v0 {
        Ok::<Adt0, Adt2>(Adt0::C0(v1, v2))
    } else {
        Err::<Adt0, Adt2>(Adt2::C1(v3))
    }
}

pub fn f114(v0: Adt3, v1: Fn3, v2: Adt99, v3: Option<Adt107>) -> R<Adt96> {
    let m359 = v3;
    match m359 {
        None => {
            f186(v0, v1, v2)
        }
        Some(v4) => {
            Ok(Adt96::C0(f56(f182(v4)?)))
        }
    }
}

pub fn f115(p365: List<Adt39>, p366: List<Str>) -> Option<Str> {
    let m364 = (p365, p366);
    {
        let mut m364 = m364;
        loop {
            let step = {
                let (v0, v1) = m364;
                let m360 = v0;
                match m360.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f187(match v2.clone() {
                            Adt39::C0(h361, _, _, _, _, _, _, _) => {
                                h361
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt39>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt39::C0(h362, _, _, _, _, _, _, _) => {
                                    h362
                                }
                            }))
                        } else {
                            Ok::<(List<Adt39>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt39::C0(h363, _, _, _, _, _, _, _) => {
                                    h363
                                }
                            }, v1)))
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

pub fn f116(p369: List<Adt15>, p370: List<Adt15>) -> List<Adt15> {
    let m368 = (p369, p370);
    {
        let mut m368 = m368;
        loop {
            let step = {
                let (v0, v1) = m368;
                let m367 = v0;
                match m367.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
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

pub fn f117(v0: Adt39) -> R<Result<List<Adt15>, Adt102>> {
    let m373 = f188(match v0.clone() {
        Adt39::C0(h371, _, _, _, _, _, _, _) => {
            h371
        }
    }, f189(match v0.clone() {
        Adt39::C0(h372, _, _, _, _, _, _, _) => {
            h372
        }
    })?);
    match m373 {
        None => {
            f190(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt102>(v1))
        }
    }
}

pub fn f118(v0: Str, v1: Str) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())))
}

pub fn f119() -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("string"))), List::<(Str, Adt9)>::nil()))
}

pub fn f120(v0: u64) -> R<Adt9> {
    Ok(Adt9::C2(Adt10::C0(f191(v0, 0i64)?, 0i64)))
}

pub fn f121(v0: List<(Str, Adt9)>) -> Adt8 {
    Adt8::C0(v0)
}

pub fn f122(v0: Str) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::<(Str, Adt9)>::nil()))
}

pub fn f123(v0: Adt39, v1: Adt25, v2: Fn5) -> R<Result<Adt36, Adt93>> {
    let m380 = f192(Adt9::C5({
        let m376 = match v0.clone() {
            Adt39::C0(_, _, _, _, h375, _, _, _) => {
                h375
            }
        };
        match m376 {
            Adt8::C0(h374) => {
                h374
            }
        }
    }), Adt9::C5({
        let m379 = match v0 {
            Adt39::C0(_, _, _, _, h378, _, _, _) => {
                h378
            }
        };
        match m379 {
            Adt8::C0(h377) => {
                h377
            }
        }
    }), f193(v1.clone()))?;
    match m380 {
        Adt111::C0 => {
            let c381: Fn5 = v2;
            c381.apply(v1)
        }
        Adt111::C1(v3, v4) => {
            Ok(Ok::<Adt36, Adt93>(Adt36::C2(f87(f194(v3, v4)?, true))))
        }
        Adt111::C2(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt111::C3(_) => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt111::C4 => {
            Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f124(v0: Fn2, v1: Adt39, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    f195(v1, v0, v2)
}

pub fn f125(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt93> {
    let m382 = v0;
    match m382.uncons() {
        None => {
            f196(1u64, f197(Str::lit("key"), v1))
        }
        Some((v2, v3)) => {
            let m383 = v3;
            match m383.uncons() {
                None => {
                    let a385 = match v2.clone() {
                        Adt1::C0(h384, _) => {
                            h384
                        }
                    };
                    let a386 = 1u64;
                    if nat_eq(a385, a386) {
                        f198(f47(Str::lit("read:"), f48(match v2 {
                            Adt1::C0(_, h387) => {
                                h387
                            }
                        })))
                    } else {
                        f199()
                    }
                }
                Some((_, _)) => {
                    f199()
                }
            }
        }
    }
}

pub fn f126(v0: List<Adt1>, v1: Adt25) -> Result<Adt53, Adt93> {
    let m388 = v0;
    match m388.uncons() {
        None => {
            f196(1u64, f197(Str::lit("first"), v1))
        }
        Some((v2, v3)) => {
            let m389 = v3;
            match m389.uncons() {
                None => {
                    let a391 = match v2.clone() {
                        Adt1::C0(h390, _) => {
                            h390
                        }
                    };
                    let a392 = 1u64;
                    if nat_eq(a391, a392) {
                        f196(2u64, f47(f48(match v2 {
                            Adt1::C0(_, h393) => {
                                h393
                            }
                        }), f197(Str::lit("second"), v1)))
                    } else {
                        f199()
                    }
                }
                Some((v4, v5)) => {
                    let m394 = v5;
                    match m394.uncons() {
                        None => {
                            let a396 = match v2.clone() {
                                Adt1::C0(h395, _) => {
                                    h395
                                }
                            };
                            let a397 = 1u64;
                            if nat_eq(a396, a397) {
                                let a399 = match v4.clone() {
                                    Adt1::C0(h398, _) => {
                                        h398
                                    }
                                };
                                let a400 = 2u64;
                                if nat_eq(a399, a400) {
                                    f198(f47(f47(Str::lit("chain:"), f48(match v2 {
                                        Adt1::C0(_, h401) => {
                                            h401
                                        }
                                    })), f47(Str::lit("|"), f48(match v4 {
                                        Adt1::C0(_, h402) => {
                                            h402
                                        }
                                    }))))
                                } else {
                                    f199()
                                }
                            } else {
                                f199()
                            }
                        }
                        Some((_, _)) => {
                            f199()
                        }
                    }
                }
            }
        }
    }
}

pub fn f127(v0: List<Adt1>, v1: u64) -> Result<Adt53, Adt93> {
    let m403 = v0;
    match m403.uncons() {
        None => {
            f196(v1, Str::lit("edge"))
        }
        Some((_, _)) => {
            f199()
        }
    }
}

pub fn f128(v0: Fn4, v1: Adt39, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    f200(v1, {
        let c404: Fn4 = v0;
        c404.apply(v2)?
    })
}

pub fn f129(v0: Adt25, v1: Str) -> bool {
    let a406 = match v0 {
        Adt25::C0(_, h405, _, _, _) => {
            h405
        }
    };
    let a407 = v1;
    equal(a406, a407)
}

pub fn f130(v0: Adt25) -> Result<Adt105, Adt93> {
    let m409 = f162(Str::lit("user_name"), Adt106::C0, match v0 {
        Adt25::C0(_, _, _, h408, _) => {
            h408
        }
    });
    match m409 {
        Adt108::C0 => {
            f203(f165(Str::lit("user_name"), f204()))
        }
        Adt108::C1 => {
            f205()
        }
        Adt108::C2(v1) => {
            f201(f202(List::cons(Str::lit("Hello, "), List::cons(f164(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f131(v0: Adt25) -> Result<Adt105, Adt93> {
    let m411 = f162(Str::lit("capital_question"), Adt106::C1, match v0 {
        Adt25::C0(_, _, _, h410, _) => {
            h410
        }
    });
    match m411 {
        Adt108::C0 => {
            f203(f165(Str::lit("capital_question"), f206(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt108::C1 => {
            f205()
        }
        Adt108::C2(v1) => {
            f201(f202(List::cons(Str::lit("Sampling result: "), List::cons(f164(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f132(v0: Adt25) -> R<Result<Adt105, Adt93>> {
    let m413 = f162(Str::lit("client_roots"), Adt106::C2, match v0 {
        Adt25::C0(_, _, _, h412, _) => {
            h412
        }
    });
    match m413 {
        Adt108::C0 => {
            Ok(f203(f165(Str::lit("client_roots"), f209())))
        }
        Adt108::C1 => {
            Ok(f205())
        }
        Adt108::C2(v1) => {
            Ok(f201(f202(List::cons(Str::lit("Found "), List::cons(f207(f208(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f133(v0: Adt25) -> Result<Adt105, Adt93> {
    f210(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f134(v0: Adt25) -> R<Result<Adt105, Adt93>> {
    if f211(v0.clone(), Str::lit("multiple-inputs")) {
        f212(f162(Str::lit("user_name"), Adt106::C0, match v0.clone() {
            Adt25::C0(_, _, _, h414, _) => {
                h414
            }
        }), f162(Str::lit("greeting"), Adt106::C1, match v0.clone() {
            Adt25::C0(_, _, _, h415, _) => {
                h415
            }
        }), f162(Str::lit("client_roots"), Adt106::C2, match v0 {
            Adt25::C0(_, _, _, h416, _) => {
                h416
            }
        }))
    } else {
        Ok(f213(f214(Adt108::C0, Adt108::C0, Adt108::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f135(v0: Adt25) -> R<Result<Adt105, Adt93>> {
    let v1: u64 = f215(f216(v0.clone()))?;
    let a417 = v1;
    let a418 = 49u64;
    if nat_eq(a417, a418) {
        Ok(f217(v0))
    } else {
        let a419 = v1;
        let a420 = 50u64;
        if nat_eq(a419, a420) {
            Ok(f218(v0))
        } else {
            Ok(f213(f165(Str::lit("step1"), f219()), Str::lit("1")))
        }
    }
}

pub fn f136(v0: Adt25) -> Result<Adt105, Adt93> {
    f210(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f137(v0: Adt25) -> Result<Adt105, Adt93> {
    let m422 = match v0.clone() {
        Adt25::C0(_, _, _, h421, _) => {
            h421
        }
    };
    match m422 {
        None => {
            f220(f221({
                let m425 = match v0 {
                    Adt25::C0(h424, _, _, _, _) => {
                        h424
                    }
                };
                match m425 {
                    Adt20::C0(_, h423, _, _, _, _) => {
                        h423
                    }
                }
            }))
        }
        Some(v1) => {
            let m427 = match v1.clone() {
                Adt8::C0(h426) => {
                    h426
                }
            };
            match m427.uncons() {
                None => {
                    f220(f221({
                        let m430 = match v0 {
                            Adt25::C0(h429, _, _, _, _) => {
                                h429
                            }
                        };
                        match m430 {
                            Adt20::C0(_, h428, _, _, _, _) => {
                                h428
                            }
                        }
                    }))
                }
                Some((_, _)) => {
                    f201(f202(List::cons(Str::lit("capabilities-ok: received "), List::cons({
                        let a432 = f222(match v1 {
                            Adt8::C0(h431) => {
                                h431
                            }
                        });
                        let a433 = Str::lit(",");
                        join(a432, a433)
                    }, List::<Str>::nil()))))
                }
            }
        }
    }
}

pub fn f138(v0: List<Adt42>) -> Adt53 {
    f142(v0, None::<Adt9>, false, List::<Adt54>::nil())
}

pub fn f139() -> Adt42 {
    f148(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f140(v0: Str, v1: Str) -> Adt42 {
    Adt42::C2(Adt47::C0(v0, v1, None::<Adt44>, None::<Adt8>))
}

pub fn f141(v0: Str, v1: Str, v2: Str) -> Adt42 {
    Adt42::C4(Adt49::C0(Adt50::C0(Adt51::C0(v0, Some(v1), v2, None::<Adt8>)), None::<Adt44>, None::<Adt8>))
}

pub fn f142(v0: List<Adt42>, v1: Option<Adt9>, v2: bool, v3: List<Adt54>) -> Adt53 {
    Adt53::C0(Adt41::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f143(v0: Adt25, v1: List<Adt54>) -> Adt53 {
    f142(List::cons(f102(f223({
        let m436 = match v0 {
            Adt25::C0(h435, _, _, _, _) => {
                h435
            }
        };
        match m436 {
            Adt20::C0(_, _, _, _, h434, _) => {
                h434
            }
        }
    })), List::<Adt42>::nil()), None::<Adt9>, false, v1)
}

pub fn f144() -> R<List<Adt54>> {
    Ok(List::cons(f224(0u64, 100u64)?, List::cons(f224(50u64, 100u64)?, List::cons(f224(100u64, 100u64)?, List::<Adt54>::nil()))))
}

pub fn f145() -> R<List<Adt54>> {
    Ok(List::cons(f224(50u64, 100u64)?, List::cons(f224(10u64, 100u64)?, List::<Adt54>::nil())))
}

pub fn f146() -> R<Adt9> {
    Ok(Adt9::C5(List::cons((Str::lit("sum"), f120(3u64)?), List::cons((Str::lit("label"), Adt9::C3(Str::lit("three"))), List::<(Str, Adt9)>::nil()))))
}

pub fn f147() -> Adt9 {
    Adt9::C5(List::cons((Str::lit("sum"), Adt9::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt9::C3(Str::lit("three"))), List::<(Str, Adt9)>::nil())))
}

pub fn f148(v0: Str, v1: Str) -> Adt42 {
    Adt42::C1(Adt46::C0(v0, v1, None::<Adt44>, None::<Adt8>))
}

pub fn f149() -> Str {
    let m438 = {
        let a437 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a437)
    };
    match m438 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f150(v0: Adt25) -> Result<Adt53, Adt93> {
    if f42(v0.clone(), Str::lit("test_missing_capability")) {
        let m444 = {
            let m443 = {
                let m442 = match v0 {
                    Adt25::C0(h441, _, _, _, _) => {
                        h441
                    }
                };
                match m442 {
                    Adt20::C0(_, h440, _, _, _, _) => {
                        h440
                    }
                }
            };
            match m443 {
                Adt21::C0(_, _, h439, _, _) => {
                    h439
                }
            }
        };
        match m444 {
            None => {
                Err::<Adt53, Adt93>(Adt93::C8(f225()))
            }
            Some(_) => {
                Ok::<Adt53, Adt93>(f226())
            }
        }
    } else {
        if f42(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt53, Adt93>(f138(List::cons(f102(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt42>::nil())))
        } else {
            if f42(v0, Str::lit("test_arguments")) {
                Ok::<Adt53, Adt93>(f138(List::cons(f102(Str::lit("arguments accepted")), List::<Adt42>::nil())))
            } else {
                Ok::<Adt53, Adt93>(f226())
            }
        }
    }
}

pub fn f151(v0: Adt13, v1: List<Adt48>, v2: Adt24) -> Result<Adt36, Adt93> {
    let m446 = match v2 {
        Adt24::C0(_, h445) => {
            h445
        }
    };
    match m446 {
        None => {
            Ok::<Adt36, Adt93>(Adt36::C4(Adt55::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt93>(Adt93::C5(v3))
        }
    }
}

pub fn f152() -> Adt13 {
    Adt13::C0(300000u64, Adt14::C0)
}

pub fn f153() -> List<Adt48> {
    f227(f228())
}

pub fn f154(v0: Adt13, v1: Fn6, v2: Adt26) -> R<Result<Adt36, Adt93>> {
    Ok(f229(v0, match v2.clone() {
        Adt26::C0(_, h447, _, _) => {
            h447
        }
    }, {
        let c448: Fn6 = v1;
        c448.apply(match v2 {
            Adt26::C0(_, h449, _, _) => {
                h449
            }
        })?
    }))
}

pub fn f155() -> Adt13 {
    Adt13::C0(300000u64, Adt14::C1)
}

pub fn f156(v0: Str) -> R<Option<List<Adt50>>> {
    let m450 = f230(v0.clone(), f228());
    match m450 {
        None => {
            f231(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f157(v0: Adt13, v1: List<Adt58>, v2: Adt24) -> Result<Adt36, Adt93> {
    let m452 = match v2 {
        Adt24::C0(_, h451) => {
            h451
        }
    };
    match m452 {
        None => {
            Ok::<Adt36, Adt93>(Adt36::C6(Adt57::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt93>(Adt93::C5(v3))
        }
    }
}

pub fn f158() -> List<Adt58> {
    List::cons(Adt58::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt5>>, f232(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt44>, None::<Adt8>), List::<Adt58>::nil())
}

pub fn f159(v0: Adt13, v1: List<Adt60>, v2: Adt24) -> Result<Adt36, Adt93> {
    let m454 = match v2 {
        Adt24::C0(_, h453) => {
            h453
        }
    };
    match m454 {
        None => {
            Ok::<Adt36, Adt93>(Adt36::C7(Adt59::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt36, Adt93>(Adt93::C5(v3))
        }
    }
}

pub fn f160() -> List<Adt60> {
    List::cons(f233(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt61>>), List::cons(f233(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f234(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f234(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt61>::nil())))), List::cons(f233(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f234(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt61>::nil()))), List::cons(f233(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt61>>), List::<Adt60>::nil()))))
}

pub fn f161() -> Str {
    Str::lit("test_input_required_result_prompt")
}

pub fn f162(v0: Str, v1: Adt106, v2: Option<Adt8>) -> Adt108 {
    let m455 = v2;
    match m455 {
        None => {
            Adt108::C0
        }
        Some(v3) => {
            let m456 = f235(v0, v3);
            match m456 {
                Adt116::C0 => {
                    Adt108::C0
                }
                Adt116::C1(v4) => {
                    let m457 = f236(v4);
                    match m457 {
                        None => {
                            Adt108::C1
                        }
                        Some(v5) => {
                            if f237(v1, v5.clone()) {
                                Adt108::C2(v5)
                            } else {
                                Adt108::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f163(v0: Str) -> Adt62 {
    Adt62::C0(None::<Str>, List::cons(Adt63::C0(Adt45::C0, Adt42::C0(Adt43::C0(f202(List::cons(Str::lit("Prompt with context: "), List::cons(v0, List::<Str>::nil()))), None::<Adt44>, None::<Adt8>))), List::<Adt63>::nil()))
}

pub fn f164(v0: Adt8, v1: Str, v2: Str) -> Str {
    let m458 = f235(Str::lit("content"), v0);
    match m458 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            let m459 = f236(v3);
            match m459 {
                None => {
                    v2
                }
                Some(v4) => {
                    f238(v1, v4, v2)
                }
            }
        }
    }
}

pub fn f165(v0: Str, v1: Adt68) -> List<(Str, Adt68)> {
    List::cons((v0, v1), List::<(Str, Adt68)>::nil())
}

pub fn f166(v0: Str, v1: Str, v2: Str) -> Adt68 {
    Adt68::C0(Adt69::C0(Adt70::C0(None::<Adt71>, v0, f239(v1, v2))))
}

pub fn f167(v0: List<Adt60>, v1: Fn7, v2: Adt27) -> Result<Adt36, Adt93> {
    let m461 = f240(match v2.clone() {
        Adt27::C0(_, h460, _, _, _) => {
            h460
        }
    }, v0);
    match m461 {
        None => {
            Err::<Adt36, Adt93>(Adt93::C1(match v2 {
                Adt27::C0(_, h462, _, _, _) => {
                    h462
                }
            }))
        }
        Some(v3) => {
            let m464 = f241(f242(v3), f243(match v2.clone() {
                Adt27::C0(_, _, h463, _, _) => {
                    h463
                }
            }));
            match m464 {
                None => {
                    Ok::<Adt36, Adt93>(Adt36::C8({
                        let c467: Fn7 = v1;
                        c467.apply(match v2.clone() {
                            Adt27::C0(_, h468, _, _, _) => {
                                h468
                            }
                        }, f243(match v2 {
                            Adt27::C0(_, _, h469, _, _) => {
                                h469
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt36, Adt93>(Adt93::C4({
                        let a465 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a466 = Str::lit(" ");
                        join(a465, a466)
                    }))
                }
            }
        }
    }
}

pub fn f168(v0: Str, v1: List<(Str, Str)>) -> Adt62 {
    let a470 = v0.clone();
    let a471 = Str::lit("test_simple_prompt");
    if equal(a470, a471) {
        f244(List::cons(f245(Str::lit("This is a simple prompt for testing.")), List::<Adt63>::nil()))
    } else {
        let a472 = v0.clone();
        let a473 = Str::lit("test_prompt_with_arguments");
        if equal(a472, a473) {
            f244(List::cons(f245({
                let a474 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f246(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f246(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a475 = Str::lit("");
                join(a474, a475)
            }), List::<Adt63>::nil()))
        } else {
            let a476 = v0.clone();
            let a477 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a476, a477) {
                f244(List::cons(Adt63::C0(Adt45::C0, Adt42::C4(Adt49::C0(f247(f246(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt44>, None::<Adt8>))), List::cons(f245(Str::lit("Please process the embedded resource above.")), List::<Adt63>::nil())))
            } else {
                let a478 = v0;
                let a479 = Str::lit("test_prompt_with_image");
                if equal(a478, a479) {
                    f244(List::cons(Adt63::C0(Adt45::C0, Adt42::C1(Adt46::C0(f248(), Str::lit("image/png"), None::<Adt44>, None::<Adt8>))), List::cons(f245(Str::lit("Please analyze the image above.")), List::<Adt63>::nil())))
                } else {
                    f244(List::<Adt63>::nil())
                }
            }
        }
    }
}

pub fn f169(v0: Fn8, v1: Adt28) -> R<Result<Adt36, Adt93>> {
    let m481 = {
        let c480: Fn8 = v0;
        c480.apply(v1)?
    };
    match m481 {
        Ok(v3) => {
            Ok(Ok::<Adt36, Adt93>(Adt36::C9(f249(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt36, Adt93>(v2))
        }
    }
}

pub fn f170(v0: Adt28) -> R<Result<List<Str>, Adt93>> {
    let m483 = match v0.clone() {
        Adt28::C0(_, h482, _, _) => {
            h482
        }
    };
    match m483 {
        Adt29::C0(v1) => {
            let m485 = f240(match v1.clone() {
                Adt30::C0(h484, _) => {
                    h484
                }
            }, f160());
            match m485 {
                None => {
                    Ok(Err::<List<Str>, Adt93>(Adt93::C1(match v1 {
                        Adt30::C0(h486, _) => {
                            h486
                        }
                    })))
                }
                Some(v2) => {
                    if f250(f242(v2), {
                        let m489 = match v0.clone() {
                            Adt28::C0(_, _, h488, _) => {
                                h488
                            }
                        };
                        match m489 {
                            Adt31::C0(h487, _) => {
                                h487
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt93>(f251({
                            let m492 = match v0.clone() {
                                Adt28::C0(_, _, h491, _) => {
                                    h491
                                }
                            };
                            match m492 {
                                Adt31::C0(_, h490) => {
                                    h490
                                }
                            }
                        }, f252(match v1 {
                            Adt30::C0(h493, _) => {
                                h493
                            }
                        }, {
                            let m496 = match v0 {
                                Adt28::C0(_, _, h495, _) => {
                                    h495
                                }
                            };
                            match m496 {
                                Adt31::C0(h494, _) => {
                                    h494
                                }
                            }
                        }))?))
                    } else {
                        Ok(Err::<List<Str>, Adt93>(Adt93::C4(Str::lit("the prompt declares no such argument"))))
                    }
                }
            }
        }
        Adt29::C1(v3) => {
            let a497 = v3.clone();
            let a498 = f232();
            if equal(a497, a498) {
                let a502 = {
                    let m501 = match v0.clone() {
                        Adt28::C0(_, _, h500, _) => {
                            h500
                        }
                    };
                    match m501 {
                        Adt31::C0(h499, _) => {
                            h499
                        }
                    }
                };
                let a503 = Str::lit("id");
                if equal(a502, a503) {
                    Ok(Ok::<List<Str>, Adt93>(f251({
                        let m506 = match v0 {
                            Adt28::C0(_, _, h505, _) => {
                                h505
                            }
                        };
                        match m506 {
                            Adt31::C0(_, h504) => {
                                h504
                            }
                        }
                    }, f253())?))
                } else {
                    Ok(Err::<List<Str>, Adt93>(Adt93::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt93>(Adt93::C2(v3)))
            }
        }
    }
}

pub fn f171(v0: Bytes, p516: u64, p517: u64, p518: List<Adt100>) -> R<Result<(List<Adt100>, u64), Adt2>> {
    let m515 = (p516, (p517, p518));
    {
        let mut m515 = m515;
        loop {
            let step = {
                let (v1, (v2, v3)) = m515;
                let m507 = v2;
                if m507 == 0 {
                    Err::<(u64, (u64, List<Adt100>)), R<Result<(List<Adt100>, u64), Adt2>>>(Ok(Ok::<(List<Adt100>, u64), Adt2>((f254(v3, List::<Adt100>::nil()), v1))))
                } else {
                    let v4 = m507 - 1;
                    let m508 = f7(v0.clone(), v1)?;
                    match m508 {
                        None => {
                            Err::<(u64, (u64, List<Adt100>)), R<Result<(List<Adt100>, u64), Adt2>>>(Ok(Err::<(List<Adt100>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            let m510 = f7(v0.clone(), match v5.clone() {
                                Adt95::C0(_, h509) => {
                                    h509
                                }
                            })?;
                            match m510 {
                                None => {
                                    Err::<(u64, (u64, List<Adt100>)), R<Result<(List<Adt100>, u64), Adt2>>>(Ok(Err::<(List<Adt100>, u64), Adt2>(Adt2::C0(match v5 {
                                        Adt95::C0(_, h511) => {
                                            h511
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt100>)), R<Result<(List<Adt100>, u64), Adt2>>>((match v6.clone() {
                                        Adt95::C0(_, h512) => {
                                            h512
                                        }
                                    }, (v4, List::cons(Adt100::C0(f29(match v5 {
                                        Adt95::C0(h513, _) => {
                                            h513
                                        }
                                    })?, f29(match v6 {
                                        Adt95::C0(h514, _) => {
                                            h514
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: Bytes, v1: Adt95, v2: Adt95, v3: (List<Adt100>, u64)) -> R<Result<Adt99, Adt2>> {
    let m520 = f7(v0.clone(), {
        let (_, h519) = v3.clone();
        h519
    })?;
    match m520 {
        None => {
            Ok(Err::<Adt99, Adt2>(Adt2::C0({
                let (_, h521) = v3;
                h521
            })))
        }
        Some(v4) => {
            let m523 = f27(v0.clone(), match v4.clone() {
                Adt95::C0(_, h522) => {
                    h522
                }
            })?;
            match m523 {
                None => {
                    Ok(Err::<Adt99, Adt2>(Adt2::C0(match v4 {
                        Adt95::C0(_, h524) => {
                            h524
                        }
                    })))
                }
                Some(v5) => {
                    let m528 = f255(v0.clone(), {
                        let a526 = match v4.clone() {
                            Adt95::C0(_, h525) => {
                                h525
                            }
                        };
                        let a527 = 4u64;
                        nat_add(a526, a527)?
                    })?;
                    match m528 {
                        None => {
                            Ok(Err::<Adt99, Adt2>(Adt2::C0({
                                let a530 = match v4 {
                                    Adt95::C0(_, h529) => {
                                        h529
                                    }
                                };
                                let a531 = 4u64;
                                nat_add(a530, a531)?
                            })))
                        }
                        Some(v6) => {
                            f256(v0, Adt99::C0(f29(match v1 {
                                Adt95::C0(h532, _) => {
                                    h532
                                }
                            })?, f29(match v2 {
                                Adt95::C0(h533, _) => {
                                    h533
                                }
                            })?, {
                                let (h534, _) = v3;
                                h534
                            }, match v4 {
                                Adt95::C0(h535, _) => {
                                    h535
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt95::C0(h536, _) => {
                                    h536
                                }
                            }, Adt101::C0(0i64, Str::lit(""), Str::lit(""), f257())), match v6 {
                                Adt95::C0(_, h537) => {
                                    h537
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f173(p542: List<Adt100>, p543: List<u64>) -> List<u64> {
    let m541 = (p542, p543);
    {
        let mut m541 = m541;
        loop {
            let step = {
                let (v0, v1) = m541;
                let m538 = v0;
                match m538.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<u64>), List<u64>>((v3, {
                            let a539 = f258(v2);
                            let a540 = v1;
                            append_list(a539, a540)
                        }))
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

pub fn f174(p546: List<Adt100>, p547: List<Adt100>) -> List<Adt100> {
    let m545 = (p546, p547);
    {
        let mut m545 = m545;
        loop {
            let step = {
                let (v0, v1) = m545;
                let m544 = v0;
                match m544.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<Adt100>), List<Adt100>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<Adt100>), List<Adt100>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(p552: List<List<u64>>, p553: List<u64>) -> List<u64> {
    let m551 = (p552, p553);
    {
        let mut m551 = m551;
        loop {
            let step = {
                let (v0, v1) = m551;
                let m548 = v0;
                match m548.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a549 = f259(v2);
                            let a550 = v1;
                            append_list(a549, a550)
                        }))
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

pub fn f176(p556: List<List<u64>>, p557: List<List<u64>>) -> List<List<u64>> {
    let m555 = (p556, p557);
    {
        let mut m555 = m555;
        loop {
            let step = {
                let (v0, v1) = m555;
                let m554 = v0;
                match m554.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
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

pub fn f177() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f178() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f179() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f180() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f181() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f182(v0: Adt107) -> R<Adt104> {
    f260(f261(v0.clone()), f262(None::<Adt109>, f263(v0)))
}

pub fn f183(v0: Adt3, v1: Fn1, v2: Adt99) -> R<Adt104> {
    if f264(match v2.clone() {
        Adt99::C0(h558, _, _, _, _, _, _, _) => {
            h558
        }
    }) {
        f265(match v2.clone() {
            Adt99::C0(_, _, _, _, _, _, _, h559) => {
                h559
            }
        }, f266(v0, match v2.clone() {
            Adt99::C0(_, _, _, _, _, _, h560, _) => {
                h560
            }
        }), Fn3::F267(v1, v2.clone()), match v2.clone() {
            Adt99::C0(_, _, _, _, h561, _, _, _) => {
                h561
            }
        }, match v2.clone() {
            Adt99::C0(_, _, h562, _, _, _, _, _) => {
                h562
            }
        }, f268(match v2 {
            Adt99::C0(_, _, _, h563, _, _, _, _) => {
                h563
            }
        })?)
    } else {
        Ok(f269())
    }
}

pub fn f184(v0: List<Adt100>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m564 = f270(v0, f271())?;
    match m564 {
        Adt120::C0 => {
            Ok(None::<Adt107>)
        }
        Adt120::C1 => {
            Ok(Some(Adt107::C0))
        }
        Adt120::C2 => {
            Ok(Some(Adt107::C0))
        }
        Adt120::C3(v2) => {
            f272(f273(v2), v1, Adt107::C0)
        }
    }
}

pub fn f185(v0: List<Adt100>, v1: List<List<u64>>) -> R<Option<Adt107>> {
    let m565 = f270(v0, f274())?;
    match m565 {
        Adt120::C0 => {
            Ok(None::<Adt107>)
        }
        Adt120::C1 => {
            Ok(Some(Adt107::C1))
        }
        Adt120::C2 => {
            Ok(Some(Adt107::C1))
        }
        Adt120::C3(v2) => {
            f272(f275(v2), v1, Adt107::C1)
        }
    }
}

pub fn f186(v0: Adt3, v1: Fn3, v2: Adt99) -> R<Adt96> {
    if f264(match v2.clone() {
        Adt99::C0(h566, _, _, _, _, _, _, _) => {
            h566
        }
    }) {
        f276(f266(v0, match v2.clone() {
            Adt99::C0(_, _, _, _, _, _, h567, _) => {
                h567
            }
        }), v1, match v2.clone() {
            Adt99::C0(_, _, _, _, h568, _, _, _) => {
                h568
            }
        }, match v2.clone() {
            Adt99::C0(_, _, h569, _, _, _, _, _) => {
                h569
            }
        }, f268(match v2 {
            Adt99::C0(_, _, _, h570, _, _, _, _) => {
                h570
            }
        })?)
    } else {
        Ok(Adt96::C0(f56(f269())))
    }
}

pub fn f187(v0: Str, p575: List<Str>) -> bool {
    let m574 = p575;
    {
        let mut m574 = m574;
        loop {
            let step = {
                let v1 = m574;
                let m571 = v1;
                match m571.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a572 = v0.clone();
                        let a573 = v2;
                        if equal(a572, a573) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Str, v1: List<u64>) -> Option<Adt102> {
    if f277(v1) {
        Some(Adt102::C0(v0))
    } else {
        None::<Adt102>
    }
}

pub fn f189(v0: Str) -> R<List<u64>> {
    f29({
        let a576 = v0;
        utf8_encode(a576)
    })
}

pub fn f190(v0: Adt39) -> R<Result<List<Adt15>, Adt102>> {
    let m577 = f278(v0.clone());
    match m577 {
        None => {
            f279(v0.clone(), f280(f281(), match v0.clone() {
                Adt39::C0(h578, _, _, _, _, _, _, _) => {
                    h578
                }
            }, Adt9::C5({
                let m581 = match v0.clone() {
                    Adt39::C0(_, _, _, _, h580, _, _, _) => {
                        h580
                    }
                };
                match m581 {
                    Adt8::C0(h579) => {
                        h579
                    }
                }
            }), List::cons(Adt110::C0(Adt9::C5({
                let m584 = match v0 {
                    Adt39::C0(_, _, _, _, h583, _, _, _) => {
                        h583
                    }
                };
                match m584 {
                    Adt8::C0(h582) => {
                        h582
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt110>::nil()), f282())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt15>, Adt102>(v1))
        }
    }
}

pub fn f191(p589: u64, p590: i64) -> R<i64> {
    let m588 = (p589, p590);
    {
        let mut m588 = m588;
        loop {
            let step = {
                let (v0, v1) = m588;
                let m585 = v0;
                if m585 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m585 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a586 = v1;
                        let a587 = -1i64;
                        int_sub(a586, a587)?
                    }))
                }
            };
            match step {
                Ok(next) => { m588 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Adt9, v1: Adt9, v2: Adt9) -> R<Adt111> {
    f283(v0, f284(), Adt113::C0, List::cons(Adt114::C0(v1, v2, List::<Adt112>::nil()), List::<Adt114>::nil()))
}

pub fn f193(v0: Adt25) -> Adt9 {
    let m592 = match v0 {
        Adt25::C0(_, _, h591, _, _) => {
            h591
        }
    };
    match m592 {
        None => {
            Adt9::C5(List::<(Str, Adt9)>::nil())
        }
        Some(v1) => {
            Adt9::C5(match v1 {
                Adt8::C0(h593) => {
                    h593
                }
            })
        }
    }
}

pub fn f194(v0: List<Adt112>, v1: Str) -> R<Str> {
    let v2: Str = f285(v0)?;
    let a596 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a594 = v2.clone();
        let a595 = Str::lit("");
        if equal(a594, a595) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a597 = Str::lit(" ");
    Ok(join(a596, a597))
}

pub fn f195(v0: Adt39, v1: Fn2, v2: Adt25) -> R<Result<Adt36, Adt93>> {
    let m599 = {
        let c598: Fn2 = v1;
        c598.apply(v2)?
    };
    match m599 {
        Ok(v4) => {
            f286(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt36, Adt93>(v3))
        }
    }
}

pub fn f196(v0: u64, v1: Str) -> Result<Adt53, Adt93> {
    Err::<Adt53, Adt93>(Adt93::C15(f45(v0, v1)))
}

pub fn f197(v0: Str, v1: Adt25) -> Str {
    let m601 = match v1 {
        Adt25::C0(_, _, h600, _, _) => {
            h600
        }
    };
    match m601 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m602 = f235(v0, v2);
            match m602 {
                Adt116::C0 => {
                    Str::lit("")
                }
                Adt116::C1(v3) => {
                    let m603 = f287(v3);
                    match m603 {
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

pub fn f198(v0: Str) -> Result<Adt53, Adt93> {
    Ok::<Adt53, Adt93>(f138(List::cons(f102(v0), List::<Adt42>::nil())))
}

pub fn f199() -> Result<Adt53, Adt93> {
    Err::<Adt53, Adt93>(f49())
}

pub fn f200(v0: Adt39, v1: Result<Adt105, Adt93>) -> R<Result<Adt36, Adt93>> {
    let m604 = v1;
    match m604 {
        Ok(v3) => {
            let m605 = v3;
            match m605 {
                Adt105::C0(v4) => {
                    f286(v0, v4)
                }
                Adt105::C1(v5) => {
                    Ok(Ok::<Adt36, Adt93>(Adt36::C12(v5)))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt36, Adt93>(v2))
        }
    }
}

pub fn f201(v0: Str) -> Result<Adt105, Adt93> {
    Ok::<Adt105, Adt93>(Adt105::C0(Adt53::C0(Adt41::C0(List::cons(Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)), List::<Adt42>::nil()), None::<Adt9>, None::<bool>), List::<Adt54>::nil())))
}

pub fn f202(v0: List<Str>) -> Str {
    let a606 = v0;
    let a607 = Str::lit("");
    join(a606, a607)
}

pub fn f203(v0: List<(Str, Adt68)>) -> Result<Adt105, Adt93> {
    Ok::<Adt105, Adt93>(Adt105::C1(Adt85::C0(v0)))
}

pub fn f204() -> Adt68 {
    f166(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f205() -> Result<Adt105, Adt93> {
    Err::<Adt105, Adt93>(Adt93::C11)
}

pub fn f206(v0: Str, v1: u64) -> Adt68 {
    Adt68::C1(Adt73::C0(List::cons(f288(v0), List::<Adt74>::nil()), None::<Adt79>, None::<Str>, None::<Adt81>, None::<Adt10>, v1, None::<List<Str>>, None::<Adt8>, None::<List<Adt39>>, None::<Adt82>))
}

pub fn f207(v0: u64) -> R<Str> {
    let a608 = f289(v0, 0i64)?;
    Ok(format_i64(a608))
}

pub fn f208(v0: Adt8) -> u64 {
    let m609 = f235(Str::lit("roots"), v0);
    match m609 {
        Adt116::C0 => {
            0u64
        }
        Adt116::C1(v1) => {
            let m610 = f290(v1);
            match m610 {
                None => {
                    0u64
                }
                Some(v2) => {
                    let a611 = v2;
                    length_list(a611)
                }
            }
        }
    }
}

pub fn f209() -> Adt68 {
    Adt68::C2(Some(f32()))
}

pub fn f210(v0: Adt25, v1: Str, v2: Str) -> Result<Adt105, Adt93> {
    if f211(v0.clone(), v1.clone()) {
        let m613 = f162(Str::lit("confirm"), Adt106::C0, match v0 {
            Adt25::C0(_, _, _, h612, _) => {
                h612
            }
        });
        match m613 {
            Adt108::C0 => {
                f213(f165(Str::lit("confirm"), f292()), v1)
            }
            Adt108::C1 => {
                f205()
            }
            Adt108::C2(v3) => {
                if f291(v3) {
                    f201(v2)
                } else {
                    f213(f165(Str::lit("confirm"), f292()), v1)
                }
            }
        }
    } else {
        f213(f165(Str::lit("confirm"), f292()), v1)
    }
}

pub fn f211(v0: Adt25, v1: Str) -> bool {
    let m615 = match v0 {
        Adt25::C0(_, _, _, _, h614) => {
            h614
        }
    };
    match m615 {
        None => {
            false
        }
        Some(v2) => {
            let a616 = v2;
            let a617 = v1;
            equal(a616, a617)
        }
    }
}

pub fn f212(v0: Adt108, v1: Adt108, v2: Adt108) -> R<Result<Adt105, Adt93>> {
    if if f293(v0.clone()) {
        true
    } else {
        if f293(v1.clone()) {
            true
        } else {
            f293(v2.clone())
        }
    } {
        Ok(f205())
    } else {
        let m618 = f214(v0.clone(), v1.clone(), v2.clone());
        match m618.uncons() {
            None => {
                Ok(f201(f202(List::cons(Str::lit("Name: "), List::cons(f164(f294(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f164(f294(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f207(f208(f294(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f213(f214(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f213(v0: List<(Str, Adt68)>, v1: Str) -> Result<Adt105, Adt93> {
    Ok::<Adt105, Adt93>(Adt105::C1(Adt85::C2(v0, v1)))
}

pub fn f214(v0: Adt108, v1: Adt108, v2: Adt108) -> List<(Str, Adt68)> {
    let a621 = if f295(v0) {
        f165(Str::lit("user_name"), f204())
    } else {
        List::<(Str, Adt68)>::nil()
    };
    let a622 = {
        let a619 = if f295(v1) {
            f165(Str::lit("greeting"), f206(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt68)>::nil()
        };
        let a620 = if f295(v2) {
            f165(Str::lit("client_roots"), f209())
        } else {
            List::<(Str, Adt68)>::nil()
        };
        append_list(a619, a620)
    };
    append_list(a621, a622)
}

pub fn f215(v0: Str) -> R<u64> {
    f62({
        let a623 = v0;
        utf8_encode(a623)
    }, 0u64)
}

pub fn f216(v0: Adt25) -> Str {
    let m625 = match v0 {
        Adt25::C0(_, _, _, _, h624) => {
            h624
        }
    };
    match m625 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f217(v0: Adt25) -> Result<Adt105, Adt93> {
    let m627 = f162(Str::lit("step1"), Adt106::C0, match v0 {
        Adt25::C0(_, _, _, h626, _) => {
            h626
        }
    });
    match m627 {
        Adt108::C0 => {
            f213(f165(Str::lit("step1"), f219()), Str::lit("1"))
        }
        Adt108::C1 => {
            f205()
        }
        Adt108::C2(v1) => {
            f213(f165(Str::lit("step2"), f296()), f202(List::cons(Str::lit("2"), List::cons(f164(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f218(v0: Adt25) -> Result<Adt105, Adt93> {
    let m629 = f162(Str::lit("step2"), Adt106::C0, match v0.clone() {
        Adt25::C0(_, _, _, h628, _) => {
            h628
        }
    });
    match m629 {
        Adt108::C0 => {
            f213(f165(Str::lit("step2"), f296()), f216(v0))
        }
        Adt108::C1 => {
            f205()
        }
        Adt108::C2(v1) => {
            f201(f202(List::cons(Str::lit("Multi-round complete for "), List::cons(f297(f216(v0)), List::cons(Str::lit(" who likes "), List::cons(f164(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f219() -> Adt68 {
    f166(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f220(v0: List<(Str, Adt68)>) -> Result<Adt105, Adt93> {
    let m630 = v0.clone();
    match m630.uncons() {
        None => {
            f201(Str::lit("No supported capabilities declared"))
        }
        Some((_, _)) => {
            f213(v0, Str::lit("capabilities-test"))
        }
    }
}

pub fn f221(v0: Adt21) -> List<(Str, Adt68)> {
    let a633 = if f298(match v0.clone() {
        Adt21::C0(_, _, _, h631, _) => {
            h631
        }
    }) {
        f165(Str::lit("elicit_input"), f166(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt68)>::nil()
    };
    let a634 = if f298(match v0 {
        Adt21::C0(_, _, h632, _, _) => {
            h632
        }
    }) {
        f165(Str::lit("sample_input"), f206(Str::lit("Sample request"), 50u64))
    } else {
        List::<(Str, Adt68)>::nil()
    };
    append_list(a633, a634)
}

pub fn f222(v0: List<(Str, Adt9)>) -> List<Str> {
    f299(f300(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f223(v0: Option<Adt23>) -> Str {
    let m635 = v0;
    match m635 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m636 = v1;
            match m636 {
                Adt23::C0(v2) => {
                    v2
                }
                Adt23::C1(v3) => {
                    let a637 = v3;
                    format_i64(a637)
                }
            }
        }
    }
}

pub fn f224(v0: u64, v1: u64) -> R<Adt54> {
    Ok(Adt54::C0(f301(v0)?, Some(f301(v1)?), Some({
        let a640 = List::cons(Str::lit("Completed step"), List::cons({
            let a638 = f289(v0, 0i64)?;
            format_i64(a638)
        }, List::cons(Str::lit("of"), List::cons({
            let a639 = f289(v1, 0i64)?;
            format_i64(a639)
        }, List::<Str>::nil()))));
        let a641 = Str::lit(" ");
        join(a640, a641)
    })))
}

pub fn f225() -> Adt21 {
    Adt21::C0(None::<Adt8>, None::<Adt8>, Some(f32()), None::<Adt8>, None::<Adt8>)
}

pub fn f226() -> Adt53 {
    f138(List::cons(f102(Str::lit("ok")), List::<Adt42>::nil()))
}

pub fn f227(v0: List<(Adt48, List<Adt50>)>) -> List<Adt48> {
    f302(f303(v0, List::<Adt48>::nil()), List::<Adt48>::nil())
}

pub fn f228() -> List<(Adt48, List<Adt50>)> {
    List::cons(f304(f305(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f247(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f304(f305(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f306(Str::lit("test://static-binary"), Str::lit("image/png"), f248())), List::cons(f304(f305(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f247(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f304(f305(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f247(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt48, List<Adt50>)>::nil()))))
}

pub fn f229(v0: Adt13, v1: Str, v2: Option<List<Adt50>>) -> Result<Adt36, Adt93> {
    let m642 = v2;
    match m642 {
        None => {
            Err::<Adt36, Adt93>(Adt93::C2(v1))
        }
        Some(v3) => {
            let m643 = v3.clone();
            match m643.uncons() {
                None => {
                    Err::<Adt36, Adt93>(Adt93::C9(Str::lit("a listed resource has no contents")))
                }
                Some((_, _)) => {
                    Ok::<Adt36, Adt93>(Adt36::C5(Adt56::C0(v0, v3)))
                }
            }
        }
    }
}

pub fn f230(v0: Str, p652: List<(Adt48, List<Adt50>)>) -> Option<List<Adt50>> {
    let m651 = p652;
    {
        let mut m651 = m651;
        loop {
            let step = {
                let v1 = m651;
                let m644 = v1;
                match m644.uncons() {
                    None => {
                        Err::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(None::<List<Adt50>>)
                    }
                    Some((v2, v3)) => {
                        let a648 = {
                            let m647 = {
                                let (h646, _) = v2;
                                h646
                            };
                            match m647 {
                                Adt48::C0(_, _, _, h645, _, _, _, _, _) => {
                                    h645
                                }
                            }
                        };
                        let a649 = v0.clone();
                        if equal(a648, a649) {
                            Err::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(Some({
                                let (_, h650) = v2;
                                h650
                            }))
                        } else {
                            Ok::<List<(Adt48, List<Adt50>)>, Option<List<Adt50>>>(v3)
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

pub fn f231(v0: Str) -> R<Option<List<Adt50>>> {
    let m653 = f307(v0.clone())?;
    match m653 {
        None => {
            Ok(None::<List<Adt50>>)
        }
        Some(v1) => {
            let m654 = f308(v1)?;
            match m654 {
                None => {
                    Ok(None::<List<Adt50>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f247(v0, Str::lit("application/json"), v2), List::<Adt50>::nil())))
                }
            }
        }
    }
}

pub fn f232() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f233(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt61>>) -> Adt60 {
    Adt60::C0(v0, Some(v1), None::<List<Adt5>>, Some(v2), v3, None::<Adt8>)
}

pub fn f234(v0: Str, v1: Str) -> Adt61 {
    Adt61::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f235(v0: Str, v1: Adt8) -> Adt116 {
    f309(v0, match v1 {
        Adt8::C0(h655) => {
            h655
        }
    })
}

pub fn f236(v0: Adt9) -> Option<Adt8> {
    let m656 = v0;
    match m656 {
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

pub fn f237(v0: Adt106, v1: Adt8) -> bool {
    let m657 = v0;
    match m657 {
        Adt106::C0 => {
            f310(v1)
        }
        Adt106::C1 => {
            f311(v1)
        }
        Adt106::C2 => {
            f312(v1)
        }
    }
}

pub fn f238(v0: Str, v1: Adt8, v2: Str) -> Str {
    let m658 = f235(v0, v1);
    match m658 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            let m659 = f287(v3);
            match m659 {
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

pub fn f239(v0: Str, v1: Str) -> Adt8 {
    Adt8::C0(List::cons((Str::lit("type"), Adt9::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt9::C5(List::cons((v0.clone(), Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil()))), List::cons((Str::lit("required"), Adt9::C4(List::cons(Adt9::C3(v0), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil()))))
}

pub fn f240(v0: Str, p665: List<Adt60>) -> Option<Adt60> {
    let m664 = p665;
    {
        let mut m664 = m664;
        loop {
            let step = {
                let v1 = m664;
                let m660 = v1;
                match m660.uncons() {
                    None => {
                        Err::<List<Adt60>, Option<Adt60>>(None::<Adt60>)
                    }
                    Some((v2, v3)) => {
                        let a662 = match v2.clone() {
                            Adt60::C0(h661, _, _, _, _, _) => {
                                h661
                            }
                        };
                        let a663 = v0.clone();
                        if equal(a662, a663) {
                            Err::<List<Adt60>, Option<Adt60>>(Some(v2))
                        } else {
                            Ok::<List<Adt60>, Option<Adt60>>(v3)
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

pub fn f241(p671: List<Adt61>, v1: List<(Str, Str)>) -> Option<Str> {
    let m670 = p671;
    {
        let mut m670 = m670;
        loop {
            let step = {
                let v0 = m670;
                let m666 = v0;
                match m666.uncons() {
                    None => {
                        Err::<List<Adt61>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f313(v2.clone()) {
                            let m668 = f314(match v2.clone() {
                                Adt61::C0(h667, _, _, _) => {
                                    h667
                                }
                            }, v1.clone());
                            match m668 {
                                None => {
                                    Err::<List<Adt61>, Option<Str>>(Some(match v2 {
                                        Adt61::C0(h669, _, _, _) => {
                                            h669
                                        }
                                    }))
                                }
                                Some(_) => {
                                    Ok::<List<Adt61>, Option<Str>>(v3)
                                }
                            }
                        } else {
                            Ok::<List<Adt61>, Option<Str>>(v3)
                        }
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

pub fn f242(v0: Adt60) -> List<Adt61> {
    let m673 = match v0 {
        Adt60::C0(_, _, _, _, h672, _) => {
            h672
        }
    };
    match m673 {
        None => {
            List::<Adt61>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f243(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m674 = v0;
    match m674 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f244(v0: List<Adt63>) -> Adt62 {
    Adt62::C0(None::<Str>, v0)
}

pub fn f245(v0: Str) -> Adt63 {
    Adt63::C0(Adt45::C0, Adt42::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>)))
}

pub fn f246(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m675 = f314(v0, v1);
    match m675 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f247(v0: Str, v1: Str, v2: Str) -> Adt50 {
    Adt50::C0(Adt51::C0(v0, Some(v1), v2, None::<Adt8>))
}

pub fn f248() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f249(v0: List<Str>) -> R<Adt64> {
    let v1: u64 = f315(v0.clone(), 0u64)?;
    Ok(Adt64::C0(Adt65::C0(f316(v0), Some(v1), Some({
        let a676 = f317();
        let a677 = v1;
        nat_lt(a676, a677)
    }))))
}

pub fn f250(p683: List<Adt61>, v1: Str) -> bool {
    let m682 = p683;
    {
        let mut m682 = m682;
        loop {
            let step = {
                let v0 = m682;
                let m678 = v0;
                match m678.uncons() {
                    None => {
                        Err::<List<Adt61>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a680 = match v2 {
                            Adt61::C0(h679, _, _, _) => {
                                h679
                            }
                        };
                        let a681 = v1.clone();
                        if equal(a680, a681) {
                            Err::<List<Adt61>, bool>(true)
                        } else {
                            Ok::<List<Adt61>, bool>(v3)
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

pub fn f251(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f299(f318(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f252(v0: Str, v1: Str) -> List<Str> {
    let a684 = v0;
    let a685 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a684, a685) {
        let a686 = v1;
        let a687 = Str::lit("resourceUri");
        equal(a686, a687)
    } else {
        false
    } {
        f319()
    } else {
        List::<Str>::nil()
    }
}

pub fn f253() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f254(p690: List<Adt100>, p691: List<Adt100>) -> List<Adt100> {
    let m689 = (p690, p691);
    {
        let mut m689 = m689;
        loop {
            let step = {
                let (v0, v1) = m689;
                let m688 = v0;
                match m688.uncons() {
                    None => {
                        Err::<(List<Adt100>, List<Adt100>), List<Adt100>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt100>, List<Adt100>), List<Adt100>>((v3, List::cons(v2, v1)))
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

pub fn f255(v0: Bytes, v1: u64) -> R<Option<Adt95>> {
    let a693 = v1;
    let a694 = {
        let a692 = v0.clone();
        length_bytes(a692)
    };
    if nat_eq(a693, a694) {
        Ok(Some(Adt95::C0(Bytes::lit(&[]), v1)))
    } else {
        f7(v0, v1)
    }
}

pub fn f256(v0: Bytes, v1: Adt99, v2: u64) -> R<Result<Adt99, Adt2>> {
    let a696 = v2;
    let a697 = {
        let a695 = v0.clone();
        length_bytes(a695)
    };
    if nat_eq(a696, a697) {
        Ok(Ok::<Adt99, Adt2>(v1))
    } else {
        let m698 = f27(v0.clone(), v2)?;
        match m698 {
            None => {
                Ok(Err::<Adt99, Adt2>(Adt2::C0(v2)))
            }
            Some(v3) => {
                let m701 = f320(v0.clone(), {
                    let a699 = v2;
                    let a700 = 4u64;
                    nat_add(a699, a700)?
                }, v3, List::<List<u64>>::nil())?;
                match m701 {
                    Ok(v5) => {
                        Ok(f321(v0.clone(), f322(v1, {
                            let (h702, _) = v5;
                            h702
                        }), f323(v0, {
                            let (_, h703) = v5;
                            h703
                        })?))
                    }
                    Err(v4) => {
                        Ok(Err::<Adt99, Adt2>(v4))
                    }
                }
            }
        }
    }
}

pub fn f257() -> Bytes {
    Bytes::lit(&[])
}

pub fn f258(v0: Adt100) -> List<u64> {
    let a714 = f28({
        let a705 = match v0.clone() {
            Adt100::C0(h704, _) => {
                h704
            }
        };
        length_list(a705)
    });
    let a715 = {
        let a712 = match v0.clone() {
            Adt100::C0(h706, _) => {
                h706
            }
        };
        let a713 = {
            let a710 = f28({
                let a708 = match v0.clone() {
                    Adt100::C0(_, h707) => {
                        h707
                    }
                };
                length_list(a708)
            });
            let a711 = match v0 {
                Adt100::C0(_, h709) => {
                    h709
                }
            };
            append_list(a710, a711)
        };
        append_list(a712, a713)
    };
    append_list(a714, a715)
}

pub fn f259(v0: List<u64>) -> List<u64> {
    let a717 = f28({
        let a716 = v0.clone();
        length_list(a716)
    });
    let a718 = v0;
    append_list(a717, a718)
}

pub fn f260(v0: u64, v1: Adt9) -> R<Adt104> {
    let m719 = f324(v1)?;
    match m719 {
        Ok(v3) => {
            Ok(Adt104::C0(v0, f108(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f57())
        }
    }
}

pub fn f261(v0: Adt107) -> u64 {
    let m720 = v0;
    match m720 {
        Adt107::C0 => {
            f325()
        }
        Adt107::C1 => {
            f325()
        }
    }
}

pub fn f262(v0: Option<Adt109>, v1: Adt84) -> Adt9 {
    Adt9::C5(f326(v0, v1))
}

pub fn f263(v0: Adt107) -> Adt84 {
    Adt84::C0(f327(), f328(v0), None::<Adt9>)
}

pub fn f264(v0: List<u64>) -> bool {
    f53(v0, f329())
}

pub fn f265(v0: Adt101, v1: Adt3, v2: Fn3, v3: u64, v4: List<Adt100>, v5: Result<Adt9, Adt117>) -> R<Adt104> {
    let m721 = v5;
    match m721 {
        Ok(v7) => {
            f332(v0, v1, v2, v3, v4, f333(v7)?)
        }
        Err(_) => {
            f260(f330(), f262(None::<Adt109>, f331(Adt118::C0)))
        }
    }
}

pub fn f266(v0: Adt3, v1: Bytes) -> Adt3 {
    Adt3::C0(match v0.clone() {
        Adt3::C0(h722, _, _, _, _, _, _) => {
            h722
        }
    }, match v0.clone() {
        Adt3::C0(_, h723, _, _, _, _, _) => {
            h723
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h724, _, _, _, _) => {
            h724
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h725, _, _, _) => {
            h725
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, h726, _, _) => {
            h726
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, h727, _) => {
            h727
        }
    }, v1)
}

pub fn f267(v0: Fn1, v1: Adt99, v2: Adt18) -> R<Result<Adt36, Adt93>> {
    let c728: Fn1 = v0;
    c728.apply(match v1 {
        Adt99::C0(_, _, _, _, _, _, _, h729) => {
            h729
        }
    }, v2)
}

pub fn f268(v0: Bytes) -> R<Result<Adt9, Adt117>> {
    f334(f29(v0)?)
}

pub fn f269() -> Adt104 {
    Adt104::C0(f335(), List::cons(Adt100::C0(f336(), f337()), List::<Adt100>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f270(v0: List<Adt100>, v1: List<u64>) -> R<Adt120> {
    let m730 = f338(v0, v1, Adt121::C0)?;
    match m730 {
        Adt121::C0 => {
            Ok(Adt120::C0)
        }
        Adt121::C1(v2) => {
            let m731 = f339(v2);
            match m731 {
                None => {
                    Ok(Adt120::C1)
                }
                Some(v3) => {
                    Ok(Adt120::C3(v3))
                }
            }
        }
        Adt121::C2 => {
            Ok(Adt120::C2)
        }
    }
}

pub fn f271() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f272(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt107) -> R<Option<Adt107>> {
    let m732 = v0;
    match m732 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f340(v3, v1)? {
                Ok(None::<Adt107>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f273(v0: List<u64>) -> Option<List<u64>> {
    let m733 = f341(v0);
    match m733 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f275(v1)
        }
    }
}

pub fn f274() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f275(v0: List<u64>) -> Option<List<u64>> {
    let m734 = v0.clone();
    match m734.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a735 = v1;
            let a736 = 91u64;
            if nat_eq(a735, a736) {
                f342(f343(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f342(f344(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f276(v0: Adt3, v1: Fn3, v2: u64, v3: List<Adt100>, v4: Result<Adt9, Adt117>) -> R<Adt96> {
    let m737 = v4;
    match m737 {
        Ok(v6) => {
            f345(v0, v1, v2, v3, f333(v6)?)
        }
        Err(_) => {
            Ok(Adt96::C0(f56(f260(f330(), f262(None::<Adt109>, f331(Adt118::C0)))?)))
        }
    }
}

pub fn f277(v0: List<u64>) -> bool {
    let m738 = v0.clone();
    match m738.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a742 = if f346(v0.clone(), false) {
                let a740 = {
                    let a739 = v0;
                    length_list(a739)
                };
                let a741 = f347();
                nat_le(a740, a741)
            } else {
                false
            };
            bool_not(a742)
        }
    }
}

pub fn f278(v0: Adt39) -> Option<Adt102> {
    let m747 = f348(match v0.clone() {
        Adt39::C0(h743, _, _, _, _, _, _, _) => {
            h743
        }
    }, {
        let m746 = match v0.clone() {
            Adt39::C0(_, _, _, _, h745, _, _, _) => {
                h745
            }
        };
        match m746 {
            Adt8::C0(h744) => {
                h744
            }
        }
    });
    match m747 {
        None => {
            f349(match v0.clone() {
                Adt39::C0(h748, _, _, _, _, _, _, _) => {
                    h748
                }
            }, {
                let m751 = match v0 {
                    Adt39::C0(_, _, _, _, h750, _, _, _) => {
                        h750
                    }
                };
                match m751 {
                    Adt8::C0(h749) => {
                        h749
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f279(v0: Adt39, v1: Result<Adt122, Adt102>) -> R<Result<List<Adt15>, Adt102>> {
    let m752 = v1;
    match m752 {
        Ok(v3) => {
            let m753 = f350(v0)?;
            match m753 {
                None => {
                    Ok(Ok::<List<Adt15>, Adt102>(f116(match v3 {
                        Adt122::C0(h754, _) => {
                            h754
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

pub fn f280(p761: u64, v1: Str, v2: Adt9, p762: List<Adt110>, p763: Adt122) -> R<Result<Adt122, Adt102>> {
    let m760 = (p761, (p762, p763));
    {
        let mut m760 = m760;
        loop {
            let step = {
                let (v0, (v3, v4)) = m760;
                let m755 = v0;
                if m755 == 0 {
                    Err::<(u64, (List<Adt110>, Adt122)), R<Result<Adt122, Adt102>>>(Ok(Err::<Adt122, Adt102>(Adt102::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m755 - 1;
                    let m756 = v3;
                    match m756.uncons() {
                        None => {
                            Err::<(u64, (List<Adt110>, Adt122)), R<Result<Adt122, Adt102>>>(Ok(Ok::<Adt122, Adt102>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m757 = f351(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m757 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt110>, Adt122)), R<Result<Adt122, Adt102>>>((v5, ({
                                        let (h758, _) = v9;
                                        h758
                                    }, {
                                        let (_, h759) = v9;
                                        h759
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt110>, Adt122)), R<Result<Adt122, Adt102>>>(Ok(Err::<Adt122, Adt102>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m760 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281() -> u64 {
    67108864u64
}

pub fn f282() -> Adt122 {
    Adt122::C0(List::<Adt15>::nil(), List::<List<u64>>::nil())
}

pub fn f283(v0: Adt9, p782: u64, p783: Adt113, p784: List<Adt114>) -> R<Adt111> {
    let m781 = (p782, (p783, p784));
    {
        let mut m781 = m781;
        loop {
            let step = {
                let (v1, (v2, v3)) = m781;
                let m764 = v1;
                if m764 == 0 {
                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C4))
                } else {
                    let v4 = m764 - 1;
                    let m765 = v3;
                    match m765.uncons() {
                        None => {
                            let m766 = v2;
                            match m766 {
                                Adt113::C0 => {
                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C0))
                                }
                                Adt113::C1(v5, v6) => {
                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m767 = v2.clone();
                            match m767 {
                                Adt113::C0 => {
                                    let m768 = v7.clone();
                                    match m768 {
                                        Adt114::C0(v9, v10, v11) => {
                                            let m769 = f352(v0.clone(), v9, v10, v11.clone())?;
                                            match m769 {
                                                Adt125::C0(v12) => {
                                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (Adt113::C1(v11, v12), v8)))
                                                }
                                                Adt125::C1(v13) => {
                                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C2(v13)))
                                                }
                                                Adt125::C2(v14) => {
                                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C3(v14)))
                                                }
                                                Adt125::C3(v15) => {
                                                    let v16: List<Adt114> = {
                                                        let a770 = v15;
                                                        let a771 = v8;
                                                        append_list(a770, a771)
                                                    };
                                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt114::C1(_, _, _) => {
                                            Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (v2, v8)))
                                        }
                                        Adt114::C2(_, _, _, _) => {
                                            let v24: Adt123 = f353(v7, v8)?;
                                            Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (match v24.clone() {
                                                Adt123::C0(h772, _) => {
                                                    h772
                                                }
                                            }, match v24 {
                                                Adt123::C0(_, h773) => {
                                                    h773
                                                }
                                            })))
                                        }
                                        Adt114::C3(v25) => {
                                            Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (Adt113::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt114::C4(_, _, _, _) => {
                                            let v39: Adt123 = f353(v7, v8)?;
                                            Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (match v39.clone() {
                                                Adt123::C0(h777, _) => {
                                                    h777
                                                }
                                            }, match v39 {
                                                Adt123::C0(_, h778) => {
                                                    h778
                                                }
                                            })))
                                        }
                                        Adt114::C5(v26, v27, v28, v29) => {
                                            let m774 = f354(v26, v27, v28, v29.clone());
                                            match m774 {
                                                Adt125::C0(v30) => {
                                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (Adt113::C1(v29, v30), v8)))
                                                }
                                                Adt125::C1(v31) => {
                                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C2(v31)))
                                                }
                                                Adt125::C2(v32) => {
                                                    Err::<(u64, (Adt113, List<Adt114>)), R<Adt111>>(Ok(Adt111::C3(v32)))
                                                }
                                                Adt125::C3(v33) => {
                                                    let v34: List<Adt114> = {
                                                        let a775 = v33;
                                                        let a776 = v8;
                                                        append_list(a775, a776)
                                                    };
                                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt113::C1(v40, v41) => {
                                    let v42: Adt123 = f355(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt113, List<Adt114>)), R<Adt111>>((v4, (match v42.clone() {
                                        Adt123::C0(h779, _) => {
                                            h779
                                        }
                                    }, match v42 {
                                        Adt123::C0(_, h780) => {
                                            h780
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m781 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284() -> u64 {
    67108864u64
}

pub fn f285(v0: List<Adt112>) -> R<Str> {
    let a785 = List::cons(Str::lit(""), f356(v0, List::<Str>::nil())?);
    let a786 = Str::lit("/");
    Ok(join(a785, a786))
}

pub fn f286(v0: Adt39, v1: Adt53) -> R<Result<Adt36, Adt93>> {
    if f357({
        let m789 = match v1.clone() {
            Adt53::C0(h788, _) => {
                h788
            }
        };
        match m789 {
            Adt41::C0(h787, _, _) => {
                h787
            }
        }
    })? {
        f358(v0, v1)
    } else {
        Ok(Err::<Adt36, Adt93>(Adt93::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f287(v0: Adt9) -> Option<Str> {
    let m790 = v0;
    match m790 {
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

pub fn f288(v0: Str) -> Adt74 {
    Adt74::C0(Adt45::C0, Adt75::C0(Adt76::C0(Adt43::C0(v0, None::<Adt44>, None::<Adt8>))), None::<Adt8>)
}

pub fn f289(p793: u64, p794: i64) -> R<i64> {
    let m792 = (p793, p794);
    {
        let mut m792 = m792;
        loop {
            let step = {
                let (v0, v1) = m792;
                let m791 = v0;
                if m791 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m791 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f359(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: Adt9) -> Option<List<Adt9>> {
    let m795 = v0;
    match m795 {
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

pub fn f291(v0: Adt8) -> bool {
    let m796 = f235(Str::lit("content"), v0);
    match m796 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v1) => {
            let m797 = f236(v1);
            match m797 {
                None => {
                    false
                }
                Some(v2) => {
                    let m798 = f235(Str::lit("ok"), v2);
                    match m798 {
                        Adt116::C0 => {
                            false
                        }
                        Adt116::C1(v3) => {
                            let m799 = f360(v3);
                            match m799 {
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

pub fn f292() -> Adt68 {
    f166(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f293(v0: Adt108) -> bool {
    let m800 = v0;
    match m800 {
        Adt108::C0 => {
            false
        }
        Adt108::C1 => {
            true
        }
        Adt108::C2(_) => {
            false
        }
    }
}

pub fn f294(v0: Adt108) -> Adt8 {
    let m801 = v0;
    match m801 {
        Adt108::C0 => {
            f32()
        }
        Adt108::C1 => {
            f32()
        }
        Adt108::C2(v1) => {
            v1
        }
    }
}

pub fn f295(v0: Adt108) -> bool {
    let m802 = v0;
    match m802 {
        Adt108::C0 => {
            true
        }
        Adt108::C1 => {
            false
        }
        Adt108::C2(_) => {
            false
        }
    }
}

pub fn f296() -> Adt68 {
    f166(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f297(v0: Str) -> Str {
    let v1: Bytes = {
        let a803 = v0;
        utf8_encode(a803)
    };
    let m810 = {
        let a807 = v1.clone();
        let a808 = 1u64;
        let a809 = {
            let a805 = {
                let a804 = v1;
                length_bytes(a804)
            };
            let a806 = 1u64;
            nat_sub(a805, a806)
        };
        slice_bytes(a807, a808, a809)
    };
    match m810 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m812 = {
                let a811 = v2;
                utf8_decode(a811)
            };
            match m812 {
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

pub fn f298(v0: Option<Adt8>) -> bool {
    let m813 = v0;
    match m813 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f299(p816: List<Str>, p817: List<Str>) -> List<Str> {
    let m815 = (p816, p817);
    {
        let mut m815 = m815;
        loop {
            let step = {
                let (v0, v1) = m815;
                let m814 = v0;
                match m814.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(p821: List<(Str, Adt9)>, p822: List<Str>) -> List<Str> {
    let m820 = (p821, p822);
    {
        let mut m820 = m820;
        loop {
            let step = {
                let (v0, v1) = m820;
                let m818 = v0;
                match m818.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h819, _) = v2;
                            h819
                        }, v1)))
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

pub fn f301(v0: u64) -> R<Adt10> {
    Ok(Adt10::C0(f289(v0, 0i64)?, 0i64))
}

pub fn f302(p825: List<Adt48>, p826: List<Adt48>) -> List<Adt48> {
    let m824 = (p825, p826);
    {
        let mut m824 = m824;
        loop {
            let step = {
                let (v0, v1) = m824;
                let m823 = v0;
                match m823.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt48>), List<Adt48>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt48>), List<Adt48>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f303(p830: List<(Adt48, List<Adt50>)>, p831: List<Adt48>) -> List<Adt48> {
    let m829 = (p830, p831);
    {
        let mut m829 = m829;
        loop {
            let step = {
                let (v0, v1) = m829;
                let m827 = v0;
                match m827.uncons() {
                    None => {
                        Err::<(List<(Adt48, List<Adt50>)>, List<Adt48>), List<Adt48>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt48, List<Adt50>)>, List<Adt48>), List<Adt48>>((v3, List::cons({
                            let (h828, _) = v2;
                            h828
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: Adt48, v1: Adt50) -> (Adt48, List<Adt50>) {
    (v0, List::cons(v1, List::<Adt50>::nil()))
}

pub fn f305(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt48 {
    Adt48::C0(v0, v1, None::<List<Adt5>>, v2, Some(v3), Some(v4), None::<Adt44>, None::<u64>, None::<Adt8>)
}

pub fn f306(v0: Str, v1: Str, v2: Str) -> Adt50 {
    Adt50::C1(Adt52::C0(v0, Some(v1), v2, None::<Adt8>))
}

pub fn f307(v0: Str) -> R<Option<Str>> {
    let m832 = f361(v0)?;
    match m832 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a833 = f362(f257(), v1);
            Ok(utf8_decode(a833))
        }
    }
}

pub fn f308(v0: Str) -> R<Option<Str>> {
    let m836 = f363(Adt9::C5(List::cons((Str::lit("id"), Adt9::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt9::C1(true)), List::cons((Str::lit("data"), Adt9::C3({
        let a834 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a835 = Str::lit(" ");
        join(a834, a835)
    })), List::<(Str, Adt9)>::nil())))))?;
    match m836 {
        Ok(v1) => {
            let a837 = v1;
            Ok(utf8_decode(a837))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f309(v0: Str, p843: List<(Str, Adt9)>) -> Adt116 {
    let m842 = p843;
    {
        let mut m842 = m842;
        loop {
            let step = {
                let v1 = m842;
                let m838 = v1;
                match m838.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, Adt116>(Adt116::C0)
                    }
                    Some((v2, v3)) => {
                        let m840 = f364({
                            let (h839, _) = v2;
                            h839
                        }, v0.clone());
                        match m840 {
                            Adt126::C0 => {
                                Err::<List<(Str, Adt9)>, Adt116>(Adt116::C1({
                                    let (_, h841) = v2;
                                    h841
                                }))
                            }
                            Adt126::C1 => {
                                Ok::<List<(Str, Adt9)>, Adt116>(v3)
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

pub fn f310(v0: Adt8) -> bool {
    if f365(v0.clone()) {
        f366(v0)
    } else {
        false
    }
}

pub fn f311(v0: Adt8) -> bool {
    if f367(v0.clone()) {
        if f368(f235(Str::lit("model"), v0.clone())) {
            f369(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f312(v0: Adt8) -> bool {
    let m844 = f235(Str::lit("roots"), v0);
    match m844 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v1) => {
            let m845 = f290(v1);
            match m845 {
                None => {
                    false
                }
                Some(v2) => {
                    f370(v2)
                }
            }
        }
    }
}

pub fn f313(v0: Adt61) -> bool {
    let m847 = match v0 {
        Adt61::C0(_, _, _, h846) => {
            h846
        }
    };
    match m847 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f314(v0: Str, p854: List<(Str, Str)>) -> Option<Str> {
    let m853 = p854;
    {
        let mut m853 = m853;
        loop {
            let step = {
                let v1 = m853;
                let m848 = v1;
                match m848.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a850 = {
                            let (h849, _) = v2;
                            h849
                        };
                        let a851 = v0.clone();
                        if equal(a850, a851) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h852) = v2;
                                h852
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(p859: List<Str>, p860: u64) -> R<u64> {
    let m858 = (p859, p860);
    {
        let mut m858 = m858;
        loop {
            let step = {
                let (v0, v1) = m858;
                let m855 = v0;
                match m855.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a856 = v1;
                            let a857 = 1u64;
                            nat_add(a856, a857)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: List<Str>) -> List<Str> {
    f299(f371(v0, f317(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f317() -> u64 {
    100u64
}

pub fn f318(v0: Str, p863: List<Str>, p864: List<Str>) -> R<List<Str>> {
    let m862 = (p863, p864);
    {
        let mut m862 = m862;
        loop {
            let step = {
                let (v1, v2) = m862;
                let m861 = v1;
                match m861.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f372(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319() -> List<Str> {
    f299(f373(f153(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f320(v0: Bytes, p870: u64, p871: u64, p872: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt2>> {
    let m869 = (p870, (p871, p872));
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v1, (v2, v3)) = m869;
                let m865 = v2;
                if m865 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>(Ok(Ok::<(List<List<u64>>, u64), Adt2>((f176(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m865 - 1;
                    let m866 = f7(v0.clone(), v1)?;
                    match m866 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>(Ok(Err::<(List<List<u64>>, u64), Adt2>(Adt2::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt2>>>((match v5.clone() {
                                Adt95::C0(_, h867) => {
                                    h867
                                }
                            }, (v4, List::cons(f29(match v5 {
                                Adt95::C0(h868, _) => {
                                    h868
                                }
                            })?, v3))))
                        }
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

pub fn f321(v0: Bytes, v1: Adt99, v2: Result<(Adt101, u64), Adt2>) -> Result<Adt99, Adt2> {
    let m873 = v2;
    match m873 {
        Ok(v4) => {
            let a876 = {
                let (_, h874) = v4.clone();
                h874
            };
            let a877 = {
                let a875 = v0;
                length_bytes(a875)
            };
            if nat_eq(a876, a877) {
                Ok::<Adt99, Adt2>(Adt99::C0(match v1.clone() {
                    Adt99::C0(h878, _, _, _, _, _, _, _) => {
                        h878
                    }
                }, match v1.clone() {
                    Adt99::C0(_, h879, _, _, _, _, _, _) => {
                        h879
                    }
                }, match v1.clone() {
                    Adt99::C0(_, _, h880, _, _, _, _, _) => {
                        h880
                    }
                }, match v1.clone() {
                    Adt99::C0(_, _, _, h881, _, _, _, _) => {
                        h881
                    }
                }, match v1.clone() {
                    Adt99::C0(_, _, _, _, h882, _, _, _) => {
                        h882
                    }
                }, match v1.clone() {
                    Adt99::C0(_, _, _, _, _, h883, _, _) => {
                        h883
                    }
                }, match v1 {
                    Adt99::C0(_, _, _, _, _, _, h884, _) => {
                        h884
                    }
                }, {
                    let (h885, _) = v4;
                    h885
                }))
            } else {
                Err::<Adt99, Adt2>(Adt2::C1({
                    let (_, h886) = v4;
                    h886
                }))
            }
        }
        Err(v3) => {
            Err::<Adt99, Adt2>(v3)
        }
    }
}

pub fn f322(v0: Adt99, v1: List<List<u64>>) -> Adt99 {
    Adt99::C0(match v0.clone() {
        Adt99::C0(h887, _, _, _, _, _, _, _) => {
            h887
        }
    }, match v0.clone() {
        Adt99::C0(_, h888, _, _, _, _, _, _) => {
            h888
        }
    }, match v0.clone() {
        Adt99::C0(_, _, h889, _, _, _, _, _) => {
            h889
        }
    }, match v0.clone() {
        Adt99::C0(_, _, _, h890, _, _, _, _) => {
            h890
        }
    }, match v0.clone() {
        Adt99::C0(_, _, _, _, h891, _, _, _) => {
            h891
        }
    }, v1, match v0.clone() {
        Adt99::C0(_, _, _, _, _, _, h892, _) => {
            h892
        }
    }, match v0 {
        Adt99::C0(_, _, _, _, _, _, _, h893) => {
            h893
        }
    })
}

pub fn f323(v0: Bytes, v1: u64) -> R<Result<(Adt101, u64), Adt2>> {
    let a895 = v1;
    let a896 = {
        let a894 = v0.clone();
        length_bytes(a894)
    };
    if nat_eq(a895, a896) {
        Ok(Ok::<(Adt101, u64), Adt2>((Adt101::C0(0i64, Str::lit(""), Str::lit(""), f257()), v1)))
    } else {
        f374(v0, v1)
    }
}

pub fn f324(v0: Adt9) -> R<Result<List<u64>, Adt117>> {
    f375(f376(), v0)
}

pub fn f325() -> u64 {
    403u64
}

pub fn f326(v0: Option<Adt109>, v1: Adt84) -> List<(Str, Adt9)> {
    List::cons((Str::lit("jsonrpc"), Adt9::C3(f377())), {
        let a897 = f378(v0);
        let a898 = List::cons((Str::lit("error"), f379(v1)), List::<(Str, Adt9)>::nil());
        append_list(a897, a898)
    })
}

pub fn f327() -> i64 {
    -32000i64
}

pub fn f328(v0: Adt107) -> Str {
    let m899 = v0;
    match m899 {
        Adt107::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt107::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f329() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f330() -> u64 {
    400u64
}

pub fn f331(v0: Adt118) -> Adt84 {
    Adt84::C0(f380(v0.clone()), f381(v0), None::<Adt9>)
}

pub fn f332(v0: Adt101, v1: Adt3, v2: Fn3, v3: u64, v4: List<Adt100>, v5: Result<Adt124, Adt118>) -> R<Adt104> {
    let m900 = v5;
    match m900 {
        Ok(v7) => {
            f384(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f260(f382(v6.clone()), f262(f383(v6.clone()), f331(v6)))
        }
    }
}

pub fn f333(v0: Adt9) -> R<Result<Adt124, Adt118>> {
    let m901 = v0;
    match m901 {
        Adt9::C0 => {
            Ok(f386(None::<Adt109>, Adt119::C0))
        }
        Adt9::C1(_) => {
            Ok(f386(None::<Adt109>, Adt119::C0))
        }
        Adt9::C2(_) => {
            Ok(f386(None::<Adt109>, Adt119::C0))
        }
        Adt9::C3(_) => {
            Ok(f386(None::<Adt109>, Adt119::C0))
        }
        Adt9::C4(_) => {
            Ok(f386(None::<Adt109>, Adt119::C1))
        }
        Adt9::C5(v1) => {
            f385(Adt8::C0(v1))
        }
    }
}

pub fn f334(v0: List<u64>) -> R<Result<Adt9, Adt117>> {
    let m902 = f387(v0.clone())?;
    match m902 {
        Adt133::C0 => {
            f388(f389(), 0u64, v0)
        }
        Adt133::C1(v1) => {
            Ok(Err::<Adt9, Adt117>(Adt117::C0(v1)))
        }
        Adt133::C2(v2) => {
            Ok(Err::<Adt9, Adt117>(Adt117::C1(v2)))
        }
    }
}

pub fn f335() -> u64 {
    405u64
}

pub fn f336() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f337() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f338(p907: List<Adt100>, v1: List<u64>, p908: Adt121) -> R<Adt121> {
    let m906 = (p907, p908);
    {
        let mut m906 = m906;
        loop {
            let step = {
                let (v0, v2) = m906;
                let m903 = v0;
                match m903.uncons() {
                    None => {
                        Err::<(List<Adt100>, Adt121), R<Adt121>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f390(match v3.clone() {
                            Adt100::C0(h904, _) => {
                                h904
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt100>, Adt121), R<Adt121>>((v4, f391(v2, match v3 {
                                Adt100::C0(_, h905) => {
                                    h905
                                }
                            })))
                        } else {
                            Ok::<(List<Adt100>, Adt121), R<Adt121>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f392(v0);
    if f393(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f340(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f390(v0.clone(), f394())? {
        Ok(true)
    } else {
        if f390(v0.clone(), f395())? {
            Ok(true)
        } else {
            if f390(v0.clone(), f396())? {
                Ok(true)
            } else {
                f397(v0, v1)
            }
        }
    }
}

pub fn f341(p913: List<u64>) -> Option<List<u64>> {
    let m912 = p913;
    {
        let mut m912 = m912;
        loop {
            let step = {
                let v0 = m912;
                let m909 = v0;
                match m909.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a910 = v1;
                        let a911 = 58u64;
                        if nat_eq(a910, a911) {
                            Err::<List<u64>, Option<List<u64>>>(f398(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
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

pub fn f342(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m914 = v0;
    match m914 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f399({
                let (_, h915) = v1;
                h915
            }) {
                Some({
                    let (h916, _) = v1;
                    h916
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f343(p921: List<u64>, p922: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m920 = (p921, p922);
    {
        let mut m920 = m920;
        loop {
            let step = {
                let (v0, v1) = m920;
                let m917 = v0;
                match m917.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a918 = v2;
                        let a919 = 93u64;
                        if nat_eq(a918, a919) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f400(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
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

pub fn f344(p927: List<u64>, p928: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m926 = (p927, p928);
    {
        let mut m926 = m926;
        loop {
            let step = {
                let (v0, v1) = m926;
                let m923 = v0;
                match m923.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f400(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a924 = v2;
                        let a925 = 58u64;
                        if nat_eq(a924, a925) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f400(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m926 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: Adt3, v1: Fn3, v2: u64, v3: List<Adt100>, v4: Result<Adt124, Adt118>) -> R<Adt96> {
    let m929 = v4;
    match m929 {
        Ok(v6) => {
            f401(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            Ok(Adt96::C0(f56(f260(f382(v5.clone()), f262(f383(v5.clone()), f331(v5)))?)))
        }
    }
}

pub fn f346(p932: List<u64>, v1: bool) -> bool {
    let m931 = p932;
    {
        let mut m931 = m931;
        loop {
            let step = {
                let v0 = m931;
                let m930 = v0;
                match m930.uncons() {
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
                Ok(next) => { m931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347() -> u64 {
    64u64
}

pub fn f348(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt102> {
    let m933 = f309(Str::lit("type"), v1);
    match m933 {
        Adt116::C0 => {
            Some(Adt102::C3(v0))
        }
        Adt116::C1(v2) => {
            let m934 = f287(v2);
            match m934 {
                None => {
                    Some(Adt102::C3(v0))
                }
                Some(v3) => {
                    let a935 = v3;
                    let a936 = Str::lit("object");
                    if equal(a935, a936) {
                        None::<Adt102>
                    } else {
                        Some(Adt102::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f349(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt102> {
    let m937 = f309(Str::lit("$schema"), v1);
    match m937 {
        Adt116::C0 => {
            None::<Adt102>
        }
        Adt116::C1(v2) => {
            let m938 = f287(v2);
            match m938 {
                None => {
                    Some(Adt102::C2(v0))
                }
                Some(v3) => {
                    let a939 = v3;
                    let a940 = f404();
                    if equal(a939, a940) {
                        None::<Adt102>
                    } else {
                        Some(Adt102::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f350(v0: Adt39) -> R<Option<Adt102>> {
    let m942 = match v0.clone() {
        Adt39::C0(_, _, _, _, _, h941, _, _) => {
            h941
        }
    };
    match m942 {
        None => {
            Ok(None::<Adt102>)
        }
        Some(v1) => {
            let m945 = f349(match v0.clone() {
                Adt39::C0(h943, _, _, _, _, _, _, _) => {
                    h943
                }
            }, match v1.clone() {
                Adt8::C0(h944) => {
                    h944
                }
            });
            match m945 {
                None => {
                    let m949 = f280(f281(), match v0 {
                        Adt39::C0(h946, _, _, _, _, _, _, _) => {
                            h946
                        }
                    }, Adt9::C5(match v1.clone() {
                        Adt8::C0(h947) => {
                            h947
                        }
                    }), List::cons(Adt110::C0(Adt9::C5(match v1 {
                        Adt8::C0(h948) => {
                            h948
                        }
                    }), false, List::<Str>::nil()), List::<Adt110>::nil()), f282())?;
                    match m949 {
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

pub fn f351(v0: Str, v1: Adt9, v2: Adt110, v3: Adt122, v4: List<Adt110>) -> R<Result<(List<Adt110>, Adt122), Adt102>> {
    let m950 = v2;
    match m950 {
        Adt110::C0(v5, v6, v7) => {
            let m951 = f360(v5.clone());
            match m951 {
                None => {
                    let m952 = f236(v5);
                    match m952 {
                        None => {
                            Ok(Err::<(List<Adt110>, Adt122), Adt102>(Adt102::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f405(v0, v1, match v9 {
                                Adt8::C0(h953) => {
                                    h953
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt110>, Adt122), Adt102>((v4, v3)))
                }
            }
        }
    }
}

pub fn f352(v0: Adt9, v1: Adt9, v2: Adt9, v3: List<Adt112>) -> R<Adt125> {
    let m954 = f360(v1.clone());
    match m954 {
        None => {
            let m955 = f236(v1);
            match m955 {
                None => {
                    Ok(Adt125::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f406(v0, match v5 {
                        Adt8::C0(h956) => {
                            h956
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt125::C3(List::<Adt114>::nil()))
            } else {
                Ok(Adt125::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f353(v0: Adt114, v1: List<Adt114>) -> R<Adt123> {
    let m957 = v0;
    match m957 {
        Adt114::C0(_, _, _) => {
            Ok(Adt123::C0(Adt113::C0, v1))
        }
        Adt114::C1(_, _, _) => {
            Ok(Adt123::C0(Adt113::C0, v1))
        }
        Adt114::C2(v8, v9, v10, v11) => {
            Ok(f407(v8, v9, v10, {
                let a958 = v11;
                let a959 = 1u64;
                nat_add(a958, a959)?
            }, v1))
        }
        Adt114::C3(v12) => {
            Ok(Adt123::C0(Adt113::C1(v12, Str::lit("not")), v1))
        }
        Adt114::C4(v13, _, v15, v16) => {
            Ok(f408(v13, v15, v16, v1))
        }
        Adt114::C5(_, _, _, _) => {
            Ok(Adt123::C0(Adt113::C0, v1))
        }
    }
}

pub fn f354(v0: Adt115, v1: Adt9, v2: Adt9, v3: List<Adt112>) -> Adt125 {
    let m960 = f236(v1);
    match m960 {
        None => {
            Adt125::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m961 = v0;
            match m961 {
                Adt115::C0 => {
                    f409(match v4 {
                        Adt8::C0(h962) => {
                            h962
                        }
                    }, v2, v3)
                }
                Adt115::C1 => {
                    f410(match v4 {
                        Adt8::C0(h963) => {
                            h963
                        }
                    }, v2, v3)
                }
                Adt115::C2 => {
                    f411(match v4 {
                        Adt8::C0(h964) => {
                            h964
                        }
                    }, v2, v3)
                }
                Adt115::C3 => {
                    f412(match v4 {
                        Adt8::C0(h965) => {
                            h965
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f355(v0: Adt114, v1: List<Adt114>, v2: List<Adt112>, v3: Str) -> Adt123 {
    let m966 = v0;
    match m966 {
        Adt114::C0(_, _, _) => {
            Adt123::C0(Adt113::C1(v2, v3), v1)
        }
        Adt114::C1(v7, v8, v9) => {
            f413(v7, v8, v9, v1)
        }
        Adt114::C2(v10, v11, v12, v13) => {
            f407(v10, v11, v12, v13, v1)
        }
        Adt114::C3(_) => {
            Adt123::C0(Adt113::C0, v1)
        }
        Adt114::C4(_, v16, v17, v18) => {
            f408(v16, v17, v18, v1)
        }
        Adt114::C5(_, _, _, _) => {
            Adt123::C0(Adt113::C1(v2, v3), v1)
        }
    }
}

pub fn f356(p969: List<Adt112>, p970: List<Str>) -> R<List<Str>> {
    let m968 = (p969, p970);
    {
        let mut m968 = m968;
        loop {
            let step = {
                let (v0, v1) = m968;
                let m967 = v0;
                match m967.uncons() {
                    None => {
                        Err::<(List<Adt112>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt112>, List<Str>), R<List<Str>>>((v3, List::cons(f414(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m968 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(p979: List<Adt42>) -> R<bool> {
    let m978 = p979;
    {
        let mut m978 = m978;
        loop {
            let step = {
                let v0 = m978;
                let m971 = v0;
                match m971.uncons() {
                    None => {
                        Err::<List<Adt42>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m972 = v1;
                        match m972 {
                            Adt42::C0(_) => {
                                Ok::<List<Adt42>, R<bool>>(v2)
                            }
                            Adt42::C1(v4) => {
                                if f415(match v4 {
                                    Adt46::C0(h973, _, _, _) => {
                                        h973
                                    }
                                })? {
                                    Ok::<List<Adt42>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt42>, R<bool>>(Ok(false))
                                }
                            }
                            Adt42::C2(v5) => {
                                if f415(match v5 {
                                    Adt47::C0(h974, _, _, _) => {
                                        h974
                                    }
                                })? {
                                    Ok::<List<Adt42>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt42>, R<bool>>(Ok(false))
                                }
                            }
                            Adt42::C3(_) => {
                                Ok::<List<Adt42>, R<bool>>(v2)
                            }
                            Adt42::C4(v7) => {
                                let m976 = match v7 {
                                    Adt49::C0(h975, _, _) => {
                                        h975
                                    }
                                };
                                match m976 {
                                    Adt50::C0(_) => {
                                        Ok::<List<Adt42>, R<bool>>(v2)
                                    }
                                    Adt50::C1(v9) => {
                                        if f415(match v9 {
                                            Adt52::C0(_, _, h977, _) => {
                                                h977
                                            }
                                        })? {
                                            Ok::<List<Adt42>, R<bool>>(v2)
                                        } else {
                                            Err::<List<Adt42>, R<bool>>(Ok(false))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: Adt39, v1: Adt53) -> R<Result<Adt36, Adt93>> {
    if f416(match v1.clone() {
        Adt53::C0(h980, _) => {
            h980
        }
    }) {
        f417(v1)
    } else {
        let m982 = match v0 {
            Adt39::C0(_, _, _, _, _, h981, _, _) => {
                h981
            }
        };
        match m982 {
            None => {
                f417(v1)
            }
            Some(v2) => {
                f418(v2, v1)
            }
        }
    }
}

pub fn f359(v0: i64, v1: i64) -> R<i64> {
    let a984 = v0;
    let a985 = {
        let a983 = v1;
        int_neg(a983)?
    };
    int_sub(a984, a985)
}

pub fn f360(v0: Adt9) -> Option<bool> {
    let m986 = v0;
    match m986 {
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

pub fn f361(v0: Str) -> R<Option<List<u64>>> {
    let m987 = f419(f420(f421())?, f420(v0)?);
    match m987 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m988 = f419(f422(List::cons(47u64, f420(f423())?)), f422(v1));
            match m988 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f422(v2);
                    if if f424(v3.clone()) {
                        f425(v3.clone())
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

pub fn f362(v0: Bytes, v1: List<u64>) -> Bytes {
    let a989 = v0;
    let a990 = f426(f257(), f427(List::<Bytes>::nil(), v1));
    append_bytes(a989, a990)
}

pub fn f363(v0: Adt9) -> R<Result<Bytes, Adt117>> {
    let m991 = f324(v0)?;
    match m991 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt117>(f362(f257(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt117>(v2))
        }
    }
}

pub fn f364(v0: Str, v1: Str) -> Adt126 {
    let a992 = v0;
    let a993 = v1;
    if equal(a992, a993) {
        Adt126::C0
    } else {
        Adt126::C1
    }
}

pub fn f365(v0: Adt8) -> bool {
    if f428(f235(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f428(f235(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f428(f235(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f366(v0: Adt8) -> bool {
    let m994 = f235(Str::lit("content"), v0);
    match m994 {
        Adt116::C0 => {
            true
        }
        Adt116::C1(v1) => {
            let m995 = f236(v1);
            match m995 {
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

pub fn f367(v0: Adt8) -> bool {
    if f428(f235(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f428(f235(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f368(v0: Adt116) -> bool {
    let m996 = v0;
    match m996 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v1) => {
            let m997 = f287(v1);
            match m997 {
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

pub fn f369(v0: Adt8) -> bool {
    let m998 = f235(Str::lit("content"), v0);
    match m998 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v1) => {
            let m999 = f236(v1.clone());
            match m999 {
                None => {
                    let m1000 = f290(v1);
                    match m1000 {
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

pub fn f370(p1004: List<Adt9>) -> bool {
    let m1003 = p1004;
    {
        let mut m1003 = m1003;
        loop {
            let step = {
                let v0 = m1003;
                let m1001 = v0;
                match m1001.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m1002 = f236(v1);
                        match m1002 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(v3) => {
                                if f368(f235(Str::lit("uri"), v3)) {
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
                Ok(next) => { m1003 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(p1008: List<Str>, p1009: u64, p1010: List<Str>) -> List<Str> {
    let m1007 = (p1008, (p1009, p1010));
    {
        let mut m1007 = m1007;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1007;
                let m1005 = v0;
                match m1005.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m1006 = v1;
                        if m1006 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m1006 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1007 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: Str, v1: Str) -> R<bool> {
    Ok(f429(f420(v0)?, f420(v1)?))
}

pub fn f373(p1014: List<Adt48>, p1015: List<Str>) -> List<Str> {
    let m1013 = (p1014, p1015);
    {
        let mut m1013 = m1013;
        loop {
            let step = {
                let (v0, v1) = m1013;
                let m1011 = v0;
                match m1011.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt48::C0(_, _, _, h1012, _, _, _, _, _) => {
                                h1012
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374(v0: Bytes, v1: u64) -> R<Result<(Adt101, u64), Adt2>> {
    let m1016 = f430(v0.clone(), v1)?;
    match m1016 {
        None => {
            Ok(Err::<(Adt101, u64), Adt2>(Adt2::C0(v1)))
        }
        Some(v2) => {
            let m1019 = f431(v0.clone(), {
                let a1017 = v1;
                let a1018 = 8u64;
                nat_add(a1017, a1018)?
            })?;
            match m1019 {
                Ok(v4) => {
                    f432(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt101, u64), Adt2>(v3))
                }
            }
        }
    }
}

pub fn f375(v0: u64, v1: Adt9) -> R<Result<List<u64>, Adt117>> {
    let m1020 = f433(v0, List::cons(Adt127::C1(0u64, v1), List::<Adt127>::nil()), List::<u64>::nil())?;
    match m1020 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt117>(f434(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt117>(v3))
        }
    }
}

pub fn f376() -> u64 {
    67108864u64
}

pub fn f377() -> Str {
    Str::lit("2.0")
}

pub fn f378(v0: Option<Adt109>) -> List<(Str, Adt9)> {
    let m1021 = v0;
    match m1021 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f435(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f379(v0: Adt84) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("code"), Adt9::C2(Adt10::C0(match v0.clone() {
        Adt84::C0(h1022, _, _) => {
            h1022
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt9::C3(match v0.clone() {
        Adt84::C0(_, h1023, _) => {
            h1023
        }
    })), f436(match v0 {
        Adt84::C0(_, _, h1024) => {
            h1024
        }
    }))))
}

pub fn f380(v0: Adt118) -> i64 {
    let m1025 = v0;
    match m1025 {
        Adt118::C0 => {
            f437()
        }
        Adt118::C1(_, _) => {
            f438()
        }
    }
}

pub fn f381(v0: Adt118) -> Str {
    let m1026 = v0;
    match m1026 {
        Adt118::C0 => {
            Str::lit("Parse error")
        }
        Adt118::C1(_, v2) => {
            f439(v2)
        }
    }
}

pub fn f382(v0: Adt118) -> u64 {
    let m1027 = v0;
    match m1027 {
        Adt118::C0 => {
            f330()
        }
        Adt118::C1(_, _) => {
            f330()
        }
    }
}

pub fn f383(v0: Adt118) -> Option<Adt109> {
    let m1028 = v0;
    match m1028 {
        Adt118::C0 => {
            None::<Adt109>
        }
        Adt118::C1(v1, _) => {
            v1
        }
    }
}

pub fn f384(v0: Adt101, v1: Adt3, v2: Fn3, v3: u64, v4: List<Adt100>, v5: Adt124) -> R<Adt104> {
    f440(v0, f441(v4.clone())?, f442(v1.clone(), v2, v3, Some(f443(v4, match v1 {
        Adt3::C0(_, _, _, _, h1029, _, _) => {
            h1029
        }
    }, f444(v5.clone()), match v5.clone() {
        Adt124::C0(_, h1030, _) => {
            h1030
        }
    })?), f445(), v5)?)
}

pub fn f385(v0: Adt8) -> R<Result<Adt124, Adt118>> {
    let m1031 = f446(v0.clone())?;
    match m1031 {
        Adt148::C0 => {
            let m1032 = f235(Str::lit("method"), v0.clone());
            match m1032 {
                Adt116::C0 => {
                    Ok(f386(None::<Adt109>, f447(v0)))
                }
                Adt116::C1(_) => {
                    Ok(f386(None::<Adt109>, if f448(v0) {
                        Adt119::C8
                    } else {
                        Adt119::C7
                    }))
                }
            }
        }
        Adt148::C1(v2) => {
            Ok(f449(v2, v0))
        }
        Adt148::C2 => {
            Ok(f386(None::<Adt109>, Adt119::C5))
        }
    }
}

pub fn f386(v0: Option<Adt109>, v1: Adt119) -> Result<Adt124, Adt118> {
    Err::<Adt124, Adt118>(Adt118::C1(v0, v1))
}

pub fn f387(v0: List<u64>) -> R<Adt133> {
    Ok(f450({
        let a1033 = v0.clone();
        length_list(a1033)
    }, f451(Adt134::C0, 0u64, 0u64, v0)?))
}

pub fn f388(p1040: Adt135, p1041: u64, p1042: List<u64>) -> R<Result<Adt9, Adt117>> {
    let m1039 = (p1040, (p1041, p1042));
    {
        let mut m1039 = m1039;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1039;
                let m1034 = v2;
                match m1034.uncons() {
                    None => {
                        let m1035 = f452(v0, v1)?;
                        match m1035 {
                            Ok(v3) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt9, Adt117>>>(Ok(Ok::<Adt9, Adt117>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt9, Adt117>>>(Ok(Err::<Adt9, Adt117>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m1036 = f453(v0, v1, v5)?;
                        match m1036 {
                            Adt149::C0(v7) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt9, Adt117>>>(Ok(Err::<Adt9, Adt117>(v7)))
                            }
                            Adt149::C1(v8) => {
                                Ok::<(Adt135, (u64, List<u64>)), R<Result<Adt9, Adt117>>>((v8, ({
                                    let a1037 = v1;
                                    let a1038 = 1u64;
                                    nat_add(a1037, a1038)?
                                }, v6)))
                            }
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

pub fn f389() -> Adt135 {
    Adt135::C0(Adt136::C0, List::<Adt140>::nil(), 0u64)
}

pub fn f390(p1049: List<u64>, p1050: List<u64>) -> R<bool> {
    let m1048 = (p1049, p1050);
    {
        let mut m1048 = m1048;
        loop {
            let step = {
                let (v0, v1) = m1048;
                let m1043 = v0;
                match m1043.uncons() {
                    None => {
                        let m1044 = v1;
                        match m1044.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1045 = v1;
                        match m1045.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a1046 = f454(v4)?;
                                let a1047 = f454(v6)?;
                                if nat_eq(a1046, a1047) {
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
                Ok(next) => { m1048 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Adt121, v1: List<u64>) -> Adt121 {
    let m1051 = v0;
    match m1051 {
        Adt121::C0 => {
            Adt121::C1(v1)
        }
        Adt121::C1(_) => {
            Adt121::C2
        }
        Adt121::C2 => {
            Adt121::C2
        }
    }
}

pub fn f392(v0: List<u64>) -> List<u64> {
    f400(f455(f400(f455(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f393(p1054: List<u64>) -> bool {
    let m1053 = p1054;
    {
        let mut m1053 = m1053;
        loop {
            let step = {
                let v0 = m1053;
                let m1052 = v0;
                match m1052.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f456(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1053 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f395() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f396() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f397(v0: List<u64>, p1057: List<List<u64>>) -> R<bool> {
    let m1056 = p1057;
    {
        let mut m1056 = m1056;
        loop {
            let step = {
                let v1 = m1056;
                let m1055 = v1;
                match m1055.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f390(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: List<u64>) -> Option<List<u64>> {
    let m1058 = v0;
    match m1058.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m1059 = v2;
            match m1059.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a1060 = v1;
                    let a1061 = 47u64;
                    if if nat_eq(a1060, a1061) {
                        let a1062 = v3;
                        let a1063 = 47u64;
                        nat_eq(a1062, a1063)
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

pub fn f399(v0: List<u64>) -> bool {
    let m1064 = v0;
    match m1064.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a1065 = v1;
            let a1066 = 58u64;
            if nat_eq(a1065, a1066) {
                let m1067 = v2.clone();
                match m1067.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f457(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f400(p1070: List<u64>, p1071: List<u64>) -> List<u64> {
    let m1069 = (p1070, p1071);
    {
        let mut m1069 = m1069;
        loop {
            let step = {
                let (v0, v1) = m1069;
                let m1068 = v0;
                match m1068.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f401(v0: Adt3, v1: Fn3, v2: u64, v3: List<Adt100>, v4: Adt124) -> R<Adt96> {
    f458(f441(v3.clone())?, f442(v0.clone(), v1, v2, Some(f443(v3, match v0 {
        Adt3::C0(_, _, _, _, h1072, _, _) => {
            h1072
        }
    }, f444(v4.clone()), match v4.clone() {
        Adt124::C0(_, h1073, _) => {
            h1073
        }
    })?), f445(), v4)?)
}

pub fn f402(v0: u64) -> bool {
    let a1074 = 48u64;
    let a1075 = v0;
    if if nat_le(a1074, a1075) {
        let a1076 = v0;
        let a1077 = 57u64;
        nat_le(a1076, a1077)
    } else {
        false
    } {
        true
    } else {
        let a1078 = 65u64;
        let a1079 = v0;
        if if nat_le(a1078, a1079) {
            let a1080 = v0;
            let a1081 = 90u64;
            nat_le(a1080, a1081)
        } else {
            false
        } {
            true
        } else {
            let a1082 = 97u64;
            let a1083 = v0;
            if if nat_le(a1082, a1083) {
                let a1084 = v0;
                let a1085 = 122u64;
                nat_le(a1084, a1085)
            } else {
                false
            } {
                true
            } else {
                let a1086 = v0;
                let a1087 = 33u64;
                if nat_eq(a1086, a1087) {
                    true
                } else {
                    let a1088 = v0;
                    let a1089 = 35u64;
                    if nat_eq(a1088, a1089) {
                        true
                    } else {
                        let a1090 = v0;
                        let a1091 = 36u64;
                        if nat_eq(a1090, a1091) {
                            true
                        } else {
                            let a1092 = v0;
                            let a1093 = 37u64;
                            if nat_eq(a1092, a1093) {
                                true
                            } else {
                                let a1094 = v0;
                                let a1095 = 38u64;
                                if nat_eq(a1094, a1095) {
                                    true
                                } else {
                                    let a1096 = v0;
                                    let a1097 = 39u64;
                                    if nat_eq(a1096, a1097) {
                                        true
                                    } else {
                                        let a1098 = v0;
                                        let a1099 = 42u64;
                                        if nat_eq(a1098, a1099) {
                                            true
                                        } else {
                                            let a1100 = v0;
                                            let a1101 = 43u64;
                                            if nat_eq(a1100, a1101) {
                                                true
                                            } else {
                                                let a1102 = v0;
                                                let a1103 = 45u64;
                                                if nat_eq(a1102, a1103) {
                                                    true
                                                } else {
                                                    let a1104 = v0;
                                                    let a1105 = 46u64;
                                                    if nat_eq(a1104, a1105) {
                                                        true
                                                    } else {
                                                        let a1106 = v0;
                                                        let a1107 = 94u64;
                                                        if nat_eq(a1106, a1107) {
                                                            true
                                                        } else {
                                                            let a1108 = v0;
                                                            let a1109 = 95u64;
                                                            if nat_eq(a1108, a1109) {
                                                                true
                                                            } else {
                                                                let a1110 = v0;
                                                                let a1111 = 96u64;
                                                                if nat_eq(a1110, a1111) {
                                                                    true
                                                                } else {
                                                                    let a1112 = v0;
                                                                    let a1113 = 124u64;
                                                                    if nat_eq(a1112, a1113) {
                                                                        true
                                                                    } else {
                                                                        let a1114 = v0;
                                                                        let a1115 = 126u64;
                                                                        nat_eq(a1114, a1115)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
    let a1116 = 48u64;
    let a1117 = v0;
    if if nat_le(a1116, a1117) {
        let a1118 = v0;
        let a1119 = 57u64;
        nat_le(a1118, a1119)
    } else {
        false
    } {
        true
    } else {
        let a1120 = 65u64;
        let a1121 = v0;
        if if nat_le(a1120, a1121) {
            let a1122 = v0;
            let a1123 = 90u64;
            nat_le(a1122, a1123)
        } else {
            false
        } {
            true
        } else {
            let a1124 = 97u64;
            let a1125 = v0;
            if if nat_le(a1124, a1125) {
                let a1126 = v0;
                let a1127 = 122u64;
                nat_le(a1126, a1127)
            } else {
                false
            } {
                true
            } else {
                let a1128 = v0;
                let a1129 = 95u64;
                if nat_eq(a1128, a1129) {
                    true
                } else {
                    let a1130 = v0;
                    let a1131 = 45u64;
                    if nat_eq(a1130, a1131) {
                        true
                    } else {
                        let a1132 = v0;
                        let a1133 = 46u64;
                        nat_eq(a1132, a1133)
                    }
                }
            }
        }
    }
}

pub fn f404() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f405(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>, v3: bool, v4: List<Str>, v5: Adt122, v6: List<Adt110>) -> R<Result<(List<Adt110>, Adt122), Adt102>> {
    let m1134 = f459(v0.clone(), v1, v2.clone());
    match m1134 {
        None => {
            let m1135 = f460(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1135 {
                Adt150::C0(v8) => {
                    Ok(Err::<(List<Adt110>, Adt122), Adt102>(v8))
                }
                Adt150::C1(v9) => {
                    Ok(Ok::<(List<Adt110>, Adt122), Adt102>((f461(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt110>, Adt122), Adt102>(v7))
        }
    }
}

pub fn f406(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt112>) -> R<Adt125> {
    let m1136 = f462(v1.clone(), v2.clone())?;
    match m1136 {
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
            f463(v0, v1, v2, v3)
        }
    }
}

pub fn f407(v0: List<Adt9>, v1: Adt9, v2: List<Adt112>, v3: u64, v4: List<Adt114>) -> Adt123 {
    let m1137 = v0;
    match m1137.uncons() {
        None => {
            let a1138 = v3;
            let a1139 = 1u64;
            if nat_eq(a1138, a1139) {
                Adt123::C0(Adt113::C0, v4)
            } else {
                Adt123::C0(Adt113::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt123::C0(Adt113::C0, List::cons(Adt114::C0(v5, v1.clone(), v2.clone()), List::cons(Adt114::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f408(v0: Option<Adt9>, v1: Adt9, v2: List<Adt112>, v3: List<Adt114>) -> Adt123 {
    let m1140 = v0;
    match m1140 {
        None => {
            Adt123::C0(Adt113::C0, v3)
        }
        Some(v4) => {
            Adt123::C0(Adt113::C0, List::cons(Adt114::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f409(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt112>) -> Adt125 {
    let m1141 = f309(Str::lit("anyOf"), v0);
    match m1141 {
        Adt116::C0 => {
            Adt125::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m1142 = f290(v3);
            match m1142 {
                None => {
                    Adt125::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1143 = v4;
                    match m1143.uncons() {
                        None => {
                            Adt125::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt125::C3(List::cons(Adt114::C0(v5, v1.clone(), v2.clone()), List::cons(Adt114::C1(v6, v1, v2), List::<Adt114>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f410(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt112>) -> Adt125 {
    let m1144 = f309(Str::lit("oneOf"), v0);
    match m1144 {
        Adt116::C0 => {
            Adt125::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m1145 = f290(v3);
            match m1145 {
                None => {
                    Adt125::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1146 = v4;
                    match m1146.uncons() {
                        None => {
                            Adt125::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt125::C3(List::cons(Adt114::C0(v5, v1.clone(), v2.clone()), List::cons(Adt114::C2(v6, v1, v2, 0u64), List::<Adt114>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f411(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt112>) -> Adt125 {
    let m1147 = f309(Str::lit("not"), v0);
    match m1147 {
        Adt116::C0 => {
            Adt125::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            Adt125::C3(List::cons(Adt114::C0(v3, v1, v2.clone()), List::cons(Adt114::C3(v2), List::<Adt114>::nil())))
        }
    }
}

pub fn f412(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt112>) -> Adt125 {
    let m1148 = f309(Str::lit("if"), v0.clone());
    match m1148 {
        Adt116::C0 => {
            Adt125::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            Adt125::C3(List::cons(Adt114::C0(v3, v1.clone(), v2.clone()), List::cons(Adt114::C4(f464(Str::lit("then"), v0.clone()), f464(Str::lit("else"), v0), v1, v2), List::<Adt114>::nil())))
        }
    }
}

pub fn f413(v0: List<Adt9>, v1: Adt9, v2: List<Adt112>, v3: List<Adt114>) -> Adt123 {
    let m1149 = v0;
    match m1149.uncons() {
        None => {
            Adt123::C0(Adt113::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt123::C0(Adt113::C0, List::cons(Adt114::C0(v4, v1.clone(), v2.clone()), List::cons(Adt114::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f414(v0: Adt112) -> R<Str> {
    let m1150 = v0;
    match m1150 {
        Adt112::C0(v1) => {
            f465(v1)
        }
        Adt112::C1(v2) => {
            let a1151 = f289(v2, 0i64)?;
            Ok(format_i64(a1151))
        }
    }
}

pub fn f415(v0: Str) -> R<bool> {
    let m1153 = f466(f29({
        let a1152 = v0;
        utf8_encode(a1152)
    })?)?;
    match m1153 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f416(v0: Adt41) -> bool {
    let m1155 = match v0 {
        Adt41::C0(_, _, h1154) => {
            h1154
        }
    };
    match m1155 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f417(v0: Adt53) -> R<Result<Adt36, Adt93>> {
    let m1157 = match v0.clone() {
        Adt53::C0(_, h1156) => {
            h1156
        }
    };
    match m1157.uncons() {
        None => {
            Ok(Ok::<Adt36, Adt93>(Adt36::C2(match v0 {
                Adt53::C0(h1158, _) => {
                    h1158
                }
            })))
        }
        Some((_, _)) => {
            if f467(match v0.clone() {
                Adt53::C0(_, h1159) => {
                    h1159
                }
            })? {
                Ok(Ok::<Adt36, Adt93>(Adt36::C3(v0)))
            } else {
                Ok(Err::<Adt36, Adt93>(Adt93::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f418(v0: Adt8, v1: Adt53) -> R<Result<Adt36, Adt93>> {
    let m1163 = {
        let m1162 = match v1.clone() {
            Adt53::C0(h1161, _) => {
                h1161
            }
        };
        match m1162 {
            Adt41::C0(_, h1160, _) => {
                h1160
            }
        }
    };
    match m1163 {
        None => {
            Ok(Err::<Adt36, Adt93>(Adt93::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1166 = f192(Adt9::C5(match v0.clone() {
                Adt8::C0(h1164) => {
                    h1164
                }
            }), Adt9::C5(match v0 {
                Adt8::C0(h1165) => {
                    h1165
                }
            }), v2)?;
            match m1166 {
                Adt111::C0 => {
                    f417(v1)
                }
                Adt111::C1(_, _) => {
                    Ok(Err::<Adt36, Adt93>(Adt93::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt111::C2(_) => {
                    Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt111::C3(_) => {
                    Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt111::C4 => {
                    Ok(Err::<Adt36, Adt93>(Adt93::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f419(p1172: List<u64>, p1173: List<u64>) -> Option<List<u64>> {
    let m1171 = (p1172, p1173);
    {
        let mut m1171 = m1171;
        loop {
            let step = {
                let (v0, v1) = m1171;
                let m1167 = v0;
                match m1167.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1168 = v1;
                        match m1168.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1169 = v2;
                                let a1170 = v4;
                                if nat_eq(a1169, a1170) {
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
                Ok(next) => { m1171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f420(v0: Str) -> R<List<u64>> {
    f29({
        let a1174 = v0;
        utf8_encode(a1174)
    })
}

pub fn f421() -> Str {
    Str::lit("test://template/")
}

pub fn f422(v0: List<u64>) -> List<u64> {
    f434(v0, List::<u64>::nil())
}

pub fn f423() -> Str {
    Str::lit("data")
}

pub fn f424(v0: List<u64>) -> bool {
    let m1175 = v0;
    match m1175.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f425(p1180: List<u64>) -> bool {
    let m1179 = p1180;
    {
        let mut m1179 = m1179;
        loop {
            let step = {
                let v0 = m1179;
                let m1176 = v0;
                match m1176.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1177 = v1;
                        let a1178 = 47u64;
                        if nat_eq(a1177, a1178) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1179 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(p1185: Bytes, p1186: List<Bytes>) -> Bytes {
    let m1184 = (p1185, p1186);
    {
        let mut m1184 = m1184;
        loop {
            let step = {
                let (v0, v1) = m1184;
                let m1181 = v1;
                match m1181.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1182 = v2;
                            let a1183 = v0;
                            append_bytes(a1182, a1183)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(p1189: List<Bytes>, p1190: List<u64>) -> List<Bytes> {
    let m1188 = (p1189, p1190);
    {
        let mut m1188 = m1188;
        loop {
            let step = {
                let (v0, v1) = m1188;
                let m1187 = v1;
                match m1187.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f468(f469(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f428(v0: Adt116, v1: Str) -> bool {
    let m1191 = v0;
    match m1191 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(v2) => {
            let m1192 = f287(v2);
            match m1192 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1193 = v3;
                    let a1194 = v1;
                    equal(a1193, a1194)
                }
            }
        }
    }
}

pub fn f429(p1200: List<u64>, p1201: List<u64>) -> bool {
    let m1199 = (p1200, p1201);
    {
        let mut m1199 = m1199;
        loop {
            let step = {
                let (v0, v1) = m1199;
                let m1195 = v0;
                match m1195.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1196 = v1;
                        match m1196.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1197 = v2;
                                let a1198 = v4;
                                if nat_eq(a1197, a1198) {
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
                Ok(next) => { m1199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f430(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a1205 = {
        let a1202 = v1;
        let a1203 = 8u64;
        nat_add(a1202, a1203)?
    };
    let a1206 = {
        let a1204 = v0.clone();
        length_bytes(a1204)
    };
    if nat_le(a1205, a1206) {
        Ok(Some(f470(f470(f470(f470(f470(f470(f470(f470(0i64, f62(v0.clone(), v1)?)?, f62(v0.clone(), {
            let a1207 = v1;
            let a1208 = 1u64;
            nat_add(a1207, a1208)?
        })?)?, f62(v0.clone(), {
            let a1209 = v1;
            let a1210 = 2u64;
            nat_add(a1209, a1210)?
        })?)?, f62(v0.clone(), {
            let a1211 = v1;
            let a1212 = 3u64;
            nat_add(a1211, a1212)?
        })?)?, f62(v0.clone(), {
            let a1213 = v1;
            let a1214 = 4u64;
            nat_add(a1213, a1214)?
        })?)?, f62(v0.clone(), {
            let a1215 = v1;
            let a1216 = 5u64;
            nat_add(a1215, a1216)?
        })?)?, f62(v0.clone(), {
            let a1217 = v1;
            let a1218 = 6u64;
            nat_add(a1217, a1218)?
        })?)?, f62(v0, {
            let a1219 = v1;
            let a1220 = 7u64;
            nat_add(a1219, a1220)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f431(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt2>> {
    let m1221 = f7(v0, v1)?;
    match m1221 {
        None => {
            Ok(Err::<(Str, u64), Adt2>(Adt2::C0(v1)))
        }
        Some(v2) => {
            let m1223 = f471(f29(match v2.clone() {
                Adt95::C0(h1222, _) => {
                    h1222
                }
            })?);
            match m1223 {
                None => {
                    Ok(Err::<(Str, u64), Adt2>(Adt2::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt2>((v3, match v2 {
                        Adt95::C0(_, h1224) => {
                            h1224
                        }
                    })))
                }
            }
        }
    }
}

pub fn f432(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt101, u64), Adt2>> {
    let m1226 = f431(v0.clone(), {
        let (_, h1225) = v2;
        h1225
    })?;
    match m1226 {
        Ok(v4) => {
            f472(v0, v1, {
                let (h1227, _) = v2;
                h1227
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt101, u64), Adt2>(v3))
        }
    }
}

pub fn f433(p1232: u64, p1233: List<Adt127>, p1234: List<u64>) -> R<Result<List<u64>, Adt117>> {
    let m1231 = (p1232, (p1233, p1234));
    {
        let mut m1231 = m1231;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1231;
                let m1228 = v0;
                if m1228 == 0 {
                    Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt117>>>(Ok(Err::<List<u64>, Adt117>(Adt117::C13)))
                } else {
                    let v3 = m1228 - 1;
                    let m1229 = v1;
                    match m1229.uncons() {
                        None => {
                            Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt117>>>(Ok(Ok::<List<u64>, Adt117>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m1230 = f473(v4, v5, v2)?;
                            match m1230 {
                                Adt152::C0(v6) => {
                                    Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt117>>>(Ok(Err::<List<u64>, Adt117>(v6)))
                                }
                                Adt152::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt117>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1231 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(p1237: List<u64>, p1238: List<u64>) -> List<u64> {
    let m1236 = (p1237, p1238);
    {
        let mut m1236 = m1236;
        loop {
            let step = {
                let (v0, v1) = m1236;
                let m1235 = v0;
                match m1235.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f435(v0: Adt109) -> Adt9 {
    let m1239 = v0;
    match m1239 {
        Adt109::C0(v1) => {
            Adt9::C3(v1)
        }
        Adt109::C1(v2) => {
            Adt9::C2(Adt10::C0(v2, 0i64))
        }
    }
}

pub fn f436(v0: Option<Adt9>) -> List<(Str, Adt9)> {
    let m1240 = v0;
    match m1240 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f437() -> i64 {
    -32700i64
}

pub fn f438() -> i64 {
    -32600i64
}

pub fn f439(v0: Adt119) -> Str {
    let m1241 = v0;
    match m1241 {
        Adt119::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt119::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt119::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt119::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt119::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt119::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt119::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt119::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt119::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f440(v0: Adt101, v1: bool, v2: Adt141) -> R<Adt104> {
    let m1243 = match v2.clone() {
        Adt141::C0(_, h1242) => {
            h1242
        }
    };
    match m1243 {
        Ok(v3) => {
            let m1244 = f474(v3.clone())?;
            match m1244 {
                None => {
                    Ok(f57())
                }
                Some(v4) => {
                    f475(v0, f476(match v3.clone() {
                        Adt142::C0(h1245, _, _) => {
                            h1245
                        }
                    }), f477(v1, f260(f478(), f479(match v2 {
                        Adt141::C0(h1246, _) => {
                            h1246
                        }
                    }, v4))?, f480(f481(match v3.clone() {
                        Adt142::C0(_, _, h1247) => {
                            h1247
                        }
                    }, List::<List<u64>>::nil())?, f482(f476(match v3 {
                        Adt142::C0(h1248, _, _) => {
                            h1248
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f260(f483(v5.clone()), f262(Some(match v2 {
                Adt141::C0(h1249, _) => {
                    h1249
                }
            }), f484(v5)?))
        }
    }
}

pub fn f441(p1254: List<Adt100>) -> R<bool> {
    let m1253 = p1254;
    {
        let mut m1253 = m1253;
        loop {
            let step = {
                let v0 = m1253;
                let m1250 = v0;
                match m1250.uncons() {
                    None => {
                        Err::<List<Adt100>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f390(match v1.clone() {
                            Adt100::C0(h1251, _) => {
                                h1251
                            }
                        }, f485())? {
                            if f486(f487(44u64, match v1 {
                                Adt100::C0(_, h1252) => {
                                    h1252
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt100>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt100>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt100>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: Adt3, v1: Fn3, v2: u64, v3: Option<Adt128>, v4: List<Str>, v5: Adt124) -> R<Adt141> {
    Ok(Adt141::C0(match v5.clone() {
        Adt124::C0(h1255, _, _) => {
            h1255
        }
    }, f488(v0, v1, v2, f489(v3, v4, v5)?)?))
}

pub fn f443(v0: List<Adt100>, v1: List<Adt15>, v2: Option<Str>, v3: Str) -> R<Adt128> {
    let v4: Adt147 = f490(Adt130::C0, f270(v0.clone(), f491())?);
    let v5: Adt147 = f490(Adt130::C1, f270(v0.clone(), f492())?);
    let v6: Adt147 = if f493(v3) {
        f490(Adt130::C2, f494(f270(v0.clone(), f495())?)?)
    } else {
        f496()
    };
    Ok(Adt128::C0(match v4.clone() {
        Adt147::C0(h1256, _) => {
            h1256
        }
    }, match v5.clone() {
        Adt147::C0(h1257, _) => {
            h1257
        }
    }, match v6.clone() {
        Adt147::C0(h1258, _) => {
            h1258
        }
    }, f497(match v4 {
        Adt147::C0(_, h1259) => {
            h1259
        }
    }, f497(match v5 {
        Adt147::C0(_, h1260) => {
            h1260
        }
    }, match v6 {
        Adt147::C0(_, h1261) => {
            h1261
        }
    })), f498(v0, v1, v2)?))
}

pub fn f444(v0: Adt124) -> Option<Str> {
    let a1263 = match v0.clone() {
        Adt124::C0(_, h1262, _) => {
            h1262
        }
    };
    let a1264 = Str::lit("tools/call");
    if equal(a1263, a1264) {
        let m1266 = match v0 {
            Adt124::C0(_, _, h1265) => {
                h1265
            }
        };
        match m1266 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1267 = f235(Str::lit("name"), v1);
                match m1267 {
                    Adt116::C0 => {
                        None::<Str>
                    }
                    Adt116::C1(v2) => {
                        f287(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f445() -> List<Str> {
    List::cons(f499(), List::<Str>::nil())
}

pub fn f446(v0: Adt8) -> R<Adt148> {
    let m1268 = f235(Str::lit("id"), v0);
    match m1268 {
        Adt116::C0 => {
            Ok(Adt148::C0)
        }
        Adt116::C1(v1) => {
            let m1269 = v1;
            match m1269 {
                Adt9::C0 => {
                    Ok(Adt148::C2)
                }
                Adt9::C1(_) => {
                    Ok(Adt148::C2)
                }
                Adt9::C2(v3) => {
                    let m1270 = f500(v3)?;
                    match m1270 {
                        None => {
                            Ok(Adt148::C2)
                        }
                        Some(v4) => {
                            Ok(Adt148::C1(Adt109::C1(v4)))
                        }
                    }
                }
                Adt9::C3(v2) => {
                    Ok(Adt148::C1(Adt109::C0(v2)))
                }
                Adt9::C4(_) => {
                    Ok(Adt148::C2)
                }
                Adt9::C5(_) => {
                    Ok(Adt148::C2)
                }
            }
        }
    }
}

pub fn f447(v0: Adt8) -> Adt119 {
    if f448(v0) {
        Adt119::C8
    } else {
        Adt119::C3
    }
}

pub fn f448(v0: Adt8) -> bool {
    if f501(Str::lit("result"), v0.clone()) {
        true
    } else {
        f501(Str::lit("error"), v0)
    }
}

pub fn f449(v0: Adt109, v1: Adt8) -> Result<Adt124, Adt118> {
    let m1271 = f235(Str::lit("jsonrpc"), v1.clone());
    match m1271 {
        Adt116::C0 => {
            f386(Some(v0), Adt119::C2)
        }
        Adt116::C1(v2) => {
            let m1272 = f287(v2);
            match m1272 {
                None => {
                    f386(Some(v0), Adt119::C2)
                }
                Some(v3) => {
                    let a1273 = v3;
                    let a1274 = f377();
                    if equal(a1273, a1274) {
                        f502(v0, v1)
                    } else {
                        f386(Some(v0), Adt119::C2)
                    }
                }
            }
        }
    }
}

pub fn f450(v0: u64, v1: u64) -> Adt133 {
    let a1275 = v1;
    let a1276 = 0u64;
    if nat_eq(a1275, a1276) {
        Adt133::C0
    } else {
        let a1277 = v1;
        let a1278 = 2u64;
        if nat_eq(a1277, a1278) {
            Adt133::C2(v0)
        } else {
            Adt133::C1({
                let a1282 = v0;
                let a1283 = {
                    let a1279 = v1;
                    let a1280 = 3u64;
                    let a1281 = 0u64;
                    nat_quot(a1279, a1280, a1281)
                };
                nat_sub(a1282, a1283)
            })
        }
    }
}

pub fn f451(p1311: Adt134, p1312: u64, p1313: u64, p1314: List<u64>) -> R<u64> {
    let m1310 = (p1311, (p1312, (p1313, p1314)));
    {
        let mut m1310 = m1310;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1310;
                let m1284 = v3;
                match m1284.uncons() {
                    None => {
                        let m1285 = v0;
                        match m1285 {
                            Adt134::C0 => {
                                Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt134::C1 => {
                                Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt134::C2 => {
                                Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt134::C3 => {
                                Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1286 = v0;
                        match m1286 {
                            Adt134::C0 => {
                                let a1287 = v4;
                                let a1288 = 128u64;
                                if nat_lt(a1287, a1288) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1289 = v4;
                                    let a1290 = 194u64;
                                    if nat_lt(a1289, a1290) {
                                        let v6: u64 = f503({
                                            let a1291 = List::cons(v4, v5);
                                            length_list(a1291)
                                        })?;
                                        Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1292 = v4;
                                        let a1293 = 224u64;
                                        if nat_lt(a1292, a1293) {
                                            Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1294 = v4;
                                            let a1295 = 224u64;
                                            if nat_eq(a1294, a1295) {
                                                Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1296 = v4;
                                                let a1297 = 237u64;
                                                if nat_eq(a1296, a1297) {
                                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1298 = v4;
                                                    let a1299 = 240u64;
                                                    if nat_lt(a1298, a1299) {
                                                        Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1300 = v4;
                                                        let a1301 = 240u64;
                                                        if nat_eq(a1300, a1301) {
                                                            Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1302 = v4;
                                                            let a1303 = 244u64;
                                                            if nat_eq(a1302, a1303) {
                                                                Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1304 = v4;
                                                                let a1305 = 244u64;
                                                                if nat_lt(a1304, a1305) {
                                                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f503({
                                                                        let a1306 = List::cons(v4, v5);
                                                                        length_list(a1306)
                                                                    })?;
                                                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt134::C1 => {
                                if f504(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f503({
                                        let a1307 = List::cons(v4, v5);
                                        length_list(a1307)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt134::C2 => {
                                if f504(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f503({
                                        let a1308 = List::cons(v4, v5);
                                        length_list(a1308)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt134::C3 => {
                                if f504(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f503({
                                        let a1309 = List::cons(v4, v5);
                                        length_list(a1309)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: Adt135, v1: u64) -> R<Result<Adt9, Adt117>> {
    let m1316 = match v0.clone() {
        Adt135::C0(h1315, _, _) => {
            h1315
        }
    };
    match m1316 {
        Adt136::C0 => {
            Ok(Err::<Adt9, Adt117>(Adt117::C3))
        }
        Adt136::C1 => {
            Ok(Err::<Adt9, Adt117>(Adt117::C3))
        }
        Adt136::C2 => {
            Ok(Err::<Adt9, Adt117>(Adt117::C3))
        }
        Adt136::C3 => {
            Ok(Err::<Adt9, Adt117>(Adt117::C3))
        }
        Adt136::C4 => {
            Ok(Err::<Adt9, Adt117>(Adt117::C3))
        }
        Adt136::C5 => {
            Ok(Err::<Adt9, Adt117>(Adt117::C3))
        }
        Adt136::C6(_, _, _) => {
            Ok(Err::<Adt9, Adt117>(Adt117::C3))
        }
        Adt136::C7(_, _) => {
            Ok(Err::<Adt9, Adt117>(Adt117::C3))
        }
        Adt136::C8(v3) => {
            let m1317 = f505(v3)?;
            match m1317 {
                Adt155::C0 => {
                    Ok(Err::<Adt9, Adt117>(Adt117::C7(v1)))
                }
                Adt155::C1 => {
                    Ok(Err::<Adt9, Adt117>(Adt117::C8(v1)))
                }
                Adt155::C2(v4) => {
                    let m1322 = {
                        let m1321 = f506(v4, match v0.clone() {
                            Adt135::C0(_, h1319, _) => {
                                h1319
                            }
                        }, match v0 {
                            Adt135::C0(_, _, h1320) => {
                                h1320
                            }
                        });
                        match m1321 {
                            Adt135::C0(h1318, _, _) => {
                                h1318
                            }
                        }
                    };
                    match m1322 {
                        Adt136::C0 => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C1 => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C2 => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C3 => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C4 => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C5 => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C6(_, _, _) => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C7(_, _) => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C8(_) => {
                            Ok(Err::<Adt9, Adt117>(Adt117::C3))
                        }
                        Adt136::C9(v5) => {
                            Ok(Ok::<Adt9, Adt117>(v5))
                        }
                    }
                }
            }
        }
        Adt136::C9(v2) => {
            Ok(Ok::<Adt9, Adt117>(v2))
        }
    }
}

pub fn f453(v0: Adt135, v1: u64, v2: u64) -> R<Adt149> {
    let m1324 = match v0.clone() {
        Adt135::C0(h1323, _, _) => {
            h1323
        }
    };
    match m1324 {
        Adt136::C0 => {
            if f507(v2) {
                Ok(Adt149::C1(v0))
            } else {
                f508(v0, v1, v2)
            }
        }
        Adt136::C1 => {
            if f507(v2) {
                Ok(Adt149::C1(v0))
            } else {
                let a1325 = v2;
                let a1326 = 93u64;
                if nat_eq(a1325, a1326) {
                    Ok(f509(v0, v1))
                } else {
                    f508(v0, v1, v2)
                }
            }
        }
        Adt136::C2 => {
            Ok(f510(v0, v1, v2))
        }
        Adt136::C3 => {
            if f507(v2) {
                Ok(Adt149::C1(v0))
            } else {
                let a1327 = v2;
                let a1328 = 125u64;
                if nat_eq(a1327, a1328) {
                    Ok(f511(v0, v1))
                } else {
                    let a1329 = v2;
                    let a1330 = 34u64;
                    if nat_eq(a1329, a1330) {
                        Ok(f512(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f513(Adt117::C2(v1)))
                    }
                }
            }
        }
        Adt136::C4 => {
            if f507(v2) {
                Ok(Adt149::C1(v0))
            } else {
                let a1331 = v2;
                let a1332 = 34u64;
                if nat_eq(a1331, a1332) {
                    Ok(f512(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f513(Adt117::C2(v1)))
                }
            }
        }
        Adt136::C5 => {
            if f507(v2) {
                Ok(Adt149::C1(v0))
            } else {
                let a1333 = v2;
                let a1334 = 58u64;
                if nat_eq(a1333, a1334) {
                    Ok(f512(v0, Adt136::C0))
                } else {
                    Ok(f513(Adt117::C2(v1)))
                }
            }
        }
        Adt136::C6(v3, v4, v5) => {
            f514(v0, v1, v2, v3, v4, v5)
        }
        Adt136::C7(v6, v7) => {
            f515(v0, v1, v2, v6, v7)
        }
        Adt136::C8(v8) => {
            f516(v0, v1, v2, v8)
        }
        Adt136::C9(_) => {
            Ok(f510(v0, v1, v2))
        }
    }
}

pub fn f454(v0: u64) -> R<u64> {
    let a1335 = 64u64;
    let a1336 = v0;
    if if nat_lt(a1335, a1336) {
        let a1337 = v0;
        let a1338 = 91u64;
        nat_lt(a1337, a1338)
    } else {
        false
    } {
        let a1339 = v0;
        let a1340 = 32u64;
        nat_add(a1339, a1340)
    } else {
        Ok(v0)
    }
}

pub fn f455(p1343: List<u64>) -> List<u64> {
    let m1342 = p1343;
    {
        let mut m1342 = m1342;
        loop {
            let step = {
                let v0 = m1342;
                let m1341 = v0;
                match m1341.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f517(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: u64) -> bool {
    let a1344 = v0;
    let a1345 = 9u64;
    if nat_eq(a1344, a1345) {
        true
    } else {
        let a1346 = 31u64;
        let a1347 = v0;
        if nat_lt(a1346, a1347) {
            let a1348 = v0;
            let a1349 = 127u64;
            nat_lt(a1348, a1349)
        } else {
            false
        }
    }
}

pub fn f457(p1352: List<u64>) -> bool {
    let m1351 = p1352;
    {
        let mut m1351 = m1351;
        loop {
            let step = {
                let v0 = m1351;
                let m1350 = v0;
                match m1350.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f518(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f458(v0: bool, v1: Adt141) -> R<Adt96> {
    let m1354 = f519(match v1.clone() {
        Adt141::C0(_, h1353) => {
            h1353
        }
    });
    match m1354 {
        None => {
            Ok(Adt96::C0(f56(f521(v0, v1)?)))
        }
        Some(v2) => {
            f520(v0, v1, v2)
        }
    }
}

pub fn f459(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>) -> Option<Adt102> {
    let m1355 = f522(f523(), v2.clone());
    match m1355 {
        None => {
            let m1356 = f524(f525(), v2.clone());
            match m1356 {
                None => {
                    f526(v0, v1, v2)
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

pub fn f460(v0: Str, v1: List<(Str, Adt9)>, v2: bool, v3: List<Str>, v4: Adt122) -> R<Adt150> {
    let m1357 = f309(Str::lit("x-mcp-header"), v1.clone());
    match m1357 {
        Adt116::C0 => {
            Ok(Adt150::C1(v4))
        }
        Adt116::C1(v5) => {
            f527(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f461(v0: List<(Str, Adt9)>, v1: bool, v2: List<Str>, v3: List<Adt110>) -> List<Adt110> {
    f528(Str::lit("properties"), v0.clone(), v1, v2, f528(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f529(Str::lit("items"), v0.clone(), f529(Str::lit("additionalProperties"), v0.clone(), f530(Str::lit("allOf"), v0.clone(), f530(Str::lit("anyOf"), v0.clone(), f530(Str::lit("oneOf"), v0.clone(), f529(Str::lit("not"), v0.clone(), f529(Str::lit("if"), v0.clone(), f529(Str::lit("then"), v0.clone(), f529(Str::lit("else"), v0, v3)))))))))))
}

pub fn f462(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt125> {
    Ok(f531(List::cons((Str::lit("type"), f532(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f533(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f534(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f535(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f535(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f535(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f535(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1358 = f536(v0.clone(), v1.clone())?;
        let a1359 = List::cons((Str::lit("required"), f537(v0, v1)), List::<(Str, Adt151)>::nil());
        append_list(a1358, a1359)
    })))))))))
}

pub fn f463(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt112>) -> R<Adt125> {
    Ok(f538(f539(v0, v1.clone(), v2.clone(), v3.clone()), f538(f540(v1.clone(), v2.clone(), v3.clone()), f538(f541(v1.clone(), v2.clone(), v3.clone()), f538(f542(v1.clone(), v2.clone(), v3.clone())?, f538(f543(v1.clone(), v2.clone(), v3.clone()), f538(f544(Adt115::C0, f409(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f538(f544(Adt115::C1, f410(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f538(f544(Adt115::C2, f411(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f544(Adt115::C3, f412(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f464(v0: Str, v1: List<(Str, Adt9)>) -> Option<Adt9> {
    let m1360 = f309(v0, v1);
    match m1360 {
        Adt116::C0 => {
            None::<Adt9>
        }
        Adt116::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f465(v0: Str) -> R<Str> {
    let m1363 = {
        let a1362 = f362(f257(), f434(f545(f29({
            let a1361 = v0.clone();
            utf8_encode(a1361)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1362)
    };
    match m1363 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f466(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1365 = f546({
        let a1364 = v0.clone();
        length_list(a1364)
    }, v0, List::<u64>::nil())?;
    match m1365 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f434(v1, List::<u64>::nil())))
        }
    }
}

pub fn f467(v0: List<Adt54>) -> R<bool> {
    let m1366 = v0;
    match m1366.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f547(v2, match v1 {
                Adt54::C0(h1367, _, _) => {
                    h1367
                }
            })
        }
    }
}

pub fn f468(p1376: Bytes, p1377: List<Bytes>) -> List<Bytes> {
    let m1375 = (p1376, p1377);
    {
        let mut m1375 = m1375;
        loop {
            let step = {
                let (v0, v1) = m1375;
                let m1368 = v1;
                match m1368.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1371 = {
                            let a1369 = v2.clone();
                            length_bytes(a1369)
                        };
                        let a1372 = {
                            let a1370 = v0.clone();
                            length_bytes(a1370)
                        };
                        if nat_eq(a1371, a1372) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1373 = v2;
                                let a1374 = v0;
                                append_bytes(a1373, a1374)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: u64) -> Bytes {
    let m1381 = {
        let a1378 = f548();
        let a1379 = v0;
        let a1380 = 1u64;
        slice_bytes(a1378, a1379, a1380)
    };
    match m1381 {
        None => {
            f257()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f470(v0: i64, v1: u64) -> R<i64> {
    f359({
        let a1382 = v0;
        let a1383 = 256i64;
        int_mul(a1382, a1383)?
    }, f289(v1, 0i64)?)
}

pub fn f471(v0: List<u64>) -> Option<Str> {
    let a1384 = f362(f257(), v0);
    utf8_decode(a1384)
}

pub fn f472(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt101, u64), Adt2>> {
    let m1386 = f7(v0, {
        let (_, h1385) = v3.clone();
        h1385
    })?;
    match m1386 {
        None => {
            Ok(Err::<(Adt101, u64), Adt2>(Adt2::C0({
                let (_, h1387) = v3;
                h1387
            })))
        }
        Some(v4) => {
            Ok(f549(v1, v2, {
                let (h1388, _) = v3;
                h1388
            }, v4))
        }
    }
}

pub fn f473(v0: Adt127, v1: List<Adt127>, v2: List<u64>) -> R<Adt152> {
    let m1389 = v0;
    match m1389 {
        Adt127::C0(v3) => {
            Ok(Adt152::C1(v1, f434(v3, v2)))
        }
        Adt127::C1(v4, v5) => {
            f550(v4, v5, v1, v2)
        }
        Adt127::C2(v6, v7) => {
            let m1390 = v7;
            match m1390.uncons() {
                None => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt152::C1(List::cons(Adt127::C1(v6, v8), List::cons(Adt127::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt127::C3(v10, v11) => {
            let m1391 = v11.clone();
            match m1391.uncons() {
                None => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt127::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt127::C4(v14, v15) => {
            let m1392 = v15;
            match m1392.uncons() {
                None => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt152::C1(List::cons(Adt127::C0({
                        let a1394 = f551({
                            let (h1393, _) = v16;
                            h1393
                        })?;
                        let a1395 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1394, a1395)
                    }), List::cons(Adt127::C1(v14, {
                        let (_, h1396) = v16;
                        h1396
                    }), List::cons(Adt127::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt127::C5(v18, v19) => {
            let m1397 = v19.clone();
            match m1397.uncons() {
                None => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt127::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f474(v0: Adt142) -> R<Option<Adt8>> {
    let m1399 = f552(match v0.clone() {
        Adt142::C0(h1398, _, _) => {
            h1398
        }
    })?;
    match m1399 {
        None => {
            Ok(None::<Adt8>)
        }
        Some(v1) => {
            Ok(Some(Adt8::C0({
                let a1402 = v1;
                let a1403 = {
                    let m1401 = f553(v0);
                    match m1401 {
                        Adt8::C0(h1400) => {
                            h1400
                        }
                    }
                };
                append_list(a1402, a1403)
            })))
        }
    }
}

pub fn f475(v0: Adt101, v1: Option<Adt91>, v2: Adt104) -> R<Adt104> {
    let m1404 = v1;
    match m1404 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a1406 = match v2.clone() {
                Adt104::C0(h1405, _, _, _, _) => {
                    h1405
                }
            };
            let a1407 = f478();
            if nat_eq(a1406, a1407) {
                Ok(f554(f555(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f476(v0: Adt36) -> Option<Adt91> {
    let m1408 = v0;
    match m1408 {
        Adt36::C0(_) => {
            None::<Adt91>
        }
        Adt36::C1(_) => {
            None::<Adt91>
        }
        Adt36::C2(_) => {
            None::<Adt91>
        }
        Adt36::C3(_) => {
            None::<Adt91>
        }
        Adt36::C4(_) => {
            None::<Adt91>
        }
        Adt36::C5(_) => {
            None::<Adt91>
        }
        Adt36::C6(_) => {
            None::<Adt91>
        }
        Adt36::C7(_) => {
            None::<Adt91>
        }
        Adt36::C8(_) => {
            None::<Adt91>
        }
        Adt36::C9(_) => {
            None::<Adt91>
        }
        Adt36::C10(_) => {
            None::<Adt91>
        }
        Adt36::C11 => {
            None::<Adt91>
        }
        Adt36::C12(_) => {
            None::<Adt91>
        }
        Adt36::C13(v13) => {
            Some(Adt91::C0(match v13 {
                Adt86::C0(_, _, h1409) => {
                    h1409
                }
            }, Adt92::C0, false))
        }
        Adt36::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f477(v0: bool, v1: Adt104, v2: Option<List<List<u64>>>) -> Adt104 {
    let m1410 = v2;
    match m1410 {
        None => {
            f57()
        }
        Some(v3) => {
            f556(v0, v1, f176(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f478() -> u64 {
    200u64
}

pub fn f479(v0: Adt109, v1: Adt8) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(f377())), List::cons((Str::lit("id"), f435(v0)), List::cons((Str::lit("result"), Adt9::C5(match v1 {
        Adt8::C0(h1411) => {
            h1411
        }
    })), List::<(Str, Adt9)>::nil()))))
}

pub fn f480(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
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
                    Some(f176(v3, v2))
                }
            }
        }
    }
}

pub fn f481(p1417: List<Adt144>, p1418: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1416 = (p1417, p1418);
    {
        let mut m1416 = m1416;
        loop {
            let step = {
                let (v0, v1) = m1416;
                let m1414 = v0;
                match m1414.uncons() {
                    None => {
                        Err::<(List<Adt144>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1415 = f557(v2)?;
                        match m1415 {
                            None => {
                                Err::<(List<Adt144>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt144>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
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

pub fn f482(v0: Option<Adt91>) -> R<Option<List<List<u64>>>> {
    let m1419 = v0;
    match m1419 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f558(v1)
        }
    }
}

pub fn f483(v0: Adt145) -> u64 {
    let m1420 = v0;
    match m1420 {
        Adt145::C0(_) => {
            f330()
        }
        Adt145::C1(_, _) => {
            f330()
        }
        Adt145::C2(_) => {
            f330()
        }
        Adt145::C3(_) => {
            f559()
        }
        Adt145::C4(v6) => {
            f560(v6)
        }
    }
}

pub fn f484(v0: Adt145) -> R<Adt84> {
    Ok(Adt84::C0(f561(v0.clone()), f562(v0.clone()), f563(v0)?))
}

pub fn f485() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f486(p1423: List<List<u64>>) -> R<bool> {
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
                        if f564(v1)? {
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

pub fn f487(v0: u64, p1428: List<u64>, p1429: List<u64>, p1430: List<List<u64>>) -> List<List<u64>> {
    let m1427 = (p1428, (p1429, p1430));
    {
        let mut m1427 = m1427;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1427;
                let m1424 = v1;
                match m1424.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f176(List::cons(f400(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1425 = v4;
                        let a1426 = v0;
                        if nat_eq(a1425, a1426) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f400(v2, List::<u64>::nil()), v3))))
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

pub fn f488(v0: Adt3, v1: Fn3, v2: u64, v3: Result<Adt153, Adt145>) -> R<Result<Adt142, Adt145>> {
    let m1431 = v3;
    match m1431 {
        Ok(v5) => {
            f565(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt142, Adt145>(v4))
        }
    }
}

pub fn f489(v0: Option<Adt128>, v1: List<Str>, v2: Adt124) -> R<Result<Adt153, Adt145>> {
    Ok(f566(f567(match v2.clone() {
        Adt124::C0(_, h1432, _) => {
            h1432
        }
    }), f568(v0, v1, v2)?))
}

pub fn f490(v0: Adt130, v1: Adt120) -> Adt147 {
    let m1433 = v1;
    match m1433 {
        Adt120::C0 => {
            f496()
        }
        Adt120::C1 => {
            Adt147::C0(None::<Str>, Some(Adt129::C6(v0)))
        }
        Adt120::C2 => {
            Adt147::C0(None::<Str>, Some(Adt129::C7(v0)))
        }
        Adt120::C3(v2) => {
            let m1434 = f471(v2);
            match m1434 {
                None => {
                    Adt147::C0(None::<Str>, Some(Adt129::C6(v0)))
                }
                Some(v3) => {
                    Adt147::C0(Some(v3), None::<Adt129>)
                }
            }
        }
    }
}

pub fn f491() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f492() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f493(v0: Str) -> bool {
    let m1435 = f569(v0);
    match m1435 {
        Adt159::C0 => {
            false
        }
        Adt159::C1 => {
            true
        }
        Adt159::C2 => {
            true
        }
        Adt159::C3 => {
            true
        }
    }
}

pub fn f494(v0: Adt120) -> R<Adt120> {
    let m1436 = v0.clone();
    match m1436 {
        Adt120::C0 => {
            Ok(v0)
        }
        Adt120::C1 => {
            Ok(v0)
        }
        Adt120::C2 => {
            Ok(v0)
        }
        Adt120::C3(v1) => {
            let m1437 = f570(v1);
            match m1437 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1438 = f466(v2)?;
                    match m1438 {
                        None => {
                            Ok(Adt120::C1)
                        }
                        Some(v3) => {
                            Ok(Adt120::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f495() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f496() -> Adt147 {
    Adt147::C0(None::<Str>, None::<Adt129>)
}

pub fn f497(v0: Option<Adt129>, v1: Option<Adt129>) -> Option<Adt129> {
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

pub fn f498(v0: List<Adt100>, v1: List<Adt15>, v2: Option<Str>) -> R<List<Adt131>> {
    Ok(f571(f572(v0, v1, v2, List::<Adt131>::nil())?, List::<Adt131>::nil()))
}

pub fn f499() -> Str {
    Str::lit("2026-07-28")
}

pub fn f500(v0: Adt10) -> R<Option<i64>> {
    let m1442 = f573(40u64, match v0.clone() {
        Adt10::C0(h1440, _) => {
            h1440
        }
    }, match v0 {
        Adt10::C0(_, h1441) => {
            h1441
        }
    })?;
    match m1442 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f574(v1))
        }
    }
}

pub fn f501(v0: Str, v1: Adt8) -> bool {
    f575(f235(v0, v1))
}

pub fn f502(v0: Adt109, v1: Adt8) -> Result<Adt124, Adt118> {
    let m1443 = f235(Str::lit("method"), v1.clone());
    match m1443 {
        Adt116::C0 => {
            f386(Some(v0), f447(v1))
        }
        Adt116::C1(v2) => {
            let m1444 = f287(v2);
            match m1444 {
                None => {
                    f386(Some(v0), Adt119::C4)
                }
                Some(v3) => {
                    f576(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f503(v0: u64) -> R<u64> {
    let a1447 = {
        let a1445 = v0;
        let a1446 = 3u64;
        nat_mul(a1445, a1446)?
    };
    let a1448 = 1u64;
    nat_add(a1447, a1448)
}

pub fn f504(v0: u64, v1: u64, v2: u64) -> bool {
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

pub fn f505(v0: Adt138) -> R<Adt155> {
    if f577(match v0.clone() {
        Adt138::C0(_, h1453, _, _, _, _, _) => {
            h1453
        }
    }) {
        let v1: i64 = {
            let a1459 = if match v0.clone() {
                Adt138::C0(_, _, _, _, _, h1454, _) => {
                    h1454
                }
            } {
                let a1456 = match v0.clone() {
                    Adt138::C0(_, _, _, _, _, _, h1455) => {
                        h1455
                    }
                };
                int_neg(a1456)?
            } else {
                match v0.clone() {
                    Adt138::C0(_, _, _, _, _, _, h1457) => {
                        h1457
                    }
                }
            };
            let a1460 = match v0.clone() {
                Adt138::C0(_, _, _, _, h1458, _, _) => {
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
            Ok(Adt155::C1)
        } else {
            Ok(Adt155::C2(Adt9::C2(Adt10::C0(if match v0.clone() {
                Adt138::C0(h1467, _, _, _, _, _, _) => {
                    h1467
                }
            } {
                let a1469 = match v0 {
                    Adt138::C0(_, _, h1468, _, _, _, _) => {
                        h1468
                    }
                };
                int_neg(a1469)?
            } else {
                match v0 {
                    Adt138::C0(_, _, h1470, _, _, _, _) => {
                        h1470
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt155::C0)
    }
}

pub fn f506(v0: Adt9, v1: List<Adt140>, v2: u64) -> Adt135 {
    let m1471 = v1.clone();
    match m1471.uncons() {
        None => {
            Adt135::C0(Adt136::C9(v0), List::<Adt140>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1472 = v3;
            match m1472 {
                Adt140::C0(v5) => {
                    Adt135::C0(Adt136::C2, List::cons(Adt140::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt140::C1(v6, v7) => {
                    let m1473 = v7;
                    match m1473 {
                        None => {
                            Adt135::C0(Adt136::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt135::C0(Adt136::C2, List::cons(Adt140::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f507(v0: u64) -> bool {
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

pub fn f508(v0: Adt135, v1: u64, v2: u64) -> R<Adt149> {
    let a1482 = v2;
    let a1483 = 34u64;
    if nat_eq(a1482, a1483) {
        Ok(f512(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), false)))
    } else {
        let a1484 = v2;
        let a1485 = 123u64;
        if nat_eq(a1484, a1485) {
            f578(v0, Adt140::C1(List::<(Str, Adt9)>::nil(), None::<Str>), Adt136::C3, v1)
        } else {
            let a1486 = v2;
            let a1487 = 91u64;
            if nat_eq(a1486, a1487) {
                f578(v0, Adt140::C0(List::<Adt9>::nil()), Adt136::C1, v1)
            } else {
                let a1488 = v2;
                let a1489 = 116u64;
                if nat_eq(a1488, a1489) {
                    Ok(f512(v0, Adt136::C7(0u64, 1u64)))
                } else {
                    let a1490 = v2;
                    let a1491 = 102u64;
                    if nat_eq(a1490, a1491) {
                        Ok(f512(v0, Adt136::C7(1u64, 1u64)))
                    } else {
                        let a1492 = v2;
                        let a1493 = 110u64;
                        if nat_eq(a1492, a1493) {
                            Ok(f512(v0, Adt136::C7(2u64, 1u64)))
                        } else {
                            let a1494 = v2;
                            let a1495 = 45u64;
                            if nat_eq(a1494, a1495) {
                                Ok(f512(v0, f579(true, Adt139::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1496 = v2;
                                let a1497 = 48u64;
                                if nat_eq(a1496, a1497) {
                                    Ok(f512(v0, f579(false, Adt139::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f504(v2, 49u64, 57u64) {
                                        Ok(f512(v0, f579(false, Adt139::C2, f580({
                                            let a1498 = v2;
                                            let a1499 = 48u64;
                                            nat_sub(a1498, a1499)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f513(Adt117::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f509(v0: Adt135, v1: u64) -> Adt149 {
    let m1501 = match v0.clone() {
        Adt135::C0(_, h1500, _) => {
            h1500
        }
    };
    match m1501.uncons() {
        None => {
            f513(Adt117::C2(v1))
        }
        Some((v2, v3)) => {
            let m1502 = v2;
            match m1502 {
                Adt140::C0(v4) => {
                    Adt149::C1(f506(Adt9::C4(f581(v4, List::<Adt9>::nil())), v3, {
                        let a1504 = match v0 {
                            Adt135::C0(_, _, h1503) => {
                                h1503
                            }
                        };
                        let a1505 = 1u64;
                        nat_sub(a1504, a1505)
                    }))
                }
                Adt140::C1(_, _) => {
                    f513(Adt117::C2(v1))
                }
            }
        }
    }
}

pub fn f510(v0: Adt135, v1: u64, v2: u64) -> Adt149 {
    let m1507 = match v0.clone() {
        Adt135::C0(h1506, _, _) => {
            h1506
        }
    };
    match m1507 {
        Adt136::C0 => {
            f513(Adt117::C2(v1))
        }
        Adt136::C1 => {
            f513(Adt117::C2(v1))
        }
        Adt136::C2 => {
            if f507(v2) {
                Adt149::C1(v0)
            } else {
                let a1508 = v2;
                let a1509 = 44u64;
                if nat_eq(a1508, a1509) {
                    f582(v0, v1)
                } else {
                    let a1510 = v2;
                    let a1511 = 93u64;
                    if nat_eq(a1510, a1511) {
                        f509(v0, v1)
                    } else {
                        let a1512 = v2;
                        let a1513 = 125u64;
                        if nat_eq(a1512, a1513) {
                            f511(v0, v1)
                        } else {
                            f513(Adt117::C2(v1))
                        }
                    }
                }
            }
        }
        Adt136::C3 => {
            f513(Adt117::C2(v1))
        }
        Adt136::C4 => {
            f513(Adt117::C2(v1))
        }
        Adt136::C5 => {
            f513(Adt117::C2(v1))
        }
        Adt136::C6(_, _, _) => {
            f513(Adt117::C2(v1))
        }
        Adt136::C7(_, _) => {
            f513(Adt117::C2(v1))
        }
        Adt136::C8(_) => {
            f513(Adt117::C2(v1))
        }
        Adt136::C9(_) => {
            if f507(v2) {
                Adt149::C1(v0)
            } else {
                f513(Adt117::C10(v1))
            }
        }
    }
}

pub fn f511(v0: Adt135, v1: u64) -> Adt149 {
    let m1515 = match v0.clone() {
        Adt135::C0(_, h1514, _) => {
            h1514
        }
    };
    match m1515.uncons() {
        None => {
            f513(Adt117::C2(v1))
        }
        Some((v2, v3)) => {
            let m1516 = v2;
            match m1516 {
                Adt140::C0(_) => {
                    f513(Adt117::C2(v1))
                }
                Adt140::C1(v5, _) => {
                    Adt149::C1(f506(Adt9::C5(f583(v5, List::<(Str, Adt9)>::nil())), v3, {
                        let a1518 = match v0 {
                            Adt135::C0(_, _, h1517) => {
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

pub fn f512(v0: Adt135, v1: Adt136) -> Adt149 {
    Adt149::C1(f584(v0, v1))
}

pub fn f513(v0: Adt117) -> Adt149 {
    Adt149::C0(v0)
}

pub fn f514(v0: Adt135, v1: u64, v2: u64, v3: Adt137, v4: List<u64>, v5: bool) -> R<Adt149> {
    let m1520 = v3;
    match m1520 {
        Adt137::C0 => {
            let a1521 = v2;
            let a1522 = 34u64;
            if nat_eq(a1521, a1522) {
                Ok(f585(v0, v1, v4, v5))
            } else {
                let a1523 = v2;
                let a1524 = 92u64;
                if nat_eq(a1523, a1524) {
                    Ok(f512(v0, Adt136::C6(Adt137::C1, v4, v5)))
                } else {
                    let a1525 = v2;
                    let a1526 = 32u64;
                    if nat_lt(a1525, a1526) {
                        Ok(f513(Adt117::C4(v1)))
                    } else {
                        Ok(f512(v0, Adt136::C6(Adt137::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt137::C1 => {
            let a1527 = v2;
            let a1528 = 117u64;
            if nat_eq(a1527, a1528) {
                Ok(f512(v0, Adt136::C6(Adt137::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1529 = f586(v2);
                let a1530 = 0u64;
                if nat_eq(a1529, a1530) {
                    Ok(f513(Adt117::C5(v1)))
                } else {
                    Ok(f512(v0, Adt136::C6(Adt137::C0, List::cons(f586(v2), v4), v5)))
                }
            }
        }
        Adt137::C2(v6, v7, v8) => {
            let a1531 = 15u64;
            let a1532 = f587(v2);
            if nat_lt(a1531, a1532) {
                Ok(f513(Adt117::C5(v1)))
            } else {
                let a1535 = {
                    let a1533 = v6;
                    let a1534 = 1u64;
                    nat_add(a1533, a1534)?
                };
                let a1536 = 4u64;
                if nat_eq(a1535, a1536) {
                    f588(v0, v1, {
                        let a1539 = {
                            let a1537 = v7;
                            let a1538 = 16u64;
                            nat_mul(a1537, a1538)?
                        };
                        let a1540 = f587(v2);
                        nat_add(a1539, a1540)?
                    }, v8, v4, v5)
                } else {
                    Ok(f512(v0, Adt136::C6(Adt137::C2({
                        let a1541 = v6;
                        let a1542 = 1u64;
                        nat_add(a1541, a1542)?
                    }, {
                        let a1545 = {
                            let a1543 = v7;
                            let a1544 = 16u64;
                            nat_mul(a1543, a1544)?
                        };
                        let a1546 = f587(v2);
                        nat_add(a1545, a1546)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt137::C3(v9) => {
            let a1547 = v2;
            let a1548 = 92u64;
            if nat_eq(a1547, a1548) {
                Ok(f512(v0, Adt136::C6(Adt137::C4(v9), v4, v5)))
            } else {
                Ok(f513(Adt117::C6(v1)))
            }
        }
        Adt137::C4(v10) => {
            let a1549 = v2;
            let a1550 = 117u64;
            if nat_eq(a1549, a1550) {
                Ok(f512(v0, Adt136::C6(Adt137::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f513(Adt117::C6(v1)))
            }
        }
    }
}

pub fn f515(v0: Adt135, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt149> {
    let a1551 = v2;
    let a1552 = f589(v3, v4);
    if nat_eq(a1551, a1552) {
        let a1556 = {
            let a1553 = v4;
            let a1554 = 1u64;
            nat_add(a1553, a1554)?
        };
        let a1557 = {
            let a1555 = f590(v3);
            length_list(a1555)
        };
        if nat_eq(a1556, a1557) {
            Ok(f591(v0, f592(v3)))
        } else {
            Ok(f512(v0, Adt136::C7(v3, {
                let a1558 = v4;
                let a1559 = 1u64;
                nat_add(a1558, a1559)?
            })))
        }
    } else {
        Ok(f513(Adt117::C2(v1)))
    }
}

pub fn f516(v0: Adt135, v1: u64, v2: u64, v3: Adt138) -> R<Adt149> {
    let m1561 = match v3.clone() {
        Adt138::C0(_, h1560, _, _, _, _, _) => {
            h1560
        }
    };
    match m1561 {
        Adt139::C0 => {
            let a1562 = v2;
            let a1563 = 48u64;
            if nat_eq(a1562, a1563) {
                Ok(f593(v0, v3, Adt139::C1))
            } else {
                if f504(v2, 49u64, 57u64) {
                    f594(v0, v1, v2, v3, false)
                } else {
                    Ok(f513(Adt117::C7(v1)))
                }
            }
        }
        Adt139::C1 => {
            let a1564 = v2;
            let a1565 = 46u64;
            if nat_eq(a1564, a1565) {
                Ok(f593(v0, v3, Adt139::C3))
            } else {
                if f595(v2) {
                    Ok(f593(v0, v3, Adt139::C5))
                } else {
                    if f596(v2) {
                        Ok(f513(Adt117::C7(v1)))
                    } else {
                        f597(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt139::C2 => {
            if f596(v2) {
                f594(v0, v1, v2, v3, false)
            } else {
                let a1566 = v2;
                let a1567 = 46u64;
                if nat_eq(a1566, a1567) {
                    Ok(f593(v0, v3, Adt139::C3))
                } else {
                    if f595(v2) {
                        Ok(f593(v0, v3, Adt139::C5))
                    } else {
                        f597(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt139::C3 => {
            if f596(v2) {
                f594(v0, v1, v2, v3, true)
            } else {
                Ok(f513(Adt117::C7(v1)))
            }
        }
        Adt139::C4 => {
            if f596(v2) {
                f594(v0, v1, v2, v3, true)
            } else {
                if f595(v2) {
                    Ok(f593(v0, v3, Adt139::C5))
                } else {
                    f597(v0, v1, v2, v3)
                }
            }
        }
        Adt139::C5 => {
            let a1568 = v2;
            let a1569 = 43u64;
            if nat_eq(a1568, a1569) {
                Ok(f512(v0, f579(match v3.clone() {
                    Adt138::C0(h1570, _, _, _, _, _, _) => {
                        h1570
                    }
                }, Adt139::C6, match v3.clone() {
                    Adt138::C0(_, _, h1571, _, _, _, _) => {
                        h1571
                    }
                }, match v3.clone() {
                    Adt138::C0(_, _, _, h1572, _, _, _) => {
                        h1572
                    }
                }, match v3 {
                    Adt138::C0(_, _, _, _, h1573, _, _) => {
                        h1573
                    }
                }, false, 0i64)))
            } else {
                let a1574 = v2;
                let a1575 = 45u64;
                if nat_eq(a1574, a1575) {
                    Ok(f512(v0, f579(match v3.clone() {
                        Adt138::C0(h1576, _, _, _, _, _, _) => {
                            h1576
                        }
                    }, Adt139::C6, match v3.clone() {
                        Adt138::C0(_, _, h1577, _, _, _, _) => {
                            h1577
                        }
                    }, match v3.clone() {
                        Adt138::C0(_, _, _, h1578, _, _, _) => {
                            h1578
                        }
                    }, match v3 {
                        Adt138::C0(_, _, _, _, h1579, _, _) => {
                            h1579
                        }
                    }, true, 0i64)))
                } else {
                    if f596(v2) {
                        f598(v0, v1, v2, Adt138::C0(match v3.clone() {
                            Adt138::C0(h1580, _, _, _, _, _, _) => {
                                h1580
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, h1581, _, _, _, _, _) => {
                                h1581
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, _, h1582, _, _, _, _) => {
                                h1582
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, _, _, h1583, _, _, _) => {
                                h1583
                            }
                        }, match v3 {
                            Adt138::C0(_, _, _, _, h1584, _, _) => {
                                h1584
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f513(Adt117::C7(v1)))
                    }
                }
            }
        }
        Adt139::C6 => {
            if f596(v2) {
                f598(v0, v1, v2, v3.clone(), match v3 {
                    Adt138::C0(_, _, _, _, _, h1585, _) => {
                        h1585
                    }
                })
            } else {
                Ok(f513(Adt117::C7(v1)))
            }
        }
        Adt139::C7 => {
            if f596(v2) {
                f598(v0, v1, v2, v3.clone(), match v3 {
                    Adt138::C0(_, _, _, _, _, h1586, _) => {
                        h1586
                    }
                })
            } else {
                f597(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f517(v0: u64) -> bool {
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

pub fn f518(v0: u64) -> bool {
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

pub fn f519(v0: Result<Adt142, Adt145>) -> Option<Adt94> {
    let m1595 = v0;
    match m1595 {
        Ok(_) => {
            None::<Adt94>
        }
        Err(v2) => {
            let m1596 = v2;
            match m1596 {
                Adt145::C0(_) => {
                    None::<Adt94>
                }
                Adt145::C1(_, _) => {
                    None::<Adt94>
                }
                Adt145::C2(_) => {
                    None::<Adt94>
                }
                Adt145::C3(_) => {
                    None::<Adt94>
                }
                Adt145::C4(v8) => {
                    f599(v8)
                }
            }
        }
    }
}

pub fn f520(v0: bool, v1: Adt141, v2: Adt94) -> R<Adt96> {
    if f600(match v2.clone() {
        Adt94::C0(h1597, _) => {
            h1597
        }
    }) {
        Ok(Adt96::C1(v2))
    } else {
        Ok(Adt96::C0(f56(f521(v0, Adt141::C0(match v1 {
            Adt141::C0(h1598, _) => {
                h1598
            }
        }, Err::<Adt142, Adt145>(f601())))?)))
    }
}

pub fn f521(v0: bool, v1: Adt141) -> R<Adt104> {
    f440(Adt101::C0(0i64, Str::lit(""), Str::lit(""), f257()), v0, v1)
}

pub fn f522(p1602: List<Str>, v1: List<(Str, Adt9)>) -> Option<Str> {
    let m1601 = p1602;
    {
        let mut m1601 = m1601;
        loop {
            let step = {
                let v0 = m1601;
                let m1599 = v0;
                match m1599.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1600 = f309(v2.clone(), v1.clone());
                        match m1600 {
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
                Ok(next) => { m1601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f524(p1609: List<(Str, u64)>, v1: List<(Str, Adt9)>) -> Option<Str> {
    let m1608 = p1609;
    {
        let mut m1608 = m1608;
        loop {
            let step = {
                let v0 = m1608;
                let m1603 = v0;
                match m1603.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1605 = f309({
                            let (h1604, _) = v2.clone();
                            h1604
                        }, v1.clone());
                        match m1605 {
                            Adt116::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt116::C1(v4) => {
                                if f602({
                                    let (_, h1606) = v2;
                                    h1606
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1607, _) = v2;
                                        h1607
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f526(v0: Str, v1: Adt9, v2: List<(Str, Adt9)>) -> Option<Adt102> {
    let m1610 = f309(Str::lit("$ref"), v2);
    match m1610 {
        Adt116::C0 => {
            None::<Adt102>
        }
        Adt116::C1(v3) => {
            let m1611 = f287(v3);
            match m1611 {
                None => {
                    None::<Adt102>
                }
                Some(v4) => {
                    let m1612 = f603(v1, v4.clone());
                    match m1612 {
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

pub fn f527(v0: Str, v1: List<(Str, Adt9)>, v2: Adt9, v3: bool, v4: List<Str>, v5: Adt122) -> R<Adt150> {
    if v3 {
        let m1613 = f287(v2);
        match m1613 {
            None => {
                Ok(Adt150::C0(Adt102::C7(v0)))
            }
            Some(v6) => {
                f604(v0, v1, v6.clone(), f189(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt150::C0(Adt102::C10(v0)))
    }
}

pub fn f528(v0: Str, v1: List<(Str, Adt9)>, v2: bool, v3: List<Str>, v4: List<Adt110>) -> List<Adt110> {
    let m1614 = f309(v0.clone(), v1);
    match m1614 {
        Adt116::C0 => {
            v4
        }
        Adt116::C1(v5) => {
            let m1615 = f236(v5);
            match m1615 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1616 = v0;
                    let a1617 = Str::lit("properties");
                    if equal(a1616, a1617) {
                        f605(match v6 {
                            Adt8::C0(h1618) => {
                                h1618
                            }
                        }, v2, v3, v4)
                    } else {
                        f606(match v6 {
                            Adt8::C0(h1619) => {
                                h1619
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f529(v0: Str, v1: List<(Str, Adt9)>, v2: List<Adt110>) -> List<Adt110> {
    let m1620 = f309(v0, v1);
    match m1620 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            List::cons(Adt110::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f530(v0: Str, v1: List<(Str, Adt9)>, v2: List<Adt110>) -> List<Adt110> {
    let m1621 = f309(v0, v1);
    match m1621 {
        Adt116::C0 => {
            v2
        }
        Adt116::C1(v3) => {
            let m1622 = f290(v3);
            match m1622 {
                None => {
                    v2
                }
                Some(v4) => {
                    f607(v4, v2)
                }
            }
        }
    }
}

pub fn f531(p1629: List<(Str, Adt151)>) -> Adt125 {
    let m1628 = p1629;
    {
        let mut m1628 = m1628;
        loop {
            let step = {
                let v0 = m1628;
                let m1623 = v0;
                match m1623.uncons() {
                    None => {
                        Err::<List<(Str, Adt151)>, Adt125>(Adt125::C3(List::<Adt114>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1625 = {
                            let (_, h1624) = v1;
                            h1624
                        };
                        match m1625 {
                            Adt151::C0 => {
                                Ok::<List<(Str, Adt151)>, Adt125>(v2)
                            }
                            Adt151::C1 => {
                                Err::<List<(Str, Adt151)>, Adt125>(Adt125::C0({
                                    let (h1626, _) = v1;
                                    h1626
                                }))
                            }
                            Adt151::C2 => {
                                Err::<List<(Str, Adt151)>, Adt125>(Adt125::C1({
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

pub fn f532(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt151> {
    let m1630 = f309(Str::lit("type"), v0);
    match m1630 {
        Adt116::C0 => {
            Ok(Adt151::C0)
        }
        Adt116::C1(v2) => {
            let m1631 = f287(v2.clone());
            match m1631 {
                None => {
                    let m1632 = f290(v2);
                    match m1632 {
                        None => {
                            Ok(Adt151::C2)
                        }
                        Some(v4) => {
                            f610(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f608(v3.clone()) {
                        if f609(v3, v1)? {
                            Ok(Adt151::C0)
                        } else {
                            Ok(Adt151::C1)
                        }
                    } else {
                        Ok(Adt151::C2)
                    }
                }
            }
        }
    }
}

pub fn f533(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt151> {
    let m1633 = f309(Str::lit("enum"), v0);
    match m1633 {
        Adt116::C0 => {
            Ok(Adt151::C0)
        }
        Adt116::C1(v2) => {
            let m1634 = f290(v2);
            match m1634 {
                None => {
                    Ok(Adt151::C2)
                }
                Some(v3) => {
                    f611(v3, v1)
                }
            }
        }
    }
}

pub fn f534(v0: List<(Str, Adt9)>, v1: Adt9) -> R<Adt151> {
    let m1635 = f309(Str::lit("const"), v0);
    match m1635 {
        Adt116::C0 => {
            Ok(Adt151::C0)
        }
        Adt116::C1(v2) => {
            let m1636 = f612(v2, v1)?;
            match m1636 {
                Adt160::C0 => {
                    Ok(Adt151::C0)
                }
                Adt160::C1 => {
                    Ok(Adt151::C1)
                }
                Adt160::C2 => {
                    Ok(Adt151::C2)
                }
            }
        }
    }
}

pub fn f535(v0: Str, v1: u64, v2: List<(Str, Adt9)>, v3: Adt9) -> R<Adt151> {
    let m1637 = f309(v0, v2);
    match m1637 {
        Adt116::C0 => {
            Ok(Adt151::C0)
        }
        Adt116::C1(v4) => {
            let m1638 = f613(v4);
            match m1638 {
                None => {
                    Ok(Adt151::C2)
                }
                Some(v5) => {
                    let m1639 = f613(v3);
                    match m1639 {
                        None => {
                            Ok(Adt151::C0)
                        }
                        Some(v6) => {
                            if f614(v1, f615(v6, v5)?) {
                                Ok(Adt151::C0)
                            } else {
                                Ok(Adt151::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f536(v0: List<(Str, Adt9)>, v1: Adt9) -> R<List<(Str, Adt151)>> {
    let m1640 = v1;
    match m1640 {
        Adt9::C0 => {
            Ok(List::<(Str, Adt151)>::nil())
        }
        Adt9::C1(_) => {
            Ok(List::<(Str, Adt151)>::nil())
        }
        Adt9::C2(_) => {
            Ok(List::<(Str, Adt151)>::nil())
        }
        Adt9::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f616(Str::lit("minLength"), true, v0.clone(), f617(v4.clone())?)?), List::cons((Str::lit("maxLength"), f616(Str::lit("maxLength"), false, v0, f617(v4)?)?), List::<(Str, Adt151)>::nil())))
        }
        Adt9::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f616(Str::lit("minItems"), true, v0.clone(), f618(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f616(Str::lit("maxItems"), false, v0, f618(v5, 0u64)?)?), List::<(Str, Adt151)>::nil())))
        }
        Adt9::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f616(Str::lit("minProperties"), true, v0.clone(), f619(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f616(Str::lit("maxProperties"), false, v0, f619(v6, 0u64)?)?), List::<(Str, Adt151)>::nil())))
        }
    }
}

pub fn f537(v0: List<(Str, Adt9)>, v1: Adt9) -> Adt151 {
    let m1641 = f309(Str::lit("required"), v0);
    match m1641 {
        Adt116::C0 => {
            Adt151::C0
        }
        Adt116::C1(v2) => {
            let m1642 = f290(v2);
            match m1642 {
                None => {
                    Adt151::C2
                }
                Some(v3) => {
                    let m1643 = f236(v1);
                    match m1643 {
                        None => {
                            Adt151::C0
                        }
                        Some(v4) => {
                            f620(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f538(v0: Adt125, v1: Adt125) -> Adt125 {
    let m1644 = v0.clone();
    match m1644 {
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
            let m1645 = v1.clone();
            match m1645 {
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
                        let a1646 = v5;
                        let a1647 = v9;
                        append_list(a1646, a1647)
                    })
                }
            }
        }
    }
}

pub fn f539(v0: Adt9, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt112>) -> Adt125 {
    let m1648 = f309(Str::lit("$ref"), v1);
    match m1648 {
        Adt116::C0 => {
            Adt125::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v4) => {
            let m1649 = f287(v4);
            match m1649 {
                None => {
                    Adt125::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1650 = f603(v0, v5.clone());
                    match m1650 {
                        None => {
                            Adt125::C2(v5)
                        }
                        Some(v6) => {
                            Adt125::C3(List::cons(Adt114::C0(v6, v2, v3), List::<Adt114>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f540(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt112>) -> Adt125 {
    let m1651 = f309(Str::lit("properties"), v0);
    match m1651 {
        Adt116::C0 => {
            Adt125::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m1652 = f236(v3);
            match m1652 {
                None => {
                    Adt125::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1653 = f236(v1);
                    match m1653 {
                        None => {
                            Adt125::C3(List::<Adt114>::nil())
                        }
                        Some(v5) => {
                            Adt125::C3(f621(match v4 {
                                Adt8::C0(h1654) => {
                                    h1654
                                }
                            }, match v5 {
                                Adt8::C0(h1655) => {
                                    h1655
                                }
                            }, v2, List::<Adt114>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f541(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt112>) -> Adt125 {
    let m1656 = f309(Str::lit("additionalProperties"), v0.clone());
    match m1656 {
        Adt116::C0 => {
            Adt125::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m1657 = f236(v1);
            match m1657 {
                None => {
                    Adt125::C3(List::<Adt114>::nil())
                }
                Some(v4) => {
                    let m1658 = f360(v3.clone());
                    match m1658 {
                        None => {
                            Adt125::C3(f624(match v4 {
                                Adt8::C0(h1660) => {
                                    h1660
                                }
                            }, f623(v0), v3, v2, List::<Adt114>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt125::C3(List::<Adt114>::nil())
                            } else {
                                if f622(match v4 {
                                    Adt8::C0(h1659) => {
                                        h1659
                                    }
                                }, f623(v0)) {
                                    Adt125::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt125::C3(List::<Adt114>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f542(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt112>) -> R<Adt125> {
    let m1661 = f309(Str::lit("items"), v0);
    match m1661 {
        Adt116::C0 => {
            Ok(Adt125::C3(List::<Adt114>::nil()))
        }
        Adt116::C1(v3) => {
            let m1662 = f290(v1);
            match m1662 {
                None => {
                    Ok(Adt125::C3(List::<Adt114>::nil()))
                }
                Some(v4) => {
                    Ok(Adt125::C3(f625(v4, 0u64, v3, v2, List::<Adt114>::nil())?))
                }
            }
        }
    }
}

pub fn f543(v0: List<(Str, Adt9)>, v1: Adt9, v2: List<Adt112>) -> Adt125 {
    let m1663 = f309(Str::lit("allOf"), v0);
    match m1663 {
        Adt116::C0 => {
            Adt125::C3(List::<Adt114>::nil())
        }
        Adt116::C1(v3) => {
            let m1664 = f290(v3);
            match m1664 {
                None => {
                    Adt125::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1665 = v4.clone();
                    match m1665.uncons() {
                        None => {
                            Adt125::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt125::C3(f626(v4, v1, v2, List::<Adt114>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f544(v0: Adt115, v1: Adt125, v2: List<(Str, Adt9)>, v3: Adt9, v4: List<Adt112>) -> Adt125 {
    let m1666 = v1.clone();
    match m1666 {
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
            let m1667 = v8;
            match m1667.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt125::C3(List::cons(Adt114::C5(v0, Adt9::C5(v2), v3, v4), List::<Adt114>::nil()))
                }
            }
        }
    }
}

pub fn f545(p1674: List<u64>, p1675: List<u64>) -> List<u64> {
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

pub fn f546(p1684: u64, p1685: List<u64>, p1686: List<u64>) -> R<Option<List<u64>>> {
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
                                                    let m1682 = f627(v6, v8, v10, v12, f628(v13.clone()))?;
                                                    match m1682 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f434(v14, v2))))
                                                        }
                                                    }
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

pub fn f547(p1691: List<Adt54>, p1692: Adt10) -> R<bool> {
    let m1690 = (p1691, p1692);
    {
        let mut m1690 = m1690;
        loop {
            let step = {
                let (v0, v1) = m1690;
                let m1687 = v0;
                match m1687.uncons() {
                    None => {
                        Err::<(List<Adt54>, Adt10), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f629(f615(match v2.clone() {
                            Adt54::C0(h1688, _, _) => {
                                h1688
                            }
                        }, v1)?) {
                            Ok::<(List<Adt54>, Adt10), R<bool>>((v3, match v2 {
                                Adt54::C0(h1689, _, _) => {
                                    h1689
                                }
                            }))
                        } else {
                            Err::<(List<Adt54>, Adt10), R<bool>>(Ok(false))
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

pub fn f548() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f549(v0: i64, v1: Str, v2: Str, v3: Adt95) -> Result<(Adt101, u64), Adt2> {
    Ok::<(Adt101, u64), Adt2>((Adt101::C0(v0, v1, v2, match v3.clone() {
        Adt95::C0(h1693, _) => {
            h1693
        }
    }), match v3 {
        Adt95::C0(_, h1694) => {
            h1694
        }
    }))
}

pub fn f550(v0: u64, v1: Adt9, v2: List<Adt127>, v3: List<u64>) -> R<Adt152> {
    let m1695 = v1;
    match m1695 {
        Adt9::C0 => {
            Ok(Adt152::C1(v2, f434(f630(2u64), v3)))
        }
        Adt9::C1(v4) => {
            Ok(Adt152::C1(v2, f434(if v4 {
                f630(0u64)
            } else {
                f630(1u64)
            }, v3)))
        }
        Adt9::C2(v5) => {
            if f631(v5.clone())? {
                Ok(Adt152::C1(v2, f434(f632(v5)?, v3)))
            } else {
                Ok(Adt152::C0(Adt117::C11))
            }
        }
        Adt9::C3(v6) => {
            Ok(Adt152::C1(v2, f434(f551(v6)?, v3)))
        }
        Adt9::C4(v7) => {
            let a1696 = v0;
            let a1697 = f633();
            if nat_lt(a1696, a1697) {
                Ok(Adt152::C1(f634(91u64, Adt127::C2({
                    let a1698 = v0;
                    let a1699 = 1u64;
                    nat_add(a1698, a1699)?
                }, v7), v2), v3))
            } else {
                Ok(Adt152::C0(Adt117::C12))
            }
        }
        Adt9::C5(v8) => {
            let a1700 = v0;
            let a1701 = f633();
            if nat_lt(a1700, a1701) {
                Ok(Adt152::C1(f634(123u64, Adt127::C4({
                    let a1702 = v0;
                    let a1703 = 1u64;
                    nat_add(a1702, a1703)?
                }, v8), v2), v3))
            } else {
                Ok(Adt152::C0(Adt117::C12))
            }
        }
    }
}

pub fn f551(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1705 = f434(f635(f29({
            let a1704 = v0;
            utf8_encode(a1704)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1706 = List::cons(34u64, List::<u64>::nil());
        append_list(a1705, a1706)
    }))
}

pub fn f552(v0: Adt36) -> R<Option<List<(Str, Adt9)>>> {
    let m1707 = v0;
    match m1707 {
        Adt36::C0(v1) => {
            Ok(Some(f636(v1)?))
        }
        Adt36::C1(v2) => {
            Ok(Some(f637(v2)?))
        }
        Adt36::C2(v3) => {
            f638(f639(v3)?)
        }
        Adt36::C3(v4) => {
            f638(f639(match v4 {
                Adt53::C0(h1708, _) => {
                    h1708
                }
            })?)
        }
        Adt36::C4(v5) => {
            Ok(Some(f640(v5)?))
        }
        Adt36::C5(v6) => {
            Ok(Some(f641(v6)?))
        }
        Adt36::C6(v7) => {
            Ok(Some(f642(v7)?))
        }
        Adt36::C7(v8) => {
            Ok(Some(f643(v8)?))
        }
        Adt36::C8(v9) => {
            f638(f644(v9)?)
        }
        Adt36::C9(v10) => {
            Ok(Some(f645(v10)?))
        }
        Adt36::C10(_) => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Adt36::C11 => {
            Ok(Some(List::<(Str, Adt9)>::nil()))
        }
        Adt36::C12(v12) => {
            f638(f646(v12)?)
        }
        Adt36::C13(v13) => {
            f638(f647({
                let m1711 = match v13 {
                    Adt86::C0(_, _, h1710) => {
                        h1710
                    }
                };
                match m1711 {
                    Adt88::C0(h1709, _, _) => {
                        h1709
                    }
                }
            }))
        }
        Adt36::C14(v14) => {
            Ok(f648(v14))
        }
    }
}

pub fn f553(v0: Adt142) -> Adt8 {
    Adt8::C0(List::cons((Str::lit("resultType"), Adt9::C3(f649(f650(match v0.clone() {
        Adt142::C0(h1712, _, _) => {
            h1712
        }
    })))), List::cons((Str::lit("_meta"), Adt9::C5(f651({
        let m1715 = match v0 {
            Adt142::C0(_, h1714, _) => {
                h1714
            }
        };
        match m1715 {
            Adt143::C0(h1713, _) => {
                h1713
            }
        }
    }))), List::<(Str, Adt9)>::nil())))
}

pub fn f554(v0: Option<Option<List<u64>>>, v1: Adt104) -> Adt104 {
    let m1716 = v0;
    match m1716 {
        None => {
            f57()
        }
        Some(v2) => {
            Adt104::C0(match v1.clone() {
                Adt104::C0(h1717, _, _, _, _) => {
                    h1717
                }
            }, match v1.clone() {
                Adt104::C0(_, h1718, _, _, _) => {
                    h1718
                }
            }, match v1.clone() {
                Adt104::C0(_, _, h1719, _, _) => {
                    h1719
                }
            }, match v1 {
                Adt104::C0(_, _, _, h1720, _) => {
                    h1720
                }
            }, v2)
        }
    }
}

pub fn f555(v0: Adt101, v1: Adt91) -> R<Option<Option<List<u64>>>> {
    let m1722 = f652(match v0.clone() {
        Adt101::C0(_, _, _, h1721) => {
            h1721
        }
    })?;
    match m1722 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1725 = f653(f654(match v1 {
                Adt91::C0(h1723, _, _) => {
                    h1723
                }
            }, v2, match v0 {
                Adt101::C0(h1724, _, _, _) => {
                    h1724
                }
            })?)?;
            match m1725 {
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

pub fn f556(v0: bool, v1: Adt104, v2: List<List<u64>>) -> Adt104 {
    if f655(v0, match v1.clone() {
        Adt104::C0(h1726, _, _, _, _) => {
            h1726
        }
    }, v2.clone()) {
        f656(match v1.clone() {
            Adt104::C0(h1727, _, _, _, _) => {
                h1727
            }
        }, {
            let a1729 = v2;
            let a1730 = List::cons(match v1 {
                Adt104::C0(_, _, _, h1728, _) => {
                    h1728
                }
            }, List::<List<u64>>::nil());
            append_list(a1729, a1730)
        })
    } else {
        v1
    }
}

pub fn f557(v0: Adt144) -> R<Option<List<u64>>> {
    let m1731 = f657(f658(v0))?;
    match m1731 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1732 = f324(v1)?;
            match m1732 {
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

pub fn f558(v0: Adt91) -> R<Option<List<List<u64>>>> {
    if f659(v0.clone()) {
        let m1736 = f660({
            let m1735 = match v0 {
                Adt91::C0(h1734, _, _) => {
                    h1734
                }
            };
            match m1735 {
                Adt88::C0(h1733, _, _) => {
                    h1733
                }
            }
        });
        match m1736 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1737 = f657(v1)?;
                match m1737 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1738 = f324(v2)?;
                        match m1738 {
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

pub fn f559() -> u64 {
    404u64
}

pub fn f560(v0: Adt93) -> u64 {
    let m1739 = v0;
    match m1739 {
        Adt93::C0(_) => {
            f330()
        }
        Adt93::C1(_) => {
            f330()
        }
        Adt93::C2(_) => {
            f330()
        }
        Adt93::C3(_) => {
            f330()
        }
        Adt93::C4(_) => {
            f330()
        }
        Adt93::C5(_) => {
            f330()
        }
        Adt93::C6(_) => {
            f107()
        }
        Adt93::C7(_) => {
            f661()
        }
        Adt93::C8(_) => {
            f330()
        }
        Adt93::C9(_) => {
            f107()
        }
        Adt93::C10 => {
            f330()
        }
        Adt93::C11 => {
            f330()
        }
        Adt93::C12(_) => {
            f107()
        }
        Adt93::C13 => {
            f107()
        }
        Adt93::C14 => {
            f107()
        }
        Adt93::C15(_) => {
            f107()
        }
    }
}

pub fn f561(v0: Adt145) -> i64 {
    let m1740 = v0;
    match m1740 {
        Adt145::C0(_) => {
            f662()
        }
        Adt145::C1(_, _) => {
            f663()
        }
        Adt145::C2(_) => {
            f664()
        }
        Adt145::C3(_) => {
            f665()
        }
        Adt145::C4(v6) => {
            f666(v6)
        }
    }
}

pub fn f562(v0: Adt145) -> Str {
    let m1741 = v0;
    match m1741 {
        Adt145::C0(v1) => {
            f667(v1)
        }
        Adt145::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt145::C2(v4) => {
            f668(v4)
        }
        Adt145::C3(_) => {
            Str::lit("Method not found")
        }
        Adt145::C4(v6) => {
            let m1742 = v6;
            match m1742 {
                Adt93::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt93::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt93::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt93::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt93::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt93::C5(_) => {
                    Str::lit("Invalid params: invalid cursor")
                }
                Adt93::C6(_) => {
                    Str::lit("Internal error")
                }
                Adt93::C7(_) => {
                    Str::lit("Rate limit exceeded: too many tool calls")
                }
                Adt93::C8(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt93::C9(_) => {
                    Str::lit("Internal error")
                }
                Adt93::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt93::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt93::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt93::C13 => {
                    Str::lit("Internal error")
                }
                Adt93::C14 => {
                    Str::lit("Internal error")
                }
                Adt93::C15(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f563(v0: Adt145) -> R<Option<Adt9>> {
    let m1743 = v0;
    match m1743 {
        Adt145::C0(_) => {
            Ok(None::<Adt9>)
        }
        Adt145::C1(v2, v3) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("supported"), Adt9::C4(f669(v3))), List::cons((Str::lit("requested"), Adt9::C3(v2)), List::<(Str, Adt9)>::nil())))))
        }
        Adt145::C2(_) => {
            Ok(None::<Adt9>)
        }
        Adt145::C3(_) => {
            Ok(None::<Adt9>)
        }
        Adt145::C4(v6) => {
            let m1744 = v6;
            match m1744 {
                Adt93::C0(_) => {
                    Ok(None::<Adt9>)
                }
                Adt93::C1(_) => {
                    Ok(None::<Adt9>)
                }
                Adt93::C2(v9) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(v9)), List::<(Str, Adt9)>::nil()))))
                }
                Adt93::C3(_) => {
                    Ok(None::<Adt9>)
                }
                Adt93::C4(v11) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("reason"), Adt9::C3(v11)), List::<(Str, Adt9)>::nil()))))
                }
                Adt93::C5(_) => {
                    Ok(None::<Adt9>)
                }
                Adt93::C6(_) => {
                    Ok(None::<Adt9>)
                }
                Adt93::C7(v14) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("limit"), Adt9::C2(Adt10::C0(f289(v14, 0i64)?, 0i64))), List::<(Str, Adt9)>::nil()))))
                }
                Adt93::C8(v15) => {
                    Ok(Some(Adt9::C5(List::cons((Str::lit("requiredCapabilities"), f670(v15)), List::<(Str, Adt9)>::nil()))))
                }
                Adt93::C9(_) => {
                    Ok(None::<Adt9>)
                }
                Adt93::C10 => {
                    Ok(None::<Adt9>)
                }
                Adt93::C11 => {
                    Ok(None::<Adt9>)
                }
                Adt93::C12(_) => {
                    Ok(None::<Adt9>)
                }
                Adt93::C13 => {
                    Ok(None::<Adt9>)
                }
                Adt93::C14 => {
                    Ok(None::<Adt9>)
                }
                Adt93::C15(_) => {
                    Ok(None::<Adt9>)
                }
            }
        }
    }
}

pub fn f564(v0: List<u64>) -> R<bool> {
    let m1745 = f487(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1745.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f671(v2)? {
                Ok(false)
            } else {
                f390(f392(v1), f672())
            }
        }
    }
}

pub fn f565(v0: Adt3, v1: Fn3, v2: u64, v3: Adt153) -> R<Result<Adt142, Adt145>> {
    f673(v0.clone(), v1, v2, v3.clone(), f674(match v0 {
        Adt3::C0(_, _, h1746, _, _, _, _) => {
            h1746
        }
    }, match v3 {
        Adt153::C0(_, h1747, _, _) => {
            h1747
        }
    }))
}

pub fn f566(v0: Adt156, v1: Adt157) -> Result<Adt153, Adt145> {
    let m1748 = v1;
    match m1748 {
        Adt157::C0(v3) => {
            f675(v3, v0)
        }
        Adt157::C1(v2) => {
            Err::<Adt153, Adt145>(v2)
        }
    }
}

pub fn f567(v0: Str) -> Adt156 {
    let a1749 = v0.clone();
    let a1750 = Str::lit("server/discover");
    if equal(a1749, a1750) {
        Adt156::C1(Adt154::C0)
    } else {
        let a1751 = v0.clone();
        let a1752 = Str::lit("tools/list");
        if equal(a1751, a1752) {
            Adt156::C1(Adt154::C1)
        } else {
            let a1753 = v0.clone();
            let a1754 = Str::lit("tools/call");
            if equal(a1753, a1754) {
                Adt156::C1(Adt154::C2)
            } else {
                let a1755 = v0.clone();
                let a1756 = Str::lit("resources/list");
                if equal(a1755, a1756) {
                    Adt156::C1(Adt154::C3)
                } else {
                    let a1757 = v0.clone();
                    let a1758 = Str::lit("resources/read");
                    if equal(a1757, a1758) {
                        Adt156::C1(Adt154::C4)
                    } else {
                        let a1759 = v0.clone();
                        let a1760 = Str::lit("resources/templates/list");
                        if equal(a1759, a1760) {
                            Adt156::C1(Adt154::C5)
                        } else {
                            let a1761 = v0.clone();
                            let a1762 = Str::lit("prompts/list");
                            if equal(a1761, a1762) {
                                Adt156::C1(Adt154::C6)
                            } else {
                                let a1763 = v0.clone();
                                let a1764 = Str::lit("prompts/get");
                                if equal(a1763, a1764) {
                                    Adt156::C1(Adt154::C7)
                                } else {
                                    let a1765 = v0.clone();
                                    let a1766 = Str::lit("completion/complete");
                                    if equal(a1765, a1766) {
                                        Adt156::C1(Adt154::C8)
                                    } else {
                                        let a1767 = v0.clone();
                                        let a1768 = Str::lit("tasks/get");
                                        if equal(a1767, a1768) {
                                            Adt156::C1(Adt154::C9)
                                        } else {
                                            let a1769 = v0.clone();
                                            let a1770 = Str::lit("tasks/update");
                                            if equal(a1769, a1770) {
                                                Adt156::C1(Adt154::C10)
                                            } else {
                                                let a1771 = v0;
                                                let a1772 = Str::lit("tasks/cancel");
                                                if equal(a1771, a1772) {
                                                    Adt156::C1(Adt154::C11)
                                                } else {
                                                    Adt156::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f568(v0: Option<Adt128>, v1: List<Str>, v2: Adt124) -> R<Adt157> {
    f676(v0, v1, v2.clone(), f677(match v2 {
        Adt124::C0(_, _, h1773) => {
            h1773
        }
    }))
}

pub fn f569(v0: Str) -> Adt159 {
    let m1774 = f567(v0);
    match m1774 {
        Adt156::C0 => {
            Adt159::C0
        }
        Adt156::C1(v1) => {
            f678(v1)
        }
    }
}

pub fn f570(v0: List<u64>) -> Option<List<u64>> {
    let m1775 = f679(f680(), v0);
    match m1775 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1776 = f679(f681(), f400(v1, List::<u64>::nil()));
            match m1776 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f400(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f571(p1779: List<Adt131>, p1780: List<Adt131>) -> List<Adt131> {
    let m1778 = (p1779, p1780);
    {
        let mut m1778 = m1778;
        loop {
            let step = {
                let (v0, v1) = m1778;
                let m1777 = v0;
                match m1777.uncons() {
                    None => {
                        Err::<(List<Adt131>, List<Adt131>), List<Adt131>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt131>, List<Adt131>), List<Adt131>>((v3, List::cons(v2, v1)))
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

pub fn f572(v0: List<Adt100>, p1787: List<Adt15>, v2: Option<Str>, p1788: List<Adt131>) -> R<List<Adt131>> {
    let m1786 = (p1787, p1788);
    {
        let mut m1786 = m1786;
        loop {
            let step = {
                let (v1, v3) = m1786;
                let m1781 = v1;
                match m1781.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt131>), R<List<Adt131>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f682(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt15>, List<Adt131>), R<List<Adt131>>>((v5, List::cons(Adt131::C0(match v4.clone() {
                                Adt15::C0(_, h1782, _, _) => {
                                    h1782
                                }
                            }, match v4.clone() {
                                Adt15::C0(_, _, h1783, _) => {
                                    h1783
                                }
                            }, match v4.clone() {
                                Adt15::C0(_, _, _, h1784) => {
                                    h1784
                                }
                            }, f683(v0.clone(), match v4 {
                                Adt15::C0(_, h1785, _, _) => {
                                    h1785
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt15>, List<Adt131>), R<List<Adt131>>>((v5, v3))
                        }
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

pub fn f573(p1804: u64, p1805: i64, p1806: i64) -> R<Option<i64>> {
    let m1803 = (p1804, (p1805, p1806));
    {
        let mut m1803 = m1803;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1803;
                let m1789 = v0;
                if m1789 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1789 - 1;
                    if f684(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1792 = {
                            let a1790 = v2;
                            let a1791 = 0i64;
                            compare(a1790, a1791)
                        };
                        if match m1792 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f684({
                                let a1793 = v1;
                                let a1794 = 10i64;
                                let a1795 = 0i64;
                                int_rem(a1793, a1794, a1795)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1796 = v1;
                                    let a1797 = 10i64;
                                    let a1798 = 0i64;
                                    int_quot(a1796, a1797, a1798)?
                                }, f359(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1799 = v1;
                                let a1800 = 10i64;
                                int_mul(a1799, a1800)?
                            }, {
                                let a1801 = v2;
                                let a1802 = 1i64;
                                int_sub(a1801, a1802)?
                            })))
                        }
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

pub fn f574(v0: i64) -> Option<i64> {
    let m1809 = {
        let a1807 = 9007199254740991i64;
        let a1808 = v0;
        compare(a1807, a1808)
    };
    if if match m1809 {
        Ordering::Less => {
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
        let m1812 = {
            let a1810 = v0;
            let a1811 = -9007199254740991i64;
            compare(a1810, a1811)
        };
        match m1812 {
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

pub fn f575(v0: Adt116) -> bool {
    let m1813 = v0;
    match m1813 {
        Adt116::C0 => {
            false
        }
        Adt116::C1(_) => {
            true
        }
    }
}

pub fn f576(v0: Adt109, v1: Str, v2: Adt8) -> Result<Adt124, Adt118> {
    let m1814 = f235(Str::lit("params"), v2);
    match m1814 {
        Adt116::C0 => {
            Ok::<Adt124, Adt118>(Adt124::C0(v0, v1, None::<Adt8>))
        }
        Adt116::C1(v3) => {
            let m1815 = f236(v3);
            match m1815 {
                None => {
                    f386(Some(v0), Adt119::C6)
                }
                Some(v4) => {
                    Ok::<Adt124, Adt118>(Adt124::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f577(v0: Adt139) -> bool {
    let m1816 = v0;
    match m1816 {
        Adt139::C0 => {
            false
        }
        Adt139::C1 => {
            true
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
    }
}

pub fn f578(v0: Adt135, v1: Adt140, v2: Adt136, v3: u64) -> R<Adt149> {
    let a1818 = match v0.clone() {
        Adt135::C0(_, _, h1817) => {
            h1817
        }
    };
    let a1819 = f633();
    if nat_lt(a1818, a1819) {
        Ok(Adt149::C1(Adt135::C0(v2, List::cons(v1, match v0.clone() {
            Adt135::C0(_, h1820, _) => {
                h1820
            }
        }), {
            let a1822 = match v0 {
                Adt135::C0(_, _, h1821) => {
                    h1821
                }
            };
            let a1823 = 1u64;
            nat_add(a1822, a1823)?
        })))
    } else {
        Ok(f513(Adt117::C9(v3)))
    }
}

pub fn f579(v0: bool, v1: Adt139, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt136 {
    Adt136::C8(Adt138::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f580(v0: u64) -> i64 {
    let a1824 = v0;
    let a1825 = 0u64;
    if nat_eq(a1824, a1825) {
        0i64
    } else {
        let a1826 = v0;
        let a1827 = 1u64;
        if nat_eq(a1826, a1827) {
            1i64
        } else {
            let a1828 = v0;
            let a1829 = 2u64;
            if nat_eq(a1828, a1829) {
                2i64
            } else {
                let a1830 = v0;
                let a1831 = 3u64;
                if nat_eq(a1830, a1831) {
                    3i64
                } else {
                    let a1832 = v0;
                    let a1833 = 4u64;
                    if nat_eq(a1832, a1833) {
                        4i64
                    } else {
                        let a1834 = v0;
                        let a1835 = 5u64;
                        if nat_eq(a1834, a1835) {
                            5i64
                        } else {
                            let a1836 = v0;
                            let a1837 = 6u64;
                            if nat_eq(a1836, a1837) {
                                6i64
                            } else {
                                let a1838 = v0;
                                let a1839 = 7u64;
                                if nat_eq(a1838, a1839) {
                                    7i64
                                } else {
                                    let a1840 = v0;
                                    let a1841 = 8u64;
                                    if nat_eq(a1840, a1841) {
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

pub fn f581(p1844: List<Adt9>, p1845: List<Adt9>) -> List<Adt9> {
    let m1843 = (p1844, p1845);
    {
        let mut m1843 = m1843;
        loop {
            let step = {
                let (v0, v1) = m1843;
                let m1842 = v0;
                match m1842.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt9>, List<Adt9>), List<Adt9>>((v3, List::cons(v2, v1)))
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

pub fn f582(v0: Adt135, v1: u64) -> Adt149 {
    let m1847 = match v0.clone() {
        Adt135::C0(_, h1846, _) => {
            h1846
        }
    };
    match m1847.uncons() {
        None => {
            f513(Adt117::C2(v1))
        }
        Some((v2, _)) => {
            let m1848 = v2;
            match m1848 {
                Adt140::C0(_) => {
                    f512(v0, Adt136::C0)
                }
                Adt140::C1(_, _) => {
                    f512(v0, Adt136::C4)
                }
            }
        }
    }
}

pub fn f583(p1851: List<(Str, Adt9)>, p1852: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m1850 = (p1851, p1852);
    {
        let mut m1850 = m1850;
        loop {
            let step = {
                let (v0, v1) = m1850;
                let m1849 = v0;
                match m1849.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Adt9)>), List<(Str, Adt9)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<(Str, Adt9)>), List<(Str, Adt9)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: Adt135, v1: Adt136) -> Adt135 {
    Adt135::C0(v1, match v0.clone() {
        Adt135::C0(_, h1853, _) => {
            h1853
        }
    }, match v0 {
        Adt135::C0(_, _, h1854) => {
            h1854
        }
    })
}

pub fn f585(v0: Adt135, v1: u64, v2: List<u64>, v3: bool) -> Adt149 {
    let m1856 = {
        let a1855 = f362(f257(), f434(v2, List::<u64>::nil()));
        utf8_decode(a1855)
    };
    match m1856 {
        None => {
            f513(Adt117::C14)
        }
        Some(v4) => {
            if v3 {
                f685(v0, v1, v4)
            } else {
                f591(v0, Adt9::C3(v4))
            }
        }
    }
}

pub fn f586(v0: u64) -> u64 {
    let a1857 = v0;
    let a1858 = 34u64;
    if nat_eq(a1857, a1858) {
        34u64
    } else {
        let a1859 = v0;
        let a1860 = 92u64;
        if nat_eq(a1859, a1860) {
            92u64
        } else {
            let a1861 = v0;
            let a1862 = 47u64;
            if nat_eq(a1861, a1862) {
                47u64
            } else {
                let a1863 = v0;
                let a1864 = 98u64;
                if nat_eq(a1863, a1864) {
                    8u64
                } else {
                    let a1865 = v0;
                    let a1866 = 102u64;
                    if nat_eq(a1865, a1866) {
                        12u64
                    } else {
                        let a1867 = v0;
                        let a1868 = 110u64;
                        if nat_eq(a1867, a1868) {
                            10u64
                        } else {
                            let a1869 = v0;
                            let a1870 = 114u64;
                            if nat_eq(a1869, a1870) {
                                13u64
                            } else {
                                let a1871 = v0;
                                let a1872 = 116u64;
                                if nat_eq(a1871, a1872) {
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

pub fn f587(v0: u64) -> u64 {
    let a1873 = v0;
    let a1874 = 48u64;
    if nat_lt(a1873, a1874) {
        16u64
    } else {
        let a1875 = v0;
        let a1876 = 58u64;
        if nat_lt(a1875, a1876) {
            let a1877 = v0;
            let a1878 = 48u64;
            nat_sub(a1877, a1878)
        } else {
            let a1879 = v0;
            let a1880 = 65u64;
            if nat_lt(a1879, a1880) {
                16u64
            } else {
                let a1881 = v0;
                let a1882 = 71u64;
                if nat_lt(a1881, a1882) {
                    let a1883 = v0;
                    let a1884 = 55u64;
                    nat_sub(a1883, a1884)
                } else {
                    let a1885 = v0;
                    let a1886 = 97u64;
                    if nat_lt(a1885, a1886) {
                        16u64
                    } else {
                        let a1887 = v0;
                        let a1888 = 103u64;
                        if nat_lt(a1887, a1888) {
                            let a1889 = v0;
                            let a1890 = 87u64;
                            nat_sub(a1889, a1890)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f588(v0: Adt135, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt149> {
    let a1891 = v3;
    let a1892 = 0u64;
    if nat_eq(a1891, a1892) {
        if f504(v2, 55296u64, 56319u64) {
            Ok(f512(v0, Adt136::C6(Adt137::C3(v2), v4, v5)))
        } else {
            if f504(v2, 56320u64, 57343u64) {
                Ok(f513(Adt117::C6(v1)))
            } else {
                Ok(f512(v0, Adt136::C6(Adt137::C0, f686(v2, v4)?, v5)))
            }
        }
    } else {
        if f504(v2, 56320u64, 57343u64) {
            Ok(f512(v0, Adt136::C6(Adt137::C0, f686({
                let a1901 = 65536u64;
                let a1902 = {
                    let a1899 = {
                        let a1895 = {
                            let a1893 = v3;
                            let a1894 = 55296u64;
                            nat_sub(a1893, a1894)
                        };
                        let a1896 = 1024u64;
                        nat_mul(a1895, a1896)?
                    };
                    let a1900 = {
                        let a1897 = v2;
                        let a1898 = 56320u64;
                        nat_sub(a1897, a1898)
                    };
                    nat_add(a1899, a1900)?
                };
                nat_add(a1901, a1902)?
            }, v4)?, v5)))
        } else {
            Ok(f513(Adt117::C6(v1)))
        }
    }
}

pub fn f589(v0: u64, v1: u64) -> u64 {
    let m1905 = {
        let a1903 = f590(v0);
        let a1904 = v1;
        index_list(a1903, a1904)
    };
    match m1905 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f590(v0: u64) -> List<u64> {
    let a1906 = v0;
    let a1907 = 0u64;
    if nat_eq(a1906, a1907) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1908 = v0;
        let a1909 = 1u64;
        if nat_eq(a1908, a1909) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f591(v0: Adt135, v1: Adt9) -> Adt149 {
    Adt149::C1(f506(v1, match v0.clone() {
        Adt135::C0(_, h1910, _) => {
            h1910
        }
    }, match v0 {
        Adt135::C0(_, _, h1911) => {
            h1911
        }
    }))
}

pub fn f592(v0: u64) -> Adt9 {
    let a1912 = v0;
    let a1913 = 0u64;
    if nat_eq(a1912, a1913) {
        Adt9::C1(true)
    } else {
        let a1914 = v0;
        let a1915 = 1u64;
        if nat_eq(a1914, a1915) {
            Adt9::C1(false)
        } else {
            Adt9::C0
        }
    }
}

pub fn f593(v0: Adt135, v1: Adt138, v2: Adt139) -> Adt149 {
    f512(v0, Adt136::C8(Adt138::C0(match v1.clone() {
        Adt138::C0(h1916, _, _, _, _, _, _) => {
            h1916
        }
    }, v2, match v1.clone() {
        Adt138::C0(_, _, h1917, _, _, _, _) => {
            h1917
        }
    }, match v1.clone() {
        Adt138::C0(_, _, _, h1918, _, _, _) => {
            h1918
        }
    }, match v1.clone() {
        Adt138::C0(_, _, _, _, h1919, _, _) => {
            h1919
        }
    }, match v1.clone() {
        Adt138::C0(_, _, _, _, _, h1920, _) => {
            h1920
        }
    }, match v1 {
        Adt138::C0(_, _, _, _, _, _, h1921) => {
            h1921
        }
    })))
}

pub fn f594(v0: Adt135, v1: u64, v2: u64, v3: Adt138, v4: bool) -> R<Adt149> {
    let v5: u64 = {
        let a1923 = match v3.clone() {
            Adt138::C0(_, _, _, h1922, _, _, _) => {
                h1922
            }
        };
        let a1924 = 0u64;
        if if nat_eq(a1923, a1924) {
            let a1925 = v2;
            let a1926 = 48u64;
            nat_eq(a1925, a1926)
        } else {
            false
        } {
            0u64
        } else {
            let a1928 = match v3.clone() {
                Adt138::C0(_, _, _, h1927, _, _, _) => {
                    h1927
                }
            };
            let a1929 = 1u64;
            nat_add(a1928, a1929)?
        }
    };
    let v6: i64 = if v4 {
        f359(match v3.clone() {
            Adt138::C0(_, _, _, _, h1930, _, _) => {
                h1930
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt138::C0(_, _, _, _, h1931, _, _) => {
                h1931
            }
        }
    };
    let a1932 = f687();
    let a1933 = v5;
    if nat_lt(a1932, a1933) {
        Ok(f513(Adt117::C8(v1)))
    } else {
        let m1936 = {
            let a1934 = 99999i64;
            let a1935 = v6;
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
            Ok(f513(Adt117::C8(v1)))
        } else {
            Ok(f512(v0, f579(match v3.clone() {
                Adt138::C0(h1937, _, _, _, _, _, _) => {
                    h1937
                }
            }, if v4 {
                Adt139::C4
            } else {
                Adt139::C2
            }, f359({
                let a1939 = match v3.clone() {
                    Adt138::C0(_, _, h1938, _, _, _, _) => {
                        h1938
                    }
                };
                let a1940 = 10i64;
                int_mul(a1939, a1940)?
            }, f580({
                let a1941 = v2;
                let a1942 = 48u64;
                nat_sub(a1941, a1942)
            }))?, v5, v6, match v3.clone() {
                Adt138::C0(_, _, _, _, _, h1943, _) => {
                    h1943
                }
            }, match v3 {
                Adt138::C0(_, _, _, _, _, _, h1944) => {
                    h1944
                }
            })))
        }
    }
}

pub fn f595(v0: u64) -> bool {
    let a1945 = v0;
    let a1946 = 101u64;
    if nat_eq(a1945, a1946) {
        true
    } else {
        let a1947 = v0;
        let a1948 = 69u64;
        nat_eq(a1947, a1948)
    }
}

pub fn f596(v0: u64) -> bool {
    f504(v0, 48u64, 57u64)
}

pub fn f597(v0: Adt135, v1: u64, v2: u64, v3: Adt138) -> R<Adt149> {
    let m1949 = f505(v3)?;
    match m1949 {
        Adt155::C0 => {
            Ok(f513(Adt117::C7(v1)))
        }
        Adt155::C1 => {
            Ok(f513(Adt117::C8(v1)))
        }
        Adt155::C2(v4) => {
            Ok(f510(f506(v4, match v0.clone() {
                Adt135::C0(_, h1950, _) => {
                    h1950
                }
            }, match v0 {
                Adt135::C0(_, _, h1951) => {
                    h1951
                }
            }), v1, v2))
        }
    }
}

pub fn f598(v0: Adt135, v1: u64, v2: u64, v3: Adt138, v4: bool) -> R<Adt149> {
    let v5: i64 = f359({
        let a1953 = match v3.clone() {
            Adt138::C0(_, _, _, _, _, _, h1952) => {
                h1952
            }
        };
        let a1954 = 10i64;
        int_mul(a1953, a1954)?
    }, f580({
        let a1955 = v2;
        let a1956 = 48u64;
        nat_sub(a1955, a1956)
    }))?;
    let m1959 = {
        let a1957 = 99999i64;
        let a1958 = v5;
        compare(a1957, a1958)
    };
    if match m1959 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f513(Adt117::C8(v1)))
    } else {
        Ok(f512(v0, f579(match v3.clone() {
            Adt138::C0(h1960, _, _, _, _, _, _) => {
                h1960
            }
        }, Adt139::C7, match v3.clone() {
            Adt138::C0(_, _, h1961, _, _, _, _) => {
                h1961
            }
        }, match v3.clone() {
            Adt138::C0(_, _, _, h1962, _, _, _) => {
                h1962
            }
        }, match v3 {
            Adt138::C0(_, _, _, _, h1963, _, _) => {
                h1963
            }
        }, v4, v5)))
    }
}

pub fn f599(v0: Adt93) -> Option<Adt94> {
    let m1964 = v0;
    match m1964 {
        Adt93::C0(_) => {
            None::<Adt94>
        }
        Adt93::C1(_) => {
            None::<Adt94>
        }
        Adt93::C2(_) => {
            None::<Adt94>
        }
        Adt93::C3(_) => {
            None::<Adt94>
        }
        Adt93::C4(_) => {
            None::<Adt94>
        }
        Adt93::C5(_) => {
            None::<Adt94>
        }
        Adt93::C6(_) => {
            None::<Adt94>
        }
        Adt93::C7(_) => {
            None::<Adt94>
        }
        Adt93::C8(_) => {
            None::<Adt94>
        }
        Adt93::C9(_) => {
            None::<Adt94>
        }
        Adt93::C10 => {
            None::<Adt94>
        }
        Adt93::C11 => {
            None::<Adt94>
        }
        Adt93::C12(_) => {
            None::<Adt94>
        }
        Adt93::C13 => {
            None::<Adt94>
        }
        Adt93::C14 => {
            None::<Adt94>
        }
        Adt93::C15(v12) => {
            Some(v12)
        }
    }
}

pub fn f600(v0: u64) -> bool {
    let a1965 = v0;
    let a1966 = f688();
    nat_lt(a1965, a1966)
}

pub fn f601() -> Adt145 {
    Adt145::C4(Adt93::C9(Str::lit("the effect tag does not fit thirty-two bits")))
}

pub fn f602(v0: u64, v1: Adt9) -> bool {
    let a1967 = v0;
    let a1968 = 0u64;
    if nat_eq(a1967, a1968) {
        f689(v1)
    } else {
        let a1969 = v0;
        let a1970 = 1u64;
        if nat_eq(a1969, a1970) {
            f690(v1)
        } else {
            let a1971 = v0;
            let a1972 = 2u64;
            if nat_eq(a1971, a1972) {
                f691(v1)
            } else {
                let a1973 = v0;
                let a1974 = 3u64;
                if nat_eq(a1973, a1974) {
                    f692(v1)
                } else {
                    let a1975 = v0;
                    let a1976 = 4u64;
                    if nat_eq(a1975, a1976) {
                        f693(v1)
                    } else {
                        let a1977 = v0;
                        let a1978 = 5u64;
                        if nat_eq(a1977, a1978) {
                            f694(v1)
                        } else {
                            let a1979 = v0;
                            let a1980 = 6u64;
                            if nat_eq(a1979, a1980) {
                                f695(v1)
                            } else {
                                let a1981 = v0;
                                let a1982 = 7u64;
                                if nat_eq(a1981, a1982) {
                                    f696(v1)
                                } else {
                                    f697(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f603(v0: Adt9, v1: Str) -> Option<Adt9> {
    let a1983 = v1.clone();
    let a1984 = Str::lit("#");
    if equal(a1983, a1984) {
        Some(v0)
    } else {
        let m1985 = f236(v0);
        match m1985 {
            None => {
                None::<Adt9>
            }
            Some(v2) => {
                let m1986 = f235(Str::lit("$defs"), v2);
                match m1986 {
                    Adt116::C0 => {
                        None::<Adt9>
                    }
                    Adt116::C1(v3) => {
                        let m1987 = f236(v3);
                        match m1987 {
                            None => {
                                None::<Adt9>
                            }
                            Some(v4) => {
                                f698(v1, match v4 {
                                    Adt8::C0(h1988) => {
                                        h1988
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

pub fn f604(v0: Str, v1: List<(Str, Adt9)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt122) -> R<Adt150> {
    let m1989 = v3.clone();
    match m1989.uncons() {
        None => {
            Ok(Adt150::C0(Adt102::C8(v0)))
        }
        Some((_, _)) => {
            if f346(v3.clone(), true) {
                let m1990 = f699(v1);
                match m1990 {
                    None => {
                        Ok(Adt150::C0(Adt102::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f700(v3)?;
                        if f701(v9.clone(), match v5.clone() {
                            Adt122::C0(_, h1991) => {
                                h1991
                            }
                        }) {
                            Ok(Adt150::C0(Adt102::C12(v0, v2)))
                        } else {
                            Ok(Adt150::C1(Adt122::C0(List::cons(Adt15::C0(v0, v2, f299(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt122::C0(h1992, _) => {
                                    h1992
                                }
                            }), List::cons(v9, match v5 {
                                Adt122::C0(_, h1993) => {
                                    h1993
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt150::C0(Adt102::C9(v0)))
            }
        }
    }
}

pub fn f605(p1998: List<(Str, Adt9)>, v1: bool, v2: List<Str>, p1999: List<Adt110>) -> List<Adt110> {
    let m1997 = (p1998, p1999);
    {
        let mut m1997 = m1997;
        loop {
            let step = {
                let (v0, v3) = m1997;
                let m1994 = v0;
                match m1994.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt110>), List<Adt110>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt9)>, List<Adt110>), List<Adt110>>((v5, List::cons(Adt110::C0({
                            let (_, h1995) = v4;
                            h1995
                        }, v1, List::cons({
                            let (h1996, _) = v4;
                            h1996
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606(p2003: List<(Str, Adt9)>, p2004: List<Adt110>) -> List<Adt110> {
    let m2002 = (p2003, p2004);
    {
        let mut m2002 = m2002;
        loop {
            let step = {
                let (v0, v1) = m2002;
                let m2000 = v0;
                match m2000.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt110>), List<Adt110>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt9)>, List<Adt110>), List<Adt110>>((v3, List::cons(Adt110::C0({
                            let (_, h2001) = v2;
                            h2001
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2002 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f607(p2007: List<Adt9>, p2008: List<Adt110>) -> List<Adt110> {
    let m2006 = (p2007, p2008);
    {
        let mut m2006 = m2006;
        loop {
            let step = {
                let (v0, v1) = m2006;
                let m2005 = v0;
                match m2005.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt110>), List<Adt110>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt9>, List<Adt110>), List<Adt110>>((v3, List::cons(Adt110::C0(v2, false, List::<Str>::nil()), v1)))
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

pub fn f608(v0: Str) -> bool {
    let a2009 = v0.clone();
    let a2010 = Str::lit("null");
    if equal(a2009, a2010) {
        true
    } else {
        let a2011 = v0.clone();
        let a2012 = Str::lit("boolean");
        if equal(a2011, a2012) {
            true
        } else {
            let a2013 = v0.clone();
            let a2014 = Str::lit("object");
            if equal(a2013, a2014) {
                true
            } else {
                let a2015 = v0.clone();
                let a2016 = Str::lit("array");
                if equal(a2015, a2016) {
                    true
                } else {
                    let a2017 = v0.clone();
                    let a2018 = Str::lit("number");
                    if equal(a2017, a2018) {
                        true
                    } else {
                        let a2019 = v0.clone();
                        let a2020 = Str::lit("string");
                        if equal(a2019, a2020) {
                            true
                        } else {
                            let a2021 = v0;
                            let a2022 = Str::lit("integer");
                            equal(a2021, a2022)
                        }
                    }
                }
            }
        }
    }
}

pub fn f609(v0: Str, v1: Adt9) -> R<bool> {
    let a2023 = v0.clone();
    let a2024 = f702(v1.clone());
    if equal(a2023, a2024) {
        Ok(true)
    } else {
        let a2025 = v0;
        let a2026 = Str::lit("integer");
        if equal(a2025, a2026) {
            let m2027 = f613(v1);
            match m2027 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f703(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f610(p2030: List<Adt9>, v1: Adt9, p2031: bool) -> R<Adt151> {
    let m2029 = (p2030, p2031);
    {
        let mut m2029 = m2029;
        loop {
            let step = {
                let (v0, v2) = m2029;
                let m2028 = v0;
                match m2028.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt9>, bool), R<Adt151>>(Ok(Adt151::C0))
                        } else {
                            Err::<(List<Adt9>, bool), R<Adt151>>(Ok(Adt151::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f608(f704(v3.clone())) {
                            Ok::<(List<Adt9>, bool), R<Adt151>>((v4, if v2 {
                                true
                            } else {
                                f609(f704(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt9>, bool), R<Adt151>>(Ok(Adt151::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2029 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f611(p2035: List<Adt9>, v1: Adt9) -> R<Adt151> {
    let m2034 = p2035;
    {
        let mut m2034 = m2034;
        loop {
            let step = {
                let v0 = m2034;
                let m2032 = v0;
                match m2032.uncons() {
                    None => {
                        Err::<List<Adt9>, R<Adt151>>(Ok(Adt151::C1))
                    }
                    Some((v2, v3)) => {
                        let m2033 = f612(v2, v1.clone())?;
                        match m2033 {
                            Adt160::C0 => {
                                Err::<List<Adt9>, R<Adt151>>(Ok(Adt151::C0))
                            }
                            Adt160::C1 => {
                                Ok::<List<Adt9>, R<Adt151>>(v3)
                            }
                            Adt160::C2 => {
                                Err::<List<Adt9>, R<Adt151>>(Ok(Adt151::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f612(v0: Adt9, v1: Adt9) -> R<Adt160> {
    f705(f706(), List::cons((v0, v1), List::<(Adt9, Adt9)>::nil()))
}

pub fn f613(v0: Adt9) -> Option<Adt10> {
    let m2036 = v0;
    match m2036 {
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

pub fn f614(v0: u64, v1: Adt161) -> bool {
    let a2037 = v0;
    let a2038 = 0u64;
    if nat_eq(a2037, a2038) {
        let a2039 = f707(v1);
        bool_not(a2039)
    } else {
        let a2040 = v0;
        let a2041 = 1u64;
        if nat_eq(a2040, a2041) {
            let a2042 = f629(v1);
            bool_not(a2042)
        } else {
            let a2043 = v0;
            let a2044 = 2u64;
            if nat_eq(a2043, a2044) {
                f629(v1)
            } else {
                f707(v1)
            }
        }
    }
}

pub fn f615(v0: Adt10, v1: Adt10) -> R<Adt161> {
    f708(v0.clone(), v1.clone(), f709(match v0 {
        Adt10::C0(h2045, _) => {
            h2045
        }
    }), f709(match v1 {
        Adt10::C0(h2046, _) => {
            h2046
        }
    }))
}

pub fn f616(v0: Str, v1: bool, v2: List<(Str, Adt9)>, v3: u64) -> R<Adt151> {
    let m2047 = f309(v0, v2);
    match m2047 {
        Adt116::C0 => {
            Ok(Adt151::C0)
        }
        Adt116::C1(v4) => {
            let m2048 = f710(v4);
            match m2048 {
                None => {
                    Ok(Adt151::C2)
                }
                Some(v5) => {
                    let m2051 = {
                        let a2049 = v5;
                        let a2050 = 0i64;
                        compare(a2049, a2050)
                    };
                    if match m2051 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt151::C2)
                    } else {
                        if v1 {
                            if f707(f711(f289(v3, 0i64)?, v5)) {
                                Ok(Adt151::C1)
                            } else {
                                Ok(Adt151::C0)
                            }
                        } else {
                            if f629(f711(f289(v3, 0i64)?, v5)) {
                                Ok(Adt151::C1)
                            } else {
                                Ok(Adt151::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f617(v0: Str) -> R<u64> {
    f712(f29({
        let a2052 = v0;
        utf8_encode(a2052)
    })?, 0u64)
}

pub fn f618(p2057: List<Adt9>, p2058: u64) -> R<u64> {
    let m2056 = (p2057, p2058);
    {
        let mut m2056 = m2056;
        loop {
            let step = {
                let (v0, v1) = m2056;
                let m2053 = v0;
                match m2053.uncons() {
                    None => {
                        Err::<(List<Adt9>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt9>, u64), R<u64>>((v3, {
                            let a2054 = v1;
                            let a2055 = 1u64;
                            nat_add(a2054, a2055)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f619(p2063: List<(Str, Adt9)>, p2064: u64) -> R<u64> {
    let m2062 = (p2063, p2064);
    {
        let mut m2062 = m2062;
        loop {
            let step = {
                let (v0, v1) = m2062;
                let m2059 = v0;
                match m2059.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt9)>, u64), R<u64>>((v3, {
                            let a2060 = v1;
                            let a2061 = 1u64;
                            nat_add(a2060, a2061)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(p2068: List<Adt9>, v1: Adt8) -> Adt151 {
    let m2067 = p2068;
    {
        let mut m2067 = m2067;
        loop {
            let step = {
                let v0 = m2067;
                let m2065 = v0;
                match m2065.uncons() {
                    None => {
                        Err::<List<Adt9>, Adt151>(Adt151::C0)
                    }
                    Some((v2, v3)) => {
                        let m2066 = f287(v2);
                        match m2066 {
                            None => {
                                Err::<List<Adt9>, Adt151>(Adt151::C2)
                            }
                            Some(v4) => {
                                if f501(v4, v1.clone()) {
                                    Ok::<List<Adt9>, Adt151>(v3)
                                } else {
                                    Err::<List<Adt9>, Adt151>(Adt151::C1)
                                }
                            }
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

pub fn f621(p2075: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, v2: List<Adt112>, p2076: List<Adt114>) -> List<Adt114> {
    let m2074 = (p2075, p2076);
    {
        let mut m2074 = m2074;
        loop {
            let step = {
                let (v0, v3) = m2074;
                let m2069 = v0;
                match m2069.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt114>), List<Adt114>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2071 = f309({
                            let (h2070, _) = v4.clone();
                            h2070
                        }, v1.clone());
                        match m2071 {
                            Adt116::C0 => {
                                Ok::<(List<(Str, Adt9)>, List<Adt114>), List<Adt114>>((v5, v3))
                            }
                            Adt116::C1(v6) => {
                                Ok::<(List<(Str, Adt9)>, List<Adt114>), List<Adt114>>((v5, List::cons(Adt114::C0({
                                    let (_, h2072) = v4;
                                    h2072
                                }, v6, List::cons(Adt112::C0({
                                    let (h2073, _) = v4;
                                    h2073
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(p2081: List<(Str, Adt9)>, v1: List<(Str, Adt9)>) -> bool {
    let m2080 = p2081;
    {
        let mut m2080 = m2080;
        loop {
            let step = {
                let v0 = m2080;
                let m2077 = v0;
                match m2077.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2079 = f309({
                            let (h2078, _) = v2;
                            h2078
                        }, v1.clone());
                        match m2079 {
                            Adt116::C0 => {
                                Err::<List<(Str, Adt9)>, bool>(true)
                            }
                            Adt116::C1(_) => {
                                Ok::<List<(Str, Adt9)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2080 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623(v0: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m2082 = f309(Str::lit("properties"), v0);
    match m2082 {
        Adt116::C0 => {
            List::<(Str, Adt9)>::nil()
        }
        Adt116::C1(v1) => {
            let m2083 = f236(v1);
            match m2083 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt8::C0(h2084) => {
                            h2084
                        }
                    }
                }
            }
        }
    }
}

pub fn f624(p2091: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, v2: Adt9, v3: List<Adt112>, p2092: List<Adt114>) -> List<Adt114> {
    let m2090 = (p2091, p2092);
    {
        let mut m2090 = m2090;
        loop {
            let step = {
                let (v0, v4) = m2090;
                let m2085 = v0;
                match m2085.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<Adt114>), List<Adt114>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2087 = f309({
                            let (h2086, _) = v5.clone();
                            h2086
                        }, v1.clone());
                        match m2087 {
                            Adt116::C0 => {
                                Ok::<(List<(Str, Adt9)>, List<Adt114>), List<Adt114>>((v6, List::cons(Adt114::C0(v2.clone(), {
                                    let (_, h2088) = v5;
                                    h2088
                                }, List::cons(Adt112::C0({
                                    let (h2089, _) = v5;
                                    h2089
                                }), v3.clone())), v4)))
                            }
                            Adt116::C1(_) => {
                                Ok::<(List<(Str, Adt9)>, List<Adt114>), List<Adt114>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2090 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f625(p2097: List<Adt9>, p2098: u64, v2: Adt9, v3: List<Adt112>, p2099: List<Adt114>) -> R<List<Adt114>> {
    let m2096 = (p2097, (p2098, p2099));
    {
        let mut m2096 = m2096;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2096;
                let m2093 = v0;
                match m2093.uncons() {
                    None => {
                        Err::<(List<Adt9>, (u64, List<Adt114>)), R<List<Adt114>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt9>, (u64, List<Adt114>)), R<List<Adt114>>>((v6, ({
                            let a2094 = v1;
                            let a2095 = 1u64;
                            nat_add(a2094, a2095)?
                        }, List::cons(Adt114::C0(v2.clone(), v5, List::cons(Adt112::C1(v1), v3.clone())), v4))))
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

pub fn f626(p2102: List<Adt9>, v1: Adt9, v2: List<Adt112>, p2103: List<Adt114>) -> List<Adt114> {
    let m2101 = (p2102, p2103);
    {
        let mut m2101 = m2101;
        loop {
            let step = {
                let (v0, v3) = m2101;
                let m2100 = v0;
                match m2100.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt114>), List<Adt114>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt9>, List<Adt114>), List<Adt114>>((v5, List::cons(Adt114::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2101 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2104 = f713(v0)?;
    match m2104 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2105 = f713(v1)?;
            match m2105 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2106 = v2;
                    let a2107 = 61u64;
                    if nat_eq(a2106, a2107) {
                        f714(v5, v6, v4, v3)
                    } else {
                        let m2108 = f713(v2)?;
                        match m2108 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2109 = v3;
                                let a2110 = 61u64;
                                if nat_eq(a2109, a2110) {
                                    f715(v5, v6, v7, v4)
                                } else {
                                    let m2111 = f713(v3)?;
                                    match m2111 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f716(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f628(v0: List<u64>) -> bool {
    let m2112 = v0;
    match m2112.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f629(v0: Adt161) -> bool {
    let m2113 = v0;
    match m2113 {
        Adt161::C0 => {
            false
        }
        Adt161::C1 => {
            false
        }
        Adt161::C2 => {
            true
        }
    }
}

pub fn f630(v0: u64) -> List<u64> {
    f590(v0)
}

pub fn f631(v0: Adt10) -> R<bool> {
    let a2131 = {
        let m2117 = {
            let a2115 = f717();
            let a2116 = match v0.clone() {
                Adt10::C0(h2114, _) => {
                    h2114
                }
            };
            compare(a2115, a2116)
        };
        if match m2117 {
            Ordering::Less => {
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
            let m2122 = {
                let a2120 = match v0.clone() {
                    Adt10::C0(h2118, _) => {
                        h2118
                    }
                };
                let a2121 = {
                    let a2119 = f717();
                    int_neg(a2119)?
                };
                compare(a2120, a2121)
            };
            if match m2122 {
                Ordering::Less => {
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
                let m2126 = {
                    let a2124 = 9999i64;
                    let a2125 = match v0.clone() {
                        Adt10::C0(_, h2123) => {
                            h2123
                        }
                    };
                    compare(a2124, a2125)
                };
                if match m2126 {
                    Ordering::Less => {
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
                    let m2130 = {
                        let a2128 = match v0 {
                            Adt10::C0(_, h2127) => {
                                h2127
                            }
                        };
                        let a2129 = -9999i64;
                        compare(a2128, a2129)
                    };
                    match m2130 {
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
    Ok(bool_not(a2131))
}

pub fn f632(v0: Adt10) -> R<List<u64>> {
    let m2135 = {
        let a2133 = match v0.clone() {
            Adt10::C0(_, h2132) => {
                h2132
            }
        };
        let a2134 = 0i64;
        compare(a2133, a2134)
    };
    if if match m2135 {
        Ordering::Less => {
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
        let m2139 = {
            let a2137 = 0i64;
            let a2138 = match v0.clone() {
                Adt10::C0(_, h2136) => {
                    h2136
                }
            };
            compare(a2137, a2138)
        };
        match m2139 {
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
        let a2142 = f718(match v0.clone() {
            Adt10::C0(h2140, _) => {
                h2140
            }
        })?;
        let a2143 = List::cons(101u64, f718(match v0 {
            Adt10::C0(_, h2141) => {
                h2141
            }
        })?);
        Ok(append_list(a2142, a2143))
    } else {
        f718(match v0 {
            Adt10::C0(h2144, _) => {
                h2144
            }
        })
    }
}

pub fn f633() -> u64 {
    512u64
}

pub fn f634(v0: u64, v1: Adt127, v2: List<Adt127>) -> List<Adt127> {
    List::cons(Adt127::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f635(p2147: List<u64>, p2148: List<u64>) -> R<List<u64>> {
    let m2146 = (p2147, p2148);
    {
        let mut m2146 = m2146;
        loop {
            let step = {
                let (v0, v1) = m2146;
                let m2145 = v0;
                match m2145.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f434(f719(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f636(v0: Adt37) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f720(match v0.clone() {
        Adt37::C0(_, h2149, _, _) => {
            h2149
        }
    })), List::cons((Str::lit("capabilities"), f721(match v0.clone() {
        Adt37::C0(_, _, h2150, _) => {
            h2150
        }
    })), {
        let a2153 = f722(Str::lit("instructions"), match v0.clone() {
            Adt37::C0(_, _, _, h2151) => {
                h2151
            }
        });
        let a2154 = f723(match v0 {
            Adt37::C0(h2152, _, _, _) => {
                h2152
            }
        })?;
        append_list(a2153, a2154)
    })))
}

pub fn f637(v0: Adt38) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("tools"), Adt9::C4(f724(match v0.clone() {
        Adt38::C0(_, _, h2155) => {
            h2155
        }
    }))), {
        let a2158 = f722(Str::lit("nextCursor"), match v0.clone() {
            Adt38::C0(_, h2156, _) => {
                h2156
            }
        });
        let a2159 = f723(match v0 {
            Adt38::C0(h2157, _, _) => {
                h2157
            }
        })?;
        append_list(a2158, a2159)
    }))
}

pub fn f638(v0: Option<List<(Str, Adt9)>>) -> R<Option<List<(Str, Adt9)>>> {
    let m2160 = v0;
    match m2160 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            let m2161 = f657(Adt9::C5(v1))?;
            match m2161 {
                None => {
                    Ok(None::<List<(Str, Adt9)>>)
                }
                Some(v2) => {
                    let m2162 = f236(v2);
                    match m2162 {
                        None => {
                            Ok(None::<List<(Str, Adt9)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt8::C0(h2163) => {
                                    h2163
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f639(v0: Adt41) -> R<Option<List<(Str, Adt9)>>> {
    let m2165 = f725(match v0.clone() {
        Adt41::C0(h2164, _, _) => {
            h2164
        }
    })?;
    match m2165 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt9::C4(v1)), {
                let a2168 = f726(Str::lit("structuredContent"), match v0.clone() {
                    Adt41::C0(_, h2166, _) => {
                        h2166
                    }
                });
                let a2169 = f727(Str::lit("isError"), match v0 {
                    Adt41::C0(_, _, h2167) => {
                        h2167
                    }
                });
                append_list(a2168, a2169)
            })))
        }
    }
}

pub fn f640(v0: Adt55) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("resources"), Adt9::C4(f728(match v0.clone() {
        Adt55::C0(_, _, h2170) => {
            h2170
        }
    })?)), {
        let a2173 = f722(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h2171, _) => {
                h2171
            }
        });
        let a2174 = f723(match v0 {
            Adt55::C0(h2172, _, _) => {
                h2172
            }
        })?;
        append_list(a2173, a2174)
    }))
}

pub fn f641(v0: Adt56) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("contents"), Adt9::C4(f729(match v0.clone() {
        Adt56::C0(_, h2175) => {
            h2175
        }
    }))), f723(match v0 {
        Adt56::C0(h2176, _) => {
            h2176
        }
    })?))
}

pub fn f642(v0: Adt57) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt9::C4(f730(match v0.clone() {
        Adt57::C0(_, _, h2177) => {
            h2177
        }
    }))), {
        let a2180 = f722(Str::lit("nextCursor"), match v0.clone() {
            Adt57::C0(_, h2178, _) => {
                h2178
            }
        });
        let a2181 = f723(match v0 {
            Adt57::C0(h2179, _, _) => {
                h2179
            }
        })?;
        append_list(a2180, a2181)
    }))
}

pub fn f643(v0: Adt59) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("prompts"), Adt9::C4(f731(match v0.clone() {
        Adt59::C0(_, _, h2182) => {
            h2182
        }
    }))), {
        let a2185 = f722(Str::lit("nextCursor"), match v0.clone() {
            Adt59::C0(_, h2183, _) => {
                h2183
            }
        });
        let a2186 = f723(match v0 {
            Adt59::C0(h2184, _, _) => {
                h2184
            }
        })?;
        append_list(a2185, a2186)
    }))
}

pub fn f644(v0: Adt62) -> R<Option<List<(Str, Adt9)>>> {
    let m2188 = f732(match v0.clone() {
        Adt62::C0(_, h2187) => {
            h2187
        }
    })?;
    match m2188 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2190 = f722(Str::lit("description"), match v0 {
                    Adt62::C0(h2189, _) => {
                        h2189
                    }
                });
                let a2191 = List::cons((Str::lit("messages"), Adt9::C4(v1)), List::<(Str, Adt9)>::nil());
                append_list(a2190, a2191)
            }))
        }
    }
}

pub fn f645(v0: Adt64) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("completion"), Adt9::C5(List::cons((Str::lit("values"), Adt9::C4(f669({
        let m2194 = match v0.clone() {
            Adt64::C0(h2193) => {
                h2193
            }
        };
        match m2194 {
            Adt65::C0(h2192, _, _) => {
                h2192
            }
        }
    }))), {
        let a2201 = f733(Str::lit("total"), {
            let m2197 = match v0.clone() {
                Adt64::C0(h2196) => {
                    h2196
                }
            };
            match m2197 {
                Adt65::C0(_, h2195, _) => {
                    h2195
                }
            }
        })?;
        let a2202 = f727(Str::lit("hasMore"), {
            let m2200 = match v0 {
                Adt64::C0(h2199) => {
                    h2199
                }
            };
            match m2200 {
                Adt65::C0(_, _, h2198) => {
                    h2198
                }
            }
        });
        append_list(a2201, a2202)
    }))), List::<(Str, Adt9)>::nil()))
}

pub fn f646(v0: Adt85) -> R<Option<List<(Str, Adt9)>>> {
    let m2203 = f734(f735(v0.clone()), List::<(Str, Adt9)>::nil())?;
    match m2203 {
        None => {
            Ok(None::<List<(Str, Adt9)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2204 = f736(v1);
                let a2205 = f737(f738(v0));
                append_list(a2204, a2205)
            }))
        }
    }
}

pub fn f647(v0: Adt66) -> Option<List<(Str, Adt9)>> {
    let m2206 = f739(v0.clone());
    match m2206 {
        None => {
            None::<List<(Str, Adt9)>>
        }
        Some(v1) => {
            Some({
                let a2207 = f740(f741(v0.clone()), f742(f743(v0)));
                let a2208 = v1;
                append_list(a2207, a2208)
            })
        }
    }
}

pub fn f648(v0: Adt91) -> Option<List<(Str, Adt9)>> {
    let m2210 = match v0.clone() {
        Adt91::C0(_, h2209, _) => {
            h2209
        }
    };
    match m2210 {
        Adt92::C0 => {
            f647({
                let m2213 = match v0 {
                    Adt91::C0(h2212, _, _) => {
                        h2212
                    }
                };
                match m2213 {
                    Adt88::C0(h2211, _, _) => {
                        h2211
                    }
                }
            })
        }
        Adt92::C1 => {
            Some(List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f649(v0: Adt162) -> Str {
    let m2214 = v0;
    match m2214 {
        Adt162::C0 => {
            Str::lit("complete")
        }
        Adt162::C1 => {
            Str::lit("input_required")
        }
        Adt162::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f650(v0: Adt36) -> Adt162 {
    let m2215 = v0;
    match m2215 {
        Adt36::C0(_) => {
            Adt162::C0
        }
        Adt36::C1(_) => {
            Adt162::C0
        }
        Adt36::C2(_) => {
            Adt162::C0
        }
        Adt36::C3(_) => {
            Adt162::C0
        }
        Adt36::C4(_) => {
            Adt162::C0
        }
        Adt36::C5(_) => {
            Adt162::C0
        }
        Adt36::C6(_) => {
            Adt162::C0
        }
        Adt36::C7(_) => {
            Adt162::C0
        }
        Adt36::C8(_) => {
            Adt162::C0
        }
        Adt36::C9(_) => {
            Adt162::C0
        }
        Adt36::C10(_) => {
            Adt162::C0
        }
        Adt36::C11 => {
            Adt162::C0
        }
        Adt36::C12(_) => {
            Adt162::C1
        }
        Adt36::C13(_) => {
            Adt162::C2
        }
        Adt36::C14(_) => {
            Adt162::C0
        }
    }
}

pub fn f651(v0: Option<Adt4>) -> List<(Str, Adt9)> {
    let m2216 = v0;
    match m2216 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((f744(), f745(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f652(v0: Bytes) -> R<Option<List<Adt88>>> {
    let a2218 = {
        let a2217 = v0.clone();
        length_bytes(a2217)
    };
    let a2219 = 0u64;
    if nat_eq(a2218, a2219) {
        Ok(Some(List::<Adt88>::nil()))
    } else {
        let m2220 = f268(v0)?;
        match m2220 {
            Ok(v2) => {
                let m2221 = f290(v2);
                match m2221 {
                    None => {
                        Ok(None::<List<Adt88>>)
                    }
                    Some(v3) => {
                        Ok(f746(v3, List::<Adt88>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt88>>)
            }
        }
    }
}

pub fn f653(v0: List<Adt88>) -> R<Option<List<u64>>> {
    let m2222 = f747(v0);
    match m2222 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m2223 = f324(v1)?;
            match m2223 {
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

pub fn f654(v0: Adt88, v1: List<Adt88>, v2: i64) -> R<List<Adt88>> {
    Ok(List::cons(v0.clone(), f748(f749(f750(v0), v1, v2, List::<Adt88>::nil())?, List::<Adt88>::nil())))
}

pub fn f655(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f751(v2) {
            let a2224 = v1;
            let a2225 = 200u64;
            nat_eq(a2224, a2225)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f656(v0: u64, v1: List<List<u64>>) -> Adt104 {
    Adt104::C0(v0, f752(), List::<List<u64>>::nil(), f753(v1), None::<List<u64>>)
}

pub fn f657(v0: Adt9) -> R<Option<Adt9>> {
    f754(f755(), Adt163::C0(v0), List::<Adt164>::nil())
}

pub fn f658(v0: Adt144) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(f377())), List::cons((Str::lit("method"), Adt9::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt9::C5(List::cons((Str::lit("progressToken"), f756(match v0.clone() {
        Adt144::C0(h2226, _) => {
            h2226
        }
    })), List::cons((Str::lit("progress"), Adt9::C2({
        let m2229 = match v0.clone() {
            Adt144::C0(_, h2228) => {
                h2228
            }
        };
        match m2229 {
            Adt54::C0(h2227, _, _) => {
                h2227
            }
        }
    })), {
        let a2236 = f757({
            let m2232 = match v0.clone() {
                Adt144::C0(_, h2231) => {
                    h2231
                }
            };
            match m2232 {
                Adt54::C0(_, h2230, _) => {
                    h2230
                }
            }
        });
        let a2237 = f758({
            let m2235 = match v0 {
                Adt144::C0(_, h2234) => {
                    h2234
                }
            };
            match m2235 {
                Adt54::C0(_, _, h2233) => {
                    h2233
                }
            }
        });
        append_list(a2236, a2237)
    })))), List::<(Str, Adt9)>::nil()))))
}

pub fn f659(v0: Adt91) -> bool {
    if match v0.clone() {
        Adt91::C0(_, _, h2238) => {
            h2238
        }
    } {
        let m2240 = match v0 {
            Adt91::C0(_, h2239, _) => {
                h2239
            }
        };
        match m2240 {
            Adt92::C0 => {
                true
            }
            Adt92::C1 => {
                true
            }
        }
    } else {
        false
    }
}

pub fn f660(v0: Adt66) -> Option<Adt9> {
    let m2241 = f647(v0);
    match m2241 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("jsonrpc"), Adt9::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt9::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt9::C5(v1)), List::<(Str, Adt9)>::nil())))))
        }
    }
}

pub fn f661() -> u64 {
    429u64
}

pub fn f662() -> i64 {
    -32602i64
}

pub fn f663() -> i64 {
    -32022i64
}

pub fn f664() -> i64 {
    -32020i64
}

pub fn f665() -> i64 {
    -32601i64
}

pub fn f666(v0: Adt93) -> i64 {
    let m2242 = v0;
    match m2242 {
        Adt93::C0(_) => {
            f662()
        }
        Adt93::C1(_) => {
            f662()
        }
        Adt93::C2(_) => {
            f662()
        }
        Adt93::C3(_) => {
            f662()
        }
        Adt93::C4(_) => {
            f662()
        }
        Adt93::C5(_) => {
            f662()
        }
        Adt93::C6(_) => {
            f759()
        }
        Adt93::C7(_) => {
            f760()
        }
        Adt93::C8(_) => {
            f761()
        }
        Adt93::C9(_) => {
            f759()
        }
        Adt93::C10 => {
            f662()
        }
        Adt93::C11 => {
            f662()
        }
        Adt93::C12(_) => {
            f759()
        }
        Adt93::C13 => {
            f759()
        }
        Adt93::C14 => {
            f759()
        }
        Adt93::C15(_) => {
            f759()
        }
    }
}

pub fn f667(v0: Adt146) -> Str {
    let m2243 = v0;
    match m2243 {
        Adt146::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt146::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt146::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt146::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt146::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt146::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt146::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt146::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt146::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f668(v0: Adt129) -> Str {
    let m2244 = v0;
    match m2244 {
        Adt129::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt129::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt129::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt129::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt129::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt129::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt129::C6(v1) => {
            let m2245 = v1;
            match m2245 {
                Adt130::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt130::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt130::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt129::C7(v2) => {
            let m2246 = v2;
            match m2246 {
                Adt130::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt130::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt130::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt129::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt129::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt129::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f669(v0: List<Str>) -> List<Adt9> {
    f581(f762(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f670(v0: Adt21) -> Adt9 {
    Adt9::C5({
        let a2258 = f763(Str::lit("experimental"), match v0.clone() {
            Adt21::C0(h2247, _, _, _, _) => {
                h2247
            }
        });
        let a2259 = {
            let a2256 = f763(Str::lit("roots"), match v0.clone() {
                Adt21::C0(_, h2248, _, _, _) => {
                    h2248
                }
            });
            let a2257 = {
                let a2254 = f763(Str::lit("sampling"), match v0.clone() {
                    Adt21::C0(_, _, h2249, _, _) => {
                        h2249
                    }
                });
                let a2255 = {
                    let a2252 = f763(Str::lit("elicitation"), match v0.clone() {
                        Adt21::C0(_, _, _, h2250, _) => {
                            h2250
                        }
                    });
                    let a2253 = f763(Str::lit("extensions"), match v0 {
                        Adt21::C0(_, _, _, _, h2251) => {
                            h2251
                        }
                    });
                    append_list(a2252, a2253)
                };
                append_list(a2254, a2255)
            };
            append_list(a2256, a2257)
        };
        append_list(a2258, a2259)
    })
}

pub fn f671(p2262: List<List<u64>>) -> R<bool> {
    let m2261 = p2262;
    {
        let mut m2261 = m2261;
        loop {
            let step = {
                let v0 = m2261;
                let m2260 = v0;
                match m2260.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f764(f392(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2261 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f672() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f673(v0: Adt3, v1: Fn3, v2: u64, v3: Adt153, v4: bool) -> R<Result<Adt142, Adt145>> {
    if v4 {
        f765(v0, v1, v2, f766(match v3.clone() {
            Adt153::C0(_, h2263, _, _) => {
                h2263
            }
        }, match v3.clone() {
            Adt153::C0(_, _, h2264, _) => {
                h2264
            }
        }, match v3 {
            Adt153::C0(_, _, _, h2265) => {
                h2265
            }
        }))
    } else {
        Ok(Err::<Adt142, Adt145>(Adt145::C3(f767(match v3 {
            Adt153::C0(_, h2266, _, _) => {
                h2266
            }
        }))))
    }
}

pub fn f674(v0: Adt7, v1: Adt154) -> bool {
    let m2267 = v1;
    match m2267 {
        Adt154::C0 => {
            true
        }
        Adt154::C1 => {
            f768(match v0 {
                Adt7::C0(_, _, _, _, _, h2268, _) => {
                    h2268
                }
            })
        }
        Adt154::C2 => {
            f768(match v0 {
                Adt7::C0(_, _, _, _, _, h2269, _) => {
                    h2269
                }
            })
        }
        Adt154::C3 => {
            f769(match v0 {
                Adt7::C0(_, _, _, _, h2270, _, _) => {
                    h2270
                }
            })
        }
        Adt154::C4 => {
            f769(match v0 {
                Adt7::C0(_, _, _, _, h2271, _, _) => {
                    h2271
                }
            })
        }
        Adt154::C5 => {
            f769(match v0 {
                Adt7::C0(_, _, _, _, h2272, _, _) => {
                    h2272
                }
            })
        }
        Adt154::C6 => {
            f768(match v0 {
                Adt7::C0(_, _, _, h2273, _, _, _) => {
                    h2273
                }
            })
        }
        Adt154::C7 => {
            f768(match v0 {
                Adt7::C0(_, _, _, h2274, _, _, _) => {
                    h2274
                }
            })
        }
        Adt154::C8 => {
            f298(match v0 {
                Adt7::C0(_, _, h2275, _, _, _, _) => {
                    h2275
                }
            })
        }
        Adt154::C9 => {
            f770(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2276) => {
                    h2276
                }
            }, f33())
        }
        Adt154::C10 => {
            f770(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2277) => {
                    h2277
                }
            }, f33())
        }
        Adt154::C11 => {
            f770(match v0 {
                Adt7::C0(_, _, _, _, _, _, h2278) => {
                    h2278
                }
            }, f33())
        }
    }
}

pub fn f675(v0: Adt158, v1: Adt156) -> Result<Adt153, Adt145> {
    let m2279 = v1;
    match m2279 {
        Adt156::C0 => {
            Err::<Adt153, Adt145>(Adt145::C3(match v0 {
                Adt158::C0(_, h2280, _, _) => {
                    h2280
                }
            }))
        }
        Adt156::C1(v2) => {
            Ok::<Adt153, Adt145>(Adt153::C0(match v0.clone() {
                Adt158::C0(h2281, _, _, _) => {
                    h2281
                }
            }, v2, match v0.clone() {
                Adt158::C0(_, _, h2282, _) => {
                    h2282
                }
            }, match v0 {
                Adt158::C0(_, _, _, h2283) => {
                    h2283
                }
            }))
        }
    }
}

pub fn f676(v0: Option<Adt128>, v1: List<Str>, v2: Adt124, v3: Result<Adt20, Adt146>) -> R<Adt157> {
    let m2284 = v3;
    match m2284 {
        Ok(v5) => {
            Ok(f771(v1, v2.clone(), v5.clone(), f772(v0, f569(match v2.clone() {
                Adt124::C0(_, h2285, _) => {
                    h2285
                }
            }), match v2.clone() {
                Adt124::C0(_, h2286, _) => {
                    h2286
                }
            }, match v5 {
                Adt20::C0(h2287, _, _, _, _, _) => {
                    h2287
                }
            }, f773(match v2 {
                Adt124::C0(_, _, h2288) => {
                    h2288
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt157::C1(Adt145::C0(v4)))
        }
    }
}

pub fn f677(v0: Option<Adt8>) -> Result<Adt20, Adt146> {
    let m2289 = v0;
    match m2289 {
        None => {
            Err::<Adt20, Adt146>(Adt146::C0)
        }
        Some(v1) => {
            let m2290 = f235(Str::lit("_meta"), v1);
            match m2290 {
                Adt116::C0 => {
                    Err::<Adt20, Adt146>(Adt146::C1)
                }
                Adt116::C1(v2) => {
                    let m2291 = f236(v2);
                    match m2291 {
                        None => {
                            Err::<Adt20, Adt146>(Adt146::C4)
                        }
                        Some(v3) => {
                            f774(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f678(v0: Adt154) -> Adt159 {
    let m2292 = v0;
    match m2292 {
        Adt154::C0 => {
            Adt159::C0
        }
        Adt154::C1 => {
            Adt159::C0
        }
        Adt154::C2 => {
            Adt159::C1
        }
        Adt154::C3 => {
            Adt159::C0
        }
        Adt154::C4 => {
            Adt159::C2
        }
        Adt154::C5 => {
            Adt159::C0
        }
        Adt154::C6 => {
            Adt159::C0
        }
        Adt154::C7 => {
            Adt159::C1
        }
        Adt154::C8 => {
            Adt159::C0
        }
        Adt154::C9 => {
            Adt159::C3
        }
        Adt154::C10 => {
            Adt159::C3
        }
        Adt154::C11 => {
            Adt159::C3
        }
    }
}

pub fn f679(p2298: List<u64>, p2299: List<u64>) -> Option<List<u64>> {
    let m2297 = (p2298, p2299);
    {
        let mut m2297 = m2297;
        loop {
            let step = {
                let (v0, v1) = m2297;
                let m2293 = v0;
                match m2293.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2294 = v1;
                        match m2294.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2295 = v4;
                                let a2296 = v2;
                                if nat_eq(a2295, a2296) {
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
                Ok(next) => { m2297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f680() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f681() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f682(v0: Adt15, v1: Option<Str>) -> bool {
    let m2300 = v1;
    match m2300 {
        None => {
            false
        }
        Some(v2) => {
            let a2302 = match v0 {
                Adt15::C0(h2301, _, _, _) => {
                    h2301
                }
            };
            let a2303 = v2;
            equal(a2302, a2303)
        }
    }
}

pub fn f683(v0: List<Adt100>, v1: Str) -> R<Adt132> {
    let m2306 = f494(f270(v0, {
        let a2304 = f775();
        let a2305 = f54(v1)?;
        append_list(a2304, a2305)
    })?)?;
    match m2306 {
        Adt120::C0 => {
            Ok(Adt132::C0)
        }
        Adt120::C1 => {
            Ok(Adt132::C1)
        }
        Adt120::C2 => {
            Ok(Adt132::C1)
        }
        Adt120::C3(v2) => {
            let m2307 = f471(v2);
            match m2307 {
                None => {
                    Ok(Adt132::C1)
                }
                Some(v3) => {
                    Ok(Adt132::C2(v3))
                }
            }
        }
    }
}

pub fn f684(v0: i64) -> bool {
    let a2314 = {
        let m2310 = {
            let a2308 = v0;
            let a2309 = 0i64;
            compare(a2308, a2309)
        };
        if match m2310 {
            Ordering::Less => {
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
            let m2313 = {
                let a2311 = 0i64;
                let a2312 = v0;
                compare(a2311, a2312)
            };
            match m2313 {
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
    bool_not(a2314)
}

pub fn f685(v0: Adt135, v1: u64, v2: Str) -> Adt149 {
    let m2316 = match v0.clone() {
        Adt135::C0(_, h2315, _) => {
            h2315
        }
    };
    match m2316.uncons() {
        None => {
            f513(Adt117::C2(v1))
        }
        Some((v3, v4)) => {
            let m2317 = v3;
            match m2317 {
                Adt140::C0(_) => {
                    f513(Adt117::C2(v1))
                }
                Adt140::C1(v6, _) => {
                    Adt149::C1(Adt135::C0(Adt136::C5, List::cons(Adt140::C1(v6, Some(v2)), v4), match v0 {
                        Adt135::C0(_, _, h2318) => {
                            h2318
                        }
                    }))
                }
            }
        }
    }
}

pub fn f686(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2319 = v0;
    let a2320 = 128u64;
    if nat_lt(a2319, a2320) {
        Ok(List::cons(v0, v1))
    } else {
        let a2321 = v0;
        let a2322 = 2048u64;
        if nat_lt(a2321, a2322) {
            Ok(List::cons({
                let a2323 = 128u64;
                let a2324 = f776(v0, 64u64);
                nat_add(a2323, a2324)?
            }, List::cons({
                let a2325 = 192u64;
                let a2326 = f777(v0, 64u64);
                nat_add(a2325, a2326)?
            }, v1)))
        } else {
            let a2327 = v0;
            let a2328 = 65536u64;
            if nat_lt(a2327, a2328) {
                Ok(List::cons({
                    let a2329 = 128u64;
                    let a2330 = f776(v0, 64u64);
                    nat_add(a2329, a2330)?
                }, List::cons({
                    let a2331 = 128u64;
                    let a2332 = f776(f777(v0, 64u64), 64u64);
                    nat_add(a2331, a2332)?
                }, List::cons({
                    let a2333 = 224u64;
                    let a2334 = f777(v0, 4096u64);
                    nat_add(a2333, a2334)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2335 = 128u64;
                    let a2336 = f776(v0, 64u64);
                    nat_add(a2335, a2336)?
                }, List::cons({
                    let a2337 = 128u64;
                    let a2338 = f776(f777(v0, 64u64), 64u64);
                    nat_add(a2337, a2338)?
                }, List::cons({
                    let a2339 = 128u64;
                    let a2340 = f776(f777(v0, 4096u64), 64u64);
                    nat_add(a2339, a2340)?
                }, List::cons({
                    let a2341 = 240u64;
                    let a2342 = f777(v0, 262144u64);
                    nat_add(a2341, a2342)?
                }, v1)))))
            }
        }
    }
}

pub fn f687() -> u64 {
    18u64
}

pub fn f688() -> u64 {
    4294967296u64
}

pub fn f689(v0: Adt9) -> bool {
    let m2343 = f287(v0);
    match m2343 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f690(v0: Adt9) -> bool {
    let m2344 = f613(v0);
    match m2344 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f691(v0: Adt9) -> bool {
    let m2345 = f710(v0);
    match m2345 {
        None => {
            false
        }
        Some(v1) => {
            let a2349 = {
                let m2348 = {
                    let a2346 = v1;
                    let a2347 = 0i64;
                    compare(a2346, a2347)
                };
                match m2348 {
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
            bool_not(a2349)
        }
    }
}

pub fn f692(v0: Adt9) -> bool {
    let m2350 = f290(v0);
    match m2350 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f693(v0: Adt9) -> bool {
    let m2351 = f236(v0);
    match m2351 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f694(v0: Adt9) -> bool {
    let m2352 = f290(v0);
    match m2352 {
        None => {
            false
        }
        Some(v1) => {
            f778(v1)
        }
    }
}

pub fn f695(v0: Adt9) -> bool {
    let m2353 = f360(v0);
    match m2353 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f696(v0: Adt9) -> bool {
    let m2354 = f287(v0.clone());
    match m2354 {
        None => {
            let m2355 = f290(v0);
            match m2355 {
                None => {
                    false
                }
                Some(v2) => {
                    f779(v2)
                }
            }
        }
        Some(v1) => {
            f608(v1)
        }
    }
}

pub fn f697(v0: Adt9) -> bool {
    let m2356 = f290(v0);
    match m2356 {
        None => {
            false
        }
        Some(v1) => {
            let m2357 = v1;
            match m2357.uncons() {
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

pub fn f698(v0: Str, p2375: List<(Str, Adt9)>) -> Option<Adt9> {
    let m2374 = p2375;
    {
        let mut m2374 = m2374;
        loop {
            let step = {
                let v1 = m2374;
                let m2358 = v1;
                match m2358.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, Option<Adt9>>(None::<Adt9>)
                    }
                    Some((v2, v3)) => {
                        let a2362 = v0.clone();
                        let a2363 = {
                            let a2360 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2359, _) = v2.clone();
                                h2359
                            }, List::<Str>::nil()));
                            let a2361 = Str::lit("");
                            join(a2360, a2361)
                        };
                        if equal(a2362, a2363) {
                            Err::<List<(Str, Adt9)>, Option<Adt9>>(Some({
                                let (_, h2364) = v2;
                                h2364
                            }))
                        } else {
                            let m2366 = f236({
                                let (_, h2365) = v2.clone();
                                h2365
                            });
                            match m2366 {
                                None => {
                                    Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                }
                                Some(v4) => {
                                    let m2367 = f235(Str::lit("$anchor"), v4);
                                    match m2367 {
                                        Adt116::C0 => {
                                            Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                        }
                                        Adt116::C1(v5) => {
                                            let m2368 = f287(v5);
                                            match m2368 {
                                                None => {
                                                    Ok::<List<(Str, Adt9)>, Option<Adt9>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2371 = v0.clone();
                                                    let a2372 = {
                                                        let a2369 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2370 = Str::lit("");
                                                        join(a2369, a2370)
                                                    };
                                                    if equal(a2371, a2372) {
                                                        Err::<List<(Str, Adt9)>, Option<Adt9>>(Some({
                                                            let (_, h2373) = v2;
                                                            h2373
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
                Ok(next) => { m2374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f699(v0: List<(Str, Adt9)>) -> Option<Adt16> {
    let m2376 = f309(Str::lit("type"), v0);
    match m2376 {
        Adt116::C0 => {
            None::<Adt16>
        }
        Adt116::C1(v1) => {
            let m2377 = f287(v1);
            match m2377 {
                None => {
                    None::<Adt16>
                }
                Some(v2) => {
                    f780(v2)
                }
            }
        }
    }
}

pub fn f700(v0: List<u64>) -> R<List<u64>> {
    Ok(f434(f781(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f701(v0: List<u64>, p2380: List<List<u64>>) -> bool {
    let m2379 = p2380;
    {
        let mut m2379 = m2379;
        loop {
            let step = {
                let v1 = m2379;
                let m2378 = v1;
                match m2378.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f782(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(v0: Adt9) -> Str {
    let m2381 = v0;
    match m2381 {
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

pub fn f703(v0: Adt10) -> R<bool> {
    if f684(match v0.clone() {
        Adt10::C0(h2382, _) => {
            h2382
        }
    }) {
        Ok(true)
    } else {
        f783(40u64, match v0.clone() {
            Adt10::C0(h2383, _) => {
                h2383
            }
        }, match v0 {
            Adt10::C0(_, h2384) => {
                h2384
            }
        })
    }
}

pub fn f704(v0: Adt9) -> Str {
    let m2385 = f287(v0);
    match m2385 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f705(p2392: u64, p2393: List<(Adt9, Adt9)>) -> R<Adt160> {
    let m2391 = (p2392, p2393);
    {
        let mut m2391 = m2391;
        loop {
            let step = {
                let (v0, v1) = m2391;
                let m2386 = v0;
                if m2386 == 0 {
                    Err::<(u64, List<(Adt9, Adt9)>), R<Adt160>>(Ok(Adt160::C2))
                } else {
                    let v2 = m2386 - 1;
                    let m2387 = v1;
                    match m2387.uncons() {
                        None => {
                            Err::<(u64, List<(Adt9, Adt9)>), R<Adt160>>(Ok(Adt160::C0))
                        }
                        Some((v3, v4)) => {
                            let m2390 = f784({
                                let (h2388, _) = v3;
                                h2388
                            }, {
                                let (_, h2389) = v3;
                                h2389
                            }, v4)?;
                            match m2390 {
                                Adt165::C0 => {
                                    Err::<(u64, List<(Adt9, Adt9)>), R<Adt160>>(Ok(Adt160::C1))
                                }
                                Adt165::C1(v5) => {
                                    Ok::<(u64, List<(Adt9, Adt9)>), R<Adt160>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f706() -> u64 {
    67108864u64
}

pub fn f707(v0: Adt161) -> bool {
    let m2394 = v0;
    match m2394 {
        Adt161::C0 => {
            true
        }
        Adt161::C1 => {
            false
        }
        Adt161::C2 => {
            false
        }
    }
}

pub fn f708(v0: Adt10, v1: Adt10, v2: Adt161, v3: Adt161) -> R<Adt161> {
    let m2395 = v2;
    match m2395 {
        Adt161::C0 => {
            let m2397 = v3;
            match m2397 {
                Adt161::C0 => {
                    Ok(f785(Adt161::C0, f786(f787(match v0.clone() {
                        Adt10::C0(h2398, _) => {
                            h2398
                        }
                    })?, match v0 {
                        Adt10::C0(_, h2399) => {
                            h2399
                        }
                    }, f787(match v1.clone() {
                        Adt10::C0(h2400, _) => {
                            h2400
                        }
                    })?, match v1 {
                        Adt10::C0(_, h2401) => {
                            h2401
                        }
                    })?))
                }
                Adt161::C1 => {
                    Ok(Adt161::C0)
                }
                Adt161::C2 => {
                    Ok(Adt161::C0)
                }
            }
        }
        Adt161::C1 => {
            let m2396 = v3;
            match m2396 {
                Adt161::C0 => {
                    Ok(Adt161::C2)
                }
                Adt161::C1 => {
                    Ok(Adt161::C1)
                }
                Adt161::C2 => {
                    Ok(Adt161::C0)
                }
            }
        }
        Adt161::C2 => {
            let m2402 = v3;
            match m2402 {
                Adt161::C0 => {
                    Ok(Adt161::C2)
                }
                Adt161::C1 => {
                    Ok(Adt161::C2)
                }
                Adt161::C2 => {
                    f786(match v0.clone() {
                        Adt10::C0(h2403, _) => {
                            h2403
                        }
                    }, match v0 {
                        Adt10::C0(_, h2404) => {
                            h2404
                        }
                    }, match v1.clone() {
                        Adt10::C0(h2405, _) => {
                            h2405
                        }
                    }, match v1 {
                        Adt10::C0(_, h2406) => {
                            h2406
                        }
                    })
                }
            }
        }
    }
}

pub fn f709(v0: i64) -> Adt161 {
    f711(v0, 0i64)
}

pub fn f710(v0: Adt9) -> Option<i64> {
    let m2407 = v0;
    match m2407 {
        Adt9::C0 => {
            None::<i64>
        }
        Adt9::C1(_) => {
            None::<i64>
        }
        Adt9::C2(v2) => {
            if f684(match v2.clone() {
                Adt10::C0(_, h2408) => {
                    h2408
                }
            }) {
                Some(match v2 {
                    Adt10::C0(h2409, _) => {
                        h2409
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

pub fn f711(v0: i64, v1: i64) -> Adt161 {
    let m2412 = {
        let a2410 = v0;
        let a2411 = v1;
        compare(a2410, a2411)
    };
    if match m2412 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Adt161::C0
    } else {
        let m2415 = {
            let a2413 = v1;
            let a2414 = v0;
            compare(a2413, a2414)
        };
        if match m2415 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt161::C2
        } else {
            Adt161::C1
        }
    }
}

pub fn f712(p2424: List<u64>, p2425: u64) -> R<u64> {
    let m2423 = (p2424, p2425);
    {
        let mut m2423 = m2423;
        loop {
            let step = {
                let (v0, v1) = m2423;
                let m2416 = v0;
                match m2416.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2417 = v2;
                        let a2418 = 128u64;
                        if if nat_lt(a2417, a2418) {
                            true
                        } else {
                            let a2419 = 191u64;
                            let a2420 = v2;
                            nat_lt(a2419, a2420)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2421 = v1;
                                let a2422 = 1u64;
                                nat_add(a2421, a2422)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f713(v0: u64) -> R<Option<u64>> {
    let a2426 = 64u64;
    let a2427 = v0;
    if if nat_lt(a2426, a2427) {
        let a2428 = v0;
        let a2429 = 91u64;
        nat_lt(a2428, a2429)
    } else {
        false
    } {
        Ok(Some({
            let a2430 = v0;
            let a2431 = 65u64;
            nat_sub(a2430, a2431)
        }))
    } else {
        let a2432 = 96u64;
        let a2433 = v0;
        if if nat_lt(a2432, a2433) {
            let a2434 = v0;
            let a2435 = 123u64;
            nat_lt(a2434, a2435)
        } else {
            false
        } {
            Ok(Some({
                let a2436 = v0;
                let a2437 = 71u64;
                nat_sub(a2436, a2437)
            }))
        } else {
            let a2438 = 47u64;
            let a2439 = v0;
            if if nat_lt(a2438, a2439) {
                let a2440 = v0;
                let a2441 = 58u64;
                nat_lt(a2440, a2441)
            } else {
                false
            } {
                Ok(Some({
                    let a2442 = v0;
                    let a2443 = 4u64;
                    nat_add(a2442, a2443)?
                }))
            } else {
                let a2444 = v0;
                let a2445 = 43u64;
                if nat_eq(a2444, a2445) {
                    Ok(Some(62u64))
                } else {
                    let a2446 = v0;
                    let a2447 = 47u64;
                    if nat_eq(a2446, a2447) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f714(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2448 = v3;
        let a2449 = 61u64;
        if nat_eq(a2448, a2449) {
            let a2450 = f788(v1, 16u64);
            let a2451 = 0u64;
            nat_eq(a2450, a2451)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f789(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f715(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2452 = f788(v2, 4u64);
        let a2453 = 0u64;
        nat_eq(a2452, a2453)
    } else {
        false
    } {
        Ok(Some(f790(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f716(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2456 = {
            let a2454 = v0;
            let a2455 = 4u64;
            nat_mul(a2454, a2455)?
        };
        let a2457 = f791(v1, 16u64);
        nat_add(a2456, a2457)?
    }, List::cons({
        let a2460 = {
            let a2458 = f788(v1, 16u64);
            let a2459 = 16u64;
            nat_mul(a2458, a2459)?
        };
        let a2461 = f791(v2, 4u64);
        nat_add(a2460, a2461)?
    }, List::cons({
        let a2464 = {
            let a2462 = f788(v2, 4u64);
            let a2463 = 64u64;
            nat_mul(a2462, a2463)?
        };
        let a2465 = v3;
        nat_add(a2464, a2465)?
    }, List::<u64>::nil()))))
}

pub fn f717() -> i64 {
    999999999999999999i64
}

pub fn f718(v0: i64) -> R<List<u64>> {
    let m2468 = {
        let a2466 = v0;
        let a2467 = 0i64;
        compare(a2466, a2467)
    };
    if match m2468 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f792(20u64, {
            let a2469 = v0;
            int_neg(a2469)?
        }, List::<u64>::nil())?))
    } else {
        f792(20u64, v0, List::<u64>::nil())
    }
}

pub fn f719(v0: u64) -> R<List<u64>> {
    let a2470 = v0;
    let a2471 = 34u64;
    if nat_eq(a2470, a2471) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2472 = v0;
        let a2473 = 92u64;
        if nat_eq(a2472, a2473) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2474 = v0;
            let a2475 = 8u64;
            if nat_eq(a2474, a2475) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2476 = v0;
                let a2477 = 9u64;
                if nat_eq(a2476, a2477) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2478 = v0;
                    let a2479 = 10u64;
                    if nat_eq(a2478, a2479) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2480 = v0;
                        let a2481 = 12u64;
                        if nat_eq(a2480, a2481) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2482 = v0;
                            let a2483 = 13u64;
                            if nat_eq(a2482, a2483) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2484 = v0;
                                let a2485 = 32u64;
                                if nat_lt(a2484, a2485) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f793(f777(v0, 16u64))?, List::cons(f793(f776(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f720(v0: List<Str>) -> Adt9 {
    Adt9::C4(f669(v0))
}

pub fn f721(v0: Adt7) -> Adt9 {
    Adt9::C5({
        let a2503 = f763(Str::lit("experimental"), match v0.clone() {
            Adt7::C0(h2486, _, _, _, _, _, _) => {
                h2486
            }
        });
        let a2504 = {
            let a2501 = f763(Str::lit("logging"), match v0.clone() {
                Adt7::C0(_, h2487, _, _, _, _, _) => {
                    h2487
                }
            });
            let a2502 = {
                let a2499 = f763(Str::lit("completions"), match v0.clone() {
                    Adt7::C0(_, _, h2488, _, _, _, _) => {
                        h2488
                    }
                });
                let a2500 = {
                    let a2497 = f794(Str::lit("prompts"), match v0.clone() {
                        Adt7::C0(_, _, _, h2489, _, _, _) => {
                            h2489
                        }
                    });
                    let a2498 = {
                        let a2495 = f795(match v0.clone() {
                            Adt7::C0(_, _, _, _, h2490, _, _) => {
                                h2490
                            }
                        });
                        let a2496 = {
                            let a2493 = f794(Str::lit("tools"), match v0.clone() {
                                Adt7::C0(_, _, _, _, _, h2491, _) => {
                                    h2491
                                }
                            });
                            let a2494 = f763(Str::lit("extensions"), match v0 {
                                Adt7::C0(_, _, _, _, _, _, h2492) => {
                                    h2492
                                }
                            });
                            append_list(a2493, a2494)
                        };
                        append_list(a2495, a2496)
                    };
                    append_list(a2497, a2498)
                };
                append_list(a2499, a2500)
            };
            append_list(a2501, a2502)
        };
        append_list(a2503, a2504)
    })
}

pub fn f722(v0: Str, v1: Option<Str>) -> List<(Str, Adt9)> {
    let m2505 = v1;
    match m2505 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C3(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f723(v0: Adt13) -> R<List<(Str, Adt9)>> {
    Ok(List::cons((Str::lit("ttlMs"), f120(match v0.clone() {
        Adt13::C0(h2506, _) => {
            h2506
        }
    })?), List::cons((Str::lit("cacheScope"), Adt9::C3(f796(match v0 {
        Adt13::C0(_, h2507) => {
            h2507
        }
    }))), List::<(Str, Adt9)>::nil())))
}

pub fn f724(v0: List<Adt39>) -> List<Adt9> {
    f581(f797(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f725(v0: List<Adt42>) -> R<Option<List<Adt9>>> {
    let m2508 = f798(v0, List::<Adt9>::nil())?;
    match m2508 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f581(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f726(v0: Str, v1: Option<Adt9>) -> List<(Str, Adt9)> {
    let m2509 = v1;
    match m2509 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f727(v0: Str, v1: Option<bool>) -> List<(Str, Adt9)> {
    let m2510 = v1;
    match m2510 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C1(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f728(v0: List<Adt48>) -> R<List<Adt9>> {
    Ok(f581(f799(v0, List::<Adt9>::nil())?, List::<Adt9>::nil()))
}

pub fn f729(v0: List<Adt50>) -> List<Adt9> {
    f581(f800(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f730(v0: List<Adt58>) -> List<Adt9> {
    f581(f801(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f731(v0: List<Adt60>) -> List<Adt9> {
    f581(f802(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f732(v0: List<Adt63>) -> R<Option<List<Adt9>>> {
    let m2511 = f803(v0, List::<Adt9>::nil())?;
    match m2511 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f581(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f733(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt9)>> {
    let m2512 = v1;
    match m2512 {
        None => {
            Ok(List::<(Str, Adt9)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f120(v2)?), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f734(p2518: List<(Str, Adt68)>, p2519: List<(Str, Adt9)>) -> R<Option<List<(Str, Adt9)>>> {
    let m2517 = (p2518, p2519);
    {
        let mut m2517 = m2517;
        loop {
            let step = {
                let (v0, v1) = m2517;
                let m2513 = v0;
                match m2513.uncons() {
                    None => {
                        Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2515 = f804({
                            let (_, h2514) = v2;
                            h2514
                        })?;
                        match m2515 {
                            None => {
                                Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>(Ok(None::<List<(Str, Adt9)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt68)>, List<(Str, Adt9)>), R<Option<List<(Str, Adt9)>>>>((v3, List::cons(({
                                    let (h2516, _) = v2;
                                    h2516
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f735(v0: Adt85) -> List<(Str, Adt68)> {
    let m2520 = v0;
    match m2520 {
        Adt85::C0(v1) => {
            v1
        }
        Adt85::C1(_) => {
            List::<(Str, Adt68)>::nil()
        }
        Adt85::C2(v3, _) => {
            v3
        }
    }
}

pub fn f736(v0: List<(Str, Adt9)>) -> List<(Str, Adt9)> {
    let m2521 = v0.clone();
    match m2521.uncons() {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt9::C5(f583(v0, List::<(Str, Adt9)>::nil()))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f737(v0: Option<Str>) -> List<(Str, Adt9)> {
    let m2522 = v0;
    match m2522 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f738(v0: Adt85) -> Option<Str> {
    let m2523 = v0;
    match m2523 {
        Adt85::C0(_) => {
            None::<Str>
        }
        Adt85::C1(v2) => {
            Some(v2)
        }
        Adt85::C2(_, v4) => {
            Some(v4)
        }
    }
}

pub fn f739(v0: Adt66) -> Option<List<(Str, Adt9)>> {
    let m2524 = v0;
    match m2524 {
        Adt66::C0(_) => {
            Some(List::<(Str, Adt9)>::nil())
        }
        Adt66::C1(_, v3) => {
            let m2525 = f805(v3);
            match m2525 {
                None => {
                    None::<List<(Str, Adt9)>>
                }
                Some(v4) => {
                    Some(List::cons((Str::lit("inputRequests"), v4), List::<(Str, Adt9)>::nil()))
                }
            }
        }
        Adt66::C2(_, v6) => {
            Some(List::cons((Str::lit("result"), Adt9::C5(match v6 {
                Adt8::C0(h2526) => {
                    h2526
                }
            })), List::<(Str, Adt9)>::nil()))
        }
        Adt66::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f379(v8)), List::<(Str, Adt9)>::nil()))
        }
        Adt66::C4(_) => {
            Some(List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f740(v0: Adt67, v1: Str) -> List<(Str, Adt9)> {
    List::cons((Str::lit("taskId"), Adt9::C3(match v0.clone() {
        Adt67::C0(h2527, _, _, _, _, _) => {
            h2527
        }
    })), List::cons((Str::lit("status"), Adt9::C3(v1)), {
        let a2533 = f806(Str::lit("statusMessage"), match v0.clone() {
            Adt67::C0(_, h2528, _, _, _, _) => {
                h2528
            }
        });
        let a2534 = List::cons((Str::lit("createdAt"), Adt9::C3(match v0.clone() {
            Adt67::C0(_, _, h2529, _, _, _) => {
                h2529
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt9::C3(match v0.clone() {
            Adt67::C0(_, _, _, h2530, _, _) => {
                h2530
            }
        })), List::cons((Str::lit("ttlMs"), f807(match v0.clone() {
            Adt67::C0(_, _, _, _, h2531, _) => {
                h2531
            }
        })), f808(Str::lit("pollIntervalMs"), match v0 {
            Adt67::C0(_, _, _, _, _, h2532) => {
                h2532
            }
        }))));
        append_list(a2533, a2534)
    }))
}

pub fn f741(v0: Adt66) -> Adt67 {
    let m2535 = v0;
    match m2535 {
        Adt66::C0(v1) => {
            v1
        }
        Adt66::C1(v2, _) => {
            v2
        }
        Adt66::C2(v4, _) => {
            v4
        }
        Adt66::C3(v6, _) => {
            v6
        }
        Adt66::C4(v8) => {
            v8
        }
    }
}

pub fn f742(v0: Adt87) -> Str {
    let m2536 = v0;
    match m2536 {
        Adt87::C0 => {
            Str::lit("working")
        }
        Adt87::C1 => {
            Str::lit("input_required")
        }
        Adt87::C2 => {
            Str::lit("completed")
        }
        Adt87::C3 => {
            Str::lit("failed")
        }
        Adt87::C4 => {
            Str::lit("cancelled")
        }
    }
}

pub fn f743(v0: Adt66) -> Adt87 {
    let m2537 = v0;
    match m2537 {
        Adt66::C0(_) => {
            Adt87::C0
        }
        Adt66::C1(_, _) => {
            Adt87::C1
        }
        Adt66::C2(_, _) => {
            Adt87::C2
        }
        Adt66::C3(_, _) => {
            Adt87::C3
        }
        Adt66::C4(_) => {
            Adt87::C4
        }
    }
}

pub fn f744() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f745(v0: Adt4) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt4::C0(h2538, _, _, _, _, _) => {
            h2538
        }
    })), List::cons((Str::lit("version"), Adt9::C3(match v0.clone() {
        Adt4::C0(_, _, h2539, _, _, _) => {
            h2539
        }
    })), {
        let a2548 = f722(Str::lit("title"), match v0.clone() {
            Adt4::C0(_, h2540, _, _, _, _) => {
                h2540
            }
        });
        let a2549 = {
            let a2546 = f722(Str::lit("description"), match v0.clone() {
                Adt4::C0(_, _, _, h2541, _, _) => {
                    h2541
                }
            });
            let a2547 = {
                let a2544 = f722(Str::lit("websiteUrl"), match v0.clone() {
                    Adt4::C0(_, _, _, _, h2542, _) => {
                        h2542
                    }
                });
                let a2545 = f809(match v0 {
                    Adt4::C0(_, _, _, _, _, h2543) => {
                        h2543
                    }
                });
                append_list(a2544, a2545)
            };
            append_list(a2546, a2547)
        };
        append_list(a2548, a2549)
    })))
}

pub fn f746(p2553: List<Adt9>, p2554: List<Adt88>) -> Option<List<Adt88>> {
    let m2552 = (p2553, p2554);
    {
        let mut m2552 = m2552;
        loop {
            let step = {
                let (v0, v1) = m2552;
                let m2550 = v0;
                match m2550.uncons() {
                    None => {
                        Err::<(List<Adt9>, List<Adt88>), Option<List<Adt88>>>(Some(f748(v1, List::<Adt88>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2551 = f810(v2);
                        match m2551 {
                            None => {
                                Err::<(List<Adt9>, List<Adt88>), Option<List<Adt88>>>(None::<List<Adt88>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt9>, List<Adt88>), Option<List<Adt88>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f747(v0: List<Adt88>) -> Option<Adt9> {
    let m2555 = f811(v0, List::<Adt9>::nil());
    match m2555 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C4(f581(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f748(p2558: List<Adt88>, p2559: List<Adt88>) -> List<Adt88> {
    let m2557 = (p2558, p2559);
    {
        let mut m2557 = m2557;
        loop {
            let step = {
                let (v0, v1) = m2557;
                let m2556 = v0;
                match m2556.uncons() {
                    None => {
                        Err::<(List<Adt88>, List<Adt88>), List<Adt88>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt88>, List<Adt88>), List<Adt88>>((v3, List::cons(v2, v1)))
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

pub fn f749(v0: Str, p2564: List<Adt88>, v2: i64, p2565: List<Adt88>) -> R<List<Adt88>> {
    let m2563 = (p2564, p2565);
    {
        let mut m2563 = m2563;
        loop {
            let step = {
                let (v1, v3) = m2563;
                let m2560 = v1;
                match m2560.uncons() {
                    None => {
                        Err::<(List<Adt88>, List<Adt88>), R<List<Adt88>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a2561 = f750(v4.clone());
                        let a2562 = v0.clone();
                        if if equal(a2561, a2562) {
                            true
                        } else {
                            f812(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt88>, List<Adt88>), R<List<Adt88>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt88>, List<Adt88>), R<List<Adt88>>>((v5, List::cons(v4, v3)))
                        }
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

pub fn f750(v0: Adt88) -> Str {
    let m2568 = f741(match v0 {
        Adt88::C0(h2567, _, _) => {
            h2567
        }
    });
    match m2568 {
        Adt67::C0(h2566, _, _, _, _, _) => {
            h2566
        }
    }
}

pub fn f751(v0: List<List<u64>>) -> bool {
    let m2569 = v0;
    match m2569.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f752() -> List<Adt100> {
    List::cons(Adt100::C0(f813(), f672()), List::cons(Adt100::C0(f814(), f815()), List::cons(Adt100::C0(f816(), f817()), List::<Adt100>::nil())))
}

pub fn f753(v0: List<List<u64>>) -> List<u64> {
    f818(f176(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f754(p2584: u64, p2585: Adt163, p2586: List<Adt164>) -> R<Option<Adt9>> {
    let m2583 = (p2584, (p2585, p2586));
    {
        let mut m2583 = m2583;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2583;
                let m2570 = v0;
                if m2570 == 0 {
                    Err::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>(Ok(None::<Adt9>))
                } else {
                    let v3 = m2570 - 1;
                    let m2571 = v1;
                    match m2571 {
                        Adt163::C0(v4) => {
                            let m2572 = v4.clone();
                            match m2572 {
                                Adt9::C0 => {
                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(v4), v2)))
                                }
                                Adt9::C1(_) => {
                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(v4), v2)))
                                }
                                Adt9::C2(_) => {
                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(v4), v2)))
                                }
                                Adt9::C3(v7) => {
                                    let v8: Str = f819(v7)?;
                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(Adt9::C3(v8)), v2)))
                                }
                                Adt9::C4(v9) => {
                                    let m2573 = v9;
                                    match m2573.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C0(v10), List::cons(Adt164::C0(List::<Adt9>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt9::C5(v12) => {
                                    let m2574 = v12;
                                    match m2574.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f819({
                                                let (h2575, _) = v13;
                                                h2575
                                            })?;
                                            Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C0({
                                                let (_, h2576) = v13;
                                                h2576
                                            }), List::cons(Adt164::C1(v15, List::<(Str, Adt9)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt163::C1(v16) => {
                            let m2577 = v2;
                            match m2577.uncons() {
                                None => {
                                    Err::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m2578 = v17;
                                    match m2578 {
                                        Adt164::C0(v19, v20) => {
                                            let v21: List<Adt9> = List::cons(v16, v19);
                                            let m2579 = v20;
                                            match m2579.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(Adt9::C4(f581(v21, List::<Adt9>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C0(v22), List::cons(Adt164::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt164::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt9)> = List::cons((v24, v16), v25);
                                            let m2580 = v26;
                                            match m2580.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C1(Adt9::C5(f583(v27, List::<(Str, Adt9)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f819({
                                                        let (h2581, _) = v28;
                                                        h2581
                                                    })?;
                                                    Ok::<(u64, (Adt163, List<Adt164>)), R<Option<Adt9>>>((v3, (Adt163::C0({
                                                        let (_, h2582) = v28;
                                                        h2582
                                                    }), List::cons(Adt164::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m2583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f755() -> u64 {
    67108864u64
}

pub fn f756(v0: Adt23) -> Adt9 {
    let m2587 = v0;
    match m2587 {
        Adt23::C0(v1) => {
            Adt9::C3(v1)
        }
        Adt23::C1(v2) => {
            Adt9::C2(Adt10::C0(v2, 0i64))
        }
    }
}

pub fn f757(v0: Option<Adt10>) -> List<(Str, Adt9)> {
    let m2588 = v0;
    match m2588 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt9::C2(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f758(v0: Option<Str>) -> List<(Str, Adt9)> {
    let m2589 = v0;
    match m2589 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt9::C3(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f759() -> i64 {
    -32603i64
}

pub fn f760() -> i64 {
    429i64
}

pub fn f761() -> i64 {
    -32021i64
}

pub fn f762(p2592: List<Str>, p2593: List<Adt9>) -> List<Adt9> {
    let m2591 = (p2592, p2593);
    {
        let mut m2591 = m2591;
        loop {
            let step = {
                let (v0, v1) = m2591;
                let m2590 = v0;
                match m2590.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C3(v2), v1)))
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

pub fn f763(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt9)> {
    let m2594 = v1;
    match m2594 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C5(match v2 {
                Adt8::C0(h2595) => {
                    h2595
                }
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f764(v0: List<u64>) -> R<bool> {
    let m2596 = v0;
    match m2596.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2597 = v2;
            match m2597.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2598 = f454(v1)?;
                    let a2599 = 113u64;
                    if if nat_eq(a2598, a2599) {
                        let a2600 = v3;
                        let a2601 = 61u64;
                        nat_eq(a2600, a2601)
                    } else {
                        false
                    } {
                        Ok(f820(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f765(v0: Adt3, v1: Fn3, v2: u64, v3: Result<Adt18, Adt146>) -> R<Result<Adt142, Adt145>> {
    let m2602 = v3;
    match m2602 {
        Ok(v5) => {
            f821(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt142, Adt145>(Adt145::C0(v4)))
        }
    }
}

pub fn f766(v0: Adt154, v1: Adt20, v2: Adt8) -> Result<Adt18, Adt146> {
    let m2603 = v0;
    match m2603 {
        Adt154::C0 => {
            f822(v1, v2)
        }
        Adt154::C1 => {
            f823(v1, v2)
        }
        Adt154::C2 => {
            f824(v1, v2)
        }
        Adt154::C3 => {
            f825(v1, v2)
        }
        Adt154::C4 => {
            f826(v1, v2)
        }
        Adt154::C5 => {
            f827(v1, v2)
        }
        Adt154::C6 => {
            f828(v1, v2)
        }
        Adt154::C7 => {
            f829(v1, v2)
        }
        Adt154::C8 => {
            f830(v1, v2)
        }
        Adt154::C9 => {
            f831(v1, v2)
        }
        Adt154::C10 => {
            f832(v1, v2)
        }
        Adt154::C11 => {
            f833(v1, v2)
        }
    }
}

pub fn f767(v0: Adt154) -> Str {
    let m2604 = v0;
    match m2604 {
        Adt154::C0 => {
            Str::lit("server/discover")
        }
        Adt154::C1 => {
            Str::lit("tools/list")
        }
        Adt154::C2 => {
            Str::lit("tools/call")
        }
        Adt154::C3 => {
            Str::lit("resources/list")
        }
        Adt154::C4 => {
            Str::lit("resources/read")
        }
        Adt154::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt154::C6 => {
            Str::lit("prompts/list")
        }
        Adt154::C7 => {
            Str::lit("prompts/get")
        }
        Adt154::C8 => {
            Str::lit("completion/complete")
        }
        Adt154::C9 => {
            Str::lit("tasks/get")
        }
        Adt154::C10 => {
            Str::lit("tasks/update")
        }
        Adt154::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f768(v0: Option<Adt11>) -> bool {
    let m2605 = v0;
    match m2605 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f769(v0: Option<Adt12>) -> bool {
    let m2606 = v0;
    match m2606 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f770(v0: Option<Adt8>, v1: Str) -> bool {
    let m2607 = v0;
    match m2607 {
        None => {
            false
        }
        Some(v2) => {
            f501(v1, v2)
        }
    }
}

pub fn f771(v0: List<Str>, v1: Adt124, v2: Adt20, v3: Option<Adt129>) -> Adt157 {
    let m2608 = v3;
    match m2608 {
        None => {
            f834(v1, v2.clone(), f835(match v2 {
                Adt20::C0(h2609, _, _, _, _, _) => {
                    h2609
                }
            }, v0))
        }
        Some(v4) => {
            Adt157::C1(Adt145::C2(v4))
        }
    }
}

pub fn f772(v0: Option<Adt128>, v1: Adt159, v2: Str, v3: Str, v4: Adt8) -> R<Option<Adt129>> {
    let m2610 = v0;
    match m2610 {
        None => {
            Ok(None::<Adt129>)
        }
        Some(v5) => {
            let m2612 = match v5.clone() {
                Adt128::C0(_, _, _, h2611, _) => {
                    h2611
                }
            };
            match m2612 {
                None => {
                    f836(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f773(v0: Option<Adt8>) -> Adt8 {
    let m2613 = v0;
    match m2613 {
        None => {
            f32()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f774(v0: Adt8) -> Result<Adt20, Adt146> {
    let m2614 = f837(v0.clone());
    match m2614 {
        Adt166::C0 => {
            f838(v0.clone(), f235(f839(), v0.clone()), f235(f840(), v0))
        }
        Adt166::C1 => {
            Err::<Adt20, Adt146>(Adt146::C2)
        }
        Adt166::C2 => {
            Err::<Adt20, Adt146>(Adt146::C3)
        }
    }
}

pub fn f775() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f776(v0: u64, v1: u64) -> u64 {
    let a2615 = v0;
    let a2616 = v1;
    let a2617 = 0u64;
    nat_rem(a2615, a2616, a2617)
}

pub fn f777(v0: u64, v1: u64) -> u64 {
    let a2618 = v0;
    let a2619 = v1;
    let a2620 = 0u64;
    nat_quot(a2618, a2619, a2620)
}

pub fn f778(p2624: List<Adt9>) -> bool {
    let m2623 = p2624;
    {
        let mut m2623 = m2623;
        loop {
            let step = {
                let v0 = m2623;
                let m2621 = v0;
                match m2621.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2622 = f287(v1);
                        match m2622 {
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
                Ok(next) => { m2623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f779(p2628: List<Adt9>) -> bool {
    let m2627 = p2628;
    {
        let mut m2627 = m2627;
        loop {
            let step = {
                let v0 = m2627;
                let m2625 = v0;
                match m2625.uncons() {
                    None => {
                        Err::<List<Adt9>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2626 = f287(v1);
                        match m2626 {
                            None => {
                                Err::<List<Adt9>, bool>(false)
                            }
                            Some(v3) => {
                                if f608(v3) {
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
                Ok(next) => { m2627 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f780(v0: Str) -> Option<Adt16> {
    let a2629 = v0.clone();
    let a2630 = Str::lit("string");
    if equal(a2629, a2630) {
        Some(Adt16::C0)
    } else {
        let a2631 = v0.clone();
        let a2632 = Str::lit("integer");
        if equal(a2631, a2632) {
            Some(Adt16::C1)
        } else {
            let a2633 = v0;
            let a2634 = Str::lit("boolean");
            if equal(a2633, a2634) {
                Some(Adt16::C2)
            } else {
                None::<Adt16>
            }
        }
    }
}

pub fn f781(p2643: List<u64>, p2644: List<u64>) -> R<List<u64>> {
    let m2642 = (p2643, p2644);
    {
        let mut m2642 = m2642;
        loop {
            let step = {
                let (v0, v1) = m2642;
                let m2635 = v0;
                match m2635.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2636 = 65u64;
                        let a2637 = v2;
                        if if nat_le(a2636, a2637) {
                            let a2638 = v2;
                            let a2639 = 90u64;
                            nat_le(a2638, a2639)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2640 = v2;
                                let a2641 = 32u64;
                                nat_add(a2640, a2641)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f782(p2651: List<u64>, p2652: List<u64>) -> bool {
    let m2650 = (p2651, p2652);
    {
        let mut m2650 = m2650;
        loop {
            let step = {
                let (v0, v1) = m2650;
                let m2645 = v0;
                match m2645.uncons() {
                    None => {
                        let m2646 = v1;
                        match m2646.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2647 = v1;
                        match m2647.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2648 = v4;
                                let a2649 = v6;
                                if nat_eq(a2648, a2649) {
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
                Ok(next) => { m2650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f783(p2664: u64, p2665: i64, p2666: i64) -> R<bool> {
    let m2663 = (p2664, (p2665, p2666));
    {
        let mut m2663 = m2663;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2663;
                let m2653 = v0;
                if m2653 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2653 - 1;
                    let m2656 = {
                        let a2654 = v2;
                        let a2655 = 0i64;
                        compare(a2654, a2655)
                    };
                    if match m2656 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f684({
                            let a2657 = v1;
                            let a2658 = 10i64;
                            let a2659 = 0i64;
                            int_rem(a2657, a2658, a2659)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2660 = v1;
                                let a2661 = 10i64;
                                let a2662 = 0i64;
                                int_quot(a2660, a2661, a2662)?
                            }, f359(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
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

pub fn f784(v0: Adt9, v1: Adt9, v2: List<(Adt9, Adt9)>) -> R<Adt165> {
    let a2667 = f841(v0.clone());
    let a2668 = f841(v1.clone());
    if nat_eq(a2667, a2668) {
        let m2669 = v0;
        match m2669 {
            Adt9::C0 => {
                Ok(Adt165::C1(v2))
            }
            Adt9::C1(v3) => {
                let m2670 = f360(v1);
                match m2670 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v4) => {
                        let a2671 = v3;
                        let a2672 = v4;
                        if equal(a2671, a2672) {
                            Ok(Adt165::C1(v2))
                        } else {
                            Ok(Adt165::C0)
                        }
                    }
                }
            }
            Adt9::C2(v5) => {
                let m2673 = f613(v1);
                match m2673 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v6) => {
                        if f842(f615(v5, v6)?) {
                            Ok(Adt165::C1(v2))
                        } else {
                            Ok(Adt165::C0)
                        }
                    }
                }
            }
            Adt9::C3(v7) => {
                let m2674 = f287(v1);
                match m2674 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v8) => {
                        let a2675 = v7;
                        let a2676 = v8;
                        if equal(a2675, a2676) {
                            Ok(Adt165::C1(v2))
                        } else {
                            Ok(Adt165::C0)
                        }
                    }
                }
            }
            Adt9::C4(v9) => {
                let m2677 = f290(v1);
                match m2677 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v10) => {
                        Ok(f843(f844(v9, v10, List::<(Adt9, Adt9)>::nil()), v2))
                    }
                }
            }
            Adt9::C5(v11) => {
                let m2678 = f236(v1);
                match m2678 {
                    None => {
                        Ok(Adt165::C0)
                    }
                    Some(v12) => {
                        let a2680 = f619(v11.clone(), 0u64)?;
                        let a2681 = f619(match v12.clone() {
                            Adt8::C0(h2679) => {
                                h2679
                            }
                        }, 0u64)?;
                        if nat_eq(a2680, a2681) {
                            Ok(f843(f845(v11, match v12 {
                                Adt8::C0(h2682) => {
                                    h2682
                                }
                            }, List::<(Adt9, Adt9)>::nil()), v2))
                        } else {
                            Ok(Adt165::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt165::C0)
    }
}

pub fn f785(v0: Adt161, v1: Adt161) -> Adt161 {
    let m2683 = v0;
    match m2683 {
        Adt161::C0 => {
            f846(v1)
        }
        Adt161::C1 => {
            Adt161::C1
        }
        Adt161::C2 => {
            v1
        }
    }
}

pub fn f786(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt161> {
    let m2684 = f711(f847(v0, v1)?, f847(v2, v3)?);
    match m2684 {
        Adt161::C0 => {
            Ok(Adt161::C0)
        }
        Adt161::C1 => {
            f848(v0, f849(v0)?, v2, f849(v2)?)
        }
        Adt161::C2 => {
            Ok(Adt161::C2)
        }
    }
}

pub fn f787(v0: i64) -> R<i64> {
    let m2687 = {
        let a2685 = v0;
        let a2686 = 0i64;
        compare(a2685, a2686)
    };
    if match m2687 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a2688 = v0;
        int_neg(a2688)
    } else {
        Ok(v0)
    }
}

pub fn f788(v0: u64, v1: u64) -> u64 {
    let a2689 = v0;
    let a2690 = v1;
    let a2691 = 0u64;
    nat_rem(a2689, a2690, a2691)
}

pub fn f789(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2694 = {
            let a2692 = v0;
            let a2693 = 4u64;
            nat_mul(a2692, a2693)?
        };
        let a2695 = f791(v1, 16u64);
        nat_add(a2694, a2695)?
    }, List::<u64>::nil()))
}

pub fn f790(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2698 = {
            let a2696 = v0;
            let a2697 = 4u64;
            nat_mul(a2696, a2697)?
        };
        let a2699 = f791(v1, 16u64);
        nat_add(a2698, a2699)?
    }, List::cons({
        let a2702 = {
            let a2700 = f788(v1, 16u64);
            let a2701 = 16u64;
            nat_mul(a2700, a2701)?
        };
        let a2703 = f791(v2, 4u64);
        nat_add(a2702, a2703)?
    }, List::<u64>::nil())))
}

pub fn f791(v0: u64, v1: u64) -> u64 {
    let a2704 = v0;
    let a2705 = v1;
    let a2706 = 0u64;
    nat_quot(a2704, a2705, a2706)
}

pub fn f792(p2718: u64, p2719: i64, p2720: List<u64>) -> R<List<u64>> {
    let m2717 = (p2718, (p2719, p2720));
    {
        let mut m2717 = m2717;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2717;
                let m2707 = v0;
                if m2707 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2707 - 1;
                    let v4: i64 = {
                        let a2708 = v1;
                        let a2709 = 10i64;
                        let a2710 = 0i64;
                        int_quot(a2708, a2709, a2710)?
                    };
                    let v5: i64 = {
                        let a2711 = v1;
                        let a2712 = 10i64;
                        let a2713 = 0i64;
                        int_rem(a2711, a2712, a2713)
                    };
                    let m2716 = {
                        let a2714 = v4;
                        let a2715 = 1i64;
                        compare(a2714, a2715)
                    };
                    if match m2716 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f850(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f850(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2717 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f793(v0: u64) -> R<u64> {
    let a2721 = v0;
    let a2722 = 10u64;
    if nat_lt(a2721, a2722) {
        let a2723 = 48u64;
        let a2724 = v0;
        nat_add(a2723, a2724)
    } else {
        let a2725 = 87u64;
        let a2726 = v0;
        nat_add(a2725, a2726)
    }
}

pub fn f794(v0: Str, v1: Option<Adt11>) -> List<(Str, Adt9)> {
    let m2727 = v1;
    match m2727 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f851(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f795(v0: Option<Adt12>) -> List<(Str, Adt9)> {
    let m2728 = v0;
    match m2728 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt9::C5({
                let a2731 = f727(Str::lit("subscribe"), match v1.clone() {
                    Adt12::C0(h2729, _) => {
                        h2729
                    }
                });
                let a2732 = f727(Str::lit("listChanged"), match v1 {
                    Adt12::C0(_, h2730) => {
                        h2730
                    }
                });
                append_list(a2731, a2732)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f796(v0: Adt14) -> Str {
    let m2733 = v0;
    match m2733 {
        Adt14::C0 => {
            Str::lit("public")
        }
        Adt14::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f797(p2736: List<Adt39>, p2737: List<Adt9>) -> List<Adt9> {
    let m2735 = (p2736, p2737);
    {
        let mut m2735 = m2735;
        loop {
            let step = {
                let (v0, v1) = m2735;
                let m2734 = v0;
                match m2734.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt39>, List<Adt9>), List<Adt9>>((v3, List::cons(f852(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f798(p2741: List<Adt42>, p2742: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m2740 = (p2741, p2742);
    {
        let mut m2740 = m2740;
        loop {
            let step = {
                let (v0, v1) = m2740;
                let m2738 = v0;
                match m2738.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2739 = f853(v2)?;
                        match m2739 {
                            None => {
                                Err::<(List<Adt42>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(None::<List<Adt9>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt42>, List<Adt9>), R<Option<List<Adt9>>>>((v3, List::cons(v4, v1)))
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

pub fn f799(p2745: List<Adt48>, p2746: List<Adt9>) -> R<List<Adt9>> {
    let m2744 = (p2745, p2746);
    {
        let mut m2744 = m2744;
        loop {
            let step = {
                let (v0, v1) = m2744;
                let m2743 = v0;
                match m2743.uncons() {
                    None => {
                        Err::<(List<Adt48>, List<Adt9>), R<List<Adt9>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt48>, List<Adt9>), R<List<Adt9>>>((v3, List::cons(f854(v2)?, v1)))
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

pub fn f800(p2749: List<Adt50>, p2750: List<Adt9>) -> List<Adt9> {
    let m2748 = (p2749, p2750);
    {
        let mut m2748 = m2748;
        loop {
            let step = {
                let (v0, v1) = m2748;
                let m2747 = v0;
                match m2747.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt50>, List<Adt9>), List<Adt9>>((v3, List::cons(f855(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f801(p2753: List<Adt58>, p2754: List<Adt9>) -> List<Adt9> {
    let m2752 = (p2753, p2754);
    {
        let mut m2752 = m2752;
        loop {
            let step = {
                let (v0, v1) = m2752;
                let m2751 = v0;
                match m2751.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt9>), List<Adt9>>((v3, List::cons(f856(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f802(p2757: List<Adt60>, p2758: List<Adt9>) -> List<Adt9> {
    let m2756 = (p2757, p2758);
    {
        let mut m2756 = m2756;
        loop {
            let step = {
                let (v0, v1) = m2756;
                let m2755 = v0;
                match m2755.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt60>, List<Adt9>), List<Adt9>>((v3, List::cons(f857(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f803(p2762: List<Adt63>, p2763: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m2761 = (p2762, p2763);
    {
        let mut m2761 = m2761;
        loop {
            let step = {
                let (v0, v1) = m2761;
                let m2759 = v0;
                match m2759.uncons() {
                    None => {
                        Err::<(List<Adt63>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2760 = f858(v2)?;
                        match m2760 {
                            None => {
                                Err::<(List<Adt63>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(None::<List<Adt9>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt63>, List<Adt9>), R<Option<List<Adt9>>>>((v3, List::cons(v4, v1)))
                            }
                        }
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

pub fn f804(v0: Adt68) -> R<Option<Adt9>> {
    let m2764 = v0;
    match m2764 {
        Adt68::C0(v1) => {
            Ok(Some(f859(Str::lit("elicitation/create"), Some(f860(v1)))))
        }
        Adt68::C1(v2) => {
            let m2765 = f861(v2)?;
            match m2765 {
                None => {
                    Ok(None::<Adt9>)
                }
                Some(v3) => {
                    Ok(Some(f859(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt68::C2(v4) => {
            Ok(Some(f859(Str::lit("roots/list"), {
                let m2766 = v4;
                match m2766 {
                    None => {
                        None::<Adt9>
                    }
                    Some(v5) => {
                        Some(Adt9::C5(match v5 {
                            Adt8::C0(h2767) => {
                                h2767
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f805(v0: List<(Str, Adt68)>) -> Option<Adt9> {
    let m2768 = f862(v0, List::<(Str, Adt9)>::nil());
    match m2768 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(f583(v1, List::<(Str, Adt9)>::nil())))
        }
    }
}

pub fn f806(v0: Str, v1: Option<Str>) -> List<(Str, Adt9)> {
    let m2769 = v1;
    match m2769 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C3(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f807(v0: Option<i64>) -> Adt9 {
    let m2770 = v0;
    match m2770 {
        None => {
            Adt9::C0
        }
        Some(v1) => {
            f863(v1)
        }
    }
}

pub fn f808(v0: Str, v1: Option<i64>) -> List<(Str, Adt9)> {
    let m2771 = v1;
    match m2771 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f863(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f809(v0: Option<List<Adt5>>) -> List<(Str, Adt9)> {
    let m2772 = v0;
    match m2772 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt9::C4(f864(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f810(v0: Adt9) -> Option<Adt88> {
    let m2773 = f236(v0);
    match m2773 {
        None => {
            None::<Adt88>
        }
        Some(v1) => {
            let m2774 = f865(Str::lit("task"), v1.clone());
            match m2774 {
                None => {
                    None::<Adt88>
                }
                Some(v2) => {
                    let m2775 = f866(v2);
                    match m2775 {
                        None => {
                            None::<Adt88>
                        }
                        Some(v3) => {
                            let m2776 = f867(Str::lit("createdMs"), v1.clone());
                            match m2776 {
                                None => {
                                    None::<Adt88>
                                }
                                Some(v4) => {
                                    let m2777 = f865(Str::lit("plan"), v1);
                                    match m2777 {
                                        None => {
                                            None::<Adt88>
                                        }
                                        Some(v5) => {
                                            let m2778 = f868(v5);
                                            match m2778 {
                                                None => {
                                                    None::<Adt88>
                                                }
                                                Some(v6) => {
                                                    Some(Adt88::C0(v3, v4, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f811(p2782: List<Adt88>, p2783: List<Adt9>) -> Option<List<Adt9>> {
    let m2781 = (p2782, p2783);
    {
        let mut m2781 = m2781;
        loop {
            let step = {
                let (v0, v1) = m2781;
                let m2779 = v0;
                match m2779.uncons() {
                    None => {
                        Err::<(List<Adt88>, List<Adt9>), Option<List<Adt9>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2780 = f869(v2);
                        match m2780 {
                            None => {
                                Err::<(List<Adt88>, List<Adt9>), Option<List<Adt9>>>(None::<List<Adt9>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt88>, List<Adt9>), Option<List<Adt9>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2781 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f812(v0: Adt88, v1: i64) -> R<bool> {
    let m2787 = {
        let m2786 = f741(match v0.clone() {
            Adt88::C0(h2785, _, _) => {
                h2785
            }
        });
        match m2786 {
            Adt67::C0(_, _, _, _, h2784, _) => {
                h2784
            }
        }
    };
    match m2787 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a2794 = {
                let m2793 = {
                    let a2791 = {
                        let a2789 = v1;
                        let a2790 = match v0 {
                            Adt88::C0(_, h2788, _) => {
                                h2788
                            }
                        };
                        int_sub(a2789, a2790)?
                    };
                    let a2792 = v2;
                    compare(a2791, a2792)
                };
                match m2793 {
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
            Ok(bool_not(a2794))
        }
    }
}

pub fn f813() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f814() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f815() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f816() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f817() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f818(p2799: List<List<u64>>, p2800: List<u64>) -> List<u64> {
    let m2798 = (p2799, p2800);
    {
        let mut m2798 = m2798;
        loop {
            let step = {
                let (v0, v1) = m2798;
                let m2795 = v0;
                match m2795.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2796 = f870(v2);
                            let a2797 = v1;
                            append_list(a2796, a2797)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2798 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f819(v0: Str) -> R<Str> {
    let m2803 = {
        let a2802 = f362(f257(), f434(f871(f29({
            let a2801 = v0;
            utf8_encode(a2801)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2802)
    };
    match m2803 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f820(v0: List<u64>) -> bool {
    let m2804 = v0;
    match m2804.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2805 = v1;
            let a2806 = 48u64;
            if nat_eq(a2805, a2806) {
                f872(v2)
            } else {
                false
            }
        }
    }
}

pub fn f821(v0: Adt3, v1: Fn3, v2: u64, v3: Adt18) -> R<Result<Adt142, Adt145>> {
    if f873(f874(v3.clone())) {
        Ok(Ok::<Adt142, Adt145>(f875(v0.clone(), Adt36::C0(f876(v0)))))
    } else {
        Ok(f877(v0.clone(), v3.clone(), f878(match v0.clone() {
            Adt3::C0(_, _, _, _, _, _, h2807) => {
                h2807
            }
        }, match v0 {
            Adt3::C0(_, _, _, _, _, h2808, _) => {
                h2808
            }
        }, v2, v1, v3)?))
    }
}

pub fn f822(v0: Adt20, _: Adt8) -> Result<Adt18, Adt146> {
    Ok::<Adt18, Adt146>(Adt18::C0(Adt19::C0(v0)))
}

pub fn f823(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2809 = f879(Str::lit("cursor"), v1);
    match m2809 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C1(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f824(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2810 = f880(Str::lit("name"), v1.clone());
    match m2810 {
        Ok(v2) => {
            let m2811 = f881(Str::lit("arguments"), v1.clone());
            match m2811 {
                Ok(v3) => {
                    let m2812 = f881(Str::lit("inputResponses"), v1.clone());
                    match m2812 {
                        Ok(v4) => {
                            let m2813 = f879(Str::lit("requestState"), v1);
                            match m2813 {
                                Ok(v5) => {
                                    Ok::<Adt18, Adt146>(Adt18::C2(Adt25::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt18, Adt146>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt18, Adt146>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt18, Adt146>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt18, Adt146>(v9)
        }
    }
}

pub fn f825(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2814 = f879(Str::lit("cursor"), v1);
    match m2814 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C3(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f826(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2815 = f880(Str::lit("uri"), v1.clone());
    match m2815 {
        Ok(v2) => {
            let m2816 = f881(Str::lit("inputResponses"), v1.clone());
            match m2816 {
                Ok(v3) => {
                    let m2817 = f879(Str::lit("requestState"), v1);
                    match m2817 {
                        Ok(v4) => {
                            Ok::<Adt18, Adt146>(Adt18::C4(Adt26::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt18, Adt146>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt18, Adt146>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt18, Adt146>(v7)
        }
    }
}

pub fn f827(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2818 = f879(Str::lit("cursor"), v1);
    match m2818 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C5(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f828(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2819 = f879(Str::lit("cursor"), v1);
    match m2819 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C6(Adt24::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f829(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2820 = f880(Str::lit("name"), v1.clone());
    match m2820 {
        Ok(v2) => {
            let m2821 = f882(Str::lit("arguments"), v1.clone());
            match m2821 {
                Ok(v3) => {
                    let m2822 = f881(Str::lit("inputResponses"), v1.clone());
                    match m2822 {
                        Ok(v4) => {
                            let m2823 = f879(Str::lit("requestState"), v1);
                            match m2823 {
                                Ok(v5) => {
                                    Ok::<Adt18, Adt146>(Adt18::C7(Adt27::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt18, Adt146>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt18, Adt146>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt18, Adt146>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt18, Adt146>(v9)
        }
    }
}

pub fn f830(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2824 = f883(Str::lit("ref"), v1.clone());
    match m2824 {
        Ok(v2) => {
            let m2825 = f884(v2);
            match m2825 {
                Ok(v3) => {
                    let m2826 = f883(Str::lit("argument"), v1.clone());
                    match m2826 {
                        Ok(v4) => {
                            let m2827 = f885(v4);
                            match m2827 {
                                Ok(v5) => {
                                    let m2828 = f886(v1);
                                    match m2828 {
                                        Ok(v6) => {
                                            Ok::<Adt18, Adt146>(Adt18::C8(Adt28::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt18, Adt146>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt18, Adt146>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt18, Adt146>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt18, Adt146>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt18, Adt146>(v11)
        }
    }
}

pub fn f831(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2829 = f880(Str::lit("taskId"), v1);
    match m2829 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C9(Adt33::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f832(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2830 = f880(Str::lit("taskId"), v1.clone());
    match m2830 {
        Ok(v2) => {
            let m2831 = f883(Str::lit("inputResponses"), v1);
            match m2831 {
                Ok(v3) => {
                    Ok::<Adt18, Adt146>(Adt18::C10(Adt34::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt18, Adt146>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt18, Adt146>(v5)
        }
    }
}

pub fn f833(v0: Adt20, v1: Adt8) -> Result<Adt18, Adt146> {
    let m2832 = f880(Str::lit("taskId"), v1);
    match m2832 {
        Ok(v2) => {
            Ok::<Adt18, Adt146>(Adt18::C11(Adt35::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt18, Adt146>(v3)
        }
    }
}

pub fn f834(v0: Adt124, v1: Adt20, v2: Option<Adt145>) -> Adt157 {
    let m2833 = v2;
    match m2833 {
        None => {
            Adt157::C0(Adt158::C0(match v0.clone() {
                Adt124::C0(h2834, _, _) => {
                    h2834
                }
            }, match v0.clone() {
                Adt124::C0(_, h2835, _) => {
                    h2835
                }
            }, v1, f773(match v0 {
                Adt124::C0(_, _, h2836) => {
                    h2836
                }
            })))
        }
        Some(v3) => {
            Adt157::C1(v3)
        }
    }
}

pub fn f835(v0: Str, v1: List<Str>) -> Option<Adt145> {
    f887(f888(v0.clone(), v1.clone()), v0, v1)
}

pub fn f836(v0: Adt128, v1: Adt159, v2: Str, v3: Str, v4: Adt8) -> R<Option<Adt129>> {
    let m2838 = match v0.clone() {
        Adt128::C0(h2837, _, _, _, _) => {
            h2837
        }
    };
    match m2838 {
        None => {
            Ok(Some(Adt129::C0))
        }
        Some(v5) => {
            let a2839 = v5;
            let a2840 = v3;
            if equal(a2839, a2840) {
                let m2842 = match v0.clone() {
                    Adt128::C0(_, h2841, _, _, _) => {
                        h2841
                    }
                };
                match m2842 {
                    None => {
                        Ok(Some(Adt129::C1))
                    }
                    Some(v6) => {
                        let a2843 = v6;
                        let a2844 = v2;
                        if equal(a2843, a2844) {
                            f889(f890(match v0.clone() {
                                Adt128::C0(_, _, h2845, _, _) => {
                                    h2845
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt128::C0(_, _, _, _, h2846) => {
                                    h2846
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt129::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt129::C3))
            }
        }
    }
}

pub fn f837(v0: Adt8) -> Adt166 {
    f891(f235(f839(), v0.clone()), f235(f840(), v0))
}

pub fn f838(v0: Adt8, v1: Adt116, v2: Adt116) -> Result<Adt20, Adt146> {
    let m2847 = v1;
    match m2847 {
        Adt116::C0 => {
            Err::<Adt20, Adt146>(Adt146::C2)
        }
        Adt116::C1(v3) => {
            let m2848 = v2;
            match m2848 {
                Adt116::C0 => {
                    Err::<Adt20, Adt146>(Adt146::C3)
                }
                Adt116::C1(v4) => {
                    f892(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f839() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f840() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f841(v0: Adt9) -> u64 {
    let m2849 = v0;
    match m2849 {
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

pub fn f842(v0: Adt161) -> bool {
    let m2850 = v0;
    match m2850 {
        Adt161::C0 => {
            false
        }
        Adt161::C1 => {
            true
        }
        Adt161::C2 => {
            false
        }
    }
}

pub fn f843(v0: Option<List<(Adt9, Adt9)>>, v1: List<(Adt9, Adt9)>) -> Adt165 {
    let m2851 = v0;
    match m2851 {
        None => {
            Adt165::C0
        }
        Some(v2) => {
            Adt165::C1({
                let a2852 = v2;
                let a2853 = v1;
                append_list(a2852, a2853)
            })
        }
    }
}

pub fn f844(p2858: List<Adt9>, p2859: List<Adt9>, p2860: List<(Adt9, Adt9)>) -> Option<List<(Adt9, Adt9)>> {
    let m2857 = (p2858, (p2859, p2860));
    {
        let mut m2857 = m2857;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2857;
                let m2854 = v0;
                match m2854.uncons() {
                    None => {
                        let m2855 = v1;
                        match m2855.uncons() {
                            None => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt9>, (List<Adt9>, List<(Adt9, Adt9)>)), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2856 = v1;
                        match m2856.uncons() {
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
                Ok(next) => { m2857 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f845(p2866: List<(Str, Adt9)>, v1: List<(Str, Adt9)>, p2867: List<(Adt9, Adt9)>) -> Option<List<(Adt9, Adt9)>> {
    let m2865 = (p2866, p2867);
    {
        let mut m2865 = m2865;
        loop {
            let step = {
                let (v0, v2) = m2865;
                let m2861 = v0;
                match m2861.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2863 = f309({
                            let (h2862, _) = v3;
                            h2862
                        }, v1.clone());
                        match m2863 {
                            Adt116::C0 => {
                                Err::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>(None::<List<(Adt9, Adt9)>>)
                            }
                            Adt116::C1(v5) => {
                                Ok::<(List<(Str, Adt9)>, List<(Adt9, Adt9)>), Option<List<(Adt9, Adt9)>>>((v4, List::cons(({
                                    let (_, h2864) = v3;
                                    h2864
                                }, v5), v2)))
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

pub fn f846(v0: Adt161) -> Adt161 {
    let m2868 = v0;
    match m2868 {
        Adt161::C0 => {
            Adt161::C2
        }
        Adt161::C1 => {
            Adt161::C1
        }
        Adt161::C2 => {
            Adt161::C0
        }
    }
}

pub fn f847(v0: i64, v1: i64) -> R<i64> {
    f359(f289(f849(v0)?, 0i64)?, v1)
}

pub fn f848(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt161> {
    let a2869 = v1;
    let a2870 = v3;
    if nat_lt(a2869, a2870) {
        Ok(f711(f893({
            let a2871 = v3;
            let a2872 = v1;
            nat_sub(a2871, a2872)
        }, v0)?, v2))
    } else {
        let a2873 = v3;
        let a2874 = v1;
        if nat_lt(a2873, a2874) {
            Ok(f711(v0, f893({
                let a2875 = v1;
                let a2876 = v3;
                nat_sub(a2875, a2876)
            }, v2)?))
        } else {
            Ok(f711(v0, v2))
        }
    }
}

pub fn f849(v0: i64) -> R<u64> {
    f894(20u64, v0, 0u64)
}

pub fn f850(v0: i64) -> u64 {
    let m2879 = {
        let a2877 = v0;
        let a2878 = 1i64;
        compare(a2877, a2878)
    };
    if match m2879 {
        Ordering::Less => {
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
        let m2882 = {
            let a2880 = v0;
            let a2881 = 2i64;
            compare(a2880, a2881)
        };
        if match m2882 {
            Ordering::Less => {
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
            let m2885 = {
                let a2883 = v0;
                let a2884 = 3i64;
                compare(a2883, a2884)
            };
            if match m2885 {
                Ordering::Less => {
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
                let m2888 = {
                    let a2886 = v0;
                    let a2887 = 4i64;
                    compare(a2886, a2887)
                };
                if match m2888 {
                    Ordering::Less => {
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
                    let m2891 = {
                        let a2889 = v0;
                        let a2890 = 5i64;
                        compare(a2889, a2890)
                    };
                    if match m2891 {
                        Ordering::Less => {
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
                        let m2894 = {
                            let a2892 = v0;
                            let a2893 = 6i64;
                            compare(a2892, a2893)
                        };
                        if match m2894 {
                            Ordering::Less => {
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
                            let m2897 = {
                                let a2895 = v0;
                                let a2896 = 7i64;
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
                                54u64
                            } else {
                                let m2900 = {
                                    let a2898 = v0;
                                    let a2899 = 8i64;
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
                                    55u64
                                } else {
                                    let m2903 = {
                                        let a2901 = v0;
                                        let a2902 = 9i64;
                                        compare(a2901, a2902)
                                    };
                                    if match m2903 {
                                        Ordering::Less => {
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

pub fn f851(v0: Adt11) -> Adt9 {
    Adt9::C5(f727(Str::lit("listChanged"), match v0 {
        Adt11::C0(h2904) => {
            h2904
        }
    }))
}

pub fn f852(v0: Adt39) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt39::C0(h2905, _, _, _, _, _, _, _) => {
            h2905
        }
    })), {
        let a2923 = f722(Str::lit("title"), match v0.clone() {
            Adt39::C0(_, h2906, _, _, _, _, _, _) => {
                h2906
            }
        });
        let a2924 = {
            let a2921 = f809(match v0.clone() {
                Adt39::C0(_, _, h2907, _, _, _, _, _) => {
                    h2907
                }
            });
            let a2922 = {
                let a2919 = f722(Str::lit("description"), match v0.clone() {
                    Adt39::C0(_, _, _, h2908, _, _, _, _) => {
                        h2908
                    }
                });
                let a2920 = List::cons((Str::lit("inputSchema"), Adt9::C5({
                    let m2911 = match v0.clone() {
                        Adt39::C0(_, _, _, _, h2910, _, _, _) => {
                            h2910
                        }
                    };
                    match m2911 {
                        Adt8::C0(h2909) => {
                            h2909
                        }
                    }
                })), {
                    let a2917 = f763(Str::lit("outputSchema"), match v0.clone() {
                        Adt39::C0(_, _, _, _, _, h2912, _, _) => {
                            h2912
                        }
                    });
                    let a2918 = {
                        let a2915 = f895(match v0.clone() {
                            Adt39::C0(_, _, _, _, _, _, h2913, _) => {
                                h2913
                            }
                        });
                        let a2916 = f896(match v0 {
                            Adt39::C0(_, _, _, _, _, _, _, h2914) => {
                                h2914
                            }
                        });
                        append_list(a2915, a2916)
                    };
                    append_list(a2917, a2918)
                });
                append_list(a2919, a2920)
            };
            append_list(a2921, a2922)
        };
        append_list(a2923, a2924)
    }))
}

pub fn f853(v0: Adt42) -> R<Option<Adt9>> {
    let m2925 = v0;
    match m2925 {
        Adt42::C0(v1) => {
            Ok(Some(f897(v1)))
        }
        Adt42::C1(v2) => {
            Ok(Some(f898(Str::lit("image"), match v2.clone() {
                Adt46::C0(h2926, _, _, _) => {
                    h2926
                }
            }, match v2.clone() {
                Adt46::C0(_, h2927, _, _) => {
                    h2927
                }
            }, match v2.clone() {
                Adt46::C0(_, _, h2928, _) => {
                    h2928
                }
            }, match v2 {
                Adt46::C0(_, _, _, h2929) => {
                    h2929
                }
            })))
        }
        Adt42::C2(v3) => {
            Ok(Some(f898(Str::lit("audio"), match v3.clone() {
                Adt47::C0(h2930, _, _, _) => {
                    h2930
                }
            }, match v3.clone() {
                Adt47::C0(_, h2931, _, _) => {
                    h2931
                }
            }, match v3.clone() {
                Adt47::C0(_, _, h2932, _) => {
                    h2932
                }
            }, match v3 {
                Adt47::C0(_, _, _, h2933) => {
                    h2933
                }
            })))
        }
        Adt42::C3(v4) => {
            Ok(Some(f899(v4)?))
        }
        Adt42::C4(v5) => {
            Ok(Some(f900(v5)))
        }
    }
}

pub fn f854(v0: Adt48) -> R<Adt9> {
    Ok(Adt9::C5({
        let a2957 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt48::C0(h2934, _, _, _, _, _, _, _, _) => {
                h2934
            }
        })), List::<(Str, Adt9)>::nil());
        let a2958 = {
            let a2955 = f722(Str::lit("title"), match v0.clone() {
                Adt48::C0(_, h2935, _, _, _, _, _, _, _) => {
                    h2935
                }
            });
            let a2956 = {
                let a2953 = f809(match v0.clone() {
                    Adt48::C0(_, _, h2936, _, _, _, _, _, _) => {
                        h2936
                    }
                });
                let a2954 = {
                    let a2951 = List::cons((Str::lit("uri"), Adt9::C3(match v0.clone() {
                        Adt48::C0(_, _, _, h2937, _, _, _, _, _) => {
                            h2937
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a2952 = {
                        let a2949 = f722(Str::lit("description"), match v0.clone() {
                            Adt48::C0(_, _, _, _, h2938, _, _, _, _) => {
                                h2938
                            }
                        });
                        let a2950 = {
                            let a2947 = f722(Str::lit("mimeType"), match v0.clone() {
                                Adt48::C0(_, _, _, _, _, h2939, _, _, _) => {
                                    h2939
                                }
                            });
                            let a2948 = {
                                let a2945 = f901(match v0.clone() {
                                    Adt48::C0(_, _, _, _, _, _, h2940, _, _) => {
                                        h2940
                                    }
                                });
                                let a2946 = {
                                    let a2943 = f902(match v0.clone() {
                                        Adt48::C0(_, _, _, _, _, _, _, h2941, _) => {
                                            h2941
                                        }
                                    })?;
                                    let a2944 = f896(match v0 {
                                        Adt48::C0(_, _, _, _, _, _, _, _, h2942) => {
                                            h2942
                                        }
                                    });
                                    append_list(a2943, a2944)
                                };
                                append_list(a2945, a2946)
                            };
                            append_list(a2947, a2948)
                        };
                        append_list(a2949, a2950)
                    };
                    append_list(a2951, a2952)
                };
                append_list(a2953, a2954)
            };
            append_list(a2955, a2956)
        };
        append_list(a2957, a2958)
    }))
}

pub fn f855(v0: Adt50) -> Adt9 {
    let m2959 = v0;
    match m2959 {
        Adt50::C0(v1) => {
            Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(match v1.clone() {
                Adt51::C0(h2960, _, _, _) => {
                    h2960
                }
            })), {
                let a2964 = f722(Str::lit("mimeType"), match v1.clone() {
                    Adt51::C0(_, h2961, _, _) => {
                        h2961
                    }
                });
                let a2965 = List::cons((Str::lit("text"), Adt9::C3(match v1.clone() {
                    Adt51::C0(_, _, h2962, _) => {
                        h2962
                    }
                })), f896(match v1 {
                    Adt51::C0(_, _, _, h2963) => {
                        h2963
                    }
                }));
                append_list(a2964, a2965)
            }))
        }
        Adt50::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("uri"), Adt9::C3(match v2.clone() {
                Adt52::C0(h2966, _, _, _) => {
                    h2966
                }
            })), {
                let a2970 = f722(Str::lit("mimeType"), match v2.clone() {
                    Adt52::C0(_, h2967, _, _) => {
                        h2967
                    }
                });
                let a2971 = List::cons((Str::lit("blob"), Adt9::C3(match v2.clone() {
                    Adt52::C0(_, _, h2968, _) => {
                        h2968
                    }
                })), f896(match v2 {
                    Adt52::C0(_, _, _, h2969) => {
                        h2969
                    }
                }));
                append_list(a2970, a2971)
            }))
        }
    }
}

pub fn f856(v0: Adt58) -> Adt9 {
    Adt9::C5({
        let a2992 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt58::C0(h2972, _, _, _, _, _, _, _) => {
                h2972
            }
        })), List::<(Str, Adt9)>::nil());
        let a2993 = {
            let a2990 = f722(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2973, _, _, _, _, _, _) => {
                    h2973
                }
            });
            let a2991 = {
                let a2988 = f809(match v0.clone() {
                    Adt58::C0(_, _, h2974, _, _, _, _, _) => {
                        h2974
                    }
                });
                let a2989 = {
                    let a2986 = List::cons((Str::lit("uriTemplate"), Adt9::C3(match v0.clone() {
                        Adt58::C0(_, _, _, h2975, _, _, _, _) => {
                            h2975
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a2987 = {
                        let a2984 = f722(Str::lit("description"), match v0.clone() {
                            Adt58::C0(_, _, _, _, h2976, _, _, _) => {
                                h2976
                            }
                        });
                        let a2985 = {
                            let a2982 = f722(Str::lit("mimeType"), match v0.clone() {
                                Adt58::C0(_, _, _, _, _, h2977, _, _) => {
                                    h2977
                                }
                            });
                            let a2983 = {
                                let a2980 = f901(match v0.clone() {
                                    Adt58::C0(_, _, _, _, _, _, h2978, _) => {
                                        h2978
                                    }
                                });
                                let a2981 = f896(match v0 {
                                    Adt58::C0(_, _, _, _, _, _, _, h2979) => {
                                        h2979
                                    }
                                });
                                append_list(a2980, a2981)
                            };
                            append_list(a2982, a2983)
                        };
                        append_list(a2984, a2985)
                    };
                    append_list(a2986, a2987)
                };
                append_list(a2988, a2989)
            };
            append_list(a2990, a2991)
        };
        append_list(a2992, a2993)
    })
}

pub fn f857(v0: Adt60) -> Adt9 {
    Adt9::C5({
        let a3009 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt60::C0(h2994, _, _, _, _, _) => {
                h2994
            }
        })), List::<(Str, Adt9)>::nil());
        let a3010 = {
            let a3007 = f722(Str::lit("title"), match v0.clone() {
                Adt60::C0(_, h2995, _, _, _, _) => {
                    h2995
                }
            });
            let a3008 = {
                let a3005 = f809(match v0.clone() {
                    Adt60::C0(_, _, h2996, _, _, _) => {
                        h2996
                    }
                });
                let a3006 = {
                    let a3003 = f722(Str::lit("description"), match v0.clone() {
                        Adt60::C0(_, _, _, h2997, _, _) => {
                            h2997
                        }
                    });
                    let a3004 = {
                        let a3001 = {
                            let m2999 = match v0.clone() {
                                Adt60::C0(_, _, _, _, h2998, _) => {
                                    h2998
                                }
                            };
                            match m2999 {
                                None => {
                                    List::<(Str, Adt9)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt9::C4(f903(v1))), List::<(Str, Adt9)>::nil())
                                }
                            }
                        };
                        let a3002 = f896(match v0 {
                            Adt60::C0(_, _, _, _, _, h3000) => {
                                h3000
                            }
                        });
                        append_list(a3001, a3002)
                    };
                    append_list(a3003, a3004)
                };
                append_list(a3005, a3006)
            };
            append_list(a3007, a3008)
        };
        append_list(a3009, a3010)
    })
}

pub fn f858(v0: Adt63) -> R<Option<Adt9>> {
    let m3012 = f853(match v0.clone() {
        Adt63::C0(_, h3011) => {
            h3011
        }
    })?;
    match m3012 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("role"), Adt9::C3(f904(match v0 {
                Adt63::C0(h3013, _) => {
                    h3013
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt9)>::nil())))))
        }
    }
}

pub fn f859(v0: Str, v1: Option<Adt9>) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(v0)), f726(Str::lit("params"), v1)))
}

pub fn f860(v0: Adt69) -> Adt9 {
    let m3014 = v0;
    match m3014 {
        Adt69::C0(v1) => {
            f905(v1)
        }
        Adt69::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("mode"), Adt9::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt9::C3(match v2.clone() {
                Adt72::C0(h3015, _) => {
                    h3015
                }
            })), List::cons((Str::lit("url"), Adt9::C3(match v2 {
                Adt72::C0(_, h3016) => {
                    h3016
                }
            })), List::<(Str, Adt9)>::nil()))))
        }
    }
}

pub fn f861(v0: Adt73) -> R<Option<Adt9>> {
    let m3018 = f906(match v0.clone() {
        Adt73::C0(h3017, _, _, _, _, _, _, _, _, _) => {
            h3017
        }
    })?;
    match m3018 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("messages"), Adt9::C4(v1)), {
                let a3020 = f907(v0.clone());
                let a3021 = List::cons((Str::lit("maxTokens"), f120(match v0.clone() {
                    Adt73::C0(_, _, _, _, _, h3019, _, _, _, _) => {
                        h3019
                    }
                })?), f908(v0));
                append_list(a3020, a3021)
            }))))
        }
    }
}

pub fn f862(p3027: List<(Str, Adt68)>, p3028: List<(Str, Adt9)>) -> Option<List<(Str, Adt9)>> {
    let m3026 = (p3027, p3028);
    {
        let mut m3026 = m3026;
        loop {
            let step = {
                let (v0, v1) = m3026;
                let m3022 = v0;
                match m3022.uncons() {
                    None => {
                        Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m3024 = f909({
                            let (_, h3023) = v2;
                            h3023
                        });
                        match m3024 {
                            None => {
                                Err::<(List<(Str, Adt68)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>(None::<List<(Str, Adt9)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt68)>, List<(Str, Adt9)>), Option<List<(Str, Adt9)>>>((v3, List::cons(({
                                    let (h3025, _) = v2;
                                    h3025
                                }, v4), v1)))
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

pub fn f863(v0: i64) -> Adt9 {
    Adt9::C2(Adt10::C0(v0, 0i64))
}

pub fn f864(v0: List<Adt5>) -> List<Adt9> {
    f581(f910(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f865(v0: Str, v1: Adt8) -> Option<Adt8> {
    let m3029 = f235(v0, v1);
    match m3029 {
        Adt116::C0 => {
            None::<Adt8>
        }
        Adt116::C1(v2) => {
            f236(v2)
        }
    }
}

pub fn f866(v0: Adt8) -> Option<Adt66> {
    let m3030 = f911(v0.clone());
    match m3030 {
        None => {
            None::<Adt66>
        }
        Some(v1) => {
            let m3031 = f912(Str::lit("status"), v0.clone());
            match m3031 {
                None => {
                    None::<Adt66>
                }
                Some(v2) => {
                    let a3032 = v2.clone();
                    let a3033 = Str::lit("working");
                    if equal(a3032, a3033) {
                        Some(Adt66::C0(v1))
                    } else {
                        let a3034 = v2.clone();
                        let a3035 = Str::lit("cancelled");
                        if equal(a3034, a3035) {
                            Some(Adt66::C4(v1))
                        } else {
                            let a3036 = v2.clone();
                            let a3037 = Str::lit("input_required");
                            if equal(a3036, a3037) {
                                let m3038 = f913(Str::lit("inputRequests"), v0);
                                match m3038 {
                                    None => {
                                        None::<Adt66>
                                    }
                                    Some(v3) => {
                                        Some(Adt66::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a3039 = v2.clone();
                                let a3040 = Str::lit("completed");
                                if equal(a3039, a3040) {
                                    let m3041 = f865(Str::lit("result"), v0);
                                    match m3041 {
                                        None => {
                                            None::<Adt66>
                                        }
                                        Some(v4) => {
                                            Some(Adt66::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a3042 = v2;
                                    let a3043 = Str::lit("failed");
                                    if equal(a3042, a3043) {
                                        let m3044 = f865(Str::lit("error"), v0);
                                        match m3044 {
                                            None => {
                                                None::<Adt66>
                                            }
                                            Some(v5) => {
                                                let m3045 = f914(v5);
                                                match m3045 {
                                                    None => {
                                                        None::<Adt66>
                                                    }
                                                    Some(v6) => {
                                                        Some(Adt66::C3(v1, v6))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        None::<Adt66>
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f867(v0: Str, v1: Adt8) -> Option<i64> {
    let m3046 = f235(v0, v1);
    match m3046 {
        Adt116::C0 => {
            None::<i64>
        }
        Adt116::C1(v2) => {
            f710(v2)
        }
    }
}

pub fn f868(v0: Adt8) -> Option<Adt89> {
    let m3047 = f867(Str::lit("parkAfterMs"), v0.clone());
    match m3047 {
        None => {
            None::<Adt89>
        }
        Some(v1) => {
            let m3048 = f913(Str::lit("parked"), v0.clone());
            match m3048 {
                None => {
                    None::<Adt89>
                }
                Some(v2) => {
                    let m3049 = f867(Str::lit("finishAfterMs"), v0.clone());
                    match m3049 {
                        None => {
                            None::<Adt89>
                        }
                        Some(v3) => {
                            let m3050 = f865(Str::lit("outcome"), v0);
                            match m3050 {
                                None => {
                                    None::<Adt89>
                                }
                                Some(v4) => {
                                    let m3051 = f915(v4);
                                    match m3051 {
                                        None => {
                                            None::<Adt89>
                                        }
                                        Some(v5) => {
                                            Some(Adt89::C0(v1, v2, v3, v5))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f869(v0: Adt88) -> Option<Adt9> {
    let m3053 = f647(match v0.clone() {
        Adt88::C0(h3052, _, _) => {
            h3052
        }
    });
    match m3053 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            let m3055 = f916(match v0.clone() {
                Adt88::C0(_, _, h3054) => {
                    h3054
                }
            });
            match m3055 {
                None => {
                    None::<Adt9>
                }
                Some(v2) => {
                    Some(Adt9::C5(List::cons((Str::lit("task"), Adt9::C5(v1)), List::cons((Str::lit("createdMs"), f863(match v0 {
                        Adt88::C0(_, h3056, _) => {
                            h3056
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt9)>::nil())))))
                }
            }
        }
    }
}

pub fn f870(v0: List<u64>) -> List<u64> {
    let a3059 = f917();
    let a3060 = {
        let a3057 = v0;
        let a3058 = f918();
        append_list(a3057, a3058)
    };
    append_list(a3059, a3060)
}

pub fn f871(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f919(v0, 0u64, v1)
}

pub fn f872(p3067: List<u64>) -> bool {
    let m3066 = p3067;
    {
        let mut m3066 = m3066;
        loop {
            let step = {
                let v0 = m3066;
                let m3061 = v0;
                match m3061.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a3062 = v1;
                        let a3063 = 48u64;
                        if if nat_eq(a3062, a3063) {
                            true
                        } else {
                            let a3064 = v1;
                            let a3065 = 46u64;
                            nat_eq(a3064, a3065)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f873(v0: Adt154) -> bool {
    let m3068 = v0;
    match m3068 {
        Adt154::C0 => {
            true
        }
        Adt154::C1 => {
            false
        }
        Adt154::C2 => {
            false
        }
        Adt154::C3 => {
            false
        }
        Adt154::C4 => {
            false
        }
        Adt154::C5 => {
            false
        }
        Adt154::C6 => {
            false
        }
        Adt154::C7 => {
            false
        }
        Adt154::C8 => {
            false
        }
        Adt154::C9 => {
            false
        }
        Adt154::C10 => {
            false
        }
        Adt154::C11 => {
            false
        }
    }
}

pub fn f874(v0: Adt18) -> Adt154 {
    let m3069 = v0;
    match m3069 {
        Adt18::C0(_) => {
            Adt154::C0
        }
        Adt18::C1(_) => {
            Adt154::C1
        }
        Adt18::C2(_) => {
            Adt154::C2
        }
        Adt18::C3(_) => {
            Adt154::C3
        }
        Adt18::C4(_) => {
            Adt154::C4
        }
        Adt18::C5(_) => {
            Adt154::C5
        }
        Adt18::C6(_) => {
            Adt154::C6
        }
        Adt18::C7(_) => {
            Adt154::C7
        }
        Adt18::C8(_) => {
            Adt154::C8
        }
        Adt18::C9(_) => {
            Adt154::C9
        }
        Adt18::C10(_) => {
            Adt154::C10
        }
        Adt18::C11(_) => {
            Adt154::C11
        }
    }
}

pub fn f875(v0: Adt3, v1: Adt36) -> Adt142 {
    Adt142::C0(v1, Adt143::C0(Some(match v0 {
        Adt3::C0(h3070, _, _, _, _, _, _) => {
            h3070
        }
    }), None::<Adt8>), List::<Adt144>::nil())
}

pub fn f876(v0: Adt3) -> Adt37 {
    Adt37::C0(match v0.clone() {
        Adt3::C0(_, _, _, h3071, _, _, _) => {
            h3071
        }
    }, f445(), match v0.clone() {
        Adt3::C0(_, _, h3072, _, _, _, _) => {
            h3072
        }
    }, match v0 {
        Adt3::C0(_, h3073, _, _, _, _, _) => {
            h3073
        }
    })
}

pub fn f877(v0: Adt3, v1: Adt18, v2: Result<Adt36, Adt93>) -> Result<Adt142, Adt145> {
    let m3074 = v2;
    match m3074 {
        Ok(v4) => {
            if f920(f874(v1.clone()), v4.clone()) {
                f921(v0, v1, v4)
            } else {
                Err::<Adt142, Adt145>(Adt145::C4(f922()))
            }
        }
        Err(v3) => {
            Err::<Adt142, Adt145>(Adt145::C4(v3))
        }
    }
}

pub fn f878(v0: Bytes, v1: u64, v2: u64, v3: Fn3, v4: Adt18) -> R<Result<Adt36, Adt93>> {
    f923(v0.clone(), v1, v2, v3, v4.clone(), f924(v0, v4)?)
}

pub fn f879(v0: Str, v1: Adt8) -> Result<Option<Str>, Adt146> {
    let m3075 = f235(v0.clone(), v1);
    match m3075 {
        Adt116::C0 => {
            Ok::<Option<Str>, Adt146>(None::<Str>)
        }
        Adt116::C1(v2) => {
            let m3076 = f287(v2);
            match m3076 {
                None => {
                    Err::<Option<Str>, Adt146>(Adt146::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt146>(Some(v3))
                }
            }
        }
    }
}

pub fn f880(v0: Str, v1: Adt8) -> Result<Str, Adt146> {
    let m3077 = f235(v0.clone(), v1);
    match m3077 {
        Adt116::C0 => {
            Err::<Str, Adt146>(Adt146::C7(v0))
        }
        Adt116::C1(v2) => {
            let m3078 = f287(v2);
            match m3078 {
                None => {
                    Err::<Str, Adt146>(Adt146::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt146>(v3)
                }
            }
        }
    }
}

pub fn f881(v0: Str, v1: Adt8) -> Result<Option<Adt8>, Adt146> {
    let m3079 = f235(v0.clone(), v1);
    match m3079 {
        Adt116::C0 => {
            Ok::<Option<Adt8>, Adt146>(None::<Adt8>)
        }
        Adt116::C1(v2) => {
            let m3080 = f236(v2);
            match m3080 {
                None => {
                    Err::<Option<Adt8>, Adt146>(Adt146::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt8>, Adt146>(Some(v3))
                }
            }
        }
    }
}

pub fn f882(v0: Str, v1: Adt8) -> Result<Option<List<(Str, Str)>>, Adt146> {
    let m3081 = f235(v0.clone(), v1);
    match m3081 {
        Adt116::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt146>(None::<List<(Str, Str)>>)
        }
        Adt116::C1(v2) => {
            let m3082 = f236(v2);
            match m3082 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt146>(Adt146::C8(v0))
                }
                Some(v3) => {
                    let m3084 = f925(v0, match v3 {
                        Adt8::C0(h3083) => {
                            h3083
                        }
                    });
                    match m3084 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt146>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt146>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f883(v0: Str, v1: Adt8) -> Result<Adt8, Adt146> {
    let m3085 = f235(v0.clone(), v1);
    match m3085 {
        Adt116::C0 => {
            Err::<Adt8, Adt146>(Adt146::C7(v0))
        }
        Adt116::C1(v2) => {
            let m3086 = f236(v2);
            match m3086 {
                None => {
                    Err::<Adt8, Adt146>(Adt146::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt8, Adt146>(v3)
                }
            }
        }
    }
}

pub fn f884(v0: Adt8) -> Result<Adt29, Adt146> {
    let m3087 = f880(Str::lit("type"), v0.clone());
    match m3087 {
        Ok(v1) => {
            let a3088 = v1.clone();
            let a3089 = Str::lit("ref/prompt");
            if equal(a3088, a3089) {
                let m3090 = f880(Str::lit("name"), v0.clone());
                match m3090 {
                    Ok(v2) => {
                        let m3091 = f879(Str::lit("title"), v0);
                        match m3091 {
                            Ok(v3) => {
                                Ok::<Adt29, Adt146>(Adt29::C0(Adt30::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt29, Adt146>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt29, Adt146>(v5)
                    }
                }
            } else {
                let a3092 = v1;
                let a3093 = Str::lit("ref/resource");
                if equal(a3092, a3093) {
                    let m3094 = f880(Str::lit("uri"), v0);
                    match m3094 {
                        Ok(v6) => {
                            Ok::<Adt29, Adt146>(Adt29::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt29, Adt146>(v7)
                        }
                    }
                } else {
                    Err::<Adt29, Adt146>(Adt146::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt29, Adt146>(v8)
        }
    }
}

pub fn f885(v0: Adt8) -> Result<Adt31, Adt146> {
    let m3095 = f880(Str::lit("name"), v0.clone());
    match m3095 {
        Ok(v1) => {
            let m3096 = f880(Str::lit("value"), v0);
            match m3096 {
                Ok(v2) => {
                    Ok::<Adt31, Adt146>(Adt31::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt31, Adt146>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt31, Adt146>(v4)
        }
    }
}

pub fn f886(v0: Adt8) -> Result<Option<Adt32>, Adt146> {
    let m3097 = f235(Str::lit("context"), v0);
    match m3097 {
        Adt116::C0 => {
            Ok::<Option<Adt32>, Adt146>(None::<Adt32>)
        }
        Adt116::C1(v1) => {
            let m3098 = f236(v1);
            match m3098 {
                None => {
                    Err::<Option<Adt32>, Adt146>(Adt146::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m3099 = f882(Str::lit("arguments"), v2);
                    match m3099 {
                        Ok(v3) => {
                            Ok::<Option<Adt32>, Adt146>(Some(Adt32::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt32>, Adt146>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f887(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt145> {
    if v0 {
        None::<Adt145>
    } else {
        Some(Adt145::C1(v1, v2))
    }
}

pub fn f888(v0: Str, p3104: List<Str>) -> bool {
    let m3103 = p3104;
    {
        let mut m3103 = m3103;
        loop {
            let step = {
                let v1 = m3103;
                let m3100 = v1;
                match m3100.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3101 = v2;
                        let a3102 = v0.clone();
                        if equal(a3101, a3102) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f889(v0: Option<Adt129>, v1: List<Adt131>, v2: Adt8) -> R<Option<Adt129>> {
    let m3105 = v0;
    match m3105 {
        None => {
            f926(v1, f927(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f890(v0: Option<Str>, v1: Adt159, v2: Adt8) -> Option<Adt129> {
    let m3106 = v1.clone();
    match m3106 {
        Adt159::C0 => {
            None::<Adt129>
        }
        Adt159::C1 => {
            f928(v0, f929(v1, v2))
        }
        Adt159::C2 => {
            f928(v0, f929(v1, v2))
        }
        Adt159::C3 => {
            f928(v0, f929(v1, v2))
        }
    }
}

pub fn f891(v0: Adt116, v1: Adt116) -> Adt166 {
    let m3107 = v0;
    match m3107 {
        Adt116::C0 => {
            Adt166::C1
        }
        Adt116::C1(_) => {
            let m3108 = v1;
            match m3108 {
                Adt116::C0 => {
                    Adt166::C2
                }
                Adt116::C1(_) => {
                    Adt166::C0
                }
            }
        }
    }
}

pub fn f892(v0: Adt8, v1: Adt9, v2: Adt9) -> Result<Adt20, Adt146> {
    let m3109 = f287(v1);
    match m3109 {
        None => {
            Err::<Adt20, Adt146>(Adt146::C5)
        }
        Some(v3) => {
            let m3110 = f930(v2);
            match m3110 {
                None => {
                    Err::<Adt20, Adt146>(Adt146::C6)
                }
                Some(v4) => {
                    let m3111 = f881(f931(), v0.clone());
                    match m3111 {
                        Ok(v6) => {
                            let m3112 = f932(v0.clone());
                            match m3112 {
                                Ok(v8) => {
                                    let m3113 = f933(v0.clone());
                                    match m3113 {
                                        Ok(v10) => {
                                            Ok::<Adt20, Adt146>(Adt20::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt20, Adt146>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt20, Adt146>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt20, Adt146>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f893(p3118: u64, p3119: i64) -> R<i64> {
    let m3117 = (p3118, p3119);
    {
        let mut m3117 = m3117;
        loop {
            let step = {
                let (v0, v1) = m3117;
                let m3114 = v0;
                if m3114 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3114 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3115 = v1;
                        let a3116 = 10i64;
                        int_mul(a3115, a3116)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f894(p3130: u64, p3131: i64, p3132: u64) -> R<u64> {
    let m3129 = (p3130, (p3131, p3132));
    {
        let mut m3129 = m3129;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3129;
                let m3120 = v0;
                if m3120 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3120 - 1;
                    let v4: u64 = {
                        let a3121 = v2;
                        let a3122 = 1u64;
                        nat_add(a3121, a3122)?
                    };
                    let m3125 = {
                        let a3123 = v1;
                        let a3124 = 10i64;
                        compare(a3123, a3124)
                    };
                    if match m3125 {
                        Ordering::Less => {
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
                            let a3126 = v1;
                            let a3127 = 10i64;
                            let a3128 = 0i64;
                            int_quot(a3126, a3127, a3128)?
                        }, v4)))
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

pub fn f895(v0: Option<Adt40>) -> List<(Str, Adt9)> {
    let m3133 = v0;
    match m3133 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt9::C5({
                let a3145 = f722(Str::lit("title"), match v1.clone() {
                    Adt40::C0(h3134, _, _, _, _) => {
                        h3134
                    }
                });
                let a3146 = {
                    let a3143 = f727(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt40::C0(_, h3135, _, _, _) => {
                            h3135
                        }
                    });
                    let a3144 = {
                        let a3141 = f727(Str::lit("destructiveHint"), match v1.clone() {
                            Adt40::C0(_, _, h3136, _, _) => {
                                h3136
                            }
                        });
                        let a3142 = {
                            let a3139 = f727(Str::lit("idempotentHint"), match v1.clone() {
                                Adt40::C0(_, _, _, h3137, _) => {
                                    h3137
                                }
                            });
                            let a3140 = f727(Str::lit("openWorldHint"), match v1 {
                                Adt40::C0(_, _, _, _, h3138) => {
                                    h3138
                                }
                            });
                            append_list(a3139, a3140)
                        };
                        append_list(a3141, a3142)
                    };
                    append_list(a3143, a3144)
                };
                append_list(a3145, a3146)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f896(v0: Option<Adt8>) -> List<(Str, Adt9)> {
    f763(Str::lit("_meta"), v0)
}

pub fn f897(v0: Adt43) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt9::C3(match v0.clone() {
        Adt43::C0(h3147, _, _) => {
            h3147
        }
    })), {
        let a3150 = f901(match v0.clone() {
            Adt43::C0(_, h3148, _) => {
                h3148
            }
        });
        let a3151 = f896(match v0 {
            Adt43::C0(_, _, h3149) => {
                h3149
            }
        });
        append_list(a3150, a3151)
    })))
}

pub fn f898(v0: Str, v1: Str, v2: Str, v3: Option<Adt44>, v4: Option<Adt8>) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(v0)), List::cons((Str::lit("data"), Adt9::C3(v1)), List::cons((Str::lit("mimeType"), Adt9::C3(v2)), {
        let a3152 = f901(v3);
        let a3153 = f896(v4);
        append_list(a3152, a3153)
    }))))
}

pub fn f899(v0: Adt48) -> R<Adt9> {
    Ok(Adt9::C5({
        let a3177 = List::cons((Str::lit("type"), Adt9::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt48::C0(h3154, _, _, _, _, _, _, _, _) => {
                h3154
            }
        })), List::<(Str, Adt9)>::nil()));
        let a3178 = {
            let a3175 = f722(Str::lit("title"), match v0.clone() {
                Adt48::C0(_, h3155, _, _, _, _, _, _, _) => {
                    h3155
                }
            });
            let a3176 = {
                let a3173 = f809(match v0.clone() {
                    Adt48::C0(_, _, h3156, _, _, _, _, _, _) => {
                        h3156
                    }
                });
                let a3174 = {
                    let a3171 = List::cons((Str::lit("uri"), Adt9::C3(match v0.clone() {
                        Adt48::C0(_, _, _, h3157, _, _, _, _, _) => {
                            h3157
                        }
                    })), List::<(Str, Adt9)>::nil());
                    let a3172 = {
                        let a3169 = f722(Str::lit("description"), match v0.clone() {
                            Adt48::C0(_, _, _, _, h3158, _, _, _, _) => {
                                h3158
                            }
                        });
                        let a3170 = {
                            let a3167 = f722(Str::lit("mimeType"), match v0.clone() {
                                Adt48::C0(_, _, _, _, _, h3159, _, _, _) => {
                                    h3159
                                }
                            });
                            let a3168 = {
                                let a3165 = f901(match v0.clone() {
                                    Adt48::C0(_, _, _, _, _, _, h3160, _, _) => {
                                        h3160
                                    }
                                });
                                let a3166 = {
                                    let a3163 = f902(match v0.clone() {
                                        Adt48::C0(_, _, _, _, _, _, _, h3161, _) => {
                                            h3161
                                        }
                                    })?;
                                    let a3164 = f896(match v0 {
                                        Adt48::C0(_, _, _, _, _, _, _, _, h3162) => {
                                            h3162
                                        }
                                    });
                                    append_list(a3163, a3164)
                                };
                                append_list(a3165, a3166)
                            };
                            append_list(a3167, a3168)
                        };
                        append_list(a3169, a3170)
                    };
                    append_list(a3171, a3172)
                };
                append_list(a3173, a3174)
            };
            append_list(a3175, a3176)
        };
        append_list(a3177, a3178)
    }))
}

pub fn f900(v0: Adt49) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f855(match v0.clone() {
        Adt49::C0(h3179, _, _) => {
            h3179
        }
    })), {
        let a3182 = f901(match v0.clone() {
            Adt49::C0(_, h3180, _) => {
                h3180
            }
        });
        let a3183 = f896(match v0 {
            Adt49::C0(_, _, h3181) => {
                h3181
            }
        });
        append_list(a3182, a3183)
    })))
}

pub fn f901(v0: Option<Adt44>) -> List<(Str, Adt9)> {
    let m3184 = v0;
    match m3184 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt9::C5({
                let a3190 = f934(match v1.clone() {
                    Adt44::C0(h3185, _, _) => {
                        h3185
                    }
                });
                let a3191 = {
                    let a3188 = f935(match v1.clone() {
                        Adt44::C0(_, h3186, _) => {
                            h3186
                        }
                    });
                    let a3189 = f722(Str::lit("lastModified"), match v1 {
                        Adt44::C0(_, _, h3187) => {
                            h3187
                        }
                    });
                    append_list(a3188, a3189)
                };
                append_list(a3190, a3191)
            })), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f902(v0: Option<u64>) -> R<List<(Str, Adt9)>> {
    let m3192 = v0;
    match m3192 {
        None => {
            Ok(List::<(Str, Adt9)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f120(v1)?), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f903(v0: List<Adt61>) -> List<Adt9> {
    f581(f936(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f904(v0: Adt45) -> Str {
    let m3193 = v0;
    match m3193 {
        Adt45::C0 => {
            Str::lit("user")
        }
        Adt45::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f905(v0: Adt70) -> Adt9 {
    Adt9::C5({
        let a3200 = {
            let m3195 = match v0.clone() {
                Adt70::C0(h3194, _, _) => {
                    h3194
                }
            };
            match m3195 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt9::C3(Str::lit("form"))), List::<(Str, Adt9)>::nil())
                }
            }
        };
        let a3201 = List::cons((Str::lit("message"), Adt9::C3(match v0.clone() {
            Adt70::C0(_, h3196, _) => {
                h3196
            }
        })), List::cons((Str::lit("requestedSchema"), Adt9::C5({
            let m3199 = match v0 {
                Adt70::C0(_, _, h3198) => {
                    h3198
                }
            };
            match m3199 {
                Adt8::C0(h3197) => {
                    h3197
                }
            }
        })), List::<(Str, Adt9)>::nil()));
        append_list(a3200, a3201)
    })
}

pub fn f906(v0: List<Adt74>) -> R<Option<List<Adt9>>> {
    let m3202 = f937(v0, List::<Adt9>::nil())?;
    match m3202 {
        None => {
            Ok(None::<List<Adt9>>)
        }
        Some(v1) => {
            Ok(Some(f581(v1, List::<Adt9>::nil())))
        }
    }
}

pub fn f907(v0: Adt73) -> List<(Str, Adt9)> {
    let a3213 = {
        let m3204 = match v0.clone() {
            Adt73::C0(_, h3203, _, _, _, _, _, _, _, _) => {
                h3203
            }
        };
        match m3204 {
            None => {
                List::<(Str, Adt9)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f938(v1)), List::<(Str, Adt9)>::nil())
            }
        }
    };
    let a3214 = {
        let a3211 = f722(Str::lit("systemPrompt"), match v0.clone() {
            Adt73::C0(_, _, h3205, _, _, _, _, _, _, _) => {
                h3205
            }
        });
        let a3212 = {
            let a3209 = {
                let m3207 = match v0.clone() {
                    Adt73::C0(_, _, _, h3206, _, _, _, _, _, _) => {
                        h3206
                    }
                };
                match m3207 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt9::C3(f939(v2))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3210 = f940(Str::lit("temperature"), match v0 {
                Adt73::C0(_, _, _, _, h3208, _, _, _, _, _) => {
                    h3208
                }
            });
            append_list(a3209, a3210)
        };
        append_list(a3211, a3212)
    };
    append_list(a3213, a3214)
}

pub fn f908(v0: Adt73) -> List<(Str, Adt9)> {
    let a3226 = {
        let m3216 = match v0.clone() {
            Adt73::C0(_, _, _, _, _, _, h3215, _, _, _) => {
                h3215
            }
        };
        match m3216 {
            None => {
                List::<(Str, Adt9)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f720(v1)), List::<(Str, Adt9)>::nil())
            }
        }
    };
    let a3227 = {
        let a3224 = f763(Str::lit("metadata"), match v0.clone() {
            Adt73::C0(_, _, _, _, _, _, _, h3217, _, _) => {
                h3217
            }
        });
        let a3225 = {
            let a3222 = {
                let m3219 = match v0.clone() {
                    Adt73::C0(_, _, _, _, _, _, _, _, h3218, _) => {
                        h3218
                    }
                };
                match m3219 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt9::C4(f724(v2))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3223 = {
                let m3221 = match v0 {
                    Adt73::C0(_, _, _, _, _, _, _, _, _, h3220) => {
                        h3220
                    }
                };
                match m3221 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt9::C5(f941(v3))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            append_list(a3222, a3223)
        };
        append_list(a3224, a3225)
    };
    append_list(a3226, a3227)
}

pub fn f909(v0: Adt68) -> Option<Adt9> {
    let m3228 = v0;
    match m3228 {
        Adt68::C0(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt9::C5(f942(v1))), List::<(Str, Adt9)>::nil()))))
        }
        Adt68::C1(_) => {
            None::<Adt9>
        }
        Adt68::C2(v3) => {
            Some(Adt9::C5(List::cons((Str::lit("method"), Adt9::C3(Str::lit("roots/list"))), {
                let m3229 = v3;
                match m3229 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt9::C5(match v4 {
                            Adt8::C0(h3230) => {
                                h3230
                            }
                        })), List::<(Str, Adt9)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f910(p3233: List<Adt5>, p3234: List<Adt9>) -> List<Adt9> {
    let m3232 = (p3233, p3234);
    {
        let mut m3232 = m3232;
        loop {
            let step = {
                let (v0, v1) = m3232;
                let m3231 = v0;
                match m3231.uncons() {
                    None => {
                        Err::<(List<Adt5>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt5>, List<Adt9>), List<Adt9>>((v3, List::cons(f943(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f911(v0: Adt8) -> Option<Adt67> {
    let m3235 = f912(Str::lit("taskId"), v0.clone());
    match m3235 {
        None => {
            None::<Adt67>
        }
        Some(v1) => {
            let m3236 = f912(Str::lit("createdAt"), v0.clone());
            match m3236 {
                None => {
                    None::<Adt67>
                }
                Some(v2) => {
                    let m3237 = f912(Str::lit("lastUpdatedAt"), v0.clone());
                    match m3237 {
                        None => {
                            None::<Adt67>
                        }
                        Some(v3) => {
                            Some(Adt67::C0(v1, f912(Str::lit("statusMessage"), v0.clone()), v2, v3, f867(Str::lit("ttlMs"), v0.clone()), f867(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f912(v0: Str, v1: Adt8) -> Option<Str> {
    let m3238 = f235(v0, v1);
    match m3238 {
        Adt116::C0 => {
            None::<Str>
        }
        Adt116::C1(v2) => {
            f287(v2)
        }
    }
}

pub fn f913(v0: Str, v1: Adt8) -> Option<List<(Str, Adt68)>> {
    let m3239 = f865(v0, v1);
    match m3239 {
        None => {
            None::<List<(Str, Adt68)>>
        }
        Some(v2) => {
            f944(match v2 {
                Adt8::C0(h3240) => {
                    h3240
                }
            }, List::<(Str, Adt68)>::nil())
        }
    }
}

pub fn f914(v0: Adt8) -> Option<Adt84> {
    let m3241 = f867(Str::lit("code"), v0.clone());
    match m3241 {
        None => {
            None::<Adt84>
        }
        Some(v1) => {
            let m3242 = f912(Str::lit("message"), v0.clone());
            match m3242 {
                None => {
                    None::<Adt84>
                }
                Some(v2) => {
                    Some(Adt84::C0(v1, v2, {
                        let m3243 = f235(Str::lit("data"), v0);
                        match m3243 {
                            Adt116::C0 => {
                                None::<Adt9>
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

pub fn f915(v0: Adt8) -> Option<Adt90> {
    let m3244 = f865(Str::lit("completes"), v0.clone());
    match m3244 {
        None => {
            let m3245 = f865(Str::lit("fails"), v0);
            match m3245 {
                None => {
                    None::<Adt90>
                }
                Some(v2) => {
                    let m3246 = f914(v2);
                    match m3246 {
                        None => {
                            None::<Adt90>
                        }
                        Some(v3) => {
                            Some(Adt90::C1(v3))
                        }
                    }
                }
            }
        }
        Some(v1) => {
            Some(Adt90::C0(v1))
        }
    }
}

pub fn f916(v0: Adt89) -> Option<Adt9> {
    let m3248 = f805(match v0.clone() {
        Adt89::C0(_, h3247, _, _) => {
            h3247
        }
    });
    match m3248 {
        None => {
            None::<Adt9>
        }
        Some(v1) => {
            Some(Adt9::C5(List::cons((Str::lit("parkAfterMs"), f863(match v0.clone() {
                Adt89::C0(h3249, _, _, _) => {
                    h3249
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f863(match v0.clone() {
                Adt89::C0(_, _, h3250, _) => {
                    h3250
                }
            })), List::cons((Str::lit("outcome"), f945(match v0 {
                Adt89::C0(_, _, _, h3251) => {
                    h3251
                }
            })), List::<(Str, Adt9)>::nil()))))))
        }
    }
}

pub fn f917() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f918() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f919(p3256: List<u64>, p3257: u64, p3258: List<u64>) -> List<u64> {
    let m3255 = (p3256, (p3257, p3258));
    {
        let mut m3255 = m3255;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3255;
                let m3252 = v0;
                match m3252.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f946(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt167 = f947(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt167::C0(h3253, _) => {
                                h3253
                            }
                        }, match v5 {
                            Adt167::C0(_, h3254) => {
                                h3254
                            }
                        })))
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

pub fn f920(v0: Adt154, v1: Adt36) -> bool {
    let m3259 = v1;
    match m3259 {
        Adt36::C0(_) => {
            f948(v0, Adt154::C0)
        }
        Adt36::C1(_) => {
            f948(v0, Adt154::C1)
        }
        Adt36::C2(_) => {
            f948(v0, Adt154::C2)
        }
        Adt36::C3(_) => {
            f948(v0, Adt154::C2)
        }
        Adt36::C4(_) => {
            f948(v0, Adt154::C3)
        }
        Adt36::C5(_) => {
            f948(v0, Adt154::C4)
        }
        Adt36::C6(_) => {
            f948(v0, Adt154::C5)
        }
        Adt36::C7(_) => {
            f948(v0, Adt154::C6)
        }
        Adt36::C8(_) => {
            f948(v0, Adt154::C7)
        }
        Adt36::C9(_) => {
            f948(v0, Adt154::C8)
        }
        Adt36::C10(_) => {
            f948(v0, Adt154::C9)
        }
        Adt36::C11 => {
            if f948(v0.clone(), Adt154::C10) {
                true
            } else {
                f948(v0, Adt154::C11)
            }
        }
        Adt36::C12(_) => {
            f949(v0)
        }
        Adt36::C13(_) => {
            f950(v0)
        }
        Adt36::C14(v15) => {
            let m3261 = match v15 {
                Adt91::C0(_, h3260, _) => {
                    h3260
                }
            };
            match m3261 {
                Adt92::C0 => {
                    f948(v0, Adt154::C9)
                }
                Adt92::C1 => {
                    if f948(v0.clone(), Adt154::C10) {
                        true
                    } else {
                        f948(v0, Adt154::C11)
                    }
                }
            }
        }
    }
}

pub fn f921(v0: Adt3, v1: Adt18, v2: Adt36) -> Result<Adt142, Adt145> {
    let m3262 = v2.clone();
    match m3262 {
        Adt36::C0(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C1(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C2(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C3(v9) => {
            Ok::<Adt142, Adt145>(f955(v0, f952(v1), v9))
        }
        Adt36::C4(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C5(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C6(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C7(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C8(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C9(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C10(_) => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C11 => {
            Ok::<Adt142, Adt145>(f875(v0, v2))
        }
        Adt36::C12(v3) => {
            f951(v0, f952(v1), v3)
        }
        Adt36::C13(v4) => {
            f953(v0, f952(v1), v4)
        }
        Adt36::C14(v5) => {
            f954(v0, f952(v1), v5)
        }
    }
}

pub fn f922() -> Adt93 {
    Adt93::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f923(v0: Bytes, v1: u64, v2: u64, v3: Fn3, v4: Adt18, v5: Result<Adt18, Adt93>) -> R<Result<Adt36, Adt93>> {
    let m3263 = v5;
    match m3263 {
        Ok(v7) => {
            f956(v0, v4, f957(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt36, Adt93>(v6))
        }
    }
}

pub fn f924(v0: Bytes, v1: Adt18) -> R<Result<Adt18, Adt93>> {
    let m3264 = v1.clone();
    match m3264 {
        Adt18::C0(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C1(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C2(v2) => {
            f958(v0, v2)
        }
        Adt18::C3(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C4(v3) => {
            f959(v0, v3)
        }
        Adt18::C5(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C6(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C7(v4) => {
            f960(v0, v4)
        }
        Adt18::C8(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C9(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C10(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
        Adt18::C11(_) => {
            Ok(Ok::<Adt18, Adt93>(v1))
        }
    }
}

pub fn f925(v0: Str, v1: List<(Str, Adt9)>) -> Result<List<(Str, Str)>, Adt146> {
    let m3265 = f961(v0, v1, List::<(Str, Str)>::nil());
    match m3265 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt146>(f962(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt146>(v3)
        }
    }
}

pub fn f926(p3269: List<Adt131>, v1: Adt8) -> R<Option<Adt129>> {
    let m3268 = p3269;
    {
        let mut m3268 = m3268;
        loop {
            let step = {
                let v0 = m3268;
                let m3266 = v0;
                match m3266.uncons() {
                    None => {
                        Err::<List<Adt131>, R<Option<Adt129>>>(Ok(None::<Adt129>))
                    }
                    Some((v2, v3)) => {
                        let m3267 = f963(v2, v1.clone())?;
                        match m3267 {
                            None => {
                                Ok::<List<Adt131>, R<Option<Adt129>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt131>, R<Option<Adt129>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f927(v0: Adt8) -> Adt8 {
    let m3270 = f235(Str::lit("arguments"), v0);
    match m3270 {
        Adt116::C0 => {
            f32()
        }
        Adt116::C1(v1) => {
            let m3271 = f236(v1);
            match m3271 {
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

pub fn f928(v0: Option<Str>, v1: Option<Str>) -> Option<Adt129> {
    let m3272 = v0;
    match m3272 {
        None => {
            Some(Adt129::C2)
        }
        Some(v2) => {
            let m3273 = v1;
            match m3273 {
                None => {
                    None::<Adt129>
                }
                Some(v3) => {
                    let a3274 = v2;
                    let a3275 = v3;
                    if equal(a3274, a3275) {
                        None::<Adt129>
                    } else {
                        Some(Adt129::C5)
                    }
                }
            }
        }
    }
}

pub fn f929(v0: Adt159, v1: Adt8) -> Option<Str> {
    let m3276 = v0;
    match m3276 {
        Adt159::C0 => {
            None::<Str>
        }
        Adt159::C1 => {
            let m3277 = f235(Str::lit("name"), v1);
            match m3277 {
                Adt116::C0 => {
                    None::<Str>
                }
                Adt116::C1(v2) => {
                    f287(v2)
                }
            }
        }
        Adt159::C2 => {
            let m3278 = f235(Str::lit("uri"), v1);
            match m3278 {
                Adt116::C0 => {
                    None::<Str>
                }
                Adt116::C1(v3) => {
                    f287(v3)
                }
            }
        }
        Adt159::C3 => {
            let m3279 = f235(Str::lit("taskId"), v1);
            match m3279 {
                Adt116::C0 => {
                    None::<Str>
                }
                Adt116::C1(v4) => {
                    f287(v4)
                }
            }
        }
    }
}

pub fn f930(v0: Adt9) -> Option<Adt21> {
    let m3280 = f236(v0);
    match m3280 {
        None => {
            None::<Adt21>
        }
        Some(v1) => {
            let m3281 = f964(Str::lit("experimental"), v1.clone());
            match m3281 {
                None => {
                    None::<Adt21>
                }
                Some(v2) => {
                    let m3282 = f964(Str::lit("roots"), v1.clone());
                    match m3282 {
                        None => {
                            None::<Adt21>
                        }
                        Some(v3) => {
                            let m3283 = f964(Str::lit("sampling"), v1.clone());
                            match m3283 {
                                None => {
                                    None::<Adt21>
                                }
                                Some(v4) => {
                                    let m3284 = f964(Str::lit("elicitation"), v1.clone());
                                    match m3284 {
                                        None => {
                                            None::<Adt21>
                                        }
                                        Some(v5) => {
                                            let m3285 = f964(Str::lit("extensions"), v1);
                                            match m3285 {
                                                None => {
                                                    None::<Adt21>
                                                }
                                                Some(v6) => {
                                                    Some(Adt21::C0(v2, v3, v4, v5, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f931() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f932(v0: Adt8) -> Result<Option<Adt22>, Adt146> {
    let m3286 = f879(f965(), v0);
    match m3286 {
        Ok(v2) => {
            let m3287 = v2;
            match m3287 {
                None => {
                    Ok::<Option<Adt22>, Adt146>(None::<Adt22>)
                }
                Some(v3) => {
                    let m3288 = f966(v3);
                    match m3288 {
                        Adt168::C0 => {
                            Err::<Option<Adt22>, Adt146>(Adt146::C8(f965()))
                        }
                        Adt168::C1(v4) => {
                            Ok::<Option<Adt22>, Adt146>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt22>, Adt146>(v1)
        }
    }
}

pub fn f933(v0: Adt8) -> Result<Option<Adt23>, Adt146> {
    let m3289 = f235(f967(), v0);
    match m3289 {
        Adt116::C0 => {
            Ok::<Option<Adt23>, Adt146>(None::<Adt23>)
        }
        Adt116::C1(v1) => {
            let m3290 = f287(v1.clone());
            match m3290 {
                None => {
                    let m3291 = f710(v1);
                    match m3291 {
                        None => {
                            Err::<Option<Adt23>, Adt146>(Adt146::C8(f967()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt23>, Adt146>(Some(Adt23::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt23>, Adt146>(Some(Adt23::C0(v2)))
                }
            }
        }
    }
}

pub fn f934(v0: Option<List<Adt45>>) -> List<(Str, Adt9)> {
    let m3292 = v0;
    match m3292 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt9::C4(f968(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f935(v0: Option<Adt10>) -> List<(Str, Adt9)> {
    let m3293 = v0;
    match m3293 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt9::C2(v1)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f936(p3296: List<Adt61>, p3297: List<Adt9>) -> List<Adt9> {
    let m3295 = (p3296, p3297);
    {
        let mut m3295 = m3295;
        loop {
            let step = {
                let (v0, v1) = m3295;
                let m3294 = v0;
                match m3294.uncons() {
                    None => {
                        Err::<(List<Adt61>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt61>, List<Adt9>), List<Adt9>>((v3, List::cons(f969(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f937(p3301: List<Adt74>, p3302: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3300 = (p3301, p3302);
    {
        let mut m3300 = m3300;
        loop {
            let step = {
                let (v0, v1) = m3300;
                let m3298 = v0;
                match m3298.uncons() {
                    None => {
                        Err::<(List<Adt74>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3299 = f970(v2)?;
                        match m3299 {
                            None => {
                                Err::<(List<Adt74>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(None::<List<Adt9>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt74>, List<Adt9>), R<Option<List<Adt9>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f938(v0: Adt79) -> Adt9 {
    Adt9::C5({
        let a3312 = {
            let m3304 = match v0.clone() {
                Adt79::C0(h3303, _, _, _) => {
                    h3303
                }
            };
            match m3304 {
                None => {
                    List::<(Str, Adt9)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt9::C4(f581(f971(v1, List::<Adt9>::nil()), List::<Adt9>::nil()))), List::<(Str, Adt9)>::nil())
                }
            }
        };
        let a3313 = {
            let a3310 = f940(Str::lit("costPriority"), match v0.clone() {
                Adt79::C0(_, h3305, _, _) => {
                    h3305
                }
            });
            let a3311 = {
                let a3308 = f940(Str::lit("speedPriority"), match v0.clone() {
                    Adt79::C0(_, _, h3306, _) => {
                        h3306
                    }
                });
                let a3309 = f940(Str::lit("intelligencePriority"), match v0 {
                    Adt79::C0(_, _, _, h3307) => {
                        h3307
                    }
                });
                append_list(a3308, a3309)
            };
            append_list(a3310, a3311)
        };
        append_list(a3312, a3313)
    })
}

pub fn f939(v0: Adt81) -> Str {
    let m3314 = v0;
    match m3314 {
        Adt81::C0 => {
            Str::lit("none")
        }
        Adt81::C1 => {
            Str::lit("thisServer")
        }
        Adt81::C2 => {
            Str::lit("allServers")
        }
    }
}

pub fn f940(v0: Str, v1: Option<Adt10>) -> List<(Str, Adt9)> {
    let m3315 = v1;
    match m3315 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt9::C2(v2)), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f941(v0: Adt82) -> List<(Str, Adt9)> {
    let m3317 = match v0 {
        Adt82::C0(h3316) => {
            h3316
        }
    };
    match m3317 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt9::C3(f972(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f942(v0: Adt69) -> List<(Str, Adt9)> {
    let m3318 = v0;
    match m3318 {
        Adt69::C0(v1) => {
            let a3325 = {
                let m3320 = match v1.clone() {
                    Adt70::C0(h3319, _, _) => {
                        h3319
                    }
                };
                match m3320 {
                    None => {
                        List::<(Str, Adt9)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt9::C3(Str::lit("form"))), List::<(Str, Adt9)>::nil())
                    }
                }
            };
            let a3326 = List::cons((Str::lit("message"), Adt9::C3(match v1.clone() {
                Adt70::C0(_, h3321, _) => {
                    h3321
                }
            })), List::cons((Str::lit("requestedSchema"), Adt9::C5({
                let m3324 = match v1 {
                    Adt70::C0(_, _, h3323) => {
                        h3323
                    }
                };
                match m3324 {
                    Adt8::C0(h3322) => {
                        h3322
                    }
                }
            })), List::<(Str, Adt9)>::nil()));
            append_list(a3325, a3326)
        }
        Adt69::C1(v3) => {
            List::cons((Str::lit("mode"), Adt9::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt9::C3(match v3.clone() {
                Adt72::C0(h3327, _) => {
                    h3327
                }
            })), List::cons((Str::lit("url"), Adt9::C3(match v3 {
                Adt72::C0(_, h3328) => {
                    h3328
                }
            })), List::<(Str, Adt9)>::nil())))
        }
    }
}

pub fn f943(v0: Adt5) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("src"), Adt9::C3(match v0.clone() {
        Adt5::C0(h3329, _, _, _) => {
            h3329
        }
    })), {
        let a3335 = f722(Str::lit("mimeType"), match v0.clone() {
            Adt5::C0(_, h3330, _, _) => {
                h3330
            }
        });
        let a3336 = {
            let a3333 = f973(match v0.clone() {
                Adt5::C0(_, _, h3331, _) => {
                    h3331
                }
            });
            let a3334 = f974(match v0 {
                Adt5::C0(_, _, _, h3332) => {
                    h3332
                }
            });
            append_list(a3333, a3334)
        };
        append_list(a3335, a3336)
    }))
}

pub fn f944(p3342: List<(Str, Adt9)>, p3343: List<(Str, Adt68)>) -> Option<List<(Str, Adt68)>> {
    let m3341 = (p3342, p3343);
    {
        let mut m3341 = m3341;
        loop {
            let step = {
                let (v0, v1) = m3341;
                let m3337 = v0;
                match m3337.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Adt68)>), Option<List<(Str, Adt68)>>>(Some(f975(v1, List::<(Str, Adt68)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m3339 = f976({
                            let (_, h3338) = v2;
                            h3338
                        });
                        match m3339 {
                            None => {
                                Err::<(List<(Str, Adt9)>, List<(Str, Adt68)>), Option<List<(Str, Adt68)>>>(None::<List<(Str, Adt68)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt9)>, List<(Str, Adt68)>), Option<List<(Str, Adt68)>>>((v3, List::cons(({
                                    let (h3340, _) = v2;
                                    h3340
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3341 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f945(v0: Adt90) -> Adt9 {
    let m3344 = v0;
    match m3344 {
        Adt90::C0(v1) => {
            Adt9::C5(List::cons((Str::lit("completes"), Adt9::C5(match v1 {
                Adt8::C0(h3345) => {
                    h3345
                }
            })), List::<(Str, Adt9)>::nil()))
        }
        Adt90::C1(v2) => {
            Adt9::C5(List::cons((Str::lit("fails"), f379(v2)), List::<(Str, Adt9)>::nil()))
        }
    }
}

pub fn f946(v0: u64, v1: List<u64>) -> List<u64> {
    let a3346 = v0;
    let a3347 = 0u64;
    if nat_eq(a3346, a3347) {
        v1
    } else {
        let a3348 = v0;
        let a3349 = 1u64;
        if nat_eq(a3348, a3349) {
            List::cons(194u64, v1)
        } else {
            let a3350 = v0;
            let a3351 = 2u64;
            if nat_eq(a3350, a3351) {
                List::cons(226u64, v1)
            } else {
                let a3352 = v0;
                let a3353 = 3u64;
                if nat_eq(a3352, a3353) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f947(v0: u64, v1: u64, v2: List<u64>) -> Adt167 {
    let a3354 = v0;
    let a3355 = 0u64;
    if nat_eq(a3354, a3355) {
        f977(v1, v2)
    } else {
        let a3356 = v0;
        let a3357 = 1u64;
        if nat_eq(a3356, a3357) {
            let a3358 = 128u64;
            let a3359 = v1;
            if if nat_le(a3358, a3359) {
                let a3360 = v1;
                let a3361 = 159u64;
                nat_le(a3360, a3361)
            } else {
                false
            } {
                Adt167::C0(0u64, f978(v2))
            } else {
                f977(v1, List::cons(194u64, v2))
            }
        } else {
            let a3362 = v0;
            let a3363 = 2u64;
            if nat_eq(a3362, a3363) {
                let a3364 = v1;
                let a3365 = 128u64;
                if nat_eq(a3364, a3365) {
                    Adt167::C0(3u64, v2)
                } else {
                    let a3366 = v1;
                    let a3367 = 129u64;
                    if nat_eq(a3366, a3367) {
                        Adt167::C0(4u64, v2)
                    } else {
                        f977(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a3368 = v0;
                let a3369 = 3u64;
                if nat_eq(a3368, a3369) {
                    let a3370 = 170u64;
                    let a3371 = v1;
                    if if nat_le(a3370, a3371) {
                        let a3372 = v1;
                        let a3373 = 174u64;
                        nat_le(a3372, a3373)
                    } else {
                        false
                    } {
                        Adt167::C0(0u64, f978(v2))
                    } else {
                        f977(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a3374 = 166u64;
                    let a3375 = v1;
                    if if nat_le(a3374, a3375) {
                        let a3376 = v1;
                        let a3377 = 169u64;
                        nat_le(a3376, a3377)
                    } else {
                        false
                    } {
                        Adt167::C0(0u64, f978(v2))
                    } else {
                        f977(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f948(v0: Adt154, v1: Adt154) -> bool {
    let a3378 = f767(v0);
    let a3379 = f767(v1);
    equal(a3378, a3379)
}

pub fn f949(v0: Adt154) -> bool {
    let m3380 = v0;
    match m3380 {
        Adt154::C0 => {
            false
        }
        Adt154::C1 => {
            false
        }
        Adt154::C2 => {
            true
        }
        Adt154::C3 => {
            false
        }
        Adt154::C4 => {
            true
        }
        Adt154::C5 => {
            false
        }
        Adt154::C6 => {
            false
        }
        Adt154::C7 => {
            true
        }
        Adt154::C8 => {
            false
        }
        Adt154::C9 => {
            false
        }
        Adt154::C10 => {
            false
        }
        Adt154::C11 => {
            false
        }
    }
}

pub fn f950(v0: Adt154) -> bool {
    let m3381 = v0;
    match m3381 {
        Adt154::C0 => {
            false
        }
        Adt154::C1 => {
            false
        }
        Adt154::C2 => {
            true
        }
        Adt154::C3 => {
            false
        }
        Adt154::C4 => {
            false
        }
        Adt154::C5 => {
            false
        }
        Adt154::C6 => {
            false
        }
        Adt154::C7 => {
            false
        }
        Adt154::C8 => {
            false
        }
        Adt154::C9 => {
            false
        }
        Adt154::C10 => {
            false
        }
        Adt154::C11 => {
            false
        }
    }
}

pub fn f951(v0: Adt3, v1: Adt20, v2: Adt85) -> Result<Adt142, Adt145> {
    f979(v0, v2.clone(), f980(match v1 {
        Adt20::C0(_, h3382, _, _, _, _) => {
            h3382
        }
    }, f735(v2)))
}

pub fn f952(v0: Adt18) -> Adt20 {
    let m3383 = v0;
    match m3383 {
        Adt18::C0(v1) => {
            match v1 {
                Adt19::C0(h3384) => {
                    h3384
                }
            }
        }
        Adt18::C1(v2) => {
            match v2 {
                Adt24::C0(h3385, _) => {
                    h3385
                }
            }
        }
        Adt18::C2(v3) => {
            match v3 {
                Adt25::C0(h3386, _, _, _, _) => {
                    h3386
                }
            }
        }
        Adt18::C3(v4) => {
            match v4 {
                Adt24::C0(h3387, _) => {
                    h3387
                }
            }
        }
        Adt18::C4(v5) => {
            match v5 {
                Adt26::C0(h3388, _, _, _) => {
                    h3388
                }
            }
        }
        Adt18::C5(v6) => {
            match v6 {
                Adt24::C0(h3389, _) => {
                    h3389
                }
            }
        }
        Adt18::C6(v7) => {
            match v7 {
                Adt24::C0(h3390, _) => {
                    h3390
                }
            }
        }
        Adt18::C7(v8) => {
            match v8 {
                Adt27::C0(h3391, _, _, _, _) => {
                    h3391
                }
            }
        }
        Adt18::C8(v9) => {
            match v9 {
                Adt28::C0(h3392, _, _, _) => {
                    h3392
                }
            }
        }
        Adt18::C9(v10) => {
            match v10 {
                Adt33::C0(h3393, _) => {
                    h3393
                }
            }
        }
        Adt18::C10(v11) => {
            match v11 {
                Adt34::C0(h3394, _, _) => {
                    h3394
                }
            }
        }
        Adt18::C11(v12) => {
            match v12 {
                Adt35::C0(h3395, _) => {
                    h3395
                }
            }
        }
    }
}

pub fn f953(v0: Adt3, v1: Adt20, v2: Adt86) -> Result<Adt142, Adt145> {
    f981(v0, v2, f982(match v1 {
        Adt20::C0(_, h3396, _, _, _, _) => {
            h3396
        }
    }))
}

pub fn f954(v0: Adt3, v1: Adt20, v2: Adt91) -> Result<Adt142, Adt145> {
    f983(v0, v2, f982(match v1 {
        Adt20::C0(_, h3397, _, _, _, _) => {
            h3397
        }
    }))
}

pub fn f955(v0: Adt3, v1: Adt20, v2: Adt53) -> Adt142 {
    Adt142::C0(Adt36::C2(match v2.clone() {
        Adt53::C0(h3398, _) => {
            h3398
        }
    }), {
        let m3400 = f875(v0, Adt36::C11);
        match m3400 {
            Adt142::C0(_, h3399, _) => {
                h3399
            }
        }
    }, f984(v1, match v2 {
        Adt53::C0(_, h3401) => {
            h3401
        }
    }))
}

pub fn f956(v0: Bytes, v1: Adt18, v2: Result<Adt36, Adt93>) -> R<Result<Adt36, Adt93>> {
    let m3402 = v2;
    match m3402 {
        Ok(v4) => {
            f985(v0, f874(v1.clone()), f986(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt36, Adt93>(v3))
        }
    }
}

pub fn f957(v0: u64, v1: u64, v2: Fn3, v3: Adt18) -> R<Result<Adt36, Adt93>> {
    let m3403 = v3.clone();
    match m3403 {
        Adt18::C0(_) => {
            let c3405: Fn3 = v2;
            c3405.apply(v3)
        }
        Adt18::C1(_) => {
            let c3406: Fn3 = v2;
            c3406.apply(v3)
        }
        Adt18::C2(_) => {
            if f987(v0, v1) {
                let c3404: Fn3 = v2;
                c3404.apply(v3)
            } else {
                Ok(Err::<Adt36, Adt93>(Adt93::C7(v0)))
            }
        }
        Adt18::C3(_) => {
            let c3407: Fn3 = v2;
            c3407.apply(v3)
        }
        Adt18::C4(_) => {
            let c3408: Fn3 = v2;
            c3408.apply(v3)
        }
        Adt18::C5(_) => {
            let c3409: Fn3 = v2;
            c3409.apply(v3)
        }
        Adt18::C6(_) => {
            let c3410: Fn3 = v2;
            c3410.apply(v3)
        }
        Adt18::C7(_) => {
            let c3411: Fn3 = v2;
            c3411.apply(v3)
        }
        Adt18::C8(_) => {
            let c3412: Fn3 = v2;
            c3412.apply(v3)
        }
        Adt18::C9(_) => {
            let c3413: Fn3 = v2;
            c3413.apply(v3)
        }
        Adt18::C10(_) => {
            let c3414: Fn3 = v2;
            c3414.apply(v3)
        }
        Adt18::C11(_) => {
            let c3415: Fn3 = v2;
            c3415.apply(v3)
        }
    }
}

pub fn f958(v0: Bytes, v1: Adt25) -> R<Result<Adt18, Adt93>> {
    let m3419 = f988(v0, Str::lit("tools/call"), match v1.clone() {
        Adt25::C0(_, h3416, _, _, _) => {
            h3416
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, _, h3417) => {
            h3417
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, h3418, _) => {
            h3418
        }
    })?;
    match m3419 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt93>(Adt18::C2(Adt25::C0(match v1.clone() {
                Adt25::C0(h3420, _, _, _, _) => {
                    h3420
                }
            }, match v1.clone() {
                Adt25::C0(_, h3421, _, _, _) => {
                    h3421
                }
            }, match v1.clone() {
                Adt25::C0(_, _, h3422, _, _) => {
                    h3422
                }
            }, match v1 {
                Adt25::C0(_, _, _, h3423, _) => {
                    h3423
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt93>(v2))
        }
    }
}

pub fn f959(v0: Bytes, v1: Adt26) -> R<Result<Adt18, Adt93>> {
    let m3427 = f988(v0, Str::lit("resources/read"), match v1.clone() {
        Adt26::C0(_, h3424, _, _) => {
            h3424
        }
    }, match v1.clone() {
        Adt26::C0(_, _, _, h3425) => {
            h3425
        }
    }, match v1.clone() {
        Adt26::C0(_, _, h3426, _) => {
            h3426
        }
    })?;
    match m3427 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt93>(Adt18::C4(Adt26::C0(match v1.clone() {
                Adt26::C0(h3428, _, _, _) => {
                    h3428
                }
            }, match v1.clone() {
                Adt26::C0(_, h3429, _, _) => {
                    h3429
                }
            }, match v1 {
                Adt26::C0(_, _, h3430, _) => {
                    h3430
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt93>(v2))
        }
    }
}

pub fn f960(v0: Bytes, v1: Adt27) -> R<Result<Adt18, Adt93>> {
    let m3434 = f988(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt27::C0(_, h3431, _, _, _) => {
            h3431
        }
    }, match v1.clone() {
        Adt27::C0(_, _, _, _, h3432) => {
            h3432
        }
    }, match v1.clone() {
        Adt27::C0(_, _, _, h3433, _) => {
            h3433
        }
    })?;
    match m3434 {
        Ok(v3) => {
            Ok(Ok::<Adt18, Adt93>(Adt18::C7(Adt27::C0(match v1.clone() {
                Adt27::C0(h3435, _, _, _, _) => {
                    h3435
                }
            }, match v1.clone() {
                Adt27::C0(_, h3436, _, _, _) => {
                    h3436
                }
            }, match v1.clone() {
                Adt27::C0(_, _, h3437, _, _) => {
                    h3437
                }
            }, match v1 {
                Adt27::C0(_, _, _, h3438, _) => {
                    h3438
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt18, Adt93>(v2))
        }
    }
}

pub fn f961(v0: Str, p3444: List<(Str, Adt9)>, p3445: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt146> {
    let m3443 = (p3444, p3445);
    {
        let mut m3443 = m3443;
        loop {
            let step = {
                let (v1, v2) = m3443;
                let m3439 = v1;
                match m3439.uncons() {
                    None => {
                        Err::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt146>>(Ok::<List<(Str, Str)>, Adt146>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3441 = f287({
                            let (_, h3440) = v3;
                            h3440
                        });
                        match m3441 {
                            None => {
                                Err::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt146>>(Err::<List<(Str, Str)>, Adt146>(Adt146::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt9)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt146>>((v4, List::cons(({
                                    let (h3442, _) = v3;
                                    h3442
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f962(p3448: List<(Str, Str)>, p3449: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3447 = (p3448, p3449);
    {
        let mut m3447 = m3447;
        loop {
            let step = {
                let (v0, v1) = m3447;
                let m3446 = v0;
                match m3446.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f963(v0: Adt131, v1: Adt8) -> R<Option<Adt129>> {
    let m3451 = f989(match v0.clone() {
        Adt131::C0(_, h3450, _, _) => {
            h3450
        }
    }, v1);
    match m3451 {
        Adt116::C0 => {
            Ok(f990(v0))
        }
        Adt116::C1(v2) => {
            if f991(v2.clone()) {
                Ok(f990(v0))
            } else {
                f992(v0, v2)
            }
        }
    }
}

pub fn f964(v0: Str, v1: Adt8) -> Option<Option<Adt8>> {
    let m3452 = f235(v0, v1);
    match m3452 {
        Adt116::C0 => {
            Some(None::<Adt8>)
        }
        Adt116::C1(v2) => {
            let m3453 = f236(v2);
            match m3453 {
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

pub fn f965() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f966(v0: Str) -> Adt168 {
    let a3454 = v0.clone();
    let a3455 = Str::lit("debug");
    if equal(a3454, a3455) {
        Adt168::C1(Adt22::C0)
    } else {
        let a3456 = v0.clone();
        let a3457 = Str::lit("info");
        if equal(a3456, a3457) {
            Adt168::C1(Adt22::C1)
        } else {
            let a3458 = v0.clone();
            let a3459 = Str::lit("notice");
            if equal(a3458, a3459) {
                Adt168::C1(Adt22::C2)
            } else {
                let a3460 = v0.clone();
                let a3461 = Str::lit("warning");
                if equal(a3460, a3461) {
                    Adt168::C1(Adt22::C3)
                } else {
                    let a3462 = v0.clone();
                    let a3463 = Str::lit("error");
                    if equal(a3462, a3463) {
                        Adt168::C1(Adt22::C4)
                    } else {
                        let a3464 = v0.clone();
                        let a3465 = Str::lit("critical");
                        if equal(a3464, a3465) {
                            Adt168::C1(Adt22::C5)
                        } else {
                            let a3466 = v0.clone();
                            let a3467 = Str::lit("alert");
                            if equal(a3466, a3467) {
                                Adt168::C1(Adt22::C6)
                            } else {
                                let a3468 = v0;
                                let a3469 = Str::lit("emergency");
                                if equal(a3468, a3469) {
                                    Adt168::C1(Adt22::C7)
                                } else {
                                    Adt168::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f967() -> Str {
    Str::lit("progressToken")
}

pub fn f968(v0: List<Adt45>) -> List<Adt9> {
    f581(f993(v0, List::<Adt9>::nil()), List::<Adt9>::nil())
}

pub fn f969(v0: Adt61) -> Adt9 {
    Adt9::C5({
        let a3478 = List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
            Adt61::C0(h3470, _, _, _) => {
                h3470
            }
        })), List::<(Str, Adt9)>::nil());
        let a3479 = {
            let a3476 = f722(Str::lit("title"), match v0.clone() {
                Adt61::C0(_, h3471, _, _) => {
                    h3471
                }
            });
            let a3477 = {
                let a3474 = f722(Str::lit("description"), match v0.clone() {
                    Adt61::C0(_, _, h3472, _) => {
                        h3472
                    }
                });
                let a3475 = f727(Str::lit("required"), match v0 {
                    Adt61::C0(_, _, _, h3473) => {
                        h3473
                    }
                });
                append_list(a3474, a3475)
            };
            append_list(a3476, a3477)
        };
        append_list(a3478, a3479)
    })
}

pub fn f970(v0: Adt74) -> R<Option<Adt9>> {
    let m3481 = f994(match v0.clone() {
        Adt74::C0(_, h3480, _) => {
            h3480
        }
    })?;
    match m3481 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("role"), Adt9::C3(f904(match v0.clone() {
                Adt74::C0(h3482, _, _) => {
                    h3482
                }
            }))), List::cons((Str::lit("content"), v1), f896(match v0 {
                Adt74::C0(_, _, h3483) => {
                    h3483
                }
            }))))))
        }
    }
}

pub fn f971(p3487: List<Adt80>, p3488: List<Adt9>) -> List<Adt9> {
    let m3486 = (p3487, p3488);
    {
        let mut m3486 = m3486;
        loop {
            let step = {
                let (v0, v1) = m3486;
                let m3484 = v0;
                match m3484.uncons() {
                    None => {
                        Err::<(List<Adt80>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt80>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C5(f722(Str::lit("name"), match v2 {
                            Adt80::C0(h3485) => {
                                h3485
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f972(v0: Adt83) -> Str {
    let m3489 = v0;
    match m3489 {
        Adt83::C0 => {
            Str::lit("auto")
        }
        Adt83::C1 => {
            Str::lit("required")
        }
        Adt83::C2 => {
            Str::lit("none")
        }
    }
}

pub fn f973(v0: Option<List<Str>>) -> List<(Str, Adt9)> {
    let m3490 = v0;
    match m3490 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt9::C4(f669(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f974(v0: Option<Adt6>) -> List<(Str, Adt9)> {
    let m3491 = v0;
    match m3491 {
        None => {
            List::<(Str, Adt9)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt9::C3(f995(v1))), List::<(Str, Adt9)>::nil())
        }
    }
}

pub fn f975(p3494: List<(Str, Adt68)>, p3495: List<(Str, Adt68)>) -> List<(Str, Adt68)> {
    let m3493 = (p3494, p3495);
    {
        let mut m3493 = m3493;
        loop {
            let step = {
                let (v0, v1) = m3493;
                let m3492 = v0;
                match m3492.uncons() {
                    None => {
                        Err::<(List<(Str, Adt68)>, List<(Str, Adt68)>), List<(Str, Adt68)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt68)>, List<(Str, Adt68)>), List<(Str, Adt68)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f976(v0: Adt9) -> Option<Adt68> {
    let m3496 = f236(v0);
    match m3496 {
        None => {
            None::<Adt68>
        }
        Some(v1) => {
            let m3497 = f912(Str::lit("method"), v1.clone());
            match m3497 {
                None => {
                    None::<Adt68>
                }
                Some(v2) => {
                    let a3498 = v2.clone();
                    let a3499 = Str::lit("elicitation/create");
                    if equal(a3498, a3499) {
                        let m3500 = f865(Str::lit("params"), v1);
                        match m3500 {
                            None => {
                                None::<Adt68>
                            }
                            Some(v3) => {
                                f996(v3)
                            }
                        }
                    } else {
                        let a3501 = v2;
                        let a3502 = Str::lit("roots/list");
                        if equal(a3501, a3502) {
                            f997(v1)
                        } else {
                            None::<Adt68>
                        }
                    }
                }
            }
        }
    }
}

pub fn f977(v0: u64, v1: List<u64>) -> Adt167 {
    let a3503 = v0;
    let a3504 = 32u64;
    if nat_lt(a3503, a3504) {
        if f998(v0) {
            Adt167::C0(0u64, List::cons(v0, v1))
        } else {
            Adt167::C0(0u64, f978(v1))
        }
    } else {
        let a3505 = v0;
        let a3506 = 127u64;
        if nat_eq(a3505, a3506) {
            Adt167::C0(0u64, f978(v1))
        } else {
            let a3507 = v0;
            let a3508 = 194u64;
            if nat_eq(a3507, a3508) {
                Adt167::C0(1u64, v1)
            } else {
                let a3509 = v0;
                let a3510 = 226u64;
                if nat_eq(a3509, a3510) {
                    Adt167::C0(2u64, v1)
                } else {
                    Adt167::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f978(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f979(v0: Adt3, v1: Adt85, v2: bool) -> Result<Adt142, Adt145> {
    if v2 {
        Ok::<Adt142, Adt145>(f875(v0, Adt36::C12(v1)))
    } else {
        Err::<Adt142, Adt145>(Adt145::C4(Adt93::C8(f999(f735(v1)))))
    }
}

pub fn f980(v0: Adt21, v1: List<(Str, Adt68)>) -> bool {
    let a3511 = f1000(v1.clone());
    if if if bool_not(a3511) {
        true
    } else {
        f298(match v0.clone() {
            Adt21::C0(_, _, _, h3512, _) => {
                h3512
            }
        })
    } {
        let a3513 = f1001(v1.clone());
        if bool_not(a3513) {
            true
        } else {
            f298(match v0.clone() {
                Adt21::C0(_, _, h3514, _, _) => {
                    h3514
                }
            })
        }
    } else {
        false
    } {
        let a3515 = f1002(v1);
        if bool_not(a3515) {
            true
        } else {
            f298(match v0 {
                Adt21::C0(_, h3516, _, _, _) => {
                    h3516
                }
            })
        }
    } else {
        false
    }
}

pub fn f981(v0: Adt3, v1: Adt86, v2: bool) -> Result<Adt142, Adt145> {
    if v2 {
        Ok::<Adt142, Adt145>(f875(v0, Adt36::C13(v1)))
    } else {
        Err::<Adt142, Adt145>(Adt145::C4(Adt93::C8(f1003())))
    }
}

pub fn f982(v0: Adt21) -> bool {
    f770(match v0 {
        Adt21::C0(_, _, _, _, h3517) => {
            h3517
        }
    }, f33())
}

pub fn f983(v0: Adt3, v1: Adt91, v2: bool) -> Result<Adt142, Adt145> {
    if v2 {
        Ok::<Adt142, Adt145>(f875(v0, Adt36::C14(v1)))
    } else {
        Err::<Adt142, Adt145>(Adt145::C4(Adt93::C8(f1003())))
    }
}

pub fn f984(v0: Adt20, v1: List<Adt54>) -> List<Adt144> {
    let m3519 = match v0 {
        Adt20::C0(_, _, _, _, h3518, _) => {
            h3518
        }
    };
    match m3519 {
        None => {
            List::<Adt144>::nil()
        }
        Some(v2) => {
            f1004(f1005(v2, v1, List::<Adt144>::nil()), List::<Adt144>::nil())
        }
    }
}

pub fn f985(v0: Bytes, v1: Adt154, v2: Str, v3: Adt36) -> R<Result<Adt36, Adt93>> {
    let m3520 = v3.clone();
    match m3520 {
        Adt36::C0(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C1(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C2(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C3(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C4(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C5(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C6(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C7(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C8(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C9(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C10(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C11 => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C12(v4) => {
            f1006(v0, v1, v2, v4)
        }
        Adt36::C13(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
        Adt36::C14(_) => {
            Ok(Ok::<Adt36, Adt93>(v3))
        }
    }
}

pub fn f986(v0: Adt18) -> Str {
    let m3521 = v0;
    match m3521 {
        Adt18::C0(_) => {
            Str::lit("")
        }
        Adt18::C1(_) => {
            Str::lit("")
        }
        Adt18::C2(v1) => {
            match v1 {
                Adt25::C0(_, h3522, _, _, _) => {
                    h3522
                }
            }
        }
        Adt18::C3(_) => {
            Str::lit("")
        }
        Adt18::C4(v2) => {
            match v2 {
                Adt26::C0(_, h3523, _, _) => {
                    h3523
                }
            }
        }
        Adt18::C5(_) => {
            Str::lit("")
        }
        Adt18::C6(_) => {
            Str::lit("")
        }
        Adt18::C7(v3) => {
            match v3 {
                Adt27::C0(_, h3524, _, _, _) => {
                    h3524
                }
            }
        }
        Adt18::C8(_) => {
            Str::lit("")
        }
        Adt18::C9(_) => {
            Str::lit("")
        }
        Adt18::C10(_) => {
            Str::lit("")
        }
        Adt18::C11(_) => {
            Str::lit("")
        }
    }
}

pub fn f987(v0: u64, v1: u64) -> bool {
    let a3525 = v1;
    let a3526 = v0;
    nat_lt(a3525, a3526)
}

pub fn f988(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt8>) -> R<Result<Option<Str>, Adt93>> {
    Ok(f1007(f1008(v0, v1, v2, v3)?, f1009(v4)))
}

pub fn f989(p3532: List<Str>, p3533: Adt8) -> Adt116 {
    let m3531 = (p3532, p3533);
    {
        let mut m3531 = m3531;
        loop {
            let step = {
                let (v0, v1) = m3531;
                let m3527 = v0;
                match m3527.uncons() {
                    None => {
                        Err::<(List<Str>, Adt8), Adt116>(Adt116::C0)
                    }
                    Some((v2, v3)) => {
                        let m3528 = f235(v2, v1);
                        match m3528 {
                            Adt116::C0 => {
                                Err::<(List<Str>, Adt8), Adt116>(Adt116::C0)
                            }
                            Adt116::C1(v4) => {
                                let m3529 = v3.clone();
                                match m3529.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt8), Adt116>(Adt116::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3530 = f236(v4);
                                        match m3530 {
                                            None => {
                                                Err::<(List<Str>, Adt8), Adt116>(Adt116::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt8), Adt116>((v3, v7))
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
                Ok(next) => { m3531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f990(v0: Adt131) -> Option<Adt129> {
    let m3535 = match v0.clone() {
        Adt131::C0(_, _, _, h3534) => {
            h3534
        }
    };
    match m3535 {
        Adt132::C0 => {
            None::<Adt129>
        }
        Adt132::C1 => {
            Some(Adt129::C10(match v0 {
                Adt131::C0(h3536, _, _, _) => {
                    h3536
                }
            }))
        }
        Adt132::C2(_) => {
            None::<Adt129>
        }
    }
}

pub fn f991(v0: Adt9) -> bool {
    let m3537 = v0;
    match m3537 {
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

pub fn f992(v0: Adt131, v1: Adt9) -> R<Option<Adt129>> {
    let m3539 = match v0.clone() {
        Adt131::C0(_, _, _, h3538) => {
            h3538
        }
    };
    match m3539 {
        Adt132::C0 => {
            Ok(Some(Adt129::C8(match v0 {
                Adt131::C0(h3540, _, _, _) => {
                    h3540
                }
            })))
        }
        Adt132::C1 => {
            Ok(Some(Adt129::C10(match v0 {
                Adt131::C0(h3541, _, _, _) => {
                    h3541
                }
            })))
        }
        Adt132::C2(v2) => {
            if f1010(match v0.clone() {
                Adt131::C0(_, _, h3542, _) => {
                    h3542
                }
            }, v2, v1)? {
                Ok(None::<Adt129>)
            } else {
                Ok(Some(Adt129::C9(match v0 {
                    Adt131::C0(h3543, _, _, _) => {
                        h3543
                    }
                })))
            }
        }
    }
}

pub fn f993(p3546: List<Adt45>, p3547: List<Adt9>) -> List<Adt9> {
    let m3545 = (p3546, p3547);
    {
        let mut m3545 = m3545;
        loop {
            let step = {
                let (v0, v1) = m3545;
                let m3544 = v0;
                match m3544.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt9>), List<Adt9>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt9>), List<Adt9>>((v3, List::cons(Adt9::C3(f904(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f994(v0: Adt75) -> R<Option<Adt9>> {
    let m3548 = v0;
    match m3548 {
        Adt75::C0(v1) => {
            f1011(v1)
        }
        Adt75::C1(v2) => {
            let m3549 = f1012(v2, List::<Adt9>::nil())?;
            match m3549 {
                None => {
                    Ok(None::<Adt9>)
                }
                Some(v3) => {
                    Ok(Some(Adt9::C4(f581(v3, List::<Adt9>::nil()))))
                }
            }
        }
    }
}

pub fn f995(v0: Adt6) -> Str {
    let m3550 = v0;
    match m3550 {
        Adt6::C0 => {
            Str::lit("light")
        }
        Adt6::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f996(v0: Adt8) -> Option<Adt68> {
    let m3551 = f912(Str::lit("message"), v0.clone());
    match m3551 {
        None => {
            None::<Adt68>
        }
        Some(v1) => {
            let m3552 = f235(Str::lit("mode"), v0.clone());
            match m3552 {
                Adt116::C0 => {
                    let m3553 = f865(Str::lit("requestedSchema"), v0);
                    match m3553 {
                        None => {
                            None::<Adt68>
                        }
                        Some(v2) => {
                            Some(Adt68::C0(Adt69::C0(Adt70::C0(None::<Adt71>, v1, v2))))
                        }
                    }
                }
                Adt116::C1(v3) => {
                    let m3554 = f287(v3);
                    match m3554 {
                        None => {
                            None::<Adt68>
                        }
                        Some(v4) => {
                            let a3555 = v4.clone();
                            let a3556 = Str::lit("url");
                            if equal(a3555, a3556) {
                                let m3557 = f912(Str::lit("url"), v0);
                                match m3557 {
                                    None => {
                                        None::<Adt68>
                                    }
                                    Some(v5) => {
                                        Some(Adt68::C0(Adt69::C1(Adt72::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a3558 = v4;
                                let a3559 = Str::lit("form");
                                if equal(a3558, a3559) {
                                    let m3560 = f865(Str::lit("requestedSchema"), v0);
                                    match m3560 {
                                        None => {
                                            None::<Adt68>
                                        }
                                        Some(v6) => {
                                            Some(Adt68::C0(Adt69::C0(Adt70::C0(Some(Adt71::C0), v1, v6))))
                                        }
                                    }
                                } else {
                                    None::<Adt68>
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f997(v0: Adt8) -> Option<Adt68> {
    let m3561 = f235(Str::lit("params"), v0);
    match m3561 {
        Adt116::C0 => {
            Some(Adt68::C2(None::<Adt8>))
        }
        Adt116::C1(v1) => {
            let m3562 = f236(v1);
            match m3562 {
                None => {
                    None::<Adt68>
                }
                Some(v2) => {
                    Some(Adt68::C2(Some(v2)))
                }
            }
        }
    }
}

pub fn f998(v0: u64) -> bool {
    let a3563 = v0;
    let a3564 = 9u64;
    if nat_eq(a3563, a3564) {
        true
    } else {
        let a3565 = v0;
        let a3566 = 10u64;
        if nat_eq(a3565, a3566) {
            true
        } else {
            let a3567 = v0;
            let a3568 = 13u64;
            nat_eq(a3567, a3568)
        }
    }
}

pub fn f999(v0: List<(Str, Adt68)>) -> Adt21 {
    Adt21::C0(None::<Adt8>, if f1002(v0.clone()) {
        Some(f32())
    } else {
        None::<Adt8>
    }, if f1001(v0.clone()) {
        Some(f32())
    } else {
        None::<Adt8>
    }, if f1000(v0) {
        Some(f32())
    } else {
        None::<Adt8>
    }, None::<Adt8>)
}

pub fn f1000(p3573: List<(Str, Adt68)>) -> bool {
    let m3572 = p3573;
    {
        let mut m3572 = m3572;
        loop {
            let step = {
                let v0 = m3572;
                let m3569 = v0;
                match m3569.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3571 = {
                            let (_, h3570) = v1;
                            h3570
                        };
                        match m3571 {
                            Adt68::C0(_) => {
                                Err::<List<(Str, Adt68)>, bool>(true)
                            }
                            Adt68::C1(_) => {
                                Ok::<List<(Str, Adt68)>, bool>(v2)
                            }
                            Adt68::C2(_) => {
                                Ok::<List<(Str, Adt68)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1001(p3578: List<(Str, Adt68)>) -> bool {
    let m3577 = p3578;
    {
        let mut m3577 = m3577;
        loop {
            let step = {
                let v0 = m3577;
                let m3574 = v0;
                match m3574.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3576 = {
                            let (_, h3575) = v1;
                            h3575
                        };
                        match m3576 {
                            Adt68::C0(_) => {
                                Ok::<List<(Str, Adt68)>, bool>(v2)
                            }
                            Adt68::C1(_) => {
                                Err::<List<(Str, Adt68)>, bool>(true)
                            }
                            Adt68::C2(_) => {
                                Ok::<List<(Str, Adt68)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1002(p3583: List<(Str, Adt68)>) -> bool {
    let m3582 = p3583;
    {
        let mut m3582 = m3582;
        loop {
            let step = {
                let v0 = m3582;
                let m3579 = v0;
                match m3579.uncons() {
                    None => {
                        Err::<List<(Str, Adt68)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3581 = {
                            let (_, h3580) = v1;
                            h3580
                        };
                        match m3581 {
                            Adt68::C0(_) => {
                                Ok::<List<(Str, Adt68)>, bool>(v2)
                            }
                            Adt68::C1(_) => {
                                Ok::<List<(Str, Adt68)>, bool>(v2)
                            }
                            Adt68::C2(_) => {
                                Err::<List<(Str, Adt68)>, bool>(true)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1003() -> Adt21 {
    Adt21::C0(None::<Adt8>, None::<Adt8>, None::<Adt8>, None::<Adt8>, Some(Adt8::C0(List::cons((f33(), Adt9::C5(List::<(Str, Adt9)>::nil())), List::<(Str, Adt9)>::nil()))))
}

pub fn f1004(p3586: List<Adt144>, p3587: List<Adt144>) -> List<Adt144> {
    let m3585 = (p3586, p3587);
    {
        let mut m3585 = m3585;
        loop {
            let step = {
                let (v0, v1) = m3585;
                let m3584 = v0;
                match m3584.uncons() {
                    None => {
                        Err::<(List<Adt144>, List<Adt144>), List<Adt144>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt144>, List<Adt144>), List<Adt144>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3585 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1005(v0: Adt23, p3590: List<Adt54>, p3591: List<Adt144>) -> List<Adt144> {
    let m3589 = (p3590, p3591);
    {
        let mut m3589 = m3589;
        loop {
            let step = {
                let (v1, v2) = m3589;
                let m3588 = v1;
                match m3588.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt144>), List<Adt144>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt54>, List<Adt144>), List<Adt144>>((v4, List::cons(Adt144::C0(v0.clone(), v3), v2)))
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

pub fn f1006(v0: Bytes, v1: Adt154, v2: Str, v3: Adt85) -> R<Result<Adt36, Adt93>> {
    if f949(v1.clone()) {
        if f1013(v3.clone()) {
            let m3592 = f1014(v0, f767(v1), v2, v3)?;
            match m3592 {
                Ok(v5) => {
                    Ok(Ok::<Adt36, Adt93>(Adt36::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt36, Adt93>(v4))
                }
            }
        } else {
            Ok(Err::<Adt36, Adt93>(Adt93::C13))
        }
    } else {
        Ok(Err::<Adt36, Adt93>(Adt93::C12(f767(v1))))
    }
}

pub fn f1007(v0: Result<Option<Str>, Adt93>, v1: bool) -> Result<Option<Str>, Adt93> {
    let m3593 = v0;
    match m3593 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt93>(v3)
            } else {
                Err::<Option<Str>, Adt93>(Adt93::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt93>(v2)
        }
    }
}

pub fn f1008(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt93>> {
    let m3594 = v3;
    match m3594 {
        None => {
            Ok(Ok::<Option<Str>, Adt93>(None::<Str>))
        }
        Some(v4) => {
            Ok(f1015(f1016(v0, v1, v2, v4)?))
        }
    }
}

pub fn f1009(v0: Option<Adt8>) -> bool {
    let m3595 = v0;
    match m3595 {
        None => {
            true
        }
        Some(v1) => {
            f1017(match v1 {
                Adt8::C0(h3596) => {
                    h3596
                }
            })
        }
    }
}

pub fn f1010(v0: Adt16, v1: Str, v2: Adt9) -> R<bool> {
    let m3597 = v0;
    match m3597 {
        Adt16::C0 => {
            let m3598 = f287(v2);
            match m3598 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3599 = v1;
                    let a3600 = v3;
                    Ok(equal(a3599, a3600))
                }
            }
        }
        Adt16::C1 => {
            f1018(v1, v2)
        }
        Adt16::C2 => {
            let m3601 = f360(v2);
            match m3601 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3602 = v1;
                        let a3603 = Str::lit("true");
                        Ok(equal(a3602, a3603))
                    } else {
                        let a3604 = v1;
                        let a3605 = Str::lit("false");
                        Ok(equal(a3604, a3605))
                    }
                }
            }
        }
    }
}

pub fn f1011(v0: Adt76) -> R<Option<Adt9>> {
    let m3606 = v0;
    match m3606 {
        Adt76::C0(v1) => {
            Ok(Some(f897(v1)))
        }
        Adt76::C1(v2) => {
            Ok(Some(f898(Str::lit("image"), match v2.clone() {
                Adt46::C0(h3607, _, _, _) => {
                    h3607
                }
            }, match v2.clone() {
                Adt46::C0(_, h3608, _, _) => {
                    h3608
                }
            }, match v2.clone() {
                Adt46::C0(_, _, h3609, _) => {
                    h3609
                }
            }, match v2 {
                Adt46::C0(_, _, _, h3610) => {
                    h3610
                }
            })))
        }
        Adt76::C2(v3) => {
            Ok(Some(f898(Str::lit("audio"), match v3.clone() {
                Adt47::C0(h3611, _, _, _) => {
                    h3611
                }
            }, match v3.clone() {
                Adt47::C0(_, h3612, _, _) => {
                    h3612
                }
            }, match v3.clone() {
                Adt47::C0(_, _, h3613, _) => {
                    h3613
                }
            }, match v3 {
                Adt47::C0(_, _, _, h3614) => {
                    h3614
                }
            })))
        }
        Adt76::C3(v4) => {
            Ok(Some(f1019(v4)))
        }
        Adt76::C4(v5) => {
            f1020(v5)
        }
    }
}

pub fn f1012(p3618: List<Adt76>, p3619: List<Adt9>) -> R<Option<List<Adt9>>> {
    let m3617 = (p3618, p3619);
    {
        let mut m3617 = m3617;
        loop {
            let step = {
                let (v0, v1) = m3617;
                let m3615 = v0;
                match m3615.uncons() {
                    None => {
                        Err::<(List<Adt76>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3616 = f1011(v2)?;
                        match m3616 {
                            None => {
                                Err::<(List<Adt76>, List<Adt9>), R<Option<List<Adt9>>>>(Ok(None::<List<Adt9>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt76>, List<Adt9>), R<Option<List<Adt9>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1013(v0: Adt85) -> bool {
    let m3620 = v0;
    match m3620 {
        Adt85::C0(v1) => {
            let m3621 = v1;
            match m3621.uncons() {
                None => {
                    false
                }
                Some((_, _)) => {
                    true
                }
            }
        }
        Adt85::C1(_) => {
            true
        }
        Adt85::C2(_, _) => {
            true
        }
    }
}

pub fn f1014(v0: Bytes, v1: Str, v2: Str, v3: Adt85) -> R<Result<Adt85, Adt93>> {
    let m3622 = v3.clone();
    match m3622 {
        Adt85::C0(_) => {
            Ok(Ok::<Adt85, Adt93>(v3))
        }
        Adt85::C1(v5) => {
            Ok(f1021(f1022(v0, v1, v2, v5)?, List::<(Str, Adt68)>::nil()))
        }
        Adt85::C2(v6, v7) => {
            Ok(f1021(f1022(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f1015(v0: Option<Str>) -> Result<Option<Str>, Adt93> {
    let m3623 = v0;
    match m3623 {
        None => {
            Err::<Option<Str>, Adt93>(Adt93::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt93>(Some(v1))
        }
    }
}

pub fn f1016(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3627 = f1023(v0, {
        let a3624 = v1;
        utf8_encode(a3624)
    }, {
        let a3625 = v2;
        utf8_encode(a3625)
    }, {
        let a3626 = v3;
        utf8_encode(a3626)
    })?;
    match m3627 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3628 = v4;
            Ok(utf8_decode(a3628))
        }
    }
}

pub fn f1017(p3633: List<(Str, Adt9)>) -> bool {
    let m3632 = p3633;
    {
        let mut m3632 = m3632;
        loop {
            let step = {
                let v0 = m3632;
                let m3629 = v0;
                match m3629.uncons() {
                    None => {
                        Err::<List<(Str, Adt9)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3631 = f236({
                            let (_, h3630) = v1;
                            h3630
                        });
                        match m3631 {
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
                Ok(next) => { m3632 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1018(v0: Str, v1: Adt9) -> R<bool> {
    let m3634 = f613(v1);
    match m3634 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3635 = f500(v2)?;
            match m3635 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3637 = f1024(f29({
                        let a3636 = v0;
                        utf8_encode(a3636)
                    })?)?;
                    match m3637 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f684({
                                let a3638 = v3;
                                let a3639 = v4;
                                int_sub(a3638, a3639)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f1019(v0: Adt77) -> Adt9 {
    Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt9::C3(match v0.clone() {
        Adt77::C0(h3640, _, _, _) => {
            h3640
        }
    })), List::cons((Str::lit("name"), Adt9::C3(match v0.clone() {
        Adt77::C0(_, h3641, _, _) => {
            h3641
        }
    })), List::cons((Str::lit("input"), Adt9::C5({
        let m3644 = match v0.clone() {
            Adt77::C0(_, _, h3643, _) => {
                h3643
            }
        };
        match m3644 {
            Adt8::C0(h3642) => {
                h3642
            }
        }
    })), f896(match v0 {
        Adt77::C0(_, _, _, h3645) => {
            h3645
        }
    }))))))
}

pub fn f1020(v0: Adt78) -> R<Option<Adt9>> {
    let m3647 = f725(match v0.clone() {
        Adt78::C0(_, h3646, _, _, _) => {
            h3646
        }
    })?;
    match m3647 {
        None => {
            Ok(None::<Adt9>)
        }
        Some(v1) => {
            Ok(Some(Adt9::C5(List::cons((Str::lit("type"), Adt9::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt9::C3(match v0.clone() {
                Adt78::C0(h3648, _, _, _, _) => {
                    h3648
                }
            })), List::cons((Str::lit("content"), Adt9::C4(v1)), {
                let a3654 = f726(Str::lit("structuredContent"), match v0.clone() {
                    Adt78::C0(_, _, h3649, _, _) => {
                        h3649
                    }
                });
                let a3655 = {
                    let a3652 = f727(Str::lit("isError"), match v0.clone() {
                        Adt78::C0(_, _, _, h3650, _) => {
                            h3650
                        }
                    });
                    let a3653 = f896(match v0 {
                        Adt78::C0(_, _, _, _, h3651) => {
                            h3651
                        }
                    });
                    append_list(a3652, a3653)
                };
                append_list(a3654, a3655)
            }))))))
        }
    }
}

pub fn f1021(v0: Option<Str>, v1: List<(Str, Adt68)>) -> Result<Adt85, Adt93> {
    let m3656 = v0;
    match m3656 {
        None => {
            Err::<Adt85, Adt93>(Adt93::C14)
        }
        Some(v2) => {
            let m3657 = v1.clone();
            match m3657.uncons() {
                None => {
                    Ok::<Adt85, Adt93>(Adt85::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt85, Adt93>(Adt85::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f1022(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3661 = f1025(v0, {
        let a3658 = v1;
        utf8_encode(a3658)
    }, {
        let a3659 = v2;
        utf8_encode(a3659)
    }, {
        let a3660 = v3;
        utf8_encode(a3660)
    })?;
    match m3661 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3662 = v4;
            Ok(utf8_decode(a3662))
        }
    }
}

pub fn f1023(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3664 = f1026();
    let a3665 = {
        let a3663 = v0.clone();
        length_bytes(a3663)
    };
    if nat_le(a3664, a3665) {
        let a3667 = f1027(v3.clone())?;
        let a3668 = {
            let a3666 = v3.clone();
            length_bytes(a3666)
        };
        if nat_eq(a3667, a3668) {
            Ok(None::<Bytes>)
        } else {
            f1028(v0, v1, v2, v3.clone(), f1027(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1024(v0: List<u64>) -> R<Option<i64>> {
    let m3669 = v0.clone();
    match m3669.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3670 = v1;
            let a3671 = 45u64;
            if nat_eq(a3670, a3671) {
                let m3672 = v2.clone();
                match m3672.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3673 = f1029(v2, 0i64)?;
                        match m3673 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3674 = v5;
                                    int_neg(a3674)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f1029(v0, 0i64)
            }
        }
    }
}

pub fn f1025(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3676 = f1026();
    let a3677 = {
        let a3675 = v0.clone();
        length_bytes(a3675)
    };
    if nat_le(a3676, a3677) {
        Ok(Some({
            let a3680 = f1030(v3.clone())?;
            let a3681 = {
                let a3678 = Bytes::lit(&[0x2e]);
                let a3679 = f1030(f1031(v0, v1, v2, v3)?)?;
                append_bytes(a3678, a3679)
            };
            append_bytes(a3680, a3681)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1026() -> u64 {
    32u64
}

pub fn f1027(v0: Bytes) -> R<u64> {
    let (_, h3682) = f1032(Fn9::F1033(v0.clone()), {
        let a3683 = v0.clone();
        length_bytes(a3683)
    }, (0u64, {
        let a3684 = v0;
        length_bytes(a3684)
    }))?;
    Ok(h3682)
}

pub fn f1028(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3688 = {
        let a3685 = v3.clone();
        let a3686 = 0u64;
        let a3687 = v4;
        slice_bytes(a3685, a3686, a3687)
    };
    match m3688 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3699 = {
                let a3696 = v3.clone();
                let a3697 = {
                    let a3689 = v4;
                    let a3690 = 1u64;
                    nat_add(a3689, a3690)?
                };
                let a3698 = {
                    let a3694 = {
                        let a3691 = v3;
                        length_bytes(a3691)
                    };
                    let a3695 = {
                        let a3692 = v4;
                        let a3693 = 1u64;
                        nat_add(a3692, a3693)?
                    };
                    nat_sub(a3694, a3695)
                };
                slice_bytes(a3696, a3697, a3698)
            };
            match m3699 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f1034(v0, v1, v2, f1035(v5)?, f1035(v6)?)
                }
            }
        }
    }
}

pub fn f1029(p3705: List<u64>, p3706: i64) -> R<Option<i64>> {
    let m3704 = (p3705, p3706);
    {
        let mut m3704 = m3704;
        loop {
            let step = {
                let (v0, v1) = m3704;
                let m3700 = v0;
                match m3700.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3701 = f1036(v2);
                        match m3701 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f359({
                                    let a3702 = v1;
                                    let a3703 = 10i64;
                                    int_mul(a3702, a3703)?
                                }, f580(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1030(v0: Bytes) -> R<Bytes> {
    let (_, h3707) = f1037(Fn10::F1038(v0.clone()), {
        let a3708 = v0;
        length_bytes(a3708)
    }, (0u64, f257()))?;
    Ok(h3707)
}

pub fn f1031(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f1039(v0, {
        let a3709 = f1040(v1, v2);
        let a3710 = v3;
        append_bytes(a3709, a3710)
    })
}

pub fn f1032(v0: Fn9, p3714: u64, p3715: (u64, u64)) -> R<(u64, u64)> {
    let m3713 = (p3714, p3715);
    {
        let mut m3713 = m3713;
        loop {
            let step = {
                let (v1, v2) = m3713;
                let m3711 = v1;
                if m3711 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3711 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3712: Fn9 = v0.clone();
                        c3712.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1033(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3717 = {
            let (h3716, _) = v1;
            h3716
        };
        let a3718 = 1u64;
        nat_add(a3717, a3718)?
    }, {
        let a3721 = {
            let (_, h3719) = v1;
            h3719
        };
        let a3722 = {
            let a3720 = v0.clone();
            length_bytes(a3720)
        };
        if nat_eq(a3721, a3722) {
            let a3724 = f62(v0, {
                let (h3723, _) = v1;
                h3723
            })?;
            let a3725 = 46u64;
            if nat_eq(a3724, a3725) {
                let (h3726, _) = v1;
                h3726
            } else {
                let (_, h3727) = v1;
                h3727
            }
        } else {
            let (_, h3728) = v1;
            h3728
        }
    }))
}

pub fn f1034(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3729 = v3;
    match m3729 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3730 = v4;
            match m3730 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f1041(f1031(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f1035(v0: Bytes) -> R<Option<Bytes>> {
    let a3735 = {
        let a3732 = {
            let a3731 = v0.clone();
            length_bytes(a3731)
        };
        let a3733 = 2u64;
        let a3734 = 0u64;
        nat_rem(a3732, a3733, a3734)
    };
    let a3736 = 0u64;
    if nat_eq(a3735, a3736) {
        let v1: (u64, (Bytes, u64)) = f1042(Fn11::F1043(v0.clone()), {
            let a3738 = {
                let a3737 = v0;
                length_bytes(a3737)
            };
            let a3739 = 2u64;
            let a3740 = 0u64;
            nat_quot(a3738, a3739, a3740)
        }, (0u64, (f257(), 0u64)))?;
        let a3743 = {
            let (_, h3741) = {
                let (_, h3742) = v1.clone();
                h3742
            };
            h3741
        };
        let a3744 = 0u64;
        if nat_eq(a3743, a3744) {
            Ok(Some({
                let (h3745, _) = {
                    let (_, h3746) = v1;
                    h3746
                };
                h3745
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1036(v0: u64) -> Option<u64> {
    let a3747 = v0;
    let a3748 = 48u64;
    if if nat_lt(a3747, a3748) {
        true
    } else {
        let a3749 = 57u64;
        let a3750 = v0;
        nat_lt(a3749, a3750)
    } {
        None::<u64>
    } else {
        Some({
            let a3751 = v0;
            let a3752 = 48u64;
            nat_sub(a3751, a3752)
        })
    }
}

pub fn f1037(v0: Fn10, p3756: u64, p3757: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3755 = (p3756, p3757);
    {
        let mut m3755 = m3755;
        loop {
            let step = {
                let (v1, v2) = m3755;
                let m3753 = v1;
                if m3753 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3753 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3754: Fn10 = v0.clone();
                        c3754.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1038(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3759 = {
            let (h3758, _) = v1.clone();
            h3758
        };
        let a3760 = 1u64;
        nat_add(a3759, a3760)?
    }, {
        let a3763 = {
            let (_, h3761) = v1;
            h3761
        };
        let a3764 = f1044(f62(v0, {
            let (h3762, _) = v1;
            h3762
        })?)?;
        append_bytes(a3763, a3764)
    }))
}

pub fn f1039(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f1045(f1046(v0, v1)?)
}

pub fn f1040(v0: Bytes, v1: Bytes) -> Bytes {
    let a3771 = f1047({
        let a3765 = v0.clone();
        length_bytes(a3765)
    });
    let a3772 = {
        let a3769 = v0;
        let a3770 = {
            let a3767 = f1047({
                let a3766 = v1.clone();
                length_bytes(a3766)
            });
            let a3768 = v1;
            append_bytes(a3767, a3768)
        };
        append_bytes(a3769, a3770)
    };
    append_bytes(a3771, a3772)
}

pub fn f1041(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3775 = {
        let a3773 = v0.clone();
        length_bytes(a3773)
    };
    let a3776 = {
        let a3774 = v1.clone();
        length_bytes(a3774)
    };
    if nat_eq(a3775, a3776) {
        let a3777 = f1048(v0, v1)?;
        let a3778 = 0u64;
        Ok(nat_eq(a3777, a3778))
    } else {
        Ok(false)
    }
}

pub fn f1042(v0: Fn11, p3782: u64, p3783: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3781 = (p3782, p3783);
    {
        let mut m3781 = m3781;
        loop {
            let step = {
                let (v1, v2) = m3781;
                let m3779 = v1;
                if m3779 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3779 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3780: Fn11 = v0.clone();
                        c3780.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3781 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1043(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f1049(v0, v1)
}

pub fn f1044(v0: u64) -> R<Bytes> {
    let a3790 = f1050({
        let a3784 = v0;
        let a3785 = 16u64;
        let a3786 = 0u64;
        nat_quot(a3784, a3785, a3786)
    })?;
    let a3791 = f1050({
        let a3787 = v0;
        let a3788 = 16u64;
        let a3789 = 0u64;
        nat_rem(a3787, a3788, a3789)
    })?;
    Ok(append_bytes(a3790, a3791))
}

pub fn f1045(v0: Adt169) -> R<Bytes> {
    let a3812 = f1051(match v0.clone() {
        Adt169::C0(h3792, _, _, _, _, _, _, _) => {
            h3792
        }
    })?;
    let a3813 = {
        let a3810 = f1051(match v0.clone() {
            Adt169::C0(_, h3793, _, _, _, _, _, _) => {
                h3793
            }
        })?;
        let a3811 = {
            let a3808 = f1051(match v0.clone() {
                Adt169::C0(_, _, h3794, _, _, _, _, _) => {
                    h3794
                }
            })?;
            let a3809 = {
                let a3806 = f1051(match v0.clone() {
                    Adt169::C0(_, _, _, h3795, _, _, _, _) => {
                        h3795
                    }
                })?;
                let a3807 = {
                    let a3804 = f1051(match v0.clone() {
                        Adt169::C0(_, _, _, _, h3796, _, _, _) => {
                            h3796
                        }
                    })?;
                    let a3805 = {
                        let a3802 = f1051(match v0.clone() {
                            Adt169::C0(_, _, _, _, _, h3797, _, _) => {
                                h3797
                            }
                        })?;
                        let a3803 = {
                            let a3800 = f1051(match v0.clone() {
                                Adt169::C0(_, _, _, _, _, _, h3798, _) => {
                                    h3798
                                }
                            })?;
                            let a3801 = f1051(match v0 {
                                Adt169::C0(_, _, _, _, _, _, _, h3799) => {
                                    h3799
                                }
                            })?;
                            append_bytes(a3800, a3801)
                        };
                        append_bytes(a3802, a3803)
                    };
                    append_bytes(a3804, a3805)
                };
                append_bytes(a3806, a3807)
            };
            append_bytes(a3808, a3809)
        };
        append_bytes(a3810, a3811)
    };
    Ok(append_bytes(a3812, a3813))
}

pub fn f1046(v0: Bytes, v1: Bytes) -> R<Adt169> {
    let v2: Adt170 = f1052(v0)?;
    let v3: Adt169 = f1053(f1054(f1055(), f1056(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f1054(f1054(f1055(), f1056(v2, 1549556828u32))?, f1057(v3))
}

pub fn f1047(v0: u64) -> Bytes {
    let a3839 = f469({
        let a3817 = {
            let a3814 = v0;
            let a3815 = 16777216u64;
            let a3816 = 0u64;
            nat_quot(a3814, a3815, a3816)
        };
        let a3818 = 256u64;
        let a3819 = 0u64;
        nat_rem(a3817, a3818, a3819)
    });
    let a3840 = {
        let a3837 = f469({
            let a3823 = {
                let a3820 = v0;
                let a3821 = 65536u64;
                let a3822 = 0u64;
                nat_quot(a3820, a3821, a3822)
            };
            let a3824 = 256u64;
            let a3825 = 0u64;
            nat_rem(a3823, a3824, a3825)
        });
        let a3838 = {
            let a3835 = f469({
                let a3829 = {
                    let a3826 = v0;
                    let a3827 = 256u64;
                    let a3828 = 0u64;
                    nat_quot(a3826, a3827, a3828)
                };
                let a3830 = 256u64;
                let a3831 = 0u64;
                nat_rem(a3829, a3830, a3831)
            });
            let a3836 = f469({
                let a3832 = v0;
                let a3833 = 256u64;
                let a3834 = 0u64;
                nat_rem(a3832, a3833, a3834)
            });
            append_bytes(a3835, a3836)
        };
        append_bytes(a3837, a3838)
    };
    append_bytes(a3839, a3840)
}

pub fn f1048(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3841) = f1032(Fn9::F1058(v0.clone(), v1), {
        let a3842 = v0;
        length_bytes(a3842)
    }, (0u64, 0u64))?;
    Ok(h3841)
}

pub fn f1049(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f1059(f62(v0.clone(), {
        let (h3843, _) = v1.clone();
        h3843
    })?);
    let v3: u64 = f1059(f62(v0, {
        let a3845 = {
            let (h3844, _) = v1.clone();
            h3844
        };
        let a3846 = 1u64;
        nat_add(a3845, a3846)?
    })?);
    let a3847 = v2;
    let a3848 = 16u64;
    if if nat_lt(a3847, a3848) {
        let a3849 = v3;
        let a3850 = 16u64;
        nat_lt(a3849, a3850)
    } else {
        false
    } {
        Ok(({
            let a3852 = {
                let (h3851, _) = v1.clone();
                h3851
            };
            let a3853 = 2u64;
            nat_add(a3852, a3853)?
        }, ({
            let a3860 = {
                let (h3854, _) = {
                    let (_, h3855) = v1.clone();
                    h3855
                };
                h3854
            };
            let a3861 = f469({
                let a3858 = {
                    let a3856 = v2;
                    let a3857 = 16u64;
                    nat_mul(a3856, a3857)?
                };
                let a3859 = v3;
                nat_add(a3858, a3859)?
            });
            append_bytes(a3860, a3861)
        }, {
            let (_, h3862) = {
                let (_, h3863) = v1;
                h3863
            };
            h3862
        })))
    } else {
        Ok(({
            let a3865 = {
                let (h3864, _) = v1.clone();
                h3864
            };
            let a3866 = 2u64;
            nat_add(a3865, a3866)?
        }, ({
            let (h3867, _) = {
                let (_, h3868) = v1.clone();
                h3868
            };
            h3867
        }, {
            let a3871 = {
                let (_, h3869) = {
                    let (_, h3870) = v1;
                    h3870
                };
                h3869
            };
            let a3872 = 1u64;
            nat_add(a3871, a3872)?
        })))
    }
}

pub fn f1050(v0: u64) -> R<Bytes> {
    Ok(f469({
        let a3873 = v0;
        let a3874 = 10u64;
        if nat_lt(a3873, a3874) {
            let a3875 = 48u64;
            let a3876 = v0;
            nat_add(a3875, a3876)?
        } else {
            let a3877 = 87u64;
            let a3878 = v0;
            nat_add(a3877, a3878)?
        }
    }))
}

pub fn f1051(v0: u32) -> R<Bytes> {
    let a3883 = f1060(v0, 24u32)?;
    let a3884 = {
        let a3881 = f1060(v0, 16u32)?;
        let a3882 = {
            let a3879 = f1060(v0, 8u32)?;
            let a3880 = f1060(v0, 0u32)?;
            append_bytes(a3879, a3880)
        };
        append_bytes(a3881, a3882)
    };
    Ok(append_bytes(a3883, a3884))
}

pub fn f1052(v0: Bytes) -> R<Adt170> {
    let a3886 = f1061();
    let a3887 = {
        let a3885 = v0.clone();
        length_bytes(a3885)
    };
    if nat_lt(a3886, a3887) {
        Ok(f1062(f1063(v0)?))
    } else {
        f1064(v0)
    }
}

pub fn f1053(v0: Adt169, v1: u64, v2: Bytes) -> R<Adt169> {
    let v3: u64 = {
        let a3888 = v2.clone();
        length_bytes(a3888)
    };
    let v4: u64 = {
        let a3894 = {
            let a3891 = {
                let a3889 = v3;
                let a3890 = 72u64;
                nat_add(a3889, a3890)?
            };
            let a3892 = 64u64;
            let a3893 = 0u64;
            nat_quot(a3891, a3892, a3893)
        };
        let a3895 = 64u64;
        nat_mul(a3894, a3895)?
    };
    let (_, h3896) = f1065(Fn12::F1066(v2, v1, v3, v4), {
        let a3897 = v4;
        let a3898 = 64u64;
        let a3899 = 0u64;
        nat_quot(a3897, a3898, a3899)
    }, (0u64, v0))?;
    Ok(h3896)
}

pub fn f1054(v0: Adt169, v1: Adt170) -> R<Adt169> {
    let v2: Adt171 = f1067(Fn13::F1068, 64u64, Adt171::C0(match v0.clone() {
        Adt169::C0(h3900, _, _, _, _, _, _, _) => {
            h3900
        }
    }, match v0.clone() {
        Adt169::C0(_, h3901, _, _, _, _, _, _) => {
            h3901
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h3902, _, _, _, _, _) => {
            h3902
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h3903, _, _, _, _) => {
            h3903
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h3904, _, _, _) => {
            h3904
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h3905, _, _) => {
            h3905
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h3906, _) => {
            h3906
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, _, h3907) => {
            h3907
        }
    }, v1, 0u64))?;
    Ok(Adt169::C0(f1069(match v0.clone() {
        Adt169::C0(h3908, _, _, _, _, _, _, _) => {
            h3908
        }
    }, match v2.clone() {
        Adt171::C0(h3909, _, _, _, _, _, _, _, _, _) => {
            h3909
        }
    }), f1069(match v0.clone() {
        Adt169::C0(_, h3910, _, _, _, _, _, _) => {
            h3910
        }
    }, match v2.clone() {
        Adt171::C0(_, h3911, _, _, _, _, _, _, _, _) => {
            h3911
        }
    }), f1069(match v0.clone() {
        Adt169::C0(_, _, h3912, _, _, _, _, _) => {
            h3912
        }
    }, match v2.clone() {
        Adt171::C0(_, _, h3913, _, _, _, _, _, _, _) => {
            h3913
        }
    }), f1069(match v0.clone() {
        Adt169::C0(_, _, _, h3914, _, _, _, _) => {
            h3914
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, h3915, _, _, _, _, _, _) => {
            h3915
        }
    }), f1069(match v0.clone() {
        Adt169::C0(_, _, _, _, h3916, _, _, _) => {
            h3916
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, h3917, _, _, _, _, _) => {
            h3917
        }
    }), f1069(match v0.clone() {
        Adt169::C0(_, _, _, _, _, h3918, _, _) => {
            h3918
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, _, h3919, _, _, _, _) => {
            h3919
        }
    }), f1069(match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h3920, _) => {
            h3920
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, _, _, h3921, _, _, _) => {
            h3921
        }
    }), f1069(match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h3922) => {
            h3922
        }
    }, match v2 {
        Adt171::C0(_, _, _, _, _, _, _, h3923, _, _) => {
            h3923
        }
    })))
}

pub fn f1055() -> Adt169 {
    Adt169::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f1056(v0: Adt170, v1: u32) -> Adt170 {
    Adt170::C0({
        let a3925 = match v0.clone() {
            Adt170::C0(h3924, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3924
            }
        };
        let a3926 = v1;
        fixed_u32::bit_xor(a3925, a3926)
    }, {
        let a3928 = match v0.clone() {
            Adt170::C0(_, h3927, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3927
            }
        };
        let a3929 = v1;
        fixed_u32::bit_xor(a3928, a3929)
    }, {
        let a3931 = match v0.clone() {
            Adt170::C0(_, _, h3930, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3930
            }
        };
        let a3932 = v1;
        fixed_u32::bit_xor(a3931, a3932)
    }, {
        let a3934 = match v0.clone() {
            Adt170::C0(_, _, _, h3933, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3933
            }
        };
        let a3935 = v1;
        fixed_u32::bit_xor(a3934, a3935)
    }, {
        let a3937 = match v0.clone() {
            Adt170::C0(_, _, _, _, h3936, _, _, _, _, _, _, _, _, _, _, _) => {
                h3936
            }
        };
        let a3938 = v1;
        fixed_u32::bit_xor(a3937, a3938)
    }, {
        let a3940 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, h3939, _, _, _, _, _, _, _, _, _, _) => {
                h3939
            }
        };
        let a3941 = v1;
        fixed_u32::bit_xor(a3940, a3941)
    }, {
        let a3943 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, h3942, _, _, _, _, _, _, _, _, _) => {
                h3942
            }
        };
        let a3944 = v1;
        fixed_u32::bit_xor(a3943, a3944)
    }, {
        let a3946 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, h3945, _, _, _, _, _, _, _, _) => {
                h3945
            }
        };
        let a3947 = v1;
        fixed_u32::bit_xor(a3946, a3947)
    }, {
        let a3949 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, h3948, _, _, _, _, _, _, _) => {
                h3948
            }
        };
        let a3950 = v1;
        fixed_u32::bit_xor(a3949, a3950)
    }, {
        let a3952 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, h3951, _, _, _, _, _, _) => {
                h3951
            }
        };
        let a3953 = v1;
        fixed_u32::bit_xor(a3952, a3953)
    }, {
        let a3955 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, h3954, _, _, _, _, _) => {
                h3954
            }
        };
        let a3956 = v1;
        fixed_u32::bit_xor(a3955, a3956)
    }, {
        let a3958 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, h3957, _, _, _, _) => {
                h3957
            }
        };
        let a3959 = v1;
        fixed_u32::bit_xor(a3958, a3959)
    }, {
        let a3961 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3960, _, _, _) => {
                h3960
            }
        };
        let a3962 = v1;
        fixed_u32::bit_xor(a3961, a3962)
    }, {
        let a3964 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3963, _, _) => {
                h3963
            }
        };
        let a3965 = v1;
        fixed_u32::bit_xor(a3964, a3965)
    }, {
        let a3967 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3966, _) => {
                h3966
            }
        };
        let a3968 = v1;
        fixed_u32::bit_xor(a3967, a3968)
    }, {
        let a3970 = match v0 {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3969) => {
                h3969
            }
        };
        let a3971 = v1;
        fixed_u32::bit_xor(a3970, a3971)
    })
}

pub fn f1057(v0: Adt169) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt169::C0(h3972, _, _, _, _, _, _, _) => {
            h3972
        }
    }, match v0.clone() {
        Adt169::C0(_, h3973, _, _, _, _, _, _) => {
            h3973
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h3974, _, _, _, _, _) => {
            h3974
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h3975, _, _, _, _) => {
            h3975
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h3976, _, _, _) => {
            h3976
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h3977, _, _) => {
            h3977
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h3978, _) => {
            h3978
        }
    }, match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h3979) => {
            h3979
        }
    }, 2147483648u32, f1070(), f1070(), f1070(), f1070(), f1070(), f1070(), 768u32)
}

pub fn f1058(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3981 = {
            let (h3980, _) = v2;
            h3980
        };
        let a3982 = 1u64;
        nat_add(a3981, a3982)?
    }, {
        let a4034 = {
            let (_, h3983) = v2;
            h3983
        };
        let a4035 = {
            let v3: u8 = {
                let a3986 = f1071(v0, {
                    let (h3984, _) = v2;
                    h3984
                });
                let a3987 = f1071(v1, {
                    let (h3985, _) = v2;
                    h3985
                });
                fixed_u8::bit_xor(a3986, a3987)
            };
            let a4032 = {
                let a4008 = {
                    let a3996 = {
                        let a3990 = {
                            let a3988 = v3;
                            let a3989 = 128u8;
                            fixed_u8::bit_and(a3988, a3989)
                        };
                        let a3991 = 128u8;
                        if equal(a3990, a3991) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3997 = {
                        let a3994 = {
                            let a3992 = v3;
                            let a3993 = 64u8;
                            fixed_u8::bit_and(a3992, a3993)
                        };
                        let a3995 = 64u8;
                        if equal(a3994, a3995) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3996, a3997)?
                };
                let a4009 = {
                    let a4006 = {
                        let a4000 = {
                            let a3998 = v3;
                            let a3999 = 32u8;
                            fixed_u8::bit_and(a3998, a3999)
                        };
                        let a4001 = 32u8;
                        if equal(a4000, a4001) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a4007 = {
                        let a4004 = {
                            let a4002 = v3;
                            let a4003 = 16u8;
                            fixed_u8::bit_and(a4002, a4003)
                        };
                        let a4005 = 16u8;
                        if equal(a4004, a4005) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4006, a4007)?
                };
                nat_add(a4008, a4009)?
            };
            let a4033 = {
                let a4030 = {
                    let a4018 = {
                        let a4012 = {
                            let a4010 = v3;
                            let a4011 = 8u8;
                            fixed_u8::bit_and(a4010, a4011)
                        };
                        let a4013 = 8u8;
                        if equal(a4012, a4013) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a4019 = {
                        let a4016 = {
                            let a4014 = v3;
                            let a4015 = 4u8;
                            fixed_u8::bit_and(a4014, a4015)
                        };
                        let a4017 = 4u8;
                        if equal(a4016, a4017) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4018, a4019)?
                };
                let a4031 = {
                    let a4028 = {
                        let a4022 = {
                            let a4020 = v3;
                            let a4021 = 2u8;
                            fixed_u8::bit_and(a4020, a4021)
                        };
                        let a4023 = 2u8;
                        if equal(a4022, a4023) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a4029 = {
                        let a4026 = {
                            let a4024 = v3;
                            let a4025 = 1u8;
                            fixed_u8::bit_and(a4024, a4025)
                        };
                        let a4027 = 1u8;
                        if equal(a4026, a4027) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4028, a4029)?
                };
                nat_add(a4030, a4031)?
            };
            nat_add(a4032, a4033)?
        };
        nat_add(a4034, a4035)?
    }))
}

pub fn f1059(v0: u64) -> u64 {
    let a4036 = v0;
    let a4037 = 48u64;
    if nat_lt(a4036, a4037) {
        16u64
    } else {
        let a4038 = v0;
        let a4039 = 58u64;
        if nat_lt(a4038, a4039) {
            let a4040 = v0;
            let a4041 = 48u64;
            nat_sub(a4040, a4041)
        } else {
            let a4042 = v0;
            let a4043 = 97u64;
            if nat_lt(a4042, a4043) {
                16u64
            } else {
                let a4044 = v0;
                let a4045 = 103u64;
                if nat_lt(a4044, a4045) {
                    let a4046 = v0;
                    let a4047 = 87u64;
                    nat_sub(a4046, a4047)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f1060(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f469({
        let v2: u32 = {
            let a4048 = f1072(v0, v1);
            let a4049 = 255u32;
            fixed_u32::bit_and(a4048, a4049)
        };
        let a4238 = {
            let a4142 = {
                let a4094 = {
                    let a4070 = {
                        let a4058 = {
                            let a4052 = {
                                let a4050 = v2;
                                let a4051 = 2147483648u32;
                                fixed_u32::bit_and(a4050, a4051)
                            };
                            let a4053 = 2147483648u32;
                            if equal(a4052, a4053) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a4059 = {
                            let a4056 = {
                                let a4054 = v2;
                                let a4055 = 1073741824u32;
                                fixed_u32::bit_and(a4054, a4055)
                            };
                            let a4057 = 1073741824u32;
                            if equal(a4056, a4057) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4058, a4059)?
                    };
                    let a4071 = {
                        let a4068 = {
                            let a4062 = {
                                let a4060 = v2;
                                let a4061 = 536870912u32;
                                fixed_u32::bit_and(a4060, a4061)
                            };
                            let a4063 = 536870912u32;
                            if equal(a4062, a4063) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a4069 = {
                            let a4066 = {
                                let a4064 = v2;
                                let a4065 = 268435456u32;
                                fixed_u32::bit_and(a4064, a4065)
                            };
                            let a4067 = 268435456u32;
                            if equal(a4066, a4067) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4068, a4069)?
                    };
                    nat_add(a4070, a4071)?
                };
                let a4095 = {
                    let a4092 = {
                        let a4080 = {
                            let a4074 = {
                                let a4072 = v2;
                                let a4073 = 134217728u32;
                                fixed_u32::bit_and(a4072, a4073)
                            };
                            let a4075 = 134217728u32;
                            if equal(a4074, a4075) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a4081 = {
                            let a4078 = {
                                let a4076 = v2;
                                let a4077 = 67108864u32;
                                fixed_u32::bit_and(a4076, a4077)
                            };
                            let a4079 = 67108864u32;
                            if equal(a4078, a4079) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4080, a4081)?
                    };
                    let a4093 = {
                        let a4090 = {
                            let a4084 = {
                                let a4082 = v2;
                                let a4083 = 33554432u32;
                                fixed_u32::bit_and(a4082, a4083)
                            };
                            let a4085 = 33554432u32;
                            if equal(a4084, a4085) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a4091 = {
                            let a4088 = {
                                let a4086 = v2;
                                let a4087 = 16777216u32;
                                fixed_u32::bit_and(a4086, a4087)
                            };
                            let a4089 = 16777216u32;
                            if equal(a4088, a4089) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4090, a4091)?
                    };
                    nat_add(a4092, a4093)?
                };
                nat_add(a4094, a4095)?
            };
            let a4143 = {
                let a4140 = {
                    let a4116 = {
                        let a4104 = {
                            let a4098 = {
                                let a4096 = v2;
                                let a4097 = 8388608u32;
                                fixed_u32::bit_and(a4096, a4097)
                            };
                            let a4099 = 8388608u32;
                            if equal(a4098, a4099) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a4105 = {
                            let a4102 = {
                                let a4100 = v2;
                                let a4101 = 4194304u32;
                                fixed_u32::bit_and(a4100, a4101)
                            };
                            let a4103 = 4194304u32;
                            if equal(a4102, a4103) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4104, a4105)?
                    };
                    let a4117 = {
                        let a4114 = {
                            let a4108 = {
                                let a4106 = v2;
                                let a4107 = 2097152u32;
                                fixed_u32::bit_and(a4106, a4107)
                            };
                            let a4109 = 2097152u32;
                            if equal(a4108, a4109) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a4115 = {
                            let a4112 = {
                                let a4110 = v2;
                                let a4111 = 1048576u32;
                                fixed_u32::bit_and(a4110, a4111)
                            };
                            let a4113 = 1048576u32;
                            if equal(a4112, a4113) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4114, a4115)?
                    };
                    nat_add(a4116, a4117)?
                };
                let a4141 = {
                    let a4138 = {
                        let a4126 = {
                            let a4120 = {
                                let a4118 = v2;
                                let a4119 = 524288u32;
                                fixed_u32::bit_and(a4118, a4119)
                            };
                            let a4121 = 524288u32;
                            if equal(a4120, a4121) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a4127 = {
                            let a4124 = {
                                let a4122 = v2;
                                let a4123 = 262144u32;
                                fixed_u32::bit_and(a4122, a4123)
                            };
                            let a4125 = 262144u32;
                            if equal(a4124, a4125) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4126, a4127)?
                    };
                    let a4139 = {
                        let a4136 = {
                            let a4130 = {
                                let a4128 = v2;
                                let a4129 = 131072u32;
                                fixed_u32::bit_and(a4128, a4129)
                            };
                            let a4131 = 131072u32;
                            if equal(a4130, a4131) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a4137 = {
                            let a4134 = {
                                let a4132 = v2;
                                let a4133 = 65536u32;
                                fixed_u32::bit_and(a4132, a4133)
                            };
                            let a4135 = 65536u32;
                            if equal(a4134, a4135) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4136, a4137)?
                    };
                    nat_add(a4138, a4139)?
                };
                nat_add(a4140, a4141)?
            };
            nat_add(a4142, a4143)?
        };
        let a4239 = {
            let a4236 = {
                let a4188 = {
                    let a4164 = {
                        let a4152 = {
                            let a4146 = {
                                let a4144 = v2;
                                let a4145 = 32768u32;
                                fixed_u32::bit_and(a4144, a4145)
                            };
                            let a4147 = 32768u32;
                            if equal(a4146, a4147) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a4153 = {
                            let a4150 = {
                                let a4148 = v2;
                                let a4149 = 16384u32;
                                fixed_u32::bit_and(a4148, a4149)
                            };
                            let a4151 = 16384u32;
                            if equal(a4150, a4151) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4152, a4153)?
                    };
                    let a4165 = {
                        let a4162 = {
                            let a4156 = {
                                let a4154 = v2;
                                let a4155 = 8192u32;
                                fixed_u32::bit_and(a4154, a4155)
                            };
                            let a4157 = 8192u32;
                            if equal(a4156, a4157) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a4163 = {
                            let a4160 = {
                                let a4158 = v2;
                                let a4159 = 4096u32;
                                fixed_u32::bit_and(a4158, a4159)
                            };
                            let a4161 = 4096u32;
                            if equal(a4160, a4161) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4162, a4163)?
                    };
                    nat_add(a4164, a4165)?
                };
                let a4189 = {
                    let a4186 = {
                        let a4174 = {
                            let a4168 = {
                                let a4166 = v2;
                                let a4167 = 2048u32;
                                fixed_u32::bit_and(a4166, a4167)
                            };
                            let a4169 = 2048u32;
                            if equal(a4168, a4169) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a4175 = {
                            let a4172 = {
                                let a4170 = v2;
                                let a4171 = 1024u32;
                                fixed_u32::bit_and(a4170, a4171)
                            };
                            let a4173 = 1024u32;
                            if equal(a4172, a4173) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4174, a4175)?
                    };
                    let a4187 = {
                        let a4184 = {
                            let a4178 = {
                                let a4176 = v2;
                                let a4177 = 512u32;
                                fixed_u32::bit_and(a4176, a4177)
                            };
                            let a4179 = 512u32;
                            if equal(a4178, a4179) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a4185 = {
                            let a4182 = {
                                let a4180 = v2;
                                let a4181 = 256u32;
                                fixed_u32::bit_and(a4180, a4181)
                            };
                            let a4183 = 256u32;
                            if equal(a4182, a4183) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4184, a4185)?
                    };
                    nat_add(a4186, a4187)?
                };
                nat_add(a4188, a4189)?
            };
            let a4237 = {
                let a4234 = {
                    let a4210 = {
                        let a4198 = {
                            let a4192 = {
                                let a4190 = v2;
                                let a4191 = 128u32;
                                fixed_u32::bit_and(a4190, a4191)
                            };
                            let a4193 = 128u32;
                            if equal(a4192, a4193) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4199 = {
                            let a4196 = {
                                let a4194 = v2;
                                let a4195 = 64u32;
                                fixed_u32::bit_and(a4194, a4195)
                            };
                            let a4197 = 64u32;
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
                                let a4200 = v2;
                                let a4201 = 32u32;
                                fixed_u32::bit_and(a4200, a4201)
                            };
                            let a4203 = 32u32;
                            if equal(a4202, a4203) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4209 = {
                            let a4206 = {
                                let a4204 = v2;
                                let a4205 = 16u32;
                                fixed_u32::bit_and(a4204, a4205)
                            };
                            let a4207 = 16u32;
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
                                let a4212 = v2;
                                let a4213 = 8u32;
                                fixed_u32::bit_and(a4212, a4213)
                            };
                            let a4215 = 8u32;
                            if equal(a4214, a4215) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4221 = {
                            let a4218 = {
                                let a4216 = v2;
                                let a4217 = 4u32;
                                fixed_u32::bit_and(a4216, a4217)
                            };
                            let a4219 = 4u32;
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
                                let a4222 = v2;
                                let a4223 = 2u32;
                                fixed_u32::bit_and(a4222, a4223)
                            };
                            let a4225 = 2u32;
                            if equal(a4224, a4225) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4231 = {
                            let a4228 = {
                                let a4226 = v2;
                                let a4227 = 1u32;
                                fixed_u32::bit_and(a4226, a4227)
                            };
                            let a4229 = 1u32;
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
        };
        nat_add(a4238, a4239)?
    }))
}

pub fn f1061() -> u64 {
    64u64
}

pub fn f1062(v0: Adt169) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt169::C0(h4240, _, _, _, _, _, _, _) => {
            h4240
        }
    }, match v0.clone() {
        Adt169::C0(_, h4241, _, _, _, _, _, _) => {
            h4241
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h4242, _, _, _, _, _) => {
            h4242
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h4243, _, _, _, _) => {
            h4243
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h4244, _, _, _) => {
            h4244
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h4245, _, _) => {
            h4245
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h4246, _) => {
            h4246
        }
    }, match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h4247) => {
            h4247
        }
    }, f1070(), f1070(), f1070(), f1070(), f1070(), f1070(), f1070(), f1070())
}

pub fn f1063(v0: Bytes) -> R<Adt169> {
    let v1: u64 = {
        let a4248 = v0.clone();
        length_bytes(a4248)
    };
    let v2: u64 = {
        let a4254 = {
            let a4251 = {
                let a4249 = v1;
                let a4250 = 72u64;
                nat_add(a4249, a4250)?
            };
            let a4252 = 64u64;
            let a4253 = 0u64;
            nat_quot(a4251, a4252, a4253)
        };
        let a4255 = 64u64;
        nat_mul(a4254, a4255)?
    };
    let (_, h4256) = f1065(Fn12::F1073(v0, v1, v2), {
        let a4257 = v2;
        let a4258 = 64u64;
        let a4259 = 0u64;
        nat_quot(a4257, a4258, a4259)
    }, (0u64, f1055()))?;
    Ok(h4256)
}

pub fn f1064(v0: Bytes) -> R<Adt170> {
    Ok(Adt170::C0(f1074(v0.clone(), 0u64)?, f1074(v0.clone(), 4u64)?, f1074(v0.clone(), 8u64)?, f1074(v0.clone(), 12u64)?, f1074(v0.clone(), 16u64)?, f1074(v0.clone(), 20u64)?, f1074(v0.clone(), 24u64)?, f1074(v0.clone(), 28u64)?, f1074(v0.clone(), 32u64)?, f1074(v0.clone(), 36u64)?, f1074(v0.clone(), 40u64)?, f1074(v0.clone(), 44u64)?, f1074(v0.clone(), 48u64)?, f1074(v0.clone(), 52u64)?, f1074(v0.clone(), 56u64)?, f1074(v0, 60u64)?))
}

pub fn f1065(v0: Fn12, p4263: u64, p4264: (u64, Adt169)) -> R<(u64, Adt169)> {
    let m4262 = (p4263, p4264);
    {
        let mut m4262 = m4262;
        loop {
            let step = {
                let (v1, v2) = m4262;
                let m4260 = v1;
                if m4260 == 0 {
                    Err::<(u64, (u64, Adt169)), R<(u64, Adt169)>>(Ok(v2))
                } else {
                    let v3 = m4260 - 1;
                    Ok::<(u64, (u64, Adt169)), R<(u64, Adt169)>>((v3, {
                        let c4261: Fn12 = v0.clone();
                        c4261.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1066(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt169)) -> R<(u64, Adt169)> {
    Ok(({
        let a4266 = {
            let (h4265, _) = v4.clone();
            h4265
        };
        let a4267 = 64u64;
        nat_add(a4266, a4267)?
    }, f1054({
        let (_, h4268) = v4;
        h4268
    }, f1075(v0, v2, v3, {
        let a4271 = {
            let a4269 = v1;
            let a4270 = v2;
            nat_add(a4269, a4270)?
        };
        let a4272 = 8u64;
        nat_mul(a4271, a4272)?
    }, {
        let (h4273, _) = v4;
        h4273
    })?)?))
}

pub fn f1067(v0: Fn13, p4277: u64, p4278: Adt171) -> R<Adt171> {
    let m4276 = (p4277, p4278);
    {
        let mut m4276 = m4276;
        loop {
            let step = {
                let (v1, v2) = m4276;
                let m4274 = v1;
                if m4274 == 0 {
                    Err::<(u64, Adt171), R<Adt171>>(Ok(v2))
                } else {
                    let v3 = m4274 - 1;
                    Ok::<(u64, Adt171), R<Adt171>>((v3, {
                        let c4275: Fn13 = v0.clone();
                        c4275.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1068(v0: Adt171) -> R<Adt171> {
    f1076(v0)
}

pub fn f1069(v0: u32, v1: u32) -> u32 {
    let m4281 = {
        let a4279 = f1077(v0);
        let a4280 = f1077(v1);
        fixed_u64::checked_add(a4279, a4280)
    };
    match m4281 {
        None => {
            0u32
        }
        Some(v2) => {
            f1078(v2)
        }
    }
}

pub fn f1070() -> u32 {
    0u32
}

pub fn f1071(v0: Bytes, v1: u64) -> u8 {
    let m4284 = {
        let a4282 = v0;
        let a4283 = v1;
        index_bytes(a4282, a4283)
    };
    match m4284 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1072(v0: u32, v1: u32) -> u32 {
    let m4287 = {
        let a4285 = v0;
        let a4286 = v1;
        fixed_u32::shift_right(a4285, a4286)
    };
    match m4287 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1073(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt169)) -> R<(u64, Adt169)> {
    Ok(({
        let a4289 = {
            let (h4288, _) = v3.clone();
            h4288
        };
        let a4290 = 64u64;
        nat_add(a4289, a4290)?
    }, f1054({
        let (_, h4291) = v3;
        h4291
    }, f1079(v0, v1, v2, {
        let (h4292, _) = v3;
        h4292
    })?)?))
}

pub fn f1074(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f1080(f1081(v0.clone(), v1), f1081(v0.clone(), {
        let a4293 = v1;
        let a4294 = 1u64;
        nat_add(a4293, a4294)?
    }), f1081(v0.clone(), {
        let a4295 = v1;
        let a4296 = 2u64;
        nat_add(a4295, a4296)?
    }), f1081(v0, {
        let a4297 = v1;
        let a4298 = 3u64;
        nat_add(a4297, a4298)?
    })))
}

pub fn f1075(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt170> {
    Ok(Adt170::C0(f1082(v0.clone(), v1, v2, v3, v4)?, f1082(v0.clone(), v1, v2, v3, {
        let a4299 = v4;
        let a4300 = 4u64;
        nat_add(a4299, a4300)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4301 = v4;
        let a4302 = 8u64;
        nat_add(a4301, a4302)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4303 = v4;
        let a4304 = 12u64;
        nat_add(a4303, a4304)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4305 = v4;
        let a4306 = 16u64;
        nat_add(a4305, a4306)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4307 = v4;
        let a4308 = 20u64;
        nat_add(a4307, a4308)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4309 = v4;
        let a4310 = 24u64;
        nat_add(a4309, a4310)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4311 = v4;
        let a4312 = 28u64;
        nat_add(a4311, a4312)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4313 = v4;
        let a4314 = 32u64;
        nat_add(a4313, a4314)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4315 = v4;
        let a4316 = 36u64;
        nat_add(a4315, a4316)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4317 = v4;
        let a4318 = 40u64;
        nat_add(a4317, a4318)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4319 = v4;
        let a4320 = 44u64;
        nat_add(a4319, a4320)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4321 = v4;
        let a4322 = 48u64;
        nat_add(a4321, a4322)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4323 = v4;
        let a4324 = 52u64;
        nat_add(a4323, a4324)?
    })?, f1082(v0.clone(), v1, v2, v3, {
        let a4325 = v4;
        let a4326 = 56u64;
        nat_add(a4325, a4326)?
    })?, f1082(v0, v1, v2, v3, {
        let a4327 = v4;
        let a4328 = 60u64;
        nat_add(a4327, a4328)?
    })?))
}

pub fn f1076(v0: Adt171) -> R<Adt171> {
    let v1: u32 = f1083(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, _, _, h4329) => {
            h4329
        }
    });
    let v2: u32 = f1069(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, h4330, _, _) => {
            h4330
        }
    }, f1069(f1084(match v0.clone() {
        Adt171::C0(_, _, _, _, h4331, _, _, _, _, _) => {
            h4331
        }
    }), f1069(f1085(match v0.clone() {
        Adt171::C0(_, _, _, _, h4332, _, _, _, _, _) => {
            h4332
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, h4333, _, _, _, _) => {
            h4333
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, h4334, _, _, _) => {
            h4334
        }
    }), f1069(v1, {
        let m4337 = match v0.clone() {
            Adt171::C0(_, _, _, _, _, _, _, _, h4336, _) => {
                h4336
            }
        };
        match m4337 {
            Adt170::C0(h4335, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4335
            }
        }
    }))));
    let v3: u32 = f1069(f1086(match v0.clone() {
        Adt171::C0(h4338, _, _, _, _, _, _, _, _, _) => {
            h4338
        }
    }), f1087(match v0.clone() {
        Adt171::C0(h4339, _, _, _, _, _, _, _, _, _) => {
            h4339
        }
    }, match v0.clone() {
        Adt171::C0(_, h4340, _, _, _, _, _, _, _, _) => {
            h4340
        }
    }, match v0.clone() {
        Adt171::C0(_, _, h4341, _, _, _, _, _, _, _) => {
            h4341
        }
    }));
    Ok(Adt171::C0(f1069(v2, v3), match v0.clone() {
        Adt171::C0(h4342, _, _, _, _, _, _, _, _, _) => {
            h4342
        }
    }, match v0.clone() {
        Adt171::C0(_, h4343, _, _, _, _, _, _, _, _) => {
            h4343
        }
    }, match v0.clone() {
        Adt171::C0(_, _, h4344, _, _, _, _, _, _, _) => {
            h4344
        }
    }, f1069(match v0.clone() {
        Adt171::C0(_, _, _, h4345, _, _, _, _, _, _) => {
            h4345
        }
    }, v2), match v0.clone() {
        Adt171::C0(_, _, _, _, h4346, _, _, _, _, _) => {
            h4346
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, h4347, _, _, _, _) => {
            h4347
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, h4348, _, _, _) => {
            h4348
        }
    }, f1088(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, _, h4349, _) => {
            h4349
        }
    }), {
        let a4351 = match v0 {
            Adt171::C0(_, _, _, _, _, _, _, _, _, h4350) => {
                h4350
            }
        };
        let a4352 = 1u64;
        nat_add(a4351, a4352)?
    }))
}

pub fn f1077(v0: u32) -> u64 {
    let m4354 = {
        let a4353 = v0;
        fixed_u64::convert(i128::from(a4353))
    };
    match m4354 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1078(v0: u64) -> u32 {
    let m4358 = {
        let a4357 = {
            let a4355 = v0;
            let a4356 = 4294967295u64;
            fixed_u64::bit_and(a4355, a4356)
        };
        fixed_u32::convert(i128::from(a4357))
    };
    match m4358 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1079(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt170> {
    Ok(Adt170::C0(f1089(v0.clone(), v1, v2, v3)?, f1089(v0.clone(), v1, v2, {
        let a4359 = v3;
        let a4360 = 4u64;
        nat_add(a4359, a4360)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4361 = v3;
        let a4362 = 8u64;
        nat_add(a4361, a4362)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4363 = v3;
        let a4364 = 12u64;
        nat_add(a4363, a4364)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4365 = v3;
        let a4366 = 16u64;
        nat_add(a4365, a4366)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4367 = v3;
        let a4368 = 20u64;
        nat_add(a4367, a4368)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4369 = v3;
        let a4370 = 24u64;
        nat_add(a4369, a4370)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4371 = v3;
        let a4372 = 28u64;
        nat_add(a4371, a4372)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4373 = v3;
        let a4374 = 32u64;
        nat_add(a4373, a4374)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4375 = v3;
        let a4376 = 36u64;
        nat_add(a4375, a4376)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4377 = v3;
        let a4378 = 40u64;
        nat_add(a4377, a4378)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4379 = v3;
        let a4380 = 44u64;
        nat_add(a4379, a4380)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4381 = v3;
        let a4382 = 48u64;
        nat_add(a4381, a4382)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4383 = v3;
        let a4384 = 52u64;
        nat_add(a4383, a4384)?
    })?, f1089(v0.clone(), v1, v2, {
        let a4385 = v3;
        let a4386 = 56u64;
        nat_add(a4385, a4386)?
    })?, f1089(v0, v1, v2, {
        let a4387 = v3;
        let a4388 = 60u64;
        nat_add(a4387, a4388)?
    })?))
}

pub fn f1080(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a4393 = f1090(v0, 24u32);
    let a4394 = {
        let a4391 = f1090(v1, 16u32);
        let a4392 = {
            let a4389 = f1090(v2, 8u32);
            let a4390 = v3;
            fixed_u32::bit_or(a4389, a4390)
        };
        fixed_u32::bit_or(a4391, a4392)
    };
    fixed_u32::bit_or(a4393, a4394)
}

pub fn f1081(v0: Bytes, v1: u64) -> u32 {
    let m4397 = {
        let a4395 = v0;
        let a4396 = v1;
        index_bytes(a4395, a4396)
    };
    match m4397 {
        None => {
            0u32
        }
        Some(v2) => {
            let m4399 = {
                let a4398 = v2;
                fixed_u32::convert(i128::from(a4398))
            };
            match m4399 {
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

pub fn f1082(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a4402 = {
        let a4400 = v4;
        let a4401 = 8u64;
        nat_add(a4400, a4401)?
    };
    let a4403 = v2;
    if nat_eq(a4402, a4403) {
        Ok(f1091(v3, true))
    } else {
        let a4406 = {
            let a4404 = v4;
            let a4405 = 4u64;
            nat_add(a4404, a4405)?
        };
        let a4407 = v2;
        if nat_eq(a4406, a4407) {
            Ok(f1091(v3, false))
        } else {
            Ok(f1080(f1092(v0.clone(), v1, v4), f1092(v0.clone(), v1, {
                let a4408 = v4;
                let a4409 = 1u64;
                nat_add(a4408, a4409)?
            }), f1092(v0.clone(), v1, {
                let a4410 = v4;
                let a4411 = 2u64;
                nat_add(a4410, a4411)?
            }), f1092(v0, v1, {
                let a4412 = v4;
                let a4413 = 3u64;
                nat_add(a4412, a4413)?
            })))
        }
    }
}

pub fn f1083(v0: u64) -> u32 {
    let a4414 = v0;
    let a4415 = 32u64;
    if nat_lt(a4414, a4415) {
        let a4416 = v0;
        let a4417 = 16u64;
        if nat_lt(a4416, a4417) {
            let a4418 = v0;
            let a4419 = 8u64;
            if nat_lt(a4418, a4419) {
                let a4420 = v0;
                let a4421 = 4u64;
                if nat_lt(a4420, a4421) {
                    let a4422 = v0;
                    let a4423 = 2u64;
                    if nat_lt(a4422, a4423) {
                        let a4424 = v0;
                        let a4425 = 1u64;
                        if nat_lt(a4424, a4425) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4426 = v0;
                        let a4427 = 3u64;
                        if nat_lt(a4426, a4427) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4428 = v0;
                    let a4429 = 6u64;
                    if nat_lt(a4428, a4429) {
                        let a4430 = v0;
                        let a4431 = 5u64;
                        if nat_lt(a4430, a4431) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4432 = v0;
                        let a4433 = 7u64;
                        if nat_lt(a4432, a4433) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4434 = v0;
                let a4435 = 12u64;
                if nat_lt(a4434, a4435) {
                    let a4436 = v0;
                    let a4437 = 10u64;
                    if nat_lt(a4436, a4437) {
                        let a4438 = v0;
                        let a4439 = 9u64;
                        if nat_lt(a4438, a4439) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4440 = v0;
                        let a4441 = 11u64;
                        if nat_lt(a4440, a4441) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4442 = v0;
                    let a4443 = 14u64;
                    if nat_lt(a4442, a4443) {
                        let a4444 = v0;
                        let a4445 = 13u64;
                        if nat_lt(a4444, a4445) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4446 = v0;
                        let a4447 = 15u64;
                        if nat_lt(a4446, a4447) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4448 = v0;
            let a4449 = 24u64;
            if nat_lt(a4448, a4449) {
                let a4450 = v0;
                let a4451 = 20u64;
                if nat_lt(a4450, a4451) {
                    let a4452 = v0;
                    let a4453 = 18u64;
                    if nat_lt(a4452, a4453) {
                        let a4454 = v0;
                        let a4455 = 17u64;
                        if nat_lt(a4454, a4455) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4456 = v0;
                        let a4457 = 19u64;
                        if nat_lt(a4456, a4457) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4458 = v0;
                    let a4459 = 22u64;
                    if nat_lt(a4458, a4459) {
                        let a4460 = v0;
                        let a4461 = 21u64;
                        if nat_lt(a4460, a4461) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4462 = v0;
                        let a4463 = 23u64;
                        if nat_lt(a4462, a4463) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4464 = v0;
                let a4465 = 28u64;
                if nat_lt(a4464, a4465) {
                    let a4466 = v0;
                    let a4467 = 26u64;
                    if nat_lt(a4466, a4467) {
                        let a4468 = v0;
                        let a4469 = 25u64;
                        if nat_lt(a4468, a4469) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4470 = v0;
                        let a4471 = 27u64;
                        if nat_lt(a4470, a4471) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4472 = v0;
                    let a4473 = 30u64;
                    if nat_lt(a4472, a4473) {
                        let a4474 = v0;
                        let a4475 = 29u64;
                        if nat_lt(a4474, a4475) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4476 = v0;
                        let a4477 = 31u64;
                        if nat_lt(a4476, a4477) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4478 = v0;
        let a4479 = 48u64;
        if nat_lt(a4478, a4479) {
            let a4480 = v0;
            let a4481 = 40u64;
            if nat_lt(a4480, a4481) {
                let a4482 = v0;
                let a4483 = 36u64;
                if nat_lt(a4482, a4483) {
                    let a4484 = v0;
                    let a4485 = 34u64;
                    if nat_lt(a4484, a4485) {
                        let a4486 = v0;
                        let a4487 = 33u64;
                        if nat_lt(a4486, a4487) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4488 = v0;
                        let a4489 = 35u64;
                        if nat_lt(a4488, a4489) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4490 = v0;
                    let a4491 = 38u64;
                    if nat_lt(a4490, a4491) {
                        let a4492 = v0;
                        let a4493 = 37u64;
                        if nat_lt(a4492, a4493) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4494 = v0;
                        let a4495 = 39u64;
                        if nat_lt(a4494, a4495) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4496 = v0;
                let a4497 = 44u64;
                if nat_lt(a4496, a4497) {
                    let a4498 = v0;
                    let a4499 = 42u64;
                    if nat_lt(a4498, a4499) {
                        let a4500 = v0;
                        let a4501 = 41u64;
                        if nat_lt(a4500, a4501) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4502 = v0;
                        let a4503 = 43u64;
                        if nat_lt(a4502, a4503) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4504 = v0;
                    let a4505 = 46u64;
                    if nat_lt(a4504, a4505) {
                        let a4506 = v0;
                        let a4507 = 45u64;
                        if nat_lt(a4506, a4507) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4508 = v0;
                        let a4509 = 47u64;
                        if nat_lt(a4508, a4509) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4510 = v0;
            let a4511 = 56u64;
            if nat_lt(a4510, a4511) {
                let a4512 = v0;
                let a4513 = 52u64;
                if nat_lt(a4512, a4513) {
                    let a4514 = v0;
                    let a4515 = 50u64;
                    if nat_lt(a4514, a4515) {
                        let a4516 = v0;
                        let a4517 = 49u64;
                        if nat_lt(a4516, a4517) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4518 = v0;
                        let a4519 = 51u64;
                        if nat_lt(a4518, a4519) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4520 = v0;
                    let a4521 = 54u64;
                    if nat_lt(a4520, a4521) {
                        let a4522 = v0;
                        let a4523 = 53u64;
                        if nat_lt(a4522, a4523) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4524 = v0;
                        let a4525 = 55u64;
                        if nat_lt(a4524, a4525) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4526 = v0;
                let a4527 = 60u64;
                if nat_lt(a4526, a4527) {
                    let a4528 = v0;
                    let a4529 = 58u64;
                    if nat_lt(a4528, a4529) {
                        let a4530 = v0;
                        let a4531 = 57u64;
                        if nat_lt(a4530, a4531) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4532 = v0;
                        let a4533 = 59u64;
                        if nat_lt(a4532, a4533) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4534 = v0;
                    let a4535 = 62u64;
                    if nat_lt(a4534, a4535) {
                        let a4536 = v0;
                        let a4537 = 61u64;
                        if nat_lt(a4536, a4537) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4538 = v0;
                        let a4539 = 63u64;
                        if nat_lt(a4538, a4539) {
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

pub fn f1084(v0: u32) -> u32 {
    let a4542 = {
        let a4540 = f1093(v0, 6u32, 26u32);
        let a4541 = f1093(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4540, a4541)
    };
    let a4543 = f1093(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4542, a4543)
}

pub fn f1085(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4549 = {
        let a4544 = v0;
        let a4545 = v1;
        fixed_u32::bit_and(a4544, a4545)
    };
    let a4550 = {
        let a4547 = {
            let a4546 = v0;
            fixed_u32::bit_not(a4546)
        };
        let a4548 = v2;
        fixed_u32::bit_and(a4547, a4548)
    };
    fixed_u32::bit_xor(a4549, a4550)
}

pub fn f1086(v0: u32) -> u32 {
    let a4553 = {
        let a4551 = f1093(v0, 2u32, 30u32);
        let a4552 = f1093(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4551, a4552)
    };
    let a4554 = f1093(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4553, a4554)
}

pub fn f1087(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4563 = {
        let a4559 = {
            let a4555 = v0;
            let a4556 = v1;
            fixed_u32::bit_and(a4555, a4556)
        };
        let a4560 = {
            let a4557 = v0;
            let a4558 = v2;
            fixed_u32::bit_and(a4557, a4558)
        };
        fixed_u32::bit_xor(a4559, a4560)
    };
    let a4564 = {
        let a4561 = v1;
        let a4562 = v2;
        fixed_u32::bit_and(a4561, a4562)
    };
    fixed_u32::bit_xor(a4563, a4564)
}

pub fn f1088(v0: Adt170) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt170::C0(_, h4565, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4565
        }
    }, match v0.clone() {
        Adt170::C0(_, _, h4566, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4566
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, h4567, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4567
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, h4568, _, _, _, _, _, _, _, _, _, _, _) => {
            h4568
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, h4569, _, _, _, _, _, _, _, _, _, _) => {
            h4569
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, h4570, _, _, _, _, _, _, _, _, _) => {
            h4570
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, h4571, _, _, _, _, _, _, _, _) => {
            h4571
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, h4572, _, _, _, _, _, _, _) => {
            h4572
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, h4573, _, _, _, _, _, _) => {
            h4573
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, h4574, _, _, _, _, _) => {
            h4574
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, h4575, _, _, _, _) => {
            h4575
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4576, _, _, _) => {
            h4576
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4577, _, _) => {
            h4577
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4578, _) => {
            h4578
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4579) => {
            h4579
        }
    }, f1069(f1094(match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4580, _) => {
            h4580
        }
    }), f1069(match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, h4581, _, _, _, _, _, _) => {
            h4581
        }
    }, f1069(f1095(match v0.clone() {
        Adt170::C0(_, h4582, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4582
        }
    }), match v0 {
        Adt170::C0(h4583, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4583
        }
    }))))
}

pub fn f1089(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4586 = {
        let a4584 = v3;
        let a4585 = 8u64;
        nat_add(a4584, a4585)?
    };
    let a4587 = v2;
    if nat_eq(a4586, a4587) {
        Ok(f1096({
            let a4590 = {
                let a4588 = v1;
                let a4589 = 8u64;
                nat_mul(a4588, a4589)?
            };
            let a4591 = 4294967296u64;
            let a4592 = 0u64;
            nat_quot(a4590, a4591, a4592)
        }))
    } else {
        let a4595 = {
            let a4593 = v3;
            let a4594 = 4u64;
            nat_add(a4593, a4594)?
        };
        let a4596 = v2;
        if nat_eq(a4595, a4596) {
            Ok(f1096({
                let a4599 = {
                    let a4597 = v1;
                    let a4598 = 8u64;
                    nat_mul(a4597, a4598)?
                };
                let a4600 = 4294967296u64;
                let a4601 = 0u64;
                nat_rem(a4599, a4600, a4601)
            }))
        } else {
            Ok(f1080(f1092(v0.clone(), v1, v3), f1092(v0.clone(), v1, {
                let a4602 = v3;
                let a4603 = 1u64;
                nat_add(a4602, a4603)?
            }), f1092(v0.clone(), v1, {
                let a4604 = v3;
                let a4605 = 2u64;
                nat_add(a4604, a4605)?
            }), f1092(v0, v1, {
                let a4606 = v3;
                let a4607 = 3u64;
                nat_add(a4606, a4607)?
            })))
        }
    }
}

pub fn f1090(v0: u32, v1: u32) -> u32 {
    let m4610 = {
        let a4608 = v0;
        let a4609 = v1;
        fixed_u32::shift_left(a4608, a4609)
    };
    match m4610 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1091(v0: u64, v1: bool) -> u32 {
    if v1 {
        f1096({
            let a4611 = v0;
            let a4612 = 4294967296u64;
            let a4613 = 0u64;
            nat_quot(a4611, a4612, a4613)
        })
    } else {
        f1096({
            let a4614 = v0;
            let a4615 = 4294967296u64;
            let a4616 = 0u64;
            nat_rem(a4614, a4615, a4616)
        })
    }
}

pub fn f1092(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4617 = v2;
    let a4618 = v1;
    if nat_lt(a4617, a4618) {
        let m4621 = {
            let a4619 = v0;
            let a4620 = v2;
            index_bytes(a4619, a4620)
        };
        match m4621 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4623 = {
                    let a4622 = v3;
                    fixed_u32::convert(i128::from(a4622))
                };
                match m4623 {
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
        let a4624 = v2;
        let a4625 = v1;
        if nat_eq(a4624, a4625) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f1093(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4626 = f1072(v0, v1);
    let a4627 = f1090(v0, v2);
    fixed_u32::bit_or(a4626, a4627)
}

pub fn f1094(v0: u32) -> u32 {
    let a4630 = {
        let a4628 = f1093(v0, 17u32, 15u32);
        let a4629 = f1093(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4628, a4629)
    };
    let a4631 = f1072(v0, 10u32);
    fixed_u32::bit_xor(a4630, a4631)
}

pub fn f1095(v0: u32) -> u32 {
    let a4634 = {
        let a4632 = f1093(v0, 7u32, 25u32);
        let a4633 = f1093(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4632, a4633)
    };
    let a4635 = f1072(v0, 3u32);
    fixed_u32::bit_xor(a4634, a4635)
}

pub fn f1096(v0: u64) -> u32 {
    let (_, h4636) = {
        let (_, h4637) = f1097(Fn14::F1098, 32u64, (v0, (1u32, 0u32)));
        h4637
    };
    h4636
}

pub fn f1097(v0: Fn14, p4641: u64, p4642: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4640 = (p4641, p4642);
    {
        let mut m4640 = m4640;
        loop {
            let step = {
                let (v1, v2) = m4640;
                let m4638 = v1;
                if m4638 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4638 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4639: Fn14 = v0.clone();
                        c4639.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4640 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1098(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4643, _) = v0;
        h4643
    };
    let v2: u32 = {
        let (h4644, _) = {
            let (_, h4645) = v0;
            h4645
        };
        h4644
    };
    let v3: u32 = {
        let (_, h4646) = {
            let (_, h4647) = v0;
            h4647
        };
        h4646
    };
    ({
        let a4648 = v1;
        let a4649 = 2u64;
        let a4650 = 0u64;
        nat_quot(a4648, a4649, a4650)
    }, (f1090(v2, 1u32), {
        let a4654 = {
            let a4651 = v1;
            let a4652 = 2u64;
            let a4653 = 0u64;
            nat_rem(a4651, a4652, a4653)
        };
        let a4655 = 1u64;
        if nat_eq(a4654, a4655) {
            let a4656 = v3;
            let a4657 = v2;
            fixed_u32::bit_or(a4656, a4657)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
